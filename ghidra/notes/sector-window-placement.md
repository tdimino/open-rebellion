# Sector Window Placement

Recovered 2026-10-06 from REBEXE.EXE with Homebrew Ghidra 12.1.3 (read-only,
no analysis). Supplements `FUN_00429ce0.c` (the sector window opener used by
all four finders and the galaxy view's system click). Every function named
here has a `FUN_<address>.c` note in this directory.

## `FUN_00429ce0(view, system_id)` — open or raise a sector window

The opener, called by `FUN_00429440` for every Display/double-click in a
finder, and by the galaxy view when the player clicks a system:

1. `PostMessageA(view->+0x18, 0x409, 0, 0)` — a custom message, hyp: a
   focus/repaint hint to the galaxy view.
2. Look up the system via `FUN_004f3970(view->+0x9c, system_id)`. If the
   system is not found, return null.
3. Compute the window's type key: `(system_id & 0xffff) << 6 | 1`. This
   gives each system a unique child-window id.
4. `FUN_00604500(view->+0x6c, type_key)` — search the view's child list for
   an already-open sector window with this key. **If found, return it
   immediately.** The existing window is raised, not duplicated.

### Placement when a new window is needed

5. `FUN_0041d2d0(1)` — hyp: disable repaints while building.
6. Compute the two columns from the side (`view->+0x9c`):
   ```c
   primary_x   = side == 1 ? 0x3c : 0x78;   // 60 or 120
   y           = side == 1 ? 0x23 : 0x28;   // 35 or 40
   secondary_x = side == 1 ? 300  : 365;
   ```
   Both columns share `y`. The window is always 235 by 360 (step 10); the
   300/365 value is the second column's x, not a width.

7. Walk the view's child window list (`FUN_005f5060(view->+0x6c)`),
   scanning for existing sector windows (identified by `+0x24 & 0x3f == 1`)
   at the target column. The walk finds **two** candidates:
   - `param_1`: a sector window at the target column (`+0x28 == target_x`).
   - `puStack_20`: any other sector window at column 1.
   Both or either may be null.

8. **If no existing window exists at either position**: call
   `FUN_00526560(system, &out)` to get the system's screen position. If
   `out >= DAT_00658bd8 / 2` (the right half of the galaxy), use the
   secondary column. With one column taken, the new window takes the other.

9. **If two candidates exist**: call `FUN_00526560(system, &position)` again.
   If the position is on the left half (`< DAT_00658bd8 / 2`), replace the
   primary-column window; otherwise the secondary-column window. Reuse the
   preferred window's position (`+0x28` x, `+0x2c` y). **The existing window
   at the reused position is destroyed**: `FUN_005f54a0(view->+0x478,
   window->+0x24)` removes it from a reference list and `FUN_00600f90(view,
   window->+0x24)` destroys it.

10. Build the new sector window:
    ```c
    FUN_004591d0(alloc, hInstance, x, y, 0xeb, 0x168, view, palette, type_key, system)
    ```
    The window is 235 by 360 pixels (`0xeb` by `0x168`).

11. Show it (`FUN_005ffce0(window, 0)`) and register it
    (`FUN_0042ac70(view, window)`, `FUN_005f4f10(view->+0x6c, window)`).

12. Z-order: `SetWindowPos(window, HWND_BOTTOM, ...)` then
    `SetWindowPos(toolbar, window, ...)` — the new sector window is placed
    behind the toolbar, so it appears below the cockpit/toolbar but above
    the galaxy background.

## `FUN_00604500(list, key)` — find a child window by type key

Searches a binary tree (`+0x4` left/right at `+0x4`/`+0x8`, key at `+0x24`)
for a node whose `+0x24` matches `key`. Returns the node or null.

This is a standard BST find. The tree is the view's child window list, and
the key at `+0x24` is the window's type identifier (e.g., `0x15` for Fleet
Finder, `0x16` for Troop Finder, or `(system_id << 6) | 1` for a sector
window).

## `FUN_00600f90(view, key)` — destroy a child window

```c
FUN_00600f90(this, key):
    node = FUN_00604500(this->+0x6c, key);  // find by key
    if (node) {
        FUN_005f4fa0(this->+0x6c, node);    // remove from list
        FUN_00600280(node);                   // cleanup
        node->vtable[0](1);                  // destructor with free
    }
    return node != null;
```

Finds the window by its type key in the view's child list, removes it from
the list, cleans it up, and calls its destructor. This is how the sector
window opener destroys a stale sector window to make room for a new one at
the same position.

## `FUN_00526560(system, &out)` — get the system's screen x-position

```c
FUN_00526560(this, out):
    if (this->+0x2c != 0)
        *out = *(this->+0x2c + 0x48);
    else
        *out = 0;
```

Returns the system's screen x-coordinate from its record at `+0x2c + 0x48`.
hyp: `+0x2c` is the system's class record (CStarSystem data), and `+0x48`
is the x-position in galaxy-map pixels. This is used to decide which side of
the galaxy the system is on.

## `DAT_00658bd8` — galaxy width

A static word, `0x03ff` (1023), with its neighbour `DAT_00658bda` (also 1023)
as the height. Only `FUN_00429ce0` and `FUN_00425d00` read them; nothing
writes them. So the halves split at x 511. hyp: the x `FUN_00526560` reads is
the record's map position, `Sector.x` in the port.

## What happens to an already-open sector window

When `FUN_00604500` finds an existing window with the same `type_key`
(derived from `system_id`), the opener returns it immediately without
creating a new one. The caller (e.g., `FUN_00429440`) then opens a
sub-window in it (Fleet window, Defense window) and selects the target
entity. **The existing sector window is raised/focused** by the caller's own
logic, not by the opener.

When a sector window needs to open for a *different* system but the target
screen position is occupied, the occupying sector window is **destroyed**
(`FUN_00600f90`) and replaced.

## Port notes

- The port should reuse an existing sector window for the same system rather
  than opening a duplicate.
- At most two sector windows can be open: one on each side of the galaxy.
  The original destroys one to make room for a new one.
- The side-of-galaxy test uses the system's x-coordinate from its class
  record, compared to half the galaxy width.
- The sector window is placed behind the toolbar in z-order.
- port: the `FUN_00429ce0` type_key formula `(system_id << 6) | 1` is the
  sector window's identity; the port can use the system key directly.
- port: `FUN_00600f90` is a general child-window destructor, not
  sector-specific.

## Open

- The exact meaning of `+0x478` in the view (the reference list that
  `FUN_005f54a0` removes from before destruction).
- The toolbar window at `+0x248` and its z-order relationship.
- Whether `FUN_0042ac70` registers the window for focus tracking or event
  routing.
- The `FUN_004591d0` sector window constructor's full parameter list and
  the sector window's own vtable.

## Supporting decompiles

`FUN_00429ce0`, `FUN_00604500`, `FUN_00600f90`, `FUN_00526560`.
