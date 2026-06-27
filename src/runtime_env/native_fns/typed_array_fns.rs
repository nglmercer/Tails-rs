use crate::errors::{Error, Result};
use crate::objects::js_array::{TypedArray, TypedArrayType};
use crate::objects::Value;
use crate::vm::interpreter::Interpreter;

fn get_typed_array(interp: &Interpreter, idx: usize) -> Result<&TypedArray> {
    if let crate::vm::interpreter::HeapValue::TypedArray(arr) = &interp.heap[idx] {
        Ok(arr)
    } else {
        Err(Error::TypeError("Not a TypedArray".into()))
    }
}

fn get_typed_array_mut(interp: &mut Interpreter, idx: usize) -> Result<&mut TypedArray> {
    if let crate::vm::interpreter::HeapValue::TypedArray(arr) = &mut interp.heap[idx] {
        Ok(arr)
    } else {
        Err(Error::TypeError("Not a TypedArray".into()))
    }
}

fn parse_typed_array_type(name: &str) -> Option<TypedArrayType> {
    match name {
        "Int8Array" => Some(TypedArrayType::Int8Array),
        "Uint8Array" => Some(TypedArrayType::Uint8Array),
        "Uint8ClampedArray" => Some(TypedArrayType::Uint8ClampedArray),
        "Int16Array" => Some(TypedArrayType::Int16Array),
        "Uint16Array" => Some(TypedArrayType::Uint16Array),
        "Int32Array" => Some(TypedArrayType::Int32Array),
        "Uint32Array" => Some(TypedArrayType::Uint32Array),
        "Float32Array" => Some(TypedArrayType::Float32Array),
        "Float64Array" => Some(TypedArrayType::Float64Array),
        "BigInt64Array" => Some(TypedArrayType::BigInt64Array),
        "BigUint64Array" => Some(TypedArrayType::BigUint64Array),
        _ => None,
    }
}

pub(super) fn native_typed_array_constructor(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let type_name = match args.first() {
        Some(Value::String(s)) => s.as_str(),
        _ => {
            return Err(Error::TypeError(
                "TypedArray constructor requires type name".into(),
            ))
        }
    };

    let kind = parse_typed_array_type(type_name)
        .ok_or_else(|| Error::TypeError(format!("Unknown TypedArray type: {}", type_name)))?;

    let length = match args.get(1) {
        Some(Value::Integer(i)) => *i as usize,
        Some(Value::Float(f)) => *f as usize,
        Some(Value::Array(arr_idx)) => {
            if let crate::vm::interpreter::HeapValue::Array(arr) = &interp.heap[*arr_idx] {
                arr.elements.len()
            } else {
                0
            }
        }
        _ => 0,
    };

    let typed_array = TypedArray::new(kind.clone(), length);
    let heap_idx = interp.heap.len();
    interp
        .heap
        .push(crate::vm::interpreter::HeapValue::TypedArray(typed_array));
    Ok(Value::TypedArray(heap_idx))
}

