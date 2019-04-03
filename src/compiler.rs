use crate::ast::*;
use failure::Error;
use wasmly::WebAssembly::*;
use wasmly::*;

enum IdentifierType {
    Global,
    Local,
    Function,
}

struct Compiler {
    wasm: wasmly::App,
    ast: crate::ast::App,
    symbols: Vec<String>,
    global_names: Vec<String>,
    global_values: Vec<f64>,
    local_names: Vec<String>,
    heap_position: f64,
    function_defs: Vec<TopLevelOperation>,
    function_names: Vec<String>,
    function_implementations: Vec<wasmly::Function>,
    non_imported_functions: Vec<String>,
    recur_depth: u32,
}

impl Compiler {
    fn new(app: crate::ast::App) -> Compiler {
        let mut c = Compiler {
            wasm: wasmly::App::new(vec![]),
            ast: app,
            symbols: vec![],
            global_names: vec![],
            global_values: vec![],
            local_names: vec![],
            heap_position: 4.0, //start at 4 so nothing has 0 address
            function_defs: vec![],
            function_names: vec![],
            function_implementations: vec![],
            non_imported_functions: vec![],
            recur_depth: 0,
        };
        c.initialize();
        c
    }

    fn initialize(&mut self) {
        //Get imports so we can start creating app
        let import_defs = self
            .ast
            .children
            .iter()
            .filter_map(|x| match x {
                TopLevelOperation::ExternalFunction(x) => Some(x),
                _ => None,
            })
            .collect::<Vec<&ExternalFunction>>();

        let mut imports = vec![];
        for def in import_defs {
            self.function_names.push(def.name.clone());
            imports.push(Import::ImportFunction(ImportFunction::new(
                def.name.clone(),
                def.params.iter().map(|_| DataType::F64).collect(),
                Some(DataType::F64),
            )))
        }
        self.wasm = wasmly::App::new(imports);
        self.function_defs = self
            .ast
            .children
            .iter()
            .filter_map(|x| match x {
                TopLevelOperation::DefineFunction(_) => Some(x.clone()),
                TopLevelOperation::DefineTestFunction(_) => Some(x.clone()),
                _ => None,
            })
            .collect::<Vec<TopLevelOperation>>();
    }

    fn process_globals(&mut self) {
        let global_defs = self
            .ast
            .children
            .iter()
            .filter_map(|x| match x {
                TopLevelOperation::DefineGlobal(x) => Some(x.clone()),
                _ => None,
            })
            .collect::<Vec<crate::ast::Global>>();
        for def in global_defs {
            self.global_names.push(def.name.clone());
            let v = self.get_global_value(&def.value);
            self.global_values.push(v);
        }
    }

    fn float_to_bytes(&self, i: f64) -> Vec<u8> {
        let raw_bytes: [u8; 8] = unsafe { std::mem::transmute(i) };
        let bytes: Vec<u8> = raw_bytes.to_vec();
        bytes
    }

    fn create_global_data(&mut self, v: Vec<GlobalValue>) -> f64 {
        let mut bytes = vec![];
        for i in 0..v.len() {
            let v = self.get_global_value(&v[i]);
            let b = self.float_to_bytes(v);
            bytes.extend_from_slice(&b);
        }
        self.create_data(bytes)
    }

    fn get_symbol_value(&mut self, t:&str) -> f64 {
        // no symbol has the value 0
        let v = self.symbols.iter().enumerate().find(|x|&x.1==&t);
        if let Some(i) = v {
            return i.0 as f64+1.0;
        } else {
            self.symbols.push(t.to_string());
            return self.symbols.len() as f64;
        }
    }

