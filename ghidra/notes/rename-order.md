---
title: "Rename order (0x203)"
description: "The in-place rename flow from popup menu to CoolStringField edit, max name length, and the name setter on CRebObject"
category: "ghidra"
created: 2026-10-06
updated: 2026-10-06
tags: [rename, order, coolStringField, popup-menu, fleet-window]
---

# Rename order (0x203)

Order `0x203` (Rename) and `0x215` share a handler in `FUN_00486fb0.c`. Both
bypass the unrecruited check in the popup menu builder (`object-popup-menu.md`,
line 106). The flow: popup menu click → `FUN_0041cdf0` → `FUN_00436020` →
`FUN_00486fb0(0x203, target, selection, param_4)`.

## Handler in `FUN_00486fb0` (LAB_00487439)

```c
case 0x203:
    goto LAB_00487439;
case 0x215:
LAB_00487439:
    piVar6 = FUN_00487c50(this, param_1, param_2);  // build the order
    if (piVar6 == NULL) return;
    if (param_4 == 0) {
        FUN_0041d600(piVar6);   // start the rename edit UI
        return;
    }
    FUN_005f3090(piVar6 + 0x11, param_4);  // set name directly (from param_4)
    iVar7 = 0;
    goto LAB_0048746a;                      // execute via FUN_00487740
```

When `param_4 == 0` (typical popup menu path, no name supplied), it opens the
in-place edit. When `param_4` carries a name string (load, replay, network),
it applies the name and executes immediately.

## Order factory (`FUN_00487c50`, `FUN_004f5cd0`, `FUN_0051f8f0`)

`FUN_00487c50` calls `FUN_004f5cd0(kind)` which gates on `kind >= 0x200`
(plus a few low IDs: 1, 2, 3, 5, 0x11). For `0x203`, it passes the gate and
calls `FUN_0051f8f0(0x203)` to look up the order in the factory table at
`DAT_006b6fe4`. The table entries are 8 bytes: kind at offset 0, factory
function pointer at offset 4. The table count is at `DAT_006b8fd8`.

After construction, the order's `+0x20` is set to the player side, and
`vtable+0x24` and `vtable+0x2c` are called (hyp: set target and set source).

## Routing to the window (`FUN_0041d600` → `FUN_00429350`)

`FUN_0041d600` calls `FUN_00429350(galaxy_view, order)`:

1. Gets the rename target from the order via `vtable+0x28`.
2. Iterates the galaxy view's modeless window list at `+0x6c`
   (`FUN_005f5500 / FUN_005f5c60`).
3. For each window, checks if the target object belongs to it via
   `FUN_004f5940(window + 0x45, &local)`.
4. **Sector window** (type & 0x3f == 9, vtable `0x0065e600`): calls
   `FUN_00458b50(window, order)`. This stores the order at `+0x268`, sets
   `+0x164` bit 3, and calls `FUN_00458640` to enter rename mode.
5. **Active window** (window HWND == galaxy `+0xb8`): calls `vtable+0x78`
   (start rename) on the window with the order.

## Start edit (`FUN_004ac7a0`, vtable `+0x78` on `CustomDialogBox`)

Shared by fleet window (type 4) and other `CustomDialogBox` subclasses:

1. Gets the galaxy view (`FUN_00422ca0`) and the player side from `+0x9c`.
2. Gets the target from the order via `vtable+0x28` and resolves the object.
3. Stores the pending order at `+0x13c` (window `[0x4f]`).
4. Hides the current edit if any (`vtable+0x80`).
5. Gets the selection rectangle from `vtable+0x88`.
6. Creates a `CoolStringField` (`FUN_00604cf0`) at that rectangle with
   style `WS_CHILD | WS_VISIBLE | WS_CLIPSIBLINGS` (`0x56000000`).
   The field is stored at `+0x138`.
7. Calls `vtable+0x18` with argument `10`: slot `+0x18` of the
   CoolStringField vtable (`0x0066e038`) is `0x00604f70`, which calls
   `FUN_00600290(10)` (font table `0x6be4f8` → `+0x7c`) and relays the
   layout. So 10 is the font, as `FUN_006002b0(10)` elsewhere, not a length.
8. Gets the current object name via `FUN_004f6270` (checks `+0x34`, then
   `+0x2c` class name, then default at `DAT_006b120c`).
9. Sets the text via `FUN_00604f90`, which truncates to the max length at
   `+0xa4` if set.
10. Selects all text: `FUN_00605110(field, 0, -1)` sets `+0xb4 = 0`,
    `+0xb8 = -1`.
11. Sets focus to the edit field.

### CoolStringField (`FUN_00604cf0`)

Constructor creates a Win32 control with class name `CoolStringField`
(`s_CoolStringField_006ac7f4`). Key fields:

| Offset | Purpose |
|--------|---------|
| `+0x18` | HWND |
| `+0x98` | Text buffer (via `FUN_005f35e0`) |
| `+0xa4` | Max text length (0 = unlimited) |
| `+0xa8` | Owner window |
| `+0xb4` | Selection start |
| `+0xb8` | Selection end |
| `+0xc4` | X position |
| `+0xc8` | Y position |
| `+0xd8` | Width |
| `+0xdc` | Height |

## Commit edit (`FUN_004ac950`, vtable `+0x7c`)

1. Gets the text from the edit field's `+0x98` (via `+0x138 → +0x98`).
2. If the text is non-empty:
   - Copies it to the pending order's `+0x44` via `FUN_005f3090`.
   - Calls `FUN_0041ce20` on the pending order (hyp: executes the order,
     which routes to the name setter `FUN_004f6e60`).
