//! Test-only, versioned direct entry into the production tactical battle scene.

use rebellion_core::ids::{CapitalShipKey, FighterKey, SystemKey};
use rebellion_core::missions::MissionFaction;
use rebellion_core::world::{FighterEntry, Fleet, GameWorld, ShipInstance};
use rebellion_render::tactical_view::{
    tactical_character_content_resource, TacticalAttackTarget, WeaponKind,
};
#[cfg(feature = "interface-test-fixtures")]
use rebellion_render::TacticalLodView;
use rebellion_render::{
    CockpitFaction, CockpitState, MessageLog, TacticalState, TacticalTrenchRunOutcome,
};
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
const PRODUCTION_PARTICIPANTS_SCENARIO: u32 = 8;
const PRODUCTION_PARTICIPANTS_CONTROL_SCENARIO: u32 = 9;
const PRODUCTION_FIGHTER_DETAIL_JOURNEY_SCENARIO: u32 = 10;
const GROUP_PRESENTATION_SCENARIO: u32 = 11;
const EFFECT_PRESENTATION_SCENARIO: u32 = 12;
const PROJECTILE_FIELD_PRESENTATION_SCENARIO: u32 = 13;
const SELECTED_DAMAGE_PRESENTATION_SCENARIO: u32 = 14;
const SUBSYSTEM_FIELD_COMMAND_PRESENTATION_SCENARIO: u32 = 15;
const LIVE_SUBSYSTEM_DAMAGE_PRESENTATION_SCENARIO: u32 = 16;
const SUBSYSTEM_REPAIR_MOBILITY_PRESENTATION_SCENARIO: u32 = 17;
const MANEUVER_MOVEMENT_PRESENTATION_SCENARIO: u32 = 18;
const COMMAND_ASSIGNMENT_PRESENTATION_SCENARIO: u32 = 19;
const COMMAND_EXECUTION_PRESENTATION_SCENARIO: u32 = 20;
const COMMAND_PROGRESSION_PRESENTATION_SCENARIO: u32 = 21;
const ATTACK_TARGETING_PRESENTATION_SCENARIO: u32 = 22;
const ATTACK_TARGET_LIFECYCLE_PRESENTATION_SCENARIO: u32 = 23;
const DEATH_STAR_PRESENTATION_SCENARIO: u32 = 24;
const BATTLE_RESULTS_PRESENTATION_SCENARIO: u32 = 25;
const BATTLE_OPTIONS_PRESENTATION_SCENARIO: u32 = 26;
const BATTLE_OPTIONS_WITHDRAWAL_SCENARIO: u32 = 27;
const DEATH_STAR_LASER_JOURNEY_SCENARIO: u32 = 28;
const TRENCH_RUN_SUCCESS_SCENARIO: u32 = 29;
const TRENCH_RUN_FAILURE_SCENARIO: u32 = 30;
const SELECTED_NAVIGATION_PRESENTATION_SCENARIO: u32 = 31;
const NAVIGATION_CAMERA_PRESENTATION_SCENARIO: u32 = 32;
const EMPTY_SPACE_PRESENTATION_SCENARIO: u32 = 33;
const DETAIL_ESCORT_PRESENTATION_SCENARIO: u32 = 34;
const BATTLE_ALERT_PRESENTATION_SCENARIO: u32 = 35;
const TACTICAL_AUDIO_PRESENTATION_SCENARIO: u32 = 36;

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
    pub production_participants: bool,
    pub suppress_capital_fallback: bool,
    pub focus_player_fighter: bool,
    pub group_presentation: bool,
    pub effect_presentation: bool,
    pub projectile_field_presentation: bool,
    pub selected_damage_presentation: bool,
    pub selected_navigation_presentation: bool,
    pub navigation_camera_presentation: bool,
    pub empty_space_presentation: bool,
    pub detail_escort_presentation: bool,
    pub battle_alert_presentation: bool,
    pub tactical_audio_presentation: bool,
    pub subsystem_field_command_presentation: bool,
    pub live_subsystem_damage_presentation: bool,
    pub subsystem_repair_mobility_presentation: bool,
    pub maneuver_movement_presentation: bool,
    pub command_assignment_presentation: bool,
    pub command_execution_presentation: bool,
    pub command_progression_presentation: bool,
    pub attack_targeting_presentation: bool,
    pub attack_target_lifecycle_presentation: bool,
    pub death_star_presentation: bool,
    pub battle_results_presentation: bool,
    pub battle_options_presentation: bool,
    pub battle_options_withdrawal: bool,
    pub death_star_laser_journey: bool,
    pub trench_run_outcome: Option<TacticalTrenchRunOutcome>,
    lod_fixture: TacticalLodFixture,
}

