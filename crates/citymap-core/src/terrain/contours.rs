//! Contour lines and sea areas of a ground, computed on request from its private height grid
//! for whoever draws the terrain (ADR-076, X1, CCC1, DDD1). Neither is stored in the model.

use citymap_specification::units::SignedLength;

use super::{CENTIMETRES_PER_METRE, CliffLine, GridShape, Ground, Terrain, coast};
use crate::geometry::{Point, Polyline};
use crate::identifier::Identifier;

/// One contour line: where the ground is `level_metres` high. Walking along `course`, the
/// higher ground is on the left; a closed line ends where it starts. A line stops within one
/// grid square of a cliff line, so a cliff is drawn by its own stroke (ADR-076, X1).
#[derive(Clone, Debug, PartialEq)]
pub struct Contour {
    /// The height of the line, in metres.
    pub level_metres: SignedLength,
    /// The line's points.
    pub course: Polyline,
}

impl Terrain {
    /// The contour lines of `ground` at every multiple of `interval` above its lowest point and
    /// up to its highest, lowest level first; empty for a ground the site does not have or an
    /// interval of 0 (ADR-076, CCC1).
    ///
    /// # Panics
    ///
    /// Never: every level is a whole number of centimetres within the ground's own heights.
    pub fn contours(&self, ground: Identifier, interval: SignedLength) -> Vec<Contour> {
        let Some(found) = self
            .grounds
            .iter()
            .find(|candidate| candidate.identifier == ground)
        else {
            return Vec::new();
        };
        let step = interval.steps().abs();
        if step == 0 {
            return Vec::new();
        }
        let grid = found.grid_shape();
        let points = grid.points_per_side;
        let heights = &found.grid.centimetres;
        let lowest = heights.iter().copied().min().map_or(0, i64::from);
        let highest = heights.iter().copied().max().map_or(0, i64::from);
        let mut levels = Vec::new();
        let mut level = (lowest.div_euclid(step) + 1) * step;
        while level <= highest {
            levels.push(level);
            level += step;
        }
        let value = |row: usize, column: usize| i64::from(heights[row * points + column]);
        let mut contours = Vec::new();
        for (&level, courses) in levels
            .iter()
            .zip(coast::traced_at_levels(&value, &levels, grid))
        {
            let level_metres = SignedLength::new(level as f64 / CENTIMETRES_PER_METRE)
                .expect("a height in whole centimetres is a valid length");
            for course in courses {
                for piece in away_from_cliffs(&course, &found.cliff_lines, grid.spacing_metres) {
                    if let Ok(course) = Polyline::new(coast::straightened(&piece)) {
                        contours.push(Contour {
                            level_metres,
                            course,
                        });
                    }
                }
            }
        }
        contours
    }

    /// The sea of `ground` as closed areas: each open shoreline piece closed along the site's
    /// edge with the sea on its right, and each closed piece that holds sea; empty without a
    /// coast or for a ground the site does not have (ADR-076, DDD1). An island, a closed piece
    /// with land inside, is not cut out; the coast shapes of pass 1 make none.
    ///
    /// # Panics
    ///
    /// Never: every shoreline piece has at least two points.
    pub fn sea_areas(&self, ground: Identifier) -> Vec<Polyline> {
        let Some(found) = self
            .grounds
            .iter()
            .find(|candidate| candidate.identifier == ground)
        else {
            return Vec::new();
        };
        let extent = found.grid.extent_metres;
        let courses: Vec<&[Point]> = found
            .shorelines
            .iter()
            .map(|shoreline| shoreline.course.points())
            .collect();
        let (closed, open): (Vec<&[Point]>, Vec<&[Point]>) = courses
            .into_iter()
            .partition(|points| points.first() == points.last());
        let mut areas: Vec<Polyline> = closed
            .into_iter()
            .filter(|points| twice_area(points) < 0.0)
            .filter_map(|points| Polyline::new(points.to_vec()).ok())
            .collect();
        let mut used = vec![false; open.len()];
        for first in 0..open.len() {
            if used[first] {
                continue;
            }
            let mut area: Vec<Point> = Vec::new();
            let mut current = first;
            loop {
                used[current] = true;
                area.extend_from_slice(open[current]);
                let end = around(*area.last().expect("a piece has points"), extent);
                let next = (0..open.len())
                    .filter(|&candidate| candidate == first || !used[candidate])
                    .min_by(|&one, &other| {
                        let ahead = |index: usize| {
                            (around(open[index][0], extent) - end).rem_euclid(4.0 * extent)
                        };
                        ahead(one).total_cmp(&ahead(other))
                    })
                    .expect("the first piece is always a candidate");
                let start = around(open[next][0], extent);
                area.extend(corners_between(end, start, extent));
                if next == first {
                    area.push(open[first][0]);
                    break;
                }
                current = next;
            }
            if let Ok(polyline) = Polyline::new(area) {
                areas.push(polyline);
            }
        }
        areas
    }
}

