//! App-owned commit boundary, independent of production input collection and painting.
//!
//! Port contract: SW-05B. The kernel owns lifecycle; adapters own prepared/live
//! feature content and gameplay validation. There is no mutable-manager escape
//! hatch. SW-05G supplies the production caller after input dispatch completes.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use rebellion_render::windowing_policy::SUBJECT_COMMAND_CONTEXT;
use rebellion_windowing::{
    CaptureCancelReason, ContentReport, ReconcileReport, RoutingSnapshot, SubjectId, WindowCommand,
    WindowCommit, WindowError, WindowEvent, WindowId, WindowKind, WindowManager, WindowOwner,
    WindowSpec, WindowTransition,
};

/// Feature boundary. Preparation and validation must leave live content/gameplay
/// unchanged on error. Publication and receipt handling are infallible: all
/// fallible asset/subject work belongs in `prepare` or `revalidate`.
///
/// A receipt must cancel the matching drag payload before dropping its content;
/// the input router also consumes the returned receipts to quarantine release.
/// `execute` applies the already validated intent through the game's existing
/// authoritative action path; it must not substitute a mutable content index.
pub trait WindowContentAdapter {
    type OpenIntent;
    type Prepared;
    type Action;
    type Error;

    fn prepare(
        &mut self,
        spec: WindowSpec,
        intent: Self::OpenIntent,
    ) -> Result<Self::Prepared, Self::Error>;
    fn publish(&mut self, window: WindowId, prepared: Self::Prepared);
    fn event(&mut self, event: WindowEvent);
    fn revalidate(&mut self, window: WindowId, action: &Self::Action) -> Result<(), Self::Error>;
    fn execute(&mut self, action: Self::Action);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowScene {
    Galaxy,
    Tactical,
    Suspended,
}

/// Whole-frame application holds. Pause-alert rectangles stay in the renderer's
/// base-child hit regions; pausing simulation is not a strategic keyboard hold.
/// Consumers freeze these alongside their routing lease, and only tighten that
/// lease mid-frame. Releasing a hold takes effect on the next input lease.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ApplicationInputLocks {
    pub event_screen: bool,
    pub briefing: bool,
    pub fixture_setup: bool,
    pub scene_transition: bool,
}

