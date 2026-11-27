//! Golden test of stage S1: the wards of a crowded specification, with its picture. Windows
//! only, the reference platform (ARCHITECTURE.md §8, ADR-020).

#![cfg(windows)]

use std::fs;

use citymap_command_line::yaml;
use citymap_core::{layout, model, site};
use citymap_render::{Style, render_png};
use citymap_specification::specification::Specification;

const PICTURES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../output/golden");

/// A bay, a river into it and a river across the site, a pin near a cell corner, two pins in
/// one cell, a pin in the sea and a pin on a river (ADR-081).
const WARDHOLM: &str = "\
schema: 1
identity:
  name: Wardholm
  seed: 3
settlement:
  population: 4000
site:
  extent_metres: 1200
  terrain:
    coast: { bearing_degrees: south_east, shape: bay, distance_fraction: 0.5, width_metres: 500 }
  water:
    rivers:
      - { identifier: wend, from: north, to: south_east, width_metres: 30 }
      - { identifier: lisk, from: west, to: north_east, width_metres: 12 }
districts:
  - { identifier: old_town, name: Old Town, function: residential, weight: 1 }
landmarks:
  - identifier: corner_tower
    name: Corner Tower
    function: military
    importance: major
    resolved: { x: 300.01, y: 599.99, rotation_degrees: 0, width_metres: 20, depth_metres: 20 }
    pinned: true
  - identifier: keep
    name: Keep
    function: military
    importance: major
    resolved: { x: 420, y: 820, rotation_degrees: 0, width_metres: 30, depth_metres: 30 }
    pinned: true
  - identifier: well
    name: Well
    function: utility
    importance: minor
    resolved: { x: 470, y: 870, rotation_degrees: 0, width_metres: 5, depth_metres: 5 }
    pinned: true
  - identifier: beacon
    name: Beacon
    function: utility
    importance: minor
    resolved: { x: 1150, y: 60, rotation_degrees: 0, width_metres: 5, depth_metres: 5 }
    pinned: true
open_space:
  - identifier: mill_green
    kind: green
    resolved: { x: 600, y: 1000, rotation_degrees: 0, width_metres: 40, depth_metres: 40 }
    pinned: true
roads:
  - { identifier: high_road, ends: [west, east], importance: major }
presets:
  era: early_medieval
  height_grid_points_per_side: 128
";

#[test]
fn wardholm_layout_output() {
    let specification: Specification =
        yaml::from_text(WARDHOLM).expect("the test specification is valid");
    fs::create_dir_all(PICTURES).expect("the output folder can be made");
    let png =
        render_png(&model::generate(&specification), &Style::default()).expect("the map renders");
    fs::write(format!("{PICTURES}/wardholm-layout.png"), png).expect("the picture is written");
    let site = site::run(&specification);
    insta::assert_json_snapshot!(layout::run(&specification, &site));
}
