use crate::objects::Value;

pub type NativeFunction = fn(args: &[Value]) -> Result<Value, String>;

pub struct NativeRegistry {
    functions: Vec<NativeFunction>,
}

impl NativeRegistry {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }
    
    pub fn register(&mut self, func: NativeFunction) -> usize {
        let id = self.functions.len();
        self.functions.push(func);
        id
    }
    
    pub fn get(&self, id: usize) -> Option<&NativeFunction> {
        self.functions.get(id)
    }
    
    pub fn call(&self, id: usize, args: &[Value]) -> Result<Value, String> {
        let func = self.get(id)
            .ok_or_else(|| "Function not found".to_string())?;
        func(args)
    }
}

impl Default for NativeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn tails_register_native(
    registry: *mut NativeRegistry,
    func: NativeFunction,
) -> usize {
    if registry.is_null() {
        return usize::MAX;
    }
    
    let registry = unsafe { &mut *registry };
    registry.register(func)
}

#[no_mangle]
pub extern "C" fn tails_call_native(
    registry: *const NativeRegistry,
    id: usize,
    args: *const TailsValue,
    args_len: usize,
) -> TailsValue {
    if registry.is_null() {
        return TailsValue {
            tag: 0,
            data: 0,
        };
    }
    
    let registry = unsafe { &*registry };
    let args = if args.is_null() || args_len == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(args, args_len) }
    };
    
    let values: Vec<Value> = args.iter()
        .map(|v| tails_value_to_value(*v))
        .collect();
    
    match registry.call(id, &values) {
        Ok(value) => value_to_tails_value(value),
        Err(_) => TailsValue {
            tag: 0,
            data: 0,
        },
    }
}

use super::{TailsValue, value_to_tails_value, tails_value_to_value};