pub(super) fn native_typed_array_from(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let type_name = match args.first() {
        Some(Value::String(s)) => s.as_str(),
        _ => {
            return Err(Error::TypeError(
                "TypedArray.from requires type name".into(),
            ))
        }
    };

    let kind = parse_typed_array_type(type_name)
        .ok_or_else(|| Error::TypeError(format!("Unknown TypedArray type: {}", type_name)))?;

    let iterable = args.get(1).cloned().unwrap_or(Value::Undefined);
    let map_fn = args.get(2).cloned();

    let mut elements = Vec::new();

    if let Value::Array(arr_idx) = &iterable {
        // Clone elements to avoid borrow issues
        let arr_elements: Vec<Value> =
            if let crate::vm::interpreter::HeapValue::Array(arr) = &interp.heap[*arr_idx] {
                arr.elements.clone()
            } else {
                Vec::new()
            };

        for elem in arr_elements {
            let val = if let Some(map_fn) = &map_fn {
                interp.call_value(map_fn, &Value::Undefined, std::slice::from_ref(&elem))?
            } else {
                elem.clone()
            };
            elements.push(val);
        }
    }

    let length = elements.len();
    let typed_array = TypedArray::new(kind.clone(), length);

    let mut arr = typed_array;
    for (i, elem) in elements.iter().enumerate() {
        if let Value::Integer(i_val) = elem {
            match &kind {
                TypedArrayType::Int8Array => arr.set_value(i, *i_val as i8),
                TypedArrayType::Uint8Array | TypedArrayType::Uint8ClampedArray => {
                    arr.set_value(i, *i_val as u8)
                }
                TypedArrayType::Int16Array => arr.set_value(i, *i_val as i16),
                TypedArrayType::Uint16Array => arr.set_value(i, *i_val as u16),
                TypedArrayType::Int32Array => arr.set_value(i, *i_val as i32),
                TypedArrayType::Uint32Array => arr.set_value(i, *i_val as u32),
                TypedArrayType::Float32Array => arr.set_value(i, *i_val as f32),
                TypedArrayType::Float64Array => arr.set_value(i, *i_val as f64),
                _ => {}
            }
        } else if let Value::Float(f_val) = elem {
            match kind {
                TypedArrayType::Float32Array => arr.set_value(i, *f_val as f32),
                TypedArrayType::Float64Array => arr.set_value(i, *f_val),
                _ => {}
            }
        }
    }

    let heap_idx = interp.heap.len();
    interp
        .heap
        .push(crate::vm::interpreter::HeapValue::TypedArray(arr));
    Ok(Value::TypedArray(heap_idx))
}

pub(super) fn native_typed_array_of(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let type_name = match args.first() {
        Some(Value::String(s)) => s.as_str(),
        _ => return Err(Error::TypeError("TypedArray.of requires type name".into())),
    };

    let kind = parse_typed_array_type(type_name)
        .ok_or_else(|| Error::TypeError(format!("Unknown TypedArray type: {}", type_name)))?;

    let length = args.len().saturating_sub(1);
    let typed_array = TypedArray::new(kind.clone(), length);

    let mut arr = typed_array;
    for (i, elem) in args.iter().skip(1).enumerate() {
        if let Value::Integer(i_val) = elem {
            match &kind {
                TypedArrayType::Int8Array => arr.set_value(i, *i_val as i8),
                TypedArrayType::Uint8Array | TypedArrayType::Uint8ClampedArray => {
                    arr.set_value(i, *i_val as u8)
                }
                TypedArrayType::Int16Array => arr.set_value(i, *i_val as i16),
                TypedArrayType::Uint16Array => arr.set_value(i, *i_val as u16),
                TypedArrayType::Int32Array => arr.set_value(i, *i_val as i32),
                TypedArrayType::Uint32Array => arr.set_value(i, *i_val as u32),
                TypedArrayType::Float32Array => arr.set_value(i, *i_val as f32),
                TypedArrayType::Float64Array => arr.set_value(i, *i_val as f64),
                _ => {}
            }
        } else if let Value::Float(f_val) = elem {
            match &kind {
                TypedArrayType::Float32Array => arr.set_value(i, *f_val as f32),
                TypedArrayType::Float64Array => arr.set_value(i, *f_val),
                _ => {}
            }
        }
    }

    let heap_idx = interp.heap.len();
    interp
        .heap
        .push(crate::vm::interpreter::HeapValue::TypedArray(arr));
    Ok(Value::TypedArray(heap_idx))
}

