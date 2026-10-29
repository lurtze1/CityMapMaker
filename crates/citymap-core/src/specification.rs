//! The city specification as typed values (specification-version-1 §2). Only the shape is
//! checked here; ranges and cross-field rules belong to validation. Every struct rejects keys
//! it does not know, so a typo in a file is an error.

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::random::Seed;
use crate::units::{Gradient, Length, Weight};

/// One city specification, as read from a specification file.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    /// The city's districts; 1 to `MAXIMUM_DISTRICTS` (validation).
    pub districts: Vec<District>,
    /// The roads that reach or cross the site; at least one (validation, #97).
    pub roads: Vec<Road>,
    /// The generation presets.
    pub presets: Presets,
}

/// The city's name, its default seed and free notes for the GM.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

/// The settlement's size: the only size input of the generator.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Settlement {
    /// The number of inhabitants.
    pub population: u32,
}

/// The site: the square piece of land the map shows, measured from its south-west corner.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cliff {
    /// The direction from the site's centre toward the cliff.
    pub bearing_degrees: Bearing,
    /// How far the ground drops.
    pub height_metres: Length,
}

/// A rise of the ground around a point.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Slope {
    /// The direction the ground falls toward.
    pub toward_degrees: Bearing,
    /// How far the ground falls per kilometre.
    pub metres_per_kilometre: Gradient,
}

/// A raised flat part on one side of the site.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

impl<'de> Deserialize<'de> for Bearing {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Bearing, D::Error> {
        let text = String::deserialize(deserializer)?;
        Bearing::from_text(&text).ok_or_else(|| D::Error::custom(Bearing::EXPECTED))
    }
}

/// A district of the city (ADR-060): what it is for, how rich it is, and the rules that place
/// it. The generator assigns wards to it; `weight` is its size relative to the other districts.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Jurisdiction {
    /// The town, the default.
    #[default]
    Town,
    /// Someone else, such as a castle or a church.
    Separate,
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
}

/// A rule as a file writes it, every field optional and read one by one, so `Bearing` sees a
/// `region` as written; a tagged enum would first turn `0x10` into 16 (ADR-059).
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
struct ConstraintAsWritten {
    /// Which rule this is.
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

/// Why a rule as written cannot become a `Constraint`.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ConstraintFieldError {
    /// The rule was given a field it does not take.
    #[error("`{field}` is not a field of `{rule}`; it takes {expected}")]
    NotAField {
        /// The rule's name.
        rule: &'static str,
        /// The field it does not take.
        field: &'static str,
        /// The fields it takes, `hard` and `weight` included, as one list.
        expected: String,
    },
    /// The rule lacks a field it needs.
    #[error("`{rule}` needs `{field}`")]
    Missing {
        /// The rule's name.
        rule: &'static str,
        /// The field it needs.
        field: &'static str,
    },
}

impl TryFrom<ConstraintAsWritten> for Constraint {
    type Error = ConstraintFieldError;

    fn try_from(written: ConstraintAsWritten) -> Result<Constraint, ConstraintFieldError> {
        let name = written.rule;
        let given = [
            ("region", written.region.is_some()),
            ("width_degrees", written.width_degrees.is_some()),
            ("target", written.target.is_some()),
            ("within_metres", written.within_metres.is_some()),
            ("beyond_metres", written.beyond_metres.is_some()),
            ("metres", written.metres.is_some()),
        ];
        if let Some((field, _)) = given
            .into_iter()
            .find(|(field, present)| *present && !name.fields().contains(field))
        {
            return Err(ConstraintFieldError::NotAField {
                rule: name.name(),
                field,
                expected: name
                    .fields()
                    .iter()
                    .chain(&["hard", "weight"])
                    .map(|field| format!("`{field}`"))
                    .collect::<Vec<_>>()
                    .join(", "),
            });
        }
        let needs = |field: &'static str| ConstraintFieldError::Missing {
            rule: name.name(),
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

/// How important a road is; the order is from least to most important.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Importance {
    /// A minor road.
    Minor,
    /// A major road.
    Major,
}

/// What is built where a road crosses a river.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiverCrossingKind {
    /// A bridge, the default.
    #[default]
    Bridge,
    /// A ford.
    Ford,
}

/// Generation presets that pick defaults for the whole city.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Presets {
    /// The historical era whose defaults the generator uses.
    pub era: Era,
}

/// The historical era of a city, written in snake case in a file (`early_medieval`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
