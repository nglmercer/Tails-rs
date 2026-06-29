# Tails-rs

A TypeScript-first runtime implemented in Rust.

## Overview

Tails-rs is a JavaScript/TypeScript runtime built from scratch in Rust. It compiles source code to bytecode and executes it on a stack-based virtual machine with garbage collection. It supports many modern JavaScript features including classes, promises, async/await, ES modules, and more.

## Quick Start

```bash
# Build
cargo build --release

# Run a script
./target/release/tails script.ts

# See all features in action
cargo run --bin tails -- examples/all_features.ts
```

## Documentation

- [Installation](docs/installation.md) — Build instructions and feature flags
- [Usage](docs/usage.md) — CLI and library usage
- [Native Modules](docs/native-modules.md) — Module system, imports, and architecture
- [Features](docs/features.md) — Complete list of supported JavaScript/TypeScript features
- [Architecture](docs/architecture.md) — Source layout and design overview
- [Testing](docs/testing.md) — Running the test suite
- [Roadmap](docs/roadmap.md) — Completed work and planned features
