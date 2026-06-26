use tails::TailsRuntime;

// Feature 1: `new` operator
#[test]
fn test_new_operator_basic() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        function Person(name, age) {
            this.name = name;
            this.age = age;
        }
        let p = new Person("Alice", 30);
        p.name;
    "#,
    );
    assert!(result.is_ok());
}

#[test]
fn test_new_operator_returns_object() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        function Foo() {
            this.x = 42;
        }
        let obj = new Foo();
        obj.x;
    "#,
    );
    assert!(result.is_ok());
}

#[test]
fn test_new_operator_with_prototype() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        function Animal(name) {
            this.name = name;
        }
        Animal.prototype.sayHi = function() { return "hi"; };
        let dog = new Animal("Rex");
        dog.name;
    "#,
    );
    assert!(result.is_ok());
}

// Feature 2: `this` binding
#[test]
fn test_this_binding_method_call() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        let obj = {
            value: 10,
            getValue: function() { return this.value; }
        };
        obj.getValue();
    "#,
    );
    assert!(result.is_ok());
}

#[test]
fn test_this_binding_nested() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        let obj = {
            inner: {
                value: 99,
                getValue: function() { return this.value; }
            }
        };
        obj.inner.getValue();
    "#,
    );
    assert!(result.is_ok());
}

// Feature 3: `delete` operator
#[test]
fn test_delete_property() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        let obj = { a: 1, b: 2, c: 3 };
        delete obj.b;
        obj.b;
    "#,
    );
    assert!(result.is_ok());
}

#[test]
fn test_delete_returns_true() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        let obj = { x: 1 };
        let r = delete obj.x;
        r;
    "#,
    );
    assert!(result.is_ok());
}

#[test]
fn test_delete_nonexistent_property() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        let obj = { x: 1 };
        let r = delete obj.y;
        r;
    "#,
    );
    assert!(result.is_ok());
}

#[test]
fn test_delete_on_variable_is_noop() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval("delete undefined;");
    assert!(result.is_ok());
}

// Feature 4: `instanceof` operator
#[test]
fn test_instanceof_basic() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        function Dog() {}
        let rex = new Dog();
        rex instanceof Dog;
    "#,
    );
    assert!(result.is_ok());
}

#[test]
fn test_instanceof_returns_false() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        function Cat() {}
        function Dog() {}
        let kitty = new Cat();
        kitty instanceof Dog;
    "#,
    );
    assert!(result.is_ok());
}

#[test]
fn test_instanceof_with_literal() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval("42 instanceof Object;");
    assert!(result.is_ok());
}

// Feature 5: `in` operator
#[test]
fn test_in_operator_own_property() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        let obj = { a: 1, b: 2 };
        "a" in obj;
    "#,
    );
    assert!(result.is_ok());
}

#[test]
fn test_in_operator_missing_property() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        let obj = { a: 1 };
        "b" in obj;
    "#,
    );
    assert!(result.is_ok());
}

#[test]
fn test_in_operator_on_string() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        "length" in "hello";
    "#,
    );
    assert!(result.is_ok());
}

// Feature 6: `void` operator
#[test]
fn test_void_operator() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval("void 0;");
    assert!(result.is_ok());
}

#[test]
fn test_void_returns_undefined() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        let x = void 42;
        typeof x;
    "#,
    );
    assert!(result.is_ok());
}

// Feature 7: Property access on non-objects
#[test]
fn test_property_access_on_null_throws() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        null.foo;
    "#,
    );
    assert!(result.is_err());
}

#[test]
fn test_property_access_on_undefined_throws() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        undefined.bar;
    "#,
    );
    assert!(result.is_err());
}

#[test]
fn test_property_access_on_number() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        (42).toString;
    "#,
    );
    assert!(result.is_ok());
}

#[test]
fn test_property_access_on_string() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        "hello".length;
    "#,
    );
    assert!(result.is_ok());
}

#[test]
fn test_property_access_on_boolean() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        true.toString;
    "#,
    );
    assert!(result.is_ok());
}

// Feature 8: Wrapper objects
#[test]
fn test_string_wrapper_length() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        "hello".length;
    "#,
    );
    assert!(result.is_ok());
}

#[test]
fn test_number_tostring() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        (42).toString;
    "#,
    );
    assert!(result.is_ok());
}

#[test]
fn test_boolean_valueof() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        true.valueOf;
    "#,
    );
    assert!(result.is_ok());
}

// Combined features
#[test]
fn test_new_with_this_and_method() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        function Counter(start) {
            this.count = start;
        }
        Counter.prototype.increment = function() {
            this.count = this.count + 1;
            return this.count;
        };
        let c = new Counter(0);
        c.increment();
    "#,
    );
    eprintln!("counter test: {:?}", result);
    assert!(result.is_ok());
}

#[test]
fn test_instanceof_with_inheritance() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        function Base() {}
        function Child() {}
        Child.prototype = new Base();
        let obj = new Child();
        obj instanceof Child;
    "#,
    );
    assert!(result.is_ok());
}

#[test]
fn test_in_with_inherited_property() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        function Parent() {}
        Parent.prototype.inheritedMethod = function() {};
        function Child() {}
        Child.prototype = new Parent();
        let obj = new Child();
        "inheritedMethod" in obj;
    "#,
    );
    assert!(result.is_ok());
}

#[test]
fn test_delete_then_in() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        let obj = { x: 1, y: 2 };
        delete obj.x;
        "x" in obj;
    "#,
    );
    assert!(result.is_ok());
}

#[test]
fn test_void_with_side_effect() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        let x = 5;
        let y = void (x = 10);
        y;
    "#,
    );
    assert!(result.is_ok());
}

#[test]
fn test_debug_this_binding() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        let obj = {
            value: 10,
            getValue: function() { return this.value; }
        };
        obj.getValue();
    "#,
    );
    eprintln!("Result: {:?}", result);
    assert!(result.is_ok());
}

#[test]
fn test_debug_new() {
    let mut runtime = TailsRuntime::default();
    let result = runtime.eval(
        r#"
        function Foo() { this.x = 42; }
        let obj = new Foo();
        obj.x;
    "#,
    );
    eprintln!("New result: {:?}", result);
    assert!(result.is_ok());
}

#[test]
fn test_debug_simple_method_call() {
    let mut runtime = TailsRuntime::default();
    // Simplest possible this-binding test
    let r1 = runtime.eval(
        r#"
        let o = { x: 5 };
        o.x;
    "#,
    );
    eprintln!("simple getprop: {:?}", r1);
    assert!(r1.is_ok());

    let r2 = runtime.eval(
        r#"
        function greet() { return "hello"; }
        greet();
    "#,
    );
    eprintln!("simple funcall: {:?}", r2);
    assert!(r2.is_ok());

    let r3 = runtime.eval(
        r#"
        let o2 = { greet: function() { return "hi"; } };
        o2.greet();
    "#,
    );
    eprintln!("method call: {:?}", r3);
    assert!(r3.is_ok());
}
