---
title: "Upscale All 2,231 DLL BMPs With Per-Asset Tool Routing"
type: feat
status: planning
date: 2026-03-25
project: open-rebellion
tags: [assets, upscaling, bmp, dll, hd, execplan]
---

# Upscale All 2,231 DLL BMPs With Per-Asset Tool Routing

This ExecPlan is a living document. Sections Progress, Surprises &
Discoveries, Decision Log, and Outcomes & Retrospective must be
kept up to date as work proceeds.

## Purpose / Big Picture

Open Rebellion already knows how to prefer HD PNG overrides when they exist and
fall back to the original staged BMP when they do not. The missing piece is a
safe, repeatable production plan for creating those HD overrides for every DLL
bitmap without using the wrong upscaler on the wrong art. This plan defines
that route for all 2,231 BMPs used by `COMMON.DLL`, `GOKRES.DLL`,
`STRATEGY.DLL`, and `TACTICAL.DLL`.

At the end of this plan, `data/hd/` will contain PNG files at the exact paths
`crates/rebellion-render/src/bmp_cache.rs` already checks first:
`data/hd/{dll-dir-name}/{resource_id}.png`. A player should be able to launch
the game with no code changes and see HD art automatically wherever an HD PNG
exists, while any missing asset still falls back to the staged original BMP.

## Progress

- [x] (2026-03-25 21:21Z) Read `agent_docs/assets.md`, `agent_docs/dll-resource-catalog.md`, `~/.claude/agent_docs/tools.md`, `data/hd/README.md`, `crates/rebellion-render/src/bmp_cache.rs`, `scripts/stage-ui-assets.py`, `scripts/upscale-assets.py`, and `scripts/vertex-upscale.py`.
- [x] (2026-03-25 21:21Z) Verified staged BMP counts in `data/base/ui/`: `common-dll` 321, `gokres-dll` 580, `strategy-dll` 1042, `tactical-dll` 288.
- [x] (2026-03-25 21:21Z) Verified the HD lookup contract is `data/hd/{dll-dir-name}/{resource_id}.png` before fallback to `data/base/ui/{dll-dir-name}/BMP/{resource_id}.bmp`.
- [x] (2026-03-25 21:21Z) Verified `/tmp/common-png` and `/tmp/gokres-png` exist, and discovered `/tmp/strategy-png` and `/tmp/tactical-png` are currently missing.
- [x] (2026-03-25 21:21Z) Verified `upscayl-bin` CLI syntax and confirmed it wants `-m` as the model directory and `-n` as the model name.
- [x] (2026-03-25 21:21Z) Built a complete per-phase routing plan that assigns all 2,231 BMPs to a primary tool and leaves `UltraSharpV2` as the review challenger model.
- [ ] Clear `data/hd/` back to README-only state.
- [ ] Rebuild any missing `/tmp/{dll}-png` source directories.
- [ ] Generate the 10-image review batch and stop for human approval.
- [ ] Run Phase 2 `COMMON.DLL` bulk output.
- [ ] Run Phase 3 `GOKRES.DLL` bulk output.
- [ ] Run Phase 4 `STRATEGY.DLL` bulk output.
- [ ] Run Phase 5 `TACTICAL.DLL` bulk output.
- [ ] Validate file counts and in-game HD fallback behavior.

## Surprises & Discoveries

Observation: `data/hd/README.md` explicitly says the previous waifu2x outputs were cleared and the DLL rerun should use Upscayl plus `PBRify_UpscalerV4`, not the old blanket waifu2x pass.
Evidence: `data/hd/README.md` contains the line `Cleared waifu2x outputs. Redo with Upscayl+PBRify after code wave completes.`

Observation: the existing `scripts/upscale-assets.py` is not the right batch runner for this task.
Evidence: it is EData-specific, writes `data/hd/EData/EDATA_NNN.png`, and only knows the EData naming contract rather than the DLL contract `data/hd/{dll-dir-name}/{resource_id}.png`.

Observation: the two `/tmp` PNG source trees the user said already existed for `STRATEGY` and `TACTICAL` are not present in this workspace right now.
Evidence: `/tmp/common-png` contains 321 PNGs and `/tmp/gokres-png` contains 580 PNGs, but `/tmp/strategy-png` and `/tmp/tactical-png` do not exist.

