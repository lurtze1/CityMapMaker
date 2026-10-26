//! Stage S0, the site: the land before the city (ARCHITECTURE.md §4). Milestone 0b builds
//! only the river courses: each river runs straight from the middle of its `from` edge stretch
//! to the middle of its `to` edge stretch (ADR-039).

use serde::Serialize;

use crate::geometry::{Point, Polyline};
use crate::identifier::{self, Identifier, IdentifierKind};
use crate::specification::{CompassRegion, River, Specification};
use crate::units::{Length, SignedLength};

/// The output of stage S0.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SiteOutput {
    /// One course per river, in the order of the specification.
    pub rivers: Vec<RiverCourse>,
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

/// Runs stage S0 on a validated specification.
///
/// # Panics
///
/// In a debug build, when two rivers share an identifier; validation rejects that first.
pub fn run(specification: &Specification) -> SiteOutput {
    let extent = specification.site.extent_metres;
    let rivers: Vec<RiverCourse> = specification
        .site
        .water
        .rivers
        .iter()
        .map(|river| river_course(river, extent))
        .collect();
    let identifiers: Vec<Identifier> = rivers.iter().map(|river| river.identifier).collect();
    identifier::debug_assert_unique(&identifiers);
    SiteOutput { rivers }
}

fn river_course(river: &River, extent: Length) -> RiverCourse {
    let course = Polyline::new(vec![
        stretch_middle(river.from, extent),
        stretch_middle(river.to, extent),
    ])
    .expect("two points always make a polyline");
    RiverCourse {
        identifier: Identifier::new(IdentifierKind::River, &[], &[&river.identifier]),
        course,
        width_metres: river.width_metres,
    }
}

/// The middle of a region's edge stretch: the side's middle for north, east, south and west,
/// the corner for the four diagonals, and the site's centre for `Centre` (ADR-039).
fn stretch_middle(region: CompassRegion, extent: Length) -> Point {
    let (east, north) = match region {
        CompassRegion::North => (0.0, 1.0),
        CompassRegion::NorthEast => (1.0, 1.0),
        CompassRegion::East => (1.0, 0.0),
        CompassRegion::SouthEast => (1.0, -1.0),
        CompassRegion::South => (0.0, -1.0),
        CompassRegion::SouthWest => (-1.0, -1.0),
        CompassRegion::West => (-1.0, 0.0),
        CompassRegion::NorthWest => (-1.0, 1.0),
        CompassRegion::Centre => (0.0, 0.0),
    };
    let half = extent.get() / 2.0;
    let coordinate = |direction: f64| {
        SignedLength::new(half + direction * half).expect("a point on the site is a valid length")
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
    use crate::random::Seed;
    use crate::specification::{Era, Identity, Presets, Settlement, Site, Water};

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
                water: Water { rivers },
            },
            presets: Presets {
                era: Era::EarlyMedieval,
            },
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

    fn any_specification() -> impl Strategy<Value = Specification> {
        let rivers = prop::collection::btree_map("[a-z]{1,8}", (any_region(), any_region()), 0..6)
            .prop_map(|rivers| {
                rivers
                    .into_iter()
                    .map(|(identifier, (from, to))| river(&identifier, from, to))
                    .collect()
            });
        (1.0..1e6_f64, rivers).prop_map(|(extent, rivers)| specification(extent, rivers))
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(64))]

        #[test]
        fn the_same_specification_gives_the_same_output(specification in any_specification()) {
            prop_assert_eq!(run(&specification), run(&specification));
        }

        #[test]
        fn river_identifiers_are_unique(specification in any_specification()) {
            let output = run(&specification);
            let identifiers: Vec<Identifier> =
                output.rivers.iter().map(|river| river.identifier).collect();
            prop_assert!(identifier::check_unique(&identifiers).is_ok());
        }

        #[test]
        fn every_course_stays_on_the_site(specification in any_specification()) {
            let extent = specification.site.extent_metres.get();
            for river in run(&specification).rivers {
                for point in river.course.points() {
                    prop_assert!((0.0..=extent).contains(&point.x.get()));
                    prop_assert!((0.0..=extent).contains(&point.y.get()));
                }
            }
        }
    }
}
