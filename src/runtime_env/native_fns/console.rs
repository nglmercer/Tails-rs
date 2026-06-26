use crate::errors::Result;
use crate::objects::Value;
use crate::vm::interpreter::Interpreter;

use super::helpers::to_display_string;

pub(super) fn native_console_log(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let parts: Vec<String> = args.iter().map(|a| to_display_string(interp, a)).collect();
    println!("{}", parts.join(" "));
    Ok(Value::Undefined)
}

pub(super) fn native_console_warn(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let parts: Vec<String> = args.iter().map(|a| to_display_string(interp, a)).collect();
    eprintln!("{}", parts.join(" "));
    Ok(Value::Undefined)
}

pub(super) fn native_console_error(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let parts: Vec<String> = args.iter().map(|a| to_display_string(interp, a)).collect();
    eprintln!("{}", parts.join(" "));
    Ok(Value::Undefined)
}

pub(super) fn native_console_info(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let parts: Vec<String> = args.iter().map(|a| to_display_string(interp, a)).collect();
    println!("{}", parts.join(" "));
    Ok(Value::Undefined)
}
