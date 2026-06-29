## Variables & Types
- **Declaration**: `let`, `const`, `var`
- **Primitives**: `number`, `string`, `boolean`, `undefined`, `null`
- **Operators**: Arithmetic (`+`, `-`, `*`, `/`, `%`, `**`), compound assignment (`+=`, `-=`, etc.), comparison (`==`, `===`, `!=`, `!==`, `<`, `>`, `<=`, `>=`), logical (`&&`, `||`, `!`), bitwise (`~`)
- **Increment/Decrement**: `++`, `--`
- **Type Inspection**: `typeof`, `void`

## Control Flow
- **Conditionals**: `if` / `else if` / `else`, ternary `? :`
- **Loops**: `for`, `while`, `do...while`, `for...in`
- **Jump**: `break`, `continue`, `switch` / `case` / `default`

## Functions
- Declarations and expressions
- Arrow functions (with and without braces)
- Closures and lexical scoping
- Higher-order functions

## Classes (OOP)
- Class declarations and expressions
- Constructors and instance methods
- Static methods
- Getters and setters
- Inheritance with `extends` and `super`
- `instanceof` operator

## Objects & Arrays

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

## Strings
- `charAt()`, `charCodeAt()`
- `slice()`, `substring()`
- `indexOf()`, `includes()`
- `replace()`, `split()`, `trim()`
- Case conversion: `toLowerCase()`, `toUpperCase()`
- Testing: `startsWith()`, `endsWith()`
- Padding: `padStart()`, `padEnd()`, `repeat()`

## Math
- Constants: `Math.PI`, `Math.E`
- Functions: `abs()`, `floor()`, `ceil()`, `round()`, `min()`, `max()`, `pow()`, `sqrt()`, `log()`, `sin()`, `cos()`, `tan()`
- `Math.random()`

## JSON
- `JSON.stringify()`
- `JSON.parse()`

## Promise & Async
- `Promise` constructor, `resolve`, `reject`
- `.then()`, `.catch()`, `.finally()`
- `Promise.all()`
- `await` operator
- Timers: `setTimeout()`, `setInterval()`, `clearInterval()`

## Error Handling
- `try` / `catch` / `finally`
- `throw` with any value
- Error subclasses with real stack traces (`Error`, `TypeError`, `ReferenceError`, `SyntaxError`, `RangeError`)

## Global Functions
- `parseInt()`, `parseFloat()`
- `isNaN()`, `isFinite()`
- `Number.parseInt()`, `Number.parseFloat()`
- `Number.isNaN()`, `Number.isFinite()`
- `atob()`, `btoa()` — Base64 encoding/decoding

## Encoding
- `atob()` / `btoa()` — Base64 encoding/decoding

## Buffer (native module)
- `Buffer.alloc()`, `Buffer.from()`, `Buffer.concat()`, `Buffer.isBuffer()`, `Buffer.byteLength()`
- Instance: `toString()`, `write()`, `slice()`, `copy()`, `fill()`, `compare()`, `equals()`, `indexOf()`

## process (native module)
- `process.platform`, `process.arch`, `process.pid`
- `process.cwd()`, `process.chdir()`
- `process.env`, `process.argv`
- `process.exit()`, `process.stdout.write()`, `process.stderr.write()`
- `process.hrtime()`, `process.nextTick()`

## Intl (native module)
- `Intl.DateTimeFormat` — Date/time formatting with `format()` and `formatToParts()`
- `Intl.NumberFormat` — Number formatting with decimal, currency, and percent styles

## Destructuring & Spread
- Array destructuring with skipping
- Object destructuring with aliasing
- Array spread operator (`...`)

## Other
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
