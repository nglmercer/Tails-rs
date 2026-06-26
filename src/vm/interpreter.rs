use std::collections::HashMap;
use crate::compiler::{CompiledModule, Instruction};
use crate::errors::{Error, Result};
use crate::objects::Value;
use crate::runtime_env::native_fns::NATIVE_TABLE;

#[derive(Debug, Clone)]
pub struct JsObject {
    pub properties: HashMap<String, Value>,
    pub prototype: Option<usize>,
}

impl JsObject {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
            prototype: None,
        }
    }

    pub fn with_prototype(prototype: Option<usize>) -> Self {
        Self {
            properties: HashMap::new(),
            prototype,
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
    pub prototype: Option<usize>,
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
    closure_var_count: usize,
    func_heap_idx: Option<usize>,
    this_value: Option<Value>,
    is_construct: bool,
}

pub struct Interpreter {
    pub(crate) globals: HashMap<String, Value>,
    pub(crate) stack: Vec<Value>,
    pub(crate) heap: Vec<HeapValue>,
    call_stack: Vec<CallFrame>,
    current_module: Option<CompiledModule>,
}

impl Interpreter {
    pub fn new() -> Result<Self> {
        let mut interp = Self {
            globals: HashMap::new(),
            stack: Vec::new(),
            heap: Vec::new(),
            call_stack: Vec::new(),
            current_module: None,
        };
        interp.init_builtins();
        Ok(interp)
    }

    fn init_builtins(&mut self) {
        // Global functions
        self.globals.insert("parseInt".into(), Value::NativeFunction(10));
        self.globals.insert("parseFloat".into(), Value::NativeFunction(11));
        self.globals.insert("isNaN".into(), Value::NativeFunction(12));
        self.globals.insert("isFinite".into(), Value::NativeFunction(13));

        // Timer stubs
        self.globals.insert("setTimeout".into(), Value::NativeFunction(14));
        self.globals.insert("setInterval".into(), Value::NativeFunction(15));
        self.globals.insert("clearTimeout".into(), Value::NativeFunction(16));
        self.globals.insert("clearInterval".into(), Value::NativeFunction(17));

        // console object
        let console_obj_idx = self.heap.len();
        let mut console_props = HashMap::new();
        console_props.insert("log".into(), Value::NativeFunction(0));
        console_props.insert("warn".into(), Value::NativeFunction(1));
        console_props.insert("error".into(), Value::NativeFunction(2));
        console_props.insert("info".into(), Value::NativeFunction(3));
        self.heap.push(HeapValue::Object(JsObject { properties: console_props, prototype: None }));
        self.globals.insert("console".into(), Value::Object(console_obj_idx));

        // Object
        let object_obj_idx = self.heap.len();
        let mut object_props = HashMap::new();
        object_props.insert("keys".into(), Value::NativeFunction(4));
        object_props.insert("values".into(), Value::NativeFunction(5));
        object_props.insert("entries".into(), Value::NativeFunction(6));
        object_props.insert("assign".into(), Value::NativeFunction(7));
        self.heap.push(HeapValue::Object(JsObject { properties: object_props, prototype: None }));
        self.globals.insert("Object".into(), Value::Object(object_obj_idx));

        // JSON
        let json_obj_idx = self.heap.len();
        let mut json_props = HashMap::new();
        json_props.insert("parse".into(), Value::NativeFunction(8));
        json_props.insert("stringify".into(), Value::NativeFunction(9));
        self.heap.push(HeapValue::Object(JsObject { properties: json_props, prototype: None }));
        self.globals.insert("JSON".into(), Value::Object(json_obj_idx));

        // Math
        let math_obj_idx = self.heap.len();
        let mut math_props = HashMap::new();
        math_props.insert("PI".into(), Value::Float(std::f64::consts::PI));
        math_props.insert("E".into(), Value::Float(std::f64::consts::E));
        math_props.insert("abs".into(), Value::NativeFunction(18));
        math_props.insert("floor".into(), Value::NativeFunction(19));
        math_props.insert("ceil".into(), Value::NativeFunction(20));
        math_props.insert("round".into(), Value::NativeFunction(21));
        math_props.insert("min".into(), Value::NativeFunction(22));
        math_props.insert("max".into(), Value::NativeFunction(23));
        math_props.insert("random".into(), Value::NativeFunction(24));
        math_props.insert("pow".into(), Value::NativeFunction(25));
        math_props.insert("sqrt".into(), Value::NativeFunction(26));
        math_props.insert("log".into(), Value::NativeFunction(27));
        math_props.insert("sin".into(), Value::NativeFunction(28));
        math_props.insert("cos".into(), Value::NativeFunction(29));
        math_props.insert("tan".into(), Value::NativeFunction(30));
        self.heap.push(HeapValue::Object(JsObject { properties: math_props, prototype: None }));
        self.globals.insert("Math".into(), Value::Object(math_obj_idx));

        // Number constructor
        let number_obj_idx = self.heap.len();
        let mut number_props = HashMap::new();
        number_props.insert("isFinite".into(), Value::NativeFunction(13));
        number_props.insert("isNaN".into(), Value::NativeFunction(12));
        number_props.insert("parseFloat".into(), Value::NativeFunction(11));
        number_props.insert("parseInt".into(), Value::NativeFunction(10));
        self.heap.push(HeapValue::Object(JsObject { properties: number_props, prototype: None }));
        self.globals.insert("Number".into(), Value::Object(number_obj_idx));
    }

    pub fn execute(&mut self, module: &CompiledModule) -> Result<Value> {
        self.current_module = Some(module.clone());
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
                                let closure_count = f.closure.len();

                                self.call_stack.push(CallFrame {
                                    return_address,
                                    base_pointer,
                                    closure_var_count: closure_count,
                                    func_heap_idx: Some(func_idx),
                                    this_value: None,
                                    is_construct: false,
                                });

                                for closure_var in &f.closure {
                                    self.stack.push(closure_var.clone());
                                }

                                for arg in args {
                                    self.stack.push(arg);
                                }

                                pc = f.bytecode_index;
                                continue;
                            }
                        }
                        Value::NativeFunction(native_idx) => {
                            let result = self.call_native(native_idx, &Value::Undefined, &args)?;
                            self.stack.push(result);
                        }
                        _ => {
                            return Err(Error::TypeError(format!("{} is not a function", self.value_to_string(&callee))));
                        }
                    }
                }
                Instruction::CallMethod(argc) => {
                    let mut args = Vec::new();
                    for _ in 0..*argc {
                        args.push(self.stack.pop()
                            .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?);
                    }
                    args.reverse();

                    let key = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let object = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;

                    let method = self.get_property(&object, &key)?;

                    match method {
                        Value::Function(func_idx) => {
                            if let HeapValue::Function(f) = &self.heap[func_idx] {
                                let f_clone = f.clone();
                                let return_address = pc + 1;
                                let base_pointer = self.stack.len();
                                let closure_count = f_clone.closure.len();

                                self.call_stack.push(CallFrame {
                                    return_address,
                                    base_pointer,
                                    closure_var_count: closure_count,
                                    func_heap_idx: Some(func_idx),
                                    this_value: Some(object),
                                    is_construct: false,
                                });

                                for closure_var in &f_clone.closure {
                                    self.stack.push(closure_var.clone());
                                }

                                for arg in args {
                                    self.stack.push(arg);
                                }

                                pc = f_clone.bytecode_index;
                                continue;
                            }
                        }
                        Value::NativeFunction(native_idx) => {
                            let result = self.call_native(native_idx, &object, &args)?;
                            self.stack.push(result);
                        }
                        _ => {
                            return Err(Error::TypeError(format!("{} is not a function", self.value_to_string(&method))));
                        }
                    }
                }
                Instruction::Construct(argc) => {
                    let mut args = Vec::new();
                    for _ in 0..*argc {
                        args.push(self.stack.pop()
                            .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?);
                    }
                    args.reverse();

                    let constructor = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;

                    let proto_idx = if let Value::Function(func_idx) = &constructor {
                        if let Value::Object(proto_obj_idx) = self.get_property(&constructor, &Value::String("prototype".to_string()))? {
                            Some(proto_obj_idx)
                        } else {
                            None
                        }
                    } else {
                        return Err(Error::TypeError(format!("{} is not a constructor", self.value_to_string(&constructor))));
                    };

                    let new_obj_heap_idx = self.heap.len();
                    self.heap.push(HeapValue::Object(JsObject::with_prototype(proto_idx)));
                    let this_val = Value::Object(new_obj_heap_idx);

                    if let Value::Function(func_idx) = constructor {
                        if let HeapValue::Function(f) = &self.heap[func_idx] {
                            let f_clone = f.clone();
                            let return_address = pc + 1;
                            let base_pointer = self.stack.len();
                            let closure_count = f_clone.closure.len();

                            self.call_stack.push(CallFrame {
                                return_address,
                                base_pointer,
                                closure_var_count: closure_count,
                                func_heap_idx: Some(func_idx),
                                this_value: Some(this_val.clone()),
                                is_construct: true,
                            });

                            for closure_var in &f_clone.closure {
                                self.stack.push(closure_var.clone());
                            }

                            for arg in args {
                                self.stack.push(arg);
                            }

                            pc = f_clone.bytecode_index;
                            continue;
                        }
                    }
                }
                Instruction::LoadThis => {
                    let this = self.call_stack.last()
                        .and_then(|f| f.this_value.clone())
                        .unwrap_or(Value::Undefined);
                    self.stack.push(this);
                }
                Instruction::Return => {
                    let return_value = self.stack.pop()
                        .unwrap_or(Value::Undefined);

                    if let Some(frame) = self.call_stack.pop() {
                        if frame.closure_var_count > 0 {
                            if let Some(heap_idx) = frame.func_heap_idx {
                                if let HeapValue::Function(f) = &mut self.heap[heap_idx] {
                                    f.closure.clear();
                                    for i in 0..frame.closure_var_count {
                                        let val = self.stack.get(frame.base_pointer + i)
                                            .cloned()
                                            .unwrap_or(Value::Undefined);
                                        f.closure.push(val);
                                    }
                                }
                            }
                        }

                        let final_value = if frame.is_construct {
                            match &return_value {
                                Value::Object(_) | Value::Array(_) | Value::Function(_) => return_value,
                                _ => frame.this_value.clone().unwrap_or(return_value),
                            }
                        } else {
                            return_value
                        };

                        self.stack.truncate(frame.base_pointer);
                        self.stack.push(final_value);
                        pc = frame.return_address;
                        continue;
                    } else {
                        return Ok(return_value);
                    }
                }
                Instruction::Pop => {
                    self.stack.pop();
                }
                Instruction::Dup => {
                    let val = self.stack.last()
                        .cloned()
                        .unwrap_or(Value::Undefined);
                    self.stack.push(val);
                }
                Instruction::MakeFunction(func_idx) => {
                    let func_info = module.functions[*func_idx as usize].clone();

                    let proto_obj_idx = self.heap.len();
                    self.heap.push(HeapValue::Object(JsObject::new()));

                    let heap_idx = self.heap.len();
                    self.heap.push(HeapValue::Function(JsFunction {
                        name: func_info.name,
                        params: func_info.params,
                        bytecode_index: func_info.bytecode_index,
                        closure: Vec::new(),
                        prototype: Some(proto_obj_idx),
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

                    let proto_obj_idx = self.heap.len();
                    self.heap.push(HeapValue::Object(JsObject::new()));

                    let heap_idx = self.heap.len();
                    self.heap.push(HeapValue::Function(JsFunction {
                        name: func_info.name,
                        params: func_info.params,
                        bytecode_index: func_info.bytecode_index,
                        closure: closure_vars,
                        prototype: Some(proto_obj_idx),
                    }));

                    self.stack.push(Value::Function(heap_idx));
                }
                Instruction::NewObject => {
                    let heap_idx = self.heap.len();
                    self.heap.push(HeapValue::Object(JsObject::new()));
                    self.stack.push(Value::Object(heap_idx));
                }
                Instruction::SetProperty => {
                    let key = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let value = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let object = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;

                    if let Value::Object(obj_idx) = &object {
                        if let HeapValue::Object(obj) = &mut self.heap[*obj_idx] {
                            if let Value::String(key_str) = &key {
                                obj.properties.insert(key_str.clone(), value);
                            }
                        }
                    }
                    self.stack.push(object);
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
                Instruction::BitNot => {
                    let value = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let num = self.to_number(&value)?;
                    self.stack.push(Value::Integer(!(num as i64) as i64));
                }
                Instruction::Void => {
                    self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    self.stack.push(Value::Undefined);
                }
                Instruction::Delete => {
                    let key = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let object = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;

                    let result = self.delete_property(&object, &key);
                    self.stack.push(result);
                }
                Instruction::InstanceOf => {
                    let right = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let left = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;

                    let result = self.instanceof_check(&left, &right)?;
                    self.stack.push(result);
                }
                Instruction::In => {
                    let right = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let left = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;

                    let result = self.in_check(&left, &right)?;
                    self.stack.push(result);
                }
                Instruction::Throw => {
                    let value = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    return Err(Error::RuntimeError(format!("Thrown: {}", self.value_to_string(&value))));
                }
                Instruction::ToString => {
                    let value = self.stack.pop()
                        .ok_or_else(|| Error::RuntimeError("Stack underflow".into()))?;
                    let s = self.value_to_string_raw(&value);
                    self.stack.push(Value::String(s));
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

    pub fn call_value(&mut self, callee: &Value, this: &Value, args: &[Value]) -> Result<Value> {
        match callee {
            Value::Function(func_idx) => {
                if let HeapValue::Function(f) = &self.heap[*func_idx] {
                    let f_clone = f.clone();
                    let return_address = self.current_module.as_ref()
                        .map(|m| m.instructions.len())
                        .unwrap_or(0);
                    let base_pointer = self.stack.len();
                    let closure_count = f_clone.closure.len();

                    self.call_stack.push(CallFrame {
                        return_address,
                        base_pointer,
                        closure_var_count: closure_count,
                        func_heap_idx: Some(*func_idx),
                        this_value: Some(this.clone()),
                        is_construct: false,
                    });

                    for closure_var in &f_clone.closure {
                        self.stack.push(closure_var.clone());
                    }
                    for arg in args {
                        self.stack.push(arg.clone());
                    }

                    if let Some(module) = self.current_module.clone() {
                        self.execute_from(&module, f_clone.bytecode_index)
                    } else {
                        Ok(Value::Undefined)
                    }
                } else {
                    Err(Error::TypeError("Not a function".into()))
                }
            }
            Value::NativeFunction(native_idx) => {
                self.call_native(*native_idx, this, args)
            }
            _ => Err(Error::TypeError(format!("{} is not a function", self.value_to_string(callee)))),
        }
    }

    fn call_native(&mut self, idx: usize, this: &Value, args: &[Value]) -> Result<Value> {
        if idx < NATIVE_TABLE.len() {
            NATIVE_TABLE[idx](self, this, args)
        } else {
            Err(Error::RuntimeError(format!("Unknown native function index: {}", idx)))
        }
    }
    
    fn get_property(&self, object: &Value, key: &Value) -> Result<Value> {
        match object {
            Value::Null | Value::Undefined => {
                return Err(Error::TypeError(format!(
                    "Cannot read properties of {} (reading '{}')",
                    self.value_to_string(object),
                    self.value_to_string(key)
                )));
            }
            Value::Object(obj_idx) => {
                if let HeapValue::Object(obj) = &self.heap[*obj_idx] {
                    if let Value::String(key_str) = key {
                        if let Some(val) = obj.properties.get(key_str) {
                            return Ok(val.clone());
                        }
                        if let Some(proto_idx) = obj.prototype {
                            let proto_val = Value::Object(proto_idx);
                            return self.get_property(&proto_val, key);
                        }
                    }
                }
            }
            Value::Array(arr_idx) => {
                if let HeapValue::Array(arr) = &self.heap[*arr_idx] {
                    match key {
                        Value::String(key_str) => {
                            if key_str == "length" {
                                return Ok(Value::Float(arr.elements.len() as f64));
                            }
                            if let Ok(index) = key_str.parse::<usize>() {
                                return Ok(arr.elements.get(index).cloned().unwrap_or(Value::Undefined));
                            }
                            return self.get_array_method(key_str);
                        }
                        Value::Integer(index) => {
                            let idx = *index as usize;
                            return Ok(arr.elements.get(idx).cloned().unwrap_or(Value::Undefined));
                        }
                        Value::Float(f) => {
                            let idx = *f as usize;
                            return Ok(arr.elements.get(idx).cloned().unwrap_or(Value::Undefined));
                        }
                        _ => {}
                    }
                }
            }
            Value::String(s) => {
                return self.get_property_from_primitive_string(s, key);
            }
            Value::Integer(_) | Value::Float(_) => {
                return self.get_property_from_primitive_number(object, key);
            }
            Value::Boolean(_) => {
                return self.get_property_from_primitive_boolean(object, key);
            }
            Value::Function(func_idx) => {
                if let Value::String(key_str) = key {
                    if key_str == "prototype" {
                        if let HeapValue::Function(f) = &self.heap[*func_idx] {
                            if let Some(proto_idx) = f.prototype {
                                return Ok(Value::Object(proto_idx));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(Value::Undefined)
    }

    fn get_array_method(&self, name: &str) -> Result<Value> {
        let idx = match name {
            "push" => 31,
            "pop" => 32,
            "shift" => 33,
            "unshift" => 34,
            "slice" => 35,
            "splice" => 36,
            "indexOf" => 37,
            "includes" => 38,
            "find" => 39,
            "findIndex" => 40,
            "map" => 41,
            "filter" => 42,
            "reduce" => 43,
            "forEach" => 44,
            "some" => 45,
            "every" => 46,
            "join" => 47,
            "reverse" => 48,
            "sort" => 49,
            "concat" => 50,
            "flat" => 51,
            _ => return Ok(Value::Undefined),
        };
        Ok(Value::NativeFunction(idx))
    }

    fn get_property_from_primitive_string(&self, s: &str, key: &Value) -> Result<Value> {
        if let Value::String(key_str) = key {
            match key_str.as_str() {
                "length" => return Ok(Value::Float(s.len() as f64)),
                _ => {}
            }
            return self.get_string_method(key_str);
        }
        Ok(Value::Undefined)
    }

    fn get_string_method(&self, name: &str) -> Result<Value> {
        let idx = match name {
            "charAt" => 52,
            "charCodeAt" => 53,
            "slice" => 54,
            "substring" => 55,
            "indexOf" => 56,
            "includes" => 57,
            "replace" => 58,
            "split" => 59,
            "trim" => 60,
            "toLowerCase" => 61,
            "toUpperCase" => 62,
            "startsWith" => 63,
            "endsWith" => 64,
            "repeat" => 65,
            "padStart" => 66,
            "padEnd" => 67,
            _ => return Ok(Value::Undefined),
        };
        Ok(Value::NativeFunction(idx))
    }

    fn get_property_from_primitive_number(&self, n: &Value, key: &Value) -> Result<Value> {
        if let Value::String(key_str) = key {
            match key_str.as_str() {
                "toString" | "toFixed" | "valueOf" => {
                    return Ok(self.make_native_number_method(key_str));
                }
                _ => {}
            }
        }
        Ok(Value::Undefined)
    }

    fn get_property_from_primitive_boolean(&self, b: &Value, key: &Value) -> Result<Value> {
        if let Value::String(key_str) = key {
            match key_str.as_str() {
                "toString" | "valueOf" => {
                    return Ok(self.make_native_boolean_method(key_str));
                }
                _ => {}
            }
        }
        Ok(Value::Undefined)
    }

    fn make_native_number_method(&self, _name: &str) -> Value {
        Value::NativeFunction(0)
    }

    fn make_native_boolean_method(&self, _name: &str) -> Value {
        Value::NativeFunction(0)
    }

    fn delete_property(&mut self, object: &Value, key: &Value) -> Value {
        match object {
            Value::Object(obj_idx) => {
                if let HeapValue::Object(obj) = &mut self.heap[*obj_idx] {
                    if let Value::String(key_str) = key {
                        if obj.properties.remove(key_str).is_some() {
                            return Value::Boolean(true);
                        }
                    }
                }
                Value::Boolean(false)
            }
            Value::Array(arr_idx) => {
                if let Value::String(key_str) = key {
                    if let Ok(index) = key_str.parse::<usize>() {
                        if let HeapValue::Array(arr) = &mut self.heap[*arr_idx] {
                            if index < arr.elements.len() {
                                arr.elements[index] = Value::Undefined;
                                return Value::Boolean(true);
                            }
                        }
                    }
                }
                Value::Boolean(false)
            }
            _ => Value::Boolean(true),
        }
    }

    fn instanceof_check(&self, left: &Value, right: &Value) -> Result<Value> {
        let proto_key = Value::String("prototype".to_string());
        let right_proto = match self.get_property(right, &proto_key) {
            Ok(val) => val,
            Err(_) => return Ok(Value::Boolean(false)),
        };

        let proto_idx = match &right_proto {
            Value::Object(idx) => *idx,
            _ => return Ok(Value::Boolean(false)),
        };

        let mut current = left.clone();
        loop {
            match &current {
                Value::Object(obj_idx) => {
                    if let HeapValue::Object(obj) = &self.heap[*obj_idx] {
                        if obj.prototype == Some(proto_idx) {
                            return Ok(Value::Boolean(true));
                        }
                        if let Some(parent_idx) = obj.prototype {
                            current = Value::Object(parent_idx);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                Value::Array(arr_idx) => {
                    break;
                }
                _ => break,
            }
        }
        Ok(Value::Boolean(false))
    }

    fn in_check(&self, key: &Value, object: &Value) -> Result<Value> {
        match object {
            Value::Object(obj_idx) => {
                if let HeapValue::Object(obj) = &self.heap[*obj_idx] {
                    if let Value::String(key_str) = key {
                        if obj.properties.contains_key(key_str) {
                            return Ok(Value::Boolean(true));
                        }
                        if let Some(proto_idx) = obj.prototype {
                            let proto_val = Value::Object(proto_idx);
                            return self.in_check(key, &proto_val);
                        }
                    }
                }
                Ok(Value::Boolean(false))
            }
            Value::Array(arr_idx) => {
                if let HeapValue::Array(arr) = &self.heap[*arr_idx] {
                    if let Value::String(key_str) = key {
                        if key_str == "length" {
                            return Ok(Value::Boolean(true));
                        }
                        if let Ok(index) = key_str.parse::<usize>() {
                            return Ok(Value::Boolean(index < arr.elements.len()));
                        }
                    }
                }
                Ok(Value::Boolean(false))
            }
            Value::String(s) => {
                if let Value::String(key_str) = key {
                    if key_str == "length" {
                        return Ok(Value::Boolean(true));
                    }
                    if let Ok(index) = key_str.parse::<usize>() {
                        return Ok(Value::Boolean(index < s.len()));
                    }
                }
                Ok(Value::Boolean(false))
            }
            _ => Ok(Value::Boolean(false)),
        }
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
    
    pub(crate) fn is_truthy(&self, value: &Value) -> bool {
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
    
    fn to_number(&self, value: &Value) -> Result<f64> {
        match value {
            Value::Integer(n) => Ok(*n as f64),
            Value::Float(n) => Ok(*n),
            Value::Boolean(b) => Ok(if *b { 1.0 } else { 0.0 }),
            Value::Null => Ok(0.0),
            Value::Undefined => Ok(f64::NAN),
            Value::String(s) => {
                if s.is_empty() { Ok(0.0) }
                else { Ok(s.parse::<f64>().unwrap_or(f64::NAN)) }
            }
            _ => Ok(f64::NAN),
        }
    }

    fn value_to_string_raw(&self, value: &Value) -> String {
        match value {
            Value::Undefined => "undefined".to_string(),
            Value::Null => "null".to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Integer(n) => n.to_string(),
            Value::Float(n) => {
                if *n == (*n as i64) as f64 {
                    (*n as i64).to_string()
                } else {
                    n.to_string()
                }
            }
            Value::String(s) => s.clone(),
            Value::BigInt(n) => format!("{}n", n),
            Value::Function(_) => "[Function]".to_string(),
            Value::NativeFunction(_) => "[Native Function]".to_string(),
            Value::Object(_) => "[Object]".to_string(),
            Value::Array(_) => "[Array]".to_string(),
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
