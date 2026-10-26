//! Pure generator core: turns a city spec and a seed into a city model.
//! Nothing in this crate performs I/O, reads a clock, or draws ambient randomness; every
//! adapter (CLI, export, UI) lives in another crate and depends on this one.

pub mod geometry;
pub mod identifier;
pub mod random;
pub mod site;
pub mod specification;
pub mod units;
