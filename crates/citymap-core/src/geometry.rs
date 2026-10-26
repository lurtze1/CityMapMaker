//! Geometry facade: our own point and polyline types and the geometry operations on them.
//! This is the only module that imports `geo`; callers never see a `geo` type, so the
//! backend can be replaced without touching them (ADR-020).
//!
//! # Geo functions used
//!
//! - `geo::Coord`: the point type passed into `geo`.
//! - `geo::Line` and `geo::Line::new`: one segment, built from two `Coord` values.
//! - `geo::algorithm::line_intersection::line_intersection`: where two segments meet.
//! - `geo::algorithm::line_intersection::LineIntersection`: its result, a point or an overlap.
//!
//! `span_through` and the ordering in `crossings_along` use our own `+ - * / sqrt`.

use geo::algorithm::line_intersection::{LineIntersection, line_intersection};
use geo::{Coord, Line};
use serde::Serialize;

use crate::units::SignedLength;

/// A position in the model plane, in metres from the origin.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Point {
    /// Distance east of the origin.
    pub x: SignedLength,
    /// Distance north of the origin.
    pub y: SignedLength,
}

/// An open chain of straight segments through at least two points, such as a road or a
/// river course. Consecutive points form one segment each.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct Polyline {
    points: Vec<Point>,
}

/// Why a list of points could not become a `Polyline`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum PolylineError {
    /// The list held fewer than two points, so it has no segment.
    #[error("a polyline needs at least two points, got {count}")]
    FewerThanTwoPoints {
        /// How many points the list held.
        count: usize,
    },
}

impl Polyline {
    /// Builds a polyline through `points` in the given order.
    ///
    /// # Errors
    ///
    /// `PolylineError::FewerThanTwoPoints` when `points` holds zero points or one point.
    pub fn new(points: Vec<Point>) -> Result<Polyline, PolylineError> {
        if points.len() < 2 {
            return Err(PolylineError::FewerThanTwoPoints {
                count: points.len(),
            });
        }
        Ok(Polyline { points })
    }

    /// The points of the polyline, in order.
    pub fn points(&self) -> &[Point] {
        &self.points
    }

    fn segments(&self) -> impl Iterator<Item = (Point, Point)> + '_ {
        self.points.windows(2).map(|pair| (pair[0], pair[1]))
    }
}

/// Every point where a segment of `first` crosses or touches a segment of `second`; an
/// overlap of two collinear segments gives its two end points. Points are rounded to the
/// centimetre, sorted by x then y, and listed once each.
///
/// # Panics
///
/// Never: `geo` returns points inside both segments' bounding boxes, which
/// always round to a valid `SignedLength` value.
pub fn segment_intersections(first: &Polyline, second: &Polyline) -> Vec<Point> {
    let mut intersections = Vec::new();
    for first_segment in first.segments() {
        for second_segment in second.segments() {
            let (lower, upper) = if first_segment <= second_segment {
                (first_segment, second_segment)
            } else {
                (second_segment, first_segment)
            };
            match line_intersection(line_from(lower), line_from(upper)) {
                None => {}
                Some(LineIntersection::SinglePoint { intersection, .. }) => {
                    intersections.push(point_from(intersection));
                }
                Some(LineIntersection::Collinear { intersection }) => {
                    intersections.push(point_from(intersection.start));
                    intersections.push(point_from(intersection.end));
                }
            }
        }
    }
    intersections.sort_unstable();
    intersections.dedup();
    intersections
}

/// One place where a road crosses a river, with the road segment it lies on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Crossing {
    /// Where the road crosses, rounded to the centimetre.
    pub position: Point,
    /// The start and end of the road segment the crossing lies on, in the road's direction.
    pub road_segment: (Point, Point),
}

/// Where `road` crosses `river`, in order along `road` from its first point. Touching the
/// river at one of the road's own two ends is no crossing, and neither is a stretch where the
/// two lie on one line (ADR-048).
///
/// # Panics
///
/// Never: see `segment_intersections`.
pub fn crossings_along(road: &Polyline, river: &Polyline) -> Vec<Crossing> {
    let road_ends = [road.points[0], road.points[road.points.len() - 1]];
    let mut crossings: Vec<Crossing> = Vec::new();
    for road_segment in road.segments() {
        let mut on_segment: Vec<Point> = river
            .segments()
            .filter_map(|river_segment| {
                match line_intersection(line_from(road_segment), line_from(river_segment)) {
                    Some(LineIntersection::SinglePoint { intersection, .. }) => {
                        Some(point_from(intersection))
                    }
                    Some(LineIntersection::Collinear { .. }) | None => None,
                }
            })
            .filter(|position| !road_ends.contains(position))
            .collect();
        on_segment.sort_by(|first, second| {
            squared_distance(road_segment.0, *first)
                .total_cmp(&squared_distance(road_segment.0, *second))
        });
        for position in on_segment {
            if crossings.last().map(|last| last.position) != Some(position) {
                crossings.push(Crossing {
                    position,
                    road_segment,
                });
            }
        }
    }
    crossings
}

