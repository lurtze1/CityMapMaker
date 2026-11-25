//! The terrain of stage S0: one ground with a height field (ADR-076). Heights are whole
//! centimetres on a square grid of `presets.height_grid_points_per_side` points per side,
//! blended between the points. The grid stays private: other stages read heights only through
//! `Terrain::height_at`, so its storage can change without breaking them.

mod coast;
mod contours;

use std::fmt;

use serde::Serialize;

pub use coast::Shoreline;
pub use contours::Contour;

use crate::geometry::Point;
use crate::identifier::{Identifier, IdentifierKind};
use citymap_specification::direction::{Direction, toward};
use citymap_specification::specification::{Cliff, CompassRegion, Plateau, Slope, Specification};
use citymap_specification::units::{Fraction, Length, SignedLength};

/// The key of the one ground of version 1, the walkable surface (ADR-076, K1).
const SURFACE: &str = "surface";

/// Centimetres per metre: heights are stored in whole centimetres (ADR-076, N1).
const CENTIMETRES_PER_METRE: f64 = 100.0;

/// How far a hill or a hollow lies from the centre toward `at` when the file gives no
/// distance, as a share of the half-side (ADR-076, T1).
const POINT_DISTANCE_FRACTION: f64 = 0.5;

/// How far a cliff's or a plateau's line lies from the centre toward its bearing when the
/// file gives no distance, as a share of the half-side (ADR-076, U4).
const LINE_DISTANCE_FRACTION: f64 = 0.5;

/// How wide a plateau's ramp is, centred on its line; a pass 1 stand-in (ADR-076).
const PLATEAU_RAMP_METRES: f64 = 50.0;

/// Metres per kilometre, to turn a slope's gradient into metres per metre.
const METRES_PER_KILOMETRE: f64 = 1000.0;

/// The grounds of the site; version 1 has exactly one, `SURFACE` (REQUIREMENTS.md F2b).
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Terrain {
    /// Every ground, in identifier order.
    pub grounds: Vec<Ground>,
}

/// One walkable ground: its identifier, the size of its height grid, its height range, the
/// lines of its cliffs and its shoreline.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Ground {
    /// The ground's stable identifier, of kind `ground`.
    pub identifier: Identifier,
    /// How many height points the grid has along each side of the site.
    pub points_per_side: u32,
    /// The lowest height on the ground, in metres.
    pub lowest_metres: SignedLength,
    /// The highest height on the ground, in metres.
    pub highest_metres: SignedLength,
    /// Where each cliff crosses the site, in identifier order (ADR-076, L1).
    pub cliff_lines: Vec<CliffLine>,
    /// The pieces of the shoreline, in identifier order; empty without a coast (ADR-076).
    pub shorelines: Vec<Shoreline>,
    #[serde(skip)]
    grid: HeightGrid,
}

/// A cliff's line from one edge of the site to another. Walking from `from` to `to`, the low
/// side is on the right, as OpenStreetMap draws cliffs; `low_side_degrees` says the same as a
/// compass direction (ADR-076, FF4, II2).
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CliffLine {
    /// The cliff's stable identifier, of kind `cliff`, under its ground.
    pub identifier: Identifier,
    /// Where the line enters the site.
    pub from: Point,
    /// Where the line leaves the site.
    pub to: Point,
    /// The direction the ground drops toward, in whole degrees clockwise from north, at right
    /// angles to the line.
    pub low_side_degrees: u16,
}

#[derive(Clone, PartialEq)]
struct HeightGrid {
    points_per_side: usize,
    extent_metres: f64,
    centimetres: Vec<i32>,
    shore_centimetres: Vec<i32>,
}

impl fmt::Debug for HeightGrid {
    #[expect(
        clippy::renamed_function_params,
        reason = "full words (CODING_STANDARDS.md) instead of the standard library's `f`"
    )]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HeightGrid")
            .field("points_per_side", &self.points_per_side)
            .field("extent_metres", &self.extent_metres)
            .finish_non_exhaustive()
    }
}

impl Terrain {
    /// The identifier of the surface ground.
    pub fn surface() -> Identifier {
        Identifier::new(IdentifierKind::Ground, &[], &[SURFACE])
    }

    /// The height of `ground` at `point`, blended from the four grid points around it and
    /// rounded to the centimetre; `None` for a ground the site does not have or a point
    /// outside the site.
    pub fn height_at(&self, ground: Identifier, point: Point) -> Option<SignedLength> {
        let found = self
            .grounds
            .iter()
            .find(|candidate| candidate.identifier == ground)?;
        let centimetres = found.grid.blend(&found.grid.centimetres, point)?;
        SignedLength::new(centimetres / CENTIMETRES_PER_METRE).ok()
    }

    /// Whether `point` on `ground` lies in the sea, from the signed distance to the shore
    /// blended like a height; `None` for a ground the site does not have or a point outside
    /// the site (ADR-076, Q4).
    pub fn is_sea(&self, ground: Identifier, point: Point) -> Option<bool> {
        let found = self
            .grounds
            .iter()
            .find(|candidate| candidate.identifier == ground)?;
        if found.grid.shore_centimetres.is_empty() {
            return found
                .grid
                .blend(&found.grid.centimetres, point)
                .map(|_| false);
        }
        Some(found.grid.blend(&found.grid.shore_centimetres, point)? < 0.0)
    }
}

impl HeightGrid {
    fn blend(&self, values: &[i32], point: Point) -> Option<f64> {
        let last = self.points_per_side - 1;
        let (x, y) = (point.x.get(), point.y.get());
        let on_site = 0.0..=self.extent_metres;
        if !on_site.contains(&x) || !on_site.contains(&y) {
            return None;
        }
        let squares = last as f64 / self.extent_metres;
        let (column, across) = cell_and_offset(x * squares, last);
        let (row, up) = cell_and_offset(y * squares, last);
        let at = |column: usize, row: usize| f64::from(values[row * self.points_per_side + column]);
        let south = at(column, row) * (1.0 - across) + at(column + 1, row) * across;
        let north = at(column, row + 1) * (1.0 - across) + at(column + 1, row + 1) * across;
        Some(south * (1.0 - up) + north * up)
    }
}

fn cell_and_offset(position: f64, last: usize) -> (usize, f64) {
    let position = position.min(last as f64);
    let cell = (position.floor() as usize).min(last - 1);
    (cell, position - cell as f64)
}

