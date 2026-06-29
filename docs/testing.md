```bash
# Run all tests (default features)
cargo test

# Run without optional modules
cargo test --no-default-features

# Run with specific features only
cargo test --no-default-features -F fs -F path
```
