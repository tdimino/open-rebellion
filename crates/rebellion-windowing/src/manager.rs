use std::collections::{BTreeMap, BTreeSet};
use std::num::NonZeroU64;

use crate::{
    Cardinality, InputPolicy, OpenPlacement, StackBand, WindowId, WindowKey, WindowKind,
    WindowOwner, WindowSpec,
};

/// Manager command implemented by the current registry slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowCommand {
    /// Register or reopen a semantic window.
    Open(WindowSpec),
    /// Raise and focus an eligible window after pointer activation.
    Activate(WindowId),
    /// Raise and focus an eligible window explicitly.
    Raise(WindowId),
    /// Hide a history-capable detail while retaining its semantic identity/content.
    Minimize(WindowId),
    /// Restore a minimized detail to the top of its eligible band.
    Restore(WindowId),
    /// Move keyboard focus without changing visible order.
    Focus(crate::FocusTarget),
    /// Move keyboard focus to the persistent strategic root.
    FocusRoot,
    /// Give one eligible visible window temporary pointer ownership.
    CapturePointer(WindowId),
    /// Release temporary pointer ownership after physical completion.
    ReleasePointer(WindowId),
    /// Interrupt the active gesture for a typed coordinator/lifecycle reason.
    CancelCapture(CaptureCancelReason),
    /// Close a window and every actual lifecycle descendant.
    Close(WindowId),
}

/// Effect of reopening an existing compatible semantic key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExistingEffect {
    /// Identity, order, and all externally owned content remain unchanged.
    NoOp,
    /// The existing visible identity moved to the top of its eligible order.
    Raised,
    /// The minimized identity left history and returned to visible order.
    RestoredAndRaised,
}

/// Observable result of a successful manager command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowTransition {
    /// A new window identity was allocated.
    Opened { id: WindowId },
    /// A compatible existing identity handled the open request.
    Existing {
        /// Preserved identity.
        id: WindowId,
        /// Family-declared duplicate behavior.
        effect: ExistingEffect,
    },
    /// An eligible visible window was raised and focused.
    Raised {
        /// Preserved window identity.
        id: WindowId,
    },
    /// A detail entered history after cleanup and any oldest-entry eviction.
    Minimized {
        /// Retained detail identity.
        id: WindowId,
        /// Child-first identities removed by ordinary eviction teardown.
        evicted: Vec<WindowId>,
    },
    /// A minimized detail returned to visible order without allocation.
    Restored {
        /// Retained detail identity.
        id: WindowId,
    },
    /// Keyboard focus changed without changing visible order.
    Focused {
        /// Focus target before the command.
        previous: Option<crate::FocusTarget>,
        /// Focus target after the command.
        current: Option<crate::FocusTarget>,
    },
    /// Temporary pointer ownership changed.
    CaptureChanged {
        /// Capture owner before the command.
        previous: Option<WindowId>,
        /// Capture owner after the command.
        current: Option<WindowId>,
    },
    /// The command was valid but had no state change to commit.
    Unchanged {
        /// Identity whose state was already in the requested condition.
        subject: TransitionSubject,
        /// Typed explanation for the absent state change.
        reason: UnchangedReason,
    },
    /// A requested root and all of its actual descendants were removed.
    Closed {
        /// Identity named by the close command.
        root: WindowId,
        /// Deterministic child-first teardown receipt.
        closed: Vec<WindowId>,
        /// Eligible focus retained or selected after teardown.
        restored_focus: Option<crate::FocusTarget>,
    },
}

/// Identity associated with a successful no-op transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionSubject {
    /// The persistent strategic scene rather than an allocated window.
    StrategicRoot,
    /// One allocated window identity.
    Window(WindowId),
}

/// Why a valid command committed no state change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnchangedReason {
    /// The window was already the highest eligible peer.
    AlreadyTop,
    /// The window was already visible.
    AlreadyVisible,
    /// The window was already minimized.
    AlreadyMinimized,
    /// No pointer capture existed to release or cancel.
    NotCaptured,
    /// Focus already matched the requested target.
    FocusAlreadySet,
}

/// Why an in-progress pointer gesture was interrupted before physical release.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureCancelReason {
    /// A newly opened strategic modal made the prior owner ineligible.
    ModalOpened,
    /// The capture owner or one of its lifecycle owners closed.
    OwnerClosed,
    /// The capture owner or its lifecycle owner was minimized.
    Minimized,
    /// A coordinator moved the active input scene elsewhere.
    SceneChanged,
    /// An application-wide hold began.
    HoldStarted,
    /// The application lost operating-system focus.
    FocusLost,
    /// The current UI session was discarded.
    SessionReset,
}

/// Why registered content was removed from the manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloseReason {
    /// The named window was closed explicitly.
    Requested,
    /// An actual lifecycle owner closed.
    OwnerClosed,
    /// A later lifecycle slice atomically replaced the record.
    Replaced,
    /// The fixed-capacity history evicted the oldest record.
    Evicted,
    /// Reconciliation confirmed the content was removed.
    ContentRemoved,
    /// The complete UI session was discarded.
    SessionReset,
}

/// Ordered renderer-facing cleanup receipt emitted by a manager commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowEvent {
    /// Clear the named window's gesture payload before any later teardown.
    CaptureCancelled {
        /// Former capture owner.
        window: WindowId,
        /// Cause of the interruption.
        reason: CaptureCancelReason,
    },
    /// Drop the matching feature content after any capture cancellation.
    Closed {
        /// Removed session identity.
        window: WindowId,
        /// Semantic identity needed by content adapters.
        key: WindowKey,
        /// Cause of removal.
        reason: CloseReason,
    },
    /// Realize semantic focus only after teardown receipts are processed.
    FocusChanged {
        /// Focus before the transition.
        previous: Option<crate::FocusTarget>,
        /// Focus after the transition.
        current: Option<crate::FocusTarget>,
    },
}

/// Successful atomic manager commit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowCommit {
    /// State transition produced by the command.
    pub transition: WindowTransition,
    /// Ordered cleanup receipts to apply around the transition.
    pub events: Vec<WindowEvent>,
}

/// Classifies a broken registry or visible-order invariant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvariantKind {
    /// Registry and semantic-key index membership disagree.
    RegistryKeyMismatch,
    /// One identity occurs more than once in visible order.
    DuplicateOrder,
    /// A registered identity is absent from visible order, or vice versa.
    MissingVisibleOrder,
    /// Key shape, family policy, or family capacity is inconsistent.
    Cardinality,
    /// A window-owned record names an absent owner.
    OwnerMissing,
    /// Visible order does not respect stack-band or modal boundaries.
    BandOrder,
    /// A child is not above its registered owner.
    ChildBelowOwner,
    /// The current focus target is absent, hidden, or blocked by a modal.
    IneligibleFocus,
    /// The pointer capture owner is absent, hidden, or blocked by a modal.
    IneligibleCapture,
    /// More than twelve minimized detail identities are retained.
    HistoryOverflow,
    /// A minimized identity occurs more than once in history.
    DuplicateHistory,
    /// A history entry is also visible.
    VisibleInHistory,
    /// A registered hidden identity has no history entry.
    MinimizedWithoutHistory,
    /// A history entry is absent from the registry or cannot minimize.
    InvalidHistoryEntry,
}

