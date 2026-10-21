# Cross-analysis: plots and blocks

> Written 2026-10-20 by Claude for the ADR-025 cross-analysis. This document turns research
> into proposed decisions, so it follows the CLAUDE.md rule "Explicit over implied" in full
> (ADR-023). Quoted source uncertainty keeps the source's own words. Nothing here is binding
> until Jeffrey answers the questions in section 5 and a DECISIONS.md entry records the answer.

The facts are in `data/research/facts/plots.yaml` (245 facts), with their sources in
`data/research/sources/plots.yaml`, historical units in `data/research/units/plots.yaml` and
new quantities in `data/research/quantities/plots.yaml`. Every figure below names the fact id
it rests on; metres are computed from the unit factor in the units file. Prescribed sizes are
stored in their historical unit (perch, Fuss, ken, jo, vara, pie, palmo, estado, braza, mu)
so the conversion stays visible.

Jargon used below, explained once:
- **Plot** (burgage, lot, solar, Hofstatte, ayral, chō-wari plot): the land one household
  holds and builds on. **Frontage** is its width along the street, **depth** its length back
  from the street.
- **Block**: land enclosed by streets on all sides; in the ARCHITECTURE.md glossary, a face of
  the street graph in S4. A **parcel** is a subdivision of a block in S4.
- **Building coverage**: the share of an area covered by buildings.
- **Burgage cycle** (Conzen): a plot starts with one house on the street front, its back fills
  with buildings over decades, then it is cleared and rebuilt.
- **Binary block** (Séraphin): a long block split down its middle by a gap under 0.5 m, with
  shallow houses back to back.
- **Kaishochi**: the empty square left in the middle of a Japanese castle-town block.

## 1. Scope

In scope: plot frontage and depth, plot area, plot proportions, standard plot units in
charters and colonial ordinances, block sizes and shapes, plots per block, gaps between plots,
building coverage, ordinary house footprints, storeys and building height caps, and the
subdivision of plots over time.

Reports read: `settlement/parcels-density.md`, `settlement/traditions.md`,
`settlement/timber-ports.md`, `settlement/social-structure.md`,
`settlement/growth-generators.md`, `city-foundation-and-growth.md`, the seven reports and the
README under `sources-by-language/`, `town-plan-corpus.md`, `town-plan-corpus-world.md`, and
`settlement/verification-streets-walls.md` for statuses.

Out of scope here and left to other topics: population density, street widths, market square
and plaza sizes, wall circuits, landmark footprints. `social-structure.md` and
`growth-generators.md` hold no plot or block figures; they contribute only the wealth
gradient ("rich frontage on main streets, poor in back courts") and the observation that
generators lay out plots after streets.

Status counts: 3 verified, 14 disputed, 228 unverified, 0 corrected, 0 superseded. Only
`verification-streets-walls.md` checks a plot or block figure (Heian-kyō block, Chang'an
Zhengpingfang). Every other fact is unverified or disputed.

Repair round 2026-10-21. A sample check found errors in this topic, and fact format v2
(ADR-025) added five fields. Every one of the 245 facts was checked against the passage its
`report` names. Changes:
- `approximate: true` on 70 facts whose report gives the figure with "about", "c.", "nearly"
  or a similar word; the word stays in the note.
- Years: 13 facts lost their years because the report gives no date for that figure (for
  example Roman height caps named by emperor, the Caernarfon burgage count, whose 1283-92 is
  the date of the walled town, and the Rabastens charter ratio, whose 1306 is the date of
  the market square). 79 facts had their years corrected or completed: centuries now span
  their full range (13th century = 1201-1300), dynasty dates the report does not give were
  replaced by the report's own dates (Chang'an wards 7th-9th c., not 618-907), and figures
  read off a plan carry the plan's date (Caernarfon 1610, not 1283-1610).
