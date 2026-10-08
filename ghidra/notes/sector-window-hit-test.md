# Sector Window Hit Tests

Sector window class `FUN_004591d0`, vtable `0x00659f18`. Two hit-test
methods: `+0x70` = `FUN_0045c830` (container, for move orders) and `+0x68` =
`FUN_0045c6b0` (object, for selections).

## Item structure (list +0x164)

Each planet item is a 0x74-byte object (base `FUN_0060ba00`). Relevant
offsets:

| Offset | Type | Field |
|---|---|---|
| `+0x0c` | `uint` | flag / match key (system id low 24 bits) |
| `+0x14` | string | system name |
| `+0x20` | `bitmap*` | primary bitmap (planet composite) |
| `+0x24` | `bitmap*` | secondary bitmap |
| `+0x28` | `int` | x position (client pixels) |
| `+0x2c` | `int` | y position (client pixels) |
| `+0x34` | `int` | y-offset to label (= planet_h + 11) |
| `+0x40` | `RECT` | hit-test rectangle (4 LONGs: left, top, right, bottom) |
| `+0x54` | `uint` | status flags (alliance/empire/fleet bits) |
| `+0x70` | `bitmap*` | fleet status bitmap drawn at composite bottom |

## Hit rect formula

Written once in `FUN_00459e30` (vtable slot 14, sector window init), never
updated. `FUN_0045bbb0` (planet refresh) reads `+0x40` but never writes it.

```
item.x      = ftol(sector_relative_x_scaled)
item.y      = ftol(sector_relative_y_scaled)

rect.left   = item.x
rect.top    = item.y
rect.right  = item.x + planet_bitmap_width
rect.bottom = item.y + planet_bitmap_height
```

Source: `FUN_00459e30` lines (from decompile):

```c
SetRect(&tStack_58, iVar3, iVar8, iStack_60 + iVar3, iStack_68 + iVar8);
*(int *)((int)pvVar4 + 0x28) = iVar3;
*(int *)((int)pvVar4 + 0x2c) = iVar8;
*(LONG *)((int)pvVar4 + 0x40) = tStack_58.left;
*(LONG *)((int)pvVar4 + 0x44) = tStack_58.top;
*(LONG *)((int)pvVar4 + 0x48) = tStack_58.right;
*(LONG *)((int)pvVar4 + 0x4c) = tStack_58.bottom;
```

`iStack_60` and `iStack_68` are measured from a type-1 planet bitmap:

```c
uVar17 = FUN_0045c970(1);                         // resource 0x27e4 = 10212
piVar7 = FUN_005fbd20(pvVar4, piVar12, uVar17, uVar20);
iStack_60 = FUN_005fc0e0(piVar7);                 // bitmap_width  = 37
iStack_68 = FUN_005fc0f0(piVar7);                 // bitmap_height = 37
```

All 28 planet bitmaps (STRATEGY.DLL 10212-10239) are **37 x 37** (verified from
extracted BMPs). The hit rect is therefore:

```
origin = (x, y)    in client pixels
size   = 37 x 37   pixels
```

The label offset is also set during init (`FUN_00459e30` line 293):
```c
*(int *)((int)pvVar4 + 0x34) = iStack_68 + 0xb;   // = 37 + 11 = 48
```

## Container hit test: +0x70 = FUN_0045c830

Delegates to `FUN_0045c660` which walks list `+0x164` front-to-back:

```c
piVar1 = list.first();   // *(code **)(*(int *)((int)this + 0x164) + 8)
while (piVar1 != NULL) {
    PtInRect((RECT *)(piVar1 + 0x10), pt);   // piVar1 + 0x10 = +0x40
    if (hit) return piVar1;
    piVar1 = list.next();
}
return NULL;
```

Returns the id of the first planet item whose 37x37 rect contains the point.
If items overlap, the item added first to the list wins (append order from
`FUN_005f59f0` during init).

## Object hit test: +0x68 = FUN_0045c6b0

1. `FUN_0045c660` finds the planet item whose rect holds the point.
2. `FUN_0060a860` searches overlay list `+0x174` for the first item with
   id `0x10000` at `+0x0c` (the headquarters overlay).
