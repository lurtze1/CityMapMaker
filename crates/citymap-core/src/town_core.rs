//! The town core of S0 (specification-version-1 §4): chosen after the roads of step one and
//! their crossings, before the roads that end at the core (ADR-049).

use std::cmp::Reverse;

use serde::Serialize;

use crate::geometry::{self, Point, Polyline};
use crate::identifier::{Identifier, IdentifierKind};
use crate::site::{RiverCourse, RiverCrossing, RoadCourse, RoadJunction};
use citymap_specification::specification::{Importance, River, Road};
use citymap_specification::units::{Length, SignedLength};

/// The town core: where the roads that end at the core end, and where S3 starts its links.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TownCore {
    /// Always `Identifier::new(Core, [], ["core"])`; there is one core.
    pub identifier: Identifier,
    /// Where the core is.
    pub position: Point,
    /// What the core lies on: a river crossing, a road junction, a road or a river; `None`
    /// only for the site centre (ADR-049, #139).
    pub on: Option<Identifier>,
    /// Which rule of specification-version-1 §4 chose the core.
    pub rule: CoreRule,
    /// Set when the river rule decided between equally important roads.
    pub notice: Option<CoreNotice>,
}

/// The rule of specification-version-1 §4 that chose the core.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CoreRule {
    /// The best place where roads cross a river or each other.
    Crossing,
    /// No crossing: the point nearest the site centre on a road of the highest importance.
    OneRoad,
    /// No road of step one: the river point nearest the site centre.
    River,
    /// No road of step one and no river: the site centre.
    SiteCentre,
}

/// The core is on a river crossing, chosen over a road junction of equally important roads.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CoreNotice {
    /// The crossing that became the core, such as `market_road.ash`.
    pub chosen: String,
    /// The best junction of equally important roads, such as `market_road+north_road`.
    pub passed_over: String,
}

/// A place the core can be: a river crossing or a road junction.
struct Candidate {
    object: Identifier,
    name: String,
    position: Point,
    importance: Importance,
    on_river: bool,
}

/// Chooses the core from the step-one roads, their crossings and junctions, and the rivers.
pub(crate) fn choose(
    extent: Length,
    roads: &[(&Road, RoadCourse)],
    crossings: &[RiverCrossing],
    junctions: &[RoadJunction],
    rivers: &[(&River, &RiverCourse)],
) -> TownCore {
    let half = SignedLength::new(extent.get() / 2.0).expect("half the site is a valid length");
    let centre = Point { x: half, y: half };
    let importance_of = |road: Identifier| {
        roads
            .iter()
            .find(|(_, course)| course.identifier == road)
            .map(|(road, _)| road.importance)
            .expect("every crossing's road is laid out")
    };
    let mut candidates: Vec<Candidate> = crossings
        .iter()
        .map(|crossing| Candidate {
            object: crossing.identifier,
            name: crossing.name.clone(),
            position: crossing.position,
            importance: importance_of(crossing.road),
            on_river: true,
        })
        .collect();
    candidates.extend(junctions.iter().map(|junction| Candidate {
        object: junction.identifier,
        name: junction.name.clone(),
        position: junction.position,
        importance: importance_of(junction.roads[0]).max(importance_of(junction.roads[1])),
        on_river: false,
    }));
    candidates.sort_by_cached_key(|candidate| {
        (
            Reverse(candidate.importance),
            Reverse(candidate.on_river),
            geometry::squared_centimetres(candidate.position, centre),
            candidate.name.clone(),
        )
    });
    if let Some(chosen) = candidates.first() {
        let passed_over = candidates.iter().find(|other| {
            !other.on_river
                && other.importance == chosen.importance
                && other.position != chosen.position
        });
        let notice = passed_over
            .filter(|_| chosen.on_river)
            .map(|other| CoreNotice {
                chosen: chosen.name.clone(),
                passed_over: other.name.clone(),
            });
        return core_at(
            chosen.position,
            Some(chosen.object),
            CoreRule::Crossing,
            notice,
        );
    }
    if let Some(top) = roads.iter().map(|(road, _)| road.importance).max() {
        let on_roads = roads
            .iter()
            .filter(|(road, _)| road.importance == top)
            .map(|(road, course)| (road.identifier.as_str(), course.identifier, &course.course));
        let (object, position) = nearest_on(on_roads, centre);
        return core_at(position, Some(object), CoreRule::OneRoad, None);
    }
    if rivers.is_empty() {
        return core_at(centre, None, CoreRule::SiteCentre, None);
    }
    let on_rivers = rivers
        .iter()
        .map(|(river, course)| (river.identifier.as_str(), course.identifier, &course.course));
    let (object, position) = nearest_on(on_rivers, centre);
    core_at(position, Some(object), CoreRule::River, None)
}

fn nearest_on<'a>(
    lines: impl Iterator<Item = (&'a str, Identifier, &'a Polyline)>,
    centre: Point,
) -> (Identifier, Point) {
    let (_, _, object, position) = lines
        .map(|(name, object, line)| {
            let point = geometry::closest_point(line, centre);
            (
                geometry::squared_centimetres(point, centre),
                name,
                object,
                point,
            )
        })
        .min_by(|first, second| first.0.cmp(&second.0).then(first.1.cmp(second.1)))
        .expect("the caller passes at least one line");
    (object, position)
}