Observation: the `GOKRES` catalog summary is off by one.
Evidence: `data/base/ui/gokres-dll/BMP/` contains 580 BMPs, but the documented range buckets add up to 579. The unlisted resource is `14151.bmp` at `122x50`, so the implementation plan must not silently drop it.

Observation: `STRATEGY.DLL` contains mixed content inside broad ranges.
Evidence: `10000-10999` is mostly small chrome but also contains a small number of large result and panel assets; `11000-11999` is mostly small state art but also contains large event images such as `11160-11165` and `11554-11559`.

## Decision Log

Decision: keep Phase 0 destructive only inside `data/hd/`, and preserve `data/hd/README.md`.
Rationale: the README is the explicit marker that the previous bad run was intentionally removed. Deleting it would erase useful repository context for later work.
Date/Author: 2026-03-25 / Codex

Decision: treat `PBRify_UpscalerV4` as the default local model for game-texture art, HUD art, chrome, ship diagrams, and tactical sprites.
Rationale: the prior experiments and tools reference both say `PBRify_UpscalerV4` is purpose-built for 2000s-era game textures and is better suited than waifu2x for textured game art.
Date/Author: 2026-03-25 / Codex

Decision: keep `UltraSharpV2` in the review batch only unless the user explicitly promotes it after seeing the comparison.
Rationale: `UltraSharpV2` is a plausible secondary model, but the prior experiments already favor `PBRify_UpscalerV4` as the main production choice.
Date/Author: 2026-03-25 / Codex

Decision: route all face-first portrait assets to Vertex AI Imagen rather than Gemini.
Rationale: the prior experiments showed Gemini modernizes faces and drifts away from the original painted 1998 look, while Vertex preserved portrait fidelity without hallucination.
Date/Author: 2026-03-25 / Codex

Decision: keep Gemini limited to large scene art, backgrounds, and selected result screens rather than all large ranges wholesale.
Rationale: Gemini gives the best detail lift on large structures and scene compositions, but it is the riskiest tool for anything with recognizable faces or repeated small objects.
Date/Author: 2026-03-25 / Codex

Decision: write review outputs directly into the final `data/hd/{dll-dir-name}/{resource_id}.png` locations instead of a separate scratch tree.
Rationale: this lets the first ten approved images count toward final completion, avoids paying twice for Gemini or Vertex on the same IDs, and lets the game test the real fallback path immediately.
Date/Author: 2026-03-25 / Codex

## Outcomes & Retrospective

Planning is complete. The repository now has a production-oriented execution
plan that covers all 2,231 DLL BMPs, not just a curated subset. The plan is
deliberately conservative in two places: it forces a 10-image review gate
before bulk work, and it limits Gemini to the asset families where the prior
experiments clearly justified a generative tool.

The largest remaining execution risk is not code. It is review discipline:
Phase 1 must be treated as a hard stop. If the user dislikes the look of
`PBRify`, `UltraSharp`, Vertex, or Gemini on its assigned sample, that lane
must be rerouted before any bulk step starts.

## Context and Orientation

This plan uses three file-system terms repeatedly:

`resource ID` means the numeric bitmap ID extracted from the original DLL, such
as `10553.bmp` or `2112.bmp`.

`staged BMP` means the original extracted art already copied into the runtime
layout `data/base/ui/{dll-dir-name}/BMP/{resource_id}.bmp` by
`scripts/stage-ui-assets.py`.

`HD override` means the PNG file the renderer checks first at
`data/hd/{dll-dir-name}/{resource_id}.png`. If it exists, the game uses it. If
it does not exist, the game falls back to the staged BMP.

The current repository state matters:

- `crates/rebellion-render/src/bmp_cache.rs` already implements the HD-first
  lookup and uses the exact DLL directory names `common-dll`, `gokres-dll`,
  `strategy-dll`, and `tactical-dll`.
- `scripts/stage-ui-assets.py` already defines the staged BMP source-of-truth at
  `data/base/ui/{dll-dir-name}/BMP/{resource_id}.bmp`.
- `data/hd/README.md` says the old waifu2x outputs were wrong and should be
  replaced.
- The source PNG conversions currently available in `/tmp/` are incomplete in
  this workspace, so the plan includes a recovery command to rebuild them from
  the staged BMP files.

The production routing in this plan assigns every BMP to one of four real
output tools and keeps a fifth tool for review only:

| Tool | Bulk Count | Review Count | Why It Exists |
|------|------------|--------------|---------------|
| `PBRify_UpscalerV4` via Upscayl CLI | 1,648 | 2 | Best default for textured game UI, chrome, HUD, ship sprites, and ship diagrams |
| `waifu2x` | 284 | 2 | Best for clean geometric widgets, tiny indicators, and line-heavy diagram art |
| Vertex AI Imagen 4.0 upscale | 137 | 2 | Best for portraits and portrait-like mini-icons that must not hallucinate |
| Gemini generative image edit | 162 | 2 | Best for large scene art, backgrounds, and selected result screens |
| `UltraSharpV2` via Upscayl CLI | 0 | 2 | Review-only challenger model for a final `PBRify` sanity check |

The bulk counts above sum to 2,231 exactly.

The exact repository-relative files that govern this plan are:

- `agent_docs/assets.md`
- `agent_docs/dll-resource-catalog.md`
- `agent_docs/game-media.md`
- `data/hd/README.md`
- `crates/rebellion-render/src/bmp_cache.rs`
- `scripts/stage-ui-assets.py`
- `scripts/upscale-assets.py`
- `scripts/vertex-upscale.py`

## Plan of Work

Start by deleting only generated HD output directories so the repository
returns to the clean state described by `data/hd/README.md`. Then verify or
rebuild the `/tmp/{dll}-png` input directories because every bulk tool in this
plan expects PNG input, not palette-indexed BMP.

Next, create exactly ten review images: two each for `PBRify`, `UltraSharp`,
waifu2x, Vertex, and Gemini. Those ten files should be written into the final
`data/hd/{dll-dir-name}/{resource_id}.png` paths and visually reviewed before
any bulk pass starts. This review gate is the protection against repeating the
earlier mistake of applying one tool to everything.

After the review gate, run four bulk phases, one DLL at a time. Each phase is
grouped by tool so the commands stay mechanically simple and the output path
contract stays identical across all DLLs. `COMMON.DLL` is the lightest and
cheaply validates the full pipeline. `GOKRES.DLL` is where portrait fidelity
matters and where the catalog bug must be handled. `STRATEGY.DLL` is the most
expensive phase because it holds most of the large scene art. `TACTICAL.DLL`
finishes the combat-facing HUD and sprite work with the same local-model path
used elsewhere.

End by validating two things separately: that the file counts in `data/hd/`
match the plan, and that the game actually prefers those files at runtime by
showing the HD overrides in places already wired to `BmpCache`.

## Concrete Steps

1. Confirm the staged BMP inventory before generating any HD art.

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
for d in data/base/ui/common-dll/BMP data/base/ui/gokres-dll/BMP data/base/ui/strategy-dll/BMP data/base/ui/tactical-dll/BMP; do
  printf "%s: " "$d"
  find "$d" -maxdepth 1 -type f -name '*.bmp' | wc -l
done
```

Expected output: `321`, `580`, `1042`, and `288` in that order.

2. Clear only generated HD outputs and keep the README.

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
find data/hd -mindepth 1 -maxdepth 1 ! -name README.md -exec rm -rf {} +
find data/hd -maxdepth 2 -type f | sort
```

Expected output: only `data/hd/README.md` remains.

3. Rebuild any missing `/tmp/{dll}-png` trees from the staged BMP files.

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
uv run --with pillow python - <<'PY'
from pathlib import Path
from PIL import Image

mapping = {
    "common": Path("data/base/ui/common-dll/BMP"),
    "gokres": Path("data/base/ui/gokres-dll/BMP"),
    "strategy": Path("data/base/ui/strategy-dll/BMP"),
    "tactical": Path("data/base/ui/tactical-dll/BMP"),
}

for name, src in mapping.items():
    dst = Path(f"/tmp/{name}-png")
    dst.mkdir(parents=True, exist_ok=True)
    for bmp in src.glob("*.bmp"):
        out = dst / f"{bmp.stem}.png"
        if out.exists():
            continue
        Image.open(bmp).convert("RGB").save(out, "PNG")
    print(name, len(list(dst.glob("*.png"))))
