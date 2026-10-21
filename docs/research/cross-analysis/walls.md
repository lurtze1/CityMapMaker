# Cross-analysis: walls and gates

Written 2026-10-20 for ADR-025; revised 2026-10-21 in the repair round (§7). This document
turns research into decisions, so it follows the CLAUDE.md rule "Explicit over implied" in
full; quoted source uncertainty keeps its own words (ADR-023). Every figure taken from a report
is a fact in `data/research/facts/walls.yaml`, named by its id without the `walls.` prefix where
a table column says so. Figures derived by arithmetic on those facts are not facts: the size
bands in the §2.1 reading, the Xi'an rampart spacing check in §2.5, the Cologne 401 to 770 ha
step in §2.6, the loop-area checks in §3 (a closed loop of length P holds at most P^2/4pi) and
the two-thirds ratio in Q9. The 0.3 to 4.0 gates per km in §2.4 and Q5 is walls-growth.md's own
arithmetic, also not a fact. Jeffrey decides every question in §6.

## 1. Scope

- **Topic.** Walled areas, circuit lengths, wall height and thickness, towers and their spacing,
  ditches and glacis, gate counts, number of rings and the area step between rings, walls that
  use water, razed walls that became ring roads, citadels.
- **Reports read (read-only):** `settlement/walls-growth.md`, `settlement/verification-streets-walls.md`,
  `town-plan-time-series.md` (the version corrected on 2026-10-21), `city-foundation-and-growth.md`, the seven
  `sources-by-language/*.md` and their README, `town-plan-corpus.md`, `town-plan-corpus-world.md`,
  `settlement/peninsula-ports.md`, `settlement/cliff-towns.md`.
- **Result.** 512 facts, 99 sources (`sources/walls.yaml`), 5 units (`units/walls.yaml`: two
  sizes of li, mile, foot, inch), 13 quantities (`quantities/walls.yaml`). Status counts:
  32 verified, 4 corrected, 5 superseded, 36 disputed (18 pairs), 435 unverified. The validator
  reports 0 errors and the 41 analysis tests pass. §7 lists what the repair round of 2026-10-21
  changed.
- **How the time-series report is used.** An independent review found that §14 and §15 of
  `town-plan-time-series.md` overstated the city blocks; the report was corrected on 2026-10-21. This
  analysis takes no fact and no pattern from §14 or §15. Every time-series fact cites a city
  block, and §5 lists the exceptions the city blocks show.
- **Sources that are reports.** Where a report gives a figure without a per-figure source (the
  two town-plan corpora, the time-series report, search snippets), the fact cites a source entry
  for the report's own claim (`citymapmaker-*-2026`) and stays `unverified`.
- **Era words in the facts:** `ancient` before Rome, `roman`, `early_medieval` 500-999,
  `high_medieval` 1000-1299, `late_medieval` 1300-1499, `early_modern` 1500-1799,
  `nineteenth_century` 1800-1899. A fact without a year in its report carries no year.
- **Left out of the facts:** figures in units the reports do not define (Djenné "deux milles et
  demi", Timbuktu "trois milles", Benin "5 or 6 Dutch miles", Mbanza Kongo "about 1 mile"
  translating "miglio", the Zhou-li ladder 9/7/5/12 li, Dur-Sharrukin "16280 Assyrian units",
  Samarangana wall widths and gate widths in hasta that the report did not convert). §4 lists them.

## 2. The picture

### 2.1 Circuit and walled area, European and Mediterranean towns

| Size | Place, era | Circuit | Area | Fact ids |
|---|---|---|---|---|
| Small planted | Caernarfon, 1283-1292 | 734 m | c. 4 ha | caernarfon.circuit, caernarfon.area |
| Small planted | Conwy, 1283-1287 | about 1.2 km (corrected) | 10 ha (verified) | conwy.circuit, conwy.area |
| Small planted | Stollhofen, 13th c. | 842.4 m | not given | stollhofen.circuit |
| Small planted | San Giovanni Valdarno, 1299 | rectangle about 187 x 467 m | not given | san-giovanni.wall-length, san-giovanni.wall-width |
| Small planted | Monpazier, 1284 | rectangle 400 x 220 m | not given | monpazier.length, monpazier.width |
| Small | Aigues-Mortes, 1272 | 1,640 to 1,650 m | not given | aigues-mortes.circuit-beyond, aigues-mortes.circuit-wikipedia |
| Small | Dubrovnik | about 1,940 m | 96.7 ha "Old City" (see §3) | dubrovnik.circuit, dubrovnik.area |
| Middle | Ávila, 11th-14th c. | 2,516 m (verified) | 31 or 33 ha (disputed) | avila.circuit, avila.area-31, avila.area-33 |
| Middle | Nördlingen, 1327 | 2.6 to 2.7 km (verified) | 4 x the 1215 area | nordlingen.circuit, nordlingen.area-growth-1327 |
| Middle | Turin, Roman | 2,875 m | 45 ha | turin.circuit, turin.area |
| Middle | Pompeii, 79 | about 3.2 km | 66 ha | pompeii.circuit, pompeii.area |
| Middle | Visby | 3.4 km | not given | visby.circuit |
| Middle | Cologne, Roman | almost 4 km (verified) | 97 ha (corrected) | cologne.roman-circuit, cologne.roman-area |
| Middle | York, 13th-14th c. | 4.43 km (corrected) | 106 ha (verified) | york.circuit, york.area |
| Middle | Paestum | 4.75 km | 120 ha | paestum.circuit, paestum.area |
| Large | Nuremberg outer, 14th-15th c. | 5 km | not given | nuremberg.outer-circuit |
| Large | Paris, Philip Augustus 1190-1213 | about 5.1 km (verified) | 273 or 253 ha (disputed) | paris.philip-augustus-circuit, paris.philip-augustus-area |
| Large | Siena, 14th c. | 7 km | not given | siena.circuit |
| Large | Cologne, 1180-1259 | "over 8 km" (untraceable) | 401 ha (verified) | cologne.circuit-1180, cologne.area-1180 |
| Large | Florence, 1284-1333 | 8.5 km (verified) | 430 ha (verified) | florence.circuit, florence.area |
| Capital | Rome, Aurelian Wall | 19 km | 1,370 ha | rome.aurelian-circuit, rome.aurelian-area |
| Capital | Constantinople | land wall 5.7 or 6.5 km | about 1,400 ha | constantinople.theodosian-land-wall, constantinople.walled-area |

