# Several walkable grounds above one point: five representations compared

> Written 2026-11-09 by a research agent for issue #196. A claim here is binding only when a
> DECISIONS.md entry cites it (ADR-023: this report keeps its sources' uncertainty). The
> decision itself is an ADR in the milestone 2 design, made by Jeffrey. Sections marked
> "Author's analysis" are this report's own reasoning about the project, not a source's claim.

## Why this report exists

ADR-056 makes S0 (milestone 2) turn the named terrain features (`coast`, `cliffs`, `hills`,
`hollows`, `slope`, `plateau`) into one ground: one height field. REQUIREMENTS.md F2b reserves
"several walkable grounds above the same point (underground holds, tree cities, platforms)" for
a later version and says version 1 "must not make a second one impossible". Bearna (REQUIREMENTS.md
§8.3) has a cliff top, a cliff face, water level and an entrance "carved into the cliff face"
(the Gateway to Dis); ARCHITECTURE.md §9 puts "Bearna: cliffs and levels, cave gate" at
milestone 9. Foundry VTT is an output consumer (F16), with version 13 as the target until an ADR
changes it.

The project rules this report measures against:

- Determinism (ARCHITECTURE.md §1, §5; ADR-020): same specification and seed give byte-identical
  model JSON and SVG on the same machine and toolchain; topology decisions use `+ - * / sqrt floor`
  and integer-quantised comparisons; `libm` for transcendentals; values serialised rounded to the
  centimetre; collections in canonical order; stage outputs carry stable identifiers, never indices.
- Locality (§5): the unit is the grid cell; per-cell content derives from the cell and its
  one-ring; adding a landmark dirties its cell and ring only.

Scale used for every cost estimate below (from the task brief): a site of 500 m to 5 km across,
cells of about 1 m to 5 m. That gives these column counts (author's arithmetic):

| Site across | 5 m cells | 1 m cells |
|---|---|---|
| 500 m | 10,000 | 250,000 |
| 5 km | 1,000,000 | 25,000,000 |

For the volumetric representations the estimates assume a vertical range of 300 m (sea floor to
cliff top plus a hold below the town). That range is an assumption of this report, not a fact
about any calibration city.

## Sources read

| Short name | Source | What was read |
|---|---|---|
| Arches | Peytavie, Galin, Grosjean, Mérillou, "Arches: a Framework for Modeling Complex Terrains", Computer Graphics Forum 28(2), 2009, pages 457 to 467. Author PDF: https://perso.liris.cnrs.fr/egalin/Articles/2009-arches.pdf | Full text (sections 3, 7, Table 1) |
| Beneš 2001 | Beneš, Forsbach, "Layered Data Representation for Visual Simulation of Terrain Erosion", Spring Conference on Computer Graphics 2001. PDF: https://www.cs.purdue.edu/homes/bbenes/papers/Benes01SCCG.pdf | Full text (sections 2 and 4) |
| Review 2019 | Galin and others, "A Review of Digital Terrain Modeling", Computer Graphics Forum 38(2), 2019: https://diglib.eg.org/handle/10.1111/cgf13657 | Abstract only; the full text could not be downloaded, so nothing here cites its content |
| DF Z-level | Dwarf Fortress Wiki, "Z-level": https://dwarffortresswiki.org/index.php/Z-level | Full page (edited 4 August 2026) |
| DF Tile | Dwarf Fortress Wiki, "Tile": https://dwarffortresswiki.org/index.php/Tile | Full page |
| DF Ramp, Stairs | https://dwarffortresswiki.org/index.php/Ramp, https://dwarffortresswiki.org/index.php/Stairs | Opening sections |
| MC Chunk | Minecraft Wiki, "Chunk format": https://minecraft.wiki/w/Chunk_format | Section, palette and heightmap parts |
| MC Heightmap | Minecraft Wiki, "Heightmap": https://minecraft.wiki/w/Heightmap | Full page |
| MC Map | Minecraft Wiki, "Map item format": https://minecraft.wiki/w/Map_item_format | Colour and shading parts (first 100,000 characters) |
| Navigation mesh | Wikipedia, "Navigation mesh": https://en.wikipedia.org/wiki/Navigation_mesh | Full page |
| Recast | Recast and Detour README: https://github.com/recastnavigation/recastnavigation; `rcHeightfield`, `rcSpan`: https://recastnav.com/structrcHeightfield.html; `rcConfig`: https://recastnav.com/structrcConfig.html; `dtOffMeshConnection`: https://recastnav.com/structdtOffMeshConnection.html | Those pages |
| Foundry 12.320 | Foundry release notes 12.320 (26 April 2024): https://foundryvtt.com/releases/12.320 | Scene Regions part |
| Foundry 14.353 | Foundry release notes 14.353 (16 January 2026, prototype): https://foundryvtt.com/releases/14.353 | Scene Levels part |
| Foundry 14.359 | Foundry release notes 14.359 (1 April 2026, first stable version 14): https://foundryvtt.com/releases/14.359 | Scene Levels part |
| Foundry Regions | "Scene Regions" article (written for version 14.369): https://foundryvtt.com/article/scene-regions/ | Full article |
| Foundry v13 types | Version 13 documentation: RegionData https://foundryvtt.com/api/v13/interfaces/foundry.documents.types.RegionData.html, BaseRegion https://foundryvtt.com/api/v13/classes/foundry.documents.BaseRegion.html, WallData https://foundryvtt.com/api/v13/interfaces/foundry.documents.types.WallData.html, BaseScene https://foundryvtt.com/api/v13/classes/foundry.documents.BaseScene.html | Field lists |
| Foundry v14 LevelData | https://foundryvtt.com/api/v14/interfaces/foundry.documents.types.LevelData.html | Field list |
| Levels module | Package page https://foundryvtt.com/packages/levels; 2021 jam page https://theripper93.itch.io/levels | Package page and jam page; the module wiki (https://wiki.theripper93.com/levels) blocked automated reading and was not read |
| Universal VTT | Dungeondraft guide, "Universal VTT": https://dungeondraft-encyclopaedia.gitbook.io/guide/final-steps/exporting-your-map/universal-vtt | Search summary only; weak source, no official specification was found |

## 1. Height map (one height per point)

**What it is.** A grid that stores one height per column. Version 1 of S0 produces exactly this
(ARCHITECTURE.md §4). Minecraft keeps height maps next to its voxels: "Heightmaps are maps to store
the Y-level of the highest block at each horizontal coordinate" (MC Heightmap), with several kinds
(highest non-air block, highest motion-blocking block, ocean floor).

**Can represent.** Hills, hollows, slopes, plateaus, coast and shore, and a cliff as a large height
difference over a short distance (ADR-056). Beneš 2001 says height fields are cheap: "For a height
field of 1024x1024 elements we need n MB of data" (n bytes per element).

**Cannot represent.** Anything with two walkable heights above one point. Arches says height-field
methods "cannot represent overhangs, arches or even caves"; Beneš 2001 says height-field erosion
cannot dig "horizontal caves". So: no hold under the town, no tree-city platform over a street, no
cave behind Bearna's cliff face, no bridge deck with a street underneath.

**Top-down overview.** Directly drawable: hillshade, contours or hachures from the height
differences. Minecraft's map item shades every pixel by comparing its height with the block to its
north: lower is darkest, equal is middle, higher is brightest (MC Map).

**Determinism (author's analysis).** Good fit. Heights can be stored as whole centimetres (the
§5 serialisation unit), so comparisons are integer comparisons.

**Locality (author's analysis).** A column's height is a sum of feature contributions. A feature
with a bounded radius (hill, hollow) changes only columns inside its radius plus the one-ring. A
feature without a bound (`slope` with `toward_degrees`, `coast` with a bearing) changes the whole
site; that is true for every representation below, because it comes from the vocabulary, not the
storage.

**Foundry export.** One ground is one Foundry scene image. Version 13 has elevation on tokens and
regions (Foundry 12.320: "Elevation ranges are now set at the Scene Region level"), so a height map
can export as a single level, optionally with regions for height bands. Nothing more is needed.

**Cost.** 4 bytes per column (a 32-bit centimetre value): about 40 kilobytes (500 m, 5 m cells) to
about 100 megabytes (5 km, 1 m cells) (author's arithmetic). Generation is one pass over columns
times features.

## 2. Fixed z-levels (Dwarf Fortress)

**What it is.** The world is a stack of discrete horizontal layers of tiles. "Each layer of view
is a discrete z-level with a value relative to the bottom of the map space" (DF Z-level). A tile is
"approximately two meters long, two meters wide, and three meters high" (DF Tile, which also gives
a second, conflicting estimate of about 2.4 m by 2.4 m from movement speeds). A tile has a floor
section and a wall section (DF Tile).

**Scale in the source.** The largest embark is 16 by 16 local tiles of 48 by 48 tiles, "768×768
embark tiles (approx. 6144 feet or 1873 meters across)" (DF Tile). Default worlds give "around 50
z-levels of land ... with an additional 15 z-levels of empty sky", and the depth can range "to a
minimum of 6 and a maximum well in excess of 600" (DF Z-level). The same page says "reducing the
number of z-levels ... can reduce processor demand". So the largest Dwarf Fortress site is about the
middle of the CityMapMaker site range.

**Can represent.** Any number of grounds above one point, caves, tunnels, holds under a town,
bridges, towers. Moving between levels needs explicit connectors: stairs ("need to be connected to
other stairs of the appropriate type to function", DF Stairs) and ramps, where "a dwarf moving via
a ramp will move diagonally across Z-levels" (DF Ramp).

**Cannot represent.** Heights between levels. Every ground sits at a multiple of the level height
(about 3 m in Dwarf Fortress). A gentle slope becomes a staircase of ramps; a 1 m terrace or a 0.5 m
kerb is not representable. A city on rolling ground needs either small levels (many of them) or
loses its height detail.

**Top-down overview.** Dwarf Fortress shows one z-level at a time and the player scrolls between
them (DF Z-level); the game marks "trees or terrain on the level below" with a symbol (DF Ramp).
A single overview picture therefore has to be composed: for each column, draw the topmost
non-empty level, as the Minecraft map does for blocks (author's analysis).

**Determinism (author's analysis).** Excellent: everything is integers (column, row, level).

**Locality (author's analysis).** Excellent at the storage level: an edit touches the tiles it
covers. Connectors (stairs, ramps) are local too.

**Foundry export (author's analysis).** Each z-level could be one Foundry level, but a city with 50
to 100 z-levels would give 50 to 100 Foundry levels, most of them mostly empty. An exporter would
have to group levels into walkable grounds first, which is the problem representation 5 solves.

**Cost (author's arithmetic).** With 3 m levels over the assumed 300 m range: 100 levels. At 5 km
and 1 m cells that is 2.5 billion tiles; at 1 byte per tile, about 2.5 gigabytes dense. At 5 km and
5 m cells, about 100 megabytes. Sparse storage (only levels that are not all air or all rock) cuts
this a lot, as in Minecraft below.

## 3. Voxels (Minecraft)

**What it is.** A regular three-dimensional grid of cells, each holding a material. Minecraft
stores the world in chunks split into sections; each section is a "16×16×16-block area" with its
own palette; all entries in a section use the minimum bit length, "with a floor of 4 bits"; and when
"the palette holds one block state, the data array is not required, and that block fills the whole
section" (MC Chunk, as summarised by the fetch tool). The overworld chunk is 16×384×16 blocks
(MC Chunk). Beneš 2001 on dense voxels: for a 1024-cube array "we need n·1024³ i.e., n GB".

**Can represent.** Everything in the F2b list: caves, holds, overhangs, arches, platforms, bridges.
Arches says voxel representations can model overhangs but are "computationally demanding"
(section 2 of the paper).

**Cannot represent.** Shapes finer than one voxel; Minecraft's blocks are 1 m. Smooth slopes become
steps unless a surface is reconstructed from the voxels (Arches does that with an implicit surface).
Voxels say where material is, not where someone can walk: walkability has to be derived (Recast
does exactly that, see representation 5).

**Top-down overview.** The Minecraft map item draws, for each column, the first block from above
whose colour is not the transparent "NONE": "Blocks that use the NONE color are not drawn; they
instead draw the block underneath them" (MC Map; "first from above" is the fetch tool's inference,
not the page's wording). Water shows "the depth to the first non-water block beneath it". In the
Nether, which has a ceiling, "maps render as a kind of static" (MC Map): a top-down view of a
roofed world shows the roof, and the source gives no cave rendering.

**Determinism (author's analysis).** Excellent: integer coordinates, integer materials. Generation
by noise must use the stateless `hash(seed, cell_x, cell_y)` of §5, extended with a level.

**Locality (author's analysis).** Excellent: a voxel edit touches its section.

**Foundry export (author's analysis).** Same as z-levels: voxels must first be grouped into walkable
grounds and then exported per ground. Walls (Foundry wall segments) have to be traced from voxel
boundaries.

**Cost (author's arithmetic).** At 1 m voxels over 300 m: 500 m site, 75 megabytes dense; 5 km site,
7.5 gigabytes dense at 1 byte per voxel. Minecraft-style sparse sections (single-material sections
store no array) bring the real cost close to "number of sections that cross a surface", but the
code to manage that is the largest of the five options.

## 4. Columns of solid spans (layered terrain, Arches and Beneš)

**What it is.** A two-dimensional grid where each column holds an ordered list of vertical
intervals. Beneš 2001: "We save the entire landscape as a two dimensional array. Every element of
the array is consisting of one-dimensional array consisting of elements containing information
about all underlying layers", described as "some kind of interval data structure that can be
thought as RLE in voxels" (run-length encoding: one entry per run of equal cells). Arches: "A
terrain is defined as a two dimensional grid of material stacks. A material stack is defined as a
stack of material layers that are characterized by their thickness and the corresponding material
type [Beneš 2001]."

Recast (representation 5's build tool) uses the same shape internally: `rcHeightfield` keeps, per
column of the ground grid, a linked list of `rcSpan` entries with a lower limit `smin` (inclusive)
and an upper limit `smax` (exclusive), in whole multiples of the cell height `ch` (Recast).

**Can represent.** Caves, overhangs, arches, holds: "overhangs, arches and caves can be easily
created by inserting an air layer between two bedrock layers" (Arches). Beneš 2001: layers with
density zero "can represent caves and holes. This is an advantage of this representation inherited
from the voxels that cannot be captured by height fields." Heights are continuous (a thickness
per layer), not stepped as with z-levels.

**Cannot represent.** Vertical walls finer than one column, and anything that is not vertical in
structure (a tunnel that runs at a slant is a staircase of columns). A span says where material
is, not where a ground is: "the top of span 2 in this column" and "the top of span 2 in the next
column" are not the same ground unless something groups them. The sources describe erosion and
sculpting, not grounds, walkability or towns.

**Top-down overview (author's analysis).** The top of the topmost span is a height map, so the
surface ground draws exactly as representation 1. Lower grounds are hidden; they need a cut-away,
a dashed outline, or a separate sheet. Arches itself renders in three dimensions by polygonising
an implicit surface (section 6 of the paper), which is not useful for a top-down handout.

**Determinism (author's analysis).** Good if span limits are whole centimetres, spans in a column
are kept sorted and merged (Arches merges and sorts layers, section 3.3), and no floating sums
decide topology. The Arches surface reconstruction (convolution, root finding along columns) is
float-heavy and would need the §5 float rules; the project would not need it for a top-down map.

**Locality (author's analysis).** Storage is per column, so an edit is local. Grouping spans into
named grounds is not automatically local: a flood fill that joins touching spans can merge two
grounds when one new span bridges them, and a ground numbered by discovery order would change
identity far away. Ground identifiers must therefore come from the specification (§5 already does
this for landmarks), not from the grouping.

**Foundry export (author's analysis).** Needs the same grouping step as voxels: spans to grounds,
grounds to Foundry levels.

**Cost.** Arches Table 1 reports a cave scene with "effective voxel resolution" 1,000 × 750 × 8,000
in 21 megabytes, a canyon of 1,000 × 500 × 5,000 in 14 megabytes and a cliff of 2,000 × 500 × 4,000
in 26 megabytes, and notes the cliff "requires more memory because its overall geometry is more
complex with many different material layers". Beneš 2001 says processing visits "k·n²" elements
instead of "n³", "where k is the number of the layers – this is usually much smaller". Author's
arithmetic for a city: most columns hold one span (the ground) and a few hundred thousand hold two
or three (holds, cave). At 8 bytes per span (two 32-bit centimetre limits) plus a 4-byte offset per
column, a 5 km site at 1 m costs about 300 megabytes; at 5 m about 12 megabytes; a 500 m site at
1 m about 3 megabytes.

## 5. Walkable surfaces joined by links (navigation meshes; Foundry levels)

**What it is.** A set of surfaces, each a footprint polygon with its own height (or its own small
height field), plus links between surfaces. Wikipedia describes a navigation mesh as "a collection
of two-dimensional convex polygons" where "adjacent polygons are connected to each other in a
graph", and says it "allows traversable areas that overlap above and below at different heights",
unlike a two-dimensional grid. Recast builds such a mesh in four steps: it "rasterizes the input
triangle meshes into voxels", removes voxels "in areas where agents would not be able to move",
divides the walkable areas "into sets of polygonal regions", then re-triangulates them "into a
navmesh" (Recast README, Zlib licence). Walkability uses `walkableHeight` ("minimum floor to
'ceiling' height"), `walkableClimb` (flowing "over low lying obstructions such as curbs and
up/down stairways") and `walkableSlopeAngle` (Recast `rcConfig`). Links that are not adjacency,
such as a ladder or a jump, are off-mesh connections: "a user defined traversable connection made
up to two vertices", with a user-assigned `userId` (Recast `dtOffMeshConnection`; the page does
not say whether they are two-way).

Foundry's own model is of this kind. Version 14 "Scene Levels" let a scene "stack multiple images
at defined elevations", with "walls, light sources, and other placeable objects ... specific to
only specific levels" or "shared across multiple (or all) levels" (Foundry 14.359). A level stores
`elevation: {bottom, top}`, `background`, `foreground`, `sort` and `visibility: {levels}` (Foundry
v14 LevelData; the page lists types only, no descriptions). Tokens move between levels with "the new
Change Level behavior" of a region, and a "Define Surface" behavior controls light, movement, sight
and sound through a region's bottom or top (Foundry 14.353, prototype notes). A "Teleport Token"
behavior "allows connecting disparate Regions, which may not even be on the same Scene" (Foundry
Regions article).

**Can represent.** Every F2b case as named objects: the town ground, a hold under it, a tree-city
platform, Bearna's cliff top, water level and cave interior, each with a footprint and a vertical
position, plus links (stairs, ramps, the cave mouth, a lift). Each surface carries its own heights,
so a surface can be rolling ground (its own height map) or a flat deck.

**Cannot represent.** Solid material: there is no rock, only floors. Questions such as "how thick
is the rock between the hold and the street" or "is this cliff overhanging" need either an explicit
field on the surface or a volumetric model underneath. Two surfaces can overlap in plan, so nothing
in the representation itself stops an impossible pair (a hold whose ceiling is above the street);
validation has to check it (author's analysis).

**Top-down overview (author's analysis).** Draw surfaces from lowest to highest; an upper surface
covers a lower one. A hidden surface needs a chosen convention: dashed outline, a separate inset,
or a separate sheet per surface. Links are drawable as symbols (stair, cave mouth) at their
positions; Dwarf Fortress's down-ramp symbol "▼" is "a display nicety, not a type of terrain" (DF
Ramp), the same idea.

**Determinism (author's analysis).** Good if footprints go through the `geometry` facade with the
ADR-020 rules (coordinates quantised to 0.01 m after every overlay, zero-area polygons rejected),
surfaces are kept in identifier order, and link identifiers are hashed from the sorted pair of
surface identifiers plus a stable key, as street identifiers are hashed from their endpoints (§5).

**Locality (author's analysis).** A surface is one object with an extent, not a cell. Adding a hold
changes the hold, its links, and the surface above it only inside the hold's footprint and ring (for
example, a cave mouth). It does not change surfaces elsewhere as long as surface identifiers come
from the specification. The S0 stream key `("site", cell)` would need the surface in it, for example
`("site", surface, cell)`.

**Foundry export.** Closest fit of the five for version 14: one surface to one Foundry level with
`elevation {bottom, top}`, one link to one region with Change Level or Teleport Token. For version
13 (the current target) the situation is different:

- The version 13 Scene schema has no levels field (Foundry v13 types, BaseScene).
- Version 13 regions have an elevation field. Foundry 12.320 says "the bottom/top elevation was
  removed from BaseShapeData and added to the RegionDocument"; the version 13 BaseRegion schema
  lists `elevation` as a schema field (an object of sub-fields); the version 13 RegionData page
  lists `elevation?: number`. These two version 13 pages disagree, and this report could not
  resolve it.
- Version 13 `WallData` has no elevation or height field (`c`, `dir`, `door`, `move`, `sight`,
  `sound`, `light`, `threshold`, `flags`) (Foundry v13 types).
- Multi-level scenes in version 13 rely on the Levels module: its package page lists release 6.1.0
  for Foundry 13 and 7.0.x for Foundry 14 (Levels module). The 2021 jam page says a range such as
  "3,12" gives "the floor heigh and ceiling height of that section", tiles get an elevation range,
  and "using the module wall height in conjunction with levels is highly suggested"; that page is
  about five years old, and the module wiki was not read.
- Universal VTT (F16, milestone 10) stores an image, walls, portals and lights (Dungeondraft guide,
  weak source); no elevation field appeared in what was read, which suggests one file per ground.

**Cost (author's analysis).** Proportional to the number of surfaces and links, not to the site
area: tens to hundreds of surfaces and links, kilobytes. Each surface still needs its own heights;
the surface ground of a city is a full height map (representation 1), so the total is "one height
map plus small ones".

## Combinations seen in the sources

- Minecraft keeps voxels and height maps together; the height map is derived from the voxels (MC
  Heightmap, MC Chunk).
- Recast keeps spans (representation 4) as an intermediate and outputs walkable polygons with links
  (representation 5) (Recast).
- Arches keeps material stacks (representation 4) and reconstructs a surface from them (Arches).
- Foundry version 14 stores per level an image and an elevation range, and lets placeables belong to
  one or several levels (Foundry 14.359): representation 5 at the output end.

Author's analysis: a combination that the sources suggest for this project is "a height map per
surface plus links" (1 + 5), with spans (4) as an optional internal step if a later stage needs
rock thickness or overhangs. This is an observation, not a recommendation; the choice is Jeffrey's.

## What version 1 must avoid now (author's analysis)

Each item names the trap, then what avoids it. None of them adds a second ground in version 1.

1. **Indexing terrain by (x, y) alone.** A stage that reads `height[x][y]` assumes one ground.
   Avoid: stages read terrain through a query interface on the S0 output, for example
   `height_at(ground, point)` and `grounds_at(point)` (a list, with one element in version 1).
2. **Exposing the height field's storage.** If the model JSON (F15) or another stage reads the raw
   grid, changing the representation later breaks every reader. Avoid: keep the grid private to the
   site module, behind the query interface (the same pattern as the `geometry` facade, ADR-020).
3. **Grounds without identity.** Avoid: give the one version 1 ground a stable identifier now
   (kind `ground`, from a fixed key such as the word `surface`), and let every position that needs a
   height carry it, or default to it in one place.
4. **Ground identifiers from order.** "Ground 0 is the surface, ground 1 is below" breaks when a
   ground is added between them, and breaks §5 ("identifiers, never indices"). Avoid: identifiers
   from the specification, never from sorting or flood-fill order.
5. **Random stream keys without room for a ground.** If milestone 2 keys S0 noise by
   `("site", cell)` and a later version re-keys it by `("site", ground, cell)`, every version 1 map
   changes. Avoid: decide now whether the default ground's key hashes to the same value as today's
   key, so adding grounds leaves the surface ground byte-identical.
6. **Water, shoreline and cliff lines as site-wide singletons.** Bearna has water at the foot of the
   cliff and the underground sea of Dis. Avoid: S0 outputs that are "the water mask" become "the
   water of a ground" in the type, with one ground in version 1.
7. **Constraint evaluators that read heights directly.** `on_high_ground` and `on_low_ground` (F3)
   compare heights. Avoid: they go through the query interface, so that "high ground" later means
   "high on this ground".
8. **Renderer assumptions.** ARCHITECTURE.md §7 has one `<g>` per layer. Avoid: name the layer
   group after the ground, so a second ground is a second group, not a new renderer.
9. **Foundry export that assumes one level.** Milestone 7b emits one PNG and scene JSON. Avoid:
   name the PNG and the scene elevation after the ground identifier, so a second ground is a second
   image (version 14 level, or version 13 Levels tile).
10. **Cliff semantics that cannot grow.** ADR-056 makes a cliff a large height difference on one
    ground. Under representation 5 a cliff with a cave behind it may become a boundary between two
    surfaces. Avoid: keep `cliffs` as a named feature in the output (S0 already outputs "cliff
    lines"), not only as baked heights, so a later version can attach a cave mouth to a cliff line.

## Neutral comparison

| Property | 1 Height map | 2 Z-levels | 3 Voxels | 4 Spans | 5 Surfaces and links |
|---|---|---|---|---|---|
| Several grounds above one point | No | Yes | Yes | Yes | Yes |
| Caves, overhangs | No | Yes, stepped | Yes, stepped | Yes | Only as surfaces, no rock |
| Smooth height on one ground | Yes | No, stepped by level height | No, stepped by voxel | Yes | Yes, per surface |
| Knows where people can walk | Implicitly (whole site) | Floors and connectors | Must be derived | Must be derived | Yes, by construction |
| Named grounds | One | Must be grouped | Must be grouped | Must be grouped | Yes, by construction |
| Links between grounds | Not needed | Stairs, ramps | Must be derived | Must be derived | Explicit objects |
| Top-down overview | Direct | Topmost level per column | Topmost voxel per column | Topmost span = height map | Lowest to highest; hidden ones need a convention |
| Determinism fit | Good, integer centimetres | Excellent, integers | Excellent, integers | Good, sorted integer spans | Good, through the geometry facade |
| Locality of edits | Per feature radius | Per tile | Per section | Per column; grouping can be non-local | Per surface footprint |
| Foundry version 13 export | One scene | Group first, then Levels module | Group first, then Levels module | Group first, then Levels module | Levels module; region elevation |
| Foundry version 14 export | One level | Group first | Group first | Group first | One surface to one level, links to regions |
| Memory, 5 km at 1 m (author's estimate) | About 100 megabytes | About 2.5 gigabytes dense | About 7.5 gigabytes dense | About 300 megabytes | Kilobytes plus one height map per surface |
| Memory, 500 m at 5 m (author's estimate) | About 40 kilobytes | About 1 megabyte dense | About 3 megabytes dense (1 m vertical) | About 120 kilobytes | Kilobytes plus one height map per surface |
| Implementation effort (author's judgement) | Smallest | Medium | Largest | Medium | Medium; overlap validation needed |

## Questions for Jeffrey in the milestone 2 design

1. Which representation, or which combination, does the project use for several grounds later?
   Options from this report: 1 alone (no second ground, F2b dropped); 2; 3; 4; 5; 1 + 5 (a height
   map per surface, plus links); 4 inside S0 with 5 as the output.
2. Does version 1 already carry a ground identifier in the S0 output and the model JSON (one
   ground, item 3 above), or does it only hide the height field behind a query interface (items 1
   and 2) and add the identifier later?
3. Is a later ground a full-site layer or a footprint polygon (a hold under one district, a
   platform over one street)?
4. Is a ground's vertical position written in the specification (for example a hold's
   `depth_metres` below the surface) or generated?
5. Are links (stairs, ramps, lifts, the Gateway to Dis) specification items with identifiers, or
   generated by a stage?
6. How does the overview map show a ground that is hidden under another: dashed outline, inset,
   one sheet per ground, or not at all on the handout?
7. Does the Foundry export of several grounds target version 13 with the Levels module, or version
   14 native Scene Levels (version 14 has been stable since 1 April 2026, release 14.359)? Changing
   the version 13 target needs an ADR (REQUIREMENTS.md F16).
8. Does the S0 random stream key change to include the ground, and must the surface ground stay
   byte-identical when a second ground is added (item 5 above)?
9. Bearna is milestone 9 in ARCHITECTURE.md §9 ("cliffs and levels, cave gate"), while F2b reserves
   several grounds for a later version. Does milestone 9 draw the cave gate as a symbol on one
   ground, or does it need the second ground?
10. In version 1, is a cliff only baked heights on one ground (ADR-056), or does S0 also keep the
    cliff as a named line that a later cave mouth can attach to (item 10 above)?
11. Do tree cities and built platforms count as grounds in this model, or as buildings with floors
    (a later building stage)?
12. What vertical quantisation does a ground use: whole centimetres as in the §5 serialisation
    rule, or a coarser step?
