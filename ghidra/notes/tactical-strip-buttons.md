---
title: "Tactical Strip Buttons: Captions, Hit Test, Disabled Slots and Key Presses"
description: "The CoolStrobeButton behind the tactical task-force and fighter-group strips: caption string, font and colours (FUN_005ef240, FUN_00602520, FUN_00602d30), the assigned and unassigned looks (FUN_005c5480, FUN_005c54d0), the window procedure FUN_006028c0 and its hit test, disabled fighter slots, message 0x40e, number-key presses (FUN_00603710) and press sounds"
category: "ghidra"
tags: [tactical, strip, task-force, fighters, buttons, captions, fonts, hotkeys, sound]
---

# Tactical strip buttons

Answers task-graph nodes `or-ep-task-force-strip-5kdd.1` (the caption) and
`or-ep-task-force-strip-5kdd.2` (input) from this project's decompiles.
`tactical-group-strips.md` gives the strips' geometry and what a selection
posts; this note covers the button class under them. Bit names below are
bits of the button's state word `+0xc4`.

## What the port needs

1. **Caption.** A task-force slot's caption is TEXTTACT `0xad81 + i`, the
   digits `1` to `8`, in application font 4: Arial, `lfHeight` 16 (a 16 px
   cell), weight 400, `lfQuality` 2. It is centred in the 25 x 17 slot,
   transparent background. It draws only while the slot is assigned (bit
   `0x20`). Caption colours: black `0x02000000` at rest, white `0x02ffffff`
   while pressed and white while selected; an unassigned slot draws no
   caption. Fighter slots and both headers have no caption.
2. **Unassigned task-force slot.** Enabled (bit 2 clear), so it takes a
   click. Outside modes `0xe` and `0x10` the click changes nothing: the slot
   paints bitmap 1007 at rest and while pressed, its toggle bit 8 is clear,
   and `FUN_0059b450` returns without acting. In those modes the same click
   assigns the slot (`tactical-group-strips.md`).
3. **Disabled fighter slot.** Takes no click. Vtable `+0x10` sets bit 2 and
   disables the window; the hit test then refuses every point.
4. **Number keys.** `FUN_00603710` posts `WM_COMMAND` with the slot id to
   the tactical window, so keys 1-8 reach `FUN_0059b450`, including its
   unassigned branch. Nothing releases the key press on key-up.
5. **Hit test.** `x <= width` and `y <= height`, edges inclusive, on
   unsigned client coordinates. A 25 x 17 slot takes a press on every pixel
   of its rectangle; a captured release also counts at x 25 or y 17, one
   pixel past the right or bottom edge.
6. **Press sound.** None. No strip slot or header gets a control sound.

Neither side branches anywhere in this note's code except the two header
bitmaps (`FUN_0059be80.c:27-32`, `FUN_0059c030.c:23-27`); fonts, colours,
hit test, key handling and sound are the same for the Empire and the
Alliance.

## or-ep-task-force-strip-5kdd.1: the task-force slot caption

### Source string and setter

`FUN_0059be80` builds eight slots and registers each with
`FUN_005ef240(this, slot, &caption_key, &hover_key)` (`FUN_0059be80.c:69`).
The caption key is `{0xad81 + i, DAT_0065d428}`: `local_14 = i - 0x527f`,
that is `0xad81 + i` as a 16-bit id (`:66`), with group `DAT_0065d428 = 3`
(`:24`; `text-sources.md:20`), the TEXTTACT module
(`tactical-fighter-group-panel.md:157`).

`FUN_005ef240` creates the button window (`FUN_005ffce0`,
`FUN_005ef240.c:6`), sets the caption when argument 2 is non-zero
(`FUN_00602520`, `:7-9`), adds the hover text when argument 3 is non-zero
(`FUN_00600a40`, `:10-12`), and links the button into the window's child list
at `+0x6c` (`FUN_005f4f10`, `:13`).

