# Research: district types in pre-industrial towns

Researched 2026-10-20 for the CityMapMaker district-type list. Every claim below has a cited URL; S41
returned HTTP 403 and S42 and S43 were seen only as search summaries. Claims I saw only in search-result summaries, without
fetching the page, are marked "(unverified)". Wikipedia sources are labelled "(Wikipedia)".

## District types

### Findings

1. **A "district" in the sources is defined in four ways, and they overlap.**
   - **By function.** Example: Japanese castle towns had merchant and artisan streets
     allocated by trade, with names like Kaji-machi (blacksmiths) and Daiku-machi
     (carpenters). [S7]
   - **By social group.** This covers religion, ethnicity, caste and rank. Examples: the
     Venetian Ghetto (1516) [S14], the Moroccan mellah [S6], the Gujarati pol ("linked by
     caste, profession, or religion") [S15], and samurai quarters ranked by distance to the
     castle [S7].
   - **By administration.** Examples: London's wards [S9], Florence's quarters and
     gonfaloni [S10], Rome's rioni [S2] and Venice's sestieri [S1]. These are counting and
     governing units. Each one contained a mix of functions.
   - **By age.** Examples: Prague's New Town, founded in 1348 outside the walls as a
     separate royal town [S18], and Kazimierz, founded in 1335 from two suburbs of Kraków
     [S19]. The faubourg (a "fore-town" that grew along the road outside a gate) is the
     age-and-position type [S5].
   - Glossary: a *ward* is a city subdivision for taxes, watch duty or elections. A *parish*
     is the area served by one church. A *quarter* is any recognisable city part. The word
     goes back to Roman towns, which the two main streets cut into four parts [S3].

2. **Administrative units are almost never single-function.**
   - Florence's four quarters were political units, each with four gonfaloni (militia and
     voting districts). The 51 parishes were a separate, finer grid. [S10]
   - Jerusalem's "four quarters" are a 19th-century map convention, possibly from an 1840s
     British map. The source says there was "no governing principle of ethnic segregation"
     and mixed residence was common. [S11]
   - Lesson: a labelled quarter on a map can be a mapmaker's simplification of a mixed
     reality.

3. **Function clusters are real, but they cluster along streets, not in whole districts.**
   - Medieval Bristol put water trades by the river: fishing, tanning and leather working.
     Weavers, tanners, dyers and fullers were grouped in Redcliffe/Temple. Ironworking and
     shipbuilding were outside the walls at Narrow Quay. [S22]
   - The Broadmead suburb of Bristol held a Dominican friary next to tanning, leatherworking,
     ironworking and cloth trades. River access was the key asset. [S23]
   - Worcester's suburb outside the walls had tilemaking, metalworking and tanning. [S24]
   - Fez's souk streets run from the gates to the main mosque. Most funduqs (merchant inns)
     sit on those streets. [S12]
   - Pattern: **noxious and water-hungry crafts sit at the edge, often outside the walls,
     next to a river.** Religious houses such as friaries also sit in these suburbs.

4. **Planned capitals had explicit zones, and those zones did match our "type" idea.**
   - Tang Chang'an had a Palace City for the emperor, an Imperial City for government workers
     and an Outer City of walled residential wards. Trade was confined to two official
     markets, East and West. [S16]
   - Heian-kyō (Kyoto) had an East and a West Market, one in each half of the city. [S17]
   - Jaipur's walled city had nine blocks. Two held the state buildings and palaces. [S20]
   - Japanese castle towns had rings: samurai quarters near the castle, then merchant and
     artisan streets, foot soldiers at the outer edge of the merchant districts, and temples
     on the outer rim as a defensive belt. [S7]
   - Spanish colonial towns (Laws of the Indies) had a Plaza Mayor at the centre. A hospital
     stood near the church. Slaughterhouses, fisheries and tanneries were placed so their
     filth could be removed. A commons (ejido) was set aside for growth and grazing. [S25]

