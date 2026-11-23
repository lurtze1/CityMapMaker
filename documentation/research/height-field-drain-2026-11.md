# Research for #167: must the S0 height field drain (Priority-Flood)?

Date: 2026-11-09. Status: research report under ADR-023. It reports sources with their
uncertainty and decides nothing. The options table and the questions for Jeffrey are at the end.

Read in the repository: CLAUDE.md; ARCHITECTURE.md §1, §4 (S0 to S7 rows), §5, §9;
DECISIONS.md ADR-020, ADR-056, ADR-057, ADR-058, ADR-059; `city-generation-gaps-2026-10.md`
§6 and recommendation 11; REQUIREMENTS.md lines on `on_high_ground` and `on_low_ground`;
`specification-version-1.md` §4 rule table; `specifications/truro.yaml` (site section).

Terms: a **depression** (pit, sink) is a cell group with no downhill path to an outlet;
**filling** raises it to its lowest rim point (spill point); an **outlet** or **seed** is a
cell where water may leave, and Priority-Flood starts from them; a **flat** is a level area
where water has no direction; the **epsilon variant** makes each filled cell a tiny amount
(epsilon) higher than the next cell toward the outlet, so flats get a direction.

## 1. What Priority-Flood is (sources)

- [1] (Barnes, Lehman and Mulla 2014; full text read) defines the filled surface W by three
  criteria taken from Planchon and Darboux: every cell of W is at least the original height;
  every cell has a path to the boundary that descends by at least epsilon per step, "where ǫ
  may be zero"; W is the lowest such surface. The algorithm pushes all edge cells onto a
  priority queue, pops the lowest, raises each unprocessed neighbour to at least the popped
  cell's height, and pushes it. Each cell is pushed once. The paper says it works on 4-, 8-
  or n-connected grids and on irregular meshes such as a Delaunay triangulation.
- Cost [1]: O(n) for whole-number elevations (with a bucket or hierarchical queue), O(n log n)
  for floating-point elevations with a heap. The "improved" variant sends cells found inside a
  depression to a plain first-in-first-out queue ("pit queue"); the paper reports 12.5%
  median and 37.2% maximum speed-up on 44 Minnesota county grids (about 2·10⁸ cells each). The
  paper gives no absolute run times. It also says floating-point grids "may be quantized and
  treated as integer DEMs" when there are known resolution increments.
- Ordering [1] §4, quoted: "for depression filling with ǫ = 0, it does not matter whether the
  priority queue has a total order or a strict weak order, as both will produce the same
  results. However, if ǫ ≠ 0, or if Priority-Flood is used for watershed labeling or to
  determine flow directions directly, then the ordering is important and a total order is the
  best choice." The paper makes the order total with a second key, "an integer which begins at
  zero and is incremented every time a cell is added to the priority queue", so equal cells
  leave in insertion order. It says this total order "guarantees that each cell has a
  least-cost (i.e. shortest) path to its flooding source", so depressions "drain from multiple
  outlets, provided the outlets are of equal elevation".
- Epsilon [1] §7.1: plain filling "will produce mathematically-flat surfaces". With epsilon,
  "if too small an ǫ is used, then adding it to a cell may produce no change in its elevation;
  if too great an ǫ is used, then large depressions may be converted into mesas rising above
  the surrounding landscape". The paper recommends the C99 `NextAfter` function (the next
  representable float) and says even that cannot be guaranteed correct when precision
  approaches that of the storage type.
- Seeds and holes [1]: cells "not part of the DEM" carry a NoData value "assumed to be some
  extremely negative number", so they are lower than every data cell. The FEMA `overflow`
  documentation [5] states the consequence explicitly: NoData boundary cells get priority
  −∞, and with its default `fill_holes = False` "nodata regions act as drainage outlets".
  The Whitebox fill tool [6] instead has an optional `max_depth` ("maximum depression depth
  to fill") and says "Depression breaching should be preferred in most cases" (breaching:
  cutting a channel through the rim instead of raising the inside). Both are tool
  documentation, not peer-reviewed; medium-strength sources.
