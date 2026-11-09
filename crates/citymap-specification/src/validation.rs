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

/// The populations the size law of ADR-021 covers; outside it the generator extrapolates and
/// warns.
pub const POPULATION_WITHOUT_WARNING: RangeInclusive<u32> = 1_000..=200_000;

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
    /// A pinned object's centre lies outside the site (ADR-071).
    #[error("{field}: the centre ({x}, {y}) lies outside the site (0 to {extent} on both axes)")]
    PinnedOutsideSite {
        /// The field path of the placement.
        field: String,
        /// The centre's x in the file, in metres.
        x: String,
        /// The centre's y in the file, in metres.
        y: String,
        /// The site's `extent_metres`.
        extent: String,
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
///
/// # Examples
///
/// ```
/// use citymap_specification::validation::{validate, ValidationError};
/// # use citymap_specification::specification::{
/// #     CompassRegion, District, DistrictFunction, Enclosure, Era, Identity, Importance,
/// #     Jurisdiction, LandCover, Presets, River, RiverCrossingKind, Road, RoadEnd, Seed,
/// #     Settlement, Site, Specification, Terrain, Water, Wealth,
/// # };
/// # use citymap_specification::units::{Length, Weight};
/// # let length = |metres| Length::new(metres).expect("a valid length");
/// # let mut specification = Specification {
/// #     schema: 1,
/// #     identity: Identity { name: "Harrowmere".to_string(), seed: Seed::new(1), notes: String::new() },
/// #     settlement: Settlement { population: 6000 },
/// #     site: Site {
/// #         extent_metres: length(2400.0),
/// #         terrain: Terrain::default(),
/// #         land_cover: LandCover::default(),
/// #         water: Water {
/// #             rivers: vec![River {
/// #                 identifier: "ash".to_string(),
/// #                 from: CompassRegion::West,
/// #                 to: CompassRegion::East,
/// #                 width_metres: length(18.0),
/// #             }],
/// #             ..Water::default()
/// #         },
/// #     },
/// #     roads: vec![Road {
/// #         identifier: "north_road".to_string(),
/// #         ends: [RoadEnd::Region(CompassRegion::North), RoadEnd::Region(CompassRegion::South)],
/// #         importance: Importance::Major,
/// #         river_crossing: RiverCrossingKind::Bridge,
/// #     }],
/// #     defences: Default::default(),
/// #     open_space: Vec::new(),
/// #     crossings: Vec::new(),
/// #     districts: vec![District {
/// #         identifier: "old_town".to_string(),
/// #         name: "Old Town".to_string(),
/// #         function: DistrictFunction::Residential,
/// #         wealth: Wealth::Middling,
/// #         group: None,
/// #         enclosure: Enclosure::Open,
/// #         jurisdiction: Jurisdiction::Town,
/// #         weight: Weight::DEFAULT,
/// #         constraints: Vec::new(),
/// #         notes: String::new(),
/// #     }],
/// #     landmarks: Vec::new(),
/// #     presets: Presets::new(Era::EarlyMedieval),
/// # };
/// assert_eq!(validate(&specification), Ok(()));
///
/// specification.settlement.population = 0;
/// let errors = validate(&specification).expect_err("a population of 0 breaks a rule");
/// assert!(matches!(errors[..], [ValidationError::PopulationOutOfRange { population: 0 }]));
/// ```
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
    let extent = specification.site.extent_metres;
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
        let resolved = landmark.resolved.as_ref();
        check_placement(resolved, landmark.pinned, extent, &owner, errors);
        check_constraints(&landmark.constraints, &owner, errors);
    }
}

fn check_open_space_and_crossings(
    specification: &Specification,
    errors: &mut Vec<ValidationError>,
) {
    let extent = specification.site.extent_metres;
    for (index, space) in specification.open_space.iter().enumerate() {
        let owner = format!("{OPEN_SPACE}[{index}]");
        let resolved = space.resolved.as_ref();
        check_placement(resolved, space.pinned, extent, &owner, errors);
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
        let resolved = crossing.resolved.as_ref();
        check_placement(resolved, crossing.pinned, extent, &owner, errors);
        check_constraints(&crossing.constraints, &owner, errors);
    }
}

