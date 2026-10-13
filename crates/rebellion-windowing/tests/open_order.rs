use std::collections::BTreeSet;
use std::fmt::Debug;
use std::hash::Hash;

use rebellion_windowing::{
    Cardinality, ExistingEffect, ExistingOpenPolicy, FocusOnOpen, FocusRestoration, InputPolicy,
    OpenPlacement, PresentationPolicy, StackBand, SubjectId, WindowCommand, WindowError, WindowId,
    WindowKey, WindowKind, WindowManager, WindowOwner, WindowPolicy, WindowSpec, WindowTransition,
};

fn assert_opaque_id_traits<T: Copy + Debug + Eq + Hash + Ord>() {}

fn modeless_subject_spec(kind: WindowKind, subject: u64) -> WindowSpec {
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

fn sector_spec(subject: u64) -> WindowSpec {
    WindowSpec {
        key: WindowKey::contextual(WindowKind::Sector, SubjectId::new(1, subject)),
        owner: WindowOwner::StrategicRoot,
        policy: WindowPolicy {
            cardinality: Cardinality::Subject { max_open: Some(2) },
            band: StackBand::Low,
            placement: OpenPlacement::BottomOfBand,
            existing: ExistingOpenPolicy::NoOp,
            input: InputPolicy::Modeless,
            presentation: PresentationPolicy::None,
            focus_on_open: FocusOnOpen::Window,
            restoration: FocusRestoration::PreviousEligible,
        },
    }
}

fn singleton_spec(kind: WindowKind) -> WindowSpec {
    WindowSpec {
        key: WindowKey::singleton(kind),
        owner: WindowOwner::StrategicRoot,
        policy: WindowPolicy {
            cardinality: Cardinality::Singleton,
            band: StackBand::Normal,
            placement: OpenPlacement::TopOfBand,
            existing: ExistingOpenPolicy::NoOp,
            input: InputPolicy::StrategicModal,
            presentation: PresentationPolicy::None,
            focus_on_open: FocusOnOpen::Window,
            restoration: FocusRestoration::PreviousEligible,
        },
    }
}

fn transient_menu_spec(owner: WindowId) -> WindowSpec {
    WindowSpec {
        key: WindowKey::singleton(WindowKind::Menu),
        owner: WindowOwner::Window(owner),
        policy: WindowPolicy {
            cardinality: Cardinality::Singleton,
            band: StackBand::Transient,
            placement: OpenPlacement::AboveOwner,
            existing: ExistingOpenPolicy::NoOp,
            input: InputPolicy::DismissOnOutsidePress,
            presentation: PresentationPolicy::None,
            focus_on_open: FocusOnOpen::Window,
            restoration: FocusRestoration::SavedMenuTarget,
        },
    }
}

fn opened_id(transition: WindowTransition) -> WindowId {
    match transition {
        WindowTransition::Opened { id } => id,
        other => panic!("expected a new window, got {other:?}"),
    }
}

#[test]
fn fresh_opens_allocate_monotonic_nonzero_ids() {
    // Source: strategic-windowing-toolkit.md, "Conclusions that constrain the port" item 1.
    // Break caught: reusing or failing to advance the manager's session-local allocator.
    let mut manager = WindowManager::new();

    let first = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::System,
                41,
            )))
            .expect("first distinct open succeeds")
            .transition,
    );
    let second = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::System,
                42,
            )))
            .expect("second distinct open succeeds")
            .transition,
    );

    assert_eq!(first.get(), 1);
    assert_eq!(second.get(), 2);
    assert!(first.get() < second.get());
}

#[test]
fn lookup_order_is_independent_of_paint_order() {
    // Source: strategic-windowing-toolkit.md, "Four separate authorities".
    // Break caught: deriving visible order from registry or semantic-key iteration.
    let mut manager = WindowManager::new();
    let detail = modeless_subject_spec(WindowKind::System, 42);
    let sector = sector_spec(7);

    let detail_id = opened_id(
        manager
            .apply(WindowCommand::Open(detail))
            .expect("detail opens")
            .transition,
    );
    let sector_id = opened_id(
        manager
            .apply(WindowCommand::Open(sector))
            .expect("sector opens")
            .transition,
    );

    assert_eq!(manager.id_for(detail.key), Some(detail_id));
    assert_eq!(manager.id_for(sector.key), Some(sector_id));
    assert_eq!(manager.visible_order(), &[sector_id, detail_id]);
}

