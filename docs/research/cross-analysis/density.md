# Cross-analysis: density

Written 2026-10-20 (ADR-025). This document turns research into decisions, so it follows the
CLAUDE.md explicit-language rule in full (ADR-023). Words such as "about" or "c." inside quotes
or fact notes are the sources' own uncertainty. Arithmetic done in this document, not in a
report, is marked "(arithmetic here)".

Terms used below:

- **Density**: inhabitants per hectare (1 ha = 100 m x 100 m). A figure per km² is divided by
  100 to give per ha.
- **Settled area**: the town including suburbs outside the walls and empty land inside them.
  This is how Cesaretti et al. 2016 measured their 173 towns.
- **Walled area**: the area inside one wall circuit, whether built on or not.
- **Built area**: built-up land only, sometimes including streets; the fact note says which.
- **Site area**: an archaeological site, a heritage listing or a city area that is none of the
  three above; the fact note says what it covers.
- **Gross density**: population divided by a large enclosing area that contains fields,
  gardens or ruins (Old Oyo).
- **Scaling law**: `area = a × population^b`. When `b` is below 1, bigger towns are denser.

## 1. Scope

Reports read (none edited):

- `docs/research/settlement/parcels-density.md`, `verification-population.md`, `README.md`,
  `social-structure.md` (no density figures; it states that markets per population were not
  found)
- `docs/research/settlement/timber-ports.md`, `table-use.md`, `walls-growth.md`,
  `verification-streets-walls.md` (skimmed for population and area figures)
- `docs/research/city-foundation-and-growth.md`
- `docs/research/sources-by-language/README.md` and all seven reports (`africa.md`,
  `chinese.md`, `continental.md`, `iberian.md`, `islamic.md`, `japanese.md`, `south-asia.md`)
- `docs/research/town-plan-corpus.md`, `docs/research/town-plan-corpus-world.md`
- The spec proposal (`origin/m1/spec-proposal:docs/proposals/spec-v1.md`) and REQUIREMENTS.md
  F2, T1 and §8 on that branch, for section 5.

Output:

| File | Content |
|---|---|
| `data/research/facts/density.yaml` | 378 facts |
| `data/research/sources/density.yaml` | 92 sources |
| `data/research/quantities/density.yaml` | 10 new quantities: `settled_area`, `site_area`, `district_area`, `household_size`, `household_count`, `people_per_institution`, `scaling_exponent`, `scaling_prefactor`, `share_of_towns`, `land_use_share` |

Status of the 378 facts: 34 verified, 5 corrected, 5 superseded, 18 disputed, 316 unverified.
By quantity: 137 population, 87 population_density, 61 walled_area, 41 site_area, 11
household_count, 10 household_size, 7 land_use_share, 6 scaling_exponent, 6 built_area, 4
settled_area, 3 people_per_institution, 2 scaling_prefactor, 2 district_area, 1 share_of_towns.

Every fact id below starts with `density.`; the tables drop that prefix to save space.

Source ids follow the agreed convention. Web pages without a year end in `-nd` (no date). Where a
report gives a figure with no traceable source, the fact cites a source entry for the report's
own claim (`citymapmaker-<report>-2026`, kind `web`), and its note says so. The two town-plan
corpora state that their populations come from general literature and are order-of-magnitude;
of their 122 facts, 118 are `unverified` and 4 are `disputed` (conflicts 6, 8, 9 and 10).