/// Checks a `resolved` placement and its `pinned` flag; shared by every object that has them.
fn check_placement(
    resolved: Option<&Placement>,
    pinned: bool,
    extent: Length,
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
    if pinned && (placement.x > extent || placement.y > extent) {
        errors.push(ValidationError::PinnedOutsideSite {
            field: field.clone(),
            x: placement.x.get().to_string(),
            y: placement.y.get().to_string(),
            extent: extent.get().to_string(),
        });
    }
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
    /// The population is allowed but outside `POPULATION_WITHOUT_WARNING`, so the generator
    /// extrapolates.
    #[error(
        "settlement.population: {population} is outside {POPULATION_WITHOUT_WARNING:?}, the range the size law covers; the generator extrapolates"
    )]
    PopulationExtrapolated {
        /// The population as written.
        population: u32,
    },
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
///
/// # Examples
///
/// ```
/// use citymap_specification::validation::warnings;
/// # use citymap_specification::specification::{
/// #     CompassRegion, District, DistrictFunction, Enclosure, Era, Identity, Importance,
/// #     Jurisdiction, LandCover, Presets, River, RiverCrossingKind, Road, RoadEnd, Seed,
/// #     Settlement, Site, Specification, Terrain, Water, Wealth,
/// # };
/// # use citymap_specification::units::{Length, Weight};
/// # let length = |metres| Length::new(metres).expect("a valid length");
/// # let mut specification = Specification {
/// #     schema: 1,
/// #     identity: Identity { name: "Harrowmere".to_string(), seed: Seed::new(1), notes: String::new() },
/// #     settlement: Settlement { population: 6000 },
/// #     site: Site {
/// #         extent_metres: length(2400.0),
/// #         terrain: Terrain::default(),
/// #         land_cover: LandCover::default(),
/// #         water: Water {
/// #             rivers: vec![River {
/// #                 identifier: "ash".to_string(),
/// #                 from: CompassRegion::West,
/// #                 to: CompassRegion::East,
/// #                 width_metres: length(18.0),
/// #             }],
/// #             ..Water::default()
/// #         },
/// #     },
/// #     roads: vec![Road {
/// #         identifier: "north_road".to_string(),
/// #         ends: [RoadEnd::Region(CompassRegion::North), RoadEnd::Region(CompassRegion::South)],
/// #         importance: Importance::Major,
/// #         river_crossing: RiverCrossingKind::Bridge,
/// #     }],
/// #     defences: Default::default(),
/// #     open_space: Vec::new(),
/// #     crossings: Vec::new(),
/// #     districts: vec![District {
/// #         identifier: "old_town".to_string(),
/// #         name: "Old Town".to_string(),
/// #         function: DistrictFunction::Residential,
/// #         wealth: Wealth::Middling,
/// #         group: None,
/// #         enclosure: Enclosure::Open,
/// #         jurisdiction: Jurisdiction::Town,
/// #         weight: Weight::DEFAULT,
/// #         constraints: Vec::new(),
/// #         notes: String::new(),
/// #     }],
/// #     landmarks: Vec::new(),
/// #     presets: Presets::new(Era::EarlyMedieval),
/// # };
/// assert!(warnings(&specification).is_empty());
///
/// specification.presets.style = "inc".to_string();
/// let found = warnings(&specification);
/// assert_eq!(found.len(), 1);
/// assert!(found[0].to_string().starts_with("presets.style"), "{}", found[0]);
/// ```
pub fn warnings(specification: &Specification) -> Vec<ValidationWarning> {
    let population = specification.settlement.population;
    let extrapolated = (POPULATION.contains(&population)
        && !POPULATION_WITHOUT_WARNING.contains(&population))
    .then_some(ValidationWarning::PopulationExtrapolated { population });
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
    extrapolated
        .into_iter()
        .chain(labels)
        .chain(style)
        .chain(packs)
        .collect()
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