- Depressions as real features [3] (Barnes, Callaghan and Wickert 2021, abstract and
  introduction read): models "often eliminate depressions through filling or breaching;
  however, this can produce unrealistic results"; depressions "often host lakes and
  wetlands". Their Fill-Spill-Merge method keeps depressions and fills them only with the
  water that reaches them. It targets hydrology models with rainfall; I do not know of a map
  generator that uses it.
- Locality [2] (Barnes 2016, parallel Priority-Flood, HTML read): "many of the cells in the
  central tile are part of a depression when the DEM is considered as a whole" and "In
  general, there is no way to guarantee locality and disparate tiles may need to be
  repeatedly loaded". The parallel method merges the tiles' perimeters into one global
  "spillover graph".
- Map generators: Azgaar's Fantasy Map Generator [7] (source read, `river-generator.ts`,
  `resolveDepressions`) does not use Priority-Flood. It loops over land cells lowest first and
  raises any cell not higher than its lowest neighbour to that neighbour plus 0.1, up to a
  configured number of iterations, and logs "Unresolved depressions" when the loop does not
  converge. Lakes are features with their own height; a lake that cannot be resolved late in
  the loop is marked `closed` and kept as a sink. So it keeps chosen depressions (closed
  lakes) and uses a fixed epsilon (0.1 height units) for everything else. Red Blob Games [8]
  lists "Ignore, Lake, Fill, Carve" for local minima; only the headings were readable (weak).
- City generators: I found no source in which a city generator drains a height field. I did
  not check Watabou's or ProbableTrain's code for this.
- Rust facts: `std::collections::BinaryHeap` needs `T: Ord`; its documentation says nothing
  about the order of equal elements [9], so equal keys must not reach it if order matters.
  `f64` implements `PartialOrd` but not `Ord`; `f64::total_cmp` gives the IEEE 754 total
  order, which "does not always agree with" `PartialOrd` (negative zero sorts before positive
  zero) [10].

## 2. Question 1: what goes wrong downstream without draining, and who would read it

Drainage matters only to a stage that asks "where does water go" or "where does water
stay". Reading the plan (ARCHITECTURE.md §4 and §9) stage by stage:

