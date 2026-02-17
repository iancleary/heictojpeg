# CLAUDE.md — heictojpeg

## Overview

HEIC-to-JPEG converter. Originally written in Go, with an in-progress Rust port on the `rust-port` branch. Converts `.heic` files to `.jpg` with EXIF preservation, using multi-threading for batch processing.

## Branches

- **main** — Go implementation (working, stable)
- **origin/rust-port** — Rust rewrite (in progress), uses `libheif-rs`, `image`, `rayon`

## Commands (Rust port)

```bash
just build          # cargo build
just release        # cargo build --release
just test           # cargo test
just lint           # cargo clippy -- -D warnings
just fmt            # cargo fmt
just fmt-check      # cargo fmt --check
just run <ARGS>     # cargo run -- <args>
```

## Structure (Rust port)

```
src/
  main.rs          # CLI entry point
  cli.rs           # Argument parsing, help/error output
  convert.rs       # HEIC→JPEG conversion logic
  lib.rs           # Library root
Cargo.toml         # Dependencies: libheif-rs, image, rayon, img-parts, walkdir
justfile           # Task runner recipes
testdata/images/   # Test HEIC/AVIF files and expected JPEG output
```

## Go Implementation (main branch)

```
main.go            # Entry point and conversion logic
main_test.go       # Tests
go.mod / go.sum    # Go dependencies (goheif, walk for Windows GUI)
```

## Key Dependencies (Rust)

- `libheif-rs` — HEIC decoding (requires libheif system library)
- `image` — JPEG encoding
- `rayon` — Parallel processing
- `img-parts` — EXIF extraction/insertion
- `walkdir` — Directory traversal

## Where to Look

- **src/convert.rs** — core conversion logic (Rust)
- **src/cli.rs** — CLI argument handling (Rust)
- **main.go** — everything for the Go version
- **testdata/** — test fixtures
