# Proposal: spec schema version 1 and the example city

Status: draft for Jeffrey's review, 2026-10-19. Nothing here is decided. The example city is
fictional and exists only to exercise every field; it is not campaign lore (ADR-016).

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
  level: 7
  population: 6000
  size_class: town          # hamlet | village | town | city | metropolis

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
    harbour: { side: east, length_m: 300 }
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
    footprint: large            # small | medium | large | huge
    constraints:
      - { rule: compass_region, region: north_east, hard: true }
      - { rule: on_high_ground, hard: false, weight: 2 }
    resolved: null
    pinned: false
    notes: ""
  - id: guildhall
    name: Weavers' Hall
    kind: hall
    footprint: medium
    constraints: [{ rule: in_district, target: craft, hard: true }]
    resolved: { x: 1310.25, y: 880.50, rotation_deg: 0 }
    pinned: true
    notes: pinned to test that pins survive a seed change
  - id: lighthouse
    name: The Harrow Light
    kind: tower
    footprint: small
    constraints:
      - { rule: in_district, target: docks, hard: true }
      - { rule: far_from, target: temple, hard: false, weight: 1 }
      - { rule: min_spacing, metres: 40, hard: true }
    resolved: null
    pinned: false
    notes: ""

roads:
  - { id: north_road, side: north, importance: major, destination: Kingsreach }
  - { id: west_road, side: west, importance: minor, destination: the mill villages }

presets:
  era: early_medieval         # early_medieval | high_medieval | late_medieval
  style: ink
  packs: []
```

## 2. The schema this implies

Types below are what `citymap-cli` parses into and `citymap-core` consumes. Every struct is
`deny_unknown_fields`; every enum is a closed list; every id is unique across the whole
file, not only within its list, so a constraint target such as `near: market` names exactly
one thing; constraint targets refer to ids in this file only (REQUIREMENTS.md F3). Every
enum is internally tagged (REQUIREMENTS.md F1): a variant with fields is written as a map
whose first key is `kind`, as `constraints[]` shows with `rule`; `ConstraintParams` uses
`rule` as its tag. Every rule in this section is proposed by Claude; Jeffrey decides them all
by merging this proposal or by answering §6.

| Field | Type | Required | Notes |
|---|---|---|---|
| `schema` | integer, must be 1 | yes | first key; anything else is exit 2 |
| `identity.name` | string, 1 to 80 chars | yes | |
| `identity.seed` | u64 | yes | `--seed` overrides for one run |
| `identity.notes` | string | no, default "" | ignored by the generator |
| `settlement.level` | integer 0 to 25 | yes | Pathfinder settlement level |
| `settlement.population` | integer 1 to 1,000,000 | yes | drives built area with the era preset |
| `settlement.size_class` | enum | yes | hamlet, village, town, city, metropolis |
| `site.extent_m` | number 500 to 5000 | yes | REQUIREMENTS.md non-goal: 500 m to 5 km |
| `site.terrain.coast` | `{side, kind}` or null | no | kind: bay, open, strait, estuary |
| `site.terrain.cliffs[]` | `{side, height_m}` | no | |
| `site.terrain.hills[]` | `{at, radius_m}` | no | `at` is a compass point or `centre` |
| `site.terrain.slope` | `{toward, metres_per_km}` or null | no | |
| `site.terrain.plateau` | `{side, height_m}` or null | no | Bearna needs this at milestone 9 |
| `site.water.rivers[]` | `{id, from, to, width_m}` | no | from and to are compass regions except `centre`; a river exits into coast if present |
| `site.water.lakes[]` | `{id, at, radius_m}` | no | |
| `site.water.harbour` | `{side, length_m}` or null | no | requires coast or a river on that side |
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
| `landmarks[].resolved` | `{x, y, rotation_deg}` or null | no | metres in the site frame, origin south-west corner |
| `landmarks[].pinned` | bool | no, default false | true requires `resolved`, else validation error |
| `roads[]` | `{id, side, importance, destination}` | yes, at least 1 | importance: major, minor; destination is free text for the legend |
| `presets.era` | enum | yes | early_medieval, high_medieval, late_medieval |
| `presets.style` | string | no, default "ink" | resolved per ARCHITECTURE.md §6 |
| `presets.packs[]` | string list | no, default [] | |

## 3. Closed enums

- Compass side: north, east, south, west. Compass region (for `compass_region`): north,
  north_east, east, south_east, south, south_west, west, north_west, centre.
- District type: residential, craftsmen, merchant, harbour, farms, noble, temple_quarter,
  military, shanty, market. Adding one is a schema change.
- Landmark kind: temple, hall, tower, castle, inn, shrine, academy, theatre, arsenal,
  bridgehead, well, monument. Each has a default footprint; the spec's `footprint` overrides.
- Footprint class: small (under 150 m²), medium (150 to 600), large (600 to 2500), huge
  (over 2500).

## 4. Constraints, version 1 (REQUIREMENTS.md F3)

| rule | parameters | target type | meaning |
|---|---|---|---|
| `compass_region` | `region` | none | candidate lies in that region of the site |
| `near` | `target`, optional `within_m` (default 150) | any spec id | candidate centre within the distance of the target |
| `far_from` | `target`, optional `beyond_m` (default 300) | any spec id | the opposite |
| `on_water` | none | none | candidate touches coast, river, lake or harbour |
| `on_high_ground` | none | none | candidate elevation in the top quartile of the site |
| `inside_walls` | `target` | wall id | candidate inside that wall ring |
| `outside_walls` | `target` | wall id | candidate outside that wall ring |
| `in_district` | `target` | district id | candidate inside that district |
| `min_spacing` | `metres` | none | no other landmark centre within the distance |
| `fixed` | `x`, `y` | none | identical to a pinned resolved placement; kept for packs that place things |

Every constraint has `hard` (default false) and `weight` (default 1, ignored when hard).
`ConstraintParams` is the closed enum with one variant per row (ADR-017).

## 5. Validation errors the example must trigger in tests

One unit test per message, each naming the field path:
- missing `schema`; `schema: 2`.
- unknown field anywhere (`deny_unknown_fields`).
- duplicate id within a list; a constraint target that does not exist.
- `pinned: true` without `resolved`.
- a gate on a side with no entry road.
- a harbour on a side with no coast or river.
- `extent_m` outside 500 to 5000; `population` zero; NaN anywhere.
- more than 100 districts or 1000 landmarks.

## 6. Questions for Jeffrey

1. Is `size_class` redundant with `population` plus `level`? Options: keep both and validate
   consistency; derive it; drop it.
2. Should `roads[].destination` be required? It feeds the legend ("road to Kingsreach") and
   nothing else.
3. Footprint thresholds in §3 are my guesses from medieval building sizes. Keep, or defer to
   the era preset?
4. Is `fixed` worth a rule, or is `pinned` enough and `fixed` is dropped from F3?
5. F3 lists landmark, district, open space and crossing as the only constraint targets. This
   proposal also targets a river (`near: ash`) and a wall (`inside_walls: outer`), which F3
   does not allow. Options: (a) widen F3 to every spec id that has a position (add river,
   lake, harbour, wall) and keep the proposal; (b) keep F3 and drop river and wall targets
   from the proposal, which makes `inside_walls` take no target and leaves "near the river"
   impossible to say. Claude recommends (a), applied to F3 in the same PR that adopts this
   proposal.