Reading: below 15 ha the circuits run 0.7 to 1.3 km; 30 to 120 ha towns have 2.5 to 4.75 km;
the 270 to 430 ha cities have 5 to 8.5 km.

### 2.2 Circuit by rank, China

| Rank | Median circuit, Qing encyclopedia (li_qing, 576 m) | Mean, Gu Zuyu (li_skinner, 500 m) | Fact ids |
|---|---|---|---|
| Empire capital | not given | 82.0 li (n=2) | china-capital.circuit-mean |
| Prefecture | 9.0 li, p25-p75 5.9-12.3 (n=132) | 10.2 li | china-prefecture.circuit-median, china-prefecture.circuit-mean |
| Department / sub-prefecture | 6.0 li, p25-p75 3.8-9.0 (n=183) | 6.65 li | china-department.circuit-median, china-subprefecture.circuit-mean |
| County | 4.3 li, p25-p75 3.0-6.0 (n=838) | 4.44 li | china-county.circuit-median, china-county.circuit-mean |
| Garrison / post | 7.3 li / 3.0 li | not given | china-garrison.circuit-median, china-post.circuit-median |
| All walled seats, 19th c. | mean 2.76 km, mean area 0.78 km2 | | china-seats.circuit-mean, china-seats.area-mean |

Single places: Fengshan 2,765 m, Songjiang 5.46 km, Pingyao 5.2 km, Suzhou 14.3 km, Xi'an
13.74 km (verified), Nanjing 35 km, Tang Chang'an 36.7 km and 83.1 km2.

### 2.3 Circuit and area, rest of the world

| Region | Places and figures | Fact ids |
|---|---|---|
| Ancient Near East | Dur-Sharrukin 1,758.6 x 1,635 m, about 290 ha, 157 towers; Nineveh about 12 km, 750 ha; Babylon inner 400 ha on the east bank; Nippur 135 ha | dur-sharrukin.*, nineveh.*, babylon.inner-area-east, nippur.area |
| West Africa | Kano 11 miles (Johnston), 14 to 24 km (corpus-world), wall 9-15 m high; Old Oyo three rings of 7.5, 18 and 28 km, 200 and about 3,000 ha; Benin inner bank 11 or 12 km | kano.*, old-oyo.*, benin-city.* |
| East Africa | Harar 3.5 km, 48 ha; Gondar royal enclosure 900 m | harar.circuit, harar.walled-area, gondar.enclosure-circuit |
| Islamic world | Jerusalem 4 km, 90 ha; Fez el-Bali 280 ha and el-Jdid 90 ha; Cairo Fatimid 110 ha; Tunis 270 ha; Khiva inner 26 ha, outer 6 km | jerusalem.*, fez.*, cairo.fatimid-area, tunis.area, khiva.* |
| South Asia | Ahmedabad 9.39 km; Delhi 8.85 km or 6 km, about 600 ha; Jaipur about 650 ha; Lahore 250 ha; Fatehpur Sikri 6 km on three sides | ahmedabad.circuit, delhi.*, jaipur.walled-area, lahore.area, fatehpur-sikri.circuit |
| Southeast and East Asia | Seoul 18.6 km, 1,600 ha; Suwon 5.7 km, 130 ha; Mandalay 2 km square, 400 ha; Hue 520 ha; Ayutthaya 700 ha; Bangkok core 410 ha | seoul.*, suwon.*, mandalay.*, hue.citadel-area, ayutthaya.area, bangkok.walled-core-area |
| Americas | Lima about 5 km2 with 34 bastions; Havana 140 ha; New Orleans 60 ha; New Amsterdam 30 ha; Cahokia palisade 3 km, 80 ha | lima.*, havana.area, new-orleans.area, new-amsterdam.area, cahokia.* |
| Smallest | Lo Manthang 750 m, 3 ha, 1 gate; Aït Benhaddou 3-4 ha; Trelleborg 136 m inner diameter | lo-manthang.*, ait-benhaddou.area, trelleborg.inner-diameter |

