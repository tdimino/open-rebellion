---
title: "Strategic Windowing Toolkit Contract"
description: "Original strategic-window identity, registry, stacking, modality, focus, capture, keyboard routing, close, and restoration behavior"
category: "ghidra"
created: 2026-10-10
updated: 2026-10-14
---

# Strategic Windowing Toolkit Contract

This note records the behavioral contract needed by the planned
`rebellion-windowing` crate. It distinguishes the original game's shared
window machinery from its feature-specific painting and contents. The result
is sufficient to shape a narrow, reversible identity, ordering, focus,
capture, and per-dispatch routing API without copying the Win32 implementation.
The API-shaping Wine queue is complete; Battle Alert and Battle Results retain
explicitly provisional visible policy because neither occurred in the
available saves.

The executable studied is the owned `REBEXE.EXE` with SHA-256
`b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`.
Addresses below are for that build. Proprietary assets and original-runtime
captures are not committed. The repository-visible
[`strategic-windowing-runtime-evidence.md`](strategic-windowing-runtime-evidence.md)
records the retained observations, artifact hashes, and bounded runtime limits.

The original manual's windowing discussion on printed pages 63--64 supplies
the terms *modeless* and *modal* and names Status, Finder, Battle Summary,
Encyclopedia, and Message as the minimum modal set. The governing port scope
is [`../../docs/plans/2026-10-10-feat-strategic-window-modality-and-focus.md`](../../docs/plans/2026-10-10-feat-strategic-window-modality-and-focus.md);
audit authority remains
[`../../docs/qa/2026-09-10-interface-parity-audit/2026-10-06-manual-window-checklists.md`](../../docs/qa/2026-09-10-interface-parity-audit/2026-10-06-manual-window-checklists.md).

## Evidence notation

- **fact:** direct decompile, instruction, manual, or live-state observation.
- **runtime:** repeatable visible behavior in the original executable under
  Wine 9.0.
- **port:** an explicit Open Rebellion design decision supported by the
  recovered visible contract, but not a claim that the native code uses the
  same mechanism.
- **unknown:** bounded native detail that remains unresolved.

The compact instruction evidence is in
[`strategic-windowing-toolkit.instructions.txt`](strategic-windowing-toolkit.instructions.txt).
Fresh bounded Ghidra decompiles of the small shared helpers are committed as
`FUN_00600280.c`, `FUN_00601080.c`, `FUN_00601090.c`, `FUN_006011f0.c`,
`FUN_00601320.c`, `FUN_00601340.c`, `FUN_004fcee0.c`, `FUN_006068f0.c`,
`FUN_006044d0.c`, and `FUN_00606960.c`. The three short tail-entry helpers at
`0x00606940`, `0x00607d00`, and `0x00607d30` were not recognized as function
entries by the clean import; their complete bytes are preserved in the
instruction appendix instead of fabricating decompiler boundaries. That
appendix also retains the complete `FUN_005ffe40`, `FUN_005ffe60`, and
`FUN_005ffe80` display and enable-state wrappers cited by the shared vtable.
Larger constructors and dispatchers were inspected in the existing corpus and
with targeted `objdump` ranges rather than copied into this note.

The detail-owner adjudication below additionally cites the committed
`FUN_00429ce0.c`, `FUN_004591d0.c`, `FUN_0045aac0.c`, `FUN_00441190.c`, four
receiving detail constructors (`FUN_00452fc0.c`, `FUN_004a7790.c`,
`FUN_004a2630.c`, and `FUN_0049f130.c`), and their common `FUN_004ac120.c`,
`FUN_00606380.c`, and `FUN_005ff910.c` base-constructor chain.

## Conclusions that constrain the port

1. **Identity is semantic, not HWND identity.** A strategic top-level object
   has a stable type key at `+0x24`. Multi-instance detail windows combine an
   object identity with a kind; singleton dialogs use a canonical type key.
2. **Existence, presentation history, focus, and capture are different
   authorities.** The registry at strategic-view `+0x6c` is keyed lookup, not
   z-order. Presentation records at `+0x2e0`, their order at `+0x428`, and a
   separate registry at `+0x478` implement the visible reference rail/MRU.
   Native focus remains Win32 focus. Pointer capture is temporary interaction
   state.
3. **Modality is an input boundary, not a paint band.** The documented modal
   families carry policy bit 0 at object `+0xb8`, and that bit suppresses the
   strategic F1--F7 path. Live Encyclopedia checks show that its enabled
   underlay does not receive normal pointer input, while no modal holds
   permanent capture. Live Status, Build Selection, Create Mission, and Move
   Confirmation checks establish the same whole-strategic-view boundary for
   those four bit-0 families. Bit 0 alone still does not prove the pointer
   scope of an unobserved family.
4. **Opening an existing window is family-specific.** Finder, Message, and
   Encyclopedia singleton openers return without raising. Existing detail
   windows are restored/raised. A duplicate Sector opener returns its object
   without itself raising it; a caller may subsequently raise/select a detail
   presentation. The port must declare this policy per window kind.
