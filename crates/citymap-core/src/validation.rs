//! Validation: the rules of a specification that its types cannot hold, such as ranges and
//! rules between fields (specification-version-1 §5). It returns every error it finds, not
//! only the first, and each error names the field it is about (ADR-046).

use std::collections::BTreeSet;
use std::ops::RangeInclusive;

use crate::specification::{
    Bearing, CORE_WORD, CoastShape, CompassRegion, Constraint, Harbour, RoadEnd, Rule,
    Specification,
};
use crate::units::Length;

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

/// The field path of the lakes list.
pub const LAKES: &str = "site.water.lakes";

/// The field path of the harbour.
pub const HARBOUR: &str = "site.water.harbour";

/// The field path of the canals list.
pub const CANALS: &str = "site.water.canals";

/// How far, in degrees, a coast or a river end may lie from a harbour's bearing.
pub const HARBOUR_WATER_DEGREES: u16 = 45;

/// The allowed whole width of a compass region's slice, in degrees.
pub const SLICE_WIDTH_DEGREES: RangeInclusive<u16> = 1..=360;

/// The field path of the land cover.
pub const LAND_COVER: &str = "site.land_cover";

/// The field path of the terrain.
pub const TERRAIN: &str = "site.terrain";

/// How far, in degrees, a river's end may lie from an estuary's bearing.
pub const ESTUARY_RIVER_DEGREES: u16 = 45;

/// The field path of the districts list.
pub const DISTRICTS: &str = "districts";

