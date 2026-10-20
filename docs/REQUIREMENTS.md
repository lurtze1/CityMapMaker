# Requirements — CityMapMaker

Last reviewed: 2026-10-19. Written from a discussion with Jeffrey and a read of the *Peace and War*
vault. Items marked **GM?** need an answer from Jeffrey before they can be built.

## 1. Purpose

Generate good-looking, annotated **overview maps of fantasy cities** from a written spec, so the
GM gets usable handouts for cities he has described in prose but cannot draw. The map must be
logical for the city's environment (coast, cliffs, river, hills), honour the named landmarks and
districts the GM has already written, and be regenerable later with additions without losing what
was already approved.

## 2. Primary user and use

- One user: Jeffrey, GM of *Peace and War*, backend engineer, reviewing code while learning Rust.
- Use: write or dictate a spec, run the CLI, open the PNG, iterate on the spec, pin what is
  good, add landmarks as the campaign grows. An AI assistant may write or edit specs; it does
  not drive a GUI.
- Output consumers, in priority order: the GM's screen and Foundry VTT (sessions run online
  as of 2026-10-19), then printed A4 handouts, with A3 as a nice-to-have.

## 3. Goals

- G1 **Spec in, map out.** A declarative spec file is the only input. No GUI required.
- G2 **Environment-aware layout.** Coast, harbour, cliffs, hills, rivers and walls shape the
  street network and district placement. "Plausible" is decided by Jeffrey at review
  tiers T1 to T4 (§9); the hard rules are: no street crosses water without a bridge or ford,
  no building stands in water or on a cliff face, every district constraint is satisfied.
- G3 **Named landmarks and districts honoured.** Every named item in the spec appears, placed
  according to its constraints, and is numbered in a legend.
- G4 **Deterministic and stable.** Same spec and seed give the identical map. Pinned landmarks
  never move when the seed changes or when new landmarks are added. Detail inside the
  edited cell and its one-ring of neighbour cells is allowed to change; everything outside
  that ring must be byte-identical (ARCHITECTURE.md §5).
- G5 **Setting rules are pluggable as data.** Rule packs are YAML loaded by name; the
  evaluators they reference are compiled into the one binary (F4). World-specific rules
  (compass-locked seasonal districts,
  stable boulevards versus uncertain alleys) are rule packs loaded with the spec, not core code.
- G6 **Hand-drawn look.** Ink-and-parchment line art with a legible legend, exported as SVG and
  print-resolution PNG.
- G7 **Extensible to dungeons and a UI** without rewriting core: same command path for spec
  edits whether they come from a file, a CLI flag, an AI, or a future tool.
- G8 **Diverse output.** The generator must not collapse into one kind of city. Measured, not
  assumed (see §7).

## 4. Non-goals (for now)

- Battlemaps at 5-foot grid scale. Not a goal now; a possible future generator behind the
  same pipeline, like dungeon maps. Nothing in the core may assume overview scale is the only
  scale.
