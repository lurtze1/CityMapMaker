//! The `generate` command: specification file in, map files out (REQUIREMENTS.md F17).

use std::fs;
use std::path::{Path, PathBuf};

use citymap_core::model;
use citymap_core::town_core::CoreNotice;
use citymap_render::{Style, render_png, render_svg};
use citymap_specification::specification::{RoadEnd, Seed, Specification};

use crate::CommandLineError;
use crate::arguments::GenerateArguments;
use crate::validate::{Valid, read_valid};

/// Reads the specification, generates the city and writes its SVG map, and with `--png` its
/// PNG, to the output folder as `<specification file name>-seed-<seed>.svg` and `.png`;
/// returns the paths written and the notices to show.
///
/// # Errors
///
/// `Reading` or `Writing` for a file problem, `InvalidSpecification` or `BrokenRules` for a
/// bad specification,
/// `Rendering` when the PNG cannot be drawn.
///
/// # Panics
///
/// Never: a path that could be read as a file has a file name.
pub fn generate(arguments: &GenerateArguments) -> Result<Generated, CommandLineError> {
    let path = &arguments.specification;
    let Valid {
        mut specification,
        warnings,
    } = read_valid(path)?;
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
    let mut files = vec![(
        output.join(format!("{stem}.svg")),
        render_svg(&model, &style).into_bytes(),
    )];
    if arguments.png {
        let png =
            render_png(&model, &style).map_err(|source| CommandLineError::Rendering { source })?;
        files.push((output.join(format!("{stem}.png")), png));
    }
    let files = files
        .into_iter()
        .map(|(file, bytes)| write(&file, &bytes))
        .collect::<Result<Vec<PathBuf>, CommandLineError>>()?;
    let notices = model
        .site
        .core
        .notice
        .iter()
        .map(core_notice)
        .chain(model.site.zero_length_roads.iter().map(|road| {
            format!("road `{road}` has length zero: the town core lies on its own edge stretch")
        }))
        .chain(roads_to_places(&specification))
        .collect();
    Ok(Generated {
        files,
        notices,
        warnings,
    })
}

/// What `generate` did: the files it wrote, and the notices and warnings to show.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Generated {
    /// Every file written, the SVG first.
    pub files: Vec<PathBuf>,
    /// Remarks about the result that are no error, one sentence each.
    pub notices: Vec<String>,
    /// Remarks about the specification that may point to a mistake, such as a misspelt label.
    pub warnings: Vec<String>,
}

fn roads_to_places(specification: &Specification) -> impl Iterator<Item = String> + '_ {
    specification.roads.iter().filter_map(|road| {
        road.ends.iter().find_map(|end| match end {
            RoadEnd::Place(place) => Some(format!(
                "road `{}` ends at the pinned landmark or open space `{place}`; S0 lays out such \
                 roads from milestone 4, so this map does not show it yet",
                road.identifier
            )),
            RoadEnd::Region(_) | RoadEnd::Core => None,
        })
    })
}

fn core_notice(notice: &CoreNotice) -> String {
    format!(
        "the town core is the river crossing `{}`, chosen over the road crossing `{}` of equally \
         important roads because it is on the river; a finer importance level (#48) might \
         describe the town better",
        notice.chosen, notice.passed_over
    )
}

fn write(file: &Path, bytes: &[u8]) -> Result<PathBuf, CommandLineError> {
    fs::write(file, bytes).map_err(|error| CommandLineError::Writing {
        path: file.to_path_buf(),
        message: error.to_string(),
    })?;
    Ok(file.to_path_buf())
}
