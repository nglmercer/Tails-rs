use crate::errors::{Error, Result};
use crate::objects::Value;
use crate::vm::interpreter::Interpreter;

use super::helpers::{is_truthy, to_string_value};

pub(super) fn native_assert(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let condition = args.first().cloned().unwrap_or(Value::Undefined);

    if !is_truthy(&condition) {
        let message = args
            .get(1)
            .map(|v| to_string_value(interp, v))
            .unwrap_or_else(|| "Assertion failed".to_string());
        return Err(Error::RuntimeError(message));
    }

    Ok(Value::Undefined)
}

pub(super) fn native_assert_strict_equal(
    _interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let actual = args.first().cloned().unwrap_or(Value::Undefined);
    let expected = args.get(1).cloned().unwrap_or(Value::Undefined);

    if actual == expected {
        Ok(Value::Undefined)
    } else {
        let message = format!(
            "Values are not strictly equal. Expected: {:?}, Actual: {:?}",
            expected, actual
        );
        Err(Error::RuntimeError(message))
    }
}
