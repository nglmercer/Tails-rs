use crate::errors::{Error, Result};
use crate::objects::Value;
use crate::vm::interpreter::Interpreter;

pub(super) fn native_bigint_constructor(
    _interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let value = args.first().cloned().unwrap_or(Value::Undefined);
    match value {
        Value::BigInt(_) => Ok(value),
        Value::Integer(n) => Ok(Value::BigInt(n as i128)),
        Value::Float(n) => Ok(Value::BigInt(n as i128)),
        Value::String(s) => {
            let parsed: i128 = s
                .parse()
                .map_err(|_| Error::TypeError(format!("Cannot convert '{}' to BigInt", s)))?;
            Ok(Value::BigInt(parsed))
        }
        Value::Boolean(b) => Ok(Value::BigInt(if b { 1 } else { 0 })),
        _ => Err(Error::TypeError("Cannot convert value to BigInt".into())),
    }
}
