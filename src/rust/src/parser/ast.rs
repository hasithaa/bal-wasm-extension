use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AstNode {
    Module(ModuleNode),
    Function(FunctionNode),
    Service(ServiceNode),
    Variable(VariableNode),
    Type(TypeNode),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModuleNode {
    pub imports: Vec<ImportNode>,
    pub declarations: Vec<AstNode>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImportNode {
    pub path: String,
    pub alias: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionNode {
    pub name: String,
    pub params: Vec<ParameterNode>,
    pub return_type: Option<TypeNode>,
    pub body: Vec<AstNode>,
    pub visibility: Visibility,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceNode {
    pub name: String,
    pub endpoints: Vec<String>,
    pub resources: Vec<FunctionNode>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParameterNode {
    pub name: String,
    pub param_type: TypeNode,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TypeNode {
    pub type_name: String,
    pub is_array: bool,
    pub is_optional: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VariableNode {
    pub name: String,
    pub var_type: TypeNode,
    pub initializer: Option<Box<AstNode>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Visibility {
    Public,
    Private,
}
