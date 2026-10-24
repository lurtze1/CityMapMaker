//! The YAML facade: the only module that calls `serde_saphyr` (ADR-034).
//! Every other module reads and writes YAML through `from_text` and `to_text`, so the YAML
//! crate can be replaced here alone.

use std::error::Error;
use std::fmt;

use serde::Serialize;
use serde::de::DeserializeOwned;

/// Why YAML text could not become a value, or a value could not become YAML text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct YamlError {
    message: String,
}

impl fmt::Display for YamlError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for YamlError {}

/// Reads one value of type `T` from YAML text; the error names the line and column.
///
/// # Errors
///
/// Returns `YamlError` when the text is not valid YAML or does not match `T`, for example an
/// unknown key, a duplicate key, a missing field or an unknown enum value.
pub fn from_text<T: DeserializeOwned>(text: &str) -> Result<T, YamlError> {
    serde_saphyr::from_str(text).map_err(|error| YamlError {
        message: error.to_string(),
    })
}

/// Writes one value as YAML text that `from_text` reads back to an equal value.
///
/// # Errors
///
/// Returns `YamlError` when the value cannot be written as YAML.
pub fn to_text<T: Serialize>(value: &T) -> Result<String, YamlError> {
    serde_saphyr::to_string(value).map_err(|error| YamlError {
        message: error.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use citymap_core::rng::Seed;
    use citymap_core::specification::{Era, Identity, Presets, Settlement, Specification};
    use proptest::prelude::*;

    use super::{YamlError, from_text, to_text};

    const HEADER: &str = "\
schema: 1
identity:
  name: Harrowmere
  seed: 20261019
  notes: Fictional river port used by the test suite.
settlement:
  population: 6000
presets:
  era: early_medieval
";

    fn harrowmere() -> Specification {
        Specification {
            schema: 1,
            identity: Identity {
                name: "Harrowmere".to_string(),
                seed: Seed::new(20_261_019),
                notes: "Fictional river port used by the test suite.".to_string(),
            },
            settlement: Settlement { population: 6000 },
            presets: Presets {
                era: Era::EarlyMedieval,
            },
        }
    }

    fn read(text: &str) -> Result<Specification, YamlError> {
        from_text(text)
    }

    #[test]
    fn the_example_header_reads_to_the_expected_value() {
        assert_eq!(read(HEADER), Ok(harrowmere()));
    }

    #[test]
    fn notes_default_to_empty() {
        let text = HEADER.replace(
            "  notes: Fictional river port used by the test suite.\n",
            "",
        );
        let specification = read(&text).expect("a header without notes is valid");
        assert_eq!(specification.identity.notes, "");
    }

    #[test]
    fn an_unknown_key_is_rejected_at_every_level() {
        let cases = [
            ("schema: 1\n", "schema: 1\ncolour: red\n"),
            (
                "  name: Harrowmere\n",
                "  name: Harrowmere\n  colour: red\n",
            ),
            (
                "  population: 6000\n",
                "  population: 6000\n  colour: red\n",
            ),
            (
                "  era: early_medieval\n",
                "  era: early_medieval\n  colour: red\n",
            ),
        ];
        for (line, with_unknown_key) in cases {
            let text = HEADER.replace(line, with_unknown_key);
            let error = read(&text).expect_err("an unknown key is an error");
            assert!(
                error.to_string().contains("unknown field `colour`"),
                "{error}"
            );
        }
    }

    #[test]
    fn a_missing_required_field_is_rejected() {
        let lines = [
            "schema: 1\n",
            "  name: Harrowmere\n",
            "  seed: 20261019\n",
            "  population: 6000\n",
            "  era: early_medieval\n",
        ];
        for line in lines {
            let text = HEADER.replace(line, "");
            assert!(read(&text).is_err(), "removing {line:?} must fail");
        }
    }

    #[test]
    fn a_duplicate_key_is_rejected() {
        let text = HEADER.replace(
            "  population: 6000\n",
            "  population: 6000\n  population: 9\n",
        );
        assert!(read(&text).is_err());
    }

    #[test]
    fn an_unknown_era_is_rejected() {
        let text = HEADER.replace("early_medieval", "bronze_age");
        assert!(read(&text).is_err());
    }

    fn any_era() -> impl Strategy<Value = Era> {
        prop_oneof![
            Just(Era::EarlyMedieval),
            Just(Era::HighMedieval),
            Just(Era::LateMedieval),
        ]
    }

    fn any_specification() -> impl Strategy<Value = Specification> {
        (
            any::<u32>(),
            any::<String>(),
            any::<u64>(),
            any::<String>(),
            any::<u32>(),
            any_era(),
        )
            .prop_map(
                |(schema, name, seed, notes, population, era)| Specification {
                    schema,
                    identity: Identity {
                        name,
                        seed: Seed::new(seed),
                        notes,
                    },
                    settlement: Settlement { population },
                    presets: Presets { era },
                },
            )
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(64))]

        #[test]
        fn a_specification_survives_yaml_and_back(specification in any_specification()) {
            let text = to_text(&specification).expect("a specification always writes");
            prop_assert_eq!(read(&text), Ok(specification));
        }
    }
}