/// Diagnostic result from validating manager-owned open/order state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvariantViolation {
    /// Broken invariant class.
    pub kind: InvariantKind,
    /// Identities directly involved in the violation.
    pub windows: Vec<WindowId>,
}

/// Renderer-independent strategic-window registry and visible order.
#[derive(Debug)]
pub struct WindowManager {
    registry: BTreeMap<WindowId, WindowRecord>,
    key_index: BTreeMap<WindowKey, WindowId>,
    visible_order: Vec<WindowId>,
    presentation_history: Vec<WindowId>,
    focused: Option<crate::FocusTarget>,
    previous_eligible: Option<WindowId>,
    saved_menu_focus: BTreeMap<WindowId, Option<crate::FocusTarget>>,
    capture_owner: Option<WindowId>,
    next_id: Option<NonZeroU64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WindowRecord {
    spec: WindowSpec,
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowManager {
    /// Fixed original rail bound, recovered from FUN_00428b40.
    pub const PRESENTATION_HISTORY_CAPACITY: usize = 12;

    /// Creates an empty manager for one UI session.
    #[must_use]
    pub fn new() -> Self {
        Self {
            registry: BTreeMap::new(),
            key_index: BTreeMap::new(),
            visible_order: Vec::new(),
            presentation_history: Vec::new(),
            focused: None,
            previous_eligible: None,
            saved_menu_focus: BTreeMap::new(),
            capture_owner: None,
            next_id: Some(NonZeroU64::MIN),
        }
    }

    /// Looks up the current session identity for a semantic key.
    #[must_use]
    pub fn id_for(&self, key: WindowKey) -> Option<WindowId> {
        self.key_index.get(&key).copied()
    }

    /// Returns the visible bottom-to-top window order.
    #[must_use]
    pub fn visible_order(&self) -> &[WindowId] {
        &self.visible_order
    }

    /// Returns minimized detail identities from oldest to newest; content stays external.
    #[must_use]
    pub fn presentation_history(&self) -> &[WindowId] {
        &self.presentation_history
    }

    /// Returns the current semantic keyboard-focus target, or the root.
    #[must_use]
    pub fn focused(&self) -> Option<crate::FocusTarget> {
        self.focused
    }

    /// Returns the window that temporarily owns pointer motion and release.
    #[must_use]
    pub fn capture_owner(&self) -> Option<WindowId> {
        self.capture_owner
    }

    /// Freezes an owned lease for one input dispatch, unaffected by later commits.
    #[must_use]
    pub fn begin_frame(&self) -> crate::snapshot::RoutingSnapshot {
        crate::snapshot::RoutingSnapshot::new(
            self.registry
                .iter()
                .map(|(id, record)| (*id, record.spec))
                .collect(),
            self.visible_order.clone(),
            self.presentation_history.clone(),
            self.focused,
            self.capture_owner,
        )
    }

    /// Discards the current UI session while retaining allocator progression.
    pub fn reset_session(&mut self) -> Vec<WindowEvent> {
        let focused_before = self.focused;
        let mut events = Vec::new();
        if let Some(window) = self.capture_owner.take() {
            events.push(WindowEvent::CaptureCancelled {
                window,
                reason: CaptureCancelReason::SessionReset,
            });
        }
        let mut closed = BTreeSet::new();
        for window in self
            .visible_order
            .iter()
            .rev()
            .chain(self.presentation_history.iter().rev())
            .copied()
        {
            // A hidden owned detail must be torn down before its visible owner.
            for member in self.close_order(window) {
                if !closed.insert(member) {
                    continue;
                }
                events.push(WindowEvent::Closed {
                    window: member,
                    key: self.registry[&member].spec.key,
                    reason: CloseReason::SessionReset,
                });
            }
        }

        self.registry.clear();
        self.key_index.clear();
        self.visible_order.clear();
        self.presentation_history.clear();
        self.focused = None;
        self.previous_eligible = None;
        self.saved_menu_focus.clear();
        if focused_before.is_some() {
            events.push(WindowEvent::FocusChanged {
                previous: focused_before,
                current: None,
            });
        }
        events
    }

    /// Validates the registry, semantic-key index, ownership, and visible order.
    pub fn validate(&self) -> Result<(), InvariantViolation> {
        for (id, record) in &self.registry {
            if self.key_index.get(&record.spec.key) != Some(id) {
                return Err(InvariantViolation {
                    kind: InvariantKind::RegistryKeyMismatch,
                    windows: vec![*id],
                });
            }
        }
        for (key, id) in &self.key_index {
            if self.registry.get(id).map(|record| record.spec.key) != Some(*key) {
                return Err(InvariantViolation {
                    kind: InvariantKind::RegistryKeyMismatch,
                    windows: vec![*id],
                });
            }
        }

        let mut seen = BTreeSet::new();
        for id in &self.visible_order {
            if !seen.insert(*id) {
                return Err(InvariantViolation {
                    kind: InvariantKind::DuplicateOrder,
                    windows: vec![*id],
                });
            }
            if !self.registry.contains_key(id) {
                return Err(InvariantViolation {
                    kind: InvariantKind::MissingVisibleOrder,
                    windows: vec![*id],
                });
            }
        }
        if self.presentation_history.len() > Self::PRESENTATION_HISTORY_CAPACITY {
            return Err(InvariantViolation {
                kind: InvariantKind::HistoryOverflow,
                windows: self.presentation_history.clone(),
            });
        }
        let mut history_seen = BTreeSet::new();
        for id in &self.presentation_history {
            if !history_seen.insert(*id) {
                return Err(InvariantViolation {
                    kind: InvariantKind::DuplicateHistory,
                    windows: vec![*id],
                });
            }
            if seen.contains(id) {
                return Err(InvariantViolation {
                    kind: InvariantKind::VisibleInHistory,
                    windows: vec![*id],
                });
            }
            if !self
                .registry
                .get(id)
                .is_some_and(|record| Self::can_minimize(record.spec))
            {
                return Err(InvariantViolation {
                    kind: InvariantKind::InvalidHistoryEntry,
                    windows: vec![*id],
                });
            }
        }
        if let Some(id) = self
            .registry
            .keys()
            .find(|id| !seen.contains(id) && !history_seen.contains(id))
        {
            return Err(InvariantViolation {
                kind: InvariantKind::MinimizedWithoutHistory,
                windows: vec![*id],
            });
        }

        let mut families = BTreeMap::new();
        for (id, record) in &self.registry {
            let has_valid_key_shape = match record.spec.policy.cardinality {
                Cardinality::Singleton => record.spec.key.subject.is_none(),
                Cardinality::Subject { .. } => record.spec.key.subject.is_some(),
            };
            if !has_valid_key_shape {
                return Err(InvariantViolation {
                    kind: InvariantKind::Cardinality,
                    windows: vec![*id],
                });
            }

            if let Some((family_id, family_policy, count)) = families.get_mut(&record.spec.key.kind)
            {
                if *family_policy != record.spec.policy {
                    return Err(InvariantViolation {
                        kind: InvariantKind::Cardinality,
                        windows: vec![*family_id, *id],
                    });
                }
                *count += 1;
            } else {
                families.insert(record.spec.key.kind, (*id, record.spec.policy, 1_usize));
            }
        }
        for (_kind, (id, policy, count)) in families {
            // A subjectless singleton has only one possible semantic key, so
            // the registry/key-index bijection already proves its bound.
            if let Cardinality::Subject {
                max_open: Some(limit),
            } = policy.cardinality
            {
                if count > limit {
                    return Err(InvariantViolation {
                        kind: InvariantKind::Cardinality,
                        windows: vec![id],
                    });
                }
            }
        }

        let positions: BTreeMap<_, _> = self
            .visible_order
            .iter()
            .enumerate()
            .map(|(position, id)| (*id, position))
            .collect();
        for (id, record) in &self.registry {
            if let WindowOwner::Window(owner) = record.spec.owner {
                if !self.registry.contains_key(&owner) {
                    return Err(InvariantViolation {
                        kind: InvariantKind::OwnerMissing,
                        windows: vec![*id, owner],
                    });
                }
                if let Some(position) = positions.get(id) {
                    if positions
                        .get(&owner)
                        .is_none_or(|owner_position| position <= owner_position)
                    {
                        return Err(InvariantViolation {
                            kind: InvariantKind::ChildBelowOwner,
                            windows: vec![*id, owner],
                        });
                    }
                }
            }
        }

        for pair in self.visible_order.windows(2) {
            if self.registry[&pair[0]].spec.policy.band > self.registry[&pair[1]].spec.policy.band {
                return Err(InvariantViolation {
                    kind: InvariantKind::BandOrder,
                    windows: pair.to_vec(),
                });
            }
        }

        if let (Some(lowest_modal), Some(active_modal)) = (
            self.visible_order
                .iter()
                .position(|id| self.registry[id].spec.policy.input == InputPolicy::StrategicModal),
            self.visible_order
                .iter()
                .rposition(|id| self.registry[id].spec.policy.input == InputPolicy::StrategicModal),
        ) {
            for id in &self.visible_order {
                let record = self.registry[id];
                if record.spec.policy.input == InputPolicy::StrategicModal {
                    continue;
                }
                let boundary = match record.spec.owner {
                    WindowOwner::StrategicRoot => lowest_modal,
                    WindowOwner::Window(owner) => {
                        if positions[&owner] >= active_modal {
                            continue;
                        }
                        active_modal
                    }
                };
                // `boundary` itself is a modal, while `id` is known non-modal.
                if self.visible_order[boundary..].contains(id) {
                    return Err(InvariantViolation {
                        kind: InvariantKind::BandOrder,
                        windows: vec![self.visible_order[boundary], *id],
                    });
                }
            }
        }

        if let Some(focused) = self.focused {
            if !self.is_focus_eligible(focused.window) {
                return Err(InvariantViolation {
                    kind: InvariantKind::IneligibleFocus,
                    windows: vec![focused.window],
                });
            }
        }
        if let Some(capture_owner) = self.capture_owner {
            if !self.is_focus_eligible(capture_owner) {
                return Err(InvariantViolation {
                    kind: InvariantKind::IneligibleCapture,
                    windows: vec![capture_owner],
                });
            }
        }

        Ok(())
    }

    /// Applies one command atomically.
    pub fn apply(&mut self, command: WindowCommand) -> Result<WindowCommit, WindowError> {
        match command {
            WindowCommand::Open(spec) => self.open(spec),
            WindowCommand::Activate(id) | WindowCommand::Raise(id) => self.raise_and_focus(id),
            WindowCommand::Minimize(id) => self.minimize(id),
            WindowCommand::Restore(id) => self.restore(id),
            WindowCommand::Focus(target) => self.focus(target),
            WindowCommand::FocusRoot => self.focus_root(),
            WindowCommand::CapturePointer(id) => self.capture_pointer(id),
            WindowCommand::ReleasePointer(id) => self.release_pointer(id),
            WindowCommand::CancelCapture(reason) => self.cancel_capture(reason),
            WindowCommand::Close(id) => self.close(id),
        }
    }

    fn open(&mut self, spec: WindowSpec) -> Result<WindowCommit, WindowError> {
        if let Some(id) = self.id_for(spec.key) {
            let existing = self.registry[&id].spec;
            if existing.owner != spec.owner {
                return Err(WindowError::DuplicateKeyWithDifferentOwner(spec.key));
            }
            if existing.policy != spec.policy {
                return Err(WindowError::DuplicateKeyWithDifferentPolicy(spec.key));
            }

            let focus_before = self.focused;
            let effect = match existing.policy.existing {
                crate::ExistingOpenPolicy::NoOp => ExistingEffect::NoOp,
                crate::ExistingOpenPolicy::RaiseRestore => {
                    if self.presentation_history.contains(&id) {
                        let mut commit = self.restore(id)?;
                        commit.transition = WindowTransition::Existing {
                            id,
                            effect: ExistingEffect::RestoredAndRaised,
                        };
                        return Ok(commit);
                    }
                    self.raise_visible(id);
                    if self.is_focus_eligible(id) {
                        self.set_focus(Some(crate::FocusTarget {
                            window: id,
                            control: None,
                        }));
                    }
                    ExistingEffect::Raised
                }
            };
            let mut events = Vec::new();
            if existing.policy.input == InputPolicy::StrategicModal {
                events.extend(self.cancel_capture_if_ineligible(CaptureCancelReason::ModalOpened));
            }
            if focus_before != self.focused {
                events.push(WindowEvent::FocusChanged {
                    previous: focus_before,
                    current: self.focused,
                });
            }
            return Ok(WindowCommit {
                transition: WindowTransition::Existing { id, effect },
                events,
            });
        }

        self.validate_new_spec(spec)?;
        let insertion = self.proposed_insertion_index(spec);
        let focus_before = self.focused;

        let raw_id = self.next_id.ok_or(WindowError::IdExhausted)?;
        let id = WindowId(raw_id);
        self.next_id = NonZeroU64::new(raw_id.get().wrapping_add(1));

        self.registry.insert(id, WindowRecord { spec });
        self.key_index.insert(spec.key, id);
        self.visible_order.insert(insertion, id);
        let mut events = Vec::new();
        if spec.key.kind == WindowKind::Menu {
            self.saved_menu_focus.insert(id, focus_before);
        }
        if spec.policy.input == InputPolicy::StrategicModal {
            events.extend(self.cancel_capture_if_ineligible(CaptureCancelReason::ModalOpened));
        }
        if spec.policy.focus_on_open == crate::FocusOnOpen::Window && self.is_focus_eligible(id) {
            self.set_focus(Some(crate::FocusTarget {
                window: id,
                control: None,
            }));
        } else if self.focused.map_or_else(
            || self.active_modal().is_some(),
            |target| !self.is_focus_eligible(target.window),
        ) {
            let fallback = self.topmost_eligible_focus();
            self.set_focus(fallback);
        }
        if focus_before != self.focused {
            events.push(WindowEvent::FocusChanged {
                previous: focus_before,
                current: self.focused,
            });
        }

        Ok(WindowCommit {
            transition: WindowTransition::Opened { id },
            events,
        })
    }

    fn focus(&mut self, target: crate::FocusTarget) -> Result<WindowCommit, WindowError> {
        self.validate_focus_target(target)?;
        let previous = self.focused;
        self.set_focus(Some(target));
        Ok(WindowCommit {
            transition: WindowTransition::Focused {
                previous,
                current: Some(target),
            },
            events: (previous != Some(target))
                .then_some(WindowEvent::FocusChanged {
                    previous,
                    current: Some(target),
                })
                .into_iter()
                .collect(),
        })
    }

    fn focus_root(&mut self) -> Result<WindowCommit, WindowError> {
        if let Some(modal) = self.active_modal() {
            return Err(WindowError::RootBlockedBy(modal));
        }
        let previous = self.focused;
        self.set_focus(None);
        Ok(WindowCommit {
            transition: WindowTransition::Focused {
                previous,
                current: None,
            },
            events: previous
                .map(|previous| WindowEvent::FocusChanged {
                    previous: Some(previous),
                    current: None,
                })
                .into_iter()
                .collect(),
        })
    }

    fn raise_and_focus(&mut self, id: WindowId) -> Result<WindowCommit, WindowError> {
        let target = crate::FocusTarget {
            window: id,
            control: None,
        };
        self.validate_focus_target(target)?;
        let previous = self.focused;
        self.raise_visible(id);
        self.set_focus(Some(target));
        Ok(WindowCommit {
            transition: WindowTransition::Raised { id },
            events: (previous != Some(target))
                .then_some(WindowEvent::FocusChanged {
                    previous,
                    current: Some(target),
                })
                .into_iter()
                .collect(),
        })
    }

    fn capture_pointer(&mut self, id: WindowId) -> Result<WindowCommit, WindowError> {
        if !self.registry.contains_key(&id) {
            return Err(WindowError::UnknownWindow(id));
        }
        if !self.is_focus_eligible(id) {
            return Err(WindowError::IneligibleCapture(id));
        }
        if let Some(owner) = self.capture_owner {
            if owner != id {
                return Err(WindowError::CaptureOwnedBy(owner));
            }
        }

        let previous = self.capture_owner;
        self.capture_owner = Some(id);
        Ok(WindowCommit {
            transition: WindowTransition::CaptureChanged {
                previous,
                current: Some(id),
            },
            events: Vec::new(),
        })
    }

    fn release_pointer(&mut self, id: WindowId) -> Result<WindowCommit, WindowError> {
        if !self.registry.contains_key(&id) {
            return Err(WindowError::UnknownWindow(id));
        }
        if let Some(owner) = self.capture_owner {
            if owner != id {
                return Err(WindowError::CaptureOwnedBy(owner));
            }
        } else {
            return Ok(WindowCommit {
                transition: WindowTransition::Unchanged {
                    subject: TransitionSubject::Window(id),
                    reason: UnchangedReason::NotCaptured,
                },
                events: Vec::new(),
            });
        }

        let previous = self.capture_owner;
        self.capture_owner = None;
        Ok(WindowCommit {
            transition: WindowTransition::CaptureChanged {
                previous,
                current: None,
            },
            events: Vec::new(),
        })
    }

    fn cancel_capture(&mut self, reason: CaptureCancelReason) -> Result<WindowCommit, WindowError> {
        let Some(previous) = self.capture_owner.take() else {
            return Ok(WindowCommit {
                transition: WindowTransition::Unchanged {
                    subject: TransitionSubject::StrategicRoot,
                    reason: UnchangedReason::NotCaptured,
                },
                events: Vec::new(),
            });
        };
        Ok(WindowCommit {
            transition: WindowTransition::CaptureChanged {
                previous: Some(previous),
                current: None,
            },
            events: vec![WindowEvent::CaptureCancelled {
                window: previous,
                reason,
            }],
        })
    }

    fn cancel_capture_if_ineligible(&mut self, reason: CaptureCancelReason) -> Option<WindowEvent> {
        let window = self.capture_owner?;
        if self.is_focus_eligible(window) {
            return None;
        }
        self.capture_owner = None;
        Some(WindowEvent::CaptureCancelled { window, reason })
    }

    fn close(&mut self, root: WindowId) -> Result<WindowCommit, WindowError> {
        self.close_with_reason(root, CloseReason::Requested)
    }

    fn close_with_reason(
        &mut self,
        root: WindowId,
        reason: CloseReason,
    ) -> Result<WindowCommit, WindowError> {
        if !self.registry.contains_key(&root) {
            return Err(WindowError::UnknownWindow(root));
        }
        let closing: BTreeSet<_> = self
            .registry
            .keys()
            .copied()
            .filter(|candidate| *candidate == root || self.is_descendant_of(*candidate, root))
            .collect();
        let closed = self.close_order(root);
        let focused_before = self.focused;
        let saved_menu_target = self.saved_menu_focus.get(&root).copied();
        let mut events = Vec::new();
        if self.capture_owner.is_some_and(|id| closing.contains(&id)) {
            let window = self
                .capture_owner
                .take()
                .expect("checked capture owner is present");
            events.push(WindowEvent::CaptureCancelled {
                window,
                reason: CaptureCancelReason::OwnerClosed,
            });
        }

        self.visible_order.retain(|id| !closing.contains(id));
        self.presentation_history.retain(|id| !closing.contains(id));
        for id in &closed {
            let record = self
                .registry
                .remove(id)
                .expect("close receipt IDs come from the registry");
            self.key_index.remove(&record.spec.key);
            self.saved_menu_focus.remove(id);
            events.push(WindowEvent::Closed {
                window: *id,
                key: record.spec.key,
                reason: if *id == root {
                    reason
                } else {
                    CloseReason::OwnerClosed
                },
            });
        }

        let current_focus_survives = focused_before.map_or_else(
            || self.active_modal().is_none(),
            |target| self.is_focus_eligible(target.window),
        );
        let eligible_saved_menu_target = saved_menu_target.filter(|target| match target {
            Some(target) => self.is_focus_eligible(target.window),
            None => self.active_modal().is_none(),
        });
        let restored_focus = if current_focus_survives {
            focused_before
        } else if let Some(saved) = eligible_saved_menu_target {
            saved
        } else {
            self.previous_eligible
                .filter(|id| self.is_focus_eligible(*id))
                .map(|window| crate::FocusTarget {
                    window,
                    control: None,
                })
                .or_else(|| self.topmost_eligible_focus())
        };
        self.focused = restored_focus;
        if focused_before != restored_focus {
            events.push(WindowEvent::FocusChanged {
                previous: focused_before,
                current: restored_focus,
            });
        }
        if self
            .previous_eligible
            .is_some_and(|id| !self.registry.contains_key(&id) || !self.visible_order.contains(&id))
        {
            self.previous_eligible = None;
        }

        Ok(WindowCommit {
            transition: WindowTransition::Closed {
                root,
                closed,
                restored_focus,
            },
            events,
        })
    }

    fn close_order(&self, root: WindowId) -> Vec<WindowId> {
        let mut ids: Vec<_> = self
            .visible_order
            .iter()
            .rev()
            .chain(self.presentation_history.iter().rev())
            .copied()
            .filter(|id| *id == root || self.is_descendant_of(*id, root))
            .collect();
        // Stable reverse-presentation order among equal-depth siblings; descendants first.
        ids.sort_by_key(|id| {
            let mut depth = 0;
            let mut current = *id;
            while current != root {
                let WindowOwner::Window(owner) = self.registry[&current].spec.owner else {
                    break;
                };
                depth += 1;
                current = owner;
            }
            std::cmp::Reverse(depth)
        });
        ids
    }

    fn can_minimize(spec: WindowSpec) -> bool {
        spec.policy.presentation == crate::PresentationPolicy::History
            && spec.policy.input == InputPolicy::Modeless
            && matches!(
                spec.key.kind,
                WindowKind::System
                    | WindowKind::Fleet
                    | WindowKind::Defenses
                    | WindowKind::Missions
            )
    }

    fn minimize(&mut self, id: WindowId) -> Result<WindowCommit, WindowError> {
        let record = self
            .registry
            .get(&id)
            .ok_or(WindowError::UnknownWindow(id))?;
        if !Self::can_minimize(record.spec) {
            return Err(WindowError::NotMinimizable(id));
        }
        if self.presentation_history.contains(&id) {
            return Ok(WindowCommit {
                transition: WindowTransition::Unchanged {
                    subject: TransitionSubject::Window(id),
                    reason: UnchangedReason::AlreadyMinimized,
                },
                events: Vec::new(),
            });
        }
        let descendants: Vec<_> = self
            .close_order(id)
            .into_iter()
            .filter(|child| *child != id)
            .collect();
        if descendants
            .iter()
            .any(|child| self.registry[child].spec.policy.input == InputPolicy::StrategicModal)
        {
            return Err(WindowError::OwnerHasModalChild(id));
        }
        // Non-transient child content is not implicitly discarded by minimizing its owner.
        // A child already in history may remain registered, but cannot restore beneath a hidden owner.
        if descendants.iter().any(|child| {
            self.visible_order.contains(child)
                && self.registry[child].spec.policy.band != StackBand::Transient
        }) {
            return Err(WindowError::InvalidPlacement);
        }
        let focused_before = self.focused;
        let mut events = Vec::new();
        if self
            .capture_owner
            .is_some_and(|owner| owner == id || descendants.contains(&owner))
        {
            events.extend(self.cancel_capture(CaptureCancelReason::Minimized)?.events);
        }
        for child in descendants {
            if self
                .registry
                .get(&child)
                .is_some_and(|record| record.spec.policy.band == StackBand::Transient)
            {
                events.extend(
                    self.close_with_reason(child, CloseReason::OwnerClosed)?
                        .events
                        .into_iter()
                        .filter(|event| !matches!(event, WindowEvent::FocusChanged { .. })),
                );
            }
        }
        self.visible_order.retain(|visible| *visible != id);
        self.presentation_history.push(id);
        let mut evicted = Vec::new();
        if self.presentation_history.len() > Self::PRESENTATION_HISTORY_CAPACITY {
            let oldest = self.presentation_history[0];
            let commit = self.close_with_reason(oldest, CloseReason::Evicted)?;
            if let WindowTransition::Closed { closed, .. } = commit.transition {
                evicted = closed;
            }
            events.extend(
                commit
                    .events
                    .into_iter()
                    .filter(|event| !matches!(event, WindowEvent::FocusChanged { .. })),
            );
        }
        if self
            .focused
            .is_some_and(|target| !self.is_focus_eligible(target.window))
        {
            self.focused = self
                .previous_eligible
                .filter(|previous| self.is_focus_eligible(*previous))
                .map(|window| crate::FocusTarget {
                    window,
                    control: None,
                })
                .or_else(|| self.topmost_eligible_focus());
        }
        if self
            .previous_eligible
            .is_some_and(|previous| !self.visible_order.contains(&previous))
        {
            self.previous_eligible = None;
        }
        if focused_before != self.focused {
            events.push(WindowEvent::FocusChanged {
                previous: focused_before,
                current: self.focused,
            });
        }
        Ok(WindowCommit {
            transition: WindowTransition::Minimized { id, evicted },
            events,
        })
    }

    fn restore(&mut self, id: WindowId) -> Result<WindowCommit, WindowError> {
        let spec = self
            .registry
            .get(&id)
            .ok_or(WindowError::UnknownWindow(id))?
            .spec;
        if !self.presentation_history.contains(&id) {
            return Ok(WindowCommit {
                transition: WindowTransition::Unchanged {
                    subject: TransitionSubject::Window(id),
                    reason: UnchangedReason::AlreadyVisible,
                },
                events: Vec::new(),
            });
        }
        if let WindowOwner::Window(owner) = spec.owner {
            if !self.visible_order.contains(&owner)
                || (spec.policy.band == StackBand::Transient && !self.is_focus_eligible(owner))
            {
                return Err(WindowError::InvalidPlacement);
            }
        }
        let focused_before = self.focused;
        self.presentation_history.retain(|entry| *entry != id);
        let insertion = self.proposed_insertion_index(spec);
        self.visible_order.insert(insertion, id);
        self.raise_visible(id);
        if self.is_focus_eligible(id) {
            self.set_focus(Some(crate::FocusTarget {
                window: id,
                control: None,
            }));
        }
        Ok(WindowCommit {
            transition: WindowTransition::Restored { id },
            events: (focused_before != self.focused)
                .then_some(WindowEvent::FocusChanged {
                    previous: focused_before,
                    current: self.focused,
                })
                .into_iter()
                .collect(),
        })
    }

    fn validate_focus_target(&self, target: crate::FocusTarget) -> Result<(), WindowError> {
        if !self.registry.contains_key(&target.window) {
            return Err(WindowError::UnknownWindow(target.window));
        }
        if !self.visible_order.contains(&target.window) {
            return Err(WindowError::IneligibleFocus(target));
        }
        if let Some(modal) = self.active_modal() {
            if target.window != modal && !self.is_descendant_of(target.window, modal) {
                return Err(WindowError::BlockedByModal {
                    window: target.window,
                    modal,
                });
            }
        }
        Ok(())
    }

    fn set_focus(&mut self, target: Option<crate::FocusTarget>) {
        if self.focused == target {
            return;
        }
        if let Some(previous) = self.focused {
            if target.is_none_or(|next| next.window != previous.window) {
                self.previous_eligible = Some(previous.window);
            }
        }
        self.focused = target;
    }

    fn active_modal(&self) -> Option<WindowId> {
        self.visible_order
            .iter()
            .rev()
            .copied()
            .find(|id| self.registry[id].spec.policy.input == InputPolicy::StrategicModal)
    }

    fn is_focus_eligible(&self, id: WindowId) -> bool {
        if !self.registry.contains_key(&id) {
            return false;
        }
        if !self.visible_order.contains(&id) {
            return false;
        }
        self.active_modal()
            .is_none_or(|modal| id == modal || self.is_descendant_of(id, modal))
    }

    fn topmost_eligible_focus(&self) -> Option<crate::FocusTarget> {
        self.visible_order
            .iter()
            .rev()
            .copied()
            .find(|id| self.is_focus_eligible(*id))
            .map(|window| crate::FocusTarget {
                window,
                control: None,
            })
    }

    fn validate_new_spec(&self, spec: WindowSpec) -> Result<(), WindowError> {
        match spec.policy.cardinality {
            Cardinality::Singleton if spec.key.subject.is_some() => {
                return Err(WindowError::SubjectMismatch(spec.key));
            }
            Cardinality::Subject { .. } if spec.key.subject.is_none() => {
                return Err(WindowError::SubjectMismatch(spec.key));
            }
            Cardinality::Singleton | Cardinality::Subject { .. } => {}
        }

        let uses_saved_menu_target =
            spec.policy.restoration == crate::FocusRestoration::SavedMenuTarget;
        if uses_saved_menu_target != (spec.key.kind == WindowKind::Menu) {
            return Err(WindowError::KindPolicyMismatch(spec.key.kind));
        }

        if self.registry.values().any(|record| {
            record.spec.key.kind == spec.key.kind && record.spec.policy != spec.policy
        }) {
            return Err(WindowError::KindPolicyMismatch(spec.key.kind));
        }

        if let WindowOwner::Window(owner) = spec.owner {
            if !self.registry.contains_key(&owner) {
                return Err(WindowError::UnknownOwner(owner));
            }
        }

        let is_transient = spec.policy.band == StackBand::Transient;
        let is_above_owner = spec.policy.placement == OpenPlacement::AboveOwner;
        if is_transient != is_above_owner
            || (is_above_owner && spec.owner == WindowOwner::StrategicRoot)
        {
            return Err(WindowError::InvalidPlacement);
        }
        if is_transient {
            let active_modal = self
                .visible_order
                .iter()
                .rposition(|id| self.registry[id].spec.policy.input == InputPolicy::StrategicModal);
            let owner_position = match spec.owner {
                WindowOwner::Window(owner) => self
                    .visible_order
                    .iter()
                    .position(|candidate| *candidate == owner),
                WindowOwner::StrategicRoot => None,
            };
            if active_modal.is_some_and(|modal| owner_position.is_none_or(|owner| owner < modal)) {
                return Err(WindowError::InvalidPlacement);
            }
        }
        if spec.policy.input == InputPolicy::StrategicModal {
            let modal_position = self.band_insertion_index(spec);
            for id in &self.visible_order[modal_position..] {
                let record = self.registry[id];
                if record.spec.policy.input == InputPolicy::StrategicModal {
                    continue;
                }
                let owner_is_eligible = match record.spec.owner {
                    WindowOwner::StrategicRoot => false,
                    WindowOwner::Window(owner) => self
                        .visible_order
                        .iter()
                        .position(|candidate| *candidate == owner)
                        .is_some_and(|owner_position| owner_position >= modal_position),
                };
                if !owner_is_eligible {
                    return Err(WindowError::InvalidPlacement);
                }
            }
        }

        if let Cardinality::Subject {
            max_open: Some(limit),
        } = spec.policy.cardinality
        {
            let open = self
                .registry
                .values()
                .filter(|record| record.spec.key.kind == spec.key.kind)
                .count();
            if open >= limit {
                return Err(WindowError::CardinalityExceeded(spec.key.kind));
            }
        }

        if let WindowOwner::Window(owner) = spec.owner {
            let Some(owner_position) = self
                .visible_order
                .iter()
                .position(|candidate| *candidate == owner)
            else {
                return Err(WindowError::InvalidPlacement);
            };
            if self.proposed_insertion_index(spec) <= owner_position {
                return Err(WindowError::InvalidPlacement);
            }
        }

        Ok(())
    }

    fn band_insertion_index(&self, spec: WindowSpec) -> usize {
        match spec.policy.placement {
            OpenPlacement::BottomOfBand => self
                .visible_order
                .iter()
                .position(|candidate| self.registry[candidate].spec.policy.band >= spec.policy.band)
                .unwrap_or(self.visible_order.len()),
            OpenPlacement::TopOfBand | OpenPlacement::AboveOwner => self
                .visible_order
                .iter()
                .position(|candidate| self.registry[candidate].spec.policy.band > spec.policy.band)
                .unwrap_or(self.visible_order.len()),
        }
    }

    fn proposed_insertion_index(&self, spec: WindowSpec) -> usize {
        self.constrain_to_modal_boundary(spec, self.band_insertion_index(spec))
    }

    fn raise_visible(&mut self, id: WindowId) {
        let subtree: Vec<_> = self
            .visible_order
            .iter()
            .copied()
            .filter(|candidate| *candidate == id || self.is_descendant_of(*candidate, id))
            .collect();
        let subtree_ids: BTreeSet<_> = subtree.iter().copied().collect();
        self.visible_order
            .retain(|candidate| !subtree_ids.contains(candidate));

        for member in subtree {
            let spec = self.registry[&member].spec;
            let band_top = self
                .visible_order
                .iter()
                .position(|candidate| self.registry[candidate].spec.policy.band > spec.policy.band)
                .unwrap_or(self.visible_order.len());
            let insertion = self.constrain_to_modal_boundary(spec, band_top);
            self.visible_order.insert(insertion, member);
        }
    }

    fn is_descendant_of(&self, candidate: WindowId, ancestor: WindowId) -> bool {
        let mut current = candidate;
        for _ in 0..self.registry.len() {
            let Some(record) = self.registry.get(&current) else {
                return false;
            };
            let WindowOwner::Window(owner) = record.spec.owner else {
                return false;
            };
            if owner == ancestor {
                return true;
            }
            current = owner;
        }
        false
    }

    fn constrain_to_modal_boundary(&self, spec: WindowSpec, insertion: usize) -> usize {
        if spec.policy.input == InputPolicy::StrategicModal {
            return insertion;
        }

        let Some(lowest_modal) = self.visible_order.iter().position(|candidate| {
            self.registry[candidate].spec.policy.input == InputPolicy::StrategicModal
        }) else {
            return insertion;
        };
        let active_modal = self
            .visible_order
            .iter()
            .rposition(|candidate| {
                self.registry[candidate].spec.policy.input == InputPolicy::StrategicModal
            })
            .unwrap_or(lowest_modal);

        let boundary = match spec.owner {
            WindowOwner::StrategicRoot => lowest_modal,
            WindowOwner::Window(owner) => {
                let owner_position = self
                    .visible_order
                    .iter()
                    .position(|candidate| *candidate == owner)
                    .unwrap_or(lowest_modal);
                if owner_position >= active_modal {
                    return insertion;
                }
                active_modal
            }
        };
        insertion.min(boundary)
    }
}

/// Typed manager rejection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowError {
    /// The session-local nonzero identity space is exhausted.
    IdExhausted,
    /// A command names an identity outside the current registry.
    UnknownWindow(WindowId),
    /// A window-owned spec names an identity outside the current registry.
    UnknownOwner(WindowId),
    /// Only history-capable modeless detail families can minimize.
    NotMinimizable(WindowId),
    /// Minimization would hide an owned strategic modal.
    OwnerHasModalChild(WindowId),
    /// The placement declaration does not agree with the owner or stack band.
    InvalidPlacement,
    /// The key shape disagrees with the family's declared cardinality.
    SubjectMismatch(WindowKey),
    /// The family already has its declared maximum number of subjects open.
    CardinalityExceeded(WindowKind),
    /// An existing semantic key was requested with a different lifetime owner.
    DuplicateKeyWithDifferentOwner(WindowKey),
    /// An existing semantic key was requested with a different behavior policy.
    DuplicateKeyWithDifferentPolicy(WindowKey),
    /// A registered family was requested with a different policy declaration.
    KindPolicyMismatch(WindowKind),
    /// A registered target is not currently visible and focusable.
    IneligibleFocus(crate::FocusTarget),
    /// A registered target is not currently visible and pointer-eligible.
    IneligibleCapture(WindowId),
    /// Another window currently owns the temporary pointer capture.
    CaptureOwnedBy(WindowId),
    /// A higher strategic modal blocks the requested window operation.
    BlockedByModal {
        /// Requested target.
        window: WindowId,
        /// Active modal boundary.
        modal: WindowId,
    },
    /// The strategic root cannot receive focus through an active modal.
    RootBlockedBy(WindowId),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ExistingOpenPolicy, FocusOnOpen, FocusRestoration, PresentationPolicy, SubjectId,
        WindowPolicy,
    };