5. **Enclaves for a social group were often walled and gated, and often sat next to power.**
   - Venetian Ghetto: locked at night, two bridges open only by day. [S14]
   - Mellah: walled, with a fortified gate, usually near the citadel, palace or governor's
     residence. Fez's mellah adjoins the royal palace. [S6]
   - London Steelyard: a walled Hanseatic merchant enclave with its own warehouses, chapel,
     guildhall and law. [S26]
   - Dejima: an artificial island for foreign merchants, about 120 m by 75 m. [S27]
   - Pols in Ahmedabad: gates closed at night, about 360 pols inside the fortified city. [S15]
   - Cairo's harat (residential quarters) were often gated at night. [S29]
   - Cathedral closes had gates locked at night and their own jurisdiction. [S4]
   - Glossary: an *enclave* is an area inside a town that belongs to a different group or
     law than the area around it. A *liberty* was an English area outside the normal
     borough or hundred jurisdiction; the Tower of London had one. [S28]

6. **Some districts are institutions, not neighbourhoods.**
   - The Venetian Arsenal (state shipyard) covered about 45 ha, roughly 15% of Venice. It
     was walled and employed almost 16,000 people at its peak. [S30]
   - Tenochtitlan's walled ceremonial precinct was about 500 m per side with about 45 public
     buildings. [S31]
   - Srirangam's whole town lived inside the seven walled enclosures of one temple. [S21]
   - Edo: samurai and daimyō residences reportedly covered up to 70% of the land, temples and
     shrines about 15%. (Wikipedia; the page flags the figures as unsourced and possibly
     AI-generated, so treat as unverified.) [S8]

7. **Cemeteries sit outside the walls.** Cairo's City of the Dead (al-Qarafa) lies outside
   the walled city [S29b]. Fez's original Jewish cemetery lay just outside Bab Guissa [S12].

8. **Ages and edges matter.** Aleppo's Christians moved out to the north-west suburb of
   Jdeydeh in 1420 [S13]. Prague grew as five separate towns, independent until 1784 [S18].
   The faubourg grew along roads from the gates [S5].

