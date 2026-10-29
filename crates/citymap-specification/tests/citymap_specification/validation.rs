//! Tests of `validate`, `warnings` and `check_constraints` through the public items only.

use proptest::prelude::*;

use crate::support::{
    add_district, any_valid_specification, bridge, citadel, compass_region, gate, gradient,
    guildhall, harbour_at, identifiers, landmark_near, length, market, near, placement, road_gate,
    rule, valid, wall, weight, with_defences, with_gate_and_citadel, with_land_cover,
    with_landmark, with_market_and_bridge, with_terrain, with_walls, with_water,
};
use citymap_specification::specification::{
    Bearing, CORE_WORD, Canal, Coast, CoastShape, CompassRegion, Constraint, District, GateKind,
    Hill, InsideCover, Landmark, OutsideCover, RoadEnd, Rule, Specification, Terrain,
};
use citymap_specification::units::Weight;
use citymap_specification::validation::{
    CANALS, CITADEL, CROSSINGS, DISTRICTS, GATES, HARBOUR, LAKES, LAND_COVER, LANDMARKS,
    MAXIMUM_DISTRICTS, MAXIMUM_LANDMARKS, OPEN_SPACE, RIVERS, ROADS, TERRAIN, ValidationError,
    ValidationWarning, WALLS, check_constraints, validate, warnings,
};

fn only_error(specification: &Specification) -> ValidationError {
    let errors = validate(specification).expect_err("the specification breaks a rule");
    assert_eq!(errors.len(), 1, "{errors:?}");
    errors[0].clone()
}

#[test]
fn the_valid_example_passes() {
    assert_eq!(validate(&valid()), Ok(()));
}

#[test]
fn an_unknown_schema_is_an_error() {
    let mut specification = valid();
    specification.schema = 2;
    assert_eq!(
        only_error(&specification),
        ValidationError::UnknownSchema { found: 2 }
    );
}

#[test]
fn a_name_is_one_to_eighty_characters() {
    for (name, characters) in [(String::new(), 0), ("a".repeat(81), 81)] {
        let mut specification = valid();
        specification.identity.name = name;
        assert_eq!(
            only_error(&specification),
            ValidationError::NameLength { characters }
        );
    }
}

#[test]
fn population_is_one_to_a_million() {
    for population in [0, 1_000_001] {
        let mut specification = valid();
        specification.settlement.population = population;
        assert_eq!(
            only_error(&specification),
            ValidationError::PopulationOutOfRange { population }
        );
    }
}

#[test]
fn extent_is_five_hundred_to_five_thousand_metres() {
    for extent in [499.99, 5000.01] {
        let mut specification = valid();
        specification.site.extent_metres = length(extent);
        assert!(matches!(
            only_error(&specification),
            ValidationError::ExtentOutOfRange { .. }
        ));
    }
}

#[test]
fn an_identifier_used_by_a_river_and_a_road_is_a_duplicate() {
    let mut specification = valid();
    specification.roads[0].identifier = "ash".to_string();
    assert_eq!(
        only_error(&specification),
        ValidationError::DuplicateIdentifier {
            list: ROADS,
            identifier: "ash".to_string()
        }
    );
}

#[test]
fn reserved_words_and_dots_are_not_identifiers() {
    for identifier in ["core", "centre", "north_east", "north_road.ash"] {
        let mut specification = valid();
        specification.roads[0].identifier = identifier.to_string();
        assert_eq!(
            only_error(&specification),
            ValidationError::ReservedIdentifier {
                list: ROADS,
                identifier: identifier.to_string()
            }
        );
    }
}

#[test]
fn a_river_is_wider_than_zero() {
    let mut specification = valid();
    specification.site.water.rivers[0].width_metres = length(0.0);
    assert_eq!(
        only_error(&specification),
        ValidationError::ZeroWaterwayWidth {
            list: RIVERS,
            waterway: "ash".to_string()
        }
    );
}

#[test]
fn a_river_does_not_end_at_the_centre() {
    let mut specification = valid();
    specification.site.water.rivers[0].to = CompassRegion::Centre;
    assert_eq!(
        only_error(&specification),
        ValidationError::WaterwayEndAtCentre {
            list: RIVERS,
            waterway: "ash".to_string()
        }
    );
}

#[test]
fn a_river_leaves_at_another_stretch() {
    let mut specification = valid();
    specification.site.water.rivers[0].to = CompassRegion::NorthWest;
    assert_eq!(
        only_error(&specification),
        ValidationError::WaterwayEndsEqual {
            list: RIVERS,
            waterway: "ash".to_string()
        }
    );
}

#[test]
fn a_city_needs_a_road() {
    let mut specification = valid();
    specification.roads.clear();
    assert_eq!(only_error(&specification), ValidationError::NoRoads);
}

#[test]
fn a_road_has_at_most_one_core_end() {
    let mut specification = valid();
    specification.roads[1].ends = [RoadEnd::Core, RoadEnd::Core];
    assert_eq!(
        only_error(&specification),
        ValidationError::BothRoadEndsCore {
            road: "west_road".to_string()
        }
    );
}

#[test]
fn a_road_has_two_different_ends() {
    let mut specification = valid();
    specification.roads[0].ends[1] = RoadEnd::Region(CompassRegion::North);
    assert_eq!(
        only_error(&specification),
        ValidationError::RoadEndsEqual {
            road: "north_road".to_string()
        }
    );
}

#[test]
fn a_road_does_not_end_at_the_centre() {
    let mut specification = valid();
    specification.roads[1].ends[1] = RoadEnd::Region(CompassRegion::Centre);
    assert_eq!(
        only_error(&specification),
        ValidationError::RoadEndAtCentre {
            road: "west_road".to_string()
        }
    );
}

#[test]
fn a_road_end_at_a_landmark_names_the_road() {
    let mut specification = valid();
    specification.roads[1].ends[1] = RoadEnd::Place("guildhall".to_string());
    let error = only_error(&specification);
    assert_eq!(
        error,
        ValidationError::RoadEndAtPlace {
            road: "west_road".to_string(),
            place: "guildhall".to_string()
        }
    );
    assert!(error.to_string().contains("west_road"), "{error}");
}

#[test]
fn a_duplicate_within_one_list_is_reported_once() {
    let mut specification = valid();
    let copy = specification.roads[0].clone();
    specification.roads.push(copy.clone());
    specification.roads.push(copy);
    assert_eq!(
        only_error(&specification),
        ValidationError::DuplicateIdentifier {
            list: ROADS,
            identifier: "north_road".to_string()
        }
    );
}

#[test]
fn two_equal_place_ends_are_one_equal_ends_error_and_one_place_error() {
    let mut specification = valid();
    let guildhall = RoadEnd::Place("guildhall".to_string());
    specification.roads[1].ends = [guildhall.clone(), guildhall];
    let errors = validate(&specification).expect_err("two rules are broken");
    assert_eq!(errors.len(), 2, "{errors:?}");
}

type BreakTerrain = fn(&mut Terrain);

type BreakCanal = fn(&mut Canal);

fn zero_size_field(field: &str) -> ValidationError {
    ValidationError::ZeroSize {
        field: format!("{TERRAIN}.{field}"),
    }
}

#[test]
fn a_site_with_every_terrain_feature_passes() {
    assert_eq!(validate(&with_terrain()), Ok(()));
}

