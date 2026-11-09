//! The city model: one field per stage output, each with exactly one producer
//! (ARCHITECTURE.md §3). Milestone 0b has only S0.

use serde::Serialize;

use crate::site::{self, SiteOutput};
use citymap_specification::specification::Specification;

/// Everything the generator made for one city; the renderer reads only this.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CityModel {
    /// The output of stage S0.
    pub site: SiteOutput,
}

/// Runs every stage on a specification, in order.
///
/// # Panics
///
/// In a debug build, when a stage output holds a duplicate identifier (see `site::run`).
pub fn generate(specification: &Specification) -> CityModel {
    CityModel {
        site: site::run(specification),
    }
}
