use crate::carriage::ProductRoom;
use std::collections::HashSet;

use super::test_support::{test_death_star, test_fighter, test_session, test_ship};
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

// FUN_005ca6d0 case 6 makes the result in state 2 (FUN_005d0820), whose first
// update speaks (FUN_005d0340): the active Alliance group's call for the
// Alliance player, event 0x115 for the Imperial station owner. The run
// begins (state 5, FUN_005cfe80 at 0x5c6924) when the group's leader reaches
// its entry point (FUN_005c6610.c:112-145), with the state-5 call and the
// first chatter slot.
#[test]
fn death_star_attack_entry_routes_each_local_perspective_once() {
    for player_is_alliance in [true, false] {
        let mut attacking_fighter = test_fighter(3, true);
        attacking_fighter.fighter_group = 2;
        attacking_fighter.order = TacticalOrder::AttackDeathStar;
        let mut session = test_session(Vec::new(), vec![attacking_fighter], player_is_alliance);
        session.death_star = Some(test_death_star(false));
        begin_original_attack_order(&mut session, &[], &[0], TacticalOrder::AttackDeathStar);
        assert_eq!(
            session.take_pending_voice_cues(),
            vec![if player_is_alliance {
                TacticalVoiceCue::trench_run_started(2)
            } else {
                TacticalVoiceCue::death_star_under_attack()
            }]
        );
        assert_eq!(session.trench_run_group, Some(2));
        assert_eq!(session.trench_run_remaining_ms, None, "state 2 until it arrives");
        assert_eq!(session.tactical_rng, OriginalTacticalRng::new(0x5eed_ba77), "no draw yet");

        session.fighters[0].source_position = session.trench_run_entry_point(true);
        session.advance_trench_run_approach(ORIGINAL_TACTICAL_STEP_MILLISECONDS);
        assert_eq!(
            session.trench_run_remaining_ms,
            Some(ORIGINAL_TRENCH_RUN_DURATION_MILLISECONDS)
        );
        let chatter = TacticalVoiceCue::trench_run_chatter(session.trench_run_chatter[0]);
        let expected = if player_is_alliance {
            vec![TacticalVoiceCue::trench_run_committed(2), chatter]
        } else {
            vec![chatter]
        };
        assert_eq!(session.take_pending_voice_cues(), expected);
    }
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

// FUN_005ab650.c:297-320 (Alliance, owner +0x168 == 0, types 0x1d-0x20)
// and :383-406 (Empire, owner 1, types 0x21-0x24) group each side apart,
// and FUN_005ae460 splits only that side's largest group. port: a
// squadron with no craft left is grouped by neither.
#[test]
fn each_sides_fighter_groups_split_on_its_own_squadrons_alone() {
    for (side, other) in [(true, false), (false, true)] {
        let (side_class, other_class) = if side { (3, 5) } else { (5, 3) };
        let mut fighters = vec![test_fighter(other_class, other)];
        fighters.extend((0..8).map(|_| test_fighter(side_class, side)));
        let mut empty = test_fighter(side_class, side);
        empty.squad_count = 0;
        fighters.push(empty);
        for fighter in &mut fighters {
            fighter.fighter_group = 4;
        }
        BattleSession::assign_original_fighter_groups(&mut fighters);
        let groups: Vec<u8> = fighters
            .iter()
            .map(|fighter| fighter.fighter_group)
            .collect();
        assert_eq!(groups, vec![0, 1, 1, 1, 1, 0, 0, 0, 0, 4], "side {side}");
    }
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
        bombarding: false,
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
    // `+0x134` is the class's torpedoes times the squadron size
    // (`FUN_005030c0`, stored by `FUN_005b9c60.c:74`): 4 × 12.
    assert_eq!(fighters[0].torpedo_strength, 48);
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
    assert_eq!(fighter.squad_count, 9);
    fighter.hull_current = 0.0;
    fighter.refresh_craft_count();
    assert_eq!(fighter.squad_count, 0);
    assert!(!fighter.alive);
}

#[test]
fn the_craft_count_is_the_hull_over_eight_truncated_and_at_least_one_while_hull_is_left() {
    // FUN_005b8e00 (0x5b8e29-0x5b8e56): 0 only in state 0xb, else
    // max(1, ftol(strength / 8.0)) with _DAT_0066c6b8 = 8.0
    // (ghidra/notes/tactical-results.md 3).
    let mut fighter = test_fighter(1, true);
    for (hull, craft) in [
        (96.0, 12),
        (79.9, 9),
        (16.0, 2),
        (15.9, 1),
        (12.0, 1),
        (0.5, 1),
    ] {
        fighter.hull_current = hull;
        fighter.refresh_craft_count();
        assert_eq!(fighter.squad_count, craft, "hull {hull}");
        assert!(fighter.alive, "hull {hull}");
    }
    fighter.hull_current = 0.0;
    fighter.refresh_craft_count();
    assert_eq!(fighter.squad_count, 0);
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
    // FUN_005b8e00: max(1, ftol(92 / 8.0)).
    assert_eq!(fighters[0].squad_count, 11);
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

// Source: FUN_005ba7f0 Death Star laser charge rate to 100; FUN_005ba420
// (operational: +0x74 clear, +0x68 non-zero, +0x78 clear) and FUN_005ba450
// (attackable: operational and +0x70 clear).
#[test]
fn death_star_operational_predicate_resources_and_laser_charge_match_source_fields() {
    let mut death_star = test_death_star(false);
    assert!(death_star.operational());
    assert!(death_star.attackable());
    assert_eq!(death_star.resource_id(), 5010);
    death_star.advance_laser_charge(300.0);
    assert_eq!(death_star.laser_charge, 100.0);
    death_star.advance_laser_charge(1.0);
    assert_eq!(death_star.laser_charge, 100.0);

    death_star.action_committed = true;
    assert!(!death_star.operational());
    death_star.action_committed = false;

    // The shield blocks the attack, not the station.
    death_star.shielded = true;
    assert!(death_star.operational());
    assert!(!death_star.attackable());

    let mut destroyed = test_death_star(false);
    destroyed.hull = 0.0;
    assert!(!destroyed.operational());
    assert_eq!(destroyed.resource_id(), 5010);

    // An uncompleted station is never operational, draws 5020 and keeps
    // its charge where it is.
    let mut unfinished = test_death_star(false);
    unfinished.not_completed = true;
    assert!(!unfinished.operational());
    assert!(!unfinished.attackable());
    assert_eq!(unfinished.resource_id(), 5020);
    unfinished.advance_laser_charge(300.0);
    assert_eq!(unfinished.laser_charge, 0.0);
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
        laser_charge: 99.95,
        ..test_death_star(true)
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
        laser_charge: 99.95,
        ..test_death_star(false)
    });

    assert!(!opponent_owned.step());
    assert_eq!(opponent_owned.combat_tick, 1);
    // No ready line for the opponent's station; its computer side fires at
    // once (FUN_005c8780), and the shot warns the local Alliance with 0x91
    // (FUN_005ba520.c:25-31).
    assert_eq!(
        opponent_owned.take_pending_voice_cues(),
        vec![TacticalVoiceCue::death_star_fire_warning()]
    );
}

