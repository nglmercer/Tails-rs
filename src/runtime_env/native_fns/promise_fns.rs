use crate::errors::Result;
use crate::objects::Value;
use crate::vm::interpreter::Interpreter;

pub(super) fn native_promise_constructor(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let executor = args.first().cloned().unwrap_or(Value::Undefined);

    let promise_idx = interp.heap.len();
    interp.heap.push(crate::vm::interpreter::HeapValue::Promise(
        crate::objects::js_promise::JsPromise::new(),
    ));

    if matches!(executor, Value::Function(_) | Value::NativeFunction(_)) {
        let resolve_fn = interp.create_resolve_fn(promise_idx);
        let reject_fn = interp.create_reject_fn(promise_idx);
        interp.call_value(&executor, &Value::Undefined, &[resolve_fn, reject_fn])?;
    }

    Ok(Value::Promise(promise_idx))
}

pub(super) fn native_promise_then(
    interp: &mut Interpreter,
    this: &Value,
    args: &[Value],
) -> Result<Value> {
    let callback = args.first().cloned().unwrap_or(Value::Undefined);
    let on_fulfilled = if let Value::Promise(idx) = this {
        *idx
    } else {
        return Ok(Value::Undefined);
    };

    let new_promise_idx = interp.heap.len();
    interp.heap.push(crate::vm::interpreter::HeapValue::Promise(
        crate::objects::js_promise::JsPromise::new(),
    ));

    let state_snapshot =
        if let crate::vm::interpreter::HeapValue::Promise(promise) = &interp.heap[on_fulfilled] {
            promise.state.clone()
        } else {
            return Ok(Value::Promise(new_promise_idx));
        };

    match state_snapshot {
        crate::objects::js_promise::PromiseState::Fulfilled(value) => {
            let result = interp.call_value(&callback, &Value::Undefined, &[value])?;
            interp.resolve_promise(new_promise_idx, result);
        }
        crate::objects::js_promise::PromiseState::Rejected(reason) => {
            interp.reject_promise(new_promise_idx, reason);
        }
        crate::objects::js_promise::PromiseState::Pending => {
            if let crate::vm::interpreter::HeapValue::Promise(promise) =
                &mut interp.heap[on_fulfilled]
            {
                let cb_idx = match callback {
                    Value::Function(idx) => idx,
                    _ => 0,
                };
                promise
                    .then_handlers
                    .push(crate::objects::js_promise::PromiseHandler {
                        callback: cb_idx,
                        resolve: true,
                    });
            }
        }
    }

    Ok(Value::Promise(new_promise_idx))
}

pub(super) fn native_promise_catch(
    interp: &mut Interpreter,
    this: &Value,
    args: &[Value],
) -> Result<Value> {
    let callback = args.first().cloned().unwrap_or(Value::Undefined);
    let on_rejected = if let Value::Promise(idx) = this {
        *idx
    } else {
        return Ok(Value::Undefined);
    };

    let new_promise_idx = interp.heap.len();
    interp.heap.push(crate::vm::interpreter::HeapValue::Promise(
        crate::objects::js_promise::JsPromise::new(),
    ));

    let state_snapshot =
        if let crate::vm::interpreter::HeapValue::Promise(promise) = &interp.heap[on_rejected] {
            promise.state.clone()
        } else {
            return Ok(Value::Promise(new_promise_idx));
        };

    match state_snapshot {
        crate::objects::js_promise::PromiseState::Rejected(reason) => {
            let result = interp.call_value(&callback, &Value::Undefined, &[reason])?;
            interp.resolve_promise(new_promise_idx, result);
        }
        crate::objects::js_promise::PromiseState::Fulfilled(value) => {
            interp.resolve_promise(new_promise_idx, value);
        }
        crate::objects::js_promise::PromiseState::Pending => {
            if let crate::vm::interpreter::HeapValue::Promise(promise) =
                &mut interp.heap[on_rejected]
            {
                let cb_idx = match callback {
                    Value::Function(idx) => idx,
                    _ => 0,
                };
                promise
                    .catch_handlers
                    .push(crate::objects::js_promise::PromiseHandler {
                        callback: cb_idx,
                        resolve: false,
                    });
            }
        }
    }

    Ok(Value::Promise(new_promise_idx))
}

