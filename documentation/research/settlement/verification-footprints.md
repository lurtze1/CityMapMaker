## Verification: landmark footprints

Fact-check of `documentation/research/settlement/landmark-footprints.md` (branch `research/settlement`), 2026-10-20.
Each number below was read on the page linked next to it, fetched by me on 2026-10-20, unless marked
**(search summary only)**. That label means the page refused my fetch (HTTP 403 or similar) and I saw the
figure only in a search engine's summary. "(mine)" marks my own arithmetic. 1 ft = 0.3048 m and
1 acre = 4,046.86 m2 in all conversions.

Verdicts:
- **VERIFIED**: the second source agrees within about 10%.
- **CORRECTED**: the second source differs by more than 10%, or shows the first value is impossible.
- **UNVERIFIABLE**: I found no independent second source I could fetch.

Proposed classes being tested: tiny < 80 m2, small 80-300, medium 300-1,200, large 1,200-5,000,
huge 5,000-20,000, precinct > 20,000.

Warning about "area": several first values are **bounding boxes**, meaning length x width of the
smallest rectangle around the building, not the area it actually covers. For a cross-shaped church
the real footprint is clearly smaller than its bounding box. I mark these "bbox".

| Building | First value | Second value | Second source | Verdict | Class change |
|---|---|---|---|---|---|
| Chartres Cathedral | 10,875 m2 "ground area" (Wikipedia) | total length 130.20 m; chancel and transept 64 m across. Bounding box 130.2 x 64 = 8,333 m2 (mine). A building cannot cover more ground than its bounding box, so 10,875 m2 cannot be the footprint. The real footprint is below 8,333 m2. I found no direct area figure. The 10,875 figure appears elsewhere only in a 2008 Wikipedia copy. | [chartres-tourisme.com, dimensions](https://www.chartres-tourisme.com/en/the-cathedral/history-of-chartres-cathedral/dimensions) (official tourist office) | CORRECTED (to "under 8,333 m2", exact value unknown) | n (stays huge unless the real footprint is below 5,000 m2; Cologne at a similar length covers 7,914, so I expect huge, but I do not know) |
| Cologne Cathedral | 144.58 x 86.25 m; 7,914 m2 | Gesamtlänge außen 144.58; Gesamtbreite außen 86.25 m; überbaute Fläche 7,914 qm (marked approximate) | [koelner-dom.de, Der Dom in Zahlen](https://www.koelner-dom.de/erleben/der-dom-in-zahlen) (cathedral's official site) | VERIFIED | n |
| Hagia Sophia | 82 x 73 m; 5,986 m2 (bbox, first agent's arithmetic) | L. 82 m, W. 73 m | [Columbia University, Media Center for Art History](https://mcid.mcah.columbia.edu/work/w0000519?page=%2C1) | VERIFIED (dimensions). The area is a bbox; the real footprint is somewhat smaller. | n, but borderline: the bbox is only 20% above the 5,000 huge/large boundary |
| St Mary Redcliffe, Bristol | 76 x 36 m; 1,916 m2 (Wikipedia's stated area; note 76 x 36 = 2,736, so 1,916 is not the bbox) | "Length to end of Nave 240 ft" (73.2 m, mine); "Transepts 117 ft" (35.7 m, mine); nave and aisles 59 ft wide (18.0 m, mine); transept incl. aisles 44 ft wide (13.4 m, mine). Rough cross-shaped footprint: 73.2 x 18.0 plus transept arms (35.7 - 18.0) x 13.4 = about 1,550 m2 (mine, ignores porch, tower and chapels). | [Alison Stones, Images of Medieval Art and Architecture, Univ. of Pittsburgh](https://www.medart.pitt.edu/image/england/Bristol/StMaryRedcliffe/Bristol-StMarysRed.html) | VERIFIED (dimensions). The 1,916 m2 area has no second source; my rough estimate from the second source's dimensions is about 1,550. | n (both values are large) |
| Westminster Hall | 73 x 20 m; about 1,547 m2 | "240 by 67 feet (73 by 20 meters) ... area of 16,080 square feet (1,460 square meters)" | [Unofficial Royalty, Westminster Hall](https://unofficialroyalty.com/westminster-hall) (popular history site; weak source) | VERIFIED (6% apart). Note: [Grace's Guide](https://www.gracesguide.co.uk/Westminster_Hall) gives 1,547 in Wikipedia's exact words, so it is not independent. parliament.uk refused every fetch (403). | n |
| White Tower, London | 36 x 32 m; 1,152 m2 | "36m x 32.5m (118 feet x 106 feet 6 inches)" = 1,170 m2 (mine) | [Gatehouse Gazetteer, Tower of London](https://gatehouse-gazetteer.info/English%20sites/1904.html) | VERIFIED | n, but borderline: 1,170 is 2.5% under the 1,200 medium/large boundary; the corner turrets push the real outline over it |
| Rochester Castle keep | 21 x 21 m; 441 m2 | "70 by 70 feet externally" = 21.3 m square, 455 m2 (mine). The Historic England list entry 1011030 gives "21 m square" **(search summary only, 403)**. | [Handmade Kent, Rochester Castle](https://www.handmadekent.co.uk/rochester-castle/) (local guide; weak source) | VERIFIED | n |
| Lejre royal hall | 48 x 11 m; 528 m2 | "48.3 meters in length by 11.5 meters in width" = 555 m2 (mine) | [UCLA Viking site, Archaeology and the legendary Hleidargard](https://www.viking.ucla.edu/hrolf/archaeology.html) | VERIFIED (5% apart) | n |
| Borg chieftain's longhouse | 83 x 9.5 m; 789 m2 | "83 meters long and 12 meters wide" (the museum's reconstruction, stated to copy the original) = 996 m2 (mine) | [Museum Nord, The Chieftain's House](https://www.museumnord.no/en/exhibition/craftsmen/) (the museum that runs the site) | CORRECTED: width 9.5 vs 12 m, area 789 vs 996 m2. I could not read the excavation report, so I do not know which width is the excavated one. | n (both medium) |
| The Globe, original 1599 | about 30 m diameter; 707 m2 (circle) | "a 20-sided polygon 100 feet in diameter", measured "across the points" (Orrell's reading; the author says the evidence is ambiguous and Orrell later gave 97.61 ft). A 20-sided polygon 30.48 m across the points covers 718 m2 (mine). | [Gabriel Egan, Early Modern Literary Studies, Sheffield Hallam Univ.](https://extra.shu.ac.uk/emls/si-13/egan/) | VERIFIED | n |
| The Globe, 1997 reconstruction | not given separately | "a 20-sided Globe of about 100 feet external diameter" (same as above, so 718 m2, mine) | same as above | VERIFIED | n |
| The Rose | 22 m diameter, 14-sided; 380 m2 | "fourteen sides approximately 72' in diameter" = 21.9 m, 378 m2 as a circle (mine). After the 1592 enlargement only the yard was measured: about 163 m2. | [REED Online, Rose Playhouse background](https://ereed.org/collections/rosep/background-playhouse/) (Records of Early English Drama, academic) | VERIFIED | n |
| Corderie, Venetian Arsenal | 317 x 21 m; 6,657 m2 | "316-meter-long uninterrupted space"; width not given. 316 x 21 = 6,636 m2 (mine) | [Inexhibit, Arsenale of Venice](https://www.inexhibit.com/mymuseum/venice-arsenale-biennale/) (architecture magazine) | VERIFIED (length only; width rests on the Biennale alone) | n |
| Venetian Arsenal area | 45 ha (450,000 m2) | "ocupa una superficie de 48 hectáreas" | [italia.it (Italian national tourist board)](https://www.italia.it/es/veneto/venecia/arsenal-de-venecia) | VERIFIED (7% apart) | n |
| Palais des Papes | 11,000 m2 site (15,000 floor space) | Only the floor space confirmed: "15,000 m²" ([ITER newsline](https://www.iter.org/node/25174)); the palace's official site gives "15 000 m2 de plancher" **(search summary only; my fetch of [palais-des-papes.com](https://palais-des-papes.com/) showed no area figure)**. No independent source for the 11,000 m2 footprint. | as stated | UNVERIFIABLE (footprint) | n (11,000 or anything up to the 15,000 floor space is huge) |
| Palais Garnier | 12,000 m2 (Wikipedia) | "11 237 mètres carrés de surface au sol, 173 mètres de long et 125 mètres de large" | [France Guide, Palais Garnier](https://www.franceguide.info/fr/paris/que-faire-a-paris/palais-garnier/) (tourist guide); [Come to Paris](https://www.cometoparis.com/fre/decouvrir-paris/monuments-de-paris/opera-garnier-s933) gives 172 x 124 m | VERIFIED (area 7% apart). The 173 x 125 m outline differs from Wikipedia's 154.9 x 70.2/101.2 m; that outline probably includes the ramps and terraces, but I do not know. | n |
| Great Mosque of Kairouan | over 9,000 m2 | "rectangular plan 125 m long by 75 m wide" = 9,375 m2 (mine) | [Discover Islamic Art, Museum With No Frontiers](https://islamicart.museumwnf.org/database_item.php?id=monument%3BISL%3Btn%3BMon01%3B2%3Ben) | VERIFIED. The same page gives the prayer hall as 137 x 37 m, which is longer than the whole building, so that page has at least one error. UNESCO gives 135 x 80 m **(search summary only, 403)**. | n |
| Mosque-Cathedral of Córdoba | 180 x 130 m; 23,400 m2 | "planta rectangular de ciento setenta y cinco metros de norte a sur y ciento veintiocho de este a oeste" = 175 x 128 m = 22,400 m2 (mine). Roofed prayer hall: 115 x 128 m = 14,720 m2 (mine); the same document elsewhere gives "superficie cubierta ... aproximadamente 14.200 m2"; courtyard 5,527.70 m2. | [Plan Director Mezquita-Catedral de Córdoba 2020](https://mezquita-catedraldecordoba.es/site/assets/files/29170/plan_director_mezquita-catedral_de_co_rdoba_2020_propuesta.pdf) (the cathedral chapter's official conservation plan; text lines 676-678, 3464-3480, 10882) | VERIFIED (4% apart) | n for the whole compound (precinct). y if only the roofed hall is the "building": 14,200-14,720 m2 is huge, not precinct. |
| Brihadisvara compound, Thanjavur | 240.79 x 121.92 m; 29,357 m2 | "length (240 metres) being exactly twice its width (120 metres)" = 28,800 m2 (mine) | [Sahapedia, Notes on certain aspects of Brihadisvara Temple](http://www.sahapedia.org/notes-certain-aspects-of-brihadisvara-temple) (Indian cultural encyclopedia). ASI Trichy Circle gives 240.9 x 122 m **(search summary only; fetch returned HTTP 500)**. | VERIFIED | n |
| Meenakshi Temple compound | 57,000 m2 (5.7 ha) | "The temple complex spans 14 acres" = 56,656 m2 (mine). Other sites give 15, 17 or 45 acres **(search summary only)**. | [meenakshiammantemple.com](https://www.meenakshiammantemple.com/) (temple information site, not certainly official) | VERIFIED | n |
| Fountains Abbey precinct | 28 ha | "some seventy acres" = 28.3 ha (mine) | [The Cistercians in Yorkshire, Digital Humanities Institute, Univ. of Sheffield](https://www.dhi.ac.uk/cistercians/fountains/precinct/) | VERIFIED | n |
| Ulugh Beg Madrasa | 81 x 56 m; 4,536 m2 | "The madrassa itself measure 56m by 81m" | [Uzbek Travel, Ulugbek Medressa](https://uzbek-travel.com/about-uzbekistan/monuments/ulugbek-medressa/) (travel agency; weak source) | VERIFIED | n, but borderline: 9% under the 5,000 large/huge boundary |
| Sultan Han, Aksaray | 4,900 m2 total | "Total external area is 4,500 square meters, excluding the towers and portals" | [Aksaray Sultanhan Caravanserai, iaSU2012 conference proceedings, Mukogawa Women's Univ.](https://www.mukogawa-u.ac.jp/~iasu2012/pdf/iaSU2012_Proceedings_506.pdf) (academic paper, p. 277) | VERIFIED (8% apart; the gap is the towers and portals) | n, but borderline: both values sit just under 5,000 |
| Tōdai-ji Daibutsuden | 57 x 50 m; 2,850 m2 | "Width 57.012m, Length 50.480m, Height 48.742m" = 2,878 m2 (mine) | [Tōdai-ji official site](https://www.todaiji.or.jp/en/information/daibutsuden/) | VERIFIED | n |
| Merchant Adventurers' Hall, York | about 30 x 13 m; 390 m2 | Undercroft (the ground floor, same outline as the hall above): "87 ft." long, "varying between 37 ft. and 42 ft. in width" = 26.5 x 11.3-12.8 m, 299-339 m2 (mine). The hall is "eight bays long"; six bays of "about 11 ft." plus one of 16 ft and a screens bay of 8 ft give about 90 ft = 27.4 m (mine). | [RCHME, Inventory of the City of York vol. 5, pp. 81-88, British History Online](https://www.british-history.ac.uk/rchme/york/vol5/pp81-88) (Royal Commission survey) | CORRECTED: 299-339 m2, not 390. | y, borderline: the corrected value straddles the 300 m2 small/medium boundary |
| St Mark's Campanile | 12 x 12 m; 144 m2 | "a brick square shaft with a 12 meters side" | [Italyscapes, St Mark's Campanile](https://www.italyscapes.com/places/veneto/venice/towers/st-marks-campanile/) (travel site; weak source) | VERIFIED | n |
| Leaning Tower of Pisa, base | 15.484 m diameter; 188 m2 | "The outside diameter of the leaning Tower of Pisa's base is 15.484 meters." Foundation ring: 19.6 m diameter = 302 m2 (mine), from Burland's Ingenia article **(search summary only, 403)**. | [towerofpisa.org, facts](https://www.towerofpisa.org/leaning-tower-of-pisa-facts/) (visitor site; identical number to Wikipedia, so possibly copied from it) | VERIFIED (weakly independent) | n for the visible base (small). The buried foundation (302) would be medium, but a map draws what is above ground. |
| Ise Naikū main shrine | 10.9 x 5.5 m; 60 m2 | No independent figure found. World History Encyclopedia gives only the height (10 m); the 10.9 x 5.5 m figure appears on Shinto Wiki **(search summary only, 403)**, a wiki that may copy Wikipedia. Japanese searches found no 間口/奥行 (width/depth) figures. | [World History Encyclopedia, Ise Grand Shrine](https://www.worldhistory.org/Ise_Grand_Shrine/) (height only) | UNVERIFIABLE | n (I do not know) |

### Newly found sizes (for the gaps)

**Wells and well-houses.** These fill the gap the first agent left. All are tiny.

| Thing | Size | Area (mine) | Source |
|---|---|---|---|
| Medieval town well shaft, 12-13 High Street, Colchester | "internal diameter of 1.55m with stone lining 30cm thick" | 2.1 m across outside, about 3.4 m2 | [Colchester Heritage Explorer MCC1753](https://colchesterheritage.co.uk/Monument/MCC1753) |
| Medieval well, Ipswich (IPS 1244) | "1.70m in diameter with a wall thickness of 0.26m" | about 2.3 m2 | [Suffolk Heritage Explorer MSF36079](https://heritage.suffolk.gov.uk/Monument/MSF36079) |
| Well-house, Nine Wells, St Davids (roofed chamber over a basin) | "1.7m square and 2.1m high" | 2.9 m2 | [Coflein 305398 (Royal Commission, Wales)](https://coflein.gov.uk/en/site/305398) |
| Dupath holy well-house, Cornwall (largest in Cornwall) | 3.9 x 3.59 m externally **(search summary only, Historic England 1013663 returned 403)** | 14 m2 | Historic England list entry 1013663 |

So a medieval well or well-house is 2-15 m2. A town well with a small roof or pump house stays far under 80 m2.

**Market cross bases.** Tiny.

| Thing | Size | Area (mine) | Source |
|---|---|---|---|
| Village cross, Shillingstone, Dorset | base "3.00 m by 2.90 m"; lower step "2.90 m by 2.85 m"; upper step "2.10 m by 2.05 m"; socket stone "1.05 m square" | 8.7 m2 | [Dorset HER MDO4674](https://heritage.dorsetcouncil.gov.uk/Monument/MDO4674) |
| Market cross, Brigstock, Northamptonshire (scheduled monument) | four steps "decreasing in size from about 3.8m square to 1.25m square"; socket stone "approximately 0.7m square" | 14.4 m2 | [Northamptonshire HER DNN4252](https://her.northamptonshire.gov.uk/Designation/DNN4252) |

**Small market halls and market houses.** Ledbury, Chipping Campden and Thaxted: the listing texts give
counts, not measurements: Ledbury "7 posts on the long sides and 3 at the ends"
([listing text](https://britishlistedbuildings.co.uk/101349392-market-house-ledbury)); Chipping Campden
"2 x 5 arches" ([listing text](https://britishlistedbuildings.co.uk/101078442-market-hall-chipping-campden));
Thaxted Guildhall "3 storeys and cellars", no size
([listing text](https://britishlistedbuildings.co.uk/101112905-guildhall-thaxted)). No dimensions found
for those three: UNVERIFIABLE. Other market halls with sizes:

| Thing | Size | Area (mine) | Source |
|---|---|---|---|
| Hereford Market Hall, High Town (demolished; also the town's hall) | "84 feet long and 34 feet wide" = 25.6 x 10.4 m (mine) | 265 m2 | [Herefordshire Through Time (county council archaeology service), Public buildings](https://htt.herefordshire.gov.uk/herefordshires-past/the-post-medieval-period/architecture/buildings-1500-1750/public-buildings) |
| Yeovil Market House, 1740 | "70 feet long by 20 feet wide (21.3m x 6m)" | 128 m2 | [Yeovil's History, The Market House](https://www.yeovilhistory.info/markethouse-borough.htm) (local history site) |
| Dunster Yarn Market, 1609 (octagonal) | 9.4 m in diameter **(search summary only, Historic England 1015706 returned 403)** | about 70 m2 | Historic England scheduled monument 1015706 |

So a small English market hall or market house is about 70-270 m2: small, or tiny for the octagonal
open kind. **Small town halls:** I found no separate small town hall with measurements. Hereford's
market hall above was the town's hall; it is the only figure I have.

**Slipways.**

| Thing | Size | Area (mine) | Source |
|---|---|---|---|
| Possible slipway, late medieval royal shipyard, Small Hythe, Kent (early 15th to mid 16th c.) | excavated "linear hollow ... 25 m long and 12 m wide" | 300 m2 | [Kent HER MKE14001](https://heritage.kent.gov.uk/Monument/MKE14001) |

The record calls it a possible slipway; "the overall size of the shipyard remains unclear". Together with
the first agent's ship lengths (27-30 m), a slipway for one ship is about 25-30 x 6-12 m: 150-360 m2,
on the small/medium boundary, and long and thin (aspect ratio 2-5:1).

**Edwardian castles.**

| Thing | Size | Area (mine) | Source |
|---|---|---|---|
| Beaumaris, inner ward | "about 3/4 of an acre" | 3,035 m2 (verifies the first agent's 3,000) | [castlewales.com, Beaumaris](https://www.castlewales.com/beaumar.html) |
| Beaumaris, gap between inner and outer walls | "around 18.3 metres (60 ft.)"; inner walls "4.7 metres (15.5 ft.) thick" | not computed | [World History Encyclopedia, Beaumaris Castle](https://www.worldhistory.org/Beaumaris_Castle/) |
| Harlech, inner ward courtyard | "39 meters wide and 47 meters long in the western part and 53 meters in the eastern part" | 1,830-2,070 m2 courtyard only | [Medieval Heritage, Harlech Castle](https://medievalheritage.eu/en/main-page/heritage/wales/harlech-castle/) |
| Conwy, wards | outer ward "50 x 30 meters"; inner ward "25 x 22 meters" | 2,050 m2 of courtyards only | [Medieval Heritage, Conwy Castle](https://medievalheritage.eu/en/main-page/heritage/wales/conwy-castle/) |
| Conwy, overall | about 125 x 40 m **(search summary only, ancient-history-sites.com returned 403)** | about 5,000 m2 | ancient-history-sites.com, Conwy Castle |
| UNESCO property areas | Beaumaris 2 ha, Harlech 1.7 ha, Conwy castle and town walls 1.1 ha **(search summary only, whc.unesco.org returned 403)**. These are protected-area boundaries, not castle outlines. | | whc.unesco.org document 216636 |

I found no overall outline measurement I could fetch for Conwy, Harlech or Beaumaris. Rough reading: the
Beaumaris inner ward alone is 3,000 m2, and there is an 18 m ring of outer ward plus two wall thicknesses
around it, so the whole castle is clearly over 5,000 m2 (huge). That is a judgement from the figures
above, not a measurement.

**Gate tower bases.** Monnow Bridge gate and Micklegate Bar: no dimensions found. The Gatehouse Gazetteer
describes the Monnow gate only in words ("elliptical in shape"); the RCHME York volume 2 has measured plans
of Micklegate Bar (pp. 97-99), but only the list of illustrations was readable. UNVERIFIABLE. Other town
gates with sizes:

| Thing | Size | Area (mine) | Source |
|---|---|---|---|
| Town gate, Barrack Street, Loughrea, Ireland (15th c., single passage) | "about 6 metres of length and 7.70 metres of width" | 46 m2 | [Irish Stones, Loughrea town wall and gate](https://www.irishstones.org/place.aspx?p=885&i=2) |
| Myślibórz Gate, Lipiany, Poland (gate tower) | "square plan with dimensions 9×9 meters" | 81 m2 | [Medieval Heritage, Lipiany city defensive walls](https://medievalheritage.eu/en/main-page/heritage/poland/lipiany-city-defensive-walls/) |
| Old Gate (Altpörtel), Speyer, Germany (single tower) | about 13.00 x 8.70 m at the base **(search summary only, structurae.net returned 403)** | 113 m2 | structurae.net, Old City Gate |

So a single-tower town gate is about 45-115 m2: on the tiny/small boundary.

### Effect on the proposed classes

1. **No verified correction moves a well-known example out of its class.** The corrections are Chartres
   (10,875 m2 is impossible; real value below 8,333 m2, most likely still huge), Borg (789 to 996 m2,
   still medium) and the Merchant Adventurers' Hall (390 to 299-339 m2).
2. **One borderline move:** the Merchant Adventurers' Hall drops onto the 300 m2 small/medium boundary.
   It was the smallest example cited for "medium" and for the hall default. The medium range still has
   Rochester (455), Lejre (555), the Globe (718) and Borg (996), so the hall default of medium holds.
3. **Several examples sit within 10% of a boundary**, so they can fall either side depending on
   whether turrets, porches or towers are counted: White Tower 1,170 (boundary 1,200); Ulugh Beg 4,536
   and Sultan Han 4,500-4,900 (boundary 5,000); Hagia Sophia 5,986 as a bbox (boundary 5,000);
   Merchant Adventurers' Hall 299-339 (boundary 300). This is a weakness of hard boundaries, not of
   the data.
4. **Córdoba depends on the definition.** The whole compound (22,400 m2) is a precinct; the roofed
   prayer hall alone (14,200-14,720 m2) is huge. Which one counts is Jeffrey's decision.
5. **The gap findings support the proposal:** wells and well-houses 2-15 m2 and market-cross bases
   9-14 m2 confirm **tiny** for well and monument (the first agent called the well evidence weak; it is
   now direct). Small market halls 70-265 m2 are **small**. Edwardian stone castles exceed 5,000 m2
   overall, which supports **huge** as the castle default. A one-ship slipway is about 150-360 m2,
   small to medium.
6. **One default to reconsider:** bridgehead/gate. Measured single-tower gates are 46-113 m2, which
   straddles tiny and small. The proposed default "small" is defensible; "tiny" would also fit. Which one
   to use is Jeffrey's decision.
7. **Source quality:** St Mark's, Pisa, Rochester, Westminster Hall and Ulugh Beg are verified only
   against tourist or popular-history sites. They agree with Wikipedia, but those sites may have copied
   it. Historic England, UNESCO and parliament.uk refused every fetch (HTTP 403), so no register text from
   those bodies was read directly.

Downloaded files used: a local download (Córdoba Plan Director),
a local download (Sultanhan paper), a local download and
a local download (no dimensions found in either).
