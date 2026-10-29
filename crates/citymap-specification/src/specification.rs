//! The city specification as typed values (specification-version-1 §2). Only the shape is
//! checked here; ranges and cross-field rules belong to validation. Every struct rejects keys
//! it does not know, so a typo in a file is an error.

use std::borrow::Cow;

use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Value};

use crate::units::{Gradient, Length, Rotation, Weight};

/// One city specification, as read from a specification file.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Specification {
    /// The schema version of the file; version 1 is the only one that exists.
    pub schema: u32,
    /// The city's name, seed and notes.
    pub identity: Identity,
    /// The settlement's size.
    pub settlement: Settlement,
    /// The land the map shows, before the city: its size and its water.
    pub site: Site,
    /// The roads that reach or cross the site; at least one (validation, #97).
    pub roads: Vec<Road>,
    /// The city's walls, citadel and gates; none when the file has no `defences` key.
    #[serde(default)]
    pub defences: Defences,
    /// The city's market squares, plazas, greens and parade grounds.
    #[serde(default)]
    pub open_space: Vec<OpenSpace>,
    /// The bridges and fords placed by rules; a road over a river has one without a listing.
    #[serde(default)]
    pub crossings: Vec<Crossing>,
    /// The city's districts; 1 to `MAXIMUM_DISTRICTS` (validation).
    pub districts: Vec<District>,
    /// The city's landmarks; at most `MAXIMUM_LANDMARKS` (validation).
    #[serde(default)]
    pub landmarks: Vec<Landmark>,
    /// The generation presets.
    pub presets: Presets,
}

/// The city's name, its default seed and free notes for the GM.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Identity {
    /// The city's name, shown on the map.
    pub name: String,
    /// The master seed used when the command line gives none.
    pub seed: Seed,
    /// Free text the generator ignores; empty when the file has none.
    #[serde(default)]
    pub notes: String,
}

/// The master seed of one generation run, as given on the command line or in a specification
/// file, where it is written as a plain number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct Seed(u64);

impl Seed {
    /// Wraps a master seed.
    pub fn new(value: u64) -> Seed {
        Seed(value)
    }

    /// The master seed as a number.
    pub fn value(self) -> u64 {
        self.0
    }
}

/// The settlement's size: the only size input of the generator.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Settlement {
    /// The number of inhabitants.
    pub population: u32,
}

/// The site: the square piece of land the map shows, measured from its south-west corner.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Site {
    /// The side of the square site; the whole city fits inside it.
    pub extent_metres: Length,
    /// The shape of the land; none when the file has no `terrain` key.
    #[serde(default)]
    pub terrain: Terrain,
    /// The water on the site; none when the file has no `water` key.
    #[serde(default)]
    pub water: Water,
    /// What covers the ground around and inside the city; none when the file has no
    /// `land_cover` key.
    #[serde(default)]
    pub land_cover: LandCover,
}

/// What covers the ground (ADR-058). A name in `outside` or `inside` says the cover is
/// there and lets the generator place it; a band places an outside cover itself.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LandCover {
    /// Covers around the city that the generator places.
    #[serde(default)]
    pub outside: Vec<OutsideCover>,
    /// Covers inside the city that the generator places.
    #[serde(default)]
    pub inside: Vec<InsideCover>,
    /// Outside covers placed as a band reaching in from the site's edge.
    #[serde(default)]
    pub bands: Vec<Band>,
}

/// A cover of the ground around a city.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum OutsideCover {
    /// Fields.
    Farmland,
    /// Fruit trees in rows.
    Orchards,
    /// Open land of low shrubs.
    Heath,
    /// Bare rock.
    Rock,
    /// Wet, soft ground.
    Marsh,
    /// Woodland.
    Forest,
}

/// A cover of the ground inside a city.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum InsideCover {
    /// Private gardens.
    Gardens,
    /// Public parks.
    Parks,
    /// Small woods.
    Groves,
}

/// An outside cover placed in one direction, reaching in from the site's edge.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Band {
    /// The cover the band is made of.
    pub cover: OutsideCover,
    /// The direction from the site's centre toward the band.
    pub bearing_degrees: Bearing,
    /// How far the band reaches in from the site's edge.
    pub depth_metres: Length,
}

/// The shape of the land as named features (ADR-056). The words are the file's vocabulary,
/// not generation rules: S0 turns every feature into one ground.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Terrain {
    /// The sea's edge, if the site has one.
    #[serde(default)]
    pub coast: Option<Coast>,
    /// Sheer drops of the ground.
    #[serde(default)]
    pub cliffs: Vec<Cliff>,
    /// Rises of the ground around a point.
    #[serde(default)]
    pub hills: Vec<Hill>,
    /// Dips of the ground around a point, the mirror image of a hill.
    #[serde(default)]
    pub hollows: Vec<Hollow>,
    /// A tilt of the whole site, if it has one.
    #[serde(default)]
    pub slope: Option<Slope>,
    /// A raised flat part of the site, if it has one.
    #[serde(default)]
    pub plateau: Option<Plateau>,
}

/// The sea's edge: where the sea lies, the coastline's shape and what the shore is made of.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Coast {
    /// The direction from the site's centre toward the sea.
    pub bearing_degrees: Bearing,
    /// The shape of the coastline.
    pub shape: CoastShape,
    /// What the shore is made of; the generator picks when the file has none.
    #[serde(default)]
    pub shore: Option<Shore>,
}

/// The shape of a coastline.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CoastShape {
    /// The sea reaches into the land in a curve.
    Bay,
    /// A straight coast along open sea.
    Open,
    /// A narrow sea between this land and other land.
    Strait,
    /// A river widens into the sea; it needs a river that reaches the sea there.
    Estuary,
}

/// What a shore is made of.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Shore {
    /// Sand or shingle.
    Beach,
    /// Bare rock.
    Rock,
    /// Marsh that the tide floods.
    SaltMarsh,
}

/// A sheer drop of the ground on one side of the site.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Cliff {
    /// The direction from the site's centre toward the cliff.
    pub bearing_degrees: Bearing,
    /// How far the ground drops.
    pub height_metres: Length,
}

