# City through the ages: twelve cities at several dates

> Written 2026-10-19 by three research agents, four cities each, and assembled by Claude.
> GitHub issue #26. A claim here is binding only when a DECISIONS.md entry cites it.
> Population and hectare figures are approximate; see the conventions below.

Research report, 2026-10-19. Purpose: the two earlier corpora (`town-plan-corpus.md`,
`town-plan-corpus-world.md`) show one city at one date. This one shows one city at three to
six dates spanning two to fifteen centuries, to answer a question the generator's design
depends on: when a city grows, shrinks, burns or changes hands, what stays in the same place?
That set is what "pinned" has to mean (ADR-005, REQUIREMENTS.md F7), and it is the first
evidence for the v2 goal "evolution over time" that REQUIREMENTS.md keeps out of v1.

Terms: *pinned* means a landmark whose position is an input, never moved by regeneration
(LEARNING.md); a *spine* is the one street that joins the main gate or crossing to the
market; *walls into boulevards* is the pattern where a demolished wall ring becomes a ring
road on its own line; *disabitato* is the Italian term for the empty zone inside Rome's
wall between about 550 and 1870.

Conventions:

- **Every plan URL was fetched on 2026-10-19** by the agent that found it and returned the
  expected Wikimedia Commons file page or, for the Irish Historic Towns Atlas, the Royal Irish
  Academy page and a PDF with HTTP 200. Exceptions, each marked where it appears: the Vienna
  Third Military Survey sheet and the Moscow Atlas Russicus copy were found via the Commons API
  with the file page not fetched; the UCD Digital Library sheets for Dublin were not verified;
  the Dublin 1728 Brooking plan is on a Royal Society of Antiquaries of Ireland (RSAI) page, not
  Commons, and this report does not record whether that page was fetched. Pixel sizes and
  licence tags are as the host states them. "(unverified)" marks a fact the agent could not
  check in this session.
- **Plan types**: survey = measured ground plan; bird's-eye = oblique pictorial view, reliable
  for topology and gate counts, not for distances; reconstruction = drawn later from texts and
  excavation.
- **Date depicted** is the state of the city the plan shows, which differs from the drawing
  date for reconstructions and reprints.
- **Hectare and population figures** come from standard secondary literature, not from
  measurement of the plans; every one is marked "(unverified)" unless a census is cited.
- Two plans less than five years apart (Ayutthaya 1687 and 1690) are treated as one state.

## 1. Overview

| § | City | Plans | Span | Regress event in the span |
|---|---|---|---|---|
| 2 | Rome | 5 | c. 350 to 1873 | population down by a factor of twenty (95 percent), built area down 70 to 78 percent |
| 3 | Constantinople | 4 | 1420 to 1882 | 1453 conquest; fires of 1660, 1693, 1718, 1756, 1865, 1870 |
| 4 | Paris | 4 | 1380 to 1871 | 1420s occupation and plague; 1871 Commune fires |
| 5 | London | 4 | 1561 to 1799 | 1666 Great Fire, four fifths of the walled city |
| 6 | Amsterdam | 6 | 1544 to 1839 | none; a 150-year plateau with no change of extent |
| 7 | Vienna | 5 | 1547 to 1878 | 1683 siege, suburbs burned by the defenders |
| 8 | Dublin | 6 | 1610 to 1837 | none; one landmark moved 800 m and dragged growth with it |
| 9 | Moscow | 5 | 1610 to 1886 | fire of 1611 (everything outside the Kremlin and Kitai-gorod); fire of 1812 (about two thirds of the built city) |
| 10 | Mexico City | 6 | 1520 to 1884 | 1521 siege razed the city; 1629 to 1634 flood |
| 11 | Edo / Tokyo | 6 | c. 1630 to 1886 | 1657 fire, 100,000 dead; 1868 change of regime |
| 12 | Ayutthaya | 4 | c. 1665 to 1926 | 1767 destruction and 160 years of abandonment |
| 13 | Timgad | 6 | 100 CE to 1927 | shrank from 90 ha to a 1 ha fort; abandoned by the 8th century |

Per city: a table of dated plans (A), one block per consecutive pair saying what grew, what
shrank, what stayed fixed and what moved (B), and one closing line (C). §14 is the cross-city
finding. §15 is what the generator should take from it.

## 2. Rome

### A. Plans

| Date depicted | Plan (author, date drawn) | Type | Pop. at that date | Plan size px | URL | Licence |
|---|---|---|---|---|---|---|
| c. 300–400 AD (ancient city at its largest extent) | *Forma Urbis Romae*, Rodolfo Lanciani, 46 sheets, drawn 1893–1901; this file is the assembled overview | reconstruction (ancient remains in red over a modern survey base in black) | 700,000–1,000,000 (unverified; estimates for the 2nd–4th c. range 450,000–1,200,000) | 14000 x 9500 | https://commons.wikimedia.org/wiki/File:Rodolfo_Lanciani_-_Forma_Urbis_Romae_-_Overview.jpg | Public domain (PDM 1.0; author d. 1929) |
| 1551 | *Roma*, Leonardo Bufalini, surveyed 1541–1551, woodcut printed 1551; this file is the Accademia delle Scienze di Torino copy photographed by Henry A. Millon | survey (first printed ichnographic plan of Rome; ancient monuments drawn as reconstructed plans among the modern streets) | 45,000–55,000 (1526 census: 55,035; after the 1527 sack the figure fell and recovered to this range by 1550) (unverified) | 5113 x 3692 | https://commons.wikimedia.org/wiki/File:Mappa_di_Roma,_1551_-_Archivio_Accademia_delle_Scienze_Torino,_Millon_66_28_328.jpg | Public domain (PDM 1.0) |
| 1748 | *Nuova Pianta di Roma* (the "Pianta Grande"), Giovanni Battista Nolli, surveyed 1736–1748, 12 engraved sheets; this file is the Cleveland Museum of Art composite | survey (ichnographic; public interiors of churches drawn open; the empty walled zone, the *disabitato*, drawn as gardens and vineyards) | 140,000–160,000 (1750: 157,000 (unverified)) | 14000 x 11956 | https://commons.wikimedia.org/wiki/File:Nuova_pianta_di_Roma_data_in_luce_da_Giambattista_Nolli_l%E2%80%99anno_MDCCXLVII,_known_as_La_Pianta_Grande_di_Roma_(IA_clevelandart-2020.276-nuova-pianta-di-roma).tif | Public domain (PDM 1.0) |
| 1866 | *Pianta della Città di Roma*, Augusto Fornari, engraved 1866 (Geographicus scan) | survey (pocket map; last plan of papal Rome before 1870) | 200,000–215,000 (1866 papal count: 210,000 (unverified)) | 6000 x 4299 | https://commons.wikimedia.org/wiki/File:1866_Fornari_Pocket_Map_or_Case_Map_of_Rome_Italy_-_Geographicus_-_Rome-fornari-1866.jpg | Public domain (PDM 1.0) |
| 1873 | *Piano regolatore* of Rome ("Piano Viviani"), Ing. Alessandro Viviani for the Comune di Roma, 1873 | survey base with the proposed new quarters and streets overlaid (planning map) | 225,000–245,000 (1871 census: 244,484 in the comune, 212,432 in the walled city) | 1500 x 1149 | https://commons.wikimedia.org/wiki/File:Piano_Viviani_(Roma,_1873)_-_1.jpg | Public domain (PDM 1.0) |

Span: c. 350 AD to 1873, 15 centuries. Note on the 1873 plan: it is the only one of the five
at low resolution (1500 px); the 1866 Fornari plan is the high-resolution survey of the same
state and the two are read together below.

### B. Changes between consecutive plans

**c. 350 AD (Lanciani) to 1551 (Bufalini)**

- Grew: nothing. The Aurelian Wall (271–275 AD, 19 km, enclosing 1,370 ha) was the outer
  limit on both plans and stayed the outer limit until 1870. The Leonine Wall around the
  Vatican (848–852) and Borgo is the only new walled piece, outside the ancient circuit on the
  right bank.
- Shrank or was destroyed: the city collapsed from 700,000–1,000,000 to 45,000–55,000. The
  Gothic siege of 537–538 cut the aqueducts; the hills (Esquiline, Caelian, Aventine, Quirinal,
  Viminal) emptied and became the *disabitato* of vineyards and ruins. The inhabited zone
  (*abitato*) contracted into the Tiber bend (Campus Martius), Trastevere and the Borgo, where
  river water and the single surviving aqueduct (Acqua Vergine, repaired 1453) reached. The
  Forum Romanum became pasture (*Campo Vaccino*); the Circus Maximus and the Baths of Caracalla
  and Diocletian stood as empty shells; the imperial fora were buried 5–10 m deep.
- Stayed fixed: the Aurelian Wall line and its gates (Porta Flaminia/del Popolo, Porta
  Salaria, Porta Maggiore, Porta San Giovanni/Asinaria, Porta Appia/San Sebastiano, Porta
  Ostiense/San Paolo, Porta Portese), the Tiber bridges on their ancient piers (Pons
  Aelius = Ponte Sant'Angelo; Pons Fabricius and Pons Cestius at the Tiber Island; Ponte Sisto
  of 1473 on the piers of the Pons Aurelius; Pons Milvius), Hadrian's Mausoleum (= Castel
  Sant'Angelo), the Pantheon (church of S. Maria ad Martyres since 609, unmoved), the
  Capitoline Hill as civic centre (Palazzo Senatorio on the Tabularium), the Colosseum, the
  Via Flaminia/Via Lata = Via del Corso as the north–south spine, Via Appia, St Peter's on the
  site fixed in 319–333, the Lateran, S. Maria Maggiore, the Theatre of Marcellus (a fortress,
  then a palace, same footprint), the Tiber Island with its hospital (Aesculapius temple site).
- Moved or vanished: the eleven aqueducts dead except the Vergine; the Forum and the Colosseum
  quarried for stone; the Circus Maximus, the imperial palaces on the Palatine, the Baths and
  the Porticus of the Campus Martius gone except as footprints that the street grid of the
  Campus Martius still follows (Piazza Navona is the Stadium of Domitian, unmoved in outline).
  The papal centre moved from the Lateran (east, abandoned 1309) to the Vatican (west, from
  1377).
- Rate: built area fell from 1,200–1,370 ha (the whole walled area was built in the 2nd
  century) to 300–400 ha in 1551 (unverified; Bufalini's *abitato* is one quarter to one third
  of the walled area). Change: −70 to −78 percent over 12 centuries.

**1551 (Bufalini) to 1748 (Nolli)**

- Grew: within the same wall. Sixtus V's straight streets (1585–1590) cut the *disabitato*
  radially from S. Maria Maggiore: Via Felice/Sistina to Trinità dei Monti and Via Merulana to
  the Lateran; Via Pia (1561) from the Quirinal to Porta Pia; the Acqua Felice (1587) and Acqua
  Paola (1612) refilled the hills with water so convents and villas (Villa Borghese 1605,
  Villa Ludovisi) spread onto the Pincian and Quirinal. The trident from Piazza del Popolo
  (Via del Babuino 1525, Via di Ripetta 1518, Corso) was already on Bufalini; by Nolli it was
  built solid. Piazza Navona, Piazza di Spagna (Spanish Steps 1725), Trevi (1732–1762), the
  Quirinal Palace (1583–1650), the new St Peter's (1506–1626) and its colonnade (1656–1667),
  the Gesù and the baroque church fronts filled the Campus Martius without extending it.
- Shrank or was destroyed: no disaster. The *disabitato* stayed empty: Nolli shows the
  Esquiline, Caelian and Aventine as vineyards. Population tripled while the built area grew
  only 20 to 40 percent (see Rate below), mostly by raising building height.
- Stayed fixed: everything in the previous list; the parish churches of the Campus Martius are
  on both plans in the same positions; the Ghetto (walled 1555, between Bufalini and Nolli)
  sits on the same riverside block on Nolli; the Campidoglio reshaped by Michelangelo in place.
  Street network of the Campus Martius identical block by block.
- Moved or vanished: the Via Papalis (papal procession route) lost importance to the Corso;
  the 1551 ancient-ruin reconstructions that Bufalini drew in the fields are absent on Nolli
  because he drew only what stood; the Forum stayed Campo Vaccino; Monte Testaccio stayed empty.
- Rate: built area 300–400 ha to 400–500 ha (unverified); +20 to +40 percent in two
  centuries.

**1748 (Nolli) to 1866/1873 (Fornari, Viviani)**

- Grew: still inside the Aurelian Wall until 1870. Piazza del Popolo was rebuilt as an oval
  with the Pincio terrace (Valadier, 1811–1824); the first railway station (Termini, 1863–1874)
  was set on the Esquiline in the *disabitato*; the Viviani plan drew the Esquiline grid around
  Piazza Vittorio Emanuele II, the Via Nazionale from Termini to Piazza Venezia (1870s), the
  Castro Pretorio barracks quarter, Prati di Castello outside the Borgo, and Testaccio as an
  industrial quarter, all of which were built 1873–1900 on the vineyards that Nolli drew.
- Shrank or was destroyed: no fire or siege; the French siege of 1849 damaged the Janiculum and
  Porta San Pancrazio only. The 1870 Tiber flood (3 m in the Campus Martius) led to the
  Lungotevere embankments (1876–1900), which came after the 1873 plan.
- Stayed fixed: the Aurelian Wall and all gates (the Porta Pia breach of 20 September 1870 is a
  hole beside the gate, not a moved gate); the Corso as spine; every Tiber bridge of 1748 still
  present in 1873; one bridge was added in the span, the iron railway bridge Ponte
  dell'Industria (1863, [List of bridges in Rome](https://en.wikipedia.org/wiki/List_of_bridges_in_Rome)),
  and Ponte Garibaldi was planned 1873 and opened 1888; Castel Sant'Angelo, St Peter's, Pantheon, Capitoline, Colosseum, Lateran,
  S. Maria Maggiore, Piazza Navona, Piazza di Spagna, Quirinal (now the royal palace, same
  building); the Ghetto still in place in 1873 (demolished 1888). The four basilicas and the
  Tiber Island hospital are in the same positions on all five plans.
- Moved or vanished: the state moved into the city (Quirinal from papal to royal, 1871) without
  moving a building. The Tiber's natural banks with mills and river stairs on Nolli and Fornari
  vanished under the embankments after 1876. The *disabitato* as a category vanished: it is
  where all 1873–1900 growth went.
- Rate: 400–500 ha in 1748 to 500–600 ha in 1866 (unverified; Fornari shows the same *abitato*
  plus Termini); the 1873 plan then allotted a further 600–800 ha inside the wall. +10 to
  +30 percent over 120 years, then a planned doubling.

### C. Closing line

