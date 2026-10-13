# Strategic Windowing Toolkit Implementation Plan

> **For future implementers:** Execute the reviewed plan task-by-task using the applicable implementation workflow. This document does not authorize implementation or delegation. Steps use checkbox (`- [ ]`) syntax for tracking.

**Status:** Active implementation. Tasks 0 through 5A are merged. The
2026-10-12 combined routing/coordinator slice implements Task 4 and the shared
coordinator, router, holds, and legacy gate from Task 5B in one review unit.
Backend event normalization finishes alongside feature-family migration,
beginning with Task 6 after that gate lands.

**Baseline:** Inspected worktree `/data/tmp/open-rebellion-strategic-window-modality-focus`, commit `f0648ef0e6b1f832a9f7643ccd6321e9f7ea5b52`. Revalidate before execution.

**Goal:** Add a renderer- and platform-independent strategic-window kernel, migrate every existing strategic surface to it as the sole authority for identity, ordering, focus, modality, capture, and input routing, and prove the recovered original behavior without overstating unresolved runtime evidence.

**Architecture:** A new std-only `rebellion-windowing` crate will hold semantic window records and produce immutable per-frame routing snapshots. `rebellion-render` will translate those decisions into current-frame hit regions, per-event input permits, egui focus requests, and a transparent modal input barrier while continuing to own all window contents. A prerequisite adapter experiment must prove exact paint and hit order on locked egui 0.31.1; repeatedly raising every Area is not an implementation of arbitrary stack order. `rebellion-app` will own the manager, combine its route with the existing non-window game-mode locks, and apply feature actions after the current snapshot has finished routing the frame.

**Tech Stack:** Rust 2021 workspace, standard-library collections, egui through `egui-macroquad`, macroquad, existing Rust unit/integration tests, cargo-mutants, the existing WASM package path, and Playwright-based interface-parity scripts.

**Spec:** `ghidra/notes/strategic-windowing-toolkit.md` (behavioral authority), governed in scope by `docs/plans/2026-10-10-feat-strategic-window-modality-and-focus.md`, with acceptance recorded by `docs/qa/2026-09-10-interface-parity-audit/2026-10-06-manual-window-checklists.md`.

**Latest evidence:**
[`2026-10-12-strategic-window-routing-coordinator.md`](../qa/2026-09-10-interface-parity-audit/evidence/2026-10-12-strategic-window-routing-coordinator.md)
records immutable route/reconciliation behavior and the production
coordinator boundary. It does not promote visual acceptance cells.

## Global Constraints

- `rebellion-windowing` is a std-only, ephemeral UI-state crate. It must not depend on `rebellion-core`, `rebellion-render`, `rebellion-app`, egui, macroquad, assets, serialization, the filesystem, audio, or platform APIs.
- Public domain concepts are semantic window kinds, opaque session identities, ownership, visibility, order, focus, modality, and bounded pointer capture. Do not expose HWNDs, Win32 message numbers, intrusive tree nodes, or egui `Order` values.
- Identity/existence, bottom-to-top order, twelve-entry presentation history, focused top-level/control, modal boundary, and pointer capture remain separate state. No collection may silently stand in for two of them.
- Route one frame's keyboard and pointer events against one immutable `RoutingSnapshot`. Applying `Close` during that frame affects only the next snapshot, so the closing event never falls through.
- A visible strategic modal blocks all lower strategic input, including clicks outside its rectangle and cockpit hotkeys. Its own controls and eligible owned transients remain usable; its blocked owner may still paint. Modality is not lifetime pointer capture.
- A registered window is not destroyed because it failed to paint. Reconciliation distinguishes explicitly absent feature state from temporarily unpaintable or off-scene content; a missing modal remains a fail-closed blocker until an explicit lifecycle action closes it.
- Duplicate-open behavior is declared per family: Finder/Message/Encyclopedia singletons and duplicate Sector opens are no-ops; detail windows restore/raise; contextual dialogs use an explicit context key rather than a generic raise rule.
- Parent close atomically removes actual lifecycle descendants and clears descendant focus/capture before selecting a focus target outside that subtree. A navigation source or return route is not automatically a lifecycle owner. Task 0 resolves the specification/source conflict about detail-window owners before that policy is implemented.
- The presentation rail is a maximum-12 minimized-detail history, not the complete window stack and not a focus stack.
- Existing feature modules continue to own tabs, rows, text, selections, scroll state, assets, and game queries. The toolkit does not become a desktop framework or a replacement for egui controls.
- Keep Alliance and Empire on the same windowing policy path. Faction selects content/art only.
- Keep event screens, briefing locks, fixture setup holds, pause-alert geometry, targeting, and game-mode transitions explicit in the application. Whole-frame holds and rectangle-only base children are distinct; do not turn a pause-alert rectangle into a full strategic keyboard lock.
- Do not add public kinds for native type 7 or the two unidentified bit-0 classes. Their semantics remain unresolved.
- System Finder policy is included, but implementing its missing port content is outside this plan. Battle Results adopts the common policy in the tactical scene without claiming the unavailable original-runtime capture.
- Each source-backed behavior test names the observable behavior and cites the relevant `ghidra/notes/strategic-windowing-toolkit.md` section or original helper in a test comment.
- Every automatable behavior slice starts with a test that is observed failing for the intended reason. Documentation and evidence slices use objective checks rather than artificial red/green tests.
- Preserve proprietary assets and Wine captures outside Git. Never commit `data/base`, capture directories, or generated proprietary packs.
- Keep `.beads/` local and uncommitted. This document proposes beads only; bead creation happens after plan approval.
- Use the actual implementing agent's required co-author trailer. Do not hard-code another agent's identity into commits.
- Do not add generalized owner-subtree modality without supporting evidence. Menus use an explicitly labeled outside-press dismissal rule; Rename is a focused control within its detail window, never a registered top-level.
- No event may be accepted by two unrelated surfaces. Eligibility, topmost hit ownership, key consumption, and cross-frame gesture ownership are separate checks.
- Every abbreviated `cargo ...` command in this plan must be executed as `env PATH=/home/will/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin cargo ...` on this host; commands that print the prefix spell out the same rule.

## Review Focus

1. **Same-event fallthrough after close:** `routing::escape_close_keeps_the_old_modal_keyboard_lease_until_the_next_snapshot` and `routing::outside_release_that_closes_a_modal_cannot_activate_the_exposed_cockpit` in Task 4 must prove the old route is retained.
2. **Missing modal content:** `routing::an_unpainted_strategic_modal_is_retained_as_a_fail_closed_blocker` in Task 4 must prove renderer failure cannot expose the cockpit.
3. **Owner teardown with descendants and capture:** `lifecycle::closing_an_owner_removes_descendants_and_clears_focus_and_capture_before_restoration` in Task 3 must assert exact close order and the restored target.
4. **Family-specific duplicate behavior:** `open_order::duplicate_sector_and_singleton_opens_preserve_order_while_detail_reopen_raises` in Task 2 must exercise all three policies in one sequence.
5. **Focused child bubbling under modality:** `routing::handled_text_stays_with_the_control_and_unhandled_keys_stop_at_a_strategic_modal` in Task 4 and the browser journey in Task 12 must prove typing never activates cockpit hotkeys.

---

## Definition of Done

The implementation is complete only when all of the following are true:

- `rebellion-windowing` can be tested without a renderer, game world, OS window, or asset tree.
- Its manager deterministically implements open/no-op/raise, ownership, close cascade, minimize/restore, twelve-entry presentation history, focus, temporary capture, reasoned reconciliation, modal eligibility, and immutable routing snapshots.
- All currently implemented strategic families use manager-issued `WindowId` values and the shared snapshot for cross-family order and input. Their private vectors/options remain content stores only. Fleet detail identity is `(System DatId, Fleet kind)`; selection of a fleet is content.
- No strategic feature module independently calls `egui::Context::move_to_top`; the single adapter realizes order.
- `FrameKeyboardOwner` is deleted, and `GalaxyMapState::pointer_blocked` / `strategic_input_enabled` are derived from the snapshot plus explicitly named non-window locks rather than a hand-maintained list of window families.
- Status, available Finders, Message Index, Encyclopedia, Build Selection, Mission creation, Move confirmation, Battle Alert, and Battle Results/Summary enter the common strategic-modal boundary. Scrap confirmation is clearly labeled as a port policy, not recovered original evidence.
- Sector duplicate opens no longer reorder the Sector store; details restore/raise; Finder/Message/Encyclopedia duplicate commands leave order and focus unchanged.
- A parent close removes actual owned children; close-triggering keyboard/pointer input never reaches the newly exposed surface in the same frame or through a later orphan release; missing modal rendering fails closed. A Sector close must not cascade details solely because a Sector opened them.
- Alliance and Empire pass the same unit, integration, and browser journeys.
- The workspace compiles, scoped tests and mutation gates pass, WASM packages, ledger validation passes, and both native and packaged-browser journeys pass on both factions with inspected artifacts. A missing native run is an outstanding delivery gate, not a browser pass. The early adapter proof must pass first-frame and warm-frame behavior.
- Audit records distinguish automated port proof, retained Wine observations,
  bounded original-runtime gaps, and still-missing native-Windows evidence. No
  audit cell is accepted solely because this architecture landed.

## Scope

### Included

- A new shared crate and its public lifecycle/routing contract.
- A centralized policy table for every existing strategic window, dialog, transient, and the two tactical strategic modals.
- Migration of Sector; System/Manufacturing; Fleet; Defenses; Missions detail; Fleet/Troop/Personnel Finder; Message Index; Encyclopedia; Status; Build Selection; Mission creation; Move confirmation; Scrap confirmation; object/agent/game-speed/GID menus; in-place rename control focus; Battle Alert; and Battle Results/Summary.
- Central egui hit registration, order realization, focus reporting, and transparent modal pointer blocking.
- Application ownership, immutable frame routing, explicit application locks, and deterministic reconciliation.
- Unit, integration, browser, packaging, audit, roadmap, evidence, and progress updates required by the repository workflow.

### Non-goals

- Recreating the native `Cool*` class hierarchy, Win32 dispatch, HWND registries, native message queues, or intrusive trees.
- Implementing System Finder content, missing Message content, missing mission behavior, or other feature completeness unrelated to window lifecycle/routing.
- Redesigning any window's pixels, layout, text, rows, search behavior, or game commands.
- Persisting window state in saves/replays or stabilizing `WindowId` across sessions.
- Solving non-strategic main-menu, setup, credits, multiplayer, or save/load panel navigation.
- Rewriting the full-screen `GameMode::GameOptions` surface; the snapshot gates the strategic F1 command that enters it, while its separate mode/input implementation remains unchanged.
- Claiming native-Windows parity or claiming original-runtime behavior for
  Battle Alert or Battle Results beyond their retained provisional evidence.
- Publishing a general-purpose crate outside this workspace.

## Specification and Evidence References

Executors must read these before changing code:

- `ghidra/notes/strategic-windowing-toolkit.md` — normative behavior and confidence labels.
- `ghidra/notes/strategic-windowing-toolkit.instructions.txt` plus its ten cited bounded decompiles — exact static evidence.
- `ghidra/notes/strategic-windowing-runtime-evidence.md` — sanitized retained
  Wine observations, artifact hashes, and limitations. The proprietary local
  captures and complete manifest remain uncommitted.
- `docs/plans/2026-10-10-feat-strategic-window-modality-and-focus.md` — architectural scope and migration intent.
- `docs/qa/2026-09-10-interface-parity-audit/2026-10-06-manual-window-checklists.md` — audit cells `CMD-04-C005`, `C016` through `C020`.
- `docs/qa/2026-09-10-interface-parity-audit/known-deviations.md` — temporary differences that must not be silently retired.
- `AGENTS.md`, the contribution workspace `/data/projects/open-rebellion/AGENTS.md` (a temporary worktree does not place that file at `../AGENTS.md`), `agent_docs/architecture.md`, `agent_docs/agent-tooling.md`, and `agent_docs/roadmap.md` — contribution, test, evidence, and documentation rules.

Evidence labels in code comments and records retain their established meanings:

- `fact:` for direct manual/decompile/instruction evidence.
- `runtime:` for repeatable original behavior observed under Wine.
- `port:` for the chosen Rust mechanism or a safety boundary supported by visible behavior.
- `unknown:` for bounded unresolved native detail.

## Current Implementation and Exact Seams

The code has useful feature boundaries but no shared window authority:

- `crates/rebellion-app/src/main.rs` creates all strategic states, snapshots the two-case `FrameKeyboardOwner`, dispatches global shortcuts, assembles `GalaxyMapState::pointer_blocked` from family-specific rectangle/open checks, computes `strategic_input_enabled`, invokes every renderer in a fixed sequence, and hand-codes Escape precedence.
- `crates/rebellion-render/src/sector_window.rs` stores open sectors in a `Vec`; the last element is treated as focused. `open_for_system` currently calls `focus` on a duplicate, contrary to the newly recovered no-raise opener behavior.
- `crates/rebellion-render/src/system_window.rs` combines visible detail storage and a twelve-entry reference rail. It uses local vector order for focus and local rail order for presentation history.
- `fleet_window.rs`, `defenses_window.rs`, and `missions_window.rs` each own another local ordering vector.
- Finder modules hold one optional surface; Message state lives partly in the app; Encyclopedia is coordinated by `encyclopedia_surface::EncyclopediaSurfaceController`; contextual dialogs each own private optional state.
- Strategic modules independently select `egui::Order::{Middle, Foreground, Tooltip}` and call `ctx.move_to_top`, so cross-family order is partly call order.
- `GalaxyMapState::pointer_blocked` in `crates/rebellion-render/src/lib.rs` gates map input but knows nothing about a modal boundary or cross-family stack.
- `TacticalViewState` exposes `battle_alert_open()` and `over_command_center()` and renders battle alert/results outside the strategic egui window flow.
- `crates/rebellion-app/src/interface_test_fixture.rs` and `tools/interface-parity/*.mjs` provide deterministic UI scenarios, but there is no cross-family windowing-contract journey.

The migration deliberately preserves the existing content stores. Each stored top-level gains a `WindowId`; manager order replaces “last vector element is focused.” Feature actions still carry game intent, while window lifecycle actions become `WindowCommand` values applied by the app after the current snapshot has routed input.

Recent history explains two constraints on that migration. Commit `6d46bc3b` added the narrow Encyclopedia `FrameKeyboardOwner` specifically to stop same-frame hotkey fallthrough; its observable guarantee must move into the generic snapshot before the enum is deleted. Commits such as `e295d88c`, `99d561ac`, and `26355390` delivered Sector, Message/Battle Alert, and Status as feature-local modules; the plan preserves those content/action boundaries and extracts only their duplicated cross-window policy.

## Proposed Architecture

### Evidence and decision precedence

The specification governs intended behavior; the design governs scope.
Existing behavior is a compatibility baseline, not evidence of original parity.
A committed decompile that contradicts a specification summary is an explicit
research gate: record the exact call chain, propose the narrow correction,
and reconcile the authority before implementing the disputed behavior.
Do not quietly convert either a convenient hypothesis or an old test into fact.

Use four labels in task records:

- **Requirement:** specified observable behavior, with its source address/page.
- **Baseline:** behavior or structure inspected at the recorded Git revision.
- **Decision:** the Rust mechanism chosen to satisfy the requirement.
- **Gap:** missing corroboration, with a named blocking task or audit cell.

The kernel is std-only and contains `#![forbid(unsafe_code)]`.
No new external dependency, framework upgrade, persistence format, or renderer
backend is authorized by this plan.

### Dependency direction and responsibilities

```text
rebellion-windowing (std-only domain state)
       ↑                         ↑
rebellion-render           rebellion-app
       ↑                         │
       └─────────────────────────┘

rebellion-core → render/app data consumers, never → windowing
```

| Responsibility | Sole authority | Adapter responsibility |
|---|---|---|
| Semantic key and session identity | WindowManager | Translate system/sector DAT identity |
| Existence and destruction ownership | WindowManager | Prepare content before registration; apply teardown events |
| Visible order | WindowManager | Realize exact order, never derive it from family call order |
| Minimized history | WindowManager | Resolve labels, icons, and retained content |
| Focus and focused control | WindowManager | Map ControlId to stable egui widget ID |
| Modal eligibility | RoutingSnapshot | Install barrier and gate every input consumer |
| Pointer capture | WindowManager | Cancel the actual drag/held-button state on cancellation |
| Event consumption and physical gestures | FrameInputRouter | Normalize macroquad/egui inputs once |
| Geometry and pixels | Existing render modules | Collect current-frame regions before routing |
| Feature actions | Existing application handlers | Accept actions only with the producing event permit |
| Scene and application holds | StrategicWindowing | Suspend inactive records without destroying them |

`FrameInputRouter` lives in the render adapter, not the pure kernel. It owns
only the current event batch and bounded press/release bookkeeping, not a
second focus or window stack. The app owns one manager for the UI session.

### Identity corrected against the actual code

`FleetWindowState::open(world, system, ...)` and `OpenFleetWindow.system`
show that the Fleet window displays the fleets at a **system**. Therefore all
four detail families use `(kind, system DatId)`. A selected FleetKey or ship
index stays in content. Opening two fleets in one system selects different
rows in the same window, not two top-levels. Fleet movement does not silently
change a system window's key.

Sector uses `(Sector, sector DatId)`. Singleton Finders, Message, Encyclopedia,
and Build Selection use `(kind, None)`. Contextual dialogs use one stable token
per logical open, allocated by the coordinator; repeated painting never
allocates a token. The current one-at-a-time Status/confirmation content
contract is enforced in the app, not by mislabeling those kinds as recovered
native singletons.

No simulation-key conversion dependency is required. Namespace-tagged u64
subjects leave room for future session-only domains without adding them now.
WindowIds have private constructors, are monotonically allocated, and are
never reused within a manager lifetime. The coordinator retains the allocator
across campaign resets and clears all pending events; an old event can never
address a fresh campaign's window by accident.

### Ownership correction gate

The spec's detail rows say "Owning Sector". The following committed chain
supports a different interpretation:

1. `FUN_00429ce0` passes the strategic view as `param_6` to `FUN_004591d0`.
2. `FUN_004591d0` stores that parameter at Sector `+0x194`.
3. `FUN_0045aac0` looks up and inserts detail objects at
   `*(this + 0x194) + 0x6c` and passes `*(this + 0x194)` to their constructors.
4. The current Rust callers also open details through Finders and the rail,
   independently of an open Sector.

Task 0 must inspect the corresponding detail constructor/base owner assignment
and record the adjudication. The proposed correction is **strategic-root
lifecycle ownership**, with Sector as an opener/navigation context. Do not
implement Sector-to-detail cascade until this conflict is settled. If evidence
instead establishes Sector ownership, revise the opener/rail migration and
its tests before Task 7; do not silently create a hidden Sector to satisfy it.

