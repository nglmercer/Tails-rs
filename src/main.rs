use anyhow::Result;
use tails::TailsRuntime;

fn main() -> Result<()> {
    env_logger::init();
    
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: tails <script.ts>");
        std::process::exit(1);
    }
    
    let path = std::path::Path::new(&args[1]);
    if !path.exists() {
        eprintln!("Error: File '{}' not found", args[1]);
        std::process::exit(1);
    }
    
    let source = std::fs::read_to_string(path)?;
    
    let mut runtime = TailsRuntime::default()?;
    let result = runtime.eval(&source)?;
    
    println!("{:?}", result);
    
    Ok(())
}