    fn get_global_value(&mut self, v: &GlobalValue) -> f64 {
        match v {
            GlobalValue::Symbol(t) => {
                self.get_symbol_value(t)
            },
            GlobalValue::Number(t) => *t,
            GlobalValue::Text(t) => self.get_or_create_text_data(&t),
            GlobalValue::Data(t) => self.create_global_data(t.clone()),
            GlobalValue::Struct(s) => {
                let mut t:Vec<GlobalValue> = vec![];
                for i in 0..s.members.len() {
                    t.push(GlobalValue::Symbol(s.members[i].name.clone()));
                    t.push(GlobalValue::Text(s.members[i].attributes.clone().unwrap_or("".to_owned())));
                }
                t.push(GlobalValue::Number(0.0));
                self.create_global_data(t)
            },
            GlobalValue::Identifier(t) => self.resolve_identifier(t).0,
        }
    }

    fn pre_process_functions(&mut self) {
        // gather all the function names and positions we shall use
        self.non_imported_functions = vec![];
        for i in 0..self.function_defs.len() {
            if let TopLevelOperation::DefineFunction(function_def) = &self.function_defs[i] {
                self.function_names.push(function_def.name.clone());
                self.non_imported_functions.push(function_def.name.clone());
            } else if let TopLevelOperation::DefineTestFunction(function_def) =
                &self.function_defs[i]
            {
                self.function_names.push(function_def.name.clone());
                self.non_imported_functions.push(function_def.name.clone());
            }
        }

        // get the basics about our functions loaded into memory
        for i in 0..self.function_defs.len() {
            if let TopLevelOperation::DefineFunction(function_def) = &self.function_defs[i] {
                let mut function = Function::new();
                if function_def.exported {
                    function.with_name(&function_def.name);
                }
                function.with_inputs(function_def.params.iter().map(|_| DataType::F64).collect());
                function.with_output(DataType::F64);
                self.function_implementations.push(function);
            } else if let TopLevelOperation::DefineTestFunction(function_def) =
                &self.function_defs[i]
            {
                let mut function = Function::new();
                function.with_name(&format!("test_{}", function_def.name));
                function.with_output(DataType::F64);
                self.function_implementations.push(function);
            }
        }

        self.wasm.add_table(wasmly::Table::new(
            self.function_names.len() as u32,
            self.function_names.len() as u32,
        ));
    }

    fn set_heap_start(&mut self) {
        //set global heap once we know what it should be
        let final_heap_pos = {
            if self.heap_position % 4.0 != 0.0 {
                (self.heap_position / 4.0) * 4.0 + 4.0
            } else {
                self.heap_position
            }
        };
        self.wasm
            .add_global(wasmly::Global::new(final_heap_pos as i32, false));
        self.wasm
            .add_global(wasmly::Global::new(final_heap_pos as i32, true));
    }

    fn get_or_create_text_data(&mut self, str: &str) -> f64 {
        let mut bytes: Vec<u8> = str.as_bytes().into();
        bytes.push(0);
        self.create_data(bytes)
    }

    fn create_data(&mut self, bytes: Vec<u8>) -> f64 {
        let pos = self.heap_position;
        let size = bytes.len();
        self.wasm.add_data(Data::new(pos as i32, bytes));
        let mut final_heap_pos = self.heap_position + (size as f64);
        // align data to 4
        // TODO: verify if this actually matters
        if final_heap_pos % 4.0 != 0.0 {
            final_heap_pos = (final_heap_pos / 4.0) * 4.0 + 4.0;
        }
        self.heap_position = final_heap_pos;
        pos
    }

    fn resolve_identifier(&self, id: &str) -> (f64, IdentifierType) {
        if id == "nil" {
            return (0.0,IdentifierType::Global)
        }
        if id == "size_num" {
            return (8.0,IdentifierType::Global)
        }
        // look this up in reverse so shadowing works
        let mut p = self.local_names.iter().rev().position(|r| r == id);
        if p.is_some() {
            return (
                self.local_names.len() as f64 - 1.0 - p.unwrap() as f64,
                IdentifierType::Local,
            );
        }
        p = self.function_names.iter().position(|r| r == id);
        if p.is_some() {
            return (p.unwrap() as f64, IdentifierType::Function);
        }
        p = self.global_names.iter().position(|r| r == id);
        if p.is_some() {
            return (self.global_values[p.unwrap()], IdentifierType::Global);
        }
        panic!(format!("could not find identifier \"{}\"", id))
    }

