> Written 2026-10-19 before any decision was taken. A claim here is binding only when a
> DECISIONS.md entry cites it.

# Prior art survey: fantasy city / town map generators and VTT export

Scope: tools relevant to a new open-source, specification-driven fantasy CITY overview-map generator for tabletop RPGs. Researched 2026-10-19 via web search and direct fetches of official pages, repos and GitHub API. Items I could not confirm from a primary source are marked **(unverified)**.

## 1. Comparison table

| Tool | What | Stack | License / source | Core algorithm | User inputs / constraints | Output | Extensible? | Activity | Best at | Cannot do |
|---|---|---|---|---|---|---|---|---|---|---|
| Watabou MFCG (web) | Procedural medieval city overview map | Haxe + OpenFL (HTML5) | Free to use; closed source (only old snapshot open) | Spiral-seeded Voronoi + Lloyd relaxation -> patches -> wards; walls, citadel, streets via graph shortest paths | size, seed, walls, citadel, urban_castle, plaza, temple, shantytown, green, farms, river, coast, sea, gates (URL params); warp tool; feature "required" toggles; styles | PNG, SVG, JSON (JSON -> City Viewer 3D) | URL params only; no API | Active (0.11.x in 2024, updates Feb 2025; Perilous Shores/Dwellings updated through 2026) | Best-looking, most "fantasy-ish" city maps; Azgaar hand-off | No pinned landmarks/districts from a specification; no VTT/Foundry export; no code access to current version |
| watabou/TownGeneratorOS | Open snapshot of MFCG | Haxe (OpenFL, msignal) | GPL-3.0 | Same as above (documented below) | nPatches, seed; plaza/citadel/walls randomly chosen | Rendered canvas (no exporter in repo) | Fork it; GPL copyleft | Dormant: last push 2021-01-09; 1,945 stars | Reference implementation of the ward/patch algorithm | No water bodies, no options UI, no exports (README says so) |
| ProbableTrain MapGenerator | Tensor-field "American" city map generator | TypeScript, HTML canvas, Gulp | LGPL-3.0 (repo COPYING.LESSER) | Chen et al. 2008 tensor fields -> RK4 streamline tracing -> graph -> polygons -> lots | grid/radial field placement, dsep/dstep (road density), water noise, parks count, building size, styles | PNG, SVG, STL, heightmap PNG | Edit `colour_schemes.json`; no API | Dormant: last push 2024-05-09; 1,475 stars | Smooth, controllable road networks from a direction field | Medieval/fantasy look, walls, districts, semantics; no VTT |
| Azgaar FMG | World/regional map generator with burgs (settlements) | JS (migrating to TS, Vite), D3, SVG | MIT | Voronoi/polygon map (O'Leary, Red Blob inspired); burgs placed by population/suitability | Many; burgs have population, capital, port, citadel, walls, plaza, temple, shanty flags; emblem | PNG/JPEG/SVG, JSON (full/minimal), GeoJSON (cells, routes, rivers, markers, zones), burgs CSV, .map save | Large, open code; URL params; third-party Foundry importer (azgaar-foundry) | Very active: pushed 2026-10-19; 6,117 stars | World-scale context; hands burgs to MFCG with flags | Does not draw city street maps itself |
| Dungeon Alchemist | AI-assisted 3D battlemap maker | Unity app (Steam) | Commercial, EUR 37.99; closed | In-house procedural room filling (explicitly "no generative AI / LLMs") | Theme per room, draw rooms, terrain tools | JPG/MP4/WebM + UVTT data file (walls, doors, lights); Foundry/Roll20/FGU | Steam Workshop; DA Level Importer module for Foundry v14 | Early Access since 2022-03-31; planned 1.0 late 2026 | Fast furnished battlemaps with lighting | Not a city/overview generator |
| Dungeondraft | Battlemap editor | Godot (GDScript) | Commercial USD 19.99; closed | Manual drawing | Manual | PNG/JPG/WEBP, .dd2vtt (origin of UVTT) | Modding API (GDScript): tool scripts and export-format scripts; asset packs | Maintained (1.1.x; modding since 1.1.0.0) | De-facto UVTT reference exporter | No procedural city generation |
| Wonderdraft | World/region map painter | Godot-based desktop app (unverified engine) | Commercial USD 29.99; closed | Manual painting + landmass generators | Manual | PNG/JPG (PDF/multiplane per 3rd-party) | Custom symbols/themes | Mature; version not shown | Painterly regional maps | City layout generation |
| Inkarnate | Browser map painter (world/region/city/battle) | Web app | Freemium; closed | Manual asset painting | Manual | PNG up to 8K; "precise pixels-per-grid VTT export" (2024) | Cloneable public maps | Active (Base plan July 2026; v2.0 planned) | Huge asset library, city stamps | No generation; no UVTT/dd2vtt |
| CC3+ City Designer 3 | CAD-style city add-on for CC3+ | Windows (FastCAD) | Commercial GBP 27.95; closed | Random Street / House Builder tools; "Random City" wizard (2017 annual) | Street tool settings, City Wizard options | FCW native; PNG/JPG/BMP export (SVG via workaround, unverified) | Macros; annuals; Watabou import style packs | Maintained; CC4 in prep | Precise, printable, indexed city maps | Specification-driven whole-city generation; no VTT walls |
| Worldographer / Cityographer | Hex-world + settlement generator | Java desktop | Commercial (W2025 crowdfunded 2024); closed | Random settlement with street-layout options, building data (residents, inventories) | Population, tech level, river, coast, street layout, era | PNG and native format (exports not documented online) | Icon packs | Worldographer 2025 released after Sept 2024 campaign | Generates a city AND its inhabitants/shops | Aesthetic quality; no VTT walls; closed |
| Dungeon Scrawl | Browser dungeon/town sketcher by probabletrain (Roll20-owned) | Web app | Free + Pro; not open source (unverified) | Manual | Manual | PNG, SVG, PDF (WIP), .uvtt (Pro, Sept 2026) | Roll20 integration | Active | Instant clean line-art dungeons | City generation |
| Illwinter's Floorplan Generator | Grid-based dungeon/wilderness floorplans | Native desktop (Win/Linux/Mac), Steam app 630900 | Commercial ~USD 8; closed | Random dungeon rules editor; fractal terrain | Rules, ASCII import | Multi-page PDF, PNG, 3D-look | Own icons, Workshop | Low activity | Cheap printable dungeon pages | Towns/cities (not a feature) |
| Python OSS (Ewael, hiive, cityGen) | Small Python medieval city generators | Python (shapely/geopandas) | Unstated/none | Voronoi -> inner/outer city -> categories | population, density, walls, castle, river | JSON + matplotlib viewer | Fork | Dormant, ~1 star | Shows the simplest viable pipeline | Quality, VTT, water |
| LLM/AI (2024-2026) | CityGenAgent, CityGen, COHO, text-to-image sites | Research / SaaS | Mostly no code; SaaS closed | LLM-written block/building programs (CityGenAgent); GNN/VAE/diffusion (others); diffusion images (SaaS) | Natural-language prompt | 3D layouts / raster images | None for TTRPG | 2024-2026 papers | Prompt-to-layout idea | Editable, VTT-usable fantasy city data |

## 2. Per-tool findings

### 2.1 Watabou: Medieval Fantasy City Generator (MFCG)
- Pages: https://watabou.itch.io/medieval-fantasy-city-generator , https://watabou.github.io/city.html , FAQ https://watabou.github.io/faq.html
- What: browser generator of medieval city overview maps; "aims for a nice looking map, not an accurate city model." Started as a 2017 r/proceduralgeneration challenge.
- Stack: Haxe + OpenFL compiled to HTML5 (FAQ: "all generators are written in Haxe and OpenFL").
- License: free; maps may be copied, modified, used in commercial adventures, attribution appreciated not required. Current code is closed; FAQ: author "usually doesn't publish code unless a project is finished"; only an early version is on GitHub.
- Algorithm: see TownGeneratorOS below (the live version adds water bodies, district view, alleys, roofs, forests, warp tool).
- Inputs: context-menu options; "setting which features such as city walls and rivers a city must include"; warp tool; procedural district names. URL parameters (community-documented at https://forum.profantasy.com/discussion/comment/93506 and used by Azgaar): `size` (population-ish, up to ~500), `seed`, `name`, `hub`, `random`, `elevation`, `citadel`, `urban_castle`, `temple`, `plaza`, `shantytown`, `green`, `farms`, `river`, `coast`, `sea`, `walls`, `gates` (0/1 toggles, `gates`=count; Azgaar passes roads=-1 meaning "auto"). **(URL param list unverified against current build.)**
- Output: PNG, SVG, JSON. JSON feeds City Viewer (3D). Azgaar FMG integration both directions (FMG -> MFCG with burg flags; MFCG -> FMG via `from=MFCG&size&coast&port&river`, see https://github.com/Azgaar/Fantasy-Map-Generator/wiki/URL-parameters).
- Extensibility: none beyond URL params and JSON export. Third-party "Watabou Bridge" Unreal plugin wraps the web versions (Fab listing).
- Activity: devlog 0.10.1 (Feb 2024) ... 0.11.4 forests (Oct 2024), "Three generators updated" (Feb 2025). Planned: islands/ponds/confluences, more organic district streets.
- Best: aesthetics and the ward/wall/citadel vocabulary everyone expects. Cannot: accept a written specification (pinned landmarks, named districts, adjacency rules), export VTT scene data, or be embedded/extended in code.

### 2.2 watabou/TownGeneratorOS
- Repo: https://github.com/watabou/TownGeneratorOS (GPL-3.0, Haxe, 1,945 stars, last push 2021-01-09, created 2017-08-03; README: lacks water bodies, options UI, "some smaller ones").
- Algorithm (read from `Source/com/watabou/towngenerator/building/Model.hx`):
  1. `nPatches` sets size (6 small town, 10 large town, 15 small city, 24 large city, 40 metropolis). `plazaNeeded`, `citadelNeeded`, `wallsNeeded` are random booleans; `seed` fixes RNG. `build()` is retried on failure (e.g. "Bad citadel shape!" if citadel compactness < 0.75).
  2. `buildPatches`: `nPatches*8` points on a jittered spiral -> Voronoi -> 3 Lloyd relaxations on central points -> regions sorted by distance; region 0 = plaza (if needed), region `nPatches` = citadel; regions < nPatches are "inside city" / inside walls.
  3. `optimizeJunctions`: collapse edges < 8 units.
  4. `buildWalls`: CurtainWall around inner patches (avoiding citadel), towers, gates; discard patches > 3x wall radius; Castle with own wall on citadel.
  5. `buildStreets`: Topology graph; shortest path from each gate to nearest plaza corner (or center) = streets; roads from gates outward; tidy, dedupe, chain into arteries, smooth.
  6. `createWards`: Market on plaza; GateWards near gates (p=0.5 walled / 0.2); WARDS list (CraftsmenWard most common, MerchantWard, Cathedral, AdministrationWard, PatriciateWard, MilitaryWard, Slum, Market, Park) lightly shuffled; each ward claims best patch via static `rateLocation`; leftovers -> Slum; outside walls -> Farm (p=0.2, compactness >= 0.7) else plain Ward.
  7. `buildGeometry`: each ward subdivides its patch into building footprints.
- Output: on-screen render only. Extensibility: forkable (GPL copyleft matters for your license choice). Ports exist (C# `runegri/Town`).
- Best: compact, readable reference for the "wards on Voronoi patches" approach. Cannot: water, options, export; code is 2017-era.

### 2.3 ProbableTrain MapGenerator
- Repo: https://github.com/ProbableTrain/MapGenerator ; live: https://maps.probabletrain.com ; itch: https://probabletrain.itch.io/city-generator
- Stack: TypeScript, HTML canvas, RoughJS (sketch style), simplify-js, isect; Gulp build. License LGPL-3.0 (GitHub API). 1,475 stars; last push 2024-05-09 (dormant).
- Algorithm (docs/algorithmoverview.md): follows Chen, Esch, Wonka, Mueller, Zhang, "Interactive Procedural Street Modeling", SIGGRAPH 2008 (https://www.sci.utah.edu/~chengu/street_sig08/street_project.htm). Tensor field = blend of grid and radial basis fields; roads = RK4 streamlines along major/minor eigenvectors with `dsep` spacing and `dstep`; major roads first, minor after; streamlines simplified and intersected into a graph; polygons found by right-turn tracing (assumes no dead ends); lots by shrinking polygons and recursive longest-edge splits; parks = polygons of the major graph; water = one streamline through a noisy field (Simplex), tensor degenerates to zero in water so roads stop.
- Inputs (docs/usageguide.md): `setRecommended` (4 grids + 1 radial), add/remove grid/radial, per-field size/decay/angle; water noise params, `simplifyTolerance`; main/major/minor road generation; `addBuildings`, min building size, `shrinkSpacing`; `numParks`; styles (Google/Apple-like, Default, Heightmap); `highDPI`, `imageScale`.
- Output: PNG, SVG, STL zip (one file per component), heightmap PNG.
- Extensibility: no API; colour schemes via `src/colour_schemes.json` and rebuild.
- Best: coherent, controllable road geometry; the tensor-field idea maps directly to "spec says: radial old town around the keep, grid new town by the docks". Cannot: walls, gates, districts with semantics, medieval irregularity, VTT output.

### 2.4 Azgaar's Fantasy Map Generator
- Repo: https://github.com/Azgaar/Fantasy-Map-Generator (MIT, 6,117 stars, pushed 2026-10-19; JS migrating to TS, Vite/Vitest/Playwright, Electron desktop, Docker). Wiki: https://github.com/Azgaar/Fantasy-Map-Generator/wiki
- Algorithm: Voronoi polygon map (inspired by O'Leary and Red Blob Games), heightmap, climate, biomes, cultures, states, religions, routes, burgs.
- Burg handling: burgs auto-placed by population/suitability or added manually; each burg carries name, population, group (city/town...), capital, port, citadel, walls, plaza, temple, shanty-town flags, emblem, elevation, temperature, lat/lon (burgs CSV export, wiki GIS-data-export). Burg editor links to Watabou MFCG/Village Generator passing those flags (community-documented; FMG sets roads/gates to -1 = auto; size = population/1000) **(exact code path unverified: raw `modules/ui/burg-editor.js` 404s after the repo restructure)**. Reverse link documented: `from=MFCG`, `size`, `coast`, `port`, `river`, burg identifier in seed.
- Export: PNG/JPEG/SVG; JSON (full, minimal, pack cells, grid cells); GeoJSON (cells with identifier/height/biome/type/population/state/province/culture/religion/neighbors; routes; rivers; markers; zones); burgs CSV; `.map` save/load; URL params (`seed`, `options`, `width/height`, `scale`, `burg`, `layers`, `maplink`).
- Foundry: unofficial importer https://github.com/Ethck/azgaar-foundry (needs FULL JSON + image; creates journals; v0.4.0).
- Best: the world-level context and the cleanest existing "burg -> city generator" contract. Cannot: draw streets/buildings; its city hand-off is a one-way URL with ~15 booleans, no district/landmark detail.

### 2.5 Other Watabou tools (Procgen Arcana)
- Village Generator https://watabou.itch.io/village-generator : Haxe/OpenFL; tags (island, estuary, grove, highway, dense, district), map size/aspect, layers, styles; Shift+Enter rerolls village keeping terrain; PNG/SVG/JSON (JSON has water polygons, roads, palisades); road network uses Parish-Mueller (L-system) approach per author; closed source; 1.6.9 ~Sept 2026. Author says village and city generators are "very different" codebases.
- Perilous Shores https://watabou.itch.io/perilous-shores : regional maps (settlements, dangers, roads, sea routes, hex grid); Blueprint editor (1.8.0, Mar 2026), walled towns (1.9.1, Jul 2026), JSON save/load (2.0.0); closed source, "no generative AI used".
- One Page Dungeon https://watabou.itch.io/one-page-dungeon : tags (large, winding, compact, multi-level...), seeds, water level; PNG/SVG/JSON/Markdown notes; closed source ("not open source at this moment"); 1.2.7 Jan 2026; no built-in VTT export (third-party Foundry module uses its maps).
- Toy Town https://watabou.itch.io/toy-town : 3D prototype (Haxe/OpenFL/Away3D) with an old MFCG inside; cannot import MFCG maps; superseded in practice by City Viewer (drag MFCG JSON in).
- Dwellings https://watabou.itch.io/dwellings : house floorplans; JSON export 1.4.0 (Nov 2025), roofs 1.4.1 (Dec 2025); UVTT export requested, not available; candidate to replace Procgen Mansion for exploring MFCG buildings.

### 2.6 Dungeon Alchemist
- https://www.dungeonalchemist.com/ , Steam https://store.steampowered.com/app/1588530/ (EUR 37.99, Win/Mac/Linux, Early Access since 2022-03-31, 1.0 planned late 2026, 93% positive of 1,934 reviews).
- Algorithm: draw rooms, pick theme, in-house procedural filling of walls/floors/objects/lighting; developer states "no generative AI or LLMs". Text-to-map is only a feature request (featurebase).
- Output: JPG/MP4/WebM image + separate data file with walls/doors/lights (UVTT), animated exports for Foundry/FGU; Foundry v14 "Dungeon Alchemist Level Importer" (multi-floor to Scene Levels). Steam Workshop.
- Best: furnished, lit battlemaps in minutes. Cannot: city-scale overview maps.

### 2.7 Dungeondraft
- https://dungeondraft.net/ (USD 19.99 one-time, Win 10 x64 / Ubuntu, no ARM64). Godot engine (godotengine.org showcase).
- Export: PNG/JPG/WEBP and `.dd2vtt` Universal VTT; printer-friendly filter.
- Modding API https://megasploot.github.io/DungeondraftModdingAPI/ : GDScript mods = folder with `.ddmod` JSON manifest + `.gd` scripts, each with `start()`; **tool scripts** (new toolbar tools, UI panels) and **export-format scripts** (embed extra data into PNG/JPG/WEBP or "extract data such as line-of-sight information to build custom VTT export formats"); exposed classes Editor, World, Level, Tool, Toolset, ToolPanel, Toolbar, Exporter, Walls, Objects, Pathways, ObjectLibraryPanel, PathLibraryPanel, AssetPack, AssetInstance; per-node data via `Script.SaveDataToMapNode`; mods since 1.1.0.0 (Mar 2023).
- Relevance: a generator could emit a Dungeondraft-importable mod/tool, but there is no documented "import map JSON" path; the practical route is emitting `.dd2vtt` directly (format below). Cannot: generate anything procedurally itself.

### 2.8 Wonderdraft
- http://wonderdraft.net/ : USD 29.99 one-time, Win 10 x64 / Ubuntu / Surface Pro (no ARM64), DRM-free, offline; landmass generators, coastline smoothing, rivers/roads, symbol painting, labels/themes, 512-8192 px. Export PNG/JPG (PDF and "multiplane" layered export per 3rd-party, unverified). No city generation, no VTT data, no API (custom symbols/themes only).

### 2.9 Inkarnate
- https://inkarnate.com/ : browser editor, free tier + Base (USD 5.99/mo or 29.99/yr, July 2026) / Creator / Studio (commercial use); 30K+ assets; exports up to 8K; "precise pixels-per-grid VTT export" (Mar 2024 changelog). No generation; no UVTT; AI: company allowed then banned AI art in its marketplace (Oct 2025, Blizzardwatch) - no generative feature of its own (3rd-party claims of "Inkarnate AI" **unverified**).

### 2.10 Campaign Cartographer 3+ / City Designer 3
- https://www.profantasy.com/products/cd3.asp : GBP 27.95 add-on for CC3+ (Windows); 2,000+ symbols, House Builder, Street Builder ("whole street of different buildings in two clicks"), Street Indexer (hyperlinked index), Floorplan Creator; Random Street tool is "the backbone" of large towns (https://rpgmaps.profantasy.com/random-city-generator/); separate "Random City" wizard in the 2017 Annual; 2024 Annual style pack for importing Watabou output. Native FCW; raster export; no VTT wall data. CC4 in preparation (random dungeon beta).

### 2.11 Worldographer (Inkwell Ideas) / Cityographer
- https://inkwellideas.com/worldographer/ , https://inkwellideas.com/2018/10/worldographer-city-generation-editing-preview/ , Worldographer 2025 campaign (Sept 2024). Java desktop (Win/Mac/Linux installers). Ports Cityographer: inputs tech level, population, river, coast; generates city/town/village/battlemat with street-layout options, plus data for every building (residents, shop inventories/prices, tavern patrons); 2025 adds modern/sci-fi/western/post-apoc icons. Exports not documented online **(unverified; PNG assumed)**. Best: city + inhabitants from a few sliders. Cannot: modern look, VTT walls, open code.

### 2.12 Dungeon Scrawl
- https://dungeonscrawl.com/about , https://blog.dungeonscrawl.com/uvtt-export : free browser tool by Keir (probabletrain), Roll20-owned since 2023; v2 rebuild; PNG/SVG, PDF WIP, high-res Pro; `.uvtt` export (Pro, 9 Sept 2026) with auto-detected walls/doors/lights, default 70 px/cell. Not open source (a CC0 question in comments, **unverified**). No city generation.

### 2.13 Illwinter's Floorplan Generator
- https://www.illwinter.com/floorplan , Steam app 630900 (~USD 8; Win/Linux/Mac). Grid-based dungeon and wilderness floorplans, ~50 blending terrains, fractal caves/rivers, random dungeon generator with user-editable rules, ASCII layout import, own icons; export multi-page PDF, PNG (Roll20), 3D-look. No town/village generator feature listed (screenshots show inn/alleys/castle only).

### 2.14 Small open-source generators
- https://github.com/Ewael/medieval_city_generator (Python 3): points -> Voronoi -> drop far border cells -> inner/outer city -> categories (forest, lake, houses, mansions, parks, cathedral/castle) -> streets around districts -> optional walls; CLI `--population`, `--density`, `--walls`, `--castle`, `--river` (WIP); JSON + viewer. No license stated; dormant.
- https://github.com/hiive/Medieval-Map-Generation (Python, fork of LAVS-TM): `City(population, density, has_walls, has_castle, has_river)`; JSON + PyQt viewer; 2021-era deps.
- https://github.com/jmespadero/cityGen (Python + Blender; SVG step images).
- https://github.com/runegri/Town (C# port of watabou), https://delca.itch.io/town (Unity, MIT, displaced Voronoi), https://github.com/Grzybojad/ProceduralCityGeneration (C++/Unreal), https://github.com/TobieD/City-Generator (C#/Unity).
- Text-only settlement generators (no map): Eigengrau's Essential Establishment Generator (JS/Twine), opendnd/dominia, Donjon, fantasytowngenerator.com (closed web app with NPC schedules; page fetch returned empty).

### 2.15 AI / LLM-driven (2024-2026)
- Research: CityGenAgent "Imagine a City" (arXiv 2602.05362, Feb 2026): natural-language -> Block Program + Building Program via two fine-tuned models (SFT + RL with spatial-alignment and visual-consistency rewards); outputs constrained, editable programs; no code release found. CityGen (arXiv 2312.01508, CVPR-W 2025) infinite controllable layouts (diffusion); COHO (2407.11294) graph masked autoencoder; "Controllable Generation of Large-Scale 3D Urban Layouts" (2509.23804) GNN + CVAE; "Remote-Sensing City Layout Extraction with MLLM" (2608.16484) frames layout as constrained code generation by an MLLM. None targets fantasy/TTRPG or 2D overview maps.
- Hobby: https://github.com/FarmerBarnyard/campaign-hub (Oct 2026): Ollama-backed campaign generator; town maps drawn by a separate deterministic "mapgen" render service seeded from the location title and overworld context (coast/lakes/rivers/biomes) - i.e. LLM writes the lore, a classic procedural renderer draws the map. No license.
- SaaS "text to fantasy map" (fantasygen.net, musely.ai, zsky.ai, imgkits): diffusion images from a prompt; not editable, no layers, no VTT data.
- Foundry side: fvtt-mcp servers (e.g. fvtt-mcp-molten5e `create-scene`, and the foundry-mcp `generate-map` tool present in this session) create scenes from an image + sidecar walls/lights JSON, which is a plausible delivery path for a generator.

### 2.16 Foundry VTT import: Universal VTT (.dd2vtt / .df2vtt / .uvtt)
- Specification (Arkenforge): https://arkenforge.com/universal-vtt-files/ . JSON with:
  - `format` (decimal version, e.g. 0.3)
  - `resolution: { map_origin:{x,y}, map_size:{x,y} (in grid squares), pixels_per_grid }`
  - `line_of_sight`: array of polylines (arrays of `{x,y}` in grid units) -> Foundry walls
  - `objects_line_of_sight`: same, for objects
  - `portals`: `[{ position:{x,y}, bounds:[{x,y},{x,y}], rotation (radians), closed, freestanding }]` -> doors
  - `environment: { baked_lighting, ambient_light (hex) }`
  - `lights: [{ position, range, intensity, color (hex), shadows }]`
  - `image`: base64 PNG or WEBP
  - Units are grid squares; custom tags allowed; no layers/tokens/animation.
- Importers: Foundry "Universal Battlemap Importer" by Moo Man, https://foundryvtt.com/packages/dd-import , repo https://github.com/moo-man/FVTT-DD-Import (MIT; v6.1.1, verified Foundry 14; accepts Dungeondraft/DungeonFog/Arkenforge/AutoWall exports; options fidelity and wall offset; stores image in User Data or S3). Also Quick Battlemap Importer (image/video + optional walls/lights JSON, v12-14). Roll20 (UniversalVTTImporter script, official), FGU and MapTool native.
- What a generator must emit for Foundry via UVTT: a rendered PNG/WEBP of the whole map, a chosen `pixels_per_grid` (70-100 typical; Dungeon Scrawl defaults 70), `map_size` matching image/ppg, `line_of_sight` polylines for walls/buildings (keep count modest - Foundry performance degrades with thousands of walls, so for a city overview emit only major walls, or none), `portals` for gates/doors, optional `lights`. Caveat from users: wall data transfers reliably, lights less consistently across tools.
- Alternative: emit a native Foundry Scene JSON (Scene "Import Data" right-click; schema `SceneData` with `walls[]` (`c:[x1,y1,x2,y2]`, door, move/sight flags), `lights[]` (AmbientLightData), `notes[]` (NoteData map pins linked to JournalEntry), `tiles[]`, `grid:{type,size,distance,units}`, `background.src`, `padding`). This is the only route that supports **map notes/pins for landmarks and districts**, which UVTT cannot express. **(Field names from v12/v13 API docs and memory; verify against your target Foundry version.)** Caveat: Scene Import Data overwrites an existing scene; a tiny module or MCP tool (e.g. fvtt-mcp `create-scene`) can create a fresh one. Keep coordinates in absolute canvas pixels and scene padding 0 when generating.

## 3. What to steal, and the gap that remains

Steal:
1. The TownGeneratorOS pipeline shape (patches -> walls/citadel -> gate-to-plaza streets -> ward assignment via `rateLocation` scoring -> per-ward footprints) and its ward vocabulary; it is GPL-3.0, so re-implement rather than copy if you want a permissive license.
2. ProbableTrain's tensor-field + streamline road tracing (LGPL, TypeScript) for road geometry that can be *steered*: a specification's "radial around the keep / grid in the new quarter / roads follow the river" becomes field basis functions, which is exactly the control knob the Voronoi approach lacks.
3. Azgaar's burg contract (population, capital, port, citadel, walls, plaza, temple, shanty, coast, river, elevation, emblem) as the minimum input schema, so FMG worlds can feed the generator; mirror MFCG's URL parameter names for compatibility.
4. Watabou's export trio (PNG, SVG with semantic groups, JSON with polygons) plus the Village Generator's "reroll village, keep terrain" idea (separate seeds per layer).
5. Worldographer/Cityographer's notion that a city is data (buildings with occupants) not just pixels; and Dungeon Alchemist's "no generative AI, procedural with explicit rules" positioning for community acceptance.
6. UVTT as the first VTT target (dd-import is maintained to Foundry 14), with native Scene JSON + Notes as the second to carry landmark pins.

Gap no surveyed tool fills:
- None accepts a **written requirements specification** (population, named districts with adjacency, pinned landmarks at positions, "temple north of the market", walls with N gates facing given roads, river entering west) and solves for a layout satisfying it. MFCG has ~15 booleans and a warp brush; ProbableTrain has fields but no semantics; Python toys have 5 flags.
- None is simultaneously open source, actively maintained, medieval-looking, and VTT-aware: TownGeneratorOS (dormant 2021, GPL, no water), ProbableTrain (dormant 2024, modern grids), Azgaar (active, but no streets).
- No tool exports district/landmark metadata to Foundry (notes/journals); all VTT paths stop at image + walls + lights.
- LLM work (CityGenAgent etc.) shows "NL -> constrained layout program" is viable, but nothing targets 2D fantasy city overview maps; a specification-parser front end (LLM optional) feeding a deterministic constraint-aware procedural core is the open niche.