    fn modeless_spec(kind: WindowKind, subject: u64) -> WindowSpec {
        WindowSpec {
            key: WindowKey::contextual(kind, SubjectId::new(2, subject)),
            owner: WindowOwner::StrategicRoot,
            policy: WindowPolicy {
                cardinality: Cardinality::Subject { max_open: None },
                band: StackBand::Normal,
                placement: OpenPlacement::TopOfBand,
                existing: ExistingOpenPolicy::RaiseRestore,
                input: InputPolicy::Modeless,
                presentation: PresentationPolicy::History,
                focus_on_open: FocusOnOpen::Window,
                restoration: FocusRestoration::PreviousEligible,
            },
        }
    }

    #[test]
    fn validate_reports_each_private_history_corruption() {
        // Port invariants: bounded, unique, registered hidden details, disjoint from order.
        for case in 0..6 {
            let mut manager = WindowManager::new();
            let id = match manager
                .apply(WindowCommand::Open(modeless_spec(WindowKind::System, 1)))
                .unwrap()
                .transition
            {
                WindowTransition::Opened { id } => id,
                other => panic!("expected open, got {other:?}"),
            };
            manager.apply(WindowCommand::Minimize(id)).unwrap();
            let kind = match case {
                0 => {
                    manager.presentation_history.push(id);
                    InvariantKind::DuplicateHistory
                }
                1 => {
                    manager.visible_order.push(id);
                    InvariantKind::VisibleInHistory
                }
                2 => {
                    manager.presentation_history = vec![id; 13];
                    InvariantKind::HistoryOverflow
                }
                3 => {
                    manager.presentation_history.clear();
                    InvariantKind::MinimizedWithoutHistory
                }
                4 => {
                    manager
                        .registry
                        .get_mut(&id)
                        .unwrap()
                        .spec
                        .policy
                        .presentation = PresentationPolicy::None;
                    InvariantKind::InvalidHistoryEntry
                }
                _ => {
                    manager.registry.remove(&id);
                    manager.key_index.clear();
                    InvariantKind::InvalidHistoryEntry
                }
            };
            assert_eq!(
                manager.validate().unwrap_err().kind,
                kind,
                "history corruption case {case}"
            );
        }
    }

