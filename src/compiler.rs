use crate::ast::*;
use byteorder::{LittleEndian, WriteBytesExt};
use failure::Error;
use wasmly::WebAssembly::*;
use wasmly::*;

enum IdentifierType {
    Global,
    Local,
    Function,
}

fn to_wasm(op: WasmOperation) -> Option<WebAssembly> {
    match op {
        WasmOperation::Identifier(x) => match x.as_str() {
            "I32" => Some(WebAssembly::I32),
            "I64" => Some(WebAssembly::I64),
            "F32" => Some(WebAssembly::F32),
            "F64" => Some(WebAssembly::F64),
            "ANYFUNC" => Some(WebAssembly::ANYFUNC),
            "FUNC" => Some(WebAssembly::FUNC),
            "EMPTY" => Some(WebAssembly::EMPTY),
            "SECTION_CUSTOM" => Some(WebAssembly::SECTION_CUSTOM),
            "SECTION_TYPE" => Some(WebAssembly::SECTION_TYPE),
            "SECTION_IMPORT" => Some(WebAssembly::SECTION_IMPORT),
            "SECTION_FUNCTION" => Some(WebAssembly::SECTION_FUNCTION),
            "SECTION_TABLE" => Some(WebAssembly::SECTION_TABLE),
            "SECTION_MEMORY" => Some(WebAssembly::SECTION_MEMORY),
            "SECTION_GLOBAL" => Some(WebAssembly::SECTION_GLOBAL),
            "SECTION_EXPORT" => Some(WebAssembly::SECTION_EXPORT),
            "SECTION_START" => Some(WebAssembly::SECTION_START),
            "SECTION_ELEMENT" => Some(WebAssembly::SECTION_ELEMENT),
            "SECTION_CODE" => Some(WebAssembly::SECTION_CODE),
            "SECTION_DATA" => Some(WebAssembly::SECTION_DATA),
            "UNREACHABLE" => Some(WebAssembly::UNREACHABLE),
            "NOP" => Some(WebAssembly::NOP),
            "BLOCK" => Some(WebAssembly::BLOCK),
            "LOOP" => Some(WebAssembly::LOOP),
            "IF" => Some(WebAssembly::IF),
            "ELSE" => Some(WebAssembly::ELSE),
            "END" => Some(WebAssembly::END),
            "BR" => Some(WebAssembly::BR),
            "BR_IF" => Some(WebAssembly::BR_IF),
            "BR_TABLE" => Some(WebAssembly::BR_TABLE),
            "RETURN" => Some(WebAssembly::RETURN),
            "CALL" => Some(WebAssembly::CALL),
            "CALL_INDIRECT" => Some(WebAssembly::CALL_INDIRECT),
            "DROP" => Some(WebAssembly::DROP),
            "SELECT" => Some(WebAssembly::SELECT),
            "LOCAL_GET" => Some(WebAssembly::LOCAL_GET),
            "LOCAL_SET" => Some(WebAssembly::LOCAL_SET),
            "LOCAL_TEE" => Some(WebAssembly::LOCAL_TEE),
            "GLOBAL_GET" => Some(WebAssembly::GLOBAL_GET),
            "GLOBAL_SET" => Some(WebAssembly::GLOBAL_SET),
            "I32_LOAD" => Some(WebAssembly::I32_LOAD),
            "I64_LOAD" => Some(WebAssembly::I64_LOAD),
            "F32_LOAD" => Some(WebAssembly::F32_LOAD),
            "F64_LOAD" => Some(WebAssembly::F64_LOAD),
            "I32_LOAD8_S" => Some(WebAssembly::I32_LOAD8_S),
            "I32_LOAD8_U" => Some(WebAssembly::I32_LOAD8_U),
            "I32_LOAD16_S" => Some(WebAssembly::I32_LOAD16_S),
            "I32_LOAD16_U" => Some(WebAssembly::I32_LOAD16_U),
            "I64_LOAD8_S" => Some(WebAssembly::I64_LOAD8_S),
            "I64_LOAD8_U" => Some(WebAssembly::I64_LOAD8_U),
            "I64_LOAD16_S" => Some(WebAssembly::I64_LOAD16_S),
            "I64_LOAD16_U" => Some(WebAssembly::I64_LOAD16_U),
            "I64_LOAD32_S" => Some(WebAssembly::I64_LOAD32_S),
            "I64_LOAD32_U" => Some(WebAssembly::I64_LOAD32_U),
            "I32_STORE" => Some(WebAssembly::I32_STORE),
            "I64_STORE" => Some(WebAssembly::I64_STORE),
            "F32_STORE" => Some(WebAssembly::F32_STORE),
            "F64_STORE" => Some(WebAssembly::F64_STORE),
            "I32_STORE8" => Some(WebAssembly::I32_STORE8),
            "I32_STORE16" => Some(WebAssembly::I32_STORE16),
            "I64_STORE8" => Some(WebAssembly::I64_STORE8),
            "I64_STORE16" => Some(WebAssembly::I64_STORE16),
            "I64_STORE32" => Some(WebAssembly::I64_STORE32),
            "CURRENT_MEMORY" => Some(WebAssembly::CURRENT_MEMORY),
            "GROW_MEMORY" => Some(WebAssembly::GROW_MEMORY),
            "I32_CONST" => Some(WebAssembly::I32_CONST),
            "I64_CONST" => Some(WebAssembly::I64_CONST),
            "F32_CONST" => Some(WebAssembly::F32_CONST),
            "F64_CONST" => Some(WebAssembly::F64_CONST),
            "I32_EQZ" => Some(WebAssembly::I32_EQZ),
            "I32_EQ" => Some(WebAssembly::I32_EQ),
            "I32_NE" => Some(WebAssembly::I32_NE),
            "I32_LT_S" => Some(WebAssembly::I32_LT_S),
            "I32_LT_U" => Some(WebAssembly::I32_LT_U),
            "I32_GT_S" => Some(WebAssembly::I32_GT_S),
            "I32_GT_U" => Some(WebAssembly::I32_GT_U),
            "I32_LE_S" => Some(WebAssembly::I32_LE_S),
            "I32_LE_U" => Some(WebAssembly::I32_LE_U),
            "I32_GE_S" => Some(WebAssembly::I32_GE_S),
            "I32_GE_U" => Some(WebAssembly::I32_GE_U),
            "I64_EQZ" => Some(WebAssembly::I64_EQZ),
            "I64_EQ" => Some(WebAssembly::I64_EQ),
            "I64_NE" => Some(WebAssembly::I64_NE),
            "I64_LT_S" => Some(WebAssembly::I64_LT_S),
            "I64_LT_U" => Some(WebAssembly::I64_LT_U),
            "I64_GT_S" => Some(WebAssembly::I64_GT_S),
            "I64_GT_U" => Some(WebAssembly::I64_GT_U),
            "I64_LE_S" => Some(WebAssembly::I64_LE_S),
            "I64_LE_U" => Some(WebAssembly::I64_LE_U),
            "I64_GE_S" => Some(WebAssembly::I64_GE_S),
            "I64_GE_U" => Some(WebAssembly::I64_GE_U),
            "F32_EQ" => Some(WebAssembly::F32_EQ),
            "F32_NE" => Some(WebAssembly::F32_NE),
            "F32_LT" => Some(WebAssembly::F32_LT),
            "F32_GT" => Some(WebAssembly::F32_GT),
            "F32_LE" => Some(WebAssembly::F32_LE),
            "F32_GE" => Some(WebAssembly::F32_GE),
            "F64_EQ" => Some(WebAssembly::F64_EQ),
            "F64_NE" => Some(WebAssembly::F64_NE),
            "F64_LT" => Some(WebAssembly::F64_LT),
            "F64_GT" => Some(WebAssembly::F64_GT),
            "F64_LE" => Some(WebAssembly::F64_LE),
            "F64_GE" => Some(WebAssembly::F64_GE),
            "I32_CLZ" => Some(WebAssembly::I32_CLZ),
            "I32_CTZ" => Some(WebAssembly::I32_CTZ),
            "I32_POPCNT" => Some(WebAssembly::I32_POPCNT),
            "I32_ADD" => Some(WebAssembly::I32_ADD),
            "I32_SUB" => Some(WebAssembly::I32_SUB),
            "I32_MUL" => Some(WebAssembly::I32_MUL),
            "I32_DIV_S" => Some(WebAssembly::I32_DIV_S),
            "I32_DIV_U" => Some(WebAssembly::I32_DIV_U),
            "I32_REM_S" => Some(WebAssembly::I32_REM_S),
            "I32_REM_U" => Some(WebAssembly::I32_REM_U),
            "I32_AND" => Some(WebAssembly::I32_AND),
            "I32_OR" => Some(WebAssembly::I32_OR),
            "I32_XOR" => Some(WebAssembly::I32_XOR),
            "I32_SHL" => Some(WebAssembly::I32_SHL),
            "I32_SHR_S" => Some(WebAssembly::I32_SHR_S),
            "I32_SHR_U" => Some(WebAssembly::I32_SHR_U),
            "I32_ROTL" => Some(WebAssembly::I32_ROTL),
            "I32_ROTR" => Some(WebAssembly::I32_ROTR),
            "I64_CLZ" => Some(WebAssembly::I64_CLZ),
            "I64_CTZ" => Some(WebAssembly::I64_CTZ),
            "I64_POPCNT" => Some(WebAssembly::I64_POPCNT),
            "I64_ADD" => Some(WebAssembly::I64_ADD),
            "I64_SUB" => Some(WebAssembly::I64_SUB),
            "I64_MUL" => Some(WebAssembly::I64_MUL),
            "I64_DIV_S" => Some(WebAssembly::I64_DIV_S),
            "I64_DIV_U" => Some(WebAssembly::I64_DIV_U),
            "I64_REM_S" => Some(WebAssembly::I64_REM_S),
            "I64_REM_U" => Some(WebAssembly::I64_REM_U),
            "I64_AND" => Some(WebAssembly::I64_AND),
            "I64_OR" => Some(WebAssembly::I64_OR),
            "I64_XOR" => Some(WebAssembly::I64_XOR),
            "I64_SHL" => Some(WebAssembly::I64_SHL),
            "I64_SHR_S" => Some(WebAssembly::I64_SHR_S),
            "I64_SHR_U" => Some(WebAssembly::I64_SHR_U),
            "I64_ROTL" => Some(WebAssembly::I64_ROTL),
            "I64_ROTR" => Some(WebAssembly::I64_ROTR),
            "F32_ABS" => Some(WebAssembly::F32_ABS),
            "F32_NEG" => Some(WebAssembly::F32_NEG),
            "F32_CEIL" => Some(WebAssembly::F32_CEIL),
            "F32_FLOOR" => Some(WebAssembly::F32_FLOOR),
            "F32_TRUNC" => Some(WebAssembly::F32_TRUNC),
            "F32_NEAREST" => Some(WebAssembly::F32_NEAREST),
            "F32_SQRT" => Some(WebAssembly::F32_SQRT),
            "F32_ADD" => Some(WebAssembly::F32_ADD),
            "F32_SUB" => Some(WebAssembly::F32_SUB),
            "F32_MUL" => Some(WebAssembly::F32_MUL),
            "F32_DIV" => Some(WebAssembly::F32_DIV),
            "F32_MIN" => Some(WebAssembly::F32_MIN),
            "F32_MAX" => Some(WebAssembly::F32_MAX),
            "F32_COPYSIGN" => Some(WebAssembly::F32_COPYSIGN),
            "F64_ABS" => Some(WebAssembly::F64_ABS),
            "F64_NEG" => Some(WebAssembly::F64_NEG),
            "F64_CEIL" => Some(WebAssembly::F64_CEIL),
            "F64_FLOOR" => Some(WebAssembly::F64_FLOOR),
            "F64_TRUNC" => Some(WebAssembly::F64_TRUNC),
            "F64_NEAREST" => Some(WebAssembly::F64_NEAREST),
            "F64_SQRT" => Some(WebAssembly::F64_SQRT),
            "F64_ADD" => Some(WebAssembly::F64_ADD),
            "F64_SUB" => Some(WebAssembly::F64_SUB),
            "F64_MUL" => Some(WebAssembly::F64_MUL),
            "F64_DIV" => Some(WebAssembly::F64_DIV),
            "F64_MIN" => Some(WebAssembly::F64_MIN),
            "F64_MAX" => Some(WebAssembly::F64_MAX),
            "F64_COPYSIGN" => Some(WebAssembly::F64_COPYSIGN),
            "I32_WRAP_F64" => Some(WebAssembly::I32_WRAP_F64),
            "I32_TRUNC_S_F32" => Some(WebAssembly::I32_TRUNC_S_F32),
            "I32_TRUNC_U_F32" => Some(WebAssembly::I32_TRUNC_U_F32),
            "I32_TRUNC_S_F64" => Some(WebAssembly::I32_TRUNC_S_F64),
            "I32_TRUNC_U_F64" => Some(WebAssembly::I32_TRUNC_U_F64),
            "I64_EXTEND_S_I32" => Some(WebAssembly::I64_EXTEND_S_I32),
            "I64_EXTEND_U_I32" => Some(WebAssembly::I64_EXTEND_U_I32),
            "I64_TRUNC_S_F32" => Some(WebAssembly::I64_TRUNC_S_F32),
            "I64_TRUNC_U_F32" => Some(WebAssembly::I64_TRUNC_U_F32),
            "I64_TRUNC_S_F64" => Some(WebAssembly::I64_TRUNC_S_F64),
            "I64_TRUNC_U_F64" => Some(WebAssembly::I64_TRUNC_U_F64),
            "F32_CONVERT_S_I32" => Some(WebAssembly::F32_CONVERT_S_I32),
            "F32_CONVERT_U_I32" => Some(WebAssembly::F32_CONVERT_U_I32),
            "F32_CONVERT_S_I64" => Some(WebAssembly::F32_CONVERT_S_I64),
            "F32_CONVERT_U_I64" => Some(WebAssembly::F32_CONVERT_U_I64),
            "F32_DEMOTE_F64" => Some(WebAssembly::F32_DEMOTE_F64),
            "F64_CONVERT_S_I32" => Some(WebAssembly::F64_CONVERT_S_I32),
            "F64_CONVERT_U_I32" => Some(WebAssembly::F64_CONVERT_U_I32),
            "F64_CONVERT_S_I64" => Some(WebAssembly::F64_CONVERT_S_I64),
            "F64_CONVERT_U_I64" => Some(WebAssembly::F64_CONVERT_U_I64),
            "F64_PROMOTE_F32" => Some(WebAssembly::F64_PROMOTE_F32),
            "I32_REINTERPRET_F32" => Some(WebAssembly::I32_REINTERPRET_F32),
            "I64_REINTERPRET_F64" => Some(WebAssembly::I64_REINTERPRET_F64),
            "F32_REINTERPRET_I32" => Some(WebAssembly::F32_REINTERPRET_I32),
            "F64_REINTERPRET_I64" => Some(WebAssembly::F64_REINTERPRET_I64),
            "DESC_FUNCTION" => Some(WebAssembly::DESC_FUNCTION),
            "DESC_TABLE" => Some(WebAssembly::DESC_TABLE),
            "DESC_MEMORY" => Some(WebAssembly::DESC_MEMORY),
            "DESC_GLOBAL" => Some(WebAssembly::DESC_GLOBAL),
            "LIMIT_MIN" => Some(WebAssembly::LIMIT_MIN),
            "LIMIT_MIN_MAX" => Some(WebAssembly::LIMIT_MIN_MAX),
            "IMMUTABLE" => Some(WebAssembly::IMMUTABLE),
            "MUTABLE" => Some(WebAssembly::MUTABLE),
            "EMPTY_VEC" => Some(WebAssembly::EMPTY_VEC),
            _ => panic!(format!("unknown wasm opcode:{}", x)),
        },
        WasmOperation::Number(x) => Some(wasmly::int(x)),
        WasmOperation::Comment(_) => None,
    }
}

