## Concentric town walls: how common, how far apart (research for #207)

Research date: 2026-11-09. Issue #207 asks four questions for the `separate_ring` and `gap_metres`
fields of ADR-063, whose default `SEPARATE_RING_GAP_METRES` is 20 (Claude's estimate). Every
number below comes from a page fetched in this session unless it is marked "(search snippet,
unread)". "The source says" keeps each source's own certainty (ADR-023). Arithmetic I did myself
is marked "(my calculation)". This report does not decide; the question for Jeffrey is at the end.

### Terms

- **Double wall, concentric wall:** a second, usually lower wall in front of the main wall, with an
  open strip between them that is part of the defence.
- **Lists** (French *lices*), **Zwinger** (German), **braies** (French *fausse-braie*, an older
  term), **peribolos** (Byzantine), **barbacana** (Spanish, in al-Andalus an outer wall in front of
  the main wall): names for that strip or for the outer wall that makes it.
- **Successive rings:** a newer, wider wall built around suburbs that grew outside an older wall,
  so houses, not an empty strip, lie between the two.

### What the project already knows (not repeated below)

- Two rings at Carcassonne with open lists between them: walls.carcassonne.ring-count,
  walls.carcassonne.inner-circuit (about 1.1 km, 26 towers), walls.carcassonne.outer-circuit
  (about 1.5 km, outer wall 1230s to 1280s, 19 towers).
- Constantinople's land wall dimensions: walls.constantinople.inner-land-wall-thickness and
  -height, walls.constantinople.outer-land-wall-thickness (2 m at the base) and -height (8.5 to
  9 m), walls.constantinople.moat-width ("over 20 m", about 20 m in front of the outer wall).
- Two rings at Nuremberg: walls.nuremberg.ring-count, walls.nuremberg.outer-circuit (14th to
  15th century, about 5 km, wide dry moat).
- Successive rings with growth between: Cologne (walls.cologne.ring-growth, 97 to 401 ha),
  Florence (walls.florence.ring-count, 4 to 6 rings), Nördlingen (walls.nordlingen.ring-count,
  walls.nordlingen.area-growth-1327), Paris, Moscow, Bern; settlement/walls-growth.md "Growth
  patterns" item 1 and cross-analysis/walls.md §5 item 1 (nested rings, radial roads through all
  of them, old gate sites become squares).
- No fact in data/research/facts/walls.yaml gives the width of a strip between two walls. The
  words zwinger, braies, peribolos and Beaumaris do not occur there. This report fills that gap.

### Question 1: how common, which towns, which periods

**Towns with a double wall and an empty strip between (the Carcassonne case):**

| Town | Period | What the source says | Source |
|---|---|---|---|
| Constantinople | 5th century | Inner wall, then a terrace (peribolos), then the lower outer wall, then a second terrace (parateichion) and the moat | [W-Constantinople], [TAN] |
| Nicaea (İznik) | 13th century | John III Vatatzes (1222 to 1254) added "a second circle, concentric to the former", modelled on Constantinople (search snippet, unread: Bevilacqua 2020); the outer wall is 1.6 m thick, 3 to 4 m high | [Schneider-1938] |
| Seville | 12th to 13th century | A barbacana 1.45 m thick in front of the Almohad wall | [Seville] |
| Jerusalem | 1099 | A Seljuk outer wall just inside the ditch, "not an independent structure; it was linked to the city at multiple points" | [W-Siege-1099] |
| Carcassonne | 1240s onward | "Deux enceintes ... sont séparées par un espace plat : les lices"; the lists were levelled and protected by a rampart from 1240 | [W-fr-Carcassonne], [Carcassonne-dossier] |
| Wels (Austria) | 13th to 14th century | "eine zweite, niedrigere Mauer vorgelagert", on the south side only | [Wels] |
| Ávila | 14th century | "una segunda barrera de menor tamaño que hacía las veces de barbacana" on the calle de San Segundo side only | [W-es-Avila] |
| Kraków | from about 1404 | "construction of a second, outer line of walls began"; it surrounded the whole city except the marshy southwest | [W-Krakow-walls] |
| Nuremberg | 15th century | High wall, a 15 m wide "kennel" (Zwinger), the kennel wall rising from the moat, the dry moat (moat finished 1452) | [W-Nuremberg] |