### 2.4 Gates

| Group | Gate counts | Fact ids |
|---|---|---|
| Small planted European | Caernarfon 2, Conwy 3 (+2 posterns), San Giovanni 4, Monpazier 6 | caernarfon.gates, conwy.gates, san-giovanni.gates, monpazier.gates |
| Middle European | Exeter 4, York 4 main (verified), Cardiff 5, Nördlingen 5 (verified), Edinburgh about 6, Southampton 8, Ávila 9 (verified), Siena 8 | exeter.gates, york.gates, cardiff.gates, nordlingen.gates, edinburgh.gates, southampton.gates, avila.gates, siena.gates |
| Large European | Nuremberg 4 main, Cologne 1180 wall 12 (verified), Coventry 12, Florence 12 or 15 (disputed), Paris Philip Augustus 13 or 15 (disputed) | nuremberg.outer-gates, cologne.gates-1180, coventry.gates, florence.gates-12, florence.gates-15, paris.philip-augustus-gates |
| Chinese counties | 4 gates 59 %, 3 gates 16 %, 5 gates 12 %, 6+ 9 %, 1-2 gates 4 %; mean over all seats 4.24 | china-county.gates-*-share, china-seats.gates-mean |
| Chinese prefectures | 4 gates 44 %, 5 gates 16 %, 3 gates 10 %, 6+ 29 % | china-prefecture.gates-*-share |
| Textual ideals | Kaogong ji 12 (3 per side), Arthashastra 12, Samarangana 12 (3 per side), Manasara 4 | kaogong-ji.gates, arthashastra.gates, samarangana.gates, manasara.gates |
| Large walls elsewhere | Kano 13 (historic) or 15 (today, verified), Nineveh 15, Lahore 13, Nanjing 13, Delhi 10 or 13-14, Ahmedabad 18 or 12, Guangzhou 16, Hanoi rampart 16 | kano.*, nineveh.gates, lahore.gates, nanjing.gates, delhi.*, ahmedabad.*, guangzhou.gates, hanoi.rampart-gates |
| Dozens | Beijing 24 across four enclosures, Moscow Skorodom about 34, Edo moat 36, Bangkok about 60 | beijing.gates-total, moscow.skorodom-gates, edo.moat-gates, bangkok.gates |
| Water gates beside land gates | Suzhou 6 land + 5 water, Hangzhou 13 + 5, Ayutthaya about 16 + 20, Constantinople 9 land against 14 and 13 on the two sea walls | suzhou.*, hangzhou.*, ayutthaya.*, constantinople.*-gates |
| Gates that changed | Djenné 11 gates, 3 blocked, 8 left; Suzhou 8 gates, 2 blocked in early Song; Harar 5, a sixth added in the 19th c.; Kano "2 of them new" | djenne.gates-original, djenne.gates-open, suzhou.gates-original, harar.gates-historic, harar.gates-19c, kano.gates-today |

walls-growth.md computes 0.3 to 4.0 gates per km of wall over twelve towns (its own arithmetic)
and states that smaller circuits have more gates per km.

### 2.5 Towers, wall size, ditch, glacis

