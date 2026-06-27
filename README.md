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

**Typed Arrays**
- Constructors: `Int8Array`, `Uint8Array`, `Uint8ClampedArray`, `Int16Array`, `Uint16Array`, `Int32Array`, `Uint32Array`, `Float32Array`, `Float64Array`, `BigInt64Array`, `BigUint64Array`
- Static methods: `from()`, `of()`
- Instance methods: `get()`, `set()`, `subarray()`, `slice()`
- Properties: `length`, `byteLength`, `byteOffset`, `BYTES_PER_ELEMENT`

**ES6+ Collections**
- **Map**: `new Map()`, `get()`, `set()`, `has()`, `delete()`, `clear()`, `forEach()`, `keys()`, `values()`, `entries()`, `size`
- **Set**: `new Set()`, `add()`, `has()`, `delete()`, `clear()`, `forEach()`, `keys()`, `values()`, `entries()`, `size`
- **WeakMap**: `new WeakMap()`, `get()`, `set()`, `has()`, `delete()`
- **WeakSet**: `new WeakSet()`, `add()`, `has()`, `delete()`

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
- **Reflect** API (static methods stubbed—needs implementation)
- `in` operator, `instanceof` operator
- **Optional chaining** (`?.`) and **nullish coalescing** (`??`)
- **Type annotations** (TypeScript)
- **Typed Arrays**: `Int8Array`, `Uint8Array`, `Float32Array`, etc. with full API
- **ES6+ Collections**: `Map`, `Set`, `WeakMap`, `WeakSet` with full API

## Roadmap

> Based on current implementation status. Contributions welcome!

### 🚧 In Progress / Next Up
- **Reflect API** — Native implementations for `get`, `set`, `apply`, `construct`, etc.
- **Generators** — Runtime support for `function*`, `yield`, and `.next()`
- **for...of loop** — Iterator protocol execution

### 📅 Near-Term Goals
- **Symbol** type and `Symbol.iterator`
- **Function prototypes**
  - `Function.prototype.bind()`, `.call()`, `.apply()`
- **Array enhancements**
  - `copyWithin`, `fill`, `findLast`, `findLastIndex`, `flatMap`, `lastIndexOf`
  - Static methods: `Array.isArray()`, `Array.from()`, `Array.of()`
- **Object methods**
  - `Object.is()`, `Object.seal()`, `Object.isExtensible()`, `Object.preventExtensions()` (full API)
- **Promise enhancements**
  - `Promise.any()`, `Promise.allSettled()`, `Promise.race()` (complete)

### 🔮 Future / Research
- **Iteration & Generators**
  - Async generators and `for await...of`
  - Iterator helpers (`map`, `filter`, `take`, `drop` on iterables)
- **Built-in Objects**
  - `Date` and `RegExp` (full implementation)
  - `BigInt` primitive type
  - `Error` subclasses (`TypeError`, `ReferenceError`, `SyntaxError`, `RangeError`)
- **Encoding & Intl**
  - `atob()` / `btoa()`
  - Basic Intl APIs (`Intl.DateTimeFormat`, `Intl.NumberFormat`)
- **Web APIs & FFI**
  - `fetch` and `Response`/`Request` types
  - Enhanced FFI for Rust interop
- **Performance**
  - JIT compilation tier
  - Optimized GC (generational, concurrent)
- **Node.js Compatibility**
  - `Buffer` and `Uint8Array` parity for Node-style APIs
  - `process` global and core modules

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
