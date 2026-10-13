---
title: "Strategic Windowing Toolkit Contract"
description: "Original strategic-window identity, registry, stacking, modality, focus, capture, keyboard routing, close, and restoration behavior"
category: "ghidra"
created: 2026-10-10
updated: 2026-10-10
---

# Strategic Windowing Toolkit Contract

This note records the provisional behavioral contract needed by the planned
`rebellion-windowing` crate. It distinguishes the original game's shared
window machinery from its feature-specific painting and contents. The result
is sufficient to shape a narrow, reversible identity, ordering, focus,
capture, and per-dispatch routing API without copying the Win32 implementation.
Per-family input policy remains provisional where the runtime queue is open.

The executable studied is the owned `REBEXE.EXE` with SHA-256
`b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`.
Addresses below are for that build. Proprietary assets and original-runtime
captures are not committed. The repository-visible
[`strategic-windowing-runtime-evidence.md`](strategic-windowing-runtime-evidence.md)
records the retained observations, their limits, and the open capture queue.

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
instruction appendix instead of fabricating decompiler boundaries. Larger
constructors and dispatchers were inspected in the existing corpus and with
targeted `objdump` ranges rather than copied into this note.

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
   permanent capture. Bit 0 alone does not prove the same pointer scope for
   every other dialog that carries it.
4. **Opening an existing window is family-specific.** Finder, Message, and
   Encyclopedia singleton openers return without raising. Existing detail
   windows are restored/raised. A duplicate Sector opener returns its object
   without itself raising it; a caller may subsequently raise/select a detail
   child. The port must declare this policy per window kind.
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
| `+0x10c` | Saved focus HWND | Restored by `FUN_00606580`; initialized null and no strategic writes recovered; high/unused here |
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
registries and tears down its trees. `FUN_00606580` can restore a saved HWND
from `+0x10c`, but that field is initialized to zero by `FUN_00606380` and no
strategic writer was recovered. It is not a sound basis for a port-wide focus
stack.

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

## Opening, close, and restoration

### Close sequence

`FUN_00606960` posts `(message=0x405, wParam=this+0x24, lParam=this)` to the
parent HWND. `FUN_006007b0` and the strategic `FUN_00422ce0` handler remove the
appropriate presentation entry, call `FUN_00600f90`, and post `0x467`.
Destruction and replacement focus therefore occur on subsequent native
message dispatches, not by synchronously returning a new input target from
the close callback.

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

### Existing-window policy

| Family | Existing-instance behavior |
|---|---|
| System/Fleet/Troop/Personnel Finder | Singleton opener returns/no-ops; it does not explicitly raise or focus |
| Message Index | Singleton opener returns/no-ops |
| Encyclopedia | Singleton opener returns/no-ops |
| Sector | Same subject key returns the existing object without an explicit raise in the opener; a caller that opens/selects a detail child can then raise that presentation; at most two sectors are retained |
| System/Manufacturing, Fleet, Defenses, Missions detail | Existing subject+kind is restored from MRU when minimized and moved to `HWND_TOP` |
| Status and command dialogs | Contextual creation; no general duplicate-raise rule established |

The singleton conclusions come from `FUN_0042a000`, `FUN_0042a0c0`,
`FUN_0042a4d0`, `FUN_0042a180`, `FUN_0042a240`, and `FUN_00429f30`. Sector
behavior comes from `FUN_00429ce0`; detail behavior from `FUN_0045aac0`.
Consequently, a generic `open == raise` rule would be a parity defect.

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
next F2/cockpit click then worked.

