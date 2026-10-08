---
title: "Galaxy View Framing — Backdrop, Coordinate Transform, Galaxy Size, GID Caption"
description: "Traced original layout for backdrop bitmap, system placement formula, galaxy size filtering, GID caption, and legend"
category: "ghidra"
created: 2026-10-08
updated: 2026-10-08
---

# Galaxy View Framing

Traced from FUN_00422ce0 (galaxy view window proc), FUN_00427010 (backdrop
setup), FUN_00425d00 (coordinate transform), FUN_0042d230 (GID overlay), and
SECTORSD.json / SYSTEMSD.json data analysis.

---

## 1. Backdrop Bitmap

**Source**: FUN_00427010 (line 1–51 of decompiled body)

| Resource | Description | Size | Source |
|----------|-------------|------|--------|
| 902 | Bright starfield (Alliance default) | 607 × 437 | STRATEGY.DLL |
| 903 | Dim starfield (Empire default) | 607 × 437 | STRATEGY.DLL |
| 900 | Alliance cockpit shell frame | full shell | STRATEGY.DLL |
| 901 | Empire cockpit shell frame | full shell | STRATEGY.DLL |

The starfield is blitted at a faction-specific offset inside the shell frame:

| Faction | Starfield Offset (x, y) | Shell Overlay |
|---------|-------------------------|---------------|
| Alliance | **(21, 25)** | 900 at (0, 0) |
| Empire | **(84, 27)** | 901 at (0, 0) |

No stretch, no scale. The 607 × 437 starfield is placed 1:1, then the shell
frame (with transparent cutout for the galaxy aperture) overlays it.

The starfield is not clipped to the galaxy view's client rect: it shows
through every transparent part of the shell, beside the droids and between
the consoles (Wine captures, 2026-10-08).

**Current port**: the aperture rects Alliance (55, 40, 485, 350) and
Empire (120, 40, 480, 355) are the galaxy view's client rects
(`FUN_00421c70`) and bound input and the map's own layers.
`draw_galaxy_backdrop` paints the whole 607 × 437 starfield at its offset
before `main.rs` sets the aperture clip, so the shell's cut-outs show it as
the original's do (corrected 2026-10-08; it had been clipped to the
aperture, leaving black gaps).

---

## 2. Coordinate Transform

**Source**: FUN_00425d00, assembly 0x00425f92–0x00426000

```
x_screen = (int)(coord_x * 607.0 / 1024.0) + backdrop_x_offset
y_screen = (int)(coord_y * 437.0 / 1024.0) + backdrop_y_offset
```

Where:
- `coord_x` = low 16 bits of packed u32 at `system->+0x2c+0x4c`
- `coord_y` = high 16 bits of the same packed u32
- `607.0` = DAT_00658be0 (backdrop width)
- `437.0` = DAT_00658be8 (backdrop height)
- `1024` = DAT_00658bd8 + 1 = DAT_00658bda + 1 (galaxy width/height = 1023, domain 0–1023)

| Faction | backdrop_x_offset | backdrop_y_offset |
|---------|------------------|------------------|
| Alliance | 21 | 25 |
| Empire | 84 | 27 |

The transform is a direct linear mapping from the 0–1023 galaxy coordinate
space into the 607 × 437 pixel starfield, offset by the shell's galaxy
aperture position. No camera, no zoom, no centering.

**Current port** (lib.rs:176–182, 219, 223): uses a camera-based projection
centred on `GALAXY_CAMERA_CENTER = (450.0, 470.0)` (marked "hyp: not traced"):
```rust
let sx = (dat_x - cam_x) * zoom + viewport_x + viewport_width / 2.0;
let sy = (dat_y - cam_y) * zoom + viewport_y + viewport_height / 2.0;
```
This is structurally wrong: the original has no camera centre, no zoom, and
no centring offset. The correct transform is the linear formula above.

### Verification

DAT coordinate range (all 200 systems):
- X: 118–791 → screen: 118 × 607 / 1024 + 21 = **91** to 791 × 607 / 1024 + 21 = **490**
- Y: 103–839 → screen: 103 × 437 / 1024 + 25 = **69** to 839 × 437 / 1024 + 25 = **383**

All within the 607 × 437 starfield. The backdrop is never cropped if the
formula is correct.

---

## 3. Galaxy Size Filtering