PY
```

Expected output: `common 321`, `gokres 580`, `strategy 1042`, `tactical 288`.

4. Create the destination folders once so every later command writes directly to
the final HD override layout.

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
mkdir -p data/hd/common-dll data/hd/gokres-dll data/hd/strategy-dll data/hd/tactical-dll
```

Expected output: no error output, and the four DLL directories exist under
`data/hd/`.

5. Generate the 10-image review batch and stop for approval.

Review batch asset list:

- `PBRify` sample 1: `COMMON 11001` button sprite
- `PBRify` sample 2: `TACTICAL 1302` hull and shield panel
- `UltraSharp` sample 1: `GOKRES 1088` ship status view
- `UltraSharp` sample 2: `STRATEGY 10553` panel frame
- `waifu2x` sample 1: `COMMON 10001` widget art
- `waifu2x` sample 2: `STRATEGY 904` small map toggle
- Vertex sample 1: `GOKRES 2112` portrait
- Vertex sample 2: `GOKRES 19008` character mini-icon
- Gemini sample 1: `STRATEGY 6208` event scene
- Gemini sample 2: `COMMON 20001` main menu background

`PBRify` review commands:

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
/Applications/Upscayl.app/Contents/Resources/bin/upscayl-bin -i /tmp/common-png/11001.png -o data/hd/common-dll/11001.png -m ~/tools/upscale-models -n 4x-PBRify_UpscalerV4 -s 4 -f png
/Applications/Upscayl.app/Contents/Resources/bin/upscayl-bin -i /tmp/tactical-png/1302.png -o data/hd/tactical-dll/1302.png -m ~/tools/upscale-models -n 4x-PBRify_UpscalerV4 -s 4 -f png
```

`UltraSharpV2` review commands:

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
/Applications/Upscayl.app/Contents/Resources/bin/upscayl-bin -i /tmp/gokres-png/1088.png -o data/hd/gokres-dll/1088.png -m ~/tools/upscale-models -n 4x-UltraSharpV2 -s 4 -f png
/Applications/Upscayl.app/Contents/Resources/bin/upscayl-bin -i /tmp/strategy-png/10553.png -o data/hd/strategy-dll/10553.png -m ~/tools/upscale-models -n 4x-UltraSharpV2 -s 4 -f png
```

waifu2x review commands:

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
~/.local/bin/waifu2x -i /tmp/common-png/10001.png -o data/hd/common-dll/10001.png -n 1 -s 4 -f png
~/.local/bin/waifu2x -i /tmp/strategy-png/904.png -o data/hd/strategy-dll/904.png -n 1 -s 4 -f png
```

Vertex review commands:

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
GOOGLE_CLOUD_PROJECT=dream-daimon uv run scripts/vertex-upscale.py --input /tmp/gokres-png/2112.png --output data/hd/gokres-dll/2112.png --factor 4
GOOGLE_CLOUD_PROJECT=dream-daimon uv run scripts/vertex-upscale.py --input /tmp/gokres-png/19008.png --output data/hd/gokres-dll/19008.png --factor 4
```

Gemini review commands:

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
python3 ~/.claude/skills/nano-banana-pro/scripts/edit_image.py \
  "Enhance this image to high resolution with sharp details and crisp edges. Preserve all elements exactly. Maintain the original 1998 game-art style. Do not add any new objects, ships, faces, or interface elements." \
  /tmp/strategy-png/6208.png \
  --aspect-ratio 16:9 \
  --temperature 0.3 \
  --output /tmp/gemini-review/strategy-6208 \
  --filename 6208
uv run --with pillow python - <<'PY'
from pathlib import Path
from PIL import Image
src = next(Path('/tmp/gemini-review/strategy-6208').glob('6208_image_0_0.*'))
Image.open(src).save('data/hd/strategy-dll/6208.png', 'PNG')
PY

python3 ~/.claude/skills/nano-banana-pro/scripts/edit_image.py \
  "Enhance this image to high resolution with sharp details and crisp edges. Preserve all elements exactly. Maintain the original 1998 game-art style. Do not add any new objects, ships, faces, or interface elements." \
  /tmp/common-png/20001.png \
  --aspect-ratio 4:3 \
  --temperature 0.3 \
  --output /tmp/gemini-review/common-20001 \
  --filename 20001
