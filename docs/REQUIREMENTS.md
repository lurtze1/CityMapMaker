# Requirements — CityMapMaker

Status: v0.1, 2026-10-19. Written from a discussion with Jeffrey and a read of the *Peace and War*
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
  today), then printed A4 handouts, with A3 as a nice-to-have.

## 3. Goals

- G1 **Spec in, map out.** A declarative spec file is the only input. No GUI required.
- G2 **Environment-aware layout.** Coast, harbour, cliffs, hills, rivers and walls shape the
  street network and district placement plausibly for a pre-industrial city.
- G3 **Named landmarks and districts honoured.** Every named item in the spec appears, placed
  according to its constraints, and is numbered in a legend.
- G4 **Deterministic and stable.** Same spec and seed give the identical map. Pinned landmarks
  never move when the seed changes or when new landmarks are added. Nearby detail may adjust;
  far-away detail must not.
- G5 **Setting rules are pluggable.** World-specific rules (compass-locked seasonal districts,
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
- World or regional maps. City scale only (roughly 500 m to 5 km across). A regional map is
  a different problem (climate, rivers, borders) and Azgaar already solves it; the overlap is
  the city's **site**. The spec's site section must therefore be importable: hand-authored
  now, derived later from an Azgaar export, a height-map image or a future generator of our
  own. Core only sees a site and never knows its origin (see ARCHITECTURE.md §4, S0).

## 5. Functional requirements

### 5.1 Spec
- F1 Spec is a YAML or JSON file validated against a published schema; errors name the field.
- F2 Spec, version 1, contains:
  - identity: name, seed, free-text notes (ignored by the generator, context for whoever
    writes the spec);
  - settlement block: level, population, size class;
  - site: terrain (coast with direction, cliffs, hills, slope, plateau), water (river with
    course and width, lake, harbour, tidal flag, canals), land cover around and inside the
    city (forest, marsh, farmland, orchards, heath, rock, parks, gardens, groves);
  - defences: wall type and count, citadel, gates with directions;
  - open space: plazas, market squares, greens, with constraints like landmarks;
  - crossings: bridges and fords, since they fix street topology;
  - districts: name, type, size weight, constraints, notes;
  - landmarks: id, name, kind, footprint class, constraints, optional resolved placement,
    `pinned`, notes;
  - entry roads: direction, importance, named destination;
  - presets: style, era or culture, rule packs to load.
- F2b Reserved for version 2, schema must leave room: history layers (old wall rings, razed
  walls as ring roads, ruined quarters), infrastructure kinds (wells, mills, aqueducts,
  cemeteries), climate and roof style, hazard and lore zones (flood plain, uncertain streets).
- F3 Constraints vocabulary, first version: `compass_region`, `near`, `far_from`, `on_water`,
  `on_high_ground`, `inside_walls`, `outside_walls`, `in_district`, `min_spacing`, `fixed`.
  Each hard or soft with a weight.
- F4 Unknown rule types from a rule pack the spec names are an error, not silently ignored.
- F5 The spec may embed the Pathfinder 2e settlement block fields so a vault note can carry
  both; mapping documented.

### 5.2 Generation
- F6 Pipeline stages: site → macro layout (walls, districts) → landmarks → streets → blocks and
  parcels → buildings → labels → decorators. Each stage is a pure function with its own RNG
  streams; see ARCHITECTURE.md.
- F7 A landmark with `pinned: true` and a resolved placement is reproduced exactly. A landmark
  without one is placed by constraints and its placement written back to the spec on request
  (`--write-back`), so the user can then pin it.
- F8 Placement order is stable: pinned first, then by spec order. Landmark N+1 may not move
  landmarks 1..N; on collision the generator searches locally and otherwise reports failure.
- F9 Walls derive from the set of intramural districts; gates sit where entry roads cross the
  wall; a palisade, a stone wall and a torn-down-and-rebuilt wall are distinct wall types.
- F10 Streets: primary roads gate-to-market and between major landmarks, secondary streets per
  district following a direction field (radial around plazas, boundary-following along walls
  and water, noise for organic wards), tertiary alleys per block.
- F11 Districts can be locked to a compass sector and to adjacency with water or walls.
- F12 Farms, shanties and suburbs may sit outside walls when the spec says so.

### 5.3 Output
- F13 SVG with layers as groups: terrain, water, walls, streets, buildings, landmarks, labels,
  legend, optional paper layer. PNG at a chosen DPI: screen resolution and A4 at 300 DPI
  must work; A3 at 300 DPI is a nice-to-have.
- F14 Numbered landmark markers with a legend; labels placed without overlap; leader lines only
  as a last resort.
- F15 A machine-readable model export (JSON) with all ids, so later tools and Foundry export
  can reference objects.
- F16 Foundry VTT, two steps. Minimal, right after the first handout: a label-free PNG plus
  scene JSON with correct dimensions and grid so the map is usable in a session. Full, later:
  city wall as walls, gates as doors, landmark notes linked to journal entries, Universal VTT.

### 5.4 CLI
- F17 `generate <spec> [--seed N] [--out DIR] [--png] [--write-back]`.
- F18 `validate <spec>`.
- F19 `sweep <spec> --seeds A..B` writes a contact sheet and metrics CSV (see §7).

## 6. Quality requirements

- Q1 Determinism verified in CI: generating twice gives byte-identical model JSON; a change of
  seed leaves pinned placements byte-identical; adding one landmark leaves all objects outside
  its neighbourhood byte-identical.
- Q2 A level-9 city of 10,000 people generates in under 5 seconds on Jeffrey's machine; a
  render to A3 PNG in under 15 seconds. Not optimised beyond that until it matters.
- Q3 Core has no I/O and no platform dependencies; it compiles to WASM without changes (not a
  release target, a purity check).
- Q4 Labels at 300 DPI are at least 8 pt equivalent; landmark numbers readable on A4.
- Q5 Scale: the model is in metres; the renderer fits the city to the page by default and
  always draws a scale bar and north arrow. `--scale 1:N` locks a ratio for comparable series.
  If fit-to-page makes the median building smaller than about 2 px at the chosen DPI, the
  renderer warns and suggests a larger paper size or a district inset.

## 7. Diversity requirement (the "only cloudy days" problem)

Calibrating on Truro alone would bias every parameter towards one timber port. Mitigation:
- Three calibration cities with deliberately different geometry (below), in order: Truro
  (simple), Arcachon (hard constraints), Bearna (vertical terrain).
- An **expressive range harness** from the first milestone that can place anything: run many
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
  presence that may want green space rules.
- **GM?** Which parts of the city sit on top of the cliffs and which at water level? Is there a
  wall at all, given cliffs? Where do the three waters meet relative to the city? Name of the
  royal seat. Are the Seelie and Unseelie courts present as districts or only as politics?

## 9. Review tiers (what we look at, in what order)

The build order in ARCHITECTURE.md says what gets built. This table says what Jeffrey looks at
and judges at each checkpoint, and what is deliberately ignored so early reviews stay honest.
A tier is passed when its "judge" column is good enough to build on, not when it is pretty.

| Tier | What is on the page | Judge | Ignore |
|---|---|---|---|
| T0 Site | Coastline, water, hills, cliffs, land cover; no city | Does it read as the place in the brief? Is the sea where it should be? | Style, labels |
| T1 A city | A blob of anonymous buildings inside a wall on that site | Does it read as a city at all? Size plausible for the population? Does it sit on the terrain sensibly? | Streets, districts, landmarks |
| T2 Landmarks | T1 plus numbered landmark markers and a legend | Is every named landmark present, where the brief says, at a believable footprint? Does pinning hold across seeds? | Street quality, district character |
| T3 Districts | T2 plus districts shaded or hatched differently | Are districts where the constraints say? Do they have sensible size and adjacency (docks on water, farms outside)? | Street quality, building detail |
| T4 Streets | T3 plus primary, secondary and tertiary streets | Do roads go gate to market? Do districts have the right texture (radial, grid, organic)? Dead ends and absurd blocks? | Building detail, labels |
| T5 Buildings | T4 plus footprints per parcel, landmark footprints | Density gradient, block shapes, landmark footprints readable | Final style |
| T6 Handout | T5 with labels, legend, style preset, screen PNG and A4 print PNG | Can you hand this to players? Readable in print? | Nothing |
| T7 Rules | Arcachon with seasonal districts and moving landmarks | Do rule packs change only what they should? Rotation moves the theatre, never the castle | Visual polish |
| T8 Terrain | Bearna with cliffs, levels, cave gate | Does vertical terrain work without special cases in core? | Visual polish |
| T9 Range | Contact sheets across many seeds for all three cities | Do the cities look different from each other and from themselves across seeds? Any collapse to one shape? | Individual map quality |

Every tier ends with a PNG in `out/` that was looked at, and notes in `docs/DECISIONS.md` if
the look changed a decision. Tiers can be revisited: a later tier may send us back to fix an
earlier one, and that is expected.

## 10. Open questions for the GM (collected)

All **GM?** items in §8. Everything else raised during the 2026-10-19 review is resolved; see
DECISIONS.md ADR-010 to ADR-012.
