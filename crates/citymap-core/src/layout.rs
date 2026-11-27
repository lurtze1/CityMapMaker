//! Stage S1, the macro layout (ARCHITECTURE.md §4): a coarse grid of cells over the ground,
//! one ward site per cell, the Voronoi wards of those sites, and each ward cut along the
//! shore and the river banks into land, sea and river wards (ADR-081).

use serde::Serialize;

use crate::geometry::{self, Area, Point, Polygon};
use crate::identifier::{self, Identifier, IdentifierKind};
use crate::random::random_stream_for;
use crate::site::SiteOutput;
use crate::terrain::Terrain;
use citymap_specification::specification::{Placement, Specification};
use citymap_specification::units::SignedLength;

/// The share of a cell's side, in its middle, where the cell's own ward site may land, so the
/// sites of two full neighbouring cells stay at least 30 % of a cell apart (ADR-020, minimum
/// spacing); next to a short last cell the gap is smaller, and a pin may lie anywhere.
pub const SITE_SPREAD: f64 = 0.7;

/// A land ward smaller than this share of a full cell's area is a sliver left where water cut
/// a Voronoi region near its edge; it joins the land ward it shares the longest border with
/// (ADR-081, Jeffrey at stage 6).
pub const SMALL_WARD_SHARE: f64 = 0.25;

/// The output of stage S1 so far: the wards of the ground.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LayoutOutput {
    /// The ground the wards lie on; version 1 has one, the surface (ADR-076).
    pub ground: Identifier,
    /// How many cells lie along each side of the site; the last ones may be cut short.
    pub cells_per_side: u32,
    /// Every ward, sorted by identifier.
    pub wards: Vec<Ward>,
}

/// One ward: the land, sea or river part of one site's Voronoi region.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Ward {
    /// Derived from the site's owner (its cell, or the pinned object), the cover and the
    /// piece number, so it never depends on a list position.
    pub identifier: Identifier,
    /// The cell the site lies in.
    pub cell: Identifier,
    /// The pinned landmark, open space or crossing whose centre is the site, if any.
    pub pinned: Option<Identifier>,
    /// Whether the ward is land, sea or river.
    pub cover: Cover,
    /// The site the ward was grown from; it lies in the ward unless water cut it off.
    pub site: Point,
    /// The ward's shape.
    pub shape: Polygon,
}

/// What a ward lies on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Cover {
    /// Dry land.
    Land,
    /// The sea of the ground.
    Sea,
    /// A river, between its banks.
    River,
}

impl Cover {
    fn name(self) -> &'static str {
        match self {
            Cover::Land => "land",
            Cover::Sea => "sea",
            Cover::River => "river",
        }
    }
}

/// A point the Voronoi regions grow from, and who owns it.
struct Site {
    owner: Identifier,
    cell: Identifier,
    pinned: Option<Identifier>,
    position: Point,
}

/// Runs stage S1 on a specification and the output of S0.
///
/// # Panics
///
/// In a debug build, when two wards share an identifier.
pub fn run(specification: &Specification, site: &SiteOutput) -> LayoutOutput {
    let ground = Terrain::surface();
    let extent = site.extent_metres.get();
    let cell_size = specification.presets.cell_size_metres;
    let cells_per_side = cells_per_side(extent, cell_size.get());
    let sites = sites(
        specification,
        ground,
        extent,
        cell_size.get(),
        cells_per_side,
    );
    let positions: Vec<Point> = sites.iter().map(|site| site.position).collect();
    let regions = geometry::voronoi_regions(&positions, extent);
    let sea = Area::from_rings(&site.terrain.sea_areas(ground));
    let rivers = site.rivers.iter().fold(Area::empty(), |rivers, river| {
        rivers.union(&Area::widened(&river.course, river.width_metres.get()))
    });
    let cliffs: Vec<Area> = site
        .terrain
        .grounds
        .iter()
        .filter(|candidate| candidate.identifier == ground)
        .flat_map(|found| &found.cliff_lines)
        .map(|cliff| Area::right_of(cliff.from, cliff.to, extent))
        .collect();
    let cuts = Cuts {
        sea: &sea,
        rivers: &rivers,
        cliffs: &cliffs,
    };
    let mut wards: Vec<(Ward, Vec<bool>)> = sites
        .iter()
        .zip(regions)
        .filter_map(|(site, region)| region.map(|region| cut(site, &region, &cuts)))
        .flatten()
        .collect();
    wards.sort_by_key(|(ward, _)| ward.identifier);
    let wards = merge_small_land(wards, SMALL_WARD_SHARE * cell_size.get() * cell_size.get());
    let identifiers: Vec<Identifier> = wards.iter().map(|ward| ward.identifier).collect();
    identifier::debug_assert_unique(&identifiers);
    LayoutOutput {
        ground,
        cells_per_side,
        wards,
    }
}