#[test]
fn low_band_insertions_do_not_raise_unrelated_normal_windows() {
    // Source: strategic-windowing-toolkit.md, research-gate answer 3.
    // Break caught: interpreting HWND_BOTTOM-style Sector insertion as global activation.
    let mut manager = WindowManager::new();
    let system_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::System,
                41,
            )))
            .expect("System opens")
            .transition,
    );
    let fleet_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::Fleet,
                42,
            )))
            .expect("Fleet opens")
            .transition,
    );
    let sector_one = opened_id(
        manager
            .apply(WindowCommand::Open(sector_spec(1)))
            .expect("first low-band Sector opens")
            .transition,
    );
    let sector_two = opened_id(
        manager
            .apply(WindowCommand::Open(sector_spec(2)))
            .expect("second low-band Sector opens")
            .transition,
    );

    assert_eq!(
        manager.visible_order(),
        &[sector_two, sector_one, system_id, fleet_id]
    );
}

#[test]
fn transient_open_stays_above_registered_owner_and_normal_band() {
    // Source: strategic-windowing-toolkit.md, research-gate answer 3.
    // Break caught: treating an owner-relative transient as ordinary Normal-band content.
    let mut manager = WindowManager::new();
    let sector_id = opened_id(
        manager
            .apply(WindowCommand::Open(sector_spec(7)))
            .expect("Sector opens")
            .transition,
    );
    let system_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::System,
                41,
            )))
            .expect("System opens")
            .transition,
    );
    let fleet_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::Fleet,
                42,
            )))
            .expect("unrelated Fleet opens")
            .transition,
    );
    let menu_id = opened_id(
        manager
            .apply(WindowCommand::Open(transient_menu_spec(system_id)))
            .expect("owned transient opens")
            .transition,
    );

    assert_eq!(
        manager.visible_order(),
        &[sector_id, system_id, fleet_id, menu_id]
    );
}

#[test]
fn background_detail_open_cannot_overtake_an_active_modal() {
    // Source: strategic-windowing-toolkit.md, research-gate answers 3 and 4.
    // Break caught: conflating paint bands with the active strategic input boundary.
    let mut manager = WindowManager::new();
    let system_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::System,
                41,
            )))
            .expect("System opens")
            .transition,
    );
    let finder_id = opened_id(
        manager
            .apply(WindowCommand::Open(singleton_spec(
                WindowKind::SystemFinder,
            )))
            .expect("modal Finder opens")
            .transition,
    );
    assert_eq!(
        manager.apply(WindowCommand::Open(transient_menu_spec(system_id))),
        Err(WindowError::InvalidPlacement)
    );
    let menu_id = opened_id(
        manager
            .apply(WindowCommand::Open(transient_menu_spec(finder_id)))
            .expect("a transient owned by the active modal is eligible")
            .transition,
    );
    let fleet_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::Fleet,
                42,
            )))
            .expect("background Fleet detail still registers")
            .transition,
    );

    assert_eq!(
        manager.visible_order(),
        &[system_id, fleet_id, finder_id, menu_id]
    );
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn modal_open_rejects_an_ineligible_existing_transient_atomically() {
    // Source: strategic-windowing-toolkit.md, research-gate answers 3 and 4.
    // Break caught: placing a new modal below a transient owned by its underlay.
    let mut manager = WindowManager::new();
    let system_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::System,
                41,
            )))
            .expect("System opens")
            .transition,
    );
    let menu_id = opened_id(
        manager
            .apply(WindowCommand::Open(transient_menu_spec(system_id)))
            .expect("System-owned menu opens")
            .transition,
    );
    let before = manager.visible_order().to_vec();

    let finder = singleton_spec(WindowKind::SystemFinder);
    assert_eq!(
        manager.apply(WindowCommand::Open(finder)),
        Err(WindowError::InvalidPlacement)
    );
    assert_eq!(manager.id_for(finder.key), None);
    assert_eq!(manager.visible_order(), before);

    let fleet_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::Fleet,
                42,
            )))
            .expect("rejection does not consume an identity")
            .transition,
    );
    assert_eq!(fleet_id.get(), 3);
    assert_eq!(manager.visible_order(), &[system_id, fleet_id, menu_id]);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn transient_owned_by_lower_modal_cannot_overtake_active_modal() {
    // Source: implementation plan, Task 2 active-modal order boundary.
    // Break caught: using the lowest modal instead of the topmost active modal for eligibility.
    let mut manager = WindowManager::new();
    let lower_modal_id = opened_id(
        manager
            .apply(WindowCommand::Open(singleton_spec(
                WindowKind::SystemFinder,
            )))
            .expect("first modal opens")
            .transition,
    );
    let active_modal_id = opened_id(
        manager
            .apply(WindowCommand::Open(singleton_spec(WindowKind::FleetFinder)))
            .expect("second modal becomes active")
            .transition,
    );
    let before = manager.visible_order().to_vec();

    assert_eq!(
        manager.apply(WindowCommand::Open(transient_menu_spec(lower_modal_id))),
        Err(WindowError::InvalidPlacement)
    );
    assert_eq!(manager.visible_order(), before);

    let menu_id = opened_id(
        manager
            .apply(WindowCommand::Open(transient_menu_spec(active_modal_id)))
            .expect("the active modal may own a transient")
            .transition,
    );
    assert_eq!(
        manager.visible_order(),
        &[lower_modal_id, active_modal_id, menu_id]
    );
    assert_eq!(manager.validate(), Ok(()));
    assert_eq!(menu_id.get(), 3);
}

