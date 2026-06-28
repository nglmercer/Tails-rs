use tails::{TailsRuntime, Value};

// ---- BigInt ----
#[test]
fn test_bigint_literal() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"42n;"#).unwrap();
    assert_eq!(r, Value::BigInt(42));
}

#[test]
fn test_bigint_typeof() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"typeof 100n;"#).unwrap();
    assert_eq!(r, Value::String("bigint".to_string()));
}

#[test]
fn test_bigint_addition() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"10n + 20n;"#).unwrap();
    assert_eq!(r, Value::BigInt(30));
}

#[test]
fn test_bigint_subtraction() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"50n - 20n;"#).unwrap();
    assert_eq!(r, Value::BigInt(30));
}

#[test]
fn test_bigint_multiplication() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"6n * 7n;"#).unwrap();
    assert_eq!(r, Value::BigInt(42));
}

#[test]
fn test_bigint_division() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"100n / 4n;"#).unwrap();
    assert_eq!(r, Value::BigInt(25));
}

#[test]
fn test_bigint_modulo() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"17n % 5n;"#).unwrap();
    assert_eq!(r, Value::BigInt(2));
}

#[test]
fn test_bigint_power() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"2n ** 10n;"#).unwrap();
    assert_eq!(r, Value::BigInt(1024));
}

#[test]
fn test_bigint_negate() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"-42n;"#).unwrap();
    assert_eq!(r, Value::BigInt(-42));
}

#[test]
fn test_bigint_comparison() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"10n < 20n;"#).unwrap();
    assert_eq!(r, Value::Boolean(true));
}

#[test]
fn test_bigint_equality() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"42n === 42n;"#).unwrap();
    assert_eq!(r, Value::Boolean(true));
}

#[test]
fn test_bigint_constructor() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"BigInt(123);"#).unwrap();
    assert_eq!(r, Value::BigInt(123));
}

#[test]
fn test_bigint_from_string() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"BigInt("456");"#).unwrap();
    assert_eq!(r, Value::BigInt(456));
}

// ---- Date ----
#[test]
fn test_date_constructor() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"let d = new Date(0); d.getTime();"#).unwrap();
    assert_eq!(r, Value::Float(0.0));
}

#[test]
fn test_date_from_millis() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"let d = new Date(1000); d.getTime();"#).unwrap();
    assert_eq!(r, Value::Float(1000.0));
}

#[test]
fn test_date_from_components() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(r#"let d = new Date(2024, 0, 15, 12, 30, 45); d.getFullYear();"#)
        .unwrap();
    assert_eq!(r, Value::Float(2024.0));
}

#[test]
fn test_date_get_month() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(r#"let d = new Date(2024, 5, 15); d.getMonth();"#)
        .unwrap();
    assert_eq!(r, Value::Float(5.0));
}

#[test]
fn test_date_get_date() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(r#"let d = new Date(2024, 0, 15); d.getDate();"#)
        .unwrap();
    assert_eq!(r, Value::Float(15.0));
}

#[test]
fn test_date_to_iso_string() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"let d = new Date(0); d.toISOString();"#).unwrap();
    assert_eq!(r, Value::String("1970-01-01T00:00:00.000Z".to_string()));
}

#[test]
fn test_date_value_of() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"let d = new Date(12345); d.valueOf();"#).unwrap();
    assert_eq!(r, Value::Float(12345.0));
}

#[test]
fn test_date_now_static() {
    let mut rt = TailsRuntime::default();
    let r = rt.eval(r#"typeof Date.now();"#).unwrap();
    assert_eq!(r, Value::String("number".to_string()));
}

#[test]
fn test_date_parse_iso() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(r#"Date.parse("2024-01-15T00:00:00.000Z");"#)
        .unwrap();
    // Should be some number > 0
    match r {
        Value::Float(f) => assert!(f > 0.0),
        _ => panic!("Expected Float"),
    }
}

// ---- RegExp ----
#[test]
fn test_regexp_constructor() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(r#"let re = new RegExp("hello"); re.test("hello world");"#)
        .unwrap();
    assert_eq!(r, Value::Boolean(true));
}

#[test]
fn test_regexp_test_false() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(r#"let re = new RegExp("xyz"); re.test("hello world");"#)
        .unwrap();
    assert_eq!(r, Value::Boolean(false));
}

#[test]
fn test_regexp_with_flags() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(r#"let re = new RegExp("hello", "i"); re.test("HELLO world");"#)
        .unwrap();
    assert_eq!(r, Value::Boolean(true));
}

#[test]
fn test_regexp_to_string() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(r#"let re = new RegExp("abc", "gi"); re.toString();"#)
        .unwrap();
    assert_eq!(r, Value::String("/abc/gi".to_string()));
}

#[test]
fn test_regexp_exec_with_capture() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(r#"let re = new RegExp("(\\d+)-(\\d+)-(\\d+)"); re.exec("2024-01-15");"#)
        .unwrap();
    match r {
        Value::Array(idx) => {
            let arr = rt.get_array_element(&Value::Array(idx), 0);
            assert_eq!(arr, Some(Value::String("2024-01-15".to_string())));
            let g1 = rt.get_array_element(&Value::Array(idx), 1);
            assert_eq!(g1, Some(Value::String("2024".to_string())));
            let g2 = rt.get_array_element(&Value::Array(idx), 2);
            assert_eq!(g2, Some(Value::String("01".to_string())));
            let g3 = rt.get_array_element(&Value::Array(idx), 3);
            assert_eq!(g3, Some(Value::String("15".to_string())));
        }
        _ => panic!("Expected Array"),
    }
}

#[test]
fn test_regexp_source() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(r#"let re = new RegExp("hello", "gi"); re.source();"#)
        .unwrap();
    assert_eq!(r, Value::String("hello".to_string()));
}

#[test]
fn test_regexp_flags() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(r#"let re = new RegExp("abc", "gim"); re.flags();"#)
        .unwrap();
    assert_eq!(r, Value::String("gim".to_string()));
}

#[test]
fn test_regexp_global() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(r#"let re = new RegExp("a", "g"); re.global();"#)
        .unwrap();
    assert_eq!(r, Value::Boolean(true));
    let r = rt
        .eval(r#"let re = new RegExp("a"); re.global();"#)
        .unwrap();
    assert_eq!(r, Value::Boolean(false));
}

#[test]
fn test_regexp_ignore_case() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(r#"let re = new RegExp("a", "i"); re.ignoreCase();"#)
        .unwrap();
    assert_eq!(r, Value::Boolean(true));
}

#[test]
fn test_regexp_last_index() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(r#"let re = new RegExp("a", "g"); re.lastIndex();"#)
        .unwrap();
    assert_eq!(r, Value::Float(0.0));
}

#[test]
fn test_regexp_constructor_from_regexp() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(r#"let re1 = new RegExp("abc"); let re2 = new RegExp(re1); re2.source();"#)
        .unwrap();
    assert_eq!(r, Value::String("abc".to_string()));
}

// ---- Iterator Helpers ----
#[test]
fn test_symbol_iterator_on_array() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(
            r#"
        typeof Symbol.iterator;
    "#,
        )
        .unwrap();
    assert_eq!(r, Value::String("symbol".to_string()));
}

#[test]
fn test_array_iterator_method() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(
            r#"
        let arr = [1, 2, 3];
        let iter = arr[Symbol.iterator];
        typeof iter;
    "#,
        )
        .unwrap();
    assert_eq!(r, Value::String("function".to_string()));
}