fn decode(code: u32) -> Option<TacticalFixtureRequest> {
    if code >> 16 != TACTICAL_FAMILY {
        return None;
    }
    let scenario = code & 0xff;
    let (
        proof_enabled,
        production_participants,
        suppress_capital_fallback,
        focus_player_fighter,
        group_presentation,
        effect_presentation,
        projectile_field_presentation,
        selected_damage_presentation,
        lod_fixture,
    ) = match scenario {
        BATTLE_SCENARIO => (
            true,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            TacticalLodFixture::Default,
        ),
        BATTLE_WITHOUT_PROOF_SCENARIO => (
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            TacticalLodFixture::Default,
        ),
        LOD_CLOSE_SCENARIO => (
            true,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            TacticalLodFixture::Close,
        ),
        LOD_MEDIUM_SCENARIO => (
            true,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            TacticalLodFixture::Medium,
        ),
        LOD_FAR_SCENARIO => (
            true,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            TacticalLodFixture::Far,
        ),
        LOD_JOURNEY_SCENARIO => (
            true,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            TacticalLodFixture::Journey,
        ),
        CAMERA_JOURNEY_SCENARIO => (
            true,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            TacticalLodFixture::CameraJourney,
        ),
        PRODUCTION_PARTICIPANTS_SCENARIO => (
            false,
            true,
            false,
            false,
            false,
            false,
            false,
            false,
            TacticalLodFixture::Default,
        ),
        PRODUCTION_PARTICIPANTS_CONTROL_SCENARIO => (
            false,
            false,
            true,
            false,
            false,
            false,
            false,
            false,
            TacticalLodFixture::Default,
        ),
        PRODUCTION_FIGHTER_DETAIL_JOURNEY_SCENARIO => (
            false,
            true,
            false,
            true,
            false,
            false,
            false,
            false,
            TacticalLodFixture::Default,
        ),
        GROUP_PRESENTATION_SCENARIO => (
            false,
            true,
            false,
            false,
            true,
            false,
            false,
            false,
            TacticalLodFixture::Default,
        ),
        EFFECT_PRESENTATION_SCENARIO => (
            false,
            true,
            false,
            false,
            false,
            true,
            false,
            false,
            TacticalLodFixture::Default,
        ),
        PROJECTILE_FIELD_PRESENTATION_SCENARIO => (
            false,
            true,
            false,
            false,
            false,
            false,
            true,
            false,
            TacticalLodFixture::Default,
        ),
        SELECTED_DAMAGE_PRESENTATION_SCENARIO => (
            false,
            true,
            false,
            false,
            false,
            false,
            false,
            true,
            TacticalLodFixture::Default,
        ),
        SUBSYSTEM_FIELD_COMMAND_PRESENTATION_SCENARIO => (
            false,
            true,
            false,
            false,
            false,
            false,
            false,
            false,
            TacticalLodFixture::Default,
        ),
        LIVE_SUBSYSTEM_DAMAGE_PRESENTATION_SCENARIO => (
            false,
            true,
            false,
            false,
            false,
            false,
            false,
            false,
            TacticalLodFixture::Default,
        ),
        SUBSYSTEM_REPAIR_MOBILITY_PRESENTATION_SCENARIO
        | MANEUVER_MOVEMENT_PRESENTATION_SCENARIO
        | COMMAND_ASSIGNMENT_PRESENTATION_SCENARIO
        | COMMAND_EXECUTION_PRESENTATION_SCENARIO
        | COMMAND_PROGRESSION_PRESENTATION_SCENARIO
        | ATTACK_TARGETING_PRESENTATION_SCENARIO
        | ATTACK_TARGET_LIFECYCLE_PRESENTATION_SCENARIO
        | DEATH_STAR_PRESENTATION_SCENARIO
        | BATTLE_RESULTS_PRESENTATION_SCENARIO
        | BATTLE_OPTIONS_PRESENTATION_SCENARIO
        | BATTLE_OPTIONS_WITHDRAWAL_SCENARIO
        | DEATH_STAR_LASER_JOURNEY_SCENARIO
        | TRENCH_RUN_SUCCESS_SCENARIO
        | TRENCH_RUN_FAILURE_SCENARIO
        | SELECTED_NAVIGATION_PRESENTATION_SCENARIO
        | NAVIGATION_CAMERA_PRESENTATION_SCENARIO
        | EMPTY_SPACE_PRESENTATION_SCENARIO
        | DETAIL_ESCORT_PRESENTATION_SCENARIO
        | BATTLE_ALERT_PRESENTATION_SCENARIO
        | TACTICAL_AUDIO_PRESENTATION_SCENARIO => (
            false,
            true,
            false,
            false,
            false,
            false,
            false,
            false,
            TacticalLodFixture::Default,
        ),
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
        production_participants,
        suppress_capital_fallback,
        focus_player_fighter: focus_player_fighter
            || matches!(
                scenario,
                DEATH_STAR_PRESENTATION_SCENARIO
                    | DEATH_STAR_LASER_JOURNEY_SCENARIO
                    | TRENCH_RUN_SUCCESS_SCENARIO
                    | TRENCH_RUN_FAILURE_SCENARIO
            ),
        group_presentation,
        effect_presentation,
        projectile_field_presentation,
        selected_damage_presentation,
        selected_navigation_presentation: scenario == SELECTED_NAVIGATION_PRESENTATION_SCENARIO,
        navigation_camera_presentation: scenario == NAVIGATION_CAMERA_PRESENTATION_SCENARIO,
        empty_space_presentation: scenario == EMPTY_SPACE_PRESENTATION_SCENARIO,
        detail_escort_presentation: scenario == DETAIL_ESCORT_PRESENTATION_SCENARIO,
        battle_alert_presentation: scenario == BATTLE_ALERT_PRESENTATION_SCENARIO,
        tactical_audio_presentation: scenario == TACTICAL_AUDIO_PRESENTATION_SCENARIO,
        subsystem_field_command_presentation: scenario
            == SUBSYSTEM_FIELD_COMMAND_PRESENTATION_SCENARIO,
        live_subsystem_damage_presentation: scenario == LIVE_SUBSYSTEM_DAMAGE_PRESENTATION_SCENARIO,
        subsystem_repair_mobility_presentation: scenario
            == SUBSYSTEM_REPAIR_MOBILITY_PRESENTATION_SCENARIO,
        maneuver_movement_presentation: scenario == MANEUVER_MOVEMENT_PRESENTATION_SCENARIO,
        command_assignment_presentation: scenario == COMMAND_ASSIGNMENT_PRESENTATION_SCENARIO,
        command_execution_presentation: scenario == COMMAND_EXECUTION_PRESENTATION_SCENARIO,
        command_progression_presentation: scenario == COMMAND_PROGRESSION_PRESENTATION_SCENARIO,
        attack_targeting_presentation: scenario == ATTACK_TARGETING_PRESENTATION_SCENARIO,
        attack_target_lifecycle_presentation: scenario
            == ATTACK_TARGET_LIFECYCLE_PRESENTATION_SCENARIO,
        death_star_presentation: matches!(
            scenario,
            DEATH_STAR_PRESENTATION_SCENARIO
                | DEATH_STAR_LASER_JOURNEY_SCENARIO
                | TRENCH_RUN_SUCCESS_SCENARIO
                | TRENCH_RUN_FAILURE_SCENARIO
        ),
        battle_results_presentation: scenario == BATTLE_RESULTS_PRESENTATION_SCENARIO,
        battle_options_presentation: matches!(
            scenario,
            BATTLE_OPTIONS_PRESENTATION_SCENARIO | BATTLE_OPTIONS_WITHDRAWAL_SCENARIO
        ),
        battle_options_withdrawal: scenario == BATTLE_OPTIONS_WITHDRAWAL_SCENARIO,
        death_star_laser_journey: scenario == DEATH_STAR_LASER_JOURNEY_SCENARIO,
        trench_run_outcome: match scenario {
            TRENCH_RUN_SUCCESS_SCENARIO => Some(TacticalTrenchRunOutcome::Success),
            TRENCH_RUN_FAILURE_SCENARIO => Some(TacticalTrenchRunOutcome::Failure),
            _ => None,
        },
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
        .find(|(_, class)| {
            class.is_alliance
                && !class.is_empire
                && class.hull > 0
                && (!request.battle_options_presentation || class.hyperdrive > 0)
                && (!(request.command_execution_presentation
                    || request.command_progression_presentation
                    || request.attack_targeting_presentation
                    || request.attack_target_lifecycle_presentation)
                    || class.fighter_capacity > 0)
        })
        .map(|(key, class)| (key, class.hull))
        .ok_or("no Alliance capital-ship class")?;
    let empire_ship = world
        .capital_ship_classes
        .iter()
        .find(|(_, class)| {
            class.is_empire
                && !class.is_alliance
                && class.hull > 0
                && (!request.battle_options_presentation || class.hyperdrive > 0)
                && (!(request.command_execution_presentation
                    || request.command_progression_presentation
                    || request.attack_targeting_presentation
                    || request.attack_target_lifecycle_presentation)
                    || class.fighter_capacity > 0)
        })
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

    let player_is_alliance = request.faction == CockpitFaction::Alliance;
    let make_fleet = |is_alliance: bool,
                      ship: CapitalShipKey,
                      hull: u32,
                      fighter: Option<FighterKey>| {
        let expanded = request.projectile_field_presentation
            || request.detail_escort_presentation
            || request.subsystem_field_command_presentation
            || request.live_subsystem_damage_presentation
            || request.subsystem_repair_mobility_presentation
            || request.maneuver_movement_presentation
            || request.command_execution_presentation
            || request.command_progression_presentation
            || request.attack_targeting_presentation
            || request.attack_target_lifecycle_presentation
            || request.battle_results_presentation
            || request.selected_navigation_presentation
            || request.navigation_camera_presentation
            || ((request.group_presentation || request.effect_presentation)
                && is_alliance == player_is_alliance);
        let ship_count = if request.battle_options_withdrawal {
            2
        } else if request.projectile_field_presentation
            || request.detail_escort_presentation
            || request.subsystem_field_command_presentation
            || request.live_subsystem_damage_presentation
            || request.subsystem_repair_mobility_presentation
            || request.maneuver_movement_presentation
            || request.command_execution_presentation
            || request.command_progression_presentation
            || request.attack_targeting_presentation
            || request.attack_target_lifecycle_presentation
            || request.battle_results_presentation
            || request.selected_navigation_presentation
            || request.navigation_camera_presentation
        {
            3
        } else if request.effect_presentation && expanded {
            6
        } else if expanded {
            8
        } else {
            1
        };
        let fighter_count = if request.battle_results_presentation {
            3
        } else if request.command_assignment_presentation
            || request.command_progression_presentation
            || request.attack_targeting_presentation
            || request.attack_target_lifecycle_presentation
        {
            2
        } else if request.group_presentation && expanded {
            4
        } else {
            1
        };
        Fleet {
            location: system,
            capital_ships: (0..ship_count)
                .map(|_| {
                    ShipInstance::new(ship, i32::try_from(hull).unwrap_or(i32::MAX), is_alliance)
                })
                .collect(),
            fighters: fighter
                .map(|class| {
                    (0..fighter_count)
                        // `FighterEntry::count` is the number of strategic
                        // squadrons. `FIGHTSD.squadron_size` supplies the
                        // twelve craft inside each tactical object.
                        .map(|_| FighterEntry { class, count: 1 })
                        .collect()
                })
                .unwrap_or_default(),
            characters: vec![],
            is_alliance,
            has_death_star: request.death_star_presentation && !is_alliance,
        }
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
    if request.detail_escort_presentation {
        let player_fleet = if player_is_alliance {
            attacker
        } else {
            defender
        };
        let character = world
            .characters
            .iter()
            .find(|(_, character)| {
                !character.is_killed
                    && character.is_major
                    && character.current_fleet.is_none()
                    && tactical_character_content_resource(character.dat_id, character.is_major)
                        .is_some()
                    && if player_is_alliance {
                        character.is_alliance
                    } else {
                        character.is_empire
                    }
            })
            .map(|(key, _)| key)
            .ok_or("no unassigned faction character for tactical contents fixture")?;
        world.fleets[player_fleet].characters.push(character);
        world.characters[character].current_fleet = Some(player_fleet);
        world.characters[character].current_system = None;
    }
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
    if !request.battle_alert_presentation {
        tactical.dismiss_battle_alert_for_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.focus_player_fighter {
        tactical.focus_player_fighter_for_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.group_presentation {
        tactical.configure_group_presentation_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.effect_presentation {
        tactical.configure_effect_presentation_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.projectile_field_presentation {
        tactical.configure_projectile_field_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.selected_damage_presentation {
        tactical.configure_selected_damage_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.selected_navigation_presentation {
        tactical.configure_selected_navigation_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.navigation_camera_presentation {
        tactical.configure_selected_navigation_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.subsystem_field_command_presentation {
        tactical.configure_subsystem_field_command_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.live_subsystem_damage_presentation {
        tactical.configure_live_subsystem_damage_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.subsystem_repair_mobility_presentation {
        tactical.configure_subsystem_repair_mobility_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.maneuver_movement_presentation {
        tactical.configure_maneuver_movement_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.command_assignment_presentation {
        tactical.configure_command_assignment_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.command_execution_presentation {
        tactical.configure_command_execution_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.command_progression_presentation {
        tactical.configure_command_progression_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.attack_targeting_presentation {
        tactical.configure_attack_targeting_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.attack_target_lifecycle_presentation {
        tactical.configure_attack_target_lifecycle_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.death_star_presentation {
        tactical.configure_death_star_presentation_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.death_star_laser_journey {
        tactical.configure_death_star_laser_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if let Some(outcome) = request.trench_run_outcome {
        tactical.configure_trench_run_outcome_fixture(outcome);
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.battle_results_presentation {
        tactical.configure_battle_results_presentation_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.battle_options_withdrawal {
        tactical.configure_withdrawal_warning_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.empty_space_presentation {
        tactical.configure_empty_space_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.detail_escort_presentation {
        tactical.configure_detail_escort_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.tactical_audio_presentation {
        tactical.configure_tactical_audio_fixture();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if !request.production_participants {
        tactical.disable_original_participant_rendering();
    }
    #[cfg(feature = "interface-test-fixtures")]
    if request.suppress_capital_fallback {
        tactical.suppress_mapped_capital_fallback();
    }
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
    production_participants: bool,
    suppress_capital_fallback: bool,
    focus_player_fighter: bool,
    group_presentation: bool,
    effect_presentation: bool,
    projectile_field_presentation: bool,
    selected_damage_presentation: bool,
    selected_navigation_presentation: bool,
    navigation_camera_presentation: bool,
    empty_space_presentation: bool,
    detail_escort_presentation: bool,
    battle_alert_presentation: bool,
    tactical_audio_presentation: bool,
    battle_alert_open: bool,
    tactical_music_mdata_id: u32,
    tactical_weapon_audio_wave_first: u32,
    tactical_weapon_audio_wave_last: u32,
    tactical_weapon_audio_variant_count: u32,
    tactical_voice_variant_count: u32,
    subsystem_field_command_presentation: bool,
    live_subsystem_damage_presentation: bool,
    subsystem_repair_mobility_presentation: bool,
    maneuver_movement_presentation: bool,
    command_assignment_presentation: bool,
    command_execution_presentation: bool,
    command_progression_presentation: bool,
    attack_targeting_presentation: bool,
    attack_target_lifecycle_presentation: bool,
    death_star_presentation: bool,
    battle_results_presentation: bool,
    battle_options_presentation: bool,
    battle_options_withdrawal: bool,
    death_star_laser_journey: bool,
    trench_run_outcome: Option<&'a str>,
    tactical_lod: &'a str,
    system: &'a str,
    system_picture_id: u8,
    planet_resource_id: u32,
    palette_resource_id: u32,
    attacker_ships: usize,
    defender_ships: usize,
    fighters: usize,
    source_layout: Option<FixtureSourceLayout>,
    participants: Vec<FixtureParticipant<'a>>,
    effects: Vec<FixtureEffect>,
    projectiles: Vec<FixtureProjectile>,
    fields: Vec<FixtureField>,
    subsystem_repairs: Vec<FixtureSubsystemRepair>,
    selected_ship: Option<FixtureSelectedShip<'a>>,
    death_star: Option<FixtureDeathStar>,
    error: Option<&'a str>,
}

#[derive(Serialize)]
struct FixtureDeathStar {
    faction: &'static str,
    is_attacker: bool,
    tactical_ordinal: u8,
    resource_id: u32,
    opposing_resource_id: u32,
    operational: bool,
    hull: f32,
    laser_charge: f32,
    destroyed: bool,
    action_committed: bool,
    source_position: [f32; 3],
}

#[derive(Serialize)]
struct FixtureSelectedShip<'a> {
    name: &'a str,
    faction: &'static str,
    class_dat_id: u32,
    hud_resource: Option<u32>,
    hull_current: i32,
    hull_max: i32,
    shield_current: i32,
    shield_max: i32,
    subsystem_percentages: [u8; 5],
    subsystem_resources: [u32; 5],
    subsystem_hit_limits: [u8; 5],
    subsystem_damage_hits: [u8; 5],
    base_engine_power: f32,
    engine_mode_bonus: f32,
    active_tractor_power: f32,
    effective_engine_power: f32,
    maneuverability: u32,
    maneuver_state_value: Option<f32>,
    movement_alignment: f32,
    source_position: [f32; 3],
    source_forward: [f32; 3],
    source_desired_forward: [f32; 3],
    source_velocity: [f32; 3],
    source_waypoint: Option<[f32; 3]>,
    order_code: u8,
    tactic_code: u8,
    damage_control: u8,
    contents: Vec<FixtureShipContent<'a>>,
}

#[derive(Serialize)]
struct FixtureShipContent<'a> {
    label: &'a str,
    resource_id: u32,
}

#[derive(Serialize)]
struct FixtureSubsystemRepair {
    ship: usize,
    kind: &'static str,
    hits_before: u8,
    hits_after: u8,
}

#[derive(Serialize)]
struct FixtureEffect {
    target: usize,
    resource_base: u32,
    resource_id: u32,
    frame: u8,
    frame_count: u8,
    source_size: [u16; 2],
}

#[derive(Serialize)]
struct FixtureProjectile {
    source: usize,
    target: usize,
    kind: &'static str,
    longitudinal_scale: f32,
    shape_variant: u8,
    color_selector: u8,
    elapsed: f32,
    duration: f32,
    progress: f32,
}

#[derive(Serialize)]
struct FixtureField {
    target: usize,
    resource_id: u32,
    frame: u8,
    tractor_sources: u8,
    gravity_sources: u8,
    tractor_source_ids: Vec<usize>,
    gravity_source_ids: Vec<usize>,
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
    tactical_ordinal: Option<u8>,
    resource_base: Option<u32>,
    opposing_resource_base: Option<u32>,
    initial_close_resource: Option<u32>,
    initial_far_resource: Option<u32>,
    initial_indicator_resource: Option<u32>,
    task_force: Option<u8>,
    fighter_group: Option<u8>,
    active_close_resource: Option<u32>,
    active_far_resource: Option<u32>,
    active_indicator_resource: Option<u32>,
    active: bool,
    retreating: Option<bool>,
    source_position: [f32; 3],
    source_forward: Option<[f32; 3]>,
    source_desired_forward: Option<[f32; 3]>,
    source_waypoint: Option<[f32; 3]>,
    order_code: u8,
    tactic_code: u8,
    recovery_state_code: Option<u8>,
    recovery_target: Option<usize>,
    attack_target_kind: Option<&'static str>,
    attack_target_index: Option<usize>,
}

fn fixture_attack_target(
    target: Option<TacticalAttackTarget>,
) -> (Option<&'static str>, Option<usize>) {
    match target {
        Some(TacticalAttackTarget::CapitalShip(index)) => (Some("capital-ship"), Some(index)),
        Some(TacticalAttackTarget::FighterGroup(index)) => (Some("fighter-group"), Some(index)),
        None => (None, None),
    }
}

pub(crate) fn emit_ready(request: TacticalFixtureRequest, tactical: &TacticalState) {
    let Some(session) = tactical.session.as_ref() else {
        emit_failed(request, "battle session missing after fixture entry");
        return;
    };
    let layout = session.source_layout;
    let mut participants = Vec::with_capacity(session.ships.len() + session.fighters.len());
    participants.extend(session.ships.iter().map(|ship| {
        let (attack_target_kind, attack_target_index) = fixture_attack_target(ship.attack_target);
        let tactical_ordinal = ship
            .tactical_resource
            .map(|resource| resource.tactical_ordinal)
            .or_else(|| {
                ship.death_star_resource
                    .map(|resource| resource.tactical_ordinal)
            });
        let resource_base = ship
            .tactical_resource
            .map(|resource| resource.mesh_resource_base)
            .or_else(|| {
                ship.death_star_resource
                    .map(|resource| resource.resource_base_without_flag)
            });
        FixtureParticipant {
            kind: "capital-ship",
            name: &ship.name,
            faction: if ship.identity.is_alliance {
                "alliance"
            } else {
                "empire"
            },
            class_dat_id: ship.identity.class_dat_id.index(),
            fleet_roster_index: ship.identity.fleet_roster_index,
            tactical_ordinal,
            resource_base,
            opposing_resource_base: ship
                .death_star_resource
                .map(|resource| resource.resource_base_with_flag),
            initial_close_resource: None,
            initial_far_resource: None,
            initial_indicator_resource: None,
            task_force: Some(ship.task_force),
            fighter_group: None,
            active_close_resource: None,
            active_far_resource: None,
            active_indicator_resource: None,
            active: ship.alive,
            retreating: Some(ship.retreating),
            source_position: [
                ship.source_position.x,
                ship.source_position.y,
                ship.source_position.z,
            ],
            source_forward: Some([
                ship.source_forward.x,
                ship.source_forward.y,
                ship.source_forward.z,
            ]),
            source_desired_forward: Some([
                ship.source_desired_forward.x,
                ship.source_desired_forward.y,
                ship.source_desired_forward.z,
            ]),
            source_waypoint: ship
                .source_waypoint
                .map(|waypoint| [waypoint.x, waypoint.y, waypoint.z]),
            order_code: ship.order.source_code(),
            tactic_code: ship.tactic.source_code(),
            recovery_state_code: None,
            recovery_target: None,
            attack_target_kind,
            attack_target_index,
        }
    }));
    participants.extend(session.fighters.iter().map(|fighter| {
        let (attack_target_kind, attack_target_index) =
            fixture_attack_target(fighter.attack_target);
        let player_side = fighter.is_attacker == session.player_is_attacker;
        FixtureParticipant {
            kind: "fighter-group",
            name: &fighter.name,
            faction: if fighter.identity.is_alliance {
                "alliance"
            } else {
                "empire"
            },
            class_dat_id: fighter.identity.class_dat_id.index(),
            fleet_roster_index: fighter.identity.fleet_roster_index,
            tactical_ordinal: fighter
                .tactical_resource
                .map(|resource| resource.tactical_ordinal),
            resource_base: fighter
                .tactical_resource
                .map(|resource| resource.first_side_resource_base),
            opposing_resource_base: fighter
                .tactical_resource
                .map(|resource| resource.opposing_side_resource_base),
            initial_close_resource: fighter
                .tactical_resource
                .map(|resource| resource.initial_close_resource(player_side)),
            initial_far_resource: fighter
                .tactical_resource
                .map(|resource| resource.initial_far_resource(player_side)),
            initial_indicator_resource: fighter
                .tactical_resource
                .map(|resource| resource.initial_indicator_resource(player_side)),
            task_force: None,
            fighter_group: Some(fighter.fighter_group),
            active_close_resource: fighter.tactical_resource.map(|resource| {
                resource.grouped_close_resource(player_side, fighter.fighter_group)
            }),
            active_far_resource: fighter
                .tactical_resource
                .map(|resource| resource.grouped_far_resource(player_side, fighter.fighter_group)),
            active_indicator_resource: fighter.tactical_resource.map(|resource| {
                resource.grouped_indicator_resource(player_side, fighter.fighter_group)
            }),
            active: fighter.alive,
            retreating: None,
            source_position: [
                fighter.source_position.x,
                fighter.source_position.y,
                fighter.source_position.z,
            ],
            source_forward: None,
            source_desired_forward: None,
            source_waypoint: None,
            order_code: fighter.order.source_code(),
            tactic_code: fighter.tactic.source_code(),
            recovery_state_code: Some(fighter.recovery_state.source_code()),
            recovery_target: fighter.recovery_target,
            attack_target_kind,
            attack_target_index,
        }
    }));
    let effects = session
        .impact_effects
        .iter()
        .map(|effect| {
            let (width, height) = effect.sequence.source_size();
            FixtureEffect {
                target: effect.target,
                resource_base: effect.sequence.resource_base(),
                resource_id: effect.sequence.resource_base() + u32::from(effect.frame),
                frame: effect.frame,
                frame_count: effect.sequence.frame_count(),
                source_size: [width, height],
            }
        })
        .collect();
    let projectiles = session
        .weapon_effects
        .iter()
        .map(|effect| FixtureProjectile {
            source: effect.source,
            target: effect.target,
            kind: match effect.kind {
                WeaponKind::LaserCannon => "laser",
                WeaponKind::Turbolaser => "turbolaser",
                WeaponKind::IonCannon => "ion",
                WeaponKind::FighterAttack => "torpedo",
            },
            longitudinal_scale: effect.longitudinal_scale,
            shape_variant: effect.shape_variant,
            color_selector: effect.color_selector,
            elapsed: effect.elapsed,
            duration: effect.duration,
            progress: effect.progress(),
        })
        .collect();
    let fields = session
        .field_effects
        .iter()
        .filter_map(|effect| {
            let (tractor_sources, gravity_sources) = effect.source_counts();
            let (tractor_source_ids, gravity_source_ids) = effect.source_ids();
            Some(FixtureField {
                target: effect.target,
                resource_id: effect.resource_id()?,
                frame: effect.frame,
                tractor_sources,
                gravity_sources,
                tractor_source_ids: tractor_source_ids.to_vec(),
                gravity_source_ids: gravity_source_ids.to_vec(),
            })
        })
        .collect();
    let subsystem_repairs = session
        .subsystem_repairs
        .iter()
        .map(|repair| FixtureSubsystemRepair {
            ship: repair.ship,
            kind: repair.kind.label(),
            hits_before: repair.hits_before,
            hits_after: repair.hits_after,
        })
        .collect();
    let selected_ship = session.selected_ship.and_then(|index| {
        let ship = session.ships.get(index)?;
        let mobility = session.subsystem_mobility(index)?;
        Some(FixtureSelectedShip {
            name: &ship.name,
            faction: if ship.identity.is_alliance {
                "alliance"
            } else {
                "empire"
            },
            class_dat_id: ship.identity.class_dat_id.index(),
            hud_resource: ship
                .tactical_resource
                .map(|resource| resource.hud_resource()),
            hull_current: ship.hull_current,
            hull_max: ship.hull_max,
            shield_current: ship.shield,
            shield_max: ship.shield_max,
            subsystem_percentages: ship.subsystem_condition.percentages(),
            subsystem_resources: ship.subsystem_condition.resource_ids(),
            subsystem_hit_limits: ship.subsystem_capacity.hit_limits(),
            subsystem_damage_hits: ship.subsystem_damage.hits(),
            base_engine_power: mobility.base_engine_power,
            engine_mode_bonus: mobility.engine_mode_bonus,
            active_tractor_power: mobility.active_tractor_power,
            effective_engine_power: mobility.effective_engine_power,
            maneuverability: ship.maneuverability,
            maneuver_state_value: ship.maneuver_state_value,
            movement_alignment: session.movement_alignment(index).unwrap_or(0.0),
            source_position: [
                ship.source_position.x,
                ship.source_position.y,
                ship.source_position.z,
            ],
            source_forward: [
                ship.source_forward.x,
                ship.source_forward.y,
                ship.source_forward.z,
            ],
            source_desired_forward: [
                ship.source_desired_forward.x,
                ship.source_desired_forward.y,
                ship.source_desired_forward.z,
            ],
            source_velocity: [
                ship.source_velocity.x,
                ship.source_velocity.y,
                ship.source_velocity.z,
            ],
            source_waypoint: ship
                .source_waypoint
                .map(|waypoint| [waypoint.x, waypoint.y, waypoint.z]),
            order_code: ship.order.source_code(),
            tactic_code: ship.tactic.source_code(),
            damage_control: ship.damage_control,
            contents: ship
                .contents
                .iter()
                .map(|content| FixtureShipContent {
                    label: content.label.as_str(),
                    resource_id: content.resource_id,
                })
                .collect(),
        })
    });
    let death_star = session.death_star.map(|death_star| FixtureDeathStar {
        faction: if death_star.is_alliance {
            "alliance"
        } else {
            "empire"
        },
        is_attacker: death_star.is_attacker,
        tactical_ordinal: death_star.resource.tactical_ordinal,
        resource_id: death_star.resource_id(),
        opposing_resource_id: death_star.resource.resource_base_with_flag,
        operational: death_star.operational(),
        hull: death_star.hull,
        laser_charge: death_star.laser_charge,
        destroyed: death_star.destroyed,
        action_committed: death_star.action_committed,
        source_position: [
            death_star.source_position.x,
            death_star.source_position.y,
            death_star.source_position.z,
        ],
    });
    emit(&FixtureRecord {
        schema_version: 35,
        status: "battle-ready",
        family: "tactical",
        fixture_code: request.code,
        faction: if request.faction == CockpitFaction::Alliance {
            "alliance"
        } else {
            "empire"
        },
        proof_enabled: request.proof_enabled,
        production_participants: request.production_participants,
        suppress_capital_fallback: request.suppress_capital_fallback,
        focus_player_fighter: request.focus_player_fighter,
        group_presentation: request.group_presentation,
        effect_presentation: request.effect_presentation,
        projectile_field_presentation: request.projectile_field_presentation,
        selected_damage_presentation: request.selected_damage_presentation,
        selected_navigation_presentation: request.selected_navigation_presentation,
        navigation_camera_presentation: request.navigation_camera_presentation,
        empty_space_presentation: request.empty_space_presentation,
        detail_escort_presentation: request.detail_escort_presentation,
        battle_alert_presentation: request.battle_alert_presentation,
        tactical_audio_presentation: request.tactical_audio_presentation,
        battle_alert_open: tactical.battle_alert_open(),
        tactical_music_mdata_id: 307,
        tactical_weapon_audio_wave_first: 13_033,
        tactical_weapon_audio_wave_last: 13_054,
        tactical_weapon_audio_variant_count: 22,
        tactical_voice_variant_count: 285,
        subsystem_field_command_presentation: request.subsystem_field_command_presentation,
        live_subsystem_damage_presentation: request.live_subsystem_damage_presentation,
        subsystem_repair_mobility_presentation: request.subsystem_repair_mobility_presentation,
        maneuver_movement_presentation: request.maneuver_movement_presentation,
        command_assignment_presentation: request.command_assignment_presentation,
        command_execution_presentation: request.command_execution_presentation,
        command_progression_presentation: request.command_progression_presentation,
        attack_targeting_presentation: request.attack_targeting_presentation,
        attack_target_lifecycle_presentation: request.attack_target_lifecycle_presentation,
        death_star_presentation: request.death_star_presentation,
        battle_results_presentation: request.battle_results_presentation,
        battle_options_presentation: request.battle_options_presentation,
        battle_options_withdrawal: request.battle_options_withdrawal,
        death_star_laser_journey: request.death_star_laser_journey,
        trench_run_outcome: session.trench_run_outcome.map(|outcome| match outcome {
            TacticalTrenchRunOutcome::Success => "success",
            TacticalTrenchRunOutcome::Failure => "failure",
        }),
        tactical_lod: request.lod_fixture.label(),
        system: &session.system_name,
        system_picture_id: session.system_picture_id,
        planet_resource_id: 5500 + u32::from(session.system_picture_id),
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
        effects,
        projectiles,
        fields,
        subsystem_repairs,
        selected_ship,
        death_star,
        error: None,
    });
}

pub(crate) fn emit_failed(request: TacticalFixtureRequest, error: &str) {
    emit(&FixtureRecord {
        schema_version: 35,
        status: "failed",
        family: "tactical",
        fixture_code: request.code,
        faction: if request.faction == CockpitFaction::Alliance {
            "alliance"
        } else {
            "empire"
        },
        proof_enabled: request.proof_enabled,
        production_participants: request.production_participants,
        suppress_capital_fallback: request.suppress_capital_fallback,
        focus_player_fighter: request.focus_player_fighter,
        group_presentation: request.group_presentation,
        effect_presentation: request.effect_presentation,
        projectile_field_presentation: request.projectile_field_presentation,
        selected_damage_presentation: request.selected_damage_presentation,
        selected_navigation_presentation: request.selected_navigation_presentation,
        navigation_camera_presentation: request.navigation_camera_presentation,
        empty_space_presentation: request.empty_space_presentation,
        detail_escort_presentation: request.detail_escort_presentation,
        battle_alert_presentation: request.battle_alert_presentation,
        tactical_audio_presentation: request.tactical_audio_presentation,
        battle_alert_open: false,
        tactical_music_mdata_id: 307,
        tactical_weapon_audio_wave_first: 13_033,
        tactical_weapon_audio_wave_last: 13_054,
        tactical_weapon_audio_variant_count: 22,
        tactical_voice_variant_count: 285,
        subsystem_field_command_presentation: request.subsystem_field_command_presentation,
        live_subsystem_damage_presentation: request.live_subsystem_damage_presentation,
        subsystem_repair_mobility_presentation: request.subsystem_repair_mobility_presentation,
        maneuver_movement_presentation: request.maneuver_movement_presentation,
        command_assignment_presentation: request.command_assignment_presentation,
        command_execution_presentation: request.command_execution_presentation,
        command_progression_presentation: request.command_progression_presentation,
        attack_targeting_presentation: request.attack_targeting_presentation,
        attack_target_lifecycle_presentation: request.attack_target_lifecycle_presentation,
        death_star_presentation: request.death_star_presentation,
        battle_results_presentation: request.battle_results_presentation,
        battle_options_presentation: request.battle_options_presentation,
        battle_options_withdrawal: request.battle_options_withdrawal,
        death_star_laser_journey: request.death_star_laser_journey,
        trench_run_outcome: None,
        tactical_lod: request.lod_fixture.label(),
        system: "",
        system_picture_id: 0,
        planet_resource_id: 0,
        palette_resource_id: 0,
        attacker_ships: 0,
        defender_ships: 0,
        fighters: 0,
        source_layout: None,
        participants: Vec::new(),
        effects: Vec::new(),
        projectiles: Vec::new(),
        fields: Vec::new(),
        subsystem_repairs: Vec::new(),
        selected_ship: None,
        death_star: None,
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
        assert!(decode(0x10108).unwrap().production_participants);
        assert!(decode(0x10109).unwrap().suppress_capital_fallback);
        assert!(decode(0x1010a).unwrap().production_participants);
        assert!(decode(0x1010b).unwrap().group_presentation);
        assert!(decode(0x1010c).unwrap().effect_presentation);
        assert!(decode(0x1010d).unwrap().projectile_field_presentation);
        assert!(decode(0x1010e).unwrap().selected_damage_presentation);
        assert!(
            decode(0x1010f)
                .unwrap()
                .subsystem_field_command_presentation
        );
        assert!(decode(0x10110).unwrap().live_subsystem_damage_presentation);
        assert!(
            decode(0x10111)
                .unwrap()
                .subsystem_repair_mobility_presentation
        );
        assert!(decode(0x10112).unwrap().maneuver_movement_presentation);
        assert!(decode(0x10113).unwrap().command_assignment_presentation);
        assert!(decode(0x10114).unwrap().command_execution_presentation);
        assert!(decode(0x10115).unwrap().command_progression_presentation);
        assert!(decode(0x10116).unwrap().attack_targeting_presentation);
        assert!(
            decode(0x10117)
                .unwrap()
                .attack_target_lifecycle_presentation
        );
        assert!(decode(0x10118).unwrap().death_star_presentation);
        assert!(decode(0x10118).unwrap().focus_player_fighter);
        assert!(decode(0x10119).unwrap().battle_results_presentation);
        assert!(decode(0x1011a).unwrap().battle_options_presentation);
        assert!(!decode(0x1011a).unwrap().battle_options_withdrawal);
        assert!(decode(0x1011b).unwrap().battle_options_presentation);
        assert!(decode(0x1011b).unwrap().battle_options_withdrawal);
        assert!(decode(0x1011c).unwrap().death_star_presentation);
        assert!(decode(0x1011c).unwrap().death_star_laser_journey);
        assert_eq!(
            decode(0x1011d).unwrap().trench_run_outcome,
            Some(TacticalTrenchRunOutcome::Success)
        );
        assert_eq!(
            decode(0x1011e).unwrap().trench_run_outcome,
            Some(TacticalTrenchRunOutcome::Failure)
        );
        assert!(decode(0x1011f).unwrap().selected_navigation_presentation);
        assert!(decode(0x10120).unwrap().navigation_camera_presentation);
        assert!(decode(0x10121).unwrap().empty_space_presentation);
        assert!(decode(0x10122).unwrap().detail_escort_presentation);
        assert!(decode(0x10123).unwrap().battle_alert_presentation);
        assert!(decode(0x10124).unwrap().tactical_audio_presentation);
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
        assert!(decode(0x10125).is_none());
        assert!(decode(0x10301).is_none());
        assert!(decode(0x00101).is_none());
    }
}
