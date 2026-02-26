# HEIC to JPEG Converter

A fast, concurrent CLI tool for batch-converting `.heic` files to `.jpg`. Written in Go.

## Features

- **Batch conversion** — process entire directories of HEIC files at once
- **Concurrent** — uses goroutines for parallel conversion
- **Flexible input** — current directory, specific directory, or single file
- **Organized output** — converted files saved to a `jpegs/` subfolder
- **Detailed logging** — `logs.txt` with file sizes and timing stats

## Installation

### Build from source

```bash
git clone https://github.com/iancleary/heictojpeg.git
cd heictojpeg
go build -o heictojpeg .
```

### Install with Go

```bash
go install github.com/iancleary/heictojpeg@latest
```

### Download a release

Grab a binary from [GitHub Releases](https://github.com/iancleary/heictojpeg/releases) if available.

## Usage

```bash
# Convert all .heic files in the current directory
heictojpeg

# Convert all .heic files in a specific directory
heictojpeg /path/to/photos

# Convert a single file
heictojpeg /path/to/photo.heic
```

Converted images are saved to a `jpegs/` subfolder in the target directory.

## Example Output

```
IMG_0267.HEIC==IMG_0267.HEIC 1.2MB > Converted > jpegs/IMG_0267.jpg 1.0MB
IMG_0719.HEIC==IMG_0719.HEIC 977.9KB > Converted > jpegs/IMG_0719.jpg 953.0KB

325 Files
Total time taken: 27s 220ms
Average time per file: 0s 83ms

Total HEIC files size: 311.0MB
Total JPEG folder size: 280.4MB
```

## Development

```bash
go test ./...
go build -o heictojpeg .
```

## Attribution

Inspired by the original work from [cckalen/heictojpeg](https://github.com/cckalen/heictojpeg). Credit to the original author for the initial concept.

## License

MIT
