use crate::errors::Result;
use crate::objects::Value;
use crate::vm::interpreter::{HeapValue, Interpreter};

use super::helpers::to_string_value;

pub(super) fn native_process_exit(
    _interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let code = match args.first() {
        Some(Value::Integer(n)) => *n as i32,
        Some(Value::Float(n)) => *n as i32,
        _ => 0,
    };
    std::process::exit(code);
}

pub(super) fn native_process_cwd(
    _interp: &mut Interpreter,
    _this: &Value,
    _args: &[Value],
) -> Result<Value> {
    match std::env::current_dir() {
        Ok(path) => Ok(Value::String(path.to_string_lossy().to_string())),
        Err(e) => Err(crate::errors::Error::RuntimeError(format!(
            "cwd failed: {}",
            e
        ))),
    }
}

pub(super) fn native_process_chdir(
    _interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let dir = args
        .first()
        .map(|v| to_string_value(_interp, v))
        .unwrap_or_default();
    match std::env::set_current_dir(&dir) {
        Ok(()) => Ok(Value::Undefined),
        Err(e) => Err(crate::errors::Error::RuntimeError(format!(
            "chdir failed: {}",
            e
        ))),
    }
}

pub(super) fn native_process_stdout_write(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let data = args
        .first()
        .map(|v| to_string_value(interp, v))
        .unwrap_or_default();
    use std::io::Write;
    let _ = std::io::stdout().write_all(data.as_bytes());
    Ok(Value::Boolean(true))
}

pub(super) fn native_process_hrtime(
    _interp: &mut Interpreter,
    _this: &Value,
    _args: &[Value],
) -> Result<Value> {
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = dur.as_secs() as i64;
    let nanos = dur.subsec_nanos() as i64;

    // Return [seconds, nanoseconds] as an array
    let arr_idx = _interp.heap.len();
    _interp.heap.push(HeapValue::Array(
        crate::vm::interpreter::JsArray {
            elements: vec![Value::Integer(secs), Value::Integer(nanos)],
        },
    ));
    Ok(Value::Array(arr_idx))
}

pub(super) fn native_process_hrtime_bigint(
    _interp: &mut Interpreter,
    _this: &Value,
    _args: &[Value],
) -> Result<Value> {
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let nanos = dur.as_nanos() as i128;
    Ok(Value::BigInt(nanos))
}

pub(super) fn native_process_next_tick(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    // Simplified: just execute the callback immediately
    if let Some(callback) = args.first() {
        let _ = interp.call_value(callback, &Value::Undefined, &[]);
    }
    Ok(Value::Undefined)
}
