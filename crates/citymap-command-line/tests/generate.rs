//! The `generate` command end to end: real files in, real files out, in the temporary folder
//! Cargo gives integration tests.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

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
        png: false,
    }
}

#[test]
fn the_example_becomes_an_svg_named_after_the_file_and_seed() {
    let output = folder("example");
    let files = generate(&arguments(Path::new(EXAMPLE), None, &output))
        .expect("the example works")
        .files;
    assert_eq!(files, [output.join("example-seed-1.svg")]);
    let text = fs::read_to_string(&files[0]).expect("the file was written");
    assert!(text.starts_with("<svg"), "{text}");
}

#[test]
fn the_example_gives_the_core_notice() {
    let output = folder("notice");
    let generated =
        generate(&arguments(Path::new(EXAMPLE), None, &output)).expect("the example works");
    assert_eq!(generated.notices.len(), 1, "{:?}", generated.notices);
    assert!(
        generated.notices[0].contains("`market_road.ash`"),
        "{:?}",
        generated.notices
    );
    assert!(
        generated.notices[0].contains("`market_road+north_road`"),
        "{:?}",
        generated.notices
    );
}

#[test]
fn the_seed_argument_replaces_the_specification_seed_in_the_name() {
    let output = folder("seed");
    let files = generate(&arguments(Path::new(EXAMPLE), Some(42), &output))
        .expect("it works")
        .files;
    assert_eq!(files, [output.join("example-seed-42.svg")]);
}