5. **Closing is asynchronous at the shared boundary.** The child posts message
   `0x405` to its owner. The owner removes and destroys the keyed child, then
   posts `0x467` to choose, expose, and focus the next eligible surface. Input
   eligibility must therefore be snapshotted for one physical input dispatch
   so the dismissing key or click cannot fall through.
6. **Keyboard ownership follows focused children.** Edit and list controls own
   their handled keys. Unhandled `WM_KEYDOWN` can forward to the parent
   top-level, and the strategic view applies its modal/function-key rule.
7. **The contract is faction-neutral.** Alliance and Empire use different art
   and command targets, but the examined classes, keys, registry, close,
   keyboard, and modal machinery are shared. Both factions corroborated the
   key visible behaviors under Wine.

## Common object and native-window layout

`FUN_005ff910` constructs the common native window/control base.
`FUN_00606380` extends it as the shared game-window object, installing vtable
`0x0066e060`; `FUN_004ac120` is the `CustomDialogBox`-like derived base with
vtable `0x0065bf80`. Names here describe behavior, not recovered RTTI.

| Offset | Meaning | Evidence and confidence |
|---:|---|---|
| `+0x18` | Native `HWND` | Passed to `DestroyWindow`, focus, enable, show, position, and message APIs; high |
| `+0x20` | Parent/owner C++ object | Close and unhandled-key paths dereference its `+0x18` HWND or virtual dispatcher; high |
| `+0x24` | Semantic child/type key | Ordered-tree lookup/removal and `0x405` close payload; high |
| `+0x28/+0x2c` | Initial x/y | `CreateWindowExA` inputs; high |
| `+0x30/+0x34` | Initial width/height | `CreateWindowExA` inputs; high |
| `+0x38/+0x3c` | Runtime position/size state | Updated by the base dispatcher; medium-high |
| `+0x40` | Base behavior flags | Used by common message handling; medium |
| `+0x48/+0x4c` | Class/title strings | Registration/creation inputs; high |
| `+0x50/+0x54` | Style/ex-style | `CreateWindowExA` inputs; high |
| `+0x68` | Native child id/menu value | Window-creation input; high |
| `+0x6c` | Root of keyed child tree | Insert/lookup/remove helpers; high |
| `+0x74/+0x88` | Other common intrusive trees | Shared notification/helper registries, not strategic z-order; medium-high |
| `+0xb8` | Game-window policy bits | Bit 0 dialog/modal-key policy; bit 4 movable title drag; high for those bits |
| `+0xbc` | Previous/related presentation pointer | Used with active presentation state; medium |
| `+0x10c` | Saved focus HWND | `FUN_006073b0` stores the `SetFocus(+0x18)` result here; `FUN_00606580` restores it; high |
| `+0x110` | Minimized/hidden presentation state | Toggled by `FUN_00607d00`/`FUN_00607d30`; high |

`FUN_005ffce0` registers the class if necessary and calls `CreateWindowExA`.
`FUN_00600310` associates the object with window-long slot 0 and forwards
native messages to virtual slot `+0x14`; it also records move, size, and
destruction state. The relevant shared game-window vtable slots are:

| Slot | Target | Contract |
|---:|---|---|
| `+0x04` | `FUN_005ffe40` | Show/hide through `ShowWindow` |
| `+0x0c/+0x10` | `FUN_005ffe60` / `FUN_005ffe80` | Enable/disable wrappers |
| `+0x14` | `FUN_00606650` | Derived/native message dispatcher |
| `+0x2c` | `FUN_00607d00` | Restore/show a minimized presentation |
| `+0x30` | `FUN_00606960` | Request asynchronous keyed close |

## Four separate authorities

### 1. Keyed child registry

The strategic view's `+0x6c` field is a threaded ordered tree keyed by the
child object's `+0x24` value. Node links appear at `+0x04` left, `+0x08`
right, `+0x0c` parent, and `+0x10` successor/thread. `FUN_005f4f10` inserts,
`FUN_00604500` looks up by key, and `FUN_005f4fa0` removes. Iteration order is
key order. It is therefore an existence/identity registry, **not** front-to-
back order.

`FUN_00600f90` looks up the keyed child, removes the node, invokes the
`FUN_00600280` `DestroyWindow` wrapper, and destroys the C++ object. A second
global tree at application object `+0x88`, populated by `FUN_00601260` and
cleared by `FUN_006011f0`, maps every native HWND. That global native registry
is also not strategic z-order.

### 2. Presentation rail and MRU

The strategic view owns a separate presentation registry at `+0x478` whose
nodes have a key at `+0x18` (`FUN_0042ac70`). `FUN_00428b40` maintains at most
12 records of size `0x1c` at `+0x2e0`, an order array at `+0x428`, and count at
`+0x458`; a thirteenth entry evicts the oldest. `FUN_00429020(view, 1..12)`
restores the chosen record, removes its presentation entry, and sends `0x467`
to recalculate active focus/presentation.

This is the source of the reference rail/minimized-window history. It must not
be conflated with the complete open-window stack or the modal boundary.

### 3. Native focus

