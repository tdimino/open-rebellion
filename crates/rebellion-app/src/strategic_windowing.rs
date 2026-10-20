//! Application-owned strategic window coordinator and commit boundary.

use std::collections::BTreeSet;

use rebellion_data::encyclopedia_presenter::EncyclopediaEntryIntent;
use rebellion_data::encyclopedia_session::EncyclopediaSession;
use rebellion_render::{
    build_selection::{PreparedBuildSelection, PreparedBuildSelectionConfirm},
    defenses_window::PreparedDefensesWindow,
    fleet_finder::{PreparedFleetFinder, FLEET_FINDER_NAME_CONTROL},
    fleet_window::PreparedFleetWindow,
    message_index::PreparedMessageIndex,
    missions_window::PreparedMissionsWindow,
    personnel_finder::{PreparedPersonnelFinder, PERSONNEL_FINDER_NAME_CONTROL},
    sector_window::PreparedSectorWindow,
    status_window::PreparedStatusWindow,
    system_window::PreparedSystemWindow,
    tactical_view::TacticalWindowPhase,
    troop_finder::{PreparedTroopFinder, TROOP_FINDER_NAME_CONTROL},
    windowing_policy::{contextual_spec, SUBJECT_BATTLE, SUBJECT_COMMAND_CONTEXT},
};
use rebellion_windowing::{
    BubblePolicy, ContentReport, ContentStatus, ExistingEffect, FocusTarget, ReconcileReport,
    RouteTarget, RoutingSnapshot, SubjectId, WindowCommand, WindowCommit, WindowError, WindowEvent,
    WindowId, WindowKind, WindowOwner, WindowTransition,
};

use crate::encyclopedia_surface::{EncyclopediaSurfaceController, EncyclopediaWindowOpenError};

/// Result of applying one queued command at the dispatch commit boundary.
#[derive(Debug)]
pub struct AppliedWindowCommand {
    /// Original queued command.
    pub command: WindowCommand,
    /// Atomic kernel result.
    pub result: Result<WindowCommit, WindowError>,
    /// Matching attached content identities released by teardown receipts.
    pub removed_content: Vec<WindowId>,
    /// Build Selection content prepared beside its singleton open.
    pub prepared_build_selection: Option<PreparedBuildSelection>,
    /// Identity whose attachment is valid for `prepared_build_selection`.
    pub build_selection_content_id: Option<WindowId>,
    /// Gameplay-free confirmation released only by its exact accepted close.
    pub confirmed_build_selection: Option<PreparedBuildSelectionConfirm>,
    /// System content prepared alongside this semantic open.
    pub prepared_system: Option<PreparedSystemWindow>,
    /// Identity whose attachment is valid for `prepared_system` after commit.
    pub system_content_id: Option<WindowId>,
    /// Defenses content prepared alongside this semantic open.
    pub prepared_defenses: Option<PreparedDefensesWindow>,
    /// Identity whose attachment is valid for `prepared_defenses` after commit.
    pub defenses_content_id: Option<WindowId>,
    /// Missions content prepared alongside this semantic open.
    pub prepared_missions: Option<PreparedMissionsWindow>,
    /// Identity whose attachment is valid for `prepared_missions` after commit.
    pub missions_content_id: Option<WindowId>,
    /// Fleet content prepared alongside this command, never published before
    /// the manager accepts the semantic identity.
    pub prepared_fleet: Option<PreparedFleetWindow>,
    /// Identity whose attachment is valid for `prepared_fleet` after commit.
    pub fleet_content_id: Option<WindowId>,
    /// Fleet Finder content prepared beside its singleton open.
    pub prepared_fleet_finder: Option<PreparedFleetFinder>,
    /// Identity whose attachment is valid for `prepared_fleet_finder`.
    pub fleet_finder_content_id: Option<WindowId>,
    /// Troop Finder content prepared beside its singleton open.
    pub prepared_troop_finder: Option<PreparedTroopFinder>,
    /// Identity whose attachment is valid for `prepared_troop_finder`.
    pub troop_finder_content_id: Option<WindowId>,
    /// Personnel Finder content prepared beside its singleton open.
    pub prepared_personnel_finder: Option<PreparedPersonnelFinder>,
    /// Identity whose attachment is valid for `prepared_personnel_finder`.
    pub personnel_finder_content_id: Option<WindowId>,
    /// Message Index content prepared beside its singleton open.
    pub prepared_message_index: Option<PreparedMessageIndex>,
    /// Identity whose attachment is valid for `prepared_message_index`.
    pub message_index_content_id: Option<WindowId>,
    /// Status content prepared beside its contextual open or replacement.
    pub prepared_status: Option<PreparedStatusWindow>,
    /// Identity whose attachment is valid for `prepared_status`.
    pub status_content_id: Option<WindowId>,
    /// Sector content prepared beside a duplicate open or atomic replacement.
    pub prepared_sector: Option<PreparedSectorWindow>,
    /// Identity whose attachment is valid for `prepared_sector` after commit.
    pub sector_content_id: Option<WindowId>,
}

/// Exact Encyclopedia identity and ordered manager receipts committed together.
#[derive(Debug)]
pub struct CommittedEncyclopediaOpen {
    /// Singleton identity accepted by the manager.
    pub window_id: WindowId,
    /// Capture/focus receipts emitted by the open and semantic-focus commits.
    pub events: Vec<WindowEvent>,
}

/// Exact manager identity of the tactical modal for one battle session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TacticalWindowIdentity {
    /// Opaque manager identity for this modal phase.
    pub id: WindowId,
    /// Stable subject shared by every phase of one battle session.
    pub subject: SubjectId,
    /// Distinct Alert or Results semantic kind.
    pub kind: WindowKind,
}

/// Atomic result of synchronizing the manager with the tactical phase.
#[derive(Debug)]
pub struct CommittedTacticalPhase {
    /// Active tactical modal after the commit, if this phase has one.
    pub window: Option<TacticalWindowIdentity>,
    /// Ordered focus and capture receipts emitted by the commit.
    pub events: Vec<WindowEvent>,
    /// Exact retired tactical content identities.
    pub removed_content: Vec<WindowId>,
}

#[derive(Debug)]
struct QueuedWindowCommand {
    command: WindowCommand,
    prepared_build_selection: Option<PreparedBuildSelection>,
    prepared_build_selection_confirm: Option<PreparedBuildSelectionConfirm>,
    prepared_system: Option<PreparedSystemWindow>,
    prepared_defenses: Option<PreparedDefensesWindow>,
    prepared_missions: Option<PreparedMissionsWindow>,
    prepared_fleet: Option<PreparedFleetWindow>,
    prepared_fleet_finder: Option<PreparedFleetFinder>,
    prepared_troop_finder: Option<PreparedTroopFinder>,
    prepared_personnel_finder: Option<PreparedPersonnelFinder>,
    prepared_message_index: Option<PreparedMessageIndex>,
    prepared_status: Option<PreparedStatusWindow>,
    prepared_sector: Option<PreparedSectorWindow>,
}

/// Application scene whose registered windows currently participate in routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowScene {
    /// Strategic command center and galaxy map.
    Galaxy,
    /// Tactical combat scene.
    Tactical,
    /// Temporary application surface that suspends strategic input.
    Suspended,
}

/// Non-window application surfaces that hold strategic input.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ApplicationInputLocks {
    /// Advisor briefing sequence.
    pub briefing: bool,
    /// Deterministic test fixture setup.
    pub fixture_setup: bool,
    /// In-progress scene transition.
    pub scene_transition: bool,
}

impl ApplicationInputLocks {
    /// Whether any application surface blocks strategic dispatch.
    #[must_use]
    pub const fn blocks_strategic_input(self) -> bool {
        self.briefing || self.fixture_setup || self.scene_transition
    }
}

/// Temporary, explicitly named owners that still live outside the shared
/// manager during family-by-family migration. They may only make a route more
/// restrictive; they never allocate or mirror a [`WindowId`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LegacyInputBlockers {
    /// A legacy top-level with an application-modal input boundary.
    pub strategic_modal: bool,
    /// A legacy edit/list control that owns keyboard input for this frame.
    pub focused_control: bool,
    /// A legacy transient menu that owns dismissal and item input.
    pub transient_menu: bool,
    /// A legacy root gesture, such as target selection, holding the pointer.
    pub root_gesture: bool,
}

impl LegacyInputBlockers {
    const fn blocks_migrated_input(self) -> bool {
        self.strategic_modal || self.focused_control || self.transient_menu || self.root_gesture
    }

    const fn blocks_root_keyboard(self) -> bool {
        self.strategic_modal || self.focused_control || self.transient_menu
    }

    const fn blocks_root_pointer(self) -> bool {
        self.strategic_modal || self.transient_menu || self.root_gesture
    }

    /// Whether dispatch introduced a restrictive legacy owner that was not
    /// present in this frozen frame-start state. Releases are deliberately not
    /// reported: they cannot make the current lease permissive.
    #[must_use]
    pub const fn new_restriction_started(self, current: Self) -> bool {
        (!self.strategic_modal && current.strategic_modal)
            || (!self.focused_control && current.focused_control)
            || (!self.transient_menu && current.transient_menu)
            || (!self.root_gesture && current.root_gesture)
    }
}

/// Sticky record of legacy ownership that starts during one production frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegacyInputRestrictionLatch {
    frozen: LegacyInputBlockers,
    started: bool,
}

impl LegacyInputRestrictionLatch {
    #[must_use]
    pub const fn new(frozen: LegacyInputBlockers) -> Self {
        Self {
            frozen,
            started: false,
        }
    }

    pub fn observe(&mut self, current: LegacyInputBlockers) {
        self.started |= self.frozen.new_restriction_started(current);
    }

    pub fn note_start(&mut self) {
        self.started = true;
    }

    #[must_use]
    pub const fn started(self) -> bool {
        self.started
    }
}

/// Notices operating-system focus losses between frames. A loss cancels the
/// manager's pointer capture, since the release that would end it may never
/// arrive.
#[derive(Debug, Default)]
pub struct FocusLossWatch {
    seen: Option<u32>,
}

impl FocusLossWatch {
    /// Whether `count` records a loss since the previous observation. The
    /// first observation only sets the baseline.
    pub fn lost(&mut self, count: u32) -> bool {
        let lost = self.seen.is_some_and(|seen| seen != count);
        self.seen = Some(count);
        lost
    }
}

/// Running count of focus losses for a host that reports focus as a state
/// sampled each frame (AppKit's `-[NSApplication isActive]`) rather than as
/// an event. Each active-to-inactive change is one loss.
#[derive(Debug, Default)]
#[cfg_attr(
    not(target_os = "macos"),
    allow(dead_code, reason = "only the macOS host samples its focus state")
)]
pub struct FocusStateCounter {
    was_active: bool,
    losses: u32,
}

#[cfg_attr(
    not(target_os = "macos"),
    allow(dead_code, reason = "only the macOS host samples its focus state")
)]
impl FocusStateCounter {
    /// Records one sample and returns the losses counted so far.
    pub fn sample(&mut self, active: bool) -> u32 {
        if self.was_active && !active {
            self.losses = self.losses.wrapping_add(1);
        }
        self.was_active = active;
        self.losses
    }
}

/// Immutable production routing lease for one frame.
///
/// The manager snapshot and legacy restriction state are copied together before
/// dispatch. Later closes or hold releases cannot make this lease permissive;
/// callers must construct the next frame boundary to observe them.
#[derive(Debug, Clone)]
pub struct StrategicFrameBoundary {
    snapshot: RoutingSnapshot,
    locks: ApplicationInputLocks,
    legacy: LegacyInputBlockers,
}

impl StrategicFrameBoundary {
    #[must_use]
    pub const fn snapshot(&self) -> &RoutingSnapshot {
        &self.snapshot
    }

    /// Whether the normalized migrated router must fail closed behind a legacy
    /// owner. This does not register that owner in the shared manager.
    #[must_use]
    pub const fn migrated_input_held(&self) -> bool {
        self.locks.blocks_strategic_input() || self.legacy.blocks_migrated_input()
    }

    /// Whether a manager-owned modal excludes every retained legacy surface.
    /// The app uses this as a temporary bridge around the single legacy egui
    /// pass; migrated content is painted after that bridge is released.
    #[must_use]
    pub fn migrated_modal_active(&self) -> bool {
        self.snapshot.active_modal().is_some()
    }

    /// Whether an active legacy surface may read input at all. Legacy blockers
    /// do not disable their own controls; manager routes and application holds do.
    #[must_use]
    pub fn legacy_surface_input_allowed(&self) -> bool {
        !self.locks.blocks_strategic_input()
            && self.snapshot.pointer_allows_root()
            && matches!(
                self.snapshot.keyboard_route().target,
                RouteTarget::StrategicRoot
            )
    }

    /// Permit for the retained legacy detail windows (Sector, System,
    /// Defenses, Missions) to read input. Application locks and legacy
    /// strategic modals hold them as they hold migrated content: outside
    /// clicks under Encyclopedia, Status, Build Selection, Create Mission and
    /// Move Confirmation reach no underlying control in either faction
    /// (`ghidra/notes/strategic-windowing-toolkit.md`, "Strategic
    /// modality"). Transient menus and root gestures keep their own rules.
    #[must_use]
    pub fn retained_window_input_allowed(&self) -> bool {
        !self.migrated_modal_active()
            && !self.locks.blocks_strategic_input()
            && !self.legacy.strategic_modal
    }

    /// Permit for one manager-owned surface in this frozen frame.
    ///
    /// Unlike a legacy/root permit, a transient Menu remains interactive while
    /// its dismissal boundary deliberately makes `pointer_allows_root` false.
    #[must_use]
    pub fn managed_window_input_allowed(&self, window: WindowId) -> bool {
        !self.migrated_input_held() && self.snapshot.allows_input(window)
    }

    /// Permit for a named legacy pointer owner itself. Unlike the strategic
    /// underlay permit, this ignores legacy blockers so an active owner can close.
    #[must_use]
    pub fn legacy_owned_pointer_allowed(&self) -> bool {
        !self.locks.blocks_strategic_input() && self.snapshot.pointer_allows_root()
    }

    /// Permit for legacy pointer polling at the strategic underlay/root.
    #[must_use]
    pub fn legacy_pointer_allowed(&self) -> bool {
        self.legacy_owned_pointer_allowed() && !self.legacy.blocks_root_pointer()
    }

    /// Permit for legacy keyboard polling at the strategic root.
    #[must_use]
    #[allow(
        dead_code,
        reason = "family migrations consume the root-keyboard permit independently"
    )]
    pub fn legacy_keyboard_allowed(&self) -> bool {
        !self.locks.blocks_strategic_input()
            && !self.legacy.blocks_root_keyboard()
            && matches!(
                self.snapshot.keyboard_route().target,
                RouteTarget::StrategicRoot
            )
    }

    /// Root shortcuts use the frozen keyboard lease. A migrated modeless owner
    /// may still bubble an unhandled event, so the router decides event-locally.
    #[must_use]
    pub fn root_shortcuts_allowed(&self) -> bool {
        !self.locks.blocks_strategic_input()
            && !self.legacy.blocks_root_keyboard()
            && self.snapshot.keyboard_route().bubble == BubblePolicy::ToRoot
    }
}

/// Sole application authority for manager commands, scenes, and content receipts.
#[derive(Debug)]
pub struct StrategicWindowing {
    manager: rebellion_windowing::WindowManager,
    pending: Vec<QueuedWindowCommand>,
    pending_galaxy: Vec<QueuedWindowCommand>,
    applying: Vec<QueuedWindowCommand>,
    scene: WindowScene,
    galaxy: BTreeSet<WindowId>,
    tactical: BTreeSet<WindowId>,
    attached_content: BTreeSet<WindowId>,
    next_context_subject: Option<u64>,
    next_battle_subject: Option<u64>,
    battle_session_token: Option<u64>,
    battle_subject: Option<SubjectId>,
    tactical_window: Option<TacticalWindowIdentity>,
}

impl Default for StrategicWindowing {
    fn default() -> Self {
        Self::new()
    }
}

impl StrategicWindowing {
    /// Creates an empty Galaxy-scene coordinator.
    #[must_use]
    pub fn new() -> Self {
        Self {
            manager: rebellion_windowing::WindowManager::new(),
            pending: Vec::new(),
            pending_galaxy: Vec::new(),
            applying: Vec::new(),
            scene: WindowScene::Galaxy,
            galaxy: BTreeSet::new(),
            tactical: BTreeSet::new(),
            attached_content: BTreeSet::new(),
            next_context_subject: Some(1),
            next_battle_subject: Some(1),
            battle_session_token: None,
            battle_subject: None,
            tactical_window: None,
        }
    }

    /// Current application scene.
    #[must_use]
    pub const fn scene(&self) -> WindowScene {
        self.scene
    }

    /// Current immutable input and paint lease.
    #[must_use]
    pub fn begin_frame(&self) -> RoutingSnapshot {
        self.manager.begin_frame()
    }

    /// Freezes the manager, application locks, and named legacy restrictions
    /// before production input dispatch starts.
    #[must_use]
    pub fn begin_production_frame(
        &self,
        locks: ApplicationInputLocks,
        legacy: LegacyInputBlockers,
    ) -> StrategicFrameBoundary {
        StrategicFrameBoundary {
            snapshot: self.begin_frame(),
            locks,
            legacy,
        }
    }

