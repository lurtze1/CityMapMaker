## Street networks, measured

> **Corrections after verification (2026-10-20):** see `verification-streets-walls.md`. Boeing 2019, Cardillo 2006, Strano 2012 and Barthelemy 2013 numbers are verified digit for digit; Boeing's values are whole municipalities (Venice includes Mestre), and São Paulo, not Rome, is the highest-entropy non-US city. Hillier's ±15° statement is from Hillier 1997 (an axial map of part of London); the "planned grid near 90°" line was this report's own inference. The Roman 8 ft / 16 ft is a private right of way, not a public street minimum. Heian-kyō major streets ranged up to about 51 m, not only 24 m. Chang'an wards were of unequal size.

Research date: 2026-10-20. Scope: measured numbers for pre-industrial street networks, plus the
network metrics urban-morphology researchers use, so the street stage can be calibrated and
checked. `docs/research/research-academic.md` §2.7 and §2.9 already name Space Syntax, Conzen,
Caniggia and Boeing's entropy as ideas; this report adds the numbers behind them.

Conventions in this file:

- Every number has a source. Rows marked **(unverified)** or "not fetched" come from search
  snippets or dead links and are kept for completeness, not as evidence. Otherwise the URL was
  fetched in this session. A number marked
  **(computed)** is arithmetic I did on a fetched number; the formula is shown.
- **(unverified)** means the claim came from a search snippet and the page could not be fetched,
  or the page did not show the claim.
- "Graph" here means the street network as a set of **nodes** (intersections and dead ends) and
  **edges** (street segments between two nodes). **Degree** of a node = number of street
  segments that meet there: degree 1 is a dead end, degree 3 a T- or Y-junction, degree 4 a
  crossroads (X-junction).

### Findings

1. **Organic old cores are T-junction networks; planned grids are X-junction networks.** In
   twenty 1-square-mile samples, the "self-organized" cities Ahmedabad, Bologna, Cairo and
   Venice have more degree-3 nodes than degree-4 nodes; the planned grids New York, San
   Francisco and Washington have the reverse (Cardillo et al. 2006). Strano et al. 2012 make the
   same point: T-junctions are "typical of self-organised or 'organic' urban networks, like those
   of Venice or Cairo", grids of top-down planning. Excavation at Islamic Merv (8th–13th c.)
   found that "many streets end in T-junctions or staggered crossroads" (Williams).
2. **Old cores are fine-grained.** Venice, Mogadishu, Helsinki, Jerusalem and Casablanca have the
   shortest median street segments of 100 world cities; Venice's median is 23.2 m (Boeing 2019).
   In 1-square-mile samples, Ahmedabad has 2870 intersections against 32 in suburban Irvine
   (Cardillo et al. 2006).
3. **Streets in organic cores still meet at near-right angles or run on almost straight.**
   Hillier (1997) measured axial-line junctions in "deformed grids" (irregular grids that grew
   rather than were drawn): a "surprisingly high proportion" are within about 15° of 90°, or
   within about 15° of a straight 180° continuation. Long lines tend to end in near-straight
   continuations; short lines tend to end in near-right angles. He states this holds "even in
   the most tortuous cul-de-sac sequences" of a traditional Islamic town.
4. **Orientation entropy separates grids from everything else, but not old from new.** Rome has
   the highest orientation entropy of the non-US cities in Boeing's 100 (φ = 0.005, almost
   perfectly disordered); Chicago the lowest (φ = 0.90). But Kyoto, whose core is an 8th-century
   grid, scores φ = 0.357, and Moscow and Istanbul score as disordered as Rome. Entropy measures
   "is it one grid?", not "is it old?".
5. **Planning leaves its mark in centrality and block shape, not in simple counts.** For Paris
   1789–2010, average degree stayed at ⟨k⟩ ≈ 3.0 throughout. Haussmann's new avenues are only
   about 6 % of today's network length, yet they reorganised where betweenness centrality
   concentrates and flattened the block-shape distribution (Barthelemy et al. 2013).
6. **Old networks are stable at the top of the hierarchy.** In 200 years of growth north of Milan,
   "high-centrality roads that form the backbone of the urban structure" stayed fixed while
   densification (filling in between existing roads) and exploration (new roads pushing
   outward) added the rest (Strano et al. 2012). That supports the build order "main roads
   first, then fill".
