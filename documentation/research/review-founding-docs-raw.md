# Founding-document review — CityMapMaker

Reviewed 2026-10-19 against commit `5d5cb62`. Files: `CLAUDE.md`, `docs/REQUIREMENTS.md` (REQ),
`docs/ARCHITECTURE.md` (ARCH), `docs/DECISIONS.md` (ADR), `docs/CODING_STANDARDS.md` (CS),
and the four `docs/research/*.md` reports (context only). Line numbers are from that commit. Since #113 the folder is `documentation/`.

Standard applied: every rule says what, who decides, and what passes; no hedging; every
decision names its alternatives. Items below are where the documents fall short of that.

---

## 1. MISSING

1. **Hard-constraint failure behaviour.** REQ L102 (F8) "reports failure" and ARCH L137
   "a reported failure" never say what the user gets: non-zero exit with no files, or files
   plus a diagnostics list? Must `--write-back` still run? Which landmark, which constraint,
   which candidates were tried? Decide and write it next to F8.
2. **Error handling end to end.** CS L39 picks `thiserror`/`anyhow`; ARCH L113 names
   `StageError`. Nothing says the shape of a failure (structured data vs. message), how
   stage errors compose into one `GenerateError`, or whether warnings (Q5 legibility
   guard, REQ L142) are a separate channel from errors.
3. **CLI exit codes.** REQ §5.4 (L124-127) defines commands, not exit codes. Needed:
   success, invalid spec, constraint failure, I/O error, at minimum.
4. **Logging.** CLAUDE L66-67 permits `tracing` events in core; no document says who
   subscribes, which flag sets verbosity, that output goes to stderr, or that tracing
   output is excluded from the byte-identical claim.
5. **Spec schema versioning.** ADR-002 L18-19 says the schema "must be versioned"; REQ L69
   and L84 speak of "version 1" and "version 2". No field name, no policy for an old spec
   (reject vs. migrate), no rule on whether `--write-back` stamps the version. The pack
   file also has `version: 1` (ARCH L146) with no stated meaning.
6. **Output versioning.** F15 model JSON (REQ L118) has no version field; Foundry scene
   JSON (F16) names no Foundry version (research-rendering L26 says v13). Goldens and
   later tools depend on both.
7. **Golden snapshot storage and review mechanics.** ARCH L212-213, CS L49-50 and L63 say
   snapshots are reviewed; not stated: directory, size policy for SVG snapshots, whether
   PNGs are ever snapshot-tested (recommend: never, say so), the command
   (`cargo insta review` vs. `INSTA_UPDATE=`), and that Jeffrey is the one who accepts.
8. **CI definition.** ARCH L220 "CI runs fmt, clippy..." and milestone 0 (ARCH L230)
   mention CI; no provider, no OS matrix (the "any machine" claim at ARCH L10 is untested
   unless Linux is in the matrix), no statement of what blocks a merge. `.github/` does
   not exist.
9. **Contribution flow.** CLAUDE L27 "PR or commit text", CS L42 "unless the PR explains",
   CS L56 commit bodies. The git log shows direct commits to `main`. Decide: branch per
   step or direct to main; who commits; review before or after commit; who merges.
10. **Release and version numbers for the tool itself.** None. Say either "no releases,
    `main` is the product, crate versions stay 0.1.0" or define tags per tier.
11. **Research to decision process.** CLAUDE L16 "reference, not gospel". No rule for
    promoting a research claim into an ADR, or for marking a report obsolete. Two reports
    were written for a TypeScript target (research-determinism L15, research-rendering L3)
    and list npm libraries; an AI pair will read `roughjs`/`rbush` as recommendations.
    Add a header line to each report: "predates ADR-003; library tables do not apply".
12. **Glossary.** Undefined and overlapping: *step* (CLAUDE L24, L90) vs *milestone*
    (ARCH §9) vs *tier* (REQ §9) vs *stage* (ARCH §4); *cell* vs *ward* vs *district* vs
    *block* vs *parcel* (ARCH L89, L104-109, L139); *preset* (style preset, era preset,
    and "the preset's area per person" at REQ L269); *rule* vs *evaluator* vs *constraint*
    vs *pack stage* (ARCH §6). One table, one definition each.
13. **Untrusted spec and pack files.** Specs are AI-written and will be copied around.
    Nothing says: no YAML anchors/aliases or custom tags, size limits on lists, that
    `serde_yaml` is archived upstream (2024) and which fork is used. Packs are data plus
    compiled evaluators, so a pack cannot run code; state that explicitly.