**Source**: SECTORSD.DAT `galaxy_size` field (byte offset 28, u32), confirmed
from `data/base/json/SECTORSD.json` and `tools/dat-dumper/src/types/sectors.rs:78`.

### Data

| Galaxy Size Value | Sectors | Systems | Cumulative Sectors | Cumulative Systems |
|-------------------|---------|---------|--------------------|--------------------|
| 1 (Small) | 10 | 100 | 10 | 100 |
| 2 (Medium) | 5 | 50 | 15 | 150 |
| 3 (Large) | 5 | 50 | 20 | 200 |

Each sector has exactly 10 systems. A Small galaxy game includes only sectors
with `galaxy_size == 1`. A Medium game adds `galaxy_size == 2`. A Large game
uses all 20 sectors.

### Filtering Rule

```
include_sector = sector.galaxy_size <= chosen_galaxy_size
```

Where `chosen_galaxy_size`:
- Small / Standard → 1
- Medium / Large → 2
- Large / Huge → 3

(Our `GalaxySize` enum maps Standard=1, Large=2, Huge=3 — values match the
DAT field directly.)

### Current Bug

**`crates/rebellion-data/src/lib.rs:169`**: the sector loading loop iterates
over ALL `sectors_file.sectors` with no galaxy_size filter:

```rust
for dat in &sectors_file.sectors {
    // ... creates Sector for every record
    let key = world.sectors.insert(sector);
    sector_key_map.insert(dat.id, key);
}
```

Similarly, the system loop at line 195 creates systems for every sector. The
result is that a "Small" galaxy loads all 200 systems across 20 sectors,
making systems spread across the entire 607 × 437 backdrop with no clustering.
Tom's observation: "10 tight clusters of ~8–10 systems each" in the original
Small game vs "all 200 systems scattered" in the port.

The `galaxy_size` field IS parsed by the DAT dumper
(`tools/dat-dumper/src/types/sectors.rs:78`) but never read by the seeding
code. `GalaxySize` is only used for the maintenance budget parameter
(`seeds.rs:1850–1880`).

### Fix

In `create_world_from_dat` (lib.rs:169), filter:

```rust
let chosen_size = seed_options.galaxy_size as u32;
for dat in &sectors_file.sectors {
    if dat.galaxy_size > chosen_size {
        continue;  // skip sectors not in this galaxy tier
    }
    // ... rest of sector creation
}
```

Systems inherit their sector's inclusion — a system referencing a missing
sector already fails with `sector_key_map.get()`, so the system loop
auto-filters once the sector loop is fixed.

---

## 4. GID Caption

**Source**: FUN_00422ce0 WM_CREATE (lines 186–234) and FUN_0042d230 (GID
overlay paint).

### Text Label (at galaxy view object +0xe0, which is param_1+0x38)

| Property | Alliance | Empire | Source |
|----------|----------|--------|--------|
| Position (x, y) | (104, 18) | (500, 18) | FUN_00601b30 call at line 188/208 |
| Color | 0x20000ff (blue) | 0x200ff00 (green) | FUN_00601c90 call at line 230 |
| Font index | 10 | 10 | FUN_00601c60 call at line 234 |
| Font face | Arial | Arial | FUN_005ff830 → "Arial" |
| Font height | 14 px | 14 px | FUN_0060eed0 case 10 |
| Font weight | 400 (FW_NORMAL) | 400 (FW_NORMAL) | FUN_0060eed0 case 10 |
| DrawText format | 1 (DT_CENTER) | 1 (DT_CENTER) | FUN_00403e90 call at line 232 |

### GID Overlay Bitmap (per mode)

From FUN_0042d230: each GID mode loads a STRATEGY.DLL bitmap resource:

| GID Mode | Alliance Resource | Empire Resource |
|----------|-------------------|-----------------|
| 0 (Popular Support) | 0x2d3c (11580) | 0x2d40 (11584) |
| 1 | 0x2d3d (11581) | 0x2d41 (11585) |
| 2 | 0x2d3e (11582) | 0x2d42 (11586) |
| 3 | 0x2d3f (11583) | 0x2d43 (11587) |

Overlay position: Alliance (167, 20), Empire (561, 20).

### Current Port

`crates/rebellion-render/src/lib.rs:439` (`draw_gid_caption`): uses 11 px
macroquad `draw_text`. The original uses Arial 14 px — the port text is too
small and uses the wrong font metrics.

---

## 5. Font Table