#[test]
fn battle_outcome_voice_distinguishes_draw_destruction_and_each_withdrawal_side() {
    // FUN_005bae60 assigns separate outcome events for withdrawal,
    // destruction victory, and defeat in each faction's recovered bank;
    // FUN_005a9ec0.c:291-328 picks one from each side's lost flag (nothing
    // left) and withdrawn flag (a unit jumped).
    // Each side: (alive, jumped); a ship that jumped is out of the battle.
    let voice = |player_is_alliance: bool, alliance: (bool, bool), empire: (bool, bool)| {
        let mut session = test_session(
            vec![
                test_ship(64, 0, true, alliance.0),
                test_ship(128, 0, false, empire.0),
            ],
            Vec::new(),
            player_is_alliance,
        );
        session.morale.jumped = [alliance.1, empire.1];
        session.queue_battle_outcome_voice();
        session.take_pending_voice_cues()
    };
    let alliance = TacticalVoiceFaction::Alliance;
    let empire = TacticalVoiceFaction::Empire;

    assert!(voice(true, (true, false), (true, false)).is_empty(), "no outcome");
    assert_eq!(
        voice(true, (true, false), (false, false)),
        vec![TacticalVoiceCue::battle_won(alliance, false)]
    );
    assert_eq!(
        voice(true, (true, false), (false, true)),
        vec![TacticalVoiceCue::battle_won(alliance, true)]
    );
    assert_eq!(
        voice(true, (false, true), (true, false)),
        vec![TacticalVoiceCue::withdrawal_complete(alliance)]
    );
    assert_eq!(
        voice(true, (false, false), (true, false)),
        vec![TacticalVoiceCue::battle_lost(alliance)]
    );
    assert_eq!(
        voice(false, (false, true), (true, false)),
        vec![TacticalVoiceCue::battle_won(empire, true)]
    );
    assert_eq!(
        voice(false, (true, false), (false, false)),
        vec![TacticalVoiceCue::battle_lost(empire)]
    );
}

