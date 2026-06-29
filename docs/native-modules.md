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

## Available Native Modules

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

## Module Architecture

Each feature-gated module is split into two layers:

- **`modules/<name>/`** — Pure Rust implementation with no dependency on the runtime. Contains the actual business logic (fs operations, path manipulation, etc.)
- **`src/runtime_env/native_fns/<name>_fns.rs`** — Thin adapter that converts between runtime `Value` types and the pure module functions

This separation keeps the core runtime lightweight and the module logic testable independently.
