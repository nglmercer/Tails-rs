use crate::compiler::Compiler;
use crate::errors::Result;
use crate::objects::Value;
use crate::vm::Interpreter;
use std::path::Path;

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

    pub fn eval_module(&mut self, source: &str, base_path: &Path) -> Result<Value> {
        let prev = self.interpreter.current_module_path.clone();
        self.interpreter.current_module_path = Some(base_path.to_string_lossy().to_string());
        let compiler = Compiler::new(self.config.enable_type_checking);
        let compiled = compiler.compile(source)?;
        let result = self.interpreter.execute_module(&compiled);
        self.interpreter.current_module_path = prev;
        result
    }

    pub fn import(&mut self, module_path: &Path) -> Result<Value> {
        let source = std::fs::read_to_string(module_path)
            .map_err(|e| crate::errors::Error::RuntimeError(format!("Failed to read module: {}", e)))?;
        let base = module_path.parent().unwrap_or(Path::new("."));
        self.eval_module(&source, base)
    }
    
    pub fn get_global(&self, name: &str) -> Option<Value> {
        self.interpreter.get_global(name)
    }
    
    pub fn set_global(&mut self, name: &str, value: Value) {
        self.interpreter.set_global(name, value);
    }

    pub fn get_module_export(&self, module_path: &str, name: &str) -> Option<Value> {
        self.interpreter.module_registry.get(module_path)
            .and_then(|exports| exports.get(name).cloned())
    }
}

impl Default for TailsRuntime {
    fn default() -> Self {
        Self::new(RuntimeConfig::default()).expect("Failed to create default runtime")
    }
}
