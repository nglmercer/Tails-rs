use crate::errors::Result;
use crate::objects::Value;

pub mod lexer;
pub mod parser;
pub mod type_checker;
pub mod bytecode;

pub struct Compiler {
    type_checking: bool,
}

impl Compiler {
    pub fn new(type_checking: bool) -> Self {
        Self { type_checking }
    }
    
    pub fn compile(&self, source: &str) -> Result<CompiledModule> {
        let tokens = lexer::tokenize(source)?;
        let ast = parser::parse(&tokens)?;
        
        if self.type_checking {
            type_checker::TypeChecker::check(&ast)?;
        }
        
        bytecode::generate(&ast)
    }
}

#[derive(Debug, Clone)]
pub struct CompiledModule {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub functions: Vec<CompiledFunction>,
}

#[derive(Debug, Clone)]
pub struct CompiledFunction {
    pub name: Option<String>,
    pub params: Vec<String>,
    pub bytecode_index: usize,
    pub param_count: usize,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    LoadConst(u32),
    LoadNull,
    LoadUndefined,
    LoadTrue,
    LoadFalse,
    StoreGlobal(String),
    LoadGlobal(String),
    StoreLocal(u16),
    LoadLocal(u16),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Power,
    Negate,
    Not,
    Eq,
    StrictEq,
    NotEqual,
    StrictNotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Jump(u32),
    JumpIf(u32),
    JumpIfNot(u32),
    Call(u16),
    Return,
    Pop,
    MakeFunction(u32),
    MakeClosure(u32, Vec<u16>),
    NewObject,
    SetProperty,
    GetProperty,
    NewArray(u32),
    ArrayPush,
    TypeOf,
    InstanceOf,
    In,
    Delete,
    Void,
}