#[test]
fn lower_modal_owned_modeless_open_stays_below_active_modal() {
    // Source: implementation plan, Task 2 active-modal order boundary.
    // Break caught: treating any modal owner as eligible to overtake a newer modal.
    let mut manager = WindowManager::new();
    let lower_modal_id = opened_id(
        manager
            .apply(WindowCommand::Open(singleton_spec(
                WindowKind::SystemFinder,
            )))
            .expect("first modal opens")
            .transition,
    );
    let active_modal_id = opened_id(
        manager
            .apply(WindowCommand::Open(singleton_spec(WindowKind::FleetFinder)))
            .expect("second modal becomes active")
            .transition,
    );
    let child = WindowSpec {
        owner: WindowOwner::Window(lower_modal_id),
        ..modeless_subject_spec(WindowKind::System, 42)
    };
    let child_id = opened_id(
        manager
            .apply(WindowCommand::Open(child))
            .expect("lower-modal child opens below active modal")
            .transition,
    );

    assert_eq!(
        manager.visible_order(),
        &[lower_modal_id, child_id, active_modal_id]
    );
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn window_owned_open_cannot_commit_below_its_owner() {
    // Source: implementation plan, Task 2 owner/order validation.
    // Break caught: accepting an owner link whose band placement contradicts child-above-owner.
    let mut manager = WindowManager::new();
    let owner_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::System,
                41,
            )))
            .expect("owner opens")
            .transition,
    );
    let before = manager.visible_order().to_vec();

    let mut below_owner = modeless_subject_spec(WindowKind::Fleet, 42);
    below_owner.owner = WindowOwner::Window(owner_id);
    below_owner.policy.band = StackBand::Low;
    below_owner.policy.placement = OpenPlacement::BottomOfBand;
    assert_eq!(
        manager.apply(WindowCommand::Open(below_owner)),
        Err(WindowError::InvalidPlacement)
    );
    assert_eq!(manager.id_for(below_owner.key), None);
    assert_eq!(manager.visible_order(), before);

    let valid_child = WindowSpec {
        owner: WindowOwner::Window(owner_id),
        policy: WindowPolicy {
            band: StackBand::Normal,
            placement: OpenPlacement::TopOfBand,
            ..below_owner.policy
        },
        ..below_owner
    };
    let child_id = opened_id(
        manager
            .apply(WindowCommand::Open(valid_child))
            .expect("compatible owner-relative order opens")
            .transition,
    );
    assert_eq!(child_id.get(), 2);
    assert_eq!(manager.visible_order(), &[owner_id, child_id]);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn raising_an_owner_keeps_its_visible_subtree_above_it() {
    // Source: implementation plan, Task 2 owner/order and RaiseRestore contracts.
    // Break caught: raising a parent alone above its child and invalidating owner order.
    let mut manager = WindowManager::new();
    let parent = modeless_subject_spec(WindowKind::System, 41);
    let parent_id = opened_id(
        manager
            .apply(WindowCommand::Open(parent))
            .expect("parent opens")
            .transition,
    );
    let child = WindowSpec {
        owner: WindowOwner::Window(parent_id),
        ..modeless_subject_spec(WindowKind::Fleet, 42)
    };
    let child_id = opened_id(
        manager
            .apply(WindowCommand::Open(child))
            .expect("child opens above parent")
            .transition,
    );
    let sibling_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::Defenses,
                43,
            )))
            .expect("unrelated sibling opens")
            .transition,
    );
    assert_eq!(manager.visible_order(), &[parent_id, child_id, sibling_id]);

    assert_eq!(
        manager
            .apply(WindowCommand::Open(parent))
            .expect("reopening parent raises its subtree")
            .transition,
        WindowTransition::Existing {
            id: parent_id,
            effect: ExistingEffect::Raised,
        }
    );
    assert_eq!(manager.visible_order(), &[sibling_id, parent_id, child_id]);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn open_order_validation_accepts_registry_key_and_band_invariants() {
    // Source: implementation plan, Task 2 order-only validation contract.
    // Break caught: a successful open sequence leaving divergent registry/index/order state.
    let mut manager = WindowManager::new();
    manager
        .apply(WindowCommand::Open(sector_spec(7)))
        .expect("Sector opens");
    let system_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::System,
                41,
            )))
            .expect("System opens")
            .transition,
    );
    manager
        .apply(WindowCommand::Open(transient_menu_spec(system_id)))
        .expect("transient menu opens");

    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn subject_cardinality_and_singleton_shape_are_enforced() {
    // Source: strategic-windowing-toolkit.md, "Existing-window policy" and family matrix.
    // Break caught: admitting a subject on a singleton, omitting a required subject, or
    // exceeding a subject family's declared capacity.
    let mut manager = WindowManager::new();
    let mut contextual_singleton = singleton_spec(WindowKind::FleetFinder);
    contextual_singleton.key =
        WindowKey::contextual(WindowKind::FleetFinder, SubjectId::new(2, 41));
    assert_eq!(
        manager.apply(WindowCommand::Open(contextual_singleton)),
        Err(WindowError::SubjectMismatch(contextual_singleton.key))
    );

    let singleton = singleton_spec(WindowKind::FleetFinder);
    let singleton_id = opened_id(
        manager
            .apply(WindowCommand::Open(singleton))
            .expect("a correctly shaped singleton opens")
            .transition,
    );
    assert_eq!(singleton_id.get(), 1);

    let mut subjectless_detail = modeless_subject_spec(WindowKind::System, 41);
    subjectless_detail.key = WindowKey::singleton(WindowKind::System);
    assert_eq!(
        manager.apply(WindowCommand::Open(subjectless_detail)),
        Err(WindowError::SubjectMismatch(subjectless_detail.key))
    );

    manager
        .apply(WindowCommand::Open(sector_spec(1)))
        .expect("first Sector opens");
    manager
        .apply(WindowCommand::Open(sector_spec(2)))
        .expect("second Sector opens");
    assert_eq!(
        manager.apply(WindowCommand::Open(sector_spec(3))),
        Err(WindowError::CardinalityExceeded(WindowKind::Sector))
    );
}

