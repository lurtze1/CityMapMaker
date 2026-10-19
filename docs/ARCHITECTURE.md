# Architecture — CityMapMaker

Status: v0.1, 2026-10-19. Nothing is built yet; this is the target shape. Change it when a
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
| constraint | a rule attached to one spec item (landmark, district, open space) |
| pack | a YAML file of rules and attribute stages, loaded by name from the spec |

## 1. Principles

1. **The spec is the state.** Everything the user has said about a city lives in one spec
   document. There is no hidden state in the generator and no state in a UI.
2. **Generation is a pure function.** `generate(spec, seed) -> CityModel`. No I/O, no clock,
   no ambient randomness. Same inputs, byte-identical output, on any machine.
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
  crates/
    citymap-core/            pure: spec types, model types, ids, rng, stages, rule engine
    citymap-rules-std/       standard evaluators (compass_region, near, on_water, ...)
    citymap-rules-pnw/       Peace and War pack: seasonal districts, uncertain alleys (later)
    citymap-render/          CityModel + Style -> SVG string; labels, legend; PNG via resvg
    citymap-cli/             clap binary: generate, validate, sweep, write-back
    citymap-export-foundry/  later: Scene JSON + Universal VTT
    citymap-metrics/         expressive-range metrics over a CityModel (used by sweep)
  specs/                     example.yaml only; real-city specs live in the vault (ADR-016)
  packs/                     rule pack data files
  styles/                    style presets (ink, ancient, ...) and bundled OFL fonts
  tests/                     cross-crate integration and golden tests
  analysis/                  Python notebooks for expressive-range plots (not in the build)
  docs/
  out/                       generated output, git-ignored
```

Dependency direction is strictly downward: `cli -> render -> core`, `cli -> rules-* -> core`,
`export -> core`, `metrics -> core`. Nothing depends on `cli`. `core` depends on no crate that
does I/O. A later `citymap-ui` crate depends on `cli`-level code or on core, never the reverse.

## 3. Core types (target shape; the definitions in code win, and this section is updated to match them)

```rust
// citymap-core::spec. One field per topic, mirroring REQUIREMENTS.md F2.
pub struct Spec { pub identity: Identity,          // name, seed, notes
                  pub settlement: Settlement,      // level, population, size class
                  pub site: Site,                  // terrain, water, land cover
                  pub defences: Defences,          // walls, citadel, gates
                  pub open_space: Vec<OpenSpace>,  // plazas, squares, greens
                  pub crossings: Vec<Crossing>,    // bridges, fords
                  pub districts: Vec<DistrictSpec>,
                  pub landmarks: Vec<LandmarkSpec>,
                  pub roads: Vec<EntryRoad>,
                  pub presets: Presets }           // style, era, rule packs

pub struct LandmarkSpec { pub id: Id, pub name: String, pub kind: LandmarkKind,
                          pub footprint: FootprintClass, pub constraints: Vec<Constraint>,
                          pub resolved: Option<Placement>, pub pinned: bool,
                          pub notes: String }

// A constraint is untyped only at the file boundary. Validation looks the rule name up in
// the registry and parses `params` into that evaluator's own typed struct; stages only see
// the typed form. This is the "parse, don't validate" rule from CODING_STANDARDS.md.
pub struct RawConstraint { pub rule: RuleName, pub hard: bool, pub weight: f64,
                           pub params: serde_yaml::Value }          // boundary only
pub struct Constraint    { pub rule: RuleName, pub hard: bool, pub weight: f64,
                           pub params: Box<dyn EvaluatorParams> }   // what stages see

// citymap-core::model (output). The model is the sum of the stage outputs, one field per
// stage, so each field has exactly one producer. Every object carries a stable Id.
pub struct CityModel { pub site: SiteOutput,          // S0
                       pub layout: LayoutOutput,      // S1: ward mesh, districts, walls, gates
                       pub landmarks: LandmarkOutput, // S2
                       pub streets: StreetOutput,     // S3
                       pub blocks: BlockOutput,       // S4: blocks and parcels
                       pub buildings: BuildingOutput, // S5
                       pub labels: LabelOutput,       // S6: anchors and priorities
                       pub attributes: Attributes }   // S7: decorator attributes by Id