/// A rise of the ground around a point.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Hill {
    /// Where the hill stands; `centre` is the site's centre.
    pub at: CompassRegion,
    /// How far the hill reaches from its top.
    pub radius_metres: Length,
    /// How high the top stands above the ground around it.
    pub height_metres: Length,
}

/// A dip of the ground around a point.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Hollow {
    /// Where the hollow lies; `centre` is the site's centre.
    pub at: CompassRegion,
    /// How far the hollow reaches from its lowest point.
    pub radius_metres: Length,
    /// How deep the lowest point lies below the ground around it.
    pub depth_metres: Length,
}

/// A tilt of the whole site.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Slope {
    /// The direction the ground falls toward.
    pub toward_degrees: Bearing,
    /// How far the ground falls per kilometre.
    pub metres_per_kilometre: Gradient,
}

/// A raised flat part on one side of the site.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Plateau {
    /// The direction from the site's centre toward the plateau.
    pub bearing_degrees: Bearing,
    /// How high the plateau stands above the rest of the site.
    pub height_metres: Length,
}

/// A direction from the site's centre: whole degrees clockwise from north (`115`) or a
/// compass word (`south_east`, 135 degrees). It keeps the form the file used, so writing the
/// file back never changes it (ADR-056).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bearing {
    /// Whole degrees; validation rejects anything above `MAXIMUM_DEGREES`.
    Degrees(u16),
    /// A compass word; validation rejects `centre`, which has no direction.
    Word(CompassRegion),
}

impl Bearing {
    /// The largest bearing in degrees.
    pub const MAXIMUM_DEGREES: u16 = 359;

    /// What a file must write for a bearing; the reading error when it writes something else.
    pub const EXPECTED: &str = "a bearing: whole degrees from 0 to 359 in plain digits, or a compass word such as `south_east`";

    /// Reads a bearing as a file writes it: plain digits without a leading zero, or a compass
    /// word. `None` for anything else, such as `0x10`, `+5`, `007` or `12.5`, which could not
    /// be written back in the same form.
    ///
    /// # Examples
    ///
    /// ```
    /// use citymap_specification::specification::{Bearing, CompassRegion};
    ///
    /// assert_eq!(Bearing::from_text("115"), Some(Bearing::Degrees(115)));
    /// assert_eq!(Bearing::from_text("south_east"), Some(Bearing::Word(CompassRegion::SouthEast)));
    /// assert_eq!(Bearing::from_text("007"), None);
    /// ```
    pub fn from_text(text: &str) -> Option<Bearing> {
        let plain_digits = !text.is_empty()
            && text.bytes().all(|byte| byte.is_ascii_digit())
            && (text == "0" || !text.starts_with('0'));
        if plain_digits {
            return text.parse().ok().map(Bearing::Degrees);
        }
        CompassRegion::ALL
            .into_iter()
            .find(|region| region.name() == text)
            .map(Bearing::Word)
    }

    /// The bearing in degrees; `None` for `centre` or a number above `MAXIMUM_DEGREES`.
    pub const fn degrees(self) -> Option<u16> {
        match self {
            Bearing::Degrees(degrees) if degrees <= Bearing::MAXIMUM_DEGREES => Some(degrees),
            Bearing::Degrees(_) => None,
            Bearing::Word(region) => region.bearing(),
        }
    }
}

/// The water on the site; the generator reads only rivers until later milestones.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Water {
    /// The rivers that cross the site, from one edge stretch to another.
    #[serde(default)]
    pub rivers: Vec<River>,
    /// The lakes on the site.
    #[serde(default)]
    pub lakes: Vec<Lake>,
    /// The harbour, if the site has one.
    #[serde(default)]
    pub harbour: Option<Harbour>,
    /// Whether the sea rises and falls with the tide; only drawn, false when the file says
    /// nothing.
    #[serde(default)]
    pub tidal: bool,
    /// Dug waterways from one edge stretch to another; read now, generated from milestone 5.
    #[serde(default)]
    pub canals: Vec<Canal>,
}

/// A lake around a point of the site.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Lake {
    /// The lake's identifier in this file, unique across the whole file.
    pub identifier: String,
    /// Where the lake lies; `centre` is the site's centre.
    pub at: CompassRegion,
    /// How far the lake reaches from its middle.
    pub radius_metres: Length,
}

/// A harbour on the site's coast or river.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Harbour {
    /// The harbour's identifier in this file, unique across the whole file.
    pub identifier: String,
    /// The direction from the site's centre toward the harbour; a coast or a river end lies
    /// within `HARBOUR_WATER_DEGREES` of it (validation).
    pub bearing_degrees: Bearing,
    /// How long the harbour's waterfront is.
    pub length_metres: Length,
}

/// A dug waterway that enters the site at one edge stretch and leaves at another.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Canal {
    /// The canal's identifier in this file, unique across the whole file.
    pub identifier: String,
    /// The edge stretch where the canal enters.
    pub from: CompassRegion,
    /// The edge stretch where the canal leaves.
    pub to: CompassRegion,
    /// The canal's width.
    pub width_metres: Length,
}

/// A river that enters the site at one edge stretch and leaves at another.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct River {
    /// The river's identifier in this file, unique across the whole file.
    pub identifier: String,
    /// The edge stretch where the river enters.
    pub from: CompassRegion,
    /// The edge stretch where the river leaves.
    pub to: CompassRegion,
    /// The river's width.
    pub width_metres: Length,
}

/// One of the eight compass directions or `centre`; as a river or road end, a stretch of the
/// site's edge (`centre` is not an edge and is rejected there by validation).
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum CompassRegion {
    /// North.
    North,
    /// North-east.
    NorthEast,
    /// East.
    East,
    /// South-east.
    SouthEast,
    /// South.
    South,
    /// South-west.
    SouthWest,
    /// West.
    West,
    /// North-west.
    NorthWest,
    /// The middle of the site, not an edge.
    Centre,
}

impl CompassRegion {
    /// Every compass region, in the order of the enum.
    pub const ALL: [CompassRegion; 9] = [
        CompassRegion::North,
        CompassRegion::NorthEast,
        CompassRegion::East,
        CompassRegion::SouthEast,
        CompassRegion::South,
        CompassRegion::SouthWest,
        CompassRegion::West,
        CompassRegion::NorthWest,
        CompassRegion::Centre,
    ];

