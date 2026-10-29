//! The argument facade: the only module that calls `clap` (ADR-043). It turns the command
//! line into our own `Request`, so the parser can be replaced here alone.
//!
//! # Clap functions used
//!
//! - `clap::Command`: `new`, `about`, `subcommand`, `subcommand_required`, `arg`,
//!   `try_get_matches_from`.
//! - `clap::Arg`: `new`, `required`, `long`, `value_name`, `help`, `value_parser`,
//!   `default_value`, `action` with `ArgAction::SetTrue`; `clap::value_parser!`.
//! - `clap::ArgMatches`: `subcommand`, `get_one`, `get_flag`; `clap::Error`: `kind`, `render`;
//!   `clap::error::ErrorKind::DisplayHelp`.

use std::ffi::OsString;
use std::path::PathBuf;

use clap::error::ErrorKind;
use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::CommandLineError;

/// What the user asked for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Request {
    /// Generate a map (REQUIREMENTS.md F17).
    Generate(GenerateArguments),
    /// Read and check a specification without generating (REQUIREMENTS.md F18).
    Validate(ValidateArguments),
    /// Print this text and stop, such as the help asked for with `--help`.
    Show(String),
}

/// The arguments of `generate`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GenerateArguments {
    /// The specification file to read.
    pub specification: PathBuf,
    /// The seed for this run; `None` uses the specification's own seed.
    pub seed: Option<u64>,
    /// The folder the map files are written to; default `output`.
    pub output: PathBuf,
    /// Whether a PNG is written next to the SVG.
    pub png: bool,
}

/// The arguments of `validate`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidateArguments {
    /// The specification file to read.
    pub specification: PathBuf,
}

/// Reads the command line; the first item is the program's name.
///
/// # Errors
///
/// `CommandLineError::InvalidArguments` with the parser's message for a wrong command line.
pub fn parse<I, T>(arguments: I) -> Result<Request, CommandLineError>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    match command().try_get_matches_from(arguments) {
        Ok(matches) => Ok(request(&matches)),
        Err(error) if error.kind() == ErrorKind::DisplayHelp => {
            Ok(Request::Show(error.render().to_string()))
        }
        Err(error) => {
            let text = error.render().to_string();
            let message = text.strip_prefix("error: ").unwrap_or(&text).trim_end();
            Err(CommandLineError::InvalidArguments {
                message: message.to_string(),
            })
        }
    }
}

fn command() -> Command {
    let specification = Arg::new("specification")
        .required(true)
        .value_name("SPECIFICATION")
        .value_parser(clap::value_parser!(PathBuf))
        .help("The specification file, in YAML");
    let seed = Arg::new("seed")
        .long("seed")
        .value_name("SEED")
        .value_parser(clap::value_parser!(u64))
        .help("The seed for this run instead of the specification's seed");
    let output = Arg::new("output")
        .long("output")
        .value_name("FOLDER")
        .value_parser(clap::value_parser!(PathBuf))
        .default_value("output")
        .help("The folder the map files are written to");
    let png = Arg::new("png")
        .long("png")
        .action(ArgAction::SetTrue)
        .help("Also write the map as a PNG");
    Command::new("citymap")
        .about("Generates a city map from a specification")
        .subcommand_required(true)
        .subcommand(
            Command::new("generate")
                .about("Writes the map of one specification")
                .arg(specification.clone())
                .arg(seed)
                .arg(output)
                .arg(png),
        )
        .subcommand(
            Command::new("validate")
                .about("Reads and checks one specification without generating")
                .arg(specification),
        )
}

fn request(matches: &ArgMatches) -> Request {
    match matches.subcommand() {
        Some(("validate", validate)) => Request::Validate(ValidateArguments {
            specification: specification(validate),
        }),
        _ => Request::Generate(generate_arguments(matches)),
    }
}

fn specification(matches: &ArgMatches) -> PathBuf {
    matches
        .get_one::<PathBuf>("specification")
        .expect("clap requires the specification")
        .clone()
}

fn generate_arguments(matches: &ArgMatches) -> GenerateArguments {
    let (_, generate) = matches
        .subcommand()
        .expect("clap requires the generate subcommand");
    GenerateArguments {
        specification: specification(generate),
        seed: generate.get_one::<u64>("seed").copied(),
        output: generate
            .get_one::<PathBuf>("output")
            .expect("clap gives output a default")
            .clone(),
        png: generate.get_flag("png"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate(arguments: &[&str]) -> GenerateArguments {
        match parse(arguments).expect("the command line is valid") {
            Request::Generate(arguments) => arguments,
            other => panic!("expected generate, got {other:?}"),
        }
    }

    #[test]
    fn validate_reads_its_specification() {
        let request = parse(["citymap", "validate", "specifications/example.yaml"])
            .expect("the command line is valid");
        assert_eq!(
            request,
            Request::Validate(ValidateArguments {
                specification: PathBuf::from("specifications/example.yaml"),
            })
        );
    }

    #[test]
    fn validate_without_a_specification_is_invalid_input() {
        let error = parse(["citymap", "validate"]).expect_err("the specification is required");
        assert_eq!(error.exit_code(), 2);
    }

    #[test]
    fn generate_reads_every_argument() {
        let arguments = generate(&[
            "citymap",
            "generate",
            "specifications/example.yaml",
            "--seed",
            "7",
            "--output",
            "maps",
            "--png",
        ]);
        assert_eq!(
            arguments,
            GenerateArguments {
                specification: PathBuf::from("specifications/example.yaml"),
                seed: Some(7),
                output: PathBuf::from("maps"),
                png: true,
            }
        );
    }

    #[test]
    fn seed_and_output_are_optional() {
        let arguments = generate(&["citymap", "generate", "city.yaml"]);
        assert_eq!(arguments.seed, None);
        assert_eq!(arguments.output, PathBuf::from("output"));
        assert!(!arguments.png);
    }

    #[test]
    fn help_is_text_to_show_not_an_error() {
        let request = parse(["citymap", "--help"]).expect("help is not an error");
        assert!(matches!(request, Request::Show(text) if text.contains("generate")));
    }

    #[test]
    fn a_missing_specification_is_invalid_input() {
        let error = parse(["citymap", "generate"]).expect_err("the specification is required");
        assert!(matches!(error, CommandLineError::InvalidArguments { .. }));
        assert_eq!(error.exit_code(), 2);
    }

    #[test]
    fn a_seed_that_is_not_a_number_is_invalid_input() {
        let error = parse(["citymap", "generate", "city.yaml", "--seed", "seven"])
            .expect_err("the seed is a number");
        assert_eq!(error.exit_code(), 2);
    }
}