// Source: FUN_005ba7f0 Death Star superlaser predicates.
#[test]
fn death_star_superlaser_requires_player_ownership_full_charge_and_enemy_target() {
    let target = test_ship(64, 0, true, true);
    let mut session = test_session(vec![target], Vec::new(), false);
    session.death_star = Some(TacticalDeathStar {
        laser_charge: 99.0,
        ..test_death_star(false)
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
        source_position: TacticalWorldPosition {
            x: 0.0,
            y: 0.0,
            z: 24.0,
        },
        laser_charge: 100.0,
        ..test_death_star(false)
    });

    assert!(session.commit_death_star_shot(0));
    assert_eq!(
        session.take_pending_voice_cues(),
        vec![TacticalVoiceCue::death_star_firing()]
    );
    assert_eq!(session.death_star.unwrap().laser_charge, 0.0);
    // The shot leaves `+0x78` alone (`FUN_005ba520`); only the departure
    // `FUN_005ba5e0` sets it.
    assert!(!session.death_star.unwrap().action_committed);
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

// Source: FUN_005c6610 (state 2 to 5 on arrival), FUN_005d04e0 (the roll),
// FUN_005cfec0 (the 120000 ms result window) and FUN_005ba890 (state 6
// zeroes the hull and charge).
#[test]
fn attack_death_star_order_uses_the_timed_native_result_producer() {
    let mut fighter = test_fighter(3, true);
    fighter.selected = true;
    fighter.order = TacticalOrder::AttackDeathStar;
    fighter.maneuverability = 9;
    fighter.torpedo_strength = 1000;
    let mut session = test_session(Vec::new(), vec![fighter], true);
    // Seed 15 draws a maneuver of 8, which holds, then the certain success.
    session.tactical_rng = OriginalTacticalRng::new(15);
    session.selected_fighter_group = Some(0);
    session.death_star = Some(test_death_star(false));
    session.fighters[0].source_position = session.trench_run_entry_point(true);

    begin_original_attack_order(&mut session, &[], &[0], TacticalOrder::AttackDeathStar);
    assert_eq!(session.trench_run_remaining_ms, None);
    session.advance_trench_run_approach(ORIGINAL_TACTICAL_STEP_MILLISECONDS);
    assert_eq!(
        session.trench_run_remaining_ms,
        Some(ORIGINAL_TRENCH_RUN_DURATION_MILLISECONDS)
    );
    session.advance_original_trench_run(ORIGINAL_TRENCH_RUN_DURATION_MILLISECONDS - 1.0);
    assert_eq!(session.trench_run_outcome, None);
    assert_eq!(session.death_star.unwrap().hull, 100.0);
    session.advance_original_trench_run(1.0);

    let death_star = session.death_star.unwrap();
    assert_eq!(death_star.hull, 0.0);
    assert_eq!(death_star.laser_charge, 0.0);
    assert_eq!(death_star.resource_id(), 5010);
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
// retained participant list after a successful trench run; FUN_005d08d0
// takes the whole group, with no distance test.
#[test]
fn successful_trench_run_applies_the_source_half_group_casualty_rule() {
    let fighters = (0..4)
        .map(|_| {
            let mut fighter = test_fighter(3, true);
            fighter.order = TacticalOrder::AttackDeathStar;
            fighter.maneuverability = 9;
            fighter.torpedo_strength = 1000;
            fighter
        })
        .collect::<Vec<_>>();
    let mut session = test_session(Vec::new(), fighters, true);
    session.tactical_rng = OriginalTacticalRng::new(15);
    session.death_star = Some(test_death_star(false));
    session.fighters[0].source_position = session.trench_run_entry_point(true);

    begin_original_attack_order(&mut session, &[], &[0], TacticalOrder::AttackDeathStar);
    session.advance_trench_run_approach(ORIGINAL_TACTICAL_STEP_MILLISECONDS);
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
    assert_eq!(session.death_star.unwrap().hull, 0.0);
}

// Source: FUN_005bae60 Death Star attack failure voice event.
// hyp: a group lost before it reaches its entry point ends the run as a
// failure.
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
    session.death_star = Some(test_death_star(false));
    begin_original_attack_order(&mut session, &[], &[0], TacticalOrder::AttackDeathStar);

    assert!(session.step());
    assert!(!session.fighters[0].alive);
    assert_eq!(session.death_star.unwrap().hull, 100.0);
    assert_eq!(session.trench_run_pending_outcome, None, "no roll was made");
    assert_eq!(
        session.trench_run_outcome,
        Some(TacticalTrenchRunOutcome::Failure)
    );
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
        cues.iter()
            .filter(|cue| **cue == TacticalVoiceCue::trench_run_outcome(
                0,
                TacticalTrenchRunOutcome::Failure,
            ))
            .count(),
        1
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

// Trench-run result state 6 destroys the Death Star (`FUN_005cfec0`;
// `ghidra/notes/tactical-death-star-path.md`, "Trench-run result routing").
// A fleet has the Death Star only as its family-0x18 ship (`FUN_005003d0`),
// so that ship dies with the station whatever its own ship result says.
#[test]
fn a_death_star_destroyed_in_battle_leaves_its_fleet_without_one_and_its_ship_dead() {
    use crate::world::{CapitalShipClass, Fleet, ShipInstance, System};

    // The rule takes no side: either side's fleet given the class loses it.
    for owner_is_alliance in [false, true] {
        let mut world = GameWorld::default();
        let system = world.systems.insert(System::default());
        let mut class = |family, is_alliance: bool| {
            world.capital_ship_classes.insert(CapitalShipClass {
                dat_id: DatId::new(if family == 0x18 {
                    crate::world::DEATH_STAR_CLASS_ID
                } else {
                    0x1400_0001
                }),
                family,
                is_alliance,
                is_empire: !is_alliance,
                hull: 100,
                ..CapitalShipClass::default()
            })
        };
        let death_star = class(0x18, owner_is_alliance);
        let destroyer = class(0x14, owner_is_alliance);
        let cruiser = class(0x14, !owner_is_alliance);
        let owner = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![
                ShipInstance::new(destroyer, 100, owner_is_alliance),
                ShipInstance::new(death_star, 100, owner_is_alliance),
            ],
            is_alliance: owner_is_alliance,
            ..Fleet::default()
        });
        let enemy = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![ShipInstance::new(cruiser, 100, !owner_is_alliance)],
            is_alliance: !owner_is_alliance,
            ..Fleet::default()
        });
        world.systems[system].fleets = vec![owner, enemy];
        world.systems[system].fleets.sort_unstable();
        assert!(world.fleet_has_death_star(owner));

        // The owner defends.
        let mut session = BattleSession::new(&world, system, enemy, owner, true, 1, 0x5eed);
        let station = session.death_star.as_mut().expect("the owner's Death Star");
        assert!(!station.is_attacker);
        station.hull = 0.0;
        session.winner = Some(CombatWinner::Defender);

        apply::apply_results(
            &session,
            &mut world,
            &mut crate::troop_transport::TroopTransportState::default(),
            &mut crate::movement::MovementState::new(),
            &ProductRoom::default(),
            &mut Vec::new(),
        );

        assert!(!world.fleet_has_death_star(owner));
        let classes: Vec<_> = world.fleets[owner]
            .capital_ships
            .iter()
            .map(|ship| ship.class)
            .collect();
        assert_eq!(classes, [destroyer], "the Death Star ship is dead and gone");
    }
}

// FUN_00599c90.c:99-186 seats each side's retreat permission (`side+0x80`):
// port: it holds when the retreat chain (FUN_00516730 through FUN_005545e0)
// finds the side a place. The chain runs on past a side's own systems to the
// neutral ones (stage 6, FUN_005551e0) and the other side's (stage 7,
// FUN_005552e0), where a fleet is accepted (FUN_005073d0.c:103-113), so the
// side that holds nothing retreats to the other side's system.
#[test]
fn a_side_retreats_to_a_system_it_holds_else_to_the_other_sides() {
    use crate::world::{CapitalShipClass, ControlKind, Fleet, ShipInstance, System};

    for refuge_side in [crate::dat::Faction::Alliance, crate::dat::Faction::Empire] {
        let mut world = GameWorld::default();
        let system = world.systems.insert(System::default());
        world.systems.insert(System {
            x: 30,
            control: ControlKind::Controlled(refuge_side),
            ..System::default()
        });
        let mut fleet = |is_alliance: bool| {
            let class = world.capital_ship_classes.insert(CapitalShipClass {
                dat_id: DatId::new(0x1400_0001),
                family: 0x14,
                is_alliance,
                is_empire: !is_alliance,
                hull: 100,
                ..CapitalShipClass::default()
            });
            world.fleets.insert(Fleet {
                location: system,
                capital_ships: vec![ShipInstance::new(class, 100, is_alliance)],
                is_alliance,
                ..Fleet::default()
            })
        };
        let alliance = fleet(true);
        let empire = fleet(false);
        world.systems[system].fleets = vec![alliance, empire];
        world.systems[system].fleets.sort_unstable();

        let mut session = BattleSession::new(&world, system, alliance, empire, true, 1, 0x5eed);
        apply::seat_morale(
            &mut session,
            &world,
            &crate::troop_transport::TroopTransportState::default(),
            &crate::movement::MovementState::new(),
            Some(true),
        );
        assert_eq!(session.morale.retreat_permitted, [true, true], "{refuge_side:?}");
    }
}

