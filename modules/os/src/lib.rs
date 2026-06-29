#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub model: String,
    pub speed: f64,
    pub times: CpuTimes,
}

#[derive(Debug, Clone)]
pub struct CpuTimes {
    pub user: i64,
    pub nice: i64,
    pub sys: i64,
    pub idle: i64,
    pub irq: i64,
}

pub fn platform() -> &'static str {
    if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "macos") {
        "darwin"
    } else if cfg!(target_os = "windows") {
        "win32"
    } else {
        "unknown"
    }
}

pub fn arch() -> &'static str {
    if cfg!(target_arch = "x86_64") {
        "x64"
    } else if cfg!(target_arch = "aarch64") {
        "arm64"
    } else if cfg!(target_arch = "x86") {
        "x86"
    } else {
        "unknown"
    }
}

pub fn cpus() -> Vec<CpuInfo> {
    let count = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    (0..count)
        .map(|_| CpuInfo {
            model: "CPU".to_string(),
            speed: 0.0,
            times: CpuTimes {
                user: 0,
                nice: 0,
                sys: 0,
                idle: 0,
                irq: 0,
            },
        })
        .collect()
}

pub fn totalmem() -> f64 {
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
            for line in content.lines() {
                if let Some(rest) = line.strip_prefix("MemTotal:") {
                    if let Some(kb_str) = rest.split_whitespace().next() {
                        if let Ok(kb) = kb_str.parse::<f64>() {
                            return kb * 1024.0;
                        }
                    }
                }
            }
        }
    }
    16.0 * 1024.0 * 1024.0 * 1024.0
}

pub fn freemem() -> f64 {
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
            for line in content.lines() {
                if let Some(rest) = line.strip_prefix("MemAvailable:") {
                    if let Some(kb_str) = rest.split_whitespace().next() {
                        if let Ok(kb) = kb_str.parse::<f64>() {
                            return kb * 1024.0;
                        }
                    }
                }
            }
        }
    }
    8.0 * 1024.0 * 1024.0 * 1024.0
}

pub fn uptime() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

pub fn hostname() -> std::io::Result<String> {
    if let Ok(hostname) = std::env::var("HOSTNAME") {
        if !hostname.is_empty() {
            return Ok(hostname);
        }
    }
    #[cfg(unix)]
    {
        let mut buf = [0u8; 256];
        if unsafe { libc::gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf.len()) } == 0 {
            if let Ok(s) = std::ffi::CStr::from_bytes_until_nul(&buf) {
                let hostname = s.to_string_lossy().to_string();
                if !hostname.is_empty() {
                    return Ok(hostname);
                }
            }
        }
    }
    Ok("localhost".to_string())
}

pub fn os_type() -> &'static str {
    if cfg!(target_os = "linux") {
        "Linux"
    } else if cfg!(target_os = "macos") {
        "Darwin"
    } else if cfg!(target_os = "windows") {
        "Windows_NT"
    } else {
        "Unknown"
    }
}

pub fn release() -> String {
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/proc/version") {
            return content.trim().to_string();
        }
    }
    "unknown".to_string()
}

pub fn homedir() -> String {
    if let Ok(home) = std::env::var("HOME") {
        if !home.is_empty() {
            return home;
        }
    }
    #[cfg(target_os = "windows")]
    {
        if let Ok(home) = std::env::var("USERPROFILE") {
            if !home.is_empty() {
                return home;
            }
        }
    }
    "/".to_string()
}

pub fn tmpdir() -> String {
    std::env::temp_dir().to_string_lossy().to_string()
}
