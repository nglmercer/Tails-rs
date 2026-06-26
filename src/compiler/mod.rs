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
            type_checker::check(&ast)?;
        }
        
        bytecode::generate(&ast)
    }
}

#[derive(Debug, Clone)]
pub struct CompiledModule {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
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
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Return,
    Pop,
    Negate,
    Not,
}
