//! The A/B timing of ADR-076, X1: contour lines traced on the model's own height grid against
//! contour lines traced on 1,024 × 1,024 samples of `height_at`, for Truro. Writes
//! `output/truro-model-grid.png` and `output/truro-samples.png` and prints both timings.
//!
//! Run with `cargo run --release -p citymap-command-line --example contour_timing`.

use std::fs;
use std::time::Instant;

use citymap_command_line::yaml;
use citymap_core::geometry::{Point, Polyline};
use citymap_core::model;
use citymap_core::terrain::{Contour, Terrain};
use citymap_render::{Style, render_png_with_contours};
use citymap_specification::specification::Specification;
use citymap_specification::units::SignedLength;

const TRURO: &str = "specifications/truro.yaml";

const SAMPLES_PER_SIDE: u32 = 1024;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let specification: Specification = yaml::from_text(&fs::read_to_string(TRURO)?)?;
    let city = model::generate(&specification);
    let terrain = &city.site.terrain;
    let style = Style::default();
    let interval = SignedLength::new(style.contour_interval.get())?;

    let started = Instant::now();
    let on_the_grid = terrain.contours(Terrain::surface(), interval);
    let grid_time = started.elapsed();

    let started = Instant::now();
    let from_samples = sampled_contours(terrain, city.site.extent_metres.get(), interval.get());
    let sample_time = started.elapsed();

    fs::create_dir_all("output")?;
    fs::write(
        "output/truro-model-grid.png",
        render_png_with_contours(&city, &style, &on_the_grid)?,
    )?;
    fs::write(
        "output/truro-samples.png",
        render_png_with_contours(&city, &style, &from_samples)?,
    )?;
    println!(
        "model grid ({} points per side): {} lines in {:.3} s",
        specification.presets.height_grid_points_per_side,
        on_the_grid.len(),
        grid_time.as_secs_f64()
    );
    println!(
        "{SAMPLES_PER_SIDE} samples per side: {} segments in {:.3} s",
        from_samples.len(),
        sample_time.as_secs_f64()
    );
    Ok(())
}

/// Contour segments from a square of samples of `height_at`, each square crossed by a level
/// giving one segment between the crossings on its edges, without joining them into lines.
fn sampled_contours(terrain: &Terrain, extent: f64, interval: f64) -> Vec<Contour> {
    let last = SAMPLES_PER_SIDE - 1;
    let spacing = extent / f64::from(last);
    let height = |column: u32, row: u32| {
        terrain
            .height_at(
                Terrain::surface(),
                point(f64::from(column) * spacing, f64::from(row) * spacing),
            )
            .map_or(0.0, SignedLength::get)
    };
    let heights: Vec<Vec<f64>> = (0..SAMPLES_PER_SIDE)
        .map(|row| {
            (0..SAMPLES_PER_SIDE)
                .map(|column| height(column, row))
                .collect()
        })
        .collect();
    let mut contours = Vec::new();
    for row in 0..last as usize {
        for column in 0..last as usize {
            let corners = [
                (column, row, heights[row][column]),
                (column + 1, row, heights[row][column + 1]),
                (column + 1, row + 1, heights[row + 1][column + 1]),
                (column, row + 1, heights[row + 1][column]),
            ];
            let low = corners
                .iter()
                .map(|corner| corner.2)
                .fold(f64::INFINITY, f64::min);
            let high = corners
                .iter()
                .map(|corner| corner.2)
                .fold(f64::NEG_INFINITY, f64::max);
            let mut level = (low / interval).ceil() * interval;
            while level <= high {
                let crossings: Vec<Point> = (0..4)
                    .filter_map(|edge| {
                        let (one, other) = (corners[edge], corners[(edge + 1) % 4]);
                        if (one.2 < level) == (other.2 < level) {
                            return None;
                        }
                        let share = (level - one.2) / (other.2 - one.2);
                        let x = (one.0 as f64 + share * (other.0 as f64 - one.0 as f64)) * spacing;
                        let y = (one.1 as f64 + share * (other.1 as f64 - one.1 as f64)) * spacing;
                        Some(point(x, y))
                    })
                    .collect();
                for pair in crossings.as_chunks::<2>().0 {
                    if let (Ok(course), Ok(level_metres)) =
                        (Polyline::new(pair.to_vec()), SignedLength::new(level))
                    {
                        contours.push(Contour {
                            level_metres,
                            course,
                        });
                    }
                }
                level += interval;
            }
        }
    }
    contours
}

fn point(x: f64, y: f64) -> Point {
    Point {
        x: SignedLength::new(x).expect("a point on the site is a valid length"),
        y: SignedLength::new(y).expect("a point on the site is a valid length"),
    }
}
