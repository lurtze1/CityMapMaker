# Decision log — CityMapMaker

One entry per real decision. Newest at the bottom. Format: context, decision, consequences.
Superseded entries stay and point at their replacement.

## ADR-001 Requirements and research before code (2026-10-19)

Context: vibe-coded project with a single reviewer; risk of building the wrong thing fast.
Decision: write ground rules, requirements and architecture first; research prior art with
four reports in `docs/research/`; proceed in small reviewed milestones.
Consequences: slower start, months-long project by intent.

## ADR-002 Spec-driven generator, not a GUI driven by an AI (2026-10-19)

Context: the original idea was a map editor that an AI would operate like a human.
Decision: the input is a declarative spec; an AI writes specs. A GUI comes later and only
emits spec edits through the same command path.
Consequences: no UI is planned. If Jeffrey requests one, it becomes a milestone after 10 and
depends on `citymap-core` and `citymap-commands` only. The spec schema is the primary API and
is versioned (REQUIREMENTS.md F1).

## ADR-003 Rust (2026-10-19)

Context: compared TypeScript, Rust, C#, Java, C++, C, Odin, Python. Deciding factors were a
robust 2D geometry kernel, headless SVG rasterisation, deterministic floats, enforced module
purity, and the reviewer's fluency. Jeffrey knows C# and wants to learn Rust.
Decision: Rust, stable, native Windows MSVC toolchain. Python allowed only under `analysis/`.
Rejected: TypeScript (browser-first advantage not needed, weaker determinism), C# (close second,
chosen against for learning value), Java (JTS is excellent but tooling joyless), C++ (CGAL but
build and safety cost), Odin and C (no ecosystem).
Consequences: slower first weeks; a rough.js-style sketch renderer must be ported; a browser
preview is built only if Jeffrey asks for one, and then through WASM.

## ADR-004 Hash-based per-object RNG, never sequential (2026-10-19)

Context: pinned landmarks must survive seed changes and additions; adding one landmark must
not reshuffle the city. Sequential RNG consumption breaks this by construction.
Decision: every random value derives from `hash(master_seed, stage_salt, stable_object_id)`.
Stable ids derived from parents and keys, never array indices. See research-determinism.md.
Consequences: lint-level ban on ambient randomness; ordering discipline on all collections.

## ADR-005 Pins are resolved intent stored in the spec (2026-10-19)

Context: Lipp et al. 2011 protected layers and Azgaar's burg lock both keep user placements
across regeneration; Azgaar loses ids.
Decision: a landmark spec carries `constraints` (intent), optional `resolved` placement and
`pinned`. Pinned placements are inputs: copied into the model untouched and injected upstream
as fixed sites and obstacles. `--write-back` persists generated placements so they can be
pinned.
Consequences: the spec file is also the save file; no separate project format.

## ADR-006 Setting rules as rule packs (2026-10-19)

Context: Jeffrey does not want today's world physics baked in as binding future rules.
Decision: rules are data referencing registered evaluator types; packs have their own crates
and RNG salts; unknown evaluators are errors.
Consequences: Arcachon's seasonal districts and the "uncertain alleys" layer are the first
pack, built at milestone 8, not before.

## ADR-007 Ink line-art overview maps, SVG-first (2026-10-19)

Context: existing campaign maps are bought painted battlemaps; no city maps exist.
Decision: output is an ink-and-parchment overview handout with a numbered legend, SVG with
layer groups, PNG via resvg. No painted textures. Battlemaps stay out of scope.
Consequences: sketchiness lives in geometry (deterministic), SVG filters only for paper.

## ADR-008 Calibration cities and diversity check (2026-10-19)

Context: single-sample calibration bias ("every photo was a cloudy day").
Decision: calibrate on Truro, then Arcachon, then Bearna, chosen for different geometry; build
an expressive range harness early and tune presets per era, never from one city.
Consequences: `citymap-metrics` and `cli sweep` exist by milestone 5.

## ADR-009 Never invent lore (2026-10-19)

Context: vault rule "interview, don't invent"; a past AI-invented NPC file had to be deleted.
Decision: the generator never names anything; briefs list unknowns as GM? questions.
Consequences: filler buildings are anonymous; name generation is out of scope.

## ADR-010 Feedback loop is "open a PNG" (2026-10-19)

Context: no website needed; tool is for one person.
Decision: CLI writes files to `out/`; the user opens them. A folder-viewer wrapper is built only if
Jeffrey asks for one. No web UI is planned.

## ADR-011 Model in metres, scale is a render parameter (2026-10-19)

