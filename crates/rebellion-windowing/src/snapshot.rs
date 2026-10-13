use std::collections::{BTreeMap, BTreeSet};

use crate::{
    FocusTarget, InputPolicy, WindowEvent, WindowId, WindowKey, WindowOwner, WindowPolicy,
};

/// One semantic recipient selected for an input event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteTarget {
    /// The persistent strategic cockpit or map.
    StrategicRoot,
    /// One window or one of its focused controls.
    Window(FocusTarget),
    /// A modal or dismissible window consumes the event without a content hit.
    BlockedBy(WindowId),
}

/// Whether an unhandled keyboard event may continue to the strategic root.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BubblePolicy {
    /// A modeless target may pass an unhandled key to the root once.
    ToRoot,
    /// A modal or transient boundary stops propagation at the window.
    StopAtWindow,
}

/// Immutable keyboard lease for one routing snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyboardRoute {
    /// First semantic recipient.
    pub target: RouteTarget,
    /// Propagation boundary after the recipient reports unhandled input.
    pub bubble: BubblePolicy,
}

/// Immutable pointer lease for one event and hit set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointerRoute {
    /// Sole semantic recipient.
    pub target: RouteTarget,
}

/// Explicit feature-content state reported after painting a snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentStatus {
    /// Content exists and can be painted.
    Ready,
    /// Content exists but could not be painted in this frame.
    Unpaintable,
    /// Feature content was explicitly removed.
    Absent,
    /// Content is intentionally dormant because its scene is suspended.
    Suspended,
}

/// Reconciliation report for one snapshot member.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContentReport {
    /// Session identity being reported.
    pub window: WindowId,
    /// Explicit content state, independent of paint receipts.
    pub status: ContentStatus,
}

/// Result of conservative manager/content reconciliation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ReconcileReport {
    /// Identities removed after explicit modeless absence.
    pub removed: Vec<WindowId>,
    /// Modal identities retained as fail-closed blockers.
    pub blocking_unpainted: Vec<WindowId>,
    /// Unknown, duplicate, contradictory, or missing report identities.
    pub invalid_reports: Vec<WindowId>,
    /// Ordered lifecycle receipts for removed content.
    pub events: Vec<WindowEvent>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SnapshotRecord {
    pub key: WindowKey,
    pub owner: WindowOwner,
    pub policy: WindowPolicy,
}

/// Owned, immutable strategic-window routing state for one dispatch frame.
#[derive(Debug, Clone)]
pub struct RoutingSnapshot {
    manager_id: u64,
    records: BTreeMap<WindowId, SnapshotRecord>,
    members: BTreeSet<WindowId>,
    paint_order: Vec<WindowId>,
    presentation_history: Vec<WindowId>,
    focused: Option<FocusTarget>,
    capture_owner: Option<WindowId>,
}

impl RoutingSnapshot {
    pub(crate) fn new(
        manager_id: u64,
        records: BTreeMap<WindowId, SnapshotRecord>,
        visible_order: &[WindowId],
        presentation_history: &[WindowId],
        active: &BTreeSet<WindowId>,
        focused: Option<FocusTarget>,
        capture_owner: Option<WindowId>,
    ) -> Self {
        let paint_order = visible_order
            .iter()
            .copied()
            .filter(|id| active.contains(id))
            .collect();
        let presentation_history = presentation_history
            .iter()
            .copied()
            .filter(|id| active.contains(id))
            .collect();
        let focused = focused.filter(|target| active.contains(&target.window));
        let capture_owner = capture_owner.filter(|id| active.contains(id));
        Self {
            manager_id,
            records,
            members: active.clone(),
            paint_order,
            presentation_history,
            focused,
            capture_owner,
        }
    }

    /// Active visible identities in deterministic bottom-to-top order.
    #[must_use]
    pub fn paint_order(&self) -> &[WindowId] {
        &self.paint_order
    }

    /// Active minimized identities in oldest-to-newest order.
    #[must_use]
    pub fn presentation_history(&self) -> &[WindowId] {
        &self.presentation_history
    }

    /// Semantic key captured for an identity in this snapshot.
    #[must_use]
    pub fn key(&self, id: WindowId) -> Option<WindowKey> {
        self.members
            .contains(&id)
            .then(|| self.records.get(&id).map(|record| record.key))
            .flatten()
    }

    /// Lifecycle owner captured for an identity in this snapshot.
    #[must_use]
    pub fn owner(&self, id: WindowId) -> Option<WindowOwner> {
        self.members
            .contains(&id)
            .then(|| self.records.get(&id).map(|record| record.owner))
            .flatten()
    }

    /// Semantic keyboard focus captured for this frame.
    #[must_use]
    pub const fn focused(&self) -> Option<FocusTarget> {
        self.focused
    }

    /// Pointer-capture owner captured for this frame.
    #[must_use]
    pub const fn capture_owner(&self) -> Option<WindowId> {
        self.capture_owner
    }

    /// Highest active strategic-modal identity.
    #[must_use]
    pub fn active_modal(&self) -> Option<WindowId> {
        self.paint_order
            .iter()
            .rev()
            .copied()
            .find(|id| self.records[id].policy.input == InputPolicy::StrategicModal)
    }

    /// Whether an identity is active and visible in this snapshot.
    #[must_use]
    pub fn is_visible(&self, id: WindowId) -> bool {
        self.paint_order.contains(&id)
    }

