//! The `generate` command: specification file in, map files out (REQUIREMENTS.md F17).

use std::fs;
use std::path::{Path, PathBuf};

use citymap_core::model;
use citymap_core::random::Seed;
use citymap_core::specification::Specification;
use citymap_render::{Style, render_png, render_svg};

use crate::CommandLineError;
use crate::arguments::GenerateArguments;
use crate::yaml;

/// Reads the specification, generates the city and writes its SVG map, and with `--png` its
/// PNG, to the output folder as `<specification file name>-seed-<seed>.svg` and `.png`;
/// returns the paths written.
///
/// # Errors
///
/// `Reading` or `Writing` for a file problem, `InvalidSpecification` for a bad specification,
/// `Rendering` when the PNG cannot be drawn.
///
/// # Panics
///
/// Never: a path that could be read as a file has a file name.
pub fn generate(arguments: &GenerateArguments) -> Result<Vec<PathBuf>, CommandLineError> {
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
    let model = model::generate(&specification);
    let style = Style::default();
    let name = path
        .file_stem()
        .expect("a path that could be read as a file has a file name")
        .to_string_lossy();
    let stem = format!("{name}-seed-{}", specification.identity.seed.value());
    let output = &arguments.output;
    fs::create_dir_all(output).map_err(|error| CommandLineError::Writing {
        path: output.clone(),
        message: error.to_string(),
    })?;
    let mut written = vec![write(
        &output.join(format!("{stem}.svg")),
        render_svg(&model, &style).as_bytes(),
    )?];
    if arguments.png {
        let png =
            render_png(&model, &style).map_err(|source| CommandLineError::Rendering { source })?;
        written.push(write(&output.join(format!("{stem}.png")), &png)?);
    }
    Ok(written)
}

fn write(file: &Path, bytes: &[u8]) -> Result<PathBuf, CommandLineError> {
    fs::write(file, bytes).map_err(|error| CommandLineError::Writing {
        path: file.to_path_buf(),
        message: error.to_string(),
    })?;
    Ok(file.to_path_buf())
}
