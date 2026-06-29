use crate::errors::Result;
use crate::objects::Value;
use crate::vm::interpreter::Interpreter;

use super::helpers::to_display_string;
use colored::*;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static USE_COLORS: std::cell::Cell<bool> = const { std::cell::Cell::new(true) };
    static USE_TIMESTAMPS: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static GROUP_DEPTH: std::cell::Cell<i32> = const { std::cell::Cell::new(0) };
    static TIMERS: RefCell<HashMap<String, u128>> = RefCell::new(HashMap::new());
}

pub fn set_colors(enabled: bool) {
    USE_COLORS.with(|c| c.set(enabled));
}

pub fn set_timestamps(enabled: bool) {
    USE_TIMESTAMPS.with(|c| c.set(enabled));
}

pub fn get_use_colors() -> bool {
    USE_COLORS.with(|c| c.get())
}

fn get_indent() -> String {
    GROUP_DEPTH.with(|d| "  ".repeat(d.get() as usize))
}

fn get_timestamp() -> String {
    USE_TIMESTAMPS.with(|ts| {
        if ts.get() {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default();
            let secs = now.as_secs();
            let hours = (secs / 3600) % 24;
            let minutes = (secs / 60) % 60;
            let seconds = secs % 60;
            format!("[{:02}:{:02}:{:02}] ", hours, minutes, seconds)
        } else {
            String::new()
        }
    })
}

fn colorize_value(interp: &Interpreter, v: &Value) -> String {
    let use_colors = get_use_colors();
    let raw = to_display_string(interp, v);

    if !use_colors {
        return raw;
    }

    match v {
        Value::String(_) => raw.green().to_string(),
        Value::Integer(_) | Value::Float(_) => raw.magenta().to_string(),
        Value::Boolean(_) => raw.yellow().to_string(),
        Value::Null => raw.red().bold().to_string(),
        Value::Undefined => raw.dimmed().to_string(),
        Value::Function(_) => raw.cyan().to_string(),
        Value::NativeFunction(_) => raw.cyan().to_string(),
        Value::Object(_) => {
            if raw.starts_with('{') {
                raw.white().to_string()
            } else {
                raw
            }
        }
        Value::Array(_) => raw.yellow().to_string(),
        Value::Date(_) => raw.blue().to_string(),
        Value::RegExp(_) => raw.red().to_string(),
        _ => raw,
    }
}

pub(super) fn native_console_log(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let indent = get_indent();
    let timestamp = get_timestamp();
    let parts: Vec<String> = args.iter().map(|a| colorize_value(interp, a)).collect();
    println!("{}{}{}", timestamp, indent, parts.join(" "));
    Ok(Value::Undefined)
}

pub(super) fn native_console_warn(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let indent = get_indent();
    let timestamp = get_timestamp();
    let use_colors = get_use_colors();
    let parts: Vec<String> = args.iter().map(|a| colorize_value(interp, a)).collect();
    let msg = parts.join(" ");
    if use_colors {
        eprintln!("{}{}{}", timestamp, indent, msg.yellow());
    } else {
        eprintln!("{}{}{}", timestamp, indent, msg);
    }
    Ok(Value::Undefined)
}

pub(super) fn native_console_error(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let indent = get_indent();
    let timestamp = get_timestamp();
    let use_colors = get_use_colors();
    let parts: Vec<String> = args.iter().map(|a| colorize_value(interp, a)).collect();
    let msg = parts.join(" ");
    if use_colors {
        eprintln!("{}{}{}", timestamp, indent, msg.red());
    } else {
        eprintln!("{}{}{}", timestamp, indent, msg);
    }
    Ok(Value::Undefined)
}

pub(super) fn native_console_info(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let indent = get_indent();
    let timestamp = get_timestamp();
    let use_colors = get_use_colors();
    let parts: Vec<String> = args.iter().map(|a| colorize_value(interp, a)).collect();
    let msg = parts.join(" ");
    if use_colors {
        println!("{}{}{}", timestamp, indent, msg.blue());
    } else {
        println!("{}{}{}", timestamp, indent, msg);
    }
    Ok(Value::Undefined)
}

