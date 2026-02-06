# Rust Shell

A simple interactive shell written in Rust for learning purposes.

## Features

- Interactive command prompt
- Built-in commands: `echo`, `exit`, `type`
- External command execution
- Command parsing with whitespace handling

## Building

Requires Rust 1.70+ (2024 edition).

```bash
cargo build --release
```

## Running

```bash
cargo run
```

Or after building:

```bash
./target/release/shell
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

## Project Structure

- `src/main.rs` - Main entry point and command loop
- `src/builtin.rs` - Built-in command implementations
- `src/command.rs` - Command trait definitions
- `src/external.rs` - External command execution
- `tests/integration_test.rs` - Integration tests

## Usage

Start the shell and enter commands:

```
$ echo hello world
hello world
$ type echo
echo is a shell builtin
$ exit
```