The main `FUN_00422ce0` handler's `0x467` path chooses an eligible strategic
child, uses `ShowWindow` and `BringWindowToTop`, and calls `SetFocus` on the
chosen child or the strategic view. It tracks active/previous presentation
objects at strategic-view `+0xb8/+0xbc` (distinct from each child object's own
policy word at `+0xb8`). If the current object is absent it walks registered
native siblings to find an eligible replacement.

The base destructor `FUN_005ffb60` removes the object from helper/global
registries and tears down its trees. `FUN_00606580` restores the HWND saved at
`+0x10c`. `FUN_00606380` initializes the field to zero and `FUN_006073b0`
fills it with the window that held focus before `SetFocus(+0x18)`. It is not a
port-wide focus stack.

Menus have their own explicit focus rule: `FUN_00442380` stores the prior
focus in `DAT_006b28fc` and focuses the menu; `FUN_00442430` destroys the menu
and restores that HWND only if focus remained in the menu/submenu family;
`FUN_004424c0` tracks submenu focus.

### 4. Pointer capture

`FUN_00601080` and `FUN_00601090` are `SetCapture` and `ReleaseCapture`
wrappers. A movable modeless title drag at `FUN_00606ee0` requires policy bit
4 and no foreign capture, then temporarily captures. The base destructor
restores capture to the next registered capture helper, if any.

`FUN_00601340` checks `GetCapture`, falls back through `FUN_00601320` to the
active registered HWND, and sends private message `0x40d` asking that owner to
release/cancel capture. The strategic `0x468/0x482` dialog-opening path loops
the application pump until this helper reports no capture, then briefly
captures the galaxy during construction and releases it. No modal constructor
holds capture for the modal's lifetime.

### Window drag

The movable bit is `+0xb8 & 4`. The System (`FUN_00452fc0`), Fleet
(`FUN_004a2630`), Defenses (`FUN_004a7790`) and Missions (`FUN_0049f130`)
windows OR it in. A sector window's base gets policy 0 (`FUN_004591d0` →
`FUN_004ac120` → `FUN_00606380`), so sector windows stay in their columns.
Under Wine, a title drag on the Dolomar sector window (Alliance,
alliance-day1169-deathstar-trench-run played on to day 1170) left it in
place.
`FUN_00606380` also sets `+0xf4` to 1, the live mode:
- A left press that reaches the base procedure (`FUN_00606650` 0x201 →
  `FUN_00606ee0`) records the grab offset and captures the pointer.
- Each `WM_MOUSEMOVE` with the drag on (`+0xe0`) calls `SetWindowPos` to put
  the window at the pointer less that offset (`FUN_006071e0`), with no
  clamp.
- The release ends it (`FUN_00607010`). The XOR-outline mode (`+0xf4 & 2`)
  is not used by these windows.

Every left press on the window's own surface reaches the base. The Missions
(`FUN_0049f4b0`), Fleet (`FUN_004a29c0`) and Defenses (`FUN_004a86d0`)
procedures first send or post 0x467 for themselves and then pass it on. The
System procedure (`FUN_004534f0`) passes it on unless it lands on an
overview band. Presses on the child controls (title buttons, tabs, lists)
go to those controls.

Under Wine, on the Empire assassin-combat save at day 416, Carida's System
window moved 80 pixels left and Berchest's 120 right, by the pointer's
travel and without snapping.

port: `system_window::TitleDrag`, `surface_drag_grab` and
`title_drag_step`. A primary press on the window's background widget, under
all of its control widgets (title buttons, tabs, bands, the System and
Defenses lists, list rows), takes hold, as does any press outside the
rectangles of the child controls the port tests by hand: the Fleet window's
two lists and tab strip, and the Missions window's mission list and, while
a mission is selected, its member list. hyp: the Fleet window's
selected-name label and picture panel pass presses to the window.

## Opening, close, and restoration

### Close sequence

`FUN_00606960` posts `(message=0x405, wParam=*(this+0x24), lParam=this)` to the
HWND at `(*(this+0x20)+0x18)`. The generic `FUN_006007b0` `0x405` case calls
`FUN_00600f90` to remove and destroy the keyed child. The strategic
`FUN_00422ce0` path additionally removes the corresponding presentation entry
and posts `0x467` for later restoration. Destruction and replacement focus
therefore occur on subsequent native message dispatches, not by synchronously
returning a new input target from the close callback.

Parent destruction is a cascade, not independent orphan cleanup.
`FUN_005ffb60` tears down the parent's `+0x6c` registry through
`FUN_006044d0 -> FUN_005f4f00 -> 0x005f50a0`; the last helper repeatedly
removes the first registered child and invokes that child's virtual
destructor. The child destructor in turn unregisters native/capture state.
**port:** closing an owner must atomically close its descendants and clear any
descendant focus/capture before choosing a restoration target outside that
subtree.

**port:** route each physical input event against an immutable dispatch
snapshot. A close transition may update manager state, but the event that
closed the surface remains consumed by the old owner. A later independent
event, even when batched into the same render frame, receives a new snapshot.
This reproduces the recovered no-fallthrough outcome without reproducing the
Win32 post queue.

### Detail lifetime owner

The strategic view is the registry/destruction owner of all four
Sector-launched detail families; the Sector that opened them is not their
lifecycle owner. The complete stored-owner trace is:

1. `FUN_00429ce0` runs on the strategic view and passes its `this` pointer as
   constructor parameter 6 to `FUN_004591d0`.
2. `FUN_004591d0` forwards that pointer through `FUN_004ac120` and also stores
   it at Sector `+0x194`.
3. `FUN_0045aac0` runs on the Sector, but uses `*(this + 0x194)` for detail
   lookup in `+0x6c`, creation, presentation registration, keyed insertion,
   MRU restoration, and raising. It passes the same pointer into each receiving
   System/Manufacturing, Defenses, Fleet, or Missions constructor.
4. The strategic pointer arrives as constructor parameter 6 in
   `FUN_00452fc0` and `FUN_004a7790`, and as parameter 4 in the shorter
   `FUN_004a2630` and `FUN_0049f130` signatures. Each normalizes it to
   `FUN_004ac120` parameter 6, then forwards it through
   `FUN_00606380` parameter 6 to `FUN_005ff910` parameter 8.
   `FUN_005ff910` performs the final assignment, storing that parameter at the
   detail object's common owner field `+0x20`.
5. `FUN_00606960` closes a child by dereferencing `+0x20`, taking that object's
   `+0x18` HWND, and posting `0x405` with the child's semantic key and object.
   A detail therefore posts close to the strategic view, not to its opening
   Sector.

**counter-evidence:** `FUN_0045aac0` is a Sector method and uses the Sector
HWND as the source for `MapWindowPoints`; the Sector therefore supplies the
opening gesture and coordinate/navigation context. It does not insert the
detail into Sector `+0x6c`, pass the Sector as the common owner, or receive the
detail's close message. `FUN_00441190` independently constructs the same four
detail families directly from the strategic view, passes that view into the
same constructor slots, and inserts the result into strategic-view `+0x6c`.
The root registry, direct construction path, and stored `+0x20` recipient
outweigh the incidental Sector opener context.

**fact (high static confidence):** a detail is registered under and posts close
to the strategic root; it is absent from the opening Sector's child registry.
The generic Sector-owner cascade therefore cannot reach that detail.

**port:** closing a Sector must not close details opened from its context.
Those details remain independently managed until an explicit detail close,
content reconciliation, or strategic-root teardown. Both faction saves
corroborated this behavior under Wine: closing Sseswenna left open
Manufacturing, Defenses, and Fleet details alive. This remains research
evidence, not port-side or native-Windows acceptance.

### Existing-window policy

| Family | Existing-instance behavior |
|---|---|
| System/Fleet/Troop/Personnel Finder | Singleton opener returns/no-ops; it does not explicitly raise or focus |
| Message Index | Singleton opener returns/no-ops |
| Encyclopedia | Singleton opener returns/no-ops |
| Sector | Same subject key returns the existing object without an explicit raise in the opener; a caller that opens/selects a detail child can then raise that presentation; at most two sectors are retained |
| System/Manufacturing, Fleet, Defenses, Missions detail | Existing subject+kind is restored from MRU when minimized and moved to `HWND_TOP` |
| Status and contextual command dialogs | Contextual creation; no general duplicate-raise rule established |
| Build Selection | Normal strategic input cannot re-enter its opener while it owns the boundary; the direct programmatic opener has no recovered duplicate guard, so the public port API rejects re-entrant open rather than promising native duplicate semantics |

The singleton conclusions come from `FUN_0042a000`, `FUN_0042a0c0`,
`FUN_0042a4d0`, `FUN_0042a180`, `FUN_0042a240`, and `FUN_00429f30`. Sector
behavior comes from `FUN_00429ce0`; detail behavior from `FUN_0045aac0`.
Consequently, a generic `open == raise` rule would be a parity defect.

## Saved windows and a load

A strategic save ends with the view's interface, after the world and the
controllers: `FUN_004095b0` → `FUN_00401250` → `FUN_00415f60` → view slot
`+0x3c` (`FUN_00440ea0`, vtable `0x00658bf8`) writes it, and `FUN_00409480`
→ `FUN_00401270` → `FUN_00415f80`, which rebuilds the strategic view
(`FUN_00414330(app, 8)`), reads it back through slot `+0x40`
(`FUN_00440f80`).

- The record: a u16 with the ending code (`+0xa8`) high and the GID mode
  (`+0xc8`, set by `FUN_00425d00.c:126`) low; a u16 count; then each child
  window from the bottom of the z-order up (`FUN_00440f20`, then
  `GW_HWNDPREV`): a six-u32 descriptor (`FUN_0049ecf0`: type `+0x24 & 0x3f`,
  x, y, w, h from `+0x28..+0x34`, shown `+0x110`) and the class's slot
  `+0x24` payload.
- Payloads: sector (type 1, `FUN_0045d2b0`) its sector and key; Fleet (4,
  `FUN_004a7300`) its system, key, tab `+0x194` and `+0x16c..+0x174`; the
  "Battle in Progress..." overlay (7, `FUN_0049f0f0`) its key; System (9,
  `FUN_00458ea0`), Defenses (10, `FUN_004aa6d0`) and Missions (11,
  `FUN_004a0b00`) their system and key. No page is saved but the Fleet
  window's tab; other types write only a descriptor and are not rebuilt.