    /// The name a specification file uses, such as `north_east`.
    pub const fn name(self) -> &'static str {
        match self {
            CompassRegion::North => "north",
            CompassRegion::NorthEast => "north_east",
            CompassRegion::East => "east",
            CompassRegion::SouthEast => "south_east",
            CompassRegion::South => "south",
            CompassRegion::SouthWest => "south_west",
            CompassRegion::West => "west",
            CompassRegion::NorthWest => "north_west",
            CompassRegion::Centre => "centre",
        }
    }

    /// The direction in degrees clockwise from north; `None` for `centre`.
    pub const fn bearing(self) -> Option<u16> {
        match self {
            CompassRegion::North => Some(0),
            CompassRegion::NorthEast => Some(45),
            CompassRegion::East => Some(90),
            CompassRegion::SouthEast => Some(135),
            CompassRegion::South => Some(180),
            CompassRegion::SouthWest => Some(225),
            CompassRegion::West => Some(270),
            CompassRegion::NorthWest => Some(315),
            CompassRegion::Centre => None,
        }
    }
}

impl Serialize for Bearing {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Bearing::Degrees(degrees) => serializer.serialize_u16(*degrees),
            Bearing::Word(region) => serializer.serialize_str(region.name()),
        }
    }
}

impl JsonSchema for Bearing {
    fn schema_name() -> Cow<'static, str> {
        "Bearing".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let word = generator.subschema_for::<CompassRegion>();
        json_schema!({
            "description": Bearing::EXPECTED,
            "anyOf": [
                { "type": "integer", "minimum": 0, "maximum": Bearing::MAXIMUM_DEGREES },
                word,
            ],
        })
    }
}

impl<'de> Deserialize<'de> for Bearing {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Bearing, D::Error> {
        let text = String::deserialize(deserializer)?;
        Bearing::from_text(&text).ok_or_else(|| D::Error::custom(Bearing::EXPECTED))
    }
}

/// A district of the city (ADR-060): what it is for, how rich it is, and the rules that place
/// it. The generator assigns wards to it; `weight` is its size relative to the other districts.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct District {
    /// The district's identifier in this file, unique across the whole file.
    pub identifier: String,
    /// The name the legend shows.
    pub name: String,
    /// What the district is for.
    pub function: DistrictFunction,
    /// How rich the district is; middling when the file says nothing.
    #[serde(default)]
    pub wealth: Wealth,
    /// A free word rule packs may read, such as `jotun`; none when the file has none.
    #[serde(default)]
    pub group: Option<String>,
    /// Whether the district has its own boundary, which is only drawn; open when the file
    /// says nothing.
    #[serde(default)]
    pub enclosure: Enclosure,
    /// Who governs the district; the town when the file says nothing.
    #[serde(default)]
    pub jurisdiction: Jurisdiction,
    /// The district's size relative to the other districts.
    pub weight: Weight,
    /// The rules that place the district.
    #[serde(default)]
    pub constraints: Vec<Constraint>,
    /// Free text the generator ignores; empty when the file has none.
    #[serde(default)]
    pub notes: String,
}

/// What a district is for. A culture's own quarters map onto one of these in a rule pack.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DistrictFunction {
    /// Homes.
    Residential,
    /// Workshops.
    Craft,
    /// Noxious or water-bound industry, such as tanneries and dye works.
    Works,
    /// Trade and markets.
    Commerce,
    /// Quays and warehouses.
    Harbour,
    /// Where the city is ruled from.
    SeatOfPower,
    /// Barracks and arsenals.
    Military,
    /// A walled or set-apart religious ground.
    ReligiousPrecinct,
    /// Fields inside the city.
    Farmland,
    /// Graves.
    Cemetery,
    /// Open ground nobody builds on.
    OpenGround,
}

/// How rich a district is.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Wealth {
    /// Poor.
    Poor,
    /// Neither poor nor rich, the default.
    #[default]
    Middling,
    /// Rich.
    Rich,
}

/// Whether a district has its own boundary.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Enclosure {
    /// No boundary of its own, the default.
    #[default]
    Open,
    /// A boundary with gates.
    Gated,
    /// A wall of its own.
    Walled,
}

/// Who governs a district.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Jurisdiction {
    /// The town, the default.
    #[default]
    Town,
    /// Someone else, such as a castle or a church.
    Separate,
}

/// A landmark of the city (ADR-061): a building or monument the map numbers and the legend
/// names. Without `pinned` the generator places it by its rules.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Landmark {
    /// The landmark's identifier in this file, unique across the whole file.
    pub identifier: String,
    /// The name the legend shows.
    pub name: String,
    /// What the landmark is for.
    pub function: LandmarkFunction,
    /// A free word the legend shows and rule packs match, such as `cathedral`.
    #[serde(default)]
    pub label: Option<String>,
    /// How important the landmark is; minor when the file says nothing.
    #[serde(default)]
    pub importance: Importance,
    /// The size the file asks for; the generator picks it when the file has none.
    #[serde(default)]
    pub footprint: Option<Footprint>,
    /// The rules that place the landmark.
    #[serde(default)]
    pub constraints: Vec<Constraint>,
    /// Where the landmark stands, written by the generator or by hand.
    #[serde(default)]
    pub resolved: Option<Placement>,
    /// Whether `resolved` is kept exactly; false when the file says nothing.
    #[serde(default)]
    pub pinned: bool,
    /// Free text the generator ignores; empty when the file has none.
    #[serde(default)]
    pub notes: String,
}

/// How far apart an inner wall with `separate_ring` stands from the outer wall when the file
/// gives no `gap_metres`; Claude's estimate until research #207 gives a source (ADR-063).
pub const SEPARATE_RING_GAP_METRES: f64 = 20.0;

/// The city's defences (ADR-063).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Defences {
    /// The walls; what each encloses comes only from its `encloses` list.
    #[serde(default)]
    pub walls: Vec<Wall>,
    /// The city's own stronghold, if it has one (ADR-064).
    #[serde(default)]
    pub citadel: Option<Citadel>,
    /// The gates that rules can point to, and the gates without a road (ADR-064).
    #[serde(default)]
    pub gates: Vec<Gate>,
}

