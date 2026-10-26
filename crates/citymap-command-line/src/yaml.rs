//! The YAML facade: the only module that calls `serde_saphyr` (ADR-034).
//! Every other module reads and writes YAML through `from_text` and `to_text`, so the YAML
//! crate can be replaced here alone.

use serde::Serialize;
use serde::de::DeserializeOwned;

/// A place in YAML text: line and column, both counted from 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextPosition {
    /// The line, counted from 1.
    pub line: u64,
    /// The column, counted from 1.
    pub column: u64,
}

/// Why YAML text could not become a value, or a value could not become YAML text. It holds
/// the YAML crate's message and position as our own values, never the crate's error type.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum YamlError {
    /// The text is not valid YAML or does not match the expected type.
    #[error("{message}")]
    Reading {
        /// What went wrong, as the YAML crate words it, with a snippet of the text.
        message: String,
        /// Where in the text it went wrong; `None` when the YAML crate gives no place.
        position: Option<TextPosition>,
    },
    /// The value could not be written as YAML.
    #[error("{message}")]
    Writing {
        /// What went wrong, as the YAML crate words it.
        message: String,
    },
}

/// Reads one value of type `T` from YAML text; the error names the line and column.
///
/// # Errors
///
/// Returns `YamlError::Reading` when the text is not valid YAML or does not match `T`, for
/// example an unknown key, a duplicate key, a missing field or an unknown enum value.
pub fn from_text<T: DeserializeOwned>(text: &str) -> Result<T, YamlError> {
    serde_saphyr::from_str(text).map_err(|error| YamlError::Reading {
        message: error.to_string(),
        position: error.location().map(|location| TextPosition {
            line: location.line(),
            column: location.column(),
        }),
    })
}