/// A battle system and, 30 away, a system each side holds, with one fleet
/// per side at the battle: a ship (tag 1 the Alliance's, 2 the Empire's)
/// with a hyperdrive and room for four squadrons, carrying a squadron
/// without one. Returns the fleets and the refuges, the Alliance's first.
/// port: test fixture values, not rules.
fn withdrawal_world() -> (GameWorld, SystemKey, [FleetKey; 2], [SystemKey; 2]) {
    use crate::world::{
        CapitalShipClass, ControlKind, FighterClass, FighterEntry, Fleet, ShipInstance, System,
    };

    let mut world = GameWorld::default();
    let system = world.systems.insert(System::default());
    let refuges = [crate::dat::Faction::Alliance, crate::dat::Faction::Empire].map(|side| {
        world.systems.insert(System {
            x: 30,
            control: ControlKind::Controlled(side),
            ..System::default()
        })
    });
    let fleets = [(true, 1), (false, 2)].map(|(is_alliance, tag)| {
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(0x1400_0001 + tag),
            family: 0x14,
            is_alliance,
            is_empire: !is_alliance,
            hull: 100,
            hyperdrive: 1,
            fighter_capacity: 4,
            ..CapitalShipClass::default()
        });
        let squadron = world.fighter_classes.insert(FighterClass {
            dat_id: DatId::new(0x1c00_0001 + tag),
            squadron_size: 6,
            is_alliance,
            ..FighterClass::default()
        });
        let mut ship = ShipInstance::new(class, 100, is_alliance);
        ship.tag = tag;
        world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![ship],
            fighters: vec![FighterEntry {
                class: squadron,
                count: 1,
                carrier: tag,
            }],
            is_alliance,
            ..Fleet::default()
        })
    });
    world.systems[system].fleets = fleets.to_vec();
    world.last_ship_tag = 2;
    (world, system, fleets, refuges)
}

/// The battle of `withdrawal_world` won by `winner_is_alliance`'s side:
/// the side `jumper_is_alliance`'s ship jumped (state 9), the loser's ship
/// is destroyed unless it jumped, and the player is the loser.
fn battle_with_a_jump(
    world: &GameWorld,
    system: SystemKey,
    fleets: [FleetKey; 2],
    winner_is_alliance: bool,
    jumper_is_alliance: bool,
) -> BattleSession {
    let mut session = BattleSession::new(world, system, fleets[0], fleets[1], !winner_is_alliance, 0, 1);
    for ship in &mut session.ships {
        if ship.identity.is_alliance == jumper_is_alliance {
            ship.retreated = true;
            ship.alive = false;
        } else if ship.identity.is_alliance != winner_is_alliance {
            ship.alive = false;
            ship.hull_current = 0;
        }
    }
    session.destroy_squadrons_docked_in_lost_ships();
    session.winner = Some(if winner_is_alliance {
        CombatWinner::Attacker
    } else {
        CombatWinner::Defender
    });
    session.phase = BattlePhase::Results;
    session
}

// FUN_0040c780 writes the battle back without moving a ship or fleet
// (FUN_0040cfc0 hull and counters, FUN_0040d320 squadron parents,
// FUN_0040d510 overflow to the system); the only departure is
// FUN_00516ef0.c:142-180, which flags the non-winner's fleets and system
// squadrons for event 0x307. A winner whose ship jumped keeps it at the
// battle system with the squadron that docked in it at the end
// (FUN_005bef90.c:72-96).
#[test]
fn a_winner_whose_ship_jumped_stays_at_the_system_with_its_docked_squadrons() {
    for winner_is_alliance in [true, false] {
        let (mut world, system, fleets, _) = withdrawal_world();
        let mut session = battle_with_a_jump(&world, system, fleets, winner_is_alliance, winner_is_alliance);
        // Its squadron flew on after the ship left (FUN_005c8780.c:102-125).
        for fighter in session
            .fighters
            .iter_mut()
            .filter(|fighter| fighter.identity.is_alliance == winner_is_alliance)
        {
            fighter.docked = None;
            fighter.alive = true;
        }
        let mut movement = crate::movement::MovementState::new();
        let mut log = Vec::new();
        apply::apply_results(
            &session,
            &mut world,
            &mut crate::troop_transport::TroopTransportState::default(),
            &mut movement,
            &ProductRoom::default(),
            &mut log,
        );
        let winner = fleets[usize::from(!winner_is_alliance)];
        assert!(movement.get(winner).is_none(), "{winner_is_alliance}: {log:?}");
        assert!(world.systems[system].fleets.contains(&winner), "{log:?}");
        assert_eq!(
            world.fleets[winner]
                .capital_ships
                .iter()
                .map(|ship| ship.hull_current)
                .collect::<Vec<_>>(),
            vec![100],
            "the jumped ship keeps its hull (FUN_0040cfc0)"
        );
        let tag = if winner_is_alliance { 1 } else { 2 };
        assert_eq!(
            world.fleets[winner]
                .fighters
                .iter()
                .map(|entry| (entry.count, entry.carrier))
                .collect::<Vec<_>>(),
            vec![(1, tag)],
            "docked in the jumped ship"
        );
    }
}

// FUN_00516ef0.c:142-180: every fleet of the non-winner is flagged
// (FUN_004f78e0(obj, 1)) and event 0x307 moves it where the retreat chain
// places it (FUN_00515480 -> FUN_005545e0); the results name that system
// when the fleet is the player's (0x700d). The squadron docked in its jumped
// ship goes along.
#[test]
fn a_loser_that_withdrew_leaves_through_the_retreat_chain() {
    for (loser_is_alliance, player_lost) in [(true, true), (false, true), (true, false), (false, false)] {
        let (mut world, system, fleets, refuges) = withdrawal_world();
        let mut session = battle_with_a_jump(&world, system, fleets, !loser_is_alliance, loser_is_alliance);
        session.player_is_attacker = loser_is_alliance == player_lost;
        let mut movement = crate::movement::MovementState::new();
        let mut log = Vec::new();
        let result = apply::apply_results(
            &session,
            &mut world,
            &mut crate::troop_transport::TroopTransportState::default(),
            &mut movement,
            &ProductRoom::default(),
            &mut log,
        );
        let (loser, winner) = if loser_is_alliance {
            (fleets[0], fleets[1])
        } else {
            (fleets[1], fleets[0])
        };
        let refuge = refuges[usize::from(!loser_is_alliance)];
        assert_eq!(
            movement.get(loser).map(|order| order.destination),
            Some(refuge),
            "{loser_is_alliance}: {log:?}"
        );
        assert_eq!(result.withdrawal, player_lost.then_some(refuge));
        assert_eq!(world.fleets[loser].fighters.len(), 1, "its squadron rides along");
        assert!(world.systems[system].fleets.contains(&winner), "{log:?}");
    }
}

