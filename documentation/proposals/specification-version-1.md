# Proposal: specification schema version 1 and the example city

Status: accepted by Jeffrey on 2026-10-20 with the changes recorded in ADR-021 and §6. The
example city is fictional and exists only to exercise every field; it is not campaign lore
(ADR-016).

## 1. The example specification

```yaml
schema: 1

identity:
  name: Harrowmere
  seed: 20261019
  notes: >
    Fictional river port used by the test suite. Everything below is invented to touch most
    fields; the fields it leaves out are exercised by the section 5 tests. No campaign lore.

settlement:
  population: 6000

site:
  extent_metres: 2400             # side of the square site in metres; the city must fit inside
  terrain:                        # superseded by ADR-056: bearing_degrees, shape, shore, hollows, hill height
    coast: { side: east, kind: bay }              # side: north|east|south|west|none
    cliffs: []                                    # list of { side, height_metres }
    hills: [{ at: north_west, radius_metres: 500 }]
    slope: { toward: east, metres_per_kilometre: 8 }
    plateau: null
  water:
    rivers:
      - { identifier: ash, from: north_west, to: east, width_metres: 18 }
    lakes: []
    harbour: { identifier: quay, side: east, length_metres: 300 }
    tidal: true
    canals: []
  land_cover:
    outside: [farmland, orchards]
    inside: [gardens]
    forest: { side: north, depth_metres: 400 }
    marsh: { side: south, depth_metres: 150 }

roads:
  - { identifier: north_road, ends: [north, south], importance: major }    # through road
  - { identifier: west_road, ends: [west, core], importance: minor }        # ends at the town core
  - { identifier: weavers_lane, ends: [core, guildhall], importance: minor } # local road to a pinned landmark

defences:
  walls:
    - { identifier: outer, kind: stone, encloses: [old_town, craft, market_q, castle], along_water: wall }
  citadel: { identifier: keep, side: north, inside: outer }
  gates:
    - { kind: road_gate, identifier: north_gate, road: north_road, wall: outer, end: north }
    - { kind: water_gate, identifier: quay_gate, wall: outer, side: east }

open_space:
  - identifier: market
    kind: market_square
    constraints: [{ rule: near, target: ash, hard: false, weight: 2 }]
    resolved: { x: 1300.00, y: 1250.00, rotation_degrees: 0, width_metres: 60, depth_metres: 40 }
    pinned: true
  - identifier: green
    kind: green
    constraints: [{ rule: in_district, target: old_town, hard: true }]

crossings:
  - identifier: ash_bridge
    river: ash
    kind: bridge
    constraints: [{ rule: near, target: market, hard: false, weight: 3 }]
    resolved: { x: 1402.00, y: 1150.00, rotation_degrees: 90, width_metres: 8, depth_metres: 30 }
    pinned: true

districts:
  - identifier: old_town
    name: Old Town
    function: residential
    wealth: middling
    weight: 3
    constraints: [{ rule: near, target: market, hard: true }]
    notes: oldest part, around the market
  - identifier: craft
    name: Craftsmen's Quarter
    function: craft
    weight: 2
    constraints: [{ rule: compass_region, region: north_east, hard: true }]
  - identifier: market_q
    name: Merchants' Quarter
    function: commerce
    wealth: rich
    weight: 2
    constraints: [{ rule: near, target: market, hard: false, weight: 2 }]
  - identifier: castle
    name: The Keep Ward
    function: seat_of_power
    enclosure: walled
    jurisdiction: separate
    weight: 1
    constraints: [{ rule: near, target: old_keep, hard: true }]
  - identifier: docks
    name: The Docks
    function: harbour
    group: river_folk
    weight: 2
    constraints: [{ rule: on_water, hard: true }]
  - identifier: tanneries
    name: Tanners' Row
    function: works
    wealth: poor
    weight: 1
    constraints: [{ rule: on_water, hard: true }]
  - identifier: graves
    name: St Ash Field
    function: cemetery
    weight: 1
    constraints: [{ rule: near, target: west_road, hard: false, weight: 1 }]
  - identifier: commons
    name: Harrow Common
    function: open_ground
    weight: 1
    constraints: [{ rule: far_from, target: market, hard: false, weight: 1 }]
  - identifier: fields
    name: Outer Fields
    function: farmland
    weight: 1
    constraints: [{ rule: compass_region, region: south_west, hard: false, weight: 1 }]

landmarks:
  - identifier: old_keep
    name: The Old Keep
    function: defence
    importance: major
    constraints: [{ rule: compass_region, region: north, hard: true }]
    resolved: { x: 1250.00, y: 1700.00, rotation_degrees: 0, width_metres: 40, depth_metres: 52 }
    pinned: true
    notes: pinned, so the keep ward can grow around it (districts see only pinned objects)
  - identifier: temple
    name: Temple of the Tide
    function: religious
    label: cathedral
    constraints:
      - { rule: compass_region, region: north_east, hard: true }
      - { rule: on_high_ground, hard: false, weight: 2 }
    resolved: null
    pinned: false
    notes: ""
  - identifier: guildhall
    name: Weavers' Hall
    function: civic
    label: guildhall
    importance: major
    footprint: { class: large, depth_metres: 60 }
    constraints:
      - { rule: in_district, target: craft, hard: true }
      - { rule: near, target: north_gate, hard: false, weight: 2 }
    resolved: { x: 1610.25, y: 1480.50, rotation_degrees: 0, width_metres: 30, depth_metres: 60 }
    pinned: true
    notes: pinned to test that pins survive a seed change
  - identifier: lighthouse
    name: The Harrow Light
    function: utility
    label: lighthouse
    footprint: { class: small }
    constraints:
      - { rule: in_district, target: docks, hard: true }
      - { rule: far_from, target: temple, hard: false, weight: 1 }
      - { rule: min_spacing, metres: 40, hard: true }
    resolved: { x: 2210.00, y: 1300.00, rotation_degrees: 0, width_metres: 11, depth_metres: 11 }
    pinned: false
    notes: an unpinned resolved placement, used only when reuse_positions is on

presets:
  era: early_medieval         # early_medieval | high_medieval | late_medieval
  style: ink
  packs: []
  reuse_positions: false      # `--reuse-positions` / `--no-reuse-positions` override per run
  reuse_within_metres: 50
  reuse_weight: 1
  centre_radius_metres: 150
```

