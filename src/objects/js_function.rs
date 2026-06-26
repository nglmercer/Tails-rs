use crate::compiler::CompiledModule;
use crate::objects::Value;

#[derive(Debug, Clone)]
pub enum FunctionKind {
    UserDefined(UserDefinedFunction),
    Native(NativeFunction),
}

#[derive(Debug, Clone)]
pub struct UserDefinedFunction {
    pub name: Option<String>,
    pub params: Vec<String>,
    pub body: CompiledModule,
    pub closure: Option<Closure>,
}

#[derive(Debug, Clone)]
pub struct NativeFunction {
    pub name: String,
    pub func: usize,
}

#[derive(Debug, Clone)]
pub struct Closure {
    pub variables: Vec<(String, Value)>,
}

impl UserDefinedFunction {
    pub fn new(name: Option<String>, params: Vec<String>, body: CompiledModule) -> Self {
        Self {
            name,
            params,
            body,
            closure: None,
        }
    }
}

impl NativeFunction {
    pub fn new(name: &str, func: usize) -> Self {
        Self {
            name: name.to_string(),
            func,
        }
    }
}