// A Death Star lost in battle clears its usable bit (FUN_004f7b80 ->
// FUN_004f7410), and event 0x301 reruns FUN_005178b0 (FUN_00514510): the
// battle's own results leave bit 11 off its sector.
#[test]
fn a_death_star_lost_in_battle_clears_its_sector_with_the_results() {
    use crate::world::{CapitalShipClass, Fleet, Sector, ShipInstance, System};

    let mut world = GameWorld::default();
    let sector = world.sectors.insert(Sector::default());
    let system = world.systems.insert(System {
        sector,
        ..System::default()
    });
    let mut class = |family, is_alliance: bool| {
        world.capital_ship_classes.insert(CapitalShipClass {
            family,
            is_alliance,
            is_empire: !is_alliance,
            hull: 100,
            ..CapitalShipClass::default()
        })
    };
    let death_star = class(0x18, false);
    let cruiser = class(0x14, true);
    let empire = world.fleets.insert(Fleet {
        location: system,
        capital_ships: vec![ShipInstance::new(death_star, 100, false)],
        ..Fleet::default()
    });
    let alliance = world.fleets.insert(Fleet {
        location: system,
        capital_ships: vec![ShipInstance::new(cruiser, 100, true)],
        is_alliance: true,
        ..Fleet::default()
    });
    world.systems[system].fleets = vec![empire, alliance];
    world.systems[system].fleets.sort_unstable();
    world.death_star_sectors.insert(sector);

    let mut session = BattleSession::new(&world, system, alliance, empire, true, 1, 0x5eed);
    for ship in session.ships.iter_mut().filter(|ship| !ship.is_attacker) {
        ship.hull_current = 0;
        ship.alive = false;
    }

    let mut log = Vec::new();
    apply::apply_results(
        &session,
        &mut world,
        &mut crate::troop_transport::TroopTransportState::default(),
        &mut crate::movement::MovementState::new(),
        &ProductRoom::default(),
        &mut log,
    );
    assert!(!world.fleet_has_death_star(empire), "{log:?}");
    assert!(world.death_star_sectors.is_empty(), "{log:?}");
}

/// A capital ship class of the side with a hyperdrive and room for
/// `fighter_capacity` squadrons, and a squadron class of the side.
/// port: test fixture values, not rules.
fn side_classes(
    world: &mut GameWorld,
    is_alliance: bool,
    fighter_capacity: u32,
) -> (crate::ids::CapitalShipKey, FighterKey) {
    let ship = world.capital_ship_classes.insert(crate::world::CapitalShipClass {
        dat_id: DatId::new(0x1400_0001),
        family: 0x14,
        is_alliance,
        is_empire: !is_alliance,
        hull: 100,
        hyperdrive: 1,
        fighter_capacity,
        ..crate::world::CapitalShipClass::default()
    });
    (ship, squadron_class(world, is_alliance))
}

/// port: test fixture values, not rules.
fn squadron_class(world: &mut GameWorld, is_alliance: bool) -> FighterKey {
    world.fighter_classes.insert(crate::world::FighterClass {
        dat_id: DatId::new(0x1c00_0001),
        squadron_size: 6,
        is_alliance,
        ..crate::world::FighterClass::default()
    })
}

/// A one-ship fleet of the side at `system` carrying `fighters`.
/// port: test fixture values, not rules.
fn one_ship_fleet(
    world: &mut GameWorld,
    system: SystemKey,
    class: crate::ids::CapitalShipKey,
    is_alliance: bool,
    fighters: Vec<crate::world::FighterEntry>,
) -> FleetKey {
    let mut ship = crate::world::ShipInstance::new(class, 100, is_alliance);
    world.last_ship_tag += 1;
    ship.tag = world.last_ship_tag;
    let fleet = world.fleets.insert(crate::world::Fleet {
        location: system,
        capital_ships: vec![ship],
        fighters,
        is_alliance,
        ..crate::world::Fleet::default()
    });
    world.systems[system].fleets.push(fleet);
    fleet
}

/// Run `apply_results` with fresh transport and movement states.
fn apply_battle(session: &BattleSession, world: &mut GameWorld) -> (apply::BattleReturn, Vec<String>) {
    let mut log = Vec::new();
    let result = apply::apply_results(
        session,
        world,
        &mut crate::troop_transport::TroopTransportState::default(),
        &mut crate::movement::MovementState::new(),
        &ProductRoom::default(),
        &mut log,
    );
    (result, log)
}

fn winner_of(winner_is_alliance: bool) -> CombatWinner {
    // The Alliance attacks in these battles.
    if winner_is_alliance {
        CombatWinner::Attacker
    } else {
        CombatWinner::Defender
    }
}