German Wikipedia and English Wikipedia also list Zwinger at Amberg, Aschersleben, Delitzsch,
Dinkelsbühl, Görlitz, Ingelheim, Jihlava, Jüterbog, Neubrandenburg, Nördlingen, Templin, Warsaw and
Wolframs-Eschenbach, without widths or dates [W-de-Zwinger], [W-Zwinger]. The German article on the
Nördlingen wall itself mentions no Zwinger and no second wall, only the moat [W-de-Nordlingen]; the
two Wikipedia pages disagree on Nördlingen and I did not resolve it. For Dinkelsbühl I found no page
on its Zwinger that I could read.

**Towns with successive rings and town growth between (the Milan case):**

| Town | Rings | Source |
|---|---|---|
| Milan | Roman square of about 700 m sides; Maximian's walls over 100 ha; the medieval ring from 1156 with its moat (today's Cerchia dei Navigli); the Spanish walls of 1546 to 1560 (Italian Wikipedia: 1548 to 1562), about 11 km (today's Cerchia dei Bastioni). Neither page says in words that suburbs filled the space between; the Italian page says the Spanish walls were built "per sostituire le mura medievali" | [W-Milan], [W-it-Milan-Spanish] |
| Rothenburg ob der Tauber | First wall after 1172 (Markusturm, Weißer Turm survive); an extension "Bereits im Jahr 1204", wall 1,400 to 2,400 m; the hospital quarter added early 14th century, 2,400 to 3,360 m | [W-de-Rothenburg] |
| Nördlingen | The 1327 wall "ersetzte den alten Verteidigungswall", whose line survives in the old-town streets | [W-de-Nordlingen] |
| Nuremberg | Before 1320 to 1325 the districts on both banks of the Pegnitz were fortified separately, then joined; the Zwinger came later, so Nuremberg is both kinds | [W-Nuremberg] |
| Cologne, Florence, Paris, Moscow | Already in the facts (see above) | — |

**Towns with one wall only (no strip, no ring):**

| Town | What the source says | Source |
|---|---|---|
| Visby | One circuit; outside it trenches; 17th-century "two parallel walls running circa 25 metres" out from the wall at one gate (Österport), not a ring | [W-Visby] |
| Aigues-Mortes | "L'enceinte quadrangulaire est flanquée de quinze tours et percée de dix portes"; the page mentions no outer wall and no ditch, so single is my reading, not the source's words | [W-fr-Aigues-Mortes] |
| Rhodes | A crescent of earth-filled wall with scarp, bastions, moat, counterscarp and glacis; terrepleins built "in the middle of the moat" after 1480; no second wall is described | [W-Rhodes] |
| Famagusta, Ghent | Not found: no page I could read says whether the wall was single or double | — |

**Prevalence in words.** The Münster institute for town history says the circuit Zwinger was "die
bedeutendste Innovation des 14. und 15. Jhs." and was built mainly in large, rich towns; small towns
generally stayed without one [Muenster]. English Wikipedia dates the spread: Zwinger reappeared in
central Europe in the first half of the 13th century, became far more common in the late 13th
century, and many were built in the 15th and 16th centuries [W-Zwinger].

**Counts (my calculation, from the tables above):** 9 towns with a sourced empty strip, of which 3
have it only on part of the circuit (Wels, Ávila, Kraków less its southwest); 4 towns read in this
session with successive rings and growth between, plus 4 already in the facts; 3 towns read as single
walls; 2 not found. Nuremberg counts in both of the first two groups. These counts come from the
towns I chose to look up, not from a survey, so they say nothing about the share of all European
towns.

### Question 2: the width of the strip

| Place | Strip width | What is measured | Certainty | Source |
|---|---|---|---|---|
| Seville | about 3 m | wall to barbacana | wiki mirror; excavation reports seen only as search snippets (3 m, and a hypothetical 4 m near Puerta de Santa María) | [Seville] |
| Wels | 4.5 m | "ein 4,5 Meter breiter Zwinger vor den Graben" | city archive page, no author named | [Wels] |
| Nicaea | 6 to 9 m (my conversion of 20 to 30 feet) | "an external wall, at a distance of 20-30 feet" | weak source: an unattributed hobby page | [Piperno] |
| Kraków | 10 m | "Ten meters outside the inner wall was an outer, lower one" | single source (Wikipedia) | [W-Florian] |
| Nicaea | 14 m | "The outer wall is about 14 m. distant from the main wall" | Schneider, Antiquity 1938 (footnote text garbled on the page) | [Schneider-1938] |
| Nuremberg | 15 m | "15-meter-wide kennel" | Wikipedia; a German paper by Clauß says "bis 15 m" (search snippet, unread: the server refused the fetch) | [W-Nuremberg] |
| Constantinople | 15 to 20 m | peribolos, between inner and outer wall | popular archaeology site | [TAN] |
| Constantinople | 50 to 64 ft, 15.2 to 19.5 m (my conversion) | peribolos | blog citing Turnbull, Osprey Fortress 25 (2004) | [Blog-Turnbull] |
| Carcassonne | 25 m | "large de 25 m dans l'angle", southwest corner only | archaeologist (Fourdrin 2014), one point | [Fourdrin] |
| Constantinople, Blachernae | 26 m | the two walls of the Blachernae enclosure "stand some 26 m apart" | Wikipedia | [W-Constantinople] |
| Beaumaris (castle, for comparison) | about 18 m | the outer ward "approximately 60 feet (18 m) across"; the page does not say this is the gap | Wikipedia, ambiguous | [W-Beaumaris] |

Not a strip between two walls, so left out of the summary: Constantinople's parateichion (outer
wall to moat) 18 m [TAN] or 61 ft, 18.6 m [Blog-Turnbull]; Visby's 25 m gate walls.