#[test]
fn test_array_iterator_call() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(
            r#"
        let arr = [1, 2, 3];
        let iter = arr[Symbol.iterator]();
        typeof iter;
    "#,
        )
        .unwrap();
    assert_eq!(r, Value::String("object".to_string()));
}

#[test]
fn test_iterator_has_to_array() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(
            r#"
        let arr = [1, 2, 3];
        let iter = arr[Symbol.iterator]();
        typeof iter.toArray;
    "#,
        )
        .unwrap();
    assert_eq!(r, Value::String("function".to_string()));
}

#[test]
fn test_iterator_to_array_basic() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(
            r#"
        let arr = [1, 2, 3];
        let iter = arr[Symbol.iterator]();
        let result = iter.toArray();
        result.length;
    "#,
        )
        .unwrap();
    assert_eq!(r, Value::Float(3.0));
}

#[test]
fn test_iterator_to_array() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(
            r#"
        let arr = [1, 2, 3];
        let iter = arr[Symbol.iterator]();
        let result = iter.toArray();
        result.length;
    "#,
        )
        .unwrap();
    assert_eq!(r, Value::Float(3.0));
}

#[test]
fn test_iterator_map() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(
            r#"
        let arr = [1, 2, 3];
        let iter = arr[Symbol.iterator]();
        let mapped = iter.map(function(x) { return x * 2; });
        let result = mapped.toArray();
        result.length;
    "#,
        )
        .unwrap();
    assert_eq!(r, Value::Float(3.0));
}

#[test]
fn test_iterator_filter() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(
            r#"
        let arr = [1, 2, 3, 4, 5];
        let iter = arr[Symbol.iterator]();
        let filtered = iter.filter(function(x) { return x > 2; });
        let result = filtered.toArray();
        result.length;
    "#,
        )
        .unwrap();
    assert_eq!(r, Value::Float(3.0));
}

#[test]
fn test_iterator_take() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(
            r#"
        let arr = [1, 2, 3, 4, 5];
        let iter = arr[Symbol.iterator]();
        let taken = iter.take(3);
        let result = taken.toArray();
        result.length;
    "#,
        )
        .unwrap();
    assert_eq!(r, Value::Float(3.0));
}

#[test]
fn test_iterator_drop() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(
            r#"
        let arr = [1, 2, 3, 4, 5];
        let iter = arr[Symbol.iterator]();
        let dropped = iter.drop(2);
        let result = dropped.toArray();
        result.length;
    "#,
        )
        .unwrap();
    assert_eq!(r, Value::Float(3.0));
}

#[test]
fn test_iterator_for_each() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(
            r#"
        let sum = 0;
        let arr = [1, 2, 3];
        let iter = arr[Symbol.iterator]();
        iter.forEach(function(x) { sum = sum + x; });
        sum;
    "#,
        )
        .unwrap();
    assert_eq!(r, Value::Integer(6));
}

// ---- for await...of ----
#[test]
fn test_for_await_of_simple() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(
            r#"
        let sum = 0;
        let arr = [1, 2, 3];
        for (let val of arr) {
            sum = sum + val;
        }
        sum;
    "#,
        )
        .unwrap();
    assert_eq!(r, Value::Integer(6));
}

#[test]
fn test_for_await_of_with_promises() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(
            r#"
        let results = [];
        let arr = [Promise.resolve(1)];
        for await (let val of arr) {
            results.push(val);
        }
        results.length;
    "#,
        )
        .unwrap();
    assert_eq!(r, Value::Float(1.0));
}

#[test]
fn test_for_await_of_with_resolved_promises() {
    let mut rt = TailsRuntime::default();
    let r = rt
        .eval(
            r#"
        let results = [];
        let arr = [Promise.resolve(1), Promise.resolve(2), Promise.resolve(3)];
        for await (let val of arr) {
            results.push(val);
        }
        results.length;
    "#,
        )
        .unwrap();
    assert_eq!(r, Value::Float(3.0));
}