// FUN_0040c780.c:171-177 imports the system's own squadrons through
// FUN_0040d260; FUN_00503370 destroys one whose craft are all lost (slot
// +0xac, reason 0xc), and FUN_0040d320.c:54-85 keeps each survivor in no
// hangar a child of the system. A class whose squadrons all died leaves no
// entry, and each survivor stays on its own side.
#[test]
fn a_system_squadron_lost_in_battle_leaves_the_system_and_its_survivors_stay() {
    use crate::world::{ControlKind, System};

    for winner_is_alliance in [true, false] {
        let mut world = GameWorld::default();
        let system = world.systems.insert(System {
            control: ControlKind::Controlled(crate::dat::Faction::of_alliance(winner_is_alliance)),
            ..System::default()
        });
        let mut fleets = [FleetKey::default(); 2];
        let mut classes = [FighterKey::default(); 2];
        for is_alliance in [true, false] {
            // No hangar room: every squadron ends in no hangar.
            let (ship, squadron) = side_classes(&mut world, is_alliance, 0);
            fleets[usize::from(!is_alliance)] = one_ship_fleet(&mut world, system, ship, is_alliance, Vec::new());
            classes[usize::from(!is_alliance)] = squadron;
        }
        let winner_side = usize::from(!winner_is_alliance);
        let lost_class = squadron_class(&mut world, winner_is_alliance);
        world.systems[system].add_squadrons(classes[winner_side], 2, winner_is_alliance);
        world.systems[system].add_squadrons(lost_class, 1, winner_is_alliance);
        world.systems[system].add_squadrons(classes[1 - winner_side], 1, !winner_is_alliance);

        let mut session = BattleSession::new(&world, system, fleets[0], fleets[1], true, 0, 1);
        let survivor_class = classes[winner_side];
        let first = session
            .fighters
            .iter()
            .position(|fighter| fighter.class_key == survivor_class)
            .unwrap();
        session.fighters[first].destroy();
        for fighter in session.fighters.iter_mut().filter(|fighter| fighter.class_key == lost_class) {
            fighter.destroy();
        }
        for ship in session
            .ships
            .iter_mut()
            .filter(|ship| ship.identity.is_alliance != winner_is_alliance)
        {
            ship.alive = false;
            ship.hull_current = 0;
        }
        session.winner = Some(winner_of(winner_is_alliance));
        session.phase = BattlePhase::Results;
        let (_, log) = apply_battle(&session, &mut world);

        assert_eq!(
            world.systems[system]
                .fighters
                .iter()
                .map(|entry| (entry.class, entry.count, entry.is_alliance))
                .collect::<Vec<_>>(),
            vec![(survivor_class, 1, winner_is_alliance)],
            "{winner_is_alliance}: {log:?}"
        );
    }
}

// FUN_0040d320.c:54-85 makes each squadron docked at the end a child of its
// carrier's strategic ship, so a carrier holds per class the squadrons that
// docked in it, and a class all of whose squadrons died leaves no entry
// (FUN_00503370, reason 0xc).
#[test]
fn each_carrier_holds_per_class_the_squadrons_that_docked_in_it() {
    use crate::world::{ControlKind, FighterEntry, System};

    for winner_is_alliance in [true, false] {
        let mut world = GameWorld::default();
        let system = world.systems.insert(System::default());
        // The loser's fleet leaves for its side's system (FUN_00516ef0.c:142-180).
        for side in [crate::dat::Faction::Alliance, crate::dat::Faction::Empire] {
            world.systems.insert(System {
                x: 30,
                control: ControlKind::Controlled(side),
                ..System::default()
            });
        }
        let mut fleets = [FleetKey::default(); 2];
        let mut kinds = Vec::new();
        for is_alliance in [true, false] {
            let (ship, pair) = side_classes(&mut world, is_alliance, 4);
            let single = squadron_class(&mut world, is_alliance);
            let lost = squadron_class(&mut world, is_alliance);
            let tag = world.last_ship_tag + 1;
            let entry = |class, count| FighterEntry { class, count, carrier: tag };
            fleets[usize::from(!is_alliance)] = one_ship_fleet(
                &mut world,
                system,
                ship,
                is_alliance,
                vec![entry(pair, 2), entry(single, 1), entry(lost, 1)],
            );
            kinds.push((is_alliance, pair, single, lost, tag));
        }
        let mut session = BattleSession::new(&world, system, fleets[0], fleets[1], true, 0, 1);
        for &(_, _, _, lost, _) in &kinds {
            for fighter in session.fighters.iter_mut().filter(|fighter| fighter.class_key == lost) {
                fighter.destroy();
            }
        }
        session.winner = Some(winner_of(winner_is_alliance));
        session.phase = BattlePhase::Results;
        let (_, log) = apply_battle(&session, &mut world);

        for (is_alliance, pair, single, _, tag) in kinds {
            assert_eq!(
                world.fleets[fleets[usize::from(!is_alliance)]]
                    .fighters
                    .iter()
                    .map(|entry| (entry.class, entry.count, entry.carrier))
                    .collect::<Vec<_>>(),
                vec![(pair, 2, tag), (single, 1, tag)],
                "{winner_is_alliance} {is_alliance}: {log:?}"
            );
        }
    }
}

// FUN_0040b0e0 exports every fleet a side has at the system, so the
// winner's lead fleet may be lost while another of its fleets holds the
// system. The post-battle steps follow the lead while it lives, else the
// side's first usable fleet there: the original walks a system's fleets in
// mode 1 (FUN_00509530, FUN_004ffef0(system, 1, side); FUN_00516ef0.c:63-73
// and :143; FUN_0040b0e0.c:158), which skips a fleet whose ships are all on
// their own legs. `apply::apply_results` picks it.
#[test]
fn the_post_battle_steps_follow_the_winners_surviving_fleet() {
    use crate::world::{ControlKind, ShipTransit, System};

    for (winner_is_alliance, lead_lost) in [(true, true), (false, true), (true, false), (false, false)] {
        let mut world = GameWorld::default();
        // The loser's own system, and another of its own 30 away that its
        // fleet leaves for (VM, `empire-day660-deathstar-pre-alert.sav`: a
        // loser at its own held Carida and Ando ended at Halowan).
        let system = world.systems.insert(System {
            control: ControlKind::Controlled(crate::dat::Faction::of_alliance(!winner_is_alliance)),
            ..System::default()
        });
        let refuge = world.systems.insert(System {
            x: 30,
            control: ControlKind::Controlled(crate::dat::Faction::of_alliance(!winner_is_alliance)),
            ..System::default()
        });
        let (winner_ship, _) = side_classes(&mut world, winner_is_alliance, 0);
        let (loser_ship, _) = side_classes(&mut world, !winner_is_alliance, 0);
        let lead = one_ship_fleet(&mut world, system, winner_ship, winner_is_alliance, Vec::new());
        let other = one_ship_fleet(&mut world, system, winner_ship, winner_is_alliance, Vec::new());
        let loser = one_ship_fleet(&mut world, system, loser_ship, !winner_is_alliance, Vec::new());
        // A fleet of the winner whose only ship is on its own leg, listed
        // first.
        let travelling = one_ship_fleet(&mut world, system, winner_ship, winner_is_alliance, Vec::new());
        world.fleets[travelling].capital_ships[0].transit = Some(ShipTransit {
            from: system,
            to: system,
            arrival_tick: 10,
        });
        world.systems[system].fleets = if lead_lost {
            vec![travelling, lead, other, loser]
        } else {
            // The other fleet comes first at the system.
            vec![travelling, other, lead, loser]
        };
        let (attacker, defender) = if winner_is_alliance { (lead, loser) } else { (loser, lead) };
        let mut session = BattleSession::new(&world, system, attacker, defender, true, 0, 1);
        if lead_lost {
            for ship in session.ships.iter_mut().filter(|ship| ship.fleet == Some(lead)) {
                ship.alive = false;
                ship.hull_current = 0;
            }
        }
        session.winner = Some(winner_of(winner_is_alliance));
        session.phase = BattlePhase::Results;
        let (result, log) = apply_battle(&session, &mut world);

        assert!(
            log.contains(&format!(
                "[battle] post-battle fleet={loser:?} outcome=Moved({refuge:?})"
            )),
            "{log:?}"
        );
        let expected = if lead_lost { other } else { lead };
        assert_eq!(
            result.winner_fleet,
            Some(expected),
            "{winner_is_alliance} {lead_lost}: {log:?}"
        );
    }
}

