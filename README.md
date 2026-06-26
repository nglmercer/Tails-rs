# Tails-rs

A TypeScript-first runtime implemented in Rust.

## Overview

Tails-rs is a JavaScript/TypeScript runtime built from scratch in Rust. It features a bytecode compiler, stack-based virtual machine, and supports many modern JavaScript features including classes, promises, async/await, and ES modules.

## Features

- **Bytecode Compilation**: Source code is compiled to bytecode for efficient execution
- **Virtual Machine**: Stack-based interpreter with garbage collection
- **TypeScript/JavaScript Syntax**: Supports modern JS/TS features
- **Classes**: Full support for class declarations, inheritance, getters/setters, and static methods
- **Promises**: Native Promise implementation with `.then()`, `.catch()`, `.finally()`
- **Async/Await**: Async operations with awaited promises
- **ES Modules**: Import/export syntax with support for named, default, and namespace imports
- **Error Handling**: Try/catch/finally with proper exception handling
- **Proxy Objects**: JavaScript Proxy API support
- **Objects & Arrays**: Native object and array operations

## Installation

```bash
cargo build --release
```

## Usage

### CLI

```bash
./target/release/tails script.ts
```

### As a Library

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

## Supported Features

### Basic Operations
- Arithmetic: `+`, `-`, `*`, `/`, `%`, `**`
- Comparison: `==`, `===`, `!=`, `!==`, `<`, `>`, `<=`, `>=`
- Logical: `&&`, `||`, `!`, `typeof`, `void`
- Variables: `let`, `const`, `var`

### Control Flow
- Conditional: `if`/`else`
- Loops: `while`
- Functions: declarations, expressions, closures

### Classes
- Class declarations and expressions
- Constructor and instance methods
- Static methods
- Getters and setters
- Inheritance with `extends` and `super`

### Async
- `Promise` constructor, `resolve`, `reject`, `all`
- `.then()`, `.catch()`, `.finally()` chaining
- `await` operator
- `setTimeout`

### Modules
- `import` / `export` statements
- Named, default, and namespace imports
- Module registry for dependency management

## Architecture

```
src/
├── compiler/     # Lexer, parser, bytecode generator, type checker
├── vm/           # Virtual machine with interpreter and GC
├── runtime_env/  # Native functions and async runtime
├── objects/      # JS value types (objects, arrays, functions, promises)
└── ffi/          # Foreign function interface
```

## Testing

```bash
cargo test
```

## License

MIT