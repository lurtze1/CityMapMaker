# Research: maps at the table, legibility, believability, fantasy factors

Date: 2026-10-20. Scope: what makes a fantasy city overview map useful and believable at a
Pathfinder 2e table. Label placement algorithms, fonts, hand-drawn rendering and prior-art
generators are already covered in `research-rendering.md` and `research-tools.md` and are
not repeated here.

Labels used below:
- **[scholarly]** cartography teaching material or library/archive writing about historic maps.
- **[game]** a published game source or game-design tool. Useful for play, not history.
- **[community]** forum posts, blogs, tutorials by hobbyists. Opinion only.
- **[fiction]** a published fictional setting.
- **(unverified)** I saw it only in a search snippet and could not fetch the page.

## Maps at the table

### Legibility conventions

**General cartographic principles [scholarly].**
- Esri's cartography lead Aileen Buckley names five design principles: legibility ("the
  ability to be seen and understood"), visual contrast, figure-ground organisation,
  hierarchical organisation ("the visual separation of your map into layers of
  information") and balance (Esri ArcUser, Winter 2012).
  - *Figure-ground*: the eye splits the picture into a foreground "figure" and a background
    "ground". The thing that matters must be the figure.
- Axis Maps: "A map's visual hierarchy should match its intellectual hierarchy." Important
  things get bigger, bolder, higher-contrast symbols; ancillary things are "more subtle".
  They recommend a "squint test": squint at the map; the important elements must still
  stand out.
- *Generalisation* (one line: deliberately simplifying or dropping detail to fit the
  map's scale and purpose) is defined by the ITC/Twente GI body of knowledge as "the
  process of simplifying or modifying the representation of geographic features on a map".
  The usual operators (selection, simplification, aggregation, displacement, exaggeration)
  appear in search snippets from teaching sites; I could not fetch a page that lists them
  (unverified).

**Historic town views, Braun & Hogenberg [scholarly].**
- *Civitates Orbis Terrarum* (city atlas, Cologne, from 1572) used the bird's-eye view.
  Braun's stated goal, quoted by Cambridge University Library: the viewer should "look
  into all the roads and streets and see also all the buildings and open spaces".
- The same post: the views were drawn "with minimal reference to scale, proportion or
  relevant position of buildings", and the bird's-eye style gives "prominence ... to taller
  structures such as churches & towers". In Braun's Cambridge, college buildings get "much
  greater prominence" than other buildings. So: importance, not accuracy, drove size.
- Braun "was careful to include the proper names of places" (same post). Formal symbol
  legends came later; Rocque's London survey (1746) "used symbology widely".
- The Granada plate has an ornate cartouche (decorative frame) holding a numbered key of 19
  landmarks, "1. S. Christoual" to "19. Yglesia Mayor", plus foreground figures (a man with
  a donkey, cloaked women, riders) (Source Library page). This is one plate; I did not find
  a source saying how common numbered keys are across the atlas.

**Fantasy-map practice [community].**
- Jonathan Roberts (Fantastic Maps, 2011) layers a city map in this order: roads (main
  roads wider), then blocks as the negative space between roads, then a pattern per
  district, then icons for points of interest on top. He omits roof-level building texture
  because districts and individual locations "can easily get confused" with it. Roads get a
  dark glow so they stay readable over district fills.
- EN World "Cartography Workshop" thread (2007): a poster admires the Ptolus map "with every
  building, but that is really labor intensive" and prefers "the broad overview showing the
  major streets and points of interest", with colour-coded districts. Another poster draws
  blocks and streets "without having to define specific buildings".
- Steve Davies (ProFantasy, 2011): place walls early because they shape growth; put the main
  gate "at one of the higher points in the wall" (out of floods); "Cities on ridges ... tend
  to be linear, with anchor attractions at either end of the main road"; plain cities grow
  along trade routes; "respect the river routes". Secrets go in GM-only notes, not on the
  player map.
- Watabou, author of the Medieval Fantasy City Generator (devlog ~2017): "walls make a pile
  of polygons look like a fantasy city map", and "a castle adds some asymmetry and make[s] a
  map look better". Small temples are not drawn distinctly; the main temple is one large
  central building. Ward labels are a "helper", not "proper mapmaking labelling". A ward is
  named for its dominant use but is mixed: a craftsmen ward "doesn't consist entirely of
  workshops, there are also pubs, churches etc."
- Jared Blando and Mike Schley: I found no fetchable text by either author on city-map
  conventions. See Not found.

### What GMs use city maps for

All items in this section are **[community]** unless marked.
- **Planning under uncertainty.** Davies: "it's difficult to know where players will go
  next", so the GM needs "overall planning and a good city map". He keeps encounter ideas as
  GM-only notes on the map.
- **Districts over distances.** Sir Poley (blog, 2018): "No player is ever going to
  remember, or care about, the actual distance." Districts should be few: "seven as an
  absolute maximum, but as few as three is perfectly acceptable". Points of interest are
  numbered with one to three sentences each; each district can have its own random
  encounter table.
- **Heists.** Davyd Barker (D&D Beyond, 2024): the map is "something the players can
  interact with in-character as they plot out their strategy". The GM keeps a labelled
  version with secrets; players get a "rough or incomplete map" and earn more detail.
  Players draw their plan on the VTT map.
- **Map as a prop.** steelbuddha (Gamegrene, 2004): showing sketched city maps "shows great
  appreciation in the players' faces"; draw cities as an aerial view, not on graph or hex
  paper, so players do not expect a tactical fight. A commenter reports a tangible map
  helped a player focus.
- **Navigation, chases, ambush points.** Search snippets mention navigation and chase use
  (pursuit, line of sight, staging points) on D&D Beyond and SEO pages; I could not fetch a
  page that states this clearly (unverified).
- **Game source [game].** Pathfinder 2e GM Core (p. 138, Archives of Nethys): "Don't
  underestimate the usefulness of sketching a map of significant settlements ... This
  isn't intended to be a picture-perfect rendition drawn to scale."
- I found no survey or poll with numbers on what GMs use city maps for. See Not found.

### Believability heuristics

**S. John Ross, *Medieval Demographics Made Easy* v1.10 [game].** The author states it is
"a distilled gaming tool, unsuitable for schoolwork", drawn "from several countries across
multiple centuries", leaning late-medieval. Figures, quoted from the PDF:
- Settlement sizes: villages 20-1,000 (typical 50-300); towns 1,000-8,000 (typical ~2,500);
  cities 8,000-12,000; big cities 12,000-100,000.
- "Cities and towns tend to have walls only if they're politically important and/or
  frequently threatened."
- "Large population centers are the result of traffic": coasts, navigable rivers and trade
  routes; where several arteries converge, a city.
- Density inside a town: ~150 people per hectare (61 per acre); crowded ~300/ha, sparse ~75/ha.
  A city of 10,000 needs about 67 hectares inside the walls.
- Dwellings: 15-25 per acre typical, ~3 people per dwelling; buildings = 1-2x dwellings.
- Medieval buildings are small: "When in doubt, draw it smaller than your first-blush
  instinct suggests." Wood and half-timber buildings stand 1-3 storeys. Shops are often one
  perch (~16 ft) of street frontage with a deep plot behind.
- Support values (SV = people needed to support one business): shoemakers 150, taverns 400,
  bakers 800, butchers 1,200, blacksmiths 1,500, inns 2,000, magic-shops 2,800, booksellers
  6,300 (full table in the PDF). One noble household per ~200 people, one lawyer per 650,
  one clergy per 40, one priest per 25-30 clergy, one law officer per 150 in a well-kept city.
- Castles: about one functioning castle per 50,000 people; 75% in settled land, 25% on
  borders and wilderness.
- Fantasy note in the PDF: "If magic is common, the GM may decide a square mile of land can
  support many more" people.

**Pathfinder 2e GM Core [game]** (Archives of Nethys; remastered content, which replaced the
Gamemastery Guide).
- Settlement stat block (p. 170): name and level; traits (type first: village, town, city,
  metropolis); short description; government; population with ancestry breakdown; languages;
  religions (official and prohibited); threats; other characteristics; significant NPCs.
- Level: "A village is usually level 0-1, a town level 2-4, a city level 5-7, and a
  metropolis 8 or above", explicitly "simply guidelines".
- Building settlements (p. 138): "Towns with a population over a thousand typically have
  defensive walls." Growth adds stone fortifications that "further segment the city into
  districts or boroughs". Start from "the major trade route ... typically a river"; larger
  cities need "a deep-water harbor or a major overland trade road".
- Very large cities can use one stat block per district (seen in an AoN search snippet for
  the first-edition Gamemastery Guide; unverified for 2e).

**Other game sources.**
- Raging Swan Press *Village Backdrop* series [game]: one village per short supplement, built
  to drop into a campaign. I only saw retailer blurbs, not the internal format (unverified).
- Watabou's ward names reportedly follow *A Magical Medieval Society: Western Europe*
  (Gate, Craftsmen, Market, Merchant, Patriciate, Slum), per a MetaFilter comment (unverified; no URL recorded).

**Community.**
- EN World "Mapping the Town" thread (2025-2026): Celebrim asks "If a nation or city hasn't
  seen routine war in the last 800 years, why would it have walls?" and sketches open
  interior towns versus walled, garrisoned border towns. Koloth: building materials follow
  local resources (logs, sod, mud).

### Fantasy-specific factors

All items are **[community]**, **[fiction]** or **[game]**; none is history.
- **Flying threats.** EN World "Do castles make sense in a world of dragons & spells?"
  (2010): a dragon is "just a different form of siege gun"; "anti-dragon castles" need
  "very little modification" to high-medieval technology (Celebrim); "the best defense is a
  matched offense" (Thornir Alekeg), i.e. own aerial forces; fantasy fortifications "may
  not look much like castles of the high middle ages" (Haltherrion). Snippet-only ideas
  from other threads: netting or roofs over courtyards, rope nets on tower poles, griffon
  stables and launch space (unverified).
- **Magic versus walls.** "Mapping the Town" thread: Koloth says *fly*, *teleport* and
  *rock to mud* make walls "mostly useless against planned attacks"; magic such as Wall of
  Stone may shape defence. Zaukrie, quoted there: "the more people in an area, the less
  flying creatures can attack."
- **Magic and economy.** UngainlyTitan (2010 thread): "economics is based on some
  assumptions about magic"; cheap food magic or portals would change sieges and supply.
  Ross (above): common magic lets land support more people.
- **Magical construction.** Scott Gable (Kobold Press, 2022): one caster "can complete in
  hours what it might take dozens of workers a day or more". Snippets (unverified) mention
  Joshua Palmatier deriving street lighting and transit from a city's magic source.
- **Verticality [fiction].** Eberron's Sharn (Legendkeeper summary): the city rises on
  plateaus; core towers hold whole districts; wards are stacked vertically and "the
  boundaries between vertical sections are not clean"; "Altitude is the most reliable form
  of gatekeeping ever devised"; skycoaches and open floating-disc lifts; upper bridges have
  *feather fall* enchantments, lower ones do not. A manifest zone that boosts flight magic
  makes this possible (snippet only, unverified).
- **Mixed-size peoples.** EN World "Giant Sized Buildings in the City of Brass" (2018):
  "doggie doors" in large doors and medium stairs beside large ones; CapnZapp: "fantastical
  trumps realistic". Snippets (unverified): all doors must fit all sizes; building only to
  halfling scale may cause de facto segregation; the square-cube law makes falls harmless to
  very small folk.
- **Game rule hook [game].** The PF2e stat block lists population "with ancestry
  breakdown" and "threats". Those are the two fields a spec can use to drive these factors.

### What this means for a generator

- **Draw by importance, not by accuracy.** Braun & Hogenberg enlarged churches and towers;
  Axis Maps says visual order must match intellectual order. A spec field such as landmark
  importance must drive symbol size and line weight, and the generator must pass a squint
  test at handout size: wall, main streets, landmarks, then districts, then filler.
- **Omit buildings by default at overview scale.** Every practitioner source here prefers
  streets, blocks, district fills and landmark icons over roof-level detail. Optional
  building texture is a renderer choice; Jeffrey decides whether it exists.
- **Produce a GM map and a player map from one generation.** Davies and Barker both split
  secrets from the player view. Same seed, two render profiles: GM (all labels, secrets,
  numbers) and player (no secrets, optionally incomplete).
- **Use a numbered key for landmarks.** Two examples use one: a historic one (the Granada plate)
  and a modern one (Sir Poley); how common it is across atlases is unknown. Keep districts to 3-7 named areas as a spec validation warning, not a
  hard rule; Jeffrey decides.
- **Ship the believability numbers as a rule pack, labelled as game data.** Ross's
  density (~150/ha), dwellings per acre, SVs and PF2e's level bands give sanity checks:
  "walled area too small for population", "town over 1,000 without walls", "no fresh-water
  source or transit route" (the wording of the cited Pathfinder guidance). These are warnings with a source citation, not truth.
- **Treat fantasy factors as spec inputs, not defaults.** Population by ancestry, threats
  (for example flying), and magic level become spec fields that rule packs read (anti-air
  towers, mixed-scale gates, vertical wards). The generator never invents them; a missing
  one is a GM? question.

### Sources

All fetched on 2026-10-20 unless marked.
- Aileen Buckley, "Make Maps People Want to Look At", Esri ArcUser, Winter 2012:
  https://www.esri.com/news/arcuser/0112/make-maps-people-want-to-look-at.html
- Axis Maps, Cartography Guide, "Visual hierarchy": https://axismaps.com/guide/visual-hierarchy
- ITC Twente GI body of knowledge, "Cartographic generalization":
  https://ltb.itc.utwente.nl/659/concept/174062
- Cambridge University Library Special Collections, "Pictures, Perspectives and Plans"
  (2014): https://specialcollections-blog.lib.cam.ac.uk/?p=7504
- Source Library, Braun & Hogenberg Granada plate: https://sourcelibrary.org/artwork/art-granada
- Jonathan Roberts, "A Quick City Map Tutorial", Fantastic Maps, 2011:
  https://www.fantasticmaps.com/?p=1185
- Steve Davies, "Mapping Cities - Part 1", ProFantasy, 2011:
  https://rpgmaps.profantasy.com/mapping-cities-part-1
- Watabou, "Some answers and comments", MFCG devlog:
  https://watabou.itch.io/medieval-fantasy-city-generator/devlog/1579/some-answers-and-comments
- EN World, "Cartography Workshop" (2007):
  https://www.enworld.org/threads/cartography-workshop.174110/post-3233163
- Sir Poley, "On Towns in RPGs, Part 5: Building a Playable City" (2018):
  https://www.goodreads.com/author_blog_posts/16837272-on-towns-in-rpgs-part-5-building-a-playable-city?tab=book
- Davyd Barker, "Without a Hitch: Running Heists in D&D Using a VTT", D&D Beyond, 2024:
  https://www.dndbeyond.com/posts/1710-without-a-hitch-running-heists-in-d-d-using-a-vtt
- steelbuddha, "Cartography: A Powerful GM Tool", Gamegrene, 2004:
  https://www.gamegrene.com/node/345
- S. John Ross, *Medieval Demographics Made Easy* v1.10 (PDF, text extracted locally):
  https://www.martinralya.com/wp-content/uploads/2021/05/Medieval-Demographics-Made-Easy.pdf
- Pathfinder 2e GM Core, settlement stat block, Archives of Nethys:
  https://2e.aonprd.com/Rules.aspx?ID=1168
- Pathfinder 2e GM Core, Building Settlements, Archives of Nethys:
  https://2e.aonprd.com/Rules.aspx?ID=1139 and https://2e.aonprd.com/Rules.aspx?ID=2974
- EN World, "Mapping the Town - What should a Fantasy Town look like?" (2025-2026):
  https://www.enworld.org/posts/9895992/
- EN World, "Do castles make sense in a world of dragons & spells?" (2010):
  https://www.enworld.org/goto/post?id=5141016
- EN World, "Giant Sized Buildings in the City of Brass" (2018):
  https://www.enworld.org/goto/post?id=7390375
- Scott Gable, "A Magical City", Kobold Press, 2022: https://koboldpress.com/a-magical-city/
- Legendkeeper, Sharn description (fan wiki summarising Eberron):
  https://legendkeeper.com/p/cmo8thuxo0zxs0jk97fcrg8zk/m3767mkn/e3mo86eu

### Not found

- Any text by Mike Schley on city-map design or labelling.
- The table of contents or city advice of Jared Blando's *How to Draw Fantasy Art and RPG
  Maps* or *Fantasy Mapmaker: How to Draw RPG Cities* (publisher pages returned no
  description).
- A Cartographers' Guild tutorial on city overview conventions (search returned none).
- Any survey, poll or reddit thread with numbers on what GMs use city maps for. Reddit
  pages were not fetchable (mirror returned 503).
- A Lost Kingdoms settlement guide.
- The internal format of Raging Swan Press settlement books.
- The Eberron source book text for Sharn (fandom wiki returned 402); only a fan summary.
- A fetchable list of cartographic generalisation operators (colorado.pressbooks.pub 403).
- Any source on magical infrastructure (sewers, light, water) that I could fetch in full.
