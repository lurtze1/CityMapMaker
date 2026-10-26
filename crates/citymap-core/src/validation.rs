//! Validation: the rules of a specification that its types cannot hold, such as ranges and
//! rules between fields (specification-version-1 §5). It returns every error it finds, not
//! only the first, and each error names the field it is about (ADR-046).

use std::collections::BTreeSet;

use crate::specification::{CompassRegion, RoadEnd, Specification};

/// The only schema version that exists.
pub const SCHEMA_VERSION: u32 = 1;

/// The longest city name, in characters.
pub const MAXIMUM_NAME_CHARACTERS: usize = 80;

/// The largest population the generator accepts.
pub const MAXIMUM_POPULATION: u32 = 1_000_000;

/// The smallest and largest side of the site, in metres.
pub const EXTENT_RANGE_METRES: (f64, f64) = (500.0, 5000.0);

/// One broken rule of a specification; the message starts with the field it is about.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ValidationError {
    /// The file is for a schema version this generator does not read.
    #[error("schema: version {found} is unknown; the only version is 1")]
    UnknownSchema {
        /// The version in the file.
        found: u32,
    },
    /// The city's name is empty or longer than `MAXIMUM_NAME_CHARACTERS`.
    #[error("identity.name: {characters} characters; a name has 1 to 80")]
    NameLength {
        /// How many characters the name has.
        characters: usize,
    },
    /// The population is zero or above `MAXIMUM_POPULATION`.
    #[error("settlement.population: {population} is outside 1 to 1,000,000")]
    PopulationOutOfRange {
        /// The population in the file.
        population: u32,
    },
    /// The site's side is outside `EXTENT_RANGE_METRES`.
    #[error("site.extent_metres: {extent} is outside 500 to 5000")]
    ExtentOutOfRange {
        /// The side in the file, in metres.
        extent: String,
    },
    /// Two objects share an identifier, in one list or in two.
    #[error("identifier `{identifier}` is used twice")]
    DuplicateIdentifier {
        /// The identifier used twice.
        identifier: String,
    },
    /// An identifier is a word the file uses for something else, or contains a `.`.
    #[error(
        "identifier `{identifier}` is reserved: `core`, `centre`, compass regions and names with `.` are"
    )]
    ReservedIdentifier {
        /// The reserved identifier.
        identifier: String,
    },
    /// A river is zero metres wide.
    #[error("site.water.rivers `{river}`: width_metres is 0; a river is wider than that")]
    ZeroRiverWidth {
        /// The river's identifier.
        river: String,
    },
    /// A river starts or ends at `centre`, which is not an edge (until #130).
    #[error("site.water.rivers `{river}`: `centre` is not an edge stretch")]
    RiverEndAtCentre {
        /// The river's identifier.
        river: String,
    },
    /// A river enters and leaves at the same edge stretch (until #131).
    #[error("site.water.rivers `{river}`: `from` and `to` are the same stretch")]
    RiverEndsEqual {
        /// The river's identifier.
        river: String,
    },
    /// The file has no road.
    #[error("roads: a city needs at least one road")]
    NoRoads,
    /// Both ends of a road are `core`.
    #[error("roads `{road}`: both ends are `core`; at most one end is")]
    BothRoadEndsCore {
        /// The road's identifier.
        road: String,
    },
    /// Both ends of a road are the same compass region.
    #[error("roads `{road}`: both ends are the same")]
    RoadEndsEqual {
        /// The road's identifier.
        road: String,
    },
    /// A road ends at `centre`, which is not an edge.
    #[error("roads `{road}`: `centre` is not an edge stretch")]
    RoadEndAtCentre {
        /// The road's identifier.
        road: String,
    },
    /// A road ends at a landmark or open space, which milestone 0b does not have yet.
    #[error("roads `{road}`: end `{place}` is a landmark or open space; milestone 0b has none")]
    RoadEndAtPlace {
        /// The road's identifier.
        road: String,
        /// The end as written in the file.
        place: String,
    },
}