/// A straight piece `length` long, centred on `centre` and running the way `segment` runs.
///
/// # Panics
///
/// When `segment` has two equal points, or the piece leaves `±MAXIMUM_ABSOLUTE_METRES`.
pub fn span_through(segment: (Point, Point), centre: Point, length: f64) -> Polyline {
    let (start, end) = segment;
    let east = end.x.get() - start.x.get();
    let north = end.y.get() - start.y.get();
    let size = (east * east + north * north).sqrt();
    assert!(
        size > 0.0,
        "a segment with two equal points has no direction"
    );
    let half = length / 2.0;
    let at = |sign: f64| Point {
        x: SignedLength::new(centre.x.get() + sign * half * east / size)
            .expect("a span near the site is a valid length"),
        y: SignedLength::new(centre.y.get() + sign * half * north / size)
            .expect("a span near the site is a valid length"),
    };
    Polyline {
        points: vec![at(-1.0), at(1.0)],
    }
}

fn squared_distance(first: Point, second: Point) -> f64 {
    let east = second.x.get() - first.x.get();
    let north = second.y.get() - first.y.get();
    east * east + north * north
}

fn coordinate_from(point: Point) -> Coord<f64> {
    Coord {
        x: point.x.get(),
        y: point.y.get(),
    }
}

fn line_from((start, end): (Point, Point)) -> Line<f64> {
    Line::new(coordinate_from(start), coordinate_from(end))
}