Rome shows that the wall ring and the river crossings outlive the population by a factor of
twenty: a city can lose 95 percent of its people and keep every gate, bridge, hill-top church
and spine street exactly in place, and regrow 1,300 years later inside the same ring.
Proposal (Jeffrey decides): pin gates, bridges and the primary spine before placing a single
house, and let built area shrink and regrow without touching them.

---

## 3. Constantinople / Istanbul

### A. Plans

| Date depicted | Plan (author, date drawn) | Type | Pop. at that date | Plan size px | URL | Licence |
|---|---|---|---|---|---|---|
| 1420–1430 (Byzantine city before 1453) | *Constantinopolis*, Cristoforo Buondelmonti, *Liber insularum Archipelagi*, original 1420–1422; this file is the Düsseldorf manuscript copy of 1485–1490 | bird's-eye (schematic; walls, columns, major churches drawn as symbols) | 40,000–50,000 (unverified; estimates for the last Byzantine decades range 25,000–70,000) | 2184 x 3181 | https://commons.wikimedia.org/wiki/File:Map_of_Constantinople_by_Cristoforo_Buondelmonti,_D%C3%BCsseldorf_copy.png | Public domain (PDM 1.0; author d. 1430) |
| 1479–1535 (early Ottoman city; the prototype drawing dates from 1479–1490 under Mehmed II (unverified), the woodcut was cut c. 1530–1535) | *Byzantivm sive Costantineopolis*, Giovanni Andrea Vavassore, woodcut, Venice, c. 1535; this file is the Germanisches Nationalmuseum copy | bird's-eye | 200,000–400,000 (Ottoman tax surveys of 1477 give 60,000–80,000; of 1535, 400,000 (unverified)) | 1800 x 1267 | https://commons.wikimedia.org/wiki/File:Giovanni_Andreas_Vavassore,_Byzantivm_sive_Costantineopolis.jpg | Public domain (PDM 1.0; author d. 1572) |
| 1776 (surveyed October–November 1776, verified 1786) | *Plan de Constantinople*, François (Franz) Kauffer and J.-B. Lechevalier, surveyed 1776, this edition engraved Weimar 1807; this file is the National Library of Israel copy | survey (first triangulated plan of the city) | 300,000–500,000 (unverified; no census; estimates for 1770–1800 range 300,000–600,000) | 16782 x 13849 | https://commons.wikimedia.org/wiki/File:Franz_Kauffer,_Plan_de_Constantinople_(FL165745902_2369973).jpg | Public domain (PDM 1.0, author's death date unknown, work older than 120 years) |
| 1882 | *Plan von Constantinopel mit den Vorstädten, dem Hafen, und einem Theile des Bosporus*, C. Stolpe, 1882 (UWM Libraries copy) | survey | 850,000–900,000 (1885 Ottoman census: 873,565) | 8654 x 7008 | https://commons.wikimedia.org/wiki/File:Plan_von_Constantinopel_mit_den_Vorst%C3%A4dten,_dem_Hafen,_und_einem_Theile_des_Bosporus.jpg | Public domain (PDM 1.0) |

Span: 1420 to 1882, 4.6 centuries. The Düsseldorf Buondelmonti copy is used instead of the
Paris BnF copy (Commons file 592 x 746 px) because it is four times larger; it is a copy
made 60 years after the original and adds nothing Ottoman, so it still depicts the Byzantine
city.

### B. Changes between consecutive plans

**1420 (Buondelmonti) to 1479–1535 (Vavassore)**

- Grew: inside the same Theodosian Wall (413 AD, 5.7 km of land wall enclosing 1,400 ha with
  the sea walls). Mehmed II repopulated the city by forced resettlement after 1453; Vavassore
  shows the city built solid where Buondelmonti showed fields and orchards between isolated
  monasteries. Galata/Pera across the Golden Horn, a walled Genoese colony on Buondelmonti,
  spread uphill on Vavassore. Üsküdar on the Asian shore appears on Vavassore.
- Shrank or was destroyed: the 1453 siege breached the land wall at the Lycus valley (Gate of
  St Romanus/Topkapı); the wall was repaired in place. The Byzantine city of 1420 was already
  at its minimum: Buondelmonti draws large empty areas inside the walls, cisterns open as
  gardens, the Great Palace in ruins.
- Stayed fixed: the Theodosian land wall and its gates (Golden Gate, Gate of Rhegion/Mevlevihane,
  St Romanus/Topkapı, Charisius/Edirnekapı), the sea walls, Hagia Sophia (church to mosque
  1453, same building), the Hippodrome with its obelisk and serpent column, the Mese spine
  from Hagia Sophia to the Charisius Gate (Ottoman Divanyolu), the Column of Constantine in his
  Forum (Çemberlitaş), the Column of Arcadius/Xerolophos, the Aqueduct of Valens, the Galata
  Tower (1348) on both, the Golden Horn as harbour with its chain, the Pantokrator and
  Chora monasteries (as mosques), the Acropolis point (Seraglio Point).
