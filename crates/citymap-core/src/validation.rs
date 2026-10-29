//! Validation: the rules of a specification that its types cannot hold, such as ranges and
//! rules between fields (specification-version-1 §5). It returns every error it finds, not
//! only the first, and each error names the field it is about (ADR-046).

use std::collections::{BTreeMap, BTreeSet};
use std::ops::{Range, RangeInclusive};

use crate::specification::{
    Bearing, CORE_WORD, CoastShape, CompassRegion, Constraint, GateKind, Harbour, Placement,
    RoadEnd, Rule, Specification,
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

/// The field path of the landmarks list.
pub const LANDMARKS: &str = "landmarks";

/// The most landmarks a specification may have, for the reason `MAXIMUM_DISTRICTS` gives.
pub const MAXIMUM_LANDMARKS: usize = 1000;

/// The labels the default rule pack knows (specification-version-1 §3). They move into that
/// pack when rule packs exist; until then an unknown label is warned about against this list.
pub const KNOWN_LABELS: [&str; 5] = ["cathedral", "guildhall", "lighthouse", "shrine", "pagoda"];

/// The drawing styles that exist; any other `presets.style` is a warning (ADR-065).
pub const KNOWN_STYLES: [&str; 1] = ["ink"];

/// The rule packs that exist besides the default pack; any other `presets.packs` entry is a
/// warning (ADR-065).
pub const KNOWN_PACKS: [&str; 0] = [];

/// The field path of the presets.
pub const PRESETS: &str = "presets";

/// The turn a placement may have, in degrees: from 0 up to, but not including, 360.
pub const ROTATION_DEGREES: Range<f64> = 0.0..360.0;

/// The field path of the walls list.
pub const WALLS: &str = "defences.walls";

/// The field path of the citadel.
pub const CITADEL: &str = "defences.citadel";

/// The field path of the gates list.
pub const GATES: &str = "defences.gates";

/// The field path of the open space list.
pub const OPEN_SPACE: &str = "open_space";

/// The field path of the crossings list.
pub const CROSSINGS: &str = "crossings";

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
    /// The file has more landmarks than `MAXIMUM_LANDMARKS`.
    #[error("landmarks: {count} landmarks; at most {MAXIMUM_LANDMARKS}")]
    TooManyLandmarks {
        /// How many landmarks the file has.
        count: usize,
    },
    /// An object is pinned but has no `resolved` placement to keep.
    #[error("{field}: `pinned` is true but there is no `resolved` placement to keep")]
    PinnedWithoutPlacement {
        /// The field path of the object.
        field: String,
    },
    /// A placement's turn is outside `ROTATION_DEGREES`.
    #[error("{field}: {degrees} degrees; a turn is from 0 up to 360")]
    RotationOutOfRange {
        /// The field path of the turn.
        field: String,
        /// The turn in the file, in degrees.
        degrees: String,
    },
    /// A road ends at a landmark that is not pinned, so its position is unknown in S0.
    #[error("roads `{road}`: end `{place}` is not pinned; pin it first")]
    RoadEndNotPinned {
        /// The road's identifier.
        road: String,
        /// The end as written in the file.
        place: String,
    },
    /// A wall's `encloses` names something that is no district and no wall.
    #[error("defences.walls `{wall}`: `{entry}` in `encloses` is no district and no wall")]
    EnclosesNeitherDistrictNorWall {
        /// The wall's identifier.
        wall: String,
        /// The entry as written.
        entry: String,
    },
    /// Walls enclose each other in a ring, so none of them can be inside the other.
    #[error("defences.walls: {} enclose each other in a ring", walls.join(" -> "))]
    WallRing {
        /// The walls of the ring, in the order they enclose each other, the first repeated
        /// at the end.
        walls: Vec<String>,
    },
    /// A district or wall is listed under two walls that do not enclose each other, so it
    /// would lie in two separate areas.
    #[error(
        "defences.walls: `{entry}` is listed under `{first}` and `{second}`, and neither encloses the other"
    )]
    UnderTwoWalls {
        /// The district's or wall's identifier.
        entry: String,
        /// The first wall that lists it.
        first: String,
        /// The second wall that lists it.
        second: String,
    },
    /// A wall gives `gap_metres` without `separate_ring: true`, so the gap would be ignored.
    #[error("defences.walls `{wall}`: `gap_metres` needs `separate_ring: true`")]
    GapWithoutSeparateRing {
        /// The wall's identifier.
        wall: String,
    },
    /// The citadel's `wall` names no wall of the file.
    #[error("defences.citadel `{citadel}`: `{wall}` in `wall` is no wall of this file")]
    CitadelWithoutWall {
        /// The citadel's identifier.
        citadel: String,
        /// The `wall` as written.
        wall: String,
    },
    /// A gate's `wall` names no wall of the file.
    #[error("defences.gates `{gate}`: `{wall}` in `wall` is no wall of this file")]
    GateWithoutWall {
        /// The gate's identifier.
        gate: String,
        /// The `wall` as written.
        wall: String,
    },
    /// A road gate's `road` names no road of the file.
    #[error("defences.gates `{gate}`: `{road}` in `road` is no road of this file")]
    GateWithoutRoad {
        /// The gate's identifier.
        gate: String,
        /// The `road` as written.
        road: String,
    },
    /// A road gate's `end` is not one of its road's two ends.
    #[error("defences.gates `{gate}`: `{end}` in `end` is not one of the ends of `{road}`")]
    GateEndNotOnRoad {
        /// The gate's identifier.
        gate: String,
        /// The `end` as written.
        end: String,
        /// The road's identifier.
        road: String,
    },
    /// A crossing's `river` names no river of the file.
    #[error("crossings `{crossing}`: `{river}` is no river of this file")]
    CrossingWithoutRiver {
        /// The crossing's identifier.
        crossing: String,
        /// The `river` as written.
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
    /// A road end names no compass region, `core`, landmark or open space.
    #[error("roads `{road}`: end `{place}` names no landmark or open space")]
    RoadEndAtPlace {
        /// The road's identifier.
        road: String,
        /// The end as written in the file.
        place: String,
    },
    /// A rule's `target` names nothing in the file and is not `core`.
    #[error("{field}: `{target}` names nothing in this file")]
    TargetMissing {
        /// The field path of the target.
        field: String,
        /// The target as written.
        target: String,
    },
    /// A rule's `target` names something of another kind than the rule points to, such as
    /// `inside_walls: market`.
    #[error("{field}: `{target}` is no {expected} of this file")]
    TargetOfWrongKind {
        /// The field path of the target.
        field: String,
        /// The target as written.
        target: String,
        /// The kind the rule points to: `wall` or `district`.
        expected: &'static str,
    },
    /// A district's rule points to a wall, gate or citadel, which are made from the districts
    /// (ADR-066).
    #[error(
        "{field}: `{target}` is a wall, gate or citadel; those are made from the districts, so a district rule cannot point to them"
    )]
    DistrictRuleToDefence {
        /// The field path of the target.
        field: String,
        /// The target as written.
        target: String,
    },
    /// Objects wait for each other in a ring: each is placed after what its rules, its
    /// `encloses` or its `wall` point to, and the last points back to the first (ADR-066).
    #[error(
        "{list}: {} wait for each other in a ring ({}); remove a rule, or pin a landmark, open space or crossing in the ring",
        objects.join(" -> "),
        arrows.join("; ")
    )]
    DependencyRing {
        /// The field path of the list of the ring's first object.
        list: &'static str,
        /// The objects of the ring, each followed by one it waits for, the first repeated at
        /// the end. The ring starts at its first landmark, open space or crossing in the
        /// order of the lists, else at its first object.
        objects: Vec<String>,
        /// For each arrow of `objects`, the field that makes the object wait, with its rule
        /// and target, such as `districts[0].constraints[0] near market` or
        /// `defences.gates[0].wall town_wall`.
        arrows: Vec<String>,
    },
}