/// Builds the terrain of `specification`: one ground at its base height above sea level, with
/// the slope, the hills, the hollows, the cliffs and the plateau added, then shaped by the
/// coast (ADR-076). Each feature rounds its own contribution to whole centimetres, so the sum
/// does not depend on the order of the features.
///
/// # Panics
///
/// When `presets.height_grid_points_per_side` is below 2; validation keeps it at 16 or more.
pub fn build(specification: &Specification) -> Terrain {
    let points_per_side = specification.presets.height_grid_points_per_side as usize;
    assert!(
        points_per_side >= 2,
        "a height grid has at least two points per side"
    );
    let extent = specification.site.extent_metres.get();
    let centre = extent / 2.0;
    let grid = GridShape {
        points_per_side,
        spacing_metres: extent / (points_per_side - 1) as f64,
        centre,
    };
    let terrain = &specification.site.terrain;
    let sea = terrain
        .coast
        .as_ref()
        .and_then(|_| coast::Sea::new(specification, centre));
    let shore_cliff = sea
        .as_ref()
        .and_then(|sea| shore_cliff(sea, &terrain.cliffs));
    let shore_distance = |index: usize| {
        shore_cliff.and_then(|(chosen, distance)| (chosen == index).then_some(distance))
    };
    let base = terrain.base_height_metres.map_or_else(
        || {
            shore_cliff.map_or(0, |(chosen, _)| {
                whole_centimetres(terrain.cliffs[chosen].height_metres.get())
            })
        },
        |base| whole_centimetres(base.get()),
    );
    let mut centimetres = vec![base; points_per_side * points_per_side];
    if let Some(slope) = terrain.slope.as_ref().and_then(SlopeFall::new) {
        grid.add_everywhere(&mut centimetres, |x, y| slope.centimetres(x, y));
    }
    for bump in bumps(specification, centre) {
        bump.add_to(&mut centimetres, grid);
    }
    let drops: Vec<CliffDrop> = terrain
        .cliffs
        .iter()
        .enumerate()
        .filter_map(|(index, cliff)| CliffDrop::new(cliff, shore_distance(index), centre))
        .collect();
    for drop in &drops {
        grid.add_everywhere(&mut centimetres, |x, y| drop.centimetres(x, y));
    }
    if let Some(rise) = terrain
        .plateau
        .as_ref()
        .and_then(|plateau| PlateauRise::new(plateau, centre))
    {
        grid.add_everywhere(&mut centimetres, |x, y| rise.centimetres(x, y));
    }
    let (shore_centimetres, shorelines) = sea.as_ref().map_or_else(
        || (Vec::new(), Vec::new()),
        |sea| shape_by_the_sea(sea, &drops, grid, &mut centimetres),
    );
    let mut cliff_lines: Vec<CliffLine> = drops
        .iter()
        .filter_map(|drop| drop.crossing(centre))
        .collect();
    cliff_lines.sort_by_key(|line| line.identifier);
    let lowest = centimetres
        .iter()
        .copied()
        .min()
        .expect("the grid has at least four points");
    let highest = centimetres
        .iter()
        .copied()
        .max()
        .expect("the grid has at least four points");
    let metres = |value: i32| {
        SignedLength::new(f64::from(value) / CENTIMETRES_PER_METRE)
            .expect("a height in whole centimetres is a valid length")
    };
    Terrain {
        grounds: vec![Ground {
            identifier: Terrain::surface(),
            points_per_side: specification.presets.height_grid_points_per_side,
            lowest_metres: metres(lowest),
            highest_metres: metres(highest),
            cliff_lines,
            shorelines,
            grid: HeightGrid {
                points_per_side,
                extent_metres: extent,
                centimetres,
                shore_centimetres,
            },
        }],
    }
}

/// The one cliff that stands on the shore by the beach rule, with its distance: the tallest
/// cliff without a distance that faces the coast, the first in the file on a tie. Any other
/// facing cliff keeps the usual default (ADR-076, VV1).
fn shore_cliff(sea: &coast::Sea, cliffs: &[Cliff]) -> Option<(usize, SignedLength)> {
    let mut chosen: Option<(usize, SignedLength)> = None;
    for (index, cliff) in cliffs.iter().enumerate() {
        let Some(distance) = sea.shore_cliff_distance(cliff) else {
            continue;
        };
        let taller = chosen
            .is_none_or(|(best, _)| cliff.height_metres.get() > cliffs[best].height_metres.get());
        if taller {
            chosen = Some((index, distance));
        }
    }
    chosen
}

/// Shapes the heights by `sea` and traces its shoreline; returns the sea's level at every grid
/// point and the shoreline pieces.
fn shape_by_the_sea(
    sea: &coast::Sea,
    drops: &[CliffDrop],
    grid: GridShape,
    centimetres: &mut [i32],
) -> (Vec<i32>, Vec<Shoreline>) {
    let signed = coast::shape_ground(sea, drops, grid, centimetres);
    let shorelines = coast::shorelines(&signed, grid);
    (signed, shorelines)
}

/// The size and placement of the height grid, for a feature that adds to it.
#[derive(Clone, Copy)]
struct GridShape {
    points_per_side: usize,
    spacing_metres: f64,
    centre: f64,
}

impl GridShape {
    /// The position of a grid point, in metres from the site's centre.
    fn position(self, row: usize, column: usize) -> (f64, f64) {
        (
            column as f64 * self.spacing_metres - self.centre,
            row as f64 * self.spacing_metres - self.centre,
        )
    }

    fn add_everywhere(self, centimetres: &mut [i32], height: impl Fn(f64, f64) -> i32) {
        for row in 0..self.points_per_side {
            for column in 0..self.points_per_side {
                let (x, y) = self.position(row, column);
                let cell = &mut centimetres[row * self.points_per_side + column];
                *cell = add(*cell, height(x, y));
            }
        }
    }
}

fn add(height: i32, contribution: i32) -> i32 {
    height
        .checked_add(contribution)
        .expect("the heights of a site fit in an i32 of centimetres")
}

fn whole_centimetres(metres: f64) -> i32 {
    let steps = SignedLength::new(metres)
        .expect("a feature's height is a valid length")
        .steps();
    i32::try_from(steps).expect("a feature's height fits in an i32 of centimetres")
}

/// A straight line across the site at right angles to `step`, `distance_metres` from the
/// centre along it.
#[derive(Clone, Copy)]
struct Line {
    step: Direction,
    distance_metres: f64,
}

impl Line {
    fn new(
        degrees: u16,
        (fraction, metres): (Option<Fraction>, Option<SignedLength>),
        half_side: f64,
    ) -> Line {
        let share = fraction.map_or(LINE_DISTANCE_FRACTION, Fraction::get);
        Line {
            step: toward(degrees),
            distance_metres: metres.map_or(share * half_side, SignedLength::get),
        }
    }

    fn beyond(self, x: f64, y: f64) -> f64 {
        x * self.step.x + y * self.step.y - self.distance_metres
    }
}

/// A cliff: every point on the low side of its line is `centimetres` lower, so the drop
/// spans one grid square (ADR-076, L1, FF4).
struct CliffDrop {
    identifier: Identifier,
    on_shore: bool,
    line: Line,
    low_side: f64,
    low_side_degrees: u16,
    centimetres: i32,
}

impl CliffDrop {
    fn new(
        cliff: &Cliff,
        shore_distance: Option<SignedLength>,
        half_side: f64,
    ) -> Option<CliffDrop> {
        let distance = match shore_distance {
            Some(metres) => (None, Some(metres)),
            None => (cliff.distance_fraction, cliff.distance_metres),
        };
        let degrees = cliff.bearing_degrees.degrees()?;
        let line = Line::new(degrees, distance, half_side);
        let toward_bearing = match cliff.drops_toward_degrees {
            None => true,
            Some(drops) => {
                let low = toward(drops.degrees()?);
                low.x * line.step.x + low.y * line.step.y > 0.0
            }
        };
        Some(CliffDrop {
            identifier: cliff_identifier(cliff, degrees, toward_bearing),
            on_shore: shore_distance.is_some(),
            line,
            low_side: if toward_bearing { 1.0 } else { -1.0 },
            low_side_degrees: if toward_bearing {
                degrees
            } else {
                (degrees + 180) % 360
            },
            centimetres: whole_centimetres(cliff.height_metres.get()),
        })
    }

    fn centimetres(&self, x: f64, y: f64) -> i32 {
        if self.line.beyond(x, y) * self.low_side > 0.0 {
            -self.centimetres
        } else {
            0
        }
    }