- Moved or vanished: the Church of the Holy Apostles (Buondelmonti) was demolished in 1461 and
  the Fatih mosque (1463–1470) built on the same hilltop; the Great Palace site stayed ruinous
  while the new Topkapı Palace (1459–1465) was built on the Acropolis; the Blachernae palace
  vanished; the Forum of Theodosius became the Old Palace (Eski Saray) and later the bazaar
  district; the Grand Bazaar (bedesten 1456) is new on Vavassore. The imperial centre thus moved
  a short way north-east, from the Great Palace between the Hippodrome and Hagia Sophia
  ([Great Palace of Constantinople](https://en.wikipedia.org/wiki/Great_Palace_of_Constantinople))
  to Seraglio Point beyond Hagia Sophia
  ([Topkapı Palace](https://en.wikipedia.org/wiki/Topkap%C4%B1_Palace)): both sites adjoin
  the head of the Mese, and the new one lies past its end, not along it.
- Rate: built area 400–600 ha in 1420 (unverified; one third to one half of the walled area)
  to 1,000–1,300 ha by 1535 (walled area filled) (unverified). +100 to +200 percent in one
  century, all inside the fixed wall.

**1479–1535 (Vavassore) to 1776 (Kauffer)**

- Grew: the walled city was full by 1535, so growth went outside: Eyüp up the Golden Horn,
  Kasımpaşa (arsenal) and Galata/Pera on the north shore, villages along the Bosporus
  (Beşiktaş, Ortaköy), and Üsküdar on the Asian side, all drawn by Kauffer as separate built
  patches. The great imperial mosques fixed the skyline: Bayezid (1506), Şehzade (1548),
  Süleymaniye (1557), Sultan Ahmed (1616) on the Hippodrome's edge, Yeni Cami (1665) at the
  Eminönü landing.
- Shrank or was destroyed: fires of 1660 (two thirds of the city), 1693, 1718 and 1756
  each burned thousands of timber houses; each time the street plan was rebuilt on its own
  lines because property boundaries (vakıf and private) stayed. The 1766 earthquake brought
  down the Fatih mosque dome; the mosque was rebuilt on its plan.
- Stayed fixed: the Theodosian walls and gates (Kauffer surveys them exactly), Hagia Sophia,
  Hippodrome, Divanyolu/Mese, Çemberlitaş, Aqueduct of Valens, Galata Tower, Topkapı Palace,
  Grand Bazaar, Fatih, Golden Horn harbour, the Eminönü landing opposite Galata. Every
  Vavassore landmark that is a masonry monument is on Kauffer in the same place.
- Moved or vanished: the Byzantine Forum of Theodosius vanished under Bayezid Square; the
  Galata city walls were on Kauffer but partly built over; the Byzantine harbours on the Marmara
  (Theodosius/Langa) were silted and were market gardens on Kauffer (they were on Buondelmonti
  as harbours).
- Rate: walled city stayed 1,000–1,300 ha built; suburbs added 400–700 ha (unverified). Total
  +40 to +60 percent in 2.4 centuries, none of it inside the wall.

**1776 (Kauffer) to 1882 (Stolpe)**

- Grew: Pera/Beyoğlu became a second city along the Grande Rue (İstiklal) with embassies and
  stone apartment blocks; Dolmabahçe Palace (1856) and Çırağan (1871) on the Bosporus moved
  the court out of Topkapı; the Galata Bridge (first pontoon bridge 1836, second 1845,
  Stolpe's iron bridge of 1875) joined the two shores for the first time since the Byzantine
  chain; the Rumeli railway (1872) runs on Stolpe along the Marmara sea wall into Sirkeci,
  cutting through the Topkapı gardens; the Tünel funicular (1875) in Pera.
- Shrank or was destroyed: the Hocapaşa fire of 1865 (Divanyolu district) and the Pera fire
  of 1870 (Beyoğlu, 3,000 buildings) were followed by regularised straight streets on Stolpe:
  Divanyolu widened on its old line, Pera rebuilt in masonry on a tidied version of its old
  streets.
- Stayed fixed: all walls and gates (Stolpe still draws the complete Theodosian land wall; the
  Marmara sea wall was breached only by the railway), Hagia Sophia, Hippodrome, Divanyolu,
  Çemberlitaş, Aqueduct, Galata Tower, Topkapı, Grand Bazaar, every imperial mosque, the
  Eminönü–Galata crossing point (now a bridge, same landing on each shore), Eyüp, Üsküdar
  landing.
- Moved or vanished: the court moved from Topkapı to Dolmabahçe (4 km north); the Old Palace
  site became the War Ministry (now Istanbul University); the Byzantine Boukoleon harbour
  palace was cut by the railway; wooden houses replaced by masonry along fire lines.
- Rate: built area 1,500–2,000 ha in 1776 to 2,500–3,500 ha in 1882 (unverified). +50 to
  +100 percent in one century, all outside the wall or on the Bosporus shores.

### C. Closing line

Constantinople shows that a conquest changing religion, language and ruling house relocates
the palace near the head of the spine but keeps the spine, the walls, the main church and the
harbour landing, and that repeated city-wide fires regenerate largely the same street lines
because land ownership is fixed even when buildings are not; the 1865 and 1870 fires added
widened and regularised streets. Proposal (Jeffrey decides): treat the palace as movable and
the spine, primary church, walls and landing as pinned.

---

## 4. Paris

### A. Plans

| Date depicted | Plan (author, date drawn) | Type | Pop. at that date | Plan size px | URL | Licence |
|---|---|---|---|---|---|---|
| 1380 | *Paris en 1380*, Henri Legrand, *Histoire générale de Paris, Plans de restitution*, drawn 1868, engraved by E. Lebel (Gallica copy) | reconstruction (from tax rolls and charters; shows the Charles V wall as completed 1383) | 200,000–280,000 (unverified; the 1328 hearth roll gives 61,098 hearths; after the plague the 1380 figure is contested between 100,000 and 280,000) | 10100 x 7894 | https://commons.wikimedia.org/wiki/File:Henri_Legrand,_Histoire_g%C3%A9n%C3%A9rale_de_Paris._Plans_de_restitution._Paris_en_1380,_1868_-_Gallica.jpg | Public domain (author d. 1878; page states PD France, attribution required) |
| c. 1550 | *La Ville, Cité et Université de Paris* ("plan de Bâle"), Olivier Truschet and Germain Hoyau, woodcut, 1550–1553; this file is the unique Basel University Library copy (Kartenslg AA 124) | bird's-eye | 250,000–300,000 (unverified) | 10890 x 7820 | https://commons.wikimedia.org/wiki/File:Map_of_Paris_by_Truschet_and_Hoyau_-_Basel_University_Library.jpg | Public domain (PDM 1.0; credit Basel University Library) |
| 1734–1736 (surveyed) | *Plan de Paris* ("plan de Turgot"), Louis Bretez for Michel-Étienne Turgot, surveyed 1734–1736, engraved by Claude Lucas, published 1739, 20 sheets; this file is the Norman B. Leventhal Map Center composite | bird's-eye (measured base, every building drawn in isometric) | 550,000–600,000 (unverified; no census; 1750 estimates 560,000–600,000) | 35690 x 28030 | https://commons.wikimedia.org/wiki/File:Turgot_map_of_Paris_-_Norman_B._Leventhal_Map_Center.jpg | Public domain (PDM 1.0) |
| 1871 | *Nouveau plan de Paris divisé en 20 arrondissements*, Auguste Logerot, 1871 (David Rumsey copy) | survey (shows the 1860 annexation limits, Thiers wall, boulevards built and planned) | 1,800,000–1,900,000 (1872 census: 1,851,792) | 22232 x 15550 | https://commons.wikimedia.org/wiki/File:Auguste_Logerot,_Nouveau_plan_de_Paris_divise_en_20_arrondissements,_1871.jpg | Public domain (PDM 1.0) |

Span: 1380 to 1871, 4.9 centuries. The ALPAGE vector reconstruction of 1380 (Commons
File:Plan_restitué_de_Paris_en_1380_-_ALPAGE.svg, 2403 x 2570, CC BY 2.0 fr) is a modern
alternative to Legrand and is cited for cross-checking only; Legrand is used because it is in
the same engraving convention as the other three.

### B. Changes between consecutive plans

**1380 (Legrand) to c. 1550 (Truschet–Hoyau)**

- Grew: inside the Charles V wall (1356–1383) on the right bank and the Philippe Auguste wall
  (1190–1215) on the left bank, the fields between the two right-bank walls (Marais, Temple,
  Saint-Antoine) filled with houses; faubourgs grew outside every gate along the roads:
  Saint-Honoré west, Saint-Denis and Saint-Martin north, Saint-Antoine east, Saint-Germain,
  Saint-Jacques and Saint-Marcel south. Truschet–Hoyau draws the faubourgs as continuous
  ribbons; Legrand draws them as scattered houses. The Louvre was converted from fortress to
  palace in place (Lescot wing 1546).
- Shrank or was destroyed: the English occupation (1420–1436) and the plague halved the
  population to 100,000 or below in the 1420s–1430s (unverified); the lost ground was regained
  inside the same walls by 1500. No fire or siege changed the plan.
- Stayed fixed: Île de la Cité with Notre-Dame (1163–1345), the Palais de la Cité and
  Sainte-Chapelle; the two bridge pairs on the Roman axis (Grand Pont/Pont au Change and Petit
  Pont, with Pont Notre-Dame on the 1413 line) carrying rue Saint-Martin/rue Saint-Denis north
  and rue Saint-Jacques south; Les Halles on the 1137 site; the Louvre; the Temple enclosure;
  Saint-Germain-des-Prés abbey; Sainte-Geneviève abbey; Place de Grève with the Maison aux
  Piliers (the Hôtel de Ville of 1357, rebuilt on the same spot 1533–1628); the Bastille
  (1370–1383); the Innocents cemetery; rue Saint-Honoré, rue Saint-Antoine; every gate of the
  Charles V wall (Saint-Honoré, Montmartre, Saint-Denis, Saint-Martin, Temple, Saint-Antoine)
  on both plans.
- Moved or vanished: the Philippe Auguste wall on the right bank was absorbed into houses and
  is a line of property boundaries on Truschet–Hoyau (rue des Fossés names); the royal residence
  moved from the Cité to the Hôtel Saint-Pol (1360s) and then to the Louvre (1540s), all three
  within 1.5 km on the same bank; the Hôtel Saint-Pol was demolished 1519–1543 and is streets
  on Truschet–Hoyau.
- Rate: 440 ha walled (Charles V + Philippe Auguste left bank) and 350–400 ha built in 1380
  (unverified) to 500–600 ha built including faubourgs c. 1550 (unverified). +30 to +60
  percent in 1.7 centuries.

**c. 1550 (Truschet–Hoyau) to 1734–1736 (Turgot)**

- Grew: west and north-west. The Louis XIII extension (1633–1636) walled in the Tuileries and
  the Butte Saint-Roch quarter; Île Saint-Louis was formed from two islets and built 1614–1650;
  Place Royale/des Vosges (1605–1612) and Place Dauphine (1607) are new; the Pont Neuf (1578–1607)
  is new (it is not the first bridge off the Roman axis: the Pont Saint-Michel was first built
  in 1378, [Pont Saint-Michel](https://en.wikipedia.org/wiki/Pont_Saint-Michel)); faubourg Saint-Germain became an aristocratic quarter
  (Invalides 1671–1676); faubourg Saint-Honoré and the Champs-Élysées axis (1670) opened the
  west. The Charles V and Louis XIII walls were demolished 1670–1705 and replaced by the tree-
  lined Grands Boulevards, which Turgot draws complete from the Bastille to the Madeleine.
- Shrank or was destroyed: no disaster. The Hôtel Saint-Pol area and the Marais lost nobility
  to the faubourg Saint-Germain but kept their buildings.
- Stayed fixed: Notre-Dame, Palais de la Cité, Sainte-Chapelle; the Roman axis bridges (Pont
  au Change rebuilt 1639–1647, Petit Pont, Pont Notre-Dame, all on their lines) and rue
  Saint-Martin/Saint-Denis/Saint-Jacques; Les Halles; the Louvre (enlarged in place, Cour
  Carrée 1546–1670s); Hôtel de Ville on Place de Grève; Temple; Saint-Germain-des-Prés;
  Sainte-Geneviève; the Bastille; the Innocents cemetery; Saint-Eustache; rue Saint-Honoré and
  rue Saint-Antoine as the east–west spine; the Sorbonne; the Arsenal. Of the gates, the Porte
  Saint-Denis and Porte Saint-Martin were rebuilt as triumphal arches (1672, 1674) on the exact
  gate sites.
- Moved or vanished: the walls became boulevards (the first "walls into boulevards" case in
  this corpus); the Hôtel des Tournelles (Truschet–Hoyau) demolished 1563 and replaced by Place
  des Vosges; the Tuileries palace (1564) is new; the moats of the Louvre filled; the
  Pré-aux-Clercs (open meadow on Truschet–Hoyau) built over.
- Rate: 500–600 ha to 1,100–1,300 ha built (unverified; Turgot's built extent). +100 to +130
  percent in 1.9 centuries.

**1734–1736 (Turgot) to 1871 (Logerot)**

- Grew: in every direction. The Fermiers-Généraux wall (1784–1791, 24 km, 3,370 ha) and then
  the Thiers fortifications (1841–1844, 33 km, 7,800 ha) set two new rings; the 1860 annexation
  of the communes between them (Montmartre, Belleville, Passy, Vaugirard, Grenelle and others)
  created the 20 arrondissements that Logerot colours. Haussmann's works (1853–1870) cut the
  boulevards de Sébastopol, Saint-Michel, Saint-Germain (partly), Malesherbes, the avenue de
  l'Opéra (planned on Logerot), rue de Rivoli extended east; six railway termini (1837–1849);
  canal Saint-Martin (1825); the Bois de Boulogne and Buttes-Chaumont as parks. New bridges:
  Concorde (1791), Austerlitz, Iéna, Arts, Arcole, Alma and others, from 10 bridges on Turgot
  to 24 on Logerot.
- Shrank or was destroyed: the Bastille demolished 1789–1790 (Place de la Bastille, column
  1840); the Commune fires of May 1871 (Tuileries palace, Hôtel de Ville, Palais de Justice in
  part, Cour des Comptes) are the state Logerot's 1871 plan shows or immediately precedes; the
  Hôtel de Ville was rebuilt 1874–1882 on its site, the Tuileries was not rebuilt (demolished
  1883). Haussmann demolished most of the Île de la Cité's houses (the medieval island of Legrand
  and Truschet–Hoyau survives only as Notre-Dame, Sainte-Chapelle, Palais, Place Dauphine).
- Stayed fixed: Notre-Dame; Palais de la Cité; Sainte-Chapelle; the Roman axis (boulevard de
  Sébastopol was cut parallel to rue Saint-Martin, not on it, so rue Saint-Martin, rue
  Saint-Denis and rue Saint-Jacques remain); Pont au Change, Petit Pont, Pont Notre-Dame, Pont
  Neuf; Les Halles (rebuilt as Baltard's iron pavilions 1854–1870 on the same 1137 site);
  Louvre (completed 1852–1857 in place); Hôtel de Ville site; Saint-Germain-des-Prés; the
  Panthéon on the Sainte-Geneviève abbey hill; the Grands Boulevards on the Charles V wall line;
  the Porte Saint-Denis and Saint-Martin arches; Invalides; Place des Vosges; Place Vendôme;
  Saint-Eustache; Sorbonne; Temple site (tower demolished 1808, square and market on it).
- Moved or vanished: the Innocents cemetery emptied 1786 and became a market square; the
  Temple tower gone; the Bastille gone; the Bièvre river on the left bank, open on Turgot, was
  being covered (completed 1912) and is partly absent on Logerot; the Seine's river stairs and
  the houses on the Pont Notre-Dame and Pont au Change (on Turgot) removed 1786–1788; the
  Fermiers-Généraux wall demolished 1860 and is a boulevard ring on Logerot.
- Rate: 1,100–1,300 ha to 4,000–5,000 ha built inside the 7,800 ha of the Thiers wall
  (unverified). +250 to +350 percent in 1.35 centuries.

### C. Closing line

Paris shows that each demolished wall ring becomes a boulevard ring in the same place, so the
wall line is pinned even after the wall is gone, and that the Roman crossing axis (two bridges
and three streets) stays the spine through five centuries of new bridges and boulevards.
Proposal (Jeffrey decides): keep retired wall rings as road rings and keep the first river
crossing as the spine origin.

---

## 5. London

### A. Plans

| Date depicted | Plan (author, date drawn) | Type | Pop. at that date | Plan size px | URL | Licence |
|---|---|---|---|---|---|---|
| 1561–1570 | *Civitas Londinum* ("Agas map"), author unknown (attributed to Ralph Agas; derived from the Copperplate map of c. 1553–1559), woodcut; this file is the London Archives copy printed c. 1633 (page says "around 1663"; the standard date is 1633) | bird's-eye | 80,000–120,000 (unverified) | 10832 x 4192 | https://commons.wikimedia.org/wiki/File:Agas-woodcut-map-of-London.tif | Public domain (PDM 1.0) |
| 1676 (as rebuilt after the 1666 Fire) | *A Large and Accurate Map of the City of London*, John Ogilby and William Morgan, surveyed 1673–1676, published 1676–1677, 20 sheets at 100 feet to the inch; this file is the 1894 reduced copy (London and Middlesex Archaeological Society) scanned from *Maps of Old London* (1908) | survey (first measured ground plan of London; covers the City and its liberties only) | 450,000–500,000 for the whole metropolis (unverified); the City within the walls 80,000–100,000 (unverified) | 4000 x 2366 | https://commons.wikimedia.org/wiki/File:City_of_London_Ogilby_and_Morgan%27s_Map_of_1677.jpg | Public domain (PDM 1.0) |
| 1746 (surveyed 1737–1746) | *A plan of the cities of London and Westminster, and borough of Southwark, with the contiguous buildings*, John Rocque, engraved John Pine, published Pine and Tinney 1746, 24 sheets at 26 inches to the mile; this file is the Library of Congress low-resolution composite | survey | 650,000–700,000 (unverified) | 6048 x 3164 | https://commons.wikimedia.org/wiki/File:John_Rocque%27s_map_of_London_and_Westminster,_1746_low_res.jpg | Public domain (PDM 1.0) |
| 1792–1799 (this sheet set dated 1795) | *Plan of the Cities of London and Westminster, the Borough of Southwark, and parts adjoining, shewing every house*, Richard Horwood, surveyed 1792–1799, 32 sheets at 26 inches to the mile; this file is the British Library Crace copy composite | survey (every house numbered) | 950,000–1,000,000 (1801 census: 959,310 in the area later called the County of London (unverified as to extent)) | 4000 x 2208 | https://commons.wikimedia.org/wiki/File:Plan_of_the_Cities_of_London_and_Westminster,_1790s,_R._Horwood.png | Public domain (PDM 1.0) |

Span: 1561 to 1799, 2.4 centuries. Note: the Ogilby–Morgan file covers the City of London
only (the walled city plus liberties, 1.5 km east to west). Comparison 1 and comparison 2 are
therefore made on the City extent; Westminster and Southwark are compared only between Agas,
Rocque and Horwood. The Gallica sheet set of Horwood's third edition (80 sheets, Commons
"btv1b52517400k") is the high-resolution alternative and was not used because it is 80
separate files.

### B. Changes between consecutive plans

**1561–1570 (Agas) to 1676 (Ogilby–Morgan)**

- Grew: Agas shows the walled City full, Southwark along the bridge road, the Strand ribbon
  of palaces to Westminster, and fields north of the wall at Moorfields and Spitalfields. By
  1676 (outside the Ogilby–Morgan frame but documented) Covent Garden (1630s), Lincoln's Inn
  Fields, Bloomsbury Square (1661), St James's Square (1665–1670) and Spitalfields had been
  laid out; the City liberties on Ogilby–Morgan (Smithfield, Clerkenwell edge, Shoreditch end,
  Whitechapel end, the Minories) are built solid where Agas showed gardens.
- Shrank or was destroyed: the Great Fire of 2–5 September 1666 burned 176 ha (436 acres,
  four fifths of the walled City and 25 ha outside the wall to the west), 13,200 houses, 87
  parish churches, St Paul's, the Royal Exchange, Guildhall. The whole burned area is on
  Ogilby–Morgan as rebuilt 1667–1676 under the Rebuilding Acts.
- Stayed fixed: this is the key case. Wren's, Evelyn's and Hooke's plans for a new grid or
  radial layout were rejected within weeks because freeholders rebuilt on their own plots; the
  Ogilby–Morgan survey shows the pre-Fire street network re-laid on its own lines, with only
  widening (Cheapside, Thames Street, Fleet Street), the new King Street–Queen Street pair
  from Guildhall to the river, and a few alleys closed. Fixed by name: the Roman wall and all
  seven gates (Ludgate, Newgate, Aldersgate, Cripplegate, Moorgate, Bishopsgate, Aldgate) and
  the Tower; London Bridge, the only bridge, with houses on it in both; St Paul's on the same
  site (Wren's cathedral 1675–1710 on the plan of the old, rotated 8 degrees); Guildhall
  (walls survived, re-roofed); the Royal Exchange (1570, on Agas as a later insertion; rebuilt
  1669 on the site); Cheapside, Cornhill, Leadenhall Street, Fleet Street–Ludgate Hill, Thames
  Street, Bishopsgate Street, Gracechurch Street–Fish Street Hill to the bridge; Smithfield;
  Leadenhall market; the Fleet river line; 51 of the 87 churches rebuilt by Wren on their old
  sites.
- Moved or vanished: 35 burned parish churches not rebuilt (parishes merged; 51 plus 35 is
  86, one short of the 87 burned, and the sources used here do not resolve the difference); Baynard's Castle
  (on Agas) destroyed and not rebuilt; the Steelyard replaced; the Fleet river, a tidal creek on
  Agas, was canalised 1670–1674 as the Fleet Ditch with wharves (on Ogilby–Morgan); houses on
  the wall line at Ludgate rebuilt as the wall was pierced.
- Rate: City within the walls 133 ha, unchanged; built area of the metropolis 400–500 ha in
  1560 to 1,200–1,500 ha in 1676 (unverified). +150 to +250 percent in one century; inside the
  wall, street lines kept, some widened and one new street pair added, despite 80 percent
  destruction.

**1676 (Ogilby–Morgan) to 1746 (Rocque)**

- Grew: west and north. The West End estates (Seven Dials 1690s, Hanover Square 1717, Cavendish
  Square 1717, Grosvenor Square 1725–1731, Berkeley Square 1730s) and Mayfair filled the gap
  between the Strand and Hyde Park; Spitalfields and Bethnal Green filled with weavers;
  Southwark spread along the Borough and to Bermondsey; Westminster Bridge (1739–1750) is on
  Rocque's plan as under construction, the second Thames bridge in central London (Putney
  Bridge, upstream, had opened in 1729, [Putney Bridge](https://en.wikipedia.org/wiki/Putney_Bridge));
  Greenwich and Deptford are on Rocque's sheets as separate towns. The 50 New Churches of
  1711 (Hawksmoor's Christ Church Spitalfields, St George's Bloomsbury and others) are new
  fixed points.
- Shrank or was destroyed: no disaster. The Fleet Ditch, open on Ogilby–Morgan, was covered
  from Holborn Bridge to Fleet Bridge in 1737 and is the Fleet Market on Rocque.
- Stayed fixed: the wall and the seven gates (all still standing on Rocque; demolished
  1760–1767), the Tower, London Bridge (houses still on it in 1746; removed 1758–1762),
  St Paul's, Guildhall, Royal Exchange, Monument (1677), Cheapside–Cornhill–Leadenhall,
  Fleet Street–Strand–Charing Cross–Whitehall to Westminster Abbey and Hall, Smithfield,
  Leadenhall, Covent Garden piazza, the Inns of Court, Westminster Abbey (same since 1245),
  Whitehall (palace burned 1698; Banqueting House and the street remain), St James's Palace,
  Lambeth Palace.
- Moved or vanished: Whitehall Palace burned 1698 and was never rebuilt (streets and houses on
  Rocque); the open Fleet became a covered market; Moorfields drained and laid out as walks;
  the Fleet Ditch's wharves vanished.
- Rate: 1,200–1,500 ha to 2,500–3,000 ha built (unverified). +70 to +130 percent in 70 years.

**1746 (Rocque) to 1792–1799 (Horwood)**

- Grew: north to the New Road (1756, now Marylebone–Euston–Pentonville Road), with Portland
  Place (1770s), Bedford Square (1775–1783), Fitzroy Square (1790s), Somers Town and
  Pentonville; east through Whitechapel, Stepney and Mile End; south through Southwark,
  Lambeth, Newington and Kennington once Westminster Bridge (1750) and Blackfriars Bridge
  (1769) opened the south bank, with the new roads to them (Westminster Bridge Road, Blackfriars
  Road, St George's Circus 1771). Three bridges where there was one in 1676. Somerset House
  rebuilt 1776–1796 on its site. The docks (West India 1802, London 1805) come after Horwood.
- Shrank or was destroyed: no disaster. The City gates were demolished 1760–1767 and the wall
  was partly removed where it blocked traffic; the houses on London Bridge removed 1758–1762.
- Stayed fixed: the Tower, St Paul's, Guildhall, Royal Exchange, Monument, Bank of England
  (on its 1734 site, enlarged by Soane 1788–1808), London Bridge (the medieval bridge still in
  place on Horwood; replaced 1824–1831 by a new bridge 30 m upstream, after this plan), every
  main street named above, Smithfield and Leadenhall markets, Westminster Abbey and Hall,
  St James's, the West End squares of 1676–1746, Covent Garden, the Inns of Court, the wall
  line as a line of streets (London Wall, Houndsditch, Old Bailey). The gate names stayed as
  street and ward names on Horwood after the gates were gone (Ludgate Hill, Newgate Street,
  Aldersgate Street, Bishopsgate, Aldgate, Moorgate).
- Moved or vanished: the seven gates; the City wall at Ludgate, Newgate and Aldgate; the
  Fleet river covered to the Thames (Bridge Street built over it to Blackfriars Bridge, 1760s);
  the Tyburn and Westbourne streams culverted under the new West End streets; Ranelagh and
  Vauxhall gardens still open; the open fields between Marylebone and Paddington built.
- Rate: 2,500–3,000 ha to 3,500–4,500 ha built (unverified). +30 to +60 percent in 50 years.

### C. Closing line

London shows that a street network survives even the total destruction of the buildings on it
(1666) because plot boundaries are the durable object, and that a wall ring whose gates have
been demolished persists as a line of streets and a set of gate-named places. Proposal
(Jeffrey decides): store plots and street lines as the pinned layer and treat buildings as
regenerable on top of them; demolished gates stay as named nodes.

---

## 6. Amsterdam

### A. Plans

| Date depicted | Plan (author, date drawn) | Type | Pop. at that date | Plan size px | URL | Licence |
|---|---|---|---|---|---|---|
| 1544 | Cornelis Anthonisz, woodcut in 12 blocks, 1544 (after his 1538 painting) | bird's-eye | 12,000-15,000 (unverified) | 12401 x 12022 | https://commons.wikimedia.org/wiki/File:Map_of_Amsterdam,_1544_by_Cornelis_Anthonisz._Universiteit_van_Amsterdam_OTM_HB-KZL_W.X._015.jpg | Public domain (PD mark 1.0) |
| 1550-1570 (c. 1560) | Jacob van Deventer, manuscript survey for Philip II, between 1550 and 1570 (Noord-Hollands Archief copy) | survey | 25,000-30,000 (unverified) | 4283 x 5229 | https://commons.wikimedia.org/wiki/File:Kaart_van_Amsterdam_c1550_(City_Map)_Jacob_van_Deventer.jpg | Public domain (PD mark 1.0) |
| 1625 | Balthasar Florisz. van Berckenrode, engraved 1625 (Stadsarchief Amsterdam copy) | bird's-eye on a measured base | 105,000 (1622 count, unverified) | 19221 x 16774 | https://commons.wikimedia.org/wiki/File:Map_of_Amsterdam,_1625_by_Balthasar_Florisz._van_Berckenrode_Stadsarchief_Amsterdam_UZFA00029000001.jpg | Public domain (PD mark 1.0) |
| c. 1688 | Frederik de Wit, "Exactissima Amstelodami veteris et novissimi delineatio", c. 1688 | survey (plan with block fill) | 200,000 (unverified) | 5370 x 4507 | https://commons.wikimedia.org/wiki/File:Amsterdam1688.png | Public domain (PD mark 1.0) |
| 1772 | Pieter Mol (engraver), published Jan Mol, "Nieuwe kaart van de Wydberoemde Koopstat Amsteldam", 1772 (Rijksmuseum RP-P-AO-20-68) | survey | 200,000-210,000 (unverified) | 6330 x 5122 | https://commons.wikimedia.org/wiki/File:Plattegrond_van_Amsterdam_en_omstreken_Nieuwe_kaart_van_de_Wydberoemde_Koopstat_Amsteldam_met_deszelfs_Gebied_Beneevens_alle_de_Plaisante_Weegen_en_Paden_buiten_de_Stat,_Als_naa_Diemen,_de_geheele_Diemer_Meer,_Als_meede,_RP-P-AO-20-68.jpg | CC0 1.0 |
| 1839 | Gebrs. Diederichs, "Omnibus Kaart of Nieuwe Platte Grond der Stad Amsterdam", 1839 (National Library of Norway copy) | survey | 210,000-220,000 (unverified; 1830 census 202,000 unverified) | 8843 x 6786 | https://commons.wikimedia.org/wiki/File:Omnibus_Kaart_of_Nieuwe_Platte_Grond_der_Stad_Amsterdam_waarop_alle_de_-_no-nb_krt_00779.jpg | Public domain (PD mark 1.0) |

Also verified, not used in the pairs below: Joan Blaeu 1649, 6937 x 5463, PD,
https://commons.wikimedia.org/wiki/File:Map_of_Amsterdam_-_Amstelodami_Celeberrimi_Hollandiae_Emporii_Delineatio_Nova_(J.Blaeu,_1649).jpg
(shows the city between the third (1613) and fourth (1663) extensions).
Not found on Commons: a 1660s-1680s Blaeu plan of the finished ring; de Wit c. 1688 is used instead.

### B. Between plans

**1544 -> c. 1560 (Anthonisz -> Van Deventer)**
- Grew: no new wall. Building densified inside the existing medieval wall (Singel on the west, Kloveniersburgwal and Geldersekade on the east). The unwalled shipyard quarter Lastage east of the wall is a suburb on both plans.
- Shrank or was destroyed: nothing recorded between the two plans.
- Stayed fixed: Dam with the old town hall and the Nieuwe Kerk; Oude Kerk; Damrak and Rokin (the Amstel outlet) as the spine; Warmoesstraat and Nieuwendijk; Sint Antoniespoort (the later Waag); Haarlemmerpoort; Regulierspoort; Schreierstoren; Montelbaanstoren; the wall line itself.
- Moved or vanished: nothing. The two plans differ in projection (oblique view versus measured plan), not in content.
- Rate: walled area 150-180 ha on both (unverified); change in built area not estimable from these plans.

**c. 1560 -> 1625 (Van Deventer -> Florisz)**
- Grew: three planned extensions. First (1585): Lastage walled in, Uilenburg, Marken and Rapenburg islands built on the east. Second (1592-1593): Vlooienburg and the area to the Amstel. Third (1613): the western canal ring (Herengracht, Keizersgracht, Prinsengracht from Brouwersgracht to Leidsegracht), the Jordaan, Haarlemmerbuurt and the Westelijke Eilanden, all inside a new bastioned wall. Westerkerk (begun 1620), Noorderkerk (1620-1623) and Zuiderkerk (1603-1611) are on the 1625 plan.
- Shrank or was destroyed: nothing; the medieval wall was removed and its ditch (Singel) became an inner canal.
- Stayed fixed: Dam, Nieuwe Kerk, Oude Kerk, Damrak-Rokin spine, Warmoesstraat, Nieuwendijk, Kalverstraat, Sint Antoniespoort (converted to the Waag 1617, same building), Schreierstoren, Montelbaanstoren. The whole 1560 street net is intact inside the 1625 city.
- Moved or vanished: Haarlemmerpoort and Regulierspoort moved outward with the wall; the medieval wall itself vanished into canal quays.
- Rate: walled area from 150-180 ha to 440-460 ha (unverified), a factor of 2.5-3 in 65 years.

**1625 -> c. 1688 (Florisz -> de Wit)**
- Grew: fourth extension (1663): the three canals continued from Leidsegracht round to the Amstel and on to the east (Plantage, Oostelijke Eilanden Kattenburg, Wittenburg, Oostenburg with the Admiralty and VOC yards). A new wall with 26 bastions enclosed everything; Amstel locks (1673) and new gates (Leidsepoort, Utrechtsepoort, Weesperpoort, Muiderpoort). The Plantage was laid out but stayed gardens.
- Shrank or was destroyed: the Nieuwe Kerk burned in 1645 and was rebuilt on the same footprint; the medieval town hall burned in 1652 and was replaced on the same square by the new Stadhuis (1648-1665).
- Stayed fixed: Dam, Oude Kerk, Nieuwe Kerk, Westerkerk, Zuiderkerk, Waag, Damrak-Rokin, the entire 1613 canal ring and the Jordaan grid; Montelbaanstoren and Schreierstoren.
- Moved or vanished: the 1613 western wall segment south of Leidsegracht vanished into the extended canals; the eastern 1585 wall (Oudeschans line) became an inner canal; the Regulierspoort was replaced by the Munttoren (1620) at the same spot and the gate function moved out to the Utrechtsepoort.
- Rate: walled area from 440-460 ha to 700-730 ha (unverified), plus 60 percent in 60 years; the 1663 ring was about half built by 1688 (Plantage and eastern Prinsengracht stayed open).

**c. 1688 -> 1772 (de Wit -> Mol)**
- Grew: no new wall and no extension. Infill inside the 1663 wall only: eastern islands filled, Plantage still gardens.
- Shrank or was destroyed: no event; population stagnant at 200,000-210,000 across the whole 84 years (unverified).
- Stayed fixed: everything named above, plus the 26 bastions and all gates; Stadhuis on the Dam; Beurs of Hendrick de Keyser (1611) on the Rokin.
- Moved or vanished: nothing of city scale.
- Rate: walled area unchanged (700-730 ha); built area change not estimable from these plans.

**1772 -> 1839 (Mol -> Diederichs)**
- Grew: still inside the 1663 wall. New harbour works on the IJ side: Entrepotdok (1827), Oosterdok (1832) and Westerdok (1834) cut the IJ front with dikes; Plantage partly built.
- Shrank or was destroyed: the bastions were levelled into walks from the 1810s on; the Hendrick de Keyser Beurs on the Rokin was demolished in 1837 (the new Beurs by Zocher opened on the Dam in 1845, after this pair).
- Stayed fixed: Dam, Stadhuis (Royal Palace from 1808), Nieuwe Kerk, Oude Kerk, Westerkerk, Zuiderkerk, Noorderkerk, Waag, Munttoren, Montelbaanstoren, Schreierstoren, the complete canal ring and every street of 1688.
- Moved or vanished: bastions and most gate buildings; Haarlemmerpoort rebuilt as the Willemspoort (1840, next to its old site); the exchange moved from the Rokin to the Dam.
- Rate: walled area unchanged (700-730 ha); built area change not estimable from these plans.

### C. Closing line

Amsterdam shows that the fixed set is the Dam with its two churches and the Damrak-Rokin water spine: four extensions (1585, 1592, 1613, 1663) were all added outside it. The landmarks that moved were the gates, which moved outward with each new wall (Haarlemmerpoort and Regulierspoort between the 1560 and 1625 plans; the Regulierspoort's gate function to the Utrechtsepoort after 1663; the Willemspoort of 1840 beside the old Haarlemmerpoort site), and the exchange, which moved 300 m onto the Dam.

---

## 7. Vienna

### A. Plans

| Date depicted | Plan (author, date drawn) | Type | Pop. at that date | Plan size px | URL | Licence |
|---|---|---|---|---|---|---|
| 1547 | Bonifaz Wolmuet, measured plan 1547; Albert Camesina lithographic reproduction 1857 (the original is in the Wien Museum; the Commons scan is Camesina's) | survey (reproduction of a survey) | 20,000-50,000 (unverified) | 3840 x 3072 | https://commons.wikimedia.org/wiki/File:Map_Vienna_1547.jpg | Public domain (PD mark 1.0) |
| 1683 | Daniel Suttinger, plan of the city as in 1683, dated 11 December 1684; Albert Camesina lithograph 1876 (original lost) | survey (reproduction of a survey) | 80,000-100,000 city and suburbs before the siege (unverified) | 3840 x 3636 | https://commons.wikimedia.org/wiki/File:Wien_Plan-Suttinger-1683(1684).jpg | Public domain (PD mark 1.0) |
| 1683 (siege) | Giovanni Giacomo de Rossi, "Vienna assediata dalle Armi Ottomane gli 12 Luglio 1683", c. 1683 | bird's-eye siege plan | as above | 13843 x 10776 | https://commons.wikimedia.org/wiki/File:Vienna_assediata_dalle_Armi_Ottomane_gli_12_Lugliio_1683.jpg | Public domain (PD mark 1.0) |
| 1769-1773 | Joseph Daniel Huber, perspective plan of Vienna and suburbs to the Linienwall, drawn 1769-1773, printed 1778, 1:1440 | bird's-eye on a measured base | 200,000 (1777, unverified; 1754 census 175,000 unverified) | 3840 x 3333 | https://commons.wikimedia.org/wiki/File:Wien-1773_(1778)-Huber.jpg | Public domain (PD mark 1.0) |
| 1877-1878 | Artaria & Comp., "Orientirungs- und Strassen-Plan Wien mit Vororten", 1877/1878, c. 1:19,350 (BnF copy, sheet 1 of 2) | survey | 726,105 (1880 census, Vienna within the Linienwall) | 8609 x 8397 | https://commons.wikimedia.org/wiki/File:Orientirungs-und_Strassen-Plan_Wien_mit_Vororten._Original_-_Ausgabe_1877_-_1878_-_btv1b53102680r_(1_of_2).jpg | Public domain (PD-old, PD-1923) |

Also verified via the Commons API (file page not fetched): Third Military Survey sheet "Aufnahmeblatt 4757-1-a 1875 Wien Innenstadt", 9598 x 7433, PD, 1:12,500 (an alternative 1870s survey). Huber's plan at full resolution exists on Commons only as single sheets (example: "Wien Huber-Plan Blatt Q", 3018 x 3630, PD); the whole-plan file above is 3840 px wide.

### B. Between plans

**1547 -> 1683 (Wolmuet -> Suttinger)**
- Grew: not outward. The medieval wall was replaced by a bastioned trace (begun after the 1529 siege, 12 bastions by the 1670s) with a cleared glacis of 400-500 m. Hofburg enlarged (Amalienburg 1575-1611, Leopoldine wing 1660-1681). The suburbs outside the glacis grew between the plans but lie off the Wolmuet sheet.
- Shrank or was destroyed: in July 1683 the defenders burned the suburbs before the Ottoman siege; the de Rossi plan shows the glacis empty and the siege works. The walled city was not taken and its plan did not change.
- Stayed fixed: Stephansdom; Hofburg (Schweizerhof core); Graben, Kärntner Strasse, Rotenturmstrasse and Wollzeile as the spines; Hoher Markt, Am Hof, Neuer Markt; Schottenstift; Michaelerkirche; Rotenturm gate site; Kärntnertor site; the Danube arm (Donaukanal) on the north-east.
- Moved or vanished: the medieval wall and towers vanished into the bastioned trace, which sits on the same line; gates were rebuilt on the same axes.
- Rate: walled area 110-130 ha on both (unverified); built area inside the wall unchanged; suburbs reduced to zero in 1683.

**1683 -> 1769-1773 (Suttinger -> Huber)**
- Grew: outward, in all directions to the Linienwall (the outer earth rampart of 1704, 2-3 km from the centre): the burned suburbs were rebuilt and extended, with Karlskirche (1716-1737), Belvedere (1714-1723), Schwarzenberg and Liechtenstein palaces, Leopoldstadt on the island. Inside the walls: baroque rebuilding of nearly every block, Peterskirche (1701-1733), Hofbibliothek (1723-1726).
- Shrank or was destroyed: nothing after 1683; the plagues of 1679 and 1713 did not change the plan.
- Stayed fixed: Stephansdom, Hofburg, Graben, Kärntner Strasse, Rotenturmstrasse, Hoher Markt, Am Hof, Schottenstift, Michaelerkirche, every bastion and gate of 1683, the empty glacis.
- Moved or vanished: nothing inside the walls; the suburbs of 1683 were rebuilt on the same radial roads.
- Rate: area inside the Linienwall 5,000-6,000 ha (unverified); built suburbs from zero (1683) to a continuous ring, not estimable in hectares from these plans.

**1769-1773 -> 1877-1878 (Huber -> Artaria)**
- Grew: outward past the Linienwall into the Vororte (incorporated 1892, after this plan) and inward onto the glacis: bastions demolished from 1858, Ringstrasse opened 1865, with Opera (1869), Votivkirche (1879), Rathaus, Parliament, University and the two museums under construction or just finished. The Vorstädte became districts 2-9 in 1850. Danube regulated 1870-1875 into a straight bed north-east of the city; railways and stations (Südbahnhof, Westbahnhof, Nordbahnhof, Franz-Josefs-Bahnhof) at the Linienwall.
- Shrank or was destroyed: the whole bastioned fortress and glacis (100-130 ha of open ground, unverified) were removed; the 1848 fighting did not change the plan.
- Stayed fixed: Stephansdom, Hofburg, Graben, Kärntner Strasse, Rotenturmstrasse, Hoher Markt, Am Hof, Schottenstift, Karlskirche, Belvedere, all radial roads from the inner city to the Linienwall, the Linienwall itself (until 1894), the Donaukanal.
- Moved or vanished: bastions, gates and glacis replaced by the Ringstrasse (on the glacis, not on the wall line); the old Danube arms north of Leopoldstadt vanished into the regulated bed; the Wien river still open (covered 1895-1899, after this plan).
- Rate: built area at least tripled inside the Linienwall (unverified); population from 200,000 to 726,105, a factor of 3.6 in 105 years.

### C. Closing line

Vienna shows that a fortification ring can be deleted and replaced by a boulevard on the same band without moving a single landmark inside it: the fixed set is the cathedral, the palace, the two market squares and the radial roads, and the ring is the one layer that changed. The Ringstrasse lies on the glacis beside the old wall line, not on it. Proposal (Jeffrey decides): the generator can swap the ring layer without touching the fixed set.

---

## 8. Dublin

### A. Plans

| Date depicted | Plan (author, date drawn) | Type | Pop. at that date | Plan size px | URL | Licence |
|---|---|---|---|---|---|---|
| 1610 | John Speed, "The Countie of Leinster with the citie Dublin described", engraved Jodocus Hondius, 1610 (the Dublin plan is the inset on the county sheet; BnF copy) | bird's-eye (plan with building elevations) | 10,000-20,000 (unverified) | 6824 x 5288 | https://commons.wikimedia.org/wiki/File:The_Countie_of_Leinster_with_the_citie_Dublin_described_-_Performed_by_John_Speed_;_Iodocus_Hondius_caelavit_-_btv1b53056982f_(1_of_2).jpg | Public domain (PD-old-70, PD-1923) |
| 1610 (redrawn 1811) | "A Plan of Dublin in 1610", redrawing of Speed published 1811 | reconstruction | as above | 2085 x 1741 | https://commons.wikimedia.org/wiki/File:DUBLIN(1811)_p020_A_PLAN_OF_DUBLIN_IN_1610.jpg | Public domain |
| 1728 | Charles Brooking, "A Map of the City and Suburbs of Dublin", 1728 (RSAI lantern slide of the whole map, with the prospect at top) | survey (plan with a prospect) | 90,000-120,000 (unverified) | 7573 x 7559 | https://rsai.locloudhosting.net/items/show/24915 | Rights holder RSAI; no licence stated (reuse terms unverified) |
| 1756 | John Rocque, "An Exact Survey of the City and Suburbs of Dublin", engraved Andrew Dury, 1756, c. 1:2,570, 144 x 102 cm (BnF copy) | survey | 130,000-140,000 (unverified) | 17401 x 12416 | https://commons.wikimedia.org/wiki/File:An_Exact_Survey_of_the_City_and_Suburbs_of_Dublin_in_Which_is_Express'd_the_Ground_Plot_of_all_Publick_Buildings_Dwelling_Houses,_Ware_Houses,_Stables,_Court_Yards,_and_c_-_by_John_Rocque..._;_A._Dury_sculp.t..._-_btv1b530571173.jpg | Public domain (PD-old-70, PD-1923) |
| 1836 | W. B. Clarke, engraved E. Turrell, Society for the Diffusion of Useful Knowledge, 1836 (Boston Public Library copy) | survey | 204,155 (1831 census, city) to 232,726 (1841 census, city) | 3972 x 3346 | https://commons.wikimedia.org/wiki/File:1836_Dublin,_drawn_by_W.B._Clarke,_archt_;_engraved_by_E._Turrell_;_published_under_the_Superintendence_of_the_Society_for_the_Diffusion_of_Useful_Knowledge,_from_the_Digital_Commonwealth_-_commonwealth_ht2505534.jpg | Public domain (PD mark 1.0) |
| 1837 | Commissioners for Municipal Corporation Boundaries (Ireland), "Map of the City of Dublin", 1837 | survey | as above | 2934 x 2028 | https://commons.wikimedia.org/wiki/File:City_of_Dublin_1837_map.jpg | Public domain (UK Crown copyright expired) |

No 1840s-1860s Ordnance Survey or Thom's plan of the whole city was found on Wikimedia Commons; the 1836 SDUK plan and the 1837 boundary map are used for the fourth date. The 1847 OS 1:1056 sheets are at UCD Digital Library (digital.ucd.ie, CC BY-NC-SA 4.0, single sheets only, not verified).

Irish Historic Towns Atlas, Royal Irish Academy, digital editions (free PDFs; pages returned HTTP 200 with the expected titles; PDFs returned HTTP 200 application/pdf on HEAD; map content not opened, so "(unverified)"):
- Dublin part I, to 1610 (H. B. Clarke, 2002): https://www.ria.ie/irish-historic-towns-atlas/dublin-part-i-to-1610 ; Map 1-5 PDFs, for example https://s3-eu-west-1.amazonaws.com/assets.ria.ie/ihta/ihta-digital/viking-towns/dublin-part-i-1610/IHTA_11_Dublin+Part+1_Map3.pdf (3.0 MB).
- Dublin part II, 1610 to 1756 (Colm Lennon, 2008): https://www.ria.ie/irish-historic-towns-atlas/dublin-part-ii-1610-to-1756 ; Map 1, 2, 3, 4 and 17 PDFs, for example https://s3-eu-west-1.amazonaws.com/assets.ria.ie/ihta/ihta-digital/towns-in-18th-century/dublin+part+II/IHTA_19_Dublin_Part2_Map3.pdf (53.5 MB).
- Dublin part III, 1756 to 1847 (Rob Goodbody, 2014): https://www.ria.ie/irish-historic-towns-atlas/dublin-part-iii-1756-to-1847 ; Map 1-4 and 25 PDFs, for example https://s3-eu-west-1.amazonaws.com/assets.ria.ie/ihta/ihta-digital/towns-in-19th-century/Dublin+part+III/IHTA-26-Dublin-Part3-Map3.pdf (99.8 MB).
In the IHTA series convention Map 1 is the growth map by period and Map 3 is the principal historic plan redrawn at 1:2500 (unverified for these three parts). The RIA states that some historic maps and images in the print editions are left out of the digital editions for copyright reasons. Licence: the RIA pages give citation guidance per PDF; no open licence statement was found on the pages. Treat as free to read, reuse terms unverified.

### B. Between plans

**1610 -> 1728 (Speed -> Brooking)**
- Grew: east along both banks of the Liffey and north across it. South bank: Dame Street from the Castle to Trinity College (founded 1592, on Speed as a new building at the edge), St Stephen's Green laid out 1664, College Green, quays built along the whole south shore. North bank: Oxmantown Green, Smithfield (1660s), Royal Barracks (1704), Jervis estate (Capel Street, Jervis Street, 1670s-1690s), St Mary's church. Four new bridges: Bloody Bridge (1670), Essex Bridge (1676), Arran Bridge (1683), Ormond Bridge (1684). Royal Hospital Kilmainham (1680-1684) west. Custom House at Essex Bridge (1707).
- Shrank or was destroyed: the medieval city wall lost most of its circuit; the Castle burned in 1684 and was rebuilt on the same site; St Mary's Abbey (north bank) is on Speed as ruins and gone on Brooking.
- Stayed fixed: Dublin Castle (south-east corner of the old wall); Christ Church cathedral on the ridge; St Patrick's cathedral outside the wall to the south; St Audoen's; the one medieval bridge (Old Bridge at Bridge Street, 1428); the spine High Street - Castle Street - Dame Street east and Thomas Street - James's Street west; Cornmarket; St Werburgh's; the Poddle course.
- Moved or vanished: city gates (Newgate, St Nicholas's, Dame's Gate) demolished; the Thingmote mound (on Speed) levelled 1685; the wide tidal Liffey shore replaced by walled quays, so the river edge moved 50-150 m toward the river on both banks (unverified).
- Rate: walled city 18-22 ha (unverified); built area 1728 about 10 times the 1610 walled area (unverified).

**1728 -> 1756 (Brooking -> Rocque)**
- Grew: north-east (Gardiner estate: Henrietta Street 1720s-1740s, Sackville Mall and Marlborough Street; Rutland Square laid out just after 1756) and south-east (Dawson Street, Molesworth Street, Kildare House 1745-1748, the later Leinster House). Parliament House on College Green (1729-1739). City Basin (reservoir) south-west. Rocque's survey shows every building footprint; Brooking's does not.
- Shrank or was destroyed: nothing of plan scale.
- Stayed fixed: Castle, Christ Church, St Patrick's, Trinity College, the five bridges of 1728 (Old, Bloody, Arran, Ormond, Essex), Custom House at Essex Bridge, Royal Barracks, Royal Hospital, St Stephen's Green, Smithfield, Dame Street, Thomas Street, High Street, Capel Street.
- Moved or vanished: nothing; the Liberties (Earl of Meath's, St Sepulchre's) densified in place.
- Rate: built area plus 20-40 percent in 28 years (unverified).

**1756 -> 1836/1837 (Rocque -> SDUK and boundary map)**
- Grew: a ring. Circular Roads (North from 1763, South 1780s-1790s), Grand Canal (reached the Liffey 1796) and Royal Canal (1790-1817) enclose the city; Mountjoy Square (1790s), Merrion Square (from 1762), Fitzwilliam Square (1791-1830) fill the east; Wide Streets Commissioners (from 1757) cut Parliament Street (1762), widened Dame Street, extended Sackville Street to the river and built Westmoreland and D'Olier Streets (1800s). Carlisle Bridge (1794) and Richmond Bridge (1816); Whitworth Bridge replaced the Old Bridge on the same site (1816); Queen's Bridge replaced Arran Bridge on the same site (1768); the Four Courts (1786-1802, [Four Courts](https://en.wikipedia.org/wiki/Four_Courts)), King's Inns (from 1800), Dublin-Kingstown railway (1834) on the 1836 plan.
- Shrank or was destroyed: the 1707 Custom House was abandoned when Gandon's Custom House opened downstream in 1791; the Parliament House lost its function in 1800 (Bank of Ireland 1803), building unchanged; the medieval wall remnants mostly gone.
- Stayed fixed: Castle, Christ Church, St Patrick's, St Audoen's, Trinity College, Parliament House building, Royal Hospital, Royal Barracks, St Stephen's Green, the bridge sites at Bridge Street and Essex Street (bridges rebuilt in place), Dame Street, Thomas Street, Capel Street, Sackville Street, Henrietta Street, Smithfield.
- Moved or vanished: the Custom House moved 800 m downstream (the one major relocation in Dublin's history); Ormond Bridge collapsed in 1802 and was replaced by Richmond Bridge 100 m away (unverified distance); the port moved east with the new Custom House and docks.
- Rate: built area about doubled in 80 years (unverified); population from 130,000-140,000 to 232,726 (1841 census).

### C. Closing line

Dublin shows that bridges and the two cathedrals are the true pins: bridges were rebuilt twice on the same sites (Whitworth Bridge 1816, Queen's Bridge 1768), while the one landmark that moved, the Custom House, dragged the whole port and the city's centre of gravity 800 m east with it. Proposal (Jeffrey decides): treat a moved landmark as an event that re-weights growth, not as a cosmetic change.

---

## 9. Moscow

### A. Plans

| Date depicted | Plan (author, date drawn) | Type | Pop. at that date | Plan size px | URL | Licence |
|---|---|---|---|---|---|---|
| 1600-1610 | "Sigismund plan" (Moscovia urbs), drawn J. G. Abelin, engraved Lucas Kilian, 1610, dedicated to Sigismund III | bird's-eye | 100,000-200,000 (unverified) | 5636 x 4113 | https://commons.wikimedia.org/wiki/File:Sigismund%27s_plan_of_Moscow_1610.jpg | Public domain (PD mark 1.0) |
| 1739 | Ivan Michurin, "Plan imperatorskago stolichnago goroda Moskvy", first geodetic survey, 1739 (cleaned scan from etomesto.ru) | survey | 140,000 (unverified) | 8700 x 8300 | https://commons.wikimedia.org/wiki/File:Michurin_map_of_Moscow,_1739_(cleaned).jpg | Public domain (PD mark 1.0) |
| 1813 | Russian army topographers, plan of Moscow with the area burned in 1812 shaded, 1813 (from A. N. Bulgakov's book, 1813) | survey | 160,000-170,000 (post-fire, unverified) | 2300 x 1883 | https://commons.wikimedia.org/wiki/File:Plan_of_Moscow_1813.jpg | Public domain (PD mark 1.0) |
| 1818 | Egor Cheliev for the Commission for Buildings in Moscow, "Prozhektirovannyi plan stolichnago goroda Moskvy", engraved A. Florov, 1818, 1:12,600 (the projected post-fire plan that followed the Commission's 1817 survey) | survey plus projected street lines | 180,000-200,000 (unverified) | 31729 x 29980 | https://commons.wikimedia.org/wiki/File:1818_moscow-cheliev.jpg | Public domain (PD mark 1.0, PD-old-100) |
| 1874-1886 (printed 1888) | N. N. Smirnov and D. P. Roshkov, "Nivelirnyi plan goroda Moskvy", 2nd edition 1888, Moscow City Duma, 6 sheets (triangulation 1874-1877, surveys to 1886) | survey | 753,459 (1882 census) | 29749 x 30000 | https://commons.wikimedia.org/wiki/File:Leveling_plan_of_Moscow,_1888.jpg | Public domain (PD-Russia) |

Note on the Commons 1888 file: its Date field says "1988"; the title, description and source say 1888. Also found via the Commons API (file page not fetched): Michurin plan as printed in the Atlas Russicus 1745 (Library of Congress copy), 6689 x 6451, PD, https://commons.wikimedia.org/wiki/File:Plan_imperatorskago_stolichnago_goroda_Moskvy._LOC_77695545.jpg (unverified). The 1817 Commission plan itself exists on Commons only as a 486 x 636 fragment (File:Map of Moscow 1817. Fragment1.jpg) and a 1000 x 786 fire-damage map signed 27 May 1817 (File:Fire of moscow 1812 1000px.jpg); the 1818 Cheliev plan is used as the full-size post-fire plan.

### B. Between plans

**1600-1610 -> 1739 (Sigismund -> Michurin)**
- Grew: the four concentric rings are complete on both: Kremlin, Kitai-gorod (brick wall 1535-1538), Bely Gorod (stone wall 1586-1593, 10 km), Zemlyanoy Gorod (earth rampart with timber wall 1591-1592, 16 km, including Zamoskvorechye south of the river). Between the plans the suburbs (slobody) spread beyond the earth rampart on all sides; the German Quarter on the Yauza and the Preobrazhenskoye and Lefortovo quarters appear on Michurin. The Kamer-Kollezhsky customs rampart (1742, 37 km) was built three years after Michurin.
- Shrank or was destroyed: the Sigismund plan depicts the city before the fire of 1611 (set by the Polish garrison), which destroyed everything outside the Kremlin and Kitai-gorod; the city was rebuilt on the same radial-ring plan. Fires of 1626, 1648 and 1737 (the last one two years before Michurin) each burned large parts; none changed the street net.
- Stayed fixed: Kremlin and its walls and towers; Red Square; St Basil's (1555-1561); Ivan the Great bell tower; Kitai-gorod wall and gates; Bely Gorod wall line and its gates (Tverskie, Nikitskie, Arbatskie, Prechistenskie, Sretenskie, Pokrovskie); the radial roads Tverskaya, Nikitskaya, Arbat, Prechistenka, Sretenka, Pokrovka, Ordynka; the Neglinnaya river open along the Kremlin's west wall; the Moskva and Yauza courses.
- Moved or vanished: the timber wall on the earth rampart (Skorodom) burned in 1611 and was not rebuilt; the earth rampart stayed.
- Rate: area inside the earth rampart 1,800-1,900 ha on both (unverified); built area outside it grew from scattered villages to continuous slobody, not estimable in hectares from these plans.

**1739 -> 1813/1818 (Michurin -> fire map and Cheliev)**
- Grew: outward to the Kamer-Kollezhsky rampart (1742), which became the city limit. Inside: Bely Gorod wall demolished 1770s-1790s and its line turned into the Boulevard Ring (begun 1796, finished 1820s); Vodootvodny canal (1783-1786) dug south of the Kremlin; Pashkov House (1784-1786), Moscow University (1786-1793), Foundling Home (1764-1781).
- Shrank or was destroyed: the fire of 2-6 September 1812 destroyed about two thirds of the built city (6,000-6,500 of 9,000 houses, unverified); the 1813 plan shows the burned area shaded, covering most of the Bely Gorod and Zemlyanoy Gorod and all of Zamoskvorechye, with the Kremlin, Kitai-gorod and the north-west sector least damaged. Population fell from 270,000 (1811, unverified) to 160,000-170,000 (1813, unverified).
- Stayed fixed: Kremlin, Red Square, St Basil's, Kitai-gorod wall, all radial roads, the Boulevard Ring on the Bely Gorod line, the Zemlyanoy rampart line, the Moskva, Yauza and Vodootvodny courses. The 1818 projected plan keeps every pre-fire street and straightens only block edges and squares.
- Moved or vanished: Bely Gorod wall vanished into boulevards; the 1818 plan projects the Zemlyanoy rampart into the Garden Ring (levelled 1816-1830), the Neglinnaya into a culvert (1817-1819, Alexander Garden over it), the Kremlin moat and the shops on Red Square removed (1813-1818), Theatre Square cleared (1818-1824). Kitai-gorod wall kept (until 1934).
- Rate: area inside the city limit from 1,800-1,900 ha (earth rampart) to 7,000-7,200 ha (Kamer-Kollezhsky rampart) (unverified); built area in 1813 about one third of 1811, rebuilt by 1818-1825 to the same extent (unverified).

**1818 -> 1874-1886 (Cheliev -> Smirnov and Roshkov)**
- Grew: fill-in to the Kamer-Kollezhsky rampart and past it on the railway sides. Nine railway termini at or outside the Garden Ring (Nikolaevsky 1851, Yaroslavsky 1862, Kursky 1866, Brestsky 1870, and others); factory districts on the Yauza and south-east; Cathedral of Christ the Saviour (1839-1883) on the Moskva west of the Kremlin; Historical Museum (1875-1883) closing the north end of Red Square; Upper Trading Rows under construction (1890-1893, after this plan).
- Shrank or was destroyed: nothing of plan scale; the Alekseyevsky convent was demolished for the Cathedral of Christ the Saviour (same site reused).
- Stayed fixed: Kremlin, Red Square, St Basil's, Kitai-gorod wall, Boulevard Ring, Garden Ring (now fully built as the ring road the 1818 plan projected), every radial road of 1610, Tverskaya as the main spine north-west, the Moskva, Yauza and Vodootvodny courses, Theatre Square with the Bolshoi (1825, rebuilt after the 1853 fire on the same footprint).
- Moved or vanished: the Zemlyanoy rampart itself (gone by 1830); the Kamer-Kollezhsky rampart reduced to a customs line, then a street ring; Neglinnaya fully underground.
- Rate: built area about tripled in 70 years (unverified); population from 180,000-200,000 to 753,459 (1882 census), a factor of 4.

### C. Closing line

Moscow shows that the radial-ring skeleton outlives everything built on it: two city-wide fires (1611, everything outside the Kremlin and Kitai-gorod; 1812, about two thirds of the built city) and the demolition of two of the four wall rings left every radial road and the Kremlin-Red Square core in place, and each removed wall became a ring road on the same line, so the generator should pin the ring lines as geometry even when the wall objects are deleted.

---

## 10. Mexico City (Tenochtitlan)

### A. Plans

| Date depicted | Plan (author, date drawn) | Type | Pop. at that date | Plan size px | URL | Licence |
|---|---|---|---|---|---|---|
| 1519-1520 | "Nuremberg map", woodcut printed by Friedrich Peypus with the Latin Cortés letters, Nuremberg 1524 | Bird's-eye, schematic (after an indigenous or Spanish sketch) | 200,000-400,000 (Wikipedia, Tenochtitlan; one survey gives 212,500) | 3,029 x 1,931 | https://commons.wikimedia.org/wiki/File:Map_of_Tenochtitlan,_1524.jpg | Public domain (PD-old-100, PD-US); CC Public Domain Mark 1.0 |
| 1550-1556 | "Mapa de Uppsala" / Santa Cruz map, Tlatelolco painters, c. 1550-1556 | Bird's-eye pictorial | 20,000-40,000 total; 22,000 indigenous counted in 1525 (unverified for 1550) | 2,000 x 4,160 | https://commons.wikimedia.org/wiki/File:Santa_Cruz_Map.jpg | Host states CC BY-SA 4.0 (uploader's claim on a scan of a 16th-century public-domain painting) |
| 1628 | "Forma y levantado de la ciudad de México", Juan Gómez de Trasmonte, 1628 (file is the 1907 Ruffoni lithograph reissue) | Bird's-eye | 30,000-60,000 (unverified; 57,000 in 1689 per Wikipedia, History of Mexico City) | 14,592 x 11,612 | https://commons.wikimedia.org/wiki/File:Forma_y_Levantado_de_La_Ciudad_de_Mexico,_1628,_por_Juan_Gomez_de_Trasmonte.jpg | Public domain (PD-old-assumed, PD-US-expired); CC Public Domain Mark 1.0 |
| 1793 | "Plan general de la Ciudad de México", surveyed by Diego García Conde 1793, engraved Fabregat 1807; file is the Edward Mogg reduction, London 1811 | Survey | 104,760 excluding officials (1793 count); 112,926 in the 1790 Revillagigedo census (Wikipedia, History of Mexico City) | 14,414 x 12,992 | https://commons.wikimedia.org/wiki/File:Plan_general_de_la_Ciudad_de_Mexico_-_Levantado_por_el_Teniente_Coronel_Don_Diego_Garcia_Conde,_en_el_ano_de_1793._Gravado_(IA_dr_plan-general-de-la-ciudad-de-mexico-levantado-por-el-teniente-coronel-don-11419000).jpg | Public domain (PD-US-expired, PD-old-100); CC Public Domain Mark 1.0; Rumsey permission note says CC BY-NC-SA 3.0 and contact for commercial use |
| 1863 | "Plan de Mexico et de ses environs", Nouvelles Annales de la géographie (V.-A. Malte-Brun ed.), Paris 1863 | Survey-derived printed plan | 200,000 (unverified) | 6,955 x 10,314 | https://commons.wikimedia.org/wiki/File:Plan_de_Mexico_et_de_ses_environs_(34458777021).jpg | "No known copyright restrictions" (Flickr Commons, Université de Caen) |
| 1884 | "General Plan of Mexico City", unidentified author, 1884; file is a 2024 photograph of the sheet at Museo Soumaya | Survey | 300,000 (unverified) | 3,372 x 2,532 | https://commons.wikimedia.org/wiki/File:'General_Plan_of_Mexico_City'_(1884)_by_unidentified_artist_-_Museo_Soumaya_-_Mexico_2024.jpg | CC BY-SA 4.0 (photographer's licence) |

### B. Between plans

**1520 (Nuremberg) to 1550 (Uppsala)**
- Grew: nothing; the Spanish traza of 1522-1524 was laid as a rectangular grid of 180 ha in 100 blocks (Wikipedia, History of Mexico City) over the centre of the razed island city, with indigenous barrios around it.
- Shrank or was destroyed: the siege of 1521 razed the city; the Templo Mayor and the sacred precinct (a walled square 500 m on a side) were demolished and their stone reused; population fell from 200,000-400,000 to the 22,000 indigenous counted in 1525.
- Stayed fixed: the island site; the three causeways (Tepeyac to the north, Tlacopan/Tacuba to the west, Iztapalapa to the south) became the Spanish city's three main streets and are still streets; the central plaza of the sacred precinct became the Plaza Mayor (Zócalo); the cathedral was built on the Templo Mayor side of the plaza; Moctezuma's "New Houses" became the Cortés/viceregal palace (National Palace site); the Chapultepec aqueduct line along the Tacuba causeway; Tlatelolco and its market to the north.
- Moved or vanished: the twin temples and all pyramids; the lakeside chinampa fringe on the Uppsala map is already reduced; the Aztec dike of Nezahualcóyotl survives only as a line.
- Rate: Tenochtitlan 800-1,350 ha (Wikipedia gives 8-13.5 km²) to a Spanish traza of 180 ha plus barrios: a shrink of 75-85 percent of built area in 30 years (unverified; the Uppsala map is not to scale).

**1550 (Uppsala) to 1628 (Trasmonte)**
- Grew: the traza filled with masonry; the Alameda park laid out 1592 at the west edge; convents (San Francisco, Santo Domingo, San Agustín, La Merced) occupy whole blocks; the Desagüe drainage cut started in 1607 with 4,500 workers.
- Shrank or was destroyed: no destruction between the two dates; the flood of 1629 (one year after the plan) kept the city under water for several years and a move of the capital was debated in 1630 and rejected.
- Stayed fixed: Plaza Mayor, cathedral (new foundation on the same site 1562-1573), palace, the Tacuba, Tepeyac and Iztapalapa axes, Tlatelolco church, the Alameda once laid out, the Acequia Real canal into the plaza.
- Moved or vanished: the lakeshore retreated east of the city; most of the island's secondary canals were filled; the city reads as a grid on dry land with water only on the east.
- Rate: not estimable from these plans (Trasmonte is a bird's-eye without scale).

**1628 (Trasmonte) to 1793 (García Conde)**
- Grew: west and south-west: Paseo de Bucareli (1775-1778) outside the Alameda; the city divided into 8 cuarteles mayores by Revillagigedo's 1782 ordinances; the García Conde sheet shows a built rectangle of about 3 km by 2.5 km with suburbs along the causeways.
- Shrank or was destroyed: the 1629-1634 flood emptied blocks temporarily; no permanent loss.
- Stayed fixed: Plaza Mayor, cathedral (dedicated 1656, towers finished 1813), National Palace, Alameda, Tacuba street, Calzada de Tlalpan (Iztapalapa causeway), Calzada de los Misterios (Tepeyac causeway), the big convents on their 1550 blocks, Tlatelolco.
- Moved or vanished: Lake Texcoco's shore now 2-3 km east of the plaza; the Acequia Real is one of the last open canals.
- Rate: population 57,000 (1689) to 105,000-113,000 (1790-1793); built area change not estimable from these plans.

**1793 (García Conde) to 1863 (Malte-Brun)**
- Grew: little outside the 1793 rectangle; the Buenavista railway station (Mexico-Veracruz line, opened in stages from 1857) at the north-west; first colonias (Santa María la Ribera, 1859) platted to the north-west.
- Shrank or was destroyed: the 1861 Nationalisation of Church Property closed every convent; convent blocks were cut through by new streets (Calle 5 de Mayo and Independencia through San Francisco; Calle 16 de Septiembre); the US occupation of 1847 caused no demolition of the plan.
- Stayed fixed: every item in the previous "fixed" list; the eight cuarteles; the causeway axes; the garitas (customs gates) at the city edge.
- Moved or vanished: convent precincts subdivided; the Paseo de la Reforma (cut 1864-1866 from the Alameda to Chapultepec) is not yet on the 1863 sheet.
- Rate: not estimable from these plans; population 105,000 (1793) to 200,000 (1863, unverified).

**1863 (Malte-Brun) to 1884 (Museo Soumaya plan)**
- Grew: west and north-west along Paseo de la Reforma toward Chapultepec: colonias Guerrero (1874), Arquitectos, Santa María la Ribera filling; railway yards at Buenavista and San Lázaro; mule-tram lines along the causeways.
- Shrank or was destroyed: nothing.
- Stayed fixed: Zócalo, cathedral, National Palace, Alameda, the three causeway axes, the convent churches that survived as parish churches on their original sites.
- Moved or vanished: the last garitas lost their function; Lake Texcoco shore now 5 km east.
- Rate: built area grew 20-40 percent in 21 years (unverified; estimated from the two sheets' extents).

### C. Closing line
Mexico City shows that the plaza, the palace on the palace site and the three causeway streets have not moved in 500 years across conquest, a five-year flood and the removal of the lake; the Templo Mayor was demolished in 1521 and the cathedral was built on the Templo Mayor side of the plaza, so the main religious site shifted within the plaza rather than staying on its footprint. Proposal (Jeffrey decides): pin the civic core and the spine roads, and treat the shoreline as the thing that moves.

---

## 11. Edo / Tokyo

### A. Plans

| Date depicted | Plan (author, date drawn) | Type | Pop. at that date | Plan size px | URL | Licence |
|---|---|---|---|---|---|---|
| c. 1630-1631 | "Bushū Toshima-gōri Edo-no-shō zu", cadastral plan of central Edo, c. 1630-1631; file is the 1682 woodblock reprint (Library of Congress / WDL 9931), scale c. 1:7,500 | Survey (cadastral) | 150,000-300,000 (unverified; Wikipedia, Edo, gives "around 300,000" at 1657 with citation needed) | 1,390 x 1,024 | https://commons.wikimedia.org/wiki/File:Map_of_Bush%C5%AB_Toshima_District,_Edo_WDL9931.png | Public domain (PD-old-100); CC Public Domain Mark 1.0 |
| 1748 | "Bunken Enkyō Edo ōezu" (分間延享江戸大絵図), Ishikawa Ryūsen, Edo 1748 (Enkyō 5), one sheet 96.3 x 129.6 cm, National Diet Library | Survey (bunken = measured plan) | 1,000,000 (Wikipedia, Edo, estimate for 1721) | 11,318 x 8,812 (PDF, 3 pages) | https://commons.wikimedia.org/wiki/File:NDL2542456_%E5%88%86%E9%96%93%E5%BB%B6%E4%BA%AB%E6%B1%9F%E6%88%B8%E5%A4%A7%E7%B5%B5%E5%9B%B3.pdf | PD-Japan, PD-scan |
| c. 1855 | "Ansei kaisei Fukyō Oedo ezu" (安政改正府鄕御江戶繪圖), Takashiba San'yu, c. 1855 | Survey-based printed whole-city map | 1,000,000-1,200,000 (unverified) | 8,999 x 8,571 | https://commons.wikimedia.org/wiki/File:Sketch_Map_of_the_City_and_Villages_of_Edo,_Ansei_Revised_Edition.jpg | Public domain (PD-old-70, PD-US-expired); CC Public Domain Mark 1.0 |
| 1844-1848 (alternate for the 1850s slot) | Edo map, Kōka era revision, ed. Takai Ranzan, University of Texas scan | Survey-based printed whole-city map | 1,000,000-1,200,000 (unverified) | 5,625 x 4,769 | https://commons.wikimedia.org/wiki/File:Edo_1844-1848_Map.jpg | Public domain (PD-old-100); CC Public Domain Mark 1.0 |
| 1880 | Woodblock map of Tokyo, Meiji 13, Geographicus scan | Survey-based printed map | 1,000,000-1,200,000 (unverified) | 4,000 x 2,899 | https://commons.wikimedia.org/wiki/File:1880_Japanese_Meiji_Woodblock_Map_of_Tokyo_(EDO)_-_Geographicus_-_Tokyo-meiji-1880.jpg | Public domain (PD-US-expired); CC Public Domain Mark 1.0 |
| 1886 | "Revised New Tokyo Map (vertical projection)", Meiji 19, pocket map, Geographicus scan | Survey-based printed map with ward colouring and statistics table | 1,200,000-1,400,000 (unverified; Tokyo City 1889 census figure 1,389,684 is unverified) | 3,616 x 5,000 | https://commons.wikimedia.org/wiki/File:1886_Meiji_19_Japanese_Map_of_Tokyo,_Japan_-_Geographicus_-_Tokyo-meiji19-1886.jpg | Public domain (PD-Japan, PD-US-expired) |

The 1630s plan is the oldest surviving measured plan of the city; Commons holds it at 1,390 x 1,024 px only. A 1636 "Kan'ei 13 Edo ōezu" exists as an NDL scan (11,193 x 10,010 px, 6 pages) at https://commons.wikimedia.org/wiki/Category:%E5%AF%9B%E6%B0%B8%E5%8D%81%E4%B8%89%E5%B9%B4%E6%B1%9F%E6%88%B8%E5%A4%A7%E7%B5%B5%E5%9B%B3 but the category dates the print to 1703 and the sheet's content date could not be confirmed; it is listed as (unverified) and not used below.

### B. Between plans

**c. 1630 (Bushū Toshima) to 1748 (Enkyō ōezu)**
- Grew: in every direction, above all east across the Sumida (Honjo, Fukagawa) after Ryōgoku bridge (1659); north to Asakusa and Yoshiwara (moved there 1657); south along the Tōkaidō to Shinagawa; the Hibiya inlet filled and Tsukiji reclaimed; the outer moat ring completed (Kanda river cut, 1620s); daimyō middle and lower residences pushed to the ring between the two moats.
- Shrank or was destroyed: the Meireki fire of 1657 burnt "a vast portion of the city" with an estimated 100,000 dead; the castle keep (tenshu) burnt and was never rebuilt; the rebuilding added firebreaks (hirokōji) and wider streets and moved temples and residences outward.
- Stayed fixed: Edo Castle and its inner and outer moat spiral with its named gates (Ōtemon, Sakurada, Hanzō, Tokiwabashi, Sujikai); Nihonbashi bridge as the zero point of the Gokaidō; the Sumida river line; Zōjō-ji at Shiba and Kan'ei-ji at Ueno; Sensō-ji at Asakusa; the Tōkaidō, Nakasendō and Ōshū kaidō spines; the merchant quarter (shitamachi) east of the castle and the samurai hills (yamanote) west.
- Moved or vanished: the keep; Yoshiwara (from near Nihonbashi to beyond Asakusa); many daimyō main residences shifted to outside the inner moat; the shoreline advanced into the bay.
- Rate: the 1630 cadastral sheet covers the castle-town core only (under 1,000 ha); by 1748 the city covered 6,000-8,000 ha with a population of 1,000,000 (unverified area; the plans are at different scales).

**1748 (Enkyō ōezu) to c. 1855 (Ansei ezu)**
- Grew: little in extent; the shogunate fixed the city boundary (shubiki) in 1818; infill and densification; new temple quarters at the margins.
- Shrank or was destroyed: repeated fires (Bunsei 1829, Kōka 1846) each burned tens of thousands of houses and were rebuilt on the same plots; the Ansei earthquake of 11 November 1855 ([1855 Edo earthquake](https://en.wikipedia.org/wiki/1855_Edo_earthquake)) and its fires (7,000-10,000 dead, unverified) fall at or just after the plan's date.
- Stayed fixed: every castle gate and moat; Nihonbashi; Ryōgoku and the other Sumida bridges; the daimyō upper residences stand on the same plots identified by the same crests on both sheets; the temples; the five highways.
- Moved or vanished: nothing of note at city scale; plot-level changes of ownership only.
- Rate: built area change 0-10 percent in 107 years (unverified); growth is in density, not extent.

**c. 1855 (Ansei ezu) to 1886 (Meiji 19 map)**
- Grew: the city boundary is unchanged, but function changed: Shimbashi station and the Yokohama railway (1872), Ueno station (1883), Ginza rebuilt in brick after the 1872 fire, the 15 wards of Tokyo City (1878), Hibiya parade ground and the first Marunouchi offices on former daimyō land, Ueno Park (1873) on the burnt Kan'ei-ji precinct.
- Shrank or was destroyed: the 1868 Battle of Ueno burnt Kan'ei-ji; the Meiji government demolished most daimyō residences, leaving the samurai hills temporarily empty (population fell to 500,000-600,000 around 1870, unverified, then recovered).
- Stayed fixed: the castle became the Imperial Palace on the same site with the same moats and gates; Nihonbashi; the Sumida bridges; Sensō-ji, Zōjō-ji, Kanda Myōjin; the highway spines; the shitamachi merchant blocks with their Edo-period street lines.
- Moved or vanished: the daimyō crests disappear from the map and ward names replace them; the inner-moat segments begin to be filled (most filling is after 1900); Kan'ei-ji's main hall gone.
- Rate: extent of the 15 wards 8,000-8,500 ha (unverified), about the same as the 1818 shubiki line; change is in land use, not area.

### C. Closing line
Edo shows that a fortification ring (the moat spiral with its gates) and one road zero-point (Nihonbashi) stay fixed across a fire that destroyed most of the city and a change of regime that replaced every landowner around them: the generator should pin the defensive ring and the spine origin, and let ownership and building stock regenerate freely inside.

---

## 12. Ayutthaya (the regress case)

### A. Plans

| Date depicted | Plan (author, date drawn) | Type | Pop. at that date | Plan size px | URL | Licence |
|---|---|---|---|---|---|---|
| 1690 | Engelbert Kaempfer, working sketch of the city made on the spot in 1690, redrawn as the "improved" plan in the Comprehensive Atlas of the VOC vol. VII (Nationaal Archief), scale c. 1:24,000, north up | Survey (compass sketch, redrawn) | 150,000-200,000 city (unverified) | 4,461 x 3,174 | https://commons.wikimedia.org/wiki/File:Ayutthaya_City_Plan_Improved_Kaempfer_1690.png | Public domain (PD-old-70); CC Public Domain Mark 1.0 |
| 1687 | Simon de La Loubère, "Du Royaume de Siam", Paris 1691, after the survey by the French engineer de Lamare of 1687; engraved by Michault | Survey | 150,000-200,000 city (unverified) | 1,500 x 1,371 | https://commons.wikimedia.org/wiki/File:Ayutthaya_City_Map_(Siam)_by_Loubere_1691.jpg | Public domain (PD-old-100, PD-US-expired); CC Public Domain Mark 1.0 |
| c. 1665 (published 1726) | François Valentijn, "Judia, de Hoofd-Stad van Siam", Oud en Nieuw Oost-Indiën, 1726; copied after Johannes Vingboons 1665 (ayutthaya-history.com, "Mapping Iudea") | Bird's-eye, derivative; shape distorted | 150,000-200,000 city (unverified) | 4,328 x 3,305 | https://commons.wikimedia.org/wiki/File:Map_of_Ayutthaya_(Judia)_Golden_Age_by_Francois_Valentijn_1726.png | Public domain (PD-old-70); CC Public Domain Mark 1.0 |
| 1926 | Phraya Boran Rachathanin, "Phaen Thi Krung Sri Ayutthaya", archaeological plan of the ruin, 1926 (National Archives of Thailand); coloured version | Survey of the ruin | under 10,000 in the island town (unverified) | 7,094 x 5,516 | https://commons.wikimedia.org/wiki/File:Ayutthaya_City_Map_by_Phraya_Boran_Rachathanin_1926.jpg | Public domain (PD-old-70); CC Public Domain Mark 1.0 |

The Kaempfer and La Loubère plans are 3 years apart and are treated as one state below. Order used for the blocks: 1665 (Valentijn/Vingboons) to 1687-1690 (La Loubère, Kaempfer) to 1926 (Boran).

### B. Between plans

**c. 1665 (Valentijn after Vingboons) to 1687-1690 (La Loubère, Kaempfer)**
- Grew: the two surveys show foreign quarters that the Vingboons view draws schematically: the Portuguese settlement on the south bank, the Dutch lodge on the south-east bank of the Pa Sak, the Japanese village south of it, the French seminary (St Joseph) to the south-west, the Chinese quarter inside the south-east of the island; Kaempfer draws the fortified wall with its forts and water gates.
- Shrank or was destroyed: nothing; the city was at its peak.
- Stayed fixed: the island ringed by the Lopburi river (north and west), the Pa Sak (east) and the Chao Phraya (south); the brick wall along the island edge; the Grand Palace in the north-west corner of the island with Wat Phra Si Sanphet against it; the front palace (Chantharakasem) at the north-east corner; Wat Mahathat and Wat Ratchaburana in the island centre east of the palace; Wat Phanan Choeng across the river at the south-east; the north-south canals across the island (Khlong Pratu Khao Pluak, Khlong Makham Riang) and the east-west Khlong Pratu Thep Mi; one bridge (Thamnop Ro) to the mainland.
- Moved or vanished: nothing real moved; the Vingboons view distorts the island to a near circle, so positions are topological only.
- Rate: not estimable from these plans (the earlier one is a distorted view).

**1687-1690 (La Loubère, Kaempfer) to 1926 (Boran)**
- Grew: a new small market town (Hua Ro) in the north-east corner of the island and ribbon settlement along the Pa Sak and the Lopburi; the provincial government compound on the Chantharakasem site; the Bangkok-Ayutthaya railway (1896) on the mainland east of the island.
- Shrank or was destroyed: the Burmese siege of 14 months ended 7 April 1767; the city was burnt, the palace and the royal wats looted and their Buddha images melted; the brick wall was pulled down in the 1780s and its bricks shipped to build the walls of Bangkok (unverified, standard account); the population was deported or fled; the capital moved to Thonburi (1767) and Bangkok (1782).
- Stayed fixed: the river ring and the canal lines; every masonry wat and chedi, which Boran maps at their 17th-century positions (Wat Phra Si Sanphet, Wat Mahathat, Wat Ratchaburana, Wat Phra Ram, Wat Chai Watthanaram, Wat Yai Chai Mongkhon, Wat Phanan Choeng); the Grand Palace platform; the line of the city wall as a trace; the foreign-settlement sites survive only as church and shrine foundations (St Joseph, the Japanese village, Baan Hollanda).
- Moved or vanished: the wall (removed); all timber housing, the floating markets and the bazaar street; the royal quarter functions (court, mint, arsenals) gone; the market moved from the centre and south to the north-east corner.
- Rate: the island is 400-700 ha (unverified, 4 km by 2 km at the widest); built-up occupation fell from the whole island plus suburbs to 10-20 percent of the island in 1926 (unverified, estimated from the Boran sheet); population from 150,000-200,000 to under 10,000.

### C. Closing line
Ayutthaya shows that after total destruction and 160 years of abandonment the hydrology (river ring, canals) and every masonry religious or palace core are still in place while all housing, walls and markets are gone: Proposal (Jeffrey decides): the pins are water lines and stone monuments; housing, markets and timber walls are fully regenerable, and a "regress" state keeps the pins as ruins.

---

## 13. Timgad / Thamugadi (the grew-then-abandoned case)

### A. Plans

| Date depicted | Plan (author, date drawn) | Type | Pop. at that date | Plan size px | URL | Licence |
|---|---|---|---|---|---|---|
| 100 CE founding square | Plan of Timgad in F. Haverfield, Ancient Town Planning, Oxford 1913 (redrawn from Ballu's survey; Wellcome M0009552) | Reconstruction of the founding grid from the excavated ruin | 3,000-4,000 at foundation (French Wikipedia, Timgad, citing 200-900 veterans) | 3,425 x 3,100 | https://commons.wikimedia.org/wiki/File:Plan_of_Timgad_in_Haverfield's_Ancient_Town_Planning_Wellcome_M0009552.jpg | CC BY 4.0 (Wellcome) |
| Ruin as excavated to 1902 | Albert Ballu, "Plan général", in Théâtre et forum de Timgad, Paris 1902, p. 39 | Survey of the excavated ruin | 15,000 at the city's peak, early 3rd century (French Wikipedia, citing C. Courtois) | 3,134 x 2,073 | https://commons.wikimedia.org/wiki/File:Timgad_-_Plan_General_(page_39_crop).jpg | Public domain (PD-old-70); CC Public Domain Mark 1.0 |
| Ruin as excavated to 1904 (full city with suburbs) | "Plan des ruines de Timgad en 1904", Pl. XLII in Boeswillwald, Cagnat and Ballu, Timgad, une cité africaine sous l'empire romain, Paris 1905; two halves | Survey of the excavated ruin | 15,000 at peak (as above) | left 2,503 x 3,293; right 2,551 x 3,230 | https://commons.wikimedia.org/wiki/File:Timgad,_une_cit%C3%A9_africaine_sous_l'empire_romain_-_Pl._XLII_-_Plan_des_ruines_de_Timgad_en_1904_(partie_gauche).jpg and https://commons.wikimedia.org/wiki/File:Timgad,_une_cit%C3%A9_africaine_sous_l'empire_romain_-_Pl._XLII_-_Plan_des_ruines_de_Timgad_en_1904_(partie_droite).jpg | Public domain (PD-1923, US) |
| Ruin in 1927 (after the Godet excavations of the fort and south) | "Timgad-Plan, nach Guide bleu", lantern slide, ETH-Bibliothek Zürich, 1927 | Survey (tourist guide plan of the whole ruin) | 0 (uninhabited ruin) | 3,628 x 2,866 | https://commons.wikimedia.org/wiki/File:ETH-BIB-Timgad-Plan,_nach_Guide_bleu-Dia_247-Z-00107.tif | CC BY-SA 4.0 |
| 539 CE Byzantine fort | "Plan de la forteresse de Timgad", SVG by Azzedine Chergui, 2026, after the UNESCO site maps (which derive from Jean Lassus, La forteresse byzantine de Thamugadi, 1981) | Reconstruction | garrison in the hundreds (unverified) | 512 x 512 nominal (SVG) | https://commons.wikimedia.org/wiki/File:Timgad_Fortress_plan_fr.svg | CC BY-SA 1.0 |
| Ruin as mapped after the 1880-1962 excavations | "Carte des vestiges archéologiques à Timgad", SVG by Lofo7, 2008 | Reconstruction (modern synthesis) | 0 | 1,150 x 1,430 nominal (SVG) | https://commons.wikimedia.org/wiki/File:Timgad_archaeological_sites_map-fr.svg | GFDL 1.2+ and CC BY-SA 4.0/3.0/2.5/2.0/1.0 |

Excavation history (French Wikipedia, Timgad): first reports 1765 (Bruce) and 1851 (Renier); systematic excavation from 1880 under Duthoit, then Albert Ballu from 1889 for 38 years; Charles Godet (d. 1945) on the fort and the southern necropolis for about thirty years; Jean Lassus studied the fort (report 1955, published 1981); no excavation since 1962.

### B. Between plans

**100 CE founding square (Haverfield) to the full city at its 3rd-century peak (1904 plan of the ruin)**
- Grew: in every direction but mainly west and south-west: the Capitol built outside the original wall, the Sertius market, the great baths north and south, the Temple of the Genius of the Colony marking expansion past the enclosure, later Christian basilicas; the suburbs are loosely organised, not on the grid.
- Shrank or was destroyed: nothing between these two states.
- Stayed fixed: the decumanus maximus and the cardo; the forum at their junction; the theatre south of the forum; the Arch of Trajan on the decumanus at the west edge of the founding square; the north and east gates; the grid of 132 insulae of about 400 m² (French Wikipedia; Haverfield's plan shows the founding square of about 355 m on a side, unverified).
- Moved or vanished: the original enclosure wall was dismantled and built over as the city spread; the west gate became an internal arch.
- Rate: 12 ha at foundation to more than 90 ha (French Wikipedia intro; the same article elsewhere gives 60 ha of ruins): a factor of 5 to 7.5 in 100-150 years.

**Full city (1904 plan) to the Byzantine state of 539 CE (fort plan)**
- Grew: nothing; the fort of 539 is a new walled rectangle of 120 m by 80 m (about 1 ha) with 8 towers and walls preserved to 14 m, placed 250 m south of the city over the Aqua Septimiana spring sanctuary, built from the city's own stone (French Wikipedia).
- Shrank or was destroyed: the Vandals sacked the city in the 5th century; the Aurès Berbers evacuated or destroyed it by 484 and again in 535; Solomon's reconquest of 539 rebuilt a garrison town; occupation continued in reduced form until the 7th-century conquest and the city was empty by the 8th century (English Wikipedia).
- Stayed fixed: the cardo and decumanus were still the routes; the churches inside the Roman city continued; the fort anchors on the spring, which did not move.
- Moved or vanished: the civic centre moved from the forum to the fort and the spring; the suburbs emptied; the theatre and baths went out of use.
- Rate: from 60-90 ha to a 1 ha fort plus a partly occupied core: a shrink of more than 90 percent of the built area.

**539 CE state to the ruin plans of 1902-1927**
- Grew: nothing; the site was buried under sand and only exposed by excavation from 1880 to 1962.
- Shrank or was destroyed: full abandonment by the 8th century; stone robbing was limited because no later town stood nearby.
- Stayed fixed: everything in masonry: the complete street grid, the forum, the theatre, the arch, the Capitol, the baths, the fort; the 1902, 1904 and 1927 plans differ only in how much had been dug out.
- Moved or vanished: all timber, all population, all function.
- Rate: 0 ha occupied from the 8th century to 1927; the excavated area grew from about 10 ha (1902) to the whole 60-90 ha (1927) (unverified; estimated from the three plans).

### C. Closing line
Timgad shows that a street grid and its civic masonry survive both a six-fold expansion and 1,200 years of abandonment unchanged, while a shrink state does not contract back onto the old centre but re-anchors on water and defence (the fort on the spring, 250 m outside the grid). Proposal (Jeffrey decides): keep the original grid and monuments as pins in every later state, and give a shrink state its own new pinned stronghold placed by water, with the old pins kept as ruins.

## 14. Cross-city finding: what stays fixed

Across twelve cities, sixty-one dated plans and two to fifteen centuries each, the set that
is in the same place on every plan of a city is largely the same set everywhere; the "Moved
in" column names every exception the city blocks record:

| Layer | Fixed in | Moved in | Verdict |
|---|---|---|---|
| Water lines: river, coast, canal courses | all twelve (Mexico City's lakeshore retreated, the island and causeways did not) | none | pinned by nature; buried rivers persist as streets (Fleet, Neglinnaya, Bièvre) |
| Wall ring line | Rome and Constantinople (wall standing), Paris, Moscow and London (after demolition, as boulevards or streets), Amsterdam (each superseded line kept as a canal), Edo (moat ring), Ayutthaya (as a trace), Madurai in the world corpus (`town-plan-corpus-world.md`, Madurai 1906: the Veli streets on the demolished fort wall) | Vienna (the Ringstrasse lies on the glacis, not on the wall line), Amsterdam (gates moved outward with each new wall), Timgad (founding wall dismantled and built over as the city grew) | pinned in most cases; a demolished wall usually becomes a boulevard, a canal or a street ring on or beside its line |
| Primary river crossing | Rome, Paris, London, Dublin, Amsterdam, Moscow, Mexico City (causeways), Edo (Nihonbashi) | Dublin's Ormond Bridge moved 100 m once | pinned; bridges were rebuilt on their own sites twice in Dublin (Whitworth, Queen's) |
| Spine street: gate or crossing to market | all twelve | none | pinned |
| Main temple or church | Rome's Pantheon, Hagia Sophia (church to mosque, same building), Notre-Dame, St Paul's on its site, Oude Kerk, Stephansdom, Christ Church, St Basil's, Sensō-ji, every masonry wat in Ayutthaya | Mexico City (Templo Mayor demolished 1521; the cathedral stands on the Templo Mayor side of the plaza, not on the temple) | pinned; survives a change of religion in Constantinople, where the building itself was kept |
| Market square | Paris (Les Halles), London (Leadenhall, Smithfield), Amsterdam (Dam), Vienna (Hoher Markt, Neuer Markt), Dublin (Cornmarket, Smithfield), Moscow (Red Square), Mexico City (Plaza Mayor, Tlatelolco), Constantinople (Grand Bazaar, from 1456); the Rome and Edo blocks name no market | Ayutthaya (market moved from the centre and south to the north-east corner after 1767), Timgad (civic centre moved from the forum to the fort in 539) | pinned while the city lives; moves after regress |
| Citadel or castle | Rome (Castel Sant'Angelo), London (Tower), Vienna (Hofburg), Dublin (Castle), Moscow (Kremlin), Edo (castle), Timgad (fort, once built) | none | pinned |
| Ruler's residence | Amsterdam, Vienna, Moscow, Edo | Rome (papal centre, Lateran to Vatican), Constantinople (Great Palace to Topkapı, a short way past the head of the Mese; Topkapı to Dolmabahçe, 4 km), Paris (Cité, Hôtel Saint-Pol, Louvre, all within 1.5 km), Dublin (Custom House 800 m, not a palace but the same effect) | movable; a move re-weights where the city grows next |
| Street network inside the ring | street lines largely kept, with widening and firebreaks added: London after 1666 (widening, King Street–Queen Street), Moscow after 1611 and 1812, Constantinople after six fires (straight regularised streets after 1865 and 1870), Edo after 1657 (firebreaks, wider streets) | no street line moved after a fire; Mexico City 1521 is a siege, not a fire: a new grid was laid over the razed city, keeping only the causeway axes | pinned through fire, because plot boundaries are the durable object, not buildings; not pinned through total destruction and resettlement |
| Buildings | none | all | regenerable |
| Built extent | Amsterdam 1688 to 1839 (plateau) | Rome down 75 percent, Timgad down 90 percent, Ayutthaya down to 10 to 20 percent of the island, London up tenfold | free; grows in planned steps (rings, estates), shrinks by emptying the inside of the ring, not by moving the ring |

Two findings that the single-date corpora could not show:

1. **Regress keeps the pins.** Rome, Timgad and Ayutthaya each lost 70 to more than 90 percent
   of their built area and kept every water line, masonry monument and spine street. Rome kept
   its wall standing and Ayutthaya kept its wall line as a trace; Timgad's founding wall had
   already been dismantled during its growth, before the regress. A shrunken city is the same
   skeleton with the inside emptied. Timgad adds one rule: the
   shrink state did not contract onto the old centre but re-anchored on water and defence,
   a new 1 ha fort on the spring 250 m outside the grid.
2. **Fire is not regress.** Four cities had city-wide fires and were rebuilt on largely the
   same street lines, with widening and firebreaks added: London (1666, rebuilt 1667–1676;
   three planned replacement layouts were rejected within weeks because freeholders rebuilt
   on their own plots), Moscow (1611; 1812, rebuilt by 1818–1825, unverified), Constantinople
   (1660, 1693, 1718, 1756, 1865, 1870) and Edo (1657); the blocks give no rebuilding time for
   Constantinople and Edo. The exception is Ayutthaya, burned in 1767 together with conquest
   and deportation and not rebuilt: that is regress, not fire alone.

## 15. What the generator should take from this

Proposed in the PR that carries this report; Jeffrey decides, at milestone 4 for items 1 and
2 (landmarks, pins) and when v2 is scoped for items 3 to 5.

1. **The pinned layer is geometry, not landmarks only.** ADR-005 pins landmarks. This corpus
   says the durable set is: water lines, the wall ring line, the primary crossing, the spine,
   the main temple, the market (except after regress: Ayutthaya, Timgad) and the citadel. Districts, buildings and the ruler's residence
   are not in it. Recommendation: at milestone 4, `pinned` stays a landmark flag as designed,
   and the S1 macro stage treats wall line, crossing and spine as derived from pinned inputs
   so that pinning a gate and a market fixes the spine between them. No new specification field.
2. **A moved landmark is an event.** Dublin's Custom House moved once and the port, the docks
   and the fashionable quarter followed it. If v1 lets a specification move a pinned landmark, the
   result is a different city. Proposal (Jeffrey decides): treat that as intended behaviour,
   not as a bug to smooth over.
3. **For v2 "evolution", the state machine is: grow by adding a ring or an estate outside
   the current edge; shrink by emptying the inside and keeping the skeleton as ruins; burn by
   regenerating buildings on unchanged plots; relocate the ruler and re-weight growth.** These four
   operations cover most transitions in the twelve cities. They do not cover: Timgad's new
   fort on the spring outside the old grid (a new pinned stronghold), Mexico City's 1521
   replacement grid over the razed city (a new street net), and walls into boulevards, which
   is a separate fifth operation (item 4).
4. **Walls into boulevards** is one operation (Paris twice: the Grands Boulevards on the
   Charles V and Louis XIII walls, the boulevard ring on the Fermiers-Généraux wall; Moscow
   twice; Amsterdam as canals; Madurai as the Veli streets, `town-plan-corpus-world.md`,
   Madurai 1906): the wall object is deleted and a road ring is created on its line. Vienna is
   a variant: the Ringstrasse was built on the glacis beside the wall line, not on it. It needs the wall line to be stable geometry, which item 1
   gives.
5. **Plot boundaries are the right unit for the "burn" operation.** The S4 blocks stage should
   produce plots whose identifiers survive a regeneration of S5 buildings, which the
   stable-identifier rule (ARCHITECTURE.md §5) already requires. This corpus is the reason.

Not in scope here: the minor-town series (issue #27), which tests whether these findings hold
at 500 m to 5 km scale, where the generator lives.
