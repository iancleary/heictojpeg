use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::time::Instant;

use crate::convert::convert_heic_to_jpeg;

/// Resolve the input path to a directory and list of HEIC files.
fn resolve_input(input_path: &str) -> Result<(PathBuf, Vec<PathBuf>), Box<dyn std::error::Error>> {
    let path = Path::new(input_path);
    let metadata = fs::metadata(path)?;

    if metadata.is_dir() {
        let files = get_heic_files(path)?;
        Ok((path.to_path_buf(), files))
    } else {
        let parent = path.parent().unwrap_or(Path::new(".")).to_path_buf();
        Ok((parent, vec![path.to_path_buf()]))
    }
}

/// Get all .heic files in a directory.
fn get_heic_files(dir: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext.eq_ignore_ascii_case("heic") {
                    files.push(path);
                }
            }
        }
    }
    files.sort();
    Ok(files)
}

/// Ensure the output JPEG directory exists.
fn ensure_jpeg_dir(base_dir: &Path) -> PathBuf {
    let jpeg_dir = base_dir.join("jpegs");
    fs::create_dir_all(&jpeg_dir).expect("Failed to create jpegs directory");
    jpeg_dir
}

/// Get the output JPEG path for a given HEIC file.
fn jpeg_output_path(jpeg_dir: &Path, heic_path: &Path) -> PathBuf {
    let stem = heic_path.file_stem().unwrap_or_default();
    jpeg_dir.join(format!("{}.jpg", stem.to_string_lossy()))
}

/// Format bytes into human-readable size.
fn human_readable_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    if unit_idx == 0 {
        format!("{}B", bytes)
    } else {
        format!("{:.1}{}", size, UNITS[unit_idx])
    }
}

/// Process all HEIC files, converting them to JPEG in parallel.
fn process_files(heic_files: &[PathBuf], jpeg_dir: &Path) -> Vec<(String, Result<(), String>)> {
    use rayon::prelude::*;

    heic_files
        .par_iter()
        .map(|heic_path| {
            let file_name = heic_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            println!("Processing file: {}", file_name);

            let output_path = jpeg_output_path(jpeg_dir, heic_path);
            let result = convert_heic_to_jpeg(heic_path, &output_path)
                .map_err(|e| format!("error details: {}", e));

            (file_name, result)
        })
        .collect()
}

/// Save conversion logs to a file in the JPEG directory.
fn save_logs(
    jpeg_dir: &Path,
    base_dir: &Path,
    results: &[(String, Result<(), String>)],
    duration: std::time::Duration,
) {
    let log_path = jpeg_dir.join("logs.txt");
    let mut log_lines = Vec::new();
    let mut total_heic_size: u64 = 0;
    let mut total_jpeg_size: u64 = 0;

    for (file_name, result) in results {
        let heic_path = base_dir.join(file_name);
        let jpeg_path = jpeg_output_path(jpeg_dir, &heic_path);

        let heic_size = fs::metadata(&heic_path).map(|m| m.len()).unwrap_or(0);
        let jpeg_size = fs::metadata(&jpeg_path).map(|m| m.len()).unwrap_or(0);

        total_heic_size += heic_size;
        total_jpeg_size += jpeg_size;

        let stem = Path::new(file_name)
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy();

        match result {
            Ok(()) => {
                log_lines.push(format!(
                    "{} {} > Converted > jpegs/{}.jpg {}",
                    file_name,
                    human_readable_size(heic_size),
                    stem,
                    human_readable_size(jpeg_size)
                ));
            }
            Err(e) => {
                log_lines.push(format!("{} > Error: {}", file_name, e));
            }
        }
    }

    let file_count = results.len();
    let avg_duration = if file_count > 0 {
        duration / file_count as u32
    } else {
        duration
    };

    log_lines.push(String::new());
    log_lines.push(format!("{} Files", file_count));
    log_lines.push(format!("Total Time Taken=={:?}", duration));
    log_lines.push(format!("Average Time Per File=={:?}", avg_duration));
    log_lines.push(format!(
        "Total HEIC File Size=={}",
        human_readable_size(total_heic_size)
    ));
    log_lines.push(format!(
        "Total JPEG Folder Size=={}",
        human_readable_size(total_jpeg_size)
    ));

    println!("Saving logs to logs.txt...");
    fs::write(log_path, log_lines.join("\n")).expect("Failed to write log file");
}

#[derive(Debug)]
pub struct Command {}

