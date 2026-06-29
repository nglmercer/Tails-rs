use std::io::Write;

pub fn cwd() -> std::io::Result<String> {
    Ok(std::env::current_dir()?.to_string_lossy().to_string())
}

pub fn chdir(dir: &str) -> std::io::Result<()> {
    std::env::set_current_dir(dir)
}

pub fn stdout_write(data: &str) -> std::io::Result<usize> {
    let mut stdout = std::io::stdout();
    stdout.write_all(data.as_bytes())?;
    Ok(data.len())
}

pub fn hrtime() -> (u64, u32) {
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    (dur.as_secs(), dur.subsec_nanos())
}

pub fn hrtime_bigint() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
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

pub fn pid() -> u32 {
    std::process::id()
}

pub fn env_vars() -> Vec<(String, String)> {
    std::env::vars().collect()
}

pub fn argv() -> Vec<String> {
    std::env::args().collect()
}