Districts not listed in any wall's `encloses` (docks, tanneries, graves, commons, fields) lie
outside every wall. The market is pinned so that district rules may target it (§4,
visibility). The labels `cathedral`, `guildhall` and `lighthouse` are listed in the default
rule pack, so the example produces no label warning.

## 2. The schema this implies

Types below are what `citymap-command-line` parses into and `citymap-core` consumes. Every struct is
`deny_unknown_fields`; every enum is a closed list; every identifier is unique across the whole
file, not only within its list, so a constraint target such as `near: market` names exactly
one thing; constraint targets refer to identifiers in this file only, and only to identifiers of
things with a position (REQUIREMENTS.md F3, §4 below). Every enum with fields is internally tagged
(REQUIREMENTS.md F1): a variant is written as a map whose first key is its tag, `kind` for
gates and `rule` for constraints. An enum without fields is a plain string
(`era: early_medieval`). Jeffrey accepted this section on 2026-10-20 with the answers in §6.

| Field | Type | Required | Notes |
|---|---|---|---|
| `schema` | integer, must be 1 | yes | first key; anything else is exit 2 |
| `identity.name` | string, 1 to 80 chars | yes | |
| `identity.seed` | u64 | yes | `--seed` overrides for one run |
| `identity.notes` | string | no, default "" | ignored by the generator |
| `settlement.population` | integer 1 to 1,000,000 | yes | drives built area with the era preset; the only size input |
| `site.extent_metres` | number 500 to 5000 | yes | REQUIREMENTS.md non-goal: 500 m to 5 km |
| `site.terrain.*` | — | — | superseded by ADR-056: every side is `bearing_degrees`, the coast is `{bearing_degrees, shape, shore}`, hills have `height_metres`, and `hollows[]` is added |
| `site.terrain.coast` | `{side, kind}` or null | no | kind: bay, open, strait, estuary |
| `site.terrain.cliffs[]` | `{side, height_metres}` | no | |
| `site.terrain.hills[]` | `{at, radius_metres}` | no | `at` is a compass region of the site; `centre` is the site centre (the town core does not exist yet in S0) |
| `site.terrain.slope` | `{toward, metres_per_kilometre}` or null | no | |
| `site.terrain.plateau` | `{side, height_metres}` or null | no | Bearna needs this at milestone 9 |
| `site.water.rivers[]` | `{identifier, from, to, width_metres}` | no | from and to are compass regions except `centre`; a river exits into coast if present |
| `site.water.lakes[]` | `{identifier, at, radius_metres}` | no | |
| `site.water.harbour` | `{identifier, side, length_metres}` or null | no | requires coast or a river on that side; superseded by ADR-057: `bearing_degrees` instead of `side` |
| `site.water.tidal` | bool | no, default false | render only |
| `site.water.canals[]` | `{identifier, from, to, width_metres}` | no | version 1 parses, generator ignores until milestone 5 |
| `site.land_cover.outside[]` | enum list | no | farmland, orchards, heath, rock, marsh, forest |
| `site.land_cover.inside[]` | enum list | no | gardens, parks, groves |
| `site.land_cover.forest`, `.marsh` | `{side, depth_metres}` or null | no | superseded by ADR-058: `bands[] {cover, bearing_degrees, depth_metres}` for any outside cover |
| `roads[]` | `{identifier, ends, importance, river_crossing}` | yes, at least 1 | site features laid out in S0, like rivers. A road has no direction: `ends` is a pair of endpoints, each a compass region except `centre` (that part of the site's edge), or `core`, or the identifier of a pinned landmark or pinned open space (its position is known before S0). Two compass regions make a through road; one compass region and one other end make an ending road; no compass region makes a local road, which lies wholly on the map, inside or outside the walls. The two ends differ and at most one is `core`. S0 lays roads out in three steps: first every road without a `core` end; then the core (§4, town core); then every road with a `core` end. A road with a `core` end ends at the core; S3 links the core to every market square with a main street. importance: major, minor. `river_crossing`: bridge or ford, default bridge; where the road crosses a river, S0 creates that crossing with identifier `<road>.<river>`, or `<road>.<river>.<n>` numbered from 1 along the road from its first listed end when it crosses that river more than once. |
| `defences.walls[]` | `{identifier, kind, encloses[], along_water}` | no | kind: palisade, stone, razed (the line of a torn-down wall); the wall count F2 asks for is the length of this list. `encloses` lists district identifiers and wall identifiers and is the only source for what lies inside a wall: a district listed under a wall, or under a wall that wall encloses, lies inside it (nesting is transitive); a district listed nowhere lies outside every wall. Two walls that do not list each other are separate areas side by side; a district lies in at most one innermost wall. Where the enclosed districts touch coast, river or lake the boundary follows the water, so a wall can run from water to water; `along_water` is `wall` (a sea or river wall, default) or `open` (the water alone). |
| `defences.citadel` | `{identifier, side, inside}` or null | no | `inside` is a wall identifier; `side` is a compass side of that wall's area. To grow a ward around a keep, use a pinned `defence` landmark instead (districts see only pinned objects). |
| `defences.gates[]` | tagged by `kind` | no | A gate stands wherever a road crosses a wall (F9), listed or not. `road_gate {identifier, road, wall, end}` only names such a gate so it can be a target. `end` is optional; when given it is one of the road's own `ends` values and picks, of the points where the road crosses that wall, the one closest to that end. How often a road crosses a wall is known only from the geometry, so a road that crosses the wall more than once without `end`, or never, stops generation with exit 3 naming the gate, the road and the wall. `postern {identifier, wall, side}` and `water_gate {identifier, wall, side}` are gates without a road; `side` is a compass side of the named wall. |
| `open_space[]` | `{identifier, kind, constraints[], resolved, pinned}` | no | kind: market_square, plaza, green, parade_ground. An unpinned open space draws its size and shape from its kind's range in the default rule pack: market_square and plaza 2,500 to 11,250 m², aspect 1 : 1 to 2 : 1; green 80 to 100 m a side; parade_ground 90 m square. A Spanish-colonial pack gives plaza the 1573 Ordinances range (84 to 223 m long, at least 1.5 : 1). |
| `crossings[]` | `{identifier, river, kind, constraints[], resolved, pinned}` | no | kind: bridge, ford. An unpinned crossing's footprint is 8 m wide (the road width once street widths are decided, cross-analysis group C) and as long as the river's `width_metres`, plus 5 m on each bank for a bridge. These numbers are Claude's estimate without a source, kept as default-pack data. Lists crossings placed by rules. Where a road crosses a river, a crossing of the road's `river_crossing` kind exists without being listed; it is created in S0, its position is fixed by the courses, and it is not a constraint target in version 1. |
| `districts[]` | `{identifier, name, function, wealth, group, enclosure, jurisdiction, weight, constraints[], notes}` | yes, 1 to 100 | `function` is a closed enum (§3); `wealth` poor, middling, rich, default middling; `group` an optional free identifier that rule packs may read (for example `jotun`); `enclosure` open, gated, walled, default open, render only in version 1 (a district boundary line drawn heavier; no gates, not a constraint target); `jurisdiction` town or separate, default town; `name` is the legend label; `notes` is optional, default "", ignored by the generator |
| `landmarks[]` | `{identifier, name, function, label, importance, footprint, constraints[], resolved, pinned, notes}` | no, 0 to 1000 | `function` is a closed enum (§3); `label` is an optional free word (`cathedral`, `pagoda`) shown in the legend and matched against loaded rule packs (§3); `importance` minor or major, default minor, the same ordered list as roads; F10 links every major landmark to the main streets |
| `landmarks[].footprint` | `{class, width_metres, depth_metres}`, each part optional; whole field optional | no | resolved in order: the function's default class and shape (§3), then a known label's pack defaults, then `class`, then `width_metres` and `depth_metres` each override one side. `depth_metres` is the long side of the shape ratio. Without `width_metres` and `depth_metres`, the area is drawn within the class range from the landmark's own random stream (§3); with one of them, the other follows from the shape ratio; any written side may put the area outside the class range, because written sizes win. Core only ever sees a width and a depth. |
| `*.resolved` | `{x, y, rotation_degrees, width_metres, depth_metres}` or null | no | On landmarks, open spaces and crossings. The centre in metres in the site frame, origin south-west corner, plus the size, so a pin keeps its size. |
| `*.pinned` | bool | no, default false | On landmarks, open spaces and crossings. true requires `resolved`, else validation error. Without `pinned`, `resolved` is ignored unless `presets.reuse_positions` is on. |
| `presets.era` | enum | yes | early_medieval, high_medieval, late_medieval |
| `presets.style` | string | no, default "ink" | resolved per ARCHITECTURE.md §6 |
| `presets.packs[]` | string list | no, default [] | |
| `presets.reuse_positions` | bool | no, default false | When true, every unpinned landmark, open space and crossing with a `resolved` placement gets one extra soft rule, `near_point` with that position, `within_metres` from `presets.reuse_within_metres` and weight from `presets.reuse_weight`. `--reuse-positions` and `--no-reuse-positions` override it for one run. |
| `presets.reuse_within_metres` | number | no, default 50 | `within_metres` of the `near_point` rule, in metres: the distance at which the soft rule counts as met, not a hard bound; must be positive |
| `presets.reuse_weight` | number | no, default 1 | weight of the `near_point` rule; must be positive |
| `presets.centre_radius_metres` | number | no, default 150 | radius around the town core that counts as the compass region `centre`; must be positive |

## 3. Closed enums and defaults

- Compass side (superseded for terrain and water by `Bearing`, ADR-056): north, east, south, west. Compass region: north, north_east, east,
  south_east, south, south_west, west, north_west, centre. As a road or river end, a region
  is that part of the site's edge. For the `compass_region` rule it is measured from the town
  core: `centre` is everything within `presets.centre_radius_metres` of the core, and the other
  eight are 45° sectors around the core, `north` centred on north; a sector boundary belongs
  to the sector clockwise from it.
- District function: residential, craft, works (noxious or water-bound industry), commerce,
  harbour, seat_of_power, military, religious_precinct, farmland, cemetery, open_ground.
  Adding one is a schema change. Culture-specific quarters (samurai quarter, mellah, ward) are
  rule-pack mappings onto a function plus attributes.
- Landmark function, with its default footprint class and default shape (length : width). This table
  is data in the default rule pack that every specification loads, not part of the schema; any pack
  can override a row. The `production` row is Claude's estimate without a source (no workshop
  measurements in the research, cross-analysis footprints.md gap 5), re-checked when facts exist
  (#74, #77):

  | function | examples | default class | default shape |
  |---|---|---|---|
  | religious | church, temple, mosque, shrine | large | 2.5 : 1 |
  | defence | castle, keep, tower, gatehouse | large | 1.3 : 1 |
  | seat_of_power | palace, government house | huge | 1.3 : 1 |
  | civic | town hall, guildhall, courthouse | medium | 2 : 1 |
  | commerce | market hall, warehouse, exchange | small | 2 : 1 |
  | lodging | inn, caravanserai | medium | 1.2 : 1 |
  | learning | academy, madrasa, college | large | 1.4 : 1 |
  | entertainment | theatre, arena | medium | 1 : 1 |
  | health | hospital, bathhouse, leper house | medium | 2 : 1 |
  | utility | well, fountain, mill, lighthouse | tiny | 1 : 1 |
  | military | arsenal, barracks | huge | 3 : 1 |
  | monument | column, market cross, statue | tiny | 1 : 1 |
  | production | smithy, carpenter, wheelwright, mill, tannery, brewery, ropewalk | small | 1.5 : 1 |

- Footprint class: nominal square side and the area range it draws from. Ranges are
  half-open (a class includes its lower bound and excludes its upper bound); the draw is
  uniform in the logarithm of the area, then split into width and depth by the shape. The
  model keeps the real size; the renderer draws every landmark at no less than a minimum
  drawn size for the chosen scale, a settable value whose default is chosen when the
  renderer is built, so a 2 m well stays visible on an overview map.

  | class | nominal side | area range (m²) |
  |---|---|---|
  | micro | 2 m | 1 to 9 |
  | tiny | 6 m | 9 to 80 |
  | small | 12 m | 80 to 300 |
  | medium | 25 m | 300 to 1,200 |
  | large | 50 m | 1,200 to 5,000 |
  | huge | 100 m | 5,000 to 20,000 |

  Sources: `documentation/research/settlement/landmark-footprints.md` and its verification. A walled
  compound of 20,000 m² and over (an abbey, a temple complex, a naval arsenal) is a precinct,
  drawn as an area of the city; precincts are a future idea, not version 1.
- Labels: the default rule pack lists exactly `cathedral`, `guildhall`, `lighthouse`,
  `shrine` and `pagoda` in version 1; Jeffrey adds labels to it in a PR. A rule pack may list
  labels with a function, a default class and a default shape
  (for example `cathedral`: religious, huge, 2 : 1; `pagoda`: religious, small, 1 : 1). A
  landmark whose `label` matches a loaded pack takes those defaults; its own `footprint`
  fields still win. A label no loaded pack knows is shown in the legend only, and the
  generator prints a warning naming the label, so a typo surfaces.

## 4. Constraints, version 1 (REQUIREMENTS.md F3)

| rule | parameters | target type | meaning |
|---|---|---|---|
| `compass_region` | `region` | none | the candidate's centre lies in that region, measured from the town core (§3) |
| `near` | `target`, optional `within_metres` (default 150) | any positioned specification identifier | some part of the candidate's footprint within the distance of the target |
| `far_from` | `target`, optional `beyond_metres` (default 300) | any positioned specification identifier | every part of the candidate's footprint beyond the distance |
| `on_water` | none | none | candidate touches coast, river, lake or harbour |
| `on_high_ground` | none | none | candidate elevation in the top quartile of the site |
| `inside_walls` | `target` | wall identifier | candidate inside the area that wall encloses; landmarks, open spaces and crossings only (districts use `encloses`) |
| `outside_walls` | `target` | wall identifier | candidate outside that area; landmarks, open spaces and crossings only |
| `in_district` | `target` | district identifier | candidate inside that district |
| `min_spacing` | `metres` | none | at least that gap between the candidate's footprint and every already-placed landmark, open space and crossing |
| `near_point` | `x`, `y`, `within_metres` | none | added by the generator only, for `reuse_positions`; a specification file that writes it is a validation error |

A positioned specification identifier is the identifier of a landmark, district, open space,
crossing, river, lake, harbour, road, wall, gate or citadel, or the keyword `core` (the town core,
laid out in S0). A list of constraints applies all of them together: "near the river, near the north
gate, inside the outer wall" is three entries.

Distances are measured edge to edge: the shortest gap between the candidate's footprint, with
its rotation, and the target's geometry, which is 0 when they touch. Rivers, roads and walls
are lines; districts, lakes and harbours are areas; landmarks, open spaces and crossings are
their footprints; gates and the citadel are their outlines. Whether a soft rule scores
pass/fail or falls off gradually beyond the distance is decided by Jeffrey in the ten-line
design of the S2 placement step (issue #49).

Visibility: a rule, a road end or a gate may point only to something that exists when its
own object is laid out: something from an earlier stage (S0: rivers, lakes, harbour, roads,
the core; S1, in this order: districts, then walls from `encloses`, then gates, then the
citadel), something earlier in the dependency order of the same stage, or something pinned. Anything else is a validation error naming both identifiers
("pin it first, or point to something laid out earlier"). A pinned object's rules are checked
once everything they point to exists; a broken hard rule is a constraint failure, exit 3
(F7, F8).

Town core: the candidates are the points where roads cross rivers or each other, and the
points where a road meets the harbour. The core is the candidate whose more important road
ranks highest; among equally important roads a road–river or road–harbour candidate comes
before a road–road crossing; then the one closest to the site centre; identifier order (the
alphabetical order of names, ADR-049) breaks only
an exact tie. When the river-or-harbour rule decided between equally important roads, the
generator prints a notice naming both candidates, because a finer importance level (#48) might
express the intended town better. With no such crossing, the core is the point closest to the
site centre on any road of the highest importance present; the default rule pack widens that
road around the core into a street market. A better representation of linear towns is issue
#76 (ADR-021 seventh round). The one-road rule above uses only roads from step one. When every
road ends at `core` (step one is empty), the core is the river point closest to the site
centre, or the site centre when there is no river (ADR-021 fifth round, Jeffrey's choice).

Every constraint has `hard` (default false) and `weight` (default 1, ignored when hard; must
be positive).

The YAML is flat: `{ rule: near, target: market, hard: false, weight: 2 }`. In Rust the
constraint is one closed enum, internally tagged by `rule`, and every variant carries its
own parameters plus `hard` and `weight` (ADR-017, ADR-021). `#[serde(flatten)]` is not used,
because with `deny_unknown_fields` it rejects every valid rule; a scratch test on
2026-10-20 (serde 1.0.229, serde_yaml_ng 0.10) showed this, and showed that the
per-variant shape rejects typos with messages that list the valid fields.

Candidates the scoring code works on: ward, landmark, open space, crossing. A district's
rules are checked on wards, when S1 decides which ward gets which district; by the visibility
rule they may point only to S0 objects, earlier districts and pinned objects. Walls, gates
and the citadel are made from the districts, so a district rule cannot point to them.

## 5. Validation errors the example must trigger in tests

One unit test per message, each naming the field path:
- missing `schema`; `schema: 2`.
- unknown field anywhere (`deny_unknown_fields`).
- duplicate identifier within a list; the same identifier in two different lists (for example a
  district and an open space); a constraint target that does not exist.
- `pinned: true` without `resolved`.
- a pinned landmark, open space or crossing whose resolved position or footprint lies
  outside the site (0 to `extent_metres` on both axes); checked in `citymap-command-line`, not in core.
- a road whose two `ends` are equal; a road with both ends `core`; an end that is `centre`;
  an end naming a landmark that is not pinned (the message says "pin it first").
- a `road_gate` with an `end` that is not one of that road's `ends`; a `road_gate`,
  `postern` or `water_gate` whose `road` or `wall` names a missing identifier or an identifier of
  another type; a citadel `inside`, a crossing `river` or a wall `encloses` entry that names a
  missing identifier or an identifier of another type; a road end naming a missing identifier.
- a constraint target of the wrong type (`inside_walls: market`); a rule, road end or gate
  that breaks the visibility rule (§4); two or more objects whose rules target each other in
  a ring (F8), naming all of them.
- a district listed under two walls that do not enclose each other; `inside_walls` or
  `outside_walls` on a district; a wall that encloses itself, directly or in a ring.
- an identifier equal to `core`, `centre` or a compass region name, or containing `.` (reserved for
  generated crossing identifiers such as `north_road.ash`).
- a `centre_radius_metres` that is zero or negative; a constraint `weight` that is zero or
  negative; a `radius_metres`, `width_metres`, `length_metres`, `depth_metres` or `height_metres`
  that is zero or negative.
- `identity.name` empty or over 80 characters; `population` over 1,000,000; no districts; no
  roads; a river `from` or `to` equal to `centre`.
- a road end naming an identifier that is neither a landmark nor an open space; a road end naming a
  landmark or open space that is not pinned ("pin it first").
- a district rule pointing to a wall, gate or citadel (visibility).
- the unknown-label warning (a warning, not an error; tested on stderr).
- the population warning: a `settlement.population` outside 1,000 to 200,000, the range the
  population-to-area law was fitted on; the generator extrapolates and names the range
  (density.md question 4).
- the core notice when the river-or-harbour rule decided between equally important roads.
- both `--reuse-positions` and `--no-reuse-positions` on one command line (CLI test, exit 2).
- a harbour on a side with no coast or river.
- `extent_metres` outside 500 to 5000; `population` zero; NaN anywhere.
- a footprint `width_metres` or `depth_metres` that is zero or negative.
- a negative `within_metres`, `beyond_metres` or `min_spacing` `metres`; a `reuse_within_metres` or
  `reuse_weight` that is zero or negative.
- a `near_point` rule written in a specification file.
- more than 100 districts or 1000 landmarks.

Generation-time failures (exit 3, F8), each with a test: a `road_gate` whose road crosses its
wall more than once without `end`, or never; a pinned object that breaks a hard rule (F7, exit 3 per F8);
no candidate satisfying every hard rule within `MAX_PLACEMENT_RINGS` (F8).

## 6. Jeffrey's answers, 2026-10-20

1. `settlement.level` and `settlement.size_class` are dropped; `population` is the only size
   input. Status words such as "town" carry culture-specific presumptions (a parish church is
   European), so features are written out explicitly instead.
2. `roads[].destination` is dropped.
3. Footprint classes stay as defaults; a landmark may override the class, the width or the
   depth.
4. The `fixed` rule is dropped; `pinned` is the only way to place exactly.
5. F3 is widened to every positioned specification identifier (§4).
6. Roads are site features with a course, laid out in S0 like rivers. Road courses are fixed
   in version 1; the town core forms where a road crosses a river or another road. Through
   roads keep their course and become main streets; ending roads end at the core, or at the
   market square when the specification has one (superseded by item 28). A town without a market square holds its market in
   the street. Roads that bend toward a site placed first are a future idea (issue #31).
7. An unpinned `resolved` is ignored by default; `presets.reuse_positions` (with a
   command-line override) turns it into a soft `near_point` rule. Per-object soft pinning is
   a future idea (issue #40).
8. Constraints are flat in YAML, one closed enum with per-variant `hard` and `weight` in Rust,
   no macro. A two-step reader comes back only when Claude shows in a PR that it removes more
   code than it adds; Jeffrey decides.
9. Crossings are a fourth candidate kind; district rules are checked on wards.
10. Landmarks, open spaces and crossings can be pinned. Pinning a wall shape and pinning
    listed gates are future ideas (issues #41, #42).
11. Districts are one closed list of eleven functions plus `wealth`, `group`, `enclosure` and
    `jurisdiction`, at one level. Mixed districts, special per-city attributes and recursive
    nesting are future ideas (issues #43, #44, #45).
12. Landmark `kind` becomes a closed list of functions plus an optional free `label`; rule
    packs map labels to defaults, and an unknown label warns.
13. Footprint classes from micro to huge with default shapes (`micro` was added in
    item 31); the size within a class is
    drawn by seed, and `resolved` stores width and depth so pins keep their size. Precincts
    are a future idea.
14. With several road–river or road–road crossings, the core is the crossing on the most important
    road, then the one closest to the site centre; identifier order breaks only an exact tie, so
    renaming an identifier never moves the town. A specification with no such crossing gets its town
    core on the most important road, at the point closest to the site centre (one-road towns exist).
15. Landmarks get `importance`, minor or major, the same ordered list as roads; size never
    implies importance (the Western Wall is small but major). More levels are a future idea
    (issue #48).
16. Where a road crosses a river, S0 creates a crossing with identifier `<road>.<river>` (or
    `<road>.<river>.<n>`, numbered from 1 along the road from its first listed end, when the
    road crosses that river more than once); its kind is
    the road's `river_crossing`, bridge by default, ford when the specification says so.
17. `near`, `far_from` and `min_spacing` measure edge to edge, from the candidate's footprint
    to the target's geometry; `min_spacing` counts landmarks, open spaces and crossings.
18. `presets.reuse_within_m` (default 50) and `presets.reuse_weight` (default 1) make the soft
    pin settable instead of fixed numbers.
19. The function-to-footprint table is data in the default rule pack, overridable by packs.
20. Roads have no direction: `ends: [a, b]`; an end is a compass region, `core`, or a pinned
    landmark or open space (item 28); a road gate names its end by value (`end: north`). Roads to unpinned landmarks
    and routes through several stops are a future idea (issue #31).
21. Placement follows dependencies: an object is placed after everything its rules target;
    mutual targets are a validation error.
22. Roads are laid out in three steps within S0: every road without a `core` end (through
    roads, roads to pinned landmarks or open spaces, local roads), then the core, then roads
    with a `core` end. Roads between two places on the map (core, pinned landmarks or pinned
    open spaces, inside or outside the walls) are local roads. The rejected orderings (core-
    ending roads aimed at the site centre; no core-ending roads; several passes) are in
    ADR-021, fifth round, with their reasons.
23. One visibility rule replaces case-by-case fixes: a rule, road end or gate points only to
    something laid out earlier or pinned (§4). Generation order from a dependency graph and
    growth phases are future ideas (issues #50, #51).
24. `compass_region` is measured from the town core: eight 45° sectors and `centre` within
    `presets.centre_radius_m`. An absolute map grid is a future idea (issue #52).
25. `encloses` lists districts and walls and is the only source for what lies inside a wall;
    nesting is transitive; separate walls stand side by side; the boundary follows water,
    with `along_water: wall | open`.
26. Adding an object never moves objects of its own kind or of earlier kinds: a new landmark
    moves nothing, a new open space can move landmarks, a new crossing can move open spaces
    and landmarks. Pinned objects never move.
27. A gate's `end` is optional and picks the crossing closest to that end; a road crossing its
    wall more than once without `end`, or never, stops generation with a message.
28. A road with a `core` end ends at the core, and S3 links the core to every market square
    with a main street; a road end may also name a pinned open space, such as a pinned market.
29. District rules see only S0 objects, earlier districts and pinned objects, because walls,
    gates and the citadel are made from the districts. A ward around a keep uses a pinned
    `defence` landmark. Running the stages in several passes is a future idea (issue #51).
30. Group A of the cross-analysis (documentation/research/cross-analysis/README.md), 2026-10-21:
    among equally important roads a river or harbour candidate comes before a road–road
    crossing, with a notice; a road meeting the harbour is a core candidate; castle-first
    towns wait for #31; the 150 m centre radius stays until the minor-town series (#27)
    measures real cores; `core` is a constraint target and the default pack gives the market
    a soft `near: core`; a one-road town keeps a point core with a street market (#76).
31. Footprints, 2026-10-21: no new labels in version 1 (subtypes are #77); `cathedral`
    becomes 2 : 1; `commerce` becomes small; a `micro` class (1 to 9 m²) with a minimum drawn
    size in the renderer; open spaces get size ranges per kind in the default pack.
32. A population outside 1,000 to 200,000 is extrapolated with a warning.
33. A thirteenth landmark function, `production`, for named workshops (default small
    1.5 : 1, an estimate without a source); ordinary workshops stay in `craft` districts.

Recorded in ADR-021.
