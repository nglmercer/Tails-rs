use crate::errors::Result;
use crate::objects::Value;
use crate::vm::interpreter::Interpreter;

use super::helpers::to_f64;

pub(super) fn native_math_abs(
    _interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let n = args.first().map(to_f64).unwrap_or(0.0);
    Ok(Value::Float(n.abs()))
}

pub(super) fn native_math_floor(
    _interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let n = args.first().map(to_f64).unwrap_or(0.0);
    Ok(Value::Float(n.floor()))
}

pub(super) fn native_math_ceil(
    _interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let n = args.first().map(to_f64).unwrap_or(0.0);
    Ok(Value::Float(n.ceil()))
}

pub(super) fn native_math_round(
    _interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let n = args.first().map(to_f64).unwrap_or(0.0);
    Ok(Value::Float(n.round()))
}

pub(super) fn native_math_min(
    _interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let mut result = f64::INFINITY;
    for arg in args {
        let n = to_f64(arg);
        if n < result {
            result = n;
        }
    }
    Ok(Value::Float(result))
}

pub(super) fn native_math_max(
    _interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let mut result = f64::NEG_INFINITY;
    for arg in args {
        let n = to_f64(arg);
        if n > result {
            result = n;
        }
    }
    Ok(Value::Float(result))
}

pub(super) fn native_math_random(
    _interp: &mut Interpreter,
    _this: &Value,
    _args: &[Value],
) -> Result<Value> {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let s = RandomState::new();
    let mut hasher = s.build_hasher();
    hasher.write_u64(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64,
    );
    let bits = hasher.finish();
    Ok(Value::Float((bits as f64) / (u64::MAX as f64)))
}

pub(super) fn native_math_pow(
    _interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let base = args.first().map(to_f64).unwrap_or(0.0);
    let exp = args.get(1).map(to_f64).unwrap_or(0.0);
    Ok(Value::Float(base.powf(exp)))
}

pub(super) fn native_math_sqrt(
    _interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let n = args.first().map(to_f64).unwrap_or(0.0);
    Ok(Value::Float(n.sqrt()))
}

pub(super) fn native_math_log(
    _interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let n = args.first().map(to_f64).unwrap_or(0.0);
    Ok(Value::Float(n.ln()))
}

pub(super) fn native_math_sin(
    _interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let n = args.first().map(to_f64).unwrap_or(0.0);
    Ok(Value::Float(n.sin()))
}

pub(super) fn native_math_cos(
    _interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let n = args.first().map(to_f64).unwrap_or(0.0);
    Ok(Value::Float(n.cos()))
}

pub(super) fn native_math_tan(
    _interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let n = args.first().map(to_f64).unwrap_or(0.0);
    Ok(Value::Float(n.tan()))
}