uv run --with pillow python - <<'PY'
from pathlib import Path
from PIL import Image
src = next(Path('/tmp/gemini-review/common-20001').glob('20001_image_0_0.*'))
Image.open(src).save('data/hd/common-dll/20001.png', 'PNG')
PY
```

Expected output: 10 PNGs exist under `data/hd/` in the final DLL folders. Stop
here and visually review them before bulk work.

6. Phase 2: bulk `COMMON.DLL` with per-range routing.

Phase 2 routing:

| Resource IDs | Count | Tool | Why |
|--------------|-------|------|-----|
| `10000-10999` present IDs | 101 | waifu2x | Scrollbars, checkboxes, sliders, and small core widgets are mostly clean geometric UI |
| `11000-11999` present IDs | 215 | `PBRify` | Button sprites have beveled game-texture shading and benefit from texture-oriented ESRGAN |
| `15000-15999` present IDs | 3 | waifu2x | Tiny miscellaneous icons are closer to geometric icon art than textured scenes |
| `20001-20002` | 2 | Gemini | Main-menu backgrounds benefit from scene-detail generation |

Bulk commands:

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
uv run --with pillow python - <<'PY'
from pathlib import Path
import subprocess

waifu_ids = {int(p.stem) for p in Path('/tmp/common-png').glob('*.png') if 10000 <= int(p.stem) <= 10999}
waifu_ids |= {15856, 15922, 15990}
for rid in sorted(waifu_ids):
    subprocess.run([
        str(Path.home() / '.local/bin/waifu2x'),
        '-i', f'/tmp/common-png/{rid}.png',
        '-o', f'data/hd/common-dll/{rid}.png',
        '-n', '1', '-s', '4', '-f', 'png',
    ], check=True)
PY

uv run --with pillow python - <<'PY'
from pathlib import Path
import subprocess
for png in sorted(Path('/tmp/common-png').glob('*.png')):
    rid = int(png.stem)
    if 11000 <= rid <= 11999:
        subprocess.run([
            '/Applications/Upscayl.app/Contents/Resources/bin/upscayl-bin',
            '-i', str(png),
            '-o', f'data/hd/common-dll/{rid}.png',
            '-m', str(Path.home() / 'tools/upscale-models'),
            '-n', '4x-PBRify_UpscalerV4',
            '-s', '4',
            '-f', 'png',
        ], check=True)
PY

for rid in 20001 20002; do
  python3 ~/.claude/skills/nano-banana-pro/scripts/edit_image.py \
    "Enhance this image to high resolution with sharp details and crisp edges. Preserve all elements exactly. Maintain the original 1998 game-art style. Do not add any new objects, ships, faces, or interface elements." \
    "/tmp/common-png/${rid}.png" \
    --aspect-ratio 4:3 \
    --temperature 0.3 \
    --output "/tmp/gemini-common-${rid}" \
    --filename "${rid}"
  uv run --with pillow python - <<PY
from pathlib import Path
from PIL import Image
src = next(Path('/tmp/gemini-common-${rid}').glob('${rid}_image_0_0.*'))
Image.open(src).save('data/hd/common-dll/${rid}.png', 'PNG')
PY
done
```

Expected output: 321 PNGs under `data/hd/common-dll/`.
Estimated time and cost: about 15 to 30 minutes locally for waifu2x plus
`PBRify`, plus 2 Gemini calls. Gemini cost for this phase is about
`$0.26-$0.48`.

7. Phase 3: bulk `GOKRES.DLL` with portrait-safe routing.

Phase 3 routing:

| Resource IDs | Count | Tool | Why |
|--------------|-------|------|-----|
| `2000-2999` | 61 | Vertex | These are the `80x80` painted portraits |
| `19000-19999` | 76 | Vertex | These are character mini-icons and still portrait-like |
| `5000-6999` | 42 | waifu2x | Reactor diagrams are line-heavy and closer to clean technical art |
| `21000-26999` | 102 | waifu2x | Weapon arc and damage-detail icons are diagram-style assets |
| `1-999`, `1000-1999`, `3000-4999`, `7000-10999`, `14151`, `16000-18999`, `27000+` | 299 | `PBRify` | Facility views, ship views, ship construction views, damage views, entity mini-icons, and the unlisted `14151.bmp` are textured game art |

Bulk commands:

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
uv run --with pillow python - <<'PY'
from pathlib import Path
import subprocess