fn point_from(coordinate: Coord<f64>) -> Point {
    Point {
        x: SignedLength::new(coordinate.x)
            .expect("an intersection lies inside the input segments' bounding boxes"),
        y: SignedLength::new(coordinate.y)
            .expect("an intersection lies inside the input segments' bounding boxes"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn point(x: f64, y: f64) -> Point {
        Point {
            x: SignedLength::new(x).expect("test coordinate is a valid length"),
            y: SignedLength::new(y).expect("test coordinate is a valid length"),
        }
    }

    fn polyline(coordinates: &[(f64, f64)]) -> Polyline {
        Polyline::new(coordinates.iter().map(|&(x, y)| point(x, y)).collect())
            .expect("test polyline has at least two points")
    }

    #[test]
    fn two_crossing_segments_meet_in_one_point() {
        let first = polyline(&[(0.0, 0.0), (10.0, 10.0)]);
        let second = polyline(&[(0.0, 10.0), (10.0, 0.0)]);
        assert_eq!(
            segment_intersections(&first, &second),
            vec![point(5.0, 5.0)]
        );
    }

    #[test]
    fn a_crossing_point_is_rounded_to_the_centimetre() {
        let first = polyline(&[(0.0, 0.0), (3.0, 1.0)]);
        let second = polyline(&[(1.0, 0.0), (1.0, 3.0)]);
        assert_eq!(
            segment_intersections(&first, &second),
            vec![point(1.0, 0.33)]
        );
    }

    #[test]
    fn segments_touching_at_an_end_point_meet_there() {
        let first = polyline(&[(0.0, 0.0), (5.0, 5.0)]);
        let second = polyline(&[(5.0, 5.0), (5.0, 0.0)]);
        assert_eq!(
            segment_intersections(&first, &second),
            vec![point(5.0, 5.0)]
        );
    }

    #[test]
    fn parallel_segments_that_do_not_touch_do_not_meet() {
        let first = polyline(&[(0.0, 0.0), (5.0, 5.0)]);
        let second = polyline(&[(0.0, 1.0), (5.0, 6.0)]);
        assert_eq!(segment_intersections(&first, &second), Vec::<Point>::new());
    }

    #[test]
    fn a_collinear_overlap_gives_its_two_end_points() {
        let first = polyline(&[(0.0, 0.0), (5.0, 5.0)]);
        let second = polyline(&[(3.0, 3.0), (6.0, 6.0)]);
        assert_eq!(
            segment_intersections(&first, &second),
            vec![point(3.0, 3.0), point(5.0, 5.0)]
        );
    }

    #[test]
    fn polylines_crossing_several_times_give_every_crossing_in_order() {
        let zigzag = polyline(&[(0.0, 0.0), (2.0, 4.0), (4.0, 0.0), (6.0, 4.0)]);
        let horizontal = polyline(&[(-1.0, 2.0), (3.0, 2.0), (7.0, 2.0)]);
        assert_eq!(
            segment_intersections(&zigzag, &horizontal),
            vec![point(1.0, 2.0), point(3.0, 2.0), point(5.0, 2.0)]
        );
    }

    #[test]
    fn a_shared_vertex_is_listed_once() {
        let first = polyline(&[(0.0, 0.0), (5.0, 5.0), (10.0, 0.0)]);
        let second = polyline(&[(5.0, 0.0), (5.0, 10.0)]);
        assert_eq!(
            segment_intersections(&first, &second),
            vec![point(5.0, 5.0)]
        );
    }

    #[test]
    fn a_polyline_with_one_point_is_rejected() {
        assert_eq!(
            Polyline::new(vec![point(1.0, 1.0)]),
            Err(PolylineError::FewerThanTwoPoints { count: 1 })
        );
    }

    #[test]
    fn a_polyline_with_no_points_is_rejected() {
        assert_eq!(
            Polyline::new(Vec::new()),
            Err(PolylineError::FewerThanTwoPoints { count: 0 })
        );
    }

    #[test]
    fn crossings_come_in_order_along_the_road() {
        let road = polyline(&[(0.0, 0.0), (10.0, 10.0), (20.0, 0.0)]);
        let river = polyline(&[(0.0, 4.0), (20.0, 4.0)]);
        let positions: Vec<Point> = crossings_along(&road, &river)
            .iter()
            .map(|crossing| crossing.position)
            .collect();
        assert_eq!(positions, [point(4.0, 4.0), point(16.0, 4.0)]);
        let backwards = polyline(&[(20.0, 0.0), (10.0, 10.0), (0.0, 0.0)]);
        let positions: Vec<Point> = crossings_along(&backwards, &river)
            .iter()
            .map(|crossing| crossing.position)
            .collect();
        assert_eq!(positions, [point(16.0, 4.0), point(4.0, 4.0)]);
    }

    #[test]
    fn touching_the_river_at_a_road_end_is_no_crossing() {
        let road = polyline(&[(0.0, 0.0), (10.0, 10.0)]);
        let river = polyline(&[(0.0, 10.0), (20.0, 10.0)]);
        assert_eq!(crossings_along(&road, &river), Vec::new());
    }

    #[test]
    fn lying_on_the_river_is_no_crossing() {
        let road = polyline(&[(0.0, 0.0), (10.0, 0.0)]);
        let river = polyline(&[(-5.0, 0.0), (15.0, 0.0)]);
        assert_eq!(crossings_along(&road, &river), Vec::new());
    }

    #[test]
    fn a_crossing_carries_its_road_segment() {
        let road = polyline(&[(0.0, 0.0), (10.0, 10.0), (20.0, 0.0)]);
        let river = polyline(&[(0.0, 4.0), (20.0, 4.0)]);
        let crossings = crossings_along(&road, &river);
        assert_eq!(
            crossings[1].road_segment,
            (point(10.0, 10.0), point(20.0, 0.0))
        );
    }

    #[test]
    fn a_span_is_centred_and_runs_along_its_segment() {
        let span = span_through((point(0.0, 0.0), point(0.0, 100.0)), point(0.0, 50.0), 28.0);
        assert_eq!(span.points(), [point(0.0, 36.0), point(0.0, 64.0)]);
    }

    fn polyline_strategy() -> impl Strategy<Value = Polyline> {
        prop::collection::vec((-100.0..100.0_f64, -100.0..100.0_f64), 2..6)
            .prop_map(|coordinates| polyline(&coordinates))
    }

    fn lies_within_bounding_box(candidate: Point, polyline: &Polyline) -> bool {
        let points = polyline.points();
        let minimum_x = points.iter().map(|point| point.x).min();
        let maximum_x = points.iter().map(|point| point.x).max();
        let minimum_y = points.iter().map(|point| point.y).min();
        let maximum_y = points.iter().map(|point| point.y).max();
        Some(candidate.x) >= minimum_x
            && Some(candidate.x) <= maximum_x
            && Some(candidate.y) >= minimum_y
            && Some(candidate.y) <= maximum_y
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(64))]

        #[test]
        fn intersections_do_not_depend_on_argument_order(
            first in polyline_strategy(),
            second in polyline_strategy(),
        ) {
            prop_assert_eq!(
                segment_intersections(&first, &second),
                segment_intersections(&second, &first)
            );
        }

        #[test]
        fn every_intersection_lies_within_both_bounding_boxes(
            first in polyline_strategy(),
            second in polyline_strategy(),
        ) {
            for intersection in segment_intersections(&first, &second) {
                prop_assert!(lies_within_bounding_box(intersection, &first));
                prop_assert!(lies_within_bounding_box(intersection, &second));
            }
        }
    }
}