    #[test]
    fn id_exhaustion_is_an_atomic_typed_error() {
        // Private-state setup reaches the otherwise impractical u64 exhaustion boundary.
        let mut manager = WindowManager::new();
        manager.next_id = NonZeroU64::new(u64::MAX);
        manager
            .apply(WindowCommand::Open(modeless_spec(WindowKind::System, 1)))
            .expect("the final nonzero ID remains allocatable");

        let registry_before = manager.registry.clone();
        let key_index_before = manager.key_index.clone();
        let order_before = manager.visible_order.clone();
        assert_eq!(
            manager.apply(WindowCommand::Open(modeless_spec(WindowKind::Fleet, 2))),
            Err(WindowError::IdExhausted)
        );

        assert_eq!(manager.registry, registry_before);
        assert_eq!(manager.key_index, key_index_before);
        assert_eq!(manager.visible_order, order_before);
        assert_eq!(manager.next_id, None);
    }

    #[test]
    fn close_clears_a_dead_global_previous_focus_candidate() {
        // Private-state assertion covers the contract that no dead focus reference
        // survives even when the current focus belongs to an unrelated window.
        let mut manager = WindowManager::new();
        let first = match manager
            .apply(WindowCommand::Open(modeless_spec(WindowKind::System, 1)))
            .expect("first window opens")
            .transition
        {
            WindowTransition::Opened { id } => id,
            other => panic!("expected first open, got {other:?}"),
        };
        let previous = match manager
            .apply(WindowCommand::Open(modeless_spec(WindowKind::Fleet, 2)))
            .expect("second window opens")
            .transition
        {
            WindowTransition::Opened { id } => id,
            other => panic!("expected second open, got {other:?}"),
        };
        manager
            .apply(WindowCommand::Focus(crate::FocusTarget {
                window: first,
                control: None,
            }))
            .expect("first window regains focus");
        assert_eq!(manager.previous_eligible, Some(previous));

        manager
            .apply(WindowCommand::Close(previous))
            .expect("unrelated previous candidate closes");

        assert_eq!(manager.focused.map(|target| target.window), Some(first));
        assert_eq!(manager.previous_eligible, None);
        assert_eq!(manager.validate(), Ok(()));
    }

