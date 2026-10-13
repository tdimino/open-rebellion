//! Application-owned strategic window coordinator and commit boundary.

use std::collections::BTreeSet;

use rebellion_render::windowing_policy::SUBJECT_COMMAND_CONTEXT;
use rebellion_windowing::{
    BubblePolicy, ContentReport, ReconcileReport, RouteTarget, RoutingSnapshot, SubjectId,
    WindowCommand, WindowCommit, WindowError, WindowEvent, WindowId, WindowTransition,
};

/// Result of applying one queued command at the dispatch commit boundary.
#[derive(Debug)]
pub struct AppliedWindowCommand {
    /// Original queued command.
    pub command: WindowCommand,
    /// Atomic kernel result.
    pub result: Result<WindowCommit, WindowError>,
    /// Matching attached content identities released by teardown receipts.
    pub removed_content: Vec<WindowId>,
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
    pending: Vec<WindowCommand>,
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
        self.pending.push(command);
    }

    /// Applies queued commands in insertion order after input and painting finish.
    pub fn apply_pending(&mut self) -> Vec<AppliedWindowCommand> {
        if self.scene == WindowScene::Suspended {
            return Vec::new();
        }
        let pending = std::mem::take(&mut self.pending);
        pending
            .into_iter()
            .map(|command| {
                let result = self.manager.apply(command);
                let mut removed_content = Vec::new();
                if let Ok(commit) = &result {
                    self.record_transition(&commit.transition, &commit.events);
                    removed_content = self.release_matching_content(&commit.events);
                }
                AppliedWindowCommand {
                    command,
                    result,
                    removed_content,
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
    use super::*;
    use rebellion_render::windowing_policy::{contextual_spec, singleton_spec};
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
