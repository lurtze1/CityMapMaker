# Cross-analysis: landmark footprints and open spaces

Topic file: `data/research/facts/footprints.yaml` (245 facts). Sources added:
`data/research/sources/footprints.yaml`. New quantities: `footprint_length`, `footprint_width`,
`precinct_area`, `open_space_length`, `open_space_width`. New unit: `ft`.

This document turns research into decisions, so it follows the CLAUDE.md rule "Explicit over
implied" in full (ADR-023). Where a figure below is approximate, the approximation is the
source's, and the fact's note says so. Every ratio or area marked "(arithmetic here)" was
computed in this document from fact values; it is not a fact and has no fact id.

Fact ids below drop the `footprints.` prefix: `cologne-cathedral.area` means
`footprints.cologne-cathedral.area`.

## 1. Scope

Covered: the ground area, length, width and length-to-width ratio (aspect) of landmark
buildings and compounds, and the size of market squares, plazas, greens and parade grounds,
as far as the reports below give them.

Reports read: `settlement/landmark-footprints.md`, `settlement/verification-footprints.md`,
`settlement/district-types.md`, `settlement/water-infrastructure.md`,
`settlement/table-use.md`, `city-foundation-and-growth.md`, the seven
`sources-by-language/*.md` reports and their README, `town-plan-corpus.md` and
`town-plan-corpus-world.md`. `table-use.md`, `water-infrastructure.md` and the
sources-by-language README hold no footprint or open-space size; `water-infrastructure.md`
repeats the Venetian Arsenal's 45 ha, recorded once under `venetian-arsenal.precinct`.

Not covered: plot, block and street sizes (other topics), town and wall-circuit areas
(other topics), building heights.

Two rules decide the quantity of each fact:

- `footprint_area`: one continuous built outline, its own courtyards included (a caravanserai,
  a madrasa, a mosque with its courtyard).
- `precinct_area`: a walled or fenced compound with several separate buildings and open
  ground (an abbey, a temple enclosure, a palace compound, a castle ward, an arsenal).

Claude applied these two rules when filling the facts; Jeffrey decides any change to them
when reviewing this PR.

Counts after the repair round of 2026-10-21: 245 facts, 137 source entries in
`sources/footprints.yaml`. Status counts: 41 verified, 4 corrected, 1 superseded,
18 disputed, 181 unverified. Only `verification-footprints.md` verifies; every figure from
the other reports is unverified. 76 facts are approximate, 19 facts carry `disputes`
(10 pairs, section 3), 3 facts carry `source_via`.

### Repair round 2026-10-21

A sample check found errors; every fact was checked again against the report it cites, and
the fields of fact format version 2 were filled.

- `approximate: true` set on 76 facts whose report gives the figure with "about", "c.",
  "~", "nearly" or "around", or whose source marks it approximate (`cologne-cathedral.area`).
- Years removed from 12 facts whose report gives no date for the figure: `harlech.*`,
  `conwy.*` (3), `hereford-market-hall.*` (2), `slipway.one-ship-area`, `qufu-confucius.*`
  (2, the report says "Ming-Qing" only), `tenochtitlan-ceremonial.side`,
  `great-zimbabwe.great-enclosure-wall`, `old-oyo-palace.precinct` ("to c. 1837" is the end
  of the city).
- Years corrected on 52 facts: centuries now span their full range (13th century =
  1201–1300), "15th c. on" at Agadez is 1401–1500, St Mary Redcliffe is 1201–1500 with era
  `medieval`, Great Zimbabwe is 1001–1500 (11th–15th century), Monnow Bridge is c. 1272 (the
  bridge, not the gate), Dejima is 1636, Miletus and Nagoya keep only their start year.
- `year_basis` set on all 211 facts that keep years: `built` (144), `plan_date` (23),
  `described` (16), `depicted` (15), `founded` (12), `valid_until` (1). Four rules apply. A
  town-plan corpus figure takes the plan's date as `plan_date` when the
  section is dated by its plan, and `depicted` when the section is dated by the period the
  plan depicts (Tiryns, Dholavira, Trelleborg, Srirangam, Kahun, Heijo, Cahokia); the note
  says so. The Tang Chang'an markets (7th-9th c.) are `depicted` as in the streets topic. `valid_until` is a figure valid until the place was abandoned, with only `year_to`:
  `old-oyo-palace.precinct`, Old Oyo "to c. 1837". A fact from `verification-footprints.md` keeps the construction dates of the same
  building in `landmark-footprints.md`, the report it verifies, and the note says so
  (19 facts). The Gérard bastide years are `founded`; the 1573 Ordinances, Tenochtitlan 1519,
  Ishibashi-juku, Cairo's old tannery and Djenné are `described`.
- `year_approximate: true` set on 40 facts ("c. 1078", "c. 1600s", "early 15th century").
- `disputes` set on 19 facts in 10 mutual pairs (section 3).
- `source_via` set on 3 facts: `heywood-castle.bailey-area` (Historic England 1020959 via
  the Forestry England PDF), `new-inn-gloucester.chamber-area` (Historic England listing via
  British Listed Buildings), `nagoya.kaishochi-side-library` (名古屋城史 via the Nagoya City
  Library report). Three source entries were added for the originals.
- Status changed on 4 facts: `borg-longhouse.aspect` and `laws-of-indies.plaza-min-width-ft`
  from disputed to unverified (no second figure for the same quantity exists);
  `laws-of-indies.plaza-min-length` and `laws-of-indies.plaza-max-length` from unverified to
  disputed (they conflict with the ft facts).
- Facts whose source is a corpus report itself (`citymapmaker-town-plans-2026`,
  `citymapmaker-town-plans-world-2026`, for example Cahokia and Shuri) stay; the corpus
  gives no citation for them, and each note now says so.
- No fact in this topic came from the time-series report, so none was removed.

### Verification batch 1 (2026-10-21)

Issue #74, batch 1, checked 51 unverified facts that questions 1 to 8 rest on against their
original sources: 32 verified, 3 corrected, 0 disputed, 16 not verified because the source
was not reachable or the figure was not found (the nine Gérard bastide squares, the two
Aleppo souq figures, the two Conwy figures, Speyer Altpörtel, Dunster Yarn Market and Dupath
well). Changed values:

- `colchester-well.area`: 3.4 m² becomes 3.6 m². The record gives 1.55 m inside with a 0.30 m
  lining, so 2.15 m across outside, not 2.1 m. The measured wells now span 2.3–3.6 m²
  (question 5 and the tiny row of §5.2 updated). All wells stay under the 9 m² floor, so the
  recommendation of question 5 does not change.
- `kandariya-mahadeva.area`: the area stays 620 m²; the years change from c. 1030 to c.
  1025–1050, as the article gives them. No recommendation changes.
- `hen-domen.site-area`: the area stays 1,571 m²; the year changes from c. 1070 to 1071–1100
  ("soon after 1071", "completed before 1100"). No recommendation changes.

Six cited facts take their source from a CityMapMaker report (`kings-head-aylesbury.*`,
`zamosc.*`, `neuf-brisach.*`, `savannah.*`, `bath.*`) and two are cited as not used
(`laws-of-indies.plaza-good-width`, `laws-of-indies.plaza-max-width`); they were not checked
in this batch.

## 2. The picture

Class names follow spec-v1 §3: tiny 9–80 m², small 80–300, medium 300–1,200, large
1,200–5,000, huge 5,000–20,000, precinct 20,000 and over (precinct is not a version 1 class).
The ranges are half-open as spec-v1 §3 defines them: a class includes its lower bound and
excludes its upper bound, so 5,000 m² is huge. spec-v1 §3 owns these ranges and the
half-open convention; Jeffrey decides any change to them, in spec-v1.

### 2.1 Buildings, by type

