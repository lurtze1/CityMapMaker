//! Rendering: turns a `CityModel` and a `Style` into an SVG map with one `<g>` per layer
//! (ARCHITECTURE.md §7). The SVG's user units are metres and its y axis points down, so the
//! model's northward y is flipped here (ADR-042).

mod raster;
mod svg_document;

use citymap_core::geometry::Point;
use citymap_core::model::CityModel;
use citymap_core::terrain::{Contour, Terrain};
use citymap_specification::specification::{Importance, RiverCrossingKind};
use citymap_specification::units::{Length, SignedLength};

pub use crate::raster::RasterError;
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
    /// Width of a PNG in pixels; default 2048. The height keeps the map's shape.
    pub width_pixels: u32,
    /// Roads; default `#8a6f4e`, light brown.
    pub road_colour: String,
    /// Width of a major road; default 10 m.
    pub major_road_width: Length,
    /// Width of a minor road; default 5 m.
    pub minor_road_width: Length,
    /// Bridges; default `#5b4a3a`, dark brown.
    pub bridge_colour: String,
    /// Fords; default `#c9b38a`, sand.
    pub ford_colour: String,
    /// The town core's ring, drawn on top so what the core lies on stays visible; default
    /// `#2e8b57`, green.
    pub core_colour: String,
    /// The diameter of the core's ring; default 40 m.
    pub core_size: Length,
    /// The line width of the core's ring; default 4 m.
    pub core_line_width: Length,
    /// The height between two contour lines; default 5 m (ADR-076, X1).
    pub contour_interval: Length,
    /// Contour lines; default `#b59b74`, faded brown.
    pub contour_colour: String,
    /// The width of a contour line; default 1 m.
    pub contour_width: Length,
    /// Contour lines in the sea, depth lines; default `#9dbfdc`, faded blue, as charts draw
    /// depth contours (ADR-076, III1).
    pub depth_contour_colour: String,
    /// Cliff lines and their ticks; default `#5b4a3a`, dark brown.
    pub cliff_colour: String,
    /// The width of a cliff line and its ticks; default 2 m.
    pub cliff_width: Length,
    /// The distance between two ticks on a cliff's low side; default 20 m (ADR-076, GGG1).
    pub cliff_tick_spacing: Length,
    /// How far a cliff's tick reaches down from its line; default 10 m.
    pub cliff_tick_length: Length,
    /// The shoreline; default `#3f6f94`, dark blue. The sea is filled in `water_colour`.
    pub shoreline_colour: String,
    /// The width of the shoreline; default 2 m.
    pub shoreline_width: Length,
}

impl Default for Style {
    fn default() -> Style {
        Style {
            land_colour: "#f4ecd8".to_string(),
            border_colour: "#5b4a3a".to_string(),
            border_width: Length::new(4.0).expect("4 m is a valid length"),
            water_colour: "#6f9fc8".to_string(),
            width_pixels: 2048,
            road_colour: "#8a6f4e".to_string(),
            major_road_width: Length::new(10.0).expect("10 m is a valid length"),
            minor_road_width: Length::new(5.0).expect("5 m is a valid length"),
            bridge_colour: "#5b4a3a".to_string(),
            ford_colour: "#c9b38a".to_string(),
            core_colour: "#2e8b57".to_string(),
            core_size: Length::new(40.0).expect("40 m is a valid length"),
            core_line_width: Length::new(4.0).expect("4 m is a valid length"),
            contour_interval: Length::new(5.0).expect("5 m is a valid length"),
            contour_colour: "#b59b74".to_string(),
            contour_width: Length::new(1.0).expect("1 m is a valid length"),
            depth_contour_colour: "#9dbfdc".to_string(),
            cliff_colour: "#5b4a3a".to_string(),
            cliff_width: Length::new(2.0).expect("2 m is a valid length"),
            cliff_tick_spacing: Length::new(20.0).expect("20 m is a valid length"),
            cliff_tick_length: Length::new(10.0).expect("10 m is a valid length"),
            shoreline_colour: "#3f6f94".to_string(),
            shoreline_width: Length::new(2.0).expect("2 m is a valid length"),
        }
    }
}

