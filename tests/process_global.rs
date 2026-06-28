use std::path::Path;
use tails::TailsRuntime;

#[test]
fn test_process_platform() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval_module(
        r#"
        import process from "./process.native";
        process.platform;
    "#,
        Path::new("/tmp/test_module.ts"),
    );
    assert!(r.is_ok());
    let val = r.unwrap();
    if let tails::Value::String(s) = val {
        assert!(
            s == "linux" || s == "darwin" || s == "win32",
            "Unexpected platform: {}",
            s
        );
    } else {
        panic!("Expected string for platform");
    }
}

#[test]
fn test_process_arch() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval_module(
        r#"
        import process from "./process.native";
        process.arch;
    "#,
        Path::new("/tmp/test_module.ts"),
    );
    assert!(r.is_ok());
    let val = r.unwrap();
    if let tails::Value::String(s) = val {
        assert!(
            s == "x64" || s == "arm64" || s == "unknown",
            "Unexpected arch: {}",
            s
        );
    } else {
        panic!("Expected string for arch");
    }
}

#[test]
fn test_process_pid() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval_module(
        r#"
        import process from "./process.native";
        process.pid;
    "#,
        Path::new("/tmp/test_module.ts"),
    );
    assert!(r.is_ok());
    if let tails::Value::Integer(n) = r.unwrap() {
        assert!(n > 0, "PID should be positive");
    } else {
        panic!("Expected integer for pid");
    }
}

#[test]
fn test_process_cwd() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval_module(
        r#"
        import process from "./process.native";
        process.cwd();
    "#,
        Path::new("/tmp/test_module.ts"),
    );
    assert!(r.is_ok());
    if let tails::Value::String(s) = r.unwrap() {
        assert!(!s.is_empty(), "cwd should not be empty");
    } else {
        panic!("Expected string for cwd");
    }
}

#[test]
fn test_process_argv() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval_module(
        r#"
        import process from "./process.native";
        process.argv.length;
    "#,
        Path::new("/tmp/test_module.ts"),
    );
    assert!(r.is_ok());
    let val = r.unwrap();
    match val {
        tails::Value::Float(n) => assert!(n >= 1.0, "argv should have at least one element"),
        tails::Value::Integer(n) => assert!(n >= 1, "argv should have at least one element"),
        _ => panic!("Expected number for argv.length"),
    }
}

#[test]
fn test_process_env() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval_module(
        r#"
        import process from "./process.native";
        typeof process.env;
    "#,
        Path::new("/tmp/test_module.ts"),
    );
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), tails::Value::String("object".to_string()));
}

#[test]
fn test_process_stdout_write() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval_module(
        r#"
        import process from "./process.native";
        process.stdout.write("test");
    "#,
        Path::new("/tmp/test_module.ts"),
    );
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), tails::Value::Boolean(true));
}
