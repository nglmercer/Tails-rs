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
- Error subclasses with real stack traces (`Error`, `TypeError`, `ReferenceError`, `SyntaxError`, `RangeError`)

### Global Functions
- `parseInt()`, `parseFloat()`
- `isNaN()`, `isFinite()`
- `Number.parseInt()`, `Number.parseFloat()`
- `Number.isNaN()`, `Number.isFinite()`
- `atob()`, `btoa()` — Base64 encoding/decoding

### Encoding & Buffer
- `atob()` / `btoa()` — Base64 encoding/decoding
- `Buffer` — Node.js-compatible binary data handling
  - Static: `Buffer.alloc()`, `Buffer.from()`, `Buffer.concat()`, `Buffer.isBuffer()`, `Buffer.byteLength()`
  - Instance: `toString()`, `write()`, `slice()`, `copy()`, `fill()`, `compare()`, `equals()`, `indexOf()`, `length`

### process Global
- `process.platform`, `process.arch`, `process.pid`
- `process.cwd()`, `process.chdir()`
- `process.env`, `process.argv`
- `process.exit()`, `process.stdout.write()`, `process.stderr.write()`
- `process.hrtime()`, `process.nextTick()`

### Intl APIs
- `Intl.DateTimeFormat` — Date/time formatting with `format()` and `formatToParts()`
- `Intl.NumberFormat` — Number formatting with decimal, currency, and percent styles

### Path Module
- `path.join()` — Join path segments
- `path.resolve()` — Resolve to absolute path
- `path.basename()` — Get filename from path
- `path.dirname()` — Get directory from path
- `path.extname()` — Get file extension
- `path.relative()` — Get relative path between two paths
- `path.isAbsolute()` — Check if path is absolute
- `path.normalize()` — Normalize path (resolve `.` and `..`)
- `path.sep` — Path separator (`/` or `\`)
- `path.delimiter` — Path delimiter (`:` or `;`)

### File System (fs) Module
- `fs.readFileSync()` — Read file contents synchronously
- `fs.writeFileSync()` — Write data to file synchronously
- `fs.existsSync()` — Check if file/directory exists
- `fs.mkdirSync()` — Create directory (with `recursive` option)
- `fs.readdirSync()` — List directory contents
- `fs.statSync()` — Get file metadata (size, isFile, isDirectory, mtimeMs)
- `fs.unlinkSync()` — Delete a file
- `fs.rmSync()` — Remove file or directory (with `recursive` option)
- `fs.copyFileSync()` — Copy a file
- `fs.renameSync()` — Rename/move a file
- `fs.appendFileSync()` — Append data to a file

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

### ✅ Recently Completed
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
- **Buffer** — Node.js-compatible `Buffer` class with `alloc()`, `from()`, `concat()`, `isBuffer()`, `toString()`, `write()`, `slice()`, `copy()`, `fill()`, `compare()`, `equals()`, `indexOf()`
- **process global** — `process.platform`, `process.arch`, `process.pid`, `process.cwd()`, `process.chdir()`, `process.env`, `process.argv`, `process.exit()`, `process.stdout.write()`, `process.hrtime()`, `process.nextTick()`
- **Intl APIs** — `Intl.DateTimeFormat` with `format()` and `formatToParts()`, `Intl.NumberFormat` with currency and percent styles
- **path module** — `path.join()`, `path.resolve()`, `path.basename()`, `path.dirname()`, `path.extname()`, `path.relative()`, `path.isAbsolute()`, `path.normalize()`, `path.sep`, `path.delimiter`
- **fs module** — `fs.readFileSync()`, `fs.writeFileSync()`, `fs.existsSync()`, `fs.mkdirSync()`, `fs.readdirSync()`, `fs.statSync()`, `fs.unlinkSync()`, `fs.rmSync()`, `fs.copyFileSync()`, `fs.renameSync()`, `fs.appendFileSync()`

### 🔮 Future / Research
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