pub(super) fn native_console_table(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    if args.is_empty() {
        println!("[]");
        return Ok(Value::Undefined);
    }

    let indent = get_indent();
    let timestamp = get_timestamp();
    let use_colors = get_use_colors();

    match &args[0] {
        Value::Array(arr_idx) => {
            if let crate::vm::interpreter::HeapValue::Array(arr) = &interp.heap[*arr_idx] {
                if arr.elements.is_empty() {
                    println!("{}{}(empty table)", timestamp, indent);
                    return Ok(Value::Undefined);
                }

                let mut all_keys: Vec<String> = Vec::new();
                for elem in &arr.elements {
                    if let Value::Object(obj_idx) = elem {
                        if let crate::vm::interpreter::HeapValue::Object(obj) =
                            &interp.heap[*obj_idx]
                        {
                            for key in obj.properties.keys() {
                                if !all_keys.contains(key) {
                                    all_keys.push(key.clone());
                                }
                            }
                        }
                    }
                }
                all_keys.sort();

                if all_keys.is_empty() {
                    let parts: Vec<String> = arr
                        .elements
                        .iter()
                        .enumerate()
                        .map(|(i, e)| format!("{}: {}", i, colorize_value(interp, e)))
                        .collect();
                    println!("{}{}[{}]", timestamp, indent, parts.join(", "));
                    return Ok(Value::Undefined);
                }

                let index_width = format!("{}", arr.elements.len() - 1).len().max(5);
                let mut col_widths: Vec<usize> = all_keys.iter().map(|k| k.len()).collect();

                for elem in &arr.elements {
                    if let Value::Object(obj_idx) = elem {
                        if let crate::vm::interpreter::HeapValue::Object(obj) =
                            &interp.heap[*obj_idx]
                        {
                            for (i, key) in all_keys.iter().enumerate() {
                                if let Some(val) = obj.properties.get(key) {
                                    let val_str = to_display_string(interp, val);
                                    col_widths[i] = col_widths[i].max(val_str.len());
                                }
                            }
                        }
                    }
                }

                let header_idx = "(index)".to_string();
                let mut header = format!("{:width$}", header_idx, width = index_width);
                for key in &all_keys {
                    if use_colors {
                        header.push_str(&format!(" | {}", key.bold()));
                    } else {
                        header.push_str(&format!(" | {}", key));
                    }
                }
                let separator = "-".repeat(header.len());
                println!("{}{}{}", timestamp, indent, header);
                println!("{}{}{}", timestamp, indent, separator);

                for (row_idx, elem) in arr.elements.iter().enumerate() {
                    let mut row = format!("{:width$}", row_idx, width = index_width);
                    if let Value::Object(obj_idx) = elem {
                        if let crate::vm::interpreter::HeapValue::Object(obj) =
                            &interp.heap[*obj_idx]
                        {
                            for (i, key) in all_keys.iter().enumerate() {
                                let val_str = if let Some(val) = obj.properties.get(key) {
                                    colorize_value(interp, val)
                                } else {
                                    "undefined".to_string()
                                };
                                row.push_str(&format!(
                                    " | {:width$}",
                                    val_str,
                                    width = col_widths[i]
                                ));
                            }
                        }
                    } else {
                        let val_str = colorize_value(interp, elem);
                        row.push_str(&format!(" | {}", val_str));
                    }
                    println!("{}{}{}", timestamp, indent, row);
                }
            }
        }
        Value::Object(obj_idx) => {
            if let crate::vm::interpreter::HeapValue::Object(obj) = &interp.heap[*obj_idx] {
                let mut props: Vec<(&String, &Value)> = obj.properties.iter().collect();
                props.sort_by(|a, b| a.0.cmp(b.0));

                if props.is_empty() {
                    println!("{}{{}}", timestamp);
                    return Ok(Value::Undefined);
                }

                let key_width = props.iter().map(|(k, _)| k.len()).max().unwrap_or(5);
                let val_width = props
                    .iter()
                    .map(|(_, v)| to_display_string(interp, v).len())
                    .max()
                    .unwrap_or(5);

                let header = if use_colors {
                    format!(
                        "{:width_key$} | {:width_val$}",
                        "Key".bold(),
                        "Value".bold(),
                        width_key = key_width,
                        width_val = val_width
                    )
                } else {
                    format!(
                        "{:width_key$} | {:width_val$}",
                        "Key",
                        "Value",
                        width_key = key_width,
                        width_val = val_width
                    )
                };
                let separator = "-".repeat(header.len());
                println!("{}{}{}", timestamp, indent, header);
                println!("{}{}{}", timestamp, indent, separator);

                for (key, val) in &props {
                    let val_str = colorize_value(interp, val);
                    println!(
                        "{}{}{:width_key$} | {:width_val$}",
                        timestamp,
                        indent,
                        key,
                        val_str,
                        width_key = key_width,
                        width_val = val_width
                    );
                }
            }
        }
        _ => {
            let parts: Vec<String> = args.iter().map(|a| colorize_value(interp, a)).collect();
            println!("{}{}{}", timestamp, indent, parts.join(" "));
        }
    }

    Ok(Value::Undefined)
}