`FUN_00602520` is the caption setter. It frees the old string object at
`+200` (`FUN_00602520.c:14-17`), stores a new one built from the key by
`FUN_005f2fc0` (`:18-27`), which loads it through `FUN_005f37e0`
(`FUN_005f2fc0.c:8`; `text-sources.md:22`), and lays the caption out
(`FUN_006026e0`, `FUN_00602520.c:28`).

### Font

`FUN_006026e0` measures the caption with `DrawTextA(..., DT_CALCRECT |
DT_CENTER)` (format `0x401`) in the button's font (`FUN_006002c0`,
`FUN_006026e0.c:24-38`), then centres the text rectangle in the button's
width `+0x38` and height `+0x3c`: left `(w - text_w) / 2`, top
`(h - text_h) / 2`, both integer division (`:40-48`). The paint draws the
string in that rectangle with `DT_CENTER` and a transparent background
(`FUN_00602d30.c:83-90`), selecting the font first (`FUN_006002c0`, `:83`) and
restoring the old one after (`FUN_006002f0`, `:90`).

`FUN_006002c0` selects the font handle at `+0x7c` into the DC and keeps the
previous one at `+0x80` (`FUN_006002c0.c:7-13`). The window base ctor sets
`+0x7c` from `FUN_00603850()` through `FUN_00600290`
(`FUN_005ff910.c:144-147`), which takes entry `n` of the font table
`DAT_006be4f8` (`FUN_00600290.c:7-8`, `FUN_0060eea0.c:8-13`).
`FUN_00603850` returns `DAT_006be3b8 + 0xb0` (`FUN_00603850.c:8-10`).

`DAT_006be3b8` is the application object. `FUN_005fb850` stores `this`
there and zeroes `+0xb0` (`FUN_005fb850.c:63-64`); its one caller is
`FUN_00413000` (`FUN_00413000.c:43`), which sets `+0xb0 = 4` at the end of
its construction (`:141`); `FUN_004165b0` allocates the `0x160`-byte object
and calls `FUN_00413000` (`FUN_004165b0.c:80`, `:88`). So the `+0xb0 = 4`
is on the `DAT_006be3b8` that `FUN_00603850` reads, and every strip button
built after start-up takes font entry 4.

The button's vtable `+0x18`, `FUN_00602ff0`, would change the font and
re-lay the caption (`FUN_00602ff0.c:5-6`); no strip builder calls it.

Font entry 4: `FUN_0060eed0` gives it `lfHeight` 16 through the switch
default (`FUN_0060eed0.c:49-51`), weight 400 (`:89`), `lfQuality` 2 (`:25`)
and the face at `this + 0x38` (`:26-47`), which is `Arial`
(`text-sources.md:90`). `galaxy-view-framing.md:221-245` tabulates all 14
entries: entry 4 is Arial, `lfHeight` 16, weight 400, the same as entry 0.

### Colours

The ctor sets the four text colours (`FUN_00602150.c:75-78`);
`FUN_0059be80` overrides the selected one (`FUN_00603010(slot, 4,
0x2ffffff)`, `FUN_0059be80.c:64`; `FUN_00603010.c:16-17`). The paint picks
image and colour from the state (`FUN_00602d30.c:12-62`; the `WM_PAINT`
case passes bit 1 as the state, `FUN_006028c0.c:27-28`):

| Look | Condition | Image (assigned) | Caption colour |
|---|---|---|---|
| Rest | bits 1, 4, `0x40` clear, or bit 1 set with the pointer outside | `+0x94`, 1005 | `+0xb4` `0x02000000` black |
| Pressed | bit 1 and the pointer inside (`+0xf8`) | `+0x98`, 1006 | `+0xb8` `0x02ffffff` white |
| Selected | bit 4 | `+0xa0`, 1006 | `+0xbc` `0x02ffffff` white |
| Disabled | bit 2 or `0x40` | `+0x9c`, 1007 | `+0xc0` `0x02404040` |

