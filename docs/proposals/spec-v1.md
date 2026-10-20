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

defences:
  walls:
    - { id: outer, kind: stone, encloses: [old_town, craft, market_q] }
  citadel: { id: keep, side: north, inside: outer }
  gates:
    - { id: north_gate, wall: outer, side: north }
    - { id: west_gate, wall: outer, side: west }

open_space:
  - { id: market, kind: market_square, constraints: [{ rule: near, target: ash, hard: false, weight: 2 }] }
  - { id: green, kind: green, constraints: [{ rule: in_district, target: old_town, hard: true }] }

crossings:
  - { id: ash_bridge, river: ash, kind: bridge, constraints: [{ rule: near, target: market, hard: false, weight: 3 }] }

districts:
  - id: old_town
    name: Old Town
    type: residential
    weight: 3
    constraints: [{ rule: inside_walls, target: outer, hard: true }]
    notes: oldest part, around the market
  - id: craft
    name: Craftsmen's Quarter
    type: craftsmen
    weight: 2
    constraints:
      - { rule: inside_walls, target: outer, hard: true }
      - { rule: compass_region, region: north_east, hard: true }
  - id: market_q
    name: Merchants' Quarter
    type: merchant
    weight: 2
    constraints: [{ rule: near, target: market, hard: false, weight: 2 }]
  - id: docks
    name: The Docks
    type: harbour
    weight: 2
    constraints:
      - { rule: on_water, hard: true }
      - { rule: outside_walls, target: outer, hard: true }
  - id: fields
    name: Outer Fields
    type: farms
    weight: 1
    constraints: [{ rule: outside_walls, target: outer, hard: true }]

landmarks:
  - id: temple
    name: Temple of the Tide
    kind: temple
    constraints:
      - { rule: compass_region, region: north_east, hard: true }
      - { rule: on_high_ground, hard: false, weight: 2 }
    resolved: null
    pinned: false
    notes: ""
  - id: guildhall
    name: Weavers' Hall
    kind: hall
    footprint: { class: large, depth_m: 60 }
    constraints:
      - { rule: in_district, target: craft, hard: true }
      - { rule: near, target: north_gate, hard: false, weight: 2 }
    resolved: { x: 1310.25, y: 880.50, rotation_deg: 0 }
    pinned: true
    notes: pinned to test that pins survive a seed change
  - id: lighthouse
    name: The Harrow Light
    kind: tower
    footprint: { class: small }
    constraints:
      - { rule: in_district, target: docks, hard: true }
      - { rule: far_from, target: temple, hard: false, weight: 1 }
      - { rule: min_spacing, metres: 40, hard: true }
    resolved: null
    pinned: false
    notes: ""

roads:
  - { id: north_road, side: north, importance: major }
  - { id: west_road, side: west, importance: minor }

presets:
  era: early_medieval         # early_medieval | high_medieval | late_medieval
  style: ink
  packs: []