for png in sorted(Path('/tmp/gokres-png').glob('*.png')):
    rid = int(png.stem)
    if 2000 <= rid <= 2999 or 19000 <= rid <= 19999:
        subprocess.run([
            'uv', 'run', 'scripts/vertex-upscale.py',
            '--input', str(png),
            '--output', f'data/hd/gokres-dll/{rid}.png',
            '--factor', '4',
        ], check=True, env={**__import__('os').environ, 'GOOGLE_CLOUD_PROJECT': 'dream-daimon'})
PY

uv run --with pillow python - <<'PY'
from pathlib import Path
import subprocess
for png in sorted(Path('/tmp/gokres-png').glob('*.png')):
    rid = int(png.stem)
    if 5000 <= rid <= 6999 or 21000 <= rid <= 26999:
        subprocess.run([
            str(Path.home() / '.local/bin/waifu2x'),
            '-i', str(png),
            '-o', f'data/hd/gokres-dll/{rid}.png',
            '-n', '1', '-s', '4', '-f', 'png',
        ], check=True)
PY

uv run --with pillow python - <<'PY'
from pathlib import Path
import subprocess
for png in sorted(Path('/tmp/gokres-png').glob('*.png')):
    rid = int(png.stem)
    if (
        1 <= rid <= 999 or
        1000 <= rid <= 1999 or
        3000 <= rid <= 4999 or
        7000 <= rid <= 10999 or
        rid == 14151 or
        16000 <= rid <= 18999 or
        rid >= 27000
    ):
        subprocess.run([
            '/Applications/Upscayl.app/Contents/Resources/bin/upscayl-bin',
            '-i', str(png),
            '-o', f'data/hd/gokres-dll/{rid}.png',
            '-m', str(Path.home() / 'tools/upscale-models'),
            '-n', '4x-PBRify_UpscalerV4',
            '-s', '4',
            '-f', 'png',
        ], check=True)
PY
```

Expected output: 580 PNGs under `data/hd/gokres-dll/`.
Estimated time and cost: local work roughly 30 to 60 minutes, plus Vertex rate
limit time. Vertex handles 137 billable images in this phase, so the hard floor
at `5 req/min` is about 28 minutes. Using the current Google pricing page at
`$0.06` per upscale, this phase costs about `$8.22`.

8. Phase 4: bulk `STRATEGY.DLL`, which is the expensive mixed-content phase.

Phase 4 routing:

| Resource IDs | Count | Tool | Why |
|--------------|-------|------|-----|
| `900-903` | 4 | Gemini | Large map backgrounds and overlays |
| `1000-1999` present IDs | 73 | Gemini | `400x200` character panels are large scene art and benefit from detail lift |
| `6000-6999` present IDs | 61 | Gemini | Event scenes and briefings are large narrative compositions |
| `10757-10763`, `11125-11126`, `11160-11165`, `11554-11559`, `11619` | 22 | Gemini | Large result and event screens hidden inside otherwise mixed UI ranges |
| `904-906` | 3 | waifu2x | Tiny display toggles are clean geometric UI |
| `9000-9999` | 33 | waifu2x | Facility and system indicators are small simple icons |
| Everything else in `STRATEGY.DLL` | 846 | `PBRify` | The remaining assets are overwhelmingly panel chrome, buttons, frames, and other textured strategy-layer UI |

Bulk commands:

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
uv run --with pillow python - <<'PY'
from pathlib import Path
import subprocess
gemini_ids = set(range(900, 904))
gemini_ids |= {int(p.stem) for p in Path('/tmp/strategy-png').glob('*.png') if 1000 <= int(p.stem) <= 1999}
gemini_ids |= {int(p.stem) for p in Path('/tmp/strategy-png').glob('*.png') if 6000 <= int(p.stem) <= 6999}
gemini_ids |= set(range(10757, 10764))
gemini_ids |= set(range(11125, 11127))
gemini_ids |= set(range(11160, 11166))
gemini_ids |= set(range(11554, 11560))
gemini_ids.add(11619)
print('\n'.join(map(str, sorted(gemini_ids))))
PY > /tmp/strategy-gemini.ids

while read -r rid; do
  python3 ~/.claude/skills/nano-banana-pro/scripts/edit_image.py \
    "Enhance this image to high resolution with sharp details and crisp edges. Preserve all elements exactly. Maintain the original 1998 game-art style. Do not add any new objects, ships, faces, or interface elements." \
    "/tmp/strategy-png/${rid}.png" \
    --aspect-ratio 16:9 \
    --temperature 0.3 \
    --output "/tmp/gemini-strategy-${rid}" \
    --filename "${rid}"
  uv run --with pillow python - <<PY
from pathlib import Path
from PIL import Image
src = next(Path('/tmp/gemini-strategy-${rid}').glob('${rid}_image_0_0.*'))
Image.open(src).save('data/hd/strategy-dll/${rid}.png', 'PNG')
PY
done < /tmp/strategy-gemini.ids

uv run --with pillow python - <<'PY'
from pathlib import Path
import subprocess
for rid in [904, 905, 906] + list(range(9001, 9034)):
    subprocess.run([
        str(Path.home() / '.local/bin/waifu2x'),
        '-i', f'/tmp/strategy-png/{rid}.png',
        '-o', f'data/hd/strategy-dll/{rid}.png',
        '-n', '1', '-s', '4', '-f', 'png',
    ], check=True)
PY

uv run --with pillow python - <<'PY'
from pathlib import Path
import subprocess
gemini_ids = {int(line.strip()) for line in Path('/tmp/strategy-gemini.ids').read_text().splitlines() if line.strip()}
for png in sorted(Path('/tmp/strategy-png').glob('*.png')):
    rid = int(png.stem)
    if rid in gemini_ids or rid in {904, 905, 906} or 9000 <= rid <= 9999:
        continue
    subprocess.run([
        '/Applications/Upscayl.app/Contents/Resources/bin/upscayl-bin',
        '-i', str(png),
        '-o', f'data/hd/strategy-dll/{rid}.png',
        '-m', str(Path.home() / 'tools/upscale-models'),
        '-n', '4x-PBRify_UpscalerV4',
        '-s', '4',
        '-f', 'png',
    ], check=True)
PY
```