/// A wall: the outline of the area its enclosed districts cover. Two walls whose areas touch
/// share that stretch, unless the inner one keeps a `separate_ring`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Wall {
    /// The wall's identifier in this file, unique across the whole file.
    pub identifier: String,
    /// What the wall is built of.
    pub kind: WallKind,
    /// The districts and walls inside it; a wall inside it brings that wall's districts too.
    pub encloses: Vec<String>,
    /// What stands where the enclosed area meets coast, river or lake; a wall by default.
    #[serde(default)]
    pub along_water: AlongWater,
    /// Whether this wall keeps a gap from a wall outside it instead of sharing its line.
    #[serde(default)]
    pub separate_ring: bool,
    /// The gap of a separate ring; `SEPARATE_RING_GAP_METRES` when the file gives none.
    #[serde(default)]
    pub gap_metres: Option<Length>,
}

/// What a wall is built of.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WallKind {
    /// A timber palisade.
    Palisade,
    /// A stone wall.
    Stone,
    /// The line of a torn-down wall.
    Razed,
}

/// What stands where a wall's area meets coast, river or lake.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AlongWater {
    /// A sea or river wall, the default.
    #[default]
    Wall,
    /// The water alone.
    Open,
}

/// The city's own stronghold, part of a wall on one side of that wall's area: its outer side is
/// the wall. A ward around a castle is a pinned `defence` landmark instead (ADR-064).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Citadel {
    /// The citadel's identifier in this file, unique across the whole file.
    pub identifier: String,
    /// The side of the wall's area it stands on, from the centre of that area.
    pub bearing_degrees: Bearing,
    /// The wall it is part of.
    pub wall: String,
}

/// A named gate in a wall. A gate stands wherever a road crosses a wall, listed or not; a
/// listed one gets a name rules can point to, or is a gate without a road (ADR-064).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "GateAsWritten")]
pub struct Gate {
    /// Which gate this is, with the kind's own fields.
    #[serde(flatten)]
    pub kind: GateKind,
    /// The gate's identifier in this file, unique across the whole file.
    pub identifier: String,
    /// The wall the gate is in.
    pub wall: String,
}

/// The kind of a `Gate`, with its own fields.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum GateKind {
    /// The gate where a road crosses the wall.
    RoadGate {
        /// The road that passes through.
        road: String,
        /// One of the road's `ends`; of the points where the road crosses the wall, the one
        /// closest to that end. Without it the road must cross the wall exactly once.
        #[serde(skip_serializing_if = "Option::is_none")]
        end: Option<RoadEnd>,
    },
    /// A small gate without a road.
    Postern {
        /// The side of the wall's area it is on, from the centre of that area.
        bearing_degrees: Bearing,
    },
    /// A gate onto the water, without a road.
    WaterGate {
        /// The side of the wall's area it is on, from the centre of that area.
        bearing_degrees: Bearing,
    },
}

/// The name of a gate kind, as the `kind` key of a file writes it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum GateKindName {
    RoadGate,
    Postern,
    WaterGate,
}

impl GateKindName {
    const ALL: [GateKindName; 3] = [
        GateKindName::RoadGate,
        GateKindName::Postern,
        GateKindName::WaterGate,
    ];

    const fn name(self) -> &'static str {
        match self {
            GateKindName::RoadGate => "road_gate",
            GateKindName::Postern => "postern",
            GateKindName::WaterGate => "water_gate",
        }
    }

    const fn fields(self) -> &'static [&'static str] {
        match self {
            GateKindName::RoadGate => &["road", "end"],
            GateKindName::Postern | GateKindName::WaterGate => &["bearing_degrees"],
        }
    }

    const fn required(self) -> &'static [&'static str] {
        match self {
            GateKindName::RoadGate => &["road"],
            GateKindName::Postern | GateKindName::WaterGate => &["bearing_degrees"],
        }
    }
}

impl JsonSchema for Gate {
    fn schema_name() -> Cow<'static, str> {
        "Gate".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let kinds = GateKindName::ALL.map(|kind| (kind.name(), kind.fields(), kind.required()));
        let written = GateAsWritten::json_schema(generator);
        one_entry_per_kind(&written, "kind", &kinds, &["identifier", "wall"])
    }
}

/// A gate as a file writes it, every kind's fields optional and read one by one, so `Bearing`
/// sees a `bearing_degrees` as written (ADR-059, ADR-064).
#[derive(Clone, Debug, PartialEq, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct GateAsWritten {
    /// Which kind of gate this is; `one_entry_per_kind` writes its schema.
    #[schemars(skip)]
    kind: GateKindName,
    /// The gate's identifier.
    identifier: String,
    /// The wall the gate is in.
    wall: String,
    /// `road_gate`: the road that passes through.
    road: Option<String>,
    /// `road_gate`: which of the road's ends picks the crossing.
    end: Option<RoadEnd>,
    /// `postern` and `water_gate`: the side of the wall's area.
    bearing_degrees: Option<Bearing>,
}

impl TryFrom<GateAsWritten> for Gate {
    type Error = FieldError;

    fn try_from(written: GateAsWritten) -> Result<Gate, FieldError> {
        let name = written.kind;
        let given = [
            ("road", written.road.is_some()),
            ("end", written.end.is_some()),
            ("bearing_degrees", written.bearing_degrees.is_some()),
        ];
        field_not_taken(name.name(), &given, name.fields(), &["identifier", "wall"])?;
        let needs = |field: &'static str| FieldError::Missing {
            kind: name.name(),
            field,
        };
        let bearing_degrees = || {
            written
                .bearing_degrees
                .ok_or_else(|| needs("bearing_degrees"))
        };
        let kind = match name {
            GateKindName::RoadGate => GateKind::RoadGate {
                road: written.road.ok_or_else(|| needs("road"))?,
                end: written.end,
            },
            GateKindName::Postern => GateKind::Postern {
                bearing_degrees: bearing_degrees()?,
            },
            GateKindName::WaterGate => GateKind::WaterGate {
                bearing_degrees: bearing_degrees()?,
            },
        };
        Ok(Gate {
            kind,
            identifier: written.identifier,
            wall: written.wall,
        })
    }
}

