//! Fixture-only campaign content for windowing Journey C.
//!
//! This module deliberately does not open, focus, or otherwise publish a
//! window. The browser must reach every modal through the same controls,
//! menus, targeting cursor, and manager receipts used by the application.

use rebellion_core::dat::Faction;
use rebellion_core::world::{GameWorld, ManufacturingFacilityInstance};
use rebellion_render::{strategic_primary_controls, CockpitLayout, GalaxyMapState};
use rebellion_windowing::{RoutingSnapshot, WindowId, WindowKind};
use serde_json::{json, Value};

use super::{FixtureRequest, Scenario};

/// Stocks one idle local agent plus a legitimate producer of every
/// manufacturing family so Mission and Build use real live entry paths.
pub(super) fn prepare(request: FixtureRequest, world: &mut GameWorld) {
    if request.scenario != Scenario::WindowingContract {
        return;
    }
    let Some(primary) = world.systems.keys().next() else {
        return;
    };
    let side = Faction::of_alliance(matches!(
        request.faction,
        rebellion_render::CockpitFaction::Alliance
    ));
    let player_is_alliance = matches!(request.faction, rebellion_render::CockpitFaction::Alliance);
    if let Some((_, character)) = world.characters.iter_mut().find(|(_, character)| {
        character.is_alliance == player_is_alliance
            && character.is_empire != player_is_alliance
            && !character.is_killed
            && !character.is_captive
            && !character.on_mission
            && !character.on_mandatory_mission
    }) {
        character.current_system = Some(primary);
        character.current_fleet = None;
        character.recruited = true;
    }
    let missing = [
        (0x2800_0001, true),
        (0x2900_0002, false),
        (0x2a00_0003, false),
    ];
    for (class, is_shipyard) in missing {
        let present = world.systems[primary]
            .manufacturing_facilities
            .iter()
            .any(|key| {
                world
                    .manufacturing_facilities
                    .get(*key)
                    .is_some_and(|facility| {
                        facility.class_dat_id.raw() == class && facility.side == side
                    })
            });
        if present {
            continue;
        }
        let facility = world
            .manufacturing_facilities
            .insert(ManufacturingFacilityInstance {
                class_dat_id: rebellion_core::ids::DatId::new(class),
                side,
                is_shipyard,
            });
        world.systems[primary]
            .manufacturing_facilities
            .push(facility);
    }
}

fn kind_name(kind: WindowKind) -> &'static str {
    match kind {
        WindowKind::Sector => "sector",
        WindowKind::System => "system",
        WindowKind::Fleet => "fleet",
        WindowKind::Defenses => "defenses",
        WindowKind::Missions => "missions",
        WindowKind::SystemFinder => "system-finder",
        WindowKind::FleetFinder => "fleet-finder",
        WindowKind::TroopFinder => "troop-finder",
        WindowKind::PersonnelFinder => "personnel-finder",
        WindowKind::MessageIndex => "message-index",
        WindowKind::Encyclopedia => "encyclopedia",
        WindowKind::Status => "status",
        WindowKind::BuildSelection => "build-selection",
        WindowKind::MissionDialog => "mission-dialog",
        WindowKind::MoveConfirmation => "move-confirmation",
        WindowKind::ScrapConfirmation => "scrap-confirmation",
        WindowKind::BattleAlert => "battle-alert",
        WindowKind::BattleResults => "battle-results",
        WindowKind::Menu => "menu",
    }
}

/// Read-only sources used by [`ModalBoundaryWatch`].
pub struct ModalBoundaryWindows<'a> {
    pub snapshot: &'a RoutingSnapshot,
    pub layout: CockpitLayout,
    pub faction: rebellion_render::CockpitFaction,
    pub map: &'a GalaxyMapState,
}

/// Emits the modal owner and stable underlay state used by Journey C. It does
/// not mutate renderer, manager, focus, capture, or campaign state.
#[derive(Debug, Default)]
pub struct ModalBoundaryWatch {
    sequence: u64,
    last_state: Option<Value>,
}

impl ModalBoundaryWatch {
    pub fn observe(
        &mut self,
        request: FixtureRequest,
        world: &GameWorld,
        windows: &ModalBoundaryWindows<'_>,
    ) {
        if request.scenario != Scenario::WindowingContract {
            return;
        }
        let snapshot = windows.snapshot;
        let active_modal = snapshot.active_modal().map(|id| {
            let key = snapshot.key(id).expect("active modal has a live key");
            json!({ "id": id.get(), "kind": kind_name(key.kind) })
        });
        let map_points = world
            .systems
            .iter()
            .take(4)
            .map(|(_, system)| {
                let (x, y) = rebellion_render::galaxy_camera(
                    (
                        windows.layout.galaxy.x,
                        windows.layout.galaxy.y,
                        windows.layout.galaxy.width,
                        windows.layout.galaxy.height,
                    ),
                    windows.layout.scale,
                    windows.faction,
                )
                .to_screen(f32::from(system.x), f32::from(system.y));
                json!({ "dat_id": system.dat_id.raw(), "point": [x, y] })
            })
            .collect::<Vec<_>>();
        let selected_system = windows
            .map
            .selected_system
            .and_then(|key| world.systems.get(key))
            .map(|system| system.dat_id.raw());
        let state = json!({
            "status": "modal-boundary-state",
            "code": request.code,
            "scale": windows.layout.scale,
            "active_modal": active_modal,
            "selected_system_dat_id": selected_system,
            "paint_order": snapshot.paint_order().iter().map(|id| id.get()).collect::<Vec<_>>(),
            "history": snapshot.presentation_history().iter().map(|id| id.get()).collect::<Vec<_>>(),
            "focused": snapshot.focused().map(|focus| json!({
                "window": focus.window.get(),
                "control": focus.control.map(|control| control.0),
            })),
            "capture": snapshot.capture_owner().map(WindowId::get),
            "map_points": map_points,
            "cockpit": strategic_primary_controls(windows.faction)
                .iter()
                .map(|control| (format!("{:?}", control.button).to_lowercase(), [
                    windows.layout.canvas.x
                        + (control.rect.x + control.rect.width / 2.0) * windows.layout.scale,
                    windows.layout.canvas.y
                        + (control.rect.y + control.rect.height / 2.0) * windows.layout.scale,
                ]))
                .collect::<Vec<_>>(),
        });
        if self.last_state.as_ref() == Some(&state) {
            return;
        }
        self.sequence = self.sequence.saturating_add(1);
        self.last_state = Some(state.clone());
        let mut report = state;
        report
            .as_object_mut()
            .expect("modal state is an object")
            .insert("sequence".into(), json!(self.sequence));
        macroquad::logging::info!(
            "[windowing-contract-state] {}",
            serde_json::to_string(&report).expect("serialize modal-boundary observation")
        );
    }
}
