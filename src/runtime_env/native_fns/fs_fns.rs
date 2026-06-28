use crate::errors::{Error, Result};
use crate::objects::Value;
use crate::vm::interpreter::{HeapValue, Interpreter, JsObject};

use super::helpers::to_string_value;

pub(super) fn native_fs_read_file_sync(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let path = args
        .first()
        .map(|v| to_string_value(interp, v))
        .unwrap_or_default();

    let content = std::fs::read_to_string(&path).map_err(|_e| {
        Error::RuntimeError(format!(
            "ENOENT: no such file or directory, open '{}'",
            path
        ))
    })?;

    Ok(Value::String(content))
}

pub(super) fn native_fs_write_file_sync(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let path = args
        .first()
        .map(|v| to_string_value(interp, v))
        .unwrap_or_default();
    let content = args
        .get(1)
        .map(|v| to_string_value(interp, v))
        .unwrap_or_default();

    std::fs::write(&path, content)
        .map_err(|_e| Error::RuntimeError(format!("EACCES: permission denied, open '{}'", path)))?;

    Ok(Value::Undefined)
}

pub(super) fn native_fs_exists_sync(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let path = args
        .first()
        .map(|v| to_string_value(interp, v))
        .unwrap_or_default();

    Ok(Value::Boolean(std::path::Path::new(&path).exists()))
}

pub(super) fn native_fs_mkdir_sync(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let path = args
        .first()
        .map(|v| to_string_value(interp, v))
        .unwrap_or_default();

    let recursive = args
        .get(1)
        .and_then(|v| {
            if let Value::Object(idx) = v {
                if let HeapValue::Object(obj) = &interp.heap[*idx] {
                    return obj
                        .properties
                        .get("recursive")
                        .map(|v| matches!(v, Value::Boolean(true)));
                }
            }
            Some(false)
        })
        .unwrap_or(false);

    if recursive {
        std::fs::create_dir_all(&path).map_err(|_e| {
            Error::RuntimeError(format!("EACCES: permission denied, mkdir '{}'", path))
        })?;
    } else {
        std::fs::create_dir(&path).map_err(|_e| {
            Error::RuntimeError(format!("EACCES: permission denied, mkdir '{}'", path))
        })?;
    }

    Ok(Value::Undefined)
}

pub(super) fn native_fs_readdir_sync(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let path = args
        .first()
        .map(|v| to_string_value(interp, v))
        .unwrap_or_default();

    let entries: Vec<Value> = std::fs::read_dir(&path)
        .map_err(|_e| {
            Error::RuntimeError(format!(
                "ENOENT: no such file or directory, scandir '{}'",
                path
            ))
        })?
        .filter_map(|entry| entry.ok())
        .map(|entry| {
            let name = entry.file_name().to_string_lossy().to_string();
            Value::String(name)
        })
        .collect();

    let arr_idx = interp.heap.len();
    interp
        .heap
        .push(HeapValue::Array(crate::vm::interpreter::JsArray {
            elements: entries,
        }));
    Ok(Value::Array(arr_idx))
}

pub(super) fn native_fs_stat_sync(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let path = args
        .first()
        .map(|v| to_string_value(interp, v))
        .unwrap_or_default();

    let metadata = std::fs::metadata(&path).map_err(|_e| {
        Error::RuntimeError(format!(
            "ENOENT: no such file or directory, stat '{}'",
            path
        ))
    })?;

    let mut props = std::collections::HashMap::new();
    props.insert("size".into(), Value::Integer(metadata.len() as i64));
    props.insert("isFile".into(), Value::Boolean(metadata.is_file()));
    props.insert("isDirectory".into(), Value::Boolean(metadata.is_dir()));
    props.insert(
        "isSymbolicLink".into(),
        Value::Boolean(metadata.file_type().is_symlink()),
    );

    // Mode (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        props.insert("mode".into(), Value::Integer(metadata.mode() as i64));
    }

    // Modified time
    if let Ok(modified) = metadata.modified() {
        if let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH) {
            props.insert(
                "mtimeMs".into(),
                Value::Float(duration.as_secs_f64() * 1000.0),
            );
        }
    }

    // Created time (platform specific)
    #[cfg(unix)]
    if let Ok(created) = metadata.created() {
        if let Ok(duration) = created.duration_since(std::time::UNIX_EPOCH) {
            props.insert(
                "birthtimeMs".into(),
                Value::Float(duration.as_secs_f64() * 1000.0),
            );
        }
    }

    let stat_idx = interp.heap.len();
    interp.heap.push(HeapValue::Object(JsObject {
        properties: props,
        prototype: None,
        extensible: true,
    }));
    Ok(Value::Object(stat_idx))
}