**unknown:** the exact native pre-command pointer gate was not identified. It
occurs before an enabled underlay control's command reaches `FUN_00422ce0` in
the observed Encyclopedia scenario. The manual's minimum modal set plus that
runtime observation justify a shared strategic boundary for those documented
families. For other bit-0 dialogs, full pointer blocking remains a `hyp:` or a
conservative `port:` policy until runtime evidence establishes its scope.
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
| Sector | Key `(system_low16 << 6) \| 1`; max two | Strategic view; modeless lower/detail band; created at `HWND_BOTTOM`, with toolbar placed above | Top-level and owned controls can focus; no family-lifetime capture or movable-title bit recovered | Duplicate returns existing without an opener-side raise; following detail selection may raise; third replaces the sector on the requested galaxy half; generic keyed close restores through `0x467` | Shared class; Alliance two-window runtime; `FUN_00429ce0`; high |
| System / Manufacturing | Detail kind `9`, subject-keyed, many | Strategic-view child launched and positioned from Sector context; normal modeless detail/presentation band; presentation-rail capable | Top-level/controls focus; movable bit 4 uses temporary title-drag capture | Reopen restores from MRU if needed and raises; async keyed close restores through the strategic owner and `0x467` | Shared class; `FUN_0045aac0`, `FUN_00441190`; high |
| Fleet | Detail kind `4`, subject-keyed, many | Strategic-view child launched and positioned from Sector context; modeless detail/presentation band | Top-level/controls focus; bit-4 title drag temporarily captures | Reopen restores/raises; async keyed close restores through the strategic owner | Shared class; same static path; high |
| Defenses | Detail kind `10`, subject-keyed, many | Strategic-view child launched and positioned from Sector context; modeless detail/presentation band | Top-level/controls focus; bit-4 title drag temporarily captures | Reopen restores/raises; async keyed close restores through the strategic owner | Shared class; same static path; high |
| Missions detail | Detail kind `11`, subject-keyed, many | Strategic-view child launched and positioned from Sector context; modeless detail/presentation band | Top-level/controls focus; bit-4 title drag temporarily captures | Reopen restores/raises; async keyed close restores through the strategic owner | Shared class; same static path; high |
| System Finder | Type `0x14`, singleton | Strategic view; strategic modal, bit 0 | Edit/list/top-level focus; handled keys stay in child; no lifetime capture | Existing request no-ops; Escape/Close posts keyed close, then `0x467` restoration | Shared class; Alliance/Empire route reachable; `FUN_0042a000`, `FUN_00460090`; high |
| Fleet Finder | Type `0x15`, singleton | Strategic view; strategic modal, bit 0 | Edit/list/top-level focus; no lifetime capture | Existing request no-ops; shared async close/restoration | Shared class; `FUN_0042a0c0`, `FUN_00461750`; high |
| Troop Finder | Type `0x16`, singleton | Strategic view; strategic modal, bit 0 | Edit/list/top-level focus; no lifetime capture | Existing request no-ops; shared async close/restoration | Shared class; `FUN_0042a4d0`, `FUN_0046cbc0`; high |
| Personnel Finder | Type `0x17`, singleton | Strategic view; strategic modal, bit 0 | Edit/list/top-level focus; no lifetime capture | Existing request no-ops; shared async close/restoration | Shared class; `FUN_0042a180`, `FUN_00463500`; high |
| Message Index | Type `0x0d`, singleton | Strategic view; strategic modal, bit 0 | List/buttons/top-level focus; no lifetime capture | Existing request no-ops; close exposes and refocuses retained modeless windows | Shared class; Alliance runtime close; `FUN_0042a240`, `FUN_00466350`; high |
| Encyclopedia | Type `0x19`, singleton | Strategic view; strategic modal, bit 0 | Edit/list/top-level focus; edit owns typed prefix search; no lifetime capture | Existing request no-ops; Escape/Close restores strategic target on later dispatch | Shared class and behavior corroborated for both factions; `FUN_00429f30`, `FUN_0045d400`; high |
| Status | Type `0x1a`, contextual | Strategic view; strategic modal, bit 0 | Controls/top-level focus; no lifetime capture | Opened through strategic `0x468`; async close/`0x467` restoration | Shared class; manual + `FUN_00442d70`; high static, no fresh runtime |
| Battle Alert | Type `0x06`, contextual dialog | Strategic view; bit-0 dialog policy; whole-view pointer scope remains a hypothesis and conservative port default | Dialog controls focus; construction drains old capture; no lifetime capture | Created through `0x468/0x482`; async keyed close/restoration | Shared class; `FUN_0044f670`, `FUN_0044f860`; high static, no fresh runtime |
| Battle Results / Summary | Type `0x1c`, contextual | Strategic view; strategic modal, bit 0 | Tabs/controls/top-level focus; no lifetime capture | Created through `0x468/0x482`; keyed close/`0x467` restoration | Shared class; manual + `FUN_0044c410`, `FUN_0044c630`; high static, unreachable in capture saves |
| Build Selection | Type/key `0x62`, de facto singleton global `DAT_006b28ac`; duplicate behavior unresolved | Strategic-view registry, opened with Manufacturing context; bit-0 dialog policy plus movable bit (`+0xb8 = 5`); whole-view pointer scope remains a hypothesis and conservative port default | List/edit/buttons focus; temporary title-drag capture only | The recovered opener overwrites the global and contains no duplicate guard; shared keyed close removes the registered instance and restores through the strategic owner | Shared class; `FUN_00437df0`, `FUN_00437880`; high static except duplicate and pointer policy |
| Mission creation | Type `0x0c`, contextual | Strategic view/context; bit-0 dialog policy; whole-view pointer scope remains a hypothesis and conservative port default | Lists/buttons/top-level focus; no lifetime capture | Context-created; shared keyed close/restoration | Shared class; `FUN_0046a750`; high static, no fresh runtime |
| Move confirmation | Type `0x10`, contextual | Strategic view; bit-0 dialog policy; whole-view pointer scope remains a hypothesis and conservative port default | Dialog controls focus; construction drains old capture; no lifetime capture | Strategic dispatcher creates it; shared keyed close/restoration | Shared class; `FUN_0044f060`; high static, no fresh runtime |
| Object / Agent / game menus | One global active menu family | Invoking strategic top-level/control; transient above owner, not persistent registry authority | Menu/submenu focus; prior HWND saved; no persistent capture established | Dismiss destroys menu and restores saved focus only if focus stayed within menu family | Shared machinery; `FUN_00442380`, `FUN_00442430`, `FUN_004424c0`; high |
| In-place Rename | Child string field, one per active editor | Owning detail window; transient child, no independent stack identity | Edit owns text/Enter; any selection capture is control-bounded | Commit/cancel within owner; destroyed with parent; parent remains restoration target | Shared control; `FUN_004ac7a0`, `FUN_004ac950`, `FUN_004aca40`; high |
| Cockpit / toolbar / reference rail | Persistent strategic owner and children; rail has 12 presentations | Base of strategic stack; modeless controls; rail is MRU/presentation, not window registry | Owner/control focus; interaction capture only where a control requests it | `0x467` focuses owner when no eligible child; rail restores chosen presentation | Side art/targets differ, machinery shared; `FUN_00422ce0`, `FUN_00428b40`, `FUN_00429020`; high |
| Unknown transient class | Type `7` in generic creation switch | Context owner unknown; exact stack/input policy unresolved | Unknown; no public semantic kind or capture rule justified | Generic keyed lifetime only | `FUN_00441190`; low semantic confidence; not API-shaping |

