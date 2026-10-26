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
