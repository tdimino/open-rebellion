---
title: "Strategic Window Modality and Focus"
description: "Design for recovering the original strategic-window contract, introducing a renderer-independent windowing crate, and migrating the existing strategic surfaces to one focus and input authority"
type: feat
status: draft
created: 2026-10-10
updated: 2026-10-10
tags: [interface, parity, windowing, focus, modality, input, architecture]
---

# Strategic Window Modality and Focus

## Decision

Open Rebellion will stop adding feature-specific strategic-window focus and
input exceptions. It will first recover the original game's shared window
contract, then add a renderer-independent `rebellion-windowing` crate and
migrate the existing strategic window families to that single authority.

This is not a reimplementation of every original `Cool*` visual control.
Window contents, bitmap painting, lists, tabs, text fields, asset access,
game-state queries, and egui/macroquad integration remain in
`rebellion-render`. The new crate owns only shared window lifecycle and input
policy.

The migration may use short-lived adapters, but the accepted production path
must not retain two competing authorities for focus, stacking, modality, or
input routing.

## Intent

The original game exposes modeless strategic windows that can overlap and a
documented set of modal windows that prevent interaction with the underlying
game. Open Rebellion currently reproduces those surfaces through independent
render modules. Each module makes local decisions about focus, egui layer
order, input, and dismissal, while `rebellion-app` maintains additional global
exceptions.

The immediate user-visible goal is correct strategic-window behavior:

- the topmost eligible window receives keyboard input;
- pointer input reaches the topmost eligible surface under the pointer;
- a modal window blocks all underlying strategic input, including clicks
  outside the modal rectangle;
- raising, opening, closing, and restoring windows produce the original order;
- dismissing a window cannot pass the same input through to the newly exposed
  window or cockpit control;
- the behavior is the same for Alliance and Empire surfaces;
- the implementation is shared rather than copied into each window family.

The architectural goal is to make those rules deterministic and testable
without a renderer or a running campaign. Adding the next strategic window
should require declaring its identity and policy, not extending a hand-kept
matrix in the application loop.

## Acceptance authority

This design serves the following interface-audit cells:

| Cell | Required behavior |
|---|---|
| `CMD-04-C005` | Raising an existing strategic window follows the recovered original behavior. |
| `CMD-04-C016` | Status windows block underlying input. |
| `CMD-04-C017` | Finder windows block underlying input. |
| `CMD-04-C018` | Battle Summary blocks underlying input. |
| `CMD-04-C019` | Encyclopedia blocks underlying input. |
| `CMD-04-C020` | Message windows block underlying input. |

The manual states that Status, Finder, Battle Summary, Encyclopedia, and
Message windows are modal. That statement establishes the minimum modal set;
it does not prove the complete stacking, focus, capture, keyboard, dismissal,
or restoration contract. Those rules require executable tracing and original
runtime observation before implementation choices may be called parity.

The strict acceptance authorities remain:

- `docs/qa/2026-09-10-interface-parity-audit/2026-10-06-manual-window-checklists.md`;
- `docs/qa/2026-09-10-interface-parity-audit/2026-10-06-surface-ledger.json`;
- `docs/qa/2026-09-10-interface-parity-audit/known-deviations.md`;
- the applicable original-runtime capture and browser evidence records.

## Current implementation

There is no project-owned shared window manager. The port currently combines
three partial mechanisms.

### Application-level exceptions

`crates/rebellion-app/src/main.rs` owns cross-feature exceptions such as
`FrameKeyboardOwner`, `GalaxyMapState::pointer_blocked`, and
`strategic_input_enabled`. The pointer flag is assembled from a hand-kept list
of open windows and rectangle checks. The keyboard owner recognizes only the
cases explicitly added to it.

This creates a negative maintenance contract: every new window must be added
to every relevant exception list, and omissions route input to an unrelated
cockpit shortcut or underlying surface.

### Per-family miniature window managers

Modeless window families such as Sector, System, Fleet, Defenses, and Missions
keep private collections of open windows. Several treat the last vector entry
as focused and call `egui::Context::move_to_top` independently. Because
families render in a fixed application order, cross-family ordering is partly
an artifact of call order rather than one strategic-window stack.

Modal and modal-like surfaces such as Status, Finders, Message, and
Encyclopedia choose high egui order bands locally and may raise themselves
each frame. They do not share an authoritative modal boundary.

### Direct egui control use

The port has functional counterparts to original controls, but not direct
Rust analogues of the `Cool*` class hierarchy:

| Original mechanism | Current port pattern |
|---|---|
| `CoolGameWindow` and the base window procedure | Application frame loop, `egui::Context`, and per-feature `egui::Area` calls |
| `CustomDialogBox` | Independent `*WindowState`, `*WindowAction`, and `draw_*` modules |
| `CoolDragList` | Feature-local list geometry, scrolling, selection, and drag code |
| `CoolStringField` | `egui::TextEdit` plus feature-local editing or search rules |
| `CoolTabControl` | Feature-local tab enums, rectangles, painting, and interaction |
| `CoolStrobeButton` | Feature-local hit testing and normal/pressed bitmap painting |
| Child-window registry and z-order | Private collections plus scattered egui layer operations |

Those feature modules are useful existing boundaries. This design preserves
their content and action APIs while replacing their duplicated lifecycle and
ownership rules.

## Evidence-first research gate

The implementation plan must begin with a bounded reverse-engineering work
item. No production window-manager API is final until this work produces
`ghidra/notes/strategic-windowing-toolkit.md` and its cited decompiles.

### Static trace

The trace starts from the shared mechanisms already visible in the corpus:

- `FUN_005ff910`, the common native-control/window construction path;
- `FUN_00600310`, the base Win32 dispatcher that associates an `HWND` with
  the C++ object and invokes its virtual window procedure;
- `FUN_006007b0`, shared game-window message handling;
- `FUN_00604500`, child-window lookup by type key;
- `FUN_00600f90`, child-window removal and destruction;
- all callers of `SetFocus`, `GetFocus`, `SetWindowPos`, `SetCapture`,
  `GetCapture`, `ReleaseCapture`, `EnableWindow`, `PeekMessage`,
  `TranslateMessage`, and `DispatchMessage` on the strategic path;
- constructors, vtables, window procedures, and close paths for every window
  in the migration matrix.

The trace must record addresses, call chains, vtable slots, field offsets,
message IDs, type keys, confidence, and unresolved branches. A Markdown note
without the decompiled or disassembled source it cites is not sufficient.

### Questions the trace must answer

1. What data structure is the original child-window registry, and which data
   separately records focus or z-order?
2. Does opening an existing singleton raise it, focus a child control, or
   merely return the existing object to its caller?
3. What establishes ordering between Sector windows, detailed object windows,
   the cockpit/toolbar, transient children, and modal windows?
4. Are documented modal windows implemented through a nested message loop,
   a disabled owner, capture, message filtering, or a combination?
5. Which window owns keyboard input when an edit or list child has focus?
6. Do unhandled keys bubble to the strategic application or stop at the
   focused top-level window?
7. What happens to focus and capture when the focused window closes,
   minimizes, opens an Encyclopedia or Status child, or destroys its parent?
8. Can the input that dismisses one surface activate a newly exposed surface
   during the same native message dispatch?
9. Which dialogs not named in the manual's minimum modal list also block their
   owner or the whole strategic view?
10. Is Battle Summary governed by the same mechanism even though the port
    currently renders battle results in the tactical scene?
11. Are any answers faction-specific because the two sides instantiate
    different window classes or command routes?

### Dynamic corroboration

Static conclusions that affect visible behavior require original-runtime Wine
checks for both factions where the state is reachable. Capture scenarios must
include overlapping modeless windows, raising an existing window, a modal
opened above modeless windows, outside-modal clicks, focused text entry,
Escape or close-button dismissal, parent/child transitions, and post-close
focus restoration.

Wine evidence is sufficient for this local implementation gate. It is not
represented as native-Windows acceptance.

### Uncertainty rule

Unknown behavior remains explicit. An unresolved item may be implemented as a
`hyp:` or `port:` boundary only when the applicable audit cell remains open and
the deviation register names it. Tests must not silently turn a convenient
egui behavior into a claim about the original.

## Proposed crate boundary

Add `crates/rebellion-windowing` to the workspace as a small Rust library.

### Dependency direction

```text
rebellion-windowing
        ↑
rebellion-render  ←  rebellion-core
        ↑
rebellion-app
```

`rebellion-windowing` must not depend on `rebellion-app`,
`rebellion-render`, egui, macroquad, assets, the filesystem, audio, platform
APIs, or campaign simulation. It should not depend on `rebellion-core` unless
an evidence-backed identity requirement cannot be expressed with opaque window
identities. The expected design needs no such dependency.

`rebellion-render` adapts the kernel's decisions to egui layer ordering,
hit regions, pointer consumption, focus requests, and painting. The
application owns or coordinates the strategic windowing state with the other
strategic UI states.

