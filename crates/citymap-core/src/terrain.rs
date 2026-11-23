//! The terrain of stage S0: one ground with a height field (ADR-076). Heights are whole
//! centimetres on a square grid of `presets.height_grid_points_per_side` points per side,
//! blended between the points. The grid stays private: other stages read heights only through
//! `Terrain::height_at`, so its storage can change without breaking them.

use std::fmt;

use serde::Serialize;

use crate::geometry::Point;
use crate::identifier::{Identifier, IdentifierKind};
use citymap_specification::direction::{Direction, toward};
use citymap_specification::specification::{CompassRegion, Slope, Specification};
use citymap_specification::units::{Fraction, Length, SignedLength};

/// The key of the one ground of version 1, the walkable surface (ADR-076, K1).
const SURFACE: &str = "surface";

/// Centimetres per metre: heights are stored in whole centimetres (ADR-076, N1).
const CENTIMETRES_PER_METRE: f64 = 100.0;

/// How far a hill or a hollow lies from the centre toward `at` when the file gives no
/// distance, as a share of the half-side (ADR-076, T1).
pub const POINT_DISTANCE_FRACTION: f64 = 0.5;

/// Metres per kilometre, to turn a slope's gradient into metres per metre.
const METRES_PER_KILOMETRE: f64 = 1000.0;

/// The grounds of the site; version 1 has exactly one, `SURFACE` (REQUIREMENTS.md F2b).
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Terrain {
    /// Every ground, in identifier order.
    pub grounds: Vec<Ground>,
}

/// One walkable ground: its identifier, the size of its height grid and its height range.
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
    #[serde(skip)]
    grid: HeightGrid,
}

#[derive(Clone, PartialEq)]
struct HeightGrid {
    points_per_side: usize,
    extent_metres: f64,
    centimetres: Vec<i32>,
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
        found.grid.height_at(point)
    }
}

impl HeightGrid {
    fn height_at(&self, point: Point) -> Option<SignedLength> {
        let last = self.points_per_side - 1;
        let (x, y) = (point.x.get(), point.y.get());
        let on_site = 0.0..=self.extent_metres;
        if !on_site.contains(&x) || !on_site.contains(&y) {
            return None;
        }
        let squares = last as f64 / self.extent_metres;
        let (column, across) = cell_and_offset(x * squares, last);
        let (row, up) = cell_and_offset(y * squares, last);
        let at = |column: usize, row: usize| {
            f64::from(self.centimetres[row * self.points_per_side + column])
        };
        let south = at(column, row) * (1.0 - across) + at(column + 1, row) * across;
        let north = at(column, row + 1) * (1.0 - across) + at(column + 1, row + 1) * across;
        let centimetres = south * (1.0 - up) + north * up;
        SignedLength::new(centimetres / CENTIMETRES_PER_METRE).ok()
    }
}

fn cell_and_offset(position: f64, last: usize) -> (usize, f64) {
    let position = position.min(last as f64);
    let cell = (position.floor() as usize).min(last - 1);
    (cell, position - cell as f64)
}

/// Builds the terrain of `specification`: one ground whose height at the site's centre is 0,
/// with the slope, the hills and the hollows added (ADR-076). Each feature rounds its own contribution to whole
/// centimetres, so the sum does not depend on the order of the features.
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
    let spacing_metres = extent / (points_per_side - 1) as f64;
    let centre = extent / 2.0;
    let slope = specification
        .site
        .terrain
        .slope
        .as_ref()
        .and_then(SlopeFall::new);
    let mut centimetres = Vec::with_capacity(points_per_side * points_per_side);
    for row in 0..points_per_side {
        for column in 0..points_per_side {
            let x = column as f64 * spacing_metres - centre;
            let y = row as f64 * spacing_metres - centre;
            centimetres.push(slope.map_or(0, |slope| slope.centimetres(x, y)));
        }
    }
    let grid = GridShape {
        points_per_side,
        spacing_metres,
        centre,
    };
    for bump in bumps(specification, extent / 2.0) {
        bump.add_to(&mut centimetres, grid);
    }
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
            grid: HeightGrid {
                points_per_side,
                extent_metres: extent,
                centimetres,
            },
        }],
    }
}

#[derive(Clone, Copy)]
struct GridShape {
    points_per_side: usize,
    spacing_metres: f64,
    centre: f64,
}

/// A hill (a positive height) or a hollow (a negative one): a circle footprint around a point,
/// and the profile (1 - u²)², the family (1 - u^p)^q of ADR-076 S5 with p = q = 2.
#[derive(Clone, Copy, Debug, PartialEq)]
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
        let steps = SignedLength::new(self.height_metres * falloff * falloff)
            .expect("a hill's height is a valid length")
            .steps();
        i32::try_from(steps).expect("a hill's height fits in an i32 of centimetres")
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
                *cell = cell
                    .checked_add(self.centimetres(x, y))
                    .expect("the heights of a site fit in an i32 of centimetres");
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
        let steps = SignedLength::new(-self.metres_per_metre * along)
            .expect("a slope's height is a valid length")
            .steps();
        i32::try_from(steps).expect("a slope's height fits in an i32 of centimetres")
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::{Terrain, build};
    use crate::geometry::Point;
    use crate::identifier::{Identifier, IdentifierKind};
    use citymap_specification::specification::{
        Bearing, CompassRegion, Defences, Era, Hill, Hollow, Identity, LandCover, Presets, Seed,
        Settlement, Site, Slope, Specification, Water,
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

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(64))]

        #[test]
        fn a_hill_changes_only_the_points_within_its_radius(
            region in 0..9_usize,
            share in 0.0..=1.0_f64,
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
            first in 0..9_usize,
            second in 0..9_usize,
            radius in 10.0..=400.0_f64,
        ) {
            let hills = vec![
                hill(CompassRegion::ALL[first], radius, 17.0),
                hill(CompassRegion::ALL[second], radius * 0.7, 9.0),
            ];
            let mut reversed = hills.clone();
            reversed.reverse();
            prop_assert_eq!(build(&with_hills(hills, 33)), build(&with_hills(reversed, 33)));
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
