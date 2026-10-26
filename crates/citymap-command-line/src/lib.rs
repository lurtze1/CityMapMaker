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
    /// An output file or folder could not be written.
    #[error("cannot write {}: {message}", path.display())]
    Writing {
        /// The file or folder that could not be written.
        path: PathBuf,
        /// What the operating system reported.
        message: String,
    },
}

impl CommandLineError {
    /// The process exit code for this error: 2 for invalid input, 1 for reading and writing.
    pub fn exit_code(&self) -> u8 {
        match self {
            CommandLineError::InvalidArguments { .. }
            | CommandLineError::InvalidSpecification { .. } => 2,
            CommandLineError::Reading { .. } | CommandLineError::Writing { .. } => 1,
        }
    }
}
