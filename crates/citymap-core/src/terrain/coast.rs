//! The coast of stage S0 (ADR-076, Q4): the coast's shape word draws the sea as an area, the
//! land within `SHORE_RAMP_METRES` of a beach runs down to sea level, and the ground under the
//! sea falls away from the shore. Sea level is 0 m. Every pass 1 proportion is a named
//! constant that the specification overrides per coast (ADR-075).

use std::collections::{BTreeMap, BTreeSet};

use citymap_specification::direction::{power, toward};
use citymap_specification::specification::{Cliff, Coast, CoastShape, River, Specification};
use citymap_specification::units::{Fraction, Length, SignedLength};
use citymap_specification::validation::{
    ESTUARY_MOUTH_FRACTION, SHORE_DISTANCE_FRACTION, reaches_the_sea,
};
use serde::Serialize;

use super::{CENTIMETRES_PER_METRE, CliffDrop, GridShape, Line, Terrain, whole_centimetres};
use crate::geometry::{Point, Polyline};
use crate::identifier::{Identifier, IdentifierKind};
use crate::site::stretch_middle;

/// How far a bay reaches into the land when the file gives no depth, as a share of the
/// half-side (ADR-076).
const BAY_DEPTH_FRACTION: f64 = 0.5;

/// A bay's depth divided by its width when the file gives no width; Dublin Bay's 7 km by 10 km
/// (ADR-076).
const BAY_DEPTH_TO_WIDTH: f64 = 0.7;

/// How wide a strait's sea is when the file gives no width, as a share of the half-side
/// (ADR-076).
const STRAIT_WIDTH_FRACTION: f64 = 0.3;

/// How far an estuary reaches inland when the file gives no reach, as a share of the
/// half-side (ADR-076, MM4).
const ESTUARY_REACH_FRACTION: f64 = 0.5;

/// How far from a beach the land runs down to sea level (ADR-076, Q4).
const SHORE_RAMP_METRES: f64 = 100.0;

/// How fast the sea floor falls away from the shore, in metres per metre (ADR-076, NN5).
const SEA_FLOOR_FALL: f64 = 0.05;

/// The furthest the sea floor falls below the ground's own height (ADR-076, NN5).
const SEA_FLOOR_DEEPEST_METRES: f64 = 30.0;

/// How far below sea level a point of the sea is put when its floor would lie at or above
/// sea level (ADR-076, NN5).
const SEA_FLOOR_SHALLOWEST_METRES: f64 = 1.0;

/// How far, in degrees, a cliff without a distance may face away from the coast's bearing and
/// still stand on the shore (ADR-076, Q4).
const SHORE_CLIFF_DEGREES: u16 = 45;

/// One piece of the shoreline. Walking along `course`, the land is on the left and the sea on
/// the right, as OpenStreetMap draws coastlines; a closed piece ends where it starts
/// (ADR-076, NN5).
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Shoreline {
    /// The piece's stable identifier, of kind `shoreline`, under its ground.
    pub identifier: Identifier,
    /// The points of the shoreline, without points that lie on a straight line between their
    /// neighbours.
    pub course: Polyline,
}

/// The sea of a coast: the town's shore line and the shape cut from the land.
pub(super) struct Sea {
    shore: Line,
    degrees: u16,
    shape: Shape,
    estuaries: Vec<Trumpet>,
}

enum Shape {
    Open,
    Strait {
        width_metres: f64,
    },
    Bay {
        depth_metres: f64,
        half_width_metres: f64,
    },
}

/// An estuary's sea along its river: from the mouth on the shore it narrows inland as
/// mouth × (river ÷ mouth)^(inland ÷ reach), a convergent estuary (ADR-076, LL1, MM4).
struct Trumpet {
    mouth_x: f64,
    mouth_y: f64,
    inland_x: f64,
    inland_y: f64,
    reach_metres: f64,
    mouth_half_width_metres: f64,
    river_half_width_metres: f64,
}

