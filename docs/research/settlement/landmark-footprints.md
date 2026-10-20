## Landmark footprints, measured

Research for CityMapMaker, 2026-10-20. Question: are the guessed footprint classes
(small 10 x 10 m, medium 20 x 20 m, large 40 x 40 m, huge 60 x 60 m) and the default class
per landmark kind right for real pre-1900 towns?

Terms used:
- **Footprint**: the area a thing covers on the ground, seen from above.
- **Building vs compound**: a building is one roofed structure. A compound (also "precinct")
  is a walled or fenced area holding several buildings and open courts, for example an abbey,
  a temple enclosure or a castle with its courtyards.
- **Bailey / ward**: the walled courtyard of a castle. A **motte** is the earth mound a
  wooden or stone tower stood on. A **keep** is the main strong tower of a castle.
- **Nave**: the long main body of a church. **Transept**: the cross arms.
- **Caravanserai / khan**: a walled inn for merchants and their animals, built around a courtyard.
- **Madrasa**: an Islamic college, usually rooms around a courtyard.
- **Cavea**: the seating bowl of a Roman theatre.

Source labels: "(Wikipedia)" marks a Wikipedia page. "(mine)" marks arithmetic I did
on a fetched number. "(unverified)" marks a number I saw only in a search-engine summary
and could not fetch myself. Every other number was read on the fetched page linked in the
Sources list.

### Findings

1. **Sizes span four orders of magnitude.** A market-cross shaft covers a few square metres;
   a Friday mosque or cathedral covers 8,000-25,000 m2; a temple precinct or naval arsenal
   covers 5-270 hectares. Four square classes from 100 m2 to 3,600 m2 cover only the middle.
2. **The guessed small, medium and large classes hold up.** Towers measure 110-190 m2
   (guess: 100). Halls and keeps measure 390-1,150 m2 (guess: 400). Big parish churches and
   royal halls measure 1,500-1,900 m2 (guess: 1,600).
3. **The guessed huge class is too small.** 60 x 60 m is 3,600 m2. Measured cathedrals are
   7,900-10,900 m2, Friday mosques 9,000-26,000 m2, palaces 11,000-12,000 m2.
4. **A class below "small" is missing.** Shrines, wells, crosses and columns measure
   5-65 m2. A 10 x 10 m default makes them visibly too big on a map.
5. **Squares are wrong for many kinds.** Halls and longhouses are 3-9 times longer than
   wide (Borg 83 x 9.5 m, Lejre 48 x 11 m). Churches are 2-4 times longer than wide. Ropewalks
   are 15 times longer than wide. A per-kind aspect ratio (length divided by width) matters
   as much as the area.
6. **Many "landmarks" are compounds.** Abbeys, Indian and Chinese temple complexes, castles,
   colleges, caravanserais and arsenals are walled precincts with courts. Their outer outline
   is far larger than any one roofed building in them.
7. **The biggest compounds are districts, not footprints.** Fountains Abbey precinct 28 ha,
   Venetian Arsenal 45 ha, Temple of Heaven about 270 ha. These should be modelled as a
   district or ward of the city, not as one landmark polygon.

### Measured footprints

Areas marked "(mine)" are length x width, or pi x r squared for round shapes, computed by me.
"Building" = one roofed structure. "Compound" = walled precinct with several buildings or courts.

