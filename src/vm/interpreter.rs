use std::collections::HashMap;
use crate::compiler::{CompiledModule, Instruction};
use crate::errors::{Error, Result};
use crate::objects::Value;

#[derive(Debug, Clone)]
pub struct JsObject {
    pub properties: HashMap<String, Value>,
}

impl JsObject {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct JsArray {
    pub elements: Vec<Value>,
}

#[derive(Debug, Clone)]
pub struct JsFunction {
    pub name: Option<String>,
    pub params: Vec<String>,
    pub bytecode_index: usize,
    pub closure: Vec<Value>,
}

#[derive(Debug, Clone)]
pub enum HeapValue {
    String(String),
    Object(JsObject),
    Array(JsArray),
    Function(JsFunction),
}

#[derive(Debug, Clone)]
struct CallFrame {
    return_address: usize,
    base_pointer: usize,
}

pub struct Interpreter {
    globals: HashMap<String, Value>,
    stack: Vec<Value>,
    heap: Vec<HeapValue>,
    call_stack: Vec<CallFrame>,
}

impl Interpreter {
    pub fn new() -> Result<Self> {
        Ok(Self {
            globals: HashMap::new(),
            stack: Vec::new(),
            heap: Vec::new(),
            call_stack: Vec::new(),
        })
    }
    
    pub fn execute(&mut self, module: &CompiledModule) -> Result<Value> {
        self.execute_from(module, 0)
    }
    
