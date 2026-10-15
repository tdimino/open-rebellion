//! Battle fixtures shared by the engine tests and rebellion-render's view tests.

use super::resource_ids::{
    capital_ship_tactical_resource, death_star_tactical_resource, fighter_tactical_resource,
};
use super::*;

/// port: test fixture values, not rules.
pub fn test_ship(dat_id: u32, roster: usize, is_alliance: bool, alive: bool) -> TacticalShip {
    TacticalShip {
        class_key: CapitalShipKey::default(),
        identity: TacticalObjectIdentity {
            class_dat_id: DatId::new(dat_id),
            fleet_roster_index: roster,
            is_alliance,
        },
        tactical_resource: capital_ship_tactical_resource(DatId::new(dat_id)),
        death_star_resource: death_star_tactical_resource(DatId::new(dat_id)),
        source_position: TacticalWorldPosition::ORIGIN,
        source_forward: if is_alliance {
            TacticalWorldVector::SOURCE_FORWARD
        } else {
            TacticalWorldVector {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            }
        },
        source_desired_forward: if is_alliance {
            TacticalWorldVector::SOURCE_FORWARD
        } else {
            TacticalWorldVector {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            }
        },
        source_velocity: TacticalWorldVector::ZERO,
        source_waypoint: None,
        navigation_route: Vec::new(),
        source_collision_envelope: None,
        name: format!("ship-{dat_id}"),
        x: 0.0,
        y: 0.0,
        hull_current: 1,
        hull_max: 1,
        shield: 0,
        shield_max: 0,
        subsystem_condition: TacticalSubsystemCondition {
            shields: 0,
            weapons: 0,
            tractor: 0,
            engines: 0,
            hyperdrive: 0,
        },
        subsystem_capacity: TacticalSubsystemCapacity {
            shields: false,
            weapons: false,
            tractor: false,
            engines: false,
            hyperdrive: 0,
        },
        subsystem_damage: TacticalSubsystemDamage::default(),
        sublight_engine_power: 100.0,
        maneuverability: 0,
        tractor_beam_power: 25.0,
        fighter_capacity: 1,
        maneuver_state_value: None,
        damage_control: 100,
        is_attacker: is_alliance,
        alive,
        selected: false,
        order: TacticalOrder::None,
        tactic: TacticalTactic::StandOff,
        task_force: u8::try_from(roster.min(7)).unwrap_or(7),
        fleet: None,
        fleet_ship_index: roster,
        sprite_id: None,
        weapon_arcs: [TacticalWeaponArc::default(); 4],
        weapon_ranges: TacticalWeaponRanges::default(),
        weapon_recharge_rate: 0.0,
        shield_recharge_rate: 0.0,
        shield_recharge_carry: 0.0,
        weapon_recharge_queue: Vec::new(),
        attack_target: None,
        manual_targets: Vec::new(),
        escort_target: None,
        contents: Vec::new(),
        retreating: false,
        retreat_progress: 0.0,
        retreated: false,
    }
}

/// port: test fixture values, not rules.
pub fn test_fighter(dat_id: u32, is_alliance: bool) -> TacticalFighter {
    TacticalFighter {
        held_by_system: false,
        class_key: FighterKey::default(),
        identity: TacticalObjectIdentity {
            class_dat_id: DatId::new(dat_id),
            fleet_roster_index: 0,
            is_alliance,
        },
        tactical_resource: fighter_tactical_resource(DatId::new(dat_id)),
        source_position: TacticalWorldPosition::ORIGIN,
        sublight_engine_power: 10.0,
        fleet: None,
        fleet_fighter_index: 0,
        fleet_squadron_index: 0,
        name: format!("fighter-{dat_id}"),
        x: 0.0,
        y: 0.0,
        squad_count: 12,
        hull_current: 96.0,
        hull_max: 96.0,
        shield: 5.0,
        shield_max: 5.0,
        weapon_arc: TacticalWeaponArc::new(5, 0, 0),
        weapon_ranges: TacticalWeaponRanges {
            laser_cannon: 12.0,
            ion_cannon: 0.0,
            turbolaser: 0.0,
        },
        torpedo_strength: 0,
        torpedo_range: 0.0,
        maneuverability: 5,
        is_attacker: is_alliance,
        alive: true,
        fighter_group: 0,
        selected: false,
        order: TacticalOrder::None,
        tactic: TacticalTactic::StandOff,
        attack_target: None,
        escort_target: None,
        recovery_state: TacticalFighterRecoveryState::AwaitingCarrier,
        recovery_target: None,
    }
}

/// port: test fixture values, not rules.
pub fn test_session(
    ships: Vec<TacticalShip>,
    fighters: Vec<TacticalFighter>,
    player_is_attacker: bool,
) -> BattleSession {
    BattleSession {
        system: SystemKey::default(),
        system_name: "Test System".to_string(),
        system_picture_id: 1,
        attacker_fleet: Some(FleetKey::default()),
        defender_fleet: Some(FleetKey::default()),
        attacker_is_alliance: true,
        player_is_attacker,
        phase: BattlePhase::Combat,
        ships,
        fighters,
        death_star: None,
        death_star_beam: None,
        trench_run_outcome: None,
        trench_run_remaining_ms: None,
        trench_run_group: None,
        trench_run_success_rating: 1,
        trench_run_pending_outcome: None,
        trench_run_participants: Vec::new(),
        trench_run_chatter: [ORIGINAL_TRENCH_RUN_SILENT_EVENT; 9],
        trench_run_chatter_cursor: 0,
        trench_run_cinematic_pending: false,
        navigation_sets: original_navigation_point_sets(OriginalTacticalLayout::default()),
        source_layout: OriginalTacticalLayout::default(),
        selected_ship: None,
        selected_fighter_group: None,
        placement_confirmed: true,
        start_tick: 0,
        combat_tick: 0,
        tactical_rng: OriginalTacticalRng::new(0x5eed_ba77),
        weapon_effects: Vec::new(),
        impact_effects: Vec::new(),
        pending_audio_cues: Vec::new(),
        pending_voice_cues: Vec::new(),
        pending_log_lines: Vec::new(),
        field_effects: Vec::new(),
        subsystem_repairs: Vec::new(),
        paused: true,
        combat_speed: 1,
        step_accumulator: 0.0,
        winner: None,
        stalemate: TacticalStalemateWatch::default(),
        engagement: TacticalEngagementState::default(),
        holder_is_alliance: None,
        blockaded_at_start: false,
        withdrawal_destination: None,
    }
}
