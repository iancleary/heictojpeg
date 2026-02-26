# CLAUDE.md — heictojpeg

## Overview
Go CLI tool that batch-converts HEIC images to JPEG using concurrent workers.

## Structure
- `main.go` — CLI entry point, concurrency, HEIC→JPEG conversion logic (~300 lines)
- `main_test.go` — unit tests (~150 lines)
- `go.mod` / `go.sum` — Go 1.19, depends on `adrium/goheif`

## Build & Test
```bash
go build -o heictojpeg .
go test ./...
```

## Key Details
- Output goes to `jpegs/` subfolder in the target directory
- Generates `logs.txt` with conversion stats
- Three input modes: no args (cwd), directory path, single file path
- Uses goroutines for concurrent conversion
