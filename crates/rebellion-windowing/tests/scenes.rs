use std::collections::BTreeSet;

use rebellion_windowing::{
    BubblePolicy, CaptureCancelReason, Cardinality, ControlId, ExistingOpenPolicy, FocusOnOpen,
    FocusRestoration, FocusTarget, InputPolicy, KeyboardRoute, OpenPlacement, PresentationPolicy,
    RouteTarget, StackBand, SubjectId, WindowCommand, WindowError, WindowEvent, WindowId,
    WindowKey, WindowKind, WindowManager, WindowOwner, WindowPolicy, WindowSpec, WindowTransition,
};

fn detail(subject: u64) -> WindowSpec {
    WindowSpec {
        key: WindowKey::contextual(WindowKind::System, SubjectId::new(2, subject)),
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

fn modal() -> WindowSpec {
    let mut spec = detail(0);
    spec.key = WindowKey::singleton(WindowKind::Encyclopedia);
    spec.policy.cardinality = Cardinality::Singleton;
    spec.policy.existing = ExistingOpenPolicy::NoOp;
    spec.policy.input = InputPolicy::StrategicModal;
    spec.policy.presentation = PresentationPolicy::None;
    spec
}

fn menu(owner: WindowId) -> WindowSpec {
    let mut spec = modal();
    spec.key = WindowKey::singleton(WindowKind::Menu);
    spec.owner = WindowOwner::Window(owner);
    spec.policy.band = StackBand::Transient;
    spec.policy.placement = OpenPlacement::AboveOwner;
    spec.policy.input = InputPolicy::DismissOnOutsidePress;
    spec.policy.restoration = FocusRestoration::SavedMenuTarget;
    spec
}

fn open(manager: &mut WindowManager, spec: WindowSpec) -> WindowId {
    match manager.apply(WindowCommand::Open(spec)).unwrap().transition {
        WindowTransition::Opened { id } => id,
        other => panic!("expected a fresh window: {other:?}"),
    }
}

fn focus(window: WindowId) -> FocusTarget {
    FocusTarget {
        window,
        control: None,
    }
}

fn target(window: WindowId) -> RouteTarget {
    RouteTarget::Window(focus(window))
}

#[test]
fn scene_membership_changes_apply_and_snapshot_together() {
    // Port: explicit ID sets select scenes; retained records never imply active input.
    // Break caught: filtering only snapshot paint while apply still sees a dormant modal.
    let mut manager = WindowManager::new();
    let active = open(&mut manager, detail(1));
    let dormant = open(&mut manager, modal());
    let old = manager.begin_frame();
    assert_eq!(
        manager.apply(WindowCommand::FocusRoot),
        Err(WindowError::RootBlockedBy(dormant))
    );
    manager
        .set_active_windows(&BTreeSet::from([active]))
        .unwrap();
    assert_eq!(manager.focused(), Some(focus(active)));
    manager.apply(WindowCommand::FocusRoot).unwrap();
    manager.apply(WindowCommand::Activate(active)).unwrap();
    manager
        .apply(WindowCommand::CapturePointer(active))
        .unwrap();
    manager
        .apply(WindowCommand::ReleasePointer(active))
        .unwrap();
    let frame = manager.begin_frame();
    assert_eq!(frame.paint_order(), &[active]);
    assert_eq!(frame.active_modal(), None);
    assert_eq!(
        frame
            .pointer_route(&BTreeSet::from([active, dormant]))
            .target,
        target(active)
    );
    assert!(frame.pointer_allows_root());
    assert_eq!(frame.keyboard_route().bubble, BubblePolicy::ToRoot);
    assert_eq!(manager.id_for(modal().key), Some(dormant));
    assert!(manager.visible_order().contains(&dormant));
    assert_eq!(old.active_modal(), Some(dormant));
    assert_eq!(old.keyboard_route().target, target(dormant));
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn active_membership_changes_command_eligibility_and_snapshot_together() {
    // Break caught: dormant windows remain eligible for focus, raise, activate or capture.
    let mut manager = WindowManager::new();
    let first = open(&mut manager, detail(1));
    let second = open(&mut manager, detail(2));
    manager
        .set_active_windows(&BTreeSet::from([first]))
        .unwrap();
    let order = manager.visible_order().to_vec();
    for command in [
        WindowCommand::Focus(focus(second)),
        WindowCommand::Raise(second),
        WindowCommand::Activate(second),
    ] {
        assert_eq!(
            manager.apply(command),
            Err(WindowError::IneligibleFocus(focus(second)))
        );
    }
    assert_eq!(
        manager.apply(WindowCommand::CapturePointer(second)),
        Err(WindowError::IneligibleCapture(second))
    );
    assert_eq!(manager.visible_order(), order);
    let frame = manager.begin_frame();
    assert!(!frame.allows_input(second));
    assert!(!frame.is_visible(second));
    assert_eq!(frame.keyboard_route().target, target(first));
    manager
        .set_active_windows(&BTreeSet::from([second]))
        .unwrap();
    manager.apply(WindowCommand::Activate(second)).unwrap();
    assert_eq!(
        manager
            .begin_frame()
            .pointer_route(&BTreeSet::from([first, second]))
            .target,
        target(second)
    );
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn scene_unknown_id_preserves_membership_and_focus() {
    // Break caught: assigning the new set before validating every ID partially changes scenes.
    let mut manager = WindowManager::new();
    let active = open(&mut manager, detail(1));
    let dormant = open(&mut manager, detail(2));
    let stale = open(&mut manager, detail(3));
    manager.apply(WindowCommand::Close(stale)).unwrap();
    manager
        .set_active_windows(&BTreeSet::from([active]))
        .unwrap();
    let control = FocusTarget {
        window: active,
        control: Some(ControlId(9)),
    };
    manager.apply(WindowCommand::Focus(control)).unwrap();
    manager
        .apply(WindowCommand::CapturePointer(active))
        .unwrap();
    let order = manager.visible_order().to_vec();
    assert_eq!(
        manager.set_active_windows(&BTreeSet::from([dormant, stale])),
        Err(WindowError::UnknownWindow(stale))
    );
    assert_eq!(manager.focused(), Some(control));
    assert_eq!(manager.capture_owner(), Some(active));
    assert_eq!(manager.visible_order(), order);
    assert_eq!(manager.begin_frame().paint_order(), &[active]);
    assert!(!manager.begin_frame().allows_input(dormant));
    let fresh = open(&mut manager, detail(4));
    assert_eq!(fresh.get(), stale.get() + 1);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn unknown_active_member_rejects_scene_update_without_mutation() {
    // Break caught: an identity from a reset session becomes an active member of a new session.
    let mut manager = WindowManager::new();
    let stale = open(&mut manager, detail(1));
    manager.reset_session();
    let current = open(&mut manager, detail(2));
    let control = FocusTarget {
        window: current,
        control: Some(ControlId(7)),
    };
    manager.apply(WindowCommand::Focus(control)).unwrap();
    assert_eq!(
        manager.set_active_windows(&BTreeSet::from([stale, current])),
        Err(WindowError::UnknownWindow(stale))
    );
    assert_eq!(manager.begin_frame().paint_order(), &[current]);
    assert_eq!(manager.focused(), Some(control));
    assert_eq!(manager.capture_owner(), None);
    assert_eq!(manager.id_for(detail(1).key), None);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn scene_restriction_removes_inactive_modal_and_focus_from_route() {
    // Break caught: a restrictive view keeps dormant modal/capture/focus or re-admits removed IDs.
    let mut manager = WindowManager::new();
    let underlay = open(&mut manager, detail(1));
    let modal = open(&mut manager, modal());
    manager.apply(WindowCommand::CapturePointer(modal)).unwrap();
    let original = manager.begin_frame();
    let view = original.restrict_to(&BTreeSet::from([underlay]));
    assert_eq!(view.paint_order(), &[underlay]);
    assert_eq!(view.active_modal(), None);
    assert_eq!(view.capture_owner(), None);
    assert_eq!(view.focused(), Some(focus(underlay)));
    assert_eq!(
        view.keyboard_route(),
        KeyboardRoute {
            target: target(underlay),
            bubble: BubblePolicy::ToRoot
        }
    );
    assert_eq!(
        view.pointer_route(&BTreeSet::from([modal])).target,
        RouteTarget::StrategicRoot
    );
    assert!(!view.allows_input(modal));
    assert!(view.pointer_allows_root());
    let twice = view.restrict_to(&BTreeSet::from([underlay, modal]));
    assert_eq!(twice.paint_order(), &[underlay]);
    assert_eq!(twice.active_modal(), None);
    assert_eq!(manager.focused(), Some(focus(modal)));
    assert_eq!(original.capture_owner(), Some(modal));
    assert_eq!(
        original.pointer_route(&BTreeSet::new()).target,
        target(modal)
    );
    let empty = view.restrict_to(&BTreeSet::new());
    assert_eq!(empty.focused(), None);
    assert_eq!(empty.keyboard_route().target, RouteTarget::StrategicRoot);
}

#[test]
fn scene_change_cancels_capture_before_focus_and_preserves_old_lease() {
    // Break caught: scene change silently drops capture, duplicates cancellation, or orders focus first.
    let mut manager = WindowManager::new();
    let first = open(&mut manager, detail(1));
    let second = open(&mut manager, detail(2));
    manager.apply(WindowCommand::Focus(focus(first))).unwrap();
    manager.apply(WindowCommand::CapturePointer(first)).unwrap();
    let old = manager.begin_frame();
    let active = BTreeSet::from([second]);
    assert_eq!(
        manager.set_active_windows(&active).unwrap(),
        vec![
            WindowEvent::CaptureCancelled {
                window: first,
                reason: CaptureCancelReason::SceneChanged
            },
            WindowEvent::FocusChanged {
                previous: Some(focus(first)),
                current: Some(focus(second))
            },
        ]
    );
    assert_eq!(manager.capture_owner(), None);
    assert_eq!(manager.begin_frame().capture_owner(), None);
    assert_eq!(old.capture_owner(), Some(first));
    assert_eq!(old.pointer_route(&BTreeSet::new()).target, target(first));
    assert!(manager.set_active_windows(&active).unwrap().is_empty());
    assert!(manager
        .apply(WindowCommand::ReleasePointer(first))
        .unwrap()
        .events
        .is_empty());
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn scene_membership_retains_dormant_history_and_new_opens_are_active() {
    // Break caught: scene changes destroy history/content or new opens inherit dormant membership.
    let mut manager = WindowManager::new();
    let hidden = open(&mut manager, detail(1));
    manager.apply(WindowCommand::Minimize(hidden)).unwrap();
    let active = open(&mut manager, detail(2));
    manager
        .set_active_windows(&BTreeSet::from([active]))
        .unwrap();
    assert_eq!(manager.presentation_history(), &[hidden]);
    assert!(manager.begin_frame().presentation_history().is_empty());
    assert_eq!(manager.id_for(detail(1).key), Some(hidden));
    let fresh = open(&mut manager, detail(3));
    assert_eq!(manager.begin_frame().paint_order(), &[active, fresh]);
    let view = manager.begin_frame().restrict_to(&BTreeSet::from([hidden]));
    assert!(view.paint_order().is_empty());
    assert!(view.presentation_history().is_empty());
    manager
        .set_active_windows(&BTreeSet::from([active, fresh, hidden]))
        .unwrap();
    assert_eq!(manager.begin_frame().presentation_history(), &[hidden]);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn active_owner_can_open_a_menu_without_a_dormant_modal_rejecting_it() {
    // Break caught: a retained modal's placement check still blocks another scene's command.
    let mut manager = WindowManager::new();
    let owner = open(&mut manager, detail(1));
    let dormant = open(&mut manager, modal());
    manager
        .set_active_windows(&BTreeSet::from([owner]))
        .unwrap();
    let opened = manager.apply(WindowCommand::Open(menu(owner)));
    assert!(
        opened.is_ok(),
        "dormant modal rejected active menu: {opened:?}"
    );
    let menu = manager.id_for(menu(owner).key).unwrap();
    assert_eq!(manager.begin_frame().paint_order(), &[owner, menu]);
    assert_eq!(manager.begin_frame().active_modal(), None);
    assert_eq!(manager.begin_frame().keyboard_route().target, target(menu));
    assert_eq!(
        manager.begin_frame().pointer_route(&BTreeSet::new()).target,
        RouteTarget::BlockedBy(menu)
    );
    assert_eq!(manager.id_for(modal().key), Some(dormant));
    assert_eq!(manager.validate(), Ok(()));
    // An explicitly mixed scene must not commit an incompatible modal/menu placement.
    assert_eq!(
        manager.set_active_windows(&BTreeSet::from([owner, menu, dormant])),
        Err(WindowError::InvalidPlacement)
    );
    assert_eq!(manager.begin_frame().paint_order(), &[owner, menu]);
    assert_eq!(manager.focused(), Some(focus(menu)));
    manager
        .set_active_windows(&BTreeSet::from([dormant]))
        .unwrap();
    assert_eq!(manager.begin_frame().paint_order(), &[dormant]);
    assert_eq!(
        manager.begin_frame().keyboard_route().target,
        target(dormant)
    );
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn active_modal_can_open_while_another_scenes_menu_is_dormant() {
    // Break caught: dormant transients prevent the next scene's modal from committing.
    let mut manager = WindowManager::new();
    let owner = open(&mut manager, detail(1));
    let dormant_menu = open(&mut manager, menu(owner));
    manager.set_active_windows(&BTreeSet::new()).unwrap();
    let opened = manager.apply(WindowCommand::Open(modal()));
    assert!(
        opened.is_ok(),
        "dormant menu rejected active modal: {opened:?}"
    );
    let modal = manager.id_for(modal().key).unwrap();
    assert_eq!(manager.begin_frame().paint_order(), &[modal]);
    assert_eq!(manager.begin_frame().active_modal(), Some(modal));
    assert_eq!(
        manager
            .begin_frame()
            .pointer_route(&BTreeSet::from([dormant_menu]))
            .target,
        RouteTarget::BlockedBy(modal)
    );
    assert_eq!(manager.validate(), Ok(()));
    manager
        .set_active_windows(&BTreeSet::from([owner, dormant_menu]))
        .unwrap();
    assert_eq!(manager.begin_frame().paint_order(), &[owner, dormant_menu]);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn reopening_an_owner_keeps_its_dormant_menu_in_the_transient_band() {
    // Break caught: moving a dormant descendant below an active modal breaks stack bands.
    for owner_active in [false, true] {
        let mut manager = WindowManager::new();
        let owner = open(&mut manager, detail(1));
        let dormant_menu = open(&mut manager, menu(owner));
        let active = if owner_active {
            BTreeSet::from([owner])
        } else {
            BTreeSet::new()
        };
        manager.set_active_windows(&active).unwrap();
        let modal = open(&mut manager, modal());
        manager.apply(WindowCommand::Open(detail(1))).unwrap();
        assert_eq!(
            manager.validate(),
            Ok(()),
            "reopen broke bands; owner_active={owner_active}"
        );
        assert_eq!(manager.visible_order().last(), Some(&dormant_menu));
        assert!(!manager.begin_frame().is_visible(dormant_menu));
        assert_eq!(manager.begin_frame().keyboard_route().target, target(modal));
        manager
            .set_active_windows(&BTreeSet::from([owner, dormant_menu]))
            .unwrap();
        assert_eq!(manager.begin_frame().paint_order(), &[owner, dormant_menu]);
        assert_eq!(manager.validate(), Ok(()));
    }
}

#[test]
fn owner_reopen_rejects_an_incompatible_active_child_reorder_atomically() {
    // Break caught: raising an owner places its active child below that owner.
    let mut manager = WindowManager::new();
    let owner_spec = detail(1);
    let owner = open(&mut manager, owner_spec);
    let mut child_spec = detail(2);
    child_spec.key = WindowKey::contextual(WindowKind::Fleet, SubjectId::new(2, 2));
    child_spec.owner = WindowOwner::Window(owner);
    let child = open(&mut manager, child_spec);
    let modal = open(&mut manager, modal());
    manager
        .set_active_windows(&BTreeSet::from([owner, child, modal]))
        .unwrap();
    let order = manager.visible_order().to_vec();
    manager.apply(WindowCommand::CapturePointer(modal)).unwrap();
    assert_eq!(
        manager.apply(WindowCommand::Open(owner_spec)),
        Err(WindowError::InvalidPlacement)
    );
    assert_eq!(manager.visible_order(), order);
    assert_eq!(manager.focused(), Some(focus(modal)));
    assert_eq!(manager.capture_owner(), Some(modal));
    assert_eq!(manager.begin_frame().paint_order(), &[owner, child, modal]);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn modal_open_and_replace_reject_an_unrelated_active_transient_atomically() {
    // Break caught: an unrelated transient subtree is mistaken for modal ancestry.
    for replacing in [false, true] {
        let mut manager = WindowManager::new();
        let owner = open(&mut manager, detail(1));
        let menu = open(&mut manager, menu(owner));
        let mut child_spec = detail(2);
        child_spec.key = WindowKey::contextual(WindowKind::Fleet, SubjectId::new(2, 2));
        child_spec.owner = WindowOwner::Window(menu);
        child_spec.policy.band = StackBand::Transient;
        child_spec.policy.placement = OpenPlacement::AboveOwner;
        let child = open(&mut manager, child_spec);
        let old = open(&mut manager, detail(3));
        manager
            .set_active_windows(&BTreeSet::from([owner, menu, child]))
            .unwrap();
        manager.apply(WindowCommand::CapturePointer(child)).unwrap();
        let order = manager.visible_order().to_vec();
        let command = if replacing {
            WindowCommand::Replace { old, new: modal() }
        } else {
            WindowCommand::Open(modal())
        };
        assert_eq!(manager.apply(command), Err(WindowError::InvalidPlacement));
        assert_eq!(manager.visible_order(), order);
        assert_eq!(manager.focused(), Some(focus(child)));
        assert_eq!(manager.capture_owner(), Some(child));
        assert_eq!(manager.id_for(modal().key), None);
        assert_eq!(manager.id_for(detail(3).key), Some(old));
        assert_eq!(manager.begin_frame().paint_order(), &[owner, menu, child]);
        assert_eq!(manager.validate(), Ok(()));
        let fresh = open(&mut manager, detail(4));
        assert_eq!(fresh.get(), old.get() + 1);
    }
}

#[test]
fn scene_update_rejects_active_children_that_only_borrow_dormant_owner_order() {
    // Break caught: owner position above a modal substitutes for actual modal ancestry.
    let mut manager = WindowManager::new();
    let owner = open(&mut manager, detail(1));
    let menu = open(&mut manager, menu(owner));
    let mut child_spec = detail(2);
    child_spec.key = WindowKey::contextual(WindowKind::Fleet, SubjectId::new(2, 2));
    child_spec.owner = WindowOwner::Window(menu);
    child_spec.policy.band = StackBand::Transient;
    child_spec.policy.placement = OpenPlacement::AboveOwner;
    let child = open(&mut manager, child_spec);
    manager.set_active_windows(&BTreeSet::new()).unwrap();
    let modal = open(&mut manager, modal());
    manager.apply(WindowCommand::CapturePointer(modal)).unwrap();
    let order = manager.visible_order().to_vec();
    assert_eq!(
        manager.set_active_windows(&BTreeSet::from([modal, child])),
        Err(WindowError::InvalidPlacement)
    );
    assert_eq!(manager.visible_order(), order);
    assert_eq!(manager.begin_frame().paint_order(), &[modal]);
    assert_eq!(manager.focused(), Some(focus(modal)));
    assert_eq!(manager.capture_owner(), Some(modal));
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn scene_modal_placement_preserves_existing_children_only_above_a_lower_modal() {
    // Break caught: admitting an unrelated upper modal, or rejecting a valid lower one.
    for placement in [OpenPlacement::TopOfBand, OpenPlacement::BottomOfBand] {
        let mut manager = WindowManager::new();
        let existing = open(&mut manager, modal());
        let child = open(&mut manager, menu(existing));
        manager
            .set_active_windows(&BTreeSet::from([existing, child]))
            .unwrap();
        manager.apply(WindowCommand::CapturePointer(child)).unwrap();
        let mut new = modal();
        new.key = WindowKey::singleton(WindowKind::MessageIndex);
        new.policy.placement = placement;
        let result = manager.apply(WindowCommand::Open(new));
        if placement == OpenPlacement::TopOfBand {
            assert_eq!(result, Err(WindowError::InvalidPlacement));
            assert_eq!(manager.visible_order(), &[existing, child]);
            assert_eq!(manager.id_for(new.key), None);
        } else {
            let commit = result.unwrap();
            let WindowTransition::Opened { id } = commit.transition else {
                panic!("expected a new lower modal: {commit:?}");
            };
            assert_eq!(manager.begin_frame().paint_order(), &[id, existing, child]);
        }
        assert_eq!(manager.begin_frame().active_modal(), Some(existing));
        assert_eq!(manager.focused(), Some(focus(child)));
        assert_eq!(manager.capture_owner(), Some(child));
        assert_eq!(manager.validate(), Ok(()));
    }
}

#[test]
fn scene_rejects_fresh_details_and_owned_modals_from_an_unrelated_dormant_owner() {
    // Break caught: a dormant owner above a modal lends eligibility to a fresh child.
    for owned_modal in [false, true] {
        let mut manager = WindowManager::new();
        let owner_spec = detail(1);
        let owner = open(&mut manager, owner_spec);
        manager.set_active_windows(&BTreeSet::new()).unwrap();
        let boundary = open(&mut manager, modal());
        manager.apply(WindowCommand::Open(owner_spec)).unwrap();
        assert_eq!(manager.visible_order(), &[boundary, owner]);
        manager
            .apply(WindowCommand::CapturePointer(boundary))
            .unwrap();
        let mut child = if owned_modal {
            let mut spec = modal();
            spec.key = WindowKey::singleton(WindowKind::MessageIndex);
            spec.policy.band = StackBand::Transient;
            spec.policy.placement = OpenPlacement::AboveOwner;
            spec
        } else {
            let mut spec = detail(2);
            spec.key = WindowKey::contextual(WindowKind::Fleet, SubjectId::new(2, 2));
            spec
        };
        child.owner = WindowOwner::Window(owner);
        assert_eq!(
            manager.apply(WindowCommand::Open(child)),
            Err(WindowError::InvalidPlacement)
        );
        assert_eq!(manager.visible_order(), &[boundary, owner]);
        assert_eq!(manager.begin_frame().paint_order(), &[boundary]);
        assert_eq!(manager.focused(), Some(focus(boundary)));
        assert_eq!(manager.capture_owner(), Some(boundary));
        assert_eq!(manager.id_for(child.key), None);
        assert_eq!(manager.validate(), Ok(()));
        let fresh = open(&mut manager, detail(3));
        assert_eq!(fresh.get(), boundary.get() + 1);
    }
}

#[test]
fn owner_reopen_with_mixed_band_modals_rejects_temporary_child_clamp_inversion() {
    // Port: bands and modality are independent; a subtree clamp must not invert bands.
    let mut manager = WindowManager::new();
    let mut owner_spec = detail(1);
    owner_spec.policy.band = StackBand::Low;
    let owner = open(&mut manager, owner_spec);
    let mut child_spec = detail(2);
    child_spec.key = WindowKey::contextual(WindowKind::Fleet, SubjectId::new(2, 2));
    child_spec.owner = WindowOwner::Window(owner);
    let child = open(&mut manager, child_spec);
    let mut normal_spec = modal();
    normal_spec.owner = WindowOwner::Window(owner);
    let normal_modal = open(&mut manager, normal_spec);
    manager.set_active_windows(&BTreeSet::new()).unwrap();
    let mut low_spec = modal();
    low_spec.key = WindowKey::singleton(WindowKind::MessageIndex);
    low_spec.policy.band = StackBand::Low;
    let low_modal = open(&mut manager, low_spec);
    manager
        .set_active_windows(&BTreeSet::from([owner, child, normal_modal, low_modal]))
        .unwrap();
    let order = [owner, low_modal, child, normal_modal];
    assert_eq!(manager.visible_order(), order);
    assert_eq!(manager.validate(), Ok(()));
    manager
        .apply(WindowCommand::CapturePointer(normal_modal))
        .unwrap();
    assert_eq!(
        manager.apply(WindowCommand::Open(owner_spec)),
        Err(WindowError::InvalidPlacement)
    );
    assert_eq!(manager.visible_order(), order);
    assert_eq!(manager.begin_frame().paint_order(), order);
    assert_eq!(manager.focused(), Some(focus(normal_modal)));
    assert_eq!(manager.capture_owner(), Some(normal_modal));
    assert_eq!(manager.validate(), Ok(()));
}