The general cascade contract remains mandatory: actual owned menus and future
owned surfaces die with their owner. In-place Rename is a control, so clearing
the owning detail's content destroys it without a WindowId.

### Ordering policy without conflating modality

Use `Low`, `Normal`, and `Transient` semantic bands:

- Low: Sector surfaces, with new Sector insertion at the bottom of that band.
- Normal: modeless details and named strategic dialogs.
- Transient: the active popup menu family, above its invoking surface.

Modality is an independent policy. A normal modal opens above other normal
records; a normal modeless background open is inserted below the lowest modal
and does not steal focus. A user-origin raise below a modal is rejected.
These are labeled port ordering decisions pending broader runtime comparison.

The active modal is the highest **active-scene** visible strategic modal.
Eligible transient descendants of that modal may sit above it. A root-owned
menu or unrelated transient may not pierce the boundary merely because its
band is higher. Opening a modal closes/cancels incompatible transient menus
and capture through explicit coordinator transitions.

Owner constraints must agree with band constraints. Reject invalid combinations
before mutation. Raising an eligible owner moves its permitted transient
subtree as a unit; it cannot leave a child below its owner. There is no general
reparent operation in this scope.

### Lifecycle, focus, and history

The committed kernel states are `Visible` and `Minimized`. A requested close
is represented by an app-owned pending command and the retained immutable
frame lease until the post-routing commit. That is the explicit `Closing`
boundary: the old snapshot remains valid, while the next snapshot excludes
closed records. Do not add a second native-style deferred queue inside the
manager or return permissive success for unfinished commands.

A successful command returns one `WindowCommit`: its transition plus every
cleanup event. Consumers process cancellation before dropping content, then
closed records, then final focus realization. No family infers descendant
cleanup by scanning only the requested root.

Focus tracks one current target and one previous eligible top-level candidate.
Only the popup-menu family stores its own saved focus target, as supported by
`FUN_00442380`/`FUN_00442430`. Do not create a general saved-focus stack from
unused native `+0x10c` fields. Closing preserves focus if it already moved
outside the closing subtree. Otherwise use a valid menu-saved target where
applicable, then the previous eligible target, then topmost eligible surface,
then root. A disappeared control falls back to its surviving window.

Minimize retains all content including tabs, selections, expansion, and scroll.
Only history-capable detail windows minimize. Cancel capture and transient
children before hiding an owner; reject minimizing an owner with an active
modal child rather than hiding a modal into an invisible blocker. Restore
removes the history entry and raises/focuses in the proper band. History has
exactly 12 maximum entries; a thirteenth removes the oldest. Closing the
corresponding content record on eviction is a **port preservation decision**,
not a stronger original-runtime claim.

The rail reads one snapshot of history. A click resolves the displayed slot
into a WindowId immediately; never enqueue only a mutable slot index that could
refer to a different window at commit time.

### Reconciliation: content lifecycle is not paint success

Replace an unqualified `rendered_ids` comparison with explicit reports:

- `Ready`: live content is available; a paint receipt is expected if active and
  visible, not merely because minimized content remains live.
- `Unpaintable`: content still exists but art or presentation failed; preserve
  the record and input boundary, report the failure.
- `Absent`: the feature explicitly confirms its content was removed; modeless
  reconciliation may close it. A modal still requires an explicit close action.
- `Suspended`: intentionally outside the current scene; no paint is expected.

Reports are scoped to IDs from the old snapshot. Newly committed opens cannot
be considered missing. Unknown, duplicate, or foreign-session reports produce
a diagnostic and no implicit open/close. Reconciliation **never opens** a
window from a presence boolean. It may not silently promote content stores
into a competing existence authority.

The coordinator applies cleanup receipts to every affected content store after
commands and reconciliation. `retain_registered` may assert the invariant or
remove confirmed dead content; it must not reset the manager from a hash-map
iteration. After each batch, manager records and content IDs agree, except for
explicitly documented unavailable/suspended states.

## Public Kernel Interfaces

The following declarations are interface contracts, not implementation bodies.
Introduce each command only with its owning task's implementation/tests; no
committed stub returns `Ok(Unchanged)` for unimplemented behavior.

```rust
// identity.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowId(/* private NonZeroU64 */);
impl WindowId { pub const fn get(self) -> u64; }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubjectId { pub namespace: u32, pub value: u64 }
impl SubjectId { pub const fn new(namespace: u32, value: u64) -> Self; }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ControlId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WindowKind {
    Sector, System, Fleet, Defenses, Missions,
    SystemFinder, FleetFinder, TroopFinder, PersonnelFinder,
    MessageIndex, Encyclopedia, Status, BuildSelection, MissionDialog,
    MoveConfirmation, ScrapConfirmation, BattleAlert, BattleResults, Menu,
}
impl WindowKind { pub const ALL: [Self; 19]; }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowKey { pub kind: WindowKind, pub subject: Option<SubjectId> }
impl WindowKey {
    pub const fn singleton(kind: WindowKind) -> Self;
    pub const fn contextual(kind: WindowKind, subject: SubjectId) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowOwner { StrategicRoot, Window(WindowId) }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FocusTarget { pub window: WindowId, pub control: Option<ControlId> }
```

The 19-kind inventory deliberately excludes Rename, unknown native classes,
cockpit widgets, and a separate GID-menu top-level. The GID root/submenus are
parts of the one active Menu presentation; their original family membership
remains a named evidence gap.

```rust
// policy.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StackBand { Low, Normal, Transient }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenPlacement { BottomOfBand, TopOfBand, AboveOwner }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExistingOpenPolicy { NoOp, RaiseRestore }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputPolicy { Modeless, StrategicModal, DismissOnOutsidePress }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresentationPolicy { None, History }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusOnOpen { Preserve, Window }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cardinality { Singleton, Subject { max_open: Option<usize> } }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusRestoration { PreviousEligible, SavedMenuTarget }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowPolicy {
    pub cardinality: Cardinality,
    pub band: StackBand,
    pub placement: OpenPlacement,
    pub existing: ExistingOpenPolicy,
    pub input: InputPolicy,
    pub presentation: PresentationPolicy,
    pub focus_on_open: FocusOnOpen,
    pub restoration: FocusRestoration,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowSpec {
    pub key: WindowKey,
    pub owner: WindowOwner,
    pub policy: WindowPolicy,
}
```

Policies are declared once in the render adapter's exhaustive table and tested
against the source matrix. The kernel validates key shape and capacity and
stores the declared policy. Two opens of the same kind with inconsistent
policies are rejected, so a caller cannot bypass singleton/capacity limits
with a different policy value. Per-open coordinates and subjects are not policy.
Sector column selection remains in Sector content; the kernel enforces max two
and an atomic replacement boundary without knowing galaxy halves.

```rust
// manager.rs
pub const PRESENTATION_HISTORY_CAPACITY: usize = 12;
pub struct WindowManager { /* BTreeMap registry + key index; separate Vec order/history */ }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowCommand {
    Open(WindowSpec),
    Replace { old: WindowId, new: WindowSpec },
    Activate(WindowId),
    Raise(WindowId),
    Minimize(WindowId),
    Restore(WindowId),
    Focus(FocusTarget),
    FocusRoot,
    CapturePointer(WindowId),
    ReleasePointer(WindowId),
    CancelCapture(CaptureCancelReason),
    Close(WindowId),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExistingEffect { NoOp, Raised, RestoredAndRaised }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnchangedReason { AlreadyTop, AlreadyVisible, AlreadyMinimized, NotCaptured, FocusAlreadySet }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionSubject { StrategicRoot, Window(WindowId) }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowTransition {
    Opened { id: WindowId },
    Existing { id: WindowId, effect: ExistingEffect },
    Replaced { old: WindowId, new: WindowId },
    Raised { id: WindowId },
    Minimized { id: WindowId, evicted: Vec<WindowId> },
    Restored { id: WindowId },
    Focused { previous: Option<FocusTarget>, current: Option<FocusTarget> },
    CaptureChanged { previous: Option<WindowId>, current: Option<WindowId> },
    Closed { root: WindowId, closed: Vec<WindowId>, restored_focus: Option<FocusTarget> },
    Unchanged { subject: TransitionSubject, reason: UnchangedReason },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloseReason { Requested, OwnerClosed, Replaced, Evicted, ContentRemoved, SessionReset }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureCancelReason { ModalOpened, OwnerClosed, Minimized, SceneChanged, HoldStarted, FocusLost, SessionReset }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowEvent {
    CaptureCancelled { window: WindowId, reason: CaptureCancelReason },
    Closed { window: WindowId, key: WindowKey, reason: CloseReason },
    FocusChanged { previous: Option<FocusTarget>, current: Option<FocusTarget> },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowCommit {
    pub transition: WindowTransition,
    pub events: Vec<WindowEvent>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowError {
    IdExhausted,
    UnknownWindow(WindowId), UnknownOwner(WindowId), InvalidOwnerCycle,
    InvalidPlacement, SubjectMismatch(WindowKey), CardinalityExceeded(WindowKind),
    KindPolicyMismatch(WindowKind), InvalidReplacement(WindowId),
    DuplicateKeyWithDifferentOwner(WindowKey), DuplicateKeyWithDifferentPolicy(WindowKey),
    NotMinimizable(WindowId), OwnerHasModalChild(WindowId),
    IneligibleFocus(FocusTarget), RootBlockedBy(WindowId),
    IneligibleCapture(WindowId), CaptureOwnedBy(WindowId),
    BlockedByModal { window: WindowId, modal: WindowId },
}
impl WindowManager {
    pub fn new() -> Self;
    pub fn apply(&mut self, command: WindowCommand) -> Result<WindowCommit, WindowError>;
    pub fn id_for(&self, key: WindowKey) -> Option<WindowId>;
    pub fn begin_frame(&self) -> RoutingSnapshot;
    pub fn set_active_windows(&mut self, active: &std::collections::BTreeSet<WindowId>)
        -> Result<Vec<WindowEvent>, WindowError>;
    pub fn reconcile(&mut self, frame: &RoutingSnapshot, reports: &[ContentReport])
        -> ReconcileReport;
    pub fn reset_session(&mut self) -> Vec<WindowEvent>;
    pub fn validate(&self) -> Result<(), InvariantViolation>;
}
```

`Replace` validates the complete new spec, capacity after removal, ownership,
and ID allocation before touching the old subtree. It rejects a new owner in
the subtree being replaced. Failure leaves old content, key index, order,
history, focus, capture, and allocator unchanged. Success emits child-first
teardown events and the new ID in one commit. Use it for third-Sector
replacement, menu replacement, and one-at-a-time contextual dialog replacement.
No intermediate snapshot can see three Sectors or a partially replaced menu.

`Activate` means a routed pointer press raises/focuses its owner. `Focus` alone
does not reorder. An already-top `Raise` can still change focus; return
`Unchanged(AlreadyTop)` only when both order and focus already match. A duplicate
`Open(NoOp)` changes neither. Stale closes from two callbacks are handled by
the coordinator's once-per-event admission, not by silently ignoring every
unknown ID in the kernel.

Every error is atomic. Preflight all affected keys and links before committing.
The manager never logs, serializes, calls egui, or panics on a user's command.
`reset_session` returns all cancellation/close events and empties records,
history, focus, and capture while preserving monotonic allocation.

```rust
// snapshot.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteTarget { StrategicRoot, Window(FocusTarget), BlockedBy(WindowId) }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BubblePolicy { ToRoot, StopAtWindow }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyboardRoute { pub target: RouteTarget, pub bubble: BubblePolicy }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointerRoute { pub target: RouteTarget }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentStatus { Ready, Unpaintable, Absent, Suspended }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContentReport { pub window: WindowId, pub status: ContentStatus }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReconcileReport {
    pub removed: Vec<WindowId>,
    pub blocking_unpainted: Vec<WindowId>,
    pub invalid_reports: Vec<WindowId>,
    pub events: Vec<WindowEvent>,
}
#[derive(Debug, Clone)]
pub struct RoutingSnapshot { /* immutable owned state, independent of manager */ }
impl RoutingSnapshot {
    pub fn paint_order(&self) -> &[WindowId];
    pub fn presentation_history(&self) -> &[WindowId];
    pub fn key(&self, id: WindowId) -> Option<WindowKey>;
    pub fn owner(&self, id: WindowId) -> Option<WindowOwner>;
    pub fn focused(&self) -> Option<FocusTarget>;
    pub fn capture_owner(&self) -> Option<WindowId>;
    pub fn keyboard_route(&self) -> KeyboardRoute;
    pub fn pointer_route(&self, hit_windows: &std::collections::BTreeSet<WindowId>) -> PointerRoute;
    pub fn active_modal(&self) -> Option<WindowId>;
    pub fn is_visible(&self, id: WindowId) -> bool;
    pub fn allows_input(&self, id: WindowId) -> bool;
    pub fn pointer_allows_root(&self) -> bool;
    pub fn restrict_to(&self, active: &std::collections::BTreeSet<WindowId>) -> Self;
}
```

`allows_input` means modal/visibility eligibility only. It is **not** permission
for every eligible overlapping window to consume a pointer or key event.
`pointer_route` selects exactly one topmost hit, or active capture regardless
of hit position, or a blocker/root. The renderer uses the selected route for
all custom raw-input paths as well as egui responses.

`restrict_to` creates an owned scene view before routing begins. Inactive
records contribute neither hits, focus, capture, nor modal boundaries. If the
old focused target is inactive, derive an eligible active modal/topmost/root
route for that scene without destroying the dormant record. The coordinator
cancels cross-scene capture explicitly before taking this view. Scene membership
is frozen for the frame, never inferred from what happened to paint.