A pressed slot with the pointer dragged outside paints the rest look even
when bit 4 is set (`FUN_00602d30.c:19-25`, `:43-45`). The colours are
`PALETTERGB` values (`0x02` high byte). No strip code sets bit 2 or `0x40` on
a task-force slot, so the disabled row is never seen there with a caption.

### Why an unassigned slot draws no caption

The paint draws the caption only while `+200` holds a non-empty string and
bit `0x20` is set (`FUN_00602d30.c:80-82`). Every slot has its string
(`FUN_0059be80.c:69`); bit `0x20` decides.

- The ctor sets `+0xc4 = 0x10020` (`FUN_00602150.c:37`), and
  `FUN_0059be80` adds bit 8 (`FUN_0059be80.c:61`).
- `FUN_005c52e0` keeps a second image pair: `+0x114` and `+0x118`, both
  1007 here (`FUN_005c52e0.c:27-39`; arguments `0x3ef, 0x3ef` at
  `FUN_0059be80.c:55-56`), with `+0x11c = 1` (`FUN_005c52e0.c:40`).
- `FUN_005c54d0` (unassign) runs only while `+0x11c` is non-zero: it sets
  `+0x11c = 0`, swaps `+0x94` with `+0x114` and `+0x98` with `+0x118`, and
  clears bits `0x28`, that is caption bit `0x20` and toggle bit 8
  (`FUN_005c54d0.c:7-15`). `FUN_0059be80` calls it on every slot before the
  slot gets its caption (`FUN_0059be80.c:65`).
- `FUN_005c5480` (assign) is the inverse: it runs while `+0x11c` is 0, swaps
  the pairs back and sets bits `0x28` (`FUN_005c5480.c:7-15`).

So an unassigned slot paints 1007 at rest and while pressed, with bit
`0x20` clear and no caption; an assigned slot paints 1005 and 1006 with its
digit. A0 `TAC-02-C006` (`empire-day30-yaga-alert-control`, task forces 1
and 2 assigned) shows no glyph on slot 3.

Two paths assign and unassign. `FUN_0059b550` walks the eight slots: a slot
whose task force exists and is unassigned gets the hover text `+0xd8 + 4 i`
and `FUN_005c5480`; one whose task force is gone gets the hover text at
`DAT_006bc398` and `FUN_005c54d0` (`FUN_0059b550.c:12-26`). When its index
argument is below 8 it sets bits 1 and 4 on that slot and clears them on
the others (`:27-34`). It is reached as `FUN_0059fc50` → `FUN_005c0e00` →
`FUN_0059b550` with the existence flags of the eight task forces
(`FUN_0059fc50.c:12-18`, `FUN_005c0e00.c:5-6`). `FUN_0059b450`'s unassigned
branch is the other path (`FUN_0059b450.c:39-44`).

### TEXTTACT ids used by both strips

Keys are group 3 (TEXTTACT). Values read from TEXTTACT.DLL `RT_STRING`.
The hover texts reach the tooltip window `DAT_006be5a4` as `TTM_ADDTOOLA`
(`0x404`, `FUN_00600970.c:22`) when the button is registered and as
`TTM_UPDATETIPTEXTA` (`0x40c`, `FUN_00600cc0.c:20-21`) when it changes.