#[test]
fn a_terrain_size_of_zero_is_an_error() {
    let cases: [(BreakTerrain, &str); 8] = [
        (
            |terrain| terrain.cliffs[0].height_metres = length(0.0),
            "cliffs[0].height_metres",
        ),
        (
            |terrain| terrain.hills[0].radius_metres = length(0.0),
            "hills[0].radius_metres",
        ),
        (
            |terrain| terrain.hills[0].height_metres = length(0.0),
            "hills[0].height_metres",
        ),
        (
            |terrain| terrain.hollows[0].radius_metres = length(0.0),
            "hollows[0].radius_metres",
        ),
        (
            |terrain| terrain.hollows[0].depth_metres = length(0.0),
            "hollows[0].depth_metres",
        ),
        (
            |terrain| {
                if let Some(slope) = &mut terrain.slope {
                    slope.metres_per_kilometre = gradient(0.0);
                }
            },
            "slope.metres_per_kilometre",
        ),
        (
            |terrain| {
                if let Some(plateau) = &mut terrain.plateau {
                    plateau.height_metres = length(0.0);
                }
            },
            "plateau.height_metres",
        ),
        (
            |terrain| {
                terrain.hills.push(Hill {
                    at: CompassRegion::South,
                    radius_metres: length(1.0),
                    height_metres: length(0.0),
                });
            },
            "hills[1].height_metres",
        ),
    ];
    for (break_it, field) in cases {
        let mut specification = with_terrain();
        break_it(&mut specification.site.terrain);
        assert_eq!(only_error(&specification), zero_size_field(field));
    }
}

#[test]
fn a_bearing_above_359_degrees_is_an_error() {
    let mut specification = with_terrain();
    specification.site.terrain.cliffs[0].bearing_degrees = Bearing::Degrees(360);
    assert_eq!(
        only_error(&specification),
        ValidationError::BearingOutOfRange {
            field: format!("{TERRAIN}.cliffs[0].bearing_degrees"),
            degrees: 360,
        }
    );
}

#[test]
fn a_bearing_of_centre_is_an_error() {
    let mut specification = with_terrain();
    if let Some(slope) = &mut specification.site.terrain.slope {
        slope.toward_degrees = Bearing::Word(CompassRegion::Centre);
    }
    assert_eq!(
        only_error(&specification),
        ValidationError::BearingAtCentre {
            field: format!("{TERRAIN}.slope.toward_degrees"),
        }
    );
}

#[test]
fn an_estuary_needs_a_river_that_reaches_its_coast() {
    let mut specification = with_terrain();
    if let Some(coast) = &mut specification.site.terrain.coast {
        coast.bearing_degrees = Bearing::Degrees(136);
    }
    assert_eq!(
        only_error(&specification),
        ValidationError::EstuaryWithoutRiver
    );
}

#[test]
fn a_coast_that_is_no_estuary_needs_no_river() {
    let mut specification = with_terrain();
    if let Some(coast) = &mut specification.site.terrain.coast {
        coast.bearing_degrees = Bearing::Degrees(136);
        coast.shape = CoastShape::Bay;
    }
    assert_eq!(validate(&specification), Ok(()));
}

#[test]
fn a_river_within_45_degrees_of_the_estuary_reaches_it() {
    let mut specification = with_terrain();
    if let Some(coast) = &mut specification.site.terrain.coast {
        coast.bearing_degrees = Bearing::Degrees(135);
    }
    assert_eq!(validate(&specification), Ok(()));
}

#[test]
fn an_estuary_across_north_measures_the_short_way_round() {
    let mut specification = with_terrain();
    specification.site.water.rivers[0].to = CompassRegion::North;
    if let Some(coast) = &mut specification.site.terrain.coast {
        coast.bearing_degrees = Bearing::Degrees(350);
    }
    assert_eq!(validate(&specification), Ok(()));
}

#[test]
fn an_estuary_with_a_broken_bearing_reports_only_the_bearing() {
    let mut specification = with_terrain();
    if let Some(coast) = &mut specification.site.terrain.coast {
        coast.bearing_degrees = Bearing::Word(CompassRegion::Centre);
    }
    assert_eq!(
        only_error(&specification),
        ValidationError::BearingAtCentre {
            field: format!("{TERRAIN}.coast.bearing_degrees"),
        }
    );
}

#[test]
fn a_site_with_every_kind_of_water_passes() {
    assert_eq!(validate(&with_water()), Ok(()));
}

#[test]
fn a_lake_radius_of_zero_is_an_error() {
    let mut specification = with_water();
    specification.site.water.lakes[0].radius_metres = length(0.0);
    assert_eq!(
        only_error(&specification),
        ValidationError::ZeroSize {
            field: format!("{LAKES}[0].radius_metres"),
        }
    );
}

#[test]
fn a_harbour_length_of_zero_is_an_error() {
    let mut specification = with_water();
    if let Some(harbour) = &mut specification.site.water.harbour {
        harbour.length_metres = length(0.0);
    }
    assert_eq!(
        only_error(&specification),
        ValidationError::ZeroSize {
            field: format!("{HARBOUR}.length_metres"),
        }
    );
}

#[test]
fn a_harbour_needs_a_coast_or_a_river_end_near_its_bearing() {
    assert_eq!(
        only_error(&harbour_at(200)),
        ValidationError::HarbourWithoutWater {
            harbour: "quay".to_string(),
        }
    );
}

#[test]
fn a_river_end_within_45_degrees_serves_the_harbour() {
    assert_eq!(validate(&harbour_at(270)), Ok(()));
    assert_eq!(validate(&harbour_at(135)), Ok(()));
    assert!(validate(&harbour_at(136)).is_err());
}

#[test]
fn a_coast_within_45_degrees_serves_the_harbour() {
    let mut specification = harbour_at(200);
    specification.site.terrain.coast = Some(Coast {
        bearing_degrees: Bearing::Word(CompassRegion::South),
        shape: CoastShape::Open,
        shore: None,
    });
    assert_eq!(validate(&specification), Ok(()));
}

#[test]
fn a_harbour_bearing_above_359_is_reported_once() {
    assert_eq!(
        only_error(&harbour_at(360)),
        ValidationError::BearingOutOfRange {
            field: format!("{HARBOUR}.bearing_degrees"),
            degrees: 360,
        }
    );
}

#[test]
fn a_broken_coast_bearing_does_not_also_fail_the_harbour() {
    let mut specification = harbour_at(200);
    specification.site.terrain.coast = Some(Coast {
        bearing_degrees: Bearing::Degrees(400),
        shape: CoastShape::Open,
        shore: None,
    });
    assert_eq!(
        only_error(&specification),
        ValidationError::BearingOutOfRange {
            field: format!("{TERRAIN}.coast.bearing_degrees"),
            degrees: 400,
        }
    );
}

#[test]
fn a_canal_gets_the_checks_of_a_river() {
    let cases: [(BreakCanal, ValidationError); 3] = [
        (
            |canal| canal.width_metres = length(0.0),
            ValidationError::ZeroWaterwayWidth {
                list: CANALS,
                waterway: "cut".to_string(),
            },
        ),
        (
            |canal| canal.to = CompassRegion::Centre,
            ValidationError::WaterwayEndAtCentre {
                list: CANALS,
                waterway: "cut".to_string(),
            },
        ),
        (
            |canal| canal.to = CompassRegion::South,
            ValidationError::WaterwayEndsEqual {
                list: CANALS,
                waterway: "cut".to_string(),
            },
        ),
    ];
    for (break_it, error) in cases {
        let mut specification = with_water();
        break_it(&mut specification.site.water.canals[0]);
        assert_eq!(only_error(&specification), error);
    }
}

#[test]
fn a_lake_and_a_canal_share_no_identifier() {
    let mut specification = with_water();
    specification.site.water.canals[0].identifier = "mere".to_string();
    assert_eq!(
        only_error(&specification),
        ValidationError::DuplicateIdentifier {
            list: CANALS,
            identifier: "mere".to_string(),
        }
    );
}

