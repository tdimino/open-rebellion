//! Test-only, versioned direct entry into the production tactical battle scene.

use rebellion_core::ids::{CapitalShipKey, FighterKey, SystemKey};
use rebellion_core::missions::MissionFaction;
use rebellion_core::world::{FighterEntry, Fleet, GameWorld, ShipInstance};
use rebellion_render::{CockpitFaction, CockpitState, MessageLog, TacticalState};
use serde::Serialize;

use crate::tactical_flow::{self, BattleEntry};
use crate::GameMode;

const TACTICAL_FAMILY: u32 = 1;
const BATTLE_SCENARIO: u32 = 1;

extern "C" {
    fn open_rebellion_interface_fixture_code() -> u32;
    fn open_rebellion_interface_fixture_emit(ptr: *const u8, len: usize);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TacticalFixtureRequest {
    pub faction: CockpitFaction,
    pub code: u32,
}

fn decode(code: u32) -> Option<TacticalFixtureRequest> {
    if code >> 16 != TACTICAL_FAMILY || code & 0xff != BATTLE_SCENARIO {
        return None;
    }
    let faction = match (code >> 8) & 0xff {
        1 => CockpitFaction::Alliance,
        2 => CockpitFaction::Empire,
        _ => return None,
    };
    Some(TacticalFixtureRequest { faction, code })
}

pub(crate) fn requested() -> Option<TacticalFixtureRequest> {
    decode(unsafe { open_rebellion_interface_fixture_code() })
}

/// Add a minimal opposing encounter and enter it through the same function as
/// campaign-triggered battles. This function is not compiled into production.
#[expect(
    clippy::too_many_arguments,
    reason = "Keep every mutated campaign state explicit at this test-only bridge."
)]
pub(crate) fn apply(
    request: TacticalFixtureRequest,
    world: &mut GameWorld,
    tactical: &mut TacticalState,
    cooldowns: &mut std::collections::HashMap<SystemKey, u64>,
    messages: &mut MessageLog,
    player_faction: &mut MissionFaction,
    cockpit: &mut CockpitState,
    game_mode: &mut GameMode,
) -> Result<(), String> {
    let system = world
        .systems
        .iter()
        .find(|(_, system)| !system.is_destroyed && system.fleets.is_empty())
        .map(|(key, _)| key)
        .ok_or("no empty, intact system for the battle fixture")?;
    let alliance_ship = world
        .capital_ship_classes
        .iter()
        .find(|(_, class)| class.is_alliance && !class.is_empire && class.hull > 0)
        .map(|(key, class)| (key, class.hull))
        .ok_or("no Alliance capital-ship class")?;
    let empire_ship = world
        .capital_ship_classes
        .iter()
        .find(|(_, class)| class.is_empire && !class.is_alliance && class.hull > 0)
        .map(|(key, class)| (key, class.hull))
        .ok_or("no Imperial capital-ship class")?;
    let alliance_fighter = world
        .fighter_classes
        .iter()
        .find(|(_, class)| class.is_alliance && !class.is_empire)
        .map(|(key, _)| key);
    let empire_fighter = world
        .fighter_classes
        .iter()
        .find(|(_, class)| class.is_empire && !class.is_alliance)
        .map(|(key, _)| key);

    let make_fleet =
        |is_alliance: bool, ship: CapitalShipKey, hull: u32, fighter: Option<FighterKey>| Fleet {
            location: system,
            capital_ships: vec![ShipInstance::new(
                ship,
                i32::try_from(hull).unwrap_or(i32::MAX),
                is_alliance,
            )],
            fighters: fighter
                .map(|class| vec![FighterEntry { class, count: 12 }])
                .unwrap_or_default(),
            characters: vec![],
            is_alliance,
            has_death_star: false,
        };
    let attacker = world.fleets.insert(make_fleet(
        true,
        alliance_ship.0,
        alliance_ship.1,
        alliance_fighter,
    ));
    let defender = world.fleets.insert(make_fleet(
        false,
        empire_ship.0,
        empire_ship.1,
        empire_fighter,
    ));
    world.systems[system].fleets.extend([attacker, defender]);

    let player_is_attacker = request.faction == CockpitFaction::Alliance;
    tactical_flow::begin_player_battle(
        world,
        BattleEntry {
            system,
            attacker,
            defender,
            player_is_attacker,
            tick: 0,
        },
        tactical,
        cooldowns,
        messages,
        game_mode,
    )
    .map_err(|error| format!("fixture battle entry failed: {error:?}"))?;
    *player_faction = if player_is_attacker {
        MissionFaction::Alliance
    } else {
        MissionFaction::Empire
    };
    cockpit.faction = request.faction;
    Ok(())
}

#[derive(Serialize)]
struct FixtureRecord<'a> {
    schema_version: u32,
    status: &'a str,
    family: &'a str,
    fixture_code: u32,
    faction: &'a str,
    system: &'a str,
    attacker_ships: usize,
    defender_ships: usize,
    fighters: usize,
    error: Option<&'a str>,
}

pub(crate) fn emit_ready(request: TacticalFixtureRequest, tactical: &TacticalState) {
    let Some(session) = tactical.session.as_ref() else {
        emit_failed(request, "battle session missing after fixture entry");
        return;
    };
    emit(&FixtureRecord {
        schema_version: 1,
        status: "battle-ready",
        family: "tactical",
        fixture_code: request.code,
        faction: if request.faction == CockpitFaction::Alliance {
            "alliance"
        } else {
            "empire"
        },
        system: &session.system_name,
        attacker_ships: session.ships.iter().filter(|ship| ship.is_attacker).count(),
        defender_ships: session
            .ships
            .iter()
            .filter(|ship| !ship.is_attacker)
            .count(),
        fighters: session.fighters.len(),
        error: None,
    });
}

pub(crate) fn emit_failed(request: TacticalFixtureRequest, error: &str) {
    emit(&FixtureRecord {
        schema_version: 1,
        status: "failed",
        family: "tactical",
        fixture_code: request.code,
        faction: if request.faction == CockpitFaction::Alliance {
            "alliance"
        } else {
            "empire"
        },
        system: "",
        attacker_ships: 0,
        defender_ships: 0,
        fighters: 0,
        error: Some(error),
    });
}

fn emit(record: &FixtureRecord<'_>) {
    let bytes = serde_json::to_vec(record).expect("serialize tactical fixture record");
    unsafe { open_rebellion_interface_fixture_emit(bytes.as_ptr(), bytes.len()) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tactical_codes_are_versioned_and_do_not_accept_gid_codes() {
        assert_eq!(decode(0x10101).unwrap().faction, CockpitFaction::Alliance);
        assert_eq!(decode(0x10201).unwrap().faction, CockpitFaction::Empire);
        assert!(decode(0x10100).is_none());
        assert!(decode(0x10102).is_none());
        assert!(decode(0x10301).is_none());
        assert!(decode(0x00101).is_none());
    }
}
