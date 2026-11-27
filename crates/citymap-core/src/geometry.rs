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
//! - `geo::LineString`, `geo::Point` and `ClosestPoint::closest_point`, with its result
//!   `geo::Closest`: the point of a polyline nearest to another point.
//!
//! - `geo::Polygon`, `geo::MultiPolygon`, `geo::MultiPoint`: the shapes passed into `geo`.
//! - `Voronoi::voronoi_cells_with_params` with `VoronoiParams` and `VoronoiClip::Polygon`
//!   (feature `spade`, ADR-081): the Voronoi regions of the ward sites, clipped to the site.
//! - `BooleanOps::union`, `intersection` and `difference`: cutting wards by water.
//! - `Buffer::buffer`: a river's course widened to its width.
//! - `Area::unsigned_area`, `BoundingRect::bounding_rect`, `Intersects::intersects`: area,
//!   bounds and point tests of polygons.
//!
//! `span_through`, the ordering in `crossings_along` and the strips of collinear sites in
//! `voronoi_regions` use our own `+ - * / sqrt`.

use geo::algorithm::closest_point::ClosestPoint;
use geo::algorithm::line_intersection::{LineIntersection, line_intersection};
use geo::{
    Area as _, BooleanOps, BoundingRect, Buffer, Closest, Coord, Intersects, Line, LineString,
    MultiPoint, MultiPolygon, Voronoi, VoronoiClip, VoronoiParams,
};
use serde::Serialize;

use citymap_specification::units::SignedLength;

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

/// One place where a road crosses a river, with the road segment it lies on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Crossing {
    /// Where the road crosses, rounded to the centimetre.
    pub position: Point,
    /// The start and end of the road segment the crossing lies on, in the road's direction.
    pub road_segment: (Point, Point),
}