7. **Islamic towns: three street orders, and the dead end is the normal residential street.**
   Cairo's streets fall into main arteries (shari' or qasaba), quarter connectors with open ends,
   and cul-de-sacs; "Most of the streets in Cairo were of this type" (the cul-de-sac). Lanes and
   cul-de-sacs were legally "semi-private, collectively-owned spaces" (Hakim, via CSBE). No
   measured dead-end percentage for any Islamic old core was found.
8. **Legal minimum widths existed in several cultures, and most cited minima were small.** Roman Twelve Tables:
   8 Roman feet (about 2.37 m) on the straight, double on bends. Islamic hadith: 7 cubits
   (adhru') when neighbours dispute a road's width. London 1667: narrowest lanes to be widened
   to 14 ft (unverified). Planned East Asian capitals are the exception, with 12 m minor streets
   and 85–150 m ceremonial avenues.
9. **Encroachment narrows streets over time.** Merv's excavated east–west street was "initially
   some 6m wide" and was narrowed to about 5 m by rebuilding on both sides (Williams). In
   Heian-kyō, road edges were turned into fields and house plots ("kosho") despite an imperial
   ban (Kyoto Prefecture). A generator can model age as street narrowing plus kinks.

### Numbers

#### Street widths by class

| Culture | Era | Street class | Value | Source |
|---|---|---|---|---|
| Roman (law) | c. 450 BC, Twelve Tables | any public road (via), minimum | 8 Roman ft (~2.37 m) straight; double on curves | [Wikipedia: Roman roads](https://en.wikipedia.org/wiki/Roman_roads) |
| Roman (practice) | Late Republic | rural public road, common width | ~12 Roman ft | same |
| Roman (measured) | various | road widths found | 1.1 m to over 7.0 m | same |
| Roman | Imperial | principal roads, average of ~500 examined (Hugh Davies) | 22 pedes, ~6.5 m | [Designing Buildings Wiki](https://www.designingbuildings.co.uk/wiki/The_history_of_the_dimensions_and_design_of_roads%2C_streets_and_carriageways) |
| Roman, Pompeii | AD 79 | Via dell'Abbondanza (main street), ~900 m long | ~4.5–5 m | [Madain Project](https://madainproject.com/via_dell_abbondanza) |
| Roman, Pompeii | AD 79 | ordinary streets / widest | 2.4–4.5 m / just over 7 m (unverified; Connolly via a dead link) | not fetched |
| Islamic (law) | hadith, 7th c. onward | disputed road, width to be set | 7 cubits (adhru'); translated on the page as "seven handspans" | [Tohed hadith page](https://en.tohed.com/hadith/silsilah-sahihah/chapter/483/sub/15488/) |
| Islamic (unit) | Baghdad, mid-8th c. | one cubit (al-dhira' al-sawda') | 49.5 cm; other cubits 58–80 cm | [CSBE, Hakim](https://www.csbe.org/legislation) |
| Islamic (computed) | — | 7 cubits | 3.5 m to 5.6 m **(computed: 7 × 0.495 to 7 × 0.80)** | from the two rows above |
| Islamic (law) | Maliki case (Ashhab) | remaining road width in a ruling on encroachment | 8 cubits; owner wanted 1–2 cubits more, building ordered demolished | [CSBE, Hakim](https://www.csbe.org/legislation) |
| Islamic (law) | classical fiqh | fina' (owner's usable strip in front of the door) on a main street | not more than half the street width; on lanes and cul-de-sacs it can cover the whole lane | same |
| Islamic, Merv (Sultan Kala) | 8th–13th c. | substantial east–west streets | c. 3–4 m, some up to 5 m | [Internet Archaeology 25, Williams, §8.3](https://www.intarch.ac.uk/journal/issue25/1/8_3.html) |
| Islamic, Merv | 8th–13th c. | alleys | c. 2–3.5 m | same |
| Islamic, Merv | 8th–13th c. | one excavated street | 6 m when laid out, ~5 m after encroachment | same |
| Medieval England (law) | 1114–1118, Leges Henrici Primi | via Regia | wide enough for two wagons to pass; 16 armed knights abreast. No feet figure in the law | [Designing Buildings Wiki](https://www.designingbuildings.co.uk/wiki/The_history_of_the_dimensions_and_design_of_roads%2C_streets_and_carriageways) |
| Medieval Italy (law) | Bologna, 1288 | portico (covered walkway) minimum height | 2.66 m, so a rider can pass | [Bologna Welcome](https://bolognawelcome.com/en/blog/top-6-facts-about-bolognas-unesco-porticoes) |
| Medieval France | bastide Monpazier, 1284 | carreyrous (rear service lanes) | 1–2 m (unverified, snippet from france-voyage.com) | not fetched |
| Medieval France | bastide Monpazier | androns (fire gaps between houses) | a few tens of cm (unverified) | not fetched |
| Early modern England | London, 1667 | narrowest lanes, new minimum | 14 ft (unverified) | not fetched |
| Victorian England (law) | London Building Act 1894 | street minimum | 40 ft; 24 ft carriageway stayed standard | [Designing Buildings Wiki](https://www.designingbuildings.co.uk/wiki/The_history_of_the_dimensions_and_design_of_roads%2C_streets_and_carriageways) |
| Japan, Heian-kyō | 794 onward | Suzaku-ōji (central avenue) | 28 jō, ~85 m | [Wikipedia: Heian-kyō](https://en.wikipedia.org/wiki/Heian-ky%C5%8D); [Kyoto Prefecture archive](https://hyakugo.pref.kyoto.lg.jp/eng/?p=294) |
| Japan, Heian-kyō | 794 onward | ōji (major streets) | 8 jō, 24 m | [Kyoto Prefecture archive](https://hyakugo.pref.kyoto.lg.jp/eng/?p=294) |
| Japan, Heian-kyō | 794 onward | kōji / shōji (minor streets) | 4 jō, ~12 m | same; Wikipedia: Heian-kyō |
| China, Tang Chang'an | 7th–9th c. | Zhuque Avenue | 150 m wide, 5 km long | [Shaanxi government](https://en.shaanxi.gov.cn/as/hac/hos/201704/t20170428_1595612.html) |
| China, Tang Chang'an | 7th–9th c. | main streets / narrowest streets | ~100 m / 20–25 m | same |
| China (canon) | Kaogong ji (Zhou ritual text) | north–south streets of the ideal capital | "nine carriage tracks in width" | [Wikipedia: Rites of Zhou](https://en.wikipedia.org/wiki/Rites_of_Zhou) |

#### Blocks, grain and network counts

| Culture / place | Era | Measure | Value | Source |
|---|---|---|---|---|
| Heian-kyō | 794 | block (chō) | 40 jō square, ~121 m per side | [Wikipedia: Heian-kyō](https://en.wikipedia.org/wiki/Heian-ky%C5%8D) |
| Heian-kyō | 794 | plots per chō | 32 plots in 4 rows of 8, served by internal N–S alleys | [JAANUS: machi](https://aisf.or.jp/~jaanus/deta/m/machi.htm) |
| Heian-kyō | 794 | city size | ~4.5 km E–W × 5.2 km N–S; each bō = 16 chō | Wikipedia: Heian-kyō |
| Heian-kyō (computed) | 794 | street share of a minor-street grid cell | ~17 % **(computed: 1 − (121 / (121 + 12))², ignoring the wide ōji)** | from the rows above |
| Edo Japan | 17th–19th c. | chō as length unit | 60 ken, ~109 m | [JAANUS: machi](https://aisf.or.jp/~jaanus/deta/m/machi.htm) |
| Tang Chang'an | 7th–9th c. | ward (fang) | about 1 li, ~530 m per side; Zhengpingfang measured 533.6 × 464.6 m | [China Daily via chinaservicesinfo](https://www.chinaservicesinfo.com/s/202605/19/WS6a0bc472498e36855033ca59/unearthing-life-in-the-mists-of-time.html) |
| Tang Chang'an | 7th–9th c. | wards / grid | 108 fang; 11 N–S avenues, 14 E–W streets; 9,721 × 8,651 m | [Shaanxi government](https://en.shaanxi.gov.cn/as/hac/hos/201704/t20170428_1595612.html) |
| Tang Chang'an | — | internal ward roads | cross-shaped, later T-shaped in Zhengpingfang | China Daily via chinaservicesinfo |
| Pompeii | AD 79 | block lengths along Via dell'Abbondanza | 32–147 m; many 32–35 m; near forum 50–70 m | [Pompeii Perspectives](https://pompeiiperspectives.org/index.php/insula-attributes) |
| Kaogong ji ideal capital | Zhou canon | plan | square of 9 li per side, 3 gates per side, 9 N–S and 9 E–W streets | [Wikipedia: Rites of Zhou](https://en.wikipedia.org/wiki/Rites_of_Zhou) |

#### Network statistics: 1-square-mile samples (Cardillo et al. 2006)

Source: [Cardillo, Scellato, Latora, Porta, arXiv physics/0510162](https://arxiv.org/pdf/physics/0510162).
Each sample is 1 square mile = 2.59 km² = 259 ha. N = intersections, K = street segments,
Cost = total street length (m), l = mean segment length (m), M = meshedness (defined below).
Columns ⟨k⟩ and m/ha are **(computed)**: ⟨k⟩ = 2K/N; m/ha = Cost / 259.

| City sample | Type (paper's label) | N | K | Cost (m) | l (m) | M | ⟨k⟩ (computed) | street m/ha (computed) |
|---|---|---|---|---|---|---|---|---|
| Ahmedabad | self-organized | 2870 | 4387 | 121037 | 27.59 | 0.262 | 3.06 | 467 |
| Venice | self-organized | 1840 | 2407 | 75219 | 31.25 | 0.152 | 2.62 | 290 |
| Cairo | self-organized | 1496 | 2255 | 84395 | 37.47 | 0.253 | 3.01 | 326 |
| Seoul | (organic per text) | 869 | 1307 | 68121 | 52.12 | 0.253 | 3.01 | 263 |
| Bologna | self-organized | 541 | 773 | 51219 | 66.26 | 0.214 | 2.86 | 198 |
| London | — | 488 | 730 | 52800 | 72.33 | 0.249 | 2.99 | 204 |
| Vienna | — | 467 | 692 | 49935 | 72.16 | 0.242 | 2.96 | 193 |
| Paris | — | 335 | 494 | 44109 | 89.29 | 0.241 | 2.95 | 170 |
| Savannah | planned grid | 584 | 958 | 62050 | 64.77 | 0.322 | 3.28 | 240 |
| New York | planned grid | 248 | 419 | 36172 | 86.33 | 0.348 | 3.38 | 140 |
| Barcelona | planned | 210 | 323 | 36179 | 112.01 | 0.275 | 3.08 | 140 |
| Irvine 1 | planned suburb | 32 | 36 | 11234 | 312.07 | 0.085 | 2.25 | 43 |

The paper's own findings: Ahmedabad, Cairo and Venice have the shortest mean segments; New
York, Savannah and San Francisco have M > 0.3; Irvine and Walnut Creek have M < 0.1 (tree-like);
"organic patterns such as Ahmedabad, Cairo and Seoul also exhibit high values of meshedness".

#### Network statistics: Boeing 2019, 100 world cities (selected)

Source: [Boeing, "Urban Spatial Order", arXiv 1808.00600 (Applied Network Science 4:67)](https://arxiv.org/pdf/1808.00600).
**Caution:** each value covers the whole municipality from OpenStreetMap (2018), not the
historic core. Only Venice is close to a pure pre-industrial network. φ = orientation order,
Hₒ = orientation entropy (nats), l = median segment length (m), ς = circuity, ⟨k⟩ = average
degree, P_de = share of nodes that are dead ends, P_4w = share that are 4-way.

| City | φ | Hₒ | l (m) | ς | ⟨k⟩ | P_de | P_4w |
|---|---|---|---|---|---|---|---|
| Venice | 0.017 | 3.564 | 23.2 | 1.090 | 2.474 | 0.300 | 0.073 |
| Jerusalem | 0.014 | 3.568 | 44.0 | 1.092 | 2.735 | 0.180 | 0.109 |
| Rome | 0.005 | 3.578 | 73.7 | 1.070 | 2.820 | 0.161 | 0.145 |
| Damascus | 0.043 | 3.536 | 65.8 | 1.085 | 2.801 | 0.146 | 0.107 |
| Prague | 0.049 | 3.529 | 84.5 | 1.065 | 2.807 | 0.177 | 0.171 |
| London | 0.015 | 3.566 | 70.3 | 1.061 | 2.561 | 0.251 | 0.070 |
| Lisbon | 0.023 | 3.558 | 60.8 | 1.068 | 2.923 | 0.108 | 0.154 |
| Istanbul | 0.007 | 3.576 | 50.1 | 1.059 | 2.998 | 0.093 | 0.174 |
| Cairo | 0.041 | 3.538 | 66.6 | 1.067 | 2.996 | 0.085 | 0.171 |
| Kyoto | 0.357 | 3.148 | 63.3 | 1.090 | 2.887 | 0.134 | 0.157 |
| Paris | 0.016 | 3.566 | 71.5 | 1.023 | 3.110 | 0.050 | 0.240 |
| Barcelona | 0.108 | 3.462 | 78.1 | 1.052 | 3.135 | 0.078 | 0.303 |
| Manhattan | 0.669 | 2.650 | 82.2 | 1.017 | 3.508 | 0.027 | 0.572 |
| Chicago | 0.899 | 2.083 | 105.3 | 1.016 | 3.343 | 0.074 | 0.507 |

Ranges across all 100 cities, from the [Barthelemy & Boeing 2024 review, arXiv 2409.08016](https://arxiv.org/pdf/2409.08016):
⟨k⟩ 2.30–3.55 (median 2.93); dead-end share 2 %–39 % (median 14 %); 4-way share 4 %–59 %
(median 18 %); mean edge length 64.5–537.5 m (median 118.6 m). European cities on average have
the highest entropy, the highest dead-end share and the lowest average degree (Boeing 2019).

#### Historic change, measured

| Place | Period | Measure | Value | Source |
|---|---|---|---|---|
| Groane, north of Milan, 125 km², 29 centres | 1833 → 2007 | nodes | 255 → over 5000 | [Strano et al. 2012, arXiv 1203.0300](https://arxiv.org/pdf/1203.0300) |
| same | 1914 → 1980 | average degree | 2.57 → 2.8 | same |
| same | 1833 → 2007 | 4-way share N4/N | 11 % → 15.5 % | same |
| same | 1833 → 2007 | r_N = (dead ends + T-junctions) / all junctions | 0.87 → 0.835 | same |
| same | before 1933 | block shape factor Φ | single peak, mean ~0.5, SD 0.25 | same |
| same | after 1955 | block shape factor Φ | second peak at 0.62 (regular blocks) | same |
| same | 1833 → 2007 | relative spread of block areas σ/μ | 0.5 → 0.26 | same |
| same | 2007 | block area distribution P(A) ~ A^(−τ) | τ = 1.9 ± 0.1 (was ~1.2 in 1833) | same |
| Paris | 1789 → 2010 | average degree | ⟨k⟩ ≈ 3.0, constant | [Barthelemy et al. 2013, ar5iv 1307.2203](https://ar5iv.arxiv.org/html/1307.2203) |
| Paris | 1836 / 1888 | nodes; total street length | ~3000, ~400 km / ~6000, almost 700 km | same |
| Paris | before Haussmann | block shape factor | centred near Φ = 0.5 ("rectangles") | same |
| Paris | after Haussmann | Haussmann's new streets as share of today's length | ~6 % | same |
| 131 world cities | 2014 | block area tail exponent | τ ≈ 2 | [Louf & Barthelemy 2014, arXiv 1410.2094](https://arxiv.org/pdf/1410.2094) |
| 100 European cities, historic core vs planned extension | 2025 | intersections per km²; mean segment length | 103 vs 87; 78 m vs 88 m (master's thesis, not peer-reviewed; network type not stated in abstract) | [Kulzhanova 2025 abstract](https://webthesis.biblio.polito.it/39015/8/Kulzhanova_abstract_EN.pdf) |

Note the Kulzhanova density (103 per km²) and the Cardillo Venice density (1840 / 2.59 ≈ 710
per km², computed) differ by a factor of seven. They likely use different networks (drivable
streets vs every passage); I do not know which. A generator must state which network it
measures.

### Metrics a generator can compute

All of these run on the street graph the generator already has. Costs: "linear" = one pass over
nodes or edges; "all-pairs" = shortest paths from every node, which is slow above a few
thousand nodes but fine for a town.

| Metric | Definition (plain English) | Organic old core | Planned grid | Cost | Source |
|---|---|---|---|---|---|
| Junction mix: P(k=1), P(k=3), P(k=4) | Share of nodes that are dead ends, T/Y-junctions, crossroads. Degree-2 nodes (bends) are dropped first | P(3) > P(4). Venice: dead ends 30 %, 4-way 7 % | P(4) > P(3). Manhattan: dead ends 3 %, 4-way 57 % | linear | Cardillo 2006; Boeing 2019 |
| Average degree ⟨k⟩ | Mean number of segments per node; equals 2K/N | 2.5–3.0 (Venice 2.47, Rome 2.82) | 3.3–3.5 (Chicago 3.34, Manhattan 3.51) | linear | Boeing 2019 |
| r_N | (dead ends + T-junctions) / all junctions | 0.87 (rural/pre-urban Groane 1833) | lower; 0.835 even after 200 years of unplanned growth | linear | Strano 2012 |
| Median segment length l | Middle value of the lengths of all street segments | Venice 23 m; Ahmedabad mean 28 m; Cairo mean 37 m | Manhattan 82 m; Chicago 105 m | linear | Boeing 2019; Cardillo 2006 |
| Street length per hectare | Total street length / area | Ahmedabad 467, Cairo 326, Venice 290 m/ha (computed) | New York 140, Barcelona 140 m/ha (computed) | linear | Cardillo 2006 |
| Orientation entropy Hₒ and order φ | Put every segment's compass bearing (and its reverse) into 36 bins of 10°, shifted by −5° so 0° and 90° sit mid-bin; Hₒ = −Σ P(i) ln P(i). Max = ln 36 = 3.584; perfect 4-way grid = 1.386. φ = 1 − ((Hₒ − 1.386) / (3.584 − 1.386))² | φ ≈ 0.00–0.05 (Rome 0.005, Venice 0.017) | φ ≈ 0.67–0.90 (Manhattan, Chicago). Caution: Kyoto's old grid only 0.357 at city scale | linear | Boeing 2019 |
| Circuity ς | Total street length / total straight-line length between the two ends of each segment | 1.07–1.09 (Rome, Venice, Jerusalem) | 1.016–1.017 (Chicago, Manhattan) | linear | Boeing 2019 |
| Meshedness M | How loopy the graph is: M = F / (2N − 5), with F = K − N + 1 the number of enclosed faces (blocks). 0 = tree, 1 = fully triangulated | 0.15 (Venice) to 0.26 (Ahmedabad) | 0.32–0.35 (Savannah, New York); planned cul-de-sac suburbs < 0.1 | linear | Cardillo 2006 |
| Block shape factor Φ | Block area / area of the circle whose diameter is the block's longest span. Square = 0.64 (computed: 2/π); elongated blocks → low | broad, single-peaked around 0.5 (Groane pre-1933, Paris pre-1836) | sharp peaks; Manhattan at 0.3 and 0.5 (two rectangle types) | linear (needs block polygons) | Strano 2012; Louf & Barthelemy 2014; Barthelemy 2013 |
| Block area distribution | Histogram of block areas; tail fitted as P(A) ~ A^(−τ) | wide spread; σ/μ ≈ 0.5; τ ≈ 1.2 in a young rural network (Groane) | no planned-grid measurement found; Groane's later, denser network has σ/μ ≈ 0.26 and τ ≈ 1.9, but that is unplanned growth, not a planned grid | linear | Strano 2012 |
| Junction angle distribution | At each junction, the angles between the streets that meet | most within ~15° of 90° or of 180° (straight-on), even in organic towns | almost all ~90° | linear | Hillier 1997 |
| Betweenness centrality (BC) | For each node or street, the share of all shortest paths that pass through it. A proxy for through-traffic | central high-BC "backbone" stable for centuries | planning moves where high BC sits (Paris) | all-pairs | Strano 2012; Barthelemy 2013 |
| Space Syntax integration | Built on the **axial map** (fewest longest straight sight-lines covering every space). Integration of a line = how few line-changes on average reach every other line; the most integrated 10–25 % form the "integration core" (unverified; Istanbul study snippet) | Old-town markets and main churches sit on high-integration lines (research-academic.md §2.7) | grid: integration spread evenly | axial map is hard to build; integration is all-pairs | Hillier 1997 |

Space Syntax caveats, both from Hillier 1997: there is no unique way to draw an axial map, so
values differ between analysts; and absolute integration values depend on map size, so only
rankings inside one town are comparable. No published integration values for a historic core
were found that a generator could use as targets.

**Which metrics best separate organic from planned.** By the sources above: (1) junction mix
(P(3) vs P(4)) and dead-end share, (2) median segment length / street length per hectare,
(3) orientation order φ, which separates single grids from everything else, (4) block shape
factor distribution, which Barthelemy et al. 2013 found is where planning shows when simple
counts do not. Average degree alone is weak: Paris stayed at 3.0 through Haussmann.

**Cheapest set with the most signal:** junction mix, ⟨k⟩, median segment length, φ, circuity,
meshedness. All linear-time, all deterministic given a fixed graph, and all published for real
cities, so the generator's output can be placed next to real values.

### What this means for a generator

- **Default junction mix for an organic town: more T than X, plenty of dead ends.** Target
  P(4) below P(3); for a Venice-like or medina-like town aim for 15–30 % dead ends and 7–15 %
  4-way nodes (Boeing 2019 values for whole municipalities, used as proxies; no historic-core node
  measurements were found). A gridded "planned" ward targets 4-way
  share above 30 % and dead ends below 8 %.
- **Use real widths as rule-pack data, per culture.** Minimums: Roman 8 ft (~2.4 m); Islamic
  7 cubits (3.5–5.6 m depending on the cubit) for through-streets, 2–3.5 m alleys (Merv); a
  medieval European main street 5–6 m with 2–3.5 m lanes is an extrapolation from Merv and
  Pompeii, not a European measurement (no European measured series was found). East Asian capitals use a
  three-step table: ~12 m / 24 m / 85–150 m on a ~121 m (Heian) or ~530 m (Chang'an) block.
- **Snap junction angles to two modes.** When a new street meets an old one, prefer ~90°
  (±15°) for short side streets and a near-straight continuation (180° ± 15°) for long through
  routes (Hillier 1997). This gives the "deformed grid" look without a grid.
- **Build in hierarchy order and freeze the backbone.** Main roads (gate to market) first and
  never moved; densification fills between them; exploration extends outward (Strano 2012).
  This matches Caniggia's matrix / building / connecting routes in research-academic.md and
  fits the pin rule (landmark N+1 never moves 1..N).
- **Ship a metrics report with every map.** Compute junction mix, ⟨k⟩, median segment length,
  street m/ha, φ, circuity, meshedness and block-shape histogram per run, and print them next
  to the matching row from the tables above (Venice, Cairo, Bologna, Rome, Kyoto, Manhattan).
  This is the corpus comparison research-academic.md §2.9 asks for, with real numbers.
- **Age a street by narrowing it.** Encroachment is documented in Merv (6 m → 5 m) and in
  Heian-kyō (kosho). An "age" spec field can narrow older streets and add small kinks
  deterministically.

### Sources

Fetched in this session:

- Boeing, G. 2019. Urban Spatial Order: Street Network Orientation, Configuration, and Entropy. Applied Network Science 4:67. https://arxiv.org/abs/1808.00600 and https://arxiv.org/pdf/1808.00600
- Barthelemy, M., Boeing, G. 2024. A review of the structure of street networks. https://arxiv.org/pdf/2409.08016
- Strano, E., Nicosia, V., Latora, V., Porta, S., Barthelemy, M. 2012. Elementary processes governing the evolution of road networks. Scientific Reports 2:296. https://arxiv.org/pdf/1203.0300
- Barthelemy, M., Bordin, P., Berestycki, H., Gribaudi, M. 2013. Self-organization versus top-down planning in the evolution of a city. Scientific Reports 3:2153. https://ar5iv.arxiv.org/html/1307.2203
- Louf, R., Barthelemy, M. 2014. A typology of street patterns. J. R. Soc. Interface 11:20140924. https://arxiv.org/abs/1410.2094 and https://arxiv.org/pdf/1410.2094
- Cardillo, A., Scellato, S., Latora, V., Porta, S. 2006. Structural properties of planar graphs of urban street patterns. https://arxiv.org/pdf/physics/0510162
- Hillier, B. 1997. The hidden geometry of deformed grids. Space Syntax 1st Symposium. https://www.spacesyntax.net/symposia-archive/SSS1/SpSx%201st%20Symposium%2097%20-2003%20pdf/1st%20Symposium%20Vol%20III%20pdf/35%20Hillier%20300.pdf
- Kulzhanova, A. 2025. Fractal and network-based analysis of historic and modern street patterns across European cities (master's thesis abstract, Politecnico di Torino). https://webthesis.biblio.polito.it/39015/8/Kulzhanova_abstract_EN.pdf
- Williams, T. The landscapes of Islamic Merv, Turkmenistan. Internet Archaeology 25. https://intarch.ac.uk/journal/issue25/1/index.html and §8.3 https://www.intarch.ac.uk/journal/issue25/1/8_3.html
- CSBE, Islamic building legislation (Hakim). https://www.csbe.org/legislation
- Hadith on road width, Silsilah as-Sahihah 1218. https://en.tohed.com/hadith/silsilah-sahihah/chapter/483/sub/15488/
- The Case of Al-Fina' in Cairo (NTNU publication, p. 161; author not shown on page). https://ntnu.no/trykk/publikasjoner/hosiar_nooraddin/HTML/files/assets/basic-html/page-191.html
- Wikipedia, Roman roads. https://en.wikipedia.org/wiki/Roman_roads
- Designing Buildings Wiki, The history of the dimensions and design of roads, streets and carriageways. https://www.designingbuildings.co.uk/wiki/The_history_of_the_dimensions_and_design_of_roads%2C_streets_and_carriageways
- Madain Project, Via dell'Abbondanza. https://madainproject.com/via_dell_abbondanza
- Pompeii Perspectives, Insula attributes. https://pompeiiperspectives.org/index.php/insula-attributes
- Wikipedia, Heian-kyō. https://en.wikipedia.org/wiki/Heian-ky%C5%8D
- Kyoto Prefecture, Hyakugo archive, Roads turned into farm fields. https://hyakugo.pref.kyoto.lg.jp/eng/?p=294
- JAANUS, machi. https://aisf.or.jp/~jaanus/deta/m/machi.htm
- Shaanxi Provincial Government, Chang'an. https://en.shaanxi.gov.cn/as/hac/hos/201704/t20170428_1595612.html
- China Daily via chinaservicesinfo, Unearthing life in the mists of time. https://www.chinaservicesinfo.com/s/202605/19/WS6a0bc472498e36855033ca59/unearthing-life-in-the-mists-of-time.html
- Wikipedia, Rites of Zhou. https://en.wikipedia.org/wiki/Rites_of_Zhou
- Bologna Welcome, Top 6 facts about Bologna's porticoes. https://bolognawelcome.com/en/blog/top-6-facts-about-bolognas-unesco-porticoes
- Noorthouck 1773, A New History of London, Book 1 Ch. 15 (British History Online). https://www.british-history.ac.uk/node/39344

### Not found

- **Measured dead-end share for any Islamic historic core** (Fez, Tunis, Aleppo, Damascus, Cairo,
  Yazd). Sources describe cul-de-sacs as the majority street type in Cairo but give no
  percentage. Boeing's Cairo, Damascus and Jerusalem values are whole-city, not old-core.
- **Measured street widths for a medieval European town by class** (e.g. York, Winchester,
  Freiburg, Monpazier). Statutes regulating widths are documented for Siena (1262), Bologna
  and Todi, but no width figure was found; the Siena paper is a scanned PDF with no text layer.
- **Exact London 1667 widths per street class.** They are in Noorthouck's Appendix XLIX, which
  was not on the fetched page. The 14 ft lane figure is unverified.
- **Pompeii street widths from a primary survey.** The 2008 hand measurements exist but the
  tables were not on the fetched page. The 2.4–4.5 m range is unverified.
- **Timgad insula and street dimensions.** Sources conflict (20 m vs ~21 m blocks); not
  settled.
- **Marshall's A/B/C/D types with numeric thresholds.** The types (A "Altstadt" irregular, B
  "Bilateral" grid, C mixed, D "Distributory" loop-and-cul-de-sac) are described in search
  snippets (unverified) that came from what looks like an unauthorised copy of the book, so I
  did not use them. Barthelemy & Boeing 2024 confirm only that Marshall proposed a typology. No
  numeric T/X/cul-de-sac cut-offs were found.
- **Space Syntax integration values for a named historic core** usable as targets.
- **Share of town area that is street or open space** in a measured pre-industrial town. Only
  the Heian-kyō figure above, computed from the plan, is given.
- **Orientation entropy computed on historic cores only** (as opposed to whole municipalities).