- A website or multi-user product. This is a personal tool.
- Painted, textured, Inkarnate-like rendering. Line art only.
- Simulation of history, economy or population growth. Layout is a design problem, not a sim.
- City evolution over time ("they built a new quarter in the last ten years", "after the
  riots"). Version 1 describes a city as it is now; a spec says what exists, not how it got
  there. Evolution, events and history layers are version 2 or later, and the spec format
  must not make them impossible (see F2b).
- Inventing lore. The generator never names a landmark, district or street on its own.
- World or regional maps. City scale only: between 500 m and 5 km across. Outside that
  range the generator is not required to work. A regional map is
  a different problem (climate, rivers, borders) and Azgaar already solves it; the overlap is
  the city's **site**. The spec's site section must therefore be importable: hand-authored
  now, derived later from an Azgaar export, a height-map image or a future generator of our
  own. Core only sees a site and never knows its origin (see ARCHITECTURE.md §4, S0).

## 5. Functional requirements

### 5.1 Spec
- F1 Spec is a YAML or JSON file validated against a published schema; errors name the field.
  Field-naming errors come from `serde_path_to_error`; every spec struct uses
  `deny_unknown_fields`; enums are internally tagged; each error message has a unit test.
  The first key of every spec is `schema: 1`. A missing or unknown schema version is rejected
  with exit code 2. No migration tooling exists until a version 2 schema exists.
  `--write-back` never changes the schema version. Spec and pack files are untrusted input:
  YAML anchors, aliases and custom tags are rejected; at most 1000 landmarks and 100
  districts per spec; a pack is data and cannot run code.
- F2 Spec, version 1, contains:
  - identity: name, seed (`u64`), free-text notes (ignored by the generator, context for
    whoever writes the spec);
  - settlement block: population (ADR-021: no game-system level, no size class);
  - site: terrain (coast with direction, cliffs, hills, slope, plateau), water (river with
    course and width, lake, harbour, tidal flag, canals), land cover around and inside the
    city (forest, marsh, farmland, orchards, heath, rock, parks, gardens, groves);
  - defences: wall type and count, citadel, gates with directions;
  - open space: plazas, market squares, greens, with constraints like landmarks;
  - crossings: bridges and fords, since they fix street topology;
  - districts: name, type, size weight, constraints, notes;
  - landmarks: id, name, kind, optional footprint (`class`, `width_m`, `depth_m`; the kind's
    default class when absent; resolved to one width and depth before core), constraints,
    optional resolved placement, `pinned`, notes;
  - entry roads: direction, importance;
  - presets: style, era or culture, rule packs to load.
- F2b Reserved for version 2, schema must leave room: history layers (old wall rings, razed
  walls as ring roads, ruined quarters), infrastructure kinds (wells, mills, aqueducts,
  cemeteries), climate and roof style, hazard and lore zones (flood plain, uncertain streets).
- F3 Constraints vocabulary, first version: `compass_region`, `near`, `far_from`, `on_water`,
  `on_high_ground`, `inside_walls`, `outside_walls`, `in_district`, `min_spacing`.
  Each hard or soft with a weight. A constraint target (`near: market`) is always the spec id
  of something with a position (landmark, district, open space, crossing, river, lake,
  harbour, wall, gate, citadel), never a road id and never a generated id. Exact placement
  is done only by pinning (F7), not by a constraint.
- F4 Unknown rule types from a rule pack the spec names are an error, not silently ignored.
- F5 Withdrawn (ADR-021). The spec does not embed game-system fields such as the Pathfinder 2e
  settlement block. A translator from a game system to explicit spec fields is a future idea
  (issue #30).

### 5.2 Generation
- F6 Pipeline stages: site → macro layout (walls, districts) → landmarks → streets → blocks and
  parcels → buildings → labels → decorators. Each stage is a pure function with its own RNG
  streams; see ARCHITECTURE.md.
- F7 A landmark with `pinned: true` and a resolved placement is reproduced exactly. A landmark
  without one is placed by constraints and its placement written back to the spec on request
  (`--write-back`), so the user can then pin it. `--write-back` edits only the `resolved`,
  `pinned` and `seed` keys in place and preserves every comment, key order and formatting;
  it never re-serialises the whole file. A pinned placement that violates a hard site rule
  (a building in water after the site changed) is a constraint failure (F8); pins never move
  silently.
- F8 Placement order is stable: pinned first, then by spec order. Landmark N+1 must not move
  landmarks 1..N. On collision the generator searches outward in a deterministic spiral of at
  most `MAX_PLACEMENT_RINGS` (a core constant, 12, changed only by ADR). If no candidate
  satisfies every hard constraint, generation stops: exit code 3, no output files written,
  `--write-back` does not run, and stderr lists the landmark id, the failed rule and the
  candidates tried.
- F9 Walls derive from the set of intramural districts; gates sit where entry roads cross the
  wall; a palisade, a stone wall and a torn-down-and-rebuilt wall are distinct wall types.
- F10 Streets: primary roads gate-to-market and between major landmarks, secondary streets per
  district following a direction field (radial around plazas, boundary-following along walls
  and water, noise for organic wards), tertiary alleys per block.
- F11 Districts can be locked to a compass sector and to adjacency with water or walls.
- F12 Farms, shanties and suburbs are placed outside the walls when the spec says so, and
  inside otherwise.

### 5.3 Output
- F13 SVG with layers as groups: terrain, water, walls, streets, buildings, landmarks, labels,
  legend, optional paper layer. PNG at a chosen DPI: screen resolution and A4 at 300 DPI
  must work; A3 at 300 DPI is a nice-to-have.
- F14 Numbered landmark markers with a legend; labels placed without overlap; leader lines only
  as a last resort.
- F15 A machine-readable model export (JSON) with all ids, so later tools and Foundry export
  can reference objects. It carries `schema` (the model schema version, starting at 1) and
  `generator` (the git commit hash of the build).
- F16 Foundry VTT, two steps. Minimal, right after the first handout: a label-free PNG plus
  scene JSON with correct dimensions and grid so the map is usable in a session. Full, later:
  city wall as walls, gates as doors, landmark notes linked to journal entries, Universal VTT.
  The Foundry target version is v13 until an ADR changes it.

### 5.4 CLI
- F17 `generate <spec> [--seed N] [--out DIR] [--png] [--dpi N] [--scale 1:N] [--write-back]`.
  `--seed` overrides the spec's seed for that run only; the seed used is part of every output
  file name; `--write-back` writes the seed used into the spec.
- F18 `validate <spec>`.
- F19 `sweep <spec> --seeds A..B` writes a contact sheet and metrics CSV (see §7).
- F20 Exit codes: 0 success; 1 I/O or internal error; 2 invalid input (spec, pack or style
  file fails validation, including a duplicate rule id across packs or an unknown evaluator);
  3 constraint failure. Warnings never change the exit code.

## 6. Quality requirements

- Q1 Determinism verified in CI on Windows and Linux: generating twice gives byte-identical
  model JSON and SVG; a change of seed leaves pinned placements byte-identical; adding one
  landmark leaves all objects outside its cell and one-ring byte-identical. PNG bytes are not
  compared.
- Q2 In a release build on Jeffrey's machine: a city of 10,000 people generates in
  under 5 seconds and renders to an A3 PNG in under 15 seconds. `sweep` has no budget.
  No optimisation work is done while these limits hold. When a limit is exceeded, Jeffrey
  decides whether to raise the limit or optimise.
- Q3 Core has no I/O and no platform dependencies; it compiles to WASM without changes (not a
  release target, a purity check).
- Q4 Labels at 300 DPI are at least 8 pt equivalent; landmark numbers readable on A4.
- Q5 Scale: the model is in metres; the renderer fits the city to the page by default and
  always draws a scale bar and north arrow. `--scale 1:N` locks a ratio for comparable series.
  If fit-to-page makes the median building smaller than 2 px at the chosen DPI, the
  renderer warns and suggests a larger paper size or a district inset.

## 7. Diversity requirement (the "only cloudy days" problem)

Calibrating on Truro alone would bias every parameter towards one timber port. Mitigation:
- Three calibration cities with deliberately different geometry (below), in order: Truro
  (simple), Arcachon (hard constraints), Bearna (vertical terrain).
- An **expressive range harness**, built at milestone 5 (the first milestone with streets):
  run many
  seeds, compute cheap metrics (enclosed fraction, street density gradient from centre, block
  size distribution, share of water-adjacent blocks, landmark-to-market graph distance, wall
  perimeter to area), plot them, and compare against a handful of real medieval town plans.
  Presets must be tuned per era or culture, never from one sample.

## 8. Calibration city briefs

These are extracted from the vault, not invented. Every landmark named here exists as a vault
note or is stated in the city's note. Where the vault is silent the item is listed under GM?.

### 8.1 Truro (calibration city 1: simple port)

Source: `Lore/Shards/Ilse of Green/Cities/Truro/Truro.md` and `Truro at Festival.md`.

- Settlement level 9, city, population 10,000. South-eastern coast of the Isle of Green, warm
  water port, gateway between Ys Ywain and the other shards.
- **Look:** early-medieval Viking-meets-Irish fishing town. Mostly timber: longhouses,
  workshops, net-sheds, halls. A few squat stone buildings for the things that matter.
  Plank walkways, fish-drying racks, upturned boats, shipyards. Everything tips toward the
  harbour.
- **Defences:** palisade walls on the landward side only. Sea side open.
- **Terrain:** sea to the east and south-east; the city sits in a bay or inlet. Mental
  model: Dublin. A river runs through the city to the sea (GM 2026-10-19, not yet shown to
  players; course and which bank the Lodge is on are free). Hills nearby with shifting ore
  veins (ironworking), interior behind the hills with goblinoid raiders.
- **Economy to show:** artisan shipyards (bespoke longboats and galleys), pearl diving and
  fishing, iron smithing supplied from the hills.
- **Landmarks (vault + GM 2026-10-19):** The Truro Lodge (oldest Pathfinder lodge, stone;
  central, adjacent to the harbour and warehouse district), the Council's Hall (a longhall
  holding the Council of Two), The Raised Anchor, The Debtor's Shrine, shrines (plural,
  stone; one in the residential district north-west of the Lodge), the harbour, the ship
  manufactories south and south-west of the harbour, a festival market space (the
  Pathfinder Festival spreads market stands citywide).
