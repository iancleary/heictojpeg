# List available recipes
help:
    @just --list

# Build the project
build:
    cargo build

# Build release
release:
    cargo build --release

# Run clippy
lint:
    cargo clippy -- -D warnings

# Format code
fmt:
    cargo fmt

# Check formatting
fmt-check:
    cargo fmt --check

# Run tests
test:
    cargo test

# Check without building
check:
    cargo check

# Run with arguments
run *ARGS:
    cargo run -- {{ARGS}}
