# Research: terrain shapes and terrain drawing for milestone 2 (pass 1)

Date: 2026-11-09. Scope: S0 "terrain straight from the specification's shapes, no noise" on a
site 500 m to 5 km square, drawn as SVG and PNG. This report follows ADR-023: outside sources
keep their uncertainty, each has a URL, and nothing here is a decision. Each question ends with
numbered options for Jeffrey.

How the sources were read: web search plus fetching the page or the source file. Where a page
could not be fetched, that is named. "My arithmetic" marks numbers I computed, not a source.

## 1. Coast shapes and harbour proportions

### 1.1 How generators place a coast

**Watabou, Medieval Fantasy City Generator (MFCG).**
- Devlog 0.4.0 (2017): the generator "can place a coast next to a city"; it first marks part of
  the terrain as water, then builds wards so none fall in water. The coastline is rough and,
  in the author's words, "almost always" lacks bays and peninsulas. The wall facing the water
  is removed and seafront blocks can become docks with piers. At that time the coast was
  random ("2 out of 3 layouts are coastal").
  https://watabou.itch.io/medieval-fantasy-city-generator/devlog/4923/040-coastal-cities
- Devlog 0.4.1: coast became an on/off option; no direction or shape control is mentioned.
  https://watabou.itch.io/medieval-fantasy-city-generator/devlog/6271/041-options
- A `sea` URL parameter as a direction is reported only by a Reddit user (weak source, not
  fetched successfully; seen in a search snippet): `sea=0` puts the sea at about 3 o'clock,
  `sea=0.5` at 12 o'clock, so it reads as a fraction of a full turn. I could not confirm this
  in any official text.
- Users asked for edge control for river and coast; I found no evidence it was implemented
  (search snippet from the MFCG itch.io comments, weak source).

**Azgaar's Fantasy Map Generator** (read from the source file
`src/model/physical/heightmap-generator.ts`, master branch, 2026-11-09;
https://github.com/Azgaar/Fantasy-Map-Generator and the wiki
https://github.com/Azgaar/Fantasy-Map-Generator/wiki/Heightmap-template-editor).
- There is no "coast" primitive. Land and sea come from a heightmap built by a template of
  steps (Hill, Pit, Range, Trough, Strait, Mask, Add, Multiply, Smooth); the coast is wherever
  height crosses sea level (20 on a 0 to 100 scale).
- `Strait` is "a vertical or horizontal depression across the map": parameter `d` is only
  vertical or horizontal, `w` is width capped at a third of the map; the location is random
  between 30 % and 70 % of the map. The wiki says you cannot control its exact location.
- `Mask` lowers cells near the map edges with `distance = (1 - nx²)(1 - ny²)`, where `nx`, `ny`
  run from -1 to 1 across the map: a pure polynomial, no transcendental function.
- Positions of hills and pits are given as percentage ranges of the map width and height,
  for example `x:15-85 y:20-80`.