/// Joins every small land ward, in identifier order, into the land ward on the same side of
/// every cliff that it shares the longest border with, preferring wards that are not small
/// themselves; ties go to the identifier. A small ward that holds its own pinned site stays, so
/// a pin always keeps its ward.
fn merge_small_land(wards: Vec<(Ward, Vec<bool>)>, smallest: f64) -> Vec<Ward> {
    let (wards, sides): (Vec<Ward>, Vec<Vec<bool>>) = wards.into_iter().unzip();
    let is_small = |ward: &Ward| {
        ward.cover == Cover::Land
            && ward.shape.area_square_metres() < smallest
            && !(ward.pinned.is_some() && ward.shape.covers(ward.site))
    };
    let small: Vec<bool> = wards.iter().map(is_small).collect();
    let mut kept: Vec<Option<Ward>> = wards.into_iter().map(Some).collect();
    for index in 0..kept.len() {
        if !small[index] {
            continue;
        }
        let Some(piece) = kept[index].take() else {
            continue;
        };
        let receiver = kept
            .iter()
            .enumerate()
            .filter_map(|(other, ward)| ward.as_ref().map(|ward| (other, ward)))
            .filter(|(other, ward)| {
                ward.cover == Cover::Land
                    && sides[*other] == sides[index]
                    && touching(&piece.shape, &ward.shape)
            })
            .map(|(other, ward)| {
                let shared = geometry::shared_border_metres(&piece.shape, &ward.shape);
                (
                    other,
                    small[other],
                    (shared * 100.0).round() as i64,
                    ward.identifier,
                )
            })
            .filter(|(_, _, shared, _)| *shared > 0)
            .min_by_key(|&(_, other_small, shared, identifier)| {
                (other_small, std::cmp::Reverse(shared), identifier)
            });
        let joined = receiver.and_then(|(other, ..)| {
            let ward = kept[other].as_ref()?;
            geometry::joined(&ward.shape, &piece.shape).map(|shape| (other, shape))
        });
        match joined {
            Some((other, shape)) => {
                if let Some(ward) = kept[other].as_mut() {
                    ward.shape = shape;
                }
            }
            None => kept[index] = Some(piece),
        }
    }
    kept.into_iter().flatten().collect()
}

fn touching(first: &Polygon, second: &Polygon) -> bool {
    let bounds = |polygon: &Polygon| {
        polygon.exterior().iter().fold(
            (i64::MAX, i64::MAX, i64::MIN, i64::MIN),
            |(west, south, east, north), point| {
                let (x, y) = (point.x.steps(), point.y.steps());
                (west.min(x), south.min(y), east.max(x), north.max(y))
            },
        )
    };
    let (first_west, first_south, first_east, first_north) = bounds(first);
    let (second_west, second_south, second_east, second_north) = bounds(second);
    first_west <= second_east + 2
        && second_west <= first_east + 2
        && first_south <= second_north + 2
        && second_south <= first_north + 2
}

fn cells_per_side(extent: f64, cell_size: f64) -> u32 {
    let (extent, cell_size) = (centimetres(extent), centimetres(cell_size));
    let count = (extent + cell_size - 1) / cell_size;
    u32::try_from(count).expect("validation keeps the cell count within u32")
}