#[test]
fn the_png_flag_also_writes_a_png() {
    let output = folder("png");
    let request = GenerateArguments {
        png: true,
        ..arguments(Path::new(EXAMPLE), None, &output)
    };
    let files = generate(&request).expect("the example works").files;
    assert_eq!(
        files,
        [
            output.join("example-seed-1.svg"),
            output.join("example-seed-1.png")
        ]
    );
    let png = fs::read(&files[1]).expect("the file was written");
    assert_eq!(png[..4], [0x89, b'P', b'N', b'G']);
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

#[test]
fn a_specification_that_breaks_rules_lists_them_and_is_invalid_input() {
    let output = folder("rules");
    fs::create_dir_all(&output).expect("the folder can be made");
    let text = fs::read_to_string(EXAMPLE)
        .expect("the example reads")
        .replace("population: 6000", "population: 0")
        .replace("ends: [west, core]", "ends: [west, guildhall]");
    let specification = output.join("rules.yaml");
    fs::write(&specification, text).expect("the file can be written");
    let error = generate(&arguments(&specification, None, &output)).expect_err("it breaks rules");
    let CommandLineError::BrokenRules { errors, .. } = &error else {
        panic!("expected broken rules, got {error}");
    };
    assert_eq!(errors.len(), 2, "{error}");
    assert!(error.to_string().contains("west_road"), "{error}");
    assert_eq!(error.exit_code(), 2);
}

#[test]
fn an_output_folder_that_is_a_file_is_a_writing_error() {
    let output = folder("blocked");
    fs::create_dir_all(&output).expect("the folder can be made");
    let blocker = output.join("blocker");
    fs::write(&blocker, "").expect("the file can be written");
    let error = generate(&arguments(Path::new(EXAMPLE), None, &blocker))
        .expect_err("a file cannot hold other files");
    assert!(matches!(error, CommandLineError::Writing { .. }), "{error}");
    assert_eq!(error.exit_code(), 1);
}

#[test]
fn an_unknown_label_warns_and_a_road_to_a_pinned_landmark_gets_a_notice() {
    let output = folder("landmark");
    fs::create_dir_all(&output).expect("the temporary folder can be made");
    let example = fs::read_to_string(EXAMPLE).expect("the example is readable");
    let road = example.replace("ends: [west, core]", "ends: [west, well]");
    assert_ne!(road, example, "the example has the road to change");
    let text = road
        + concat!(
            "landmarks:\n",
            "  - identifier: well\n",
            "    name: The Well\n",
            "    function: utility\n",
            "    label: wel\n",
            "    resolved: { x: 600, y: 1200, rotation_degrees: 0, width_metres: 3, depth_metres: 3 }\n",
            "    pinned: true\n",
        );
    let specification = output.join("landmark.yaml");
    fs::write(&specification, text).expect("the specification can be written");
    let generated =
        generate(&arguments(&specification, None, &output)).expect("a warning is no error");
    assert_eq!(generated.warnings.len(), 1, "{:?}", generated.warnings);
    assert!(
        generated.warnings[0].contains("`wel`"),
        "{:?}",
        generated.warnings
    );
    assert!(
        generated.notices.iter().any(|notice| notice
            .contains("road `west_road` ends at the pinned landmark or open space `well`")),
        "{:?}",
        generated.notices
    );
    let path = specification.to_str().expect("the temporary path is text");
    let folder_text = output.to_str().expect("the temporary path is text");
    let program = run(&["generate", path, "--output", folder_text]);
    assert_eq!(program.status.code(), Some(0));
    let stderr = String::from_utf8(program.stderr).expect("stderr is text");
    assert!(stderr.contains("\nwarning: landmarks[0].label"), "{stderr}");
}

#[test]
fn an_unknown_style_and_pack_warn() {
    let output = folder("presets");
    fs::create_dir_all(&output).expect("the temporary folder can be made");
    let example = fs::read_to_string(EXAMPLE).expect("the example is readable");
    let text = example.replace(
        "  era: early_medieval\n",
        "  era: early_medieval\n  style: inc\n  packs: [jotun]\n",
    );
    assert_ne!(text, example, "the example has the era to change");
    let specification = output.join("presets.yaml");
    fs::write(&specification, text).expect("the specification can be written");
    let path = specification.to_str().expect("the temporary path is text");
    let folder_text = output.to_str().expect("the temporary path is text");
    let program = run(&["generate", path, "--output", folder_text]);
    assert_eq!(program.status.code(), Some(0));
    let stderr = String::from_utf8(program.stderr).expect("stderr is text");
    assert!(stderr.contains("\nwarning: presets.style"), "{stderr}");
    assert!(stderr.contains("\nwarning: presets.packs[0]"), "{stderr}");
}

#[test]
fn a_file_with_errors_also_shows_its_warnings() {
    let output = folder("errors_and_warnings");
    fs::create_dir_all(&output).expect("the temporary folder can be made");
    let example = fs::read_to_string(EXAMPLE).expect("the example is readable");
    let text = example.replace("population: 6000", "population: 0")
        + "landmarks:\n  - { identifier: well, name: The Well, function: utility, label: wel }\n";
    let specification = output.join("broken.yaml");
    fs::write(&specification, text).expect("the specification can be written");
    let error = generate(&arguments(&specification, None, &output))
        .expect_err("a population of 0 breaks a rule");
    let message = error.to_string();
    assert!(message.contains("settlement.population"), "{message}");
    assert!(
        message.contains("\nwarning: landmarks[0].label"),
        "{message}"
    );
    assert_eq!(error.exit_code(), 2);
}

fn run(arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_citymap"))
        .args(arguments)
        .output()
        .expect("the program runs")
}

#[test]
fn the_program_exits_with_the_codes_of_requirement_f20() {
    let output = folder("program");
    let output_text = output.to_str().expect("the temporary path is text");
    let success = run(&["generate", EXAMPLE, "--output", output_text]);
    assert_eq!(success.status.code(), Some(0));
    let notices = String::from_utf8(success.stderr).expect("stderr is text");
    assert!(notices.starts_with("notice: the town core"), "{notices}");
    let missing = run(&["generate", "absent.yaml", "--output", output_text]);
    assert_eq!(missing.status.code(), Some(1));
    let invalid = run(&["generate", "--bogus"]);
    assert_eq!(invalid.status.code(), Some(2));
    let message = String::from_utf8(invalid.stderr).expect("stderr is text");
    assert!(
        message.starts_with("error: unexpected argument"),
        "{message}"
    );
    assert!(!message.contains("error: error:"), "{message}");
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn two_runs_write_the_same_bytes(seed in any::<u64>()) {
        let first = folder("first");
        let second = folder("second");
        let example = Path::new(EXAMPLE);
        let first = generate(&arguments(example, Some(seed), &first)).expect("it works").files;
        let second = generate(&arguments(example, Some(seed), &second)).expect("it works").files;
        prop_assert_eq!(first.len(), second.len());
        for (first, second) in first.iter().zip(&second) {
            prop_assert_eq!(fs::read(first).expect("written"), fs::read(second).expect("written"));
        }
    }
}
