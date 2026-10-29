//! The `validate` command end to end: the program reads a file and exits with the codes of
//! REQUIREMENTS.md F20, without writing a map.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const EXAMPLE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../specifications/example.yaml"
);

fn file(name: &str, text: &str) -> PathBuf {
    let folder = Path::new(env!("CARGO_TARGET_TMPDIR")).join("validate");
    fs::create_dir_all(&folder).expect("the temporary folder can be made");
    let path = folder.join(name);
    fs::write(&path, text).expect("the specification can be written");
    path
}

fn validate(path: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_citymap"))
        .arg("validate")
        .arg(path)
        .output()
        .expect("the program runs")
}

fn example_with(from: &str, to: &str) -> String {
    let example = fs::read_to_string(EXAMPLE).expect("the example is readable");
    let text = example.replace(from, to);
    assert_ne!(text, example, "the example has `{from}` to change");
    text
}

#[test]
fn a_valid_file_exits_with_zero_and_says_so() {
    let output = validate(Path::new(EXAMPLE));
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8(output.stdout).expect("stdout is text");
    assert!(stdout.ends_with("is valid\n"), "{stdout}");
}

#[test]
fn a_broken_rule_exits_with_two_and_names_the_field() {
    let path = file(
        "broken.yaml",
        &example_with("population: 6000", "population: 0"),
    );
    let output = validate(&path);
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).expect("stderr is text");
    assert!(stderr.contains("settlement.population"), "{stderr}");
}

#[test]
fn a_missing_file_exits_with_one() {
    let output = validate(Path::new("absent.yaml"));
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn a_warning_goes_to_stderr_and_keeps_exit_zero() {
    let path = file(
        "warning.yaml",
        &example_with(
            "  era: early_medieval\n",
            "  era: early_medieval\n  style: inc\n",
        ),
    );
    let output = validate(&path);
    assert_eq!(output.status.code(), Some(0));
    let stderr = String::from_utf8(output.stderr).expect("stderr is text");
    assert!(stderr.contains("warning: presets.style"), "{stderr}");
}