/// Checks every rule of specification-version-1 §5 that milestone 0b's fields can break.
///
/// # Errors
///
/// Every broken rule, in the order of the file: header fields, rivers, then roads.
pub fn validate(specification: &Specification) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    if specification.schema != SCHEMA_VERSION {
        errors.push(ValidationError::UnknownSchema {
            found: specification.schema,
        });
    }
    let characters = specification.identity.name.chars().count();
    if characters == 0 || characters > MAXIMUM_NAME_CHARACTERS {
        errors.push(ValidationError::NameLength { characters });
    }
    let population = specification.settlement.population;
    if population == 0 || population > MAXIMUM_POPULATION {
        errors.push(ValidationError::PopulationOutOfRange { population });
    }
    let extent = specification.site.extent_metres.get();
    if extent < EXTENT_RANGE_METRES.0 || extent > EXTENT_RANGE_METRES.1 {
        errors.push(ValidationError::ExtentOutOfRange {
            extent: extent.to_string(),
        });
    }
    check_identifiers(specification, &mut errors);
    check_rivers(specification, &mut errors);
    check_roads(specification, &mut errors);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn check_identifiers(specification: &Specification, errors: &mut Vec<ValidationError>) {
    let rivers = specification
        .site
        .water
        .rivers
        .iter()
        .map(|river| &river.identifier);
    let roads = specification.roads.iter().map(|road| &road.identifier);
    let mut seen = BTreeSet::new();
    for identifier in rivers.chain(roads) {
        if !seen.insert(identifier) {
            errors.push(ValidationError::DuplicateIdentifier {
                identifier: identifier.clone(),
            });
        }
        let is_region = CompassRegion::ALL
            .into_iter()
            .any(|region| region.name() == identifier);
        if identifier == "core" || is_region || identifier.contains('.') {
            errors.push(ValidationError::ReservedIdentifier {
                identifier: identifier.clone(),
            });
        }
    }
}

fn check_rivers(specification: &Specification, errors: &mut Vec<ValidationError>) {
    for river in &specification.site.water.rivers {
        let named = || river.identifier.clone();
        if river.width_metres.get() == 0.0 {
            errors.push(ValidationError::ZeroRiverWidth { river: named() });
        }
        if river.from == CompassRegion::Centre || river.to == CompassRegion::Centre {
            errors.push(ValidationError::RiverEndAtCentre { river: named() });
        }
        if river.from == river.to {
            errors.push(ValidationError::RiverEndsEqual { river: named() });
        }
    }
}

