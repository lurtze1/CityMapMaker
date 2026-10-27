# Coding standards — CityMapMaker

The named patterns we follow, so a review can ask "does this change respect X" instead of
arguing taste. Each has a one-line test a reviewer can apply.

## Architecture patterns

| Pattern | What it means here | Reviewer test |
|---|---|---|
| Hexagonal (ports and adapters) | `citymap-core` defines types and functions; CLI, file I/O, Foundry export and any UI are adapters that depend on core | Does core import anything from an adapter crate? Then reject |
| Functional core, imperative shell | Core is pure functions over immutable values; only `citymap-command-line` reads, writes, prints | Any `std::fs`, `std::env`, `std::time`, `println!` in core? Then reject |
| Parse, don't validate | `citymap-command-line` parses YAML once into typed structs that cannot be invalid; core never sees YAML or an untyped value | Does core re-check something its type already guarantees, match on a string that should be an enum, or import a YAML crate? Then reject |
| Pipeline of immutable stage outputs | Each stage is `fn(ctx, upstream) -> Result<Output>`; outputs are values keyed by stable id; nothing mutates upstream data | Does a stage take `&mut` to an earlier stage's output? Then reject |
| Newtypes for units and ids | `SignedLength`, `Length`, `Identifier`, `Seed`, `Cell` are distinct types | Is a bare `f64` or `u64` crossing a public function boundary where a unit is implied? Then wrap it |
| Command pattern for edits | Every edit, from any source, is a spec mutation | Does anything outside core call a stage directly? Then route it through the spec |
| Registry for evaluators | Rule names are looked up in a `BTreeMap` built explicitly in the CLI and passed through `Ctx` | Is there an `if rule == "..."` chain, or a link-time registration crate? Then reject |
| Hash-based determinism | Random values come from `random_stream_for(stage, identifier)`; ids derive from parents and keys | Any `thread_rng`, index-derived id, or iteration over a `HashMap` that affects output? Then reject |

Deliberately not used: entity component systems, async, trait-heavy abstraction before a
second concrete implementation exists, `unsafe`.

## Comments

- No inline comments, with exactly one exception: where naming cannot carry the reason a
  piece of code is the way it is, a comment states that reason. There is no second
  exception. A comment that says what the code does is rejected in review.
- Reasons for a design choice go in `documentation/DECISIONS.md`, not in the code.
- The no-inline-comment rule applies to Rust source only. Configuration files (workflow
  YAML, `Cargo.toml`, `clippy.toml`, `deny.toml`) carry a comment where the format needs
  one: a GitHub Action pinned to a commit SHA has its version tag as a comment on the same
  line, because the SHA is otherwise unreadable and Dependabot reads that tag to propose
  updates. Decided by Jeffrey 2026-10-19; rejected alternative: versions only in the commit
  message, which review found unreadable.
- Lint allows never appear at a use site. They live in `[workspace.lints]` in the root
  `Cargo.toml`, decided once at milestone 0 by ADR, with the reason for each in that ADR.
