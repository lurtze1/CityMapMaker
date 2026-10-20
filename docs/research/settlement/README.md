# Settlement knowledge base

> Written 2026-10-20 by eight research agents, one per topic, and assembled by Claude (issue
> #33). A claim here is binding only when a DECISIONS.md entry cites it. Many case facts come
> from encyclopaedia pages; scholarly books were mostly not readable. Claims seen only in search
> summaries are marked "(unverified)"; an agent's own arithmetic is marked as such.

Purpose: Jeffrey asked for a broad research pass on anything useful for the generator:
settlement growth, construction, and social factors. This folder holds one report per topic.
This page lists them, then pulls out what several reports agree on. Companion report:
`docs/research/city-foundation-and-growth.md` (issue #32), on why towns are founded where they
are.

## Reports

| File | Topic | Feeds |
|---|---|---|
| `streets.md` | Street widths, junction mix, block sizes, network metrics, measured | S3, sweep harness |
| `parcels-density.md` | People per hectare, household size, plot sizes, population to area | S4, S5 |
| `social-structure.md` | Who lives and works where: wealth, noxious trades, quarters, edge institutions, markets | rule packs |
| `walls-growth.md` | Wall circuits, gates, growth rings, fringe belts, old walls as ring roads | S1, F2b history layers |
| `water-infrastructure.md` | Wells, fountains, mills, bridges, quays, drainage, fire rules | F2b infrastructure |
| `traditions.md` | Town-building rules of eight traditions (Islamic, Chinese, Japanese, Indian, Southeast Asian, Mesoamerican and Andean, African, European and Spanish colonial) | rule packs |
| `growth-generators.md` | Academic and game generators that grow towns or lay roads first | stage order |
| `table-use.md` | Map legibility, what GMs use city maps for, believability heuristics, fantasy factors | rendering, labels |

Each report ends with "What this means for a generator" and a "Not found" list.

## What several reports agree on

Terms: a *constraint kind* is one entry in the closed list of placement rules (REQUIREMENTS.md
F3, ADR-017); a *rule pack* is the data file that switches rules on and sets their numbers.

1. **Roads and gates come before most placements.** Streets (freeze the gate-to-market backbone
   first), social structure (leper houses, gallows, cattle markets and suburbs attach to gates
   and outbound roads), water (mills at bridges and gates) and growth generators (Beneš 2014
   grows the town from road junctions) all need roads early. This agrees with the roads-first
   decision for spec v1 and with the companion report.
2. **Culture differences are mostly two shapes.** Directional zoning around a centre (the
   Arthashastra assigns classes to compass sides; the Kaogong ji puts temple left, market
   behind; Pingyao puts civil offices east) and status falling with distance from a centre
   building (Japanese castle towns, Madurai, Yoruba towns, Islamic souks). Two constraint kinds
   could cover most of `traditions.md` and `social-structure.md`.
3. **Candidate constraint kinds that F3 does not have yet.** Collected across reports, not
   decided: `sector_of` (lies in compass sector Y of anchor Z), `rank_by_distance` (status falls
   with distance from an anchor), `downstream_of` (below a water feature on the same river),
   `along` (follows a road or water line), `at_junction`, `near_gate` and `on_road_out`, and an
   enclave kind (a bounded block with few gates and an anchor). Adding any of them is an
   ADR-017 change.
4. **Rules carry a strength.** `traditions.md` and `social-structure.md` label each rule law,
   treatise ideal, strong custom or tendency. That maps onto hard and soft constraints: law
   hard, the rest soft with a weight. Storing the label in the pack lets a GM loosen an ideal
   plan into a lived-in town.
5. **Population to area has a measured formula.** Cesaretti et al. (2016), 173 European towns
   around 1300: area = 0.119 × population^0.714 (unit assumed hectares; the fetched text does
   not state it). That gives about 27 ha for 2,000 people and 86 ha for 10,000, so bigger towns
   are denser. Density bands per culture (medieval Europe 100 to 120 per ha, Roman 150 to 250,
   Classic Maya cores 10 to 20) belong in rule packs.
6. **Real numbers exist for calibration.** Plot frontages snap to a unit (perch, ken); burgage
   plots are about 1:6. Organic towns have more T-junctions than crossroads and 15 to 30% dead
   ends; side streets meet at about 90°, through-routes run nearly straight (Hillier). Gates
   number 2 to 15, about one per main road. Towers stand 30 to 60 m apart on a medieval wall.
7. **Growth comes in layers.** Fill the plots, then ribbon suburbs along roads out of the gates,
   then a fringe belt of large-plot institutions just outside the wall, then optionally a new
   wall. A retired wall becomes a ring road. Each phase can run on new land only, with earlier
   phases kept as pins, which is the pin rule applied to time. This fits the F2b history
   layers.
8. **Every map can carry its own measurements.** `streets.md` lists cheap network metrics
   (junction mix, average degree, median segment length, orientation order, circuity,
   meshedness) with values for real cities. Printing them next to the real values is the
   corpus comparison the sweep harness needs at milestone 5.
9. **Some data does not exist, so the spec or a GM? question supplies it.** Not found anywhere:
   wells per population, markets per population, share of town area by use, building coverage
   per plot, the width of the lane inside a wall or of a fringe belt. These stay explicit inputs
   or rule-pack values marked "uncalibrated", never silent defaults.
10. **Fantasy inputs are never invented.** Ancestry mix, threats such as flying attackers, and
    magic level change settlement form in published settings, but they are GM inputs. A missing
    one is a GM? question.

## Tensions to decide later

- **Buildings at overview scale.** `table-use.md` finds practitioners prefer streets, blocks,
  district fills and landmark icons over drawing every building. Milestone 6 plans parcels and
  buildings. Whether buildings are drawn, optional or texture only is Jeffrey's decision.
- **GM map and player map.** Two render profiles from one generation (secrets hidden on the
  player version) is a new output requirement; Jeffrey decides whether it enters REQUIREMENTS.md.
- **The rich in the centre?** Sjoberg's model (elite centre, poor edge) holds only for the poor
  in the studies found; in Amsterdam the richest lived one ring out from the market and held the
  main-street fronts. A rule pack has to choose a model; neither is universal.
- **Intersection density depends on what counts as a street.** One source gives about 103
  junctions per km² for historic cores, a Venice sample about 710. The generator must state
  which network it measures (cart streets only, or every passage).