/// Checks every rule of specification-version-1 §5 and ADR-056 that the fields read so far
/// can break.
///
/// # Errors
///
/// Every broken rule: header fields, identifiers, terrain, water, land cover, districts,
/// walls, citadel, gates, landmarks, open space, crossings, roads, rule targets and
/// dependency rings, then presets; each problem once.
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
    check_walls(specification, &mut errors);
    check_citadel_and_gates(specification, &mut errors);
    check_landmarks(specification, &mut errors);
    check_open_space_and_crossings(specification, &mut errors);
    check_roads(specification, &mut errors);
    check_dependencies(specification, &mut errors);
    check_presets(specification, &mut errors);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn check_identifiers(specification: &Specification, errors: &mut Vec<ValidationError>) {
    let mut seen = BTreeSet::new();
    let mut reported = BTreeSet::new();
    for (list, identifier) in file_identifiers(specification) {
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

/// Every identifier of the file with the field path of its list, in the order the identifier
/// checks read them; a repeat is reported at its second place in this order.
fn file_identifiers(specification: &Specification) -> Vec<(&'static str, &String)> {
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
    let landmarks = specification
        .landmarks
        .iter()
        .map(|landmark| (LANDMARKS, &landmark.identifier));
    let walls = specification
        .defences
        .walls
        .iter()
        .map(|wall| (WALLS, &wall.identifier));
    let citadel = specification
        .defences
        .citadel
        .iter()
        .map(|citadel| (CITADEL, &citadel.identifier));
    let gates = specification
        .defences
        .gates
        .iter()
        .map(|gate| (GATES, &gate.identifier));
    let open_space = specification
        .open_space
        .iter()
        .map(|space| (OPEN_SPACE, &space.identifier));
    let crossings = specification
        .crossings
        .iter()
        .map(|crossing| (CROSSINGS, &crossing.identifier));
    let roads = specification
        .roads
        .iter()
        .map(|road| (ROADS, &road.identifier));
    rivers
        .chain(lakes)
        .chain(harbour)
        .chain(canals)
        .chain(districts)
        .chain(landmarks)
        .chain(open_space)
        .chain(crossings)
        .chain(walls)
        .chain(citadel)
        .chain(gates)
        .chain(roads)
        .collect()
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

fn check_walls(specification: &Specification, errors: &mut Vec<ValidationError>) {
    let walls = &specification.defences.walls;
    let is_district = |name: &String| {
        specification
            .districts
            .iter()
            .any(|district| district.identifier == *name)
    };
    let inner_walls: BTreeMap<&String, Vec<&String>> = walls
        .iter()
        .map(|wall| {
            let inner = wall
                .encloses
                .iter()
                .filter(|entry| walls.iter().any(|other| other.identifier == **entry))
                .collect();
            (&wall.identifier, inner)
        })
        .collect();
    for (index, wall) in walls.iter().enumerate() {
        for entry in &wall.encloses {
            if !is_district(entry) && !inner_walls.contains_key(entry) {
                errors.push(ValidationError::EnclosesNeitherDistrictNorWall {
                    wall: wall.identifier.clone(),
                    entry: entry.clone(),
                });
            }
        }
        match (wall.separate_ring, wall.gap_metres) {
            (false, Some(_)) => errors.push(ValidationError::GapWithoutSeparateRing {
                wall: wall.identifier.clone(),
            }),
            (true, Some(gap)) => {
                check_size(
                    gap.get(),
                    &format!("{WALLS}[{index}]"),
                    "gap_metres",
                    errors,
                );
            }
            (_, None) => {}
        }
    }
    let rings = rings(walls.iter().map(|wall| &wall.identifier), &inner_walls);
    if rings.is_empty() {
        check_entries_under_walls(specification, &inner_walls, errors);
    }
    errors.extend(
        rings
            .into_iter()
            .map(|walls| ValidationError::WallRing { walls }),
    );
}

/// The rings of a graph whose `next` gives, for every object, the objects it points to: each
/// ring once, starting at its first object in `order`, and every object in at most one ring,
/// so a ring through an object of an earlier ring is left for the next run. Walls that
/// enclose each other and objects that wait for each other share it.
fn rings<'a>(
    order: impl Iterator<Item = &'a String>,
    next: &BTreeMap<&'a String, Vec<&'a String>>,
) -> Vec<Vec<String>> {
    let mut rings = Vec::new();
    let mut in_a_ring: BTreeSet<&String> = BTreeSet::new();
    for start in order {
        if in_a_ring.contains(start) {
            continue;
        }
        let mut path = vec![start];
        let mut seen = in_a_ring.clone();
        seen.insert(start);
        if ring_through(start, start, next, &mut path, &mut seen) {
            in_a_ring.extend(path.iter().copied());
            path.push(start);
            rings.push(path.into_iter().cloned().collect());
        }
    }
    rings
}

/// Whether a chain of `next` through no object of `seen` leads from `current` back to
/// `start`; `path` then holds it.
fn ring_through<'a>(
    start: &String,
    current: &'a String,
    next: &BTreeMap<&'a String, Vec<&'a String>>,
    path: &mut Vec<&'a String>,
    seen: &mut BTreeSet<&'a String>,
) -> bool {
    for following in &next[current] {
        if *following == start {
            return true;
        }
        if seen.insert(following) {
            path.push(following);
            if ring_through(start, following, next, path, seen) {
                return true;
            }
            path.pop();
        }
    }
    false
}

fn check_entries_under_walls(
    specification: &Specification,
    inner_walls: &BTreeMap<&String, Vec<&String>>,
    errors: &mut Vec<ValidationError>,
) {
    let encloses = |outer: &String, inner: &String| {
        let mut seen = BTreeSet::new();
        let mut open: Vec<&String> = inner_walls[outer].clone();
        while let Some(wall) = open.pop() {
            if seen.insert(wall) {
                open.extend(inner_walls[wall].iter().copied());
            }
        }
        seen.contains(inner)
    };
    let is_district = |name: &String| {
        specification
            .districts
            .iter()
            .any(|district| district.identifier == *name)
    };
    let mut listed: BTreeMap<&String, BTreeSet<&String>> = BTreeMap::new();
    for wall in &specification.defences.walls {
        for entry in &wall.encloses {
            if is_district(entry) || inner_walls.contains_key(entry) {
                listed.entry(entry).or_default().insert(&wall.identifier);
            }
        }
    }
    for (entry, lists) in listed {
        let lists: Vec<&String> = lists.into_iter().collect();
        for (position, first) in lists.iter().enumerate() {
            for second in &lists[position + 1..] {
                if !encloses(first, second) && !encloses(second, first) {
                    errors.push(ValidationError::UnderTwoWalls {
                        entry: entry.clone(),
                        first: (*first).clone(),
                        second: (*second).clone(),
                    });
                }
            }
        }
    }
}

fn check_citadel_and_gates(specification: &Specification, errors: &mut Vec<ValidationError>) {
    let defences = &specification.defences;
    let is_wall = |name: &String| defences.walls.iter().any(|wall| wall.identifier == *name);
    if let Some(citadel) = &defences.citadel {
        check_bearing(citadel.bearing_degrees, CITADEL, "bearing_degrees", errors);
        if !is_wall(&citadel.wall) {
            errors.push(ValidationError::CitadelWithoutWall {
                citadel: citadel.identifier.clone(),
                wall: citadel.wall.clone(),
            });
        }
    }
    for (index, gate) in defences.gates.iter().enumerate() {
        if !is_wall(&gate.wall) {
            errors.push(ValidationError::GateWithoutWall {
                gate: gate.identifier.clone(),
                wall: gate.wall.clone(),
            });
        }
        match &gate.kind {
            GateKind::RoadGate { road, end } => {
                match specification
                    .roads
                    .iter()
                    .find(|other| other.identifier == *road)
                {
                    None => errors.push(ValidationError::GateWithoutRoad {
                        gate: gate.identifier.clone(),
                        road: road.clone(),
                    }),
                    Some(found) => {
                        if let Some(end) = end.as_ref().filter(|end| !found.ends.contains(end)) {
                            errors.push(ValidationError::GateEndNotOnRoad {
                                gate: gate.identifier.clone(),
                                end: String::from(end.clone()),
                                road: road.clone(),
                            });
                        }
                    }
                }
            }
            GateKind::Postern { bearing_degrees } | GateKind::WaterGate { bearing_degrees } => {
                let owner = format!("{GATES}[{index}]");
                check_bearing(*bearing_degrees, &owner, "bearing_degrees", errors);
            }
        }
    }
}

fn check_landmarks(specification: &Specification, errors: &mut Vec<ValidationError>) {
    let count = specification.landmarks.len();
    if count > MAXIMUM_LANDMARKS {
        errors.push(ValidationError::TooManyLandmarks { count });
    }
    for (index, landmark) in specification.landmarks.iter().enumerate() {
        let owner = format!("{LANDMARKS}[{index}]");
        if let Some(footprint) = &landmark.footprint {
            let field = format!("{owner}.footprint");
            for (key, side) in [
                ("width_metres", footprint.width_metres),
                ("depth_metres", footprint.depth_metres),
            ] {
                if let Some(side) = side {
                    check_size(side.get(), &field, key, errors);
                }
            }
        }
        check_placement(landmark.resolved.as_ref(), landmark.pinned, &owner, errors);
        check_constraints(&landmark.constraints, &owner, errors);
    }
}

fn check_open_space_and_crossings(
    specification: &Specification,
    errors: &mut Vec<ValidationError>,
) {
    for (index, space) in specification.open_space.iter().enumerate() {
        let owner = format!("{OPEN_SPACE}[{index}]");
        check_placement(space.resolved.as_ref(), space.pinned, &owner, errors);
        check_constraints(&space.constraints, &owner, errors);
    }
    for (index, crossing) in specification.crossings.iter().enumerate() {
        let owner = format!("{CROSSINGS}[{index}]");
        let on_a_river = specification
            .site
            .water
            .rivers
            .iter()
            .any(|river| river.identifier == crossing.river);
        if !on_a_river {
            errors.push(ValidationError::CrossingWithoutRiver {
                crossing: crossing.identifier.clone(),
                river: crossing.river.clone(),
            });
        }
        check_placement(crossing.resolved.as_ref(), crossing.pinned, &owner, errors);
        check_constraints(&crossing.constraints, &owner, errors);
    }
}

/// Checks a `resolved` placement and its `pinned` flag; shared by every object that has them.
fn check_placement(
    resolved: Option<&Placement>,
    pinned: bool,
    owner: &str,
    errors: &mut Vec<ValidationError>,
) {
    let Some(placement) = resolved else {
        if pinned {
            errors.push(ValidationError::PinnedWithoutPlacement {
                field: owner.to_string(),
            });
        }
        return;
    };
    let field = format!("{owner}.resolved");
    let degrees = placement.rotation_degrees.degrees();
    if !ROTATION_DEGREES.contains(&degrees) {
        errors.push(ValidationError::RotationOutOfRange {
            field: format!("{field}.rotation_degrees"),
            degrees: degrees.to_string(),
        });
    }
    check_size(placement.width_metres.get(), &field, "width_metres", errors);
    check_size(placement.depth_metres.get(), &field, "depth_metres", errors);
}

/// A remark about a specification that does not stop generation (REQUIREMENTS.md F20).
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ValidationWarning {
    /// A landmark's label is not one the default rule pack knows; it may be a typo.
    #[error("{field}: label `{label}` is not a known label ({KNOWN_LABELS:?}); is it a typo?")]
    UnknownLabel {
        /// The field path of the label.
        field: String,
        /// The label as written.
        label: String,
    },
    /// The style is not one that exists; it may be a typo.
    #[error(
        "{PRESETS}.style: style `{style}` is not a known style ({KNOWN_STYLES:?}); is it a typo?"
    )]
    UnknownStyle {
        /// The style as written.
        style: String,
    },
    /// A rule pack is not one that exists; it may be a typo.
    #[error(
        "{field}: pack `{pack}` is not a known rule pack ({KNOWN_PACKS:?}); is it a typo, or a pack that does not exist yet?"
    )]
    UnknownPack {
        /// The field path of the pack.
        field: String,
        /// The pack as written.
        pack: String,
    },
}

