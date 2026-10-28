# CLAUDE.md — CityMapMaker

Specification-driven generator of fantasy **city overview maps** for Jeffrey's *Peace and War*
campaign (Pathfinder 2e). A written city specification goes in; an annotated, hand-drawn-looking map
(SVG, PNG, later Foundry VTT) comes out. Read this file fully, then the documents below in order.

## Documents

| File | Owns |
|---|---|
| `documentation/REQUIREMENTS.md` | Goals, non-goals, functional and quality requirements, review tiers, the city briefs, GM? questions |
| `documentation/ARCHITECTURE.md` | Glossary, principles, crate layout, types, command model, stage pipeline, logging, determinism invariants, rule packs, rendering, testing, build order |
| `documentation/CODING_STANDARDS.md` | Code rules: patterns with reviewer tests, comments, Rust conventions, review and issue hygiene |
| `documentation/DECISIONS.md` | Decision log, ADR style, each naming rejected alternatives. Every real choice gets an entry in the same commit |
| `documentation/LEARNING.md` | Primer: every concept the other documents use, what it means here, and where to learn it. Extended in the PR that introduces a new concept |
| `documentation/research/*.md` | Research reports. Binding only when a decision entry cites them; two predate the Rust decision |
| `data/research/` | Research facts as YAML, validated in CI, queried through generated SQLite (ADR-025) |

Each rule lives in exactly one of those files. This file points; it does not restate.

## The vault

Campaign lore lives in an Obsidian vault at
`D:\Google Drive\RPG DRIVE\Peace and War\Peace and War\` with its own CLAUDE.md. It is the
single source of truth for names, landmarks and geography. This repo never invents lore: a
missing fact becomes a GM? question in REQUIREMENTS.md. A GM? answer is written here first
and into the vault note in the same session. City specifications are written from the vault
notes and committed under `specifications/`, next to `example.yaml` (ADR-054).

## Working agreement

- **Pace.** Months, not an afternoon. Designs come in batches and independent issues run on
  parallel branches (ADR-053). A step is defined in the ARCHITECTURE.md glossary. Jeffrey reviews and merges every PR (ADR-014); he is learning
  Rust, so diffs stay small and idioms are explained in the PR body.
- **Seven stages per issue** (ADR-040): 1. Claude picks up the issue and discusses it with
  Jeffrey, ending with the ten-line design and his yes; 2. Claude writes the code; 3. Claude
  reviews it, and Claude starts one agent that gets the diff and the documents, never
  Claude's conclusions; 4. Claude cuts as much code as possible while keeping the core
  functionality; 5. Claude reviews again; 6. Claude and Jeffrey review it in the conversation,
  and Jeffrey decides on the cuts; 7. Claude opens the PR. CodeRabbit is optional: only
  Jeffrey asks it for a review.
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
  Exempt: research reports under `documentation/research/` that report outside sources keep the
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
cargo run -p citymap-command-line -- generate specifications/example.yaml --seed 1 --output output/
```

## Hard rules, one line each; the owning document has the full text

1. Core is pure: no I/O, clock, environment or threads. ARCHITECTURE.md §1, CODING_STANDARDS.md patterns.
2. No ambient randomness; hashed per-object streams only. ARCHITECTURE.md §5.
3. Stable identifiers, never indices, in every stage output. ARCHITECTURE.md §5.
4. Pins are inputs; landmark N+1 never moves 1..N; a pin that breaks a hard rule is a failure. REQUIREMENTS.md F7, F8.
5. Deterministic floats: quantised topology, `libm` for transcendentals, banned std methods; geometry only through the `geometry` facade; same machine and toolchain give the same output (ADR-020). CODING_STANDARDS.md Rust conventions.
6. Rules are data; evaluators are compiled; constraint kinds are a closed enum. ARCHITECTURE.md §6, ADR-017.
7. No inline comments except a why where naming cannot; doc comments on every public item. CODING_STANDARDS.md Comments.
8. A UI only sends commands. ARCHITECTURE.md §3b.
9. Every stage has unit, property and golden tests. ARCHITECTURE.md §8.

## Definition of done for a step

- `cargo fmt --check`, `clippy -D warnings`, `cargo test` clean on both CI platforms.
- New behaviour has unit, property and golden tests as ARCHITECTURE.md §8 requires.
- Snapshot changes are a separate commit, accepted by Jeffrey with `cargo insta review`.
- A step that changes anything visual ends with a PNG in `output/` that Jeffrey looked at.
- `documentation/DECISIONS.md` has an entry for every choice made; ARCHITECTURE.md matches the code.
- The PR body names the issue it closes.
- The branch is rebased on `main`; Jeffrey merges with a squash merge (ADR-041).

## Licensing

MIT OR Apache-2.0 (ADR-012). Watabou's TownGeneratorOS is GPL-3.0 and ProbableTrain's
generator is LGPL-3.0: ideas are re-implemented, code is never copied from them.