14. **Performance budget scope.** Q2 (REQ L134-136) gives whole-pipeline budgets only, and
    does not say release or debug build (10x difference). `sweep` (F19) has no budget.
    Property tests on a full `generate` at 5 s x 256 cases is too slow; say property
    tests run on toy specs and name the case count.
15. **Vault and repo sync.** CLAUDE L18-20 makes the vault the truth; REQ §8 copies vault
    facts into this repo and `specifications/*.yaml` will carry more. No rule for: who updates
    REQ §8 and the spec when the vault changes; where a GM? answer is recorded first
    (vault, then here?); whether private campaign lore may live in a public MIT repo
    (ADR-012). Jeffrey decides the last one; it is not asked anywhere.
16. **`--write-back` semantics.** F7/F17: rewriting a YAML file. serde does not preserve
    comments, key order or formatting. Decide: lossy round-trip, a separate `*.resolved.yaml`,
    or targeted patching of only `resolved`/`pinned` keys.
17. **Seed type and precedence.** Seed is in the spec (REQ L70) and on the CLI (REQ L125);
    type (u64? string hashed as research-determinism L13 suggests?) and precedence are
    unstated. See Contradictions 2.
18. **What constraints may reference.** F3 `near`, `in_district`, `far_from` need a
    target. If a target may be a generated id (a street, a block), a stage algorithm
    change silently breaks specs. State: constraint targets are spec ids only, never
    generated ids.
19. **Which artefacts are byte-identical.** ARCH L10 and Q1 (REQ L131) say "output";
    PNG bytes depend on resvg/png-encoder versions. State: model JSON and SVG are
    byte-identical; PNG is not a determinism artefact.
20. **Pack and style file resolution.** The spec names packs and styles (REQ L83); not
    stated whether they are looked up in `packs/` relative to cwd, relative to the spec
    file, or embedded in the binary (`include_str!`).
21. **Parallelism in the shell.** Core has no threads (CLAUDE L66). Nothing says whether
    `sweep` may run seeds in parallel in the CLI. Say yes or no.
22. **`analysis/` environment.** No `requirements.txt`/`pyproject`; no Python version.
    One line suffices.
23. **Id collision policy.** `Id` is a 64-bit hash (ARCH L96-97). Not stated: whether the
    object kind is part of the hash, and whether a duplicate id at a stage boundary is an
    error, an assertion, or ignored.

## 2. REMOVE OR SHRINK

1. **ARCH §2 crate list (L26-44)** names seven crates before milestone 0, two marked
   "later". Keep `core`, `render`, `cli`; add the others in the milestone that creates
   them. Rots otherwise.
2. **ARCH §7 candidate crate list (L194-198)** duplicates ADR "Open decisions" L105-106.
   Keep it in one place (ADR).
3. **Tier-to-milestone mapping** appears in REQ §9 (L266-277), ARCH §9 (L228-241, tier in
   the first column) and ARCH L223-225 prose. One table, one file.
4. **Hard rules stated three to four times.** Determinism: CLAUDE rule 2, ARCH §5, CS L17,
   CS L44-46, ADR-004. Comments: CLAUDE rule 7 (L81-84) and CS L21-31 near verbatim.
   Purity: CLAUDE rule 1, ARCH principle 2, CS L11. Decide one owner (CS for code rules,
   ARCH for invariants) and make CLAUDE.md a pointer list.
5. **ARCH §3 Rust struct code (L52-94)** is detailed code before milestone 1 and its own
   header says "the definitions in code win". Shrink to a field-name list now and delete
   after milestone 1 in favour of a pointer to `citymap-core::spec`.
6. **ARCH §6 pack YAML example (L143-158)** invents `selector`, `from: district.compass`
   and a `stages:` block with no decision behind them and no need before milestone 8.
   Mark "illustration, not a format" or remove.
7. **ARCH L187-189 label placement algorithm** (greedy eight-position, R-tree, simulated
   annealing) is research-rendering §1 restated as design before milestone 7. Move to open
   decisions with a deadline.
8. **Hedged library choices inside the architecture:** ARCH L122-123 "SmallRng-like",
   "rand_pcg or rand_xoshiro"; L108 "straight-skeleton or recursive split"; L198
   "ttf-parser or fontdue". Either decide (ADR) or list under open decisions; not in the
   target-shape text.
