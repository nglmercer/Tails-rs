use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct FileStat {
    pub size: u64,
    pub is_file: bool,
    pub is_directory: bool,
    pub is_symbolic_link: bool,
    pub mode: Option<u32>,
    pub mtime_ms: Option<f64>,
    pub birthtime_ms: Option<f64>,
}

pub fn read_file(path: &str) -> std::io::Result<String> {
    fs::read_to_string(path)
}

pub fn write_file(path: &str, content: &str) -> std::io::Result<()> {
    fs::write(path, content)
}

pub fn exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn mkdir(path: &str, recursive: bool) -> std::io::Result<()> {
    if recursive {
        fs::create_dir_all(path)
    } else {
        fs::create_dir(path)
    }
}

pub fn readdir(path: &str) -> std::io::Result<Vec<String>> {
    let entries: Vec<String> = fs::read_dir(path)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect();
    Ok(entries)
}

pub fn stat(path: &str) -> std::io::Result<FileStat> {
    let metadata = fs::metadata(path)?;

    let mode = {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            Some(metadata.mode())
        }
        #[cfg(not(unix))]
        {
            None
        }
    };

    let mtime_ms = metadata.modified().ok().and_then(|t| {
        t.duration_since(std::time::UNIX_EPOCH)
            .ok()
            .map(|d| d.as_secs_f64() * 1000.0)
    });

    let birthtime_ms = {
        #[cfg(unix)]
        {
            metadata.created().ok().and_then(|t| {
                t.duration_since(std::time::UNIX_EPOCH)
                    .ok()
                    .map(|d| d.as_secs_f64() * 1000.0)
            })
        }
        #[cfg(not(unix))]
        {
            None
        }
    };

    Ok(FileStat {
        size: metadata.len(),
        is_file: metadata.is_file(),
        is_directory: metadata.is_dir(),
        is_symbolic_link: metadata.file_type().is_symlink(),
        mode,
        mtime_ms,
        birthtime_ms,
    })
}

pub fn unlink(path: &str) -> std::io::Result<()> {
    fs::remove_file(path)
}

pub fn rm(path: &str, recursive: bool) -> std::io::Result<()> {
    if recursive {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}

pub fn copy_file(src: &str, dest: &str) -> std::io::Result<u64> {
    fs::copy(src, dest)
}

pub fn rename(old_path: &str, new_path: &str) -> std::io::Result<()> {
    fs::rename(old_path, new_path)
}

pub fn append_file(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    file.write_all(content.as_bytes())
}
