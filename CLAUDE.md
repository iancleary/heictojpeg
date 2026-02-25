# CLAUDE.md — heictojpeg

HEIC-to-JPEG batch converter written in Go. Concurrent, preserves EXIF.

## Quick Reference

| What        | Where / Command             |
|-------------|-----------------------------|
| Build       | `go build -o heictojpeg .`  |
| Test        | `go test -v ./...`          |
| Lint        | `go vet ./...`              |
| Module      | `heictojpeg` (Go 1.19)     |
| Dependency  | `github.com/adrium/goheif`  |

## Structure

```
main.go            # Entry point and conversion logic (~300 lines)
main_test.go       # Tests (table-driven)
go.mod / go.sum    # Go dependencies
testdata/images/   # Test HEIC/AVIF files and expected JPEG output
docs/              # Architecture documentation
```

## Architecture

Single-file CLI. Worker-pool pattern using goroutines (one per CPU core). Processes `.heic` files → `jpegs/` subfolder with EXIF preservation via SOI/APP1 marker injection.

Key flow: `resolveInput() → ensureJPEGDir() → processFiles() → saveLogsToFile()`

## Code Conventions

- Keep it single-file unless complexity demands splitting
- All exported functions should have doc comments
- Tests live in `main_test.go` — table-driven preferred
- No `target/` directory (Rust port was abandoned)

## Where to Look

- **main.go** — all conversion logic
- **testdata/** — test fixtures
- [docs/architecture.md](docs/architecture.md) — detailed design notes
