use std::collections::{BTreeMap, BTreeSet};

use crate::{FocusTarget, InputPolicy, WindowId, WindowKey, WindowOwner, WindowSpec};

/// One semantic recipient frozen for a physical input dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteTarget {
    /// The persistent strategic scene.
    StrategicRoot,
    /// An allocated window, optionally with its focused child control.
    Window(FocusTarget),
    /// No hit is eligible through this modal boundary.
    BlockedBy(WindowId),
}

/// Whether an unhandled key may continue from its owner to the strategic root.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BubblePolicy {
    /// Root delivery is permitted only after child/top-level handling reports unhandled.
    ToRoot,
    /// The modal or transient boundary consumes unhandled keys.
    StopAtWindow,
}

/// Frozen focused recipient and propagation boundary; actual handling is adapter-owned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyboardRoute {
    /// Initial recipient, including an eligible focused child control.
    pub target: RouteTarget,
    /// Limit on delivery after the initial recipient reports unhandled.
    pub bubble: BubblePolicy,
}

/// The single window, blocker, or root selected for one pointer dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointerRoute {
    /// Semantic pointer recipient; control hit testing stays with the adapter.
    pub target: RouteTarget,
}

/// Owned immutable input lease independent of later manager commits or teardown.
///
/// Source: FUN_00606960/FUN_00422ce0 defer close/restoration to later dispatches.
/// Port: copy identities and policies instead of reproducing the native message queue.
#[derive(Debug, Clone)]
pub struct RoutingSnapshot {
    records: BTreeMap<WindowId, WindowSpec>,
    paint_order: Vec<WindowId>,
    presentation_history: Vec<WindowId>,
    focused: Option<FocusTarget>,
    capture_owner: Option<WindowId>,
}

impl RoutingSnapshot {
    pub(crate) fn new(
        records: BTreeMap<WindowId, WindowSpec>,
        paint_order: Vec<WindowId>,
        presentation_history: Vec<WindowId>,
        focused: Option<FocusTarget>,
        capture_owner: Option<WindowId>,
    ) -> Self {
        Self {
            records,
            paint_order,
            presentation_history,
            focused,
            capture_owner,
        }
    }

    /// Returns visible identities in frozen bottom-to-top paint order.
    #[must_use]
    pub fn paint_order(&self) -> &[WindowId] {
        &self.paint_order
    }

    /// Returns minimized identities from oldest to newest, independently of paint order.
    #[must_use]
    pub fn presentation_history(&self) -> &[WindowId] {
        &self.presentation_history
    }

    /// Returns the semantic key of a visible or minimized snapshot member.
    #[must_use]
    pub fn key(&self, id: WindowId) -> Option<WindowKey> {
        self.records.get(&id).map(|record| record.key)
    }

    /// Returns the actual lifecycle owner of a visible or minimized snapshot member.
    #[must_use]
    pub fn owner(&self, id: WindowId) -> Option<WindowOwner> {
        self.records.get(&id).map(|record| record.owner)
    }

    /// Returns the keyboard focus captured when the lease was created.
    #[must_use]
    pub fn focused(&self) -> Option<FocusTarget> {
        self.focused
    }

    /// Returns the temporary pointer owner captured when the lease was created.
    #[must_use]
    pub fn capture_owner(&self) -> Option<WindowId> {
        self.capture_owner
    }

    /// Returns a further restricted scene view without changing this lease or the manager.
    ///
    /// Metadata is retained for lifecycle ancestry; dormant records contribute no paint,
    /// history, focus, capture or input boundary. Repeated restrictions cannot re-admit IDs.
    #[must_use]
    pub fn restrict_to(&self, active: &BTreeSet<WindowId>) -> Self {
        let mut view = self.clone();
        view.paint_order.retain(|id| active.contains(id));
        view.presentation_history.retain(|id| active.contains(id));
        view.capture_owner = view.capture_owner.filter(|id| view.allows_input(*id));
        if view.focused.map_or_else(
            || view.active_modal().is_some(),
            |target| !view.allows_input(target.window),
        ) {
            view.focused = view
                .paint_order
                .iter()
                .rev()
                .copied()
                .find(|id| view.allows_input(*id))
                .map(|window| FocusTarget {
                    window,
                    control: None,
                });
        }
        view
    }