    /// Queues one command without mutating the current frame's state.
    #[allow(
        dead_code,
        reason = "feature-family migrations enqueue the first commands"
    )]
    pub fn enqueue(&mut self, command: WindowCommand) {
        self.pending.push(QueuedWindowCommand {
            command,
            prepared_build_selection: None,
            prepared_build_selection_confirm: None,
            prepared_system: None,
            prepared_defenses: None,
            prepared_missions: None,
            prepared_fleet: None,
            prepared_fleet_finder: None,
            prepared_troop_finder: None,
            prepared_personnel_finder: None,
            prepared_message_index: None,
            prepared_status: None,
            prepared_sector: None,
        });
    }

    /// Queues validated Build Selection context beside its singleton open.
    pub fn enqueue_build_selection_open(&mut self, prepared: PreparedBuildSelection) {
        self.pending.push(QueuedWindowCommand {
            command: WindowCommand::Open(prepared.spec()),
            prepared_build_selection: Some(prepared),
            prepared_build_selection_confirm: None,
            prepared_system: None,
            prepared_defenses: None,
            prepared_missions: None,
            prepared_fleet: None,
            prepared_fleet_finder: None,
            prepared_troop_finder: None,
            prepared_personnel_finder: None,
            prepared_message_index: None,
            prepared_status: None,
            prepared_sector: None,
        });
    }

    /// Queues the exact close paired with an inert Build Selection confirm.
    /// The candidate is returned only if that close commits and releases the
    /// attached content identity.
    pub fn enqueue_build_selection_confirm(&mut self, prepared: PreparedBuildSelectionConfirm) {
        self.pending.push(QueuedWindowCommand {
            command: WindowCommand::Close(prepared.window_id()),
            prepared_build_selection: None,
            prepared_build_selection_confirm: Some(prepared),
            prepared_system: None,
            prepared_defenses: None,
            prepared_missions: None,
            prepared_fleet: None,
            prepared_fleet_finder: None,
            prepared_troop_finder: None,
            prepared_personnel_finder: None,
            prepared_message_index: None,
            prepared_status: None,
            prepared_sector: None,
        });
    }

    /// Queues validated System content beside its semantic detail open.
    pub fn enqueue_system_open(&mut self, prepared: PreparedSystemWindow) {
        self.pending.push(QueuedWindowCommand {
            command: WindowCommand::Open(prepared.spec()),
            prepared_build_selection: None,
            prepared_build_selection_confirm: None,
            prepared_system: Some(prepared),
            prepared_defenses: None,
            prepared_missions: None,
            prepared_fleet: None,
            prepared_fleet_finder: None,
            prepared_troop_finder: None,
            prepared_personnel_finder: None,
            prepared_message_index: None,
            prepared_status: None,
            prepared_sector: None,
        });
    }

    /// Queues validated Sector content beside either an exact duplicate open
    /// or one atomic same-half replacement. The renderer remains untouched
    /// until the matching manager receipt is published.
    pub fn enqueue_sector_open(&mut self, prepared: PreparedSectorWindow) {
        let new = prepared.spec();
        self.pending.push(QueuedWindowCommand {
            command: prepared
                .replacing()
                .map_or(WindowCommand::Open(new), |old| WindowCommand::Replace {
                    old,
                    new,
                }),
            prepared_build_selection: None,
            prepared_build_selection_confirm: None,
            prepared_system: None,
            prepared_defenses: None,
            prepared_missions: None,
            prepared_fleet: None,
            prepared_fleet_finder: None,
            prepared_troop_finder: None,
            prepared_personnel_finder: None,
            prepared_message_index: None,
            prepared_status: None,
            prepared_sector: Some(prepared),
        });
    }

    /// Revalidates one in-flight Sector intent before the manager can apply its
    /// destructive Replace. A vanished candidate is dropped, not committed.
    pub fn refresh_queued_sector_open(
        &mut self,
        previous: PreparedSectorWindow,
        refreshed: Option<PreparedSectorWindow>,
    ) -> bool {
        for queue in [&mut self.applying, &mut self.pending] {
            let Some(index) = queue
                .iter()
                .position(|queued| queued.prepared_sector == Some(previous))
            else {
                continue;
            };
            if let Some(refreshed) = refreshed {
                let new = refreshed.spec();
                let queued = &mut queue[index];
                queued.command = refreshed
                    .replacing()
                    .map_or(WindowCommand::Open(new), |old| WindowCommand::Replace {
                        old,
                        new,
                    });
                queued.prepared_sector = Some(refreshed);
            } else {
                queue.remove(index);
            }
            return true;
        }
        false
    }

    /// Queues validated Defenses content beside its semantic detail open.
    pub fn enqueue_defenses_open(&mut self, prepared: PreparedDefensesWindow) {
        self.pending.push(QueuedWindowCommand {
            command: WindowCommand::Open(prepared.spec()),
            prepared_build_selection: None,
            prepared_build_selection_confirm: None,
            prepared_system: None,
            prepared_defenses: Some(prepared),
            prepared_missions: None,
            prepared_fleet: None,
            prepared_fleet_finder: None,
            prepared_troop_finder: None,
            prepared_personnel_finder: None,
            prepared_message_index: None,
            prepared_status: None,
            prepared_sector: None,
        });
    }

    /// Queues validated Missions content beside its semantic detail open.
    pub fn enqueue_missions_open(&mut self, prepared: PreparedMissionsWindow) {
        self.pending.push(QueuedWindowCommand {
            command: WindowCommand::Open(prepared.spec()),
            prepared_build_selection: None,
            prepared_build_selection_confirm: None,
            prepared_system: None,
            prepared_defenses: None,
            prepared_missions: Some(prepared),
            prepared_fleet: None,
            prepared_fleet_finder: None,
            prepared_troop_finder: None,
            prepared_personnel_finder: None,
            prepared_message_index: None,
            prepared_status: None,
            prepared_sector: None,
        });
    }

    /// Queues validated System content for publication only after Galaxy is
    /// active. Battle Results destinations must not join Tactical membership.
    pub fn enqueue_galaxy_system_open(&mut self, prepared: PreparedSystemWindow) {
        self.pending_galaxy.push(QueuedWindowCommand {
            command: WindowCommand::Open(prepared.spec()),
            prepared_build_selection: None,
            prepared_build_selection_confirm: None,
            prepared_system: Some(prepared),
            prepared_defenses: None,
            prepared_missions: None,
            prepared_fleet: None,
            prepared_fleet_finder: None,
            prepared_troop_finder: None,
            prepared_personnel_finder: None,
            prepared_message_index: None,
            prepared_status: None,
            prepared_sector: None,
        });
    }

    /// Queues a validated Fleet content candidate beside its semantic open.
    /// The candidate returns to the app only after the manager applies it.
    pub fn enqueue_fleet_open(&mut self, prepared: PreparedFleetWindow) {
        self.pending.push(QueuedWindowCommand {
            command: WindowCommand::Open(prepared.spec()),
            prepared_build_selection: None,
            prepared_build_selection_confirm: None,
            prepared_system: None,
            prepared_defenses: None,
            prepared_missions: None,
            prepared_fleet: Some(prepared),
            prepared_fleet_finder: None,
            prepared_troop_finder: None,
            prepared_personnel_finder: None,
            prepared_message_index: None,
            prepared_status: None,
            prepared_sector: None,
        });
    }

    /// Queues validated Fleet content for publication only after Galaxy is
    /// active, preserving the outgoing Results frame's immutable lease.
    pub fn enqueue_galaxy_fleet_open(&mut self, prepared: PreparedFleetWindow) {
        self.pending_galaxy.push(QueuedWindowCommand {
            command: WindowCommand::Open(prepared.spec()),
            prepared_build_selection: None,
            prepared_build_selection_confirm: None,
            prepared_system: None,
            prepared_defenses: None,
            prepared_missions: None,
            prepared_fleet: Some(prepared),
            prepared_fleet_finder: None,
            prepared_troop_finder: None,
            prepared_personnel_finder: None,
            prepared_message_index: None,
            prepared_status: None,
            prepared_sector: None,
        });
    }

    /// Queues inert Fleet Finder content beside its semantic singleton open.
    pub fn enqueue_fleet_finder_open(&mut self, prepared: PreparedFleetFinder) {
        self.pending.push(QueuedWindowCommand {
            command: WindowCommand::Open(prepared.spec()),
            prepared_build_selection: None,
            prepared_build_selection_confirm: None,
            prepared_system: None,
            prepared_defenses: None,
            prepared_missions: None,
            prepared_fleet: None,
            prepared_fleet_finder: Some(prepared),
            prepared_troop_finder: None,
            prepared_personnel_finder: None,
            prepared_message_index: None,
            prepared_status: None,
            prepared_sector: None,
        });
    }

    /// Queues inert Troop Finder content beside its semantic singleton open.
    pub fn enqueue_troop_finder_open(&mut self, prepared: PreparedTroopFinder) {
        self.pending.push(QueuedWindowCommand {
            command: WindowCommand::Open(prepared.spec()),
            prepared_build_selection: None,
            prepared_build_selection_confirm: None,
            prepared_system: None,
            prepared_defenses: None,
            prepared_missions: None,
            prepared_fleet: None,
            prepared_fleet_finder: None,
            prepared_troop_finder: Some(prepared),
            prepared_personnel_finder: None,
            prepared_message_index: None,
            prepared_status: None,
            prepared_sector: None,
        });
    }

    /// Queues inert Personnel Finder content beside its semantic singleton open.
    pub fn enqueue_personnel_finder_open(&mut self, prepared: PreparedPersonnelFinder) {
        self.pending.push(QueuedWindowCommand {
            command: WindowCommand::Open(prepared.spec()),
            prepared_build_selection: None,
            prepared_build_selection_confirm: None,
            prepared_system: None,
            prepared_defenses: None,
            prepared_missions: None,
            prepared_fleet: None,
            prepared_fleet_finder: None,
            prepared_troop_finder: None,
            prepared_personnel_finder: Some(prepared),
            prepared_message_index: None,
            prepared_status: None,
            prepared_sector: None,
        });
    }

    /// Queues inert Message Index content beside its semantic singleton open.
    pub fn enqueue_message_index_open(&mut self, prepared: PreparedMessageIndex) {
        self.pending.push(QueuedWindowCommand {
            command: WindowCommand::Open(prepared.spec()),
            prepared_build_selection: None,
            prepared_build_selection_confirm: None,
            prepared_system: None,
            prepared_defenses: None,
            prepared_missions: None,
            prepared_fleet: None,
            prepared_fleet_finder: None,
            prepared_troop_finder: None,
            prepared_personnel_finder: None,
            prepared_message_index: Some(prepared),
            prepared_status: None,
            prepared_sector: None,
        });
    }

    /// Queues validated contextual Status content. A later logical open
    /// replaces the exact prior Status identity only after preparation has
    /// succeeded; the renderer publishes from the matching manager receipt.
    pub fn enqueue_status_open(
        &mut self,
        prepared: PreparedStatusWindow,
        replacing: Option<WindowId>,
    ) {
        let new = prepared.spec();
        self.pending.push(QueuedWindowCommand {
            command: replacing.map_or(WindowCommand::Open(new), |old| WindowCommand::Replace {
                old,
                new,
            }),
            prepared_build_selection: None,
            prepared_build_selection_confirm: None,
            prepared_system: None,
            prepared_defenses: None,
            prepared_missions: None,
            prepared_fleet: None,
            prepared_fleet_finder: None,
            prepared_troop_finder: None,
            prepared_personnel_finder: None,
            prepared_message_index: None,
            prepared_status: Some(prepared),
            prepared_sector: None,
        });
    }

    /// Queues manager-owned System minimization.
    pub fn enqueue_system_minimize(&mut self, window: WindowId) {
        self.enqueue(WindowCommand::Minimize(window));
    }

    /// Queues manager-owned Defenses minimization.
    pub fn enqueue_defenses_minimize(&mut self, window: WindowId) {
        self.enqueue(WindowCommand::Minimize(window));
    }

    /// Queues manager-owned Missions minimization.
    pub fn enqueue_missions_minimize(&mut self, window: WindowId) {
        self.enqueue(WindowCommand::Minimize(window));
    }

    /// Commits a validated Encyclopedia singleton after this frame's routing
    /// and painting have finished. Content becomes attached only with the
    /// exact identity accepted by the manager.
    pub fn commit_encyclopedia_open(
        &mut self,
        controller: &mut EncyclopediaSurfaceController,
        session: &EncyclopediaSession,
        intent: EncyclopediaEntryIntent,
    ) -> Result<CommittedEncyclopediaOpen, EncyclopediaWindowOpenError> {
        let id = controller.open(session, intent, &mut self.manager)?;
        let events = controller.take_manager_events();
        self.current_scene_mut().insert(id);
        self.attached_content.insert(id);
        Ok(CommittedEncyclopediaOpen {
            window_id: id,
            events,
        })
    }

    /// Queues manager-owned Fleet minimization.
    pub fn enqueue_fleet_minimize(&mut self, window: WindowId) {
        self.enqueue(WindowCommand::Minimize(window));
    }

    /// Applies queued commands in insertion order after input and painting finish.
    #[cfg(test)]
    pub fn apply_pending(&mut self) -> Vec<AppliedWindowCommand> {
        let count = self.begin_apply_batch();
        (0..count)
            .filter_map(|_| self.apply_next_pending())
            .collect()
    }

    /// Freeze the exact command batch. Receipt-side enqueues remain pending
    /// for the next frame even if an invalid command is removed here.
    pub fn begin_apply_batch(&mut self) -> usize {
        if self.scene == WindowScene::Suspended {
            return 0;
        }
        if self.applying.is_empty() {
            if self.scene == WindowScene::Galaxy {
                self.applying = std::mem::take(&mut self.pending_galaxy);
            }
            self.applying.extend(std::mem::take(&mut self.pending));
        }
        self.applying.len()
    }

    /// Applies exactly one queued command, allowing the app to publish its
    /// receipt and refresh the next Sector candidate before another mutation.
    pub fn apply_next_pending(&mut self) -> Option<AppliedWindowCommand> {
        if self.scene == WindowScene::Suspended {
            return None;
        }
        if self.applying.is_empty() {
            return None;
        }
        let queued = self.applying.remove(0);
        Some(self.apply_queued(queued))
    }

    fn apply_queued(&mut self, queued: QueuedWindowCommand) -> AppliedWindowCommand {
        let QueuedWindowCommand {
            command,
            prepared_build_selection,
            prepared_build_selection_confirm,
            prepared_system,
            prepared_defenses,
            prepared_missions,
            prepared_fleet,
            prepared_fleet_finder,
            prepared_troop_finder,
            prepared_personnel_finder,
            prepared_message_index,
            prepared_status,
            prepared_sector,
        } = queued;
        let result = self.manager.apply(command);
        let mut removed_content = Vec::new();
        let mut build_selection_content_id = None;
        let mut confirmed_build_selection = None;
        let mut system_content_id = None;
        let mut defenses_content_id = None;
        let mut missions_content_id = None;
        let mut fleet_content_id = None;
        let mut fleet_finder_content_id = None;
        let mut troop_finder_content_id = None;
        let mut personnel_finder_content_id = None;
        let mut message_index_content_id = None;
        let mut status_content_id = None;
        let mut sector_content_id = None;
        if let Ok(commit) = &result {
            self.record_transition(&commit.transition, &commit.events);
            removed_content = self.release_matching_content(&commit.events);
            if prepared_build_selection.is_some() {
                build_selection_content_id = match &commit.transition {
                    WindowTransition::Opened { id } => Some(*id),
                    WindowTransition::Existing { id, .. } => {
                        self.attached_content.contains(id).then_some(*id)
                    }
                    _ => None,
                };
            }
            if let Some(prepared) = prepared_build_selection_confirm {
                let window = prepared.window_id();
                if matches!(
                    &commit.transition,
                    WindowTransition::Closed { root, closed, .. }
                        if *root == window && closed.contains(&window)
                ) && removed_content.contains(&window)
                {
                    confirmed_build_selection = Some(prepared);
                }
            }
            if prepared_system.is_some() {
                system_content_id = match &commit.transition {
                    WindowTransition::Opened { id } => Some(*id),
                    WindowTransition::Existing { id, .. } => {
                        self.attached_content.contains(id).then_some(*id)
                    }
                    _ => None,
                };
            }
            if prepared_defenses.is_some() {
                defenses_content_id = match &commit.transition {
                    WindowTransition::Opened { id } => Some(*id),
                    WindowTransition::Existing { id, .. } => {
                        self.attached_content.contains(id).then_some(*id)
                    }
                    _ => None,
                };
            }
            if prepared_missions.is_some() {
                missions_content_id = match &commit.transition {
                    WindowTransition::Opened { id } => Some(*id),
                    WindowTransition::Existing { id, .. } => {
                        self.attached_content.contains(id).then_some(*id)
                    }
                    _ => None,
                };
            }
            if prepared_fleet.is_some() {
                fleet_content_id = match &commit.transition {
                    WindowTransition::Opened { id } => Some(*id),
                    WindowTransition::Existing { id, .. } => {
                        self.attached_content.contains(id).then_some(*id)
                    }
                    _ => None,
                };
            }
            if prepared_fleet_finder.is_some() {
                fleet_finder_content_id = match &commit.transition {
                    WindowTransition::Opened { id } => Some(*id),
                    WindowTransition::Existing { id, .. } => {
                        self.attached_content.contains(id).then_some(*id)
                    }
                    _ => None,
                };
                if let WindowTransition::Opened { id } = commit.transition {
                    let focus = FocusTarget {
                        window: id,
                        control: Some(FLEET_FINDER_NAME_CONTROL),
                    };
                    if let Ok(focus_commit) = self.manager.apply(WindowCommand::Focus(focus)) {
                        self.record_transition(&focus_commit.transition, &focus_commit.events);
                    }
                }
            }
            if prepared_troop_finder.is_some() {
                troop_finder_content_id = match &commit.transition {
                    WindowTransition::Opened { id } => Some(*id),
                    WindowTransition::Existing {
                        id,
                        effect: ExistingEffect::NoOp,
                    } => self.attached_content.contains(id).then_some(*id),
                    _ => None,
                };
                if let WindowTransition::Opened { id } = commit.transition {
                    let focus = FocusTarget {
                        window: id,
                        control: Some(TROOP_FINDER_NAME_CONTROL),
                    };
                    if let Ok(focus_commit) = self.manager.apply(WindowCommand::Focus(focus)) {
                        self.record_transition(&focus_commit.transition, &focus_commit.events);
                    }
                }
            }
            if prepared_personnel_finder.is_some() {
                personnel_finder_content_id = match &commit.transition {
                    WindowTransition::Opened { id } => Some(*id),
                    WindowTransition::Existing {
                        id,
                        effect: ExistingEffect::NoOp,
                    } => self.attached_content.contains(id).then_some(*id),
                    _ => None,
                };
                if let WindowTransition::Opened { id } = commit.transition {
                    let focus = FocusTarget {
                        window: id,
                        control: Some(PERSONNEL_FINDER_NAME_CONTROL),
                    };
                    if let Ok(focus_commit) = self.manager.apply(WindowCommand::Focus(focus)) {
                        self.record_transition(&focus_commit.transition, &focus_commit.events);
                    }
                }
            }
            if prepared_message_index.is_some() {
                message_index_content_id = match &commit.transition {
                    WindowTransition::Opened { id } => Some(*id),
                    WindowTransition::Existing { id, .. } => {
                        self.attached_content.contains(id).then_some(*id)
                    }
                    _ => None,
                };
            }
            if prepared_status.is_some() {
                status_content_id = match &commit.transition {
                    WindowTransition::Opened { id } => Some(*id),
                    WindowTransition::Replaced { new, .. } => Some(*new),
                    WindowTransition::Existing { id, .. } => {
                        self.attached_content.contains(id).then_some(*id)
                    }
                    _ => None,
                };
            }
            if prepared_sector.is_some() {
                sector_content_id = match &commit.transition {
                    WindowTransition::Opened { id }
                    | WindowTransition::Replaced { new: id, .. } => Some(*id),
                    WindowTransition::Existing {
                        id,
                        effect: ExistingEffect::NoOp,
                    } => self.attached_content.contains(id).then_some(*id),
                    _ => None,
                };
            }
        }
        AppliedWindowCommand {
            command,
            result,
            removed_content,
            prepared_build_selection,
            build_selection_content_id,
            confirmed_build_selection,
            prepared_system,
            system_content_id,
            prepared_defenses,
            defenses_content_id,
            prepared_missions,
            missions_content_id,
            prepared_fleet,
            fleet_content_id,
            prepared_fleet_finder,
            fleet_finder_content_id,
            prepared_troop_finder,
            troop_finder_content_id,
            prepared_personnel_finder,
            personnel_finder_content_id,
            prepared_message_index,
            message_index_content_id,
            prepared_status,
            status_content_id,
            prepared_sector,
            sector_content_id,
        }
    }

    /// Attaches successfully prepared feature content to a committed identity.
    #[allow(
        dead_code,
        reason = "feature-family migrations attach the first content"
    )]
    pub fn attach_content(&mut self, id: WindowId) -> bool {
        if self.manager.begin_frame().key(id).is_none() {
            return false;
        }
        self.attached_content.insert(id)
    }

    /// Acknowledges renderer publication only after the exact prepared Fleet
    /// content has accepted the manager identity.
    pub fn acknowledge_fleet_publication(&mut self, id: WindowId) -> bool {
        self.manager
            .begin_frame()
            .key(id)
            .is_some_and(|key| key.kind == rebellion_windowing::WindowKind::Fleet)
            && self.attached_content.insert(id)
    }

    /// Acknowledges Build Selection publication only for its exact singleton.
    pub fn acknowledge_build_selection_publication(&mut self, id: WindowId) -> bool {
        self.manager
            .begin_frame()
            .key(id)
            .is_some_and(|key| key.kind == rebellion_windowing::WindowKind::BuildSelection)
            && self.attached_content.insert(id)
    }

    /// Acknowledges System publication only for the exact manager identity.
    pub fn acknowledge_system_publication(&mut self, id: WindowId) -> bool {
        self.manager
            .begin_frame()
            .key(id)
            .is_some_and(|key| key.kind == rebellion_windowing::WindowKind::System)
            && self.attached_content.insert(id)
    }

    /// Acknowledges Defenses publication only for the exact manager identity.
    pub fn acknowledge_defenses_publication(&mut self, id: WindowId) -> bool {
        self.manager
            .begin_frame()
            .key(id)
            .is_some_and(|key| key.kind == rebellion_windowing::WindowKind::Defenses)
            && self.attached_content.insert(id)
    }

    /// Acknowledges Missions publication only for the exact manager identity.
    pub fn acknowledge_missions_publication(&mut self, id: WindowId) -> bool {
        self.manager
            .begin_frame()
            .key(id)
            .is_some_and(|key| key.kind == rebellion_windowing::WindowKind::Missions)
            && self.attached_content.insert(id)
    }

    /// Acknowledges Fleet Finder publication only for its exact singleton.
    pub fn acknowledge_fleet_finder_publication(&mut self, id: WindowId) -> bool {
        self.manager
            .begin_frame()
            .key(id)
            .is_some_and(|key| key.kind == rebellion_windowing::WindowKind::FleetFinder)
            && self.attached_content.insert(id)
    }

    /// Acknowledges Troop Finder publication only for its exact singleton.
    pub fn acknowledge_troop_finder_publication(&mut self, id: WindowId) -> bool {
        self.manager
            .begin_frame()
            .key(id)
            .is_some_and(|key| key.kind == rebellion_windowing::WindowKind::TroopFinder)
            && self.attached_content.insert(id)
    }

    /// Acknowledges Personnel Finder publication only for its exact singleton.
    pub fn acknowledge_personnel_finder_publication(&mut self, id: WindowId) -> bool {
        self.manager
            .begin_frame()
            .key(id)
            .is_some_and(|key| key.kind == rebellion_windowing::WindowKind::PersonnelFinder)
            && self.attached_content.insert(id)
    }

    /// Acknowledges Message Index publication only for its exact singleton.
    pub fn acknowledge_message_index_publication(&mut self, id: WindowId) -> bool {
        self.manager
            .begin_frame()
            .key(id)
            .is_some_and(|key| key.kind == rebellion_windowing::WindowKind::MessageIndex)
            && self.attached_content.insert(id)
    }

    /// Acknowledges Status publication only for the exact contextual identity.
    pub fn acknowledge_status_publication(&mut self, id: WindowId) -> bool {
        self.manager
            .begin_frame()
            .key(id)
            .is_some_and(|key| key.kind == rebellion_windowing::WindowKind::Status)
            && self.attached_content.insert(id)
    }

    /// Acknowledges Sector publication only for the exact manager identity.
    pub fn acknowledge_sector_publication(&mut self, id: WindowId) -> bool {
        self.manager
            .begin_frame()
            .key(id)
            .is_some_and(|key| key.kind == rebellion_windowing::WindowKind::Sector)
            && self.attached_content.insert(id)
    }

    /// Compensates a failed renderer publication or rail eviction by closing
    /// the exact manager identity and returning the content teardown target.
    pub fn rollback_fleet_publication(&mut self, id: WindowId) -> Vec<WindowId> {
        self.rollback_publication(id)
    }

    /// Compensates a failed Build Selection publication with an exact close.
    pub fn rollback_build_selection_publication(&mut self, id: WindowId) -> Vec<WindowId> {
        self.rollback_publication(id)
    }

    /// Compensates a failed detail publication or temporary-rail eviction by
    /// closing the exact manager identity.
    pub fn rollback_detail_publication(&mut self, id: WindowId) -> Vec<WindowId> {
        self.rollback_publication(id)
    }

    /// Compensates a failed Fleet Finder publication with an exact close.
    pub fn rollback_fleet_finder_publication(&mut self, id: WindowId) -> Vec<WindowId> {
        self.rollback_publication(id)
    }

    /// Compensates a failed Troop Finder publication with an exact close.
    pub fn rollback_troop_finder_publication(&mut self, id: WindowId) -> Vec<WindowId> {
        self.rollback_publication(id)
    }

    /// Compensates a failed Personnel Finder publication with an exact close.
    pub fn rollback_personnel_finder_publication(&mut self, id: WindowId) -> Vec<WindowId> {
        self.rollback_publication(id)
    }

    /// Compensates a failed Message Index publication with an exact close.
    pub fn rollback_message_index_publication(&mut self, id: WindowId) -> Vec<WindowId> {
        self.rollback_publication(id)
    }

    /// Compensates a failed Status publication with an exact close.
    pub fn rollback_status_publication(&mut self, id: WindowId) -> Vec<WindowId> {
        self.rollback_publication(id)
    }

    /// Compensates a failed Sector publication with an exact close.
    pub fn rollback_sector_publication(&mut self, id: WindowId) -> Vec<WindowId> {
        self.rollback_publication(id)
    }

    fn rollback_publication(&mut self, id: WindowId) -> Vec<WindowId> {
        let Ok(commit) = self.manager.apply(WindowCommand::Close(id)) else {
            return Vec::new();
        };
        self.record_transition(&commit.transition, &commit.events);
        let mut removed = self.release_matching_content(&commit.events);
        if !removed.contains(&id) {
            removed.push(id);
        }
        removed
    }

    /// Whether feature content remains attached to an identity.
    #[must_use]
    #[allow(dead_code, reason = "feature-family migrations query attached content")]
    pub fn has_content(&self, id: WindowId) -> bool {
        self.attached_content.contains(&id)
    }

    /// Allocates a nonpersistent command-context subject.
    #[allow(
        dead_code,
        reason = "contextual dialog migration consumes this allocator"
    )]
    pub fn next_context_subject(&mut self) -> Result<SubjectId, WindowError> {
        let value = self.next_context_subject.ok_or(WindowError::IdExhausted)?;
        self.next_context_subject = value.checked_add(1);
        Ok(SubjectId::new(SUBJECT_COMMAND_CONTEXT, value))
    }

    /// Reconciles explicit content state and releases only matching attachments.
    #[allow(dead_code, reason = "feature-family migrations submit content reports")]
    pub fn reconcile(
        &mut self,
        frame: &RoutingSnapshot,
        reports: &[ContentReport],
    ) -> ReconcileReport {
        let result = self.manager.reconcile(frame, reports);
        self.release_matching_content(&result.events);
        self.remove_from_scenes(&result.events);
        result
    }

    /// Atomically synchronizes Alert/Combat/Results registration for the
    /// authoritative tactical phase. One subject survives all phases of a
    /// battle and is retired only when the session becomes inactive.
    #[cfg(test)]
    fn commit_tactical_phase(
        &mut self,
        phase: TacticalWindowPhase,
    ) -> Result<CommittedTacticalPhase, WindowError> {
        let battle = (phase != TacticalWindowPhase::Inactive)
            .then_some(self.battle_session_token.unwrap_or(0));
        self.commit_tactical_phase_for_battle(battle, phase)
    }

    fn commit_tactical_phase_for_battle(
        &mut self,
        battle_session: Option<u64>,
        phase: TacticalWindowPhase,
    ) -> Result<CommittedTacticalPhase, WindowError> {
        assert_eq!(
            battle_session.is_some(),
            phase != TacticalWindowPhase::Inactive,
            "an active tactical phase must identify its battle session"
        );
        let desired_kind = match phase {
            TacticalWindowPhase::Alert => Some(WindowKind::BattleAlert),
            TacticalWindowPhase::Results => Some(WindowKind::BattleResults),
            TacticalWindowPhase::Combat | TacticalWindowPhase::Inactive => None,
        };
        let new_battle = battle_session.is_some() && battle_session != self.battle_session_token;
        let needs_subject =
            battle_session.is_some() && (new_battle || self.battle_subject.is_none());
        let allocated_subject = if needs_subject {
            let value = self.next_battle_subject.ok_or(WindowError::IdExhausted)?;
            Some((SubjectId::new(SUBJECT_BATTLE, value), value.checked_add(1)))
        } else {
            None
        };
        let subject = allocated_subject
            .map(|(subject, _)| subject)
            .or(self.battle_subject);

        if let Some(kind) = desired_kind {
            let subject = subject.expect("active tactical phases allocate a battle subject");
            if !new_battle {
                if let Some(window) = self.tactical_window.filter(|window| window.kind == kind) {
                    return Ok(CommittedTacticalPhase {
                        window: Some(window),
                        events: Vec::new(),
                        removed_content: Vec::new(),
                    });
                }
            }

            let spec = contextual_spec(kind, subject, WindowOwner::StrategicRoot);
            let command = self
                .tactical_window
                .map_or(WindowCommand::Open(spec), |old| WindowCommand::Replace {
                    old: old.id,
                    new: spec,
                });
            let commit = self.manager.apply(command)?;
            self.record_transition(&commit.transition, &commit.events);
            let removed_content = self.release_matching_content(&commit.events);
            let id = match commit.transition {
                WindowTransition::Opened { id } | WindowTransition::Replaced { new: id, .. } => id,
                _ => unreachable!("tactical phase sync only opens or replaces"),
            };
            if let Some((_, next)) = allocated_subject {
                self.next_battle_subject = next;
            }
            self.battle_session_token = battle_session;
            self.battle_subject = Some(subject);
            let window = TacticalWindowIdentity { id, subject, kind };
            self.attached_content.insert(id);
            self.tactical_window = Some(window);
            return Ok(CommittedTacticalPhase {
                window: Some(window),
                events: commit.events,
                removed_content,
            });
        }

        let mut events = Vec::new();
        let mut removed_content = Vec::new();
        if let Some(window) = self.tactical_window {
            let commit = self.manager.apply(WindowCommand::Close(window.id))?;
            self.record_transition(&commit.transition, &commit.events);
            removed_content = self.release_matching_content(&commit.events);
            events = commit.events;
            self.tactical_window = None;
        }
        if phase == TacticalWindowPhase::Inactive {
            self.battle_session_token = None;
            self.battle_subject = None;
        } else {
            let subject = subject.expect("combat belongs to an active battle subject");
            if let Some((_, next)) = allocated_subject {
                self.next_battle_subject = next;
            }
            self.battle_session_token = battle_session;
            self.battle_subject = Some(subject);
        }
        Ok(CommittedTacticalPhase {
            window: None,
            events,
            removed_content,
        })
    }

    /// Commits an explicitly identified battle session and scene before the
    /// next immutable routing snapshot. A changed session always gets a fresh
    /// subject, even when Results transitions directly to a new Alert.
    pub fn commit_scene_phase_for_battle(
        &mut self,
        scene: WindowScene,
        battle_session: Option<u64>,
        tactical_phase: TacticalWindowPhase,
    ) -> Result<CommittedTacticalPhase, WindowError> {
        let mut events = Vec::new();
        let mut removed_content = Vec::new();

        if scene == WindowScene::Galaxy {
            let retired =
                self.commit_tactical_phase_for_battle(None, TacticalWindowPhase::Inactive)?;
            events.extend(retired.events);
            removed_content.extend(retired.removed_content);
        }

        events.extend(self.enter_scene(scene));
        if scene == WindowScene::Tactical {
            let committed =
                self.commit_tactical_phase_for_battle(battle_session, tactical_phase)?;
            events.extend(committed.events);
            removed_content.extend(committed.removed_content);
            return Ok(CommittedTacticalPhase {
                window: committed.window,
                events,
                removed_content,
            });
        }

        Ok(CommittedTacticalPhase {
            window: self.tactical_window,
            events,
            removed_content,
        })
    }

    /// Advances a tactical modal only when the deferred action still names
    /// the exact live battle and window identity. A stale prior-frame action
    /// therefore cannot close a newer battle's modal.
    pub fn commit_tactical_transition_if_exact(
        &mut self,
        battle_session: u64,
        window: TacticalWindowIdentity,
        next_phase: TacticalWindowPhase,
    ) -> Result<Option<CommittedTacticalPhase>, WindowError> {
        let valid_transition = matches!(
            (window.kind, next_phase),
            (WindowKind::BattleAlert, TacticalWindowPhase::Combat)
                | (WindowKind::BattleAlert, TacticalWindowPhase::Results)
                | (WindowKind::BattleResults, TacticalWindowPhase::Inactive)
        );
        if !valid_transition
            || self.battle_session_token != Some(battle_session)
            || self.battle_subject != Some(window.subject)
            || self.tactical_window != Some(window)
        {
            return Ok(None);
        }
        let next_session = (next_phase != TacticalWindowPhase::Inactive).then_some(battle_session);
        self.commit_tactical_phase_for_battle(next_session, next_phase)
            .map(Some)
    }

    /// Commits scene membership and the authoritative tactical phase before
    /// the next immutable routing snapshot. This compatibility entry point
    /// retains the currently observed battle; production supplies its token
    /// through [`Self::commit_scene_phase_for_battle`].
    #[cfg(test)]
    pub fn commit_scene_phase(
        &mut self,
        scene: WindowScene,
        tactical_phase: TacticalWindowPhase,
    ) -> Result<CommittedTacticalPhase, WindowError> {
        let battle = (tactical_phase != TacticalWindowPhase::Inactive)
            .then_some(self.battle_session_token.unwrap_or(0));
        self.commit_scene_phase_for_battle(scene, battle, tactical_phase)
    }

    /// Reports the active tactical modal against the exact frozen snapshot.
    /// A phase mismatch stays registered as unpaintable so reconciliation
    /// cannot expose the command-center underlay permissively.
    #[must_use]
    pub fn tactical_content_reports(
        &self,
        frame: &RoutingSnapshot,
        phase: TacticalWindowPhase,
    ) -> Vec<ContentReport> {
        let Some(window) = self
            .tactical_window
            .filter(|window| frame.key(window.id).is_some())
        else {
            return Vec::new();
        };
        let ready = matches!(
            (window.kind, phase),
            (WindowKind::BattleAlert, TacticalWindowPhase::Alert)
                | (WindowKind::BattleResults, TacticalWindowPhase::Results)
        );
        vec![ContentReport {
            window: window.id,
            status: if ready {
                ContentStatus::Ready
            } else {
                ContentStatus::Unpaintable
            },
        }]
    }

    /// Switches active membership before the next frame snapshot is created.
    pub fn enter_scene(&mut self, scene: WindowScene) -> Vec<WindowEvent> {
        if self.scene == scene {
            return Vec::new();
        }
        self.scene = scene;
        let active = match scene {
            WindowScene::Galaxy => self.galaxy.clone(),
            WindowScene::Tactical => self.tactical.clone(),
            WindowScene::Suspended => BTreeSet::new(),
        };
        self.manager
            .set_active_windows(&active)
            .expect("scene sets contain only manager-owned identities")
    }

    /// Discards queued work, content, gestures, and registered session state.
    #[allow(
        dead_code,
        reason = "campaign reset migration calls this shared teardown"
    )]
    pub fn reset_session(&mut self) -> Vec<WindowEvent> {
        self.pending.clear();
        self.pending_galaxy.clear();
        self.applying.clear();
        self.galaxy.clear();
        self.tactical.clear();
        self.attached_content.clear();
        self.battle_session_token = None;
        self.battle_subject = None;
        self.tactical_window = None;
        self.scene = WindowScene::Galaxy;
        self.manager.reset_session()
    }

    fn record_transition(&mut self, transition: &WindowTransition, events: &[WindowEvent]) {
        match transition {
            WindowTransition::Opened { id } => self.current_scene_mut().insert(*id),
            WindowTransition::Replaced { new, .. } => self.current_scene_mut().insert(*new),
            WindowTransition::Existing { .. }
            | WindowTransition::Raised { .. }
            | WindowTransition::Minimized { .. }
            | WindowTransition::Restored { .. }
            | WindowTransition::Focused { .. }
            | WindowTransition::ControlInvalidated { .. }
            | WindowTransition::CaptureChanged { .. }
            | WindowTransition::Unchanged { .. }
            | WindowTransition::Closed { .. } => false,
        };
        self.remove_from_scenes(events);
    }

    fn current_scene_mut(&mut self) -> &mut BTreeSet<WindowId> {
        match self.scene {
            WindowScene::Galaxy => &mut self.galaxy,
            WindowScene::Tactical => &mut self.tactical,
            WindowScene::Suspended => {
                unreachable!("suspended scenes defer every queued window command")
            }
        }
    }

    fn remove_from_scenes(&mut self, events: &[WindowEvent]) {
        for event in events {
            if let WindowEvent::Closed { window, .. } = event {
                self.galaxy.remove(window);
                self.tactical.remove(window);
            }
        }
    }

    fn release_matching_content(&mut self, events: &[WindowEvent]) -> Vec<WindowId> {
        let mut removed = Vec::new();
        for event in events {
            if let WindowEvent::Closed { window, .. } = event {
                if self.attached_content.remove(window) {
                    removed.push(*window);
                }
            }
        }
        removed
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use egui_macroquad::egui;
    use rebellion_core::dat::{ExplorationStatus, Faction, SectorGroup};
    use rebellion_core::delivery::DeliveryState;
    use rebellion_core::economy::EconomyState;
    use rebellion_core::fog::FogState;
    use rebellion_core::ids::{DatId, SystemKey};
    use rebellion_core::manufacturing::{BuildableKind, ManufacturingState, ProductionArea};
    use rebellion_core::missions::{
        MissionFaction, MissionKind, MissionMember, MissionRequest, MissionState,
    };
    use rebellion_core::movement::MovementState;
    use rebellion_core::troop_transport::TroopTransportState;
    use rebellion_core::world::{
        BuildableClass, Character, ControlKind, Fleet, GameWorld, ManufacturingFacilityInstance,
        Sector, System,
    };
    use rebellion_render::defenses_window::{draw_managed_defenses_windows, DefensesWindowReport};
    use rebellion_render::fleet_window::FleetWindowEntry;
    use rebellion_render::fleet_window::FleetWindowReport;
    use rebellion_render::missions_window::MissionsWindowReport;
    use rebellion_render::system_window::{draw_managed_system_windows, SystemWindowInputEvent};
    use rebellion_render::windowing_input::{
        FrameInputRouter, InputDisposition, InputEventId, InputRecipient, PointerButtonId,
        PointerPhase,
    };
    use rebellion_render::windowing_policy::{contextual_spec, singleton_spec, system_subject};
    use rebellion_render::{
        fleet_finder::{FleetFinderState, FLEET_FINDER_NAME_CONTROL},
        message_index::{MessageIndexAction, MessageIndexState},
        personnel_finder::{
            PersonnelFinderState, PersonnelTab, PersonnelView, PERSONNEL_FINDER_NAME_CONTROL,
        },
        status_rows::{StatusObject, StatusSources},
        status_window::StatusWindowState,
        troop_finder::{TroopFinderState, TroopTab, TROOP_FINDER_NAME_CONTROL},
        BmpCache, CockpitFaction, CockpitLayout, CockpitViewport, DefensesWindowState,
        FleetWindowState, MissionsWindowState, SectorWindowState, SystemWindowState,
        SystemWindowTab,
    };
    use rebellion_windowing::{
        ContentStatus, ControlId, FocusTarget, SubjectId, WindowKey, WindowKind, WindowOwner,
        WindowSpec,
    };

    fn opened_id(applied: &AppliedWindowCommand) -> WindowId {
        match &applied.result {
            Ok(WindowCommit {
                transition: WindowTransition::Opened { id },
                ..
            }) => *id,
            other => panic!("expected successful open, got {other:?}"),
        }
    }

    fn fleet_world() -> (GameWorld, SystemKey) {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(36),
            name: "Sesswenna".into(),
            group: SectorGroup::Core,
            x: 317,
            y: 248,
            systems: Vec::new(),
        });
        let system = world.systems.insert(System {
            dat_id: DatId::new(100),
            name: "Sluis Van".into(),
            sector,
            x: 320,
            y: 250,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: Vec::new(),
            ground_units: Vec::new(),
            special_forces: Vec::new(),
            fighters: Vec::new(),
            defense_facilities: Vec::new(),
            manufacturing_facilities: Vec::new(),
            production_facilities: Vec::new(),
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Uncontrolled,
        });
        (world, system)
    }

    #[test]
    fn sector_window_coordinator_commits_atomic_replace_and_duplicate_noop() {
        fn add_sector(world: &mut GameWorld, template: SystemKey, dat: u32, x: u16) -> SystemKey {
            let sector = world.sectors.insert(Sector {
                dat_id: DatId::new(dat),
                name: format!("Sector {dat}"),
                group: SectorGroup::Core,
                x,
                y: 250,
                systems: Vec::new(),
            });
            let system = world.systems.insert(System {
                dat_id: DatId::new(dat + 100),
                name: format!("System {dat}"),
                sector,
                x,
                ..world.systems[template].clone()
            });
            world.sectors[sector].systems.push(system);
            system
        }

        let (mut world, left) = fleet_world();
        let right = add_sector(&mut world, left, 37, 800);
        let far_left = add_sector(&mut world, left, 38, 100);
        let mut state = SectorWindowState::default();
        let mut coordinator = StrategicWindowing::new();

        let publish =
            |system, state: &mut SectorWindowState, coordinator: &mut StrategicWindowing| {
                coordinator.enqueue_sector_open(
                    state
                        .prepare_open(&world, system, CockpitFaction::Alliance)
                        .unwrap(),
                );
                let mut applied = coordinator.apply_pending().remove(0);
                let prepared = applied.prepared_sector.take().unwrap();
                let id = applied.sector_content_id.unwrap();
                let commit = applied.result.as_ref().unwrap();
                assert_eq!(
                    state.commit_open(prepared, &commit.transition, &coordinator.begin_frame()),
                    Some(id)
                );
                assert!(coordinator.acknowledge_sector_publication(id));
                (id, applied)
            };
        let (left_id, _) = publish(left, &mut state, &mut coordinator);
        let _ = publish(right, &mut state, &mut coordinator);

        let prepared = state
            .prepare_open(&world, far_left, CockpitFaction::Alliance)
            .unwrap();
        assert_eq!(prepared.replacing(), Some(left_id));
        coordinator.enqueue_sector_open(prepared);
        let mut replaced = coordinator.apply_pending().remove(0);
        assert!(matches!(
            replaced.command,
            WindowCommand::Replace { old, .. } if old == left_id
        ));
        assert_eq!(replaced.removed_content, [left_id]);
        assert_eq!(state.window_count(), 2, "publication is still staged");
        let replacement = replaced.sector_content_id.unwrap();
        assert_eq!(
            state.commit_open(
                replaced.prepared_sector.take().unwrap(),
                &replaced.result.as_ref().unwrap().transition,
                &coordinator.begin_frame(),
            ),
            Some(replacement)
        );
        state.remove_content(&replaced.removed_content);
        assert!(coordinator.acknowledge_sector_publication(replacement));
        assert_eq!(state.window_count(), 2);

        coordinator.enqueue_sector_open(
            state
                .prepare_open(&world, far_left, CockpitFaction::Alliance)
                .unwrap(),
        );
        let mut duplicate = coordinator.apply_pending().remove(0);
        assert!(matches!(
            duplicate.result.as_ref().unwrap().transition,
            WindowTransition::Existing {
                id,
                effect: ExistingEffect::NoOp,
            } if id == replacement
        ));
        assert!(duplicate.removed_content.is_empty());
        assert_eq!(duplicate.sector_content_id, Some(replacement));
        assert_eq!(
            state.commit_open(
                duplicate.prepared_sector.take().unwrap(),
                &duplicate.result.as_ref().unwrap().transition,
                &coordinator.begin_frame(),
            ),
            Some(replacement)
        );
        assert_eq!(state.window_count(), 2);
    }

    #[test]
    fn build_selection_windowing_modal_cancels_capture() {
        use rebellion_render::build_selection::BuildSelectionState;
        use rebellion_windowing::CaptureCancelReason;

        let mut coordinator = StrategicWindowing::new();
        coordinator.enqueue(WindowCommand::Open(contextual_spec(
            WindowKind::Fleet,
            SubjectId::new(1, 1),
            WindowOwner::StrategicRoot,
        )));
        let owner = opened_id(&coordinator.apply_pending()[0]);
        coordinator.enqueue(WindowCommand::CapturePointer(owner));
        assert!(coordinator.apply_pending()[0].result.is_ok());

        let state = BuildSelectionState::default();
        coordinator.enqueue_build_selection_open(
            state
                .prepare_open(
                    SystemKey::default(),
                    ProductionArea::TrainingFacility,
                    true,
                    vec![BuildableKind::Troop(DatId::new(0x1000_0001))],
                )
                .unwrap(),
        );
        let applied = coordinator.apply_pending();
        let commit = applied[0].result.as_ref().unwrap();
        assert!(commit.events.contains(&WindowEvent::CaptureCancelled {
            window: owner,
            reason: CaptureCancelReason::ModalOpened,
        }));
    }

    #[test]
    fn build_selection_windowing_confirm_is_once() {
        use rebellion_render::build_selection::{BuildSelectionAction, BuildSelectionState};

        let (mut world, system) = fleet_world();
        let yard_class = DatId::new(0x2900_0002);
        let product_class = DatId::new(0x1000_0001);
        world.buildable_classes.insert(
            yard_class,
            BuildableClass {
                refined_material_cost: 10,
                processing_rate: 4,
                is_alliance: true,
                is_empire: false,
                ..BuildableClass::default()
            },
        );
        world.buildable_classes.insert(
            product_class,
            BuildableClass {
                refined_material_cost: 8,
                is_alliance: true,
                is_empire: false,
                ..BuildableClass::default()
            },
        );
        let yard = world
            .manufacturing_facilities
            .insert(ManufacturingFacilityInstance {
                class_dat_id: yard_class,
                side: Faction::Alliance,
                is_shipyard: false,
            });
        world.systems[system].manufacturing_facilities.push(yard);

        let mut coordinator = StrategicWindowing::new();
        let mut state = BuildSelectionState::default();
        coordinator.enqueue_build_selection_open(
            state
                .prepare_open(
                    system,
                    ProductionArea::TrainingFacility,
                    true,
                    vec![BuildableKind::Troop(product_class)],
                )
                .unwrap(),
        );
        let mut opened = coordinator.apply_pending().remove(0);
        let id = opened.build_selection_content_id.unwrap();
        assert_eq!(
            state.commit_open(
                opened.prepared_build_selection.take().unwrap(),
                &opened.result.as_ref().unwrap().transition,
            ),
            Some(id)
        );
        assert!(coordinator.acknowledge_build_selection_publication(id));

        let prepared = state.prepare_confirm(&world).unwrap();
        coordinator.enqueue_build_selection_confirm(prepared);
        coordinator.enqueue_build_selection_confirm(prepared);
        let mut applied = coordinator.apply_pending();
        let first = applied.remove(0);
        let second = applied.remove(0);
        assert_eq!(first.confirmed_build_selection, Some(prepared));
        assert!(second.result.is_err());
        assert_eq!(second.confirmed_build_selection, None);
        assert_eq!(
            state
                .commit_confirm(prepared, &first.removed_content, &world)
                .map(|committed| committed.action()),
            Some(BuildSelectionAction::Confirm {
                system,
                area: ProductionArea::TrainingFacility,
                kind: BuildableKind::Troop(product_class),
                count: 1,
            })
        );
        assert_eq!(
            state.commit_confirm(prepared, &first.removed_content, &world),
            None
        );
    }

    fn fleet_layout() -> CockpitLayout {
        let viewport = CockpitViewport {
            x: 0.0,
            y: 0.0,
            width: 640.0,
            height: 480.0,
        };
        CockpitLayout {
            canvas: viewport,
            galaxy: viewport,
            scale: 1.0,
        }
    }

    // Source authority shared by every cross-family test below: the recovered
    // Window-family matrix in ghidra/notes/strategic-windowing-toolkit.md and
    // System FUN_0045aac0/FUN_00452fc0, Fleet FUN_004a2630, Defenses
    // FUN_004a7790, and Missions FUN_0049f130.
    struct CrossFamilyDetailFixture {
        world: GameWorld,
        system: SystemKey,
        fog: FogState,
        missions: MissionState,
        system_windows: SystemWindowState,
        fleet_windows: FleetWindowState,
        defenses_windows: DefensesWindowState,
        missions_windows: MissionsWindowState,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct CrossFamilyContentObservation {
        system: (SystemWindowTab, Option<ProductionArea>),
        fleet: FleetWindowReport,
        defenses: DefensesWindowReport,
        missions: MissionsWindowReport,
        visible_defenses_rows: Vec<String>,
    }

    impl CrossFamilyDetailFixture {
        fn new() -> Self {
            let (mut world, system) = fleet_world();
            world.systems[system].control = ControlKind::Controlled(Faction::Alliance);
            let mut fog = FogState::new(Faction::Alliance);
            fog.reveal(system);
            Self {
                world,
                system,
                fog,
                missions: MissionState::new(),
                system_windows: SystemWindowState::default(),
                fleet_windows: FleetWindowState::default(),
                defenses_windows: DefensesWindowState::default(),
                missions_windows: MissionsWindowState::default(),
            }
        }

        fn specs(&self) -> [WindowSpec; 4] {
            [
                self.system_windows
                    .prepare_open(
                        &self.world,
                        self.system,
                        (40, 50),
                        CockpitFaction::Alliance,
                        fleet_layout(),
                    )
                    .unwrap()
                    .spec(),
                self.fleet_windows
                    .prepare_open(
                        &self.world,
                        self.system,
                        (40, 50),
                        CockpitFaction::Alliance,
                        fleet_layout(),
                        None,
                    )
                    .unwrap()
                    .spec(),
                self.defenses_windows
                    .prepare_open(
                        &self.world,
                        self.system,
                        (40, 50),
                        CockpitFaction::Alliance,
                        fleet_layout(),
                    )
                    .unwrap()
                    .spec(),
                self.missions_windows
                    .prepare_open(
                        &self.world,
                        &self.fog,
                        &self.missions,
                        self.system,
                        (40, 50),
                        CockpitFaction::Alliance,
                        fleet_layout(),
                    )
                    .unwrap()
                    .spec(),
            ]
        }

        fn add_fleet(&mut self) -> rebellion_core::ids::FleetKey {
            let fleet = self.world.insert_fleet(Fleet {
                location: self.system,
                is_alliance: true,
                ..Fleet::default()
            });
            self.world.systems[self.system].fleets.push(fleet);
            fleet
        }

        fn add_character(&mut self, name: &str, on_mission: bool) -> MissionMember {
            MissionMember::Character(self.world.characters.insert(Character {
                dat_id: DatId::new(832 + self.world.characters.len() as u32),
                name: name.into(),
                is_alliance: true,
                current_system: Some(self.system),
                on_mission,
                recruited: true,
                ..Character::default()
            }))
        }

        fn add_mission_with_decoy(&mut self) -> MissionMember {
            let agent = self.add_character("Mission Agent", true);
            let decoy = self.add_character("Mission Decoy", true);
            self.missions.dispatch(MissionRequest {
                kind: MissionKind::Diplomacy,
                faction: MissionFaction::Alliance,
                team: vec![agent],
                decoys: vec![decoy],
                target_system: self.system,
                target_character: None,
                target_object: None,
                tick: 0,
            });
            decoy
        }

        fn publish_all(
            &mut self,
            coordinator: &mut StrategicWindowing,
            fleet_selection: Option<FleetWindowEntry>,
            new_content: bool,
        ) -> [WindowId; 4] {
            coordinator.enqueue_system_open(
                self.system_windows
                    .prepare_open(
                        &self.world,
                        self.system,
                        (40, 50),
                        CockpitFaction::Alliance,
                        fleet_layout(),
                    )
                    .unwrap(),
            );
            coordinator.enqueue_fleet_open(
                self.fleet_windows
                    .prepare_open(
                        &self.world,
                        self.system,
                        (40, 50),
                        CockpitFaction::Alliance,
                        fleet_layout(),
                        fleet_selection,
                    )
                    .unwrap(),
            );
            coordinator.enqueue_defenses_open(
                self.defenses_windows
                    .prepare_open(
                        &self.world,
                        self.system,
                        (40, 50),
                        CockpitFaction::Alliance,
                        fleet_layout(),
                    )
                    .unwrap(),
            );
            coordinator.enqueue_missions_open(
                self.missions_windows
                    .prepare_open(
                        &self.world,
                        &self.fog,
                        &self.missions,
                        self.system,
                        (40, 50),
                        CockpitFaction::Alliance,
                        fleet_layout(),
                    )
                    .unwrap(),
            );

            let mut applied = coordinator.apply_pending().into_iter();
            let mut system = applied.next().unwrap();
            let system_id = system.system_content_id.unwrap();
            assert_eq!(
                self.system_windows.commit_open(
                    system.prepared_system.take().unwrap(),
                    &system.result.as_ref().unwrap().transition,
                ),
                Some(system_id)
            );
            let mut fleet = applied.next().unwrap();
            let fleet_id = fleet.fleet_content_id.unwrap();
            assert_eq!(
                self.fleet_windows.commit_open(
                    fleet.prepared_fleet.take().unwrap(),
                    &fleet.result.as_ref().unwrap().transition,
                ),
                Some(fleet_id)
            );
            let mut defenses = applied.next().unwrap();
            let defenses_id = defenses.defenses_content_id.unwrap();
            assert_eq!(
                self.defenses_windows.commit_open(
                    defenses.prepared_defenses.take().unwrap(),
                    &defenses.result.as_ref().unwrap().transition,
                ),
                Some(defenses_id)
            );
            let mut missions = applied.next().unwrap();
            let missions_id = missions.missions_content_id.unwrap();
            assert_eq!(
                self.missions_windows.commit_open(
                    missions.prepared_missions.take().unwrap(),
                    &missions.result.as_ref().unwrap().transition,
                ),
                Some(missions_id)
            );
            assert!(applied.next().is_none());

            if new_content {
                assert!(coordinator.acknowledge_system_publication(system_id));
                assert!(coordinator.acknowledge_fleet_publication(fleet_id));
                assert!(coordinator.acknowledge_defenses_publication(defenses_id));
                assert!(coordinator.acknowledge_missions_publication(missions_id));
            } else {
                for id in [system_id, fleet_id, defenses_id, missions_id] {
                    assert!(coordinator.has_content(id));
                }
            }

            [system_id, fleet_id, defenses_id, missions_id]
        }

        fn draw_defenses_frames(
            &mut self,
            coordinator: &StrategicWindowing,
            frames: Vec<(Vec<egui::Event>, Vec<PointerPhase>)>,
        ) -> Vec<String> {
            let ctx = egui::Context::default();
            let mut cache = BmpCache::new();
            let economy = EconomyState::default();
            let window = self.defenses_windows.window_id(self.system).unwrap();
            let mut router = FrameInputRouter::default();
            let mut visible_rows = Vec::new();
            for (index, (events, phases)) in frames.into_iter().enumerate() {
                let frame = u64::try_from(index + 1).unwrap();
                let snapshot = coordinator.begin_frame();
                router.begin_frame(frame, &snapshot);
                let mut routed = Vec::new();
                for (ordinal, phase) in phases.into_iter().enumerate() {
                    let event = InputEventId {
                        frame,
                        ordinal: u32::try_from(ordinal).unwrap(),
                    };
                    assert_eq!(
                        router.collect_pointer(event, &BTreeSet::from([window]), phase),
                        InputRecipient::Window(window)
                    );
                    routed.push((event, phase));
                }
                let input = egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(640.0, 480.0),
                    )),
                    time: Some(index as f64 * 0.05),
                    events,
                    ..egui::RawInput::default()
                };
                let output = ctx.run(input, |ctx| {
                    let _ = draw_managed_defenses_windows(
                        ctx,
                        &self.world,
                        &self.fog,
                        &self.missions,
                        &economy,
                        &mut self.defenses_windows,
                        CockpitFaction::Alliance,
                        fleet_layout(),
                        &mut cache,
                        &snapshot,
                        &routed,
                        &mut router,
                        true,
                    );
                });
                visible_rows = output
                    .shapes
                    .into_iter()
                    .filter_map(|shape| match shape.shape {
                        egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
                        _ => None,
                    })
                    .filter(|text| text.starts_with("Agent "))
                    .collect();
            }
            visible_rows
        }

        fn select_system_tab(&mut self, coordinator: &StrategicWindowing, tab: SystemWindowTab) {
            let point = self
                .system_windows
                .tab_screen_rect(fleet_layout(), self.system, tab)
                .unwrap()
                .center();
            let button = |pressed| egui::Event::PointerButton {
                pos: point,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::default(),
            };
            let frames = [
                (vec![egui::Event::PointerMoved(point)], vec![]),
                (vec![egui::Event::PointerMoved(point)], vec![]),
                (
                    vec![button(true)],
                    vec![PointerPhase::Press(PointerButtonId::Primary)],
                ),
                (
                    vec![button(false)],
                    vec![PointerPhase::Release(PointerButtonId::Primary)],
                ),
            ];
            let ctx = egui::Context::default();
            let movement = MovementState::default();
            let manufacturing = ManufacturingState::new();
            let deliveries = DeliveryState::new();
            let mut cache = BmpCache::new();
            let window = self.system_windows.window_id(self.system).unwrap();
            let mut router = FrameInputRouter::default();
            for (index, (events, phases)) in frames.into_iter().enumerate() {
                let frame = u64::try_from(index + 1).unwrap();
                let snapshot = coordinator.begin_frame();
                router.begin_frame(frame, &snapshot);
                let mut routed = Vec::new();
                for (ordinal, phase) in phases.into_iter().enumerate() {
                    let event = InputEventId {
                        frame,
                        ordinal: u32::try_from(ordinal).unwrap(),
                    };
                    assert_eq!(
                        router.collect_pointer(event, &BTreeSet::from([window]), phase),
                        InputRecipient::Window(window)
                    );
                    routed.push(SystemWindowInputEvent {
                        id: event,
                        point: (point.x, point.y),
                        phase,
                    });
                }
                let input = egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(640.0, 480.0),
                    )),
                    time: Some(index as f64 * 0.05),
                    events,
                    ..egui::RawInput::default()
                };
                let _ = ctx.run(input, |ctx| {
                    let _ = draw_managed_system_windows(
                        ctx,
                        &self.world,
                        &movement,
                        &self.fog,
                        &self.missions,
                        &manufacturing,
                        &deliveries,
                        &mut self.system_windows,
                        CockpitFaction::Alliance,
                        fleet_layout(),
                        &mut cache,
                        &snapshot,
                        &routed,
                        &mut router,
                        true,
                    );
                });
            }
        }

        fn scroll_and_select_defenses(&mut self, coordinator: &StrategicWindowing) -> Vec<String> {
            let rect = self
                .defenses_windows
                .screen_rect(fleet_layout(), self.system)
                .unwrap();
            let thumb = rect.min + egui::vec2(220.0, 100.0);
            let dragged = rect.min + egui::vec2(220.0, 200.0);
            let first_visible_cell = rect.min + egui::vec2(35.0, 115.0);
            let button = |point, button, pressed| egui::Event::PointerButton {
                pos: point,
                button,
                pressed,
                modifiers: egui::Modifiers::default(),
            };
            self.draw_defenses_frames(
                coordinator,
                vec![
                    (vec![egui::Event::PointerMoved(thumb)], vec![]),
                    (vec![egui::Event::PointerMoved(thumb)], vec![]),
                    (
                        vec![button(thumb, egui::PointerButton::Primary, true)],
                        vec![PointerPhase::Press(PointerButtonId::Primary)],
                    ),
                    (
                        vec![egui::Event::PointerMoved(dragged)],
                        vec![PointerPhase::Motion(PointerButtonId::Primary)],
                    ),
                    (
                        vec![button(dragged, egui::PointerButton::Primary, false)],
                        vec![PointerPhase::Release(PointerButtonId::Primary)],
                    ),
                    (vec![egui::Event::PointerMoved(first_visible_cell)], vec![]),
                    (
                        vec![button(
                            first_visible_cell,
                            egui::PointerButton::Secondary,
                            true,
                        )],
                        vec![PointerPhase::Press(PointerButtonId::Secondary)],
                    ),
                ],
            )
        }

        fn content_observation(
            &mut self,
            coordinator: &StrategicWindowing,
        ) -> CrossFamilyContentObservation {
            CrossFamilyContentObservation {
                system: self.system_windows.selection(self.system).unwrap(),
                fleet: self
                    .fleet_windows
                    .report(
                        &self.world,
                        &MovementState::default(),
                        &self.fog,
                        &TroopTransportState::default(),
                        self.system,
                    )
                    .unwrap(),
                defenses: self
                    .defenses_windows
                    .report(
                        &self.world,
                        &self.fog,
                        &self.missions,
                        &EconomyState::default(),
                        self.system,
                    )
                    .unwrap(),
                missions: self
                    .missions_windows
                    .report(&self.world, &self.fog, &self.missions, self.system)
                    .unwrap(),
                visible_defenses_rows: self
                    .draw_defenses_frames(coordinator, vec![(vec![], vec![])]),
            }
        }
    }

    #[test]
    fn detail_windows_use_system_subject_and_distinct_kind() {
        let mut fixture = CrossFamilyDetailFixture::new();
        let expected_subject = system_subject(&fixture.world, fixture.system).unwrap();
        let specs = fixture.specs();
        assert_eq!(
            specs.map(|spec| spec.key.kind),
            [
                WindowKind::System,
                WindowKind::Fleet,
                WindowKind::Defenses,
                WindowKind::Missions,
            ]
        );
        assert!(specs
            .iter()
            .all(|spec| spec.key.subject == Some(expected_subject)));
        assert!(specs
            .iter()
            .all(|spec| spec.owner == WindowOwner::StrategicRoot));

        let mut coordinator = StrategicWindowing::new();
        let ids = fixture.publish_all(&mut coordinator, None, true);
        let snapshot = coordinator.begin_frame();
        assert_eq!(snapshot.paint_order(), ids);
        assert_eq!(
            snapshot.pointer_route(&ids.into_iter().collect()).target,
            RouteTarget::Window(FocusTarget {
                window: ids[3],
                control: None,
            })
        );
    }

    #[test]
    fn detail_windows_cross_family_activation_changes_one_stack() {
        let mut fixture = CrossFamilyDetailFixture::new();
        let mut coordinator = StrategicWindowing::new();
        let ids = fixture.publish_all(&mut coordinator, None, true);
        let frozen = coordinator.begin_frame();
        let event = InputEventId {
            frame: 1,
            ordinal: 0,
        };
        let mut router = FrameInputRouter::default();
        router.begin_frame(1, &frozen);
        let overlap = BTreeSet::from([ids[1], ids[2]]);
        let recipient = router.collect_pointer(
            event,
            &overlap,
            PointerPhase::Press(PointerButtonId::Primary),
        );
        assert_eq!(recipient, InputRecipient::Window(ids[2]));
        assert!(router.claim(event, recipient));
        assert!(!router.claim(event, recipient));
        assert!(!router.claim(event, InputRecipient::Window(ids[1])));
        assert!(!router.claim_root(event));

        let InputRecipient::Window(activated) = recipient else {
            panic!("overlap event must route to exactly one detail window");
        };
        coordinator.enqueue(WindowCommand::Activate(activated));
        let raised = coordinator.apply_pending();
        assert_eq!(raised.len(), 1);
        assert!(matches!(
            raised[0].result.as_ref().unwrap().transition,
            WindowTransition::Raised { id } if id == activated
        ));
        assert_eq!(frozen.paint_order(), ids);
        assert_eq!(
            coordinator.begin_frame().paint_order(),
            [ids[0], ids[1], ids[3], ids[2]]
        );
    }

    #[test]
    fn fleet_window_switching_selected_fleet_preserves_window_identity() {
        let mut fixture = CrossFamilyDetailFixture::new();
        let first = fixture.add_fleet();
        let second = fixture.add_fleet();
        let mut coordinator = StrategicWindowing::new();
        let ids = fixture.publish_all(&mut coordinator, Some(FleetWindowEntry::Fleet(first)), true);
        assert!(fixture
            .fleet_windows
            .select(fixture.system, FleetWindowEntry::Fleet(second)));
        assert_eq!(
            fixture.fleet_windows.window_id(fixture.system),
            Some(ids[1])
        );
        assert_eq!(
            fixture.fleet_windows.selection(fixture.system),
            Some((
                Some(FleetWindowEntry::Fleet(second)),
                rebellion_render::FleetWindowTab::CapitalShips,
            ))
        );
        assert_eq!(
            coordinator.begin_frame().key(ids[1]),
            Some(WindowKey::contextual(
                WindowKind::Fleet,
                system_subject(&fixture.world, fixture.system).unwrap(),
            ))
        );
    }

    #[test]
    fn detail_windows_reopen_preserves_tabs_rows_scroll_and_selection() {
        let mut fixture = CrossFamilyDetailFixture::new();
        let selected = fixture.add_fleet();
        for index in 0..12 {
            fixture.add_character(&format!("Agent {index:02}"), false);
        }
        let mission_decoy = fixture.add_mission_with_decoy();
        let mut coordinator = StrategicWindowing::new();
        let ids = fixture.publish_all(
            &mut coordinator,
            Some(FleetWindowEntry::Fleet(selected)),
            true,
        );
        fixture.select_system_tab(&coordinator, SystemWindowTab::Mines);
        assert_eq!(
            fixture.system_windows.window_id(fixture.system),
            Some(ids[0])
        );
        assert_eq!(fixture.system_windows.window_count(), 1);
        assert!(fixture.missions_windows.show_member(
            &fixture.missions,
            fixture.system,
            mission_decoy
        ));
        assert_eq!(
            fixture.scroll_and_select_defenses(&coordinator),
            (3..12)
                .map(|index| format!("Agent {index:02}"))
                .collect::<Vec<_>>()
        );
        let before = fixture.content_observation(&coordinator);
        assert_eq!(before.system, (SystemWindowTab::Mines, None));
        assert_eq!(
            before.fleet.selected,
            Some(FleetWindowEntry::Fleet(selected))
        );
        assert_eq!(before.defenses.rows.len(), 12);
        assert_eq!(before.defenses.selected, Some(3));
        assert_eq!(before.missions.selected, Some(0));
        assert_eq!(
            before.missions.tab,
            rebellion_render::missions_window::MissionsTab::Decoys
        );

        coordinator.enqueue_system_minimize(ids[0]);
        coordinator.enqueue_fleet_minimize(ids[1]);
        coordinator.enqueue_defenses_minimize(ids[2]);
        coordinator.enqueue_missions_minimize(ids[3]);
        let minimized = coordinator.apply_pending();
        assert_eq!(minimized.len(), 4);
        assert_eq!(coordinator.begin_frame().presentation_history(), ids);

        let reopened = fixture.publish_all(&mut coordinator, None, false);
        assert_eq!(reopened, ids);
        assert!(coordinator.begin_frame().presentation_history().is_empty());
        assert_eq!(coordinator.begin_frame().paint_order(), ids);
        assert_eq!(fixture.content_observation(&coordinator), before);
    }

    #[test]
    fn reference_rail_thirteenth_entry_tears_down_oldest_content() {
        let mut coordinator = StrategicWindowing::new();
        let mut ids = Vec::new();
        for value in 0..13 {
            coordinator.enqueue(WindowCommand::Open(contextual_spec(
                WindowKind::System,
                SubjectId::new(2, 10_000 + value),
                WindowOwner::StrategicRoot,
            )));
            let opened = coordinator.apply_pending().remove(0);
            let id = opened_id(&opened);
            assert!(coordinator.attach_content(id));
            coordinator.enqueue(WindowCommand::Minimize(id));
            let minimized = coordinator.apply_pending().remove(0);
            if value < 12 {
                assert!(minimized.removed_content.is_empty());
            } else {
                assert!(matches!(
                    minimized.result.as_ref().unwrap().transition,
                    WindowTransition::Minimized {
                        id: minimized_id,
                        ref evicted,
                    } if minimized_id == id && evicted == &[ids[0]]
                ));
                assert_eq!(minimized.removed_content, [ids[0]]);
            }
            ids.push(id);
        }

        let snapshot = coordinator.begin_frame();
        assert_eq!(snapshot.presentation_history(), &ids[1..]);
        assert!(snapshot.key(ids[0]).is_none());
        assert!(!coordinator.has_content(ids[0]));
        assert!(ids[1..].iter().all(|id| coordinator.has_content(*id)));
    }

    #[test]
    fn reference_rail_close_removes_only_matching_history_entry() {
        let mut coordinator = StrategicWindowing::new();
        let mut ids = Vec::new();
        for value in 0..3 {
            coordinator.enqueue(WindowCommand::Open(contextual_spec(
                WindowKind::Fleet,
                SubjectId::new(2, 20_000 + value),
                WindowOwner::StrategicRoot,
            )));
            let opened = coordinator.apply_pending().remove(0);
            let id = opened_id(&opened);
            assert!(coordinator.attach_content(id));
            coordinator.enqueue(WindowCommand::Minimize(id));
            assert!(coordinator.apply_pending()[0].removed_content.is_empty());
            ids.push(id);
        }
        assert_eq!(coordinator.begin_frame().presentation_history(), ids);

        coordinator.enqueue(WindowCommand::Close(ids[1]));
        let closed = coordinator.apply_pending().remove(0);
        assert_eq!(closed.removed_content, [ids[1]]);
        assert_eq!(
            coordinator.begin_frame().presentation_history(),
            [ids[0], ids[2]]
        );
        assert!(coordinator.has_content(ids[0]));
        assert!(!coordinator.has_content(ids[1]));
        assert!(coordinator.has_content(ids[2]));
    }

    #[test]
    fn detail_windows_stale_system_subject_rejects_action_without_retargeting() {
        let mut fixture = CrossFamilyDetailFixture::new();
        let stale_system = fixture.system;
        let mut coordinator = StrategicWindowing::new();
        let ids = fixture.publish_all(&mut coordinator, None, true);
        let mut replacement = fixture.world.systems.remove(stale_system).unwrap();
        replacement.dat_id = DatId::new(101);
        replacement.name = "Replacement".into();
        let replacement = fixture.world.systems.insert(replacement);
        assert_ne!(replacement, stale_system);

        assert!(fixture
            .system_windows
            .prepare_open(
                &fixture.world,
                stale_system,
                (40, 50),
                CockpitFaction::Alliance,
                fleet_layout(),
            )
            .is_none());
        assert!(fixture
            .fleet_windows
            .prepare_open(
                &fixture.world,
                stale_system,
                (40, 50),
                CockpitFaction::Alliance,
                fleet_layout(),
                None,
            )
            .is_none());
        assert!(fixture
            .defenses_windows
            .prepare_open(
                &fixture.world,
                stale_system,
                (40, 50),
                CockpitFaction::Alliance,
                fleet_layout(),
            )
            .is_none());
        assert!(fixture
            .missions_windows
            .prepare_open(
                &fixture.world,
                &fixture.fog,
                &fixture.missions,
                stale_system,
                (40, 50),
                CockpitFaction::Alliance,
                fleet_layout(),
            )
            .is_none());

        let frame = coordinator.begin_frame();
        let reports = [
            fixture
                .system_windows
                .content_reports(&frame, &fixture.world),
            fixture
                .fleet_windows
                .content_reports(&frame, &fixture.world),
            fixture
                .defenses_windows
                .content_reports(&frame, &fixture.world),
            fixture
                .missions_windows
                .content_reports(&frame, &fixture.world),
        ]
        .concat();
        assert_eq!(reports.len(), 4);
        assert!(reports
            .iter()
            .all(|report| report.status == ContentStatus::Absent));
        for (state_id, expected_id) in [
            fixture.system_windows.window_id(stale_system),
            fixture.fleet_windows.window_id(stale_system),
            fixture.defenses_windows.window_id(stale_system),
            fixture.missions_windows.window_id(stale_system),
        ]
        .into_iter()
        .zip(ids)
        {
            assert_eq!(state_id, Some(expected_id));
        }
        assert!(fixture.system_windows.window_id(replacement).is_none());
        assert!(fixture.fleet_windows.window_id(replacement).is_none());
        assert!(fixture.defenses_windows.window_id(replacement).is_none());
        assert!(fixture.missions_windows.window_id(replacement).is_none());
    }

    #[test]
    fn detail_windows_background_open_preserves_active_modal_focus() {
        let mut coordinator = StrategicWindowing::new();
        coordinator
            .enqueue_fleet_finder_open(FleetFinderState::prepare_open(CockpitFaction::Alliance));
        let modal = opened_id(&coordinator.apply_pending()[0]);
        let focused = coordinator.begin_frame().focused();

        let (world, system) = fleet_world();
        let mut system_windows = SystemWindowState::default();
        coordinator.enqueue_system_open(
            system_windows
                .prepare_open(
                    &world,
                    system,
                    (40, 50),
                    CockpitFaction::Alliance,
                    fleet_layout(),
                )
                .unwrap(),
        );
        let mut opened = coordinator.apply_pending().remove(0);
        let detail = opened.system_content_id.unwrap();
        assert_eq!(
            system_windows.commit_open(
                opened.prepared_system.take().unwrap(),
                &opened.result.as_ref().unwrap().transition,
            ),
            Some(detail)
        );
        assert!(coordinator.acknowledge_system_publication(detail));

        let snapshot = coordinator.begin_frame();
        assert_eq!(snapshot.active_modal(), Some(modal));
        assert_eq!(snapshot.focused(), focused);
        assert_eq!(snapshot.paint_order(), [detail, modal]);
        assert!(!snapshot.allows_input(detail));
    }

    #[test]
    fn system_detail_commits_raise_restore_and_exact_removal_receipts() {
        let (world, system) = fleet_world();
        let mut state = SystemWindowState::default();
        let mut coordinator = StrategicWindowing::new();
        let prepared = state
            .prepare_open(
                &world,
                system,
                (40, 50),
                CockpitFaction::Alliance,
                fleet_layout(),
            )
            .unwrap();

        coordinator.enqueue_system_open(prepared);
        assert!(coordinator.begin_frame().paint_order().is_empty());
        let mut opened = coordinator.apply_pending().remove(0);
        let id = opened.system_content_id.unwrap();
        let commit = opened.result.as_ref().unwrap();
        assert_eq!(
            state.commit_open(opened.prepared_system.take().unwrap(), &commit.transition),
            Some(id)
        );
        assert!(coordinator.acknowledge_system_publication(id));
        assert_eq!(state.window_id(system), Some(id));

        coordinator.enqueue_system_minimize(id);
        let minimized = coordinator.apply_pending().remove(0);
        assert!(matches!(
            minimized.result.as_ref().unwrap().transition,
            WindowTransition::Minimized { id: minimized_id, .. } if minimized_id == id
        ));
        assert!(coordinator.has_content(id));
        assert!(!coordinator.begin_frame().is_visible(id));
        assert_eq!(coordinator.begin_frame().presentation_history(), [id]);

        let reopened = state
            .prepare_open(
                &world,
                system,
                (300, 300),
                CockpitFaction::Alliance,
                fleet_layout(),
            )
            .unwrap();
        coordinator.enqueue_system_open(reopened);
        let mut restored = coordinator.apply_pending().remove(0);
        assert_eq!(restored.system_content_id, Some(id));
        assert_eq!(
            state.commit_open(
                restored.prepared_system.take().unwrap(),
                &restored.result.as_ref().unwrap().transition,
            ),
            Some(id)
        );
        assert!(coordinator.begin_frame().is_visible(id));
        assert!(coordinator.begin_frame().presentation_history().is_empty());

        coordinator.enqueue(WindowCommand::Close(id));
        let closed = coordinator.apply_pending().remove(0);
        assert_eq!(closed.removed_content, [id]);
        state.remove_content(&closed.removed_content);
        assert!(!state.contains_window(id));
        assert!(!coordinator.has_content(id));
    }

    #[test]
    fn fleet_commit_publishes_content_and_history_only_after_manager_transition() {
        let (world, system) = fleet_world();
        let mut state = FleetWindowState::default();
        let mut coordinator = StrategicWindowing::new();
        let prepared = state
            .prepare_open(
                &world,
                system,
                (40, 50),
                CockpitFaction::Alliance,
                fleet_layout(),
                None,
            )
            .unwrap();

        coordinator.enqueue_fleet_open(prepared);
        assert!(coordinator.begin_frame().paint_order().is_empty());
        let mut opened = coordinator.apply_pending().remove(0);
        let id = opened.fleet_content_id.unwrap();
        let commit = opened.result.as_ref().unwrap();
        assert_eq!(
            state.commit_open(opened.prepared_fleet.take().unwrap(), &commit.transition),
            Some(id)
        );
        assert!(!coordinator.has_content(id));
        assert!(coordinator.acknowledge_fleet_publication(id));
        assert!(coordinator.has_content(id));
        assert_eq!(state.window_id(system), Some(id));

        coordinator.enqueue_fleet_minimize(id);
        let minimized = coordinator.apply_pending().remove(0);
        assert!(matches!(
            minimized.result.as_ref().unwrap().transition,
            WindowTransition::Minimized {
                id: minimized_id, ..
            } if minimized_id == id
        ));
        assert!(coordinator.has_content(id));
        assert!(!coordinator.begin_frame().is_visible(id));
        assert_eq!(coordinator.begin_frame().presentation_history(), [id]);

        coordinator.enqueue(WindowCommand::Restore(id));
        let restored = coordinator.apply_pending().remove(0);
        assert_eq!(restored.fleet_content_id, None);
        assert!(matches!(
            restored.result.as_ref().unwrap().transition,
            WindowTransition::Restored { id: restored_id, .. } if restored_id == id
        ));
        assert!(coordinator.begin_frame().is_visible(id));
        assert!(coordinator.begin_frame().presentation_history().is_empty());

        coordinator.enqueue(WindowCommand::Close(id));
        let closed = coordinator.apply_pending().remove(0);
        assert_eq!(closed.removed_content, [id]);
        state.remove_content(&closed.removed_content);
        assert!(!coordinator.has_content(id));
        assert_eq!(state.window_count(), 0);

        coordinator.enqueue_fleet_minimize(id);
        let rejected = coordinator.apply_pending().remove(0);
        assert!(rejected.result.is_err());
    }

    #[test]
    fn defenses_detail_commits_raise_restore_and_exact_removal_receipts() {
        let (world, system) = fleet_world();
        let mut state = DefensesWindowState::default();
        let mut coordinator = StrategicWindowing::new();
        let prepared = state
            .prepare_open(
                &world,
                system,
                (40, 50),
                CockpitFaction::Alliance,
                fleet_layout(),
            )
            .unwrap();

        coordinator.enqueue_defenses_open(prepared);
        assert!(coordinator.begin_frame().paint_order().is_empty());
        let mut opened = coordinator.apply_pending().remove(0);
        let id = opened.defenses_content_id.unwrap();
        assert_eq!(
            state.commit_open(
                opened.prepared_defenses.take().unwrap(),
                &opened.result.as_ref().unwrap().transition,
            ),
            Some(id)
        );
        assert!(coordinator.acknowledge_defenses_publication(id));
        assert_eq!(state.window_id(system), Some(id));

        coordinator.enqueue_defenses_minimize(id);
        let minimized = coordinator.apply_pending().remove(0);
        assert!(matches!(
            minimized.result.as_ref().unwrap().transition,
            WindowTransition::Minimized { id: minimized_id, .. } if minimized_id == id
        ));
        assert!(coordinator.has_content(id));
        assert!(!coordinator.begin_frame().is_visible(id));
        assert!(state.contains_window(id));

        coordinator.enqueue(WindowCommand::Restore(id));
        let restored = coordinator.apply_pending().remove(0);
        assert_eq!(restored.defenses_content_id, None);
        assert!(matches!(
            restored.result.as_ref().unwrap().transition,
            WindowTransition::Restored { id: restored_id, .. } if restored_id == id
        ));
        assert!(coordinator.begin_frame().is_visible(id));
        assert!(state.contains_window(id));

        coordinator.enqueue(WindowCommand::Close(id));
        let closed = coordinator.apply_pending().remove(0);
        assert_eq!(closed.removed_content, [id]);
        state.remove_content(&closed.removed_content);
        assert!(!state.contains_window(id));
        assert!(!coordinator.has_content(id));
    }

    #[test]
    fn missions_detail_commits_raise_restore_and_exact_removal_receipts() {
        let (world, system) = fleet_world();
        let fog = FogState::new(Faction::Alliance);
        let missions = MissionState::new();
        let mut state = MissionsWindowState::default();
        let mut coordinator = StrategicWindowing::new();
        let prepared = state
            .prepare_open(
                &world,
                &fog,
                &missions,
                system,
                (40, 50),
                CockpitFaction::Alliance,
                fleet_layout(),
            )
            .unwrap();

        coordinator.enqueue_missions_open(prepared);
        assert!(coordinator.begin_frame().paint_order().is_empty());
        let mut opened = coordinator.apply_pending().remove(0);
        let id = opened.missions_content_id.unwrap();
        assert_eq!(
            state.commit_open(
                opened.prepared_missions.take().unwrap(),
                &opened.result.as_ref().unwrap().transition,
            ),
            Some(id)
        );
        assert!(coordinator.acknowledge_missions_publication(id));
        assert_eq!(state.window_id(system), Some(id));

        coordinator.enqueue_missions_minimize(id);
        let minimized = coordinator.apply_pending().remove(0);
        assert!(matches!(
            minimized.result.as_ref().unwrap().transition,
            WindowTransition::Minimized { id: minimized_id, .. } if minimized_id == id
        ));
        assert!(coordinator.has_content(id));
        assert!(!coordinator.begin_frame().is_visible(id));
        assert!(state.contains_window(id));

        coordinator.enqueue(WindowCommand::Restore(id));
        let restored = coordinator.apply_pending().remove(0);
        assert_eq!(restored.missions_content_id, None);
        assert!(matches!(
            restored.result.as_ref().unwrap().transition,
            WindowTransition::Restored { id: restored_id, .. } if restored_id == id
        ));
        assert!(coordinator.begin_frame().is_visible(id));
        assert!(state.contains_window(id));

        coordinator.enqueue(WindowCommand::Close(id));
        let closed = coordinator.apply_pending().remove(0);
        assert_eq!(closed.removed_content, [id]);
        state.remove_content(&closed.removed_content);
        assert!(!state.contains_window(id));
        assert!(!coordinator.has_content(id));
    }

    #[test]
    fn rejected_renderer_publication_rolls_back_the_exact_manager_identity() {
        let (world, system) = fleet_world();
        let mut state = FleetWindowState::default();
        assert!(state.open(
            &world,
            system,
            (10, 20),
            CockpitFaction::Alliance,
            fleet_layout(),
        ));
        let prepared = state
            .prepare_open(
                &world,
                system,
                (40, 50),
                CockpitFaction::Alliance,
                fleet_layout(),
                None,
            )
            .unwrap();
        let mut coordinator = StrategicWindowing::new();
        coordinator.enqueue_fleet_open(prepared);
        let mut applied = coordinator.apply_pending().remove(0);
        let id = applied.fleet_content_id.unwrap();
        assert!(coordinator.begin_frame().is_visible(id));
        assert!(!coordinator.has_content(id));
        assert_eq!(
            state.commit_open(
                applied.prepared_fleet.take().unwrap(),
                &applied.result.as_ref().unwrap().transition,
            ),
            None,
        );

        assert_eq!(coordinator.rollback_fleet_publication(id), [id]);
        assert!(!coordinator.begin_frame().is_visible(id));
        assert!(!coordinator.has_content(id));
    }

    #[test]
    fn finder_duplicate_open_preserves_id_selection_and_focus() {
        let mut coordinator = StrategicWindowing::new();
        let mut finder = FleetFinderState::default();
        coordinator
            .enqueue_fleet_finder_open(FleetFinderState::prepare_open(CockpitFaction::Alliance));
        assert!(coordinator.begin_frame().paint_order().is_empty());

        let mut opened = coordinator.apply_pending().remove(0);
        let id = opened.fleet_finder_content_id.unwrap();
        let transition = &opened.result.as_ref().unwrap().transition;
        assert_eq!(
            finder.commit_open(opened.prepared_fleet_finder.take().unwrap(), transition),
            Some(id)
        );
        assert!(coordinator.acknowledge_fleet_finder_publication(id));
        let name_focus = FocusTarget {
            window: id,
            control: Some(FLEET_FINDER_NAME_CONTROL),
        };
        assert_eq!(coordinator.begin_frame().focused(), Some(name_focus));

        finder.set_tab(rebellion_render::fleet_finder::FinderTab::Imperial);
        coordinator
            .enqueue_fleet_finder_open(FleetFinderState::prepare_open(CockpitFaction::Empire));
        let mut duplicate = coordinator.apply_pending().remove(0);
        assert!(matches!(
            duplicate.result.as_ref().unwrap().transition,
            WindowTransition::Existing {
                id: existing,
                effect: ExistingEffect::NoOp,
            } if existing == id
        ));
        assert_eq!(duplicate.fleet_finder_content_id, Some(id));
        assert_eq!(
            finder.commit_open(
                duplicate.prepared_fleet_finder.take().unwrap(),
                &duplicate.result.as_ref().unwrap().transition,
            ),
            Some(id)
        );
        assert_eq!(finder.window_id(), Some(id));
        assert_eq!(coordinator.begin_frame().focused(), Some(name_focus));
    }

    #[test]
    fn troop_finder_duplicate_open_preserves_id_selection_and_focus() {
        let mut coordinator = StrategicWindowing::new();
        let mut finder = TroopFinderState::default();
        coordinator
            .enqueue_troop_finder_open(TroopFinderState::prepare_open(CockpitFaction::Alliance));

        let mut opened = coordinator.apply_pending().remove(0);
        let id = opened.troop_finder_content_id.unwrap();
        assert_eq!(
            finder.commit_open(
                opened.prepared_troop_finder.take().unwrap(),
                &opened.result.as_ref().unwrap().transition,
            ),
            Some(id)
        );
        assert!(coordinator.acknowledge_troop_finder_publication(id));
        let name_focus = FocusTarget {
            window: id,
            control: Some(TROOP_FINDER_NAME_CONTROL),
        };
        assert_eq!(coordinator.begin_frame().focused(), Some(name_focus));

        finder.set_tab(TroopTab::Imperial);
        coordinator
            .enqueue_troop_finder_open(TroopFinderState::prepare_open(CockpitFaction::Empire));
        let mut duplicate = coordinator.apply_pending().remove(0);
        assert!(matches!(
            duplicate.result.as_ref().unwrap().transition,
            WindowTransition::Existing {
                id: existing,
                effect: ExistingEffect::NoOp,
            } if existing == id
        ));
        assert_eq!(duplicate.troop_finder_content_id, Some(id));
        assert_eq!(
            finder.commit_open(
                duplicate.prepared_troop_finder.take().unwrap(),
                &duplicate.result.as_ref().unwrap().transition,
            ),
            Some(id)
        );
        assert_eq!(finder.window_id(), Some(id));
        assert_eq!(coordinator.begin_frame().focused(), Some(name_focus));
    }

    #[test]
    fn troop_finder_wrong_publication_is_rejected_and_rolled_back() {
        let mut coordinator = StrategicWindowing::new();
        coordinator
            .enqueue_fleet_finder_open(FleetFinderState::prepare_open(CockpitFaction::Alliance));
        let fleet_finder = opened_id(&coordinator.apply_pending()[0]);
        assert!(!coordinator.acknowledge_troop_finder_publication(fleet_finder));

        coordinator
            .enqueue_troop_finder_open(TroopFinderState::prepare_open(CockpitFaction::Alliance));
        let applied = coordinator.apply_pending().remove(0);
        let troop_finder = applied.troop_finder_content_id.unwrap();
        assert_ne!(troop_finder, fleet_finder);
        assert!(!coordinator.has_content(troop_finder));

        assert_eq!(
            coordinator.rollback_troop_finder_publication(troop_finder),
            [troop_finder]
        );
        assert!(!coordinator.begin_frame().is_visible(troop_finder));
        assert!(!coordinator.has_content(troop_finder));
    }

    #[test]
    fn personnel_finder_duplicate_open_preserves_id_selection_and_focus() {
        let mut coordinator = StrategicWindowing::new();
        let mut finder = PersonnelFinderState::default();
        coordinator.enqueue_personnel_finder_open(PersonnelFinderState::prepare_open(
            CockpitFaction::Alliance,
        ));

        let mut opened = coordinator.apply_pending().remove(0);
        let id = opened.personnel_finder_content_id.unwrap();
        assert_eq!(
            finder.commit_open(
                opened.prepared_personnel_finder.take().unwrap(),
                &opened.result.as_ref().unwrap().transition,
            ),
            Some(id)
        );
        assert!(coordinator.acknowledge_personnel_finder_publication(id));
        let name_focus = FocusTarget {
            window: id,
            control: Some(PERSONNEL_FINDER_NAME_CONTROL),
        };
        assert_eq!(coordinator.begin_frame().focused(), Some(name_focus));

        finder.set_tab(PersonnelTab::Imperial);
        finder.set_view(PersonnelView::Characters);
        coordinator.enqueue_personnel_finder_open(PersonnelFinderState::prepare_open(
            CockpitFaction::Empire,
        ));
        let mut duplicate = coordinator.apply_pending().remove(0);
        assert!(matches!(
            duplicate.result.as_ref().unwrap().transition,
            WindowTransition::Existing {
                id: existing,
                effect: ExistingEffect::NoOp,
            } if existing == id
        ));
        assert_eq!(duplicate.personnel_finder_content_id, Some(id));
        assert_eq!(
            finder.commit_open(
                duplicate.prepared_personnel_finder.take().unwrap(),
                &duplicate.result.as_ref().unwrap().transition,
            ),
            Some(id)
        );
        assert_eq!(finder.window_id(), Some(id));
        assert_eq!(coordinator.begin_frame().focused(), Some(name_focus));
    }

    #[test]
    fn personnel_finder_wrong_publication_is_rejected_and_rolled_back() {
        let mut coordinator = StrategicWindowing::new();
        coordinator
            .enqueue_fleet_finder_open(FleetFinderState::prepare_open(CockpitFaction::Alliance));
        let fleet_finder = opened_id(&coordinator.apply_pending()[0]);
        assert!(!coordinator.acknowledge_personnel_finder_publication(fleet_finder));

        coordinator.enqueue_personnel_finder_open(PersonnelFinderState::prepare_open(
            CockpitFaction::Alliance,
        ));
        let applied = coordinator.apply_pending().remove(0);
        let personnel_finder = applied.personnel_finder_content_id.unwrap();
        assert_ne!(personnel_finder, fleet_finder);
        assert!(!coordinator.has_content(personnel_finder));

        assert_eq!(
            coordinator.rollback_personnel_finder_publication(personnel_finder),
            [personnel_finder]
        );
        assert!(!coordinator.begin_frame().is_visible(personnel_finder));
        assert!(!coordinator.has_content(personnel_finder));
    }

    #[test]
    fn message_index_app_state_closes_only_from_exact_receipt() {
        let mut coordinator = StrategicWindowing::new();
        let mut index = MessageIndexState::new();
        coordinator.enqueue_message_index_open(
            MessageIndexState::prepare_open(0x82)
                .unwrap()
                .with_initial_row(2),
        );
        assert!(coordinator.begin_frame().paint_order().is_empty());

        let mut opened = coordinator.apply_pending().remove(0);
        assert_eq!(
            opened
                .prepared_message_index
                .as_ref()
                .unwrap()
                .initial_row(),
            Some(2)
        );
        let id = opened.message_index_content_id.unwrap();
        let (_, actions) = index
            .commit_open(
                opened.prepared_message_index.take().unwrap(),
                &opened.result.as_ref().unwrap().transition,
            )
            .unwrap();
        assert_eq!(actions, [MessageIndexAction::AdviceShown]);
        assert!(coordinator.acknowledge_message_index_publication(id));
        assert!(coordinator.has_content(id));

        coordinator.enqueue_message_index_open(MessageIndexState::prepare_open(0x79).unwrap());
        let mut duplicate = coordinator.apply_pending().remove(0);
        assert_eq!(duplicate.message_index_content_id, Some(id));
        assert_eq!(
            index.commit_open(
                duplicate.prepared_message_index.take().unwrap(),
                &duplicate.result.as_ref().unwrap().transition,
            ),
            Some((id, Vec::new()))
        );
        assert_eq!(index.category(), 0x82);

        let closing_frame = coordinator.begin_frame();
        coordinator.enqueue(WindowCommand::Close(id));
        let closed = coordinator.apply_pending().remove(0);
        assert_eq!(closed.removed_content, [id]);
        assert_eq!(closing_frame.active_modal(), Some(id));
        assert!(index.is_open(), "app state waits for the close receipt");
        assert_eq!(
            index.remove_content(&closed.removed_content),
            [
                MessageIndexAction::AdviceHidden,
                MessageIndexAction::MarkCategoryRead(Some(
                    rebellion_render::message_log::MessageRail::Advice,
                )),
            ]
        );
        assert!(!index.is_open());
        assert!(!coordinator.has_content(id));
    }

    #[test]
    fn status_context_replacement_publishes_new_identity_atomically() {
        let mut coordinator = StrategicWindowing::new();
        let mut state = StatusWindowState::default();
        let mut world = GameWorld::default();
        let first = world.characters.insert(Default::default());
        let second = world.characters.insert(Default::default());
        let missions = rebellion_core::missions::MissionState::new();
        let movement = rebellion_core::movement::MovementState::new();
        let transport = rebellion_core::troop_transport::TroopTransportState::new();
        let manufacturing = rebellion_core::manufacturing::ManufacturingState::new();
        let sources = StatusSources {
            world: &world,
            missions: &missions,
            movement: &movement,
            transport: &transport,
            manufacturing: &manufacturing,
            today: 0,
        };

        let first_subject = coordinator.next_context_subject().unwrap();
        coordinator.enqueue_status_open(
            StatusWindowState::prepare_open(sources, StatusObject::Character(first), first_subject)
                .unwrap(),
            None,
        );
        let mut opened = coordinator.apply_pending().remove(0);
        let old = opened.status_content_id.unwrap();
        assert_eq!(
            state.commit_open(
                opened.prepared_status.take().unwrap(),
                &opened.result.as_ref().unwrap().transition,
            ),
            Some(old)
        );
        assert!(coordinator.acknowledge_status_publication(old));

        let second_subject = coordinator.next_context_subject().unwrap();
        coordinator.enqueue_status_open(
            StatusWindowState::prepare_open(
                sources,
                StatusObject::Character(second),
                second_subject,
            )
            .unwrap(),
            Some(old),
        );
        assert_eq!(state.window_id(), Some(old));
        let mut replaced = coordinator.apply_pending().remove(0);
        let new = replaced.status_content_id.unwrap();
        assert_ne!(new, old);
        assert_eq!(replaced.removed_content, [old]);
        assert_eq!(
            state.commit_open(
                replaced.prepared_status.take().unwrap(),
                &replaced.result.as_ref().unwrap().transition,
            ),
            Some(new)
        );
        state.remove_content(&replaced.removed_content);
        assert_eq!(state.window_id(), Some(new));
        assert!(coordinator.acknowledge_status_publication(new));
        assert!(!coordinator.has_content(old));
        assert!(coordinator.has_content(new));
    }

    #[test]
    fn finder_unhandled_function_key_stops_at_modal_boundary() {
        let mut coordinator = StrategicWindowing::new();
        coordinator
            .enqueue_fleet_finder_open(FleetFinderState::prepare_open(CockpitFaction::Alliance));
        let id = opened_id(&coordinator.apply_pending()[0]);
        let frame = coordinator.begin_frame();
        let event = InputEventId {
            frame: 1,
            ordinal: 0,
        };
        let mut router = FrameInputRouter::default();
        router.begin_frame(1, &frame);
        router.collect_keyboard(event);

        assert_eq!(router.keyboard_recipient(event), InputRecipient::Window(id));
        router.report_key(
            event,
            InputRecipient::Window(id),
            InputDisposition::Unhandled,
        );
        assert!(!router.claim_root(event));
    }

    #[test]
    fn finder_close_key_cannot_open_another_surface_in_same_frame() {
        let mut coordinator = StrategicWindowing::new();
        coordinator
            .enqueue_fleet_finder_open(FleetFinderState::prepare_open(CockpitFaction::Alliance));
        let id = opened_id(&coordinator.apply_pending()[0]);
        let closing_frame = coordinator.begin_production_frame(
            ApplicationInputLocks::default(),
            LegacyInputBlockers::default(),
        );
        let event = InputEventId {
            frame: 7,
            ordinal: 0,
        };
        let mut router = FrameInputRouter::default();
        router.begin_frame(7, closing_frame.snapshot());
        router.collect_keyboard(event);
        router.report_key(event, InputRecipient::Window(id), InputDisposition::Handled);

        coordinator.enqueue(WindowCommand::Close(id));
        assert!(coordinator.apply_pending()[0].result.is_ok());

        assert!(!router.claim_root(event));
        assert_eq!(
            closing_frame
                .snapshot()
                .pointer_route(&BTreeSet::new())
                .target,
            RouteTarget::BlockedBy(id)
        );
        assert!(!closing_frame.root_shortcuts_allowed());
        assert!(coordinator.begin_frame().paint_order().is_empty());
    }

    #[test]
    fn finder_modal_blocks_unmigrated_legacy_input_paths() {
        let mut coordinator = StrategicWindowing::new();
        coordinator
            .enqueue_fleet_finder_open(FleetFinderState::prepare_open(CockpitFaction::Alliance));
        let id = opened_id(&coordinator.apply_pending()[0]);
        let frame = coordinator.begin_production_frame(
            ApplicationInputLocks::default(),
            LegacyInputBlockers::default(),
        );

        assert_eq!(frame.snapshot().active_modal(), Some(id));
        assert!(!frame.retained_window_input_allowed());
        assert!(!frame.legacy_surface_input_allowed());
        assert!(!frame.legacy_owned_pointer_allowed());
        assert!(!frame.legacy_pointer_allowed());
        assert!(!frame.legacy_keyboard_allowed());
        assert!(!frame.root_shortcuts_allowed());
    }

    #[test]
    fn strategic_windowing_pending_commands_do_not_mutate_current_snapshot() {
        let mut coordinator = StrategicWindowing::new();
        let before = coordinator.begin_frame();
        coordinator.enqueue(WindowCommand::Open(singleton_spec(
            WindowKind::Encyclopedia,
            WindowOwner::StrategicRoot,
        )));
        assert!(before.paint_order().is_empty());
        assert!(coordinator.begin_frame().paint_order().is_empty());
        let applied = coordinator.apply_pending();
        assert_eq!(
            coordinator.begin_frame().paint_order(),
            &[opened_id(&applied[0])]
        );
    }

    #[test]
    fn apply_batch_preserves_non_sector_order_and_defers_receipt_side_enqueue() {
        let mut coordinator = StrategicWindowing::new();
        for kind in [WindowKind::Encyclopedia, WindowKind::MessageIndex] {
            coordinator.enqueue(WindowCommand::Open(singleton_spec(
                kind,
                WindowOwner::StrategicRoot,
            )));
        }
        assert_eq!(coordinator.begin_apply_batch(), 2);
        let first = coordinator.apply_next_pending().unwrap();
        assert!(matches!(
            first.command,
            WindowCommand::Open(spec) if spec.key.kind == WindowKind::Encyclopedia
        ));
        coordinator.enqueue(WindowCommand::Open(singleton_spec(
            WindowKind::Status,
            WindowOwner::StrategicRoot,
        )));
        let second = coordinator.apply_next_pending().unwrap();
        assert!(matches!(
            second.command,
            WindowCommand::Open(spec) if spec.key.kind == WindowKind::MessageIndex
        ));
        assert!(coordinator.apply_next_pending().is_none());
        assert_eq!(coordinator.begin_apply_batch(), 1);
        let next_frame = coordinator.apply_next_pending().unwrap();
        assert!(matches!(
            next_frame.command,
            WindowCommand::Open(spec) if spec.key.kind == WindowKind::Status
        ));
    }

    #[test]
    fn strategic_windowing_failed_open_preserves_previous_content() {
        let mut coordinator = StrategicWindowing::new();
        for subject in [1, 2] {
            coordinator.enqueue(WindowCommand::Open(contextual_spec(
                WindowKind::Sector,
                SubjectId::new(1, subject),
                WindowOwner::StrategicRoot,
            )));
        }
        let applied = coordinator.apply_pending();
        let first = opened_id(&applied[0]);
        assert!(coordinator.attach_content(first));
        coordinator.enqueue(WindowCommand::Open(contextual_spec(
            WindowKind::Sector,
            SubjectId::new(1, 3),
            WindowOwner::StrategicRoot,
        )));
        assert!(coordinator.apply_pending()[0].result.is_err());
        assert!(coordinator.has_content(first));
        assert!(coordinator.begin_frame().is_visible(first));
    }

    #[test]
    fn strategic_windowing_receipts_remove_only_matching_content_ids() {
        let mut coordinator = StrategicWindowing::new();
        for kind in [WindowKind::Encyclopedia, WindowKind::MessageIndex] {
            coordinator.enqueue(WindowCommand::Open(singleton_spec(
                kind,
                WindowOwner::StrategicRoot,
            )));
        }
        let applied = coordinator.apply_pending();
        let first = opened_id(&applied[0]);
        let second = opened_id(&applied[1]);
        assert!(coordinator.attach_content(first));
        assert!(coordinator.attach_content(second));
        coordinator.enqueue(WindowCommand::Close(first));
        let closed = coordinator.apply_pending();
        assert_eq!(closed[0].removed_content, vec![first]);
        assert!(!coordinator.has_content(first));
        assert!(coordinator.has_content(second));
    }

    #[test]
    fn strategic_windowing_reset_discards_pending_old_campaign_actions() {
        let mut coordinator = StrategicWindowing::new();
        coordinator.enqueue(WindowCommand::Open(singleton_spec(
            WindowKind::Encyclopedia,
            WindowOwner::StrategicRoot,
        )));
        assert_eq!(coordinator.begin_apply_batch(), 1);
        coordinator.reset_session();
        assert!(coordinator.apply_pending().is_empty());
        assert!(coordinator.begin_frame().paint_order().is_empty());
    }

    #[test]
    fn suspended_scene_defers_new_window_identity_until_an_active_scene() {
        let mut coordinator = StrategicWindowing::new();
        coordinator.enter_scene(WindowScene::Suspended);
        coordinator.enqueue(WindowCommand::Open(singleton_spec(
            WindowKind::Encyclopedia,
            WindowOwner::StrategicRoot,
        )));
        assert!(coordinator.apply_pending().is_empty());
        assert!(coordinator.begin_frame().paint_order().is_empty());
        coordinator.enter_scene(WindowScene::Galaxy);
        assert_eq!(coordinator.apply_pending().len(), 1);
        assert_eq!(coordinator.begin_frame().paint_order().len(), 1);
    }

    #[test]
    fn suspended_scene_preserves_dependent_command_order() {
        let mut coordinator = StrategicWindowing::new();
        let window_spec = singleton_spec(WindowKind::Encyclopedia, WindowOwner::StrategicRoot);
        coordinator.enqueue(WindowCommand::Open(window_spec));
        let first = coordinator.apply_pending();
        let id = opened_id(&first[0]);

        coordinator.enter_scene(WindowScene::Suspended);
        coordinator.enqueue(WindowCommand::Open(window_spec));
        coordinator.enqueue(WindowCommand::Close(id));
        assert!(coordinator.apply_pending().is_empty());

        coordinator.enter_scene(WindowScene::Galaxy);
        let applied = coordinator.apply_pending();
        assert_eq!(applied.len(), 2);
        assert!(applied.iter().all(|result| result.result.is_ok()));
        assert!(coordinator.begin_frame().paint_order().is_empty());
    }

    #[test]
    fn scene_membership_suspends_without_deleting_registered_content() {
        let mut coordinator = StrategicWindowing::new();
        coordinator.enqueue(WindowCommand::Open(singleton_spec(
            WindowKind::Encyclopedia,
            WindowOwner::StrategicRoot,
        )));
        let applied = coordinator.apply_pending();
        let id = opened_id(&applied[0]);
        assert!(coordinator.attach_content(id));
        let focused_control = FocusTarget {
            window: id,
            control: Some(ControlId(7)),
        };
        coordinator.enqueue(WindowCommand::Focus(focused_control));
        assert!(coordinator.apply_pending()[0].result.is_ok());
        coordinator.enter_scene(WindowScene::Suspended);
        assert!(!coordinator.begin_frame().is_visible(id));
        assert!(coordinator.has_content(id));
        coordinator.enter_scene(WindowScene::Galaxy);
        assert!(coordinator.begin_frame().is_visible(id));
        assert_eq!(coordinator.begin_frame().focused(), Some(focused_control));
    }

    #[test]
    fn context_subjects_are_monotonic_and_namespaced() {
        let mut coordinator = StrategicWindowing::new();
        let first = coordinator.next_context_subject().unwrap();
        let second = coordinator.next_context_subject().unwrap();
        assert_eq!(first.namespace, SUBJECT_COMMAND_CONTEXT);
        assert_eq!(second.value, first.value + 1);
        assert_ne!(
            WindowKey::contextual(WindowKind::Status, first),
            WindowKey::contextual(WindowKind::Status, second)
        );
    }

    #[test]
    fn tactical_windowing_phase_change_uses_same_battle_subject_distinct_kind() {
        use rebellion_render::tactical_view::TacticalWindowPhase;
        use rebellion_windowing::CloseReason;

        let mut coordinator = StrategicWindowing::new();
        coordinator.enter_scene(WindowScene::Tactical);

        let alert = coordinator
            .commit_tactical_phase(TacticalWindowPhase::Alert)
            .expect("Battle Alert opens");
        let alert_window = alert.window.expect("Alert owns a window");
        let alert_frame = coordinator.begin_frame();

        let results = coordinator
            .commit_tactical_phase(TacticalWindowPhase::Results)
            .expect("Results atomically replaces Alert");
        let results_window = results.window.expect("Results owns a window");

        assert_eq!(alert_window.subject, results_window.subject);
        assert_eq!(alert_window.kind, WindowKind::BattleAlert);
        assert_eq!(results_window.kind, WindowKind::BattleResults);
        assert_ne!(alert_window.id, results_window.id);
        assert_eq!(results.removed_content, [alert_window.id]);
        assert!(results.events.iter().any(|event| matches!(
            event,
            WindowEvent::Closed {
                window,
                reason: CloseReason::Replaced,
                ..
            } if *window == alert_window.id
        )));
        assert!(!coordinator.has_content(alert_window.id));
        assert!(coordinator.has_content(results_window.id));
        assert_eq!(alert_frame.active_modal(), Some(alert_window.id));
        assert_eq!(
            coordinator.begin_frame().active_modal(),
            Some(results_window.id)
        );
    }

    #[test]
    fn tactical_windowing_back_to_back_battles_allocate_distinct_subjects() {
        use rebellion_render::tactical_view::TacticalWindowPhase;

        let mut coordinator = StrategicWindowing::new();
        let old_results = coordinator
            .commit_scene_phase_for_battle(
                WindowScene::Tactical,
                Some(41),
                TacticalWindowPhase::Results,
            )
            .unwrap()
            .window
            .unwrap();
        let old_frame = coordinator.begin_frame();

        let new_alert = coordinator
            .commit_scene_phase_for_battle(
                WindowScene::Tactical,
                Some(42),
                TacticalWindowPhase::Alert,
            )
            .unwrap()
            .window
            .unwrap();

        assert_ne!(new_alert.subject, old_results.subject);
        assert_ne!(new_alert.id, old_results.id);
        assert_eq!(old_frame.active_modal(), Some(old_results.id));
        assert_eq!(coordinator.begin_frame().active_modal(), Some(new_alert.id));
    }

    #[test]
    fn tactical_windowing_result_destination_waits_for_the_galaxy_scene() {
        use rebellion_render::tactical_view::TacticalWindowPhase;

        let (world, system) = fleet_world();
        let state = SystemWindowState::default();
        let mut coordinator = StrategicWindowing::new();
        coordinator
            .commit_scene_phase_for_battle(
                WindowScene::Tactical,
                Some(7),
                TacticalWindowPhase::Results,
            )
            .unwrap();
        let prepared = state
            .prepare_open(
                &world,
                system,
                (85, 55),
                CockpitFaction::Alliance,
                fleet_layout(),
            )
            .unwrap();

        coordinator.enqueue_galaxy_system_open(prepared);
        assert!(coordinator.apply_pending().is_empty());

        coordinator
            .commit_scene_phase_for_battle(WindowScene::Galaxy, None, TacticalWindowPhase::Inactive)
            .unwrap();
        let applied = coordinator.apply_pending();
        let system_window = applied[0].system_content_id.unwrap();
        assert!(coordinator.begin_frame().is_visible(system_window));

        coordinator
            .commit_scene_phase_for_battle(
                WindowScene::Tactical,
                Some(8),
                TacticalWindowPhase::Combat,
            )
            .unwrap();
        assert!(!coordinator.begin_frame().is_visible(system_window));
    }

    #[test]
    fn tactical_windowing_combat_gap_retains_the_battle_subject() {
        use rebellion_render::tactical_view::TacticalWindowPhase;

        let mut coordinator = StrategicWindowing::new();
        let alert = coordinator
            .commit_scene_phase(WindowScene::Tactical, TacticalWindowPhase::Alert)
            .unwrap()
            .window
            .unwrap();

        let combat = coordinator
            .commit_scene_phase(WindowScene::Tactical, TacticalWindowPhase::Combat)
            .unwrap();
        assert_eq!(combat.window, None);

        let results = coordinator
            .commit_scene_phase(WindowScene::Tactical, TacticalWindowPhase::Results)
            .unwrap()
            .window
            .unwrap();
        assert_eq!(results.subject, alert.subject);
        assert_eq!(results.kind, WindowKind::BattleResults);
    }

    #[test]
    fn tactical_windowing_suspension_preserves_the_exact_modal_identity() {
        use rebellion_render::tactical_view::TacticalWindowPhase;

        let mut coordinator = StrategicWindowing::new();
        let alert = coordinator
            .commit_scene_phase(WindowScene::Tactical, TacticalWindowPhase::Alert)
            .unwrap()
            .window
            .unwrap();

        let suspended = coordinator
            .commit_scene_phase(WindowScene::Suspended, TacticalWindowPhase::Alert)
            .unwrap();
        assert_eq!(suspended.window, Some(alert));
        assert_eq!(coordinator.begin_frame().active_modal(), None);

        let restored = coordinator
            .commit_scene_phase(WindowScene::Tactical, TacticalWindowPhase::Alert)
            .unwrap();
        assert_eq!(restored.window, Some(alert));
        assert_eq!(coordinator.begin_frame().active_modal(), Some(alert.id));
    }

    #[test]
    fn tactical_windowing_dormant_galaxy_modal_does_not_block_battle_input() {
        use rebellion_render::tactical_view::TacticalWindowPhase;

        let mut coordinator = StrategicWindowing::new();
        coordinator.enqueue(WindowCommand::Open(singleton_spec(
            WindowKind::Encyclopedia,
            WindowOwner::StrategicRoot,
        )));
        let galaxy_modal = opened_id(&coordinator.apply_pending()[0]);
        assert!(coordinator.attach_content(galaxy_modal));

        let alert = coordinator
            .commit_scene_phase_for_battle(
                WindowScene::Tactical,
                Some(41),
                TacticalWindowPhase::Alert,
            )
            .expect("Battle Alert opens while the Encyclopedia is dormant")
            .window
            .expect("Battle Alert receives a manager identity");
        let battle_frame = coordinator.begin_frame();

        assert_eq!(alert.kind, WindowKind::BattleAlert);
        assert_eq!(battle_frame.active_modal(), Some(alert.id));
        assert_eq!(
            battle_frame.key(alert.id),
            Some(WindowKey::contextual(alert.kind, alert.subject))
        );
        assert_eq!(battle_frame.key(galaxy_modal), None);
        assert!(coordinator.has_content(galaxy_modal));
    }

    #[test]
    fn tactical_windowing_alert_blocks_command_center_outside_panel() {
        use rebellion_render::tactical_view::TacticalWindowPhase;

        let mut coordinator = StrategicWindowing::new();
        let committed = coordinator
            .commit_scene_phase(WindowScene::Tactical, TacticalWindowPhase::Alert)
            .expect("Battle Alert opens before the tactical snapshot");
        let alert = committed.window.expect("Alert owns a window").id;
        let frame = coordinator.begin_frame();

        assert_eq!(frame.active_modal(), Some(alert));
        assert_eq!(
            frame.pointer_route(&BTreeSet::new()).target,
            RouteTarget::BlockedBy(alert)
        );
        assert_eq!(frame.keyboard_route().bubble, BubblePolicy::StopAtWindow);
        assert_eq!(
            coordinator.tactical_content_reports(&frame, TacticalWindowPhase::Alert),
            [ContentReport {
                window: alert,
                status: ContentStatus::Ready,
            }]
        );
    }

    #[test]
    fn tactical_windowing_results_blocks_underlay_until_next_snapshot() {
        use rebellion_render::tactical_view::TacticalWindowPhase;
        use rebellion_windowing::CloseReason;

        let mut coordinator = StrategicWindowing::new();
        let results = coordinator
            .commit_scene_phase(WindowScene::Tactical, TacticalWindowPhase::Results)
            .expect("Battle Results opens")
            .window
            .expect("Results owns a window")
            .id;
        let closing_frame = coordinator.begin_frame();

        let closed = coordinator
            .commit_scene_phase(WindowScene::Tactical, TacticalWindowPhase::Inactive)
            .expect("the exact Results identity closes");

        assert_eq!(closed.removed_content, [results]);
        assert!(closed.events.iter().any(|event| matches!(
            event,
            WindowEvent::Closed {
                window,
                reason: CloseReason::Requested,
                ..
            } if *window == results
        )));
        assert_eq!(closing_frame.active_modal(), Some(results));
        assert_eq!(
            closing_frame.pointer_route(&BTreeSet::new()).target,
            RouteTarget::BlockedBy(results)
        );
        assert_eq!(coordinator.begin_frame().active_modal(), None);
    }

    #[test]
    fn tactical_windowing_new_campaign_discards_old_queued_confirm() {
        use rebellion_render::tactical_view::TacticalWindowPhase;

        let mut coordinator = StrategicWindowing::new();
        let old_results = coordinator
            .commit_scene_phase(WindowScene::Tactical, TacticalWindowPhase::Results)
            .unwrap()
            .window
            .unwrap();
        coordinator.enqueue(WindowCommand::Close(old_results.id));

        coordinator.reset_session();
        assert!(coordinator.apply_pending().is_empty());
        let new_alert = coordinator
            .commit_scene_phase(WindowScene::Tactical, TacticalWindowPhase::Alert)
            .unwrap()
            .window
            .unwrap();

        assert_ne!(new_alert.id, old_results.id);
        assert_ne!(new_alert.subject, old_results.subject);
        assert_eq!(coordinator.begin_frame().active_modal(), Some(new_alert.id));
    }

    #[test]
    fn tactical_windowing_scene_switch_cancels_capture_and_orphan_release() {
        use rebellion_render::tactical_view::TacticalWindowPhase;
        use rebellion_windowing::CaptureCancelReason;

        let mut coordinator = StrategicWindowing::new();
        coordinator.enqueue(WindowCommand::Open(contextual_spec(
            WindowKind::Fleet,
            SubjectId::new(1, 77),
            WindowOwner::StrategicRoot,
        )));
        let galaxy_window = opened_id(&coordinator.apply_pending()[0]);
        coordinator.enqueue(WindowCommand::CapturePointer(galaxy_window));
        assert!(coordinator.apply_pending()[0].result.is_ok());
        assert_eq!(
            coordinator.begin_frame().capture_owner(),
            Some(galaxy_window)
        );

        let committed = coordinator
            .commit_scene_phase(WindowScene::Tactical, TacticalWindowPhase::Combat)
            .unwrap();

        assert!(committed.events.contains(&WindowEvent::CaptureCancelled {
            window: galaxy_window,
            reason: CaptureCancelReason::SceneChanged,
        }));
        assert_eq!(coordinator.begin_frame().capture_owner(), None);
        assert_eq!(
            coordinator
                .begin_frame()
                .pointer_route(&BTreeSet::new())
                .target,
            RouteTarget::StrategicRoot
        );
    }

    #[test]
    fn tactical_windowing_return_restores_only_valid_galaxy_content() {
        use rebellion_render::tactical_view::TacticalWindowPhase;
        use rebellion_windowing::ContentStatus;

        let mut coordinator = StrategicWindowing::new();
        for subject in [11, 12] {
            coordinator.enqueue(WindowCommand::Open(contextual_spec(
                WindowKind::Fleet,
                SubjectId::new(1, subject),
                WindowOwner::StrategicRoot,
            )));
        }
        let opened = coordinator.apply_pending();
        let valid = opened_id(&opened[0]);
        let stale = opened_id(&opened[1]);
        assert!(coordinator.attach_content(valid));
        assert!(coordinator.attach_content(stale));
        coordinator
            .commit_scene_phase(WindowScene::Tactical, TacticalWindowPhase::Combat)
            .unwrap();

        coordinator
            .commit_scene_phase(WindowScene::Galaxy, TacticalWindowPhase::Inactive)
            .unwrap();
        let return_candidate = coordinator.begin_frame();
        let reconciliation = coordinator.reconcile(
            &return_candidate,
            &[
                ContentReport {
                    window: valid,
                    status: ContentStatus::Ready,
                },
                ContentReport {
                    window: stale,
                    status: ContentStatus::Absent,
                },
            ],
        );

        assert_eq!(reconciliation.removed, [stale]);
        assert!(return_candidate.paint_order().contains(&stale));
        let first_galaxy_frame = coordinator.begin_production_frame(
            ApplicationInputLocks::default(),
            LegacyInputBlockers::default(),
        );
        assert_eq!(first_galaxy_frame.snapshot().paint_order(), [valid]);
        assert_eq!(first_galaxy_frame.snapshot().active_modal(), None);
        assert!(coordinator.has_content(valid));
        assert!(!coordinator.has_content(stale));
    }

    #[test]
    fn tactical_windowing_unpaintable_results_remains_fail_closed() {
        use rebellion_render::tactical_view::TacticalWindowPhase;

        let mut coordinator = StrategicWindowing::new();
        let results = coordinator
            .commit_scene_phase(WindowScene::Tactical, TacticalWindowPhase::Results)
            .unwrap()
            .window
            .unwrap()
            .id;
        let frame = coordinator.begin_frame();

        let reports = coordinator.tactical_content_reports(&frame, TacticalWindowPhase::Combat);
        let reconciliation = coordinator.reconcile(&frame, &reports);

        assert_eq!(
            reports,
            [ContentReport {
                window: results,
                status: ContentStatus::Unpaintable,
            }]
        );
        assert_eq!(reconciliation.blocking_unpainted, [results]);
        assert!(reconciliation.removed.is_empty());
        assert_eq!(coordinator.begin_frame().active_modal(), Some(results));
    }

    #[test]
    fn windowing_legacy_bridge_blocks_raw_paths_under_migrated_modal() {
        let mut coordinator = StrategicWindowing::new();
        coordinator.enqueue(WindowCommand::Open(singleton_spec(
            WindowKind::Encyclopedia,
            WindowOwner::StrategicRoot,
        )));
        let modal = opened_id(&coordinator.apply_pending()[0]);

        let frame = coordinator.begin_production_frame(
            ApplicationInputLocks::default(),
            LegacyInputBlockers::default(),
        );

        assert_eq!(frame.snapshot().active_modal(), Some(modal));
        assert!(frame.migrated_modal_active());
        assert!(!frame.legacy_surface_input_allowed());
        assert!(!frame.legacy_owned_pointer_allowed());
        assert!(!frame.legacy_pointer_allowed());
        assert!(!frame.legacy_keyboard_allowed());
        assert!(!frame.root_shortcuts_allowed());
    }

    #[test]
    fn windowing_frame_boundary_keeps_close_event_lease() {
        let mut coordinator = StrategicWindowing::new();
        coordinator.enqueue(WindowCommand::Open(singleton_spec(
            WindowKind::Encyclopedia,
            WindowOwner::StrategicRoot,
        )));
        let modal = opened_id(&coordinator.apply_pending()[0]);
        let closing_frame = coordinator.begin_production_frame(
            ApplicationInputLocks::default(),
            LegacyInputBlockers::default(),
        );

        coordinator.enqueue(WindowCommand::Close(modal));
        assert!(coordinator.apply_pending()[0].result.is_ok());

        assert_eq!(closing_frame.snapshot().active_modal(), Some(modal));
        assert!(!closing_frame.legacy_pointer_allowed());
        assert!(!closing_frame.root_shortcuts_allowed());
        let next_frame = coordinator.begin_production_frame(
            ApplicationInputLocks::default(),
            LegacyInputBlockers::default(),
        );
        assert!(next_frame.legacy_pointer_allowed());
        assert!(next_frame.legacy_keyboard_allowed());
        assert!(next_frame.root_shortcuts_allowed());
    }

    #[test]
    fn windowing_no_family_production_frame_preserves_legacy_root() {
        let coordinator = StrategicWindowing::new();
        let frame = coordinator.begin_production_frame(
            ApplicationInputLocks::default(),
            LegacyInputBlockers::default(),
        );

        assert!(frame.snapshot().paint_order().is_empty());
        assert!(!frame.migrated_modal_active());
        assert!(frame.legacy_surface_input_allowed());
        assert!(frame.legacy_owned_pointer_allowed());
        assert!(frame.legacy_pointer_allowed());
        assert!(frame.legacy_keyboard_allowed());
        assert!(frame.root_shortcuts_allowed());
        assert!(!frame.migrated_input_held());
    }

    #[test]
    fn windowing_legacy_blocker_restricts_without_shadow_identity() {
        let coordinator = StrategicWindowing::new();
        let frame = coordinator.begin_production_frame(
            ApplicationInputLocks::default(),
            LegacyInputBlockers {
                transient_menu: true,
                ..LegacyInputBlockers::default()
            },
        );

        assert!(frame.snapshot().paint_order().is_empty());
        assert!(frame.migrated_input_held());
        assert!(frame.legacy_surface_input_allowed());
        assert!(frame.legacy_owned_pointer_allowed());
        assert!(!frame.legacy_pointer_allowed());
        assert!(!frame.legacy_keyboard_allowed());
        assert!(!frame.root_shortcuts_allowed());
    }

    #[test]
    fn managed_menu_keeps_its_own_input_lease_while_root_is_blocked() {
        let mut coordinator = StrategicWindowing::new();
        coordinator.enqueue(WindowCommand::Open(contextual_spec(
            WindowKind::System,
            SubjectId::new(rebellion_render::windowing_policy::SUBJECT_SYSTEM, 36),
            WindowOwner::StrategicRoot,
        )));
        let owner = opened_id(&coordinator.apply_pending()[0]);
        coordinator.enqueue(WindowCommand::Open(singleton_spec(
            WindowKind::Menu,
            WindowOwner::Window(owner),
        )));
        let menu = opened_id(&coordinator.apply_pending()[0]);

        let frame = coordinator.begin_production_frame(
            ApplicationInputLocks::default(),
            LegacyInputBlockers::default(),
        );
        assert!(!frame.legacy_owned_pointer_allowed());
        assert!(frame.managed_window_input_allowed(menu));

        let held = coordinator.begin_production_frame(
            ApplicationInputLocks::default(),
            LegacyInputBlockers {
                strategic_modal: true,
                ..LegacyInputBlockers::default()
            },
        );
        assert!(!held.managed_window_input_allowed(menu));
    }

    #[test]
    fn retained_windows_hold_input_under_application_locks_and_legacy_modals() {
        let coordinator = StrategicWindowing::new();
        let frame = |locks, legacy| coordinator.begin_production_frame(locks, legacy);
        let none = LegacyInputBlockers::default();

        assert!(frame(ApplicationInputLocks::default(), none).retained_window_input_allowed());
        for locks in [
            ApplicationInputLocks {
                briefing: true,
                ..ApplicationInputLocks::default()
            },
            ApplicationInputLocks {
                scene_transition: true,
                ..ApplicationInputLocks::default()
            },
        ] {
            assert!(!frame(locks, none).retained_window_input_allowed());
        }
        let modal = LegacyInputBlockers {
            strategic_modal: true,
            ..none
        };
        assert!(!frame(ApplicationInputLocks::default(), modal).retained_window_input_allowed());
        for keeps_own_rules in [
            LegacyInputBlockers {
                transient_menu: true,
                ..none
            },
            LegacyInputBlockers {
                root_gesture: true,
                ..none
            },
        ] {
            assert!(frame(ApplicationInputLocks::default(), keeps_own_rules)
                .retained_window_input_allowed());
        }
    }

    #[test]
    fn retained_windows_hold_input_under_a_migrated_modal() {
        let mut coordinator = StrategicWindowing::new();
        coordinator.enqueue(WindowCommand::Open(singleton_spec(
            WindowKind::Encyclopedia,
            WindowOwner::StrategicRoot,
        )));
        opened_id(&coordinator.apply_pending()[0]);
        let frame = coordinator.begin_production_frame(
            ApplicationInputLocks::default(),
            LegacyInputBlockers::default(),
        );

        assert!(!frame.retained_window_input_allowed());
    }

    #[test]
    fn a_focus_loss_is_reported_once_after_the_baseline() {
        let mut watch = FocusLossWatch::default();

        assert!(!watch.lost(3));
        assert!(!watch.lost(3));
        assert!(watch.lost(4));
        assert!(!watch.lost(4));
        assert!(watch.lost(0));
    }

    #[test]
    fn a_sampled_focus_state_counts_each_change_to_inactive_as_one_loss() {
        let mut focus = FocusStateCounter::default();
        let mut watch = FocusLossWatch::default();
        let mut cancels = Vec::new();

        for active in [false, true, true, false, false, true, false] {
            cancels.push(watch.lost(focus.sample(active)));
        }

        assert_eq!(
            cancels,
            [false, false, false, true, false, false, true],
            "a host inactive from the start loses nothing; each later \
             active-to-inactive change cancels once"
        );
    }

    #[test]
    fn windowing_legacy_restriction_latch_remembers_ephemeral_owner() {
        let frozen = LegacyInputBlockers::default();
        let mut observed_latch = LegacyInputRestrictionLatch::new(frozen);
        observed_latch.observe(LegacyInputBlockers {
            strategic_modal: true,
            ..LegacyInputBlockers::default()
        });
        observed_latch.observe(frozen);

        let mut explicit_latch = LegacyInputRestrictionLatch::new(frozen);
        explicit_latch.note_start();
        explicit_latch.observe(frozen);

        assert!(observed_latch.started());
        assert!(explicit_latch.started());
    }
}