9. **REQ §8 calibration briefs (L161-257)** are a dated copy of vault lore. After
   milestone 1 the spec file is the brief; the section should shrink to source pointers,
   the generator-relevant geometry facts, and the GM? list.
10. **Research reports' obsolete sections:** research-determinism L58 has an S6
    "labels/names" stage with a names RNG, contradicting ADR-009; research-rendering §1
    is an npm stack table. Do not edit the reports; add the "predates ADR-003" header
    (Missing 11).
11. **Temporal phrasing that rots:** REQ L21 "sessions run online today"; REQ L3 and
    ARCH L3 "v0.1" statuses with no rule for when they change.

## 3. CONTRADICTIONS

1. **Pins vs. hard site rules.** CLAUDE L74-75 and ARCH L134-137: a pinned placement is
   copied untouched, never recomputed. REQ L30-31 (G2): no building in water or on a cliff
   face; REQ L271 (T3): every adjacency constraint satisfied. A pinned landmark after the
   site changes (Truro's river, REQ L173-175, is exactly this case) can violate G2. Which
   wins, and is it an error or a warning?
2. **Where the seed lives.** REQ L70 puts `seed` in the spec identity; REQ L125 has
   `--seed N`; ARCH L8-9 "the spec is the state", yet `generate(spec, seed)` takes the
   seed separately (CLAUDE L67, ARCH L9). Precedence undecided.
3. **"On any machine" vs. one toolchain.** ARCH L10 claims byte-identical output on any
   machine; CLAUDE L52 and ADR-003 target Windows MSVC only; Q1 (REQ L131) tests
   determinism by generating twice on one machine. The claim is untested as written.
4. **Locality unit: grid cell or Voronoi ward?** ARCH L138-139 and REQ L33-35 define
   locality by "cell and one-ring". ARCH L105 makes S1 a Voronoi ward mesh and keys S1/S3
   RNG by `ward_id`, S0 by `cell`. Voronoi with Lloyd relaxation (the MFCG method,
   research-tools §1) is global; inserting a site moves non-neighbour wards. Either no
   relaxation, or locality is defined on the grid and wards are derived per cell.
5. **Tier order vs. milestone order.** REQ L271 T3 is "T2 plus districts" (cumulative);
   ARCH L233 delivers T3 at milestone 3 and T2 at milestone 4.
6. **Harness timing.** REQ L150 "expressive range harness from the first milestone";
   ADR-008 L71 "exist by milestone 5"; ARCH L235 milestone 5.
7. **Optimisation rule vs. planned optimisation.** Q2 (REQ L135) "no optimisation work is
   done while these limits hold"; ARCH L117-118 plans incremental recomputation "once
   stages are stable". Who may schedule it and under which rule?
8. **Trait abstraction before a second implementation.** CS L19-20 forbids trait-heavy
   abstraction before a second concrete implementation; CS L42-43 forbids generics in
   public core APIs without justification; ARCH L75-76 `Box<dyn EvaluatorParams>` and L160
   `trait Evaluator` plus a registry are designed before the first evaluator exists.
9. **`write-back` is a command (ARCH L34) and a flag (REQ L125).**
10. **CLI surface vs. requirements.** F13 "PNG at a chosen DPI" and Q5 `--scale 1:N` have
    no flag in F17 (REQ L125); `--png` has no DPI argument.
11. **"Pluggable" rule packs (G5, ADR-006 "own crates") vs. F4 "unknown rule types are an
    error".** Every evaluator crate must be linked into the one binary; packs are
    data-pluggable, not binary-pluggable. ARCH L157 "registered in citymap-rules-pnw"
    implies otherwise.
12. **No-comment rule vs. lint allows.** CLAUDE L81-82 "allowed only when naming cannot
    make the code self-explanatory"; CS L37-38 "individual lints may be allowed with a
    comment saying why". Both are exceptions to one absolute sentence (CS L24); write one
    rule with its exhaustive exception list.
13. **"No UI work before milestone 7" (ADR-002 L18)** while ARCH §9 has no UI milestone at
    all; and ARCH L47-48 "depends on cli-level code or on core" leaves the UI's dependency
    undecided.
14. **CLAUDE L13 says ARCHITECTURE contains a "command model"**; ARCHITECTURE has no such
    section, only principle 3 (L10-13).

## 4. AMBIGUOUS

1. CLAUDE L24-25 "One milestone step per session": *step* undefined; ARCH L223 says a
   milestone is "one or two sessions". Define step = one reviewed commit series ending in
   a green `cargo test`, or similar.
2. CLAUDE L27 "PR or commit text": pick one (see Missing 9).
3. CLAUDE L28 "anything beyond a trivial fix": define trivial (e.g. no behaviour change,
   no new public item, under N lines). "get a yes" names no one; say "from Jeffrey".
4. CLAUDE L30-31 "materially different code": acceptable as a judgement call only if the
   fallback is stated: "when unsure, ask".
5. REQ L102 / ARCH L136 "searches locally", "bounded ... spiral search": bound value and
   owner (constant in core, spec field, pack parameter?) unstated.
6. REQ L136 Q2 "on Jeffrey's machine": release or debug build.
7. ARCH L117 "once stages are stable": who declares a stage stable, and by what test.
8. ARCH L164 "merge by rule id": later pack overrides, or duplicate id is an error?
9. ARCH L50 "this section is updated to match them": by whom and when; say "in the same
   commit as the type change".
10. ARCH L166 "published in `documentation/`": file name, generated at build or committed, and
    whether a test checks it is current.
11. ARCH L47-48 "depends on `cli`-level code or on core": undecided "or".
12. ARCH L172-174 "rarely", "seldom changed": fine as description; L173 "reviewed by
    Jeffrey" is the only line in that table that names a reviewer; the other two rows
    should too.
13. CS L37-38 "may be allowed": who approves a lint allow (Jeffrey, in review) and where it
    lives (per-crate `lib.rs` vs `[workspace.lints]`).
14. REQ L269 "the preset's area per person": which preset holds this number, and who sets
    it.
15. ADR L107-108 open decision on metric comparison has no owner or deadline; the other
    open decision (L105-106) has both.
16. CLAUDE L94 "reviewed diff": who reviews (Jeffrey) and in which tool.
17. REQ L17-18 "An AI assistant may write or edit specs": may it also commit them, and
    does a spec edit count as a step needing a plan (CLAUDE L28)?
18. ARCH L10 "byte-identical output": which files (see Missing 19).
19. ADR-006 L54 "packs have their own crates" and ARCH L31-32: does `citymap-rules-std`
    count as a pack, and is its YAML in `packs/` or implicit?

## 5. RUST-SPECIFIC RISKS

1. **Trait object inside the spec (ARCH L75-76).** `Spec` must be `Serialize`,
   `Deserialize`, `Clone`, `PartialEq`, `Debug` for `--write-back`, schemars, the
   round-trip property test (ARCH L211) and insta. `Box<dyn EvaluatorParams>` derives none
   of these; `typetag` fixes serde but relies on `inventory`, which does not work on
   `wasm32-unknown-unknown` (breaks the Q3 purity check), and schemars cannot describe it.
   Recommendation: `Spec` stays plain data; constraint params are a closed
   `enum ConstraintParams` with one variant per F3 rule (ten variants), serde-tagged by
   `rule`. Packs add attribute stages, not new constraint kinds, until a second setting
   needs one; then revisit with an ADR. Keep a separate non-serialised `ValidatedSpec`
   only if the enum proves insufficient.
2. **`serde_yaml::Value` in core (ARCH L74).** Ties core to an archived crate and to YAML.
   Recommendation: core never sees YAML; `citymap-cli` parses YAML to `serde_json::Value`
   (or straight to typed structs) and core types derive serde only. Record the YAML crate
   choice (`serde_yaml_ng`, `serde_yml`, or `serde_yaml` pinned) in an ADR.
3. **"Registry populated by crates" (CS L16, ARCH L19).** Reads as link-time registration
   (`inventory`/`linkme`): non-deterministic order, no WASM, ambient global state.
   Recommendation: explicit construction in the shell,
   `Registry::new().with(citymap_rules_std::evaluators())`, stored in `Ctx` as a
   `BTreeMap<RuleName, Box<dyn Evaluator>>`. Also decide the `candidate` type in
   `Evaluator::score` (ARCH L160): one trait per stage or a `Candidate` enum; a single
   untyped trait fights the "parse, don't validate" rule.
4. **Index-based geometry libraries vs. "never an index" (ARCH L128).** `spade`, `geo`
   and any half-edge mesh hand out indices/handles. Recommendation: write the invariant
   as "indices may exist inside one stage function; every stage output carries ids only";
   sort inputs by id before insertion (already stated, ARCH L130); add a debug assertion
   that no two objects in a stage output share an id.
5. **f64 determinism details.** `+ - * / sqrt` are IEEE-exact on x86_64 and wasm32; Rust
   does not contract to FMA unless `mul_add` is called; `libm` is pure Rust and
   bit-stable. Remaining holes: `f64::sin/cos/atan2/exp/ln/powf/hypot` methods call the
   platform libm; `partial_cmp().unwrap()` panics on NaN; `-C target-cpu=native` in a
   local `.cargo/config.toml`. Recommendation: clippy `disallowed_methods` for the std
   float transcendental methods, `clippy::float_cmp` denied, `f64::total_cmp` for sorts,
   NaN rejected at the spec boundary, and a committed `.cargo/config.toml` with no
   `target-cpu`. Implement "fixed decimal precision" (ARCH L95) as a `Metres` newtype with
   a custom `Serialize`, not as ad-hoc rounding.
6. **`rand` algorithms are not stable across versions.** `SmallRng` is documented as
   subject to change; `gen_range` on floats and `Uniform` changed between rand 0.8 and
   0.9. Every golden snapshot depends on them. Recommendation: name the concrete generator
   (`Pcg64Mcg` or `Xoshiro256**`) in an ADR, pin the exact `rand` version, and implement
   the two distributions needed (uniform f64 from 53 bits, uniform integer range) in
   `citymap-core::rng` so goldens survive a `rand` upgrade.
7. **Top-level `tests/` in a virtual workspace (ARCH L40, CS L49).** `cargo test
   --workspace` does not run a `tests/` directory at the root unless the root is a
   package. Recommendation: integration and golden tests live in
   `crates/citymap-cli/tests/` or a dev-only `crates/citymap-tests/` crate; say which.
8. **`citymap-rules-std` as a separate crate (ARCH L31).** F1/F4 validation needs the
   registry; core cannot validate without linking rules-std, which depends on core.
   Recommendation: `citymap-core::rules::std` until milestone 8 creates the first real
   external pack; consistent with CS L19-20.
9. **`clippy::pedantic` + `-D warnings` + `--all-targets` + no comments.** Pedantic fires
   `cast_precision_loss`, `cast_possible_truncation`, `must_use_candidate`,
   `missing_panics_doc`, `module_name_repetitions` constantly in geometry and test code,
   and the comment rule forbids the usual `// reason` next to `#[allow]`. Recommendation:
   `[workspace.lints.clippy] pedantic = "warn"` plus an explicit allow list decided once
   in an ADR, applied in `Cargo.toml`, never per site.
10. **Field-naming validation errors (F1).** serde errors on nested or `untagged` enums do
    not name the path. Recommendation: `serde_path_to_error`, internally tagged enums only,
    `deny_unknown_fields` on every spec struct, and a unit test per error message.
11. **Structured stage errors.** `Result<Output, StageError>` with `thiserror` (ARCH L113,
    CS L39): make the constraint-failure variant carry data (landmark id, rule name,
    candidates tried) so tests assert on it and the CLI formats it; `Display` is derived
    from that data, not the other way round.
12. **PNG and SVG snapshots.** PNG bytes change with every resvg/tiny-skia/png release;
    SVG goldens with multi-stroke wobble are large. Recommendation: goldens are model JSON
    for all stages, SVG for the three calibration cities only, PNG never; `.gitattributes`
    already pins LF, keep that for `.snap` files.
13. **`tracing` in a pure core.** Zero cost without a subscriber and fine, but field
    expressions must not call non-deterministic code and are excluded from the
    byte-identical claim; write that sentence (Missing 4).

## 6. KEEP

1. **CS L8-17, the pattern table with a one-line reviewer test per pattern.** This is the
   most useful page in the repo: it turns taste into yes/no questions. Extend it; never
   collapse it into prose.
2. **REQ §9 review tiers (L259-281): Judge and Ignore columns, "only Jeffrey passes a
   tier", "pretty is not a criterion before T6".** Together with the GM? convention
   (REQ L4, L158-159) and ADR-009, this is what keeps early reviews honest and lore
   un-invented.
3. **ADR entries that name rejected alternatives (ADR-003, ADR-012) and the inline
   "Rejected alternative" at ARCH L91-93**, plus ARCH §5 determinism design mapped
   one-to-one onto the property tests in ARCH §8 item 2. The invariant-to-test mapping
   is the project's real specification; any cleanup must preserve it verbatim.