#[test]
fn a_harbour_identifier_is_checked_for_reserved_words() {
    let mut specification = with_water();
    if let Some(harbour) = &mut specification.site.water.harbour {
        harbour.identifier = "core".to_string();
    }
    assert_eq!(
        only_error(&specification),
        ValidationError::ReservedIdentifier {
            list: HARBOUR,
            identifier: "core".to_string(),
        }
    );
}

#[test]
fn land_cover_with_names_and_a_band_passes() {
    assert_eq!(validate(&with_land_cover()), Ok(()));
}

#[test]
fn a_cover_listed_twice_is_an_error_at_the_repeat() {
    let mut specification = with_land_cover();
    specification
        .site
        .land_cover
        .outside
        .push(OutsideCover::Farmland);
    specification
        .site
        .land_cover
        .inside
        .push(InsideCover::Gardens);
    let errors = validate(&specification).expect_err("two lists repeat a cover");
    assert_eq!(
        errors,
        [
            ValidationError::DuplicateCover {
                field: format!("{LAND_COVER}.outside[2]"),
            },
            ValidationError::DuplicateCover {
                field: format!("{LAND_COVER}.inside[1]"),
            },
        ]
    );
}

#[test]
fn a_band_needs_depth_and_a_direction() {
    let mut specification = with_land_cover();
    specification.site.land_cover.bands[0].depth_metres = length(0.0);
    specification.site.land_cover.bands[0].bearing_degrees = Bearing::Word(CompassRegion::Centre);
    let errors = validate(&specification).expect_err("the band breaks two rules");
    assert_eq!(
        errors,
        [
            ValidationError::BearingAtCentre {
                field: format!("{LAND_COVER}.bands[0].bearing_degrees"),
            },
            ValidationError::ZeroSize {
                field: format!("{LAND_COVER}.bands[0].depth_metres"),
            },
        ]
    );
}

fn constraint_errors(constraints: &[Constraint]) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    check_constraints(constraints, "districts[0]", &mut errors);
    errors
}

#[test]
fn rules_within_their_ranges_pass() {
    let constraints = [
        compass_region(Bearing::Word(CompassRegion::NorthEast), None),
        compass_region(Bearing::Word(CompassRegion::Centre), None),
        rule(
            Rule::NotInCompassRegion {
                region: Bearing::Degrees(115),
                width_degrees: Some(20),
            },
            weight(2.0),
        ),
        rule(
            Rule::Near {
                target: "market".to_string(),
                within_metres: length(0.0),
            },
            Weight::DEFAULT,
        ),
    ];
    assert_eq!(constraint_errors(&constraints), []);
}

#[test]
fn a_weight_of_zero_is_an_error() {
    let constraints = [rule(Rule::OnWater, weight(0.0))];
    assert_eq!(
        constraint_errors(&constraints),
        [ValidationError::ZeroSize {
            field: "districts[0].constraints[0].weight".to_string(),
        }]
    );
}

#[test]
fn a_compass_region_bearing_above_359_is_an_error() {
    assert_eq!(
        constraint_errors(&[compass_region(Bearing::Degrees(400), None)]),
        [ValidationError::BearingOutOfRange {
            field: "districts[0].constraints[0].region".to_string(),
            degrees: 400,
        }]
    );
}

#[test]
fn a_slice_is_1_to_360_degrees_wide() {
    for degrees in [0, 361] {
        let constraints = [compass_region(Bearing::Degrees(90), Some(degrees))];
        assert_eq!(
            constraint_errors(&constraints),
            [ValidationError::WidthOutOfRange {
                field: "districts[0].constraints[0].width_degrees".to_string(),
                degrees,
            }]
        );
    }
    let whole_circle = [compass_region(Bearing::Degrees(90), Some(360))];
    assert_eq!(constraint_errors(&whole_circle), []);
}

#[test]
fn the_centre_has_no_width() {
    let constraints = [compass_region(
        Bearing::Word(CompassRegion::Centre),
        Some(45),
    )];
    assert_eq!(
        constraint_errors(&constraints),
        [ValidationError::WidthAtCentre {
            field: "districts[0].constraints[0].width_degrees".to_string(),
        }]
    );
}

#[test]
fn a_city_needs_a_district() {
    let mut specification = valid();
    specification.districts.clear();
    assert_eq!(only_error(&specification), ValidationError::NoDistricts);
}

#[test]
fn a_city_has_at_most_100_districts() {
    let mut specification = valid();
    let template = specification.districts[0].clone();
    specification.districts = (0..=MAXIMUM_DISTRICTS)
        .map(|index| District {
            identifier: format!("district_{index}"),
            ..template.clone()
        })
        .collect();
    assert_eq!(
        only_error(&specification),
        ValidationError::TooManyDistricts { count: 101 }
    );
}

#[test]
fn a_district_weight_of_zero_is_an_error() {
    let mut specification = valid();
    specification.districts[0].weight = weight(0.0);
    assert_eq!(
        only_error(&specification),
        ValidationError::ZeroSize {
            field: "districts[0].weight".to_string(),
        }
    );
}

#[test]
fn a_district_cannot_have_a_wall_rule() {
    let mut specification = valid();
    let target = || "outer".to_string();
    specification.districts[0].constraints = [
        Rule::InsideWalls { target: target() },
        Rule::OutsideWalls { target: target() },
        Rule::MinSpacing {
            metres: length(20.0),
        },
    ]
    .into_iter()
    .map(|rule| Constraint {
        rule,
        hard: true,
        weight: Weight::DEFAULT,
    })
    .collect();
    let field = |index: usize| ValidationError::DistrictWallRule {
        field: format!("districts[0].constraints[{index}]"),
    };
    assert_eq!(validate(&specification), Err(vec![field(0), field(1)]));
}

#[test]
fn a_district_rule_is_checked_like_every_rule() {
    let mut specification = valid();
    specification.districts[0].constraints = vec![Constraint {
        rule: Rule::OnWater,
        hard: false,
        weight: weight(0.0),
    }];
    assert_eq!(
        only_error(&specification),
        ValidationError::ZeroSize {
            field: "districts[0].constraints[0].weight".to_string(),
        }
    );
}

#[test]
fn a_district_identifier_is_checked_for_reserved_words() {
    let mut specification = valid();
    specification.districts[0].identifier = "north".to_string();
    assert_eq!(
        only_error(&specification),
        ValidationError::ReservedIdentifier {
            list: DISTRICTS,
            identifier: "north".to_string(),
        }
    );
}

#[test]
fn a_district_and_a_river_share_no_identifier() {
    let mut specification = valid();
    specification.districts[0].identifier = "ash".to_string();
    assert_eq!(
        only_error(&specification),
        ValidationError::DuplicateIdentifier {
            list: DISTRICTS,
            identifier: "ash".to_string(),
        }
    );
}

#[test]
fn a_pinned_landmark_passes() {
    assert_eq!(validate(&with_landmark(guildhall(true))), Ok(()));
}

#[test]
fn pinned_without_a_placement_is_an_error() {
    let mut landmark = guildhall(true);
    landmark.resolved = None;
    assert_eq!(
        only_error(&with_landmark(landmark)),
        ValidationError::PinnedWithoutPlacement {
            field: "landmarks[0]".to_string(),
        }
    );
}

fn outside(field: &str, x: &str, y: &str) -> ValidationError {
    ValidationError::PinnedOutsideSite {
        field: field.to_string(),
        x: x.to_string(),
        y: y.to_string(),
        extent: "2400".to_string(),
    }
}

#[test]
fn a_pinned_landmark_has_its_centre_inside_the_site() {
    let mut landmark = guildhall(true);
    let mut placement = placement(90.0);
    placement.x = length(2400.01);
    landmark.resolved = Some(placement);
    assert_eq!(
        only_error(&with_landmark(landmark)),
        outside("landmarks[0].resolved", "2400.01", "1300")
    );
}

