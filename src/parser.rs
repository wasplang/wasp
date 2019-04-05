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
        pair: pair!(tag!("//"),take_while!(is_comment_char))>>
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
    operator_identifiers<CompleteStr,String>,
    do_parse!(
        id: alt!(map!(tag!(">>"),to_string)|map!(tag!("<<"),to_string)|map!(tag!(">="),to_string)|map!(tag!("<="),to_string)|map!(tag!(">"),to_string)|map!(tag!("<"),to_string)|map!(tag!("or"),to_string)|map!(tag!("and"),to_string)|map!(tag!("!="),to_string)|map!(tag!("=="),to_string)|map!(tag!("+"),to_string)|map!(tag!("-"),to_string)|map!(tag!("*"),to_string)|map!(tag!("/"),to_string)|map!(tag!("%"),to_string)|map!(tag!("|"),to_string)|map!(tag!("&"),to_string))>>
        (id)
    )
);

named!(
    unary_operator_identifiers<CompleteStr,String>,
    do_parse!(
        id: alt!(map!(tag!("^"),to_string)|map!(tag!("~"),to_string)|map!(tag!("!"),to_string))>>
        (id)
    )
);


named!(
    function_identifiers<CompleteStr,String>,
    do_parse!(
        id: alt!(map!(tag!("call"),to_string)|token_identifier|map!(tag!("do"),to_string)|map!(tag!("if"),to_string))>>
        (id)
    )
);


named!(
    token_data_type<CompleteStr,DataType>,
    do_parse!(
        t: map!(alt!(tag!("()")|tag!("i32")|tag!("i64")|tag!("f32")|tag!("f64")), to_string) >>
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
    base_float<CompleteStr,String>,
    do_parse!(
            num: map!(take_while1!(is_digit), to_string) >>
            tag!(".") >>
            den: map!(take_while1!(is_digit), to_string) >>
            (format!("{}.{}",num,den).to_owned())
    )
);

named!(
    base_int<CompleteStr,String>,
    do_parse!(
            num: map!(take_while1!(is_digit), to_string) >>
            (num.to_owned())
    )
);

named!(
    negative_number<CompleteStr,f64>,
    do_parse!(
        tag!("-")
            >> num: alt!(base_float|base_int)
            >> (-num.parse::<f64>().unwrap())
    )
);

named!(
    positive_number<CompleteStr,f64>,
    do_parse!(
         num: alt!(base_float|base_int)
            >> (num.parse::<f64>().unwrap())
    )
);

named!(
    token_number<CompleteStr,f64>,
    alt!(positive_number|negative_number)
);

named!(external_function<CompleteStr, TopLevelOperation>,
  do_parse!(
    ws!(tag!("extern"))   >>
    function_name: ws!(token_identifier) >>
    ws!(tag!("("))   >>
    params: ws!(separated_list!(tag!(","),ws!(token_identifier))) >>
    ws!(tag!(")"))   >>
    (TopLevelOperation::ExternalFunction(ExternalFunction{name:function_name,params:params}))
  )
);

named!(expression_comment<CompleteStr, Expression>,
  do_parse!(
    tag!("//") >>
    comment: map!(take_while!(is_comment_char),to_string) >>
    (Expression::Comment(comment))
  )
);

named!(expression_literal_string<CompleteStr, Expression>,
    do_parse!(
      text: ws!(token_text) >>
      (Expression::TextLiteral(text))
    )
);

named!(expression_literal_token<CompleteStr, Expression>,
    do_parse!(
      text: ws!(token_symbol) >>
      (Expression::SymbolLiteral(text))
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
      (Expression::Number(1.0))
    )
);

named!(boolean_false<CompleteStr, Expression>,
    do_parse!(
      tag!("false") >>
      (Expression::Number(0.0))
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
    ws!(tag!("let"))   >>
    ws!(tag!("("))   >>
    bindings: ws!(many0!(ws!(expression_let_pair))) >>
    ws!(tag!(")"))   >>
    ws!(tag!("{"))   >>
    expressions: ws!(many1!(ws!(expression))) >>
    tag!("}")   >>
    (Expression::Let(OperationLet{bindings:bindings,expressions:expressions}))
  )
);

named!(expression_loop<CompleteStr, Expression>,
  do_parse!(
    ws!(tag!("loop"))   >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("{"))   >>
    expressions: ws!(many1!(ws!(expression))) >>
    tag!("}")   >>
    (Expression::Loop(OperationLoop{bindings:vec![],expressions:expressions}))
  )
);

named!(expression_recur<CompleteStr, Expression>,
  do_parse!(
    tag!("recur")   >>
    (Expression::Recur(OperationRecur{bindings:vec![]}))
  )
);

named!(expression_fnsig<CompleteStr, Expression>,
  do_parse!(
    ws!(tag!("fn"))   >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("("))   >>
    many0!(ws!(token_comment)) >>
    inputs: ws!(separated_list!(tag!(","),ws!(token_data_type))) >>
    many0!(ws!(token_comment)) >>
    ws!(tag!(")"))   >>
    ws!(tag!("->"))   >>
    many0!(ws!(token_comment)) >>
    output: opt!(ws!(token_data_type)) >>
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
    alt!(expression_if_statement|expression_fnsig|expression_let|expression_operator_call|expression_unary_operator_call|expression_assignment|expression_function_call|expression_populate|expression_loop|expression_recur|expression_number|boolean_true|boolean_false|expression_comment|expression_literal_token|expression_literal_string|expression_identifier)
);