    #[allow(clippy::cyclomatic_complexity)]
    fn process_expression(&mut self, i: usize, e: &Expression) {
        match e {
            Expression::SymbolLiteral(x) => {
                let v = self.get_symbol_value(x);
                self.function_implementations[i]
                    .with_instructions(vec![F64_CONST, v.into()]);
            }
            Expression::Populate(x) => {
                let val = self.resolve_identifier(&x.name);
                self.function_implementations[i].with_local(DataType::F64);
                self.local_names.push("".to_string());
                let loc_storage = (self.local_names.len() - 1) as i32;
                match val.1 {
                    IdentifierType::Function => {
                        let fn_def = self
                            .function_defs
                            .iter()
                            .find(|k| {
                                if let TopLevelOperation::DefineFunction(y) = k {
                                    y.name == x.name
                                } else {
                                    false
                                }
                            })
                            .unwrap();
                        let param_count = if let TopLevelOperation::DefineFunction(f) = fn_def {
                            f.params.len() - 1
                        } else {
                            panic!("not sure how got here");
                        };

                        let expr: Vec<_> = x.elements.chunks(param_count).rev().collect();
                        for j in 0..expr.len() {
                            if j == 0 && j != expr.len() - 1 {
                                for k in 0..expr[j].len() {
                                    self.process_expression(i, &expr[j][k])
                                }
                                self.function_implementations[i].with_instructions(vec![
                                    F64_CONST, 0.0.into(),
                                    CALL, (val.0 as i32).into(),
                                    LOCAL_SET, (loc_storage as i32).into(),
                                ]);
                            } else if j == expr.len() - 1 {
                                for k in 0..expr[j].len() {
                                    self.process_expression(i, &expr[j][k])
                                }
                                self.function_implementations[i].with_instructions(vec![
                                    LOCAL_GET, (loc_storage as i32).into(),
                                    CALL, (val.0 as i32).into(),
                                ]);
                                break;
                            } else {
                                for k in 0..expr[j].len() {
                                    self.process_expression(i, &expr[j][k])
                                }
                                self.function_implementations[i].with_instructions(vec![
                                    LOCAL_GET, (loc_storage as i32).into(),
                                    CALL, (val.0 as i32).into(),
                                    LOCAL_SET, (loc_storage as i32).into(),
                                ]);
                            }
                        }
                    }
                    _ => panic!("cannot populate on non function"),
                }
            }
            Expression::FnSig(x) => {
                let t = self
                    .wasm
                    .add_type(FunctionType::new(x.inputs.clone(), x.output.clone()));
                self.function_implementations[i]
                    .with_instructions(vec![F64_CONST, (t as f64).into()]);
            }
            Expression::Loop(x) => {
                self.recur_depth = 0;
                for j in 0..x.bindings.len() {
                    let binding = &x.bindings[j];
                    self.process_expression(i, &binding.1);
                    self.function_implementations[i].with_local(DataType::F64);
                    self.function_implementations[i]
                        .with_instructions(vec![LOCAL_SET, (self.local_names.len() as u32).into()]);
                    self.local_names.push((&binding.0).to_string());
                }
                if !x.expressions.is_empty() {
                    self.function_implementations[i].with_instructions(vec![LOOP, F64]);
                    for k in 0..x.expressions.len() {
                        self.process_expression(i, &x.expressions[k]);
                        if k != x.expressions.len() - 1 {
                            self.function_implementations[i].with_instructions(vec![DROP]);
                        }
                    }
                    self.function_implementations[i].with_instructions(vec![END]);
                } else {
                    panic!("useless infinite loop detected")
                }
                for _ in 0..x.bindings.len() {
                    self.local_names.pop();
                }
            }
            Expression::Recur(x) => {
                for j in 0..x.bindings.len() {
                    let binding = &x.bindings[j];
                    let name = (&binding.0).to_string();
                    let val = self.resolve_identifier(&name);
                    match val.1 {
                        IdentifierType::Local => {
                            self.process_expression(i, &binding.1);
                            self.function_implementations[i]
                                .with_instructions(vec![LOCAL_SET, (val.0 as i32).into()]);
                        }
                        _ => panic!("cannot recur by rebinding a non-local identifier"),
                    }
                }
                self.function_implementations[i].with_instructions(vec![
                    F64_CONST,
                    0.0.into(),
                    BR,
                    self.recur_depth.into(),
                ]);
            }
            Expression::Let(x) => {
                for j in 0..x.bindings.len() {
                    let binding = &x.bindings[j];
                    self.process_expression(i, &binding.1);
                    self.function_implementations[i].with_local(DataType::F64);
                    self.function_implementations[i]
                        .with_instructions(vec![LOCAL_SET, (self.local_names.len() as u32).into()]);
                    self.local_names.push((&binding.0).to_string());
                }
                for k in 0..x.expressions.len() {
                    self.process_expression(i, &x.expressions[k]);
                    if k != x.expressions.len() - 1 {
                        self.function_implementations[i].with_instructions(vec![DROP]);
                    }
                }
                for _ in 0..x.bindings.len() {
                    self.local_names.pop();
                }
            }
            Expression::FunctionCall(x) => {
                if &x.function_name == "do" {
                    if !x.params.is_empty() {
                        for k in 0..x.params.len() {
                            self.process_expression(i, &x.params[k]);
                            if k != x.params.len() - 1 {
                                self.function_implementations[i].with_instructions(vec![DROP]);
                            }
                        }
                    } else {
                        panic!("useless do detected")
                    }
                } else if &x.function_name == "call" {
                    if x.params.len() >= 2 {
                        if let Expression::FnSig(sig) = &x.params[0] {
                            for k in 2..x.params.len() {
                                self.process_expression(i, &x.params[k]);
                            }
                            self.process_expression(i, &x.params[1]);
                            self.function_implementations[i]
                                .with_instructions(vec![I32_TRUNC_S_F64]);
                            let t = self.wasm.add_type(FunctionType::new(
                                sig.inputs.clone(),
                                sig.output.clone(),
                            ));
                            self.function_implementations[i].with_instructions(vec![
                                CALL_INDIRECT,
                                t.into(),
                                0.into(),
                            ]);
                            if sig.output.is_none() {
                                self.function_implementations[i]
                                    .with_instructions(vec![F64_CONST, 0.0.into()]);
                            }
                        } else {
                            panic!("call must begin with a function signature not an expression")
                        }
                    } else {
                        panic!("call must have at least function signature and function index")
                    }
                } else if &x.function_name == "if" {
                    self.recur_depth += 1;
                    if x.params.len() == 2 {
                        self.process_expression(i, &x.params[0]);
                        self.function_implementations[i].with_instructions(vec![
                            F64_CONST,
                            0.0.into(),
                            F64_EQ,
                            I32_CONST,
                            0.into(),
                            I32_EQ,
                        ]);
                        self.function_implementations[i].with_instructions(vec![IF, F64]);
                        self.process_expression(i, &x.params[1]);
                        self.function_implementations[i].with_instructions(vec![
                            ELSE,
                            F64_CONST,
                            0.0.into(),
                            END,
                        ]);
                    } else if x.params.len() == 3 {
                        self.process_expression(i, &x.params[0]);
                        self.function_implementations[i].with_instructions(vec![
                            F64_CONST,
                            0.0.into(),
                            F64_EQ,
                            I32_CONST,
                            0.into(),
                            I32_EQ,
                        ]);
                        self.function_implementations[i].with_instructions(vec![IF, F64]);
                        self.process_expression(i, &x.params[1]);
                        self.function_implementations[i].with_instructions(vec![ELSE]);
                        self.process_expression(i, &x.params[2]);
                        self.function_implementations[i].with_instructions(vec![END]);
                    } else {
                        panic!("invalid number of params for if")
                    }
                } else if &x.function_name == "mem" {
                    if x.params.len() == 1 {
                        self.process_expression(i, &x.params[0]);
                        self.function_implementations[i].with_instructions(vec![I32_TRUNC_S_F64]);
                        self.function_implementations[i].with_instructions(vec![
                            I32_LOAD8_U,
                            0.into(),
                            0.into(),
                            F64_CONVERT_S_I32,
                        ]);
                    } else if x.params.len() == 2 {
                        for k in 0..x.params.len() {
                            self.process_expression(i, &x.params[k]);
                            self.function_implementations[i]
                                .with_instructions(vec![I32_TRUNC_S_F64]);
                        }
                        self.function_implementations[i].with_instructions(vec![
                            I32_STORE8,
                            0.into(),
                            0.into(),
                        ]);
                        self.function_implementations[i]
                            .with_instructions(vec![F64_CONST, 0.0.into()]);
                    } else {
                        panic!("invalid number params for mem")
                    }
                } else if &x.function_name == "mem_heap_start" {
                    if x.params.len() == 0 {
                        self.function_implementations[i].with_instructions(vec![
                            GLOBAL_GET,
                            0.into(),
                            F64_CONVERT_S_I32,
                        ]);
                    } else {
                        panic!("invalid number params for mem_heap_start")
                    }
                } else if &x.function_name == "mem_heap_end" {
                    if x.params.len() == 0 {
                        self.function_implementations[i].with_instructions(vec![
                            GLOBAL_GET,
                            1.into(),
                            F64_CONVERT_S_I32,
                        ]);
                    } else if x.params.len() == 1 {
                        self.process_expression(i, &x.params[0]);
                        self.function_implementations[i].with_instructions(vec![I32_TRUNC_S_F64]);
                        self.function_implementations[i].with_instructions(vec![
                            GLOBAL_SET,
                            1.into(),
                            I32_CONST,
                            0.into(),
                        ]);
                    } else {
                        panic!("invalid number params for mem_heap_start")
                    }
                } else if &x.function_name == "mem_num" {
                    if x.params.len() == 1 {
                        self.process_expression(i, &x.params[0]);
                        self.function_implementations[i].with_instructions(vec![
                            I32_TRUNC_S_F64,
                            F64_LOAD,
                            (0 as i32).into(),
                            (0 as i32).into(),
                        ]);
                    } else if x.params.len() == 2 {
                        self.process_expression(i, &x.params[0]);
                        self.function_implementations[i].with_instructions(vec![I32_TRUNC_S_F64]);
                        self.process_expression(i, &x.params[1]);
                        self.function_implementations[i].with_instructions(vec![
                            F64_STORE,
                            (0 as i32).into(),
                            (0 as i32).into(),
                        ]);
                        self.function_implementations[i]
                            .with_instructions(vec![F64_CONST, 0.0.into()]);
                    } else {
                        panic!("invalid number params for mem_num")
                    }
                } else if &x.function_name == "=="
                    || &x.function_name == "!="
                    || &x.function_name == "<="
                    || &x.function_name == ">="
                    || &x.function_name == "<"
                    || &x.function_name == ">"
                {
                    if x.params.len() != 2 {
                        panic!(
                            "operator {} expected 2 parameters",
                            (&x.function_name).as_str()
                        );
                    }
                    self.process_expression(i, &x.params[0]);
                    self.process_expression(i, &x.params[1]);
                    let mut f = match (&x.function_name).as_str() {
                        "==" => vec![F64_EQ],
                        "!=" => vec![F64_NE],
                        "<=" => vec![F64_LE],
                        ">=" => vec![F64_GE],
                        "<" => vec![F64_LT],
                        ">" => vec![F64_GT],
                        _ => panic!("unexpected operator"),
                    };
                    f.extend(vec![F64_CONVERT_S_I32]);
                    self.function_implementations[i].with_instructions(f);
                } else if &x.function_name == "&"
                    || &x.function_name == "|"
                    || &x.function_name == "^"
                    || &x.function_name == "<<"
                    || &x.function_name == ">>"
                {
                    if x.params.len() != 2 {
                        panic!(
                            "operator {} expected 2 parameters",
                            (&x.function_name).as_str()
                        );
                    }
                    self.process_expression(i, &x.params[0]);
                    self.function_implementations[i].with_instructions(vec![I64_TRUNC_S_F64]);
                    self.process_expression(i, &x.params[1]);
                    self.function_implementations[i].with_instructions(vec![I64_TRUNC_S_F64]);
                    let mut f = match (&x.function_name).as_str() {
                        "&" => vec![I64_AND],
                        "|" => vec![I64_OR],
                        "^" => vec![I64_XOR],
                        "<<" => vec![I64_SHL],
                        ">>" => vec![I64_SHR_S],
                        _ => panic!("unexpected operator"),
                    };
                    f.extend(vec![F64_CONVERT_S_I64]);
                    self.function_implementations[i].with_instructions(f);
                } else if &x.function_name == "+"
                    || &x.function_name == "-"
                    || &x.function_name == "*"
                    || &x.function_name == "/"
                    || &x.function_name == "%"
                {
                    if x.params.len() < 2 {
                        panic!(
                            "operator {} expected at least 2 parameters",
                            (&x.function_name).as_str()
                        );
                    }
                    for p in 0..x.params.len() {
                        self.process_expression(i, &x.params[p]);

                        if &x.function_name == "%" {
                            self.function_implementations[i]
                                .with_instructions(vec![I64_TRUNC_S_F64]);
                        }
                        if p != 0 {
                            let f = match (&x.function_name).as_str() {
                                "+" => vec![F64_ADD],
                                "-" => vec![F64_SUB],
                                "*" => vec![F64_MUL],
                                "/" => vec![F64_DIV],
                                "%" => vec![I64_REM_S, F64_CONVERT_S_I64],
                                _ => panic!("unexpected operator"),
                            };
                            self.function_implementations[i].with_instructions(f);
                        }
                    }
                } else if &x.function_name == "!" {
                    if x.params.len() != 1 {
                        panic!(
                            "operator {} expected 1 parameters",
                            (&x.function_name).as_str()
                        );
                    }

                    self.process_expression(i, &x.params[0]);
                    self.function_implementations[i].with_instructions(vec![
                        F64_CONST,
                        0.0.into(),
                        F64_EQ,
                        F64_CONVERT_S_I32,
                    ]);
                } else if &x.function_name == "~" {
                    if x.params.len() != 1 {
                        panic!(
                            "operator {} expected 1 parameters",
                            (&x.function_name).as_str()
                        );
                    }

                    self.process_expression(i, &x.params[0]);
                    self.function_implementations[i].with_instructions(vec![
                        I64_TRUNC_S_F64,
                        I64_CONST,
                        (-1 as i32).into(),
                        I64_XOR,
                        F64_CONVERT_S_I64,
                    ]);
                } else if &x.function_name == "and" {
                    if x.params.len() != 2 {
                        panic!(
                            "operator {} expected 2 parameters",
                            (&x.function_name).as_str()
                        );
                    }

                    self.process_expression(i, &x.params[0]);
                    self.function_implementations[i].with_instructions(vec![
                        I64_TRUNC_S_F64,
                        I64_CONST,
                        0.into(),
                        I64_NE,
                    ]);
                    self.process_expression(i, &x.params[1]);
                    self.function_implementations[i].with_instructions(vec![
                        I64_TRUNC_S_F64,
                        I64_CONST,
                        0.into(),
                        I64_NE,
                        I32_AND,
                        F64_CONVERT_S_I32,
                    ]);
                } else if &x.function_name == "or" {
                    if x.params.len() != 2 {
                        panic!(
                            "operator {} expected 2 parameters",
                            (&x.function_name).as_str()
                        );
                    }

                    self.process_expression(i, &x.params[0]);
                    self.function_implementations[i].with_instructions(vec![I64_TRUNC_S_F64]);
                    self.process_expression(i, &x.params[1]);
                    self.function_implementations[i].with_instructions(vec![
                        I64_TRUNC_S_F64,
                        I64_OR,
                        I64_CONST,
                        0.into(),
                        I64_NE,
                        F64_CONVERT_S_I32,
                    ]);
                } else {
                    let (function_handle, _) = self.resolve_identifier(&x.function_name);
                    for k in 0..x.params.len() {
                        self.process_expression(i, &x.params[k])
                    }
                    self.function_implementations[i]
                        .with_instructions(vec![CALL, (function_handle as i32).into()]);
                }
            }
            Expression::TextLiteral(x) => {
                let pos = self.get_or_create_text_data(&x);
                self.function_implementations[i]
                    .with_instructions(vec![F64_CONST, (pos as f64).into()]);
            }
            Expression::Identifier(x) => {
                let val = self.resolve_identifier(&x);
                match val.1 {
                    IdentifierType::Global => {
                        self.function_implementations[i]
                            .with_instructions(vec![F64_CONST, val.0.into()]);
                    }
                    IdentifierType::Local => {
                        self.function_implementations[i]
                            .with_instructions(vec![LOCAL_GET, (val.0 as i32).into()]);
                    }
                    IdentifierType::Function => {
                        self.function_implementations[i]
                            .with_instructions(vec![F64_CONST, val.0.into()]);
                    }
                }
            }
            Expression::Comment(_) => {}
            Expression::Number(x) => {
                self.function_implementations[i].with_instructions(vec![F64_CONST, (*x).into()]);
            }
        }
    }

