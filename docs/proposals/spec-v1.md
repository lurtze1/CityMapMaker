# Proposal: spec schema version 1 and the example city

Status: accepted by Jeffrey on 2026-10-20 with the changes recorded in ADR-021 and §6. The
example city is fictional and exists only to exercise every field; it is not campaign lore
(ADR-016).

## 1. The example spec

```yaml
schema: 1

identity:
  name: Harrowmere
  seed: 20261019
  notes: >
    Fictional river port used by the test suite. Everything below is invented to touch every
    field once; no campaign lore.

settlement:
  population: 6000

site:
  extent_m: 2400             # side of the square site in metres; the city must fit inside
  terrain:
    coast: { side: east, kind: bay }              # side: north|east|south|west|none
    cliffs: []                                    # list of { side, height_m }
    hills: [{ at: north_west, radius_m: 500 }]
    slope: { toward: east, metres_per_km: 8 }
    plateau: null
  water:
    rivers:
      - { id: ash, from: north_west, to: east, width_m: 18 }
    lakes: []
    harbour: { id: quay, side: east, length_m: 300 }
    tidal: true
    canals: []
  land_cover:
    outside: [farmland, orchards]
    inside: [gardens]
    forest: { side: north, depth_m: 400 }
    marsh: { side: south_east, depth_m: 150 }

roads:
  - { id: north_road, from: north, to: south, importance: major }    # through road
  - { id: west_road, from: west, to: core, importance: minor }       # ends at the town core

defences:
  walls:
    - { id: outer, kind: stone, encloses: [old_town, craft, market_q, castle] }
  citadel: { id: keep, side: north, inside: outer }
  gates:
    - { kind: road_gate, id: north_gate, road: north_road, wall: outer, end: from }
    - { kind: water_gate, id: quay_gate, wall: outer, side: east }

open_space:
  - id: market
    kind: market_square
    constraints: [{ rule: near, target: ash, hard: false, weight: 2 }]
    resolved: null
    pinned: false
  - id: green
    kind: green
    constraints: [{ rule: in_district, target: old_town, hard: true }]

crossings:
  - id: ash_bridge
    river: ash
    kind: bridge
    constraints: [{ rule: near, target: market, hard: false, weight: 3 }]
    resolved: { x: 1402.00, y: 1150.00, rotation_deg: 90, width_m: 8, depth_m: 30 }
    pinned: true

districts:
  - id: old_town
    name: Old Town
    function: residential
    wealth: middling
    weight: 3
    constraints: [{ rule: inside_walls, target: outer, hard: true }]
    notes: oldest part, around the market
  - id: craft
    name: Craftsmen's Quarter
    function: craft
    weight: 2
    constraints:
      - { rule: inside_walls, target: outer, hard: true }
      - { rule: compass_region, region: north_east, hard: true }
  - id: market_q
    name: Merchants' Quarter
    function: commerce
    wealth: rich
    weight: 2
    constraints: [{ rule: near, target: market, hard: false, weight: 2 }]
  - id: castle
    name: The Keep Ward
    function: seat_of_power
    enclosure: walled
    jurisdiction: separate
    weight: 1
    constraints: [{ rule: near, target: keep, hard: true }]
  - id: docks
    name: The Docks
    function: harbour
    group: river_folk
    weight: 2
    constraints:
      - { rule: on_water, hard: true }
      - { rule: outside_walls, target: outer, hard: true }
  - id: tanneries
    name: Tanners' Row
    function: works
    wealth: poor
    weight: 1
    constraints:
      - { rule: on_water, hard: true }
      - { rule: outside_walls, target: outer, hard: true }
  - id: graves
    name: St Ash Field
    function: cemetery
    weight: 1
    constraints: [{ rule: near, target: west_road, hard: false, weight: 1 }]
  - id: commons
    name: Harrow Common
    function: open_ground
    weight: 1
    constraints: [{ rule: outside_walls, target: outer, hard: true }]
  - id: fields
    name: Outer Fields
    function: farmland
    weight: 1
    constraints: [{ rule: outside_walls, target: outer, hard: true }]

landmarks:
  - id: temple
    name: Temple of the Tide
    function: religious
    label: cathedral
    constraints:
      - { rule: compass_region, region: north_east, hard: true }
      - { rule: on_high_ground, hard: false, weight: 2 }
    resolved: null
    pinned: false
    notes: ""
  - id: guildhall
    name: Weavers' Hall
    function: civic
    label: guildhall
    footprint: { class: large, depth_m: 60 }
    constraints:
      - { rule: in_district, target: craft, hard: true }
      - { rule: near, target: north_gate, hard: false, weight: 2 }
    resolved: { x: 1310.25, y: 880.50, rotation_deg: 0, width_m: 30, depth_m: 60 }
    pinned: true
    notes: pinned to test that pins survive a seed change
  - id: lighthouse
    name: The Harrow Light
    function: utility
    label: lighthouse
    footprint: { class: small }
    constraints:
      - { rule: in_district, target: docks, hard: true }
      - { rule: far_from, target: temple, hard: false, weight: 1 }
      - { rule: min_spacing, metres: 40, hard: true }
    resolved: { x: 2210.00, y: 1300.00, rotation_deg: 0, width_m: 11, depth_m: 11 }
    pinned: false
    notes: an unpinned resolved placement, used only when reuse_positions is on

presets:
  era: early_medieval         # early_medieval | high_medieval | late_medieval
  style: ink
  packs: []
  reuse_positions: false      # `--reuse-positions` / `--no-reuse-positions` override per run
```

