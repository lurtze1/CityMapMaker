//! Validation: the rules of a specification that its types cannot hold, such as ranges and
//! rules between fields (specification-version-1 §5). It returns every error it finds, not
//! only the first, and each error names the field it is about (ADR-046).

use std::collections::BTreeSet;
use std::ops::RangeInclusive;

use crate::specification::{Bearing, CORE_WORD, CoastShape, CompassRegion, RoadEnd, Specification};

/// The only schema version that exists.
pub const SCHEMA_VERSION: u32 = 1;

/// The allowed length of the city's name, in characters.
pub const NAME_CHARACTERS: RangeInclusive<usize> = 1..=80;

/// The allowed population.
pub const POPULATION: RangeInclusive<u32> = 1..=1_000_000;

/// The allowed side of the site, in metres.
pub const EXTENT_METRES: RangeInclusive<f64> = 500.0..=5000.0;

/// The field path of the rivers list.
pub const RIVERS: &str = "site.water.rivers";

/// The field path of the terrain.
pub const TERRAIN: &str = "site.terrain";

/// How far, in degrees, a river's end may lie from an estuary's bearing.
pub const ESTUARY_RIVER_DEGREES: u16 = 45;

/// The field path of the roads list.
pub const ROADS: &str = "roads";

/// One broken rule of a specification; the message starts with the field it is about.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ValidationError {
    /// The file is for a schema version this generator does not read.
    #[error("schema: version {found} is unknown; the only version is {SCHEMA_VERSION}")]
    UnknownSchema {
        /// The version in the file.
        found: u32,
    },
    /// The city's name has a length outside `NAME_CHARACTERS`.
    #[error("identity.name: {characters} characters; a name has {NAME_CHARACTERS:?}")]
    NameLength {
        /// How many characters the name has.
        characters: usize,
    },
    /// The population is outside `POPULATION`.
    #[error("settlement.population: {population} is outside {POPULATION:?}")]
    PopulationOutOfRange {
        /// The population in the file.
        population: u32,
    },
    /// The site's side is outside `EXTENT_METRES`.
    #[error("site.extent_metres: {extent} is outside {EXTENT_METRES:?}")]
    ExtentOutOfRange {
        /// The side in the file, in metres.
        extent: String,
    },
    /// Two objects share an identifier, in one list or in two.
    #[error("{list}: identifier `{identifier}` is used more than once")]
    DuplicateIdentifier {
        /// The field path of the list where it is used again.
        list: &'static str,
        /// The identifier used more than once.
        identifier: String,
    },
    /// An identifier is a word the file uses for something else, or contains a `.`.
    #[error("{list}: identifier `{identifier}` is reserved (`core`, compass regions, a `.`)")]
    ReservedIdentifier {
        /// The field path of the list it is in.
        list: &'static str,
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
    /// A size that must be more than zero is zero.
    #[error("{field}: 0; this size is more than 0")]
    ZeroSize {
        /// The field path of the size.
        field: String,
    },
    /// A bearing in degrees is above `Bearing::MAXIMUM_DEGREES`.
    #[error(
        "{field}: {degrees} degrees; a bearing is 0 to {}",
        Bearing::MAXIMUM_DEGREES
    )]
    BearingOutOfRange {
        /// The field path of the bearing.
        field: String,
        /// The degrees in the file.
        degrees: u16,
    },
    /// A bearing is the word `centre`, which has no direction.
    #[error("{field}: `centre` has no direction")]
    BearingAtCentre {
        /// The field path of the bearing.
        field: String,
    },
    /// An estuary has no river that reaches the sea at its coast.
    #[error(
        "site.terrain.coast: an estuary needs a river whose `to` lies within {ESTUARY_RIVER_DEGREES} degrees of the coast's bearing"
    )]
    EstuaryWithoutRiver,
    /// The file has no road.
    #[error("roads: a city needs at least one road")]
    NoRoads,
    /// Both ends of a road are `core`.
    #[error("roads `{road}`: both ends are `core`; at most one end is")]
    BothRoadEndsCore {
        /// The road's identifier.
        road: String,
    },
    /// Both ends of a road are the same.
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
    /// A road end names something other than a compass region or `core`.
    #[error(
        "roads `{road}`: end `{place}` names no pinned landmark or open space; milestone 0b has none"
    )]
    RoadEndAtPlace {
        /// The road's identifier.
        road: String,
        /// The end as written in the file.
        place: String,
    },
}