struct Compiler {
    wasm: wasmly::App,
    ast: crate::ast::App,
    global_names: Vec<String>,
    global_values: Vec<i32>,
    local_names: Vec<String>,
    heap_position: i32,
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
            global_names: vec![],
            global_values: vec![],
            local_names: vec![],
            heap_position: 4, //start at 4 so nothing has 0 address
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
                def.params.iter().map(|_| DataType::I32).collect(),
                Some(DataType::I32),
            )))
        }
        self.wasm = wasmly::App::new(imports);
        self.function_defs = self
            .ast
            .children
            .iter()
            .filter_map(|x| match x {
                TopLevelOperation::DefineFunction(_) => Some(x.clone()),
                TopLevelOperation::DefineWasmFunction(_) => Some(x.clone()),
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

    fn int_to_bytes(&self, i: i32) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.write_i32::<LittleEndian>(i).unwrap();
        bytes
    }

    fn create_global_data(&mut self, v: Vec<GlobalValue>) -> i32 {
        let mut bytes = vec![];
        for i in 0..v.len() {
            let v = self.get_global_value(&v[i]);
            let b = self.int_to_bytes(v);
            bytes.extend_from_slice(&b);
        }
        self.create_data(bytes)
    }

    fn get_global_value(&mut self, v: &GlobalValue) -> i32 {
        match v {
            GlobalValue::Number(t) => *t,
            GlobalValue::Text(t) => self.get_or_create_text_data(&t),
            GlobalValue::Data(t) => self.create_global_data(t.clone()),
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
            } else if let TopLevelOperation::DefineWasmFunction(function_def) =
                &self.function_defs[i]
            {
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
                function.with_inputs(function_def.params.iter().map(|_| DataType::I32).collect());
                function.with_output(DataType::I32);
                self.function_implementations.push(function);
            } else if let TopLevelOperation::DefineWasmFunction(function_def) =
                &self.function_defs[i]
            {
                let mut function = Function::new();
                if function_def.exported {
                    function.with_name(&function_def.name);
                }
                function.with_inputs(function_def.params.clone());
                if !function_def.outputs.is_empty() {
                    function.with_output(function_def.outputs[0].clone())
                }
                self.function_implementations.push(function);
            } else if let TopLevelOperation::DefineTestFunction(function_def) =
                &self.function_defs[i]
            {
                let mut function = Function::new();
                function.with_name(&format!("test_{}", function_def.name));
                function.with_output(DataType::I32);
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
            if self.heap_position % 4 != 0 {
                (self.heap_position / 4) * 4 + 4
            } else {
                self.heap_position
            }
        };
        self.wasm
            .add_global(wasmly::Global::new(final_heap_pos as i32, false));
        self.wasm
            .add_global(wasmly::Global::new(final_heap_pos as i32, true));
    }

    fn get_or_create_text_data(&mut self, str: &str) -> i32 {
        let mut bytes: Vec<u8> = str.as_bytes().into();
        bytes.push(0);
        self.create_data(bytes)
    }

    fn create_data(&mut self, bytes: Vec<u8>) -> i32 {
        let pos = self.heap_position;
        let size = bytes.len();
        self.wasm.add_data(Data::new(pos, bytes));
        let mut final_heap_pos = self.heap_position + (size as i32);
        // align data to 4
        // TODO: verify if this actually matters
        if final_heap_pos % 4 != 0 {
            final_heap_pos = (final_heap_pos / 4) * 4 + 4;
        }
        self.heap_position = final_heap_pos;
        pos
    }

    fn resolve_identifier(&self, id: &str) -> (i32, IdentifierType) {
        // look this up in reverse so shadowing works
        let mut p = self.local_names.iter().rev().position(|r| r == id);
        if p.is_some() {
            return (
                self.local_names.len() as i32 - 1 - p.unwrap() as i32,
                IdentifierType::Local,
            );
        }
        p = self.function_names.iter().position(|r| r == id);
        if p.is_some() {
            return (p.unwrap() as i32, IdentifierType::Function);
        }
        p = self.global_names.iter().position(|r| r == id);
        if p.is_some() {
            return (self.global_values[p.unwrap()], IdentifierType::Global);
        }
        panic!(format!("could not find identifier \"{}\"", id))
    }

    fn process_wasm(&mut self, i: usize, e: &[WasmOperation]) {
        let wasm = e.iter().filter_map(|x| to_wasm(x.clone())).collect();
        self.function_implementations[i].with_instructions(wasm);
    }

    #[allow(clippy::cyclomatic_complexity)]
    fn process_expression(&mut self, i: usize, e: &Expression) {
        match e {
            Expression::Populate(x) => {
                let val = self.resolve_identifier(&x.name);
                self.function_implementations[i].with_local(DataType::I32);
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
                                    I32_CONST,
                                    0.into(),
                                    CALL,
                                    val.0.into(),
                                    LOCAL_SET,
                                    loc_storage.into(),
                                ]);
                            } else if j == expr.len() - 1 {
                                for k in 0..expr[j].len() {
                                    self.process_expression(i, &expr[j][k])
                                }
                                self.function_implementations[i].with_instructions(vec![
                                    LOCAL_GET,
                                    loc_storage.into(),
                                    CALL,
                                    val.0.into(),
                                ]);
                                break;
                            } else {
                                for k in 0..expr[j].len() {
                                    self.process_expression(i, &expr[j][k])
                                }
                                self.function_implementations[i].with_instructions(vec![
                                    LOCAL_GET,
                                    loc_storage.into(),
                                    CALL,
                                    val.0.into(),
                                    LOCAL_SET,
                                    loc_storage.into(),
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
                self.function_implementations[i].with_instructions(vec![I32_CONST, t.into()]);
            }
            Expression::Loop(x) => {
                self.recur_depth = 0;
                for j in 0..x.bindings.len() {
                    let binding = &x.bindings[j];
                    self.process_expression(i, &binding.1);
                    self.function_implementations[i].with_local(DataType::I32);
                    self.function_implementations[i]
                        .with_instructions(vec![LOCAL_SET, (self.local_names.len() as u32).into()]);
                    self.local_names.push((&binding.0).to_string());
                }
                if !x.expressions.is_empty() {
                    self.function_implementations[i].with_instructions(vec![LOOP, I32]);
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
                                .with_instructions(vec![LOCAL_SET, val.0.into()]);
                        }
                        _ => panic!("cannot recur by rebinding a non-local identifier"),
                    }
                }
                self.function_implementations[i].with_instructions(vec![
                    I32_CONST,
                    0.into(),
                    BR,
                    self.recur_depth.into(),
                ]);
            }
            Expression::Let(x) => {
                for j in 0..x.bindings.len() {
                    let binding = &x.bindings[j];
                    self.process_expression(i, &binding.1);
                    self.function_implementations[i].with_local(DataType::I32);
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
                                    .with_instructions(vec![I32_CONST, 0.into()]);
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
                        self.function_implementations[i].with_instructions(vec![IF, I32]);
                        self.process_expression(i, &x.params[1]);
                        self.function_implementations[i].with_instructions(vec![
                            ELSE,
                            I32_CONST,
                            0.into(),
                            END,
                        ]);
                    } else if x.params.len() == 3 {
                        self.process_expression(i, &x.params[0]);
                        self.function_implementations[i].with_instructions(vec![IF, I32]);
                        self.process_expression(i, &x.params[1]);
                        self.function_implementations[i].with_instructions(vec![ELSE]);
                        self.process_expression(i, &x.params[2]);
                        self.function_implementations[i].with_instructions(vec![END]);
                    } else {
                        panic!("invalid number of params for if")
                    }
                } else if &x.function_name == "mem" {
                    if x.params.len() == 1 {
                        for k in 0..x.params.len() {
                            self.process_expression(i, &x.params[k])
                        }
                        self.function_implementations[i].with_instructions(vec![
                            I32_LOAD8_U,
                            0.into(),
                            0.into(),
                        ]);
                    } else if x.params.len() == 2 {
                        for k in 0..x.params.len() {
                            self.process_expression(i, &x.params[k])
                        }
                        self.function_implementations[i].with_instructions(vec![
                            I32_STORE8,
                            0.into(),
                            0.into(),
                        ]);
                        self.function_implementations[i]
                            .with_instructions(vec![I32_CONST, 0.into()]);
                    } else {
                        panic!("invalid number params for mem")
                    }
                } else if &x.function_name == "mem32" {
                    if x.params.len() == 1 {
                        for k in 0..x.params.len() {
                            self.process_expression(i, &x.params[k])
                        }
                        self.function_implementations[i].with_instructions(vec![
                            I32_LOAD,
                            0.into(),
                            0.into(),
                        ]);
                    } else if x.params.len() == 2 {
                        for k in 0..x.params.len() {
                            self.process_expression(i, &x.params[k])
                        }
                        self.function_implementations[i].with_instructions(vec![
                            I32_STORE,
                            0.into(),
                            0.into(),
                        ]);
                        self.function_implementations[i]
                            .with_instructions(vec![I32_CONST, 0.into()]);
                    } else {
                        panic!("invalid number params for mem")
                    }
                } else if &x.function_name == "=="
                    || &x.function_name == "!="
                    || &x.function_name == "<="
                    || &x.function_name == ">="
                    || &x.function_name == "<"
                    || &x.function_name == ">"
                    || &x.function_name == "&"
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
                    self.process_expression(i, &x.params[1]);
                    let f = match (&x.function_name).as_str() {
                        "==" => vec![I32_EQ],
                        "!=" => vec![I32_NE],
                        "<=" => vec![I32_LE_S],
                        ">=" => vec![I32_GE_S],
                        "<" => vec![I32_LT_S],
                        ">" => vec![I32_GT_S],
                        "&" => vec![I32_AND],
                        "|" => vec![I32_OR],
                        "^" => vec![I32_XOR],
                        "<<" => vec![I32_SHL],
                        ">>" => vec![I32_SHR_S],
                        _ => panic!("unexpected operator"),
                    };
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
                        let f = match (&x.function_name).as_str() {
                            "+" => vec![I32_ADD],
                            "-" => vec![I32_SUB],
                            "*" => vec![I32_MUL],
                            "/" => vec![I32_DIV_S],
                            "%" => vec![I32_REM_S],
                            _ => panic!("unexpected operator"),
                        };
                        self.process_expression(i, &x.params[p]);
                        if p != 0 {
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
                    self.function_implementations[i].with_instructions(vec![I32_EQZ]);
                } else if &x.function_name == "~" {
                    if x.params.len() != 1 {
                        panic!(
                            "operator {} expected 1 parameters",
                            (&x.function_name).as_str()
                        );
                    }

                    self.process_expression(i, &x.params[0]);
                    self.function_implementations[i].with_instructions(vec![
                        I32_CONST,
                        (-1 as i32).into(),
                        I32_XOR,
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
                        I32_CONST,
                        0.into(),
                        I32_NE,
                    ]);
                    self.process_expression(i, &x.params[1]);
                    self.function_implementations[i].with_instructions(vec![
                        I32_CONST,
                        0.into(),
                        I32_NE,
                        I32_AND,
                    ]);
                } else if &x.function_name == "or" {
                    if x.params.len() != 2 {
                        panic!(
                            "operator {} expected 2 parameters",
                            (&x.function_name).as_str()
                        );
                    }

                    self.process_expression(i, &x.params[0]);
                    self.process_expression(i, &x.params[1]);
                    self.function_implementations[i].with_instructions(vec![
                        I32_OR,
                        I32_CONST,
                        0.into(),
                        I32_NE,
                    ]);
                } else {
                    let (function_handle, _) = self.resolve_identifier(&x.function_name);
                    for k in 0..x.params.len() {
                        self.process_expression(i, &x.params[k])
                    }
                    self.function_implementations[i]
                        .with_instructions(vec![CALL, function_handle.into()]);
                }
            }
            Expression::TextLiteral(x) => {
                let pos = self.get_or_create_text_data(&x);
                self.function_implementations[i].with_instructions(vec![I32_CONST, pos.into()]);
            }
            Expression::Identifier(x) => {
                let val = self.resolve_identifier(&x);
                match val.1 {
                    IdentifierType::Global => {
                        self.function_implementations[i]
                            .with_instructions(vec![I32_CONST, val.0.into()]);
                    }
                    IdentifierType::Local => {
                        self.function_implementations[i]
                            .with_instructions(vec![LOCAL_GET, val.0.into()]);
                    }
                    IdentifierType::Function => {
                        self.function_implementations[i]
                            .with_instructions(vec![I32_CONST, val.0.into()]);
                    }
                }
            }
            Expression::Comment(_) => {}
            Expression::Number(x) => {
                self.function_implementations[i].with_instructions(vec![I32_CONST, (*x).into()]);
            }
            Expression::EmptyList => {
                self.function_implementations[i].with_instructions(vec![I32_CONST, 0.into()]);
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
            } else if let TopLevelOperation::DefineWasmFunction(f) = self.function_defs[i].clone() {
                for j in 0..f.locals.len() {
                    self.function_implementations[i].with_local(f.locals[j].clone());
                }
                self.process_wasm(i, &f.children.clone());
            } else if let TopLevelOperation::DefineTestFunction(f) = self.function_defs[i].clone() {
                self.function_implementations[i].with_local(DataType::I32);
                self.local_names = vec![("").to_string()];
                self.function_implementations[i].with_instructions(vec![BLOCK, I32]);
                for j in 0..f.children.len() {
                    self.process_expression(i, &f.children[j].clone());
                    self.function_implementations[i].with_instructions(vec![
                        LOCAL_SET,
                        0.into(),
                        LOCAL_GET,
                        0.into(),
                        I32_CONST,
                        0.into(),
                        I32_NE,
                        IF,
                        EMPTY,
                        LOCAL_GET,
                        0.into(),
                        BR,
                        1.into(),
                        END,
                    ]);
                }
                self.function_implementations[i].with_instructions(vec![
                    I32_CONST,
                    0.into(),
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