impl Command {
    pub fn run(args: &[String]) -> Result<Command, Box<dyn std::error::Error>> {
        if args.len() < 2 {
            // Default to current directory
            return Self::run_conversion(".");
        }

        // Check for special flags
        match args[1].as_str() {
            "--version" | "-v" => {
                print_version();
                process::exit(0);
            }
            "--help" | "-h" => {
                print_help();
                process::exit(0);
            }
            _ => {}
        }

        if args.len() > 2 {
            return Err("too many arguments, expecting: heictojpeg [path]".into());
        }

        Self::run_conversion(&args[1])
    }

    fn run_conversion(input_path: &str) -> Result<Command, Box<dyn std::error::Error>> {
        println!("Starting the program...");

        let (base_dir, heic_files) = resolve_input(input_path)?;

        if heic_files.is_empty() {
            println!("No HEIC files found.");
            return Ok(Command {});
        }

        println!("Found {} HEIC file(s)", heic_files.len());

        let jpeg_dir = ensure_jpeg_dir(&base_dir);
        let start = Instant::now();
        let results = process_files(&heic_files, &jpeg_dir);
        let duration = start.elapsed();

        save_logs(&jpeg_dir, &base_dir, &results, duration);

        let success_count = results.iter().filter(|(_, r)| r.is_ok()).count();
        let error_count = results.len() - success_count;

        println!(
            "\nProgram completed! {} converted, {} errors, took {:?}",
            success_count, error_count, duration
        );

        Ok(Command {})
    }
}

pub fn print_version() {
    println!("heictojpeg {}", env!("CARGO_PKG_VERSION"));
}

pub fn print_error(error: &str) {
    const RED: &str = "\x1b[31m";
    const RESET: &str = "\x1b[0m";
    println!("{}Problem parsing arguments: {error}{}", RED, RESET);
}

pub fn print_help() {
    const BOLD: &str = "\x1b[1m";
    const CYAN: &str = "\x1b[36m";
    const GREEN: &str = "\x1b[32m";
    const YELLOW: &str = "\x1b[33m";
    const RESET: &str = "\x1b[0m";

    println!(
        "📸 HEIC to JPEG converter - https://github.com/iancleary/heictojpeg{}",
        RESET
    );
    println!();
    println!("{}{}VERSION:{}", BOLD, YELLOW, RESET);
    println!("    {}{}{}", GREEN, env!("CARGO_PKG_VERSION"), RESET);
    println!();
    println!("{}{}USAGE:{}", BOLD, YELLOW, RESET);
    println!("    {} heictojpeg [PATH]{}", GREEN, RESET);
    println!();
    println!("     PATH: path to a directory of HEIC files or a single HEIC file");
    println!("           (defaults to current directory if omitted)");
    println!();
    println!("     Converted JPEG files are saved to a 'jpegs/' subdirectory");
    println!("     alongside the source files. EXIF data is preserved.");
    println!();
    println!("{}{}OPTIONS:{}", BOLD, YELLOW, RESET);
    println!(
        "    {}  -v, --version{}    Print version information",
        GREEN, RESET
    );
    println!(
        "    {}  -h, --help{}       Print help information",
        GREEN, RESET
    );
    println!();
    println!("{}{}EXAMPLES:{}", BOLD, YELLOW, RESET);
    println!(
        "    {} # Convert all HEIC files in current directory{}",
        CYAN, RESET
    );
    println!("    {} heictojpeg{}", GREEN, RESET);
    println!();
    println!(
        "    {} # Convert all HEIC files in a specific directory{}",
        CYAN, RESET
    );
    println!("    {} heictojpeg ~/Photos{}", GREEN, RESET);
    println!();
    println!("    {} # Convert a single file{}", CYAN, RESET);
    println!("    {} heictojpeg photo.heic{}", GREEN, RESET);
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_readable_size() {
        assert_eq!(human_readable_size(0), "0B");
        assert_eq!(human_readable_size(500), "500B");
        assert_eq!(human_readable_size(1024), "1.0KB");
        assert_eq!(human_readable_size(1536), "1.5KB");
        assert_eq!(human_readable_size(1_048_576), "1.0MB");
        assert_eq!(human_readable_size(1_073_741_824), "1.0GB");
    }

    #[test]
    fn test_version_output() {
        let version = env!("CARGO_PKG_VERSION");
        assert!(!version.is_empty());
        let parts: Vec<&str> = version.split('.').collect();
        assert_eq!(parts.len(), 3);
    }
}