Context: cities differ hugely in size; previous hand-made maps used ad-hoc scales.
Decision: all model coordinates are metres. The renderer fits to page by default, draws a
scale bar and north arrow on every map, and accepts `--scale 1:N` for comparable series. A
legibility guard warns when buildings would be too small and suggests insets.
Consequences: Foundry export derives pixels-per-grid from metres; no pixel units in core.

## ADR-012 License: MIT OR Apache-2.0 (2026-10-19)

Context: Jeffrey wants the most open practical license and worries about AI-generated code
of unknown provenance.
Decision: dual MIT OR Apache-2.0, the Rust ecosystem convention. `LICENSE-MIT` and
`LICENSE-APACHE` at the root; crates declare `license = "MIT OR Apache-2.0"`.
Consequences: the license does not cure copied copyleft code; the mitigation stays the
CLAUDE.md rule to re-implement ideas and never paste from GPL or LGPL sources. `cargo deny`
checks dependency licenses in CI.

## ADR-013 CI on GitHub Actions, Windows and Linux (2026-10-19)

Context: the determinism claim "byte-identical on every CI platform" needs two platforms;
the repo already has a GitHub remote; Actions is free for a public repository and within
the free allowance for a private one at this project's size.
Decision: GitHub Actions, one Windows and one Linux job, both required for merge.
Rejected: GitLab CI (a second system next to the GitHub remote); no CI (determinism across
platforms would be untested).
Consequences: milestone 0 adds `.github/workflows/ci.yml`.

## ADR-014 Branch and pull request per step (2026-10-19)

Context: one developer with an AI pair; CI exists from milestone 0.
Decision: a branch and PR per step from the start; Jeffrey merges; CI gates the merge.
Rejected: direct commits to `main` with review in the IDE (no CI gate, no PR record).
Consequences: the founding documents were committed directly to `main` before this ADR;
that is accepted and not repeated.

## ADR-015 No releases; tiers are tagged (2026-10-19)

Context: a personal tool with one user.
Decision: no releases, no version bumps; `main` is the product; a `tier-N` git tag marks
each passed tier.
Rejected: semantic-versioned releases (nothing consumes them).

## ADR-016 Real-city specs live in the vault, not in this public repository (2026-10-19)

Context: the repository is MIT OR Apache-2.0 and public on GitHub; the specs carry private
campaign lore.
Decision: real-city specs live in the vault at `Maps/specs/` and are git-ignored here. Only
`specs/example.yaml`, a fictional city, is committed and used by tests.
Rejected: committing specs to a private repository (loses the public CI allowance and the
open-source intent); committing them publicly (leaks campaign lore to players).
Consequences: golden tests for the calibration cities run only on Jeffrey's machine with a
`CITYMAP_SPECS` environment variable pointing at the vault; CI runs goldens on the example
spec only.

## ADR-017 Constraint parameters are a closed enum; one trait object in core (2026-10-19)

Context: the first sketch put `Box<dyn EvaluatorParams>` inside `Spec`. A trait object there
cannot derive serde, schemars, `PartialEq` or `Clone`, and the usual fix (`typetag`) needs
link-time registration that does not build for `wasm32`, which breaks the purity check.
Decision: `ConstraintParams` is a closed enum with one variant per F3 rule, tagged by rule
name. Evaluators are the single trait object in core, held in a `BTreeMap` built explicitly
in the CLI. Packs add attribute stages, not constraint kinds; a new kind is a new variant
plus an ADR.
Rejected: trait object in the spec (above); `typetag`; `inventory`/`linkme` registration.
Consequences: the "rules are data" goal holds for packs; the constraint vocabulary is
versioned with the schema.

## ADR-018 RNG is Pcg64Mcg seeded by SplitMix64; distributions are our own (2026-10-19)

Context: `rand`'s `SmallRng` is documented as subject to change, and `gen_range` on floats
changed between `rand` 0.8 and 0.9. Every golden snapshot depends on the exact bit stream.
Decision: the generator is `Pcg64Mcg` from `rand_pcg`, pinned to an exact version. Streams
are seeded by `splitmix64(master_seed, stage_salt, object_id)`. The two distributions we
need, uniform `f64` in `[0,1)` from 53 bits and uniform integer in a range, are implemented
in `citymap-core::rng`, so a `rand` upgrade cannot change goldens.
Rejected: `SmallRng` (unstable algorithm); `rand` distributions (version-dependent);
`Xoshiro256**` (equally good; Pcg64Mcg chosen for its smaller state and one-line seeding).

## ADR-019 Workspace lint levels and the allow list (2026-10-19)

