> Written 2026-10-19 for a TypeScript target, before ADR-003 chose Rust. The findings
> stand; the library tables and code snippets do not apply.

# Rendering & annotation research: hand-drawn fantasy city overview maps

Date: 2026-10-19. Target: TypeScript, SVG-first, Node (headless batch) + browser, later Foundry VTT export.
Legend: **[V]** verified against a primary source (URL given), **[P]** partially verified / inferred from a secondary source, **[U]** unverified (own knowledge or inference; test before relying on it).

---

## 1. Recommended rendering stack (one page)

Principle: generate one **plain, static SVG document string** (no DOM dependency) from the city model, then rasterise that same string with resvg in Node and show it directly in the browser. Keep the "sketchy" geometry *in the SVG* (paths), not in filters, so output is deterministic, portable, and cheap to rasterise. Use SVG filters only for an optional paper/ink layer that can be switched off for VTT export.

| Layer | Library | Version (npm, 2026-10) | License | Role | Notes |
|---|---|---|---|---|---|
| Sketchy geometry | `roughjs` | 4.6.6 [V] | MIT [V] | `rough.generator()` turns rects/polygons/paths into wobbly multi-stroke outlines + hachure/cross-hatch/dots fills; `generator.toPaths(drawable)` returns `{d, stroke, strokeWidth, fill}` so you can emit `<path>` yourself. Documented as usable "on a server or in a web worker" without a canvas [V]. | Use `seed` (1..2^31) for reproducible output [V]. Deps: `hachure-fill` 0.5.2 (MIT), `points-on-curve`, `points-on-path`, `path-data-parser` [V]. DOM-free import in Node: **[U]** believed true for the ESM bundle; add a smoke test in CI. |
| Standalone hatching | `hachure-fill` | 0.5.2 [V] | MIT [V] | Fill any polygon with parallel lines at a gap/angle (used internally by rough.js); use it for cliffs, water hachures, shadow sides of buildings. | README not retrieved; API is `hachureLines(polygons, gap, angle)` **[U]**. |
| Freehand stroke outlines | `perfect-freehand` | 1.2.3 [V] | MIT [V] | Converts a polyline + pressure into a variable-width ink outline polygon (pen-like tapering). Good for coastlines, rivers, main roads. | Pure math, no DOM. |
| Spatial index for collisions | `rbush` | 4.0.1 [V] | MIT [V] | R-tree for label/icon bounding-box collision tests. | Also used by labelgun (rbush ^2) [V]. |
| Label placement | own code (greedy 8-position + optional SA) using `rbush`; optionally `@d3fc/d3fc-label-layout` 7.0.4 (MIT) [V] for greedy/annealing strategies, or `label-locator` 0.0.4 (MIT, TS port of d3-labeler, "not bound to any rendering library") [V] | | | d3fc-label-layout depends on d3fc-rebind/data-join (D3 selection model) [V] – less convenient headless. labelgun 6.1.0 (MIT) only decides show/hide by weight [V]. Recommendation: write the placer (≈300 lines) and use rbush. |
| Text measuring in Node | `opentype.js` (**[U]** version; MIT) or font metrics from resvg bbox | | | Needed to compute label widths headlessly; resvg-js can also return an SVG bbox (v2 feature) [V]. |
| Rasteriser (Node) | `@resvg/resvg-js` | 2.6.2 [V] | MPL-2.0 [V] | SVG string → PNG. `fontFiles`/`fontBuffers`, `loadSystemFonts:false` for speed, `fitTo:{mode:'width', value}` for 300 DPI scaling, `background` option [V]. Prebuilt binaries for Win/macOS/Linux, Node ≥10 [V]. Own benchmark: ~2–4× faster than sharp for SVG rendering [P, self-reported]. | No native text rendering, i.e. fonts must be supplied; SVG `<font>` elements unsupported but CSS `@font-face`/supplied TTF work [V/P]. |
| Rasteriser (browser) | `@resvg/resvg-wasm` | 2.6.2 [V] | MPL-2.0 | Same engine via WASM for client-side PNG export; or simply draw the SVG into `<canvas>` via `Image` and `toBlob()`. | WASM slower than native [V]. |
| Fallback rasteriser | `sharp` | 0.35.5 [V] | Apache-2.0 [V] | librsvg-based; `density` (default 72 DPI, 1–100000) scales SVG input [V]; `limitInputPixels` default 0x3FFF² [V]. | Depends on fontconfig for fonts (needs system font install) **[P]**. Use only if you already use sharp for post-processing (WebP, tiling). |
| Browser-engine fallback | `puppeteer` | 25.14.0 [V] | Apache-2.0 | Only if you need exact Chrome filter rendering (feTurbulence/feDisplacementMap) for hero images. Heavy. | Node ≥22.12 [V]. |
| Canvas option (if ever needed) | `@napi-rs/canvas` 1.0.11 (MIT) or `skia-canvas` 4.0.0 (MIT) [V] | | | Skia-backed Node canvas; not needed for the SVG-first pipeline. | |
| Fonts (bundled, OFL) | IM Fell English, Cinzel, Almendra, MedievalSharp, Pirata One, Uncial Antiqua, Grenze [V all OFL via google/fonts METADATA] | | SIL OFL 1.1 | Embed as base64 `@font-face` in exported SVG; pass TTF files to resvg. | OFL permits bundling and distribution; keep OFL.txt with the fonts. |
| Foundry export | own code | | | Emit Foundry Scene JSON (v13 `SceneData`: width, height, padding, background, grid{type,size,distance,units}, walls[], lights[], notes[]) and/or a Universal VTT `.dd2vtt` JSON (base64 PNG + grid + LOS + lights) [V]. | No existing Watabou-city→Foundry module found [V]. |