Expected output: 1,042 PNGs under `data/hd/strategy-dll/`.
Estimated time and cost: this is the longest phase. Local `PBRify` plus waifu2x
work is likely 60 to 120 minutes. Gemini handles 160 images here, so budget
roughly `$20.80-$38.40` using the prior measured Gemini cost range
`$0.13-$0.24/image`. If review shows face drift is still unacceptable on the
scene-heavy character panels, reroute `1000-1999` to Vertex or `PBRify` before
starting this phase.

9. Phase 5: bulk `TACTICAL.DLL`.

Phase 5 routing:

| Resource IDs | Count | Tool | Why |
|--------------|-------|------|-----|
| `1000-1999` present IDs | 154 | `PBRify` | Combat HUD, gauges, and panel art are textured game UI |
| `2000-2999` present IDs | 130 | `PBRify` | Tactical ship sprites are pre-rendered game art and need consistency across sets |
| `40000+` present IDs | 4 | `PBRify` | Small tactical labels belong with the same combat-facing UI lane |

Bulk command:

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
uv run --with pillow python - <<'PY'
from pathlib import Path
import subprocess
for png in sorted(Path('/tmp/tactical-png').glob('*.png')):
    rid = int(png.stem)
    subprocess.run([
        '/Applications/Upscayl.app/Contents/Resources/bin/upscayl-bin',
        '-i', str(png),
        '-o', f'data/hd/tactical-dll/{rid}.png',
        '-m', str(Path.home() / 'tools/upscale-models'),
        '-n', '4x-PBRify_UpscalerV4',
        '-s', '4',
        '-f', 'png',
    ], check=True)
PY
```

Expected output: 288 PNGs under `data/hd/tactical-dll/`.
Estimated time and cost: roughly 20 to 45 minutes locally, no API cost.

10. Validate the final file counts and exact folder names.

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
for d in data/hd/common-dll data/hd/gokres-dll data/hd/strategy-dll data/hd/tactical-dll; do
  printf "%s: " "$d"
  find "$d" -maxdepth 1 -type f -name '*.png' | wc -l
done
```

Expected output after all phases: `321`, `580`, `1042`, and `288`.