    #[test]
    fn close_clears_a_hidden_global_previous_focus_candidate() {
        // Private-state setup represents the later hidden/minimized state. A
        // registered identity outside visible order is not a viable restore target.
        let mut manager = WindowManager::new();
        let current = match manager
            .apply(WindowCommand::Open(modeless_spec(WindowKind::System, 1)))
            .expect("first window opens")
            .transition
        {
            WindowTransition::Opened { id } => id,
            other => panic!("expected first open, got {other:?}"),
        };
        let hidden_previous = match manager
            .apply(WindowCommand::Open(modeless_spec(WindowKind::Fleet, 2)))
            .expect("second window opens")
            .transition
        {
            WindowTransition::Opened { id } => id,
            other => panic!("expected second open, got {other:?}"),
        };
        manager
            .apply(WindowCommand::Focus(crate::FocusTarget {
                window: current,
                control: None,
            }))
            .expect("first window regains focus");
        assert_eq!(manager.previous_eligible, Some(hidden_previous));
        manager.visible_order.retain(|id| *id != hidden_previous);

        manager
            .apply(WindowCommand::Close(current))
            .expect("visible current window closes");

        assert_eq!(manager.previous_eligible, None);
        assert_eq!(manager.focused, None);
    }

    #[test]
    fn focus_rejects_a_registered_but_hidden_private_state_target() {
        // Private-state setup represents the future minimized/hidden state without
        // implementing that later bead's public command.
        let mut manager = WindowManager::new();
        let hidden = match manager
            .apply(WindowCommand::Open(modeless_spec(WindowKind::System, 1)))
            .expect("window opens")
            .transition
        {
            WindowTransition::Opened { id } => id,
            other => panic!("expected open, got {other:?}"),
        };
        manager.visible_order.clear();
        let target = crate::FocusTarget {
            window: hidden,
            control: Some(crate::ControlId(1)),
        };

        assert_eq!(
            manager.focus(target),
            Err(WindowError::IneligibleFocus(target))
        );
        assert_eq!(manager.focused.map(|focus| focus.window), Some(hidden));
    }