impl Style {
    /// The width a road of this importance is drawn with.
    pub fn road_width(&self, importance: Importance) -> Length {
        match importance {
            Importance::Major => self.major_road_width,
            Importance::Minor => self.minor_road_width,
        }
    }
}

/// The SVG text of `model` drawn in `style`: groups `terrain` (land, sea, contour lines,
/// cliffs, shoreline), `water`, `roads`, `crossings`, `core` and `frame`, in that order; every
/// model object carries its identifier as `id`.
pub fn render_svg(model: &CityModel, style: &Style) -> String {
    render_svg_with_contours(model, style, &contours_of(model, style))
}

/// The SVG text of `model` drawn in `style` with the given contour lines instead of the
/// model's own, to compare ways of tracing them (ADR-076, X1).
pub fn render_svg_with_contours(model: &CityModel, style: &Style, contours: &[Contour]) -> String {
    let extent = model.site.extent_metres.get();
    let border = style.border_width.get();
    let terrain = terrain_group(model, style, contours);
    let mut water = Group::new("water");
    for river in &model.site.rivers {
        water = water.polyline(
            &river.identifier.to_string(),
            &flipped(river.course.points(), extent),
            &Paint {
                fill: "none",
                stroke: &style.water_colour,
                stroke_width: river.width_metres.get(),
            },
        );
    }
    let mut roads = Group::new("roads");
    for road in &model.site.roads {
        roads = roads.polyline(
            &road.identifier.to_string(),
            &flipped(road.course.points(), extent),
            &Paint {
                fill: "none",
                stroke: &style.road_colour,
                stroke_width: style.road_width(road.importance).get(),
            },
        );
    }
    let mut crossings = Group::new("crossings");
    for crossing in &model.site.crossings {
        let colour = match crossing.kind {
            RiverCrossingKind::Bridge => &style.bridge_colour,
            RiverCrossingKind::Ford => &style.ford_colour,
        };
        crossings = crossings.polyline(
            &crossing.identifier.to_string(),
            &flipped(crossing.span.points(), extent),
            &Paint {
                fill: "none",
                stroke: colour,
                stroke_width: crossing.width.get(),
            },
        );
    }
    let core = &model.site.core;
    let core_group = Group::new("core").circle(
        &core.identifier.to_string(),
        core.position.x.get(),
        extent - core.position.y.get(),
        style.core_size.get() / 2.0,
        &Paint {
            fill: "none",
            stroke: &style.core_colour,
            stroke_width: style.core_line_width.get(),
        },
    );
    let frame = Group::new("frame").rectangle(
        border / 2.0,
        border / 2.0,
        extent - border,
        extent - border,
        &Paint {
            fill: "none",
            stroke: &style.border_colour,
            stroke_width: border,
        },
    );
    Document::new(extent, extent)
        .group(terrain)
        .group(water)
        .group(roads)
        .group(crossings)
        .group(core_group)
        .group(frame)
        .into_text()
}

fn contours_of(model: &CityModel, style: &Style) -> Vec<Contour> {
    let interval =
        SignedLength::new(style.contour_interval.get()).expect("a length is a valid signed length");
    model.site.terrain.contours(Terrain::surface(), interval)
}

