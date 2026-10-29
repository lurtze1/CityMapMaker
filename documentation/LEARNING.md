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
computes each object's numbers from `hash(seed, stage, object identifier)`; the cathedral's numbers
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

**Stable identifiers.** Every generated object gets an identity computed from what it is made of
(its kind, its parents, its grid cell), not from its position in a list. A street keeps its
identifier across regenerations as long as its endpoints exist, so a label or a pin can point at it.
Index-based identifiers are the "topological naming problem" that CAD programs suffer from.
Read: https://wiki.freecad.org/Topological_naming_problem

**Pinning as resolved intent.** A landmark specification has an intent (constraints: "north-east,
near the market") and, once placed, a resolved placement (x, y). Pinning freezes the resolved
placement in the specification file so it becomes an input, not an output. The generator reads it
and builds around it. This is Lipp et al.'s "protected layer" idea applied to a specification file.
Read: Lipp, Scherzer, Wonka, Wimmer 2011, "Interactive Modeling of City Layouts using Layers of
Procedural Content", https://peterwonka.net/Publications/pdfs/2011.EG.Lipp.CityControl.final.pdf

**Deterministic floating point.** Addition, subtraction, multiplication, division and square
root give the same bits on every machine. Sine, cosine, exponent and logarithm do not; each
platform's maths library rounds differently. So we route those through `libm`, a pure-Rust
library with fixed results, and we never let a transcendental function decide a yes-or-no
question such as which side of a line a point is on.
Read: https://docs.rs/libm ; Bruce Dawson's "Floating-point determinism",
https://randomascii.wordpress.com/2013/07/16/floating-point-determinism/

**SplitMix64 and hash mixing.** `mix_seed` is the SplitMix64 finaliser: three shift-xor-multiply
steps that scramble 64 bits so that inputs differing in one bit give unrelated outputs. We
use it as a hash: `Identifier`, the stage salt and the stream seed all feed their parts through it
one 64-bit word at a time. Every text key goes in with its length first; without that, the
keys `"ab", "c"` and `"a", "bc"` would feed the same bytes and get the same identifier (ADR-018,
ADR-030).
Read: Steele, Lea and Flood 2014, "Fast Splittable Pseudorandom Number Generators",
https://doi.org/10.1145/2714064.2660195 ; Vigna's reference code,
https://prng.di.unimi.it/splitmix64.c

**PCG (`Pcg64Mcg`).** The generator behind every random stream: a 128-bit state multiplied
by a fixed constant per draw, then a "random rotation" step that turns the state into 64
output bits. Fast, small, statistically strong, and a published fixed algorithm. The crate
version is pinned exactly (`=0.10.2`) because a changed algorithm would change every map
(ADR-018, ADR-032).
Read: Melissa O'Neill, https://www.pcg-random.org/ and the paper linked there.

**Uniform decimal numbers from 53 bits.** An `f64` holds 53 significant bits. Taking the top
53 bits of a 64-bit draw and multiplying by 2^-53 gives every multiple of 2^-53 from 0 up to
but not including 1, each exactly once and equally likely, with no rounding. Using all 64
bits or dividing by 2^64 - 1 instead rounds some values together and can even return 1.0
(ADR-033).
Read: Vigna, "Generating uniform doubles in the unit interval",
https://prng.di.unimi.it/ (section "Generating uniform doubles in the unit interval")

**Unbiased whole numbers in a range (modulo bias).** `x % 6` on a random 64-bit `x` looks
fair but is not: 2^64 is not a multiple of 6, so 0 to 3 come up one time more than 4 and 5
out of every 2^64 draws. For large ranges the difference becomes large. Lemire's method
multiplies the draw by the range, keeps the high half, and throws away the rare draws that
land in the uneven remainder, so every value is exactly equally likely (ADR-033).
Read: Lemire 2019, "Fast Random Integer Generation in an Interval",
https://arxiv.org/abs/1805.10941

**Negative zero and `total_cmp`.** Floating point has two zeros, `0.0` and `-0.0`; `==` says
they are equal, but they have different bits, so they serialise differently. `f64::total_cmp`
gives every float, including `-0.0` and NaN, one fixed place in a single order, which is what
sorting and stable output need. `SignedLength` compares with `total_cmp` and stores every
"rounds to zero" value as `0.0`, so equal lengths have equal bits (#82).
Read: https://doc.rust-lang.org/std/primitive.f64.html#method.total_cmp

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

**Internally tagged enum, `deny_unknown_fields`, `flatten`.** Three serde attributes the
specification reader relies on. An internally tagged enum (`#[serde(tag = "rule")]`) reads a map and
uses one key, here `rule: near`, to pick the variant. `deny_unknown_fields` turns any key
the type does not know into an error, so a typo like `targt` fails instead of being
silently ignored. `flatten` would let one struct absorb another's fields, but it cannot be
combined with `deny_unknown_fields`: a scratch test on 2026-10-20 showed the combination
rejects every valid rule. That is why each constraint variant carries `hard` and `weight`
itself (ADR-021).
Read: serde's attribute reference, https://serde.rs/container-attrs.html and
https://serde.rs/enum-representations.html

**Hand-written `Deserialize` and `Serialize`.** Most specification types derive serde's
traits. `Bearing` writes them by hand: `deserialize` asks the YAML reader for the scalar as
text (`String::deserialize`), so it sees `0x10` exactly as written, and `Bearing::from_text`
accepts only plain digits or a compass word; `D::Error::custom` turns a refusal into a
reading error that keeps the line and column. `serialize` writes a number as a number and a
word as a word, so a file read and written back keeps its form (ADR-056). An untagged enum
(`#[serde(untagged)]`, which tries each variant in turn) was the first version; it was
dropped because the YAML reader had already turned `0x10` into 16 before the enum saw it.
Read: https://serde.rs/impl-deserialize.html and https://serde.rs/impl-serialize.html

**`#[serde(try_from = "...")]` and serde's buffering.** An internally tagged enum such as
`Constraint` cannot know its variant until it has seen the `rule` key, so serde first reads
the whole map into a buffer of plain values, and only then hands each field to its type.
In that buffer the YAML reader has already decided that `0x10` is the number 16, so a type
that wants the text as written never sees it. `try_from = "ConstraintAsWritten"` tells
serde to read a plain struct instead, which is read field by field without a buffer, and
then to call `TryFrom` to turn it into the enum; an `Err` from `try_from` becomes a reading
error with the line and column (ADR-059).
Read: https://serde.rs/container-attrs.html#from and https://serde.rs/enum-representations.html#internally-tagged

**Soft pin (`reuse_positions`).** A pin keeps an object exactly where it was. A soft pin
only adds a soft constraint "prefer near where you were": a `near_point` rule within 50 m
of the old centre, weight 1 (both settable: `presets.reuse_within_metres`,
`presets.reuse_weight`). Because the rule is soft, 50 m is where it counts as met, not a
wall: rules with more total weight can pull the object further away. In version 1 it is one
switch for the whole specification.

## Architecture patterns

**The S0 stage and the town core.** S0 lays out the land before the city: rivers first, then
the roads in three steps (roads between two edges; the town core, chosen from where those
roads cross rivers and each other; roads that end at the core), then every bridge and road
junction as an object with its own identifier. The core lies on an object: a river crossing
(bridge or ford), a junction, a road or a river; only with no road of step one and no river is
it the bare site centre, and then `on` is `None` (ADR-047 to ADR-050,
specification-version-1 §4).
Read: ARCHITECTURE.md §4 and specification-version-1 §2 and §4 in this repository.

**Notices as data.** A notice is a remark about a result that is not an error, such as "the
river rule chose the core between equally important roads". The pure core cannot print, so
it stores the notice as a value in its output (`TownCore.notice`,
`SiteOutput.zero_length_roads`), and
the command line prints `notice: ...`. A test can then check the notice without reading the
screen (ADR-049).
Read: Gary Bernhardt, "Functional Core, Imperative Shell",
https://www.destroyallsoftware.com/screencasts/catalog/functional-core-imperative-shell

**Hexagonal architecture, also called ports and adapters.** The core of the program knows
nothing about files, terminals or screens. Those are adapters plugged in at the edge. Our
`citymap-core` is the hexagon; `citymap-command-line` is an adapter.
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
`SignedLength(f64)` cannot be added to `Pixels(f64)` by accident.
Read: Rust Book ch. 20, "Using the Newtype Pattern",
https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits

**Command pattern.** An edit is an object describing the edit ("pin landmark X"), applied by
one function, instead of code scattered across whatever UI produced it. ARCHITECTURE.md §3b.
Read: https://refactoring.guru/design-patterns/command

**Architecture decision record (ADR).** A short note per decision: context, decision,
alternatives rejected, consequences. `documentation/DECISIONS.md` is a list of them.
Read: https://adr.github.io/

**Modules versus classes.** A class in C# or Java does four jobs at once: it holds data,
holds methods, shares behaviour through inheritance, and is a box for privacy. Rust splits
them: a `struct` or `enum` holds the data; an `impl` block gives it methods; a `trait` is a
shared ability several types can have (like an interface, with no inherited data); and a
module (`mod`, usually one file) is the box that decides what is public (`pub`). Plain
functions such as `mix_seed` live in a module without any type around them. Everything inside
one module can see each other's private parts, which is why tests in the same file can test
private helpers.
Read: Rust Book ch. 7.2 and ch. 10.2,
https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html ,
https://doc.rust-lang.org/book/ch10-02-traits.html

**Facade around a library.** Our own small type or module in front of an outside crate, so
the rest of the code never names that crate. `RandomStream` holds the `Pcg64Mcg` privately and
offers only the draws we use; replacing the generator touches one file. A library gets no
facade when there is nothing to wrap: `serde` and `thiserror` work by adding code to our own
types (trait impls and derives), not by functions we call (ADR-020, ADR-028, ADR-029,
ADR-032).
Read: https://refactoring.guru/design-patterns/facade

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

**Least-cost path.** A road's course between its two ends is the route that minimises
a cost (distance plus slope plus water crossings). Dijkstra's algorithm on a grid.
Read: Amit Patel, "Introduction to A*", https://www.redblobgames.com/pathfinding/a-star/introduction.html

**Segment intersection.** Two straight segments either miss each other, meet in one point,
or lie on one line and overlap. S0 asks this for every road against every river and every
other road. A point inside both lines is a real crossing; a point at either end of the road
or the river is only a touch, and an overlap is a road lying on the river; neither becomes a
bridge or a junction (ADR-048). `geo` answers the question; the `geometry` facade turns its answer into
our own `Crossing` values, ordered along the road.
Read: https://docs.rs/geo/0.33.1/geo/algorithm/line_intersection/fn.line_intersection.html

**Closest point on a line.** For a point and a polyline, the nearest point of the polyline
is either the foot of a perpendicular dropped onto one segment, or one of the segment's ends.
The town core uses it when no road crosses anything: the core is the point of the main road
nearest the site centre (ADR-049).
Read: https://docs.rs/geo/0.33.1/geo/algorithm/closest_point/trait.ClosestPoint.html

**Comparing in whole centimetres.** Two distances that look equal can differ in their last
bit as floating-point numbers, and then the order depends on rounding. Our lengths are
already rounded to the centimetre, so `SignedLength::steps` turns them into whole numbers
and `geometry::squared_centimetres` compares squared distances as integers: the town core's
ranking is exact on every machine (CODING_STANDARDS.md, Rust conventions).
Read: Bruce Dawson, "Comparing Floating Point Numbers",
https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/

## Rendering

**SVG basics: `viewBox`, user units, y down.** An SVG file is text that describes shapes. Its
`viewBox="0.00 0.00 2400.00 2400.00"` says which part of an endless drawing plane is shown,
in "user units"; we use metres, so a river 18 m wide is a line with `stroke-width="18.00"`.
SVG counts y downwards from the top, while the model counts y upwards from the south, so `render_svg`
writes every point as `extent - y` (ADR-042). Each layer is a `<g>` group with an `id`.
Read: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/viewBox

**Rasterising to PNG.** An SVG is a description; a PNG is a grid of pixels. Rasterising
draws the description onto a pixel grid of a chosen size. `resvg` does this with its
`tiny-skia` renderer; with its default features off it reads no system fonts or image
files and draws no text at all, so nothing on the computer changes the picture; labels need
bundled fonts at milestone 7 (ADR-044).
Read: https://github.com/linebender/resvg

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

**Unit tests in the same file.** Rust keeps unit tests at the bottom of the file they test,
in `#[cfg(test)] mod tests { use super::*; ... }`. `#[cfg(test)]` means the block is only
compiled by `cargo test`, so it costs nothing in the real program, and because it is a
sub-module it can reach private helpers. Tests that use only the public interface, and our
golden snapshots, go in a separate `tests/` folder per crate (CODING_STANDARDS.md).
Read: Rust Book ch. 11.3, https://doc.rust-lang.org/book/ch11-03-test-organization.html

**Debug assertions.** A check that runs only in a debug build (`cargo build`, `cargo test`)
and is compiled out of a release build (`cargo build --release`). `cfg!(debug_assertions)` is
true in the first and false in the second. `debug_assert_unique` uses it to stop on a
duplicate identifier while developing, without slowing the release program. A test marked
`#[should_panic(expected = "...")]` passes only when the code stops with that message.
Read: Rust Reference, https://doc.rust-lang.org/reference/conditional-compilation.html#debug_assertions

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

**`disallowed_types`.** The same mechanism as `disallowed_methods`, for whole types. We ban
`std::collections::HashMap` and `HashSet` in every crate: their iteration order changes from
run to run, and one careless loop over them would make the same seed draw a different map.
`BTreeMap` and `BTreeSet` are always in sorted order (ADR-031).
Read: https://rust-lang.github.io/rust-clippy/stable/index.html#disallowed_types

**Squash merges, rebasing and stacked PRs.** A squash merge puts one commit per PR on `main`,
so `main` holds whole features only (ADR-041). Rebasing replays a branch's commits on top of
the newest `main` instead of merging `main` into it, so the history stays a straight line.
A stack is a chain of PRs where each builds on the one below; `gh stack` keeps the chain
rebased and merges it in order.
Read: https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/incorporating-changes-from-a-pull-request/about-pull-request-merges ;
https://git-scm.com/book/en/v2/Git-Branching-Rebasing ; https://github.com/github/gh-stack

## Dependencies

**Crate.** Rust's unit of code sharing, like a NuGet package in C#. A crate is either a
library another crate depends on, or a binary. Our workspace has two crates of our own and
depends on others from crates.io, the public registry. `documentation/research/crates-2026-10.md`
checks every crate we consider before an ADR picks it.
Read: Rust Book ch. 7 and ch. 14, https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html ,
https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html

**Why a dependency is checked before it is added.** Five things, each a question the crate
report answers: licence (GPL would void our MIT OR Apache-2.0, ADR-012); maintenance (an
abandoned crate pins us to old versions of everything it touches); `unsafe` (below); wasm32
(below); and how often it breaks its API, because every breaking release costs a day.
Adding a crate needs one line in `documentation/DECISIONS.md` (CODING_STANDARDS.md, Rust conventions).

**`unsafe`.** A Rust keyword that switches off the compiler's memory-safety checks for a
block of code. Safe Rust cannot read freed memory or write past the end of an array;
`unsafe` code can, so a bug in it crashes or corrupts silently. Our own crates forbid it
(`unsafe_code = "forbid"` in the workspace lints). A dependency that uses it is not banned,
but how much it uses and why is recorded in the crate report.
Read: Rust Book ch. 20, https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html

**wasm32 as a purity check.** `wasm32-unknown-unknown` is the WebAssembly target: a
browser sandbox with no file system, no clock, no network and no threads. CI builds
`citymap-core` for it on every PR not because we ship to the browser, but because the build
fails the moment core, or any crate core depends on, touches a file, the clock or a thread.
That turns hard rule 1 in CLAUDE.md ("core is pure") into a compiler error instead of a
review opinion.
Read: https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html

**Facts as data (YAML source, generated SQLite).** Research figures are kept as small YAML
records in git, so every change is a reviewable diff, and a script turns them into a SQLite
database for questions such as "all street widths before 1300". The database is a build
output, like `target/`: delete it and rebuild. ADR-025; format in `data/research/README.md`.
Read: SQLite's "Appropriate Uses" page, https://www.sqlite.org/whentouse.html

**`cargo deny` and licences.** A tool that reads every crate in the dependency tree, including
the crates our crates depend on, and fails CI when one breaks a rule in `deny.toml`. Ours: only
the licences on the allow list (MIT, Apache-2.0, BSD and a few more), never copyleft licences
(GPL, LGPL, AGPL, MPL), which would oblige us to publish under their terms; only crates.io as
a source; no `*` versions; security advisories fail. `include-dev = true` makes it check
test-only crates too (ADR-027).
Read: the cargo-deny book, https://embarkstudios.github.io/cargo-deny/

**Derive macros.** `#[derive(Clone, Debug, thiserror::Error)]` above a type asks the compiler
to write standard code for it at compile time: a copy method, a debug print, an error
message. `thiserror` is a crate that only provides such a derive: `#[error("...")]` on each
variant becomes the `Display` text, so no hand-written `impl` is needed (ADR-029).
Read: Rust Reference, https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros ;
https://docs.rs/thiserror

**Reading YAML with serde.** serde splits reading into two halves: the format crate
(`serde-saphyr` for YAML) turns text into a stream of values, and the derive on our types
says where each value goes. `#[serde(deny_unknown_fields)]` turns a typo in a key into an
error; `#[serde(default)]` fills a missing optional key. The `yaml` facade is the only module
that names the YAML crate (ADR-034).
Read: https://serde.rs/ and https://serde.rs/container-attrs.html

**Command-line arguments with clap's builder.** `clap` reads `generate city.yaml --seed 7`
into typed values and writes the help text and error messages. Its builder API is ordinary
function calls (`Command::new("generate").arg(...)`), so it stays inside the `arguments`
facade; its derive API would put clap attributes on our own types (ADR-043).
Read: https://docs.rs/clap/4.6.7/clap/_tutorial/index.html

**Error enums and exit codes.** `CommandLineError` is a `thiserror` enum with one variant per
kind of failure, and `exit_code()` maps each variant to the number REQUIREMENTS.md F20 gives
it: 2 for invalid input, 1 for reading, drawing and writing. `fn main() -> ExitCode` returns that
number to the operating system instead of calling `std::process::exit` (ADR-043).
Read: https://doc.rust-lang.org/std/process/struct.ExitCode.html

## Rust idioms in our code

**`From<String>` with serde's `from` and `into`.** `#[serde(from = "String", into =
"String")]` on `RoadEnd` tells serde to read a plain word and convert it with our own
`From<String> for RoadEnd`, and to write it with the second implementation,
`From<RoadEnd> for String`. One word in the file, a
typed value in the code: `north_east` becomes `RoadEnd::Region(CompassRegion::NorthEast)`
(ADR-045).
Read: https://serde.rs/container-attrs.html#from

**Hand-written serde for a newtype.** `SignedLength` implements `Serialize` and `Deserialize` by
hand instead of deriving them: serialising writes the plain `f64`; deserialising reads an
`f64` and passes it through `SignedLength::new`, so a file can never smuggle in NaN or an
out-of-range value. A derive would have accepted any `f64` (ADR-028).
Read: https://serde.rs/impl-serialize.html and https://serde.rs/impl-deserialize.html

**`Copy` versus `Clone`.** `Copy` types are copied silently on every assignment, like plain
numbers; only small types made of plain values may be `Copy` (`Identifier`, `Seed`, `SignedLength`).
`Clone` is an explicit `.clone()` call. `RandomStream` is neither on purpose: a copied stream
would hand the same random numbers to two users, so the compiler refuses to copy it.
Read: https://doc.rust-lang.org/std/marker/trait.Copy.html

**Types that rule out bad values.** `NonZeroU64` is a standard integer type that can never be
0; `NonZeroU64::new(n)` returns `None` for 0. `next_below(bound: NonZeroU64)` therefore has no
"bound is zero" case to handle: the caller deals with 0 before the call. This is "parse,
don't validate" with a type from the standard library.
Read: https://doc.rust-lang.org/std/num/type.NonZeroU64.html


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
6. serde, https://serde.rs/ , before milestone 1, since the specification is serde all the way down.
7. "Rust for Rustaceans" by Jon Gjengset (book), after milestone 3, for ownership in graph
   code and when to use trait objects.
8. Jon Gjengset's "Crust of Rust" videos, https://www.youtube.com/c/JonGjengset , the
   episodes on iterators, lifetimes and `dyn`.

Where a concept here is not in this list, add it to this file in the PR that introduces it.