#[test]
fn open_order_atomic_rejection_preserves_allocator() {
    // Source: implementation plan, Task 2 atomic-open contract.
    // Break caught: allocating or partially indexing a spec before placement validation.
    let mut manager = WindowManager::new();
    let first = modeless_subject_spec(WindowKind::System, 41);
    let first_id = opened_id(
        manager
            .apply(WindowCommand::Open(first))
            .expect("valid detail opens")
            .transition,
    );
    let before = manager.visible_order().to_vec();

    let mut invalid = modeless_subject_spec(WindowKind::Fleet, 42);
    invalid.policy.placement = OpenPlacement::AboveOwner;
    assert_eq!(
        manager.apply(WindowCommand::Open(invalid)),
        Err(WindowError::InvalidPlacement)
    );
    assert_eq!(manager.id_for(invalid.key), None);
    assert_eq!(manager.id_for(first.key), Some(first_id));
    assert_eq!(manager.visible_order(), before);

    let mut transient_without_owner_placement = transient_menu_spec(first_id);
    transient_without_owner_placement.policy.placement = OpenPlacement::TopOfBand;
    assert_eq!(
        manager.apply(WindowCommand::Open(transient_without_owner_placement)),
        Err(WindowError::InvalidPlacement)
    );
    assert_eq!(manager.id_for(transient_without_owner_placement.key), None);
    assert_eq!(manager.visible_order(), before);

    let mut foreign = WindowManager::new();
    foreign
        .apply(WindowCommand::Open(modeless_subject_spec(
            WindowKind::System,
            91,
        )))
        .expect("foreign first ID opens");
    let foreign_second_id = opened_id(
        foreign
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::Fleet,
                92,
            )))
            .expect("foreign second ID opens")
            .transition,
    );
    let unknown_owner = transient_menu_spec(foreign_second_id);
    assert_eq!(
        manager.apply(WindowCommand::Open(unknown_owner)),
        Err(WindowError::UnknownOwner(foreign_second_id))
    );
    assert_eq!(manager.id_for(unknown_owner.key), None);
    assert_eq!(manager.visible_order(), before);

    let second_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::Fleet,
                42,
            )))
            .expect("rejection does not consume an identity")
            .transition,
    );
    assert_eq!(second_id.get(), 2);
}

