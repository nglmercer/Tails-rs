fn main() {
    let mut runtime = tails::TailsRuntime::default();
    let result =
        runtime.eval(r#"const key = "" + (1 + 2); const obj = { [key]: "three" }; obj["3"];"#);
    println!("{:?}", result);
}