- **Districts (GM 2026-10-19):** harbour and warehouses (adjacent to the Lodge); residential
  with a shrine, north-west of the Lodge; craftsmen (blacksmiths, craftsmen and the like)
  north-east of the Lodge; ship manufactories south and south-west of the harbour; wealthy
  halls (vault: stone, placement open); farms or fisher villages outside the palisade. There
  is **no Jotunnborn quarter**: the newer parts of the city, western and more inland but
  still inside the palisade, have many buildings built to Jotun scale. Model that as a
  district attribute (building scale), not a district. The palisade was expanded to enclose
  the newer parts; that is a Truro fact, not a general generator rule.
- **Rule hooks:** "uncertain alleys" layer from Physical Reality (busy boulevards stable, dark
  alleys fluid). Festival state could be a decorator (banners, stands), not geometry.
- **GM?** Where are the wealthy halls (the vault names them among the stone buildings but
  does not place them)? (The riots branch is out of scope: version 1 describes the city as
  it is now. City evolution over time is a later version, see §4.)

### 8.2 Arcachon (calibration city 2: hard constraints)

Source: `Lore/Shards/Valleys of Flowers/Cities/Arcachon/Arcachon.md` and linked notes.

- Capital of Arcataine, City of Four Seasons, patron city of Catuchon. Level **GM?**,
  population **GM?** (the note has no settlement block).
