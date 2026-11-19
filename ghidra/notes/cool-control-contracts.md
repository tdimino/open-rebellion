---
title: "Cool Control Input, Drag, Palette, and Draw Contracts"
description: "Modal mouse routing, system-command policy, CoolGameWindow drag modes, control WndProcs, scrollbar subclasses, GID palette remapping, and sprite draw mode 5"
category: "ghidra"
created: 2026-11-14
updated: 2026-11-19
---

# Cool Control Input, Drag, Palette, and Draw Contracts

This note answers bead `or-q-cool-controls-0u0` for the owned
`REBEXE.EXE` whose SHA-256 is
`b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`.
Addresses are for that build. The result is a behavioral contract for the
Rust port, not a request to reproduce Win32 object layout or message numbers.

The committed decompile evidence is beside this note: `FUN_00600310.c`,
`FUN_006010e0.c`, `FUN_006011f0.c`, `FUN_00601260.c`,
`FUN_00606380.c`, `FUN_00606ee0.c`, `FUN_00607010.c`,
`FUN_006071e0.c`, `FUN_006028c0.c`, `FUN_00605160.c`,
`FUN_00606650.c`, `FUN_006083c0.c`, `FUN_0060c210.c`,
`FUN_0060d020.c`, `FUN_0060e460.c`, `FUN_00610900.c`,
`FUN_005fddd0.c`, `FUN_0041cb30.c`, and `FUN_005fe640.c`.
The small hook at `0x00600ef0`, checkbox procedure at `0x006039a0`, slider
procedure at `0x00604b80`, common scrollbar procedure at `0x0060aca0`,
scrollbar cleanup at `0x0060b500`, and tab procedure at `0x0060d860` were
checked at instruction level because the clean import did not emit separate
decompile files for those entries.

## Modal mouse hook

`FUN_006010e0` installs a thread-local `WH_MOUSE` hook whose callback is
`FUN_00600ef0` when the first modal is pushed. It stores modal records in the
stack rooted at `DAT_006be4e0`/`DAT_006be4e4` and inserts the modal HWND into
that object's allowed-HWND tree at `+0x88`. `FUN_00601260` adds owned control
HWNDs to the active modal's same tree. `FUN_006011f0` removes the modal record
and unhooks when the stack becomes empty.

The callback has one rule:

- forward to `CallNextHookEx` when the hook code is negative, is
  `HC_NOREMOVE` (3), there is no active modal, or the top modal record has no
  object;
- otherwise read the target HWND from the `MOUSEHOOKSTRUCT`, look it up with
  `FUN_0060f070` in the top modal's `+0x88` tree, and forward only if present;
- return 1 for every other target, swallowing that mouse event before an
  enabled underlay control can produce a command.

This is stack-top modality. It is not owner disabling, a nested message loop,
permanent capture, or a late `WM_COMMAND` rejection.

**Port comparison.** `rebellion-windowing::RoutingSnapshot` selects the
highest-painted modal and allows input only to that `WindowId` and its
ownership descendants. Pointer capture is considered before hit routing and
remains interaction-scoped. That is the same semantic boundary as the
top-modal HWND allowlist without copying native handles or hook lifetime into
the renderer-independent crate. No port node is required for modality.

## CoolWindow system-command policy

`FUN_00600310` is the common native WndProc. `WM_CREATE` associates the object
with window-long slot 0, records its HWND at `+0x18`, clears the client area,
and calls virtual slot `+0x14`. Move and size messages update object geometry;
other messages dispatch through that virtual slot when the object is in the
global registry.

For `WM_SYSCOMMAND` (`0x112`), the procedure masks `wParam` with `0xfff0`
and applies policy bits from object `+0x40`:

| Command | Value | Blocking policy |
|---|---:|---:|
| `SC_MOVE` | `0xf010` | bit 0 (`1`) |
| `SC_SIZE` | `0xf000` | bit 1 (`2`) |
| `SC_MINIMIZE` | `0xf020` | bit 2 (`4`) |
| `SC_MAXIMIZE` | `0xf030` | bit 3 (`8`) |
| `SC_RESTORE` | `0xf120` | bit 2 while iconic, or bit 3 while zoomed |

It also blocks a caption double-click (`WM_NCLBUTTONDBLCLK`, hit
`HTCAPTION`) when the window is already zoomed and maximize is forbidden, or
iconic and minimize is forbidden. Allowed operations go to the object's
virtual WndProc.

**Port comparison.** Open Rebellion has canvas-composed strategic windows,
not independent operating-system top-levels or an OS system menu. Its typed
window commands and policy are the semantic interface; adding `SC_*` flags to
`rebellion-windowing` would expose an unused platform mechanism. This is an
intentional adapter exclusion, not a missing domain abstraction.