- The load: the u16's high byte becomes the ending code, and a GID mode
  other than the view's is replayed as `WM_COMMAND(0x140, mode)`. View
  `+0xc0` bit 4 holds for the restore, so restored windows open silently.
  `FUN_00441190` builds each window in order at the saved place, created
  hidden, inserted into `+0x6c` and (but type 7) `+0x478`. A window saved
  shown, the overlay, and every sector window (its branch clears the rail
  flag, `0x441248` → `0x4413f1`) show through slot `+0x2c(5)`; a hidden one
  goes back on the rail (slot `+0x08`, then `FUN_00428b40`). Every sector
  window and the compact legend `+0x248` then drop to `HWND_BOTTOM`
  (`FUN_00440f80.c:31-38`).
- Under Wine, Empire day-2 start played on to day 6: a System window
  dragged to (322, 92) and switched to Shipyards came back at (322, 92) on
  the overview; a minimized one came back on the rail; the sector window
  came back under the System window. Saved with the GID menu open (day 8):
  the menu did not come back, the System window it hid came back on the
  rail, and the sector window it hid came back shown.
- The focus after a load is on the top restored window that is shown and
  is not a sector window (Wine runs r, t1, t3, u and w1,
  empire-fleet-to-yaga-minor at day 38).

Port: the save's interface section (`rebellion_core::saved_interface`,
`agent_docs/save-load.md`) keeps the GID mode and the Sector, System,
Fleet, Defenses and Missions windows; the port has no type 7 overlay. A
minimized window keeps its z serial on the rail, so `saved_interface`
writes every window in one list by z serial, a hidden one at its own place
and a window the GID menu hides as hidden (`FUN_00440ea0.c:19-24`).
`restore_interface` rebuilds them in that order and appends each hidden one
to the rail as it meets it (`FUN_00428b40`); Fleet windows open through the
window manager, so `finish_interface_restore` gives a hidden one its rail
slot before the manager's minimize, raises every window again in saved
order, and `GalaxyFocusState::after_load` takes the focus. During play a
minimize still appends to the rail; only a load orders it by z.

## Modality and input routing

The manual describes Status, the four Finders, Battle Summary, Encyclopedia,
and Message as modal. Their constructors all set child policy bit 0 in
`+0xb8`. The main strategic `WM_KEYDOWN` path scans the keyed child registry;
if any child has bit 0, it suppresses F1--F7 before routing Game Options,
System/Fleet/Troop/Personnel Finder, Message, or Encyclopedia.

That bit is a useful dialog-policy marker but not a complete modal identity:
Mission creation (type `0x0c`), Move confirmation (`0x10`), Battle Alert
(`0x06`), Build Selection (`0x62`, policy `5`), Battle Results (`0x1c`), and
two other dialog classes (`FUN_0046f140`, `FUN_0049ee20`) also set bit 0.
Meanwhile ordinary `WM_COMMAND` cases do not all recheck the bit.

The modal implementation is **not** any of the following in isolation:

- not a nested `DialogBox`-style message loop: the application continues its
  single `PeekMessage`/preprocess/`TranslateAccelerator`/`TranslateMessage`/
  `DispatchMessage` loop in `FUN_00413560`;
- not disabled-owner modality: no strategic calls to the shared
  enable/disable wrappers were found, and live owner/controls remain enabled;
- not permanent pointer capture: live `GetCapture` was null and the modal
  constructors release their temporary construction capture;
- not an unconditional strategic `WM_COMMAND` gate: a diagnostic direct post
  of System Finder command `0x12d` opened it while Encyclopedia was present.

**runtime:** normal clicks outside Encyclopedia did not dismiss it or activate
known-good underlying System Finder controls for either faction. F2 was also
suppressed. Focused Encyclopedia title input accepted `tallon` and selected
Talon Karrde without firing cockpit hotkeys. Escape closed Encyclopedia; the
next F2/cockpit click then worked. The same outside-click and F2 probes were
repeated for Status (`0x1a`, policy `1`), Build Selection (`0x62`, policy `5`),
Create Mission (`0x0c`, policy `1`), and Move Confirmation (`0x10`, policy
`1`) in both faction saves. In every case the active surface retained the
boundary, Escape dismissed it, and the prior details were exposed only on the
later dispatch.

**unknown:** the exact native pre-command pointer gate was not identified. It
occurs before an enabled underlay control's command reaches `FUN_00422ce0` in
the observed scenarios. The manual, static policy words, and repeated runtime
checks justify a shared strategic boundary for Encyclopedia, Status, Build
Selection, Create Mission, and Move Confirmation. Battle Alert, Battle
Results, the two unnamed bit-0 classes, and any rare owner-local dialog retain
an explicit conservative `port:` whole-view policy until runtime narrows it.
Reproducing an HWND-specific interception hook would be an implementation
accident, not a useful port contract.

## Keyboard ownership and bubbling