| Id | Text | Role | Set by |
|---|---|---|---|
| `0xad81`-`0xad88` | `1`-`8` | Caption, task-force slot `i` | `FUN_0059be80.c:66`, `:69` |
| `0xad91` | `Task Forces` | Hover, task-force header | `FUN_0059be80.c:43-44` |
| `0xad92` | `Unassigned Task Force` | Hover, task-force slot when built and when unassigned | `FUN_0059be80.c:67`, `:69`; `FUN_0059b550.c:16` (`DAT_006bc398`, set to `0xad92` at `0x0059d096`) |
| `0xad93`-`0xad9a` | `Task Force 1`-`Task Force 8` | Hover, assigned task-force slot | records `+0xd8`-`+0xf4` (`FUN_0059a8d0.c:37-44`; set at `0x0059d0c6`-`0x0059d216`), used by `FUN_0059b450.c:40`, `FUN_0059b550.c:22` |
| `0xad9b` | `Fighter Squadrons` | Hover, fighter header | `FUN_0059c030.c:40-41` |
| `0xad9c` | `Inactive Fighter Group` | Hover, fighter slot when disabled | `FUN_0059b690.c:25` (`DAT_006bc2b4`, set at `0x0059d276`) |
| `0xad9d`-`0xada0` | `Red`/`Blue`/`Green`/`Gold Fighter Group` | Hover, fighter slot when built and when enabled | `FUN_0059c030.c:61-62`; `FUN_0059b690.c:18` (records `+0xf8`-`+0x104`, `FUN_0059a8d0.c:45-48`; set at `0x0059d2a6`-`0x0059d336`) |

No fighter slot or header has a caption: each passes 0 as argument 2
(`FUN_0059be80.c:44`, `FUN_0059c030.c:41`, `:62`), so `+200` stays 0
(`FUN_00602150.c:74`). TEXTTACT `0xad89`-`0xad8c` hold the bare group names
`Red`, `Blue`, `Green`, `Gold`; no strip code uses them.

## or-ep-task-force-strip-5kdd.2: the strip buttons' input

### The window procedure

Every window class the base registers uses `FUN_00600310` as its
`lpfnWndProc`, with class style `0xb` (`CS_VREDRAW | CS_HREDRAW |
CS_DBLCLKS`) (`FUN_005ffc80.c:16`; called with `0xb` from
`FUN_005ffce0.c:29`). `FUN_00600310` stores the object in window long 0 on
`WM_CREATE` and forwards every message to the object's vtable `+0x14`
(`FUN_00600310.c:121-134`, `:103-107`). The strip buttons' vtables are
`0x0066df58` (`FUN_00602150.c:35`; the headers and fighter slots) and
`0x0066ca60` (`FUN_005c52e0.c:20`; the task-force slots). Both hold
`FUN_006028c0` at `+0x14`, `FUN_006028a0` at `+0x10`, `FUN_00602880` at
`+0xc` and the paint `FUN_00602d30` at `+0x24` (vtable dumps of both
addresses). So `FUN_006028c0` is the CoolStrobeButton procedure, and the
task-force subclass overrides neither its procedure nor its paint.

### Press, release and hit test

- **Press** (`WM_LBUTTONDOWN`, and `WM_LBUTTONDBLCLK` through the same code,
  `FUN_006028c0.c:89-93`, `:152`): hit test (`:93`); a press before the
  first paint (bit `0x10000`, set by the ctor and cleared by `WM_PAINT`,
  `:30`) or off the button does nothing (`:94-100`). A hit sets bit 1,
  captures the mouse (`FUN_00601080`), repaints, posts `0x40e` to the parent
  (`:101-105`), plays the control sound if `+0xdc` is non-zero (`:106-108`)
  and starts the auto-repeat object if `+0xfc` is non-zero (`:109-131`).
  Both strip builders pass 0 for `+0xfc` (`FUN_0059be80.c:55-56`,
  `FUN_0059c030.c:52-53`; `FUN_00602150.c:119-125`), so strip buttons never
  repeat.
- **Release** (`WM_LBUTTONUP`, `:150-198`): only after a hit press and while
  this button holds the capture (`:164-173`). It clears bit 1, releases the
  capture (`:174-183`) and tests the point again (`:184`). On a hit it flips
  bit 4 when toggle bit 8 is set (`:186-188`) and posts `WM_COMMAND` with the
  button id to the parent (`:189-192`).
