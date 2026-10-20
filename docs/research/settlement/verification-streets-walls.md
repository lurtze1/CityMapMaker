## Verification: streets and walls

Checked 2026-10-20 against `research/settlement:docs/research/settlement/streets.md` and
`walls-growth.md`. Primary PDFs (Boeing 2019, Cardillo 2006, Strano 2012, Barthelemy 2013,
Hillier 1997) were downloaded and read as text; they are in
a local scratch folder (not committed).
"(my calc)" marks arithmetic I did; the formula is given in the notes.

Verdicts: **VERIFIED** = the value is in the primary source or in two independent sources.
**CORRECTED** = the report's value or wording is wrong, or a better source disagrees; both
values are given. **UNVERIFIABLE** = I could not reach a primary or second independent source.

### 1. Boeing 2019, Applied Network Science 4:67 (arXiv 1808.00600, accepted version, Table 1)

| Claim | Report value | Verified value | Source | Verdict |
|---|---|---|---|---|
| Venice row (φ, Hₒ, l, ς, ⟨k⟩, P_de, P_4w) | 0.017, 3.564, 23.2 m, 1.090, 2.474, 0.300, 0.073 | identical | Boeing 2019 Table 1 | VERIFIED |
| Manhattan row | 0.669, 2.650, 82.2, 1.017, 3.508, 0.027, 0.572 | identical | same | VERIFIED |
| Jerusalem, Rome, Damascus, Prague, London, Lisbon, Istanbul, Cairo, Kyoto, Paris, Barcelona, Chicago rows | as in report | all 12 rows identical, digit for digit | same | VERIFIED |
| Venice, Mogadishu, Helsinki, Jerusalem, Casablanca have the shortest median segments | as stated | stated in text | same | VERIFIED |
| European cities: highest entropy and dead-end share, lowest ⟨k⟩ | as stated | stated in text | same | VERIFIED |
| "Rome has the highest orientation entropy of the non-US cities" (Finding 4) | Rome φ 0.005 | São Paulo is higher (φ 0.002, Hₒ 3.581); paper: "Charlotte ... São Paulo and Rome immediately follow" | same | CORRECTED: Rome is second among non-US cities, after São Paulo |
| Chicago lowest, φ = 0.90 | 0.899 / 0.90 | 0.899 in table, "0.90" in text | same | VERIFIED |

### 2. Cardillo, Scellato, Latora, Porta 2006, Phys Rev E 73:066107 (arXiv physics/0510162, Tables I and II)

| Claim | Report value | Verified value | Source | Verdict |
|---|---|---|---|---|
| Ahmedabad N, K, Cost, l, M | 2870, 4387, 121037, 27.59, 0.262 | identical | Table I, II | VERIFIED |
| Venice | 1840, 2407, 75219, 31.25, 0.152 | identical | same | VERIFIED |
| Cairo | 1496, 2255, 84395, 37.47, 0.253 | identical | same | VERIFIED |
| Bologna | 541, 773, 51219, 66.26, 0.214 | identical | same | VERIFIED |
| New York | 248, 419, 36172, 86.33, 0.348 | identical | same | VERIFIED |
| Seoul, London, Vienna, Paris, Savannah, Barcelona, Irvine 1 | as in report | identical | same | VERIFIED |
| San Francisco | not in report table | 169, 271, 38187, 140.91, M 0.309; ⟨k⟩ 3.21, 147 m/ha (my calc) | same | added |
| Washington | not in report table | 192, 303, 36342, 119.94, M 0.293; ⟨k⟩ 3.16, 140 m/ha (my calc) | same | added |
| ⟨k⟩ = 2K/N column | 3.06, 2.62, 3.01 ... | same values (my calc) | from Table I | VERIFIED |
| Street m/ha = Cost / 259 ha | 467, 290, 326, 198, 140 ... | 1 sq mi = 258.999 ha; 467.3, 290.4, 325.9, 197.8, 139.7 (my calc) | from Table I | VERIFIED |
| P(k=3) > P(k=4) in Ahmedabad, Bologna, Cairo, Venice; reverse in New York, San Francisco, Washington | as stated | stated in text (Fig. 2) | Cardillo 2006 | VERIFIED |
| Type labels | Irvine 1 "planned suburb", "suburban Irvine"; Barcelona "planned"; London, Vienna "—" | Irvine 1 is "an industrial area showing enormous blocks" (class "modernist"); Irvine 2 is the "lollipop" suburb. Barcelona is "grid-iron". London, Vienna, Bologna, Venice are "medieval fabrics (european)"; Ahmedabad, Cairo "medieval (arabic)"; Paris and Seoul "mixed"; Washington and New Delhi "baroque" | same | CORRECTED (labels only) |