    fn process_functions(&mut self) {
        // now lets process the insides of our functions
        for i in 0..self.function_defs.len() {
            if let TopLevelOperation::DefineFunction(f) = self.function_defs[i].clone() {
                self.local_names = f.params.clone();
                for j in 0..f.children.len() {
                    self.process_expression(i, &f.children[j].clone());
                    if j != f.children.len() - 1 {
                        self.function_implementations[i].with_instructions(vec![DROP]);
                    }
                }
                //end the function
                self.function_implementations[i].with_instructions(vec![END]);
            } else if let TopLevelOperation::DefineTestFunction(f) = self.function_defs[i].clone() {
                self.function_implementations[i].with_local(DataType::F64);
                self.local_names = vec![("").to_string()];
                self.function_implementations[i].with_instructions(vec![BLOCK, F64]);
                for j in 0..f.children.len() {
                    self.process_expression(i, &f.children[j].clone());
                    self.function_implementations[i].with_instructions(vec![
                        LOCAL_SET,
                        (0 as i32).into(),
                        LOCAL_GET,
                        (0 as i32).into(),
                        F64_CONST,
                        0.0.into(),
                        F64_NE,
                        IF,
                        EMPTY,
                        LOCAL_GET,
                        (0 as i32).into(),
                        BR,
                        (1 as i32).into(),
                        END,
                    ]);
                }
                self.function_implementations[i].with_instructions(vec![
                    F64_CONST,
                    0.0.into(),
                    END,
                    END,
                ]);
            }
        }

        //now that we are done with everything, put funcions in the app
        let num_funcs = self.function_defs.len();
        for _ in 0..num_funcs {
            let f = self.function_implementations.remove(0);
            self.wasm.add_function(f);
        }

        self.wasm.add_elements(
            0,
            self.function_names
                .iter()
                .enumerate()
                .map(|(i, _)| Element::new(i as u32))
                .collect::<Vec<Element>>(),
        )
    }

    fn complete(&mut self) -> Vec<u8> {
        self.wasm.to_bytes()
    }
}

pub fn compile(app: crate::ast::App) -> Result<Vec<u8>, Error> {
    let mut compiler = Compiler::new(app);
    compiler.pre_process_functions();
    compiler.process_globals();
    compiler.process_functions();
    compiler.set_heap_start();
    Ok(compiler.complete())
}