// The mode-1 walk (FUN_00509530, FUN_004ffef0(system, 1, side)) skips a
// fleet with no usable ship, so a lead that survives with only a ship on
// its own leg, its fighting ship lost, does not lead the post-battle steps:
// the side's first usable fleet does. `apply::apply_results` picks it.
#[test]
fn a_surviving_lead_without_a_usable_ship_hands_the_steps_to_the_next_fleet() {
    use crate::world::{ControlKind, ShipTransit, System};

    for winner_is_alliance in [true, false] {
        let mut world = GameWorld::default();
        let system = world.systems.insert(System {
            control: ControlKind::Controlled(crate::dat::Faction::of_alliance(!winner_is_alliance)),
            ..System::default()
        });
        let (winner_ship, _) = side_classes(&mut world, winner_is_alliance, 0);
        let (loser_ship, _) = side_classes(&mut world, !winner_is_alliance, 0);
        let lead = one_ship_fleet(&mut world, system, winner_ship, winner_is_alliance, Vec::new());
        let mut travelling = crate::world::ShipInstance::new(winner_ship, 100, winner_is_alliance);
        travelling.transit = Some(ShipTransit {
            from: system,
            to: system,
            arrival_tick: 10,
        });
        world.fleets[lead].capital_ships.push(travelling);
        let other = one_ship_fleet(&mut world, system, winner_ship, winner_is_alliance, Vec::new());
        let loser = one_ship_fleet(&mut world, system, loser_ship, !winner_is_alliance, Vec::new());
        let (attacker, defender) = if winner_is_alliance { (lead, loser) } else { (loser, lead) };
        let mut session = BattleSession::new(&world, system, attacker, defender, true, 0, 1);
        for ship in session.ships.iter_mut().filter(|ship| ship.fleet == Some(lead)) {
            ship.alive = false;
            ship.hull_current = 0;
        }
        session.winner = Some(winner_of(winner_is_alliance));
        session.phase = BattlePhase::Results;
        let (result, log) = apply_battle(&session, &mut world);

        assert!(world.fleets.contains_key(lead), "the travelling ship keeps it");
        assert!(!world.fleets[lead].usable());
        assert_eq!(result.winner_fleet, Some(other), "{winner_is_alliance}: {log:?}");
    }
}

// FUN_0040b0e0 exports only usable fleets' children (the mode-1 walk
// FUN_004ffef0(system, 1, side)): a lead fleet whose only ship is on its
// own leg puts neither that ship nor the squadron aboard it in the battle,
// and the side fights with its usable fleet.
#[test]
fn a_lead_fleet_without_a_usable_ship_puts_nothing_in_the_battle() {
    use crate::world::{FighterEntry, ShipTransit, System};

    for is_alliance in [true, false] {
        let mut world = GameWorld::default();
        let system = world.systems.insert(System::default());
        let (ship, squadron) = side_classes(&mut world, is_alliance, 4);
        let tag = world.last_ship_tag + 1;
        let lead = one_ship_fleet(
            &mut world,
            system,
            ship,
            is_alliance,
            vec![FighterEntry {
                class: squadron,
                count: 1,
                carrier: tag,
            }],
        );
        world.fleets[lead].capital_ships[0].transit = Some(ShipTransit {
            from: system,
            to: system,
            arrival_tick: 10,
        });
        let standing = one_ship_fleet(&mut world, system, ship, is_alliance, Vec::new());
        let (enemy_ship, _) = side_classes(&mut world, !is_alliance, 0);
        let enemy = one_ship_fleet(&mut world, system, enemy_ship, !is_alliance, Vec::new());
        let (attacker, defender) = if is_alliance { (lead, enemy) } else { (enemy, lead) };
        let session = BattleSession::new(&world, system, attacker, defender, true, 0, 1);
        assert!(session.ships.iter().all(|ship| ship.fleet != Some(lead)), "{is_alliance}");
        assert!(session.fighters.iter().all(|fighter| fighter.fleet != Some(lead)), "{is_alliance}");
        assert!(session.ships.iter().any(|ship| ship.fleet == Some(standing)));
    }
}