Pipeline (headless): model → `renderSvg(model, style, seed)` (pure TS, emits string) → `labels.ts` (measure + place, rbush) → `legend.ts` (numbered legend + leader lines) → `@resvg/resvg-js` (PNG at 300 DPI) → optional `sharp` (WebP/tiles) → optional `foundry.ts` (Scene JSON / dd2vtt).
Pipeline (browser): same SVG string injected into DOM; PNG via canvas `drawImage` or resvg-wasm.

Print maths (A3 at 300 DPI): 297 × 420 mm = 11.69 × 16.54 in → **3508 × 4961 px** (landscape 4961 × 3508). A4 at 300 DPI = 2480 × 3508 px.

---

## 2. Hand-drawn / sketchy rendering techniques

### 2.1 rough.js (recommended core)
- API: `rough.generator(config, size)` returns a generator whose methods mirror RoughCanvas but return serialisable *drawables*; `generator.toPaths(drawable)` returns `PathInfo[]` (`d`, `stroke`, `strokeWidth`, `fill`, or `pattern`) to be rendered in order. Documented as useful "where no drawing context exists, such as on a server or in a web worker" [V]. https://github.com/rough-stuff/rough/wiki/RoughGenerator
- Options & defaults [V]: `roughness` 1, `bowing` 1, `seed` (unset/0 = random; 1..2^31), `stroke` #000, `strokeWidth` 1, `fillStyle` hachure (also solid, zigzag, cross-hatch, dots, sunburst, dashed, zigzag-line), `fillWeight` = strokeWidth/2, `hachureAngle` −41°, `hachureGap` = 4×strokeWidth, `curveStepCount` 9, `curveFitting` 0.95, `simplification` 0, `disableMultiStroke`/`disableMultiStrokeFill`, `preserveVertices` false. https://github.com/rough-stuff/rough/wiki
- npm: `roughjs` 4.6.6, MIT, deps hachure-fill ^0.5.2, points-on-path, points-on-curve, path-data-parser [V]. https://registry.npmjs.org/roughjs/latest
- Tips (own experience, [U]): roughness 0.6–1.0 for buildings at city scale, 1.5+ looks messy; set `disableMultiStroke: true` on tiny buildings (<6 px) to avoid blobbing; `preserveVertices: true` for walls so towers stay attached; use one `seed` per map and derive per-feature seeds (hash of id) so re-renders are stable (same idea as tldraw's shape-ID-seeded PRNG [V] https://tldraw.dev/blog/engineering-imperfection-with-draw-shapes).
- `svg2roughjs` (MIT) converts an existing SVG DOM into rough style; browser-DOM oriented, includes a `nodejs-cli` folder (untested) [V/P]. https://github.com/fskpf/svg2roughjs

### 2.2 Wobbly lines without rough.js
- Perturb polyline vertices with low-frequency noise and draw 2 passes with different seeds; tldraw rounds corners with quadratic Béziers depending on angle and caps rounding at ¼ of adjacent segment length [V]. https://tldraw.dev/blog/engineering-imperfection-with-draw-shapes
- `perfect-freehand` 1.2.3 (MIT) produces a tapered ink outline polygon from points + pressure [V]. https://registry.npmjs.org/perfect-freehand/latest
- Blender Freestyle's Perlin-noise line modifier (amplitude, frequency, octaves, seed) is a good parameter model; a different seed per pass avoids identical passes overlapping [V]. https://blenderartists.org/t/animatable-sketchy-look-with-freestyle/627845

### 2.3 Hatching, hachures, stipple
- `hachure-fill` 0.5.2 (MIT) fills polygons with parallel lines [V]. https://registry.npmjs.org/hachure-fill/latest
- Classic hachure rules (Lawrence 1979 via Wikipedia): strokes along steepest gradient, rows perpendicular, length/thickness ∝ slope, thinner/sparser on the lit side, flat tops left blank; fine parallel lines also used along coastlines/lakes [V]. https://en.wikipedia.org/wiki/Hachure_map
- Rock/cliff hachure automation: stroke width derived from shaded-relief grey values (ETH research) [P]. https://research-collection.ethz.ch/handle/20.500.11850/201368
- Stipple: emit `<circle r=0.3>` with Poisson-disk/blue-noise spacing; density from a scalar field (own approach, [U]). Keep stipple counts bounded (<50k circles) for resvg speed [U].

### 2.4 SVG filters: ink bleed and paper
- Wobble: `feTurbulence` (low baseFrequency) + `feDisplacementMap` (small `scale`) on `SourceGraphic` [V]. https://css-tricks.com/?p=311126 and https://www.smashingmagazine.com/2021/09/deep-dive-wonderful-world-svg-displacement-filtering/
- Paper grain: `feTurbulence type="fractalNoise"` (+`stitchTiles="stitch"`), optionally `feDiffuseLighting` for relief, `feColorMatrix` for warm tint/alpha, composite over art [V for primitives; recipe is synthesis]. https://www.creativebloq.com/netmag/how-go-beyond-basics-svg-filters-71412280 , https://css-tricks.com/?p=335760
- Ink bleed: `feMorphology dilate` + `feGaussianBlur` + `feComposite`/threshold via `feColorMatrix` **[U]** (standard recipe, not sourced here).
- Headless support: resvg's unsupported-list covers SVG font elements, altGlyph, kerning etc. and does **not** list feTurbulence/feDisplacementMap/feMorphology as unsupported [V] https://github.com/linebender/resvg/blob/main/docs/unsupported.md ; resvg claims near-complete static SVG 1.1 (filters included) **[P]**. Filters are rasterised per filter region at output resolution, so a full-page turbulence filter at 4961×3508 is the single most expensive step **[U]**; render once, measure, and offer a `--no-paper` flag. librsvg (sharp) lists feTurbulence/feDisplacementMap as supported elements [V] https://librsvg-b807a6.pages.gitlab.gnome.org/devel-docs/features.html . Browser rendering of these filters is inconsistent across engines (historic bugs) [V] https://lists.w3.org/Archives/Public/www-svg/2010Aug/0038.html
- Recommendation: geometry-level sketchiness (rough.js) everywhere; filters only for a paper background rect and an optional ink-bleed on the line layer; verify pixel parity between resvg and Chrome in a snapshot test.

### 2.5 Headless renderers compared
| | resvg-js | sharp (librsvg) | @napi-rs/canvas / skia-canvas | puppeteer |
|---|---|---|---|---|
| Install | prebuilt .node, zero deps [V] | prebuilt libvips [V] | prebuilt Skia [V] | downloads Chromium |
| Fonts | explicit TTF files/buffers; disable system fonts for speed [V] | fontconfig/system [P] | register fonts via API [P] | system/CSS |
| SVG filters | yes [P] | yes [V] | n/a (draw yourself) | full Chrome |
| Speed (SVG→PNG) | fastest in own benchmark (39.6 vs sharp 10.9 ops/s, 2023) [P] | slower | n/a | slowest |
| Text on path | supported (resvg supports textPath) **[P]** | yes | manual | yes |

---

## 3. How the reference generators render

### 3.1 Watabou – Medieval Fantasy City Generator (open-source 2017 snapshot)
- Repo `watabou/TownGeneratorOS`, Haxe + OpenFL, **GPL-3.0** (so copy ideas, not code) [V]; lacks later features (water bodies, options UI) [V]. https://github.com/watabou/TownGeneratorOS
- Files: `mapping/CityMap.hx` (draw order), `mapping/Brush.hx` (stroke constants), `mapping/Palette.hx` (palettes), `building/CurtainWall.hx`, `wards/*.hx` [V].
- Draw order (bottom→top): roads → patches/buildings per ward → hit areas → walls+towers (main wall, then citadel) [V].
- Roads: two-pass casing: wide stroke `MAIN_STREET + NORMAL_STROKE` in `palette.medium`, then narrower `MAIN_STREET − NORMAL_STROKE` in `palette.paper` on top [V].
- Walls: polygon outline `THICK_STROKE` (1.8 units) in `palette.dark`; gates as short perpendicular lines at `THICK_STROKE×2` width, length `THICK_STROKE×1.5`; towers = filled dark circles r = `THICK_STROKE` (1.5× for citadel) [V].
- Buildings: outline pass (stroke `thickness×2`) then fill pass without stroke, leaving a half-width border; castle outline `NORMAL_STROKE×4`, cathedral `NORMAL_STROKE×2`; parks = grove polygons filled `palette.medium` [V]. Constants: `NORMAL_STROKE 0.3`, `THICK_STROKE 1.8`, `THIN_STROKE 0.15` (model units) [V].
- Palettes (paper/light/medium/dark) [V]: DEFAULT ccc5b8/99948a/67635c/1a1917; INK cccac2/9a979b/6c6974/130f26; ANCIENT ccc5a3/a69974/806f4d/342414; COLOUR fff2c8/d6a36e/869a81/4c5950; BW, BLUEPRINT, NIGHT, SIMPLE.
- The modern (closed) version adds SVG/PNG/JSON export, custom palettes for roads/fields/greens/water/roofs, "B&W"/"Ink" presets [P]; its hand-drawn styles are not in the OS repo [V]. Perilous Shores (closed) draws forests as "cloud" blobs with tree sprites along the bottom edge and optional shadows [V] https://watabou.itch.io/perilous-shores/devlog/678429/163-improved-forests
- Lesson: the Watabou look is mostly *casing + two-tone fills + heavy wall line*; sketchiness is a thin layer on top.

### 3.2 Azgaar – Fantasy Map Generator (MIT)
- Repo has moved to a build step with TypeScript (≥1.110) [V]; style presets live in `public/styles/*.json` (ancient, atlas, cinderwood, clean, cyberpunk, darkSeas, default, frostbite, gloom, ink, light, monochrome, night, pale, watercolor) [V]; textures `public/images/pattern1-6.png`, `waves.png`, `whitecaps.png` [V].
- Style model: one record per layer with `attrs` (stroke, stroke-width, dasharray, fill, opacity, font-family/size, filter = `url(#id)` or CSS function such as `sepia(0.6)`/`blur(Npx)`), `options`, `groups`; states have a blurred halo group; vignette uses a mask; labels use `text-shadow` [V]. https://raw.githubusercontent.com/Azgaar/Fantasy-Map-Generator/master/docs/architecture/style.md
- Relief icons: drawn as `<use>` referencing `<symbol>`s; lowland relief from per-biome weighted icon pools with a density 0–250, hills/mountains (height ≥50) by elevation; no overlap avoidance described [V]. A WebGL2 layer refactor exists with SVG fallback (mirror commit, [P]). https://raw.githubusercontent.com/Azgaar/Fantasy-Map-Generator/master/docs/prd/relief-icon-pools.md
- Burg icon sets include a `watabou` set (city/fort/village svgs) [V]. Relief icon artwork licence: not confirmed separately from the MIT code **[U]**.
- Fonts: `Azgaar/fmg-fonts` adds OFL fonts via linked `@font-face` CSS (Monomakh Unicode, Germanica) [V]. https://github.com/Azgaar/fmg-fonts
- Export: SVG, PNG (fragment/tiled), JSON/GeoJSON for GIS [V].

### 3.3 Dungeon Scrawl
- Browser app, exports SVG and PNG, multi-page PDF; style presets, layer opacity/blend modes; hatching (incl. "inverted hatching" for above-ground) is a preset feature applied to wall edges [V]. https://probabletrain.itch.io/dungeon-scrawl/devlog , https://news.ycombinator.com/item?id=23614042
- Internal rendering engine: **[U]** not found (no public source; blog at blog.dungeonscrawl.com not retrieved). Developer confirmed isometric = rotate + squeeze transform [V]. Visually it is polygon-boolean geometry with offset hatch lines clipped to a band outside the wall – reproducible with `polygon-clipping` + `hachure-fill` **[U]**.

### 3.4 Red Blob Games
- mapgen2 "map icons": drop one hand-drawn SVG icon at each polygon centre; polygons are evenly-but-irregularly spaced so icons look scattered; icon set CC BY 4.0 [V]. https://www.redblobgames.com/maps/mapgen2/map-icons.html
- mapgen4: 3D render with custom projection/outline; convention that rivers/coasts are top-down, mountains/trees side-on [V]. https://www.redblobgames.com/maps/mapgen4/
- Hill/mountain icon interpolation experiment (2025) [V]. https://www.redblobgames.com/x/2547-mountain-icon/

---

## 4. Label placement

### 4.1 Rules (Imhof 1962/1975 via literature)
- Point labels: horizontal, close to the point, label above preferred to below; Imhof recommends *upper-right*; later systems use 8 candidate positions ranked (ArcGIS: 1 = right & slightly above; 7–8 worst) [V]. A 2024 study found users prefer *above* the point [V]. https://arxiv.org/abs/2407.11996 , https://cartogis.org/docs/proceedings/archive/auto-carto-6/pdf/a-program-for-automatic-name-placement.pdf
- Line features (rivers, streets): label along the feature, on the upper/smoother side, letter-spaced, never upside-down (own summary of Imhof, [U]).
- Area features (wards, districts): letter-spaced, following the area's long axis, inside the area; spread letters, not words.
- Never cross a feature boundary with a label if avoidable; labels of the same class share size/style.

### 4.2 Algorithms and JS libraries
- Greedy 8-position with priority order + collision index (mapbox/maplibre approach: check then insert into a grid index; collided labels dropped; 100 px viewport padding for stability) [V]. https://github.com/mapbox/mapbox-gl-native/wiki/Collision-Detection , https://app.unpkg.com/maplibre-gl@6.12.0/files/src/symbol/collision_index.ts
- Simulated annealing: d3-labeler (1000 Monte Carlo sweeps, `alt_energy` hook) [V] https://github.com/tinker10/D3-Labeler ; TS port `label-locator` 0.0.4 MIT [V]; `@d3fc/d3fc-label-layout` 7.0.4 MIT (greedy, annealing, remove-overlaps) [V]; `labelgun` 6.1.0 MIT (weight-based show/hide on rbush) [V].
- Spatial index: `rbush` 4.0.1 MIT [V].
- Recommended: static map ⇒ you can afford SA. Do greedy first (fast, deterministic with seed), then a short SA pass (≈2000 iterations) on the energy = overlaps×w1 + leader length×w2 + position-rank×w3 + feature-overlap×w4. Deterministic seed keeps batch output stable.

### 4.3 Curved labels
- SVG `<textPath href="#id" startOffset="50%">` with `text-anchor="middle"`; text past the path end is clipped; browsers differ slightly in text length so measure with the same font metrics you embed [V]. https://www.jenkov.com/tutorials/svg/textpath-element.html , https://observablehq.com/d/1b63355e548aad14
- Smooth the guide path (Catmull-Rom), reverse it if direction would make text upside-down, use `letter-spacing` for area names. resvg supports textPath **[P]**.

### 4.4 Leader lines and numbered legend
- When a landmark label does not fit: place a numbered marker (circle with number) at the feature, keep the number in the legend; leader lines only when the marker itself must move (then straight line, no crossing, ≤ 2 bends) **[U, standard practice]**. Legend box placed by the same collision engine as a large rectangle with lowest priority; prefer corners.

---

## 5. Fonts (all SIL OFL 1.1, verified in google/fonts METADATA.pb)
| Font | Use | Source |
|---|---|---|
| IM Fell English (+ italic, SC) | body labels, legend; antique printed look | https://raw.githubusercontent.com/google/fonts/main/ofl/imfellenglish/METADATA.pb |
| Cinzel | city title, ward names (caps, letter-spaced) | https://raw.githubusercontent.com/google/fonts/main/ofl/cinzel/METADATA.pb |
| Almendra | display/ward names, lighter medieval serif | https://raw.githubusercontent.com/google/fonts/main/ofl/almendra/METADATA.pb |
| MedievalSharp | display accents | https://raw.githubusercontent.com/google/fonts/main/ofl/medievalsharp/METADATA.pb |
| Pirata One | blackletter display (use sparingly) | https://raw.githubusercontent.com/google/fonts/main/ofl/pirataone/METADATA.pb |
| Uncial Antiqua | uncial display | https://raw.githubusercontent.com/google/fonts/main/ofl/uncialantiqua/METADATA.pb |
| Grenze / Grenze Gotisch | readable blackletter-ish serif at small sizes | https://raw.githubusercontent.com/google/fonts/main/ofl/grenze/METADATA.pb |
- Caution: IM Fell from the designer's own site carries a more restrictive licence; use the Google Fonts (OFL) copies [V]. https://lists.fedoraproject.org/pipermail/fonts/2010-June/001158.html
- Azgaar's `fmg-fonts` adds Monomakh Unicode and Germanica (OFL) [V].
- Convention: title in caps display face; wards/districts in letter-spaced small caps; streets in italic; water in italic (blue or dark) – classic sourcebook styling **[U]**.

---

## 6. Terrain depiction at city scale
- Coastline: double line (dark line + lighter offset line) plus 3–6 fine parallel "water hachures" fading out, as in historical maps [V, Wikipedia hachure note]. Watabou-style water fill is flat; Azgaar adds `oceanLayers` concentric bands + wave patterns [V].
- Cliffs: hachures along steepest gradient, width ∝ slope, lit side thinner [V].
- Hills: side-view bump symbols drawn back-to-front (top of map first) so lower icons overlap higher ones; scatter at polygon/cell centres (Red Blob) [V]; interpolated hill→mountain icon idea [V].
- Rivers: tapered width toward source; use perfect-freehand for an ink outline; label along path.
- Forests: Azgaar/Perilous style = blob outline ("cloud") + individual tree sprites along the bottom edge, with optional shadows [V]; Red Blob = per-cell icons [V].
- Farmland: strip hatching per field at a per-field angle (Watabou-like fields); `hachure-fill` with gap 2–3 px at print scale **[U]**.
- Style references: Azgaar style JSONs (`ink`, `ancient`, `watercolor`) [V]; Watabou palettes [V]; "Cartographic Relief Presentation" (Imhof) for hachure theory [V].

---

## 7. Headless export pipeline (Node)
1. Build SVG string with explicit `width`/`height`/`viewBox` (librsvg uses viewBox as pixel size when width/height missing; missing all three ⇒ measures painted extents) [V]. https://macmyths.com/how-to-render-svg-code-as-an-image/
2. Embed fonts: `<style>@font-face{font-family:'IM Fell English';src:url(data:font/ttf;base64,...)}</style>` for the distributable SVG; for resvg pass the same TTFs via `fontFiles`/`fontBuffers` and set `loadSystemFonts:false` [V]. (Whether resvg honours base64 `@font-face` inside `<style>` is **[U]** – pass files explicitly anyway.)
3. `new Resvg(svg, { fitTo:{mode:'width', value:4961}, font:{fontFiles:[...], loadSystemFonts:false}, background:'#f3ead8' }).render().asPng()` [V API].
4. Optional `sharp(png).webp()` or `.tile()` for Foundry/web.
- Size: a 4961×3508 RGBA PNG is ~70 MB raw; compressed typical 8–25 MB for line art with paper texture, 2–6 MB without texture **[U, estimate]**. SVG file size driven by rough.js multi-stroke paths: a 3000-building city ≈ 6000 paths ≈ 3–8 MB; use `fixedDecimalPlaceDigits:1`, `disableMultiStroke` for small buildings, and `<use>` symbols for trees/hills **[U]**.
- Performance: resvg-js ~2–4× faster than sharp for SVG rasterising (2023 self-benchmark) [P]; biggest costs are full-page filters and tens of thousands of tiny paths **[U]**. Memory: watch `limitInputPixels` in sharp (default 268 M px, fine for A3) [V].
- Determinism: rough.js `seed`, your PRNG for placement, same font files ⇒ byte-identical PNGs across runs (modulo resvg version) **[U]**.

---

## 8. Foundry VTT / Universal VTT
- Foundry v13 `SceneData`: `width`, `height` (canvas px, normally background size), `padding` (fraction, default 0.25 per community docs), `background` (TextureData with `src`), `grid: GridData{type, size, style, thickness, color, alpha, distance, units}`, `walls: WallData[]` (`c:[x0,y0,x1,y1]`, `move`, `sight`, `light`, `sound`, `door`, `ds`, `dir`, `threshold`), `lights: AmbientLightData[]`, `notes: NoteData[]`, `navigation`, `initial{x,y,scale}` [V]. https://foundryvtt.com/api/v13/interfaces/foundry.documents.types.SceneData.html , .../GridData.html , .../WallData.html
- Import routes: right-click scene → "Import Data" overwrites an existing scene (needs a scene to exist) [V] https://foundryvtt.com/article/scenes/ ; modules such as Auto Wall Companion import walls JSON (set padding 0 first) [V]; Quick Battlemap Importer applies grid/walls/lights from JSON (Dungeon Alchemist format) [V]; One Page Parser builds scenes from Watabou's One Page Dungeon PNG+JSON [V]. No module imports Watabou *city* JSON [V].
- Azgaar's `azgaar-foundry` (MIT, rewritten for v12) imports the FULL JSON into journals + map notes/pins; the picture is linked manually; grid/walls/lights not handled [V]. https://github.com/Ethck/azgaar-foundry
- Universal VTT (`.dd2vtt`/`.uvtt`) spec (Arkenforge): `format`, `resolution{map_origin{x,y}, map_size{x,y} (squares), pixels_per_grid}`, `line_of_sight` (arrays of [x,y] in squares), `objects_line_of_sight`, `portals[{position, bounds, rotation (rad), closed, freestanding}]`, `environment{baked_lighting, ambient_light}`, `lights[{position, range, intensity, color, shadows}]`, `image` (base64 PNG/WebP) [V]. https://arkenforge.com/universal-vtt-files/ Foundry needs a UVTT importer module (e.g. Universal Battlemap Importer) **[U]**.
- What a city overview scene needs: a square grid (e.g. 1 square = 50–100 ft at city scale, or gridless `type 0`), `pixels_per_grid` ≈ 50–100, map pixel size ≤ 16384 per side (WebGL texture limit) **[U]**, optional walls only for the city wall polyline (as `move`/`sight`-blocking) and gate segments as doors, optional lights for lamps/beacons, and `notes` for numbered landmarks linked to journal pages. Export two PNGs: one with labels (player handout) and one without labels + legend (scene background) **[recommendation]**.

---

## 9. Accessibility and print
- Minimum label size in print: 6 pt absolute floor, prefer ≥8 pt (Minnesota IT map guide) [V] https://mn.gov/mnit/assets/Accessibility%20-%20map-design-guide%20%28updated%29_tcm38-565153.pdf ; Macmillan recommends ≥12 pt where possible [V]. At A3/300 DPI, 8 pt = 33 px; 6 pt = 25 px.
- Contrast: text 4.5:1 (small) / 3:1 (large); fills vs outlines ≥3:1 (Massachusetts checklist) [V] https://www.mass.gov/doc/static-digital-map-accessibility-checklist/download
- On a sepia base (#f3ead8 paper, #2b2118 ink), ink text passes easily; mid-brown secondary labels (#6b4e2e) need checking; avoid red/green-only encodings [V].
- Accent colours: Okabe-Ito 8-colour CVD-safe palette (also luminance-distinct) or Paul Tol's muted/light sets for more classes [V] https://ctan.math.illinois.edu/macros/latex/contrib/colorblind/documentation.pdf ; pair colour with pattern/number (redundant encoding).
- Line weights for print: hairlines ≥0.25 pt (≈1 px at 300 DPI); keep rough.js `strokeWidth` ≥ 1.2 px at output scale **[U]**.

---

## 10. Open questions / things to verify in a spike
1. Import `roughjs` ESM in plain Node and call `rough.generator().toPaths()` without jsdom [U].
2. resvg-js rendering of `feTurbulence`/`feDisplacementMap`/`textPath` and time for a full A3 page with and without paper filter [P/U].
3. File sizes for a 3000-building city (SVG and PNG) with/without multi-stroke [U].
4. Which UVTT importer module to recommend for Foundry v13 [U].
5. Licence of Azgaar's relief icon artwork if you intend to reuse symbols [U] – safer: draw your own or use Red Blob's CC BY 4.0 set.

## Sources (additional)
- rough.js site https://roughjs.com/ ; resvg-js README https://github.com/thx/resvg-js/blob/main/README.md ; resvg README https://github.com/linebender/resvg/blob/main/README.md
- sharp constructor docs https://sharp.pixelplumbing.com/api-constructor/
- Watabou source files: https://raw.githubusercontent.com/watabou/TownGeneratorOS/master/Source/com/watabou/towngenerator/mapping/CityMap.hx , .../Brush.hx , .../Palette.hx ; devlog https://watabou.itch.io/medieval-fantasy-city-generator/devlog/6592/source-code
- Azgaar repo tree https://api.github.com/repos/Azgaar/Fantasy-Map-Generator/git/trees/master?recursive=1
- Mapbox label docs https://docs.mapbox.com/help/dive-deeper/optimize-map-label-placement.md
- Hachure/terrain: https://en.wikipedia.org/wiki/Terrain_cartography , https://www.esri.com/news/arcuser/0701/althillshade.html
- Canvas libs comparison (secondary) https://www.pkgpulse.com/guides/node-canvas-vs-napi-rs-canvas-vs-skia-canvas-server-2026