    #[test]
    fn validate_reports_private_order_corruption() {
        let mut manager = WindowManager::new();
        let mut low = modeless_spec(WindowKind::Sector, 1);
        low.policy.band = StackBand::Low;
        low.policy.placement = OpenPlacement::BottomOfBand;
        let low_id = match manager
            .apply(WindowCommand::Open(low))
            .expect("low window opens")
            .transition
        {
            WindowTransition::Opened { id } => id,
            other => panic!("expected fresh low window, got {other:?}"),
        };
        let normal_id = match manager
            .apply(WindowCommand::Open(modeless_spec(WindowKind::System, 2)))
            .expect("normal window opens")
            .transition
        {
            WindowTransition::Opened { id } => id,
            other => panic!("expected fresh normal window, got {other:?}"),
        };

        manager.visible_order.push(normal_id);
        assert_eq!(
            manager.validate(),
            Err(InvariantViolation {
                kind: InvariantKind::DuplicateOrder,
                windows: vec![normal_id],
            })
        );

        manager.visible_order.pop();
        manager.visible_order.swap(0, 1);
        assert_eq!(
            manager.validate(),
            Err(InvariantViolation {
                kind: InvariantKind::BandOrder,
                windows: vec![normal_id, low_id],
            })
        );
    }

    #[test]
    fn validate_reports_family_policy_and_capacity_corruption() {
        let mut manager = WindowManager::new();
        let mut first = modeless_spec(WindowKind::Sector, 1);
        first.policy.cardinality = Cardinality::Subject { max_open: Some(2) };
        let mut second = modeless_spec(WindowKind::Sector, 2);
        second.policy.cardinality = Cardinality::Subject { max_open: Some(2) };
        let first_id = match manager
            .apply(WindowCommand::Open(first))
            .expect("first bounded subject opens")
            .transition
        {
            WindowTransition::Opened { id } => id,
            other => panic!("expected first bounded subject, got {other:?}"),
        };
        let second_id = match manager
            .apply(WindowCommand::Open(second))
            .expect("second bounded subject opens")
            .transition
        {
            WindowTransition::Opened { id } => id,
            other => panic!("expected second bounded subject, got {other:?}"),
        };
        assert_eq!(manager.validate(), Ok(()));

        for record in manager.registry.values_mut() {
            record.spec.policy.cardinality = Cardinality::Subject { max_open: Some(1) };
        }
        assert_eq!(
            manager.validate(),
            Err(InvariantViolation {
                kind: InvariantKind::Cardinality,
                windows: vec![first_id],
            })
        );

        for record in manager.registry.values_mut() {
            record.spec.policy.cardinality = Cardinality::Subject { max_open: Some(2) };
        }
        manager
            .registry
            .get_mut(&second_id)
            .unwrap()
            .spec
            .policy
            .presentation = PresentationPolicy::None;
        assert_eq!(
            manager.validate(),
            Err(InvariantViolation {
                kind: InvariantKind::Cardinality,
                windows: vec![first_id, second_id],
            })
        );
    }

