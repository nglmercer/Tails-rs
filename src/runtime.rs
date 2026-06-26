use crate::compiler::Compiler;
use crate::errors::{Error, Result};
use crate::objects::Value;
use crate::vm::Interpreter;

#[derive(Default)]
pub struct RuntimeConfig {
    pub enable_type_checking: bool,
    pub max_heap_size: usize,
}

pub struct TailsRuntime {
    interpreter: Interpreter,
    config: RuntimeConfig,
}

impl TailsRuntime {
    pub fn new(config: RuntimeConfig) -> Result<Self> {
        let interpreter = Interpreter::new()?;
        Ok(Self { interpreter, config })
    }
    
    pub fn eval(&mut self, source: &str) -> Result<Value> {
        let compiler = Compiler::new(self.config.enable_type_checking);
        let compiled = compiler.compile(source)?;
        self.interpreter.execute(&compiled)
    }
    
    pub fn get_global(&self, name: &str) -> Option<Value> {
        self.interpreter.get_global(name)
    }
    
    pub fn set_global(&mut self, name: &str, value: Value) {
        self.interpreter.set_global(name, value);
    }
}

impl Default for TailsRuntime {
    fn default() -> Self {
        Self::new(RuntimeConfig::default()).expect("Failed to create default runtime")
    }
}