#[test]
fn a_pinned_open_space_has_its_centre_inside_the_site() {
    let mut space = market(true);
    let mut placement = placement(0.0);
    placement.y = length(3000.0);
    space.resolved = Some(placement);
    assert_eq!(
        only_error(&with_market_and_bridge(space, bridge("ash"))),
        outside("open_space[0].resolved", "1200", "3000")
    );
}

#[test]
fn a_pinned_crossing_has_its_centre_inside_the_site() {
    let mut crossing = bridge("ash");
    let mut placement = placement(0.0);
    placement.x = length(5000.0);
    crossing.resolved = Some(placement);
    crossing.pinned = true;
    assert_eq!(
        only_error(&with_market_and_bridge(market(false), crossing)),
        outside("crossings[0].resolved", "5000", "1300")
    );
}

#[test]
fn a_centre_on_the_edge_of_the_site_is_inside() {
    let mut landmark = guildhall(true);
    let mut placement = placement(90.0);
    placement.x = length(2400.0);
    placement.y = length(0.0);
    landmark.resolved = Some(placement.clone());
    assert_eq!(validate(&with_landmark(landmark.clone())), Ok(()));
    placement.x = length(0.0);
    placement.y = length(2400.0);
    landmark.resolved = Some(placement);
    assert_eq!(validate(&with_landmark(landmark)), Ok(()));
}

#[test]
fn a_centre_outside_on_both_axes_is_one_error() {
    let mut landmark = guildhall(true);
    let mut placement = placement(90.0);
    placement.x = length(2500.0);
    placement.y = length(2600.0);
    landmark.resolved = Some(placement);
    assert_eq!(
        only_error(&with_landmark(landmark)),
        outside("landmarks[0].resolved", "2500", "2600")
    );
}

#[test]
fn an_unpinned_placement_outside_the_site_is_only_a_hint() {
    let mut landmark = guildhall(false);
    let mut placement = placement(90.0);
    placement.x = length(9000.0);
    landmark.resolved = Some(placement);
    assert_eq!(validate(&with_landmark(landmark)), Ok(()));
}

#[test]
fn a_turn_is_from_0_up_to_360_degrees() {
    for degrees in [-0.01, 360.0] {
        let mut landmark = guildhall(true);
        landmark.resolved = Some(placement(degrees));
        assert_eq!(
            only_error(&with_landmark(landmark)),
            ValidationError::RotationOutOfRange {
                field: "landmarks[0].resolved.rotation_degrees".to_string(),
                degrees: degrees.to_string(),
            }
        );
    }
    let mut landmark = guildhall(true);
    landmark.resolved = Some(placement(359.99));
    assert_eq!(validate(&with_landmark(landmark)), Ok(()));
}

#[test]
fn a_landmark_size_of_zero_is_an_error() {
    let mut landmark = guildhall(true);
    if let Some(footprint) = &mut landmark.footprint {
        footprint.depth_metres = Some(length(0.0));
    }
    if let Some(resolved) = &mut landmark.resolved {
        resolved.width_metres = length(0.0);
    }
    let errors = validate(&with_landmark(landmark)).expect_err("two sizes are 0");
    assert_eq!(
        errors,
        [
            ValidationError::ZeroSize {
                field: "landmarks[0].footprint.depth_metres".to_string(),
            },
            ValidationError::ZeroSize {
                field: "landmarks[0].resolved.width_metres".to_string(),
            },
        ]
    );
}

#[test]
fn a_city_has_at_most_1000_landmarks() {
    let mut specification = valid();
    specification.landmarks = (0..=MAXIMUM_LANDMARKS)
        .map(|index| Landmark {
            identifier: format!("landmark_{index}"),
            ..guildhall(false)
        })
        .collect();
    assert_eq!(
        only_error(&specification),
        ValidationError::TooManyLandmarks { count: 1001 }
    );
}

#[test]
fn a_road_may_end_at_a_pinned_landmark() {
    let mut specification = with_landmark(guildhall(true));
    specification.roads[1].ends = [RoadEnd::Core, RoadEnd::Place("guildhall".to_string())];
    assert_eq!(validate(&specification), Ok(()));
}

#[test]
fn a_road_to_a_landmark_that_is_not_pinned_says_pin_it_first() {
    let mut specification = with_landmark(guildhall(false));
    specification.roads[1].ends = [RoadEnd::Core, RoadEnd::Place("guildhall".to_string())];
    let error = only_error(&specification);
    assert!(error.to_string().contains("pin it first"), "{error}");
    assert_eq!(
        error,
        ValidationError::RoadEndNotPinned {
            road: "west_road".to_string(),
            place: "guildhall".to_string(),
        }
    );
}

#[test]
fn a_zero_preset_size_is_an_error() {
    for key in [
        "reuse_within_metres",
        "reuse_weight",
        "centre_radius_metres",
    ] {
        let mut specification = valid();
        let presets = &mut specification.presets;
        match key {
            "reuse_within_metres" => presets.reuse_within_metres = length(0.0),
            "reuse_weight" => presets.reuse_weight = Weight::new(0.0).expect("0 is a weight"),
            _ => presets.centre_radius_metres = length(0.0),
        }
        assert_eq!(
            only_error(&specification),
            ValidationError::ZeroSize {
                field: format!("presets.{key}"),
            }
        );
    }
}

#[test]
fn an_unknown_style_or_pack_is_a_warning_not_an_error() {
    let mut specification = valid();
    specification.presets.style = "inc".to_string();
    specification.presets.packs = vec!["jotun".to_string()];
    assert_eq!(validate(&specification), Ok(()));
    assert_eq!(
        warnings(&specification),
        [
            ValidationWarning::UnknownStyle {
                style: "inc".to_string(),
            },
            ValidationWarning::UnknownPack {
                field: "presets.packs[0]".to_string(),
                pack: "jotun".to_string(),
            },
        ]
    );
    assert_eq!(warnings(&valid()), []);
}

#[test]
fn a_population_outside_the_size_law_is_a_warning_not_an_error() {
    for population in [999, 200_001] {
        let mut specification = valid();
        specification.settlement.population = population;
        assert_eq!(validate(&specification), Ok(()));
        assert_eq!(
            warnings(&specification),
            [ValidationWarning::PopulationExtrapolated { population }]
        );
    }
    for population in [1_000, 200_000] {
        let mut specification = valid();
        specification.settlement.population = population;
        assert_eq!(warnings(&specification), []);
    }
}

#[test]
fn a_population_that_breaks_the_rule_gives_no_warning() {
    let mut specification = valid();
    specification.settlement.population = 0;
    assert_eq!(warnings(&specification), []);
}

#[test]
fn an_unknown_label_is_a_warning_not_an_error() {
    let mut landmark = guildhall(true);
    landmark.label = Some("cathdral".to_string());
    let specification = with_landmark(landmark);
    assert_eq!(validate(&specification), Ok(()));
    assert_eq!(
        warnings(&specification),
        [ValidationWarning::UnknownLabel {
            field: "landmarks[0].label".to_string(),
            label: "cathdral".to_string(),
        }]
    );
    assert_eq!(warnings(&with_landmark(guildhall(true))), []);
}

#[test]
fn a_landmark_and_a_district_share_no_identifier() {
    let mut landmark = guildhall(true);
    landmark.identifier = "old_town".to_string();
    assert_eq!(
        only_error(&with_landmark(landmark)),
        ValidationError::DuplicateIdentifier {
            list: LANDMARKS,
            identifier: "old_town".to_string(),
        }
    );
}