## CoolGameWindow title drag

`FUN_00606380` initializes drag mode `+0xf4` to 1. `FUN_00606ee0` begins a
drag only when policy `+0xb8` has bit 2 (`4`) and capture is free or already
owned by the window. It records the grab point, marks `+0xe0` active, and
takes temporary capture.

The two recovered modes are:

- **mode 1, live:** `FUN_006071e0` maps the cursor into the parent and calls
  `SetWindowPos(pointer - grab)` during movement;
- **mode 2, outline:** press stores the pointer and original top-left at
  `+0xe4..+0xf0`; movement erases and redraws a window-sized XOR rectangle
  with `ROP2=7` rather than moving the HWND.

`FUN_00607010` ends both modes. It clears active state, releases capture if
owned, erases the outline or reads the live HWND rectangle, then clamps the
final top-left to the rectangle at `+0xbc`, `+0xc0`, `+0xc4`, `+0xc8`. The
right and bottom tests include the full window width and height at
`+0x38`/`+0x3c`; `SetWindowPos` applies a corrected final position.

**Port comparison.** The renderer implements live title movement, temporary
capture and the common release-time clamp through
`strategic_window_common::update_title_drag` and
`system_window::clamp_window_to_galaxy`. System, Fleet, Defenses and Missions
supply their full window sizes. Sector detail is immovable; no retained
constructor enables mode 2 for a current consumer. The
[consumer matrix](cool-control-migration-matrix.md#coolgamewindow-title-dragging) records the
`.10` inventory and the bounded outline-mode exclusion.

## Eleven control WndProc families

The virtual slot at vtable `+0x14` is the control's WndProc. The 11 recovered
families share the CoolWindow association and dispatch above but own different
state machines:

| Family / vtable | WndProc | Recovered responsibility | Port node |
|---|---|---|---|
| CoolStrobeButton / `0x66df58` | `FUN_006028c0` | create/palette and paint states; arm/capture on press; click/repeat timer; cancel/capture cleanup; owner notifications and optional sound/tooltip | `or-ep-uniform-ui-controls-gru5.2` |
| Checkbox / `0x66df88` | `0x006039a0` | bind and paint the current bitmap/toggle state, delegating remaining button behavior to StrobeButton | `or-ep-uniform-ui-controls-gru5.2` |
| Slider / `0x66e010` | `0x00604b80` | create/paint; capture and begin on left press; update while left is held and capture is owned; release and notify owner with current value | `or-ep-uniform-ui-controls-gru5.8` |
| CoolStringField / `0x66e038` | `FUN_00605160` | focus/caret/selection/overwrite; accepted-character insertion and type-ahead `0x408`; navigation, edit, Insert and Enter; Enter posts `0x407`; unhandled keys forward to owner | `or-ep-uniform-ui-controls-gru5.7` |
| CoolGameWindow / `0x66e060` | `FUN_00606650` | top-level create/paint, title-drag mouse routing, timer/command/key dispatch | `or-ep-uniform-ui-controls-gru5.10` |
| CoolDragList / `0x66e0f0` | `FUN_006083c0` | row paint; bounded keyboard selection; modifier/right-press selection; capture and thresholded drag; reveal/scroll; release reports outside drop `0x29a` or selection `0x29b` | `or-ep-uniform-ui-controls-gru5.4` and `.5` |
| ScrollBar base / `0x66e158` | `FUN_0060aca0` | create/paint, erase suppression, arrow command dispatch, pointer routing and cancellation/repeat cleanup | `or-ep-uniform-ui-controls-gru5.4` and `.8` |
| CoolSelectionBox / `0x66e1c8` | `FUN_0060c210` | paint current item, open popup, and forward character/key input to owner | `or-ep-uniform-ui-controls-gru5.6` |
| Selection popup / `0x66e1f0` | `FUN_0060d020` | host DragList plus scrollbar, forward pointer input, dismiss on outside press, commit selected row and close | `or-ep-uniform-ui-controls-gru5.6` |
| CoolTabControl / `0x66e230` | `FUN_0060d860` | map child-button commands to a selected tab, toggle old/new child state, and notify parent | `or-ep-uniform-ui-controls-gru5.3` |
| CoolwinStatusBar / `0x66e258` | `FUN_0060e460` | paint outline and `current/max` fill with an optional tick interval | `or-ep-uniform-ui-controls-gru5.8` |

The mapping is behavioral, not an instruction to reproduce inheritance. The
uniform-controls epic uses composition and typed outcomes while keeping
feature data, art selection, and game commands in feature modules.

## Vertical and horizontal scrollbar subclasses

Vtables `0x66e2e8` and `0x66e340` retain the common WndProc at slot `+0x14`
and the common base operations through `+0x20`. Slots `+0x24..+0x48` are
orientation-specific geometry, hit-testing, paint, press, page and thumb-drag
operations. The vertical family begins with `FUN_0060f6a0` and the horizontal
family with `FUN_00610130`.

`FUN_0060b500` is shared release/cancel cleanup: reset pressed state, release
capture when owned, kill the repeat timer at `+0xb0`, redraw, and notify the
owner. `FUN_00610900`, the horizontal thumb movement path, advances only when
both cursor x *and* cursor y differ from their prior values. The vertical twin
tests its active axis. The horizontal `&&` is an original bug, not a visible
contract to preserve. The existing generic vertical ScrollBar is the right
Rust seam; epic task `.8` owns horizontal adoption and records the deliberate
axis-only correction.

## GID legend palette remap

`FUN_005fddd0` operates only on an 8-bit bitmap. It calls the supplied
transform for each of 256 palette entries, then builds a 256-byte table by
choosing the original-palette entry with minimum squared RGB distance from
each transformed color. The source index is the initial candidate, so it wins
any distance tie. If it is not an exact match, all indices are scanned in
ascending order and replace the current winner only on strict improvement;
after the first improvement, later equal-distance candidates cannot replace
it. A zero distance stops the scan.

The callback at `LAB_004ac550` transforms each RGB channel independently:

```text
dark(channel) = floor(channel / 6) + 50
```

The transformed range is therefore 50 through 92. The fourth `RGBQUAD` byte
does not participate. `FUN_00452240` applies this table to the copied galaxy
backdrop before keying the legend content and frame over it.

**Port comparison.** `gid_legend.rs` currently paints a flat premultiplied
black overlay, which is not equivalent to nearest-entry indexed remapping.
Port node `or-port-gid-palette-darkening-ms1p` owns the exact replacement.

## Sprite draw mode 5

`FUN_0041cb30` selects sprite draw behavior. Mode 2 calls `FUN_005fe140`;
mode 3 installs the sprite's table at `+0x30` and calls `FUN_005fe3d0`; mode 5
installs the same pointer and calls `FUN_005fe640`; mode 4 does not draw; the
default uses `FUN_005fd0f0`.

Mode 5 requires an initialized palette table at bitmap `+0x24` and consumes
512 bytes as two indexed maps:

```text
if source_index != transparent_key:
    destination = table[source_index]
else:
    destination = table[256 + old_destination_index]
```

The first half remaps foreground/source indices. The second half remaps the
existing background through transparent source pixels. This is not alpha
blending. No current generic Rust primitive or confirmed live Rust consumer
matches it. Port node `or-port-sprite-draw-mode-5-yv8o` first inventories
mode-5 consumers, then implements the operation only if a supported consumer
needs it; otherwise it records a complete exclusion rather than dead API.

## Result for `rebellion-windowing`

The crate matches the API-shaping modal contract: stack-top modal ownership,
descendant eligibility, capture-before-hit routing, and one routing snapshot
per physical event. The native `SC_*` policy is correctly absent from the
renderer-independent domain because the port has no native top-level system
menu. Pixel drag modes, release clamping, palettes, bitmap composites, and
control-local state machines belong in `rebellion-render`, with only capture,
focus leases, ownership, and routing authority shared through
`rebellion-windowing`.

Every current difference is therefore either deliberately excluded above or
owned by an explicit `port` bead: the existing uniform-control tasks `.2` to
`.8`, title drag `.10`, GID remapping `or-port-gid-palette-darkening-ms1p`,
and mode 5 `or-port-sprite-draw-mode-5-yv8o`.

The production consumer inventory, proposed composition APIs, art ownership,
exclusions and interface-parity crosswalk are recorded in
[`cool-control-migration-matrix.md`](cool-control-migration-matrix.md).


## Frame-batched renderer input

The renderer preserves the source event order when the host batches input:
`FUN_006028c0` arms a button using its button-down coordinates, so a later
motion cannot replace the press hit. Rectangular and bitmap-mask adapters
carry that hit separately from the current hover/release hit.

`FUN_006071e0` publishes live window motion before `FUN_00607010` ends
capture and clamps the final position. The shared title adapter therefore
uses the release frame's last position, retains the exact managed area
identity, and updates presentation copies after movement. A lost capture
without a release retains the last published position.

StringField receives the parent's current keyboard lease. An Encyclopedia
field hidden by a topic cannot reclaim a stale search lease when its owner
focuses the index. This keeps `FUN_00605160` field behavior separate from the
feature's child-focus policy.