```

## 2. The schema this implies

Types below are what `citymap-cli` parses into and `citymap-core` consumes. Every struct is
`deny_unknown_fields`; every enum is a closed list; every id is unique across the whole
file, not only within its list, so a constraint target such as `near: market` names exactly
one thing; constraint targets refer to ids in this file only, and only to ids of things with
a position (REQUIREMENTS.md F3, §4 below). Every
enum is internally tagged (REQUIREMENTS.md F1): a variant with fields is written as a map
whose first key is `kind`, as `constraints[]` shows with `rule`; `ConstraintParams` uses
`rule` as its tag. Jeffrey accepted this section on 2026-10-20 with the answers in §6.

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
| `defences.walls[]` | `{id, kind, encloses[]}` | no | kind: palisade, stone, razed; `encloses` lists district ids; the wall count F2 asks for is the length of this list, there is no separate count field |
| `defences.citadel` | `{id, side, inside}` or null | no | `inside` is a wall id |
| `defences.gates[]` | `{id, wall, side}` | no | a gate side must have an entry road, else a validation error |
| `open_space[]` | `{id, kind, constraints[]}` | no | kind: market_square, plaza, green, parade_ground |
| `crossings[]` | `{id, river, kind, constraints[]}` | no | kind: bridge, ford |
| `districts[]` | `{id, name, type, weight, constraints[], notes}` | yes, 1 to 100 | type is a closed enum (§3); `name` is the legend label; `notes` is optional, default "", ignored by the generator |
| `landmarks[]` | `{id, name, kind, footprint, constraints[], resolved, pinned, notes}` | no, 0 to 1000 | |
| `landmarks[].footprint` | `{class, width_m, depth_m}`, each part optional; whole field optional | no | resolved in order: the kind's default class (§3), then `class`, then `width_m` and `depth_m` each override one side; core only ever sees a width and a depth |
| `landmarks[].resolved` | `{x, y, rotation_deg}` or null | no | metres in the site frame, origin south-west corner |
| `landmarks[].pinned` | bool | no, default false | true requires `resolved`, else validation error |
| `roads[]` | `{id, side, importance}` | yes, at least 1 | importance: major, minor; major roads become main streets from gate to market at milestone 5 |
| `presets.era` | enum | yes | early_medieval, high_medieval, late_medieval |
| `presets.style` | string | no, default "ink" | resolved per ARCHITECTURE.md §6 |
| `presets.packs[]` | string list | no, default [] | |

## 3. Closed enums

- Compass side: north, east, south, west. Compass region (for `compass_region`): north,
  north_east, east, south_east, south, south_west, west, north_west, centre.
- District type: residential, craftsmen, merchant, harbour, farms, noble, temple_quarter,
  military, shanty, market. Adding one is a schema change.
- Landmark kind and its default footprint class: temple large, hall medium, tower small,
  castle huge, inn medium, shrine small, academy large, theatre large, arsenal large,
  bridgehead small, well small, monument small. The spec's `footprint` overrides (§2).
- Footprint class and its default width × depth: small 10 × 10 m, medium 20 × 20 m,
  large 40 × 40 m, huge 60 × 60 m. A rule pack may override these numbers later.

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

A positioned spec id is the id of a landmark, district, open space, crossing, river, lake,
harbour, wall, gate or citadel. A road id is not a target. A list of constraints applies all
of them together: "near the river, near the north gate, inside the outer wall" is three
entries.

Every constraint has `hard` (default false) and `weight` (default 1, ignored when hard).
`ConstraintParams` is the closed enum with one variant per row (ADR-017).

## 5. Validation errors the example must trigger in tests

One unit test per message, each naming the field path:
- missing `schema`; `schema: 2`.
- unknown field anywhere (`deny_unknown_fields`).
- duplicate id within a list; the same id in two different lists (for example a district and
  an open space); a constraint target that does not exist.
- `pinned: true` without `resolved`.
- a pinned landmark whose resolved position or footprint lies outside the site
  (0 to `extent_m` on both axes); checked in `citymap-cli`, not in core.
- a gate on a side with no entry road.
- a harbour on a side with no coast or river.
- `extent_m` outside 500 to 5000; `population` zero; NaN anywhere.
- a footprint `width_m` or `depth_m` that is zero or negative.
- a negative `within_m`, `beyond_m` or `min_spacing` `metres`.
- a constraint target that names a road (a road has no position to be near).
- more than 100 districts or 1000 landmarks.

## 6. Jeffrey's answers, 2026-10-20

1. `settlement.level` and `settlement.size_class` are dropped; `population` is the only size
   input. Status words such as "town" carry culture-specific presumptions (a parish church is
   European), so features are written out explicitly instead.
2. `roads[].destination` is dropped; `side` and `importance` are enough for version 1.
3. Footprint classes stay as defaults, each with a width and depth; a landmark may override
   the class, the width or the depth.
4. The `fixed` rule is dropped; `pinned` is the only way to place a landmark exactly.
5. F3 is widened to every positioned spec id (§4), applied to REQUIREMENTS.md in this PR.

Recorded in ADR-021.
