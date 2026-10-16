//! Application-owned strategic window coordinator and commit boundary.

use std::collections::BTreeSet;

use rebellion_core::ids::SystemKey;
use rebellion_data::encyclopedia_presenter::EncyclopediaEntryIntent;
use rebellion_data::encyclopedia_session::EncyclopediaSession;
use rebellion_render::{
    fleet_finder::{PreparedFleetFinder, FLEET_FINDER_NAME_CONTROL},
    fleet_window::PreparedFleetWindow,
    system_window::PreparedSystemWindow,
    windowing_policy::SUBJECT_COMMAND_CONTEXT,
};
use rebellion_windowing::{
    BubblePolicy, ContentReport, ExistingEffect, FocusTarget, ReconcileReport, RouteTarget,
    RoutingSnapshot, SubjectId, WindowCommand, WindowCommit, WindowError, WindowEvent, WindowId,
    WindowTransition,
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
    /// System content prepared alongside this semantic open.
    pub prepared_system: Option<PreparedSystemWindow>,
    /// Identity whose attachment is valid for `prepared_system` after commit.
    pub system_content_id: Option<WindowId>,
    /// Fleet content prepared alongside this command, never published before
    /// the manager accepts the semantic identity.
    pub prepared_fleet: Option<PreparedFleetWindow>,
    /// Identity whose attachment is valid for `prepared_fleet` after commit.
    pub fleet_content_id: Option<WindowId>,
    /// Fleet Finder content prepared beside its singleton open.
    pub prepared_fleet_finder: Option<PreparedFleetFinder>,
    /// Identity whose attachment is valid for `prepared_fleet_finder`.
    pub fleet_finder_content_id: Option<WindowId>,
    /// Temporary rail-adapter payload committed only with System minimize.
    pub minimized_system: Option<(WindowId, SystemKey, (i16, i16))>,
    /// Exact temporary System rail projection retired after restore commit.
    pub restored_system: Option<WindowId>,
    /// Temporary rail-adapter payload committed only with Fleet minimize.
    pub minimized_fleet: Option<(WindowId, SystemKey, (i16, i16))>,
    /// Exact temporary rail projection retired only after the caller also
    /// commits the returned Fleet content restore.
    pub restored_fleet: Option<WindowId>,
}

/// Exact Encyclopedia identity and ordered manager receipts committed together.
#[derive(Debug)]
pub struct CommittedEncyclopediaOpen {
    /// Singleton identity accepted by the manager.
    pub window_id: WindowId,
    /// Capture/focus receipts emitted by the open and semantic-focus commits.
    pub events: Vec<WindowEvent>,
}

