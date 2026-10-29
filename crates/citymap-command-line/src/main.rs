//! The `citymap` program: reads the command line, runs the command, and turns an error into
//! a message on stderr and the exit code REQUIREMENTS.md F20 gives it.

use std::process::ExitCode;

use citymap_command_line::arguments::{self, Request};
use citymap_command_line::generate::generate;
use citymap_command_line::validate::read_valid;

fn main() -> ExitCode {
    let result = arguments::parse(std::env::args_os()).and_then(|request| match request {
        Request::Generate(arguments) => generate(&arguments).map(|generated| {
            for file in generated.files {
                println!("wrote {}", file.display());
            }
            for notice in generated.notices {
                eprintln!("notice: {notice}");
            }
            for warning in generated.warnings {
                eprintln!("warning: {warning}");
            }
        }),
        Request::Validate(arguments) => read_valid(&arguments.specification).map(|valid| {
            println!("{} is valid", arguments.specification.display());
            for warning in valid.warnings {
                eprintln!("warning: {warning}");
            }
        }),
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