impl Ground {
    fn grid_shape(&self) -> GridShape {
        let points_per_side = self.grid.points_per_side;
        GridShape {
            points_per_side,
            spacing_metres: self.grid.extent_metres / (points_per_side - 1) as f64,
            centre: self.grid.extent_metres / 2.0,
        }
    }
}

/// How far clockwise along the site's edge `point` lies, from the south-west corner up the
/// west edge, along the north edge, down the east edge and back along the south edge.
fn around(point: Point, extent: f64) -> f64 {
    let (x, y) = (point.x.get(), point.y.get());
    let distances = [
        (x, y),
        (y - extent, extent + x),
        (x - extent, 3.0 * extent - y),
        (y, 4.0 * extent - x),
    ];
    distances
        .iter()
        .min_by(|one, other| one.0.abs().total_cmp(&other.0.abs()))
        .map_or(0.0, |&(_, along)| along.rem_euclid(4.0 * extent))
}

/// The site's corners passed when walking clockwise along its edge from `from` to `to`.
fn corners_between(from: f64, to: f64, extent: f64) -> Vec<Point> {
    let corner = |x: f64, y: f64| Point {
        x: SignedLength::new(x).expect("a corner is a valid length"),
        y: SignedLength::new(y).expect("a corner is a valid length"),
    };
    let corners = [
        (extent, corner(0.0, extent)),
        (2.0 * extent, corner(extent, extent)),
        (3.0 * extent, corner(extent, 0.0)),
        (4.0 * extent, corner(0.0, 0.0)),
    ];
    let span = (to - from).rem_euclid(4.0 * extent);
    let mut passed: Vec<(f64, Point)> = corners
        .iter()
        .map(|&(at, point)| ((at - from).rem_euclid(4.0 * extent), point))
        .filter(|&(ahead, _)| ahead > 0.0 && ahead < span)
        .collect();
    passed.sort_by(|one, other| one.0.total_cmp(&other.0));
    passed.into_iter().map(|(_, point)| point).collect()
}

/// Twice the signed area of a closed course: below 0 when it runs clockwise.
fn twice_area(points: &[Point]) -> f64 {
    points
        .windows(2)
        .map(|pair| pair[0].x.get() * pair[1].y.get() - pair[1].x.get() * pair[0].y.get())
        .sum()
}

/// `course` cut into the pieces that lie further than `spacing_metres` from every cliff line.
fn away_from_cliffs(
    course: &[Point],
    cliffs: &[CliffLine],
    spacing_metres: f64,
) -> Vec<Vec<Point>> {
    let near = |point: Point| {
        cliffs
            .iter()
            .any(|cliff| distance_to_segment(point, cliff.from, cliff.to) <= spacing_metres)
    };
    let mut pieces = vec![Vec::new()];
    for &point in course {
        if near(point) {
            if pieces.last().is_some_and(|piece| !piece.is_empty()) {
                pieces.push(Vec::new());
            }
        } else if let Some(piece) = pieces.last_mut() {
            piece.push(point);
        }
    }
    pieces.retain(|piece| piece.len() >= 2);
    pieces
}

fn distance_to_segment(point: Point, from: Point, to: Point) -> f64 {
    let (along_x, along_y) = (to.x.get() - from.x.get(), to.y.get() - from.y.get());
    let (offset_x, offset_y) = (point.x.get() - from.x.get(), point.y.get() - from.y.get());
    let length_squared = along_x * along_x + along_y * along_y;
    let share = if length_squared == 0.0 {
        0.0
    } else {
        ((offset_x * along_x + offset_y * along_y) / length_squared).clamp(0.0, 1.0)
    };
    let (gap_x, gap_y) = (offset_x - share * along_x, offset_y - share * along_y);
    (gap_x * gap_x + gap_y * gap_y).sqrt()
}

#[cfg(test)]
mod tests {
    use super::corners_between;
    use crate::geometry::Point;
    use citymap_specification::units::SignedLength;

    fn point(x: f64, y: f64) -> Point {
        Point {
            x: SignedLength::new(x).expect("test input is a valid length"),
            y: SignedLength::new(y).expect("test input is a valid length"),
        }
    }

    #[test]
    fn walking_clockwise_passes_the_corners_in_order() {
        assert_eq!(
            corners_between(50.0, 250.0, 100.0),
            [point(0.0, 100.0), point(100.0, 100.0)],
            "from the west edge to the east edge, over the north-west and north-east corners"
        );
    }

    #[test]
    fn a_walk_past_the_south_west_corner_wraps_around() {
        assert_eq!(
            corners_between(350.0, 50.0, 100.0),
            [point(0.0, 0.0)],
            "from the south edge to the west edge"
        );
    }

    #[test]
    fn a_walk_to_the_same_place_or_onto_a_corner_adds_no_corner_twice() {
        assert_eq!(corners_between(50.0, 50.0, 100.0), []);
        assert_eq!(
            corners_between(100.0, 200.0, 100.0),
            [],
            "from corner to corner"
        );
        assert_eq!(
            corners_between(50.0, 300.0, 100.0),
            [point(0.0, 100.0), point(100.0, 100.0)]
        );
    }
}
