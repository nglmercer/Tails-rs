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