#[test]
fn a_pinned_open_space_or_crossing_needs_a_placement() {
    let mut space = market(true);
    space.resolved = None;
    let mut crossing = bridge("ash");
    crossing.pinned = true;
    let errors = validate(&with_market_and_bridge(space, crossing))
        .expect_err("two pins have nothing to keep");
    assert_eq!(
        errors,
        [
            ValidationError::PinnedWithoutPlacement {
                field: "open_space[0]".to_string(),
            },
            ValidationError::PinnedWithoutPlacement {
                field: "crossings[0]".to_string(),
            },
        ]
    );
}

#[test]
fn a_crossing_names_a_river_of_the_file() {
    for river in ["brook", "west_road"] {
        assert_eq!(
            only_error(&with_market_and_bridge(market(true), bridge(river))),
            ValidationError::CrossingWithoutRiver {
                crossing: "ash_bridge".to_string(),
                river: river.to_string(),
            }
        );
    }
}

#[test]
fn open_space_and_crossing_rules_are_checked_like_every_rule() {
    let weightless = || Constraint {
        rule: Rule::OnWater,
        hard: false,
        weight: weight(0.0),
    };
    let mut space = market(true);
    space.constraints.push(weightless());
    let mut crossing = bridge("ash");
    crossing.constraints.push(weightless());
    let zero = |owner: &str| ValidationError::ZeroSize {
        field: format!("{owner}.constraints[0].weight"),
    };
    assert_eq!(
        validate(&with_market_and_bridge(space, crossing)),
        Err(vec![zero("open_space[0]"), zero("crossings[0]")])
    );
}

#[test]
fn a_road_may_end_at_a_pinned_open_space_only() {
    let mut specification = with_market_and_bridge(market(true), bridge("ash"));
    specification.roads[1].ends = [RoadEnd::Core, RoadEnd::Place("market".to_string())];
    assert_eq!(validate(&specification), Ok(()));
    specification.open_space[0].pinned = false;
    assert_eq!(
        only_error(&specification),
        ValidationError::RoadEndNotPinned {
            road: "west_road".to_string(),
            place: "market".to_string(),
        }
    );
}

#[test]
fn an_open_space_and_a_crossing_share_no_identifier() {
    let mut crossing = bridge("ash");
    crossing.identifier = "market".to_string();
    assert_eq!(
        only_error(&with_market_and_bridge(market(true), crossing)),
        ValidationError::DuplicateIdentifier {
            list: CROSSINGS,
            identifier: "market".to_string(),
        }
    );
}

#[test]
fn a_citadel_and_every_gate_kind_pass() {
    let gates = vec![
        gate(
            "north_gate",
            "town_wall",
            road_gate("north_road", Some(RoadEnd::Region(CompassRegion::North))),
        ),
        gate("west_gate", "town_wall", road_gate("west_road", None)),
        gate(
            "sally_port",
            "town_wall",
            GateKind::Postern {
                bearing_degrees: Bearing::Degrees(200),
            },
        ),
        gate(
            "quay_gate",
            "town_wall",
            GateKind::WaterGate {
                bearing_degrees: Bearing::Word(CompassRegion::East),
            },
        ),
    ];
    let citadel = citadel("town_wall", Bearing::Word(CompassRegion::North));
    assert_eq!(validate(&with_defences(Some(citadel), gates)), Ok(()));
}

#[test]
fn a_citadel_is_part_of_a_wall() {
    for wall in ["nowhere", "docks"] {
        let citadel = citadel(wall, Bearing::Degrees(0));
        assert_eq!(
            only_error(&with_defences(Some(citadel), Vec::new())),
            ValidationError::CitadelWithoutWall {
                citadel: "keep".to_string(),
                wall: wall.to_string(),
            }
        );
    }
}

#[test]
fn a_gate_names_a_wall_and_a_road() {
    for wall in ["nowhere", "docks"] {
        let gates = vec![gate("north_gate", wall, road_gate("north_road", None))];
        assert_eq!(
            only_error(&with_defences(None, gates)),
            ValidationError::GateWithoutWall {
                gate: "north_gate".to_string(),
                wall: wall.to_string(),
            }
        );
    }
    for road in ["nowhere", "town_wall"] {
        let gates = vec![gate("north_gate", "town_wall", road_gate(road, None))];
        assert_eq!(
            only_error(&with_defences(None, gates)),
            ValidationError::GateWithoutRoad {
                gate: "north_gate".to_string(),
                road: road.to_string(),
            }
        );
    }
}

#[test]
fn a_road_gate_end_is_one_of_its_road_ends() {
    let west = Some(RoadEnd::Region(CompassRegion::West));
    let gates = vec![gate(
        "north_gate",
        "town_wall",
        road_gate("north_road", west),
    )];
    assert_eq!(
        only_error(&with_defences(None, gates)),
        ValidationError::GateEndNotOnRoad {
            gate: "north_gate".to_string(),
            end: "west".to_string(),
            road: "north_road".to_string(),
        }
    );
}

#[test]
fn citadel_and_gate_bearings_are_checked() {
    let postern = GateKind::Postern {
        bearing_degrees: Bearing::Degrees(400),
    };
    let citadel = citadel("town_wall", Bearing::Word(CompassRegion::Centre));
    assert_eq!(
        validate(&with_defences(
            Some(citadel),
            vec![gate("sally_port", "town_wall", postern)]
        )),
        Err(vec![
            ValidationError::BearingAtCentre {
                field: "defences.citadel.bearing_degrees".to_string(),
            },
            ValidationError::BearingOutOfRange {
                field: "defences.gates[0].bearing_degrees".to_string(),
                degrees: 400,
            },
        ])
    );
}

#[test]
fn a_gate_and_a_wall_share_no_identifier() {
    let gates = vec![gate(
        "town_wall",
        "town_wall",
        road_gate("north_road", None),
    )];
    assert_eq!(
        only_error(&with_defences(None, gates)),
        ValidationError::DuplicateIdentifier {
            list: GATES,
            identifier: "town_wall".to_string(),
        }
    );
}

#[test]
fn every_citadel_and_gate_message_starts_with_its_list() {
    let name = || "x".to_string();
    let citadel = ValidationError::CitadelWithoutWall {
        citadel: name(),
        wall: name(),
    };
    assert!(citadel.to_string().starts_with(CITADEL), "{citadel}");
    for error in [
        ValidationError::GateWithoutWall {
            gate: name(),
            wall: name(),
        },
        ValidationError::GateWithoutRoad {
            gate: name(),
            road: name(),
        },
        ValidationError::GateEndNotOnRoad {
            gate: name(),
            end: name(),
            road: name(),
        },
    ] {
        assert!(error.to_string().starts_with(GATES), "{error}");
    }
}

#[test]
fn nested_walls_pass() {
    let mut inner = wall("inner", &["old_town"]);
    inner.separate_ring = true;
    inner.gap_metres = Some(length(25.0));
    let walls = vec![wall("outer", &["inner", "old_town", "docks"]), inner];
    assert_eq!(validate(&with_walls(walls)), Ok(()));
}

#[test]
fn encloses_names_a_district_or_a_wall() {
    for entry in ["nowhere", "ash"] {
        assert_eq!(
            only_error(&with_walls(vec![wall("outer", &[entry])])),
            ValidationError::EnclosesNeitherDistrictNorWall {
                wall: "outer".to_string(),
                entry: entry.to_string(),
            }
        );
    }
}

#[test]
fn a_wall_that_encloses_itself_is_a_ring() {
    let ring = |names: &[&str]| ValidationError::WallRing {
        walls: names.iter().map(ToString::to_string).collect(),
    };
    assert_eq!(
        only_error(&with_walls(vec![wall("outer", &["outer"])])),
        ring(&["outer", "outer"])
    );
    let walls = vec![
        wall("first", &["second"]),
        wall("second", &["third"]),
        wall("third", &["first", "docks"]),
    ];
    assert_eq!(
        only_error(&with_walls(walls)),
        ring(&["first", "second", "third", "first"])
    );
}

