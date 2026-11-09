//! The `schema` command and the committed schema file.
//!
//! The file matches the types; the schema accepts the example, an empty optional key and each
//! rule and gate kind with its needed fields, and rejects a rule or gate with the wrong keys.

use std::fs;
use std::process::Command;

use citymap_command_line::schema::specification_schema;
use citymap_command_line::yaml;
use serde_json::Value;

const SCHEMA_FILE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../documentation/specification.schema.json"
);

const EXAMPLE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../specifications/example.yaml"
);

fn validator() -> jsonschema::Validator {
    let schema: Value = serde_json::from_str(&specification_schema()).expect("the schema is JSON");
    jsonschema::draft202012::new(&schema).expect("the schema is a valid draft 2020-12 schema")
}

fn example_with(from: &str, to: &str) -> Value {
    let example = fs::read_to_string(EXAMPLE).expect("the example is readable");
    let text = example.replace(from, to);
    assert_ne!(text, example, "the example has `{from}` to change");
    yaml::from_text(&text).expect("the changed example is still YAML")
}

#[test]
fn the_committed_schema_file_matches_the_types() {
    let committed = fs::read_to_string(SCHEMA_FILE).unwrap_or_default();
    assert!(
        committed == specification_schema(),
        "documentation/specification.schema.json is stale; in Git Bash or cmd run \
         `cargo run -q -p citymap-command-line -- schema > documentation/specification.schema.json` \
         (PowerShell 5.1 would add a byte order mark)"
    );
}

#[test]
fn the_program_prints_the_schema() {
    let output = Command::new(env!("CARGO_BIN_EXE_citymap"))
        .arg("schema")
        .output()
        .expect("the program runs");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8(output.stdout).expect("stdout is text"),
        specification_schema()
    );
}

#[test]
fn the_example_passes_the_schema() {
    let example: Value = yaml::from_text(&fs::read_to_string(EXAMPLE).expect("readable"))
        .expect("the example is YAML");
    let errors: Vec<String> = validator()
        .iter_errors(&example)
        .map(|error| error.to_string())
        .collect();
    assert!(errors.is_empty(), "{errors:#?}");
}

#[test]
fn the_schema_rejects_what_a_rule_or_gate_does_not_take() {
    let cases = [
        ("a rule without the field it needs", "{ rule: near }"),
        (
            "a rule with a field of another rule",
            "{ rule: min_spacing, metres: 10, target: market }",
        ),
        ("an unknown rule", "{ rule: nearby, target: market }"),
        (
            "a bearing above 359",
            "{ rule: compass_region, region: 400 }",
        ),
    ];
    for (case, rule) in cases {
        let specification = example_with(
            "    weight: 1\n",
            &format!("    weight: 1\n    constraints:\n      - {rule}\n"),
        );
        assert!(!validator().is_valid(&specification), "{case}: {rule}");
    }
}

#[test]
fn the_schema_accepts_each_rule_with_only_its_needed_fields() {
    let rules = [
        "{ rule: compass_region, region: north }",
        "{ rule: near, target: market, weight: 2 }",
        "{ rule: on_water, hard: true }",
        "{ rule: min_spacing, metres: 10 }",
    ];
    for rule in rules {
        let specification = example_with(
            "    weight: 1\n",
            &format!("    weight: 1\n    constraints:\n      - {rule}\n"),
        );
        let errors: Vec<String> = validator()
            .iter_errors(&specification)
            .map(|error| error.to_string())
            .collect();
        assert!(errors.is_empty(), "{rule}: {errors:#?}");
    }
}

#[test]
fn an_unknown_key_fails_the_schema() {
    let specification = example_with(
        "  population: 6000\n",
        "  population: 6000\n  colour: red\n",
    );
    assert!(!validator().is_valid(&specification));
}

#[test]
fn an_empty_optional_key_passes_the_schema() {
    for (from, to) in [
        ("presets:\n", "landmarks:\npresets:\n"),
        (
            "  era: early_medieval\n",
            "  era: early_medieval\n  packs:\n",
        ),
    ] {
        let specification = example_with(from, to);
        let errors: Vec<String> = validator()
            .iter_errors(&specification)
            .map(|error| error.to_string())
            .collect();
        assert!(errors.is_empty(), "{to}: {errors:#?}");
    }
}

#[test]
fn the_schema_checks_each_gate_kind() {
    let walls = "defences:\n  walls:\n    - { identifier: town_wall, kind: stone, encloses: [old_town] }\n  gates:\n";
    let accepted = [
        "{ kind: road_gate, identifier: north_gate, wall: town_wall, road: north_road }",
        "{ kind: postern, identifier: small_gate, wall: town_wall, bearing_degrees: east }",
        "{ kind: water_gate, identifier: quay_gate, wall: town_wall, bearing_degrees: 90 }",
    ];
    let rejected = [
        "{ kind: road_gate, identifier: north_gate, wall: town_wall }",
        "{ kind: postern, identifier: small_gate, wall: town_wall, road: north_road }",
        "{ kind: sally_port, identifier: small_gate, wall: town_wall }",
    ];
    for (gates, valid) in [(accepted, true), (rejected, false)] {
        for gate in gates {
            let specification =
                example_with("presets:\n", &format!("{walls}    - {gate}\npresets:\n"));
            assert_eq!(validator().is_valid(&specification), valid, "{gate}");
        }
    }
}
