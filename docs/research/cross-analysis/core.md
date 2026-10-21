# Cross-analysis: town core, founding and growth

> Written 2026-10-20 by Claude for the research-facts work (ADR-025). This document turns
> research into decisions, so it follows the CLAUDE.md rule "Explicit over implied" in full
> (ADR-023): uncertainty is quoted from a source, never added in my own voice. Nothing here is
> decided until Jeffrey answers §8 and a DECISIONS.md entry records it. The figures are in
> `data/research/facts/core.yaml`; every fact id below starts with `core.`.

Terms used once and explained here:

- **Core**: the spec's town core, the point the generator derives in S0 from roads and rivers
  (spec-v1 §4, "Town core"). Compass regions are measured from it.
- **Seed**: the first thing a town grows around (a ford, a shrine, a fort, a market).
- **Spine**: the one street that joins the main gate or crossing to the market
  (town-plan-time-series.md, terms).
- **Percentage point**: the difference between two percentages; a chance rising from 0.05% to
  1.25% rises by 1.2 percentage points.
- **Odds ratio**: how many times higher the odds of an outcome are in one group than in another.
- **Strahler order**: a river size rank; two first-order streams join into a second-order
  stream, and so on.
- **Plan unit**: a part of a town laid out in one go, with its own street and plot pattern.
- **Faubourg**: a suburb that grew along the road out of a town gate.
- **Glacis**: open ground kept clear outside a fortification.
- **Jōkamachi**: a Japanese castle town.
- **Emporium**: an early-medieval trading port on a shore (Dorestad, Birka, Hedeby).
- **Persistence**: a place, line or building that stays where it was across centuries.

## 1. Scope

Topic: where towns were founded (relative to rivers, crossings, roads, coasts and hills),
where the core, market, main temple and citadel sit relative to the crossing and the main
road, distances between them, how fast and in which steps towns grew, and what stays in place
across centuries.