- Doc comments (`///`) are required on every public item (crate, module, type, function),
  placed above the item, one to three lines stating what it is for and why it exists.
  Blank `///` and `//!` lines count towards the three. The `# Errors` and `# Panics`
  sections required by Clippy pedantic come on top of those three lines. Never inside a
  function body. Decided by Jeffrey 2026-10-19; rejected alternatives: no doc comments at
  all, or doc comments on modules only. Clippy sections allowed by Jeffrey 2026-10-22 (#90).
  Three lines is the target, not an absolute limit (Jeffrey, 2026-10-26, #104): a doc comment
  is longer only when shortening it would lose information. Claude names each longer comment
  and its reason in the PR body; Jeffrey decides in review. A facade module's doc comment
  lists every external function it calls under `# <Crate> functions used`, which is always
  allowed (ADR-042).

## Naming

- Names are full words: our own Rust and Python names (types, classes, functions, modules,
  constants, variables, crates), document text, file and folder names, and YAML keys.
  `Specification`, not `Spec`; `Identifier`, not `Id`; `extent_metres`, not `extent_m`.
  Decided by Jeffrey 2026-10-25 (ADR-035).
- Stays short, because it cannot change or the short form is the real name: Rust keywords,
  macros and standard-library names (`fn`, `u64`, `Vec`, `fmt`); names of external crates;
  the format names YAML, JSON, SVG and PNG; process identifiers (ADR numbers, issue and PR
  numbers, `T0` to `T9`, `GM?`); folder and file names a tool requires (`src/`, `.github/`,
  `Cargo.toml`).
- A function that implements a published algorithm is named for what it does; its doc
  comment names the algorithm (`stream_seed`, documented as SplitMix64).
- Short names an external library brings in stay behind our facade, which uses full words
  (`RandomStream` wraps `Pcg64Mcg`). A library without a facade needs Jeffrey's exception
  in an ADR entry. Derive-based libraries are such an exception: serde (ADR-028) and
  thiserror (ADR-029) attach to our own types as derives, so a facade would only move an
  import path (#111).
- The Rust API Guidelines still decide upper and lower case.
- Reviewer test: does a new name contain a shortened word that is not on the stays-short
  list? Then reject.
- Every rule in this section except casing was decided by Jeffrey on 2026-10-24 and
  2026-10-25 (ADR-035); Claude proposed the stays-short list, Jeffrey accepted it. Only
  Jeffrey changes these rules, with a new ADR entry. Casing follows the Rust API Guidelines
  rule under "Rust conventions".

## Rust conventions

- Follow the Rust API Guidelines (rust-lang.github.io/api-guidelines) for naming, docs and
  error types.
- `rustfmt` defaults. `[workspace.lints.clippy] pedantic = "warn"` plus the allow list from
  the milestone-0 ADR; CI runs clippy with `-D warnings`.
- Errors: `thiserror` enums in every crate, `citymap-command-line` included (ADR-043). No panics on
  bad input; `unwrap` and `expect` only on invariants, with a message naming the invariant.
- `#![warn(missing_docs)]` in every library crate enforces the doc-comment rule.
- No `unsafe`. No `async`. No generics or lifetimes in public core APIs unless the PR
  explains the need; own your data (`String`, `Vec`) at boundaries.
- Collections: `Vec` in canonical order, `BTreeMap` and `BTreeSet`. `HashMap` and `HashSet`
  are banned in every crate by `clippy::disallowed_types`; Jeffrey decided the ban and its
  workspace-wide scope (ADR-031), and only Jeffrey lifts it, with a new ADR entry.
- Floats: `f64` everywhere. Topology decisions use quantised integers. Transcendental
  functions go through `libm`; the std methods `f64::sin`, `cos`, `tan`, `atan2`, `exp`,
  `ln`, `powf`, `hypot` are banned by `clippy::disallowed_methods`. The geometry crates
  behind the `geometry` facade are exempt until #67, and Jeffrey decides when #67 is complete
  (ADR-020); code outside the facade is not. The output guarantee is the same machine and
  toolchain, not every platform. `clippy::float_cmp` is
  denied. Sorting floats uses `total_cmp`. NaN is rejected at the spec boundary. A committed
  `.cargo/config.toml` sets `rustflags = []`; this declares the repository's own flags and
  cannot override two other sources: Cargo joins the `rustflags` arrays of every config
  file, and a `RUSTFLAGS` environment variable replaces all of
  them. The reference build is CI, which runs with `RUSTFLAGS` set to empty; a local build
  with `target-cpu` set is the user's own deviation and is not a bug in the project.
  Proposed in PR #18 on 2026-10-19; Jeffrey decides by merging it. Rejected alternative: a build script that
  fails when `target-cpu` is present, which would also block legitimate local experiments. The `SignedLength` and `Length` newtypes own the rounding
  and the serialisation of lengths; no ad-hoc rounding elsewhere.
- Modules: one module per stage in core (`site`, `macro_layout`, `placement`, `streets`,
  `blocks`, `buildings`, `labels`, `decorators`) plus `specification`, `validation`, `model`,
  `town_core`, `identifier`, `random`, `rules`, `units`, `geometry`.
- Tests next to the code for units and properties; each crate's `tests/` for cross-crate
  and golden snapshots (`insta`) of that crate's output. Snapshot files are reviewed in the diff.
- Dependencies: add one only with an ADR line; `cargo deny` checks licenses and duplicates.

## Review hygiene

- Trivial fix, the only change Claude makes without a plan and a yes: no behaviour change,
  no new public item, under 20 changed lines. A spec edit is trivial unless it adds a
  landmark or district. Everything else gets a design of at most ten lines (types, crate,
  stage, tests) and Jeffrey's yes before any code is written.

- Flow: every step is a branch named `m<milestone>/<short-name>` and a pull request
  against `main`. Claude opens the PR; Jeffrey reviews and merges; nobody commits to `main`
  directly after milestone 0 starts. CI must be green before merge (ADR-014).
- Merging: a squash merge, the only method the ruleset allows, so `main` gets one commit per
  PR. The PR title becomes that commit's subject and the PR body its message. Claude updates
  a branch by rebasing it on `main` and pushing with `git push --force-with-lease`, never by
  merging `main` into it. A file rename (`git mv`) is its own PR, because a squash joins it
  with content changes and git then loses the file's history (ADR-041).
- Review conversations: every CodeRabbit (or human) review comment is either fixed, with a
  reply saying what changed, or denied, with a reply giving the reason; then the
  conversation is marked resolved. The ruleset blocks a merge while any conversation is
  open (ADR-022). Claude decides fix or deny for each comment and processes all comments
  before asking Jeffrey to merge; Jeffrey overrules a denial by reopening the conversation.
  CodeRabbit is optional and reviews a PR only when Jeffrey comments `@coderabbitai review`
  (the changes since its last review) or `@coderabbitai full review` (the whole PR); Claude
  never posts either. Claude processes its comments like any other, including the ones it
  lists outside the diff, which get one reply in a PR comment (ADR-040).
- Issues: one GitHub issue per smallest independently testable piece of work, for example
  "parse the `site.coast` field", "reject a spec without `schema`", "exit code 3 on
  constraint failure". Never an issue called "build the generator" or "implement streets".
  An issue that needs more than one step is split before work starts. GitHub milestones
  mirror §9 of ARCHITECTURE.md; labels carry the tier (`tier:T0` to `tier:T9`).
- Conventional commit subjects: `feat(streets): ...`, `fix(random): ...`, `docs: ...`,
  `test(macro): ...`, `chore: ...`. A PR title uses the same form, because it becomes the
  commit subject on `main`. The PR body explains why, names the tier or milestone and says
  `Closes #N` for the GitHub issue it completes.
- One concern per commit inside a branch, and one concern per PR on `main`; a refactor and a
  behaviour change never share one.
- Non-obvious Rust idioms are explained in the PR body, never in a code comment, since
  the reviewer is learning the language and the explanation belongs to the change, not the
  file.
- "Boring over clever": the reviewer rejects code they cannot read in one pass, even if
  it is correct.
- Snapshot updates are a separate commit with the reason in the subject.

## Codebases worth reading for shape

- `resvg` / `usvg`: a parse-into-typed-tree then render pipeline in Rust, no DOM.
- `ripgrep`: how a Rust CLI crate wraps pure library crates.
- `tiny-skia`: small, pure, well-documented geometry and raster code.