/// A square, green or parade ground (ADR-062). Without `pinned` the generator places it by
/// its rules and draws its size from its kind's range in the rule pack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct OpenSpace {
    /// The open space's identifier in this file, unique across the whole file.
    pub identifier: String,
    /// What kind of open space it is.
    pub kind: OpenSpaceKind,
    /// The rules that place the open space.
    #[serde(default)]
    pub constraints: Vec<Constraint>,
    /// Where the open space lies, written by the generator or by hand.
    #[serde(default)]
    pub resolved: Option<Placement>,
    /// Whether `resolved` is kept exactly; false when the file says nothing.
    #[serde(default)]
    pub pinned: bool,
}

/// What kind of open space it is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OpenSpaceKind {
    /// A square for a market.
    MarketSquare,
    /// A paved square.
    Plaza,
    /// A grassed common.
    Green,
    /// Ground for drilling troops.
    ParadeGround,
}

/// A bridge or ford placed by rules (ADR-062); it fixes how streets cross the river.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Crossing {
    /// The crossing's identifier in this file, unique across the whole file.
    pub identifier: String,
    /// The identifier of the river it crosses.
    pub river: String,
    /// Whether it is a bridge or a ford.
    pub kind: RiverCrossingKind,
    /// The rules that place the crossing.
    #[serde(default)]
    pub constraints: Vec<Constraint>,
    /// Where the crossing lies, written by the generator or by hand.
    #[serde(default)]
    pub resolved: Option<Placement>,
    /// Whether `resolved` is kept exactly; false when the file says nothing.
    #[serde(default)]
    pub pinned: bool,
}

/// What a landmark is for; each has a default footprint class and shape in the rule pack.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LandmarkFunction {
    /// A church, temple, mosque or shrine.
    Religious,
    /// A castle, keep, tower or gatehouse.
    Defence,
    /// A palace or government house.
    SeatOfPower,
    /// A town hall, guildhall or courthouse.
    Civic,
    /// A market hall, warehouse or exchange.
    Commerce,
    /// An inn or caravanserai.
    Lodging,
    /// An academy, madrasa or college.
    Learning,
    /// A theatre or arena.
    Entertainment,
    /// A hospital, bathhouse or leper house.
    Health,
    /// A well, fountain, mill or lighthouse.
    Utility,
    /// An arsenal or barracks.
    Military,
    /// A column, market cross or statue.
    Monument,
    /// A smithy, tannery, brewery or other workshop.
    Production,
}

/// The size a file asks for a landmark; each part is optional and overrides the defaults
/// of its function and label (specification-version-1 §3).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Footprint {
    /// The size class whose area range the generator draws from.
    #[serde(default)]
    pub class: Option<FootprintClass>,
    /// The width.
    #[serde(default)]
    pub width_metres: Option<Length>,
    /// The depth, the long side of the shape when the generator draws the size.
    #[serde(default)]
    pub depth_metres: Option<Length>,
}

/// A landmark's size class, from a 2 m well to a 100 m palace (specification-version-1 §3).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FootprintClass {
    /// About 2 m a side.
    Micro,
    /// About 6 m a side.
    Tiny,
    /// About 12 m a side.
    Small,
    /// About 25 m a side.
    Medium,
    /// About 50 m a side.
    Large,
    /// About 100 m a side.
    Huge,
}

/// Where a landmark, open space or crossing stands: its centre in the site frame, measured
/// from the south-west corner, its turn and its size, so a pin keeps its size too.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Placement {
    /// The centre's distance east of the site's west edge.
    pub x: Length,
    /// The centre's distance north of the site's south edge.
    pub y: Length,
    /// The turn, clockwise from north, from 0 up to 360 (validation).
    pub rotation_degrees: Rotation,
    /// The width.
    pub width_metres: Length,
    /// The depth.
    pub depth_metres: Length,
}

/// How far `near` reaches when the file gives no `within_metres`.
pub const NEAR_WITHIN_METRES: f64 = 150.0;

/// How far `far_from` keeps away when the file gives no `beyond_metres`.
pub const FAR_FROM_BEYOND_METRES: f64 = 300.0;

/// How wide, in degrees, a compass region's slice is when the file gives no `width_degrees`;
/// a compass word's slice has the same width.
pub const COMPASS_REGION_WIDTH_DEGREES: u16 = 45;

/// One rule about where an object goes (REQUIREMENTS.md F3, ADR-059). It is soft unless
/// `hard` is true; a soft rule's `weight` says how much it counts. A file writes it flat:
/// `{ rule: near, target: market, weight: 2 }`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "ConstraintAsWritten")]
pub struct Constraint {
    /// Which rule this is, with the rule's own fields.
    #[serde(flatten)]
    pub rule: Rule,
    /// Whether the rule must hold; false when the file says nothing.
    pub hard: bool,
    /// How much the rule counts when it is soft; `Weight::DEFAULT` when the file says nothing.
    pub weight: Weight,
}