| Measure | Values | Fact ids |
|---|---|---|
| Tower spacing | Ávila 28.6-28.9 m (verified); Conwy 46 m (verified); Arthashastra 50.7 m; Paris Philip Augustus 60 m (verified); Constantinople inner land wall 21-77 m (typical curtain 40-60 m), outer 48-78 m; Xi'an rampart 120 m (unverified, 98 ramparts on 13.74 km would be 140 m) | avila.tower-spacing, conwy.tower-spacing, arthashastra.tower-spacing, paris.philip-augustus-tower-spacing, constantinople.*-tower-spacing, xian.rampart-spacing |
| Wall height, European | York 4 m, Florence 6 m, Paris 6-8 m, Prague 6-10 m, Conwy 9 m, Ávila 12 m, Constantinople 12 m, Dubrovnik up to 25 m | york.wall-height, florence.wall-height, paris.philip-augustus-wall-height, prague.new-town-wall-height, conwy.wall-height, avila.wall-height, constantinople.inner-land-wall-height, dubrovnik.wall-height |
| Wall height, elsewhere | Harar 5 m, Samarangana 5.5-7.2 m, Jaipur 6 m, Chinese mean 7.47 m, Mandalay 8 m, Khiva 8-10 m, Kano 9-15 m, Xi'an 12 m | harar.wall-height, samarangana.wall-height, jaipur.wall-height, china-seats.wall-height-mean, mandalay.wall-height, khiva.inner-wall-height, kano.wall-height, xian.wall-height |
| Wall thickness, European | Conwy 1.68 m, York 1.8 m, Vila-real almost 2 m, Ávila 3 m, Paris about 3 m at base, Prague 3-5 m, Constantinople inner 4.5-6 m, Dubrovnik land 4-6 m and sea 1.5-5 m | conwy.wall-thickness, york.wall-thickness, vila-real.wall-thickness, avila.wall-thickness, paris.philip-augustus-wall-thickness, prague.new-town-wall-thickness, constantinople.inner-land-wall-thickness, dubrovnik.*-wall-thickness |
| Wall thickness, elsewhere | Ahmedabad 1.2-1.5 m, Jaipur 3 m, Great Zimbabwe 5 m, Chinese mean base 7.26 m, Kano about 12 m at base, Xi'an 15-18 m at base | ahmedabad.wall-thickness, jaipur.wall-thickness, great-zimbabwe.great-enclosure-thickness, china-seats.wall-thickness-mean, kano.wall-thickness, xian.wall-thickness-base |
| Ditch width | Prague 10 m, Arthashastra 16.9-23.7 m (three moats), Xi'an 18 m (verified), York 18.3 m, Constantinople over 20 m, San Giovanni moat plus outer road 21.9 m, Hue 23 m, Stollhofen 24.3 m, Songjiang 32 m, Mandalay 64 m | prague.old-town-moat-width, arthashastra.moat-widths, xian.moat-width, york.ditch-width, constantinople.moat-width, san-giovanni.moat-and-outer-road, hue.moat-width, stollhofen.moat-width, songjiang.moat-width, mandalay.moat-width |
| Ditch depth | York 3 m, Xi'an 6 m (verified), Prague 8 m, Constantinople up to 10 m | york.ditch-depth, xian.moat-depth, prague.old-town-moat-depth, constantinople.moat-depth |
| Moat share | 96 % of 19th-c. Chinese walled seats | china-seats.moat-share |
| Glacis | Delhi 274 m round the fort after 1857; Vienna 400-500 m or about 500 m | delhi.fort-glacis, vienna.glacis-400-500, vienna.glacis-500 |
| Road inside the wall | San Giovanni 11.7 m | san-giovanni.intramural-road |

### 2.6 Rings and growth steps

| Place | Rings and steps | Fact ids |
|---|---|---|
| Cologne | 97, 122, 223, 401 ha; then 770 and 1,006 ha; steps between the first four areas x1.26 to x1.83 (corrected); the step from 401 to 770 ha is x1.92 (derived) | cologne.roman-area, cologne.area-950, cologne.area-1106, cologne.area-1180, cologne.area-1794, cologne.area-1883, cologne.ring-growth |
| Paris | 273 ha (Philip Augustus), 440 ha (Charles V plus left bank), 3,370 ha (Fermiers-Généraux, 24 km), 7,800 ha (Thiers) | paris.philip-augustus-area, paris.charles-v-area, paris.fermiers-generaux-*, paris.thiers-area |
| Amsterdam | 75 ha (1514), 100 or 150-180 ha (1544), 440-460 ha (1625), 700-730 ha (1688), 26 bastions | amsterdam.walled-area-*, amsterdam.bastions-1663 |
| Nördlingen | one step of x4 in 1327; two rings readable | nordlingen.area-growth-1327, nordlingen.ring-count |
| Moscow | four rings: Kremlin 2.2 km and 28 ha, Kitai-gorod 60 ha, Bely Gorod 9 or 10 km, earth rampart 15 or 16 km and 1,800-1,900 ha; customs rampart 37 km and 7,000-7,200 ha | moscow.* |
| Vienna | walled 110-130 ha; Linienwall 5,000-6,000 ha | vienna.walled-area, vienna.linienwall-area |
| Other ring counts | Florence 4 to 6 over time; Bern 3 stages; Old Oyo 3; Beijing 4; Hue 3; Bangkok 3 water rings; Carcassonne 2; Nuremberg 2; Srirangam 7 | florence.ring-count, bern.wall-stages, old-oyo.ring-count, beijing.ring-count, hue.ring-count, bangkok.ring-count, carcassonne.ring-count, nuremberg.ring-count, srirangam.ring-count |
| Walled against built | Hausa walls 2 to 3 times the town; Bristol 55 ha walled, 130 ha settled | hausa.walled-to-built, bristol.walled-area, bristol.settled-area |
| Share of cities walled | China 62 % in 1368, 95 % in 1576 | china-cities.walled-share-1368, china-cities.walled-share-1576 |

### 2.7 Citadels and ruler's enclosures

| Place | Size | Fact ids |
|---|---|---|
| Timgad Byzantine fort | 120 x 80 m, about 1 ha, 8 towers | timgad.fort-* |
| Maastricht castrum | about 170 x 90 m, 2 gates | maastricht.castrum-* |
| Durham plateau (castle, cathedral, green) | c. 10 ha | durham.plateau-area |
| Moscow Kremlin | 2.2 km, 28 ha, 20 towers, 5 gates | moscow.kremlin-* |
| Kano palace compound | about 33 ha | kano.palace-compound-area |
| Beijing Forbidden City | 72 ha | beijing.forbidden-city-area |
| Old Oyo palace wall | 7.5 km, 200 ha | old-oyo.inner-* |
| Edo castle complex | about 300 ha | edo.castle-area |

