# Sprite tracks

Three deterministic sprite tracks under `scripts/sprites/`, driven either directly or through the
sprite-forge profile `scripts/sprites/rebellion.pipeline.json`. None of them calls a paid provider,
none of them changes a Rust crate, and none of them is a parity deliverable: the 1998 tactical
view is Direct3D Retained Mode geometry, and generated sprites can never close a parity cell.

| Track | Script | Input | Output | Classification |
|---|---|---|---|---|
| A. Original-mesh direction sheets | `render_original_meshes.py` | 87 staged type-301 meshes + 397 textures/palettes (`data/base/ui/tactical-dll/TACTICAL3D/runtime/`) | `data/models/sprites/{id}.png` + `.json` (sprite-forge atlas-v2, 8 directions), `provenance.json`, `review/` | experimental-remaster review artifact, `runtime_eligible: false` |
| B. Advisor family transform | `faithful_hd_pipeline.py --kind advisor` (+ `type302.py`) | 68 authored type-302 advisor frames over four anchors | `data/hd/{alsprite,emsprite}-dll/{id}.png` records in `data/hd/manifest.json` | faithful-hd candidate, offline half of P45C; not consumed by the runtime yet |
| C. Native fighter textures | `export_fighter_textures.py` | 77 type-303 textures 4000-4134 | `assets/references/ref-fighter-textures/` (mode P + `palette.json` + `INDEX.md`) | reference (palette-preserving) |

Shared reader: `scripts/sprites/tactical3d.py` decodes `ORTMESH\0`, `ORTINDEX` and `ORTPAL00`
objects following the Go writer (`tools/stage-ui-assets/tactical3d_runtime.go`) and applies the
Rust handedness (negate z, swap triangle indices 1 and 2). Its self-test decodes every manifest
object: `uv run scripts/sprites/tactical3d.py`. Textures resolve by name for mesh materials and by
id for fighters and palettes; rule 2 `planet_pair` textures bind palette `id + 30` from their
header, rule 1 `battle_active` textures take the selected battle palette (5531 by default,
5531-5557 per system).

## Running

```bash
# whole profile (deterministic, free; the tracks are independent stages)
python3 ~/.claude/skills/sprite-forge/scripts/run_pipeline.py scripts/sprites/rebellion.pipeline.json --dry-run
python3 ~/.claude/skills/sprite-forge/scripts/run_pipeline.py scripts/sprites/rebellion.pipeline.json --stage render-meshes

# Track A directly: proof family first, then everything
uv run scripts/sprites/render_original_meshes.py --ids 2560 2561 2562
uv run scripts/sprites/render_original_meshes.py --all --cell 128
uv run scripts/sprites/render_original_meshes.py --ids 2560 --unlit --check-palette   # exactness gate

# Track B
uv run scripts/faithful_hd_pipeline.py generate --kind advisor
uv run scripts/faithful_hd_pipeline.py verify --kind advisor
python3 scripts/sprites/type302.py export --dll alsprite-dll --family 2001 --out data/hd/review/type302/alsprite-2001/source
python3 ~/.claude/skills/sprite-forge/scripts/pixel_scale.py --mode scale2x --family data/hd/review/type302/alsprite-2001/source --output-dir data/hd/review/type302/alsprite-2001/scale2x

# Track C
uv run scripts/sprites/export_fighter_textures.py --palette 5531
```

`uv run --with pillow` when the ambient `python3` lacks Pillow. Blender is spawned by the Track A
script itself (`BLENDER` env, default `/opt/homebrew/bin/blender`, tested on 5.0.1); the
sprite-forge location comes from `SPRITE_FORGE_DIR` or `--sprite-forge`.

## Track A: what is authentic and what is not

The rig reproduces the recovered constants in `crates/rebellion-render/src/tactical_assets.rs`,
with the evidence files named in `provenance.json`:

- Camera (P57B2A): pitch 30, Alliance yaw -30 (`--faction empire` gives 150), field 0.2 so
  fovy = 2·atan(0.2), near 1.0, far = distance × 2.5, position and up vector from
  `OriginalTacticalCamera::pose`.
