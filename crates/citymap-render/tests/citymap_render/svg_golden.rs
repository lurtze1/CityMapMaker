//! Golden test of the SVG renderer: the text for a fixed city. Windows only, the reference
//! platform (ARCHITECTURE.md §8, ADR-020).

#![cfg(windows)]

use citymap_core::model;
use citymap_render::{Style, render_svg};
use citymap_specification::specification::{
    Bearing, Cliff, Coast, CoastShape, CompassRegion, Defences, Era, Hill, Identity, Importance,
    LandCover, Presets, River, RiverCrossingKind, Road, RoadEnd, Seed, Settlement, Site, Slope,
    Specification, Terrain, Water,
};
use citymap_specification::units::{Gradient, Length};

fn length(value: f64) -> Length {
    Length::new(value).expect("test input is a valid length")
}

fn cliff(identifier: &str, bearing: u16, height: f64, drops_toward: Option<u16>) -> Cliff {
    Cliff {
        identifier: Some(identifier.to_string()),
        bearing_degrees: Bearing::Degrees(bearing),
        height_metres: length(height),
        distance_fraction: None,
        distance_metres: None,
        drops_toward_degrees: drops_toward.map(Bearing::Degrees),
    }
}

/// A bay to the east with a cliff on its shore, a cliff facing the town, a hill and a slope.
fn terrain() -> Terrain {
    Terrain {
        base_height_metres: None,
        coast: Some(Coast {
            bearing_degrees: Bearing::Word(CompassRegion::East),
            shape: CoastShape::Bay,
            shore: None,
            distance_fraction: None,
            distance_metres: None,
            depth_fraction: None,
            depth_metres: None,
            width_fraction: None,
            width_metres: Some(length(700.0)),
        }),
        cliffs: vec![
            cliff("white_cliffs", 80, 40.0, None),
            cliff("west_scarp", 270, 25.0, Some(90)),
        ],
        hills: vec![Hill {
            at: CompassRegion::North,
            radius_metres: length(300.0),
            height_metres: length(30.0),
            distance_fraction: None,
            distance_metres: None,
        }],
        hollows: Vec::new(),
        slope: Some(Slope {
            toward_degrees: Bearing::Word(CompassRegion::SouthEast),
            metres_per_kilometre: Gradient::new(10.0).expect("test input is a valid gradient"),
        }),
        plateau: None,
    }
}

fn river(identifier: &str, from: CompassRegion, to: CompassRegion, width: f64) -> River {
    River {
        identifier: identifier.to_string(),
        from,
        to,
        width_metres: Length::new(width).expect("test input is a valid length"),
        mouth: None,
    }
}

#[test]
fn harrowmere_svg() {
    let specification = Specification {
        schema: 1,
        identity: Identity {
            name: "Harrowmere".to_string(),
            seed: Seed::new(1),
            notes: String::new(),
        },
        settlement: Settlement { population: 6000 },
        site: Site {
            extent_metres: length(2400.0),
            terrain: terrain(),
            land_cover: LandCover::default(),
            water: Water {
                rivers: vec![
                    river("ash", CompassRegion::NorthWest, CompassRegion::East, 18.0),
                    river("brook", CompassRegion::South, CompassRegion::West, 4.5),
                ],
                ..Water::default()
            },
        },
        districts: Vec::new(),
        landmarks: Vec::new(),
        defences: Defences::default(),
        open_space: Vec::new(),
        crossings: Vec::new(),
        roads: vec![Road {
            identifier: "north_road".to_string(),
            ends: [
                RoadEnd::Region(CompassRegion::North),
                RoadEnd::Region(CompassRegion::South),
            ],
            importance: Importance::Major,
            river_crossing: RiverCrossingKind::Bridge,
        }],
        presets: Presets {
            height_grid_points_per_side: 48,
            ..Presets::new(Era::EarlyMedieval)
        },
    };
    insta::assert_snapshot!(render_svg(
        &model::generate(&specification),
        &Style::default()
    ));
}