/// Checks every rule of specification-version-1 §5 and ADR-056 that the fields read so far
/// can break.
///
/// # Errors
///
/// Every broken rule: header fields, identifiers, terrain, rivers, then roads; each problem
/// once.
pub fn validate(specification: &Specification) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    if specification.schema != SCHEMA_VERSION {
        errors.push(ValidationError::UnknownSchema {
            found: specification.schema,
        });
    }
    let characters = specification.identity.name.chars().count();
    if !NAME_CHARACTERS.contains(&characters) {
        errors.push(ValidationError::NameLength { characters });
    }
    let population = specification.settlement.population;
    if !POPULATION.contains(&population) {
        errors.push(ValidationError::PopulationOutOfRange { population });
    }
    let extent = specification.site.extent_metres.get();
    if !EXTENT_METRES.contains(&extent) {
        errors.push(ValidationError::ExtentOutOfRange {
            extent: extent.to_string(),
        });
    }
    check_identifiers(specification, &mut errors);
    check_terrain(specification, &mut errors);
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
        .map(|river| (RIVERS, &river.identifier));
    let roads = specification
        .roads
        .iter()
        .map(|road| (ROADS, &road.identifier));
    let mut seen = BTreeSet::new();
    let mut reported = BTreeSet::new();
    for (list, identifier) in rivers.chain(roads) {
        if seen.insert(identifier) {
            let is_region = CompassRegion::ALL
                .into_iter()
                .any(|region| region.name() == identifier);
            if identifier == CORE_WORD || is_region || identifier.contains('.') {
                errors.push(ValidationError::ReservedIdentifier {
                    list,
                    identifier: identifier.clone(),
                });
            }
        } else if reported.insert(identifier) {
            errors.push(ValidationError::DuplicateIdentifier {
                list,
                identifier: identifier.clone(),
            });
        }
    }
}

fn check_terrain(specification: &Specification, errors: &mut Vec<ValidationError>) {
    let terrain = &specification.site.terrain;
    if let Some(coast) = &terrain.coast {
        let field = format!("{TERRAIN}.coast");
        if let Some(coast_degrees) =
            check_bearing(coast.bearing_degrees, &field, "bearing_degrees", errors)
            && coast.shape == CoastShape::Estuary
        {
            let reaches_the_sea = specification.site.water.rivers.iter().any(|river| {
                river.to.bearing().is_some_and(|river_degrees| {
                    degrees_apart(coast_degrees, river_degrees) <= ESTUARY_RIVER_DEGREES
                })
            });
            if !reaches_the_sea {
                errors.push(ValidationError::EstuaryWithoutRiver);
            }
        }
    }
    for (index, cliff) in terrain.cliffs.iter().enumerate() {
        let field = format!("{TERRAIN}.cliffs[{index}]");
        check_bearing(cliff.bearing_degrees, &field, "bearing_degrees", errors);
        check_size(cliff.height_metres.get(), &field, "height_metres", errors);
    }
    for (index, hill) in terrain.hills.iter().enumerate() {
        let field = format!("{TERRAIN}.hills[{index}]");
        check_size(hill.radius_metres.get(), &field, "radius_metres", errors);
        check_size(hill.height_metres.get(), &field, "height_metres", errors);
    }
    for (index, hollow) in terrain.hollows.iter().enumerate() {
        let field = format!("{TERRAIN}.hollows[{index}]");
        check_size(hollow.radius_metres.get(), &field, "radius_metres", errors);
        check_size(hollow.depth_metres.get(), &field, "depth_metres", errors);
    }
    if let Some(slope) = &terrain.slope {
        let field = format!("{TERRAIN}.slope");
        check_bearing(slope.toward_degrees, &field, "toward_degrees", errors);
        check_size(
            slope.metres_per_kilometre.metres_per_kilometre(),
            &field,
            "metres_per_kilometre",
            errors,
        );
    }
    if let Some(plateau) = &terrain.plateau {
        let field = format!("{TERRAIN}.plateau");
        check_bearing(plateau.bearing_degrees, &field, "bearing_degrees", errors);
        check_size(plateau.height_metres.get(), &field, "height_metres", errors);
    }
}