pub(super) fn native_console_dir(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    if args.is_empty() {
        return Ok(Value::Undefined);
    }

    let indent = get_indent();
    let timestamp = get_timestamp();
    let use_colors = get_use_colors();
    let max_depth = args.get(1).and_then(|d| {
        if let Value::Integer(n) = d {
            Some(*n as usize)
        } else {
            None
        }
    });

    fn format_deep(
        interp: &Interpreter,
        v: &Value,
        current_depth: usize,
        max_depth: usize,
        indent_level: usize,
        use_colors: bool,
    ) -> String {
        let pad = "  ".repeat(indent_level);

        match v {
            Value::Object(obj_idx) => {
                if let crate::vm::interpreter::HeapValue::Object(obj) = &interp.heap[*obj_idx] {
                    if obj.properties.is_empty() {
                        return "{}".to_string();
                    }
                    if current_depth >= max_depth {
                        return "[Object]".to_string();
                    }

                    let mut props: Vec<(&String, &Value)> = obj.properties.iter().collect();
                    props.sort_by(|a, b| a.0.cmp(b.0));

                    let mut result = "{\n".to_string();
                    for (i, (key, val)) in props.iter().enumerate() {
                        let val_str = format_deep(
                            interp,
                            val,
                            current_depth + 1,
                            max_depth,
                            indent_level + 1,
                            use_colors,
                        );
                        let comma = if i < props.len() - 1 { "," } else { "" };
                        if use_colors {
                            result.push_str(&format!(
                                "{}  {}: {}{}\n",
                                pad,
                                key.bold(),
                                val_str,
                                comma
                            ));
                        } else {
                            result.push_str(&format!("{}  {}: {}{}\n", pad, key, val_str, comma));
                        }
                    }
                    result.push_str(&format!("{} }}", pad));
                    result
                } else {
                    "[Object]".to_string()
                }
            }
            Value::Array(arr_idx) => {
                if let crate::vm::interpreter::HeapValue::Array(arr) = &interp.heap[*arr_idx] {
                    if arr.elements.is_empty() {
                        return "[]".to_string();
                    }
                    if current_depth >= max_depth {
                        return "[Array]".to_string();
                    }

                    let mut result = "[\n".to_string();
                    for (i, elem) in arr.elements.iter().enumerate() {
                        let val_str = format_deep(
                            interp,
                            elem,
                            current_depth + 1,
                            max_depth,
                            indent_level + 1,
                            use_colors,
                        );
                        let comma = if i < arr.elements.len() - 1 { "," } else { "" };
                        result.push_str(&format!("{}  {}{}\n", pad, val_str, comma));
                    }
                    result.push_str(&format!("{} ]", pad));
                    result
                } else {
                    "[Array]".to_string()
                }
            }
            Value::Function(idx) => {
                if let crate::vm::interpreter::HeapValue::Function(f) = &interp.heap[*idx] {
                    let name = f.name.as_deref().unwrap_or("anonymous");
                    if f.prototype.is_some() && f.super_class.is_some() {
                        if use_colors {
                            format!("[class {}]", name.cyan())
                        } else {
                            format!("[class {}]", name)
                        }
                    } else if use_colors {
                        format!("[Function: {}]", name.cyan())
                    } else {
                        format!("[Function: {}]", name)
                    }
                } else {
                    "[Function]".to_string()
                }
            }
            Value::NativeFunction(_) => {
                if use_colors {
                    "[NativeFunction]".cyan().to_string()
                } else {
                    "[NativeFunction]".to_string()
                }
            }
            Value::String(s) => {
                if use_colors {
                    format!("\"{}\"", s.green())
                } else {
                    format!("\"{}\"", s)
                }
            }
            Value::Integer(n) => {
                if use_colors {
                    n.to_string().magenta().to_string()
                } else {
                    n.to_string()
                }
            }
            Value::Float(n) => {
                let val = if *n == (*n as i64) as f64 {
                    (*n as i64).to_string()
                } else {
                    n.to_string()
                };
                if use_colors {
                    val.magenta().to_string()
                } else {
                    val
                }
            }
            Value::Boolean(b) => {
                let val = b.to_string();
                if use_colors {
                    val.yellow().to_string()
                } else {
                    val
                }
            }
            Value::Null => {
                if use_colors {
                    "null".red().bold().to_string()
                } else {
                    "null".to_string()
                }
            }
            Value::Undefined => {
                if use_colors {
                    "undefined".dimmed().to_string()
                } else {
                    "undefined".to_string()
                }
            }
            Value::Map(idx) => {
                if let crate::vm::interpreter::HeapValue::Map(map) = &interp.heap[*idx] {
                    format!("Map({})", map.entries.len())
                } else {
                    "Map".to_string()
                }
            }
            Value::Set(idx) => {
                if let crate::vm::interpreter::HeapValue::Set(set) = &interp.heap[*idx] {
                    format!("Set({})", set.values.len())
                } else {
                    "Set".to_string()
                }
            }
            Value::Date(idx) => {
                if let crate::vm::interpreter::HeapValue::Date(d) = &interp.heap[*idx] {
                    if use_colors {
                        format!("Date({})", d.to_utc_string().blue())
                    } else {
                        format!("Date({})", d.to_utc_string())
                    }
                } else {
                    "Date".to_string()
                }
            }
            Value::RegExp(idx) => {
                if let crate::vm::interpreter::HeapValue::RegExp(r) = &interp.heap[*idx] {
                    if use_colors {
                        format!("/{}/{}", r.source.red(), r.flags)
                    } else {
                        format!("/{}/{}", r.source, r.flags)
                    }
                } else {
                    "RegExp".to_string()
                }
            }
            _ => to_display_string(interp, v),
        }
    }

    let max_depth = max_depth.unwrap_or(2);
    let formatted = format_deep(interp, &args[0], 0, max_depth, 0, use_colors);
    println!("{}{}{}", timestamp, indent, formatted);

    Ok(Value::Undefined)
}