- **Site:** peninsula on the western coast of the Valleys of Flowers, water on three sides,
  wrapped around an inlet of the Sea of Wine. Visitors arrive by ship on the north side.
  Landward exit is to the south.
- **Look:** Byzantine court splendour filtered through High Medieval southern France. Thick
  walls, modest windows, all-weather architecture because each district has real seasonal
  weather. Walls never breached; strong stone fortifications.
- **Districts, fixed to the compass; the season each holds rotates on a king's death:**
  - North, currently Spring: commercial and maritime hub, taverns, shops, warehouses, nearest
    the inlet.
  - East, currently Winter: apothecaries, doctors, butchers, tanners, mortuaries.
  - South, currently Autumn: seat of government, landward exit, otherwise undeveloped.
  - West, currently Summer: cultural and residential heart, noble quarter, theatre, pleasure
    houses.
- **Landmarks (vault):**
  - Castle Polignac, fixed, always South.
  - The Grand Academy of Arcachon, fixed, North.
  - Temple of Resplendent Healing, fixed, East, cross-shaped, with a market square outside.
  - Midsummer's Eve theatre, moves with the season, currently West, in the former naval academy
    building (French Gothic).
  - The Purple Dragon tavern, moves with the season, currently North tavern row.
  - Grand Arsenal, fixed, location to confirm (**GM?**), Purple Navy shipyard, needs water.
  - Noble quarter, currently West.
  - A blood-spewing fountain somewhere in the city (Act One hook; a decorator, not a landmark).
- **Rule hooks this city forces:** `compass_region` as a hard district constraint; a "season"
  attribute per district with a rotation rule; `fixed` versus `moves_with_season` landmarks,
  where a season rotation re-places the moving ones but never the fixed ones. This is the
  stress test for rule packs and for pinning.
- **GM?** Settlement level and population. Compass direction of the inlet mouth. Whether the
  Grand Arsenal is North (maritime) or West. Number and placement of gates on the south wall.
  Is the peninsula flat or does it rise toward the castle?

### 8.3 Bearna (calibration city 3: vertical terrain)

Source: `Lore/Shards/Ilse of Green/Cities/Bearna/Bearna.md`, `The Adventurer's Rest.md`,
`Ilse of Green.md`.