pub(super) fn native_typed_array_get(
    interp: &mut Interpreter,
    this: &Value,
    args: &[Value],
) -> Result<Value> {
    let idx = match this {
        Value::TypedArray(idx) => *idx,
        _ => return Err(Error::TypeError("Not a TypedArray".into())),
    };

    let index = match args.first() {
        Some(Value::Integer(i)) => *i as usize,
        Some(Value::Float(f)) => *f as usize,
        _ => return Ok(Value::Undefined),
    };

    let arr = get_typed_array(interp, idx)?;

    match arr.kind {
        TypedArrayType::Int8Array => Ok(arr
            .get::<i8>(index)
            .map(|v| Value::Integer(v as i64))
            .unwrap_or(Value::Undefined)),
        TypedArrayType::Uint8Array | TypedArrayType::Uint8ClampedArray => Ok(arr
            .get::<u8>(index)
            .map(|v| Value::Integer(v as i64))
            .unwrap_or(Value::Undefined)),
        TypedArrayType::Int16Array => Ok(arr
            .get::<i16>(index)
            .map(|v| Value::Integer(v as i64))
            .unwrap_or(Value::Undefined)),
        TypedArrayType::Uint16Array => Ok(arr
            .get::<u16>(index)
            .map(|v| Value::Integer(v as i64))
            .unwrap_or(Value::Undefined)),
        TypedArrayType::Int32Array => Ok(arr
            .get::<i32>(index)
            .map(|v| Value::Integer(v as i64))
            .unwrap_or(Value::Undefined)),
        TypedArrayType::Uint32Array => Ok(arr
            .get::<u32>(index)
            .map(|v| Value::Integer(v as i64))
            .unwrap_or(Value::Undefined)),
        TypedArrayType::Float32Array => Ok(arr
            .get::<f32>(index)
            .map(|v| Value::Float(v as f64))
            .unwrap_or(Value::Undefined)),
        TypedArrayType::Float64Array => Ok(arr
            .get::<f64>(index)
            .map(Value::Float)
            .unwrap_or(Value::Undefined)),
        _ => Ok(Value::Undefined),
    }
}

pub(super) fn native_typed_array_set(
    interp: &mut Interpreter,
    this: &Value,
    args: &[Value],
) -> Result<Value> {
    let idx = match this {
        Value::TypedArray(idx) => *idx,
        _ => return Err(Error::TypeError("Not a TypedArray".into())),
    };

    let index = match args.first() {
        Some(Value::Integer(i)) => *i as usize,
        Some(Value::Float(f)) => *f as usize,
        _ => return Err(Error::TypeError("Invalid index".into())),
    };

    let value = args.get(1).cloned().unwrap_or(Value::Undefined);

    let arr = get_typed_array_mut(interp, idx)?;

    match (arr.kind.clone(), value) {
        (TypedArrayType::Int8Array, Value::Integer(v)) => arr.set_value(index, v as i8),
        (TypedArrayType::Uint8Array | TypedArrayType::Uint8ClampedArray, Value::Integer(v)) => {
            arr.set_value(index, v as u8)
        }
        (TypedArrayType::Int16Array, Value::Integer(v)) => arr.set_value(index, v as i16),
        (TypedArrayType::Uint16Array, Value::Integer(v)) => arr.set_value(index, v as u16),
        (TypedArrayType::Int32Array, Value::Integer(v)) => arr.set_value(index, v as i32),
        (TypedArrayType::Uint32Array, Value::Integer(v)) => arr.set_value(index, v as u32),
        (TypedArrayType::Float32Array, Value::Float(v)) => arr.set_value(index, v as f32),
        (TypedArrayType::Float32Array, Value::Integer(v)) => arr.set_value(index, v as f32),
        (TypedArrayType::Float64Array, Value::Float(v)) => arr.set_value(index, v),
        (TypedArrayType::Float64Array, Value::Integer(v)) => arr.set_value(index, v as f64),
        _ => {}
    }

    Ok(Value::Undefined)
}

pub(super) fn native_typed_array_length(
    interp: &mut Interpreter,
    this: &Value,
    _args: &[Value],
) -> Result<Value> {
    let idx = match this {
        Value::TypedArray(idx) => *idx,
        _ => return Err(Error::TypeError("Not a TypedArray".into())),
    };

    let arr = get_typed_array(interp, idx)?;
    let element_size = TypedArray::element_size(&arr.kind);
    let length = arr.byte_length / element_size;
    Ok(Value::Float(length as f64))
}

