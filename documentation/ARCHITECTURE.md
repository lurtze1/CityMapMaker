# Architecture — CityMapMaker

Last reviewed: 2026-10-19. Nothing is built yet; this is the target shape. Change it when a
decision changes, and log the change in `DECISIONS.md`.

## 0. Glossary

| Term | Definition |
|---|---|
| step | one reviewed series of commits on one branch, ending in a merged PR with green CI |
| milestone | a numbered row in §9; one or more steps; done when its "done when" column holds |
| tier | a review checkpoint in REQUIREMENTS.md §9; passed only by Jeffrey answering yes to every judge question |
| stage | one pure pipeline function S0 to S7 in §4 |
| cell | one square of the coarse grid laid over the site in S1; the unit of locality and of RNG streams |
| ward | one polygon of the ward mesh in S1, one per cell site; the unit the macro stage assigns a district type to |
| district | a named area from the spec; made of one or more wards |
| block | a face of the street graph in S4 |
| parcel | a subdivision of a block in S4; holds at most one building |
| preset | a named bundle of tuning numbers: an era preset (area per person, street widths) or a style preset (colours, strokes) |
| rule | one entry in a rule pack: an evaluator name, hard or soft, a weight, parameters |
| evaluator | compiled code that scores a candidate placement for one rule name |
| constraint | a rule attached to one spec item (landmark, district, open space, crossing) |
| pack | a YAML file of rules and attribute stages, loaded by name from the spec |
| id | the stable name tag of one object in a stage output: a kind plus a 64-bit hash (§5, ADR-030) |
| kind | which sort of object an id names (node, street, block, parcel, landmark, ...); a closed list in `citymap-core::identifier` |
| parent | an object whose id goes into the hash of another object's id, such as the block of a parcel; the id itself does not store it |

## 1. Principles

1. **The spec is the state.** Everything the user has said about a city lives in one spec
   document. There is no hidden state in the generator and no state in a UI.
