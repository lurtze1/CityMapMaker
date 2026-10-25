# Why cities are founded where they are, and how they grow

> Written 2026-10-20 by four research agents, one per slice, and assembled by Claude (issue
> #32). A claim here is binding only when a DECISIONS.md entry cites it. Most case-study facts
> come from encyclopaedia and UNESCO pages; the quantitative studies were read as working
> papers. Claims seen only in search summaries are marked "(unverified)".

Purpose: during the spec v1 review (PR #20) Jeffrey asked whether the pipeline order, which
builds the town (S1) before its roads (S3), matches how real towns came to be. He decided that
version 1 treats important entry roads as site features, like rivers, and asked for research
into foundation and growth across cultures: roads versus towns, forts versus sites, planned
foundations, organic and event-driven growth, and the balance of causes.

Terms used throughout: *first-nature geography* is what the land gives (river, coast, hill,
soil); *second-nature geography* is what people already built (other towns, roads); a *seed* is
the first thing a town grows around (a ford, a shrine, a fort, a market); a *plan unit* is a
part of a town laid out in one go, with its own street and plot pattern; a *burgage plot* is a
long narrow town lot with its short side on the street.

## 1. Summary

1. **Water is the strongest measured cause.** Two independent studies agree. In Europe
   800-1800, Bosker and Buringh find a navigable river the strongest physical advantage, in
   both versions of their study (see §2 for why the two versions give different numbers). Worldwide over 2,000
   years, one step up in river size meant urbanising about 8 years earlier, and every 100 km
   from the coast 1.3 years later (Motamed, Florax and Masters 2014). After Rome fell, Britain's
   re-founded towns chose water: 77% had coastal or navigable-river access, against 52% of Roman
   towns (Michaels and Rauch).
2. **Roads matter for location less than rivers, but they shape the plan.** In the 2012
   working paper of Bosker and Buringh a site on an old Roman road had no advantage and a road
   junction helped only before about 1600; in the published 2017 version a Roman road gives a
   small but significant advantage (+0.4 percentage points per century, against +1.2 for a
   river). Yet Roman road density does predict later market towns (more than double
   the chance in one German frontier strip) and today's wealth, except where camels replaced
   carts (Dalgaard et al.). Inside the town, the old road almost always becomes the main street
   and the market sits on it.
3. **Both orders happen, and a town can say which.** Road first: Maastricht, Watling Street
   towns, Tokaido post towns, Innsbruck, probably Alnwick. Town first, roads pulled in:
   Lugdunum (Lyon), Cusco and the Inca centres. Site first, road bent to it: Ludlow (the ford
   moved to a new bridge for the town), Japanese castle towns (the main road was "reworked to
   pass through the city" and in front of the castle). No study gives a single share for each
   order.
4. **The fort usually came after the route.** The most common order in the fortification slice
   is route or crossing, then fort, then town: Wallingford, Newcastle, Conwy, the Rhine toll
   castles, Crusader route castles, Siberian ostrogs. The second pattern is settlement, then
   fort (Norman castles inserted into town corners by the river; Spanish presidios protecting a
   mission). The third is defensible site first (Ludlow, oppida, Indian hill forts). Hill sites
   drift down to the plain over time (Bibracte to Autun, Chittorgarh's lower town, Golconda to
   Hyderabad).
5. **Almost no "new" city was founded on empty ground.** Constantinople, St Petersburg,
   Batavia, Cologne, Dur-Sharrukin, Fatehpur Sikri, Baghdad, Washington and Syracuse all sat on
   or beside something older. Only Amarna was claimed as empty.
6. **Planned foundations state their criteria, and the criteria repeat across cultures.**
   Vitruvius, Aristotle, the Arthashastra and the Spanish Ordinances of 1573 name the same list:
   healthy air, fresh water, ground above marsh, defensible, reachable by land and water,
   farmland and timber nearby. Each culture adds wind and sun rules. The Ordinances also state a
   build order: site, main plaza, streets to the gates and roads, church and royal lots, house
   lots, palisade, then commons and fields.
7. **The centre is placed by rule.** On a coast the plaza or forum goes at the landing place;
   inland at the middle (Vitruvius, Ordinance 112). Indian and East Asian models put the palace
   north of centre; Baghdad put palace and mosque at the exact centre.
8. **Religion, capitals, mines and fairs are strong but rare causes.** Only 1% of candidate
   sites in Bosker and Buringh were archbishoprics and 0.2% capitals. A market grant alone did
   not make a town: only 39% of English places with a market by 1300 were still market towns
   around 1600.
9. **Plans outlast buildings, and disaster rarely erases them.** London after 1666 kept its
   medieval streets; Lisbon after 1755 put a grid patch inside the old core; Noto after 1693
   moved 8 km. Planned grids also decay: Timgad spilled beyond its grid, Freiburg dropped its
   plot norm, Bern grew in three walled stages.
10. **Growth outside the walls follows the approach roads.** Ribbon suburbs of long narrow plots
    line the road beyond a gate (Dublin, York), often with a market widening just outside it.

## 2. Where the slices disagree

- **"Roads first" versus "water first".** The roads and fortification slices find many towns
  where a road or crossing came first; the quantitative studies find roads weak and water
  strong. Both are true: most crossings that seeded towns are *river* crossings (a ford or
  bridge), so the road and the river act together. A road alone, away from water, is a weak
  seed. For the generator this means the crossing of a road and a river is the strongest
  candidate for a town core.
- **Two versions of Bosker and Buringh.** Verified afterwards (issue #36,
  `documentation/research/settlement/verification-population.md` on PR #35). The slices quote the 2012
  working paper: 1,784 candidate sites (all cities plus the 456 bishoprics of the year 600), a
  base chance of about 12%, a river worth about +6 percentage points, no Roman road effect, a
  road hub positive before 1600 only, and an existing city 20-100 km away worth about +1
  point. The published 2017 paper is a different study: 259,776 random locations, a base
  chance of 0.05% per century, a river +1.2 points, the sea +0.7, a Roman road +0.4
  (significant), a hub about +7 (which the authors call unreliable), a city 20-100 km away
  +0.02 and one within 20 km -0.05. The direction agrees (water first, roads second); the
  numbers must never be mixed. The slice texts below are unchanged and quote the working
  paper.
- **Other figures checked.** Motamed, Florax and Masters (8 years per river order, 1.3 years
  per 100 km from the coast, 18 years for frost, at the 10% urbanisation threshold) and
  Michaels and Rauch (3 of 20 British versus 16 of 20 French large cities near a Roman town
  within 5 km; 77% versus 52% water access) are verified against their working papers.

## 3. What this means for CityMapMaker

Proposals for Jeffrey. None is decided until a DECISIONS.md entry says so.

1. **Roads as site features (already decided for version 1).** The evidence supports it: in
   most founding stories a road or crossing exists before the town. There is a generator
   precedent too: Beneš, Wilkie and Křivánek (2014) first build roads between off-map cities
   and grow the town from road junctions, bridge ends and harbours (issue #33 report on growth
   generators). A road needs a course
   (where it enters and where it leaves or ends), laid down in S0 next to rivers, so that
   landmarks can be `near` it and the market can sit where it crosses a river or another road.
2. **A road may predate the town or be built to it.** The roads slice suggests a per-road flag.
   For version 1, every spec road can be treated as predating the town; roads built to the
   town already exist as the generated main streets of S3. A flag is a candidate for later.
3. **The town core seeks a crossing.** The strongest seed is where a road crosses a river or
   another road. Without a pin, S1 can score candidate core positions by distance to such
   crossings, weighted by a rule pack (river attraction versus road attraction differs by
   culture and era).
4. **Gates follow roads.** Gates stand where roads meet the wall in every slice. A spec road
   that crosses a wall implies a gate; a gate without a road (a postern, a water gate) stays
   possible as an explicit spec entry.
5. **Founding cause as data, later.** Every slice proposes a closed list of founding causes
   (ford, bridge, road junction, harbour, pass foot, portage, caravan halt, fort or castle,
   monastery or shrine, mine, fair, administrative seat, colony, capital, rebuild). Physical
   causes become placement rules; non-physical ones (an oracle, a dream, a ruler's prestige) are
   lore only. Candidate for F2b, the version-2 reserved list.
6. **Older cores and growth phases, later.** Most towns fit around something older, and grow in
   plan units that never rewrite earlier ones unless a rebuild event says so. This fits the F2b
   history layers already reserved (old wall rings, razed walls as ring roads).
7. **Fort-to-town relation is culture data.** At the gate with the market below (Norman
   borough), at a corner of the wall (Edwardian Wales), at the centre with rings of rank
   (Japan), a plaza in front (Spanish colonies). That is a rule-pack choice, not core code.

## 4. The four slices

The full slice reports follow unchanged except for heading levels. Each has its own sources
and "Not found" list.


### 4A. Roads and cities

Question: which came first, the long-distance road or the town? How often? What does each
leave in the street plan? Written for a generator that may lay down roads before the town,
the same way it lays down rivers.

Jargon used below:
- **First-nature geography**: what the land gives you (river, coast, hill, soil).
- **Second-nature geography**: what people already built (other towns, roads).
- **Persistence**: a place that mattered long ago still matters today.
- **Path dependence**: a place stays important after the reason it was founded has gone.
- **Portage**: carrying boats and cargo overland around rapids or between rivers.
- **Fall line**: in the eastern USA, the line where rivers drop off the hard upland onto the
  coastal plain; the last rapids before the sea.
- **Burh**: a fortified Anglo-Saxon town (c. 880-950).
- **Burgage plot**: a long narrow medieval town lot with its short side on the street.
- **Mansio**: a Roman state inn and stable on a main road.
- **Caravanserai / sarai**: a walled roadside inn for caravans.
- **Odds ratio**: how many times more likely X is in one group than in another.

#### Findings

- **Water beat roads in medieval Europe.** Bosker & Buringh studied over 250,000 possible
  city sites and every real city in Europe from 800 to 1800. Physical geography decided
  where cities appeared, especially in the early centuries. A navigable river raised a
  site's chance of becoming a city by about 6 percentage points, against a baseline of
  about 12%. A site on a former Roman road gave no advantage. Sites on a Roman road that
  were *not* a junction did slightly worse.
  Source: https://repub.eur.nl/pub/99124 (JUE 2017, vol. 98, pp. 139-157) and the working
  paper http://www.aeaweb.org/conference/2012/retrieve.php?pdfid=203 (pp. 17-19). The
  numbers here come from the working-paper version; the final journal tables may differ.
- **Road junctions mattered, but only before about 1600.** A site where two or more Roman
  roads crossed (a "hub") had a better chance of becoming a city before 1600. After 1600
  the effect was negative or zero. River access became *more* important over time.
  Source: same working paper, p. 27 and Table 3.
- **Old roads were often the wrong roads later.** Bosker & Buringh quote Lopez (1956): in
  the later Middle Ages "a new network of roads was put into effective operation, different
  totally in structure ... from the ancient one", routed for commerce rather than soldiers.
  Source: same working paper, footnote 32.
- **Transport usually decides where cities go.** Bairoch (1988, p. 143), as quoted by Bosker
  & Buringh: "The critical role played by transport in the location of cities does not rule
  out exceptions, but statistically speaking these are in the minority." The same passage
  lists the classic strategic sites: a river crossing, the foot of a mountain pass, a hill
  over the countryside. Source: same working paper, pp. 4-5. (I did not read Bairoch himself.)
- **Roman roads do predict later towns and wealth, across the whole empire.** Dalgaard,
  Kaarsen, Olsson & Selaya: more Roman road density goes with more settlements in 500 CE,
  more modern roads, and more economic activity today. Roman road density explains about
  12% of differences in modern road density; a 1% denser Roman network goes with about
  0.24% denser modern roads (0.15% after geographic controls). Economic activity in late
  antiquity rises about 0.6% per percentage point of road density; today the elasticity is
  0.5 to 1. Source: https://web.econ.ku.dk/dalgaard/Work/WPs/Romanroads.pdf (2018, pp. 17-19).
- **The road effect vanishes where the wheel was abandoned.** In North Africa and the Middle
  East camels replaced carts from the first millennium CE. There the link between Roman
  roads and later roads or wealth weakens to insignificance. Caravan routes replaced roads.
  Source: same paper, abstract and section 4.3; 2021 version
  https://web.econ.ku.dk/pabloselaya/papers/RomanRoads.pdf (abstract).
- **The mechanism is market towns.** In a 100 km strip along the Roman frontier in Germany
  (398 market towns), a 0.86 km² cell containing a Roman road was 0.3 percentage points more
  likely to get a medieval market town. The base rate was 0.22%, so a Roman road more than
  doubles the chance. Source: 2021 version, section 4.4, pp. 32-33.
- **Roman roads mostly ignored what was there before.** Roman engineers drove straight lines
  from a held point A to a target point B for military use. They "consciously avoided linking
  Via Appia to a number of existing Latin settlements". Pre-Roman oppida (Celtic hill-towns)
  hardly predict Roman road density. Geography explains about 40% of road density.
  Source: https://web.econ.ku.dk/dalgaard/Work/WPs/Romanroads.pdf, pp. 12-15.
- **But Roman roads linked towns, and new towns grew along them.** Michaels & Rauch: "During
  the Roman era roads connected major towns, and other towns emerged alongside these roads."
  Some Roman towns were placed on or near Iron Age sites; others were new.
  Source: https://cep.lse.ac.uk/pubs/download/dp1248.pdf, pp. 3 and 9.
- **A network can be wiped and re-seeded.** After Rome fell, Britain's towns largely emptied;
  France's did not. Persistence of Roman town sites (as an odds ratio) was about 7-13 in
  Britain and 20-40 in France. Only 3 of Britain's 20 largest cities sit near a Roman town,
  against 16 of 20 in France. Re-founded British towns chose water: 77% had coastal or
  navigable-river access in the Middle Ages, against 52% in the Roman era. Source:
  https://cep.lse.ac.uk/pubs/download/dp1248.pdf, pp. 2-4, 18, footnote 42.
- **Break-of-transport points create towns that outlive the reason.** In the south-eastern
  USA "an urban area of some size is found nearly every place a river crosses the fall
  line". Being within 15 miles of a portage site predicts 50% to 110% higher population
  density today, although portage became obsolete in the 1800s.
  Source: https://personal.lse.ac.uk/sturmd/brixen/pdf/Bleakley-Lin-QJE-2012.pdf, pp. 588, 607.
- **Road-first settlements take a linear shape.** Roman "small towns" on Watling Street show
  "ribbon settlement development (settlements developing in accordance with main roads)",
  with strip buildings whose open end faced the road. Source:
  https://warwick.ac.uk/fac/arts/classics/warwickclassicsnetwork/romancoventry/resources/tripontium/life
- **Street villages grow until too long, then branch.** The Straßendorf (street village) is a
  linear settlement along a road. Once it reaches a length where lengthwise growth is no
  longer practical, new streets are added parallel or at right angles to the main street.
  Source: https://repositum.tuwien.at/handle/20.500.12708/193934?mode=full
- **Planted medieval towns put the market at the road.** In Herefordshire, planned towns had
  a market place "often wedge-shaped" at one end, with plots laid out "with the narrow front
  to the main street". Towns sat on strategic sites "such as on a river crossing".
  Source: https://htt.herefordshire.gov.uk/herefordshires-past/the-medieval-period/towns/the-origin-of-towns
- **Some road networks were built to tie existing centres together.** The Inca Qhapaq Ñan
  (over 30,000 km) was "a political project ... linking towns and centers of production and
  worship". It grew over several centuries from earlier Andean roads and peaked in the 15th
  century. Its four main routes start from the central square of Cusco.
  Source: https://whc.unesco.org/en/list/1459/ (fetched with curl; WebFetch was blocked).
- **Other networks were inherited, not built.** The Persian Royal Road (Sardis to Susa, 111
  stations, about 2,500 km) was probably an older Assyrian route that the Persians took over.
  Source: https://www.livius.org/articles/concept/royal-road/

#### Case studies

1. **Oxford, England.** Era: Middle Saxon to 10th century.
   - First: the river crossing. "Middle Saxon activity around the Thames River Crossing at
     St Aldates had developed into a defended settlement (known as a burh) centred on the
     Carfax crossroads by the early 10th century."
   - Plan trace: four streets meeting at a central crossroads (Carfax). The street market
     "occupied the roads meeting at Carfax". Burgage plots front the axial streets. The
     Norman castle was later laid over existing streets in the west end.
   - Source: https://www.oxford.gov.uk/downloads/file/956/historic-urban-characterisation--introduction-and-overview-map (p. 2)
2. **Frankfurt am Main, Germany.** Era: Roman (late 1st century) to Carolingian (794).
   - First: a ford on a defensible island hill (the Domhügel), reachable dry-shod only from
     the west. The name means "ford of the Franks". A Roman road from Mainz to Heddernheim
     ran through the higher parts of the valley. The place sat where major trade routes met.
   - Plan trace: old core on the hill at the crossing; Charlemagne's palace (794); walls and
     ditches from 838. A bridge date was not found in this source.
   - Source: https://en.wikipedia.org/wiki/History_of_Frankfurt (encyclopedia, not scholarly).
3. **Maastricht, Netherlands.** Era: Roman (1st century CE onward).
   - First: the road. The Romans paved "an already existing road" from Bavay and Tongeren
     to Cologne. A bridge over the Meuse existed by 70 CE. Later a castrum (late Roman fort,
     about 170 x 90 m, two gates) guarded the bridgehead.
   - Plan trace: the ancient route "can still be followed" through Tongerseweg, Vrijthof,
     Havenstraat and Plankstraat to the bridge. The medieval core grew around the fort and
     the tomb of St Servatius.
   - Source: https://livius.org/articles/place/traiectum-ad-mosam-maastricht
4. **Alnwick, England.** Era: Anglian (pre-1066) to 12th century.
   - First: probably the road junction. Conzen argued the town began where routes from
     Eglingham, Edlingham and Whittingham met and headed for the coast. This is a hypothesis;
     the survey says there is no physical proof. The Great North Road crossed the Aln here;
     a bridge existed from the 12th century.
   - Plan trace: a large triangular market place at the junction, flanked by burgage strips
     running back from Bondgate, Narrowgate, Fenkle Street and Market Street. The triangle
     was open in the Middle Ages and later built over (infill). A town gate stood where the
     road met the wall (Bondgate). The castle sits beside, not on, the junction.
   - Source: https://www.northumberland.gov.uk/NorthumberlandCountyCouncil/media/Planning-and-Building/Conservation/Archaeology/Alnwick.pdf
     (Northumberland Extensive Urban Survey, 2009, pp. 9-13).
5. **Innsbruck, Austria (pass-route town).** Era: 12th century.
   - First: the trade route over the Brenner Pass, part of the Via Imperii (an imperial road
     under royal protection). The town sits at the Inn crossing on that route. The name means
     "bridge over the Inn". The Counts of Andechs acquired the town in 1180. Transit revenue
     made it rich.
   - Plan trace: the bridge is the town's emblem (on its arms since 1267). I did not find a
     source on how the old-town street follows the route.
   - Source: https://en.wikipedia.org/wiki/Innsbruck (encyclopedia). Dates (1133 vs 1180)
     differ between sources.
6. **Tripontium and Letocetum, Watling Street, England.** Era: Roman (1st-4th century).
   - First: the road. Watling Street was a military road. Mansiones and horse-changing posts
     were placed at intervals (about 10-20 miles, a day's ride). Small towns grew around them.
   - Plan trace: ribbon development; strip buildings with an open end on the road (shops,
     stalls, workshops) and private space behind. No planned grid.
   - Source: https://warwick.ac.uk/fac/arts/classics/warwickclassicsnetwork/romancoventry/resources/tripontium/life
7. **Lugdunum (Lyon), France.** Era: 43 BCE onward.
   - First: the town, then the roads. Founded at the Rhône-Saône junction for Roman refugees.
     It was not an important existing oppidum. Agrippa then built roads from it in four
     directions, which made it the hub of Roman Gaul. A case of "roads attracted by a town".
   - Plan trace: not covered in this source.
   - Source: https://web.econ.ku.dk/dalgaard/Work/WPs/Romanroads.pdf, pp. 13-14.
8. **London, England.** Era: Roman, then 7th-9th century.
   - First: a Roman-founded town (not a major Iron Age site) at the centre of the road
     network. It stopped functioning around 450 CE. About 150 years later a trading place
     grew a few km outside the walls. After Viking raids in the 9th century people moved back
     inside the surviving Roman walls.
   - Plan trace: old walls and road ends re-used centuries later; town moved, then moved back.
   - Source: https://cep.lse.ac.uk/pubs/download/dp1248.pdf, footnote 36.
9. **Richmond and Augusta, USA (portage towns).** Era: 1700s.
   - First: the break in river transport (the fall line). In 1733 Byrd wrote that the
     uppermost landings on the James and Appomattox "are naturally intended for Marts".
     These became Richmond and Petersburg. Augusta grew where Native traders' canoes met
     colonists at the falls of the Savannah. Roads and railways came later; the first railway
     paralleled the river and bypassed the transfer point, yet the cities stayed.
   - Plan trace: not covered in this source.
   - Source: https://personal.lse.ac.uk/sturmd/brixen/pdf/Bleakley-Lin-QJE-2012.pdf, pp. 594-595.
10. **Agadez, Niger (trans-Saharan caravan town).** Era: 15th-16th century.
    - First: the caravan crossroads and nomad camps. Tuareg tribes settled "respecting the
      boundaries of old encampments, which gave rise to a street pattern still in place
      today". The centre is "an important crossroads of the caravan trade".
    - Plan trace: 11 quarters of irregular shape, each following an old camp's outline.
    - Source: https://whc.unesco.org/en/list/1268/ (fetched with curl).
11. **Timbuktu, Mali.** Era: founded 5th century; peak 15th-16th century.
    - First: the meeting point of desert and river trade, "at the gateway to the Sahara
      desert ... near to the river". It was "a crossroads and an important market place"
      where salt from Teghaza met gold, cattle and grain from the south.
    - Plan trace: not covered in this source beyond the three great mosques.
    - Source: https://whc.unesco.org/en/list/119/ (fetched with curl).
12. **Aleppo and Damascus, Syria.** Era: Hellenistic and Roman to Ottoman.
    - First: the crossroads. Aleppo is "at the crossroads of several trade routes since the
      2nd millennium B.C."; Damascus sits "at the crossroads of the orient and the occident".
    - Plan trace: the Graeco-Roman street layout survives under the medieval city. Damascus
      "is based on a Roman plan" with streets oriented north-south or east-west. In Aleppo
      the Hellenistic via recta (straight street) was the main axis; the Roman land grid was
      measured from its crossing with the main cardo next to the Great Mosque. Caravanserais
      (khans) sit in the old fabric.
    - Sources: https://whc.unesco.org/en/list/21/ and https://whc.unesco.org/en/list/20/
      (curl), and https://maxvanberchem.org/en/scientific-activities/projects/architecture/14-architecture/127-bab-qinnasrin-in-aleppo-structure-of-the-urban-fabric
      (curl). The claim that the souq replaced the colonnaded street step by step comes from
      a slide record I could not open: (unverified).
13. **Yoshida-juku, Tōkaidō, Japan (post town).** Era: established 1601.
    - First: the highway. The Tōkaidō had 53 official post stations.
    - Plan trace: the post town "stretched for 2.6 kilometers along the highway". In 1802 it
      had 2 honjin (official inns for lords), 1 waki-honjin (secondary inn) and 65 inns. It
      lay inside the castle town of Yoshida.
    - Source: https://en.wikipedia.org/wiki/Yoshida-juku (encyclopedia).
14. **Huánuco Pampa, Peru (Inca road centre).** Era: Inca, 15th century.
    - First: the road and the state together. One of at least six "New Cuscos" on the main
      Cusco-Quito road. The highway typically passed through the plaza, as in Cusco.
    - Plan trace: a central plaza of 550 x 350 m with an ushnu (ceremonial platform);
      pathways radiate from the plaza and divide the town into up to 12 sectors; 497
      storehouses; over 4,000 buildings on about 2 km².
    - Source: https://en.wikipedia.org/wiki/Hu%C3%A1nuco_Pampa (encyclopedia).

#### Quantitative evidence

| Study | Region, period | Key numbers |
|---|---|---|
| Bosker & Buringh, *City seeds*, J. Urban Econ. 98 (2017) 139-157 | Europe, 800-1800, 250,000+ candidate sites | Base chance of becoming a city about 12%. Navigable river: about +6 percentage points. On a Roman road: no gain; non-hub Roman road sites slightly worse. Roman road hub: positive only before 1600. Existing city 20-100 km away: about +1 point; 0-20 km or over 100 km: no effect. Rival site within 20 km: negative. |
| Dalgaard, Kaarsen, Olsson & Selaya (2018 WP; 2021 rev.) | Roman Empire at 117 CE, 1°x1° cells | Roman road density explains about 12% of modern road density; elasticity 0.24 (0.15 with controls). Late-antique settlement elasticity 0.5-1; modern activity 0.5-1. No effect where the wheel was abandoned (N. Africa, Middle East). Germany frontier strip: Roman road raises chance of a medieval market town by 0.3 points on a 0.22% base. |
| Michaels & Rauch, *Resetting the urban network*, Econ. J. 128 (2018) 378-412; CEP DP 1248 (2013) | Britain vs France, 117-2012, 1 km grid | Persistence odds ratio about 7-13 in Britain, 20-40 in France. 3 of 20 largest British cities near a Roman town vs 16 of 20 in France. Towns with coastal access: 52% Roman Britain, 77% medieval Britain; France 54% to 51%. In 1700, 42% of French urban population lacked good water access vs 13% in Britain. |
| Bleakley & Lin, *Portage and path dependence*, QJE 127 (2012) 587-644 | US South, Mid-Atlantic, Midwest | Within 15 miles of a portage: +50% to +110% population density today. 10% farther away: 6% lower tract density, 2% lower by lights and county. |

#### What this means for a generator

- **Lay down the long-distance roads before the town, as with rivers, but give each road a
  date or era relative to the founding.** The evidence supports both orders. Road first:
  Maastricht, Watling Street towns, Tōkaidō post towns, Innsbruck, Alnwick. Town first, roads
  pulled in: Lugdunum, Cusco, Inca centres, Persian royal cities. So the spec must say, per
  road, `predates_town` or `built_to_town`. A `predates_town` road is fixed terrain the town
  must fit. A `built_to_town` road is placed after the core, aimed at a gate.
- **Treat the crossing point as the seed.** Most cases begin at a river crossing (Oxford,
  Frankfurt, Maastricht, Innsbruck), a road junction (Alnwick, Oxford's Carfax), or a break of
  transport (fall-line towns, Timbuktu). The generator needs the river and the roads first,
  then finds their intersections, then places the core there. Founding causes the spec must
  express: `ford`, `bridge`, `road_junction`, `pass_foot`, `portage` (break of transport),
  `caravan_halt` (camp or caravanserai), `post_station`, `planted_by_lord`, `fort_or_colony`.
  (`castle`, `monastery` and `port` belong to other research slices.)
- **Make the old road the main street, and put the market on it.** Plan traces to generate:
  market place at the crossing (Carfax) or as a triangle/wedge in the fork of roads
  (Alnwick, Herefordshire planted towns); burgage plots with narrow fronts on the road;
  gates where roads meet the wall; ribbon or street-village growth along a road
  (Watling Street, Tōkaidō). A street village that gets too long branches into parallel or
  perpendicular streets (Straßendorf study).
- **Let the market triangle fill in over time.** Alnwick's open triangle was later built over.
  An "era" or "age" parameter can turn an open market into infilled blocks with a narrower
  street.
- **Road value depends on culture and era; make it a rule-pack weight, not a constant.** In
  Europe, roads predicted towns until about 1600, then water took over. Where carts were
  abandoned for camels, roads stopped mattering and caravan routes (no paved road, wide
  tracks, camp-shaped quarters as at Agadez) took their place. A rule pack should be able to
  set "road attraction" against "river attraction".
- **Allow re-founding on old traces.** London and post-Roman Britain show a town abandoned
  and re-occupied inside surviving walls, or shifted beside them. The spec could express
  "ruined predecessor" so old road ends and wall lines exist before the current town is laid.

#### Sources

Fetched and read (by WebFetch, or by curl plus local text extraction where WebFetch could not
read the page):
- https://repub.eur.nl/pub/99124
- http://www.aeaweb.org/conference/2012/retrieve.php?pdfid=203
- https://web.econ.ku.dk/dalgaard/Work/WPs/Romanroads.pdf
- https://web.econ.ku.dk/pabloselaya/papers/RomanRoads.pdf
- https://ideas.repec.org/p/cep/cepdps/dp1248.html
- https://cep.lse.ac.uk/pubs/download/dp1248.pdf
- https://personal.lse.ac.uk/sturmd/brixen/pdf/Bleakley-Lin-QJE-2012.pdf
- https://www.oxford.gov.uk/downloads/file/956/historic-urban-characterisation--introduction-and-overview-map
- https://livius.org/articles/place/traiectum-ad-mosam-maastricht
- https://www.livius.org/articles/concept/royal-road/
- https://en.wikipedia.org/wiki/History_of_Frankfurt
- https://en.wikipedia.org/wiki/Innsbruck
- https://en.wikipedia.org/wiki/Yoshida-juku
- https://en.wikipedia.org/wiki/Hu%C3%A1nuco_Pampa
- https://warwick.ac.uk/fac/arts/classics/warwickclassicsnetwork/romancoventry/resources/tripontium/life
- https://htt.herefordshire.gov.uk/herefordshires-past/the-medieval-period/towns/the-origin-of-towns
- https://repositum.tuwien.at/handle/20.500.12708/193934?mode=full
- https://www.northumberland.gov.uk/NorthumberlandCountyCouncil/media/Planning-and-Building/Conservation/Archaeology/Alnwick.pdf
- https://whc.unesco.org/en/list/1459/
- https://whc.unesco.org/en/list/1268/
- https://whc.unesco.org/en/list/119/
- https://whc.unesco.org/en/list/21/
- https://whc.unesco.org/en/list/20/
- https://whc.unesco.org/en/list/1442/
- https://maxvanberchem.org/en/scientific-activities/projects/architecture/14-architecture/127-bab-qinnasrin-in-aleppo-structure-of-the-urban-fabric
- https://en.banglapedia.org/index.php/Sher_Shah

#### Not found

- **Bairoch, *Cities and Economic Development* (1988):** not read directly. Only the p. 143
  quote, as cited by Bosker & Buringh.
- **Bosker & Buringh final 2017 tables:** not read. The numbers come from the 2012 working
  paper.
- **Michaels & Rauch 2018 final journal text:** paywalled. The numbers come from CEP DP 1248
  (2013).
- **The share of all towns that are road-first vs town-first:** not found. No study gives a
  single percentage. Bosker & Buringh measure probabilities, not founding order.
- **A scholarly source on a town that grew around a caravanserai or a Grand Trunk Road
  sarai:** not found. Banglapedia confirms sarais at regular intervals on Sher Shah's roads
  but says nothing about towns growing around them. The claim of about 1,700 sarais appears
  only on non-scholarly sites: (unverified).
- **Historic England list entry 1003811 (Watling Street small town and mansio):** blocked by
  Cloudflare. Its wording, as quoted in a search summary, is (unverified).
- **The Aleppo colonnaded street turning into the souq step by step:** the Qatar University
  record was blocked (403): (unverified).
- **Innsbruck old-town street following the Brenner road:** not found.
- **Frankfurt bridge date:** not found in a fetched source.
- **Chinese examples** (Grand Canal towns, Silk Road oasis towns): not researched in depth.
  The UNESCO Silk Roads page (1442) mentions posthouses, caravanserai and way stations along
  the 8,700 km Chang'an-Tianshan corridor, but no town plans.

### 4B. Fortifications and site

Date: 2026-10-20. Slice: did forts go to naturally defensible sites, or did a crossing, route or
settlement come first and a fort follow to control it? Did towns then grow under the fort?

Method note. Most of the pages I could fetch are Wikipedia articles, which summarise and cite the
scholarship. I could not read the scholarly books themselves: Beresford, Creighton and Higham,
Haslam, and Baker and Brookes are all paywalled or print-only. Where only a search-result snippet
backed a claim and I could not fetch the page, the claim is marked "(unverified)".


Terms used below:
- **Burh**: an Anglo-Saxon fortified town or fort, built mostly under King Alfred (late 9th century).
- **Burghal Hidage**: a 10th-century list of burhs, giving the manpower each one needed.
- **Motte-and-bailey**: an early Norman castle made of an earth mound (motte) and a fenced yard (bailey).
- **Jōkamachi**: a Japanese "castle town" built around a daimyō's (regional lord's) castle.
- **Ostrog**: a Russian wooden stockade fort, used in Siberia from the 1600s.
- **Presidio**: a Spanish frontier fort with a garrison.
- **Oppidum** (plural oppida): a large fortified Iron Age Celtic settlement.
- **Bridgehead**: a fortified position that guards one end of a bridge or crossing.
- **Llys**: a Welsh prince's court or palace.

#### Findings

- **The route usually came before the fort.** In most cases I checked, the crossing, harbour or road existed first and the fort was put there to control it. Examples: Wallingford (Thames ford, then burh, then castle), Conwy (river crossing), Newcastle (Roman bridge, then fort, then Norman castle), Montreal/Shobak (Egypt–Syria passage), Pfalzgrafenstein (Rhine toll). Sources: [Wallingford](https://en.wikipedia.org/wiki/Wallingford,_Oxfordshire), [Conwy Castle](https://en.wikipedia.org/wiki/Conwy_Castle), [The Castle, Newcastle](https://en.wikipedia.org/wiki/The_Castle,_Newcastle), [Montreal castle](https://en.wikipedia.org/wiki/Montreal_(Crusader_castle)), [Pfalzgrafenstein](https://en.wikipedia.org/wiki/Pfalzgrafenstein_Castle).
- **Burhs were sited on routes and water.** Burhs were placed "on strategic sites on the coast, near ports or overlooking roads and trade routes", and many were "also situated on rivers". Many reused Roman walls (Winchester, Exeter, York, Dover) or Iron Age hill-fort ditches. New planned burhs were laid out on flat ground in rectangular grids (Oxford, Wallingford, Cricklade, Wareham). Alfred apparently intended no farm or village to be more than 20 miles (32 km) from a burh. Source: [Burh](https://en.wikipedia.org/wiki/Burh).
- **Burh sites were sometimes replaced.** Some burhs were temporary forts later replaced by a better-placed town: Pilton by Barnstaple, Halwell by Totnes and Kingsbridge (Haslam is cited for this). The list includes "former Roman towns", "temporary forts and substantial new towns". Source: [Burghal Hidage](https://en.wikipedia.org/wiki/Burghal_Hidage).
- **Norman urban castles were inserted into existing towns, at their corners, next to the river.** A paper by Michael Fradley (Exeter, read by Oliver Creighton in 2010) says urban castles were "imposed within the corners of urban defensive circuits". They were often on level ground beside a river, showing a "clear relationship" with the river system. They often reused late Saxon burh sites, and some pre-Conquest churches were demolished to make room. Source: [Medievalists.net, "Norman Imposition"](https://www.medievalists.net/2010/06/norman-imposition-the-medieval-castle-and-the-urban-space-1050%e2%80%931150/).
- **Norman new towns were laid out at the castle gate, around a market.** "Many new towns grew up outside a lord's castle or abbey gates", which provided "customers for the town market" (Windsor, Ludlow, St Albans). The layouts centred on a market place, either a square or a long wide street, with narrow plots set end-on to the street. On the Welsh border there were "numerous failed boroughs which never became more than villages". Source: [Mondes normands, Town life](https://mondes-normands.caen.fr/angleterre/cultures/gb_fr/culture4_3.htm).
- **Castles and towns were placed along waterways.** Rebecca Tyson (2023) argues that William fitzOsbern's castles and towns in the southern Welsh March were "strategically positioned along waterways to facilitate trade and control maritime navigation". Source: [Bristol research portal](https://research-information.bris.ac.uk/en/publications/castles-commerce-and-control-william-fitzosbern-and-the-managemen/).
- **Creighton on castles and towns (unverified).** Search snippets say that Creighton's *Castles and Landscapes* (2005) finds castle-building mostly rural, with urban castles mainly royal. They also say his 2004 bailey paper finds that whole settlements were sometimes planned or re-planned around castles, with baileys acting as market foci. I could not fetch either text (HTTP 402 and 403). Sources tried: [Equinox chapter](https://equinoxpub.com/home/view-chapter/?id=20395), [Exeter repository](https://ore.exeter.ac.uk/repository/handle/10036/30136).
- **Jōkamachi plan rules.** Samurai districts surrounded the castle, and "higher-ranked vassals owned a compound closer to the castle". Merchant and craft districts lay "outside Samurai-machi". Temple districts were "placed on the outer rim" and "contributed to reinforcing the city's defense". The design aimed to "stimulate commerce by reworking the closest main road to pass through the city". The main road ran "through the front of the castle rather than the back … regardless of geographical concerns". Inside the town, roads were "cranked or had dead ends to elongate the route to the castle". The source also notes that sites that "emphasized the castle's defensive abilities did not necessarily offer good access". Source: [Jōkamachi](https://en.wikipedia.org/wiki/J%C5%8Dkamachi).
- **Japanese castles moved from mountains to plains.** Mountain castles (yamajiro) were the most common type and "provided the best natural defenses". Azuchi (1570s) was "located on a plain", and the article links the change mainly to firearms. Under the Tokugawa each domain was limited to one castle, with exceptions. Source: [Japanese castle](https://en.wikipedia.org/wiki/Japanese_castle).
- **Oppida usually went from an open settlement to a fortified one, then down to the plain.** "The first were open settlements, followed by fortified oppida." "Most oppida are situated on heights, but some are located on flat areas of land", in elevated positions that let them dominate nearby trade routes. After the Roman conquest, the change to a Roman town "often involved a change of location from the hilltop into the plain". Source: [Oppidum](https://en.wikipedia.org/wiki/Oppidum).
- **Chinese walls were administrative and ranked.** "The size of the walled area and the elaboration of wall construction were normally directly proportional to the rank of the city in the administrative hierarchy." The ideal was a square with symmetric gates and the main gate in the centre of the south wall. "In areas of rugged relief, however, a square form was usually replaced by one of irregular shape." Later, "an outer wall was often erected to enclose settlement that had spread outside the city". Walls often enclosed more land than was built up. "A city was not a city without walls." Source: [Chinese city wall](https://en.wikipedia.org/wiki/Chinese_city_wall).
- **Hill forts lose to the plain when the threat changes.** Chittorgarh's plains "Lower Town" developed after 1568, once artillery made the hill fort obsolete. Golconda's capital moved to Hyderabad on the Musi River "to extend the capital beyond the fortified Golconda" and avoid "water shortages experienced at Golconda". Sources: [Chittorgarh Fort](https://en.wikipedia.org/wiki/Chittorgarh_Fort), [Hyderabad](https://en.wikipedia.org/wiki/Hyderabad), [Golconda Fort](https://en.wikipedia.org/wiki/Golconda_Fort).

#### Case studies

**1. Wallingford, England. 9th to 11th century.**
- First: route (the Thames ford), then a burh, then a castle.
- Evidence: Wallingford "developed around an important crossing point". By 1066 it was "the lowest point at which the river could be forded". Alfred enclosed it "with substantial earthworks" in the 9th century. After 1066 a motte-and-bailey castle was "Established … within an Anglo-Saxon burgh". The first bridge is recorded in 1141.
- Plan trace: a rectangular burh enclosure (the source calls it "four roughly square areas around the centre"), with the Norman castle inserted inside the burh.
- Source: [Wallingford](https://en.wikipedia.org/wiki/Wallingford,_Oxfordshire). The grid plan is from [Burh](https://en.wikipedia.org/wiki/Burh).

**2. Ludlow, England (Welsh March). About 1075 to the 12th century.**
- First: a defensible site (a hill promontory in a bend of the River Teme), then the castle, then a planned town at its gate.
- Evidence: Roger de Lacy "began the construction of Ludlow Castle on the western promontory of the hill about 1075". The first street ran along the ridge as a wide marketplace "running from the castle gates east". The town grew on a grid "adapted somewhat to match the local topography".
- Plan trace: the castle sits at the high end of the ridge, and the market street runs straight from its gate. Old Street ran down to a ford on the ancient route south. That ford was later replaced by a bridge at the foot of Broad Street, so the crossing moved to suit the new town.
- Source: [Ludlow](https://en.wikipedia.org/wiki/Ludlow). The "grid attached to the market place" is from [Mondes normands](https://mondes-normands.caen.fr/angleterre/cultures/gb_fr/culture4_3.htm).

**3. Conwy, Wales. 1283 to 1287.**
- First: a route (the river crossing) and an existing settlement (the abbey and a princely court), then the castle and walled town together.
- Evidence: the site "controlled an important crossing point over the River Conwy". It held Aberconwy Abbey and a llys. Edward I moved the abbey about eight miles up the valley to Maenan. Castle and town walls were one project. Under the 1284 charter the castle's constable was also the town's mayor. Part of the llys was "incorporated into the wall". The source calls the choice of site "in part a symbolic act to demonstrate English power".
- Plan trace: the castle forms one end of the town-wall circuit. The older settlement was cleared and replaced.
- Source: [Conwy Castle](https://en.wikipedia.org/wiki/Conwy_Castle).

**4. Caernarfon, Wales. Roman period, then 1088 onward, then 1283.**
- First: a site with water access (a peninsula between the Seiont and the Menai Strait), reused by the Romans nearby, then a Norman motte, then the Edwardian castle and town.
- Evidence: Hugh d'Avranches built a motte-and-bailey after 1088 "on a peninsula, bounded by the River Seiont and the Menai Strait". Edward I's layout was "mostly dictated by the lie of the land" and incorporated the earlier motte. The town walls were "connected to the castle". The source says "houses were cleared to allow the construction", with compensation paid three years later.
- Plan trace: the castle is on the waterside edge of the walled town, and the walls join the castle.
- Source: [Caernarfon Castle](https://en.wikipedia.org/wiki/Caernarfon_Castle).

**5. Newcastle upon Tyne, England. 2nd century and 1080.**
- First: a route (the Roman bridge Pons Aelius), then a Roman fort guarding it, then the Norman castle on the same spot, which gave the town its name.
- Evidence: the Roman fort sat on "a rocky outcrop overlooking the new bridge". Robert Curthose built a wooden motte-and-bailey in 1080 on the fort site, and Henry II built the stone keep in 1172–77. The town walls (from about 1265) took over the defensive role, and from the early 17th century houses and shops were built on the castle site.
- Plan trace: a classic bridgehead. The fort is on high ground at the crossing, and the town grows next to it and is later walled.
- Source: [The Castle, Newcastle](https://en.wikipedia.org/wiki/The_Castle,_Newcastle).

**6. Pfalzgrafenstein and Kaub, Germany. 1326/27.**
- First: a route (the Rhine shipping lane) and an existing town (Kaub), then a toll fort in the river.
- Evidence: Ludwig IV built it to increase toll revenue, with chain booms across the river. Tolls were paid at a customs house in Kaub because ships could not dock at the island. Gutenfels Castle above Kaub also collected tolls.
- Plan trace: a fort placed on the route itself, not on high ground. The town supplies the services.
- Source: [Pfalzgrafenstein Castle](https://en.wikipedia.org/wiki/Pfalzgrafenstein_Castle).

**7. Montreal (Shobak), Transjordan, Crusader states. 1115.**
- First: a route (the main passage from Egypt to Syria and the pilgrim road to Mecca), then a castle on a defensible hill beside it.
- Evidence: built by Baldwin I on a "round hilltop site". It "dominated the main passage from Egypt to Syria", and its holder could tax traders and pilgrims.
- Plan trace: no Crusader town is described. The modern town of Shoubak lies next to the ruins.
- Source: [Montreal (Crusader castle)](https://en.wikipedia.org/wiki/Montreal_(Crusader_castle)).

**8. Himeji, Japan. 1333, 1346, 1601–09.**
- First: a defensible site (Himeyama hill), then a series of forts and castles. The town came later; its date was not found.
- Evidence: the hill "was first fortified in 1333 by Akamatsu Norimura" and replaced by a castle in 1346. Ikeda Terumasa added three moats in 1601–09. Approaching attackers travel "in a spiral pattern around the complex".
- Plan trace: a spiral approach inside the castle. Why the site was chosen, and whether the Sanyōdō road was moved, were not found.
- Source: [Himeji Castle](https://en.wikipedia.org/wiki/Himeji_Castle).

**9. Kanazawa, Japan. Late Muromachi period, 1580, 1592.**
- First: a defensible site, already occupied by a fortified temple (the Kanazawa Gobō of the Ikkō-ikki), with a town growing around it. Then the castle replaced the temple.
- Evidence: the Gobō was a "natural fortress", backed by high hills and flanked on two sides by rivers. A jōkamachi grew around the temple fortress, "the start of what would become the city of Kanazawa". Sakuma Morimasa (1580) began the moats and the town layout. Later, temples were grouped some distance from the castle.
- Plan trace: the castle is on the highest ground between the Sai and Asano rivers, with temples on the outskirts.
- Source: [Kanazawa Castle](https://en.wikipedia.org/wiki/Kanazawa_Castle).

**10. Edo, Japan. Late 12th century, 1457, 1590–1636.**
- First: a fortified residence on a waterside site, then Ōta Dōkan's castle (1457), then the Tokugawa rebuild with land reclamation for the town.
- Evidence: the area "is bordered by either the sea or the Kanda River, allowing ships access". "Initially, parts of the area were lying under water." Earth from the moats was used as landfill, which laid groundwork for merchant settlement. The moats form "roughly concentric circles".
- Plan trace: the town ground was literally made from the castle's moats.
- Source: [Edo Castle](https://en.wikipedia.org/wiki/Edo_Castle).

**11. Tobolsk and Yakutsk, Siberia. 1587 and 1632/1643.**
- First: a route (the rivers) and an existing native centre, then the ostrog, then the town.
- Evidence: Tobolsk was founded on 14 June 1587 by Cossacks under Daniil Chulkov, at the Tobol–Irtysh confluence and "near the ruins of Qashliq", the Siberian Tatar capital. Its Kremlin is "on a high river bank". Yakutsk began as the Lensky Ostrog in 1632, about 70 km from the present site. It was moved to the Tuymaada valley in 1642–43; the reason was not found. The ostrog article says "Many of these forts later transformed into large Siberian cities."
- Plan trace: the fort is on a river bank at a confluence. A first site could be abandoned and the fort moved.
- Sources: [Tobolsk](https://en.wikipedia.org/wiki/Tobolsk), [Yakutsk](https://en.wikipedia.org/wiki/Yakutsk), [Ostrog](https://en.wikipedia.org/wiki/Ostrog_(fortress)).

**12. San Antonio and Tucson, Spanish frontier. 1718 and 1775.**
- First: a settlement (a mission, or an Indigenous village by a river), then the presidio to protect it, then a civil town around the presidio.
- Evidence: the San Antonio presidio (1718) was built "to protect Mission San Antonio de Valero", about half a mile from it. "As settlers concentrated around the presidio complex and mission, it began to form the town of Béjar." Its Plaza de Armas was later anchored by the Governor's Palace (1749). The Tucson presidio site (1775) is "on the east terrace overlooking the Santa Cruz River floodplain", near the O'odham village Cuk Ṣon. The fort was "the founding structure of what became the city of Tucson". More generally, presidios protected the silver route (Camino Real de Tierra Adentro) and confined Indigenous groups.
- Plan trace: a plaza in front of the fort becomes the town centre.
- Sources: [Presidio San Antonio de Béxar](https://en.wikipedia.org/wiki/Presidio_San_Antonio_de_B%C3%A9xar), [Presidio of Tucson](https://en.wikipedia.org/wiki/Presidio_San_Agust%C3%ADn_del_Tucs%C3%B3n), [Presidio](https://en.wikipedia.org/wiki/Presidio).

**13. Bibracte and Manching, Celtic oppida. 3rd to 1st century BC.**
- Bibracte. First: a defensible site (three summits of Mont Beuvray), with ramparts enclosing about 200 ha, later reduced to about 135 ha. Plan: a central road from the main gate, artisan quarters "near the gates", and elite houses on the central plateau. It was abandoned after Autun was founded on lower ground about 25 km away, around 15 BC.
- Manching. First: a settlement and cult site (from the 3rd century BC) at "the crossing of two trade routes" in "a riverine plain" near the Paar–Danube confluence. The wall came later: about 150 BC, then about 104 BC.
- Sources: [Bibracte](https://en.wikipedia.org/wiki/Bibracte), [Oppidum of Manching](https://en.wikipedia.org/wiki/Oppidum_of_Manching).

**14. Chittorgarh and Golconda/Hyderabad, India. Medieval to 16th century.**
- First: a defensible site (an isolated plateau rising "180 m above the surrounding plains", or a granite hill), then the fort, then a lower or successor town once the fort lost value.
- Evidence: Chittorgarh's "Lower Town" developed after 1568, and the capital moved to Udaipur. Golconda grew from "a small mud fort" to a granite citadel whose outer wall "enclosed the city". It was a diamond trade centre near the Kollur mine. The capital moved to Hyderabad on the Musi in 1590/91 because of water shortage.
- Plan trace: the high fort with a town inside, followed later by a plains town at the foot or a new city nearby.
- Sources: [Chittorgarh Fort](https://en.wikipedia.org/wiki/Chittorgarh_Fort), [Golconda Fort](https://en.wikipedia.org/wiki/Golconda_Fort), [Hyderabad](https://en.wikipedia.org/wiki/Hyderabad).

**15. Nuremberg, Germany. About 1000 to 1050 (partial).**
- First: a defensible site (a sandstone ridge). A ford 4 km away at Megelendorf was in use in 1025–30. The castle "dominat[es] the historical center". The source does not say whether the town predates the castle: not found.
- Source: [Nuremberg Castle](https://en.wikipedia.org/wiki/Nuremberg_Castle).

#### Patterns

- **Most common order: route, then fort, then town.** This holds for Anglo-Saxon burhs, Norman castles in England and Wales, Rhine toll castles, Crusader route castles and Siberian ostrogs. The fort goes to an existing crossing, river or road (cases 1, 3, 4, 5, 6, 7, 11).
- **Settlement, then fort.** The Norman conquest castles were inserted into existing towns, usually at a corner by the river (Fradley/Creighton). Spanish presidios were built to protect an existing mission or village (case 12), and Conwy was built over an abbey and a princely court (case 3).
- **Defensible site first, then the route bends to it.** Ludlow is the clearest case: the ford and bridge moved to serve the new town. Japanese jōkamachi say outright that the nearest main road was reworked to pass through the town and in front of the castle. Hill-fort cultures (oppida, Indian forts, Japanese yamajiro, Himeji) also pick the high ground first.
- **Defence drifts down to the plain over time.** The Celtic hilltop gave way to the Roman plain (Bibracte to Autun), and the Japanese mountain castle to the plain castle (Azuchi onward). Indian hill forts gave way to plains towns (Chittorgarh lower town, Hyderabad). The causes named are firearms or artillery, water supply, commerce and administration.
- **Chinese county seats are administrative first.** The wall is the defining feature of a city. Its size follows administrative rank, and its shape is ideally square unless the terrain forbids it. The sources I read do not say that a route or crossing caused these foundings (not found).
- **Town placement relative to the fort.** The town at the castle gate with the market below (Ludlow, Windsor) is the European castle-borough pattern. The castle at a corner or edge of the town wall is the European walled-town pattern (Conwy, Caernarfon, urban castles). Rings of rank around a central castle, with temples at the rim, is the Japanese pattern. A plaza in front of the fort is the Spanish pattern. Craft quarters at the gates is the oppidum pattern.

#### What this means for a generator

- **Terrain and water must exist before anything else.** Every fort decision in the evidence reads the terrain: hill, promontory, river bend, confluence, peninsula, ford.
- **A route or crossing layer must exist before the fort, in most founding types.** The spec needs a founding cause that says "control crossing X", "control road Y", "toll the river" or "protect settlement Z". The fort is then placed relative to that object (at the bridgehead, on high ground overlooking it, or in the stream itself).
- **The spec must also allow "defensible site first, routes bend to it".** In that case roads are generated or re-routed after the fort and town, so that the main road passes the castle gate (Ludlow, jōkamachi). This needs a road re-routing stage after the town core is placed, not only before it.
- **Founding causes the spec must express:** crossing control; route or pass control; toll; harbour or water access; protecting an existing settlement or religious site; conquest insertion into an existing town; administrative seat (Chinese); refuge on high ground (oppidum, Indian hill fort); a fort replacing an earlier fort or temple on the same spot (Himeji, Kanazawa, Newcastle).
- **The fort-to-town placement relation must be a parameter, not hard-coded.** The options are: at the gate with the market below; at a corner or edge of the town wall; at the centre with rings of rank; a plaza in front of the fort; inside a large enclosure. The fort must exist before the town core, because the market, the main street and the wall circuit attach to it.
- **Growth stages need a "move down the hill" option.** Optional, for the GM to decide: a later lower town at the foot of the hill, or a successor town on the plain (Chittorgarh, Bibracte to Autun, Golconda to Hyderabad). Town walls can also be added later to enclose suburbs (Chinese outer walls, Newcastle).

#### Sources

All fetched during this research:
- https://en.wikipedia.org/wiki/Burghal_Hidage
- https://en.wikipedia.org/wiki/Burh
- https://en.wikipedia.org/wiki/Wallingford,_Oxfordshire
- https://en.wikipedia.org/wiki/Ludlow
- https://en.wikipedia.org/wiki/Conwy_Castle
- https://en.wikipedia.org/wiki/Caernarfon_Castle
- https://en.wikipedia.org/wiki/The_Castle,_Newcastle
- https://www.medievalists.net/2010/06/norman-imposition-the-medieval-castle-and-the-urban-space-1050%e2%80%931150/
- https://mondes-normands.caen.fr/angleterre/cultures/gb_fr/culture4_3.htm
- https://research-information.bris.ac.uk/en/publications/castles-commerce-and-control-william-fitzosbern-and-the-managemen/
- https://www.medievalists.net/2012/05/the-burh-of-wallingford-and-its-context-in-wessex/ (abstract only; no siting detail)
- https://gatehouse-gazetteer.info/Books/booktext/mtwocrh.html (publisher summary only)
- https://en.wikipedia.org/wiki/Pfalzgrafenstein_Castle
- https://en.wikipedia.org/wiki/Nuremberg_Castle
- https://en.wikipedia.org/wiki/Montreal_(Crusader_castle)
- https://en.wikipedia.org/wiki/J%C5%8Dkamachi
- https://en.wikipedia.org/wiki/Japanese_castle
- https://en.wikipedia.org/wiki/Himeji_Castle
- https://en.wikipedia.org/wiki/Kanazawa_Castle
- https://en.wikipedia.org/wiki/Edo_Castle
- https://en.wikipedia.org/wiki/Tobolsk
- https://en.wikipedia.org/wiki/Yakutsk
- https://en.wikipedia.org/wiki/Ostrog_(fortress)
- https://en.wikipedia.org/wiki/Presidio
- https://en.wikipedia.org/wiki/Presidio_San_Antonio_de_B%C3%A9xar
- https://en.wikipedia.org/wiki/Presidio_San_Agust%C3%ADn_del_Tucs%C3%B3n
- https://en.wikipedia.org/wiki/Bibracte
- https://en.wikipedia.org/wiki/Oppidum
- https://en.wikipedia.org/wiki/Oppidum_of_Manching
- https://en.wikipedia.org/wiki/Chittorgarh_Fort
- https://en.wikipedia.org/wiki/Golconda_Fort
- https://en.wikipedia.org/wiki/Hyderabad
- https://en.wikipedia.org/wiki/Chinese_city_wall

#### Not found

- **Beresford, *New Towns of the Middle Ages* (1967).** No text reachable. A search snippet says it has a chapter "Beau Lieu: or, the Choice of Site" and covers 171 English and 84 Welsh plantations (unverified).
- **Creighton and Higham, *Medieval Town Walls* (2005).** Only a publisher summary was reachable. No statement on siting or on castle and town-wall chronology.
- **Creighton, *Castles and Landscapes* and his 2004 bailey paper.** Paywalled or forbidden (HTTP 402 and 403). The claims in Findings are unverified.
- **Haslam and Baker & Brookes on burh siting.** Only abstracts or blurbs were reachable. Baker and Brookes stress communications and infrastructure, per the publisher blurb seen in search (unverified).
- **The Cricklade VCH entry (Ermin Street crossing).** HTTP 401.
- **Why the Himeji hill was chosen, the town's date, and Sanyōdō diversion.** Not found.
- **Why Yakutsk was moved in 1642–43.** Not found.
- **The Laws of the Indies site rules for presidios.** Not found.
- **German Burgstädte as a category.** Not found beyond Nuremberg, and Nuremberg's town-versus-castle order was not found.
- **A Crusader town attached to a castle (a burgus).** Not found.
- **Chinese county-seat siting criteria** (rivers, fengshui, roads). Not found in a fetched source. Skinner's *The City in Late Imperial China* and Knapp's *China's Walled Cities* were not reachable. The Leiden thesis returned HTTP 502.

### 4C. Planned foundations

Scope: cities founded on purpose by a state, coloniser or ruler. What site criteria the founders
stated or applied, whether something already stood there, and what plan followed.

Method: WebSearch and WebFetch, October 2026. Every claim below carries the URL it was read
from. Quotes are copied from the fetched page and kept short. A claim seen only in a search
result snippet, not on a fetched page, is marked "(unverified)".

Terms used once and explained here:

- **apoikia**: a Greek colony, a new self-governing city sent out by a mother city.
- **colonia**: a Roman town with full Roman legal status, often settled by army veterans.
- **castra**: a Roman military camp or fortress.
- **cardo / decumanus**: the north-south and east-west main streets of a Roman grid.
- **bastide**: a planned new town in 13th-14th century south-west France.
- **paréage**: a contract where two lords share the founding and the income of a new town.
- **ward**: here, a block of the city plan that repeats as the city grows.

#### Findings

1. **Founders wrote down site rules, and the rules repeat across cultures.** Vitruvius
   (Rome, 1st c. BC), Aristotle (Greece, 4th c. BC), the Arthashastra (India) and the Spanish
   Ordinances of 1573 all name the same short list: healthy air, good water, defensible
   ground, access by land and water, farmland and building materials nearby. Each adds a
   local preference about wind and sun direction.
2. **Health is the first criterion in the two most detailed rule-books.** Vitruvius:
   "First comes the choice of a very healthy site." The 1573 Ordinances (no. 34) judge health
   by "the abundance of old men or of young men of good complexion". Both reject marshes.
   Vitruvius tells of a whole town (Salpia) moved four miles off a marsh to a healthy site.
3. **Almost every "new" city sat on or beside something older.** Byzantium (Greek town) under
   Constantinople, Nyenskans under St Petersburg, Jayakarta under Batavia, the Oppidum
   Ubiorum under Cologne, the village Maganuba under Dur-Sharrukin, the village of Sikri under
   Fatehpur Sikri, a pre-Islamic Baghdad under the Round City, Georgetown and Alexandria
   inside the District of Columbia, a Sicel settlement under Syracuse. Only Akhetaten (Amarna)
   was explicitly claimed as empty land "not being the property of a god". The 1573
   Ordinances require a site that is "vacant" or taken with the natives' consent, yet
   ordinance 39 also allows demolishing neighbouring towns for building material.
4. **Order of laying down is stated in the sources.** The 1573 Ordinances give an explicit
   order: choose region, then site, then draw the plan "with cord and ruler" starting from
   the main square, then streets to the gates and roads, then church and royal lots, then
   lots by lottery, then a palisade or ditch around the plaza, then commons, pasture and
   farmland outside. The WIC 1625 instructions for New Amsterdam (survey and plat, then
   fort, then houses, then farms outside) and Savannah (repeating wards with farm tracts
   outside) follow the same pattern: centre first, defence early, fields last.
5. **The centre is placed by rule, and the rule depends on the coast.** Vitruvius: forum by
   the harbour if on the sea, "in the middle of the town" if inland. The 1573 Ordinances
   (no. 112) say the same for the plaza. The Arthashastra puts the palace north of the
   centre; Kaogong ji, Chang'an and Heian-kyō put the palace at the north. Baghdad put
   palace and mosque at the exact centre of a circle.
6. **Planned grids do not stay planned.** Timgad "quickly outgrew its original plans and
   spilled beyond the orthogonal grid in a more loosely organized fashion". Freiburg's
   50 x 100 foot plot norm was dropped for new areas around 1200 and old plots were split.
   Bern grew westward in three wall stages, each marked by a gate tower. New Amsterdam's
   five-bastion plan was never built; a smaller four-sided fort went up instead.
7. **Founders also chose for reasons a map cannot show**: a dream (Alexandria), a god's
   guidance (Amarna, Syracuse's Delphic oracle), a saint's prophecy (Fatehpur Sikri), the
   ruler's name in the wall length (Dur-Sharrukin), politics and frontier distance
   (Constantinople, Isfahan, Baghdad). These matter to a spec as "founding cause" even
   though they do not change the terrain rules.

#### Site-selection rules from the sources

| Source | Culture | Era | Criteria (with source URL) |
|---|---|---|---|
| Vitruvius, *De architectura* Book I ch. 4-7 (Morgan transl.) | Roman | c. 30-15 BC | Ch. 4: "First comes the choice of a very healthy site"; "high, neither misty nor frosty", temperate; no marshes nearby unless by the sea and drainable; test the site by sacrificing grazing cattle and examining their livers. Ch. 5: towers "round or polygonal"; roads approach gates "not in a straight line, but from the right to the left"; town "in circular form", not square. Ch. 6: lay streets so "to exclude the winds from the alleys". Ch. 7: forum "close to the harbour" if on the sea, else "in the middle of the town"; guardian temples "on the very highest point"; Venus, Vulcan, Mars "outside the walls". https://www.gutenberg.org/files/20239/20239-h/20239-h.htm |
| Aristotle, *Politics* VII, parts 5, 6, 11 (Jowett transl.) | Greek | c. 350 BC | Territory "difficult of access to the enemy, and easy of egress to the inhabitants"; city central to its land; access to sea, port may sit outside the walls; east-facing and east winds healthiest; "natural abundance of springs", else cisterns for siege; "an acropolis is suited to an oligarchy or a monarchy, but a plain to a democracy"; mix Hippodamian regular streets with irregular ones for defence; walls are needed. http://classics.mit.edu/Aristotle/politics.7.seven.html |
| Kaogong ji (in the *Rites of Zhou*) | Chinese | about 3rd c. BC (majority view) | Capital "a square nine li on one side; each side has three gates"; "nine north-south and nine east-west streets". https://en.wikipedia.org/wiki/Rites_of_Zhou . Ancestral temple left, altar of soil and grain right, court in front, market behind (unverified: seen only in search snippets, not on a fetched page). |
| Shijin sōō ("four gods correspond") | Japanese, from Chinese models | applied in tradition to 794; first text c. 11th c. | Mountain north, river east, road west, open plain south (unverified: search snippet citing the Sakuteiki). A Kyushu University study says "the first textual evidence of this interpretation of the Four Gods in the landscape postdates the construction of Heian" by at least two centuries. https://kyushu-u.elsevierpure.com/en/activities/pleasing-the-four-gods-shijin-s%C5%8D%C5%8D-site-selection-and-site-adaptat/ |
| Kautilya, *Arthashastra* Book II ch. 3-4 (Shamasastry transl.) | Indian | date not checked | Capital in the centre of the kingdom, "in a locality naturally best fitted for the purpose, such as the bank of the confluence of rivers", or a perennial pool, lake or tank; fort "circular, rectangular, or square"; ringed by a canal, "connected with both land and water paths". Inside: "three royal roads from west to east and three from south to north"; "twelve gates"; palace "to the north from the centre". https://en.wikisource.org/wiki/Arthashastra/Book_II |
| Manasara | Indian | final form c. 700 CE (Acharya) or later | Has chapters on site selection (ch. 4), soil testing (ch. 5), ground plans (ch. 7), village planning (ch. 9), town planning (ch. 10). https://en.wikipedia.org/wiki/Manasara . Sarvatobhadra is one of eight village types in ch. 9. https://www.wisdomlib.org/definition/sarvatobhadra . The criteria themselves: not found. |
| Philip II, Ordinances of 1573 ("Laws of the Indies"), Ministerio de la Vivienda 1973 edition with English translation | Spanish colonial | 13 July 1573 | No. 34: judge health by old men, healthy animals, clear sky; "it is better that it be cold". No. 35: fertile land, pasture, forests for wood, "good and plentiful water". No. 37: "good access and outlet by sea and by land". No. 39: water nearby. No. 40: not too high (wind, hard access), not low (unhealthy), "medium elevation"; hills to west or east; on a river, "on the eastern bank". No. 41: avoid coasts (pirates, health) unless a good harbour is needed. No. 111: "elevated and healthy location; [be] with means of fortification", fertile soil, fuel, timber, fresh water, natives, "open to the north wind"; harbour quality; "not near lagoons or marshes". No. 123: inland towns on a navigable river; filth trades downstream. https://www.huduser.gov/portal/sites/default/files/pdf/The-Laws-of-the-Indies.pdf |
| WIC instructions to Verhulst and Fredericxsz, 1625 | Dutch colonial | April 1625 | Three candidate sites in order: High Island in the Delaware, "the hook of the Manattes", or a third spot; survey and plat; town facing the river with a ditch on three sides; five-sided bastioned fort. https://www.gothamcenter.org/blog/notes-on-the-planned-town-of-fort-amsterdam |

#### Case studies

**1. Syracuse (Sicily)**
- Era: c. 733 BC (Thucydides' date, "generally ... considered the most reliable").
- Founder: colonists from Corinth led by Archias.
- Stated reasons: the Delphic oracle directed Archias to Ortygia island. Ortygia "naturally lent itself to use as a fortified settlement due to its harbors and defensible position".
- Prior settlement: yes, an existing Sicel settlement.
- Plan: island core became the religious and political centre; later activity shifted to the mainland. Mainland quarter layout: not found.
- Sources: https://en.wikipedia.org/wiki/History_of_Syracuse,_Sicily ; https://en.wikipedia.org/wiki/Ortygia

**2. Massalia (Marseille)**
- Era: Greek colony (date not checked).
- Founder: Phocaeans.
- Stated reasons / site: "Its harbour lies at the foot of a theatre-like rock which faces south"; the rock and city well fortified; temples on the headland. Land "too poor for grain" but good for olives and vines (Strabo 4.1.5).
- Prior settlement: not found.
- Plan: harbour below, temples on the headland. Street layout: not found.
- Source: Strabo, *Geography* 4.1.4-5, https://penelope.uchicago.edu/Thayer/E/Roman/Texts/Strabo/4A*.html

**3. Byzantium / Constantinople**
- Era: Greek Byzantium (date not checked); refounded by Constantine 324 AD, consecrated 11 May 330.
- Stated reasons, Greek founding: the "city of the blind" story. Herodotus 4.144: the Persian Megabazus said "the Calchedonians must at that time have been blind, for had they not been, they would never have chosen the worse site". https://fgh.perseids.org/read/greekLit/tlg0016/tlg001/perseus-eng2/4.144.1-4.145.1 . Tacitus, *Annals* 12.63, gives the oracle version: "seek a home opposite the country of the blind"; Byzantium "is favoured with a fertile soil and with a prolific sea", with "huge shoals of fish". https://penelope.uchicago.edu/Thayer/E/Roman/Texts/Tacitus/Annals/12B*.html . Polybius (via Vasiliev): without Byzantium's consent "not a single commercial vessel could enter or leave the Black Sea". https://www.ellopos.net/elpenor/vasilief/foundation-of-constantinople.asp
- Stated reasons, Constantine: Rome was "too far from the frontiers"; Byzantium was "readily defended, with easy access to the Danube or the Euphrates frontiers"; Golden Horn and Sea of Marmara cut the length of land wall needed; on both the Europe-Asia land route and the Black Sea-Mediterranean sea route; "in the Golden Horn an excellent and spacious harbor". https://en.wikipedia.org/wiki/Constantinople
- Prior settlement: yes, the Greek city of Byzantium.
- Plan: 14 regions like Rome; Augustaion square by palace, Hippodrome and Senate; Mese, the main colonnaded street, runs through a chain of forums to the Golden Gate; "several elevations within its walls matched Rome's seven hills"; Constantinian wall, later Theodosian double wall about 2 km further west. Same source.
- Alternatives Constantine considered (Troy, Serdica, Nicomedia): not found on fetched pages.

**4. Alexandria (Egypt)**
- Era: 331 BC (per a search result; not checked on a fetched page).
- Founder: Alexander the Great.
- Stated reasons: Plutarch, *Alexander* 26: a dream of Homer's verses about Pharos; Alexander saw "a strip of land similar to a fairly broad isthmus, running between a large lagoon and the sea" which "terminates in a great harbor".
- Prior settlement: not found.
- Plan: outlined with barley-meal, "a rounded area on the dark soil, to whose inner arc straight lines succeeded" (a cloak shape).
- Source: https://livius.org/sources/content/plutarch/plutarchs-alexander/the-foundation-of-alexandria (Austin transl.)

**5. Akhetaten (Amarna, Egypt)**
- Era: Akhenaten's regnal year 5 (Earlier Proclamation).
- Founder: Akhenaten.
- Stated reasons: "it is Pharaoh who has discovered it: not being the property of a god, not being the property of a goddess"; "it is the Aten, my father, who advised me concerning it".
- Prior settlement: explicitly claimed none.
- Plan: fixed boundaries marked by stelae on both banks; "I shall not expand beyond the southern stela ... nor ... beyond the northern stela"; build on the east bank, "enclosed for Him by the mountain"; promised a House of the Aten, royal apartments, and tombs "in the eastern mountain".
- Sources: https://www.amarnaproject.com/amarna-the-place/boundary-stelae/ ; https://en.wikipedia.org/wiki/Boundary_Stelae_of_Akhenaten

**6. Dur-Sharrukin (Khorsabad, Assyria)**
- Era: built c. 716-706 BC; abandoned as capital after 705 BC.
- Founder: Sargon II.
- Stated reasons: not found in fetched sources. Symbolism: wall length "16280 Assyrian units", said by Sargon to equal the numerical value of his name.
- Prior settlement: "the village on the site was named Maganuba".
- Plan: near-rectangle 1,758.6 x 1,635 m; 157 towers, seven gates; walled terrace with palace and temples on the northern wall; a citadel in the south-west corner.
- Source: https://en.wikipedia.org/wiki/Dur-Sharrukin

**7. Timgad (Thamugadi, Algeria)**
- Era: c. 100 AD.
- Founder: Trajan, as a colonia, settled largely by veterans.
- Stated reasons: "a Roman bastion against the Berbers in the nearby Aures Mountains"; fertile farmland; at the meeting of six roads.
- Prior settlement: none named.
- Plan: orthogonal grid; decumanus maximus and cardo cross at the forum; walled; designed for about 15,000 people; later "spilled beyond the orthogonal grid in a more loosely organized fashion".
- Source: https://en.wikipedia.org/wiki/Timgad (UNESCO page returned 403)

**8. Chester (Deva) and Cologne (castra and colonia that became medieval towns)**
- Chester: fortress begun mid-70s AD by Legio II Adiutrix; reason: "commanding access to the sea via the River Dee and dividing the Brigantes from the Ordovices"; on a sandstone bluff with the river bend on south and west; placed so as not to block the harbour; "playing card" plan, four gates, about 25 ha; in 907 the walls "were repaired and incorporated into the defences" of the burh. https://en.wikipedia.org/wiki/Deva_Victrix
- Cologne: the Ubii were resettled by Agrippa in 38 BC; their centre on "a natural rise that was protected from flooding" became the Oppidum Ubiorum; raised to colonia c. 50 AD; wall almost 4 km with 19 towers and nine gates; forum at the cardo-decumanus crossing; "The grid of Roman streets can still be seen in the street plan of today's Cologne." https://en.wikipedia.org/wiki/Colonia_Claudia_Ara_Agrippinensium

**9. Baghdad, the Round City**
- Era: begun 2 August 762; palace and mosque by 763; the rest by 766.
- Founder: Caliph al-Mansur.
- Stated reasons: control of routes along the Tigris; mild climate; "topography (critical for fortification)"; water. Al-Mansur quoted (via Wiet 1971): "This is indeed the city that I am to found, where I am to live, and where my descendants will reign afterward." https://en.wikipedia.org/wiki/History_of_Baghdad . Primary-source wording: al-Mansur called the site "excellent as a military camp", with the Tigris linking it "to lands as far as China" and the Euphrates bringing goods from Syria (unverified: search snippet of a 1944 British document; al-Tabari and al-Ya'qubi translations not fetched). Ya'qubi's line that the island between Tigris and Euphrates "is a marketplace for the world" (unverified: search snippet of a textbook).
- Prior settlement: yes, a pre-Islamic Baghdad village; Ctesiphon about 30 km away.
- Plan: circle of radius about 1 km; four gates named for Kufa, Syria (al-Sham), Khorasan and Basra, each starting a road; palace, main mosque and guard headquarters at the centre; rings of houses inside the walls. https://en.wikipedia.org/wiki/Round_city_of_Baghdad

**10. Sui Daxing / Tang Chang'an and Heian-kyō**
- Daxing: founded 582 by Emperor Wen of Sui, "southeast of the much ruined Han dynasty Chang'an". Han reasons for the region: Guanzhong "offered much greater natural protection". Plan: Palace City, Imperial City (offices), outer city of about 110 walled wards; walled East and West Markets of nine blocks each; Tang walls about 5 by 6 miles. https://en.wikipedia.org/wiki/Chang%27an . Planner Yuwen Kai and the six-ridges siting: not found.
- Heian-kyō (Kyoto): 794, Emperor Kanmu, on Wake no Kiyomaro's advice after leaving Nagaoka-kyō; grid modelled on Chang'an, under one-third its area, about 4.5 x 5.2 km; palace "placed at the northern center of the city rather than at its geographical center"; Suzaku-ōji avenue about 85 m wide splits Left and Right capitals; no continuous wall. https://en.wikipedia.org/wiki/Heian-ky%C5%8D . The four-gods siting rule is a later reading (see table).

**11. Freiburg im Breisgau and Bern (Zähringer towns)**
- Freiburg: founded 1120 by Konrad and Berthold III of Zähringen "as a free market town", at "a junction of trade routes". https://en.wikipedia.org/wiki/Freiburg_im_Breisgau . Settlement began about 1100; market right about 1120; then market street, parish church and town wall were built. The 1218 confirmation of the town law says each plot shall be 100 feet long and 50 wide ("Singule autem aree in longitudine centum, in latitudine quinquaginta pedes habebunt"), about 16.2 x 32.4 m; archaeology shows this was a real plot size, used pragmatically, and abandoned in new areas around 1200. Löbbecke, *Schau-ins-Land* 126, https://regionalia.blb-karlsruhe.de/files/17820/BLB_Loebbecke_Parzellenstruktur_Freiburg.pdf . The same article reports that the "Zähringer cross" (main street cross with parallel streets) was a 20th-century model by Ernst Hamm, which the author questions.
- Bern: founded 1191 by Berchtold V on "a hilly peninsula bounded by the Aare on three sides", with Castle Nydegg at the eastern tip; "three longitudinal streets"; westward growth in three walled stages marked by the Zytglogge, Käfigturm (1255-60) and Christoffelturm (1344-46) gates. https://en.wikipedia.org/wiki/Old_City_of_Bern

**12. Monpazier and Aigues-Mortes (French royal new towns)**
- Monpazier: 1285, Edward I as Duke of Gascony, by paréage: "the lord of Biron supplied the land, Edward the authority and permission"; grid with a central square and market hall; church at a corner of the marketplace. https://en.wikipedia.org/wiki/Monpazier . Site (hill above the Dropt): not found.
- Aigues-Mortes: Louis IX obtained the land in 1240 by exchange with the Psalmody abbey; reason: "Louis IX wanted direct access to the Mediterranean Sea", since other ports belonged to his brother or Aragon; prior: salt marshes, fishing and salt communities, Charlemagne's Matafère tower (791); Tour de Constance built 1242-1254 on the old tower site; walls ordered 1272, ramparts about 1,650 m; channels silted repeatedly. https://en.wikipedia.org/wiki/Aigues-Mortes . Grid plan: not on the fetched page.

**13. Spanish colonial towns under the 1573 Ordinances**
- Era: 1573 onward. Founder: Crown via licensed settlers.
- Stated reasons: see the table, ordinances 34-41, 111, 123.
- Prior settlement: ordinance 110 requires a site "that is vacant and that can be occupied without doing harm to the Indians and natives or with their free consent".
- Plan: no. 110: plan by "cord and ruler, beginning with the main square from which streets are to run to the gates and principal roads", leaving room to "spread in the same manner". No. 112: plaza at the landing place on the coast, at the centre inland; length at least 1.5 x width. No. 113: plaza sized to population, 200 x 300 ft minimum, 530 x 800 max, 400 x 600 "a good proportion". No. 114: four main streets from the middle of each side and two from each corner; corners face the four winds. No. 116: wide streets in cold places, narrow in hot. No. 118-124: smaller plazas for parish churches; on the coast the main church visible from the sea and usable for defence; inland the temple stands apart, raised, approached by steps. No. 122: slaughterhouses, tanneries downwind or downstream. No. 126-128: no private lots on the plaza; lots by lottery; palisade or ditch around the plaza. No. 129-130: commons, then pasture, then farmland by lottery. https://www.huduser.gov/portal/sites/default/files/pdf/The-Laws-of-the-Indies.pdf

**14. New Amsterdam and Batavia (Dutch companies)**
- New Amsterdam: WIC instructions 1625 (see table): settlement facing the river, ditch on three sides, five-sided bastioned fort with 110 houses inside and a church-school-infirmary building; ten farmhouses outside; family lots about 600 x 2,400 ft. What was built: settlers went to Manhattan; the plan "did not fit Manhattan's geography"; a smaller four-sided fort with the town outside it. https://www.gothamcenter.org/blog/notes-on-the-planned-town-of-fort-amsterdam . Bedrock was "too close to ground level for a deep ditch". https://www.atlasofmutualheritage.nl/page/7101/amsterdam-fort-new-york
- Batavia: Dutch trading post on the Ciliwung opposite Jayakarta from 1610; Coen razed Jayakarta in May 1619; castle enlarged; first canals dug 1619 east of the river; after the 1628-29 siege a moat and city wall "based on the work of Simon Stevin" enclosed the city. https://en.wikipedia.org/wiki/Batavia,_Dutch_East_Indies

**15. Later capitals: St Petersburg, Washington, Fatehpur Sikri, Isfahan, and Savannah**
- St Petersburg: 1703, Peter the Great, wanting a seaport better than Arkhangelsk, "closed to shipping during the winter"; captured the Swedish fortress Nyenskans (town of Nyen) and founded the Peter and Paul Fortress on Hare Island; site "built on swamp and water"; Trezzini's 1716 canal grid for Vasilyevsky Island, partly built; from 1737 the centre at the Admiralty where three radial streets meet. https://en.wikipedia.org/wiki/Saint_Petersburg
- Washington: Residence Act 1790 placed the capital on the Potomac; Georgetown and Alexandria already inside the district; L'Enfant's 1791 grid with diagonal avenues; Capitol on Jenkins Hill, "a pedestal awaiting a monument"; President's House on a ridge; revised by Ellicott 1792. https://en.wikipedia.org/wiki/L%27Enfant_Plan
- Fatehpur Sikri: Akbar, after Sheikh Salim Chishti foretold his son's birth (1569); Abul Fazl: "his holy heart desired to give outward splendour to this spot which possessed spiritual grandeur"; prior village of Sikri with earlier housing and temples; on a rocky ridge about 40 m high; wall of about 6 km on three sides and a lake on the fourth; left 1585, abandoned by 1610, often blamed on "the failure of the water supply" (not certain). https://en.wikipedia.org/wiki/Fatehpur_Sikri
- Isfahan: Shah Abbas I moved the capital from Qazvin in 1598, farther from Ottoman and Uzbek threats, with more control of the Gulf trade, on the Zayanderud oasis river, and to centralise power; new Naqsh-e Jahan square (1598-1629), Chaharbagh avenue, and a 2 km vaulted bazaar that "links the old city with the new". https://en.wikipedia.org/wiki/Naqsh-e_Jahan_Square
- Savannah (British): 1733, Oglethorpe; ward = four residential tything blocks plus four civic trust blocks around a square; five-acre gardens and 45-acre farm plots outside; first six wards, eventually 24 "in general accordance with the original design". https://en.wikipedia.org/wiki/Oglethorpe_Plan . Site choice (Yamacraw Bluff): not on the fetched page.

#### What this means for a generator

- **Founding cause is a spec field, separate from terrain.** The sources split reasons into
  physical (water, harbour, defensible height, river crossing, trade route, farmland,
  health) and non-physical (frontier politics, religion, omen, ruler's prestige). A closed
  enum of causes fits: `Harbour`, `RiverCrossing`, `RouteJunction`, `DefensibleHeight`,
  `Frontier`, `Capital`, `Religious`, `Colony`, `MarketCharter`. Physical causes become
  site-scoring rules; non-physical ones are recorded for labels and lore only.
- **Site scoring can be a rule pack built from the table above.** Weighted criteria that the
  sources agree on: fresh water within reach, ground above marsh and flood, defensible on at
  least some sides, access by land and water, farmland nearby, avoid marsh. Culture-specific
  extras (Vitruvius' wind rules, the Ordinances' north wind and eastern river bank, the
  Arthashastra's river confluence) belong in separate rule packs.
- **Lay down in this order for a planned town:** site, then pre-existing features (older
  settlement, fort, road, ford), then the anchor (plaza, forum, fort or palace), then main
  streets from the anchor to gates and outside roads, then the wall or ditch, then lots,
  then commons and fields outside. This is the order the 1573 Ordinances and the 1625 WIC
  instructions state.
- **Anchor placement has explicit rules worth encoding:** on a coast, the centre goes at the
  landing place; inland, at the middle (Vitruvius, Ordinances 112). East Asian and Indian
  models put the palace north of centre. Temples go on the highest point (Vitruvius) or
  apart and raised (Ordinances 124). Filth trades go downstream or downwind (Ordinances
  122-123).
- **Pins and prior settlement:** most "new" cities sit on an older village, fort or road. The
  spec needs a way to declare an older core, and the planned grid must fit around it
  (Constantinople, Cologne, Chester, Batavia, Isfahan's bazaar linking old and new).
- **Growth breaks the plan:** a generator for an old planned city needs a later stage that
  adds irregular suburbs outside the grid (Timgad), staged wall rings with gate towers
  (Bern), and split plots (Freiburg). A plan that was never built as designed (New
  Amsterdam) is also realistic.

#### Sources

All fetched during this research.

- Vitruvius, *Ten Books on Architecture*, Morgan transl., Project Gutenberg: https://www.gutenberg.org/files/20239/20239-h/20239-h.htm
- Aristotle, *Politics* VII, Jowett transl.: http://classics.mit.edu/Aristotle/politics.7.seven.html
- Laws of the Indies 1573, Ministerio de la Vivienda 1973, Spanish and English: https://www.huduser.gov/portal/sites/default/files/pdf/The-Laws-of-the-Indies.pdf
- Kautilya, *Arthashastra* Book II, Shamasastry transl., Wikisource: https://en.wikisource.org/wiki/Arthashastra/Book_II
- Rites of Zhou / Kaogong ji: https://en.wikipedia.org/wiki/Rites_of_Zhou
- Manasara: https://en.wikipedia.org/wiki/Manasara ; https://www.wisdomlib.org/definition/sarvatobhadra ; https://www.wisdomlib.org/hinduism/book/manasara-english-translation/d/doc421054.html
- Kyushu University, "Pleasing the Four Gods": https://kyushu-u.elsevierpure.com/en/activities/pleasing-the-four-gods-shijin-s%C5%8D%C5%8D-site-selection-and-site-adaptat/
- Herodotus 4.144, Godley transl.: https://fgh.perseids.org/read/greekLit/tlg0016/tlg001/perseus-eng2/4.144.1-4.145.1
- Tacitus, *Annals* 12.63, Loeb: https://penelope.uchicago.edu/Thayer/E/Roman/Texts/Tacitus/Annals/12B*.html
- Vasiliev, *History of the Byzantine Empire*: https://www.ellopos.net/elpenor/vasilief/foundation-of-constantinople.asp
- Strabo, *Geography* Book 4, Loeb: https://penelope.uchicago.edu/Thayer/E/Roman/Texts/Strabo/4A*.html
- Plutarch, *Alexander* 26, Austin transl.: https://livius.org/sources/content/plutarch/plutarchs-alexander/the-foundation-of-alexandria
- Amarna Project, Boundary Stelae: https://www.amarnaproject.com/amarna-the-place/boundary-stelae/
- Löbbecke, F., "Und soll jegliche Hofstätte sein hundert Schuh lang und fünfzig breit", *Schau-ins-Land* 126: https://regionalia.blb-karlsruhe.de/files/17820/BLB_Loebbecke_Parzellenstruktur_Freiburg.pdf
- Howe, R., "Notes on the Planned Town of Fort Amsterdam", Gotham Center 2014: https://www.gothamcenter.org/blog/notes-on-the-planned-town-of-fort-amsterdam
- Atlas of Mutual Heritage, Fort Amsterdam: https://www.atlasofmutualheritage.nl/page/7101/amsterdam-fort-new-york
- Wikipedia articles (secondary): Constantinople, History of Syracuse Sicily, Ortygia, Boundary Stelae of Akhenaten, Dur-Sharrukin, Timgad, Deva Victrix, Colonia Claudia Ara Agrippinensium, Round city of Baghdad, History of Baghdad, Chang'an, Heian-kyō, Freiburg im Breisgau, Old City of Bern, Monpazier, Aigues-Mortes, Batavia Dutch East Indies, Saint Petersburg, L'Enfant Plan, Fatehpur Sikri, Naqsh-e Jahan Square, Oglethorpe Plan (URLs given inline above).

#### Not found

- Al-Tabari's and al-Ya'qubi's own words on Baghdad's site in a fetched translation. Only
  search snippets; the Leiden chapter by P. A. Webb (2025) would not download.
- Kaogong ji full passage with temple-left, altar-right, court-front, market-behind on a
  fetched page (bjaxiscloud and the Routledge translation pages did not load).
- Manasara's actual site and soil criteria (chapters 4-5 text).
- Thucydides 6.3 text on Syracuse (Perseus pages returned only navigation).
- Constantine's alternative sites (Troy, Serdica, Nicomedia).
- Sargon II's stated reasons for Dur-Sharrukin; Akhenaten stela translator name.
- Yuwen Kai and the six-ridges siting of Daxing.
- Portuguese colonial town rules (e.g. Salvador da Bahia 1549 instructions): not searched in
  depth, nothing fetched.
- Russian planned towns other than St Petersburg (e.g. Catherine II's town regularisation):
  not searched in depth, nothing fetched.
- UNESCO Timgad page and UCL Amarna page returned HTTP 403.

### 4D. Organic and event-driven growth

Date: 2026-10-20. Scope: how towns that were not planned in one go got started and grew, what
sudden events did to towns, and what the synthesis and quantitative literature says about the
weight of each cause. Every claim has a URL that was fetched. Where only a search-engine summary
was seen and the page itself was not read, the claim is marked "(unverified)".


#### Findings

1. **Water access is the strongest measurable "city seed".** Two independent quantitative
   studies agree. In Europe 800-1800, a site on a navigable river had about 6 percentage points
   higher chance of becoming a city (Bosker and Buringh, working-paper version). Worldwide over
   2000 years, cells closer to the coast or on bigger rivers urbanised earlier (Motamed, Florax
   and Masters 2014). Sources: [B&B], [MFM].
2. **Physical geography matters most early; neighbouring towns matter more later.** Bosker and
   Buringh: "first nature" geography (the physical site: rivers, sea, soil, terrain) dominated in
   the Middle Ages. "Second nature" geography (where the site sits relative to existing towns)
   became important later, roughly after 1600. Early on, a site too close to an existing town lost
   out. Later, a site 20-100 km from an existing city did slightly better (about 1 percentage
   point). Source: [B&B].
3. **Roads are weaker seeds than rivers.** In Bosker and Buringh, a site on an old Roman road
   had no advantage, and non-hub Roman-road sites were even worse off. A crossing of two Roman
   roads (a "hub") helped before 1600 only. Source: [B&B].
4. **Religion and politics are powerful but rare causes.** In the same data, being an
   archbishopric or a capital "substantially increases" urban chances. But only 1% of candidate
   sites were archbishoprics and 0.2% were capitals before becoming cities. They are, in the
   authors' words, "major exceptions". Source: [B&B].
5. **A market grant does not make a town.** In England and Wales, 57% of markets were created in
   the 13th century. Of places with a market or market grant by 1300, only 39% were still market
   towns around 1600. Of places that already had a market in 1200, 72% were. Early sites in the
   best trading niches survived. Source: [GAZ].
6. **The street-and-plot plan outlasts everything else.** Conzen split the town into three
   layers: the town plan (streets, plots, building footprints), the building fabric, and land use.
   His "burgage cycle" (burgage: a long, narrow medieval town plot) is the slow filling of the back
   of plots with buildings until they are cleared and rebuilt. His "fringe belt" is a ring of
   open, institutional land (parks, hospitals, utilities) formed at the town edge when growth
   pauses. Source: [WH].
7. **Even disaster rarely erases the old street plan.** London after 1666 kept its medieval
   streets because owners rebuilt on their own plots; only widening and brick rules changed.
   Lisbon after 1755 did get a new grid, under a strong central authority. Noto after 1693 was
   rebuilt on a new site 8 km away. Sources: [LOC], [POMB], [NOTO].
8. **Growth outside the gate follows the approach road.** Dublin's western suburb was long narrow
   plots on both sides of the road out of the western gate (late 12th-13th c.). Outside York's
   Micklegate Bar the approach was lined with burgage plots, and a horse and cattle market held
   there "partly explains the width of the road today". Sources: [DUB], [YORK].

#### Case studies

Each: name; era; cause; what came first; plan trace; source.

1. **St Gallen (Switzerland).** 8th c. onward. Cause: monastery. First: a hermit's cell by the
   river Steinach (c. 612, legend), then the abbey (c. 720). A settlement grew around the abbey
   by the 10th century; a wall round the abbey was raised c. 954 and finished by 975, after which
   the adjoining settlement grew into a town. The town then pulled away from the abbey (bought its
   liberty 1415). Plan trace: abbey precinct with its own square at the core, town wrapped around
   it on north and west. Sources: [STG-W], [STG-U].
2. **Bury St Edmunds (England).** c. 633 monastery; grid c. 1080. Cause: monastery plus shrine of
   the martyred king Edmund. First: the abbey. Then Abbot Baldwin laid out a grid town beside it.
   By Domesday (1086) 342 houses stood on land previously ploughed (one local source). Plan trace:
   a planned grid next to an abbey precinct; a pilgrim/monastic town can still be a planned one.
   Sources: [BURY-W], [BURY-A].
3. **Dorestad and Birka (emporia).** c. 680-850 (Dorestad); c. 750-980 (Birka). Cause: long-
   distance trade at a waterway junction. Emporium (also "wic"): an early-medieval trading port.
   First: the waterfront. Dorestad's wooden jetties stretched about 3 km along the Lek and Rhine;
   the settlement looks divided by an orthogonal system of c. 4 m lanes, with plots of standard
   width and 36-47 m length. Birka sat "at the convergence of several important waterways", run
   from a royal residence across the strait, and was overtaken by Sigtuna. Plan trace: a ribbon
   along the water, plots running back from the shore, a separate seat of power nearby. Both were
   abandoned: an emporium can die and move. Hodges called the early phase "beach markets" and the
   later phase king-founded wics (unverified: seen only in a search summary of [EMP]). Sources:
   [DOR], [BIRKA].
4. **Santiago de Compostela (Spain).** Early 9th c. onward. Cause: pilgrimage. First: a Roman
   tomb identified as St James's; the city was "born around the Holy Tomb". Destroyed late 10th
   c., rebuilt in the 11th. Pilgrim road (Camino) lined with hospitals and monasteries along the
   way. Plan trace: cathedral and its squares as the core; road-borne pilgrim services.
   Source: [SANT].
5. **Varanasi (India).** 1st millennium BCE onward. Cause: trade crossing first, pilgrimage later.
   First: settlement "where the old trade route through North India crossed the River Ganga";
   pilgrimage growth from at least the 6th c. CE. Plan trace: a 6.5 km riverfront of 84 ghats
   (stepped landings) with stone steps from the 14th c., big buildings above them, narrow alleys
   with temples behind. Source: [VAR]. Concentric pilgrimage circuits round the city (e.g.
   Panchakroshi, about 80-88 km) are reported only by non-scholarly pages (unverified).
6. **Kutná Hora (Bohemia).** End of 13th c. onward. Cause: silver mining boom. First: the mines.
   UNESCO: the ground plan "is determined by mining", and the town "retains an earlier pattern of
   communications predating the city's actual origin". The large Cistercian abbey at Sedlec lies
   1.5 km away. Plan trace: irregular plan shaped by the mines, older roads kept inside it.
   Source: [KH].
7. **Potosí (Bolivia).** 16th c. Cause: silver mining. First: a small pre-Hispanic hamlet at
   4,000 m and the silver mountain (Cerro). Became an "Imperial City" after 1572. Industry: 22
   reservoirs feeding 140 ore mills; about 5,000 mine workings around. Plan trace: the colonists'
   quarter and the forced labourers' quarter separated "by an artificial river"; a mint, many
   churches; the mountain and water works dominate the setting. Source: [POT].
8. **Ouro Preto (Brazil).** Early 18th c. Cause: gold rush. First: mining camps on steep slopes.
   Plan trace: "Along the original winding road and within the irregular layout following the
   contours of the landscape lie squares, public buildings ... and churches". A gold town has a
   long winding spine road on a hillside, not a grid. Source: [OP].
9. **Provins (France).** 11th-13th c. Cause: Champagne trade fairs. First: an existing town of
   the Counts of Champagne. Plan trace: upper town and lower town, merchant houses, about 150
   vaulted storage cellars, open trading spaces, and walls "built for the protection of the
   fairs". Source: [PROV].
10. **Timbuktu (Mali).** Founded 5th c. per UNESCO (other dates exist; not checked); peak
    15th-16th c. Cause: desert-edge market at a river. First: a site "at the gateway to the Sahara
    ... near to the river". Market for salt from the north and gold, cattle and grain from the
    south; three great mosques and large public squares. Plan trace: mosques and market squares
    as anchors. Source: [TIM].
11. **Lisbon Baixa (Portugal).** 1755 earthquake. Cause: rebuild after disaster. First: a
    medieval organic quarter. After: "the medieval streets gave place to an orthogonal city",
    streets fixed at 40 ft, façade detail graded by street importance, the royal palace replaced
    by a square open to the river. Plan trace: a grid patch inside an organic city. Source: [POMB].
12. **London (England).** 1666 fire. Cause: rebuild after disaster. About 13,000 houses and 87
    churches burned. Wren, Evelyn, Hooke and others proposed grids; none was built because owners
    had to keep their plots. Result: medieval street layout kept, streets widened, brick and stone
    required. Plan trace: none; the old plan survives. Source: [LOC].
13. **Noto (Sicily).** 1693 earthquake. Cause: rebuild after disaster. Rebuilt on a new site 8 km
    south of the old one, on a grid by G. B. Landolina using the slope for "scenographic effects".
    Other towns of the region show every option: rebuilt in place (Catania), new town beside the
    old (Ragusa), or simply repaired (Caltagirone). Sources: [NOTO], [VDN].
14. **Bruges and Ephesus (relocation by silting).** Bruges got sea access when a 1134 storm cut
    the Zwin inlet; silting from the late 13th c. made it unusable c. 1500; Damme and Sluis lay on
    the Zwin as harbour towns. Ephesus: shoreline shifts from sedimentation "led to several
    relocations of the city site and its harbours"; a new city founded in the 4th c. BCE. Plan
    trace: outports downstream; an abandoned harbour; the town core left inland. Sources: [ZWIN],
    [EPH], [BRU].
15. **Fatehpur Sikri (India).** 1571-1585. Cause: sudden capital move. Built 1571-73, rectangular
    grid, 6 km wall on three sides, nine gates, palace and great mosque at the centre; capital
    moved to Lahore in 1585. Plan trace: a complete planned city that stops abruptly. Source: [FS].

#### The balance of causes

- **Bosker and Buringh (Europe, 800-1800).** Working-paper version [B&B] uses 1,784 candidate
  sites (all cities plus all 456 bishoprics of the year 600); the 2017 published version uses
  over 250,000 random locations [B&B-17]. The numbers below come from the working paper; the
  published numbers may differ. Findings: river location about +6 percentage points; sea location
  loses significance once regional effects are controlled; Roman roads no advantage, hubs helpful
  only pre-1600; rugged terrain and (early on) poor farmland reduce chances; 20-100 km from an
  existing city about +1 point later on; being too close to a rival river site hurts. Urban
  population rose 30-fold (0.7 to 21 million) while total population rose 6-fold; urbanisation
  went from 3% to 15%. 40% of the cities of 1300 fell below 5,000 people in the 14th century,
  largely due to the Black Death. The paper also summarises the narrative literature (Pirenne,
  Bairoch, Hohenberg and Lees): towns were founded near abbeys, monasteries and shrines, or at
  river crossings, the foot of a mountain pass, or a hill overlooking the land, but trade and
  transport "are often viewed to overshadow these religious and defensive motives".
- **Motamed, Florax and Masters 2014 (whole world, 2000 years, 62,290 half-degree cells).**
  Earlier urbanisation goes with better farmland, seasonal frost, nearness to sea or navigable
  river, and lower elevation. At the 10% urban threshold: +0.01 farmland suitability = about 0.3
  years earlier; +100 km from the coast = 1.3 years later; +100 m elevation = about 0.2 years
  later; one step bigger river (Strahler order) = about 8 years earlier; seasonal frost = about 18
  years earlier. The effects are similar within each continent and are strongest in the early
  stages of city formation. Source: [MFM].
- **Bosker, Buringh and van Zanden 2013 (Europe vs Middle East and North Africa, 800-1800).**
  The Islamic world's urban stagnation and Europe's rise have region-specific causes: transport
  mode (camel vs ship) and European local self-government that made cities less dependent on the
  state. Source: [BBZ].
- **Christaller (central place theory, 1933).** On a flat, uniform plain, towns space themselves
  in a triangular lattice with hexagonal market areas and a size hierarchy. "K" is the nesting
  rule: K=3 market, K=4 transport (small towns on the roads between big ones), K=7 administration.
  Source: [CPT]. **Hohenberg and Lees** add a second system: cities as "junctions in a
  transnational network" of trade, not only central places of a region. Source: [HL].
- **Mumford, Kostof, Bairoch.** Not read in this session. Kostof's *The City Shaped* has five
  chapters: "Organic" patterns, the grid, the city as diagram, the grand manner, the skyline
  (unverified: table of contents seen only in a search summary). No verified quotation from
  Mumford or Bairoch.

Plain summary: where a town appears is driven first by water (river, then sea), then farmland and
easy terrain, then distance to other towns. Religion, capitals and mines are strong but rare
triggers. Roads matter less than water for location, but they shape how the town grows.

#### What this means for a generator

- **Spec needs a "seed" cause, and the generator lays it first.** Allowed seed kinds, from the
  cases: river crossing, harbour or beach market, shrine or monastery, castle/seat of power, mine,
  fair, desert-edge or route market, planned foundation (capital or rebuild). The seed's footprint
  (precinct, waterfront, mine, market square) is placed before streets.
- **Order of laying down: site, water, seed, approach roads, first plan unit, walls, growth
  units.** Approach roads predate the town in Kutná Hora and are kept inside it, so roads come
  before walls. Each later growth phase is its own plan unit (Conzen) with its own plot pattern;
  the spec should list phases in order, and a later phase never rewrites an earlier one unless it
  is an explicit "rebuild" event.
- **Growth outside the walls follows approach roads.** Ribbon suburbs of long narrow plots on
  both sides of the road beyond a gate (Dublin), and a widened road or market just outside the
  gate (York). A fringe belt of open/institutional land can sit where the wall stopped growth.
- **Event types the spec should express as data:** pilgrimage (axis from gate to shrine, hostels
  along it), mining boom (irregular hillside fabric following contours, worker quarter split from
  owners' quarter, water works), fair (large open trading spaces, warehouses/cellars), disaster
  rebuild (three options: rebuild in place keeping streets as London; grid patch inside the old
  core as Lisbon; new site as Noto), silting (outport downstream, dead harbour), capital move
  (whole planned grid, possibly abandoned).
- **Water outranks roads when placing a seed.** If the generator ever chooses a site, river
  access should be weighted above road hubs, per both quantitative studies.
- **Plot pattern carries age.** Back-of-plot infill (burgage cycle) is a cheap, deterministic way
  to show that a district is old.

#### Sources

- [B&B] Bosker and Buringh, "City seeds" working paper (AEA 2012 conference version), fetched as
  PDF: http://www.aeaweb.org/conference/2012/retrieve.php?pdfid=203
- [B&B-17] Published version abstract, J. Urban Economics 98 (2017):
  https://ideas.repec.org/a/eee/juecon/v98y2017icp139-157.html
- [MFM] Motamed, Florax, Masters, Tinbergen discussion paper 14-002 (full PDF):
  https://papers.tinbergen.nl/14002.pdf ; abstract:
  https://ideas.repec.org/a/kap/jecgro/v19y2014i3p339-368.html
- [BBZ] Bosker, Buringh, van Zanden 2013 abstract:
  https://ideas.repec.org/a/tpr/restat/v95y2013i4p1418-1437.html
- [GAZ] Gazetteer of Markets and Fairs in England and Wales to 1516, introduction:
  https://archives.history.ac.uk/cmh/gaz/fullintro.html
- [WH] Whitehand 2001, "British urban morphology: the Conzenian tradition", Urban Morphology 5(2):
  https://journal.urbanform.org/index.php/jum/article/download/3896/3218/13149
- [CPT] Central place theory: https://en.wikipedia.org/wiki/Central_place_theory
- [HL] Hohenberg and Lees, book description:
  https://live-sas-www-history.pantheon.sas.upenn.edu/node/13049
- [STG-W] https://en.wikipedia.org/wiki/St._Gallen ; [STG-U] https://whc.unesco.org/en/list/268/
- [BURY-W] https://en.wikipedia.org/wiki/Bury_St_Edmunds ;
  [BURY-A] https://www.abbeyofstedmund.org.uk/?p=953
- [DOR] Review of Dorestad excavations, The Medieval Review:
  https://scholarworks.iu.edu/journals/index.php/tmr/article/view/17372
- [BIRKA] https://whc.unesco.org/en/list/555/
- [EMP] https://en.wikipedia.org/wiki/Emporium_(early_medieval)
- [SANT] https://whc.unesco.org/en/list/347/
- [VAR] https://whc.unesco.org/en/tentativelists/6526/
- [KH] https://whc.unesco.org/en/list/732/
- [POT] https://whc.unesco.org/en/list/420/
- [OP] https://whc.unesco.org/en/list/124/
- [PROV] https://whc.unesco.org/en/list/873/
- [TIM] https://whc.unesco.org/en/list/119/
- [POMB] https://en.wikipedia.org/wiki/Pombaline_style
- [LOC] Library of Congress, "Phoenix rising":
  https://blogs.loc.gov/maps/2025/12/phoenix-rising-urban-planning-after-the-great-fire-of-london/
- [NOTO] https://en.wikipedia.org/wiki/Noto,_Sicily ; [VDN] https://whc.unesco.org/en/list/1024/
- [ZWIN] https://en.wikipedia.org/wiki/Zwin ; [BRU] https://whc.unesco.org/en/list/996/
- [EPH] https://whc.unesco.org/en/list/1018/
- [FS] https://whc.unesco.org/en/list/255/
- [DUB] https://www.medievalists.net/2010/01/17/the-western-suburb-of-medieval-dublin-its-first-century/
- [YORK] City of York, Character Area 23 appraisal (PDF):
  https://www.york.gov.uk/downloads/file/1080/area-23-blossom-street-and-nunnery-lane

Note: UNESCO pages returned 403 to the fetch tool and were downloaded with curl instead.

#### Not found

- Full text of Kostof, *The City Shaped*; Mumford, *The City in History*; Bairoch, *Cities and
  Economic Development*. No verified quotations or numbers from them.
- Hamwic's street grid and 42 ha size: seen only in search summaries (unverified).
- Hodges's Type A/B/C emporia typology: seen only in a search summary (unverified).
- Mecca, Canterbury, Freiberg, gold-rush towns of North America or Australia, refugee towns:
  not researched in this session.
- Islamic-city organic fabric rules (Hakim, *Arabic-Islamic Cities*): book exists; its content
  on dead-end lanes and building rules was not read.
- A precise Conzen definition of "plan unit" from a peer-reviewed source: not found; the
  description in Finding 6 uses Whitehand's three-layer definition only.
- Effect sizes from the 2017 published Bosker and Buringh paper: not found (paywalled).