/// Writes one value as YAML text that `from_text` reads back to an equal value.
///
/// # Errors
///
/// Returns `YamlError::Writing` when the value cannot be written as YAML.
pub fn to_text<T: Serialize>(value: &T) -> Result<String, YamlError> {
    serde_saphyr::to_string(value).map_err(|error| YamlError::Writing {
        message: error.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use citymap_core::random::Seed;
    use citymap_core::specification::{
        CompassRegion, Era, Identity, Importance, Presets, River, RiverCrossingKind, Road, RoadEnd,
        Settlement, Site, Specification, Water,
    };
    use citymap_core::units::Length;
    use proptest::prelude::*;

    use super::{TextPosition, YamlError, from_text, to_text};

    const HEADER: &str = "\
schema: 1
identity:
  name: Harrowmere
  seed: 20261019
  notes: Fictional river port used by the test suite.
settlement:
  population: 6000
site:
  extent_metres: 2400
  water:
    rivers:
      - identifier: ash
        from: north_west
        to: east
        width_metres: 18
roads:
  - identifier: north_road
    ends: [north, south]
    importance: major
    river_crossing: ford
  - identifier: west_road
    ends: [west, core]
    importance: minor
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
            site: Site {
                extent_metres: length(2400.0),
                water: Water {
                    rivers: vec![River {
                        identifier: "ash".to_string(),
                        from: CompassRegion::NorthWest,
                        to: CompassRegion::East,
                        width_metres: length(18.0),
                    }],
                },
            },
            roads: vec![
                Road {
                    identifier: "north_road".to_string(),
                    ends: [
                        RoadEnd::Region(CompassRegion::North),
                        RoadEnd::Region(CompassRegion::South),
                    ],
                    importance: Importance::Major,
                    river_crossing: RiverCrossingKind::Ford,
                },
                Road {
                    identifier: "west_road".to_string(),
                    ends: [RoadEnd::Region(CompassRegion::West), RoadEnd::Core],
                    importance: Importance::Minor,
                    river_crossing: RiverCrossingKind::Bridge,
                },
            ],
            presets: Presets {
                era: Era::EarlyMedieval,
            },
        }
    }

    fn length(value: f64) -> Length {
        Length::new(value).expect("test input is a valid non-negative length")
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
            (
                "  extent_metres: 2400\n",
                "  extent_metres: 2400\n  colour: red\n",
            ),
            ("    rivers:\n", "    colour: red\n    rivers:\n"),
            (
                "        width_metres: 18\n",
                "        width_metres: 18\n        colour: red\n",
            ),
            (
                "    importance: major\n",
                "    importance: major\n    colour: red\n",
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
    fn a_reading_error_carries_line_and_column() {
        let text = HEADER.replace(
            "schema: 1
",
            "schema: 1
colour: red
",
        );
        let error = read(&text).expect_err("an unknown key is an error");
        let YamlError::Reading { position, .. } = error else {
            panic!("reading text gives a reading error, got {error:?}");
        };
        assert_eq!(position, Some(TextPosition { line: 2, column: 1 }));
    }

    #[test]
    fn a_missing_required_field_is_rejected() {
        let lines = [
            "schema: 1\n",
            "  name: Harrowmere\n",
            "  seed: 20261019\n",
            "  population: 6000\n",
            "  extent_metres: 2400\n",
            "        from: north_west\n",
            "        width_metres: 18\n",
            "  - identifier: north_road\n",
            "    ends: [north, south]\n",
            "    importance: major\n",
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

    #[test]
    fn a_negative_river_width_is_rejected() {
        let text = HEADER.replace("width_metres: 18", "width_metres: -18");
        let error = read(&text).expect_err("a negative width is an error");
        assert!(error.to_string().contains("length is negative"), "{error}");
    }

    #[test]
    fn site_fields_outside_milestone_0b_are_rejected_by_name() {
        let text = HEADER.replace("  water:\n", "  terrain: {}\n  water:\n");
        let error = read(&text).expect_err("terrain is not read yet");
        assert!(
            error.to_string().contains("unknown field `terrain`"),
            "{error}"
        );
    }

    #[test]
    fn a_site_without_water_has_no_rivers() {
        let start = HEADER.find("  water:\n").expect("the header has water");
        let end = HEADER.find("roads:").expect("the header has roads");
        let text = format!("{}{}", &HEADER[..start], &HEADER[end..]);
        let specification = read(&text).expect("water is optional");
        assert_eq!(specification.site.water.rivers, Vec::<River>::new());
    }

    #[test]
    fn an_unknown_compass_region_is_rejected() {
        let text = HEADER.replace("from: north_west", "from: up_river");
        assert!(read(&text).is_err());
    }

    #[test]
    fn a_road_without_river_crossing_crosses_by_bridge() {
        let specification = read(HEADER).expect("the header is valid");
        assert_eq!(
            specification.roads[1].river_crossing,
            RiverCrossingKind::Bridge
        );
    }

    #[test]
    fn a_landmark_end_is_read_as_a_place() {
        let text = HEADER.replace("ends: [west, core]", "ends: [west, guildhall]");
        let specification = read(&text).expect("a place end reads; validation rejects it");
        assert_eq!(
            specification.roads[1].ends[1],
            RoadEnd::Place("guildhall".to_string())
        );
    }

    #[test]
    fn a_road_has_exactly_two_ends() {
        for ends in ["ends: [west]", "ends: [west, core, east]"] {
            let text = HEADER.replace("ends: [west, core]", ends);
            assert!(read(&text).is_err(), "{ends} must fail");
        }
    }

    #[test]
    fn roads_are_required() {
        let start = HEADER.find("roads:").expect("the header has roads");
        let end = HEADER.find("presets:").expect("the header has presets");
        let text = format!("{}{}", &HEADER[..start], &HEADER[end..]);
        let error = read(&text).expect_err("roads are required");
        assert!(error.to_string().contains("roads"), "{error}");
    }

    fn any_era() -> impl Strategy<Value = Era> {
        prop_oneof![
            Just(Era::EarlyMedieval),
            Just(Era::HighMedieval),
            Just(Era::LateMedieval),
        ]
    }

    fn any_compass_region() -> impl Strategy<Value = CompassRegion> {
        prop::sample::select(CompassRegion::ALL.to_vec())
    }

    fn any_road_end() -> impl Strategy<Value = RoadEnd> {
        prop_oneof![
            any_compass_region().prop_map(RoadEnd::Region),
            Just(RoadEnd::Core),
            "place_[a-z]{1,8}".prop_map(RoadEnd::Place),
        ]
    }

    fn any_road() -> impl Strategy<Value = Road> {
        (
            any::<String>(),
            any_road_end(),
            any_road_end(),
            prop_oneof![Just(Importance::Minor), Just(Importance::Major)],
            prop_oneof![
                Just(RiverCrossingKind::Bridge),
                Just(RiverCrossingKind::Ford)
            ],
        )
            .prop_map(
                |(identifier, first, second, importance, river_crossing)| Road {
                    identifier,
                    ends: [first, second],
                    importance,
                    river_crossing,
                },
            )
    }

    fn any_length() -> impl Strategy<Value = Length> {
        (0.0..=1e6_f64).prop_map(length)
    }

    fn any_river() -> impl Strategy<Value = River> {
        (
            any::<String>(),
            any_compass_region(),
            any_compass_region(),
            any_length(),
        )
            .prop_map(|(identifier, from, to, width_metres)| River {
                identifier,
                from,
                to,
                width_metres,
            })
    }

    fn any_site() -> impl Strategy<Value = Site> {
        (any_length(), prop::collection::vec(any_river(), 0..4)).prop_map(
            |(extent_metres, rivers)| Site {
                extent_metres,
                water: Water { rivers },
            },
        )
    }

    fn any_specification() -> impl Strategy<Value = Specification> {
        (
            any::<u32>(),
            any::<String>(),
            any::<u64>(),
            any::<String>(),
            any::<u32>(),
            any_site(),
            prop::collection::vec(any_road(), 1..4),
            any_era(),
        )
            .prop_map(
                |(schema, name, seed, notes, population, site, roads, era)| Specification {
                    schema,
                    identity: Identity {
                        name,
                        seed: Seed::new(seed),
                        notes,
                    },
                    settlement: Settlement { population },
                    site,
                    roads,
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
