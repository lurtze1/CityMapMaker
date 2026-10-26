//! Rendering: turns a `CityModel` and a `Style` into an SVG map with one `<g>` per layer
//! (ARCHITECTURE.md §7). The SVG's user units are metres and its y axis points down, so the
//! model's northward y is flipped here (ADR-042).

mod svg_document;

use citymap_core::model::CityModel;
use citymap_core::units::Length;

use crate::svg_document::{Document, Group, Paint};

/// How the map looks. Not part of the city model: the same model renders in any style.
#[derive(Clone, Debug, PartialEq)]
pub struct Style {
    /// Fill of the site, any SVG colour; default `#f4ecd8`, parchment.
    pub land_colour: String,
    /// Outline of the site; default `#5b4a3a`, dark brown.
    pub border_colour: String,
    /// Width of the site's outline; default 4 m.
    pub border_width: Length,
    /// Rivers; default `#6f9fc8`, muted blue. A river is drawn as wide as it is.
    pub water_colour: String,
}

impl Default for Style {
    fn default() -> Style {
        Style {
            land_colour: "#f4ecd8".to_string(),
            border_colour: "#5b4a3a".to_string(),
            border_width: Length::new(4.0).expect("4 m is a valid length"),
            water_colour: "#6f9fc8".to_string(),
        }
    }
}

/// The SVG text of `model` drawn in `style`: a `terrain` group with the site, then a `water`
/// group with one polyline per river, whose `id` is the river's identifier.
pub fn render_svg(model: &CityModel, style: &Style) -> String {
    let extent = model.site.extent_metres.get();
    let border = style.border_width.get();
    let terrain = Group::new("terrain").rectangle(
        border / 2.0,
        border / 2.0,
        extent - border,
        extent - border,
        &Paint {
            fill: &style.land_colour,
            stroke: &style.border_colour,
            stroke_width: border,
        },
    );
    let mut water = Group::new("water");
    for river in &model.site.rivers {
        let points: Vec<(f64, f64)> = river
            .course
            .points()
            .iter()
            .map(|point| (point.x.get(), extent - point.y.get()))
            .collect();
        water = water.polyline(
            &river.identifier.to_string(),
            &points,
            &Paint {
                fill: "none",
                stroke: &style.water_colour,
                stroke_width: river.width_metres.get(),
            },
        );
    }
    Document::new(extent, extent)
        .group(terrain)
        .group(water)
        .into_text()
}

#[cfg(test)]
mod tests {
    use citymap_core::identifier::{Identifier, IdentifierKind};
    use citymap_core::model;
    use citymap_core::random::Seed;
    use citymap_core::specification::{
        CompassRegion, Era, Identity, Presets, River, Settlement, Site, Specification, Water,
    };
    use proptest::prelude::*;

    use super::*;

    fn length(value: f64) -> Length {
        Length::new(value).expect("test input is a valid length")
    }

    fn city(extent: f64, rivers: Vec<(&str, CompassRegion, CompassRegion)>) -> CityModel {
        let rivers = rivers
            .into_iter()
            .map(|(identifier, from, to)| River {
                identifier: identifier.to_string(),
                from,
                to,
                width_metres: length(18.0),
            })
            .collect();
        model::generate(&Specification {
            schema: 1,
            identity: Identity {
                name: "Harrowmere".to_string(),
                seed: Seed::new(1),
                notes: String::new(),
            },
            settlement: Settlement { population: 6000 },
            site: Site {
                extent_metres: length(extent),
                water: Water { rivers },
            },
            presets: Presets {
                era: Era::EarlyMedieval,
            },
        })
    }

    #[test]
    fn a_site_without_rivers_is_one_rectangle() {
        let text = render_svg(&city(2400.0, Vec::new()), &Style::default());
        assert!(
            text.contains(r#"viewBox="0.00 0.00 2400.00 2400.00""#),
            "{text}"
        );
        assert!(text.contains(r#"<g id="terrain">"#), "{text}");
        assert!(text.contains(r##"stroke="#5b4a3a""##), "{text}");
        assert!(!text.contains("<polyline"), "{text}");
    }

    #[test]
    fn north_is_at_the_top() {
        let text = render_svg(
            &city(
                2400.0,
                vec![("ash", CompassRegion::North, CompassRegion::South)],
            ),
            &Style::default(),
        );
        assert!(
            text.contains(r#"points="1200.00,0.00 1200.00,2400.00""#),
            "{text}"
        );
    }

    #[test]
    fn a_river_carries_its_identifier_and_width() {
        let text = render_svg(
            &city(
                2400.0,
                vec![("ash", CompassRegion::West, CompassRegion::East)],
            ),
            &Style::default(),
        );
        let identifier = Identifier::new(IdentifierKind::River, &[], &["ash"]);
        assert!(text.contains(&format!(r#"id="{identifier}""#)), "{text}");
        assert!(text.contains(r#"stroke-width="18.00""#), "{text}");
    }

    fn any_region() -> impl Strategy<Value = CompassRegion> {
        prop_oneof![
            Just(CompassRegion::North),
            Just(CompassRegion::NorthEast),
            Just(CompassRegion::East),
            Just(CompassRegion::SouthEast),
            Just(CompassRegion::South),
            Just(CompassRegion::SouthWest),
            Just(CompassRegion::West),
            Just(CompassRegion::NorthWest),
            Just(CompassRegion::Centre),
        ]
    }

    fn any_city() -> impl Strategy<Value = CityModel> {
        let rivers = prop::collection::btree_map("[a-z]{1,8}", (any_region(), any_region()), 0..6);
        (0.01..1e6_f64, rivers).prop_map(|(extent, rivers)| {
            let rivers = rivers
                .iter()
                .map(|(identifier, &(from, to))| (identifier.as_str(), from, to))
                .collect();
            city(extent, rivers)
        })
    }

    const NUMBER_ATTRIBUTES: [&str; 7] = [
        "x",
        "y",
        "width",
        "height",
        "stroke-width",
        "viewBox",
        "points",
    ];

    fn numbers_in(text: &str) -> Vec<String> {
        let parts: Vec<&str> = text.split('"').collect();
        let mut numbers = Vec::new();
        for pair in parts.chunks(2) {
            if let [before, value] = pair {
                let name = before
                    .trim_end_matches('=')
                    .rsplit(' ')
                    .next()
                    .unwrap_or("");
                if NUMBER_ATTRIBUTES.contains(&name) {
                    numbers.extend(value.split([' ', ',']).map(str::to_string));
                }
            }
        }
        numbers
    }

    fn has_two_decimals(number: &str) -> bool {
        let digits = number.strip_prefix('-').unwrap_or(number);
        let Some((whole, fraction)) = digits.split_once('.') else {
            return false;
        };
        number != "-0.00"
            && !whole.is_empty()
            && whole.chars().all(|character| character.is_ascii_digit())
            && fraction.len() == 2
            && fraction.chars().all(|character| character.is_ascii_digit())
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(64))]

        #[test]
        fn the_same_model_gives_the_same_text(model in any_city()) {
            prop_assert_eq!(
                render_svg(&model, &Style::default()),
                render_svg(&model, &Style::default())
            );
        }

        #[test]
        fn every_number_has_two_decimals(model in any_city()) {
            let text = render_svg(&model, &Style::default());
            let numbers = numbers_in(&text);
            prop_assert!(!numbers.is_empty());
            for number in numbers {
                prop_assert!(has_two_decimals(&number), "{} in {}", number, text);
            }
        }
    }
}