#[test]
fn a_district_under_two_separate_walls_is_an_error() {
    let walls = vec![
        wall("harbour_wall", &["docks"]),
        wall("land_wall", &["docks", "old_town"]),
    ];
    assert_eq!(
        only_error(&with_walls(walls)),
        ValidationError::UnderTwoWalls {
            entry: "docks".to_string(),
            first: "harbour_wall".to_string(),
            second: "land_wall".to_string(),
        }
    );
}

#[test]
fn a_wall_under_two_separate_walls_is_an_error() {
    let walls = vec![
        wall("harbour_wall", &["inner"]),
        wall("land_wall", &["inner"]),
        wall("inner", &["docks"]),
    ];
    assert_eq!(
        only_error(&with_walls(walls)),
        ValidationError::UnderTwoWalls {
            entry: "inner".to_string(),
            first: "harbour_wall".to_string(),
            second: "land_wall".to_string(),
        }
    );
}

#[test]
fn a_district_under_walls_three_levels_apart_passes() {
    let walls = vec![
        wall("outer", &["middle", "docks"]),
        wall("middle", &["inner"]),
        wall("inner", &["docks"]),
    ];
    assert_eq!(validate(&with_walls(walls)), Ok(()));
}

#[test]
fn a_name_listed_twice_or_unknown_is_reported_once() {
    assert_eq!(
        validate(&with_walls(vec![wall("outer", &["docks", "docks"])])),
        Ok(())
    );
    let walls = vec![wall("first", &["nowhere"]), wall("second", &["nowhere"])];
    let errors = validate(&with_walls(walls)).expect_err("`nowhere` is no district");
    assert_eq!(errors.len(), 2, "{errors:?}");
}

#[test]
fn a_ring_is_found_from_a_wall_that_leads_into_it() {
    let walls = vec![
        wall("gatehouse", &["first"]),
        wall("first", &["second"]),
        wall("second", &["first"]),
    ];
    assert_eq!(
        only_error(&with_walls(walls)),
        ValidationError::WallRing {
            walls: vec![
                "first".to_string(),
                "second".to_string(),
                "first".to_string()
            ],
        }
    );
}

#[test]
fn walls_in_a_ring_are_not_also_checked_for_two_walls() {
    let walls = vec![
        wall("first", &["second", "docks"]),
        wall("second", &["first"]),
        wall("third", &["docks"]),
    ];
    let errors = validate(&with_walls(walls)).expect_err("the walls form a ring");
    assert!(
        errors
            .iter()
            .all(|error| matches!(error, ValidationError::WallRing { .. })),
        "{errors:?}"
    );
}

#[test]
fn a_gap_needs_a_separate_ring_and_more_than_0() {
    let mut gap_only = wall("outer", &["docks"]);
    gap_only.gap_metres = Some(length(20.0));
    assert_eq!(
        only_error(&with_walls(vec![gap_only])),
        ValidationError::GapWithoutSeparateRing {
            wall: "outer".to_string(),
        }
    );
    let mut zero_gap = wall("outer", &["docks"]);
    zero_gap.separate_ring = true;
    zero_gap.gap_metres = Some(length(0.0));
    assert_eq!(
        only_error(&with_walls(vec![zero_gap])),
        ValidationError::ZeroSize {
            field: "defences.walls[0].gap_metres".to_string(),
        }
    );
}

#[test]
fn a_wall_and_a_district_share_no_identifier() {
    assert_eq!(
        only_error(&with_walls(vec![wall("docks", &["old_town"])])),
        ValidationError::DuplicateIdentifier {
            list: WALLS,
            identifier: "docks".to_string(),
        }
    );
}

#[test]
fn every_wall_message_starts_with_its_list() {
    let name = || "x".to_string();
    for error in [
        ValidationError::EnclosesNeitherDistrictNorWall {
            wall: name(),
            entry: name(),
        },
        ValidationError::WallRing {
            walls: vec![name(), name()],
        },
        ValidationError::GapWithoutSeparateRing { wall: name() },
        ValidationError::UnderTwoWalls {
            entry: name(),
            first: name(),
            second: name(),
        },
    ] {
        assert!(error.to_string().starts_with(WALLS), "{error}");
    }
}

#[test]
fn every_placed_object_message_starts_with_its_list() {
    let name = || "x".to_string();
    let crossing = ValidationError::CrossingWithoutRiver {
        crossing: name(),
        river: name(),
    };
    assert!(crossing.to_string().starts_with(CROSSINGS), "{crossing}");
    let road = ValidationError::RoadEndNotPinned {
        road: name(),
        place: name(),
    };
    assert!(road.to_string().starts_with(ROADS), "{road}");
    let ring = ValidationError::DependencyRing {
        list: OPEN_SPACE,
        objects: vec![name(), name()],
        arrows: vec![name()],
    };
    assert!(ring.to_string().starts_with(OPEN_SPACE), "{ring}");
}

#[test]
fn every_constraint_message_starts_with_its_field() {
    let width = || "districts[0].constraints[0].width_degrees".to_string();
    for error in [
        ValidationError::WidthOutOfRange {
            field: width(),
            degrees: 0,
        },
        ValidationError::WidthAtCentre { field: width() },
        ValidationError::DistrictWallRule { field: width() },
        ValidationError::PinnedWithoutPlacement { field: width() },
        ValidationError::RotationOutOfRange {
            field: width(),
            degrees: "360".to_string(),
        },
        ValidationError::ZeroSize { field: width() },
        ValidationError::TargetMissing {
            field: width(),
            target: width(),
        },
        ValidationError::TargetOfWrongKind {
            field: width(),
            target: width(),
            expected: "wall",
        },
        ValidationError::DistrictRuleToDefence {
            field: width(),
            target: width(),
        },
    ] {
        assert!(error.to_string().starts_with(&width()), "{error}");
    }
}

#[test]
fn every_message_starts_with_its_field() {
    let road = || "r".to_string();
    let cases = [
        (ValidationError::UnknownSchema { found: 2 }, "schema"),
        (
            ValidationError::NameLength { characters: 0 },
            "identity.name",
        ),
        (
            ValidationError::PopulationOutOfRange { population: 0 },
            "settlement.population",
        ),
        (
            ValidationError::ExtentOutOfRange {
                extent: "1".to_string(),
            },
            "site.extent_metres",
        ),
        (
            ValidationError::DuplicateIdentifier {
                list: RIVERS,
                identifier: road(),
            },
            RIVERS,
        ),
        (
            ValidationError::ReservedIdentifier {
                list: ROADS,
                identifier: road(),
            },
            ROADS,
        ),
        (
            ValidationError::ZeroWaterwayWidth {
                list: RIVERS,
                waterway: road(),
            },
            RIVERS,
        ),
        (
            ValidationError::WaterwayEndAtCentre {
                list: CANALS,
                waterway: road(),
            },
            CANALS,
        ),
        (
            ValidationError::WaterwayEndsEqual {
                list: CANALS,
                waterway: road(),
            },
            CANALS,
        ),
        (
            ValidationError::HarbourWithoutWater { harbour: road() },
            HARBOUR,
        ),
        (
            ValidationError::ZeroSize {
                field: format!("{TERRAIN}.hills[0].radius_metres"),
            },
            TERRAIN,
        ),
        (
            ValidationError::BearingOutOfRange {
                field: format!("{TERRAIN}.plateau.bearing_degrees"),
                degrees: 360,
            },
            TERRAIN,
        ),
        (
            ValidationError::BearingAtCentre {
                field: format!("{TERRAIN}.coast.bearing_degrees"),
            },
            TERRAIN,
        ),
        (ValidationError::EstuaryWithoutRiver, TERRAIN),
        (
            ValidationError::DuplicateCover {
                field: format!("{LAND_COVER}.inside[1]"),
            },
            LAND_COVER,
        ),
        (ValidationError::NoRoads, ROADS),
        (ValidationError::BothRoadEndsCore { road: road() }, ROADS),
        (ValidationError::RoadEndsEqual { road: road() }, ROADS),
        (ValidationError::RoadEndAtCentre { road: road() }, ROADS),
        (
            ValidationError::RoadEndAtPlace {
                road: road(),
                place: road(),
            },
            ROADS,
        ),
    ];
    for (error, field) in cases {
        assert!(error.to_string().starts_with(field), "{error}");
    }
}

