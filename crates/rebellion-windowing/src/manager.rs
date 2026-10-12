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
}

/// Effect of reopening an existing compatible semantic key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExistingEffect {
    /// Identity, order, and all externally owned content remain unchanged.
    NoOp,
    /// The existing visible identity moved to the top of its eligible order.
    Raised,
}

/// Observable result of a successful manager command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

/// Successful atomic manager commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowCommit {
    /// State transition produced by the command.
    pub transition: WindowTransition,
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
    /// Creates an empty manager for one UI session.
    #[must_use]
    pub fn new() -> Self {
        Self {
            registry: BTreeMap::new(),
            key_index: BTreeMap::new(),
            visible_order: Vec::new(),
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
        if let Some(id) = self.registry.keys().find(|id| !seen.contains(id)) {
            return Err(InvariantViolation {
                kind: InvariantKind::MissingVisibleOrder,
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
                let Some(owner_position) = positions.get(&owner) else {
                    return Err(InvariantViolation {
                        kind: InvariantKind::OwnerMissing,
                        windows: vec![*id, owner],
                    });
                };
                if positions[id] <= *owner_position {
                    return Err(InvariantViolation {
                        kind: InvariantKind::ChildBelowOwner,
                        windows: vec![*id, owner],
                    });
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

        Ok(())
    }

    /// Applies one command atomically.
    pub fn apply(&mut self, command: WindowCommand) -> Result<WindowCommit, WindowError> {
        let WindowCommand::Open(spec) = command;
        if let Some(id) = self.id_for(spec.key) {
            let existing = self.registry[&id].spec;
            if existing.owner != spec.owner {
                return Err(WindowError::DuplicateKeyWithDifferentOwner(spec.key));
            }
            if existing.policy != spec.policy {
                return Err(WindowError::DuplicateKeyWithDifferentPolicy(spec.key));
            }

            let effect = match existing.policy.existing {
                crate::ExistingOpenPolicy::NoOp => ExistingEffect::NoOp,
                crate::ExistingOpenPolicy::RaiseRestore => {
                    self.raise_visible(id);
                    ExistingEffect::Raised
                }
            };
            return Ok(WindowCommit {
                transition: WindowTransition::Existing { id, effect },
            });
        }

        self.validate_new_spec(spec)?;
        let insertion = self.proposed_insertion_index(spec);

        let raw_id = self.next_id.ok_or(WindowError::IdExhausted)?;
        let id = WindowId(raw_id);
        self.next_id = NonZeroU64::new(raw_id.get().wrapping_add(1));

        self.registry.insert(id, WindowRecord { spec });
        self.key_index.insert(spec.key, id);
        self.visible_order.insert(insertion, id);

        Ok(WindowCommit {
            transition: WindowTransition::Opened { id },
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
    /// A window-owned spec names an identity outside the current registry.
    UnknownOwner(WindowId),
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
}
