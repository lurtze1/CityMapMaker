//! Stage S0, the site: the land before the city (ARCHITECTURE.md §4). In milestone 0b rivers
//! and roads run straight between edge-stretch middles, roads to the core straight to it, and
//! the core is chosen between those two steps (ADR-039, ADR-047, ADR-049, ADR-050).

use serde::Serialize;

use crate::geometry::{self, Point, Polyline};
use crate::identifier::{self, Identifier, IdentifierKind};
use crate::town_core::{self, TownCore};
use citymap_specification::specification::{
    CompassRegion, Importance, River, RiverCrossingKind, Road, RoadEnd, Specification,
};
use citymap_specification::units::{Length, SignedLength};

/// The output of stage S0.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SiteOutput {
    /// The side of the square site, copied from the specification.
    pub extent_metres: Length,
    /// One course per river, in the order of the specification.
    pub rivers: Vec<RiverCourse>,
    /// One course per road, in the order of the specification: roads between two edge
    /// stretches (step one) and roads to the core (step three, specification-version-1 §2).
    pub roads: Vec<RoadCourse>,
    /// Every place where any road crosses a river, by road and then river in specification
    /// order, and along the road from its first end.
    pub crossings: Vec<RiverCrossing>,
    /// Every place where two roads really cross, by road pair in specification order and
    /// along the first road (ARCHITECTURE.md §4, S0 row).
    pub junctions: Vec<RoadJunction>,
    /// The town core, chosen after the roads of step one and their crossings.
    pub core: TownCore,
    /// Roads whose course has length zero, by identifier as written in the specification;
    /// each is reported as a notice, the map keeps them (ADR-050).
    pub zero_length_roads: Vec<String>,
}

/// How wide a crossing is, in metres: a default-pack value without a source (ADR-021).
pub const CROSSING_WIDTH_METRES: f64 = 8.0;

/// How far a bridge reaches onto each bank, in metres: a default-pack value (ADR-021).
pub const BRIDGE_BANK_METRES: f64 = 5.0;

/// A place where two roads really cross; sharing or touching an end is no junction.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RoadJunction {
    /// Derived from `name`.
    pub identifier: Identifier,
    /// The two road identifiers in alphabetical order joined by `+`, such as
    /// `market_road+north_road`, with `.<n>` when the two roads cross again.
    pub name: String,
    /// The two roads, in the order of `name`.
    pub roads: [Identifier; 2],
    /// Where the two centre lines cross.
    pub position: Point,
}

/// A place where a road crosses a river, which S0 creates (specification-version-1 §2).
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RiverCrossing {
    /// Derived from `name`, as a crossing listed in a specification would be (ADR-048).
    pub identifier: Identifier,
    /// `<road>.<river>`, or `<road>.<river>.<n>` when the road crosses that river again.
    pub name: String,
    /// The road that crosses.
    pub road: Identifier,
    /// The river it crosses.
    pub river: Identifier,
    /// A bridge or a ford, from the road's `river_crossing`.
    pub kind: RiverCrossingKind,
    /// Where the road's centre line crosses the river's.
    pub position: Point,
    /// The crossing's centre line along the road: the river's width, plus the banks for a bridge.
    pub span: Polyline,
    /// How wide the crossing is, `CROSSING_WIDTH_METRES`.
    pub width: Length,
}

/// The path of one road across the site.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RoadCourse {
    /// Derived from the road's identifier in the specification.
    pub identifier: Identifier,
    /// The centre line, from the road's first end to its second.
    pub course: Polyline,
    /// The road's importance, copied from the specification.
    pub importance: Importance,
}

/// The path of one river across the site.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RiverCourse {
    /// Derived from the river's identifier in the specification.
    pub identifier: Identifier,
    /// The centre line, from where the river enters to where it leaves.
    pub course: Polyline,
    /// The river's width, copied from the specification.
    pub width_metres: Length,
}