impl ApplicationInputLocks {
    pub const fn blocks_strategic_input(self) -> bool {
        self.event_screen || self.briefing || self.fixture_setup || self.scene_transition
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CommitError<E> {
    Preparation(E),
    Revalidation(E),
    Window(WindowError),
    /// Raw Open/Replace must use the prepared-content entry points.
    ContentRequired,
    /// The subject is retained but cannot currently receive this confirmation.
    InactiveWindow(WindowId),
    InputHeld,
    /// Replacement cannot move a retained identity across application scenes.
    SceneMismatch(WindowId),
}

#[derive(Debug)]
pub struct AppliedWindowCommand<E> {
    pub command: WindowCommand,
    pub result: Result<WindowCommit, CommitError<E>>,
}

enum Pending<I, A> {
    Command(WindowCommand),
    Content { command: WindowCommand, intent: I },
    Confirm { window: WindowId, action: A },
}

pub struct StrategicWindowing<I, A> {
    manager: WindowManager,
    pending: VecDeque<Pending<I, A>>,
    scenes: BTreeMap<WindowId, WindowScene>,
    scene: WindowScene,
    locks: ApplicationInputLocks,
    // Never reset this allocator: old command contexts cannot alias a fresh campaign.
    next_context: Option<u64>,
}

impl<I, A> Default for StrategicWindowing<I, A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I, A> StrategicWindowing<I, A> {
    pub fn new() -> Self {
        Self {
            manager: WindowManager::new(),
            pending: VecDeque::new(),
            scenes: BTreeMap::new(),
            scene: WindowScene::Galaxy,
            locks: ApplicationInputLocks::default(),
            next_context: Some(1),
        }
    }

    pub fn begin_frame(&self) -> RoutingSnapshot {
        self.manager.begin_frame()
    }

    pub fn scene(&self) -> WindowScene {
        self.scene
    }

    pub fn input_locks(&self) -> ApplicationInputLocks {
        self.locks
    }

    pub fn next_context_subject(&mut self) -> Result<SubjectId, WindowError> {
        let value = self.next_context.ok_or(WindowError::IdExhausted)?;
        self.next_context = value.checked_add(1);
        Ok(SubjectId::new(SUBJECT_COMMAND_CONTEXT, value))
    }

    pub fn enqueue(&mut self, command: WindowCommand) {
        self.pending.push_back(Pending::Command(command));
    }

    pub fn enqueue_open(&mut self, spec: WindowSpec, intent: I) {
        self.pending.push_back(Pending::Content {
            command: WindowCommand::Open(spec),
            intent,
        });
    }

    pub fn enqueue_replace(&mut self, old: WindowId, new: WindowSpec, intent: I) {
        self.pending.push_back(Pending::Content {
            command: WindowCommand::Replace { old, new },
            intent,
        });
    }

    /// One-shot confirmation tied to an exact allocated identity. Closing that
    /// identity consumes it: a duplicate/stale confirmation cannot execute again.
    /// Validation occurs immediately before close; a rejection leaves it open.
    pub fn enqueue_confirm(&mut self, window: WindowId, action: A) {
        self.pending.push_back(Pending::Confirm { window, action });
    }

    /// Commit a deterministic batch after routing. Commands are individually
    /// atomic; off-scene content intents retain their relative order for later.
    /// Newly published windows are eligible only in the next frozen snapshot.
    pub fn apply_pending<T>(&mut self, adapter: &mut T) -> Vec<AppliedWindowCommand<T::Error>>
    where
        T: WindowContentAdapter<OpenIntent = I, Action = A>,
    {
        let mut applied = Vec::new();
        for pending in std::mem::take(&mut self.pending) {
            let command = match &pending {
                Pending::Command(command) | Pending::Content { command, .. } => *command,
                Pending::Confirm { window, .. } => WindowCommand::Close(*window),
            };
            if let Pending::Content { command, .. } = &pending {
                let spec = match command {
                    WindowCommand::Open(spec) => *spec,
                    WindowCommand::Replace { new, .. } => *new,
                    _ => unreachable!("content intents are created only by open/replace"),
                };
                if self.scene == WindowScene::Suspended || self.scene_for(spec) != self.scene {
                    self.pending.push_back(pending);
                    continue;
                }
            }
            let result = self.apply_one(pending, adapter);
            applied.push(AppliedWindowCommand { command, result });
        }
        applied
    }

    fn apply_one<T>(
        &mut self,
        pending: Pending<I, A>,
        adapter: &mut T,
    ) -> Result<WindowCommit, CommitError<T::Error>>
    where
        T: WindowContentAdapter<OpenIntent = I, Action = A>,
    {
        match pending {
            Pending::Command(WindowCommand::Open(_) | WindowCommand::Replace { .. }) => {
                Err(CommitError::ContentRequired)
            }
            Pending::Command(command) => {
                if self.locks.blocks_strategic_input()
                    && matches!(command, WindowCommand::CapturePointer(_))
                {
                    return Err(CommitError::InputHeld);
                }
                let commit = self.manager.apply(command).map_err(CommitError::Window)?;
                self.deliver(&commit.events, adapter);
                Ok(commit)
            }
            Pending::Content { command, intent } => {
                let spec = match command {
                    WindowCommand::Open(spec) => spec,
                    WindowCommand::Replace { old, new } => {
                        if self
                            .scenes
                            .get(&old)
                            .is_some_and(|scene| *scene != self.scene)
                        {
                            return Err(CommitError::SceneMismatch(old));
                        }
                        new
                    }
                    _ => unreachable!("content intents are created only by open/replace"),
                };
                // A compatible existing open preserves feature content. The
                // manager still validates owner/policy and applies raise/restore.
                let existing = matches!(command, WindowCommand::Open(_))
                    && self.manager.id_for(spec.key).is_some();
                let prepared = if existing {
                    None
                } else {
                    Some(
                        adapter
                            .prepare(spec, intent)
                            .map_err(CommitError::Preparation)?,
                    )
                };
                let commit = self.manager.apply(command).map_err(CommitError::Window)?;
                // Teardown old payloads before publishing. Focus realization
                // follows publication so the new ID already has a content store.
                for event in &commit.events {
                    if !matches!(event, WindowEvent::FocusChanged { .. }) {
                        self.deliver(&[*event], adapter);
                    }
                }
                if let Some(prepared) = prepared {
                    let id = match commit.transition {
                        WindowTransition::Opened { id } => id,
                        WindowTransition::Replaced { new, .. } => new,
                        _ => unreachable!("a fresh open/replace publishes one new identity"),
                    };
                    self.scenes.insert(id, self.scene_for(spec));
                    adapter.publish(id, prepared);
                }
                for event in &commit.events {
                    if matches!(event, WindowEvent::FocusChanged { .. }) {
                        adapter.event(*event);
                    }
                }
                Ok(commit)
            }
            Pending::Confirm { window, action } => {
                let frame = self.manager.begin_frame();
                if frame.key(window).is_none() {
                    return Err(CommitError::Window(WindowError::UnknownWindow(window)));
                }
                if !frame.allows_input(window) {
                    return Err(CommitError::InactiveWindow(window));
                }
                if self.locks.blocks_strategic_input() {
                    return Err(CommitError::InputHeld);
                }
                adapter
                    .revalidate(window, &action)
                    .map_err(CommitError::Revalidation)?;
                let commit = self
                    .manager
                    .apply(WindowCommand::Close(window))
                    .map_err(CommitError::Window)?;
                self.deliver(&commit.events, adapter);
                adapter.execute(action);
                Ok(commit)
            }
        }
    }

    /// Apply explicit availability reports; paint absence never implies teardown.
    pub fn reconcile<T: WindowContentAdapter>(
        &mut self,
        frame: &RoutingSnapshot,
        reports: &[ContentReport],
        adapter: &mut T,
    ) -> ReconcileReport {
        let report = self.manager.reconcile(frame, reports);
        self.deliver(&report.events, adapter);
        report
    }

    /// Pre-frame scene boundary. Retain dormant content and let the kernel repair
    /// capture/focus atomically before accepting input in the selected scene.
    pub fn enter_scene<T: WindowContentAdapter>(
        &mut self,
        scene: WindowScene,
        adapter: &mut T,
    ) -> Result<Vec<WindowEvent>, WindowError> {
        let active: BTreeSet<_> = self
            .scenes
            .iter()
            .filter_map(|(id, member_scene)| (*member_scene == scene).then_some(*id))
            .collect();
        let events = self.manager.set_active_windows(&active)?;
        self.scene = scene;
        self.deliver(&events, adapter);
        Ok(events)
    }

    pub fn set_input_locks<T: WindowContentAdapter>(
        &mut self,
        locks: ApplicationInputLocks,
        adapter: &mut T,
    ) -> Result<Vec<WindowEvent>, WindowError> {
        let events = if locks.blocks_strategic_input() && !self.locks.blocks_strategic_input() {
            self.manager
                .apply(WindowCommand::CancelCapture(
                    CaptureCancelReason::HoldStarted,
                ))?
                .events
        } else {
            Vec::new()
        };
        self.locks = locks;
        self.deliver(&events, adapter);
        Ok(events)
    }

    pub fn reset_session<T: WindowContentAdapter>(&mut self, adapter: &mut T) -> Vec<WindowEvent> {
        self.pending.clear();
        let events = self.manager.reset_session();
        self.deliver(&events, adapter);
        self.scenes.clear();
        self.scene = WindowScene::Galaxy;
        self.locks = ApplicationInputLocks::default();
        events
    }

    fn deliver<T: WindowContentAdapter>(&mut self, events: &[WindowEvent], adapter: &mut T) {
        for event in events {
            if let WindowEvent::Closed { window, .. } = event {
                self.scenes.remove(window);
            }
            adapter.event(*event);
        }
    }

    fn scene_for(&self, spec: WindowSpec) -> WindowScene {
        match spec.key.kind {
            WindowKind::BattleAlert | WindowKind::BattleResults => WindowScene::Tactical,
            WindowKind::Menu => match spec.owner {
                WindowOwner::StrategicRoot => WindowScene::Galaxy,
                WindowOwner::Window(owner) => self
                    .scenes
                    .get(&owner)
                    .copied()
                    .unwrap_or(WindowScene::Galaxy),
            },
            WindowKind::Sector
            | WindowKind::System
            | WindowKind::Fleet
            | WindowKind::Defenses
            | WindowKind::Missions
            | WindowKind::SystemFinder
            | WindowKind::FleetFinder
            | WindowKind::TroopFinder
            | WindowKind::PersonnelFinder
            | WindowKind::MessageIndex
            | WindowKind::Encyclopedia
            | WindowKind::Status
            | WindowKind::BuildSelection
            | WindowKind::MissionDialog
            | WindowKind::MoveConfirmation
            | WindowKind::ScrapConfirmation => WindowScene::Galaxy,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_render::windowing_policy::{contextual_spec, singleton_spec};
    use rebellion_windowing::{FocusTarget, WindowKind, WindowOwner};
    use std::collections::BTreeMap;

    #[derive(Default)]
    struct Content {
        live: BTreeMap<WindowId, &'static str>,
        events: Vec<WindowEvent>,
        log: Vec<&'static str>,
        valid: bool,
        effects: Vec<u64>,
    }

    impl WindowContentAdapter for Content {
        type OpenIntent = &'static str;
        type Prepared = &'static str;
        type Action = u64;
        type Error = &'static str;
        fn prepare(
            &mut self,
            _: WindowSpec,
            intent: &'static str,
        ) -> Result<&'static str, Self::Error> {
            self.log.push("prepare");
            if intent == "fail" {
                Err("preparation failed")
            } else {
                Ok(intent)
            }
        }
        fn publish(&mut self, id: WindowId, prepared: &'static str) {
            self.log.push("publish");
            assert!(self.live.insert(id, prepared).is_none());
        }
        fn event(&mut self, event: WindowEvent) {
            match event {
                WindowEvent::CaptureCancelled { window, .. } => {
                    assert!(self.live.contains_key(&window));
                    self.log.push("cancel");
                }
                WindowEvent::Closed { window, .. } => {
                    self.live.remove(&window);
                    self.log.push("drop");
                }
                WindowEvent::FocusChanged { current, .. } => {
                    if let Some(target) = current {
                        assert!(self.live.contains_key(&target.window));
                    }
                    self.log.push("focus");
                }
            }
            self.events.push(event);
        }
        fn revalidate(&mut self, _: WindowId, _: &u64) -> Result<(), Self::Error> {
            self.log.push("revalidate");
            if self.valid {
                Ok(())
            } else {
                Err("stale subject")
            }
        }
        fn execute(&mut self, action: u64) {
            self.log.push("execute");
            self.effects.push(action);
        }
    }

    fn detail(subject: u64) -> WindowSpec {
        contextual_spec(
            WindowKind::Fleet,
            SubjectId::new(2, subject),
            WindowOwner::StrategicRoot,
        )
    }
    fn modal(subject: u64) -> WindowSpec {
        contextual_spec(
            WindowKind::MoveConfirmation,
            SubjectId::new(3, subject),
            WindowOwner::StrategicRoot,
        )
    }
    fn open(
        coordinator: &mut StrategicWindowing<&'static str, u64>,
        content: &mut Content,
        spec: WindowSpec,
        intent: &'static str,
    ) -> WindowId {
        coordinator.enqueue_open(spec, intent);
        let applied = coordinator.apply_pending(content);
        match applied[0].result.as_ref().unwrap().transition {
            WindowTransition::Opened { id } => id,
            _ => panic!("expected new identity"),
        }
    }

    #[test]
    fn strategic_windowing_pending_commands_do_not_mutate_current_snapshot() {
        let mut coordinator = StrategicWindowing::new();
        let mut content = Content::default();
        let id = open(&mut coordinator, &mut content, modal(1), "modal");
        let frame = coordinator.begin_frame();
        coordinator.enqueue(WindowCommand::Close(id));
        assert_eq!(coordinator.begin_frame().active_modal(), Some(id));
        assert_eq!(content.live[&id], "modal");
        assert!(coordinator.apply_pending(&mut content)[0].result.is_ok());
        assert_eq!(frame.active_modal(), Some(id));
        assert!(!frame.pointer_allows_root());
        assert!(coordinator.begin_frame().pointer_allows_root());
        assert!(coordinator.apply_pending(&mut content).is_empty());
    }

    #[test]
    fn strategic_windowing_failed_open_preserves_previous_content() {
        let mut coordinator = StrategicWindowing::new();
        let mut content = Content::default();
        let old = open(&mut coordinator, &mut content, detail(1), "old");
        let frame = coordinator.begin_frame();
        coordinator.enqueue_replace(old, detail(2), "fail");
        assert!(matches!(
            coordinator.apply_pending(&mut content)[0].result,
            Err(CommitError::Preparation("preparation failed"))
        ));
        assert_eq!(content.live[&old], "old");
        assert_eq!(coordinator.begin_frame().paint_order(), frame.paint_order());
        let mut invalid = detail(2);
        invalid.key.subject = None;
        coordinator.enqueue_replace(old, invalid, "prepared");
        assert!(matches!(
            coordinator.apply_pending(&mut content)[0].result,
            Err(CommitError::Window(WindowError::SubjectMismatch(_)))
        ));
        assert_eq!(content.live.len(), 1);
        assert_eq!(content.live[&old], "old");
        let new = open(&mut coordinator, &mut content, detail(2), "new");
        assert_eq!(new.get(), old.get() + 1);
    }

    #[test]
    fn strategic_windowing_receipts_remove_only_matching_content_ids() {
        let mut coordinator = StrategicWindowing::new();
        let mut content = Content::default();
        let keep = open(&mut coordinator, &mut content, detail(1), "keep");
        let old = open(&mut coordinator, &mut content, detail(2), "old");
        let menu = open(
            &mut coordinator,
            &mut content,
            singleton_spec(WindowKind::Menu, WindowOwner::Window(old)),
            "child",
        );
        coordinator.enqueue(WindowCommand::CapturePointer(menu));
        assert!(coordinator.apply_pending(&mut content)[0].result.is_ok());
        content.log.clear();
        content.events.clear();
        coordinator.enqueue_replace(old, detail(3), "replacement");
        let applied = coordinator.apply_pending(&mut content);
        let WindowTransition::Replaced { new, .. } = applied[0].result.as_ref().unwrap().transition
        else {
            panic!()
        };
        assert_eq!(
            content.live,
            BTreeMap::from([(keep, "keep"), (new, "replacement")])
        );
        assert_eq!(
            content.log,
            ["prepare", "cancel", "drop", "drop", "publish", "focus"]
        );
        let closed: Vec<_> = content
            .events
            .iter()
            .filter_map(|e| match e {
                WindowEvent::Closed { window, .. } => Some(*window),
                _ => None,
            })
            .collect();
        assert_eq!(closed, [menu, old]);
    }

    #[test]
    fn strategic_windowing_confirm_commits_once_after_revalidation() {
        let mut coordinator = StrategicWindowing::new();
        let mut content = Content::default();
        let id = open(&mut coordinator, &mut content, modal(1), "modal");
        coordinator.enqueue_confirm(id, 7);
        assert!(matches!(
            coordinator.apply_pending(&mut content)[0].result,
            Err(CommitError::Revalidation("stale subject"))
        ));
        assert!(content.effects.is_empty());
        assert_eq!(content.live[&id], "modal");
        content.valid = true;
        content.log.clear();
        coordinator.enqueue_confirm(id, 7);
        coordinator.enqueue_confirm(id, 7);
        let results = coordinator.apply_pending(&mut content);
        assert!(results[0].result.is_ok());
        assert!(
            matches!(results[1].result, Err(CommitError::Window(WindowError::UnknownWindow(window))) if window == id)
        );
        assert_eq!(content.effects, [7]);
        assert_eq!(content.log, ["revalidate", "drop", "focus", "execute"]);
        assert!(coordinator.apply_pending(&mut content).is_empty());
    }

    #[test]
    fn strategic_windowing_reset_discards_pending_old_campaign_actions() {
        let mut coordinator = StrategicWindowing::new();
        let mut content = Content {
            valid: true,
            ..Content::default()
        };
        let id = open(&mut coordinator, &mut content, modal(1), "old");
        let old_context = coordinator.next_context_subject().unwrap();
        coordinator.enqueue_confirm(id, 7);
        coordinator.enqueue_open(detail(2), "queued");
        coordinator
            .enter_scene(WindowScene::Tactical, &mut content)
            .unwrap();
        coordinator.reset_session(&mut content);
        assert!(coordinator.apply_pending(&mut content).is_empty());
        assert!(content.live.is_empty());
        assert!(content.effects.is_empty());
        assert_eq!(coordinator.scene(), WindowScene::Galaxy);
        assert!(coordinator.next_context_subject().unwrap().value > old_context.value);
        let fresh = open(&mut coordinator, &mut content, modal(1), "fresh");
        assert!(fresh.get() > id.get());
        coordinator.enqueue_confirm(id, 7);
        assert!(coordinator.apply_pending(&mut content)[0].result.is_err());
        assert_eq!(content.live[&fresh], "fresh");
    }

    #[test]
    fn strategic_windowing_duplicate_open_preserves_content_without_preparation() {
        let mut coordinator = StrategicWindowing::new();
        let mut content = Content::default();
        let spec = singleton_spec(WindowKind::Encyclopedia, WindowOwner::StrategicRoot);
        let id = open(&mut coordinator, &mut content, spec, "original");
        content.log.clear();
        coordinator.enqueue_open(spec, "fail");
        assert!(
            matches!(coordinator.apply_pending(&mut content)[0].result.as_ref().unwrap().transition, WindowTransition::Existing { id: existing, .. } if existing == id)
        );
        assert!(content.log.is_empty());
        assert_eq!(content.live[&id], "original");
    }

    #[test]
    fn strategic_windowing_defers_off_scene_content_and_retains_dormant_modal() {
        let mut coordinator = StrategicWindowing::new();
        let mut content = Content::default();
        let galaxy = open(&mut coordinator, &mut content, modal(1), "galaxy");
        let battle = contextual_spec(
            WindowKind::BattleResults,
            SubjectId::new(4, 1),
            WindowOwner::StrategicRoot,
        );
        coordinator.enqueue_open(battle, "battle");
        assert!(coordinator.apply_pending(&mut content).is_empty());
        assert_eq!(content.live.len(), 1);
        coordinator
            .enter_scene(WindowScene::Tactical, &mut content)
            .unwrap();
        assert!(coordinator.begin_frame().pointer_allows_root());
        assert_eq!(coordinator.apply_pending(&mut content).len(), 1);
        let battle_id = coordinator.begin_frame().active_modal().unwrap();
        assert_ne!(battle_id, galaxy);
        assert_eq!(content.live[&galaxy], "galaxy");
        let menu = open(
            &mut coordinator,
            &mut content,
            singleton_spec(WindowKind::Menu, WindowOwner::Window(battle_id)),
            "menu",
        );
        assert!(coordinator.begin_frame().allows_input(menu));
        coordinator
            .enter_scene(WindowScene::Suspended, &mut content)
            .unwrap();
        assert!(coordinator.begin_frame().paint_order().is_empty());
        coordinator
            .enter_scene(WindowScene::Galaxy, &mut content)
            .unwrap();
        assert_eq!(coordinator.begin_frame().active_modal(), Some(galaxy));
        assert!(!coordinator.begin_frame().is_visible(menu));
        assert_eq!(content.live.len(), 3);
    }

    #[test]
    fn strategic_windowing_holds_cancel_capture_without_creating_a_modal() {
        let mut coordinator = StrategicWindowing::new();
        let mut content = Content::default();
        let id = open(&mut coordinator, &mut content, detail(1), "detail");
        coordinator.enqueue(WindowCommand::CapturePointer(id));
        coordinator.apply_pending(&mut content);
        let locks = ApplicationInputLocks {
            briefing: true,
            ..ApplicationInputLocks::default()
        };
        let events = coordinator.set_input_locks(locks, &mut content).unwrap();
        assert!(
            matches!(events[0], WindowEvent::CaptureCancelled { window, reason: CaptureCancelReason::HoldStarted } if window == id)
        );
        assert!(coordinator.input_locks().blocks_strategic_input());
        assert!(coordinator.begin_frame().active_modal().is_none());
        let frame_locks = coordinator.input_locks();
        coordinator
            .set_input_locks(ApplicationInputLocks::default(), &mut content)
            .unwrap();
        assert!(frame_locks.blocks_strategic_input());
        assert!(!coordinator.input_locks().blocks_strategic_input());
        for locks in [
            ApplicationInputLocks {
                event_screen: true,
                ..Default::default()
            },
            ApplicationInputLocks {
                fixture_setup: true,
                ..Default::default()
            },
            ApplicationInputLocks {
                scene_transition: true,
                ..Default::default()
            },
        ] {
            assert!(locks.blocks_strategic_input());
        }
        // Pause-alert geometry remains a spatial renderer child, never a whole-frame hold.
        assert!(!ApplicationInputLocks::default().blocks_strategic_input());
    }

    #[test]
    fn strategic_windowing_unchanged_unlocked_holds_preserve_capture() {
        let mut coordinator = StrategicWindowing::new();
        let mut content = Content::default();
        let id = open(&mut coordinator, &mut content, detail(1), "detail");
        coordinator.enqueue(WindowCommand::CapturePointer(id));
        assert!(coordinator.apply_pending(&mut content)[0].result.is_ok());
        content.events.clear();

        let events = coordinator
            .set_input_locks(ApplicationInputLocks::default(), &mut content)
            .unwrap();

        assert!(events.is_empty());
        assert!(content.events.is_empty());
        assert_eq!(coordinator.begin_frame().capture_owner(), Some(id));
    }

    #[test]
    fn strategic_windowing_reconciliation_delivers_exact_cleanup_and_retains_unpaintable_modal() {
        let mut coordinator = StrategicWindowing::new();
        let mut content = Content::default();
        let detail = open(&mut coordinator, &mut content, detail(1), "detail");
        let modal = open(&mut coordinator, &mut content, modal(1), "modal");
        let report = coordinator.reconcile(
            &coordinator.begin_frame(),
            &[
                ContentReport {
                    window: detail,
                    status: rebellion_windowing::ContentStatus::Absent,
                },
                ContentReport {
                    window: modal,
                    status: rebellion_windowing::ContentStatus::Unpaintable,
                },
            ],
            &mut content,
        );
        assert_eq!(report.removed, [detail]);
        assert_eq!(report.blocking_unpainted, [modal]);
        assert_eq!(content.live, BTreeMap::from([(modal, "modal")]));
        assert_eq!(coordinator.begin_frame().active_modal(), Some(modal));
    }

    #[test]
    fn strategic_windowing_context_allocation_is_namespaced_and_fails_closed_on_exhaustion() {
        let mut coordinator: StrategicWindowing<&str, u64> = StrategicWindowing::new();
        assert_eq!(
            coordinator.next_context_subject().unwrap(),
            SubjectId::new(3, 1)
        );
        coordinator.next_context = Some(u64::MAX);
        assert_eq!(
            coordinator.next_context_subject().unwrap(),
            SubjectId::new(3, u64::MAX)
        );
        assert_eq!(
            coordinator.next_context_subject(),
            Err(WindowError::IdExhausted)
        );
    }

    #[test]
    fn strategic_windowing_raw_open_cannot_bypass_content_preparation() {
        let mut coordinator: StrategicWindowing<&str, u64> = StrategicWindowing::new();
        let mut content = Content::default();
        coordinator.enqueue(WindowCommand::Open(detail(1)));
        assert!(matches!(
            coordinator.apply_pending(&mut content)[0].result,
            Err(CommitError::ContentRequired)
        ));
        assert!(coordinator.begin_frame().paint_order().is_empty());
        assert!(content.live.is_empty());
    }

    #[test]
    fn strategic_windowing_blocked_confirmation_never_revalidates_or_executes() {
        let mut coordinator = StrategicWindowing::new();
        let mut content = Content {
            valid: true,
            ..Content::default()
        };
        let detail = open(&mut coordinator, &mut content, detail(1), "detail");
        let modal = open(&mut coordinator, &mut content, modal(1), "modal");
        content.log.clear();
        coordinator.enqueue_confirm(detail, 7);
        assert!(
            matches!(coordinator.apply_pending(&mut content)[0].result, Err(CommitError::InactiveWindow(window)) if window == detail)
        );
        assert!(content.log.is_empty());
        assert!(content.effects.is_empty());
        assert_eq!(
            coordinator.begin_frame().focused(),
            Some(FocusTarget {
                window: modal,
                control: None
            })
        );
    }
}
