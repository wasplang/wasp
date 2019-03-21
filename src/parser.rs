use crate::ast::*;
use failure::Error;
use nom::types::CompleteStr;
use std::str;
use wasmly::DataType;

fn to_string(s: CompleteStr) -> String {
    s.to_string()
}

fn is_start_identifier_char(c: char) -> bool {
    c == '_' || c == '$' || c.is_alphabetic()
}

fn is_identifier_char(c: char) -> bool {
    c == '_' || c == '!' || c == '-' || c == '$' || c.is_alphanumeric()
}

fn is_text_char(c: char) -> bool {
    c != '"'
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn is_comment_char(c: char) -> bool {
    c != '\r' && c != '\n'
}

fn to_data_type(c: &str) -> DataType {
    match c {
        "i32" => DataType::I32,
        "i64" => DataType::I64,
        "f32" => DataType::F32,
        "f64" => DataType::F64,
        _ => panic!("invalid type"),
    }
}

named!(
    token_comment<CompleteStr,String>,
    do_parse!(
        pair: pair!(tag!(";"),take_while!(is_comment_char))>>
        (pair.0.to_string())
    )
);

named!(
    token_identifier<CompleteStr,String>,
    do_parse!(
        start: map!(take_while1!(is_start_identifier_char), to_string) >>
        end: map!(take_while!(is_identifier_char), to_string) >>
        (format!("{}{}",start,end).to_string())
    )
);

named!(
    function_identifiers<CompleteStr,String>,
    do_parse!(
        id: alt!(map!(tag!("call"),to_string)|token_identifier|map!(tag!("do"),to_string)|map!(tag!("if"),to_string)|map!(tag!(">>"),to_string)|map!(tag!("<<"),to_string)|map!(tag!(">="),to_string)|map!(tag!("<="),to_string)|map!(tag!(">"),to_string)|map!(tag!("<"),to_string)|map!(tag!("or"),to_string)|map!(tag!("and"),to_string)|map!(tag!("!="),to_string)|map!(tag!("=="),to_string)|map!(tag!("+"),to_string)|map!(tag!("-"),to_string)|map!(tag!("*"),to_string)|map!(tag!("/"),to_string)|map!(tag!("%"),to_string)|map!(tag!("|"),to_string)|map!(tag!("&"),to_string)|map!(tag!("^"),to_string)|map!(tag!("~"),to_string)|map!(tag!("!"),to_string))>>
        (id)
    )
);

named!(
    token_data_type<CompleteStr,DataType>,
    do_parse!(
        t: map!(alt!(tag!("i32")|tag!("i64")|tag!("f32")|tag!("f64")), to_string) >>
        (to_data_type(&t))
    )
);

named!(
    token_text<CompleteStr,String>,
    do_parse!(
        tag!("\"")
            >> text: map!(take_while!(is_text_char), to_string)
            >> tag!("\"")
            >> (text)
    )
);

named!(
    token_symbol<CompleteStr,String>,
    do_parse!(
        tag!(":")
            >> text: map!(take_while!(is_identifier_char), to_string)
            >> (text)
    )
);

named!(
    negative_number<CompleteStr,i32>,
    do_parse!(
        tag!("-")
            >> num: map!(take_while1!(is_digit), to_string)
            >> (-num.parse::<i32>().unwrap())
    )
);

named!(
    positive_number<CompleteStr,i32>,
    do_parse!(
         num: map!(take_while1!(is_digit), to_string)
            >> (num.parse::<i32>().unwrap())
    )
);

named!(
    token_number<CompleteStr,i32>,
    alt!(positive_number|negative_number)
);

named!(external_function<CompleteStr, TopLevelOperation>,
  do_parse!(
    tag!("(")   >>
    ws!(tag!("extern"))   >>
    function_name: ws!(token_identifier) >>
    ws!(tag!("["))   >>
    params: many0!(ws!(token_identifier)) >>
    ws!(tag!("]"))   >>
    tag!(")")   >>
    (TopLevelOperation::ExternalFunction(ExternalFunction{name:function_name,params:params}))
  )
);

named!(empty_list<CompleteStr, Expression>,
  do_parse!(
    tag!("(") >>
    ws!(many0!(ws!(token_comment))) >>
    tag!(")") >>
    (Expression::EmptyList)
  )
);

named!(expression_comment<CompleteStr, Expression>,
  do_parse!(
    tag!(";") >>
    comment: map!(take_while!(is_comment_char),to_string) >>
    (Expression::Comment(comment))
  )
);

named!(expression_literal_string<CompleteStr, Expression>,
    do_parse!(
      text: ws!(alt!(token_symbol|token_text)) >>
      (Expression::TextLiteral(text))
    )
);

named!(expression_identifier<CompleteStr, Expression>,
    do_parse!(
      text: ws!(token_identifier) >>
      (Expression::Identifier(text))
    )
);

named!(expression_number<CompleteStr, Expression>,
    do_parse!(
      num: ws!(token_number) >>
      (Expression::Number(num))
    )
);

named!(boolean_true<CompleteStr, Expression>,
    do_parse!(
      tag!("true") >>
      (Expression::Number(1))
    )
);

named!(boolean_false<CompleteStr, Expression>,
    do_parse!(
      tag!("false") >>
      (Expression::Number(0))
    )
);

named!(expression_let_pair<CompleteStr, (String, Expression)>,
  do_parse!(
    id: ws!(token_identifier)   >>
    exp: ws!(expression)   >>
    ((id,exp))
  )
);

named!(expression_let<CompleteStr, Expression>,
  do_parse!(
    tag!("(")   >>
    ws!(tag!("let"))   >>
    ws!(tag!("["))   >>
    bindings: ws!(many0!(ws!(expression_let_pair))) >>
    ws!(tag!("]"))   >>
    expressions: ws!(many1!(ws!(expression))) >>
    tag!(")")   >>
    (Expression::Let(OperationLet{bindings:bindings,expressions:expressions}))
  )
);

named!(expression_loop<CompleteStr, Expression>,
  do_parse!(
    tag!("(")   >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("loop"))   >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("["))   >>
    many0!(ws!(token_comment)) >>
    bindings: ws!(many0!(ws!(expression_let_pair))) >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("]"))   >>
    expressions: ws!(many1!(ws!(expression))) >>
    tag!(")")   >>
    (Expression::Loop(OperationLoop{bindings:bindings,expressions:expressions}))
  )
);

