//! The city specification as typed values (specification-version-1 §2). Only the shape is
//! checked here; ranges and cross-field rules belong to validation. Every struct rejects keys
//! it does not know, so a typo in a file is an error.

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::random::Seed;
use crate::units::{Gradient, Length};

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

/// The water on the site; milestone 0b reads only rivers.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Water {
    /// The rivers that cross the site, from one edge stretch to another.
    #[serde(default)]
    pub rivers: Vec<River>,
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