Context: CODING_STANDARDS.md requires `clippy::pedantic` with an allow list decided once,
never at a use site, and a ban on platform-dependent float methods. Lint: an automated
check that flags code the compiler accepts but a reviewer would reject.
Decision: lints live only in `[workspace.lints]` in the root `Cargo.toml`, inherited by
every crate through `[lints] workspace = true`. Levels: `missing_docs` warn,
`unsafe_code` forbid, `clippy::pedantic` warn at priority -1, `float_cmp` deny,
`disallowed_methods` deny with the banned `f64` methods `sin`, `cos`, `tan`, `atan2`,
`exp`, `ln`, `powf`, `hypot` listed in `clippy.toml`, each pointing at `libm`. CI runs
clippy with `-D warnings`, so a warn here fails CI.
Allowed, with reasons:
- `cast_precision_loss`, `cast_possible_truncation`, `cast_sign_loss`: geometry converts
  between integer cell indices and float metres constantly; each cast site is reviewed
  instead of flagged.
- `many_single_char_names`, `similar_names`: `x`, `y`, `dx`, `dy` are the correct names in
  geometry code.
- `module_name_repetitions`: `spec::Spec` is clearer than `spec::Root`.
- `must_use_candidate`: flags every pure function whose result could be ignored; all of
  core is pure, so the lint is noise.
Rejected: per-crate `#![warn]` attributes (two places to keep in sync); `clippy::restriction`
group (too broad, meant for picking single lints); allowing lints at use sites with a
comment (forbidden by the comment rule).
Consequences: adding or removing an allowed lint is a new ADR entry, approved by Jeffrey in
the PR that carries it.

## ADR-020 Geometry behind a facade; same-machine determinism first (2026-10-21)

