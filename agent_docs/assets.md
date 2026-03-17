---
title: "Asset Pipeline"
description: "HD upscaling, 3D model generation, and encyclopedia content pipelines"
category: "agent-docs"
created: 2026-03-13
updated: 2026-03-16
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
| DLL sprites | ~1500+ | GOKRES/STRATEGY/TACTICAL/COMMON/ALSPRITE/EMSPRITE.DLL | Resource IDs |

**Prerequisite**: `data/base/EData/` must contain extracted BMPs from a legal game copy. Extracted game data also at `~/Desktop/Programming/star-wars-rebellion/GData/`.

### Upscaling Tools

**Primary: waifu2x-ncnn-vulkan** (INSTALLED)

```bash
# Wrapper with model path baked in:
~/.local/bin/waifu2x -i input.png -o output.png -n 1 -s 4 -f png

# Binary: ~/tools/waifu2x/waifu2x-ncnn-vulkan-20250915-macos/
# Models: models-cunet (default), models-upconv_7_anime_style_art_rgb, models-upconv_7_photo
```

Universal binary (x86_64 + arm64). Native Apple Silicon via Vulkan. Tested on M4 Max — 400x200 → 1600x800 in <1s.

**Important**: Do not call waifu2x directly on `EData/` — the original BMPs are indexed-palette and use extensionless names (`EDATA.042`). Use `scripts/upscale-assets.py` which handles palette→RGB conversion and renames output to `EDATA_NNN.png` (the format `encyclopedia.rs` expects).

**Comparison: PBRify_UpscalerV4 via chaiNNer or Upscayl** (MODELS DOWNLOADED)

```
~/tools/upscale-models/4x-PBRify_UpscalerV4.pth  (134MB, Kim2091, May 2025)
~/tools/upscale-models/4x-UltraSharpV2.pth        (134MB, Kim2091, May 2025)
```

**PBRify_UpscalerV4** — DAT2 architecture, purpose-built for 2000s-era game textures with DDS compression removal. Supersedes the older 4x SGI model by the same author. **UltraSharpV2** is a strong secondary comparison. Load `.pth` files in chaiNNer (chainner.app, macOS MPS backend) or **Upscayl v2.5** (Feb 2026, free, wraps ncnn-vulkan with custom model import).

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

### Upscaling Strategy (Per Category)

| Category | Best Method | Why |
|----------|------------|-----|
| Capital ships | Gemini edit | Invents coherent hull detail |
| Facilities | Gemini edit | Can add plausible architectural detail |
| Portraits | Vertex AI Imagen | No face modernization |
| Fighters/sprites | Vertex AI Imagen | No object duplication |
| Planets | Vertex AI Imagen | No non-canonical detail |
| Troops/beasts | Vertex AI Imagen | Faithful enlargement |

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

### File Organization

```
data/
├── base/              # Original game data (user-extracted, gitignored)
│   ├── *.DAT
│   ├── EData/         # Original BMPs (EDATA.NNN, 3-digit zero-padded)
│   └── TEXTSTRA.DLL
├── hd/                # AI-upscaled PNGs (generated, checked in)
│   ├── EData/         # EDATA_NNN.png (underscore, PNG extension)
│   ├── sprites/       # Upscaled DLL sprites
│   └── manifest.json
└── models/            # 3D tactical models (final outputs only)
    ├── optimized/     # DRACO-compressed GLBs (from prepare-rebellion-models.sh)
    └── sprites/       # Pre-rendered sprite sheets (from Blender)

scripts/
├── models-staging/    # Raw GLBs from Hunyuan/Meshy (gitignored, not in data/)
└── logs/              # JSONL generation logs (gitignored)
```

### Code Integration

`crates/rebellion-render/src/encyclopedia.rs` loads HD PNGs with BMP fallback:

1. Check `data/hd/EData/EDATA_NNN.png` (HD upscaled)
2. Fall back to `data/base/EData/EDATA.NNN` (original BMP)

Requires `"png"` feature in `rebellion-render/Cargo.toml`:
```toml
image = { version = "0.25", default-features = false, features = ["bmp", "png"] }
```

---

## Pipeline 2: 3D Tactical Combat Models

### Architecture: Pre-Rendered Sprite Sheets

macroquad 0.4 has no native glTF/GLB loading (GitHub issue #456, still open). Solution: pre-render 3D models to sprite sheets via Blender — authentically how the original 1998 game worked.

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

Source: 142 Empire lines (VOICEFXE.DLL, IDs 15001-15144) + ~140 Alliance lines (VOICEFXA.DLL).

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

316+ UI elements extracted from game DLLs, organized in `assets/references/ref-ui/` (10 categories).
See `assets/references/ref-ui/INDEX.md` for full inventory with resource ID numbering scheme.

### Unextracted DLLs (from `~/Desktop/Programming/star-wars-rebellion/`)

| DLL | Contents | Frames |
|-----|----------|--------|
| **ALBRIEF.DLL** | Alliance briefing/cockpit screens | ~20-50 |
| **EMBRIEF.DLL** | Empire briefing/cockpit screens | ~20-50 |
| **ALSPRITE.DLL** | C-3PO + R2-D2 animated advisor (Alliance) | 251 |
| **EMSPRITE.DLL** | Imperial droid animated advisor (Empire) | 250 |
| **REBDLOG.DLL** | Dialog box UI chrome | ~20-30 |

Extract via: `uv run scripts/extract-dll-resources.py ALBRIEF.DLL --output ref-ui/11-alliance-briefing/`

### Upscaling Strategy

| Category | Files | Method | Why |
|----------|-------|--------|-----|
| Briefing backgrounds | ~40-100 | Gemini edit | Large scenes, benefits from detail |
| Droid advisor frames | 501 | Vertex AI Imagen | Must be consistent across all frames |
| Buttons/controls | 33 | Vertex AI Imagen | Pixel-perfect state alignment |
| Status panels | 129 | Vertex AI Imagen | Entity recognition must be preserved |
| Event screens | 83 | Gemini edit | Narrative scenes, generative detail |
| Everything else | ~30 | Vertex AI Imagen | Faithful enlargement |

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