/// The remarks about a specification that does not stop generation; they never change the
/// exit code (REQUIREMENTS.md F20, ADR-061).
pub fn warnings(specification: &Specification) -> Vec<ValidationWarning> {
    let presets = &specification.presets;
    let style = (!KNOWN_STYLES.contains(&presets.style.as_str())).then(|| {
        ValidationWarning::UnknownStyle {
            style: presets.style.clone(),
        }
    });
    let packs = presets
        .packs
        .iter()
        .enumerate()
        .filter(|(_, pack)| !KNOWN_PACKS.contains(&pack.as_str()))
        .map(|(index, pack)| ValidationWarning::UnknownPack {
            field: format!("{PRESETS}.packs[{index}]"),
            pack: pack.clone(),
        });
    let labels = specification
        .landmarks
        .iter()
        .enumerate()
        .filter_map(|(index, landmark)| {
            let label = landmark.label.as_ref()?;
            (!KNOWN_LABELS.contains(&label.as_str())).then(|| ValidationWarning::UnknownLabel {
                field: format!("{LANDMARKS}[{index}].label"),
                label: label.clone(),
            })
        });
    labels.chain(style).chain(packs).collect()
}

fn check_presets(specification: &Specification, errors: &mut Vec<ValidationError>) {
    let presets = &specification.presets;
    for (key, size) in [
        ("reuse_within_metres", presets.reuse_within_metres.get()),
        ("reuse_weight", presets.reuse_weight.get()),
        ("centre_radius_metres", presets.centre_radius_metres.get()),
    ] {
        check_size(size, PRESETS, key, errors);
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
                RoadEnd::Place(place) => {
                    let landmarks = specification
                        .landmarks
                        .iter()
                        .map(|landmark| (&landmark.identifier, landmark.pinned));
                    let open_space = specification
                        .open_space
                        .iter()
                        .map(|space| (&space.identifier, space.pinned));
                    let pinned = landmarks
                        .chain(open_space)
                        .find(|(identifier, _)| *identifier == place)
                        .map(|(_, pinned)| pinned);
                    let (road, place) = (named(), place.clone());
                    match pinned {
                        Some(true) => {}
                        Some(false) => {
                            errors.push(ValidationError::RoadEndNotPinned { road, place });
                        }
                        None => errors.push(ValidationError::RoadEndAtPlace { road, place }),
                    }
                }
                RoadEnd::Region(_) | RoadEnd::Core => {}
            }
        }
    }
}

/// What an identifier of the file names, for rule targets and the dependency graph
/// (ADR-066).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Named {
    /// A district.
    District,
    /// A wall.
    Wall,
    /// A gate or the citadel.
    GateOrCitadel,
    /// A landmark, open space or crossing.
    Placed {
        /// Whether its `pinned` is true; a pinned object is an input, so nothing waits for it.
        pinned: bool,
    },
    /// A river, lake, harbour, canal or road, laid out in S0 before every object.
    SiteFeature,
}

impl Named {
    /// Whether an object that points to this one is placed after it.
    const fn is_waited_for(self) -> bool {
        !matches!(self, Named::SiteFeature | Named::Placed { pinned: true })
    }
}

/// For one object of the dependency graph, each object it waits for with the field that makes
/// it wait, such as `districts[0].constraints[0] near market`.
type Arrows<'a> = Vec<(&'a String, String)>;

/// Checks every rule target, then finds the objects that wait for each other in a ring
/// (ADR-066). A wall ring is left to `check_walls`. A file with an identifier used twice is
/// not checked, because `check_identifiers` reports it and the two objects would be one.
fn check_dependencies(specification: &Specification, errors: &mut Vec<ValidationError>) {
    let identifiers = file_identifiers(specification);
    let unique: BTreeSet<&String> = identifiers
        .iter()
        .map(|(_, identifier)| *identifier)
        .collect();
    if unique.len() < identifiers.len() {
        return;
    }
    let objects = graph_objects(specification);
    let mut names: BTreeMap<&String, Named> = unique
        .into_iter()
        .map(|identifier| (identifier, Named::SiteFeature))
        .collect();
    names.extend(
        objects
            .iter()
            .map(|(_, identifier, named)| (*identifier, *named)),
    );
    let lists: BTreeMap<&String, &'static str> = objects
        .iter()
        .map(|(list, identifier, _)| (*identifier, *list))
        .collect();
    let graph = dependencies(specification, &names, errors);
    let next = graph
        .iter()
        .map(|(object, arrows)| (*object, arrows.iter().map(|(target, _)| *target).collect()))
        .collect();
    let mut fields: BTreeMap<(&String, &String), &String> = BTreeMap::new();
    for (object, arrows) in &graph {
        for (target, field) in arrows {
            fields.entry((*object, *target)).or_insert(field);
        }
    }
    let is_pinnable = |named: &Named| matches!(named, Named::Placed { .. });
    let pinnable = objects.iter().filter(|(_, _, named)| is_pinnable(named));
    let others = objects.iter().filter(|(_, _, named)| !is_pinnable(named));
    let order = pinnable.chain(others).map(|(_, identifier, _)| *identifier);
    for ring in rings(order, &next) {
        if ring
            .iter()
            .all(|identifier| names[identifier] == Named::Wall)
        {
            continue;
        }
        let arrows = ring
            .windows(2)
            .map(|pair| fields[&(&pair[0], &pair[1])].clone())
            .collect();
        errors.push(ValidationError::DependencyRing {
            list: lists[&ring[0]],
            objects: ring,
            arrows,
        });
    }
}

/// The objects of the dependency graph with their list and kind, in the order of the file's
/// lists (REQUIREMENTS.md F2): walls, citadel, gates, open space, crossings, districts,
/// landmarks.
fn graph_objects(specification: &Specification) -> Vec<(&'static str, &String, Named)> {
    let defences = &specification.defences;
    let placed = |pinned| Named::Placed { pinned };
    defences
        .walls
        .iter()
        .map(|wall| (WALLS, &wall.identifier, Named::Wall))
        .chain(
            defences
                .citadel
                .iter()
                .map(|citadel| (CITADEL, &citadel.identifier, Named::GateOrCitadel)),
        )
        .chain(
            defences
                .gates
                .iter()
                .map(|gate| (GATES, &gate.identifier, Named::GateOrCitadel)),
        )
        .chain(
            specification
                .open_space
                .iter()
                .map(|space| (OPEN_SPACE, &space.identifier, placed(space.pinned))),
        )
        .chain(
            specification
                .crossings
                .iter()
                .map(|crossing| (CROSSINGS, &crossing.identifier, placed(crossing.pinned))),
        )
        .chain(
            specification
                .districts
                .iter()
                .map(|district| (DISTRICTS, &district.identifier, Named::District)),
        )
        .chain(
            specification
                .landmarks
                .iter()
                .map(|landmark| (LANDMARKS, &landmark.identifier, placed(landmark.pinned))),
        )
        .collect()
}