## 3. Conflicts

| Competing facts | What differs | Reason | Status |
|---|---|---|---|
| cologne.roman-area / cologne.roman-area-kuladig / cologne.roman-area-schaefer | 97 against 25 against 99 ha | kuladig prints a wrong figure; a 3.9 km square loop holds 95 ha | corrected (97); 25 superseded; 99 a snippet, unverified |
| cologne.ring-growth / cologne.ring-growth-old | x1.26-1.83 against x1.8-4.9 | the old range rested on the 25 ha | corrected; old superseded |
| florence.area / florence.area-650 | 430 against 650 ha | an 8.5 km loop holds at most 575 ha | 430 verified; 650 superseded |
| florence.gates-12 / florence.gates-15; florence.towers-63 / florence.towers-73 | 12 or 15 gates, 63 or 73 towers | two secondary sources, no primary | disputed |
| avila.towers-88 / avila.towers-87; avila.area-31 / avila.area-33 | 88 or 87 towers, 31 or 33 ha | Wikipedia against official tourism and hoteles.net | disputed |
| paris.philip-augustus-area / -area-253; paris.philip-augustus-gates / -gates-15 | 273 against 253 ha; 13 against 15 gates | the heritage body (Mérimée) against Wikipedia; the verification report calls both a conflict and corrects neither | disputed |
| york.circuit / york.circuit-walled-stretches | 4.43 against 3.4 km | 3.4 km leaves out the gaps for water; 106 ha cannot fit in 3.4 km | 4.43 corrected; 3.4 superseded |
| conwy.circuit / conwy.circuit-wikipedia | 1.2 against 1.3 km | heritage body against Wikipedia | 1.2 corrected; 1.3 superseded |
| kano.circuit-johnston / kano.size-unesco / kano.circuit-range | 11 miles against "14km radius" against 14-24 km | UNESCO's "14km radius" is not a circuit length and cannot be compared (verification report); the 14-24 km range contains Johnston's 17.7 km | not a conflict: all three unverified |
| kano.gates-johnston / kano.gates-today | 13 against 15 | different dates; 2 gates are "new" | not a conflict: both kept with their dates |
| xian.circuit / xian.prefecture-circuit-gazetteer | 13.74 km against 40 li (about 23 km) | not explained in chinese.md; the gazetteer's first circuit may be another wall, or the li differs | disputed |
| suzhou.land-gates-qing / suzhou.gates-1229 | 6 land gates against 5 | Qing gazetteer against the 1229 stele; chinese.md says the stele needs checking | disputed |
| ahmedabad.gates / ahmedabad.gates-1855 | 18 (1879) against 12 (1855 plan title) | different dates and sources; neither report reconciles them | disputed |
| delhi.circuit / delhi.circuit-corpus; delhi.gates / delhi.gates-corpus | 8.85 against 6 km; 10 against 13-14 gates | gazetteer against the corpus-world's reading of a plan; a 6 km loop holds at most 286 ha, less than the 600 ha the corpus-world itself gives, so the 6 km figure does not fit (P^2/4pi check from verification-streets-walls.md Notes) | disputed |
| amsterdam.walled-area-1544 / amsterdam.walled-area-1544-corpus | 150-180 against c. 100 ha | two unverified secondary estimates | disputed |
| constantinople.theodosian-land-wall / -6-5 | 5.7 against 6.5 km | present length (Wikipedia) against the corpus-world's figure for 1422 | disputed; the difference is not explained |
| moscow.bely-gorod-circuit / -circuit-9; moscow.garden-ring-length / moscow.skorodom-circuit | 10 against 9 km; 16 against 15 km | two reports give different values for the same wall; 10 and 16 km lie outside the rounding of c. 9 and c. 15 km | disputed |
| benin-city.inner-moat-snippet / benin-city.inner-bank | 12 against 11 km | snippet against Connah's survey via the corpus-world; the same earthwork, and 11 and 12 km lie outside each other's rounding | disputed |
| venice.arsenale-wall-3-2 / venice.arsenale-wall-mile | 3.2 km against about a mile | Wikipedia against a web page | disputed |
| prague.new-town-area / prague.new-town-area-lead | 250 ha against 7.5 km2 | the same article; 7.5 km2 is far beyond what a 3.5 km wall encloses; 250 ha is also more than a closed 3.5 km loop holds (about 97 ha), which fits only because the New Town wall closes against the Old Town and the river rather than forming a full ring | disputed |
| dubrovnik.area / dubrovnik.circuit | 96.7 ha against a 1,940 m wall | a 1,940 m loop holds at most about 30 ha, so 96.7 ha covers more than the walled town | unverified; not usable as a walled area |
| Vienna ring road: walls-growth.md §8 against town-plan-time-series.md §7 (Vienna) | walls-growth says the Ring was built on the old wall; the time-series city block says on the glacis, "not on the wall line" | the time-series city block is the more specific statement | kept as a pattern note (§5), no fact |