impl Trumpet {
    /// The trumpet along `river`, from where its course crosses the shore toward its `from`;
    /// `None` when the course does not cross the shore from land to sea.
    fn new(
        shore: Line,
        river: &River,
        half_side: f64,
        (mouth_metres, reach_metres): (f64, f64),
    ) -> Option<Trumpet> {
        let extent = Length::new(half_side * 2.0).ok()?;
        {
            let end = |region| {
                let point = stretch_middle(region, extent);
                (point.x.get() - half_side, point.y.get() - half_side)
            };
            let ((from_x, from_y), (to_x, to_y)) = (end(river.from), end(river.to));
            let (inland, seaward) = (shore.beyond(from_x, from_y), shore.beyond(to_x, to_y));
            if inland >= 0.0 || seaward <= 0.0 {
                return None;
            }
            let share = inland / (inland - seaward);
            let (mouth_x, mouth_y) = (
                from_x + share * (to_x - from_x),
                from_y + share * (to_y - from_y),
            );
            let (walk_x, walk_y) = (from_x - mouth_x, from_y - mouth_y);
            let length = (walk_x * walk_x + walk_y * walk_y).sqrt();
            Some(Trumpet {
                mouth_x,
                mouth_y,
                inland_x: walk_x / length,
                inland_y: walk_y / length,
                reach_metres,
                mouth_half_width_metres: mouth_metres / 2.0,
                river_half_width_metres: river.width_metres.get() / 2.0,
            })
        }
    }

    /// How far `(x, y)` lies outside the trumpet, in metres across it; below 0 inside.
    fn level(&self, x: f64, y: f64) -> f64 {
        let (offset_x, offset_y) = (x - self.mouth_x, y - self.mouth_y);
        let inland = offset_x * self.inland_x + offset_y * self.inland_y;
        let aside = (offset_x * self.inland_y - offset_y * self.inland_x).abs();
        let share = (inland / self.reach_metres).clamp(0.0, 1.0);
        let narrowing = self.river_half_width_metres / self.mouth_half_width_metres;
        let half_width = self.mouth_half_width_metres * power(narrowing, share);
        (aside - half_width).max(inland - self.reach_metres)
    }
}

impl Sea {
    /// The sea of the site's coast, with an estuary for every river whose mouth is one and
    /// that reaches the sea; `None` without a coast or when the coast's bearing is not valid.
    pub(super) fn new(specification: &Specification, half_side: f64) -> Option<Sea> {
        let coast: &Coast = specification.site.terrain.coast.as_ref()?;
        let degrees = coast.bearing_degrees.degrees()?;
        let share = coast
            .distance_fraction
            .map_or(SHORE_DISTANCE_FRACTION, Fraction::get);
        let shore = Line {
            step: toward(degrees),
            distance_metres: coast
                .distance_metres
                .map_or(share * half_side, SignedLength::get),
        };
        let measure = |(fraction, metres): (Option<Fraction>, Option<Length>), default: f64| {
            metres.map_or_else(
                || fraction.map_or(default, Fraction::get) * half_side,
                Length::get,
            )
        };
        let width = (coast.width_fraction, coast.width_metres);
        let shape = match coast.shape {
            CoastShape::Open => Shape::Open,
            CoastShape::Strait => Shape::Strait {
                width_metres: measure(width, STRAIT_WIDTH_FRACTION),
            },
            CoastShape::Bay => {
                let depth_metres = measure(
                    (coast.depth_fraction, coast.depth_metres),
                    BAY_DEPTH_FRACTION,
                );
                let width_metres = measure(width, depth_metres / BAY_DEPTH_TO_WIDTH / half_side);
                Shape::Bay {
                    depth_metres,
                    half_width_metres: width_metres / 2.0,
                }
            }
        };
        let estuaries = specification
            .site
            .water
            .rivers
            .iter()
            .filter(|river| reaches_the_sea(specification, river))
            .filter_map(|river| {
                let mouth = river.mouth.as_ref()?;
                let width = measure(
                    (mouth.width_fraction, mouth.width_metres),
                    ESTUARY_MOUTH_FRACTION,
                );
                let reach = measure(
                    (mouth.reach_fraction, mouth.reach_metres),
                    ESTUARY_REACH_FRACTION,
                );
                Trumpet::new(shore, river, half_side, (width, reach))
            })
            .collect();
        Some(Sea {
            shore,
            degrees,
            shape,
            estuaries,
        })
    }

