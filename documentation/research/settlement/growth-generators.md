> Written 2026-10-20. Research only; binding only when a DECISIONS.md entry cites it.
> Complements `documentation/research/research-academic.md` (2026-10-19). Entries already covered
> there (Weber 2009, Emilien 2012, Vanegas 2009, Lechner, Groenewegen, Mas 2020, Bulbul 2023,
> Galin roads) are re-checked here only for **build order** and **pins**, the two questions
> this report answers.

## Growth-based and roads-first generators

Words used below, explained once:

- **Roads-first**: the generator lays the long-distance roads before it decides where the town is.
- **Growth simulation**: the generator runs many small time steps; each step adds a bit of town.
- **Agent**: a small simulated actor (a settler, a developer) that makes choices each step.
- **Interest map**: a grid of scores ("how good is it to build here?") made by adding weighted terms such as slope or distance to water.
- **Nucleus**: a starting point that a town grows out from, for example a crossroads.
- **Pin**: an element the user fixes; the generator must build around it and never move it.
- **Deterministic**: same inputs and seed give the same output, every time, on every machine.

### Summary table

"Pins" means: can the user keep chosen elements fixed while the rest is generated?
"y (seeds)" means only the starting elements are user-given, not arbitrary later pins.

| Name | Year | Kind | Build order | Growth over time | Pins | Licence | Source |
|---|---|---|---|---|---|---|---|
| Weber, Müller, Wonka, Gross, "Interactive Geometric Simulation of 4D Cities" | 2009 | paper | given street(s) -> major streets per step -> quarters -> land use -> minor streets -> lots -> buildings (building lags planning) | y | y (seeds; initial layout can be an existing city; edits during run) | paper only | [Weber PDF](https://www.peterwonka.net/Publications/pdfs/2009.EG.Weber.UrbanSimulation.FinalVersion.pdf) |
| Emilien et al., "Procedural Generation of Villages on Arbitrary Terrains" | 2012 | paper | terrain -> (house seed -> road to it) repeated per scenario event -> parcels -> 3D houses | y (event list) | y (seeds: connection points, scenario) | paper only | [Emilien PDF](https://perso.liris.cnrs.fr/egalin/Articles/2012-villages.pdf) |
| Vanegas, Aliaga, Benes, Waddell, "Interactive Design of Urban Spaces..." | 2009 | paper | user sketches highways -> behaviour and geometry iterate to equilibrium | n (iterates to balance, not history) | **y** (any variable or region can be constrained; rest is completed) | paper only | [Vanegas PDF](https://www.cs.purdue.edu/homes/bbenes/papers/Vanegas09ToG.pdf) |
| Lechner, Watson, Wilensky et al., "Procedural Modeling of Urban Land Use" | 2006 | paper (NetLogo agents) | terrain -> developer agents prospect, build, extend roads per tick | y | y (artist paints land use or behaviour) | paper only | [Lechner TR](https://ccl.northwestern.edu/papers/2006/TR-2007-33.pdf) |
| Groenewegen et al., "Procedural City Layout Generation Based on Urban Land Use Models" | 2009 | paper | city size/location -> **highways around and out of core** -> districts by land-use model | n (present state only) | n | paper only | [Groenewegen PDF](https://graphics.tudelft.nl/~rafa/myPapers/bidarra.EG09.pdf) |
| Galin, Peytavie, Guérin, Benes, "Authoring Hierarchical Road Networks" | 2011 | paper | terrain -> user places settlements -> highways -> major -> minor roads (merged) -> geometry | n | y (settlements are user-placed) | paper only | [Galin PDF](https://perso.liris.cnrs.fr/eric.guerin/download/pg2011.pdf) |
| Beneš, Wilkie, Křivánek, "Procedural Modelling of Urban Road Networks" | 2014 | paper | **roads between neighbouring cities first** -> their junctions become nuclei -> loop: minor roads, traffic, new major roads | y | n (only "historical context" parameters) | paper only | [Beneš PDF](https://cgg.mff.cuni.cz/~jaroslav/papers/2014-cities/2014-cities-paper.pdf) |
| Nishida, Garcia-Dorado, Aliaga, "Example-Driven Procedural Urban Roads" | 2016 | paper | sketch area + optional manual highways -> grow arterials from seeds -> local streets -> blocks, parcels, buildings | n (growth is an algorithm, not history) | y (manual roads, seed positions) | paper only | [Nishida PDF](https://www.cs.purdue.edu/cgvlab/papers/aliaga/cgf15.pdf) |
| Song & Whitehead, "TownSim" | 2019 | paper + Python code | water + short road + building patch -> agents walk -> busy cells become roads, buildings | y | n | not checked | [TownSim PDF](https://www.pcgworkshop.com/archive/song2019agentbased.pdf) |
| Barthélemy & Flammini, "Modeling urban street patterns" | 2008 | urban-science model | small connected network -> add centres every few steps -> roads grow toward them | y | n | paper only | [arXiv](https://arxiv.org/html/0708.4360v2) |
| SLEUTH (Clarke) | 1997- | cellular automaton (CA: grid cells that change state by local rules) | initial urban extent + roads + exclusion -> 4 growth rules per cycle | y | y (exclusion layer cells never urbanise) | not checked | [Candau, Rasmussen, Clarke 2000](https://people.geog.ucsb.edu/~kclarke/ucime/banff2000/533-jc-paper.htm) |
| Batty, Longley, Fotheringham, DLA (diffusion-limited aggregation) | 1989 | urban-science model | seed -> random walkers stick on contact | y | n | paper only | (unverified; see Not found) |
| GDMC competition (Salge et al.) | 2018- | competition | entrants get an unseen map; any order | varies | n/a | per entry | [arXiv 1803.09853](https://arxiv.org/abs/1803.09853) |
| AgentCraft (GDMC 2021 entry) | 2021 | competition entry | find site -> first building + first road node -> agents act; busy nodes become roads | y | n | code at github.com/aith/agentcraft, licence not checked | [CEUR paper](https://ceur-ws.org/Vol-3217/paper3.pdf) |
| van der Staaij et al., GDMC 2022 winner | 2023 | competition entry + paper | terrain feature maps -> **one road piece at flattest spot** -> agents add roads, plots, walls per step (blueprint) -> plot types -> blocks | y (~60 steps) | n | not found | [arXiv 2309.10871](https://arxiv.org/abs/2309.10871) |
| dos Santos, Aranha, de Carvalho, LUTI agents | 2022 | paper + code | **road graph given first** -> agent develops cells near roads by accessibility | y | y (roads immutable) | not checked | [arXiv 2211.01959](https://arxiv.org/pdf/2211.01959) |
| Dwarf Fortress (town maps) | old talk | game | centre point -> walls/keep -> buildings spread outward; farmland shrinks with population | partly | n | proprietary | [DF Talk #13](https://mail.bay12games.com/media/df_talk_13_transcript.html) |
| Foundation (Polymorph Games) | 2019- | game | player paints zones; villagers' walking makes paths | y | y (forbidden zones) | proprietary | [Steam thread](https://steamcommunity.com/app/690830/discussions/2/1679189548052208992) |
| Manor Lords | 2024- | game | player draws roads, then curved plots along them | player-driven | n/a | proprietary | [Siliconera](https://www.siliconera.com/?p=1024606) |
| Banished | 2014 | game | player orders every building | player-driven | n/a | proprietary | [Wikipedia](https://en.wikipedia.org/wiki/Banished_(video_game)) |
| Townscaper (Oskar Stålberg) | 2020 | toy | player places blocks on irregular grid -> algorithm shapes houses | n | y (every block is user-placed) | proprietary | [Steam](https://store.steampowered.com/app/1291340/Townscaper/) |
| CityEngine "Grow Streets" | current | commercial tool | (existing or selected streets) -> major streets until quarters close -> minor streets | n | partly (extend only selected streets) | proprietary | [Esri docs](https://doc.arcgis.com/en/cityengine/latest/help/help-grow-a-street.htm) |
| Watabou Medieval Fantasy City Generator | 2017- | tool | not documented on fetched pages; see research-academic.md 2.7 | n | n (no fixed parts found) | GPL-3.0 (old source) | [GitHub](https://github.com/watabou/TownGeneratorOS), [itch](https://watabou.itch.io/medieval-fantasy-city-generator) |
| Watabou Village Generator | 2020- | tool | not documented; "highway"/"crossroads" tags put village at a road junction | n | partly (reroll village, keep terrain) | closed source | [itch](https://watabou.itch.io/village-generator), [1.6.4 devlog](https://watabou.itch.io/village-generator/devlog/840277/164-crossroads-and-numbered-houses) |
| Tarquiscani, Evolving City Generation | 2018-19 | hobby demo | not described | y | n | not stated | [itch](https://tarquiscani.itch.io/evolving-city-generation) |

### Notes per entry

**Weber et al. 2009.** Inputs are terrain, water and forest maps, an initial layout "ranging from a single street up to a larger (potentially already existing) city", land-use types, and user input that can change over time. Each step: expand major streets, develop the new quarters, blocks and lots, then compute traffic and land use. Major streets grow first; minor streets fill each quarter once its land use is known. A key idea: "let the simulation first create planned geometry and have the actual building lag by several time steps". Growth picks "grow seeds" by sampling existing nodes so streets appear in a believable historical order, unlike Parish & Müller. Street patterns are organic, grid or radial. Randomness is used (left/right choice, Gaussian angles); no seed handling is described. Pins: the starting layout and growth centres are user-given, and the run "can be edited during simulation". ([PDF](https://www.peterwonka.net/Publications/pdfs/2009.EG.Weber.UrbanSimulation.FinalVersion.pdf))

**Emilien et al. 2012.** Three steps: grow a "village skeleton" (house seeds plus roads), split land into parcels, then make 3D houses. The skeleton loop alternates: place one building seed, then connect it to the road network at once; new roads update the interest maps. A **growth scenario** is a user list of timed events: "change village type" (for example defensive, then farming after a war) or "seed n buildings of type B". Seeds are placed by random trial plus an "aggregation test" weighted by an interest value. Interest terms include sociability, worship (attraction to church), accessibility (preferred road distance), slope, water and fortifications. The village connects to the outside through user-given connection points at the region edge. ([PDF](https://perso.liris.cnrs.fr/egalin/Articles/2012-villages.pdf))

**Vanegas et al. 2009.** Not a history simulation: geometry and population/jobs are iterated until they agree. The paper's figure 1 is the best example of pins: the designer "constrains the downtown of the existing city and the forest area", the system adds people and builds new roads, parcels and buildings elsewhere, "leaving the original downtown unchanged". "Any design variable can be constrained or changed." Highways are sketched by the designer, not generated. ([PDF](https://www.cs.purdue.edu/homes/bbenes/papers/Vanegas09ToG.pdf))

**Lechner et al. 2006.** NetLogo grid of 12 m patches. Developer agents (residential, commercial, industrial) each tick prospect, build and check profit; road hierarchy has primary and tertiary roads, organic or grid. The artist paints to steer behaviour or set land use directly. Modern land use, not medieval. ([TR PDF](https://ccl.northwestern.edu/papers/2006/TR-2007-33.pdf))

**Groenewegen et al. 2009.** States plainly it "does not intend to model the city growth process but aims at generating only the present state". Order: user sets size and location; preliminary highways are drawn "around the core and leading out of the city"; then districts are placed at scored random candidates. Supports three European historic cores: mercantile, feudal, absolutist. A roads-before-districts order with no simulation. ([PDF](https://graphics.tudelft.nl/~rafa/myPapers/bidarra.EG09.pdf))

**Galin et al. 2011.** Regional, not intra-town. User places cities, towns and villages and their sizes; the system builds highways first, then major and minor roads, merging each level into the previous. Paths minimise a cost over slope, water, vegetation, curvature and a user-painted control map, and reuse earlier roads. Road class = smaller of the two settlement sizes it joins. Settlements are pinned by construction. ([PDF](https://perso.liris.cnrs.fr/eric.guerin/download/pg2011.pdf))

**Beneš, Wilkie, Křivánek 2014.** The clearest **roads-first** town generator. Argues "a city cannot be meaningfully simulated without taking its neighbourhood into account". Initialisation builds major roads between pairs of off-map neighbouring cities (Dijkstra over land and water), then every junction, bridge end and harbour becomes a growth nucleus. Main loop: grow minor roads (pattern chosen by historical period), simulate traffic between neighbour pairs, build new major roads when capacity runs out. Small settlements grow together into one town. Per-city input is minimal: a reusable "historical context". ([PDF](https://cgg.mff.cuni.cz/~jaroslav/papers/2014-cities/2014-cities-paper.pdf))

**Nishida et al. 2016.** User paints the target area and terrain, picks OpenStreetMap example networks, and may add manual roads such as highways. Growth starts from user or automatic seeds in a FIFO queue (first-in, first-out); arterial roads first, then local streets; each step uses either a patch copied from the example or statistics from it, adapting to terrain and "already generated roads". Then blocks, parcels, buildings. ([PDF](https://www.cs.purdue.edu/cgvlab/papers/aliaga/cgf15.pdf))

**TownSim (Song & Whitehead 2019).** This is the "Song et al." work found. Map is a pixel grid. Seed: a water patch, "a small stretch of straight road and building area close to the water", 100 agents. Each step agents consume, work, trade and rest; cells gain prosperity and traffic; above thresholds forest/greenfield becomes road or building. Update order is "re-randomizing the order each time", which is the opposite of what our determinism rules allow. Evaluated by directionality, connectivity, curvature. Python code at github.com/AugmentedDesignLab/town-sim-py (licence not checked). ([PDF](https://www.pcgworkshop.com/archive/song2019agentbased.pdf))

**Barthélemy & Flammini 2008.** Physics model of street growth. Start with a few connected centres; every few steps add new centres; each unconnected centre pulls one short road segment per step from the nearest network point; when two centres share a nearest point, the road heads along the sum of the two unit vectors. Effects: roads bend and meet perpendicularly; edges per node rises from about 1 (tree) to about 1.3 (loops). Centres placed with exponential falloff from a centre give a power-law block-area spread like Dresden. ([arXiv HTML](https://arxiv.org/html/0708.4360v2))

**SLEUTH.** Name = Slope, Land cover, Exclusion, Urban extent, Transportation, Hillshade (the six input layers). Four growth rules: spontaneous, new spreading centres, edge growth, road-influenced. Five coefficients: dispersion, breed, spread, slope resistance, road gravity; slopes over 21% cannot urbanise. Runs many Monte Carlo trials (repeated random runs) and reports the most common result plus an uncertainty map. Excluded cells never urbanise. Calibrated on real cities, not built for fantasy maps. ([Banff 2000 paper](https://people.geog.ucsb.edu/~kclarke/ucime/banff2000/533-jc-paper.htm))

**GDMC.** Yearly competition since 2018: write code that builds a settlement on an unseen Minecraft map. AgentCraft (2nd in 2021): picks a site, places a small cast of agents, one building and "the first node of a road network", then steps agents; a cell's prosperity rises when agents act there and a road is built when it passes a threshold; also writes a chronicle. ([CEUR](https://ceur-ws.org/Vol-3217/paper3.pdf)) The 2022 winner (van der Staaij et al., CoG 2023) plans a blueprint in memory: place one road piece at the flattest spot, then for about 60 steps let agents add roads, plots, walls and trees; agents can be switched on or off at chosen times to control order; each agent picks the best of a random subset of valid spots. ([arXiv](https://arxiv.org/abs/2309.10871), [PDF](https://arxiv.org/pdf/2309.10871))

**LUTI agents (dos Santos et al. 2022).** Starts "from an empty map consisting only of the road network graph"; road cells "become immutable"; one agent develops sites within 50 m of roads in order of accessibility. A pure roads-first, then fill, method. ([arXiv PDF](https://arxiv.org/pdf/2211.01959))

**Dwarf Fortress.** From an older developer talk (date and version not stated on the page): the town generator "picks out a spot" for the centre, respects "whatever wall or keep it's decided to put down", and spreads buildings outward over farmland. During world generation buildings are tracked only as counts; the map is built from those numbers on first visit. Tarn Adams mentions extending maps outward when population grows. Current-version behaviour: not verified. ([transcript](https://mail.bay12games.com/media/df_talk_13_transcript.html))

**Foundation.** A moderator states "a road building tool will not be added at any point"; paths form where villagers walk and fade when unused; paved upgrades for busy paths were planned, not confirmed. Forbidden zones block paths. ([Steam thread](https://steamcommunity.com/app/690830/discussions/2/1679189548052208992))

**Manor Lords.** Houses use "a 4 point plotting system that can curve along roads and other buildings". The player places roads; the game fits plots along them. No official developer source fetched. ([Siliconera](https://www.siliconera.com/?p=1024606); player discussion on medieval accuracy: [Steam](https://steamcommunity.com/app/1363080/discussions/0/4355619782840362614))

**Banished.** The player "must order the building of various structures". Road mechanics: not found in fetched sources. Nothing to borrow for automatic generation. ([Wikipedia](https://en.wikipedia.org/wiki/Banished_(video_game)))

**Townscaper.** Player plops coloured blocks on "the irregular grid"; the algorithm turns them into houses, arches, stairs, bridges. It is the extreme of pins: the user places everything coarse, the generator only does detail. That it uses wave function collapse is (unverified; only a paywalled interview snippet seen). ([Steam](https://store.steampowered.com/app/1291340/Townscaper/))

**CityEngine Grow Streets.** Can start empty, extend a street layer, or extend only selected streets. Major streets grow until they enclose a quarter; then minor streets subdivide it. Obstacle map and height map steer growth. Many values are random in ranges; no seed setting on the page. ([Esri docs](https://doc.arcgis.com/en/cityengine/latest/help/help-grow-a-street.htm))

**Watabou generators.** MFCG goal: "a nice looking map, not an accurate model of a city"; old source is GPL-3.0 and lacks water bodies. Village Generator: closed source; author says roads use the Parish-Müller approach or its successor; Shift+Enter "rerolls the village while keeping the same terrain"; "crossroads" tag puts the village at a highway junction. Whether roads are built before houses: not stated. ([MFCG itch](https://watabou.itch.io/medieval-fantasy-city-generator), [GitHub](https://github.com/watabou/TownGeneratorOS), [Village itch](https://watabou.itch.io/village-generator), [1.6.4 devlog](https://watabou.itch.io/village-generator/devlog/840277/164-crossroads-and-numbered-houses))

**Mas, Martin, Patow 2020 and Bulbul 2023.** Not re-fetched this session (Eurographics library returned 403). See `documentation/research/research-academic.md` 2.7: Mas runs a year-by-year loop of user-authored events (growth, wall creation and expansion, battles); Bulbul grows a town to a target population by weighted principles.

### What we could borrow

Ideas only. No code from TownSim, AgentCraft or any GPL/LGPL source.

1. **Roads-first as an option before S1 (Beneš 2014, Groenewegen 2009, LUTI 2022).** Trace the spec's entry roads across the S0 site as least-cost paths *before* the ward mesh. Their junctions, bridge ends and fords become extra fixed sites in S1's Voronoi, exactly like pinned landmark sites already are. Gates then fall where these roads cross the wall ring. S3 keeps them as the frozen primary layer. Cost: one new stage between S0 and S1; no simulation, so determinism is untouched.
2. **Growth phases as an ordered spec list (Emilien's growth scenario, Mas 2020 events, Weber's initial layout).** A spec could list phases such as "founding core", "wall ring 1", "suburb along north road", "wall ring 2". Each phase runs S1-S3 only on new land, and everything from earlier phases enters as pins. This is our rule "landmark N+1 never moves 1..N" applied to phases. RNG key gets a phase id, for example `("macro", phase_id, cell)`.
3. **Interest maps as rule-pack data for S1 and S2 (Emilien 2012).** Score candidate cells as a weighted sum of slope, distance to water, distance to road, attraction to church or market, distance to wall. The weights live in the rule pack (rules are data, ADR-017). Pick the best of a fixed, hashed candidate list instead of Emilien's random retry loop, so it stays deterministic.
4. **Grow S3 secondary streets in a fixed order (Weber 2009, CityEngine, Nishida 2016).** Major streets first until they close quarters, then minor streets inside each quarter, with seeds processed from a queue sorted by stable id. TownSim and AgentCraft shuffle agent order each step; we must not.
5. **Freeze-and-regrow controls (Vanegas 2009, Village Generator).** "Constrain this area, regenerate the rest" and "reroll the town, keep the terrain" map onto our per-stage RNG streams: changing only the S1-S5 seed while S0 stays fixed is already free in our design and worth exposing as a CLI flag.
6. **Street statistics for property tests (Barthélemy 2008, TownSim).** Check that junction angles cluster near 90 degrees for organic districts and that edges per node (K/N, which is half the average degree ⟨k⟩ = 2K/N used in `streets.md`) lies roughly between 1.0 and 1.3 for small towns, as reported by the cited models (an exact test bound is a later design choice); TownSim's directionality, connectivity and curvature are ready-made metrics for golden and expressive-range checks.

Not recommended: full agent or CA simulation (TownSim, AgentCraft, SLEUTH, Lechner). All rely on many random updates in shuffled order, run for minutes, and give little direct control over pinned landmarks.

### Sources

All fetched this session (PDFs read via text extraction):

- https://www.peterwonka.net/Publications/pdfs/2009.EG.Weber.UrbanSimulation.FinalVersion.pdf
- https://perso.liris.cnrs.fr/egalin/Articles/2012-villages.pdf
- https://www.cs.purdue.edu/homes/bbenes/papers/Vanegas09ToG.pdf
- https://ccl.northwestern.edu/papers/2006/TR-2007-33.pdf
- https://graphics.tudelft.nl/~rafa/myPapers/bidarra.EG09.pdf
- https://perso.liris.cnrs.fr/eric.guerin/download/pg2011.pdf
- https://cgg.mff.cuni.cz/~jaroslav/papers/2014-cities/2014-cities-paper.pdf
- https://www.cs.purdue.edu/cgvlab/papers/aliaga/cgf15.pdf
- https://www.pcgworkshop.com/archive/song2019agentbased.pdf
- https://arxiv.org/abs/0708.4360 and https://arxiv.org/html/0708.4360v2
- https://people.geog.ucsb.edu/~kclarke/ucime/banff2000/533-jc-paper.htm
- https://arxiv.org/abs/1803.09853
- https://arxiv.org/abs/2103.14950
- https://arxiv.org/pdf/2108.02955
- https://ceur-ws.org/Vol-3217/paper3.pdf
- https://arxiv.org/abs/2309.10871 and https://arxiv.org/pdf/2309.10871
- https://arxiv.org/pdf/2211.01959
- https://mail.bay12games.com/media/df_talk_13_transcript.html
- https://steamcommunity.com/app/690830/discussions/2/1679189548052208992
- https://www.siliconera.com/?p=1024606
- https://steamcommunity.com/app/1363080/discussions/0/4355619782840362614
- https://en.wikipedia.org/wiki/Banished_(video_game)
- https://store.steampowered.com/app/1291340/Townscaper/
- https://doc.arcgis.com/en/cityengine/latest/help/help-grow-a-street.htm
- https://github.com/watabou/TownGeneratorOS
- https://watabou.itch.io/medieval-fantasy-city-generator
- https://watabou.itch.io/village-generator
- https://watabou.itch.io/village-generator/devlog/840277/164-crossroads-and-numbered-houses
- https://tarquiscani.itch.io/evolving-city-generation

### Not found

- **Batty, Longley, Fotheringham 1989 DLA paper** (Environment and Planning A): UCL page returned 403; known only from search snippets (tested on Taunton). (unverified)
- **Batty, "Cities and Complexity"** (2005 book): no fetchable source found.
- **"Song et al. on historical town generation"**: no paper with that description found; TownSim (Song & Whitehead 2019) is the closest match.
- **Miskolc "simulated morphogenesis"** (Heritage 2024, DOI 10.3390/heritage7100276): MDPI and DOAJ returned 403. (unverified)
- **Mas 2020, Bulbul 2023, Weber/Galin/Nishida Eurographics library pages**: 403; author PDFs used instead where available.
- **Songs of Syx**: no source on how roads, rooms or settlements are generated.
- **Banished road mechanics**: not in fetched sources.
- **Manor Lords official plot rules**: Hooded Horse wiki returned 403; only press and player posts.
- **Oskar Stålberg technical talks**: no transcript fetched; WFC in Townscaper (unverified).
- **Watabou build order** (MFCG, Village Generator): not described on fetched pages.
- **Seed / determinism handling** in Weber, Emilien, Beneš, Nishida, CityEngine: not described in the fetched text.
- **2024-2026 papers on historically informed or growth-based city generation**: none found beyond the 2023 GDMC paper.
- **Licences of TownSim, AgentCraft, autoplanner, Evolving City Generation repos**: not checked or not stated.