- **Hit test** `FUN_006035f0`: bit 2, or no image for the current look,
  is no hit (`FUN_006035f0.c:24-26`); bit `0x10` tests the bitmap's pixels
  (`:28-31`); otherwise the point hits when `x <= +0x38` and `y <= +0x3c`
  (`:33-36`). `x` is `lParam & 0xffff` and `y` is `lParam >> 16`, unsigned,
  so a point left of or above the button (negative under capture) is a
  large value and misses. `+0x38` and `+0x3c` are the width and height
  (`FUN_005ff910.c:135-136`; the button style `0x46000000`,
  `FUN_00602150.c:30`, has neither `0x800000` nor `0x400000`, so no frame is
  subtracted, `FUN_005ff910.c:116-128`). Both slot builders pass 1 as the
  ctor's last argument, so bit `0x10` stays clear and slots use the
  rectangle; both headers pass 0 and use their bitmap
  (`FUN_00602150.c:115-117`; `FUN_0059be80.c:39`, `:55-56`;
  `FUN_0059c030.c:35`, `:52-53`). The Battle Options buttons
  (`FUN_005dab90.c:25`, `:39`, `:55`, `:70`, `:79`) and the withdraw
  confirmation's Confirm and Cancel (`FUN_005f0f40.c:25`, `:35`) pass 1
  too and use the rectangle (`tactical_battle_options_control_at`,
  `tactical_withdraw_confirmation_control_at`).

For a 25 x 17 task-force slot the rectangle test accepts x 0-25 and y 0-17.
A press arrives only inside the client area, x 0-24 and y 0-16, so every
pixel of the slot takes it; the release, delivered under capture, also hits
at x 25 or y 17. A 25 x 15 fighter slot accepts y 0-15 the same way.

### Message 0x40e

The press posts `0x40e` to the parent with `wParam = 0x201 << 16 | id` and
`lParam` the point (`FUN_006028c0.c:104-105`). The parent of every strip
button is the tactical window (the ctor's parent argument `this`,
`FUN_0059be80.c:39`, `:55`, `FUN_0059c030.c:35`, `:52`; built from
`FUN_0059b080.c:11-12`). Its procedure `FUN_0059ab40` (vtable `0x0066bb00`
`+0x14`) has no `0x40e` case (`FUN_0059ab40.c:19-153`) and passes it to
`FUN_005ff530` (`:154-156`), which has none either and calls
`FUN_006007b0` (`FUN_005ff530.c:56-57`); that handles only `0x18`, `0x205`
and `0x405` before `DefWindowProcA` (`FUN_006007b0.c:8-25`). The tactical
window ignores `0x40e`. Ghidra finds six uses of the immediate `0x40e` in
REBEXE: five pushes that post it (`FUN_006028c0`, `FUN_0059bd30`,
`FUN_006083c0` twice, and code at `0x0060adcb`) and one `ADD` in
`FUN_0049a350`; none compares a message with it.

### Fighter slots and vtable +0x10

`FUN_0059c030` calls vtable `+0x10` on each fighter slot after registering
it (`FUN_0059c030.c:63`). For vtable `0x0066df58` that is `FUN_006028a0`:
it sets bit 2 (`FUN_006028a0.c:5`; `FUN_006030c0.c:5`) and disables the
window (`FUN_005ffe80`, `FUN_006028a0.c:6`; `FUN_005ffe80.c:7-10`).
`FUN_00602880` (`+0xc`) is the inverse: it clears bit 2 and enables the
window (`FUN_00602880.c:5-6`, `FUN_005ffe60.c:7-10`). `FUN_0059b690`
enables the slot of each group that exists and disables the others,
changing the hover text with it (`FUN_0059b690.c:13-28`; `FUN_005ffea0`
reads `IsWindowEnabled`, `FUN_005ffea0.c:5`).

A disabled fighter slot paints the look of bit 2: image `+0x9c`, bitmap
1020 (`FUN_0059c030.c:59`), colour `+0xc0` (`FUN_00602d30.c:12-15`,
`:26-33`). A click on it does nothing at the slot: Windows delivers no mouse
input to a disabled window, and the procedure would refuse it anyway, since
the hit test fails (`FUN_006035f0.c:24-26`), the press returns before
setting bit 1 or posting (`FUN_006028c0.c:97-100`) and the release returns
at `:164-165`. The tactical window's own procedure returns 0 for
`WM_LBUTTONDOWN` (`FUN_0059ab40.c:150-151`).

