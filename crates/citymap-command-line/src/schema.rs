//! The schema facade: the only module that calls `schemars` to build a schema (ADR-070). The
//! derives on the specification types say what the schema holds; this module turns them into
//! the JSON text that `citymap schema` prints and `documentation/specification.schema.json`
//! keeps.
//!
//! # Schemars functions used
//!
//! - `generate::SchemaSettings::draft2020_12`, `into_generator`, `into_root_schema_for`.
//! - `Schema`, written out with `serde_json::to_string_pretty`.

use citymap_specification::specification::Specification;
use schemars::generate::SchemaSettings;

/// The JSON schema of a specification file, draft 2020-12, as pretty JSON text ending in a
/// newline; the same text on every run.
///
/// # Panics
///
/// When `serde_json` cannot write the schema's JSON value as text, which it always can.
pub fn specification_schema() -> String {
    let schema = SchemaSettings::draft2020_12()
        .into_generator()
        .into_root_schema_for::<Specification>();
    let text = serde_json::to_string_pretty(&schema)
        .expect("a schema is plain JSON, which always writes as text");
    text + "\n"
}
