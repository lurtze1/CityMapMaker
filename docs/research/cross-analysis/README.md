# Cross-analysis of the research: overview

Six topic reports turn the research reports (#34, #35, #38, #28) into facts under
`data/research/` (ADR-025) and into questions for Jeffrey. This page gives the totals, the
conflicts that changed a claim, and every question sorted by the milestone that needs the
answer. Each question lives in its topic report with options, fact ids and a recommendation;
Jeffrey decides each one.

## Totals (2026-10-20)

| Topic | Report | Facts | Verified | Corrected | Disputed | Superseded | Questions |
|---|---|---|---|---|---|---|---|
| Density and population | [density.md](density.md) | 376 | 34 | 5 | 18 | 5 | 9 |
| Streets | [streets.md](streets.md) | 298 | 80 | 3 | 14 | 0 | 11 |
| Walls and gates | [walls.md](walls.md) | 512 | 33 | 6 | 24 | 7 | 9 |
| Plots and blocks | [plots.md](plots.md) | 245 | 3 | 0 | 20 | 0 | 10 |
| Footprints and open spaces | [footprints.md](footprints.md) | 245 | 41 | 4 | 18 | 1 | 8 |
| Town core, founding and growth | [core.md](core.md) | 153 | 20 | 1 | 6 | 4 | 7 |
| All, validated together | | 1,829 | 211 | 19 | 100 | 17 | 54 |

510 sources and 69 units, 0 validator errors with all six topics merged. 1,482 facts are
`unverified`: taken from a report, not yet checked against the source.

## Claims the cross-analysis changed

- Medieval European density: Russell's 100–120 people per hectare described ancient cities;
  the verified median is 114 (middle half 93–152), and 11 percent of towns in 1300 exceed 200
  (density.md, conflict 1).
- A single "Roman density band" of 150–250 per hectare has no source; density rises with town
  size (91 per ha at 10 ha, 427 at 1,000 ha) (density.md).
- The Arthashastra royal road is 8 danda in the Sanskrit and 4 in the 1915 translation
  (streets.md, conflict 1); the Islamic seven-cubit rule settles disputes and is not a
  minimum width for every lane (streets.md, conflict 2).
- Cologne's Roman walled area is 97 hectares, not 25 (walls.md, conflict 1).
- The time-series report (#28) claimed that markets, wall lines and street lines never move;
  its own city blocks show exceptions (Ayutthaya and Timgad markets; Vienna, Amsterdam and
  Timgad walls; street changes after fires). #28 is corrected; core.md and walls.md cite the
  city blocks only.
- The spec's footprint table (spec-v1 §3) is right for a parish church and a small castle and
  wrong elsewhere: market halls are small, mosques and temples are close to square, a keep is
  medium, a caravanserai is large (footprints.md §5).
- For the town core, the research ranks water above roads, and no source supports the 150 m
  `centre` radius (core.md §7).

## Questions by when they are needed

### A. Spec version 1 (PR #20), before it merges

| # | Question | Topic report | Recommendation in the report |
|---|---|---|---|
| A1 | Does a road–river crossing outrank a road–road crossing for the core? | core Q1 | only between roads of equal importance |
| A2 | Is a road meeting the harbour a core candidate? | core Q2 | yes, ranked like a river crossing |
| A3 | Can the core sit at a pinned landmark (castle-first town)? | core Q3 | defer to #31 |
| A4 | What happens to the 150 m centre radius? | core Q4 | keep, with a dated revisit |
| A5 | Default "market near core" rule, which needs `core` as a constraint target? | core Q5 | yes |
| A6 | One-road town: core as a point or a stretch of road? | core Q6 | a point |
| A7 | Religious labels (`chapel`, `mosque`, `friday_mosque`, `temple`; `shrine` tiny) | footprints Q1 | add the labels |
| A8 | `cathedral` shape 2.5 : 1 or 2 : 1 | footprints Q2 | 2 : 1 |
| A9 | Defence labels (`castle` huge, `keep` medium, `tower` and `gatehouse` small) | footprints Q3 | add the labels |
| A10 | Commerce default class medium or small | footprints Q4 | small |
| A11 | Tiny class floor 9 m² or lower | footprints Q5 | keep 9 m² |
| A12 | `caravanserai` label, large 1.3 : 1 | footprints Q6 | add |
| A13 | `ropewalk` label, huge 15 : 1 | footprints Q7 | add |
| A14 | Default sizes per open-space kind | footprints Q8 | add, market square 50–100 m |
| A15 | Populations outside 1,000–200,000 | density Q4 | warn |

### B. Milestone with S1 (districts, walls) and the population-to-area step

| # | Question | Topic report |
|---|---|---|
| B1 | How population becomes an area (constant, the Cesaretti law, per-era law) | density Q1 |
| B2 | Which area the formula gives (whole town, walled, built) | density Q2 |
| B3 | Does the seed vary density, and by how much | density Q3 |
| B4 | Density inside the town (uniform, gradient, per-district multiplier) | density Q5 |
| B5 | May a rule pack replace the European law | density Q6 |
| B6 | Truro at 10,000 people | density Q7 |
| B7 | Role of the game source (150 per ha) | density Q8 |
| B8 | Empty land inside a wall | walls Q1 |
| B9 | Drawn thickness of a stone wall | walls Q3 |
| B10 | Default ditch | walls Q4 |
| B11 | How a razed wall is drawn | walls Q7 |
| B12 | Where the citadel sits on its wall | walls Q8 |
| B13 | Water-side wall thickness | walls Q9 |

### C. Milestone with S3 (streets)

| # | Question | Topic report |
|---|---|---|
| C1–C3 | Primary, secondary and tertiary widths | streets Q1–Q3 |
| C4 | Widths per era | streets Q4 |
| C5 | Width within a class | streets Q5 |
| C6 | Width of link streets | streets Q6 |
| C7 | Junction angles | streets Q7 |
| C8 | Junction mix and dead-end targets | streets Q8 |
| C9 | Gradient limits | streets Q9 |
| C10 | Widths in historical units in culture packs | streets Q10 |
| C11 | Arthashastra width for an Indic pack | streets Q11 |
| C12 | Tower spacing on stone walls | walls Q2 |
| C13 | May generated streets cross a wall | walls Q5 |
| C14 | Lane inside the wall | walls Q6 |

### D. Milestone with S4 and S5 (blocks, parcels, buildings)

| # | Question | Topic report |
|---|---|---|
| D1–D10 | Where defaults live, plot model, frontage, depth, block size, splitting, coverage, house footprint, storeys, which statuses a default may rest on | plots Q1–Q10 |
| D11 | People per dwelling | density Q9 |

### E. Later

| # | Question | Topic report |
|---|---|---|
| E1 | What a growth phase keeps fixed (#51) | core Q7 |

## How to query

```bash
analysis/.venv/Scripts/python analysis/build_facts_db.py
analysis/.venv/Scripts/python -c "import sqlite3; db = sqlite3.connect('out/facts.sqlite'); print(db.execute(\"SELECT place, min, max, unit, status FROM facts WHERE quantity = 'street_width' AND year_to < 1300\").fetchall())"
```
