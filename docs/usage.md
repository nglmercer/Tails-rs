## CLI

```bash
# Run a script
./target/release/tails script.ts

# Watch for changes
cargo run --bin tails -- --watch examples/all_features.ts
```

## As a Library

```rust
use tails::{TailsRuntime, Value};

let mut runtime = TailsRuntime::default();
let result = runtime.eval("
    class Calc {
        add(a, b) { return a + b; }
    }
    new Calc().add(3, 7);
")?;
```