pub(super) fn native_typed_array_byte_length(
    interp: &mut Interpreter,
    this: &Value,
    _args: &[Value],
) -> Result<Value> {
    let idx = match this {
        Value::TypedArray(idx) => *idx,
        _ => return Err(Error::TypeError("Not a TypedArray".into())),
    };

    let arr = get_typed_array(interp, idx)?;
    Ok(Value::Float(arr.byte_length as f64))
}

pub(super) fn native_typed_array_byte_offset(
    interp: &mut Interpreter,
    this: &Value,
    _args: &[Value],
) -> Result<Value> {
    let idx = match this {
        Value::TypedArray(idx) => *idx,
        _ => return Err(Error::TypeError("Not a TypedArray".into())),
    };

    let arr = get_typed_array(interp, idx)?;
    Ok(Value::Float(arr.byte_offset as f64))
}

pub(super) fn native_typed_array_subarray(
    interp: &mut Interpreter,
    this: &Value,
    args: &[Value],
) -> Result<Value> {
    let idx = match this {
        Value::TypedArray(idx) => *idx,
        _ => return Err(Error::TypeError("Not a TypedArray".into())),
    };

    let begin = match args.first() {
        Some(Value::Integer(i)) => *i as usize,
        Some(Value::Float(f)) => *f as usize,
        _ => 0,
    };

    let end = match args.get(1) {
        Some(Value::Integer(i)) => *i as usize,
        Some(Value::Float(f)) => *f as usize,
        _ => {
            let arr = get_typed_array(interp, idx)?;
            let element_size = TypedArray::element_size(&arr.kind);
            arr.byte_length / element_size
        }
    };

    let arr = get_typed_array(interp, idx)?;
    let element_size = TypedArray::element_size(&arr.kind);
    let byte_begin = begin * element_size;
    let byte_end = end * element_size;
    let new_byte_length = byte_end.saturating_sub(byte_begin);

    let new_buffer = arr.buffer[byte_begin..byte_begin + new_byte_length].to_vec();
    let new_arr = TypedArray::from_buffer(arr.kind.clone(), new_buffer, 0);

    let heap_idx = interp.heap.len();
    interp
        .heap
        .push(crate::vm::interpreter::HeapValue::TypedArray(new_arr));
    Ok(Value::TypedArray(heap_idx))
}

pub(super) fn native_typed_array_slice(
    interp: &mut Interpreter,
    this: &Value,
    args: &[Value],
) -> Result<Value> {
    let idx = match this {
        Value::TypedArray(idx) => *idx,
        _ => return Err(Error::TypeError("Not a TypedArray".into())),
    };

    let begin = match args.first() {
        Some(Value::Integer(i)) => *i as usize,
        Some(Value::Float(f)) => *f as usize,
        _ => 0,
    };

    let end = match args.get(1) {
        Some(Value::Integer(i)) => *i as usize,
        Some(Value::Float(f)) => *f as usize,
        _ => {
            let arr = get_typed_array(interp, idx)?;
            let element_size = TypedArray::element_size(&arr.kind);
            arr.byte_length / element_size
        }
    };

    let arr = get_typed_array(interp, idx)?;
    let element_size = TypedArray::element_size(&arr.kind);
    let byte_begin = begin * element_size;
    let byte_end = end * element_size;
    let new_byte_length = byte_end.saturating_sub(byte_begin);

    let new_buffer = arr.buffer[byte_begin..byte_begin + new_byte_length].to_vec();
    let new_arr = TypedArray::from_buffer(arr.kind.clone(), new_buffer, 0);

    let heap_idx = interp.heap.len();
    interp
        .heap
        .push(crate::vm::interpreter::HeapValue::TypedArray(new_arr));
    Ok(Value::TypedArray(heap_idx))
}