## 2. The schema this implies

Types below are what `citymap-cli` parses into and `citymap-core` consumes. Every struct is
`deny_unknown_fields`; every enum is a closed list; every id is unique across the whole
file, not only within its list, so a constraint target such as `near: market` names exactly
one thing; constraint targets refer to ids in this file only, and only to ids of things with
a position (REQUIREMENTS.md F3, §4 below). Every enum is internally tagged (REQUIREMENTS.md
F1): a variant with fields is written as a map whose first key is its tag, `kind` for gates
and `rule` for constraints. Jeffrey accepted this section on 2026-10-20 with the answers in §6.

| Field | Type | Required | Notes |
|---|---|---|---|
| `schema` | integer, must be 1 | yes | first key; anything else is exit 2 |
| `identity.name` | string, 1 to 80 chars | yes | |
| `identity.seed` | u64 | yes | `--seed` overrides for one run |
| `identity.notes` | string | no, default "" | ignored by the generator |
| `settlement.population` | integer 1 to 1,000,000 | yes | drives built area with the era preset; the only size input |
| `site.extent_m` | number 500 to 5000 | yes | REQUIREMENTS.md non-goal: 500 m to 5 km |
| `site.terrain.coast` | `{side, kind}` or null | no | kind: bay, open, strait, estuary |
| `site.terrain.cliffs[]` | `{side, height_m}` | no | |
| `site.terrain.hills[]` | `{at, radius_m}` | no | `at` is a compass point or `centre` |
| `site.terrain.slope` | `{toward, metres_per_km}` or null | no | |
| `site.terrain.plateau` | `{side, height_m}` or null | no | Bearna needs this at milestone 9 |
| `site.water.rivers[]` | `{id, from, to, width_m}` | no | from and to are compass regions except `centre`; a river exits into coast if present |
| `site.water.lakes[]` | `{id, at, radius_m}` | no | |
| `site.water.harbour` | `{id, side, length_m}` or null | no | requires coast or a river on that side |
| `site.water.tidal` | bool | no, default false | render only |
| `site.water.canals[]` | `{id, from, to, width_m}` | no | version 1 parses, generator ignores until milestone 5 |
| `site.land_cover.outside[]` | enum list | no | farmland, orchards, heath, rock, marsh, forest |
| `site.land_cover.inside[]` | enum list | no | gardens, parks, groves |
| `site.land_cover.forest`, `.marsh` | `{side, depth_m}` or null | no | |
| `roads[]` | `{id, from, to, importance}` | yes, at least 1 | site features laid out in S0, like rivers. `from` is a compass region except `centre`. `to` is another compass region except `centre` (a through road) or `core` (a road that ends at the town core). importance: major, minor. |
| `defences.walls[]` | `{id, kind, encloses[]}` | no | kind: palisade, stone, razed; `encloses` lists district ids; the wall count F2 asks for is the length of this list, there is no separate count field |
| `defences.citadel` | `{id, side, inside}` or null | no | `inside` is a wall id |
| `defences.gates[]` | tagged by `kind` | no | A gate stands wherever a road crosses a wall (F9), listed or not. `road_gate {id, road, wall, end}` only names such a gate so it can be a target; `end` is `from` or `to`, required for a through road, which crosses a ring twice. `postern {id, wall, side}` and `water_gate {id, wall, side}` are gates without a road. |
| `open_space[]` | `{id, kind, constraints[], resolved, pinned}` | no | kind: market_square, plaza, green, parade_ground |
| `crossings[]` | `{id, river, kind, constraints[], resolved, pinned}` | no | kind: bridge, ford. Lists crossings placed by rules. Where a road crosses a river, a bridge exists without being listed. |
| `districts[]` | `{id, name, function, wealth, group, enclosure, jurisdiction, weight, constraints[], notes}` | yes, 1 to 100 | `function` is a closed enum (§3); `wealth` poor, middling, rich, default middling; `group` an optional free id that rule packs may read (for example `jotun`); `enclosure` open, gated, walled, default open; `jurisdiction` town or separate, default town; `name` is the legend label; `notes` is optional, default "", ignored by the generator |
| `landmarks[]` | `{id, name, function, label, footprint, constraints[], resolved, pinned, notes}` | no, 0 to 1000 | `function` is a closed enum (§3); `label` is an optional free word (`cathedral`, `pagoda`) shown in the legend and matched against loaded rule packs (§3) |
| `landmarks[].footprint` | `{class, width_m, depth_m}`, each part optional; whole field optional | no | resolved in order: the function's default class and shape (§3), then a known label's pack defaults, then `class`, then `width_m` and `depth_m` each override one side. Without `width_m` and `depth_m`, the area is drawn within the class range from the landmark's own random stream (§3). Core only ever sees a width and a depth. |
| `*.resolved` | `{x, y, rotation_deg, width_m, depth_m}` or null | no | On landmarks, open spaces and crossings. The centre in metres in the site frame, origin south-west corner, plus the size, so a pin keeps its size. |
| `*.pinned` | bool | no, default false | On landmarks, open spaces and crossings. true requires `resolved`, else validation error. Without `pinned`, `resolved` is ignored unless `presets.reuse_positions` is on. |
| `presets.era` | enum | yes | early_medieval, high_medieval, late_medieval |
| `presets.style` | string | no, default "ink" | resolved per ARCHITECTURE.md §6 |
| `presets.packs[]` | string list | no, default [] | |
| `presets.reuse_positions` | bool | no, default false | When true, every unpinned landmark, open space and crossing with a `resolved` placement gets one extra soft rule, `near_point` with that position, `within_m` 50 and weight 1. `--reuse-positions` and `--no-reuse-positions` override it for one run. |

