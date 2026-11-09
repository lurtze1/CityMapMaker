# Verification batch: group C, batch 1 (issue #74, batch 2)

Date: 2026-11-09. Method: issue #74. Each fact's original `source` was opened and the figure
looked up. Outcomes: `verified` (the figure matches), `corrected` (the figure differs),
`disputed`, or still `unverified` with a note saying why the source could not be checked.

## Which facts

Group C is the set of facts that the questions C1 to C14 of
`documentation/research/cross-analysis/README.md` rest on: streets questions 1 to 11 in
`cross-analysis/streets.md` and walls questions 2, 5 and 6 in `cross-analysis/walls.md`. This
batch takes every fact those questions name that was still `unverified` and has an outside
source (44 facts). Group C then ran out, so the batch continues with group D: the six facts
of the plots questions that share a source with group C (Séraphin 2008), which makes 50.

Not taken from group C:

- Four unverified facts whose `source` is a CityMapMaker report (`citymapmaker-*`), so there
  is no outside original to open: `streets.monpazier.carreyras`,
  `streets.monpazier.carreyrous-corpus`, `streets.monpazier.androns` and
  `walls.nuremberg.outer-gates`.
- Facts with another status: `verified` (the Merv, deformed-grid, Boeing, Ávila, Conwy,
  Paris and York facts) or `disputed` (`streets.villefranche.lanes`, `streets.bastides.alleys`,
  `streets.arthashastra.royal-road-sanskrit`, `streets.arthashastra.royal-road-shamasastry`).

## Counts

| Outcome | Facts |
|---|---|
| verified | 41 |
| corrected | 0 |
| disputed | 0 |
| still unverified | 9 |
| total | 50 |

No value changed. Each fact gains a dated note; a fact without a `locator` gains one with the
source's wording. Two other fields changed: `plots.figeac.head-house-length` gains
`approximate: true`, because the source says "environ", and the locator of
`walls.arthashastra.tower-spacing` moves from KAZ 2.3.10 to KAZ 2.3.11.

## Outcome per fact

Every fact listed was `unverified` before this batch.

### Séraphin 2008, Un modèle de parcellaire médiéval

| Fact | New status | Source locator | Evidence |
|---|---|---|---|
| `streets.figeac.main-street` | verified | page 39 | "La rue principale (rue de l'Ortabadial), large de 4 m à 4,50 m" |
| `streets.figeac.cross-lanes` | verified | page 39 | "moitié moins larges (de 2 à 2,40 m), souvent en impasse" |
| `streets.figeac.back-gap` | verified | page 39 | "une venelle de moins de 0,50 m de largeur, impropre à la circulation" |
| `streets.villefranche.main-streets` | verified | page 45 | "de 5,50 à 6 m de large pour les plus importantes" |
| `plots.figeac.block-width` | verified | page 39 | "une largeur de 11,50 m à 12 m pour une longueur de près de 50 m" |
| `plots.figeac.back-gap` | verified | page 39 | same sentence as `streets.figeac.back-gap` |
| `plots.figeac.row-house-depth` | verified | page 39 | "Pour une profondeur calibrée de 5,80 à 6 m" |
| `plots.figeac.head-house-length` | verified | page 39 | "de 6 x 12 m environ"; `approximate` set |
| `plots.villefranche-de-rouergue.house-depth` | verified | page 45 | "4,50 à 7,50 m de façade sur 7 à 7,50 m de profondeur" |
| `plots.bilbao.block-length` | verified | page 47 | "une trentaine de mètres de largeur et plus de 80 m de longueur" |

### Sáinz Guerra 2014, Las Villas Nuevas Medievales de Castilla y León

| Fact | New status | Source locator | Evidence |
|---|---|---|---|
| `streets.castile-new-towns.main-street` | verified | page 177 | "La calle Mayor ... tiene una anchura alrededor de las 8 varas (6,7 metros)"; this is Peñaflor de Hornija |
| `streets.castile-new-towns.streets` | verified | pages 172 and 176 | "calles de 5 varas de ancho (4,2 metros)" (Vertavillo); "La anchura de 4 varas para las calles es muy frecuente" |

### Raymond, Remarques sur la voirie des grandes villes arabes