### Task-force slots: FUN_005c54d0 and FUN_005c5480

`FUN_005c54d0` clears bits `0x20` and 8 and swaps in the 1007 pair;
`FUN_005c5480` sets them and swaps 1005 and 1006 back (the caption section
above). Neither touches bit 2, and `FUN_0059be80` never calls vtable
`+0x10` on a task-force slot, so an unassigned slot is enabled and takes a
click.

A click on an unassigned slot while `FUN_00595b90` is false (neither mode
`0xe` nor `0x10`):

1. The press hits (bit 2 clear, image 1007 present, rectangle test), sets
   bit 1 and posts `0x40e`, which the tactical window ignores. The pressed
   look paints `+0x98`, which holds 1007, with no caption: the slot looks
   the same.
2. The release clears bit 1. Bit 8 is clear, so bit 4 does not flip. It
   posts `WM_COMMAND` with the slot id; `FUN_0059ab40` sends it to
   `FUN_0059b130` (`FUN_0059ab40.c:92-94`), which calls `FUN_0059b450`
   (`tactical-group-strips.md`).
3. `FUN_0059b450` stops at the clicked slot because its task force does not
   exist and `FUN_00595b90()` is 0, and returns (`FUN_0059b450.c:18-19`,
   `:53`): no command, no other slot released.

Nothing changes on screen, which A0 `TAC-02-C012` shows: the click on blank
slot 3 has the frame hash of the rest frame `TAC-02-C008`
(`be4cae91f021…`, `docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-13-tactical-a0-capture-index.md:51`,
`:55`). In mode `0xe` or `0x10` the loop runs through and the unassigned
branch assigns the slot, presses it (bits 1 and 4) and posts `2 + i`
(`FUN_0059b450.c:39-44`).

An assigned slot has bit 8, so each mouse click flips bit 4 on release
before `FUN_0059b450` posts `2 + i` and releases the other slots
(`FUN_0059b450.c:27-37`). A second click on the selected slot therefore
clears its bit 4 while posting the same command.

### Number keys and FUN_00603710

Keys `1`-`8` on `WM_KEYDOWN` or `WM_SYSKEYDOWN` clear bits 1 and 4 on all
eight task-force slots (`FUN_0059b430.c:8-14`) and all fighter slots
(`FUN_0059b600`), then call `FUN_0059ca90(this, 0xad02 + i, 1)`
(`FUN_0059b730.c:9`, `:58-97`). `FUN_0059ca90` finds the child by id in the
list at `+0x6c` and calls `FUN_00603710` (`FUN_0059ca90.c:7-11`).
`FUN_00603710` sets `+0xf8 = 1`, plays the control sound if `+0xdc` is
non-zero, sets bit 1, repaints, and posts `WM_COMMAND` with the button id
(`FUN_00602510` reads `+0x24`, the id the ctor stored,
`FUN_00602150.c:36`) and `lParam` 0 to the parent
(`FUN_00603710.c:9-20`). It tests neither bit 2 nor the task force, so the
key reaches `FUN_0059b450` for an unassigned slot as a click does, and the
unassigned branch runs there in modes `0xe` and `0x10`.

The key press is never released by the key. `FUN_0059b730`'s key-up
switch has no case for `0x31`-`0x38` (`FUN_0059b730.c:133-179`), so
`FUN_00603780`, which clears bit 1 and resets `+0xf8` and `+0x108`
(`FUN_00603780.c:5-11`), is not called for strip slots. The procedure's own
`WM_COMMAND` case, which clears bit 1 and releases the capture when the
command carries its id (`FUN_006028c0.c:133-148`), needs a `WM_COMMAND`
sent to the button; `FUN_00603710` posts to the parent
(`FUN_00603710.c:18-20`). Bit 1 stays set until the next number key
(`FUN_0059b430`), a click on another slot (`FUN_0059b450.c:35`), a strip
refresh (`FUN_0059b550.c:27-34`) or a mouse release on the slot itself
(`FUN_006028c0.c:174`). With `+0xf8 = 1` the key-pressed slot paints the
pressed look, image 1006 and a white caption for an assigned slot, which
matches the selected look. Key autorepeat presses again: only Tab tests
the repeat bit (`FUN_0059b730.c:12`).

