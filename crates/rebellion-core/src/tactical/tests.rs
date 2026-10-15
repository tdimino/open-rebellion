use std::collections::HashSet;

use super::test_support::{test_fighter, test_session, test_ship};
use super::*;

// Source: FUN_005d3e90 effect dispatcher; FUN_005a7500 hit ordering.
#[test]
fn original_effect_dispatch_preserves_resources_sizes_and_priority_groups() {
    let cases = [
        (
            WeaponKind::LaserCannon,
            OriginalTacticalImpactStage::Hit,
            OriginalTacticalEffectSequence::StandardHit,
            3060,
            6,
            (32, 32),
        ),
        (
            WeaponKind::IonCannon,
            OriginalTacticalImpactStage::Hit,
            OriginalTacticalEffectSequence::IonHit,
            3180,
            6,
            (32, 32),
        ),
        (
            WeaponKind::LaserCannon,
            OriginalTacticalImpactStage::Damage,
            OriginalTacticalEffectSequence::StandardDamage,
            3120,
            7,
            (32, 32),
        ),
        (
            WeaponKind::IonCannon,
            OriginalTacticalImpactStage::Damage,
            OriginalTacticalEffectSequence::IonDamage,
            3240,
            16,
            (64, 32),
        ),
        (
            WeaponKind::Turbolaser,
            OriginalTacticalImpactStage::Damage,
            OriginalTacticalEffectSequence::TurbolaserDamage,
            3300,
            7,
            (32, 32),
        ),
        (
            WeaponKind::LaserCannon,
            OriginalTacticalImpactStage::Destroyed,
            OriginalTacticalEffectSequence::Destroyed,
            3360,
            16,
            (64, 64),
        ),
    ];
    for (weapon, stage, expected, base, frames, size) in cases {
        let actual = original_tactical_effect_sequence(weapon, stage);
        assert_eq!(actual, expected);
        assert_eq!(actual.resource_base(), base);
        assert_eq!(actual.frame_count(), frames);
        assert_eq!(actual.source_size(), size);
    }

    let mut effects = Vec::new();
    assert!(queue_original_tactical_impact(
        &mut effects,
        4,
        WeaponKind::IonCannon,
        OriginalTacticalImpactStage::Hit,
    ));
    assert!(!queue_original_tactical_impact(
        &mut effects,
        4,
        WeaponKind::LaserCannon,
        OriginalTacticalImpactStage::Hit,
    ));
    assert!(queue_original_tactical_impact(
        &mut effects,
        4,
        WeaponKind::LaserCannon,
        OriginalTacticalImpactStage::Damage,
    ));
    assert!(!queue_original_tactical_impact(
        &mut effects,
        4,
        WeaponKind::Turbolaser,
        OriginalTacticalImpactStage::Damage,
    ));
    assert!(queue_original_tactical_impact(
        &mut effects,
        4,
        WeaponKind::IonCannon,
        OriginalTacticalImpactStage::Destroyed,
    ));
    assert_eq!(
        effects[0].sequence,
        OriginalTacticalEffectSequence::Destroyed
    );
}

// Source: _DAT_0066d154 / FUN_005d3cc0 effect frame timing 0.1s.
#[test]
fn original_effect_frames_advance_at_ten_hertz_and_expire_exactly() {
    let mut session = test_session(Vec::new(), Vec::new(), true);
    queue_original_tactical_impact(
        &mut session.impact_effects,
        0,
        WeaponKind::LaserCannon,
        OriginalTacticalImpactStage::Hit,
    );
    session.advance_presentational_effects(0.099);
    assert_eq!(session.impact_effects[0].frame, 0);
    session.advance_presentational_effects(0.001_1);
    assert_eq!(session.impact_effects[0].frame, 1);
    session.advance_presentational_effects(0.5);
    assert!(session.impact_effects.is_empty());
}

// Source: FUN_005d3de0 / FUN_005b1ea0 / FUN_005b1f60 projectile profiles.
#[test]
fn original_projectile_profiles_preserve_thresholds_shapes_scales_colors_and_durations() {
    let cases = [
        (
            WeaponKind::LaserCannon,
            28.7,
            Some(0),
            true,
            (0.5, 1, 0, 1.0),
        ),
        (
            WeaponKind::LaserCannon,
            ORIGINAL_LASER_PROJECTILE_THRESHOLD,
            Some(0),
            true,
            (0.5, 2, 0, 1.0),
        ),
        (
            WeaponKind::Turbolaser,
            30.0,
            Some(15),
            false,
            (0.65, 1, 1, 1.0),
        ),
        (
            WeaponKind::Turbolaser,
            40.0,
            Some(14),
            false,
            (0.75, 2, 1, 1.0),
        ),
        (
            WeaponKind::Turbolaser,
            40.0,
            Some(15),
            false,
            (1.0, 3, 1, 2.0),
        ),
        (
            WeaponKind::IonCannon,
            ORIGINAL_ION_PROJECTILE_THRESHOLD,
            Some(0),
            true,
            (1.0, 3, 2, 2.0),
        ),
        (
            WeaponKind::FighterAttack,
            10.0,
            None,
            true,
            (0.2, 1, 0, 1.0),
        ),
    ];
    for (weapon, strength, ordinal, alliance, expected) in cases {
        let profile = original_projectile_profile(weapon, strength, ordinal, alliance);
        assert_eq!(
            (
                profile.longitudinal_scale,
                profile.shape_variant,
                profile.color_selector,
                profile.duration,
            ),
            expected
        );
    }
}

// Source: FUN_005ee590 / LAB_005eeb90 projectile interpolation.
#[test]
fn retained_projectile_interpolates_and_expires_by_source_duration() {
    let ships = vec![test_ship(64, 0, true, true), test_ship(128, 0, false, true)];
    let mut session = test_session(ships, Vec::new(), true);
    assert!(queue_original_tactical_projectile(
        &mut session.weapon_effects,
        &session.ships,
        0,
        1,
        WeaponKind::LaserCannon,
        ORIGINAL_LASER_PROJECTILE_THRESHOLD,
    ));
    assert_eq!(session.weapon_effects[0].shape_variant, 2);
    session.advance_presentational_effects(0.5);
    assert_eq!(session.weapon_effects[0].progress(), 0.5);
    session.advance_presentational_effects(0.499);
    assert_eq!(session.weapon_effects.len(), 1);
    session.advance_presentational_effects(0.002);
    assert!(session.weapon_effects.is_empty());
}

// Source: FUN_005b24d0 through FUN_005b2550 field effects.
#[test]
fn gravity_field_preempts_tractor_then_restores_it_at_ten_hertz() {
    let mut session = test_session(Vec::new(), Vec::new(), true);
    assert!(set_original_tactical_field(
        &mut session.field_effects,
        1,
        4,
        OriginalTacticalFieldKind::Tractor,
        true,
    ));
    assert_eq!(session.field_effects[0].resource_id(), Some(3620));
    session.advance_presentational_effects(0.1);
    assert_eq!(session.field_effects[0].resource_id(), Some(3621));
    assert!(set_original_tactical_field(
        &mut session.field_effects,
        2,
        4,
        OriginalTacticalFieldKind::Gravity,
        true,
    ));
    assert_eq!(session.field_effects[0].resource_id(), Some(3520));
    assert!(set_original_tactical_field(
        &mut session.field_effects,
        3,
        4,
        OriginalTacticalFieldKind::Tractor,
        true,
    ));
    assert_eq!(session.field_effects[0].source_counts(), (2, 1));
    assert!(set_original_tactical_field(
        &mut session.field_effects,
        2,
        4,
        OriginalTacticalFieldKind::Gravity,
        false,
    ));
    assert_eq!(session.field_effects[0].resource_id(), Some(3620));
    assert!(set_original_tactical_field(
        &mut session.field_effects,
        1,
        4,
        OriginalTacticalFieldKind::Tractor,
        false,
    ));
    assert!(set_original_tactical_field(
        &mut session.field_effects,
        3,
        4,
        OriginalTacticalFieldKind::Tractor,
        false,
    ));
    assert!(session.field_effects.is_empty());
}

// Source: FUN_005b24d0 through FUN_005b2550 field source limits.
#[test]
fn exact_field_sources_enforce_one_tractor_and_four_gravity_targets() {
    let ships = (0..6)
        .map(|index| test_ship(64, index, true, true))
        .collect();
    let mut session = test_session(ships, Vec::new(), true);

    assert!(session.set_tractor_field(0, 1, true));
    assert!(!session.set_tractor_field(0, 1, true));
    assert!(!session.set_tractor_field(0, 2, true));
    assert!(!session.set_tractor_field(2, 1, false));
    assert!(session.set_tractor_field(0, 1, false));
    assert!(session.set_tractor_field(0, 2, true));

    for target in 1..=4 {
        assert!(session.set_gravity_field(0, target, true));
    }
    assert!(!session.set_gravity_field(0, 1, true));
    assert!(!session.set_gravity_field(0, 5, true));
    assert!(!session.set_gravity_field(1, 4, false));
    assert!(session.set_gravity_field(0, 4, false));
    assert!(session.set_gravity_field(0, 5, true));

    let target_two = session
        .field_effects
        .iter()
        .find(|effect| effect.target == 2)
        .expect("target two should retain both field kinds");
    assert_eq!(target_two.source_ids(), (&[0][..], &[0][..]));
    assert_eq!(target_two.resource_id(), Some(3520));
}

// Source: FUN_005e45f0 / FUN_005e7540 subsystem condition indicators.
#[test]
fn subsystem_condition_uses_exact_five_source_bands() {
    let condition = TacticalSubsystemCondition {
        shields: 0,
        weapons: 24,
        tractor: 25,
        engines: 50,
        hyperdrive: 75,
    };
    assert_eq!(condition.resource_ids(), [1201, 1207, 1213, 1219, 1225]);
    assert_eq!(original_tactical_condition_band(1), 1);
    assert_eq!(original_tactical_condition_band(49), 2);
    assert_eq!(original_tactical_condition_band(74), 3);
    assert_eq!(original_tactical_condition_band(100), 4);
}

// Source: FUN_005b0e50 / FUN_005b3f10 subsystem condition from damage.
#[test]
fn subsystem_condition_follows_hull_hits_and_hyperdrive_capacity() {
    let condition = TacticalSubsystemCondition::from_source_state(
        980,
        1000,
        TacticalSubsystemCapacity {
            shields: true,
            weapons: true,
            tractor: true,
            engines: true,
            hyperdrive: 2,
        },
        TacticalSubsystemDamage {
            shields: 1,
            weapons: 2,
            tractor: 3,
            engines: 4,
            hyperdrive: 1,
        },
    );
    assert_eq!(condition.percentages(), [73, 48, 23, 0, 50]);
    assert_eq!(condition.resource_ids(), [1204, 1208, 1212, 1216, 1224]);

    let unavailable = TacticalSubsystemCondition::from_source_state(
        100,
        100,
        TacticalSubsystemCapacity {
            shields: false,
            weapons: false,
            tractor: false,
            engines: false,
            hyperdrive: 0,
        },
        TacticalSubsystemDamage::default(),
    );
    assert_eq!(unavailable.percentages(), [0; 5]);
}

// Source: FUN_005b1490 repair kind selection in hit order.
#[test]
fn subsystem_repair_selection_preserves_source_hit_order() {
    let damage = TacticalSubsystemDamage {
        shields: 1,
        weapons: 1,
        tractor: 2,
        engines: 2,
        hyperdrive: 1,
    };
    assert_eq!(damage.total_hits(), 7);
    let expected = [
        TacticalSubsystemKind::Engines,
        TacticalSubsystemKind::Engines,
        TacticalSubsystemKind::Shields,
        TacticalSubsystemKind::Hyperdrive,
        TacticalSubsystemKind::Tractor,
        TacticalSubsystemKind::Tractor,
        TacticalSubsystemKind::Weapons,
    ];
    for (index, kind) in expected.into_iter().enumerate() {
        assert_eq!(
            original_subsystem_repair_kind(damage, index as u16 + 1),
            Some(kind)
        );
    }
    assert_eq!(original_subsystem_repair_kind(damage, 0), None);
    assert_eq!(original_subsystem_repair_kind(damage, 8), None);
}

// Source: FUN_005b1490 repair chance; FUN_005b0330 repair interval.
#[test]
fn subsystem_repair_uses_inclusive_damage_control_chance() {
    let mut ship = test_ship(64, 0, true, true);
    ship.subsystem_capacity.engines = true;
    ship.subsystem_damage.engines = 2;
    ship.damage_control = 40;
    ship.refresh_subsystem_condition();
    assert_eq!(attempt_original_subsystem_repair(&mut ship, 0, 0, 1), None);
    assert_eq!(attempt_original_subsystem_repair(&mut ship, 0, 41, 1), None);
    let repair = attempt_original_subsystem_repair(&mut ship, 0, 40, 1)
        .expect("inclusive chance boundary should repair one engine hit");
    assert_eq!(repair.kind, TacticalSubsystemKind::Engines);
    assert_eq!((repair.hits_before, repair.hits_after), (2, 1));
    assert_eq!(ship.subsystem_damage.engines, 1);
    assert_eq!(ship.subsystem_condition.engines, 75);
    assert_eq!(ORIGINAL_SUBSYSTEM_REPAIR_INTERVAL_TICKS, 50 * 4);
}