- `year_basis` on all 187 facts that keep years. `plan_date` is the date of a plan, map,
  view, charter, ordinance or building code the figure comes from or that set it; `measured`
  a survey (Cairo 1798-1801, Fez 1912, Acoma 1934, Yakage 2017); `built` the period in which
  the measured thing stood (excavated phases); `depicted` (9 facts) the moment or period a
  plan or reconstruction shows (Hedeby c. 900-1050, Shahjahanabad 1857, Pompeii 79 CE,
  Kahun c. 1895 BCE, Tang Chang'an wards 7th-9th c.); `ended` (0 facts) a figure valid until the place was abandoned;
  `founded` the founding year of a planned town whose grid the figure describes (Monpazier,
  Lima, Heian-kyō, Heijō-kyō, Priene, Timgad, Dura-Europos); `described` the date of a
  written account (Caillié, Dapper, the Ahmedabad gazetteer, Vilhena, Ducoloner).
- `year_approximate: true` on 32 facts dated "c.", "early" or "around".
- `source_via` on 14 facts the report cites through another work: Grenville and Hoskins
  through Dodson and Ross, Conzen through the Alnwick Civic Society, the 1534 Cusco Cabildo
  through Tord Velasco, Vilhena through the Dossiê Salvador, Millon and Sugiyama through
  Florescano, the Nagoya castle history through the Nagoya City Library, and the Reizei-chō
  town record through Watanabe. Seven sources were added for the originals.
- `disputes` on 16 facts, 8 pairs, each listing the other (conflicts C1 to C6 and C8 below).
- Status: the four Monpazier facts and the two Guipúzcoa readings went from disputed to
  unverified (C7 and C8 below).
- `plots.acoma.house-block-depth` is renamed `plots.acoma.house-block-width`: the report
  calls the 10-18 m short side "deep", and the quantity is the block's short side.
- `plots.nagoya.kaisho-side-library` is stored as 36-39 m: the report does not name the ken
  type of the 20 ken.
- No fact cites the summary sections of `town-plan-time-series.md`, so none was removed.

Two kinds of evidence differ in weight, and the fact notes say which one a fact is:
- **Measured or prescribed**: a charter, ordinance, cadastre, excavation or survey figure
  (for example `plots.freiburg.hofstatte-width`, `plots.indies.peonia-width`,
  `plots.figeac.block-width`).
- **Read off a historic picture**: the two town-plan corpora read block sizes off bird's-eye
  views and plans, which they call "indicative" (for example `plots.nuremberg.block-side`).

## 2. The picture

### 2.1 Plot frontage and depth by culture and era

All values in metres unless stated.

| Culture, region | Era | Frontage | Depth | Fact ids |
|---|---|---|---|---|
| Norse, Hiberno-Norse emporia | early medieval (8th-11th c.) | 5-8 core; 8-16 for wider shore and early plots | 20-47 | `plots.sigtuna.plot-width`, `plots.ribe.plot-width`, `plots.trondheim.central-plot-width`, `plots.oslo.plot-width`, `plots.bergen.shore-plot-width`, `plots.hedeby.plot-width`, `plots.york-coppergate.plot-width`, `plots.sigtuna.plot-depth-vsm`, `plots.sigtuna.plot-depth-mh`, `plots.york-coppergate.plot-depth`, `plots.dorestad.plot-depth` |
| English charter burgage | high medieval (post-1066) | 15.1-17.6 (3-3.5 perches) | 60.4 (12 perches) | `plots.england.charter-burgage-width`, `plots.england.charter-burgage-depth`, `plots.england.charter-burgage-area` |
| English planted towns | 1225-1228; Caernarfon read off a 1610 plan | 10-24 | 20-121 | `plots.salisbury.plot-width`, `plots.salisbury.plot-depth`, `plots.sherborne.plot-width`, `plots.sherborne.plot-depth`, `plots.caernarfon.burgage-width`, `plots.caernarfon.burgage-depth` |
| English surviving pattern | medieval, surveyed 1960s | 8.5 unit and its fractions and multiples | depth : frontage about 6 | `plots.alnwick.frontage-unit`, `plots.northumberland.plot-aspect`, `plots.burton-in-lonsdale.burgage-field-width` |
| German founded towns | high medieval (12th c.; 1218 town law) | 16.2 by charter; 14-18 measured; later split to about 8 | 32.4 by charter; 32-40 measured | `plots.freiburg.hofstatte-width`, `plots.freiburg.hofstatte-depth`, `plots.freiburg.measured-width`, `plots.freiburg.measured-depth`, `plots.stollhofen.plot-area` |
| Occitan bastides | high medieval (13th-14th c.) | 4-10, mean 8 | mean 24 | `plots.bastides.frontage`, `plots.bastides.mean-width`, `plots.bastides.mean-depth`, `plots.bastides.plot-area`, `plots.monpazier.plot-width`, `plots.monpazier.plot-depth` |
| Castilian, Aragonese, Basque new towns | high and late medieval | 4.2-10 | 12.5-25 | `plots.alcaniz.lot-width`, `plots.laguardia.lot-width`, `plots.labraza.lot-width`, `plots.guipuzcoa.lot-width-reading-a`, `plots.guipuzcoa.lot-width-reading-b`, `plots.tordehumos.lot-width`, `plots.alcaniz.lot-depth`, `plots.laguardia.lot-depth`, `plots.labraza.lot-depth`, `plots.tordehumos.lot-depth` |
| Portuguese long lot | early modern (early 16th c. to 1774) | 5.3-6.6 | 11-13.2 | `plots.lisbon-bairro-alto.lot-width`, `plots.lisbon-bairro-alto.lot-depth`, `plots.vila-real-santo-antonio.lot-width`, `plots.vila-real-santo-antonio.lot-depth` |
| Dutch canal ring | early modern (1613) | 6.2 working canal; 8.5 rich canal | not found | `plots.amsterdam.prinsengracht-frontage`, `plots.amsterdam.herengracht-frontage` |
| Spanish Indies ordinances | early modern (1573) | 13.9 (peonía) and 27.9 (caballería) | 27.9 and 55.7 | `plots.indies.peonia-width`, `plots.indies.peonia-depth`, `plots.indies.caballeria-width`, `plots.indies.caballeria-depth` |
| Spanish colonial lots in practice | early modern | quarter block, about 50-60 a side (the report's reading) | same | `plots.lima.lots-per-block`, `plots.buenos-aires.lots-per-block`, `plots.cusco.solar-side` |
| Japanese castle towns | early modern (1574-1700) | 9-31 assessment band; one Kanazawa plot 27 | 20 ken (39.4) became the norm; 12-35 ken (22-64) tried | `plots.castle-towns.duty-frontage`, `plots.kanazawa.merchant-plot-frontage`, `plots.edo.plot-depth`, `plots.osaka-senba.plot-depth`, `plots.hideyoshi-towns.plot-depth`, `plots.kanazawa.merchant-plot-depth`, `plots.kanazawa.street-plot-depth`, `plots.nagoya.corner-plot-depth` |
| Kyoto townhouse (machiya) | early modern to Meiji | 5.4-6; tax unit 3 ken (5.45) | 20 or more; one plot cut from 61 to 35 in the 1590s | `plots.kyoto.machiya-frontage`, `plots.kyoto.nokiyaku-frontage`, `plots.kyoto.machiya-depth`, `plots.kyoto-reizei.plot-depth-before`, `plots.kyoto-reizei.plot-depth-after`, `plots.kyoto-1590.plot-depth` |
| Vietnamese guild streets | 1873 | 3-4 | not found | `plots.hanoi.tube-house-frontage` |
| British colonial grid | 1734 plan (colony founded 1733) | 18.3 | 27.4 | `plots.savannah.lot-width`, `plots.savannah.lot-depth` |
| Indian treatise | undated (Manasara) | 5.5 (3 rods) on the main street | 2-3 times the frontage | `plots.manasara.main-street-plot-width`, `plots.manasara.main-street-plot-aspect` |
| Ancient Egypt | c. 1895 BCE | 10 per small-house unit | 20 | `plots.kahun.small-house-unit-width`, `plots.kahun.small-house-unit-depth` |
| Game source (MDME) | none | 5 (1 perch); 3-7.3 range | "very deep" | `plots.mdme.shop-frontage`, `plots.mdme.frontage-unit-range` |

Plot proportions (depth divided by frontage): about 6 in Northumberland
(`plots.northumberland.plot-aspect`), 4 in the English charter standard (60.4 / 15.1),
2 in Freiburg (`plots.freiburg.hofstatte-depth` / `plots.freiburg.hofstatte-width`), 3 in
bastides (24 / 8, and `plots.rabastens.charter-plot-aspect`), 2-3 in the Manasara
(`plots.manasara.main-street-plot-aspect`), 2 in the Portuguese and Spanish ordinance lots,
1.67 in Bern (`plots.bern.hofstatte-aspect`, unverified). The ratios without a fact id are my
arithmetic on the cited facts.

### 2.2 Plot area

| Culture | Plot type | Area (m²) | Fact ids |
|---|---|---|---|
| Sahelian (Djenné, 1990s) | dense-quarter compound | about 100 | `plots.djenne.compound-area` |
| Aztec (Tenochtitlan, 1519) | residential compound | 100-500 | `plots.tenochtitlan.residential-plot-area` |
| Occitan bastide | house plot | 150-200 | `plots.bastides.plot-area` |
| Japanese (Heian-kyō, 794) | smallest house plot, 1/32 block | 460 | `plots.heian-kyo.henushi-area`, `plots.heian-kyo.henushi-block-fraction` |
| German founded town | charter plot | 525-530 | `plots.stollhofen.plot-area`, `plots.freiburg.hofstatte-width`, `plots.freiburg.hofstatte-depth` |
| English charter burgage | burgage | about 1,000 (a quarter acre) | `plots.england.charter-burgage-area` |
| Occitan bastide | garden plot outside the town | 1,400-1,422 | `plots.grenade.garden-plot-area` |
| Ancient Egypt (Kahun) | mansion | 2,400 | `plots.kahun.mansion-area` |
| Chinese (Yuan Dadu, 1285) | household grant of 8 mu | 4,670 or 5,340 (disputed) | `plots.dadu.grant-yuan-mu`, `plots.dadu.grant-modern-mu` |
| Japanese (Edo, 1680) | samurai and daimyo estates | 10,000-100,000 | `plots.edo-1680.samurai-estate-area` |
| Japanese (Heijō-kyō, 8th c.) | plot by court rank | 1/32 to 12 blocks | `plots.heijo-kyo.rank-plot-fraction` |

### 2.3 Block size and shape

Short side across the plots ("width") and long side ("length"), in metres.

| Block type | Culture, era | Width | Length | Fact ids |
|---|---|---|---|---|
| Binary block (one plot depth each side of a back gap) | Occitan, 13th c. | 11.5-17 | 50-70 | `plots.figeac.block-width`, `plots.figeac.block-length`, `plots.luzech.block-width`, `plots.luzech.block-length`, `plots.mercues.block-width`, `plots.caylus.block-width`, `plots.villefranche-de-rouergue.block-width` |
| Binary block, larger | Basque, medieval | 30 | over 80 | `plots.bilbao.block-width`, `plots.bilbao.block-length` |
| Tenement strip to the water | Norse, 1581 | 10-15 | 100 | `plots.bergen.tenement-strip-width`, `plots.bergen.tenement-strip-length` |
| Planted grid, small | Castilian new towns | 25-59 | 33 (Alcañiz) | `plots.castilian-new-towns.block-width`, `plots.vertavillo.block-width`, `plots.penaflor.block-width`, `plots.alcaniz.block-width`, `plots.alcaniz.block-length` |
| Planted grid, medium | Occitan bastides, English and French planted towns | 40-60 | 57.5-100 | `plots.mirande.block-side`, `plots.villeneuve-sur-lot.block-width`, `plots.villeneuve-sur-lot.block-length`, `plots.monpazier.block-width`, `plots.monpazier.block-length`, `plots.caernarfon.block-side`, `plots.galway.block-side`, `plots.neuf-brisach.block-side` |
| Organic medieval | German, Italian, Castilian, 1500-1650 | 15-80 (single "side" figure) | | `plots.toledo.block-side`, `plots.nuremberg.block-side`, `plots.nordlingen.block-side`, `plots.venice.block-side`, `plots.amsterdam-1544.block-width` |
| Organic Islamic | Ottoman Arab, Moroccan | 30-150; Aleppo 45 x 125 | | `plots.fez-1912.block-side`, `plots.cairo-1809.block-side`, `plots.aleppo.block-width`, `plots.aleppo.block-length` |
| Compound blocks | West Africa, Andes, Mesoamerica | 20-100 | | `plots.kano.compound-side`, `plots.kumasi.block-side`, `plots.cusco.kancha-side`, `plots.teotihuacan.compound-side` |
| Japanese castle-town module | early modern | 40, 50 or 60 ken square (79, 98.5, 118) | rectangles 40 x 60 ken, 60 x 120 ken | `plots.osaka-senba.block-side`, `plots.sunpu.block-side`, `plots.nagoya.block-side`, `plots.edo.block-side`, `plots.osaka-uemachi.block-width`, `plots.osaka-uemachi.block-length`, `plots.nagahama.block-width`, `plots.nagahama.block-length`, `plots.hakata.block-width`, `plots.hakata.block-length`, `plots.nagasaki.block-side` |
| Japanese grid capital | early medieval | 120 square (verified) | super-block 532 | `plots.heian-kyo.block-side`, `plots.heijo-kyo.superblock-side`, `plots.kyoto-1590.block-split-width` |
| Spanish colonial grid | early modern | 84-125 | 115-208 | `plots.puebla.block-width`, `plots.puebla.block-length`, `plots.mexico.block-width`, `plots.mexico.block-length`, `plots.santiago.block-side`, `plots.lima.block-side`, `plots.buenos-aires.block-side`, `plots.havana.block-width`, `plots.havana.block-length` |
| Colonial grids, other powers | French, British, Dutch, Mormon, 1667-1870 | 97-201 | | `plots.new-orleans.block-side`, `plots.batavia.block-width`, `plots.batavia.block-length`, `plots.salt-lake-city.block-side` |
| Chinese ward and lane blocks | Tang to Qing | 60-838 | 200-1,125 | `plots.changan.ward-ns`, `plots.changan.ward-ew`, `plots.changan.zhengpingfang-width`, `plots.changan.zhengpingfang-length`, `plots.suzhou.canal-block-width`, `plots.suzhou.canal-block-length`, `plots.dadu.xisi-hutong-length` |
| South Asian | Mughal, Tamil, 1857-1906 | 60-200 | | `plots.madurai.ring-block`, `plots.shahjahanabad.block-side` |
| Ancient grids | Greek, Roman, Hellenistic | 20-42 | 47-90 | `plots.timgad.block-side`, `plots.priene.block-width`, `plots.priene.block-length`, `plots.miletus.north-block-width`, `plots.miletus.south-block-width`, `plots.pompeii.regio-vi-block-width`, `plots.pompeii.regio-vi-block-length`, `plots.dura-europos.block-width`, `plots.dura-europos.block-length` |

Plots per block: 2 Inca kanchas (`plots.ollantaytambo.kanchas-per-block`), 4 houses in Priene
(`plots.priene.houses-per-block`), 4 quarter-block lots in Lima and Buenos Aires
(`plots.lima.lots-per-block`, `plots.buenos-aires.lots-per-block`), 8 lots in Puebla
(`plots.puebla.lots-per-block`), 10 lots per Savannah tything
(`plots.savannah.lots-per-tything`), 5-60 houses per Ahmedabad pol
(`plots.ahmedabad.houses-per-pol`). Plot counts per town: about 60-70 burgages in Caernarfon
(`plots.caernarfon.burgage-count`), about 180 in Sigtuna (`plots.sigtuna.plot-count`), 3,000
offered and about a third taken in Grenade (`plots.grenade.house-plot-count`).

Gaps and courtyards inside blocks: back gap under 0.5 m in binary blocks
(`plots.figeac.back-gap`), 0.3 m andrones in Monpazier (`plots.monpazier.andron-width`), an
empty central square of 10 or 20 ken in Japanese castle-town blocks
(`plots.edo.kaishochi-side`, `plots.nagoya.kaisho-side-abe`,
`plots.nagoya.kaisho-side-library`). Lane spacing along a street: closes every 10-20 m in
Edinburgh (`plots.edinburgh.close-spacing`), hutongs every 70-80 m in Beijing
(`plots.beijing.hutong-spacing`, `plots.beijing-1750.hutong-spacing`).

### 2.4 By district function and wealth

| Distinction | Evidence | Fact ids |
|---|---|---|
| Rich street versus working street | Amsterdam: 8.5 m on the Herengracht, 6.2 m on the warehouse canal | `plots.amsterdam.herengracht-frontage`, `plots.amsterdam.prinsengracht-frontage` |
| Rank sets plot size | Spanish Indies: horseman's lot four times a foot soldier's; Heijō-kyō: 1/32 to 12 blocks by court rank; Edo: samurai estates 1-10 ha | `plots.indies.caballeria-width`, `plots.indies.peonia-width`, `plots.heijo-kyo.rank-plot-fraction`, `plots.edo-1680.samurai-estate-area` |
| Main street versus lane | Occitan binary blocks: head houses 6-7.5 m wide and 12-20 m deep on the main street, row houses 4.5-7.5 m wide and about 6-7.5 m deep on the lanes | `plots.figeac.head-house-width`, `plots.figeac.head-house-length`, `plots.figeac.row-house-width`, `plots.figeac.row-house-depth`, `plots.villefranche-de-rouergue.head-house-width`, `plots.villefranche-de-rouergue.head-house-length`, `plots.villefranche-de-rouergue.house-frontage`, `plots.villefranche-de-rouergue.house-depth` |
| Corner plots | Nagoya corner plots 15 ken deep against 20 ken elsewhere | `plots.nagoya.corner-plot-depth`, `plots.nagoya.block-side` |
| Harbour districts | plots run from the street to the water, Bergen tenements 100 m deep, Bergen shore plots 11.5-16 m wide; Suzhou canal blocks back onto water | `plots.bergen.tenement-strip-length`, `plots.bergen.shore-plot-width`, `plots.suzhou.canal-block-width` |
| Dense poor quarters | Djenné 100 m² compound; Leiden weavers' houses 22 m²; Birka workshops at most 20 m² | `plots.djenne.compound-area`, `plots.leiden.weaver-house-area`, `plots.birka.workshop-area` |
| Elite versus commoner in one plan | Kahun: small-house units 10 x 20 m against 2,400 m² mansions | `plots.kahun.small-house-unit-width`, `plots.kahun.mansion-area` |

Florentine new towns grade lot depth down from the main street toward the walls; the sources
disagree on whether lot width also falls (continental.md). No figure is attached, so no fact
records it.

### 2.5 Buildings on the plot

| Item | Values | Fact ids |
|---|---|---|
| Building coverage | one figure: 46.23% built in Mexico City's 249.9 ha centre in 1794, a whole district including streets | `plots.mexico-1794.building-coverage` |
| Early medieval house on its plot | 5-10 m long, 4.5-7 m wide; one per plot near the street; Hedeby hall houses 17-17.5 m long | `plots.dublin-fishamble.house-length`, `plots.dublin-fishamble.house-width`, `plots.dublin-winetavern.house-length`, `plots.york-coppergate.building-length`, `plots.truso.building-length`, `plots.truso.building-width`, `plots.hedeby.hall-house-width`, `plots.hedeby.hall-house-length` |
| High medieval town house | 6 x 9 m stone house (Freiburg); row houses 4.5-7.5 m wide, 6-7.5 m deep (Occitan) | `plots.freiburg.stone-house-width`, `plots.freiburg.stone-house-length`, plus the Occitan ids in 2.4 |
| Courtyard houses | Andalusi 88-94 m² (Valencia) | `plots.valencia-boatella.house-area` |
| Smallest dwellings | 2.7 x 3.6 m (Edo); 3.3 x 4.4 m artisan house (Braga); 22 m² (Leiden) | `plots.edo.smallest-dwelling-width`, `plots.edo.smallest-dwelling-length`, `plots.braga.artisan-house-width`, `plots.leiden.weaver-house-area` |
| Roof orientation | under 3 ken (5.45 m) frontage nearly 80% gable to the street; 4 ken (7.27 m) and over, over 80% eave to the street | `plots.yakage.roof-frontage-threshold` |
| Storeys | 1 (Timbuktu, Benin), 1-2 (Agadez), 2 (Djenné, Isfahan shops), 1-3 (Acoma; MDME game source), 3-4 (Salvador lower town c. 1800), up to 9 (Roman insulae) | `plots.timbuktu.storeys`, `plots.benin.storeys`, `plots.agadez.storeys`, `plots.djenne.storeys`, `plots.isfahan.shop-storeys`, `plots.acoma.storeys`, `plots.mdme.storeys`, `plots.salvador.lower-town-storeys`, `plots.rome.insula-storeys` |
| Height caps | Rome 20.7 m then 17.8 m; Istanbul 1848 timber 10.5 m, masonry 15 m; Istanbul 1863 timber 9.5 m, masonry 13.6 m; Agadez houses 3-6 m | `plots.rome.height-cap-augustus`, `plots.rome.height-cap-trajan`, `plots.istanbul-1848.height-cap-timber`, `plots.istanbul-1848.height-cap-masonry`, `plots.istanbul-1863.height-cap-timber`, `plots.istanbul-1863.height-cap-masonry`, `plots.agadez.house-height` |

### 2.6 Subdivision over time

The reports agree on direction: plots start large and are split, never merged in bulk.
Freiburg's 16.2 m charter plots were split from about 1200 (`plots.freiburg.measured-width`);
Trondheim's early riverside plots "were later divided" (`plots.trondheim.central-plot-width`);
Buenos Aires quarter-block lots were subdivided by 1744 (`plots.buenos-aires.lots-per-block`);
Kyoto's 1590 lane cut a 31-ken plot to 18 ken (`plots.kyoto-reizei.plot-depth-before`,
`plots.kyoto-reizei.plot-depth-after`); Grenade filled about a third of its planned plots
(`plots.grenade.house-plot-count`). Alnwick allowed "only a limited amount of dividing and
combining" (parcels-density.md, no figure).

## 3. Conflicts

| # | Competing fact ids | What differs | Reason | Status |
|---|---|---|---|---|
| C1 | `plots.edo.block-side` versus `plots.edo-1680.block-side` | 60 kyō-ma ken (about 118 m) square against "about 60 x 60 m (one cho)" | The world corpus equates one chō with 60 m; japanese.md gives one chō as 60 ken, about 109 m (standard ken) or 118 m (kyō-ma). The corpus figure is the corpus author's reading of a 1680 woodblock. | disputed; the Japanese-language source is the stronger one, and only a verification report can mark the corpus figure corrected |
| C2 | `plots.osaka-senba.block-side` versus `plots.osaka-1863.block-side` | 40 ken (about 79 m) against "40 m blocks" | The corpus gives a one-line note without a source for the figure. Reading 40 ken as 40 m would produce it; I do not know whether that happened. | disputed |
| C3 | `plots.mexico.block-width`, `plots.mexico.block-length` versus `plots.mexico-1628.block-width`, `plots.mexico-1628.block-length` | 150 x 250 varas (124.5 x 207.5 m, Rena 2019) against "about 90 x 180 m (unverified)" | The corpus figure is marked unverified by its own author and read off a 1628 view | disputed |
| C4 | `plots.dadu.grant-modern-mu` versus `plots.dadu.grant-yuan-mu` | 8 mu = about 5,340 m² against about 4,670 m² | Unit factor: modern mu (667 m²) against a Yuan mu of 240 square bu (584 m², Claude's calculation, not verified). The reports also date the grant differently: parcels-density.md to the 1270s, chinese.md to 1285 | disputed |
| C5 | `plots.sigtuna.plot-depth-vsm` versus `plots.sigtuna.plot-depth-mh` | 30-40 m against 20-40 m | Two popular sources; neither cites a survey | disputed |
| C6 | `plots.nagoya.kaisho-side-abe` versus `plots.nagoya.kaisho-side-library` | central open square 10 ken against 20 ken | Abe and Shinohara's table against the Nagoya castle history quoted by the city library; japanese.md says "I do not know which is right" | disputed |
| C7 | `plots.monpazier.plot-width`, `plots.monpazier.plot-depth`, `plots.monpazier.block-width`, `plots.monpazier.block-length` versus `plots.bastides.mean-depth` and continental.md "Not found" | Monpazier 8 x 20 m plots and 45 x 100 m blocks against a bastide mean of 8 x 24 m; continental.md could not confirm popular Monpazier figures (8 x 20 m plots, 40 x 80 m blocks) | The corpus figures come from an aerial photograph and a small Viollet-le-Duc plan; no scholarly Monpazier measurement was found. continental.md names the popular figures without a source, so they are not a competing figure; the bastide mean of 24 m is a mean over many towns that Gérard says varies by place, so it measures something other than Monpazier's plots | unverified; changed from disputed in the 2026-10-21 repair round; no `disputes` |
| C8 | `plots.guipuzcoa.lot-width-reading-a` versus `plots.guipuzcoa.lot-width-reading-b` | 6 brazas = 10.08 m or 8.0 m | Sáinz Guerra gives two readings of the braza (1.68 m or 1.333 m) | unverified, with `disputes` linking the two readings; changed from disputed in the 2026-10-21 repair round, because one source giving two readings is not two sources disagreeing |
| C9 | `plots.beijing.hutong-spacing` versus chinese.md "Not found" | about 76 m (Yang 2021) against "the claim of hutong spacing of about 77 m / 44 bu was not found anywhere" | chinese.md looked for a Chinese-language source and found none; `plots.beijing-1750.hutong-spacing` (70-80 m) is a map reading that agrees with Yang | unverified; not marked disputed, because the absence of a source is not a competing figure |
| C10 | `plots.teotihuacan.compound-side` versus town-plan-corpus-world.md | 30-100 m (Florescano) against "50-60 m square" typical | The corpus value lies inside Florescano's range; it is a typical value, not a contradiction | unverified; no conflict |
| C11 | `plots.kyoto.machiya-frontage` versus the frontage-tax story | 5.4-6 m typical frontage, unverified; the folk explanation (Hideyoshi taxed frontage) is contested in japanese.md, which finds Kyoto taxed per house and only early Meiji used 3 ken of frontage (`plots.kyoto.nokiyaku-frontage`) | A cause, not a figure, is disputed; the figures agree on about 5.4 m | unverified |
| C12 | `plots.alnwick.frontage-unit` and `plots.england.charter-burgage-width` | 8.5 m surviving unit against 15-17.6 m charter width | Not a contradiction: Alnwick's unit includes "fractions and multiples", and charter plots were split (Freiburg shows the same process). Both stay unverified. | unverified; no conflict |

## 4. Gaps

1. **Building coverage per plot or block.** No report gives a coverage ratio for a plot or a
   block in any culture (parcels-density.md "Not found"). The only coverage figure is one
   district (`plots.mexico-1794.building-coverage`).
2. **Plot depth in Amsterdam, Lübeck, Venice and Ackerbürgerstädte** (continental.md "Not
   found").
3. **Islamic courtyard-house plot sizes** for Fez, Tunis, Damascus, Isfahan or Yazd
   (islamic.md and parcels-density.md "Not found"). Only house areas in one Andalusi suburb
   (`plots.valencia-boatella.house-area`).
4. **Unit factors**: the Gascon rase (Rabastens), the Bern foot, the Zhou bu. Those plots are
   stored as ratios only (`plots.rabastens.charter-plot-aspect`, `plots.bern.hofstatte-aspect`).
5. **Storeys of ordinary medieval European town houses from a scholarly source.** The only
   European figure is the MDME game source (`plots.mdme.storeys`).
6. **Terre nuove lot sizes in braccia**, Monpazier from a scholarly source, Ming-Qing
   siheyuan lot sizes, Kyoto and Kanazawa average townhouse frontages, Jaipur chowkri sizes.
7. **Households per plot.** No source gives it (parcels-density.md); it links plot counts to
   population and stays open.
8. **Verification.** No verification report checks a plot frontage, plot depth or house
   figure. Every default in section 5 rests on unverified facts.

## 5. What this means for the spec and for stages S4 and S5

Context from `docs/proposals/spec-v1.md` and the ARCHITECTURE.md stage table on
`m1/spec-proposal`: the spec has `presets.era` (early_medieval, high_medieval, late_medieval),
districts with a `function` (eleven values) and `wealth` (poor, middling, rich), and rule
packs. S3 makes tertiary streets per block; S4 makes blocks and parcels (subdivision method
is an open decision for milestone 6); S5 makes building footprints from parcels and district
type. Calibration cities are Truro, Arcachon and Bearna, all late-medieval European types.

Assumption: the defaults below go into the default rule pack as data, not into the schema,
following spec-v1 §3 ("This table is data in the default rule pack"). Jeffrey vetoes this
assumption by answering question 1 with option (c).

Assumption: the default rule pack targets the three medieval eras of the era enum and
north-west European towns, because the calibration cities are of that type. Non-European plot
models from section 2 belong in later culture packs.

Each question lists two or three options, the fact ids each option rests on, and my
recommendation. Jeffrey decides.

**Question 1. Where do plot and block defaults live?**
- (a) In the default rule pack as data, one table per era and district function, overridable
  by any pack. Rests on the spread across cultures in 2.1 and 2.3 (for example
  `plots.freiburg.hofstatte-width` against `plots.lisbon-bairro-alto.lot-width`).
- (b) In the default rule pack, and also as an optional spec field `presets.plot_unit` (width
  x depth in a named unit), because charters wrote the plot into the town law. Rests on
  `plots.freiburg.hofstatte-width`, `plots.england.charter-burgage-width`,
  `plots.indies.peonia-width`, `plots.laguardia.lot-width`.
- (c) In core code as constants.
- Recommendation: (a). The spec stays small, and a GM who knows the town's charter plot writes
  a pack, not a schema field. (b) is a schema change that one PR can add later.

**Question 2. Which plot model does the default pack use?** A plot model fixes how plots
face streets and how deep they run.
- (a) One model, the burgage strip: narrow frontage, short side to the higher-ranked street,
  depth to the block's middle line. Rests on `plots.england.charter-burgage-width`,
  `plots.alnwick.frontage-unit`, `plots.northumberland.plot-aspect`,
  `plots.sigtuna.plot-width`, `plots.caernarfon.burgage-width`.
- (b) A closed enum of three models chosen per district by the pack: burgage strip, binary
  block (shallow houses back to back on a gap under 0.5 m), wide charter plot that later
  splits. Rests on the ids in (a) plus `plots.figeac.block-width`, `plots.figeac.back-gap`,
  `plots.freiburg.hofstatte-width`, `plots.freiburg.measured-width`.
- (c) No model; S4 cuts parcels from frontage and depth ranges alone.
- Recommendation: (a) for version 1, with the model name stored in the pack so that (b) is a
  later pack addition without a schema change. The three calibration cities are burgage-type
  towns; the binary block appears only in southern France and the Basque country.

**Question 3. What frontage range does S4 draw from in the default pack?**
- (a) One range for all districts, 5-10 m. Rests on `plots.sigtuna.plot-width`,
  `plots.alnwick.frontage-unit`, `plots.bastides.frontage`, `plots.york-coppergate.plot-width`,
  `plots.amsterdam.prinsengracht-frontage`.
- (b) By district wealth: poor 4-6 m, middling 6-9 m, rich 9-18 m. Rests on the poor end
  `plots.bastides.frontage`, `plots.alcaniz.lot-width`, `plots.york-coppergate.plot-width`,
  `plots.lisbon-bairro-alto.lot-width`; the middle `plots.alnwick.frontage-unit`,
  `plots.sigtuna.plot-width`, `plots.bastides.mean-width`; the rich end
  `plots.amsterdam.herengracht-frontage`, `plots.freiburg.measured-width`,
  `plots.england.charter-burgage-width`, and the wealth gradient in 2.4.
- (c) By era: early_medieval 7-16 m, high_medieval 8-18 m, late_medieval 4-9 m (after
  splitting). Rests on `plots.sigtuna.plot-width`, `plots.bergen.shore-plot-width`,
  `plots.oslo.plot-width`, `plots.freiburg.hofstatte-width`,
  `plots.england.charter-burgage-width`, `plots.freiburg.measured-width`.
- Recommendation: (b). The spec already carries `wealth` per district, and the
  Amsterdam pair is the clearest same-town evidence that frontage follows status. Era acts
  through splitting (question 6), not through a second frontage table. The band edges are my
  grouping of the cited facts, not a source's.

**Question 4. How is plot depth set?**
- (a) Depth runs from the street to the block's middle line (back-to-back plots), so block
  width sets depth. S4 cuts a back lane when the block is deeper than twice a maximum depth.
  Rests on the binary block (`plots.figeac.row-house-depth` is half the block width), the
  Kyoto lane cut (`plots.kyoto-reizei.plot-depth-before`, `plots.kyoto-reizei.plot-depth-after`)
  and the Japanese 20-ken norm with blocks of 40 ken (`plots.osaka-senba.plot-depth`,
  `plots.osaka-senba.block-side`).
- (b) Depth drawn from a range, 20-60 m, independent of the block; leftover land in the
  block's middle becomes yard. Rests on `plots.freiburg.measured-depth`,
  `plots.salisbury.plot-depth`, `plots.england.charter-burgage-depth`,
  `plots.dorestad.plot-depth`.
- (c) Depth from frontage times a ratio drawn from 2 to 6. Rests on
  `plots.northumberland.plot-aspect`, `plots.manasara.main-street-plot-aspect`,
  `plots.rabastens.charter-plot-aspect`.
- Recommendation: (a) with a maximum depth of 60 m in the default pack (the charter standard
  and the deepest Caernarfon burgage: `plots.england.charter-burgage-depth`,
  `plots.caernarfon.burgage-depth`). It ties depth to the street graph S3 already made, so no
  plot crosses a block, and it needs no separate yard rule.

**Question 5. What block size does S3 aim for when it lays out tertiary streets?**
- (a) Short side 30-60 m, long side 50-100 m, one band for all districts. Rests on
  `plots.nuremberg.block-side`, `plots.caernarfon.block-side`, `plots.mirande.block-side`,
  `plots.monpazier.block-width`, `plots.villeneuve-sur-lot.block-width`,
  `plots.castilian-new-towns.block-width`.
- (b) By district function: commerce and craft 15-60 m (dense core), residential 40-80 m,
  seat_of_power and religious_precinct larger and set by the landmark footprint. Rests on
  `plots.toledo.block-side`, `plots.nuremberg.block-side`, `plots.nordlingen.block-side`,
  `plots.galway.block-side`, and the elite-plot evidence in 2.4.
- (c) By era: early_medieval long strips along one street with no cross lanes,
  high_medieval planted grid 40-60 x 60-100 m, late_medieval organic 30-80 m.
- Recommendation: (a) for version 1, with the band stored per district function in the pack
  so (b) is a data change. Every medieval European block length in 2.3 falls between 15 and
  100 m, and every width outside binary blocks and the Bergen strips falls between 25 and
  60 m. The corpus figures are indicative readings of pictures, too weak to support a
  per-function split in version 1.

**Question 6. Does era change plot frontage through splitting?**
- (a) Yes: S4 cuts plots at the wealth band of question 3, then late_medieval splits a share
  of middling and rich plots in two by a line from the street to the back, halving the
  frontage; early_medieval and high_medieval do not split. Rests on `plots.freiburg.hofstatte-width`, `plots.freiburg.measured-width`,
  `plots.trondheim.central-plot-width`, `plots.buenos-aires.lots-per-block`.
- (b) No splitting; era has no effect on plots in version 1.
- Recommendation: (a), with the split share a pack value marked uncalibrated. Splitting is the
  one change over time every report that discusses it agrees on (2.6). The share has no
  source; Jeffrey sets it when reviewing the first Truro PNG at tier T5.

**Question 7. How much of a plot does S5 cover with buildings?**
- (a) One building per parcel at the street front, the rest left as yard. Rests on
  `plots.dublin-fishamble.house-length` (one house per plot near the street) and the Higham
  Ferrers excavation in parcels-density.md (back plots held only pits and ditches).
- (b) Front building plus backland outbuildings whose number grows with era (the burgage
  cycle). Rests on parcels-density.md finding 6 and `plots.mexico-1794.building-coverage` as
  the only measured district value.
- (c) A coverage ratio per district function and wealth, drawn from a range, marked
  uncalibrated.
- Recommendation: (a) for version 1. ARCHITECTURE.md defines a parcel as holding "at most one
  building", so (b) needs a glossary and model change; no source gives a per-plot coverage
  ratio for (c) (gap 1).

**Question 8. What house footprint does S5 put on the street front?**
- (a) Row houses: the full parcel frontage, depth 6-12 m, sharing side walls. Rests on
  `plots.figeac.row-house-depth`, `plots.figeac.head-house-length`,
  `plots.villefranche-de-rouergue.house-depth`, `plots.freiburg.stone-house-length`.
- (b) Detached houses: 5-9 m long and 4.5-6 m wide with gaps between them, one per plot. Rests
  on `plots.dublin-fishamble.house-length`, `plots.dublin-fishamble.house-width`,
  `plots.york-coppergate.building-length`, `plots.truso.building-width`.
- (c) By era: (b) for early_medieval, (a) for high_medieval and late_medieval.
- Recommendation: (c). The early-medieval evidence is all detached timber houses on fenced
  plots ("not in a uniform modern-style row", Waterford, timber-ports.md), and the
  high-medieval house evidence from Figeac and Villefranche-de-Rouergue is row houses; the era
  enum already separates them.

**Question 9. Does the generator model storeys or height in version 1?**
- (a) No. The overview map is drawn in plan; storeys change nothing on a 2D map.
- (b) Yes, as a building attribute of 1-3 storeys by district wealth, used only for
  rendering weight. Rests on `plots.mdme.storeys` (game source) and `plots.djenne.storeys`,
  `plots.agadez.storeys` (non-European).
- Recommendation: (a). The only European storey figure is a game source (gap 5), and the
  output is an overview map drawn in plan (CLAUDE.md).

**Question 10. Which fact statuses may a default rest on?**
- (a) Any status, with the pack entry naming its fact ids, so a later verification can change
  the value in one place.
- (b) Only facts with a measured or prescribed source (charter, ordinance, excavation,
  cadastre, survey); corpus readings of pictures and the game source serve as checks, never as
  defaults.
- (c) Only verified facts; until a verification report exists, defaults are marked
  uncalibrated and set by Jeffrey from the PNG.
- Recommendation: (b). Only 3 of 245 facts are verified, so (c) leaves every default
  without evidence; (b) keeps the indicative block readings and `plots.mdme.*` out of the
  defaults. Under (b) question 5 option (a) still stands, because
  `plots.mirande.block-side`, `plots.villeneuve-sur-lot.block-width` and
  `plots.castilian-new-towns.block-width` are measured figures.

Pending question for Jeffrey, in full: Question 1, where do plot and block defaults live:
(a) in the default rule pack as data, overridable by any pack (recommended); (b) in the pack
and also as an optional spec field `presets.plot_unit`; or (c) in core code as constants?