| Stage | Reads height? | Reads drainage? | Source in the repository |
|---|---|---|---|
| S0 rivers | no | no: courses are hand-authored | §4 S0 row; gaps report §6 |
| S0 coast, water mask, shoreline | yes, presumably | no | §4 S0 row |
| S0 coast `shore: salt_marsh`; land cover `marsh` in `outside[]` | not decided | not decided | ADR-056 (shore "picked by the generator", "ignored until milestone 2"); ADR-058 (covers "the generator places") |
| S1 macro (wards, walls) | not stated | no | §4 S1 row |
| S2 placement: `on_high_ground`, `on_low_ground` | yes: "candidate elevation in the top quartile of the site" | no | ADR-059; proposal §4 table |
| S3 streets, primary least-cost | likely, through slope | no; cost function open (#159) | §4 S3 row; §9 milestone 5 |
| S4 blocks, S5 buildings | not stated | no | §4 |
| S6 labels, S7 decorators | no geometry from height | no | §4 |
| Render (terrain PNG, milestone 2) | yes | no | §9 milestone 2 |

Findings (inference from the table):

1. No stage in ARCHITECTURE.md §4 names flow direction, flow accumulation, watershed or
   wetness as an input. The one place where drainage could become an input is the placement
   of marsh (land cover and the `salt_marsh` shore), because "where does water stay" is a
   natural rule for marsh. No decision says marsh is placed by wetness; ADR-058 only says the
   generator places it.
2. Least-cost streets do not need a drained field. A least-cost path search (Dijkstra or A*)
   runs on a cost graph; a pit is just a place with some cost, not a trap. Water flow
   follows "always downhill"; a road search does not.
3. `on_low_ground` and `on_high_ground` read elevation. If they read a filled field, the
   floor of a hollow would count as the height of its rim, so a landmark asked to sit "on low
   ground" would not see the hollow as low. This is a reason to keep the raw field available
   whichever option is chosen.
4. Rendering: a filled field drawn as terrain would erase a hollow's dip from the map. Same
   reason: the raw field is the one to draw.
5. What actually breaks without draining: nothing in the current plan. Problems appear only
   if a later decision adds flow-based features (generated streams, ditches, flooding in a
   seasonal pack, marsh by wetness). Then water would collect in every accidental pit.

Does version 1 terrain even have accidental pits? Inference, not checked by a run:

- Truro (`specifications/truro.yaml`) has a coast to the south-east, a slope of 10 m/km
  toward it, three hills (radius 150 m, height 30 m) and a river, and no hollows or lakes.
- A single smooth round hill on a plane slope produces a saddle on its uphill side, not a
  pit: along the slope the hill rises, across the slope it falls away. On a grid, rounding
  can still leave single-cell pits. Hills placed close together, a cliff facing uphill, or a
  plateau edge can enclose real pits. Whether Truro has any depends on the hill profile
  (cone, bell curve) and the cell size, neither of which is decided yet.
- Hollows (ADR-056) are pits by design.

## 3. Question 2: keeping a deliberate hollow or lake as a pit

Three ways, from the sources:

- **Seed the protected cells as outlets.** Priority-Flood starts from "seed cells" [1] §5;
  nothing in the algorithm requires seeds to be on the grid edge. Pushing every cell of a
  hollow (or only its lowest cell) at its own height makes the hollow a sink that is never
  filled; land around it drains into it. This is the same mechanism as NoData holes acting as
  outlets [1][5]. Sea and lake cells from the water mask can be seeded the same way.
- **Mask them out** (NoData with priority −∞) [1][5]: the cells take no part and act as
  outlets; their own heights are not changed.
- **Mark them after the fact**, as Azgaar does with `closed` lakes [7]: a second pass, in
  Azgaar's case with an iteration limit.

Consequences of seeding (inference): a hollow seeded as an outlet also drains any accidental
pit around it whose lowest spill path leads into it, so that pit is not filled either. A cell
set ("this cell is a protected outlet") becomes part of S0's output and must be keyed by
stable identifiers like everything else (hard rule 3). Rivers are a third candidate for
outlets: if river cells are seeded, land drains into the hand-authored river instead of
being filled up to a rim.

## 4. Question 3: cost and determinism

Cost (inference from [1] and [9]; not measured):

| Grid | Cells n | Heap (float or quantised keys) | Bucket queue (integer keys) |
|---|---|---|---|
| 100 × 100 | 10⁴ | about n · log₂ n ≈ 1.3 · 10⁵ comparison steps | about n |
| 1000 × 1000 | 10⁶ | about 2 · 10⁷ comparison steps | about n, plus one bucket per centimetre of height range |

Memory: one "done" flag per cell plus the queue; a heap entry of (elevation, cell identifier)
is 16 bytes with an `i64` and a `u64`, so at most about 16 MB at 10⁶ cells. Truro's site is
2400 m across, so 1000 × 1000 means 2.4 m cells and 100 × 100 means 24 m cells. The paper
gives no absolute times; a benchmark in the implementation issue would. I do not know the
run time on Jeffrey's machine.

Determinism:

- **Epsilon 0**: [1] says any queue order gives the same filled heights. In terms of the
  definition, each filled height is the lowest possible "highest point on a path to an
  outlet", which is unique. So the issue's test "two heap implementations give the same
  output" would pass for epsilon 0 even without a tie key; it then guards against a bug,
  not against order dependence.
- **Epsilon above 0, flow directions, watershed labels**: order matters [1]. The paper's tie
  key is an insertion counter, which gives first-in-first-out among equal heights and with
  it the shortest-path and multiple-outlet guarantees quoted above. The project's
  recommendation 11 proposes the stable cell identifier instead. Both are total orders, so
  both are deterministic under any heap. They differ in result (inference): an identifier
  is a hash (ADR-030), so among equal heights cells leave in hash order, not in distance
  order, and the paper's shortest-path guarantee does not carry over. An insertion counter
  is an index inside one stage function, which ARCHITECTURE.md §5 allows ("Indices are
  allowed inside one stage function"), as long as pushes happen in a fixed order (fixed seed
  order, fixed neighbour order). A third choice is (elevation, insertion counter) with seeds
  pushed sorted by identifier.
- **Float or integer heights**: `f64` keys need `total_cmp` [10], and negative zero sorts
  before positive zero there, which plain comparison treats as equal. ARCHITECTURE.md §5 asks
  for "integer-quantised comparisons" in topology decisions, and ADR-020 quantises
  coordinates to 0.01 m. Heights quantised to whole centimetres (`i32` or `i64`) give a key
  with `Ord`, no float comparison, and allow the O(n) bucket queue [1]. Filling itself only
  copies heights (max of two values), so it adds no float arithmetic.
- **Epsilon on quantised heights**: `NextAfter` [1] has no meaning for integers; the
  smallest step is 1 cm. Inference: a filled depression w cells wide then rises up to about
  w cm above its spill point; at 2.4 m cells a 250 m wide filled area is about 100 cells, so
  up to about 1 m: the "mesa" effect [1] warns about. With `f64` and `NextAfter` the rise is
  invisible, but `NextAfter` is not in `libm`'s list of functions the project has checked
  (ADR-020 checked transcendentals; I did not check whether `libm` provides `nextafter`).

## 5. Question 4: locality — how far does one added hill reach?

- Raw field (no fill): a hill changes only the cells under its radius. Local.
- Filled field (inference from the definition in [1], supported by [2]): a cell's filled
  height is set by the lowest spill path from it to an outlet. A new hill changes filled
  heights (a) under the hill, (b) in every cell of a depression whose lowest spill path ran
  through the hill's area, and (c) in a new depression the hill creates by damming ground
  uphill of it. Cells in (b) and (c) can lie anywhere on the grid; there is no distance
  bound. [2]: "there is no way to guarantee locality". Adding a hill never lowers any filled
  height (filling is monotone in the input heights).
- With epsilon, the slope pattern inside every affected flat changes too, and with a
  hash-order tie key the pattern can change in flats the hill does not even touch, if the
  set of queued cells changes (inference).
- Relation to ARCHITECTURE.md §5 "Locality": that rule is about landmarks ("Adding a
  landmark dirties its cell and ring only"). In version 1 landmarks do not change terrain,
  so a filled field does not break the landmark locality test. It does mean a terrain edit
  in the specification can change a filled field, and anything placed by it, far away. A
  locality test for terrain does not exist in §8; whether one is wanted is a question.

## 6. Question 5: options for the ADR

| Option | What it costs | What it enables | What it leaves open |
|---|---|---|---|
| (a) No draining in version 1 | Nothing now. Later flow features need a new ADR and possibly snapshot changes | Simplest S0; raw field is local; hollows stay pits automatically | Any later flow-based feature (marsh by wetness, streams, flooding) |
| (b) Priority-Flood fill, epsilon 0, hollows, lakes and sea (and optionally rivers) seeded as outlets | About 50 to 100 lines plus tests (the paper: pseudocode "about 20 lines", reference C++ "under 100 lines"); a second field (raw and filled) in S0 output or in memory; non-local terrain edits | A drained field for a future wetness or marsh rule; deterministic under any queue order [1] | Flats stay flat: no flow direction on them, so flow accumulation needs a further step (flat resolution, Barnes 2013, cited in [1]) |
| (c) Fill with epsilon (`NextAfter` on `f64`, or 1 cm on quantised heights) | Same as (b) plus a total order whose tie key changes the result (insertion counter versus cell identifier, §4); on 1 cm steps, visible "mesas" in large filled areas | Every cell has a downhill neighbour, so flow directions and accumulation work directly | Which tie key; whether `libm` or our own code supplies `nextafter` |
| (d) Lazy: a drainage query computed only when a later stage first needs it | No cost until a consumer exists; the query's contract (raw or filled, which outlets, epsilon) is still decided then | S0 output stays the raw field; drainage arrives in the PR of its first consumer, with its tests | Same choices as (b) or (c), postponed; "every run executes all stages" (§4) is unaffected because the query belongs to the consuming stage |

Not in the brief: breaching [6] and Fill-Spill-Merge [3]; both serve hydrology models and
cost more code than (b). The line estimate for (b) is mine, from the paper's own counts.

## 7. Questions for Jeffrey

One at a time when discussed; listed here in the order they depend on each other.

1. Does any version 1 feature need to know where water flows or stays? Options:
   1. No; option (a) or (d).
   2. Yes, marsh and the `salt_marsh` shore are placed on wet or low ground.
   3. Yes, something else (Jeffrey names it).
2. If draining is chosen: which option? (b) fill with epsilon 0, (c) fill with epsilon,
   or (d) lazy query.
3. If (b) or (c): which cells are outlets? Options: 1. map edge and sea only; 2. plus lakes
   and hollows; 3. plus river cells.
4. If (c): which tie key? Options: 1. (elevation, insertion counter), the paper's form;
   2. (elevation, stable cell identifier), recommendation 11; 3. (elevation, insertion
   counter) with seeds pushed in identifier order.
5. Heights as whole centimetres (integer keys) or as `f64` with `total_cmp`?
6. Do `on_low_ground`, `on_high_ground` and the terrain render read the raw field? (Findings
   3 and 4 in §2.) Options: 1. raw field; 2. filled field; 3. each rule says which.
7. Is a locality test for terrain edits wanted in ARCHITECTURE.md §8, given §5 of this
   report? Options: 1. no; 2. yes, for the raw field only; 3. yes, for both fields.

## Sources

1. Barnes, R., Lehman, C., Mulla, D. (2014). Priority-Flood: An Optimal Depression-Filling and
   Watershed-Labeling Algorithm for Digital Elevation Models. Computers & Geosciences 62,
   117–127. Preprint full text read: https://arxiv.org/abs/1511.04463 (strong: peer-reviewed).
2. Barnes, R. (2016). Parallel Priority-Flood Depression Filling for Trillion Cell Digital
   Elevation Models on Desktops or Clusters. Computers & Geosciences 96, 56–68. HTML read:
   https://arxiv.org/html/1606.06204 (strong).
3. Barnes, R., Callaghan, K. L., Wickert, A. D. (2021). Computing water flow through complex
   landscapes, Part 3: Fill-Spill-Merge. Earth Surface Dynamics 9, 105–121. Abstract and
   introduction read: https://esurf.copernicus.org/articles/9/105/2021/ (strong).
4. RichDEM documentation, Depression Filling (epsilon filling "may create elevated regions"):
   https://richdem.readthedocs.io/en/latest/depression_filling.html (medium: tool docs).
5. FEMA `overflow` documentation, fill algorithm (NoData outlets, `fill_holes`):
   https://fema-ffrd.github.io/overflow/algorithm-details/fill (medium: tool docs).
6. whiteboxR reference, `wbt_fill_depressions` (`max_depth`, breaching preferred):
   https://whiteboxr.gishub.org/reference/wbt_fill_depressions.html (medium: R wrapper docs,
   not the native WhiteboxTools manual, which returned 404).
7. Azgaar, Fantasy Map Generator, `src/model/physical/river-generator.ts`, `resolveDepressions`,
   default branch read 2026-11-09:
   https://github.com/Azgaar/Fantasy-Map-Generator/blob/master/src/model/physical/river-generator.ts
   (primary source for what that tool does; not evidence of what is correct).
8. Red Blob Games, Rivers, section 6 "Local minima": https://www.redblobgames.com/maps/rivers/
   (weak: only headings were readable).
9. Rust standard library, `BinaryHeap`:
   https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html (strong).
10. Rust standard library, `f64::total_cmp`: https://doc.rust-lang.org/std/primitive.f64.html
    (strong).