    fn crossing(&self, half_side: f64) -> Option<CliffLine> {
        let normal = self.line.step;
        let (base_x, base_y) = (
            normal.x * self.line.distance_metres,
            normal.y * self.line.distance_metres,
        );
        let (walk_x, walk_y) = (-self.low_side * normal.y, self.low_side * normal.x);
        let (first_x, last_x) = along_within(base_x, walk_x, half_side);
        let (first_y, last_y) = along_within(base_y, walk_y, half_side);
        let (first, last) = (first_x.max(first_y), last_x.min(last_y));
        let point = |along: f64| Point {
            x: SignedLength::new(base_x + along * walk_x + half_side)
                .expect("a point on the site is a valid length"),
            y: SignedLength::new(base_y + along * walk_y + half_side)
                .expect("a point on the site is a valid length"),
        };
        (first <= last).then(|| CliffLine {
            identifier: self.identifier,
            from: point(first),
            to: point(last),
            low_side_degrees: self.low_side_degrees,
        })
    }
}

/// The range of steps along `walk` from `base` that keep one coordinate within the site.
fn along_within(base: f64, walk: f64, half_side: f64) -> (f64, f64) {
    if walk == 0.0 {
        return (f64::NEG_INFINITY, f64::INFINITY);
    }
    let (one, other) = ((-half_side - base) / walk, (half_side - base) / walk);
    (one.min(other), one.max(other))
}

fn cliff_identifier(cliff: &Cliff, degrees: u16, toward_bearing: bool) -> Identifier {
    let ground = [Terrain::surface()];
    if let Some(name) = &cliff.identifier {
        return Identifier::new(IdentifierKind::Cliff, &ground, &[name]);
    }
    let degrees = degrees.to_string();
    let distance = match (cliff.distance_fraction, cliff.distance_metres) {
        (Some(share), _) => format!("fraction {}", share.get()),
        (None, Some(metres)) => format!("metres {}", metres.get()),
        (None, None) => "default".to_string(),
    };
    let side = if toward_bearing { "toward" } else { "away" };
    Identifier::new(IdentifierKind::Cliff, &ground, &[&degrees, &distance, side])
}

/// A plateau: the side of its line toward its bearing is higher, over a ramp of
/// `PLATEAU_RAMP_METRES` centred on the line.
#[derive(Clone, Copy)]
struct PlateauRise {
    line: Line,
    height_metres: f64,
}

impl PlateauRise {
    fn new(plateau: &Plateau, half_side: f64) -> Option<PlateauRise> {
        let distance = (plateau.distance_fraction, plateau.distance_metres);
        Some(PlateauRise {
            line: Line::new(plateau.bearing_degrees.degrees()?, distance, half_side),
            height_metres: plateau.height_metres.get(),
        })
    }

    fn centimetres(self, x: f64, y: f64) -> i32 {
        let share = (self.line.beyond(x, y) / PLATEAU_RAMP_METRES + 0.5).clamp(0.0, 1.0);
        whole_centimetres(self.height_metres * share)
    }
}

/// A hill (a positive height) or a hollow (a negative one): a circle footprint around a point,
/// and the profile (1 - u²)², the family (1 - u^p)^q of ADR-076 S5 with p = q = 2.
#[derive(Clone, Copy)]
struct Bump {
    x: f64,
    y: f64,
    radius_metres: f64,
    height_metres: f64,
}

fn bumps(specification: &Specification, half_side: f64) -> Vec<Bump> {
    let terrain = &specification.site.terrain;
    let hills = terrain.hills.iter().map(|hill| {
        let distance = (hill.distance_fraction, hill.distance_metres);
        Bump::new(
            hill.at,
            distance,
            hill.radius_metres,
            hill.height_metres.get(),
            half_side,
        )
    });
    let hollows = terrain.hollows.iter().map(|hollow| {
        let distance = (hollow.distance_fraction, hollow.distance_metres);
        Bump::new(
            hollow.at,
            distance,
            hollow.radius_metres,
            -hollow.depth_metres.get(),
            half_side,
        )
    });
    hills.chain(hollows).collect()
}

impl Bump {
    fn new(
        at: CompassRegion,
        (fraction, metres): (Option<Fraction>, Option<Length>),
        radius: Length,
        height_metres: f64,
        half_side: f64,
    ) -> Bump {
        let share = fraction.map_or(POINT_DISTANCE_FRACTION, Fraction::get);
        let distance = metres.map_or(share * half_side, Length::get);
        let (x, y) = at.bearing().map_or((0.0, 0.0), |degrees| {
            let step = toward(degrees);
            (distance * step.x, distance * step.y)
        });
        Bump {
            x,
            y,
            radius_metres: radius.get(),
            height_metres,
        }
    }

    fn centimetres(self, x: f64, y: f64) -> i32 {
        let (across, up) = (x - self.x, y - self.y);
        let u = (across * across + up * up).sqrt() / self.radius_metres;
        if u >= 1.0 {
            return 0;
        }
        let falloff = 1.0 - u * u;
        whole_centimetres(self.height_metres * falloff * falloff)
    }

    fn add_to(self, centimetres: &mut [i32], grid: GridShape) {
        let last = grid.points_per_side - 1;
        let index = |offset: f64| (offset + grid.centre) / grid.spacing_metres;
        let first = |offset: f64| index(offset).ceil().max(0.0) as usize;
        let final_point = |offset: f64| (index(offset).floor().max(0.0) as usize).min(last);
        let columns = first(self.x - self.radius_metres)..=final_point(self.x + self.radius_metres);
        let rows = first(self.y - self.radius_metres)..=final_point(self.y + self.radius_metres);
        for row in rows {
            for column in columns.clone() {
                let x = column as f64 * grid.spacing_metres - grid.centre;
                let y = row as f64 * grid.spacing_metres - grid.centre;
                let cell = &mut centimetres[row * grid.points_per_side + column];
                *cell = add(*cell, self.centimetres(x, y));
            }
        }
    }
}

#[derive(Clone, Copy)]
struct SlopeFall {
    step: Direction,
    metres_per_metre: f64,
}

impl SlopeFall {
    fn new(slope: &Slope) -> Option<SlopeFall> {
        let degrees = slope.toward_degrees.degrees()?;
        Some(SlopeFall {
            step: toward(degrees),
            metres_per_metre: slope.metres_per_kilometre.metres_per_kilometre()
                / METRES_PER_KILOMETRE,
        })
    }

