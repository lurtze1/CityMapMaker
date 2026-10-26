//! Command-line adapter for the city map generator.
//!
//! Owns every side effect: file I/O, YAML parsing, logging and exit codes.

use std::process::ExitCode;

use citymap_command_line::arguments::{self, Request};
use citymap_command_line::generate::generate;

fn main() -> ExitCode {
    let result = arguments::parse(std::env::args_os()).and_then(|request| match request {
        Request::Generate(arguments) => {
            generate(&arguments).map(|file| println!("wrote {}", file.display()))
        }
        Request::Show(text) => {
            print!("{text}");
            Ok(())
        }
    });
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(error.exit_code())
        }
    }
}
