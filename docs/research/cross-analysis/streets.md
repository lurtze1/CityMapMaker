# Cross-analysis: streets

Written 2026-10-20 for ADR-025. This document turns the street figures in the research reports
into facts and into questions for the spec. It follows the CLAUDE.md rule "explicit over
implied" in full (ADR-023): where a source is approximate, the source's own words are quoted;
everything else is stated plainly. Jeffrey decides every question in section 5.

Terms used here: a *fact id* is the `id` of a record in `data/research/facts/streets.yaml`,
written without the `streets.` prefix in the tables; *primary*, *secondary* and *tertiary* are
the three street classes of REQUIREMENTS.md F10; *degree* is the number of street segments
meeting at a junction.

## 1. Scope

Reports read in full: `settlement/streets.md`, `settlement/verification-streets-walls.md`,
`sources-by-language/` (README and all seven regional reports), `settlement/traditions.md`,
`settlement/growth-generators.md`, `city-foundation-and-growth.md`, `town-plan-corpus.md`,
`town-plan-corpus-world.md`. Reports skimmed because they cite street figures:
`settlement/cliff-towns.md` (gradients, stairs), `settlement/parcels-density.md` (Beijing
hutongs), `settlement/timber-ports.md` (Waterford, Wolin).

Recorded:

| File | Count |
|---|---|
| `data/research/facts/streets.yaml` | 298 facts |
| `data/research/sources/streets.yaml` | 120 sources |
| `data/research/units/streets.yaml` | 27 historical units |
| `data/research/quantities/streets.yaml` | 28 new quantities |

Facts by status: 80 verified, 3 corrected, 17 disputed, 198 unverified, 0 superseded.
Fact format v2 fields: 45 facts are `approximate`, 190 carry years with a `year_basis`, 16 of
them `year_approximate`, 24 name a `source_via`, and 17 list `disputes` (10 pairs).
Facts by quantity: 162 street widths, 21 segment lengths, 14 gradients, 11 street lengths, 11
street counts, the rest network metrics (junction mix, degree, meshedness, circuity,
orientation order), spacing, shares and fina rules.

Recording rules applied:

- A width prescribed in a historical unit is recorded in that unit (8 `danda-kautilya`,
  14 `vara-castellana`). The unit file names the factor and its source. Where two reports give
  two factors for one unit, there are two unit ids: `danda-kautilya` (1.69 m) and
  `danda-shamasastry` (1.83 m), `cubit-black-hinz` (54.04 cm) and `cubit-black-hakim`
  (49.5 cm), `rod-manasara-104` and `rod-manasara-108`, three Ottoman zira roundings.
- Figures a report author read off a historic plan carry the plan as source (kind `map`) and
  say so in the note. Figures a report gives without any outside source carry a source entry
  for the report itself (`citymapmaker-<report>-2026`).
- Source ids follow the agreed convention. A source with no year in the report ends in `-nd`;
  a web page with no publication date ends in the access year, 2026.