    /// How far `(x, y)` lies inside the land, in metres: the distance to the shore for a
    /// straight shore, and a smooth stand-in for it around a bay's curve; below 0 at sea. The
    /// shoreline is where it is 0.
    fn level(&self, x: f64, y: f64) -> f64 {
        let beyond = self.shore.beyond(x, y);
        let shape = match self.shape {
            Shape::Open => -beyond,
            Shape::Strait { width_metres } => (-beyond).max(beyond - width_metres),
            Shape::Bay {
                depth_metres,
                half_width_metres,
            } => {
                let along = -x * self.shore.step.y + y * self.shore.step.x;
                let (across, inland) = (along / half_width_metres, -beyond / depth_metres);
                let ellipse = ((across * across + inland * inland).sqrt() - 1.0)
                    * depth_metres.min(half_width_metres);
                (-beyond).min(ellipse)
            }
        };
        self.estuaries
            .iter()
            .map(|trumpet| trumpet.level(x, y))
            .fold(shape, f64::min)
    }

    /// The distance from the centre at which `cliff` stands on the town's shore, when it has
    /// no distance of its own and faces within `SHORE_CLIFF_DEGREES` of the coast; its line
    /// then crosses the shore on the coast's bearing (ADR-076, Q4).
    pub(super) fn shore_cliff_distance(&self, cliff: &Cliff) -> Option<SignedLength> {
        if cliff.distance_fraction.is_some() || cliff.distance_metres.is_some() {
            return None;
        }
        let degrees = cliff.bearing_degrees.degrees()?;
        let apart = self.degrees.abs_diff(degrees);
        if apart.min(360 - apart) > SHORE_CLIFF_DEGREES {
            return None;
        }
        let facing = toward(degrees);
        let cosine = facing.x * self.shore.step.x + facing.y * self.shore.step.y;
        SignedLength::new(self.shore.distance_metres * cosine).ok()
    }
}

/// Whether `(x, y)`, `shore_metres` inland, lies on the high side of a cliff standing on the
/// shore and no further from it than from the sea: there the land keeps its height instead of
/// running down to sea level (ADR-076, OO1).
fn guarded_by(cliff: &CliffDrop, x: f64, y: f64, shore_metres: f64, spacing_metres: f64) -> bool {
    let beyond = cliff.line.beyond(x, y) * cliff.low_side;
    cliff.on_shore && beyond <= 0.0 && -beyond <= shore_metres + spacing_metres
}

/// Shapes `centimetres` by the sea: the shore ramp on the land and the sea floor under the
/// water. Returns the sea's `level` at every grid point in centimetres, positive on land and
/// negative at sea.
pub(super) fn shape_ground(
    sea: &Sea,
    cliffs: &[CliffDrop],
    grid: GridShape,
    centimetres: &mut [i32],
) -> Vec<i32> {
    let points = grid.points_per_side;
    let mut level = Vec::with_capacity(points * points);
    for row in 0..points {
        for column in 0..points {
            let (x, y) = grid.position(row, column);
            level.push(whole_centimetres(sea.level(x, y)));
        }
    }
    for row in 0..points {
        for column in 0..points {
            let index = row * points + column;
            let (x, y) = grid.position(row, column);
            let inland = f64::from(level[index]) / CENTIMETRES_PER_METRE;
            let height = &mut centimetres[index];
            if inland < 0.0 {
                let fall = (-inland * SEA_FLOOR_FALL).min(SEA_FLOOR_DEEPEST_METRES);
                let floor = *height - whole_centimetres(fall);
                *height = if floor >= 0 {
                    -whole_centimetres(SEA_FLOOR_SHALLOWEST_METRES)
                } else {
                    floor
                };
            } else {
                let guarded = cliffs
                    .iter()
                    .any(|cliff| guarded_by(cliff, x, y, inland, grid.spacing_metres));
                if inland < SHORE_RAMP_METRES && !guarded {
                    *height = whole_centimetres(
                        f64::from(*height) / CENTIMETRES_PER_METRE * inland / SHORE_RAMP_METRES,
                    );
                }
            }
        }
    }
    level
}