    /// Returns the topmost visible strategic modal independently of hit or paint success.
    #[must_use]
    pub fn active_modal(&self) -> Option<WindowId> {
        self.paint_order
            .iter()
            .rev()
            .copied()
            .find(|id| self.records[id].policy.input == InputPolicy::StrategicModal)
    }

    /// Whether this identity belongs to the frozen visible set.
    #[must_use]
    pub fn is_visible(&self, id: WindowId) -> bool {
        self.paint_order.contains(&id)
    }

    /// Visibility/modal eligibility only; this does not grant an event to every eligible hit.
    #[must_use]
    pub fn allows_input(&self, id: WindowId) -> bool {
        self.is_visible(id)
            && self
                .active_modal()
                .is_none_or(|modal| id == modal || self.is_descendant_of(id, modal))
    }

    /// Whether a pointer event with no eligible window hit may reach the strategic root.
    #[must_use]
    pub fn pointer_allows_root(&self) -> bool {
        self.capture_owner.is_none()
            && self.active_modal().is_none()
            && self.dismissible_boundary().is_none()
    }

    /// Chooses one owner: temporary capture, eligible topmost hit/menu, modal blocker, or root.
    #[must_use]
    pub fn pointer_route(&self, hit_windows: &BTreeSet<WindowId>) -> PointerRoute {
        if let Some(window) = self.capture_owner {
            return PointerRoute {
                target: Self::window_target(window),
            };
        }
        for id in self.paint_order.iter().rev().copied() {
            if !self.allows_input(id) {
                continue;
            }
            if hit_windows.contains(&id)
                || self.records[&id].policy.input == InputPolicy::DismissOnOutsidePress
            {
                return PointerRoute {
                    target: Self::window_target(id),
                };
            }
        }
        PointerRoute {
            target: self
                .active_modal()
                .map_or(RouteTarget::StrategicRoot, RouteTarget::BlockedBy),
        }
    }

    /// Preserves child-control focus and stops unhandled keys at modal/transient boundaries.
    #[must_use]
    pub fn keyboard_route(&self) -> KeyboardRoute {
        if let Some(focus) = self.focused {
            let mut window = focus.window;
            let mut stop = self.active_modal().is_some();
            for _ in 0..self.records.len() {
                let Some(record) = self.records.get(&window) else {
                    break;
                };
                stop |= self.is_visible(window)
                    && record.policy.input == InputPolicy::DismissOnOutsidePress;
                let WindowOwner::Window(owner) = record.owner else {
                    break;
                };
                window = owner;
            }
            return KeyboardRoute {
                target: RouteTarget::Window(focus),
                bubble: if stop {
                    BubblePolicy::StopAtWindow
                } else {
                    BubblePolicy::ToRoot
                },
            };
        }
        KeyboardRoute {
            target: RouteTarget::StrategicRoot,
            bubble: BubblePolicy::ToRoot,
        }
    }

    fn window_target(window: WindowId) -> RouteTarget {
        RouteTarget::Window(FocusTarget {
            window,
            control: None,
        })
    }

    fn dismissible_boundary(&self) -> Option<WindowId> {
        self.paint_order.iter().rev().copied().find(|id| {
            self.allows_input(*id)
                && self.records[id].policy.input == InputPolicy::DismissOnOutsidePress
        })
    }

    fn is_descendant_of(&self, candidate: WindowId, ancestor: WindowId) -> bool {
        let mut current = candidate;
        for _ in 0..self.records.len() {
            let Some(WindowOwner::Window(owner)) = self.owner(current) else {
                return false;
            };
            if owner == ancestor {
                return true;
            }
            current = owner;
        }
        false
    }
}
