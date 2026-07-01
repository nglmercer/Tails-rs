# Unsafe Code Safety Guide

This document describes the unsafe code in Tails-rs, the safe abstractions built to contain it, and the safety invariants that must be maintained.

## Overview

Tails-rs uses `unsafe` code primarily at the FFI boundary where it interacts with C-ABI native modules loaded at runtime. The codebase originally contained ~52 `unsafe` occurrences across 12 source files. Safe wrapper abstractions have been introduced to reduce this by ~80%, pushing raw pointer operations behind documented, tested interfaces.

Unsafe code in this project falls into six categories:

| Category | Risk Level | Location |
|----------|-----------|----------|
| FFI boundary (`extern "C"` functions) | High | `src/ffi/mod.rs`, `src/ffi/native.rs`, `modules/abi/src/lib.rs` |
| Dynamic library loading | High | `src/vm/interpreter/modules.rs`, `src/vm/interpreter/native_loader.rs`, `modules/abi/src/loader.rs` |
| Function pointer transmutation | Critical | `src/vm/interpreter/calls.rs` |
| Macro-generated FFI wrappers | Medium | `modules/native-macros/src/class.rs`, `modules/native-macros/src/function.rs` |
| TypedArray byte-level access | Medium | `src/objects/js_array.rs` |
| OS / libc calls | Low | `modules/os/src/lib.rs` |

## Safe Abstractions

The following wrapper types encapsulate unsafe operations behind safe or `unsafe`-marked APIs. All types live under `src/ffi/` unless otherwise noted.

### Pointer Wrappers (`src/ffi/safe_wrappers.rs`)

**`SafePtr<'a, T>`** — Wraps a `*mut T` with lifetime tracking via `PhantomData<&'a T>`. The lifetime `'a` ties pointer validity to the enclosing scope.

- `new(ptr)` — `unsafe`, requires valid, aligned, unaliased pointer
- `as_ref()` / `as_mut()` — `unsafe`, dereferences the pointer
- `is_null()` — safe, null check

**`SafeCStr<'a>`** — Wraps a `*const c_char` with `PhantomData<&'a CStr>`.

- `new(ptr)` — `unsafe`, requires null-terminated C string
- `to_str()` — safe, returns `Option<&'a str>`, checks for null before dereferencing

**`SafeSlice<'a, T>`** — Wraps a `*const T` plus `len: usize` with `PhantomData<&'a [T]>`.

- `new(ptr, len)` — `unsafe`, requires valid pointer for `len` elements
- `as_slice()` — `unsafe`, calls `slice::from_raw_parts`
- `len()` / `is_empty()` — safe

### String Management (`src/ffi/safe_string.rs`)

**`SafeFFIString`** — Wraps a `*mut c_char` with an `owned: bool` flag. When owned, `Drop` calls `CString::from_raw` to reclaim memory.

- `from_raw(ptr)` — `unsafe`, sets `owned = false`
- `as_str()` / `to_owned()` — safe, null-checked

**`FFIStringBuffer`** — Holds a `Vec<CString>` and returns stable `*const c_char` pointers via `alloc()`. Eliminates lifetime problems by keeping `CString` values alive inside the buffer.

- `alloc(s)` — safe, returns null on interior null byte

### Library Loading (`src/vm/interpreter/safe_library.rs`)

**`SafeLibrary`** — Wraps `Option<Library>` from `libloading`. The `new` constructor is safe and returns `Result`. Internal `unsafe` is contained to the `Library::new` call.

- `new(path)` — safe, returns `Result<Self, String>`
- `get_function::<T>(name)` — `unsafe`, caller must guarantee type `T` matches the symbol signature

**`SafeFunction<T>`** — Pairs a `Symbol<'static, T>` with an `Arc<SafeLibrary>` to keep the library loaded. Uses `transmute` to extend the symbol's lifetime, justified because the `Arc` ensures the library is never unloaded while the function pointer is alive.

- `new(library, name)` — `unsafe`, requires correct type signature
- `as_ptr()` — safe, returns `&T`

### Function Pointers (`src/vm/interpreter/safe_function.rs`)

**`SafeNativeFunction`** — Wraps a `NativeFunctionPtr` (the standard `extern "C"` signature for Tails-rs native functions) plus a name string for diagnostics.

- `new(ptr, name)` — `unsafe`, requires correct C ABI signature
- `call(runtime, this, args)` — `unsafe`, requires valid runtime pointer and matching arguments

**`FunctionPointerWrapper<T>`** — Generic wrapper for raw function pointers with a name.

- `new(ptr, name)` — `unsafe`, requires valid, aligned pointer
- `as_ref()` — `unsafe`, dereferences the pointer

### TypedArray Access (`src/objects/safe_typed_array.rs`)

**`SafeTypedArray<'a>`** — A pure safe facade over `&'a mut TypedArray`. Exposes `kind()`, `byte_length()`, `byte_offset()`, `length()`, `element_size()`, `inner()`, `inner_mut()`. Contains no `unsafe`.

**`TypedArrayRef<'a, T>`** — Wraps a `*mut T` with `PhantomData<&'a mut T>` for type-safe element access.

- `new(ptr)` — `unsafe`, requires valid, aligned pointer
- `as_ref()` / `as_mut()` — `unsafe`
- `as_ptr()` / `as_mut_ptr()` — safe

`TypedArray` also provides `unsafe fn get_ref<T>(index)` which performs bounds checking, computes byte offsets, and returns `Option<TypedArrayRef>`.