| Fact | New status | Source locator | Evidence |
|---|---|---|---|
| `streets.cairo.qasaba-width` | verified | paragraph 13 | "large de près de 10 mètres"; "la création fatimide (969)" |
| `streets.cairo.ordinary-streets` | verified | paragraph 8 | "Beaucoup de rues n'avaient donc pas plus de deux à trois mètres de large, et souvent moins." |
| `streets.fez.dead-end-length-share` | verified | paragraph 8 | "les impasses représentent 52, 47 et 41 % de la longueur totale de la voirie" |
| `streets.cairo.dead-end-length-share` | verified | paragraph 8 | same sentence |
| `streets.aleppo.dead-end-length-share` | verified | paragraph 8 | same sentence |

### Bartoli 2019, La lapide murata di San Giovanni Valdarno

| Fact | New status | Source locator | Evidence |
|---|---|---|---|
| `streets.san-giovanni-valdarno.wall-road` | verified | pages 144 and 147 | the plaque "LA VIA DENTRO ALLE MURA LARGA B XX"; "una strada di uniforme larghezza (20 br.)" |
| `walls.san-giovanni.intramural-road` | verified | pages 144 and 147 | same; 20 braccia at 0.5836 m give 11.67 m |

### Encyclopaedia Britannica, 9th edition, Roads and Streets (Wikisource)

| Fact | New status | Source locator | Evidence |
|---|---|---|---|
| `streets.britain.wood-paving-limit` | verified | volume XX | "it may be laid on gradients as steep as 1 in 20" |
| `streets.britain.first-class-road-ruling-gradient` | verified | volume XX | "1 in 30 being generally considered as the ruling gradient to be aimed at on first-class roads" |
| `streets.holyhead-road.ruling-gradient-britannica` | verified | volume XX | "Telford adopted 1 in 30 as the ruling gradient on the Holyhead road"; "only two gradients steeper" |

### Other sources, one or two facts each

| Fact | New status | Source locator | Evidence |
|---|---|---|---|
| `streets.indies-ordinances.streets-from-plaza` | verified | Altamira, ordinance 114, page 263 | four streets from the middle of the sides and "dos calles por cada esquina", 4 + 8 = 12 |
| `streets.valencia.boatella-secondary-streets` | verified | Mateu and others 2025, page 14 | "Tenían un ancho que oscilaba entre los 2,2 y los 3,2 metros" |
| `streets.puebla.streets` | verified | López Guzmán 2005, Internet Archive copy of 2024-12-04 | "Las calles tenían de ancho 14 varas (11'70 metros)." |
| `streets.santiago.streets` | verified | Memoria Chilena | "calles de doce varas de ancho"; Valdivia chose the site on 12 February 1541 |
| `streets.toledo.streets` | verified | Salat, page 1 | "The streets are narrow (2 to 5 meters wide)" |
| `streets.barthelemy-boeing.dead-end-share-range` | verified | Barthelemy and Boeing 2024, Table 1 | dead-end proportion range "2%-39%", median "14%" |
| `streets.pompeii.via-dell-abbondanza` | verified | Madain Project | "approximately 4.5 to 5 meters wide" |
| `streets.pompeii.via-dell-abbondanza-length` | verified | Madain Project | "measuring approximately 900 meters in length" |
| `streets.dorestad.lanes` | verified | The Medieval Review, Hodges | "an orthogonal system of c. 4m wide interspaces", c. 680 to c. 850 |
| `streets.nagoya.back-lanes` | verified | Nagoya City, Shikemichi page | "幅2メートルから幅3メートルの路地" (lanes 2 to 3 metres wide) |
| `streets.samarangana.royal-road-large` | verified | Bhoja, chapter 10, verses 6 to 7, archive.org OCR | 24, 20 and 16 kara for the three city sizes |
| `streets.samarangana.great-street-large` | verified | Bhoja, chapter 10, verse 9, archive.org OCR | two great streets of 12, 10 and 8 kara |
| `streets.holyhead-road.maximum-gradient-coflein` | verified | Coflein 419799 | "Telford's maximum ruling gradient on the Holyhead Road was 1 in 20." |
| `streets.roman.loaded-wagons-usual` | verified | Carreras and others 2019, postprint section 2 | "could seldom deal with gradients larger than 5% (Lawton, 2004)" |
| `streets.paths.pedestrian-desirable-maximum` | verified | Practical Action, Desirable maximum gradient | "the desirable maximum gradient is 12 per cent or about 1 in 8" |
| `streets.bonifacio.aragon-stairs-angle` | verified | French Wikipedia | "selon une inclinaison d'environ 45°", 189 steps, 53 metres |
| `streets.bastides.streets` | verified | English Wikipedia, Bastide | "The streets were usually 6–10 m (20–33 ft) wide" |
| `walls.arthashastra.tower-spacing` | verified | GRETIL, KAZ 2.3.11 | "triṃśad.daṇḍa.antaraṃ" (30 danda apart); the old locator 2.3.10 named the verse before |
| `walls.constantinople.inner-land-wall-tower-spacing` | verified | English Wikipedia, Construction | "the intervals vary between 21 and 77 m"; "most curtain wall sections measure between 40 and 60 meters" |