3. Clears `+0x13c` (pending order) and `+0x138` (edit field).
4. Destroys the edit field's HWND.
5. Returns focus to the window (`+0x18`).

## Cancel edit (`FUN_004aca40`, vtable `+0x80`)

1. Sets focus to `+0x18` (the window).
2. If `+0x138` is non-null:
   - `DestroyWindow(+0x138 → +0x18)`.
   - Calls the edit field's destructor.
   - Clears `+0x138`.

## Name setter (`FUN_004f6e60`)

The execution path for the rename order ultimately calls `FUN_004f6e60` on the
target `CRebObject`:

1. **Alive check**: `FUN_0053a000(object)` returns true when `(+0x24 & 0x30) == 0`
   (bits 4-5 clear). Dead or captured objects cannot be renamed.
2. Gets old name via `FUN_004f62d0`.
3. Compares old and new via `FUN_005f3390`. If identical, no-op.
4. Allocates name storage at `+0x34` via `FUN_004fc680` (if `+0x34` was null,
   allocates via `FUN_005f2f50`; returns 0 on failure).
5. Copies the new name to `+0x34` via `FUN_005f3090`.
6. Notifies both sides via `FUN_004f9ba0`.
7. Calls `vtable+0xec` with (old_name, `+0x34`, change_source) to broadcast
   the rename event. hyp: this updates display in all open windows.

## Which windows support rename

- **Fleet window** (type 4): vtable `+0x78`, `+0x7c`, `+0x80` are
  `FUN_004ac7a0`, `FUN_004ac950`, `FUN_004aca40`. These are on the
  `CustomDialogBox` base class (vtable `0x0065bf80`), so any subclass that
  doesn't override them inherits the rename edit.
- **Sector window** (type 9, vtable `0x0065e600`): uses its own handler
  `FUN_00458b50`, which stores the order at `+0x268` and flags `+0x164` bit 3
  to enter rename mode. hyp: `FUN_00458640` sets up the edit in the sector
  window's own layout.
- hyp: System window (type 7), Defenses window, and any other `CustomDialogBox`
  subclass could inherit the rename slots. The routing in `FUN_00429350` checks
  for the target object's presence in each window's inventory.

## What objects can be renamed

From `object-popup-menu.md`: orders `0x203`, `0x204`, `0x215`, and `0x241` are
exempt from the unrecruited check (line 106). This means 0x203 (Rename) is
available on unrecruited objects—it applies to any `CRebObject` whose order
list includes `0x203`.

hyp: fleets, ships, and characters all carry `0x203` in their order list (from
the vtable `+0x3c` slot). Systems may also be renameable. The exact order
lists per object class are in `object-popup-menu.md` and the vtable notes.

## Max name length

None is set. The constructor `FUN_00604cf0` writes `+0xa4 = 0`, and nothing
in the rename path writes it again; `FUN_00604f90` truncates only when
`+0xa4` is not 0. (Corrected 2026-10-06: `vtable+0x18(10)` sets the font.)

## Ship name display

Ships and fleets display their name via `FUN_004f6270`:

1. If `+0x34` is non-null: returns the custom name.
2. Else if `+0x2c` is non-null: returns the class/type name (e.g.
   "X-Wing Fighter").
3. Else: returns the default string at `DAT_006b120c`.

In the fleet window, ship list items call `FUN_004f6270` to display the ship
name. After a rename, the vtable `+0xec` notification triggers a redraw.

## Port

`FleetWindowState::begin_rename` opens the edit over the entry with the name
selected; an emptied name keeps it open (`FUN_004ac950`), Enter submits
(`PanelAction::Rename`, logged as `[interface] command=0x203
destination=rename status=applied`). Gate: `tools/interface-parity/
fleet-window.mjs`, case `rename` (fixture code 48), both sides.

## Cited decompiles

- `FUN_00486fb0.c` — Command dispatcher (0x203/0x215 handler)
- `FUN_00487c50.c` — Order factory wrapper
- `FUN_004f5cd0.c` — Order factory gate
- `FUN_0051f8f0.c` — Order factory table lookup
- `FUN_0041d600.c` — Routes to FUN_00429350
- `FUN_00429350.c` — Window routing (sector type 9 vs active window)
- `FUN_004ac7a0.c` — Start rename edit (CoolStringField creation)
- `FUN_004ac950.c` — Commit rename edit
- `FUN_004aca40.c` — Cancel rename edit
- `FUN_00604cf0.c` — CoolStringField constructor
- `FUN_00604f90.c` — CoolStringField set text
- `FUN_00605110.c` — CoolStringField set selection
- `FUN_004f6e60.c` — CRebObject name setter
- `FUN_004f6270.c` — CRebObject name getter
- `FUN_004fc680.c` — Name storage allocator
- `FUN_0053a000.c` — Object alive check
- `FUN_00458b50.c` — Sector window rename handler

## Open

- The exact order lists per object class (which classes carry 0x203 and 0x215).
  The vtable `+0x3c` slot returns a linked list of allowed orders; each class's
  list needs to be dumped.
- What `0x215` names. It shares the rename handler. hyp: "Name Fleet" vs.
  "Rename" (unit), or the two may be identical orders from different menus.
- The sector window's rename edit layout (`FUN_00458640`).
- Whether the CoolStringField's window procedure also sends EM_LIMITTEXT to
  enforce the 10-character limit on keyboard input, or relies solely on the
  truncation in FUN_00604f90.
- Where renamed object names are stored in the save file (offset within the
  object's save record at `+0x34`).