// FUN_005d14d0.c:124-127 commits an operational Death Star (`+0x78`) when
// its side retreats, so it leaves even when no hyperdrive of the side works
// (FUN_005d0f50.c:32-40 lets the Empire's Death Star carry the retreat; the
// Alliance never holds that permission). The side, with nothing left in the
// battle, loses (FUN_005a9ec0.c:274-286), and FUN_00516ef0.c:142-180 sends
// its fleet, the Death Star ship aboard, along the retreat chain.
#[test]
fn a_death_star_that_carries_its_sides_retreat_leaves_with_its_fleet() {
    use crate::world::{CapitalShipClass, ControlKind, Fleet, ShipInstance, System};

    let mut world = GameWorld::default();
    let system = world.systems.insert(System::default());
    let refuge = world.systems.insert(System {
        x: 30,
        control: ControlKind::Controlled(crate::dat::Faction::Empire),
        ..System::default()
    });
    let mut class = |family, is_alliance: bool| {
        world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(if family == 0x18 {
                crate::world::DEATH_STAR_CLASS_ID
            } else {
                0x1400_0001
            }),
            family,
            is_alliance,
            is_empire: !is_alliance,
            hull: 100,
            hyperdrive: 1,
            ..CapitalShipClass::default()
        })
    };
    let death_star = class(0x18, false);
    let cruiser = class(0x14, true);
    let empire = world.fleets.insert(Fleet {
        location: system,
        capital_ships: vec![ShipInstance::new(death_star, 100, false)],
        ..Fleet::default()
    });
    let alliance = world.fleets.insert(Fleet {
        location: system,
        capital_ships: vec![ShipInstance::new(cruiser, 100, true)],
        is_alliance: true,
        ..Fleet::default()
    });
    world.systems[system].fleets = vec![empire, alliance];
    world.systems[system].fleets.sort_unstable();

    let mut session = BattleSession::new(&world, system, alliance, empire, true, 1, 0x5eed);
    for ship in session.ships.iter_mut().filter(|ship| !ship.is_attacker) {
        ship.subsystem_condition.hyperdrive = 0;
    }
    session.morale.retreat_permitted[1] = true;
    assert!(session.may_retreat(false));
    session.retreat_side(false);
    assert!(session.finish_if_decided());
    assert_eq!(session.winner, Some(CombatWinner::Attacker));

    let mut movement = crate::movement::MovementState::new();
    let mut log = Vec::new();
    apply::apply_results(
        &session,
        &mut world,
        &mut crate::troop_transport::TroopTransportState::default(),
        &mut movement,
        &ProductRoom::default(),
        &mut log,
    );
    assert!(world.fleet_has_death_star(empire), "{log:?}");
    assert_eq!(
        movement.get(empire).map(|order| order.destination),
        Some(refuge),
        "{log:?}"
    );
}

// The result object's +0x5c names the holder when its system was under
// blockade as the battle began: a hostile fleet there and none of the
// holder's; a neutral system is never blockaded.
#[test]
fn a_battle_starts_blockaded_only_at_a_held_system_without_the_holders_fleet() {
    use crate::dat::Faction;
    use crate::world::ControlKind;
    let (mut world, system) = super::hangar_tests::carried_world();
    let [alliance, empire] = [world.systems[system].fleets[0], world.systems[system].fleets[1]];
    let start = |world: &GameWorld| BattleSession::new(world, system, alliance, empire, true, 0, 1);
    for side in [Faction::Alliance, Faction::Empire] {
        world.systems[system].control = ControlKind::Controlled(side);
        assert!(!start(&world).blockaded_at_start, "{side:?}: both fleets are there");
    }
    for (side, present, blockaded) in [
        (Faction::Empire, alliance, true),
        (Faction::Alliance, empire, true),
        (Faction::Alliance, alliance, false),
        (Faction::Empire, empire, false),
    ] {
        world.systems[system].fleets = vec![present];
        world.systems[system].control = ControlKind::Controlled(side);
        assert_eq!(start(&world).blockaded_at_start, blockaded, "{side:?} holds");
    }
    world.systems[system].fleets = vec![alliance];
    world.systems[system].control = ControlKind::Uncontrolled;
    assert!(!start(&world).blockaded_at_start, "a neutral system");
}

// FUN_005d0f50: a live opposing ship whose class projects a gravity well
// (hyp: FUN_005c7e40's unit +0x150) bars the retreat; the session lists
// those ships.
#[test]
fn the_ships_whose_class_projects_a_gravity_well_are_listed() {
    let (mut world, system) = super::hangar_tests::carried_world();
    let empire = world.systems[system].fleets[1];
    let class = world.fleets[empire].capital_ships[0].class;
    world.capital_ship_classes[class].gravity_well_projector = 1;
    let session = super::hangar_tests::battle_of(&world, system);
    let well = session.ships.iter().position(|ship| !ship.identity.is_alliance).unwrap();
    assert_eq!(session.morale.gravity_wells, [well]);
}

// FUN_0040b0e0 walks a fleet's ships in mode 1 (FUN_005001b0(fleet, 1) at
// :164, FUN_005003d0(fleet, 1) at :175): a usable ship fights, one still on
// its own leg from its yard (`+0x50` bit 5, FUN_00556430) stays out.
#[test]
fn a_ship_on_its_own_leg_does_not_fight_for_either_side() {
    use crate::world::{CapitalShipClass, Fleet, ShipInstance, ShipTransit, System};

    for is_alliance in [true, false] {
        let mut world = GameWorld::default();
        let origin = world.systems.insert(System::default());
        let system = world.systems.insert(System::default());
        let mut fleet = |is_alliance: bool, ships: u32| {
            let class = world.capital_ship_classes.insert(CapitalShipClass {
                dat_id: DatId::new(0x1400_0001),
                family: 0x14,
                is_alliance,
                is_empire: !is_alliance,
                hull: 100,
                ..CapitalShipClass::default()
            });
            world.fleets.insert(Fleet {
                location: system,
                capital_ships: ShipInstance::make(class, 100, is_alliance, ships),
                is_alliance,
                ..Fleet::default()
            })
        };
        let attacker = fleet(is_alliance, 2);
        let defender = fleet(!is_alliance, 1);
        world.fleets[attacker].capital_ships[1].transit = Some(ShipTransit {
            from: origin,
            to: system,
            arrival_tick: 9,
        });
        world.systems[system].fleets = vec![attacker, defender];
        world.systems[system].fleets.sort_unstable();

        let session = BattleSession::new(&world, system, attacker, defender, true, 1, 0x5eed);
        let indices: Vec<usize> = session
            .ships
            .iter()
            .filter(|ship| ship.fleet == Some(attacker))
            .map(|ship| ship.fleet_ship_index)
            .collect();
        assert_eq!(indices, [0], "{is_alliance}");
    }
}