### Kernel responsibilities

The crate owns:

- stable top-level and child window identity for the lifetime of a UI session;
- declared parent/owner relationships;
- open, raise, minimize/restore when applicable, and close transitions;
- one deterministic bottom-to-top order;
- evidence-backed stack bands or constraints, kept separate from modality;
- the focused top-level window and focused child/control owner when required;
- the active modal boundary;
- pointer and keyboard eligibility;
- pointer capture ownership when a recovered interaction needs it;
- reconciliation when a renderer-owned window disappears;
- a per-frame routing snapshot that prevents same-frame input fallthrough;
- invariant validation and deterministic transition results.

The crate does not own:

- egui or macroquad calls;
- bitmap resources, fonts, palettes, scaling, or window chrome;
- list rows, tabs, text editing, scrollbars, or selection contents;
- game commands, object menus, simulation mutations, or `GameWorld` queries;
- window-specific state such as selected tabs, selected units, article text,
  message filters, or finder results;
- native OS windows or a general desktop window system;
- save-game or replay persistence. Strategic window state remains ephemeral.

### Separate ordering from input policy

The design must not encode `modal`, `topmost`, and `tooltip` as synonyms.
Each registered surface has independent evidence-backed attributes:

- **identity and ownership**: what surface this is and which parent owns it;
- **stack constraint or band**: where it may paint relative to other surfaces;
- **input policy**: whether it is modeless, blocks its owner, or blocks the
  strategic view;
- **visibility state**: open, visible, minimized, or closing as supported;
- **focus capability**: whether the top-level surface or one of its children
  may own keyboard input;
- **capture state**: whether a bounded interaction temporarily owns pointer
  motion and release.

This separation lets a lower-band Sector window remain modeless, a top-level
Finder be modal, and a temporary child control sit above its parent without
inventing behavior from egui's `Order` enum.

### Identity

The kernel uses a semantic window kind and an opaque instance identity. A
singleton window uses one canonical identity. Multi-instance families use the
underlying game's stable object identity only for the current UI session.

Original type keys and command IDs are recorded in evidence and tests where
they explain behavior, but they are not save or wire formats. The crate must
not expose `SystemKey`, `FleetKey`, or another simulation type merely to name a
window.

### Commands and snapshots

The exact public names are implementation-plan decisions, but the API must
preserve this shape:

```rust
manager.apply(WindowCommand::Open(spec));
manager.apply(WindowCommand::Raise(id));
manager.apply(WindowCommand::Close(id));
manager.reconcile(visible_ids);

let frame = manager.begin_frame();
frame.paint_order();
frame.keyboard_owner();
frame.pointer_allows(id);
frame.active_modal();
```

All transitions are deterministic values that can be unit-tested without
drawing. Render modules continue to return feature actions; the adapter adds
window commands rather than mutating a second focus stack.

### Frame lease and no-fallthrough rule

At the start of a frame, the manager creates an immutable routing snapshot.
The window that owns an input event according to that snapshot retains the
lease for the rest of the frame even if handling the event closes it. The
newly exposed window or cockpit may become eligible on the next frame, but it
cannot receive the same press, release, Escape, Enter, or typed character.

If the original trace demonstrates a different boundary than one render frame,
the implementation must model the recovered native-message boundary while
retaining the same externally visible guarantee.

## Renderer integration

### Window data remains local

Each existing `*WindowState` continues to own its window-specific data. During
migration, its private collection may remain as storage, but collection order
must stop representing global focus or paint order. The shared manager supplies
the order in which visible instances are considered and painted.

Window modules must not call `move_to_top` as an independent policy decision.
An egui adapter may make the required calls centrally to realize the kernel's
order.

### Pointer routing

The adapter reports current top-level hit regions or hit candidates in the
form selected by the implementation plan. The kernel chooses eligibility from
its frame snapshot.

When a strategic modal is active, the renderer installs a full strategic-canvas
input barrier beneath the modal and above every blocked surface. It consumes
pointer interaction outside the modal without inventing a visible overlay.
Pointer events inside the modal continue to be resolved by its ordinary egui
controls.

A missing or temporarily unpaintable modal fails closed for input: it may not
expose the cockpit merely because its asset or content path failed. The window
may render the repository's established failure state or close through an
explicit action, but the manager must not silently discard the modal boundary.

### Keyboard routing

