use tails::TailsRuntime;

#[test]
fn test_basic_function() {
    let mut runtime = TailsRuntime::default();
    let result = runtime
        .eval(
            r#"
        function add(a, b) {
            return a + b;
        }
        add(3, 4);
    "#,
        )
        .unwrap();
    assert_eq!(result, tails::Value::Float(7.0));
}

#[test]
fn test_nested_function_calls() {
    let mut runtime = TailsRuntime::default();
    let result = runtime
        .eval(
            r#"
        function square(x) {
            return x * x;
        }
        function sum(a, b) {
            return a + b;
        }
        sum(square(3), square(4));
    "#,
        )
        .unwrap();
    assert_eq!(result, tails::Value::Float(25.0));
}

#[test]
fn test_recursive_factorial() {
    let mut runtime = TailsRuntime::default();
    let result = runtime
        .eval(
            r#"
        function factorial(n) {
            if (n <= 1) {
                return 1;
            }
            return n * factorial(n - 1);
        }
        factorial(5);
    "#,
        )
        .unwrap();
    assert_eq!(result, tails::Value::Float(120.0));
}

#[test]
fn test_recursive_fibonacci() {
    let mut runtime = TailsRuntime::default();
    let result = runtime
        .eval(
            r#"
        function fibonacci(n) {
            if (n <= 1) {
                return n;
            }
            return fibonacci(n - 1) + fibonacci(n - 2);
        }
        fibonacci(10);
    "#,
        )
        .unwrap();
    assert_eq!(result, tails::Value::Float(55.0));
}

#[test]
fn test_function_with_local_variables() {
    let mut runtime = TailsRuntime::default();
    let result = runtime
        .eval(
            r#"
        function calculate(x) {
            const doubled = x * 2;
            const added = doubled + 10;
            return added;
        }
        calculate(5);
    "#,
        )
        .unwrap();
    assert_eq!(result, tails::Value::Float(20.0));
}

#[test]
fn test_function_as_expression() {
    let mut runtime = TailsRuntime::default();
    let result = runtime
        .eval(
            r#"
        const multiply = function(a, b) {
            return a * b;
        };
        multiply(6, 7);
    "#,
        )
        .unwrap();
    assert_eq!(result, tails::Value::Float(42.0));
}

#[test]
fn test_closure_basic() {
    let mut runtime = TailsRuntime::default();
    let result = runtime
        .eval(
            r#"
        function makeCounter() {
            let count = 0;
            function increment() {
                count = count + 1;
                return count;
            }
            return increment;
        }
        const counter = makeCounter();
        counter();
    "#,
        )
        .unwrap();
    assert_eq!(result, tails::Value::Float(1.0));
}

#[test]
fn test_closure_multiple_calls() {
    let mut runtime = TailsRuntime::default();
    let result = runtime
        .eval(
            r#"
        function makeCounter() {
            let count = 0;
            function increment() {
                count = count + 1;
                return count;
            }
            return increment;
        }
        const counter = makeCounter();
        counter();
        counter();
        counter();
    "#,
        )
        .unwrap();
    assert_eq!(result, tails::Value::Float(3.0));
}

#[test]
fn test_closure_capture() {
    let mut runtime = TailsRuntime::default();
    let result = runtime
        .eval(
            r#"
        function makeAdder(x) {
            function adder(y) {
                return x + y;
            }
            return adder;
        }
        const add5 = makeAdder(5);
        add5(10);
    "#,
        )
        .unwrap();
    assert_eq!(result, tails::Value::Float(15.0));
}

#[test]
fn test_function_with_if_else() {
    let mut runtime = TailsRuntime::default();
    let result = runtime
        .eval(
            r#"
        function max(a, b) {
            if (a > b) {
                return a;
            } else {
                return b;
            }
        }
        max(10, 20);
    "#,
        )
        .unwrap();
    assert_eq!(result, tails::Value::Float(20.0));
}

#[test]
fn test_function_with_while_loop() {
    let mut runtime = TailsRuntime::default();
    let result = runtime
        .eval(
            r#"
        function sumUpTo(n) {
            let sum = 0;
            let i = 1;
            while (i <= n) {
                sum = sum + i;
                i = i + 1;
            }
            return sum;
        }
        sumUpTo(10);
    "#,
        )
        .unwrap();
    assert_eq!(result, tails::Value::Float(55.0));
}

#[test]
fn test_higher_order_function() {
    let mut runtime = TailsRuntime::default();
    let result = runtime
        .eval(
            r#"
        function applyTwice(f, x) {
            return f(f(x));
        }
        function double(x) {
            return x * 2;
        }
        applyTwice(double, 3);
    "#,
        )
        .unwrap();
    assert_eq!(result, tails::Value::Float(12.0));
}
