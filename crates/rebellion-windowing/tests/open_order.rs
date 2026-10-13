use std::collections::BTreeSet;
use std::fmt::Debug;
use std::hash::Hash;

use rebellion_windowing::{
    Cardinality, ExistingOpenPolicy, FocusOnOpen, FocusRestoration, InputPolicy, OpenPlacement,
    PresentationPolicy, StackBand, SubjectId, WindowId, WindowKey, WindowKind, WindowPolicy,
};

fn assert_opaque_id_traits<T: Copy + Debug + Eq + Hash + Ord>() {}

#[test]
fn subject_namespaces_do_not_alias_equal_numeric_values() {
    // Source: strategic-windowing-toolkit.md, "Conclusions that constrain the port" item 1.
    let sector = SubjectId::new(1, 42);
    let system = SubjectId::new(2, 42);

    assert_ne!(sector, system);
    assert_eq!(BTreeSet::from([sector, system]).len(), 2);
}

#[test]
fn window_keys_distinguish_kind_and_subject() {
    // Source: strategic-windowing-toolkit.md, "Window-family matrix".
    assert_opaque_id_traits::<WindowId>();

    let system_42 = WindowKey::contextual(WindowKind::System, SubjectId::new(2, 42));
    let fleet_42 = WindowKey::contextual(WindowKind::Fleet, SubjectId::new(2, 42));
    let system_43 = WindowKey::contextual(WindowKind::System, SubjectId::new(2, 43));
    let singleton = WindowKey::singleton(WindowKind::SystemFinder);

    assert_eq!(
        BTreeSet::from([system_42, fleet_42, system_43, singleton]).len(),
        4
    );
}

#[test]
fn all_window_kinds_are_unique_and_cover_the_public_inventory() {
    // Source: strategic-windowing-toolkit.md, "Window-family matrix" and its explicit exclusions.
    let expected = [
        WindowKind::Sector,
        WindowKind::System,
        WindowKind::Fleet,
        WindowKind::Defenses,
        WindowKind::Missions,
        WindowKind::SystemFinder,
        WindowKind::FleetFinder,
        WindowKind::TroopFinder,
        WindowKind::PersonnelFinder,
        WindowKind::MessageIndex,
        WindowKind::Encyclopedia,
        WindowKind::Status,
        WindowKind::BuildSelection,
        WindowKind::MissionDialog,
        WindowKind::MoveConfirmation,
        WindowKind::ScrapConfirmation,
        WindowKind::BattleAlert,
        WindowKind::BattleResults,
        WindowKind::Menu,
    ];

    assert_eq!(WindowKind::ALL, expected);
    assert_eq!(BTreeSet::from(expected).len(), 19);
}

#[test]
fn policy_axes_do_not_encode_modality_in_stack_band() {
    // Source: strategic-windowing-toolkit.md, "Modality and input routing".
    let modeless = WindowPolicy {
        cardinality: Cardinality::Subject { max_open: None },
        band: StackBand::Normal,
        placement: OpenPlacement::TopOfBand,
        existing: ExistingOpenPolicy::RaiseRestore,
        input: InputPolicy::Modeless,
        presentation: PresentationPolicy::History,
        focus_on_open: FocusOnOpen::Window,
        restoration: FocusRestoration::PreviousEligible,
    };
    let strategic_modal = WindowPolicy {
        input: InputPolicy::StrategicModal,
        ..modeless
    };

    assert_eq!(modeless.band, strategic_modal.band);
    assert_ne!(modeless.input, strategic_modal.input);
    assert_ne!(modeless, strategic_modal);
}
