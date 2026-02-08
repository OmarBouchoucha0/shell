# Rust Shell

[![Rust CI](https://github.com/yourusername/shell/actions/workflows/rust.yml/badge.svg)](https://github.com/yourusername/shell/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/yourusername/shell/branch/main/graph/badge.svg)](https://codecov.io/gh/yourusername/shell)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

A simple interactive shell written in Rust.

## Features

- Interactive command prompt with line editing (via rustyline)
- Command history with search (Ctrl+R)
- Built-in commands: echo, exit, pwd, cd, history, type
- External command execution
- Comprehensive test suite (44+ tests)

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

Run unit tests only:

```bash
cargo test --lib
```

Run integration tests only:

```bash
cargo test --test integration_test
```

## Linting and Formatting

Check code formatting:

```bash
cargo fmt -- --check
```

Format code:

```bash
cargo fmt
```

Run clippy lints:

```bash
cargo clippy -- -D warnings
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

Coverage reports are automatically uploaded to Codecov on every PR.

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

# Check licenses only
cargo deny check licenses

# Check for security advisories
cargo deny check advisories
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

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and contribution guidelines.

## Continuous Integration

Our CI runs the following checks on every PR:

- ✅ Code formatting (rustfmt)
- ✅ Linting (clippy)
- ✅ Documentation builds
- ✅ All tests (debug and release)
- ✅ Security audit (cargo audit)
- ✅ Code coverage (tarpaulin → codecov)
- ✅ Dependency linting (cargo deny)
- ✅ All targets check

## License

This project is licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.