No new units were needed. Per-acre figures are stored in the per-ha value that the report itself
converted (factor 2.471, the report's), with the per-acre figure in the note.

### Repair round 2026-10-21

A sample check found wrong years in this topic; every fact was then re-read against its
report and filled with the fields of fact format v2 (ADR-025). What changed:

- `approximate: true` on 189 facts: the report gives the figure with "about", "c.", "~",
  "roughly", "nearly" or "perhaps", or the report states that all its populations are
  approximate (both town-plan corpora). A "c." on a year only, as for Salvador, sets
  `year_approximate` and not `approximate`.
- Years removed from 93 facts, because the report gives no date for that figure. Most are
  world-corpus populations that had taken the plan's date, Hanson's Roman figures, Smith's
  Mesoamerican figures, the "late Edo" figures and the Viking-age ports.
- Years changed on 72 facts. Examples: the Cesaretti facts from an invented 1250-1350 to
  "c. 1300"; centuries to their full range (16th century is 1501-1600); `bibracte.inner-area`
  from -150 to the report's 3rd to 1st century BC (-300 to -1); `jaipur-1911.area` to the
  1914 plan; `lima-1688.area` to the 1688 plan; `lahore-1847.area`, `mandalay-1911.area`,
  `batavia-1673.area`, `havana-1740.area` and `new-orleans-1760.area` to their plan dates.
  The world-corpus pairs in section 2 therefore combine a population and an area of
  different years where the fact ids show it.
- `year_basis` on all 244 facts that have years, with these meanings: `census` a count of
  people, houses or hearths; `estimated` a dated population or density estimate; `plan_date`
  the date of the plan an area is read from; `measured` a survey or sanitary measurement;
  `built` the date of the wall or of the design population; `founded` the foundation;
  `described` the period or state of the place that the figure describes; `depicted` (11
  facts) the period a plan or reconstruction shows, for areas read from it (Pompeii as of
  79 CE, Ur, Babylon, Nineveh, Hattusa, Dholavira, Baghdad, Heijo-kyo, Teotihuacan,
  Cahokia, Tang Chang'an 7th-9th c.); and `valid_until` (4 facts) a figure valid until the place was abandoned,
  with only `year_to` (Old Oyo, "to c. 1837").
- `year_approximate: true` on 106 facts ("c. 1300", "about 1560", "to c. 1837", "late 1970s",
  "early 19th century", "mid-16th century").
- `disputes` on 20 facts in 13 pairs (conflicts 6, 8, 9, 10, 11, 12, 19 and 20).
- `source_via` on 27 facts. On 7 facts `source` now names the original: Hardoy, Lozano,
  Clerget, Lézine, Karpat, Mantran, and Chase and Chase 2016 for `maya-classic.core-canuto`
  (the corrected 5 per ha is theirs). Twelve source entries were added; four existing entries
  (`hanson-2016`, `smith-2005`, `bao-nd`, `agbaje-williams-1983`) now point to the work
  through which they were read.
- Statuses changed on 10 facts: to `disputed` `pompeii.population-hanson`,
  `amsterdam-1544.area`, `edo-late.townspeople-land-share`, `edo-chonin-land-share-unverified`;
  to `unverified` `timgad.design-population`, `aleppo.density-rejected`,
  `algiers.density-rejected`, `algiers-1789.population`, `harar-1876.population`,
  `tsumago-1843.houses`. A fact is `disputed` only when two sources give conflicting values
  for the same thing.
- Two facts added, so that a dispute names both values: `chan-chan.population-other-sources`
  and `timbuktu-15c-16c.population-larousse`. No fact was removed. No fact in this topic comes
  from `town-plan-time-series.md`.

## 2. The picture

### 2.1 Population-to-area laws

| Law | Fact ids | Values | Status |
|---|---|---|---|
| Medieval Europe c. 1300, settled area from population | `europe-1300.scaling-exponent`, `europe-1300.scaling-prefactor` | area_ha = 0.1194 × pop^0.714 (95% CI of b: 0.662 to 0.766); valid 1,000 to 200,000 people | verified |
| Regional exponents, same sample | `france-belgium-1300.scaling-exponent`, `england-1300.…`, `germany-1300.…`, `northern-italy-1300.…` | 0.790 / 0.730 / 0.754 / 0.720 | verified |
| Roman towns, population from area | `roman.hanson-scaling-exponent`, `roman.hanson-scaling-prefactor` | pop = 41.834 × area_ha^1.3361 (52 excavated sites) | verified |
| Roman stepped classes (Hanson 2016) | `roman.hanson-small-sites`, `roman.hanson-large-sites` | 100 per ha under 50 ha, 500 per ha over 400 ha; steps between not found | verified |

Worked values, both laws (computed by `verification-population.md`). The European values use
the reports' prefactor 0.119 and are rounded to the nearest whole number; with the prefactor
0.1194 given above, 10,000 people give 85.7 ha and 116.7 per ha, and 50,000 people give
270.4 ha and 184.9 per ha (arithmetic here). The facts keep the report's values:

| Medieval Europe, population | Settled area | Density | Fact ids |
|---|---|---|---|
| 2,000 | 27 ha | 74 per ha | `europe-1300.settled-area-at-2000`, `europe-1300.density-at-2000` |
| 10,000 | 85 ha | 117 per ha | `europe-1300.settled-area-at-10000`, `europe-1300.density-at-10000` |
| 50,000 | 270 ha | 186 per ha | `europe-1300.settled-area-at-50000`, `europe-1300.density-at-50000` |

| Roman town area | Density | Fact id |
|---|---|---|
| 10 ha | 91 per ha | `roman.density-at-10ha` |
| 50 ha | 156 per ha | `roman.density-at-50ha` |
| 100 ha | 197 per ha | `roman.density-at-100ha` |
| 200 ha | 248 per ha | `roman.density-at-200ha` |
| 1,000 ha | 427 per ha | `roman.density-at-1000ha` |

The Cesaretti sample itself (fact ids `europe-1300.density-median`, `-mean`,
`-interquartile`, `-range`, `-share-above-200`): median 114, mean 130, middle half 93 to 152,
range 47 to 340 per settled ha; 19 of 173 towns (11%) are above 200.

The European law at other populations (arithmetic here, area_ha = 0.1194 × pop^0.714):
1,000 people 16.6 ha (60 per ha); 1,500 people 22.1 ha (68); 6,000 people 59.5 ha (101);
200,000 people 728 ha (275); 1,000,000 people 2,296 ha (436, outside the fitted range).

### 2.2 Europe, by era

| Era | Place or scope | Density per ha | Area basis | Fact ids | Status |
|---|---|---|---|---|---|
| early medieval (c. 750-1066) | Hedeby | 60 | walled 24-26 ha, 1,500 people | `hedeby.density`, `hedeby.area`, `hedeby.population` | unverified, derived by the report |
| early medieval | Birka | 100-140 | black-earth site 7 ha, 700-1,000 people | `birka.density` | unverified, derived |
| early medieval | Kaupang | 80-200 (arithmetic here) | site 5 ha, 400-1,000 people | `kaupang.area`, `kaupang.population` | unverified |
| high medieval c. 1170 | Dublin | 375 as an upper bound | 4,500 people in 12 ha walled; different sources and dates | `dublin-1170.density-bound` | unverified; timber-ports.md calls its source weak |
| high medieval c. 1300 | 173 towns, England, France, Belgium, Germany, northern Italy | median 114, middle half 93-152 | settled | `europe-1300.density-*` | verified |
| late medieval 1356-1383 | Paris, wall of Charles V | over 341 (arithmetic here) | walled 440 ha, "more than 150,000" | `paris-charles-v.area`, `paris-charles-v.population` | corrected label, uncited figure |
| late medieval 1400 | Dutch towns, mean | 75.5 | walled, 2,850 ha in total | `netherlands-1400.mean-density` | unverified |
| late medieval 1400-1560 | Dutch towns of 10-14 ha | 62-94 | walled | `netherlands-small-towns.density` | unverified |
| early modern 1514 | Amsterdam | 160 | walled 75 ha, 2,507 houses | `amsterdam-1514.*` | unverified |
| early modern 1560 | Dutch towns, mean | 126.5 | walled, 3,250 ha in total | `netherlands-1560.mean-density` | unverified |
| early modern 1560 | 's-Hertogenbosch / Groningen / Haarlem / Delft, Leiden, Gouda | 190 / 180 / 170 / 150 | walled | `s-hertogenbosch-1560.density` and siblings | unverified |
| early modern mid-16th c. | Dordrecht, Nijmegen | over 200 | walled, 10,000+ on barely 50 ha | `dordrecht-nijmegen-16c.density` | unverified |
| early modern 1560 | Amsterdam | over 300 | walled | `amsterdam-1560.density` | unverified |
| early modern 1560 | Asperen | just over 40 | walled 22 ha, 900 people | `asperen-1560.*` | unverified |
| modern c. 1806 | Lyon / Bordeaux / Marseille / Toulouse / Lille | 750 / 410 / 420 / 200 / 260 | inside the last walls | `lyon-1806.density` and siblings | unverified; continental.md calls the forum source weak (amateur, partly unsourced) |

Town-plan corpus pairs, Europe (population and walled area both from `town-plan-corpus.md`,
order-of-magnitude; densities are arithmetic here): Caernarfon c. 1600, 1,000-1,500 on 4 ha
(250-375); Edinburgh c. 1580, 12,000-15,000 on 55 ha (218-273); Dublin 1610, 10,000-15,000 on
20 ha (500-750). Fact ids `caernarfon-1600.*`, `edinburgh-1580.*`, `dublin-1610.*`.

World-corpus pairs, Europe (arithmetic here): Bibracte 5,000-10,000 on 135-200 ha (25-74);
Constantinople 1422, 40,000-50,000 on 1,400 ha (29-36, a city shrunk inside walls built for
400,000); Venice 1500, 100,000-115,000 on 500 ha (200-230); Amsterdam 1544, 12,000-14,000 on
100 ha (120-140); Palmanova 2,000-3,000 on 50 ha (40-60); Zamość 3,000 on 24 ha (125);
Carcassonne Cité under 2,000 on 7 ha (up to 286); Neuf-Brisach 2,000 civilians on 36 ha (56).

### 2.3 Roman and ancient

| Place | Density per ha | Basis | Fact ids | Status |
|---|---|---|---|---|
| Roman towns, law | 91 at 10 ha to 427 at 1,000 ha | population from area | `roman.density-at-*` | verified |
| Roman cities, observed | 50-500 | Hanson and Ortman, via Smith 2020 | `roman.hanson-ortman-range` | unverified |
| Silchester | 130 | 5,500 people, 5 per dwelling assumed | `silchester.*` | unverified |
| Pompeii | 121-182 (arithmetic here) | Hanson 8,000-12,000 on 66 ha walled | `pompeii.population-hanson`, `pompeii.walled-area` | disputed (conflict 6) |
| Ancient cities (Russell) | 100-120 | Pompeii paper | `ancient.russell-average` | corrected (scope) |

World-corpus pairs, ancient (arithmetic here, all unverified): Nineveh 133-160; Hattusa
222-278; Priene 108-162; Dholavira 319; Babylon 500 (east-bank inner city only). Ur
(`ur.*`) gives no density: its population and area differ in scope (conflict 14).

### 2.4 Islamic world

| Place | Density per ha | Basis | Fact ids | Status |
|---|---|---|---|---|
| Data-set assumption, compact city | 150 | modelling assumption | `islamic-medieval.compact-city` | unverified |
| Data-set assumption, garden city | 75 | modelling assumption | `islamic-medieval.garden-city` | unverified |
| Sanaa, 15th c. | 60 | 25,000 on 425 ha | `sanaa-15c.*` | unverified |
| Cairo c. 1798 | 360 overall; 398 per built-up ha incl. streets | 263,000 on 730 ha; 662 ha built | `cairo-1798.*` | unverified |
| Cairo, walled Qahira core c. 1798 | 588 | 90,000 on 153 ha | `cairo-1798.qahira-density` | unverified |
| Cairo zones c. 1798 | 302-375 | Husayniyya, west, south | `cairo-1798.husayniyya-density` and siblings | unverified |
| Cairo, one quarter (Ottoman) | 400 (report's division) | 800 people on 2 ha | `cairo-ottoman.quarter-*` | unverified |
| Cairo 1897, two quarters | 464-469 | Clerget | `cairo-1897.quarter-density` | unverified |
| Tunis 1881 | 380 | Lézine via Raymond | `tunis-1881.density` | unverified |
| Aleppo 1537 | 335 (arithmetic here) | 80,000 on 238.5 ha built-up (1516) | `aleppo-1537.population`, `aleppo-1516.built-up-area` | unverified |
| Aleppo / Algiers, rejected | 710 / 2,433 | Raymond calls them improbable | `aleppo.density-rejected`, `algiers.density-rejected` | unverified |
| Fez medina, late 1970s | 650 | modern overcrowding | `fez-1970s.density` | unverified |

World-corpus pairs (arithmetic here): Jerusalem 1865, 200; Khiva Ichan Kala 192; Aleppo 1811,
229-286; Fez 1912, 270; Tunis 1861, 370. Baghdad c. 800 (`baghdad-800.*`) gives no density:
the population is greater Baghdad and the area is the Round City (conflict 14).

### 2.5 Sub-Saharan Africa

| Place | Density per ha | Basis | Fact ids | Status |
|---|---|---|---|---|
| Djenné, modern | 221 | McIntosh 1999 | `djenne-modern.density` | unverified |
| Inland Niger delta, traditional towns | 389 | McIntosh 1999 | `niger-delta.traditional-town-density` | unverified |
| Djenné hinterland, rural | 237 | rural, not a town value | `djenne-hinterland.rural-density` | unverified |
| Agadez core, 2011 | 251 (report's derivation) | 19,500 on 77.6 ha | `agadez.core-*` | unverified |
| Mapungubwe c. 1220-1300 | 300-500 (report's derivation) | 3,000-5,000 on 10 ha | `mapungubwe.*` | unverified |
| Old Oyo, to c. 1837 | 20-47 gross (report's derivation) | 60,000-140,000 over 3,000 ha walled, a third ruins | `old-oyo.*` | unverified |
| Djenné 1828 | 165-206 (arithmetic here) | 8,000-10,000 (Caillié) on today's 48.5 ha old town | `djenne-1828.population`, `djenne.old-town-area` | unverified; different dates |

Town areas without population: Lamu 16 ha, Gedi 34 ha (14th c.), Harar 48 ha walled, Sanje ya
Kati 200 ha ruin field, Djenné-Djèno 12 → 25 → 33 ha, Mbanza Kongo 89.29 ha listed.

### 2.6 South Asia

| Place | Density per ha | Basis | Fact ids | Status |
|---|---|---|---|---|
| Shahjahanabad, design | 84 (report's arithmetic) | 60,000 on 7.12 km² | `shahjahanabad-1650.*` | unverified / disputed area |
| Ahmedabad 1875, four divisions | 116 / 169 / 284 / 349 | 46.96 to 141.38 per acre | `ahmedabad-1875.*-density` | unverified |
| Ahmedabad 1879, Mandvi pol | 494 (report's division) | 10,000 on 20 ha | `ahmedabad-1879.mandvi-pol-*` | unverified |
| Benares 1901 | 84 gross | municipality, 6,162 acres | `benares-1901.density` | unverified |

World-corpus pairs (arithmetic here): Lahore 1868, 396; Jaipur 1911, 211; Srirangam 365;
Lo Manthang 300 (900 people on 3 ha).

### 2.7 East and Southeast Asia

| Place | Density per ha | Basis | Fact ids | Status |
|---|---|---|---|---|
| Kaifeng, late Northern Song | 120-130 | 12,000-13,000 per km² | `kaifeng-northern-song.density` | unverified, repost |
| Hangzhou 1241-52 / 1265-74 | 210 / up to 350 | 21,000 / 35,000 per km² | `hangzhou-1241.density`, `hangzhou-1265.density` | unverified, repost |
| Edo, townspeople land | 673.2 | 8.9 km², 16% of the city | `edo-late.townspeople-*` | unverified |
| Edo, samurai land | 168.2 | 38.7 km², 68% | `edo-late.samurai-*` | unverified |
| Edo, temple and shrine land | 56.8 | 8.8 km², 16% | `edo-late.temple-*` | unverified |

Sizes without density: Qing walled seats mean 0.78 km² (`qing-walled-seats.mean-area`); 1930s
county-and-above cities mean built area 1.1 km² (`china-1930s.mean-built-area`); Tang Chang'an
83.1 km² walled (`changan-tang.walled-area`).

World-corpus pairs (arithmetic here): Suzhou 1229, 143-214; Beijing c. 1800, 167; Seoul 1840,
125; Suwon 1795, 77-154; Heijō-kyō 35-100 (unwalled planned area); Chiang Mai 139; Ayutthaya
214-286; Batavia 1673, 235; Manila 1766, 156; Hue 96-192 (citadel area only); Mandalay 346.
Bangkok (`bangkok-1861.*`), Hangzhou (`hangzhou-1270.*`) and Guangzhou (`guangzhou-1855.*`)
give no density: their populations cover more than the walled area (conflict 14).

### 2.8 The Americas

| Place | Density per ha | Basis | Fact ids | Status |
|---|---|---|---|---|
| Tenochtitlan-Tlatelolco 1519 | 157.4 | 212,500 on 1,350 ha | `tenochtitlan-1519.*-smith` | verified |
| 15 other Late Postclassic cities | 10.4-72.2 | Smith 2005 Table 5 | `postclassic-mesoamerica.density-other-cities` | verified |
| Mayapan | 50 | Postclassic Maya | `mayapan.density` | verified |
| Tzintzuntzan / Zacapu / Santa Rita Corozal | 45 / 18 / 14 | report's arithmetic | `tzintzuntzan.*`, `zacapu.*`, `santa-rita-corozal.*` | unverified |
| Classic Maya, southern lowland cities | 5 (Tikal 517, Caracol 500 per km²) | whole mapped city | `maya-classic.core-canuto`, `tikal.density`, `caracol.density` | corrected / verified |
| Classic Maya, compact northern cities | 12-28 | Palenque, Sayil, Chunchucmil, Mayapan | `maya-classic.northern-compact` | verified |
| Classic Maya zones | urban 5-10, periurban 2-5, rural under 2 | Canuto via Smith | `maya-classic.urban-zone` and siblings | unverified |
| Buenos Aires 1744 | 100-200 people per block next to the plaza | per block, not per ha | `buenos-aires-1744.people-per-block` | unverified |
| Mexico City 1794 | 46.23% of a 249.9 ha centre built | land use, not density | `mexico-city-1794.built-share` | unverified |

World-corpus pairs (arithmetic here): Teotihuacan 50; Cahokia 6-12 (whole site); Lima c.
1690, 70-80; Havana 1740, 214; New Orleans 1760, 67; New Amsterdam 1660, 50; Pueblo Bonito
667-1,000 on one 1.2 ha building. Chan Chan (`chan-chan.*`, over 100,000 on over 20 km²) gives
no density: both figures are lower bounds, so their quotient is bounded neither above nor
below. Mexico City 1628 (`mexico-city-1628.*`) gives no density: the population includes the
barrios and the area is the traza only (conflict 14).

### 2.9 People per household

| Place | People per unit | Unit | Fact id | Status |
|---|---|---|---|---|
| England 1086 (Domesday) | 3.5-5.0 | recorded household | `england-1086.household-multiplier` | unverified |
| England, later medieval | 4.5-5.0 | household, tax records | `england-later-medieval.household-size` | unverified |
| Florence c. 1430 | 3.8 city, 4.4 countryside | hearth | `florence-1430.*` | unverified |
| Prato, 15th c. | 3.7 city, 5 countryside | household | `prato-15c.*` | unverified |
| Silchester | 5 | dwelling, assumed | `silchester.household-size` | unverified |
| Salvador c. 1730 | 5 (report's arithmetic) | fogo | `salvador-1730.households` | unverified |
| Aleppo 1537-1683 | 8 | housing unit | `aleppo-1537.persons-per-unit` | unverified |
| Algiers 1789 | 10 (report's division) | house | `algiers-1789.houses` | unverified |
| Ahmedabad 1872 | 3.01-3.35 | house | `ahmedabad-1872.people-per-house` | unverified |
| Lo Manthang | 5 (arithmetic here: 900 / 180) | house | `lo-manthang.population` | unverified |
| Game source | 3 (2-5) | dwelling | `game.ross-people-per-dwelling` | unverified, not scholarship |

People per institution: 1,000 per mosque or bath (data-set assumption,
`islamic-medieval.people-per-mosque`); 1,000 per place of worship in an Ottoman mahalle
(`ottoman.mahalle-people-per-worship`); one bath per 3,500 people in 18th-century Cairo
(`cairo-18c.people-per-bath`).

### 2.10 Settlement sizes

Rows run from their lower bound up to, but not including, the next row's lower bound. An
example given as a range sits in the row of its lower bound; its upper bound may fall in the
next row.

| Size | Examples with fact ids |
|---|---|
| under 1,000 | Kaupang 400-1,000, Birka 700-1,000, Sigtuna 750, Tsumago-juku 418, Magome-juku 717, Lo Manthang 900, Asperen 900, Mbanza Kongo 1881 600 |
| 1,000 to under 5,000 | Hedeby 1,500, Caernarfon 1,000-1,500, Monpazier 1,500-2,500, Narai-juku 2,155, Visby c. 1600 2,000-3,000, Palmanova, Zamość, Neuf-Brisach, Mapungubwe 3,000-5,000, Siyâsa 4,000, Dublin c. 1170 4,500 |
| 5,000 to under 20,000 | Silchester 5,500, Pompeii 8,000-12,000, Djenné 1828 8,000-10,000, Timbuktu 1828 10,000-12,000, Edinburgh 12,000-15,000, Leiden 1581 12,016 |
| 20,000 to under 100,000 | Novgorod peak "barely 30,000", Nuremberg c. 1575 40,000, Leiden 1622 44,745, Kagoshima 58,000, Shahjahanabad design 60,000, Aleppo 1537 80,000, Old Oyo 60,000-140,000 |
| 100,000 and over | Ahmedabad 1872 116,873, Paris c. 1380 over 150,000, Benares 1901 209,331, Tenochtitlan 212,500, Cairo 1798 263,000, Edo late period 1.3 million |

The game source gives size classes (`game.ross-village-population` and siblings): village
20-1,000, town 1,000-8,000, city 8,000-12,000, big city 12,000-100,000. The published study of
European city origins counts a place as a city from 5,000 people (`europe.city-threshold`,
verified).

### 2.11 Summary by culture and era

Medians and ranges are over the facts above, per the area basis named. A row whose basis
names one basis is not pooled across bases. A row marked "mixed" combines facts on the bases it
names; its range is a list of those facts' values, not a comparison on one basis. The last
column lists the facts each range rests on.

| Culture or region | Era | Density per ha | Basis | Facts the range rests on |
|---|---|---|---|---|
| Norse and Hiberno-Norse ports | early medieval | 60-140 | mixed: walled (Hedeby), site (Birka) | `hedeby.density`, `birka.density` |
| Western and central Europe | high medieval c. 1300 | 93-152 (median 114) | settled | `europe-1300.density-interquartile` (verified) |
| Low Countries | late medieval to early modern | 40-300+, mean 75.5 → 126.5 | walled | `netherlands-1400.mean-density`, `netherlands-1560.mean-density` |
| Roman | ancient | 91-427, rising with town area | mixed: walled or site | `roman.hanson-scaling-exponent` (verified) |
| Arab cities (Ottoman period) | early modern | 300-600 | mixed: whole city (Cairo, Tunis), walled core (Qahira) | `cairo-1798.density`, `cairo-1798.qahira-density`, `tunis-1881.density` |
| Sahel | modern measurements | 221-389 | town | `djenne-modern.density`, `niger-delta.traditional-town-density` |
| Yoruba capitals | early modern | 20-47 | gross, whole wall circuit | `old-oyo.gross-density` |
| South Asia | 19th c. | 84-349 by division, 494 in one pol | mixed: division (Ahmedabad), municipality (Benares), pol | `ahmedabad-1875.*`, `ahmedabad-1879.mandvi-pol-*`, `benares-1901.density` |
| China, Song capitals | high medieval | 120-350 | city | `kaifeng-northern-song.density`, `hangzhou-*` |
| Japan, Edo | early modern | 57 / 168 / 673 by class | land class | `edo-late.*-density` |
| Mesoamerica, Postclassic | late medieval | 10-72; Tenochtitlan 157 | city | `postclassic-mesoamerica.density-other-cities` (verified) |
| Classic Maya | early medieval | 5 (south), 12-28 (north) | whole mapped city | `maya-classic.*` (corrected, verified) |

## 3. Conflicts

Each entry names the competing fact ids, what differs, the reason when a report gives one,
and the status set.

1. **Russell's 100-120 per ha is not a medieval figure.** `ancient.russell-average`
   (parcels-density.md: medieval Europe) against `europe-1300.density-median` (114).
   Verification-population.md #2 traces the only citable "100-120" to Russell's 1977 Pompeii
   paper on ancient cities. Status: `corrected` (scope). The Cesaretti median replaces it.
2. **"Rarely above 200" is not supported.** `europe-medieval.russell-rarely-above-200` against
   `europe-1300.share-above-200` (11%, 19 of 173 towns). Status: `superseded`.
3. **The Roman 150-250 (100-400) band has no source.** `roman.likely-band`,
   `roman.outer-band` against `roman.hanson-scaling-exponent`. The band matches the law only
   for towns of 45-200 ha. Status: both `superseded`.
4. **Classic Maya core density.** `maya-classic.core-canuto` (10-20 per ha in
   parcels-density.md) against `tikal.density`, `caracol.density` (5 per ha) and
   `maya-classic.northern-compact` (12-28). 10-20 fits only compact northern cities. Status:
   `corrected` to 5.
5. **Rounding in the worked Cesaretti values.** `europe-1300.settled-area-at-10000` (86 ha in
   parcels-density.md, 85 ha in the verification) and `europe-1300.density-at-50000` (185 in
   the parcels-density.md table, 186 in its text and the verification). Status: `corrected`.
   Both reports compute with the prefactor 0.119. With it, 10,000 people give 85.4 ha and
   50,000 people 185.5 per ha (arithmetic here), so nearest-whole-number rounding gives the
   verification's 85 ha and 186 per ha. With the prefactor 0.1194 the values are 86 ha and
   185 per ha (section 2.1).
6. **Pompeii population.** `pompeii.population-hanson` (8,000-12,000) against
   `pompeii.population-corpus` (11,000-15,000, unverified) and `pompeii.population-older`
   (30,000-45,000, 400 per ha, called unsustainable by Hanson). The reports give no reason for
   the corpus range. Status: older figure `superseded`; Hanson's range and the corpus range
   `disputed`, each listing the other.
7. **Timgad design population against its walled area.** `timgad.design-population`
   (15,000) on `timgad.walled-area` (12 ha) gives 1,250 per ha (arithmetic here), above every
   accepted value. The corpus says the suburbs later grew to 3-4 times the core. Neither report
   says whether 15,000 counts the suburbs. Status: `unverified`. A population against an area is
   not two values for the same thing, so it is not a dispute.
8. **Tenochtitlan area and density.** `tenochtitlan-1519.area-smith` (1,350 ha including
   Tlatelolco, verified) against `tenochtitlan.area-1519-valero` (750 ha) and
   `tenochtitlan.area-1519-corpus` (13 km²); `tenochtitlan.density-1519-valero` (12,500-13,000
   per km², that is 125-130 per ha) against `tenochtitlan-1519.density-smith` (157.4). The
   Iberian report notes that 150,000 on 7.5 km² would be 20,000 per km², so Valero's author
   uses a larger area that the page does not name. Status: Smith `verified`, the others
   `disputed`. The three area facts list each other; the two density facts list each other.
9. **Shahjahanabad area.** `shahjahanabad-1650.area-sahapedia` (712 ha) against
   `shahjahanabad.area-second-source` (569 ha, search summary) and
   `shahjahanabad-1857.walled-area` (600 ha, unverified). No report gives a reason. Status: all
   three `disputed`.
10. **Amsterdam walled area.** `amsterdam-1514.walled-area` (75 ha, Visser) against
    `amsterdam-1544.area` (100 ha, world corpus). Thirty years apart; no report says whether
    the wall moved. Status: both `disputed`, each listing the other.
11. **Timbuktu population.** `timbuktu-16c.population` (100,000, UNESCO, no primary record)
    against `timbuktu-15c-16c.population-larousse` (25,000 for the 15th-16th century, a
    Larousse search snippet). Status: both `disputed`, each listing the other. Caillié's
    10,000-12,000 (`timbuktu-1828.population`) and Barth's 13,000 (`timbuktu-1853.population`)
    are for later dates and are not part of the dispute; the africa.md report says the sources
    do not average and a spec must pick an era.
12. **Great Zimbabwe area.** `great-zimbabwe.area-800` and `great-zimbabwe.area-80`, both on
    the same UNESCO page. Status: both `disputed`, each listing the other.
    `great-zimbabwe.daga-area` (700 ha of huts, corpus) measures a different thing and stays
    `unverified`.
13. **Algiers and Aleppo above Raymond's accepted range.** `algiers-1789.population` (50,000)
    on `algiers-1789.area` (41.1 ha) gives 1,217 per ha (the report's division); Raymond
    rejects 2,433 per ha for Algiers and 710 for Aleppo (`algiers.density-rejected`,
    `aleppo.density-rejected`) and accepts 300-600 elsewhere. Status: all three `unverified`.
    No source gives a second value for the same thing, so none of the three is a dispute.
14. **Corpus populations that cover more land than the stated area.** `hangzhou-1270.*`
    (population for the prefecture, area for the walled city: 1,000 per ha, against Bao's 210-350
    from `hangzhou-1241.density`, `hangzhou-1265.density`), `guangzhou-1855.*` (962-1,923),
    `bangkok-1861.*` (732-976), `baghdad-800.*` (545-909), `mexico-city-1628.*` (500), `ur.*`
    (1,083). The world corpus names the mismatch in its own notes for Hangzhou, Guangzhou,
    Bangkok, Baghdad and Mexico City; for Ur it gives no reason. Status: `unverified`; these
    pairs do not give a density.
15. **Islamic data-set assumption against measured Arab densities.**
    `islamic-medieval.compact-city` (150, a modelling assumption) against
    `cairo-1798.density` (360), `cairo-1798.qahira-density` (588), `tunis-1881.density` (380).
    The assumption is for a medieval city, the measurements for 1798-1881. Status: all
    `unverified`.
16. **Walled, settled, built and gross areas give different densities for the same town.**
    Cairo 1798: 360 per ha on the whole city (`cairo-1798.density`) and 398 per built-up ha
    (`cairo-1798.built-up-density`). Bristol c. 1300: 130 ha settled
    (`bristol-1300.settled-area`) against 55 ha walled (`bristol-1300.walled-area`), a factor
    of 2.4 in density. The Low Countries figures are per walled ha, Cesaretti's per settled ha,
    Old Oyo's per gross ha. Continental.md says the Dutch figures are comparable to the British
    ones only if those are also per walled ha. Status: not a disagreement between sources but a
    difference of definition; every fact note names its basis.
17. **Dublin.** `dublin-1170.density-bound` (375 per ha, if 4,500 people lived in 12 ha)
    against `dublin-1610.*` (10,000-15,000 on 20 ha walled, 500-750 per ha, arithmetic here).
    The corpus lists Dublin among towns whose suburbs exceed the walled core, so the 1610
    population counts suburbs. Status: `unverified`.
18. **Harar population.** `harar-1876.population` (35,000, unverified) against Burton's 8,000
    for 1855 named in the same note, on 48 ha (`harar.walled-area`): 729 against 167 per ha
    (arithmetic here). The report gives no date for the 35,000 and 1855 is a different date.
    Status: `unverified`.
19. **Chan Chan population.** `chan-chan.population` "over 100,000" against
    `chan-chan.population-other-sources` "20,000 to 60,000", which the same page gives for
    other sources it does not name. Status: both `disputed`, each listing the other.
20. **Edo townspeople land share.** `edo-late.townspeople-land-share` (16%, Tokyo
    Metropolitan Government) against `edo-chonin-land-share-unverified` (18%, search summary).
    Status: both `disputed`, each listing the other; the booklet figure is the fetched one.
21. **Tsumago-juku houses.** `tsumago-1843.houses` (31) conflicts with 31 hatago (inns) on the
    same page. Status: `unverified`. One source printing two different counts is not two
    sources giving values for the same thing.
22. **Game source against data.** `game.ross-average` (150) against
    `europe-1300.density-median` (114). Verification-population.md says the popular "150 per
    ha" comes from Ross, not from Russell. Status: `unverified`, marked as a game source.

Not a conflict: Edo late period 1.3 million (`edo-late.population`) against 800,000-1,000,000
c. 1700 (world corpus) are different dates, and the 1721 count (500,000 townspeople plus
500,000 samurai, `edo-1721.townspeople-population`) fits the corpus figure. Hedeby 1,500
(`hedeby.population`) lies inside the corpus range 1,000-1,500. Cairo 263,000 (Raymond)
agrees with the corpus 260,000; Aleppo built-up 349 ha (Raymond) agrees with the corpus 350
ha; Harar 48 ha appears in both africa.md and the corpus.

## 4. Gaps

- **Europe, early medieval**: no measured density; Hedeby, Birka and Kaupang densities are
  derived by the report from encyclopedia populations and areas. No figure for a town above
  1,500 people before 1100 except Dublin 4,500 (c. 1170, a source timber-ports.md calls weak)
  and an unverified York 10,000.
- **Europe, German cities**: no density per ha for Cologne, Lübeck or Nuremberg; Paris 1328
  has a hearth count (`paris-1328.hearths`) but no verified area.
- **Ratio of walled to settled area**: one data point (Bristol, 55 of 130 ha). Question 2A
  needs no conversion; question 2B depends on this ratio; question 2C depends on a ratio of
  built to settled area, for which no European figure was found.
- **Households per plot and buildings per dwelling**: not found in scholarship; only the game
  source (1-2 buildings per dwelling).
- **Medieval European people per dwelling**: only the Domesday and Italian household
  multipliers; no figure per house.
- **Building coverage per plot and land-use shares**: only Edo (68/16/16), Kagoshima
  (70/30), Tenochtitlan's epicentre (1.25%) and the Mexico City centre in 1794 (46.23% built).
- **China**: no density for an ordinary Ming-Qing county seat (Boluo is a snippet); no Tang
  Chang'an density.
- **Japan**: no castle town other than Edo and Kagoshima has class shares or densities.
- **Korea and Southeast Asia**: only unverified world-corpus pairs.
- **Spanish and Portuguese colonial towns**: no density per ha for Mexico City (1790 census)
  or Lima; only people per block (Buenos Aires) and population series (Salvador).
- **Sub-Saharan Africa**: the measured densities (Djenné, Agadez) are modern; historic values
  are derived by the report.
- **Settlements under 1,000 people**: below the Cesaretti range; only the game source,
  three timber ports (Birka, Kaupang, Sigtuna) and two corpus items (Lo Manthang, Pueblo Bonito).
- **Cities over 200,000 people**: above the Cesaretti range; only unverified corpus figures
  and Edo, whose density is split by land class.
- **Density by wealth inside a European town**: not found. Class splits exist only for Edo;
  district splits only for Cairo 1798 and Ahmedabad 1875.
- **Fantasy factors** (magic, ancestries, Jotun-scale buildings in Truro): no figures in the
  reports. Table-use.md quotes the game source: "If magic is common, the GM may decide a
  square mile of land can support many more" people. These are GM inputs.

## 5. What this means for the spec

The spec has one size input, `settlement.population` (1 to 1,000,000), and an era preset,
`presets.era` (`early_medieval`, `high_medieval`, `late_medieval`). The field table says
population "drives built area with the era preset". REQUIREMENTS.md tier T1 asks: "Is the
built area within 30% of population times the preset's area per person?" Version 1 has no
culture field; culture comes from rule packs (ADR-006). The questions below are the density
defaults the generator needs. Jeffrey decides each one; the recommendation is Claude's.

**Question 1. How does population become an area?**
Decision needed: the formula behind "drives built area with the era preset".
- A. Constant area per person per era preset (linear: area = population × m² per person). This
  is what the T1 wording assumes. Rests on: `netherlands-1400.mean-density`,
  `netherlands-1560.mean-density`, `game.ross-average`. Visser himself warns that averages
  cannot give one town's population from its area (continental.md).
- B. The Cesaretti scaling law, the same for all three era presets: area_ha = 0.1194 ×
  population^0.714. Rests on: `europe-1300.scaling-exponent`, `europe-1300.scaling-prefactor`
  (both verified). The early-medieval ports fall near the line: at 1,500 people the law gives
  22.1 ha (arithmetic here) and Hedeby measures 24-26 ha (`hedeby.area`).
- C. The scaling law with the exponent fixed at 0.714 and a different prefactor per era
  preset. Rests on the law plus `netherlands-1400.mean-density` against
  `netherlands-1560.mean-density` (Dutch towns grew denser from 1400 to 1560). No report gives
  a prefactor per era; the values would be uncalibrated.
Recommendation: B, with the prefactor and exponent stored as rule-pack data so a pack can
change them (question 6). Rejected: A (a constant area per person contradicts the verified
law, under which bigger towns are denser, and Visser warns against it); C (no report gives a
prefactor per era). With B, the T1 check compares the area named by question 2 with
0.1194 × population^0.714 ha; the field table and the T1 wording then name the area that
question 2 picks. If Jeffrey picks 2A, T1 measures the settled area and no conversion from
settled to built area is needed. If he picks 2B, T1 measures walled area and needs a ratio
of walled to settled area (one data point, section 4). If he picks 2C, T1 keeps measuring
built area and needs a ratio of built to settled area; no European built-area figure was found.

**Question 2. Which area does that formula give?**
Decision needed: what the generator fills with the formula's area, and what T1 measures.
- A. Settled area: the town plus suburbs plus empty land inside walls, as Cesaretti measured.
  Walls then enclose part of it. Rests on: `europe-1300.scaling-prefactor` (its note), 
  `bristol-1300.settled-area` (130 ha) and `bristol-1300.walled-area` (55 ha), the only
  walled-to-settled pair found.
- B. Walled area: everything inside the outermost wall. Rests on the Visser figures
  (`netherlands-*-density`) and `game.ross-10000-walled-area` (67 ha for 10,000 people).
- C. Built area: built-up land and streets only. Rests on `cairo-1798.built-up-area` and
  `aleppo-1516.built-up-area`; no European built-area figure was found.
Recommendation: A, because the verified law is defined on settled area. The share inside the
walls then comes from the spec's `encloses` lists, not from a density. Rejected: B (walled
area needs a walled-to-settled ratio, and Bristol is the only pair found); C (no European
built-area figure was found). With A, the T1 field and the field table name settled area.

**Question 3. Does the seed vary the density, and by how much?**
Decision needed: whether two cities with the same population and era get the same area.
- A. No variation: always the law's value. Rests on: `europe-1300.scaling-prefactor`.
- B. A seeded factor inside the middle half of the Cesaretti sample: density between 0.82 and
  1.33 times the law's value (93/114 and 152/114, arithmetic here). Rests on:
  `europe-1300.density-interquartile`, `europe-1300.density-median`.
- C. A seeded factor over the full observed range, 47 to 340 per ha. Rests on:
  `europe-1300.density-range`.
Recommendation: B. It keeps cities different (REQUIREMENTS.md G8) without producing the rare
extremes by chance; a spec that wants an extreme uses a rule pack. Rejected: A (every city of
one size would get the same area, against G8); C (the extremes of 173 towns would come up by
chance).

**Question 4. What happens outside the law's range of 1,000 to 200,000 people?**
Decision needed: the spec accepts 1 to 1,000,000; the law is fitted on 1,000 to 200,000.
- A. Extrapolate silently. At 1,000,000 people the law gives 2,296 ha and 436 per ha
  (arithmetic here); Edo's townspeople land reached 673 per ha (`edo-late.townspeople-density`)
  and Beijing c. 1800 167 per walled ha (`beijing-1800.*`, unverified).
- B. Extrapolate and print a warning naming the range. Rests on the range stated in
  `europe-1300.scaling-prefactor`; small towns below it: `lo-manthang.*` (900 on 3 ha),
  `birka.*` (700-1,000 on 7 ha).
- C. Reject the spec with exit 2 outside the range.
Recommendation: B. The calibration cities (Truro and Bearna at 10,000) lie inside the range;
the warning tells a GM when the numbers have no data behind them. Rejected: A (the GM is not
told that the numbers have no data behind them); C (the spec accepts 1 to 1,000,000 people,
and a valid spec would fail).

**Question 5. Is density the same everywhere inside the town?**
Decision needed: whether districts differ in density (needed by milestone 6, parcels and
buildings, and tier T5 "density gradient").
- A. Uniform density across all districts.
- B. A fixed gradient: denser at the centre, sparser at the edge. Rests on:
  `cairo-1798.qahira-density` (588 in the walled core) against `cairo-1798.density` (360
  overall); Ahmedabad divisions from 116 to 349 (`ahmedabad-1875.*-density`).
- C. A density multiplier per district function and wealth, as rule-pack data, default 1 (that
  is, uniform until a pack sets it). Rests on: Edo by land class (`edo-late.samurai-density`
  168.2, `edo-late.townspeople-density` 673.2, `edo-late.temple-density` 56.8) and the
  Ahmedabad divisions.
Recommendation: C. It needs no new constraint kind, starts uniform, and lets a Japanese or
Indian pack express the measured splits. Rejected: A (no pack could express the measured
splits); B (a fixed gradient does not fit Edo, where density follows land class, not distance
from the centre).

**Question 6. May a rule pack replace the European law?**
Decision needed: version 1 has no culture field; the measured densities outside Europe differ
by a factor of more than 100 (5 per ha for Tikal against 673 for Edo townspeople land).
- A. No: the three era presets are the only density source in version 1.
- B. Yes: a pack may set the prefactor, the exponent and the variation band. Rests on: Arab
  cities 300-600 (`cairo-1798.density`, `tunis-1881.density`), Classic Maya 5
  (`tikal.density`), Old Oyo 20-47 gross (`old-oyo.gross-density`), the Sahel 221-389
  (`djenne-modern.density`, `niger-delta.traditional-town-density`).
- C. Yes, and a pack may also switch to the Roman form, population from area
  (`roman.hanson-scaling-exponent`), which needs a second formula in code.
Recommendation: B. It is data only (ADR-017 closed kinds are untouched). Rejected: A (every
culture would get the European density, against measured values that differ by a factor of
more than 100); C (it adds code for one culture; it can follow when a Roman pack is written).

**Question 7. Truro: which density for a 10,000-person early-medieval timber port?**
Decision needed: Truro (REQUIREMENTS.md §8.1, population 10,000, `early_medieval`) is larger
than every verified timber port (`hedeby.population` 1,500; `york-10c.population` 10,000 is
unverified). The answer to question 1 decides this unless Jeffrey wants Truro to differ.
- A. The law: 86 ha settled, 117 per ha (0.1194 × 10,000^0.714 = 85.7 ha, arithmetic here;
  the report's 85 ha uses the prefactor 0.119). Rests on: `europe-1300.settled-area-at-10000`.
- B. Hedeby's density: 60 per ha, 167 ha (arithmetic here). Rests on: `hedeby.density`.
- C. Birka's density: 100-140 per ha, 71-100 ha (arithmetic here). Rests on: `birka.density`.
Recommendation: A. It follows question 1B, and Hedeby at 1,500 people already sits on the law
(68 per ha predicted against 60 measured). Rejected: B (it treats a small port's density as a
large one's); C (Birka has 700-1,000 people, so C does the same).

**Question 8. What role does the game source play?**
Decision needed: whether Medieval Demographics Made Easy (`game.ross-average` 150 per ha,
`game.ross-range` 75-300, `game.ross-10000-walled-area` 67 ha) feeds a default or a check.
Verification-population.md names it as the origin of the popular "150 per ha" (conflict 22).
- A. None: no default and no check rests on it.
- B. A sanity warning only, for example "walled area too small for the population" as
  table-use.md suggests.
- C. A default density.
Recommendation: A. Verified data covers the same ground (`europe-1300.density-median`); a
warning built on a game source would contradict the generator's own law at small
populations (the law gives 60 per ha at 1,000 people, the game source 150). Rejected: B (for
the same reason: the check would contradict the generator's own law); C (a game source would
replace verified data).

**Question 9. People per dwelling, for building counts at milestone 6.**
Decision needed: the number that turns population into dwellings when parcels and buildings
are generated.
- A. 5 per dwelling. Rests on: `silchester.household-size` (an assumption),
  `salvador-1730.households`, `prato-15c.countryside-household`.
- B. A per-era band of 3.5-5.0 per household. Rests on: `england-1086.household-multiplier`,
  `england-later-medieval.household-size`, `florence-1430.city-household` (3.8).
- C. A rule-pack value with a European default of 3.8 (urban Florence) and culture values
  from the facts (Aleppo 8, `aleppo-1537.persons-per-unit`; Ahmedabad 3.0-3.35,
  `ahmedabad-1872.people-per-house`).
Recommendation: C. Urban households were smaller than rural ones in both Italian cases, so
an urban figure fits a city map; the value is needed at milestone 6, not before. Rejected: A
(5 rests on an assumption, Salvador's fogos and a countryside figure, while both Italian
urban figures are smaller); B (one band for every culture, while Aleppo's 8 and Ahmedabad's
3.0-3.35 lie outside it).