## 3. Closed enums and defaults

- Compass side: north, east, south, west. Compass region (for `compass_region`): north,
  north_east, east, south_east, south, south_west, west, north_west, centre.
- District function: residential, craft, works (noxious or water-bound industry), commerce,
  harbour, seat_of_power, military, religious_precinct, farmland, cemetery, open_ground.
  Adding one is a schema change. Culture-specific quarters (samurai quarter, mellah, ward) are
  rule-pack mappings onto a function plus attributes.
- Landmark function, with its default footprint class and default shape (length : width):

  | function | examples | default class | default shape |
  |---|---|---|---|
  | religious | church, temple, mosque, shrine | large | 2.5 : 1 |
  | defence | castle, keep, tower, gatehouse | large | 1.3 : 1 |
  | seat_of_power | palace, government house | huge | 1.3 : 1 |
  | civic | town hall, guildhall, courthouse | medium | 2 : 1 |
  | commerce | market hall, warehouse, exchange | medium | 2 : 1 |
  | lodging | inn, caravanserai | medium | 1.2 : 1 |
  | learning | academy, madrasa, college | large | 1.4 : 1 |
  | entertainment | theatre, arena | medium | 1 : 1 |
  | health | hospital, bathhouse, leper house | medium | 2 : 1 |
  | utility | well, fountain, mill, lighthouse | tiny | 1 : 1 |
  | military | arsenal, barracks | huge | 3 : 1 |
  | monument | column, market cross, statue | tiny | 1 : 1 |

- Footprint class: nominal square side and the area range it draws from. Ranges are
  half-open (a class includes its lower bound and excludes its upper bound); the draw is
  uniform in the logarithm of the area, then split into width and depth by the shape.

  | class | nominal side | area range (m²) |
  |---|---|---|
  | tiny | 6 m | 9 to 80 |
  | small | 12 m | 80 to 300 |
  | medium | 25 m | 300 to 1,200 |
  | large | 50 m | 1,200 to 5,000 |
  | huge | 100 m | 5,000 to 20,000 |

  Sources: `docs/research/settlement/landmark-footprints.md` and its verification. A walled
  compound of 20,000 m² and over (an abbey, a temple complex, a naval arsenal) is a precinct,
  drawn as an area of the city; precincts are a future idea, not version 1.
- Labels: a rule pack may list labels with a function, a default class and a default shape
  (for example `cathedral`: religious, huge, 2.5 : 1; `pagoda`: religious, small, 1 : 1). A
  landmark whose `label` matches a loaded pack takes those defaults; its own `footprint`
  fields still win. A label no loaded pack knows is shown in the legend only, and the
  generator prints a warning naming the label, so a typo surfaces.

## 4. Constraints, version 1 (REQUIREMENTS.md F3)