Context: the geometry spike (PR #19, `docs/research/geometry-spike-2026-10.md`) compared
option A (`geo` with `spade` and `rstar` at the versions `geo` pins) and option B (`spade`,
`i_overlay` 9 and `rstar` 0.13 directly). A determinism proof (draft PR #66, closed) showed
that `libm` gives bit-identical results on Windows and Linux (140,020 inputs, hash
`0xbe131fd168e2f9fd`) and that std `sin`, `cos`, `tan`, `atan2`, `exp`, `ln`, `powf` and
`hypot` do not. An audit found that the `geo` functions on our path (point and shape
distance, simplify, concave hull, densify, rotate, skew, buffer, Voronoi) reach std `hypot`,
`sin_cos`, `tan` or `atan2`.
Implementation status: target contract. It is enforced once the `citymap-core::geom` facade,
its `geo` dependency and the Windows golden snapshots exist; until then it binds new code.
Decision (Jeffrey): all geometry goes through one facade module, `citymap-core::geom`, with
its own types; no other module imports `geo`, `spade`, `i_overlay` or `rstar`. Behind the
facade, version 1 uses `geo` for everything it offers. The facade's crate manifest has one
direct geometry dependency, `geo = { version = "0.33.1", default-features = false }`
with only the features the facade needs (the default `multithreading` feature stays off,
because core is single-threaded, CLAUDE.md hard rule 1); `spade`, `rstar` and `i_overlay` come in at the
versions `geo` selects, and `Cargo.lock` records the resolution. A direct dependency on any
of them, a version change, or a change to `geo`'s feature set is an ADR that Jeffrey
approves. Version 1 guarantees the same output for the same spec and seed on the
same machine and toolchain; identical output across platforms is a later goal (#67), reached
by replacing what sits behind the facade, never the callers. Our own code outside the facade
keeps the float rules (`libm` for transcendentals, the banned std methods), so only the
facade's backend needs replacing later. Golden snapshot tests run on Windows in CI, the
reference platform; Linux CI runs every other test. Jeffrey decides when #67 is complete. The four spike rules hold: jittered
sites only, a minimum site spacing, coordinates quantised to 0.01 m after every overlay,
zero-area polygons rejected at the boundary.
Rejected: option B now (about 200 to 300 lines of our own geometry before the first map;
Jeffrey wants the generator working first); a mixed backend now with our own code for the
risky functions (same reason); `geo` called directly without a facade (a later replacement
would touch every call site); one snapshot set for both platforms (fails on one of them
from day one); a snapshot set per platform (every visual change approved twice, the Linux
set cannot be viewed on Jeffrey's machine); no snapshot tests until #67 (visual regressions
go unnoticed).
Consequences: ARCHITECTURE.md §5 and §8, CODING_STANDARDS.md "Rust conventions" and CLAUDE.md
hard rule 5 name the facade and the same-machine scope. A Windows or Rust update can shift
unpinned geometry on the same machine (pins stay); a snapshot change after a toolchain update
is accepted like any other, with the update named in the commit subject. Closes #12.

## ADR-021 Spec schema version 1 (2026-10-20)

Context: `docs/proposals/spec-v1.md` proposed the version-1 spec with five open questions.
ADR-020 is reserved for the geometry spike (PR #19).
Decision: the proposal is accepted with these changes, decided by Jeffrey:
- The settlement block holds only `population`. `level` is dropped because it belongs to one
  game system (Pathfinder); `size_class` is dropped because status words carry
  culture-specific presumptions (a town with a parish church is European). Every feature a
  status implies is written in the spec explicitly: a market is an open space, walls are
  defences, a cathedral is a landmark.
- Entry roads hold `id`, `side` and `importance`; the free-text `destination` is dropped
  because the generator cannot use it. (Superseded by the second and fourth rounds: roads
  have `ends`.)
- A landmark `footprint` is optional and has three optional parts, `class`, `width_m` and
  `depth_m`. Each landmark kind has a default class; each class has a default width and
  depth; a written class overrides the kind's default, and a written width or depth
  overrides one side. Core receives only the resolved width and depth. (Superseded in part
  by the second round: landmark kinds became functions with a default class and shape.)
- The `fixed` constraint is dropped; pinning (F7) is the only exact placement.
- A constraint target is any spec id with a position: landmark, district, open space,
  crossing, river, lake, harbour, road, wall, gate, citadel. The harbour gets an `id` for
  this. (Road was added in the second round below.)
Rejected: keeping `size_class` and checking it against `population` (two fields for one
fact); `size_class` as a status that validation enforces (hides culture-specific rules in
the schema); a required `destination` (forces GM? questions for a label); footprint as
either a class or exact metres but not both (cannot override only one side); keeping both
`fixed` and `pinned` (two names for one mechanism; "fixed" also reads as "repaired");
keeping F3's four target kinds (makes "near the river" impossible to say).
Second round, decided by Jeffrey on 2026-10-20 while processing the CodeRabbit review:
- Roads are site features with a course (`from`, `to` a compass region or `core`; superseded
  by the fourth round: undirected `ends`), laid out
  in S0 like rivers, and can be constraint targets. Road courses are fixed in version 1; the
  town core forms where a road crosses a river or another road; through roads become main
  streets; ending roads end at the core or at the market square (superseded by the fifth and
  sixth rounds: ending roads end at the core or a pinned landmark or open space, and local
  roads exist). A gate stands wherever a
  road crosses a wall; `defences.gates[]` only names such gates or lists posterns and water
  gates. A road crossing a river implies a bridge (superseded by the third round: a crossing
  of the road's `river_crossing` kind, bridge or ford).
- An unpinned `resolved` is ignored unless `presets.reuse_positions` is on (default false,
  command-line override), which adds a soft `near_point` rule (within 50 m, weight 1). This
  amends ADR-017's closed list with `near_point`, which only the generator writes.
- Constraints are flat in YAML; in Rust one closed enum, each variant carrying `hard` and
  `weight`, no `#[serde(flatten)]` and no macro. A scratch test (serde 1.0.229,
  serde_yaml_ng 0.10) showed flatten plus `deny_unknown_fields` rejects every valid rule.
- Crossings join the candidate kinds; district rules are scored on wards.
- Landmarks, open spaces and crossings can be pinned; `resolved` stores the centre, rotation,
  width and depth.
- Districts have a closed `function` list of eleven (residential, craft, works, commerce,
  harbour, seat_of_power, military, religious_precinct, farmland, cemetery, open_ground) plus
  `wealth`, `group`, `enclosure` and `jurisdiction`, at one level.
- Landmarks have a closed `function` list of twelve (thirteen since the seventh round) plus a
  free `label`; rule packs map
  labels to defaults; an unknown label warns.
- Footprint classes are tiny (6 m), small (12 m), medium (25 m), large (50 m) and huge
  (100 m) with half-open area ranges and a default shape per function; without explicit
  width and depth the area is drawn within the class by the object's own random stream.
Rejected in the second round: bending roads toward the core in version 1 (puts the effect
before the cause in road-first towns); requiring a market square for every major road (many
towns held the market in the street); `resolved` as a hint by default (two meanings of "keep
this here"); nested `params:` YAML (more typing in every spec); `#[serde(flatten)]` (breaks
`deny_unknown_fields`); a macro for the shared fields (hides code from the reviewer); a
two-step reader (more code than the duplication it removes); landmarks-only pinning (a
market square or bridge could not be kept); pinning gates and wall shapes in version 1 (gates
follow roads; a wall shape is an area, not a point); two district levels (a fixed depth
blocks later recursive nesting); keeping `noble` and `shanty` as types (they are wealth,
not function); specific landmark kinds such as shrine and tower (they duplicate size); a
label that changes generation without a pack (a typo would silently change the map); always
the nominal size (stamped-out maps); a precinct class in version 1 (an area, not a building).
Consequences: REQUIREMENTS.md F1, F2, F3, F6, F7, F8, F9, F10, F12, F17 and Q2, the T4 review
question, and the ARCHITECTURE.md type and
stage descriptions are updated in the same PR, and F5 (embedding the Pathfinder 2e settlement
block) is withdrawn: game-system data never enters the spec. Future ideas: status templates
and game-system translators (#30), richer roads and roads that bend toward a site placed
first (#31), per-object soft pinning (#40), pinning a wall shape (#41), pinning listed gates
(#42), mixed districts (#43), special per-district attributes for Arcachon and Truro (#44),
recursive district nesting (#45), precincts (#46).
Third round, decided by Jeffrey on 2026-10-20 while processing the second CodeRabbit review:
- With several road–river or road–road crossings, the town core is the crossing on the most
  important road, then the one closest to the site centre; id order breaks only an exact tie.
  With none, the core is the point on the most important road closest to the site centre.
- Landmarks get `importance` (minor, major; default minor), one ordered list shared with
  roads, so levels can be added later (#48). F10 links every major landmark.
- Where a road crosses a river, S0 creates a crossing `<road>.<river>` whose kind is the
  road's `river_crossing` (bridge by default, or ford); its position is fixed by the courses.
  A road that crosses the same river more than once gets `<road>.<river>.<n>`, numbered from
  1 along the road from its first listed end.
- `near`, `far_from` and `min_spacing` measure edge to edge, from the candidate's rotated
  footprint to the target's geometry; `min_spacing` counts every placed landmark, open space
  and crossing. Pass/fail versus graded soft scoring is decided with the placement code (#49).
Rejected in the third round: the first crossing in id order (renaming an id would move the
town); the crossing closest to the centre regardless of roads (two minor tracks could beat
the main road's bridge); GUIDs as spec ids (unreadable in hand-written rules, and their order
is just as meaningless); rejecting a spec without a crossing (one-road towns exist);
making the market square the core (a town without one would be rejected); deriving
importance from footprint size (a small site can be very important); a `major` boolean (no
room for more levels); a number from 1 to 5 (says less to a reader than a word); requiring
every road bridge to be listed (validation would need geometry before generation); always a
bridge (a ford town could not be written); a crossing kind chosen by era (a hidden rule);
bridges as part of street geometry in S3 (two kinds of crossing in the model); measuring to
the target's centroid (wrong for a long river); measuring from the candidate's centre (an
80 m hall 50 m from a square would pass while standing 10 m from it); `min_spacing` per kind
or for landmarks only (cannot say "the square far from any bridge").
Fourth round, Jeffrey reviewing Claude's own fills on 2026-10-20:
- `presets.reuse_within_m` (default 50) and `presets.reuse_weight` (default 1) replace fixed
  numbers.
- The function-to-footprint table lives in the default rule pack as data; packs override rows.
- Roads have no direction: `ends: [a, b]`, each a compass region, `core` or a pinned landmark
  (pinned open spaces added in the sixth round);
  a road gate names its end by value. Rivers keep `from` and `to`, because water flows.
- Placement is in dependency order: pinned objects first; then an object follows everything
  its rules target, then the kind order (crossings, open spaces, landmarks), then spec order; mutual targets are a
  validation error.
Rejected in the fourth round: fixed reuse numbers (magic values); a stronger fixed default
weight (untested); the table in the schema (a change would be a schema change); `from`/`to`
on roads (implies a direction roads do not have); `first`/`last` gate values (still an
order to remember); a compass side for gates (ambiguous for bent or diagonal roads); road
ends at unpinned landmarks in version 1 (the landmark is placed after S0; issue #31); a
fixed kind order that rejects or skips rules pointing forward (cannot write "a ford near the
market").
Fifth round, Jeffrey on 2026-10-20 after an independent review agent and CodeRabbit:
- Roads are laid out in three steps within S0: every road without a `core` end; then the
  core; then roads with a `core` end. Roads without a compass end are
  local roads and may lie inside or outside the walls.
- One visibility rule: a rule, road end or gate points only to something laid out earlier or
  pinned; anything else is a validation error. A pinned object's rules are checked once
  their targets exist.
- `compass_region` is measured from the town core: eight 45° sectors and `centre` within
  `presets.centre_radius_m` (default 150).
- `encloses` lists districts and walls and is the only source for what lies inside a wall;
  nesting is transitive; separate walls stand side by side; the boundary follows water, with
  `along_water: wall | open`.
- F8's promise is per kind: a new object never moves objects of its own or an earlier kind.
- A gate's `end` is optional; a road that crosses its wall more than once without `end`, or
  never, stops generation with a message.
Rejected in the fifth round: aiming core-ending roads at the site centre (they may miss the
core); no core-ending roads (no side road into town); objects before districts (landmarks
would decide districts); skipping forward rules with a warning (quiet rule-skipping); case-
by-case fixes for each forward reference (the problem keeps returning); a dependency graph or
growth phases in version 1 (larger architecture change; issues #50, #51); a 3 × 3 grid over
the site (can lie outside the town; an absolute grid is issue #52); the whole footprint
inside a sector (a building on a boundary belongs nowhere); district rules and `encloses`
both deciding walls (two sources for one fact); dropping `encloses` (a wall's contents not
visible in one place); one shared object list to keep a global promise (schema change, and
a later bridge could not shape the streets); dropping the kind order (bridges and squares no
longer come first); requiring every road to leave the site (no castle-to-temple street); a
through road always crossing a wall twice (false for walls from water to water and for
separate walls).
Sixth round, Jeffrey on 2026-10-20 after a second independent review agent:
- A road with a `core` end ends at the core; S3 links the core to every market square with a
  main street; a road end may also name a pinned open space. This replaces "or at the market
  square" from the second round, which S0 cannot do because the market is placed in S2.
- District rules see only S0 objects, earlier districts and pinned objects; walls, gates and
  the citadel are made from the districts. A ward around a keep uses a pinned `defence`
  landmark. Several passes over the stages are a future idea (#51).
Rejected in the sixth round: ending at an unpinned market square (not placed yet in S0); only
roads to pinned markets (an unpinned market would not be connected); a pinnable citadel in
version 1 (a pinned landmark already covers it).
Seventh round, Jeffrey on 2026-10-21, the group-A questions of the cross-analysis
(docs/research/cross-analysis/README.md; facts in data/research/, batch 1 of #74 verified the
ones these rest on where the sources were reachable):
- Core ranking: among equally important roads a road–river or road–harbour candidate comes
  before a road–road crossing (Bosker and Buringh 2017: river +1.2, Roman road +0.4), and the
  generator prints a notice when that rule decided, pointing to a finer importance level
  (#48). A road meeting the harbour is a core candidate (Vitruvius; Ordinance 112).
- Castle-first towns are decided together with #31; the 150 m centre radius stays, with a
  revisit when #27 measures real cores (no source supports a value).
- `core` becomes a constraint target; the default pack gives the market a soft
  `near: core` (road-first towns, cross-analysis core.md §3.2; no figure behind it).
- A one-road town keeps a point core; the default pack widens the road around it into a
  street market; a better representation is #76.
- Footprints: no new labels in version 1 (religious, defence, caravanserai and ropewalk
  subtypes go to #77); `cathedral` 2 : 1 (measured outlines 1.7 and 2.0); `commerce` small
  (three of three market halls); a `micro` class (1 to 9 m²) keeps real well sizes
  (2.3 to 3.6 m²) and the renderer draws a minimum size; open spaces get size ranges per
  kind (market squares 2,500 to 11,250 m²).
- A population outside the law's range of 1,000 to 200,000 is extrapolated with a warning.
- An unpinned crossing is 8 m wide and as long as the river's `width_m`, plus 5 m on each bank
  for a bridge: Claude's estimate without a source, provisional default-pack data that Jeffrey
  revisits when street widths are decided (group C). Rejected: no default (placement, collision
  search and `min_spacing` need a footprint); the road's width (street widths are not decided
  yet).
- A thirteenth landmark function, `production` (smithy, carpenter, wheelwright, mill,
  tannery, brewery, ropewalk), default small 1.5 : 1, for named workshops a GM wants on the
  map; ordinary workshops stay in `craft` districts. The default is Claude's estimate without
  a source and is pack data.
Rejected in the seventh round: production buildings under `commerce` until #77 (a named
workshop would carry the wrong function, and a later addition is a schema change); no
workshop landmarks at all (a GM's named smithy could not be placed); ranking by road
importance only (ignores the water-first evidence); a river crossing always first (a minor track's ford would beat the main road);
the harbour never a candidate, or always the core; a `core` field now (roads would end in
the wrong place without #31); a population formula for the radius (no figure to scale by);
no default market rule, or a market-form enum now (a larger schema change); a core segment
(changes every measurement that uses the core); the four religious and four defence labels,
a caravanserai and a ropewalk label (subtypes are designed once, in #77); keeping
`cathedral` at 2.5 : 1 (nave against transept, not the outline); keeping `commerce` medium;
lowering `tiny` to 2 m² or a fixed `well` label (a micro class keeps real sizes and the
renderer keeps them visible); scaling market squares with population, or no sizes until
the stage exists; silent extrapolation, or rejecting a valid spec.

## ADR-022 Open review conversations block a merge (2026-10-20)

Context: CodeRabbit left 72 comments on PR #35, 23 on #38 and 21 on #20 that nobody had
processed, while its status check was green. Jeffrey wants unprocessed review points to stop
a merge the way failing CI does.
Decision: the repository ruleset requires review thread resolution. Every review comment is
fixed or denied with a reply, then resolved; the merge button stays disabled while any
conversation is open. CODING_STANDARDS.md "Review hygiene" describes the procedure.
Rejected: CodeRabbit's request-changes mode with a required approval (depends on a
third-party verdict and an option name not yet verified); a GitHub Actions job that fails on
open threads (a workflow to maintain, and resolving a thread does not trigger a new run by
itself).
Consequences: open PRs #19, #20, #35 and #38 cannot merge until their comments are
processed. Applied to the ruleset on 2026-10-20, before this entry merged.

## ADR-023 Research reports keep their sources' uncertainty (2026-10-20)

Context: CodeRabbit asked the research reports on PR #35 to follow the CLAUDE.md
explicit-language rule: no hedge words, a decision owner for every proposed rule.
Decision: research reports under `docs/research/` that report outside sources are exempt
from that rule. The folder decides what counts as a research report; Jeffrey decides, when
reviewing the PR, whether a document belongs in that folder. Where a source is approximate
or weak, the report says so in the source's own terms. The source's own wording decides;
where the report author adds a judgement of a source's strength, the report marks it as the
author's, and Jeffrey decides disputes when reviewing the PR. Documents that turn research
into decisions (ADRs, the cross-analysis, rule packs) follow the rule in full.
Rejected: applying the rule fully to research (rewriting an approximate figure as exact
misrepresents the source); a partial exemption that keeps hedges but adds decision owners
to every proposed rule in a report (decision owners belong where the decision is made).
Consequences: review comments asking a research report to remove a hedge are denied with a
reference to this entry.

## ADR-024 CodeRabbit reviews in rounds Claude starts (2026-10-20)

Context: CodeRabbit reviewed every push and paused itself after five reviewed commits (its
default), so on #20 and #39 the status read "Review paused" and a commit went unreviewed
while its check showed green.
Decision: `.coderabbit.yaml` sets `reviews.auto_review.auto_incremental_review: false`.
CodeRabbit reviews a PR automatically only when it is opened. A round is: Claude processes
every comment of one review (ADR-022), including comments listed outside the diff, pushes
the fixes in as many commits as they take, then comments `@coderabbitai review` once. Rounds
repeat until a review adds nothing new. Claude decides when that is reached and when a
commit only merges `main`; Jeffrey overrules either in the PR and decides whether to merge
after the last round. The same PR adds the ADR-023 research exemption to CodeRabbit's
`docs/**` instructions.
Rejected: turning the pause off (`auto_pause_after_reviewed_commits: 0`; reviews every push,
including half-finished work, and uses more of the rate limit); keeping the default pause of
five (a pause at an unpredictable moment, and a green check on an unreviewed commit); a
lower pause such as one or two (the same manual tagging, with the pause still surprising); a
tag after every commit (reviews half a batch of fixes).
Consequences: a "Review paused" or untriggered status is never read as approval; a merge
waits for a completed review after the last round of fixes. A commit that only merges
`main` into the branch needs no new round, because `main` was reviewed in its own PRs.
Stop rule (Jeffrey, 2026-10-21, after the fact PRs #54–#60 kept producing new Minor points
each round, many outside the diff): after three rounds on a PR, the remaining Minor and
Trivial points go into one follow-up issue per PR, and their conversations are resolved with
a reply naming that issue. Major and Critical points are always fixed before a merge.
Jeffrey decides the merge. Rejected: rounds until CodeRabbit finds nothing (each round of
fixes is new text for it to read, so the count did not reach zero); fixing one more round
without a new review (unreviewed changes).

## ADR-025 Research facts as YAML in git, queried through generated SQLite (2026-10-20)

Context: the research reports (#34, #35, #38, #28) hold hundreds of figures in prose, with
conflicts between reports and corrections in verification reports. Jeffrey wants them
cross-analysed and kept as data, so that generator defaults can cite them.
Decision: Jeffrey chose the hybrid setup on 2026-10-20. Facts live as YAML under
`data/research/` (`sources.yaml`, `units.yaml`, `quantities.yaml`, `facts/<topic>.yaml`),
one record per fact with source, locator, status and the report it came from; the format is
in `data/research/README.md`. `analysis/validate_facts.py` checks ids, references, units
against quantities, statuses and report paths; CI runs it and its unit tests on Windows and
Linux. `analysis/build_facts_db.py` writes `out/facts.sqlite`, git-ignored and never edited.
Python 3.12 with PyYAML 6.0.3 as the only dependency, under `analysis/` only. Superseded
facts stay with `superseded_by`; the prose reports stay unchanged. The cross-analysis fills
the facts one topic per PR and ends each topic with what it means for the spec; Jeffrey
decides those consequences.
Rejected: Markdown reports only (figures cannot be compared or queried); CSV files (no
nesting, no comments, fragile quoting for citations); PostgreSQL or another server (a
service to run and back up, while the whole dataset fits in one file); SQLite as the source of truth (binary diffs, no
review in a PR); Datasette as a dependency (optional browsing only).
Consequences: every new research figure that a decision cites becomes a fact first. A
generator default that rests on research names the fact ids it rests on.
Review of fact PRs (Jeffrey, 2026-10-21): CodeRabbit skips the per-topic fact, source, unit
and quantity files, because it cannot check a figure against its source and the validator
already checks their structure; it reviews the cross-analysis reports. An independent agent
checks a sample of 20 facts per topic against the report each fact cites. Rejected: CodeRabbit
on every file (hundreds of comments on data it cannot verify, and its hourly review limit);
sampling agents only (the reports turn research into decisions and need a full review).
Format, second version (Jeffrey, 2026-10-21, after the sample check found 15 errors in 120
facts): facts gain `approximate`, `year_basis` (closed list, required with years),
`year_approximate`, `source_via` and `disputes` (both sides must list each other; status
`disputed` requires at least one). The database gains the columns and a `disputes` table.
Rejected: keeping estimates, year meaning and disputes in `note` (free text, easy to drop,
not checkable); one-sided `disputes` (the sample found one-sided disputes); approximate years
as a `year_basis` value (then the basis of an approximate year is lost); invented ranges for
approximate years (the boundaries would not come from the source).
Year basis, two more values (Jeffrey, 2026-10-21, after the repair round gave the same case
two different values): `depicted`, the moment or period a plan or reconstruction shows; and
`valid_until`, a figure known to hold until that year with no known start, whatever changed
it afterwards ("before 1634", "until abandoned c. 1837"), recorded with `year_to` only. It
was first named `ended`; Jeffrey renamed it because the point is that the figure is certain
up to that year, not that something ended. Rejected names: `ended`, `until` (vague on its
own), `last_attested` (suggests nothing is known afterwards), `before` (misses "until
abandoned"); and a separate `before` value next to it (two values for one meaning). Rejected: filing depictions under `built` or `plan_date` (two
topics did, so a query on either mixed them up); dropping end dates (they place a figure in an
era, which the per-era presets need).

## ADR-026 A proof-of-concept slice before the full spec (2026-10-21)

Context: the build order (ARCHITECTURE.md §9) reaches the first image at milestone 2, after
the whole spec and its validation (milestone 1). Jeffrey wants a first proof of concept as
soon as the documentation is done, to see the pipeline work end to end.
Decision (Jeffrey): milestone 0b, after the workspace skeleton (milestone 0), is a thin
vertical slice: the spec subset that S0 roads need, the `geom` facade (ADR-020), S0 road
courses with the town core and road–river crossings, and an SVG plus PNG of them. Every rule
still holds: one small PR per issue, a ten-line design and Jeffrey's yes before each code
step, unit, property and golden tests, and a PNG Jeffrey looks at. Milestone 1 then completes
the spec on top of the slice; nothing in the slice is thrown away. The road course algorithm
in the slice is the simplest one the spec allows; the least-cost courses of REQUIREMENTS.md
F10 replace it at milestone 5. A PNG needs a rasteriser dependency (`resvg`, which
ARCHITECTURE.md §7 names for PNG output): it sits behind a facade in `citymap-render`, and
Jeffrey approves it with its own ADR in the PR that adds it.
Rejected: following §9 unchanged (the first image comes after the full spec and validation);
a throwaway prototype on a spike branch (fast, but without tests or review, and its code
would be rebuilt); SVG only without a PNG (the definition of done asks for a PNG Jeffrey
looked at).
Consequences: ARCHITECTURE.md §9 gains milestone 0b; GitHub gets a milestone "0b Proof of
concept" with one issue per smallest testable piece; milestone numbers 1 to 10 keep their
meaning.

## Open decisions

- YAML crate (`serde_yaml` is archived upstream; candidates `serde_yaml_ng`, `serde_yml`):
  settled at milestone 1; Jeffrey approves.
- Font metrics crate (`ttf-parser` versus `fontdue`) and the bundled fonts: settled at
  milestone 7; Jeffrey decides.
- Parcel subdivision method (straight skeleton versus recursive split): settled at
  milestone 6 by trying both on Truro; Jeffrey decides at tier T5.
- Label placement algorithm (greedy eight-position with R-tree, with or without simulated
  annealing): settled at milestone 7; Jeffrey decides at tier T6.
- Whether `sweep` metrics are compared against real town plans by hand or with a scripted
  corpus: Jeffrey decides at milestone 5.