/// Runs stage S0 on a specification.
///
/// # Panics
///
/// In a debug build, when two rivers or roads share an identifier (validation rejects that).
///
/// # Examples
///
/// ```
/// use citymap_core::site;
/// use citymap_core::town_core::CoreRule;
/// # use citymap_specification::specification::{
/// #     CompassRegion, District, DistrictFunction, Enclosure, Era, Identity, Importance,
/// #     Jurisdiction, LandCover, Presets, River, RiverCrossingKind, Road, RoadEnd, Seed,
/// #     Settlement, Site, Specification, Terrain, Water, Wealth,
/// # };
/// # use citymap_specification::units::{Length, Weight};
/// # let length = |metres| Length::new(metres).expect("a valid length");
/// # let specification = Specification {
/// #     schema: 1,
/// #     identity: Identity { name: "Harrowmere".to_string(), seed: Seed::new(1), notes: String::new() },
/// #     settlement: Settlement { population: 6000 },
/// #     site: Site {
/// #         extent_metres: length(2400.0),
/// #         terrain: Terrain::default(),
/// #         land_cover: LandCover::default(),
/// #         water: Water {
/// #             rivers: vec![River {
/// #                 identifier: "ash".to_string(),
/// #                 from: CompassRegion::West,
/// #                 to: CompassRegion::East,
/// #                 width_metres: length(18.0),
/// #             }],
/// #             ..Water::default()
/// #         },
/// #     },
/// #     roads: vec![Road {
/// #         identifier: "north_road".to_string(),
/// #         ends: [RoadEnd::Region(CompassRegion::North), RoadEnd::Region(CompassRegion::South)],
/// #         importance: Importance::Major,
/// #         river_crossing: RiverCrossingKind::Bridge,
/// #     }],
/// #     defences: Default::default(),
/// #     open_space: Vec::new(),
/// #     crossings: Vec::new(),
/// #     districts: vec![District {
/// #         identifier: "old_town".to_string(),
/// #         name: "Old Town".to_string(),
/// #         function: DistrictFunction::Residential,
/// #         wealth: Wealth::Middling,
/// #         group: None,
/// #         enclosure: Enclosure::Open,
/// #         jurisdiction: Jurisdiction::Town,
/// #         weight: Weight::DEFAULT,
/// #         constraints: Vec::new(),
/// #         notes: String::new(),
/// #     }],
/// #     landmarks: Vec::new(),
/// #     presets: Presets::new(Era::EarlyMedieval),
/// # };
/// let output = site::run(&specification);
/// assert_eq!(output.rivers.len(), 1);
/// assert_eq!(output.roads.len(), 1);
/// assert_eq!(output.crossings.len(), 1, "the north road bridges the river");
/// assert_eq!(output.core.rule, CoreRule::Crossing);
/// ```
pub fn run(specification: &Specification) -> SiteOutput {
    let extent = specification.site.extent_metres;
    let rivers: Vec<RiverCourse> = specification
        .site
        .water
        .rivers
        .iter()
        .map(|river| river_course(river, extent))
        .collect();
    let laid_out: Vec<(&Road, RoadCourse)> = specification
        .roads
        .iter()
        .filter_map(|road| edge_road_course(road, extent).map(|course| (road, course)))
        .collect();
    let step_one_crossings = river_crossings(&laid_out, &specification.site.water.rivers, &rivers);
    let step_one_junctions = road_junctions(&laid_out);
    let river_pairs: Vec<(&River, &RiverCourse)> = specification
        .site
        .water
        .rivers
        .iter()
        .zip(&rivers)
        .collect();
    let core = town_core::choose(
        extent,
        &laid_out,
        &step_one_crossings,
        &step_one_junctions,
        &river_pairs,
    );
    let all_roads: Vec<(&Road, RoadCourse)> = specification
        .roads
        .iter()
        .filter_map(|road| {
            edge_road_course(road, extent)
                .or_else(|| core_road_course(road, extent, core.position))
                .map(|course| (road, course))
        })
        .collect();
    let crossings = river_crossings(&all_roads, &specification.site.water.rivers, &rivers);
    let junctions = road_junctions(&all_roads);
    let zero_length_roads: Vec<String> = all_roads
        .iter()
        .filter(|(_, course)| {
            let points = course.course.points();
            points.iter().all(|point| *point == points[0])
        })
        .map(|(road, _)| road.identifier.clone())
        .collect();
    let roads: Vec<RoadCourse> = all_roads.into_iter().map(|(_, course)| course).collect();
    let identifiers: Vec<Identifier> = rivers
        .iter()
        .map(|river| river.identifier)
        .chain(roads.iter().map(|road| road.identifier))
        .chain(crossings.iter().map(|crossing| crossing.identifier))
        .chain(junctions.iter().map(|junction| junction.identifier))
        .chain([core.identifier])
        .collect();
    identifier::debug_assert_unique(&identifiers);
    SiteOutput {
        extent_metres: extent,
        rivers,
        roads,
        crossings,
        junctions,
        core,
        zero_length_roads,
    }
}