### Press sound

The procedure and `FUN_00603710` play the control sound only when the
16-bit id at `+0xdc` is non-zero (`FUN_006028c0.c:106-108`,
`FUN_00603710.c:10-12`). The ctor zeroes it (`FUN_00602150.c:79`);
`FUN_005c52e0` does not set it, nor do `FUN_005ef240` (`FUN_005ef240.c:5-14`),
`FUN_0059be80` or `FUN_0059c030`. Its setter `FUN_00602840` stores the id
at `+0xdc` (`FUN_00602840.c:7`; `audio-dispatch.md:24`); all 53 of its call
sites are in `FUN_00405560`, `FUN_004286b0`, `FUN_0046f360`,
`FUN_004665f0`, `FUN_0044c630`, `FUN_0046ce40`, `FUN_00461960`,
`FUN_0044f860`, `FUN_0045ddc0`, `FUN_004637a0` and `FUN_00460270`
(`audio-dispatch.md:59`), none in the tactical window. No strip slot or
header plays a press sound, on either side.

## Open

- What `FUN_00595b10` does with commands `2 + i` and `0xb + i` beyond
  queuing them, including any tactical sound event (`FUN_005ba980`) or
  voice it triggers, and whether the command makes `FUN_0059fc50` refresh
  the strip with the selected index (its callers `FUN_0059e760`,
  `FUN_005a03f0`, `FUN_005a0410`, `FUN_005a34e0`, `FUN_005ab0e0` are not
  read).
- Where Windows sends a click on a disabled fighter slot is Windows
  behaviour, not this binary's; no A0 capture clicks a greyed fighter slot.

## Decompiles

New with this note: `FUN_005ef240.c`, `FUN_005c5480.c`, `FUN_005c54d0.c`,
`FUN_006002c0.c`, `FUN_006002f0.c`, `FUN_005f2fc0.c`, `FUN_00603710.c`,
`FUN_00603780.c`, `FUN_00602510.c`, `FUN_006028a0.c`, `FUN_00602880.c`,
`FUN_00602ff0.c`, `FUN_0059ab40.c`, `FUN_005ffc80.c`, `FUN_005ffea0.c`,
`FUN_005c0e00.c`, `FUN_0059fc50.c`.

Also cited: `FUN_006028c0.c`, `FUN_006035f0.c`, `FUN_00602d30.c`,
`FUN_00602150.c`, `FUN_005c52e0.c`, `FUN_006026e0.c`, `FUN_00602520.c`,
`FUN_005ff910.c`, `FUN_00603850.c`, `FUN_00600290.c`, `FUN_0060eea0.c`,
`FUN_0060eed0.c`, `FUN_005fb850.c`, `FUN_00413000.c`, `FUN_004165b0.c`,
`FUN_00603010.c`, `FUN_006030c0.c`, `FUN_005ffe60.c`, `FUN_005ffe80.c`,
`FUN_00600310.c`, `FUN_005ffce0.c`, `FUN_005ff530.c`, `FUN_006007b0.c`,
`FUN_00600970.c`, `FUN_00600cc0.c`, `FUN_00602840.c`, `FUN_0059be80.c`,
`FUN_0059c030.c`, `FUN_0059b080.c`, `FUN_0059a8d0.c`, `FUN_0059b450.c`,
`FUN_0059b550.c`, `FUN_0059b690.c`, `FUN_0059b730.c`, `FUN_0059b430.c`,
`FUN_0059ca90.c`.
