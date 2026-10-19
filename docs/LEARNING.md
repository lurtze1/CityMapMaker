# Concepts and where to learn them

A primer for the terms the other documents use. Each entry says what the term means in this
project, why it matters here, and where to read more. The "read" links are the primary or
best-known source for each idea; none of them is required before milestone 0 except the Rust
Book chapters listed at the end.

## Determinism and randomness

**Seeded randomness.** A generator that produces random-looking numbers from a starting
number, the seed. Same seed, same sequence, forever. Our maps are "random" only in this
sense; nothing in the program ever asks the operating system for a random number.
Read: the `rand` book, chapter "Our RNGs", https://rust-random.github.io/book/guide-rngs.html

**Sequential versus hash-based streams.** A sequential generator hands out numbers in order;
whoever asks first gets the first number. Insert one extra ask anywhere and every later
number shifts, so adding one landmark reshuffles the whole city. A hash-based stream instead
computes each object's numbers from `hash(seed, stage, object id)`; the cathedral's numbers
do not depend on whether the market asked before it. ARCHITECTURE.md §5 and ADR-018.
Read: Squirrel Eiserloh, "Math for Game Programmers: Noise-Based RNG", GDC 2017,
https://www.youtube.com/watch?v=LWFzPP8ZbdU ; Minecraft's per-chunk seeding is the same idea,
documented at https://minecraft.wiki/w/Seed_(level_generation)

**Locality.** The promise that a change in one place changes nothing far away. We get it by
cutting the site into grid cells and deriving everything inside a cell from that cell and its
eight neighbours only. Add a landmark in one cell and only that cell plus its ring can
change; the property test in ARCHITECTURE.md §8 checks it. The Voronoi method Watabou uses
breaks locality because its relaxation step moves every polygon when one site is added.
Read: the Godot Voxel Tools page explains the failure and the fix in two paragraphs,
https://voxel-tools.readthedocs.io/en/latest/procedural_generation/

**Stable ids.** Every generated object gets an identity computed from what it is made of
(its kind, its parents, its grid cell), not from its position in a list. A street keeps its
id across regenerations as long as its endpoints exist, so a label or a pin can point at it.
Index-based ids are the "topological naming problem" that CAD programs suffer from.
Read: https://wiki.freecad.org/Topological_naming_problem

