use crate::{WindowKey, WindowOwner};

/// Semantic paint-order band, independent of input modality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StackBand {
    /// Low modeless surfaces such as Sector windows.
    Low,
    /// Ordinary strategic windows and dialogs.
    Normal,
    /// Temporary surfaces constrained above their owners.
    Transient,
}

/// Placement of a newly visible window within its stack constraints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenPlacement {
    /// Insert at the bottom of the selected band.
    BottomOfBand,
    /// Insert at the top of the selected band.
    TopOfBand,
    /// Insert directly above the declared owner.
    AboveOwner,
}

/// Behavior when an open request finds an existing compatible key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExistingOpenPolicy {
    /// Preserve the existing window, content, order, and focus.
    NoOp,
    /// Restore a minimized window if necessary, then raise it.
    RaiseRestore,
}

/// Strategic input boundary, independent of paint order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputPolicy {
    /// Input outside the window can continue to eligible strategic surfaces.
    Modeless,
    /// The window blocks all lower strategic input.
    StrategicModal,
    /// An outside press is consumed and requests explicit dismissal.
    DismissOnOutsidePress,
}

/// Whether a minimized window participates in presentation history.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresentationPolicy {
    /// The window has no presentation-history entry.
    None,
    /// The minimized window appears in shared bounded history.
    History,
}

/// Semantic focus change requested by a successful new open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusOnOpen {
    /// Preserve the current eligible focus target.
    Preserve,
    /// Focus the newly opened top-level window.
    Window,
}

/// Number and key shape of windows allowed for a family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cardinality {
    /// At most one subjectless key exists for the family.
    Singleton,
    /// Windows are distinguished by subject, with an optional family limit.
    Subject {
        /// Maximum simultaneously registered subjects, or no fixed limit.
        max_open: Option<usize>,
    },
}

/// Focus-selection rule after a window leaves the eligible set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusRestoration {
    /// Select the previous eligible target using shared window order.
    PreviousEligible,
    /// Restore the menu's saved invoker target when it remains eligible.
    SavedMenuTarget,
}

/// Complete renderer-independent policy declared for one window family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowPolicy {
    /// Family cardinality and required key shape.
    pub cardinality: Cardinality,
    /// Semantic paint-order band.
    pub band: StackBand,
    /// Initial placement within the allowed order.
    pub placement: OpenPlacement,
    /// Compatible duplicate-open behavior.
    pub existing: ExistingOpenPolicy,
    /// Strategic input boundary.
    pub input: InputPolicy,
    /// Minimized presentation behavior.
    pub presentation: PresentationPolicy,
    /// Focus behavior for a successful new open.
    pub focus_on_open: FocusOnOpen,
    /// Focus restoration after the window becomes ineligible.
    pub restoration: FocusRestoration,
}

/// Complete semantic request to register a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowSpec {
    /// Stable semantic key for the requested surface.
    pub key: WindowKey,
    /// Lifecycle owner.
    pub owner: WindowOwner,
    /// Family policy to validate and retain.
    pub policy: WindowPolicy,
}