    #[test]
    fn validate_reports_modeless_window_above_modal_boundary() {
        let mut manager = WindowManager::new();
        manager
            .apply(WindowCommand::Open(modeless_spec(WindowKind::System, 1)))
            .expect("System opens");

        let mut modal = modeless_spec(WindowKind::SystemFinder, 2);
        modal.policy.input = InputPolicy::StrategicModal;
        let modal_id = match manager
            .apply(WindowCommand::Open(modal))
            .expect("modal opens")
            .transition
        {
            WindowTransition::Opened { id } => id,
            other => panic!("expected modal, got {other:?}"),
        };
        let modeless_id = match manager
            .apply(WindowCommand::Open(modeless_spec(WindowKind::Fleet, 3)))
            .expect("background detail opens below modal")
            .transition
        {
            WindowTransition::Opened { id } => id,
            other => panic!("expected modeless detail, got {other:?}"),
        };

        let modal_position = manager
            .visible_order
            .iter()
            .position(|id| *id == modal_id)
            .unwrap();
        let modeless_position = manager
            .visible_order
            .iter()
            .position(|id| *id == modeless_id)
            .unwrap();
        manager
            .visible_order
            .swap(modal_position, modeless_position);
        assert_eq!(
            manager.validate(),
            Err(InvariantViolation {
                kind: InvariantKind::BandOrder,
                windows: vec![modal_id, modeless_id],
            })
        );
    }