/// The most districts a specification may have. A specification file is untrusted input, and
/// a file with a runaway list must fail fast instead of stalling the generator
/// (REQUIREMENTS.md F1); no real city comes near it.
pub const MAXIMUM_DISTRICTS: usize = 100;

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
    /// A river or canal is zero metres wide.
    #[error("{list} `{waterway}`: width_metres is 0; a waterway is wider than that")]
    ZeroWaterwayWidth {
        /// The field path of the list it is in.
        list: &'static str,
        /// The river's or canal's identifier.
        waterway: String,
    },
    /// A river or canal starts or ends at `centre`, which is not an edge (until #130).
    #[error("{list} `{waterway}`: `centre` is not an edge stretch")]
    WaterwayEndAtCentre {
        /// The field path of the list it is in.
        list: &'static str,
        /// The river's or canal's identifier.
        waterway: String,
    },
    /// A river or canal enters and leaves at the same edge stretch (until #131).
    #[error("{list} `{waterway}`: `from` and `to` are the same stretch")]
    WaterwayEndsEqual {
        /// The field path of the list it is in.
        list: &'static str,
        /// The river's or canal's identifier.
        waterway: String,
    },
    /// A compass region's `width_degrees` is outside 1 to 360.
    #[error("{field}: {degrees} degrees; a slice is 1 to 360 degrees wide")]
    WidthOutOfRange {
        /// The field path of the width.
        field: String,
        /// The width in the file.
        degrees: u16,
    },
    /// A compass region of `centre` has a `width_degrees`, which only a slice has.
    #[error("{field}: `centre` is a circle around the core and has no width")]
    WidthAtCentre {
        /// The field path of the width.
        field: String,
    },
    /// A land cover list names a cover that it already named earlier.
    #[error("{field}: this cover is already listed earlier in the same list")]
    DuplicateCover {
        /// The field path of the repeated entry, such as `site.land_cover.outside[2]`.
        field: String,
    },
    /// A harbour has no coast and no river end near its bearing.
    #[error(
        "site.water.harbour `{harbour}`: no coast and no river end lies within {HARBOUR_WATER_DEGREES} degrees of its bearing"
    )]
    HarbourWithoutWater {
        /// The harbour's identifier.
        harbour: String,
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
    /// The file has no district.
    #[error("districts: a city needs at least one district")]
    NoDistricts,
    /// The file has more districts than `MAXIMUM_DISTRICTS`.
    #[error("districts: {count} districts; at most {MAXIMUM_DISTRICTS}")]
    TooManyDistricts {
        /// How many districts the file has.
        count: usize,
    },
    /// A district has a wall rule; what a wall encloses comes from its `encloses` list.
    #[error("{field}: a district cannot have a wall rule; list it in the wall's `encloses`")]
    DistrictWallRule {
        /// The field path of the rule.
        field: String,
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
/// Every broken rule: header fields, identifiers, terrain, water, land cover, districts, then
/// roads; each problem once.
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
    check_water(specification, &mut errors);
    check_land_cover(specification, &mut errors);
    check_districts(specification, &mut errors);
    check_roads(specification, &mut errors);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn check_identifiers(specification: &Specification, errors: &mut Vec<ValidationError>) {
    let water = &specification.site.water;
    let rivers = water.rivers.iter().map(|river| (RIVERS, &river.identifier));
    let lakes = water.lakes.iter().map(|lake| (LAKES, &lake.identifier));
    let harbour = water
        .harbour
        .iter()
        .map(|harbour| (HARBOUR, &harbour.identifier));
    let canals = water.canals.iter().map(|canal| (CANALS, &canal.identifier));
    let districts = specification
        .districts
        .iter()
        .map(|district| (DISTRICTS, &district.identifier));
    let roads = specification
        .roads
        .iter()
        .map(|road| (ROADS, &road.identifier));
    let mut seen = BTreeSet::new();
    let mut reported = BTreeSet::new();
    for (list, identifier) in rivers
        .chain(lakes)
        .chain(harbour)
        .chain(canals)
        .chain(districts)
        .chain(roads)
    {
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

/// Checks what a list of constraints can break on its own (ADR-059): weights, compass regions
/// and slice widths. Targets are checked with the rest of the file. `owner` is the field path
/// of the constraints' object, such as `districts[2]`.
pub fn check_constraints(
    constraints: &[Constraint],
    owner: &str,
    errors: &mut Vec<ValidationError>,
) {
    for (index, constraint) in constraints.iter().enumerate() {
        let field = format!("{owner}.constraints[{index}]");
        check_size(constraint.weight.get(), &field, "weight", errors);
        if let Rule::CompassRegion {
            region,
            width_degrees,
        }
        | Rule::NotInCompassRegion {
            region,
            width_degrees,
        } = &constraint.rule
        {
            let is_centre = *region == Bearing::Word(CompassRegion::Centre);
            if !is_centre {
                check_bearing(*region, &field, "region", errors);
            }
            match width_degrees {
                Some(_) if is_centre => errors.push(ValidationError::WidthAtCentre {
                    field: format!("{field}.width_degrees"),
                }),
                Some(degrees) if !SLICE_WIDTH_DEGREES.contains(degrees) => {
                    errors.push(ValidationError::WidthOutOfRange {
                        field: format!("{field}.width_degrees"),
                        degrees: *degrees,
                    });
                }
                Some(_) | None => {}
            }
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

fn check_water(specification: &Specification, errors: &mut Vec<ValidationError>) {
    let water = &specification.site.water;
    for river in &water.rivers {
        let ends = (river.from, river.to);
        check_waterway(RIVERS, &river.identifier, ends, river.width_metres, errors);
    }
    for canal in &water.canals {
        let ends = (canal.from, canal.to);
        check_waterway(CANALS, &canal.identifier, ends, canal.width_metres, errors);
    }
    for (index, lake) in water.lakes.iter().enumerate() {
        let field = format!("{LAKES}[{index}]");
        check_size(lake.radius_metres.get(), &field, "radius_metres", errors);
    }
    if let Some(harbour) = &water.harbour {
        check_harbour(specification, harbour, errors);
    }
}

fn check_waterway(
    list: &'static str,
    identifier: &str,
    (from, to): (CompassRegion, CompassRegion),
    width: Length,
    errors: &mut Vec<ValidationError>,
) {
    let waterway = || identifier.to_string();
    if width.get() <= 0.0 {
        errors.push(ValidationError::ZeroWaterwayWidth {
            list,
            waterway: waterway(),
        });
    }
    if from == CompassRegion::Centre || to == CompassRegion::Centre {
        errors.push(ValidationError::WaterwayEndAtCentre {
            list,
            waterway: waterway(),
        });
    }
    if from == to {
        errors.push(ValidationError::WaterwayEndsEqual {
            list,
            waterway: waterway(),
        });
    }
}

fn check_harbour(
    specification: &Specification,
    harbour: &Harbour,
    errors: &mut Vec<ValidationError>,
) {
    check_size(
        harbour.length_metres.get(),
        HARBOUR,
        "length_metres",
        errors,
    );
    let harbour_degrees =
        check_bearing(harbour.bearing_degrees, HARBOUR, "bearing_degrees", errors);
    let coast = specification.site.terrain.coast.as_ref();
    let coast_bearing_broken = coast.is_some_and(|coast| coast.bearing_degrees.degrees().is_none());
    let Some(harbour_degrees) = harbour_degrees else {
        return;
    };
    if coast_bearing_broken {
        return;
    }
    let near = |degrees: Option<u16>| {
        degrees
            .is_some_and(|degrees| degrees_apart(harbour_degrees, degrees) <= HARBOUR_WATER_DEGREES)
    };
    let coast_near = coast.is_some_and(|coast| near(coast.bearing_degrees.degrees()));
    let river_near = specification
        .site
        .water
        .rivers
        .iter()
        .any(|river| near(river.from.bearing()) || near(river.to.bearing()));
    if !coast_near && !river_near {
        errors.push(ValidationError::HarbourWithoutWater {
            harbour: harbour.identifier.clone(),
        });
    }
}

fn check_land_cover(specification: &Specification, errors: &mut Vec<ValidationError>) {
    let land_cover = &specification.site.land_cover;
    check_unique_covers(
        &land_cover.outside,
        &format!("{LAND_COVER}.outside"),
        errors,
    );
    check_unique_covers(&land_cover.inside, &format!("{LAND_COVER}.inside"), errors);
    for (index, band) in land_cover.bands.iter().enumerate() {
        let field = format!("{LAND_COVER}.bands[{index}]");
        check_bearing(band.bearing_degrees, &field, "bearing_degrees", errors);
        check_size(band.depth_metres.get(), &field, "depth_metres", errors);
    }
}

fn check_unique_covers<Cover: Ord>(
    covers: &[Cover],
    list: &str,
    errors: &mut Vec<ValidationError>,
) {
    let mut seen = BTreeSet::new();
    for (index, cover) in covers.iter().enumerate() {
        if !seen.insert(cover) {
            errors.push(ValidationError::DuplicateCover {
                field: format!("{list}[{index}]"),
            });
        }
    }
}

fn check_districts(specification: &Specification, errors: &mut Vec<ValidationError>) {
    let count = specification.districts.len();
    if count == 0 {
        errors.push(ValidationError::NoDistricts);
    }
    if count > MAXIMUM_DISTRICTS {
        errors.push(ValidationError::TooManyDistricts { count });
    }
    for (index, district) in specification.districts.iter().enumerate() {
        let owner = format!("{DISTRICTS}[{index}]");
        check_size(district.weight.get(), &owner, "weight", errors);
        check_constraints(&district.constraints, &owner, errors);
        for (rule_index, constraint) in district.constraints.iter().enumerate() {
            if matches!(
                constraint.rule,
                Rule::InsideWalls { .. } | Rule::OutsideWalls { .. }
            ) {
                errors.push(ValidationError::DistrictWallRule {
                    field: format!("{owner}.constraints[{rule_index}]"),
                });
            }
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
        Band, Canal, Cliff, Coast, Constraint, District, DistrictFunction, Enclosure, Era, Harbour,
        Hill, Hollow, Identity, Importance, InsideCover, Jurisdiction, Lake, LandCover,
        OutsideCover, Plateau, Presets, River, RiverCrossingKind, Road, Rule, Settlement, Site,
        Slope, Terrain, Water, Wealth,
    };
    use crate::units::{Gradient, Length, Weight};

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
                land_cover: LandCover::default(),
                water: Water {
                    rivers: vec![river("ash", CompassRegion::NorthWest, CompassRegion::East)],
                    ..Water::default()
                },
            },
            districts: vec![District {
                identifier: "old_town".to_string(),
                name: "Old Town".to_string(),
                function: DistrictFunction::Residential,
                wealth: Wealth::Middling,
                group: None,
                enclosure: Enclosure::Open,
                jurisdiction: Jurisdiction::Town,
                weight: Weight::DEFAULT,
                constraints: Vec::new(),
                notes: String::new(),
            }],
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
            ValidationError::ZeroWaterwayWidth {
                list: RIVERS,
                waterway: "ash".to_string()
            }
        );
    }

    #[test]
    fn a_river_does_not_end_at_the_centre() {
        let mut specification = valid();
        specification.site.water.rivers[0].to = CompassRegion::Centre;
        assert_eq!(
            only_error(&specification),
            ValidationError::WaterwayEndAtCentre {
                list: RIVERS,
                waterway: "ash".to_string()
            }
        );
    }

    #[test]
    fn a_river_leaves_at_another_stretch() {
        let mut specification = valid();
        specification.site.water.rivers[0].to = CompassRegion::NorthWest;
        assert_eq!(
            only_error(&specification),
            ValidationError::WaterwayEndsEqual {
                list: RIVERS,
                waterway: "ash".to_string()
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

    type BreakCanal = fn(&mut Canal);

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

    fn with_water() -> Specification {
        let mut specification = valid();
        let water = &mut specification.site.water;
        water.lakes.push(Lake {
            identifier: "mere".to_string(),
            at: CompassRegion::SouthWest,
            radius_metres: length(150.0),
        });
        water.harbour = Some(Harbour {
            identifier: "quay".to_string(),
            bearing_degrees: Bearing::Degrees(60),
            length_metres: length(200.0),
        });
        water.tidal = true;
        water.canals.push(Canal {
            identifier: "cut".to_string(),
            from: CompassRegion::South,
            to: CompassRegion::West,
            width_metres: length(6.0),
        });
        specification
    }

    fn harbour_at(degrees: u16) -> Specification {
        let mut specification = with_water();
        if let Some(harbour) = &mut specification.site.water.harbour {
            harbour.bearing_degrees = Bearing::Degrees(degrees);
        }
        specification
    }

    #[test]
    fn a_site_with_every_kind_of_water_passes() {
        assert_eq!(validate(&with_water()), Ok(()));
    }

    #[test]
    fn a_lake_radius_of_zero_is_an_error() {
        let mut specification = with_water();
        specification.site.water.lakes[0].radius_metres = length(0.0);
        assert_eq!(
            only_error(&specification),
            ValidationError::ZeroSize {
                field: format!("{LAKES}[0].radius_metres"),
            }
        );
    }

    #[test]
    fn a_harbour_length_of_zero_is_an_error() {
        let mut specification = with_water();
        if let Some(harbour) = &mut specification.site.water.harbour {
            harbour.length_metres = length(0.0);
        }
        assert_eq!(
            only_error(&specification),
            ValidationError::ZeroSize {
                field: format!("{HARBOUR}.length_metres"),
            }
        );
    }

    #[test]
    fn a_harbour_needs_a_coast_or_a_river_end_near_its_bearing() {
        assert_eq!(
            only_error(&harbour_at(200)),
            ValidationError::HarbourWithoutWater {
                harbour: "quay".to_string(),
            }
        );
    }

    #[test]
    fn a_river_end_within_45_degrees_serves_the_harbour() {
        assert_eq!(validate(&harbour_at(270)), Ok(()));
        assert_eq!(validate(&harbour_at(135)), Ok(()));
        assert!(validate(&harbour_at(136)).is_err());
    }

    #[test]
    fn a_coast_within_45_degrees_serves_the_harbour() {
        let mut specification = harbour_at(200);
        specification.site.terrain.coast = Some(Coast {
            bearing_degrees: Bearing::Word(CompassRegion::South),
            shape: CoastShape::Open,
            shore: None,
        });
        assert_eq!(validate(&specification), Ok(()));
    }

    #[test]
    fn a_harbour_bearing_above_359_is_reported_once() {
        assert_eq!(
            only_error(&harbour_at(360)),
            ValidationError::BearingOutOfRange {
                field: format!("{HARBOUR}.bearing_degrees"),
                degrees: 360,
            }
        );
    }

    #[test]
    fn a_broken_coast_bearing_does_not_also_fail_the_harbour() {
        let mut specification = harbour_at(200);
        specification.site.terrain.coast = Some(Coast {
            bearing_degrees: Bearing::Degrees(400),
            shape: CoastShape::Open,
            shore: None,
        });
        assert_eq!(
            only_error(&specification),
            ValidationError::BearingOutOfRange {
                field: format!("{TERRAIN}.coast.bearing_degrees"),
                degrees: 400,
            }
        );
    }

    #[test]
    fn a_canal_gets_the_checks_of_a_river() {
        let cases: [(BreakCanal, ValidationError); 3] = [
            (
                |canal| canal.width_metres = length(0.0),
                ValidationError::ZeroWaterwayWidth {
                    list: CANALS,
                    waterway: "cut".to_string(),
                },
            ),
            (
                |canal| canal.to = CompassRegion::Centre,
                ValidationError::WaterwayEndAtCentre {
                    list: CANALS,
                    waterway: "cut".to_string(),
                },
            ),
            (
                |canal| canal.to = CompassRegion::South,
                ValidationError::WaterwayEndsEqual {
                    list: CANALS,
                    waterway: "cut".to_string(),
                },
            ),
        ];
        for (break_it, error) in cases {
            let mut specification = with_water();
            break_it(&mut specification.site.water.canals[0]);
            assert_eq!(only_error(&specification), error);
        }
    }

    #[test]
    fn a_lake_and_a_canal_share_no_identifier() {
        let mut specification = with_water();
        specification.site.water.canals[0].identifier = "mere".to_string();
        assert_eq!(
            only_error(&specification),
            ValidationError::DuplicateIdentifier {
                list: CANALS,
                identifier: "mere".to_string(),
            }
        );
    }

    #[test]
    fn a_harbour_identifier_is_checked_for_reserved_words() {
        let mut specification = with_water();
        if let Some(harbour) = &mut specification.site.water.harbour {
            harbour.identifier = "core".to_string();
        }
        assert_eq!(
            only_error(&specification),
            ValidationError::ReservedIdentifier {
                list: HARBOUR,
                identifier: "core".to_string(),
            }
        );
    }

    fn with_land_cover() -> Specification {
        let mut specification = valid();
        let land_cover = &mut specification.site.land_cover;
        land_cover.outside = vec![OutsideCover::Farmland, OutsideCover::Forest];
        land_cover.inside = vec![InsideCover::Gardens];
        land_cover.bands.push(Band {
            cover: OutsideCover::Forest,
            bearing_degrees: Bearing::Word(CompassRegion::North),
            depth_metres: length(300.0),
        });
        specification
    }

    #[test]
    fn land_cover_with_names_and_a_band_passes() {
        assert_eq!(validate(&with_land_cover()), Ok(()));
    }

    #[test]
    fn a_cover_listed_twice_is_an_error_at_the_repeat() {
        let mut specification = with_land_cover();
        specification
            .site
            .land_cover
            .outside
            .push(OutsideCover::Farmland);
        specification
            .site
            .land_cover
            .inside
            .push(InsideCover::Gardens);
        let errors = validate(&specification).expect_err("two lists repeat a cover");
        assert_eq!(
            errors,
            [
                ValidationError::DuplicateCover {
                    field: format!("{LAND_COVER}.outside[2]"),
                },
                ValidationError::DuplicateCover {
                    field: format!("{LAND_COVER}.inside[1]"),
                },
            ]
        );
    }

    #[test]
    fn a_band_needs_depth_and_a_direction() {
        let mut specification = with_land_cover();
        specification.site.land_cover.bands[0].depth_metres = length(0.0);
        specification.site.land_cover.bands[0].bearing_degrees =
            Bearing::Word(CompassRegion::Centre);
        let errors = validate(&specification).expect_err("the band breaks two rules");
        assert_eq!(
            errors,
            [
                ValidationError::BearingAtCentre {
                    field: format!("{LAND_COVER}.bands[0].bearing_degrees"),
                },
                ValidationError::ZeroSize {
                    field: format!("{LAND_COVER}.bands[0].depth_metres"),
                },
            ]
        );
    }

    fn weight(value: f64) -> Weight {
        Weight::new(value).expect("test input is a valid weight")
    }

    fn rule(rule: Rule, weight: Weight) -> Constraint {
        Constraint {
            rule,
            hard: false,
            weight,
        }
    }

    fn compass_region(region: Bearing, width_degrees: Option<u16>) -> Constraint {
        let region = Rule::CompassRegion {
            region,
            width_degrees,
        };
        rule(region, Weight::DEFAULT)
    }

    fn constraint_errors(constraints: &[Constraint]) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        check_constraints(constraints, "districts[0]", &mut errors);
        errors
    }

    #[test]
    fn rules_within_their_ranges_pass() {
        let constraints = [
            compass_region(Bearing::Word(CompassRegion::NorthEast), None),
            compass_region(Bearing::Word(CompassRegion::Centre), None),
            rule(
                Rule::NotInCompassRegion {
                    region: Bearing::Degrees(115),
                    width_degrees: Some(20),
                },
                weight(2.0),
            ),
            rule(
                Rule::Near {
                    target: "market".to_string(),
                    within_metres: length(0.0),
                },
                Weight::DEFAULT,
            ),
        ];
        assert_eq!(constraint_errors(&constraints), []);
    }

    #[test]
    fn a_weight_of_zero_is_an_error() {
        let constraints = [rule(Rule::OnWater, weight(0.0))];
        assert_eq!(
            constraint_errors(&constraints),
            [ValidationError::ZeroSize {
                field: "districts[0].constraints[0].weight".to_string(),
            }]
        );
    }

    #[test]
    fn a_compass_region_bearing_above_359_is_an_error() {
        assert_eq!(
            constraint_errors(&[compass_region(Bearing::Degrees(400), None)]),
            [ValidationError::BearingOutOfRange {
                field: "districts[0].constraints[0].region".to_string(),
                degrees: 400,
            }]
        );
    }

    #[test]
    fn a_slice_is_1_to_360_degrees_wide() {
        for degrees in [0, 361] {
            let constraints = [compass_region(Bearing::Degrees(90), Some(degrees))];
            assert_eq!(
                constraint_errors(&constraints),
                [ValidationError::WidthOutOfRange {
                    field: "districts[0].constraints[0].width_degrees".to_string(),
                    degrees,
                }]
            );
        }
        let whole_circle = [compass_region(Bearing::Degrees(90), Some(360))];
        assert_eq!(constraint_errors(&whole_circle), []);
    }

    #[test]
    fn the_centre_has_no_width() {
        let constraints = [compass_region(
            Bearing::Word(CompassRegion::Centre),
            Some(45),
        )];
        assert_eq!(
            constraint_errors(&constraints),
            [ValidationError::WidthAtCentre {
                field: "districts[0].constraints[0].width_degrees".to_string(),
            }]
        );
    }

    fn wall_rule(target: &str) -> Constraint {
        Constraint {
            rule: Rule::InsideWalls {
                target: target.to_string(),
            },
            hard: true,
            weight: Weight::DEFAULT,
        }
    }

    #[test]
    fn a_city_needs_a_district() {
        let mut specification = valid();
        specification.districts.clear();
        assert_eq!(only_error(&specification), ValidationError::NoDistricts);
    }

    #[test]
    fn a_city_has_at_most_100_districts() {
        let mut specification = valid();
        let template = specification.districts[0].clone();
        specification.districts = (0..=MAXIMUM_DISTRICTS)
            .map(|index| District {
                identifier: format!("district_{index}"),
                ..template.clone()
            })
            .collect();
        assert_eq!(
            only_error(&specification),
            ValidationError::TooManyDistricts { count: 101 }
        );
    }

    #[test]
    fn a_district_weight_of_zero_is_an_error() {
        let mut specification = valid();
        specification.districts[0].weight = weight(0.0);
        assert_eq!(
            only_error(&specification),
            ValidationError::ZeroSize {
                field: "districts[0].weight".to_string(),
            }
        );
    }

    #[test]
    fn a_district_cannot_have_a_wall_rule() {
        let mut specification = valid();
        specification.districts[0].constraints = vec![
            wall_rule("outer"),
            Constraint {
                rule: Rule::MinSpacing {
                    metres: length(20.0),
                },
                hard: false,
                weight: Weight::DEFAULT,
            },
        ];
        assert_eq!(
            only_error(&specification),
            ValidationError::DistrictWallRule {
                field: "districts[0].constraints[0]".to_string(),
            }
        );
    }

    #[test]
    fn a_district_rule_is_checked_like_every_rule() {
        let mut specification = valid();
        specification.districts[0].constraints = vec![Constraint {
            rule: Rule::OnWater,
            hard: false,
            weight: weight(0.0),
        }];
        assert_eq!(
            only_error(&specification),
            ValidationError::ZeroSize {
                field: "districts[0].constraints[0].weight".to_string(),
            }
        );
    }

    #[test]
    fn a_district_and_a_river_share_no_identifier() {
        let mut specification = valid();
        specification.districts[0].identifier = "ash".to_string();
        assert_eq!(
            only_error(&specification),
            ValidationError::DuplicateIdentifier {
                list: DISTRICTS,
                identifier: "ash".to_string(),
            }
        );
    }

    #[test]
    fn every_constraint_message_starts_with_its_field() {
        let width = || "districts[0].constraints[0].width_degrees".to_string();
        for error in [
            ValidationError::WidthOutOfRange {
                field: width(),
                degrees: 0,
            },
            ValidationError::WidthAtCentre { field: width() },
            ValidationError::DistrictWallRule { field: width() },
            ValidationError::ZeroSize { field: width() },
        ] {
            assert!(error.to_string().starts_with(&width()), "{error}");
        }
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
            (
                ValidationError::ZeroWaterwayWidth {
                    list: RIVERS,
                    waterway: road(),
                },
                RIVERS,
            ),
            (
                ValidationError::WaterwayEndAtCentre {
                    list: CANALS,
                    waterway: road(),
                },
                CANALS,
            ),
            (
                ValidationError::WaterwayEndsEqual {
                    list: CANALS,
                    waterway: road(),
                },
                CANALS,
            ),
            (
                ValidationError::HarbourWithoutWater { harbour: road() },
                HARBOUR,
            ),
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
            (
                ValidationError::DuplicateCover {
                    field: format!("{LAND_COVER}.inside[1]"),
                },
                LAND_COVER,
            ),
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