### 3. Hillier on junction angles

| Claim | Report value | Verified value | Source | Verdict |
|---|---|---|---|---|
| Junctions "within about 15° of 90°" or "within about 15° of a direct 180° continuation" | as stated | exact wording found: "a surprisingly high proportion are either near right angle connections, usually within about 15° of 90°, or very obtuse angles, usually within about 15° of a direct 180° continuation" | Hillier 1997, "The hidden geometry of deformed grids", Space Syntax 1st Int. Symposium, Vol. III, paper 35, §2.1 | VERIFIED, with limits below |
| Long lines end near straight, short lines near right angles | as stated | stated in same section | same | VERIFIED |
| The angle pattern holds "even in the most tortuous cul-de-sac sequences" of an Islamic town | Finding 3 | that sentence (§4.2) is about the *linear principle* (buildings in rows along lines), not about the 15° angle statistics | same | CORRECTED (misattributed) |
| Planned grid: "almost all ~90°" (metrics table) | as stated | not in Hillier; the text says intersections where neither line ends on a building are "the vast majority approximating a right angle" *inside the deformed grid* | same | CORRECTED (report's own inference, not a source finding) |
| Hillier & Hanson as source | requested in brief | the statement is in Hillier 1997 only; I did not find it in Hillier & Hanson 1984 | — | the citation should be Hillier 1997 |

### 4. Street widths

| Claim | Report value | Verified value | Source | Verdict |
|---|---|---|---|---|
| Roman, Twelve Tables: 8 ft straight, 16 ft on bends | "any public road (via), minimum" | Latin text confirmed: "Viae latitudo ex lege duodecim tabularum in porrectum octo pedes habet, in anfractum, id est ubi flexum est, sedecim." It sits in Digest 8.3 *De servitutibus praediorum rusticorum*: the width of a **right-of-way servitude** across private land, not of public streets. Festus: private ways 8 ft "by law", public ways as wide as public use needs. Varro, LL 7.15 repeats 8/16. 8 × 0.296 m = 2.37 m (my calc) | Digest 8.3.8 (Gaius), thelatinlibrary.com; Právník 12/2024 (Czech law journal) quoting Varro and Festus | CORRECTED (numbers right, meaning wrong: not a public-street minimum) |
| Islamic: 7 cubits when neighbours dispute a road's width | 7 cubits | "When you disagree about a path, its breadth should be made seven cubits"; Bukhari 2473 and Muslim 1613, also Ibn Majah 2338, Tirmidhi 1356 | hadith collections via tohed.com and hadithunlocked.com | VERIFIED |
| Merv: streets c. 3–4 m, some 5 m; alleys 2–3.5 m; one street 6 m narrowed to c. 5 m; T-junctions and staggered crossroads | as stated | all quoted exactly. Note: the 3–4 m and 2–3.5 m are estimates hedged with "perhaps" (air-photo interpretation); only the 6 m → 5 m street was excavated | Williams, Internet Archaeology 25, §8.3 | VERIFIED |
| Heian-kyō Suzaku-ōji 28 jō | ~85 m | 28 jō, "nearly 85 m" (Kyoto Prefecture); "about 83 m" (Kyoto City Archaeological Research Institute) | hyakugo.pref.kyoto.lg.jp; kyoto-arc.or.jp leaflets | VERIFIED (83–85 m) |
| Heian-kyō ōji 8 jō / 24 m | 24 m | 8 jō standard, but the Engishiki *Kyōtei* lists the avenue south of the palace at 17 jō (~51 m), and other ōji at 10 or 12 jō | Kyoto Prefecture; Engishiki transcription via search (OCR-garbled) | CORRECTED: 24 m is the common ōji; not all ōji |
| Heian-kyō kōji 4 jō / 12 m | 12 m | 4 jō, 12 m | Kyoto Prefecture; Engishiki "小路...広各四丈" | VERIFIED |
| Heian-kyō block 40 jō | ~121 m | 40 jō, "about 120 m" square | Kyoto City Archaeological Research Institute | VERIFIED (120–121 m) |
| Chang'an Zhuque Avenue | 150 m | 150–155 m from 1950s–60s surveys; 129.32 m measured near Mingde Gate | Xi'an Institute of Cultural Heritage (via search); Shaanxi gov | VERIFIED as 150–155 m, with a local 129 m reading |
| Chang'an ward size | ~1 li ≈ 530 m; Zhengpingfang 533.6 × 464.6 m | wards were of unequal size ("各坊面积不一"); 530 m is the textual norm; Zhengpingfang measurement confirmed | China Daily (2026); Xi'an archaeology institute (via search) | VERIFIED for Zhengpingfang; CORRECTED: "about 530 m" is not the size of every ward |
| Chang'an ward count | 108 | 108, 109, 110 and 120 all appear in sources | same | conflict, unresolved |

### 5. Walls

| Claim | Report value | Verified value | Source | Verdict |
|---|---|---|---|---|
| Conwy length | 1.3 km | 1.3 km (Wikipedia); "three quarters of a mile" = 1.21 km (my calc) (Cadw, the Welsh heritage body; castlewales) | Cadw visitor page; castlewales.com | CORRECTED: heritage body gives ~1.2 km |
| Conwy area | 10 ha | 10 ha (Wikipedia, medievalheritage.eu) | secondary only | VERIFIED (secondary). Geometry check: a 1.3 km square loop holds 10.6 ha, an equilateral triangle 8.1 ha (my calc), so 10 ha is near the ceiling |
| Conwy gates, towers | 3 gatehouses + 2 posterns; 21 towers ~46 m apart | same | Cadw; castlewales | VERIFIED |
| Florence length | 8.5 km | 8.5 km (two sources); "over 8 km" (Petrioli) | castellitoscani; fua.it; risorgimentofirenze.it | VERIFIED |
| Florence area | 430 vs 650 ha | **650 ha is impossible**: the largest area any 8.5 km loop can hold is a circle of 575 ha (my calc: P² / 4π). 430 ha fits | geometry | RESOLVED: 430 ha |
| Florence gates and towers | 12 gates, 63 towers (or 15 and 73) | both pairs found; no primary source | castellitoscani; visittuscany | UNVERIFIABLE (conflict stands) |
| Cologne Roman wall area | 25 ha (kuladig) vs 96.8 ha | City of Cologne statistics: "Römerstadt innerhalb der ehemaligen Römermauer (97 ha)". Wikipedia: ~97 ha. A 3.9 km square loop holds 95 ha (my calc). Kuladig does print "rund 25 Hektar", which is wrong | Stadt Köln, *Stadtgebiet* statistics PDF (2006); Wikipedia CCAA | CORRECTED: 97 ha |
| Cologne sequence | 25 → 122 → 223 → 401 ha | 97 (Roman) → 122 (c. 940–950s, plus Rhine suburb) → 223 (1106 wall, by 1141) → 401 ha (1180 wall). Then 770 ha (1794–1883) and 1,006 ha (Prussian ring 1883–88) | Stadt Köln statistics; kuladig for 122/223/401 | CORRECTED (first step) |
| Cologne growth ratios | "1.8 to 4.9× per ring" | 1.26×, 1.83×, 1.80× (my calc) | from above | CORRECTED: 1.3 to 1.8× |
| Cologne Roman wall | almost 4 km, ≥9 gates, 19 towers | same | Römisch-Germanisches Museum | VERIFIED |
| Cologne 1180 wall gates; length | 12 gates; "over 8 km" | 12 gates (kuladig). Length: the land-side half-ring is ~7.5 km (dewiki snippet); full length not found | kuladig; dewiki Gereonstor (snippet) | gates VERIFIED; length UNVERIFIABLE |
| Paris, Philip Augustus area | 253 ha | 253 ha (Wikipedia, techno-science.net, structurae) vs **273 ha** (French Ministry of Culture, Mérimée PA00086018) | pop.culture.gouv.fr | conflict: the heritage body says 273 ha |
| Paris PA gates | 15 main (11 at first) | Mérimée: 13 gates | same | conflict |
| Paris PA towers | 77 at 60 m | 77 towers, every 60 m (39 right bank, 38 left bank); ~5.1 km | techno-science.net; pariszigzag | VERIFIED |
| Paris, Charles V area | 440 ha "right bank only" | Wikipedia: "The city then spread over 440 hectares ... more than 150,000 inhabitants", uncited; the sentence describes the whole city, not the right bank alone. No French heritage source found | Wikipedia only | UNVERIFIABLE; "right bank only" label CORRECTED |
| Nördlingen | 1327 wall, 4× the walled area; length, gates not given | 4× confirmed; length 2.6–2.7 km; 5 gates, 11 towers | Wikipedia; muenchen.travel; burgenwelt.org | VERIFIED, and gaps filled (secondary) |
| Xi'an length, area | 13.74 km, 11.52 km² | 13.74 km (Xinhua, several reports); 11.52 km² (Wikipedia citing Xi'an Qujiang New District committee). A rectangle with that perimeter and area is 3.96 × 2.91 km (my calc), which fits the city's shape | news.cn; Wikipedia | length VERIFIED; area VERIFIED (single cited secondary) |
| Xi'an rampart spacing | 120 m, "within the range of an arrow shot" | "Ramparts are built at intervals of 120 metres" (no citation on Wikipedia); the arrow-shot quote was not on the page. 98 ramparts on 13.74 km would be 140 m (my calc) | Wikipedia | UNVERIFIABLE |
| Xi'an moat | 18 m wide, 6 m deep | same | Wikipedia citing Bushell 2012 | VERIFIED (secondary) |
| York length | 3.4 km or 4.0 km | 3.4 km = 2⅛ miles, the walled stretches only, gaps for water omitted (VCH via search, VisitEngland). Full length 4,840 yards = 4.43 km (my calc) (Victoria County History, *City of York*, 1961). Jorvik: 2.75 miles incl. gaps | british-history.ac.uk VCH York pp. 510–520; visitengland.com | CORRECTED: 3.4 km walled, ~4.4 km full circuit |
| York area | 106 ha | 263 acres = 106.4 ha (my calc) (VCH and Jorvik via search). Geometry: 106 ha cannot fit inside 3.4 km (max 92 ha), fits 4.4 km | search snippets | VERIFIED with the 4.4 km length |
| York gates | 4 main bars + 2 minor | VCH: four bars "on the four quarters", a fifth (Fishergate) probably added later | VCH | VERIFIED (4 + Fishergate) |
| Ávila | 2,516 m, 9 gates, 88 towers, 31 ha, 28.6 m spacing | 2,516 m and 9 gates agreed; towers 87 (official tourism, per El Español) or 88; area 31 ha or 33 ha (hoteles.net). Spacing 28.6–28.9 m (my calc: 2,516 / 88 or / 87) | elespanol.com; planomato; hoteles.net | VERIFIED for length and gates; towers and area in conflict |
| Kano length | "eleven miles" (Johnston) | 11 miles = 17.7 km (my calc). UNESCO tentative list: "a 14km radius earth structure" (wording ambiguous) | whc.unesco.org tentative list 5171 (via search) | conflict: 14 km (UNESCO) vs 17.7 km (Johnston) |
| Kano gates | 13 (Johnston) or 15 | 15 gates (Daily Trust; UNESCO-related listing) | dailytrust.com | 15 VERIFIED for today's count; 13 is Johnston's historic count |

