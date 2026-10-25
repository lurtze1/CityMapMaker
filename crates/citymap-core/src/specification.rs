//! The city specification: what a specification file says, as typed values (spec-v1 §2).
//! Only the shape is checked here; ranges and cross-field rules belong to validation.
//! Every struct rejects keys it does not know, so a typo in a file is an error.

use serde::{Deserialize, Serialize};

use crate::random::Seed;

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