3. If no overlay: return the planet item's id.
4. If overlay found: `PtInRect` on overlay `+0x40` (same rect as planet).
5. If point in overlay rect: `FUN_005fca00` does a per-pixel transparent-color
   test on overlay `+0x20` (bitmap at `(x - rect.left, y - rect.top)`).
6. Pixel opaque → return overlay id (the headquarters). Pixel transparent or
   miss → return planet id.

The headquarters overlay is a 37x37 icon (STRATEGY 0x388 = resource 904,
verified 37x37: a gold disc on a mast). Created in `FUN_0045bbb0` via
`FUN_00442130(alloc, 0x10000, object id)`, added to list `+0x174` by
`FUN_005f59f0`. Corrected 2026-10-08: earlier revisions of this note called it
a fleet-status overlay.

`FUN_005fca00` pixel test (`FUN_005fca00.c`):
```c
if (param_1 < bitmap_width && 0 < param_1 &&
    param_2 < bitmap_height && 0 < param_2) {
    if (pixel_at(param_1, param_2) != transparent_color)
        return true;
}
return false;
```

Note: coords must be `> 0` (not `>= 0`), so column 0 and row 0 are always
missed. hyp: off-by-one inherited from the original engine's bitmap class.

## Overlay items

Two kinds of overlay in list `+0x174`:

### Headquarters overlay (id 0x10000)

`FUN_0045bbb0` (the planet refresh) resolves the player side's view of the
system (`FUN_004f3220` on galaxy view `+0x194` → `+0x9c`) and counts its
objects of families `0x20..=0x22` (`FUN_00526cf0(out, view, 3)`, which fills
`local_3c` and leaves the count at `local_20`). ALLFACSD `0x20000001`, family `0x20`, is the Alliance
headquarters. A nonzero count sets status bit 8; a destroyed system
(view `+0x50` bit 3) takes status `0x40` and the `0x2800` picture instead.
While bit 8 holds, the refresh creates the overlay once (bitmap `0x388`,
`+0x54` = 1, `+0x3c` = 6), takes the object `FUN_0052bed0` found as its id, and
copies the planet's rect into `+0x40..+0x4c`. When bit 8 clears it removes
the overlay (`FUN_005f5ac0`, then the destructor) unless the object it names
is not destroyed and its container's id matches window `+0x144` (hyp: the
headquarters still stands at a system in this sector).

Because the overlay is found by its fixed id (`FUN_0060a860(+0x174,
0x10000)`), a window holds at most one: there is one headquarters. The
quadrant items are built first (`FUN_00459e30`), so the marker is appended
after them and painted above them. `FUN_0045c6b0` pixel-tests it, so a press
on the disc selects the headquarters and a press on its transparent corners
falls through to the planet.

The same refresh composites STRATEGY 905 (`0x389`, side 1) or 906 (`0x38a`,
side 2) onto the planet picture when the view's `+0x88` bit 4 is set. Both are
flame marks; what sets the bit is untraced (hyp: an uprising).

Port: `quadrant_icons::shows_headquarters` reads the view as the quadrant
icons do (the player's own objects, everything in a system it sees, and the
facilities its side knows), and `sector_window.rs` paints 904 over the planet
after the quadrant icons. port: a press on the marker selects the planet; the
port has no headquarters object to select. 905 and 906 are not drawn.

### Quadrant fleet-position overlays (flags 0x40000..0x400000)

Created per-system in `FUN_00459e30` via `FUN_00442130` with flags:

| Flag | Quadrant | Rect |
|---|---|---|
| `sys_id \| 0x40000` | top-left | `(center_x - 28, center_y - 19, center_x, center_y)` |
| `sys_id \| 0x80000` | bottom-left | `(center_x - 28, center_y + 1, center_x, center_y + 20)` |
| `sys_id \| 0x100000` | top-right | `(center_x + 1, center_y - 19, center_x + 29, center_y)` |
| `sys_id \| 0x400000` | bottom-right | `(center_x + 1, center_y + 1, center_x + 29, center_y + 20)` |

Where `center_x = x + 18` (planet_w/2), `center_y = y + 18` (planet_h/2),
and the overlay dimensions come from fleet icon resource 0x2a13 = 10771
(27x18) + 1 = 28x19.

