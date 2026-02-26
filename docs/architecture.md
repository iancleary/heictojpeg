# Architecture — heictojpeg

## Overview

Single-binary CLI that batch-converts HEIC images to JPEG with EXIF preservation.

## Processing Pipeline

```
CLI args → resolveInput()
             ↓
       ensureJPEGDir()
             ↓
       processFiles()
         ├─ fileChan (buffered, all entries)
         ├─ N workers (runtime.NumCPU goroutines)
         │   └─ processFile() → convertHeicToJpg()
         └─ logChan → aggregateLogs()
             ↓
       saveLogsToFile()
```

## Input Modes

| Args          | Behavior                                |
|---------------|-----------------------------------------|
| (none)        | Process all `.heic` in current dir      |
| `<directory>` | Process all `.heic` in that directory   |
| `<file>`      | Process that single file                |

## EXIF Preservation

The converter manually injects EXIF data into the JPEG output stream:

1. Extract EXIF bytes via `goheif.ExtractExif()`
2. Write JPEG SOI marker (`0xFF 0xD8`)
3. Write APP1 marker + EXIF payload
4. Use `writerSkipper` to skip the 2-byte SOI that `jpeg.Encode` would write

This avoids depending on a separate EXIF library while preserving orientation, camera info, and GPS data.

## Concurrency Model

- Worker count = `runtime.NumCPU()`
- Buffered channels sized to file count (no backpressure issues for typical batch sizes)
- Each worker independently decodes HEIC → encodes JPEG (CPU-bound, scales linearly with cores)
- Log aggregation happens after all workers complete

## Dependencies

| Package                         | Purpose              |
|---------------------------------|----------------------|
| `github.com/adrium/goheif`     | HEIC decode + EXIF   |
| stdlib `image/jpeg`             | JPEG encoding        |

## Testing

Tests in `main_test.go` cover:
- Input resolution (directory, file, no-args)
- File size formatting
- JPEG output path generation
- Worker pipeline integration

## Known Limitations

- JPEG quality hardcoded (default `jpeg.Encode` quality = 75)
- No recursive directory walking
- Log format is human-readable, not machine-parseable
- Division by zero if no HEIC files found (in `aggregateLogs`)
