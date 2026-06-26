use std::collections::HashMap;
use crate::compiler::CompiledModule;
use crate::errors::{Error, Result};
use crate::objects::Value;
use crate::compiler::Instruction;

pub struct Interpreter {
    globals: HashMap<String, Value>,
    stack: Vec<Value>,
}

impl Interpreter {
    pub fn new() -> Result<Self> {
        Ok(Self {
            globals: HashMap::new(),
            stack: Vec::new(),
        })
    }
    
    pub fn execute(&mut self, module: &CompiledModule) -> Result<Value> {
        let mut pc = 0;
        
        while pc < module.instructions.len() {
            let instruction = &module.instructions[pc];
            
            match instruction {
                Instruction::LoadConst(idx) => {
                    let value = module.constants[*idx as usize].clone();
                    self.stack.push(value);
                }
                Instruction::LoadNull => {
                    self.stack.push(Value::Null);
                }
                Instruction::LoadUndefined => {
                    self.stack.push(Value::Undefined);
                }
                Instruction::LoadTrue => {
                    self.stack.push(Value::Boolean(true));
                }
                Instruction::LoadFalse => {
                    self.stack.push(Value::Boolean(false));
                }
                Instruction::LoadGlobal(name) => {
                    let value = self.globals.get(name)
                        .cloned()
                        .unwrap_or(Value::Undefined);
                    self.stack.push(value);
                }
                Instruction::StoreGlobal(name) => {
                    let value = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    self.globals.insert(name.clone(), value);
                }
                Instruction::Add => {
                    let right = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let left = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let result = self.add(left, right)?;
                    self.stack.push(result);
                }
                Instruction::Sub => {
                    let right = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let left = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let result = self.sub(left, right)?;
                    self.stack.push(result);
                }
                Instruction::Mul => {
                    let right = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let left = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let result = self.mul(left, right)?;
                    self.stack.push(result);
                }
                Instruction::Div => {
                    let right = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let left = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let result = self.div(left, right)?;
                    self.stack.push(result);
                }
                Instruction::Mod => {
                    let right = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let left = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let result = self.modulo(left, right)?;
                    self.stack.push(result);
                }
                Instruction::Negate => {
                    let value = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let result = self.negate(value)?;
                    self.stack.push(result);
                }
                Instruction::Not => {
                    let value = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let result = Value::Boolean(!self.is_truthy(&value));
                    self.stack.push(result);
                }
                Instruction::Pop => {
                    self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                }
                Instruction::Return => {
                    return self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()));
                }
                _ => {
                    return Err(Error::RuntimeError(format!("Unhandled instruction: {:?}", instruction)));
                }
            }
            
            pc += 1;
        }
        
        self.stack.pop()
            .ok_or_else(|| Error::RuntimeError("Empty stack at end of execution".into()))
    }
    
    pub fn get_global(&self, name: &str) -> Option<Value> {
        self.globals.get(name).cloned()
    }
    
    pub fn set_global(&mut self, name: &str, value: Value) {
        self.globals.insert(name.to_string(), value);
    }
    
    fn add(&self, left: Value, right: Value) -> Result<Value> {
        match (&left, &right) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a + *b as f64)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            _ => Err(Error::TypeError(format!(
                "Cannot add {:?} and {:?}",
                left, right
            ))),
        }
    }
    
    fn sub(&self, left: Value, right: Value) -> Result<Value> {
        match (&left, &right) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a - *b as f64)),
            _ => Err(Error::TypeError(format!(
                "Cannot subtract {:?} from {:?}",
                right, left
            ))),
        }
    }
    
    fn mul(&self, left: Value, right: Value) -> Result<Value> {
        match (&left, &right) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a * *b as f64)),
            _ => Err(Error::TypeError(format!(
                "Cannot multiply {:?} and {:?}",
                left, right
            ))),
        }
    }
    
    fn div(&self, left: Value, right: Value) -> Result<Value> {
        match (&left, &right) {
            (Value::Integer(a), Value::Integer(b)) => {
                if *b == 0 {
                    return Err(Error::RuntimeError("Division by zero".into()));
                }
                Ok(Value::Integer(a / b))
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    return Err(Error::RuntimeError("Division by zero".into()));
                }
                Ok(Value::Float(a / b))
            }
            (Value::Integer(a), Value::Float(b)) => {
                if *b == 0.0 {
                    return Err(Error::RuntimeError("Division by zero".into()));
                }
                Ok(Value::Float(*a as f64 / b))
            }
            (Value::Float(a), Value::Integer(b)) => {
                if *b == 0 {
                    return Err(Error::RuntimeError("Division by zero".into()));
                }
                Ok(Value::Float(a / *b as f64))
            }
            _ => Err(Error::TypeError(format!(
                "Cannot divide {:?} by {:?}",
                left, right
            ))),
        }
    }
    
    fn modulo(&self, left: Value, right: Value) -> Result<Value> {
        match (&left, &right) {
            (Value::Integer(a), Value::Integer(b)) => {
                if *b == 0 {
                    return Err(Error::RuntimeError("Division by zero".into()));
                }
                Ok(Value::Integer(a % b))
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    return Err(Error::RuntimeError("Division by zero".into()));
                }
                Ok(Value::Float(a % b))
            }
            _ => Err(Error::TypeError(format!(
                "Cannot apply modulo to {:?} and {:?}",
                left, right
            ))),
        }
    }
    
    fn negate(&self, value: Value) -> Result<Value> {
        match value {
            Value::Integer(n) => Ok(Value::Integer(-n)),
            Value::Float(n) => Ok(Value::Float(-n)),
            _ => Err(Error::TypeError(format!("Cannot negate {:?}", value))),
        }
    }
    
    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Undefined => false,
            Value::Null => false,
            Value::Boolean(b) => *b,
            Value::Integer(n) => *n != 0,
            Value::Float(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::BigInt(n) => *n != 0,
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new().expect("Failed to create default interpreter")
    }
}