These are NOT searched by `FUN_0045c6b0` (flag 0x10000 does not match
`0x40003` etc.). They are the quadrant icons: a double click on one opens its
window (`FUN_004593e0` case `0x203`). The right and bottom edges are
exclusive, and each rect starts one pixel off the center on its far side
(corrected 2026-10-04 from `FUN_00459e30:369-447`). See
`sector-quadrants.md` for their show rules, art and windows.

## Label rect: FUN_0045c1f0

Not part of the hit rect. Used only for `InvalidateRect` (unioned with the
hit rect in `FUN_0045bbb0`).

```c
center_x = item.x + (item.bitmap ? bitmap_width(item.bitmap) / 2 : 20);
label_top = item.y + item.offset_0x34;    // y + 48

label.left   = center_x - 50
label.top    = label_top
label.right  = center_x + 50
label.bottom = label_top + 30
```

100 pixels wide, 30 pixels tall, horizontally centered under the planet
picture. The label rect is `UnionRect`'d with the hit rect to produce the
invalidation area but is never checked by `PtInRect`.

## Port comparison

Port: `crates/rebellion-render/src/sector_window.rs`

```rust
let planet_rect = logical_rect(window_rect, layout.scale, planet_x, planet_y, 37.0, 37.0);
```

**Hit rect size matches.** Both use 37x37. The port's `rect_contains` check
on `planet_rect` is equivalent to the original's `PtInRect` on item `+0x40`.

**Position computation matches.** Port `sector_planet_position`:
```rust
((relative_x / 13.0 * 37.0).round(), (relative_y / 10.0 * 37.0).round())
```
The original scales through FPU in `FUN_00459e30` using `FUN_00509620` to get
sector-relative coordinates, then `__ftol()` to truncate. The scale constants
(13, 10, 37) are consistent with the 37x37 grid.

**Layout differences (not hit-test-relevant):**

| Element | Original | Port |
|---|---|---|
| Planet bitmap | y to y + 37 | y to y + 37 |
| Status tracks | y + 37 to y + 41 (4px, inside composite) | y + 48 to y + 59 (3 tracks x 3px each) |
| Label text | y + 48 (top of 30px label) | y + 37 (CENTER_TOP anchor) |

The original embeds status tracks at the bottom of a 37+4=41 pixel composite
bitmap (`FUN_0045bbb0`: `FUN_005fbda0` with height `planet_h + 4`), then
places the label 7 pixels below the composite. The port draws the label
immediately below the planet and puts status tracks below the label. The
visual stacking order is reversed but the hit rect (37x37 from the planet
origin) is the same in both.

## Functions cited

| Address | Role |
|---|---|
| `FUN_004591d0` | sector window constructor |
| `FUN_00459e30` | sector window init (vtable slot 14): creates items, sets rects |
| `FUN_0045bbb0` | planet refresh: rebuilds composite bitmap, manages fleet overlay |
| `FUN_0045c660` | walk list `+0x164`, return first item whose `+0x40` rect holds the point |
| `FUN_0045c6b0` | object hit test (`+0x68`): planet rect + fleet pixel test |
| `FUN_0045c830` | container hit test (`+0x70`): delegates to `FUN_0045c660` |
| `FUN_0045c1f0` | label rect computation (100x30, centered under picture) |
| `FUN_0045c970` | system-type to STRATEGY resource id (0x27e4..0x27ff) |
| `FUN_005fca00` | per-pixel transparent-color hit test on a bitmap |
| `FUN_005fc0e0` | bitmap width (`*(header + 4)`) |
| `FUN_005fc0f0` | bitmap height (`*(header + 8)`) |
| `FUN_005fbda0` | create sized bitmap (width, height) from source palette |
| `FUN_0060a860` | list search by exact match on item `+0x0c` |
| `FUN_0060ba00` | base item constructor; stores flag at `+0x0c` |
| `FUN_0060be60` | set item bitmap pointers (`+0x20`, `+0x24`) |
| `FUN_00442130` | overlay item constructor (flag, id) |
| `FUN_00509620` | read system position from container (`*(+0x2c) + 0x4c`) |
| `FUN_0045b770` | per-system update dispatch (vtable slot 25) |
| `FUN_0045baf0` | status bar update (child window visibility/progress) |
| `FUN_0045c240` | alliance popularity bar update |
| `FUN_0045c450` | empire popularity bar update |