### 6. Strano et al. 2012 and Barthelemy et al. 2013

| Claim | Report value | Verified value | Source | Verdict |
|---|---|---|---|---|
| Groane 125 km², 29 centres; nodes 255 (1833) → >5000 (2007) | as stated | same | Strano et al. 2012, Sci Rep 2:296 (arXiv 1203.0300) | VERIFIED |
| ⟨k⟩ 2.57 → 2.8 (1914 → 1980) | as stated | same | same | VERIFIED |
| N4/N 11% → 15.5%; r_N 0.87 → 0.835 | as stated | same | same | VERIFIED |
| T-junctions typical of organic networks like Venice or Cairo | quoted | exact quote found (citing Cardillo) | same | VERIFIED |
| Shape factor: one peak ~0.5, SD 0.25 before 1933; second peak 0.62 after 1955 | as stated | same | same | VERIFIED |
| σ/μ 0.5 → 0.26; τ 1.2 → 1.9 ± 0.1 | as stated | same | same | VERIFIED |
| Stable high-centrality backbone; densification and exploration | as stated | abstract states it | same | VERIFIED |
| Paris ⟨k⟩ ≈ 3.0 constant since 1789 | as stated | "the average degree is essentially constant ⟨k⟩ ≈ 3.0 since 1789" | Barthelemy, Bordin, Berestycki, Gribaudi 2013, Sci Rep 3:2153 (arXiv 1307.2203) | VERIFIED |
| Haussmann ~6% of length | as stated | "Haussmann's new roads and avenues represent approximately 6% of the total length only (compared to nowadays network)" | same | VERIFIED |
| Nodes ~3000 → ~6000, length ~400 → almost 700 km (1836 → 1888) | as stated | same | same | VERIFIED |
| Block shape: centred near 0.5 before, "much flatter" after Haussmann | as stated | same | same | VERIFIED |
| Scope | "Paris" | all Paris numbers are for the 1789 city area only, not today's Paris | same | note |