fn core_at(
    position: Point,
    on: Option<Identifier>,
    rule: CoreRule,
    notice: Option<CoreNotice>,
) -> TownCore {
    TownCore {
        identifier: Identifier::new(IdentifierKind::Core, &[], &["core"]),
        position,
        on,
        rule,
        notice,
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::site;
    use citymap_specification::specification::{
        CompassRegion, Defences, Era, Identity, LandCover, Presets, RiverCrossingKind, RoadEnd,
        Seed, Settlement, Site, Specification, Terrain, Water,
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

    fn road(identifier: &str, ends: [RoadEnd; 2], importance: Importance) -> Road {
        Road {
            identifier: identifier.to_string(),
            ends,
            importance,
            river_crossing: RiverCrossingKind::Bridge,
        }
    }

    fn edges(first: CompassRegion, second: CompassRegion) -> [RoadEnd; 2] {
        [RoadEnd::Region(first), RoadEnd::Region(second)]
    }

    fn core_of(rivers: Vec<River>, roads: Vec<Road>) -> TownCore {
        site::run(&Specification {
            schema: 1,
            identity: Identity {
                name: "Harrowmere".to_string(),
                seed: Seed::new(1),
                notes: String::new(),
            },
            settlement: Settlement { population: 6000 },
            site: Site {
                extent_metres: length(2400.0),
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
            roads,
            presets: Presets::new(Era::EarlyMedieval),
        })
        .core
    }

    fn ash() -> River {
        river("ash", CompassRegion::NorthWest, CompassRegion::East)
    }

    fn north_road(importance: Importance) -> Road {
        road(
            "north_road",
            edges(CompassRegion::North, CompassRegion::South),
            importance,
        )
    }

    fn market_road(importance: Importance) -> Road {
        road(
            "market_road",
            edges(CompassRegion::SouthWest, CompassRegion::NorthEast),
            importance,
        )
    }

    #[test]
    fn the_example_core_is_on_the_market_bridge_with_a_notice() {
        let core = core_of(
            vec![ash()],
            vec![
                north_road(Importance::Major),
                road(
                    "west_road",
                    [RoadEnd::Region(CompassRegion::West), RoadEnd::Core],
                    Importance::Minor,
                ),
                market_road(Importance::Major),
            ],
        );
        assert_eq!(core.position, point(1600.0, 1600.0));
        assert_eq!(core.rule, CoreRule::Crossing);
        assert_eq!(
            core.on,
            Some(Identifier::new(
                IdentifierKind::Crossing,
                &[],
                &["market_road.ash"]
            ))
        );
        assert_eq!(
            core.notice,
            Some(CoreNotice {
                chosen: "market_road.ash".to_string(),
                passed_over: "market_road+north_road".to_string(),
            })
        );
        assert_eq!(
            core.identifier,
            Identifier::new(IdentifierKind::Core, &[], &["core"])
        );
    }

    #[test]
    fn a_road_crossing_counts_at_its_more_important_road() {
        let core = core_of(
            vec![ash()],
            vec![
                north_road(Importance::Minor),
                market_road(Importance::Major),
            ],
        );
        assert_eq!(core.position, point(1600.0, 1600.0));
        assert_eq!(
            core.notice.map(|notice| notice.passed_over),
            Some("market_road+north_road".to_string())
        );
    }

    #[test]
    fn a_road_crossing_of_major_roads_beats_a_river_crossing_of_a_minor_road() {
        let core = core_of(
            vec![river("ash", CompassRegion::North, CompassRegion::East)],
            vec![
                north_road(Importance::Major),
                road(
                    "east_road",
                    edges(CompassRegion::West, CompassRegion::East),
                    Importance::Major,
                ),
                road(
                    "ford_lane",
                    edges(CompassRegion::SouthWest, CompassRegion::NorthEast),
                    Importance::Minor,
                ),
            ],
        );
        assert_eq!(core.rule, CoreRule::Crossing);
        assert_eq!(core.position, point(1200.0, 1200.0));
    }

    #[test]
    fn a_river_crossing_that_outranks_by_importance_gives_no_notice() {
        let core = core_of(
            vec![ash()],
            vec![
                north_road(Importance::Major),
                road(
                    "a_lane",
                    edges(CompassRegion::North, CompassRegion::SouthEast),
                    Importance::Minor,
                ),
                road(
                    "b_lane",
                    edges(CompassRegion::South, CompassRegion::NorthEast),
                    Importance::Minor,
                ),
            ],
        );
        assert_eq!(core.position, point(1200.0, 1800.0));
        assert_eq!(core.notice, None);
    }

    #[test]
    fn roads_that_only_share_an_end_give_no_candidate() {
        let core = core_of(
            Vec::new(),
            vec![
                road(
                    "high_road",
                    edges(CompassRegion::West, CompassRegion::North),
                    Importance::Major,
                ),
                road(
                    "low_road",
                    edges(CompassRegion::West, CompassRegion::South),
                    Importance::Major,
                ),
            ],
        );
        assert_eq!(core.rule, CoreRule::OneRoad);
    }

    #[test]
    fn a_tie_between_top_roads_goes_to_the_first_name() {
        let core = core_of(
            Vec::new(),
            vec![
                road(
                    "b_road",
                    edges(CompassRegion::NorthWest, CompassRegion::NorthEast),
                    Importance::Major,
                ),
                road(
                    "a_road",
                    edges(CompassRegion::SouthWest, CompassRegion::SouthEast),
                    Importance::Major,
                ),
            ],
        );
        assert_eq!(core.position, point(1200.0, 0.0));
    }

    #[test]
    fn without_a_river_no_notice_is_given() {
        let core = core_of(
            Vec::new(),
            vec![
                north_road(Importance::Major),
                market_road(Importance::Major),
            ],
        );
        assert_eq!(core.position, point(1200.0, 1200.0));
        assert_eq!(core.notice, None);
        assert_eq!(
            core.on,
            Some(Identifier::new(
                IdentifierKind::Junction,
                &[],
                &["market_road+north_road"]
            ))
        );
    }

    #[test]
    fn without_a_crossing_the_core_is_nearest_the_centre_on_the_top_road() {
        let core = core_of(
            Vec::new(),
            vec![
                road(
                    "high_road",
                    edges(CompassRegion::NorthWest, CompassRegion::NorthEast),
                    Importance::Major,
                ),
                road(
                    "low_road",
                    edges(CompassRegion::West, CompassRegion::SouthWest),
                    Importance::Minor,
                ),
            ],
        );
        assert_eq!(core.rule, CoreRule::OneRoad);
        assert_eq!(core.position, point(1200.0, 2400.0));
        assert_eq!(
            core.on,
            Some(Identifier::new(IdentifierKind::Road, &[], &["high_road"]))
        );
    }

    #[test]
    fn with_only_core_roads_the_core_is_the_river_point_nearest_the_centre() {
        let core_road = road(
            "west_road",
            [RoadEnd::Region(CompassRegion::West), RoadEnd::Core],
            Importance::Major,
        );
        let core = core_of(
            vec![river("ash", CompassRegion::North, CompassRegion::South)],
            vec![core_road.clone()],
        );
        assert_eq!(core.rule, CoreRule::River);
        assert_eq!(core.position, point(1200.0, 1200.0));
        assert_eq!(
            core.on,
            Some(Identifier::new(IdentifierKind::River, &[], &["ash"]))
        );
        let core = core_of(Vec::new(), vec![core_road]);
        assert_eq!(core.rule, CoreRule::SiteCentre);
        assert_eq!(core.position, point(1200.0, 1200.0));
        assert_eq!(core.on, None);
    }

    #[test]
    fn an_exact_tie_goes_to_the_alphabetically_first_name() {
        let core = core_of(
            vec![
                river("beck", CompassRegion::West, CompassRegion::NorthEast),
                river("alder", CompassRegion::West, CompassRegion::SouthEast),
            ],
            vec![north_road(Importance::Major)],
        );
        assert_eq!(core.position, point(1200.0, 600.0));
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

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(64))]

        #[test]
        fn renaming_in_the_same_alphabetical_order_never_moves_the_core(
            rivers in prop::collection::btree_map("[a-z]{1,8}", any_two_edges(), 0..4),
            roads in prop::collection::btree_map("[a-z]{1,8}", (any_two_edges(), any::<bool>()), 0..4),
        ) {
            let build = |prefix: &str| {
                let rivers = rivers
                    .iter()
                    .map(|(name, &(from, to))| river(&format!("{prefix}{name}"), from, to))
                    .collect();
                let roads = roads
                    .iter()
                    .map(|(name, &((first, second), major))| {
                        let importance = if major { Importance::Major } else { Importance::Minor };
                        road(&format!("{prefix}{name}"), edges(first, second), importance)
                    })
                    .collect();
                core_of(rivers, roads).position
            };
            prop_assert_eq!(build(""), build("renamed_"));
        }

        #[test]
        fn the_core_lies_on_the_site(
            rivers in prop::collection::btree_map("[a-z]{1,8}", any_two_edges(), 0..4),
            roads in prop::collection::btree_map("[a-z]{1,8}", (any_two_edges(), any::<bool>()), 0..4),
        ) {
            let rivers = rivers.into_iter().map(|(name, (from, to))| river(&name, from, to)).collect();
            let roads = roads
                .into_iter()
                .map(|(name, ((first, second), major))| {
                    let importance = if major { Importance::Major } else { Importance::Minor };
                    road(&name, edges(first, second), importance)
                })
                .collect();
            let core = core_of(rivers, roads);
            prop_assert!((0.0..=2400.0).contains(&core.position.x.get()));
            prop_assert!((0.0..=2400.0).contains(&core.position.y.get()));
        }
    }
}
