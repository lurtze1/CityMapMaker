//! The terrain of stage S0: one ground with a height field (ADR-076). Heights are whole
//! centimetres on a square grid of `presets.height_grid_points_per_side` points per side,
//! blended between the points. The grid stays private: other stages read heights only through
//! `Terrain::height_at`, so its storage can change without breaking them.

use std::fmt;

use serde::Serialize;

use crate::geometry::Point;
use crate::identifier::{Identifier, IdentifierKind};
use citymap_specification::direction::{Direction, toward};
use citymap_specification::specification::{Slope, Specification};
use citymap_specification::units::SignedLength;

/// The key of the one ground of version 1, the walkable surface (ADR-076, K1).
const SURFACE: &str = "surface";

/// Centimetres per metre: heights are stored in whole centimetres (ADR-076, N1).
const CENTIMETRES_PER_METRE: f64 = 100.0;

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
/// with the slope added (ADR-076). Each feature rounds its own contribution to whole
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
        Bearing, Defences, Era, Identity, LandCover, Presets, Seed, Settlement, Site, Slope,
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

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(64))]

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
