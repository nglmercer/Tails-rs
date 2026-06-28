use tails::TailsRuntime;

#[test]
fn test_atob_btoa_roundtrip() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"
        let encoded = btoa("Hello, World!");
        let decoded = atob(encoded);
        decoded;
    "#);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), tails::Value::String("Hello, World!".to_string()));
}

#[test]
fn test_btoa_known_value() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"
        btoa("Hi");
    "#);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), tails::Value::String("SGk=".to_string()));
}

#[test]
fn test_atob_known_value() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"
        atob("SGk=");
    "#);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), tails::Value::String("Hi".to_string()));
}

#[test]
fn test_atob_empty_string() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"
        atob("");
    "#);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), tails::Value::String("".to_string()));
}

#[test]
fn test_btoa_empty_string() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"
        btoa("");
    "#);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), tails::Value::String("".to_string()));
}
