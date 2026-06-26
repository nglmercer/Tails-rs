use tails::compiler::Compiler;

#[test]
fn debug_compile_counter() {
    let source = r#"
let count = 0;
export function increment() {
    count = count + 1;
    return count;
}
export function getCount() {
    return count;
}
"#;
    let compiler = Compiler::new(false);
    let compiled = compiler.compile(source).unwrap();
    
    eprintln!("=== Instructions ({} total) ===", compiled.instructions.len());
    for (i, inst) in compiled.instructions.iter().enumerate() {
        eprintln!("  {:3}: {:?}", i, inst);
    }
    eprintln!("\n=== Constants ===");
    for (i, c) in compiled.constants.iter().enumerate() {
        eprintln!("  {:3}: {:?}", i, c);
    }
    eprintln!("\n=== Functions ===");
    for (i, f) in compiled.functions.iter().enumerate() {
        eprintln!("  {:3}: {:?}", i, f);
    }
}
