use tails::TailsRuntime;

#[test]
fn test_path_join() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"
        path.join("/foo", "bar", "baz");
    "#);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), tails::Value::String("/foo/bar/baz".to_string()));
}

#[test]
fn test_path_join_single() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"
        path.join("foo");
    "#);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), tails::Value::String("foo".to_string()));
}

#[test]
fn test_path_basename() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"
        path.basename("/foo/bar/baz.txt");
    "#);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), tails::Value::String("baz.txt".to_string()));
}

#[test]
fn test_path_basename_with_ext() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"
        path.basename("/foo/bar/baz.txt", ".txt");
    "#);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), tails::Value::String("baz".to_string()));
}

#[test]
fn test_path_dirname() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"
        path.dirname("/foo/bar/baz.txt");
    "#);
    assert!(r.is_ok());
    assert_eq!(
        r.unwrap(),
        tails::Value::String("/foo/bar".to_string())
    );
}

#[test]
fn test_path_extname() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"
        path.extname("/foo/bar/baz.txt");
    "#);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), tails::Value::String(".txt".to_string()));
}

#[test]
fn test_path_extname_no_ext() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"
        path.extname("/foo/bar/baz");
    "#);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), tails::Value::String("".to_string()));
}

#[test]
fn test_path_is_absolute() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"
        path.isAbsolute("/foo");
    "#);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), tails::Value::Boolean(true));
}

#[test]
fn test_path_is_not_absolute() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"
        path.isAbsolute("foo");
    "#);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), tails::Value::Boolean(false));
}

#[test]
fn test_path_normalize() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"
        path.normalize("/foo/../bar");
    "#);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), tails::Value::String("/bar".to_string()));
}

#[test]
fn test_path_normalize_dots() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"
        path.normalize("/foo/./bar/../baz");
    "#);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), tails::Value::String("/foo/baz".to_string()));
}

#[test]
fn test_path_sep() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"
        path.sep;
    "#);
    assert!(r.is_ok());
    let val = r.unwrap();
    if let tails::Value::String(s) = val {
        assert!(s == "/" || s == "\\");
    } else {
        panic!("Expected string for sep");
    }
}

#[test]
fn test_path_delimiter() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"
        path.delimiter;
    "#);
    assert!(r.is_ok());
    let val = r.unwrap();
    if let tails::Value::String(s) = val {
        assert!(s == ":" || s == ";");
    } else {
        panic!("Expected string for delimiter");
    }
}
