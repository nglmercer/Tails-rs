fn main() {
    let mut runtime = tails::TailsRuntime::default();
    let result = runtime.eval("1 << 3;");
    println!("Result: {:?}", result);
}