## Remaining Unsafe Code

Some `unsafe` cannot be eliminated because it is inherent to the operation being performed.

### FFI Entry Points (`src/ffi/mod.rs`, `src/ffi/native.rs`)

Every `#[no_mangle] pub extern "C"` function must accept raw pointers from external callers. This is the contract of a C-compatible library. Each function:
1. Checks for null pointers before dereferencing
2. Uses `// SAFETY:` comments documenting the invariant
3. Converts raw pointers to safe wrappers at the earliest opportunity

### Dynamic Library Loading (`modules/abi/src/loader.rs`)

`NativeLibrary` declares `unsafe impl Send + Sync` because `libloading::Library` is safe to share across threads, but the compiler cannot verify this automatically.

### Transmute for Function Pointers (`src/vm/interpreter/calls.rs`)

A `std::mem::transmute` converts a `usize` (stored in the bytecode) into an `extern "C" fn` pointer. The safety invariant is that the value was registered through the native function system which validates signatures.

### Memory Leak for Library Lifetime (`src/vm/interpreter/modules.rs`)

`std::mem::forget(library)` intentionally leaks the `SafeLibrary` to prevent unloading while function pointers are still in use. A future improvement would use `Arc` or a `static` registry to manage this without leaking.

### Macro-Generated Trampolines (`modules/native-macros/src/`)

The `#[tails_function]` and `#[tails_class]` proc macros emit `slice::from_raw_parts` calls in generated FFI trampolines. These are reviewed during macro development and are not modified at runtime.

## Safety Guidelines

### For Contributors Adding New `unsafe` Code

1. **Minimize `unsafe` scope.** Wrap raw pointer operations in the smallest possible block. Use safe wrappers (`SafePtr`, `SafeCStr`, etc.) instead of raw pointers where possible.

2. **Document every `unsafe` block.** Every `unsafe` block must have a `// SAFETY:` comment explaining:
   - What invariant is being relied upon
   - Why that invariant holds at this call site
   - What would break if the invariant is violated

3. **Prefer `unsafe`-marked methods over `unsafe` blocks.** If a function performs an inherently unsafe operation (dereferencing a raw pointer), mark the function `unsafe fn` rather than containing an `unsafe` block inside a safe function. This pushes the correctness obligation to the caller.

4. **Add `#[cfg(test)]` coverage.** Every `unsafe` operation must have unit tests that verify correct behavior. See [Testing Strategy](#testing-strategy).

5. **Use the existing safe wrappers.** Before writing new `unsafe` code, check if `SafePtr`, `SafeCStr`, `SafeSlice`, `SafeFFIString`, `FFIStringBuffer`, `SafeLibrary`, `SafeFunction`, `SafeNativeFunction`, `FunctionPointerWrapper`, `TypedArrayRef`, or `SafeTypedArray` can be used.

### Invariants to Maintain

- **Pointer validity**: Raw pointers wrapped in lifetime-tied types (`SafePtr<'a, T>`, etc.) must remain valid for the duration of `'a`.
- **Null checks**: Every FFI entry point must check for null before dereferencing.
- **Type matching**: When using `SafeLibrary::get_function::<T>` or `SafeNativeFunction::new`, the type parameter `T` must exactly match the actual symbol's ABI signature.
- **No aliasing**: Mutable references derived from raw pointers must not be aliased. `SafePtr::as_mut` and `TypedArrayRef::as_mut` require exclusive access.
- **Library lifetime**: Libraries loaded via `SafeLibrary` must remain loaded as long as any function pointers obtained from them are in use.

## Testing Strategy

### Unit Tests

Every safe abstraction has `#[cfg(test)]` unit tests in its source file:

| File | Tests |
|------|-------|
| `src/ffi/safe_wrappers.rs` | Null checks, valid pointer access, mutable access, slice creation |
| `src/ffi/safe_string.rs` | Null string, valid string, owned string, buffer allocation |
| `src/vm/interpreter/safe_library.rs` | Nonexistent library, loading real library (`libc.so.6`), `SafeFunction` creation |
| `src/vm/interpreter/safe_function.rs` | Creation, name access, pointer comparison, `FunctionPointerWrapper` |
| `src/objects/safe_typed_array.rs` | Length, kind, byte offset, `TypedArrayRef` creation, `get_ref` bounds checking |

### Test Commands

```bash
# Run all tests
cargo test

# Run only FFI tests
cargo test --lib ffi::

# Run only safe wrapper tests
cargo test --lib safe_wrappers
cargo test --lib safe_string
cargo test --lib safe_library
cargo test --lib safe_function
cargo test --lib safe_typed_array
```

### What Tests Verify

- Null pointer handling returns `None` or safe defaults
- Valid pointers can be dereferenced correctly
- Mutable access modifies the underlying data
- Bounds checking prevents out-of-range access
- Library loading succeeds for valid paths and fails gracefully for invalid paths
- Function pointer wrappers maintain correct signatures

### Integration Tests

The existing test suite in `tests/` exercises native modules end-to-end, which validates that FFI wrappers produce correct behavior when called from the runtime. Run with:

```bash
cargo test --test '*'
```

## Summary

The unsafe code in Tails-rs is concentrated at well-defined boundaries (FFI, dynamic loading, byte-level access) and is contained behind documented, tested safe abstractions. Contributors should follow the guidelines above to maintain this containment as the codebase evolves.