**Pinning as resolved intent.** A landmark spec has an intent (constraints: "north-east, near
the market") and, once placed, a resolved placement (x, y). Pinning freezes the resolved
placement in the spec file so it becomes an input, not an output. The generator reads it and
builds around it. This is Lipp et al.'s "protected layer" idea applied to a spec file.
Read: Lipp, Scherzer, Wonka, Wimmer 2011, "Interactive Modeling of City Layouts using Layers
of Procedural Content", https://peterwonka.net/Publications/pdfs/2011.EG.Lipp.CityControl.final.pdf

**Deterministic floating point.** Addition, subtraction, multiplication, division and square
root give the same bits on every machine. Sine, cosine, exponent and logarithm do not; each
platform's maths library rounds differently. So we route those through `libm`, a pure-Rust
library with fixed results, and we never let a transcendental function decide a yes-or-no
question such as which side of a line a point is on.
Read: https://docs.rs/libm ; Bruce Dawson's "Floating-point determinism",
https://randomascii.wordpress.com/2013/07/16/floating-point-determinism/

## Rules and constraints

**Constraint, rule, evaluator, pack** are defined in the ARCHITECTURE.md glossary. In one
sentence: a constraint is a wish attached to a landmark; an evaluator is the code that
scores how well a candidate spot satisfies that wish; a pack is a YAML file of such wishes
for a whole setting.

**Hard versus soft constraints.** A hard constraint rejects a candidate outright ("must be
inside the walls"). A soft constraint adds a weighted score ("prefer near the market").
The generator takes the best-scoring candidate that no hard constraint rejected.
Read: any introduction to constraint satisfaction; the Wikipedia entry is adequate,
https://en.wikipedia.org/wiki/Constraint_satisfaction_problem

**Closed enum versus trait object.** Two Rust ways to say "one of several kinds". A closed
enum lists every kind in one place; the compiler knows them all, can serialise them, compare
them and generate a schema. A trait object (`Box<dyn Trait>`) lets any crate add a kind
later, at the cost of the compiler no longer knowing the full set, so serialisation and
comparison need extra machinery. ADR-017 picks the enum for constraint parameters because we
need serialisation more than we need open extension.
Read: Rust Book ch. 6 (enums) and ch. 18 (trait objects),
https://doc.rust-lang.org/book/ch06-00-enums.html ,
https://doc.rust-lang.org/book/ch18-02-trait-objects.html

## Architecture patterns

**Hexagonal architecture, also called ports and adapters.** The core of the program knows
nothing about files, terminals or screens. Those are adapters plugged in at the edge. Our
`citymap-core` is the hexagon; `citymap-cli` is an adapter.
Read: Alistair Cockburn's original article, https://alistair.cockburn.us/hexagonal-architecture/

**Functional core, imperative shell.** The same idea stated as a rule: all decisions happen
in pure functions (same input, same output, no side effects); all side effects happen in a
thin shell around them. Pure functions are trivial to test and are what makes determinism
possible.
Read: Gary Bernhardt, "Boundaries" (talk, 2012),
https://www.destroyallsoftware.com/talks/boundaries

**Parse, don't validate.** Turn untrusted input into a type that cannot represent an invalid
state once, at the boundary, and never check it again. A `Direction` enum cannot hold the
string "norht"; a `String` can.
Read: Alexis King, 2019, https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/

**Newtype.** A struct with one field whose only job is to give a plain number a type:
`Metres(f64)` cannot be added to `Pixels(f64)` by accident.
Read: Rust Book ch. 20, "Using the Newtype Pattern",
https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits

**Command pattern.** An edit is an object describing the edit ("pin landmark X"), applied by
one function, instead of code scattered across whatever UI produced it. ARCHITECTURE.md §3b.
Read: https://refactoring.guru/design-patterns/command

**Architecture decision record (ADR).** A short note per decision: context, decision,
alternatives rejected, consequences. `docs/DECISIONS.md` is a list of them.
Read: https://adr.github.io/

## Geometry

**Voronoi diagram and Delaunay triangulation.** Scatter points; the Voronoi diagram gives
each point the region closer to it than to any other point; the Delaunay triangulation
connects points whose regions touch. Watabou's city wards are Voronoi regions; our ward mesh
is too, with the points placed one per grid cell. Lloyd relaxation moves every point to the
centre of its region to even them out, which is why we do not use it (locality).
Read: Amit Patel, "Polygonal Map Generation for Games",
https://www.redblobgames.com/maps/mapgen/ ; the Delaunay and Voronoi pages on the same site.

**Direction fields (tensor fields) for streets.** Instead of drawing streets one by one, define
at every point which way a street "wants" to go (around the plaza, along the wall, along
the river), then trace lines that follow the field. Grid, radial and organic street patterns
are different fields.
Read: Chen, Esch, Wonka, Müller, Zhang 2008, "Interactive Procedural Street Modeling",
https://www.sci.utah.edu/~chengu/street_sig08/street_project.htm

**Least-cost path.** The primary road from a gate to the market is the route that minimises
a cost (distance plus slope plus water crossings). Dijkstra's algorithm on a grid.
Read: Amit Patel, "Introduction to A*", https://www.redblobgames.com/pathfinding/a-star/introduction.html

## Testing

**Property-based testing.** Instead of one hand-written input and expected output, state a
property that must hold for all inputs ("generating twice gives equal output") and let the
test tool generate hundreds of random inputs looking for one that breaks it. Our determinism,
pin and locality invariants are all properties.
Read: the proptest book, https://altsysrq.github.io/proptest-book/

**Golden (snapshot) testing.** Run the generator once, store the exact output in a file, and
fail the test when the output changes. The change is then reviewed as a diff and accepted or
rejected on purpose. Catches accidental changes to anything.
Read: https://insta.rs/docs/

**Expressive range analysis.** A way to see what a generator can and cannot produce: run it
thousands of times, measure a few numbers per output, and plot them. A generator that only
makes one kind of city shows up as a single blob. REQUIREMENTS.md §7.
Read: Smith and Whitehead 2010, "Analyzing the Expressive Range of a Level Generator",
https://users.soe.ucsc.edu/~ejw/papers/smith-pcg-2010.pdf

## Build tooling

**Lint.** An automated check on source code that flags patterns the compiler accepts but a
reviewer would reject, such as comparing two floats for equality. Rust has the compiler's
own lints and Clippy, a separate tool with several hundred more, grouped into `style`,
`complexity`, `perf`, `pedantic` (strict, off by default) and `restriction` (pick one by
one). Each lint has a level: `allow` is silent, `warn` prints, `deny` fails the build,
`forbid` fails and cannot be re-allowed lower down. ADR-019 sets ours.
Read: https://doc.rust-lang.org/clippy/ ; the lint list with examples,
https://rust-lang.github.io/rust-clippy/stable/index.html

**Workspace lints.** A `[workspace.lints]` table in the root `Cargo.toml` sets lint levels
once; each crate opts in with `[lints] workspace = true`. One place to read, one place to
change. `priority = -1` on a group means a single lint named later can override the group.
Read: https://doc.rust-lang.org/cargo/reference/workspaces.html#the-lints-table

**`disallowed_methods`.** A Clippy lint fed by `clippy.toml`: a list of method paths that
may not be called, each with a reason shown in the error. We use it to ban the std float
methods whose results differ across platforms and point to `libm` instead.
Read: https://rust-lang.github.io/rust-clippy/stable/index.html#disallowed_methods

## Learning Rust for this project

In order; each is free except the two books.

1. The Rust Book, https://doc.rust-lang.org/book/ . Chapters 1 to 10 before milestone 0;
   chapters 13 (iterators and closures), 17 and 18 (traits and trait objects) before
   milestone 1; chapter 11 (tests) before reviewing any PR.
2. Rustlings, https://github.com/rust-lang/rustlings , small exercises matched to the Book.
3. Rust by Example, https://doc.rust-lang.org/rust-by-example/ , for looking up syntax.
4. The Cargo Book, workspace chapter, https://doc.rust-lang.org/cargo/reference/workspaces.html
5. The Rust API Guidelines, https://rust-lang.github.io/api-guidelines/ , the checklist our
   reviews use.
6. serde, https://serde.rs/ , before milestone 1, since the spec is serde all the way down.
7. "Rust for Rustaceans" by Jon Gjengset (book), after milestone 3, for ownership in graph
   code and when to use trait objects.
8. Jon Gjengset's "Crust of Rust" videos, https://www.youtube.com/c/JonGjengset , the
   episodes on iterators, lifetimes and `dyn`.

Where a concept here is not in this list, add it to this file in the PR that introduces it.
