//! Test-only, versioned direct entry into the production tactical battle scene.

use rebellion_core::ids::{CapitalShipKey, FighterKey, SystemKey};
use rebellion_core::missions::MissionFaction;
use rebellion_core::world::{FighterEntry, Fleet, GameWorld, ShipInstance};
#[cfg(feature = "interface-test-fixtures")]
use rebellion_render::TacticalLodView;
use rebellion_render::{CockpitFaction, CockpitState, MessageLog, TacticalState};
use serde::Serialize;

use crate::tactical_flow::{self, BattleEntry};
use crate::GameMode;

const TACTICAL_FAMILY: u32 = 1;
const BATTLE_SCENARIO: u32 = 1;
const BATTLE_WITHOUT_PROOF_SCENARIO: u32 = 2;
const LOD_CLOSE_SCENARIO: u32 = 3;
const LOD_MEDIUM_SCENARIO: u32 = 4;
const LOD_FAR_SCENARIO: u32 = 5;
const LOD_JOURNEY_SCENARIO: u32 = 6;
const CAMERA_JOURNEY_SCENARIO: u32 = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TacticalLodFixture {
    Default,
    Close,
    Medium,
    Far,
    Journey,
    CameraJourney,
}

impl TacticalLodFixture {
    const fn label(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Close => "close",
            Self::Medium => "medium",
            Self::Far => "far",
            Self::Journey => "journey",
            Self::CameraJourney => "camera-journey",
        }
    }

    #[cfg(feature = "interface-test-fixtures")]
    const fn view(self) -> Option<TacticalLodView> {
        match self {
            Self::Default => None,
            Self::Close => Some(TacticalLodView::CLOSE_FIXTURE),
            Self::Medium => Some(TacticalLodView::MEDIUM_FIXTURE),
            Self::Far => Some(TacticalLodView::FAR_FIXTURE),
            Self::Journey => None,
            Self::CameraJourney => None,
        }
    }
}

extern "C" {
    fn open_rebellion_interface_fixture_code() -> u32;
    fn open_rebellion_interface_fixture_emit(ptr: *const u8, len: usize);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TacticalFixtureRequest {
    pub faction: CockpitFaction,
    pub code: u32,
    pub proof_enabled: bool,
    lod_fixture: TacticalLodFixture,
}

fn decode(code: u32) -> Option<TacticalFixtureRequest> {
    if code >> 16 != TACTICAL_FAMILY {
        return None;
    }
    let (proof_enabled, lod_fixture) = match code & 0xff {
        BATTLE_SCENARIO => (true, TacticalLodFixture::Default),
        BATTLE_WITHOUT_PROOF_SCENARIO => (false, TacticalLodFixture::Default),
        LOD_CLOSE_SCENARIO => (true, TacticalLodFixture::Close),
        LOD_MEDIUM_SCENARIO => (true, TacticalLodFixture::Medium),
        LOD_FAR_SCENARIO => (true, TacticalLodFixture::Far),
        LOD_JOURNEY_SCENARIO => (true, TacticalLodFixture::Journey),
        CAMERA_JOURNEY_SCENARIO => (true, TacticalLodFixture::CameraJourney),
        _ => return None,
    };
    let faction = match (code >> 8) & 0xff {
        1 => CockpitFaction::Alliance,
        2 => CockpitFaction::Empire,
        _ => return None,
    };
    Some(TacticalFixtureRequest {
        faction,
        code,
        proof_enabled,
        lod_fixture,
    })
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
    // Exercise two real SYSTEMSD picture/palette identities across the faction
    // matrix instead of accepting a hard-coded global palette accidentally.
    let system_dat_id = if request.faction == CockpitFaction::Alliance {
        100
    } else {
        101
    };
    let system = world
        .systems
        .iter()
        .find(|(_, system)| {
            system.dat_id.index() == system_dat_id
                && !system.is_destroyed
                && system.fleets.is_empty()
        })
        .map(|(key, _)| key)
        .ok_or("fixture system is not empty and intact")?;
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
    #[cfg(feature = "interface-test-fixtures")]
    if request.proof_enabled {
        tactical.enable_resource_2560_proof();
        if request.lod_fixture == TacticalLodFixture::Journey {
            tactical.enable_tactical_lod_journey();
        } else if request.lod_fixture == TacticalLodFixture::CameraJourney {
            tactical.enable_original_camera_proof(request.faction == CockpitFaction::Empire);
        } else if let Some(view) = request.lod_fixture.view() {
            tactical.set_tactical_lod_fixture(view);
        }
    }
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
    proof_enabled: bool,
    tactical_lod: &'a str,
    system: &'a str,
    system_picture_id: u8,
    palette_resource_id: u32,
    attacker_ships: usize,
    defender_ships: usize,
    fighters: usize,
    source_layout: Option<FixtureSourceLayout>,
    participants: Vec<FixtureParticipant<'a>>,
    error: Option<&'a str>,
}

#[derive(Serialize)]
struct FixtureSourceLayout {
    first_active_objects: u16,
    second_active_objects: u16,
    battle_extent: f32,
    outer_positive_z: f32,
    outer_negative_z: f32,
    inner_negative_z: f32,
    inner_positive_z: f32,
}

#[derive(Serialize)]
struct FixtureParticipant<'a> {
    kind: &'static str,
    name: &'a str,
    faction: &'static str,
    class_dat_id: u32,
    fleet_roster_index: usize,
    source_position: [f32; 3],
}

