//! SVG facade: our own document and group builders over the `svg` crate. This is the only
//! module that imports `svg`; every number is written by `number` with two decimals, never by
//! the crate, so the text is byte-stable (ADR-042).
//!
//! # Svg functions used
//!
//! - `svg::Document`, `Document::new`: the root `<svg>` element with its `xmlns` attribute.
//! - `svg::node::element::{Group, Polyline, Rectangle}` and their `new`: the elements we draw.
//! - `set` and `add` on those elements, and `svg::Node::assign`: an attribute, a child element.
//! - `Display` on `svg::Document`: the text, with attributes sorted by name and escaped.

use svg::node::element::{Group as SvgGroup, Polyline, Rectangle};

/// A whole SVG document whose user units are metres, built up group by group.
pub(crate) struct Document {
    inner: svg::Document,
}

/// One layer of the map, written as `<g id="…">`.
pub(crate) struct Group {
    inner: SvgGroup,
}

/// How a shape is painted: fill colour, stroke colour and stroke width in metres.
pub(crate) struct Paint<'a> {
    pub(crate) fill: &'a str,
    pub(crate) stroke: &'a str,
    pub(crate) stroke_width: f64,
}

impl Document {
    /// An empty document showing `width` by `height` metres from the top-left corner.
    pub(crate) fn new(width: f64, height: f64) -> Document {
        let inner = svg::Document::new()
            .set("width", number(width))
            .set("height", number(height))
            .set(
                "viewBox",
                format!("0.00 0.00 {} {}", number(width), number(height)),
            );
        Document { inner }
    }

    /// Adds a group on top of the groups added before it.
    pub(crate) fn group(self, group: Group) -> Document {
        Document {
            inner: self.inner.add(group.inner),
        }
    }

    /// The SVG text of the document.
    pub(crate) fn into_text(self) -> String {
        self.inner.to_string()
    }
}

impl Group {
    /// An empty group with the given `id` attribute.
    pub(crate) fn new(identifier: &str) -> Group {
        Group {
            inner: SvgGroup::new().set("id", identifier),
        }
    }

    /// Adds a rectangle with its top-left corner at `left`, `top`.
    pub(crate) fn rectangle(
        self,
        left: f64,
        top: f64,
        width: f64,
        height: f64,
        paint: &Paint,
    ) -> Group {
        let rectangle = painted(Rectangle::new(), paint)
            .set("x", number(left))
            .set("y", number(top))
            .set("width", number(width))
            .set("height", number(height));
        Group {
            inner: self.inner.add(rectangle),
        }
    }

    /// Adds an open line through `points`, each an `(x, y)` pair, with its own `id` attribute.
    pub(crate) fn polyline(self, identifier: &str, points: &[(f64, f64)], paint: &Paint) -> Group {
        let points: Vec<String> = points
            .iter()
            .map(|&(x, y)| format!("{},{}", number(x), number(y)))
            .collect();
        let polyline = painted(Polyline::new(), paint)
            .set("id", identifier)
            .set("points", points.join(" "));
        Group {
            inner: self.inner.add(polyline),
        }
    }
}

fn painted<T: svg::Node>(mut element: T, paint: &Paint) -> T {
    element.assign("fill", paint.fill);
    element.assign("stroke", paint.stroke);
    element.assign("stroke-width", number(paint.stroke_width));
    element
}

fn number(value: f64) -> String {
    let text = format!("{value:.2}");
    if text == "-0.00" {
        "0.00".to_string()
    } else {
        text
    }
}

#[cfg(test)]
mod tests {
    use super::number;

    #[test]
    fn numbers_have_two_decimals_and_round_half_away_from_zero() {
        assert_eq!(number(2400.0), "2400.00");
        assert_eq!(number(2400.0 - 0.01), "2399.99");
        assert_eq!(number(4.5), "4.50");
        assert_eq!(number(-12.345), "-12.35");
    }

    #[test]
    fn negative_zero_is_written_as_zero() {
        assert_eq!(number(-0.0), "0.00");
        assert_eq!(number(-0.004), "0.00");
    }
}