fn check_bearing(
    bearing: Bearing,
    owner: &str,
    key: &str,
    errors: &mut Vec<ValidationError>,
) -> Option<u16> {
    let degrees = bearing.degrees();
    if degrees.is_none() {
        let field = format!("{owner}.{key}");
        errors.push(match bearing {
            Bearing::Degrees(degrees) => ValidationError::BearingOutOfRange { field, degrees },
            Bearing::Word(_) => ValidationError::BearingAtCentre { field },
        });
    }
    degrees
}

fn check_size(size: f64, owner: &str, key: &str, errors: &mut Vec<ValidationError>) {
    if size <= 0.0 {
        errors.push(ValidationError::ZeroSize {
            field: format!("{owner}.{key}"),
        });
    }
}

fn degrees_apart(first: u16, second: u16) -> u16 {
    let difference = first.abs_diff(second);
    difference.min(360 - difference)
}

fn check_rivers(specification: &Specification, errors: &mut Vec<ValidationError>) {
    for river in &specification.site.water.rivers {
        let named = || river.identifier.clone();
        if river.width_metres.get() <= 0.0 {
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
        let distinct_ends = if first == second {
            let error = if *first == RoadEnd::Core {
                ValidationError::BothRoadEndsCore { road: named() }
            } else {
                ValidationError::RoadEndsEqual { road: named() }
            };
            errors.push(error);
            vec![first]
        } else {
            vec![first, second]
        };
        for end in distinct_ends {
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
        Cliff, Coast, Era, Hill, Hollow, Identity, Importance, Plateau, Presets, River,
        RiverCrossingKind, Road, Settlement, Site, Slope, Terrain, Water,
    };
    use crate::units::{Gradient, Length};

    fn length(value: f64) -> Length {
        Length::new(value).expect("test input is a valid length")
    }

    fn gradient(metres_per_kilometre: f64) -> Gradient {
        Gradient::new(metres_per_kilometre).expect("test input is a valid gradient")
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
                terrain: Terrain::default(),
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
                list: ROADS,
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
                    list: ROADS,
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
    fn a_duplicate_within_one_list_is_reported_once() {
        let mut specification = valid();
        let copy = specification.roads[0].clone();
        specification.roads.push(copy.clone());
        specification.roads.push(copy);
        assert_eq!(
            only_error(&specification),
            ValidationError::DuplicateIdentifier {
                list: ROADS,
                identifier: "north_road".to_string()
            }
        );
    }

    #[test]
    fn two_equal_place_ends_are_one_equal_ends_error_and_one_place_error() {
        let mut specification = valid();
        let guildhall = RoadEnd::Place("guildhall".to_string());
        specification.roads[1].ends = [guildhall.clone(), guildhall];
        let errors = validate(&specification).expect_err("two rules are broken");
        assert_eq!(errors.len(), 2, "{errors:?}");
    }

    fn terrain() -> Terrain {
        Terrain {
            coast: Some(Coast {
                bearing_degrees: Bearing::Degrees(100),
                shape: CoastShape::Estuary,
                shore: None,
            }),
            cliffs: vec![Cliff {
                bearing_degrees: Bearing::Word(CompassRegion::West),
                height_metres: length(40.0),
            }],
            hills: vec![Hill {
                at: CompassRegion::NorthWest,
                radius_metres: length(300.0),
                height_metres: length(60.0),
            }],
            hollows: vec![Hollow {
                at: CompassRegion::Centre,
                radius_metres: length(120.0),
                depth_metres: length(8.0),
            }],
            slope: Some(Slope {
                toward_degrees: Bearing::Word(CompassRegion::East),
                metres_per_kilometre: gradient(15.0),
            }),
            plateau: Some(Plateau {
                bearing_degrees: Bearing::Degrees(250),
                height_metres: length(30.0),
            }),
        }
    }

    fn with_terrain() -> Specification {
        let mut specification = valid();
        specification.site.terrain = terrain();
        specification
    }

    type BreakTerrain = fn(&mut Terrain);

    fn zero_size_field(field: &str) -> ValidationError {
        ValidationError::ZeroSize {
            field: format!("{TERRAIN}.{field}"),
        }
    }

    #[test]
    fn a_site_with_every_terrain_feature_passes() {
        assert_eq!(validate(&with_terrain()), Ok(()));
    }

    #[test]
    fn a_terrain_size_of_zero_is_an_error() {
        let cases: [(BreakTerrain, &str); 8] = [
            (
                |terrain| terrain.cliffs[0].height_metres = length(0.0),
                "cliffs[0].height_metres",
            ),
            (
                |terrain| terrain.hills[0].radius_metres = length(0.0),
                "hills[0].radius_metres",
            ),
            (
                |terrain| terrain.hills[0].height_metres = length(0.0),
                "hills[0].height_metres",
            ),
            (
                |terrain| terrain.hollows[0].radius_metres = length(0.0),
                "hollows[0].radius_metres",
            ),
            (
                |terrain| terrain.hollows[0].depth_metres = length(0.0),
                "hollows[0].depth_metres",
            ),
            (
                |terrain| {
                    if let Some(slope) = &mut terrain.slope {
                        slope.metres_per_kilometre = gradient(0.0);
                    }
                },
                "slope.metres_per_kilometre",
            ),
            (
                |terrain| {
                    if let Some(plateau) = &mut terrain.plateau {
                        plateau.height_metres = length(0.0);
                    }
                },
                "plateau.height_metres",
            ),
            (
                |terrain| {
                    terrain.hills.push(Hill {
                        at: CompassRegion::South,
                        radius_metres: length(1.0),
                        height_metres: length(0.0),
                    });
                },
                "hills[1].height_metres",
            ),
        ];
        for (break_it, field) in cases {
            let mut specification = with_terrain();
            break_it(&mut specification.site.terrain);
            assert_eq!(only_error(&specification), zero_size_field(field));
        }
    }

    #[test]
    fn a_bearing_above_359_degrees_is_an_error() {
        let mut specification = with_terrain();
        specification.site.terrain.cliffs[0].bearing_degrees = Bearing::Degrees(360);
        assert_eq!(
            only_error(&specification),
            ValidationError::BearingOutOfRange {
                field: format!("{TERRAIN}.cliffs[0].bearing_degrees"),
                degrees: 360,
            }
        );
    }

    #[test]
    fn a_bearing_of_centre_is_an_error() {
        let mut specification = with_terrain();
        if let Some(slope) = &mut specification.site.terrain.slope {
            slope.toward_degrees = Bearing::Word(CompassRegion::Centre);
        }
        assert_eq!(
            only_error(&specification),
            ValidationError::BearingAtCentre {
                field: format!("{TERRAIN}.slope.toward_degrees"),
            }
        );
    }

    #[test]
    fn an_estuary_needs_a_river_that_reaches_its_coast() {
        let mut specification = with_terrain();
        if let Some(coast) = &mut specification.site.terrain.coast {
            coast.bearing_degrees = Bearing::Degrees(136);
        }
        assert_eq!(
            only_error(&specification),
            ValidationError::EstuaryWithoutRiver
        );
    }

    #[test]
    fn a_coast_that_is_no_estuary_needs_no_river() {
        let mut specification = with_terrain();
        if let Some(coast) = &mut specification.site.terrain.coast {
            coast.bearing_degrees = Bearing::Degrees(136);
            coast.shape = CoastShape::Bay;
        }
        assert_eq!(validate(&specification), Ok(()));
    }

    #[test]
    fn a_river_within_45_degrees_of_the_estuary_reaches_it() {
        let mut specification = with_terrain();
        if let Some(coast) = &mut specification.site.terrain.coast {
            coast.bearing_degrees = Bearing::Degrees(135);
        }
        assert_eq!(validate(&specification), Ok(()));
    }

    #[test]
    fn an_estuary_across_north_measures_the_short_way_round() {
        let mut specification = with_terrain();
        specification.site.water.rivers[0].to = CompassRegion::North;
        if let Some(coast) = &mut specification.site.terrain.coast {
            coast.bearing_degrees = Bearing::Degrees(350);
        }
        assert_eq!(validate(&specification), Ok(()));
    }

    #[test]
    fn an_estuary_with_a_broken_bearing_reports_only_the_bearing() {
        let mut specification = with_terrain();
        if let Some(coast) = &mut specification.site.terrain.coast {
            coast.bearing_degrees = Bearing::Word(CompassRegion::Centre);
        }
        assert_eq!(
            only_error(&specification),
            ValidationError::BearingAtCentre {
                field: format!("{TERRAIN}.coast.bearing_degrees"),
            }
        );
    }

    #[test]
    fn every_message_starts_with_its_field() {
        let road = || "r".to_string();
        let cases = [
            (ValidationError::UnknownSchema { found: 2 }, "schema"),
            (
                ValidationError::NameLength { characters: 0 },
                "identity.name",
            ),
            (
                ValidationError::PopulationOutOfRange { population: 0 },
                "settlement.population",
            ),
            (
                ValidationError::ExtentOutOfRange {
                    extent: "1".to_string(),
                },
                "site.extent_metres",
            ),
            (
                ValidationError::DuplicateIdentifier {
                    list: RIVERS,
                    identifier: road(),
                },
                RIVERS,
            ),
            (
                ValidationError::ReservedIdentifier {
                    list: ROADS,
                    identifier: road(),
                },
                ROADS,
            ),
            (ValidationError::ZeroRiverWidth { river: road() }, RIVERS),
            (ValidationError::RiverEndAtCentre { river: road() }, RIVERS),
            (ValidationError::RiverEndsEqual { river: road() }, RIVERS),
            (
                ValidationError::ZeroSize {
                    field: format!("{TERRAIN}.hills[0].radius_metres"),
                },
                TERRAIN,
            ),
            (
                ValidationError::BearingOutOfRange {
                    field: format!("{TERRAIN}.plateau.bearing_degrees"),
                    degrees: 360,
                },
                TERRAIN,
            ),
            (
                ValidationError::BearingAtCentre {
                    field: format!("{TERRAIN}.coast.bearing_degrees"),
                },
                TERRAIN,
            ),
            (ValidationError::EstuaryWithoutRiver, TERRAIN),
            (ValidationError::NoRoads, ROADS),
            (ValidationError::BothRoadEndsCore { road: road() }, ROADS),
            (ValidationError::RoadEndsEqual { road: road() }, ROADS),
            (ValidationError::RoadEndAtCentre { road: road() }, ROADS),
            (
                ValidationError::RoadEndAtPlace {
                    road: road(),
                    place: road(),
                },
                ROADS,
            ),
        ];
        for (error, field) in cases {
            assert!(error.to_string().starts_with(field), "{error}");
        }
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
            POPULATION,
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
