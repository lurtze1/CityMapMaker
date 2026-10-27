# City generation gaps, 2026-10-27

> A research report (ADR-023): it keeps the uncertainty of its sources. A claim here binds only
> through the decision log entry that cites it, and a figure such an entry cites becomes a fact
> under `data/research/` first (ADR-025). Every recommendation is marked **sourced** (a verified
> source states it) or **inference** (the report author's reading); Jeffrey decides each one in
> its own issue.

## Why this report exists

`city-generation-cross-check-2026-10.md` (#153) left topics unchecked: label placement, export
to Foundry VTT (the virtual tabletop the campaign runs on), lessons from other generator
authors, deterministic numerics and random number quality in Rust, street metrics, terrain
filling, Wave Function Collapse and sketchy rendering. This report covers those gaps with two
more verified runs.

## Method

Two deep-research workflows ran five search angles each. Each checked claim got three
independent verification votes; two refutes killed it. A topic without a verified claim below
was **not checked**, which is not the same as refuted.

| Run | Sources fetched | Claims extracted | Verified | Confirmed | Refuted | Dropped by budget |
|---|---|---|---|---|---|---|
| A: labels, Foundry, practitioners, numerics, metrics | 24 | 108 | 25 | 24 | 1 | 6 |
| B: what run A left open | 20 | 93 | 25 | 23 | 2 | 10 |

## 1. Label placement (S6)

### 1.1 Point labels are NP-hard, but small maps can be solved exactly

- Placing point labels without overlap is NP-hard: no known algorithm solves every case fast as
  the number of labels grows. At least three teams proved it independently, and it stays
  NP-complete with only four candidate positions per point (Christensen, Marks and Shieber
  1995 [1]; Formann and Wagner, cited in [2]).
- In the four-corner model an algorithm whose running time grows as n·log n reaches at least
  half the optimal label size, and unless P equals NP (the open question whether every quickly
  checkable problem is also quickly solvable) no fast algorithm can guarantee better [2]. Naive
  exact search tries 4ⁿ placements and handled about 10 points in seconds on 1990s hardware [2].
- Inference (merged votes 2-1, 3-0, 3-0): later exact methods with integer linear programming
  (an optimisation solver over whole-number choices; Zoraster 1990, Verweij and Aardal 1999,
  named by a verifier, not opened) solve hundreds to thousands of points. A city overview has
  tens of labels, so an exact solver is feasible; "must be heuristic" overstates the problem.

### 1.2 Which heuristic works best

Christensen, Marks and Shieber [1] compared six algorithms (random, greedy, discrete gradient
descent, Hirsch, Zoraster, simulated annealing) on 50 to 1,500 random points:

- Simulated annealing (random moves, accepting worse ones less often over time) "performs
  significantly better across the full range of problems considered".
- Discrete gradient descent came second, especially at high densities.
- With cartographic position preferences, annealing at zero temperature drops to about the
  level of gradient descent. "For time-critical applications, the annealing schedule can often
  be shortened or eliminated altogether."
- The study used random points with four positions; applying it to city labels is inference.

**Conflict with the design (inference).** Annealing seeded from a per-stage stream is
deterministic but global: one new label can shift every other label. That conflicts with the
locality rule (ARCHITECTURE.md §5) unless annealing is restricted to labels near the change,
with the others fixed.

### 1.3 Greedy by priority keeps earlier labels in place

Been, Daiches and Yap (2006, "Dynamic map labeling" [11]) give a greedy algorithm that places
labels from highest to lowest priority. It requires unique priorities and the rule that a label
may be blocked only by a higher-priority label; under that rule it is proved optimal, while the
unrestricted problem is NP-complete. Their requirement D4: placement and selection are a pure
function of the map state, independent of history. The proof treats each label's position as
fixed; a variant with four candidate positions is not proved optimal.

Inference: adding a lower-priority label can never move a higher-priority one. That matches the
pin rule (REQUIREMENTS.md F7, F8) if landmark N + 1 gets a lower priority than landmarks 1 to N.
It is not spatial locality: a new high-priority label can still move labels anywhere. For a
static A4 map the method reduces to greedy placement in priority order.

### 1.4 District names: a deterministic area-label method

- Imhof's rules for area labels, as summarised by Krumpe and Mendel [3]: horizontal or curved
  along the area's shape, wholly inside the area, with free space to the boundary. Imhof's own
  wording (1975, p. 136): "at least one-and-one-half the size of the letters on either end of
  the word". A curved label follows a circular arc of less than 60 degrees.
- RALF (Krumpe and Mendel [3]) places a curved area label without randomness. It approximates
  the polygon's centre line from the Voronoi graph of the boundary vertices (the network of
  points equally far from two or more vertices), prunes centre-line edges close to the
  boundary, picks long distinct paths with Dijkstra searches (shortest-path searches over a
  graph), fits the best matching circle to each path, and chooses the best position on that
  arc.
- Inference: each label depends only on its own ward polygon, so placement stays local. The
  paper starts from "an arbitrary node"; an implementation must fix that choice (for example
  the lowest stable identifier) for byte-identical output.

### 1.5 Street and river names along lines

Alinhac's rules (1964), as quoted by Barrault and Lecordix of the French mapping agency IGN
(1995 [12]): place the name along the feature, above the line, following its curve; avoid
strong curvature and prefer near-straight, near-horizontal parts that read left to right;
ignore small irregularities; avoid vertical parts, and where one cannot be avoided read upward
on the left half of the map and downward on the right half; never cross the feature or be
separated from it by another feature; repeat a long feature's name at roughly even intervals.
Their system splits each road into sections of a set maximum length, then balances even
spacing against position quality with simulated annealing followed by gradient descent. The
source gives no numeric curvature limit or letter spacing, labels only straight parts of short
road numbers on a 1:1,000,000 map, and its evaluation is weak: one cartographer rated 88% of
positions satisfactory.

Inference: applying the rules to curved street and river names, they map onto S6 constraint
kinds (maximum curvature under the label, no
crossing, above the line, a repeat interval). A deterministic search in a fixed order, or
annealing on a per-feature hashed stream, replaces global annealing.

### 1.6 What practitioners do (medium confidence: single-author code and devlogs)

- Martin O'Leary's terrain generator [4] places city labels greedily in one pass: each label
  tries four offsets (right, left, above, below) and the lowest penalty wins, ties to the first.
  Penalties: +100 per overlap with a placed label or a city marker, +100 per crossed map edge,
  +1 per river, coast or border vertex inside the box. Region labels use a separate score
  (−20,000 for overlapping an earlier region label, −3,000 outside the territory, −50,000 off
  the map). Both depend on placement order and are never revised.
- Watabou groups adjacent wards of the same type into neighbourhoods, which reduces the number
  of labels, and calls his own placement "far from perfect" [5].
- Inference: grouping wards before placement is a cheap way to need fewer labels. A greedy
  order must follow stable identifiers to stay deterministic.

## 2. Foundry VTT and Universal VTT export (milestone 10)

Universal VTT is a JSON file format for battle maps: the map image plus walls, doors and lights
in one file. The findings come from the source code of the Universal Battlemap Importer module
(moo-man/FVTT-DD-Import, current `master`, which can change) [6] and the Foundry version 13
programming interface documentation [7]. Foundry version 14 was not checked.

- **Fields read.** Walls from `line_of_sight` (plus `objects_line_of_sight` when the user
  enables it), doors from `portals` (openings in a wall: doors and windows), lights from
  `lights`, the map from an `image` field in base64 (binary data written as text), whose type
  is detected from its first bytes (PNG or webp).
- **Coordinates.** Universal VTT coordinates are in grid units. A wall point becomes
  `(p − map_origin) × pixels_per_grid + scene padding offset`. Scene width and height are
  `pixels_per_grid × map_size`, grid size equals `pixels_per_grid`. If `pixels_per_grid` is
  missing, the main path falls back to 100; the older paths have no fallback.
- **Doors and windows.** A portal with `closed: true` becomes a door (`door` 1, normal light and
  sight). A portal with `closed: false` imports as a window: no door unless the user's
  openableWindows setting is on, proximity sense, threshold 10 for light and sight.
- **Size cap.** No file-size check, but the canvas is capped at 11,000 pixels per side and
  11,000 × 11,000 pixels in area. The importer lowers the effective pixels per grid to fit,
  and fails with "Map is too large" below 10. It re-encodes the image to webp at quality 0.8 by
  default. The cap belongs to the importer, not to Foundry; Foundry's own scene-size limits
  were not checked.
- **Foundry walls.** In version 13 a Wall document is one straight segment,
  `c = [x0, y0, x1, y1]`. A door is a Wall with the numeric `door` field set, optionally with
  `ds` for its state [7]. The numeric values (none 0, door 1, secret 2; closed 0, open 1,
  locked 2) come from a verifier's memory and were not fetched.

Milestone 7b needs only a label-free PNG and scene JSON with correct dimensions and grid; the
walls, doors and Universal VTT findings above belong to milestone 10 (ARCHITECTURE.md §9).

## 3. Lessons from other generators

Two practitioner sources survived verification (medium confidence each): O'Leary's labels
(section 1.6) and Watabou's lots. Watabou's version 0.5.3 reworked alleys and lots and gave
"much fewer ugly triangular lots", with buildings inscribed more tightly into lots [5]. This
matches the merge step in the cross-check report, section 3.1.

Run B searched Azgaar, Stålberg (Townscaper), ProbableTrain, Dwarf Fortress world rejection and
Watabou's posts on walls, gates and outskirts again; no claim on them survived verification.

## 4. Deterministic numerics and random numbers in Rust

### 4.1 Float arithmetic: basic operations are portable

- Rust's float semantics (Rust RFC 3514 [8], the Rust language's own design document, not the
  IETF memo with the same number) require `+ − × ÷ %`, `sqrt`, `mul_add`, `as` casts and
  comparisons to match IEEE 754-2008 exactly, rounding ties to even, with no flush-to-zero
  (rounding tiny numbers to zero). NaN ("not a number") bit patterns are the exception.
  "Providing strict IEEE 754-2008 guarantees precludes many transformations, such as turning
  `a*b + c` into FMA operations" (FMA: fused multiply-add, one rounding instead of two). So
  `sqrt` is correctly rounded and the compiler never fuses a multiply and an add on its own.
  Transcendental functions (`sin`, `exp`, `atan2` and the like) stay unspecified.