#[derive(Debug)]
struct QueuedWindowCommand {
    command: WindowCommand,
    prepared_system: Option<PreparedSystemWindow>,
    prepared_fleet: Option<PreparedFleetWindow>,
    prepared_fleet_finder: Option<PreparedFleetFinder>,
    minimized_system: Option<(WindowId, SystemKey, (i16, i16))>,
    restored_system: Option<WindowId>,
    minimized_fleet: Option<(WindowId, SystemKey, (i16, i16))>,
    restored_fleet: Option<WindowId>,
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
    /// Full-screen authored event playback.
    pub event_screen: bool,
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
        self.event_screen || self.briefing || self.fixture_setup || self.scene_transition
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
    scene: WindowScene,
    galaxy: BTreeSet<WindowId>,
    tactical: BTreeSet<WindowId>,
    attached_content: BTreeSet<WindowId>,
    next_context_subject: Option<u64>,
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
            scene: WindowScene::Galaxy,
            galaxy: BTreeSet::new(),
            tactical: BTreeSet::new(),
            attached_content: BTreeSet::new(),
            next_context_subject: Some(1),
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
            prepared_system: None,
            prepared_fleet: None,
            prepared_fleet_finder: None,
            minimized_system: None,
            restored_system: None,
            minimized_fleet: None,
            restored_fleet: None,
        });
    }

    /// Queues validated System content beside its semantic detail open.
    pub fn enqueue_system_open(&mut self, prepared: PreparedSystemWindow) {
        self.pending.push(QueuedWindowCommand {
            command: WindowCommand::Open(prepared.spec()),
            prepared_system: Some(prepared),
            prepared_fleet: None,
            prepared_fleet_finder: None,
            minimized_system: None,
            restored_system: None,
            minimized_fleet: None,
            restored_fleet: None,
        });
    }

    /// Queues a validated Fleet content candidate beside its semantic open.
    /// The candidate returns to the app only after the manager applies it.
    pub fn enqueue_fleet_open(&mut self, prepared: PreparedFleetWindow) {
        self.pending.push(QueuedWindowCommand {
            command: WindowCommand::Open(prepared.spec()),
            prepared_system: None,
            prepared_fleet: Some(prepared),
            prepared_fleet_finder: None,
            minimized_system: None,
            restored_system: None,
            minimized_fleet: None,
            restored_fleet: None,
        });
    }

    /// Queues inert Fleet Finder content beside its semantic singleton open.
    pub fn enqueue_fleet_finder_open(&mut self, prepared: PreparedFleetFinder) {
        self.pending.push(QueuedWindowCommand {
            command: WindowCommand::Open(prepared.spec()),
            prepared_system: None,
            prepared_fleet: None,
            prepared_fleet_finder: Some(prepared),
            minimized_system: None,
            restored_system: None,
            minimized_fleet: None,
            restored_fleet: None,
        });
    }

    /// Queues manager-owned System minimization with its temporary rail
    /// projection as one commit unit.
    pub fn enqueue_system_minimize(
        &mut self,
        window: WindowId,
        system: SystemKey,
        logical_position: (i16, i16),
    ) {
        self.pending.push(QueuedWindowCommand {
            command: WindowCommand::Minimize(window),
            prepared_system: None,
            prepared_fleet: None,
            prepared_fleet_finder: None,
            minimized_system: Some((window, system, logical_position)),
            restored_system: None,
            minimized_fleet: None,
            restored_fleet: None,
        });
    }

    /// Queues restoration of the exact manager-owned System identity retained
    /// by the temporary rail adapter.
    pub fn enqueue_system_restore(&mut self, window: WindowId) {
        self.pending.push(QueuedWindowCommand {
            command: WindowCommand::Restore(window),
            prepared_system: None,
            prepared_fleet: None,
            prepared_fleet_finder: None,
            minimized_system: None,
            restored_system: Some(window),
            minimized_fleet: None,
            restored_fleet: None,
        });
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

    /// Queues manager-owned Fleet minimization and its legacy rail projection
    /// as one commit unit. A rejected command never mutates the rail.
    pub fn enqueue_fleet_minimize(
        &mut self,
        window: WindowId,
        system: SystemKey,
        logical_position: (i16, i16),
    ) {
        self.pending.push(QueuedWindowCommand {
            command: WindowCommand::Minimize(window),
            prepared_system: None,
            prepared_fleet: None,
            prepared_fleet_finder: None,
            minimized_system: None,
            restored_system: None,
            minimized_fleet: Some((window, system, logical_position)),
            restored_fleet: None,
        });
    }

    /// Queues restoration of the exact manager-owned Fleet identity retained
    /// by the shared rail. No renderer publication is needed: minimized Fleet
    /// content remains attached while its manager record is hidden.
    pub fn enqueue_fleet_restore(&mut self, window: WindowId) {
        self.pending.push(QueuedWindowCommand {
            command: WindowCommand::Restore(window),
            prepared_system: None,
            prepared_fleet: None,
            prepared_fleet_finder: None,
            minimized_system: None,
            restored_system: None,
            minimized_fleet: None,
            restored_fleet: Some(window),
        });
    }

    /// Applies queued commands in insertion order after input and painting finish.
    pub fn apply_pending(&mut self) -> Vec<AppliedWindowCommand> {
        if self.scene == WindowScene::Suspended {
            return Vec::new();
        }
        let pending = std::mem::take(&mut self.pending);
        pending
            .into_iter()
            .map(|queued| {
                let QueuedWindowCommand {
                    command,
                    prepared_system,
                    prepared_fleet,
                    prepared_fleet_finder,
                    minimized_system,
                    restored_system: queued_restored_system,
                    minimized_fleet,
                    restored_fleet: queued_restored_fleet,
                } = queued;
                let result = self.manager.apply(command);
                let mut removed_content = Vec::new();
                let mut system_content_id = None;
                let mut fleet_content_id = None;
                let mut fleet_finder_content_id = None;
                let mut committed_minimized_system = None;
                let mut committed_minimized_fleet = None;
                let mut restored_system = None;
                let mut restored_fleet = None;
                if let Ok(commit) = &result {
                    self.record_transition(&commit.transition, &commit.events);
                    removed_content = self.release_matching_content(&commit.events);
                    if matches!(commit.transition, WindowTransition::Minimized { .. }) {
                        committed_minimized_system = minimized_system;
                        committed_minimized_fleet = minimized_fleet;
                    }
                    if queued_restored_system.is_some_and(|window| {
                        matches!(
                            commit.transition,
                            WindowTransition::Restored { id, .. } if id == window
                        )
                    }) {
                        restored_system = queued_restored_system;
                    }
                    if queued_restored_fleet.is_some_and(|window| {
                        matches!(
                            commit.transition,
                            WindowTransition::Restored { id, .. } if id == window
                        )
                    }) {
                        restored_fleet = queued_restored_fleet;
                    }
                    if prepared_system.is_some() {
                        system_content_id = match &commit.transition {
                            WindowTransition::Opened { id } => Some(*id),
                            WindowTransition::Existing { id, .. } => {
                                self.attached_content.contains(id).then_some(*id)
                            }
                            _ => None,
                        };
                        if system_content_id.is_some()
                            && matches!(
                                commit.transition,
                                WindowTransition::Existing {
                                    effect: ExistingEffect::RestoredAndRaised,
                                    ..
                                }
                            )
                        {
                            restored_system = system_content_id;
                        }
                    }
                    if prepared_fleet.is_some() {
                        fleet_content_id = match &commit.transition {
                            WindowTransition::Opened { id } => Some(*id),
                            WindowTransition::Existing { id, .. } => {
                                self.attached_content.contains(id).then_some(*id)
                            }
                            _ => None,
                        };
                        if fleet_content_id.is_some()
                            && matches!(
                                commit.transition,
                                WindowTransition::Existing {
                                    effect: ExistingEffect::RestoredAndRaised,
                                    ..
                                }
                            )
                        {
                            restored_fleet = fleet_content_id;
                        }
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
                            if let Ok(focus_commit) =
                                self.manager.apply(WindowCommand::Focus(focus))
                            {
                                self.record_transition(
                                    &focus_commit.transition,
                                    &focus_commit.events,
                                );
                            }
                        }
                    }
                }
                AppliedWindowCommand {
                    command,
                    result,
                    removed_content,
                    prepared_system,
                    system_content_id,
                    prepared_fleet,
                    fleet_content_id,
                    prepared_fleet_finder,
                    fleet_finder_content_id,
                    minimized_system: committed_minimized_system,
                    restored_system,
                    minimized_fleet: committed_minimized_fleet,
                    restored_fleet,
                }
            })
            .collect()
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

    /// Acknowledges System publication only for the exact manager identity.
    pub fn acknowledge_system_publication(&mut self, id: WindowId) -> bool {
        self.manager
            .begin_frame()
            .key(id)
            .is_some_and(|key| key.kind == rebellion_windowing::WindowKind::System)
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

    /// Compensates a failed renderer publication or rail eviction by closing
    /// the exact manager identity and returning the content teardown target.
    pub fn rollback_fleet_publication(&mut self, id: WindowId) -> Vec<WindowId> {
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
        self.galaxy.clear();
        self.tactical.clear();
        self.attached_content.clear();
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
    use rebellion_core::dat::{ExplorationStatus, SectorGroup};
    use rebellion_core::ids::DatId;
    use rebellion_core::world::{ControlKind, GameWorld, Sector, System};
    use rebellion_render::windowing_input::{
        FrameInputRouter, InputDisposition, InputEventId, InputRecipient,
    };
    use rebellion_render::windowing_policy::{contextual_spec, singleton_spec};
    use rebellion_render::{
        fleet_finder::{FleetFinderState, FLEET_FINDER_NAME_CONTROL},
        CockpitFaction, CockpitLayout, CockpitViewport, FleetWindowState, SystemWindowState,
    };
    use rebellion_windowing::{
        ControlId, FocusTarget, SubjectId, WindowKey, WindowKind, WindowOwner,
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

        coordinator.enqueue_system_minimize(id, system, (40, 50));
        let minimized = coordinator.apply_pending().remove(0);
        assert_eq!(minimized.minimized_system, Some((id, system, (40, 50))));
        assert_eq!(minimized.restored_system, None);
        assert!(coordinator.has_content(id));
        assert!(!coordinator.begin_frame().is_visible(id));
        if let Some((window, system, logical_position)) = minimized.minimized_system {
            state.minimize_system_window(window, system, logical_position);
        }
        assert_eq!(
            state.rail_entries().collect::<Vec<_>>(),
            [(system, "system")]
        );

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
        assert_eq!(restored.restored_system, Some(id));
        assert_eq!(
            state.commit_open(
                restored.prepared_system.take().unwrap(),
                &restored.result.as_ref().unwrap().transition,
            ),
            Some(id)
        );
        assert!(state.restore_system_window(id));
        assert!(coordinator.begin_frame().is_visible(id));

        coordinator.enqueue(WindowCommand::Close(id));
        let closed = coordinator.apply_pending().remove(0);
        assert_eq!(closed.removed_content, [id]);
        state.remove_content(&closed.removed_content);
        assert!(!state.contains_window(id));
        assert!(!coordinator.has_content(id));
    }

    #[test]
    fn fleet_commit_publishes_content_and_rail_only_after_manager_transition() {
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
        assert_eq!(opened.restored_fleet, None);
        assert!(coordinator.has_content(id));
        assert_eq!(state.window_id(system), Some(id));

        coordinator.enqueue_fleet_minimize(id, system, (40, 50));
        let minimized = coordinator.apply_pending().remove(0);
        assert!(matches!(
            minimized.result.as_ref().unwrap().transition,
            WindowTransition::Minimized {
                id: minimized_id, ..
            } if minimized_id == id
        ));
        assert_eq!(minimized.minimized_fleet, Some((id, system, (40, 50))));
        assert_eq!(minimized.restored_fleet, None);
        assert!(coordinator.has_content(id));
        assert!(!coordinator.begin_frame().is_visible(id));
        let mut rail = SystemWindowState::default();
        if let Some((window, system, logical_position)) = minimized.minimized_fleet {
            rail.minimize_fleet_window(window, system, logical_position);
        }
        assert_eq!(rail.rail_entries().collect::<Vec<_>>(), [(system, "fleet")]);

        coordinator.enqueue_fleet_restore(id);
        let restored = coordinator.apply_pending().remove(0);
        assert_eq!(restored.fleet_content_id, None);
        assert_eq!(restored.restored_fleet, Some(id));
        assert_eq!(rail.rail_count(), 1);
        assert!(matches!(
            restored.result.as_ref().unwrap().transition,
            WindowTransition::Restored { id: restored_id, .. } if restored_id == id
        ));
        assert!(rail.restore_fleet_window(restored.restored_fleet.unwrap()));
        assert_eq!(rail.rail_count(), 0);
        assert!(coordinator.begin_frame().is_visible(id));

        coordinator.enqueue(WindowCommand::Close(id));
        let closed = coordinator.apply_pending().remove(0);
        assert_eq!(closed.removed_content, [id]);
        assert_eq!(closed.restored_fleet, None);
        state.remove_content(&closed.removed_content);
        assert!(!coordinator.has_content(id));
        assert_eq!(state.window_count(), 0);

        coordinator.enqueue_fleet_minimize(id, system, (40, 50));
        let rejected = coordinator.apply_pending().remove(0);
        assert!(rejected.result.is_err());
        assert_eq!(rejected.minimized_fleet, None);
        assert_eq!(rejected.restored_fleet, None);
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
    fn retained_windows_hold_input_under_application_locks_and_legacy_modals() {
        let coordinator = StrategicWindowing::new();
        let frame = |locks, legacy| coordinator.begin_production_frame(locks, legacy);
        let none = LegacyInputBlockers::default();

        assert!(frame(ApplicationInputLocks::default(), none).retained_window_input_allowed());
        for locks in [
            ApplicationInputLocks {
                event_screen: true,
                ..ApplicationInputLocks::default()
            },
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
