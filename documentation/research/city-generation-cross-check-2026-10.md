# City generation cross-check, 2026-10-27

> A research report (ADR-023): it keeps the uncertainty of its sources. A claim here binds only
> through the decision log entry that cites it, and a figure such an entry cites becomes a fact
> under `data/research/` first (ADR-025). Every recommendation is marked **sourced** (a verified
> source states it) or **inference** (the report author's reading); Jeffrey decides each one in
> its own issue.

## Why this report exists

Jeffrey had another agent write a guide, "Procedural Map & City Generation: A Complete Guide"
(27 October 2026, a Claude Doc shared from outside the ardo-one.nl organization, so it is not
in this repository). This report checks that guide's main claims against independent sources
and asks what both mean for CityMapMaker's plan.

## Method

1. Claude read the guide in full (Jeffrey pasted its text into the session). The research run
   itself worked from Claude's summary of the guide's claims, not from the guide.
2. A deep-research workflow ran six search angles, fetched 25 sources and extracted 120
   falsifiable claims. Each checked claim got three independent verification votes; two
   refutes killed it.
3. Budget limits: 25 claims were verified (20 confirmed, 5 refuted); 11 more were queued and
   dropped unverified. A topic without a verified claim below was **not checked**, which is not
   the same as refuted.
4. Claude added its own reading of the guide against this project's design (section 4). No
   source verified those points.

## 1. Where the guide is confirmed

| Guide claim | Verified source | Vote |
|---|---|---|
| Parish and Müller (2001) split road generation into global goals and local constraints; the local constraints snap to junctions, cut at crossings and extend to join roads | Purdue SIGGRAPH 2011 course slides [1]; Kelly and McCabe survey [3]; a verifier read section 3.3 of the original paper | 3-0 |
| Streets can be traced along a designed tensor field (Chen et al. 2008) | Chen et al. project page [2]; course slides [1] | 3-0 |
| Roads can be computed as least-cost paths over terrain (Galin et al. 2010 [9], read through [1]) | Course slides [1] | 3-0 |
| Lots are split recursively along their longest side | Vanegas et al. 2012 [4]; course slides [1] | 3-0 |

A *tensor field* gives every point two perpendicular preferred directions; streets traced along
them (*hyperstreamlines*) form two street families.

The sources add detail the guide left out. Parish and Müller's local constraints also prune a
segment back into legal ground, rotate it within a maximum angle (so roads follow a coastline),
and let a highway cross illegal ground up to a set length as a bridge or tunnel [1]. Chen et
al. blend basis fields with radial basis functions (smooth weights that fall off with distance
from a point), and the field has singularities, points where it has no defined direction [2].
The guide's "oriented bounding box" detail comes from Vanegas et al. 2012, not from Parish and
Müller, who split along the longest roughly parallel polygon edge [1][4].

## 2. Where the guide is wrong or unsourced

- **Road cost formula.** The guide gives `cost = |a − b| · (1 + α·s²) + water penalty` and
  marks it as its own heuristic. No verified source contains it. Galin et al. 2010 use a
  weighted anisotropic shortest path (cost depends on direction of travel) over an implicit
  graph, with parameterised cost functions and procedural bridges and tunnels [1][9]; the
  formulas themselves were not extracted.
- **Skeleton-based parcels.** Vanegas et al. 2012 do describe straight-skeleton subdivision
  (confirmed 3-0 [4]). A separate claim that it gives realistic results measured against real
  parcel data was refuted 0-3: the cited thesis [10] did not support it.

## 3. Findings that matter most for this project

### 3.1 Parcels (S4): oriented bounding box subdivision is locally stable

An *oriented bounding box* is the smallest rectangle, at any angle, that encloses a polygon.
Vanegas et al. 2012, section 4.3 and Algorithm 2 [4], checked against the primary PDF:

- The cut point is the midpoint of the box's longest edge plus a random shift; the cut runs
  across the long axis. Recursion stops when parcel area lies within a minimum and maximum
  area, or frontage width within a minimum and maximum width.
- The authors call the method "very stable": "the random seeds for the child parcels of a
  given parcel are computed before the recursive call ... localized changes in the block
  geometry will typically only affect the parcels nearby". They admit some edit sequences
  remove and re-add parcels. The stability is asserted, not measured.
- A post-process merges triangular and too-small parcels into the neighbour with the longest
  shared edge.
- CityEngine culls lots that are too small or have no street access [3].

### 3.2 Streets (S3): tracing leaves a graph clean-up to do

A claim that the tensor field can express local constraints as field elements was refuted 1-2:
the source [2] did not support it. Independently (inference from [1][2]), tracing alone leaves
dangling ends and near-miss junctions, so cut, snap and extend run as a graph pass after the
trace.

### 3.3 Growth-based generators are hard to steer

Kelly and McCabe (2006) [3] report that L-system generators (a grammar that rewrites road
segments into more segments) and agent-based generators are hard to control because of their
evolutionary nature. CityEngine users found a usable iteration count by trial and error, and
with real statistical input maps one cannot tell which patterns came from the generator and
which from the input. Confidence medium: one 2006 survey whose authors promoted their own
system, and it critiques the 2001 research CityEngine, not Esri's current product.

### 3.4 Float determinism

- The Rust standard library documents the precision of `f64::sin` and the other
  transcendental functions as non-deterministic across platforms, Rust versions and calls
  [11]; platform maths libraries (macOS, glibc, MSVC, wasm) differ in the last unit in the last place [5][11].
- Rapier, a Rust physics engine, is only "locally deterministic" by default: same machine,
  same Rapier version, same compiler. Its cross-platform mode needs IEEE 754-2008 targets, no
  `simd8` feature (SIMD: one instruction on several numbers at once), and transcendental
  functions through nalgebra's traits instead of std methods [6].
- Two claims were refuted 0-3 because the cited crate README [5] did not support them: that
  `+ − × ÷ sqrt` alone give identical bits on every Rust target, and that `mul_add` must be
  banned. The refutation means the source was insufficient. It does **not** show that fused
  multiply-add (one rounding instead of two) or SIMD code paths are safe; they stay open risks.

### 3.5 Evaluating a generator

- Smith and Whitehead 2010 [7]: example outputs, output counts and generation speed do not
  show a generator's range. They generate 10,000 outputs, score each on two emergent metrics
  and plot a 2D histogram with hexagonal bins, which exposed a bias toward linear levels in
  their platformer generator. Metrics must measure properties of the output, not the
  parameters that steer it. The paper studies 2D platformer levels; applying it to cities is
  inference.
- Boeing 2019 [8]: every street edge's compass bearing and its reverse go into 36 bins of 10
  degrees, shifted by −5 degrees. Orientation entropy H_o measures how evenly bearings spread
  (in nats, entropy with the natural logarithm): high means every direction, low means a grid.
  Orientation order φ = 1 − ((H_o − H_g) / (H_max − H_g))², with H_max = ln 36 ≈ 3.584 and
  H_g = ln 4 ≈ 1.386; φ runs from 0 (uniform) to 1 (one perfect grid) and is not clamped.
  OSMnx, a Python library, computes it, so real towns can be measured the same way.
- Refuted 0-3: reading a specific φ value (Rome 0.005) as proof that a generated town looks
  medieval. Compare distributions against chosen real towns, not against a threshold.

## 4. Author's judgement on the guide (not verified by any source)

1. **Scope.** Tectonics, climate, biomes and erosion are world-map topics, a REQUIREMENTS.md §4
   non-goal for now; history simulation is also a §4 non-goal. Minecraft and the science-fiction
   parameters are outside a fantasy city overview. Together that is about half the guide.
2. **Hash folding.** The guide folds a cell into one hash input as `x + P·y + seed`. That is
   linear, so (x + P, y − 1) and (x, y) give the same input and the same value, as do
   (x − 1, seed + 1) and (x, seed). ARCHITECTURE.md §5 says spatial noise uses "stateless
   `hash(seed, cell_x, cell_y)`" without saying how the parts are mixed. They must be absorbed
   one by one, as `absorb` in `random.rs` does for stream seeds, not summed first.
3. **Poisson disc sampling can break locality.** Bridson's algorithm (evenly spaced random
   points) grows from one active list and draws from one stream in order, so inserting one
   point can shift every later point. One jittered site per cell (ARCHITECTURE.md §4, S1 row)
   with the locality rule (§5) avoids that.
