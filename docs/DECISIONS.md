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
Consequences: no UI work before milestone 7; the spec schema is the primary API and must be
versioned.

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

## Open decisions

- Geometry crate choice (`geo` + `spade` + `i_overlay` versus alternatives) settled at
  milestone 0 after a spike on robustness with degenerate input; Jeffrey approves.
- Whether `sweep` metrics are compared against real town plans by hand or with a scripted
  corpus.
