# Decision log — CityMapMaker

One entry per real decision. Newest at the bottom. Format: context, decision, consequences.
Superseded entries stay and point at their replacement.

## ADR-001 Requirements and research before code (2026-10-19)

Context: vibe-coded project with a single reviewer; risk of building the wrong thing fast.
Decision: write ground rules, requirements and architecture first; research prior art with
four reports in `documentation/research/`; proceed in small reviewed milestones.
Consequences: slower start, months-long project by intent.
Rejected: none recorded; founding decision of 2026-10-19 (line added 2026-10-27, #117).

## ADR-002 Spec-driven generator, not a GUI driven by an AI (2026-10-19)

Context: the original idea was a map editor that an AI would operate like a human.
Decision: the input is a declarative spec; an AI writes specs. A GUI comes later and only
emits spec edits through the same command path.
Consequences: no UI is planned. If Jeffrey requests one, it becomes a milestone after 10 and
depends on `citymap-core` and `citymap-commands` only. The spec schema is the primary API and
is versioned (REQUIREMENTS.md F1).
Rejected, as named in the text above: a map editor an AI operates like a human.
No other alternative was recorded; founding decision of 2026-10-19 (line added 2026-10-27, #117).

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
Rejected, as named in the text above: sequential random numbers, which break pins by construction.
No other alternative was recorded; founding decision of 2026-10-19 (line added 2026-10-27, #117).

## ADR-005 Pins are resolved intent stored in the spec (2026-10-19)

Context: Lipp et al. 2011 protected layers and Azgaar's burg lock both keep user placements
across regeneration; Azgaar loses ids.
Decision: a landmark spec carries `constraints` (intent), optional `resolved` placement and
`pinned`. Pinned placements are inputs: copied into the model untouched and injected upstream
as fixed sites and obstacles. `--write-back` persists generated placements so they can be
pinned.
Consequences: the spec file is also the save file; no separate project format.
Rejected, as named in the text above: a separate project format.
No other alternative was recorded; founding decision of 2026-10-19 (line added 2026-10-27, #117).

## ADR-006 Setting rules as rule packs (2026-10-19)

Context: Jeffrey does not want today's world physics baked in as binding future rules.
Decision: rules are data referencing registered evaluator types; packs have their own crates
and RNG salts; unknown evaluators are errors.
Consequences: Arcachon's seasonal districts and the "uncertain alleys" layer are the first
pack, built at milestone 8, not before.
Rejected, as named in the text above: rules built into the code as today's world physics.
No other alternative was recorded; founding decision of 2026-10-19 (line added 2026-10-27, #117).

## ADR-007 Ink line-art overview maps, SVG-first (2026-10-19)

Context: existing campaign maps are bought painted battlemaps; no city maps exist.
Decision: output is an ink-and-parchment overview handout with a numbered legend, SVG with
layer groups, PNG via resvg. No painted textures. Battlemaps stay out of scope.
Consequences: sketchiness lives in geometry (deterministic), SVG filters only for paper.
Rejected, as named in the text above: painted textures and battlemaps.
No other alternative was recorded; founding decision of 2026-10-19 (line added 2026-10-27, #117).

## ADR-008 Calibration cities and diversity check (2026-10-19)

Context: single-sample calibration bias ("every photo was a cloudy day").
Decision: calibrate on Truro, then Arcachon, then Bearna, chosen for different geometry; build
an expressive range harness early and tune presets per era, never from one city.
Consequences: `citymap-metrics` and `cli sweep` exist by milestone 5.
Rejected, as named in the text above: calibrating on a single city.
No other alternative was recorded; founding decision of 2026-10-19 (line added 2026-10-27, #117).

## ADR-009 Never invent lore (2026-10-19)

Context: vault rule "interview, don't invent"; a past AI-invented NPC file had to be deleted.
Decision: the generator never names anything; briefs list unknowns as GM? questions.
Consequences: filler buildings are anonymous; name generation is out of scope.
Rejected, as named in the text above: a generator that names things.
No other alternative was recorded; founding decision of 2026-10-19 (line added 2026-10-27, #117).

## ADR-010 Feedback loop is "open a PNG" (2026-10-19)

Context: no website needed; tool is for one person.
Decision: CLI writes files to `output/`; the user opens them. A folder-viewer wrapper is built only if
Jeffrey asks for one. No web UI is planned.
Rejected, as named in the text above: a website or web UI.
No other alternative was recorded; founding decision of 2026-10-19 (line added 2026-10-27, #117).

## ADR-011 Model in metres, scale is a render parameter (2026-10-19)

Context: cities differ hugely in size; previous hand-made maps used ad-hoc scales.
Decision: all model coordinates are metres. The renderer fits to page by default, draws a
scale bar and north arrow on every map, and accepts `--scale 1:N` for comparable series. A
legibility guard warns when buildings would be too small and suggests insets.
Consequences: Foundry export derives pixels-per-grid from metres; no pixel units in core.
Rejected, as named in the text above: pixel units in the core and ad-hoc scales per map.
No other alternative was recorded; founding decision of 2026-10-19 (line added 2026-10-27, #117).

## ADR-012 License: MIT OR Apache-2.0 (2026-10-19)

Context: Jeffrey wants the most open practical license and worries about AI-generated code
of unknown provenance.
Decision: dual MIT OR Apache-2.0, the Rust ecosystem convention. `LICENSE-MIT` and
`LICENSE-APACHE` at the root; crates declare `license = "MIT OR Apache-2.0"`.
Consequences: the license does not cure copied copyleft code; the mitigation stays the
CLAUDE.md rule to re-implement ideas and never paste from GPL or LGPL sources. `cargo deny`
checks dependency licenses in CI.
Rejected: none recorded; founding decision of 2026-10-19 (line added 2026-10-27, #117).

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
Decision: real-city specs live in the vault at `Maps/specifications/` and are git-ignored here. Only
`specifications/example.yaml`, a fictional city, is committed and used by tests.
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

Context: the geometry spike (PR #19, `documentation/research/geometry-spike-2026-10.md`) compared
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
Added 2026-10-25 in #98 (Jeffrey approved the design): `geo = "=0.33.1"` with
`default-features = false` is the facade's dependency in `crates/citymap-core/Cargo.toml`,
with no features turned on, because every `geo` item the facade calls compiles without one.
The facade is the module `citymap-core::geometry` (full word, ADR-035), and only
`geometry.rs` imports `geo`; its module doc lists every `geo` function and type it calls.
Closes #73.

## ADR-021 Spec schema version 1 (2026-10-20)

Context: `documentation/proposals/specification-version-1.md` proposed the version-1 spec with five open questions.
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
(documentation/research/cross-analysis/README.md; facts in data/research/, batch 1 of #74 verified the
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
Decision: research reports under `documentation/research/` that report outside sources are exempt
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
`documentation/**` instructions.
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
Replaced 2026-10-26 by ADR-040 (Jeffrey): CodeRabbit reviews only when Jeffrey asks; the
rounds and the stop rule no longer apply.

## ADR-025 Research facts as YAML in git, queried through generated SQLite (2026-10-20)

Context: the research reports (#34, #35, #38, #28) hold hundreds of figures in prose, with
conflicts between reports and corrections in verification reports. Jeffrey wants them
cross-analysed and kept as data, so that generator defaults can cite them.
Decision: Jeffrey chose the hybrid setup on 2026-10-20. Facts live as YAML under
`data/research/` (`sources.yaml`, `units.yaml`, `quantities.yaml`, `facts/<topic>.yaml`),
one record per fact with source, locator, status and the report it came from; the format is
in `data/research/README.md`. `analysis/validate_facts.py` checks ids, references, units
against quantities, statuses and report paths; CI runs it and its unit tests on Windows and
Linux. `analysis/build_facts_db.py` writes `output/facts.sqlite`, git-ignored and never edited.
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

## ADR-027 Dependency licences checked by cargo deny (2026-10-21)

Context: ADR-012 licenses the project MIT OR Apache-2.0 and promises that `cargo deny`
checks dependency licences in CI. A copyleft crate pulled in several levels deep would
change what the project may be distributed under, and nobody reads every transitive
licence by hand.
Decision: Jeffrey approved on 2026-10-21. `deny.toml` at the repository root. Allowed
licences: MIT, Apache-2.0, Apache-2.0 WITH LLVM-exception, BSD-2-Clause, BSD-3-Clause, ISC,
Zlib, Unicode-3.0, CC0-1.0; any other licence fails the check, including GPL, LGPL, AGPL and
MPL in every version. Sources: crates.io only; git sources and other registries fail.
Wildcard versions (`*`) fail; path dependencies between our own crates are exempt;
cargo deny grants that exemption only to unpublished crates, so the workspace sets
`publish = false` (ADR-015: nothing is released). Two
versions of one crate in the tree give a warning, not a failure. RustSec advisories fail;
yanked crates give the default warning. CI runs `cargo deny check` on the Linux job only, through
`EmbarkStudios/cargo-deny-action` pinned by commit SHA (v2.1.1); the action is a Linux
container and the result does not depend on the platform. Adding a licence to the list
needs a new entry here.
Rejected: allowing MPL-2.0 (weak copyleft with file-level obligations to publish changes to
those files); no automated check (a copyleft crate deep in the tree would go unnoticed);
denying multiple versions (common in the ecosystem and outside our control; a warning is
enough to notice it); running the check on both CI jobs (the same result twice).
Consequences: every new dependency passes `cargo deny check` before merge. With no
third-party dependencies yet the check passes trivially; it guards future additions.

## ADR-028 serde for spec and model types; serde_json and proptest for tests (2026-10-21)

Context: issue #11 adds the `Metres` newtype (`citymap-core::units`), which owns
fixed-precision serialisation (CODING_STANDARDS.md "Rust conventions"). Spec and model
types need one serialisation contract that YAML, JSON and snapshot tests all share, and
ARCHITECTURE.md §8 requires property tests with `proptest`.
Decision (Jeffrey, 2026-10-21): three dependencies, each pinned to an exact version.
- `serde = { version = "=1.0.229", default-features = false }`, a normal dependency of
  `citymap-core`. `Metres` implements `Serialize` and `Deserialize` by hand, so the
  `derive` feature is off; `std` is off because `Metres` needs nothing from it. Turning
  on `derive`, `std` or `alloc` later (spec types holding `String` or `Vec` need one of
  the last two) is a feature change recorded as a new line in this entry.
- `serde_json = "=1.0.151"`, a dev-dependency of `citymap-core`, used only in tests to
  check that a value survives serialisation with the same bits.
- `proptest = "=1.11.0"`, a dev-dependency of `citymap-core`, with default features.
All three are MIT OR Apache-2.0, and so is every crate they pull in, or it offers one of
those licences as an option. Each version was published at least two weeks before
2026-10-21; newer releases of `serde_json` (1.0.152 to 1.0.154) and `proptest` (1.12.0)
were younger than that and were not chosen.
serde gets no facade (the facade preference of ADR-020): serde is a data-format contract,
expressed as trait impls and derives on our own types, not a library whose functions the
generator logic calls. A facade would wrap traits that every type must implement directly.
Rejected: writing our own parser and serialiser (far more code, and every format crate
would need its own adapter); `quickcheck` instead of `proptest` (ARCHITECTURE.md §8 names
`proptest`, and `proptest` shrinks a failing input to a smaller one more reliably); the
latest `serde_json` and `proptest` releases (less than two weeks old on 2026-10-21).
Consequences: the YAML crate from "Open decisions" must work with `serde` 1.x. A `serde`,
`serde_json` or `proptest` version change is a new ADR line that Jeffrey approves.
Amended 2026-10-25 (#94, Jeffrey approved the plan): `citymap-core` turns on the `serde`
features `derive` (the specification types derive `Serialize` and `Deserialize`) and
`alloc` (their `String` fields); `std` stays off. `citymap-cli` gets `serde = "=1.0.229"`
without default features, for the trait bounds of its YAML facade, and
`proptest = "=1.11.0"` as a dev-dependency for the round-trip property test.
Amended 2026-10-26 (#99, Jeffrey approved the plan in batch 1): `insta = "=1.49.0"` with the
`json` feature, a dev-dependency of `citymap-command-line`, for the golden snapshot tests of
ARCHITECTURE.md §8. Apache-2.0; published 2026-10-03, more than two weeks before. Rejected
by Claude: comparing JSON against a hand-kept file (§8 names `insta`, and `cargo insta
review` is how Jeffrey accepts a change). The model types derive `Serialize` only.
Confirmed 2026-10-27 (#111, Jeffrey): serde keeps no facade. The experiment of 2026-10-25,
a scratch workspace outside the repository: serde can be routed through a module of our own
with `#[serde(crate = "…")]` on every derive, but the derives and `#[serde(...)]` attributes
on our types stay serde's, so only the import path moves. Rejected by Jeffrey: that
re-export facade on all 25 derives (our types still depend on serde's derives).

## ADR-029 thiserror for library error types, without a facade (2026-10-22)

Context: CODING_STANDARDS.md "Rust conventions" says library crates use `thiserror` enums
for errors. `UnitError` (issue #11) is the first library error; the first draft of PR #82
wrote its `Display` and `std::error::Error` implementations by hand, and CodeRabbit
flagged the mismatch.
Decision (Jeffrey, 2026-10-22): `thiserror = "=2.0.21"`, a normal dependency of
`citymap-core` with default features. Every error enum in a library crate derives
`thiserror::Error` and states its message in an `#[error("...")]` attribute on each
variant. 2.0.21 is MIT OR Apache-2.0 and was published on 2026-09-23, more than two
weeks before 2026-10-22. It pulls in `thiserror-impl`, `proc-macro2`, `quote` and `syn`
(all MIT OR Apache-2.0) and `unicode-ident` ((MIT OR Apache-2.0) AND Unicode-3.0); every
one of those licences is on the ADR-027 allow list.
`thiserror` gets no facade, an exception to the facade preference of ADR-020: it is a
derive macro, which writes the `Display` and `Error` code into our own types at compile
time. The generator never calls a `thiserror` function, so a facade would have nothing
to wrap, and the generated code only uses the standard library.
Rejected: hand-written `Display` and `Error` implementations (about ten lines of
boilerplate per error enum, and CODING_STANDARDS.md would need to change); `thiserror`
1.x (superseded by 2.x); `snafu` (a larger API built around context selectors, which
this project does not need).
Consequences: a `thiserror` version change is a new ADR line that Jeffrey approves.
Added 2026-10-25 in #94 (Jeffrey, after CodeRabbit's review): `thiserror = "=2.0.21"` in
`citymap-cli`, whose library part (ADR-036) counts as a library crate under
CODING_STANDARDS.md. `YamlError` is a thiserror enum holding the YAML crate's message and,
when reading, its line and column copied into our own `TextPosition`. Rejected: the
hand-written `Display` and `Error` of #94's first commit (the reason above); the message
alone (a program could not use the line and column); keeping `serde_saphyr`'s error inside
(its type would leave the facade, against ADR-034 and ADR-035).
Confirmed 2026-10-27 (#111, Jeffrey): thiserror keeps no facade. The experiment of 2026-10-25,
a scratch workspace outside the repository: thiserror can be routed through a module of our
own only by naming its hidden internal module, which thiserror-impl 2.0.21 builds as
`__private` plus the patch version (`__private21`), so the path changes with every patch
release. Rejected by Jeffrey: that hidden path; dropping thiserror for hand-written `Display`
and `Error` (the code this entry replaced). CODING_STANDARDS.md "Naming" lists the exception.

## ADR-030 Id derivation, kinds and text form (2026-10-22)

Context: issue #6 adds `citymap-core::id::Id`, the stable id every stage output carries
(ARCHITECTURE.md §5). §5 fixes the inputs (kind, parent ids, stable keys) but not how they
are hashed, how a kind is represented, or how a parcel id looks.
Decision (Jeffrey, 2026-10-22):
- `Id` holds an `IdKind` and a `u64` hash. `IdKind` is a closed enum: node, street, block,
  parcel, landmark, district, open space, crossing, road, river. Each kind has a fixed tag
  (1 to 10); the tag, not the enum order, goes into the hash and the ordering, so a new kind
  is appended with the next tag and no existing id changes.
- The hash starts at 0 and absorbs one 64-bit word at a time with `mix64(state ^ word)`
  (the SplitMix64 finaliser, ADR-018): the kind tag; the number of parents; each parent's
  tag and hash; the number of keys; each key as its byte length followed by its UTF-8 bytes
  in 8-byte little-endian words, the last one padded with zeros. The length words keep
  `["ab", "c"]`, `["a", "bc"]` and `["abc"]` apart.
- Callers pass parents and keys in a canonical order (street: sorted endpoint ids); the
  order is part of the id.
- Ids sort by kind tag, then by hash. The text form is `kind:` plus 16 lower-case hex
  digits, for example `landmark:0b3a1c74f76182bc`.
- A parcel id is the same shape as every other id: kind parcel, the block as parent, the
  parcel number as key. This replaces the `block_id:k` form in §5.
Rejected: kinds as free strings (a typo silently makes a new kind; constraint kinds are a
closed enum for the same reason, ADR-017); deriving the ordering from the enum order (inserting a kind would reorder every
golden); concatenating keys without lengths (different keys collide); a general-purpose
hash crate such as `xxhash` or `fnv` (a new dependency for a few lines, and a version change
could alter every id); the readable `block_id:k` parcel form (a second id shape that every
consumer would have to parse).
Consequences: changing the absorption order, a tag or the text form changes every id and
every golden snapshot, and needs a new ADR entry. Serialising an `Id` is decided when stage
output is first written as JSON.
Amended 2026-10-26 (#99, proposed by Claude, confirmed by Jeffrey at stage 6): an `Identifier`
serialises as its text form, `kind:` plus 16 hex digits, the same string `Display` writes.
Rejected by Claude: an object `{kind, value}` (two fields to read where one string is
enough); the bare 64-bit number (loses the kind, and JSON readers round numbers above 2^53).

## ADR-031 Duplicate-id check with a BTreeSet; HashMap and HashSet banned (2026-10-22)

Context: ARCHITECTURE.md §5 says two objects with the same id in one stage output is an
error, checked by a debug assertion and a property test (issue #7). The check needs a set
to remember the ids it has seen. CODING_STANDARDS.md allowed `HashMap` for lookups whose
order never leaks, which leaves every use of a hash collection to the reviewer's judgement.
Decision (Jeffrey, 2026-10-22; the check design on Claude's proposal, the `BTreeSet` and the
ban chosen by Jeffrey, the workspace-wide scope of the ban decided by Jeffrey on Claude's
proposal):
- `id::check_unique(&[Id])` walks the ids once, inserts each into a `BTreeSet`, and returns
  `DuplicateId` naming the first id that is already in the set. The set lives only for one
  call; there is no registry of all ids, because uniqueness holds per stage output and a
  shared registry would be hidden state in a pure core.
- `id::debug_assert_unique(&[Id])` panics with the `DuplicateId` message in debug builds;
  release builds skip it, as §5 says. No stage exists yet; every stage, in the PR that
  builds it, calls it on its output.
- `std::collections::HashMap` and `HashSet` are banned through `clippy::disallowed_types`
  (`clippy.toml`, lint level in the workspace `Cargo.toml`). The ban covers every crate in
  the workspace: the render and export crates write output too, and one shared
  `clippy.toml` keeps a single list of banned methods and types.
Rejected: `HashSet` for this check (constant-time lookup and safe here, because the check
never iterates the set; rejected so that one rule covers the whole workspace and Clippy,
not a reviewer, enforces it; a `BTreeSet` lookup is about 14 comparisons for 10,000 ids);
keeping the per-use judgement of CODING_STANDARDS.md (a misuse that leaks iteration order
breaks determinism and is easy to miss in review);
banning hash collections in `citymap-core` only (a second `clippy.toml` in that crate,
which must repeat the shared banned-methods list, because Clippy reads one file per crate); one registry of every id ever created
(the same landmark id appears in several stage outputs on purpose, and a registry is
shared mutable state, which hard rule 1 forbids); sorting a copy of the ids and comparing
neighbours (it loses which duplicate came first); checking in release builds too (§5 asks
for a debug assertion).
Consequences: a crate that needs a hash collection later needs a new ADR entry and a
scoped `#[expect(clippy::disallowed_types)]` with its reason.

## ADR-032 rand_pcg version, the RngStream facade and stream seeding (2026-10-22)

Context: issue #9 adds `rng_for`, the per-object random stream of ADR-018. ADR-018 chose
`Pcg64Mcg` from `rand_pcg`, pinned to an exact version, but not the version, how the
generator is exposed, or how a 64-bit mixed seed becomes its 128-bit state.
Decision (Jeffrey, 2026-10-22, on Claude's proposal):
- `rand_pcg = "=0.10.2"`, a normal dependency of `citymap-core` with default features. It is
  MIT OR Apache-2.0, was published on 2026-04-11, and pulls in `rand_core` 0.10.1 (MIT OR
  Apache-2.0). `rand_core` is used through `rand_pcg`'s re-export, so it is not a direct
  dependency.
- `rng::rng_for(seed: Seed, stage: &str, id: Id) -> RngStream`. `Seed` wraps the `u64`
  master seed (the newtype rule of CODING_STANDARDS.md). `RngStream` is our facade: it holds
  the `Pcg64Mcg` privately, offers only `next_u64`, and is not `Clone`, so no two users share
  one stream. No other module names `rand_pcg`.
- The stage label (`"placement"`, `"pack:<id>"`) becomes the stage salt through the same
  length-prefixed byte hashing as id keys (ADR-030); those helpers moved from `id` to `rng`.
- The 128-bit state is `narrow << 64 | mix64(narrow)`, where `narrow` is
  `splitmix64(seed, salt, id.value())`; `Pcg64Mcg::new` sets the low bit, as PCG requires.
- The first three draws for seed 1, stage `placement`, landmark `market` are pinned in a
  test and were cross-checked with an independent Python implementation of `Pcg64Mcg`.
Rejected: returning `Pcg64Mcg` directly, as issue #9 first said (breaks the facade rule;
every stage would depend on `rand_pcg`); `SeedableRng::seed_from_u64` (its expansion lives
in `rand_core` and could change in an upgrade, changing every golden); a bare `u64` seed
(the newtype rule); a closed enum of stages (pack streams need a label per pack id);
`rand_pcg` 0.9 (superseded; 0.10.2 is more than two weeks old).
Consequences: a `rand_pcg` version change, or any change to the seeding, changes every
golden snapshot and needs a new ADR entry. `proptest` still brings in `rand_core` 0.9 as a
test-only dependency, so the lock file holds two `rand_core` versions; `cargo deny check`
passes with no warning about it.

## ADR-033 Our own unit-float and bounded-integer draws (2026-10-22)

Context: ADR-018 decided that the two distributions the generator needs are written in
`citymap-core::rng`, so a `rand` upgrade cannot change goldens; issue #10 builds them.
Decision (Jeffrey, 2026-10-22, on Claude's proposal):
- `RngStream::next_unit_f64()`: the top 53 bits of one draw times 2^-53, giving a value from
  0 up to but not including 1. 53 bits is the precision of an `f64`, so every result is
  exact; only a shift and a multiplication are used, which give the same bits on every
  platform (ADR-020).
- `RngStream::next_below(bound: NonZeroU64)`: Lemire's multiply-and-reject method (Lemire
  2019, "Fast Random Integer Generation in an Interval"): the 128-bit product of a draw and
  the bound; its high half is the result unless its low half falls in the uneven remainder
  `2^64 mod bound`, in which case the stream draws again. Every value is equally likely.
  `NonZeroU64` makes a bound of zero impossible to write, so there is no error case.
- No signed range: a signed whole-number range is `next_below(n)` minus an offset, and
  signed decimal ranges are a scaled `next_unit_f64`. Issue #88 holds the idea of adding
  `next_between(low, high)` when a stage needs it (Jeffrey, 2026-10-22).
- Known answers for both methods were cross-checked with an independent Python
  implementation; one test pins a bound where the first three draws are rejected.
Rejected: `rand`'s distributions (ADR-018: their output changed between versions); `x % n`
(small values come up more often when `n` does not divide 2^64); `(next_unit_f64() * n)`
rounded down (the same unevenness, through a float); dividing by 2^64 - 1 or using 64 bits
for the float (rounding makes some values more likely and can produce 1.0); a `u64` bound
with a panic or error on zero (a type that cannot be zero removes the case).
Consequences: changing either method changes every golden snapshot and needs a new ADR
entry. One `next_below` call can use more than one draw of the stream.

## ADR-034 serde-saphyr reads and writes spec YAML (2026-10-24)

Context: spec files are YAML (ARCHITECTURE.md §2). `serde_yaml` is archived upstream, and
milestone 0b needs a YAML crate for `specifications/poc.yaml` (issue #93), earlier than the
milestone 1 that "Open decisions" planned.
Decision (Jeffrey, 2026-10-24, on Claude's proposal): `serde-saphyr = "=2.0.0"`, with
default features (`serialize` and `deserialize`), a normal dependency of `citymap-cli` only,
which owns YAML parsing (ARCHITECTURE.md §2). The generator code calls it only through a
facade module in `citymap-cli` with our own error type (the facade preference of ADR-020).
The dependency is added in #94, the first issue that reads YAML.
- Exception to the two-week rule: 2.0.0 was published 2026-10-13, eleven days before this
  entry. Jeffrey chose it over 1.3.0 so the project starts on 2.x and never migrates from
  1.x; no 2.0.x patch release existed on 2026-10-24.
- Licence: MIT OR Apache-2.0. `cargo deny check` with this repo's `deny.toml` passed for
  a scratch crate using `serde = "=1.0.229"` and `serde-saphyr = "=2.0.0"`: advisories,
  bans, licences and sources all ok.
- Maintenance: releases 1.1.0 to 2.0.0 between 2026-08-15 and 2026-10-13.
- `unsafe`: `serde-saphyr` and its parser `granit-parser` both declare
  `#![forbid(unsafe_code)]` and contain no `unsafe` blocks. Some of their dependencies
  (for example `smallvec`, `simdutf8`, `encoding_rs`) use `unsafe`.
- wasm32: the upstream CI builds for `wasm32-unknown-unknown` and `wasm32-wasip1`. Not built
  in this repo: the wasm32 target is not installed here.
- Scratch test (2026-10-24, not committed): a spec-shaped struct with
  `#[serde(deny_unknown_fields)]` survived spec -> YAML -> spec unchanged; an unknown key,
  a duplicate key and an unknown enum value were each an error naming line and column;
  the value `no` stayed the string `"no"` (YAML 1.2, not a boolean).
Rejected: `serde-saphyr` 1.3.0 (a 1.x -> 2.x migration later); `yaml_serde` 0.10.7 (the
YAML organisation's continuation of `serde_yaml`, but built on `libyaml-rs`, a machine
translation of C with about 240 `unsafe` uses); `serde_yaml_ng` 0.10.0 (no release since
2024-05, MIT only, built on `unsafe-libyaml`, which RustSec lists as unmaintained,
RUSTSEC-2024-0449); `serde_yml` (unsound and unmaintained, RUSTSEC-2025-0068);
`serde_norway` (no release since 2024-12, built on `unsafe-libyaml`); the archived
`serde_yaml`.
Consequences: a version change of `serde-saphyr` is a new line in this entry that Jeffrey
approves.
Added 2026-10-25 in #94: `serde-saphyr = "=2.0.0"` in `citymap-cli`, called only from its
`yaml` facade module (`from_text`, `to_text`, `YamlError`).
ADR-021's scratch test of `#[serde(flatten)]` ran on `serde_yaml_ng`, not on
`serde-saphyr`; ADR-021's rule (no `#[serde(flatten)]` in spec types) stays as written.
Corrected 2026-10-25 (Claude): the wasm32 sentence above is wrong. The target is installed
for the project toolchain 1.99.1 (rust-toolchain.toml), not for the default `stable`
toolchain, which is where the first check ran. A scratch crate with `serde-saphyr = "=2.0.0"`
built for `wasm32-unknown-unknown` with 1.99.1 without errors on 2026-10-25.

## ADR-035 Names are full words (2026-10-25)

Context: issue #94 introduced a type named `Spec`. Jeffrey asked for full words: autocomplete
makes length free, and readable names matter more than short ones (2026-10-24).
Decision (Jeffrey, 2026-10-24 and 2026-10-25; the boundary list proposed by Claude):
- Full words in our own Rust names, document text, file and folder names, and YAML keys.
  This includes the YAML key `id` (becomes `identifier`), the unit suffixes `_m` and `_deg`
  (become `_metres` and `_degrees`) and the folder `docs/` (becomes `documentation/`).
- Stays short: Rust keywords, macros and standard-library names, which cannot be renamed
  (a type alias would leave `u64` in literals, compiler messages and documentation, so the
  code would mix two names for one thing); names of external crates; the format names
  YAML, JSON, SVG and PNG (Jeffrey); process identifiers (ADR numbers, issue and PR numbers,
  `T0` to `T9`, `GM?`); folder and file names a tool requires (`src/`, `.github/`,
  `Cargo.toml`).
- A function implementing a published algorithm gets a name for what it does, and its doc
  comment names the algorithm (Jeffrey): `mix64` becomes `mix_seed`, `splitmix64` becomes
  `stream_seed`.
- External libraries sit behind facades with full-word names (Jeffrey). A library without
  a facade needs Jeffrey's exception in an ADR entry; serde (ADR-028) and thiserror
  (ADR-029) are such exceptions until #111 decides whether they can be wrapped.
- Older ADR entries are not edited: they are history and keep the names of their time.
- The renames are separate PRs: existing code (#112), documents, files and folders (#113),
  YAML keys (#114, a schema change that amends ADR-021). New code uses full words from
  this entry on.
Rejected: Rust's usual short names (`rng`, `id`; common in the ecosystem, but a reader
must know the convention to read them, and autocomplete removes the typing they save);
full words in new code only (two naming styles side by side for months); renaming only
`Spec` (the same question returns with every type); format names in full (Jeffrey: YAML and JSON are their real names); keeping
algorithm names such as `splitmix64` as function names (a proper name says where the code
comes from, not what it does; the doc comment keeps that link); type aliases for Rust's own
names (see above).
Consequences: CODING_STANDARDS.md "Naming" holds the rule and its reviewer test. Until #112,
#113 and #114 are merged, existing short names remain and are not a review finding.
Amended 2026-10-25 (Jeffrey; #120): the rule covers our Python names in `analysis/` too, not
only Rust names. Python's own names (`self`, `_`, standard-library names) and SQLite, a product
name, stay as they are. Rejected: Python outside the rule with an explicit exception (two
naming styles in one repository).
Amended 2026-10-25 (Jeffrey, on Claude's proposal; #113): file paths inside old ADR entries
are updated when a file or folder moves (`docs/` to `documentation/`), because a path points
at a file and is not a name of its time; the rest of the old text stays. Rejected: leaving old
paths as they were (seven links would lead nowhere).

## ADR-036 The command-line crate has a library part (2026-10-25)

Context: the YAML facade of #94 lives in `citymap-cli`, a binary crate. Until `main` reads a
file (#105), only tests call the facade, so the compiler reports it as unused code and
`clippy -D warnings` fails.
Decision (Jeffrey, 2026-10-25, on Claude's proposal): `citymap-cli` has a library target,
`src/lib.rs`, next to its binary `src/main.rs`. The library holds the facades and file
handling (`pub mod yaml` first); the binary calls them. Public items of a library are never
reported as unused, and integration and golden tests in `crates/citymap-cli/tests/`
(ARCHITECTURE.md §8) can only call a library, not a binary.
Rejected: `main` calling the facade now (pulls part of #105 into #94); an `expect(dead_code)`
on the module until #105 (a lint exception at a use site, which CODING_STANDARDS.md
forbids); a separate facade crate (rejected by Claude, not offered: one more crate for
one module).
Consequences: nothing outside `citymap-cli` depends on its library; ARCHITECTURE.md §2's rule
"nothing depends on `cli`" still holds.

## ADR-037 YAML keys in full words: city file and research data (2026-10-25)

Context: ADR-035 makes YAML keys full words. The city-file format (ADR-021) and the
research data (ADR-025) still used `id`, `_m`, `_deg`, `metres_per_km`, `min` and `max`.
Decision (Jeffrey, 2026-10-25, on Claude's proposal; issue #114):
- City file: `id` becomes `identifier`; every `_m` key becomes `_metres` (`extent_metres`,
  `width_metres`, `depth_metres`, `radius_metres`, `height_metres`, `length_metres`,
  `within_metres`, `beyond_metres`, `reuse_within_metres`, `centre_radius_metres`);
  `rotation_deg` becomes `rotation_degrees`; `metres_per_km` becomes
  `metres_per_kilometre`. Rule packs follow: their `id` key becomes `identifier`.
- The schema version stays 1: no city file existed on 2026-10-25 (the vault has no
  `Maps/` folder yet), so no file breaks.
- Research data (Jeffrey chose to include it): `id` becomes `identifier`, `min` becomes
  `minimum`, `max` becomes `maximum`, in every file under `data/research/`; the SQLite
  columns follow (`identifier`, `minimum`, `maximum`, `base_minimum`, `base_maximum`), and so
  do the validator messages. A script renamed 3,240 keys; a comparison of all 30,130 values
  before and after found every value unchanged.
- Never changed (Jeffrey): values that come from sources, such as quotes, notes, place
  names, citations, URLs and locators.
- Added to ADR-035's stays-short list: the SI unit symbols used as unit identifiers (`m`,
  `km`, `m2`, `ha`, `km2`, `people_per_ha`, `people_per_km2`), because they are the official
  symbols (Jeffrey); the key `url`, an acronym like YAML (Claude, in the plan Jeffrey
  approved).
- History keeps the old names: old ADR entries and specification-version-1.md section 6, Jeffrey's
  answers of 2026-10-20 (Claude, applying ADR-035's history rule).
- Python variable names in `analysis/` are not renamed: ADR-035 covers our Rust names,
  documents, file and folder names and YAML keys, not Python code.
Rejected by Jeffrey: SI unit symbols in full (`hectare`, `people_per_hectare`; 1,585 value
changes for names that are already international standards); leaving the research data as
an exception (two key styles in one repository); a separate issue for the research data.
Rejected by Claude in the approved plan: schema version 2 (no version-1 file exists that a
reader would need to tell apart).
Consequences: #95 builds `site.extent_metres` directly. Any query on `output/facts.sqlite`
uses the new column names.

## ADR-038 Two length types: SignedLength and Length (2026-10-25)

Context: issue #83. `Metres` (ADR-028) was signed: it holds positions measured from the site's
south-west corner and steps between points, both of which can be negative. Widths, sizes and
distances are never negative, and the first of them (`width_metres` of a river, #95) arrived.
Decision (Jeffrey, 2026-10-25, on Claude's proposal; the names are Jeffrey's choice):
- Every value in the model is in metres; type names say what a value is, not its unit.
- `SignedLength` (the former `Metres`, renamed): one signed number, for the x and y of points
  and of vectors (steps between points).
- `Length`: zero or more, for widths, sizes and distances. `Length::new` rejects a value
  below zero with `UnitError::Negative` and never flips it, so a mistyped `-18` in a file is
  an error, not a silent 18. Negative zero becomes `0.0`.
- `SignedLength::absolute` gives the size of a signed length as a `Length`; that is where a
  sign is removed on purpose.
- Both share the checks of ADR-020: finite, within `MAXIMUM_ABSOLUTE_METRES`, rounded to
  the centimetre, compared with `total_cmp`, serialised as a plain number.
Rejected: one signed type with a check at every place that reads a size (option a of #83; a
forgotten check goes unnoticed; Jeffrey); `Coordinate` as the name of the signed type (a
coordinate is a position, not a step between points; Jeffrey); `Metres` plus
`PositiveMetres` (two names for one unit, and "positive" excludes zero; Jeffrey); a macro to
share the trait implementations (ADR-021: no macros that hide code from the reviewer; Claude).
Consequences: closes #83. ADR-028's `Metres` is now `SignedLength`; older entries keep the
old name. The geometry facade (#98) builds `Point` from two `SignedLength` values; a `Vector`
type follows when a calculation needs one.
Amended 2026-10-26 (#99, Jeffrey at stage 6): JSON writes a length in the shortest form that
reads back as the same value (`1200.0`, `4.5`), which `serialize_f64` already does; the
centimetre rounding makes it byte-stable. ARCHITECTURE.md §3 and §5 said "fixed decimal
precision" and now match the code. Rejected by Jeffrey: always two decimals (`1200.00`; own
serialisation code for no gain in stability); a follow-up issue (leaves the documents wrong).

## ADR-039 S0 river courses: straight lines between edge stretches (2026-10-26)

Context: #99 builds the first stage. A river names its ends by compass region
(`from`, `to`); the issue left open where on an edge stretch a course starts.
Decision (Jeffrey, batch 1 of milestone 0b): the site's edge is split into eight stretches of
equal length, one per compass direction and centred on it: north is the middle half of the
north side, north-east the last quarter of the north side plus the first quarter of the east
side, and so on round the site. A river's course is a straight line from the middle of its
`from` stretch to the middle of its `to` stretch: the side's middle for north, east, south and
west, the corner for the diagonals. The river's identifier is `Identifier::new(River, [],
[identifier])`. `site::run` calls `identifier::debug_assert_unique` on its output.
Confirmed by Jeffrey at stage 6: `Centre` maps to the site's middle and needs no case of its
own; a river that rises on the site, as at Wardat Sha's spring, is a future idea (#130).
`site::run` takes only the specification and returns `SiteOutput` without `Result`, because
S0 in milestone 0b draws no random number and cannot fail; the `Ctx` and `StageError` of
ARCHITECTURE.md §4 arrive with the first stage that needs them.
Rejected in the batch Jeffrey approved, reason by Claude: a seeded point within the stretch
(randomness the first picture does not need). Rejected by Jeffrey at stage 6: the full §4
form now (an empty `Ctx`, a `StageError` and a `Result` that do nothing yet); a debug panic on
`Centre` until #97.
Consequences: closes #99; #104 draws these courses. Moving a course off the stretch middle
later changes every snapshot that holds a river.

## ADR-040 Seven stages per issue; CodeRabbit on request (2026-10-26)

Context: Jeffrey set a fixed workflow per issue and called CodeRabbit a nice-to-have, not a
requirement.
Decision (Jeffrey): every issue goes through seven stages: 1. Claude picks up the issue and
discusses it with Jeffrey; 2. Claude writes the code; 3. Claude reviews it, and one agent
started without context reviews it; 4. Claude cuts as much code as possible while keeping
the core functionality; 5. Claude reviews again; 6. Claude and Jeffrey review it together;
7. Claude opens the PR. `.coderabbit.yaml` sets `reviews.auto_review.enabled: false`;
CodeRabbit reviews a PR only when Jeffrey comments `@coderabbitai review`, with the review
instructions in that file. Claude runs the local CodeRabbit command line when time and quota
allow, and never waits for it (Jeffrey, 2026-10-25).
Proposed by Claude, confirmed by Jeffrey at stage 6 (2026-10-26): stage 1 ends with the ten-line
design and Jeffrey's yes; Claude starts the stage 3 agent and gives it the diff and the
documents, never Claude's conclusions; stage 6 happens in the conversation and Jeffrey
decides on the cuts there; "stage", because the glossary uses "step" for one PR; the file
cannot limit who triggers a review, so Claude never posts `@coderabbitai review`.
Rejected by Jeffrey: CodeRabbit as a required review on every PR (the rounds of ADR-024);
removing CodeRabbit completely; keeping its automatic review on every new PR as optional.
Consequences: replaces ADR-024. ADR-022 stays: any review comment, from CodeRabbit or a
person, is fixed or denied and resolved before a merge. Jeffrey's PR review and merge
(ADR-014) stay after stage 6.

## ADR-041 Squash merges and rebased branches (2026-10-26)

Context: Jeffrey wants a history that reads back clearly, and every change on `main` whole:
a feature is in or it is not, never half. Merge commits put every commit of a PR on `main`.
Decision (Jeffrey): the ruleset on `main` and the repository settings allow only squash
merges. The squash commit takes the PR title as its subject and the PR body as its message.
Claude updates a branch by rebasing it on `main` and pushing with `--force-with-lease`,
never by merging `main` into it. A file rename (`git mv`) is its own PR, because a squash
joins it with content changes and git then loses the file's history (#121 showed this).
Rejected by Jeffrey, in the plan Claude proposed: merge commits (the setting until now; half
states on `main`); rebase merges (every commit of a PR lands on `main` one by one, the same
half states); squashing and accepting the lost history of renamed files.
Consequences: idioms are explained in the PR body instead of the commit body. Snapshot
changes stay a separate commit inside a PR for review, and are squashed on `main`. The
`tier-N` tags (ADR-015) go on squash commits. Applied 2026-10-26 by Claude through `gh`.

## ADR-042 The SVG renderer reads a CityModel, through an svg facade (2026-10-26)

Context: #104 builds `citymap-render`, the first renderer. ARCHITECTURE.md §7 says it takes
a `CityModel` and a `Style`; no `CityModel` existed, and the site's size lived only in the
specification.
Decision (Jeffrey, stage 1 of #104 to #106):
- `citymap-core::model::CityModel { site }` now, built by `model::generate`, which runs every
  stage. `SiteOutput` gains `extent_metres`, so the renderer reads only the model. `Style`
  (colours and the site's outline width, with documented defaults) stays outside the model.
  No empty fields for later stages; each arrives with its stage (Claude, in the approved plan).
- `render_svg(&CityModel, &Style) -> String` draws a `terrain` group with the site and a
  `water` group with one polyline per river, as wide as the river, `id` = its identifier.
- Doc comments (Jeffrey at stage 6): three lines is the target for a doc comment, and a
  longer one is allowed when shortening it would lose information; Claude names each in the
  PR body and Jeffrey decides in review. A facade's `# <Crate> functions used` list is always
  allowed; `geometry.rs` (#126) already had one. Rejected by Jeffrey: shortening the facade
  comments to three lines with the lists moved into ADRs (the ADRs would have to follow every
  facade change); removing the lists (the `use` lines then are the only record).
- The site's outline is a `frame` group drawn last, so rivers and later roads run under it
  and off the map (Jeffrey at stage 6, after looking at the first PNG). F13 lists no frame
  layer; it is added here. Rejected by Jeffrey: the outline in the `terrain` group, under
  the rivers.
- SVG is written through the facade module `svg_document` over `svg = "=0.18.0"`
  (2024-09-27, MIT OR Apache-2.0, no dependencies of its own). The crate writes `f64` with
  `to_string`, so the facade writes every number itself with two decimals. The crate sorts
  attributes by name and escapes their values, so the text is byte-stable.
- The y axis is flipped in Rust, `y = extent - y`, as each point is written.
- Golden tests live in the lowest crate that can build their input: the SVG test in
  `citymap-render/tests/`, the S0 test (it reads YAML) in `citymap-command-line/tests/`. SVG
  snapshots are allowed for the renderer's test models too. ARCHITECTURE.md §8 is amended.
- The site's outline is drawn inside the site: the rectangle is inset by half the outline's
  width, so no part of the stroke falls outside the view and is cut off (Claude).
Rejected by Jeffrey: taking `&Site` from the specification plus `&SiteOutput` until S1
(departs from §7 for less code); only `&SiteOutput` with the extent, no `CityModel` yet;
writing SVG by hand behind the same facade (escaping would be ours to build by milestone 7);
by hand without a facade; an SVG `transform="scale(1,-1)"` for the y axis (text drawn later
would need a second flip); the golden test in `citymap-command-line/tests/` with a render
dev-dependency; no SVG golden test until #105.
Consequences: the S0 snapshot gains `extent_metres`; Jeffrey accepts it again. Swapping the
`svg` crate for another writer touches only `svg_document.rs`.

## ADR-043 The generate command: clap behind a facade, our own error type (2026-10-26)

Context: #105 makes `citymap generate` work end to end, the first code that reads a file
and writes a map. REQUIREMENTS.md F17 names the arguments and F20 the exit codes.
Decision (Jeffrey, 2026-10-26, stage 1 of #104 to #106):
- `clap = "=4.6.7"` (2026-09-14, MIT OR Apache-2.0) with its builder API, called only in the
  facade module `arguments`, which returns our own `Request` and `GenerateArguments`. Default
  features stay on (Claude): coloured help and "did you mean" suggestions pull in `anstream`,
  `anstyle*`, `colorchoice`, `strsim`, `utf8parse` and `windows-sys`, all MIT or Apache-2.0;
  `cargo deny check` passes. Rejected by Claude: default features off (plainer messages for
  a handful of small crates).
- `CommandLineError` (thiserror, ADR-029) with one variant per kind of failure and
  `exit_code()`: 2 for invalid arguments or an invalid specification, 1 for reading or
  writing. `main` prints `error:` and the message and exits with that code.
Settled by the documents, not asked: `--seed` is optional and overrides the specification's
seed for one run, and the seed is in every output file name (F17); the specification file
is `specifications/example.yaml` (ADR-016), and §9's done-when command now names it.
Added by Claude in the approved plan: the file is named
`<specification file name>-seed-<seed>.svg` (rejected: the city's `identity.name`, which can
hold spaces and characters a file name cannot); `--output` defaults to `output`, the
git-ignored folder (rejected: a required `--output`; the current folder); help text is
printed with exit code 0 as `Request::Show` (rejected: letting clap exit the process itself,
which skips our exit-code mapping). The argument facade strips clap's own `error: ` prefix,
because `main` adds it to every error.
Rejected by Jeffrey: clap's derive API (its attributes would sit on our types, outside the
facade); our own parsing of `std::env::args` (help and error text to build ourselves);
`anyhow` (a test cannot tell which error happened without reading message text).
Consequences: F1's `serde_path_to_error` messages wait for validation (#97); until then a bad
specification reports the YAML reader's message with line and column. `--png` arrives with
#106. S0 draws no random number yet, so a different seed gives the same map; the seed
reaches the model unused until a stage draws from it.
Amended CODING_STANDARDS.md "Rust conventions": thiserror in every crate; its old line
allowing `anyhow` in `citymap-command-line` contradicted this entry.

## ADR-044 PNG through resvg behind the raster facade (2026-10-26)

Context: #106 makes the first picture: `generate --png` writes a PNG next to the SVG.
ARCHITECTURE.md §7 names `resvg`; ADR-026 put it behind a render facade at milestone 0b.
Decision (Jeffrey, 2026-10-26, stage 1 of #104 to #106):
- `resvg = { version = "=0.48.1", default-features = false }` (2026-08-02, Apache-2.0 OR
  MIT), called only in the facade module `raster` of `citymap-render`, through
  `png_from_svg(svg, width_pixels)`; `RasterError` holds resvg's messages as our own text.
  Without default features there is no text, no system fonts and no raster images, so no
  file on the computer can change the picture. `resvg` brings `tiny-skia` 0.12.0 and `usvg`;
  `cargo deny check` passes, with one warning: `miniz_oxide` in versions 0.8.9 and 0.9.1,
  which `deny.toml` allows as a warning. `tiny-skia` computes in `f32`, so `f32` is allowed
  inside `raster` only (Claude).
- A PNG is written only with `--png` (F17); §9's done-when command now has the flag.
- The PNG is `Style::width_pixels` wide, default 2048; the height keeps the map's shape.
  `--dpi`, `--scale` and paper sizes come at milestone 7, with a check that the city fits
  the paper at the chosen scale.
- `render_png(&CityModel, &Style)` renders the SVG and rasterises it. A rendering failure is
  `CommandLineError::Rendering`, exit code 1 (an internal error under F20).
Settled by the two-week rule of ADR-028, applied by Claude, not asked: 0.48.1, because 0.49.0
was published 2026-10-24.
Rejected by Jeffrey: always writing a PNG in 0b and amending F17; pixels per metre (a large
site gives a huge file); `--dpi` and `--scale` now.
Consequences: a PNG is never snapshot-tested (§8); Jeffrey looks at it in `output/`.
`generate` renders every file before it writes any, and returns the paths it wrote. Once
`width_pixels` can be set from outside (milestone 7), validation rejects zero and a size the
computer cannot hold, with exit code 2 (F20); today it is fixed in code.

## ADR-045 Road types in the specification (2026-10-26)

Context: #96, the first issue of the roads batch (#96, #97, #100, #103, #101, #102, built
as one stack in that order, Jeffrey 2026-10-26). specification-version-1 §2 defines
`roads[]` as `{identifier, ends, importance, river_crossing}`, at least one road.
Decision (Jeffrey, stage 1 of the roads batch):
- `Specification.roads: Vec<Road>`, required. A `Road` has `identifier`, `ends: [RoadEnd; 2]`
  (the first end is where crossing numbers start), `importance` and `river_crossing`
  (default `bridge`).
- `RoadEnd` is written as one word and read as a compass region, `core`, or otherwise
  `Place(identifier)` for a pinned landmark or open space. Milestone 0b has no landmarks, so
  validation (#97) rejects a `Place` end, naming the road.
- `Importance` is `minor` or `major`, ordered so a major road outranks a minor one.
- `specifications/example.yaml` gets three roads: `north_road` (major, north to south),
  `west_road` (minor, west to the core) and `market_road` (major, south-west to north-east;
  name by Claude). The third adds a road-road candidate for the core.
Settled by the documents, not asked: the fields, `bridge` as the default and at least one
road (specification-version-1 §2); the key `identifier` (ADR-037).
Added by Claude in the approved plan: `RoadEnd` reads through `From<String>` with
`CompassRegion::ALL` and `CompassRegion::name`; a unit test keeps `name` equal to what serde
writes (rejected: serde's untagged enum, which needs a separate one-value type for `core`).
Rejected by Jeffrey: no `Place` case, so the YAML reader refuses a landmark end (its message
cannot name the road); a hidden raw struct converted with `TryFrom` (code milestone 1
replaces); only the two roads of specification-version-1, or only `north_road`, in the example.
Consequences: every specification file and test needs `roads`. S0 ignores roads until #100.

## ADR-046 Validation of the milestone 0b fields, in the core (2026-10-26)

Context: #97. The types reject unknown keys and wrong shapes; specification-version-1 §5
lists the rules between fields that the types cannot hold.
Decision (Jeffrey, stage 1 of the roads batch): `citymap-core::validation::validate` returns
every broken rule as a list of `ValidationError` (thiserror), each message starting with the
field it is about; one problem gives one error (a name used three times, or two equal place
ends, is reported once). Every rule rather than the first is settled by issue #97 (Jeffrey).
The limits are named constants (`NAME_CHARACTERS`, `POPULATION`, `EXTENT_METRES`) that the
messages print. `generate` validates after reading and before
S0; broken rules are `CommandLineError::BrokenRules`, exit code 2 (F20): a line naming the
file and the count, then one rule per line.
Rules checked in 0b: schema version 1; a name of 1 to 80 characters; population 1 to
1,000,000; extent 500 to 5000 m; an identifier used once across rivers and roads, never
`core`, `centre`, a compass region or containing `.`; a river wider than 0 m, not ending at
`centre`, with `from` and `to` different (Jeffrey, #99 stage 6); at least one road; a road
with two different ends, at most one `core`, no `centre` end and no landmark or open-space
end, naming the road (ADR-045).
Rejected by Jeffrey: validation in `citymap-command-line` (a UI would have to repeat it);
stopping at the first broken rule (issue #97).
`centre` as a river end stays possible later for a spring or a lake (#130).
Consequences: S0 can rely on these rules. The population warning (1,000 to 200,000) and the
`serde_path_to_error` field paths of F1 are not in 0b (#138); reading errors keep the YAML
reader's line and column.

## ADR-047 S0 roads, step one: straight between edge stretches (2026-10-26)

Context: #100, step one of the road layout of specification-version-1 §2: every road
without a `core` end.
Decision (Jeffrey, stage 1 of the roads batch):
- `SiteOutput.roads: Vec<RoadCourse { identifier, course, importance }>`, in specification
  order. A road with two compass-region ends runs straight from the middle of its first
  end's stretch to the middle of its second's, as rivers do (ADR-039); no random number.
  Roads with a `core` end wait for step three (#102); a `Place` end never reaches S0,
  because validation rejects it (ADR-046).
- A road identifier is `Identifier::new(Road, [], [identifier])`; `debug_assert_unique`
  runs over rivers and roads together.
- A road that lies on a river, or runs along the frame, is accepted in 0b; roads beside
  rivers (#136) and curved courses (#131) solve it later.
- The renderer draws a `roads` group after `water` and before `frame`; a road's width comes
  from its importance (`Style::major_road_width` 10 m, `minor_road_width` 5 m, colour
  `#8a6f4e`; values by Claude). F13 lists the new S0 groups `roads`, `crossings` and `core`.
Rejected by Jeffrey: a seeded point within each stretch (needs a stream key §4 does not
name); a bent course with a seeded midpoint; refusing a road on a river or along the frame
in validation; a generation error for it (needs `Result` in S0); one `streets` group shared
with S3 later; one `site` group for everything (no switching roads off).
Consequences: the S0 and SVG snapshots gain roads; Jeffrey accepts them.

## ADR-048 S0 road-river crossings (2026-10-26)

Context: #103. specification-version-1 §2: where a road crosses a river, S0 creates a
crossing of the road's `river_crossing` kind, named `<road>.<river>`, or `<road>.<river>.<n>`
numbered from 1 along the road from its first listed end when it crosses that river again.
Decision (Jeffrey, stage 1 of the roads batch):
- `SiteOutput.crossings: Vec<RiverCrossing>` with `identifier`, `name`, `road`, `river`,
  `kind`, `position`, `span` and `width`; by road, then river, in specification order, and
  along the road. The identifier is `Identifier::new(Crossing, [], [name])`, the derivation a
  crossing listed in a specification will get, so the two match and can be pinned.
- Only a real crossing counts. Touching a river at one of the road's own two ends is no
  crossing: `west_road` ends on the core, which already lies on a bridge. A road lying on the
  same straight line as a river, with no offset, is no crossing either (Jeffrey: in the
  future no such overlap exists, except with height levels; #136).
- The geometry facade gains `crossings_along(road, river)`, ordered along the road and
  without touches at the ends of either line or overlaps, and `span_through(segment, centre,
  length)`; both use our own `+ - * / sqrt` next to `geo`'s segment intersection. It replaces
  `segment_intersections` of ADR-020's facade, which nothing used any more (Claude, review).
  A river that starts on a road is a touch, not a crossing (Claude, review). With straight
  courses every crossing is a single proper point; touches at an inner vertex of a bent line,
  and the ends of a bent overlap, need a side test that comes with curved courses (#131).
- A crossing's footprint (specification-version-1 §2, crossings row): `CROSSING_WIDTH_METRES`
  8 m wide; its `span` runs along the road, as long as the river is wide plus
  `BRIDGE_BANK_METRES` 5 m on each bank for a bridge, no banks for a ford (Claude). These are
  the provisional default-pack values of ADR-021, kept as named constants until the default
  rule pack exists (Jeffrey at stage 6; rejected: a `SiteSettings` passed into S0 now, and
  overrides in the specification file). Jeffrey: almost every real bridge differs in length,
  width, build and style, so per-bridge values follow later (#140). The span is measured
  along the road, so at a slanted crossing it is shorter than the road's way across the
  water; that stays in 0b and is designed with #140 (Jeffrey at stage 6; rejected: river
  width divided by the sine of the angle, which needs `libm` and a cap; the road's own
  entry and exit points on both river banks).
- The renderer draws a `crossings` group after `roads`: each span 8 m wide, in
  `Style::bridge_colour` (`#5b4a3a`) or `ford_colour` (`#c9b38a`) (values by Claude).
Rejected by Jeffrey: every touch a crossing (two bridges at the core); skipping only touches
at the core; two numbered crossings, or one, for an overlap; the road and river as parent
identifiers with the number as key (a listed crossing would get another identifier).
Consequences: straight courses cross at most once, so the `.<n>` numbering is tested on a
hand-built zig-zag course. Roads to the core get their crossings in #102.

## ADR-049 The town core of S0 (2026-10-26)

Context: #101. specification-version-1 §4 chooses the town core after the roads of step one
and before the roads that end at it. #103 already finds the road-river crossings.
Decision (Jeffrey, stage 1 of the roads batch):
- `citymap-core::town_core::TownCore { identifier, position, rule, notice }` in
  `SiteOutput.core`. The core has a new identifier kind, `Core` (tag 11, appended as ADR-030
  allows), so it works like every other object: SVG `id`, uniqueness, later S3 links.
- Candidates are the road-river crossings of #103 and the points where two step-one roads
  really cross inside the site; roads that only share an end, or touch at one, give none.
  Ranking, from specification-version-1 §4: the candidate whose more important road ranks
  highest; among those, a river crossing before a road crossing; then the one nearest the
  site centre; an exact tie goes to the alphabetically first name. A river crossing is named
  as in ADR-048, a road crossing by its two roads in alphabetical order joined by `+`
  (`market_road+north_road`; the `+` by Claude).
- Distances are compared as whole square centimetres (`geometry::squared_centimetres`), so
  the order is exact.
- No candidate: the point nearest the site centre on a road of the highest importance, by
  `geo`'s `ClosestPoint` behind the facade. No step-one road: the river point nearest the
  centre over all rivers, a tie to the alphabetically first river (#137 researches a
  biggest-river rule); no river either: the site centre.
- The notice travels as data: `TownCore.notice` names the chosen river crossing and the best
  road crossing of equally important roads that it beat. `generate` returns it and the
  program prints `notice: …` on stderr. S0 stays pure, without a `Result` or warning list.
- The renderer draws a `core` group on top of the crossings: an open ring, `Style::core_size`
  (40 m) across, line `core_line_width` (4 m), in `core_colour` (`#2e8b57`, green), so the
  bridge or junction under the core stays visible (Jeffrey at stage 6, after the first PNG
  with a core). Rejected by Jeffrey: the filled red square of the first draft (it hid the
  bridge); a smaller filled square; leaving it to labels at milestone 7.
- In `specifications/example.yaml` the core lands on `market_road.ash` at (1600, 1600), with
  the notice.
Rejected by Jeffrey: no identifier for the core (the one exception); a `Vec<Warning>` next to
the model now; postponing the notice; the hashed identifier order for ties; counting every
meeting point of two roads; our own closest-point arithmetic; the first or the widest river
for the river rule (the widest waits for #137).
Added by Jeffrey at stage 6 (2026-10-27): the core never floats on a bare position.
`SiteOutput.junctions` registers every place where two roads really cross as a
`RoadJunction` (identifier kind `Junction`, tag 12; name as above, `.<n>` when two roads
cross again; found like river crossings), and `TownCore.on` names what the core lies on: the
river crossing or junction it was chosen at, the road of the one-road rule, or the river of
the river rule. Only the site centre has nothing to lie on; #139 puts a landmark before it.
Junctions are not drawn; they show as crossing roads. Rejected by Jeffrey: `on` without
registering junctions (the two roads instead of one object); keeping the position alone.
Added by Claude after review: no notice when the passed-over road crossing lies on the
chosen point itself, because the river rule then changed nothing (rejected: a notice anyway);
distances use `SignedLength::steps`, the whole centimetres of the length type, instead of
rounding in the facade. Issue #101 asks that renaming never moves the core except on exact
ties; the property test checks renaming that keeps the alphabetical order, which never moves
it at all (rejected: detecting exact ties inside the test, which repeats the ranking).
Consequences: Jeffrey (2026-10-26): a tie decided by name gives a notice later too; the data
form carries it without a new structure. `generate` returns `Generated { files, notices }`.

## ADR-050 S0 roads, step three: straight to the core (2026-10-26)

Context: #102, step three of the road layout of specification-version-1 §2: every road
with a `core` end, laid out after the core is chosen (ADR-049).
Decision (Jeffrey, stage 1 of the roads batch): a road with one compass-region end and one
`core` end runs straight from the middle of that end's stretch to the core, in the order of
its `ends` (so crossing numbers start at the first listed end). `SiteOutput.roads` then holds
every road in specification order. Crossings are found again over all roads, so a road to the
core gets its river crossings through #103's function; the core itself is chosen from the
step-one roads and their crossings only (specification-version-1 §4).
Rejected by Jeffrey: an L-shaped course (a shape curved courses, #131 and #136, replace).
Rejected by Claude: appending the core roads' crossings to the step-one list and sorting it
again (two places that know the order instead of one).
A road to the core can have length zero when the core lies on that road's own edge stretch
(the one-road rule on an edge road). `SiteOutput.zero_length_roads` lists such roads and
`generate` prints a notice for each; the map keeps them (Jeffrey at stage 6: any road of
length zero is odd). Rejected by Jeffrey: accepting it silently with only a test; a
generation error (needs `Result` in S0, ADR-039).
Consequences: in the example `west_road` ends on the core, which lies on `market_road`'s
bridge over the Ash; touching the river at its own end, it makes no second crossing (ADR-048).

## ADR-051 A documents check in CI: hedge words and rejected alternatives (2026-10-27)

Context: #117. Reviews kept finding the same two things in documents: hedged wording and
rules or decisions without a decider or rejected alternatives (#99 stage 6, ADR-039).
Decision (Jeffrey, 2026-10-25 and the housekeeping batch of 2026-10-27):
- `analysis/check_documents.py`, a Python script with unit tests in `analysis/tests` (Python
  stays under `analysis/`, CLAUDE.md), run by CI next to `validate_facts.py`. It fails when a
  line a change adds to `CLAUDE.md` or `documentation/` uses a hedge word, and when an
  `## ADR-` section has no `Rejected` line. CI checks out the full history and compares
  against the PR's base commit (or the previous commit on `main`).
- The list of hedge words lives only in the script; `.coderabbit.yaml` loses its copy,
  because CodeRabbit is optional (ADR-040). The list: "usually", "should consider", "where
  appropriate", "if needed", "maybe", "perhaps", "roughly", "a few", "wherever possible",
  "as much as possible", "if possible".
- Only added lines are checked, so old text that mentions a word is not flagged; a word in
  straight or typographic quotes or backticks is mentioned, not used, and consecutive added
  lines are read together so a phrase split by a line break is found (Claude). Research
  reports are exempt (ADR-023), except the cross-analysis, which ADR-023 holds to the rule.
  A first push without a base compares against the previous commit (Claude, review).
  Rejected by Claude: fetching only the base commit instead of the full history (one more
  step that can fail, for a small repository).
- Every ADR is checked, the founding ones too: ADR-001, 002 and 004 to 012 got a dated
  `Rejected` line naming only an alternative their own text names, otherwise "none
  recorded"; nothing was invented.
- CODING_STANDARDS.md "Review hygiene" gains the checklist item for Claude.
Rejected by Jeffrey: the list in `.coderabbit.yaml` read by CI; a separate list file with a
second copy for CodeRabbit; the `Rejected` check for ADR-013 onwards only; dictating the
eleven founding alternatives; a Rust test that calls `git`; `grep` lines in the workflow.
Consequences: a PR that adds a hedge word, or an ADR without a rejected alternative, fails CI
on both platforms.

## ADR-052 Milestones 1 to 7 in two passes: thin, then deep (2026-10-28)

Context: every milestone is a first iteration, and what a later stage needs changes earlier
ones. The research of 2026-10-27 (`city-generation-cross-check-2026-10.md`,
`city-generation-gaps-2026-10.md`) already shows three cases: labels stay local only with
landmark priorities (S2 data), parcels stay local only with seeds from stable identifiers (S1
to S3 identifiers), and traced streets need a graph clean-up (S3 data). Built to tier quality
one milestone at a time, those surface at milestones 6 and 7, after earlier stages and their
snapshots were polished. Milestone 0b showed that a thin slice through the stages works.
Decision (Jeffrey, 2026-10-28, on Claude's proposal):
- Pass 1 runs milestones 1 to 7 in order, thin. A milestone is done in pass 1 when its stage
  produces output with the simplest algorithm that keeps every hard rule, with unit, property
  and golden tests (determinism, stable identifiers, pins, locality), and a PNG of Truro that
  Jeffrey looked at. Pass 1 has no tier reviews: Truro may look wrong; a broken hard rule is a
  failure.
- Pass 2 runs milestones 2 to 7 again, in order, to their "Done when" column, each with its
  tier review. Milestone 1 has no tier and is done once, in pass 1. Milestones 7b to 10 follow
  pass 2 and are built once.
- The pass 1 target per milestone is the "Pass 1 done when" column of ARCHITECTURE.md §9.
- Each pass 1 milestone ends with one "deepen" issue listing what was postponed; the research
  issues #157 to #171 are attached to the deepen issue of their milestone.
- Open decisions (parcel method, label placement, road cost) are settled in pass 2. Pass 1 uses
  a placeholder method, which is not an ADR. The font decision is the exception: it is settled
  in pass 1, milestone 7, so the legend is built with the final font crate and fonts.
  Rejected by Jeffrey: a placeholder font and the simplest text measurement in pass 1.
- Snapshot changes in both passes are accepted by Jeffrey with `cargo insta review`.
- GitHub milestones: one per milestone and pass for milestones 2 to 7.
Rejected by Jeffrey: one pass, each milestone to full depth with its tier review (the plan
before this entry; late discoveries rework polished code and snapshots); a thin pass through
milestones 3 to 6 only, then one deep pass from milestone 1 (leaves the label stage, where the
research found a locality conflict, untried until the deep pass).
Consequences: ARCHITECTURE.md §9 gains the "Pass 1 done when" column, and "one or two
sessions each" holds for pass 2 only; the glossary gains "pass"; CODING_STANDARDS.md names the
GitHub milestones and branches per pass (Claude, naming). The open decisions below are settled
in pass 2, except the font decision (pass 1, milestone 7); REQUIREMENTS.md §7 gets a harness
skeleton in pass 1. Each stage is touched twice and its snapshots change in both passes.

## ADR-053 Batch designs, parallel branches and side lanes (2026-10-28)

Context: in milestone 0b, waiting between PRs (one question, one yes and one review round per
issue) was the bottleneck. On 2026-10-25 Jeffrey agreed to a way of working for 0b that was
never written down here; on 2026-10-28 he carried it over to pass 1 (ADR-052) and added side
lanes.
Decision (Jeffrey, 2026-10-25 for 0b, extended on 2026-10-28 to every milestone from 1 on):
- Batch designs: Claude gives the ten-line designs of three or four consecutive issues of one
  milestone at once, with the questions one at a time; Jeffrey's yes covers the batch, and
  Claude then builds the issues one after another without a new yes for each. Each issue
  still goes through the seven stages of ADR-040.
- Parallel branches: issues of the current milestone that do not depend on each other get
  their own branches at the same time. Claude assigns ADR numbers up front so DECISIONS.md
  does not conflict.
- Follow-ups are folded into the issue they belong to instead of becoming their own PR.
- Side lanes: next to the current milestone, Claude works on issues that change no stage
  output, on their own branches: infrastructure and documentation (for example #160, #168,
  #169, #170) and research upkeep (for example #61, #74, #68 to #71). Issues tied to a
  milestone's topic (#157 to #159, #161 to #167) wait for that milestone's deepen issue
  (ADR-052).
- Jeffrey still reviews and merges every PR (ADR-014).
Rejected by Jeffrey: parallel branches inside the current milestone without side lanes; one
issue at a time, strictly in order.
Consequences: CLAUDE.md "Pace" no longer says one step per session; CODING_STANDARDS.md
"Review hygiene" names batches and lanes.

## Open decisions

- Font metrics crate (`ttf-parser` versus `fontdue`) and the bundled fonts: settled at
  milestone 7, pass 1 (ADR-052); Jeffrey decides.
- Parcel subdivision method (straight skeleton versus recursive split): settled at
  milestone 6, pass 2, by trying both on Truro; Jeffrey decides at tier T5.
- Label placement algorithm (greedy eight-position with R-tree, with or without simulated
  annealing): settled at milestone 7, pass 2; Jeffrey decides at tier T6.
- Whether `sweep` metrics are compared against real town plans by hand or with a scripted
  corpus: Jeffrey decides at milestone 5, pass 2.
- Cost function for least-cost roads (#159): settled at milestone 5, pass 2; Jeffrey decides.