4. **Order-dependent growth.** Parish and Müller's priority queue, agent growth and "existing
   roads cost less" make road N depend on roads 1 to N − 1. That keeps appending stable but
   not the one-ring locality of an edit.
5. **Generate, validate, reroll.** Rerolling a whole city breaks pin and edit stability. S2
   already bounds its search and then reports a failure (REQUIREMENTS.md F8); a reroll per
   block from that block's own stream would follow the same bound-then-fail pattern.
6. **No float determinism.** The guide's formulas use `exp`, `cos` and A* (a shortest-path
   search) priority queues with no word on platform maths or tie-breaking.
7. **Missing topics.** Label placement, line-art rendering and A4 legibility, output metrics,
   stability under edits, known failure modes of other generators, and Foundry export.

## 5. Recommendations

| # | Recommendation | Basis | Stage |
|---|---|---|---|
| 1 | Decide S4 as oriented bounding box subdivision with a street-access cull; record skeleton subdivision as the rejected alternative; minimum and maximum parcel area and frontage width are configurable fields, named in that ADR | sourced method [4][3], choice is inference | S4, milestone 6 |
| 2 | Derive each child parcel's seed from the parent's stable identifier and its side, never from recursion order | sourced principle [4], identifier scheme is inference | S4 |
| 3 | Make the longest-shared-edge merge of triangular and too-small parcels a named S4 sub-step | sourced [4] | S4 |
| 4 | After tracing secondary streets, run a graph pass with cut, snap and extend | inference from [1][2] | S3, milestone 5 |
| 5 | Give the road cost function its own ADR, every coefficient configurable, Galin's anisotropic cost named as an alternative; never cite the guide's formula as established | inference; Galin sourced [1][9] | S0 and S3 |
| 6 | Keep ADR-020's same-machine scope; add a continuous integration job that compares golden-output hashes between Windows and Linux and reports differences without failing | scope sourced [6][11], job is inference | continuous integration |
| 7 | In addition to the REQUIREMENTS.md §7 metrics, the sweep harness scores orientation entropy and φ, dead-end ratio, intersection density and parcel frontage, plotted as 2D histograms over many seeds | sourced method [7][8], metric list is inference | sweep, milestone 5 |
| 8 | Run the sweep on minimal synthetic sites as well as the calibration cities, so the generator's contribution is separated from the specification's | inference from [3] | sweep |
| 9 | Measure real towns with OSMnx on a fixed graph variant (unweighted bearings, simplified graph) recorded in an ADR | sourced tool [8], variant is inference | `analysis/` |
| 10 | State in ARCHITECTURE.md §5 that `hash(seed, cell_x, cell_y)` absorbs each part separately | inference (section 4 item 2) | S0, S1 |

