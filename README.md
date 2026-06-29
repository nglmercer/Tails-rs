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
- **Modular Native Modules** — Feature-gated crates under `modules/`, importable by bare name
- **Error Handling** — Try/catch/finally with thrown exceptions
- **Proxy Objects** — JavaScript Proxy API with traps
- **Rich Standard Library** — Object, Array, String, Math, JSON, Number, and global functions

## Installation

```bash
cargo build --release
```

### Build with Feature Flags

Native modules (`fs`, `path`, `process`, `os`) are compiled as optional Cargo features, all enabled by default.

```bash
# All modules (default)
cargo build --release

# Without fs and path (smaller binary)
cargo build --release --no-default-features

# Only fs and path
cargo build --release --no-default-features -F fs -F path

# Everything except os
cargo build --release -F --no-default-features -F fs -F path -F process
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

## Native Modules

Native modules are imported by bare name. They are **not** available as globals — you must always import them explicitly.

```typescript
import fs from "fs";
import path from "path";
import process from "process";
import os from "os";
import Buffer from "./buffer.native";
import Intl from "./intl.native";
import events from "./events.native";
import crypto from "./crypto.native";
```

The `.native` extension still works for all modules:

```typescript
import fs from "./fs.native";
import path from "./path.native";
```

### Available Native Modules

| Module | Feature | Crate | Description |
|--------|---------|-------|-------------|
| `fs` | `fs` | `modules/fs` | File system operations (read, write, stat, mkdir, etc.) |
| `path` | `path` | `modules/path` | Path manipulation (join, resolve, basename, etc.) |
| `process` | `process` | `modules/process` | Process info and control (env, argv, exit, etc.) |
| `os` | `os` | `modules/os` | OS information (platform, arch, cpus, memory, etc.) |
| `buffer` | *(always)* | *(built-in)* | Node.js-compatible binary data handling |
| `intl` | *(always)* | *(built-in)* | Internationalization (DateTimeFormat, NumberFormat) |
| `events` | *(always)* | *(built-in)* | EventEmitter class with on/emit/off |
| `crypto` | *(always)* | *(built-in)* | Cryptographic functions (randomBytes, randomUUID, createHash) |

### Module Architecture

Each feature-gated module is split into two layers:

- **`modules/<name>/`** — Pure Rust implementation with no dependency on the runtime. Contains the actual business logic (fs operations, path manipulation, etc.)
- **`src/runtime_env/native_fns/<name>_fns.rs`** — Thin adapter that converts between runtime `Value` types and the pure module functions

This separation keeps the core runtime lightweight and the module logic testable independently.

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
- Error subclasses with real stack traces (`Error`, `TypeError`, `ReferenceError`, `SyntaxError`, `RangeError`)

### Global Functions
- `parseInt()`, `parseFloat()`
- `isNaN()`, `isFinite()`
- `Number.parseInt()`, `Number.parseFloat()`
- `Number.isNaN()`, `Number.isFinite()`
- `atob()`, `btoa()` — Base64 encoding/decoding

### Encoding
- `atob()` / `btoa()` — Base64 encoding/decoding

### Buffer (native module)
- `Buffer.alloc()`, `Buffer.from()`, `Buffer.concat()`, `Buffer.isBuffer()`, `Buffer.byteLength()`
- Instance: `toString()`, `write()`, `slice()`, `copy()`, `fill()`, `compare()`, `equals()`, `indexOf()`

### process (native module)
- `process.platform`, `process.arch`, `process.pid`
- `process.cwd()`, `process.chdir()`
- `process.env`, `process.argv`
- `process.exit()`, `process.stdout.write()`, `process.stderr.write()`
- `process.hrtime()`, `process.nextTick()`

### Intl (native module)
- `Intl.DateTimeFormat` — Date/time formatting with `format()` and `formatToParts()`
- `Intl.NumberFormat` — Number formatting with decimal, currency, and percent styles

### Destructuring & Spread
- Array destructuring with skipping
- Object destructuring with aliasing
- Array spread operator (`...`)

### Other
- **Proxy** objects with handlers
- **ES Modules**: `import` / `export` (named, default, namespace)
- **Reflect** API with native implementations
- `in` operator, `instanceof` operator
- **Optional chaining** (`?.`) and **nullish coalescing** (`??`)
- **Type annotations** (TypeScript)
- **Typed Arrays**: `Int8Array`, `Uint8Array`, `Float32Array`, etc. with full API
- **ES6+ Collections**: `Map`, `Set`, `WeakMap`, `WeakSet` with full API
- **Symbol**: `Symbol()`, `Symbol.for()`, `Symbol.keyFor()`, well-known symbols (`Symbol.iterator`, `Symbol.toStringTag`, `Symbol.asyncIterator`, etc.)
- **for...of loop**: Iterator protocol with `Symbol.iterator`, built-in array/string iterators
- **for await...of**: Async iteration with `Symbol.asyncIterator`, automatic promise resolution
- **Iterator helpers**: `map()`, `filter()`, `take()`, `drop()`, `forEach()`, `toArray()` with chaining
- **Function.prototype**: `.bind()`, `.call()`, `.apply()`
- **Object methods**: `Object.is()`, `Object.freeze()`, `Object.seal()`, `Object.isExtensible()`, `Object.preventExtensions()`, `Object.isFrozen()`, `Object.isSealed()`
- **Promise enhancements**: `Promise.allSettled()`, `Promise.any()`, `Promise.withResolvers()`
- **Array enhancements**: `copyWithin()`, `fill()`, `findLast()`, `findLastIndex()`, `flatMap()`, `lastIndexOf()`, `Array.isArray()`, `Array.from()`, `Array.of()`
- **Reflect API**: `Reflect.get()`, `Reflect.set()`, `Reflect.apply()`, `Reflect.construct()`, `Reflect.isExtensible()`, `Reflect.preventExtensions()`, etc.
- **BigInt**: Full primitive type with `42n` literals, arithmetic, comparison, `BigInt()` constructor
- **Date**: `new Date()`, getters/setters, ISO parsing, `Date.now()`, `Date.parse()`, `Date.UTC()`
- **RegExp**: `new RegExp()`, `test()`, `exec()`, flags (`g`, `i`, `m`, `s`, `u`, `y`), `String.prototype.match/replace/search`

## Roadmap

> Based on current implementation status. Contributions welcome!

### Recently Completed
- **Modular Native Modules** — `fs`, `path`, `process`, `os` extracted to standalone crates under `modules/` with Cargo feature flags for selective inclusion
- **Bare-name Imports** — `import fs from "fs"` works alongside the legacy `import fs from "./fs.native"` syntax
- **Light Runtime** — Moved process, Buffer, Intl from globals to import-only native modules
- **Native Module System** — Import-only modules via `.native` extension, registry-based loader
- **events module** — EventEmitter class with `on()`, `emit()`, `off()`, `listenerCount()`
- **os module** — OS info: `platform()`, `arch()`, `cpus()`, `totalmem()`, `freemem()`, `uptime()`, `hostname()`, `type()`, `release()`, `homedir()`, `tmpdir()`
- **crypto module** — `randomBytes()`, `randomUUID()`, `createHash()` (SHA-224/256/384/512)
- **process module** — `process.platform`, `process.arch`, `process.pid`, `process.cwd()`, `process.chdir()`, `process.env`, `process.argv`, `process.exit()`, `process.stdout.write()`, `process.hrtime()`, `process.nextTick()`
- **buffer module** — `Buffer.alloc()`, `Buffer.from()`, `Buffer.concat()`, `Buffer.isBuffer()`, `Buffer.byteLength()`, `toString()`, `write()`, `slice()`, `copy()`, `fill()`, `compare()`, `equals()`, `indexOf()`
- **intl module** — `Intl.DateTimeFormat` with `format()` and `formatToParts()`, `Intl.NumberFormat` with currency and percent styles
- **Import-only fs/path** — `fs` and `path` removed from globals, now require explicit import
- **Reflect API** — Native implementations for `get`, `set`, `apply`, `construct`, `isExtensible`, `preventExtensions`, etc.
- **Generators** — Runtime support for `function*`, `yield`, and `.next()`
- **for...of loop** — Iterator protocol execution with `Symbol.iterator`
- **Symbol** type and well-known symbols (`Symbol.iterator`, `Symbol.toStringTag`, `Symbol.hasInstance`, `Symbol.asyncIterator`, etc.)
- **Function prototypes** — `Function.prototype.bind()`, `.call()`, `.apply()`
- **Array enhancements** — `copyWithin`, `fill`, `findLast`, `findLastIndex`, `flatMap`, `lastIndexOf`, `Array.isArray()`, `Array.from()`, `Array.of()`
- **Object methods** — `Object.is()`, `Object.seal()`, `Object.isSealed()`, `Object.freeze()`, `Object.isFrozen()`, `Object.isExtensible()`, `Object.preventExtensions()`
- **Promise enhancements** — `Promise.any()`, `Promise.allSettled()`, `Promise.withResolvers()`
- **BigInt** — Full primitive type with literals (`42n`), arithmetic, comparison, `BigInt()` constructor
- **Date** — Full implementation with `new Date()`, getters/setters, ISO string parsing, `Date.now()`, `Date.parse()`, `Date.UTC()`
- **RegExp** — Full implementation with `new RegExp()`, `test()`, `exec()`, flags support, `String.prototype.match/replace/search`
- **Iterator helpers** — `map()`, `filter()`, `take()`, `drop()`, `forEach()`, `toArray()` on array/string iterators, with chaining support
- **for await...of** — Async iteration with `Symbol.asyncIterator` support, automatic promise resolution
- **Error stack traces** — Real stack traces with function names for `Error`, `TypeError`, `ReferenceError`, `SyntaxError`, `RangeError`
- **Encoding** — `atob()` and `btoa()` for base64 encoding/decoding
- **path module** — `path.join()`, `path.resolve()`, `path.basename()`, `path.dirname()`, `path.extname()`, `path.relative()`, `path.isAbsolute()`, `path.normalize()`, `path.sep`, `path.delimiter`
- **fs module** — `fs.readFileSync()`, `fs.writeFileSync()`, `fs.existsSync()`, `fs.mkdirSync()`, `fs.readdirSync()`, `fs.statSync()`, `fs.unlinkSync()`, `fs.rmSync()`, `fs.copyFileSync()`, `fs.renameSync()`, `fs.appendFileSync()`

### Future / Research
- **More Native Modules**
  - `url` — URL parsing and manipulation
  - `stream` — Stream processing
  - `http` / `https` — HTTP client/server
  - `child_process` — Process spawning
- **Web APIs & FFI**
  - `fetch` and `Response`/`Request` types
  - Enhanced FFI for Rust interop
- **Performance**
  - JIT compilation tier
  - Optimized GC (generational, concurrent)
- **Node.js Compatibility**
  - Additional core modules (url, crypto, stream, etc.)

## Architecture

```
src/
├── compiler/      # Lexer, parser, bytecode generator, type checker
├── vm/            # Virtual machine with interpreter and GC
│   └── interpreter/
│       ├── native_loader.rs  # Registry-based native module loader
│       └── ...
├── runtime_env/   # Native function adapters and async runtime
├── objects/       # JS value types (objects, arrays, functions, promises, proxies)
├── ffi/           # Foreign function interface
└── main.rs        # CLI entry point

modules/
├── abi/           # Shared ABI types for future dlopen support
├── fs/            # Pure Rust fs operations (feature-gated)
├── path/          # Pure Rust path operations (feature-gated)
├── process/       # Pure Rust process operations (feature-gated)
└── os/            # Pure Rust os operations (feature-gated)
```

## Testing

```bash
# Run all tests (default features)
cargo test

# Run without optional modules
cargo test --no-default-features

# Run with specific features only
cargo test --no-default-features -F fs -F path
```

## License

MIT
