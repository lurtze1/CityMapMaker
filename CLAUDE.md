# CLAUDE.md — CityMapMaker

Spec-driven generator of fantasy **city overview maps** for Jeffrey's *Peace and War* campaign
(Pathfinder 2e). A written city spec goes in; an annotated, hand-drawn-looking map (SVG, PNG,
later Foundry VTT) comes out. Read this file fully, then the documents below in order.

## Documents

| File | Owns |
|---|---|
| `docs/REQUIREMENTS.md` | Goals, non-goals, functional and quality requirements, review tiers, the city briefs, GM? questions |
| `docs/ARCHITECTURE.md` | Glossary, principles, crate layout, types, command model, stage pipeline, logging, determinism invariants, rule packs, rendering, testing, build order |
| `docs/CODING_STANDARDS.md` | Code rules: patterns with reviewer tests, comments, Rust conventions, review and issue hygiene |
| `docs/DECISIONS.md` | Decision log, ADR style, each naming rejected alternatives. Every real choice gets an entry in the same commit |
| `docs/LEARNING.md` | Primer: every concept the other documents use, what it means here, and where to learn it. Extended in the PR that introduces a new concept |
| `docs/research/*.md` | Research reports. Binding only when a decision entry cites them; two predate the Rust decision |
| `data/research/` | Research facts as YAML, validated in CI, queried through generated SQLite (ADR-025) |

Each rule lives in exactly one of those files. This file points; it does not restate.

## The vault

Campaign lore lives in an Obsidian vault at
`D:\Google Drive\RPG DRIVE\Peace and War\Peace and War\` with its own CLAUDE.md. It is the
single source of truth for names, landmarks and geography. This repo never invents lore: a
missing fact becomes a GM? question in REQUIREMENTS.md. A GM? answer is written here first
and into the vault note in the same session. Real-city specs live in the vault under
`Maps/specs/` and are git-ignored here; only `specs/example.yaml` is committed (ADR-016).

## Working agreement

- **Pace.** Months, not an afternoon. One step per session: plan, implement, test, open the
  PR, stop. A step is defined in the ARCHITECTURE.md glossary. Jeffrey reviews and merges
  every PR (ADR-014); he is learning Rust, so diffs stay small and idioms are explained in
  the commit body.
- **Plan before code.** Only a trivial fix, as CODING_STANDARDS.md "Review hygiene" defines
  it, is made without a plan; everything else gets a ten-line design and a yes from Jeffrey
  first.
- **Ask about design, decide trivia.** Ask when two readings of a request lead to different
  code; when unsure whether they do, ask. Never ask about naming, formatting or choices the
  docs already settle.
- **Explicit over implied, always.** Jeffrey does not read between the lines and does not
  want to. Rules say do and do not; no "usually", "should consider", "where appropriate".
  Optional things say "optional" and who decides. Decisions name rejected alternatives.
  Never act on an assumption; ask it as a question, with options, and wait for Jeffrey's
  answer. Say "I do not know" instead of hedging. One question at a time, options listed.
  Exempt: research reports under `docs/research/` that report outside sources keep the
  uncertainty of those sources ("about", "weak source"), because removing it would
  misrepresent them (ADR-023). Anything that turns research into a decision (ADRs, the
  cross-analysis, rule packs) follows this rule in full.
- **Voice input.** Jeffrey dictates (Wispr Flow). Proper nouns arrive garbled; check them
  against the vault and ask when unsure. Never silently "fix" a word.
- **Report honestly.** Failing tests are shown with their output. A skipped step is named.
- **Issues.** One GitHub issue per smallest testable piece (CODING_STANDARDS.md, Review
  hygiene).

## Toolchain

Rust stable, native Windows MSVC, GitHub Actions CI on Windows and Linux (ADR-013). Python 3.12
only under `analysis/`. Commands once the workspace exists:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p citymap-cli -- generate specs/example.yaml --seed 1 --out out/
```

## Hard rules, one line each; the owning document has the full text

1. Core is pure: no I/O, clock, environment or threads. ARCHITECTURE.md §1, CODING_STANDARDS.md patterns.
2. No ambient randomness; hashed per-object streams only. ARCHITECTURE.md §5.
3. Stable ids, never indices, in every stage output. ARCHITECTURE.md §5.
4. Pins are inputs; landmark N+1 never moves 1..N; a pin that breaks a hard rule is a failure. REQUIREMENTS.md F7, F8.
5. Deterministic floats: quantised topology, `libm` for transcendentals, banned std methods; geometry only through the `geom` facade; same machine and toolchain give the same output (ADR-020). CODING_STANDARDS.md Rust conventions.
6. Rules are data; evaluators are compiled; constraint kinds are a closed enum. ARCHITECTURE.md §6, ADR-017.
7. No inline comments except a why where naming cannot; doc comments on every public item. CODING_STANDARDS.md Comments.
8. A UI only sends commands. ARCHITECTURE.md §3b.
9. Every stage has unit, property and golden tests. ARCHITECTURE.md §8.

## Definition of done for a step

- `cargo fmt --check`, `clippy -D warnings`, `cargo test` clean on both CI platforms.
- New behaviour has unit, property and golden tests as ARCHITECTURE.md §8 requires.
- Snapshot changes are a separate commit, accepted by Jeffrey with `cargo insta review`.
- A step that changes anything visual ends with a PNG in `out/` that Jeffrey looked at.
- `docs/DECISIONS.md` has an entry for every choice made; ARCHITECTURE.md matches the code.
- The PR body names the issue it closes.

## Licensing

MIT OR Apache-2.0 (ADR-012). Watabou's TownGeneratorOS is GPL-3.0 and ProbableTrain's
generator is LGPL-3.0: ideas are re-implemented, code is never copied from them.