named!(expression_recur<CompleteStr, Expression>,
  do_parse!(
    tag!("(")   >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("recur"))   >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("["))   >>
    many0!(ws!(token_comment)) >>
    bindings: ws!(many0!(ws!(expression_let_pair))) >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("]"))   >>
    many0!(ws!(token_comment)) >>
    tag!(")")   >>
    (Expression::Recur(OperationRecur{bindings:bindings}))
  )
);

named!(expression_fnsig<CompleteStr, Expression>,
  do_parse!(
    tag!("(")   >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("fnsig"))   >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("["))   >>
    many0!(ws!(token_comment)) >>
    inputs: ws!(many0!(ws!(token_data_type))) >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("]"))   >>
    many0!(ws!(token_comment)) >>
    output: opt!(ws!(token_data_type)) >>
    many0!(ws!(token_comment)) >>
    tag!(")")   >>
    (Expression::FnSig(OperationFnSig{inputs:inputs, output:output}))
  )
);

named!(expression_populate<CompleteStr, Expression>,
  do_parse!(
    tag!("(")   >>
    many0!(ws!(token_comment)) >>
    tag!("#")   >>
    many0!(ws!(token_comment)) >>
    name: ws!(token_identifier) >>
    many0!(ws!(token_comment)) >>
    elements: ws!(many1!(ws!(expression))) >>
    many0!(ws!(token_comment)) >>
    tag!(")")   >>
    (Expression::Populate(OperationPopulate{name:name, elements:elements}))
  )
);

named!(expression<CompleteStr, Expression>,
    alt!(expression_populate|expression_fnsig|expression_loop|expression_recur|expression_let|empty_list|expression_number|boolean_true|boolean_false|expression_comment|expression_literal_string|expression_identifier|expression_function_call)
);

named!(function_params<CompleteStr, Vec<Expression>>,
    do_parse!(
      op: many0!(ws!(expression)) >>
      (op)
    )
);

named!(expression_function_call<CompleteStr, Expression>,
  do_parse!(
    tag!("(")   >>
    function_name: ws!(function_identifiers) >>
    params: function_params >>
    tag!(")")   >>
    (Expression::FunctionCall(OperationFunctionCall{function_name:function_name,params:params}))
  )
);

named!(function_operations<CompleteStr, Vec<Expression>>,
  do_parse!(
    op: many1!(ws!(alt!(expression))) >>
    (op)
  )
);

named!(wasm_op_comment<CompleteStr, WasmOperation>,
  do_parse!(
    tag!(";") >>
    comment: map!(take_while!(is_comment_char),to_string) >>
    (crate::ast::WasmOperation::Comment(comment))
  )
);

named!(wasm_op_identifier<CompleteStr, WasmOperation>,
  do_parse!(
    op: token_identifier >>
    (crate::ast::WasmOperation::Identifier(op))
  )
);

named!(wasm_op_number<CompleteStr, WasmOperation>,
  do_parse!(
    op: token_number >>
    (crate::ast::WasmOperation::Number(op))
  )
);

named!(wasm_op<CompleteStr,WasmOperation>,
  alt!(wasm_op_comment|wasm_op_identifier|wasm_op_number)
);

named!(wasm_ops<CompleteStr, Vec<WasmOperation>>,
  do_parse!(
    op: many0!(ws!(wasm_op)) >>
    (op)
  )
);