2. **Generation is a pure function.** `generate(spec, seed) -> CityModel`. No I/O, no clock,
   no ambient randomness. Same inputs give byte-identical model JSON and SVG on the same
   machine and toolchain (ADR-020); identical output across platforms is a later goal
   (#67). PNG is not a determinism artefact.
3. **Edits are commands on the spec.** "Place a gate here" from a future UI, `--write-back`
   from the CLI, and an AI editing YAML all produce the same thing: a new spec. The generator
   never learns where an edit came from.
4. **Stages, not a monolith.** Each pipeline stage is a pure function from upstream outputs to
   its own output, with its own RNG streams and stable ids. Stages can be tested, replaced and
   re-run alone.
5. **Rules are data, evaluators are code.** World rules are JSON or YAML rule packs that
   reference evaluator types registered in code. Core ships generic evaluators; a setting pack
   adds its own evaluators in its own crate.
6. **Render reads, never writes.** The renderer consumes a `CityModel` and a style; it cannot
   change geometry. Label placement is a render concern and is deterministic too.

## 2. Workspace layout

```
CityMapMaker/
  Cargo.toml                 workspace
  crates/                    (created at milestone)
    citymap-core/            0   pure: specification, model, identifier, random, geometry, stages,
                                 built-in evaluators in core::rules
    citymap-command-line/    0   clap binary: generate, validate, sweep; owns YAML parsing and I/O;
                                 its library part (src/lib.rs) holds the facades, such as yaml (ADR-036)
    citymap-render/          0b  CityModel + Style -> SVG string; labels, legend; PNG via resvg (ADR-026)
    citymap-metrics/         5   expressive-range metrics over a CityModel, used by sweep
    citymap-commands/        4   spec mutations; first used by --write-back
    citymap-export-foundry/  7b  Scene JSON, later Universal VTT
    citymap-rules-pnw/       8   Peace and War pack evaluators; first external pack
  specifications/                     example.yaml only; real-city specs live in the vault (ADR-016)
  packs/                     rule pack data files
  styles/                    style presets (ink, ancient, ...) and bundled OFL fonts
  crates/citymap-command-line/tests/  integration and golden tests; a root tests/ dir would not run in a virtual workspace
  analysis/                  Python 3.12 notebooks, pinned by pyproject.toml; not in the build
  documentation/
  output/                       generated output, git-ignored
```

`citymap-command-line` is allowed to run `sweep` seeds in parallel threads; every crate below it is
single-threaded.

Dependency direction is strictly downward: `command-line -> render -> core`, `command-line -> rules-* -> core`,
`export -> core`, `metrics -> core`. Nothing depends on `command-line`. `core` depends on no crate that
does I/O. A `citymap-ui` crate, if one is ever requested, depends on `citymap-core` and on
`citymap-commands` (spec mutations, §3b), never on `citymap-command-line`.

## 3. Core types

Field-name lists only. The definitions in code win; whoever changes a type updates this
section in the same commit. After milestone 1 this section is replaced by a pointer to
`citymap-core::specification` and `citymap-core::model`.

- `Specification` (`citymap-core::specification`): `schema`, `identity` (name, seed, notes),
  `settlement`, `site`, `defences`, `open_space`, `crossings`, `districts`, `landmarks`,
  `roads`, `presets`. One field per topic, in the order of REQUIREMENTS.md F2. In code so
  far (#94, #95): `schema`, `identity`, `settlement` (population), `site` (`extent_metres`,
  `water.rivers`) and `presets` (era).
- `SiteOutput` (`citymap-core::site`, S0): in code so far (#99), `rivers`, each a
  `RiverCourse` with `identifier`, `course` and `width_metres`.
- `LandmarkSpec`: `identifier`, `name`, `function`, `label`, `importance`, `footprint`,
  `constraints`, `resolved`, `pinned`, `notes`. Open spaces and crossings carry `resolved` and
  `pinned` too; an unpinned open space or crossing takes its width and depth from its `kind`'s
  default in the default rule pack, so placement, collision search and `min_spacing` always
  see a footprint. Open-space sizes per kind are in the default rule pack (ADR-021 seventh
  round: market_square and plaza 2,500 to 11,250 m², aspect 1 : 1 to 2 : 1; green 80 to
  100 m a side; parade_ground 90 m square). A crossing's footprint is 8 m wide, the river's `width_metres` long, plus
  5 m on each bank for a bridge; these are provisional default-pack values that Jeffrey
  revisits when street widths are decided (specification-version-1 §2, crossings row; ADR-021 seventh round).
- `Constraint`: a closed enum with one variant per F3 rule, internally tagged by `rule`;
  every variant carries its own parameters plus `hard` and `weight`, so the YAML stays flat
  without `#[serde(flatten)]`, which breaks `deny_unknown_fields` (ADR-017, ADR-021). Core
  never holds untyped values. `near_point` is the one variant a spec file cannot write: the
  generator adds it when `presets.reuse_positions` is on, and validation rejects it in a
  spec.
- `CityModel`: one field per stage, each with exactly one producer: `site` (S0), `layout`
  (S1: wards, districts, walls, gates), `placements` (S2: listed crossings, open spaces
  and landmarks, each keyed by its spec id), `streets` (S3), `blocks` (S4:
  blocks and parcels), `buildings` (S5), `labels` (S6), `attributes` (S7).
  Rejected alternative: one flat struct with nine `Vec` fields; it hides which stage owns
  which collection.

Coordinates are `f64` metres in a local frame, rounded to the centimetre and serialised in
the shortest form that reads back as the same value (`1200.0`, `4.5`), so JSON is
byte-stable (ADR-038). `Identifier` is a newtype over a 64-bit hash with a debug-friendly
string form (`"street:7f3a..."`).

## 3b. Command model

A command is a function from a spec to a new spec: `fn apply(spec: &Spec, cmd: &Command) ->
Result<Spec, CommandError>`. The first commands are `SetSeed`, `SetResolved { object,
placement }` and `Pin { object }`, where `object` is the identifier of a landmark, open space or
crossing. `--write-back` is the first implementation: it applies `SetResolved` for every
placed landmark, open space and crossing and `SetSeed`, then patches the file. A future
UI sends commands; it never calls a stage. Commands live in `citymap-commands`, created at
the milestone that first needs a second caller. No other commands exist until a caller needs
one; a new command is a new enum variant that edits one spec field, added by a PR with a
unit test, no ADR needed. This model follows ADR-005 (pins are resolved intent stored in the
spec) and ADR-021 (open spaces and crossings are pinnable like landmarks).

## 4. Stage pipeline

| Stage | Input | Output | RNG stream key |
|---|---|---|---|
| S0 site | spec.site, spec.roads and pinned placements (hand-authored, or imported from a regional source by a CLI adapter; core does not care) | height field, water mask, shoreline, cliff lines; river courses; road courses in three steps (every road without a `core` end; then the town core, chosen as in specification-version-1 §4; then roads with a `core` end); the points where roads cross rivers and each other, and at each road–river point a crossing (kind from the road's `river_crossing`, id `<road>.<river>`, with `.<n>` numbered from 1 along the road from its first listed end when a road crosses that river more than once, position fixed by the courses; not a constraint target) | `("site", cell)` |
| S1 macro | S0, districts, defences, pinned sites | in this order: coarse grid; one ward site per cell (jittered by the cell's stream) plus pinned sites; Voronoi of those sites with no global relaxation; district assignment, wall areas from `encloses` (nested, side by side, closed by water where `along_water` says), gates where roads cross walls plus listed posterns and water gates | `("macro", cell)` |
| S2 placement | S1, crossing, open-space and landmark specs in dependency order (F8) | placed crossings, open spaces and landmarks with footprints | `("placement", object_identifier)`, where `object_identifier` is the spec id of the crossing, open space or landmark |
| S3 streets | S0, S1, S2 | primary roads (the road courses: through roads run on, ending roads end at the core or their pinned landmark or open space, local roads run between their ends; plus links from the core to every market square and to every major landmark), secondary per district (direction field), tertiary per block | `("streets", cell)` |
| S4 blocks | S3 | blocks, parcels (subdivision method: open decision, milestone 6) | `("parcels", block_id)` |
| S5 buildings | S4, district type | footprints | `("bld", parcel_id)` |
| S6 labels | S2, S3, spec names | label anchors and priorities (placement itself is in render) | `("labels", object_identifier)` |
| S7 decorators | all | attributes only, no geometry: season, stability, festival state | `("pack:<identifier>", object_identifier)` |

Each stage is `fn run(ctx: &Ctx, input: &Upstream) -> Result<Output, StageError>`. `Ctx`
carries the spec, the master seed, the rule registry and a `random_stream_for(stage, identifier)`
function. A stage may read any earlier stage's output and nothing else. In code so far (#99),
S0 builds only the river courses and is `site::run(&Specification) -> SiteOutput`; `Ctx` and
`StageError` arrive with the first stage that needs them (ADR-039).

Errors: one `GenerateError` enum in core with a variant per stage. Every variant carries
structured data (ids, rule names, candidates tried); `Display` is derived from that data and
the CLI formats it. Tests assert on the data, never on the message text. Warnings (the
legibility guard, Q5) travel on a separate `Vec<Warning>` returned next to the model, never
as errors. The CLI prints every warning to stderr, one per line prefixed `warning:`, and
exits 0.

Every run executes all stages. Incremental recomputation is built only when Q2 is violated
(REQUIREMENTS.md §6); correctness never depends on it.

## 4b. Logging

Core emits `tracing` events only and never installs a subscriber. The CLI installs the
subscriber, writes it to stderr, and sets the level from `-v` (info) and `-vv` (debug);
default is warn. Log output is excluded from the byte-identical claim. A `tracing` field
expression must not call non-deterministic code, because field expressions run during
generation.

## 5. Determinism design

- **RNG:** `random::random_stream_for(seed, stage, identifier) -> RandomStream`: the stage label is
  hashed into a salt, the seed, salt and id value are mixed with `stream_seed`, and a `Pcg64Mcg` is
  seeded from that result and its `mix_seed`; `RandomStream` hides the generator (ADR-018, ADR-032).
  Stages draw through `next_unit_f64` (0 up to but not including 1) and `next_below(n)`
  (a whole number from 0 up to but not including `n`), never through a library
  distribution (ADR-033).
  No stage ever shares a generator with another object. Spatial noise uses stateless
  `hash(seed, cell_x, cell_y)`.
- **Ids:** `Identifier = hash(kind, parent identifiers, stable keys)`, written as `kind:` plus 16
  hex digits (ADR-030). Node ids are cell-anchored; street id = hash of the sorted endpoint ids;
  block id = hash of the sorted bounding street ids; parcel id = hash of the block id and the parcel
  number `k`; landmark and district ids come from the spec. Indices are allowed inside one stage
  function; a stage output carries ids only. Two objects with the same id in one stage output is an
  error, checked by a debug assertion and a property test. The author of each stage makes it
  call `identifier::debug_assert_unique` (built on `identifier::check_unique`) on its output in
  the PR that builds the stage.
- **Ordering:** every collection iterated for generation is a `Vec` in a canonical order or a
  `BTreeMap`. Sort by id before feeding points to Delaunay. Comparators are total; ties break
  by id.
- **Floats:** topology decisions use `+ - * / sqrt floor` and integer-quantised comparisons.
  Trig and exp go through `libm`. Serialise centimetre-rounded values in their shortest form.
- **Geometry facade:** all geometry goes through `citymap-core::geometry`, which owns the point,
  polygon and line types; no other module imports `geo`, `spade`, `i_overlay` or `rstar`.
  Version 1 guarantees the same output on the same machine and toolchain; behind the facade
  `geo` 0.33 calls std transcendentals, so output across platforms can differ until #67
  replaces that backend; Jeffrey decides when #67 is complete (ADR-020).
- **Pins:** a pinned landmark's, open space's or crossing's `resolved` placement (position,
  rotation and size) is copied into the model untouched and
  injected into S1 as a fixed Voronoi site and into S3 as an obstacle. An unpinned
  `resolved` is ignored, unless `presets.reuse_positions` is on: then it adds a soft
  `near_point` rule at that centre, with `presets.reuse_within_metres` (default 50 m) and
  `presets.reuse_weight` (default 1). Unpinned crossings, open spaces and landmarks are
  placed in F8 order (dependency order); collision resolution is a bounded deterministic
  spiral search, then a reported failure. Objects placed earlier are never moved to make
  room.
- **Locality:** the unit is the grid cell, never the ward. Ward sites are derived per cell
  from that cell's stream, so a ward is keyed by its cell. There is no global Lloyd
  relaxation, because relaxation moves every site when one site is added. Per-cell content
  derives from the cell and its one-ring. Adding a landmark dirties its cell and ring only.
  Rejected alternative: Watabou-style spiral sites with Lloyd relaxation (global, so not
  local). This is testable (see §8).

## 6. Rule packs

Illustration only, not a format. The pack format is decided at milestone 8.

```yaml
# packs/peace-and-war.yaml
identifier: peace-and-war
version: 1
rules:
  - identifier: districts-compass-locked
    stage: macro
    rule: compass_region          # built-in evaluator, citymap-core::rules
    hard: true
    selector: { district_has: compass }
    params: { from: district.compass }
stages:
  - identifier: season
    after: macro
    evaluator: pnw.season         # attribute-only stage, registered in citymap-rules-pnw
```

- Constraints are the closed enum `Constraint` (§3) with one variant per F3 rule,
  serde-tagged by the rule name; each variant carries its parameters, `hard` and `weight`. The spec is therefore plain data: it derives `Serialize`,
  `Deserialize`, `Clone`, `PartialEq`, `Debug` and `JsonSchema` with no trait objects.
  A pack cannot add a constraint kind; it can add attribute stages. A second setting that
  needs a new constraint kind adds a variant through an ADR (ADR-017).
- An evaluator is `trait Evaluator { fn score(&self, ctx: &Ctx, candidate: &Candidate,
  constraint: &Constraint) -> Score }` with `Score::Reject | Score::Value(f64)`.
  `Candidate` is an enum over the placeable kinds (ward, landmark, open space, crossing).
  District rules are scored on wards. Hard rules
  reject; soft rules add weighted scores; ties break by the candidate's own RNG stream.
- The registry is `BTreeMap<RuleName, Box<dyn Evaluator>>`, built explicitly in
  `citymap-command-line` as `Registry::new().with(citymap_core::rules::builtin())`, and handed to
  core through `Ctx`. No link-time registration crates (`inventory`, `linkme`): they are
  order-dependent and do not build for WASM. This is the one trait object in core.
- Packs load in declared order and merge by rule id. Each pack's RNG streams are salted with
  the pack id, so adding a pack cannot change core stage output.
- Unknown evaluator types are a validation error. A duplicate rule id across loaded packs
  is a validation error, not an override.
- A pack or style named in the spec resolves in this order: the spec file's directory, then
  the repository's `packs/` or `styles/`, then the compiled-in standard pack and default
  style. The first match wins.
- The spec schema is generated from the Rust types with `schemars` by
  `cargo run -p citymap-command-line -- schema`, committed as `documentation/specification.schema.json`, and a test
  fails when the committed file is stale.

## 6b. Who writes what

| Artefact | Form | Author | How often |
|---|---|---|---|
| Evaluators | Rust code in `citymap-core::rules` (built-in) or a pack crate | Written by Claude, reviewed and merged by Jeffrey | one per new constraint type, each by ADR |
| Rule packs | YAML in `packs/` | Drafted by Claude from the vault's physics notes, reviewed by Jeffrey | once per setting |
| City specs | YAML in the vault's `Maps/specifications/` | Written by Claude from a vault note plus Jeffrey's dictation, reviewed by Jeffrey (trivial-fix rule in CODING_STANDARDS.md) | one per city, edited as the campaign grows |

Nobody hand-writes a spec from a blank file. The schema is optimised for an AI writing it
correctly and a validator catching the rest: generated JSON schema, strict validation with
field-naming errors, documented enums, free-text notes fields for prose. Friendliness for
manual authoring is not a design goal.

## 7. Rendering

- `citymap-render` turns a `CityModel` and a `Style` into an SVG string with one `<g>` per
  layer. Sketchiness is in the geometry: wobbled multi-stroke outlines and hachure fills
  generated with the object's own RNG stream (a rough.js-style port written by us).
  SVG filters are reserved for an optional paper layer that can be switched off.
- Label placement: numbered markers plus a legend are the guaranteed fallback. The placement
  algorithm is an open decision in DECISIONS.md, settled at milestone 7. Text metrics come
  from the bundled fonts so measurement is headless and stable.
- PNG via `resvg` + `tiny-skia` with bundled OFL fonts (`resvg` added at milestone 0b behind the render facade with its own ADR, ADR-026; the fonts
  are chosen at milestone 7 from the OFL list in research-rendering.md; Jeffrey decides);
  A4 at 300 DPI required, A3 nice-to-have.
- Foundry export (later) emits a label-free PNG plus Scene JSON and a Universal VTT file.

Crate choices are open decisions in DECISIONS.md, settled by ADR at the milestone named there.

## 8. Testing strategy

Three kinds of tests per stage, plus two project-wide harnesses.

1. **Unit tests** for evaluators, id derivation, geometry helpers, spec validation errors.
2. **Property tests** (`proptest`) on invariants, run on toy specs (at most 8 landmarks, 4
   districts) with 64 cases each:
   - Determinism: `generate(spec, seed) == generate(spec, seed)` byte-for-byte.
   - Pin invariance: for any seed pair, pinned landmark, open space and crossing placements
     are identical.
   - Monotone placement: appending an object leaves the S2 placement of every object of its
     own kind and of an earlier kind unchanged (F8); appending a landmark changes no other S2
     placement. Later stages may change: a major landmark gets its S3 link, so streets,
     blocks and buildings around it differ.
   - Locality: appending a landmark changes no object whose cell is outside the dirty ring.
   - Pack isolation: adding a pack with only attribute stages leaves all geometry unchanged.
   - Schema round-trip: spec -> YAML -> spec is identity.
3. **Golden snapshot tests** (`insta`): model JSON for every stage of every calibration spec
   at fixed seeds, and SVG for the three calibration cities only. PNG is never snapshot-tested.
   `.snap` files live in `crates/citymap-command-line/tests/snapshots/`, the directory `insta`
   creates next to the golden tests. CI runs them on Windows, the reference platform; Linux
   runs every other test (ADR-020). Only Jeffrey accepts a snapshot change,
   with `cargo insta review`; a snapshot update is its own commit with the reason in the
   subject.
4. **Expressive range harness** (`citymap sweep`): many seeds per spec, metrics CSV, contact sheet
   PNG. Compared by eye and in `analysis/` notebooks against at least three real medieval town plans chosen by Jeffrey.
   Run before tuning any preset.
5. **Visual review**: any step that touches rendering ends with a PNG in `output/` that a human
   looked at. The commit message says what was looked at.

CI is GitHub Actions on `github.com/lurtze1/CityMapMaker`, with one Windows job and one
Linux job. Each job runs `cargo fmt --check`, `cargo clippy --workspace --all-targets -D
warnings` and `cargo test --workspace`. The Linux job also builds
`citymap-core` for `wasm32-unknown-unknown` as a purity check, not a release target, and
runs `cargo deny check` against `deny.toml` (licences, advisories, sources; ADR-027). A PR
merges only when both jobs are green (ADR-013).

## 9. Build order (milestones; one or two sessions each)

The tier in parentheses is the REQUIREMENTS.md §9 checkpoint that judges the milestone's
output. This table is the only place the mapping lives.

| # | Milestone | Done when |
|---|---|---|
| 0 | Workspace skeleton, CI, `random` and `identifier` modules with tests | determinism and id property tests pass |
| 0b | Proof of concept, a thin vertical slice: spec types and validation for `schema`, `identity`, `settlement.population`, `site.extent_metres`, `site.water.rivers`, `roads` and `presets.era`; the `geometry` facade; S0 road courses in the three steps, the town core and the road–river crossings; an SVG of site, rivers, roads, core and crossings, and a PNG of it | `generate specifications/poc.yaml --seed 1 --output output/` writes an SVG and a PNG that Jeffrey looked at; two runs give byte-identical SVG |
| 1 | Spec types, schema, `validate`, Truro spec checked in with GM? placeholders | `validate specifications/truro.yaml` passes and names a deliberate error |
| 2 (T0) | S0 site: coast, hills, cliffs as fields; render terrain only to PNG | a Truro-shaped coastline PNG |
| 3 (T1, T2) | S1 macro: ward mesh, district assignment, palisade, gates | wards and wall drawn; compass lock tested on a toy spec; T1 is judged with district shading off, T2 with it on |
| 4 (T3) | S2 landmarks with pins and `--write-back` | pin invariance and monotone placement tests pass |
| 5 (T4) | S3 streets: primary least-cost, secondary by direction field | Truro passes tier T4; sweep harness exists |
| 6 (T5) | S4 and S5 blocks, parcels, buildings | first complete Truro map |
| 7 (T6) | Labels, legend, style presets, screen and A4 export | Truro handout on screen and on A4 |
| 7b (T6) | Minimal Foundry export: label-free PNG plus scene JSON with correct dimensions and grid | the Truro map opens as a scene in Foundry |
| 8 (T7) | Arcachon: seasonal pack, moving landmarks | pack isolation tests pass; Arcachon map |
| 9 (T8, T9) | Bearna: cliffs and levels, cave gate | Bearna map; expressive range compared across all three |
| 10 | Full Foundry export: city wall as walls, gates as doors, landmark notes, Universal VTT | walls and notes work in a Foundry scene |

There are no releases. `main` is the product and crate versions stay `0.1.0`. Each tier
Jeffrey passes gets an annotated git tag `tier-N` on the squash commit that passed it
(ADR-015).

Nothing in milestones 0 to 7 is allowed to know about Arcachon's seasons or Bearna's cliffs
except through the generic constraint vocabulary. If a milestone needs a special case, that is
a sign the rule system is missing an evaluator, not a reason to add an `if`.
