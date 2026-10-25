> Written 2026-10-19 for a TypeScript target, before ADR-003 chose Rust. The findings
> stand; the library tables and code snippets do not apply.

# Stable Regeneration for a Procedural Fantasy City Generator — Research Report

Date: 2026-10-19. Scope: (1) same spec + seed = identical map; (2) pinned landmarks survive seed changes and later additions; (3) adding a landmark only perturbs its neighbourhood; (4) setting rules are data/plugin driven.

Legend: **[verified]** = read in a primary source during this research; **[secondary]** = from search summaries / community sources, not read in full; **[unverified]** = my own inference or recollection, check before relying on it.

---

## 1. Recommended design (one page)

### 1.1 Seeding scheme: hash, never sequence

The single most important rule: **no stage ever draws from a shared sequential RNG.** Every random value is a pure function of `(masterSeed, stageSalt, stableObjectId[, k])`. This is the pattern Minecraft (`setPopulationSeed(worldSeed, blockX, blockZ)` → `setDecoratorSeed(populationSeed, index, step)` with salt `index + 10000*step`), Voxel Tools (`seed = globalSeed + hash(blockX, blockZ)`), Unreal PCG (per-point `Seed` attribute from `ComputeSeedFromPosition()`), and Eiserloh's "Noise-Based RNG" talk all use. Sequential RNG consumption is what makes "add one landmark" reshuffle the whole city: inserting one `rng()` call shifts every value drawn afterwards.

Concrete TypeScript scheme:

```ts
// 1. Master seed: a user string → 4×32-bit words via a string hash (xmur3 / MurmurHash3-style).
// 2. Stream key: a 32-bit hash of (masterSeed, stageSalt, objectId) — splitmix32/MurmurHash3 fmix.
// 3. Stream: sfc32 (128-bit state, passes PractRand) or splitmix32 (32-bit, fine for ≤ a few hundred draws).
type Rng = () => number;                         // [0,1)
function streamFor(seed: number, stage: string, id: string): Rng
// spatial values (noise-style, stateless): hash32(seed ^ salt, cellX, cellY) — "look-up in a giant table".
```