pub(crate) fn emit_ready(request: TacticalFixtureRequest, tactical: &TacticalState) {
    let Some(session) = tactical.session.as_ref() else {
        emit_failed(request, "battle session missing after fixture entry");
        return;
    };
    let layout = session.source_layout;
    let mut participants = Vec::with_capacity(session.ships.len() + session.fighters.len());
    participants.extend(session.ships.iter().map(|ship| FixtureParticipant {
        kind: "capital-ship",
        name: &ship.name,
        faction: if ship.identity.is_alliance {
            "alliance"
        } else {
            "empire"
        },
        class_dat_id: ship.identity.class_dat_id.index(),
        fleet_roster_index: ship.identity.fleet_roster_index,
        source_position: [
            ship.source_position.x,
            ship.source_position.y,
            ship.source_position.z,
        ],
    }));
    participants.extend(session.fighters.iter().map(|fighter| FixtureParticipant {
        kind: "fighter-group",
        name: &fighter.name,
        faction: if fighter.identity.is_alliance {
            "alliance"
        } else {
            "empire"
        },
        class_dat_id: fighter.identity.class_dat_id.index(),
        fleet_roster_index: fighter.identity.fleet_roster_index,
        source_position: [
            fighter.source_position.x,
            fighter.source_position.y,
            fighter.source_position.z,
        ],
    }));
    emit(&FixtureRecord {
        schema_version: 3,
        status: "battle-ready",
        family: "tactical",
        fixture_code: request.code,
        faction: if request.faction == CockpitFaction::Alliance {
            "alliance"
        } else {
            "empire"
        },
        proof_enabled: request.proof_enabled,
        tactical_lod: request.lod_fixture.label(),
        system: &session.system_name,
        system_picture_id: session.system_picture_id,
        palette_resource_id: 5530 + u32::from(session.system_picture_id),
        attacker_ships: session.ships.iter().filter(|ship| ship.is_attacker).count(),
        defender_ships: session
            .ships
            .iter()
            .filter(|ship| !ship.is_attacker)
            .count(),
        fighters: session.fighters.len(),
        source_layout: Some(FixtureSourceLayout {
            first_active_objects: layout.first_active_objects,
            second_active_objects: layout.second_active_objects,
            battle_extent: layout.battle_extent,
            outer_positive_z: layout.outer_positive_z,
            outer_negative_z: layout.outer_negative_z,
            inner_negative_z: layout.inner_negative_z,
            inner_positive_z: layout.inner_positive_z,
        }),
        participants,
        error: None,
    });
}

pub(crate) fn emit_failed(request: TacticalFixtureRequest, error: &str) {
    emit(&FixtureRecord {
        schema_version: 3,
        status: "failed",
        family: "tactical",
        fixture_code: request.code,
        faction: if request.faction == CockpitFaction::Alliance {
            "alliance"
        } else {
            "empire"
        },
        proof_enabled: request.proof_enabled,
        tactical_lod: request.lod_fixture.label(),
        system: "",
        system_picture_id: 0,
        palette_resource_id: 0,
        attacker_ships: 0,
        defender_ships: 0,
        fighters: 0,
        source_layout: None,
        participants: Vec::new(),
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
        assert!(decode(0x10101).unwrap().proof_enabled);
        assert!(!decode(0x10102).unwrap().proof_enabled);
        assert_eq!(
            decode(0x10103).unwrap().lod_fixture,
            TacticalLodFixture::Close
        );
        assert_eq!(
            decode(0x10104).unwrap().lod_fixture,
            TacticalLodFixture::Medium
        );
        assert_eq!(
            decode(0x10105).unwrap().lod_fixture,
            TacticalLodFixture::Far
        );
        assert_eq!(
            decode(0x10106).unwrap().lod_fixture,
            TacticalLodFixture::Journey
        );
        assert_eq!(
            decode(0x10107).unwrap().lod_fixture,
            TacticalLodFixture::CameraJourney
        );
        assert!(decode(0x10100).is_none());
        assert!(decode(0x10108).is_none());
        assert!(decode(0x10301).is_none());
        assert!(decode(0x00101).is_none());
    }
}