- Level 12 city, elf trait, capital of Ys Ywain, population 10,000 (6,500 humans, 1,500 wood
  elves, 1,000 leshies, 800 Jotunn, 200 other). Patron city of the goddess Bearna.
- **Site:** dramatic cliffs at the convergence of three bodies of water: the Ocean of Bearna,
  the Ocean of Ygir, and the underground sea of Dis reached through an entrance carved into the
  cliff face. Westernmost major settlement on the Isle.
- **Look:** magical metropolis, elven influence, state-sponsored adventuring. Exact
  architecture **GM?**.
- **Landmarks (vault):** the Gateway to Dis (cliff-face entrance, trade and danger), the
  temple of Bearna (High Priestess Durina Copperbeard), the seat of High King Niall
  (name **GM?**), The Adventurer's Rest inn (the Gang of Five), the start of the Standing
  Stones path leading east toward the Fey Woods.
- **Rule hooks:** multi-level terrain (cliff top, cliff face, water level), a harbour on which
  water (**GM?**), a non-standard "gate" that is a cave mouth, an elven quarter and a leshy
  presence. Open question, Jeffrey decides at milestone 9: does Bearna need green-space
  rules?
- **GM?** Which parts of the city sit on top of the cliffs and which at water level? Is there a
  wall at all, given cliffs? Where do the three waters meet relative to the city? Name of the
  royal seat. Are the Seelie and Unseelie courts present as districts or only as politics?

## 9. Review tiers (what we look at, in what order)

The build order in ARCHITECTURE.md says what gets built. This table says what Jeffrey looks at
and judges at each checkpoint, and what is deliberately ignored so early reviews stay honest.
Districts (T2) are judged before landmarks (T3) because landmarks are placed inside
districts by stage S2, which runs after stage S1 produces the districts; judging landmarks
first would mean judging them against districts that do not exist yet.
The "judge" column lists yes-or-no questions. A tier passes when Jeffrey answers yes to
every question. Only Jeffrey passes a tier. Pretty is not a criterion before T6.

| Tier | What is on the page | Judge | Ignore |
|---|---|---|---|
| T0 Site | Coastline, water, hills, cliffs, land cover; no city | Does it read as the place in the brief? Is the sea on the compass side the brief states? | Style, labels |
| T1 A city | A blob of anonymous buildings inside a wall on that site | Does it read as a city at all? Is the built area within 30% of population times the preset's area per person? Is every building on land and off cliff faces? | Streets, districts, landmarks |
| T2 Districts | T1 plus districts shaded or hatched differently | Are districts where the constraints say? Is each district's area within 20% of its spec weight, and is every adjacency constraint satisfied (docks on water, farms outside)? | Street quality, landmarks, building detail |
| T3 Landmarks | T2 plus numbered landmark markers and a legend | Is every named landmark present, where the brief says, with a footprint of the class the spec names? Does pinning hold across seeds? | Street quality, building detail |
| T4 Streets | T3 plus primary, secondary and tertiary streets | Do roads go gate to market? Do districts have the right texture (radial, grid, organic)? Dead ends and absurd blocks? | Building detail, labels |
| T5 Buildings | T4 plus footprints per parcel, landmark footprints | Density gradient, block shapes, landmark footprints readable | Final style |
| T6 Handout | T5 with labels, legend, style preset, screen PNG and A4 print PNG | Can you hand this to players? Readable in print? | Nothing |
| T7 Rules | Arcachon with seasonal districts and moving landmarks | Does the pack change attributes only and no geometry (pack isolation test passes)? Does a season rotation move the theatre and never the castle? | Visual polish |
| T8 Terrain | Bearna with cliffs, levels, cave gate | Does vertical terrain work without special cases in core? | Visual polish |
| T9 Range | Contact sheets across many seeds for all three cities | Do the cities look different from each other and from themselves across seeds? Any collapse to one shape? | Individual map quality |

Every tier ends with a PNG in `out/` that was looked at, and notes in `docs/DECISIONS.md` if
the look changed a decision. A later tier can fail an earlier tier's output; then the earlier tier is
reworked and re-passed before the later tier continues.

## 10. Open questions for the GM (collected)

All **GM?** items in §8. Everything else raised during the 2026-10-19 review is resolved; see
DECISIONS.md ADR-010 to ADR-012.