9. **Existing generators and settings.**
   - Watabou's TownGeneratorOS (GPL-3.0; idea only, no code) has these ward classes:
     Administration, Castle, Cathedral, Craftsmen, Farm, Gate, Market, Merchant, Military,
     Park, Patriciate, Slum, plus base classes [S32]. Our draft list is close to this one.
   - Absalom, the main Pathfinder city, has 11 districts. The wiki describes them by
     **wealth** (Petal and Ivy are "high-class havens"; Puddles and Precipice are "dangerous
     and destitute"), by **group** (the halfling districts), by **function** (Docks) and by
     **monuments** (Ascendant Court, Wise Quarter). [S33]
   - Kevin Lynch (urban planner, *The Image of the City*): a district is a sizeable area a
     person can enter, with "common identifying characteristics". It is one of five
     elements: paths, edges, districts, nodes and landmarks. [S34]
   - Lynch's definition is about recognisable *character*. Character can come from function,
     group, wealth or age.

### Quarter types by culture

| Name | Culture | Defined by | Example | Source |
|---|---|---|---|---|
| Ward | Medieval England | admin | City of London: 24 wards by 1206 | S9 |
| Parish | Medieval Europe | admin (church) | Florence: 51 parishes | S10 |
| Quarter / gonfalone | Florence | admin | 4 quarters with 16 gonfaloni (1343) | S10 |
| Sestiere | Venice, Genoa, Milan | admin | Venice: 6 | S1 |
| Rione | Rome | admin | 13 rioni, then 14 in 1586 | S2 |
| Cathedral close | Medieval Europe | group (clergy) + own law | Salisbury, York | S4 |
| Liberty | England | own law | Liberties of the Tower of London | S28 |
| Ghetto / Jewish town | Venice, Kraków | group (religion), walled | Venice 1516; Kazimierz 1495 | S14, S19 |
| Foreign merchant kontor | Hanseatic League | group + function, walled | Steelyard, London | S26 |
| Craft street | Medieval England | function | Bristol Redcliffe/Temple cloth and leather trades | S22 |
| Suburb / faubourg | Medieval Europe | age + position | Broadmead (Bristol), Worcester | S5, S23, S24 |
| New town | Central Europe | age, separate town | Prague New Town 1348 | S18 |
| Medina core / souk | Morocco | function | Fez: souk streets from gates to the Qarawiyyin | S12 |
| Kasbah / citadel | Maghreb, Levant | function (power, defence) | Fez kasbahs; Aleppo citadel | S12, S13 |
| Mellah | Morocco | group (religion), walled | Fez, next to the palace | S6 |
| Hara | Cairo | residential, gated | Mamluk Cairo | S29 |
| Mahalla / mahalle | Islamic world, Ottoman | admin + group (mosque or ethnic community) | smallest Ottoman admin unit | S35, S36 |
| Christian suburb | Aleppo | group (religion) + age | Jdeydeh, 1420 | S13 |
| Cemetery zone | Cairo, Fez | function, outside walls | al-Qarafa | S29b, S12 |
| Fang (walled ward) | Tang China | admin + residential | Chang'an, about 110 wards | S16 |
| Official market | Tang China, Heian Japan | function | East and West markets | S16, S17 |
| Imperial / palace city | Tang China | function (government) | Chang'an Imperial City | S16 |
| Banner quarter | Qing China | group (military caste) | Beijing inner city given to bannermen | S37 |
| Samurai-machi | Japan | group (rank) | higher rank nearer the castle | S7 |
| Chōnin-chi / machi | Japan | function (trade) | Kaji-machi (blacksmiths) | S7 |
| Tera-machi | Japan | function (temples), edge belt | outer rim | S7 |
| Foreign enclave | Japan | group (foreigners) | Dejima, 1636 | S27 |
| Pol | Gujarat, India | group (caste, profession, religion), gated | Ahmedabad, about 360 | S15 |
| Temple town | South India | function (religion) | Srirangam, 7 enclosures | S21 |
| Palace block | Jaipur | function (government) | 2 of 9 blocks | S20 |
| Craft guild quarter | Benin | function + kin (7 families) | Igun Street bronze casters | S38 |
| Craft ward (anguwa) | Hausa, Zaria | function | Anguwan Kahu (cotton mattresses) | S39 |
| Calpulli | Aztec | group (kin) + craft | 20 per quarter in Tenochtitlan | S31, S40 |
| Ceremonial precinct | Aztec | function (religion) | Tenochtitlan, 500 m square | S31 |
| Traza vs Indian barrios | Spanish colonial | group (Spanish or native) | Mexico City | (unverified) S41 |
| Arsenal / shipyard | Venice | function (state industry) | 45 ha, about 15% of the city | S30 |

### District counts

| Town | Era | Count | Kind of division | Source |
|---|---|---|---|---|
| Venice | historic, still in use | 6 | sestieri (admin) | S1 |
| Rome | 12th c. / 13th–16th c. / 1586 | 12 / 13 / 14 | rioni (admin) | S2 |
| Rome | Augustus | 14 | regions (admin) | S2 |
| London | 1206 / 1394 / 1550 | 24 / 25 / 26 | wards (admin) | S9 |
| Florence | after 1343 | 4 quarters, 16 gonfaloni, 51 parishes | admin, three levels | S10 |
| Florence | 1172–1343 | 6 | sestieri (unverified, search summary only) | S42 |
| Fez el-Bali | early 20th c. (Le Tourneau) | 2 quarters, 18 neighbourhoods | river banks + neighbourhoods | S12 |
| Aleppo | modern listing of historic city | 9 inside walls, 19 outside | districts | S13 |
| Jerusalem | 19th c. convention | 4 | religious/ethnic labels, unequal in size | S11 |
| Prague | until 1784 | 5 | separate towns | S18 |
| Tang Chang'an | 7th–9th c. | about 110 (the page also says 160) + 2 markets + 2 government cities | walled wards | S16 |
| Heian-kyō | 794 | 8 bō, each with 16 chō; 2 markets | grid wards | S17 |
| Tenochtitlan | 1500 | 4 quarters, 20 calpulli each | admin + kin | S31 |
| Jaipur | 1727 | 9 blocks, 2 of them palace | planned blocks | S20 |
| Ahmedabad | historic | about 360 | pols (group enclaves) | S15 |
| Istanbul | 1600 | 226 neighbourhoods (unverified, AI-summary page) | mahalle | S43 |
| Absalom (fiction, Pathfinder) | — | 11 | districts | S33 |

Reading the table:

- Two scales appear. Coarse divisions run 2 to 6: quarters, sestieri, towns, river banks.
  Fine divisions run 13 to 360: wards, rioni, parishes, pols, fang.
- A GM-readable overview map matches the coarse scale, or a little more. Absalom's 11 is the
  one game-city data point I found.
- No source I found says how many districts a GM map should show. See "Not found".

### Assessment of the draft list

Draft list: residential, craftsmen, merchant, harbour, farms, noble, temple_quarter,
military, shanty, market.

| Draft type | Verdict | Reason |
|---|---|---|
| residential | **Keep** | Every culture has it: hara, mahalle, pol, fang, calpulli. [S29, S35, S15, S16, S31] |
| craftsmen | **Keep**, rename to `craft` | Craft streets are universal [S7, S22]. Add a "noxious" flag or a separate type; see `works` below. |
| merchant | **Merge** with market into `commerce` | Chang'an and Heian markets were the merchant zone [S16, S17]. Fez souk streets are both market and merchant [S12]. Two types for one function forces a choice the sources do not make. The open market square is a *place* (a node, in Lynch's terms [S34]), not a district. |
| market | **Merge** (see above), and keep a market square as a landmark or plaza | as above |
| harbour | **Keep** | Ports need quays, warehouses and shipyards. Bristol put shipbuilding at Narrow Quay [S22]. Venice's Arsenal was 15% of the city [S30]. |
| farms | **Keep** as an edge type | Laws of the Indies reserve commons and grazing at the edge [S25]. Watabou has Farm [S32]. |
| noble | **Change to an attribute** (wealth = high) or a rule-pack group | Game comparator only, not historical evidence: Absalom's "high-class havens" are wealth labels [S33]. Samurai quarters are a rank group with their own ordering rule (higher rank nearer the castle) [S7]. That is culture-specific. |
| temple_quarter | **Keep, rename** `religious_precinct` | Covers the cathedral close [S4], the temple town [S21], the ceremonial precinct [S31] and the tera-machi belt [S7]. "Quarter" wrongly suggests housing; most were walled precincts with their own law. |
| military | **Keep, narrow** to garrison, barracks and arsenal | Group-based military housing (banner quarters [S37], ashigaru-machi [S7]) is residential + group, not a military function. |
| shanty | **Change to an attribute** (wealth = poor) + suburb position | Game comparator only: "dangerous and destitute" districts in Absalom are wealth labels [S33]. Pre-industrial poor areas sat in suburbs outside the walls [S18 for Prague's poorer Czechs in suburbs]. |
| *(missing)* `seat_of_power` | **Add** | Citadel, kasbah, palace, castle bailey, Imperial City, Jaipur palace blocks. Found in every planned capital [S16, S20, S12, S13]. Watabou has Castle and Administration [S32]. Our list has nowhere to put it. |
| *(missing)* `works` (noxious or water industry) | **Add** | Tanning, dyeing, fulling, ironworking, tilemaking, shipbuilding were pushed to rivers and suburbs [S22, S23, S24]. The Laws of the Indies place slaughterhouses and tanneries apart [S25]. These behave differently from `craft`: they sit at the edge and on water. |
| *(missing)* `cemetery` | **Add** | Outside the walls in Cairo and Fez [S29b, S12]. It is visually distinct and a GM uses it. |
| *(missing)* enclave (religious, ethnic, foreign) | **Add as an attribute, not a type** | Ghetto, mellah, Steelyard, Dejima, Jdeydeh, pol [S14, S6, S26, S27, S13, S15]. The *function* varies: the Steelyard is commerce, the mellah is residential. Group + enclosure covers them all. |
| *(missing)* suburb / faubourg / new town | **Add as an attribute** (age or position), not a type | A suburb holds any function: friary + tannery in Broadmead [S23]. |
| *(missing)* open ground / commons | **Optional add**; Jeffrey decides | Ejido [S25]; Watabou "Park" [S32]. |
| *(culture-specific)* samurai-machi, tera-machi, mellah, banner quarter, pol, calpulli, fang, sestiere | **Move to rule packs** | Each comes with its own placement rule: rank rings [S7], mellah next to the palace [S6], fang grid [S16]. A rule pack maps the culture name onto core function + group + attributes. |

### Proposed model

**Recommendation:** split "district type" into one closed function enum plus separate
attributes. Mapping a district to a single *type* loses three things the sources show:

- who lives there (group);
- how rich it is (wealth);
- whether it is old, new or outside the walls (age or position), and whether it is gated or
  under its own law (enclosure).

The mellah, the Steelyard and the Venetian Ghetto have different functions but share the
same group-and-enclosure pattern. One enum cannot express that.

**Core function enum (closed, 10 values):**

1. `residential`
2. `craft`
3. `works`: noxious or water-bound industry
4. `commerce`: replaces merchant + market
5. `harbour`
6. `seat_of_power`: castle, citadel, palace, government
7. `military`: barracks, garrison, arsenal
8. `religious_precinct`: replaces temple_quarter
9. `farmland`: replaces farms
10. `cemetery`

`open_ground` is an optional 11th value. Jeffrey decides.

**Attributes, each separate from the function:**

| Attribute | Values | Evidence |
|---|---|---|
| `wealth` | poor / middling / rich | Absalom labels [S33] (game comparator); replaces noble and shanty |
| `group` | an identifier from a rule pack or specification; open data, not a closed enum. Example: `jewish`, `samurai`, `hanse` | S6, S7, S14, S26 |
| `enclosure` | open / gated / walled | S4, S6, S14, S15, S29 |
| `jurisdiction` | town / separate | close, liberty, Steelyard, Prague towns [S4, S28, S26, S18] |
| `position` | intramural / suburb | faubourg, Broadmead [S5, S23] |

Assumption: position (inside or outside the walls) can be computed from geometry, so it does
not need to be stored. Jeffrey can veto this.

**Rule packs** map culture terms onto the core. For example, a Japanese pack defines
`samurai-machi` as `residential` + group `samurai` + wealth `rich` + "rank orders distance to
castle". The core enum stays closed. Group names live in data, which matches hard rule 6
("rules are data").

**Optional two-level naming.** The sources show two scales: coarse admin quarters (2–6) and
fine function patches (13+). One option is a named quarter layer (sestiere, ward) above
function patches. Jeffrey decides whether the map needs both or only one.

**Mixing.** The sources say districts are mixed [S10, S11, S23]. One option is a minority
function share per district. Example: residential with 20% craft. This is optional;
Jeffrey decides. I found no number for a typical mix.

### Sources

All fetched 2026-10-20.

- S1 Sestiere (Wikipedia): https://en.wikipedia.org/wiki/Sestiere
- S2 Rioni of Rome (Wikipedia): https://en.wikipedia.org/wiki/Rioni_of_Rome
- S3 Quarter (urban subdivision) (Wikipedia, stub): https://en.wikipedia.org/wiki/Quarter_(urban_subdivision)
- S4 Cathedral close (Wikipedia): https://en.wikipedia.org/wiki/Cathedral_close
- S5 Faubourg (Wikipedia): https://en.wikipedia.org/wiki/Faubourg
- S6 Mellah (Wikipedia): https://en.wikipedia.org/wiki/Mellah
- S7 Jōkamachi (Wikipedia): https://en.wikipedia.org/wiki/Jōkamachi
- S8 Edo (Wikipedia; flagged unsourced): https://en.wikipedia.org/wiki/Edo
- S9 Wards of the City of London (Wikipedia): https://en.wikipedia.org/wiki/Wards_of_the_City_of_London
- S10 Florentine Renaissance Resources, Brown University gazetteer notes: https://cds.library.brown.edu/projects/florentine_gazetteer/notes.php
- S11 Old City (Jerusalem) (Wikipedia): https://en.wikipedia.org/wiki/Old_City_(Jerusalem)
- S12 Fes el Bali (Wikipedia, citing Le Tourneau): https://en.wikipedia.org/wiki/Fes_el_Bali
- S13 Ancient City of Aleppo (Wikipedia): https://en.wikipedia.org/wiki/Ancient_City_of_Aleppo
- S14 Venetian Ghetto (Wikipedia): https://en.wikipedia.org/wiki/Venetian_Ghetto
- S15 Pol (housing) (Wikipedia): https://en.wikipedia.org/wiki/Pol_(housing)
- S16 Chang'an (Wikipedia): https://en.wikipedia.org/wiki/Chang%27an
- S17 Heian-kyō (Wikipedia): https://en.wikipedia.org/wiki/Heian-ky%C5%8D
- S18 New Town, Prague (Wikipedia): https://en.wikipedia.org/wiki/New_Town,_Prague
- S19 Kazimierz (Wikipedia): https://en.wikipedia.org/wiki/Kazimierz
- S20 Jaipur (Wikipedia): https://en.wikipedia.org/wiki/Jaipur
- S21 Srirangam (Wikipedia): https://en.wikipedia.org/wiki/Srirangam
- S22 Bristol and Avon Archaeological Society, talk report: https://bristolandavonarchaeology.org.uk/?p=343
- S23 Cotswold Archaeology Monograph 5, *Friars, Quakers, Industry and Urbanisation* (publisher page): https://www.archaeopress.com/Archaeopress/Products/9780956305480
- S24 Explore the Past (Worcestershire archive service), Sansome Street: https://explorethepast.co.uk/2018/06/archaeologists-at-sansome-street-worcester
- S25 Laws of the Indies (Wikipedia): https://en.wikipedia.org/wiki/Laws_of_the_Indies
- S26 Steelyard (Wikipedia): https://en.wikipedia.org/wiki/Steelyard
- S27 Dejima (Wikipedia): https://en.wikipedia.org/wiki/Dejima
- S28 Liberty (division) (Wikipedia): https://en.wikipedia.org/wiki/Liberty_(division)
- S29 History of Cairo (Wikipedia; harat gated at night): https://en.wikipedia.org/wiki/History_of_Cairo
- S29b Islamic Cairo (Wikipedia; City of the Dead outside walls): https://en.wikipedia.org/wiki/Islamic_Cairo
- S30 Venetian Arsenal (Wikipedia): https://en.wikipedia.org/wiki/Venetian_Arsenal
- S31 Tenochtitlan (Wikipedia): https://en.wikipedia.org/wiki/Tenochtitlan
- S32 Watabou TownGeneratorOS ward classes (GitHub; GPL-3.0, ideas only): https://github.com/watabou/TownGeneratorOS/tree/master/Source/com/watabou/towngenerator/wards
- S33 Absalom (PathfinderWiki): https://pathfinderwiki.com/wiki/Absalom
- S34 The Image of the City (Wikipedia): https://en.wikipedia.org/wiki/The_Image_of_the_City
- S35 Mahalla (Wikipedia): https://en.wikipedia.org/wiki/Mahalla
- S36 Mahalle (Wikipedia): https://en.wikipedia.org/wiki/Mahalle
- S37 Eight Banners (Wikipedia, citing Wakeman 1985): https://en.wikipedia.org/wiki/Eight_Banners
- S38 Igun Street (Wikipedia): https://en.wikipedia.org/wiki/Igun_Street
- S39 Zaria (Wikipedia): https://en.wikipedia.org/wiki/Zaria
- S40 Calpulli (Wikipedia): https://en.wikipedia.org/wiki/Calpulli
- S41 (unverified: the fetch returned 403) VoxDev, colonial segregation in Mexico City: https://voxdev.org/topic/institutions-political-economy/how-colonial-segregation-policies-impact-land-values-mexico
- S42 (unverified: search summary only) Sepoltuario database, Florence locations: https://sepoltuario.iath.virginia.edu/locations/5
- S43 (unverified: search summary only) Istanbul neighbourhood count: https://portal.yandex.com.tr/yaozet/education/ottoman-settlement-in-istanbul-id1-tl7u291V
- Fetched, but only the introduction was readable: Abu-Lughod, "The Islamic City—Historic
  Myth, Islamic Essence, and Contemporary Relevance", IJMES 19 (1987):
  https://histories.newschool.edu/node/43

### Not found

- Any source that says how many districts a GM or overview map should show. Absalom's 11 is
  one data point, not a rule.
- A count of Cairo's harat, Mamluk or Ottoman. The Raymond *Mamluk Studies Review* PDF
  returned 404.
- A reliable 16th-century count of Istanbul mahalles.
- Pre-1790 Paris quartier counts. The two Wikipedia pages fetched do not give them.
- A scholarly, comparative statement on how mixed European craft districts were. The
  Portuguese MedCrafts introduction (eHumanista 49) was fetched but contains no spatial
  claim. I did not reach Keene's work on Winchester or London.
- The text of the Laws of the Indies itself. The HUD PDF came back empty; Wikipedia's summary
  is used instead.
- Any statement on Spanish separation of Indian barrios that I could fetch and verify.
- The number and organisation of wards in Kano or Benin City.
- Any statement on how castes were arranged by ring inside Srirangam.
- Any proportion of land per district type in a European town.
