use tails::compiler::lexer::tokenize;
use tails::compiler::parser::parse;
use tails::compiler::{type_checker::TypeChecker, Compiler};

fn type_check(source: &str) -> Result<(), tails::Error> {
    let tokens = tokenize(source).unwrap();
    let ast = parse(&tokens).unwrap();
    TypeChecker::check(&ast)
}

fn compile_and_run(source: &str) -> Result<(), tails::Error> {
    let compiler = Compiler::new(true);
    let compiled = compiler.compile(source)?;
    let mut interp = tails::vm::Interpreter::new()?;
    interp.execute(&compiled)?;
    Ok(())
}

#[test]
fn test_basic_type_annotation_number() {
    type_check("let x: number = 42;").unwrap();
}

#[test]
fn test_basic_type_annotation_string() {
    type_check(r#"let x: string = "hello";"#).unwrap();
}

#[test]
fn test_basic_type_annotation_boolean() {
    type_check("let x: boolean = true;").unwrap();
}

#[test]
fn test_type_error_number_assigned_string() {
    let result = type_check("let x: number = \"hello\";");
    assert!(result.is_err());
}

#[test]
fn test_type_error_string_assigned_number() {
    let result = type_check(r#"let x: string = 42;"#);
    assert!(result.is_err());
}

#[test]
fn test_type_error_boolean_assigned_number() {
    let result = type_check("let x: boolean = 42;");
    assert!(result.is_err());
}

#[test]
fn test_const_requires_initializer() {
    let result = type_check("const x: number;");
    assert!(result.is_err());
}

#[test]
fn test_const_type_check() {
    type_check("const x: number = 42;").unwrap();
}

#[test]
fn test_untyped_variable() {
    type_check("let x = 42;").unwrap();
}

#[test]
fn test_union_type_number_string() {
    type_check("let x: number | string = 42;").unwrap();
    type_check(r#"let x: number | string = "hello";"#).unwrap();
}

#[test]
fn test_union_type_rejects_invalid() {
    let result = type_check("let x: number | string = true;");
    assert!(result.is_err());
}

#[test]
fn test_literal_type_number() {
    type_check("let x: 42 = 42;").unwrap();
}

#[test]
fn test_literal_type_rejects_wrong_value() {
    let result = type_check("let x: 42 = 43;");
    assert!(result.is_err());
}

#[test]
fn test_literal_type_string() {
    type_check(r#"let x: "hello" = "hello";"#).unwrap();
}

#[test]
fn test_literal_type_string_rejects() {
    let result = type_check(r#"let x: "hello" = "world";"#);
    assert!(result.is_err());
}

#[test]
fn test_function_param_types() {
    type_check("function add(x: number, y: number): number { return x + y; }").unwrap();
}

#[test]
fn test_function_param_type_mismatch() {
    let result = type_check(r#"function greet(name: string) { return 42; } greet(42);"#);
    assert!(result.is_err());
}

#[test]
fn test_function_return_type_check() {
    type_check(r#"function greet(name: string): string { return "hello " + name; }"#).unwrap();
}

#[test]
fn test_array_type() {
    type_check("let arr: number[] = [1, 2, 3];").unwrap();
}

#[test]
fn test_array_type_rejects_wrong_element() {
    let result = type_check(r#"let arr: number[] = [1, "two", 3];"#);
    assert!(result.is_err());
}

#[test]
fn test_tuple_type() {
    type_check(r#"let t: [string, number] = ["hello", 42];"#).unwrap();
}

#[test]
fn test_tuple_type_wrong_length() {
    let result = type_check(r#"let t: [string, number] = ["hello"];"#);
    assert!(result.is_err());
}

#[test]
fn test_tuple_type_wrong_types() {
    let result = type_check(r#"let t: [string, number] = [42, "hello"];"#);
    assert!(result.is_err());
}

#[test]
fn test_object_type() {
    type_check(r#"let obj: { name: string, age: number } = { name: "John", age: 30 };"#).unwrap();
}

#[test]
fn test_object_type_missing_property() {
    let result = type_check(r#"let obj: { name: string, age: number } = { name: "John" };"#);
    assert!(result.is_err());
}

#[test]
fn test_object_type_wrong_property_type() {
    let result =
        type_check(r#"let obj: { name: string, age: number } = { name: "John", age: "thirty" };"#);
    assert!(result.is_err());
}

#[test]
fn test_optional_property() {
    type_check(r#"let obj: { name: string, age?: number } = { name: "John" };"#).unwrap();
    type_check(r#"let obj: { name: string, age?: number } = { name: "John", age: 30 };"#).unwrap();
}

#[test]
fn test_interface_declaration() {
    type_check(
        r#"
        interface User { name: string, age: number }
        let u: User = { name: "Alice", age: 25 };
    "#,
    )
    .unwrap();
}

#[test]
fn test_interface_rejects_wrong_type() {
    let result = type_check(
        r#"
        interface User { name: string, age: number }
        let u: User = { name: "Alice", age: "twenty-five" };
    "#,
    );
    assert!(result.is_err());
}

#[test]
fn test_type_alias() {
    type_check(
        r#"
        type ID = string | number;
        let id: ID = 42;
    "#,
    )
    .unwrap();
    type_check(
        r#"
        type ID = string | number;
        let id: ID = "abc";
    "#,
    )
    .unwrap();
}

#[test]
fn test_type_alias_rejects_invalid() {
    let result = type_check(
        r#"
        type ID = string | number;
        let id: ID = true;
    "#,
    );
    assert!(result.is_err());
}

#[test]
fn test_enum_declaration() {
    type_check(
        r#"
        enum Direction { Up, Down, Left, Right }
    "#,
    )
    .unwrap();
}

#[test]
fn test_enum_with_values() {
    type_check(
        r#"
        enum Status { Active = 1, Inactive = 0 }
    "#,
    )
    .unwrap();
}

#[test]
fn test_type_assertion() {
    type_check(
        r#"
        let x: any = 42;
        let y: number = x as number;
    "#,
    )
    .unwrap();
}

#[test]
fn test_type_narrowing_after_typeof() {
    type_check(
        r#"
        let x: string | number = 42;
        if (typeof x === "string") {
            let y: string = x;
        } else {
            let z: number = x;
        }
    "#,
    )
    .unwrap();
}

#[test]
fn test_basic_generics_function() {
    type_check(
        r#"
        function identity<T>(x: T): T { return x; }
    "#,
    )
    .unwrap();
}

#[test]
fn test_any_type_accepts_all() {
    type_check("let x: any = 42;").unwrap();
    type_check(r#"let x: any = "hello";"#).unwrap();
    type_check("let x: any = true;").unwrap();
    type_check("let x: any = null;").unwrap();
}

#[test]
fn test_null_type() {
    type_check("let x: null = null;").unwrap();
}

#[test]
fn test_undefined_type() {
    type_check("let x: undefined = undefined;").unwrap();
}

#[test]
fn test_void_return() {
    type_check("function noop(): void { }").unwrap();
}

#[test]
fn test_run_typed_number() {
    compile_and_run("let x: number = 42;").unwrap();
}

#[test]
fn test_run_typed_string() {
    compile_and_run(r#"let x: string = "hello";"#).unwrap();
}

#[test]
fn test_run_typed_function() {
    compile_and_run(
        r#"
        function add(x: number, y: number): number { return x + y; }
        add(3, 4);
    "#,
    )
    .unwrap();
}

#[test]
fn test_run_typed_array() {
    compile_and_run("let arr: number[] = [1, 2, 3];").unwrap();
}

#[test]
fn test_run_typed_object() {
    compile_and_run(r#"let obj: { name: string } = { name: "test" };"#).unwrap();
}

#[test]
fn test_run_interface() {
    compile_and_run(
        r#"
        interface Point { x: number, y: number }
        let p: Point = { x: 1, y: 2 };
    "#,
    )
    .unwrap();
}

#[test]
fn test_run_type_alias() {
    compile_and_run(
        r#"
        type Num = number | string;
        let x: Num = 42;
    "#,
    )
    .unwrap();
}

#[test]
fn test_run_enum() {
    compile_and_run(
        r#"
        enum Color { Red, Green, Blue }
    "#,
    )
    .unwrap();
}

#[test]
fn test_run_type_assertion() {
    compile_and_run(
        r#"
        let x: any = 42;
        let y: number = x as number;
    "#,
    )
    .unwrap();
}

#[test]
fn test_run_function_type_signature() {
    compile_and_run(
        r#"
        function greet(name: string): string { return "hello " + name; }
        greet("world");
    "#,
    )
    .unwrap();
}

#[test]
fn test_run_tuple_type() {
    compile_and_run(r#"let t: [string, number] = ["hello", 42];"#).unwrap();
}