#[test]
fn open_order_duplicate_noop_preserves_order() {
    // Source: strategic-windowing-toolkit.md, "Existing-window policy".
    // Break caught: duplicate NoOp allocating, reordering, or replacing identity.
    let mut manager = WindowManager::new();
    let sector = sector_spec(7);
    let sector_id = opened_id(
        manager
            .apply(WindowCommand::Open(sector))
            .expect("initial Sector opens")
            .transition,
    );
    let detail_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::System,
                42,
            )))
            .expect("detail opens above Sector")
            .transition,
    );
    let before = manager.visible_order().to_vec();

    let duplicate = manager
        .apply(WindowCommand::Open(sector))
        .expect("compatible duplicate is a successful no-op");

    assert_eq!(
        duplicate.transition,
        WindowTransition::Existing {
            id: sector_id,
            effect: ExistingEffect::NoOp,
        }
    );
    assert_eq!(manager.id_for(sector.key), Some(sector_id));
    assert_eq!(manager.visible_order(), before);

    let next_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::Fleet,
                42,
            )))
            .expect("a later fresh open still gets the next identity")
            .transition,
    );
    assert_eq!((sector_id.get(), detail_id.get(), next_id.get()), (1, 2, 3));
}

#[test]
fn duplicate_sector_and_singleton_opens_preserve_order_while_detail_reopen_raises() {
    // Source: strategic-windowing-toolkit.md, "Existing-window policy".
    // Break caught: applying one generic duplicate-open rule across distinct families.
    let mut manager = WindowManager::new();
    let system = modeless_subject_spec(WindowKind::System, 41);
    let fleet = modeless_subject_spec(WindowKind::Fleet, 42);
    let sector = sector_spec(7);
    let finder = singleton_spec(WindowKind::FleetFinder);

    let system_id = opened_id(
        manager
            .apply(WindowCommand::Open(system))
            .expect("System opens")
            .transition,
    );
    let fleet_id = opened_id(
        manager
            .apply(WindowCommand::Open(fleet))
            .expect("Fleet opens")
            .transition,
    );
    let sector_id = opened_id(
        manager
            .apply(WindowCommand::Open(sector))
            .expect("Sector opens")
            .transition,
    );
    let finder_id = opened_id(
        manager
            .apply(WindowCommand::Open(finder))
            .expect("Finder opens")
            .transition,
    );
    assert_eq!(
        manager.visible_order(),
        &[sector_id, system_id, fleet_id, finder_id]
    );

    let order_before_noops = manager.visible_order().to_vec();
    assert_eq!(
        manager
            .apply(WindowCommand::Open(sector))
            .expect("duplicate Sector is accepted")
            .transition,
        WindowTransition::Existing {
            id: sector_id,
            effect: ExistingEffect::NoOp,
        }
    );
    assert_eq!(
        manager
            .apply(WindowCommand::Open(finder))
            .expect("duplicate singleton is accepted")
            .transition,
        WindowTransition::Existing {
            id: finder_id,
            effect: ExistingEffect::NoOp,
        }
    );
    assert_eq!(manager.visible_order(), order_before_noops);

    assert_eq!(
        manager
            .apply(WindowCommand::Open(system))
            .expect("detail reopen raises the same identity")
            .transition,
        WindowTransition::Existing {
            id: system_id,
            effect: ExistingEffect::Raised,
        }
    );
    assert_eq!(
        manager.visible_order(),
        &[sector_id, fleet_id, system_id, finder_id]
    );
}