| Kind | Example | Culture | Era | Length x width (m) | Area (m2) | Building or compound | Source |
|---|---|---|---|---|---|---|---|
| Cathedral | Chartres Cathedral | French Gothic | 1194-1220 | 130 x 32 / 46 (two widths given, unlabelled; likely nave-with-aisles and transept) | 10,875 (ground area) | Building | [S1] (Wikipedia) |
| Cathedral | Cologne Cathedral | German Gothic | 1248-1880 | 144.58 x 86.25 | 7,914 (building area) | Building | [S2] (Wikipedia) |
| Cathedral | Salisbury Cathedral | English Gothic | 1220-1258 | 135 long; width not found | not found | Building (in Britain's largest close, size not found) | [S3] (Wikipedia) |
| Cathedral / mosque | Hagia Sophia | Byzantine | 532-537 | 82 x 73 | 5,986 (mine) | Building | [S4] (Wikipedia) |
| Large parish church | St Mary Redcliffe, Bristol | English Gothic | 13th-15th c. | 76 x 36 (across transepts) | 1,916 | Building | [S5] (Wikipedia) |
| Large parish church | Hull Minster (Holy Trinity) | English Gothic | c.1285-1520 | 87 long; width not found | not found | Building | [S6] (Wikipedia) |
| Small parish church | Escomb Church | Anglo-Saxon | 7th c. | nave 13.3 x 4.4 internal, chancel 3.0 x 3.0 internal; about 16.3 m internal length (mine) | about 68 internal (mine) | Building | [S7] (Wikipedia) |
| Monastery church (plan) | Plan of Saint Gall basilica | Carolingian | c.820 (ideal plan) | 91.44 apse to apse x about 24 (nave 12 + two aisles 6) (width mine) | about 2,200 (mine) | Building | [S8] (Wikipedia) |
| Abbey precinct | Fountains Abbey | English Cistercian | 1132-1539 | church about 91 long; precinct 28 ha | 280,000 precinct | Compound | [S9] (Wikipedia) |
| Abbey building | Fountains Abbey infirmary hall | English Cistercian | 12th-13th c. | 52 x 21 | 1,092 (mine) | Building | [S9] (Wikipedia) |
| Chapel | Fountains Abbey infirmary chapel | English Cistercian | 12th-13th c. | 14 x 7 | 98 (mine) | Building | [S9] (Wikipedia) |
| Friday mosque | Great Mosque of Córdoba | Umayyad Andalusi | 785-987 | 180 x 130 | 23,400 (mine) | Compound (prayer hall + courtyard) | [S10] (Wikipedia) |
| Friday mosque | Great Mosque of Kairouan | Aghlabid Tunisia | 9th c. | sides 127.6 / 125.2 / 72.7 / 78 | over 9,000 | Compound (prayer hall + courtyard) | [S11] (Wikipedia) |
| Mosque prayer hall | Great Mosque of Kairouan | Aghlabid Tunisia | 9th c. | 70.6 x 37.5 | 2,648 (mine) | Building | [S11] (Wikipedia) |
| Friday mosque | Mosque of Ibn Tulun, Cairo | Tulunid Egypt | 876-879 | not given | 26,318 (with outer enclosure) | Compound | [S12] (Wikipedia) |
| Neighbourhood mosque | Mosque of Cristo de la Luz, Toledo | Andalusi | 999 | 8 x 8 | 64 (mine) | Building | [S13] (Wikipedia) |
| Minaret | Great Mosque of Kairouan minaret | Aghlabid Tunisia | 9th c. | 10.7 x 10.7 | 114 (mine) | Building | [S11] (Wikipedia) |
| Hindu temple compound | Brihadisvara Temple, Thanjavur | Chola, India | 1010 | 240.79 x 121.92 | 29,357 (mine) | Compound | [S14] (Wikipedia) |
| Hindu temple tower | Brihadisvara vimana base | Chola, India | 1010 | 30.18 x 30.18 | 911 (mine) | Building | [S14] (Wikipedia) |
| Hindu temple | Kandariya Mahadeva, Khajuraho | Chandela, India | c.1030 | 31 x 20 | 620 (mine) | Building | [S15] (Wikipedia) |
| Hindu temple compound | Meenakshi Temple, Madurai | Tamil, India | 16th-17th c. | main court about 240 per side | 57,000 (5.7 ha) | Compound | [S16] (Wikipedia) |
| Buddhist hall | Tōdai-ji Daibutsuden, Nara | Japanese | rebuilt 1709 | 57 x 50 | 2,850 (mine) | Building | [S17] (Wikipedia) |
| Buddhist hall | Hōryū-ji Kondō, Nara | Japanese | c.7th c. | 18.5 x 15.2 | 281 (mine) | Building | [S18] (New World Encyclopedia) |
| Shrine | Ise Naikū main shrine building | Japanese Shinto | rebuilt every 20 years | 10.9 x 5.5 | 60 (mine) | Building | [S19] (Wikipedia) |
| Chinese temple compound | Temple of Confucius, Qufu | Chinese | Ming-Qing | axis about 1,300 long | 16,000 (as stated; looks small for a 1.3 km axis, treat with care) | Compound | [S20] (Wikipedia) |
| Chinese temple hall | Dacheng Hall, Qufu | Chinese | 1724 | 54 x 34 | 1,836 (mine) | Building | [S20] (Wikipedia) |
| Chinese temple compound | Temple of Heaven, Beijing | Chinese Ming | 1420 | not given | 2.73 km2 or 267 ha (page gives both) | Compound (park) | [S21] (Wikipedia) |
| Chinese temple hall | Hall of Prayer for Good Harvests | Chinese Ming | 1420 (rebuilt 1890) | 36 diameter | 1,018 (mine) | Building | [S21] (Wikipedia) |
| Tower keep | White Tower, Tower of London | Norman | c.1078 | 36 x 32 (base, without corner towers) | 1,152 (mine) | Building | [S22] (Wikipedia) |
| Tower keep | Rochester Castle keep | Anglo-Norman | 1127 | 21 x 21 (external) | 441 (mine) | Building | [S23] (Wikipedia) |
| Motte | Mottes in general | Norman, Europe | 11th-12th c. | 30-90 diameter | 707-6,362 (mine) | Earthwork | [S24] (Wikipedia) |
| Motte-and-bailey castle | Hen Domen, Montgomery | Norman Welsh March | c.1070 | 50 x 40 oval site; bailey 0.13 ha | about 1,571 site (mine, ellipse); 1,300 bailey | Compound | [S25] (Wikipedia) |
| Motte-and-bailey castle | Heywood Castle, Devon | Norman | 11th-12th c. | bailey 60 x 40, motte 48.7 across | 2,400 bailey (mine) | Compound | (unverified) Historic England entry 1020959 reproduced in a PDF that I could not read [S26] |
| Edwardian castle | Beaumaris Castle, inner ward only | English, Wales | 1295 | not given | 3,000 (0.30 ha) inner ward only | Compound | [S27] (Wikipedia) |
| Royal great hall | Westminster Hall, London | Anglo-Norman | 1097-1099 | 73 x 20 (unverified, search summary of parliament.uk) | about 1,547 | Building | [S28] (Wikipedia) |
| Palace | Palais des Papes, Avignon | French Gothic | 1335-1364 | not given | 11,000 site (15,000 floor space) | Compound | [S29] (Wikipedia) |
| Palace hall | Salone dei Cinquecento, Palazzo Vecchio | Florentine | 1494 | 52 x 23 | 1,196 (mine) | Room inside building | [S30] (Wikipedia) |
| Guildhall | Merchant Adventurers' Hall, York | English | 1357-1361 | about 30 x 13 | 390 (mine) | Building | [S31] (local guide) |
| Market / cloth hall | Cloth Hall, Ypres | Flemish | 1200-1304 | 125 frontage; depth not found | not found | Building | [S32] (Wikipedia) |
| Viking hall | Lejre royal hall | Danish Viking | 9th-10th c. | 48 x 11 | 528 (mine) | Building | [S33] (National Museum of Denmark) |
| Chieftain's longhouse | Borg, Lofoten | Norse | c.500-950 | 83 x 9.5 | 789 (mine) | Building | [S34] (Wikipedia) |
| Town house | Viking Dublin "Type 1" house | Hiberno-Norse | 10th-11th c. | not found | about 36 (unverified, low-quality site) | Building | search summary only |
| Inn courtyard | King's Head, Aylesbury, courtyard | English | 15th-16th c. | 14 x 20 | 280 (mine) courtyard only | Compound (four ranges round a yard) | [S35] (Buckinghamshire HER) |
| Inn chamber | New Inn, Gloucester, upper chambers | English | c.1450 | 7.9 x 4.7-5.9 | 37-47 (mine) | Room | [S36] (listing text mirror) |
| Caravanserai | Sultan Han, Aksaray | Seljuk Anatolia | 1229 | courtyard 44 x 58 | 4,900 total; 2,552 courtyard (mine) | Compound | [S37] (Wikipedia) |
| Urban khan | Khan As'ad Pasha, Damascus | Ottoman Syria | 1752 | not given | 2,500 | Building (covered court) | [S38] (Wikipedia) |
| Madrasa | Ulugh Beg Madrasa, Samarkand | Timurid | 1417-1420 | 81 x 56; courtyard 40 x 30 | 4,536 (mine) | Compound (rooms round court) | [S39] (Wikipedia) |
| College quad | Tom Quad, Christ Church, Oxford | English | 1525-1668 | 264 x 261 ft = 80.5 x 79.6 (mine) | 6,408 (mine) quad only | Compound | [S40] (Wikipedia) |
| Playhouse | The Globe, London (original) | Elizabethan | 1599 | about 30 diameter | 707 (mine) | Building | [S41] (Wikipedia) |
| Playhouse | The Rose, London | Elizabethan | 1587 | 22 diameter, 14-sided | 380 (mine) | Building | [S42] (Wikipedia) |
| Roman theatre | Theatre of Marcellus, Rome | Roman | 13 BC | 111 diameter (half-circle) | about 4,838 (mine, half-circle) | Building | [S43] (Wikipedia) |
| Roman theatre | Theatre of Orange | Roman Gaul | 1st c. | stage wall 103 long, stage 61 long | not found | Building | [S44] (Wikipedia) |
| Roman theatre | Theatre of Pompey, Rome | Roman | 55 BC | cavea 150-160 diameter, stage 95 | about 8,800-10,000 (mine, half-circle) | Building (part of larger complex) | [S45] (Wikipedia) |
| Opera house | Palais Garnier, Paris | French | 1861-1875 | 154.9 x 70.2 (101.2 at pavilions) | 12,000 | Building | [S46] (Wikipedia) |
| Arsenal | Venetian Arsenal | Venetian | 12th-16th c. | 3.2 km rampart | 450,000 (45 ha) | Compound (district) | [S47] (Wikipedia) |
| Arsenal building | Corderie (ropewalk), Venetian Arsenal | Venetian | 1303, rebuilt 1579-1585 | 317 x 21 | 6,657 (mine) | Building | [S48] (La Biennale di Venezia) |
| Ship (sets slipway size) | Skuldelev 2 warship | Viking | c.1042 | about 30 x 3.8 | 114 (mine) | Object | [S49] (Wikipedia) |
| Ship (sets slipway size) | Sutton Hoo ship | Anglo-Saxon | c.625 | 27 long | not found | Object | [S50] (Wikipedia) |
| Bell tower | St Mark's Campanile, Venice | Venetian | rebuilt 1912 to old design | 12 x 12 | 144 (mine) | Building | [S51] (Wikipedia) |
| Bell tower | Leaning Tower of Pisa | Pisan Romanesque | 1173-1372 | 15.484 diameter | 188 (mine) | Building | [S52] (Wikipedia) |
| Watchtower | Corsican coastal towers (Martello precursors) | Genoese Corsica | 16th c. | 12-15 diameter | 113-177 (mine) | Building | [S53] (Wikipedia) |
| Watchtower | Martello tower, Saint Helena | British | c.1800s | about 14 diameter | 154 (mine) | Building | [S53] (Wikipedia) |
| Bridgehead gate | Monnow Bridge gate, Monmouth | English/Welsh | c.1272 / gate c.1300 | tower room 10 x 3 internal; bridge 34.8 x 7.3 | about 30 internal (mine) | Building on bridge | [S54] (Wikipedia) |
| Fountain | Trevi Fountain, Rome | Baroque | 1732-1762 | 49.15 wide; depth not found | not found | Structure | [S55] (Wikipedia) |
| Column | Trajan's Column, Rome | Roman | 113 | shaft 3.7 diameter; pedestal size not found | 11 shaft (mine) | Structure | [S56] (Wikipedia) |
| Column | Nelson's Column, London | British | 1840-1843 | pedestal faces carry reliefs 5.5 m square, so pedestal at least 5.5 per side (mine, inference) | at least 30 (mine) | Structure | [S57] (Wikipedia) |
| Market cross | Mercat crosses, Scotland | Scottish | medieval-1800s | 9-13 ft (2.7-4 m) tall; base not found | not found | Structure | [S58] (Wikipedia) |

### Proposed size classes and defaults per kind

**Proposal: six classes, each about four times the area of the one below (side doubles).**
Doubling the side per step matches the spread of the data, which is roughly even on a
logarithmic scale (a scale where each step multiplies instead of adds). Boundaries sit half
way between classes on that scale.

| Class | Nominal size (square equivalent) | Nominal area (m2) | Range (m2) | Measured examples in range |
|---|---|---|---|---|
| tiny | 6 x 6 m | 36 | under 80 | Cristo de la Luz mosque 64; Ise Naikū shrine 60; Nelson's Column pedestal 30+; Trajan's shaft 11; inn chamber 37-47 |
| small | 12 x 12 m | 144 | 80-300 | St Mark's Campanile 144; Pisa tower 188; Kairouan minaret 114; Corsican towers 113-177; Hōryū-ji Kondō 281; infirmary chapel 98 |
| medium | 25 x 25 m | 625 | 300-1,200 | Merchant Adventurers' Hall 390; Rochester keep 441; Lejre hall 528; Kandariya 620; Globe 707; Borg 789; Brihadisvara vimana 911; White Tower 1,152 |
| large | 50 x 50 m | 2,500 | 1,200-5,000 | Westminster Hall 1,547; Hen Domen castle 1,571; St Mary Redcliffe 1,916; Dacheng Hall 1,836; Khan As'ad Pasha 2,500; Daibutsuden 2,850; Beaumaris inner ward 3,000; Ulugh Beg madrasa 4,536; Sultan Han 4,900; Marcellus theatre 4,838 |
| huge | 100 x 100 m | 10,000 | 5,000-20,000 | Hagia Sophia 5,986; Tom Quad 6,408; Corderie 6,657; Cologne 7,914; Kairouan mosque 9,000+; Chartres 10,875; Palais des Papes 11,000; Palais Garnier 12,000 |
| precinct | not a building; drawn as a walled ward | over 20,000 | over 20,000 | Córdoba mosque 23,400; Ibn Tulun 26,318; Brihadisvara compound 29,357; Meenakshi 57,000; Fountains Abbey 280,000; Venetian Arsenal 450,000 |

**Shape matters as much as area.** Proposed default aspect ratio (length to width), from the
examples: halls and longhouses 4:1 (Lejre 4.4, Westminster 3.7, Borg 8.7, Merchant
Adventurers 2.3); churches 2.5:1 (St Mary Redcliffe 2.1, Chartres 130/46 = 2.8 at transept,
Saint Gall 3.8); towers, keeps, shrines 1:1 (Rochester, campanile, Cristo de la Luz);
ropewalks and arsenal sheds 10:1 or more (Corderie 15). All ratios are my arithmetic.
The aspect-ratio defaults are my proposal; which ones to adopt is Jeffrey's decision.

**Default class per kind** (my proposal, each with its evidence):

| Kind | Guessed default | Proposed default | Shape | Evidence |
|---|---|---|---|---|
| temple | large | large | 2.5:1 | Big parish church 1,916 m2; Daibutsuden 2,850; Dacheng Hall 1,836. A cathedral-scale temple is huge (7,900-10,900); a village church is small (Escomb about 68 internal). Spec override needed for both. |
| hall | medium | medium | 4:1 | Guildhall 390; Lejre 528; Borg 789. Westminster Hall 1,547 is a royal outlier (large). |
| tower | small | small | 1:1 | Campanile 144; Pisa 188; minaret 114; Corsican towers 113-177. |
| castle | huge | large for a motte-and-bailey or small castle; huge for a stone castle with wards | 1.3:1 | Hen Domen 1,571; Heywood bailey 2,400 (unverified); Beaumaris inner ward alone 3,000. A stone keep alone is medium (441-1,152). Proposed default: huge, because the spec "castle" implies a stone fortress with wards; a motte is overridden down. |
| inn | medium | medium | 1.2:1 | King's Head yard 280 plus surrounding ranges; New Inn chambers 7.9 m deep, so yard plus two ranges is about 30 m across (mine, estimate). A caravanserai-type inn is large (2,500-4,900). |
| shrine | small | tiny | 2:1 | Ise main building 60; Cristo de la Luz 64. |
| academy | large | large | 1.4:1 | Ulugh Beg madrasa 4,536. Tom Quad alone 6,408 (huge). Oxford-style college defaults to large. |
| theatre | large | medium | round or 1:1 | Globe 707; Rose 380. Roman stone theatres are large to huge (4,838-10,000); opera house huge (12,000). Spec override for those. |
| arsenal | large | huge | 3:1 or more | Corderie alone 6,657. A full Venetian-style arsenal is a precinct (45 ha). |
| bridgehead | small | small | 1:1 | Monnow gate: bridge 7.3 m wide, tower room 10 x 3 internal. Footprint of the gate tower itself not found; small is the closest class to those numbers. |
| well | small | tiny | 1:1 | No well or well-house dimensions found. Trevi Fountain (49 m wide) is a monumental outlier. Tiny is my judgement from the absence of large examples; evidence is weak. |
| monument | small | tiny | 1:1 | Trajan's Column shaft 3.7 m; Nelson's Column pedestal at least 5.5 m. Market-cross bases not found. |

### Comparison with the guessed defaults

- **small 10 x 10 m (100 m2): holds.** Towers measure 113-188 m2. The proposed small is 12 x 12 m.
- **medium 20 x 20 m (400 m2): holds, slightly low.** Halls and keeps measure 390-1,152 m2.
  The proposed medium is 25 x 25 m (625 m2).
- **large 40 x 40 m (1,600 m2): holds at the bottom of the range.** Big parish churches and
  royal halls measure 1,547-1,916 m2, but madrasas, khans and stone theatres reach 4,500-4,900.
  The proposed large is 50 x 50 m (2,500 m2).
- **huge 60 x 60 m (3,600 m2): wrong, too small by a factor of 2-3.** Cathedrals, Friday
  mosques, palaces and opera houses measure 7,900-12,000 m2. The proposed huge is 100 x 100 m.
- **Missing: tiny.** Shrines, monuments and wells measure under 80 m2.
- **Missing: precinct.** Temple compounds, big mosques, abbeys and arsenals measure
  23,000-450,000 m2 and are walled wards, not single footprints.
- **Squares are wrong** for halls (about 4:1), churches (about 2.5:1) and arsenal sheds
  (10:1 or more).
- Per-kind guesses that hold: temple large, hall medium, tower small, inn medium, academy
  large, bridgehead small, castle huge (with the new, larger huge).
- Per-kind guesses that are wrong: shrine (tiny, not small), well (tiny, not small; weak
  evidence), monument (tiny, not small), theatre (medium for a timber playhouse, not large),
  arsenal (huge or precinct, not large).

### Sources

All fetched on 2026-10-20.

- [S1] https://en.wikipedia.org/wiki/Chartres_Cathedral (Wikipedia)
- [S2] https://en.wikipedia.org/wiki/Cologne_Cathedral (Wikipedia)
- [S3] https://en.wikipedia.org/wiki/Salisbury_Cathedral (Wikipedia)
- [S4] https://en.wikipedia.org/wiki/Hagia_Sophia (Wikipedia)
- [S5] https://en.wikipedia.org/wiki/St_Mary_Redcliffe (Wikipedia)
- [S6] https://en.wikipedia.org/wiki/Hull_Minster (Wikipedia)
- [S7] https://en.wikipedia.org/wiki/Escomb_Church (Wikipedia)
- [S8] https://en.wikipedia.org/wiki/Plan_of_Saint_Gall (Wikipedia)
- [S9] https://en.wikipedia.org/wiki/Fountains_Abbey (Wikipedia)
- [S10] https://en.wikipedia.org/wiki/Mosque%E2%80%93Cathedral_of_C%C3%B3rdoba (Wikipedia)
- [S11] https://en.wikipedia.org/wiki/Great_Mosque_of_Kairouan (Wikipedia)
- [S12] https://en.wikipedia.org/wiki/Mosque_of_Ibn_Tulun (Wikipedia)
- [S13] https://en.wikipedia.org/wiki/Mosque_of_Cristo_de_la_Luz (Wikipedia)
- [S14] https://en.wikipedia.org/wiki/Brihadisvara_Temple,_Thanjavur (Wikipedia)
- [S15] https://en.wikipedia.org/wiki/Kandariya_Mahadeva_Temple (Wikipedia)
- [S16] https://en.wikipedia.org/wiki/Meenakshi_Temple (Wikipedia)
- [S17] https://en.wikipedia.org/wiki/T%C5%8Ddai-ji (Wikipedia)
- [S18] https://www.newworldencyclopedia.org/entry/Horyu-ji (New World Encyclopedia, Wikipedia-derived)
- [S19] https://en.wikipedia.org/wiki/Ise_Grand_Shrine (Wikipedia)
- [S20] https://en.wikipedia.org/wiki/Temple_of_Confucius,_Qufu (Wikipedia)
- [S21] https://en.wikipedia.org/wiki/Temple_of_Heaven (Wikipedia)
- [S22] https://en.wikipedia.org/wiki/White_Tower_(Tower_of_London) (Wikipedia)
- [S23] https://en.wikipedia.org/wiki/Rochester_Castle (Wikipedia)
- [S24] https://en.wikipedia.org/wiki/Motte-and-bailey_castle (Wikipedia)
- [S25] https://en.wikipedia.org/wiki/Hen_Domen (Wikipedia)
- [S26] https://www.forestryengland.uk/sites/default/files/documents/Appendix%205%20-%20Eggesford%20SM%20Management%20Plan.pdf (fetched, but the PDF text could not be extracted; figures unverified)
- [S27] https://en.wikipedia.org/wiki/Beaumaris_Castle (Wikipedia)
- [S28] https://en.wikipedia.org/wiki/Westminster_Hall (Wikipedia; area only. The 73 x 20 m figure came from a search summary of a parliament.uk page that returned 403/unreachable when fetched)
- [S29] https://en.wikipedia.org/wiki/Palais_des_Papes (Wikipedia)
- [S30] https://en.wikipedia.org/wiki/Palazzo_Vecchio (Wikipedia)
- [S31] https://strollingguides.co.uk/books/yorkshire/places/merchantadventurers.php (local guide, not scholarly)
- [S32] https://en.wikipedia.org/wiki/Cloth_Hall,_Ypres (Wikipedia)
- [S33] https://en.natmus.dk/historical-knowledge/denmark/prehistoric-period-until-1050-ad/the-viking-age/power-and-aristocracy/magnates-residences-and-royal-seats (National Museum of Denmark)
- [S34] https://en.wikipedia.org/wiki/Lofotr_Viking_Museum (Wikipedia)
- [S35] https://heritageportal.buckinghamshire.gov.uk/Monument/MBC5409 (Buckinghamshire Historic Environment Record)
- [S36] https://britishlistedbuildings.co.uk/101245714-new-inn-gloucester-westgate-ward (mirror of the Historic England listing text)
- [S37] https://en.wikipedia.org/wiki/Sultan_Han (Wikipedia)
- [S38] https://en.wikipedia.org/wiki/Khan_As%27ad_Pasha (Wikipedia)
- [S39] https://en.wikipedia.org/wiki/Ulugh_Beg_Madrasa (Wikipedia)
- [S40] https://en.wikipedia.org/wiki/Tom_Quad (Wikipedia)
- [S41] https://en.wikipedia.org/wiki/Globe_Theatre (Wikipedia)
- [S42] https://en.wikipedia.org/wiki/The_Rose_(theatre) (Wikipedia)
- [S43] https://en.wikipedia.org/wiki/Theatre_of_Marcellus (Wikipedia)
- [S44] https://en.wikipedia.org/wiki/Roman_Theatre_of_Orange (Wikipedia)
- [S45] https://en.wikipedia.org/wiki/Theatre_of_Pompey (Wikipedia)
- [S46] https://en.wikipedia.org/wiki/Palais_Garnier (Wikipedia)
- [S47] https://en.wikipedia.org/wiki/Venetian_Arsenal (Wikipedia)
- [S48] https://www.labiennale.org/en/venues/arsenale (La Biennale di Venezia, current occupant of the building)
- [S49] https://en.wikipedia.org/wiki/Skuldelev_ships (Wikipedia)
- [S50] https://en.wikipedia.org/wiki/Sutton_Hoo (Wikipedia)
- [S51] https://en.wikipedia.org/wiki/St_Mark%27s_Campanile (Wikipedia)
- [S52] https://en.wikipedia.org/wiki/Leaning_Tower_of_Pisa (Wikipedia)
- [S53] https://en.wikipedia.org/wiki/Martello_tower (Wikipedia)
- [S54] https://en.wikipedia.org/wiki/Monnow_Bridge (Wikipedia)
- [S55] https://en.wikipedia.org/wiki/Trevi_Fountain (Wikipedia)
- [S56] https://en.wikipedia.org/wiki/Trajan%27s_Column (Wikipedia)
- [S57] https://en.wikipedia.org/wiki/Nelson%27s_Column (Wikipedia)
- [S58] https://en.wikipedia.org/wiki/Mercat_cross (Wikipedia)

Source quality note: most numbers come from Wikipedia. Historic England list entries
(historicengland.org.uk) returned HTTP 403 to every fetch, including with curl, so no
scheduled-monument text could be read directly. UNESCO pages also returned 403.

### Not found

- Salisbury Cathedral width and area; size of Salisbury Cathedral Close.
- Hull Minster width and area.
- Viking Dublin house dimensions from an excavation report (only an unverified 36 m2 figure from a low-quality site).
- Viking shipyard or slipway dimensions (only ship lengths, 27-30 m, which bound the slipway length).
- Edwardian castle overall outlines (Beaumaris, Harlech, Conwy, Caernarfon pages give no overall dimensions).
- Torre del Mangia base; Tower of Hercules base; Holstentor footprint; Old Town Bridge Tower Prague footprint.
- Ypres Cloth Hall depth; any small market hall (Ledbury, Chipping Campden pages not found).
- Any town hall overall footprint (Leuven and Siena pages not found or without numbers).
- George Inn Southwark dimensions; any complete medieval coaching-inn outline.
- Any well or well-house dimensions; Fontana Maggiore basin diameter.
- Market-cross base dimensions; Trajan's Column pedestal size.
- Merton College Mob Quad dimensions.
- Buddhist pagoda base dimensions (Hōryū-ji pagoda, Shwedagon).
- Small Hindu shrine dimensions.
