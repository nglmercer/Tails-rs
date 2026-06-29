use crate::errors::Result;
use crate::objects::Value;
use crate::vm::interpreter::Interpreter;

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
    match tails_process::cwd() {
        Ok(path) => Ok(Value::String(path)),
        Err(e) => Err(crate::errors::Error::RuntimeError(format!(
            "cwd failed: {}",
            e
        ))),
    }
}

pub(super) fn native_process_chdir(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let dir = args
        .first()
        .map(|v| to_string_value(interp, v))
        .unwrap_or_default();
    match tails_process::chdir(&dir) {
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
    let _ = tails_process::stdout_write(&data);
    Ok(Value::Boolean(true))
}

pub(super) fn native_process_hrtime(
    interp: &mut Interpreter,
    _this: &Value,
    _args: &[Value],
) -> Result<Value> {
    let (secs, nanos) = tails_process::hrtime();
    let arr_idx = interp.heap.len();
    interp.heap.push(crate::vm::interpreter::HeapValue::Array(
        crate::vm::interpreter::JsArray {
            elements: vec![Value::Integer(secs as i64), Value::Integer(nanos as i64)],
        },
    ));
    Ok(Value::Array(arr_idx))
}

pub(super) fn native_process_hrtime_bigint(
    _interp: &mut Interpreter,
    _this: &Value,
    _args: &[Value],
) -> Result<Value> {
    Ok(Value::BigInt(tails_process::hrtime_bigint() as i128))
}

pub(super) fn native_process_next_tick(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    if let Some(callback) = args.first() {
        let _ = interp.call_value(callback, &Value::Undefined, &[]);
    }
    Ok(Value::Undefined)
}
