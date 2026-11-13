//! Fixture-only campaign content for windowing Journey C.
//!
//! This module deliberately does not open, focus, or otherwise publish a
//! window. The browser must reach every modal through the same controls,
//! menus, targeting cursor, and manager receipts used by the application.

use rebellion_core::dat::Faction;
use rebellion_core::world::{GameWorld, ManufacturingFacilityInstance};
use rebellion_render::{strategic_primary_controls, CockpitLayout, GalaxyMapState};
use rebellion_windowing::RoutingSnapshot;
use serde_json::{json, Value};

use super::{
    capture_window_id, focused_window_json, log_changed_observation, strategic_control_center,
    FixtureRequest, Scenario,
};

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
            json!({ "id": id.get(), "kind": key.kind.slug() })
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
            "focused": focused_window_json(snapshot),
            "capture": capture_window_id(snapshot),
            "map_points": map_points,
            "cockpit": strategic_primary_controls(windows.faction)
                .iter()
                .map(|control| (
                    format!("{:?}", control.button).to_lowercase(),
                    strategic_control_center(windows.layout, control),
                ))
                .collect::<Vec<_>>(),
        });
        log_changed_observation(state, &mut self.last_state, &mut self.sequence);
    }
}