## 4. Gaps

- **Intramural lane width:** one figure only (san-giovanni.intramural-road, 11.7 m). Newcastle
  and Norwich lanes and the Roman pomerium have no width.
- **Bastioned trace:** no bastion spacing, rampart width or ditch width; only the Vienna glacis
  and bastion counts (Vienna 12, Amsterdam 26, Lima 34, Mandalay 48, Hue 24, Pingyao 94).
- **Palisade and earthwork dimensions:** Benin "10 feet" (Dutch feet), nothing for timber walls
  in Europe; Bibracte, Hedeby and Cahokia give lengths only.
- **Gate widths:** Bibracte 20 m, Benin 5 feet; the Samarangana widths (9/8/7 hasta) are not
  converted in the report.
- **Tower size** (projection and width) and **gatehouse footprint:** not in any report.
- **A dataset of circuit length against area, or gates against roads:** none; walls-growth's
  gates-per-km figures are its own arithmetic on about twelve towns.
- **Fringe-belt width** outside a wall: not found.
- **Islamic-world wall lengths and thicknesses:** Jerusalem 4 km is the only length; areas for
  Cairo, Fez, Tunis are corpus-world estimates marked unverified.
- **Unconverted units:** listed in §1.
- **Verification coverage:** the verification report checked Conwy, Florence, Cologne, Paris,
  Nördlingen, Xi'an, York, Ávila and Kano only. 435 of 512 facts are unverified.

## 5. Patterns without numbers that matter for the generator

1. **Nested rings with radial roads running through all of them.** Old gate sites become squares
   or crossings on a ring street. Sources: walls-growth.md "Growth patterns" item 1 (Cologne,
   Paris, Florence, Moscow); town-plan-corpus.md §2.10 (Nördlingen inner ring readable as a ring
   street); town-plan-corpus-world.md §9.1 "Several concentric rings".
2. **Rings side by side, not nested.** Prague Old and New Town (walls-growth.md "Growth patterns"
   item 3); Fez el-Bali and el-Jdid, Guangzhou Old and New City with a cross wall, Madras fort and
   Black Town (town-plan-corpus-world.md §9.1 "Rings side by side"). The spec already allows this
   (two walls that do not list each other).
3. **Water closes the wall.** A D-shaped ring with its straight side on the river or shore
   (Dublin, Hedeby, Amsterdam 1544, Lima, Havana: town-plan-corpus-world.md §9.1 "D on water");
   a wall on the land side only (New Amsterdam, Galle, Quebec: same table; cliff-towns.md
   finding 6 "Cliffs replace walls"); Cádiz's coast-to-coast wall across the isthmus
   (peninsula-ports.md finding 1). Land walls are thicker than sea walls (peninsula-ports.md
   finding 1, cliff-towns.md finding 7). Sea gates outnumber land gates on peninsulas
   (peninsula-ports.md finding 2).
4. **Razed walls become ring roads, canals or street lines, with exceptions.** Moscow Boulevard
   and Garden Rings on the wall lines (town-plan-time-series.md §9 Moscow); Paris Grands
   Boulevards on the Charles V line and the Fermiers-Généraux boulevard ring (§4 Paris); London's
   wall survives as a line of streets and gate names (§5 London); Amsterdam's medieval wall
   became canal quays (§6 Amsterdam); Prague's moat became Na Příkopě (walls-growth.md item 8).
   Exceptions in the city blocks: Vienna's Ringstraße was laid on the glacis, not on the wall line
   (§7 Vienna, the block for the last pair of plans); Amsterdam's gates moved outward with each new wall (§6 Amsterdam);
   Timgad's founding wall was dismantled and built over (§13 Timgad); the Philip Augustus wall on
   the Paris right bank was absorbed into houses (§4 Paris). So the line survives as a road in
   some cities and as plot boundaries in others.
5. **A gate stands where a road meets the wall; gate count follows roads, not length.**
   city-foundation-and-growth.md §3 item 4 ("Gates follow roads"); walls-growth.md
   finding 2; Harar's five gates "matched the main roads into the town" (africa.md case 4).
   Gates can be blocked over time and leave road stubs (africa.md "What this adds", Djenné;
   chinese.md case 6, Suzhou).
6. **Walls often enclose empty land.** Siena, Nuremberg, Visby (town-plan-corpus.md §2.3, §2.9,
   §2.12), Kano and Hausa walls (walls-growth.md finding 6), Andalusi "amplias zonas vacías"
   (iberian.md case 8); against that, Chinese walls were "usually slightly larger than the
   built-up area" (chinese.md finding 3).
7. **Suburbs grow along the roads out of the gates.** walls-growth.md finding 10;
   city-foundation-and-growth.md §1 item 10; chinese.md case 3 (suburbs only at the gates on the
   main road).
