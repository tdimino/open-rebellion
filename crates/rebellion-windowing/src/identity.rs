use std::num::NonZeroU64;

/// Opaque identity for one top-level window during a UI session.
///
/// Values are allocated by the window manager and are not persistent game or
/// wire identities. The private representation prevents consumers from
/// inventing fixed IDs for singleton windows or fixtures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowId(pub(crate) NonZeroU64);

impl WindowId {
    /// Returns the nonzero diagnostic value of this session identity.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

/// Namespace-qualified subject identity used to distinguish window contexts.
///
/// The namespace keeps equal numeric values from unrelated runtime domains
/// distinct without importing simulation key types into this crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubjectId {
    /// Identifies the runtime domain that owns `value`.
    pub namespace: u32,
    /// Identifies the subject within its namespace.
    pub value: u64,
}

impl SubjectId {
    /// Creates a namespace-qualified subject identity.
    #[must_use]
    pub const fn new(namespace: u32, value: u64) -> Self {
        Self { namespace, value }
    }
}

/// Identity of a focusable control within its owning [`WindowId`].
///
/// A control value is never a global key. Consumers scope it through
/// [`FocusTarget`] so equal values in different windows remain distinct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ControlId(pub u64);

/// Semantic family of a registered top-level window.
///
/// Rename editors and cockpit widgets are controls owned by another surface,
/// not top-level kinds. Unidentified original native classes are excluded
/// until their semantics are recovered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WindowKind {
    /// Modeless Sector window.
    Sector,
    /// Modeless System or Manufacturing detail window.
    System,
    /// Modeless Fleet detail window.
    Fleet,
    /// Modeless System Defenses detail window.
    Defenses,
    /// Modeless Missions detail window.
    Missions,
    /// System Finder policy slot; its port content is not implemented yet.
    SystemFinder,
    /// Fleet or Ship Finder window.
    FleetFinder,
    /// Troop Finder window.
    TroopFinder,
    /// Personnel Finder window.
    PersonnelFinder,
    /// Message Index window.
    MessageIndex,
    /// Encyclopedia window.
    Encyclopedia,
    /// Contextual Status window.
    Status,
    /// Manufacturing Build Selection window.
    BuildSelection,
    /// Mission creation dialog.
    MissionDialog,
    /// Confirmed Move dialog.
    MoveConfirmation,
    /// Scrap confirmation dialog.
    ScrapConfirmation,
    /// Tactical Battle Alert dialog.
    BattleAlert,
    /// Battle Results or Summary window: original window type `0x1c`
    /// (`FUN_0044c410`), whose class also shows the Operation Report.
    BattleResults,
    /// One active object, agent, game-speed, or GID menu presentation.
    Menu,
}

impl WindowKind {
    /// Complete public top-level inventory in stable declaration order.
    pub const ALL: [Self; 19] = [
        Self::Sector,
        Self::System,
        Self::Fleet,
        Self::Defenses,
        Self::Missions,
        Self::SystemFinder,
        Self::FleetFinder,
        Self::TroopFinder,
        Self::PersonnelFinder,
        Self::MessageIndex,
        Self::Encyclopedia,
        Self::Status,
        Self::BuildSelection,
        Self::MissionDialog,
        Self::MoveConfirmation,
        Self::ScrapConfirmation,
        Self::BattleAlert,
        Self::BattleResults,
        Self::Menu,
    ];
}

/// Stable semantic lookup key for a singleton or contextual window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowKey {
    /// Window family.
    pub kind: WindowKind,
    /// Context identity, or `None` for a singleton key.
    pub subject: Option<SubjectId>,
}

impl WindowKey {
    /// Creates a key with no contextual subject.
    #[must_use]
    pub const fn singleton(kind: WindowKind) -> Self {
        Self {
            kind,
            subject: None,
        }
    }

    /// Creates a key for one subject within a window family.
    #[must_use]
    pub const fn contextual(kind: WindowKind, subject: SubjectId) -> Self {
        Self {
            kind,
            subject: Some(subject),
        }
    }
}

/// Lifecycle owner of a registered window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowOwner {
    /// The persistent strategic root owns the window.
    StrategicRoot,
    /// Another registered window owns the window.
    Window(WindowId),
}

/// Semantic keyboard-focus target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FocusTarget {
    /// Owning top-level window.
    pub window: WindowId,
    /// Focused child within that window, or the top-level itself when absent.
    pub control: Option<ControlId>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_id_exposes_its_allocated_nonzero_value_without_public_construction() {
        let id = WindowId(NonZeroU64::new(37).expect("test ID is nonzero"));

        assert_eq!(id.get(), 37);
    }
}