    fn centimetres(self, x: f64, y: f64) -> i32 {
        let along = x * self.step.x + y * self.step.y;
        whole_centimetres(-self.metres_per_metre * along)
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::{Terrain, build};
    use crate::geometry::Point;
    use crate::identifier::{Identifier, IdentifierKind};
    use citymap_specification::specification::{
        Bearing, Cliff, Coast, CoastShape, CompassRegion, Defences, Era, Hill, Hollow, Identity,
        LandCover, MouthShape, Plateau, Presets, River, RiverMouth, Seed, Settlement, Site, Slope,
        Specification, Water,
    };
    use citymap_specification::units::{Gradient, Length, SignedLength};

    const EXTENT: f64 = 1000.0;

    fn point(x: f64, y: f64) -> Point {
        Point {
            x: SignedLength::new(x).expect("test input is a valid length"),
            y: SignedLength::new(y).expect("test input is a valid length"),
        }
    }

    fn slope(degrees: u16, metres_per_kilometre: f64) -> Slope {
        Slope {
            toward_degrees: Bearing::Degrees(degrees),
            metres_per_kilometre: Gradient::new(metres_per_kilometre)
                .expect("test input is a valid gradient"),
        }
    }

    fn specification(slope: Option<Slope>, points_per_side: u32) -> Specification {
        let mut site = Site {
            extent_metres: Length::new(EXTENT).expect("test input is a valid length"),
            terrain: citymap_specification::specification::Terrain::default(),
            land_cover: LandCover::default(),
            water: Water::default(),
        };
        site.terrain.slope = slope;
        Specification {
            schema: 1,
            identity: Identity {
                name: "Harrowmere".to_string(),
                seed: Seed::new(1),
                notes: String::new(),
            },
            settlement: Settlement { population: 6000 },
            site,
            districts: Vec::new(),
            landmarks: Vec::new(),
            defences: Defences::default(),
            open_space: Vec::new(),
            crossings: Vec::new(),
            roads: Vec::new(),
            presets: Presets {
                height_grid_points_per_side: points_per_side,
                ..Presets::new(Era::EarlyMedieval)
            },
        }
    }

    fn point_or_none(x: f64, y: f64) -> Option<Point> {
        ((0.0..=EXTENT).contains(&x) && (0.0..=EXTENT).contains(&y)).then(|| point(x, y))
    }

    fn hill(at: CompassRegion, radius: f64, height: f64) -> Hill {
        Hill {
            at,
            radius_metres: Length::new(radius).expect("test input is a valid length"),
            height_metres: Length::new(height).expect("test input is a valid length"),
            distance_fraction: None,
            distance_metres: None,
        }
    }

    fn with_hills(hills: Vec<Hill>, points_per_side: u32) -> Specification {
        let mut specification = specification(None, points_per_side);
        specification.site.terrain.hills = hills;
        specification
    }

    fn cliff(degrees: u16, distance_metres: f64, drops: Option<u16>) -> Cliff {
        Cliff {
            identifier: None,
            bearing_degrees: Bearing::Degrees(degrees),
            height_metres: Length::new(40.0).expect("test input is a valid length"),
            distance_fraction: None,
            distance_metres: Some(
                SignedLength::new(distance_metres).expect("test input is a valid length"),
            ),
            drops_toward_degrees: drops.map(Bearing::Degrees),
        }
    }

    fn with_cliffs(cliffs: Vec<Cliff>, points_per_side: u32) -> Specification {
        let mut specification = specification(None, points_per_side);
        specification.site.terrain.cliffs = cliffs;
        specification
    }

    fn height(terrain: &Terrain, x: f64, y: f64) -> f64 {
        terrain
            .height_at(Terrain::surface(), point(x, y))
            .expect("the point lies on the site")
            .get()
    }

    #[test]
    fn a_flat_site_is_zero_everywhere_on_one_surface_ground() {
        let terrain = build(&specification(None, 16));
        assert_eq!(terrain.grounds.len(), 1);
        assert_eq!(terrain.grounds[0].identifier, Terrain::surface());
        assert_eq!(height(&terrain, 0.0, 0.0), 0.0);
        assert_eq!(height(&terrain, 333.33, 777.77), 0.0);
    }

    #[test]
    fn a_slope_falls_toward_its_bearing_and_is_zero_at_the_centre() {
        let terrain = build(&specification(Some(slope(90, 10.0)), 101));
        assert_eq!(height(&terrain, 500.0, 500.0), 0.0);
        assert_eq!(
            height(&terrain, 1000.0, 500.0),
            -5.0,
            "500 m east at 10 m per km"
        );
        assert_eq!(height(&terrain, 0.0, 500.0), 5.0);
        assert_eq!(terrain.grounds[0].lowest_metres.get(), -5.0);
        assert_eq!(terrain.grounds[0].highest_metres.get(), 5.0);
    }

    #[test]
    fn a_height_between_grid_points_is_blended() {
        let terrain = build(&specification(Some(slope(90, 10.0)), 3));
        assert_eq!(
            height(&terrain, 750.0, 500.0),
            -2.5,
            "halfway between 0 and -5"
        );
        assert_eq!(
            height(&terrain, 750.0, 750.0),
            -2.5,
            "a slope blends to itself"
        );
        let north = build(&specification(Some(slope(0, 10.0)), 3));
        assert_eq!(
            height(&north, 500.0, 750.0),
            -2.5,
            "halfway between 0 and -5 northward"
        );
    }

    #[test]
    fn a_point_on_the_far_edge_has_a_height() {
        let terrain = build(&specification(Some(slope(45, 10.0)), 20));
        assert!(
            terrain
                .height_at(Terrain::surface(), point(EXTENT, EXTENT))
                .is_some()
        );
        assert!(
            terrain
                .height_at(Terrain::surface(), point(EXTENT, 0.0))
                .is_some()
        );
    }

    #[test]
    fn a_point_off_the_site_or_an_unknown_ground_has_no_height() {
        let terrain = build(&specification(None, 16));
        assert_eq!(
            terrain.height_at(Terrain::surface(), point(-0.01, 10.0)),
            None
        );
        assert_eq!(
            terrain.height_at(Terrain::surface(), point(10.0, 1000.01)),
            None
        );
        let unknown = Identifier::new(IdentifierKind::Ground, &[], &["hold"]);
        assert_eq!(terrain.height_at(unknown, point(10.0, 10.0)), None);
    }

    #[test]
    fn a_hill_has_its_height_at_the_top_and_falls_to_nothing_at_its_radius() {
        let terrain = build(&with_hills(
            vec![hill(CompassRegion::Centre, 100.0, 30.0)],
            101,
        ));
        assert_eq!(height(&terrain, 500.0, 500.0), 30.0);
        assert_eq!(height(&terrain, 550.0, 500.0), 16.88, "(1 - 0.5²)² × 30 m");
        assert_eq!(height(&terrain, 600.0, 500.0), 0.0);
        assert_eq!(terrain.grounds[0].highest_metres.get(), 30.0);
    }

    #[test]
    fn a_hill_without_a_distance_lies_halfway_to_the_edge() {
        let terrain = build(&with_hills(
            vec![hill(CompassRegion::North, 50.0, 10.0)],
            101,
        ));
        assert_eq!(
            height(&terrain, 500.0, 750.0),
            10.0,
            "250 m north, half the half-side"
        );
        assert_eq!(height(&terrain, 500.0, 500.0), 0.0);
    }

    #[test]
    fn a_hollow_with_a_distance_in_metres_dips_there() {
        let mut specification = specification(None, 101);
        specification.site.terrain.hollows = vec![Hollow {
            at: CompassRegion::East,
            radius_metres: Length::new(50.0).expect("a valid length"),
            depth_metres: Length::new(5.0).expect("a valid length"),
            distance_fraction: None,
            distance_metres: Some(Length::new(100.0).expect("a valid length")),
        }];
        let terrain = build(&specification);
        assert_eq!(height(&terrain, 600.0, 500.0), -5.0);
        assert_eq!(terrain.grounds[0].lowest_metres.get(), -5.0);
    }

    #[test]
    fn a_hill_in_the_corner_shows_only_its_inner_part() {
        let mut corner = hill(CompassRegion::NorthEast, 200.0, 20.0);
        corner.distance_metres = Some(Length::new(707.11).expect("a valid length"));
        let terrain = build(&with_hills(vec![corner], 101));
        assert_eq!(height(&terrain, 1000.0, 1000.0), 20.0);
        assert_eq!(height(&terrain, 800.0, 800.0), 0.0);
    }

    #[test]
    fn a_cliff_drops_within_one_grid_square_beyond_its_line() {
        let terrain = build(&with_cliffs(vec![cliff(90, 105.0, None)], 101));
        assert_eq!(height(&terrain, 600.0, 500.0), 0.0);
        assert_eq!(
            height(&terrain, 605.0, 500.0),
            -20.0,
            "halfway across the square"
        );
        assert_eq!(height(&terrain, 610.0, 500.0), -40.0);
        assert_eq!(height(&terrain, 1000.0, 0.0), -40.0);
        assert_eq!(terrain.grounds[0].lowest_metres.get(), -40.0);
    }

    #[test]
    fn a_cliff_on_a_grid_column_drops_along_one_column_from_edge_to_edge() {
        for (degrees, x, y) in [(90, 100.0, 0.0), (180, 0.0, -100.0), (270, -100.0, 0.0)] {
            let terrain = build(&with_cliffs(vec![cliff(degrees, 100.0, None)], 101));
            for along in [0.0, 100.0, 900.0, 1000.0] {
                let (on_x, on_y) = if x == 0.0 {
                    (along, 500.0 + y)
                } else {
                    (500.0 + x, along)
                };
                assert_eq!(height(&terrain, on_x, on_y), 0.0, "{degrees}: on the line");
            }
        }
    }

    #[test]
    fn a_cliff_can_drop_toward_the_centre() {
        let terrain = build(&with_cliffs(vec![cliff(270, 175.0, Some(90))], 101));
        assert_eq!(
            height(&terrain, 500.0, 500.0),
            -40.0,
            "the centre lies below the wall"
        );
        assert_eq!(
            height(&terrain, 300.0, 500.0),
            0.0,
            "the top lies west of the line"
        );
    }

    #[test]
    fn a_cliff_without_a_distance_lies_halfway_to_the_edge() {
        let mut north = cliff(0, 0.0, None);
        north.distance_metres = None;
        let terrain = build(&with_cliffs(vec![north], 101));
        assert_eq!(height(&terrain, 500.0, 740.0), 0.0);
        assert_eq!(height(&terrain, 500.0, 760.0), -40.0);
    }

    #[test]
    fn a_cliff_line_runs_edge_to_edge_with_the_low_side_on_the_right() {
        let terrain = build(&with_cliffs(vec![cliff(90, 105.0, None)], 16));
        let line = &terrain.grounds[0].cliff_lines[0];
        assert_eq!(
            (line.from, line.to),
            (point(605.0, 0.0), point(605.0, 1000.0))
        );
        assert_eq!(line.low_side_degrees, 90);
        let terrain = build(&with_cliffs(vec![cliff(90, 105.0, Some(270))], 16));
        let line = &terrain.grounds[0].cliff_lines[0];
        assert_eq!(
            (line.from, line.to),
            (point(605.0, 1000.0), point(605.0, 0.0))
        );
        assert_eq!(line.low_side_degrees, 270);
    }

    #[test]
    fn a_named_cliff_takes_its_identifier_from_its_name() {
        let mut named = cliff(90, 105.0, None);
        named.identifier = Some("west_scarp".to_string());
        let terrain = build(&with_cliffs(vec![named], 16));
        assert_eq!(
            terrain.grounds[0].cliff_lines[0].identifier,
            Identifier::new(
                IdentifierKind::Cliff,
                &[Terrain::surface()],
                &["west_scarp"]
            )
        );
    }

    fn with_coast(degrees: u16, shape: CoastShape, base: Option<f64>) -> Specification {
        let mut specification = specification(None, 101);
        specification.site.terrain.base_height_metres =
            base.map(|base| SignedLength::new(base).expect("test input is a valid length"));
        specification.site.terrain.coast = Some(Coast {
            bearing_degrees: Bearing::Degrees(degrees),
            shape,
            shore: None,
            distance_fraction: None,
            distance_metres: None,
            depth_fraction: None,
            depth_metres: None,
            width_fraction: None,
            width_metres: None,
        });
        specification
    }

    fn is_sea(terrain: &Terrain, x: f64, y: f64) -> bool {
        terrain
            .is_sea(Terrain::surface(), point(x, y))
            .expect("the point lies on the site")
    }

    #[test]
    fn an_open_coast_has_sea_beyond_its_shore_and_one_shoreline_with_the_land_on_its_left() {
        let terrain = build(&with_coast(90, CoastShape::Open, None));
        assert!(
            is_sea(&terrain, 900.0, 500.0),
            "the shore lies 0.6 of the half-side east"
        );
        assert!(!is_sea(&terrain, 700.0, 500.0));
        let shorelines = &terrain.grounds[0].shorelines;
        assert_eq!(shorelines.len(), 1);
        assert_eq!(
            shorelines[0].course.points(),
            [point(800.0, 0.0), point(800.0, 1000.0)],
            "northward, so the land to the west is on the left"
        );
    }

    #[test]
    fn the_land_runs_down_to_sea_level_within_one_hundred_metres_of_a_beach() {
        let terrain = build(&with_coast(90, CoastShape::Open, Some(5.0)));
        assert_eq!(height(&terrain, 600.0, 500.0), 5.0);
        assert_eq!(height(&terrain, 750.0, 500.0), 2.5, "50 m from the shore");
        assert_eq!(height(&terrain, 800.0, 500.0), 0.0, "on the shoreline");
    }

    #[test]
    fn the_sea_floor_falls_away_from_the_shore() {
        let terrain = build(&with_coast(90, CoastShape::Open, None));
        assert_eq!(
            height(&terrain, 810.0, 500.0),
            -0.5,
            "10 m out at 5 m per 100 m"
        );
        assert_eq!(height(&terrain, 1000.0, 500.0), -10.0, "200 m out");
    }

    #[test]
    fn a_sea_floor_above_the_water_is_lowered_to_one_metre_below_it() {
        let terrain = build(&with_coast(90, CoastShape::Open, Some(50.0)));
        assert_eq!(
            height(&terrain, 1000.0, 500.0),
            -1.0,
            "50 m less 10 m is still above"
        );
        assert!(terrain.grounds[0].lowest_metres.get() >= -1.0);
    }

    #[test]
    fn a_strait_has_other_land_beyond_its_width() {
        let terrain = build(&with_coast(90, CoastShape::Strait, None));
        assert!(is_sea(&terrain, 900.0, 500.0));
        assert!(
            !is_sea(&terrain, 980.0, 500.0),
            "150 m of sea, then other land"
        );
        assert_eq!(terrain.grounds[0].shorelines.len(), 2);
    }

    #[test]
    fn a_bay_cuts_half_an_ellipse_into_the_land() {
        let terrain = build(&with_coast(90, CoastShape::Bay, None));
        assert!(
            is_sea(&terrain, 600.0, 500.0),
            "200 of the 250 m depth inland"
        );
        assert!(
            !is_sea(&terrain, 500.0, 500.0),
            "300 m inland, past the depth"
        );
        assert!(
            !is_sea(&terrain, 700.0, 800.0),
            "300 m aside, past the half-width"
        );
    }

    #[test]
    fn a_cliff_facing_the_coast_stands_on_the_shore_and_sets_the_base_height() {
        let mut specification = with_coast(90, CoastShape::Open, None);
        let mut white = cliff(90, 0.0, None);
        white.distance_metres = None;
        white.height_metres = Length::new(30.0).expect("a valid length");
        specification.site.terrain.cliffs = vec![white];
        let terrain = build(&specification);
        assert_eq!(
            height(&terrain, 500.0, 500.0),
            30.0,
            "the base is the cliff's height"
        );
        assert_eq!(
            height(&terrain, 790.0, 500.0),
            30.0,
            "no beach along the cliff"
        );
        assert_eq!(
            height(&terrain, 810.0, 500.0),
            -0.5,
            "the cliff's foot, under water"
        );
        let line = &terrain.grounds[0].cliff_lines[0];
        assert_eq!(
            line.from,
            point(800.0, 0.0),
            "the cliff stands on the shore"
        );
    }

    #[test]
    fn only_the_tallest_facing_cliff_stands_on_the_shore() {
        let mut specification = with_coast(90, CoastShape::Open, None);
        let mut low = cliff(90, 0.0, None);
        low.distance_metres = None;
        low.height_metres = Length::new(10.0).expect("a valid length");
        let mut tall = cliff(80, 0.0, None);
        tall.distance_metres = None;
        specification.site.terrain.cliffs = vec![low, tall];
        let terrain = build(&specification);
        assert_eq!(
            height(&terrain, 500.0, 500.0),
            40.0,
            "the base is the tall cliff's height"
        );
        assert_eq!(height(&terrain, 740.0, 500.0), 40.0);
        assert_eq!(
            height(&terrain, 760.0, 500.0),
            30.0,
            "the low cliff keeps 0.5 of the half-side"
        );
    }

    #[test]
    fn a_base_height_in_the_file_wins_over_the_shore_cliff() {
        let mut specification = with_coast(90, CoastShape::Open, Some(-2.0));
        let mut white = cliff(80, 0.0, None);
        white.distance_metres = None;
        specification.site.terrain.cliffs = vec![white];
        let terrain = build(&specification);
        assert_eq!(
            height(&terrain, 500.0, 500.0),
            -2.0,
            "land below sea level stays land"
        );
        assert!(!is_sea(&terrain, 500.0, 500.0));
    }

    #[test]
    fn a_river_without_a_mouth_meets_the_sea_at_its_own_width() {
        let mut plain = with_estuary();
        plain.site.water.rivers[0].mouth = None;
        assert_eq!(
            build(&plain),
            build(&with_coast(90, CoastShape::Open, None))
        );
    }

    fn with_estuary() -> Specification {
        let mut specification = with_coast(90, CoastShape::Open, None);
        specification.site.water.rivers = vec![River {
            identifier: "fal".to_string(),
            from: CompassRegion::West,
            to: CompassRegion::East,
            width_metres: Length::new(20.0).expect("a valid length"),
            mouth: Some(RiverMouth {
                shape: MouthShape::Estuary,
                width_fraction: None,
                width_metres: None,
                reach_fraction: None,
                reach_metres: None,
            }),
        }];
        specification
    }

    #[test]
    fn an_estuary_narrows_exponentially_along_its_river_from_the_shore() {
        let terrain = build(&with_estuary());
        assert!(
            is_sea(&terrain, 790.0, 580.0),
            "10 m inland the half-width is 91 m"
        );
        assert!(
            is_sea(&terrain, 680.0, 530.0),
            "120 m inland: 100 × 0.1^0.48 = 33 m"
        );
        assert!(!is_sea(&terrain, 680.0, 540.0));
        assert!(
            !is_sea(&terrain, 500.0, 500.0),
            "past its 250 m reach the river is no sea"
        );
    }

    #[test]
    fn an_estuary_reaches_as_far_inland_as_its_reach() {
        let mut specification = with_estuary();
        if let Some(mouth) = &mut specification.site.water.rivers[0].mouth {
            mouth.reach_metres = Some(Length::new(400.0).expect("a valid length"));
        }
        let terrain = build(&specification);
        assert!(
            is_sea(&terrain, 450.0, 500.0),
            "350 m inland, within the 400 m reach"
        );
        assert!(!is_sea(&terrain, 350.0, 500.0), "450 m inland, past it");
    }

    fn tiny_grid(points_per_side: usize) -> super::GridShape {
        super::GridShape {
            points_per_side,
            spacing_metres: 10.0,
            centre: 0.0,
        }
    }

    #[test]
    fn a_pond_is_one_closed_shoreline_walked_with_the_land_on_its_left() {
        let level = [500, 500, 500, 500, -500, 500, 500, 500, 500];
        let pieces = super::coast::shorelines(&level, tiny_grid(3));
        assert_eq!(pieces.len(), 1);
        let points = pieces[0].course.points();
        assert_eq!(
            points.first(),
            points.last(),
            "a closed piece ends where it starts"
        );
        assert_eq!(points.len(), 5, "the four edge midpoints around the pond");
        let twice_area: f64 = points
            .windows(2)
            .map(|pair| pair[0].x.get() * pair[1].y.get() - pair[1].x.get() * pair[0].y.get())
            .sum();
        assert!(
            twice_area < 0.0,
            "clockwise around the water, so the land is on the left"
        );
    }

    #[test]
    fn a_square_with_sea_on_opposite_corners_is_decided_by_its_mean() {
        let land_centre = super::coast::shorelines(&[-500, 500, 500, -500], tiny_grid(2));
        assert_eq!(land_centre.len(), 2, "two sea corners cut off on their own");
        let sea_centre = super::coast::shorelines(&[-500, 400, 400, -500], tiny_grid(2));
        assert_eq!(sea_centre.len(), 2, "two land corners cut off on their own");
        assert_ne!(land_centre, sea_centre);
    }

    fn squared_distance_to_segment(x: f64, y: f64, from: Point, to: Point) -> f64 {
        let (along_x, along_y) = (to.x.get() - from.x.get(), to.y.get() - from.y.get());
        let (offset_x, offset_y) = (x - from.x.get(), y - from.y.get());
        let length_squared = along_x * along_x + along_y * along_y;
        let share = if length_squared == 0.0 {
            0.0
        } else {
            ((offset_x * along_x + offset_y * along_y) / length_squared).clamp(0.0, 1.0)
        };
        let (gap_x, gap_y) = (offset_x - share * along_x, offset_y - share * along_y);
        gap_x * gap_x + gap_y * gap_y
    }

    fn contains(polygon: &[Point], x: f64, y: f64) -> bool {
        let mut inside = false;
        for pair in polygon.windows(2) {
            let (one_x, one_y) = (pair[0].x.get(), pair[0].y.get());
            let (other_x, other_y) = (pair[1].x.get(), pair[1].y.get());
            if (one_y > y) != (other_y > y)
                && x < one_x + (y - one_y) * (other_x - one_x) / (other_y - one_y)
            {
                inside = !inside;
            }
        }
        inside
    }

    fn level(metres: f64) -> SignedLength {
        SignedLength::new(metres).expect("test input is a valid length")
    }

    #[test]
    fn contours_on_a_slope_are_straight_lines_with_the_higher_ground_on_their_left() {
        let terrain = build(&specification(Some(slope(90, 10.0)), 101));
        let contours = terrain.contours(Terrain::surface(), level(2.0));
        let levels: Vec<f64> = contours
            .iter()
            .map(|line| line.level_metres.get())
            .collect();
        assert_eq!(levels, [-4.0, -2.0, 0.0, 2.0, 4.0]);
        assert_eq!(
            contours[2].course.points(),
            [point(500.0, 0.0), point(500.0, 1000.0)],
            "northward, so the higher ground to the west is on the left"
        );
    }

    #[test]
    fn a_contour_stops_within_one_grid_square_of_a_cliff() {
        let mut specification = specification(Some(slope(90, 10.0)), 101);
        let mut north = cliff(0, 0.0, None);
        north.distance_metres = None;
        specification.site.terrain.cliffs = vec![north];
        let terrain = build(&specification);
        let contours = terrain.contours(Terrain::surface(), level(2.0));
        let every_point = || contours.iter().flat_map(|line| line.course.points());
        assert!(
            every_point().any(|point| point.y.get() < 740.0),
            "contours south of it"
        );
        assert!(
            every_point().any(|point| point.y.get() > 760.0),
            "contours north of it"
        );
        for line in &contours {
            for point in line.course.points() {
                assert!(
                    (point.y.get() - 750.0).abs() > 10.0,
                    "{point:?} near the cliff"
                );
            }
        }
    }

    #[test]
    fn an_interval_of_zero_or_an_unknown_ground_gives_no_contours() {
        let terrain = build(&specification(Some(slope(90, 10.0)), 16));
        assert_eq!(terrain.contours(Terrain::surface(), level(0.0)), []);
        let unknown = Identifier::new(IdentifierKind::Ground, &[], &["hold"]);
        assert_eq!(terrain.contours(unknown, level(5.0)), []);
    }

    #[test]
    fn the_sea_of_an_open_coast_is_closed_along_the_site_edge() {
        let terrain = build(&with_coast(90, CoastShape::Open, None));
        let areas = terrain.sea_areas(Terrain::surface());
        assert_eq!(areas.len(), 1);
        assert_eq!(
            areas[0].points(),
            [
                point(800.0, 0.0),
                point(800.0, 1000.0),
                point(1000.0, 1000.0),
                point(1000.0, 0.0),
                point(800.0, 0.0),
            ]
        );
    }

    #[test]
    fn the_sea_of_a_strait_lies_between_its_two_shores() {
        let terrain = build(&with_coast(90, CoastShape::Strait, None));
        let areas = terrain.sea_areas(Terrain::surface());
        assert_eq!(areas.len(), 1);
        assert_eq!(
            areas[0].points(),
            [
                point(800.0, 0.0),
                point(800.0, 1000.0),
                point(950.0, 1000.0),
                point(950.0, 0.0),
                point(800.0, 0.0),
            ]
        );
    }

    #[test]
    fn a_site_without_a_coast_has_no_sea_areas() {
        let terrain = build(&specification(None, 16));
        assert_eq!(terrain.sea_areas(Terrain::surface()), []);
    }

    #[test]
    fn a_site_without_a_coast_has_no_sea() {
        let terrain = build(&specification(None, 16));
        assert!(!is_sea(&terrain, 999.0, 1.0));
        assert_eq!(terrain.grounds[0].shorelines, []);
    }

    #[test]
    fn a_plateau_rises_over_a_ramp_centred_on_its_line() {
        let mut specification = specification(None, 101);
        specification.site.terrain.plateau = Some(Plateau {
            bearing_degrees: Bearing::Word(CompassRegion::North),
            height_metres: Length::new(20.0).expect("a valid length"),
            distance_fraction: None,
            distance_metres: Some(SignedLength::new(100.0).expect("a valid length")),
        });
        let terrain = build(&specification);
        assert_eq!(height(&terrain, 500.0, 570.0), 0.0);
        assert_eq!(height(&terrain, 500.0, 580.0), 2.0, "a tenth up the ramp");
        assert_eq!(
            height(&terrain, 500.0, 600.0),
            10.0,
            "the middle of the ramp"
        );
        assert_eq!(height(&terrain, 500.0, 630.0), 20.0);
        assert_eq!(terrain.grounds[0].highest_metres.get(), 20.0);
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(64))]

