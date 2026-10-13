#![forbid(unsafe_code)]

//! Renderer-independent vocabulary for strategic window lifecycle and routing.
//!
//! This crate begins with semantic identities and independent policy axes. It
//! deliberately has no renderer, simulation, serialization, operating-system,
//! or asset dependencies. Lifecycle and routing behavior are introduced by
//! later implementation slices.

mod identity;
mod manager;
mod policy;
mod snapshot;

pub use identity::{
    ControlId, FocusTarget, SubjectId, WindowId, WindowKey, WindowKind, WindowOwner,
};
pub use manager::{
    CaptureCancelReason, CloseReason, ExistingEffect, InvariantKind, InvariantViolation,
    TransitionSubject, UnchangedReason, WindowCommand, WindowCommit, WindowError, WindowEvent,
    WindowManager, WindowTransition,
};
pub use policy::{
    Cardinality, ExistingOpenPolicy, FocusOnOpen, FocusRestoration, InputPolicy, OpenPlacement,
    PresentationPolicy, StackBand, WindowPolicy, WindowSpec,
};

pub use snapshot::{BubblePolicy, KeyboardRoute, PointerRoute, RouteTarget, RoutingSnapshot};
