# Tactical projectile and field path

The saved `REBEXE.EXE` project establishes two separate tactical presentation
paths: retained projectile meshes and target-attached indexed field frames.
This note records the bounded findings used by P58F3.

## Projectile dispatch

The four weapon notifications call the target render object's projectile
entry, which reaches `FUN_005d3de0` and constructs `FUN_005ee590`:

| Weapon | Dispatcher | Low/high threshold |
|---|---|---:|
| Laser | `FUN_005b1ea0` | 28.8 |
| Turbolaser | `FUN_005b1f60` | 34.666668 |
| Ion cannon | `FUN_005b2080` | 32.0 |
| Torpedo or fighter attack | `FUN_005b2c70` | 12.8 |

`FUN_005ee590` creates one of three double-sided triangle lists over twelve
fixed vertices. The constructor uses longitudinal scales 0.2, 0.4, 0.5, 0.65,
0.75, or 1.0 according to the weapon, threshold branch, and selected special
turbolaser ordinals 11, 13, 14, 22, and 25. Its material selectors are red for
Alliance fire, green for Imperial fire, and blue for ion fire. Variants one
and two live for one second. Variant three lives for two seconds.

`LAB_005eeb90` retains the captured source point, reads the target's current
point, linearly interpolates between them, and expires at the constructor's
duration. The modern renderer submits the exact recovered mesh. Its
0.001-unit faces fall below one device pixel in WebGL, so the compatibility
path restores one-pixel raster coverage along the same projected mesh axis.
This changes raster coverage only, not the recovered geometry or trajectory.

## Tractor and gravity fields

`FUN_005d3ac0` owns one shared target-attached field slot and `FUN_005d3cc0`
advances it through eight 128 by 128 frames at 0.1 seconds per frame:

- tractor field: type-303 resources 3620 through 3627;
- gravity field: type-303 resources 3520 through 3527.

Tractor sources start and stop through `FUN_005b23e0` and `FUN_005b2440`.
Gravity sources start and stop through `FUN_005b24d0` and `FUN_005b2480`.
Gravity takes visible priority in the shared slot. When its last source ends,
an active tractor field becomes visible again. Multiple source counts are
preserved rather than collapsed to one boolean.

## Acceptance boundary

P58F3 proves the constructor constants, mesh variants, colors, duration,
interpolation, field families, timing, source counts, priority, browser
transport, and framebuffer presentation. Exact combat-command start and stop
timing, the unused white material branch, fighter emission, special-weapon
states, audio, and lossless original-runtime pixel comparison remain open.