fn road_junctions(roads: &[(&Road, RoadCourse)]) -> Vec<RoadJunction> {
    let mut junctions = Vec::new();
    for (index, (first, first_course)) in roads.iter().enumerate() {
        for (second, second_course) in &roads[index + 1..] {
            let mut pair = [(first, first_course), (second, second_course)];
            pair.sort_by(|left, right| left.0.identifier.cmp(&right.0.identifier));
            let base = format!("{}+{}", pair[0].0.identifier, pair[1].0.identifier);
            let found = geometry::crossings_along(&first_course.course, &second_course.course);
            for (number, crossing) in found.iter().enumerate() {
                let name = if found.len() == 1 {
                    base.clone()
                } else {
                    format!("{base}.{}", number + 1)
                };
                junctions.push(RoadJunction {
                    identifier: Identifier::new(IdentifierKind::Junction, &[], &[&name]),
                    name,
                    roads: [pair[0].1.identifier, pair[1].1.identifier],
                    position: crossing.position,
                });
            }
        }
    }
    junctions
}

fn river_crossings(
    roads: &[(&Road, RoadCourse)],
    rivers: &[River],
    courses: &[RiverCourse],
) -> Vec<RiverCrossing> {
    let width = Length::new(CROSSING_WIDTH_METRES).expect("the crossing width is a valid length");
    let mut crossings = Vec::new();
    for (road, road_course) in roads {
        let banks = match road.river_crossing {
            RiverCrossingKind::Bridge => 2.0 * BRIDGE_BANK_METRES,
            RiverCrossingKind::Ford => 0.0,
        };
        for (river, river_course) in rivers.iter().zip(courses) {
            let found = geometry::crossings_along(&road_course.course, &river_course.course);
            let base = format!("{}.{}", road.identifier, river.identifier);
            let length = river.width_metres.get() + banks;
            for (index, crossing) in found.iter().enumerate() {
                let name = if found.len() == 1 {
                    base.clone()
                } else {
                    format!("{base}.{}", index + 1)
                };
                crossings.push(RiverCrossing {
                    identifier: Identifier::new(IdentifierKind::Crossing, &[], &[&name]),
                    name,
                    road: road_course.identifier,
                    river: river_course.identifier,
                    kind: road.river_crossing,
                    position: crossing.position,
                    span: geometry::span_through(crossing.road_segment, crossing.position, length),
                    width,
                });
            }
        }
    }
    crossings
}

fn edge_road_course(road: &Road, extent: Length) -> Option<RoadCourse> {
    let [RoadEnd::Region(first), RoadEnd::Region(second)] = road.ends else {
        return None;
    };
    Some(RoadCourse {
        identifier: Identifier::new(IdentifierKind::Road, &[], &[&road.identifier]),
        course: straight(
            stretch_middle(first, extent),
            stretch_middle(second, extent),
        ),
        importance: road.importance,
    })
}

fn core_road_course(road: &Road, extent: Length, core: Point) -> Option<RoadCourse> {
    let course = match road.ends {
        [RoadEnd::Region(region), RoadEnd::Core] => straight(stretch_middle(region, extent), core),
        [RoadEnd::Core, RoadEnd::Region(region)] => straight(core, stretch_middle(region, extent)),
        _ => return None,
    };
    Some(RoadCourse {
        identifier: Identifier::new(IdentifierKind::Road, &[], &[&road.identifier]),
        course,
        importance: road.importance,
    })
}

fn river_course(river: &River, extent: Length) -> RiverCourse {
    RiverCourse {
        identifier: Identifier::new(IdentifierKind::River, &[], &[&river.identifier]),
        course: straight(
            stretch_middle(river.from, extent),
            stretch_middle(river.to, extent),
        ),
        width_metres: river.width_metres,
    }
}