The application loop `FUN_00413560` first calls virtual preprocessor
`FUN_005fba70`, then `TranslateAccelerator`, `TranslateMessage`, and
`DispatchMessage`. The preprocessor only rewrites Enter in a focused
`CoolStringField`-like control into private parent message `0x407`; it is not a
global modal filter.

Finder handlers including `FUN_00461590` and `FUN_00463360` establish the
child rules:

- Escape invokes top-level virtual slot `+0x30`, the asynchronous close path;
- left/right traverse eligible controls or adjacent controls;
- up focuses the edit field;
- down focuses the result list;
- keys a child does not consume can reach `FUN_00606940`, which forwards
  `WM_KEYDOWN` to the parent object's virtual dispatcher.

Thus a focused edit/list owns the keystrokes it handles, while unhandled keys
may bubble to its top-level and then the strategic view. The strategic modal
guard still applies to bubbled F1--F7. The routing snapshot must retain the
owner for the whole dispatch so Escape cannot both close a dialog and trigger
an exposed cockpit binding.

## Window-family matrix

`Modeless` means normal strategic input may continue outside the window.
`Strategic modal` means normal input below the active dialog is ineligible.
`Transient child` means lifetime/input derives from an owning top-level rather
than an independent persistent stack entry.

| Surface | Identity / cardinality | Owner / stack / input | Focus / capture | Open, close, and restoration | Side / evidence |
|---|---|---|---|---|---|
| Sector | Key `(system_low16 << 6) \| 1`; max two | Strategic view; modeless lower/detail band; created at `HWND_BOTTOM`, with toolbar placed above | Top-level and owned controls can focus; no family-lifetime capture or movable-title bit recovered | Duplicate returns existing without an opener-side raise; following detail selection may raise; third replaces the sector on the requested galaxy half; generic keyed close restores through `0x467` | Shared class; Alliance two-window runtime and both-faction parent-close runtime; `FUN_00429ce0`; high |
| System / Manufacturing | Detail kind `9`, subject-keyed, many | Strategic-view registry/destruction owner; launched and positioned from Sector context; normal modeless detail/presentation band; presentation-rail capable | Top-level/controls focus; movable bit 4 uses temporary title-drag capture | Reopen restores from MRU if needed and raises; Sector close does not cascade it; async keyed close restores through the strategic owner and `0x467` | Shared class; both-faction cross-family raise and Sector-close survival; `FUN_00429ce0`, `FUN_004591d0`, `FUN_0045aac0`, `FUN_00441190`, `FUN_00452fc0`, common base/close chain; high |
| Fleet | Detail kind `4`, subject-keyed, many | Strategic-view registry/destruction owner; launched and positioned from Sector context; modeless detail/presentation band | Top-level/controls focus; bit-4 title drag temporarily captures | Reopen restores/raises; Sector close does not cascade it; async keyed close restores through the strategic owner | Shared class; same chain through `FUN_004a2630`; high |
| Defenses | Detail kind `10`, subject-keyed, many | Strategic-view registry/destruction owner; launched and positioned from Sector context; modeless detail/presentation band | Top-level/controls focus; bit-4 title drag temporarily captures | Reopen restores/raises; Sector close does not cascade it; async keyed close restores through the strategic owner | Shared class; same chain through `FUN_004a7790`; high |
| Missions detail | Detail kind `11`, subject-keyed, many | Strategic-view registry/destruction owner; launched and positioned from Sector context; modeless detail/presentation band | Top-level/controls focus; bit-4 title drag temporarily captures | Reopen restores/raises; Sector close does not cascade it; async keyed close restores through the strategic owner | Shared class; same chain through `FUN_0049f130`; high |
| System Finder | Type `0x14`, singleton | Strategic view; strategic modal, bit 0 | Edit/list/top-level focus; handled keys stay in child; no lifetime capture | Existing request no-ops; Escape/Close posts keyed close, then `0x467` restoration | Shared class; Alliance/Empire route reachable; `FUN_0042a000`, `FUN_00460090`; high |
| Fleet Finder | Type `0x15`, singleton | Strategic view; strategic modal, bit 0 | Edit/list/top-level focus; no lifetime capture | Existing request no-ops; shared async close/restoration | Shared class; `FUN_0042a0c0`, `FUN_00461750`; high |
| Troop Finder | Type `0x16`, singleton | Strategic view; strategic modal, bit 0 | Edit/list/top-level focus; no lifetime capture | Existing request no-ops; shared async close/restoration | Shared class; `FUN_0042a4d0`, `FUN_0046cbc0`; high |
| Personnel Finder | Type `0x17`, singleton | Strategic view; strategic modal, bit 0 | Edit/list/top-level focus; no lifetime capture | Existing request no-ops; shared async close/restoration | Shared class; `FUN_0042a180`, `FUN_00463500`; high |
| Message Index | Type `0x0d`, singleton | Strategic view; strategic modal, bit 0 | List/buttons/top-level focus; no lifetime capture | Existing request no-ops; close exposes and refocuses retained modeless windows | Shared class; Alliance runtime close; `FUN_0042a240`, `FUN_00466350`; high |
| Encyclopedia | Type `0x19`, singleton | Strategic view; strategic modal, bit 0 | Edit/list/top-level focus; edit owns typed prefix search; no lifetime capture | Existing request no-ops; Escape/Close restores strategic target on later dispatch | Shared class and behavior corroborated for both factions; `FUN_00429f30`, `FUN_0045d400`; high |
| Status | Type `0x1a`, contextual | Strategic view; whole-view strategic modal, policy `1` | Controls/top-level focus; no lifetime capture | Opened through strategic `0x468`; outside click and F2 blocked; Escape async close restores prior details on a later dispatch | Shared class; both-faction runtime + manual + `FUN_00442d70`; high |
| Battle Alert | Type `0x06`, contextual dialog | Strategic view; bit-0 dialog policy; whole-view pointer scope remains a hypothesis and conservative port default | Dialog controls focus; construction drains old capture; no lifetime capture | Created through `0x468/0x482`; async keyed close/restoration | Shared class; `FUN_0044f670`, `FUN_0044f860`; high static, no fresh runtime |
| Battle Results / Summary | Type `0x1c`, contextual | Strategic view; strategic modal, bit 0 | Tabs/controls/top-level focus; no lifetime capture | Created through `0x468/0x482`; keyed close/`0x467` restoration | Shared class; manual + `FUN_0044c410`, `FUN_0044c630`; high static, unreachable in capture saves |
| Build Selection | Type/key `0x62`, user-level singleton boundary; global `DAT_006b28ac` | Strategic-view registry, opened with Manufacturing context; whole-view strategic modal, policy `5` (bit 0 plus movable bit) | List/edit/buttons focus; temporary title-drag capture only | Normal input cannot re-enter its opener; direct native programmatic duplicate semantics remain unknown and are excluded from the port API; outside click/F2 blocked; Escape restores prior details | Shared class; both-faction runtime + `FUN_00437df0`, `FUN_00437880`; high visible policy, bounded direct-opener unknown |
| Mission creation | Type `0x0c`, contextual | Strategic view/context; whole-view strategic modal, policy `1` | Lists/buttons/top-level focus; no lifetime capture | Context-created; outside click/F2 blocked; Escape shared keyed close/restoration | Shared class; both-faction runtime + `FUN_0046a750`; high |
| Move confirmation | Type `0x10`, contextual | Strategic view; whole-view strategic modal, policy `1` | Dialog controls focus; construction drains old capture; no lifetime capture | Strategic dispatcher creates it; outside click/F2 blocked; Escape cancels and restores prior details | Shared class; both-faction runtime + `FUN_0044f060`; high |
| Object / Agent / game menus | One global active menu family | Invoking strategic top-level/control; transient above owner, not persistent registry authority | Menu/submenu focus; prior HWND saved; no persistent capture established | Dismiss destroys menu and restores saved focus only if focus stayed within menu family | Shared machinery; `FUN_00442380`, `FUN_00442430`, `FUN_004424c0`; high |
| In-place Rename | Child string field, one per active editor | Owning detail window; transient child, no independent stack identity | Edit owns text/Enter; any selection capture is control-bounded | Commit/cancel within owner; destroyed with parent; parent remains restoration target | Shared control; `FUN_004ac7a0`, `FUN_004ac950`, `FUN_004aca40`; high |
| Cockpit / toolbar / reference rail | Persistent strategic owner and children; rail has 12 presentations | Base of strategic stack; modeless controls; rail is MRU/presentation, not window registry | Owner/control focus; interaction capture only where a control requests it | `0x467` focuses owner when no eligible child; rail restores chosen presentation | Side art/targets differ, machinery shared; `FUN_00422ce0`, `FUN_00428b40`, `FUN_00429020`; high |
| Unknown transient class | Type `7` in generic creation switch | Context owner unknown; exact stack/input policy unresolved | Unknown; no public semantic kind or capture rule justified | Generic keyed lifetime only | `FUN_00441190`; low semantic confidence; not API-shaping |

