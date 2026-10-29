//! Golden test of stage S0: the model JSON of a fixed specification. Windows only, the
//! reference platform (ARCHITECTURE.md §8, ADR-020).

#![cfg(windows)]

use citymap_command_line::yaml;
use citymap_core::site;
use citymap_core::specification::Specification;

const HARROWMERE: &str = "\
schema: 1
identity:
  name: Harrowmere
  seed: 1
settlement:
  population: 6000
site:
  extent_metres: 2400
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
";

#[test]
fn harrowmere_site_output() {
    let specification: Specification =
        yaml::from_text(HARROWMERE).expect("the test specification is valid");
    insta::assert_json_snapshot!(site::run(&specification));
}
