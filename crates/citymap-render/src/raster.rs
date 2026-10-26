//! Raster facade: SVG text in, PNG bytes out. The only module that imports `resvg` and the
//! `usvg` and `tiny_skia` it re-exports; its default features are off, so no font or image
//! file on the computer can change the picture (ADR-044).
//!
//! # Resvg functions used
//!
//! - `resvg::usvg::Tree::from_str` with `usvg::Options::default()`, and `Tree::size`.
//! - `resvg::tiny_skia`: `Pixmap::new`, `Pixmap::as_mut`, `Pixmap::encode_png`,
//!   `Transform::from_scale`.
//! - `resvg::render`: draws the tree onto the pixmap.

use resvg::tiny_skia::{Pixmap, Transform};
use resvg::usvg::{Options, Tree};

/// Why SVG text could not become a PNG. It holds `resvg`'s message as our own text.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum RasterError {
    /// The text is not SVG that `resvg` can read.
    #[error("the SVG cannot be read: {message}")]
    InvalidSvg {
        /// What `resvg` reported.
        message: String,
    },
    /// No picture of this size can be made, such as a width of zero pixels.
    #[error("a picture of {width} by {height} pixels cannot be made")]
    InvalidSize {
        /// The width asked for, in pixels.
        width: u32,
        /// The height that follows from the SVG's shape, in pixels.
        height: u32,
    },
    /// The picture could not be written as PNG.
    #[error("the PNG cannot be written: {message}")]
    Encoding {
        /// What `resvg` reported.
        message: String,
    },
}

/// The PNG of `svg`, `width_pixels` wide, with the height that keeps the SVG's shape.
///
/// # Errors
///
/// `InvalidSvg` for unreadable text, `InvalidSize` for a zero size, `Encoding` otherwise.
pub fn png_from_svg(svg: &str, width_pixels: u32) -> Result<Vec<u8>, RasterError> {
    let tree =
        Tree::from_str(svg, &Options::default()).map_err(|error| RasterError::InvalidSvg {
            message: error.to_string(),
        })?;
    let size = tree.size();
    let scale = width_pixels as f32 / size.width();
    let height_pixels = (size.height() * scale).round() as u32;
    let mut pixmap = Pixmap::new(width_pixels, height_pixels).ok_or(RasterError::InvalidSize {
        width: width_pixels,
        height: height_pixels,
    })?;
    resvg::render(
        &tree,
        Transform::from_scale(scale, scale),
        &mut pixmap.as_mut(),
    );
    pixmap.encode_png().map_err(|error| RasterError::Encoding {
        message: error.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    const WIDE: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="10"><rect width="100" height="10" fill="#6f9fc8"/></svg>"##;

    fn header_size(png: &[u8]) -> (u32, u32) {
        let number = |start: usize| {
            u32::from_be_bytes(png[start..start + 4].try_into().expect("four bytes"))
        };
        (number(16), number(20))
    }

    #[test]
    fn the_result_is_a_png_with_the_asked_width_and_the_svg_shape() {
        let png = png_from_svg(WIDE, 200).expect("the SVG is valid");
        assert_eq!(
            png[..8],
            [0x89, b'P', b'N', b'G', b'\r', b'\n', 0x1a, b'\n']
        );
        assert_eq!(header_size(&png), (200, 20));
    }

    #[test]
    fn text_that_is_not_svg_is_an_error() {
        let error = png_from_svg("not svg", 100).expect_err("the text is not SVG");
        assert!(matches!(error, RasterError::InvalidSvg { .. }), "{error}");
    }

    #[test]
    fn a_width_of_zero_is_an_error() {
        let error = png_from_svg(WIDE, 0).expect_err("zero pixels make no picture");
        assert_eq!(
            error,
            RasterError::InvalidSize {
                width: 0,
                height: 0
            }
        );
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(64))]

        #[test]
        fn the_header_holds_the_asked_width(width in 10..4096_u32) {
            let png = png_from_svg(WIDE, width).expect("the SVG is valid");
            prop_assert_eq!(header_size(&png).0, width);
        }
    }
}