    #[test]
    fn validate_reports_lower_modal_child_above_active_modal() {
        let mut manager = WindowManager::new();
        let mut lower = modeless_spec(WindowKind::SystemFinder, 1);
        lower.policy.input = InputPolicy::StrategicModal;
        let lower_id = match manager
            .apply(WindowCommand::Open(lower))
            .expect("lower modal opens")
            .transition
        {
            WindowTransition::Opened { id } => id,
            other => panic!("expected lower modal, got {other:?}"),
        };
        let mut active = modeless_spec(WindowKind::FleetFinder, 2);
        active.policy.input = InputPolicy::StrategicModal;
        let active_id = match manager
            .apply(WindowCommand::Open(active))
            .expect("active modal opens")
            .transition
        {
            WindowTransition::Opened { id } => id,
            other => panic!("expected active modal, got {other:?}"),
        };
        let mut child = modeless_spec(WindowKind::System, 3);
        child.owner = WindowOwner::Window(active_id);
        let child_id = match manager
            .apply(WindowCommand::Open(child))
            .expect("active modal child opens")
            .transition
        {
            WindowTransition::Opened { id } => id,
            other => panic!("expected active-modal child, got {other:?}"),
        };
        assert_eq!(manager.validate(), Ok(()));

        manager.registry.get_mut(&child_id).unwrap().spec.owner = WindowOwner::Window(lower_id);
        assert_eq!(
            manager.validate(),
            Err(InvariantViolation {
                kind: InvariantKind::BandOrder,
                windows: vec![active_id, child_id],
            })
        );
    }

    #[test]
    fn validate_reports_private_ineligible_capture_corruption() {
        let mut manager = WindowManager::new();
        let background = match manager
            .apply(WindowCommand::Open(modeless_spec(WindowKind::System, 1)))
            .expect("background window opens")
            .transition
        {
            WindowTransition::Opened { id } => id,
            other => panic!("expected background window, got {other:?}"),
        };
        let mut modal = modeless_spec(WindowKind::SystemFinder, 2);
        modal.policy.input = InputPolicy::StrategicModal;
        manager
            .apply(WindowCommand::Open(modal))
            .expect("modal opens");
        manager.capture_owner = Some(background);

        assert_eq!(
            manager.validate(),
            Err(InvariantViolation {
                kind: InvariantKind::IneligibleCapture,
                windows: vec![background],
            })
        );
    }
}