fn terrain_group(model: &CityModel, style: &Style, contours: &[Contour]) -> Group {
    let extent = model.site.extent_metres.get();
    let terrain = &model.site.terrain;
    let mut group = Group::new("terrain").rectangle(
        0.0,
        0.0,
        extent,
        extent,
        &Paint {
            fill: &style.land_colour,
            stroke: "none",
            stroke_width: 0.0,
        },
    );
    for area in terrain.sea_areas(Terrain::surface()) {
        group = group.polygon(
            &flipped(area.points(), extent),
            &Paint {
                fill: &style.water_colour,
                stroke: "none",
                stroke_width: 0.0,
            },
        );
    }
    for contour in contours {
        let points = contour.course.points();
        let in_the_sea = terrain
            .is_sea(Terrain::surface(), points[points.len() / 2])
            .unwrap_or(false);
        let colour = if in_the_sea {
            &style.depth_contour_colour
        } else {
            &style.contour_colour
        };
        group = group.line(
            &flipped(points, extent),
            &Paint {
                fill: "none",
                stroke: colour,
                stroke_width: style.contour_width.get(),
            },
        );
    }
    let cliff_paint = Paint {
        fill: "none",
        stroke: &style.cliff_colour,
        stroke_width: style.cliff_width.get(),
    };
    for ground in &terrain.grounds {
        for cliff in &ground.cliff_lines {
            group = group
                .polyline(
                    &cliff.identifier.to_string(),
                    &flipped(&[cliff.from, cliff.to], extent),
                    &cliff_paint,
                )
                .segments(
                    &cliff_ticks(cliff.from, cliff.to, style, extent),
                    &cliff_paint,
                );
        }
        for shoreline in &ground.shorelines {
            group = group.polyline(
                &shoreline.identifier.to_string(),
                &flipped(shoreline.course.points(), extent),
                &Paint {
                    fill: "none",
                    stroke: &style.shoreline_colour,
                    stroke_width: style.shoreline_width.get(),
                },
            );
        }
    }
    group
}

/// The ticks of a cliff line from `from` to `to`: short strokes on its right, its low side,
/// every `cliff_tick_spacing` (ADR-076, GGG1), in SVG coordinates.
fn cliff_ticks(from: Point, to: Point, style: &Style, extent: f64) -> Vec<[(f64, f64); 2]> {
    let (walk_x, walk_y) = (to.x.get() - from.x.get(), to.y.get() - from.y.get());
    let length = (walk_x * walk_x + walk_y * walk_y).sqrt();
    let spacing = style.cliff_tick_spacing.get();
    if length == 0.0 || spacing == 0.0 {
        return Vec::new();
    }
    let (along_x, along_y) = (walk_x / length, walk_y / length);
    let reach = style.cliff_tick_length.get();
    let (down_x, down_y) = (along_y * reach, -along_x * reach);
    let count = (length / spacing).floor() as usize;
    (1..=count)
        .map(|tick| {
            let distance = tick as f64 * spacing - spacing / 2.0;
            let (x, y) = (
                from.x.get() + along_x * distance,
                from.y.get() + along_y * distance,
            );
            [(x, extent - y), (x + down_x, extent - (y + down_y))]
        })
        .collect()
}

fn flipped(points: &[Point], extent: f64) -> Vec<(f64, f64)> {
    points
        .iter()
        .map(|point| (point.x.get(), extent - point.y.get()))
        .collect()
}

/// The PNG of `model` drawn in `style`, `style.width_pixels` wide (ADR-044).
///
/// # Errors
///
/// `RasterError` when the SVG cannot become a PNG, such as for a width of zero pixels.
pub fn render_png(model: &CityModel, style: &Style) -> Result<Vec<u8>, RasterError> {
    raster::png_from_svg(&render_svg(model, style), style.width_pixels)
}

/// The PNG of `model` drawn in `style` with the given contour lines instead of the model's own,
/// to compare ways of tracing them (ADR-076, X1).
///
/// # Errors
///
/// `RasterError` when the SVG cannot become a PNG, such as for a width of zero pixels.
pub fn render_png_with_contours(
    model: &CityModel,
    style: &Style,
    contours: &[Contour],
) -> Result<Vec<u8>, RasterError> {
    raster::png_from_svg(
        &render_svg_with_contours(model, style, contours),
        style.width_pixels,
    )
}

#[cfg(test)]
mod tests {
    use citymap_core::identifier::{Identifier, IdentifierKind};
    use citymap_core::model;
    use citymap_specification::specification::{
        Bearing, Cliff, CompassRegion, Defences, Era, Identity, LandCover, Presets, River, Road,
        RoadEnd, Seed, Settlement, Site, Slope, Specification, Terrain, Water,
    };
    use citymap_specification::units::Gradient;
    use proptest::prelude::*;

    use super::*;

    fn length(value: f64) -> Length {
        Length::new(value).expect("test input is a valid length")
    }

