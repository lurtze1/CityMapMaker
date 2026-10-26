//! The city specification as typed values (specification-version-1 §2). Only the shape is
//! checked here; ranges and cross-field rules belong to validation. Every struct rejects keys
//! it does not know, so a typo in a file is an error.

use serde::{Deserialize, Serialize};

use crate::random::Seed;
use crate::units::Length;

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
    /// The roads that reach or cross the site; at least one (validation).
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
    /// The water on the site; none when the file has no `water` key.
    #[serde(default)]
    pub water: Water,
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

/// One end of a road, written in a file as one word: a compass region, `core`, or the
/// identifier of a pinned landmark or open space.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
pub enum RoadEnd {
    /// That stretch of the site's edge (`centre` is not an edge; validation rejects it).
    Region(CompassRegion),
    /// The town core, which S0 chooses (specification-version-1 §4).
    Core,
    /// A pinned landmark or open space by identifier; milestone 0b rejects it (validation).
    Place(String),
}

impl From<String> for RoadEnd {
    fn from(word: String) -> RoadEnd {
        if word == "core" {
            return RoadEnd::Core;
        }
        match CompassRegion::ALL
            .into_iter()
            .find(|region| region.name() == word)
        {
            Some(region) => RoadEnd::Region(region),
            None => RoadEnd::Place(word),
        }
    }
}

impl From<RoadEnd> for String {
    fn from(end: RoadEnd) -> String {
        match end {
            RoadEnd::Region(region) => region.name().to_string(),
            RoadEnd::Core => "core".to_string(),
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
    fn a_major_road_outranks_a_minor_road() {
        assert!(Importance::Major > Importance::Minor);
    }
}