/// The rule of a `Constraint`, with its own fields. The name is the file's word, not a
/// generation step: each word has one compiled evaluator (ADR-017).
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "rule", rename_all = "snake_case")]
pub enum Rule {
    /// The object lies in a slice around the town core, or in its centre.
    CompassRegion {
        /// The slice's direction from the core, or `centre` for the circle around the core.
        region: Bearing,
        /// The slice's whole width; `COMPASS_REGION_WIDTH_DEGREES` when the file gives none.
        #[serde(skip_serializing_if = "Option::is_none")]
        width_degrees: Option<u16>,
    },
    /// The object lies outside a slice around the town core, or outside its centre.
    NotInCompassRegion {
        /// The slice's direction from the core, or `centre` for the circle around the core.
        region: Bearing,
        /// The slice's whole width; `COMPASS_REGION_WIDTH_DEGREES` when the file gives none.
        #[serde(skip_serializing_if = "Option::is_none")]
        width_degrees: Option<u16>,
    },
    /// Some part of the object lies within a distance of the target.
    Near {
        /// The specification identifier of something with a position, or `core`.
        target: String,
        /// The distance, edge to edge; `NEAR_WITHIN_METRES` when the file gives none.
        within_metres: Length,
    },
    /// Every part of the object lies beyond a distance from the target.
    FarFrom {
        /// The specification identifier of something with a position, or `core`.
        target: String,
        /// The distance, edge to edge; `FAR_FROM_BEYOND_METRES` when the file gives none.
        beyond_metres: Length,
    },
    /// The object touches the coast, a river, a lake or the harbour.
    OnWater,
    /// The object touches no coast, river, lake or harbour.
    AwayFromWater,
    /// The object stands in the highest quarter of the site's ground.
    OnHighGround,
    /// The object stands in the lowest quarter of the site's ground.
    OnLowGround,
    /// The object lies inside the area a wall encloses.
    InsideWalls {
        /// The wall's identifier.
        target: String,
    },
    /// The object lies outside the area a wall encloses.
    OutsideWalls {
        /// The wall's identifier.
        target: String,
    },
    /// The object lies inside a district.
    InDistrict {
        /// The district's identifier.
        target: String,
    },
    /// The object lies outside a district.
    NotInDistrict {
        /// The district's identifier.
        target: String,
    },
    /// The object keeps at least a gap from every landmark, open space and crossing placed
    /// before it.
    MinSpacing {
        /// The gap, edge to edge.
        metres: Length,
    },
}

/// The name of a rule, as the `rule` key of a file writes it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RuleName {
    CompassRegion,
    NotInCompassRegion,
    Near,
    FarFrom,
    OnWater,
    AwayFromWater,
    OnHighGround,
    OnLowGround,
    InsideWalls,
    OutsideWalls,
    InDistrict,
    NotInDistrict,
    MinSpacing,
}

impl RuleName {
    const ALL: [RuleName; 13] = [
        RuleName::CompassRegion,
        RuleName::NotInCompassRegion,
        RuleName::Near,
        RuleName::FarFrom,
        RuleName::OnWater,
        RuleName::AwayFromWater,
        RuleName::OnHighGround,
        RuleName::OnLowGround,
        RuleName::InsideWalls,
        RuleName::OutsideWalls,
        RuleName::InDistrict,
        RuleName::NotInDistrict,
        RuleName::MinSpacing,
    ];

    const fn name(self) -> &'static str {
        match self {
            RuleName::CompassRegion => "compass_region",
            RuleName::NotInCompassRegion => "not_in_compass_region",
            RuleName::Near => "near",
            RuleName::FarFrom => "far_from",
            RuleName::OnWater => "on_water",
            RuleName::AwayFromWater => "away_from_water",
            RuleName::OnHighGround => "on_high_ground",
            RuleName::OnLowGround => "on_low_ground",
            RuleName::InsideWalls => "inside_walls",
            RuleName::OutsideWalls => "outside_walls",
            RuleName::InDistrict => "in_district",
            RuleName::NotInDistrict => "not_in_district",
            RuleName::MinSpacing => "min_spacing",
        }
    }

    const fn fields(self) -> &'static [&'static str] {
        match self {
            RuleName::CompassRegion | RuleName::NotInCompassRegion => &["region", "width_degrees"],
            RuleName::Near => &["target", "within_metres"],
            RuleName::FarFrom => &["target", "beyond_metres"],
            RuleName::OnWater
            | RuleName::AwayFromWater
            | RuleName::OnHighGround
            | RuleName::OnLowGround => &[],
            RuleName::InsideWalls
            | RuleName::OutsideWalls
            | RuleName::InDistrict
            | RuleName::NotInDistrict => &["target"],
            RuleName::MinSpacing => &["metres"],
        }
    }

    const fn required(self) -> &'static [&'static str] {
        match self {
            RuleName::CompassRegion | RuleName::NotInCompassRegion => &["region"],
            RuleName::Near
            | RuleName::FarFrom
            | RuleName::InsideWalls
            | RuleName::OutsideWalls
            | RuleName::InDistrict
            | RuleName::NotInDistrict => &["target"],
            RuleName::OnWater
            | RuleName::AwayFromWater
            | RuleName::OnHighGround
            | RuleName::OnLowGround => &[],
            RuleName::MinSpacing => &["metres"],
        }
    }
}

impl JsonSchema for Constraint {
    fn schema_name() -> Cow<'static, str> {
        "Constraint".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let kinds = RuleName::ALL.map(|rule| (rule.name(), rule.fields(), rule.required()));
        let written = ConstraintAsWritten::json_schema(generator);
        one_entry_per_kind(&written, "rule", &kinds, &["hard", "weight"])
    }
}

/// A `oneOf` with one entry per kind of a flat rule or gate (ADR-070). Each entry allows `tag`
/// with the kind's word, the kind's fields and the `shared` ones; it requires `tag`, the kind's
/// required fields and the `shared` fields `written` requires.
fn one_entry_per_kind(
    written: &Schema,
    tag: &str,
    kinds: &[(&str, &[&str], &[&str])],
    shared: &[&str],
) -> Schema {
    let empty = Map::new();
    let properties = written
        .get("properties")
        .and_then(Value::as_object)
        .unwrap_or(&empty);
    let written_required: Vec<&str> = written
        .get("required")
        .and_then(Value::as_array)
        .map(|names| names.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    let entries: Vec<Value> = kinds
        .iter()
        .map(|(name, fields, required)| {
            let mut allowed = Map::new();
            allowed.insert(tag.to_string(), json_schema!({ "const": name }).into());
            for field in fields.iter().chain(shared) {
                if let Some(schema) = properties.get(*field) {
                    allowed.insert((*field).to_string(), schema.clone());
                }
            }
            let needed: Vec<&str> = std::iter::once(tag)
                .chain(required.iter().copied())
                .chain(
                    shared
                        .iter()
                        .copied()
                        .filter(|field| written_required.contains(field)),
                )
                .collect();
            json_schema!({
                "type": "object",
                "properties": allowed,
                "required": needed,
                "additionalProperties": false,
            })
            .into()
        })
        .collect();
    json_schema!({ "oneOf": entries })
}

/// A rule as a file writes it, every field optional and read one by one, so `Bearing` sees a
/// `region` as written; a tagged enum would first turn `0x10` into 16 (ADR-059).
#[derive(Clone, Debug, PartialEq, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct ConstraintAsWritten {
    /// Which rule this is; `one_entry_per_kind` writes its schema.
    #[schemars(skip)]
    rule: RuleName,
    /// `compass_region` and `not_in_compass_region`: the slice's direction, or `centre`.
    region: Option<Bearing>,
    /// `compass_region` and `not_in_compass_region`: the slice's whole width.
    width_degrees: Option<u16>,
    /// `near`, `far_from` and the wall and district rules: what the rule points to.
    target: Option<String>,
    /// `near`: the distance.
    within_metres: Option<Length>,
    /// `far_from`: the distance.
    beyond_metres: Option<Length>,
    /// `min_spacing`: the gap.
    metres: Option<Length>,
    /// Whether the rule must hold; false when the file says nothing.
    hard: Option<bool>,
    /// How much a soft rule counts; `Weight::DEFAULT` when the file says nothing.
    weight: Option<Weight>,
}

