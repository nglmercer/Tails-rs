use tails::TailsRuntime;

#[test]
fn test_simple_while_in_func() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"
        function f(n) {
            let i = 0;
            while (i < n) {
                i = i + 1;
            }
            return i;
        }
        f(5);
    "#);
    match &r {
        Ok(v) => println!("result: {:?}", v),
        Err(e) => println!("error: {:?}", e),
    }
    assert_eq!(r.unwrap(), tails::Value::Float(5.0));
}

#[test]
fn test_let_sum() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"
        function f() {
            let sum = 10;
            return sum;
        }
        f();
    "#);
    match &r {
        Ok(v) => println!("result: {:?}", v),
        Err(e) => println!("error: {:?}", e),
    }
    assert_eq!(r.unwrap(), tails::Value::Float(10.0));
}

#[test]
fn test_let_add() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"
        function f() {
            let a = 3;
            let b = 4;
            return a + b;
        }
        f();
    "#);
    match &r {
        Ok(v) => println!("result: {:?}", v),
        Err(e) => println!("error: {:?}", e),
    }
    assert_eq!(r.unwrap(), tails::Value::Float(7.0));
}