/// An edge of the grid between two neighbouring points: horizontal from (row, column) to
/// (row, column + 1), or vertical from (row, column) to (row + 1, column).
type Edge = (bool, usize, usize);

struct Segment {
    from: Edge,
    to: Edge,
}

/// The shoreline pieces of `signed`, the sea's level at every grid point,
/// traced square by square (marching squares) and joined into polylines with the land on the
/// left (ADR-076).
pub(super) fn shorelines(signed: &[i32], grid: GridShape) -> Vec<Shoreline> {
    let points = grid.points_per_side;
    let value = |row: usize, column: usize| i64::from(signed[row * points + column]);
    let mut pieces: Vec<Shoreline> = traced(&value, grid)
        .into_iter()
        .map(|course| straightened(&course))
        .filter_map(|course| {
            let first = course[0];
            let key = format!("{} {}", first.x.get(), first.y.get());
            Some(Shoreline {
                identifier: Identifier::new(
                    IdentifierKind::Shoreline,
                    &[Terrain::surface()],
                    &[&key],
                ),
                course: Polyline::new(course).ok()?,
            })
        })
        .collect();
    pieces.sort_by_key(|piece| piece.identifier);
    pieces
}

/// The lines where `value` crosses 0 on the grid, traced square by square (marching squares)
/// and joined into courses with the side at or above 0 on the left, one point per grid edge
/// crossed. A closed course ends where it starts.
pub(super) fn traced(value: &impl Fn(usize, usize) -> i64, grid: GridShape) -> Vec<Vec<Point>> {
    traced_at_levels(value, &[0], grid)
        .pop()
        .unwrap_or_default()
}

/// The lines where `value` crosses each of `levels`, ascending, as `traced` gives them for one
/// level, in one sweep over the grid: a square is traced only at the levels between its
/// lowest and its highest corner.
pub(super) fn traced_at_levels(
    value: &impl Fn(usize, usize) -> i64,
    levels: &[i64],
    grid: GridShape,
) -> Vec<Vec<Vec<Point>>> {
    let points = grid.points_per_side;
    let mut segments: Vec<Vec<Segment>> = levels.iter().map(|_| Vec::new()).collect();
    for row in 0..points - 1 {
        for column in 0..points - 1 {
            let corners = [
                value(row, column),
                value(row, column + 1),
                value(row + 1, column + 1),
                value(row + 1, column),
            ];
            let low = corners.iter().copied().min().unwrap_or(0);
            let high = corners.iter().copied().max().unwrap_or(0);
            let first = levels.partition_point(|&level| level <= low);
            let last = levels.partition_point(|&level| level <= high);
            for (index, &level) in levels.iter().enumerate().take(last).skip(first) {
                let shifted = |row: usize, column: usize| value(row, column) - level;
                square_segments(row, column, &shifted, &mut segments[index]);
            }
        }
    }
    levels
        .iter()
        .zip(segments)
        .map(|(&level, segments)| {
            let crossing = |edge: Edge| {
                let (horizontal, row, column) = edge;
                let (other_row, other_column) = if horizontal {
                    (row, column + 1)
                } else {
                    (row + 1, column)
                };
                let here = value(row, column) - level;
                let there = value(other_row, other_column) - level;
                let share = here as f64 / (here - there) as f64;
                let (x, y) = grid.position(row, column);
                let (other_x, other_y) = grid.position(other_row, other_column);
                site_point(
                    x + share * (other_x - x) + grid.centre,
                    y + share * (other_y - y) + grid.centre,
                )
            };
            chains(&segments)
                .into_iter()
                .map(|chain| chain.into_iter().map(crossing).collect())
                .collect()
        })
        .collect()
}