named!(function_params<CompleteStr, Vec<Expression>>,
    do_parse!(
      op: ws!(separated_list!(tag!(","),ws!(expression))) >>
      (op)
    )
);

named!(expression_operator_call<CompleteStr, Expression>,
  do_parse!(
    tag!("(") >>
    expr_a: ws!(expression) >>
    function_name: ws!(operator_identifiers) >>
    expr_b: ws!(expression) >>
    tag!(")") >>
    (Expression::FunctionCall(OperationFunctionCall{function_name:function_name,params:vec![expr_a,expr_b]}))
  )
);

named!(expression_assignment<CompleteStr, Expression>,
  do_parse!(
    id: ws!(token_identifier) >>
    ws!(tag!("=")) >>
    expr: ws!(expression) >>
    (Expression::Assignment(OperationAssignment{id:id,value:Box::new(expr)}))
  )
);

named!(expression_if_statement<CompleteStr, Expression>,
  do_parse!(
    ws!(tag!("if")) >>
    ws!(tag!("(")) >>
    expr_a: ws!(expression) >>
    ws!(tag!(")")) >>
    ws!(tag!("{")) >>
    expr_b: ws!(expression) >>
    tag!("}") >>
    ws!(tag!("else")) >>
    ws!(tag!("{")) >>
    expr_c: ws!(expression) >>
    tag!("}") >>
    (Expression::FunctionCall(OperationFunctionCall{function_name:"if".to_string(),params:vec![expr_a,expr_b,expr_c]}))
  )
);

named!(expression_unary_operator_call<CompleteStr, Expression>,
  do_parse!(
    function_name: ws!(unary_operator_identifiers) >>
    expr_a: ws!(expression) >>
    (Expression::FunctionCall(OperationFunctionCall{function_name:function_name,params:vec![expr_a]}))
  )
);

named!(expression_function_call<CompleteStr, Expression>,
  do_parse!(
    function_name: ws!(function_identifiers) >>
    tag!("(")   >>
    params: ws!(function_params) >>
    ws!(tag!(")"))   >>
    (Expression::FunctionCall(OperationFunctionCall{function_name:function_name,params:params}))
  )
);

named!(function_operations<CompleteStr, Vec<Expression>>,
  do_parse!(
    op: many1!(ws!(alt!(expression))) >>
    (op)
  )
);

named!(define_function<CompleteStr, TopLevelOperation>,
  do_parse!(
    external_name:opt!( ws!(tag!("pub"))) >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("fn"))   >>
    many0!(ws!(token_comment)) >>
    function_name: ws!(token_identifier) >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("("))   >>
    many0!(ws!(token_comment)) >>
    params: many0!(ws!(token_identifier)) >>
    many0!(ws!(token_comment)) >>
    ws!(tag!(")"))   >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("{"))   >>
    children: function_operations >>
    tag!("}")   >>
    (TopLevelOperation::DefineFunction(FunctionDefinition{name: function_name,
    exported: external_name.is_some(),
    params: params,
    output: None,
    children: children}))
  )
);

named!(struct_pair<CompleteStr, StructMember>,
  do_parse!(
    name: token_symbol >>
    many0!(ws!(token_comment)) >>
    (StructMember{name: name})
  )
);

named!(define_struct<CompleteStr, TopLevelOperation>,
  do_parse!(
    tag!("(")   >>
    many0!(ws!(token_comment)) >>
    ws!(tag!("defstruct"))   >>
    many0!(ws!(token_comment)) >>
    name: ws!(token_identifier) >>
    many0!(ws!(token_comment)) >>
    members: many0!(ws!(struct_pair)) >>
    many0!(ws!(token_comment)) >>
    tag!(")")   >>
    (TopLevelOperation::DefineGlobal(Global{name:name,value:GlobalValue::Struct(StructDefinition{
    members: members})}))
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
    value: token_text  >>
    (GlobalValue::Text(value))
  )
);

named!(value_symbol<CompleteStr, GlobalValue>,
  do_parse!(
    value: token_symbol  >>
    (GlobalValue::Symbol(value))
  )
);

named!(global_bool_true<CompleteStr, GlobalValue>,
  do_parse!(
    tag!("true")  >>
    (GlobalValue::Number(1.0))
  )
);

named!(global_bool_false<CompleteStr, GlobalValue>,
  do_parse!(
    tag!("false")  >>
    (GlobalValue::Number(0.0))
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
    values: ws!(separated_list!(tag!(","),ws!(alt!(global_value|global_identifier)))) >>
    tag!(")")  >>
    (GlobalValue::Data(values))
  )
);

named!(global_value<CompleteStr, GlobalValue>,
  do_parse!(
    value: ws!(alt!(global_bool_true|global_bool_false|value_number|value_symbol|value_text|global_data)) >>
    (value)
  )
);

named!(define_global<CompleteStr, TopLevelOperation>,
  do_parse!(
    ws!(tag!("static"))   >>
    name: ws!(token_identifier) >>
    ws!(tag!("="))   >>
    value: global_value >>
    (TopLevelOperation::DefineGlobal(Global{name: name,value:value}))
  )
);

named!(comment<CompleteStr, TopLevelOperation>,
  do_parse!(
    tag!("//") >>
    comment: map!(take_while!(is_comment_char),to_string) >>
    (TopLevelOperation::Comment(comment))
  )
);

named!(app<CompleteStr, App>,
  do_parse!(
    op: many0!(ws!(alt!(comment|external_function|define_function|define_struct|define_global))) >>
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