fn straight(start: Point, end: Point) -> Polyline {
    Polyline::new(vec![start, end]).expect("two points always make a polyline")
}

/// The middle of a region's edge stretch: the side's middle for north, east, south and west,
/// the corner for the four diagonals, and the site's centre for `Centre` (ADR-039).
fn stretch_middle(region: CompassRegion, extent: Length) -> Point {
    let (east, north) = match region {
        CompassRegion::North => (0.5, 1.0),
        CompassRegion::NorthEast => (1.0, 1.0),
        CompassRegion::East => (1.0, 0.5),
        CompassRegion::SouthEast => (1.0, 0.0),
        CompassRegion::South => (0.5, 0.0),
        CompassRegion::SouthWest => (0.0, 0.0),
        CompassRegion::West => (0.0, 0.5),
        CompassRegion::NorthWest => (0.0, 1.0),
        CompassRegion::Centre => (0.5, 0.5),
    };
    let coordinate = |fraction: f64| {
        SignedLength::new(extent.get() * fraction).expect("a point on the site is a valid length")
    };
    Point {
        x: coordinate(east),
        y: coordinate(north),
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use citymap_specification::specification::{
        Defences, Era, Identity, LandCover, Presets, Seed, Settlement, Site, Terrain, Water,
    };

    fn length(value: f64) -> Length {
        Length::new(value).expect("test input is a valid length")
    }

    fn point(x: f64, y: f64) -> Point {
        Point {
            x: SignedLength::new(x).expect("test input is a valid length"),
            y: SignedLength::new(y).expect("test input is a valid length"),
        }
    }

    fn river(identifier: &str, from: CompassRegion, to: CompassRegion) -> River {
        River {
            identifier: identifier.to_string(),
            from,
            to,
            width_metres: length(18.0),
        }
    }

    fn road(identifier: &str, first: RoadEnd, second: RoadEnd) -> Road {
        Road {
            identifier: identifier.to_string(),
            ends: [first, second],
            importance: Importance::Major,
            river_crossing: RiverCrossingKind::Bridge,
        }
    }

    fn with_roads(mut specification: Specification, roads: Vec<Road>) -> Specification {
        specification.roads = roads;
        specification
    }

    fn specification(extent: f64, rivers: Vec<River>) -> Specification {
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
            presets: Presets::new(Era::EarlyMedieval),
        }
    }

    #[test]
    fn each_region_maps_to_the_middle_of_its_stretch() {
        let cases = [
            (CompassRegion::North, point(500.0, 1000.0)),
            (CompassRegion::NorthEast, point(1000.0, 1000.0)),
            (CompassRegion::East, point(1000.0, 500.0)),
            (CompassRegion::SouthEast, point(1000.0, 0.0)),
            (CompassRegion::South, point(500.0, 0.0)),
            (CompassRegion::SouthWest, point(0.0, 0.0)),
            (CompassRegion::West, point(0.0, 500.0)),
            (CompassRegion::NorthWest, point(0.0, 1000.0)),
            (CompassRegion::Centre, point(500.0, 500.0)),
        ];
        for (region, expected) in cases {
            assert_eq!(
                stretch_middle(region, length(1000.0)),
                expected,
                "{region:?}"
            );
        }
    }

    #[test]
    fn a_river_runs_from_its_from_stretch_to_its_to_stretch() {
        let output = run(&specification(
            2400.0,
            vec![river("ash", CompassRegion::NorthWest, CompassRegion::East)],
        ));
        let course = &output.rivers[0];
        assert_eq!(
            course.course.points(),
            [point(0.0, 2400.0), point(2400.0, 1200.0)]
        );
        assert_eq!(course.width_metres, length(18.0));
        assert_eq!(
            course.identifier,
            Identifier::new(IdentifierKind::River, &[], &["ash"])
        );
    }

    #[test]
    fn a_road_between_two_stretches_runs_between_their_middles() {
        let output = run(&with_roads(
            specification(2400.0, Vec::new()),
            vec![road(
                "market_road",
                RoadEnd::Region(CompassRegion::SouthWest),
                RoadEnd::Region(CompassRegion::NorthEast),
            )],
        ));
        let course = &output.roads[0];
        assert_eq!(
            course.course.points(),
            [point(0.0, 0.0), point(2400.0, 2400.0)]
        );
        assert_eq!(course.importance, Importance::Major);
        assert_eq!(
            course.identifier,
            Identifier::new(IdentifierKind::Road, &[], &["market_road"])
        );
    }

    #[test]
    fn a_road_to_the_core_runs_from_its_stretch_to_the_core() {
        let output = run(&with_roads(
            specification(2400.0, Vec::new()),
            vec![road(
                "west_road",
                RoadEnd::Region(CompassRegion::West),
                RoadEnd::Core,
            )],
        ));
        assert_eq!(
            output.roads[0].course.points(),
            [point(0.0, 1200.0), output.core.position]
        );
        let reversed = run(&with_roads(
            specification(2400.0, Vec::new()),
            vec![road(
                "west_road",
                RoadEnd::Core,
                RoadEnd::Region(CompassRegion::West),
            )],
        ));
        assert_eq!(
            reversed.roads[0].course.points(),
            [reversed.core.position, point(0.0, 1200.0)]
        );
    }

    #[test]
    fn a_road_to_a_core_on_its_own_stretch_has_length_zero_and_is_reported() {
        let output = run(&with_roads(
            specification(2400.0, Vec::new()),
            vec![
                road(
                    "high_road",
                    RoadEnd::Region(CompassRegion::NorthWest),
                    RoadEnd::Region(CompassRegion::NorthEast),
                ),
                road(
                    "north_lane",
                    RoadEnd::Region(CompassRegion::North),
                    RoadEnd::Core,
                ),
            ],
        ));
        assert_eq!(output.core.position, point(1200.0, 2400.0));
        assert_eq!(output.zero_length_roads, ["north_lane"]);
    }

    #[test]
    fn a_road_to_the_core_gets_its_crossings() {
        let north_road = road(
            "north_road",
            RoadEnd::Region(CompassRegion::North),
            RoadEnd::Region(CompassRegion::South),
        );
        let south_road = road(
            "south_road",
            RoadEnd::Region(CompassRegion::SouthEast),
            RoadEnd::Core,
        );
        let output = run(&with_roads(
            specification(
                2400.0,
                vec![
                    river("ash", CompassRegion::NorthWest, CompassRegion::East),
                    river("brook", CompassRegion::South, CompassRegion::East),
                ],
            ),
            vec![north_road, south_road],
        ));
        let names: Vec<&str> = output
            .crossings
            .iter()
            .map(|crossing| crossing.name.as_str())
            .collect();
        assert_eq!(names, ["north_road.ash", "south_road.brook"]);
        assert_eq!(output.crossings[1].position, point(1920.0, 720.0));
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "duplicate identifier")]
    fn two_roads_with_one_identifier_stop_a_debug_build() {
        let north_south = road(
            "north_road",
            RoadEnd::Region(CompassRegion::North),
            RoadEnd::Region(CompassRegion::South),
        );
        run(&with_roads(
            specification(2400.0, Vec::new()),
            vec![north_south.clone(), north_south],
        ));
    }

    fn north_road_over_ash(kind: RiverCrossingKind) -> SiteOutput {
        let mut north_road = road(
            "north_road",
            RoadEnd::Region(CompassRegion::North),
            RoadEnd::Region(CompassRegion::South),
        );
        north_road.river_crossing = kind;
        run(&with_roads(
            specification(
                2400.0,
                vec![river("ash", CompassRegion::NorthWest, CompassRegion::East)],
            ),
            vec![north_road],
        ))
    }

    #[test]
    fn a_road_over_a_river_makes_one_named_crossing() {
        let output = north_road_over_ash(RiverCrossingKind::Bridge);
        let crossing = &output.crossings[0];
        assert_eq!(output.crossings.len(), 1);
        assert_eq!(crossing.name, "north_road.ash");
        assert_eq!(
            crossing.identifier,
            Identifier::new(IdentifierKind::Crossing, &[], &["north_road.ash"])
        );
        assert_eq!(crossing.road, output.roads[0].identifier);
        assert_eq!(crossing.river, output.rivers[0].identifier);
        assert_eq!(crossing.position, point(1200.0, 1800.0));
    }

    #[test]
    fn a_bridge_spans_the_river_and_both_banks_and_a_ford_only_the_river() {
        let bridge = north_road_over_ash(RiverCrossingKind::Bridge);
        assert_eq!(bridge.crossings[0].kind, RiverCrossingKind::Bridge);
        assert_eq!(
            bridge.crossings[0].span.points(),
            [point(1200.0, 1814.0), point(1200.0, 1786.0)]
        );
        let ford = north_road_over_ash(RiverCrossingKind::Ford);
        assert_eq!(ford.crossings[0].kind, RiverCrossingKind::Ford);
        assert_eq!(
            ford.crossings[0].span.points(),
            [point(1200.0, 1809.0), point(1200.0, 1791.0)]
        );
    }

    #[test]
    fn crossing_one_river_twice_numbers_the_crossings_from_the_first_end() {
        let zig_zag = road(
            "north_road",
            RoadEnd::Region(CompassRegion::North),
            RoadEnd::Region(CompassRegion::South),
        );
        let course = RoadCourse {
            identifier: Identifier::new(IdentifierKind::Road, &[], &["north_road"]),
            course: Polyline::new(vec![point(0.0, 0.0), point(10.0, 10.0), point(20.0, 0.0)])
                .expect("three points make a polyline"),
            importance: Importance::Major,
        };
        let ash = river("ash", CompassRegion::West, CompassRegion::East);
        let ash_course = RiverCourse {
            identifier: Identifier::new(IdentifierKind::River, &[], &["ash"]),
            course: Polyline::new(vec![point(0.0, 4.0), point(20.0, 4.0)])
                .expect("two points make a polyline"),
            width_metres: length(2.0),
        };
        let crossings = river_crossings(&[(&zig_zag, course)], &[ash], &[ash_course]);
        let names: Vec<&str> = crossings
            .iter()
            .map(|crossing| crossing.name.as_str())
            .collect();
        assert_eq!(names, ["north_road.ash.1", "north_road.ash.2"]);
        assert_eq!(crossings[0].position, point(4.0, 4.0));
    }

    #[test]
    fn two_roads_that_cross_make_one_named_junction() {
        let output = run(&with_roads(
            specification(2400.0, Vec::new()),
            vec![
                road(
                    "north_road",
                    RoadEnd::Region(CompassRegion::North),
                    RoadEnd::Region(CompassRegion::South),
                ),
                road(
                    "market_road",
                    RoadEnd::Region(CompassRegion::SouthWest),
                    RoadEnd::Region(CompassRegion::NorthEast),
                ),
                road(
                    "west_lane",
                    RoadEnd::Region(CompassRegion::West),
                    RoadEnd::Region(CompassRegion::North),
                ),
            ],
        ));
        let names: Vec<&str> = output
            .junctions
            .iter()
            .map(|junction| junction.name.as_str())
            .collect();
        assert_eq!(names, ["market_road+north_road"]);
        let junction = &output.junctions[0];
        assert_eq!(junction.position, point(1200.0, 1200.0));
        assert_eq!(
            junction.identifier,
            Identifier::new(IdentifierKind::Junction, &[], &["market_road+north_road"])
        );
        assert_eq!(
            junction.roads,
            [output.roads[1].identifier, output.roads[0].identifier]
        );
    }

    #[test]
    fn a_site_without_rivers_has_no_courses() {
        assert_eq!(run(&specification(2400.0, Vec::new())).rivers, Vec::new());
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "duplicate identifier")]
    fn two_rivers_with_one_identifier_stop_a_debug_build() {
        run(&specification(
            2400.0,
            vec![
                river("ash", CompassRegion::North, CompassRegion::South),
                river("ash", CompassRegion::East, CompassRegion::West),
            ],
        ));
    }

    fn any_edge() -> impl Strategy<Value = CompassRegion> {
        let edges: Vec<CompassRegion> = CompassRegion::ALL
            .into_iter()
            .filter(|region| *region != CompassRegion::Centre)
            .collect();
        prop::sample::select(edges)
    }

    fn any_two_edges() -> impl Strategy<Value = (CompassRegion, CompassRegion)> {
        (any_edge(), any_edge())
            .prop_filter("two different stretches", |(first, second)| first != second)
    }

    fn any_road_ends() -> impl Strategy<Value = [RoadEnd; 2]> {
        prop_oneof![
            any_two_edges()
                .prop_map(|(first, second)| [RoadEnd::Region(first), RoadEnd::Region(second)]),
            any_edge().prop_map(|edge| [RoadEnd::Region(edge), RoadEnd::Core]),
        ]
    }

    fn any_specification() -> impl Strategy<Value = Specification> {
        let rivers =
            prop::collection::btree_map("[a-z]{1,8}", any_two_edges(), 0..6).prop_map(|rivers| {
                rivers
                    .into_iter()
                    .map(|(identifier, (from, to))| river(&identifier, from, to))
                    .collect()
            });
        let roads =
            prop::collection::btree_map("[a-z]{1,8}", any_road_ends(), 0..6).prop_map(|roads| {
                roads
                    .into_iter()
                    .map(|(identifier, [first, second])| road(&identifier, first, second))
                    .collect()
            });
        (1.0..1e6_f64, rivers, roads)
            .prop_map(|(extent, rivers, roads)| with_roads(specification(extent, rivers), roads))
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(64))]

        #[test]
        fn the_same_specification_gives_the_same_output(specification in any_specification()) {
            let serialise = |output: &SiteOutput| {
                serde_json::to_string(output).expect("the output serialises")
            };
            prop_assert_eq!(serialise(&run(&specification)), serialise(&run(&specification)));
        }

        #[test]
        fn river_road_and_crossing_identifiers_are_unique(specification in any_specification()) {
            let output = run(&specification);
            let identifiers: Vec<Identifier> = output
                .rivers
                .iter()
                .map(|river| river.identifier)
                .chain(output.roads.iter().map(|road| road.identifier))
                .chain(output.crossings.iter().map(|crossing| crossing.identifier))
                .chain(output.junctions.iter().map(|junction| junction.identifier))
                .collect();
            prop_assert!(identifier::check_unique(&identifiers).is_ok());
        }

        #[test]
        fn every_road_is_laid_out_and_every_core_road_ends_at_the_core(
            specification in any_specification(),
        ) {
            let output = run(&specification);
            prop_assert_eq!(output.roads.len(), specification.roads.len());
            for (road, course) in specification.roads.iter().zip(&output.roads) {
                prop_assert_eq!(
                    course.identifier,
                    Identifier::new(IdentifierKind::Road, &[], &[&road.identifier])
                );
                let points = course.course.points();
                match road.ends {
                    [RoadEnd::Core, _] => prop_assert_eq!(points[0], output.core.position),
                    [_, RoadEnd::Core] => {
                        prop_assert_eq!(points[points.len() - 1], output.core.position);
                    }
                    _ => {}
                }
            }
        }

        #[test]
        fn adding_a_core_road_changes_no_step_one_course_crossing_or_core(
            specification in any_specification(),
            edge in any_edge(),
        ) {
            let before = run(&specification);
            let mut with_core_road = specification.clone();
            let added = road("added_core_road", RoadEnd::Region(edge), RoadEnd::Core);
            with_core_road.roads.push(added);
            let after = run(&with_core_road);
            prop_assert_eq!(&after.core, &before.core);
            prop_assert_eq!(&after.roads[..before.roads.len()], &*before.roads);
            let kept: Vec<&RiverCrossing> = after
                .crossings
                .iter()
                .filter(|crossing| crossing.road != after.roads[before.roads.len()].identifier)
                .collect();
            let earlier: Vec<&RiverCrossing> = before.crossings.iter().collect();
            prop_assert_eq!(kept, earlier);
        }

        #[test]
        fn every_course_stays_on_the_site(specification in any_specification()) {
            let extent = specification.site.extent_metres.get();
            let output = run(&specification);
            let courses = output
                .rivers
                .iter()
                .map(|river| &river.course)
                .chain(output.roads.iter().map(|road| &road.course));
            for crossing in &output.crossings {
                prop_assert!((0.0..=extent).contains(&crossing.position.x.get()));
                prop_assert!((0.0..=extent).contains(&crossing.position.y.get()));
            }
            for course in courses {
                for point in course.points() {
                    prop_assert!((0.0..=extent).contains(&point.x.get()));
                    prop_assert!((0.0..=extent).contains(&point.y.get()));
                }
            }
        }
    }
}
