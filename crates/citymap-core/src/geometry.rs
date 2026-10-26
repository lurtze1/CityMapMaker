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
