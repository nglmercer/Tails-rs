use crate::errors::Result;
use crate::objects::Value;
use crate::vm::interpreter::Interpreter;

use super::helpers::{to_f64, to_i64, to_string_value};

pub(super) fn native_parse_int(
    _interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let s = match args.first() {
        Some(Value::String(s)) => s.clone(),
        Some(v) => to_string_value(_interp, v),
        None => return Ok(Value::Float(f64::NAN)),
    };
    let radix = args.get(1).map(|r| to_i64(r)).unwrap_or(10);
    let _trimmed = s.trim().trim_start_matches(|c: char| {
        c.is_ascii_digit() || c == '-' || c == '+' || c.is_alphabetic()
    });
    let actual = s.trim();
    let result = i64::from_str_radix(
        actual.trim_start_matches(|c: char| c == '+' || c == '-'),
        radix as u32,
    );
    match result {
        Ok(n) => {
            if actual.starts_with('-') {
                Ok(Value::Float(-n as f64))
            } else {
                Ok(Value::Float(n as f64))
            }
        }
        Err(_) => Ok(Value::Float(f64::NAN)),
    }
}

pub(super) fn native_parse_float(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let s = match args.first() {
        Some(Value::String(s)) => s.clone(),
        Some(v) => to_string_value(interp, v),
        None => return Ok(Value::Float(f64::NAN)),
    };
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Ok(Value::Float(f64::NAN));
    }
    match trimmed.parse::<f64>() {
        Ok(n) => Ok(Value::Float(n)),
        Err(_) => Ok(Value::Float(f64::NAN)),
    }
}

pub(super) fn native_is_nan(
    _interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let n = args.first().map(|v| to_f64(v)).unwrap_or(f64::NAN);
    Ok(Value::Boolean(n.is_nan()))
}

pub(super) fn native_is_finite(
    _interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let n = args.first().map(|v| to_f64(v)).unwrap_or(f64::NAN);
    Ok(Value::Boolean(n.is_finite()))
}

pub(super) fn native_set_timeout(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let callback = args.first().cloned().unwrap_or(Value::Undefined);
    let _delay = args.get(1).map(|v| to_f64(v)).unwrap_or(0.0);
    let id = interp.async_runtime.enqueue_macrotask(callback);
    Ok(Value::Float(id as f64))
}

pub(super) fn native_set_interval(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let callback = args.first().cloned().unwrap_or(Value::Undefined);
    let delay = args.get(1).map(|v| to_f64(v)).unwrap_or(0.0);
    let id = interp.async_runtime.enqueue_interval(callback, delay);
    Ok(Value::Float(id as f64))
}

pub(super) fn native_clear_timeout(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    if let Some(Value::Float(id)) = args.first() {
        interp.async_runtime.cancel_timer(*id as u32);
    }
    Ok(Value::Undefined)
}

pub(super) fn native_clear_interval(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    if let Some(Value::Float(id)) = args.first() {
        interp.async_runtime.cancel_timer(*id as u32);
    }
    Ok(Value::Undefined)
}

pub(super) fn native_number_parse_int(
    interp: &mut Interpreter,
    this: &Value,
    args: &[Value],
) -> Result<Value> {
    native_parse_int(interp, this, args)
}

pub(super) fn native_number_parse_float(
    interp: &mut Interpreter,
    this: &Value,
    args: &[Value],
) -> Result<Value> {
    native_parse_float(interp, this, args)
}

pub(super) fn native_number_is_nan(
    interp: &mut Interpreter,
    this: &Value,
    args: &[Value],
) -> Result<Value> {
    native_is_nan(interp, this, args)
}

pub(super) fn native_number_is_finite(
    interp: &mut Interpreter,
    this: &Value,
    args: &[Value],
) -> Result<Value> {
    native_is_finite(interp, this, args)
}