**Source**: FUN_0060eed0 (font creation), FUN_005ff830 (init with "Arial"),
FUN_0060ee20 (font table constructor at DAT_006be4f8).

All 14 font entries use the face name **"Arial"**:

| Index | lfHeight | lfWeight | Used For |
|-------|----------|----------|----------|
| 0 (default) | 16 | 400 (Normal) | |
| 1 | 16 | 700 (Bold) | |
| 2 | 14 | 700 (Bold) | |
| 3 | 8 | 400 (Normal) | |
| 4 | 16 | 400 (Normal) | (same as 0) |
| 5 | 16 | 700 (Bold) | (same as 1) |
| 6 | 18 | 400 (Normal) | cockpit labels |
| 7 | 18 | 700 (Bold) | cockpit labels bold |
| 8 | 12 | 400 (Normal) | |
| 9 | 12 | 400 (Normal) | (same as 8) |
| **10** | **14** | **400 (Normal)** | **GID caption** |
| 11 | 24 | 400 (Normal) | |
| 12 | 30 | 400 (Normal) | |
| 13 | 12 | 900 (Heavy) | |

lfQuality = 2 (PROOF_QUALITY) for all entries.
lfCharSet, lfOutPrecision, lfClipPrecision = 0 for all.

---

## 6. Small vs Large Galaxy

Both galaxy sizes use the **same** 607 × 437 starfield backdrop (resource
902/903) and the **same** coordinate transform. The only difference is which
sectors (and their systems) are active:

- Small: 10 sectors × 10 systems = 100 systems → clusters within the starfield
- Large: 20 sectors × 10 systems = 200 systems → full starfield utilised

The coordinates are the same DAT values in both cases. A Small galaxy simply
has fewer systems — the remaining 100 are not loaded.

---

## 7. Sector Icon Resources (Galaxy View)

**Source**: FUN_00426ee0

12 sector icon resource IDs stored at galaxy view +0x294 through +0x2c0:

| Offset | Resource ID |
|--------|-------------|
| +0x294 | 0x27a5 (10149) |
| +0x298 | 0x27a4 (10148) |
| +0x29c | 0x27a3 (10147) |
| +0x2a0 | 0x27a2 (10146) |
| +0x2a4 | 0x27a6 (10150) |
| +0x2a8 | 0x27a7 (10151) |
| +0x2ac | 0x27a8 (10152) |
| +0x2b0 | 0x27a9 (10153) |
| +0x2b4 | 0x27aa (10154) |
| +0x2b8 | 0x27ab (10155) |
| +0x2bc | 0x27ac (10156) |
| +0x2c0 | 0x27ad (10157) |

Plus two additional bitmap arrays:
- +0x2c4: resource 0x27ae (10158), 10 frames (FUN_005fbd20)
- +0x2c8: resource 0x27b6 (10166), 10 frames (FUN_005fbd20)

---

## 8. Text Label Object Layout

**Source**: FUN_00601ce0 (paint), FUN_00601b30 (set position), FUN_00601c60
(set font), FUN_00601c90 (set color), FUN_00403e90 (set format),
FUN_00601aa0 (set text).

| Offset | Type | Field |
|--------|------|-------|
| +0x08 | void* | text string (CString) |
| +0x0c | COLORREF | text color |
| +0x10 | uint | DrawText format flags |
| +0x14 | RECT | bounding rectangle (left, top, right, bottom) |
| +0x28 | HFONT | font handle |
| +0x2c | uint | additional flags (bit 4 → DT_NOCLIP toggle) |
| +0x30 | int | visibility (0 = visible, non-zero = hidden) |

Paint uses SetBkMode(TRANSPARENT), SelectObject for font, SetTextColor,
DrawTextA with computed format flags.

---

## Summary of Differences (Port vs Original)

| Aspect | Original (Traced) | Current Port | Fix |
|--------|-------------------|--------------|-----|
| **Backdrop** | 607 × 437 at (21,25)/(84,27) | Approximate aperture rects | Use exact offsets |
| **Transform** | Linear: x × 607/1024 + offset | Camera-centred projection | Replace with linear formula |
| **Galaxy size** | Filters sectors by `galaxy_size <= chosen` | Loads all 200 systems | Add filter in lib.rs:169 |
| **GID caption** | Arial 14px Normal, DT_CENTER | 11px macroquad draw_text | Use correct font metrics |
| **Camera** | Fixed, no pan/zoom | Has `GALAXY_CAMERA_CENTER` | Remove camera logic |
