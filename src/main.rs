use std::env;
use std::process;

use heictojpeg::cli;

fn main() {
    let args: Vec<String> = env::args().collect();

    let _ = cli::Command::run(&args).unwrap_or_else(|err| {
        println!();
        cli::print_error(&err.to_string());
        println!();
        cli::print_help();
        println!();
        cli::print_error(&err.to_string());
        process::exit(1);
    });
}