pub(super) fn native_console_group(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let indent = get_indent();
    let timestamp = get_timestamp();
    let parts: Vec<String> = args.iter().map(|a| colorize_value(interp, a)).collect();

    if !parts.is_empty() {
        if get_use_colors() {
            println!("{}{}{}", timestamp, indent, parts.join(" ").bold());
        } else {
            println!("{}{}{}", timestamp, indent, parts.join(" "));
        }
    }

    GROUP_DEPTH.with(|d| d.set(d.get() + 1));

    Ok(Value::Undefined)
}

pub(super) fn native_console_group_end(
    _interp: &mut Interpreter,
    _this: &Value,
    _args: &[Value],
) -> Result<Value> {
    GROUP_DEPTH.with(|d| {
        let val = d.get();
        if val > 0 {
            d.set(val - 1);
        }
    });
    Ok(Value::Undefined)
}

pub(super) fn native_console_group_collapsed(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let indent = get_indent();
    let timestamp = get_timestamp();
    let parts: Vec<String> = args.iter().map(|a| colorize_value(interp, a)).collect();

    if !parts.is_empty() {
        if get_use_colors() {
            println!("{}{}▶ {}", timestamp, indent, parts.join(" ").bold());
        } else {
            println!("{}{}▶ {}", timestamp, indent, parts.join(" "));
        }
    }

    GROUP_DEPTH.with(|d| d.set(d.get() + 1));

    Ok(Value::Undefined)
}

pub(super) fn native_console_time(
    _interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let label = if args.is_empty() {
        "default".to_string()
    } else {
        to_display_string(_interp, &args[0])
    };

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    TIMERS.with(|t| {
        t.borrow_mut().insert(label, now);
    });

    Ok(Value::Undefined)
}

pub(super) fn native_console_time_end(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let label = if args.is_empty() {
        "default".to_string()
    } else {
        to_display_string(interp, &args[0])
    };

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    let elapsed = TIMERS.with(|t| t.borrow().get(&label).map(|start| now - start).unwrap_or(0));

    let indent = get_indent();
    let timestamp = get_timestamp();
    let use_colors = get_use_colors();

    if use_colors {
        println!(
            "{}{}{}: {}ms",
            timestamp,
            indent,
            label.bold(),
            elapsed.to_string().cyan()
        );
    } else {
        println!("{}{}{}: {}ms", timestamp, indent, label, elapsed);
    }

    Ok(Value::Undefined)
}

pub(super) fn native_console_assert(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    if args.is_empty() {
        return Ok(Value::Undefined);
    }

    let condition = match &args[0] {
        Value::Boolean(b) => *b,
        Value::Integer(n) => *n != 0,
        Value::Float(n) => !n.is_nan() && *n != 0.0,
        Value::Null | Value::Undefined => false,
        Value::String(s) => !s.is_empty(),
        _ => true,
    };

    if !condition {
        let indent = get_indent();
        let timestamp = get_timestamp();
        let use_colors = get_use_colors();
        let parts: Vec<String> = if args.len() > 1 {
            args[1..]
                .iter()
                .map(|a| colorize_value(interp, a))
                .collect()
        } else {
            vec!["Assertion failed".to_string()]
        };
        let msg = parts.join(" ");
        if use_colors {
            eprintln!("{}{}{}", timestamp, indent, msg.red());
        } else {
            eprintln!("{}{}{}", timestamp, indent, msg);
        }
    }

    Ok(Value::Undefined)
}

pub(super) fn native_console_clear(
    _interp: &mut Interpreter,
    _this: &Value,
    _args: &[Value],
) -> Result<Value> {
    print!("\x1B[2J\x1B[1;1H");
    Ok(Value::Undefined)
}