    fn execute_from(&mut self, module: &CompiledModule, start_pc: usize) -> Result<Value> {
        let mut pc = start_pc;
        
        loop {
            if pc >= module.instructions.len() {
                break;
            }
            
            let instruction = module.instructions[pc].clone();
            
            match &instruction {
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
                Instruction::LoadLocal(slot) => {
                    let base = self.call_stack.last()
                        .map(|f| f.base_pointer)
                        .unwrap_or(0);
                    let idx = base + *slot as usize;
                    let value = self.stack.get(idx)
                        .cloned()
                        .unwrap_or(Value::Undefined);
                    self.stack.push(value);
                }
                Instruction::StoreLocal(slot) => {
                    let value = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let base = self.call_stack.last()
                        .map(|f| f.base_pointer)
                        .unwrap_or(0);
                    let idx = base + *slot as usize;
                    
                    if idx >= self.stack.len() {
                        self.stack.resize(idx + 1, Value::Undefined);
                    }
                    self.stack[idx] = value;
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
                Instruction::Power => {
                    let right = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let left = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let result = self.power(left, right)?;
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
                Instruction::Eq => {
                    let right = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let left = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    self.stack.push(Value::Boolean(self.is_equal(&left, &right)));
                }
                Instruction::StrictEq => {
                    let right = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let left = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    self.stack.push(Value::Boolean(left == right));
                }
                Instruction::NotEqual => {
                    let right = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let left = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    self.stack.push(Value::Boolean(!self.is_equal(&left, &right)));
                }
                Instruction::StrictNotEqual => {
                    let right = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let left = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    self.stack.push(Value::Boolean(left != right));
                }
                Instruction::Less => {
                    let right = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let left = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let result = self.less_than(&left, &right)?;
                    self.stack.push(Value::Boolean(result));
                }
                Instruction::Greater => {
                    let right = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let left = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let result = self.greater_than(&left, &right)?;
                    self.stack.push(Value::Boolean(result));
                }
                Instruction::LessEqual => {
                    let right = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let left = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let result = self.less_than_or_equal(&left, &right)?;
                    self.stack.push(Value::Boolean(result));
                }
                Instruction::GreaterEqual => {
                    let right = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let left = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let result = self.greater_than_or_equal(&left, &right)?;
                    self.stack.push(Value::Boolean(result));
                }
                Instruction::And => {
                    let right = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let left = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let result = self.is_truthy(&left) && self.is_truthy(&right);
                    self.stack.push(Value::Boolean(result));
                }
                Instruction::Or => {
                    let right = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let left = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let result = self.is_truthy(&left) || self.is_truthy(&right);
                    self.stack.push(Value::Boolean(result));
                }
                Instruction::Jump(target) => {
                    pc = *target as usize;
                    continue;
                }
                Instruction::JumpIf(target) => {
                    let value = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    if self.is_truthy(&value) {
                        pc = *target as usize;
                        continue;
                    }
                }
                Instruction::JumpIfNot(target) => {
                    let value = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    if !self.is_truthy(&value) {
                        pc = *target as usize;
                        continue;
                    }
                }
                Instruction::Call(argc) => {
                    let callee = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    
                    let mut args = Vec::new();
                    for _ in 0..*argc {
                        args.push(self.stack.pop()
                            .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?);
                    }
                    args.reverse();
                    
                    match callee {
                        Value::Function(func_idx) => {
                            let func = self.heap[func_idx].clone();
                            if let HeapValue::Function(f) = func {
                                let return_address = pc + 1;
                                let base_pointer = self.stack.len();
                                
                                self.call_stack.push(CallFrame {
                                    return_address,
                                    base_pointer,
                                });
                                
                                for arg in args {
                                    self.stack.push(arg);
                                }
                                
                                pc = f.bytecode_index;
                                continue;
                            }
                        }
                        Value::NativeFunction(native_idx) => {
                            let result = self.call_native(native_idx, &args)?;
                            self.stack.push(result);
                        }
                        _ => {
                            return Err(Error::TypeError(format!("{} is not a function", self.value_to_string(&callee))));
                        }
                    }
                }
                Instruction::Return => {
                    let return_value = self.stack.pop()
                        .unwrap_or(Value::Undefined);
                    
                    if let Some(frame) = self.call_stack.pop() {
                        self.stack.truncate(frame.base_pointer);
                        self.stack.push(return_value);
                        pc = frame.return_address;
                        continue;
                    } else {
                        return Ok(return_value);
                    }
                }
                Instruction::Pop => {
                    self.stack.pop();
                }
                Instruction::MakeFunction(func_idx) => {
                    let func_info = module.functions[*func_idx as usize].clone();
                    
                    let heap_idx = self.heap.len();
                    self.heap.push(HeapValue::Function(JsFunction {
                        name: func_info.name,
                        params: func_info.params,
                        bytecode_index: func_info.bytecode_index,
                        closure: Vec::new(),
                    }));
                    
                    self.stack.push(Value::Function(heap_idx));
                }
                Instruction::MakeClosure(func_idx, _capture_slots) => {
                    let func_info = module.functions[*func_idx as usize].clone();
                    
                    let mut closure_vars = Vec::new();
                    for slot in _capture_slots {
                        let value = self.stack.get(*slot as usize)
                            .cloned()
                            .unwrap_or(Value::Undefined);
                        closure_vars.push(value);
                    }
                    
                    let heap_idx = self.heap.len();
                    self.heap.push(HeapValue::Function(JsFunction {
                        name: func_info.name,
                        params: func_info.params,
                        bytecode_index: func_info.bytecode_index,
                        closure: closure_vars,
                    }));
                    
                    self.stack.push(Value::Function(heap_idx));
                }
                Instruction::NewObject => {
                    let heap_idx = self.heap.len();
                    self.heap.push(HeapValue::Object(JsObject::new()));
                    self.stack.push(Value::Object(heap_idx));
                }
                Instruction::SetProperty => {
                    let value = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let key = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let object = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    
                    if let Value::Object(obj_idx) = object {
                        if let HeapValue::Object(obj) = &mut self.heap[obj_idx] {
                            if let Value::String(key_str) = key {
                                obj.properties.insert(key_str, value.clone());
                            }
                        }
                    }
                    self.stack.push(value);
                }
                Instruction::GetProperty => {
                    let key = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let object = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    
                    let result = self.get_property(&object, &key)?;
                    self.stack.push(result);
                }
                Instruction::NewArray(size) => {
                    let mut elements = Vec::new();
                    for _ in 0..*size {
                        elements.push(self.stack.pop().unwrap_or(Value::Undefined));
                    }
                    elements.reverse();
                    
                    let heap_idx = self.heap.len();
                    self.heap.push(HeapValue::Array(JsArray { elements }));
                    self.stack.push(Value::Array(heap_idx));
                }
                Instruction::TypeOf => {
                    let value = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let type_str = match &value {
                        Value::Undefined => "undefined",
                        Value::Null => "object",
                        Value::Boolean(_) => "boolean",
                        Value::Integer(_) | Value::Float(_) => "number",
                        Value::String(_) => "string",
                        Value::BigInt(_) => "bigint",
                        Value::Function(_) | Value::NativeFunction(_) => "function",
                        Value::Object(_) | Value::Array(_) => "object",
                    };
                    self.stack.push(Value::String(type_str.to_string()));
                }
                _ => {
                    return Err(Error::RuntimeError(format!("Unhandled instruction: {:?}", instruction)));
                }
            }
            
            pc += 1;
        }
        
        Ok(self.stack.pop().unwrap_or(Value::Undefined))
    }
    
    pub fn get_global(&self, name: &str) -> Option<Value> {
        self.globals.get(name).cloned()
    }
    
    pub fn set_global(&mut self, name: &str, value: Value) {
        self.globals.insert(name.to_string(), value);
    }
    
    fn call_native(&mut self, _idx: usize, _args: &[Value]) -> Result<Value> {
        Ok(Value::Undefined)
    }
    
    fn get_property(&self, object: &Value, key: &Value) -> Result<Value> {
        match object {
            Value::Object(obj_idx) => {
                if let HeapValue::Object(obj) = &self.heap[*obj_idx] {
                    if let Value::String(key_str) = key {
                        return Ok(obj.properties.get(key_str).cloned().unwrap_or(Value::Undefined));
                    }
                }
            }
            Value::Array(arr_idx) => {
                if let HeapValue::Array(arr) = &self.heap[*arr_idx] {
                    if let Value::String(key_str) = key {
                        if key_str == "length" {
                            return Ok(Value::Float(arr.elements.len() as f64));
                        }
                        if let Ok(index) = key_str.parse::<usize>() {
                            return Ok(arr.elements.get(index).cloned().unwrap_or(Value::Undefined));
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(Value::Undefined)
    }
    
    fn add(&self, left: Value, right: Value) -> Result<Value> {
        match (&left, &right) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a + *b as f64)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            _ => Err(Error::TypeError(format!(
                "Cannot add {} and {}",
                self.value_to_string(&left),
                self.value_to_string(&right)
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
                "Cannot subtract {} from {}",
                self.value_to_string(&right),
                self.value_to_string(&left)
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
                "Cannot multiply {} and {}",
                self.value_to_string(&left),
                self.value_to_string(&right)
            ))),
        }
    }
    
    fn div(&self, left: Value, right: Value) -> Result<Value> {
        match (&left, &right) {
            (Value::Integer(a), Value::Integer(b)) => {
                if *b == 0 {
                    return Err(Error::RuntimeError("Division by zero".into()));
                }
                Ok(Value::Float(*a as f64 / *b as f64))
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
                "Cannot divide {} by {}",
                self.value_to_string(&left),
                self.value_to_string(&right)
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
                "Cannot apply modulo to {} and {}",
                self.value_to_string(&left),
                self.value_to_string(&right)
            ))),
        }
    }
    
    fn power(&self, left: Value, right: Value) -> Result<Value> {
        match (&left, &right) {
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(*b))),
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Float((*a as f64).powf(*b as f64))),
            _ => Err(Error::TypeError(format!(
                "Cannot raise {} to the power of {}",
                self.value_to_string(&left),
                self.value_to_string(&right)
            ))),
        }
    }
    
