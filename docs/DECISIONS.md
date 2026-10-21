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
