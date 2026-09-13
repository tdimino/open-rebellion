---
title: "Asset Pipeline"
description: "HD upscaling, 3D model generation, and encyclopedia content pipelines"
category: "agent-docs"
created: 2026-03-13
updated: 2026-09-10
tags: [asset-pipeline, upscaling, 3d-models, audio, references]
---

# Asset Pipeline

Three pipelines handle HD upscaling, 3D model generation, and encyclopedia content. Read when adding visual assets, generating models, or working on the encyclopedia viewer.

## Differentiation from swrebellion.net

The swrebellion.net community's 25th Anniversary pack (2023) substitutes original art with new interpretations. Our approach: **upscale the original 1998 assets with 2026 AI technology** and **generate 3D models for tactical combat**. Authentic aesthetic, freed from Win32.

## Pipeline 1: HD Upscaling

### Source Assets

Original BMPs from game's installed directories:

| Category | Count | Source | EDATA Range |
|----------|-------|--------|-------------|
| Facilities | ~14 | EData/ | 001-014 |
| Troops | ~10 | EData/ | 015-024 |
| Special forces | ~9 | EData/ | 025-033 |
| Fighters | ~8 | EData/ | 034-041 |
| Capital ships | ~30 | EData/ | 042-071 |
| Characters (major) | ~6 | EData/ | 072-077 |
| Characters (minor) | ~54 | EData/ | 078-137 |
| Systems (planets) | ~200 | EData/ | 138+ |
| DLL UI | 2,303 BMPs and 3,988 advisor frames | GOKRES/STRATEGY/TACTICAL/COMMON/ALSPRITE/EMSPRITE.DLL | Resource IDs |

**Prerequisite**: `data/base/EData/` must contain extracted BMPs from a legal game copy. Extracted game data also at `~/Desktop/Programming/star-wars-rebellion/GData/`.

### Faithful-HD contract

The [September 2026 plan](../docs/plans/2026-09-10-faithful-hd-pipeline/)
supersedes universal model routing. `original-parity` is the default and only
profile that counts toward interface parity. `faithful-hd` is optional and loads
only manifest-approved assets. Generative restoration is experimental.

```bash
# Generate deterministic palette-preserving candidates
uv run scripts/faithful_hd_pipeline.py generate --dll gokres-dll

# Verify every recorded source and output hash plus exact reconstruction
uv run scripts/faithful_hd_pipeline.py verify

# Approve one reviewed asset for the faithful-HD runtime profile
uv run scripts/faithful_hd_pipeline.py approve gokres-dll/17001 \
  --reviewer "reviewer-name" --evidence "docs/qa/evidence-path"
```

The first route is exact nearest 4x. It preserves indexed palettes and records
source, palette, route, configuration, generator, and output hashes. Existing
UltraSharp outputs are comparison artifacts until regenerated or imported with
matching provenance and explicit review.

**Available models** (all at `~/tools/upscale-models/`, all loadable via Spandrel):

| Model | Architecture | Size | Best For |
|-------|-------------|------|----------|
| 4x-UltraSharpV2.pth | DAT2 | 134MB | Painted-art comparison candidate |
| 4x-PBRify_UpscalerV4.pth | DAT2 | 134MB | Game textures (runner-up) |
| 4x-FSDedither-Riven.pth | ESRGAN | 64MB | Ordered-dithered art (fastest: 4.5s/20) |
| 4xTextures_GTAV_rgt-s_dither.pth | RGT | 130MB | Dithered+JPEG game textures |

**Fallback: Real-ESRGAN x4plus** (ncnn-vulkan, no Python deps)

```bash
cd ~/tools/realesrgan && ./realesrgan-ncnn-vulkan -i input.png -o output.png -n realesrgan-x4plus -s 4 -f png
```

**Shootout script** for re-running comparisons with new models:

```bash
python3 scripts/model-shootout.py --local-only       # All local models on 20 test BMPs
python3 scripts/model-shootout.py --models 3,5        # Specific models by ID
python3 scripts/model-shootout.py --html-only         # Regenerate comparison HTML
```

Test samples: `scripts/shootout-samples.json` (20 BMPs, 4 per category)