The manual's modal list is a minimum, not an exhaustive classifier. Build
Selection, Mission creation, and Move confirmation now have both-faction
whole-view runtime evidence and must enter the shared strategic-modal policy.
Battle Alert and Battle Results use an explicit conservative whole-view
`port:` default, with their audit cells left open until runtime observes them.
The unidentified type 7 and the two extra bit-0 constructors are not
API-shaping evidence until their call contexts are named.

## Research-gate answers

1. The child registry is a keyed threaded ordered tree; presentation/MRU,
   native focus, and capture are separate structures/state.
2. Existing singleton Finders, Message, and Encyclopedia merely return;
   details restore/raise; duplicate Sector returns without an opener-side
   raise, though a following detail-selection caller may raise its child.
3. Sector is created low; detail windows are independent strategic-root-owned
   modeless top-levels opened from Sector context. Transient children/menus sit
   above their actual owners, and strategic modals form an input boundary. The
   reference rail is presentation history, not z-order.
4. Strategic modality is not a nested loop, owner disable, or lifetime
   capture. Bit 0 supplies dialog/function-key policy; a still-unidentified
   pre-command native gate blocked normal underlay pointer input for
   Encyclopedia, Status, Build Selection, Create Mission, and Move
   Confirmation in both faction saves.
5. The focused edit/list child owns handled keyboard input; otherwise its
   top-level is the owner.
