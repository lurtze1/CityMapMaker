//! Golden test of the SVG renderer: the text for a fixed city. Windows only, the reference
//! platform (ARCHITECTURE.md §8, ADR-020).

#![cfg(windows)]

use citymap_core::model;
use citymap_core::random::Seed;
use citymap_core::specification::{
    CompassRegion, Era, Identity, Presets, River, Settlement, Site, Specification, Water,
};
use citymap_core::units::Length;
use citymap_render::{Style, render_svg};

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
            water: Water {
                rivers: vec![
                    river("ash", CompassRegion::NorthWest, CompassRegion::East, 18.0),
                    river("brook", CompassRegion::South, CompassRegion::West, 4.5),
                ],
            },
        },
        roads: Vec::new(),
        presets: Presets {
            era: Era::EarlyMedieval,
        },
    };
    insta::assert_snapshot!(render_svg(
        &model::generate(&specification),
        &Style::default()
    ));
}