    fn negate(&self, value: Value) -> Result<Value> {
        match value {
            Value::Integer(n) => Ok(Value::Integer(-n)),
            Value::Float(n) => Ok(Value::Float(-n)),
            _ => Err(Error::TypeError(format!("Cannot negate {}", self.value_to_string(&value)))),
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
            _ => true,
        }
    }
    
    fn is_equal(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Undefined, Value::Undefined) => true,
            (Value::Null, Value::Null) => true,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Integer(a), Value::Float(b)) => *a as f64 == *b,
            (Value::Float(a), Value::Integer(b)) => *a == *b as f64,
            (Value::String(a), Value::String(b)) => a == b,
            _ => false,
        }
    }
    
    fn less_than(&self, left: &Value, right: &Value) -> Result<bool> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => Ok(a < b),
            (Value::Float(a), Value::Float(b)) => Ok(a < b),
            (Value::Integer(a), Value::Float(b)) => Ok((*a as f64) < *b),
            (Value::Float(a), Value::Integer(b)) => Ok(*a < (*b as f64)),
            _ => Err(Error::TypeError(format!(
                "Cannot compare {} and {}",
                self.value_to_string(left),
                self.value_to_string(right)
            ))),
        }
    }
    
    fn greater_than(&self, left: &Value, right: &Value) -> Result<bool> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => Ok(a > b),
            (Value::Float(a), Value::Float(b)) => Ok(a > b),
            (Value::Integer(a), Value::Float(b)) => Ok((*a as f64) > *b),
            (Value::Float(a), Value::Integer(b)) => Ok(*a > (*b as f64)),
            _ => Err(Error::TypeError(format!(
                "Cannot compare {} and {}",
                self.value_to_string(left),
                self.value_to_string(right)
            ))),
        }
    }
    
    fn less_than_or_equal(&self, left: &Value, right: &Value) -> Result<bool> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => Ok(a <= b),
            (Value::Float(a), Value::Float(b)) => Ok(a <= b),
            (Value::Integer(a), Value::Float(b)) => Ok((*a as f64) <= *b),
            (Value::Float(a), Value::Integer(b)) => Ok(*a <= (*b as f64)),
            _ => Err(Error::TypeError(format!(
                "Cannot compare {} and {}",
                self.value_to_string(left),
                self.value_to_string(right)
            ))),
        }
    }
    
    fn greater_than_or_equal(&self, left: &Value, right: &Value) -> Result<bool> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => Ok(a >= b),
            (Value::Float(a), Value::Float(b)) => Ok(a >= b),
            (Value::Integer(a), Value::Float(b)) => Ok((*a as f64) >= *b),
            (Value::Float(a), Value::Integer(b)) => Ok(*a >= (*b as f64)),
            _ => Err(Error::TypeError(format!(
                "Cannot compare {} and {}",
                self.value_to_string(left),
                self.value_to_string(right)
            ))),
        }
    }
    
    fn value_to_string(&self, value: &Value) -> String {
        match value {
            Value::Undefined => "undefined".to_string(),
            Value::Null => "null".to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Integer(n) => n.to_string(),
            Value::Float(n) => n.to_string(),
            Value::String(s) => format!("\"{}\"", s),
            Value::BigInt(n) => format!("{}n", n),
            Value::Function(_) => "[Function]".to_string(),
            Value::NativeFunction(_) => "[Native Function]".to_string(),
            Value::Object(_) => "[Object]".to_string(),
            Value::Array(_) => "[Array]".to_string(),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new().expect("Failed to create default interpreter")
    }
}
