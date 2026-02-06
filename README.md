# Rust Shell

A simple interactive shell written in Rust.

## Features

- Interactive command prompt
- External command execution

## Building

Requires Rust 1.70+ (2024 edition).

```bash
cargo build --release
```

## Running

```bash
cargo run
```

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

