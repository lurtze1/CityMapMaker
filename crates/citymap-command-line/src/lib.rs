//! The library part of the command-line adapter: the facades and file handling that the
//! `citymap` program and the tests in `tests/` share.

pub mod arguments;
pub mod generate;
pub mod yaml;

use std::path::PathBuf;

/// Why a command failed. Each variant has the exit code REQUIREMENTS.md F20 gives it.
#[derive(Debug, thiserror::Error)]
pub enum CommandLineError {
    /// The command line itself is wrong, such as a missing file name or an unknown flag.
    #[error("{message}")]
    InvalidArguments {
        /// The argument parser's explanation, with a usage line.
        message: String,
    },
    /// The specification file could not be read.
    #[error("cannot read {}: {message}", path.display())]
    Reading {
        /// The file that could not be read.
        path: PathBuf,
        /// What the operating system reported.
        message: String,
    },
    /// The specification file is not a valid specification.
    #[error("{}: {source}", path.display())]
    InvalidSpecification {
        /// The specification file.
        path: PathBuf,
        /// What the YAML reader reported, with line and column.
        source: yaml::YamlError,
    },
    /// The specification reads, but breaks rules that validation checks.
    #[error("{}: {}", path.display(), list(errors))]
    BrokenRules {
        /// The specification file.
        path: PathBuf,
        /// Every rule it breaks, in the order of the file.
        errors: Vec<citymap_core::validation::ValidationError>,
    },
    /// The map could not be drawn as a PNG.
    #[error("cannot draw the PNG: {source}")]
    Rendering {
        /// What the renderer reported.
        source: citymap_render::RasterError,
    },
    /// An output file or folder could not be written.
    #[error("cannot write {}: {message}", path.display())]
    Writing {
        /// The file or folder that could not be written.
        path: PathBuf,
        /// What the operating system reported.
        message: String,
    },
}

fn list(errors: &[citymap_core::validation::ValidationError]) -> String {
    let lines: Vec<String> = errors.iter().map(|error| format!("\n  {error}")).collect();
    lines.concat()
}

impl CommandLineError {
    /// The process exit code for this error: 2 for invalid input, 1 for reading, drawing and
    /// writing.
    pub fn exit_code(&self) -> u8 {
        match self {
            CommandLineError::InvalidArguments { .. }
            | CommandLineError::InvalidSpecification { .. }
            | CommandLineError::BrokenRules { .. } => 2,
            CommandLineError::Reading { .. }
            | CommandLineError::Rendering { .. }
            | CommandLineError::Writing { .. } => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_error_has_the_exit_code_of_requirement_f20() {
        let path = PathBuf::from("city.yaml");
        let message = String::new();
        let invalid_specification =
            yaml::from_text::<u32>("[").expect_err("an unclosed list is invalid YAML");
        let cases = [
            (
                CommandLineError::InvalidArguments {
                    message: message.clone(),
                },
                2,
            ),
            (
                CommandLineError::Reading {
                    path: path.clone(),
                    message: message.clone(),
                },
                1,
            ),
            (
                CommandLineError::InvalidSpecification {
                    path: path.clone(),
                    source: invalid_specification,
                },
                2,
            ),
            (
                CommandLineError::BrokenRules {
                    path: path.clone(),
                    errors: Vec::new(),
                },
                2,
            ),
            (
                CommandLineError::Rendering {
                    source: citymap_render::RasterError::InvalidSize {
                        width: 0,
                        height: 0,
                    },
                },
                1,
            ),
            (CommandLineError::Writing { path, message }, 1),
        ];
        for (error, code) in cases {
            assert_eq!(error.exit_code(), code, "{error}");
        }
    }
}
