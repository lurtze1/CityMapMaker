//! The `generate` command: specification file in, map files out (REQUIREMENTS.md F17).

use std::fs;
use std::path::PathBuf;

use citymap_core::model;
use citymap_core::random::Seed;
use citymap_core::specification::Specification;
use citymap_render::{Style, render_svg};

use crate::CommandLineError;
use crate::arguments::GenerateArguments;
use crate::yaml;

/// Reads the specification, generates the city and writes its SVG map to the output folder
/// as `<specification file name>-seed-<seed>.svg`; returns the path written.
///
/// # Errors
///
/// `Reading` or `Writing` for a file problem, `InvalidSpecification` for a bad specification.
///
/// # Panics
///
/// Never: a path that could be read as a file has a file name.
pub fn generate(arguments: &GenerateArguments) -> Result<PathBuf, CommandLineError> {
    let path = &arguments.specification;
    let text = fs::read_to_string(path).map_err(|error| CommandLineError::Reading {
        path: path.clone(),
        message: error.to_string(),
    })?;
    let mut specification: Specification =
        yaml::from_text(&text).map_err(|source| CommandLineError::InvalidSpecification {
            path: path.clone(),
            source,
        })?;
    if let Some(seed) = arguments.seed {
        specification.identity.seed = Seed::new(seed);
    }
    let svg = render_svg(&model::generate(&specification), &Style::default());
    let name = path
        .file_stem()
        .expect("a path that could be read as a file has a file name")
        .to_string_lossy();
    let output = &arguments.output;
    let file = output.join(format!(
        "{name}-seed-{}.svg",
        specification.identity.seed.value()
    ));
    fs::create_dir_all(output).map_err(|error| CommandLineError::Writing {
        path: output.clone(),
        message: error.to_string(),
    })?;
    fs::write(&file, svg).map_err(|error| CommandLineError::Writing {
        path: file.clone(),
        message: error.to_string(),
    })?;
    Ok(file)
}
