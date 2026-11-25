//! Golden test of stage S0: the model JSON of a fixed specification. Windows only, the
//! reference platform (ARCHITECTURE.md §8, ADR-020).

#![cfg(windows)]

use std::fs;

use citymap_command_line::yaml;
use citymap_core::{model, site};
use citymap_render::{Style, render_png};
use citymap_specification::specification::Specification;

const PICTURES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../output/golden");

/// Writes the specification's map as `output/golden/<name>.png`, so a change to its S0
/// snapshot is reviewed as a picture (ADR-078, #287).
fn write_picture(specification: &Specification, name: &str) {
    fs::create_dir_all(PICTURES).expect("the output folder can be made");
    let png =
        render_png(&model::generate(specification), &Style::default()).expect("the map renders");
    fs::write(format!("{PICTURES}/{name}.png"), png).expect("the picture is written");
}

const HARROWMERE: &str = "\
schema: 1
identity:
  name: Harrowmere
  seed: 1
settlement:
  population: 6000
site:
  extent_metres: 2400
  terrain:
    coast: { bearing_degrees: east, shape: bay, distance_fraction: 0.55, width_metres: 700 }
    slope:
      toward_degrees: south_east
      metres_per_kilometre: 10
    hills:
      - { at: north, radius_metres: 300, height_metres: 30 }
      - { at: centre, radius_metres: 80, height_metres: 5 }
      - { at: south_west, radius_metres: 200, height_metres: 18, distance_fraction: 1.5 }
    hollows:
      - { at: south_east, radius_metres: 150, depth_metres: 20, distance_metres: 1500 }
    cliffs:
      - { identifier: west_scarp, bearing_degrees: west, height_metres: 25, distance_fraction: 0.6, drops_toward_degrees: east }
      - { bearing_degrees: 200, height_metres: 8 }
      - { bearing_degrees: north, height_metres: 6, distance_metres: -300 }
      - { identifier: white_cliffs, bearing_degrees: 80, height_metres: 40 }
      - { identifier: corner_shelf, bearing_degrees: north_east, height_metres: 10, distance_fraction: 1.35, drops_toward_degrees: 30 }
    plateau: { bearing_degrees: east, height_metres: 12, distance_fraction: -0.1 }
  water:
    rivers:
      - identifier: ash
        from: north_west
        to: east
        width_metres: 18
      - identifier: brook
        from: south
        to: west
        width_metres: 4.5
districts:
  - identifier: old_town
    name: Old Town
    function: residential
    weight: 1
roads:
  - identifier: north_road
    ends: [north, south]
    importance: major
presets:
  era: early_medieval
  height_grid_points_per_side: 256
";

#[test]
fn harrowmere_site_output() {
    let specification: Specification =
        yaml::from_text(HARROWMERE).expect("the test specification is valid");
    write_picture(&specification, "harrowmere-site");
    insta::assert_json_snapshot!(site::run(&specification));
}

const KINGSMOUTH: &str = "schema: 1
identity:
  name: Kingsmouth
  seed: 2
settlement:
  population: 4000
site:
  extent_metres: 1800
  terrain:
    base_height_metres: 3
    coast: { bearing_degrees: 170, shape: open, distance_fraction: 0.45 }
    hills:
      - { at: north_west, radius_metres: 250, height_metres: 24 }
  water:
    rivers:
      - identifier: fal
        from: north
        to: south
        width_metres: 30
        mouth: { shape: estuary, width_metres: 420, reach_fraction: 0.9 }
districts:
  - identifier: quayside
    name: Quayside
    function: harbour
    weight: 1
roads:
  - identifier: quay_road
    ends: [west, east]
    importance: major
presets:
  era: early_medieval
  height_grid_points_per_side: 192
";

#[test]
fn kingsmouth_estuary_site_output() {
    let specification: Specification =
        yaml::from_text(KINGSMOUTH).expect("the test specification is valid");
    write_picture(&specification, "kingsmouth-site");
    insta::assert_json_snapshot!(site::run(&specification));
}