        #[test]
        fn a_cliff_line_ends_on_the_site_edge_with_the_low_side_on_its_right(
            degrees in 0..360_u16,
            share in -0.9..0.9_f64,
            drops_away in any::<bool>(),
            slant in 0..90_u16,
            along in 0.1..0.9_f64,
        ) {
            let side = if drops_away { 180 } else { 0 };
            let drops = Some((degrees + side + 315 + slant) % 360);
            let terrain = build(&with_cliffs(vec![cliff(degrees, share * EXTENT / 2.0, drops)], 101));
            let line = &terrain.grounds[0].cliff_lines[0];
            for end in [line.from, line.to] {
                let (x, y) = (end.x.get(), end.y.get());
                prop_assert!((0.0..=EXTENT).contains(&x) && (0.0..=EXTENT).contains(&y));
                let on_edge = [x, y].iter().any(|value| value.min(EXTENT - value) <= 0.01);
                prop_assert!(on_edge, "{x}, {y}");
            }
            let (from_x, from_y) = (line.from.x.get(), line.from.y.get());
            let (walk_x, walk_y) = (line.to.x.get() - from_x, line.to.y.get() - from_y);
            let length = (walk_x * walk_x + walk_y * walk_y).sqrt();
            prop_assume!(length > 100.0);
            let low = citymap_specification::direction::toward(line.low_side_degrees);
            prop_assert!((low.x - walk_y / length).abs() < 1e-3, "{low:?}");
            prop_assert!((low.y + walk_x / length).abs() < 1e-3, "{low:?}");
            let (right_x, right_y) = (walk_y / length * 30.0, -walk_x / length * 30.0);
            let (middle_x, middle_y) = (from_x + along * walk_x, from_y + along * walk_y);
            let side = |sign: f64| {
                terrain.height_at(
                    Terrain::surface(),
                    point_or_none(middle_x + sign * right_x, middle_y + sign * right_y)?,
                )
            };
            let (right, left) = (side(1.0), side(-1.0));
            prop_assume!(right.is_some() && left.is_some());
            prop_assert_eq!(
                right.map(SignedLength::get).zip(left.map(SignedLength::get)),
                Some((-40.0, 0.0))
            );
        }

