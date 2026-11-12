//! Battle fixtures shared by the engine tests and rebellion-render's view tests.

use super::resource_ids::{capital_ship_tactical_resource, fighter_tactical_resource};
#[allow(
    clippy::wildcard_imports,
    reason = "the shared fixture builders intentionally mirror the tactical module's data model"
)]
use super::*;

/// port: test fixture values, not rules.
#[must_use]
pub fn test_ship(dat_id: u32, roster: usize, is_alliance: bool, alive: bool) -> TacticalShip {
    TacticalShip {
        class_key: CapitalShipKey::default(),
        identity: TacticalObjectIdentity {
            class_dat_id: DatId::new(dat_id),
            fleet_roster_index: roster,
            is_alliance,
        },
        tactical_resource: capital_ship_tactical_resource(DatId::new(dat_id)),
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
        task_force_joined: 0,
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
        outstanding_hits: 0,
        repair_timer_steps: 0,
    }
}

/// port: test fixture values, not rules.
#[must_use]
pub fn test_fighter(dat_id: u32, is_alliance: bool) -> TacticalFighter {
    TacticalFighter {
        held_by_system: false,
        off_field: false,
        carrier: None,
        docked: None,
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
#[must_use]
pub fn test_session(
    ships: Vec<TacticalShip>,
    fighters: Vec<TacticalFighter>,
    player_is_attacker: bool,
) -> BattleSession {
    BattleSession {
        system_name: "Test System".to_string(),
        system_picture_id: 1,
        attacker_fleet: Some(FleetKey::default()),
        defender_fleet: Some(FleetKey::default()),
        ..BattleSession::baseline(
            SystemKey::default(),
            ships,
            fighters,
            player_is_attacker,
            OriginalTacticalLayout::default(),
            0,
            0x5eed_ba77,
            OriginalBattleRng::default(),
        )
    }
}

/// port: an Imperial station of the side `is_attacker`, completed, with a
/// 100-point hull, no charge, no shield and no hangar.
#[must_use]
pub fn test_death_star(is_attacker: bool) -> TacticalDeathStar {
    TacticalDeathStar {
        resource: DEATH_STAR_TACTICAL_RESOURCE,
        is_attacker,
        is_alliance: false,
        source_position: TacticalWorldPosition::ORIGIN,
        hull_max: 100.0,
        hull: 100.0,
        laser_charge: 0.0,
        shielded: false,
        not_completed: false,
        action_committed: false,
        hangar_capacity: 0,
        ship: None,
    }
}