#[test]
fn a_district_may_wait_for_an_open_space_that_waits_for_districts() {
    let mut space = market(false);
    space.constraints = vec![
        near("harbour_quarter"),
        rule(
            Rule::InDistrict {
                target: "merchants".to_string(),
            },
            Weight::DEFAULT,
        ),
    ];
    let mut specification = with_market_and_bridge(space, bridge("ash"));
    add_district(&mut specification, "craft", vec![near("market")]);
    add_district(&mut specification, "harbour_quarter", Vec::new());
    add_district(
        &mut specification,
        "merchants",
        vec![near("harbour_quarter")],
    );
    assert_eq!(validate(&specification), Ok(()));
}

#[test]
fn a_target_that_names_nothing_is_an_error() {
    let mut specification = valid();
    specification.districts[0].constraints = vec![near("nowhere")];
    assert_eq!(
        only_error(&specification),
        ValidationError::TargetMissing {
            field: "districts[0].constraints[0].target".to_string(),
            target: "nowhere".to_string(),
        }
    );
}

#[test]
fn a_target_of_the_wrong_kind_is_an_error() {
    let mut landmark = guildhall(false);
    landmark.constraints = [
        Rule::InsideWalls {
            target: "market".to_string(),
        },
        Rule::OutsideWalls {
            target: CORE_WORD.to_string(),
        },
        Rule::NotInDistrict {
            target: "town_wall".to_string(),
        },
    ]
    .into_iter()
    .map(|target_rule| rule(target_rule, Weight::DEFAULT))
    .collect();
    let mut specification = with_defences(None, Vec::new());
    specification.open_space.push(market(true));
    specification.landmarks.push(landmark);
    let wrong = |index: usize, target: &str, expected| ValidationError::TargetOfWrongKind {
        field: format!("landmarks[0].constraints[{index}].target"),
        target: target.to_string(),
        expected,
    };
    assert_eq!(
        validate(&specification),
        Err(vec![
            wrong(0, "market", "wall"),
            wrong(1, "core", "wall"),
            wrong(2, "town_wall", "district"),
        ])
    );
}

#[test]
fn near_and_far_from_point_to_anything_with_a_position() {
    let mut specification = with_gate_and_citadel();
    specification.site.water = with_water().site.water;
    let far_from = |target: &str| {
        let far_from = Rule::FarFrom {
            target: target.to_string(),
            beyond_metres: length(300.0),
        };
        rule(far_from, Weight::DEFAULT)
    };
    let mut landmark = guildhall(false);
    landmark.constraints = vec![
        near("ash"),
        near("mere"),
        near("quay"),
        near("cut"),
        near("west_road"),
        near(CORE_WORD),
        near("old_town"),
        far_from("town_wall"),
        far_from("west_gate"),
        far_from("keep"),
    ];
    specification.landmarks.push(landmark);
    assert_eq!(validate(&specification), Ok(()));
}

#[test]
fn a_district_rule_cannot_point_to_a_wall_gate_or_citadel() {
    let mut specification = with_gate_and_citadel();
    specification.districts[0].constraints = vec![
        near("town_wall"),
        near("west_gate"),
        rule(
            Rule::InDistrict {
                target: "keep".to_string(),
            },
            Weight::DEFAULT,
        ),
    ];
    let defence = |index: usize, target: &str| ValidationError::DistrictRuleToDefence {
        field: format!("districts[0].constraints[{index}].target"),
        target: target.to_string(),
    };
    assert_eq!(
        validate(&specification),
        Err(vec![
            defence(0, "town_wall"),
            defence(1, "west_gate"),
            defence(2, "keep"),
        ])
    );
}

fn ring(list: &'static str, objects: &[&str], arrows: &[&str]) -> ValidationError {
    ValidationError::DependencyRing {
        list,
        objects: identifiers(objects),
        arrows: identifiers(arrows),
    }
}

#[test]
fn objects_that_wait_for_each_other_are_a_ring() {
    let mut space = market(false);
    space.constraints = vec![near("craft")];
    let mut specification = with_market_and_bridge(space, bridge("ash"));
    add_district(&mut specification, "craft", vec![near("market")]);
    let error = only_error(&specification);
    assert_eq!(
        error,
        ring(
            OPEN_SPACE,
            &["market", "craft", "market"],
            &[
                "open_space[0].constraints[0] near craft",
                "districts[1].constraints[0] near market"
            ]
        )
    );
    assert_eq!(
        error.to_string(),
        "open_space: market -> craft -> market wait for each other in a ring (open_space[0].constraints[0] near craft; districts[1].constraints[0] near market); remove a rule, or pin a landmark, open space or crossing in the ring"
    );
    specification.open_space[0].pinned = true;
    assert_eq!(validate(&specification), Ok(()));
}

#[test]
fn a_rule_that_points_to_its_own_object_is_a_ring() {
    assert_eq!(
        only_error(&with_landmark(landmark_near("guildhall", &["guildhall"]))),
        ring(
            LANDMARKS,
            &["guildhall", "guildhall"],
            &["landmarks[0].constraints[0] near guildhall"]
        )
    );
}

#[test]
fn a_ring_starts_at_its_first_landmark_and_names_the_wall_and_gate_fields() {
    let mut specification = with_gate_and_citadel();
    specification.districts[0].constraints = vec![near("guildhall")];
    specification
        .landmarks
        .push(landmark_near("guildhall", &["west_gate"]));
    assert_eq!(
        only_error(&specification),
        ring(
            LANDMARKS,
            &[
                "guildhall",
                "west_gate",
                "town_wall",
                "old_town",
                "guildhall"
            ],
            &[
                "landmarks[0].constraints[0] near west_gate",
                "defences.gates[0].wall town_wall",
                "defences.walls[0].encloses old_town",
                "districts[0].constraints[0] near guildhall"
            ]
        )
    );
}

#[test]
fn a_ring_through_the_citadel_names_its_wall_field() {
    let mut specification =
        with_defences(Some(citadel("town_wall", Bearing::Degrees(0))), Vec::new());
    specification.districts[0].constraints = vec![near("guildhall")];
    specification
        .landmarks
        .push(landmark_near("guildhall", &["keep"]));
    assert_eq!(
        only_error(&specification),
        ring(
            LANDMARKS,
            &["guildhall", "keep", "town_wall", "old_town", "guildhall"],
            &[
                "landmarks[0].constraints[0] near keep",
                "defences.citadel.wall town_wall",
                "defences.walls[0].encloses old_town",
                "districts[0].constraints[0] near guildhall"
            ]
        )
    );
}