8. **Citadel on the wall line, at an edge or a corner.** Conwy, Caernarfon (city-foundation-and-growth.md
   §4B case studies 3 and 4), Damascus, Dublin (town-plan-corpus.md §2.1, §2.15), and the corpus-world list of
   16 towns with "citadel on the edge" against 7 with "citadel in the middle" (§9.1).
9. **Gates on a hill side may be missing.** Huizhou has no moat where the hills are steep; a side
   against a steep hill may have no gate (chinese.md case 4 and "What this adds").

## 6. What this means for the spec

The spec proposal (`origin/m1/spec-proposal:docs/proposals/spec-v1.md` §2) gives walls `kind`
(palisade, stone, razed), `encloses`, `along_water` (wall or open), gates where roads cross
(F9), postern and water gates, and a citadel with `side` and `inside`. The generator still needs
the defaults below. Each question lists options, the facts, and my recommendation. Jeffrey
decides each.

**Q1. How much empty land does a wall enclose beyond its listed districts?**
- (a) None: the wall follows the outer edge of the enclosed districts with no margin. Facts:
  chinese.md finding 3 (walls "slightly larger than the built-up area"), china-county.circuit-median.
- (b) A rule-pack parameter `walled_to_built_ratio`, defined as walled area divided by
  built-up area, default 1.0. A value above 1.0 adds open land (gardens, fields) inside the
  wall until the walled area reaches that multiple of the built-up area; 1.0 adds none. Facts: hausa.walled-to-built (2-3), bristol.walled-area
  with bristol.settled-area, §5 pattern 6.
- (c) A fixed margin for every wall. No fact gives a margin in metres.
- Recommendation: (b). Its default 1.0 gives the same result as (a), and Kano, Siena and Visby stay reachable by a
  pack value without a new spec field.

**Q2. Tower spacing on a `stone` wall.**
- (a) 30 m. Facts: avila.tower-spacing (28.6-28.9 m).
- (b) A range of 40 to 60 m drawn once per wall from `stream(master_seed, "macro", wall id)`
  (ARCHITECTURE.md §5; the wall id comes from the spec). ARCHITECTURE.md §4 lists only
  `("macro", cell)` as the S1 stream key, so this option adds a second S1 key keyed by wall id. Facts:
  conwy.tower-spacing (46 m), arthashastra.tower-spacing (50.7 m),
  paris.philip-augustus-tower-spacing (60 m), constantinople.inner-land-wall-tower-spacing
  (typical curtain 40-60 m).
- (c) No towers in version 1.
- Recommendation: (b). Four independent sources from three cultures fall inside it; Ávila is
  the dense outlier and a rule pack can set 30 m.

**Q3. Drawn thickness of a `stone` wall.**
- (a) 2 m. Facts: conwy.wall-thickness 1.68 m, york.wall-thickness 1.8 m, vila-real.wall-thickness.
- (b) 3 m. Facts: avila.wall-thickness, paris.philip-augustus-wall-thickness, jaipur.wall-thickness.
- (c) 7 m. Facts: china-seats.wall-thickness-mean (7.26 m base), for a Chinese pack.
- Recommendation: (b) for the three medieval era presets, with (c) as the value a Chinese rule
  pack sets. A `palisade` has no thickness fact; Jeffrey sets that value.

**Q4. Does a `stone` wall get a ditch by default, and how wide?**
- (a) No ditch unless a rule pack adds one.
- (b) A ditch of 10 to 20 m on every land side, none along water. Facts: prague.old-town-moat-width
  (10 m), xian.moat-width (18 m), york.ditch-width (18.3 m), constantinople.moat-width (over 20 m),
  china-seats.moat-share (96 %).
- (c) (b) plus an outer road: San Giovanni has 21.9 m for moat and road together
  (san-giovanni.moat-and-outer-road).
- Recommendation: (b), the width drawn from the same per-wall stream as Q2 (b),
  `stream(master_seed, "macro", wall id)`. It matches European and Chinese
  figures alike and keeps water sides free, as Huizhou shows for steep sides (§5 pattern 9).

**Q5. May the generator's own streets cross a wall?**
- (a) No: only spec roads cross a wall; S3 streets stop at the wall. Gates then stay few.
  Facts: walls-growth.md finding 2 (0.3-4.0 gates per km), conwy.gates, york.gates,
  nuremberg.outer-gates, china-county.gates-4-share.
- (b) Yes, and each crossing becomes a postern.
- (c) Yes, up to a per-km cap from a rule pack.
- Recommendation: (a), with a sweep check that gates per km of wall stay inside 0.3 to 4.0.

**Q6. A lane just inside the wall.**
- (a) None by default; a rule pack may add one.
- (b) Always, 11.7 m wide. Facts: san-giovanni.intramural-road (one planned town).
- (c) Always, width set by the era preset; no fact gives a width for unplanned towns.
- Recommendation: (a). One measured case is too few to make it a default; walls-growth.md
  finding 5 shows the lane existed (Newcastle, Norwich) without a width.

