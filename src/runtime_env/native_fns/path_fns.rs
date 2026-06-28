use crate::errors::Result;
use crate::objects::Value;
use crate::vm::interpreter::Interpreter;

use super::helpers::to_string_value;

pub(super) fn native_path_join(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let parts: Vec<String> = args.iter().map(|v| to_string_value(interp, v)).collect();
    let mut result = std::path::PathBuf::new();
    for part in &parts {
        result.push(part);
    }
    Ok(Value::String(result.to_string_lossy().to_string()))
}

pub(super) fn native_path_resolve(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let mut result = std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| ".".to_string());

    for arg in args {
        let s = to_string_value(interp, arg);
        if std::path::Path::new(&s).is_absolute() {
            result = s;
        } else {
            result = std::path::Path::new(&result)
                .join(&s)
                .to_string_lossy()
                .to_string();
        }
    }

    // Normalize the result
    let normalized = normalize_path(&result);
    Ok(Value::String(normalized))
}

pub(super) fn native_path_basename(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let path = args
        .first()
        .map(|v| to_string_value(interp, v))
        .unwrap_or_default();
    let ext = args.get(1).map(|v| to_string_value(interp, v));

    let base = std::path::Path::new(&path)
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();

    if let Some(ext) = ext {
        if !ext.is_empty() && base.ends_with(&ext) {
            let trim_len = base.len() - ext.len();
            return Ok(Value::String(base[..trim_len].to_string()));
        }
    }

    Ok(Value::String(base))
}

pub(super) fn native_path_dirname(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let path = args
        .first()
        .map(|v| to_string_value(interp, v))
        .unwrap_or_default();

    let dir = std::path::Path::new(&path)
        .parent()
        .map(|p| {
            let s = p.to_string_lossy().to_string();
            if s.is_empty() {
                ".".to_string()
            } else {
                s
            }
        })
        .unwrap_or_else(|| ".".to_string());

    Ok(Value::String(dir))
}

pub(super) fn native_path_extname(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let path = args
        .first()
        .map(|v| to_string_value(interp, v))
        .unwrap_or_default();

    let ext = std::path::Path::new(&path)
        .extension()
        .map(|s| {
            let e = s.to_string_lossy().to_string();
            format!(".{}", e)
        })
        .unwrap_or_default();

    Ok(Value::String(ext))
}

pub(super) fn native_path_relative(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let from = args
        .first()
        .map(|v| to_string_value(interp, v))
        .unwrap_or_else(|| ".".to_string());
    let to = args
        .get(1)
        .map(|v| to_string_value(interp, v))
        .unwrap_or_default();

    let from_path = std::path::Path::new(&from)
        .canonicalize()
        .unwrap_or_else(|_| std::path::PathBuf::from(&from));
    let to_path = std::path::Path::new(&to)
        .canonicalize()
        .unwrap_or_else(|_| std::path::PathBuf::from(&to));

    let from_components: Vec<_> = from_path.components().collect();
    let to_components: Vec<_> = to_path.components().collect();

    // Find common prefix
    let mut common_len = 0;
    for (a, b) in from_components.iter().zip(to_components.iter()) {
        if a == b {
            common_len += 1;
        } else {
            break;
        }
    }

    // Build relative path
    let mut parts: Vec<String> = Vec::new();
    // Go up from 'from' to common ancestor
    for _ in common_len..from_components.len() {
        parts.push("..".to_string());
    }
    // Go down from common ancestor to 'to'
    for comp in &to_components[common_len..] {
        let s = comp.as_os_str().to_string_lossy().to_string();
        parts.push(s);
    }

    let result = if parts.is_empty() {
        ".".to_string()
    } else {
        parts.join(std::path::MAIN_SEPARATOR_STR)
    };

    Ok(Value::String(result))
}

pub(super) fn native_path_is_absolute(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let path = args
        .first()
        .map(|v| to_string_value(interp, v))
        .unwrap_or_default();
    Ok(Value::Boolean(std::path::Path::new(&path).is_absolute()))
}

pub(super) fn native_path_normalize(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let path = args
        .first()
        .map(|v| to_string_value(interp, v))
        .unwrap_or_default();
    Ok(Value::String(normalize_path(&path)))
}

fn normalize_path(path: &str) -> String {
    let mut components = Vec::new();
    let mut result = String::new();

    for component in std::path::Path::new(path).components() {
        match component {
            std::path::Component::Normal(c) => {
                let s = c.to_string_lossy().to_string();
                if s == ".." {
                    components.pop();
                } else if s != "." {
                    components.push(s);
                }
            }
            std::path::Component::ParentDir => {
                components.pop();
            }
            std::path::Component::CurDir => {}
            std::path::Component::RootDir => {
                result.push('/');
            }
            std::path::Component::Prefix(prefix) => {
                result.push_str(&prefix.as_os_str().to_string_lossy());
            }
        }
    }

    if result.is_empty() && !path.starts_with('/') {
        result = components.join("/");
        if result.is_empty() {
            result = ".".to_string();
        }
    } else {
        result.push_str(&components.join("/"));
    }

    result
}
