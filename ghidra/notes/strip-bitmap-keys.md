---
title: "Tactical Strip Bitmaps: Transparent Keys"
description: "What a loaded bitmap's +8 and +9 to +0xb hold (FUN_005fd170), and the key of each task-force and fighter-group strip bitmap read from TACTICAL.DLL"
category: "ghidra"
tags: [tactical, strip, bitmap, transparent-key, buttons]
---

# Tactical strip bitmaps: transparent keys

Answers task-graph node `or-ep-task-force-strip-5kdd.15`. Coordinates and ids follow `tactical-group-strips.md`.

## Question

Do the strip bitmaps' transparent keys, as the original derives them, equal the first stored pixel the port keys on (`BmpCache::get_macroquad_button`, `crates/rebellion-render/src/bmp_cache.rs:1744`)?

## Answer

Yes, for every strip bitmap. All sixteen are uncompressed 8-bit, so the original's key is the first stored byte, which is the byte the port reads. No leading RLE8 escape occurs.

## What the original stores

- `FUN_005fc050` finds the `RT_BITMAP` resource (`FUN_005fc050.c:13-26`), then `FUN_005fc4a0` sets `+0x10` to the palette size (header `+0x20`, else `1 << bpp`; 0 for 24 bit or more, `FUN_005fc4a0.c:8-19`) and `FUN_005fc4d0` sets `+4`, the bits pointer, to the DIB base plus header size plus four bytes per palette entry (`FUN_005fc4d0.c:11-12`). `+4` is therefore the first stored byte, the bottom-left pixel of a bottom-up DIB.
- `FUN_005fbd20` constructs with `FUN_005fc500`, which zeroes the fields (`FUN_005fc500.c:5-13`), loads through `FUN_005fc050`, and calls `FUN_005fd170` when `+4` is set (`FUN_005fbd20.c:15-18`).
- `FUN_005fd170` reads the bit depth through `FUN_005fc100` (header `+0xe`, `FUN_005fc100.c:8`):
  - 8 bit, header compression (`+0x10`) not 1: `+8` is the first stored byte (`FUN_005fd170.c:12-14`).
  - 8 bit, RLE8 (1): `+8` starts at 0; if the first byte is not 0 it is a run, and `+8` is the run's colour byte (second byte, `:17-19`); if the first byte is 0 and the second is above 2 it is a leading absolute run, and `+8` is the third byte (`:21-23`); otherwise (end of line, end of bitmap, delta) `+8` stays 0.
  - 24 bit: `+9`, `+0xa`, `+0xb` receive the first pixel's bytes 2, 1, 0 (`:29-33`), the DIB's BGR turned to RGB.
- `FUN_005fd0f0` hands the key to the blitter: `+8` for 8 bit (`FUN_005fd0f0.c:14-16`), the `+9` colour otherwise (`:19-21`). The strobe button blits every state image through it (`FUN_00602d30.c:68`).

## Strip bitmaps

Read straight from `TACTICAL.DLL` (RT_BITMAP, identical bytes to the extracted `data/base/ui/tactical-dll/BMP/<id>.bmp` the port loads). Palette values are the bitmap's own palette entry for the key.

| Resource id | Control | Bit depth | Compression | Size | First stored bytes | Original key (`+8`) | Port key | Palette entry |
|---|---|---|---|---|---|---|---|---|
| 1001 | Task-force header, Alliance | 8 | 0 (BI_RGB) | 49 x 19 | 24, 24 | 24 | 24 | (72, 72, 72) |
| 1004 | Task-force header, Empire | 8 | 0 | 49 x 19 | 15, 15 | 15 | 15 | (32, 32, 32) |
| 1008 | Fighter-group header, Alliance | 8 | 0 | 49 x 19 | 24, 24 | 24 | 24 | (72, 72, 72) |
| 1010 | Fighter-group header, Empire | 8 | 0 | 49 x 19 | 23, 23 | 23 | 23 | (66, 66, 66) |
| 1005 | Task-force slot, normal | 8 | 0 | 25 x 17 | 17, 17 | 17 | 17 | (40, 40, 40) |
| 1006 | Task-force slot, pressed | 8 | 0 | 25 x 17 | 15, 17 | 15 | 15 | (32, 32, 32) |
| 1007 | Task-force slot, unassigned | 8 | 0 | 25 x 17 | 10, 11 | 10 | 10 | (10, 10, 10) |
| 1012-1015 | Fighter slots Red-Gold, normal | 8 | 0 | 25 x 17 | 17, 17 each | 17 | 17 | (40, 40, 40) |
| 1016-1019 | Fighter slots Red-Gold, pressed | 8 | 0 | 25 x 17 | 15, 17 each | 15 | 15 | (32, 32, 32) |
| 1020 | Fighter slot, unassigned | 8 | 0 | 25 x 17 | 17, 17 | 17 | 17 | (40, 40, 40) |

Every bitmap has 256 palette entries (`biClrUsed` 0); the DIB resources are 2,054 bytes (headers) or 1,542 bytes (slots).

## Port

`button_rgba_image` (`bmp_cache.rs:2129-2145`) clears alpha wherever `BitmapHitMask::from_bytes` marks a pixel not opaque, and that mask takes the byte at the pixel-data offset as the key (`bmp_cache.rs:1507-1509`). For these sixteen bitmaps the offset is 1078 and the byte equals the original's `+8`. The two keys differ only for a bitmap that is RLE8 or 24 bit; no strip bitmap is either.

Neither side branches: the key derivation is per bitmap, and the side only picks the header id (1001 or 1008 Alliance, 1004 or 1010 Empire, `tactical-group-strips.md`). The Alliance and Empire headers differ in key (24 or 23 against 15), the slots do not.

The strip keys are dark greys of the strip's own palette; the results marks and tabs (`tactical-results.md`) and the GID legend bitmaps (`gid-legend.md`) key on palette blue (index 252).

## Open

- `FUN_005fc120` (header size used by `FUN_005fc4d0`) is not decompiled here; the DIB headers in these bitmaps are 40 bytes.

## Decompiles

`FUN_005fd170.c`, `FUN_005fbd20.c`, `FUN_005fd0f0.c`, `FUN_005fc500.c`, `FUN_005fc050.c`, `FUN_005fc4a0.c`, `FUN_005fc4d0.c`, `FUN_005fc100.c`, `FUN_005fc040.c`, `FUN_00602d30.c`, `FUN_00602150.c`, `FUN_005c52e0.c`, `FUN_00603150.c`.