**Martin O'Leary's terrain generator** (the notes page https://mewo2.com/notes/terrain returned
404 when fetched on 2026-11-09; I read the code instead:
https://github.com/mewo2/terrain/blob/master/terrain.js).
- A coastal map is the sum of a `slope` (a plane: `x·dx + y·dy` for a random direction
  vector), an inverted `cone` (height proportional to distance from the centre) and 50 random
  Gaussian bumps; then smoothing, erosion, and a sea level chosen as a random quantile
  between 0.2 and 0.6 (`generateCoast`). So the coast's side comes from the slope's
  direction, and its shape from where the sea level cuts the summed surface.
- The drawn coastline is smoothed with a fixed kernel 0.25 / 0.5 / 0.25 on neighbouring path
  points (`relaxPath`): no transcendental function.

**Academic and general.** I did not find an academic paper on placing a coast from a bearing
and a type word within the time spent; the three generators above are what I can report.

### 1.2 Real harbour proportions

| Place | What the source gives | Source and its strength |
|---|---|---|
| Dublin Bay | About 10 km wide along its north–south base (Howth Head to Dalkey Point); 7 km "in length to its apex at the centre of the city" | Wikipedia, https://en.wikipedia.org/wiki/Dublin_Bay (no area given) |
| Bergen, Vågen | 1,200 m long, branching off Byfjorden; the town began on the east shore and spread south and west over centuries | Wikipedia, https://en.wikipedia.org/wiki/V%C3%A5gen,_Bergen ; Store norske leksikon agrees on about 1,200 m (search snippet, https://snl.no/Vågen). Width: not found in any source I read |
| Kinsale | Harbour is a "fjordlike valley estuary" of the Bandon, "virtually landlocked"; town 1½ miles (about 2.4 km) from the entrance (sailing guide); "about 3 miles from the sea" (1860 Encyclopaedia Britannica) | Search snippets of https://eoceanic.com/sailing/harbours/20/ and https://jimclifford.ca/early_encyclopedia_britannica/articles/1860/eb_8th_1860_009833.html ; the two distances disagree. Width: not found |
| Waterford | Estuary "nine Irish miles long and two broad" | 1911 Encyclopaedia Britannica, seen only as a search snippet (weak). An Irish mile is longer than a statute mile; my arithmetic with 1 Irish mile ≈ 2.05 km gives about 18 km by 4 km. Wikipedia https://en.wikipedia.org/wiki/Waterford_Harbour gives no dimensions |
| Wexford Harbour | "up to ten miles (16 km) wide at its widest point" in earlier times, mud flats on both sides | Wikipedia, https://en.wikipedia.org/wiki/Wexford_Harbour ; length and mouth width not given |

Town sizes for scale (search snippets, sources disagree): medieval walled Waterford about
14 hectares, the Viking enclosure about 6 hectares (Britannica:
https://www.britannica.com/place/Waterford-Ireland ; https://irishhistory.com/historic-town-waterford-city-co-waterford);
medieval Wexford about 18 to 25 hectares (https://wexfordhub.com/wexford-town-wall/ and the
Wexford County Council town walls conservation plan,
https://www.wexfordcoco.ie/sites/default/files/content/Planning/WexfordTownPlan09-14/TownWallsConservationPlan.pdf).

What this means for scale (my arithmetic): a 14 to 25 hectare town is roughly 375 to 500 m
across. Dublin Bay (10 km by 7 km) and Wexford Harbour (up to 16 km) are larger than the
largest site (5 km), so on the site only part of such a bay shows. Bergen's Vågen (1.2 km)
fits inside a 2 to 5 km site whole. I found no source giving a typical ratio of bay depth to
width; Dublin Bay is the only case with both numbers (7 / 10 = 0.7).

### 1.3 Shape constructions seen in the sources (no transcendental needed)

These are the constructions the sources use, applied to the four `shape` words. Which word
gets which construction is not settled by any source.
- Half-plane (O'Leary's slope plus sea level; Azgaar's edge mask): water on the side the
  bearing points to, behind a line perpendicular to the bearing.
- Depression across the map (Azgaar's Strait): a band of water with land on both sides.
- A bay or estuary has no primitive in any generator I read; in Azgaar and O'Leary it only
  emerges from summed shapes.

### Options, question 1

1. Coast as a half-plane perpendicular to `bearing_degrees`; `bay` subtracts a disc or
   ellipse from the land, `estuary` subtracts a funnel along the river, `strait` is a band
   with land beyond, `open` is the plain half-plane. Proportions are named, settable fields
   with documented defaults.
2. Coast as a height field: a plane tilted toward the bearing plus a shape-specific
   depression, cut at sea level (O'Leary's and Azgaar's approach); the coastline is the
   contour at sea level.
3. Proportions fixed by real examples in `data/research/` (Dublin 0.7 depth to width; Vågen
   1.2 km long), with Jeffrey choosing which example each `shape` follows.
4. Proportions relative to the site size (for example a fraction of the site's side), not
   to real examples.

## 2. Hill and hollow profiles

Notation: `d` distance from the centre, `r` = `radius_metres`, `h` = `height_metres`
(a hollow uses `depth_metres` and a minus sign), `u = d / r`.

| Profile | Formula for `u` in 0 to 1, zero outside | Function needed | Contours (equal height steps) |
|---|---|---|---|
| Cone | `h(1 - u)` | `sqrt` for `d` | Evenly spaced circles; a point at the top, a kink at the foot. Steepest slope `h/r` everywhere |
| Gaussian bell | `h·exp(-d²/(2σ²))` | `exp` | Rings sparse at the top, dense around `d = σ`, sparse again at the toe; never reaches 0, so it needs a cut-off |
| Cosine bump | `h·0.5(1 + cos(πu))` | `cos` and `sqrt` | Sparse at top and foot, densest at `u = 0.5`; reaches 0 smoothly at `r` |
| Smoothstep | `h(1 - (3u² - 2u³))` | `sqrt` for `u` | Almost the same picture as the cosine bump (my arithmetic: steepest slope `1.5 h/r` at `u = 0.5`, cosine `π/2 ≈ 1.57 h/r`) |
| Smootherstep | `h(1 - (6u⁵ - 15u⁴ + 10u³))` | `sqrt` for `u` | Flatter top and foot, steeper middle (first and second derivatives zero at both ends) |
| Squared falloff | `h(1 - u²)²` | none (uses `d²` directly) | Smooth at the foot; my arithmetic: steepest `≈ 1.54 h/r` at `u = 1/√3 ≈ 0.58` |

Sources:
- Smoothstep `3x² - 2x³` and Perlin's smootherstep `6x⁵ - 15x⁴ + 10x³`, slope zero at both
  edges: https://en.wikipedia.org/wiki/Smoothstep
- Inigo Quilez, "useful little functions" (mirror read:
  https://memalign.github.io/m/iq-ulf/functions.htm): the "cubic pulse" is
  `1 - x²(3 - 2x)` for `x = |x - c| / w` up to `w`, which is the smoothstep bump above; the
  exponential impulse needs `exp`.
- O'Leary's `mountains` uses a squared Gaussian, `exp(-d²/(2r²))²`; `cone` is
  `sqrt(x² + y²)·slope`; `peaky` takes `sqrt` of the normalised height
  (https://github.com/mewo2/terrain/blob/master/terrain.js).
- Azgaar's `addHill` is not a formula: it spreads from a start cell to neighbours, each new
  cell getting `parent^blobPower × random(0.9 to 1.1)`, with `blobPower` from 0.93 to 0.9973
  by cell count (source file above). It needs `pow` and randomness, and its shape depends on
  the grid.
- Sculpting tools offer the same family of falloffs: Unreal Engine lists Smooth, Linear,
  Sphere and Tip; Blender lists Smooth, Smoother, Sphere, Root, Sharp, Linear and others;
  a terrain-editing module (wolfhece) lists constant, linear, Gaussian, sigmoid and sphere
  (cosine). Read only as search snippets; the Unreal page returned 403 and the wolfhece page
  did not resolve. https://docs.unrealengine.com/4.26/BuildingWorlds/Landscape/Editing/Brushes ,
  https://docs.blender.org/manual/en/3.3/sculpt_paint/brush/falloff.html

On transcendentals and ADR-020: ADR-020 names std `sin`, `cos`, `tan`, `atan2`, `exp`, `ln`,
`powf` and `hypot` as not bit-identical across platforms and requires `libm` for
transcendentals. `sqrt` is not in that list; IEEE 754 requires square root to be correctly
rounded (my knowledge, not a source read today). Whether std `sqrt` is allowed or must be
`libm::sqrt` is a project rule question, not a research finding. Profiles in `d²` (the squared
falloff) need no square root at all.

### Options, question 2

1. Cone: simplest, contours evenly spaced, visible kink at summit and foot.
2. Smoothstep bump (polynomial in `u`, needs `sqrt` once per point).
3. Squared falloff `(1 - u²)²` (polynomial in `d²`, no `sqrt`, no transcendental).
4. Cosine bump through `libm::cos`.
5. Gaussian through `libm::exp`, with a cut-off radius as a named field.
6. A profile word per hill in the specification (a new enum), default chosen by Jeffrey.

## 3. What "at: north" means for a hill

What the sources do:
- Azgaar: positions are percentage ranges of the map, for example `x:15-85 y:20-80`; there are
  no compass words. `Strait` takes only "vertical" or "horizontal"
  (wiki and source file, section 1).
- O'Leary: hill positions are uniformly random over the whole extent; the slope direction is a
  random vector (terrain.js `mountains`, `randomVector`).
- MFCG: the community-reported `sea` parameter is an angle (fraction of a turn), not an edge
  or a distance (weak source, section 1).
- I found no generator that maps a compass word to a fixed fraction of the site radius.

What the project already has (read in this repository, not outside sources):
- The proposal (`documentation/proposals/specification-version-1.md` §2) says a hill's `at` "is
  a compass region of the site; `centre` is the site centre (the town core does not exist yet
  in S0)".
- ADR for `compass_region` (DECISIONS.md line 386) measures from the town core: eight 45°
  sectors and `centre` within `presets.centre_radius_m` (default 150). For S0 that core does
  not exist yet, so a hill needs its own rule for distance from the site centre.

### Options, question 3

1. The hill's centre sits on the sector's middle bearing at a fixed fraction of the site's
   half-side (a named, settable field, for example `hill_offset_fraction`), the same for every
   hill.
2. The hill's centre sits so that the hill touches the site edge (distance = half-side minus
   `radius_metres`), clamped to the centre when the hill is too large.
3. The hill's centre sits on the site edge itself; half the hill lies outside the site.
4. The word names a whole 45° sector; the seed picks a point inside it (hashed per-object
   stream, ADR on randomness), with a minimum and maximum distance as named fields.
5. Allow an optional `distance_metres` next to `at` in the specification; when missing, one
   of options 1 to 4 applies.

## 4. Drawing terrain on a hand-drawn overview map

### 4.1 What the sources say about each technique

- **Pictorial hills (side view, "molehills").** "The most ancient form of relief depiction in
  cartography"; rarely used today. https://en.wikipedia.org/wiki/Terrain_cartography .
  Braun and Hogenberg's town atlas (Civitates Orbis Terrarum, 1572 to 1617) shows relief
  "pictorially" in bird's-eye views; I found no source that describes the exact drawing
  convention (search snippets only, for example
  https://raremaps.com/gallery/detail/93724/hierosolyma-clarissima-totius-orientis-civitas-iudaee-metropolis).
- **Hachures.** Standardised by Johann Georg Lehmann in 1799. Strokes run down the steepest
  slope; steeper slopes get thicker, shorter strokes; flat ground and hilltops stay blank;
  still used on British Ordnance Survey maps for cuttings and embankments.
  https://en.wikipedia.org/wiki/Hachure_map . O'Leary draws short slope strokes per mesh
  point, skipping points below a random threshold and jittering positions
  (`visualizeSlopes`, uses `atan`, `exp` and random numbers;
  https://github.com/mewo2/terrain/blob/master/terrain.js). Watabou's Village Generator shows
  elevation "using hachures and shading"; cliffs are listed under plans
  (https://watabou.github.io/village.html).
- **Contour lines.** Oldest known isobath 1584 (Pieter Bruinsz, Spaarne); land contours by
  Vandelli 1746; Dupain-Triel's map of France 1791 at 20 m intervals with hachures; Haxo 1801
  at 1:500 for a fortress plan; general use in Europe by about 1843. "Close contours indicate a
  steep slope"; merged contours indicate a cliff. https://en.wikipedia.org/wiki/Contour_line
- **Form lines.** Lines that show the general shape of the ground "usually without indicating
  elevations" (Merriam-Webster, https://www.merriam-webster.com/dictionary/form%20line), used
  "when available information is insufficient" for contours (a relief-representation
  document on Scribd, seen only as a search snippet, weak:
  https://www.scribd.com/document/512788970/Representation-of-Relief-2).
- **Hillshading.** Simulated light, usually from the upper left; Raisz called the harsh
  version "plastic shading". **Hypsometric tints** (layer tints): colour bands between
  contours. **Tanaka contours** (1950): contours drawn lighter on lit slopes and darker on
  shaded ones, light from the north-west. All three:
  https://en.wikipedia.org/wiki/Terrain_cartography
- **Cliffs and shore symbols.** Ordnance Survey legends list cliff ("vertical face/cliff"),
  outcrop, scree, flat rock, "marsh or salting", sand, shingle and sand foreshore as separate
  symbols; I did not see the symbol graphics (search snippets of
  https://www.ordnancesurvey.co.uk/documents/25k-raster-legend.pdf and
  https://www.ordnancesurvey.co.uk/documents/50k-raster-legend.pdf). Teaching sheets on
  USGS-style symbols describe sand as brown dots and marsh as short blue horizontal lines
  (weak, secondary: https://www.compassdude.com/map-symbols.php).
- **Fantasy map practice.** Two coastline lines close together suggest a steep underwater
  drop (https://www.instructables.com/How-to-Draw-Fantasy-Maps); beach stipple is made by
  offsetting a dotted stroke inward from the shore
  (https://www.esri.com/arcgis-blog/products/analytics/analytics/fantasy-maps-pt-3-touchability);
  both read as search snippets. Watabou's MFCG does not model elevation; the author is quoted
  as saying elevation "rarely affects the geometry of a city"; devlog 0.6.0 adds an elevation
  option that only bends alleys by a "gradient field", strongest near the castle, coast and
  river banks
  (https://watabou.itch.io/medieval-fantasy-city-generator/devlog/67134/060-custom-colors-scale-bar-elevation).
  Mike Schley's city style (ProFantasy Symbol Set 5) adds water and hills before walls and
  houses; I found no description of how he draws hills or cliffs
  (https://www.profantasy.com/products/symbol-set-5-cities-of-schley). Dungeondraft: I found
  no source on its terrain conventions in the time spent.

### 4.2 Cost and determinism as SVG (my assessment, not a source)

| Technique | Input it needs | SVG output | Randomness needed | Transcendentals |
|---|---|---|---|---|
| Contour lines | Height grid, marching squares | One path per level | None | None (linear interpolation) |
| Colour bands | Same contours, closed into polygons | Filled paths | None | None |
| Form lines | Contours, drawn dashed or broken | Paths | Optional gaps | None |
| Hachures | Slope direction per sample point | Many short lines | Jitter for a hand-drawn look (hashed stream) | Direction from gradient components needs no angle; O'Leary's length rule uses `atan` and `exp` |
| Hillshading | Normal per cell, light vector | Raster image or many polygons | None | `sqrt` for normalising |
| Cliff symbol | The cliff line | Line plus short ticks on the low side | Optional jitter | None |
| Beach dots | Shore line, offset | Dots along an offset band | Dot positions (hashed stream) | None |
| Rock shore | Shore line | Small irregular polygons along the line | Shapes (hashed stream) | None |
| Salt marsh tufts | Area behind the shore | Small three-stroke tufts | Positions (hashed stream) | None |

Marching squares (https://en.wikipedia.org/wiki/Marching_squares): each 2×2 cell gets a 4-bit
index, a lookup table picks the crossed edges, crossings use linear interpolation; saddle cells
are ambiguous and the article resolves them with the cell's average value. Cells are
independent; contour lines on topographic maps are a named application.

### Options, question 4

1. Contours only (thin lines, every N metres as a named field), plus cliff ticks and shore
   symbols. Cheapest; reads as a modern map.
2. Contours plus colour bands (hypsometric tints).
3. Hachures only, as in 18th-century plans, with jitter from hashed streams. Most
   "hand-drawn"; most strokes.
4. Contours in pass 1, hachures as a later style; the renderer picks by a style setting.
5. Form lines (contours drawn broken and unlabelled) as the hand-drawn compromise.
For shore: A1 beach dots, rock polygons, marsh tufts as listed; A2 one line style per shore
word in pass 1, symbols later.

## 5. Height grid cell size and cost

Reference scales from real data:
- Ordnance Survey Terrain 5: regular 5 m post spacing; contours at 5 m intervals with index
  contours at 25 m (OS documentation, search snippet;
  https://docs.os.uk/os-downloads/products/land-and-terrain-portfolio/os-terrain-5/os-terrain-5-technical-specification).
  One reseller page says 2 m contours, which conflicts with the OS specification.
- Copernicus DEM GLO-30: 30 m global resolution, and it is a surface model that includes
  buildings and vegetation (https://dataspace.copernicus.eu/explore-data/data-collections/copernicus-contributing-missions/collections-description/COP-DEM ,
  search snippet).
- Azgaar maps blob power to cell counts from 1,000 to 100,000 (source file above); O'Leary's
  default is 16,384 mesh points (`npts`, terrain.js). Neither is in metres.

My arithmetic for a square grid (cells per side squared; 4 bytes a cell for `f32` or `i32`,
8 for `f64`):

| Site side | Cell 2 m | Cell 5 m | Cell 10 m | Cell 25 m |
|---|---|---|---|---|
| 500 m | 250² = 62,500 (0.25 MB) | 100² = 10,000 | 50² = 2,500 | 20² = 400 |
| 2 km | 1,000² = 1.0 M (4 MB) | 400² = 160,000 | 200² = 40,000 | 80² = 6,400 |
| 5 km | 2,500² = 6.25 M (25 MB) | 1,000² = 1.0 M (4 MB) | 500² = 250,000 (1 MB) | 200² = 40,000 |

Evaluating every feature at every cell costs cells × features; with about ten features at
1 M cells that is about 10 M profile evaluations, and marching squares is one pass per contour
level. A smallest feature should span several cells: a hill of `radius_metres` 100 at 25 m
cells covers only 8 cells across, so its contours will be visibly polygonal (my assessment).
A cliff is "a large height difference over a short distance" (ADR-056); on a grid the
shortest distance is one cell, so the cell size sets how sharp a cliff can be.

ADR-020 asks for quantised values. Heights stored as whole centimetres in `i32` keep the grid
exact; heights as `f64` need a quantisation step after each sum.

### Options, question 5

1. Fixed cell size in metres (for example 5 m) as a named field; a 5 km site then has 1 M cells.
2. Fixed cell count per side (for example 512); the cell size grows with the site (about 1 m at
   500 m, about 10 m at 5 km).
3. Cell size from the smallest feature (for example radius divided by a named number), with a
   minimum and maximum.
4. No grid in pass 1: contours of single features are drawn analytically (circles for hills,
   lines for the coast and slope); a grid arrives when features overlap or noise is added.
For storage: B1 heights as whole centimetres in `i32`; B2 `f64` quantised to 0.01 m after
each step (matching ADR-020's coordinate rule).

## What I did not find

- An academic method that places a coast from a bearing and a shape word.
- Widths of Bergen's Vågen and Kinsale Harbour; a modern measured length of Waterford Harbour.
- How Braun and Hogenberg, Mike Schley or Dungeondraft draw hills and cliffs in detail.
- The actual graphics of the Ordnance Survey shore and cliff symbols (only their names).
- O'Leary's notes page itself (404 on 2026-11-09); everything attributed to him comes from his
  code.