fn centimetres(metres: f64) -> i64 {
    (metres * 100.0).round() as i64
}

/// The identifier of the cell in `column` and `row` of `ground`: the key of its random
/// stream, `("macro", ground, cell)` (ADR-081, P1).
pub fn cell_identifier(ground: Identifier, column: u32, row: u32) -> Identifier {
    Identifier::new(
        IdentifierKind::Cell,
        &[ground],
        &[&column.to_string(), &row.to_string()],
    )
}

/// The cells' own sites in row and column order, then the pins. A pin whose centre equals an
/// earlier pin's (landmarks, open spaces, crossings, each in file order) shares that pin's
/// ward; S2 reports the two pinned objects as overlapping (REQUIREMENTS.md F7).
fn sites(
    specification: &Specification,
    ground: Identifier,
    extent: f64,
    cell_size: f64,
    cells_per_side: u32,
) -> Vec<Site> {
    let cell_of = |position: Point| {
        let index = |steps: i64| {
            let cell = steps.div_euclid(centimetres(cell_size));
            u32::try_from(cell.clamp(0, i64::from(cells_per_side) - 1))
                .expect("the clamp keeps the index within the grid")
        };
        (index(position.x.steps()), index(position.y.steps()))
    };
    let mut pins: Vec<Site> = Vec::new();
    for (identifier, placement) in pinned_placements(specification) {
        let position = point(placement.x.get(), placement.y.get());
        if pins.iter().any(|pin| pin.position == position) {
            continue;
        }
        let (column, row) = cell_of(position);
        pins.push(Site {
            owner: identifier,
            cell: cell_identifier(ground, column, row),
            pinned: Some(identifier),
            position,
        });
    }
    let mut sites = Vec::new();
    for row in 0..cells_per_side {
        for column in 0..cells_per_side {
            let cell = cell_identifier(ground, column, row);
            if pins.iter().any(|pin| pin.cell == cell) {
                continue;
            }
            let mut stream = random_stream_for(specification.identity.seed, "macro", cell);
            let place = |index: u32, unit: f64| {
                let start = f64::from(index) * cell_size;
                let side = (start + cell_size).min(extent) - start;
                start + side * ((1.0 - SITE_SPREAD) / 2.0 + SITE_SPREAD * unit)
            };
            let x = place(column, stream.next_unit_f64());
            let y = place(row, stream.next_unit_f64());
            sites.push(Site {
                owner: cell,
                cell,
                pinned: None,
                position: point(x, y),
            });
        }
    }
    sites.extend(pins);
    sites
}

fn pinned_placements(specification: &Specification) -> Vec<(Identifier, Placement)> {
    let landmarks = specification
        .landmarks
        .iter()
        .filter(|landmark| landmark.pinned)
        .filter_map(|landmark| {
            let identifier =
                Identifier::new(IdentifierKind::Landmark, &[], &[&landmark.identifier]);
            landmark
                .resolved
                .clone()
                .map(|placement| (identifier, placement))
        });
    let open_spaces = specification
        .open_space
        .iter()
        .filter(|space| space.pinned)
        .filter_map(|space| {
            let identifier = Identifier::new(IdentifierKind::OpenSpace, &[], &[&space.identifier]);
            space
                .resolved
                .clone()
                .map(|placement| (identifier, placement))
        });
    let crossings = specification
        .crossings
        .iter()
        .filter(|crossing| crossing.pinned)
        .filter_map(|crossing| {
            let identifier =
                Identifier::new(IdentifierKind::Crossing, &[], &[&crossing.identifier]);
            crossing
                .resolved
                .clone()
                .map(|placement| (identifier, placement))
        });
    landmarks.chain(open_spaces).chain(crossings).collect()
}

/// What cuts a ward: the sea, the rivers, and the low side of each cliff.
struct Cuts<'a> {
    sea: &'a Area,
    rivers: &'a Area,
    cliffs: &'a [Area],
}