Carcassonne has no general width in any source I read: English Wikipedia and the UNESCO search
results give none; a page found by search gives 7 to 8 m (search snippet, unread: the site's
certificate failed). The ramparts' own teaching dossier gives no number but says the Tour de
l'Évêque "enjambe les lices au point le plus étroit du plateau" [Carcassonne-dossier].

**Summary (my calculation):** 10 sourced town values from 3 m to 26 m, taking the midpoint of each
range. They fall in two groups. Late-medieval Zwinger and barbacana strips in western and central
Europe: 3 m (Seville), 4.5 m (Wels), 10 m (Kraków), 15 m (Nuremberg). Byzantine and Byzantine-model
double walls and Carcassonne: 14 m (Nicaea; the weak source says 6 to 9 m), 15 to 20 m
(Constantinople), 25 m (one corner at Carcassonne), 26 m (Blachernae). The median of the 8 values
3, 4.5, 10, 14, 15, 17.5, 25, 26 is 14.5 m. German Wikipedia adds that early Zwinger walls stood "in
sehr geringem Abstand vor der Hauptmauer" [W-de-Zwinger], with no number.

### Question 3: near-constant gap, or bending away

- **Near-constant along a run.** At Constantinople the peribolos is given as one range, 15 to 20 m,
  over the whole double wall [TAN]; Wikipedia places the towers "at irregular distances, according
  to the rise of the terrain" but says nothing about the gap changing [W-Constantinople]. At Seville
  the excavation reports describe the barbacana "a 3 m en paralelo a la muralla" (search snippet,
  unread). The early Zwinger stood close to the main wall [W-de-Zwinger].
- **The gap changes with the ground.** At Carcassonne the lists are 25 m at the southwest corner
  [Fourdrin] and narrowest where the plateau is narrowest, at the Tour de l'Évêque
  [Carcassonne-dossier]; Fourdrin says the antique line was kept except in two sectors, where the
  13th-century works were built "en avant du périmètre d'origine" [Fourdrin].
- **The outer line ends instead of bending away.** No source I read describes an outer wall that
  swings far out around a hill or a gate and comes back. What they describe is an outer wall that
  stops where the ground already protects the town: Kraków's outer wall left out the marshy
  southwest [W-Krakow-walls]; at Constantinople the moat probably stopped in the steep Lycus valley
  and the double wall became single walls at Blachernae, "where the terrain made attacks difficult"
  [W-Constantinople], [TAN]; Wels and Ávila had their second wall on one side only [Wels],
  [W-es-Avila]. Gates break the strip with forward works: barbacanes "devant chaque porte" at
  Carcassonne [Carcassonne-dossier].

### Question 4: where inner and outer wall share a stretch

- **Constantinople, Blachernae.** The double Theodosian wall ends and a series of "single walls
  built in different periods" takes over. The Wall of Manuel I Komnenos (1143 to 1180), about 220 m
  long, is "some 5 m at the top", thicker than the 2 m outer wall and as thick as the 4.5 to 6 m
  inner wall (walls.constantinople.inner-land-wall-thickness). The source says Blachernae was a
  single "strong barrier" [TAN]; a search result calls the later Blachernae walls "a vulnerable
  section" (search snippet, unread). So the stretch is one wall in the inner wall's strength.
