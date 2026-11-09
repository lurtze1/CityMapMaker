//! The `validate` command: checks a specification without generating a map.
//!
//! REQUIREMENTS.md F18. `generate` reads its specification through the same function.

use std::fs;
use std::path::Path;

use citymap_specification::specification::Specification;
use citymap_specification::validation;

use crate::CommandLineError;
use crate::yaml;

/// A specification that reads and passes every rule, with the warnings about it.
#[derive(Clone, Debug, PartialEq)]
pub struct ValidSpecification {
    /// The specification as read.
    pub specification: Specification,
    /// Remarks about the specification that may point to a mistake, such as a misspelt label.
    pub warnings: Vec<String>,
}

/// Reads the specification file and checks every rule of validation.
///
/// # Errors
///
/// `Reading` when the file cannot be read, `InvalidSpecification` when it is no
/// specification, `BrokenRules` when it breaks rules.
pub fn read_valid(path: &Path) -> Result<ValidSpecification, CommandLineError> {
    let text = fs::read_to_string(path).map_err(|error| CommandLineError::Reading {
        path: path.to_path_buf(),
        message: error.to_string(),
    })?;
    let specification: Specification =
        yaml::from_text(&text).map_err(|source| CommandLineError::InvalidSpecification {
            path: path.to_path_buf(),
            source,
        })?;
    let warnings = validation::warnings(&specification);
    validation::validate(&specification).map_err(|errors| CommandLineError::BrokenRules {
        path: path.to_path_buf(),
        errors,
        warnings: warnings.clone(),
    })?;
    Ok(ValidSpecification {
        specification,
        warnings: warnings.iter().map(ToString::to_string).collect(),
    })
}