pub(super) fn native_promise_finally(
    interp: &mut Interpreter,
    this: &Value,
    args: &[Value],
) -> Result<Value> {
    let callback = args.first().cloned().unwrap_or(Value::Undefined);
    let on_finally = if let Value::Promise(idx) = this {
        *idx
    } else {
        return Ok(Value::Undefined);
    };

    let new_promise_idx = interp.heap.len();
    interp.heap.push(crate::vm::interpreter::HeapValue::Promise(
        crate::objects::js_promise::JsPromise::new(),
    ));

    let state_snapshot =
        if let crate::vm::interpreter::HeapValue::Promise(promise) = &interp.heap[on_finally] {
            promise.state.clone()
        } else {
            return Ok(Value::Promise(new_promise_idx));
        };

    match state_snapshot {
        crate::objects::js_promise::PromiseState::Fulfilled(value)
        | crate::objects::js_promise::PromiseState::Rejected(value) => {
            let _ = interp.call_value(&callback, &Value::Undefined, &[])?;
            interp.resolve_promise(new_promise_idx, value);
        }
        crate::objects::js_promise::PromiseState::Pending => {
            if let crate::vm::interpreter::HeapValue::Promise(promise) =
                &mut interp.heap[on_finally]
            {
                let cb_idx = match callback {
                    Value::Function(idx) => idx,
                    _ => 0,
                };
                promise
                    .finally_handlers
                    .push(crate::objects::js_promise::PromiseHandler {
                        callback: cb_idx,
                        resolve: true,
                    });
            }
        }
    }

    Ok(Value::Promise(new_promise_idx))
}

pub(super) fn native_promise_resolve(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let value = args.first().cloned().unwrap_or(Value::Undefined);

    let promise_idx = interp.heap.len();
    interp.heap.push(crate::vm::interpreter::HeapValue::Promise(
        crate::objects::js_promise::JsPromise::new(),
    ));

    interp.resolve_promise(promise_idx, value);
    Ok(Value::Promise(promise_idx))
}

pub(super) fn native_promise_reject(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let reason = args.first().cloned().unwrap_or(Value::Undefined);

    let promise_idx = interp.heap.len();
    interp.heap.push(crate::vm::interpreter::HeapValue::Promise(
        crate::objects::js_promise::JsPromise::new(),
    ));

    interp.reject_promise(promise_idx, reason);
    Ok(Value::Promise(promise_idx))
}

pub(super) fn native_promise_all(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let promises_arg = args.first().cloned().unwrap_or(Value::Undefined);

    let promise_indices: Vec<usize> = match &promises_arg {
        Value::Array(arr_idx) => {
            if let crate::vm::interpreter::HeapValue::Array(arr) = &interp.heap[*arr_idx] {
                arr.elements
                    .iter()
                    .filter_map(|v| {
                        if let Value::Promise(idx) = v {
                            Some(*idx)
                        } else {
                            None
                        }
                    })
                    .collect()
            } else {
                Vec::new()
            }
        }
        _ => Vec::new(),
    };

    let result_idx = interp.heap.len();
    interp.heap.push(crate::vm::interpreter::HeapValue::Promise(
        crate::objects::js_promise::JsPromise::new(),
    ));

    let mut all_fulfilled = true;
    let mut values = Vec::new();
    let mut first_rejection: Option<Value> = None;

    for &p_idx in &promise_indices {
        if let crate::vm::interpreter::HeapValue::Promise(p) = &interp.heap[p_idx] {
            match &p.state {
                crate::objects::js_promise::PromiseState::Fulfilled(v) => {
                    values.push(v.clone());
                }
                crate::objects::js_promise::PromiseState::Rejected(r) => {
                    if first_rejection.is_none() {
                        first_rejection = Some(r.clone());
                    }
                }
                crate::objects::js_promise::PromiseState::Pending => {
                    all_fulfilled = false;
                }
            }
        }
    }

    if let Some(reason) = first_rejection {
        interp.reject_promise(result_idx, reason);
    } else if all_fulfilled {
        let heap_idx = interp.heap.len();
        interp.heap.push(crate::vm::interpreter::HeapValue::Array(
            crate::vm::interpreter::JsArray { elements: values },
        ));
        interp.resolve_promise(result_idx, Value::Array(heap_idx));
    }

    Ok(Value::Promise(result_idx))
}

pub(super) fn native_promise_race(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let promises_arg = args.first().cloned().unwrap_or(Value::Undefined);

    let promise_indices: Vec<usize> = match &promises_arg {
        Value::Array(arr_idx) => {
            if let crate::vm::interpreter::HeapValue::Array(arr) = &interp.heap[*arr_idx] {
                arr.elements
                    .iter()
                    .filter_map(|v| {
                        if let Value::Promise(idx) = v {
                            Some(*idx)
                        } else {
                            None
                        }
                    })
                    .collect()
            } else {
                Vec::new()
            }
        }
        _ => Vec::new(),
    };

    let result_idx = interp.heap.len();
    interp.heap.push(crate::vm::interpreter::HeapValue::Promise(
        crate::objects::js_promise::JsPromise::new(),
    ));

    for &p_idx in &promise_indices {
        if let crate::vm::interpreter::HeapValue::Promise(p) = &interp.heap[p_idx] {
            match &p.state {
                crate::objects::js_promise::PromiseState::Fulfilled(v) => {
                    interp.resolve_promise(result_idx, v.clone());
                    return Ok(Value::Promise(result_idx));
                }
                crate::objects::js_promise::PromiseState::Rejected(r) => {
                    interp.reject_promise(result_idx, r.clone());
                    return Ok(Value::Promise(result_idx));
                }
                crate::objects::js_promise::PromiseState::Pending => {}
            }
        }
    }

    Ok(Value::Promise(result_idx))
}