pub struct LayoutOutput { pub wards: Vec<Ward>, pub districts: Vec<District>,
                          pub walls: Vec<Wall>, pub gates: Vec<Gate> }
// Rejected alternative: one flat struct with nine Vec fields. It hides which stage owns
// which collection, so a stage writing to another stage's data is an opinion instead of a
// type error.
```

Coordinates are `f64` metres in a local frame; serialised with a fixed decimal precision so
JSON is byte-stable. `Id` is a newtype over a 64-bit hash with a debug-friendly string form
(`"street:7f3a..."`).

## 4. Stage pipeline

| Stage | Input | Output | RNG stream key |
|---|---|---|---|
| S0 site | spec.site (hand-authored, or imported from a regional source by a CLI adapter; core does not care) | height field, water mask, shoreline, cliff lines | `("site", cell)` |
| S1 macro | S0, districts, defences, pinned landmark sites | ward mesh (Voronoi with pinned sites fixed), district assignment, wall rings, gate positions | `("macro", ward_id)` |
| S2 landmarks | S1, landmark specs in stable order | placed landmarks with footprints | `("landmark", landmark_id)` |
| S3 streets | S1, S2, entry roads | primary roads (least-cost gate to market), secondary per district (direction field), tertiary per block | `("streets", ward_id)` |
| S4 blocks | S3 | blocks, parcels (straight-skeleton or recursive split) | `("parcels", block_id)` |
| S5 buildings | S4, district type | footprints | `("bld", parcel_id)` |
| S6 labels | S2, S3, spec names | label anchors and priorities (placement itself is in render) | `("labels", object_id)` |
| S7 decorators | all | attributes only, no geometry: season, stability, festival state | `("pack:<id>", object_id)` |

Each stage is `fn run(ctx: &Ctx, input: &Upstream) -> Result<Output, StageError>`. `Ctx`
carries the spec, the master seed, the rule registry and a `rng_for(stage, id)` function. A
stage may read any earlier stage's output and nothing else.

Errors: one `GenerateError` enum in core with a variant per stage. Every variant carries
structured data (ids, rule names, candidates tried); `Display` is derived from that data and
the CLI formats it. Tests assert on the data, never on the message text. Warnings (the
legibility guard, Q5) travel on a separate `Vec<Warning>` returned next to the model, never
as errors.

The first implementation runs all stages every time. Incremental recomputation is an
optimisation to add once stages are stable; correctness never depends on it.

## 4b. Logging

Core emits `tracing` events only and never installs a subscriber. The CLI installs the
subscriber, writes it to stderr, and sets the level from `-v` (info) and `-vv` (debug);
default is warn. Log output is excluded from the byte-identical claim. A `tracing` field
expression must not call non-deterministic code, because field expressions run during
generation.

## 5. Determinism design

- **RNG:** `stream(master_seed, stage_salt, object_id) -> SmallRng-like generator`, built by
  hashing the three with a fixed 64-bit mixer (SplitMix64 finaliser) and seeding a `rand_pcg`
  or `rand_xoshiro` generator. No stage ever shares a generator with another object. Spatial
  noise uses stateless `hash(seed, cell_x, cell_y)`.
- **Ids:** cell-anchored node ids; street id = hash of sorted endpoint ids; block id = hash of
  sorted bounding street ids; parcel id = `block_id:k`; landmark and district ids come from
  the spec. Never an array index.
- **Ordering:** every collection iterated for generation is a `Vec` in a canonical order or a
  `BTreeMap`. Sort by id before feeding points to Delaunay. Comparators are total; ties break
  by id.
- **Floats:** topology decisions use `+ - * / sqrt floor` and integer-quantised comparisons.
  Trig and exp go through `libm`. Serialise with fixed precision.
- **Pins:** a pinned landmark's `resolved` placement is copied into the model untouched and
  injected into S1 as a fixed Voronoi site and into S3 as an obstacle. Unpinned landmarks are
  placed in spec order; collision resolution is a bounded deterministic spiral search, then a
  reported failure. Older landmarks are never moved to make room.
- **Locality:** S1 assigns a coarse grid; per-cell content derives from the cell and its
  one-ring. Adding a landmark dirties its cell and ring only. This is testable (see §8).

## 6. Rule packs

```yaml
# packs/peace-and-war.yaml
id: peace-and-war
version: 1
rules:
  - id: districts-compass-locked
    stage: macro
    rule: compass_region          # evaluator type, registered in citymap-rules-std
    hard: true
    selector: { district_has: compass }
    params: { from: district.compass }
