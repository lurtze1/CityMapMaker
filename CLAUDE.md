# CLAUDE.md — CityMapMaker

Spec-driven generator of fantasy **city overview maps** for Jeffrey's *Peace and War* campaign
(Pathfinder 2e). You give it a written city spec; it produces an annotated, hand-drawn-looking
map (SVG + PNG) with a numbered landmark legend. Later: dungeon maps, Foundry VTT export, a
small editing UI. Read this file fully before doing anything.

## Documents (read in this order)

| File | What |
|---|---|
| `docs/REQUIREMENTS.md` | Goals, non-goals, functional requirements, the three calibration city briefs |
| `docs/ARCHITECTURE.md` | Crate layout, command model, stage pipeline, determinism rules, testing strategy, build order |
| `docs/DECISIONS.md` | Decision log (ADR style). Add an entry whenever a real choice is made |
| `docs/research/*.md` | Research reports (tools, academic, determinism, rendering). Reference, not gospel; claims are tagged verified/unverified |

The campaign lore lives in an Obsidian vault at
`D:\Google Drive\RPG DRIVE\Peace and War\Peace and War\` with its own CLAUDE.md. That vault is
the **single source of truth for names, landmarks and geography**. This repo never invents lore.

## Pace and working agreement

- **Small steps, reviewed.** This is a months-long project, not an afternoon. One milestone step
  per session: plan it, implement it, test it, stop. Jeffrey reviews every change; he is a
  single reviewer learning Rust, so keep diffs readable and explain non-obvious Rust idioms in
  the PR or commit text.
- **Plan before code.** For anything beyond a trivial fix, state the design in a few lines
  (types, which crate, which stage, which tests) and get a yes before writing it.
- **Ask about design, decide trivia yourself.** Ask when two readings lead to materially
  different code. Do not ask about naming, formatting or library-level choices that the docs
  already settle.
- **Never invent lore.** Landmark names, district names, NPCs and geography come from the
  vault or from Jeffrey. Filler buildings are anonymous. If a brief needs a fact the vault
  lacks, add it to the "Open questions for the GM" list in `docs/REQUIREMENTS.md` instead of
  guessing.
- **Voice input.** Jeffrey often dictates (Wispr Flow). Expect garbled proper nouns; interpret
  charitably against the vault, and ask when unsure rather than "fixing" a word.
- **Record decisions.** A new library, a changed invariant, a dropped requirement: one entry
  in `docs/DECISIONS.md`, same commit.
- **Report honestly.** If tests fail, show the output. If a step was skipped, say so.

## Language and toolchain

- **Rust**, stable toolchain, native Windows (MSVC). Workspace of crates; see ARCHITECTURE.md.
- **Python 3** is allowed only under `analysis/` for notebooks and expressive-range plots. No
  Python in the build or runtime path.
- Commands (once the workspace exists):

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p citymap-cli -- generate specs/truro.yaml --seed 1 --out out/
```

## Hard rules for code

1. **`citymap-core` is pure.** No file I/O, no clock, no environment, no threads, no logging
   side effects beyond `tracing` events. It is a function from `(Spec, Seed) -> CityModel`.
2. **No ambient randomness.** `rand::thread_rng`, `rand::random`, `SystemTime`, `HashMap`
   iteration order and `HashSet` iteration order must never influence output. Use the hashed
   per-object RNG streams from `citymap-core::rng`; use `BTreeMap`/`IndexMap` or sort before
   iterating.
3. **Stable ids, never indices.** Every generated object has an id derived from its parents'
   ids and stable keys (cell coordinates, spec ids). References point at ids.
4. **Pins are inputs.** A pinned landmark's resolved placement is read from the spec and never
   recomputed. Adding landmark N+1 must not move landmarks 1..N.
5. **Deterministic floats in topology.** Only `+ - * / sqrt floor` and integer hashing decide
   which-side/which-cell questions. Trig goes through the `libm` crate. Never compare floats
   for equality; quantise first.
6. **Rules are data.** Setting-specific rules (compass-locked districts, weather, "uncertain
   alleys") live in rule packs, never as `if setting == "peace_and_war"` in core.
7. **The UI, when it exists, only sends commands** that mutate the spec. It never calls stage
   functions directly and core never knows a UI exists.
8. **Every stage has three kinds of tests**: unit, property (determinism, pin invariance,
   locality) and golden snapshot. A stage without tests is not done.

## Definition of done for a step

- `cargo fmt`, `clippy -D warnings`, `cargo test` all clean.
- New behaviour has tests of the kinds listed above.
- Golden snapshots updated deliberately (reviewed diff), never blindly accepted.
- A PNG in `out/` was actually looked at when the step changes anything visual.
- `docs/DECISIONS.md` updated if a choice was made; `docs/ARCHITECTURE.md` updated if the
  crate layout or an invariant changed.

## Licensing note

Watabou's TownGeneratorOS is GPL-3.0. We **re-implement ideas** from it; we do not copy code.
ProbableTrain's generator is LGPL-3.0, same rule. This repo is MIT OR Apache-2.0 (ADR-012).