**Important**: Do not flatten indexed source assets to RGB for deterministic
routes. Preserve palette indices, transparency, and protected masks throughout.

### Gemini Generative Upscale (Method A — TESTED)

Gemini 3 Pro Image via `edit_image.py`. Generates a higher-resolution reimagining.
400x200 → ~1376x768. Cost: ~$0.13/image at 2K, ~$0.24 at 4K.

```bash
python3 ~/.claude/skills/nano-banana-pro/scripts/edit_image.py \
  "Enhance this image to high resolution with sharp details and crisp edges. Preserve all elements exactly." \
  input.png --aspect-ratio 16:9 --output ./upscaled
```

**Results by category:**
- **Capital ships**: HIGH fidelity — hull shape, colors, angle preserved. Excellent coherent detail.
- **Planets**: MEDIUM — adds plausible but non-canonical surface detail (Aztec patterns on Coruscant).
- **Portraits**: LOW — modernizes face to photorealistic, loses 1998 painted aesthetic.
- **Fighters**: LOW — hallucinates extra ships (1 X-Wing → squadron of 7).

**Prompt refinement to reduce hallucination:**
- Add "Do not add any new objects or elements" for fighters/portraits
- Add "Maintain the exact 1998 pre-rendered CGI art style" for portraits
- Add "This is a single ship, do not duplicate it" for fighters
- Use temperature 0.3 if the API exposes it (edit endpoint currently doesn't)
- For portraits: try "Preserve the painted, slightly stylized art quality. Do not make photorealistic."

**Best for:** Capital ships, large structures, facilities. Use traditional upscaling for fighters, sprites, portraits.

### Vertex AI Imagen 4.0 Upscale (Non-Generative — TESTED, WORKING)

Pure super-resolution via `imagen-4.0-upscale-preview`. No text guidance, no hallucination.
GCP project: `dream-daimon`. Vertex AI API enabled.

```bash
# Single image
GOOGLE_CLOUD_PROJECT=dream-daimon uv run scripts/vertex-upscale.py --input image.png --factor 4

# Batch directory
GOOGLE_CLOUD_PROJECT=dream-daimon uv run scripts/vertex-upscale.py --input-dir originals/ --output vertex/
```

**Tested results (400x200 → 1600x800):**
- Luke Skywalker portrait: PERFECT — preserves 1998 painted CGI style, no modernization
- Mon Cal Cruiser: PERFECT — faithful enlargement, no invented detail
- Nebulon-B Frigate: PERFECT — thin spar preserved cleanly
- Coruscant: PERFECT — cityscape detail preserved
- X-Wing: quota limit hit (5 req/min default), retry after cooldown

**Best for:** Portraits, fighters, sprites — anything where fidelity > detail invention.
**Quota:** 5 requests/minute default. Request increase for batch processing.

### Upscaling Strategy

Use family-specific routes. Text, controls, chrome, masks, and indexed animation
stay deterministic. SwinIR and HAT lead the next painted-art comparison;
Real-ESRGAN and UltraSharp remain challengers. Diffusion outputs belong only to
the experimental remaster profile.

| Category | Method | Status |
|----------|--------|--------|
| Text, controls, chrome, masks | Nearest 4x; Scale2x/4x challenger | Deterministic foundation implemented |
| Indexed animation | Whole-sequence deterministic transform | Classification and temporal gates pending |
| Painted art | SwinIR/HAT/Real-ESRGAN/UltraSharp family shootout | Pending |
| Generative restoration | Experimental remaster only | Excluded from parity and faithful HD |

**Other tools available** (tested, still installed, usable if needed):
- **Vertex AI Imagen 4.0**: Non-generative super-resolution, zero hallucination. $0.005/img, 5 RPM quota. Requires `gcloud auth`. Script: `scripts/vertex-upscale.py`. Good fallback for any images where UltraSharp produces artifacts.
- **Topaz Gigapixel CGI**: $0.05/img via Replicate API (`topazlabs/image-upscale`, `enhance_model="CGI"`). Matched UltraSharp quality but at cost.
- **Gemini Pro + references**: Generative upscale with reference images for character identity. Stalled on reference contamination. Scripts: `scripts/gemini-upscale.py`, `scripts/select-references.py`. Could revisit for portraits if UltraSharp isn't faithful enough to specific characters.
- **Real-ESRGAN x4plus**: Free ncnn-vulkan CLI at `~/tools/realesrgan/`. Fast but slightly softer output than UltraSharp on this art style.
- **FSDedither Riven**: Fastest local model (4.5s/20 images). Purpose-built for ordered-dithered game textures. Worth revisiting if any BMPs show persistent dithering artifacts after UltraSharp.

### Scale Factors

| Source | Factor | Output | Use Case |
|--------|--------|--------|----------|
| 64x64 | 4x | 256x256 | Unit icons, small sprites |
| 128x128 | 4x | 512x512 | Character portraits, ship profiles |
| 256x256 | 4x | 1024x1024 | Encyclopedia full images |

### BMP Preprocessing (Critical)

Rebellion BMPs are 256-color indexed palette. All upscalers require RGB:
```python
from PIL import Image
img = Image.open("EDATA.042").convert('RGB')  # palette → RGB before upscaling
```

### Batch Script

```bash
uv run scripts/upscale-assets.py                    # All EData BMPs
uv run scripts/upscale-assets.py --input EDATA.042   # Single file
uv run scripts/upscale-assets.py --scale 8            # 8x for hero assets
uv run scripts/upscale-assets.py --dry-run            # Preview without processing
```

### Reference Image Collections (for Gemini multi-image upscaling)

1,088 usable reference images across 17 collections. **Audited 2026-03-27**: non-OT portraits removed, text-only book pages quarantined.

| Collection | Count | Content |
|-----------|-------|---------|
| **Wookieepedia Portraits** | 11 | OT-era film stills only (49 non-OT quarantined) |
| **CCG Card Art** | 28 | OT-era Decipher CCG painted art (28 non-OT quarantined) |
| **Illustrated Books** | 895 | 5 book extracts: characters (189), ROTJ (91), planets (211), weapons (199), starfighters (204). 170 text pages quarantined. |
| **Cross-Sections** | 33 | 1998 OT vehicle cutaway diagrams (Hans Jenssen/Richard Chasemore) |
| **GB Concept Art** | 25 | Galactic Battlegrounds concept art (MixnMojo). Same era LucasArts. |
| **Game EData** | 91 | Characters (7), ships (5), facilities (9), fighters (4), troops (8), specials (6), damage (14), missions (8), planets (9), battles (7), squadrons (14) |

Full index: `assets/references/INDEX.md` — 1,320 images across 21 collections.

### Automated Reference Selection (portraits only — under iteration)

`scripts/select-references.py` uses Gemini Flash Lite ($0.001/BMP) to select the best 3-5 references per BMP:
1. Routes BMP to pack via `scripts/reference-packs.json` (12 packs, 24 routing rules)
2. Builds contact sheet of candidates, pre-filtered by entity slug when available
3. Vision LLM picks best references (rejects text pages, non-OT content)
4. Output: `data/reference-selections.json` (consumed by `scripts/gemini-upscale.py`)

Entity mapping: `data/resource-entity-map.json` — 162 GOKRES resource IDs → character/ship names via DAT offset formulas.

**Status**: References contaminate non-portrait packs. Only use for portrait-major, portrait-mini, character-panel. All other packs use Vertex AI Imagen (no references needed).

### DLL Upscale Pipeline (2,303 BMPs)

**HD override contract**: `data/hd/{dll-dir-name}/{resource_id}.png` plus
`data/hd/manifest.json`. `BmpCache` checks reviewed entries only when
`OPEN_REBELLION_ASSET_PROFILE=faithful-hd`, validates the source and output
digests, then decodes those verified bytes. Otherwise it uses original BMPs.

| DLL | Staged BMPs | Path |
|-----|------------|------|
| COMMON | 321 | `data/base/ui/common-dll/BMP/` |
| GOKRES | 580 | `data/base/ui/gokres-dll/BMP/` |
| STRATEGY | 1,042 | `data/base/ui/strategy-dll/BMP/` |
| TACTICAL | 288 | `data/base/ui/tactical-dll/BMP/` |

Stage these assets directly from the original game DLLs in `data/base/` using the
dependency-free Go extractor:

```bash
go run ./tools/stage-ui-assets
go run ./tools/stage-ui-assets --verify
```

Use `--source` or `--output` to override those default directories. Existing BMPs
are left unchanged when their contents match; use `--force` to replace differing
files.

### Advisor Frame Staging

The same extractor also preserves the full type-302 droid corpus from an owned
installation:

| DLL | Standard BMP anchors | Type-302 frames | Path |
|-----|---------------------:|----------------:|------|
| ALSPRITE | 38 | 1,640 | `data/base/ui/alsprite-dll/` |
| EMSPRITE | 34 | 2,348 | `data/base/ui/emsprite-dll/` |

Native builds read these ignored files directly. Browser builds place the same
bytes in `runtime.orpk`; the checked-in repository contains the decoder and
packaging logic, never the copyrighted game resources. Standard BMP anchors
provide the original indexed pixels and palette. Type-302 scanlines preserve
unchanged pixels and add authored byte deltas to the remaining pixels, matching
the original renderer.

### Tactical 3D raw staging

The same dependency-free extractor has an opt-in content-addressed path for all
87 type-301 DirectX meshes and 397 type-303 texture/palette resources:

```bash
go run ./tools/stage-ui-assets --tactical-3d-only
go run ./tools/stage-ui-assets --tactical-3d-only --verify
```

Convert or verify that raw store separately:

```bash
go run ./tools/stage-ui-assets --tactical-3d-convert
go run ./tools/stage-ui-assets --tactical-3d-convert --verify
```

The ignored `data/base/ui/tactical-dll/TACTICAL3D/` store retains exact PE
identifiers and hashes without using named resources as paths. P55 decodes all
87 meshes, 370 indexed images, and 27 palettes into the ignored `runtime/`
subtree. These assets are not yet included in `runtime.orpk`; follow the
[tactical 3D asset plan](../docs/plans/2026-09-12-feat-tactical-3d-asset-pipeline.md).

**Batch upscale command** (Vertex, all non-portrait packs):
```bash
uv run scripts/vertex-upscale-batch.py              # all 2,231 BMPs
uv run scripts/vertex-upscale-batch.py --pack ship-status --limit 5  # single pack test
uv run scripts/vertex-upscale-batch.py --dry-run     # cost/time estimate
```

**Cost**: $11 for all 2,231 BMPs. **Time**: ~8h at 5 RPM default quota (request increase for faster runs). **Comparison**: `data/hd/comparison.html` (generated by `scripts/build-comparison-html.py`).

### File Organization

```
data/
├── base/              # Original game data (user-extracted, gitignored)
│   ├─�� *.DAT
��   ├─�� EData/         # Original BMPs (EDATA.NNN, 3-digit zero-padded)
│   ├── ui/            # Staged BMP/{id}.bmp and TYPE302/{id}.bin resources
│   └── TEXTSTRA.DLL
���── hd/                # AI-upscaled PNGs (generated, checked in)
│   ├── EData/         # EDATA_NNN.png (underscore, PNG extension)
│   ├── common-dll/    # HD overrides for COMMON.DLL
│   ├── gokres-dll/    # HD overrides for GOKRES.DLL
│   ├── strategy-dll/  # HD overrides for STRATEGY.DLL
│   ├── tactical-dll/  # HD overrides for TACTICAL.DLL
│   └── README.md
└── models/            # 3D tactical models (final outputs only)
    ├── optimized/     # DRACO-compressed GLBs (from prepare-rebellion-models.sh)
    └─�� sprites/       # Pre-rendered sprite sheets (from Blender)

scripts/
├── models-staging/    # Raw GLBs from Hunyuan/Meshy (gitignored, not in data/)
└── logs/              # JSONL generation logs (gitignored)
```

### Code Integration

`crates/rebellion-render/src/encyclopedia.rs` follows the active asset profile:

1. `original-parity` loads `data/base/EData/EDATA.NNN` with nearest sampling.
2. `faithful-hd` may load `data/hd/EData/EDATA_NNN.png` only when the matching
   `edata/EDATA_NNN` manifest record is approved.
3. Missing, unapproved, or invalid HD content falls back to the original BMP.

Requires `"png"` feature in `rebellion-render/Cargo.toml`:
```toml
image = { version = "0.25", default-features = false, features = ["bmp", "png"] }
```

---

## Pipeline 2: Tactical 3D assets

### Original-parity architecture

The original 1998 tactical renderer used real Direct3D Retained Mode meshes,
not pre-rendered ship sprite sheets. Strict parity therefore starts with the 87
type-301 DirectX `.x` resources and their type-303 textures. See the
[tactical 3D asset plan](../docs/plans/2026-09-12-feat-tactical-3d-asset-pipeline.md).
The runtime target is a compact deterministic mesh pack rendered through
macroquad inside the original battle aperture. GLB is useful for conversion
inspection, but it is not required in the WASM runtime.

### Experimental-remaster model generation

The generation workflow below is an optional extension. Hunyuan, Meshy, or
other replacement geometry must use the `experimental-remaster` profile and
must never count as original-interface or tactical-rendering parity evidence.
Pre-rendered sprite sheets remain useful as a low-end experimental fallback,
not as a reconstruction of the original renderer.

### 3D Generation: Dual-Provider Strategy

Adapted from World War Watcher's proven pipeline (14 Hunyuan3D Pro models).

**Primary: Hunyuan3D Pro via fal.ai**
- Endpoint: `fal-ai/hunyuan-3d/v3.1/pro/image-to-3d`
- Cost: ~$0.375/model (Geometry $0.225 + 80K faces $0.15)
- 63 models = **~$23.63 total**
- Use synchronous `fal.run` (not queue API)
- `generate_type: "Geometry"` — untextured mesh for custom rendering

**Budget: WaveSpeedAI Hunyuan3D v3.1 Rapid**
- Endpoint: WaveSpeedAI API (Feb 2026)
- Cost: **$0.0225/model** — 16x cheaper than fal.ai Pro
- Single required parameter: `image` (reference image URL)
- Ideal for bulk iteration on non-hero units

**Comparison: Meshy (fallback)**
- `--model-type lowpoly` (Meshy-6 Low Poly Mode)
- 20 credits preview, 30 credits with texture
- Use existing skill: `uv run ~/.claude/skills/meshy/scripts/meshy_text_to_3d.py`

**Rapid iteration: Microsoft Trellis 2**
- Available via 3D AI Studio and Scenario (March 2026)
- Cost: ~$0.05-0.10/model, 1-3 min, PBR always included
- Image-only input, automatic polygon count (no user control)
- Use for cheap drafts before committing to Hunyuan Pro for hero assets

### Concept Art: nano-banana-pro → Reference Images

```
Star Wars [unit name], orthographic 3/4 front view,
white background, flat cel-shaded, no drop shadows, no text,
no labels, no annotations, no dimension lines, no arrows,
clean silhouette, stylized low-poly game asset, 1024x1024
```

**CRITICAL**: Hunyuan reproduces annotation text as 3D geometry. Negative prompt is mandatory.

### Asset Split

| Category | Count | Method | Rationale |
|----------|-------|--------|-----------|
| Capital ships (hero) | ~10 | Image-to-3D (Hunyuan) | Iconic silhouettes need reference |
| Capital ships (generic) | ~20 | Text-to-3D (Meshy) | Less silhouette-critical |
| Fighters | 8 | Image-to-3D (Hunyuan) | All iconic |
| Troops | 10 | Text-to-3D (Meshy) | Generic ground units |
| Special forces | 9 | Text-to-3D (Meshy) | Generic spec ops |
| Defense facilities | 6 | Text-to-3D (Meshy) | Stationary structures |

### Model Optimization Pipeline

```
Hunyuan3D/Meshy → raw GLBs (scripts/models-staging/)
  → strip-textures.mjs (geometry only) [not yet ported from WWW]
  → gltf-transform simplify (ratio 0.10) + optimize (weld, prune)
  → gltf-transform draco (14-bit position quantization)
  → Blender batch render (8 directional frames)
  → PNG sprite atlases (data/models/sprites/) [not yet created]
```

### Scripts

```bash
# Generate 3D models (Hunyuan + Meshy)
uv run scripts/generate-rebellion-models.py                    # All models
uv run scripts/generate-rebellion-models.py --models isd xwing # Specific
uv run scripts/generate-rebellion-models.py --status           # Check pending
uv run scripts/generate-rebellion-models.py --provider meshy   # Meshy only

# Optimize GLBs
bash scripts/prepare-rebellion-models.sh                       # All staging
bash scripts/prepare-rebellion-models.sh star-destroyer.glb    # Specific

# Render sprite sheets (requires Blender)
blender --background --python scripts/render-sprite-sheets.py  # All models
```

### Blender Addons

- **Spritehandler 2** (July 2025) — purpose-built for directional/animated sprite sheet generation, eliminates most custom scripting
- **Omnidirectional Isometric Render** (itch.io, free, April 2025) — specifically for isometric directional renders

### Sprite Sheet Format

8 directional frames per model (0°, 45°, 90°, ..., 315°):
- Standard: 128x128 per frame → 1024x128 strip atlas
- Hero units: 256x256 per frame → 2048x256 strip atlas
- Output: `data/models/sprites/{unit-id}.png`

### Model Comparison Viewer

Adapted from WWW's `scripts/model-compare.html` — Three.js side-by-side viewer for A/B testing Hunyuan vs Meshy outputs. Auto-detects staging variants.

---

## Pipeline 3: Encyclopedia Content

Original TEXTSTRA.DLL strings extracted via pelite (already implemented). Encyclopedia text entries written from canonical Star Wars sources:

- Store as: `data/encyclopedia.json` mapping `dat_id → { name, description, faction, category }`
- Render in existing encyclopedia viewer text panel

---

## Pipeline 4: Reference Image Generation (nano-banana-pro)

Generate new assets using curated reference collections from the original game.
Up to 14 reference images per generation via Gemini 3 Pro Image (or Flash via `--fast`).

### Reference Collections (11 categories, 91 images)

Each collection in `assets/references/ref-{category}/` contains:
- `{name}.png` — 4x upscaled via waifu2x (for Gemini detail visibility)
- `{name}-orig.png` — original resolution (ground truth for style matching)
- `prompt-template.txt` — category-specific prompt with `{description}` placeholder

| Collection | Refs | Use Case |
|------------|------|----------|
| `ref-capital-ships` | 5 | Ship encyclopedia entries, 3D model concept art |
| `ref-fighters` | 4 | Fighter profiles, squadron sprites |
| `ref-characters` | 7 | Character portraits, status panels |
| `ref-planets` | 9 | System/planet encyclopedia views |
| `ref-troops` | 8 | Ground unit illustrations |
| `ref-special-forces` | 6 | Covert operative portraits |
| `ref-facilities` | 9 | Military/industrial architecture |
| `ref-missions` | 8 | Mission briefing scenes |
| `ref-damage-diagrams` | 14 | Ship technical schematics |
| `ref-squadron-sprites` | 14 | Top-down fighter sprites |
| `ref-battle-backgrounds` | 7 | Tactical combat space scenes |

### Scripts

```bash
# Curate reference collections from MetasharpNet editor
uv run scripts/curate-references.py --list              # show categories
uv run scripts/curate-references.py --upscale            # extract + upscale all

# Generate new assets
uv run scripts/generate-rebellion-assets.py --list       # show categories
uv run scripts/generate-rebellion-assets.py \
  --category capital-ships \
  --description "Victory-class warship, shorter wedge hull"
uv run scripts/generate-rebellion-assets.py \
  --category planets \
  --manifest data/manifests/planets.json               # batch mode

# Compare Pro vs Flash quality
uv run scripts/generate-rebellion-assets.py \
  --category fighters \
  --description "TIE Defender, three-wing configuration" \
  --compare
```

### Safety Filter Note

Gemini blocks "Star Wars" + weapon terms (e.g., "Star Destroyer"). Prompt templates use
"sci-fi space" phrasing instead. The reference images anchor the style — Gemini matches
the 1998 CGI aesthetic from the references, not from the text prompt.

### ASSET ARCHAEOLOGY STATION (Reference Viewer)

`scripts/reference-viewer.html` — PHOSPHOR VIGIL CRT-styled comparison viewer for all
11 reference collections. Shows original vs 4x-upscaled side-by-side with draggable slider.

```bash
cd open-rebellion && python3 -m http.server 8080
# Open http://localhost:8080/scripts/reference-viewer.html
```

Keyboard: `←`/`→` nav images, `[`/`]` nav categories, `G` pixel grid overlay, `D` slider/side-by-side, `Space` fit/1:1 zoom.

---

## Pipeline 5: Audio Generation

Voice cloning, SFX, music, and audio upscaling for combat, missions, and atmosphere.

### Tools (all macOS ARM64)

| Domain | Tool | Location | Type |
|--------|------|----------|------|
| Voice cloning | Voicebox (Qwen3-TTS + MLX) | `~/Applications/Voicebox.app` | Local |
| Audio upscaling | LavaSR v2 | `~/tools/lavasr/` | Local |
| Music generation | ACE-Step 1.5 | `~/tools/ace-step/` | Local |
| Sound effects | ElevenLabs SFX v2 | API | Credits |
| SFX fallback | AudioLDM2 | `diffusers` package | Local |

### Voice Cloning Pipeline

```
Original voice WAVs (8-11kHz, VOICEFX*.DLL)
  → LavaSR v2: upscale to 48kHz
  → Select 15-20s reference per speaker
  → Qwen3-TTS Base: clone to new dialogue lines
  → Output: data/sounds/voices/{faction}/{line_id}.wav
```

Source: 132 Empire lines (VOICEFXE.DLL) + 153 Alliance lines (VOICEFXA.DLL). All 285 WAV files extracted.

### SFX Generation

```bash
# ElevenLabs API (best quality for transients)
curl -X POST "https://api.elevenlabs.io/v1/sound-generation" \
  -H "xi-api-key: $ELEVENLABS_API_KEY" \
  -d '{"text": "sci-fi laser cannon blast", "duration_seconds": 2}'

# AudioLDM2 local fallback (ambient loops)
from diffusers import AudioLDM2Pipeline
pipe = AudioLDM2Pipeline.from_pretrained("cvssp/audioldm2")
```

### Music Generation

```bash
cd ~/tools/ace-step && bash start_api_server_macos.sh  # MLX backend, port 8001
# Tags: "orchestral, epic, space opera, brass fanfare, strings, war"
# LoRA fine-tuning available for original soundtrack style transfer
```

---

## Pipeline 6: UI Element Upscaling

2,441 BMPs extracted from all game DLLs. Curated subsets in `assets/references/ref-ui/` (13 categories).
Full extraction in `assets/references/ref-ui-full/` with per-DLL INDEX.md files.
See `agent_docs/dll-resource-catalog.md` for complete resource ID ranges, dimensions, and content descriptions.

### Upscaling Strategy

**Superseded in September 2026.** Use the manifest-governed, family-specific
[faithful-HD pipeline](../docs/plans/2026-09-10-faithful-hd-pipeline/).
`scripts/local-upscale-batch.py` is retained only for UltraSharp comparison
candidates; its raw outputs are not runtime-approved.

**All game DLLs fully extracted.** Including 285 voice WAV files (153 Alliance + 132 Empire).

---

## WWW Pipeline Reference

World War Watcher's 3D pipeline is the direct ancestor. Key files to reference:

| WWW File | Purpose | Adapt For |
|----------|---------|-----------|
| `scripts/generate-models.py` | Hunyuan3D fal.ai batch generation | `generate-rebellion-models.py` |
| `scripts/prepare-models.sh` | GLB optimization + DRACO | `prepare-rebellion-models.sh` |
| `scripts/strip-textures.mjs` | Remove textures from GLBs | Port from WWW (not yet copied) |
| `scripts/model-compare.html` | Three.js comparison viewer | Adapt for SW units |
| `agent_docs/3d-models.md` | Pipeline docs, best practices | Reference patterns |
| `agent_docs/asset-pipeline.md` | Full asset flow diagram | Reference patterns |

### WWW Lessons Learned
- Hunyuan3D Pro via fal.ai outperformed Meshy — all 14 production models are Hunyuan
- 3/4 front view reference images produce far better geometry than side views
- `generate_type: "Geometry"` (untextured) is cheaper and better for custom materials
- DRACO position quantization: 14-bit minimum (11-bit destroys silhouettes)
- Always strip textures before optimizing (prevents texture atlas bloat)