Rules:
- Stage salts are string constants (`"districts"`, `"streets.minor"`, `"landmark.place"`) hashed once; changing one stage's salt or algorithm cannot touch another stage's numbers.
- Object ids are **stable keys**, never array indices: grid-cell coordinates for spatial things, `parentId + ":" + localIndex` for children, the user's own `id`/name for landmarks, quantised geometry hash as last resort.
- Anything spatial that must "look continuous across cells" (noise, Poisson-disk-ish jitter) uses the stateless `hash(seed, cellX, cellY)` form so neighbouring cells can be recomputed independently and in any order (Voxel Tools' overlap trick).
- Never call `Math.random`; lint for it. Never let generation order, worker split, or iteration over a `Set`/object affect which numbers are drawn.

### 1.2 Pinning model: "resolved intent" stored in the spec, treated as hard constraints

A pin is a spec entry, not a generator output:

```jsonc
{ "id": "cathedral", "kind": "cathedral",
  "intent":   { "region": "quadrant:NE", "near": "market" },   // what the user asked for (soft/hard rules)
  "resolved": { "x": 812.4, "y": 203.7, "rot": 0.0, "footprintId": "..." },  // frozen on first generation
  "pinned": true }
```

- When a landmark is first generated, its resolved placement is written back into the spec ("snapshot"). Pinning flips `pinned: true`; from then on the generator **reads** `resolved` as a fixed input and never recomputes it. This is Lipp et al. 2011's `protected = true` mechanism (protected elements are copied to a higher layer before regeneration, then merged back on top) and Azgaar FMG's `lock` property on burgs (locked burgs are filtered out of regeneration and re-added afterwards).
- Seed changes therefore cannot move a pinned landmark: nothing about it is derived from the seed.
- Pinned landmarks are also **constraints on upstream stages**: they are injected as fixed Voronoi/Delaunay sites and as fixed "obstacles" in the street/parcel stages, so the city is generated *around* them (constraint-first), rather than placing them *after* and hoping they fit.
- Unpinned landmarks are placed in a **stable priority order** (pinned first, then by explicit `order`/creation timestamp stored in the spec). A landmark's placement may depend only on landmarks earlier in that order, so adding landmark N+1 cannot move landmarks 1..N by construction. Optionally "auto-pin on save": every generated landmark gets its `resolved` snapshot persisted, so even unpinned ones stay put unless the user explicitly asks for a reroll.
- Conflict handling when a new landmark collides with an existing one: local relaxation only — search outward from the intended spot in a deterministic spiral/ring order within a bounded radius; if nothing fits, report a constraint failure rather than silently moving older content.

### 1.3 Stage pipeline with locality and stable ids

```
spec ─► S0 site/terrain (noise: hash(seed, cell))
     ─► S1 macro layout: wall ring, district Voronoi (sites = pinned sites ∪ hashed free sites), compass-locked districts
     ─► S2 landmarks (priority order, constraint search; pinned ones are inputs)
     ─► S3 streets (major from S1/S2 anchors, minor per block; rng = stream(seed,"streets",blockId))
     ─► S4 parcels per block (rng = stream(seed,"parcels",blockId))
     ─► S5 buildings per parcel (rng = stream(seed,"bld",parcelId))
     ─► S6 labels/names (rng = stream(seed,"names",objectId))
     ─► S7 decorators (rule packs: weather, factions, flavour) — pure attribute stages
```

- Each stage is a pure function `(spec, upstreamOutputs, seed) → output`; outputs are immutable records keyed by stable id. Treat it like a Houdini/PDG node graph: a stage is dirty only if its inputs changed; recompute dirty stages only (incremental regen is an optimisation — correctness comes from determinism, not from caching).
- **Spatial partitioning for locality**: S1 assigns everything to a coarse grid (e.g. 64–128 px cells). Per-cell content (minor streets, parcels, buildings) is derived from `(seed, cellId)` plus a bounded neighbourhood (ring of 1). A pinned/added landmark marks its cell + ring dirty; everything else is bit-identical by construction. This is exactly the Minecraft/Voxel Tools chunk model.
- **Stable topological ids**: streets = hash of the two endpoint ids (endpoint = cell-anchored node id) rather than index in an array; blocks = sorted tuple of bounding street ids; parcels = `blockId:k`. Labels and user references point at these. When a street is genuinely removed/split, its id changes — that is desired ("far-away ones must not change" is guaranteed because far-away ids don't depend on the edit). Compare FreeCAD's "topological naming problem": references by auto-assigned internal index break on regeneration; references by stable, semantically derived names survive.
- Voronoi/Delaunay: pass sites to Delaunator **in a canonical order (sorted by stable id)**; pinned sites keep their cell because their coordinates are inputs. Avoid global Lloyd relaxation (it moves every site when one site is added); if relaxation is wanted, run it with pinned sites fixed and with a bounded number of iterations, and prefer per-cell jittered-grid sites (blue-noise-ish, stateless) over relaxation.

### 1.4 Rule-pack interface (data first, plugin second)

```ts
// A rule pack is JSON: a list of rules referencing *evaluator types* registered in code.
interface RulePack { id: string; version: string; requires?: string[]; rules: Rule[]; stages?: StagePlugin[] }
interface Rule { id: string; stage: "layout"|"landmarks"|"streets"|...; type: string;   // evaluator type
                 hard?: boolean; weight?: number; params: Record<string, unknown>; selector?: Selector }
// Core registry of evaluators (plugins add more):
interface Evaluator<P> { type: string; applies(ctx, candidate): boolean;
                         score(ctx, candidate, params: P): number | "reject" }     // hard → reject, soft → score
// Pure attribute stages (weather per district, faction, etc.) are plugins that add fields, never geometry.
interface StagePlugin { id: string; after: StageName; run(ctx, output, rng: (id:string)=>Rng): Attributes }
```

- Ship built-in evaluators: `compassRegion` (locks a district/landmark to N/NE/…), `nearTo`/`farFrom`, `onWater`/`onHill`, `insideWalls`, `minSpacing`, `density`. "Cathedral in the NE quadrant" is `{type:"compassRegion", hard:true, params:{region:"NE"}, selector:{kind:"cathedral"}}`.
- Hard rules act as filters (rejection sampling with deterministic candidate order); soft rules are summed into a score; ties are broken by the object's own hashed RNG, never by iteration order.
- Packs are loaded in declared order and merged by rule `id` (later pack overrides earlier), the same way Dwarf Fortress raws (`[WORLD_GEN]` token blocks, `objects/*.txt`) and Azgaar's options are plain data layered over a fixed engine.
- Each rule pack gets its own salt (`hash(packId)`) mixed into any RNG stream it requests, so adding a pack never changes the streams of core stages.
- Everything the solver needs for a later ASP/CP-SAT backend (fixed facts = pinned items, choice variables = free items, integrity constraints = hard rules, optimisation = soft rules) maps 1:1 onto this schema, so you can start with greedy/rejection placement and swap in a solver for the landmark stage only.

### 1.5 Determinism hygiene checklist (JS/TS)

1. Seeded PRNG only (`sfc32`/`splitmix32` with Murmur-style seed hashing, or `pure-rand` xoroshiro128+ with `jump` for streams); `Math.random` banned.
2. Integer hashing with `Math.imul`/`>>> 0`; coordinates quantised to integers before hashing.
3. Only `+ - * / sqrt floor` are bit-exact across engines; `Math.sin/cos/pow/exp/log/atan2/hypot` are implementation-approximated and measurably differ between V8/JSC/SpiderMonkey and even V8 versions. Keep them out of anything that decides topology (which side of a line, which cell wins) or vendor an fdlibm port.
4. Canonical ordering before every geometry call and every sort; comparators total (break ties by stable id).
5. `Map`/`Set`/array iteration order is deterministic (insertion order) but object key order puts integer-like keys first — never rely on `for…in` over objects for generation order.
6. Delaunator/d3-delaunay use Shewchuk robust predicates; same input array → same output on the same engine. Guard against duplicate points (skipped silently by `Delaunator.from`) and collinear degenerate input.
7. Golden-master tests: hash the full output for a fixed `(spec, seed)` in CI, on Node and in a browser.

---

## 2. Evidence

### 2.1 Hierarchical / per-object seeding and local determinism

**Minecraft (Java Edition).** Yarn-mapped `ChunkRandom` documents the hierarchy **[verified, Yarn docs]**:
- `setPopulationSeed(worldSeed, blockX, blockZ)` — seeds decoration/animal placement for a chunk from the world seed and the chunk's block origin (`chunkX*16, chunkZ*16`), mixed through several layers of hashing; only the low 48 bits of the world seed matter because Java `setSeed` truncates.
- `setDecoratorSeed(populationSeed, index, step)` — one stream per feature; salt = `index + 10000 * step` (feature index in list + generation-step ordinal). The doc explicitly warns that the small salt causes "weird alignments between features whose indices are close together" — a lesson for us: use a *good* mixer, not an additive salt.
- `setCarverSeed(worldSeed, chunkX, chunkZ)` for caves/ravines; `setRegionSeed(worldSeed, regionX, regionZ, salt)` for structure starts in a region.
- Data-driven structure sets expose `salt`, `spacing`, `separation` per structure family in JSON (custom worldgen). **[secondary]**
- Source: https://maven.fabricmc.net/docs/yarn-24w06a+build.7/net/minecraft/util/math/random/ChunkRandom.html ; https://minecraft.fandom.com/wiki/Custom_world_generation/structure_set ; https://github.com/IsolatedSingularity/Minecraft-Generation

**Voxel Tools (Godot).** Explicit explanation of the failure mode and the fix **[verified]**: seeding every block with the world seed gives "trees at the same position in every block"; fix is `rng.seed = global_seed + hash(Vector2i(block.x, block.z))`. Neighbouring blocks' trees are recomputed by calling the same function with neighbour coordinates — no shared state. https://voxel-tools.readthedocs.io/en/latest/procedural_generation/

**Squirrel Eiserloh, GDC 2017 "Math for Game Programmers: Noise-Based RNG".** Replace stateful RNG with a hash `noise(index, seed) → 32 bits`: random access, trivial reseeding, record/playback, lock-free parallelism. Squirrel3 had a bit-mixing flaw; use SquirrelNoise5. **[secondary]** https://www.youtube.com/watch?v=LWFzPP8ZbdU ; https://gist.github.com/kevinmoran/0198d8e9de0da7057abe8b8b34d50f86 ; https://x.com/squirreltweets/status/1421251894274625536

**Unreal Engine PCG framework.** Deterministic by default; each point carries a `Seed` attribute; GPU docs recommend `ComputeSeedFromPosition()` — if all points share one seed, random selection collapses. Community reports of child nodes using their own stream instead of the component seed (i.e. the "plug every stream into the root seed" rule). **[secondary]** https://dev.epicgames.com/documentation/unreal-engine/using-pcg-with-gpu-processing-in-unreal-engine ; https://bugnet.io/blog/fix-unreal-pcg-graph-deterministic-seed-not-applying-children

**No Man's Sky.** 64-bit seed; chained seeding — a star's position seeds its planetary system, a planet's position seeds its features; content is regenerated on demand so each location must come out identical on each visit; little server-side data. Exact hash not documented. **[secondary]** https://en.wikipedia.org/wiki/Development_of_No_Man's_Sky ; https://www.youtube.com/watch?v=C9RyEiEzMiU

**Elite Dangerous (Stellar Forge).** Sector/boxel coordinates (id64 address) derive the seed for the system; a density bitmap/model per sector; **hand-authored systems are overrides layered on top of the procedural output** ("Stellar Forge does its thing first, then anything handcrafted replaces the procgenned stuff"). Name generation uses a wang32-style hash. **[secondary, forum + reverse-engineering project]** https://forums.frontier.co.uk/threads/myth-busting-on-stellar-forge-and-the-generation-of-everything-from-stars-to-rocks.517029/ ; https://deepwiki.com/19h/ed-stellarforge

**Dwarf Fortress.** Four separate seeds (world, history, name, creature) in the `[WORLD_GEN]` block — a coarse form of stream separation so e.g. names can be rerolled without changing terrain. **[secondary]** https://dwarffortresswiki.org/index.php/Worldgen_examples

**threejs-worldgen rule #39** — an example of codifying the invariant: "no Math.random in generation code; every value derives from the world seed hashed with position, tile/cell or instance ids; same seed and options yield the same world regardless of streaming order, machine or worker." **[secondary]** https://github.com/AskAlice/threejs-worldgen/issues/39

**Why sequential consumption breaks stability** (general guides): if two subsystems share one PRNG, the result depends on which ran first; remedy is seed splitting/hashing per subsystem and a fixed consumption order. **[secondary]** https://www.abratabia.com/procedural-generation/

### 2.2 Persisting edits over regeneration (Lipp 2011, CityEngine, others)

**Lipp, Scherzer, Wonka, Wimmer 2011, "Interactive Modeling of City Layouts using Layers of Procedural Content", CGF 30(2), EG 2011.** **[verified, read the PDF]**
- Problem statement: "If the underlying procedural description changes, the whole layout has to be regenerated from scratch, losing all manual customizations since they are not persistent."
- Mechanism: a **layer stack** (streets, parcels, assignments per layer) merged via a global graph cut; **"Persistent local changes": the user marks elements `protected = true`; before regeneration the system copies all marked elements into a new higher-level layer, replaces the original layer with the regenerated layout, and the merge retains the protected elements** (Fig. 10).
- **Anchored assignments** extend the "locator" concept from their earlier façade work so parameter assignments (landmark distribution, priorities) are linked to layout elements and survive global regeneration.
- Local parcel update after a street move: find affected blocks (vertices with non-zero transform weight) → either smooth transform via mean-value coordinates or **local regeneration of only that block** when area/angle change exceeds thresholds. This is the "nearby adjusts, far away untouched" behaviour, implemented via block ownership relations.
- Notes that CityEngine (2010) had "some kind of automatic parcel updating, but no details … published."
- PDF: https://peterwonka.net/Publications/pdfs/2011.EG.Lipp.CityControl.final.pdf ; DOI https://onlinelibrary.wiley.com/doi/10.1111/j.1467-8659.2011.01865.x

**Lipp et al. 2012 "Procedural Generation of Parcels in Urban Modeling"** — claims the method is "persistent in the sense of being able to map individual parcels from before an edit operation to after" (stable parcel identity across edits). **[secondary]** https://dl.acm.org/doi/10.1111/j.1467-8659.2012.03047.x

**CityEngine "local edits"** (Tutorial 20): local edits are linked to the shape tree; changing the CGA rule structure can make previously applied local edits disappear — i.e. CityEngine's overrides are keyed by derivation path, and when the path changes the override is orphaned. Lesson: key overrides by *stable semantic ids*, not by derivation path. **[secondary]** https://doc.arcgis.com/en/cityengine/2023.0/tutorials/tutorial-20-local-edits.htm

**Azgaar's Fantasy Map Generator.** Burg lock (PR #612, discussion #610) **[verified via GitHub page text]**: a `lock` property on each burg; on "regenerate burgs" locked burgs are filtered out after setup, then re-added with cell/state updated and ids reassigned. Locked burgs survive remove-all, name/population regen; group, culture and capital status may still change; MFCG seed changes unless custom. Wiki: same seed + same options + same size → identical map, but "the seed cannot store user's changes, don't use it as a save." Lesson: locking by flag works but their ids are reassigned (a gap we avoid with stable ids). https://github.com/Azgaar/Fantasy-Map-Generator/pull/612 ; https://github.com/Azgaar/Fantasy-Map-Generator/discussions/610 ; https://github.com/Azgaar/Fantasy-Map-Generator/wiki/Quick-Start-Tutorial

**Watabou MFCG.** Seed + parameters reproduce the map; "Rebuild" regenerates at current size; "Reroll geometry" changes a building without changing the district shape; "Warp" is a local deformation with Apply/Discard. No pin/lock feature found. **[secondary]** https://forum.profantasy.com/discussion/10489/watabou-medieval-fantasy-city-generator ; https://gigazine.net/gsc_news/en/20220227-medieval-fantasy-city-generator-review/

**Dungeon Alchemist.** "Refresh Room" regenerates a selected room in its theme; generation can include/exclude objects, lights, walls, doors, floors, roofs; terrain regeneration removes placed objects but keeps rooms. No documented per-object lock across refresh found. **[secondary, could not fetch wiki]** https://dungeonalchemist.fandom.com/wiki/Sidebar_%E2%80%93_Room_Menu ; https://dungeonalchemist.fandom.com/wiki/Features

**Parametric CAD "topological naming problem".** References to faces/edges by auto-assigned internal id break when an earlier feature regenerates; the fix direction is persistent naming (semantic/derivation-based ids) and attaching to stable datums instead of regenerated topology. FreeCAD 1.0 added a toponaming algorithm. **[secondary]** https://github.com/FreeCAD/FreeCAD-documentation/blob/main/wiki/Topological_naming_problem.md ; https://www.sciencedirect.com/science/article/pii/S1110016818300814

**Houdini/PDG.** Cooking is dependency-driven: only dirty/out-of-date nodes recook; work items are regenerated in place and diffed against existing items; dirtying during a cook is invalid. Point-id stability under topology change is *not* guaranteed in SOPs (community workaround: hide rather than delete to keep point numbers). Substance Designer: node-based non-destructive graph, no override-layer concept found. **[secondary]** https://www.sidefx.com/docs/houdini/tops/cooking.html ; https://forums.odforce.net/topic/44604-reproject-dirty-animation-for-new-point-ids/

### 2.3 Constraint-first placement

**ASP / clingo (Smith & Mateas 2011; PCG Book ch. 8, Nelson & Smith).** **[verified, read ch. 8 PDF]** Generate-and-test: choice rules define the design space (`5 { contains(T,wall) : tile(T) } 10.`), integrity constraints prune it (`:- contains((1,1),wall).`), facts fix things (a pinned landmark is just a fact `at(cathedral, 812, 203).`), `#minimize` gives soft constraints; `--rand-freq=1` randomises solver choices (clingo's `--seed` controls that). Good for the *landmark* stage (tens of objects); not for parcels/buildings (thousands). https://www.pcgbook.com/chapter08.pdf ; https://www.researchgate.net/publication/224240207_Answer_Set_Programming_for_Procedural_Content_Generation_A_Design_Space_Approach ; thesis on constraint-based generation for games: https://purehost.bath.ac.uk/ws/portalfiles/portal/284256559/Smith_Procedural_Constraint_based_Generation_for_Game_Development_Thesis_Acknowledgements_.pdf

**OR-Tools CP-SAT.** `AddNoOverlap2D` over x/y interval variables for rectangles; fixed obstacles are intervals with constant start (official scheduling recipe "no overlap with fixed activities"); pinning = constant domain on the position variables. Determinism: set `num_workers=1` and a fixed `random_seed` for reproducible solutions **[unverified for CP-SAT specifically; multi-worker CP-SAT is known to be nondeterministic]**. https://github.com/google/or-tools/blob/stable/ortools/sat/docs/scheduling.md ; https://d-krupke.github.io/cpsat-primer/advanced_modelling.html

**Poisson-disk with fixed points.** Bridson 2007 seeds the background grid and active list with one sample; pre-placed pins can be inserted into the grid (and optionally the active list) before the loop so candidates are rejected against them. Pins closer than `r` to each other must be tolerated explicitly. **[Bridson verified; the fixed-point variant is my inference, no source found]** https://www.cs.ubc.ca/~rbridson/docs/bridson-siggraph07-poissondisk.pdf . For stability prefer *stateless jittered-grid blue noise* (`hash(seed, cellX, cellY)` → offset within cell) — adding a pin only invalidates the cells within `r`. **[unverified design claim]**

**Voronoi with fixed sites / Lloyd.** Lloyd relaxation moves every site, so it must be restricted to free sites with pinned/boundary sites fixed (vorflow PR #32 does exactly this; the Blue Noise Plots paper constrains one coordinate). No source found on incremental Delaunay insertion preserving cells; standard result: inserting a site only changes cells adjacent to the new cell **[unverified]**. https://github.com/rhugman/vorflow/pull/32 ; https://arxiv.org/pdf/2102.04072

### 2.4 Incremental / localised regeneration and stable ids

- Lipp 2011 block-ownership + per-block local regeneration (see 2.2) is the direct precedent for dirty-region recompute in a city generator. **[verified]**
- PDG dirty-node semantics (see 2.2). **[secondary]**
- Minecraft chunks: each chunk's population seed depends only on `(worldSeed, chunkX, chunkZ)`, so chunks can be generated in any order and still match; features crossing chunk borders are handled by generating into neighbours deterministically. **[verified]**
- Stable-ID literature is from other domains (Ensembl stable ids via location-based mapping across re-annotation; entity-resolution frameworks carrying forward prior ids; "fewest fields that still disambiguate" natural keys). No PCG-specific paper found; the `parentId:childIndex` / `hash(cell)` scheme is standard practice but **[unverified as a cited technique]**. https://asia.ensembl.org/info/genome/stable_ids/index.html ; https://zavod.opensanctions.org/best_practices/entity_id/

### 2.5 Rule/plugin architectures for PCG

- **Dwarf Fortress raws**: world rules live in text token files (`[WORLD_GEN]` block with elevation/rainfall/temperature ranges, civ counts, caps; `objects/*.txt` for creatures/biomes); population caps are only changeable by editing raws; DFHack Lua exposes `world` with worldgen parameters. **[secondary]** https://dwarffortresswiki.org/index.php/DF2014:Advanced_world_generation ; https://dwarffortresswiki.org/Lua%20scripting
- **Minecraft data packs**: worldgen (biomes, structure sets with salt/spacing, placed features) is JSON; the engine is fixed, content is data. **[secondary]** https://minecraft.fandom.com/wiki/Custom_world_generation/structure_set
- **Azgaar FMG**: options + lock flags as data; per-feature regeneration tools; maintainers point out that per-feature locks had to be added one feature at a time — argues for a generic `pinned`/`protected` flag on every entity type from day one. **[verified]**
- **Unreal PCG**: node graph where rules are graph assets; per-point attributes drive downstream nodes (attribute-stage pattern). **[secondary]**
- **WFC/model synthesis**: adjacency constraints are data (tile sets + allowed neighbours), tiles can be enabled/disabled and extra constraints added — the "constraints as data" pattern at the tile level. **[secondary]** https://blog.ptidej.net/procedural-generation-using-wave-function-collapse/
- No literature found under the term "setting-specific rule packs"; the closest formalism is ASP where a rule pack is literally an extra `.lp` file appended to the program (`clingo core.lp style.lp`). **[verified for ASP]**

### 2.6 Determinism pitfalls in JavaScript/TypeScript

- `Math.random` is unseedable (spec). Seedable libraries: **seedrandom** (ARC4 default; alea, xor128, xorwow, xorshift7, xor4096, tychei; `state:true` for save/restore; string seeds; several× slower than native) **[verified README]** https://github.com/davidbau/seedrandom ; **pure-rand** (xoroshiro128+, xorshift128+, mersenne, congruential; recommends `jump()` to split streams rather than unrelated seeds; xoroshiro128+ low bits fail linearity tests) **[verified README summary]** https://github.com/dubzzz/pure-rand ; **bryc's PRNG survey**: sfc32 (128-bit, passes PractRand, "best 2^128-state JS PRNG"), mulberry32 (32-bit, fast, skips ~1/3 of values), splitmix32 (MurmurHash3 fmix-based; recommended over mulberry32), xoshiro128** (weak low bits); seed via xmur3/MurmurHash3 so similar seeds don't correlate. **[verified]** https://github.com/bryc/code/blob/master/jshash/PRNGs.md ; Vigna's shootout https://prng.di.unimi.it/
- **Transcendental Math functions are not cross-engine stable**: ECMAScript only fixes `sqrt/abs/floor/round` and arithmetic; measured disagreements between JSC and V8 of ~36% for `hypot`, ~9% `pow`, ~3% `cos`; Node 22 vs Chrome (different V8 versions) differed on sin/cos/atan2/exp/log/pow, desynchronising a client/server simulation; V8's `**` and `Math.pow` can differ due to constant folding. Fix used by lockstep projects: fdlibm ports built on `+ - * / sqrt floor`. **[secondary, self-reported GitHub measurements]** https://github.com/vbz-gg/clockwork2/pull/1 ; https://github.com/SebastianM1999/DinoIsland/pull/47 ; https://github.com/jjgroenendijk/sunset-driver/issues/471 ; https://cmichel.io/bugs-in-exponentiation-operator/ ; https://scrapfly.dev/posts/browser-math-os-fingerprint/
- **Delaunator / d3-delaunay**: uses `robust-predicates` (Shewchuk port) for orientation since Delaunator 5 / d3-delaunay 6, so degenerate input yields valid output; triangles CCW, hull CCW, `Delaunator.from` skips duplicate points. No published cross-run/cross-engine determinism guarantee, but the algorithm is sequential and input-order dependent, so canonical input ordering + same engine ⇒ identical output; robust predicates remove the floating-point-ambiguity source of divergence for orientation but *not* for circumcenter coordinates (plain float arithmetic — deterministic for +,-,*,/ under IEEE 754). **[robustness verified; determinism reasoning unverified]** https://observablehq.com/@fil/d3-delaunay-6-robustly-released ; https://github.com/mapbox/delaunator/blob/main/README.md
- Iteration order: `Map`/`Set`/arrays iterate in insertion order (deterministic); object property order places integer-like keys ascending first, then strings in insertion order; `Array.prototype.sort` is stable since ES2019 but a non-total comparator still gives engine-dependent results. **[unverified, from the ECMAScript spec as I recall it]**
- `JSON.stringify` of the spec for hashing must use sorted keys (canonical JSON) if the spec hash is used as part of the seed. **[unverified design note]**

---

## 3. Gaps / things not confirmed

- Could not fetch the TU Wien project page or the Dungeon Alchemist wiki; Dungeon Alchemist lock behaviour unknown.
- No primary source for No Man's Sky's or Elite's exact hash functions; only chained-seeding descriptions.
- No PCG paper found on stable entity ids or on "rule packs"; the recommended id scheme is engineering practice, not cited literature.
- CP-SAT reproducibility settings (`random_seed`, single worker) not checked against current OR-Tools docs.
