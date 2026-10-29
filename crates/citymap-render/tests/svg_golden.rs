//! Golden test of the SVG renderer: the text for a fixed city. Windows only, the reference
//! platform (ARCHITECTURE.md §8, ADR-020).

#![cfg(windows)]

use citymap_core::model;
use citymap_render::{Style, render_svg};
use citymap_specification::specification::{
    CompassRegion, Defences, Era, Identity, Importance, LandCover, Presets, River,
    RiverCrossingKind, Road, RoadEnd, Seed, Settlement, Site, Specification, Terrain, Water,
};
use citymap_specification::units::Length;

fn river(identifier: &str, from: CompassRegion, to: CompassRegion, width: f64) -> River {
    River {
        identifier: identifier.to_string(),
        from,
        to,
        width_metres: Length::new(width).expect("test input is a valid length"),
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
            extent_metres: Length::new(2400.0).expect("test input is a valid length"),
            terrain: Terrain::default(),
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
        presets: Presets::new(Era::EarlyMedieval),
    };
    insta::assert_snapshot!(render_svg(
        &model::generate(&specification),
        &Style::default()
    ));
}
