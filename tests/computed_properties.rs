use tails::TailsRuntime;

#[test]
fn test_computed_property_name() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(r#"
        const key = "foo";
        const obj = { [key]: 42 };
        obj.foo;
    "#).unwrap();
    assert_eq!(result.to_string(), "42");
}

#[test]
fn test_computed_property_expression() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(r#"
        const obj = { ["b" + "ar"]: 99 };
        obj.bar;
    "#).unwrap();
    assert_eq!(result.to_string(), "99");
}

#[test]
fn test_computed_property_number() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(r#"
        const obj = { ["three"]: 3 };
        obj.three;
    "#).unwrap();
    assert_eq!(result.to_string(), "3");
}

#[test]
fn test_computed_with_shorthand() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(r#"
        const x = 10;
        const obj = { x, ["y"]: 20 };
        obj.x + obj.y;
    "#).unwrap();
    assert_eq!(result.to_string(), "30");
}