`InvariantViolation` is a diagnostic value with an explicit `InvariantKind`
and affected IDs; it is not a stringly typed user-command error:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvariantKind {
    RegistryKeyMismatch, DuplicateOrder, MissingVisibleOrder,
    Cardinality, OwnerMissing, OwnerCycle, BandOrder, ChildBelowOwner,
    InvalidActiveMembership, IneligibleFocus, IneligibleCapture,
    HistoryOverflow, DuplicateHistory, VisibleInHistory, MinimizedWithoutHistory,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvariantViolation {
    pub kind: InvariantKind,
    pub windows: Vec<WindowId>,
}
```

The validator covers duplicate registry/key/order membership, cardinality,
invalid owners/cycles, incompatible bands, child-below-owner, invisible focus,
control-without-focused-window, ineligible capture, history overflow/duplicates,
visible-history intersection, and any minimized record missing from history.
Each case has a corrupt-private-state unit test and a public-command regression.

## Renderer and Application Interfaces

### Event ownership and consumption

The adapter uses compact frame-local event tokens to relate a physical input
to all copies visible through macroquad/egui. It does not add a network or
replay protocol. Keyboard classes may aid dispatch, but classification alone
never proves that a child did not consume an F-key or Alt chord.

```rust
// rebellion-render/src/windowing_input.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct InputEventId { pub frame: u64, pub ordinal: u32 }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputDisposition { Handled, Unhandled }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputRecipient { Root, Window(WindowId), Blocked }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscapeRoute { Control(FocusTarget), Window(WindowId), Root, Blocked }

pub struct FrameInputRouter { /* event permits and bounded gesture records */ }
impl FrameInputRouter {
    pub fn begin_frame(&mut self, frame_number: u64, snapshot: &RoutingSnapshot);
    pub fn pointer_recipient(&self, event: InputEventId) -> InputRecipient;
    pub fn keyboard_recipient(&self, event: InputEventId) -> InputRecipient;
    pub fn report_key(&mut self, event: InputEventId, owner: InputRecipient,
                      result: InputDisposition);
    pub fn root_may_handle(&self, event: InputEventId) -> bool;
    pub fn cancel_window(&mut self, id: WindowId);
    pub fn reset_session(&mut self);
}
```

The collecting constructor/egui bridge accepts the existing local input types;
its exact backend hook is frozen by Task 5A's prototype before consumer changes.
It must satisfy the event matrix in the execution contracts below. Do not
invent a stable public ABI for OS scan codes, text composition, or native HWNDs.

Keyboard delivery is child → owning top-level → permitted ancestor/root,
stopping on `Handled` or a modal/transient boundary. Only that chain may see the
event; an unrelated top-level never gets it. The root dispatch runs **after**
child/top-level disposition, replacing the current early global-shortcut block.
A close, confirm, cancel, or opening action consumes its trigger. TextEdit
in-place mutation must itself be gated, not merely the resulting game action.

All pointer actions use event-local coordinates, not only the final cursor
position. A captured drag owns motion/release outside its bounds. Release
without a permitted matching press cannot click a newly exposed control.
If modal/hold/close cancels a drag, clear renderer drag payload and suppress the
orphan release until physical button-up. This applies across frame boundaries.
Wheel input targets the top eligible hit and cannot scroll both overlapping lists.

### Surface and content boundary

```rust
// rebellion-render/src/windowing.rs
pub const SUBJECT_SECTOR: u32 = 1;
pub const SUBJECT_SYSTEM: u32 = 2;
pub const SUBJECT_COMMAND_CONTEXT: u32 = 3;
pub const SUBJECT_BATTLE: u32 = 4;

pub fn system_subject(world: &GameWorld, key: SystemKey) -> Option<SubjectId>;
pub fn sector_subject(world: &GameWorld, key: SectorKey) -> Option<SubjectId>;
pub const fn policy_for(kind: WindowKind) -> WindowPolicy;
pub const fn singleton_spec(kind: WindowKind, owner: WindowOwner) -> WindowSpec;
pub const fn contextual_spec(kind: WindowKind, subject: SubjectId,
                             owner: WindowOwner) -> WindowSpec;

pub struct EguiWindowSurfaces { /* current frame's geometry/control/paint receipts */ }
impl EguiWindowSurfaces {
    pub fn begin_frame(&mut self);
    pub fn register_hit_rect(&mut self, window: WindowId, rect: egui::Rect);
    pub fn register_control(&mut self, target: FocusTarget, widget: egui::Id);
    pub fn hit_windows(&self, point: egui::Pos2) -> std::collections::BTreeSet<WindowId>;
    pub fn record_painted(&mut self, id: WindowId);
    pub fn rendered_ids(&self) -> std::collections::BTreeSet<WindowId>;
    pub fn allows_input(&self, frame: &RoutingSnapshot, window: WindowId) -> bool;
    pub fn install_modal_barrier(&self, ctx: &egui::Context, frame: &RoutingSnapshot,
                                 strategic_canvas: egui::Rect);
    pub fn realize(&self, ctx: &egui::Context, frame: &RoutingSnapshot);
}
```

A window may contribute multiple disjoint hit rectangles; registration unions
them rather than silently overwriting its first region. GID submenus use this
property. Invalid/non-finite geometry reports a diagnostic and never opens an
underlay route through an active modal. Transform logical 640×480 coordinates
once through existing `CockpitLayout`; do not introduce kernel geometry or
inclusive-edge rules that disagree with source-specific strict hit tests.

Register current geometry before map input and paint. Rects from the previous
frame are not authoritative after resize, movement, faction change, or restore.
A paint receipt is diagnostic only. Control IDs are stable within a window and
mapped to egui IDs centrally; missing control restoration falls back to window
focus rather than requesting an unrelated stale widget.

### Critical egui 0.31.1 adapter gate

The locked implementation of `Areas::move_to_top` inserts into
`wants_to_be_on_top`, a set. `Areas::end_pass` sorts by `(Order, membership)`;
it does not preserve the call order of repeated raise requests as a requested
permutation. `Area` also requests raising on interaction/new visibility.
Therefore the former algorithm "call move_to_top for every ID bottom-to-top"
is invalid as a general realization strategy.

Task 5A must prove one adapter strategy on the locked dependency **before any
family migrates**. Its preferred candidate is a single managed strategic host
with stable per-window egui ID scopes and child content UIs painted in snapshot
order; base cockpit/rail controls share the host so an empty full-screen Area
does not accidentally blanket them. Current feature geometry and bitmap draws
remain unchanged. Eligibility and per-event permits guard both built-in
controls and custom raw-input reads.

A smaller public-API adapter is acceptable only if it passes the identical
arbitrary-order, first-frame, focus, clipping, widget-state, and overlap tests.
Do not chain nested `set_sublayer` calls: the locked API documents one level
only. Do not call `reset_areas` each frame, rotate widget IDs to fake rank,
patch egui, or upgrade it without a separate reviewed decision.

Record the selected mechanism, exact helper signatures, and prototype evidence
in this plan before Task 5B begins. This is a bounded engineering gate, not
permission to migrate with an unproven assumption. If neither candidate meets
the contract, stop adapter integration, retain the kernel work, and revise the
architecture with the concrete failing experiment.

### Coordinator, transitions, and scenes

```rust
// rebellion-app/src/strategic_windowing.rs
pub struct StrategicWindowing { /* one manager, typed pending work, scene state */ }
#[derive(Debug)]
pub struct AppliedWindowCommand {
    pub command: WindowCommand,
    pub result: Result<WindowCommit, WindowError>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowScene { Galaxy, Tactical, Suspended }
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ApplicationInputLocks {
    pub event_screen: bool,
    pub briefing: bool,
    pub fixture_setup: bool,
    pub scene_transition: bool,
}
impl ApplicationInputLocks { pub const fn blocks_strategic_input(self) -> bool; }
impl StrategicWindowing {
    pub fn new() -> Self;
    pub fn begin_frame(&self) -> RoutingSnapshot;
    pub fn enqueue(&mut self, command: WindowCommand);
    pub fn apply_pending(&mut self) -> Vec<AppliedWindowCommand>;
    pub fn next_context_subject(&mut self) -> Result<SubjectId, WindowError>;
    pub fn reconcile(&mut self, frame: &RoutingSnapshot, reports: &[ContentReport])
        -> ReconcileReport;
    pub fn enter_scene(&mut self, scene: WindowScene) -> Vec<WindowEvent>;
    pub fn reset_session(&mut self) -> Vec<WindowEvent>;
}
```

During drawing, modules see immutable snapshots and emit intents; they never
obtain `manager_mut`. Preparation/commit adapters may accept `&mut WindowManager`
only after all frame input has routed. An Open intent prepares feature content
first, commits identity second, attaches the ID third. If preparation fails,
leave the prior surface unchanged. If content fails after committed open,
retain a fail-closed unavailable modal and explicit close path.

The queue's application order is deterministic. `Replace` is atomic, while
independent commands in a batch are individually committed and reported. A
failed command does not execute its paired gameplay action. Revalidation at
commit may reject stale world subjects without acting on a different object.
One physical confirm dispatch executes the gameplay command at most once.

The coordinator calls `set_active_windows` at the pre-frame scene boundary,
then constructs the snapshot. This kernel method knows only an explicit set of
registered IDs, never Galaxy/Tactical concepts. It validates unknown IDs before
mutation, cancels capture that becomes inactive, and repairs semantic focus
against the active set. `apply` uses the same active set for modal/focus/capture
eligibility; restricting only the snapshot would leave dormant modals incorrectly
blocking commands. New committed opens are active by default; the app defers
off-scene open intents until their scene is active. `restrict_to` is an additional
restrictive snapshot view, never a way to admit a command the manager rejects.
A visible dormant record remains in retained order but is omitted from paint and
input; history and registry invariants still apply to it. Reset clears membership.

`CancelCapture` is the explicit coordinator command for hold/focus-loss paths
that have no window close. It emits the same cancellation receipt as lifecycle
interruption; the router and feature payload must both receive that receipt.

Scene selection happens before snapshot construction. Galaxy IDs may remain
registered during tactical play but are omitted from the tactical scene view.
Alert/Results alone enter the strategic modal route in that scene; normal
battle input remains owned by the existing tactical controller. Do not force
root focus if an active modal rejects it. Return to Galaxy revalidates the
retained strategic content and focus before accepting new input.

New campaign, loaded campaign, faction replacement, or return that discards
campaign UI uses `reset_session` plus content/gesture/egui-focus cleanup. Game
Options temporarily suspends the scene without treating every window as absent.
All scene switches retain the current event's lease until dispatch completes.

### Frame data flow

1. Commit any explicitly scheduled pre-frame scene/hold changes; cancel old
   gestures if needed. Never use post-close state to reroute the old event.
2. Build one immutable manager snapshot and restrict it to the active scene.
3. Collect current geometry and freeze application holds/base-child hit regions.
4. Normalize the input batch once; assign each event one route/propagation chain.
5. Route owner/control input; retain `Handled` dispositions and gesture ownership.
6. Dispatch root shortcuts only for explicitly unhandled, permitted events.
7. Paint through the proven adapter in snapshot order; update content only with
   permits belonging to its window/control. Collect feature/window intents.
8. Commit pending intents, deliver cancellation/teardown/focus receipts to all
   affected content stores, then reconcile explicit content reports.
9. Assert manager/content agreement and emit changed-state diagnostics. New
   windows and new focus become eligible through the next frame's snapshot.

A restrictive hold that starts mid-frame may suppress remaining input; releasing
one never makes that frame more permissive. An ordinary modal opened by input
consumes the trigger; the retained frame lease remains the routing authority
for the rest of that batch. New-modal interaction starts next frame.

## Policy Matrix

| Family | Key / cardinality | Owner | Duplicate | Placement | Input / history |
|---|---|---|---|---|---|
| Sector | sector subject, max 2 | Root | NoOp | Low bottom | Modeless / none |
| System, Fleet, Defenses, Missions | system subject, unlimited | Root proposed; Task 0 gate | RaiseRestore | Normal top, below active modal when background-created | Modeless / 12-slot shared history |
| Four Finders | singleton | Root | NoOp | Normal top | StrategicModal / none |
| Message, Encyclopedia | singleton | Root | NoOp | Normal top | StrategicModal / none |
| Build Selection | singleton, manufacturing context stays in content | Root | NoOp | Normal top | StrategicModal / none |
| Status, Mission, Move | per-open context; app allows one current instance per family | Root or specifically traced owner | NoOp for same context | Normal top | StrategicModal / none |
| Scrap | per-open context | Root | NoOp for same context | Normal top | port: StrategicModal / none |
| Battle Alert, Results | battle subject with distinct kind | Root | NoOp for same battle/kind | Normal top | StrategicModal in active tactical scene / none |
| Object/Agent/Game Speed/GID menu | one active Menu singleton with content variant | Invoker | explicit atomic Replace | Transient above invoker | port: DismissOnOutsidePress / none |
| Rename | ControlId only | Existing detail window | existing editor rules | no stack entry | focused edit, no independent modality/history |

New opens normally take focus; background modeless creation below a modal
preserves it. `FocusRestoration::SavedMenuTarget` is used only for Menu;
other kinds use `PreviousEligible`. The absent System Finder gets a policy,
not a fake content surface or a passing family audit.

## Invariants and Error Handling

1. Identity, visible order, minimized history, focus, capture, and event claims
   have distinct storage and distinct meanings.
2. Every successful command preserves uniqueness, cardinality, owner and band
   constraints. Every rejected command is atomic, including allocation state.
3. The active scene excludes dormant windows before modality/focus is queried.
4. A single pointer event selects one top-level; keyboard follows one explicit
   chain and stops when handled. Eligibility alone is insufficient.
5. A cancelled gesture cannot deliver its release or drag payload elsewhere.
6. Close/minimize/replacement deliver complete cleanup receipts, including
   descendants and evicted presentations, before choosing replacement focus.
7. A NoOp duplicate does not reset content, origin, control focus, or capture.
8. Native, browser, egui, and macroquad input consumers obey the same route.
9. Unpaintable content does not imply destroyed content. A modal failure cannot
   expose the strategic underlay.
10. Errors are typed. Expected stale event rejection is diagnostic, not an
    unconditional debug panic; invariant corruption may assert in debug builds.
11. Diagnostic IDs are local UI identities, not persisted/wire identities.
12. A rollback never temporarily changes a migrated modal to permissive routing.

Use deterministic BTree collections for indexes/reports and explicit vectors
for order; never infer ordering from key sort. No benchmark claim is made.
Profile representative window counts and repeated open/close cycles at acceptance;
optimize only measured cost without weakening these contracts.

## File Map

### Create

| File | Responsibility |
|---|---|
| `crates/rebellion-windowing/Cargo.toml` | std-only library manifest and workspace lints |
| `crates/rebellion-windowing/src/lib.rs` | crate contract, re-exports, evidence-level documentation |
| `crates/rebellion-windowing/src/identity.rs` | semantic kinds/keys, opaque IDs, control/focus identities |
| `crates/rebellion-windowing/src/policy.rs` | renderer-independent ordering/input/open/presentation policies |
| `crates/rebellion-windowing/src/manager.rs` | records, commands, deterministic transitions, ownership, history, focus, capture |
| `crates/rebellion-windowing/src/snapshot.rs` | immutable route/paint snapshot, reconciliation report, invariants |
| `crates/rebellion-windowing/tests/open_order.rs` | identity, duplicate, band, and deterministic ordering contract |
| `crates/rebellion-windowing/tests/lifecycle.rs` | ownership, close cascade, focus, capture, minimize/history contract |
| `crates/rebellion-windowing/tests/routing.rs` | modality, pointer/keyboard route, no-fallthrough, fail-closed reconciliation |
| `crates/rebellion-render/src/windowing.rs` | policy table, subject namespaces, proven egui host/geometry adapter and modal barrier |
| `crates/rebellion-render/src/windowing_input.rs` | per-event admission, child/top-level/root consumption, gesture cancellation |
| `crates/rebellion-app/src/strategic_windowing.rs` | manager ownership, pending command boundary, context IDs, non-window locks |
| `tools/interface-parity/windowing-contract.mjs` | both-faction cross-family/modal/no-fallthrough browser journey |
| `docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-10-strategic-windowing-toolkit.md` | commands, hashes, screenshots, limitations, audit mapping |

### Modify

| File(s) | Responsibility of change |
|---|---|
| `Cargo.toml`, `Cargo.lock` | workspace member and dependency lock |
| `crates/rebellion-render/Cargo.toml`, `crates/rebellion-app/Cargo.toml` | direct path dependency on `rebellion-windowing` |
| `crates/rebellion-render/src/lib.rs` | export renderer adapter; consume centralized pointer eligibility |
| `crates/rebellion-app/src/main.rs` | own manager/snapshot, route input, enqueue/apply commands, delete legacy exception matrix |
| `crates/rebellion-app/src/interface_test_fixture.rs` | deterministic windowing scenarios and observation payloads |
| `crates/rebellion-render/src/sector_window.rs` | attach IDs, stop duplicate reorder, register surface/order/action |
| `crates/rebellion-render/src/system_window.rs` | attach IDs, delegate global order/history/focus, retain detail content |
| `crates/rebellion-render/src/fleet_window.rs` | attach IDs and delegate global lifecycle/order |
| `crates/rebellion-render/src/defenses_window.rs` | attach IDs and delegate global lifecycle/order |
| `crates/rebellion-render/src/missions_window.rs` | attach IDs and delegate global lifecycle/order |
| `crates/rebellion-render/src/fleet_finder.rs`, `troop_finder.rs`, `personnel_finder.rs` | singleton modal registration and shared focus/routing |
| `crates/rebellion-render/src/message_index.rs` | singleton modal registration and shared route |
| `crates/rebellion-app/src/encyclopedia_surface.rs` | preserve controller session/entry/return contract while adding window lifecycle |
| `crates/rebellion-render/src/encyclopedia.rs`, `encyclopedia_surface.rs` | preserve content painting and route focused controls through common permits |
| `crates/rebellion-render/src/status_window.rs` | contextual modal registration |
| `crates/rebellion-render/src/build_selection.rs` | singleton strategic-modal registration |
| `crates/rebellion-render/src/mission_dialog.rs` | contextual strategic-modal registration |
| `crates/rebellion-render/src/move_confirmation.rs` | Move recovered modal and Scrap port-modal registration |
| `crates/rebellion-render/src/agent_menu.rs`, `game_menu.rs`, `game_speed.rs`, `object_menu.rs`, `cockpit.rs` | one active menu presentation including GID submenus, base host integration, focus restoration |
| `crates/rebellion-render/src/tactical_view.rs` | expose stable battle surface/session identity and common modal actions |
| `tools/interface-parity/package.json`, `README.md`, `verify-production-exclusion.mjs` | named windowing browser command, documented prerequisites, and proof fixture bridges stay out of production |
| `agent_docs/architecture.md`, `agent_docs/roadmap.md` | shared crate boundary and completed migration checkpoint |
| audit `2026-10-06-surface-ledger.json`, `2026-10-06-audit-report.md`, `audit-report.json`, `screenshot-ledger.md`, audit `README.md`, checklist, `known-deviations.md` | evidence/status and screenshot-index changes only where gates actually pass |
| ignored local `progress.json`, if present | schema-preserving local checkpoint only; absent in inspected worktree; never stage |
| `ghidra/notes/strategic-windowing-toolkit.md`, conditionally | Task 0 source-supported detail-owner correction before lifecycle migration |
| this implementation plan and `docs/plans/INDEX.md` | completion status/link after implementation acceptance |

## Execution Rules and Dependency Graph

The implementation sequence is deliberately serial where application and
renderer integration overlap. This is not authorization to launch workers.
A task may be split into smaller commits along the named subgates; do not
combine a behavior change with an unrelated repair discovered during baseline.

```text
T0 baseline + ownership adjudication
 └─> T1 vocabulary
      └─> T2 open/order/atomic replacement
           └─> T3 lifecycle/history/focus/capture
                └─> T4 snapshot/routing/reconciliation
                     └─> T5A adapter feasibility proof
                          └─> T5B coordinator/event bridge
                               ├─> T6 Sector
                               │    └─> T7A details
                               │         └─> T7B Fleet gestures/controls
                               │              └─> T7C presentation rail
                               └─> T8 Finders/Message
                                    └─> T9 Encyclopedia/Status

T7C + T9 ─> T10 dialogs/transients ─> T11 tactical scenes
T6 + T7C + T8 + T9 + T10 + T11 ─> T12 cleanup/browser/package
T12 ─> T13 native acceptance ─> T14 evidence/documentation closure
```

T5A may prepare its reproduction while kernel work proceeds, but its production
adapter consumes the completed T4 contract. No family migration precedes its
passing gate. T7A cannot finalize lifecycle ownership before T0's decision.
T8 reuses the same integration files as T6; theoretical independence is not a
reason to make concurrent edits in this checkout.

For each behavior slice:

1. Add the named test to the specified test target/module.
2. Run the stated red command and retain the actual failure.
3. Confirm it failed for the intended behavior, not a missing asset, linker,
   tool, or unrelated workspace error. A compile failure is appropriate only
   when introducing the public vocabulary itself.
4. Implement the smallest coherent behavior needed by that slice.
5. Run the same command green, then the listed integration checks.
6. Review the diff, source labels, and state ownership before its atomic commit.

Do not land `#[ignore]` tests for future behavior, success-returning command
stubs, dead placeholder migration branches, or fabricated red transcripts.
A later-task command variant can be added in that task rather than temporarily
pretending the manager supports it. Commit descriptions below are future
implementation descriptions, not instructions to commit this planning edit.

## Task 0: Establish a Fresh Baseline and Resolve Detail Ownership

**Purpose:** Turn environment assumptions and the specification/source conflict
into explicit execution prerequisites before choosing lifecycle behavior.

**Files to read:**

- Workspace `/data/projects/open-rebellion/AGENTS.md` and repository `AGENTS.md`.
- `README.md`, `agent_docs/architecture.md`, `agent_docs/agent-tooling.md`.
- `agent_docs/roadmap.md` and the current audit `README.md`.
- The specification, its instructions file, and all directly cited decompiles.
- Local ignored `progress.json`, if present; it is absent in the inspected
  temporary worktree and must not be fabricated from an assumed template.
- `ghidra/notes/FUN_00429ce0.c`, `FUN_004591d0.c`, `FUN_0045aac0.c`, and the
  detail constructors/base-owner assignments called by the last helper.
- The original-game capture guide before any future capture activity.

**Files to modify in a future implementation slice:**

- The toolkit evidence Markdown named in the file map.
- `ghidra/notes/strategic-windowing-toolkit.md` only for a separately reviewed,
  source-supported correction to its detail-owner matrix, if confirmed.
- This plan's ownership decision and baseline fields when execution revalidates
  them. Do not rewrite unrelated historical evidence.

**Acceptance:** The exact starting revision, working-tree state, tool versions,
commands, existing failures, and owner interpretation are recorded. There is
no silent choice between a normative Sector owner and traced Root ownership.

- [ ] Run the read-only baseline commands in the verification section.
- [ ] Preserve all existing changes and confirm upstream/fork remote roles.
- [ ] Before implementation branching, fetch `origin --prune`; use a focused
  branch/worktree based on current `origin/main` under workspace policy.
- [ ] If this planning branch is reused, explicitly reconcile its base with
  current upstream without absorbing anyone else's untracked plan files.
- [ ] Record rustc/cargo/Node/Playwright/platform versions and locked egui version.
- [ ] Run the existing workspace checks before adding the new crate.
- [ ] Measure failures freshly; recent formatting/lint fixes make old baseline
  assertions insufficient.
- [ ] Trace the strategic-view pointer passed by `FUN_00429ce0` into Sector,
  stored at `+0x194`, and used by `FUN_0045aac0` for detail lookup/creation.
- [ ] Inspect the receiving detail/base constructor to distinguish the passed
  semantic owner from an incidental registry/context argument.
- [ ] Record the bounded evidence and either correct the specification's
  “Owning Sector” row to strategic root or retain the row with evidence that
  disproves the proposed correction.
- [ ] Require that decision before implementing detail close-cascade tests.
- [ ] Preserve the independent rule that actual owned transient descendants
  close with their owner; navigation origin is not sufficient ownership proof.

**Objective validation:** A reviewer can follow every owner assignment from
caller to stored owner without relying on this plan's inference alone. Baseline
logs identify actual commands and exit codes; missing runs say “not run.”
This is evidence work and does not need a synthetic failing unit test.

**Atomic commit:** `docs(windowing): record baseline and resolve detail ownership`
if the source-backed specification correction is necessary. Otherwise keep
local environment logs outside Git and include the concise decision record in
the first relevant implementation/evidence commit.

## Task 1: Introduce the Pure Kernel Vocabulary

**Purpose:** Establish semantic identities and orthogonal policy axes without
pulling renderer, simulation, or platform dependencies into the kernel.

**Create:**

- `crates/rebellion-windowing/Cargo.toml`.
- `crates/rebellion-windowing/src/lib.rs`.
- `crates/rebellion-windowing/src/identity.rs`.
- `crates/rebellion-windowing/src/policy.rs`.
- `crates/rebellion-windowing/tests/open_order.rs`.

**Modify:** Workspace `Cargo.toml` and generated `Cargo.lock` as required by the
new local member. Follow existing edition, license, and workspace-lint rules.

**Consumes:** The Public Kernel Interfaces and the 19-kind policy inventory.
**Produces:** Identity/key/policy types; no manager behavior is claimed yet.

**Acceptance:** IDs are opaque and comparable, subjects are namespace-safe,
keys distinguish singleton/contextual shape, and independent policies are
representable. The normal dependency tree contains only the new crate.

- [ ] Add `subject_namespaces_do_not_alias_equal_numeric_values`.
- [ ] Add `window_keys_distinguish_kind_and_subject`.
- [ ] Add `all_window_kinds_are_unique_and_cover_the_public_inventory`.
- [ ] Add `policy_axes_do_not_encode_modality_in_stack_band`.
- [ ] Run the red command before defining the missing public types.
- [ ] Implement the public vocabulary and documentation only.
- [ ] Keep raw `WindowId` construction private; do not expose fixed production
  IDs for Finder, Encyclopedia, or test fixture convenience.
- [ ] Use internal test helpers for malformed IDs when those are needed later.
- [ ] Keep `ControlId` local to its owning `WindowId`; equal child values across
  windows must not alias egui focus mappings.
- [ ] Verify no `serde`, world key, egui type, or OS dependency entered the crate.

**Red/green command:**

```bash
cargo test -p rebellion-windowing --test open_order
```

**Additional green checks:**

```bash
cargo check -p rebellion-windowing --all-targets
cargo tree -p rebellion-windowing --edges normal
cargo clippy -p rebellion-windowing --all-targets -- -D warnings
```

The compile-time red result is limited to introducing types. Later tasks must
show an assertion failure against real behavior wherever the interface exists.
There is no adjustable history-capacity configuration: the recovered contract
uses the named constant `PRESENTATION_HISTORY_CAPACITY = 12`.

**Atomic commit:** `feat(windowing): add semantic identities and policy types`

## Task 2: Implement Open, Ordering, and Atomic Replacement

**Purpose:** Make registry existence and deterministic visible order separate
and give replacements a safe transaction before Sector/menu migration.

**Create:** `crates/rebellion-windowing/src/manager.rs`.
**Modify:** Kernel `lib.rs` and `tests/open_order.rs`.
**Consumes:** Identity/policy types.
**Produces:** `WindowManager::new`, `id_for`, initial `apply` operations,
`WindowCommit`, transition/error vocabulary, and ordering validation.

**Acceptance:** Every key has at most one ID; IDs increase without reuse; bands
and owner constraints are stable; no-op opens do not touch focus or order;
failed opens/replacements leave all state and allocator unchanged.

Test one behavior at a time in this order:

- [ ] `fresh_opens_allocate_monotonic_nonzero_ids`.
- [ ] `lookup_order_is_independent_of_paint_order`.
- [ ] `subject_cardinality_and_singleton_shape_are_enforced`.
- [ ] `duplicate_sector_and_singleton_opens_preserve_order_while_detail_reopen_raises`.
- [ ] `existing_key_rejects_different_owner_or_policy_atomically`.
- [ ] `same_kind_cannot_bypass_capacity_by_changing_policy`.
- [ ] `low_band_insertions_do_not_raise_unrelated_normal_windows`.
- [ ] `background_detail_open_cannot_overtake_an_active_modal`.
- [ ] `activate_changes_order_and_focus_but_focus_alone_does_not`.
- [ ] `raise_already_top_still_repairs_different_focus`.
- [ ] `replacement_checks_capacity_after_removing_the_old_subtree`.
- [ ] `invalid_replacement_preserves_old_id_content_links_and_allocator`.
- [ ] `replacement_rejects_an_owner_inside_the_replaced_subtree`.
- [ ] `id_exhaustion_is_an_atomic_typed_error` using private-state setup.

**Red/green command:**

```bash
cargo test -p rebellion-windowing --test open_order
```

**Minimal implementation:**

- [ ] Use ordered maps for registry and key lookup plus a separate visible-order
  vector; never infer paint order from map iteration or allocation order.
- [ ] Validate owner/key/policy/capacity before allocating or mutating.
- [ ] Build the proposed replacement state before committing removals.
- [ ] Allocate fresh identity for a successful replacement; never repurpose the
  old ID to represent a different Sector or command context.
- [ ] Keep the new record below a blocking modal unless it belongs to that
  modal's eligible transient subtree.
- [ ] Define `Raise` and `Activate` errors for blocked/invisible targets.
- [ ] Introduce only implemented command variants at this checkpoint; complete
  the full declared enum as Tasks 3–4 land.
- [ ] Call private invariant checks in tests after every successful transition
  and compare the entire observable state after every failed transition.

**Regression checks:** `cargo test -p rebellion-windowing` and strict kernel
Clippy. Re-run the combined duplicate test after later focus/history work.

**Atomic commit:** `feat(windowing): implement deterministic opens and replacement`

## Task 3: Implement Lifecycle, History, Focus, and Capture

**Purpose:** Add close/minimize/restore behavior without turning any of these
state dimensions into an accidental alias for visible order.

**Create:** `crates/rebellion-windowing/tests/lifecycle.rs`.
**Modify:** Kernel `manager.rs`, `lib.rs`, and `tests/open_order.rs`.
**Consumes:** Atomic command/commit boundary.
**Produces:** Remaining lifecycle commands, cancellation/close/focus events,
fixed-capacity history, reset semantics, and validated focus/capture.

**Acceptance:** Descendants close child-first; no dead focus/capture/history
references survive; a thirteenth minimized detail evicts the oldest through
the same lifecycle teardown path; saved focus is used only by menus.

- [ ] Add `closing_an_owner_removes_descendants_and_clears_focus_and_capture_before_restoration`.
- [ ] Add `close_chooses_previous_eligible_survivor_outside_removed_subtree`.
- [ ] Add `menu_close_restores_saved_control_only_if_still_eligible`.
- [ ] Add `ordinary_window_close_uses_global_eligible_order_not_a_private_focus_stack`.
- [ ] Add `minimize_removes_visible_order_and_adds_history_once`.
- [ ] Add `thirteenth_minimized_detail_evicts_oldest_with_teardown_events`.
- [ ] Add `restore_removes_history_and_raises_without_reallocating_identity`.
- [ ] Add `reopening_minimized_detail_restores_and_raises`.
- [ ] Add `non_history_kind_cannot_minimize`.
- [ ] Add `minimizing_owner_with_modal_child_is_rejected_atomically`.
- [ ] Add `capture_is_exclusive_and_release_by_another_window_fails`.
- [ ] Add `strategic_modal_open_cancels_incompatible_capture_before_focus_changes`.
- [ ] Add `minimize_close_and_reset_cancel_capture_once`.
- [ ] Add `reset_clears_records_but_never_reuses_old_ids`.
- [ ] Add `focus_rejects_hidden_blocked_and_unknown_targets`.
- [ ] Add `focus_root_is_rejected_while_an_active_modal_blocks_it`.

**Red/green command:**

```bash
cargo test -p rebellion-windowing --test lifecycle
```

**Minimal implementation:**

- [ ] Traverse actual owner links to build a deterministic child-first close
  list; siblings use an explicitly stable order.
- [ ] Emit cancellation before content teardown, then close events, then final
  focus change; document receipt ordering in `WindowCommit`.
- [ ] Capture the previous eligible target before mutation and revalidate it
  after removals. Never restore a descendant of the closing subtree.
- [ ] Use menu-specific saved focus only when `SavedMenuTarget` requests it.
- [ ] Maintain history as minimized detail identities, oldest to newest.
- [ ] Keep labels/icons/content outside the kernel; an eviction reports an ID
  and close reason so the application can drop the matching content.
- [ ] Reject owner minimization that would hide an active modal child.
- [ ] Ensure command rejection preserves capture and focus as well as records.
- [ ] Retain the monotonic ID allocator across session reset to reject stale
  deferred callbacks from the discarded campaign.

**Regression checks:** Full kernel tests, especially Task 2 duplicate-open and
replacement tests. Public transition tests must inspect emitted event order,
not only the final record count.

**Atomic commit:** `feat(windowing): add lifecycle history focus and capture`

## Task 4: Implement Immutable Routes and Explicit Reconciliation

**Purpose:** Turn state into an immutable eligibility/route contract and make
missing paint fail closed without confusing it with deleted feature state.

**Create:** `crates/rebellion-windowing/src/snapshot.rs` and `tests/routing.rs`.
**Modify:** Kernel `manager.rs` and `lib.rs`.
**Consumes:** Complete lifecycle state.
**Produces:** `RoutingSnapshot`, route types, `ContentReport`,
`ReconcileReport`, and scene restriction.

**Acceptance:** Routes select one owner; modal eligibility is independent of
paint band; old snapshots remain valid after commit; absent paint never acts
as implicit modal dismissal; inactive scenes cannot block active input.

- [ ] Add `topmost_eligible_hit_wins_independent_of_hit_registration_order`.
- [ ] Add `modeless_eligibility_does_not_grant_all_windows_the_same_event`.
- [ ] Add `modal_blocks_root_inside_and_outside_its_hit_regions`.
- [ ] Add `modal_descendant_menu_is_eligible_but_unrelated_menu_is_not`.
- [ ] Add `capture_routes_motion_outside_owner_bounds`.
- [ ] Add `escape_close_keeps_the_old_modal_keyboard_lease_until_the_next_snapshot`.
- [ ] Add `outside_release_that_closes_a_modal_cannot_activate_the_exposed_cockpit`.
- [ ] Add `handled_text_stays_with_the_control_and_unhandled_keys_stop_at_a_strategic_modal`.
  At kernel level assert the propagation boundary; actual consumption is
  separately exercised through the Task 5B event router.
- [ ] Add `dismissible_menu_blocks_outside_press_until_explicit_close`.
- [ ] Add `an_unpainted_strategic_modal_is_retained_as_a_fail_closed_blocker`.
- [ ] Add `unpaintable_modeless_content_is_retained_without_implicit_close`.
- [ ] Add `explicitly_absent_modeless_content_closes_with_events`.
- [ ] Add `absent_modal_reports_blocker_until_explicit_close`.
- [ ] Add `suspended_content_is_retained_without_claiming_it_painted`.
- [ ] Add `reconcile_never_removes_windows_created_after_its_snapshot`.
- [ ] Add `contradictory_or_unknown_content_reports_are_diagnosed`.
- [ ] Add `scene_restriction_removes_inactive_modal_and_focus_from_route`.
- [ ] Add `active_membership_changes_command_eligibility_and_snapshot_together`.
- [ ] Add `unknown_active_member_rejects_scene_update_without_mutation`.

**Red/green command:**

```bash
cargo test -p rebellion-windowing --test routing
```

**Minimal implementation:**

- [ ] Copy routing-relevant records/order into snapshots; do not retain a mutable
  manager reference behind interior mutability.
- [ ] Determine modal boundary before pointer hit arbitration.
- [ ] Admit eligible descendants of the active modal while keeping unrelated
  transient-band entries blocked.
- [ ] Retain an outside-press dismissal route for Menu; consumption/close belongs
  to the application event boundary, never root click-through.
- [ ] Set explicit active membership before deriving routes and use the same
  membership for command validation; a snapshot-only restriction is insufficient.
- [ ] Scope reconciliation to snapshot members, validate reports, and return
  teardown/cancellation receipts for removals.
- [ ] Treat missing/invalid reports conservatively and report diagnostics.
- [ ] Keep paint receipts separate from `ContentStatus`; no `rendered_ids`
  subtraction is allowed to stand in for content lifecycle.
- [ ] Finish invariant validation with deliberate private-state corruption
  tests for each named invariant.

**Regression checks:** Full kernel suite plus the scoped mutation command in
the verification matrix. Mutate modal stop, duplicate-open, history bound,
ownership cascade, and snapshot retention conditions; inspect survivors.

**Atomic commit:** `feat(windowing): add immutable routing and reconciliation`

## Task 5A: Prove the Locked Egui Adapter Before Migration

**Purpose:** Resolve the load-bearing uncertainty about arbitrary stack order,
current-frame hit ownership, and built-in widget consumption.

**Create:** `crates/rebellion-render/src/windowing.rs` and
`crates/rebellion-render/src/windowing_input.rs`, initially with a bounded
adapter test harness and the chosen reusable implementation.
**Modify:** Render `Cargo.toml`, `lib.rs`; implementation evidence and the exact
adapter interfaces in this plan after the experiment selects them.
**Consumes:** Completed kernel snapshot/policy APIs and locked egui 0.31.1.
**Produces:** A proven paint/input mechanism and frozen bridge signatures.

**Acceptance:** The adapter realizes nontrivial permutations on the first and
subsequent frames, preserves widget state, and prevents blocked widgets from
mutating themselves. A screenshot of one initial order is insufficient.

- [ ] Add a bounded reproduction showing that raising every Area in desired
  order does not impose the desired permutation on the locked dependency.
- [ ] Add `windowing_adapter_reorders_existing_windows_on_the_first_requested_frame`.
- [ ] Add `windowing_adapter_keeps_widget_ids_and_text_state_across_raise`.
- [ ] Add `windowing_adapter_blocks_lower_text_edit_before_mutation`.
- [ ] Add `windowing_adapter_routes_overlap_to_exactly_one_button`.
- [ ] Add `windowing_adapter_preserves_scroll_clipping_and_root_hit_regions`.
- [ ] Add `windowing_adapter_cold_frame_matches_warm_frame_after_resize`.
- [ ] Exercise A/B/C order, C/A/B, B/C/A, new insertion, removal, and restore.
- [ ] Include one built-in TextEdit, one scroll area, one custom raw-input row,
  a cockpit button, and a blank strategic-map region.
- [ ] Run the red command against the rejected repeated-raise approach or the
  current feature-local approach and retain the failing observable result.

**Red/green command:**

```bash
cargo test -p rebellion-render windowing_adapter --features interface-test-fixtures
```

**Minimal implementation/prototype:**

- [ ] Prefer one managed host with stable per-window child UI IDs and snapshot
  paint order. Include base cockpit/rail UI in that host where required.
- [ ] Separate geometry preparation from input acceptance and painting so the
  final cursor position or previous-frame rectangles do not choose the route.
- [ ] Prove how built-in egui controls receive only eligible events; discarding
  a feature action after TextEdit already changed is a failure.
- [ ] Prove focused control → top-level → root disposition ordering.
- [ ] Verify click-to-front cannot let egui independently reorder the windows.
- [ ] Test transparent regions: empty host space must still permit map input
  when no modeless hit, application hold, capture, or modal blocks it.
- [ ] Test clipping/popup descendants; no nested-sublayer assumption is allowed.
- [ ] If a smaller supported API path passes all the same checks, prefer it
  over a broad renderer rewrite and record why it works.
- [ ] Reject per-frame area resets, rank-dependent widget IDs, dependency forks,
  and dependency upgrades as implicit fixes to the failed experiment.
- [ ] Freeze exact event collection, filtering, geometry, and host helper
  signatures in the plan/evidence before Task 5B consumes them.

**Additional objective check:** Retain first/warm-frame geometry, hit recipient,
focus, widget state, and screenshots from a small fixture run. A headless test
alone does not prove native/backend integration; Tasks 12–13 repeat the gate.

**Stop condition:** If neither candidate passes, keep the pure kernel work and
report the concrete failing experiment. Revise the adapter design before
migrating any feature; do not bury an unproven assumption in later tasks.

**Atomic commit:** `feat(render): prove deterministic strategic window adapter`

## Task 5B: Add the Coordinator and Event-Consumption Boundary

**Purpose:** Connect the proven adapter to the application without creating a
second production authority or leaving raw input paths outside the lease.

**Create:** `crates/rebellion-app/src/strategic_windowing.rs`.
**Modify:** App `Cargo.toml`, `main.rs`; render `windowing.rs`,
`windowing_input.rs`, `lib.rs`, and `cockpit.rs` where the chosen host needs it.
**Consumes:** Task 5A's frozen bridge; manager commands/events/snapshots.
**Produces:** `StrategicWindowing`, scene/hold handling, the exhaustive policy
table, explicit content receipts, and frame-local event admission.

**Acceptance:** One physical event produces at most one effect; root shortcuts
run only after actual child/top-level disposition; restrictive changes cannot
expose input during their triggering lease; both migrated and remaining legacy
surfaces respect an active common modal during intermediate commits.

- [ ] Add `strategic_windowing_pending_commands_do_not_mutate_current_snapshot`.
- [ ] Add `strategic_windowing_failed_open_preserves_previous_content`.
- [ ] Add `strategic_windowing_receipts_remove_only_matching_content_ids`.
- [ ] Add `strategic_windowing_reset_discards_pending_old_campaign_actions`.
- [ ] Add `windowing_input_handled_child_key_never_reaches_root`.
- [ ] Add `windowing_input_unhandled_modeless_key_can_reach_root_once`.
- [ ] Add `windowing_input_modal_unhandled_key_stops_at_window`.
- [ ] Add `windowing_input_macroquad_and_egui_copies_share_one_claim`.
- [ ] Add `windowing_input_event_coordinates_not_final_cursor_choose_target`.
- [ ] Add `windowing_input_cancelled_drag_quarantines_later_release`.
- [ ] Add `windowing_input_hold_release_does_not_unlock_existing_frame`.
- [ ] Add `windowing_policy_inventory_covers_all_nineteen_kinds`.
- [ ] Add `windowing_subject_fleet_detail_uses_system_dat_id`.
- [ ] Add `windowing_legacy_bridge_blocks_raw_paths_under_migrated_modal`.

**Red/green commands:**

```bash
cargo test -p rebellion-render windowing --features interface-test-fixtures
cargo test -p rebellion-app strategic_windowing --features interface-test-fixtures
```

**Minimal implementation:**

- [ ] Normalize the backend batch once and use frame/ordinal event tokens.
- [ ] Keep event-local coordinates and physical press ownership across frames.
- [ ] Integrate egui consumed-key/text state through the proven hook; do not
  declare all Function or Alt keys inherently unhandled by controls.
- [ ] Move global strategic shortcut execution after owner disposition.
- [ ] Apply pending commands at the explicit end-of-dispatch boundary and pair
  gameplay intents with successful, still-valid lifecycle/content commits.
- [ ] Deliver `CaptureCancelled` to feature drag state as well as the router.
- [ ] Keep root event-screen/briefing/fixture/scene holds separate from windows.
- [ ] Keep pause-alert and other rectangle-only base children spatially scoped.
- [ ] Add a narrow temporary bridge that combines legacy blockers with the new
  boundary; every unmigrated raw path must honor it before modal migrations.
- [ ] For each migrated family, disable its old cross-window authority in the
  same slice. Do not mirror booleans into auto-created manager windows.
- [ ] Log rejected commands and missing-modal diagnostics on state changes,
  not once per frame; no new permanent user-facing diagnostic panel is needed.
- [ ] Restrict public app access to queue/snapshot/scene methods; avoid a general
  `manager_mut` escape hatch in render callbacks.

**Regression checks:** Scoped app/render checks with and without fixtures,
existing Encyclopedia routing tests, and Task 5A adapter tests.

**Atomic commit:** `feat(app): centralize strategic window routing and commits`

## Task 6: Migrate Sector Identity and Replacement

**Purpose:** Preserve maximum-two/galaxy-half behavior while removing duplicate
raise and vector-position focus authority.

**Modify:** Render `sector_window.rs`, `windowing.rs`; app `main.rs`,
`strategic_windowing.rs`; relevant existing Sector tests and
`tools/interface-parity/sector-quadrants.mjs` only if observations require it.
**Consumes:** Atomic `Replace`, Sector subject mapping, Low-band policy.
**Produces:** Sector content records with manager IDs and shared route permits.

**Acceptance:** Duplicate Sector open preserves ID/order/focus; third Sector
atomically replaces the existing one in the matching half; failed preparation
or replacement leaves the old Sector visible and usable.

- [ ] Add `sector_window_duplicate_open_does_not_raise_or_reset_content`.
- [ ] Add `sector_window_third_open_replaces_matching_half_atomically`.
- [ ] Add `sector_window_failed_replacement_keeps_previous_sector`.
- [ ] Add `sector_window_duplicate_noop_differs_from_click_activation`.
- [ ] Add `sector_window_hit_blocks_only_covered_map_region_without_modal`.
- [ ] Add `sector_window_vector_order_does_not_choose_global_focus`.
- [ ] Add owner behavior from Task 0: under the proposed Root correction,
  `sector_window_close_preserves_root_owned_details`.

**Red/green command:**

```bash
cargo test -p rebellion-render sector_window --features interface-test-fixtures
```

**Minimal implementation:**

- [ ] Prepare the selected Sector content and resolve its DAT subject.
- [ ] Reuse existing identity for exact duplicate without moving the content
  vector, resetting selection, or opening a new context.
- [ ] Select a same-half replacement in feature code; submit `Replace` rather
  than trying to open a forbidden third Sector and then close the old one.
- [ ] Attach the successful new ID to prepared content and process teardown
  receipts for the replaced ID only.
- [ ] Paint by snapshot order and gate hits using event permits.
- [ ] Remove local `focus`/last-vector usage as a global authority while keeping
  content lookup helpers and geometry behavior.
- [ ] Ensure an actual pointer activation can raise/focus within its permitted
  band even though a duplicate opener remains a no-op.
- [ ] Keep unmigrated detail behavior behind the Task 5B bridge until Task 7A.

**Integration:** Build the fixture once and run
`node sector-quadrants.mjs --no-build` from `tools/interface-parity` on both
factions. Inspect overlap/map input rather than treating screenshots alone as
proof that lower controls did not act.

**Atomic commit:** `refactor(render): route Sector windows through shared windowing`

## Task 7A: Migrate the Four Modeless Detail Families

**Purpose:** Establish one cross-family stack while preserving each detail's
content and the resolved lifecycle owner.

**Modify:** Render `system_window.rs`, `fleet_window.rs`,
`defenses_window.rs`, `missions_window.rs`, `windowing.rs`; app `main.rs` and
`strategic_windowing.rs`. Update module tests next to their content adapters.
**Consumes:** System-subject mapping, `RaiseRestore`, lifecycle receipts.
**Produces:** ID-addressable detail content and a shared visible paint order.

**Acceptance:** System/Fleet/Defenses/Missions all key by system DAT ID plus
kind. Fleet selection is not window identity. Reopening an existing visible
detail raises it; reopening a minimized one restores the same ID and content.

- [ ] Add `detail_windows_use_system_subject_and_distinct_kind`.
- [ ] Add `fleet_window_switching_selected_fleet_preserves_window_identity`.
- [ ] Add `detail_windows_cross_family_activation_changes_one_stack`.
- [ ] Add `detail_windows_reopen_preserves_tabs_rows_scroll_and_selection`.
- [ ] Add `detail_windows_root_owner_survives_navigation_sector_close` only
  after Task 0 confirms the correction.
- [ ] Add `detail_windows_stale_system_subject_rejects_action_without_retargeting`.
- [ ] Add `detail_windows_background_open_preserves_active_modal_focus`.

**Red/green commands:**

```bash
cargo test -p rebellion-render system_window --features interface-test-fixtures
cargo test -p rebellion-render fleet_window --features interface-test-fixtures
cargo test -p rebellion-render defenses_window --features interface-test-fixtures
cargo test -p rebellion-render missions_window --features interface-test-fixtures
cargo test -p rebellion-app strategic_windowing --features interface-test-fixtures
```

Place cross-family orchestration assertions in the app coordinator tests when
constructing all content stores in render unit tests would duplicate app logic.
The command list deliberately uses one Cargo name filter per invocation.

**Minimal implementation:**

- [ ] Add `WindowId` to each open content record and keep lookup by semantic
  key/ID. Do not add a `slotmap` dependency solely to stringify keys.
- [ ] Retain content-store vectors where convenient but stop using their final
  element as cross-family focus or their iteration order as paint authority.
- [ ] Dispatch the four content renderers from the shared snapshot order.
- [ ] Preserve per-family bitmap geometry and all existing gameplay actions.
- [ ] Turn close/minimize/restore requests into pending window intents.
- [ ] Deliver close/eviction events to exactly the store containing that ID.
- [ ] Remove feature-local raises and stack mutations in the same migration.
- [ ] Keep the existing rail display connected through a temporary read adapter
  until Task 7C moves its history authority; do not maintain two live histories.

**Integration:** Run existing Fleet and Sector browser journeys and add a
bounded cross-family overlap case to the evolving windowing fixture. Click a
partially exposed Fleet/System/Defenses/Missions surface and assert both top
paint and the only recipient action.

**Atomic commit:** `refactor(render): unify modeless detail identity and order`

## Task 7B: Preserve Fleet Gestures and Focused Detail Controls

**Purpose:** Prove the shared route works for raw drag/drop and in-place text,
not merely title-bar activation and buttons.

**Modify:** `fleet_window.rs`, `defenses_window.rs`, `system_window.rs` where
Rename lives, `windowing_input.rs`, `windowing.rs`; app dispatch/receipt paths
and relevant Fleet browser assertions.
**Consumes:** Capture events, per-event permits, stable `ControlId` mapping.
**Produces:** Bounded drag ownership and child-control key disposition.

**Acceptance:** A drag owns its later release even outside the originating
window; cancellation clears feature payload; Rename is a child, never a new
manager record; Escape/Enter cannot both edit and execute a root command.

- [ ] Add `fleet_window_drag_release_outside_window_stays_with_capture_owner`.
- [ ] Add `fleet_window_modal_open_clears_drag_payload_and_capture`.
- [ ] Add `fleet_window_cancelled_drag_release_cannot_drop_or_click_underlay`.
- [ ] Add `fleet_window_invalid_drop_dispatches_no_gameplay_command`.
- [ ] Add `detail_rename_uses_child_focus_without_allocating_window`.
- [ ] Add `detail_rename_escape_cancels_edit_without_closing_detail`.
- [ ] Add `detail_rename_enter_commits_once_without_root_shortcut`.
- [ ] Add `detail_rename_missing_widget_restoration_falls_back_to_owner`.
- [ ] Add `detail_text_edit_blocked_by_modal_does_not_mutate_text`.

**Red/green commands:**

```bash
cargo test -p rebellion-render fleet_window --features interface-test-fixtures
cargo test -p rebellion-render detail_rename --features interface-test-fixtures
cargo test -p rebellion-render windowing_input --features interface-test-fixtures
```

**Minimal implementation:**

- [ ] Register child focus IDs in the owning detail's ID scope.
- [ ] Convert raw press/motion/release readers to the routed batch.
- [ ] Acquire capture only after an eligible press starts an actual gesture.
- [ ] Release capture on physical completion and cancel it on owner close,
  minimize, modal interruption, hold, scene transition, or application focus loss.
- [ ] Clear cached source/target payload before delivering cancellation onward.
- [ ] Keep rejected/cancelled release quarantined until button-up.
- [ ] Revalidate source fleet/system at commit so a moved/deleted object cannot
  be silently replaced by whatever now occupies a content index.
- [ ] Report actual child handling for text, Enter, Escape, arrows, Tab, and
  relevant chords; do not infer handling solely from keyboard class.

**Integration:** `node fleet-window.mjs --no-build` plus cross-frame interrupted
capture in the new contract journey. Preserve existing fleet loading/movement
semantics; this task changes ownership of input, not legal gameplay operations.

**Atomic commit:** `fix(render): preserve detail control and drag ownership`

## Task 7C: Move the Reference Rail to Shared History

**Purpose:** Make the twelve-slot rail a view of manager history, with labels
and images supplied by existing content stores.

**Modify:** `system_window.rs`, any existing cockpit rail call sites in
`cockpit.rs`/`main.rs`, `windowing.rs`, coordinator content teardown tests.
**Consumes:** `presentation_history`, `Minimize`, `Restore`, eviction events.
**Produces:** One history authority shared by all four detail families.

**Acceptance:** History excludes visible windows, preserves source-defined
oldest/newest ordering, caps at 12, and restores the same content ID. Eviction
removes content and actual owned transients without leaving focus/capture.

- [ ] Add `reference_rail_displays_manager_history_across_detail_families`.
- [ ] Add `reference_rail_restore_removes_entry_without_recreating_content`.
- [ ] Add `reference_rail_thirteenth_entry_tears_down_oldest_content`.
- [ ] Add `reference_rail_modal_blocks_restore_and_hover_actions`.
- [ ] Add `reference_rail_missing_art_does_not_delete_registered_detail`.
- [ ] Add `reference_rail_close_removes_only_matching_history_entry`.

**Red/green commands:**

```bash
cargo test -p rebellion-render reference_rail --features interface-test-fixtures
cargo test -p rebellion-app strategic_windowing --features interface-test-fixtures
cargo test -p rebellion-windowing --test lifecycle
```

**Minimal implementation:**

- [ ] Delete the old mutable history list as soon as the manager list is used.
- [ ] Resolve each ID into the appropriate feature's label/icon/content.
- [ ] Route rail pointer input as base strategic UI through the same modal and
  application-hold boundary; absence of an egui modal hit is irrelevant.
- [ ] Queue restore/close actions and consume their physical trigger once.
- [ ] Keep minimized content `Ready` or `Suspended` as appropriate, never
  `Absent` solely because it does not paint in the main content area.
- [ ] Route eviction receipts through the same teardown dispatcher as close.
- [ ] Remove compatibility rail lookup code left by Task 7A.

**Integration:** Minimize interleaved System/Fleet/Defenses/Missions details,
restore a middle rail entry, then overflow to 13. Retain before/after order and
content IDs, not just a screenshot showing twelve icons.

**Atomic commit:** `refactor(render): derive reference rail from shared history`

## Task 8: Migrate Finders and Message Index

**Purpose:** Establish the shared singleton strategic-modal pattern and prove
focused child handling before migrating the Encyclopedia special case.

**Modify:** `fleet_finder.rs`, `troop_finder.rs`, `personnel_finder.rs`,
`message_index.rs`, `windowing.rs`, `windowing_input.rs`; app `main.rs`,
`strategic_windowing.rs`, and fixture observations.
**Consumes:** Singleton `NoOp`, strategic-modal policy, child focus routing.
**Produces:** Three available Finder surfaces and Message on the common route.

**Acceptance:** Duplicate open does not reset selection/search/focus/order;
outside press, wheel, rail, and cockpit shortcuts remain blocked; Escape closes
only the current permitted surface. System Finder gets no fabricated content.

- [ ] Add `finder_duplicate_open_preserves_id_selection_and_focus`.
- [ ] Add `message_index_duplicate_open_preserves_rows_and_focus`.
- [ ] Add `finder_focused_control_consumes_text_arrows_enter_and_escape`.
- [ ] Add `finder_unhandled_function_key_stops_at_modal_boundary`.
- [ ] Add `message_index_outside_press_does_not_select_map_system`.
- [ ] Add `finder_close_key_cannot_open_another_surface_in_same_frame`.
- [ ] Add `finder_modal_blocks_unmigrated_legacy_input_paths`.
- [ ] Add `system_finder_policy_does_not_claim_missing_port_content`.

**Red/green commands:**

```bash
cargo test -p rebellion-render finder --features interface-test-fixtures
cargo test -p rebellion-render message_index --features interface-test-fixtures
cargo test -p rebellion-app strategic_windowing --features interface-test-fixtures
```

**Minimal implementation:**

- [ ] Register each available singleton through prepared-content/open receipts.
- [ ] Make no-op detection happen before resetting feature data.
- [ ] Map search/list child controls into the owning window's focus scope.
- [ ] Apply permits to built-in controls and raw row activation equally.
- [ ] Deliver explicit close receipts to the app-owned Message state as well
  as render-side state; no bool-based auto-registration is permitted.
- [ ] Keep System Finder's F2 behavior honestly absent while its future policy
  participates in exhaustiveness tests.
- [ ] Verify modal descendants can act while all unrelated windows are blocked.

**Integration:** Existing `fleet-finder.mjs --no-build` and
`message-index-shell.mjs --no-build`, then the new shared journey for the three
available Finder variants and Message on Alliance/Empire.

**Atomic commit:** `refactor(render): share Finder and Message modal routing`

## Task 9: Migrate Encyclopedia and Status Without Losing Controller Semantics

**Purpose:** Replace the narrow Encyclopedia keyboard exception with the shared
route while preserving session validation, entry intent, and return behavior.

**Modify:** `crates/rebellion-app/src/encyclopedia_surface.rs`, app `main.rs`,
`strategic_windowing.rs`; render `encyclopedia.rs`, `encyclopedia_surface.rs`,
`status_window.rs`, `windowing.rs`; their existing tests and browser observers.
**Consumes:** Singleton/contextual policies, child focus, explicit content errors.
**Produces:** Shared modal IDs and removal of the Encyclopedia-specific owner.

**Acceptance:** Encyclopedia retains its controller's origin/current/
session-generation contract. Status uses a fresh context subject where needed.
Both block outside input on both factions and retain an explicit unavailable
modal rather than exposing the cockpit if content painting fails.

The inspected controller lives in **rebellion-app**. Its current public open
accepts `&EncyclopediaSession` and `EncyclopediaEntryIntent` and returns
`Result<(), EncyclopediaPresenterError>`. The migration must preserve those
inputs. Add window coordination at the app commit boundary and use a combined
typed error if lifecycle commit can fail; do not replace the API with an
invented availability-only renderer method.

The proposed commit-boundary method is explicit:

```rust
pub enum EncyclopediaWindowOpenError {
    Presenter(EncyclopediaPresenterError),
    Window(WindowError),
}
impl EncyclopediaSurfaceController {
    pub fn open(
        &mut self,
        session: &EncyclopediaSession,
        intent: EncyclopediaEntryIntent,
        windows: &mut WindowManager,
    ) -> Result<WindowId, EncyclopediaWindowOpenError>;
}
```

This is called by the coordinator after routing, never by a paint callback.
Prepare validated presenter state without mutating the live controller; commit
Open; publish prepared state only for a new open. A duplicate returns its ID
without replacing the stored return route. A manager error preserves the old
controller; the existing presenter error remains available to callers through
the combined error. Do not add global mutable manager access just to call it.

- [ ] Add `encyclopedia_open_preserves_session_and_entry_intent_validation`.
- [ ] Add `encyclopedia_duplicate_open_preserves_immutable_return_route`.
- [ ] Add `encyclopedia_search_handling_precedes_cockpit_shortcuts`.
- [ ] Add `encyclopedia_escape_close_keeps_frame_keyboard_lease`.
- [ ] Add `encyclopedia_unpaintable_content_retains_modal_blocker`.
- [ ] Add `status_window_context_replacement_is_atomic`.
- [ ] Add `status_window_outside_press_and_f_keys_do_not_reach_cockpit`.
- [ ] Add `status_window_stale_subject_rejects_confirm_without_retargeting`.

**Red/green commands:**

```bash
cargo test -p rebellion-app encyclopedia_surface --features interface-test-fixtures
cargo test -p rebellion-render encyclopedia --features interface-test-fixtures
cargo test -p rebellion-render status_window --features interface-test-fixtures
cargo test -p rebellion-app strategic_windowing --features interface-test-fixtures
```

**Minimal implementation:**

- [ ] Keep Encyclopedia session/presenter preparation ahead of new open commit.
- [ ] For an existing no-op singleton, preserve original entry/return route and
  current content; do not treat repeated F7 as a fresh entry intent.
- [ ] Preserve controller teardown on session invalidation and route it through
  explicit close/reset receipts.
- [ ] Use the common event router for search text, list navigation, Escape,
  Enter, and shortcut bubbling.
- [ ] Remove `FrameKeyboardOwner` once its last production user migrates; first
  port its same-frame regression tests to the generic route.
- [ ] Use `Replace` for a one-at-a-time Status context change after validating
  the new subject; keep prior content intact on failure.
- [ ] Add an unavailable/error presentation with an explicit close route for
  committed modal content that cannot render.

**Integration:** Run the existing Encyclopedia surface/canonical-surface and
Status scripts, plus actual F7 → type → Escape → next-frame root interaction
in the contract journey. F2 is System Finder, not Encyclopedia.

**Atomic commit:** `refactor(app): unify Encyclopedia and Status window routing`

## Task 10: Migrate Command Dialogs, Menus, and GID Transients

**Purpose:** Bring the remaining modal/temporary surfaces under the same event
and lifecycle contract while preserving existing gameplay commands.

**Modify:** `build_selection.rs`, `mission_dialog.rs`, `move_confirmation.rs`,
`agent_menu.rs`, `game_menu.rs`, `game_speed.rs`, `object_menu.rs`, `cockpit.rs`,
`windowing.rs`, `windowing_input.rs`; app `main.rs`/coordinator and their tests.
Use actual GID root/submenu call sites in `cockpit.rs`; there is no new
standalone GID-window family to invent.
**Consumes:** Context subjects, `Replace`, Menu saved focus, cancellation receipts.
**Produces:** Common command-modal routes and one active Menu presentation.

**Acceptance:** Build Selection, Mission creation, and Move confirmation obey
the recovered strategic-modal rule. Scrap is explicitly a port policy. Menu
outside press consumes/dismisses; owned submenus remain eligible; Rename stays
a focused control implemented in Task 7B.

- [ ] Add `build_selection_duplicate_open_preserves_manufacturing_context`.
- [ ] Add `mission_dialog_confirm_consumes_key_and_dispatches_once`.
- [ ] Add `move_confirmation_revalidates_context_before_gameplay_effect`.
- [ ] Add `scrap_confirmation_uses_explicit_port_modal_policy`.
- [ ] Add `command_modal_open_cancels_fleet_capture_and_drag_payload`.
- [ ] Add `menu_replacement_is_atomic_and_saves_eligible_invoker_focus`.
- [ ] Add `menu_outside_press_dismisses_without_clicking_underlay`.
- [ ] Add `menu_owner_close_removes_menu_before_restoring_focus`.
- [ ] Add `menu_inside_active_modal_remains_eligible`.
- [ ] Add `gid_submenu_multiple_rects_route_to_one_menu_identity`.
- [ ] Add `gid_menu_root_actions_remain_blocked_by_unrelated_modal`.

**Red/green commands:**

```bash
cargo test -p rebellion-render build_selection --features interface-test-fixtures
cargo test -p rebellion-render mission_dialog --features interface-test-fixtures
cargo test -p rebellion-render move_confirmation --features interface-test-fixtures
cargo test -p rebellion-render menu --features interface-test-fixtures
cargo test -p rebellion-app strategic_windowing --features interface-test-fixtures
```

**Minimal implementation:**

- [ ] Prepare command content and allocate context only for a real new context;
  a duplicate same-context request is not an excuse to rebuild content.
- [ ] Atomically replace a prior one-at-a-time contextual surface if existing
  feature behavior calls for it; do not globally invent replacement semantics.
- [ ] Keep Build Selection a singleton, with manufacturing subject in content.
- [ ] Require a successful, revalidated confirm transaction before dispatching
  the existing game command; a rejected lifecycle intent cannot perform it.
- [ ] Register one active Menu with its actual invoker as owner and save only
  that menu's eligible focus return target.
- [ ] Group disjoint GID submenu rectangles under the same menu ID and content
  variant; keyboard/mouse disposition still has one event owner.
- [ ] On outside press queue close and consume that press; suppress orphan
  release so the newly exposed cockpit cannot receive the rest of the gesture.
- [ ] Do not implement speculative owner-subtree-only modality or public
  `OwnerModal` semantics. Keep the narrow dismissal policy labeled `port:`.
- [ ] Keep targeting state machines outside the kernel; their root input must
  still pass the shared route and existing application conditions.

**Integration:** Existing Build Selection, Mission, Fleet, and GID journeys,
plus outside-dismissal, modal-owned menu, menu replacement, and interrupted
Fleet drag in both-faction contract runs.

**Atomic commit(s):** `refactor(render): centralize command dialog modality`, then
`refactor(render): centralize menu lifecycle and GID input` if review size merits
two commits within this dependency gate.

## Task 11: Integrate Tactical Modals and Scene Transitions

**Purpose:** Apply strategic modal semantics to Battle Alert/Results without
forcing macroquad battle rendering into egui or leaking dormant Galaxy modals.

**Modify:** `crates/rebellion-render/src/tactical_view.rs`, app `main.rs`,
`strategic_windowing.rs`, fixture state/observations and tactical tests.
**Consumes:** `WindowScene`, scene-restricted snapshots, battle subject IDs.
**Produces:** Stable Alert/Results IDs for a battle session and common input gates.

**Acceptance:** Tactical normal controls remain tactical; Alert/Results block
their command-center underlay; Galaxy windows retained off-scene neither block
nor consume tactical input. Dismissal/return events never leak across scenes.

- [ ] Add `tactical_windowing_dormant_galaxy_modal_does_not_block_battle_input`.
- [ ] Add `tactical_windowing_alert_blocks_command_center_outside_panel`.
- [ ] Add `tactical_windowing_results_blocks_underlay_until_next_snapshot`.
- [ ] Add `tactical_windowing_phase_change_uses_same_battle_subject_distinct_kind`.
- [ ] Add `tactical_windowing_scene_switch_cancels_capture_and_orphan_release`.
- [ ] Add `tactical_windowing_return_restores_only_valid_galaxy_content`.
- [ ] Add `tactical_windowing_new_campaign_discards_old_queued_confirm`.
- [ ] Add `tactical_windowing_unpaintable_results_remains_fail_closed`.

**Red/green commands:**

```bash
cargo test -p rebellion-render tactical --features interface-test-fixtures
cargo test -p rebellion-app strategic_windowing --features interface-test-fixtures
```

**Minimal implementation:**

- [ ] Use the actual `TacticalViewState` type and its existing phase controller.
- [ ] Assign one opaque battle context subject per battle session; use distinct
  `BattleAlert`/`BattleResults` keys with that subject.
- [ ] Select active scene before deriving modal/focus/pointer routes.
- [ ] Translate macroquad hit regions into the same route contract; rendering
  stays in the existing tactical pipeline.
- [ ] Commit phase lifecycle changes at one boundary and retain the old event
  lease through it. A close/open pair must not expose an intermediate snapshot.
- [ ] Cancel gestures explicitly on scene transitions and application focus loss.
- [ ] Do not call `FocusRoot` to bypass a legitimate active-modal rejection.
- [ ] Revalidate dormant Galaxy content on return; suspend temporary Game Options
  rather than marking every strategic window absent.
- [ ] Clear all content, queues, gestures, focus mappings, and contexts on campaign
  reset, while preserving monotonically non-reused manager IDs.

**Integration:** Exercise Alert and Results through deterministic port fixtures,
including return to a Galaxy that had a modal open. Retain the evidence gap:
fixture Results coverage is not an original-game Results observation.

**Atomic commit:** `refactor(app): route battle modals through scene-aware windowing`

## Task 12: Remove Legacy Authority and Prove Browser/Package Behavior

**Purpose:** End the migration bridge and establish cross-family behavior in
both fixture and production packages.

**Create:** `tools/interface-parity/windowing-contract.mjs` and a focused helper
unit test file if new harness logic needs independently testable behavior.
**Modify:** Fixture `interface_test_fixture.rs`; tools `package.json`, `README.md`,
`verify-production-exclusion.mjs` if needed to check a supplied artifact;
app/render residual routing sites; implementation evidence.
**Consumes:** All migrated families and the complete acceptance matrix below.
**Produces:** One production authority, deterministic observations, reusable
both-faction journey, and retained packaged-production evidence.

**Acceptance:** No legacy family exception is needed for route correctness;
fixture observations report behavior without granting new production powers;
packaged production passes a real interactive journey without fixture bridges.

- [ ] Add fixture-code round-trip tests for the new scenario and existing codes.
- [ ] Add harness unit tests for any new observation validation/encoding helper.
- [ ] Add browser assertions that fail on the pre-fix behavior for overlap,
  duplicate open, modal outside input, child consumption, and close fallthrough.
- [ ] Run the new journey red against the relevant old behavior or an explicit
  narrowly controlled local regression; do not claim a historical run occurred.
- [ ] Add `WindowingContract` after the currently last enum value 64.
- [ ] At the inspected revision use enum value 65 and `SCENARIO_COUNT = 66`;
  external low byte is 66 because decoding subtracts one. Recheck before editing.
- [ ] Preserve faction/viewport/high-bit encoding using existing helpers rather
  than hard-coding complete request code 65 or 66 for every case.
- [ ] Record stable semantic keys, order, focus/control, history, capture,
  active modal, route/effect counters, and relevant content selection.
- [ ] Keep all observation/setup bridges behind `interface-test-fixtures`.
- [ ] Avoid exposing mutating general window-manager commands to production JS.

**Red/green commands:**

```bash
cargo test -p rebellion-app interface_test_fixture --features interface-test-fixtures
(cd tools/interface-parity && npm run test:unit)
(cd tools/interface-parity && npm run test:windowing)
```

`test:windowing` is introduced by this task. Specify whether it builds; add a
`:no-build` companion or pass the script's documented `--no-build` argument
for subsequent journeys against the exact same fixture artifact.

**Cleanup:**

- [ ] Remove `FrameKeyboardOwner`, family-by-family modal boolean lists,
  duplicate rectangle routing, and local cross-window raise calls.
- [ ] Derive map/root eligibility from shared routes plus named non-window locks.
- [ ] Delete the Task 5B legacy bridge after the final family is migrated.
- [ ] Search raw key/mouse/touch/wheel readers and classify each remaining call
  as backend collection, admitted content input, or out-of-scope scene input.
- [ ] Reject any strategic raw input reader that bypasses event admission.
- [ ] Keep content vectors/options, but remove their use as cross-window order,
  focus, existence policy, or implicit reconciliation commands.
- [ ] Run the complete automated verification matrix once after cleanup.

**Browser/package validation:**

- [ ] Build fixture WASM once; run both-faction contract and changed-family suites.
- [ ] Build a uniquely named production package after fixture work.
- [ ] Verify production exclusion on the actual final served artifact.
- [ ] Run production interaction without fixture setup/inspection bridges.
- [ ] Retain package/WASM hashes, logs, screenshots, viewport, faction, seed,
  browser version, revision, and dirty-diff identity for each run.
- [ ] Inspect pixels and observed actions; a successful process exit alone does
  not establish visible order or no-underlay effects.

**Atomic commits:** `test(windowing): add cross-family browser contract`, then
`refactor(app): remove legacy strategic input authority` when separation keeps
both commits passing and reviewable. Do not pre-accept audit cells here.

## Task 13: Complete Native Port Acceptance

**Purpose:** Prove the port's native backend preserves the same event, focus,
geometry, and capture contract observed in browser runs.

**Modify:** The implementation evidence Markdown only, plus test code if a
real native failure requires a separately scoped fix. Proprietary screenshots,
runtime logs, saves, and packs remain in the local evidence workspace.
**Consumes:** The verified Task 12 revision and native command contracts.
**Produces:** Inspected native evidence for Alliance/Empire and explicit limits.

**Acceptance:** Both factions run the native journey with retained binary/hash,
input method, viewport, actions, expected/actual observations, and logs. Browser
success does not substitute for this gate, and Linux/macOS port success does
not claim original native-Windows parity.

- [ ] Choose an isolated owned display/application instance; inspect any active
  capture reservations before using a shared Wine prefix/display.
- [ ] Use a fresh evidence directory for each faction/backend run.
- [ ] Build/launch with the repository wrapper and sound muted by its default.
- [ ] Confirm the dev-command directory belongs to this running instance.
- [ ] Use dev commands for supported deterministic setup and captures.
- [ ] On macOS use documented injected Press/Click/Drag only after confirming
  that the current wrapper supports the command form.
- [ ] On Linux do not invoke those commands expecting NSEvent behavior: the
  current implementation explicitly refuses that injection path.
- [ ] Use actual input on the isolated native display, manually or through an
  already-supported platform driver; record which method generated it.
- [ ] Do not create a cross-platform input injection framework for this plan.
- [ ] Execute the native journey specified in the acceptance matrix.
- [ ] Inspect close-frame/next-frame, resize, keyboard text, and capture release
  behavior rather than accepting a static image of an open modal.
- [ ] Repeat the same contract for the second faction.
- [ ] Retain missing cases as outstanding; do not write a passing placeholder
  because the available native environment could not perform them.

**Verified wrapper shape:**

```bash
./scripts/launch-native.sh --build --evidence "$windowing_native_evidence" \
  --seed 1337 --live
./scripts/dev-send.sh "$windowing_native_evidence" "Capture"
```

Before running, set `windowing_native_evidence` to a new absolute directory in
local evidence storage. Read `agent_docs/dev-commands.md` for supported setup
commands and exact arguments. `--live` requires `--evidence`; the dev wrapper's
first argument is the evidence directory, not the command string.

**Objective checks:** Evidence contains command transcripts, application/binary
identity, viewport/faction, screenshots or recording, explicit expected/actual
results, console/panic output, and artifact hashes. No fake red/green cycle is
required for this runtime-evidence slice.

**Atomic commit:** `docs(windowing): record native port acceptance evidence`
with repository-safe summaries/links only. A discovered bug returns to the
relevant implementation slice, then reruns affected browser and native gates.

## Task 14: Reconcile Evidence, Architecture, and Audit Records

**Purpose:** Publish exactly the completion state proved by Tasks 0–13 without
conflating toolkit implementation, port behavior, and original-game parity.

**Modify:**

- `agent_docs/architecture.md` and `agent_docs/roadmap.md`.
- This plan and `docs/plans/INDEX.md`.
- Toolkit evidence Markdown.
- Audit `README.md`, `2026-10-06-manual-window-checklists.md`,
  `2026-10-06-surface-ledger.json`, `2026-10-06-audit-report.md`,
  `audit-report.json`, `screenshot-ledger.md`, and `known-deviations.md` only
  where actual evidence changes their state.
- Ignored local `progress.json` only if it exists and its established schema
  requires a checkpoint. Keep it uncommitted.

**Consumes:** Exact command/evidence records and unresolved-gap inventory.
**Produces:** Consistent architecture, roadmap, audit, and completion claims.

**Acceptance:** Every changed status links to sufficient inspected evidence;
no unknown/native-only requirement is accepted from kernel or fixture tests;
no occupied deviation identifier is reused for a new finding.

- [ ] Explain crate dependency direction, sole authority, frame lease, content
  reporting, adapter choice, and scene/gesture behavior in architecture docs.
- [ ] Record the implementation checkpoint and remaining System Finder,
  original-runtime, and Windows-specific acceptance work in the roadmap.
- [ ] Map each checklist cell individually to its required evidence type.
- [ ] Retire only routing/order deviations actually removed; unrelated bitmap,
  layout, text, missing content, and interaction gaps stay open.
- [ ] Preserve `fact:`, `runtime:`, `port:`, and `unknown:` distinctions.
- [ ] Record actual native platform; never relabel Wine or browser artifacts as
  native-Windows evidence.
- [ ] Record baseline failures separately from new failures and not-run gates.
- [ ] Update all linked ledger/report summaries together where status changes.
- [ ] Validate the ledgers and inspect the resulting diff for accidental status
  promotion, obsolete paths, or proprietary/session content.
- [ ] Mark this plan complete only when its delivery gates pass; otherwise
  record a bounded implementation checkpoint with the exact remaining gates.

**Objective validation:**

```bash
node scripts/validate-interface-parity-ledgers.mjs
git diff --check
git status --short
```

Also inspect the evidence artifact hashes and every changed audit-cell link.
Documentation needs objective consistency checks, not artificial unit tests.

**Atomic commit:** `docs(windowing): reconcile architecture and acceptance records`

Future contribution publication follows workspace policy: explicit-path staging,
verified topic-branch commits, push to `will-forster`, upstream PR against
`tdimino/open-rebellion:main`, no direct main push and no self-merge. That future
workflow does not authorize a commit, push, or PR during this planning task.

## Detailed Execution Contracts

These contracts are acceptance requirements for the preceding tasks, not a
second API or an override to their interfaces. A task is incomplete if its
implementation satisfies its happy path but violates one of these shared cases.

### Event Admission Matrix

| Event / state | First eligible recipient | Allowed propagation | Required observation |
|---|---|---|---|
| Modeless overlap press | Highest eligible hit window | Its child/top-level only | One activation and one action at most |
| Blank map press, no blocker | Root/map | Existing root dispatch | No invisible full-screen host interception |
| Press outside strategic modal | Modal boundary | None to underlay | Map/cockpit selection unchanged |
| Press inside modal child control | That child | Owning modal if unhandled | Child mutates once, root unchanged |
| Wheel over overlapping lists | Highest eligible hit list/window | No unrelated list | Only one scroll offset changes |
| Captured drag motion outside bounds | Capture owner | Existing drag state machine | Geometry cannot transfer ownership |
| Captured release | Press/capture owner | No newly exposed control | One drop or cancellation |
| Release after modal cancelled drag | Quarantine | None | No drop, click, or map command |
| Release without admitted press | No click recipient | Backend hover may update | Newly exposed button does not fire |
| Text into focused modeless edit | Focused child | Owner only if truly unhandled | Text survives raise without duplicate insertion |
| Text into blocked lower edit | None | None | String does not change internally |
| Enter handled by Rename | Rename child | Stop | Rename commits once; window remains |
| Escape handled by Rename | Rename child | Stop | Edit cancels; detail remains |
| Escape unhandled by active modal child | Owning modal close handler | Stop | Modal closes after lease, no root action |
| F-key handled by child | Focused child | Stop | No cockpit action |
| F-key unhandled under modal | Owning modal/boundary | Stop at modal | No cockpit action even outside modal rect |
| F-key unhandled in modeless window | Focused top-level then root | Once if allowed | Correct cockpit command, no duplicate |
| Alt chord | Actual child/top-level chain | According to handling/boundary | No class-based unconditional root bypass |
| Menu outside press | Menu dismissal boundary | Stop | Menu closes, underlay unchanged |
| Menu item press | Menu child/item | Stop if action fires | One action plus explicit close |
| Press during event-screen hold | Application hold | None to strategic UI | No strategic state mutation |
| Press on pause-alert rectangle | Existing base child | Its scoped handler | Uncovered map remains independently eligible |
| Scene-transition trigger | Old scene recipient | Stop | No action in newly exposed scene |
| App loses focus during drag | Cancellation path | None until fresh press | Drag payload/capture clear |
| Two events in one frame | Independently identified events | Each frozen route/claim | No conflation by final cursor/key state |

For a key class not represented above, follow actual control disposition and
the same propagation boundary. Do not add a permissive default branch that
sends unknown keys directly to root before the focused control can process them.

### Keyboard Dispatch and Egui Focus

- Semantic focus belongs to the manager; egui widget focus implements that
  choice for controls, but does not independently select a global top-level.
- A control request reports both its window and stable control ID.
- Equal control IDs in different windows do not refer to the same egui widget.
- A control receives text/IME events only when its owner has the admitted route.
- Backend text events and key events may be related but are not interchangeable;
  do not synthesize duplicate characters from key presses.
- Press/repeat/release handling preserves existing control semantics and cannot
  cause repeated confirm commands from one admitted physical trigger.
- A root shortcut uses only an explicitly unhandled propagation result.
- An egui `wants_keyboard_input` boolean alone is not the whole contract: it
  cannot explain per-event ownership or a modal stop for unhandled F-keys.
- Requesting focus after paint cannot retroactively change who owned the old
  event; apply it through the next-frame semantic focus boundary.
- Removing a focused child submits `WindowCommand::InvalidateControl`; the
  manager downgrades current and saved references to the eligible owner before
  later restoration. It never focuses a stale widget in another window.
- A menu restores a saved child only if its owner/control still exists and is
  eligible. Otherwise use the normal eligible fallback.
- No generic hidden per-window focus stack is introduced for ordinary details.
- Root text and keyboard behavior outside the strategic scene remains governed
  by the existing mode implementation.

### Pointer Geometry, Paint, and First-Frame Behavior

Geometry preparation must answer the current event's hit question before
mutable feature input runs. This requirement is particularly important for
restored windows, fresh modal opens, viewport resize, and moved windows.

1. Resolve the active scene and logical-to-screen transform.
2. Prepare rectangles from current content/layout, including disjoint submenu
   regions and any existing strictly bounded bitmap hit areas.
3. Apply snapshot visibility and modal eligibility.
4. Select the topmost eligible hit using snapshot order, not registration order.
5. Assign the event permit or capture owner.
6. Run admitted content input and paint through the proven adapter.
7. Compare paint receipts to expected paint for diagnostics only.

The adapter must retain existing geometry conventions. It must not round all
coordinates to a new grid, widen strict native hit intervals, or alter clipping
simply to make a convenient test pass. Test logical 640×480 and the existing
larger viewport variants, including first frame after switching viewport.

A window with multiple hit rectangles is a union, not the last rectangle
registered. Overlapping rectangles within the same window still give one
recipient. Multiple overlapping windows select the top eligible record.

Transparent modal barriers block input rather than changing original artwork.
Verify that a barrier cannot obscure modal controls, appear above a modal-owned
menu, or intercept a normal modeless map click outside all window bounds.

A restored window appears with the same widget IDs and content state. A raise
changes paint/hit priority without erasing text, scroll offsets, selection, or
in-progress valid child interaction. A blocked lower widget must not consume
input even if its retained egui state still says it had focus previously.

### Lifecycle Transaction Matrix

| Request | Preflight | Commit | Content consequence |
|---|---|---|---|
| New open | Key, policy, owner, subject, capacity, content preparation | Allocate ID and insert atomically | Attach ID to prepared content |
| Duplicate NoOp | Same key/owner/policy | Return existing identity, no state mutation | Preserve existing content and entry context |
| Duplicate RaiseRestore | Existing compatible key | Restore if needed, raise/focus if eligible | Preserve content and widget IDs |
| Invalid open | Any preflight fails | Typed error, allocator unchanged | Retain existing surface |
| Replace | Validate full post-removal state and new owner | Child-first old teardown plus new ID | Swap prepared content after success |
| Activate | Visible, scene-active, input eligible | Raise/focus within policy | No content reset |
| Focus child | Owner visible/eligible, child mapping valid | Update focus only | Request matching widget at boundary |
| Minimize | History-enabled, no blocking modal child | Remove visible order, append history | Retain detail content |
| History overflow | Oldest minimized victim identified | Close victim through teardown path | Remove exact victim content and transients |
| Restore | Known minimized record | Remove history, restore/raise same ID | Preserve content |
| Close | Known ID and actual descendants | Cancel capture, child-first close, restore focus | Drop exact closed IDs |
| Modal open during drag | Valid open + incompatible capture | Cancel old capture before modal focus | Clear feature payload and quarantine release |
| Explicit modeless absence | Valid report for snapshot member | Close with ContentRemoved | Process ordinary teardown receipts |
| Unpaintable modal | Valid report | Keep blocker, diagnose | Paint unavailable state/explicit close if possible |
| Temporary scene exit | Scene selection and gesture cancellation | Restrict active view | Retain dormant content |
| Campaign reset | Discard pending old work | Clear records, preserve allocator progression | Clear every content/gesture/focus cache |

Do not stage gameplay changes before a transaction that can reject the source
subject or context. If a command's content preparation itself fails, its old
surface remains unchanged. If content becomes unavailable after a successful
modal open, maintain the fail-closed boundary and an explicit recovery close;
do not silently revert modality by dropping the record.

The application's queue is a sequence of individually reported commands, not
a promise that an arbitrary batch is one transaction. Use `Replace` where
atomic replacement is needed. For a phase transition comprising close/open,
no routing snapshot or input dispatch may occur between those commits.

### Content Reporting and Error Recovery

| Report | Meaning | Manager treatment | Application obligation |
|---|---|---|---|
| Ready | Feature state exists and is usable for its current lifecycle | Retain | Paint if visible/active; retain if minimized |
| Unpaintable | State exists but render preparation/assets failed | Retain, diagnose modal blocker | Preserve explicit recovery/close route |
| Absent | Feature confirms state was removed | Close modeless snapshot member; retain modal until explicit close | Supply reason and process receipts |
| Suspended | State intentionally retained off-scene/temporarily inactive | Retain | Exclude through scene membership, not paint guess |
| No report | Incomplete application accounting | Conservative diagnostic | Fix missing mapping; never infer a safe modal dismissal |
| Conflicting reports | Bug in producer accounting | Reject/diagnose contradiction | Fix duplicate ownership of reports |
| Unknown or post-snapshot ID | Stale/late accounting | Diagnose without tearing down new record | Correct frame association |

Do not derive `Absent` by checking `rendered_ids`. A minimized window, off-scene
window, occluded content path, missing texture, and deleted game subject are
five different circumstances. The report producer must know which occurred.

Diagnostics should identify semantic kind/key, runtime ID, active scene,
report category, last accepted command, and affected focus/capture where useful.
Keep them bounded and emitted on state change. Repeated missing paint must not
flood logs or be “recovered” by opening another copy of the same singleton.

For development builds, assert manager/content agreement at the frame boundary.
In production, reject invalid actions and retain a safe route; an invariant
failure should not execute a guessed game command against a fallback index.
The kernel returns typed errors and does not print or acquire platform services.

### Mixed-Migration Safety Contract

Intermediate commits must remain usable. A shared kernel that controls only
one modal is unsafe if an old raw input path can still act behind it.

| Migration stage | New authority | Temporary compatibility | Removal gate |
|---|---|---|---|
| T5A | Isolated adapter proof | Existing production routing unchanged | T5B integrates proven bridge |
| T5B | Shared event boundary and explicit holds | Named legacy blockers participate in boundary | All families migrated |
| T6–T7 | Migrated Sector/details/history | Remaining family content paths require common permits | Their migration tasks |
| T8–T9 | Available singleton/context modals | Legacy command dialogs still contribute blockers | T10 |
| T10 | Command dialogs/menus | Tactical bridge only | T11 |
| T11 | All included families | Cleanup-only obsolete code | T12 authority scan |
| T12 | Sole production manager/router | None | Browser/native gates |

The bridge is explicit and temporary. It must not watch a feature bool and
open/close a shadow manager record automatically. An old blocker can restrict
routing; it cannot create a competing ordered registry.

Each family migration removes that family's old raise/focus/open-policy writes
in the same slice. Content vectors and options may remain for storage. Their
position/presence is no longer a cross-family routing decision.

The end-of-task evidence records which legacy consumers remain. The final scan
must account for each one by deletion, migration, or an explicit out-of-scope
mode boundary. “No matches for one symbol” is not sufficient if equivalent
logic remains under a different name.

## Browser and Native Acceptance Journeys

### Deterministic Fixture Setup

The contract fixture prepares content, not outcomes. It may choose systems,
fleets, rows, and faction deterministically. It must not directly set the final
stack/focus/history state that the journey is meant to exercise through input.

Required initial conditions:

- A known faction and viewport using existing fixture encoding.
- At least two sectors in different halves and a third in a chosen half.
- Systems that can open each currently implemented detail family.
- A Fleet detail with a valid selectable row and drag source.
- Enough legitimate detail subjects for history-overflow testing.
- Available Finder, Message, Encyclopedia, Status, and command-dialog content.
- A battle fixture that can exercise Alert and Results separately from original
  save reachability.
- Muted audio and deterministic seed/world setup.

Observation payloads expose semantic outcomes and effect counters, not a
publicly callable arbitrary manager. Each assertion records before and after
selection/command counts so “nothing underneath happened” is measurable.

Use browser real pointer/keyboard actions for contract behaviors after setup.
Avoid evaluating JS that directly queues the expected window command for the
same test that is meant to prove pointer/keyboard routing.

### Journey A: Identity, Sector Policy, and Cross-Family Order

1. Open Sector A, then B; record IDs and bottom-to-top order.
2. Duplicate-open A through its real opener; assert unchanged order/focus/ID.
3. Click an exposed part of A; assert permitted activation differs from opener
   no-op and does not cross its band constraint.
4. Open Sector C in A's half; assert one atomic replacement, exactly two
   Sectors, new C identity, no transient third visible record.
5. Open System, Fleet, Defenses, and Missions details for known systems.
6. Alternate clicks on exposed portions across families; assert requested
   first-frame paint order and exactly one recipient action.
7. Repeat with all three nontrivial A/B/C permutations from Task 5A.
8. Reopen an existing detail and assert same ID/content plus raise behavior.
9. Close a Sector that was a navigation origin; under confirmed Root ownership,
   assert those details remain open.
10. Close an actual detail owner with a menu descendant and assert child-first
    teardown and valid focus restoration.

Retain both overlap screenshots and semantic recipient/order observations.
A paint-only check cannot prove that a lower raw-input row did not activate.

### Journey B: Presentation History and Stable Content

1. Change a detail tab, scroll offset, and selected row.
2. Minimize the detail and assert removal from visible order and one history ID.
3. Minimize details from the other families; assert shared history order.
4. Restore a middle entry through the rail and assert same ID/content, one
   history removal, and correct raise/focus.
5. Reopen a minimized detail through its normal opener; assert restore-and-raise.
6. Overflow history from 12 to 13 legitimate entries.
7. Assert exact oldest victim teardown, no duplicate IDs, and at most 12 entries.
8. Open a modal and attempt rail restore/close; assert history/content unchanged.
9. Dismiss the modal and repeat next frame; assert the requested rail action works.

Do not create artificial public fixed IDs to make these observations stable;
compare semantic keys and returned opaque IDs within the run.

### Journey C: Modal Pointer and Shortcut Boundary

For each available Finder, Message, Encyclopedia, Status, Build Selection,
Mission, Move, and Scrap where the fixture legitimately reaches it:

1. Open through its existing entry path and record current underlay state.
2. Click outside the modal over map, cockpit, rail, and exposed modeless detail.
3. Attempt scroll over an underlying list or map area.
4. Attempt F1–F7 and applicable root chords.
5. Assert modal remains the boundary and all unrelated selections/action
   counters/history/scene remain unchanged.
6. Exercise an allowed modal control to prove the barrier did not disable it.
7. Send Escape or the appropriate existing close gesture.
8. Assert no same-frame root action and no later orphan-release click.
9. Send a fresh root action in the next eligible frame and assert it works.
10. Duplicate-open a no-op singleton through an admissible command source and
    assert it preserves content/focus rather than resetting the surface.

Use the actual shortcut mapping: F1 Game Options, F2 System Finder policy with
missing content, F3 Fleet, F4 Troop, F5 Personnel, F6 Message, F7 Encyclopedia.
Do not report a successful System Finder content journey where no content exists.

A modal should not silently allow a forbidden duplicate-opening shortcut merely
to make the duplicate test easy. Exercise duplicate policy in manager/app tests
or through a legitimately admitted same-surface command source.

### Journey D: Focused Controls, Menus, and Gestures

1. Focus Encyclopedia search; type text and navigate its existing list.
2. Assert the focused child changes and no cockpit command runs.
3. Exercise Rename in a modeless detail; Enter commits once, Escape cancels
   only the editor, and the manager record count stays unchanged.
4. Raise another detail and return; verify stable widget IDs/content state.
5. Open an owner menu, then an eligible submenu; assert disjoint rectangles
   still route to one Menu identity.
6. Outside-press dismiss the menu over a cockpit control; assert dismissal only.
7. Repeat menu replacement and owner close; assert correct saved-focus fallback.
8. Begin a Fleet drag, move outside the originating rectangle, then release;
   assert the original capture owner handles completion.
9. Begin again, trigger a legitimate modal interruption, then release next
   frame; assert capture/payload clear and no drop or underlay click.
10. Repeat cancellation across close, minimize, scene change, and focus loss.

Tests that programmatically inject lifecycle interruption must label that setup
step. They still use actual pointer release to prove the orphan-release guard.

### Journey E: Failure, Scene, and First-Frame Cases

1. Use a fixture-only failure hook to make committed modal content unpaintable.
2. Assert the modal remains a blocker, diagnostics identify it, and an explicit
   close/recovery route exists.
3. Make a modeless render path temporarily unpaintable; assert it is retained.
4. Explicitly remove modeless content and assert receipt-driven teardown.
5. Transition to Tactical while Galaxy content remains registered.
6. Assert dormant Galaxy modals do not block normal battle input.
7. Open Alert/Results and assert command-center underlay remains blocked.
8. Dismiss/return and assert the trigger does not execute in the exposed scene.
9. Reset campaign after queuing a stale action; assert it cannot target a new ID.
10. Repeat resize/restore/open on a cold frame and a warm frame, on both factions.

Failure hooks are fixture-only and excluded from production artifacts. The
production journey covers reachable normal behavior without invoking them.

### Packaged Production Journey

Serve the actual packaged output, record its hashes, and use normal entry paths.
At minimum repeat cross-family overlap, Sector duplicate behavior, Encyclopedia
F7/search/Escape, an available command modal's outside blocking, rail restore,
and one Fleet gesture on Alliance and Empire.

There must be no dependency on fixture-only globals, request codes, mutation
hooks, or observation bridges. Visual inspection plus application logs and
ordinary outcomes establish this bounded production gate. Fixture-only deep
state assertions remain separate evidence.

The fixture script first builds production into `web/open-rebellion.wasm`, then
builds fixtures into the shared Cargo target and copies them to the isolated
`.artifacts/interface-parity/site/open-rebellion-test.wasm`. It does not copy the
fixture binary over the production `web` artifact. The shared Cargo target can
contain the most recent feature build, so never package it by hand. Record which
site and exact artifact each server serves; a fixture-site run is not production
evidence merely because the same script also built a production binary.

### Native Port Journey

Repeat the same behavior on the actual native application backend:

- Cold launch and resized viewport.
- Cross-family click-to-front with partially exposed lower windows.
- Encyclopedia opened with F7, focused text, modal outside click, Escape, and a
  fresh root action only after the closing lease.
- Available Finder/Message/Status and command-modal boundary representatives,
  with the remaining families exercised where reachable.
- Fleet press/motion/release outside bounds and interrupted-drag cancellation.
- Menu outside press and next-frame orphan release.
- Rail minimize/restore and preserved content state.
- Tactical Alert/Results and return to retained Galaxy content where reachable.
- Application focus loss/reacquisition with an active gesture.
- Both faction shells using the same policy path.

Record unreachable cases explicitly. If a required implementation gate cannot
run, leave delivery acceptance open even if the rest passes. Separate that
from original-game cases that remain intentionally unresolved in this scope.

## Exact Verification Commands and Artifact Discipline

All commands below are future execution commands. This planning revision does
not claim they have run against an implementation that does not yet exist.
Run from repository root unless a subshell says otherwise. On this host apply
the sanitized Cargo prefix stated in Global Constraints to every abbreviated
Cargo command, including commands embedded in task descriptions.

### First Safe Read-Only Commands

```bash
pwd
git status --short --branch
git remote -v
git rev-parse HEAD
git log -8 --oneline
rg --files -g AGENTS.md -g README.md -g Cargo.toml -g package.json
rg -n 'egui|egui-macroquad' Cargo.lock
rg -n 'FrameKeyboardOwner|pointer_blocked|strategic_input_enabled|move_to_top' crates
```

Read the complete applicable instructions and README files before edits.
`git fetch origin --prune` is the next branch-preparation operation, not part
of the read-only baseline. No branch, reset, stash, or cleanup is needed merely
to review this plan.

### Fresh Environment and Existing-Workspace Baseline

```bash
env PATH=/home/will/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin rustc --version
env PATH=/home/will/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin cargo --version
node --version
(cd tools/interface-parity && npm --version)
env PATH=/home/will/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin \
  cargo fmt --all -- --check
env PATH=/home/will/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin \
  cargo check --workspace --all-targets
env PATH=/home/will/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin \
  cargo clippy --workspace --all-targets -- -D warnings
node scripts/validate-interface-parity-ledgers.mjs
(cd tools/interface-parity && npm run test:unit)
```

Save command exit codes without piping them away. Do not install dependencies,
change a toolchain, or declare an environmental failure baseline merely because
an older plan mentioned it. Confirm the actual error first.

Recent history includes formatting/lint fixes (`393329f7`, `2aa46248`,
`b02fa5bf`); remeasure rather than copying historical strict-Clippy debt into
this implementation's acceptance record. An unrelated baseline is described by
its exact diagnostics, revision, and reproducibility, not a blanket waiver.

### Scoped Formatting, Compilation, and Tests

```bash
env PATH=/home/will/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin \
  cargo fmt --all -- --check
env PATH=/home/will/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin \
  cargo check -p rebellion-windowing --all-targets
env PATH=/home/will/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin \
  cargo check -p rebellion-render --all-targets
env PATH=/home/will/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin \
  cargo check -p rebellion-app --all-targets
env PATH=/home/will/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin \
  cargo check -p rebellion-render --all-targets --features interface-test-fixtures
env PATH=/home/will/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin \
  cargo check -p rebellion-app --all-targets --features interface-test-fixtures
env PATH=/home/will/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin \
  cargo test -p rebellion-windowing
env PATH=/home/will/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin \
  cargo test -p rebellion-render --features interface-test-fixtures
env PATH=/home/will/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin \
  cargo test -p rebellion-app --features interface-test-fixtures
env PATH=/home/will/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin \
  cargo test --workspace
env PATH=/home/will/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin \
  cargo clippy -p rebellion-windowing --all-targets -- -D warnings
```

Also rerun the existing workspace strict-Clippy baseline after implementation;
require no new diagnostics in touched or transitively affected code. Do not
repeat the full matrix after unchanged documentation-only wording edits unless
a check relevant to those edits requires it.

If Linux actually reports missing `-lasound`, verify the runtime library path
first. Only then use the task-local workaround rather than modifying system
configuration:

```bash
windowing_alsa_dir="$(mktemp -d)"
ln -s /lib/x86_64-linux-gnu/libasound.so.2 "$windowing_alsa_dir/libasound.so"
env LIBRARY_PATH="$windowing_alsa_dir" \
  PATH=/home/will/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin \
  cargo test --workspace
```

Record the workaround and directory in local logs. It is conditional environment
repair, not a required step or a proven current failure.

For compilation offload, follow the installed RCH skill when applicable and
retain worker/toolchain/artifact identity. This plan makes no assertion about
current worker health and does not silently substitute stale remote artifacts.

### Mutation and Invariant Validation

```bash
env PATH=/home/will/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin \
  cargo mutants -p rebellion-windowing \
    --file crates/rebellion-windowing/src/manager.rs \
    --file crates/rebellion-windowing/src/snapshot.rs
```

Inspect the installed cargo-mutants help before execution if flags changed.
`--in-diff` takes a diff file; a Git revision such as `HEAD` is not that file.
The file-scoped command above avoids that ambiguity.

Require behavioral mutants covering duplicate no-op, modal boundary, cascade,
history eviction, capture cancellation, and old-snapshot routing to be killed.
Classify equivalent/unviable mutants with concrete reasoning. A timeout is not
a killed mutant; rerun narrowly to distinguish runtime/environment problems.
Do not weaken tests or label survivors equivalent just to satisfy a percentage.

The kernel validator must be exercised after command sequences as well as by
private-state corruption tests. Include deterministic multi-step sequences
that interleave replace/minimize/modal/capture/close, because isolated method
tests can miss invalid combinations.

### Fixture Build and Changed-Family Browser Runs

```bash
(cd tools/interface-parity && npm run build)
(cd tools/interface-parity && npm run test:unit)
(cd tools/interface-parity && node windowing-contract.mjs --no-build)
(cd tools/interface-parity && node sector-quadrants.mjs --no-build)
(cd tools/interface-parity && node fleet-window.mjs --no-build)
(cd tools/interface-parity && node fleet-finder.mjs --no-build)
(cd tools/interface-parity && node build-selection.mjs --no-build)
(cd tools/interface-parity && node mission-dialog.mjs --no-build)
(cd tools/interface-parity && node status-window.mjs --no-build)
(cd tools/interface-parity && node message-index-shell.mjs --no-build)
(cd tools/interface-parity && node encyclopedia-surface.mjs)
(cd tools/interface-parity && node encyclopedia-canonical-surface.mjs)
(cd tools/interface-parity && node audit-batch.mjs --no-build)
```

The new `windowing-contract.mjs` must implement `--no-build` in Task 12.
`npm run build` calls `scripts/build-interface-test-wasm.sh`; it builds production
first, then an isolated fixture artifact/site, not a JavaScript-only bundle. Preserve each existing script's
actual supported flags rather than appending `--no-build` indiscriminately.
Check scripts that do not rebuild implicitly against the same artifact hash.

All runs are muted and retain console/page errors. Declare actual viewport and
faction coverage; do not infer two-faction coverage from one default run. Any
helper default that covers one faction must be explicitly looped by the new
contract runner.

### Production Package and Exclusion Check

Use a fresh version label to avoid overwriting another contributor's package:

```bash
./scripts/package-web.sh strategic-windowing-review
node tools/interface-parity/verify-production-exclusion.mjs
node scripts/validate-interface-parity-ledgers.mjs
```

`package-web.sh` compiles production WASM and writes
`dist/open-rebellion-web-strategic-windowing-review`; choose another unique
label if that directory already belongs to an existing run. The commonly used
`dev` label overwrites its matching package and is not intrinsically read-only.

At the inspected revision the exclusion script checks `web/open-rebellion.wasm`.
Therefore also hash the packaged WASM and confirm it equals the checked build,
or extend the verifier in Task 12 to accept the actual package path and test
that argument. Record the served URL/root and hash of the bytes actually served.
A check against a different artifact does not prove package exclusion.

Fixture build and production package are separate gates. Build order and hashes
must establish which one each browser server used. The production smoke journey
must work without fixture globals and must not inadvertently rebuild fixtures.

### Audit, Provenance, and Repository Hygiene

```bash
node scripts/validate-interface-parity-ledgers.mjs
rg -n 'FrameKeyboardOwner|move_to_top|pointer_blocked|strategic_input_enabled' \
  crates/rebellion-app/src crates/rebellion-render/src
rg -n 'is_key_pressed|is_mouse_button|mouse_wheel|mouse_position' \
  crates/rebellion-app/src crates/rebellion-render/src
git diff --check
git status --short
```

The authority scan requires human classification. `pointer_blocked` can remain
as a derived map interface; its mere presence is not a failure. A `move_to_top`
inside out-of-scope main-menu UI is different from one in a migrated strategic
feature. Conversely, a renamed local stack is still a violation.

If any touched helper crosses into the simulation code covered by provenance
rules, additionally run:

```bash
env PATH=/home/will/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin \
  cargo run -q -p provenance-scan -- check
```

Before future implementation commits, stage explicit paths only and inspect the
staged diff. Exclude `.beads/`, ignored progress/session files, generated packs,
original game data, local capture artifacts, and the competing plan files.
Attribute the actual implementing agent under workspace policy.

## Evidence Levels and Honest Completion States

| Claim | Minimum evidence | Insufficient substitute |
|---|---|---|
| Kernel implements invariant | Named passing unit/integration assertions | Type compiles |
| Egui realizes order | First/warm-frame paint + hit + widget-state proof | Repeated raise calls or one screenshot |
| Child key does not reach root | Child mutation plus unchanged root effect counter | Only checking `wants_keyboard_input` |
| Modal blocks underlay | Real outside input plus unchanged underlay outcome | Modal visually on top |
| No close fallthrough | Trigger-frame and later release observation | Next-frame screenshot alone |
| Fixture port behavior passes | Inspected both-faction fixture journey | One faction/default scenario |
| Production package passes | Checked served production artifact + real journey | Fixture run after package build |
| Native port behavior passes | Actual native backend interaction and artifacts | Browser or headless tests |
| Original Wine observation | Repeatable original run under identified Wine setup | Port fixture behavior |
| Native-Windows parity | Required Windows/original evidence under audit rules | Wine/native Linux/browser results |
| Audit cell accepted | All evidence required by that specific cell | Architecture implementation checkpoint |

Maintain separate statuses for implementation complete, browser accepted,
native-port accepted, and original/parity evidence still open. A bounded
checkpoint is useful but must not be described as final delivery acceptance.

The retained both-faction Wine ledger covers Encyclopedia input behavior,
cross-family modeless raising, Sector-close/detail survival, Status, Build
Selection, Create Mission, and Move Confirmation with selected artifact hashes.
Battle Alert did not trigger and Battle Results remained unreachable, so both
families retain provisional policy and open parity cells. The isolated display
was `:92`; do not confuse it with the workspace's shared `:91` capture service.

Retain evidence summaries and safe links in Git. Keep proprietary source assets,
screenshots where policy requires local storage, original saves, generated
packs, and runtime/session materials in the designated local capture workspace.
Do not include absolute local capture paths as supposedly portable public
artifacts without a repository-safe explanation/reference.

## Risks, Stop Conditions, and Rollback Boundaries

| Risk | Preventive gate | Failure response / rollback |
|---|---|---|
| Egui cannot realize arbitrary order through assumed API | T5A locked-dependency reproduction and first-frame tests | Stop migrations, revise adapter with failing proof |
| Detail owner inference contradicts normative table | T0 caller-to-stored-owner trace and explicit spec adjudication | Keep detail migration blocked until resolved |
| Built-in controls mutate before route filtering | TextEdit overlap test in T5A/T5B | Fix input bridge before family adoption |
| Legacy raw input bypasses migrated modal | T5B bridge and T8 regression | Revert affected family slice or repair shared boundary |
| Geometry uses old frame or final cursor only | Event-local coordinates and cold-resize tests | Correct preparation order; no kernel geometry workaround |
| Feature and manager disagree after failed open | Prepared-content transaction and receipts | Preserve old content; surface typed diagnostic |
| Sector/menu replacement partially succeeds | Atomic Replace tests | Reject before mutation; no compensating close guesses |
| Drag cancelled in manager but payload survives | T7B modal/close/scene cancellation tests | Clear payload and quarantine release through one dispatcher |
| Dormant Galaxy modal blocks Tactical | T11 scene restriction before route | Correct active membership, not forced root focus |
| Missing render auto-closes modal | Explicit ContentReport semantics | Retain blocker and unavailable close path |
| Fixture artifact mistaken for production | Build provenance and served-byte hashes | Rebuild/re-run correct package, invalidate bad claim |
| Native script assumes macOS injection on Linux | T13 platform preflight | Use actual owned-display input and record limitation |
| Runtime evidence unavailable | Separate acceptance statuses | Keep required gate open; no manufactured pass |
| Audit promotes unrelated cells | T14 per-cell evidence mapping | Revert documentation promotion without losing valid code |
| Main loop becomes unreviewable | Coordinator module and serial family slices | Split content migration from adapter change |

No save/replay format or simulation rules change. A family migration can be
reverted with its associated adapter wiring and tests without data migration.
Kernel/API changes should not be reverted independently underneath consumers;
rollback follows the dependency graph in reverse where necessary.

No implementation gate is satisfied by deleting its test, making a broad
exception, or weakening the specification to match a convenient renderer bug.
A source-backed correction such as the owner adjudication is documented and
reviewed separately from convenience-driven deviation.

## Remaining Evidence and Design Decisions

| Item | Current position | Required next action | Blocks |
|---|---|---|---|
| Detail lifetime owner | Final common assignment establishes strategic-root ownership | Preserve the recovered contract in implementation and tests | None |
| Exact egui realization hook | Repeated move-to-top rejected; managed host preferred candidate | T5A measured prototype and frozen signatures | All production migrations |
| Original pre-command pointer hook | Unknown implementation detail | Preserve observable boundary; no HWND imitation | Native-mechanism claim only |
| Menu outside dismissal | Narrow port policy; no generalized OwnerModal | Test bounded behavior and retain label | No blocker for explicit port policy |
| GID native family identity | Not fully recovered | Keep one port Menu presentation, note evidence gap | Native family attribution |
| System Finder content | Missing in port | Future feature work outside this plan | Full Finder content acceptance |
| Original cross-family order | Both factions reproduced modeless raising and Sector-close/detail survival under Wine | Retain native-Windows and port-side acceptance separately | Strict native-Windows parity claim |
| Status/Build/Mission/Move original captures | Both factions reproduced the API-shaping input boundary under Wine | Retain native-Windows and port-side acceptance separately | Strict native-Windows parity claim |
| Battle Results original capture | Unreachable in available saves | Future reachable original scenario | Original Results acceptance |
| Native Windows | Not established by current Wine evidence | Run required Windows checks separately | Windows parity claim |
| Native port environment | Execution-time observation | T13 actual backend/input method | Delivery native-port gate |
| Unknown native type 7/bit-0 classes | Semantics unresolved | No public kinds until evidence exists | Their implementation only |

The egui realization row remains a deliberate pre-implementation engineering
gate, not a claim that experimentation has already succeeded. Other rows
constrain scope and evidence claims without inventing speculative
infrastructure.

## Proposed Beads and Review Gates — Do Not Create Yet

These are planning labels, not real bead IDs. Each row corresponds to one
independently reviewable slice and inherits the exact acceptance steps from
its Task heading. No database mutation or JSONL export belongs to this task.

| Label | Depends on | Deliverable / decisive check |
|---|---|---|
| SW-00 | none | Fresh baseline and source-supported owner adjudication; exact logs/trace |
| SW-01 | SW-00 | Semantic vocabulary; `cargo test -p rebellion-windowing --test open_order` |
| SW-02 | SW-01 | Open/order/Replace atomicity; `--test open_order` |
| SW-03 | SW-02 | Lifecycle/history/focus/capture; `--test lifecycle` |
| SW-04 | SW-03 | Snapshot/reconcile/scene routes; `--test routing` plus scoped mutants |
| SW-05A | SW-04 | Locked egui feasibility; render `windowing_adapter` tests + inspected proof |
| SW-05B | SW-05A | Coordinator/event bridge; render `windowing` and app `strategic_windowing` tests |
| SW-06 | SW-05B | Sector duplicate/replacement; `sector_window` + Sector browser journey |
| SW-07A | SW-06 | Four detail identities/shared order; four module suites + cross-family case |
| SW-07B | SW-07A | Fleet capture/Rename child focus; Fleet/router/Rename tests |
| SW-07C | SW-07B | One history/rail authority; rail/lifecycle/coordinator tests |
| SW-08 | SW-05B | Available Finders/Message; module tests + both-faction journeys |
| SW-09 | SW-08 | Encyclopedia/Status controller preservation; app/render tests + journeys |
| SW-10 | SW-07C, SW-09 | Dialog/menu/GID route; module suites + modal/capture journey |
| SW-11 | SW-10 | Tactical scene-aware modals; tactical/coordinator tests |
| SW-12 | SW-06, SW-07C, SW-08, SW-09, SW-10, SW-11 | No legacy authority; full browser/package matrix |
| SW-13 | SW-12 | Actual native port journeys on both factions |
| SW-14 | SW-13 | Evidence/audit/roadmap reconciliation; ledger validator and inspected links |

When future authorized execution uses beads, retain the workspace's local-only
policy and explicit no-auto-import/no-auto-flush flags. Do not export JSONL as
routine cleanup. If coordinator-dispatched workers are used under separate
user authorization, workers retain evidence and leave closure to coordinator
verification under the applicable dispatch policy. This plan itself authorizes
neither worker assignment nor Agent Mail messages.

## Normative Requirement and Regression Traceability

| Requirement / regression | Task(s) | Decisive assertion or evidence |
|---|---|---|
| Semantic IDs instead of platform handles | T1–T2 | Opaque identity/key tests and normal dependency tree |
| Registry is not z-order | T2 | Lookup-order independence and arbitrary permutation |
| Independent state dimensions | T2–T4 | Invariant tests across order/history/focus/capture |
| Sector max two and same-half replacement | T2, T6 | Atomic replacement and both-faction Sector journey |
| Duplicate Sector does not raise | T2, T6 | No-op vs pointer activation comparison |
| Detail reopen restores/raises | T3, T7A | Same ID/content with expected order/history transition |
| Fleet detail identity is system-based | T5B, T7A | Fleet selection changes without WindowId change |
| Twelve-entry minimized history | T3, T7C | Exact thirteenth-entry eviction and restore |
| Singleton no-op preserves focus/content | T2, T8–T9 | Combined policy test and family controller tests |
| Detail owner matches actual source | T0, T7A | Traced final assignment and close behavior |
| Actual owner close cascades children | T3, T10 | Child-first receipts and saved focus eligibility |
| Modality independent of band | T1, T4 | Policy-axis and outside-modal route tests |
| Modal-owned transients remain usable | T4, T10 | Eligible descendant menu action with blocked underlay |
| Pointer chooses one topmost eligible hit | T4–T5B | Overlap recipient/action count and wheel isolation |
| Current geometry on cold/resize frame | T5A, T12–T13 | First/warm-frame hit and paint agreement |
| Built-in child input cannot mutate when blocked | T5A–T5B, T7B | Lower TextEdit unchanged under modal |
| Handled key stops at child | T5B, T7B–T9 | Text/Enter/Escape with unchanged root counters |
| Unhandled modal key cannot reach root | T4, T8–T10 | StopAtWindow route plus actual F-key journey |
| Modeless unhandled key may reach root once | T5B | Actual disposition and single root action |
| Close does not permit same-event fallthrough | T4–T5B, T8–T13 | Old lease plus same-frame action counters |
| Orphan release cannot activate exposed UI | T5B, T7B, T10 | Cross-frame cancellation/release journey |
| Temporary capture, not modal lifetime capture | T3, T7B | Exclusive bounded capture and interruption receipts |
| Missing modal stays fail closed | T4, T9, T12 | Unpaintable report plus outside-input failure hook |
| Paint absence is not content deletion | T4, T7C, T11 | Ready/minimized/Suspended/Unpaintable cases |
| Scene filtering precedes modal route | T4, T11 | Dormant Galaxy modal does not block Tactical |
| Campaign reset rejects stale work | T3, T5B, T11 | Non-reused IDs and cleared pending effects |
| Non-window holds remain explicit | T5B, T12 | Event-screen/briefing/fixture/scene route tests |
| Pause/base-child geometry stays scoped | T5B | Covered vs uncovered root hit comparison |
| All implemented strategic families migrate | T6–T11 | Policy inventory and authority scan |
| Rename is a child, not top-level | T7B | Stable record count and control focus assertions |
| GID/menu paths participate | T10 | Disjoint submenu hit union and dismissal |
| Battle Results common port policy | T11 | Tactical fixture route; original gap retained |
| Faction-independent policy | T5B, T12–T13 | One policy table and both-faction runs |
| Renderer/platform-independent kernel | T1–T4 | Dependency tree and API/diff review |
| No persistence/simulation redesign | all | File map, manifest, and final scope review |
| Fixture exclusions in production | T12 | Checked served production WASM hash |
| Browser and native port acceptance | T12–T13 | Separate retained run identities and artifacts |
| Audit truthfulness | T14 | Cell-specific evidence, validator, unresolved inventory |
| Unknown native classes excluded | T1, T14 | 19-kind inventory and evidence labels |

## Four-Pass Plan Self-Review

These are internal review passes, not claims of independent external reviewers
or successful implementation experiments.

**Pass 1 — evidence and repository fit.** Compared all three plans with the
specification, source traces, actual content/controller types, runtime manifest,
locked egui implementation, and build/native scripts. Corrected Fleet identity,
Encyclopedia controller placement, shortcut mapping, fixture numbering, stale
baseline assumptions, and unjustified original/native acceptance claims. Detail
ownership remains an explicit source-adjudication prerequisite.

**Pass 2 — architecture and event correctness.** Kept identities, registry,
order, history, focus, capture, and content independent. Replaced sequential
replacement and paint-absence reconciliation with explicit atomic/reporting
contracts. Added event consumption, blocked built-in widget mutation, current
geometry, cross-frame orphan release, scene restriction, and monotonic reset.
Rejected repeated-raise ordering and gated its replacement on measured proof.

**Pass 3 — execution and command audit.** Split adapter proof from integration,
and details from gestures and rail. Every behavior slice has named assertions,
red/green commands, integration checks, and commit scope. Runtime/documentation
slices have objective evidence checks. Dependencies include the early owner and
adapter gates. Native commands include evidence-directory requirements and the
macOS-only injection limitation; fixture and production artifacts are distinct.

**Pass 4 — consistency, scope, and delivery claims.** Rechecked task/API names,
policy inventory, dependency edges, evidence labels, and legacy-removal gates.
The plan preserves useful content stores without preserving competing authority.
No OwnerModal, Rename top-level, fixed public IDs, kernel geometry, false-success
stubs, speculative native classes, or automatic audit promotion is introduced.
The final review validates this document and its full before/after diff; it does
not claim the future kernel, adapter, browser, or native tests have passed.

The resulting execution plan is intentionally more explicit where mistakes are
expensive: adapter feasibility, event consumption, lifecycle transactions,
intermediate migration safety, and evidence provenance. It leaves feature
content algorithms and unrelated visual redesign outside the work.