// Source: FUN_005b05c0 sublight conversion; FUN_005ad750 engine mode.
#[test]
fn engine_condition_uses_damage_mode_and_active_tractor_power_not_hull() {
    let mut target = test_ship(64, 0, true, true);
    target.hull_current = 10;
    target.hull_max = 100;
    target.subsystem_capacity.engines = true;
    target.sublight_engine_power = 100.0;
    target.maneuver_state_value = Some(4.0);
    target.subsystem_damage.engines = 1;

    let mut source = test_ship(128, 0, false, true);
    source.hull_current = 50;
    source.hull_max = 100;
    source.subsystem_capacity.tractor = true;
    source.tractor_beam_power = 40.0;
    source.subsystem_damage.tractor = 1;

    let mut session = test_session(vec![target, source], Vec::new(), true);
    assert!(session.set_tractor_field(1, 0, true));
    let held = session.subsystem_mobility(0).unwrap();
    assert_eq!(held.base_engine_power, 100.0);
    assert_eq!(held.engine_mode_bonus, 5.0);
    assert_eq!(held.active_tractor_power, 10.0);
    assert_eq!(held.effective_engine_power, 68.75);
    assert_eq!(held.engine_percent, 68);
    assert_eq!(session.ships[0].subsystem_condition.engines, 68);

    assert!(session.set_tractor_field(1, 0, false));
    let released = session.subsystem_mobility(0).unwrap();
    assert_eq!(released.active_tractor_power, 0.0);
    assert_eq!(released.effective_engine_power, 78.75);
    assert_eq!(released.engine_percent, 78);
    assert_eq!(session.ships[0].subsystem_condition.engines, 78);
}

// Source: FUN_005ad750 / FUN_005afb70 engine mode bonus clamp.
#[test]
fn maneuver_state_produces_the_source_clamped_one_through_nine_bonus() {
    assert_eq!(original_engine_mode_bonus(None), 1.0);
    assert_eq!(original_engine_mode_bonus(Some(0.0)), 9.0);
    assert_eq!(original_engine_mode_bonus(Some(4.0)), 5.0);
    assert_eq!(original_engine_mode_bonus(Some(8.0)), 1.0);
    assert_eq!(original_engine_mode_bonus(Some(9.0)), 1.0);
    assert_eq!(original_engine_mode_bonus(Some(-1.0)), 9.0);
}

