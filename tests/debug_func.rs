use tails::TailsRuntime;

#[test]
fn test_simple_function() {
    let mut runtime = TailsRuntime::default();
    match runtime.eval(r#"
        function add(a, b) {
            return a + b;
        }
        add(3, 4);
    "#) {
        Ok(result) => {
            println!("Simple function result: {:?}", result);
            assert_eq!(result, tails::Value::Float(7.0));
        }
        Err(e) => {
            println!("Simple function error: {:?}", e);
            panic!("Simple function failed");
        }
    }
}

#[test]
fn test_local_vars() {
    let mut runtime = TailsRuntime::default();
    match runtime.eval(r#"
        function calculate(x) {
            const doubled = x * 2;
            return doubled;
        }
        calculate(5);
    "#) {
        Ok(result) => {
            println!("Local vars result: {:?}", result);
            assert_eq!(result, tails::Value::Float(10.0));
        }
        Err(e) => {
            println!("Local vars error: {:?}", e);
            panic!("Local vars failed");
        }
    }
}

#[test]
fn test_recursion() {
    let mut runtime = TailsRuntime::default();
    match runtime.eval(r#"
        function factorial(n) {
            if (n <= 1) {
                return 1;
            }
            return n * factorial(n - 1);
        }
        factorial(5);
    "#) {
        Ok(result) => {
            println!("Recursion result: {:?}", result);
            assert_eq!(result, tails::Value::Float(120.0));
        }
        Err(e) => {
            println!("Recursion error: {:?}", e);
            panic!("Recursion failed");
        }
    }
}