        #[test]
        fn a_plateau_lies_between_zero_and_its_height_and_rises_toward_its_bearing(
            degrees in 0..360_u16,
            share in -0.9..0.9_f64,
            x in 0.0..=EXTENT,
            y in 0.0..=EXTENT,
            step in 1.0..200.0_f64,
        ) {
            let mut specification = specification(None, 51);
            specification.site.terrain.plateau = Some(Plateau {
                bearing_degrees: Bearing::Degrees(degrees),
                height_metres: Length::new(20.0).expect("a valid length"),
                distance_fraction: None,
                distance_metres: Some(
                    SignedLength::new(share * EXTENT / 2.0).expect("a valid length"),
                ),
            });
            let terrain = build(&specification);
            let here = height(&terrain, x, y);
            prop_assert!((0.0..=20.0).contains(&here));
            let ahead = citymap_specification::direction::toward(degrees);
            let further = point_or_none(x + step * ahead.x, y + step * ahead.y);
            prop_assume!(further.is_some());
            let there = terrain
                .height_at(Terrain::surface(), further.expect("checked above"))
                .expect("the point lies on the site")
                .get();
            prop_assert!(there >= here - 0.01, "{here} then {there}");
        }

        #[test]
        fn every_grid_point_in_the_sea_lies_below_sea_level(
            degrees in 0..360_u16,
            shape in 0..3_usize,
            base in -20.0..80.0_f64,
            share in -0.9..0.9_f64,
        ) {
            let shape = [CoastShape::Open, CoastShape::Strait, CoastShape::Bay][shape];
            let mut specification = with_coast(degrees, shape, Some(base));
            if let Some(coast) = &mut specification.site.terrain.coast {
                coast.distance_fraction = Some(
                    citymap_specification::units::Fraction::new(share).expect("a valid share"),
                );
            }
            let terrain = build(&specification);
            for row in 0..101_u32 {
                for column in 0..101_u32 {
                    let (x, y) = (f64::from(column) * 10.0, f64::from(row) * 10.0);
                    if is_sea(&terrain, x, y) {
                        prop_assert!(height(&terrain, x, y) < 0.0, "{x}, {y}");
                    }
                }
            }
        }