// Source: FUN_005cd460 turn; FUN_005b2f30 movement and collision.
#[test]
fn movement_uses_effective_power_alignment_and_millisecond_integration() {
    let mut alliance = test_ship(64, 0, true, true);
    alliance.subsystem_capacity.engines = true;
    alliance.source_position.z = -56.0;
    alliance.sublight_engine_power = 100.0;
    alliance.maneuver_state_value = Some(4.0);
    alliance.subsystem_damage.engines = 1;

    let mut empire = test_ship(128, 0, false, true);
    empire.subsystem_capacity.engines = true;
    empire.source_position.z = 56.0;
    empire.sublight_engine_power = 100.0;
    empire.maneuver_state_value = Some(4.0);
    empire.subsystem_damage.engines = 1;

    let mut session = test_session(vec![alliance, empire], Vec::new(), true);
    session.advance_original_tactical_movement(250.0);
    assert_eq!(session.ships[0].source_velocity.z, 78.75);
    assert_eq!(session.ships[1].source_velocity.z, -78.75);
    assert_eq!(session.ships[0].source_position.z, -36.3125);
    assert_eq!(session.ships[1].source_position.z, 36.3125);

    session.ships[0].source_desired_forward = TacticalWorldVector {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    session.advance_original_tactical_movement(250.0);
    assert_eq!(session.ships[0].source_velocity, TacticalWorldVector::ZERO);
    assert_eq!(session.ships[0].source_position.z, -36.3125);

    session.ships[0].source_forward = TacticalWorldVector {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    session.ships[0].source_desired_forward = session.ships[0].source_forward;
    session.advance_original_tactical_movement(250.0);
    assert_eq!(session.ships[0].source_velocity.y, 78.75);
    assert_eq!(session.ships[0].source_position.y, 19.6875);
}

// Source: FUN_005b2e60 collision volume overlap test.
#[test]
fn collision_predicate_preserves_strict_source_boundaries() {
    let envelope = OriginalTacticalCollisionEnvelope {
        vertical_radius: 1.0,
        planar_diameter: 2.0,
    };
    let mut first = test_ship(64, 0, true, true);
    first.source_collision_envelope = Some(envelope);
    let mut second = test_ship(128, 0, false, true);
    second.source_collision_envelope = Some(envelope);

    second.source_position.z = 1.999;
    assert!(original_tactical_objects_overlap(&first, &second));
    second.source_position.z = 2.0;
    assert!(!original_tactical_objects_overlap(&first, &second));
    second.source_position.z = 0.0;
    second.source_position.y = 1.999;
    assert!(original_tactical_objects_overlap(&first, &second));
    second.source_position.y = 2.0;
    assert!(!original_tactical_objects_overlap(&first, &second));
}

// Source: FUN_005b2f30 / FUN_005b2e60 collision position restore.
#[test]
fn movement_restores_previous_position_when_mesh_envelopes_overlap() {
    let envelope = OriginalTacticalCollisionEnvelope {
        vertical_radius: 1.0,
        planar_diameter: 2.0,
    };
    let mut moving = test_ship(64, 0, true, true);
    moving.subsystem_capacity.engines = true;
    moving.sublight_engine_power = 4.0;
    moving.source_collision_envelope = Some(envelope);
    moving.source_position.z = 0.0;

    let mut fixed = test_ship(128, 0, false, true);
    fixed.source_collision_envelope = Some(envelope);
    fixed.source_position.z = 2.0;

    let mut session = test_session(vec![moving, fixed], Vec::new(), true);
    session.advance_original_tactical_movement(250.0);
    assert_eq!(session.ships[0].source_position.z, 0.0);
    assert_eq!(session.ships[0].source_velocity.z, 5.0);
}

// Source: FUN_005ba270 maneuverability; FUN_005cd460 turn rate.
#[test]
fn current_to_desired_turn_uses_source_rates_and_normalization() {
    let close = |left: f32, right: f32| assert!((left - right).abs() < 0.000_01);
    close(original_capital_turn_rate(0), std::f32::consts::PI / 12.0);
    close(
        original_capital_turn_rate(4),
        5.0 * std::f32::consts::PI / 18.0,
    );
    close(original_capital_turn_rate(20), std::f32::consts::PI / 2.0);

    let mut ship = test_ship(64, 0, true, true);
    ship.maneuverability = 4;
    ship.source_desired_forward = TacticalWorldVector {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    turn_original_tactical_ship(&mut ship, 250.0);
    let turn = 5.0 * std::f32::consts::PI / 72.0;
    close(ship.source_forward.x, turn.sin());
    close(ship.source_forward.y, 0.0);
    close(ship.source_forward.z, turn.cos());
    close(
        ship.source_forward
            .x
            .hypot(ship.source_forward.y)
            .hypot(ship.source_forward.z),
        1.0,
    );

    turn_original_tactical_ship(&mut ship, 10_000.0);
    assert_eq!(ship.source_forward, ship.source_desired_forward);
}

// Source: FUN_005a8f70 waypoint arrival; FUN_005cf980 recovery states.
#[test]
fn waypoint_arrival_and_fighter_docking_complete_source_executors() {
    let mut carrier = test_ship(64, 0, true, true);
    carrier.subsystem_capacity.engines = true;
    carrier.source_desired_forward = TacticalWorldVector::SOURCE_FORWARD;
    carrier.source_waypoint = Some(TacticalWorldPosition {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    });
    carrier.order = TacticalOrder::Anvil;

    let mut fighter = test_fighter(1, true);
    fighter.source_position.z = 3.5;
    fighter.sublight_engine_power = 4.0;
    fighter.order = TacticalOrder::Recover;
    fighter.recovery_state = TacticalFighterRecoveryState::Returning;
    fighter.recovery_target = Some(0);

    let mut session = test_session(vec![carrier], vec![fighter], true);
    session.advance_original_tactical_movement(250.0);
    assert_eq!(
        session.ships[0].source_position,
        TacticalWorldPosition {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    );
    assert_eq!(session.ships[0].source_waypoint, None);
    assert_eq!(session.ships[0].source_velocity, TacticalWorldVector::ZERO);
    assert_eq!(session.ships[0].order, TacticalOrder::None);

    session.advance_original_fighter_recovery(250.0);
    assert_eq!(session.fighters[0].source_position.z, 2.5);
    assert_eq!(
        session.fighters[0].recovery_state,
        TacticalFighterRecoveryState::Docking
    );
    assert!(session.fighters[0].alive);

    session.advance_original_fighter_recovery(250.0);
    assert_eq!(
        session.fighters[0].recovery_state,
        TacticalFighterRecoveryState::Recovered
    );
    assert!(!session.fighters[0].alive);
    assert_eq!(session.fighters[0].squad_count, 12);
    assert_eq!(session.fighters[0].order, TacticalOrder::None);
    assert_eq!(
        session.take_pending_voice_cues(),
        vec![
            TacticalVoiceCue::capital_maneuver_complete(TacticalVoiceFaction::Alliance, 0,),
            TacticalVoiceCue::fighter_recovery_complete(TacticalVoiceFaction::Alliance, 0,),
        ]
    );
}

// Source: FUN_005cf980 / FUN_005cf940 fighter recovery state.
#[test]
fn fighter_recovery_warns_each_player_group_once_when_carriers_are_full() {
    let mut carrier = test_ship(64, 0, true, true);
    carrier.fighter_capacity = 1;
    let mut reserved = test_fighter(1, true);
    reserved.recovery_state = TacticalFighterRecoveryState::Returning;
    reserved.recovery_target = Some(0);
    let mut first = test_fighter(1, true);
    first.fleet_squadron_index = 1;
    first.selected = true;
    let mut second = test_fighter(1, true);
    second.fleet_squadron_index = 2;
    second.selected = true;
    let mut enemy = test_fighter(5, false);
    enemy.selected = true;
    let mut session = test_session(vec![carrier], vec![reserved, first, second, enemy], true);

    begin_original_fighter_recovery(&mut session, &[1, 2, 3]);

    assert!(session.fighters[1..].iter().all(|fighter| {
        fighter.recovery_state == TacticalFighterRecoveryState::AwaitingCarrier
    }));
    assert_eq!(
        session.take_pending_voice_cues(),
        vec![TacticalVoiceCue::fighter_recovery_capacity(
            TacticalVoiceFaction::Alliance,
            0,
        )]
    );
}

// Source: FUN_005cf980 fighter recovery completion.
#[test]
fn fighter_recovery_completion_waits_for_the_last_live_group_member() {
    let carrier = test_ship(64, 0, true, true);
    let mut docking = test_fighter(1, true);
    docking.recovery_state = TacticalFighterRecoveryState::Docking;
    docking.recovery_target = Some(0);
    let mut still_returning = test_fighter(1, true);
    still_returning.fleet_squadron_index = 1;
    still_returning.recovery_state = TacticalFighterRecoveryState::Returning;
    still_returning.recovery_target = Some(0);
    still_returning.source_position.z = 8.0;
    let mut session = test_session(vec![carrier], vec![docking, still_returning], true);

    session.advance_original_fighter_recovery(250.0);

    assert!(!session.fighters[0].alive);
    assert!(session.fighters[1].alive);
    assert!(session.take_pending_voice_cues().is_empty());

    session.fighters[1].recovery_state = TacticalFighterRecoveryState::Docking;
    session.advance_original_fighter_recovery(250.0);

    assert!(!session.fighters[1].alive);
    assert_eq!(
        session.take_pending_voice_cues(),
        vec![TacticalVoiceCue::fighter_recovery_complete(
            TacticalVoiceFaction::Alliance,
            0,
        )]
    );
}

// Source: _DAT_0066d07c / _DAT_0066d094 / _DAT_0066d098 maneuver params.
#[test]
fn maneuver_waypoints_preserve_source_rotations_and_scale() {
    let current = TacticalWorldPosition {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let target = TacticalWorldPosition {
        x: 9.0,
        y: 6.0,
        z: 15.0,
    };
    let (sin, cos) = ORIGINAL_MANEUVER_ANGLE.sin_cos();
    let close = |left: f32, right: f32| assert!((left - right).abs() < 0.000_01);

    let left = original_maneuver_waypoint(TacticalOrder::LeftHook, current, target).unwrap();
    close(left.x, 1.0 + (cos * 8.0 - sin * 12.0) * 0.75);
    close(left.y, 5.0);
    close(left.z, 3.0 + (cos * 12.0 + sin * 8.0) * 0.75);

    let right = original_maneuver_waypoint(TacticalOrder::RightHook, current, target).unwrap();
    close(right.x, 1.0 + (cos * 8.0 + sin * 12.0) * 0.75);
    close(right.y, 5.0);
    close(right.z, 3.0 + (cos * 12.0 - sin * 8.0) * 0.75);

    let hammer = original_maneuver_waypoint(TacticalOrder::Hammer, current, target).unwrap();
    close(hammer.x, 7.0);
    close(hammer.y, 2.0 + (cos * 4.0 - sin * 12.0) * 0.75);
    close(hammer.z, 3.0 + (cos * 12.0 + sin * 4.0) * 0.75);

    let anvil = original_maneuver_waypoint(TacticalOrder::Anvil, current, target).unwrap();
    close(anvil.x, 7.0);
    close(anvil.y, 2.0 + (cos * 4.0 + sin * 12.0) * 0.75);
    close(anvil.z, 3.0 + (cos * 12.0 - sin * 4.0) * 0.75);
}

// Source: FUN_005d0bb0 / FUN_005a8fc0 target lifecycle.
#[test]
fn attack_target_lifecycle_reacquires_same_class_and_clears_exhausted_lists() {
    let mut player_ship = test_ship(64, 0, true, true);
    player_ship.order = TacticalOrder::AttackCapitalShips;
    player_ship.attack_target = Some(TacticalAttackTarget::CapitalShip(1));
    let mut retreating_enemy = test_ship(128, 0, false, true);
    retreating_enemy.retreating = true;
    let live_enemy = test_ship(129, 1, false, true);

    let mut player_fighter = test_fighter(1, true);
    player_fighter.order = TacticalOrder::AttackFighters;
    player_fighter.attack_target = Some(TacticalAttackTarget::FighterGroup(1));
    let mut recovered_enemy_fighter = test_fighter(5, false);
    recovered_enemy_fighter.recovery_state = TacticalFighterRecoveryState::Recovered;
    let live_enemy_fighter = test_fighter(6, false);
    let mut session = test_session(
        vec![player_ship, retreating_enemy, live_enemy],
        vec![player_fighter, recovered_enemy_fighter, live_enemy_fighter],
        true,
    );

    refresh_original_attack_targets(&mut session);
    assert_eq!(
        session.ships[0].attack_target,
        Some(TacticalAttackTarget::CapitalShip(2))
    );
    assert_eq!(
        session.fighters[0].attack_target,
        Some(TacticalAttackTarget::FighterGroup(2))
    );

    session.ships[2].alive = false;
    session.fighters[2].squad_count = 0;
    refresh_original_attack_targets(&mut session);
    assert_eq!(session.ships[0].attack_target, None);
    assert_eq!(session.fighters[0].attack_target, None);
    assert_eq!(
        session.take_pending_voice_cues(),
        vec![
            TacticalVoiceCue::capital_orders_complete(TacticalVoiceFaction::Alliance, 0,),
            TacticalVoiceCue::fighter_orders_complete(TacticalVoiceFaction::Alliance, 0,),
        ]
    );

    session.ships[0].order = TacticalOrder::AttackFighters;
    refresh_original_attack_targets(&mut session);
    assert_eq!(
        session.ships[0].attack_target, None,
        "an exhausted fighter list must not fall through to a capital target"
    );
    assert!(session.take_pending_voice_cues().is_empty());
}

// Source: FUN_005cf980 recovery state reset.
#[test]
fn recovered_fighter_clears_attack_fighters_without_capital_fire_or_audio() {
    fn session_with_source_order(order: TacticalOrder) -> BattleSession {
        let mut source = test_ship(64, 0, true, true);
        source.order = order;
        source.attack_target = (order == TacticalOrder::AttackFighters)
            .then_some(TacticalAttackTarget::FighterGroup(0));
        source.weapon_arcs[0] = TacticalWeaponArc::new(300, 0, 0);
        source.weapon_ranges.laser_cannon = 100.0;

        let mut capital = test_ship(128, 0, false, true);
        capital.hull_current = 1_000;
        capital.hull_max = 1_000;
        capital.shield = 1_000;
        capital.shield_max = 1_000;

        // FUN_005cf980 advances Docking (state 3) to Recovered (state 4)
        // without removing the strategic squadron count. FUN_005d0bb0
        // and FUN_005a8fc0 keep Attack Fighters bound to class 1, so the
        // resulting empty typed list must not fall through to a capital.
        let mut exhausted_fighter = test_fighter(5, false);
        exhausted_fighter.recovery_state = TacticalFighterRecoveryState::Docking;
        exhausted_fighter.squad_count = 12;

        let mut session = test_session(vec![source, capital], vec![exhausted_fighter], true);
        session.paused = false;
        session
    }

    let mut ordered = session_with_source_order(TacticalOrder::AttackFighters);
    let ordered_capital_before = (ordered.ships[1].hull_current, ordered.ships[1].shield);

    ordered.step();

    assert_eq!(
        ordered.fighters[0].recovery_state,
        TacticalFighterRecoveryState::Recovered
    );
    assert!(!ordered.fighters[0].alive);
    assert_eq!(ordered.fighters[0].squad_count, 12);
    assert_eq!(ordered.ships[0].attack_target, None);
    assert_eq!(
        (ordered.ships[1].hull_current, ordered.ships[1].shield),
        ordered_capital_before
    );
    assert!(ordered.weapon_effects.is_empty());
    assert!(ordered.impact_effects.is_empty());
    assert!(ordered.pending_audio_cues.is_empty());

    let mut autonomous = session_with_source_order(TacticalOrder::None);
    let autonomous_capital_before = (autonomous.ships[1].hull_current, autonomous.ships[1].shield);

    autonomous.step();

    assert_ne!(
        (autonomous.ships[1].hull_current, autonomous.ships[1].shield),
        autonomous_capital_before
    );
    assert_eq!(autonomous.weapon_effects.len(), 1);
    assert_eq!(autonomous.impact_effects.len(), 1);
    assert!(!autonomous.pending_audio_cues.is_empty());
}

// Source: FUN_005d0bb0 / FUN_005a8fc0 explicit attack targeting.
#[test]
fn explicit_attack_order_never_redirects_an_invalid_target() {
    let mut source = test_ship(64, 0, true, true);
    source.order = TacticalOrder::AttackCapitalShips;
    source.attack_target = Some(TacticalAttackTarget::CapitalShip(1));
    source.weapon_arcs[0] = TacticalWeaponArc::new(300, 0, 0);
    source.weapon_ranges.laser_cannon = 100.0;
    let dead_target = test_ship(128, 0, false, false);
    let live_target = test_ship(129, 1, false, true);
    let mut ships = vec![source, dead_target, live_target];
    let original_hull = ships[2].hull_current;
    let mut fields = Vec::new();
    let mut projectiles = Vec::new();
    let mut impacts = Vec::new();
    let mut audio_cues = Vec::new();
    let mut voice_cues = Vec::new();

    BattleSession::fire_side(
        &mut ships,
        &mut fields,
        &[0],
        &[2],
        &mut projectiles,
        &mut impacts,
        &mut audio_cues,
        &mut voice_cues,
        true,
        TacticalVoiceFaction::Alliance,
        &mut OriginalTacticalRng::new(1),
    );

    assert_eq!(ships[2].hull_current, original_hull);
    assert!(projectiles.is_empty());
    assert!(impacts.is_empty());
    assert!(audio_cues.is_empty());
}

// Source: FUN_005b5100 / FUN_005b5b60 / FUN_005b5f50 hull damage selection.
#[test]
fn hull_subsystem_selection_preserves_every_source_boundary() {
    assert_eq!(original_hull_subsystem_kind(50.0), None);
    assert_eq!(
        original_hull_subsystem_kind(50.001),
        Some(TacticalSubsystemKind::Shields)
    );
    assert_eq!(
        original_hull_subsystem_kind(65.0),
        Some(TacticalSubsystemKind::Shields)
    );
    assert_eq!(
        original_hull_subsystem_kind(65.001),
        Some(TacticalSubsystemKind::Weapons)
    );
    assert_eq!(
        original_hull_subsystem_kind(80.001),
        Some(TacticalSubsystemKind::Tractor)
    );
    assert_eq!(
        original_hull_subsystem_kind(90.001),
        Some(TacticalSubsystemKind::Engines)
    );
    assert_eq!(
        original_hull_subsystem_kind(95.001),
        Some(TacticalSubsystemKind::Hyperdrive)
    );
    assert_eq!(original_hull_subsystem_kind(100.001), None);
}

// Source: FUN_005b54d0 shield damage probability.
#[test]
fn shield_damage_uses_inclusive_source_probability() {
    let mut ship = test_ship(64, 0, true, true);
    ship.hull_current = 1000;
    ship.hull_max = 1000;
    ship.shield = 100;
    ship.shield_max = 100;
    ship.subsystem_capacity.shields = true;
    ship.refresh_subsystem_condition();
    let mut ships = vec![ship.clone()];
    let mut fields = Vec::new();
    apply_original_capital_damage(
        &mut ships,
        &mut fields,
        0,
        20,
        OriginalTacticalDamageRolls {
            shield: 20,
            hull: 0,
        },
    );
    assert_eq!(ships[0].subsystem_damage.shields, 1);
    assert_eq!(ships[0].subsystem_condition.shields, 75);

    let mut ships = vec![ship];
    apply_original_capital_damage(
        &mut ships,
        &mut fields,
        0,
        20,
        OriginalTacticalDamageRolls {
            shield: 21,
            hull: 0,
        },
    );
    assert_eq!(ships[0].subsystem_damage.shields, 0);
    assert_eq!(ships[0].subsystem_condition.shields, 100);
}

// Source: FUN_005b5100 / FUN_005b5b60 / FUN_005b5f50 hull damage subsystems.
#[test]
fn hull_damage_mutates_each_source_subsystem_category() {
    let cases = [
        (
            51,
            TacticalSubsystemDamage {
                shields: 1,
                ..TacticalSubsystemDamage::default()
            },
        ),
        (
            66,
            TacticalSubsystemDamage {
                weapons: 1,
                ..TacticalSubsystemDamage::default()
            },
        ),
        (
            81,
            TacticalSubsystemDamage {
                tractor: 1,
                ..TacticalSubsystemDamage::default()
            },
        ),
        (
            91,
            TacticalSubsystemDamage {
                engines: 1,
                ..TacticalSubsystemDamage::default()
            },
        ),
        (
            96,
            TacticalSubsystemDamage {
                hyperdrive: 1,
                ..TacticalSubsystemDamage::default()
            },
        ),
    ];
    for (roll, expected) in cases {
        let mut ship = test_ship(64, 0, true, true);
        ship.hull_current = 1000;
        ship.hull_max = 1000;
        ship.subsystem_capacity = TacticalSubsystemCapacity {
            shields: true,
            weapons: true,
            tractor: true,
            engines: true,
            hyperdrive: 2,
        };
        ship.refresh_subsystem_condition();
        let mut ships = vec![ship];
        let stage = apply_original_capital_damage(
            &mut ships,
            &mut Vec::new(),
            0,
            1,
            OriginalTacticalDamageRolls {
                shield: 100,
                hull: roll,
            },
        );
        assert_eq!(stage, OriginalTacticalImpactStage::Damage);
        assert_eq!(ships[0].hull_current, 999);
        assert_eq!(ships[0].subsystem_damage, expected);
    }
}

// Source: FUN_005b5100 hull damage subsystem destruction path.
#[test]
fn destroyed_tractor_subsystem_clears_its_live_field_source() {
    let mut source = test_ship(64, 0, true, true);
    source.hull_current = 1000;
    source.hull_max = 1000;
    source.subsystem_capacity.tractor = true;
    source.subsystem_damage.tractor = 3;
    source.refresh_subsystem_condition();
    let target = test_ship(128, 0, false, true);
    let mut ships = vec![source, target];
    let mut fields = Vec::new();
    assert!(set_original_tactical_field(
        &mut fields,
        0,
        1,
        OriginalTacticalFieldKind::Tractor,
        true,
    ));

    apply_original_capital_damage(
        &mut ships,
        &mut fields,
        0,
        1,
        OriginalTacticalDamageRolls {
            shield: 100,
            hull: 81,
        },
    );
    assert_eq!(ships[0].subsystem_damage.tractor, 4);
    assert_eq!(ships[0].subsystem_condition.tractor, 0);
    assert!(fields.is_empty());
}

// Source: FUN_005b6530 weapon arc classifier.
#[test]
fn weapon_arc_classifier_preserves_source_order_and_boundaries() {
    let source = test_ship(64, 0, true, true);
    let target_at = |x: f32, z: f32| {
        let mut target = test_ship(128, 0, false, true);
        target.source_position = TacticalWorldPosition { x, y: 0.0, z };
        target
    };

    assert_eq!(
        original_tactical_weapon_arc(&source, &target_at(0.0, 10.0)),
        OriginalTacticalWeaponArc::Fore
    );
    assert_eq!(
        original_tactical_weapon_arc(&source, &target_at(10.0, 0.0)),
        OriginalTacticalWeaponArc::Starboard
    );
    assert_eq!(
        original_tactical_weapon_arc(&source, &target_at(0.0, -10.0)),
        OriginalTacticalWeaponArc::Aft
    );
    assert_eq!(
        original_tactical_weapon_arc(&source, &target_at(-10.0, 0.0)),
        OriginalTacticalWeaponArc::Port
    );
    assert_eq!(
        original_tactical_weapon_arc(&source, &target_at(10.0, 10.0)),
        OriginalTacticalWeaponArc::Fore,
        "the +45 degree boundary belongs to the fore arc"
    );
    assert_eq!(
        original_tactical_weapon_arc(&source, &target_at(10.0, -10.0)),
        OriginalTacticalWeaponArc::Starboard,
        "the +135 degree boundary remains lateral"
    );
    assert_eq!(
        original_tactical_weapon_arc(&source, &target_at(-10.0, -10.0)),
        OriginalTacticalWeaponArc::Port,
        "the -135 degree boundary remains lateral"
    );
}

// Source: FUN_005b3a40 / FUN_005b3f10 / FUN_005b6530 weapon fire.
#[test]
fn direct_weapon_callback_fires_every_family_and_queues_the_arc_once() {
    let mut source = test_ship(64, 0, true, true);
    source.hull_current = 1000;
    source.hull_max = 1000;
    source.weapon_arcs[0] = TacticalWeaponArc::new(60, 40, 80);
    source.weapon_ranges = TacticalWeaponRanges {
        laser_cannon: 100.0,
        ion_cannon: 100.0,
        turbolaser: 100.0,
    };
    source.order = TacticalOrder::AttackCapitalShips;
    source.attack_target = Some(TacticalAttackTarget::CapitalShip(1));
    let mut target = test_ship(128, 0, false, true);
    target.source_position.z = 10.0;
    target.hull_current = 1000;
    target.hull_max = 1000;
    target.shield = 1000;
    target.shield_max = 1000;
    let mut ships = vec![source, target];
    let mut fields = Vec::new();
    let mut projectiles = Vec::new();
    let mut impacts = Vec::new();
    let mut audio_cues = Vec::new();
    let mut voice_cues = Vec::new();

    BattleSession::fire_side(
        &mut ships,
        &mut fields,
        &[0],
        &[1],
        &mut projectiles,
        &mut impacts,
        &mut audio_cues,
        &mut voice_cues,
        true,
        TacticalVoiceFaction::Alliance,
        &mut OriginalTacticalRng::new(1),
    );

    assert_eq!(
        projectiles
            .iter()
            .map(|effect| effect.kind)
            .collect::<Vec<_>>(),
        vec![
            WeaponKind::IonCannon,
            WeaponKind::LaserCannon,
            WeaponKind::Turbolaser
        ]
    );
    assert_eq!(ships[0].weapon_recharge_queue, vec![0]);
    assert!(!ships[0].weapon_arcs[0].ready);
    assert_eq!(ships[0].weapon_arcs[0].energy, 0.0);
    assert_eq!(ships[1].shield, 965);
    assert_eq!(
        audio_cues.iter().map(|cue| cue.event).collect::<Vec<_>>(),
        vec![
            TacticalAudioEvent::IonFire,
            TacticalAudioEvent::IonImpact,
            TacticalAudioEvent::LaserFire,
            TacticalAudioEvent::LaserImpact,
            TacticalAudioEvent::TurbolaserFire,
            TacticalAudioEvent::TurbolaserImpact,
        ]
    );
    assert!(audio_cues.iter().all(|cue| {
        let base = cue.event.resource_base();
        cue.resource_id >= base && cue.resource_id < base + u32::from(cue.event.variant_count())
    }));
}

// Source: FUN_005b3a40 / FUN_005b3f10 automatic weapon fire.
#[test]
fn automatic_weapon_callback_keeps_first_arc_on_ties_and_fires_each_ready_arc() {
    let mut source = test_ship(64, 0, true, true);
    source.hull_current = 100;
    source.hull_max = 100;
    source.weapon_arcs[0] = TacticalWeaponArc::new(0, 0, 30);
    source.weapon_arcs[1] = TacticalWeaponArc::new(0, 0, 30);
    source.weapon_ranges.turbolaser = 100.0;
    let mut fore = test_ship(128, 0, false, true);
    fore.source_position.z = 10.0;
    fore.hull_current = 1000;
    fore.hull_max = 1000;
    fore.shield = 1000;
    fore.shield_max = 1000;
    let mut starboard = fore.clone();
    starboard.identity.fleet_roster_index = 1;
    starboard.source_position = TacticalWorldPosition {
        x: 10.0,
        y: 0.0,
        z: 0.0,
    };
    let mut ships = vec![source, fore, starboard];
    let mut projectiles = Vec::new();
    let mut audio_cues = Vec::new();
    let mut voice_cues = Vec::new();

    BattleSession::fire_side(
        &mut ships,
        &mut Vec::new(),
        &[0],
        &[1, 2],
        &mut projectiles,
        &mut Vec::new(),
        &mut audio_cues,
        &mut voice_cues,
        true,
        TacticalVoiceFaction::Alliance,
        &mut OriginalTacticalRng::new(1),
    );

    assert_eq!(
        projectiles
            .iter()
            .map(|effect| effect.target)
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(ships[0].weapon_recharge_queue, vec![0, 1]);
    assert_eq!(audio_cues.len(), 4);
}

// Source: FUN_005d2360 weapon strength coefficient.
#[test]
fn weapon_candidates_enforce_range_and_apply_hull_scaling_only_to_auto_fire() {
    let mut source = test_ship(64, 0, true, true);
    source.hull_current = 50;
    source.hull_max = 100;
    source.weapon_arcs[0] = TacticalWeaponArc::new(0, 0, 100);
    source.weapon_ranges.turbolaser = 10.0;
    let mut target = test_ship(128, 0, false, true);
    target.source_position.z = 10.0;
    let ships = vec![source, target];

    let automatic = original_capital_weapon_candidates(&ships, 0, &[1], None);
    let direct = original_capital_weapon_candidates(&ships, 0, &[1], Some(1));
    assert!((automatic[0].turbolaser.strength - 13.333_334).abs() < 0.000_01);
    assert!((direct[0].turbolaser.strength - 26.666_668).abs() < 0.000_01);

    let mut out_of_range = ships;
    out_of_range[1].source_position.z = 10.001;
    assert_eq!(
        original_capital_weapon_candidates(&out_of_range, 0, &[1], None)[0]
            .turbolaser
            .target,
        None
    );
}

// Source: FUN_005b64e0 / FUN_005b6320 weapon recharge FIFO.
#[test]
fn weapon_recharge_is_fifo_and_preserves_leftover_energy() {
    let mut ship = test_ship(64, 0, true, true);
    ship.hull_current = 100;
    ship.hull_max = 100;
    ship.weapon_recharge_rate = 8.0;
    ship.weapon_arcs[0] = TacticalWeaponArc::new(0, 0, 10);
    ship.weapon_arcs[1] = TacticalWeaponArc::new(0, 0, 10);
    for arc in &mut ship.weapon_arcs[..2] {
        arc.ready = false;
        arc.energy = 0.0;
    }
    ship.weapon_recharge_queue = vec![0, 1];

    ship.advance_original_recharge(1.0);
    assert_eq!(ship.weapon_arcs[0].energy, 8.0);
    assert_eq!(ship.weapon_arcs[1].energy, 0.0);
    assert_eq!(ship.weapon_recharge_queue, vec![0, 1]);

    ship.advance_original_recharge(0.5);
    assert_eq!(ship.weapon_arcs[0].energy, 10.0);
    assert!(ship.weapon_arcs[0].ready);
    assert_eq!(ship.weapon_arcs[1].energy, 2.0);
    assert_eq!(ship.weapon_recharge_queue, vec![1]);
}

// Source: FUN_005b05c0 weapon recharge scale 3.75.
#[test]
fn shield_recharge_retains_fractional_source_energy_and_damage_penalties() {
    let mut ship = test_ship(64, 0, true, true);
    ship.hull_current = 100;
    ship.hull_max = 100;
    ship.shield = 0;
    ship.shield_max = 10;
    ship.shield_recharge_rate = 3.0;

    for _ in 0..4 {
        ship.advance_original_recharge(0.25);
    }
    assert_eq!(ship.shield, 3);
    assert_eq!(ship.shield_recharge_carry, 0.0);

    ship.subsystem_damage.shields = 1;
    ship.advance_original_recharge(1.0);
    assert_eq!(ship.shield, 5);
    assert!((ship.shield_recharge_carry - 0.25).abs() < f32::EPSILON);
}

// Source: FUN_005b3f10 subsystem condition affecting weapon arcs.
#[test]
fn destroyed_weapon_subsystem_disables_and_repair_recharges_live_arcs() {
    let mut ship = test_ship(64, 0, true, true);
    ship.hull_current = 100;
    ship.hull_max = 100;
    ship.subsystem_capacity.weapons = true;
    ship.weapon_recharge_rate = 10.0;
    ship.weapon_arcs[0] = TacticalWeaponArc::new(0, 0, 10);

    for _ in 0..4 {
        ship.add_subsystem_hit(TacticalSubsystemKind::Weapons);
    }
    assert!(!ship.weapon_arcs[0].ready);
    assert_eq!(ship.weapon_arcs[0].energy, 0.0);
    assert_eq!(ship.weapon_recharge_queue, vec![0]);
    ship.remove_subsystem_hit(TacticalSubsystemKind::Weapons);
    ship.advance_original_recharge(4.0);
    assert!(ship.weapon_arcs[0].ready);
    assert_eq!(ship.weapon_arcs[0].energy, 10.0);
    assert!(ship.weapon_recharge_queue.is_empty());
}

// Source: FUN_005b3a40 / FUN_005b3f10 fire; FUN_005b5100's subsystem
// roll, which FUN_005c0170 leaves to the player's own ships (the target
// here: the defender, with the player defending).
#[test]
fn live_ship_fire_uses_the_shared_subsystem_damage_path() {
    let mut source = test_ship(64, 0, true, true);
    source.hull_current = 1000;
    source.hull_max = 1000;
    source.weapon_arcs[0] = TacticalWeaponArc::new(300, 0, 0);
    source.weapon_ranges.laser_cannon = 100.0;
    let mut target = test_ship(128, 0, false, true);
    target.hull_current = 1000;
    target.hull_max = 1000;
    target.shield = 10;
    target.shield_max = 10;
    target.subsystem_capacity.shields = true;
    target.refresh_subsystem_condition();
    let mut ships = vec![source, target];
    let mut fields = Vec::new();
    let mut projectiles = Vec::new();
    let mut impacts = Vec::new();
    let mut audio_cues = Vec::new();
    let mut voice_cues = Vec::new();

    BattleSession::fire_side(
        &mut ships,
        &mut fields,
        &[0],
        &[1],
        &mut projectiles,
        &mut impacts,
        &mut audio_cues,
        &mut voice_cues,
        false,
        TacticalVoiceFaction::Alliance,
        &mut OriginalTacticalRng::new(1),
    );

    assert!(ships[1].hull_current < 1000);
    assert_eq!(ships[1].subsystem_damage.shields, 1);
    assert!(ships[1].subsystem_condition.shields < 100);
    assert_eq!(projectiles.len(), 1);
    assert_eq!(impacts.len(), 1);
    assert_eq!(audio_cues.len(), 2);
}

// Source: FUN_005a7500 / FUN_005bae60 audio event bank.
#[test]
fn tactical_audio_event_bank_covers_every_recovered_wave_once() {
    let events = [
        TacticalAudioEvent::LaserFire,
        TacticalAudioEvent::TurbolaserFire,
        TacticalAudioEvent::IonFire,
        TacticalAudioEvent::TorpedoFire,
        TacticalAudioEvent::LaserImpact,
        TacticalAudioEvent::IonImpact,
        TacticalAudioEvent::TurbolaserImpact,
        TacticalAudioEvent::TorpedoImpact,
    ];
    let cues: Vec<_> = events
        .into_iter()
        .flat_map(|event| {
            (0..event.variant_count())
                .map(move |variant| TacticalAudioCue::from_variant(event, variant))
        })
        .collect();

    assert_eq!(
        cues.iter()
            .map(|cue| cue.event.event_id())
            .collect::<Vec<_>>(),
        vec![
            0x0d, 0x0d, 0x0d, 0x0e, 0x0e, 0x0e, 0x0f, 0x0f, 0x0f, 0x10, 0x10, 0x10, 0x11, 0x11,
            0x11, 0x12, 0x12, 0x12, 0x13, 0x13, 0x13, 0x14,
        ]
    );
    assert_eq!(
        cues.iter().map(|cue| cue.resource_id).collect::<Vec<_>>(),
        (13_033..=13_054).collect::<Vec<_>>()
    );
}

// Source: FUN_0061a310 / FUN_005a8a70 MSVC-compatible global RNG.
#[test]
fn tactical_rng_preserves_the_recovered_recurrence_and_inclusive_range() {
    let mut rng = OriginalTacticalRng::new(1);
    assert_eq!(
        (0..6).map(|_| rng.next_u15()).collect::<Vec<_>>(),
        vec![41, 18_467, 6_334, 26_500, 19_169, 15_724]
    );

    let mut rng = OriginalTacticalRng::new(1);
    assert_eq!(rng.inclusive(1, 10), 2);
    assert_eq!(rng.inclusive(0, 100), 85);
    assert_ne!(rng.state(), 1);
}

#[test]
fn tactical_voice_cues_preserve_source_event_and_resource_families() {
    // FUN_005bae60 supplies the event/resource arithmetic; the named
    // command callers and audited voice map supply these family offsets.
    assert_eq!(
        TacticalVoiceCue::battle_ready(TacticalVoiceFaction::Alliance),
        TacticalVoiceCue {
            event: TacticalVoiceEvent::BattleReady,
            faction: TacticalVoiceFaction::Alliance,
            source_event: 0x20,
            resource_id: 14_001,
        }
    );
    assert_eq!(
        TacticalVoiceCue::fighter_attack(TacticalVoiceFaction::Alliance, 3).resource_id,
        14_040
    );
    assert_eq!(
        TacticalVoiceCue::capital_maneuver_complete(TacticalVoiceFaction::Alliance, 7).source_event,
        0x36
    );
    assert_eq!(
        TacticalVoiceCue::fighter_maneuver_complete(TacticalVoiceFaction::Empire, 3).source_event,
        0xb4
    );
    assert_eq!(
        TacticalVoiceCue::capital_orders_complete(TacticalVoiceFaction::Alliance, 7).source_event,
        0x50
    );
    assert_eq!(
        TacticalVoiceCue::fighter_orders_complete(TacticalVoiceFaction::Empire, 3).source_event,
        0xce
    );
    assert_eq!(
        TacticalVoiceCue::target_destroyed_by_capital(TacticalVoiceFaction::Alliance, 7)
            .source_event,
        0x5d
    );
    assert_eq!(
        TacticalVoiceCue::target_destroyed_by_fighter(TacticalVoiceFaction::Empire, 3).source_event,
        0xdb
    );
    assert_eq!(
        TacticalVoiceCue::capital_destroyed(TacticalVoiceFaction::Alliance, 7).source_event,
        0x6a
    );
    assert_eq!(
        TacticalVoiceCue::fighter_losses(TacticalVoiceFaction::Empire, 3).source_event,
        0xe8
    );
    assert_eq!(
        TacticalVoiceCue::capital_formation(TacticalVoiceFaction::Empire, 7),
        TacticalVoiceCue {
            event: TacticalVoiceEvent::CapitalFormation,
            faction: TacticalVoiceFaction::Empire,
            source_event: 0xf4,
            resource_id: 15_091,
        }
    );
    assert_eq!(
        TacticalVoiceCue::fighter_mission(TacticalVoiceFaction::Empire, 3).resource_id,
        15_104
    );
    assert_eq!(
        TacticalVoiceCue::withdrawal_started(TacticalVoiceFaction::Alliance),
        TacticalVoiceCue {
            event: TacticalVoiceEvent::WithdrawalStarted,
            faction: TacticalVoiceFaction::Alliance,
            source_event: 0x8f,
            resource_id: 14_112,
        }
    );
    assert_eq!(
        TacticalVoiceCue::fighter_recovery_complete(TacticalVoiceFaction::Alliance, 0),
        TacticalVoiceCue {
            event: TacticalVoiceEvent::FighterRecoveryComplete,
            faction: TacticalVoiceFaction::Alliance,
            source_event: 0x87,
            resource_id: 14_104,
        }
    );
    assert_eq!(
        TacticalVoiceCue::fighter_recovery_complete(TacticalVoiceFaction::Empire, 3),
        TacticalVoiceCue {
            event: TacticalVoiceEvent::FighterRecoveryComplete,
            faction: TacticalVoiceFaction::Empire,
            source_event: 0x108,
            resource_id: 15_111,
        }
    );
    assert_eq!(
        TacticalVoiceCue::fighter_recovery_capacity(TacticalVoiceFaction::Alliance, 3),
        TacticalVoiceCue {
            event: TacticalVoiceEvent::FighterRecoveryCapacity,
            faction: TacticalVoiceFaction::Alliance,
            source_event: 0x8e,
            resource_id: 14_111,
        }
    );
    assert_eq!(
        TacticalVoiceCue::fighter_recovery_capacity(TacticalVoiceFaction::Empire, 0),
        TacticalVoiceCue {
            event: TacticalVoiceEvent::FighterRecoveryCapacity,
            faction: TacticalVoiceFaction::Empire,
            source_event: 0x109,
            resource_id: 15_112,
        }
    );
    assert_eq!(
        TacticalVoiceCue::withdrawal_hyperdrive_warning(TacticalVoiceFaction::Alliance),
        TacticalVoiceCue {
            event: TacticalVoiceEvent::WithdrawalHyperdriveWarning,
            faction: TacticalVoiceFaction::Alliance,
            source_event: 0x95,
            resource_id: 14_118,
        }
    );
    assert_eq!(
        TacticalVoiceCue::withdrawal_hyperdrive_warning(TacticalVoiceFaction::Empire),
        TacticalVoiceCue {
            event: TacticalVoiceEvent::WithdrawalHyperdriveWarning,
            faction: TacticalVoiceFaction::Empire,
            source_event: 0x10e,
            resource_id: 15_117,
        }
    );
    assert_eq!(
        TacticalVoiceCue::battle_won(TacticalVoiceFaction::Empire, false).resource_id,
        15_129
    );
    assert_eq!(TacticalVoiceCue::death_star_firing().resource_id, 15_120);
    assert_eq!(TacticalVoiceCue::trench_run_started(0).source_event, 0x139);
    assert_eq!(
        TacticalVoiceCue::trench_run_committed(0).source_event,
        0x13a
    );
    assert_eq!(
        TacticalVoiceCue::trench_run_outcome(3, TacticalTrenchRunOutcome::Success),
        TacticalVoiceCue {
            event: TacticalVoiceEvent::TrenchRunSucceeded,
            faction: TacticalVoiceFaction::Alliance,
            source_event: 0x138,
            resource_id: 15_159,
        }
    );
    assert_eq!(
        TacticalVoiceCue::target_task_force_rejected(TacticalVoiceFaction::Alliance),
        TacticalVoiceCue {
            event: TacticalVoiceEvent::TargetTaskForceRejected,
            faction: TacticalVoiceFaction::Alliance,
            source_event: 0x84,
            resource_id: 14_101,
        }
    );
    assert_eq!(
        TacticalVoiceCue::target_task_force_rejected(TacticalVoiceFaction::Empire),
        TacticalVoiceCue {
            event: TacticalVoiceEvent::TargetTaskForceRejected,
            faction: TacticalVoiceFaction::Empire,
            source_event: 0x102,
            resource_id: 15_105,
        }
    );

    let mut empire = test_session(Vec::new(), Vec::new(), false);
    empire.queue_battle_ready_voice();
    assert_eq!(
        empire.take_pending_voice_cues(),
        vec![TacticalVoiceCue::battle_ready(TacticalVoiceFaction::Empire)]
    );
    empire.queue_withdrawal_started_voice();
    assert_eq!(
        empire.take_pending_voice_cues(),
        vec![TacticalVoiceCue::withdrawal_started(
            TacticalVoiceFaction::Empire
        )]
    );
}

// Source: FUN_005a5cf0 chooses the local firing group for a destroyed
// target and the local casualty group when the opponent destroys it.
#[test]
fn destruction_voice_callback_uses_source_and_target_group_perspectives() {
    let mut cues = Vec::new();
    queue_original_destruction_voice(
        &mut cues,
        TacticalVoiceFaction::Alliance,
        true,
        true,
        TacticalVoiceGroup::Fighter(2),
        false,
        TacticalVoiceGroup::Capital(4),
        OriginalTacticalImpactStage::Destroyed,
    );
    queue_original_destruction_voice(
        &mut cues,
        TacticalVoiceFaction::Alliance,
        true,
        false,
        TacticalVoiceGroup::Capital(3),
        true,
        TacticalVoiceGroup::Capital(4),
        OriginalTacticalImpactStage::Destroyed,
    );
    queue_original_destruction_voice(
        &mut cues,
        TacticalVoiceFaction::Alliance,
        true,
        false,
        TacticalVoiceGroup::Capital(3),
        true,
        TacticalVoiceGroup::Fighter(1),
        OriginalTacticalImpactStage::Destroyed,
    );
    queue_original_destruction_voice(
        &mut cues,
        TacticalVoiceFaction::Alliance,
        true,
        true,
        TacticalVoiceGroup::Capital(0),
        false,
        TacticalVoiceGroup::Capital(0),
        OriginalTacticalImpactStage::Damage,
    );

    assert_eq!(
        cues,
        vec![
            TacticalVoiceCue::target_destroyed_by_fighter(TacticalVoiceFaction::Alliance, 2,),
            TacticalVoiceCue::capital_destroyed(TacticalVoiceFaction::Alliance, 4),
            TacticalVoiceCue::fighter_losses(TacticalVoiceFaction::Alliance, 1),
        ]
    );
}

#[test]
fn trench_run_groups_route_to_their_exact_source_start_and_outcome_events() {
    // FUN_005bae60 maps these four Alliance group triplets into the
    // recovered 15133..15163 trench-run voice bank.
    let expected = [
        (0, 0x139, 0x13b, 0x13c),
        (1, 0x12d, 0x12f, 0x130),
        (2, 0x131, 0x133, 0x134),
        (3, 0x135, 0x137, 0x138),
    ];

    for (group, start, failure, success) in expected {
        assert_eq!(
            TacticalVoiceCue::trench_run_started(group).source_event,
            start
        );
        assert_eq!(
            TacticalVoiceCue::trench_run_outcome(group, TacticalTrenchRunOutcome::Failure)
                .source_event,
            failure
        );
        assert_eq!(
            TacticalVoiceCue::trench_run_outcome(group, TacticalTrenchRunOutcome::Success)
                .source_event,
            success
        );
    }
}

#[test]
fn death_star_attack_entry_routes_each_local_perspective_once() {
    // FUN_005d0340 selects the active Alliance RGBG call for the attacking
    // player and source event 0x115 for the Imperial station owner.
    let mut attacking_fighter = test_fighter(3, true);
    attacking_fighter.fighter_group = 2;
    attacking_fighter.order = TacticalOrder::AttackDeathStar;
    let mut alliance = test_session(Vec::new(), vec![attacking_fighter], true);
    alliance.death_star = Some(TacticalDeathStar {
        resource: DEATH_STAR_TACTICAL_RESOURCE,
        is_attacker: false,
        is_alliance: false,
        source_position: TacticalWorldPosition::ORIGIN,
        hull: 100.0,
        laser_charge: 0.0,
        destroyed: false,
        action_committed: false,
    });
    begin_original_attack_order(&mut alliance, &[], &[0], TacticalOrder::AttackDeathStar);
    assert_eq!(
        alliance.take_pending_voice_cues(),
        vec![
            TacticalVoiceCue::trench_run_started(2),
            TacticalVoiceCue::trench_run_committed(2),
            TacticalVoiceCue::trench_run_chatter(0x11e),
        ]
    );
    assert_eq!(
        alliance.trench_run_remaining_ms,
        Some(ORIGINAL_TRENCH_RUN_DURATION_MILLISECONDS)
    );
    assert_eq!(alliance.trench_run_group, Some(2));

    let mut attacking_fighter = test_fighter(3, true);
    attacking_fighter.fighter_group = 2;
    attacking_fighter.order = TacticalOrder::AttackDeathStar;
    let mut empire = test_session(Vec::new(), vec![attacking_fighter], false);
    empire.death_star = Some(TacticalDeathStar {
        resource: DEATH_STAR_TACTICAL_RESOURCE,
        is_attacker: false,
        is_alliance: false,
        source_position: TacticalWorldPosition::ORIGIN,
        hull: 100.0,
        laser_charge: 0.0,
        destroyed: false,
        action_committed: false,
    });
    begin_original_attack_order(&mut empire, &[], &[0], TacticalOrder::AttackDeathStar);
    assert_eq!(
        empire.take_pending_voice_cues(),
        vec![
            TacticalVoiceCue::death_star_under_attack(),
            TacticalVoiceCue::trench_run_chatter(0x11e),
        ]
    );
    assert_eq!(
        empire.trench_run_remaining_ms,
        Some(ORIGINAL_TRENCH_RUN_DURATION_MILLISECONDS)
    );
}

#[test]
fn tactical_voice_bank_maps_all_source_events_to_exact_resources() {
    // FUN_005bae60 registers every event from 0x20 through 0x13c.
    let cues = (0x20..=0x13c)
        .map(|source_event| {
            TacticalVoiceCue::from_source_event(TacticalVoiceEvent::SourceMapped, source_event)
        })
        .collect::<Vec<_>>();
    assert_eq!(cues.len(), 285);
    assert_eq!(
        cues.iter()
            .map(|cue| (cue.faction, cue.resource_id))
            .collect::<HashSet<_>>()
            .len(),
        285
    );
    assert_eq!(cues.first().unwrap().resource_id, 14_001);
    assert_eq!(cues[121].resource_id, 14_122);
    assert_eq!(cues[122].resource_id, 15_001);
    assert_eq!(cues[253].resource_id, 15_132);
    assert_eq!(cues[254].resource_id, 15_133);
    assert_eq!(cues.last().unwrap().resource_id, 15_163);
}

// Source: FUN_005d3ac0 field slot sequence.
#[test]
fn original_slot_sequence_matches_executable_branch_order() {
    let actual: Vec<_> = OriginalTacticalSlots::default().take(9).collect();
    assert_eq!(actual[0].to_bits(), (-0.0f32).to_bits());
    assert_eq!(
        actual,
        vec![0.0, 5.0, -5.0, -10.0, 10.0, 15.0, -15.0, -20.0, 20.0]
    );
}

// Source: FUN_005ab650 / FUN_005a9030 world positions from layout.
#[test]
fn production_participants_retain_dat_identity_and_source_world_positions() {
    let mut ships = vec![
        test_ship(64, 0, true, true),
        test_ship(65, 2, true, true),
        test_ship(66, 7, true, false),
        test_ship(133, 0, false, true),
        test_ship(134, 1, false, true),
        test_ship(135, 2, false, true),
    ];
    let mut fighters = vec![test_fighter(3, true), test_fighter(5, false)];

    let layout = BattleSession::assign_original_world_positions(&mut ships, &mut fighters);
    assert_eq!(layout.first_active_objects, 3);
    assert_eq!(layout.second_active_objects, 4);
    assert_eq!(layout.battle_extent, 112.0);
    assert_eq!(
        ships[0].source_position,
        TacticalWorldPosition {
            x: 0.0,
            y: 0.0,
            z: -56.0
        }
    );
    assert_eq!(
        ships[1].source_position,
        TacticalWorldPosition {
            x: 5.0,
            y: 0.0,
            z: -56.0
        }
    );
    assert_eq!(ships[2].source_position, TacticalWorldPosition::ORIGIN);
    assert_eq!(
        ships[3].source_position,
        TacticalWorldPosition {
            x: 0.0,
            y: 0.0,
            z: 56.0
        }
    );
    assert_eq!(
        ships[4].source_position,
        TacticalWorldPosition {
            x: 5.0,
            y: 0.0,
            z: 56.0
        }
    );
    assert_eq!(
        ships[5].source_position,
        TacticalWorldPosition {
            x: -5.0,
            y: 0.0,
            z: 56.0
        }
    );
    assert_eq!(
        fighters[0].source_position,
        TacticalWorldPosition {
            x: 0.0,
            y: 0.0,
            z: -36.0
        }
    );
    assert_eq!(
        fighters[1].source_position,
        TacticalWorldPosition {
            x: 0.0,
            y: 0.0,
            z: 36.0
        }
    );
    assert_eq!(ships[1].identity.class_dat_id, DatId::new(65));
    assert_eq!(ships[1].identity.fleet_roster_index, 2);
}

// Source: FUN_005ae460 fighter group assignment.
#[test]
fn fighter_groups_follow_source_ordinals_for_both_factions() {
    let mut fighters = [
        test_fighter(3, true),
        test_fighter(4, true),
        test_fighter(1, true),
        test_fighter(2, true),
        test_fighter(5, false),
        test_fighter(6, false),
        test_fighter(7, false),
        test_fighter(8, false),
    ];
    for fighter in &mut fighters {
        fighter.fighter_group = 4;
    }

    BattleSession::assign_original_fighter_groups(&mut fighters);
    assert_eq!(
        fighters.map(|fighter| fighter.fighter_group),
        [0, 1, 2, 3, 0, 1, 2, 3]
    );
}

// Source: FUN_005c81d0 task force leader selection and slots.
#[test]
fn task_force_leaders_follow_weapon_categories_square_root_counts_and_source_slots() {
    let mut ships = (0..9)
        .map(|index| test_ship(64, index, true, true))
        .collect::<Vec<_>>();
    for ship in &mut ships[..4] {
        ship.weapon_arcs[0] = TacticalWeaponArc::new(0, 1, 0);
    }
    for ship in &mut ships[4..8] {
        ship.weapon_arcs[0] = TacticalWeaponArc::new(1, 0, 0);
    }

    BattleSession::assign_original_task_forces(&mut ships);

    assert_eq!(
        ships.iter().map(|ship| ship.task_force).collect::<Vec<_>>(),
        vec![0, 0, 1, 1, 2, 2, 3, 3, 4]
    );
    assert_eq!(ships[0].source_position.y, 0.0);
    assert_eq!(ships[2].source_position.y, 8.0);
    assert_eq!(ships[4].source_position.y, -8.0);
    assert_eq!(ships[6].source_position.y, 16.0);
    assert_eq!(ships[8].source_position.y, -16.0);
}

// Source: FUN_005c81d0 sorts by -(sublight + 10000 * range).
#[test]
fn task_force_capability_order_prioritizes_range_then_available_sublight() {
    let mut ships = (0..4)
        .map(|index| test_ship(64, index, true, true))
        .collect::<Vec<_>>();
    for ship in &mut ships {
        ship.weapon_arcs[0] = TacticalWeaponArc::new(1, 0, 0);
    }
    ships[0].weapon_ranges.laser_cannon = 10.0;
    ships[0].sublight_engine_power = 50.0;
    ships[1].weapon_ranges.laser_cannon = 10.0;
    ships[1].sublight_engine_power = 200.0;
    ships[2].weapon_ranges.laser_cannon = 20.0;
    ships[2].sublight_engine_power = 1.0;
    ships[3].weapon_ranges.laser_cannon = 5.0;
    ships[3].sublight_engine_power = 500.0;

    BattleSession::assign_original_task_forces(&mut ships);

    assert_eq!(
        ships.iter().map(|ship| ship.task_force).collect::<Vec<_>>(),
        vec![1, 0, 0, 1]
    );
    assert_eq!(ships[2].source_position.x, 0.0);
    assert_eq!(ships[0].source_position.x, 0.0);
}

// Source: FUN_005ea250 / FUN_005ec400 formation footprint and slots.
#[test]
fn task_force_followers_use_largest_footprint_and_alternating_slots() {
    let mut ships = (0..3)
        .map(|index| test_ship(64, index, true, true))
        .collect::<Vec<_>>();
    for ship in &mut ships {
        ship.weapon_arcs[0] = TacticalWeaponArc::new(1, 0, 0);
        ship.source_collision_envelope = Some(OriginalTacticalCollisionEnvelope {
            vertical_radius: 1.0,
            planar_diameter: 4.0,
        });
    }
    ships[1].source_collision_envelope = Some(OriginalTacticalCollisionEnvelope {
        vertical_radius: 2.0,
        planar_diameter: 10.0,
    });
    ships[0].source_position.x = -0.0;

    BattleSession::assign_original_task_forces(&mut ships);

    assert_eq!(ships[0].task_force, 0);
    assert_eq!(ships[1].task_force, 0);
    assert_eq!(ships[2].task_force, 0);
    assert_eq!(ships[0].source_position.x.to_bits(), (-0.0_f32).to_bits());
    assert_eq!(ships[1].source_position.x, 12.0);
    assert_eq!(ships[2].source_position.x, -12.0);
    assert!(ships.iter().all(|ship| ship.source_position.y == 0.0));
}

// Source: FUN_005ea250 / FUN_005ec400 / FUN_005cbbc0 formation geometry.
#[test]
fn task_force_followers_rotate_and_move_with_the_retained_anchor() {
    let mut ships = (0..2)
        .map(|index| test_ship(64, index, true, true))
        .collect::<Vec<_>>();
    for ship in &mut ships {
        ship.weapon_arcs[0] = TacticalWeaponArc::new(1, 0, 0);
        ship.subsystem_capacity.engines = true;
        ship.sublight_engine_power = 4.0;
        ship.source_forward = TacticalWorldVector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        ship.source_desired_forward = ship.source_forward;
        ship.source_collision_envelope = Some(OriginalTacticalCollisionEnvelope {
            vertical_radius: 1.0,
            planar_diameter: 10.0,
        });
    }
    BattleSession::assign_original_task_forces(&mut ships);
    let mut session = test_session(ships, Vec::new(), true);

    session.advance_original_tactical_movement(250.0);

    assert_eq!(session.ships[0].source_position.x, 1.25);
    assert_eq!(session.ships[1].source_position.x, 1.25);
    assert_eq!(session.ships[0].source_position.z, 0.0);
    assert_eq!(session.ships[1].source_position.z, -12.0);
    assert_eq!(
        session.ships[0].source_position.y,
        session.ships[1].source_position.y
    );
}

// Source: FUN_005ae460 fighter group split threshold.
#[test]
fn empty_fighter_group_splits_first_half_only_above_seven_members() {
    let mut ten = (0..10)
        .map(|index| {
            let mut fighter = test_fighter(3, true);
            fighter.fleet_squadron_index = index;
            fighter.fighter_group = 4;
            fighter
        })
        .collect::<Vec<_>>();
    BattleSession::assign_original_fighter_groups(&mut ten);
    assert!(ten[..5].iter().all(|fighter| fighter.fighter_group == 1));
    assert!(ten[5..].iter().all(|fighter| fighter.fighter_group == 0));

    let mut seven = (0..7)
        .map(|_| {
            let mut fighter = test_fighter(3, true);
            fighter.fighter_group = 4;
            fighter
        })
        .collect::<Vec<_>>();
    BattleSession::assign_original_fighter_groups(&mut seven);
    assert!(seven.iter().all(|fighter| fighter.fighter_group == 0));
}

// Source: FUN_005b05c0 / FUN_005b9c60 sublight conversion for fighters.
#[test]
fn fighter_roster_expands_each_strategic_squadron_with_exact_dat_stats() {
    let mut world = GameWorld::default();
    let class = world.fighter_classes.insert(crate::world::FighterClass {
        dat_id: DatId::new(1),
        name: "X-wing".into(),
        is_alliance: true,
        squadron_size: 12,
        shield_strength: 5,
        sub_light_engine: 11,
        maneuverability: 8,
        laser_cannon_fore: 5,
        laser_cannon_range: 12,
        torpedoes: 4,
        torpedoes_range: 7,
        ..crate::world::FighterClass::default()
    });
    let fleet = world.fleets.insert(crate::world::Fleet {
        location: SystemKey::default(),
        capital_ships: Vec::new(),
        fighters: vec![crate::world::FighterEntry {
            class,
            count: 2,
            carrier: 0,
        }],
        characters: Vec::new(),
        is_alliance: true,
        has_death_star: false,
    });
    let mut ships = Vec::new();
    let mut fighters = Vec::new();

    BattleSession::expand_fleet(&world, fleet, true, &mut ships, &mut fighters);

    assert!(ships.is_empty());
    assert_eq!(fighters.len(), 2);
    assert_eq!(fighters[0].fleet_squadron_index, 0);
    assert_eq!(fighters[1].fleet_squadron_index, 1);
    assert_eq!(fighters[0].squad_count, 12);
    assert_eq!(fighters[0].hull_current, 96.0);
    assert_eq!(fighters[0].shield, 5.0);
    assert_eq!(fighters[0].sublight_engine_power, 4.4);
    assert_eq!(fighters[0].weapon_arc.laser_cannons, 5);
    assert_eq!(fighters[0].weapon_ranges.laser_cannon, 12.0);
    assert_eq!(fighters[0].torpedo_strength, 4);
    assert_eq!(fighters[0].torpedo_range, 7.0);
}

// Source: FUN_005b9c00 / FUN_005b9c30 fighter hull scaling.
#[test]
fn fighter_hull_drives_weapon_shield_recharge_and_live_craft_count() {
    let mut fighter = test_fighter(1, true);
    assert!((fighter.weapon_hull_factor() - 45.012).abs() < 0.001);
    assert!((fighter.shield_recharge_rate() - 0.3).abs() < 0.001);

    fighter.weapon_arc.ready = false;
    fighter.weapon_arc.energy = 0.0;
    fighter.shield = 0.0;
    fighter.advance_original_recharge(1.0);
    assert!(fighter.weapon_arc.ready);
    assert_eq!(
        fighter.weapon_arc.energy,
        fighter.weapon_arc.energy_capacity
    );
    assert!((fighter.shield - 0.3).abs() < 0.001);

    fighter.hull_current = 79.9;
    fighter.refresh_craft_count();
    assert_eq!(fighter.squad_count, 10);
    fighter.hull_current = 0.0;
    fighter.refresh_craft_count();
    assert_eq!(fighter.squad_count, 0);
    assert!(!fighter.alive);
}

// Source: FUN_005d2490 / FUN_005d24e0 fighter damage modifiers.
#[test]
fn fighter_damage_uses_float_shields_hull_and_maneuver_ratio() {
    let mut target = test_fighter(5, false);
    target.shield = 5.0;
    target.shield_max = 5.0;
    target.maneuverability = 7;
    let mut fighters = vec![target];

    assert_eq!(
        apply_original_fighter_damage(&mut fighters, 0, 9.0, 4.0),
        OriginalTacticalImpactStage::Hit
    );
    assert!((fighters[0].shield - 0.5).abs() < 0.001);
    assert_eq!(fighters[0].hull_current, 96.0);

    assert_eq!(
        apply_original_fighter_damage(&mut fighters, 0, 9.0, 4.0),
        OriginalTacticalImpactStage::Damage
    );
    assert_eq!(fighters[0].shield, 0.0);
    assert!((fighters[0].hull_current - 92.0).abs() < 0.001);
    assert_eq!(fighters[0].squad_count, 12);
}

// Source: FUN_005b23e0 through FUN_005b25d0 fighter battery sources.
#[test]
fn fighter_fore_battery_fires_source_families_and_torpedoes_at_capitals() {
    let mut fighter = test_fighter(2, true);
    fighter.weapon_arc = TacticalWeaponArc::new(8, 6, 0);
    fighter.weapon_ranges = TacticalWeaponRanges {
        laser_cannon: 20.0,
        ion_cannon: 18.0,
        turbolaser: 0.0,
    };
    fighter.torpedo_strength = 12;
    fighter.torpedo_range = 7.0;
    fighter.order = TacticalOrder::AttackCapitalShips;
    fighter.attack_target = Some(TacticalAttackTarget::CapitalShip(0));
    fighter.source_position.z = 0.0;
    let mut target = test_ship(128, 0, false, true);
    target.source_position.z = 6.0;
    target.hull_current = 1000;
    target.hull_max = 1000;
    target.shield = 0;
    target.shield_max = 0;
    let mut session = test_session(vec![target], vec![fighter], true);

    session.fighter_step();

    assert!(session.ships[0].hull_current < 1000);
    assert!(!session.fighters[0].weapon_arc.ready);
    assert_eq!(session.weapon_effects.len(), 3);
    assert_eq!(session.weapon_effects[0].kind, WeaponKind::IonCannon);
    assert_eq!(session.weapon_effects[1].kind, WeaponKind::LaserCannon);
    assert_eq!(session.weapon_effects[2].kind, WeaponKind::FighterAttack);
    assert_eq!(session.weapon_effects[0].source, usize::MAX);
    assert_eq!(
        session.weapon_effects[0].target_position,
        session.ships[0].source_position
    );
    assert_eq!(
        session
            .pending_audio_cues
            .iter()
            .map(|cue| cue.event)
            .collect::<Vec<_>>(),
        vec![
            TacticalAudioEvent::IonFire,
            TacticalAudioEvent::IonImpact,
            TacticalAudioEvent::LaserFire,
            TacticalAudioEvent::LaserImpact,
            TacticalAudioEvent::TorpedoFire,
            TacticalAudioEvent::TorpedoImpact,
        ]
    );
}

// Source: FUN_005d2490 / FUN_005d24e0 fighter-vs-fighter modifiers.
#[test]
fn dogfight_ignores_ion_and_applies_target_maneuverability() {
    let mut source = test_fighter(2, true);
    source.weapon_arc = TacticalWeaponArc::new(8, 6, 0);
    source.weapon_ranges = TacticalWeaponRanges {
        laser_cannon: 20.0,
        ion_cannon: 18.0,
        turbolaser: 0.0,
    };
    source.maneuverability = 2;
    source.order = TacticalOrder::AttackFighters;
    source.attack_target = Some(TacticalAttackTarget::FighterGroup(1));
    let mut target = test_fighter(6, false);
    target.shield = 0.0;
    target.shield_max = 0.0;
    target.maneuverability = 8;
    target.weapon_arc = TacticalWeaponArc::default();
    target.source_position.z = 10.0;
    let mut session = test_session(Vec::new(), vec![source, target], true);

    session.fighter_step();

    assert!(session.fighters[1].hull_current < 96.0);
    assert!(session.fighters[1].hull_current > 60.0);
    assert_eq!(session.weapon_effects.len(), 1);
    assert_eq!(session.weapon_effects[0].kind, WeaponKind::LaserCannon);
    assert_eq!(session.weapon_effects[0].target, usize::MAX);
}

// Source: FUN_005d0bb0 / FUN_005a8fc0 typed attack target.
#[test]
fn capital_attack_fighters_uses_typed_target_without_capital_fallback() {
    let mut source = test_ship(64, 0, true, true);
    source.weapon_arcs[0] = TacticalWeaponArc::new(300, 10, 0);
    source.weapon_ranges.laser_cannon = 100.0;
    source.weapon_ranges.ion_cannon = 100.0;
    source.maneuverability = 8;
    source.order = TacticalOrder::AttackFighters;
    source.attack_target = Some(TacticalAttackTarget::FighterGroup(0));
    let mut target = test_fighter(5, false);
    target.shield = 0.0;
    target.shield_max = 0.0;
    target.source_position.z = 10.0;
    let mut session = test_session(vec![source], vec![target], true);
    let mut effects = Vec::new();

    session.fire_capitals_at_fighters(&mut effects);

    assert!(!session.fighters[0].alive);
    assert_eq!(effects.len(), 1);
    assert_eq!(effects[0].kind, WeaponKind::LaserCannon);
    assert!(!session.ships[0].weapon_arcs[0].ready);
    assert_eq!(
        session
            .pending_audio_cues
            .iter()
            .map(|cue| cue.event)
            .collect::<Vec<_>>(),
        vec![
            TacticalAudioEvent::LaserFire,
            TacticalAudioEvent::LaserImpact
        ]
    );
}

// Source: FUN_005cf190 / FUN_005cf8f0 complete a reached waypoint and clear its
// movement intent (P58F11).
#[test]
fn ordered_navigation_route_advances_without_losing_the_next_waypoint() {
    let first = TacticalWorldPosition {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };
    let second = TacticalWorldPosition {
        x: 0.0,
        y: 0.0,
        z: 2.0,
    };
    let mut ship = test_ship(64, 0, true, true);
    ship.subsystem_capacity.engines = true;
    ship.source_waypoint = Some(first);
    ship.navigation_route = vec![first, second];
    let mut session = test_session(vec![ship], Vec::new(), true);

    session.advance_original_tactical_movement(10.0);
    assert_eq!(session.ships[0].source_position, first);
    assert_eq!(session.ships[0].source_waypoint, Some(second));
    assert_eq!(session.ships[0].navigation_route, vec![second]);

    session.advance_original_tactical_movement(10.0);
    assert_eq!(session.ships[0].source_position, second);
    assert_eq!(session.ships[0].source_waypoint, None);
    assert!(session.ships[0].navigation_route.is_empty());
}

// Source: FUN_005d0b10 / FUN_005d0bb0 manual target list.
#[test]
fn ordered_manual_targets_advance_and_clear_as_hostiles_are_destroyed() {
    let mut player = test_ship(64, 0, true, true);
    player.manual_targets = vec![
        TacticalAttackTarget::CapitalShip(1),
        TacticalAttackTarget::CapitalShip(2),
    ];
    player.attack_target = player.manual_targets.first().copied();
    let mut session = test_session(
        vec![
            player,
            test_ship(128, 0, false, true),
            test_ship(129, 1, false, true),
        ],
        Vec::new(),
        true,
    );

    session.ships[1].alive = false;
    refresh_original_attack_targets(&mut session);
    assert_eq!(
        session.ships[0].manual_targets,
        vec![TacticalAttackTarget::CapitalShip(2)]
    );
    assert_eq!(
        session.ships[0].attack_target,
        Some(TacticalAttackTarget::CapitalShip(2))
    );

    session.ships[2].alive = false;
    refresh_original_attack_targets(&mut session);
    assert!(session.ships[0].manual_targets.is_empty());
    assert_eq!(session.ships[0].attack_target, None);
}

// Source: FUN_005ca6d0 dispatches order codes 0-11 (tactical order
// execution evidence, 2026-09-17); the tactic toggle at +0x38 uses Surround
// 1 and Stand Off 2 (tactical command assignment evidence, 2026-09-17).
#[test]
fn tactical_order_and_tactic_codes_match_the_recovered_source_domain() {
    let orders = [
        (TacticalOrder::None, 0),
        (TacticalOrder::Escort, 1),
        (TacticalOrder::Recover, 2),
        (TacticalOrder::AttackFighters, 4),
        (TacticalOrder::AttackCapitalShips, 5),
        (TacticalOrder::AttackDeathStar, 6),
        (TacticalOrder::LeftHook, 7),
        (TacticalOrder::RightHook, 8),
        (TacticalOrder::Hammer, 9),
        (TacticalOrder::Anvil, 10),
        (TacticalOrder::HoldPosition, 11),
    ];
    for (order, code) in orders {
        assert_eq!(order.source_code(), code);
    }
    assert_eq!(TacticalTactic::Surround.source_code(), 1);
    assert_eq!(TacticalTactic::StandOff.source_code(), 2);
}

// Source: FUN_005ca6d0 dispatches Escort as order code 1; FUN_005d0af0,
// FUN_005cf910 and FUN_005cf920 install, retain and clear its target (P58-B14).
#[test]
fn direct_escort_retains_friendly_target_and_clears_when_it_is_destroyed() {
    let mut escort = test_ship(64, 0, true, true);
    escort.selected = true;
    let target = test_ship(65, 1, true, true);
    let enemy = test_ship(128, 0, false, true);
    let mut session = test_session(vec![escort, target, enemy], Vec::new(), true);
    session.selected_ship = Some(0);

    assert_eq!(assign_selected_escort(&mut session, 1), (1, 0));
    assert_eq!(session.ships[0].order, TacticalOrder::Escort);
    assert_eq!(session.ships[0].escort_target, Some(1));
    advance_original_escort_movement(&mut session, ORIGINAL_TACTICAL_STEP_MILLISECONDS);
    assert!(session.ships[0].source_waypoint.is_some());

    session.ships[1].alive = false;
    refresh_original_escort_targets(&mut session);
    assert_eq!(session.ships[0].order, TacticalOrder::None);
    assert_eq!(session.ships[0].escort_target, None);
    assert_eq!(session.ships[0].source_waypoint, None);
}

// Source: FUN_005ba7f0 Death Star laser charge rate to 100.
#[test]
fn death_star_operational_predicate_resources_and_laser_charge_match_source_fields() {
    let mut death_star = TacticalDeathStar {
        resource: DEATH_STAR_TACTICAL_RESOURCE,
        is_attacker: false,
        is_alliance: false,
        source_position: TacticalWorldPosition::ORIGIN,
        hull: 100.0,
        laser_charge: 0.0,
        destroyed: false,
        action_committed: false,
    };
    assert!(death_star.operational());
    assert_eq!(death_star.resource_id(), 5010);
    death_star.advance_laser_charge(300.0);
    assert_eq!(death_star.laser_charge, 100.0);
    death_star.advance_laser_charge(1.0);
    assert_eq!(death_star.laser_charge, 100.0);

    death_star.action_committed = true;
    assert!(!death_star.operational());
    death_star.action_committed = false;
    death_star.destroyed = true;
    assert!(!death_star.operational());
    assert_eq!(death_star.resource_id(), 5020);
}

#[test]
fn death_star_ready_voice_fires_once_only_when_the_players_laser_crosses_full_charge() {
    // FUN_005bae60 maps source event 0x113 to the Imperial ready line;
    // the tactical manager emits it on the player's not-ready/ready edge.
    let attacker = test_ship(64, 0, true, true);
    let defender = test_ship(128, 0, false, true);
    let mut session = test_session(vec![attacker, defender], Vec::new(), true);
    session.paused = false;
    session.death_star = Some(TacticalDeathStar {
        resource: DEATH_STAR_TACTICAL_RESOURCE,
        is_attacker: true,
        is_alliance: false,
        source_position: TacticalWorldPosition::ORIGIN,
        hull: 100.0,
        laser_charge: 99.95,
        destroyed: false,
        action_committed: false,
    });

    assert!(!session.step());
    assert_eq!(session.combat_tick, 1);
    assert_eq!(
        session.take_pending_voice_cues(),
        vec![TacticalVoiceCue::death_star_ready()]
    );
    assert!(!session.step());
    assert_eq!(session.combat_tick, 2);
    assert!(session.take_pending_voice_cues().is_empty());

    let attacker = test_ship(64, 0, true, true);
    let defender = test_ship(128, 0, false, true);
    let mut opponent_owned = test_session(vec![attacker, defender], Vec::new(), true);
    opponent_owned.paused = false;
    opponent_owned.death_star = Some(TacticalDeathStar {
        resource: DEATH_STAR_TACTICAL_RESOURCE,
        is_attacker: false,
        is_alliance: false,
        source_position: TacticalWorldPosition::ORIGIN,
        hull: 100.0,
        laser_charge: 99.95,
        destroyed: false,
        action_committed: false,
    });

    assert!(!opponent_owned.step());
    assert_eq!(opponent_owned.combat_tick, 1);
    assert!(opponent_owned.take_pending_voice_cues().is_empty());
}

#[test]
fn battle_outcome_voice_distinguishes_draw_destruction_and_each_withdrawal_side() {
    // FUN_005bae60 assigns separate outcome events for withdrawal,
    // destruction victory, and defeat in each faction's recovered bank.
    let alliance = test_ship(64, 0, true, true);
    let empire = test_ship(128, 0, false, true);
    let mut session = test_session(vec![alliance, empire], Vec::new(), true);

    session.queue_battle_outcome_voice(CombatWinner::Draw);
    assert!(session.take_pending_voice_cues().is_empty());

    session.queue_battle_outcome_voice(CombatWinner::Attacker);
    assert_eq!(
        session.take_pending_voice_cues(),
        vec![TacticalVoiceCue::battle_won(
            TacticalVoiceFaction::Alliance,
            false
        )]
    );

    session.ships[1].retreated = true;
    session.queue_battle_outcome_voice(CombatWinner::Attacker);
    assert_eq!(
        session.take_pending_voice_cues(),
        vec![TacticalVoiceCue::battle_won(
            TacticalVoiceFaction::Alliance,
            true
        )]
    );

    session.ships[1].retreated = false;
    session.ships[0].retreated = true;
    session.queue_battle_outcome_voice(CombatWinner::Defender);
    assert_eq!(
        session.take_pending_voice_cues(),
        vec![TacticalVoiceCue::withdrawal_complete(
            TacticalVoiceFaction::Alliance
        )]
    );

    session.ships[0].retreated = false;
    session.queue_battle_outcome_voice(CombatWinner::Defender);
    assert_eq!(
        session.take_pending_voice_cues(),
        vec![TacticalVoiceCue::battle_lost(
            TacticalVoiceFaction::Alliance
        )]
    );

    let alliance = test_ship(64, 0, true, true);
    let empire = test_ship(128, 0, false, true);
    let mut imperial_player = test_session(vec![alliance, empire], Vec::new(), false);
    imperial_player.ships[0].retreated = true;
    imperial_player.queue_battle_outcome_voice(CombatWinner::Defender);
    assert_eq!(
        imperial_player.take_pending_voice_cues(),
        vec![TacticalVoiceCue::battle_won(
            TacticalVoiceFaction::Empire,
            true
        )]
    );
    imperial_player.ships[0].retreated = false;
    imperial_player.queue_battle_outcome_voice(CombatWinner::Attacker);
    assert_eq!(
        imperial_player.take_pending_voice_cues(),
        vec![TacticalVoiceCue::battle_lost(TacticalVoiceFaction::Empire)]
    );
}

// Source: FUN_005ba7f0 Death Star superlaser predicates.
#[test]
fn death_star_superlaser_requires_player_ownership_full_charge_and_enemy_target() {
    let target = test_ship(64, 0, true, true);
    let mut session = test_session(vec![target], Vec::new(), false);
    session.death_star = Some(TacticalDeathStar {
        resource: DEATH_STAR_TACTICAL_RESOURCE,
        is_attacker: false,
        is_alliance: false,
        source_position: TacticalWorldPosition::ORIGIN,
        hull: 100.0,
        laser_charge: 99.0,
        destroyed: false,
        action_committed: false,
    });
    assert!(!session.commit_death_star_shot(0));
    session.death_star.as_mut().unwrap().laser_charge = 100.0;
    session.player_is_attacker = true;
    assert!(!session.commit_death_star_shot(0));
    session.player_is_attacker = false;
    session.ships[0].is_attacker = false;
    assert!(!session.commit_death_star_shot(0));
}

// Source: FUN_005ba520 fires the superlaser as FUN_005c12d0(.., 0, 1.0, 1),
// a variant-1 projectile, which lives one second (tactical-projectile-field-path.md).
#[test]
fn death_star_superlaser_resolves_after_the_source_short_projectile_duration() {
    let mut target = test_ship(64, 0, true, true);
    target.source_position.z = -24.0;
    let mut session = test_session(vec![target], Vec::new(), false);
    session.death_star = Some(TacticalDeathStar {
        resource: DEATH_STAR_TACTICAL_RESOURCE,
        is_attacker: false,
        is_alliance: false,
        source_position: TacticalWorldPosition {
            x: 0.0,
            y: 0.0,
            z: 24.0,
        },
        hull: 100.0,
        laser_charge: 100.0,
        destroyed: false,
        action_committed: false,
    });

    assert!(session.commit_death_star_shot(0));
    assert_eq!(
        session.take_pending_voice_cues(),
        vec![TacticalVoiceCue::death_star_firing()]
    );
    assert_eq!(session.death_star.unwrap().laser_charge, 0.0);
    assert!(session.death_star.unwrap().action_committed);
    assert!(session.death_star_beam.is_some());
    assert_eq!(session.death_star_beam.unwrap().color_selector, 1);
    session.advance_presentational_effects(0.999);
    assert!(session.ships[0].alive);
    session.advance_presentational_effects(0.002);
    assert!(!session.ships[0].alive);
    assert_eq!(session.ships[0].hull_current, 0);
    assert!(session.death_star_beam.is_none());
    assert!(!session.death_star.unwrap().action_committed);
    assert_eq!(session.impact_effects.len(), 1);
}

// Source: FUN_005cfec0 / FUN_005d04e0 timed trench-run producer.
#[test]
fn attack_death_star_order_uses_the_timed_native_result_producer() {
    let mut fighter = test_fighter(3, true);
    fighter.selected = true;
    fighter.order = TacticalOrder::AttackDeathStar;
    fighter.maneuverability = 9;
    let mut session = test_session(Vec::new(), vec![fighter], true);
    session.trench_run_success_rating = 9;
    session.tactical_rng = OriginalTacticalRng::new(15);
    session.selected_fighter_group = Some(0);
    session.death_star = Some(TacticalDeathStar {
        resource: DEATH_STAR_TACTICAL_RESOURCE,
        is_attacker: false,
        is_alliance: false,
        source_position: TacticalWorldPosition {
            x: 0.0,
            y: 0.0,
            z: 6.0,
        },
        hull: 100.0,
        laser_charge: 0.0,
        destroyed: false,
        action_committed: false,
    });

    begin_original_attack_order(&mut session, &[], &[0], TacticalOrder::AttackDeathStar);
    assert_eq!(
        session.trench_run_remaining_ms,
        Some(ORIGINAL_TRENCH_RUN_DURATION_MILLISECONDS)
    );
    session.advance_original_trench_run(ORIGINAL_TRENCH_RUN_DURATION_MILLISECONDS - 1.0);
    assert_eq!(session.trench_run_outcome, None);
    assert_eq!(session.death_star.unwrap().hull, 100.0);
    session.advance_original_trench_run(1.0);

    let death_star = session.death_star.unwrap();
    assert!(death_star.destroyed);
    assert_eq!(death_star.hull, 0.0);
    assert_eq!(death_star.resource_id(), 5020);
    assert!(session.weapon_effects.is_empty());
    assert_eq!(
        session.trench_run_outcome,
        Some(TacticalTrenchRunOutcome::Success)
    );
    assert_eq!(
        session.take_pending_trench_run_cinematic(),
        Some(TacticalTrenchRunOutcome::Success)
    );
    assert_eq!(session.take_pending_trench_run_cinematic(), None);
    assert_eq!(
        session.take_pending_voice_cues(),
        vec![
            TacticalVoiceCue::trench_run_started(0),
            TacticalVoiceCue::trench_run_committed(0),
            TacticalVoiceCue::trench_run_chatter(0x11e),
            TacticalVoiceCue::trench_run_chatter(0x11f),
            TacticalVoiceCue::trench_run_chatter(0x120),
            TacticalVoiceCue::trench_run_chatter(0x128),
            TacticalVoiceCue::trench_run_chatter(0x121),
            TacticalVoiceCue::trench_run_chatter(0x122),
            TacticalVoiceCue::trench_run_chatter(0x129),
            TacticalVoiceCue::trench_run_outcome(0, TacticalTrenchRunOutcome::Success),
        ]
    );
}

// Source: FUN_005cfec0 state 6 removes the first bounded half of the
// retained participant list after a successful trench run.
#[test]
fn successful_trench_run_applies_the_source_half_group_casualty_rule() {
    let mut fighters = (0..4)
        .map(|_| {
            let mut fighter = test_fighter(3, true);
            fighter.order = TacticalOrder::AttackDeathStar;
            fighter.maneuverability = 9;
            fighter
        })
        .collect::<Vec<_>>();
    fighters[0].selected = true;
    let mut session = test_session(Vec::new(), fighters, true);
    session.trench_run_success_rating = 9;
    session.tactical_rng = OriginalTacticalRng::new(15);
    session.death_star = Some(TacticalDeathStar {
        resource: DEATH_STAR_TACTICAL_RESOURCE,
        is_attacker: false,
        is_alliance: false,
        source_position: TacticalWorldPosition::ORIGIN,
        hull: 100.0,
        laser_charge: 0.0,
        destroyed: false,
        action_committed: false,
    });

    begin_original_attack_order(&mut session, &[], &[0], TacticalOrder::AttackDeathStar);
    assert_eq!(session.trench_run_participants, vec![0, 1, 2, 3]);
    assert_eq!(
        session.trench_run_pending_outcome,
        Some(TacticalTrenchRunOutcome::Success)
    );
    session.advance_original_trench_run(ORIGINAL_TRENCH_RUN_DURATION_MILLISECONDS);

    assert!(!session.fighters[0].alive);
    assert!(!session.fighters[1].alive);
    assert!(session.fighters[2].alive);
    assert!(session.fighters[3].alive);
    assert!(session.death_star.unwrap().destroyed);
}

// Source: FUN_005bae60 Death Star attack failure voice event.
#[test]
fn losing_the_committed_death_star_attackers_routes_failure_once() {
    let mut fighter = test_fighter(3, true);
    fighter.order = TacticalOrder::AttackDeathStar;
    fighter.weapon_arc = TacticalWeaponArc::default();
    fighter.shield = 0.0;
    fighter.shield_max = 0.0;
    fighter.hull_current = 1.0;
    fighter.refresh_craft_count();

    let mut defender = test_ship(128, 0, false, true);
    defender.order = TacticalOrder::AttackFighters;
    defender.attack_target = Some(TacticalAttackTarget::FighterGroup(0));
    defender.weapon_arcs[0] = TacticalWeaponArc::new(300, 0, 0);
    defender.weapon_ranges.laser_cannon = 100.0;

    let mut session = test_session(vec![defender], vec![fighter], true);
    session.paused = false;
    session.death_star = Some(TacticalDeathStar {
        resource: DEATH_STAR_TACTICAL_RESOURCE,
        is_attacker: false,
        is_alliance: false,
        source_position: TacticalWorldPosition {
            x: 0.0,
            y: 0.0,
            z: 6.0,
        },
        hull: 100.0,
        laser_charge: 0.0,
        destroyed: false,
        action_committed: false,
    });
    begin_original_attack_order(&mut session, &[], &[0], TacticalOrder::AttackDeathStar);

    assert!(session.step());
    assert!(!session.fighters[0].alive);
    assert!(!session.death_star.unwrap().destroyed);
    assert_eq!(
        session.trench_run_pending_outcome,
        Some(TacticalTrenchRunOutcome::Failure)
    );
    assert_eq!(session.take_pending_trench_run_cinematic(), None);
    session.advance_original_trench_run(ORIGINAL_TRENCH_RUN_DURATION_MILLISECONDS);
    assert_eq!(
        session.take_pending_trench_run_cinematic(),
        Some(TacticalTrenchRunOutcome::Failure)
    );
    assert_eq!(session.take_pending_trench_run_cinematic(), None);
    let cues = session.take_pending_voice_cues();
    assert_eq!(cues.first(), Some(&TacticalVoiceCue::trench_run_started(0)));
    assert!(cues.contains(&TacticalVoiceCue::fighter_losses(
        TacticalVoiceFaction::Alliance,
        0,
    )));
    assert!(cues.contains(&TacticalVoiceCue::battle_lost(
        TacticalVoiceFaction::Alliance,
    )));
    assert_eq!(
        cues.last(),
        Some(&TacticalVoiceCue::trench_run_outcome(
            0,
            TacticalTrenchRunOutcome::Failure,
        ))
    );
}

#[test]
fn trench_run_result_is_recorded_once_and_failure_remains_source_driven() {
    let mut session = test_session(Vec::new(), Vec::new(), true);
    session.record_trench_run_outcome(TacticalTrenchRunOutcome::Failure);
    session.record_trench_run_outcome(TacticalTrenchRunOutcome::Success);

    assert!(session.paused);
    assert_eq!(
        session.take_pending_trench_run_cinematic(),
        Some(TacticalTrenchRunOutcome::Failure)
    );
    assert_eq!(
        session.trench_run_outcome,
        Some(TacticalTrenchRunOutcome::Failure)
    );
    assert_eq!(session.take_pending_trench_run_cinematic(), None);
    assert_eq!(
        session.take_pending_voice_cues(),
        vec![TacticalVoiceCue::trench_run_outcome(
            0,
            TacticalTrenchRunOutcome::Failure
        )]
    );
}

#[test]
fn alliance_trench_result_selects_only_the_active_alliance_attack_group() {
    // FUN_005bae60's RGBY outcome family is selected from the Alliance
    // group actively executing the Death Star mission.
    let mut imperial_attacker = test_fighter(5, false);
    imperial_attacker.fighter_group = 1;
    imperial_attacker.order = TacticalOrder::AttackDeathStar;
    let mut inactive_alliance = test_fighter(3, true);
    inactive_alliance.fighter_group = 2;
    inactive_alliance.order = TacticalOrder::None;
    let mut active_alliance = test_fighter(4, true);
    active_alliance.fighter_group = 3;
    active_alliance.order = TacticalOrder::AttackDeathStar;
    let mut session = test_session(
        Vec::new(),
        vec![imperial_attacker, inactive_alliance, active_alliance],
        true,
    );

    session.record_trench_run_outcome(TacticalTrenchRunOutcome::Success);

    assert_eq!(
        session.take_pending_voice_cues(),
        vec![TacticalVoiceCue::trench_run_outcome(
            3,
            TacticalTrenchRunOutcome::Success
        )]
    );
}

#[test]
fn imperial_trench_run_observer_routes_failure_and_success_to_death_star_events() {
    // FUN_005bae60 supplies distinct Imperial events for a broken-off
    // Alliance attack and destruction of the Death Star.
    let mut failure = test_session(Vec::new(), Vec::new(), false);
    failure.record_trench_run_outcome(TacticalTrenchRunOutcome::Failure);
    assert_eq!(
        failure.take_pending_voice_cues(),
        vec![TacticalVoiceCue::death_star_attack_broken_off()]
    );
    assert_eq!(
        failure.trench_run_outcome,
        Some(TacticalTrenchRunOutcome::Failure)
    );

    let mut success = test_session(Vec::new(), Vec::new(), false);
    success.record_trench_run_outcome(TacticalTrenchRunOutcome::Success);
    assert_eq!(
        success.take_pending_voice_cues(),
        vec![TacticalVoiceCue::death_star_destroyed()]
    );
    assert_eq!(
        success.trench_run_outcome,
        Some(TacticalTrenchRunOutcome::Success)
    );
}
