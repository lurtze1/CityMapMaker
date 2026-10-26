//! The `generate` command end to end: real files in, real files out, in the temporary folder
//! Cargo gives integration tests.

use std::fs;
use std::path::{Path, PathBuf};

use citymap_command_line::CommandLineError;
use citymap_command_line::arguments::GenerateArguments;
use citymap_command_line::generate::generate;
use proptest::prelude::*;

const EXAMPLE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../specifications/example.yaml"
);

fn folder(name: &str) -> PathBuf {
    let folder = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = fs::remove_dir_all(&folder);
    folder
}

fn arguments(specification: &Path, seed: Option<u64>, output: &Path) -> GenerateArguments {
    GenerateArguments {
        specification: specification.to_path_buf(),
        seed,
        output: output.to_path_buf(),
    }
}

#[test]
fn the_example_becomes_an_svg_named_after_the_file_and_seed() {
    let output = folder("example");
    let file = generate(&arguments(Path::new(EXAMPLE), None, &output)).expect("the example works");
    assert_eq!(file, output.join("example-seed-1.svg"));
    let text = fs::read_to_string(&file).expect("the file was written");
    assert!(text.starts_with("<svg"), "{text}");
}

#[test]
fn the_seed_argument_replaces_the_specification_seed_in_the_name() {
    let output = folder("seed");
    let file = generate(&arguments(Path::new(EXAMPLE), Some(42), &output)).expect("it works");
    assert_eq!(file, output.join("example-seed-42.svg"));
}

#[test]
fn a_missing_file_is_a_reading_error() {
    let output = folder("missing");
    let error = generate(&arguments(&output.join("absent.yaml"), None, &output))
        .expect_err("the file does not exist");
    assert!(matches!(error, CommandLineError::Reading { .. }), "{error}");
    assert_eq!(error.exit_code(), 1);
}

#[test]
fn a_broken_specification_is_invalid_input() {
    let output = folder("broken");
    fs::create_dir_all(&output).expect("the folder can be made");
    let specification = output.join("broken.yaml");
    fs::write(&specification, "schema: 1\ncolour: red\n").expect("the file can be written");
    let error = generate(&arguments(&specification, None, &output)).expect_err("it is invalid");
    assert!(
        matches!(error, CommandLineError::InvalidSpecification { .. }),
        "{error}"
    );
    assert_eq!(error.exit_code(), 2);
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(16))]

    #[test]
    fn two_runs_write_the_same_bytes(seed in any::<u64>()) {
        let first = folder("first");
        let second = folder("second");
        let first = generate(&arguments(Path::new(EXAMPLE), Some(seed), &first)).expect("it works");
        let second = generate(&arguments(Path::new(EXAMPLE), Some(seed), &second)).expect("it works");
        prop_assert_eq!(fs::read(first).expect("written"), fs::read(second).expect("written"));
    }
}