        #[test]
        fn a_straight_shoreline_has_the_land_on_its_left_and_the_sea_on_its_right(
            degrees in 0..360_u16,
            strait in any::<bool>(),
            share in -0.8..0.8_f64,
            along in 0.2..0.8_f64,
        ) {
            let shape = if strait { CoastShape::Strait } else { CoastShape::Open };
            let mut specification = with_coast(degrees, shape, None);
            if let Some(coast) = &mut specification.site.terrain.coast {
                coast.distance_fraction = Some(
                    citymap_specification::units::Fraction::new(share).expect("a valid share"),
                );
            }
            let terrain = build(&specification);
            prop_assume!(!terrain.grounds[0].shorelines.is_empty());
            for shoreline in &terrain.grounds[0].shorelines {
                let points = shoreline.course.points();
                let (first, last) = (points[0], points[points.len() - 1]);
                let (walk_x, walk_y) = (last.x.get() - first.x.get(), last.y.get() - first.y.get());
                let length = (walk_x * walk_x + walk_y * walk_y).sqrt();
                prop_assume!(length > 200.0);
                let (middle_x, middle_y) = (first.x.get() + along * walk_x, first.y.get() + along * walk_y);
                let (left_x, left_y) = (-walk_y / length * 30.0, walk_x / length * 30.0);
                let probe = |sign: f64| {
                    terrain.is_sea(
                        Terrain::surface(),
                        point_or_none(middle_x + sign * left_x, middle_y + sign * left_y)?,
                    )
                };
                if let (Some(left), Some(right)) = (probe(1.0), probe(-1.0)) {
                    prop_assert_eq!((left, right), (false, true));
                }
            }
        }

