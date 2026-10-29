//! The schema facade: the only module that turns the specification types into a schema
//! document (ADR-070). The derives and the hand-written `JsonSchema` impls on the types say
//! what each part holds; this module builds the whole document, lets an optional key be
//! written empty, and writes the JSON text that `citymap schema` prints.
//!
//! # Schemars functions used
//!
//! - `generate::SchemaSettings::draft2020_12`, `into_generator`, `into_root_schema_for`.
//! - `Schema::to_value`.

use citymap_specification::specification::Specification;
use schemars::generate::SchemaSettings;
use serde_json::{Map, Value, json};

/// The JSON schema of a specification file as pretty JSON text, the same on every run.
///
/// It is draft 2020-12 and ends in a newline.
///
/// # Panics
///
/// When `serde_json` cannot write the schema's JSON value as text, which it always can.
pub fn specification_schema() -> String {
    let mut schema = SchemaSettings::draft2020_12()
        .into_generator()
        .into_root_schema_for::<Specification>()
        .to_value();
    allow_empty_optional_keys(&mut schema);
    let text = serde_json::to_string_pretty(&schema)
        .expect("a schema is plain JSON, which always writes as text");
    text + "\n"
}

/// Lets every key an object does not require also be `null`, everywhere in `schema`: the YAML
/// reader reads an empty `landmarks:` as the key's default, so the editor must accept it.
fn allow_empty_optional_keys(schema: &mut Value) {
    match schema {
        Value::Object(object) => {
            let required: Vec<String> = object
                .get("required")
                .and_then(Value::as_array)
                .map(|names| {
                    names
                        .iter()
                        .filter_map(Value::as_str)
                        .map(str::to_string)
                        .collect()
                })
                .unwrap_or_default();
            if let Some(Value::Object(properties)) = object.get_mut("properties") {
                for (name, property) in properties.iter_mut() {
                    if !required.contains(name) && !allows_null(property) {
                        *property = with_null(property);
                    }
                }
            }
            for value in object.values_mut() {
                allow_empty_optional_keys(value);
            }
        }
        Value::Array(values) => values.iter_mut().for_each(allow_empty_optional_keys),
        _ => {}
    }
}

fn allows_null(property: &Value) -> bool {
    let null = Value::from("null");
    match property.get("type") {
        Some(Value::String(kind)) => kind == "null",
        Some(Value::Array(kinds)) => kinds.contains(&null),
        _ => property
            .get("anyOf")
            .and_then(Value::as_array)
            .is_some_and(|options| options.iter().any(allows_null)),
    }
}

fn with_null(property: &Value) -> Value {
    let mut inner = property.clone();
    let mut outer = Map::new();
    for key in ["description", "default"] {
        if let Some(value) = inner.as_object_mut().and_then(|object| object.remove(key)) {
            outer.insert(key.to_string(), value);
        }
    }
    outer.insert("anyOf".to_string(), json!([inner, { "type": "null" }]));
    Value::Object(outer)
}