/// For every object of the graph, the objects it waits for: the districts and walls a wall
/// encloses, the wall of a gate or the citadel, and the rule targets that are neither pinned
/// nor laid out in S0. Checks every rule target on the way.
fn dependencies<'a>(
    specification: &'a Specification,
    names: &BTreeMap<&'a String, Named>,
    errors: &mut Vec<ValidationError>,
) -> BTreeMap<&'a String, Arrows<'a>> {
    let defences = &specification.defences;
    let walls = defences.walls.iter().enumerate().map(|(index, wall)| {
        let arrows = wall
            .encloses
            .iter()
            .filter(|entry| matches!(names.get(entry), Some(Named::District | Named::Wall)))
            .map(|entry| (entry, format!("{WALLS}[{index}].encloses {entry}")))
            .collect();
        (&wall.identifier, arrows)
    });
    let wall_of = |field: String, wall: &'a String| -> Arrows<'a> {
        if names.get(wall) == Some(&Named::Wall) {
            vec![(wall, format!("{field} {wall}"))]
        } else {
            Vec::new()
        }
    };
    let citadel = defences.citadel.iter().map(|citadel| {
        let arrows = wall_of(format!("{CITADEL}.wall"), &citadel.wall);
        (&citadel.identifier, arrows)
    });
    let gates = defences.gates.iter().enumerate().map(|(index, gate)| {
        let arrows = wall_of(format!("{GATES}[{index}].wall"), &gate.wall);
        (&gate.identifier, arrows)
    });
    let mut graph: BTreeMap<&String, Arrows> = walls.chain(citadel).chain(gates).collect();
    let constrained = specification
        .open_space
        .iter()
        .enumerate()
        .map(|(index, space)| {
            let owner = format!("{OPEN_SPACE}[{index}]");
            (owner, &space.identifier, &space.constraints, false)
        })
        .chain(
            specification
                .crossings
                .iter()
                .enumerate()
                .map(|(index, crossing)| {
                    let owner = format!("{CROSSINGS}[{index}]");
                    (owner, &crossing.identifier, &crossing.constraints, false)
                }),
        )
        .chain(
            specification
                .districts
                .iter()
                .enumerate()
                .map(|(index, district)| {
                    let owner = format!("{DISTRICTS}[{index}]");
                    (owner, &district.identifier, &district.constraints, true)
                }),
        )
        .chain(
            specification
                .landmarks
                .iter()
                .enumerate()
                .map(|(index, landmark)| {
                    let owner = format!("{LANDMARKS}[{index}]");
                    (owner, &landmark.identifier, &landmark.constraints, false)
                }),
        );
    for (owner, identifier, constraints, is_district) in constrained {
        let arrows = rule_targets(constraints, &owner, is_district, names, errors)
            .into_iter()
            .filter(|(target, _)| names.get(target).is_some_and(|named| named.is_waited_for()))
            .collect();
        graph.insert(identifier, arrows);
    }
    graph
}