named!(define_wasm_function<CompleteStr, TopLevelOperation>,
  do_parse!(
    tag!("(")   >>
    many0!(ws!(token_comment)) >>
    external_name:opt!( ws!(tag!("pub"))) >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("defn-wasm"))   >>
    many0!(ws!(token_comment)) >>
    function_name: ws!(token_identifier) >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("["))   >>
    many0!(ws!(token_comment)) >>
    params: many0!(ws!(token_data_type)) >>
    ws!(tag!("]"))   >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("["))   >>
    many0!(ws!(token_comment)) >>
    outputs: many0!(ws!(token_data_type)) >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("]"))   >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("["))   >>
    many0!(ws!(token_comment)) >>
    locals: many0!(ws!(token_data_type)) >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("]"))   >>
    many0!(ws!(token_comment)) >>
    children: wasm_ops >>
    many0!(ws!(token_comment)) >>
    tag!(")")   >>
    (TopLevelOperation::DefineWasmFunction(WasmFunctionDefinition{name: function_name,
    exported: external_name.is_some(),
    params: params,
    outputs: outputs,
    locals: locals,
    children: children}))
  )
);

named!(define_function<CompleteStr, TopLevelOperation>,
  do_parse!(
    tag!("(")   >>
    many0!(ws!(token_comment)) >>
    external_name:opt!( ws!(tag!("pub"))) >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("defn"))   >>
    many0!(ws!(token_comment)) >>
    function_name: ws!(token_identifier) >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("["))   >>
    many0!(ws!(token_comment)) >>
    params: many0!(ws!(token_identifier)) >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("]"))   >>
    children: function_operations >>
    tag!(")")   >>
    (TopLevelOperation::DefineFunction(FunctionDefinition{name: function_name,
    exported: external_name.is_some(),
    params: params,
    output: None,
    children: children}))
  )
);

named!(define_test_function<CompleteStr, TopLevelOperation>,
  do_parse!(
    tag!("(")   >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("deftest"))   >>
    many0!(ws!(token_comment)) >>
    function_name: ws!(token_identifier) >>
    many0!(ws!(token_comment)) >>
    children: function_operations >>
    tag!(")")   >>
    (TopLevelOperation::DefineTestFunction(TestFunctionDefinition{name: function_name,
    children: children}))
  )
);

named!(value_number<CompleteStr, GlobalValue>,
  do_parse!(
    value: token_number  >>
    (GlobalValue::Number(value))
  )
);

named!(value_text<CompleteStr, GlobalValue>,
  do_parse!(
    value: alt!(token_symbol|token_text)  >>
    (GlobalValue::Text(value))
  )
);

named!(global_bool_true<CompleteStr, GlobalValue>,
  do_parse!(
    tag!("true")  >>
    (GlobalValue::Number(1))
  )
);

named!(global_bool_false<CompleteStr, GlobalValue>,
  do_parse!(
    tag!("false")  >>
    (GlobalValue::Number(0))
  )
);

named!(global_identifier<CompleteStr, GlobalValue>,
  do_parse!(
    value: token_identifier >>
    (GlobalValue::Identifier(value))
  )
);

named!(global_data<CompleteStr, GlobalValue>,
  do_parse!(
    tag!("(")  >>
    values: ws!(many1!(ws!(alt!(global_value|global_identifier)))) >>
    tag!(")")  >>
    (GlobalValue::Data(values))
  )
);

named!(global_empty<CompleteStr, GlobalValue>,
  do_parse!(
    tag!("nil") >>
    (GlobalValue::Number(0))
  )
);

named!(global_value<CompleteStr, GlobalValue>,
  do_parse!(
    value: ws!(alt!(global_bool_true|global_bool_false|value_number|value_text|global_data|global_empty)) >>
    (value)
  )
);

named!(define_global<CompleteStr, TopLevelOperation>,
  do_parse!(
    tag!("(")   >>
    ws!(tag!("def"))   >>
    name: ws!(token_identifier) >>
    value: global_value >>
    tag!(")")   >>
    (TopLevelOperation::DefineGlobal(Global{name: name,value:value}))
  )
);

named!(comment<CompleteStr, TopLevelOperation>,
  do_parse!(
    tag!(";") >>
    comment: map!(take_while!(is_comment_char),to_string) >>
    (TopLevelOperation::Comment(comment))
  )
);

named!(app<CompleteStr, App>,
  do_parse!(
    op: many0!(ws!(alt!(comment|external_function|define_wasm_function|define_function|define_test_function|define_global))) >>
    eof!() >>
    (App{children:op})
  )
);

pub fn parse(content: &str) -> Result<App, Error> {
    let result = app(CompleteStr(content));
    match result {
        Ok((_, value)) => Ok(value),
        Err(nom::Err::Incomplete(needed)) => Err(format_err!("{:?}", needed)),
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(format_err!("{:?}", e)),
    }
}