Reports read: `city-foundation-and-growth.md` (cited as CF), `town-plan-time-series.md` (TS,
from PR #28), `town-plan-corpus.md` (C1), `town-plan-corpus-world.md` (CW), and under
`settlement/`: `growth-generators.md`, `district-types.md`, `water-infrastructure.md`,
`peninsula-ports.md` (PP), `timber-ports.md` (TP), `cliff-towns.md` (CT), `social-structure.md`
(SS), plus `verification-population.md` (VP) for the checked figures. Under
`sources-by-language/`: the README and all seven reports (africa, chinese, continental,
iberian, islamic, japanese, south-asia).

Result: 151 facts, 47 sources, 1 unit (the international mile) and 8 quantities in this
topic's files. The validator's total of 48 sources adds the shared `bipm-si-2019` in
`sources.yaml`, which no core fact cites. Status
counts: 20 verified, 1 corrected, 4 superseded, 2 disputed, 124 unverified. TS's summary
sections §14 and §15 overstated its city blocks (independent review of PR #28); the corrected
report is on main since 2026-10-21. No §14 or §15 figure is a fact; TS facts come from the
city blocks only. Placement findings without a figure are cited here by report and section and
are not facts.

### Repair round 2026-10-21

A sample check found errors in this topic, and fact format v2 (ADR-025, second version) added
five fields. Every fact was re-read against the passage its `report:` names; TS was read in
its corrected version on main. Changes:

- **Removed, 2 facts.** `core.rome.papal-centre-move` (2.5 km) and
  `core.constantinople.palace-move` (2 km north-east) came from the old TS §14; the corrected
  report gives no distance for either move. The moves themselves stay in §4.1 below.
- **`approximate: true`, 26 facts**: every figure the report gives with "about", "~",
  "slightly more than" or a similar word.
- **`year_basis`, 90 facts**: every fact that keeps years. The values mean: `measured`, the
  period of a study's data or the date of a survey; `plan_date`, the date a historic plan
  was drawn or surveyed, which for these plans is also the state it shows; `depicted` (6
  facts), the state a plan shows when it was drawn later (Lanciani's c. 350 Rome, Legrand's
  1380 Paris, the 1524 Nuremberg map of Tenochtitlan in 1519-1520, Suttinger's Vienna "as in
  1683" dated 1684; a change between two plans takes `depicted` when one of them is such a
  plan); `valid_until`, a figure valid until the place was abandoned, with only `year_to` (no facts
  in this topic); `census`, the year of a census or tax survey; `built`, the year the measured object
  was built or opened; `founded`, the year a town or market was founded; `described`, the date
  of the state the report describes when none of the other values applies; `estimated`, an
  archaeological date.
- **Years removed, 10 facts**, because the report gives no date for that figure (the year
  belonged to another figure in the same row): `core.wessex.burh-max-distance`,
  `core.yoshida-juku.length`, `core.narai-juku.length`, `core.constantinople.wall-shift`,
  `core.paris.royal-residence-spread`, `core.fatehpur-sikri.ridge-height`,
  `core.salvador.upper-town-height`, `core.luxembourg.cliff-height`,
  `core.london.churches-rebuilt-on-site`, `core.tenochtitlan.area-1519-smith`.
- **Years changed, 19 facts**, to the date the report gives for the figure: centuries are
  their full range (13th century: 1201 to 1300), walls take their building date (Aurelian
  Wall 271-275, Theodosian Wall 413, Linienwall 1704, Kamer-Kollezhsky rampart 1742), moves
  take the opening of the new site (Dolmabahçe 1856, Richmond Bridge 1816, Zocher's Beurs
  1845), and the Vienna glacis takes the 1683 plan.
- **`year_approximate: true`, 12 facts**: "c. 1325", "c. 1550", "c. 1560", "c. 1688",
  "c. 1840s", "around 1600", "around 15 BC", "c. AD 800", "early 13th century", "early 19th
  century" and the c. 350 plan.
- **`disputes`, 3 pairs (5 facts)**: Tenochtitlan time series against Valero, Valero against
  Smith, and Timgad 90 ha against 60 ha (§5 rows 8 and 9).
- **`source_via`, 2 facts**: both Gedi facts name Pradines 2010 as the source and the BCAI 26
  review as the work the report read.
- **Source changed, 3 facts**: `core.tenochtitlan.area-1519-time-series` (Wikipedia,
  Tenochtitlan), `core.mexico.traza-area` (Wikipedia, History of Mexico City) and
  `core.bergen.shoreline-shift` (Norwegian Wikipedia, Bryggen), each the work the report
  cites; 4 sources added (these three and the BCAI review).
- **Status changed, 4 facts**, all from disputed to unverified: `core.salvador.upper-town-height`,
  `core.edo.area-1748`, `core.timgad.peak-area-90`, `core.timgad.peak-area-60` (§5 rows 8, 10,
  11).
- **Values changed, 2 facts**: `core.cairo.river-distance` is 1.5-2 km, the city block's
  figure, not the 2 km of the §9.5 table; `core.salvador.upper-town-height` is 60-65 m, the
  report's own table.
- **Notes corrected**: Chiang Mai, Mandalay, Timbuktu and Isfahan now name their river and the
  point the city block measures from (Isfahan measures from the Naqsh-e Jahan maidan, not from
  the old town); the Dublin Custom House note names both sites; the Amsterdam exchange note no
  longer calls the exchange the only landmark that moved.

Out of scope: wall and gate figures, street widths, plot sizes, densities and landmark
footprints, which belong to other topics. Figures from those topics appear here only where
they measure a distance from the core or a growth step.

### Verification batch 1 (2026-10-21)

Issue #74, batch 1, checked the unverified facts that questions Q1 to Q6 cite against their
original sources. Five facts have an outside source: `core.dubrovnik.spine-length`,
`core.baghdad.centre-to-wall`, `core.yoshida-juku.length`, `core.narai-juku.length` and
`core.ishibashi-juku.length`. All five are verified (0 corrected, 0 disputed, 0 not found); no
value changed, and no recommendation of Q1 to Q6 changes. Six more cited facts take their
source from a CityMapMaker report (`core.timber-ports.*`, `core.corpus-world.*`,
`core.cliff-towns.*`, `core.peninsula-ports.*`, `core.timgad.founding-square-side`); they have
no outside source to open and stay `unverified` for a later batch.

## 2. Founding sites

| Pattern | Cultures and regions | Report and section | Fact ids |
|---|---|---|---|
| Water access is the strongest measured cause: a navigable river first, the sea second | Europe 800-1800; worldwide over 2,000 years; Britain and France | CF §1 item 1, §2, §4A, §4D; VP rows 5b, 6, 7 | `core.europe.bb2017-river-effect`, `core.europe.bb2017-sea-effect`, `core.world.mfm-river-order-lead`, `core.world.mfm-coast-distance-lead`, `core.britain.mr-water-access-roman`, `core.britain.mr-water-access-medieval` |
| A river crossing (ford, then bridge) is the seed | England (Oxford, Wallingford), Germany (Frankfurt), Netherlands (Maastricht), Austria (Innsbruck), India (Varanasi), Ireland (Dublin) | CF §4A cases 1-5, §4B case 1, §4D case 5; TP §11; TS city blocks §2, §4, §5, §8 | `core.dublin.bridge-replacement-shift` |
| A road junction is the seed | Alnwick, Oxford's Carfax, Freiburg, Timgad (six roads), Manching, Aleppo, Damascus, Agadez | CF §4A cases 4, 10, 12, §4B case 13, §4C cases 7, 11 | `core.europe.bb2017-road-hub-effect` (corrected; the authors call its size unreliable), `core.europe.bb2017-roman-road-effect` |
| A road alone gives a linear town along it | Roman Britain (Watling Street), Japan (Tōkaidō, Nakasendō), German street villages, Bavarian street markets | CF §4A findings and cases 6, 13; japanese.md finding 8; continental.md case 9 | `core.watling-street.mansio-interval`, `core.yoshida-juku.length`, `core.narai-juku.length`, `core.ishibashi-juku.length`, `core.tokaido.post-station-count`, `core.germany.dalgaard-market-town-base`, `core.germany.dalgaard-market-town-road-effect` |
| A break of transport (portage, fall line, desert edge) is the seed | Eastern USA, Timbuktu, Hedeby's portage | CF §4A case 9, §4D case 10; TP §11 | `core.us.bl-portage-density-gain`, `core.us.bl-portage-radius`, `core.timbuktu.river-port-distance` |
| A shore landing or beach market is the seed; the town is a ribbon along the water | Dorestad, Birka, Hedeby, Bergen, Dublin, Massalia, Syracuse | CF §4C cases 1-2, §4D case 3; TP layout rules; C1 §2.4 | `core.timber-ports.spine-parallel-shore-towns`, `core.corpus-world.market-water-edge`, `core.bergen.hills-distance`, `core.bergen.shoreline-shift` |
| A defensible height comes first; routes bend to it; later the town drifts down to the plain | Ludlow, Celtic oppida, Indian hill forts, Japanese mountain castles, Monemvasia, Bonifacio, Salvador | CF §4B patterns, cases 2, 13, 14; CT findings 1-4; iberian.md finding 5 | `core.chittorgarh.plateau-height`, `core.fatehpur-sikri.ridge-height`, `core.monemvasia.upper-town-height`, `core.bonifacio.upper-town-height`, `core.salvador.upper-town-height`, `core.bibracte.successor-distance`, `core.cliff-towns.citadel-on-top-towns` |
| A route or crossing comes first, then a fort to control it, then the town | Anglo-Saxon burhs, Norman England and Wales, Rhine tolls, Crusader route castles, Siberian ostrogs | CF §4B patterns and cases 1, 3-7, 11 | `core.wessex.burh-max-distance` |
| An existing settlement comes first; the new town or fort sits on or beside it | Constantinople, Cologne, Batavia, St Petersburg, Spanish presidios; Roman sites in France | CF §1 item 5, §4C finding 3, §4B case 12; VP row 7 | `core.san-antonio.presidio-to-mission`, `core.britain.mr-persistence-odds`, `core.france.mr-persistence-odds`, `core.britain.mr-top20-near-roman`, `core.france.mr-top20-near-roman`, `core.britain.mr-top20-near-roman-1700`, `core.europe.mr-near-threshold` |
| A religious seed (monastery, shrine, pilgrimage) | St Gallen, Bury St Edmunds, Santiago, Varanasi, Gedi (mosque) | CF §1 item 8, §4D cases 1-5; africa.md case 11 | `core.europe.bb2012-archbishopric-share`, `core.europe.bb2012-capital-share`, `core.gedi.growth-13th-c` |
| A mine, a fair or a ruler's decision | Kutná Hora, Potosí, Ouro Preto, Provins, Fatehpur Sikri, Amarna | CF §4C case 15, §4D cases 6-9, 15 | `core.kutna-hora.abbey-distance` |
| A market grant alone does not make a town | England and Wales | CF §1 item 8, §4D finding 5 | `core.england.markets-created-13th-c`, `core.england.market-1300-still-1600`, `core.england.market-1200-still-1600` |
| Planned foundations follow written site rules: healthy air, fresh water, ground above marsh, defensible, reachable by land and water | Roman (Vitruvius), Greek (Aristotle), Indian (Arthashastra), Spanish (1573 Ordinances), Chinese (Kaogong ji, Guanzi) | CF §4C findings 1-2 and table; iberian.md finding 1; chinese.md finding 4 | none; the rules carry no figures for site choice |
| The town stands back from its river and reaches it by canal or a port town | Cairo, Chiang Mai, Mandalay, Timbuktu, Isfahan | CW §9.5 | `core.cairo.river-distance`, `core.chiang-mai.river-distance`, `core.mandalay.river-distance`, `core.isfahan.river-distance`, `core.corpus-world.river-edge-not-touching` |
| Quarters keep the outline of a founding camp | Agadez | CF §4A case 10; africa.md finding 4 | none |

What this says in one line: water access is the strongest measured cause of where towns
start (row 1); a crossing of a road and a river is the strongest single point (CF §2, first
bullet); a road without water gives a linear town; a defensible height is a third, separate
founding order; existing settlements, religious seeds, mines, fairs, rulers' decisions and
planned foundations are further orders, each without a measured effect size.

## 3. Core, market, temple and citadel relative to the crossing and the main road

### 3.1 The core

- CF §4A "What this means": "Most cases begin at a river crossing ..., a road junction ..., or a
  break of transport", and the report recommends placing the core at the intersection of river
  and roads. CF §2 adds that "most crossings that seeded towns are *river* crossings", so road
  and river act together.
- Planned towns state a different rule. Vitruvius (CF §4C table, Book I ch. 7) and the 1573
  Ordinances no. 112 put the forum or plaza "close to the harbour" or at the landing place on a
  coast, and "in the middle of the town" inland. Indian and East Asian models put the palace
  north of the centre (Arthashastra 2.4.7 in south-asia.md; Kaogong ji; Heian-kyō in CF §4C
  case 10). Baghdad put palace and mosque at the exact centre (`core.baghdad.centre-to-wall`).
- Defensible-site towns put the seed at the castle gate (Ludlow, CF §4B case 2) or the castle
  itself (jōkamachi, CF §4B findings), and bend the main road to pass the castle front.
- The Chinese tradition states that terrain overrides the ideal plan (Guanzi, chinese.md
  finding 4).

### 3.2 The market

| Market position | Towns | Report and section | Fact ids |
|---|---|---|---|
| At the crossing itself | Oxford (street market "occupied the roads meeting at Carfax") | CF §4A case 1 | none |
| A triangle or wedge in the fork of roads, or at one end of the main street | Alnwick, Herefordshire planted towns, Breslau triangles | CF §4A case 4 and findings; continental.md finding 7 | none |
| Along the main street (street market) | Dublin High Street, Edinburgh, Bavarian Straßenmarkt towns, Watling Street ribbons | C1 §2.1, §2.8; continental.md finding 7, case 9; CF §4A | none |
| At the castle gate, market below | Ludlow, Windsor, Durham (outside the plateau wall at the neck) | CF §4B findings and case 2; C1 §2.6 | none |
| At the head of the harbour or on the water edge | Bergen, Amsterdam (Dam), New Orleans, Savannah, Quebec lower town, Havana, Brunei; jetty platforms at Hedeby and Wolin | C1 §2.4; CW §9.4; TP layout rules | `core.corpus-world.market-water-edge` |
| Outside the wall on purpose | Baghdad (moved to Karkh), Mandalay, Manila, Chiang Mai, Hue, Beijing, Mexico City (Tlatelolco) | CW §9.4; SS §10 | `core.corpus-world.market-outside-wall` |
| Around the central temple | Madurai, Srirangam, Tunis, Harar, Lhasa, Galway, Aleppo; Fez souk streets from the gates to the main mosque | CW §9.4; district-types.md finding 3 | `core.corpus-world.temple-central-market-round` |
| A planned central square from which streets run to the gates | Spanish colonial plazas, bastides (Monpazier, Villefranche), Timgad forum at the cardo-decumanus crossing | iberian.md finding 1; continental.md case 5; CF §4C case 7 | `core.timgad.founding-square-side` |
| Behind the palace by rule | Kaogong ji ideal | SS §10, rule 21; chinese.md | none |
| A market square at both ends of a bridge, made as a firebreak | Edo after 1657 | water-infrastructure.md rule 16; japanese.md finding 7 | none |
| A specialised market at its supply: cattle at a gate, fish at the quay | London (Smithfield, Billingsgate), York (Micklegate) | SS §9; CF §4D finding 8 | none |

TS defines the spine as the street from the main gate or crossing to the market; its city
blocks show it in the same place in Rome, Constantinople, Paris, Amsterdam and Mexico City
(§4.1). The
peninsula ports put markets and fora along one main street from the land gate to the far end
(`core.peninsula-ports.spine-gate-to-tip-towns`).

### 3.3 The main temple

- On the highest point: Vitruvius' guardian temples (CF §4C table); castle, main church and
  government on the top level in cliff towns (`core.cliff-towns.citadel-on-top-towns`).
- Near but not on the plaza, raised on steps, inland; visible from the sea on a coast: 1573
  Ordinances nos. 118-124 (iberian.md finding 1).
- At the centre: Arthashastra shrines "in the middle of the city" (south-asia.md); the Great
  Mosque at the heart with the souk around it (SS §10, a contested model).
- Beside the market: Monpazier, church at a corner of the market place (CF §4C case 12);
  Djenné, mosque, palace and a 400 by 150 m market esplanade as one civic core (africa.md
  finding 3); Harar, mosque, market and square as "three points of a triangle".
- At the edge: Japanese temple belts on the outer rim as a defence (CF §4B; japanese.md case
  6); Dublin's second cathedral outside the wall (C1 §2.1). CW counts 7 central and 7 edge or
  outside (`core.corpus-world.temple-central-market-round`, `core.corpus-world.temple-edge`).

### 3.4 The citadel and the ruler's seat

- CW §9.4 counts: ruler dead centre 10, ruler on the edge on high ground 14, at one end of an
  axis 6, outside the town 5 (`core.corpus-world.ruler-centre`,
  `core.corpus-world.ruler-edge-high`, `core.corpus-world.ruler-axis-end`,
  `core.corpus-world.ruler-outside`).
- Peninsula ports: at the high point or far tip, away from the land gate
  (`core.peninsula-ports.citadel-at-tip-towns`, rated strong); or on a wall corner facing the
  town (Saint-Malo, Aigues-Mortes, rated medium).
- Fort-to-town relation by culture (CF §3 item 7, §4B patterns): at the gate with the market
  below (Norman borough); at a corner or edge of the wall (Edwardian Wales; Norman urban castles
  "within the corners of urban defensive circuits", next to the river); at the centre with
  rings of rank (Japan); a plaza in front of the fort (Spanish colonies); craft quarters at the
  gates (oppida).
- Chinese county seats: the yamen on the highest ground or central (chinese.md, generator
  section, "soft constraints").

### 3.5 Distances that the reports give

A value in quotation marks is the source's own wording; its fact carries `approximate: true`.

| What | Value | Fact id |
|---|---|---|
| Centre (palace and mosque) to wall, Baghdad Round City | "about 1 km" | `core.baghdad.centre-to-wall` |
| Side of Timgad's founding square, forum at its centre crossing | "about 355 m" | `core.timgad.founding-square-side` |
| Dubrovnik's spine, gate to gate | "about 300 m" | `core.dubrovnik.spine-length` |
| Vienna's centre to the outer rampart (1704) | 2-3 km | `core.vienna.centre-to-linienwall` |
| Vienna's glacis outside the inner fortification | 400-500 m | `core.vienna.glacis-width` |
| Spanish presidio to the mission it protected | "about half a mile" | `core.san-antonio.presidio-to-mission` |
| Timgad's Byzantine fort to the old city (shrink state) | 250 m | `core.timgad.fort-to-city` |
| Bergen's shore to the hills that close the town | 200-500 m | `core.bergen.hills-distance` |
| Post-town length along the highway | 595 m to 2.6 km | `core.ishibashi-juku.length`, `core.narai-juku.length`, `core.yoshida-juku.length` |

No report gives a measured distance from the crossing to the market, from the core to the main
temple, or from the core to the citadel for a town of the generator's scale (500 m to 5 km).
See §6.

## 4. Persistence and growth

### 4.1 What stays in place

Source rule for this section: TS's summary sections §14 ("what stays fixed") and §15
overstated the city blocks. An independent review of PR #28 found errors in them; the
corrected report is on main since 2026-10-21. This section cites the city blocks (TS §2 to
§13, block B of each city) and records no §14 count as a fact. Each row names the exceptions the blocks themselves show.

| Layer | Stayed in place (city blocks) | Exceptions in the city blocks | Fact ids |
|---|---|---|---|
| Water lines (river, coast, canal) | Rome, Paris, London, Moscow, Ayutthaya (river ring and canals after 160 years of abandonment) | Shorelines moved: Mexico City's lakeshore 2-3 km, then 5 km east (§10); Dublin's river edge 50-150 m (§8); rivers buried under streets (Fleet, Neglinnaya, Bièvre) | `core.mexico.lakeshore-retreat-1793`, `core.dublin.river-edge-shift`, `core.bergen.shoreline-shift` |
| Wall line | Paris (Grands Boulevards on the Charles V line, §4), Moscow (Boulevard Ring on the Bely Gorod line, §9), London (wall line as streets, §5), Rome (Aurelian Wall to 1870, §2) | Vienna: the Ringstrasse lies on the glacis, not on the wall line (§7); Amsterdam: gates moved outward with each extension (§6); Timgad: the founding wall was dismantled and built over (§13) | `core.vienna.glacis-width`, `core.amsterdam.walled-area-1625`, `core.timgad.walled-area-100` |
| Primary river crossing | Rome (bridges on their ancient piers), Paris (Roman-axis bridges), London Bridge, Dublin (Old Bridge site rebuilt in place), Edo (Nihonbashi), Mexico City (causeways) | Dublin's Ormond Bridge replaced about 100 m away (§8); London Bridge replaced 30 m upstream after the last plan (§5); the §8 block shows Dublin bridges rebuilt on the same site twice, not five times as its closing line says | `core.dublin.bridge-replacement-shift`, `core.london.bridge-replacement-shift` |
| Spine street | Rome (Corso), Constantinople (Mese, Divanyolu), Paris (rue Saint-Martin, Saint-Denis, Saint-Jacques), Amsterdam (Damrak-Rokin), Mexico City (three causeways) | Constantinople's Divanyolu widened on its old line after the 1865 fire (§3) | none |
| Main temple or church | Pantheon, Hagia Sophia, Notre-Dame, St Paul's, Oude Kerk, Stephansdom, Christ Church, St Basil's, the cathedral on the Templo Mayor side, Sensō-ji, Ayutthaya's masonry wats | St Paul's rebuilt on its site rotated 8 degrees (§5); 35 of London's 87 burned parish churches not rebuilt (§5) | `core.london.churches-rebuilt-on-site` |
| Market square | Paris (Les Halles on the 1137 site, §4), London (Smithfield, Leadenhall, §5), Mexico City (Plaza Mayor, §10), Amsterdam (Dam, §6) | Ayutthaya: the market moved from the centre and south to the north-east corner (§12); Timgad: the civic centre moved from the forum to the fort (§13); Amsterdam: the exchange moved 300 m onto the Dam (§6) | `core.ayutthaya.occupied-share-1926`, `core.timgad.fort-to-city`, `core.amsterdam.exchange-move` |
| Citadel or castle | Castel Sant'Angelo, the Tower, the Hofburg, Dublin Castle, the Kremlin, Edo Castle | none in the blocks | none |
| Street network after a fire | London 1666 (rebuilt on its own plots), Moscow 1611 and 1812 | London widened streets and added the King Street-Queen Street pair (§5); Constantinople regularised streets after 1865 and 1870 (§3); Edo added firebreaks and wider streets after 1657 (§11); Mexico City got a new grid over the razed city in 1521 (§10) | `core.london.fire-area-1666` |

Not in place in the blocks: the ruler's residence moved in Rome (Lateran to Vatican, no
distance given), Constantinople (Great Palace to Topkapı, "a short way north-east", no
distance given; Topkapı to Dolmabahçe, 4 km) and Paris (within 1.5 km), and the Custom House
in Dublin moved 800 m (`core.paris.royal-residence-spread`,
`core.constantinople.court-move-1856`, `core.dublin.custom-house-move`); buildings and built
extent changed everywhere.

The quantitative studies agree on persistence at regional scale: Roman town sites persisted
with odds ratios of about 7-13 in Britain and 20-40 in France (`core.britain.mr-persistence-odds`,
`core.france.mr-persistence-odds`); portage towns kept a 50-110% density advantage after
portage ended (`core.us.bl-portage-density-gain`). Whole towns did move: Noto 8 km, Bibracte to
Autun 25 km, Yakutsk 70 km, Golconda to Hyderabad (CF §4B, §4D;
`core.noto.relocation`, `core.bibracte.successor-distance`, `core.yakutsk.relocation`).

### 4.2 Expansion steps

The reports name these steps; each is a separate plan unit added outside the earlier ones:

1. Ribbon suburbs along the approach roads beyond a gate, with a widened road or market just
   outside it: Dublin, York, Paris faubourgs, Chinese guanxiang at the gates on the main road or
   landing (CF §4D finding 8; TS §4; chinese.md case 3).
2. A suburb walled in as a new ring: 8 towns in CW (`core.corpus-world.suburb-walled-as-new-ring`);
   Bern in three stages (`core.bern.wall-stages`); Amsterdam's four extensions
   (`core.amsterdam.walled-area-1560`, `-1625`, `-1688`); Dublin's plan units of c. 1010-1030 and the
   stone wall of c. 1100 (TP §9).
3. A planned extension beside an organic core: 6 towns in CW
   (`core.corpus-world.planned-extension-beside-core`).
4. A demolished wall becomes a ring road on its line: Paris three times, Moscow twice, Amsterdam
   as canals, Madurai (TS city blocks §4, §6, §9; south-asia.md case 8). Vienna is the
   exception: its ring road lies on the glacis, not on the wall line (TS §7).
5. Filling and splitting inside a fixed wall: Rome tripled its population inside the same built
   area by building higher (TS §2); Constantinople filled its walls before growing outside
   (`core.constantinople.built-area-1535`); Freiburg split its plots from about 1200 (CF §4C
   case 11); Buenos Aires and the Andalusi towns filled their early open lots (iberian.md
   findings 3 and 6).
6. Drift down from a hill to the plain, or an upper town that dies first: Chittorgarh, Golconda,
   Bibracte, Monemvasia, Gozo, Salvador, Coimbra (CF §4B patterns; CT finding 4; iberian.md
   finding 5).
7. A shift of the whole town beside its first core: Gedi moved north-east to south-west and left
   its first great mosque outside the walls (`core.gedi.max-area`).

CF §4D ("What this means") states the rule the reports share: a later phase never rewrites an
earlier one unless an explicit rebuild event says so; the three rebuild options are London (same
streets), Lisbon (grid patch inside the old core) and Noto (new site).

### 4.3 Rates

The reports give areas at dates, not rates per year. Examples (all from the facts file):

| Town | Earlier | Later | Fact ids |
|---|---|---|---|
| Amsterdam, walled area | 150-180 ha (c. 1560) | 440-460 ha (1625), 700-730 ha (1688), then a 150-year plateau | `core.amsterdam.walled-area-1560`, `core.amsterdam.walled-area-1625`, `core.amsterdam.walled-area-1688` |
| Paris, built | 350-400 ha in a 440 ha wall (1380) | 500-600 (1550), 1,100-1,300 (1735), 4,000-5,000 ha (1871) | `core.paris.walled-area-1380`, `core.paris.built-area-1550`, `core.paris.built-area-1735`, `core.paris.built-area-1871` |
| London, built | 400-500 ha (1560) | 1,200-1,500 (1676), 3,500-4,500 ha (1799); walled City 133 ha throughout | `core.london.built-area-1560`, `core.london.built-area-1676`, `core.london.built-area-1799`, `core.london.walled-area` |
| Timgad | 12 ha (100 CE) | 60 or 90 ha (3rd century), factor 5-7.5 in 100-150 years | `core.timgad.walled-area-100`, `core.timgad.peak-area-90`, `core.timgad.peak-area-60`, `core.timgad.growth-factor` |
| Gedi | doubled in the early 13th century | 34 ha (14th century) | `core.gedi.growth-13th-c`, `core.gedi.max-area` |
| Djenné-Djèno | 12 ha (250 BC) | 25, then 33 ha (c. AD 800) | `core.djenne-djeno.area-early`, `core.djenne-djeno.area-late` |
| Aleppo, built | 238.5 ha (1516) | 349 ha (early 19th century); suburbs 4,481 to 8,743 housing units (1537-1683) | `core.aleppo.built-area-1516`, `core.aleppo.built-area-1800`, `core.aleppo.suburb-units-1537`, `core.aleppo.suburb-units-1683` |
| Tenochtitlan | about 180 ha (c. 1325) | 750 ha or more (1519) | `core.tenochtitlan.area-1325`, and the three 1519 facts in §5 row 9 |
| Edo, townspeople | 150,000 (1634) | 500,000 (1721) | `core.edo.townspeople-1634`, `core.edo.townspeople-1721` |
| Vienna, population | 200,000 (1777) | 726,105 (1880 census), factor 3.6 between the 1773 and 1878 plans | `core.vienna.population-growth` |
| Shrinks | Rome −70 to −78% of built area (350-1551); Mexico −75 to −85% (1520-1550); Timgad more than −90% (to 539); Ayutthaya 10-20% of the island occupied (1926) | | `core.rome.shrink-350-1551`, `core.mexico.shrink-1520-1550`, `core.timgad.shrink-539`, `core.ayutthaya.occupied-share-1926` |

In the shrink cases of Rome and Ayutthaya the water lines and masonry monuments stayed, and
Rome kept its spine (TS §2, §12); Ayutthaya lost its wall and its market moved. Rome's
"95 percent" in TS §1 refers to population; its built area fell by 70 to 78 percent
(`core.rome.shrink-350-1551`). Timgad's shrunken town re-anchored on a spring and a new
fort 250 m outside the old grid instead of on the old forum (`core.timgad.fort-to-city`).

## 5. Conflicts

| # | Competing claims | Fact ids | Reason | Status |
|---|---|---|---|---|
| 1 | Bosker and Buringh: base chance "about 12%", river "about 6 percentage points" (2012 working paper; both facts carry `approximate: true`) against 0.05% per century, river +1.2 points (2017 article) | `core.europe.bb2012-*` against `core.europe.bb2017-*` | Two different studies: 1,784 cities and bishoprics with a probit model against 259,776 random locations with a linear model (VP row 5) | 2012 superseded; the 2017 figures are current; the two are never mixed |
| 2 | CF §4A's table labels the 2012 working-paper numbers with the 2017 journal citation | note on `core.europe.bb2012-river-effect` | A labelling error in the report; the report itself flags "the final journal tables may differ" | resolved by VP; the report text stays unchanged (ADR-025) |
| 3 | A Roman road: "no advantage" (2012) against +0.4 points, positive and significant (2017) | `core.europe.bb2017-roman-road-effect` | Different studies, as row 1. CF §4D finding 3 and its generator advice quote the 2012 result | 2017 current; direction is unchanged: water outranks roads |
| 4 | Road hub effect: "about +7" (CF §2) against 6.9 (Table 1) and 7.3 (text); the authors call it an artefact, and logit or probit gives a hub effect more than 10 times smaller | `core.europe.bb2017-road-hub-effect` | Model artefact named by the authors | corrected to 6.9; its size is not usable, only that a hub is positive |
| 5 | Second-nature geography (nearness to other cities) mattered only after 1600 (2012) against first-nature geography dominating throughout 800-1800 (2017) | qualitative, VP note 5 | Different studies | 2017 current |
| 6 | TS §14 before the correction said the market square stayed in all 12 cities; the city blocks show it moved in Ayutthaya (§12) and Timgad (forum to fort, §13) | `core.ayutthaya.occupied-share-1926`, `core.timgad.fort-to-city` | the pre-correction §14 overstated the blocks (independent review of PR #28); the corrected TS on main (2026-10-21) states the same exceptions as the blocks | resolved in favour of the city blocks; no §14 figure is recorded as a fact |
| 7 | TS §14 before the correction said the wall line stayed in all 11 walled cities; the blocks show it moved in Vienna (Ringstrasse on the glacis, §7), Amsterdam (gates moved outward, §6) and Timgad (founding wall dismantled, §13) | `core.vienna.glacis-width`, `core.timgad.walled-area-100` | as row 6 | resolved in favour of the city blocks |
| 7a | TS §14 before the correction said six cities were rebuilt after fire on the same street lines; the blocks show street changes after fires in London 1666, Constantinople 1865 and 1870 and Edo 1657, and a new grid in Mexico City after 1521 | `core.london.fire-area-1666` | as row 6 | resolved in favour of the city blocks |
| 7b | TS §15 item 3 before the correction said four operations (grow, shrink, burn, relocate the ruler) cover every transition; Timgad's new fort on the spring (§13) and Mexico City's new grid (§10) fit none of them | `core.timgad.fort-to-city`, `core.mexico.traza-area` | as row 6 | not supported; cited only in §7.5, to reject it; no fact rests on it |
| 7c | TS §8C before the correction said Dublin's bridges were rebuilt five times on the same sites; the §8 block shows two rebuilds in place (Whitworth Bridge on the Old Bridge site, Queen's Bridge on the Arran Bridge site) | `core.dublin.bridge-replacement-shift` | as row 6 | resolved in favour of the city block |
| 8 | Timgad's peak area: more than 90 ha against 60 ha of ruins | `core.timgad.peak-area-90`, `core.timgad.peak-area-60` | Both in the same French Wikipedia article: one source contradicting itself, not two sources disagreeing | unverified; each fact lists the other in `disputes` (repair round 2026-10-21; was disputed) |
| 9 | Tenochtitlan in 1519: 750 ha (Valero 1990) against 800-1,350 ha (Wikipedia, Tenochtitlan, quoted in TS) and against 1,350 ha (Smith 2005) | `core.tenochtitlan.area-1519-valero`, `core.tenochtitlan.area-1519-time-series`, `core.tenochtitlan.area-1519-smith` | Three sources; iberian.md's density note says Valero's population figure "may use a larger area that includes Tlatelolco"; VP confirms only that 1,350 ha is Smith's figure. 1,350 ha lies inside 800-1,350 ha, so Smith and Wikipedia do not conflict | disputed: Valero against Wikipedia, Valero against Smith; Smith stays verified as quoted |
| 10 | Edo in 1748: 6,000-8,000 ha (TS) against late-Edo land classes of 38.7, 8.8 and 8.9 km² (japanese.md), which sum to 5,640 ha (my arithmetic) | `core.edo.area-1748` | Different dates and different definitions of the city; japanese.md states no total, so there is no competing fact | unverified (repair round 2026-10-21; was disputed) |
| 11 | Salvador's upper town: "about 65 m above the sea" (case study) against "an escarpment of about 60 m" (finding 5), same report | `core.salvador.upper-town-height` | One dossier; the report's own table gives "about 60 to 65 m" | recorded as 60-65 m, unverified (repair round 2026-10-21; was disputed) |
| 12 | Kaogong ji "market behind the court": unverified, search snippet only (CF §4C table) against SS rule 21, which SS rates "strong custom" in its own strength column and cites to the Wikisource original (chinese.md quotes the original) | qualitative | CF did not reach the original; the later reports did | resolved in favour of SS and chinese.md |
| 13 | "Roads first" (roads and fortification slices) against "water first" (quantitative studies) | qualitative, CF §2 | CF §2 reconciles them: "most crossings that seeded towns are *river* crossings" | resolved by CF |
| 14 | Innsbruck: dates 1133 against 1180 | qualitative, CF §4A case 5 | Sources differ | unresolved; no fact depends on it |

## 6. Gaps

1. No measured distance from crossing to market, core to main temple, or core to citadel in a
   town of 500 m to 5 km. The minor-town series (issue #27) was named for this and is not done.
2. No study gives the share of road-first against town-first foundings (CF §4A "Not found").
3. No study gives a radius for the "centre" of a town. The 150 m default in spec-v1 has no
   research figure behind it. The nearest figures are Timgad's 355 m founding square, Dubrovnik's
   300 m spine and Djenné's 400 by 150 m esplanade, all of different kinds.
4. No figures for what decides between two competing crossings in one town.
5. No growth rates for small towns; every rate in §4.3 is for a city or a capital except Gedi,
   Djenné-Djèno and Timgad.
6. Chinese county-seat siting criteria (rivers, feng shui, roads): not found (CF §4B "Not
   found"; chinese.md "Not found").
7. Kostof, Mumford and Bairoch were not read; Bosker and Buringh's 2017 tables beyond Table 1
   were not read.
8. Dublin's 15 m founding ridge rests on one source, a consultancy report (TP source 8, AWN
   Consulting, quoting Bradley and King's Urban Survey of 1988 and Simms and Murray). TP does not mark that source weak. This analysis
   judges one indirect consultancy source too weak to record the height as a fact; Jeffrey
   decides a dispute about that judgment (ADR-023).
9. Several facts are the research reports' own syntheses (sources `citymapmaker-*`); their
   original sources are standard secondary literature not named per figure.

## 7. Check of the spec decisions

Source of the decisions: `docs/proposals/spec-v1.md` §4 "Town core" and "Visibility", §6 items
14 and 22-29, and ADR-021 on branch `m1/spec-proposal`.

### 7.1 Core at the crossing on the most important road, then closest to the site centre

- **Supported**: a crossing as the seed (§2 rows 2-3; CF §4A "Treat the crossing point as the
  seed"; CF §3 item 3); the primary crossing stays fixed for centuries
  in the city blocks of Rome, Paris, London, Dublin and Edo (TS §2, §4, §5, §8, §11; §4.1).
- **Not supported**: the spec ranks road–river and road–road crossings only by road importance.
  The research ranks water above roads: river +1.2 against Roman road +0.4 points
  (`core.europe.bb2017-river-effect`, `core.europe.bb2017-roman-road-effect`); a hub's size is
  unreliable (`core.europe.bb2017-road-hub-effect`); CF §2 names "the crossing of a road and a
  river" as the strongest candidate.
- **Not supported, not contradicted**: "closest to the site centre" as the tie-break. The only
  written rule with a tie-break is Vitruvius and Ordinance 112: the landing place on a coast,
  the middle inland. The spec matches the inland half and ignores the coastal half: a harbour
  never becomes a core candidate.
- **Not covered**: towns founded on a defensible height with routes bent to them (Ludlow,
  jōkamachi, CF §4B patterns). The spec cannot express a core at a castle gate.

### 7.2 One-road towns: the core on the road

- **Supported**: linear road towns exist in several cultures (§2 row 4: Watling Street,
  Tōkaidō, Nakasendō, street villages; `core.yoshida-juku.length`, `core.narai-juku.length`,
  `core.ishibashi-juku.length`).
- **Not supported, not contradicted**: the core as the single point "closest to the site
  centre". In these towns the market is the road itself over its length (continental.md case 9,
  "through-roads were widened into squares"); no report names a single core point.
- **No road at all**: the spec puts the core on the river point closest to the site centre.
  Supported in part by the water-first studies and the emporia, which are ribbons along a shore
  (§2 row 6), not points.

### 7.3 Ending roads end at the core; S3 links the core to every market square

- **Supported**: the spine from the main gate or crossing to the market stays in place in the
  city blocks of Rome, Constantinople, Paris, Amsterdam and Mexico City (§4.1); roads out
  converge on a ford or bridge at Dublin (TP layout rules, rated weak); the 1573 Ordinances lay streets from the plaza to the
  gates and roads (CF §4C finding 4); the four main Inca routes start from Cusco's central square and
  Huánuco Pampa's pathways radiate from its plaza (CF §4A findings and case 14).
- **Not contradicted, with one limit**: in Oxford the market is at the crossing (Carfax) and in
  Alnwick in the road fork, so the main street from core to market has length close to zero.
  In 14 towns of CW the market is outside the wall or on the water edge
  (`core.corpus-world.market-outside-wall`, `core.corpus-world.market-water-edge`); a main street
  from the core to such a market crosses the town. Nothing in the research says that is wrong.

### 7.4 Compass regions measured from the core, centre radius 150 m

- **Supported**: zoning by direction or ring from a centre is written law or custom in several
  cultures: Arthashastra sectors from "the heart of the site", Manasara rings, Japanese rank by
  distance to the castle, Sjoberg's status falling with distance from the centre (south-asia.md
  findings; japanese.md; SS §1).
- **Not supported**: the reference point in all of these is the planned centre, the temple or
  the castle, not a road–river crossing. In a road-first town the core and the centre coincide;
  in a castle-first town (14 CW towns with the ruler on the edge on high ground,
  `core.corpus-world.ruler-edge-high`) they do not.
- **Not supported**: 150 m. No report gives a centre radius (§6 gap 3).

### 7.5 Growth phases as a future idea (issue #51)

- **Supported**: towns grow in plan units that never rewrite earlier ones unless a rebuild event
  says so (CF §4D; CF §3 item 6); the TS city blocks show growth by rings and estates outside
  the edge (Amsterdam §6, Paris §4, London §5) and shrinking by emptying the inside (Rome §2,
  Ayutthaya §12); TS §15's claim that four operations cover every transition is not supported
  (§5 row 7b); generators already grow towns as an ordered event list (growth-generators.md, Emilien 2012, Mas 2020; "What we could borrow"
  item 2).
- The research says nothing about whether phases belong in version 1. It does say which objects
  a phase must keep (§4.1) and that a moved landmark re-weights later growth
  (`core.dublin.custom-house-move`).

## 8. What this means for the spec

Jeffrey decides each question. Each lists options, the evidence and my recommendation.

**Q1. Should a road–river crossing outrank a road–road crossing when choosing the core?**

- A. Keep spec-v1: rank only by road importance, then distance to the site centre.
- B. Within the same road importance, a road–river crossing comes before a road–road crossing.
- C. A road–river crossing always comes first, whatever the road importance.

Evidence: `core.europe.bb2017-river-effect` (+1.2) against `core.europe.bb2017-roman-road-effect`
(+0.4); `core.europe.bb2017-road-hub-effect` (unreliable size); CF §2 first bullet;
`core.britain.mr-water-access-medieval`.
Recommendation: B. It follows the evidence without letting a minor track's ford beat the main
road, which ADR-021's third round rejected.
Not recommended: A (it ignores the water-first evidence, §7.1); C (a minor track's ford would
beat the main road).

**Q2. Should a harbour be a core candidate on a coast?**

- A. Keep spec-v1: a harbour never decides the core.
- B. Where a road meets the harbour, that point is a crossing candidate ranked like a road–river
  crossing.
- C. When the spec has a harbour, the core is at the harbour.

Evidence: Vitruvius Book I ch. 7 and Ordinance 112 (CF §4C); `core.europe.bb2017-sea-effect`;
emporia and timber ports (§2 row 6, `core.timber-ports.spine-parallel-shore-towns`); market on
the water edge in 7 CW towns (`core.corpus-world.market-water-edge`).
Recommendation: B. It adds the coastal half of the only written centre rule with one new
candidate kind and keeps road importance in charge.
Not recommended: A (it ignores the coastal half of that rule, §7.1); C (it takes road
importance out of the choice on every coast).

**Q3. Should a spec be able to put the core at a pinned landmark (a castle-first town)?**

- A. No in version 1; the GM pins the market and the citadel landmark instead, and the core
  stays at the crossing.
- B. Add an optional `core: <pinned landmark id>` that replaces the derived core.
- C. Leave it to issue #31 (roads that bend toward a site placed first).

Evidence: CF §4B patterns (defensible site first; Ludlow; jōkamachi main road "reworked to pass
through the city"); `core.corpus-world.ruler-edge-high`; `core.cliff-towns.citadel-on-top-towns`;
`core.peninsula-ports.citadel-at-tip-towns`.
Recommendation: C. A core away from every road leaves the roads ending in the wrong place;
#31 already holds the road change, and the core option belongs with it.
Not recommended: A (it cannot express a castle-first town, §7.1 "Not covered"); B (without the
road change of #31 the roads end in the wrong place).

**Q4. What happens to the 150 m centre radius?**

- A. Keep 150 m as the default and record in ADR-021 that no research figure supports it.
- B. Scale the radius with population by a formula chosen now.
- C. Keep 150 m now and revisit after the minor-town series (issue #27) measures real cores.

Evidence: §6 gap 3; `core.timgad.founding-square-side`, `core.dubrovnik.spine-length`,
`core.baghdad.centre-to-wall` are the only figures, and each measures something different.
Recommendation: C. There is no figure to scale by; A without a revisit date leaves an
unsupported default in place for good.
Not recommended: A (no revisit date); B (no figure to scale by).

**Q5. Should the default rule pack place the market near the core?**

- A. No; the market is placed by the spec's own rules, and S3 links it to the core (spec-v1
  item 28).
- B. Yes; add `core` as a constraint target, and the default pack gives the market a soft
  `near: core` rule. A spec overrides it.
- C. Add a market-form choice (street market, square, triangle at a road fork) to the schema.

Evidence: §3.2 (Carfax, Alnwick, Herefordshire, Bavarian street markets);
§4.1 spine row; continental.md generator section ("market form enum"). Fact ids: none.
The §3.2 rows for road-first towns are placement findings without a figure, and those are
not facts (see Result). The `near: core` rule of B therefore cites these report sections,
not fact ids.
Recommendation: B. It needs no schema change beyond one more target kind and matches the
road-first towns the core rule already assumes. C is a larger change and belongs with the open
spaces topic.
Not recommended: A (it does not place the market near the core, where §3.2 shows it in the
road-first towns); C (the larger schema change).

**Q6. In a one-road town, is the core a point or a stretch of road?**

- A. Keep spec-v1: a point, the one closest to the site centre.
- B. A point, and the default pack widens the road into a street market around it.
- C. A stretch of road: the core becomes a segment.

Evidence: §7.2; `core.yoshida-juku.length` (2.6 km), `core.narai-juku.length`,
`core.ishibashi-juku.length`; continental.md case 9.
Recommendation: B. A segment changes every measurement that uses the core (compass regions,
road ends); widening the road reproduces the street market without that change.
Not recommended: A (it leaves out the street market of continental.md case 9); C (it changes
every measurement that uses the core).

**Q7. For issue #51 (growth phases), which objects does a later phase keep fixed?**

- A. Water lines, wall line, primary crossing, spine, main temple, market and citadel, as TS
  §14 claimed before the correction.
- B. Water lines, primary crossing, spine, main temple and citadel; the market and the wall
  line are allowed to move, as the city blocks show.
- C. Decide when #51 is designed; record the exceptions now.

Evidence: §4.1 (the exceptions column); §5 rows 6 to 7c; `core.ayutthaya.occupied-share-1926`,
`core.timgad.fort-to-city`, `core.vienna.glacis-width`, `core.amsterdam.exchange-move`.
Recommendation: C. #51 is not scheduled, and the market
and wall-line exceptions need the minor-town series (issue #27) before a rule rests on them. A
is contradicted by the city blocks and is not an option I recommend at any point.
Not recommended: A (contradicted by the city blocks); B (it rests a rule on the market and
wall-line exceptions before the minor-town series measures them).