/// The land, sea and river wards of one site's region. Each land ward carries, per cliff, whether
/// it lies on the cliff's low side, so slivers never join across a cliff (ADR-081).
fn cut(site: &Site, region: &Polygon, cuts: &Cuts) -> Vec<(Ward, Vec<bool>)> {
    let (sea_parts, dry) = geometry::split(region, cuts.sea);
    let mut river_parts = Vec::new();
    let mut land_parts: Vec<(Polygon, Vec<bool>)> = Vec::new();
    for part in &dry {
        let (river, land) = geometry::split(part, cuts.rivers);
        river_parts.extend(river);
        land_parts.extend(land.into_iter().map(|piece| (piece, Vec::new())));
    }
    for low_side in cuts.cliffs {
        land_parts = land_parts
            .into_iter()
            .flat_map(|(piece, sides)| {
                let (low, high) = geometry::split(&piece, low_side);
                let with = |below: bool| {
                    let mut sides = sides.clone();
                    sides.push(below);
                    sides
                };
                let (low_sides, high_sides) = (with(true), with(false));
                low.into_iter()
                    .map(move |part| (part, low_sides.clone()))
                    .chain(high.into_iter().map(move |part| (part, high_sides.clone())))
                    .collect::<Vec<_>>()
            })
            .collect();
    }
    let without_sides =
        |parts: Vec<Polygon>| parts.into_iter().map(|part| (part, Vec::new())).collect();
    [
        (Cover::Land, land_parts),
        (Cover::Sea, without_sides(sea_parts)),
        (Cover::River, without_sides(river_parts)),
    ]
    .into_iter()
    .flat_map(|(cover, mut parts)| {
        parts.sort_by(|first, second| first.0.exterior().cmp(second.0.exterior()));
        parts
            .into_iter()
            .enumerate()
            .map(move |(index, (shape, sides))| {
                let ward = Ward {
                    identifier: Identifier::new(
                        IdentifierKind::Ward,
                        &[site.owner],
                        &[cover.name(), &(index + 1).to_string()],
                    ),
                    cell: site.cell,
                    pinned: site.pinned,
                    cover,
                    site: site.position,
                    shape,
                };
                (ward, sides)
            })
    })
    .collect()
}