/// The segments of one grid square, each from one edge crossing to another with the land on
/// its left. A square with sea on two opposite corners is decided by the mean of its corners.
fn square_segments(
    row: usize,
    column: usize,
    value: &impl Fn(usize, usize) -> i64,
    segments: &mut Vec<Segment>,
) {
    let corners = [
        value(row, column),
        value(row, column + 1),
        value(row + 1, column + 1),
        value(row + 1, column),
    ];
    let sea = corners.map(|corner| corner < 0);
    let edges = [
        (true, row, column),
        (false, row, column + 1),
        (true, row + 1, column),
        (false, row, column),
    ];
    let crossed: Vec<usize> = (0..4)
        .filter(|&edge| sea[edge] != sea[(edge + 1) % 4])
        .collect();
    match crossed.len() {
        2 => segments.push(oriented(&edges, crossed[0], crossed[1], 0, sea[0])),
        4 => {
            let mean: i64 = corners.iter().sum();
            let centre_is_sea = mean < 0;
            for (corner, &corner_is_sea) in sea.iter().enumerate() {
                if corner_is_sea != centre_is_sea {
                    let (before, after) = ((corner + 3) % 4, corner);
                    segments.push(oriented(&edges, before, after, corner, corner_is_sea));
                }
            }
        }
        _ => {}
    }
}

/// The segment between edges `first` and `second`, turned so that the land lies on its left.
/// `corner` is a corner of the square, never on the segment, and `corner_is_sea` its kind.
fn oriented(
    edges: &[Edge; 4],
    first: usize,
    second: usize,
    corner: usize,
    corner_is_sea: bool,
) -> Segment {
    let place = |edge: usize| match edge {
        0 => (0.5, 0.0),
        1 => (1.0, 0.5),
        2 => (0.5, 1.0),
        _ => (0.0, 0.5),
    };
    let corner_place = match corner {
        0 => (0.0, 0.0),
        1 => (1.0, 0.0),
        2 => (1.0, 1.0),
        _ => (0.0, 1.0),
    };
    let (from, to) = (place(first), place(second));
    let (walk_x, walk_y) = (to.0 - from.0, to.1 - from.1);
    let (corner_x, corner_y) = (corner_place.0 - from.0, corner_place.1 - from.1);
    let corner_on_left = walk_x * corner_y - walk_y * corner_x > 0.0;
    if corner_on_left == corner_is_sea {
        Segment {
            from: edges[second],
            to: edges[first],
        }
    } else {
        Segment {
            from: edges[first],
            to: edges[second],
        }
    }
}

/// Joins segments that share an edge into chains of edges, each in walking order: first the
/// open chains, each from an edge no segment ends at, then the closed ones, which end where
/// they start.
fn chains(segments: &[Segment]) -> Vec<Vec<Edge>> {
    let by_start: BTreeMap<Edge, usize> = segments
        .iter()
        .enumerate()
        .map(|(index, segment)| (segment.from, index))
        .collect();
    let ends: BTreeSet<Edge> = segments.iter().map(|segment| segment.to).collect();
    let mut used = vec![false; segments.len()];
    let mut chains = Vec::new();
    let open_starts = segments
        .iter()
        .enumerate()
        .filter(|(_, segment)| !ends.contains(&segment.from))
        .map(|(index, _)| index);
    let every_start: Vec<usize> = open_starts.chain(0..segments.len()).collect();
    for start in every_start {
        if used[start] {
            continue;
        }
        let mut chain = vec![segments[start].from];
        let mut current = start;
        loop {
            used[current] = true;
            let end = segments[current].to;
            chain.push(end);
            match by_start.get(&end) {
                Some(&next) if !used[next] => current = next,
                _ => break,
            }
        }
        chains.push(chain);
    }
    chains
}

