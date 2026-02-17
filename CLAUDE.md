# CLAUDE.md — heictojpeg

## Overview

HEIC-to-JPEG converter written in Go. Converts `.heic` files to `.jpg` with EXIF preservation, using multi-threading for batch processing.

## Structure

```
main.go            # Entry point and conversion logic
main_test.go       # Tests
go.mod / go.sum    # Go dependencies (goheif, walk for Windows GUI)
testdata/images/   # Test HEIC/AVIF files and expected JPEG output
```

## Commands

```bash
go build            # Build
go test ./...       # Run tests
go run .            # Run
```

## Where to Look

- **main.go** — all conversion logic
- **testdata/** — test fixtures