/// Checks the targets of one object's rules and returns the targets of the right kind, each
/// with its rule's field, word and target, such as `landmarks[2].constraints[0] near market`;
/// the object waits for them unless they are pinned or laid out in S0 (ADR-066). `owner` is
/// the field path of the object, such as `landmarks[2]`. A district's wall rules are left to
/// `check_districts`.
fn rule_targets<'a>(
    constraints: &'a [Constraint],
    owner: &str,
    is_district: bool,
    names: &BTreeMap<&String, Named>,
    errors: &mut Vec<ValidationError>,
) -> Arrows<'a> {
    let mut targets = Vec::new();
    for (index, constraint) in constraints.iter().enumerate() {
        let wall = Some((Named::Wall, "wall"));
        let district = Some((Named::District, "district"));
        let (target, word, expected) = match &constraint.rule {
            Rule::Near { target, .. } => (target, "near", None),
            Rule::FarFrom { target, .. } => (target, "far_from", None),
            Rule::InsideWalls { target } if !is_district => (target, "inside_walls", wall),
            Rule::OutsideWalls { target } if !is_district => (target, "outside_walls", wall),
            Rule::InDistrict { target } => (target, "in_district", district),
            Rule::NotInDistrict { target } => (target, "not_in_district", district),
            Rule::InsideWalls { .. }
            | Rule::OutsideWalls { .. }
            | Rule::CompassRegion { .. }
            | Rule::NotInCompassRegion { .. }
            | Rule::OnWater
            | Rule::AwayFromWater
            | Rule::OnHighGround
            | Rule::OnLowGround
            | Rule::MinSpacing { .. } => continue,
        };
        let rule_field = format!("{owner}.constraints[{index}]");
        let field = format!("{rule_field}.target");
        let wrong_kind = |field, expected| ValidationError::TargetOfWrongKind {
            field,
            target: target.clone(),
            expected,
        };
        if target == CORE_WORD {
            if let Some((_, expected)) = expected {
                errors.push(wrong_kind(field, expected));
            }
            continue;
        }
        match names.get(target) {
            None => errors.push(ValidationError::TargetMissing {
                field,
                target: target.clone(),
            }),
            Some(Named::Wall | Named::GateOrCitadel) if is_district => {
                errors.push(ValidationError::DistrictRuleToDefence {
                    field,
                    target: target.clone(),
                });
            }
            Some(named) => match expected {
                Some((kind, expected)) if *named != kind => {
                    errors.push(wrong_kind(field, expected));
                }
                Some(_) | None => targets.push((target, format!("{rule_field} {word} {target}"))),
            },
        }
    }
    targets
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::random::Seed;
    use crate::specification::{
        AlongWater, Band, Canal, Citadel, Cliff, Coast, Constraint, Crossing, Defences, District,
        DistrictFunction, Enclosure, Era, Footprint, FootprintClass, Gate, Harbour, Hill, Hollow,
        Identity, Importance, InsideCover, Jurisdiction, Lake, LandCover, Landmark,
        LandmarkFunction, OpenSpace, OpenSpaceKind, OutsideCover, Placement, Plateau, Presets,
        River, RiverCrossingKind, Road, Rule, Settlement, Site, Slope, Terrain, Wall, WallKind,
        Water, Wealth,
    };
    use crate::units::{Gradient, Length, Rotation, Weight};

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
            landmarks: Vec::new(),
            defences: Defences::default(),
            open_space: Vec::new(),
            crossings: Vec::new(),
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
            presets: Presets::new(Era::EarlyMedieval),
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
        let target = || "outer".to_string();
        specification.districts[0].constraints = [
            Rule::InsideWalls { target: target() },
            Rule::OutsideWalls { target: target() },
            Rule::MinSpacing {
                metres: length(20.0),
            },
        ]
        .into_iter()
        .map(|rule| Constraint {
            rule,
            hard: true,
            weight: Weight::DEFAULT,
        })
        .collect();
        let field = |index: usize| ValidationError::DistrictWallRule {
            field: format!("districts[0].constraints[{index}]"),
        };
        assert_eq!(validate(&specification), Err(vec![field(0), field(1)]));
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
    fn a_district_identifier_is_checked_for_reserved_words() {
        let mut specification = valid();
        specification.districts[0].identifier = "north".to_string();
        assert_eq!(
            only_error(&specification),
            ValidationError::ReservedIdentifier {
                list: DISTRICTS,
                identifier: "north".to_string(),
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

    fn placement(rotation_degrees: f64) -> Placement {
        Placement {
            x: length(1200.0),
            y: length(1300.0),
            rotation_degrees: Rotation::new(rotation_degrees).expect("test input is a valid turn"),
            width_metres: length(20.0),
            depth_metres: length(40.0),
        }
    }

    fn guildhall(pinned: bool) -> Landmark {
        Landmark {
            identifier: "guildhall".to_string(),
            name: "The Guildhall".to_string(),
            function: LandmarkFunction::Civic,
            label: Some("guildhall".to_string()),
            importance: Importance::Major,
            footprint: Some(Footprint {
                class: Some(FootprintClass::Medium),
                width_metres: None,
                depth_metres: Some(length(40.0)),
            }),
            constraints: Vec::new(),
            resolved: Some(placement(90.0)),
            pinned,
            notes: String::new(),
        }
    }

    fn with_landmark(landmark: Landmark) -> Specification {
        let mut specification = valid();
        specification.landmarks.push(landmark);
        specification
    }

    #[test]
    fn a_pinned_landmark_passes() {
        assert_eq!(validate(&with_landmark(guildhall(true))), Ok(()));
    }

    #[test]
    fn pinned_without_a_placement_is_an_error() {
        let mut landmark = guildhall(true);
        landmark.resolved = None;
        assert_eq!(
            only_error(&with_landmark(landmark)),
            ValidationError::PinnedWithoutPlacement {
                field: "landmarks[0]".to_string(),
            }
        );
    }

    #[test]
    fn a_turn_is_from_0_up_to_360_degrees() {
        for degrees in [-0.01, 360.0] {
            let mut landmark = guildhall(true);
            landmark.resolved = Some(placement(degrees));
            assert_eq!(
                only_error(&with_landmark(landmark)),
                ValidationError::RotationOutOfRange {
                    field: "landmarks[0].resolved.rotation_degrees".to_string(),
                    degrees: degrees.to_string(),
                }
            );
        }
        let mut landmark = guildhall(true);
        landmark.resolved = Some(placement(359.99));
        assert_eq!(validate(&with_landmark(landmark)), Ok(()));
    }

    #[test]
    fn a_landmark_size_of_zero_is_an_error() {
        let mut landmark = guildhall(true);
        if let Some(footprint) = &mut landmark.footprint {
            footprint.depth_metres = Some(length(0.0));
        }
        if let Some(resolved) = &mut landmark.resolved {
            resolved.width_metres = length(0.0);
        }
        let errors = validate(&with_landmark(landmark)).expect_err("two sizes are 0");
        assert_eq!(
            errors,
            [
                ValidationError::ZeroSize {
                    field: "landmarks[0].footprint.depth_metres".to_string(),
                },
                ValidationError::ZeroSize {
                    field: "landmarks[0].resolved.width_metres".to_string(),
                },
            ]
        );
    }

    #[test]
    fn a_city_has_at_most_1000_landmarks() {
        let mut specification = valid();
        specification.landmarks = (0..=MAXIMUM_LANDMARKS)
            .map(|index| Landmark {
                identifier: format!("landmark_{index}"),
                ..guildhall(false)
            })
            .collect();
        assert_eq!(
            only_error(&specification),
            ValidationError::TooManyLandmarks { count: 1001 }
        );
    }

    #[test]
    fn a_road_may_end_at_a_pinned_landmark() {
        let mut specification = with_landmark(guildhall(true));
        specification.roads[1].ends = [RoadEnd::Core, RoadEnd::Place("guildhall".to_string())];
        assert_eq!(validate(&specification), Ok(()));
    }

    #[test]
    fn a_road_to_a_landmark_that_is_not_pinned_says_pin_it_first() {
        let mut specification = with_landmark(guildhall(false));
        specification.roads[1].ends = [RoadEnd::Core, RoadEnd::Place("guildhall".to_string())];
        let error = only_error(&specification);
        assert!(error.to_string().contains("pin it first"), "{error}");
        assert_eq!(
            error,
            ValidationError::RoadEndNotPinned {
                road: "west_road".to_string(),
                place: "guildhall".to_string(),
            }
        );
    }

    #[test]
    fn a_zero_preset_size_is_an_error() {
        for key in [
            "reuse_within_metres",
            "reuse_weight",
            "centre_radius_metres",
        ] {
            let mut specification = valid();
            let presets = &mut specification.presets;
            match key {
                "reuse_within_metres" => presets.reuse_within_metres = length(0.0),
                "reuse_weight" => presets.reuse_weight = Weight::new(0.0).expect("0 is a weight"),
                _ => presets.centre_radius_metres = length(0.0),
            }
            assert_eq!(
                only_error(&specification),
                ValidationError::ZeroSize {
                    field: format!("presets.{key}"),
                }
            );
        }
    }

    #[test]
    fn an_unknown_style_or_pack_is_a_warning_not_an_error() {
        let mut specification = valid();
        specification.presets.style = "inc".to_string();
        specification.presets.packs = vec!["jotun".to_string()];
        assert_eq!(validate(&specification), Ok(()));
        assert_eq!(
            warnings(&specification),
            [
                ValidationWarning::UnknownStyle {
                    style: "inc".to_string(),
                },
                ValidationWarning::UnknownPack {
                    field: "presets.packs[0]".to_string(),
                    pack: "jotun".to_string(),
                },
            ]
        );
        assert_eq!(warnings(&valid()), []);
    }

    #[test]
    fn an_unknown_label_is_a_warning_not_an_error() {
        let mut landmark = guildhall(true);
        landmark.label = Some("cathdral".to_string());
        let specification = with_landmark(landmark);
        assert_eq!(validate(&specification), Ok(()));
        assert_eq!(
            warnings(&specification),
            [ValidationWarning::UnknownLabel {
                field: "landmarks[0].label".to_string(),
                label: "cathdral".to_string(),
            }]
        );
        assert_eq!(warnings(&with_landmark(guildhall(true))), []);
    }

    #[test]
    fn a_landmark_and_a_district_share_no_identifier() {
        let mut landmark = guildhall(true);
        landmark.identifier = "old_town".to_string();
        assert_eq!(
            only_error(&with_landmark(landmark)),
            ValidationError::DuplicateIdentifier {
                list: LANDMARKS,
                identifier: "old_town".to_string(),
            }
        );
    }

    fn market(pinned: bool) -> OpenSpace {
        OpenSpace {
            identifier: "market".to_string(),
            kind: OpenSpaceKind::MarketSquare,
            constraints: Vec::new(),
            resolved: Some(placement(0.0)),
            pinned,
        }
    }

    fn bridge(river: &str) -> Crossing {
        Crossing {
            identifier: "ash_bridge".to_string(),
            river: river.to_string(),
            kind: RiverCrossingKind::Bridge,
            constraints: Vec::new(),
            resolved: None,
            pinned: false,
        }
    }

    fn with_market_and_bridge(market: OpenSpace, bridge: Crossing) -> Specification {
        let mut specification = valid();
        specification.open_space.push(market);
        specification.crossings.push(bridge);
        specification
    }

    #[test]
    fn a_pinned_open_space_or_crossing_needs_a_placement() {
        let mut space = market(true);
        space.resolved = None;
        let mut crossing = bridge("ash");
        crossing.pinned = true;
        let errors = validate(&with_market_and_bridge(space, crossing))
            .expect_err("two pins have nothing to keep");
        assert_eq!(
            errors,
            [
                ValidationError::PinnedWithoutPlacement {
                    field: "open_space[0]".to_string(),
                },
                ValidationError::PinnedWithoutPlacement {
                    field: "crossings[0]".to_string(),
                },
            ]
        );
    }

    #[test]
    fn a_crossing_names_a_river_of_the_file() {
        for river in ["brook", "west_road"] {
            assert_eq!(
                only_error(&with_market_and_bridge(market(true), bridge(river))),
                ValidationError::CrossingWithoutRiver {
                    crossing: "ash_bridge".to_string(),
                    river: river.to_string(),
                }
            );
        }
    }

    #[test]
    fn open_space_and_crossing_rules_are_checked_like_every_rule() {
        let weightless = || Constraint {
            rule: Rule::OnWater,
            hard: false,
            weight: weight(0.0),
        };
        let mut space = market(true);
        space.constraints.push(weightless());
        let mut crossing = bridge("ash");
        crossing.constraints.push(weightless());
        let zero = |owner: &str| ValidationError::ZeroSize {
            field: format!("{owner}.constraints[0].weight"),
        };
        assert_eq!(
            validate(&with_market_and_bridge(space, crossing)),
            Err(vec![zero("open_space[0]"), zero("crossings[0]")])
        );
    }

    #[test]
    fn a_road_may_end_at_a_pinned_open_space_only() {
        let mut specification = with_market_and_bridge(market(true), bridge("ash"));
        specification.roads[1].ends = [RoadEnd::Core, RoadEnd::Place("market".to_string())];
        assert_eq!(validate(&specification), Ok(()));
        specification.open_space[0].pinned = false;
        assert_eq!(
            only_error(&specification),
            ValidationError::RoadEndNotPinned {
                road: "west_road".to_string(),
                place: "market".to_string(),
            }
        );
    }

    #[test]
    fn an_open_space_and_a_crossing_share_no_identifier() {
        let mut crossing = bridge("ash");
        crossing.identifier = "market".to_string();
        assert_eq!(
            only_error(&with_market_and_bridge(market(true), crossing)),
            ValidationError::DuplicateIdentifier {
                list: CROSSINGS,
                identifier: "market".to_string(),
            }
        );
    }

    fn wall(identifier: &str, encloses: &[&str]) -> Wall {
        Wall {
            identifier: identifier.to_string(),
            kind: WallKind::Stone,
            encloses: encloses.iter().map(ToString::to_string).collect(),
            along_water: AlongWater::Wall,
            separate_ring: false,
            gap_metres: None,
        }
    }

    fn with_walls(walls: Vec<Wall>) -> Specification {
        let mut specification = valid();
        let docks = District {
            identifier: "docks".to_string(),
            ..specification.districts[0].clone()
        };
        specification.districts.push(docks);
        specification.defences.walls = walls;
        specification
    }

    fn with_defences(citadel: Option<Citadel>, gates: Vec<Gate>) -> Specification {
        let mut specification = with_walls(vec![wall("town_wall", &["old_town", "docks"])]);
        specification.defences.citadel = citadel;
        specification.defences.gates = gates;
        specification
    }

    fn citadel(wall: &str, bearing_degrees: Bearing) -> Citadel {
        Citadel {
            identifier: "keep".to_string(),
            bearing_degrees,
            wall: wall.to_string(),
        }
    }

    fn gate(identifier: &str, wall: &str, kind: GateKind) -> Gate {
        Gate {
            kind,
            identifier: identifier.to_string(),
            wall: wall.to_string(),
        }
    }

    fn road_gate(road: &str, end: Option<RoadEnd>) -> GateKind {
        GateKind::RoadGate {
            road: road.to_string(),
            end,
        }
    }

    #[test]
    fn a_citadel_and_every_gate_kind_pass() {
        let gates = vec![
            gate(
                "north_gate",
                "town_wall",
                road_gate("north_road", Some(RoadEnd::Region(CompassRegion::North))),
            ),
            gate("west_gate", "town_wall", road_gate("west_road", None)),
            gate(
                "sally_port",
                "town_wall",
                GateKind::Postern {
                    bearing_degrees: Bearing::Degrees(200),
                },
            ),
            gate(
                "quay_gate",
                "town_wall",
                GateKind::WaterGate {
                    bearing_degrees: Bearing::Word(CompassRegion::East),
                },
            ),
        ];
        let citadel = citadel("town_wall", Bearing::Word(CompassRegion::North));
        assert_eq!(validate(&with_defences(Some(citadel), gates)), Ok(()));
    }

    #[test]
    fn a_citadel_is_part_of_a_wall() {
        for wall in ["nowhere", "docks"] {
            let citadel = citadel(wall, Bearing::Degrees(0));
            assert_eq!(
                only_error(&with_defences(Some(citadel), Vec::new())),
                ValidationError::CitadelWithoutWall {
                    citadel: "keep".to_string(),
                    wall: wall.to_string(),
                }
            );
        }
    }

    #[test]
    fn a_gate_names_a_wall_and_a_road() {
        for wall in ["nowhere", "docks"] {
            let gates = vec![gate("north_gate", wall, road_gate("north_road", None))];
            assert_eq!(
                only_error(&with_defences(None, gates)),
                ValidationError::GateWithoutWall {
                    gate: "north_gate".to_string(),
                    wall: wall.to_string(),
                }
            );
        }
        for road in ["nowhere", "town_wall"] {
            let gates = vec![gate("north_gate", "town_wall", road_gate(road, None))];
            assert_eq!(
                only_error(&with_defences(None, gates)),
                ValidationError::GateWithoutRoad {
                    gate: "north_gate".to_string(),
                    road: road.to_string(),
                }
            );
        }
    }

    #[test]
    fn a_road_gate_end_is_one_of_its_road_ends() {
        let west = Some(RoadEnd::Region(CompassRegion::West));
        let gates = vec![gate(
            "north_gate",
            "town_wall",
            road_gate("north_road", west),
        )];
        assert_eq!(
            only_error(&with_defences(None, gates)),
            ValidationError::GateEndNotOnRoad {
                gate: "north_gate".to_string(),
                end: "west".to_string(),
                road: "north_road".to_string(),
            }
        );
    }

    #[test]
    fn citadel_and_gate_bearings_are_checked() {
        let postern = GateKind::Postern {
            bearing_degrees: Bearing::Degrees(400),
        };
        let citadel = citadel("town_wall", Bearing::Word(CompassRegion::Centre));
        assert_eq!(
            validate(&with_defences(
                Some(citadel),
                vec![gate("sally_port", "town_wall", postern)]
            )),
            Err(vec![
                ValidationError::BearingAtCentre {
                    field: "defences.citadel.bearing_degrees".to_string(),
                },
                ValidationError::BearingOutOfRange {
                    field: "defences.gates[0].bearing_degrees".to_string(),
                    degrees: 400,
                },
            ])
        );
    }

    #[test]
    fn a_gate_and_a_wall_share_no_identifier() {
        let gates = vec![gate(
            "town_wall",
            "town_wall",
            road_gate("north_road", None),
        )];
        assert_eq!(
            only_error(&with_defences(None, gates)),
            ValidationError::DuplicateIdentifier {
                list: GATES,
                identifier: "town_wall".to_string(),
            }
        );
    }

    #[test]
    fn every_citadel_and_gate_message_starts_with_its_list() {
        let name = || "x".to_string();
        let citadel = ValidationError::CitadelWithoutWall {
            citadel: name(),
            wall: name(),
        };
        assert!(citadel.to_string().starts_with(CITADEL), "{citadel}");
        for error in [
            ValidationError::GateWithoutWall {
                gate: name(),
                wall: name(),
            },
            ValidationError::GateWithoutRoad {
                gate: name(),
                road: name(),
            },
            ValidationError::GateEndNotOnRoad {
                gate: name(),
                end: name(),
                road: name(),
            },
        ] {
            assert!(error.to_string().starts_with(GATES), "{error}");
        }
    }

    #[test]
    fn nested_walls_pass() {
        let mut inner = wall("inner", &["old_town"]);
        inner.separate_ring = true;
        inner.gap_metres = Some(length(25.0));
        let walls = vec![wall("outer", &["inner", "old_town", "docks"]), inner];
        assert_eq!(validate(&with_walls(walls)), Ok(()));
    }

    #[test]
    fn encloses_names_a_district_or_a_wall() {
        for entry in ["nowhere", "ash"] {
            assert_eq!(
                only_error(&with_walls(vec![wall("outer", &[entry])])),
                ValidationError::EnclosesNeitherDistrictNorWall {
                    wall: "outer".to_string(),
                    entry: entry.to_string(),
                }
            );
        }
    }

    #[test]
    fn a_wall_that_encloses_itself_is_a_ring() {
        let ring = |names: &[&str]| ValidationError::WallRing {
            walls: names.iter().map(ToString::to_string).collect(),
        };
        assert_eq!(
            only_error(&with_walls(vec![wall("outer", &["outer"])])),
            ring(&["outer", "outer"])
        );
        let walls = vec![
            wall("first", &["second"]),
            wall("second", &["third"]),
            wall("third", &["first", "docks"]),
        ];
        assert_eq!(
            only_error(&with_walls(walls)),
            ring(&["first", "second", "third", "first"])
        );
    }

    #[test]
    fn a_district_under_two_separate_walls_is_an_error() {
        let walls = vec![
            wall("harbour_wall", &["docks"]),
            wall("land_wall", &["docks", "old_town"]),
        ];
        assert_eq!(
            only_error(&with_walls(walls)),
            ValidationError::UnderTwoWalls {
                entry: "docks".to_string(),
                first: "harbour_wall".to_string(),
                second: "land_wall".to_string(),
            }
        );
    }

    #[test]
    fn a_wall_under_two_separate_walls_is_an_error() {
        let walls = vec![
            wall("harbour_wall", &["inner"]),
            wall("land_wall", &["inner"]),
            wall("inner", &["docks"]),
        ];
        assert_eq!(
            only_error(&with_walls(walls)),
            ValidationError::UnderTwoWalls {
                entry: "inner".to_string(),
                first: "harbour_wall".to_string(),
                second: "land_wall".to_string(),
            }
        );
    }

    #[test]
    fn a_district_under_walls_three_levels_apart_passes() {
        let walls = vec![
            wall("outer", &["middle", "docks"]),
            wall("middle", &["inner"]),
            wall("inner", &["docks"]),
        ];
        assert_eq!(validate(&with_walls(walls)), Ok(()));
    }

    #[test]
    fn a_name_listed_twice_or_unknown_is_reported_once() {
        assert_eq!(
            validate(&with_walls(vec![wall("outer", &["docks", "docks"])])),
            Ok(())
        );
        let walls = vec![wall("first", &["nowhere"]), wall("second", &["nowhere"])];
        let errors = validate(&with_walls(walls)).expect_err("`nowhere` is no district");
        assert_eq!(errors.len(), 2, "{errors:?}");
    }

    #[test]
    fn a_ring_is_found_from_a_wall_that_leads_into_it() {
        let walls = vec![
            wall("gatehouse", &["first"]),
            wall("first", &["second"]),
            wall("second", &["first"]),
        ];
        assert_eq!(
            only_error(&with_walls(walls)),
            ValidationError::WallRing {
                walls: vec![
                    "first".to_string(),
                    "second".to_string(),
                    "first".to_string()
                ],
            }
        );
    }

    #[test]
    fn walls_in_a_ring_are_not_also_checked_for_two_walls() {
        let walls = vec![
            wall("first", &["second", "docks"]),
            wall("second", &["first"]),
            wall("third", &["docks"]),
        ];
        let errors = validate(&with_walls(walls)).expect_err("the walls form a ring");
        assert!(
            errors
                .iter()
                .all(|error| matches!(error, ValidationError::WallRing { .. })),
            "{errors:?}"
        );
    }

    #[test]
    fn a_gap_needs_a_separate_ring_and_more_than_0() {
        let mut gap_only = wall("outer", &["docks"]);
        gap_only.gap_metres = Some(length(20.0));
        assert_eq!(
            only_error(&with_walls(vec![gap_only])),
            ValidationError::GapWithoutSeparateRing {
                wall: "outer".to_string(),
            }
        );
        let mut zero_gap = wall("outer", &["docks"]);
        zero_gap.separate_ring = true;
        zero_gap.gap_metres = Some(length(0.0));
        assert_eq!(
            only_error(&with_walls(vec![zero_gap])),
            ValidationError::ZeroSize {
                field: "defences.walls[0].gap_metres".to_string(),
            }
        );
    }

    #[test]
    fn a_wall_and_a_district_share_no_identifier() {
        assert_eq!(
            only_error(&with_walls(vec![wall("docks", &["old_town"])])),
            ValidationError::DuplicateIdentifier {
                list: WALLS,
                identifier: "docks".to_string(),
            }
        );
    }

    #[test]
    fn every_wall_message_starts_with_its_list() {
        let name = || "x".to_string();
        for error in [
            ValidationError::EnclosesNeitherDistrictNorWall {
                wall: name(),
                entry: name(),
            },
            ValidationError::WallRing {
                walls: vec![name(), name()],
            },
            ValidationError::GapWithoutSeparateRing { wall: name() },
            ValidationError::UnderTwoWalls {
                entry: name(),
                first: name(),
                second: name(),
            },
        ] {
            assert!(error.to_string().starts_with(WALLS), "{error}");
        }
    }

    #[test]
    fn every_placed_object_message_starts_with_its_list() {
        let name = || "x".to_string();
        let crossing = ValidationError::CrossingWithoutRiver {
            crossing: name(),
            river: name(),
        };
        assert!(crossing.to_string().starts_with(CROSSINGS), "{crossing}");
        let road = ValidationError::RoadEndNotPinned {
            road: name(),
            place: name(),
        };
        assert!(road.to_string().starts_with(ROADS), "{road}");
        let ring = ValidationError::DependencyRing {
            list: OPEN_SPACE,
            objects: vec![name(), name()],
            arrows: vec![name()],
        };
        assert!(ring.to_string().starts_with(OPEN_SPACE), "{ring}");
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
            ValidationError::PinnedWithoutPlacement { field: width() },
            ValidationError::RotationOutOfRange {
                field: width(),
                degrees: "360".to_string(),
            },
            ValidationError::ZeroSize { field: width() },
            ValidationError::TargetMissing {
                field: width(),
                target: width(),
            },
            ValidationError::TargetOfWrongKind {
                field: width(),
                target: width(),
                expected: "wall",
            },
            ValidationError::DistrictRuleToDefence {
                field: width(),
                target: width(),
            },
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

    fn near(target: &str) -> Constraint {
        let near = Rule::Near {
            target: target.to_string(),
            within_metres: length(150.0),
        };
        rule(near, Weight::DEFAULT)
    }

    fn add_district(specification: &mut Specification, identifier: &str, rules: Vec<Constraint>) {
        let district = District {
            identifier: identifier.to_string(),
            constraints: rules,
            ..specification.districts[0].clone()
        };
        specification.districts.push(district);
    }

    fn identifiers(names: &[&str]) -> Vec<String> {
        names.iter().map(ToString::to_string).collect()
    }

    fn with_gate_and_citadel() -> Specification {
        let gates = vec![gate("west_gate", "town_wall", road_gate("west_road", None))];
        with_defences(Some(citadel("town_wall", Bearing::Degrees(0))), gates)
    }

    #[test]
    fn a_district_may_wait_for_an_open_space_that_waits_for_districts() {
        let mut space = market(false);
        space.constraints = vec![
            near("harbour_quarter"),
            rule(
                Rule::InDistrict {
                    target: "merchants".to_string(),
                },
                Weight::DEFAULT,
            ),
        ];
        let mut specification = with_market_and_bridge(space, bridge("ash"));
        add_district(&mut specification, "craft", vec![near("market")]);
        add_district(&mut specification, "harbour_quarter", Vec::new());
        add_district(
            &mut specification,
            "merchants",
            vec![near("harbour_quarter")],
        );
        assert_eq!(validate(&specification), Ok(()));
    }

    #[test]
    fn a_target_that_names_nothing_is_an_error() {
        let mut specification = valid();
        specification.districts[0].constraints = vec![near("nowhere")];
        assert_eq!(
            only_error(&specification),
            ValidationError::TargetMissing {
                field: "districts[0].constraints[0].target".to_string(),
                target: "nowhere".to_string(),
            }
        );
    }

    #[test]
    fn a_target_of_the_wrong_kind_is_an_error() {
        let mut landmark = guildhall(false);
        landmark.constraints = [
            Rule::InsideWalls {
                target: "market".to_string(),
            },
            Rule::OutsideWalls {
                target: CORE_WORD.to_string(),
            },
            Rule::NotInDistrict {
                target: "town_wall".to_string(),
            },
        ]
        .into_iter()
        .map(|target_rule| rule(target_rule, Weight::DEFAULT))
        .collect();
        let mut specification = with_defences(None, Vec::new());
        specification.open_space.push(market(true));
        specification.landmarks.push(landmark);
        let wrong = |index: usize, target: &str, expected| ValidationError::TargetOfWrongKind {
            field: format!("landmarks[0].constraints[{index}].target"),
            target: target.to_string(),
            expected,
        };
        assert_eq!(
            validate(&specification),
            Err(vec![
                wrong(0, "market", "wall"),
                wrong(1, "core", "wall"),
                wrong(2, "town_wall", "district"),
            ])
        );
    }

    #[test]
    fn near_and_far_from_point_to_anything_with_a_position() {
        let mut specification = with_gate_and_citadel();
        specification.site.water = with_water().site.water;
        let far_from = |target: &str| {
            let far_from = Rule::FarFrom {
                target: target.to_string(),
                beyond_metres: length(300.0),
            };
            rule(far_from, Weight::DEFAULT)
        };
        let mut landmark = guildhall(false);
        landmark.constraints = vec![
            near("ash"),
            near("mere"),
            near("quay"),
            near("cut"),
            near("west_road"),
            near(CORE_WORD),
            near("old_town"),
            far_from("town_wall"),
            far_from("west_gate"),
            far_from("keep"),
        ];
        specification.landmarks.push(landmark);
        assert_eq!(validate(&specification), Ok(()));
    }

    #[test]
    fn a_district_rule_cannot_point_to_a_wall_gate_or_citadel() {
        let mut specification = with_gate_and_citadel();
        specification.districts[0].constraints = vec![
            near("town_wall"),
            near("west_gate"),
            rule(
                Rule::InDistrict {
                    target: "keep".to_string(),
                },
                Weight::DEFAULT,
            ),
        ];
        let defence = |index: usize, target: &str| ValidationError::DistrictRuleToDefence {
            field: format!("districts[0].constraints[{index}].target"),
            target: target.to_string(),
        };
        assert_eq!(
            validate(&specification),
            Err(vec![
                defence(0, "town_wall"),
                defence(1, "west_gate"),
                defence(2, "keep"),
            ])
        );
    }

    fn ring(list: &'static str, objects: &[&str], arrows: &[&str]) -> ValidationError {
        ValidationError::DependencyRing {
            list,
            objects: identifiers(objects),
            arrows: identifiers(arrows),
        }
    }

    fn landmark_near(identifier: &str, targets: &[&str]) -> Landmark {
        let mut landmark = guildhall(false);
        landmark.identifier = identifier.to_string();
        landmark.constraints = targets.iter().map(|target| near(target)).collect();
        landmark
    }

    #[test]
    fn objects_that_wait_for_each_other_are_a_ring() {
        let mut space = market(false);
        space.constraints = vec![near("craft")];
        let mut specification = with_market_and_bridge(space, bridge("ash"));
        add_district(&mut specification, "craft", vec![near("market")]);
        let error = only_error(&specification);
        assert_eq!(
            error,
            ring(
                OPEN_SPACE,
                &["market", "craft", "market"],
                &[
                    "open_space[0].constraints[0] near craft",
                    "districts[1].constraints[0] near market"
                ]
            )
        );
        assert_eq!(
            error.to_string(),
            "open_space: market -> craft -> market wait for each other in a ring (open_space[0].constraints[0] near craft; districts[1].constraints[0] near market); remove a rule, or pin a landmark, open space or crossing in the ring"
        );
        specification.open_space[0].pinned = true;
        assert_eq!(validate(&specification), Ok(()));
    }

    #[test]
    fn a_rule_that_points_to_its_own_object_is_a_ring() {
        assert_eq!(
            only_error(&with_landmark(landmark_near("guildhall", &["guildhall"]))),
            ring(
                LANDMARKS,
                &["guildhall", "guildhall"],
                &["landmarks[0].constraints[0] near guildhall"]
            )
        );
    }

    #[test]
    fn a_ring_starts_at_its_first_landmark_and_names_the_wall_and_gate_fields() {
        let mut specification = with_gate_and_citadel();
        specification.districts[0].constraints = vec![near("guildhall")];
        specification
            .landmarks
            .push(landmark_near("guildhall", &["west_gate"]));
        assert_eq!(
            only_error(&specification),
            ring(
                LANDMARKS,
                &[
                    "guildhall",
                    "west_gate",
                    "town_wall",
                    "old_town",
                    "guildhall"
                ],
                &[
                    "landmarks[0].constraints[0] near west_gate",
                    "defences.gates[0].wall town_wall",
                    "defences.walls[0].encloses old_town",
                    "districts[0].constraints[0] near guildhall"
                ]
            )
        );
    }

    #[test]
    fn a_ring_through_the_citadel_names_its_wall_field() {
        let mut specification =
            with_defences(Some(citadel("town_wall", Bearing::Degrees(0))), Vec::new());
        specification.districts[0].constraints = vec![near("guildhall")];
        specification
            .landmarks
            .push(landmark_near("guildhall", &["keep"]));
        assert_eq!(
            only_error(&specification),
            ring(
                LANDMARKS,
                &["guildhall", "keep", "town_wall", "old_town", "guildhall"],
                &[
                    "landmarks[0].constraints[0] near keep",
                    "defences.citadel.wall town_wall",
                    "defences.walls[0].encloses old_town",
                    "districts[0].constraints[0] near guildhall"
                ]
            )
        );
    }

    #[test]
    fn a_landmark_ring_through_inside_walls_names_every_object() {
        let mut specification = with_defences(None, Vec::new());
        specification.districts[0].constraints = vec![near("guildhall")];
        let mut landmark = guildhall(false);
        landmark.constraints = vec![rule(
            Rule::InsideWalls {
                target: "town_wall".to_string(),
            },
            Weight::DEFAULT,
        )];
        specification.landmarks.push(landmark);
        assert_eq!(
            only_error(&specification),
            ring(
                LANDMARKS,
                &["guildhall", "town_wall", "old_town", "guildhall"],
                &[
                    "landmarks[0].constraints[0] inside_walls town_wall",
                    "defences.walls[0].encloses old_town",
                    "districts[0].constraints[0] near guildhall"
                ]
            )
        );
    }

    #[test]
    fn a_crossing_in_a_ring_starts_it_before_a_district() {
        let mut crossing = bridge("ash");
        crossing.constraints = vec![near("craft")];
        let mut specification = with_market_and_bridge(market(true), crossing);
        add_district(&mut specification, "craft", vec![near("ash_bridge")]);
        assert_eq!(
            only_error(&specification),
            ring(
                CROSSINGS,
                &["ash_bridge", "craft", "ash_bridge"],
                &[
                    "crossings[0].constraints[0] near craft",
                    "districts[1].constraints[0] near ash_bridge"
                ]
            )
        );
        specification.crossings[0].pinned = true;
        specification.crossings[0].resolved = Some(placement(0.0));
        assert_eq!(validate(&specification), Ok(()));
    }

    #[test]
    fn a_pinned_landmark_breaks_a_ring() {
        let mut specification = with_landmark(landmark_near("guildhall", &["craft"]));
        add_district(&mut specification, "craft", vec![near("guildhall")]);
        assert_eq!(
            only_error(&specification),
            ring(
                LANDMARKS,
                &["guildhall", "craft", "guildhall"],
                &[
                    "landmarks[0].constraints[0] near craft",
                    "districts[1].constraints[0] near guildhall"
                ]
            )
        );
        specification.landmarks[0].pinned = true;
        assert_eq!(validate(&specification), Ok(()));
    }

    #[test]
    fn a_ring_is_reported_once_from_its_first_object() {
        let mut specification = valid();
        for (identifier, target) in [("first", "second"), ("second", "third"), ("third", "first")] {
            specification
                .landmarks
                .push(landmark_near(identifier, &[target]));
        }
        assert_eq!(
            only_error(&specification),
            ring(
                LANDMARKS,
                &["first", "second", "third", "first"],
                &[
                    "landmarks[0].constraints[0] near second",
                    "landmarks[1].constraints[0] near third",
                    "landmarks[2].constraints[0] near first"
                ]
            )
        );
    }

    #[test]
    fn two_rings_in_one_file_are_both_reported() {
        let mut specification = valid();
        for (identifier, target) in [("a", "b"), ("b", "a"), ("c", "d"), ("d", "c")] {
            specification
                .landmarks
                .push(landmark_near(identifier, &[target]));
        }
        assert_eq!(
            validate(&specification),
            Err(vec![
                ring(
                    LANDMARKS,
                    &["a", "b", "a"],
                    &[
                        "landmarks[0].constraints[0] near b",
                        "landmarks[1].constraints[0] near a"
                    ]
                ),
                ring(
                    LANDMARKS,
                    &["c", "d", "c"],
                    &[
                        "landmarks[2].constraints[0] near d",
                        "landmarks[3].constraints[0] near c"
                    ]
                ),
            ])
        );
    }

    #[test]
    fn a_second_ring_through_an_object_of_a_reported_ring_is_absent() {
        let mut specification = valid();
        for (identifier, targets) in [("a", &["b", "c"][..]), ("b", &["a"]), ("c", &["a"])] {
            specification
                .landmarks
                .push(landmark_near(identifier, targets));
        }
        assert_eq!(
            only_error(&specification),
            ring(
                LANDMARKS,
                &["a", "b", "a"],
                &[
                    "landmarks[0].constraints[0] near b",
                    "landmarks[1].constraints[0] near a"
                ]
            )
        );
    }

    #[test]
    fn a_second_wall_ring_through_a_wall_of_a_reported_ring_is_absent() {
        let walls = vec![wall("a", &["b", "c"]), wall("b", &["a"]), wall("c", &["a"])];
        assert_eq!(
            only_error(&with_walls(walls)),
            ValidationError::WallRing {
                walls: identifiers(&["a", "b", "a"]),
            }
        );
    }

    #[test]
    fn a_duplicate_identifier_skips_the_dependency_checks() {
        let mut specification = valid();
        add_district(&mut specification, "x", vec![near("hall")]);
        specification.landmarks.push(landmark_near("hall", &["x"]));
        specification.landmarks.push(landmark_near("x", &[]));
        assert_eq!(
            only_error(&specification),
            ValidationError::DuplicateIdentifier {
                list: LANDMARKS,
                identifier: "x".to_string(),
            }
        );
    }

    #[test]
    fn a_target_of_the_wrong_kind_adds_no_arrow_to_a_ring() {
        let mut space = market(false);
        space.constraints = vec![near("guildhall")];
        let mut specification = with_market_and_bridge(space, bridge("ash"));
        let mut landmark = guildhall(false);
        landmark.constraints = vec![rule(
            Rule::InsideWalls {
                target: "market".to_string(),
            },
            Weight::DEFAULT,
        )];
        specification.landmarks.push(landmark);
        assert_eq!(
            only_error(&specification),
            ValidationError::TargetOfWrongKind {
                field: "landmarks[0].constraints[0].target".to_string(),
                target: "market".to_string(),
                expected: "wall",
            }
        );
    }

    #[test]
    fn open_space_and_crossing_targets_are_checked() {
        let broken_rules = || {
            vec![
                near("nowhere"),
                rule(
                    Rule::InDistrict {
                        target: "ash".to_string(),
                    },
                    Weight::DEFAULT,
                ),
            ]
        };
        let mut space = market(true);
        space.constraints = broken_rules();
        let mut crossing = bridge("ash");
        crossing.constraints = broken_rules();
        let missing = |owner: &str| ValidationError::TargetMissing {
            field: format!("{owner}.constraints[0].target"),
            target: "nowhere".to_string(),
        };
        let wrong = |owner: &str| ValidationError::TargetOfWrongKind {
            field: format!("{owner}.constraints[1].target"),
            target: "ash".to_string(),
            expected: "district",
        };
        assert_eq!(
            validate(&with_market_and_bridge(space, crossing)),
            Err(vec![
                missing("open_space[0]"),
                wrong("open_space[0]"),
                missing("crossings[0]"),
                wrong("crossings[0]"),
            ])
        );
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

        #[test]
        fn a_chain_of_districts_passes_and_closing_it_names_every_district(count in 1..20_usize) {
            let mut specification = valid();
            let name = |index: usize| format!("district_{index}");
            for index in 1..=count {
                add_district(&mut specification, &name(index), vec![near(&name(index - 1))]);
            }
            specification.districts[0].identifier = name(0);
            prop_assert_eq!(validate(&specification), Ok(()));
            specification.districts[0].constraints = vec![near(&name(count))];
            let mut ring: Vec<String> = (0..=count).rev().map(name).collect();
            ring.insert(0, name(0));
            let mut arrows = vec![format!("districts[0].constraints[0] near {}", name(count))];
            arrows.extend((1..=count).rev().map(|index| {
                format!("districts[{index}].constraints[0] near {}", name(index - 1))
            }));
            prop_assert_eq!(
                validate(&specification),
                Err(vec![ValidationError::DependencyRing { list: DISTRICTS, objects: ring, arrows }])
            );
        }
    }
}