/// Why a rule or a gate as written cannot become its type.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum FieldError {
    /// The rule or gate was given a field its kind does not take.
    #[error("`{field}` is not a field of `{kind}`; it takes {expected}")]
    NotAField {
        /// The rule's or gate's kind.
        kind: &'static str,
        /// The field it does not take.
        field: &'static str,
        /// The fields it takes, the shared ones included, as one list.
        expected: String,
    },
    /// The rule or gate lacks a field its kind needs.
    #[error("`{kind}` needs `{field}`")]
    Missing {
        /// The rule's or gate's kind.
        kind: &'static str,
        /// The field it needs.
        field: &'static str,
    },
}

/// The first field in `given` that `fields` does not list, as a `NotAField` error naming the
/// fields the kind takes, `shared` ones included.
fn field_not_taken(
    kind: &'static str,
    given: &[(&'static str, bool)],
    fields: &[&'static str],
    shared: &[&'static str],
) -> Result<(), FieldError> {
    match given
        .iter()
        .find(|(field, present)| *present && !fields.contains(field))
    {
        Some((field, _)) => Err(FieldError::NotAField {
            kind,
            field,
            expected: fields
                .iter()
                .chain(shared)
                .map(|field| format!("`{field}`"))
                .collect::<Vec<_>>()
                .join(", "),
        }),
        None => Ok(()),
    }
}

impl TryFrom<ConstraintAsWritten> for Constraint {
    type Error = FieldError;

    fn try_from(written: ConstraintAsWritten) -> Result<Constraint, FieldError> {
        let name = written.rule;
        let given = [
            ("region", written.region.is_some()),
            ("width_degrees", written.width_degrees.is_some()),
            ("target", written.target.is_some()),
            ("within_metres", written.within_metres.is_some()),
            ("beyond_metres", written.beyond_metres.is_some()),
            ("metres", written.metres.is_some()),
        ];
        field_not_taken(name.name(), &given, name.fields(), &["hard", "weight"])?;
        let needs = |field: &'static str| FieldError::Missing {
            kind: name.name(),
            field,
        };
        let target = || written.target.clone().ok_or_else(|| needs("target"));
        let region = || written.region.ok_or_else(|| needs("region"));
        let width_degrees = written.width_degrees;
        let rule = match name {
            RuleName::CompassRegion => Rule::CompassRegion {
                region: region()?,
                width_degrees,
            },
            RuleName::NotInCompassRegion => Rule::NotInCompassRegion {
                region: region()?,
                width_degrees,
            },
            RuleName::Near => Rule::Near {
                target: target()?,
                within_metres: written.within_metres.unwrap_or_else(|| {
                    Length::new(NEAR_WITHIN_METRES).expect("the default distance is a valid length")
                }),
            },
            RuleName::FarFrom => Rule::FarFrom {
                target: target()?,
                beyond_metres: written.beyond_metres.unwrap_or_else(|| {
                    Length::new(FAR_FROM_BEYOND_METRES)
                        .expect("the default distance is a valid length")
                }),
            },
            RuleName::OnWater => Rule::OnWater,
            RuleName::AwayFromWater => Rule::AwayFromWater,
            RuleName::OnHighGround => Rule::OnHighGround,
            RuleName::OnLowGround => Rule::OnLowGround,
            RuleName::InsideWalls => Rule::InsideWalls { target: target()? },
            RuleName::OutsideWalls => Rule::OutsideWalls { target: target()? },
            RuleName::InDistrict => Rule::InDistrict { target: target()? },
            RuleName::NotInDistrict => Rule::NotInDistrict { target: target()? },
            RuleName::MinSpacing => Rule::MinSpacing {
                metres: written.metres.ok_or_else(|| needs("metres"))?,
            },
        };
        Ok(Constraint {
            rule,
            hard: written.hard.unwrap_or(false),
            weight: written.weight.unwrap_or_default(),
        })
    }
}

/// A road that reaches or crosses the site (specification-version-1 §2). It has no direction.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Road {
    /// The road's identifier in this file, unique across the whole file.
    pub identifier: String,
    /// Where the road starts and ends; the first end is where crossing numbers start.
    pub ends: [RoadEnd; 2],
    /// How important the road is; a major road outranks a minor one.
    pub importance: Importance,
    /// What a crossing of this road over a river is; a bridge when the file says nothing.
    #[serde(default)]
    pub river_crossing: RiverCrossingKind,
}

/// The word a specification file uses for the town core as a road end.
pub const CORE_WORD: &str = "core";

/// One end of a road, written in a file as one word: a compass region, `core`, or the
/// identifier of a pinned landmark or open space. Region names and `core` are read first, so
/// they are reserved and never name a place (specification-version-1 §5).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(from = "String", into = "String")]
pub enum RoadEnd {
    /// That stretch of the site's edge (`centre` is not an edge; validation rejects it).
    Region(CompassRegion),
    /// The town core, which S0 chooses (specification-version-1 §4).
    Core,
    /// A pinned landmark or open space by identifier; milestone 0b rejects it (validation, #97).
    Place(String),
}

impl From<String> for RoadEnd {
    fn from(word: String) -> RoadEnd {
        if word == CORE_WORD {
            return RoadEnd::Core;
        }
        CompassRegion::ALL
            .into_iter()
            .find(|region| region.name() == word)
            .map_or(RoadEnd::Place(word), RoadEnd::Region)
    }
}

impl From<RoadEnd> for String {
    fn from(end: RoadEnd) -> String {
        match end {
            RoadEnd::Region(region) => region.name().to_string(),
            RoadEnd::Core => CORE_WORD.to_string(),
            RoadEnd::Place(identifier) => identifier,
        }
    }
}

/// How important a road or a landmark is; the order is from least to most important.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum Importance {
    /// Minor, a landmark's default.
    #[default]
    Minor,
    /// A major road.
    Major,
}

/// What is built where a road crosses a river.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RiverCrossingKind {
    /// A bridge, the default.
    #[default]
    Bridge,
    /// A ford.
    Ford,
}