fn check_roads(specification: &Specification, errors: &mut Vec<ValidationError>) {
    if specification.roads.is_empty() {
        errors.push(ValidationError::NoRoads);
    }
    for road in &specification.roads {
        let named = || road.identifier.clone();
        let [first, second] = &road.ends;
        if *first == RoadEnd::Core && *second == RoadEnd::Core {
            errors.push(ValidationError::BothRoadEndsCore { road: named() });
        } else if first == second {
            errors.push(ValidationError::RoadEndsEqual { road: named() });
        }
        for end in &road.ends {
            match end {
                RoadEnd::Region(CompassRegion::Centre) => {
                    errors.push(ValidationError::RoadEndAtCentre { road: named() });
                }
                RoadEnd::Place(place) => errors.push(ValidationError::RoadEndAtPlace {
                    road: named(),
                    place: place.clone(),
                }),
                RoadEnd::Region(_) | RoadEnd::Core => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::random::Seed;
    use crate::specification::{
        Era, Identity, Importance, Presets, River, RiverCrossingKind, Road, Settlement, Site, Water,
    };
    use crate::units::Length;

    fn length(value: f64) -> Length {
        Length::new(value).expect("test input is a valid length")
    }

    fn river(identifier: &str, from: CompassRegion, to: CompassRegion) -> River {
        River {
            identifier: identifier.to_string(),
            from,
            to,
            width_metres: length(18.0),
        }
    }

    fn road(identifier: &str, first: RoadEnd, second: RoadEnd) -> Road {
        Road {
            identifier: identifier.to_string(),
            ends: [first, second],
            importance: Importance::Major,
            river_crossing: RiverCrossingKind::Bridge,
        }
    }

    fn valid() -> Specification {
        Specification {
            schema: 1,
            identity: Identity {
                name: "Harrowmere".to_string(),
                seed: Seed::new(1),
                notes: String::new(),
            },
            settlement: Settlement { population: 6000 },
            site: Site {
                extent_metres: length(2400.0),
                water: Water {
                    rivers: vec![river("ash", CompassRegion::NorthWest, CompassRegion::East)],
                },
            },
            roads: vec![
                road(
                    "north_road",
                    RoadEnd::Region(CompassRegion::North),
                    RoadEnd::Region(CompassRegion::South),
                ),
                road(
                    "west_road",
                    RoadEnd::Region(CompassRegion::West),
                    RoadEnd::Core,
                ),
            ],
            presets: Presets {
                era: Era::EarlyMedieval,
            },
        }
    }

    fn only_error(specification: &Specification) -> ValidationError {
        let errors = validate(specification).expect_err("the specification breaks a rule");
        assert_eq!(errors.len(), 1, "{errors:?}");
        errors[0].clone()
    }

    #[test]
    fn the_valid_example_passes() {
        assert_eq!(validate(&valid()), Ok(()));
    }

    #[test]
    fn an_unknown_schema_is_an_error() {
        let mut specification = valid();
        specification.schema = 2;
        assert_eq!(
            only_error(&specification),
            ValidationError::UnknownSchema { found: 2 }
        );
    }

    #[test]
    fn a_name_is_one_to_eighty_characters() {
        for (name, characters) in [(String::new(), 0), ("a".repeat(81), 81)] {
            let mut specification = valid();
            specification.identity.name = name;
            assert_eq!(
                only_error(&specification),
                ValidationError::NameLength { characters }
            );
        }
    }

    #[test]
    fn population_is_one_to_a_million() {
        for population in [0, 1_000_001] {
            let mut specification = valid();
            specification.settlement.population = population;
            assert_eq!(
                only_error(&specification),
                ValidationError::PopulationOutOfRange { population }
            );
        }
    }

    #[test]
    fn extent_is_five_hundred_to_five_thousand_metres() {
        for extent in [499.99, 5000.01] {
            let mut specification = valid();
            specification.site.extent_metres = length(extent);
            assert!(matches!(
                only_error(&specification),
                ValidationError::ExtentOutOfRange { .. }
            ));
        }
    }

    #[test]
    fn an_identifier_used_by_a_river_and_a_road_is_a_duplicate() {
        let mut specification = valid();
        specification.roads[0].identifier = "ash".to_string();
        assert_eq!(
            only_error(&specification),
            ValidationError::DuplicateIdentifier {
                identifier: "ash".to_string()
            }
        );
    }

    #[test]
    fn reserved_words_and_dots_are_not_identifiers() {
        for identifier in ["core", "centre", "north_east", "north_road.ash"] {
            let mut specification = valid();
            specification.roads[0].identifier = identifier.to_string();
            assert_eq!(
                only_error(&specification),
                ValidationError::ReservedIdentifier {
                    identifier: identifier.to_string()
                }
            );
        }
    }

    #[test]
    fn a_river_is_wider_than_zero() {
        let mut specification = valid();
        specification.site.water.rivers[0].width_metres = length(0.0);
        assert_eq!(
            only_error(&specification),
            ValidationError::ZeroRiverWidth {
                river: "ash".to_string()
            }
        );
    }

    #[test]
    fn a_river_does_not_end_at_the_centre() {
        let mut specification = valid();
        specification.site.water.rivers[0].to = CompassRegion::Centre;
        assert_eq!(
            only_error(&specification),
            ValidationError::RiverEndAtCentre {
                river: "ash".to_string()
            }
        );
    }

    #[test]
    fn a_river_leaves_at_another_stretch() {
        let mut specification = valid();
        specification.site.water.rivers[0].to = CompassRegion::NorthWest;
        assert_eq!(
            only_error(&specification),
            ValidationError::RiverEndsEqual {
                river: "ash".to_string()
            }
        );
    }

    #[test]
    fn a_city_needs_a_road() {
        let mut specification = valid();
        specification.roads.clear();
        assert_eq!(only_error(&specification), ValidationError::NoRoads);
    }

    #[test]
    fn a_road_has_at_most_one_core_end() {
        let mut specification = valid();
        specification.roads[1].ends = [RoadEnd::Core, RoadEnd::Core];
        assert_eq!(
            only_error(&specification),
            ValidationError::BothRoadEndsCore {
                road: "west_road".to_string()
            }
        );
    }

    #[test]
    fn a_road_has_two_different_ends() {
        let mut specification = valid();
        specification.roads[0].ends[1] = RoadEnd::Region(CompassRegion::North);
        assert_eq!(
            only_error(&specification),
            ValidationError::RoadEndsEqual {
                road: "north_road".to_string()
            }
        );
    }

    #[test]
    fn a_road_does_not_end_at_the_centre() {
        let mut specification = valid();
        specification.roads[1].ends[1] = RoadEnd::Region(CompassRegion::Centre);
        assert_eq!(
            only_error(&specification),
            ValidationError::RoadEndAtCentre {
                road: "west_road".to_string()
            }
        );
    }

    #[test]
    fn a_road_end_at_a_landmark_names_the_road() {
        let mut specification = valid();
        specification.roads[1].ends[1] = RoadEnd::Place("guildhall".to_string());
        let error = only_error(&specification);
        assert_eq!(
            error,
            ValidationError::RoadEndAtPlace {
                road: "west_road".to_string(),
                place: "guildhall".to_string()
            }
        );
        assert!(error.to_string().contains("west_road"), "{error}");
    }

    #[test]
    fn every_error_is_reported_not_only_the_first() {
        let mut specification = valid();
        specification.schema = 2;
        specification.settlement.population = 0;
        specification.roads.clear();
        let errors = validate(&specification).expect_err("three rules are broken");
        assert_eq!(errors.len(), 3, "{errors:?}");
    }

    fn any_edge() -> impl Strategy<Value = CompassRegion> {
        let edges: Vec<CompassRegion> = CompassRegion::ALL
            .into_iter()
            .filter(|region| *region != CompassRegion::Centre)
            .collect();
        prop::sample::select(edges)
    }

    fn any_two_edges() -> impl Strategy<Value = (CompassRegion, CompassRegion)> {
        (any_edge(), any_edge())
            .prop_filter("two different stretches", |(first, second)| first != second)
    }

    fn any_valid_specification() -> impl Strategy<Value = Specification> {
        (
            "[A-Za-z ]{1,80}",
            1..=MAXIMUM_POPULATION,
            500.0..=5000.0_f64,
            any_two_edges(),
            any_two_edges(),
        )
            .prop_map(|(name, population, extent, river_ends, road_ends)| {
                let mut specification = valid();
                specification.identity.name = name;
                specification.settlement.population = population;
                specification.site.extent_metres = length(extent);
                specification.site.water.rivers[0].from = river_ends.0;
                specification.site.water.rivers[0].to = river_ends.1;
                specification.roads[0].ends =
                    [RoadEnd::Region(road_ends.0), RoadEnd::Region(road_ends.1)];
                specification
            })
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(64))]

        #[test]
        fn a_specification_within_every_rule_passes(specification in any_valid_specification()) {
            prop_assert_eq!(validate(&specification), Ok(()));
        }
    }
}
