# Rust Shell

[![Rust CI](https://github.com/yourusername/shell/actions/workflows/rust.yml/badge.svg)](https://github.com/yourusername/shell/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/yourusername/shell/branch/main/graph/badge.svg)](https://codecov.io/gh/yourusername/shell)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

A simple interactive shell written in Rust.
## Building

Requires Rust 1.70+ (2024 edition).

```bash
cargo build --release
```

## Running

```bash
cargo run
```

## Testing

Run all tests:

```bash
cargo test
```


## Code Coverage

Generate coverage report locally:

```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Generate HTML report
cargo tarpaulin --out Html

# Or XML report for CI
cargo tarpaulin --out xml
```


## Security Audit

Check for security vulnerabilities:

```bash
# Install cargo-audit
cargo install cargo-audit

# Run audit
cargo audit
```

Security audits run automatically in CI on every PR.

## Dependency Management

We use `cargo-deny` to lint dependencies:

```bash
# Install cargo-deny
cargo install cargo-deny

# Check all dependency rules
cargo deny check


```

## Pre-commit Hooks

We use pre-commit hooks to ensure code quality. To set them up:

```bash
# Install pre-commit
pip install pre-commit

# Install the git hooks
pre-commit install

# Run against all files manually
pre-commit run --all-files
```

## License

This project is dual-licensed under either:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