11. Validate the runtime fallback path in the game.

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
PATH=/usr/bin:$PATH cargo run -p rebellion-app --bin open-rebellion -- data/base
```

Expected output: the game launches normally, the main menu uses
`data/hd/common-dll/20001.png` and `data/hd/common-dll/20002.png` when present,
and any ungenerated DLL asset still renders because `BmpCache` falls back to the
staged BMP path.

## Validation and Acceptance

Phase 0 is accepted when `data/hd/` contains only `README.md`.

Phase 1 is accepted only after a human reviews all 10 sample outputs and gives
an explicit keep-or-reroute decision for each lane:

- Keep or replace `PBRify` for textured UI art
- Keep or replace `UltraSharpV2` as the challenger model
- Keep or replace waifu2x for geometric widgets and diagram art
- Keep or replace Vertex for portraits and portrait-like mini-icons
- Keep or replace Gemini for large scene art

Phase 2 is accepted when `data/hd/common-dll/` contains 321 PNGs and the main
menu backgrounds `20001.png` and `20002.png` visibly render through the HD
override path.

Phase 3 is accepted when `data/hd/gokres-dll/` contains 580 PNGs, portraits
still look like the original painted art instead of modernized faces, and the
stray `14151.png` exists so the off-by-one catalog bug did not cause a dropped
asset.

Phase 4 is accepted when `data/hd/strategy-dll/` contains 1,042 PNGs and a
manual spot check on `900.png`, `1000.png`, `6208.png`, `10553.png`,
`10757.png`, and `11160.png` shows the intended tool effect without obvious
hallucinated content.

Phase 5 is accepted when `data/hd/tactical-dll/` contains 288 PNGs and the
combat HUD plus ship sprites still align and read correctly at runtime.

The overall plan is accepted when all four DLL directories hit the expected
counts, the main menu and at least one strategy screen use the HD overrides,
and deleting one generated PNG proves fallback still works by cleanly reverting
that single asset to the original BMP at runtime.

## Idempotence and Recovery

Phase 0 is intentionally destructive, but only inside generated output under
`data/hd/`. It is safe to repeat because the destination files are derived
artifacts, not source material.

The `/tmp/{dll}-png` rebuild command is idempotent. It skips PNGs that already
exist and can be rerun whenever a `/tmp` directory is missing or partially
deleted.

All bulk phases are safe to rerun if a tool crashes midway. The output contract
is one file per resource ID. Re-running a phase simply overwrites the same
target PNGs in place.

If a specific lane looks wrong after review, recover by deleting only that
lane's output directory or explicit file IDs, then rerun the corrected command.
For example, if `GOKRES` portraits need to move from Vertex to `PBRify`, delete
only `data/hd/gokres-dll/{portrait ids}.png` and rerun that subset with the new
tool. Do not clear unrelated DLL output.

If the Gemini script returns JPEG or WEBP, the plan already normalizes it back
to PNG with Pillow before writing to `data/hd/`. That post-processing step is
required because `BmpCache` only checks the `.png` filename.

## Interfaces and Dependencies

The output interface is fixed and already implemented in
`crates/rebellion-render/src/bmp_cache.rs`:

- Input fallback path: `data/base/ui/{dll-dir-name}/BMP/{resource_id}.bmp`
- HD override path: `data/hd/{dll-dir-name}/{resource_id}.png`
- DLL directory names: `common-dll`, `gokres-dll`, `strategy-dll`, `tactical-dll`

The required local tools and scripts are:

- `~/.local/bin/waifu2x`
- `/Applications/Upscayl.app/Contents/Resources/bin/upscayl-bin`
- `~/tools/upscale-models/4x-PBRify_UpscalerV4.pth`
- `~/tools/upscale-models/4x-UltraSharpV2.pth`
- `scripts/vertex-upscale.py`
- `~/.claude/skills/nano-banana-pro/scripts/edit_image.py`
- `uv run --with pillow python` for PNG conversion and Gemini output normalization

The required cloud dependencies are:

- Vertex AI project `dream-daimon`
- Vertex AI rate limit currently `5 req/min`
- Gemini API credentials for the local edit script

The final production routing in this plan is:

- `COMMON.DLL`: waifu2x 104, `PBRify` 215, Gemini 2
- `GOKRES.DLL`: Vertex 137, waifu2x 144, `PBRify` 299
- `STRATEGY.DLL`: Gemini 160, waifu2x 36, `PBRify` 846
- `TACTICAL.DLL`: `PBRify` 288

Those four lines sum to all 2,231 BMPs and match the renderer's existing HD
override contract exactly.
