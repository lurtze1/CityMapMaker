# Verification batch: group A, batch 1 (issue #74)

Date: 2026-11-09. Method: issue #74. Each fact's original `source` was opened and the figure
looked up. Outcomes: `verified` (the figure matches), `corrected` (the figure differs),
`disputed`, or still `unverified` with a note saying why the source could not be checked.

## Which facts

Group A is the set of facts that the questions A1 to A15 of
`documentation/research/cross-analysis/README.md` rest on (core questions 1 to 6, footprints
questions 1 to 8, density question 4). The verification batch of 2026-10-21 (PR #75) already
checked the facts of group A that it could reach. This batch takes every fact of group A that
was still `unverified` and has an outside source (19 facts), then the unverified facts of
group B with an outside source (23 facts: density questions 1 to 3 and 5 to 8; the wall
questions of group B name no fact identifiers).

Not taken: 16 unverified facts of group A and one of group B whose `source` is a
CityMapMaker report (`citymapmaker-*`), so there is no outside original to open:
`core.timber-ports.spine-parallel-shore-towns`, `core.corpus-world.market-water-edge`,
`core.corpus-world.ruler-edge-high`, `core.cliff-towns.citadel-on-top-towns`,
`core.peninsula-ports.citadel-at-tip-towns`, `core.timgad.founding-square-side`,
`footprints.kings-head-aylesbury.estimated-side`, `footprints.zamosc.great-market-side`,
`footprints.neuf-brisach.place-darmes-side`, `footprints.savannah.ward-square-length`,
`footprints.savannah.ward-square-width`, `footprints.bath.circus-diameter`,
`density.beijing-1800.population`, `density.beijing-1800.area`,
`density.lo-manthang.population`, `density.lo-manthang.area` and `density.york-10c.population`.

## Counts

| Outcome | Facts |
|---|---|
| verified | 36 |
| corrected | 0 |
| disputed | 0 |
| still unverified | 6 |
| total | 42 |

No value changed. Each changed fact gains a dated note; a fact without a `locator` gains one
with the source's wording.

## Outcome per fact

Every fact listed was `unverified` before this batch.

### Group A

| Fact | New status | Source locator | Evidence |
|---|---|---|---|
| `footprints.bastides.early-square-side` | verified | Gérard, PDF page 4 | "D'abord de superficie moyenne (de 50 à 55 mètres de côté)"; the source gives no year for these first squares |
| `footprints.monflanquin.square-side` | verified | Gérard, PDF page 4 | "Montflanquin en Agenais (1268) a encore 60 mètres de côté" |
| `footprints.revel.square-side` | verified | Gérard, PDF page 4 | "Revel (1342) 90 mètres" |
| `footprints.marciac.square-length` | verified | Gérard, PDF page 4 | "Marciac (1298), 70 x 140 m" |
| `footprints.marciac.square-width` | verified | Gérard, PDF page 4 | "Marciac (1298), 70 x 140 m" |
| `footprints.rabastens.square-length` | verified | Gérard, PDF page 4 | "Rabastens-de-Bigorre (1306), 75 x 150 m" |
| `footprints.rabastens.square-width` | verified | Gérard, PDF page 4 | "Rabastens-de-Bigorre (1306), 75 x 150 m" |
| `footprints.trie.square-length` | verified | Gérard, PDF page 4 | "Trie-sur-Barse (1324), 75 x 130m" |
| `footprints.trie.square-width` | verified | Gérard, PDF page 4 | "Trie-sur-Barse (1324), 75 x 130m" |
| `footprints.laws-of-indies.plaza-max-width` | verified | Altamira, Ordinance 113, pages 262 to 263 | "quinientos y treynta pies De ancho"; the editor's footnote calls "treynta pies" a copyist's error where the document is torn |
| `footprints.laws-of-indies.plaza-good-width` | verified | Altamira, Ordinance 113, pages 262 to 263 | "seisçientos pies de largo y quatroçientos de ancho" |
| `footprints.aleppo-souq-core.length` | verified | Raymond, Remarques sur la voirie, paragraph 18 | "sur une longueur de 250 mètres et une largeur de 40" |
| `footprints.aleppo-souq-core.width` | verified | Raymond, Remarques sur la voirie, paragraph 18 | same sentence |
| `footprints.dunster-yarn-market.area` | verified | Historic England 1015706, Internet Archive copy of 2024-01-30 | "The building is 9.4m in diameter"; built 1609 |
| `footprints.dupath-well.area` | verified | Historic England 1013663, Internet Archive copy of 2024-12-01 | "3.9m north east - south west by 3.59m north west - south east externally" |
| `footprints.speyer-altportel.area` | verified | structurae.net, Internet Archive copy of 2026-06-24 | "dimensions at base ca. 13.00 m x 8.70 m" |
| `footprints.conwy.overall-area` | unverified | none | ancient-history-sites.com returns 403 and the Internet Archive holds no copy |
| `footprints.conwy.overall-length` | unverified | none | same; ermakvagus.com gives 125 x 35 m, which is not this source |
| `density.edo-late.townspeople-density` | verified | Tokyo Metropolitan Government, page 017, density chart | 町人地 673.2 per hectare; the same page's 60万人 on 8.9 square kilometres gives 674 |

### Group B

| Fact | New status | Source locator | Evidence |
|---|---|---|---|
| `density.netherlands-1400.mean-density` | verified | Visser 1985, page 15 | "De gemiddelden voor 1560 en 1400 zijn dan respectievelijk 126,5 en 75,5 inwoners per hectare" |
| `density.netherlands-1560.mean-density` | verified | Visser 1985, page 15 | same sentence; 3.250 hectares for 1560 |
| `density.game.ross-average` | verified | Ross, PDF page 6 | "somewhere around 150 per hectare (61 per acre)" |
| `density.game.ross-10000-walled-area` | verified | Ross, PDF page 6 | "a city of 10,000 would be around 67 hectares" |
| `density.game.ross-range` | verified | Ross, PDF page 6 | "close to 300/ha ... as low as 75"; "roll 7d4 and drop the two highest dice" |
| `density.hedeby.area` | verified | German Wikipedia, Haithabu | "umschloss 26 Hektar Fläche"; the 24 hectares of the range is World History Encyclopedia's figure, also checked |
| `density.hedeby.population` | verified | World History Encyclopedia, Hedeby | "a population of approximately 1,500 people", around 900 |
| `density.hedeby.density` | verified | World History Encyclopedia, Hedeby | 1,500 on 24 to 26 hectares gives 58 to 63 per hectare |
| `density.bristol-1300.walled-area` | verified | Cesaretti and others 2016, Bristol paragraph | "Most maps of Bristol's walled area ca.1300 match Kermode's 55 ha" |
| `density.cairo-1798.density` | verified | Raymond, Signes urbains, paragraph 15 | "la densité est de 360 h/ha" on 730 hectares; dated "la fin du XVIIIe siècle" |
| `density.cairo-1798.built-up-area` | verified | Raymond, Signes urbains, paragraph 15 | "la surface bâtie (rues comprises) est ramenée à 662 ha" |
| `density.cairo-1798.qahira-density` | verified | Raymond, Signes urbains, paragraph 15, table | al-Qahira 153 hectares, about 90,000 inhabitants, 588 per hectare |
| `density.aleppo-1516.built-up-area` | verified | Raymond, The population of Aleppo, paragraph 47 | "between 1516 (238.5 ha)" and the early nineteenth century (349 ha) |
| `density.ahmedabad-1875.shahapur-density` | verified | Bombay Gazetteer, page 294, footnote 1 | "Shahapur, 46.96" people per acre; times 2.471 gives 116.0 |
| `density.ahmedabad-1875.jamalpur-density` | verified | Bombay Gazetteer, page 294, footnote 1 | "Jamalpur, 68.24"; gives 168.6 |
| `density.ahmedabad-1875.dariyapur-density` | verified | Bombay Gazetteer, page 294, footnote 1 | "Daryapur, 114.77"; gives 283.6 |
| `density.ahmedabad-1875.khadia-density` | verified | Bombay Gazetteer, page 294, footnote 1 | "Khadiya, 141.38"; gives 349.3 |
| `density.edo-late.samurai-density` | verified | Tokyo Metropolitan Government, page 017, density chart | 武家地 168.2; 65万人 on 38.7 square kilometres gives 168 |
| `density.edo-late.temple-density` | verified | Tokyo Metropolitan Government, page 017, density chart | 寺社地 56.8; 5万人 on 8.8 square kilometres gives 57 |
| `density.tunis-1881.density` | unverified | none | Lézine's own work is not online; Raymond, Signes urbains, paragraph 15, the work the report used, gives "pour Tunis en 1881 : 380 h/ha" |
| `density.old-oyo.gross-density` | unverified | none | Agbaje-Williams 1983 is not online |
| `density.djenne-modern.density` | unverified | none | the IRD server served a maintenance page; the Internet Archive holds no copy |
| `density.niger-delta.traditional-town-density` | unverified | none | same |

## Points for Jeffrey

- `footprints.laws-of-indies.plaza-max-width`: the figure 530 pies matches the printed
  transcription, but the editor's footnote at that place calls "treynta pies" a copyist's
  error where the document is torn. The fact is `verified` against the printed text, and the
  note quotes the footnote.
- The Edo chart pairing that the batch of 2026-10-21 could not check now holds: the chart's
  bar order and the areas and populations on the same page give the same three densities.
- No recommendation of a group A or group B question changes, because no value changed.

## Checks

`python analysis/validate_facts.py`: "1829 facts, 554 sources, 69 units, 94 quantities:
0 errors". `python -m unittest discover -s analysis/tests`: 56 tests, OK.
