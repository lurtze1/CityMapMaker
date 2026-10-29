//! The city specification: its typed values, its units and its validation.
//! Nothing in this crate performs I/O, reads a clock, or draws randomness; `citymap-core`
//! and the command-line `validate` and `schema` commands depend on it.

pub mod specification;
pub mod units;
pub mod validation;