- Light (P57B2C2A): directional RGB 0.8 along surface-to-light (5,5,1) normalised, ambient 0.5,
  no specular, Gouraud. The vertex term `min(1, diffuse·min(1, 0.5 + 0.8·max(0, n·L)) + emissive)`
  is computed in Python per vertex and baked into a colour attribute; Blender multiplies it by the
  texel and rasterises with one sample, pixel filter 0, view transform Raw, textures Non-Color,
  nearest sampling, back-face culling. `--unlit --check-palette` proves the pass-through: every
  opaque pixel must be a palette colour (87/87 meshes pass, 0 anti-aliased alpha values).
- Winding: the decoder's `(i0, i2, i1)` order is CCW-front in the reflected space; the script
  counts triangles whose geometric normal agrees with the source normals and fails a sheet when
  the disagreements win (87/87 agree completely).

Non-authentic by necessity, and recorded as such: the camera distance. The original frames the
whole battle at `extent × 1.7`; a sheet frames one model, so each LOD family gets one shared fit
distance (`family_radius / tan(fovy/2) × 1.15`) so close, medium and far share scale.
`--elevation`, `--yaw`, `--projection ortho` and `--unlit` flip `rig` to `override` in provenance.
Direction k rotates the ship k·45° about +Y (the meshes are Y-up); labels follow sprite-forge's
S, SE, E, NE, N, NW, W, SW order and `headings` records the degrees.

Review sheets: `data/models/sprites/review/{family}.png` show every LOD on black, white, palette
index 0 and a checkerboard; `review/index.html` embeds them. Everything under `data/models/` is
gitignored (derived from original game data).

Runtime consumption (an atlas loader or billboard path) would be a crate architecture change and
needs explicit approval; it is not part of this track.

## Track B: advisor family transform

`scripts/sprites/type302.py` ports `decode_type302_frame` (17-byte header, per-row offsets,
alternating skip/literal runs, wrapping deltas over the anchor indices, index 0 transparent) and
the runtime's cumulative chaining: authored families decode each frame over the previous frame's
indices, so a family is a chain, not a set. Digests matched the Rust decoder 3,988/3,988 (pure) and
68/68 (chained) through a throwaway `cargo test` that was reverted; only the Python is committed.

Authored families (`alsprite-dll` 2001→2002..2024 and 3331→3332..3346; `emsprite-dll`
2001→2002..2016 and 3001→3002..3016) become `TYPE302` records with `route.family_transform =
deterministic-family-transform`, chain provenance (anchor and chain sha256), and the P45C gates:
`frame_index`/`frame_count`, `frame_order = authored-ascending`, `cadence_seconds = 0.15`,
`static_region_shimmer = 0`, `family_atomic`. `verify` fails a whole family when one frame fails.
Tier 1 (nearest-4x) is what the records carry; the Scale2x tier-2 candidate is produced for review
only through `type302.py export` and sprite-forge `pixel_scale.py --family`, never written into
the manifest, so `verify_record` stays exact.

The Rust reader `approved_hd_assets_from_bytes` accepts the extra fields (checked with a scratch
approved copy), but `BmpCache` and `AdvisorState` do not consume advisor overrides yet; the
render-side family fallback is a later, approval-gated Rust task.

## Track C: fighter textures

`export_fighter_textures.py` asserts 77 files (41 at 32x32, 36 at 16x16), all rule 1, writes mode-P
PNGs on the chosen palette plus `palette.json` (palette RGB and sha256, per-file object/source/
indices/PNG digests) and `INDEX.md`. Family slugs come from `tactical-lookup.json` `fighters`
ranges (`base..base+9`); 4044 and 4049 fall outside every range and are `unknown`.
`assets/references/ref-squadron-sprites/` (14 files) is MetasharpNet `Names303` 128x128 upscales
with the palette discarded; it is annotated as non-native in `assets/references/INDEX.md` and
must not be used as a fidelity reference.

## Related

- `agent_docs/assets.md` for the faithful-HD contract and the tactical 3D staging.
- `docs/reference/asset-library/space-battle.md` for the graphic families and the P54-P58 evidence.
- sprite-forge (`~/.claude/skills/sprite-forge/`, `references/pipelines/open-rebellion.md`) for the
  atlas-v2 manifest, `pixel_scale.py`, `stitch_spritesheet.py` and the profile runner.