The manual's modal list is a minimum, not an exhaustive classifier. Build
Selection, Mission creation, Move confirmation, and Battle Alert must enter the
shared dialog-policy representation. A conservative whole-view blocker may be
used as an explicit `hyp:` or `port:` default, with the related audit cells and
runtime queue left open until its scope is observed. The unidentified type 7
and the two extra bit-0 constructors are not API-shaping evidence until their
call contexts are named.

## Research-gate answers

1. The child registry is a keyed threaded ordered tree; presentation/MRU,
   native focus, and capture are separate structures/state.
2. Existing singleton Finders, Message, and Encyclopedia merely return;
   details restore/raise; duplicate Sector returns without an opener-side
   raise, though a following detail-selection caller may raise its child.
3. Sector is created low, detail windows are normal modeless top-levels,
   transient children/menus sit above owners, and strategic modals form an
   input boundary. The reference rail is presentation history, not z-order.
4. Strategic modality is not a nested loop, owner disable, or lifetime
   capture. Bit 0 supplies dialog/function-key policy; a still-unidentified
   pre-command native gate blocked normal underlay pointer input in the
   observed Encyclopedia scenario.
5. The focused edit/list child owns handled keyboard input; otherwise its
   top-level is the owner.
6. Unhandled keys can forward to the parent/top-level and reach the strategic
   handler, where modal F-key suppression applies.
7. Drag capture is temporary; close posts destruction then a focus/raise
   recalculation. Menus explicitly restore their saved focus. The generic
   `+0x10c` saved-focus slot is dormant on the recovered strategic paths.
8. Posted close plus per-dispatch native routing prevents the dismissing event
   from activating the exposed surface. The port must make this invariant
   explicit with a routing snapshot.
9. Build Selection, Mission creation, Move confirmation, and Battle Alert are
   additional bit-0 dialog-policy families. Their full pointer scope remains
   unresolved. Two bit-0 classes remain semantically unnamed and therefore
   excluded from the public kind set.
10. Battle Results/Summary uses the same bit-0 and shared strategic dialog
    machinery. Its visible runtime was unreachable in the available saves.
11. No faction-specific behavioral branch was recovered. Both factions use
    the same classes/routing and corroborated Encyclopedia blocking/search/
    dismissal.

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

- Exact native code that rejects a normal pointer hit below Encyclopedia, and
  whether every other bit-0 dialog uses the same whole-view pointer scope.
- Semantic names for type 7 and the bit-0 classes constructed by
  `FUN_0046f140` and `FUN_0049ee20`.
- Whether any rare command dialog blocks only its immediate owner rather than
  the whole strategic view. A whole-view blocker is a conservative port
  default, not a recovered parity claim, until runtime narrows it.
- Fresh runtime behavior for Battle Results, Status, Battle Alert, Build
  Selection, Mission creation, and Move confirmation. Static/manual evidence
  establishes dialog policy, but their pointer scope and individual audit
  cells remain open.
- Fresh cross-family modeless raise corroboration. Two Sector windows and a
  modal above them were observed; cross-family detail ordering remains backed
  by the explicit native `SetWindowPos`, presentation, and MRU paths.
- Native Windows corroboration. Wine was approved for this implementation
  research gate and is not represented as final platform acceptance.

These gaps do not require competing focus stacks or feature-local modal
exceptions. They remain explicit data and policy refinements inside the
provisional shared contract.

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
Ghidra 12.1.4 headless and `ghidra/scripts/DecompileTargets.java`. The exact
headless invocation and tool hashes are not retained in a repository-visible
record, so that regeneration provenance remains incomplete. Runtime limits
and the remaining capture queue are recorded in
[`strategic-windowing-runtime-evidence.md`](strategic-windowing-runtime-evidence.md).
No shared Ghidra project was mutated.
