//! The coast of stage S0 (ADR-076, Q4): the coast's shape word draws the sea as an area, the
//! land within `SHORE_RAMP_METRES` of a beach runs down to sea level, and the ground under the
//! sea falls away from the shore. Sea level is 0 m. Every pass 1 proportion is a named
//! constant that the specification overrides per coast (ADR-075).

use std::f64::consts::SQRT_2;

use citymap_specification::direction::toward;
use citymap_specification::specification::{Cliff, Coast, CoastShape};
use citymap_specification::units::{Fraction, Length, SignedLength};
use serde::Serialize;

use super::{CENTIMETRES_PER_METRE, GridShape, Line, Terrain, whole_centimetres};
use crate::geometry::{Point, Polyline};
use crate::identifier::{Identifier, IdentifierKind};

/// How far the town's shore lies from the centre toward the coast's bearing when the file gives
/// no distance, as a share of the half-side (ADR-076).
const SHORE_DISTANCE_FRACTION: f64 = 0.6;

/// How far a bay reaches into the land when the file gives no depth, as a share of the
/// half-side (ADR-076).
const BAY_DEPTH_FRACTION: f64 = 0.5;

/// A bay's depth divided by its width when the file gives no width; Dublin Bay's 7 km by 10 km
/// (ADR-076).
const BAY_DEPTH_TO_WIDTH: f64 = 0.7;

/// How wide a strait's sea is when the file gives no width, as a share of the half-side
/// (ADR-076).
const STRAIT_WIDTH_FRACTION: f64 = 0.3;

/// How far from a beach the land runs down to sea level (ADR-076, Q4).
const SHORE_RAMP_METRES: f64 = 100.0;

/// How fast the sea floor falls away from the shore, in metres per metre (ADR-076, NN5).
const SEA_FLOOR_FALL: f64 = 0.05;

/// The furthest the sea floor falls below the ground's own height (ADR-076, NN5).
const SEA_FLOOR_DEEPEST_METRES: f64 = 30.0;

/// How far below sea level a point of the sea lies at least (ADR-076, NN5).
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

impl Sea {
    /// The sea of `coast`; `None` when the coast's bearing is not valid. An estuary is drawn
    /// as an open coast until #280.
    pub(super) fn new(coast: &Coast, half_side: f64) -> Option<Sea> {
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
            CoastShape::Open | CoastShape::Estuary => Shape::Open,
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
        Some(Sea {
            shore,
            degrees,
            shape,
        })
    }

