use tails::TailsRuntime;
use std::path::Path;

#[test]
fn test_inline_module() {
    eprintln!("Starting test_inline_module");
    let mut runtime = TailsRuntime::default();
    let source = r#"
        export function add(a, b) {
            return a + b;
        }
        export const PI = 3.14159;
    "#;
    let dir = Path::new(".");
    eprintln!("Calling eval_module");
    runtime.eval_module(&source, dir).unwrap();
    eprintln!("Calling eval");
    let result = runtime.eval("add(2, 3)").unwrap();
    assert_eq!(result, tails::Value::Float(5.0));
    eprintln!("Done");
}

#[test]
fn test_inline_module_no_pollution() {
    let mut runtime = TailsRuntime::default();
    let source = r#"
        const secret = "hidden";
        export function add(a, b) {
            return a + b;
        }
    "#;
    let dir = Path::new(".");
    runtime.eval_module(&source, dir).unwrap();
    let result = runtime.eval("add(2, 3)").unwrap();
    assert_eq!(result, tails::Value::Float(5.0));
    let result = runtime.eval("typeof secret !== 'undefined' ? 'leaked' : 'ok'").unwrap();
    assert_eq!(result, tails::Value::String("ok".to_string()));
}

#[test]
fn test_file_module() {
    let mut runtime = TailsRuntime::default();
    let source = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/modules/math.ts")
    ).unwrap();
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/modules");
    runtime.eval_module(&source, &dir).unwrap();
    let result = runtime.eval("add(2, 3)").unwrap();
    assert_eq!(result, tails::Value::Float(5.0));
}