| rule | parameters | target type | meaning |
|---|---|---|---|
| `compass_region` | `region` | none | candidate lies in that region of the site |
| `near` | `target`, optional `within_m` (default 150) | any positioned spec id | candidate centre within the distance of the target |
| `far_from` | `target`, optional `beyond_m` (default 300) | any positioned spec id | the opposite |
| `on_water` | none | none | candidate touches coast, river, lake or harbour |
| `on_high_ground` | none | none | candidate elevation in the top quartile of the site |
| `inside_walls` | `target` | wall id | candidate inside that wall ring |
| `outside_walls` | `target` | wall id | candidate outside that wall ring |
| `in_district` | `target` | district id | candidate inside that district |
| `min_spacing` | `metres` | none | no other landmark centre within the distance |
| `near_point` | `x`, `y`, `within_m` | none | added by the generator only, for `reuse_positions`; a spec file that writes it is a validation error |

A positioned spec id is the id of a landmark, district, open space, crossing, river, lake,
harbour, road, wall, gate or citadel. A list of constraints applies all of them together:
"near the river, near the north gate, inside the outer wall" is three entries.

Every constraint has `hard` (default false) and `weight` (default 1, ignored when hard).

The YAML is flat: `{ rule: near, target: market, hard: false, weight: 2 }`. In Rust the
constraint is one closed enum, internally tagged by `rule`, and every variant carries its
own parameters plus `hard` and `weight` (ADR-017, ADR-021). `#[serde(flatten)]` is not used,
because with `deny_unknown_fields` it rejects every valid rule; a scratch test on
2026-10-20 (serde 1.0.229, serde_yaml_ng 0.10) showed this, and showed that the
per-variant shape rejects typos with messages that list the valid fields.

Candidates the scoring code works on: ward, landmark, open space, crossing. A district's
rules are checked on wards, when S1 decides which ward gets which district.

## 5. Validation errors the example must trigger in tests

One unit test per message, each naming the field path:
- missing `schema`; `schema: 2`.
- unknown field anywhere (`deny_unknown_fields`).
- duplicate id within a list; the same id in two different lists (for example a district and
  an open space); a constraint target that does not exist.
- `pinned: true` without `resolved`.
- a pinned landmark, open space or crossing whose resolved position or footprint lies
  outside the site (0 to `extent_m` on both axes); checked in `citymap-cli`, not in core.
- a through road whose `from` equals its `to`; a road whose `from` is `core` or `centre`; a
  road whose `to` is `centre`.
- a `road_gate` naming a through road without `end`, or an ending road with `end: to`.
- a harbour on a side with no coast or river.
- `extent_m` outside 500 to 5000; `population` zero; NaN anywhere.
- a footprint `width_m` or `depth_m` that is zero or negative.
- a negative `within_m`, `beyond_m` or `min_spacing` `metres`.
- a `near_point` rule written in a spec file.
- more than 100 districts or 1000 landmarks.

## 6. Jeffrey's answers, 2026-10-20

1. `settlement.level` and `settlement.size_class` are dropped; `population` is the only size
   input. Status words such as "town" carry culture-specific presumptions (a parish church is
   European), so features are written out explicitly instead.
2. `roads[].destination` is dropped.
3. Footprint classes stay as defaults; a landmark may override the class, the width or the
   depth.
4. The `fixed` rule is dropped; `pinned` is the only way to place exactly.
5. F3 is widened to every positioned spec id (§4).
6. Roads are site features with a course, laid out in S0 like rivers. Road courses are fixed
   in version 1; the town core forms where a road crosses a river or another road. Through
   roads keep their course and become main streets; ending roads end at the core, or at the
   market square when the spec has one. A town without a market square holds its market in
   the street. Roads that bend toward a site placed first are a future idea (issue #31).
7. An unpinned `resolved` is ignored by default; `presets.reuse_positions` (with a
   command-line override) turns it into a soft `near_point` rule. Per-object soft pinning is
   a future idea (issue #40).
8. Constraints are flat in YAML, one closed enum with per-variant `hard` and `weight` in Rust,
   no macro. A two-step reader is revisited only if it ever removes more code than it adds.
9. Crossings are a fourth candidate kind; district rules are checked on wards.
10. Landmarks, open spaces and crossings can be pinned. Pinning a wall shape and pinning
    listed gates are future ideas (issues #41, #42).
11. Districts are one closed list of eleven functions plus `wealth`, `group`, `enclosure` and
    `jurisdiction`, at one level. Mixed districts, special per-city attributes and recursive
    nesting are future ideas (issues #43, #44, #45).
12. Landmark `kind` becomes a closed list of functions plus an optional free `label`; rule
    packs map labels to defaults, and an unknown label warns.
13. Five footprint classes from tiny to huge with default shapes; the size within a class is
    drawn by seed, and `resolved` stores width and depth so pins keep their size. Precincts
    are a future idea.

Recorded in ADR-021.