#[test]
fn a_landmark_ring_through_inside_walls_names_every_object() {
    let mut specification = with_defences(None, Vec::new());
    specification.districts[0].constraints = vec![near("guildhall")];
    let mut landmark = guildhall(false);
    landmark.constraints = vec![rule(
        Rule::InsideWalls {
            target: "town_wall".to_string(),
        },
        Weight::DEFAULT,
    )];
    specification.landmarks.push(landmark);
    assert_eq!(
        only_error(&specification),
        ring(
            LANDMARKS,
            &["guildhall", "town_wall", "old_town", "guildhall"],
            &[
                "landmarks[0].constraints[0] inside_walls town_wall",
                "defences.walls[0].encloses old_town",
                "districts[0].constraints[0] near guildhall"
            ]
        )
    );
}

#[test]
fn a_crossing_in_a_ring_starts_it_before_a_district() {
    let mut crossing = bridge("ash");
    crossing.constraints = vec![near("craft")];
    let mut specification = with_market_and_bridge(market(true), crossing);
    add_district(&mut specification, "craft", vec![near("ash_bridge")]);
    assert_eq!(
        only_error(&specification),
        ring(
            CROSSINGS,
            &["ash_bridge", "craft", "ash_bridge"],
            &[
                "crossings[0].constraints[0] near craft",
                "districts[1].constraints[0] near ash_bridge"
            ]
        )
    );
    specification.crossings[0].pinned = true;
    specification.crossings[0].resolved = Some(placement(0.0));
    assert_eq!(validate(&specification), Ok(()));
}

#[test]
fn a_pinned_landmark_breaks_a_ring() {
    let mut specification = with_landmark(landmark_near("guildhall", &["craft"]));
    add_district(&mut specification, "craft", vec![near("guildhall")]);
    assert_eq!(
        only_error(&specification),
        ring(
            LANDMARKS,
            &["guildhall", "craft", "guildhall"],
            &[
                "landmarks[0].constraints[0] near craft",
                "districts[1].constraints[0] near guildhall"
            ]
        )
    );
    specification.landmarks[0].pinned = true;
    assert_eq!(validate(&specification), Ok(()));
}

#[test]
fn a_ring_is_reported_once_from_its_first_object() {
    let mut specification = valid();
    for (identifier, target) in [("first", "second"), ("second", "third"), ("third", "first")] {
        specification
            .landmarks
            .push(landmark_near(identifier, &[target]));
    }
    assert_eq!(
        only_error(&specification),
        ring(
            LANDMARKS,
            &["first", "second", "third", "first"],
            &[
                "landmarks[0].constraints[0] near second",
                "landmarks[1].constraints[0] near third",
                "landmarks[2].constraints[0] near first"
            ]
        )
    );
}

#[test]
fn two_rings_in_one_file_are_both_reported() {
    let mut specification = valid();
    for (identifier, target) in [("a", "b"), ("b", "a"), ("c", "d"), ("d", "c")] {
        specification
            .landmarks
            .push(landmark_near(identifier, &[target]));
    }
    assert_eq!(
        validate(&specification),
        Err(vec![
            ring(
                LANDMARKS,
                &["a", "b", "a"],
                &[
                    "landmarks[0].constraints[0] near b",
                    "landmarks[1].constraints[0] near a"
                ]
            ),
            ring(
                LANDMARKS,
                &["c", "d", "c"],
                &[
                    "landmarks[2].constraints[0] near d",
                    "landmarks[3].constraints[0] near c"
                ]
            ),
        ])
    );
}

#[test]
fn a_second_ring_through_an_object_of_a_reported_ring_is_absent() {
    let mut specification = valid();
    for (identifier, targets) in [("a", &["b", "c"][..]), ("b", &["a"]), ("c", &["a"])] {
        specification
            .landmarks
            .push(landmark_near(identifier, targets));
    }
    assert_eq!(
        only_error(&specification),
        ring(
            LANDMARKS,
            &["a", "b", "a"],
            &[
                "landmarks[0].constraints[0] near b",
                "landmarks[1].constraints[0] near a"
            ]
        )
    );
}

#[test]
fn a_second_wall_ring_through_a_wall_of_a_reported_ring_is_absent() {
    let walls = vec![wall("a", &["b", "c"]), wall("b", &["a"]), wall("c", &["a"])];
    assert_eq!(
        only_error(&with_walls(walls)),
        ValidationError::WallRing {
            walls: identifiers(&["a", "b", "a"]),
        }
    );
}

#[test]
fn a_duplicate_identifier_skips_the_dependency_checks() {
    let mut specification = valid();
    add_district(&mut specification, "x", vec![near("hall")]);
    specification.landmarks.push(landmark_near("hall", &["x"]));
    specification.landmarks.push(landmark_near("x", &[]));
    assert_eq!(
        only_error(&specification),
        ValidationError::DuplicateIdentifier {
            list: LANDMARKS,
            identifier: "x".to_string(),
        }
    );
}

#[test]
fn a_target_of_the_wrong_kind_adds_no_arrow_to_a_ring() {
    let mut space = market(false);
    space.constraints = vec![near("guildhall")];
    let mut specification = with_market_and_bridge(space, bridge("ash"));
    let mut landmark = guildhall(false);
    landmark.constraints = vec![rule(
        Rule::InsideWalls {
            target: "market".to_string(),
        },
        Weight::DEFAULT,
    )];
    specification.landmarks.push(landmark);
    assert_eq!(
        only_error(&specification),
        ValidationError::TargetOfWrongKind {
            field: "landmarks[0].constraints[0].target".to_string(),
            target: "market".to_string(),
            expected: "wall",
        }
    );
}

#[test]
fn open_space_and_crossing_targets_are_checked() {
    let broken_rules = || {
        vec![
            near("nowhere"),
            rule(
                Rule::InDistrict {
                    target: "ash".to_string(),
                },
                Weight::DEFAULT,
            ),
        ]
    };
    let mut space = market(true);
    space.constraints = broken_rules();
    let mut crossing = bridge("ash");
    crossing.constraints = broken_rules();
    let missing = |owner: &str| ValidationError::TargetMissing {
        field: format!("{owner}.constraints[0].target"),
        target: "nowhere".to_string(),
    };
    let wrong = |owner: &str| ValidationError::TargetOfWrongKind {
        field: format!("{owner}.constraints[1].target"),
        target: "ash".to_string(),
        expected: "district",
    };
    assert_eq!(
        validate(&with_market_and_bridge(space, crossing)),
        Err(vec![
            missing("open_space[0]"),
            wrong("open_space[0]"),
            missing("crossings[0]"),
            wrong("crossings[0]"),
        ])
    );
}

#[test]
fn every_error_is_reported_not_only_the_first() {
    let mut specification = valid();
    specification.schema = 2;
    specification.settlement.population = 0;
    specification.roads.clear();
    let errors = validate(&specification).expect_err("three rules are broken");
    assert_eq!(errors.len(), 3, "{errors:?}");
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn a_specification_within_every_rule_passes(specification in any_valid_specification()) {
        prop_assert_eq!(validate(&specification), Ok(()));
    }

    #[test]
    fn a_chain_of_districts_passes_and_closing_it_names_every_district(count in 1..20_usize) {
        let mut specification = valid();
        let name = |index: usize| format!("district_{index}");
        for index in 1..=count {
            add_district(&mut specification, &name(index), vec![near(&name(index - 1))]);
        }
        specification.districts[0].identifier = name(0);
        prop_assert_eq!(validate(&specification), Ok(()));
        specification.districts[0].constraints = vec![near(&name(count))];
        let mut ring: Vec<String> = (0..=count).rev().map(name).collect();
        ring.insert(0, name(0));
        let mut arrows = vec![format!("districts[0].constraints[0] near {}", name(count))];
        arrows.extend((1..=count).rev().map(|index| {
            format!("districts[{index}].constraints[0] near {}", name(index - 1))
        }));
        prop_assert_eq!(
            validate(&specification),
            Err(vec![ValidationError::DependencyRing { list: DISTRICTS, objects: ring, arrows }])
        );
    }
}