/// The style a file uses when it gives no `presets.style` (ADR-065).
pub const DEFAULT_STYLE: &str = "ink";

/// How far `near_point` reaches for a reused position when the file gives no
/// `presets.reuse_within_metres` (ADR-065).
pub const REUSE_WITHIN_METRES: f64 = 50.0;

/// The radius around the town core that counts as the compass region `centre` when the file
/// gives no `presets.centre_radius_metres` (ADR-065).
pub const CENTRE_RADIUS_METRES: f64 = 150.0;

/// Generation presets that pick defaults for the whole city (ADR-065).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Presets {
    /// The historical era whose defaults the generator uses.
    pub era: Era,
    /// The drawing style's name; `DEFAULT_STYLE` when the file gives none.
    #[serde(default = "default_style")]
    pub style: String,
    /// The names of the rule packs on top of the default pack, in file order.
    #[serde(default)]
    pub packs: Vec<String>,
    /// Whether an unpinned object with a `resolved` placement gets a soft rule to stay near it.
    #[serde(default)]
    pub reuse_positions: bool,
    /// How far that soft rule reaches; `REUSE_WITHIN_METRES` when the file gives none.
    #[serde(default = "default_reuse_within_metres")]
    pub reuse_within_metres: Length,
    /// How much that soft rule counts; `Weight::DEFAULT` when the file gives none.
    #[serde(default)]
    pub reuse_weight: Weight,
    /// The radius around the town core that counts as `centre`; `CENTRE_RADIUS_METRES` when
    /// the file gives none.
    #[serde(default = "default_centre_radius_metres")]
    pub centre_radius_metres: Length,
}

impl Presets {
    /// The presets of a file that gives only `era`.
    pub fn new(era: Era) -> Presets {
        Presets {
            era,
            style: default_style(),
            packs: Vec::new(),
            reuse_positions: false,
            reuse_within_metres: default_reuse_within_metres(),
            reuse_weight: Weight::DEFAULT,
            centre_radius_metres: default_centre_radius_metres(),
        }
    }
}

fn default_style() -> String {
    DEFAULT_STYLE.to_string()
}

fn default_reuse_within_metres() -> Length {
    Length::new(REUSE_WITHIN_METRES).expect("the default distance is a valid length")
}

fn default_centre_radius_metres() -> Length {
    Length::new(CENTRE_RADIUS_METRES).expect("the default radius is a valid length")
}

/// The historical era of a city, written in snake case in a file (`early_medieval`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Era {
    /// The early medieval era.
    EarlyMedieval,
    /// The high medieval era.
    HighMedieval,
    /// The late medieval era.
    LateMedieval,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_region_name_is_the_name_serde_writes() {
        for region in CompassRegion::ALL {
            let written = serde_json::to_string(&region).expect("a region serialises");
            assert_eq!(written, format!("\"{}\"", region.name()));
        }
    }

    #[test]
    fn a_road_end_is_a_region_the_core_or_a_place() {
        assert_eq!(
            RoadEnd::from("north_east".to_string()),
            RoadEnd::Region(CompassRegion::NorthEast)
        );
        assert_eq!(RoadEnd::from("core".to_string()), RoadEnd::Core);
        assert_eq!(
            RoadEnd::from("guildhall".to_string()),
            RoadEnd::Place("guildhall".to_string())
        );
    }

    #[test]
    fn a_road_end_is_written_back_as_the_same_word() {
        for word in ["west", "centre", "core", "guildhall"] {
            assert_eq!(String::from(RoadEnd::from(word.to_string())), word);
        }
    }

    #[test]
    fn a_place_named_like_a_region_reads_back_as_the_region() {
        let written = String::from(RoadEnd::Place("north".to_string()));
        assert_eq!(
            RoadEnd::from(written),
            RoadEnd::Region(CompassRegion::North)
        );
    }

    #[test]
    fn a_compass_word_is_its_bearing_in_degrees() {
        assert_eq!(Bearing::Word(CompassRegion::SouthEast).degrees(), Some(135));
        assert_eq!(Bearing::Word(CompassRegion::Centre).degrees(), None);
    }

    #[test]
    fn degrees_above_the_maximum_have_no_bearing() {
        assert_eq!(Bearing::Degrees(359).degrees(), Some(359));
        assert_eq!(Bearing::Degrees(360).degrees(), None);
    }

    #[test]
    fn a_bearing_is_plain_digits_or_a_compass_word() {
        assert_eq!(Bearing::from_text("0"), Some(Bearing::Degrees(0)));
        assert_eq!(Bearing::from_text("115"), Some(Bearing::Degrees(115)));
        assert_eq!(
            Bearing::from_text("south_east"),
            Some(Bearing::Word(CompassRegion::SouthEast))
        );
        for text in [
            "", "0x10", "+5", "0o17", "007", "12.5", "1e2", "70000", "sideways",
        ] {
            assert_eq!(Bearing::from_text(text), None, "{text}");
        }
    }

    #[test]
    fn a_major_road_outranks_a_minor_road() {
        assert!(Importance::Major > Importance::Minor);
    }
}