- The x87 extended-precision problem (an old floating-point unit that keeps extra bits) is
  limited to 32-bit x86 without SSE2 (the newer vector instruction set): the i586 targets and
  i686 built for old CPUs [8][9]. Rust RFC 3514 lists 64-bit x86 and 64-bit ARM as "fully
  compliant".
- **This settles the implicit fused multiply-add part of the cross-check report's section
  3.4.** SIMD code paths stay open (its open question 2).
- Inference: explicit `mul_add` is deterministic by the same rule; whether its software
  fallback on CPUs without a fused multiply-add instruction is correctly rounded on Windows MSVC
  was not checked, so banning it remains a choice.
- Inference: `x86_64-pc-windows-msvc` and `x86_64-unknown-linux-gnu` both use SSE2, so basic
  arithmetic from the same source gives the same bits on both. ADR-020 already records that
  `libm` gives bit-identical results on both (140,020 inputs). The blocker for cross-platform
  output (#67) is the `geo` backend behind the geometry facade, which calls std transcendental
  functions; basic arithmetic is not.

### 4.2 The `libm` crate is not pure software on x86_64

In `libm` 0.2.16, the version in this repository's `Cargo.lock` [13]:

- `src/math/arch/mod.rs` sends `sqrt` and `sqrtf` to x86 inline assembly (processor
  instructions written directly in the source: `sqrtsd`, `sqrtss`)
  on x86 and x86_64 with SSE2. `fma` and `fmaf` go to `x86/fma.rs`, next to a runtime
  feature-detection module (seen, not analysed).
- These paths sit behind the `arch_enabled` configuration, which the default `arch` feature
  turns on. The `force-soft-floats` feature turns them off, so every function takes the
  software path. "Most" implementations are gated this way (vote 2-1 on that wording); the
  exceptions (i586 `ceil` and `floor`, x87 `exp`) do not apply to x86_64.
- `sqrtf` is tested exhaustively against MPFR (a reference library for correctly rounded
  arithmetic); `sqrt` extensively but not exhaustively. std's `f64::sqrt` uses the compiler
  intrinsic, not `libm`.
- Refuted 1-2: that in 2021 the SSE `sqrt` path was the only architecture-specific path [14].
- `libm` has since moved into `compiler-builtins`; later versions can change this dispatch.

In this repository `libm` arrives through `geographiclib-rs`, `i_float` and `num-traits`; the
last is reached from `geo`, `geo-types`, `rstar`, `serde-saphyr` and, as a dev-dependency,
`proptest`. The default `arch` feature is on, and `citymap-core` does not call `libm` directly
yet (`cargo tree -i libm`, 27 October 2026).

Inference: IEEE 754 requires `sqrt` and `fma` to be correctly rounded, so hardware and software
paths give the same bits for every input except NaN payload and sign bits. That is reasoning,
not a test. ADR-020 records identical `libm` results on both platforms but not which features
that proof (closed draft PR #66) enabled, so whether it covered the hardware `sqrt` path is not
known.

### 4.3 Random number generators

- **SplitMix gamma.** SplitMix's `split()` derives a per-stream constant ("gamma"). Many
  implementations copied an inverted repair check, so some seeds (for example
  `0x61c8864680b583eb`) give gamma 1. Even the correctly repaired gamma `0xaaaaaaaaaaaaaaab`
  fails PractRand (a statistical test suite) at 256 MB. There are many thousands of bad seeds,
  though most seeds are fine [10]. One primary source, written by the author of a competing
  generator, with reproducible test logs. Refuted 0-3: that such failures only show after
  large output volumes.
- **PCG streams.** A PCG generator can take an increment, a constant added at every step that
  selects one of many streams; its output permutation scrambles the internal state into the
  output. O'Neill, PCG's author, writes that changing the increment equals adding a fixed
  offset to the underlying generator's state before that permutation, so streams
  that differ only by increment are distinct but "far from" independent [15]. Her note adds
  that nearby streams are easier to correlate than she first thought; seed and stream must
  have no obvious relationship. Vigna's `corrpcg.c` [16] interleaves two streams and fails
  PractRand at 256 MB, but its constants are a constructed worst case (the second generator is
  the exact negation of the first), not evidence about randomly chosen seeds. Test method:
  write one 64-bit output from stream A, then one from stream B, repeat, and pipe the result
  into PractRand.
- Inference: neither weakness touches the project's design directly. ADR-018 and ADR-032 use
  only the SplitMix64 output mixer as a hash to seed `Pcg64Mcg`, never `split()`, and
  `Pcg64Mcg` has no increment. The remaining risk is related seed inputs: seeds 1, 2, 3, or
  neighbouring identifiers. The runs gave no quality rating for the mixer itself.

## 5. Street and block metrics

### 5.1 Definitions

- Boeing (2019 [17]) characterises street networks by entropy of street bearings (how evenly
  streets point in all compass directions), orientation order (how closely the network follows
  one grid), typical segment length, average circuity, average node degree (edges per
  junction), share of four-way intersections and share of dead ends.
- The paper on OSMnx (Boeing 2017 [18]), a Python library that downloads and measures street
  networks from OpenStreetMap, defines: **intersection density**, the density of nodes
  with more than one street (dead ends excluded); **average circuity**, total edge length
  divided by the total straight-line distance between each edge's end nodes; **average streets
  per node**, counting undirected physical streets. Average node degree counts directed in- and
  out-edges, so the two differ: Laurelhurst in Portland has 3.58 streets per node against a
  node degree of 5.53.
- Louf and Barthelemy (2014 [19]) define the block shape factor Φ = A / A_C: block area divided
  by the area of its circumscribed circle. Φ is below 1, and smaller means more elongated. They
  fingerprint a city by the distribution of Φ within each block-area bin, P(Φ | A), and cluster
  131 OpenStreetMap cities into 4 families. Block areas follow a power-law tail P(A) ~ A^(−τ)
  with τ about 2 (large blocks become rarer at a fixed rate as area grows), which the paper calls "seemingly universal": it does not tell cities apart,
  but any model of road-network growth must reproduce it.
- Refuted 1-2: that Manhattan peaks at Φ of about 0.3 and 0.5 and that a broad Φ distribution
  therefore separates organic fabric from grids.

### 5.2 Reference values (medium confidence)

Three 0.5 km² Portland sections [18] give an ordered scale of average streets per node: 3.93 in
Downtown (a grid, mostly 4-way), 3.58 in Laurelhurst (a mix of 3-way and 4-way), 2.38 in
Northwest Heights (mostly 3-way and dead ends; about a third of nodes are dead ends). These are
modern North American samples from the drivable directed network. **No verified source gave
values for medieval European town centres**; those need an OSMnx run on chosen towns.

## 6. Terrain and rendering

- **Priority-Flood** (Barnes, Lehman and Mulla 2014 [20]) floods the elevation grid inward
  from its edges with a priority queue (a queue that always hands out the lowest cell first),
  so every cell drains. Plain filling leaves flat areas level; those need the epsilon variant
  (raise each filled cell by a tiny amount) or a separate step. Cost for n cells: proportional
  to n for whole-number elevations, n·log n for floating-point. With epsilon 0, plain filling
  gives the same output under any queue order; with epsilon above 0, or for watershed labels
  (which outlet each cell drains to) and flow directions (which neighbour each cell drains
  into), order matters, and the paper breaks ties with an insertion counter. A total order is
  what makes the result independent of the heap implementation; the paper's own wording is
  softer.
- Inference: S0 already plans a height field (ARCHITECTURE.md §4, milestone 2); whether it must
  drain is open. Rivers are hand-authored, so they do not need it. If draining is required,
  the queue key is (elevation, then the cell's stable identifier).
- **Sketchy strokes (medium confidence).** The rough.js wiki [21] documents `roughness` (0 gives perfect shapes,
  values above 10 are "mostly useless") and `bowing` (how curved the lines are, 0 gives straight
  lines), both defaulting to 1. Nothing about its hachure or double-stroke algorithms was
  verified.

Not checked: Wave Function Collapse for city blocks, regularity and slivers of Voronoi cells
from jittered grid points, Squirrel3 and Squirrel5 hash quality, and whether Bridson's Poisson
disc sampling depends on order.

## 7. Recommendations and early-warning checks

| # | Recommendation | Early-warning check | Basis | Stage |
|---|---|---|---|---|
| 1 | Point labels are placed greedily in priority order, with landmark N + 1 below 1 to N; alternatives to weigh are exact integer linear programming and annealing limited to the changed neighbourhood | Test: adding a lower-priority label moves no higher-priority label | sourced [11][1][2], choice is inference | S6, milestone 7 |
| 2 | District labels use RALF-style centre-line and arc fitting; the start node is fixed by stable identifier | No label overlap; each district label inside its ward with Imhof's clearance and an arc under 60 degrees | sourced method [3], choice is inference | S6, milestone 7 |
| 3 | Street and river names follow Alinhac's rules as constraint kinds: maximum curvature, above the line, no crossing, repeat interval | Test per rule on a curved polyline | sourced rules [12], mapping is inference | S6, milestone 7 |
| 4 | Group adjacent wards of the same type before labelling, to need fewer labels | — | inference from [5] | S6 |
| 5 | Universal VTT exporter: write `map_origin`, `map_size` and `pixels_per_grid` always; keep each side at or below 11,000 pixels; gates as portals with `closed: true`; scene JSON has one Wall per polygon edge | Export tests on each of these four rules | sourced importer behaviour [6][7], rules are inference | milestone 10 |
| 6 | S4 detects triangular and sliver lots by a floor on the shape factor Φ or on the ratio of lot area to its oriented bounding box area (the smallest rectangle, at any angle, around the lot) | Property test on the floor after the merge step | sourced Φ [19], floor is inference from [5] | S4, milestone 6 |
| 7 | The sweep harness uses these definitions: streets per node (not node degree, since our graph is undirected), intersection density, circuity, dead-end share, orientation entropy, and P(Φ \| A) | A sweep reports each metric's distribution; the reference band comes from an OSMnx run on chosen medieval towns, never from the Portland values | sourced definitions [17][18][19], choice is inference | sweep, milestone 5 |
| 8 | Large generated towns reproduce the block-area power-law tail with τ near 2 | Sweep check on the fitted τ | sourced [19], check is inference | S4, sweep |
| 9 | With Rust RFC 3514, the cross-platform question narrows to the `geo` backend, transcendental functions, NaN bits, SIMD paths and `libm`'s architecture paths; options for `libm`: keep the default `arch` feature, enable `force-soft-floats`, or add a bit-comparison golden test | Golden-hash comparison between Windows and Linux in continuous integration; this reopens part of ADR-020's rejected "one snapshot set for both platforms", so Jeffrey decides | sourced [8][9][13], decision is Jeffrey's | determinism, #67 |
| 10 | Keep the SplitMix64 mixer as a hash only and `Pcg64Mcg` without increments | A PractRand tool (outside continuous integration) interleaves `random_stream_for` streams for adjacent identifiers and adjacent seeds; the data volume (for example 2³⁰ bytes) is our choice, no source gives one | sourced weaknesses [10][15][16], choice is inference | `random` module |
| 11 | If the S0 height field must drain, Priority-Flood's queue key is (elevation, stable cell identifier) | Test: same output under two heap implementations | sourced [20], key is inference | S0, milestone 2 |
| 12 | Sketchy strokes expose `roughness` and `bowing` as configurable fields, default 1 | — | sourced defaults [21], choice is inference | rendering |

## 8. Still open after the cross-check run and runs A and B

1. Values of streets per node, dead-end share, circuity, orientation entropy and P(Φ | A) for
   real medieval European town centres: this needs our own OSMnx run on chosen towns (the
   `town-plan-corpus.md` reports hold candidates), not more search.
2. Whether `random_stream_for` streams for adjacent identifiers and seeds pass PractRand when
   interleaved: this needs our own test, not more search.
3. Lessons from Azgaar, Stålberg, ProbableTrain, Dwarf Fortress and Watabou on walls and gates;
   the minimum font size for a printed A4 map; Wave Function Collapse for cities; jittered-grid
   Voronoi slivers; Squirrel3 and Squirrel5; Bridson order dependence; rough.js hachure
   algorithms. Run B searched all of these, and run A also searched the practitioner topics,
   Squirrel, Bridson and the font size, without a verified claim.

## Sources

Opened by the verification agents on 27 October 2026.

1. Christensen, J., Marks, J., & Shieber, S. (1995). An empirical study of algorithms for
   point-feature label placement. *ACM Transactions on Graphics, 14*(3).
   <https://dash.harvard.edu/bitstream/1/2032678/2/AnEmpiricalStudy.pdf>
2. Kučera, L., Mehlhorn, K., et al. *Map labeling* (lecture notes).
   <https://people.mpi-inf.mpg.de/%7Emehlhorn/ftp/MapLettering.pdf>
3. Krumpe, F., & Mendel, T. (2020). Computing curved area labels in near-real time.
   <https://arxiv.org/pdf/2001.02938>
4. O'Leary, M. *terrain* (source code of the fantasy map generator). <https://github.com/mewo2/terrain>
5. Watabou. *0.5.3: neighbourhoods, alleys and buildings* (devlog).
   <https://watabou.itch.io/medieval-fantasy-city-generator/devlog/34104/053-neighbourhoods-alleys-and-buildings>
6. moo-man. *FVTT-DD-Import* (Universal Battlemap Importer), `ddimport.js`.
   <https://raw.githubusercontent.com/moo-man/FVTT-DD-Import/master/ddimport.js>
7. Foundry VTT. *WallData*, version 13 programming interface documentation.
   <https://foundryvtt.com/api/v13/interfaces/foundry.documents.types.WallData.html>
8. Rust RFC 3514. *Float semantics*. <https://rust-lang.github.io/rfcs/3514-float-semantics.html>
9. rust-lang/rust issue 114479 (x87 floating point on i586 targets).
   <https://github.com/rust-lang/rust/issues/114479>
10. O'Neill, M. E. *Bugs in SplitMix(es)*. <https://pcg-random.org/posts/bugs-in-splitmix.html>
11. Been, K., Daiches, E., & Yap, C. (2006). Dynamic map labeling. *IEEE InfoVis 2006*.
    <https://cs.nyu.edu/visual/home/pub/infovis06.pdf>
12. Barrault, M., & Lecordix, F. (1995). An automated system for linear feature name placement
    which complies with cartographic quality criteria. *AutoCarto 12*.
    <https://cartogis.org/docs/proceedings/archive/auto-carto-12/pdf/an-automated-system-for-linear-feature-name-placement.pdf>
13. `libm` 0.2.16, `src/math/arch/mod.rs` and `configure.rs` (read on docs.rs and in the local
    Cargo registry). <https://docs.rs/crate/libm/latest/source/src/math/arch/mod.rs>
14. rust-lang/libm issue 255 (fetched through a mirror whose content matched GitHub).
    <https://github.com/rust-lang/libm/issues/255>
15. O'Neill, M. E. *Critiquing PCG's streams*.
    <https://pcg-random.org/posts/critiquing-pcg-streams.html>
16. Vigna, S. `corrpcg.c`. <https://pcg.di.unimi.it/corrpcg.c>
17. Boeing, G. (2019). Urban spatial order: street network orientation, configuration, and
    entropy. <https://geoffboeing.com/publications/urban-spatial-order-entropy/>
18. Boeing, G. (2017). OSMnx: new methods for acquiring, constructing, analyzing, and
    visualizing complex street networks. <https://arxiv.org/abs/1611.01890>
19. Louf, R., & Barthelemy, M. (2014). A typology of street patterns. *Journal of the Royal
    Society Interface*. <https://arxiv.org/abs/1410.2094>
20. Barnes, R., Lehman, C., & Mulla, D. (2014). Priority-flood: an optimal depression-filling
    and watershed-labeling algorithm for digital elevation models. *Computers & Geosciences,
    62*, 117–127. Preprint: <https://arxiv.org/pdf/1511.04463>
21. rough.js wiki. <https://github.com/rough-stuff/rough/wiki>

Run A fetched 14 more sources that yielded no verified claim, among them two rated unreliable
(an Esri blog post on minimum text sizes and a worldbuilding cartography article). Run B
fetched 9 more without a verified claim: the Dwarf Fortress wiki, Watabou's 0.3.0 devlog, the
`squirrel_noise5` source, a Peter Occil page on random number generators, the same Esri post
(rated "blog" this time), two Here Dragons Abound posts, arXiv 1104.5634, a Boris the Brave
post, and a Utrecht PDF rated unreliable.