    /// Whether modal ancestry admits input to this visible identity.
    #[must_use]
    pub fn allows_input(&self, id: WindowId) -> bool {
        self.is_visible(id)
            && self
                .active_modal()
                .is_none_or(|modal| id == modal || self.is_descendant_of(id, modal))
    }

    /// Keyboard owner and propagation boundary frozen for this frame.
    #[must_use]
    pub fn keyboard_route(&self) -> KeyboardRoute {
        let modal = self.active_modal();
        let target = self
            .focused
            .filter(|focused| self.allows_input(focused.window))
            .map(RouteTarget::Window)
            .or_else(|| {
                modal.map(|window| {
                    RouteTarget::Window(FocusTarget {
                        window,
                        control: None,
                    })
                })
            })
            .unwrap_or(RouteTarget::StrategicRoot);
        let bubble = match target {
            RouteTarget::StrategicRoot => BubblePolicy::ToRoot,
            RouteTarget::BlockedBy(_) => BubblePolicy::StopAtWindow,
            RouteTarget::Window(target) => {
                let input = self.records[&target.window].policy.input;
                if modal.is_some()
                    || matches!(
                        input,
                        InputPolicy::StrategicModal | InputPolicy::DismissOnOutsidePress
                    )
                {
                    BubblePolicy::StopAtWindow
                } else {
                    BubblePolicy::ToRoot
                }
            }
        };
        KeyboardRoute { target, bubble }
    }

    /// Selects exactly one pointer owner from current event-local hits.
    #[must_use]
    pub fn pointer_route(&self, hit_windows: &BTreeSet<WindowId>) -> PointerRoute {
        if let Some(capture) = self.capture_owner.filter(|id| self.allows_input(*id)) {
            return PointerRoute {
                target: RouteTarget::Window(FocusTarget {
                    window: capture,
                    control: None,
                }),
            };
        }

        if let Some(menu) = self.paint_order.iter().rev().copied().find(|id| {
            self.records[id].policy.input == InputPolicy::DismissOnOutsidePress
                && self.allows_input(*id)
        }) {
            let target = if hit_windows.contains(&menu) {
                RouteTarget::Window(FocusTarget {
                    window: menu,
                    control: None,
                })
            } else {
                RouteTarget::BlockedBy(menu)
            };
            return PointerRoute { target };
        }

        if let Some(window) = self
            .paint_order
            .iter()
            .rev()
            .copied()
            .find(|id| hit_windows.contains(id) && self.allows_input(*id))
        {
            return PointerRoute {
                target: RouteTarget::Window(FocusTarget {
                    window,
                    control: None,
                }),
            };
        }

        if let Some(modal) = self.active_modal() {
            return PointerRoute {
                target: RouteTarget::BlockedBy(modal),
            };
        }
        PointerRoute {
            target: RouteTarget::StrategicRoot,
        }
    }

    /// Whether a blank strategic-map region may receive pointer input.
    #[must_use]
    pub fn pointer_allows_root(&self) -> bool {
        matches!(
            self.pointer_route(&BTreeSet::new()).target,
            RouteTarget::StrategicRoot
        )
    }

    /// Returns an additional immutable scene restriction of this snapshot.
    #[must_use]
    pub fn restrict_to(&self, active: &BTreeSet<WindowId>) -> Self {
        let requested: BTreeSet<_> = active
            .iter()
            .copied()
            .filter(|id| self.members.contains(id))
            .collect();
        let retained: BTreeSet<_> = requested
            .iter()
            .copied()
            .filter(|id| self.owner_chain_is_within(*id, &requested))
            .collect();
        let paint_order: Vec<_> = self
            .paint_order
            .iter()
            .copied()
            .filter(|id| retained.contains(id))
            .collect();
        let presentation_history = self
            .presentation_history
            .iter()
            .copied()
            .filter(|id| retained.contains(id))
            .collect();
        let focused = self
            .focused
            .filter(|target| retained.contains(&target.window));
        let capture_owner = self.capture_owner.filter(|id| retained.contains(id));
        let mut restricted = Self {
            manager_id: self.manager_id,
            records: self.records.clone(),
            members: retained,
            paint_order,
            presentation_history,
            focused,
            capture_owner,
        };
        if restricted
            .focused
            .is_some_and(|target| !restricted.allows_input(target.window))
        {
            restricted.focused = None;
        }
        restricted
    }

    pub(crate) fn input_policy(&self, id: WindowId) -> Option<InputPolicy> {
        self.members
            .contains(&id)
            .then(|| self.records.get(&id).map(|record| record.policy.input))
            .flatten()
    }

    pub(crate) fn member_ids(&self) -> impl Iterator<Item = WindowId> + '_ {
        self.members.iter().copied()
    }

    pub(crate) const fn belongs_to(&self, manager_id: u64) -> bool {
        self.manager_id == manager_id
    }

    pub(crate) fn is_descendant_of(&self, candidate: WindowId, ancestor: WindowId) -> bool {
        let mut current = candidate;
        for _ in 0..self.records.len() {
            let Some(record) = self.records.get(&current) else {
                return false;
            };
            let WindowOwner::Window(owner) = record.owner else {
                return false;
            };
            if owner == ancestor {
                return true;
            }
            current = owner;
        }
        false
    }

    fn owner_chain_is_within(&self, id: WindowId, active: &BTreeSet<WindowId>) -> bool {
        let mut current = id;
        for _ in 0..self.records.len() {
            let Some(record) = self.records.get(&current) else {
                return false;
            };
            let WindowOwner::Window(owner) = record.owner else {
                return true;
            };
            if !active.contains(&owner) {
                return false;
            }
            current = owner;
        }
        false
    }
}