    fn with_road(importance: Importance) -> Specification {
        let mut specification = specification(2400.0, Vec::new());
        specification.roads = vec![Road {
            identifier: "west_lane".to_string(),
            ends: [
                RoadEnd::Region(CompassRegion::NorthWest),
                RoadEnd::Region(CompassRegion::South),
            ],
            importance,
            river_crossing: RiverCrossingKind::Bridge,
        }];
        specification
    }

    fn city(extent: f64, rivers: Vec<(&str, CompassRegion, CompassRegion)>) -> CityModel {
        model::generate(&specification(extent, rivers))
    }

    fn specification(
        extent: f64,
        rivers: Vec<(&str, CompassRegion, CompassRegion)>,
    ) -> Specification {
        let rivers = rivers
            .into_iter()
            .map(|(identifier, from, to)| River {
                identifier: identifier.to_string(),
                from,
                to,
                width_metres: length(18.0),
                mouth: None,
            })
            .collect();
        Specification {
            schema: 1,
            identity: Identity {
                name: "Harrowmere".to_string(),
                seed: Seed::new(1),
                notes: String::new(),
            },
            settlement: Settlement { population: 6000 },
            site: Site {
                extent_metres: length(extent),
                terrain: Terrain::default(),
                land_cover: LandCover::default(),
                water: Water {
                    rivers,
                    ..Water::default()
                },
            },
            districts: Vec::new(),
            landmarks: Vec::new(),
            defences: Defences::default(),
            open_space: Vec::new(),
            crossings: Vec::new(),
            roads: Vec::new(),
            presets: Presets {
                height_grid_points_per_side: 16,
                ..Presets::new(Era::EarlyMedieval)
            },
        }
    }