### Notes

- **Boeing is a whole-municipality, OpenStreetMap 2018 measure.** The paper does not state the
  network type (drivable or all streets) in the arXiv text. Venice's row covers the whole
  comune, Mestre included. Use the numbers as context, not as an old-core target.
- **Cardillo is the better old-core target.** Its samples are 1-square-mile cut-outs from Allan
  Jacobs's *Great Streets* maps, not OSM. All counts, lengths and meshedness values match.
  Meshedness recomputed from the paper's own formula with N and K gives values within 0.003
  of Table II (e.g. Ahmedabad 0.265 vs 0.262); I do not know why they differ slightly.
- **Geometry as a check.** A closed loop of length P cannot enclose more than P² / 4π. That rule
  settles Florence (650 ha impossible) and shows York's 106 ha needs the full 4.4 km circuit.
  A generator can use the same check on its own walls.
- **Calibration consequence.** The Cologne curve becomes 97 → 122 → 223 → 401 ha
  (×1.26, ×1.83, ×1.80). The report's "1.8 to 4.9×" rested on the wrong 25 ha.
- **Roman 8 ft is a servitude width.** It sets the minimum of a private right of way, not a public
  street. A rule pack can still use 2.4 m as the narrowest lane, but should not label it
  "Roman public street minimum".
- **Not checked** (outside this brief): Barthelemy & Boeing 2024 ranges, Kulzhanova 2025,
  Pompeii widths, the 49.5 cm cubit, Prague, Vienna, Moscow, Bristol, Caernarfon and the other
  English list rows.
- Sources unreachable: Springer full text of Boeing 2019 (login redirect), Schäfer 2025 on
  Roman Cologne (bot wall; a search snippet says ~99 ha), UNESCO Kano page (403), VCH York
  area sentence (only via search snippet).
