use tails::TailsRuntime;

#[test]
fn test_basic_evaluation() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval("42;");
    assert!(result.is_ok());
}

#[test]
fn test_arithmetic() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval("2 + 3;");
    assert!(result.is_ok());
}

#[test]
fn test_variable_declaration() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval("const x = 10; x;");
    assert!(result.is_ok());
}

#[test]
fn test_string_literal() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(r#""hello";"#);
    assert!(result.is_ok());
}

#[test]
fn test_boolean_literal() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval("true;");
    assert!(result.is_ok());
}

#[test]
fn test_null_literal() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval("null;");
    assert!(result.is_ok());
}

#[test]
fn test_undefined_literal() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval("undefined;");
    assert!(result.is_ok());
}

#[test]
fn test_if_statement() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval("if (true) { 1; } else { 2; }");
    assert!(result.is_ok());
}

#[test]
fn test_while_loop() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        let sum = 0;
        let i = 1;
        while (i <= 5) {
            sum = sum + i;
            i = i + 1;
        }
        sum;
    "#,
    );
    assert!(result.is_ok());
}

#[test]
fn test_function_declaration() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        function add(a, b) {
            return a + b;
        }
        add(3, 4);
    "#,
    );
    assert!(result.is_ok());
}

#[test]
fn test_global_variable() {
    let mut runtime = TailsRuntime::default();
    runtime.set_global("myVar", tails::Value::Float(100.0));
    let result = runtime.eval("myVar;");
    assert!(result.is_ok());
}

#[test]
fn test_complex_program() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        function factorial(n) {
            if (n <= 1) {
                return 1;
            }
            return n * factorial(n - 1);
        }
        factorial(5);
    "#,
    );
    assert!(result.is_ok());
}
