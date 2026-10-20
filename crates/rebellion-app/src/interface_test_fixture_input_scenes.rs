//! Fixture-only failure and scene controls for windowing journeys D-E.
//!
//! Normal interaction still arrives through browser pointer and keyboard input.
//! This bridge exists only for states that production cannot deliberately
//! manufacture: renderer failure, a deterministic tactical transition, and a
//! campaign reset with stale queued work.

use rebellion_core::combat::CombatForce;
use rebellion_core::world::{Fleet, GameWorld, ShipInstance};
use rebellion_render::fleet_window::FleetWindowState;
use rebellion_render::{MessageLog, TacticalState};
use rebellion_windowing::{
    ContentReport, ContentStatus, ReconcileReport, RoutingSnapshot, WindowId, WindowKind,
};
use serde_json::{json, Value};

use crate::tactical_flow::{self, BattleEntry};
use crate::GameMode;

use super::{FixtureRequest, Scenario};

#[cfg(target_arch = "wasm32")]
extern "C" {
    fn open_rebellion_windowing_fixture_command() -> u32;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub(crate) enum InputScenesCommand {
    ModalUnpaintable = 1,
    RecoverContent = 2,
    ModelessUnpaintable = 3,
    ModelessAbsent = 4,
    EnterTacticalAlert = 5,
    EnterTacticalCombat = 6,
    EnterTacticalResults = 7,
    ReturnToGalaxy = 8,
    QueueStaleAction = 9,
    ResetSession = 10,
    CloseCaptureOwner = 11,
    MinimizeCaptureOwner = 12,
    CancelCaptureForFocusLoss = 13,
    ReplaceActiveMenu = 14,
}

impl InputScenesCommand {
    pub(crate) const fn decode(value: u32) -> Option<Self> {
        Some(match value {
            1 => Self::ModalUnpaintable,
            2 => Self::RecoverContent,
            3 => Self::ModelessUnpaintable,
            4 => Self::ModelessAbsent,
            5 => Self::EnterTacticalAlert,
            6 => Self::EnterTacticalCombat,
            7 => Self::EnterTacticalResults,
            8 => Self::ReturnToGalaxy,
            9 => Self::QueueStaleAction,
            10 => Self::ResetSession,
            11 => Self::CloseCaptureOwner,
            12 => Self::MinimizeCaptureOwner,
            13 => Self::CancelCaptureForFocusLoss,
            14 => Self::ReplaceActiveMenu,
            _ => return None,
        })
    }

    const fn label(self) -> &'static str {
        match self {
            Self::ModalUnpaintable => "inject-modal-unpaintable",
            Self::RecoverContent => "recover-content",
            Self::ModelessUnpaintable => "inject-modeless-unpaintable",
            Self::ModelessAbsent => "inject-modeless-absent",
            Self::EnterTacticalAlert => "inject-tactical-alert",
            Self::EnterTacticalCombat => "inject-tactical-combat",
            Self::EnterTacticalResults => "inject-tactical-results",
            Self::ReturnToGalaxy => "inject-galaxy-return",
            Self::QueueStaleAction => "inject-stale-action",
            Self::ResetSession => "inject-session-reset",
            Self::CloseCaptureOwner => "inject-close-capture-owner",
            Self::MinimizeCaptureOwner => "inject-minimize-capture-owner",
            Self::CancelCaptureForFocusLoss => "inject-focus-loss",
            Self::ReplaceActiveMenu => "inject-menu-replacement",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FailureInjection {
    ModalUnpaintable,
    ModelessUnpaintable,
    ModelessAbsent,
}

impl FailureInjection {
    const fn status(self) -> ContentStatus {
        match self {
            Self::ModalUnpaintable | Self::ModelessUnpaintable => ContentStatus::Unpaintable,
            Self::ModelessAbsent => ContentStatus::Absent,
        }
    }
}

#[derive(Debug, Default)]
pub struct InputScenesFixture {
    command_sequence: u64,
    observation_sequence: u64,
    last_command: Option<InputScenesCommand>,
    failure: Option<FailureInjection>,
    failure_target: Option<WindowId>,
    last_state: Option<Value>,
    removal_count: u64,
    reset_count: u64,
    queued_target: Option<WindowId>,
    dormant_galaxy_modal: Option<WindowId>,
    stale_action_discarded: bool,
    tactical_inputs: u64,
    tactical_action: Option<&'static str>,
}

impl InputScenesFixture {
    #[cfg(target_arch = "wasm32")]
    pub fn take_command(&mut self, request: FixtureRequest) -> Option<InputScenesCommand> {
        if request.scenario != Scenario::WindowingContract {
            return None;
        }
        let command =
            InputScenesCommand::decode(unsafe { open_rebellion_windowing_fixture_command() })?;
        self.command_sequence = self.command_sequence.saturating_add(1);
        self.last_command = Some(command);
        macroquad::logging::info!(
            "[windowing-fixture-command] sequence={} command={}",
            self.command_sequence,
            command.label()
        );
        Some(command)
    }

    pub fn apply_failure(
        &mut self,
        command: InputScenesCommand,
        snapshot: &RoutingSnapshot,
    ) -> bool {
        let injection = match command {
            InputScenesCommand::ModalUnpaintable => FailureInjection::ModalUnpaintable,
            InputScenesCommand::ModelessUnpaintable => FailureInjection::ModelessUnpaintable,
            InputScenesCommand::ModelessAbsent => FailureInjection::ModelessAbsent,
            InputScenesCommand::RecoverContent => {
                self.failure = None;
                self.failure_target = None;
                return true;
            }
            _ => return false,
        };
        self.failure_target = match injection {
            FailureInjection::ModalUnpaintable => snapshot.active_modal(),
            FailureInjection::ModelessUnpaintable | FailureInjection::ModelessAbsent => snapshot
                .paint_order()
                .iter()
                .rev()
                .copied()
                .find(|id| snapshot.key(*id).is_some_and(|key| !is_modal(key.kind))),
        };
        self.failure = self.failure_target.map(|_| injection);
        true
    }

    pub fn apply_scene_command(
        &mut self,
        command: InputScenesCommand,
        request: FixtureRequest,
        world: &mut GameWorld,
        tactical: &mut TacticalState,
        messages: &mut MessageLog,
        game_mode: &mut GameMode,
    ) -> Result<bool, String> {
        match command {
            InputScenesCommand::EnterTacticalAlert => {
                enter_tactical_alert(request, world, tactical, messages, game_mode)?;
                Ok(true)
            }
            InputScenesCommand::EnterTacticalCombat => {
                tactical.dismiss_battle_alert_for_fixture();
                Ok(true)
            }
            InputScenesCommand::EnterTacticalResults => {
                tactical.dismiss_battle_alert_for_fixture();
                tactical.configure_battle_results_presentation_fixture();
                Ok(true)
            }
            InputScenesCommand::ReturnToGalaxy => {
                *game_mode = GameMode::Galaxy;
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    pub fn override_content_reports(&self, reports: &mut [ContentReport]) {
        let (Some(injection), Some(target)) = (self.failure, self.failure_target) else {
            return;
        };
        if let Some(report) = reports.iter_mut().find(|report| report.window == target) {
            report.status = injection.status();
        }
    }

    pub fn note_reconciliation(&mut self, reconciliation: &ReconcileReport) {
        if self
            .failure_target
            .is_some_and(|target| reconciliation.removed.contains(&target))
        {
            self.removal_count = self.removal_count.saturating_add(1);
            self.failure = None;
        }
    }

    pub fn queue_stale_target(&mut self, target: Option<WindowId>) {
        self.queued_target = target;
    }

    pub fn note_dormant_galaxy_modal(&mut self, target: Option<WindowId>) {
        self.dormant_galaxy_modal = target;
    }

    pub const fn queued_target(&self) -> Option<WindowId> {
        self.queued_target
    }

    pub const fn dormant_galaxy_modal(&self) -> Option<WindowId> {
        self.dormant_galaxy_modal
    }

    pub fn note_reset(&mut self, stale_action_discarded: bool) {
        self.reset_count = self.reset_count.saturating_add(1);
        self.failure = None;
        self.failure_target = None;
        self.stale_action_discarded = stale_action_discarded;
    }

    pub fn note_tactical_input(&mut self, action: &'static str, paused: bool) {
        self.tactical_inputs = self.tactical_inputs.saturating_add(1);
        self.tactical_action = Some(action);
        macroquad::logging::info!(
            "[windowing-input-scenes-action] recipient=tactical action={} paused={} count={}",
            action,
            paused,
            self.tactical_inputs,
        );
    }

    pub fn observe(&mut self, request: FixtureRequest, windows: &InputScenesWindows<'_>) {
        if request.scenario != Scenario::WindowingContract {
            return;
        }
        let snapshot = windows.snapshot;
        let mut registered = snapshot
            .paint_order()
            .iter()
            .chain(snapshot.presentation_history())
            .copied()
            .collect::<Vec<_>>();
        registered.sort_unstable();
        registered.dedup();
        let state = json!({
            "status": "input-scenes-state",
            "code": request.code,
            "command_sequence": self.command_sequence,
            "command": self.last_command.map(InputScenesCommand::label),
            "registered": registered.iter().map(|id| id.get()).collect::<Vec<_>>(),
            "record_count": registered.len(),
            "focused": snapshot.focused().map(|focus| json!({
                "window": focus.window.get(),
                "control": focus.control.map(|control| control.0),
            })),
            "capture": snapshot.capture_owner().map(WindowId::get),
            "fleet_dragging": windows.fleets.is_dragging(),
            "fleet_drag_owner": windows.fleets.drag_owner().map(WindowId::get),
            "active_modal": snapshot.active_modal().map(WindowId::get),
            "failure_target": self.failure_target.map(WindowId::get),
            "forced_status": self.failure.map(|failure| status_name(failure.status())),
            "blocking_unpainted": windows.reconciliation.blocking_unpainted
                .iter().map(|id| id.get()).collect::<Vec<_>>(),
            "removed": windows.reconciliation.removed
                .iter().map(|id| id.get()).collect::<Vec<_>>(),
            "removal_count": self.removal_count,
            "queued_target": self.queued_target.map(WindowId::get),
            "dormant_galaxy_modal": self.dormant_galaxy_modal.map(WindowId::get),
            "dormant_galaxy_registered": windows.dormant_galaxy_registered,
            "stale_action_discarded": self.stale_action_discarded,
            "tactical_inputs": self.tactical_inputs,
            "tactical_action": self.tactical_action,
            "tactical_paused": windows.tactical_paused,
            "reset_count": self.reset_count,
            "scene": windows.scene,
            "tactical_phase": windows.tactical_phase,
            "menu_rects": windows.menu_rects,
            "encyclopedia_rect": windows.encyclopedia_rect,
            "encyclopedia_content_status": windows.encyclopedia_content_status.map(status_name),
        });
        if self.last_state.as_ref() == Some(&state) {
            return;
        }
        self.observation_sequence = self.observation_sequence.saturating_add(1);
        self.last_state = Some(state.clone());
        let mut report = state;
        report
            .as_object_mut()
            .expect("input-scenes state is an object")
            .insert("sequence".into(), json!(self.observation_sequence));
        macroquad::logging::info!(
            "[windowing-contract-state] {}",
            serde_json::to_string(&report).expect("serialize input-scenes observation")
        );
    }

    #[cfg(test)]
    pub(super) fn apply_failure_for_test(&mut self, command: InputScenesCommand) {
        self.failure = match command {
            InputScenesCommand::ModalUnpaintable => Some(FailureInjection::ModalUnpaintable),
            InputScenesCommand::ModelessUnpaintable => Some(FailureInjection::ModelessUnpaintable),
            InputScenesCommand::ModelessAbsent => Some(FailureInjection::ModelessAbsent),
            InputScenesCommand::RecoverContent => None,
            _ => self.failure,
        };
    }

    #[cfg(test)]
    pub(super) const fn forced_status_for_test(&self) -> Option<ContentStatus> {
        match self.failure {
            Some(failure) => Some(failure.status()),
            None => None,
        }
    }
}

fn enter_tactical_alert(
    request: FixtureRequest,
    world: &mut GameWorld,
    tactical: &mut TacticalState,
    messages: &mut MessageLog,
    game_mode: &mut GameMode,
) -> Result<(), String> {
    if tactical.is_active() {
        return Ok(());
    }
    let system = world
        .systems
        .iter()
        .find(|(_, system)| !system.is_destroyed && system.fleets.is_empty())
        .map(|(key, _)| key)
        .ok_or("input-scenes fixture has no empty tactical system")?;
    let alliance_ship = world
        .capital_ship_classes
        .iter()
        .find(|(_, class)| class.is_alliance && !class.is_empire && class.hull > 0)
        .map(|(key, class)| (key, class.hull))
        .ok_or("input-scenes fixture has no Alliance capital ship")?;
    let empire_ship = world
        .capital_ship_classes
        .iter()
        .find(|(_, class)| class.is_empire && !class.is_alliance && class.hull > 0)
        .map(|(key, class)| (key, class.hull))
        .ok_or("input-scenes fixture has no Imperial capital ship")?;
    let make_fleet = |is_alliance: bool, class, hull| Fleet {
        location: system,
        capital_ships: vec![ShipInstance::new(
            class,
            i32::try_from(hull).unwrap_or(i32::MAX),
            is_alliance,
        )],
        fighters: Vec::new(),
        characters: Vec::new(),
        is_alliance,
        has_death_star: false,
    };
    let alliance = world
        .fleets
        .insert(make_fleet(true, alliance_ship.0, alliance_ship.1));
    let empire = world
        .fleets
        .insert(make_fleet(false, empire_ship.0, empire_ship.1));
    world.systems[system].fleets.extend([alliance, empire]);
    rebellion_core::carriage::tag_all(world);
    tactical_flow::begin_player_battle(
        world,
        BattleEntry {
            system,
            attacker: CombatForce::of_fleet(world, alliance),
            defender: CombatForce::of_fleet(world, empire),
            player_is_attacker: matches!(
                request.faction,
                rebellion_render::CockpitFaction::Alliance
            ),
            tick: 0,
            rng_seed: 0x12de_0001,
        },
        tactical,
        messages,
        game_mode,
    )
    .map_err(|error| format!("input-scenes tactical entry failed: {error:?}"))
}

pub struct InputScenesWindows<'a> {
    pub snapshot: &'a RoutingSnapshot,
    pub reconciliation: &'a ReconcileReport,
    pub fleets: &'a FleetWindowState,
    pub scene: &'static str,
    pub tactical_phase: &'static str,
    pub dormant_galaxy_registered: bool,
    pub menu_rects: &'a [[f32; 4]],
    pub encyclopedia_rect: Option<[f32; 4]>,
    pub encyclopedia_content_status: Option<ContentStatus>,
    pub tactical_paused: Option<bool>,
}

const fn is_modal(kind: WindowKind) -> bool {
    matches!(
        kind,
        WindowKind::FleetFinder
            | WindowKind::TroopFinder
            | WindowKind::PersonnelFinder
            | WindowKind::MessageIndex
            | WindowKind::Encyclopedia
            | WindowKind::Status
            | WindowKind::BuildSelection
            | WindowKind::MissionDialog
            | WindowKind::MoveConfirmation
            | WindowKind::ScrapConfirmation
            | WindowKind::BattleAlert
            | WindowKind::BattleResults
    )
}

const fn status_name(status: ContentStatus) -> &'static str {
    match status {
        ContentStatus::Ready => "ready",
        ContentStatus::Unpaintable => "unpaintable",
        ContentStatus::Absent => "absent",
        ContentStatus::Suspended => "suspended",
    }
}
