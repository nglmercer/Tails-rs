# Tails-rs

A TypeScript-first runtime implemented in Rust.

## Overview

Tails-rs is a JavaScript/TypeScript runtime built from scratch in Rust. It compiles source code to bytecode and executes it on a stack-based virtual machine with garbage collection. It supports many modern JavaScript features including classes, promises, async/await, ES modules, and more.

## Features

- **Bytecode Compiler** — Source code is compiled to bytecode for efficient execution
- **Stack-based VM** — Interpreter with a mark-and-sweep garbage collector
- **Modern Syntax** — ES2020+ features: arrow functions, destructuring, spread, template literals, etc.
- **Classes** — Declarations, expressions, inheritance, getters/setters, and static methods
- **Promises** — Native implementation with `.then()`, `.catch()`, `.finally()`, and `Promise.all`
- **Timers** — `setTimeout`, `setInterval`, and `clearInterval`
- **ES Modules** — Import/export with named, default, and namespace imports
- **Error Handling** — Try/catch/finally with thrown exceptions
- **Proxy Objects** — JavaScript Proxy API with traps
- **Rich Standard Library** — Object, Array, String, Math, JSON, Number, and global functions

## Installation

```bash
cargo build --release
```

## Usage

### CLI

```bash
# Run a script
./target/release/tails script.ts

# Watch for changes
cargo run --bin tails -- --watch examples/all_features.ts
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

## Quick Start

See a live demonstration of all supported features:

```bash
cargo run --bin tails -- examples/all_features.ts
```

## Supported Features

### Variables & Types
- **Declaration**: `let`, `const`, `var`
- **Primitives**: `number`, `string`, `boolean`, `undefined`, `null`
- **Operators**: Arithmetic (`+`, `-`, `*`, `/`, `%`, `**`), compound assignment (`+=`, `-=`, etc.), comparison (`==`, `===`, `!=`, `!==`, `<`, `>`, `<=`, `>=`), logical (`&&`, `||`, `!`), bitwise (`~`)
- **Increment/Decrement**: `++`, `--`
- **Type Inspection**: `typeof`, `void`

### Control Flow
- **Conditionals**: `if` / `else if` / `else`, ternary `? :`
- **Loops**: `for`, `while`, `do...while`, `for...in`
- **Jump**: `break`, `continue`, `switch` / `case` / `default`

### Functions
- Declarations and expressions
- Arrow functions (with and without braces)
- Closures and lexical scoping
- Higher-order functions

### Classes (OOP)
- Class declarations and expressions
- Constructors and instance methods
- Static methods
- Getters and setters
- Inheritance with `extends` and `super`
- `instanceof` operator

### Objects & Arrays

**Object methods**
- `Object.keys()`, `Object.values()`, `Object.entries()`
- `Object.assign()`

**Array methods**
- Mutation: `push()`, `pop()`, `shift()`, `unshift()`, `splice()`
- Iteration: `map()`, `filter()`, `reduce()`, `forEach()`, `find()`, `findIndex()`
- Inspection: `some()`, `every()`, `indexOf()`, `includes()`
- Transformation: `join()`, `reverse()`, `sort()`, `concat()`, `slice()`, `flat()`

### Strings
- `charAt()`, `charCodeAt()`
- `slice()`, `substring()`
- `indexOf()`, `includes()`
- `replace()`, `split()`, `trim()`
- Case conversion: `toLowerCase()`, `toUpperCase()`
- Testing: `startsWith()`, `endsWith()`
- Padding: `padStart()`, `padEnd()`, `repeat()`

### Math
- Constants: `Math.PI`, `Math.E`
- Functions: `abs()`, `floor()`, `ceil()`, `round()`, `min()`, `max()`, `pow()`, `sqrt()`, `log()`, `sin()`, `cos()`, `tan()`
- `Math.random()`

### JSON
- `JSON.stringify()`
- `JSON.parse()`

### Promise & Async
- `Promise` constructor, `resolve`, `reject`
- `.then()`, `.catch()`, `.finally()`
- `Promise.all()`
- `await` operator
- Timers: `setTimeout()`, `setInterval()`, `clearInterval()`

### Error Handling
- `try` / `catch` / `finally`
- `throw` with any value

### Global Functions
- `parseInt()`, `parseFloat()`
- `isNaN()`, `isFinite()`
- `Number.parseInt()`, `Number.parseFloat()`
- `Number.isNaN()`, `Number.isFinite()`

### Destructuring & Spread
- Array destructuring with skipping
- Object destructuring with aliasing
- Array spread operator (`...`)

### Other
- **Proxy** objects with handlers
- **ES Modules**: `import` / `export` (named, default, namespace)
- **Reflect** API
- `in` operator, `instanceof` operator

## Architecture

```
src/
├── compiler/      # Lexer, parser, bytecode generator, type checker
├── vm/            # Virtual machine with interpreter and GC
├── runtime_env/   # Native functions and async runtime
├── objects/       # JS value types (objects, arrays, functions, promises, proxies)
├── ffi/           # Foreign function interface
└── main.rs        # CLI entry point
```

## Testing

```bash
cargo test
```

## License

MIT