#[test]
fn existing_key_rejects_different_owner_or_policy_atomically() {
    // Source: implementation plan, Task 2 atomic-open contract.
    // Break caught: accepting a duplicate key that silently changes ownership or policy.
    let mut manager = WindowManager::new();
    let sector = sector_spec(7);
    let sector_id = opened_id(
        manager
            .apply(WindowCommand::Open(sector))
            .expect("Sector opens")
            .transition,
    );
    let detail = modeless_subject_spec(WindowKind::System, 42);
    let detail_id = opened_id(
        manager
            .apply(WindowCommand::Open(detail))
            .expect("detail opens")
            .transition,
    );
    let before = manager.visible_order().to_vec();

    let different_owner = WindowSpec {
        owner: WindowOwner::Window(sector_id),
        ..detail
    };
    assert_eq!(
        manager.apply(WindowCommand::Open(different_owner)),
        Err(WindowError::DuplicateKeyWithDifferentOwner(detail.key))
    );

    let different_policy = WindowSpec {
        policy: WindowPolicy {
            input: InputPolicy::StrategicModal,
            ..detail.policy
        },
        ..detail
    };
    assert_eq!(
        manager.apply(WindowCommand::Open(different_policy)),
        Err(WindowError::DuplicateKeyWithDifferentPolicy(detail.key))
    );

    assert_eq!(manager.id_for(detail.key), Some(detail_id));
    assert_eq!(manager.visible_order(), before);
    let next_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::Fleet,
                42,
            )))
            .expect("rejections do not consume an identity")
            .transition,
    );
    assert_eq!(next_id.get(), 3);
}

#[test]
fn same_kind_cannot_bypass_capacity_by_changing_policy() {
    // Source: implementation plan, Task 2 family-policy validation.
    // Break caught: treating caller-supplied policy as authority after a family is registered.
    let mut manager = WindowManager::new();
    manager
        .apply(WindowCommand::Open(sector_spec(1)))
        .expect("first Sector opens");
    manager
        .apply(WindowCommand::Open(sector_spec(2)))
        .expect("second Sector opens");
    let before = manager.visible_order().to_vec();

    let mut bypass = sector_spec(3);
    bypass.policy.cardinality = Cardinality::Subject { max_open: None };
    assert_eq!(
        manager.apply(WindowCommand::Open(bypass)),
        Err(WindowError::KindPolicyMismatch(WindowKind::Sector))
    );
    assert_eq!(manager.id_for(bypass.key), None);
    assert_eq!(manager.visible_order(), before);

    let next_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::Fleet,
                3,
            )))
            .expect("rejection does not consume an identity")
            .transition,
    );
    assert_eq!(next_id.get(), 3);
}

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
