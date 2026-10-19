# Coding standards — CityMapMaker

The named patterns we follow, so a review can ask "does this change respect X" instead of
arguing taste. Each has a one-line test a reviewer can apply.

## Architecture patterns

| Pattern | What it means here | Reviewer test |
|---|---|---|
| Hexagonal (ports and adapters) | `citymap-core` defines types and functions; CLI, file I/O, Foundry export and any UI are adapters that depend on core | Does core import anything from an adapter crate? Then reject |
| Functional core, imperative shell | Core is pure functions over immutable values; only `citymap-cli` reads, writes, prints | Any `std::fs`, `std::env`, `std::time`, `println!` in core? Then reject |
| Parse, don't validate | `citymap-cli` parses YAML once into typed structs that cannot be invalid; core never sees YAML or an untyped value | Does core re-check something its type already guarantees, match on a string that should be an enum, or import a YAML crate? Then reject |
| Pipeline of immutable stage outputs | Each stage is `fn(ctx, upstream) -> Result<Output>`; outputs are values keyed by stable id; nothing mutates upstream data | Does a stage take `&mut` to an earlier stage's output? Then reject |
| Newtypes for units and ids | `Metres`, `Id`, `Seed`, `Cell` are distinct types | Is a bare `f64` or `u64` crossing a public function boundary where a unit is implied? Then wrap it |
| Command pattern for edits | Every edit, from any source, is a spec mutation | Does anything outside core call a stage directly? Then route it through the spec |
| Registry for evaluators | Rule names are looked up in a `BTreeMap` built explicitly in the CLI and passed through `Ctx` | Is there an `if rule == "..."` chain, or a link-time registration crate? Then reject |
| Hash-based determinism | Random values come from `rng_for(stage, id)`; ids derive from parents and keys | Any `thread_rng`, index-derived id, or iteration over a `HashMap` that affects output? Then reject |

Deliberately not used: entity component systems, async, trait-heavy abstraction before a
second concrete implementation exists, `unsafe`.

## Comments

- No inline comments, with exactly one exception: where naming cannot carry the reason a
  piece of code is the way it is, a comment states that reason. There is no second
  exception. A comment that says what the code does is rejected in review.
- Reasons for a design choice go in `docs/DECISIONS.md`, not in the code.
- Lint allows never appear at a use site. They live in `[workspace.lints]` in the root
  `Cargo.toml`, decided once at milestone 0 by ADR, with the reason for each in that ADR.
- Doc comments (`///`) are required on every public item (crate, module, type, function),
  placed above the item, one to three lines stating what it is for and why it exists.
  Never inside a function body. Decided by Jeffrey 2026-10-19; rejected alternatives: no
  doc comments at all, or doc comments on modules only.

## Rust conventions

- Follow the Rust API Guidelines (rust-lang.github.io/api-guidelines) for naming, docs and
  error types.
- `rustfmt` defaults. `[workspace.lints.clippy] pedantic = "warn"` plus the allow list from
  the milestone-0 ADR; CI runs clippy with `-D warnings`.
- Errors: `thiserror` enums in library crates, `anyhow` only in `citymap-cli`. No panics on
  bad input; `unwrap` and `expect` only on invariants, with a message naming the invariant.
- `#![warn(missing_docs)]` in every library crate enforces the doc-comment rule.
- No `unsafe`. No `async`. No generics or lifetimes in public core APIs unless the PR
  explains the need; own your data (`String`, `Vec`) at boundaries.
- Collections: `Vec` in canonical order or `BTreeMap`/`BTreeSet` wherever iteration can
  touch output. `HashMap` only for lookups whose order never leaks.
- Floats: `f64` everywhere. Topology decisions use quantised integers. Transcendental
  functions go through `libm`; the std methods `f64::sin`, `cos`, `tan`, `atan2`, `exp`,
  `ln`, `powf`, `hypot` are banned by `clippy::disallowed_methods`. `clippy::float_cmp` is
  denied. Sorting floats uses `total_cmp`. NaN is rejected at the spec boundary. A committed
  `.cargo/config.toml` sets no `target-cpu`. The `Metres` newtype owns fixed-precision
  serialisation; no ad-hoc rounding elsewhere.
- Modules: one module per stage in core (`site`, `macro_layout`, `landmarks`, `streets`,
  `blocks`, `buildings`, `labels`, `decorators`) plus `spec`, `model`, `id`, `rng`, `rules`.
- Tests next to the code for units and properties; `tests/` for cross-crate and golden
  snapshots (`insta`). Snapshot files are reviewed in the diff.
- Dependencies: add one only with an ADR line; `cargo deny` checks licenses and duplicates.

## Review hygiene

- Trivial fix, the only change Claude makes without a plan and a yes: no behaviour change,
  no new public item, under 20 changed lines. A spec edit is trivial unless it adds a
  landmark or district. Everything else gets a design of at most ten lines (types, crate,
  stage, tests) and Jeffrey's yes before any code is written.

- Flow: every step is a branch named `m<milestone>/<short-name>` and a pull request
  against `main`. Claude opens the PR; Jeffrey reviews and merges; nobody commits to `main`
  directly after milestone 0 starts. CI must be green before merge (ADR-014).
- Issues: one GitHub issue per smallest independently testable piece of work, for example
  "parse the `site.coast` field", "reject a spec without `schema`", "exit code 3 on
  constraint failure". Never an issue called "build the generator" or "implement streets".
  An issue that needs more than one step is split before work starts. GitHub milestones
  mirror §9 of ARCHITECTURE.md; labels carry the tier (`tier:T0` to `tier:T9`).
- Conventional commit subjects: `feat(streets): ...`, `fix(rng): ...`, `docs: ...`,
  `test(macro): ...`, `chore: ...`. Body explains why and names the tier or milestone. A PR
  body says `Closes #N` for the GitHub issue it completes.
- One concern per commit; a refactor and a behaviour change never share one.
- Non-obvious Rust idioms are explained in the commit body, never in a code comment, since
  the reviewer is learning the language and the explanation belongs to the change, not the
  file.
- "Boring over clever": the reviewer rejects code they cannot read in one pass, even if
  it is correct.
- Snapshot updates are a separate commit with the reason in the subject.

## Codebases worth reading for shape

- `resvg` / `usvg`: a parse-into-typed-tree then render pipeline in Rust, no DOM.
- `ripgrep`: how a Rust CLI crate wraps pure library crates.
- `tiny-skia`: small, pure, well-documented geometry and raster code.
