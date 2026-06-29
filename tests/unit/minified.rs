use tails::compiler::lexer::tokenize;
use tails::compiler::parser::parse;
use tails::compiler::Compiler;

fn test_tokenize(source: &str, label: &str) {
    tokenize(source).unwrap_or_else(|e| panic!("[{}] Tokenize failed: {:?}", label, e));
}

fn test_parse(source: &str, label: &str) {
    let tokens = tokenize(source).unwrap_or_else(|e| panic!("[{}] Tokenize failed: {:?}", label, e));
    parse(&tokens).unwrap_or_else(|e| panic!("[{}] Parse failed: {:?}", label, e));
}

fn test_compile(source: &str, label: &str) {
    let compiler = Compiler::new(false);
    compiler.compile(source).unwrap_or_else(|e| panic!("[{}] Compile failed: {:?}", label, e));
}

// ============================================
// SECTION 1: Basic minified literals (no spaces)
// ============================================

#[test]
fn min_number_literals() {
    test_tokenize("42;3.14;0xff;0b1010;0o77", "number_literals");
    test_parse("42;3.14;0xff;0b1010;0o77", "number_literals");
    test_compile("42;3.14;0xff;0b1010;0o77", "number_literals");
}

#[test]
fn min_string_literals() {
    test_compile(r#""hello";"world";"escape\nhere";"""# , "string_literals");
}

#[test]
fn min_boolean_and_null() {
    test_compile("true;false;null;undefined", "boolean_and_null");
}

// ============================================
// SECTION 2: Variable declarations (minified)
// ============================================

#[test]
fn min_var_const_let() {
    test_compile("const x=42;const y=\"hello\";let z=true;var w=null", "var_const_let");
}

#[test]
fn min_declarations_no_spaces() {
    test_compile("const a=1;const b=2;const c=a+b", "declarations_no_spaces");
}

#[test]
fn min_multiple_assignments() {
    test_compile("let x=0;x=10;x=20", "multiple_assignments");
}

#[test]
fn min_compound_operators() {
    test_compile("let x=10;x+=5;x-=3;x*=2;x/=4;x%=3", "compound_operators");
}

// ============================================
// SECTION 3: Binary and unary operators (minified)
// ============================================

#[test]
fn min_arithmetic() {
    test_compile("const x=2+3;const y=10-5;const z=4*5;const w=20/4;const m=10%3;const p=2**3", "arithmetic");
}

#[test]
fn min_comparison() {
    test_compile("const a=1==2;const b=1===2;const c=1!=2;const d=1!==2;const e=1<2;const f=1>2;const g=1<=2;const h=1>=2", "comparison");
}

#[test]
fn min_logical() {
    test_compile("const a=true&&false;const b=true||false;const c=!true", "logical");
}

#[test]
fn min_unary() {
    test_compile("const a=-5;const b=!true;const c=+10", "unary");
}

#[test]
fn min_bitwise() {
    test_compile("const a=5&3;const b=5|3;const c=5^3;const d=~5;const e=1<<2;const f=8>>1", "bitwise");
}

// ============================================
// SECTION 4: Function declarations (minified)
// ============================================

#[test]
fn min_function_basic() {
    test_compile("function add(a,b){return a+b};add(1,2)", "function_basic");
}

#[test]
fn min_function_no_params() {
    test_compile("function greet(){return\"hi\"};greet()", "function_no_params");
}

#[test]
fn min_function_nested_calls() {
    test_compile("function double(x){return x*2};function quad(x){return double(double(x))};quad(5)", "function_nested_calls");
}

#[test]
fn min_function_recursive() {
    test_compile("function fact(n){if(n<=1){return 1}return n*fact(n-1)};fact(5)", "function_recursive");
}

#[test]
fn min_function_closures() {
    test_compile("function makeAdder(n){return function(x){return x+n}};const add5=makeAdder(5);add5(3)", "function_closures");
}

// ============================================
// SECTION 5: Control flow (minified)
// ============================================

#[test]
fn min_if_else() {
    test_compile("const x=10;if(x>5){x=x+1}else{x=x-1}", "if_else");
}

#[test]
fn min_if_elseif_else() {
    test_compile("const x=10;if(x>10){1}else if(x>5){2}else{3}", "if_elseif_else");
}

#[test]
fn min_while_loop() {
    test_compile("let i=0;while(i<10){i=i+1}", "while_loop");
}

#[test]
fn min_for_loop() {
    test_compile("let s=0;for(let i=0;i<100;i++){s=s+i}", "for_loop");
}

#[test]
fn min_for_in() {
    test_compile("const o={a:1,b:2};const keys=[];for(const k in o){keys.push(k)}", "for_in");
}

#[test]
fn min_for_of() {
    test_compile("const arr=[1,2,3];let sum=0;for(const v of arr){sum=sum+v}", "for_of");
}

#[test]
fn min_nested_loops() {
    test_compile("let s=0;for(let i=0;i<10;i++){for(let j=0;j<10;j++){s=s+1}}", "nested_loops");
}

// ============================================
// SECTION 6: Arrays and objects (minified)
// ============================================

#[test]
fn min_array_literal() {
    test_compile("const arr=[1,2,3,4,5];arr[0]", "array_literal");
}

#[test]
fn min_object_literal() {
    test_compile("const obj={a:1,b:\"two\",c:true};obj.a", "object_literal");
}

#[test]
fn min_nested_objects() {
    test_compile("const d={u:{n:\"J\",a:{c:\"NY\"}}};d.u.a.c", "nested_objects");
}

#[test]
fn min_array_iteration() {
    test_compile("const arr=[10,20,30];let sum=0;for(let i=0;i<arr.length;i++){sum=sum+arr[i]}", "array_iteration");
}

#[test]
fn min_object_iteration() {
    test_compile("const o={x:1,y:2,z:3};let s=0;for(const k in o){s=s+o[k]}", "object_iteration");
}

// ============================================
// SECTION 7: Ternary (minified)
// ============================================

#[test]
fn min_ternary() {
    test_compile("const x=10;const r=(x>5)?\"big\":\"small\"", "ternary");
}

#[test]
fn min_ternary_nested() {
    test_compile("const x=10;const r=(x>10)?\"a\":((x>5)?\"b\":\"c\")", "ternary_nested");
}

// ============================================
// SECTION 8: New operator (minified)
// ============================================

#[test]
fn min_new_object() {
    test_compile("const d=new Date();d", "new_object");
}

#[test]
fn min_new_array() {
    test_compile("const arr=new Array(10);arr", "new_array");
}

// ============================================
// SECTION 9: Postfix/prefix (minified)
// ============================================

#[test]
fn min_postfix_increment() {
    test_compile("let x=10;x++", "postfix_increment");
}

#[test]
fn min_postfix_decrement() {
    test_compile("let x=10;x--", "postfix_decrement");
}

#[test]
fn min_prefix_increment() {
    test_compile("let x=10;++x", "prefix_increment");
}

#[test]
fn min_prefix_decrement() {
    test_compile("let x=10;--x", "prefix_decrement");
}

// ============================================
// SECTION 10: Delete, typeof, instanceof, in (minified)
// ============================================

#[test]
fn min_delete() {
    test_compile("const o={a:1,b:2};delete o.b;o", "delete");
}

#[test]
fn min_typeof() {
    test_compile("const x=42;const t=typeof x", "typeof");
}

#[test]
fn min_instanceof() {
    test_compile("class A{};const o=new A();const r=o instanceof A", "instanceof");
}

#[test]
fn min_in_operator() {
    test_compile("const o={a:1};const r=\"a\" in o", "in_operator");
}

// ============================================
// SECTION 11: Complex minified programs
// ============================================

#[test]
fn min_fibonacci() {
    test_compile("function fib(n){if(n<=1){return n}return fib(n-1)+fib(n-2)};fib(10)", "fibonacci");
}

#[test]
fn min_factorial() {
    test_compile("function fact(n){let r=1;for(let i=2;i<=n;i++){r=r*i}return r};fact(10)", "factorial");
}

#[test]
fn min_bubble_sort() {
    test_compile("function sort(arr){for(let i=0;i<arr.length;i++){for(let j=0;j<arr.length-i-1;j++){if(arr[j]>arr[j+1]){const t=arr[j];arr[j]=arr[j+1];arr[j+1]=t}}}return arr};sort([3,1,4,1,5])", "bubble_sort");
}

#[test]
fn min_string_reversal() {
    test_compile("function rev(s){let r=\"\";for(let i=s.length-1;i>=0;i--){r=r+s[i]}return r};rev(\"hello\")", "string_reversal");
}

#[test]
fn min_sum_array() {
    test_compile("function sum(arr){let s=0;for(let i=0;i<arr.length;i++){s=s+arr[i]}return s};sum([1,2,3,4,5])", "sum_array");
}

#[test]
fn min_max_of_array() {
    test_compile("function mx(arr){let m=arr[0];for(let i=1;i<arr.length;i++){if(arr[i]>m){m=arr[i]}}return m};mx([3,1,4,1,5,9])", "max_of_array");
}

#[test]
fn min_is_palindrome() {
    test_compile("function pal(s){let l=0;let r=s.length-1;while(l<r){if(s[l]!==s[r]){return false}l++;r--}return true};pal(\"racecar\")", "is_palindrome");
}

#[test]
fn min_swap() {
    test_compile("let a=1;let b=2;let t=a;a=b;b=t", "swap");
}

#[test]
fn min_nested_if() {
    test_compile("const x=10;if(x>0){if(x>5){1}else{2}}else{0}", "nested_if");
}

#[test]
fn min_mixed_program() {
    test_compile("const PI=3.14159;function area(r){return PI*r*r};function circ(r){return 2*PI*r};area(5);circ(5)", "mixed_program");
}

#[test]
fn min_array_of_objects() {
    test_compile("const items=[{id:1,n:\"a\"},{id:2,n:\"b\"},{id:3,n:\"c\"}]", "array_of_objects");
}

#[test]
fn min_try_catch() {
    test_compile("try{const x=1/0}catch(e){0}", "try_catch");
}

#[test]
fn min_switch() {
    test_compile("const x=2;switch(x){case 1:\"one\";break;case 2:\"two\";break;default:\"other\"}", "switch");
}

#[test]
fn min_do_while() {
    test_compile("let i=0;do{i=i+1}while(i<10)", "do_while");
}