6. Unhandled keys can forward to the parent/top-level and reach the strategic
   handler, where modal F-key suppression applies.
7. Drag capture is temporary; close posts destruction then a focus/raise
   recalculation. Menus explicitly restore their saved focus. The generic
   `+0x10c` saved-focus slot is written only by `FUN_006073b0`.
8. Posted close plus per-dispatch native routing prevents the dismissing event
   from activating the exposed surface. The port must make this invariant
   explicit with a routing snapshot.
9. Build Selection, Mission creation, and Move Confirmation are additional
   bit-0 whole-strategic-view dialog families. Battle Alert shares the static
   machinery but was not triggered. Two bit-0 classes remain semantically
   unnamed and are excluded from the public kind set.
10. Battle Results/Summary uses the same bit-0 and shared strategic dialog
    machinery. Battle Alert did not appear during retained day 60--79 Alliance
    and day 3--23 Empire intervals, so Battle Results was unreachable. Both
    retain explicit conservative port policy and no visible parity claim.
11. No faction-specific behavioral branch was recovered. Both factions use
    the same classes/routing and corroborated cross-family raise, Sector-close
    survival, and Status/Build/Mission/Move blocking and dismissal.

## Current Rust seams to replace

The present port has useful feature boundaries but no single window authority:

- `crates/rebellion-app/src/main.rs` defines `FrameKeyboardOwner` for only
  Cockpit versus Encyclopedia, assembles `GalaxyMapState::pointer_blocked`
  from a hand-maintained family list, and separately calculates
  `strategic_input_enabled`.
- `sector_window.rs`, `system_window.rs`, `fleet_window.rs`,
  `defenses_window.rs`, and `missions_window.rs` each own private `Vec`s of
  open instances and independently call `egui::Context::move_to_top`.
- Finder, Message, Encyclopedia, Status, menus, and confirmations independently
  choose foreground/tooltip ordering and input gates.

The new renderer-independent kernel should therefore expose:

- semantic `WindowKind` plus opaque session `WindowId` and optional owner;
- declared cardinality and existing-open policy (`NoOp`, `RaiseRestore`,
  unresolved, or family-specific placement/replacement);
- deterministic bottom-to-top stack distinct from the 12-entry presentation
  history;
- visibility/minimized/closing lifecycle state;
- focused top-level and focused child/control identity;
- independent input policy (`Modeless`, owner-blocking if later proven, or
  strategic-modal) and temporary pointer capture;
- one immutable routing snapshot per physical input dispatch;
- explicit transitions whose renderer adapter performs as egui raise, focus,
  pointer consumption, or paint-order operations.

**port:** the kernel should model the recovered semantic result, not HWNDs,
Win32 message numbers, intrusive trees, `PostMessage`, or egui order bands.
Those are adapters/evidence, not domain API.

## Unresolved and deliberately non-load-bearing details

- Exact native code that rejects a normal pointer hit below the observed
  strategic-modal families.
- Semantic names for type 7 and the bit-0 classes constructed by
  `FUN_0046f140` and `FUN_0049ee20`.
- Whether any rare command dialog blocks only its immediate owner rather than
  the whole strategic view. A whole-view blocker is a conservative port
  default, not a recovered parity claim, until runtime narrows it.
- Direct programmatic duplicate behavior for Build Selection. Normal input
  cannot re-enter its opener while it owns the strategic boundary, and the
  port intentionally rejects re-entrant open rather than exposing this native
  uncertainty.
- Fresh runtime behavior for Battle Alert and Battle Results. Battle Alert did
  not trigger in either retained interval, so Battle Results was unreachable;
  neither receives a parity claim.
- Native Windows corroboration. Wine was approved for this implementation
  research gate and is not represented as final platform acceptance.

These gaps do not require competing focus stacks or feature-local modal
exceptions. They remain explicit data and policy refinements inside the
shared contract.

## Reproduction commands

Static disassembly used the owned binary read-only:

```sh
objdump -Mintel -d data/base/REBEXE.EXE \
  --start-address=0x600280 --stop-address=0x6002a0
objdump -Mintel -d data/base/REBEXE.EXE \
  --start-address=0x601080 --stop-address=0x601370
objdump -Mintel -d data/base/REBEXE.EXE \
  --start-address=0x606940 --stop-address=0x606990
objdump -Mintel -d data/base/REBEXE.EXE \
  --start-address=0x607d00 --stop-address=0x607d50
```

The clean bounded decompile was regenerated outside the repository with
Ghidra 12.1.4 headless and `ghidra/scripts/DecompileTargets.java`. The local
manifest retains the exact headless invocation, Ghidra archive SHA-256
`ddac49f903da9d5bac833e5cc79395098b9c33cfd3279be5f31bd00387d2d4db`,
and JDK archive SHA-256
`dbb698396d478e7fa2b1e50f4103324b2a99b90569ee27c33f2261f9215cf41e`.
Runtime commands, hashes, observations, and limits are recorded in
[`strategic-windowing-runtime-evidence.md`](strategic-windowing-runtime-evidence.md).
No shared Ghidra project was mutated.