**Q7. How is a `razed` wall drawn?**
- (a) A ring street on the old wall line, 20 to 40 m wide. Facts: moscow.garden-ring-width
  (22-43 m), san-giovanni.moat-and-outer-road (21.9 m for the strip a wall and moat occupied).
- (b) A street of normal main-street width on the line, with gate sites as squares.
- (c) A line of plot boundaries only, no street (Paris right bank, Timgad, §5 pattern 4).
- Recommendation: (a) as the default, with (c) available as a second kind later. The city
  blocks show both outcomes; (a) is the one a reader recognises on a map.

**Q8. Where does the citadel sit relative to its wall?**
- (a) On the wall line at the given `side`, sharing that stretch of wall, its area 1 to 10 ha.
  Facts: timgad.fort-area (1 ha), maastricht.castrum-length and castrum-width (170 x 90 m),
  durham.plateau-area (10 ha), §5 pattern 8.
- (b) Inside the wall, touching it at `side`, without sharing it.
- (c) At the centre (`side` ignored). Facts: moscow.kremlin-area (28 ha), beijing.forbidden-city-area.
- Recommendation: (a). It is the European and Near-Eastern majority in the corpora and matches
  the `side` field already in the spec; a centre citadel stays possible through a pinned
  `defence` landmark.

**Q9. A wall with `along_water: wall`: is the water side built like the land side?**
- (a) Same thickness and towers as the land side.
- (b) Thinner, about two thirds of the land-side thickness, with the same tower spacing. Facts:
  dubrovnik.land-wall-thickness (4-6 m) against dubrovnik.sea-wall-thickness (1.5-5 m);
  constantinople.inner-land-wall-thickness against the single sea wall
  (constantinople.golden-horn-wall-height 10 m against 12 m on land); the Golden Horn and Marmara
  walls carry 110 and 188 towers.
- (c) A thin line without towers.
- Recommendation: (b). The two-thirds ratio is my reading of the Dubrovnik midpoints
  (3.25 m against 5 m), not a source figure; Jeffrey decides whether that is close enough.

## 7. Repair round 2026-10-21

A sample check of 20 facts found estimates recorded as exact figures, years that belong to
another figure in the same report row, and the Paris heritage-body conflict recorded as a
correction. Fact format v2 (ADR-025) added `approximate`, `year_basis`, `year_approximate`,
`source_via` and `disputes`. Every one of the 512 facts was checked again against the passage
in its report. What changed:

- **`approximate: true`:** 175 facts, where the report says "about", "c.", "some" or a similar
  word. The note keeps the report's word. Every hectare figure from
  `town-plan-time-series.md` is approximate, because that report says so in its introduction.
- **Years:** 127 facts lost their years. 71 of them come from reports that give no date for
  the figure. The other 56 cite `settlement/verification-streets-walls.md`, whose tables give
  no dates; their building dates stay in the walls-growth facts for the same walls. 175 facts
  kept years with corrected values, mostly a plan date or building date in place of a span
  that mixed founding, building and demolition. A century is its full range.
- **`year_basis`:** set on all 341 facts that keep years: 119 plan_date, 93 built,
  40 described, 31 depicted, 29 founded, 18 measured, 7 ended, 3 estimated, 1 census.
  `depicted` is the state a plan or reconstruction shows when it was drawn later: the
  corpus-world sections dated by that state (Ur, Babylon, Nineveh, Kahun, Hattusa, Tiryns,
  Dholavira, Pompeii 79, Hedeby, Trelleborg, Bibracte c. 100 BC, Shahjahanabad 1857 on the
  1860 plan), Tang Chang'an 7th-9th c. and Suttinger's Vienna "as in 1683". `valid_until` is a
  figure valid until the place was abandoned or demolished, with only `year_to`: the six
  Old Oyo facts ("to c. 1837") and `mexico-city.sacred-precinct-side` (demolished 1521).
- **`year_approximate: true`:** 64 facts ("c. 1300", "late 1st century", "early 1600s").
- **`source_via`:** 12 facts now cite the original source through the work the report used
  (Ahmedabad via the Bombay gazetteer, Jaipur via Wikipedia, Xi'an via Wikipedia, Ávila via
  El Español, Benin via the corpus-world, Nineveh via the corpus-world). 15 facts changed
  source in all, including the five Timgad fort facts, which now cite French Wikipedia as the
  report names it. 9 source entries were added.
- **`disputes`:** 36 facts in 18 pairs; each side lists the other.
- **Status changes (18):** Paris Philip Augustus area and gates went from corrected and
  superseded to disputed (4 facts), because the verification report calls both a conflict.
  Constantinople land wall, the two Moscow rings and Benin inner earthwork became disputed
  (8 facts). Both Amsterdam 1544 facts (c. 100 ha against 150-180 ha) became disputed, as §3
  already listed them (2 facts). Xi'an circuit (verified) became disputed against the
  gazetteer figure that already disputed it (1 fact). The three Kano circuit facts went from disputed to unverified, because
  "14km radius" is not a circuit length and the 14-24 km range contains 17.7 km.
- **Removed:** none. No fact came from the old §14 and §15 of `town-plan-time-series.md`.
