use tails::TailsRuntime;

#[test]
fn test_process_platform() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(
        r#"
        let p = process.platform;
        p;
    "#,
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
    let r = rt.eval(
        r#"
        let a = process.arch;
        a;
    "#,
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
    let r = rt.eval(
        r#"
        let p = process.pid;
        p;
    "#,
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
    let r = rt.eval(
        r#"
        let d = process.cwd();
        d;
    "#,
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
    let r = rt.eval(
        r#"
        let a = process.argv;
        a.length;
    "#,
    );
    assert!(r.is_ok());
    if let tails::Value::Float(n) = r.unwrap() {
        assert!(n >= 1.0, "argv should have at least one element");
    } else {
        panic!("Expected number for argv.length");
    }
}

#[test]
fn test_process_env() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(
        r#"
        let home = process.env.HOME;
        typeof home;
    "#,
    );
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), tails::Value::String("string".to_string()));
}

#[test]
fn test_process_stdout_write() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(
        r#"
        let result = process.stdout.write("test");
        result;
    "#,
    );
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), tails::Value::Boolean(true));
}
