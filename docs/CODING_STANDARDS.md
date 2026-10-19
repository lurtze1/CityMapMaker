# Coding standards — CityMapMaker

The named patterns we follow, so a review can ask "does this change respect X" instead of
arguing taste. Each has a one-line test a reviewer can apply.

## Architecture patterns

| Pattern | What it means here | Reviewer test |
|---|---|---|
| Hexagonal (ports and adapters) | `citymap-core` defines types and functions; CLI, file I/O, Foundry export and any UI are adapters that depend on core | Does core import anything from an adapter crate? Then reject |
| Functional core, imperative shell | Core is pure functions over immutable values; only `citymap-cli` reads, writes, prints | Any `std::fs`, `std::env`, `std::time`, `println!` in core? Then reject |
| Parse, don't validate | YAML is parsed once at the boundary into typed structs that cannot be invalid; core trusts its types | Does core re-check something its type already guarantees, or match on a string that should be an enum? Then simplify |
| Pipeline of immutable stage outputs | Each stage is `fn(ctx, upstream) -> Result<Output>`; outputs are values keyed by stable id; nothing mutates upstream data | Does a stage take `&mut` to an earlier stage's output? Then reject |
| Newtypes for units and ids | `Metres`, `Id`, `Seed`, `Cell` are distinct types | Is a bare `f64` or `u64` crossing a public function boundary where a unit is implied? Then wrap it |
| Command pattern for edits | Every edit, from any source, is a spec mutation | Does anything outside core call a stage directly? Then route it through the spec |
| Registry for evaluators | Rule types are looked up by name in a registry populated by crates | Is there an `if rule == "..."` chain? Then it belongs in the registry |
| Hash-based determinism | Random values come from `rng_for(stage, id)`; ids derive from parents and keys | Any `thread_rng`, index-derived id, or iteration over a `HashMap` that affects output? Then reject |

Deliberately not used: entity component systems, async, trait-heavy abstraction before a
second concrete implementation exists, `unsafe`.

## Rust conventions

- Follow the Rust API Guidelines (rust-lang.github.io/api-guidelines) for naming, docs and
  error types.
- `rustfmt` defaults. `clippy::pedantic` enabled at the workspace level; individual lints may
  be allowed with a comment saying why.
- Errors: `thiserror` enums in library crates, `anyhow` only in `citymap-cli`. No panics on
  bad input; `unwrap` and `expect` only on invariants, with a message naming the invariant.
- Every public item has a doc comment. `#![warn(missing_docs)]` in library crates.
- No `unsafe`. No `async`. No generics or lifetimes in public core APIs unless the PR
  explains the need; own your data (`String`, `Vec`) at boundaries.
- Collections: `Vec` in canonical order or `BTreeMap`/`BTreeSet` wherever iteration can
  touch output. `HashMap` only for lookups whose order never leaks.
- Floats: `f64` everywhere; comparisons via quantised integers in topology; trig via `libm`.
- Modules: one module per stage in core (`site`, `macro_layout`, `landmarks`, `streets`,
  `blocks`, `buildings`, `labels`, `decorators`) plus `spec`, `model`, `id`, `rng`, `rules`.
- Tests next to the code for units and properties; `tests/` for cross-crate and golden
  snapshots (`insta`). Snapshot files are reviewed in the diff.
- Dependencies: add one only with an ADR line; `cargo deny` checks licenses and duplicates.

## Review hygiene

- Conventional commit subjects: `feat(streets): ...`, `fix(rng): ...`, `docs: ...`,
  `test(macro): ...`, `chore: ...`. Body explains why and names the tier or milestone.
- One concern per commit; a refactor and a behaviour change never share one.
- Non-obvious Rust idioms are explained in the commit body or a code comment, since the
  reviewer is learning the language.
- "Boring over clever": the reviewer rejects code they cannot read in one pass, even if
  it is correct.
- Snapshot updates are a separate commit with the reason in the subject.

## Codebases worth reading for shape

- `resvg` / `usvg`: a parse-into-typed-tree then render pipeline in Rust, no DOM.
- `ripgrep`: how a Rust CLI crate wraps pure library crates.
- `tiny-skia`: small, pure, well-documented geometry and raster code.