## 6. Mistakes to catch early

| Mistake | Early-warning check | Basis |
|---|---|---|
| A parcel edit spreads to other blocks | Property test: change one block, every parcel in other blocks stays byte-identical | inference; [4] asserts stability without measuring it |
| Slivers and too-small parcels | Property test: after the merge, no parcel below the minimum area and no triangular parcel | inference from [4] |
| Dangling streets and near-miss junctions after tracing | Dead-end ratio in the sweep; unit tests of the cut, snap and extend pass | inference |
| Windows and Linux outputs drifting apart unnoticed | The report-only golden-hash comparison in continuous integration | inference from [6][11] |
| Two cells hashing to the same noise value | Unit test: neighbouring and offset cells give distinct hashes | inference (section 4 item 2) |
| Tuning presets on hand-picked examples | No preset change without a sweep histogram before and after | sourced principle [7] |
| Reading one metric value as "medieval enough" | Compare distributions against the chosen real towns | refuted claim [8] |

## 7. Not checked

No claim on these topics survived verification, so this report neither confirms nor
contradicts the guide on them: label placement; rough.js-style rendering and hachures;
legibility at A4; Foundry VTT and Universal VTT export; Priority-Flood (filling terrain pits so
water drains); Wave Function Collapse (filling a grid by neighbour rules) for cities; quality of
hash-based random number generators (Squirrel3, SplitMix, PCG); seeded Voronoi without
relaxation; lessons from Watabou, Azgaar, O'Leary, Stålberg and Dwarf Fortress; street metrics
beyond orientation. A second research run on these gaps follows this report.

