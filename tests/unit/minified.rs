use tails::compiler::Compiler;

fn test_compile(source: &str, label: &str) {
    let compiler = Compiler::new(false);
    compiler
        .compile(source)
        .unwrap_or_else(|e| panic!("[{}] Compile failed: {:?}", label, e));
}

// ============================================
// SECTION 1: Basic minified literals (no whitespace)
// ============================================

#[test]
fn min_number_literals() {
    test_compile("42;3.14;0xff;0b1010;0o77;", "number_literals");
}

#[test]
fn min_string_literals() {
    test_compile(r#""hello";"world";"escape\nhere";"""#, "string_literals");
}

#[test]
fn min_boolean_and_null() {
    test_compile("true;false;null;undefined;", "boolean_and_null");
}

// ============================================
// SECTION 2: Variable declarations (minified)
// ============================================

#[test]
fn min_var_const_let() {
    test_compile(
        "const x=42;const y=\"hello\";let z=true;var w=null;",
        "var_const_let",
    );
}

#[test]
fn min_declarations_no_spaces() {
    test_compile("const a=1;const b=2;const c=a+b;", "declarations_no_spaces");
}

#[test]
fn min_multiple_assignments() {
    test_compile("let x=0;x=10;x=20;", "multiple_assignments");
}

#[test]
fn min_compound_operators() {
    test_compile("let x=10;x+=5;x-=3;x*=2;x/=4;x%=3;", "compound_operators");
}

// ============================================
// SECTION 3: Binary operators (minified)
// ============================================

#[test]
fn min_arithmetic() {
    test_compile(
        "const x=2+3;const y=10-5;const z=4*5;const w=20/4;const m=10%3;const p=2**3;",
        "arithmetic",
    );
}

#[test]
fn min_comparison() {
    test_compile("const a=1==2;const b=1===2;const c=1!=2;const d=1!==2;const e=1<2;const f=1>2;const g=1<=2;const h=1>=2;", "comparison");
}

#[test]
fn min_logical() {
    test_compile(
        "const a=true&&false;const b=true||false;const c=!true;",
        "logical",
    );
}

#[test]
fn min_unary() {
    test_compile("const a=-5;const b=!true;const c=+10;", "unary");
}

#[test]
fn min_bitwise() {
    test_compile(
        "const a=5&3;const b=5|3;const c=5^3;const d=~5;const e=1<<2;const f=8>>1;",
        "bitwise",
    );
}

// ============================================
// SECTION 4: Function declarations (minified)
// ============================================

#[test]
fn min_function_basic() {
    test_compile("function add(a,b){return a+b;}add(1,2);", "function_basic");
}

#[test]
fn min_function_no_params() {
    test_compile(
        "function greet(){return\"hi\";}greet();",
        "function_no_params",
    );
}

#[test]
fn min_function_nested_calls() {
    test_compile(
        "function double(x){return x*2;}function quad(x){return double(double(x));}quad(5);",
        "function_nested_calls",
    );
}

#[test]
fn min_function_recursive() {
    test_compile(
        "function fact(n){if(n<=1){return 1;}return n*fact(n-1);}fact(5);",
        "function_recursive",
    );
}

#[test]
fn min_function_closures() {
    test_compile(
        "function makeAdder(n){return function(x){return x+n;};}const add5=makeAdder(5);add5(3);",
        "function_closures",
    );
}

// ============================================
// SECTION 5: Control flow (minified)
// ============================================

#[test]
fn min_if_else() {
    test_compile("const x=10;if(x>5){x=x+1;}else{x=x-1;}", "if_else");
}

#[test]
fn min_if_elseif_else() {
    test_compile(
        "const x=10;if(x>10){1;}else if(x>5){2;}else{3;}",
        "if_elseif_else",
    );
}

#[test]
fn min_while_loop() {
    test_compile("let i=0;while(i<10){i=i+1;}", "while_loop");
}

#[test]
fn min_for_loop() {
    test_compile("let s=0;for(let i=0;i<100;i++){s=s+i;}", "for_loop");
}

#[test]
fn min_for_in() {
    test_compile(
        "const o={a:1,b:2};const keys=[];for(const k in o){keys.push(k);}",
        "for_in",
    );
}

#[test]
fn min_for_of() {
    test_compile(
        "const arr=[1,2,3];let sum=0;for(const v of arr){sum=sum+v;}",
        "for_of",
    );
}

#[test]
fn min_nested_loops() {
    test_compile(
        "let s=0;for(let i=0;i<10;i++){for(let j=0;j<10;j++){s=s+1;}}",
        "nested_loops",
    );
}

#[test]
fn min_do_while() {
    test_compile("let i=0;do{i=i+1;}while(i<10);", "do_while");
}

#[test]
fn min_switch() {
    test_compile(
        "const x=2;switch(x){case 1:\"one\";break;case 2:\"two\";break;default:\"other\";}",
        "switch",
    );
}

#[test]
fn min_try_catch() {
    test_compile("try{const x=1/0;}catch(e){0;}", "try_catch");
}

// ============================================
// SECTION 6: Arrays and objects (minified)
// ============================================

#[test]
fn min_array_literal() {
    test_compile("const arr=[1,2,3,4,5];arr[0];", "array_literal");
}

#[test]
fn min_object_literal() {
    test_compile("const obj={a:1,b:\"two\",c:true};obj.a;", "object_literal");
}

#[test]
fn min_nested_objects() {
    test_compile(
        "const d={u:{n:\"J\",a:{c:\"NY\"}}};d.u.a.c;",
        "nested_objects",
    );
}

#[test]
fn min_array_iteration() {
    test_compile(
        "const arr=[10,20,30];let sum=0;for(let i=0;i<arr.length;i++){sum=sum+arr[i];}",
        "array_iteration",
    );
}

#[test]
fn min_object_iteration() {
    test_compile(
        "const o={x:1,y:2,z:3};let s=0;for(const k in o){s=s+o[k];}",
        "object_iteration",
    );
}

#[test]
fn min_array_of_objects() {
    test_compile(
        "const items=[{id:1,n:\"a\"},{id:2,n:\"b\"},{id:3,n:\"c\"}];",
        "array_of_objects",
    );
}

// ============================================
// SECTION 7: Ternary (minified)
// ============================================

#[test]
fn min_ternary() {
    test_compile("const x=10;const r=(x>5)?\"big\":\"small\";", "ternary");
}

#[test]
fn min_ternary_nested() {
    test_compile(
        "const x=10;const r=(x>10)?\"a\":((x>5)?\"b\":\"c\");",
        "ternary_nested",
    );
}

// ============================================
// SECTION 8: New operator (minified)
// ============================================

#[test]
fn min_new_object() {
    test_compile("const d=new Date();d;", "new_object");
}

#[test]
fn min_new_array() {
    test_compile("const arr=new Array(10);arr;", "new_array");
}

// ============================================
// SECTION 9: Postfix/prefix (minified)
// ============================================

#[test]
fn min_postfix_increment() {
    test_compile("let x=10;x++;", "postfix_increment");
}

#[test]
fn min_postfix_decrement() {
    test_compile("let x=10;x--;", "postfix_decrement");
}

#[test]
fn min_prefix_increment() {
    test_compile("let x=10;++x;", "prefix_increment");
}

#[test]
fn min_prefix_decrement() {
    test_compile("let x=10;--x;", "prefix_decrement");
}

// ============================================
// SECTION 10: Delete, typeof, instanceof, in
// ============================================

#[test]
fn min_delete() {
    test_compile("const o={a:1,b:2};delete o.b;o;", "delete");
}

#[test]
fn min_typeof() {
    test_compile("const x=42;const t=typeof x;", "typeof");
}

#[test]
fn min_instanceof() {
    test_compile(
        "class A{}const o=new A();const r=o instanceof A;",
        "instanceof",
    );
}

#[test]
fn min_in_operator() {
    test_compile("const o={a:1};const r=\"a\" in o;", "in_operator");
}

// ============================================
// SECTION 11: Complex minified programs
// ============================================

#[test]
fn min_fibonacci() {
    test_compile(
        "function fib(n){if(n<=1){return n;}return fib(n-1)+fib(n-2);}fib(10);",
        "fibonacci",
    );
}

#[test]
fn min_factorial() {
    test_compile(
        "function fact(n){let r=1;for(let i=2;i<=n;i++){r=r*i;}return r;}fact(10);",
        "factorial",
    );
}

#[test]
fn min_bubble_sort() {
    test_compile("function sort(arr){for(let i=0;i<arr.length;i++){for(let j=0;j<arr.length-i-1;j++){if(arr[j]>arr[j+1]){const t=arr[j];arr[j]=arr[j+1];arr[j+1]=t;}}}return arr;}sort([3,1,4,1,5]);", "bubble_sort");
}

#[test]
fn min_string_reversal() {
    test_compile("function rev(s){let r=\"\";for(let i=s.length-1;i>=0;i--){r=r+s[i];}return r;}rev(\"hello\");", "string_reversal");
}

#[test]
fn min_sum_array() {
    test_compile("function sum(arr){let s=0;for(let i=0;i<arr.length;i++){s=s+arr[i];}return s;}sum([1,2,3,4,5]);", "sum_array");
}

#[test]
fn min_max_of_array() {
    test_compile("function mx(arr){let m=arr[0];for(let i=1;i<arr.length;i++){if(arr[i]>m){m=arr[i];}}return m;}mx([3,1,4,1,5,9]);", "max_of_array");
}

#[test]
fn min_is_palindrome() {
    test_compile("function pal(s){let l=0;let r=s.length-1;while(l<r){if(s[l]!==s[r]){return false;}l++;r--;}return true;}pal(\"racecar\");", "is_palindrome");
}

#[test]
fn min_swap() {
    test_compile("let a=1;let b=2;let t=a;a=b;b=t;", "swap");
}

#[test]
fn min_nested_if() {
    test_compile(
        "const x=10;if(x>0){if(x>5){1;}else{2;}}else{0;}",
        "nested_if",
    );
}

#[test]
fn min_mixed_program() {
    test_compile("const PI=3.14159;function area(r){return PI*r*r;}function circ(r){return 2*PI*r;}area(5);circ(5);", "mixed_program");
}

// ============================================
// SECTION 12: Class (minified)
// ============================================

#[test]
fn min_class_basic() {
    test_compile("class Foo{constructor(x){this.x=x;}method(){return this.x;}}const f=new Foo(42);f.method();", "class_basic");
}

#[test]
fn min_class_inheritance() {
    test_compile("class Base{constructor(x){this.x=x;}}class Child extends Base{getVal(){return this.x;}}const c=new Child(10);c.getVal();", "class_inheritance");
}

// ============================================
// SECTION 13: Power operator
// ============================================

#[test]
fn min_power_operator() {
    test_compile("const x=2**10;", "power_operator");
}

// ============================================
// SECTION 14: Full programs (minified)
// ============================================

#[test]
fn min_full_program() {
    test_compile(
        "const x=42;const y=\"hello\";function add(a,b){return a+b;}if(x>10){add(x,1);}else{0;}",
        "full_program",
    );
}

#[test]
fn min_fibonacci_full() {
    test_compile("function fibonacci(n){if(n<=1){return n;}return fibonacci(n-1)+fibonacci(n-2);}const result=fibonacci(10);console.log(result);", "fibonacci_full");
}

#[test]
fn min_gcd() {
    test_compile(
        "function gcd(a,b){while(b!==0){const t=b;b=a%b;a=t;}return a;}gcd(48,18);",
        "gcd",
    );
}

#[test]
fn min_is_prime() {
    test_compile("function isPrime(n){if(n<2){return false;}for(let i=2;i*i<=n;i++){if(n%i===0){return false;}}return true;}isPrime(17);", "is_prime");
}

// ============================================
// SECTION 15: NEW - Arrow functions (minified)
// ============================================

#[test]
fn min_arrow_single_param() {
    test_compile("const double=x=>x*2;double(5);", "arrow_single_param");
}

#[test]
fn min_arrow_multi_param() {
    test_compile("const add=(a,b)=>a+b;add(3,4);", "arrow_multi_param");
}

#[test]
fn min_arrow_block_body() {
    test_compile(
        "const greet=()=>{return\"hi\";};greet();",
        "arrow_block_body",
    );
}

#[test]
fn min_arrow_in_array() {
    test_compile(
        "const arr=[1,2,3];const doubled=arr.map(x=>x*2);",
        "arrow_in_array",
    );
}

#[test]
fn min_arrow_chained() {
    test_compile(
        "const arr=[1,2,3,4,5];const result=arr.filter(x=>x>2).map(x=>x*x);",
        "arrow_chained",
    );
}

// ============================================
// SECTION 16: NEW - Template literals (minified)
// ============================================

#[test]
fn min_template_literal() {
    test_compile(
        "const name=\"World\";const msg=`Hello,${name}!`;",
        "template_literal",
    );
}

#[test]
fn min_template_expression() {
    test_compile(
        "const x=5;const msg=`Result: ${x*2}`;",
        "template_expression",
    );
}

// ============================================
// SECTION 17: NEW - Optional chaining (minified)
// ============================================

#[test]
fn min_optional_chaining() {
    test_compile(
        "const obj={a:{b:1}};const val=obj?.a?.b;",
        "optional_chaining",
    );
}

#[test]
fn min_optional_call() {
    test_compile(
        "const obj={fn:()=>42};const val=obj?.fn();",
        "optional_call",
    );
}

// ============================================
// SECTION 18: NEW - Nullish coalescing (minified)
// ============================================

#[test]
fn min_nullish_coalescing() {
    test_compile(
        "const x=null;const val=x??\"default\";",
        "nullish_coalescing",
    );
}

#[test]
fn min_nullish_chain() {
    test_compile("const x=null;const val=x??\"a\"??\"b\";", "nullish_chain");
}

// ============================================
// SECTION 19: NEW - Destructuring (minified)
// ============================================

#[test]
fn min_array_destructuring() {
    test_compile("const[a,b,c]=[1,2,3];", "array_destructuring");
}

#[test]
fn min_object_destructuring() {
    test_compile("const{x,y}={x:1,y:2};", "object_destructuring");
}

#[test]
fn min_destructuring_default() {
    test_compile("const{x=10}={};", "destructuring_default");
}

#[test]
fn min_rest_destructuring() {
    test_compile("const[a,...rest]=[1,2,3,4];", "rest_destructuring");
}

// ============================================
// SECTION 20: NEW - Spread operator (minified)
// ============================================

#[test]
fn min_spread_array() {
    test_compile("const a=[1,2];const b=[...a,3,4];", "spread_array");
}

#[test]
fn min_spread_object() {
    test_compile("const o={a:1};const p={...o,b:2};", "spread_object");
}

#[test]
fn min_spread_function_args() {
    test_compile(
        "function sum(a,b,c){return a+b+c;}const r=sum(1,2,3);",
        "spread_function_args",
    );
}

// ============================================
// SECTION 21: NEW - Generators (minified)
// ============================================

#[test]
fn min_generator() {
    test_compile(
        "function* gen(){yield 1;yield 2;yield 3;}const g=gen();g.next();",
        "generator",
    );
}

// ============================================
// SECTION 22: NEW - Default parameters (minified)
// ============================================

#[test]
fn min_default_params() {
    test_compile(
        "function greet(name=\"World\"){return\"Hello \"+name;}greet();",
        "default_params",
    );
}

#[test]
fn min_default_params_multiple() {
    test_compile(
        "function add(a=1,b=2){return a+b;}add();",
        "default_params_multiple",
    );
}

#[test]
fn min_default_params_override() {
    test_compile(
        "function greet(name=\"World\"){return name;}greet(\"JS\");",
        "default_params_override",
    );
}

// ============================================
// SECTION 23: NEW - Rest parameters (minified)
// ============================================

#[test]
fn min_rest_params() {
    test_compile("function sum(...args){let s=0;for(let i=0;i<args.length;i++){s=s+args[i];}return s;}sum(1,2,3);", "rest_params");
}

#[test]
fn min_rest_with_normal() {
    test_compile(
        "function log(level,...msgs){return msgs;}log(\"info\",\"a\",\"b\");",
        "rest_with_normal",
    );
}

// ============================================
// SECTION 24: NEW - Async/Await (minified)
// ============================================

#[test]
fn min_async_function() {
    test_compile("async function fetchData(){return 42;}", "async_function");
}

#[test]
fn min_async_arrow() {
    test_compile("const fn=async()=>42;fn();", "async_arrow");
}

#[test]
fn min_await() {
    test_compile(
        "async function run(){const x=await fetchData();return x;}",
        "await",
    );
}

// ============================================
// SECTION 25: NEW - Comma operator (minified)
// ============================================

#[test]
fn min_comma_operator() {
    test_compile("let a=0;let b=0;a=1,b=2;", "comma_operator");
}

#[test]
fn min_comma_side_effect() {
    test_compile("let x=0;x++,x=x*2;", "comma_side_effect");
}

// ============================================
// SECTION 26: NEW - Full complex programs
// ============================================

#[test]
fn min_full_minified() {
    test_compile(
        "const x=42;const y=\"hello\";function add(a,b){return a+b;}if(x>10){add(x,1);}else{0;}",
        "full_minified",
    );
}

#[test]
fn min_advanced_chaining() {
    test_compile("const arr=[1,2,3,4,5];const result=arr.filter(x=>x%2===0).map(x=>x*x).reduce((a,b)=>a+b,0);", "advanced_chaining");
}

#[test]
fn min_class_with_methods() {
    test_compile("class Counter{constructor(){this.count=0;}increment(){this.count=this.count+1;}getCount(){return this.count;}}const c=new Counter();c.increment();c.increment();c.getCount();", "class_with_methods");
}

#[test]
fn min_mixed_features() {
    test_compile("function process(items){return items.filter(x=>x>0).map(x=>x*2).reduce((a,b)=>a+b,0);}process([1,-2,3,-4,5]);", "mixed_features");
}

#[test]
fn min_nested_closures() {
    test_compile(
        "function outer(a){return function(b){return function(c){return a+b+c;};};}outer(1)(2)(3);",
        "nested_closures",
    );
}

#[test]
fn min_bitwise_operations() {
    test_compile(
        "const mask=255;const flags=10;const combined=flags&mask;const toggled=flags^5;",
        "bitwise_operations",
    );
}

#[test]
fn min_shift_operations() {
    test_compile("const x=1;const y=x<<10;const z=y>>5;", "shift_operations");
}

#[test]
fn min_unary_operations() {
    test_compile(
        "const a=+\"42\";const b=-3;const c=~0;const d=!false;",
        "unary_operations",
    );
}

#[test]
fn min_complex_ternary() {
    test_compile("function grade(score){return score>=90?\"A\":score>=80?\"B\":score>=70?\"C\":\"F\";}grade(85);", "complex_ternary");
}

#[test]
fn min_fibonacci_arrow() {
    test_compile(
        "function fib(n){return n<=1?n:fib(n-1)+fib(n-2);}fib(10);",
        "fibonacci_arrow",
    );
}