The frame snapshot supplies one top-level keyboard owner. A focused edit or
list inside that window may consume keys before the window's general commands.
Keys not consumed by that owner follow the recovered propagation rule. Until
that rule is traced, they do not fall through to cockpit accelerators.

The completed migration removes `FrameKeyboardOwner` and replaces the current
window-by-window application exceptions with the shared decision. Typing in
the Encyclopedia, Finder, Message, or another focused field must never invoke
an unrelated strategic hotkey.

### Paint order

The adapter maps the recovered order to egui without treating egui's broad
`Background`, `Middle`, `Foreground`, and `Tooltip` categories as the source
of truth. It applies one central order each frame. Feature draw-call order may
not determine which unrelated family appears on top.

### Battle Summary

Battle Summary participates in the same semantic contract and test matrix even
if its content remains in `tactical_view`. The implementation need not move
battle rendering into the strategic renderer. It must expose the result
surface to the common window policy or an adapter with identical observable
semantics, as justified by the original trace.

## Migration strategy

The feature uses one topic branch and a sequence of independently reviewable
commits. Local beads will preserve the following dependency order after the
written implementation plan is approved.

### Workstream 1: recover the contract

Produce the Ghidra note, cited decompiles, runtime scenarios, and an explicit
window classification matrix. This work blocks the production API and every
parity claim.

### Workstream 2: build the pure kernel

Create the crate and implement identities, relationships, ordering,
transitions, modality, routing snapshots, reconciliation, and invariant tests.
No renderer integration belongs in this workstream.

### Workstream 3: add the egui adapter and migrate modeless families

Integrate Sector, System, Fleet, Defenses, Missions, and any other recovered
modeless strategic family. Preserve content and appearance. Remove private
global-focus meaning and per-family layer raising as each family migrates.

### Workstream 4: migrate strategic modals

Integrate Status, all Finder families, Message, and Encyclopedia. Replace the
handwritten application pointer and keyboard exceptions, add the outside-modal
input barrier, and preserve focused child-control behavior.

### Workstream 5: migrate owned children and Battle Summary

Integrate the recovered transient/owned-dialog set and the tactical Battle
Summary boundary. Do not classify a dialog by resemblance; use the research
matrix.

### Workstream 6: remove duplicate authorities and accept the feature

Search for and remove obsolete `move_to_top`, private focus-order, pointer
blocker, and keyboard-owner policies. Update documentation, deviations, audit
JSON and Markdown, roadmap evidence, and the two-faction acceptance record.

Temporary shadow assertions may compare the new manager's predicted state to
an existing family during its migration. Shadow state must be test/debug-only
and removed when that family changes authority.

## Invariants and failure handling

The implementation must enforce at least these invariants:

1. A visible registered window appears exactly once in the global order.
2. A focused window is visible and registered.
3. A captured pointer owner is visible and eligible under the current modal
   boundary.
4. A child cannot outlive an owner when the recovered contract destroys it
   with the owner.
5. A strategic modal blocks every window below its boundary and the cockpit.
6. One physical input event has at most one top-level owner.
7. Closing or reconciling a window cannot leave a stale focus, capture, parent,
   or order entry.
8. Reopening or raising a singleton cannot create a duplicate instance.
9. Reconciliation is deterministic regardless of hash-map iteration order.
10. Missing rendering content cannot accidentally weaken modality.

Invalid commands return a typed result suitable for a debug assertion and a
production diagnostic. Routine idempotent requests—such as opening an
already-open singleton when the recovered behavior raises it—are represented
as explicit successful transitions, not errors. The kernel does not log,
touch the filesystem, or attempt recovery through platform APIs.

## Testing strategy

### Kernel tests

Table-driven and behavior-sentence unit tests in `rebellion-windowing` cover:

- opening singleton and multi-instance windows;
- duplicate suppression;
- raise order across recovered bands;
- focus transfer after open, raise, minimize/restore, close, and reconcile;
- parent/child close behavior;
- nested or competing modal boundaries if the original permits them;
- pointer and keyboard eligibility above and below a modal;
- capture acquisition, retention, and release;
- no same-frame click, key, or typed-character fallthrough;
- deterministic handling of stale renderer registrations;
- failure-closed modality;
- every recovered edge case represented in the Ghidra note.

New behavior tests must be demonstrated failing before implementation and must
cite the original function, manual statement, or approved port boundary they
encode.

### Renderer tests

Scoped `rebellion-render` tests cover:

- mapping manager order to egui layer order;
- full-canvas modal pointer consumption with an unchanged visual background;
- a topmost edit retaining keyboard ownership;
- window-family adapters emitting the correct open, raise, and close commands;
- preservation of family-specific state while focus moves elsewhere;
- removal of private per-family ordering behavior;
- Battle Summary's adapter contract.

Tests should exercise production render paths where possible. Test-only
fixtures expose state; they do not replace production routing.

### Integration and acceptance

For both Alliance and Empire, native and packaged browser journeys cover:

1. overlapping modeless windows from different families;
2. clicking each visible window to raise it;
3. invoking an already-open window through a second route;
4. opening each documented modal above modeless windows;
5. clicking inside and outside the modal;
6. typing and navigating inside a focused child field or list;
7. attempting cockpit hotkeys while the modal owns input;
8. closing with the original control and any traced keyboard route;
9. verifying next-owner restoration on the following input boundary;
10. transitions from Status or another parent into contextual Encyclopedia;
11. Battle Summary blocking and dismissal;
12. no page error, panic, failed request, missing-asset diagnostic, or stale
    egui layer after repeated open/close cycles.

Original-runtime Wine captures establish the expected visible sequences.
Browser and native port captures demonstrate the implementation. Native
Windows parity remains unclaimed unless separately established.

### Required verification

The implementation plan must select exact scoped commands, including:

- `cargo test -p rebellion-windowing`;
- relevant `rebellion-render` and `rebellion-app` tests;
- canonical WASM compilation and packaging;
- the interface-audit validators and changed JSON validation;
- scoped formatting and Clippy for touched files;
- scoped `cargo mutants` for pure kernel files;
- reachability checks for every migrated production route;
- muted browser acceptance with retained screenshots and diagnostics.

Workspace failures unrelated to the change remain named baseline failures and
do not become green by omission.

## Scope boundaries

### Included

- original strategic top-level window lifecycle;
- cross-family focus and z-order;
- documented strategic modality;
- pointer and keyboard ownership;
- pointer capture needed by migrated top-level interactions;
- close and restoration semantics;
- Battle Summary's modal participation;
- back-integration of existing strategic window families;
- removal of superseded routing exceptions;
- audit and evidence updates for the claimed cells.

### Deferred

- extracting reusable `CoolDragList`, `CoolStringField`, `CoolTabControl`,
  `CoolStrobeButton`, or other visual-control analogues;
- visual redesign or responsive behavior beyond preserving accepted surfaces;
- unrelated window content, missing scrollbars, placement, or geometry defects;
- general desktop-window features not used by Rebellion;
- tactical command-panel focus outside Battle Summary;
- multiplayer chat behavior except where an existing Message surface needs the
  shared ownership contract;
- serialization of UI state;
- native-Windows evidence tooling.

Deferred control extraction should be proposed separately when repeated Rust
implementations and a recovered original contract identify a stable boundary.
It must not be smuggled into this feature as opportunistic refactoring.

## Documentation and audit updates

The completed feature updates together:

- `ghidra/notes/strategic-windowing-toolkit.md` and cited `.c` files;
- `agent_docs/architecture.md` with the new crate and dependency direction;
- `agent_docs/roadmap.md` with the verified feature boundary;
- the affected interface-audit JSON and Markdown;
- `known-deviations.md`, retiring only deviations actually removed;
- a dated evidence record containing commands, results, original captures,
  port captures, hashes, failures, and remaining limitations;
- this plan's status and completion record.

No proprietary executable, original asset, generated proprietary pack, Wine
prefix, credential, or session material is committed.

## Delivery policy

Work remains on `feat/strategic-window-modality-focus`, based on current
`origin/main`. Each bead produces a focused commit on that branch. The branch
is pushed only to the `will-forster` fork. No pull request is opened without
first telling the user and receiving the expected handoff direction.

The implementation is not complete merely because the crate exists. It is
complete only when the relevant production windows use one authority, the
duplicated global policies are removed, the tests and evidence gates pass, and
the audit records state the remaining boundary honestly.

## Design review questions

Review of this specification should focus on four decisions:

1. Is the kernel boundary narrow enough to avoid a full UI rewrite while still
   owning every behavior responsible for the current focus defects?
2. Does every strategic window that can overlap or receive blocked input enter
   the migration matrix before acceptance?
3. Does the research gate prevent egui convenience behavior from being
   mislabeled as original parity?
4. Is the single-authority rule strict enough to prevent a permanent hybrid of
   the new manager and legacy special cases?

After this document is approved, a separate implementation plan will name
files, APIs, failing tests, commit boundaries, verification commands, and the
dependency graph used to create local beads.
