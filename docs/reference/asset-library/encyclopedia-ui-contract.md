---
title: "Original Encyclopedia UI Contract"
description: "Source-backed construction, controls, callers, interaction, and remaining A0 capture matrix for the original encyclopedia"
category: reference
created: 2026-09-29
updated: 2026-09-29
tags: [encyclopedia, ui, reverse-engineering, parity, provenance]
---

# Original Encyclopedia UI Contract

This contract records the original encyclopedia surface independently of the
port's eventual catalog and renderer. It is bounded to the inspected English
`REBEXE.EXE` whose SHA-256 is
`b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`.
The semantic identity, availability, ordering, context, navigation, faction,
and system-selector rules remain defined by the
[source contract](encyclopedia-source-contract.md#semantic-research-checkpoint).
This document adds the source-proven window and input contract; it does not
change those rules or claim runtime visual acceptance.

## Evidence boundary

The evidence grades used here are deliberately separate:

- **Static source proof:** connected calls in the checked-in decompilation
  notes plus focused disassembly of missing bodies at `0x0041d6b0`,
  `0x00438800`, `0x004443a0`, `0x0045da70`, and `0x00467f10` from the
  identified executable. Revision 2 also checks the missing predecessor helper
  at `0x005f5c60` and background selector at `0x006075e0` directly.
- **Resource proof:** `strategy-dll` entries in
  `resource-inventory.json`, joined to the exact resource IDs passed by
  `FUN_0045ddc0`.
- **A3 visual leads:** Steam-guide captures 018, 019, and 027 and the French
  Abandonware capture. These corroborate composition only; they do not supply
  exact geometry, control state, English text, or strict-parity proof.
- **A0 proof:** not yet captured. Every matrix row below therefore remains
  `static-proven-runtime-pending` or an explicit source gap. E51 owns original
  executable capture; this task does not substitute third-party images for it.

All coordinates below are integer pixels relative to the encyclopedia client.
`FUN_00429f30` passes `470 × 330` (`0x1d6 × 0x14a`) to
`FUN_0045d400`/`FUN_00606380`. `FUN_0045ddc0` sizes and positions children in
that client coordinate space. The shell supplies placement through its
`+0xcc..+0xd8` rectangle to `FUN_00606980`; no fixed desktop origin is claimed.
The primary faction backgrounds are 470 × 331 resources, so their stored
bitmap height is not a second client-height claim.

## Construction and callers

### Context-free routes

`FUN_00422ce0` is the proven shell caller for both context-free entry paths:

| Trigger | Connected flow | Exact behavior |
|---|---|---|
| `WM_KEYDOWN`, virtual key `0x76` (F7) | `FUN_00422ce0` → `FUN_00429f30` | Builds empty class/entity wrappers and requests the encyclopedia directly. |
| `WM_COMMAND` `0x131` | `FUN_00422ce0` → `FUN_00429f30` | Builds the same empty wrappers, but does not open while `FUN_004fcee0() < 2`. |

`FUN_00429f30` first posts message `0x409` to the shell, then searches the
shell child collection at `+0x6c` for type/command `0x19`. It constructs only
when none exists, through `FUN_0045d400(..., 470, 330, shell, 0x19, contexts)`;
the new child is registered and shown through vtable slot `+0x2c` with argument
`5`. Repeating either route therefore does not construct a second encyclopedia
child in the same shell.

### Contextual routes

The exact trampoline at `0x0041d6b0` calls `FUN_00422ca0` to obtain the active
shell and, when non-null, forwards both typed context wrappers to
`FUN_00429f30` at call address `0x0041d6c5`. An exhaustive direct `call rel32`
cross-reference scan of the inspected executable proves five callers:

1. The unrenamed handler at `0x00438800`, command `0x67`, reads the selected
   row from the control at `this + 0x114`, takes its typed identity at row
   `+0x54`, constructs an empty companion wrapper, and calls the trampoline at
   `0x004388ff`.
2. The unrenamed handler at `0x004443a0`, command `0x66`, rejects an empty
   identity and an identity whose high byte is `0xf1`, closes its current
   surface, and calls the trampoline at `0x0044445d` with the retained context
   at `this + 0x128` and an empty companion wrapper. Command `0x65` only closes.
3. The unrenamed handler at `0x00467f10`, commands `0x67` and `0x97`, chooses a
   current child according to mode at `this + 0x118`; only a non-null child
   whose virtual `+0x10` result is `4` reaches `0x0046884e`. That branch copies
   the child context at `+0x60` and calls the trampoline at `0x0046886a` with
   the already-prepared companion wrapper.
4. `FUN_0046c3c0`, command `0x67`, reads the selected row from the mission
   dialog list at `this + 0x128`, copies its identity into a context wrapper,
   and calls `FUN_0041d6b0`.
5. `FUN_00486fb0`, event `0x100`, reads the first row from the caller-supplied
   list/collection, extracts its typed identity through `FUN_00403040`, and
   calls `FUN_0041d6b0`. Its sibling event `0x103` calls `FUN_0041d7f0`
   instead and is not an encyclopedia route.

The same scan finds only the trampoline plus the two shell sites at
`0x00423667` and `0x00423b5a` calling `FUN_00429f30` directly. This bounds the
direct-call graph; it does not claim that an unresolved indirect function
pointer has a friendly surface name. The visible owning surface for the three
unrenamed handlers and every possible dispatch into `FUN_00486fb0` is not
renamed from proximity alone. A0 capture must retain the originating control,
command/event, and typed identity for each route.

`FUN_0045d400` resolves the supplied class/entity context to the canonical
cached topic described by the source contract. A resolved current row selects
topic mode; empty or unresolved context selects index mode. No raw identity is
fabricated into a topic.

## Client composition

`FUN_0045ddc0` composes the client from `STRATEGY.DLL` resources. The two
faction paths use the same content rectangles but different shell and rail
chrome where shown below.

| Element | Rectangle `(x,y,w,h)` | Command / role | Source/resource evidence |
|---|---:|---|---|
| Client shell | `(0,0,470,330)` | Faction chrome | Alliance `0x285f`; Empire `0x2860`; both 470 × 331. |
| Topic inner overlay | `(12,14,400,306)` | Background selector `1`; same mode resource for both factions | `0x2861`, 400 × 306. |
| Index inner overlay | `(12,13,400,306)` | Background selector `2`; same mode resource for both factions | `0x2862`, 400 × 306. |
| Index baked static text | Origin `(36,48)`; extent measured from a zero-size rectangle | Drawn into background selector `2`, not a child control | `FUN_00601700` constructs text resource `0x1843`; `FUN_00601b80` measures it through `DrawTextA` flag `0x400`; `FUN_00601ce0` renders it into the index composite before registration. Its semantic label is not inferred. |
| Right rail | `(412,0,58,330)` | Faction rail chrome | Alliance `0x2959`; Empire `0x295d`. |
| Return/close | `(423,25,32,31)` Alliance; `(426,21,44,41)` Empire | `0xfb` | Alliance normal/pressed `0x2882/0x2883`; Empire `0x2888/0x2889`; layout identity `0x1954`. |
| Header/title static text | Positioned `(36,14)` | static ID overwritten to `0x15e` | Constructed from `0x1842`; reset to that resource in index mode and populated from current row `+0x14` in topic mode. |
| Selected-category static text | Positioned `(40,119)` | static ID overwritten to `0x11b` | Constructed from `0x1850`; `FUN_0045f100` replaces its text from the selected category control. |
| Mode tab container | `(423,93,32,116)` Alliance; `(426,89,44,136)` Empire | `0x66` | Holds topic/index buttons. |
| Category tab container | `(36,78,361,41)` | `0x6e` | Holds the seven category buttons. |
| Index selection field | `(143,45,245,18)` | field command set to `0x64` | `CoolStringField`, white text; selected list row `+0x14`; hidden in topic mode. |
| Index list | `(36,137,350,160)` | `0x65` | `CoolDragList`; row-height argument `20`, font/resource `0x299d`. |
| Topic body | `(17,231,395,80)` | read-only text field | `TextScrollField`; font/resource `0x299d`. |
| Forward | `(380,14,21,17)` | `0x84` | normal/pressed/disabled `0x288e/0x288f/0x2890`; layout `0x1857`. |
| Backward | `(28,14,21,17)` | `0x83` | normal/pressed/disabled `0x2891/0x2892/0x2893`; layout `0x1858`. |
| Topic art | blit origin `(12,31)` | current image | `FUN_0045f090` draws the source image without a resize request; owned first-profile images are 400 × 200, but that measurement is not a universal UI assumption. |

`FUN_0045ddc0` first chooses one faction shell and rail, then makes two
composites from that same chosen pair. It blits `0x2861` at `(12,14)` into the
first composite and registers it as background `1`. It blits `0x2862` at
`(12,13)` into the second composite, creates the `0x1843` static text object at
`(36,48)` with zero initial extent, measures it through `FUN_00601b80`, renders
that text through `DrawTextA` into the bitmap, and registers the finished
composite as background `2`. `FUN_006075e0` resolves a registered
background selector and makes it current. `FUN_0045f480(1)` selects background
`2` for index mode; `FUN_0045f480(2)` selects background `1` for topic mode.
Index background selector `2` and topic background selector `1` apply to both
factions.

The overlays are therefore mode-specific, while the underlying shell and rail
remain faction-specific. Resource `0x1843` is proven to be baked static text;
this contract does not guess its semantic meaning from its position or pixels.

The header object and navigation controls remain constructed in both modes;
the index selection field is hidden in topic mode. `FUN_0045f480` switches
child visibility and focus rather than constructing a different window.

### Mode buttons

| Mode | Command | Relative rectangle | Alliance normal/active | Empire normal/active | Behavior |
|---|---:|---:|---|---|---|
| Topic | `0x67` | `(0,0,32,31)` / `(0,0,44,41)` | `0x2886/0x2887` | `0x288c/0x288d` | Selects topic mode only when a current row exists; otherwise the handler reselects index. |
| Index | `0x68` | `(0,54,32,31)` / `(0,54,44,41)` | `0x2884/0x2885` | `0x288a/0x288b` | Selects index mode and focuses the list. |

The selected-state bitmap is copied from each button's pressed resource by
`FUN_0060d700`. `FUN_0045ddc0` also assigns that same active resource as the
mode button's disabled-state image.

### Category controls

The category labels remain unjoined to source strings. Stable UI identities are
the command, rectangle, source-identity range, and resources; category names
must not be guessed from icon appearance.

| Slot | Command | Rectangle | Source range / role | Alliance normal/active | Empire normal/active |
|---:|---:|---:|---|---|---|
| 0 | `0x6f` | `(36,78,49,41)` | full master index | `0x2864/0x2863` | same |
| 1 | `0x70` | `(88,78,49,41)` | `[0x90,0x98)` | `0x286e/0x286d` | same |
| 2 | `0x71` | `(140,78,49,41)` | `[0x14,0x20)` | `0x286c/0x286b` | `0x2878/0x2877` |
| 3 | `0x72` | `(192,78,49,41)` | `[0x20,0x30)` over admitted master rows | `0x2868/0x2867` | `0x2874/0x2873` |
| 4 | `0x73` | `(244,78,49,41)` | `[0x40,0x80)` | `0x2d60/0x2d5f` | `0x2d62/0x2d61` |
| 5 | `0x74` | `(296,78,49,41)` | `[0x10,0x14)` | `0x2870/0x286f` | `0x287a/0x2879` |
| 6 | `0x75` | `(348,78,49,41)` | `[0x30,0x40)` | `0x286a/0x2869` | `0x2876/0x2875` |

`FUN_0045f100` supplies the exact filtering and early-return semantics. In
topic mode a changed, non-forced category request is ignored. Selecting a
category in index mode binds or rebuilds the appropriate sorted collection,
updates the index label from the selected category control, and leaves focus in
the list.

## Button state contract

The shared `CoolStrobeButton` path is connected, not inferred from the bitmap
pairs:

| State | Source behavior | Bitmap slot |
|---|---|---|
| Normal | No pressed, disabled, or tab-selected flag. | constructor first resource at `+0x94`. |
| Hover | `FUN_006028c0` has no separate mouse-move/hover resource transition. | normal bitmap remains selected; an A0 capture must still check cursor/strobe timing outside this bitmap selector. |
| Pressed | Button hit-test succeeds, mouse down sets flag `0x1`, captures the pointer, and release inside posts `WM_COMMAND`; leaving the hit area cancels visual activation. | constructor second resource at `+0x98`. |
| Disabled | vtable disable path at `0x006028a0` sets flag `0x2`; `FUN_00602d30` selects state `2`. | `+0x9c`, falling back to normal if absent. Forward/back have dedicated disabled images. |
| Selected tab | `FUN_0060d7e0` clears flag `0x4` on the old tab and sets it on the new tab. | `+0xa0`, initialized by `FUN_0060d700` from the pressed resource. |

Mouse press capture and cancellation are implemented at `0x006028c0`; state
flags are set/cleared by `0x006030c0`/`0x006030f0`, and painting is selected at
`0x00602d30`. This proves the bitmap-state mapping. It does not prove a separate
animated hover effect, and none is admitted by this contract.

## Mode, focus, keyboard, and scrolling

### Index mode

- The list, category tabs, and index-side static field are shown; the body and
  directional buttons are hidden. `FUN_0045f480(1)` selects the index mode tab.
- Category selection and switching to index mode both focus the list window.
- A new single-click selection sends notification `0x29b`; a repeat click may
  send `0x29d`. Neither opens the topic. A double-click sends `0x309`.
- With list focus, Return (`VK_RETURN`, `0x0d`) marks the selected row and sends
  the same `0x309` notification. `FUN_0045da70` then selects mode command
  `0x67`, which opens topic mode for that current row.
- Left (`0x25`) and Right (`0x27`) are category navigation in index mode, not
  topic navigation. `FUN_0045fe60` reads the selected category child from the
  category container at `this + 0x124`, recovers its ordered-tree node, then
  uses `FUN_005f5c60` for the predecessor on Left or the node's threaded
  successor at `+0x10` on Right. It skips non-null candidates whose child
  windows are not visible and selects the first visible candidate through
  `FUN_0060d7e0(..., command, 1)`, which updates the selected state and sends
  `WM_COMMAND` to the encyclopedia handler.
- The index edge behavior is asymmetric and must not be replaced with the
  topic endpoint rule. If the *first* predecessor/successor lookup is null,
  both keys fall back through `FUN_005f5060` to the tree's leftmost child:
  Left on the first child reselects that first child, while Right on the last
  child wraps to the first. That immediate fallback does not run the visibility
  test. If a non-null candidate is hidden, traversal continues in the requested
  direction; reaching null only after skipping hidden candidates retains the
  current category and does not invoke the leftmost fallback.
- `FUN_0045fe60` and `FUN_0060d7e0` do not transfer focus themselves. Their
  synchronous `WM_COMMAND` reaches the category branch at `0x0045dad0`, which
  rebuilds/binds the list and explicitly calls `SetFocus` on the list window at
  `this + 0x12c`. Category keyboard selection therefore finishes with list
  focus.
- Up/Down move by one linked row; Page Up/Page Down move by the visible-row
  count derived from list height / row height; Home/End select first/last.
  Reaching either end retains the endpoint. The list does not wrap.

### Topic mode

- The list, category tabs, and index-side field are hidden. The header, body,
  image region, Backward, and Forward are shown, and `FUN_0045f480(2)` focuses
  the body window.
- Left/Right in `FUN_0045fe60` invoke commands `0x83`/`0x84`. The handler at
  `0x0045da70` follows the skip-disabled row links, retains the current row on a
  null endpoint, refreshes body/title/art for a non-null neighbor, and returns
  focus to the body.
- `FUN_0045fd20` enables each direction only when its corresponding neighbor is
  non-null. The endpoint is therefore both non-wrapping and visibly disabled.
- The body is read-only. Up/Down and Page Up/Page Down scroll the body;
  Left/Right escape the body to the encyclopedia handler and become topic
  navigation.
- Return in the read-only body is forwarded to the encyclopedia handler, whose
  default path forwards it to the shell parent. No encyclopedia-local Return
  action is defined in topic mode. Tab (`0x09`) is consumed by
  `FUN_0045fe60`; Escape (`0x1b`) invokes the encyclopedia close method.
- Command `0xfb` also invokes the close method (vtable `+0x30`).

`FUN_0041fd00` calculates body extent with `DrawTextA` flags `0x2410`, including
word breaking and calculated height. When text exceeds the 395 × 80 viewport it
enables the attached vertical scrollbar and recalculates with the reduced text
width. `FUN_0041fe70` respects line feed and tab boundaries, wraps at spaces,
and splits an overlong run to the available width. Exact glyph breaks depend on
the original font resource `0x299d` and remain an A0 pixel-comparison gate.

## Semantic rules as UI actions

The UI consumes the source contract without adding a second interpretation:

| Semantic result | UI state/action |
|---|---|
| Context-free open | Index mode, full-cache selector `0x6f`, list focus, no fabricated current topic. |
| Resolved class/entity context | Topic mode on the canonical cached row; typed source identity is routing input, not the topic key. |
| Unresolved/stale context | Index mode. No raw resource or static asset creates an otherwise unavailable row. |
| Category selection | Commands `0x6f..0x75` bind/filter the retained master cache in the proven fixed order. |
| Viewer-faction selector | Applicable topic identity remains fixed while the proven `+0x1000` or `+0x2000` image key is selected. |
| System picture selector | The proven 1–26 selector chooses key `0x2b5c..0x2b75`; it is not faction arithmetic. |
| System ancestry exclusion | An excluded type-`0xf2` ancestry view never enters the list and cannot be opened contextually. No friendlier gameplay label is inferred. |
| First/middle/last topic | Backward/Forward state follows the skip-disabled links; endpoints disable and never wrap. |
| Missing art object | `FUN_0045f090` omits the EData blit when current art at `+0x14c` is null; header/body mode remains separately constructed. The exact original no-art pixels need A0. |

### No live-stat binding recovered

The original constructor creates only the header/index fields, list, text body,
image region, tabs, and navigation/close controls above. `FUN_0045fa60` obtains
the current row title from row `+0x14`, loads the type-10 `ENCYTEXT.DLL`
resource named by the row identity, copies that narrow string into the body,
and selects the art. Its only connected world-derived display lookup is the
system picture selector used for art. There is no constructed stat-row control
and no connected lookup of current maintenance, capacity, shield, weapon,
character, or facility values. `FUN_0045fa60` writes row `+0x14` into the
header object at `this + 0x120`; the separate `CoolStringField` is the
index-selection field and is hidden in topic mode.

Accordingly, values that look like ship statistics in A3 guide capture 027 are
treated as authored body text for this contract, not live fields. If future
source tracing or an A0 controlled-state capture shows a dynamic substitution,
the source/schema review must reopen before a consumer adds a live binding.

## Finite A0 capture matrix

The matrix is finite but intentionally does not claim that source proof is a
visual pass. Each capture must be lossless at native 640 × 480, retain faction,
entry route, typed identity/current key, selected command, and original
resource identities, and compare exact client bounds and state pixels.

| ID | Required state(s) | Static expectation | Missing A0 proof / next action |
|---|---|---|---|
| `ENC-UI-01` | Alliance context-free index | `0x6f` full cache; Alliance shell/rail; shared index overlay `0x2862`, baked text resource `0x1843`, background selector `2`; list focus. | F7 and `0x131` captures, including command gate and duplicate-open behavior. |
| `ENC-UI-02` | Empire context-free index | Empire shell/rail with the same index overlay `0x2862`, baked text resource `0x1843`, and background selector `2`. | Repeat both entry paths for Empire. |
| `ENC-UI-03` | Alliance commands `0x6f..0x75` | Seven fixed x slots; proven full/range collections and stable ordering. | Capture every selected icon, list membership, label, and empty/gap result. |
| `ENC-UI-04` | Empire commands `0x6f..0x75` | Same command/range contract with faction-specific icon resources where mapped. | Capture every selected icon and collection; join localized labels only from evidence. |
| `ENC-UI-05` | Normal, pointer-over, captured press, canceled press, selected, disabled | Source bitmap slots and capture behavior above. | Capture each state for close, mode, category, and navigation controls in both factions; confirm no distinct hover bitmap/animation. |
| `ENC-UI-06` | Index selection by click, double-click, and Return | `0x29b` selects; `0x309` enters topic; list retains endpoint without wrapping. | Record notifications, focus, and selected-row pixels. |
| `ENC-UI-07` | Class-context topic across all five direct contextual callers | Canonical definition-derived row opens; body focus. | Retain caller address/surface, command/event, wrapper identity, definition `+0x30`, and current key. |
| `ENC-UI-08` | Direct entity-context topic across all five direct contextual callers | Same canonical-row topic flow. | Retain caller address/surface, command/event, entity, definition, selected key, and caller control. |
| `ENC-UI-09` | Special-range entity fallback and non-special miss | Proven fallback may resolve; non-special retry cannot invent another key. | Capture both special association branches and a non-special miss. |
| `ENC-UI-10` | Unavailable/stale context | Index mode, no fabricated current topic. | Capture title/list/mode/focus state after failed direct and fallback resolution. |
| `ENC-UI-11` | Same applicable topic as Alliance and Empire | Text identity fixed; both use topic background selector `1` and shared overlay `0x2861`, while faction shell/rail and the proven faction art key differ. | Paired capture with composite and selected image identities. |
| `ENC-UI-12` | Representative system selectors 1, 24, 25, 26 | Keys `0x2b5c`, `0x2b75`, `0x2b73`, `0x2b74`; text identity fixed. | Capture terminal non-sequential cases and both viewer sides where admitted. |
| `ENC-UI-13` | System side-view admitted/excluded | Exact type-`0xf2` ancestry predicate controls list presence. | Retain selected view and every tested ancestor identity; do not infer a gameplay label. |
| `ENC-UI-14` | First enabled topic | Backward disabled; invocation retains current. | Pointer and Left-key capture in full and one filtered collection. |
| `ENC-UI-15` | Middle topic and disabled intermediate row | Both directions enabled; navigation skips disabled row. | Requires a connected disabled-row writer or controlled observation before capture. |
| `ENC-UI-16` | Last enabled topic | Forward disabled; invocation retains current. | Pointer and Right-key capture in full and one filtered collection. |
| `ENC-UI-17` | Proven current topic with no usable art object | No EData blit; header/body remain independently populated. | Identify a legitimate source trigger, then capture; do not manufacture a catalog mapping. |
| `ENC-UI-18` | Short, wrapped, explicit-newline, long-token, and scrolling body | Read-only `0x2410` layout, scrollbar only when needed; Up/Down/Page keys scroll. | Pixel compare with original font `0x299d`; record scrollbar extent and focus. |
| `ENC-UI-19` | Topic Return, Tab, Escape, and `0xfb` | Return has no local topic action; Tab consumed; Escape/`0xfb` close. | Capture shell-visible result and focus/close routing. |
| `ENC-UI-20` | Index Left from a middle category, the first category, and across hidden predecessor candidates | Select first visible predecessor; immediate null reselects the tree's first child; running off the start only after hidden candidates retains current; list focus remains. | Capture command/state/list changes. A hidden-category case requires a connected original visibility configuration; do not fabricate one. |
| `ENC-UI-21` | Index Right from a middle category, the last category, and across hidden successor candidates | Select first visible successor; immediate null at the last child wraps to the tree's first child; running off the end only after hidden candidates retains current; list focus remains. | Capture command/state/list changes. If a connected configuration can hide the leftmost child, separately test the source's unfiltered immediate fallback. |

Guide 018 leads the index composition, guide 019 leads a character topic, and
guide 027 leads a ship topic with stat-like authored text. Their SHA-256 values
are, respectively, `0769ec7407f75133d1f85460d995ab976f0039540aec85b676381f131a0f3be8`,
`d74940b352a5e7102e0549e727c24daf70bad3593df72c85c847cbe4bce8c6c7`,
and `1c7c7264f3b57a2d4875671c0fedf7bbde21e2ed88692f7f4cd83c1796a86e78`.
They are A3 and cannot close any matrix row.

## Explicit gaps and change control

- No `ENC-UI-*` row has an A0 capture. E51 must run them against the identified
  unmodified executable and retain screenshots, inputs, focus, and resource
  provenance.
- The seven localized category labels are not joined to command IDs. Use the
  stable commands/ranges until a connected string/resource trace proves names.
- Five direct callers of `FUN_0041d6b0` are source-proven, but the visible
  originating control/surface for handlers `0x00438800`, `0x004443a0`, and
  `0x00467f10`, and for every dispatch into generic `FUN_00486fb0`, has not
  been named. Capture the caller address and control rather than assigning a
  name from neighboring code.
- The button path proves no separate hover bitmap; original cursor/strobe
  timing and exact pressed/disabled pixels still require A0.
- No legitimate null-art topic trigger is yet connected. Preserve the source
  paint branch and do not create a fake no-art catalog entry to satisfy it.
- Exact line breaks and scrollbar pixels depend on the original font/resource
  metrics and require A0 even though the wrap/scroll algorithm is static-proven.
- The category keyboard path supports skipping hidden child windows, but no
  original encyclopedia state that hides a category has yet been connected.
  E51 should exercise hidden-child branches only after recovering such a
  configuration; it must not manufacture hidden controls as original proof.
- Unproven alternate artwork, including `EDATA.192`, remains inventoried and
  unused under deferred task `orlocal-2kq`. This contract defines no alternate
  predicate, binding, switch, or acceptance row.

If an A0 presentation contradicts the frozen identity, availability, context,
navigation, faction, or system-selector semantics, reopen the source/schema
review before changing a renderer or catalog consumer. A screenshot alone must
not replace the connected source rule.