fn point(x: f64, y: f64) -> Point {
    Point {
        x: SignedLength::new(x).expect("a site lies on the site"),
        y: SignedLength::new(y).expect("a site lies on the site"),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use proptest::prelude::*;

    use super::*;
    use crate::site;
    use citymap_specification::specification::{
        Bearing, Cliff, Coast, CoastShape, CompassRegion, Defences, Era, Identity, Importance,
        LandCover, Landmark, LandmarkFunction, Presets, River, RiverCrossingKind, Road, RoadEnd,
        Seed, Settlement, Site, Terrain as TerrainSpecification, Water,
    };
    use citymap_specification::units::{Length, Rotation};

    const EXTENT: f64 = 1000.0;

    fn length(value: f64) -> Length {
        Length::new(value).expect("test input is a valid length")
    }

    fn specification(seed: u64) -> Specification {
        Specification {
            schema: 1,
            identity: Identity {
                name: "Wardholm".to_string(),
                seed: Seed::new(seed),
                notes: String::new(),
            },
            settlement: Settlement { population: 3000 },
            site: Site {
                extent_metres: length(EXTENT),
                terrain: TerrainSpecification::default(),
                land_cover: LandCover::default(),
                water: Water::default(),
            },
            districts: Vec::new(),
            landmarks: Vec::new(),
            defences: Defences::default(),
            open_space: Vec::new(),
            crossings: Vec::new(),
            roads: vec![Road {
                identifier: "high_road".to_string(),
                ends: [
                    RoadEnd::Region(CompassRegion::West),
                    RoadEnd::Region(CompassRegion::East),
                ],
                importance: Importance::Major,
                river_crossing: RiverCrossingKind::Bridge,
            }],
            presets: Presets {
                height_grid_points_per_side: 64,
                ..Presets::new(Era::EarlyMedieval)
            },
        }
    }

    fn with_water(seed: u64) -> Specification {
        let mut specification = specification(seed);
        specification.site.water.rivers.push(River {
            identifier: "wend".to_string(),
            from: CompassRegion::North,
            to: CompassRegion::South,
            width_metres: length(30.0),
            mouth: None,
        });
        specification.site.terrain.coast = Some(Coast {
            bearing_degrees: Bearing::Word(CompassRegion::East),
            shape: CoastShape::Open,
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

    fn with_cliff(seed: u64) -> Specification {
        let mut specification = with_water(seed);
        specification.site.terrain.cliffs.push(Cliff {
            identifier: None,
            bearing_degrees: Bearing::Degrees(0),
            height_metres: length(20.0),
            distance_fraction: None,
            distance_metres: Some(SignedLength::new(100.0).expect("a valid distance")),
            drops_toward_degrees: None,
        });
        specification
    }

    fn pinned_landmark(identifier: &str, x: f64, y: f64) -> Landmark {
        Landmark {
            identifier: identifier.to_string(),
            name: identifier.to_string(),
            function: LandmarkFunction::Civic,
            label: None,
            importance: Importance::Major,
            footprint: None,
            constraints: Vec::new(),
            resolved: Some(Placement {
                x: length(x),
                y: length(y),
                rotation_degrees: Rotation::new(0.0).expect("0 is a valid turn"),
                width_metres: length(20.0),
                depth_metres: length(20.0),
            }),
            pinned: true,
            notes: String::new(),
        }
    }

    fn layout_of(specification: &Specification) -> LayoutOutput {
        run(specification, &site::run(specification))
    }

    fn covers_at(layout: &LayoutOutput, x: f64, y: f64) -> Vec<Cover> {
        layout
            .wards
            .iter()
            .filter(|ward| ward.shape.covers(point(x, y)))
            .map(|ward| ward.cover)
            .collect()
    }

    fn distance_to_ring(point: &Point, ring: &[Point]) -> f64 {
        let (x, y) = (point.x.get(), point.y.get());
        (0..ring.len())
            .map(|index| {
                let (start, end) = (ring[index], ring[(index + 1) % ring.len()]);
                let (start_x, start_y) = (start.x.get(), start.y.get());
                let (along_x, along_y) = (end.x.get() - start_x, end.y.get() - start_y);
                let length = along_x * along_x + along_y * along_y;
                let share = if length == 0.0 {
                    0.0
                } else {
                    (((x - start_x) * along_x + (y - start_y) * along_y) / length).clamp(0.0, 1.0)
                };
                let (near_x, near_y) = (start_x + share * along_x, start_y + share * along_y);
                ((x - near_x).powi(2) + (y - near_y).powi(2)).sqrt()
            })
            .fold(f64::INFINITY, f64::min)
    }

    fn within_two_centimetres(first: &Polygon, second: &Polygon) -> bool {
        let close = |from: &Polygon, to: &Polygon| {
            from.exterior()
                .iter()
                .all(|point| distance_to_ring(point, to.exterior()) <= 0.02)
        };
        close(first, second) && close(second, first)
    }

    fn by_identifier(layout: &LayoutOutput) -> BTreeMap<Identifier, &Ward> {
        layout
            .wards
            .iter()
            .map(|ward| (ward.identifier, ward))
            .collect()
    }

    #[test]
    fn the_grid_counts_a_short_last_cell() {
        assert_eq!(cells_per_side(2400.0, 100.0), 24);
        assert_eq!(cells_per_side(2450.0, 100.0), 25);
        assert_eq!(cells_per_side(500.0, 1000.0), 1);
        assert_eq!(
            cells_per_side(940.94, 10.01),
            94,
            "an exact multiple whose float quotient rounds up"
        );
    }

    #[test]
    fn a_dry_site_has_one_land_ward_per_cell() {
        let layout = layout_of(&specification(1));
        assert_eq!(layout.cells_per_side, 10);
        assert_eq!(layout.wards.len(), 100);
        assert!(layout.wards.iter().all(|ward| ward.cover == Cover::Land));
        let cells: BTreeSet<Identifier> = layout.wards.iter().map(|ward| ward.cell).collect();
        assert_eq!(cells.len(), 100, "every cell has its own ward");
    }

    #[test]
    fn a_site_of_one_cell_is_one_ward() {
        let mut specification = specification(1);
        specification.presets.cell_size_metres = length(1000.0);
        let layout = layout_of(&specification);
        assert_eq!(layout.wards.len(), 1);
        assert!((layout.wards[0].shape.area_square_metres() - EXTENT * EXTENT).abs() < 1.0);
    }

    #[test]
    fn a_pin_replaces_the_site_of_its_cell() {
        let mut specification = specification(1);
        specification
            .landmarks
            .push(pinned_landmark("keep", 450.0, 650.0));
        let layout = layout_of(&specification);
        let keep = Identifier::new(IdentifierKind::Landmark, &[], &["keep"]);
        let cell = cell_identifier(Terrain::surface(), 4, 6);
        let in_cell: Vec<&Ward> = layout
            .wards
            .iter()
            .filter(|ward| ward.cell == cell)
            .collect();
        assert_eq!(in_cell.len(), 1, "the pin's ward is the cell's only ward");
        assert_eq!(in_cell[0].pinned, Some(keep));
        assert_eq!(in_cell[0].site, point(450.0, 650.0));
        assert!(in_cell[0].shape.covers(point(450.0, 650.0)));
    }

    #[test]
    fn two_pins_in_one_cell_give_two_wards() {
        let mut specification = specification(1);
        specification
            .landmarks
            .push(pinned_landmark("keep", 420.0, 620.0));
        specification
            .landmarks
            .push(pinned_landmark("well", 480.0, 680.0));
        let layout = layout_of(&specification);
        let cell = cell_identifier(Terrain::surface(), 4, 6);
        let count = layout.wards.iter().filter(|ward| ward.cell == cell).count();
        assert_eq!(count, 2);
    }

    #[test]
    fn wards_are_cut_into_land_sea_and_river_at_the_shore_and_the_banks() {
        let layout = layout_of(&with_water(1));
        assert_eq!(covers_at(&layout, 500.0, 300.0), vec![Cover::River]);
        assert_eq!(covers_at(&layout, 200.0, 300.0), vec![Cover::Land]);
        assert_eq!(
            covers_at(&layout, 990.0, 500.0),
            vec![Cover::Sea],
            "the east edge is sea"
        );
    }

    #[test]
    fn every_point_of_the_site_lies_in_exactly_one_ward() {
        let layout = layout_of(&with_water(1));
        for step_x in 0..77 {
            for step_y in 0..77 {
                let (x, y) = (
                    3.7 + 12.9 * f64::from(step_x),
                    2.3 + 12.9 * f64::from(step_y),
                );
                let found = covers_at(&layout, x, y);
                assert_eq!(found.len(), 1, "({x}, {y}) lies in {found:?}");
            }
        }
    }

    #[test]
    fn no_land_ward_lies_across_a_cliff() {
        let layout = layout_of(&with_cliff(1));
        let land = layout.wards.iter().filter(|ward| ward.cover == Cover::Land);
        let (mut above, mut below) = (0, 0);
        for ward in land {
            let heights: Vec<f64> = ward
                .shape
                .exterior()
                .iter()
                .map(|point| point.y.get())
                .collect();
            let over = heights.iter().any(|y| *y > 600.02);
            let under = heights.iter().any(|y| *y < 599.98);
            assert!(
                !(over && under),
                "{:?} lies across the cliff at y = 600",
                ward.identifier
            );
            above += usize::from(over);
            below += usize::from(under);
        }
        assert!(
            above > 10 && below > 10,
            "{above} wards above, {below} below"
        );
    }

    #[test]
    fn a_short_last_cell_keeps_its_site_inside_it() {
        let mut specification = specification(1);
        specification.site.extent_metres = length(1050.0);
        let layout = layout_of(&specification);
        assert_eq!(layout.cells_per_side, 11);
        let last = cell_identifier(Terrain::surface(), 10, 10);
        let ward = layout
            .wards
            .iter()
            .find(|ward| ward.cell == last)
            .expect("the short last cell has a ward");
        for metres in [ward.site.x.get(), ward.site.y.get()] {
            assert!((1007.5..=1042.5).contains(&metres), "{metres}");
        }
    }

    #[test]
    fn two_pins_in_a_site_of_one_cell_split_it_between_them() {
        let mut specification = specification(1);
        specification.site.extent_metres = length(500.0);
        specification.presets.cell_size_metres = length(1000.0);
        specification
            .landmarks
            .push(pinned_landmark("keep", 100.0, 250.0));
        specification
            .landmarks
            .push(pinned_landmark("well", 400.0, 250.0));
        let layout = layout_of(&specification);
        let areas: Vec<f64> = layout
            .wards
            .iter()
            .map(|ward| ward.shape.area_square_metres())
            .collect();
        assert_eq!(areas, vec![125_000.0, 125_000.0]);
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(32))]

        #[test]
        fn every_cell_site_lies_in_the_middle_of_its_cell(seed in any::<u64>()) {
            let layout = layout_of(&specification(seed));
            for ward in &layout.wards {
                let inside = |metres: f64| {
                    let offset = metres - (metres / 100.0).floor() * 100.0;
                    (14.99..=85.01).contains(&offset)
                };
                prop_assert!(inside(ward.site.x.get()) && inside(ward.site.y.get()));
            }
        }

        #[test]
        fn the_same_specification_gives_the_same_wards(seed in any::<u64>()) {
            prop_assert_eq!(layout_of(&with_water(seed)), layout_of(&with_water(seed)));
        }

        #[test]
        fn a_new_pin_moves_wards_outside_two_rings_of_cells_by_at_most_two_centimetres(
            seed in any::<u64>(),
            x in prop_oneof![0.0..=1000.0_f64, (0..=10_u32).prop_map(|cell| f64::from(cell) * 100.0)],
            y in prop_oneof![0.0..=1000.0_f64, (0..=10_u32).prop_map(|cell| f64::from(cell) * 100.0)],
        ) {
            let before = with_cliff(seed);
            let mut after = before.clone();
            after.landmarks.push(pinned_landmark("chapel", x, y));
            let index = |metres: f64| ((metres / 100.0).floor() as i64).min(9);
            let (column, row) = (index(x), index(y));
            let ring: BTreeSet<Identifier> = (column - 2..=column + 2)
                .flat_map(|near_column| (row - 2..=row + 2).map(move |near_row| (near_column, near_row)))
                .filter_map(|(near_column, near_row)| {
                    let near_column = u32::try_from(near_column).ok()?;
                    let near_row = u32::try_from(near_row).ok()?;
                    Some(cell_identifier(Terrain::surface(), near_column, near_row))
                })
                .collect();
            let (old, new) = (layout_of(&before), layout_of(&after));
            let far_count = |layout: &LayoutOutput| {
                layout.wards.iter().filter(|ward| !ring.contains(&ward.cell)).count()
            };
            prop_assert_eq!(far_count(&old), far_count(&new), "far wards appeared or vanished");
            let new = by_identifier(&new);
            for ward in old.wards.iter().filter(|ward| !ring.contains(&ward.cell)) {
                let Some(now) = new.get(&ward.identifier) else {
                    return Err(TestCaseError::fail(format!("{:?} vanished", ward.identifier)));
                };
                prop_assert_eq!((now.cover, now.site, now.pinned), (ward.cover, ward.site, ward.pinned));
                prop_assert!(within_two_centimetres(&now.shape, &ward.shape), "{:?}", ward.identifier);
            }
        }
    }
}