pub(super) fn native_fs_unlink_sync(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let path = args
        .first()
        .map(|v| to_string_value(interp, v))
        .unwrap_or_default();

    std::fs::remove_file(&path).map_err(|_e| {
        Error::RuntimeError(format!(
            "ENOENT: no such file or directory, unlink '{}'",
            path
        ))
    })?;

    Ok(Value::Undefined)
}

pub(super) fn native_fs_rm_sync(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let path = args
        .first()
        .map(|v| to_string_value(interp, v))
        .unwrap_or_default();

    let recursive = args
        .get(1)
        .and_then(|v| {
            if let Value::Object(idx) = v {
                if let HeapValue::Object(obj) = &interp.heap[*idx] {
                    return obj
                        .properties
                        .get("recursive")
                        .map(|v| matches!(v, Value::Boolean(true)));
                }
            }
            Some(false)
        })
        .unwrap_or(false);

    if recursive {
        std::fs::remove_dir_all(&path).map_err(|_e| {
            Error::RuntimeError(format!("ENOENT: no such file or directory, rm '{}'", path))
        })?;
    } else {
        std::fs::remove_file(&path).map_err(|_e| {
            Error::RuntimeError(format!("ENOENT: no such file or directory, rm '{}'", path))
        })?;
    }

    Ok(Value::Undefined)
}

pub(super) fn native_fs_copy_file_sync(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let src = args
        .first()
        .map(|v| to_string_value(interp, v))
        .unwrap_or_default();
    let dest = args
        .get(1)
        .map(|v| to_string_value(interp, v))
        .unwrap_or_default();

    std::fs::copy(&src, &dest).map_err(|_e| {
        Error::RuntimeError(format!(
            "EACCES: permission denied, copy '{}' to '{}'",
            src, dest
        ))
    })?;

    Ok(Value::Undefined)
}

pub(super) fn native_fs_rename_sync(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    let old_path = args
        .first()
        .map(|v| to_string_value(interp, v))
        .unwrap_or_default();
    let new_path = args
        .get(1)
        .map(|v| to_string_value(interp, v))
        .unwrap_or_default();

    std::fs::rename(&old_path, &new_path).map_err(|_e| {
        Error::RuntimeError(format!(
            "EACCES: permission denied, rename '{}' to '{}'",
            old_path, new_path
        ))
    })?;

    Ok(Value::Undefined)
}

pub(super) fn native_fs_append_file_sync(
    interp: &mut Interpreter,
    _this: &Value,
    args: &[Value],
) -> Result<Value> {
    use std::io::Write;

    let path = args
        .first()
        .map(|v| to_string_value(interp, v))
        .unwrap_or_default();
    let content = args
        .get(1)
        .map(|v| to_string_value(interp, v))
        .unwrap_or_default();

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|_e| Error::RuntimeError(format!("EACCES: permission denied, open '{}'", path)))?;

    file.write_all(content.as_bytes()).map_err(|_e| {
        Error::RuntimeError(format!("EACCES: permission denied, write '{}'", path))
    })?;

    Ok(Value::Undefined)
}
