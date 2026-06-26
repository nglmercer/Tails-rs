use tails::TailsRuntime;

fn main() {
    let mut rt = TailsRuntime::default();

    // Test: Promise with resolved value
    let r = rt.eval(
        r#"
        new Promise(function(resolve) { resolve(42); }).then(function(v) { return v; });
    "#,
    );
    println!("Promise resolve: {:?}", r);

    // Test: Promise.resolve
    let r2 = rt.eval(
        r#"
        Promise.resolve(42).then(function(v) { return v; });
    "#,
    );
    println!("Promise.resolve: {:?}", r2);
}