### Still unverified

| Fact | New status | Source locator | Evidence |
|---|---|---|---|
| `streets.roman.principal-road-average` | unverified | none | Davies's own work was not read; the citing page gives "6.5 metres, 22 pedes ... nearly 500 principal Roman roads" and names no work |
| `streets.al-andalus.footpath` | unverified | none | Isa ibn Musa's own book was not read; Ibn al-Rami paragraph 399, the work the report used, gives seven cubits as Muhammad ibn Talid's answer reported by Isa ibn Musa |
| `streets.al-andalus.side-lane` | unverified | none | same paragraph, four cubits |
| `streets.saltes.lanes` | unverified | none | Bazzana 2006 was not read; López and Gestoso, page 226, the work the report used, give "una anchura de 1´40 a 1´60 m" |
| `streets.london.lanes-1667` | unverified | none | Noorthouck's Appendix XLIX is not on British History Online; the act of 29 April 1667 itself (EarlyPrint A48991) widens passages "not Fourteen Foot broad ... to the Breadth of Fourteen Foot" |
| `streets.monpazier.carreyrous-snippet` | unverified | none | the source has no URL; france-voyage.com returned HTTP 403 and a web search found no width |
| `streets.wolin.street` | unverified | none | the chapter was read in full (pages 45 to 58) and gives no street width; page 54 has "about 1,5-metre-wide jetty" |
| `streets.venice.calli` | unverified | none | the woodcut prints no widths and has no scale, so the report author's reading has no stated figure to check |
| `walls.china-county.gates-4-share` | unverified | none | a count over 1,519 encyclopedia entries; a recount was not done in this batch |

## Points for Jeffrey

- `streets.wolin.street`: the source does not contain the 1.55 m street. The fact stays
  `unverified` because the method has no status for "the source was read and the figure is
  not in it". Options: 1. keep it `unverified` with the note; 2. mark it `superseded` with no
  replacement; 3. remove it. It is one of the facts behind streets question 3, option A.
- `streets.castile-new-towns.main-street`: the 8 varas is the calle Mayor of one town,
  Peñaflor de Hornija. On the same page, Grañón's calle Mayor is 4.95 to 6.19 metres (5 to 7
  varas). The fact's `place` says "Castilian villas nuevas".
- Five facts stay `unverified` although the work the report used gives the figure: the two
  Isa ibn Musa rulings, Saltés, Davies and the London lanes (where the act itself was read).
  This follows batch 1 (Tunis). Options: 1. keep that rule; 2. allow `verified` when the
  intermediate work quotes the original word for word.
- `streets.cairo.ordinary-streets` has `year_to: 1850` with `year_basis: valid_until`;
  Raymond's paragraph gives no 1850. The only date there is 1845.
- `plots.figeac.head-house-width` (not in this batch) comes from the same "6 x 12 m environ"
  and has no `approximate`.
- No recommendation of a group C question changes, because no value changed.

## Checks

`python analysis/validate_facts.py`: "1829 facts, 554 sources, 69 units, 94 quantities:
0 errors". `python -m unittest discover -s analysis/tests`: 56 tests, OK.