| Type | Area range (m²) | Class(es) | Aspect (long : short) | Fact ids |
|---|---|---|---|---|
| Cathedral | 5,986 (bbox) – 8,333 (bbox) | huge | Chartres 2.8 at the 46 m width; Cologne 144.58 × 86.25 = 1.7, Chartres 130.2 × 64 = 2.0, Hagia Sophia 82 × 73 = 1.1 (arithmetic here) | `cologne-cathedral.area`, `cologne-cathedral.length`, `cologne-cathedral.width`, `chartres-cathedral.bbox-area`, `chartres-cathedral.length`, `chartres-cathedral.width-transept`, `chartres-cathedral.width-wikipedia`, `chartres-cathedral.aspect`, `hagia-sophia.bbox-area`, `hagia-sophia.length`, `hagia-sophia.width`, `salisbury-cathedral.length` |
| Large parish or monastery church | 1,550 – 2,200 | large | 2.1 (St Mary Redcliffe), 3.8 (Saint Gall, nave without transept) | `st-mary-redcliffe.area`, `st-mary-redcliffe.area-estimate`, `st-mary-redcliffe.aspect`, `st-mary-redcliffe.length`, `st-mary-redcliffe.width`, `saint-gall-plan.church-area`, `saint-gall-plan.church-aspect`, `saint-gall-plan.church-length`, `saint-gall-plan.church-width`, `hull-minster.length`, `fountains-abbey.church-length` |
| Small parish church, chapel | 68 (internal) – 98 | tiny, small | Escomb nave 13.3 × 4.4 internal; chapel 14 × 7 = 2.0 (arithmetic here) | `escomb-church.area-internal`, `escomb-church.length-internal`, `fountains-abbey.infirmary-chapel-area` |
| Friday mosque (prayer hall + courtyard) | 5,600 (Agadez, enclosure) – 26,318 | huge, precinct | Kairouan 125 × 75 = 1.7, Córdoba 175 × 128 = 1.4 (arithmetic here) | `kairouan-mosque.area`, `kairouan-mosque.length`, `kairouan-mosque.width`, `cordoba-mosque.area`, `cordoba-mosque.length`, `cordoba-mosque.width`, `ibn-tulun-mosque.area`, `agadez-great-mosque.area` |
| Mosque prayer hall alone | 2,648 – 14,720 | large, huge | Kairouan 70.6 × 37.5 = 1.9 (arithmetic here) | `kairouan-mosque.prayer-hall-area`, `cordoba-mosque.prayer-hall-area` |
| Neighbourhood mosque | 64 | tiny | 8 × 8 = 1.0 | `cristo-de-la-luz.area` |
| Minaret | 114 | small | 1.0 | `kairouan-mosque.minaret-area` |
| Hindu temple (building) | 620 – 911 | medium | Kandariya 31 × 20 = 1.6 (arithmetic here); vimana 1.0 | `kandariya-mahadeva.area`, `brihadisvara.vimana-area` |
| Buddhist hall | 281 – 2,850 | small, large | Daibutsuden 57 × 50 = 1.1; Kondō 18.5 × 15.2 = 1.2 (arithmetic here) | `horyuji-kondo.area`, `todaiji-daibutsuden.area`, `todaiji-daibutsuden.length` |
| Chinese temple hall | 1,018 – 1,836 | medium, large | Dacheng Hall 54 × 34 = 1.6 (arithmetic here); Hall of Prayer round | `qufu-dacheng-hall.area`, `temple-of-heaven.hall-area` |
| Shinto shrine | 60 | tiny | 2 | `ise-naiku.area`, `ise-naiku.aspect` |
| Tower keep | 441 – 1,152 | medium (White Tower 4% under large) | Rochester 1.0; White Tower 36 × 32 = 1.1 (arithmetic here) | `rochester-keep.area`, `white-tower.area` |
| Motte (earthwork) | 707 – 6,362 | medium to huge | round | `motte.area`, `motte.diameter` |
| Castle, overall outline | 1,571 – 5,000 | large (Conwy at the huge line) | Hen Domen 50 × 40 = 1.25; Conwy 125 × 40 = 3.1 (arithmetic here) | `hen-domen.site-area`, `conwy.overall-area`, `conwy.overall-length` |
| Castle ward or courtyard only | 1,300 – 3,000 | large | n/a | `hen-domen.bailey-area`, `heywood-castle.bailey-area`, `beaumaris.inner-ward-area`, `harlech.inner-ward-area`, `conwy.courtyards-area`, `beaumaris.ward-gap` |
| Bell tower, watchtower | 113 – 188 | small | 1.0 or round | `st-marks-campanile.area`, `pisa-tower.area`, `corsican-towers.area`, `martello-st-helena.area`, `pisa-tower.foundation-area` |
| Town gate | 30 (internal room) – 113 | tiny, small | Loughrea 7.7 × 6 = 1.3, Speyer 13 × 8.7 = 1.5 (arithmetic here) | `loughrea-gate.area`, `lipiany-gate.area`, `speyer-altportel.area`, `monnow-gate.tower-room-area` |
| Palace (compound in a town) | 11,000 – 12,250 | huge | Fengshan 175 × 70 = 2.5 (arithmetic here) | `palais-des-papes.site-area`, `agadez-sultan-palace.precinct`, `fengshan-yamen.length`, `fengshan-yamen.width`, `fengshan-jail-office.length` (an office inside the yamen, 35 × 35 m) |
| Royal or great hall | 528 – 1,547 | medium, large | 3.7 (Westminster), 4.4 (Lejre) | `westminster-hall.area`, `westminster-hall.length`, `westminster-hall.aspect`, `lejre-hall.area`, `lejre-hall.aspect`, `salone-dei-cinquecento.area` |
| Longhouse | 789 – 996 | medium | 8.7 at 9.5 m width; 83 × 12 = 6.9 at 12 m (arithmetic here) | `borg-longhouse.length`, `borg-longhouse.width-wikipedia`, `borg-longhouse.width-museum`, `borg-longhouse.area-wikipedia`, `borg-longhouse.area-museum`, `borg-longhouse.aspect`, `trelleborg.longhouse-length` |
| Guildhall | 299 – 339 | small/medium line | 26.5 × 11.3–12.8 = 2.1–2.3 (arithmetic here); 2.3 in the first report | `merchant-adventurers-hall.area`, `merchant-adventurers-hall.length`, `merchant-adventurers-hall.width`, `merchant-adventurers-hall.aspect` |
| Market hall, market house | 70 – 265 | tiny, small | Hereford 25.6 × 10.4 = 2.5, Yeovil 21.3 × 6 = 3.6 (arithmetic here); Dunster octagon | `hereford-market-hall.area`, `hereford-market-hall.length`, `yeovil-market-house.area`, `yeovil-market-house.length`, `dunster-yarn-market.area` |
| Cloth hall, souq core | Ypres 125 m frontage; Aleppo souq 250 × 40 | huge (souq 10,000 m², arithmetic here) | souq 6.3 (arithmetic here) | `ypres-cloth-hall.frontage`, `aleppo-souq-core.length`, `aleppo-souq-core.width` |
| Inn (European, Japanese) | 390 – 490; European inn yard 280 | medium (Japanese inns, 390–490), small (European inn yard, 280) | European inn 30 m across (the report's own estimate, "about 30 m") | `ishibashi-juku.honjin-area`, `ishibashi-juku.waki-honjin-area`, `kings-head-aylesbury.courtyard-area`, `kings-head-aylesbury.estimated-side`, `new-inn-gloucester.chamber-area` |
| Caravanserai, urban khan | 2,500 – 4,900 | large | Sultan Han courtyard 58 × 44 = 1.3 (arithmetic here) | `sultan-han.area`, `sultan-han.courtyard-area`, `khan-asad-pasha.area` |
| Madrasa, college | 4,536 (madrasa); 6,408 (Tom Quad, quad only) | large, huge | 1.4 | `ulugh-beg-madrasa.area`, `ulugh-beg-madrasa.aspect`, `ulugh-beg-madrasa.courtyard-length`, `tom-quad.area` |
| Timber playhouse | 378 – 718 | medium | round, 14- or 20-sided | `globe.area`, `globe.diameter`, `rose.area`, `rose.yard-area` |
| Roman theatre | 4,838 – 10,000 | large, huge | half-circle (2 : 1) | `theatre-of-marcellus.area`, `theatre-of-marcellus.diameter`, `theatre-of-pompey.area`, `theatre-of-orange.stage-wall` |
| Opera house | 11,237 – 12,000 | huge | 154.9 × 70.2 = 2.2 or 173 × 125 = 1.4 (arithmetic here) | `palais-garnier.area`, `palais-garnier.length-wikipedia`, `palais-garnier.length-franceguide` |
| Infirmary (health) | 1,092 | medium | 52 × 21 = 2.5 (arithmetic here) | `fountains-abbey.infirmary-hall-area`, `fountains-abbey.infirmary-hall-length` |
| Ropewalk (arsenal building) | 6,636 – 6,657 | huge | 15 | `corderie.area`, `corderie.length`, `corderie.width`, `corderie.aspect` |
| Slipway, ship | 150 – 360 | small, medium | 2–5 | `small-hythe.slipway-area`, `slipway.one-ship-area`, `skuldelev-2.length`, `sutton-hoo-ship.length` |
| Well, well-house | 2.3 – 14 | below tiny (3 of 4), tiny | 1.0 or round | `colchester-well.area`, `ipswich-well.area`, `nine-wells-st-davids.area`, `dupath-well.area` |
| Monumental fountain, stepwell | Trevi 49.15 m wide; stepwell 59.7 × 12.2 | stepwell 728 m² medium (arithmetic here) | stepwell 4.9 (arithmetic here) | `trevi-fountain.width`, `dada-harir-stepwell.length`, `dada-harir-stepwell.width` |
| Column, market cross | 8.7 – 30 | tiny (Shillingstone 0.3 m² under tiny) | 1.0 | `trajans-column.shaft-area`, `nelsons-column.pedestal-area`, `shillingstone-cross.area`, `brigstock-cross.area` |
| Town house (for scale) | 36 | tiny | n/a | `dublin-viking-house.area` |

### 2.2 Compounds and precincts (20,000 m² and over, or walled with several buildings)

| Type | Size | Fact ids |
|---|---|---|
| Hindu temple compound | 2.9 ha (Thanjavur, aspect 2) – 6 ha (Madurai); temple town 63 ha (Srirangam) | `brihadisvara.precinct`, `brihadisvara.precinct-length`, `brihadisvara.precinct-aspect`, `meenakshi.precinct`, `meenakshi.court-side`, `meenakshi.enclosure-corpus`, `srirangam.precinct`, `srirangam.outer-length` |
| Chinese temple compound | 1.6 ha (Qufu, stated) – 267–273 ha (Temple of Heaven) | `qufu-confucius.precinct`, `qufu-confucius.axis-length`, `temple-of-heaven.precinct` |
| Abbey, monastery | 28 ha (Fountains); under 0.5 ha (Skellig Michael terrace 60 × 30 m) | `fountains-abbey.precinct`, `skellig-michael.terrace-length`, `skellig-michael.beehive-cell` |
| Sacred or ceremonial precinct | 14 ha (Haram al-Sharif), 4 ha (Salt Lake temple block), 25 ha (Tenochtitlan, 500 m per side, arithmetic here), 80 ha (Cahokia) | `jerusalem.haram-precinct`, `salt-lake-city.temple-block`, `tenochtitlan-ceremonial.side`, `cahokia.central-precinct`, `cahokia.monks-mound-length`, `cahokia.monks-mound-width` |
| Castle and cathedral precinct, citadel | 4 ha (Shuri) – 10 ha (Durham plateau) – 28 ha (Moscow Kremlin) – 100 ha (Hanoi) – 300 ha (Edo castle) | `shuri-castle.precinct`, `durham.plateau-precinct`, `moscow-kremlin.precinct`, `hanoi-citadel.precinct`, `edo-castle.precinct`, `tiryns.citadel-length`, `tiryns.citadel-width`, `ostia.castrum-length`, `trelleborg.ring-diameter`, `fort-st-george.inner-fort-length` |
| Palace compound or palace city | 1.2 ha (Agadez) – 2.4 ha (Moctezuma) – 33 ha (Kano) – 40 ha (Gyeongbokgung) – 72 ha (Forbidden City) – 120 ha (Heijo) – 200 ha (Old Oyo) | `agadez-sultan-palace.precinct`, `tenochtitlan-moctezuma-palace.precinct`, `kano-gidan-rumfa.precinct`, `gyeongbokgung.precinct`, `beijing-forbidden-city.precinct`, `heijo-palace.precinct`, `old-oyo-palace.precinct`, `hue-imperial-city.side`, `gondar-fasil-ghebbi.wall-length` |
| Arsenal (walled shipyard) | 24 ha (disputed) or 45–48 ha | `venetian-arsenal.precinct`, `venetian-arsenal.precinct-corpus`, `venetian-arsenal.rampart` |
| Works yard (tannery) | 0.68 ha (Aleppo) – 4 ha (Cairo) | `aleppo-tannery.area`, `cairo-old-tannery.area` |
| Enclave, elite enclosure | 1.3 ha or 120 × 75 m (Dejima), 3 ha (Tojin yashiki), 100 m across (Great Zimbabwe), 1.2 ha single building (Pueblo Bonito), 2,400 m² mansion (Kahun), 40–60 m tulou | `dejima.length`, `dejima.width`, `dejima.area-corpus`, `nagasaki.tojin-yashiki`, `great-zimbabwe.great-enclosure-across`, `great-zimbabwe.great-enclosure-wall`, `pueblo-bonito.area`, `kahun.mansion-area`, `tianluokeng.tulou-diameter` |

### 2.3 Open spaces, by spec kind

Areas in this table are arithmetic here unless a fact gives the area: length × width for a
rectangle, π × (diameter ÷ 2)² for a round space (`vila-real.praca-diameter`,
`bath.circus-diameter`). Jeffrey decides changes to these formulas when reviewing this
report.

| Spec kind | Example | Length × width (m) | Area (m²) | Aspect | Fact ids |
|---|---|---|---|---|---|
| market_square | early bastides | 50–55 square | 2,500–3,025 | 1 | `bastides.early-square-side` |
| market_square | Monflanquin, Cologne-du-Gers, Grenade, Revel | 60, 70, 75, 90 square | 3,600–8,100 | 1 | `monflanquin.square-side`, `cologne-du-gers.square-side`, `grenade.square-side`, `revel.square-side` |
| market_square | Marciac, Trie, Rabastens | 140 × 70, 130 × 75, 150 × 75 | 9,750–11,250 | 1.7–2.0 | `marciac.square-length`, `marciac.square-width`, `trie.square-length`, `trie.square-width`, `rabastens.square-length`, `rabastens.square-width` |
| market_square | Zamość Great Market | 100 × 100 | 10,000 | 1 | `zamosc.great-market-side` |
| market_square | Djenné market esplanade | 400 × 150 | 60,000 | 2.7 | `djenne.esplanade-length`, `djenne.esplanade-width` |
| market_square | Isfahan Naqsh-e Jahan (maidan) | 560 × 160 | 89,600 (8.96 ha stated) | 3.5 | `isfahan-naqsh-e-jahan.length`, `isfahan-naqsh-e-jahan.width`, `isfahan-naqsh-e-jahan.area` |
| market_square (walled ward) | Chang'an East and West Market | 1,025 × 927 | 950,000 | 1.1 | `changan.market-length`, `changan.market-width` |
| plaza | 1573 Ordinances, minimum | 83.6 long; width torn in the original | n/a | at least 1.5 | `laws-of-indies.plaza-min-length`, `laws-of-indies.plaza-min-aspect`, `laws-of-indies.plaza-min-ft`, `laws-of-indies.plaza-min-width-ft` |
| plaza | 1573 Ordinances, "good proportion" | 167.2 × 111.4 | 18,600 | 1.5 | `laws-of-indies.plaza-good-length`, `laws-of-indies.plaza-good-width` |
| plaza | 1573 Ordinances, maximum | 222.9 × 147.7 | 32,900 | 1.5 | `laws-of-indies.plaza-max-length`, `laws-of-indies.plaza-max-width`, `laws-of-indies.plaza-max-ft` |
| plaza | Mexico City plaza mayor | n/a | 20,713.4 | n/a | `mexico-city.main-plaza-area` |
| plaza | Vila Real de Santo António | 66 diameter | 3,400 | round | `vila-real.praca-diameter` |
| plaza | Miletus South Agora | 196 × 164 | 32,100 | 1.2 | `miletus.south-agora-length`, `miletus.south-agora-width` |
| plaza | Huánuco Pampa, Cahokia Grand Plaza | 550 × 350; 19 ha | 192,500; 190,000 | 1.6 | `huanuco-pampa.plaza-length`, `huanuco-pampa.plaza-width`, `cahokia.grand-plaza-area` |
| green | Savannah ward square | 96 × 82 | 7,900 | 1.2 | `savannah.ward-square-length`, `savannah.ward-square-width` |
| green | Bath Circus | 97 diameter | 7,400 | round | `bath.circus-diameter` |
| green (block-centre common ground) | Edo, Nagoya kaishochi | 20 or 36–39 square | 400–1,500 | 1 | `edo.kaishochi-side`, `nagoya.kaishochi-side-abe`, `nagoya.kaishochi-side-library` |
| parade_ground | Neuf-Brisach Place d'Armes | 90 × 90 | 8,100 | 1 | `neuf-brisach.place-darmes-side` |
| parade_ground | Dholavira ceremonial ground | 283 × 47 | 13,300 | 6.0 | `dholavira.ceremonial-ground-length`, `dholavira.ceremonial-ground-width` |
| courtyard (not a spec kind) | Córdoba sahn, Sultan Han, King's Head, Rose yard, Tom Quad | n/a | 163–6,408 | n/a | `cordoba-mosque.courtyard-area`, `sultan-han.courtyard-area`, `kings-head-aylesbury.courtyard-area`, `rose.yard-area`, `tom-quad.area` |

Spacing, not size: Djenné has a small square, school or cemetery every 50–100 m of street
(`djenne.small-square-spacing`).

## 3. Conflicts

| # | Competing facts | What differs | Reason | Status |
|---|---|---|---|---|
| 1 | `venetian-arsenal.precinct` (45 ha) vs `venetian-arsenal.precinct-corpus` (24 ha) | Arsenal area, a factor of 1.9 | The corpus gives "c. 24 ha" without a source; Wikipedia's 45 ha is confirmed by italia.it (48 ha). The corpus depicts 1500; whether 24 ha is the pre-extension arsenal is not stated in any report. | 45 ha verified; 24 ha disputed |
| 2 | `chartres-cathedral.area-reported` (10,875 m²) vs `chartres-cathedral.bbox-area` (8,333 m²) | Area | 10,875 m² exceeds the bounding box of the official dimensions, so it is impossible as a footprint. The real footprint is below 8,333 m²; its value is unknown. | 10,875 superseded; 8,333 corrected (upper bound) |
| 3 | `laws-of-indies.plaza-min-ft` / `-max-ft` / `-min-width-ft` (city-foundation, "ft") vs `laws-of-indies.plaza-min-length` / `-max-length` (iberian, pies) | Unit and the minimum width | city-foundation-and-growth.md reads an English translation that renders *pies* as feet and gives a minimum of 200 × 300; iberian.md reads the Spanish original, where the minimum width is torn. Converted at 0.3048 m, 800 ft is 243.8 m; the iberian report converts 800 pies to 222.9 m (arithmetic here for the ft figure). | pairs `plaza-min-ft` ↔ `plaza-min-length` and `plaza-max-ft` ↔ `plaza-max-length` disputed; `plaza-min-width-ft` unverified, because iberian.md gives no width value to compete with it (repair round 2026-10-21) |
| 4 | `borg-longhouse.width-wikipedia` (9.5 m) vs `borg-longhouse.width-museum` (12 m); areas 789 vs 996 m² | Width, so area and aspect (8.7 vs 6.9) | The museum gives its reconstruction; the excavated width was not readable. | the two width facts and the two area facts disputed in pairs; `borg-longhouse.aspect` unverified, because no second source gives an aspect (repair round 2026-10-21) |
| 5 | `st-mary-redcliffe.area` (1,916 m²) vs `st-mary-redcliffe.area-estimate` (1,550 m²) | Area | 1,916 has no second source; 1,550 is the verification report's rough cross-shape estimate that leaves out porch, tower and chapels. Both are large class. | both disputed |
| 6 | `dejima.length` / `dejima.width` (120 × 75 m, 0.9 ha arithmetic here) vs `dejima.area-corpus` (1.3 ha, fan-shaped) | Area | A fan shape inside a 120 × 75 m box cannot reach 1.3 ha; one of the two figures is wrong or measures something else. | all three disputed |
| 7 | `nagoya.kaishochi-side-abe` (10 ken, 20 m) vs `nagoya.kaishochi-side-library` (20 ken, 36–39 m) | Side of the block-centre ground | Two Japanese sources disagree; the japanese.md report flags it. | both disputed |
| 8 | `palais-garnier.length-wikipedia` (154.9 × 70.2 m) vs `palais-garnier.length-franceguide` (173 × 125 m) | Outline | The areas agree within 7% (12,000 vs 11,237 m²); the verification report does not know whether the larger outline includes ramps and terraces. | lengths disputed; area verified |
| 9 | `merchant-adventurers-hall.area` 299–339 m² (RCHME) vs first report 390 m² | Area | The first figure came from a local guide; the Royal Commission survey measures 87 × 37–42 ft. Moves the hall onto the small/medium line. | corrected |
| 10 | `cordoba-mosque.area` (whole compound, 22,400–23,400 m²) vs `cordoba-mosque.prayer-hall-area` (roofed hall, 14,200–14,720 m²) | Which outline is "the building" | Not a disagreement of sources but of definition: the compound is a precinct, the roofed hall alone is huge. | compound verified; hall unverified; the choice is Jeffrey's (question 1) |
| 11 | `kairouan-mosque.length` (125 m, MWNF) vs `kairouan-mosque.length-unesco` (135 m); prayer hall 70.6 m (Wikipedia) vs 137 m (MWNF, same page as the 125 m) | Outline and prayer-hall length | The MWNF page contradicts itself (a prayer hall longer than the building); UNESCO was seen only in a search summary. | 125 m verified against the area; 135 m unverified |
| 12 | `qufu-confucius.precinct` (16,000 m²) vs `qufu-confucius.axis-length` (1,300 m) | Internal consistency | A 1.3 km axis needs a strip only 12 m wide to cover 16,000 m² (arithmetic here); the report itself says "treat with care". | both unverified |
| 13 | `temple-of-heaven.precinct` 267 ha vs 2.73 km² | Area, 2% apart | The same page gives both. Recorded as a range 267–273 ha. | unverified |
| 14 | `meenakshi.precinct` (14 acres) vs other sites (15, 17 or 45 acres, search summaries only) | Area | 14 acres agrees with Wikipedia's 5.7 ha and the corpus's 6 ha; the 45-acre figure is the outlier. | verified |

## 4. Gaps

No fact in any report gives:

1. The footprint of any town hall (only Hereford's market hall, which served as the town hall).
2. Any hospital, bathhouse, hammam or leper house footprint. Health rests on one
   infirmary hall (`fountains-abbey.infirmary-hall-area`).
3. Any barracks footprint. Military rests on one ropewalk and one arsenal precinct.
4. Any warehouse, exchange or customs house footprint.
5. Any mill or lighthouse footprint (both are utility rows in the spec, and the example spec
   has a lighthouse).
6. Any pagoda base (`pagoda` is a default-pack label) or small Hindu shrine.
7. Any arena or amphitheatre (entertainment row).
8. The outer outline of a caravanserai with its walls (Sultan Han's 4,500–4,900 m² is an area
   without length and width).
9. The overall outline of an Edwardian stone castle measured from a fetched page (Conwy's
   125 × 40 m is a search summary).
10. A complete medieval coaching-inn outline; the European inn size is the report's own
    30 m estimate (`kings-head-aylesbury.estimated-side`).
11. Sizes of English or German medieval market places (wedge-shaped and street markets are
    described in city-foundation-and-growth.md and continental.md without sizes), of village
    greens, and of commons.
12. More than one parade ground per culture: Neuf-Brisach is the only European one.
13. A Chinese open square: Chinese markets in the reports are walled wards (`changan.market-*`).
14. Depths: Ypres Cloth Hall, Trevi Fountain; widths: Salisbury Cathedral, Hull Minster.
15. Whether the Ishibashi-juku honjin areas (`ishibashi-juku.*`) are plot or building areas.

## 5. Check of the spec tables (spec-v1 §3)

Source: `docs/proposals/spec-v1.md` on branch `m1/spec-proposal`, §3.

### 5.1 Function → default class and default shape

| Function | Spec default | Facts support | Facts contradict | Verdict |
|---|---|---|---|---|
| religious (church, temple, mosque, shrine) | large, 2.5 : 1 | Class: large parish and monastery churches 1,550–2,200 (`st-mary-redcliffe.area`, `st-mary-redcliffe.area-estimate`, `saint-gall-plan.church-area`); Daibutsuden 2,850 (`todaiji-daibutsuden.area`); Dacheng Hall 1,836 (`qufu-dacheng-hall.area`). Shape: churches 2.1–3.8 (`st-mary-redcliffe.aspect`, `chartres-cathedral.aspect`, `saint-gall-plan.church-aspect`). | Class: cathedrals and Friday mosques are huge or bigger (`cologne-cathedral.area`, `chartres-cathedral.bbox-area`, `kairouan-mosque.area`, `cordoba-mosque.area`); chapel, village church, neighbourhood mosque and shrine are tiny or small (`fountains-abbey.infirmary-chapel-area`, `escomb-church.area-internal`, `cristo-de-la-luz.area`, `ise-naiku.area`); Hindu temples and small Buddhist halls are medium or small (`kandariya-mahadeva.area`, `horyuji-kondo.area`). Shape: mosques, Buddhist halls and Hindu temples measure 1.1–1.7 (section 2.1). | Class holds for a large parish church only. Two of the four examples in the row (mosque, shrine) have no measured example in the large class; temple is mixed: Daibutsuden 2,850 and Dacheng Hall 1,836 are large, Hindu temples 620–911 are medium and Hōryū-ji Kondō 281 is small. Shape 2.5 : 1 holds for Christian churches only. |
| defence (castle, keep, tower, gatehouse) | large, 1.3 : 1 | Class: a motte-and-bailey castle 1,571 (`hen-domen.site-area`); castle wards 1,830–3,000 (`harlech.inner-ward-area`, `beaumaris.inner-ward-area`). Shape: Hen Domen 1.25. | Class: keeps are medium (`rochester-keep.area`, `white-tower.area`); towers are small (`st-marks-campanile.area`, `corsican-towers.area`); gatehouses are tiny or small (`loughrea-gate.area`, `lipiany-gate.area`, `speyer-altportel.area`); a stone castle with wards is at or over 5,000 (`conwy.overall-area`, `beaumaris.ward-gap`). Shape: keeps and towers are 1 : 1; Conwy is 3.1. | Class holds for a small castle only; three of the four examples are one to three classes smaller. |
| seat_of_power (palace, government house) | huge, 1.3 : 1 | Class: 11,000–12,250 (`palais-des-papes.site-area`, `agadez-sultan-palace.precinct`, `fengshan-yamen.length`, `fengshan-yamen.width`). Shape: Forbidden City 960 × 750 = 1.3 (arithmetic here). | Class: palace cities of 40–200 ha are precincts (`beijing-forbidden-city.precinct`, `heijo-palace.precinct`, `old-oyo-palace.precinct`); a royal hall alone is large (`westminster-hall.area`). Shape: Fengshan yamen 2.5. | Supported for a palace inside a town. |
| civic (town hall, guildhall, courthouse) | medium, 2 : 1 | Class: guildhall 299–339 (`merchant-adventurers-hall.area`), on the small/medium line. Shape: 2.1–2.5 (`merchant-adventurers-hall.length`, `merchant-adventurers-hall.width`, `hereford-market-hall.length`). | Class: Hereford's market hall that served as town hall is small, 265 (`hereford-market-hall.area`). | Shape supported; class rests on one example at the small/medium line. No town hall figure exists (gap 1). |
| commerce (market hall, warehouse, exchange) | medium, 2 : 1 | Shape: Hereford 2.5. | Class: every measured small market hall is tiny or small, 70–265 (`dunster-yarn-market.area`, `yeovil-market-house.area`, `hereford-market-hall.area`); the big ones are huge (`aleppo-souq-core.length`, `ypres-cloth-hall.frontage`). Shape: Yeovil 3.6; Dunster octagonal. | Class contradicted: no measured market building is medium. Warehouse and exchange have no data (gap 4). |
| lodging (inn, caravanserai) | medium, 1.2 : 1 | Class: Japanese official inns 390–490 (`ishibashi-juku.honjin-area`, `ishibashi-juku.waki-honjin-area`); a European inn yard of 280 plus ranges, 30 m across by the report's estimate (`kings-head-aylesbury.courtyard-area`, `kings-head-aylesbury.estimated-side`). Shape: Sultan Han courtyard 1.3. | Class: caravanserais and khans are large, 2,500–4,900 (`sultan-han.area`, `khan-asad-pasha.area`). | Supported for an inn; contradicted for a caravanserai. |
| learning (academy, madrasa, college) | large, 1.4 : 1 | Class and shape: Ulugh Beg 4,536, 1.4 (`ulugh-beg-madrasa.area`, `ulugh-beg-madrasa.aspect`). | Class: Tom Quad alone is 6,408 (`tom-quad.area`). | Supported for a madrasa; one example, 9% under the large/huge line. |
| entertainment (theatre, arena) | medium, 1 : 1 | Class and shape: timber playhouses 378–718, round (`globe.area`, `rose.area`). | Class: Roman theatres 4,838–10,000 and an opera house 11,237–12,000 (`theatre-of-marcellus.area`, `theatre-of-pompey.area`, `palais-garnier.area`). Shape: a Roman theatre is a half-circle, 2 : 1. | Supported for a timber playhouse. Arena has no data (gap 7). |
| health (hospital, bathhouse, leper house) | medium, 2 : 1 | Class and shape: one infirmary hall, 1,092, 52 × 21 m (`fountains-abbey.infirmary-hall-area`, `fountains-abbey.infirmary-hall-length`). | none | One data point; no hospital or bathhouse figure exists (gap 2). |
| utility (well, fountain, mill, lighthouse) | tiny, 1 : 1 | Class: well-houses 2.9–14 (`dupath-well.area`, `nine-wells-st-davids.area`). Shape: wells are round or square. | Class: three of four wells are under the tiny lower bound of 9 m² (`colchester-well.area`, `ipswich-well.area`, `nine-wells-st-davids.area`); a monumental fountain is 49 m wide (`trevi-fountain.width`); a stepwell is medium, 4.9 : 1 (`dada-harir-stepwell.length`, `dada-harir-stepwell.width`). | Supported for a well-house; the 9 m² floor sits above measured well shafts. Mill and lighthouse have no data (gap 5). |
| military (arsenal, barracks) | huge, 3 : 1 | Class: the Corderie ropewalk 6,636–6,657 (`corderie.area`). | Shape: the ropewalk is 15 : 1 (`corderie.aspect`); a full arsenal is a 45 ha precinct (`venetian-arsenal.precinct`). | Class supported by one building; shape understated for a ropewalk; barracks has no data (gap 3). |
| monument (column, market cross, statue) | tiny, 1 : 1 | Class and shape: column shaft 11, pedestal at least 30, market crosses 8.7–14.4, all square or round (`trajans-column.shaft-area`, `nelsons-column.pedestal-area`, `shillingstone-cross.area`, `brigstock-cross.area`). | Class: Shillingstone's 8.7 m² is 0.3 m² under the 9 m² floor. | Supported. |

### 5.2 Footprint classes

| Class | Spec range (m²) | Facts inside the range | Facts that contradict the range | Verdict |
|---|---|---|---|---|
| tiny | 9–80 | shrine 60, neighbourhood mosque 64, gate 46, yarn market 70, crosses 14.4, well-house 14, pedestal 30, column shaft 11 | wells 2.3, 2.9, 3.6 and the Shillingstone cross 8.7 lie under 9 | Upper bound supported; lower bound excludes measured wells (question 5). |
| small | 80–300 | towers 113–188, minaret 114, gate towers 81–113, market halls 128–265, chapel 98, Hōryū-ji Kondō 281 | Merchant Adventurers' Hall 299–339 straddles 300 | Supported. |
| medium | 300–1,200 | keeps 441–1,152, halls 528–996, playhouses 378–718, Hindu temples 620–911, Japanese inns 390–490, infirmary 1,092 | White Tower 1,152–1,170 is under 1,200 only without its corner turrets | Supported. |
| large | 1,200–5,000 | parish churches 1,550–2,200, castles 1,571–3,000 (wards), caravanserais 2,500–4,900, madrasa 4,536, Westminster Hall 1,460–1,547 | Sultan Han and Ulugh Beg sit within 10% of 5,000 | Supported. |
| huge | 5,000–20,000 | cathedrals 5,986–8,333 (bbox), Friday mosques 9,000–9,375, palaces 11,000–12,250, opera house 11,237–12,000, ropewalk 6,657, Córdoba roofed hall 14,200–14,720 | Friday mosque compounds of 22,400–26,318 exceed 20,000 | Supported; the largest mosques fall into the version 1 gap above huge (question 1). |

## 6. What this means for the spec

Each question has options, the facts it rests on and a recommendation. Jeffrey decides.

**Question 1. Religious: the default class large fits a parish church only. Which labels
does the default rule pack add?**

- A. Keep `religious: large, 2.5 : 1`. Add four labels: `chapel` small 2 : 1;
  `friday_mosque` huge 1.5 : 1, drawn as the roofed hall plus courtyard and clipped to
  20,000 m²; `mosque` tiny 1 : 1; `temple` medium 1.5 : 1. Change the existing `shrine`
  label to tiny 2 : 1.
- B. Keep the row and add no labels; specs set `footprint.class` per landmark.
- C. Change the function default to medium 1.5 : 1, the middle of all religious examples,
  and add `cathedral` and `parish_church` labels for the long Christian shapes.

Rests on: `cologne-cathedral.area`, `chartres-cathedral.bbox-area`, `st-mary-redcliffe.area`,
`st-mary-redcliffe.area-estimate`, `saint-gall-plan.church-area`,
`fountains-abbey.infirmary-chapel-area`, `escomb-church.area-internal`, `kairouan-mosque.area`,
`kairouan-mosque.length`, `kairouan-mosque.width`, `cordoba-mosque.area`,
`cordoba-mosque.prayer-hall-area`, `cristo-de-la-luz.area`, `ise-naiku.area`,
`ise-naiku.aspect`, `kandariya-mahadeva.area`, `todaiji-daibutsuden.area`,
`horyuji-kondo.area`.
Recommendation: A. It keeps the parish church as the default and makes each of the four
measured outliers one word in a spec.

**Question 2. The `cathedral` label's shape: 2.5 : 1 (spec) or 2 : 1?**

- A. Keep 2.5 : 1.
- B. Change to 2 : 1. Measured cathedral bounding boxes are 1.7 (Cologne) and 2.0
  (Chartres); the 2.5–2.8 figures measure the nave against the transept width, not the
  outline a map draws.
- C. Keep 2.5 : 1 and later draw a cross-shaped outline instead of a rectangle.

Rests on: `cologne-cathedral.length`, `cologne-cathedral.width`, `chartres-cathedral.length`,
`chartres-cathedral.width-transept`, `chartres-cathedral.aspect`, `st-mary-redcliffe.aspect`.
Recommendation: B for version 1, because the generator draws a rectangle; C is a later
rendering change and does not need a decision now.

**Question 3. Defence: the default class large fits a small castle only. What does the
default rule pack do?**

- A. Keep `defence: large, 1.3 : 1` and add labels: `castle` huge 1.3 : 1, `keep` medium
  1 : 1, `tower` small 1 : 1, `gatehouse` small 1.3 : 1.
- B. Change the default to medium 1 : 1 (a keep) and add `castle` large 1.3 : 1,
  `tower` small 1 : 1, `gatehouse` small 1.3 : 1.
- C. Keep the row and add no labels.

Rests on: `hen-domen.site-area`, `conwy.overall-area`, `conwy.overall-length`,
`beaumaris.inner-ward-area`, `beaumaris.ward-gap`, `harlech.inner-ward-area`,
`rochester-keep.area`, `white-tower.area`, `st-marks-campanile.area`, `pisa-tower.area`,
`corsican-towers.area`, `loughrea-gate.area`, `lipiany-gate.area`, `speyer-altportel.area`.
Recommendation: A. "large" stays the right default for an unlabelled small castle, and the
three smaller examples in the row each get the class their measurements give. `gatehouse`
small rather than tiny: measured gates are 46–113 m², and small keeps the gate readable.

**Question 4. Commerce: no measured market building is medium. Change the default?**

- A. Change `commerce` to small, 2.5 : 1, and add a `cloth_hall` label huge 4 : 1 for the
  Ypres type.
- B. Change `commerce` to small and keep 2 : 1; no label.
- C. Keep medium 2 : 1 until warehouse and exchange sizes are researched.

Rests on: `dunster-yarn-market.area`, `yeovil-market-house.area`, `yeovil-market-house.length`,
`hereford-market-hall.area`, `hereford-market-hall.length`, `ypres-cloth-hall.frontage`,
`aleppo-souq-core.length`, `aleppo-souq-core.width`.
Recommendation: B. Three of three measured market halls are small; the shape evidence
(2.5 and 3.6) is two buildings, too few to move the ratio; a `cloth_hall` label rests on one
frontage figure with no depth.

**Question 5. The tiny class floor of 9 m² sits above measured wells (2.3–3.6 m²). Change
it?**

- A. Keep 9 m² (a 3 × 3 m minimum). Wells are drawn larger than real, as a symbol.
- B. Lower the floor to 2 m² so tiny covers every measured well.
- C. Keep 9 m² and add a `well` label tiny with a fixed 9 m² size.

Rests on: `colchester-well.area`, `ipswich-well.area`, `nine-wells-st-davids.area`,
`dupath-well.area`, `shillingstone-cross.area`.
Recommendation: A. The class exists so the landmark is visible on an overview map, and a
3 × 3 m square is the smallest footprint the spec draws. This changes nothing in the spec.

**Question 6. Lodging: add a `caravanserai` label?**

- A. Add `caravanserai` large 1.3 : 1.
- B. Add no label; specs set `footprint.class: large`.

Rests on: `sultan-han.area`, `sultan-han.courtyard-area`, `khan-asad-pasha.area`,
`ishibashi-juku.honjin-area`, `kings-head-aylesbury.estimated-side`.
Recommendation: A. The row's own example "caravanserai" measures large in both cases, one
class above the row's default (medium).

**Question 7. Military: a ropewalk is 15 : 1, the row says 3 : 1. What does the default
rule pack do?**

- A. Keep `military: huge, 3 : 1` and add a `ropewalk` label huge 15 : 1.
- B. Change the military shape to 10 : 1.
- C. Keep the row and add no label.

Rests on: `corderie.area`, `corderie.length`, `corderie.width`, `corderie.aspect`,
`venetian-arsenal.precinct`.
Recommendation: A. The 15 : 1 measurement is a ropewalk, not an arsenal or barracks; the
row's 3 : 1 has no measurement for or against it (gap 3).

**Question 8. Open spaces have no size in the spec. Add default sizes per kind to the
default rule pack?**

- A. Add a size range and shape per open-space kind, drawn by seed like footprints:
  `market_square` 2,500–11,250 m² (the measured range, from 50 × 50 to 150 × 75 m), aspect
  1 : 1 to 2 : 1;
  `parade_ground` 90 m square (8,100 m²); `green` 80–100 m side (6,400–10,000 m²);
  `plaza` the same area and aspect as the market square, with the 1573-Ordinances range
  (83.6–222.9 m long, at least 1.5 : 1) in a Spanish-colonial pack only.
- B. Scale the market square with `settlement.population`, as Ordinance 113 does, using the
  bastide and Zamość figures as the small end.
- C. Leave open-space size to the generator stage that places them and decide it there.

Rests on: `bastides.early-square-side`, `monflanquin.square-side`, `revel.square-side`,
`marciac.square-length`, `marciac.square-width`, `rabastens.square-length`,
`rabastens.square-width`, `trie.square-length`, `trie.square-width`,
`zamosc.great-market-side`, `neuf-brisach.place-darmes-side`, `savannah.ward-square-length`,
`savannah.ward-square-width`, `bath.circus-diameter`, `laws-of-indies.plaza-min-length`,
`laws-of-indies.plaza-good-length`, `laws-of-indies.plaza-max-length`,
`laws-of-indies.plaza-min-aspect`, `laws-of-indies.plaza-min-ft`,
`laws-of-indies.plaza-max-ft`, `laws-of-indies.plaza-min-width-ft` (the conflict 3 facts),
`mexico-city.main-plaza-area`. The plaza range in option A uses the Iberian lengths and the
aspect only; the width facts `laws-of-indies.plaza-good-width` and
`laws-of-indies.plaza-max-width` are not used.
Recommendation: A. The bastide squares give a measured European range; the pinned-size rule
for landmarks (`resolved.width_m`, `resolved.depth_m`) then works for open spaces unchanged.
The green and parade-ground ranges rest on two and one measurements; the conflict on the
Ordinance's unit (conflict 3) is why the Spanish-colonial range stays in its own pack.

Open question for Jeffrey: Question 1, the religious labels. Options: A (keep large, add
`chapel`, `friday_mosque`, `mosque`, `temple`, change `shrine` to tiny), B (no labels),
C (default medium plus `cathedral` and `parish_church`). Recommendation: A.

Questions 2 to 8 are open as well; none is decided. They follow one at a time, in their
order, after Jeffrey answers question 1.
