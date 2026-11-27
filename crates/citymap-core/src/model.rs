//! The city model: one field per stage output, each with exactly one producer
//! (ARCHITECTURE.md §3): S0 and, from milestone 3, S1.

use serde::Serialize;

use crate::layout::{self, LayoutOutput};
use crate::site::{self, SiteOutput};
use citymap_specification::specification::Specification;

/// Everything the generator made for one city; the renderer reads only this.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CityModel {
    /// The output of stage S0.
    pub site: SiteOutput,
    /// The output of stage S1.
    pub layout: LayoutOutput,
}

/// Runs every stage on a specification, in order.
///
/// # Panics
///
/// In a debug build, when a stage output holds a duplicate identifier (see `site::run` and
/// `layout::run`).
pub fn generate(specification: &Specification) -> CityModel {
    let site = site::run(specification);
    let layout = layout::run(specification, &site);
    CityModel { site, layout }
}
