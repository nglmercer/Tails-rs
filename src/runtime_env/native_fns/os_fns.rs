use crate::errors::Result;
use crate::objects::Value;
use crate::vm::interpreter::{HeapValue, Interpreter, JsObject};

pub(super) fn native_os_platform(
    _interp: &mut Interpreter,
    _this: &Value,
    _args: &[Value],
) -> Result<Value> {
    let platform = if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "macos") {
        "darwin"
    } else if cfg!(target_os = "windows") {
        "win32"
    } else {
        "unknown"
    };
    Ok(Value::String(platform.to_string()))
}

pub(super) fn native_os_arch(
    _interp: &mut Interpreter,
    _this: &Value,
    _args: &[Value],
) -> Result<Value> {
    let arch = if cfg!(target_arch = "x86_64") {
        "x64"
    } else if cfg!(target_arch = "aarch64") {
        "arm64"
    } else if cfg!(target_arch = "x86") {
        "x86"
    } else {
        "unknown"
    };
    Ok(Value::String(arch.to_string()))
}

pub(super) fn native_os_cpus(
    interp: &mut Interpreter,
    _this: &Value,
    _args: &[Value],
) -> Result<Value> {
    let count = std::thread::available_parallelism()
        .map(|n| n.get() as i64)
        .unwrap_or(1);

    let mut cpus = Vec::new();
    for _i in 0..count {
        let mut props = std::collections::HashMap::new();
        props.insert("model".into(), Value::String("CPU".into()));
        props.insert("speed".into(), Value::Float(0.0));
        let mut times_props = std::collections::HashMap::new();
        times_props.insert("user".into(), Value::Integer(0));
        times_props.insert("nice".into(), Value::Integer(0));
        times_props.insert("sys".into(), Value::Integer(0));
        times_props.insert("idle".into(), Value::Integer(0));
        times_props.insert("irq".into(), Value::Integer(0));
        let times_idx = interp.gc.allocate(
            &mut interp.heap,
            HeapValue::Object(JsObject {
                properties: times_props,
                prototype: None,
                extensible: true,
            }),
        );
        props.insert("times".into(), Value::Object(times_idx));
        let cpu_idx = interp.heap.len();
        interp.heap.push(HeapValue::Object(JsObject {
            properties: props,
            prototype: None,
            extensible: true,
        }));
        cpus.push(Value::Object(cpu_idx));
    }

    let arr_idx = interp.heap.len();
    interp
        .heap
        .push(HeapValue::Array(crate::vm::interpreter::JsArray {
            elements: cpus,
        }));
    Ok(Value::Array(arr_idx))
}

pub(super) fn native_os_totalmem(
    _interp: &mut Interpreter,
    _this: &Value,
    _args: &[Value],
) -> Result<Value> {
    // Read from /proc/meminfo on Linux
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
            for line in content.lines() {
                if let Some(rest) = line.strip_prefix("MemTotal:") {
                    if let Some(kb_str) = rest.split_whitespace().next() {
                        if let Ok(kb) = kb_str.parse::<f64>() {
                            return Ok(Value::Float(kb * 1024.0));
                        }
                    }
                }
            }
        }
    }
    Ok(Value::Float(16.0 * 1024.0 * 1024.0 * 1024.0))
}

pub(super) fn native_os_freemem(
    _interp: &mut Interpreter,
    _this: &Value,
    _args: &[Value],
) -> Result<Value> {
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
            for line in content.lines() {
                if let Some(rest) = line.strip_prefix("MemAvailable:") {
                    if let Some(kb_str) = rest.split_whitespace().next() {
                        if let Ok(kb) = kb_str.parse::<f64>() {
                            return Ok(Value::Float(kb * 1024.0));
                        }
                    }
                }
            }
        }
    }
    Ok(Value::Float(8.0 * 1024.0 * 1024.0 * 1024.0))
}

pub(super) fn native_os_uptime(
    _interp: &mut Interpreter,
    _this: &Value,
    _args: &[Value],
) -> Result<Value> {
    let uptime = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64();
    Ok(Value::Float(uptime))
}

pub(super) fn native_os_hostname(
    _interp: &mut Interpreter,
    _this: &Value,
    _args: &[Value],
) -> Result<Value> {
    if let Ok(hostname) = std::env::var("HOSTNAME") {
        if !hostname.is_empty() {
            return Ok(Value::String(hostname));
        }
    }
    #[cfg(unix)]
    {
        let mut buf = [0u8; 256];
        if unsafe { libc::gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf.len()) } == 0 {
            if let Ok(s) = std::ffi::CStr::from_bytes_until_nul(&buf) {
                let hostname = s.to_string_lossy().to_string();
                if !hostname.is_empty() {
                    return Ok(Value::String(hostname));
                }
            }
        }
    }
    Ok(Value::String("localhost".to_string()))
}

pub(super) fn native_os_type(
    _interp: &mut Interpreter,
    _this: &Value,
    _args: &[Value],
) -> Result<Value> {
    let os_type = if cfg!(target_os = "linux") {
        "Linux"
    } else if cfg!(target_os = "macos") {
        "Darwin"
    } else if cfg!(target_os = "windows") {
        "Windows_NT"
    } else {
        "Unknown"
    };
    Ok(Value::String(os_type.to_string()))
}

pub(super) fn native_os_release(
    _interp: &mut Interpreter,
    _this: &Value,
    _args: &[Value],
) -> Result<Value> {
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/proc/version") {
            return Ok(Value::String(content.trim().to_string()));
        }
    }
    Ok(Value::String("unknown".to_string()))
}

pub(super) fn native_os_homedir(
    _interp: &mut Interpreter,
    _this: &Value,
    _args: &[Value],
) -> Result<Value> {
    if let Ok(home) = std::env::var("HOME") {
        if !home.is_empty() {
            return Ok(Value::String(home));
        }
    }
    #[cfg(target_os = "windows")]
    {
        if let Ok(home) = std::env::var("USERPROFILE") {
            if !home.is_empty() {
                return Ok(Value::String(home));
            }
        }
    }
    Ok(Value::String("/".to_string()))
}

pub(super) fn native_os_tmpdir(
    _interp: &mut Interpreter,
    _this: &Value,
    _args: &[Value],
) -> Result<Value> {
    let tmp = std::env::temp_dir().to_string_lossy().to_string();
    Ok(Value::String(tmp))
}
