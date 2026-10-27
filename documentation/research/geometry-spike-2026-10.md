# Geometry crate spike (issue 12), 2026-10-19

> Throwaway tests, run once on Windows with Rust 1.99.1. The test crate lived in
> `spike/geometry/` in the first commit of the PR that added this file and was deleted in the
> second, as issue 12 requires. Versions are the latest on crates.io on this date. A claim here
> binds only through the decision log entry that cites it.

## What was tested

Four crates on the kind of input a city generator produces by accident: duplicate and
near-duplicate points, collinear points, NaN and out-of-range coordinates, zero-area and
self-intersecting polygons, offsets that collapse, millimetre features at kilometre
coordinates, and the same input run twice.

| Crate | Version | Role | License | `unsafe` | wasm32 build |
|---|---|---|---|---|---|
| spade | 2.15.1 | Delaunay triangulation and Voronoi diagram | MIT OR Apache-2.0 | none, `forbid(unsafe_code)` | yes |
| i_overlay | 9.1.0 | polygon booleans (union, difference) and offsets | MIT OR Apache-2.0 | 49 uses | yes |
| rstar | 0.13.0 | R-tree spatial index (nearest, within-distance) | MIT OR Apache-2.0 | none, `forbid(unsafe_code)` | yes |
| geo | 0.33.1 | general geometry algorithms (area, centroid, hull) | MIT OR Apache-2.0 | 2 uses | yes |

Terms: Delaunay triangulation connects points into triangles so that no point lies inside
another triangle's circumcircle; its dual, the Voronoi diagram, gives each point the region
closer to it than to any other. An R-tree is an index that answers "which objects are near
this point" without scanning everything. A polygon boolean is union, intersection or
difference of two shapes. An offset (also outline or buffer) grows or shrinks a polygon by a
distance.

## Findings, spade (21 tests, all green)

- Inserting the same point twice returns the same vertex handle and leaves one vertex. Duplicate
  ward sites are therefore merged silently, which is correct for us only if the two sites were meant
  to be one; a duplicate-identifier assertion upstream must catch the other case.
- Points 1e-9 apart are kept as two vertices and produce a sliver cell. The site jitter must
  guarantee a minimum spacing; spade will not enforce one.
- Six collinear points: no inner faces, six unbounded Voronoi faces, no panic.
- NaN, 1e300 and 1e-300 coordinates return `Err(InsertionError::{NAN, TooLarge, TooSmall})`
  rather than panicking. City coordinates in metres (0 to 5000) are far inside the allowed
  range (2^-142 to 2^201).
- Same input in the same order gives the same edge set and the same iteration order, twice.
- `bulk_load` and incremental insertion gave the same edge set for a jittered grid. For an
  exactly regular grid the four co-circular corners of each cell could legally triangulate
  either way, so sites must always be jittered.
- A 20 by 20 jittered grid plus three fixed landmark sites gave 403 Voronoi faces for 403
  sites, 388 of them bounded; the unbounded ones are on the outer rim, which the site
  boundary clips anyway.
- A vertex type can carry our own data (`HasPosition` trait), so a ward site can hold its
  cell identifier and the triangulation hands it back; no index bookkeeping needed.

## Findings, i_overlay

- Union of two overlapping squares: one shape, area 175, exact.
- A zero-area polygon is dropped silently: union with a square returns the square; the
  degenerate polygon alone returns zero shapes. We must therefore check inputs for zero area
  if silence is not what we want.
- A self-intersecting bowtie is split into two shapes of area 25 each with `NonZero` fill.
- A self-touching polygon returns one shape with one contour; no panic.
- Difference returns the hole as a second contour with negative signed area, so holes are
  distinguishable by orientation.
- Offset by -2 on a 10 by 10 square gives a 6 by 6 square; offset by -6 collapses to zero
  shapes rather than an inverted polygon; offset by +2 gives area 188 (rounded corners by
  default).
- Same input twice gives identical output including vertex order.
- Precision: two 1 mm squares at 1000 m gave a union area of 1.75007e-6 against the exact
  1.75e-6, a relative error of 4e-5. i_overlay converts floats to an integer grid scaled
  to the input's bounding box, so precision is relative to the extent of each operation.
  For a 5 km city with 0.5 m detail this is far below drawing resolution, but it means two
  overlay calls with different extents can round the same edge differently. Rule for us:
  quantise polygon coordinates to a fixed grid (0.01 m) after every overlay call, and never
  compare overlay output for equality without that quantisation.

## Findings, geo and rstar

- geo: a line-shaped polygon has area 0 and a centroid on the line; an empty polygon has
  area 0 and no centroid (`None`); convex hull of collinear points gives a zero-area hull,
  no panic.
- geo pulls in `i_overlay 4.5.2` and `rstar 0.12.2` as its own dependencies while we would
  use `i_overlay 9.1.0` and `rstar 0.13.0` directly, so the build would carry two versions
  of each. `cargo deny` flags duplicate versions. The three things we used geo for (area,
  centroid, convex hull) are each under 30 lines.
- rstar: four points with three duplicates are all stored; nearest-neighbour returns one of
  the duplicates; within-distance returns all three; iteration order is identical for the
  same input.

## Draft decision (becomes ADR-020 when Jeffrey approves)

Two options survive the spike and the crate verification in `crates-2026-10.md`. They differ
on one thing: whether we take `i_overlay` and `rstar` directly at their newest versions, or
through `geo`, which pins older versions behind a stable API.

**Option A, through `geo` (recommended).** `spade` for Delaunay and Voronoi; `geo` 0.33 for
polygon booleans (`BooleanOps`), offsets (`Buffer`), area, centroid and convex hull, with
its bundled `i_overlay` 4.5 and `rstar` 0.12. One version of each crate in the tree.
Reason to prefer it: the crate verification found `i_overlay` releases a new major version
every one to three months with no changelog; `geo` absorbs that churn and has a stable,
documented API. Cost: `geo` is larger than what we use, and the spike's `i_overlay` findings
were measured on 9.1, not on the 4.5 that `geo` bundles, so the four behaviours below must
be re-run through `geo`'s API at milestone 2 before they are relied on.

**Option B, direct.** `spade`, `i_overlay` 9.1 and `rstar` 0.13 directly; area, centroid and
hull written in core. Newest code and the exact behaviour this spike measured. Cost: every
`i_overlay` major bump is a review and a potential golden-snapshot change, for months.

Rules that come with either option:
1. Ward sites are always jittered; never an exact regular grid (co-circular points).
2. A minimum site spacing is enforced before triangulation (a core constant, set at
   milestone 3).
3. Polygon coordinates are quantised to 0.01 m after every boolean or offset call, because
   the overlay engine works on an integer grid scaled to each call's extent.
4. Zero-area polygons are rejected at the specification boundary and asserted against in stage
   outputs, because the overlay engine drops them silently.
5. `spade`'s `add_constraint` panics on crossing constraints; only the `try_` and
   `_and_split` variants may be called (crate verification finding).
Rejected under both options: `delaunator` (Delaunay only, no Voronoi handles or custom
vertex data); hand-written booleans (robustness is the hard part); `straight-skeleton`
(GPL-2.0); `geo-booleanop`, `earcutr`, `geo-buffer` and relatives (dead or archived).
Unverified: the four `i_overlay` behaviours through `geo` 0.33's API; `i_overlay` with the
`i64` engine on very large extents; `spade`'s constrained Delaunay for coastline edges, which
milestone 2 needs and must test first.