- **Carcassonne.** The château comtal is "adossé à la muraille ouest" and "construit le long de
  l'enceinte intérieure", with its own ditch [W-fr-Carcassonne]; the Tour de l'Évêque, a square
  tower of the late 13th century, spans the lists at their narrowest point, closing the strip like
  a cross wall [Carcassonne-dossier].
- **Jerusalem 1099.** The outer wall "was linked to the city at multiple points", which cut the strip
  into closed no-man's-land compartments [W-Siege-1099].
- **Castles joined to a town wall.** Kraków's walls were joined to those of Wawel Castle under
  Casimir III [W-Krakow-walls]; at Nuremberg "the extensive castle complex is fully integrated into
  the fortification" [W-Nuremberg]; at Ávila the cathedral apse "quedó entestado en la propia
  muralla" [W-es-Avila]. None of these pages says whether the Zwinger or outer wall ran past these
  buildings.

Pattern (my reading of the above, not a source's words): where the two lines meet, one wall stands,
built to the inner wall's strength or stronger, or a tower or building closes the strip across.
This matches ADR-063's "drawn once in the stronger style".

### Not found

- Famagusta and Ghent: whether single or double.
- Dinkelsbühl and Rothenburg Zwinger widths; the Nuremberg paper by Clauß (access refused).
- Any survey that counts what share of medieval towns had a double wall.
- Creighton and Higham, Tracy, and the Viollet-le-Duc monograph on Carcassonne were not fetched.

### Sources

- [W-Constantinople] Wikipedia, "Walls of Constantinople": https://en.wikipedia.org/wiki/Walls_of_Constantinople
- [TAN] Turkish Archaeological News, "Theodosian Land Walls of Constantinople": https://www.turkisharchaeonews.net/object/theodosian-land-walls-constantinople
- [Blog-Turnbull] Byzantium Novum Militarium blog, 2011, citing Turnbull 2004: https://byzantiumnovummilitarium.blogspot.com/2011/02/cross-section-and-plan-of-theodosian.html
- [Schneider-1938] A. M. Schneider, "The City Walls of Nicaea", Antiquity 12 (48), 1938: https://www.cambridge.org/core/journals/antiquity/article/city-walls-of-nicaea/D4DDD4209A71A15C5D9B2401FE441CF0
- [Piperno] R. Piperno, "Nicaea" (weak source): https://romeartlover.it/Nicea.html
- [Seville] Hispanopedia mirror, "Murallas de Sevilla": https://es.hispanopedia.com/wiki/Murallas_de_Sevilla
- [W-Siege-1099] Wikipedia, "Siege of Jerusalem (1099)": https://en.wikipedia.org/wiki/Siege_of_Jerusalem_(1099)
- [W-fr-Carcassonne] Wikipedia (French), "Cité de Carcassonne": https://fr.wikipedia.org/wiki/Cité_de_Carcassonne
- [Carcassonne-dossier] Château et remparts de la Cité de Carcassonne, "Dossier thématique: l'architecture militaire": https://www.remparts-carcassonne.fr/content/download/9873785/file/L'architecture%20militaire.pdf?version=1
- [Fourdrin] J.-P. Fourdrin, "Le tracé de l'enceinte de Carcassonne au Bas Empire", Presses universitaires de Provence, 2014: https://books.openedition.org/pup/26748?lang=en
- [Wels] Stadt Wels, "Die Stadtbefestigungen": https://wels.gv.at/lebensbereiche/bildung-und-kultur/stadtarchiv-und-geschichte/die-stadtbefestigungen
- [W-es-Avila] Wikipedia (Spanish), "Muralla de Ávila": https://es.wikipedia.org/wiki/Muralla_de_Ávila
- [W-Krakow-walls] Wikipedia, "City walls of Kraków": https://en.wikipedia.org/wiki/City_walls_of_Krak%C3%B3w
- [W-Florian] Wikipedia, "St. Florian's Gate": https://en.wikipedia.org/wiki/St._Florian%27s_Gate
- [W-Nuremberg] Wikipedia, "City walls of Nuremberg": https://en.wikipedia.org/wiki/City_walls_of_Nuremberg
- [W-de-Zwinger] Wikipedia (German), "Zwinger": https://de.wikipedia.org/wiki/Zwinger
- [W-Zwinger] Wikipedia, "Zwinger": https://en.wikipedia.org/wiki/Zwinger
- [Muenster] Institut für vergleichende Städtegeschichte, Münster, "Stadtmauer und Stadtbefestigung": https://www.uni-muenster.de/Staedtegeschichte/istg_digital/EinfuehrungStaedtegeschichte/Stadtmauer.html
- [W-de-Nordlingen] Wikipedia (German), "Stadtmauer Nördlingen": https://de.wikipedia.org/wiki/Stadtmauer_Nördlingen
- [W-de-Rothenburg] Wikipedia (German), "Rothenburg ob der Tauber": https://de.wikipedia.org/wiki/Rothenburg_ob_der_Tauber
- [W-Milan] Wikipedia, "Walls of Milan": https://en.wikipedia.org/wiki/Walls_of_Milan
- [W-it-Milan-Spanish] Wikipedia (Italian), "Mura spagnole di Milano": https://it.wikipedia.org/wiki/Mura_spagnole_di_Milano
- [W-Visby] Wikipedia, "Visby City Wall": https://en.wikipedia.org/wiki/Visby_City_Wall
- [W-fr-Aigues-Mortes] Wikipedia (French), "Remparts d'Aigues-Mortes": https://fr.wikipedia.org/wiki/Remparts_d'Aigues-Mortes
- [W-Rhodes] Wikipedia, "Fortifications of Rhodes": https://en.wikipedia.org/wiki/Fortifications_of_Rhodes
- [W-Beaumaris] Wikipedia, "Beaumaris Castle": https://en.wikipedia.org/wiki/Beaumaris_Castle

### Gap widths with sources

| Value | Place, period | Source | Strength |
|---|---|---|---|
| 3 m | Seville, Almohad | [Seville] | wiki mirror |
| 4.5 m | Wels, 13th to 14th century | [Wels] | city archive |
| 6 to 9 m | Nicaea, 13th century | [Piperno] | weak |
| 10 m | Kraków, from 1404 | [W-Florian] | Wikipedia, one source |
| 14 m | Nicaea, 13th century | [Schneider-1938] | journal, 1938 |
| 15 m | Nuremberg, 15th century | [W-Nuremberg] | Wikipedia |
| 15 to 20 m | Constantinople, 5th century | [TAN], [Blog-Turnbull] | two popular sources agree |
| 25 m | Carcassonne, southwest corner | [Fourdrin] | archaeologist, one point |
| 26 m | Constantinople, Blachernae enclosure | [W-Constantinople] | Wikipedia |

### Pattern with counts

- Of 21 towns (my count; Nuremberg once): 9 had a double wall with an empty strip (3 of them on part of the circuit
  only), 8 had successive rings with growth between (4 read here, 4 from the facts), 3 had one wall,
  2 not found; Nuremberg is in two groups.
- The strip is 3 to 26 m over 8 sourced values (median 14.5 m, my calculation): narrow (3 to 15 m)
  for late-medieval Zwinger and barbacana, wider (14 to 26 m) for Byzantine double walls and
  Carcassonne.
- The gap is near-constant along a run; it changes with the ground (Carcassonne). In 4 towns the
  outer line covers only part of the circuit instead of bending away: Kraków and Constantinople
  where the ground protects (the sources say so), Wels and Ávila on one side (the sources give no
  reason).
- Where the lines meet, one wall or a building stands, as strong as the inner wall or stronger
  (Blachernae, Carcassonne château and Tour de l'Évêque, Jerusalem's links).

### Options for the documented default gap

1. **15 m.** Nuremberg's Zwinger and the low end of Constantinople's peribolos; close to the median
   of 14.5 m.
2. **20 m.** Keep the current `SEPARATE_RING_GAP_METRES`; the top of Constantinople's 15 to 20 m and
   between Carcassonne's corner (25 m) and Nuremberg (15 m); now with sources instead of an estimate.
3. **10 m.** Kraków; a late-medieval Zwinger default, narrower than every Byzantine value.
4. **Two defaults by wall `kind` or era,** for example 10 m for a Zwinger and 18 m for a Byzantine-
   style double wall. This needs a new field or rule, so it is a design change, not only a number.

Question for Jeffrey: which default gap should `SEPARATE_RING_GAP_METRES` document: 1 (15 m),
2 (20 m), 3 (10 m) or 4 (two defaults, which needs a design change)?