Open questions the run left:

1. Which label placement algorithm suits a hand-drawn A4 overview with a numbered-legend
   fallback, and is it compatible with the locality rule?
2. Does Rust code generation on x86_64 MSVC and Linux ever contract `a*b + c` into fused
   multiply-add or take target-dependent SIMD paths for `f64`?
3. Which hash-based random number generator keys the per-object streams, and which statistical
   test suite (PractRand, TestU01) gates that choice?
4. Which real town plans form the reference distribution for the sweep metrics? The
   `town-plan-corpus.md` and `town-plan-corpus-world.md` reports hold the candidates.

## Sources

Opened by the verification agents on 27 October 2026.

1. Purdue CGVLab (2011). *Urban layouts*, SIGGRAPH 2011 course notes.
   <https://www.cs.purdue.edu/cgvlab/urban/sg_2011_course/umc_SG11_02_urban_layouts.pdf>
2. Chen, G., Esch, G., Wonka, P., Müller, P., & Zhang, E. (2008). *Interactive procedural street
   modeling*, project page. <https://www2.cs.uh.edu/~chengu/Publications/streetModeling/street_modeling.html>
3. Kelly, G., & McCabe, H. (2006). *A survey of procedural techniques for city generation*,
   reached through <https://citygen.pages.dev/2006/11/Procedural-City-Generation-Survey>
   (that page only links the PDF).
4. Vanegas, C. A., Kelly, T., Weber, B., Halatsch, J., Aliaga, D. G., & Müller, P. (2012).
   Procedural generation of parcels in urban modeling. *Computer Graphics Forum, 31*(2),
   681–690. <https://diglib.eg.org/handle/10.1111/v31i2pp681-690>
5. alice-det-math crate documentation (a small crate README; weak on its own).
   <https://docs.rs/alice-det-math>
6. Rapier. *Determinism*, user guide. <https://rapier.rs/docs/user_guides/rust/determinism/>
7. Smith, G., & Whitehead, J. (2010). Analyzing the expressive range of a level generator.
   PCG Workshop. <https://www.pcgworkshop.com/archive/smith2010analyzing.pdf>
8. Boeing, G. (2019). Urban spatial order: street network orientation, configuration, and
   entropy. *Applied Network Science, 4*. <https://link.springer.com/article/10.1007/s41109-019-0189-1>
   Binning and units read from arXiv preprint 1808.00600; the φ formula sat behind a paywall
   and was checked by arithmetic against the quoted H_g, not read.
9. Galin, É., Peytavie, A., Maréchal, N., & Guérin, É. (2010). Procedural generation of roads.
   <https://perso.liris.cnrs.fr/egalin/Articles/2010-roads.pdf> (fetched; its cost formulas
   were not extracted, so claims about it rest on [1]).
10. University of Glasgow thesis, the source of the refuted skeleton claim in section 2.
    <https://theses.gla.ac.uk/4975>
11. Rust standard library, `f64::sin`.
    <https://doc.rust-lang.org/std/primitive.f64.html#method.sin>

Fetched, with no verified claim in this report: Watabou devlogs
(<https://watabou.itch.io/medieval-fantasy-city-generator/devlog/1579/some-answers-and-comments>,
<https://watabou.itch.io/medieval-fantasy-city-generator/devlog/112574/073-bug-fixes-geojson-export>,
<https://itch.io/post/346778>); <https://phiresky.github.io/procedural-cities/presentation.html>;
<https://users.rust-lang.org/t/f64-ln-1p-platform-specific-behavior/77113>;
<https://pcg-random.org/posts/bugs-in-splitmix.html>;
<https://dash.harvard.edu/bitstream/1/2032678/2/AnEmpiricalStudy.pdf>;
<https://merl.com/publications/docs/TR97-13.pdf>; <https://github.com/rough-stuff/rough/wiki>;
<https://dungeondraft-encyclopaedia.gitbook.io/guide/final-steps/exporting-your-map/universal-vtt>;
<https://arxiv.org/pdf/1511.04463>; <https://arxiv.org/pdf/2308.07307>;
<https://felixturner.github.io/hex-map-wfc/article>;
<https://www.cs.purdue.edu/homes/bbenes/papers/Galin11CGF.pdf>; <https://records.sigmm.org/?p=12589>.