        #[test]
        fn every_sea_area_is_closed_runs_clockwise_and_lies_on_the_site(
            degrees in 0..360_u16,
            shape in 0..3_usize,
            share in -0.9..0.9_f64,
        ) {
            let shape = [CoastShape::Open, CoastShape::Strait, CoastShape::Bay][shape];
            let mut specification = with_coast(degrees, shape, None);
            if let Some(coast) = &mut specification.site.terrain.coast {
                coast.distance_fraction = Some(
                    citymap_specification::units::Fraction::new(share).expect("a valid share"),
                );
            }
            let terrain = build(&specification);
            let areas = terrain.sea_areas(Terrain::surface());
            let shores: Vec<[Point; 2]> = terrain.grounds[0]
                .shorelines
                .iter()
                .flat_map(|shoreline| {
                    shoreline.course.points().windows(2).map(|pair| [pair[0], pair[1]]).collect::<Vec<_>>()
                })
                .collect();
            for row in 0..20 {
                for column in 0..20 {
                    let (x, y) = (25.0 + f64::from(column) * 50.0, 25.0 + f64::from(row) * 50.0);
                    let near_shore = shores
                        .iter()
                        .any(|&[from, to]| squared_distance_to_segment(x, y, from, to) < 1600.0);
                    if near_shore {
                        continue;
                    }
                    let inside = areas.iter().any(|area| contains(area.points(), x, y));
                    prop_assert_eq!(inside, is_sea(&terrain, x, y), "{}, {}", x, y);
                }
            }
            for area in areas {
                let points = area.points();
                prop_assert_eq!(points.first(), points.last());
                let twice_area: f64 = points
                    .windows(2)
                    .map(|pair| pair[0].x.get() * pair[1].y.get() - pair[1].x.get() * pair[0].y.get())
                    .sum();
                prop_assert!(twice_area < 0.0, "clockwise, so the sea is inside");
                for point in points {
                    prop_assert!((0.0..=EXTENT).contains(&point.x.get()));
                    prop_assert!((0.0..=EXTENT).contains(&point.y.get()));
                }
            }
        }

        #[test]
        fn the_order_of_the_cliffs_does_not_change_the_ground(
            degrees in [0..360_u16, 0..360_u16, 0..360_u16],
            rotation in 1..3_usize,
        ) {
            let cliffs: Vec<Cliff> = degrees
                .iter()
                .zip([-120.0, 35.5, 210.0])
                .map(|(degrees, distance)| cliff(*degrees, distance, None))
                .collect();
            let mut rotated = cliffs.clone();
            rotated.rotate_left(rotation);
            prop_assert_eq!(build(&with_cliffs(cliffs, 33)), build(&with_cliffs(rotated, 33)));
        }

        #[test]
        fn a_hill_changes_only_the_points_within_its_radius(
            region in 0..9_usize,
            share in 0.0..=2.0_f64,
            radius in 10.0..=300.0_f64,
            x in 0.0..=EXTENT,
            y in 0.0..=EXTENT,
        ) {
            let at = CompassRegion::ALL[region];
            let mut placed = hill(at, radius, 25.0);
            if at != CompassRegion::Centre {
                placed.distance_fraction = Some(
                    citymap_specification::units::Fraction::new(share).expect("a valid share"),
                );
            }
            let with_hill = build(&with_hills(vec![placed.clone()], 51));
            let without = build(&specification(None, 51));
            let step = at
                .bearing()
                .map(citymap_specification::direction::toward);
            let distance = placed.distance_fraction.map_or(0.0, |share| share.get() * EXTENT / 2.0);
            let (top_x, top_y) = step.map_or((500.0, 500.0), |step| {
                (500.0 + distance * step.x, 500.0 + distance * step.y)
            });
            let away = ((x - top_x) * (x - top_x) + (y - top_y) * (y - top_y)).sqrt();
            prop_assume!(away > radius + 2.0 * EXTENT / 50.0);
            let at_point = point(x, y);
            prop_assert_eq!(
                with_hill.height_at(Terrain::surface(), at_point),
                without.height_at(Terrain::surface(), at_point)
            );
        }

        #[test]
        fn the_order_of_the_hills_does_not_change_the_ground(
            regions in [0..9_usize, 0..9_usize, 0..9_usize],
            radius in 10.0..=400.0_f64,
            rotation in 1..3_usize,
        ) {
            let hills: Vec<Hill> = regions
                .iter()
                .zip([17.33, 9.07, 4.41])
                .map(|(region, height)| hill(CompassRegion::ALL[*region], radius, height))
                .collect();
            let mut rotated = hills.clone();
            rotated.rotate_left(rotation);
            prop_assert_eq!(build(&with_hills(hills, 33)), build(&with_hills(rotated, 33)));
        }

        #[test]
        fn a_hill_adds_to_every_grid_point_within_its_radius_and_to_no_other(
            region in 0..9_usize,
            share in 0.0..=2.0_f64,
            radius in 5.0..=300.0_f64,
        ) {
            let at = CompassRegion::ALL[region];
            let mut placed = hill(at, radius, 25.0);
            if at != CompassRegion::Centre {
                placed.distance_fraction = Some(
                    citymap_specification::units::Fraction::new(share).expect("a valid share"),
                );
            }
            let points = 37;
            let terrain = build(&with_hills(vec![placed.clone()], points));
            let bump = super::bumps(&with_hills(vec![placed], points), EXTENT / 2.0)[0];
            let spacing = EXTENT / f64::from(points - 1);
            let mut expected = Vec::new();
            for row in 0..points {
                for column in 0..points {
                    let x = f64::from(column) * spacing - EXTENT / 2.0;
                    let y = f64::from(row) * spacing - EXTENT / 2.0;
                    expected.push(bump.centimetres(x, y));
                }
            }
            prop_assert_eq!(&terrain.grounds[0].grid.centimetres, &expected);
        }

        #[test]
        fn every_height_lies_within_the_reach_of_the_slope(
            degrees in 0..360_u16,
            gradient in 0.0..100.0_f64,
            x in 0.0..=EXTENT,
            y in 0.0..=EXTENT,
        ) {
            let terrain = build(&specification(Some(slope(degrees, gradient)), 33));
            let corner = (EXTENT / 2.0) * 2.0_f64.sqrt();
            let reach = gradient / 1000.0 * corner + 0.01;
            prop_assert!(height(&terrain, x, y).abs() <= reach);
        }
    }
}
