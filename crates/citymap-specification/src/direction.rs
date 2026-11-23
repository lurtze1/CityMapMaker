//! The `libm` facade: the only module that calls `libm` (ADR-020, ADR-074). It turns a
//! whole-degree bearing into a direction, so every crate gets the same sine and cosine on every
//! platform. The `libm` crate can be replaced here alone.
//!
//! # Libm functions used
//!
//! - `sin`, `cos`.

use std::f64::consts::PI;

/// A one-metre step toward a bearing, split along the site's two axes as a `Point` is: `x`
/// grows toward the east and `y` toward the north.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Direction {
    /// The part of the step along the x axis, in metres.
    pub x: f64,
    /// The part of the step along the y axis, in metres.
    pub y: f64,
}

/// The direction of a bearing in whole degrees clockwise from north.
///
/// # Examples
///
/// ```
/// use citymap_specification::direction::toward;
///
/// let east = toward(90);
/// assert!((east.x - 1.0).abs() < 1e-12 && east.y.abs() < 1e-12);
/// ```
pub fn toward(degrees: u16) -> Direction {
    let radians = f64::from(degrees) * PI / 180.0;
    Direction {
        x: libm::sin(radians),
        y: libm::cos(radians),
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::toward;

    #[test]
    fn the_four_main_bearings_point_along_the_axes() {
        for (degrees, x, y) in [
            (0, 0.0, 1.0),
            (90, 1.0, 0.0),
            (180, 0.0, -1.0),
            (270, -1.0, 0.0),
        ] {
            let direction = toward(degrees);
            assert!((direction.x - x).abs() < 1e-12, "{degrees}: {direction:?}");
            assert!((direction.y - y).abs() < 1e-12, "{degrees}: {direction:?}");
        }
    }

    proptest! {
        #[test]
        fn every_direction_is_one_metre_long(degrees in 0..360_u16) {
            let direction = toward(degrees);
            let length = (direction.x * direction.x + direction.y * direction.y).sqrt();
            prop_assert!((length - 1.0).abs() < 1e-12);
        }

        #[test]
        fn the_opposite_bearing_points_back(degrees in 0..180_u16) {
            let there = toward(degrees);
            let back = toward(degrees + 180);
            prop_assert!((there.x + back.x).abs() < 1e-12);
            prop_assert!((there.y + back.y).abs() < 1e-12);
        }
    }
}