    /// How far `(x, y)` lies inside the land, in metres: the distance to the shore for a
    /// straight shore, and a smooth stand-in for it around a bay's curve; below 0 at sea. The
    /// shoreline is where it is 0.
    fn level(&self, x: f64, y: f64) -> f64 {
        let beyond = self.shore.beyond(x, y);
        match self.shape {
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
        }
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

/// A cliff standing on the shore: along it the land keeps its height instead of running down
/// to sea level (ADR-076, OO1).
pub(super) struct ShoreCliff {
    pub(super) line: Line,
    pub(super) low_side: f64,
}

impl ShoreCliff {
    fn guards(&self, x: f64, y: f64, shore_metres: f64, spacing_metres: f64) -> bool {
        let beyond = self.line.beyond(x, y) * self.low_side;
        beyond <= 0.0 && -beyond <= shore_metres + spacing_metres
    }
}

/// Shapes `centimetres` by the sea: the shore ramp on the land and the sea floor under the
/// water. Returns the sea's `level` at every grid point in centimetres, positive on land and
/// negative at sea.
pub(super) fn shape_ground(
    sea: &Sea,
    shore_cliffs: &[ShoreCliff],
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
    let is_sea: Vec<bool> = level.iter().map(|&level| level < 0).collect();
    let to_sea = distances_to(&is_sea, grid);
    let is_land: Vec<bool> = is_sea.iter().map(|sea| !sea).collect();
    let to_land = distances_to(&is_land, grid);
    let half_square = grid.spacing_metres / 2.0;
    for row in 0..points {
        for column in 0..points {
            let index = row * points + column;
            let (x, y) = grid.position(row, column);
            let height = &mut centimetres[index];
            if is_sea[index] {
                let out = f64::from(to_land[index]) - half_square;
                let fall = (out * SEA_FLOOR_FALL).min(SEA_FLOOR_DEEPEST_METRES);
                let floor = *height - whole_centimetres(fall);
                *height = if floor >= 0 {
                    -whole_centimetres(SEA_FLOOR_SHALLOWEST_METRES)
                } else {
                    floor
                };
            } else {
                let shore = f64::from(to_sea[index]) - half_square;
                let guarded = shore_cliffs
                    .iter()
                    .any(|cliff| cliff.guards(x, y, shore, grid.spacing_metres));
                if shore < SHORE_RAMP_METRES && !guarded {
                    *height = whole_centimetres(
                        f64::from(*height) / CENTIMETRES_PER_METRE * shore / SHORE_RAMP_METRES,
                    );
                }
            }
        }
    }
    level
}

/// The distance from every grid point to the nearest point in `targets`, in metres, by a
/// two-pass chamfer over the eight neighbours; infinite when there is no target.
fn distances_to(targets: &[bool], grid: GridShape) -> Vec<f32> {
    let points = grid.points_per_side;
    let straight = grid.spacing_metres as f32;
    let diagonal = (grid.spacing_metres * SQRT_2) as f32;
    let mut distance: Vec<f32> = targets
        .iter()
        .map(|&target| if target { 0.0 } else { f32::INFINITY })
        .collect();
    let forward = [
        (0, -1, straight),
        (-1, -1, diagonal),
        (-1, 0, straight),
        (-1, 1, diagonal),
    ];
    for row in 0..points {
        for column in 0..points {
            relax(&mut distance, points, row, column, &forward);
        }
    }
    let backward = [
        (0, 1, straight),
        (1, 1, diagonal),
        (1, 0, straight),
        (1, -1, diagonal),
    ];
    for row in (0..points).rev() {
        for column in (0..points).rev() {
            relax(&mut distance, points, row, column, &backward);
        }
    }
    distance
}

fn relax(
    distance: &mut [f32],
    points: usize,
    row: usize,
    column: usize,
    neighbours: &[(isize, isize, f32)],
) {
    let index = row * points + column;
    for &(rows, columns, step) in neighbours {
        let (Some(other_row), Some(other_column)) = (
            row.checked_add_signed(rows),
            column.checked_add_signed(columns),
        ) else {
            continue;
        };
        if other_row < points && other_column < points {
            let through = distance[other_row * points + other_column] + step;
            if through < distance[index] {
                distance[index] = through;
            }
        }
    }
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
    let value = |row: usize, column: usize| signed[row * points + column];
    let mut segments = Vec::new();
    for row in 0..points - 1 {
        for column in 0..points - 1 {
            square_segments(row, column, &value, &mut segments);
        }
    }
    let crossing = |edge: Edge| {
        let (horizontal, row, column) = edge;
        let (other_row, other_column) = if horizontal {
            (row, column + 1)
        } else {
            (row + 1, column)
        };
        let (here, there) = (value(row, column), value(other_row, other_column));
        let share = f64::from(here) / f64::from(here - there);
        let (x, y) = grid.position(row, column);
        let (other_x, other_y) = grid.position(other_row, other_column);
        site_point(
            x + share * (other_x - x) + grid.centre,
            y + share * (other_y - y) + grid.centre,
        )
    };
    let mut pieces: Vec<Shoreline> = chains(&segments)
        .into_iter()
        .filter_map(|chain| {
            let course = straightened(chain.into_iter().map(crossing).collect());
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

/// The segments of one grid square, each from one edge crossing to another with the land on
/// its left. A square with sea on two opposite corners is decided by the mean of its corners.
fn square_segments(
    row: usize,
    column: usize,
    value: &impl Fn(usize, usize) -> i32,
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
            let mean: i64 = corners.iter().map(|&corner| i64::from(corner)).sum();
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
    use std::collections::BTreeMap;
    let by_start: BTreeMap<Edge, usize> = segments
        .iter()
        .enumerate()
        .map(|(index, segment)| (segment.from, index))
        .collect();
    let ends: std::collections::BTreeSet<Edge> =
        segments.iter().map(|segment| segment.to).collect();
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
fn straightened(course: Vec<Point>) -> Vec<Point> {
    let mut kept: Vec<Point> = Vec::with_capacity(course.len());
    for point in course {
        while kept.len() >= 2 {
            let (first, middle) = (kept[kept.len() - 2], kept[kept.len() - 1]);
            if off_line(first, middle, point) > 0.01 {
                break;
            }
            kept.pop();
        }
        kept.push(point);
    }
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
