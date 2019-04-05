use wasmly::DataType;

#[derive(Debug)]
pub struct App {
    pub children: Vec<TopLevelOperation>,
}

#[derive(Debug, Clone)]
pub enum TopLevelOperation {
    Comment(String),
    DefineGlobal(Global),
    DefineFunction(FunctionDefinition),
    ExternalFunction(ExternalFunction),
}

#[derive(Debug, Clone)]
pub struct Global {
    pub name: String,
    pub value: GlobalValue,
}

#[derive(Debug, Clone)]
pub enum GlobalValue {
    Symbol(String),
    Number(f64),
    Text(String),
    Data(Vec<GlobalValue>),
    Identifier(String),
    Struct(StructDefinition),
}

#[derive(Debug, Clone)]
pub struct ExternalFunction {
    pub name: String,
    pub params: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    pub exported: bool,
    pub params: Vec<String>,
    pub output: Option<String>,
    pub children: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct StructMember {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct StructDefinition {
    pub members: Vec<StructMember>,
}

#[derive(Debug, Clone)]
pub struct OperationFunctionCall {
    pub function_name: String,
    pub params: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct OperationLet {
    pub bindings: Vec<(String, Expression)>,
    pub expressions: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct OperationRecur {
}

#[derive(Debug, Clone)]
pub struct OperationAssignment {
    pub id:String,
    pub value: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct OperationIfStatement {
    pub condition: Box<Expression>,
    pub if_true: Vec<Expression>,
    pub if_false: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct OperationLoop {
    pub expressions: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct OperationFnSig {
    pub inputs: Vec<DataType>,
    pub output: Option<DataType>,
}

#[derive(Debug, Clone)]
pub struct OperationPopulate {
    pub name: String,
    pub elements: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    IfStatement(OperationIfStatement),
    Assignment(OperationAssignment),
    TextLiteral(String),
    SymbolLiteral(String),
    Identifier(String),
    Comment(String),
    FunctionCall(OperationFunctionCall),
    Number(f64),
    Let(OperationLet),
    Populate(OperationPopulate),
    Recur(OperationRecur),
    Loop(OperationLoop),
    FnSig(OperationFnSig),
}