/// Where `road` crosses `river`, in order along `road` from its first point, rounded to the
/// centimetre. A touch at an end of either line is no crossing, and neither is a stretch where
/// the two lie on one line (ADR-048).
///
/// # Panics
///
/// Never: `geo` returns points inside both segments' bounding boxes, which are valid lengths.
pub fn crossings_along(road: &Polyline, river: &Polyline) -> Vec<Crossing> {
    let ends = [
        road.points[0],
        road.points[road.points.len() - 1],
        river.points[0],
        river.points[river.points.len() - 1],
    ];
    let mut crossings: Vec<Crossing> = Vec::new();
    for road_segment in road.segments() {
        let mut on_segment: Vec<Point> = river
            .segments()
            .filter_map(|river_segment| {
                let (lower, upper) = if road_segment <= river_segment {
                    (road_segment, river_segment)
                } else {
                    (river_segment, road_segment)
                };
                match line_intersection(line_from(lower), line_from(upper)) {
                    Some(LineIntersection::SinglePoint { intersection, .. }) => {
                        Some(point_from(intersection))
                    }
                    Some(LineIntersection::Collinear { .. }) | None => None,
                }
            })
            .filter(|position| !ends.contains(position))
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

/// The point of `polyline` nearest to `target`, rounded to the centimetre; on a tie, the one
/// `geo` meets first along the polyline. A polyline whose points are all equal gives that point.
///
/// # Panics
///
/// Never: the nearest point lies on the polyline, which holds valid lengths.
pub fn closest_point(polyline: &Polyline, target: Point) -> Point {
    let line_string: LineString<f64> = polyline
        .points
        .iter()
        .map(|point| coordinate_from(*point))
        .collect();
    match line_string.closest_point(&geo::Point::from(coordinate_from(target))) {
        Closest::Intersection(point) | Closest::SinglePoint(point) => point_from(point.0),
        Closest::Indeterminate => polyline.points[0],
    }
}

/// The squared distance between two points in whole square centimetres, so comparisons are
/// exact (`CODING_STANDARDS.md`, "Rust conventions").
pub fn squared_centimetres(first: Point, second: Point) -> i128 {
    let east = i128::from(second.x.steps() - first.x.steps());
    let north = i128::from(second.y.steps() - first.y.steps());
    east * east + north * north
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

/// The smallest piece of a cut polygon that is kept, in square metres; a smaller piece is a
/// sliver of rounding, not a place (ADR-020, zero-area polygons rejected; ADR-081).
pub const SMALLEST_PIECE_SQUARE_METRES: f64 = 1.0;

/// A polygon in the model plane: an outer ring and any holes, each ring an open list of at
/// least three points (the first point is not repeated), rounded to the centimetre.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Polygon {
    exterior: Vec<Point>,
    holes: Vec<Vec<Point>>,
}

impl Polygon {
    /// The outer ring.
    pub fn exterior(&self) -> &[Point] {
        &self.exterior
    }

    /// The rings of the holes; empty for most polygons.
    pub fn holes(&self) -> &[Vec<Point>] {
        &self.holes
    }

    /// The area in square metres, holes taken off.
    pub fn area_square_metres(&self) -> f64 {
        polygon_to_geo(self).unsigned_area()
    }

    /// Whether `point` lies inside the polygon or on its boundary.
    pub fn covers(&self, point: Point) -> bool {
        polygon_to_geo(self).intersects(&geo::Point::from(coordinate_from(point)))
    }
}

/// A region of the plane made of any number of polygons, such as the sea or the rivers of a
/// site; it exists to cut other polygons with `split`.
#[derive(Clone, Debug)]
pub struct Area {
    shape: MultiPolygon<f64>,
}

impl Area {
    /// The region with nothing in it.
    pub fn empty() -> Area {
        Area {
            shape: MultiPolygon::new(Vec::new()),
        }
    }

    /// The region inside the closed rings, such as the sea areas of a ground; a ring whose
    /// first and last points differ is closed by a straight segment.
    pub fn from_rings(rings: &[Polyline]) -> Area {
        rings.iter().fold(Area::empty(), |area, ring| {
            let exterior: LineString<f64> = ring
                .points
                .iter()
                .map(|point| coordinate_from(*point))
                .collect();
            let polygon = geo::Polygon::new(exterior, Vec::new());
            area.union(&Area {
                shape: MultiPolygon::new(vec![polygon]),
            })
        })
    }

    /// The band of `width` metres centred on `line`, such as a river.
    pub fn widened(line: &Polyline, width: f64) -> Area {
        let line_string: LineString<f64> = line
            .points
            .iter()
            .map(|point| coordinate_from(*point))
            .collect();
        Area {
            shape: line_string.buffer(width / 2.0),
        }
    }

    /// Everything that lies in either region.
    #[must_use]
    pub fn union(&self, other: &Area) -> Area {
        Area {
            shape: self.shape.union(&other.shape),
        }
    }
}

/// The parts of `polygon` inside `area` and outside it, each rounded to the centimetre;
/// pieces smaller than `SMALLEST_PIECE_SQUARE_METRES` are dropped.
pub fn split(polygon: &Polygon, area: &Area) -> (Vec<Polygon>, Vec<Polygon>) {
    let own = MultiPolygon::new(vec![polygon_to_geo(polygon)]);
    let apart = match (own.bounding_rect(), area.shape.bounding_rect()) {
        (Some(first), Some(second)) => !first.intersects(&second),
        _ => true,
    };
    if apart {
        return (Vec::new(), vec![polygon.clone()]);
    }
    (
        polygons_from(&own.intersection(&area.shape)),
        polygons_from(&own.difference(&area.shape)),
    )
}

/// The Voronoi region of every site, in the order of `sites`, clipped to the square from
/// (0, 0) to (`side`, `side`): the points nearer that site than any other (ADR-081, B1).
/// A region is `None` when it rounds to fewer than three distinct points, as for two sites a
/// centimetre apart, or when its site lies outside the square.
pub fn voronoi_regions(sites: &[Point], side: f64) -> Vec<Option<Polygon>> {
    let square = square_polygon(side);
    if sites.len() == 1 {
        return vec![region_from(&square)];
    }
    let multi_point: MultiPoint<f64> = sites
        .iter()
        .map(|site| geo::Point::from(coordinate_from(*site)))
        .collect();
    let Ok(cells) = multi_point
        .voronoi_cells_with_params(VoronoiParams::new().clip(VoronoiClip::Polygon(&square)))
    else {
        return strips(sites, &square);
    };
    let mut by_x: Vec<(usize, Point)> = sites.iter().copied().enumerate().collect();
    by_x.sort_by_key(|(_, site)| site.x);
    let mut regions: Vec<Option<Polygon>> = vec![None; sites.len()];
    for cell in &cells {
        let Some(bounds) = cell.bounding_rect() else {
            continue;
        };
        let from = by_x.partition_point(|(_, site)| site.x.get() < bounds.min().x);
        let owner = by_x[from..]
            .iter()
            .take_while(|(_, site)| site.x.get() <= bounds.max().x)
            .find(|(_, site)| cell.intersects(&geo::Point::from(coordinate_from(*site))));
        if let Some(&(index, _)) = owner {
            regions[index] = region_from(cell);
        }
    }
    regions
}

fn strips(sites: &[Point], square: &geo::Polygon<f64>) -> Vec<Option<Polygon>> {
    sites
        .iter()
        .map(|&site| {
            let mut ring = square.exterior().0.clone();
            for &other in sites.iter().filter(|other| **other != site) {
                ring = nearer_half(&ring, site, other);
            }
            region_from(&geo::Polygon::new(LineString::from(ring), Vec::new()))
        })
        .collect()
}

fn nearer_half(ring: &[Coord<f64>], site: Point, other: Point) -> Vec<Coord<f64>> {
    let (near, far) = (coordinate_from(site), coordinate_from(other));
    let (middle_x, middle_y) = (f64::midpoint(near.x, far.x), f64::midpoint(near.y, far.y));
    let beyond = |coordinate: Coord<f64>| {
        (coordinate.x - middle_x) * (far.x - near.x) + (coordinate.y - middle_y) * (far.y - near.y)
    };
    let open = &ring[..ring.len().saturating_sub(1)];
    let mut kept = Vec::new();
    for (index, &current) in open.iter().enumerate() {
        let next = open[(index + 1) % open.len()];
        let (here, there) = (beyond(current), beyond(next));
        if here <= 0.0 {
            kept.push(current);
        }
        if (here < 0.0 && there > 0.0) || (here > 0.0 && there < 0.0) {
            let share = here / (here - there);
            kept.push(Coord {
                x: current.x + share * (next.x - current.x),
                y: current.y + share * (next.y - current.y),
            });
        }
    }
    if let Some(&first) = kept.first() {
        kept.push(first);
    }
    kept
}

fn square_polygon(side: f64) -> geo::Polygon<f64> {
    geo::Polygon::new(
        LineString::from(vec![
            (0.0, 0.0),
            (side, 0.0),
            (side, side),
            (0.0, side),
            (0.0, 0.0),
        ]),
        Vec::new(),
    )
}

fn polygons_from(shape: &MultiPolygon<f64>) -> Vec<Polygon> {
    shape.0.iter().filter_map(polygon_from).collect()
}

fn region_from(polygon: &geo::Polygon<f64>) -> Option<Polygon> {
    let exterior = ring_from(polygon.exterior())?;
    let holes = polygon.interiors().iter().filter_map(ring_from).collect();
    Some(Polygon { exterior, holes })
}

fn polygon_from(polygon: &geo::Polygon<f64>) -> Option<Polygon> {
    let exterior = ring_from(polygon.exterior())?;
    let holes = polygon.interiors().iter().filter_map(ring_from).collect();
    let rounded = Polygon { exterior, holes };
    (rounded.area_square_metres() >= SMALLEST_PIECE_SQUARE_METRES).then_some(rounded)
}

fn ring_from(ring: &LineString<f64>) -> Option<Vec<Point>> {
    let mut points: Vec<Point> = Vec::new();
    for coordinate in &ring.0 {
        let point = point_from(*coordinate);
        if points.last() != Some(&point) {
            points.push(point);
        }
    }
    if points.len() > 1 && points.first() == points.last() {
        points.pop();
    }
    (points.len() >= 3).then_some(points)
}

fn polygon_to_geo(polygon: &Polygon) -> geo::Polygon<f64> {
    let ring = |points: &[Point]| -> LineString<f64> {
        let mut coordinates: Vec<Coord<f64>> =
            points.iter().map(|point| coordinate_from(*point)).collect();
        if let Some(&first) = coordinates.first() {
            coordinates.push(first);
        }
        LineString::from(coordinates)
    };
    geo::Polygon::new(
        ring(&polygon.exterior),
        polygon.holes.iter().map(|hole| ring(hole)).collect(),
    )
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

    fn positions(road: &Polyline, river: &Polyline) -> Vec<Point> {
        crossings_along(road, river)
            .iter()
            .map(|crossing| crossing.position)
            .collect()
    }

    fn polyline(coordinates: &[(f64, f64)]) -> Polyline {
        Polyline::new(coordinates.iter().map(|&(x, y)| point(x, y)).collect())
            .expect("test polyline has at least two points")
    }

    fn areas(regions: &[Option<Polygon>]) -> Vec<f64> {
        regions
            .iter()
            .map(|region| region.as_ref().map_or(0.0, Polygon::area_square_metres))
            .collect()
    }

    #[test]
    fn four_sites_in_a_square_get_its_four_quarters() {
        let sites = [
            point(25.0, 25.0),
            point(75.0, 25.0),
            point(25.0, 75.0),
            point(75.0, 75.0),
        ];
        let regions = voronoi_regions(&sites, 100.0);
        assert_eq!(areas(&regions), vec![2500.0; 4]);
        for (site, region) in sites.iter().zip(&regions) {
            assert!(region.as_ref().is_some_and(|region| region.covers(*site)));
        }
    }

    #[test]
    fn sites_on_one_line_get_strips_of_the_square() {
        let sites = [point(10.0, 50.0), point(50.0, 50.0), point(90.0, 50.0)];
        let regions = voronoi_regions(&sites, 100.0);
        assert_eq!(areas(&regions), vec![3000.0, 4000.0, 3000.0]);
    }

    #[test]
    fn one_site_owns_the_whole_square() {
        assert_eq!(
            areas(&voronoi_regions(&[point(3.0, 4.0)], 10.0)),
            vec![100.0]
        );
    }

    #[test]
    fn a_polygon_splits_into_the_parts_inside_and_outside_an_area() {
        let square = voronoi_regions(&[point(50.0, 50.0)], 100.0)[0]
            .clone()
            .expect("one site owns the square");
        let band = Area::widened(&polyline(&[(50.0, -10.0), (50.0, 110.0)]), 20.0);
        let (inside, outside) = split(&square, &band);
        let sum = |parts: &[Polygon]| parts.iter().map(Polygon::area_square_metres).sum::<f64>();
        assert_eq!(inside.len(), 1);
        assert_eq!(outside.len(), 2, "the band cuts the square in two");
        assert!((sum(&inside) - 2000.0).abs() < 1.0, "{}", sum(&inside));
        assert!((sum(&outside) - 8000.0).abs() < 1.0, "{}", sum(&outside));
    }

    #[test]
    fn a_polygon_away_from_an_area_stays_whole() {
        let square = voronoi_regions(&[point(5.0, 5.0)], 10.0)[0]
            .clone()
            .expect("one site owns the square");
        let far = Area::widened(&polyline(&[(500.0, 0.0), (500.0, 10.0)]), 2.0);
        assert_eq!(split(&square, &far), (Vec::new(), vec![square]));
    }

    #[test]
    fn two_crossing_segments_meet_in_one_point() {
        let first = polyline(&[(0.0, 0.0), (10.0, 10.0)]);
        let second = polyline(&[(0.0, 10.0), (10.0, 0.0)]);
        assert_eq!(positions(&first, &second), vec![point(5.0, 5.0)]);
    }

    #[test]
    fn a_crossing_point_is_rounded_to_the_centimetre() {
        let first = polyline(&[(0.0, 0.0), (3.0, 1.0)]);
        let second = polyline(&[(1.0, 0.0), (1.0, 3.0)]);
        assert_eq!(positions(&first, &second), vec![point(1.0, 0.33)]);
    }

    #[test]
    fn parallel_segments_that_do_not_touch_do_not_meet() {
        let first = polyline(&[(0.0, 0.0), (5.0, 5.0)]);
        let second = polyline(&[(0.0, 1.0), (5.0, 6.0)]);
        assert_eq!(positions(&first, &second), Vec::<Point>::new());
    }

    #[test]
    fn polylines_crossing_several_times_give_every_crossing_in_order() {
        let zigzag = polyline(&[(0.0, 0.0), (2.0, 4.0), (4.0, 0.0), (6.0, 4.0)]);
        let horizontal = polyline(&[(-1.0, 2.0), (3.0, 2.0), (7.0, 2.0)]);
        assert_eq!(
            positions(&zigzag, &horizontal),
            vec![point(1.0, 2.0), point(3.0, 2.0), point(5.0, 2.0)]
        );
    }

    #[test]
    fn a_shared_vertex_is_listed_once() {
        let first = polyline(&[(0.0, 0.0), (5.0, 5.0), (10.0, 0.0)]);
        let second = polyline(&[(5.0, 0.0), (5.0, 10.0)]);
        assert_eq!(positions(&first, &second), vec![point(5.0, 5.0)]);
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
    fn a_river_that_starts_on_the_road_is_no_crossing() {
        let road = polyline(&[(0.0, 10.0), (20.0, 10.0)]);
        let river = polyline(&[(10.0, 10.0), (10.0, 0.0)]);
        assert_eq!(crossings_along(&road, &river), Vec::new());
    }

    #[test]
    fn a_crossing_at_a_road_vertex_is_listed_once() {
        let road = polyline(&[(0.0, 0.0), (5.0, 5.0), (10.0, 10.0)]);
        let river = polyline(&[(0.0, 10.0), (10.0, 0.0)]);
        assert_eq!(positions(&road, &river), [point(5.0, 5.0)]);
    }

    #[test]
    fn a_crossing_at_a_river_vertex_is_listed_once() {
        let road = polyline(&[(0.0, 10.0), (10.0, 0.0)]);
        let river = polyline(&[(0.0, 0.0), (5.0, 5.0), (10.0, 10.0)]);
        assert_eq!(positions(&road, &river), [point(5.0, 5.0)]);
    }

    #[test]
    fn the_closest_point_is_the_foot_of_the_perpendicular_or_an_end() {
        let line = polyline(&[(0.0, 0.0), (10.0, 0.0)]);
        assert_eq!(closest_point(&line, point(4.0, 3.0)), point(4.0, 0.0));
        assert_eq!(closest_point(&line, point(-5.0, 3.0)), point(0.0, 0.0));
        let still = polyline(&[(2.0, 2.0), (2.0, 2.0)]);
        assert_eq!(closest_point(&still, point(9.0, 9.0)), point(2.0, 2.0));
    }

    #[test]
    fn squared_centimetres_are_exact_whole_numbers() {
        assert_eq!(
            squared_centimetres(point(0.0, 0.0), point(3.0, 4.0)),
            250_000
        );
        assert_eq!(squared_centimetres(point(0.01, 0.0), point(0.0, 0.0)), 1);
        assert_eq!(
            squared_centimetres(point(1.0, 2.0), point(4.0, 6.0)),
            250_000,
            "neither point at the origin"
        );
    }

    #[test]
    fn crossings_on_one_road_segment_come_in_order_along_it_whatever_the_river_order() {
        let road = polyline(&[(0.0, 2.0), (10.0, 2.0)]);
        let river = polyline(&[(8.0, 0.0), (8.0, 4.0), (2.0, 0.0)]);
        assert_eq!(
            positions(&road, &river),
            vec![point(5.0, 2.0), point(8.0, 2.0)],
            "the river meets x = 8 first, but the road meets x = 5 first"
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
            let mut forwards = positions(&first, &second);
            let mut backwards = positions(&second, &first);
            forwards.sort_unstable();
            backwards.sort_unstable();
            prop_assert_eq!(forwards, backwards);
        }

        #[test]
        fn every_intersection_lies_within_both_bounding_boxes(
            first in polyline_strategy(),
            second in polyline_strategy(),
        ) {
            for intersection in positions(&first, &second) {
                prop_assert!(lies_within_bounding_box(intersection, &first));
                prop_assert!(lies_within_bounding_box(intersection, &second));
            }
        }
    }
}