    #[test]
    fn a_site_without_rivers_is_one_rectangle() {
        let text = render_svg(&city(2400.0, Vec::new()), &Style::default());
        assert!(
            text.contains(r#"viewBox="0.00 0.00 2400.00 2400.00""#),
            "{text}"
        );
        assert!(text.contains(r#"<g id="terrain">"#), "{text}");
        let frame = text.find(r#"<g id="frame">"#).expect("the frame is drawn");
        let water = text
            .find(r#"<g id="water""#)
            .expect("the water group is drawn");
        assert!(frame > water, "the frame is drawn after the water: {text}");
        assert!(text.contains(r#"fill="none" height="2396.00""#), "{text}");
        assert!(text.contains(r##"stroke="#5b4a3a""##), "{text}");
        assert!(!text.contains("<polyline"), "{text}");
    }

    #[test]
    fn a_png_is_as_wide_as_the_style_asks() {
        let style = Style {
            width_pixels: 300,
            ..Style::default()
        };
        let png = render_png(&city(2400.0, Vec::new()), &style).expect("the map renders");
        let number = |start: usize| {
            u32::from_be_bytes(png[start..start + 4].try_into().expect("four bytes"))
        };
        assert_eq!((number(16), number(20)), (300, 300));
    }

    #[test]
    fn a_cliff_has_its_ticks_on_its_low_side() {
        let mut specification = specification(1000.0, Vec::new());
        specification.site.terrain.cliffs = vec![Cliff {
            identifier: None,
            bearing_degrees: Bearing::Degrees(90),
            height_metres: length(5.0),
            distance_fraction: None,
            distance_metres: Some(SignedLength::new(100.0).expect("a valid length")),
            drops_toward_degrees: None,
        }];
        let svg = render_svg(&model::generate(&specification), &Style::default());
        assert!(
            svg.contains("M600.00,990.00L610.00,990.00"),
            "the first tick, 10 m along the line, points east: {svg}"
        );
    }

    #[test]
    fn a_road_is_drawn_as_wide_as_its_importance() {
        for (importance, width) in [(Importance::Minor, "5.00"), (Importance::Major, "10.00")] {
            let text = render_svg(&model::generate(&with_road(importance)), &Style::default());
            let wanted = format!(r##"stroke="#8a6f4e" stroke-width="{width}""##);
            assert!(text.contains(&wanted), "{text}");
        }
    }

    #[test]
    fn a_road_carries_its_identifier_and_is_flipped() {
        let text = render_svg(
            &model::generate(&with_road(Importance::Major)),
            &Style::default(),
        );
        let identifier = Identifier::new(IdentifierKind::Road, &[], &["west_lane"]);
        assert!(text.contains(&format!(r#"id="{identifier}""#)), "{text}");
        assert!(
            text.contains(r#"points="0.00,0.00 1200.00,2400.00""#),
            "{text}"
        );
    }

    #[test]
    fn a_crossing_is_drawn_in_the_colour_of_its_kind() {
        for (kind, colour) in [
            (RiverCrossingKind::Bridge, "#5b4a3a"),
            (RiverCrossingKind::Ford, "#c9b38a"),
        ] {
            let mut specification = with_road(Importance::Major);
            specification.roads[0].river_crossing = kind;
            specification.site.water.rivers = vec![River {
                identifier: "ash".to_string(),
                from: CompassRegion::West,
                to: CompassRegion::East,
                width_metres: length(18.0),
                mouth: None,
            }];
            let text = render_svg(&model::generate(&specification), &Style::default());
            let crossings = &text[text.find(r#"<g id="crossings">"#).expect("drawn")..];
            let wanted = format!(r#"stroke="{colour}" stroke-width="8.00""#);
            assert!(crossings.contains(&wanted), "{text}");
        }
    }

    #[test]
    fn the_core_is_an_open_green_ring_centred_on_its_position() {
        let text = render_svg(
            &model::generate(&with_road(Importance::Major)),
            &Style::default(),
        );
        let core = &text[text.find(r#"<g id="core">"#).expect("drawn")..];
        assert!(core.contains("<circle cx="), "{text}");
        assert!(core.contains(r#"fill="none""#), "{text}");
        assert!(
            core.contains(r##"r="20.00" stroke="#2e8b57" stroke-width="4.00""##),
            "{text}"
        );
    }

    #[test]
    fn groups_are_drawn_in_layer_order() {
        let text = render_svg(
            &model::generate(&with_road(Importance::Major)),
            &Style::default(),
        );
        let position = |group: &str| {
            text.find(&format!(r#"<g id="{group}""#))
                .expect("every group is drawn")
        };
        let order = ["terrain", "water", "roads", "crossings", "core", "frame"].map(position);
        assert!(order.windows(2).all(|pair| pair[0] < pair[1]), "{text}");
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
        let roads = prop::collection::btree_map(
            "[a-z]{1,8}",
            (any_region(), any_region(), any::<bool>()),
            0..6,
        );
        (0.01..1e6_f64, rivers, roads).prop_map(|(extent, rivers, roads)| {
            let rivers = rivers
                .iter()
                .map(|(identifier, &(from, to))| (identifier.as_str(), from, to))
                .collect();
            let mut specification = specification(extent, rivers);
            specification.site.terrain.slope = Some(Slope {
                toward_degrees: Bearing::Degrees(135),
                metres_per_kilometre: Gradient::new(10.0).expect("10 m per km is a valid gradient"),
            });
            specification.site.terrain.cliffs = vec![Cliff {
                identifier: None,
                bearing_degrees: Bearing::Degrees(90),
                height_metres: Length::new(5.0).expect("5 m is a valid length"),
                distance_fraction: None,
                distance_metres: None,
                drops_toward_degrees: None,
            }];
            specification.roads = roads
                .into_iter()
                .map(|(identifier, (first, second, major))| Road {
                    identifier,
                    ends: [RoadEnd::Region(first), RoadEnd::Region(second)],
                    importance: if major {
                        Importance::Major
                    } else {
                        Importance::Minor
                    },
                    river_crossing: RiverCrossingKind::Bridge,
                })
                .collect();
            model::generate(&specification)
        })
    }

    const NUMBER_ATTRIBUTES: [&str; 8] = [
        "d",
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
                    numbers.extend(
                        value
                            .split([' ', ',', 'M', 'L'])
                            .filter(|number| !number.is_empty())
                            .map(str::to_string),
                    );
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