/// `course` without the points that lie within a centimetre of the straight line between the
/// points around them.
pub(super) fn straightened(course: &[Point]) -> Vec<Point> {
    let mut kept = vec![course[0]];
    let mut run: Vec<Point> = Vec::new();
    for &point in &course[1..] {
        let anchor = kept[kept.len() - 1];
        if run
            .iter()
            .all(|&middle| off_line(anchor, middle, point) <= 0.01)
        {
            run.push(point);
        } else {
            kept.push(run[run.len() - 1]);
            run = vec![point];
        }
    }
    kept.extend(run.last());
    kept
}

fn off_line(first: Point, middle: Point, last: Point) -> f64 {
    let (walk_x, walk_y) = (last.x.get() - first.x.get(), last.y.get() - first.y.get());
    let length = (walk_x * walk_x + walk_y * walk_y).sqrt();
    if length == 0.0 {
        return f64::INFINITY;
    }
    let (middle_x, middle_y) = (
        middle.x.get() - first.x.get(),
        middle.y.get() - first.y.get(),
    );
    (walk_x * middle_y - walk_y * middle_x).abs() / length
}

fn site_point(x: f64, y: f64) -> Point {
    Point {
        x: SignedLength::new(x).expect("a point on the site is a valid length"),
        y: SignedLength::new(y).expect("a point on the site is a valid length"),
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::{Trumpet, off_line, site_point, straightened};

    fn trumpet() -> Trumpet {
        Trumpet {
            mouth_x: 300.0,
            mouth_y: 0.0,
            inland_x: -1.0,
            inland_y: 0.0,
            reach_metres: 250.0,
            mouth_half_width_metres: 100.0,
            river_half_width_metres: 10.0,
        }
    }

    proptest! {
        #[test]
        fn an_estuary_only_narrows_inland_and_is_no_sea_past_its_reach(
            inland in -50.0..400.0_f64,
            further in 0.0..100.0_f64,
            aside in -150.0..150.0_f64,
        ) {
            let trumpet = trumpet();
            let level = |inland: f64| trumpet.level(300.0 - inland, aside);
            if level(inland + further) < 0.0 {
                prop_assert!(level(inland) < 0.0, "sea further inland, but land nearer the mouth");
            }
            if inland > 250.0 {
                prop_assert!(level(inland) > 0.0);
            }
        }
    }

    #[test]
    fn points_on_a_straight_line_are_dropped() {
        let course =
            [(0.0, 0.0), (1.0, 0.0), (2.0, 0.0), (3.0, 1.0)].map(|(x, y)| site_point(x, y));
        assert_eq!(straightened(&course), [course[0], course[2], course[3]]);
    }

    #[test]
    fn every_dropped_point_stays_within_a_centimetre_of_the_kept_course() {
        let course: Vec<_> = (0..=20)
            .map(|step| {
                let x = f64::from(step) * 0.5;
                site_point(x, 0.002 * x * x)
            })
            .collect();
        let kept = straightened(&course);
        assert!(kept.len() > 2, "a bend of 5 cm keeps points");
        for point in &course {
            let segment = kept
                .windows(2)
                .find(|pair| (pair[0].x.get()..=pair[1].x.get()).contains(&point.x.get()))
                .expect("the kept course spans every point");
            assert!(
                off_line(segment[0], *point, segment[1]) <= 0.01,
                "{point:?}"
            );
        }
    }
}