- Years are recorded only when the fact's own report dates that figure. `year_basis` values
  mean: `measured` the year of the network state that was measured (survey, dataset or a
  historical network date); `plan_date` the date of the plan or view the figure was read from;
  `built` the date the street was laid out (20 facts); `depicted` the moment or period whose
  built state a plan, reconstruction or excavation shows (17 facts: Pompeii AD 79, Merv
  8th-13th c., Tang Chang'an 7th-9th c., the Teotihuacan site plan c. 100-550); `ended` a
  figure valid until the place was abandoned, with only `year_to` (no facts in this topic);
  `founded` the town's founding
  date; `described` the date of the law, treatise or eyewitness text that states the figure;
  `census` and `estimated` are not used in this topic. A century is recorded as its full
  range (7th c. = 601 to 700).
- Not recorded: non-metric descriptions (Leges Henrici "16 armed knights abreast", Timbuktu
  "three riders abreast", Kaogong ji "72 chi" without a Zhou chi factor), street length per
  hectare and intersections per km² (the fact format has no length-per-area or
  count-per-area dimension), block sizes (another topic).

## 2. The picture

Metres below are what the database computes from the recorded unit. Status in brackets:
V verified, C corrected, D disputed, U unverified.

### 2.1 Medieval and early modern Europe, by class

| Class | Place, date | Width (m) | Fact id |
|---|---|---|---|
| Main street | Monpazier, 1284 | 6-8 (U) | `monpazier.carreyras` |
| Main street | Bastides in general | 6-10 (U) | `bastides.streets` |
| Main street | Villefranche-de-Rouergue, 1250-52 | 5.5-6 (U) | `villefranche.main-streets` |
| Main street | Figeac, 13th c. | 4-4.5 (U) | `figeac.main-street` |
| Main street | Castilian new towns, medieval | 6.69 (U) | `castile-new-towns.main-street` |
| Wall road | San Giovanni Valdarno, c. 1299 | 11.67 (U) | `san-giovanni-valdarno.wall-road` |
| Street | Castilian new towns | 3.34-4.18 (U) | `castile-new-towns.streets` |
| Lane | Dorestad, 680-850 | 4 (U) | `dorestad.lanes` |
| Lane | Villefranche-de-Rouergue | 2-3 (D) | `villefranche.lanes` |
| Alley | Bastides in general | 5-6 (D) | `bastides.alleys` |
| Cross lane | Figeac | 2-2.4 (U) | `figeac.cross-lanes` |
| Rear lane (carreyrou) | Monpazier | 1-2 (U), 2 (U) | `monpazier.carreyrous-snippet`, `monpazier.carreyrous-corpus` |
| Gap between plots | Monpazier; Figeac | 0.3 (U); under 0.5 (U) | `monpazier.androns`, `figeac.back-gap` |
| Street | Waterford, Viking age | over 2.5 (U) | `waterford.peter-street` |
| Street | Wolin | 1.55 (U) | `wolin.street` |
| Alley (calle) | Venice, 1500 | 1-3 (U) | `venice.calli` |
| All classes | Toledo | 2-5 (U) | `toledo.streets` |
| Lane minimum | London, 1667 | 4.27 (U) | `london.lanes-1667` |
| Rebuilt street | Lisbon Baixa, after 1755 | 12.19 (U) | `lisbon.baixa-streets` |
| Street minimum; carriageway | London, 1894 | 12.19; 7.32 (U) | `london.street-minimum-1894`, `london.carriageway-1894` |
| Portico height and width | Bologna, 1288 | 2.66 (U) | `bologna.portico-height`, `bologna.portico-width` |

No measured series of street widths by class exists in the reports for a medieval town in
England, Germany or the Low Countries (section 4).

### 2.2 Roman

| Class | Width (m) | Fact id |
|---|---|---|
| Right-of-way servitude, straight / on a bend | 2.37 / 4.74 (C) | `roman-law.right-of-way-straight`, `roman-law.right-of-way-bend` |
| Rural public road, common | 3.55 (U) | `roman.rural-road-common` |
| Principal roads, average of about 500 | 6.51 (U) | `roman.principal-road-average` |
| Roads measured | 1.1 to over 7.0 (U) | `roman.measured-road-range` |
| Pompeii main street (Via dell'Abbondanza) | 4.5-5 (U) | `pompeii.via-dell-abbondanza` |
| Pompeii ordinary streets / widest | 2.4-4.5 / just over 7 (U) | `pompeii.ordinary-streets`, `pompeii.widest-street` |

### 2.3 Islamic world

| Class | Place, date | Width (m) | Fact id |
|---|---|---|---|
| Disputed road | Hadith, 7th c. | 7 cubits = 3.49 at 0.498 m (V) | `islamic-law.disputed-road` |
| Same, cubit range 49.5-80 cm | streets.md computation | 3.5-5.6 (U) | `islamic-law.disputed-road-computed-range` |
| Livestock road | Al-Andalus, 10th-11th c. | 20 cubits = 9.96 (U) | `al-andalus.livestock-road` |
| Wide street threshold | Al-Andalus, 12th c. | 9 cubits = 4.48 (U) | `al-andalus.wide-street-threshold` |
| Footpath | Al-Andalus | 7 cubits = 3.49 (U) | `al-andalus.footpath` |
| Side lane not joining a road | Al-Andalus | 4 cubits = 1.99 (U) | `al-andalus.side-lane` |
| Encroachment threshold | Tunis, 1330s | 7 cubits = 3.49 (U) | `tunis.encroachment-threshold` |
| Main street (Qasaba) | Cairo, 969 | nearly 10 (U) | `cairo.qasaba-width` |
| Ordinary streets | Cairo, before 1850 | 2-3, "often less" (U) | `cairo.ordinary-streets` |
| Substantial streets / alleys | Merv, 8th-13th c. | 3-4 / 2-3.5 (V) | `merv.substantial-streets`, `merv.alleys` |
| Excavated street, laid out / later | Merv | 6 / 5 (V) | `merv.excavated-street-laid-out`, `merv.excavated-street-encroached` |
| Secondary streets | Valencia, La Boatella, 1150-1238 | 2.2-3.2 (U) | `valencia.boatella-secondary-streets` |
| Third-order lanes | Saltés, Almohad | 1.4-1.6 (U) | `saltes.lanes` |
| Fina, depth / share of street | Maliki law | 1.0-1.5 m / at most half (U) | `islamic.fina-depth`, `islamic-law.fina-share-main-street` |
| Dead ends, share of street length | Fez / Cairo / Aleppo | 52 / 47 / 41 % (U) | `fez.dead-end-length-share`, `cairo.dead-end-length-share`, `aleppo.dead-end-length-share` |

Ottoman Istanbul codes, all unverified (zira converted at the source's own rounding):

| Code | Classes (m) | Fact ids |
|---|---|---|
| 1839 ilmühaber | 15.20 main with 3.04 pavements; others at least 7.60 (Çelik); 15 / 11.25 / 9 / 7.5 (Yörüten) | `istanbul-1839.*` |
| 1848/49 | 9 / 7.5 / 4.5 | `istanbul-1848.*` |
| 1863 | 11.25 / 9 / 7.5 / 6, minimum 4.5 | `istanbul-1863.*` |
| 1865, after the fire | 4.55-18.94; Divanyolu 19; 25 % of plots taken | `istanbul-1865.*` |
| 1882 | 6.06-15.15 (snippet) | `istanbul-1882.road-classes` |

### 2.4 South Asian treatises and towns

| Text or place | Royal / main road | Lesser classes | Fact ids |
|---|---|---|---|
| Arthashastra, Sanskrit | 8 danda = 13.52 (D) | 4 danda 6.76; 2 danda 3.38; 5 aratni 2.11; 4 aratni 1.69; 2 aratni 0.85 | `arthashastra.*` |
| Arthashastra, Shamasastry 1915 | 4 danda = 7.32 (D) | village roads 8 danda 14.63; chariot 5 aratni 2.29 | `arthashastra.*-shamasastry` |
| Manasara | 9 rods = 16.47 | vithi 3-12 rods 5.49-21.96; lanes 1/2 to 3/4 of the large road | `manasara.*` |
| Mayamata | — | 13 widths, 1-7 danda, 1.69-11.83 | `mayamata.street-widths` |
| Samarangana, three city sizes | 24 / 20 / 16 hasta = 10.15 / 8.46 / 6.77 | great streets 5.08 / 4.23 / 3.38; carriage roads 1.69; footpaths 1.27 / 1.06 / 0.85 | `samarangana.*` |
| Sukranitisara | 30 / 20 / 15 cubits = 12.69 / 8.46 / 6.34 | marga 4.23; vithi 2.11; footpath 1.27 | `sukraniti.*` |
| Devi Purana | 40 cubits = 16.92 | branch 6.77; lane 1.27; bye-lane 0.85 | `devi-purana.*` |
| Vishvakarma Vastushastram | 10 danda typical, 5 minimum = 16.90, 8.45 | roads 2 to 3 times streets | `vishvakarma.*` |
| Jaipur, 1727 | 33.83 (111 ft); 33 from the plan | secondary 16, lanes 8 (plan) | `jaipur.*` |
| Delhi, Chandni Chowk | 36.58 (1650) or 45.72 (1883) (D) | length 1,390 m (1650) or 1.2 km (1883) (D) | `delhi.chandni-chowk-*` |
| Madurai | — | narrowest lanes 0.6 (snippet) | `madurai.narrowest-lanes` |

All Indic conversions use the Mauryan angula of 1.763 cm. South-asia.md states that applying
it to later treatises is an assumption of about plus or minus 15 %, and for the Sukranitisara,
Devi Purana and Vishvakarma figures no report gives a cubit or danda length at all; the
records apply the Mauryan factor and say so in each note.

### 2.5 East Asian capitals

| Place | Ceremonial axis | Major | Minor | Lanes | Fact ids |
|---|---|---|---|---|---|
| Chang'an, Tang | Zhuque 150-155 (V), 129.32 locally | 100 or 35-65 (D) | narrowest 20-25 | inside wards 15 | `changan.*` |
| Heian-kyō, 794 | Suzaku 28 jō = 84 (V) | common ōji 24 (C); some 30-36; south of palace 51 | kōji 12 (V) | — | `heian.*` |
| Heijō-kyō, 710 | 74 | Nijō 37 (snippet) | kōji 7 (snippet) | — | `heijo.*` |
| Dadu, Yuan | — | 24 bu = 37.44 | 12 bu = 18.72 | fire lane 6 bu 9.36; hutong 3 bu 4.68 (D); Zhuanta 4 measured | `dadu.*` |
| Beijing, Yuan onward | — | — | — | hutong about 9.14 (D), spacing 76.2 | `beijing.*` |
| Edo, after 1657 | — | Nihonbashi-dōri 18.18 | Honchō-dōri 13.79 | — | `edo.*` |
| Nagoya, 1700 | — | — | Shikemichi 7.27 | back lanes 2-3 | `nagoya.*` |
| Guangzhou, 1855 | — | — | — | commercial streets 2-3 | `guangzhou.commercial-streets` |

Ratios inside one planned capital: Dadu 24 : 12 : 6 : 3 bu; Heian 28 : 8 : 4 jō; Samarangana
royal road : great street : carriage road = 6 : 3 : 1.

### 2.6 Planned colonial and other grids

| Place, date | Street (m) | Fact id |
|---|---|---|
| Puebla, 1531 | 11.70 (14 varas) | `puebla.streets` |
| Mexico City, 1521-24 | 13.28 mean (16 varas) | `mexico-city.streets-mean` |
| Santiago de Chile, 1541 | 10.03 (12 varas) | `santiago.streets` |
| Lima, 1535 | 11 (snippet) | `lima.streets` |
| Buenos Aires, 1580 | 9.53 (snippet) | `buenos-aires.streets` |
| Quito | 11.15 (40 pies, snippet) | `quito.streets` |
| Porto, Rua Nova de São João, 1788 | 13.20 planned, 11.9-12.4 measured today | `porto.*` |
| New Orleans, 1720 | 11 (plan) | `new-orleans.streets` |
| Salt Lake City, 1870 | 40 | `salt-lake-city.streets` |
| Washington, 1791 diagonals | 50 (plan) | `washington.diagonal-avenues` |
| Teotihuacan, Avenue of the Dead | 40; 40-45 (plan) | `teotihuacan.avenue-of-the-dead-*` |
| Benin City, 1668 | about 120 feet (D); about 30 from the engraving (D) | `benin.*` |
| Kumasi, 1817 | 50-100 (sketch) | `kumasi.wide-streets` |

The 1573 Spanish Ordinances fix no street width (`indies-ordinances.streets-from-plaza`
records only the 4 + 8 streets leaving the plaza). The colonial widths of 9.5 to 13.3 m are
local choices.

### 2.7 Network shape, junctions and slope

| Measure | Organic examples | Planned examples | Fact ids |
|---|---|---|---|
| Median segment length (m), whole municipality | Venice 23.2, Jerusalem 44.0, Damascus 65.8, Cairo 66.6, Rome 73.7 | Manhattan 82.2, Chicago 105.3 | `boeing.*.median-segment` (V) |
| Mean segment length (m), 1-square-mile sample | Ahmedabad 27.59, Venice 31.25, Cairo 37.47, Bologna 66.26 | Savannah 64.77, New York 86.33, Barcelona 112.01 | `cardillo.*.mean-segment` (V) |
| Dead ends, share of nodes | Venice 0.300, London 0.251, Rome 0.161, Cairo 0.085 | Manhattan 0.027 | `boeing.*.dead-end-share` (V) |
| Four-way junctions, share of nodes | Venice 0.073, Rome 0.145 | Manhattan 0.572, Chicago 0.507 | `boeing.*.four-way-share` (V) |
| Mean degree | Venice 2.474, Rome 2.820 | Manhattan 3.508; Paris 3.0 constant 1789-2010 | `boeing.*.mean-degree`, `paris.mean-degree` (V) |
| Meshedness | Venice 0.152, Ahmedabad 0.262 | New York 0.348 | `cardillo.*.meshedness` (V) |
| Orientation order φ | Rome 0.005, Venice 0.017; São Paulo 0.002 | Kyoto 0.357, Manhattan 0.669, Chicago 0.899 | `boeing.*.orientation-order` (V) |
| Junction angles | within about 15° of 90° (short lines) or of 180° (long lines) | — | `deformed-grid.right-angle-band`, `deformed-grid.straight-on-band` (V) |

Gradients (all U): 19th-century first-class roads 1 in 30 (3.33 %), 1 in 40 advocated;
Telford's Holyhead road ruling 1 in 30, maximum 1 in 20 (5 %); animal carts 8 %, loaded
porters 10 %, pack animals and pedestrians 12 %; Roman loaded wagons "seldom" above 5 %, ruts
on 6.7-10 % stretches, mountain stretches 16-20 %; stairs about 45° (Bonifacio). Fact ids:
`britain.*`, `holyhead-road.*`, `paths.*`, `roman.loaded-wagons-usual`,
`roman.wheel-rut-stretches`, `roman.mountain-stretches`, `bonifacio.*`.

### 2.8 What the picture shows

1. Across cultures, a pre-modern town has three to five street classes, and adjacent classes
   differ by a factor between 2 and 3 (Dadu 2 at every step; Figeac 2; Samarangana 2 and 3;
   Vishvakarma 2 to 3; Manasara lanes at 1/2 to 3/4 of the large road).
2. Organic and small planned towns in Europe and the Islamic world put main streets at
   4 to 10 m, lanes at 2 to 4 m and the narrowest passages at 1 to 2 m (sections 2.1 to 2.3).
3. Planned capitals and colonial grids are a different scale: 10 to 13 m ordinary streets in
   Spanish America, 12 to 24 m in Heian-kyō and Dadu, ceremonial axes of 40 to 155 m.
4. Islamic law states widths only for disputes and new layouts; it does not set a minimum
   for every lane (islamic.md).
5. Widths shrink with age: Merv's excavated street went from 6 m to 5 m by encroachment.

## 3. Conflicts

Each row names the competing facts, what differs, the reason the reports give, and the status
recorded.

| # | Competing facts | What differs | Reason given in the reports | Status |
|---|---|---|---|---|
| 1 | `arthashastra.royal-road-sanskrit` vs `arthashastra.royal-road-shamasastry`, `arthashastra.royal-road-dutt` | royal road 8 danda (13.52 m) vs 4 danda (7.32 m) | south-asia.md: the GRETIL Sanskrit groups the royal road with seven other road types under "asta-dandah" (8 danda); the 4 comes from Shamasastry's 1915 English and Dutt's 1925 summary that follows it, "a translation split, not the Sanskrit". traditions.md, using Shamasastry, gives 4. | disputed |
| 2 | units `danda-kautilya` (1.69 m) vs `danda-shamasastry` (1.83 m) | length of one danda | unit factor: 96 angula of 1.763 cm (KAZ 2.20.18) vs Shamasastry's gloss of 6 feet. The two errors compound conflict 1: 13.52 m vs 7.32 m is a factor of 1.85. | both units kept |
| 3 | units `cubit-legal-hinz` (49.8 cm), `cubit-black-hinz` (54.04 cm), `cubit-black-hakim` (49.5 cm) | length of the cubit behind the seven-cubit rule; 7 cubits = 3.49 m to 3.78 m; streets.md's range of 3.5 to 5.6 m uses "other cubits 58-80 cm" | unit factor: Hinz (EI2) and Hakim (CSBE) give different lengths for the "black" cubit; a hadith commentary site gives 69 cm without a source and islamic.md does not use it | facts recorded with the legal cubit; range kept as `islamic-law.disputed-road-computed-range` |
| 4 | `islamic-law.disputed-road` read as a street minimum (streets.md generator section) vs as a dispute rule (islamic.md, sources-by-language README) | meaning, not value | the original texts apply 7 cubits when neighbours dispute a road or when a building encroaches; Isa ibn Musa allows 4-cubit side lanes; Ibn Habib limits the functional rule to newly founded quarters | verified (value); meaning recorded in the note |
| 5 | `al-andalus.wide-street-threshold` | 9 cubits vs 7 | manuscript variant noted by the edition's footnote | unverified; variant in the note |
| 6 | `roman-law.right-of-way-straight`, `roman-law.right-of-way-bend` | public street minimum (streets.md) vs right-of-way servitude across private land (verification) | Digest 8.3.8 and Festus: private ways 8 ft by law, public ways as wide as public use needs | corrected |
| 7 | `heian.oji-common` vs `heian.oji-south-of-palace`, `heian.oji-other-wide` | all ōji 8 jō (24 m) vs 8, 10, 12 or 17 jō | verification report: the Engishiki Kyōtei lists wider ōji; 24 m is the common ōji only | 8 jō corrected; others unverified (OCR-garbled source) |
| 8 | `heian.suzaku-oji` | 85 m vs 84 m vs 83 m | rounding by three sources of one 28-jō figure | verified as 83-85 m |
| 9 | `changan.main-streets-shaanxi`, `changan.avenues-to-gates-wikipedia` vs `changan.other-avenues-zhwiki` | main avenues about 100 m vs 35-65 m | not explained in the reports; the Shaanxi page and English Wikipedia give 100 m, Chinese Wikipedia 35-65 m | disputed |
| 10 | `changan.zhuque-avenue` vs `changan.zhuque-avenue-mingde-gate` | 150-155 m vs 129.32 m | 1950s-60s surveys vs one local reading near Mingde Gate | first verified, second unverified |
| 11 | `dadu.hutong` vs `beijing.hutong-width-yang`, `dadu.hutongs-unverified` | hutong 3 bu (4-5 m, Zhuanta measured 4 m) vs about 9 m | not explained in the reports. The 9.3 m figure equals Wang Bin's 6-bu fire lane (9.4 m); the reports do not say whether those sources call the fire lane a hutong | disputed (`dadu.hutongs-unverified` since the 2026-10-21 repair) |
| 12 | unit `bu-song-yuan` (1.56 m) vs Wang Bin's metres | Dadu main street 37.44 m vs 36 m; fire lane 9.36 m vs 9.4 m | Wang Bin converts at about 1.5 m per bu for the main streets and 1.57 m for the fire lanes | recorded in bu; Wang's metres in the notes |
| 13 | `delhi.chandni-chowk-1650` vs `delhi.chandni-chowk-1883`; `delhi.chandni-chowk-length-1650` vs `delhi.chandni-chowk-length-1883` | 40 yards (36.6 m) vs "about 50 yards" (45.7 m); 1,520 yards (1.39 km) vs "some three-quarters of a mile" (1.2 km) | different dates (1650 original, 1883 gazetteer); the reports do not say whether the street was widened, shortened or measured differently | disputed (the lengths since the 2026-10-21 repair) |
| 14 | `benin.streets-dapper` vs `benin.main-street-engraving` | about 120 feet (33.98 m if Amsterdam feet) vs about 30 m | the foot type is the report's assumption; the 30 m is read off van Meurs's engraving, made by an engraver who never saw Benin | disputed |
| 15 | `bastides.alleys` vs `villefranche.lanes` (and `figeac.cross-lanes`, `monpazier.carreyrous-corpus`) | alleys 5-6 m (general claim) vs 2-3 m, 2-2.4 m, 2 m measured in named towns | not explained; the general figure is from Wikipedia, the others from Séraphin and the town-plan corpus | disputed (the first two) |
| 16 | `kaogong-ji.streets-each-way` (note) | Zheng Xuan's road of 72 chi = 12 bu at 6 chi per bu vs Wikisource reading "11 bu" | edition difference, not checked by chinese.md | unverified |
| 17 | units `rod-manasara-104` vs `rod-manasara-108` | 1.83 m vs 1.90 m | the Manasara rod is 4 cubits of 26 angula (II.51-53) or, with dhanurgraha cubits, 108 angula | Manasara facts recorded in the 104-angula rod |
| 18 | units `zira-ozdural` (0.758), `zira-celik` (0.76), `zira-yoruten` (0.75) | 1.3 % spread | each Turkish source rounds the arşın differently | each fact uses its source's rounding |
| 19 | `istanbul-1839.main-road-celik`, `istanbul-1839.other-roads-minimum` vs `istanbul-1839.class-1` to `class-4` | two classes plus a minimum vs four classes | Çelik paraphrases the ilmühaber; Yörüten and Hamamcıoğlu tabulate four classes. Values agree where they overlap (20 and 10 zira) | unverified, both kept |
| 20 | `boeing.cairo.dead-end-share` vs `cairo.dead-end-length-share` | 8.5 % of nodes vs 47 % of street length | different measures and areas: Boeing counts nodes in the whole 2018 municipality, Raymond measures length in the old quarters | no conflict in the data; a generator must not compare them |
| 21 | `holyhead-road.ruling-gradient-britannica` vs `holyhead-road.maximum-gradient-coflein` vs `holyhead-road.whole-route-maximum` | 1 in 30 vs 1 in 20 vs 1 in 14 | a ruling (design) gradient vs a maximum on the Welsh section vs a maximum on the whole route (snippet) | unverified |
| 22 | Hillier statements in streets.md | "almost all ~90°" in planned grids; the angle pattern holding in "tortuous cul-de-sac sequences" | verification: the first is streets.md's own inference, the second is about a different principle in Hillier 1997 | not recorded as facts; only the two verified 15° bands are |
| 23 | Rome vs São Paulo as highest-entropy non-US city | φ 0.005 vs 0.002 | verification: São Paulo is first, Rome second | `boeing.sao-paulo.orientation-order` added |

### Repair round 2026-10-21

A sample check found errors in the street facts, and fact format v2 added five fields. Every
fact was checked again against the passage in its `report`. Changes:

- `holyhead-road.maximum-gradient-coflein`: the source `coflein-2026` pointed to Coflein
  record 305398 (a well-house at St Davids). It is replaced by `coflein-419799-2026`, the
  Holyhead Road record cliff-towns.md cites. The years 1815-1830 are removed from all three
  `holyhead-road.*` facts, because cliff-towns.md gives none.
- `samarangana.*` (12 facts): the years are now 1001 to 1100 (the 11th century) with
  `year_basis: described`; each note keeps the report's caveat "11th c. (Bhoja), not verified
  here".
- 45 facts gained `approximate: true`, for figures the report gives with "about", "c.", "~",
  "nearly", "almost", "some" or "approximately".
- Years removed from 28 facts whose report gives no date for that figure (among them the
  Twelve Tables servitude, five Heian-kyō facts and the Chang'an Mingde Gate reading in the
  verification report, the Barthelemy-Boeing ranges, the Haussmann share, the Castilian new
  towns, the Dadu figures in parcels-density.md, the Britannica paving limits). Years changed
  on 51 facts: centuries now span their full range (8th-13th c. = 701 to 1300), a plan reading
  carries the plan's date (Isfahan 1840, Jaipur 1914), the Chang'an Zhuque survey carries the
  1950s-60s survey dates, and "after 1657" or "after the 1865 fire" keeps only `year_from`.
- 190 facts gained `year_basis`; 16 gained `year_approximate` ("c. 1st-3rd c. CE", "c. 1299",
  "c. 1581", "c. 1840s", "c. 680-850", "c. 1150-1238", "c. 100-550").
- 24 facts gained `source_via`: the original is now the source and the work the report read
  is `source_via` (Isa ibn Musa and Ibn 'At via Ibn al-Rami; the 1839 ilmühaber via Çelik;
  Sukranitisara, Devi Purana and Rajavallabha via Dutt; the Shukuson taigaichō via Heibonsha;
  Bazzana via López and Gestoso; Hugh Davies via Designing Buildings Wiki). Nine source
  entries were added.
- 17 facts list `disputes`, in 10 pairs. Status changed from unverified to disputed on three
  facts: `delhi.chandni-chowk-length-1650`, `delhi.chandni-chowk-length-1883` (conflict 13)
  and `dadu.hutongs-unverified` (conflict 11).

## 4. Gaps

1. No measured street widths by class for a medieval town in England, Germany, the Low
   Countries or Italy. streets.md lists York, Winchester, Freiburg and Monpazier as not found;
   the Siena, Bologna and Todi statutes regulate widths, but no figure was found. The European
   rows in 2.1 come from southern France, Castile, Dorestad, Waterford, Wolin and plan readings.
2. London 1667 widths per street class are in Noorthouck's Appendix XLIX, not read.
3. Pompeii street widths from a primary survey (the 2008 hand measurements) were not read.
4. No widths for Tamil temple-town car streets, for ordinary Chinese county seats, for
   Sahelian or Swahili towns, or for Jaipur's secondary streets from a written source.
5. No metre value for the Zhou chi or bu, so the Kaogong ji widths stay in carriage tracks.
6. Heijō-kyō widths below the Suzaku avenue rest on a search snippet only.
7. No measured dead-end share by node for an Islamic old core, and no historic-core node
   statistics anywhere: every Boeing value covers a whole 2018 municipality.
8. No measured distribution of junction angles; Hillier gives the 15° bands and "a
   surprisingly high proportion", not a share.
9. No street gradient for a pre-1900 town street; the gradients are 19th-century road
   engineering, development practice and Roman roads.
10. No street length per hectare or intersections per km² can be stored: the fact format has
    no length-per-area or count-per-area dimension. Cardillo's 140 to 467 m/ha and
    Kulzhanova's 103 vs 87 intersections per km² stay in the reports.
11. Edo fire-break plaza widths and masugata (road crank) dimensions were not found.

## 5. What this means for the spec

The spec (`docs/proposals/spec-v1.md` on `m1/spec-proposal`) has `presets.era` with three
European medieval values, rule packs, and F10's three street classes: primary roads (spec
roads plus links from the core to every market square and every major landmark), secondary
streets per district, tertiary alleys per block. Neither the spec nor REQUIREMENTS.md sets a
width, a junction rule or a gradient. Each question below names the facts its options rest
on. Jeffrey decides.

**Q1. Primary street width in the default rule pack.**
- A: 6 to 8 m. Facts: `monpazier.carreyras`, `castile-new-towns.main-street`,
  `villefranche.main-streets`, `merv.excavated-street-laid-out`,
  `roman.principal-road-average`.
- B: 8 to 12 m. Facts: `bastides.streets`, `cairo.qasaba-width`,
  `san-giovanni-valdarno.wall-road`, `santiago.streets`, `puebla.streets`.
- C: 4 to 6 m. Facts: `figeac.main-street`, `pompeii.via-dell-abbondanza`, `dorestad.lanes`,
  `merv.substantial-streets`.
- Recommendation: A. Of the six European medieval main-street records in 2.1, four lie inside
  or touch 6 to 8 m (Monpazier, the bastide general claim, the Castilian new towns,
  Villefranche at 5.5-6 m); Figeac lies below and San Giovanni's wall road above. B rests on
  colonial grids, the general bastide claim and a Fatimid capital's spine; C is the narrow
  end, reached by Figeac, Pompeii and Dorestad.

**Q2. Secondary street width in the default rule pack.**
- A: 3 to 4.5 m. Facts: `castile-new-towns.streets`, `merv.substantial-streets`,
  `al-andalus.footpath`, `london.lanes-1667`, `dorestad.lanes`.
- B: 2 to 3 m. Facts: `villefranche.lanes`, `figeac.cross-lanes`,
  `valencia.boatella-secondary-streets`, `cairo.ordinary-streets`.
- C: 5 to 6 m. Facts: `bastides.alleys`.
- Recommendation: A. It is Q1-A halved (3 to 4 m, the factor 2 in 2.8) extended to the
  Castilian upper value of 4.18 m, and it holds the Castilian and Merv measurements. C rests
  on one disputed general claim.

**Q3. Tertiary alley width in the default rule pack.**
- A: 1.5 to 2.5 m. Facts: `monpazier.carreyrous-corpus`, `monpazier.carreyrous-snippet`,
  `saltes.lanes`, `al-andalus.side-lane`, `figeac.cross-lanes`, `wolin.street`.
- B: 2 to 3.5 m. Facts: `merv.alleys`, `nagoya.back-lanes`, `venice.calli`.
- C: 1 to 3 m. Facts: `venice.calli`, `toledo.streets`.
- Recommendation: A. It is Q2-A halved (1.5 to 2.25 m) rounded up to 2.5 m, and it covers the
  rear lanes and side lanes of four traditions. Gaps between plots (`monpazier.androns`, `figeac.back-gap`, under 0.5 m) are
  plot boundaries, not streets, and are not drawn as alleys.

**Q4. Do the widths depend on `presets.era`?**
- A: No; one width table for early, high and late medieval in version 1.
- B: Yes; older streets narrower by encroachment, using Merv's 6 m to 5 m (17 %). Facts:
  `merv.excavated-street-laid-out`, `merv.excavated-street-encroached`.
- C: Yes; a separate width table per era.
- Recommendation: A. The facts hold no European width series by era (gap 1), so C has no
  data. B rests on one excavated street; it fits a later "age" parameter (streets.md
  proposes one), which is a new spec field and needs its own issue.

**Q5. How a street's width is chosen within its class.**
- A: Each class has a range; each street draws its width from its own random stream.
- B: Each class has one fixed width (the middle of the range).
- C: Width falls with distance from the core within the class range, as in Florentine new
  towns (continental.md quotes Treccani on "the harmonious decrease in the width of streets"
  away from the axis; no fact records a number for it).
- Recommendation: A. It keeps the sources' ranges (ADR-023) and stays deterministic through
  per-object streams (hard rule 2). C has no figures behind it.

**Q6. Width of the link streets F10 adds (core to every market square, core to every major
landmark).**
- A: Primary width (Q1).
- B: A fourth class between primary and secondary.
- C: Secondary width (Q2).
- Facts: `indies-ordinances.streets-from-plaza` (the twelve streets leaving a plaza are the
  main streets), `cairo.qasaba-width` (the spine linking gates and markets is the widest
  street), `samarangana.royal-road-large` and `samarangana.great-street-large` (two
  classes above the ordinary streets).
- Recommendation: A. F10 already counts the links as primary, and the facts show the street
  to the market as the widest class.

**Q7. Junction angle rule for secondary and tertiary streets.**
- A: When a new street meets an existing one, snap the angle to 90° ± 15° (short streets) or
  180° ± 15° (long through streets). Facts: `deformed-grid.right-angle-band`,
  `deformed-grid.straight-on-band`.
- B: No construction rule; a property test checks a share of junctions inside those bands.
- C: No rule and no test in version 1; Jeffrey judges at tier T4.
- Recommendation: A, with the T4 visual check. B needs a target share, and no report gives
  one (gap 8), so a number would be invented.

**Q8. Junction-mix and dead-end targets.**
- A: No target in version 1; every run prints junction mix, mean degree, median segment
  length, meshedness, circuity and φ next to the reference rows in 2.7.
- B: Target bands per district texture as streets.md proposes (organic: 15-30 % dead ends,
  7-15 % four-way nodes; grid: over 30 % four-way, under 8 % dead ends). Facts:
  `boeing.venice.dead-end-share`, `boeing.rome.dead-end-share`, `boeing.venice.four-way-share`,
  `boeing.manhattan.four-way-share`, `barthelemy-boeing.dead-end-share-range`.
- C: For Islamic-style packs, a dead-end share of 41-52 % of street length. Facts:
  `fez.dead-end-length-share`, `cairo.dead-end-length-share`,
  `aleppo.dead-end-length-share`.
- Recommendation: A. The B bands come from whole 2018 municipalities, not historic cores
  (gap 7), and C belongs to a culture pack, not to the default pack.

**Q9. Street gradient limits on terrain (`site.terrain` hills, slope, cliffs, plateau).**
- A: Primary streets at most 5 % (1 in 20), routed as switchbacks along contours when the
  direct line is steeper; secondary streets at most 12 %; steeper tertiary links become
  stairs. Facts: `holyhead-road.maximum-gradient-coflein`, `britain.wood-paving-limit`,
  `roman.loaded-wagons-usual`, `paths.pedestrian-desirable-maximum`,
  `bonifacio.aragon-stairs-angle`.
- B: Primary streets at most 3.33 % (1 in 30). Facts:
  `britain.first-class-road-ruling-gradient`, `holyhead-road.ruling-gradient-britannica`.
- C: No gradient rule in version 1.
- Recommendation: A. It matches the cliff-towns report's rules R8 and R9, and Bearna
  (milestone 9) needs stairs. B is a 19th-century design target for main roads between
  towns, stricter than a town street.

**Q10. How a culture rule pack states widths.**
- A: In the historical unit with a unit id from `data/research/units/` (for example 20
  `cubit-legal-hinz`), converted once at load.
- B: In metres only, with the fact ids in the pack.
- Facts: every fact in 2.3 to 2.5; the unit conflicts 2, 3, 12, 17 and 18 in section 3.
- Recommendation: A. The sources-by-language README and south-asia.md ask for it, and it lets
  a pack switch a disputed factor (danda, cubit) without rewriting its widths.

**Q11. Which Arthashastra royal-road width an Indic pack uses (conflict 1).**
- A: 8 danda, the Sanskrit (`arthashastra.royal-road-sanskrit`).
- B: 4 danda, Shamasastry's English (`arthashastra.royal-road-shamasastry`).
- C: Both, as a pack parameter.
- Recommendation: A. It is the primary text, and the same verse gives the 8 to every road
  type it lists with the royal road. The facts stay `disputed` until Jeffrey decides.

Open question for Jeffrey: Q1, the primary street width in the default rule pack. Options:
A 6 to 8 m, B 8 to 12 m, C 4 to 6 m. Recommendation A.
