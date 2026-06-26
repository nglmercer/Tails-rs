use crate::errors::{Error, Result};
use crate::objects::Value;
use crate::vm::interpreter::Interpreter;

pub(super) fn native_proxy_constructor(interp: &mut Interpreter, _this: &Value, args: &[Value]) -> Result<Value> {
    let target = args.first().cloned().unwrap_or(Value::Undefined);
    let handler = args.get(1).cloned().unwrap_or(Value::Undefined);

    if matches!(target, Value::Undefined | Value::Null) {
        return Err(Error::TypeError("Cannot create proxy with a non-object target".into()));
    }
    if matches!(handler, Value::Undefined | Value::Null) {
        return Err(Error::TypeError("Cannot create proxy with a non-object handler".into()));
    }

    let proxy_idx = interp.heap.len();
    interp.heap.push(crate::vm::interpreter::HeapValue::Proxy(
        crate::vm::interpreter::JsProxyData { target, handler },
    ));
    Ok(Value::Proxy(proxy_idx))
}