stages:
  - id: season
    after: macro
    evaluator: pnw.season         # attribute-only stage, registered in citymap-rules-pnw
```

- An evaluator is `trait Evaluator { fn score(&self, ctx, candidate, params) -> Score }` with
  `Score::Reject | Score::Value(f64)`. Hard rules reject; soft rules add weighted scores; ties
  break by the candidate's own RNG stream.
- Packs load in declared order and merge by rule id. Each pack's RNG streams are salted with
  the pack id, so adding a pack cannot change core stage output.
- Unknown evaluator types are a validation error.
- The spec schema is generated from the Rust types with `schemars` and published in `docs/`.

## 6b. Who writes what

| Artefact | Form | Author | How often |
|---|---|---|---|
| Evaluators | Rust code in `citymap-rules-*` | Jeffrey and Claude, reviewed like any code | rarely; one per new constraint type |
| Rule packs | YAML in `packs/` | Drafted by Claude from the vault's physics notes, reviewed by Jeffrey | once per setting, seldom changed |
| City specs | YAML in `specs/` | Claude from a vault note plus Jeffrey's dictation; later a UI emits the same thing | often; one per city, edited as the campaign grows |

Nobody hand-writes a spec from a blank file. The schema is optimised for an AI writing it
correctly and a validator catching the rest: generated JSON schema, strict validation with
field-naming errors, documented enums, free-text notes fields for prose. Friendliness for
manual authoring is not a design goal.

## 7. Rendering

- `citymap-render` turns a `CityModel` and a `Style` into an SVG string with one `<g>` per
  layer. Sketchiness is in the geometry: wobbled multi-stroke outlines and hachure fills
  generated with the object's own RNG stream (a rough.js-style port written by us).
  SVG filters are reserved for an optional paper layer that can be switched off.
- Label placement: greedy eight-position placement with an R-tree for collisions, short
  simulated annealing when greedy placement leaves overlaps, numbered markers plus legend as the fallback. Text metrics
  come from the bundled fonts via a font-parsing crate so measurement is headless and stable.
- PNG via `resvg` + `tiny-skia` with bundled OFL fonts (chosen at milestone 7 from the OFL list in research-rendering.md; Jeffrey decides);
  A4 at 300 DPI required, A3 nice-to-have.
- Foundry export (later) emits a label-free PNG plus Scene JSON and a Universal VTT file.

Candidate crates. Open decision: settled at milestone 0 by a spike on robustness with
degenerate input; Jeffrey approves the final list. Candidates: `geo`,
`spade` (Delaunay/Voronoi), `i_overlay` (polygon booleans), `rstar` (R-tree), `libm`,
`rand_pcg` or `rand_xoshiro`, `serde`, `serde_yaml`, `schemars`, `resvg`, `usvg`, `tiny-skia`,
`ttf-parser` or `fontdue`, `clap`, `thiserror`, `tracing`, `insta`, `proptest`.

## 8. Testing strategy

Three kinds of tests per stage, plus two project-wide harnesses.

1. **Unit tests** for evaluators, id derivation, geometry helpers, spec validation errors.
2. **Property tests** (`proptest`) on invariants, run on toy specs (at most 8 landmarks, 4
   districts) with 64 cases each:
   - Determinism: `generate(spec, seed) == generate(spec, seed)` byte-for-byte.
   - Pin invariance: for any seed pair, pinned landmark placements are identical.
   - Monotone placement: appending a landmark leaves placements of earlier landmarks unchanged.
   - Locality: appending a landmark changes no object whose cell is outside the dirty ring.
   - Pack isolation: adding a pack with only attribute stages leaves all geometry unchanged.
   - Schema round-trip: spec -> YAML -> spec is identity.
3. **Golden snapshot tests** (`insta`): model JSON for every stage of every calibration spec
   at fixed seeds, and SVG for the three calibration cities only. PNG is never snapshot-tested.
   `.snap` files live next to the test that owns them. Only Jeffrey accepts a snapshot change,
   with `cargo insta review`; a snapshot update is its own commit with the reason in the
   subject.
4. **Expressive range harness** (`cli sweep`): many seeds per spec, metrics CSV, contact sheet
   PNG. Compared by eye and in `analysis/` notebooks against at least three real medieval town plans chosen by Jeffrey.
   Run before tuning any preset.
5. **Visual review**: any step that touches rendering ends with a PNG in `out/` that a human
   looked at. The commit message says what was looked at.

CI is GitHub Actions on `github.com/lurtze1/CityMapMaker`, with one Windows job and one
Linux job. Each job runs `cargo fmt --check`, `cargo clippy --workspace --all-targets -D
warnings`, `cargo test --workspace`, and `cargo deny check`. The Linux job also builds
`citymap-core` for `wasm32-unknown-unknown` as a purity check, not a release target. A PR
merges only when both jobs are green (ADR-013).

## 9. Build order (milestones; one or two sessions each)

Each milestone maps onto a review tier in REQUIREMENTS.md §9; the tier says what Jeffrey
judges when the milestone's PNG lands in `out/`.

| # | Milestone | Done when |
|---|---|---|
| 0 | Workspace skeleton, CI, `rng` and `Id` modules with tests | determinism and id property tests pass |
| 1 | Spec types, schema, `validate`, Truro spec checked in with GM? placeholders | `validate specs/truro.yaml` passes and names a deliberate error |
| 2 (T0) | S0 site: coast, hills, cliffs as fields; render terrain only to PNG | a Truro-shaped coastline PNG |
| 3 (T1, T3) | S1 macro: ward mesh, district assignment, palisade, gates | wards and wall drawn; compass lock tested on a toy spec |
| 4 (T2) | S2 landmarks with pins and `--write-back` | pin invariance and monotone placement tests pass |
| 5 (T4) | S3 streets: primary least-cost, secondary by direction field | Truro passes tier T4; sweep harness exists |
| 6 (T5) | S4 and S5 blocks, parcels, buildings | first complete Truro map |
| 7 (T6) | Labels, legend, style presets, screen and A4 export | Truro handout on screen and on A4 |
| 7b (T6) | Minimal Foundry export: label-free PNG plus scene JSON with correct dimensions and grid | the Truro map opens as a scene in Foundry |
| 8 (T7) | Arcachon: seasonal pack, moving landmarks | pack isolation tests pass; Arcachon map |
| 9 (T8, T9) | Bearna: cliffs and levels, cave gate | Bearna map; expressive range compared across all three |
| 10 | Full Foundry export: city wall as walls, gates as doors, landmark notes, Universal VTT | walls and notes work in a Foundry scene |

There are no releases. `main` is the product and crate versions stay `0.1.0`. Each tier
Jeffrey passes gets an annotated git tag `tier-N` on the merge commit that passed it
(ADR-015).

Nothing in milestones 0 to 7 is allowed to know about Arcachon's seasons or Bearna's cliffs
except through the generic constraint vocabulary. If a milestone needs a special case, that is
a sign the rule system is missing an evaluator, not a reason to add an `if`.
