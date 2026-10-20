//! Fixture-only setup and read-only observations for windowing journeys A-B.

use std::collections::{BTreeMap, BTreeSet};

use rebellion_core::dat::{ExplorationStatus, Faction};
use rebellion_core::fog::FogState;
use rebellion_core::ids::{DatId, SystemKey};
use rebellion_core::missions::{
    set_on_mission, MissionFaction, MissionKind, MissionMember, MissionRequest, MissionState,
};
use rebellion_core::movement::{reconcile_fleet_orbits, MovementState};
use rebellion_core::troop_transport::TroopTransportState;
use rebellion_core::world::{ControlKind, GameWorld, ShipInstance, TroopUnit};
use rebellion_render::defenses_window::{DefensesPage, DefensesWindowState};
use rebellion_render::fleet_window::{FleetWindowState, FleetWindowTab};
use rebellion_render::missions_window::{MissionsTab, MissionsWindowState};
use rebellion_render::quadrant_icons::Quadrant;
use rebellion_render::system_window::{
    reference_rail_slots, SystemWindowState, SystemWindowTab, SYSTEM_WINDOW_HEIGHT,
    SYSTEM_WINDOW_WIDTH,
};
use rebellion_render::{
    strategic_primary_controls, CockpitButton, CockpitLayout, SectorWindowState,
};
use rebellion_windowing::{RoutingSnapshot, WindowId, WindowKind, WindowOwner};
use serde_json::{json, Value};

use super::{defenses_page_name, missions_tab_name, rect_array, FixtureRequest, Scenario};

const REQUIRED_SYSTEMS: usize = 16;

fn selected_systems(world: &GameWorld) -> Vec<SystemKey> {
    let mut sectors = BTreeSet::new();
    let mut systems: Vec<_> = world
        .systems
        .iter()
        .filter_map(|(key, system)| sectors.insert(system.sector).then_some(key))
        .take(3)
        .collect();
    for key in world.systems.keys() {
        if systems.len() == REQUIRED_SYSTEMS {
            break;
        }
        if !systems.contains(&key) {
            systems.push(key);
        }
    }
    systems
}

/// Stocks legitimate campaign content and deterministic geometry. It does not
/// open, focus, minimize, restore, or close any window.
pub(super) fn prepare(
    request: FixtureRequest,
    world: &mut GameWorld,
    movement: &MovementState,
    missions: &mut MissionState,
) {
    if request.scenario != Scenario::WindowingContract {
        return;
    }
    let systems = selected_systems(world);
    let player_is_alliance = request.faction == rebellion_render::CockpitFaction::Alliance;
    let player_control = if player_is_alliance {
        Faction::Alliance
    } else {
        Faction::Empire
    };
    let mut positioned_sectors = Vec::new();
    let mut sector_slots = BTreeMap::new();

    for (index, key) in systems.iter().copied().enumerate() {
        let sector = world.systems[key].sector;
        let (sector_x, sector_y) = positioned_sectors
            .iter()
            .find_map(|(positioned, x, y)| (*positioned == sector).then_some((*x, *y)))
            .unwrap_or_else(|| {
                let x = match index {
                    0 => 704,
                    1 => 96,
                    2 => 640,
                    _ if index % 2 == 0 => 160 + ((index as u16 * 23) % 240),
                    _ => 640 + ((index as u16 * 19) % 240),
                };
                let y = 80 + ((index as u16 * 31) % 300);
                positioned_sectors.push((sector, x, y));
                if let Some(value) = world.sectors.get_mut(sector) {
                    value.x = x;
                    value.y = y;
                }
                (x, y)
            });
        if let Some(system) = world.systems.get_mut(key) {
            let slot = sector_slots.entry(sector).or_insert(0_u16);
            let offset_x = 12 + (*slot % 4) * 18;
            let offset_y = 12 + (*slot / 4) * 18;
            *slot += 1;
            if matches!(index, 0 | 2) {
                // Keep the two right-half map openers in the live aperture sliver
                // between the two fixed Sector-window columns. DAT x=466 maps
                // to x=297 for Alliance and x=360 for Empire at native scale,
                // inside neither column. Sector membership still drives
                // left/right replacement; only the real map hit point must
                // remain exposed after both columns are occupied.
                system.x = 466;
                system.y = if index == 0 { 40 } else { 120 };
            } else {
                system.x = sector_x.saturating_add(offset_x);
                system.y = if matches!(index, 12 | 13) {
                    sector_y
                } else if index == 15 {
                    // Keep the System window's current Overview tab below the
                    // Defenses Personnel tab. Fleet remains at its ordinary
                    // on-screen opener, leaving all three families a genuine
                    // exposed activation surface after viewport resize.
                    sector_y.saturating_add(40)
                } else {
                    sector_y.saturating_add(offset_y)
                };
            }
            system.exploration_status = ExplorationStatus::Explored;
            system.is_populated = true;
            system.is_destroyed = false;
            system.control = ControlKind::Controlled(player_control);
        }
    }

    if let Some(template_key) = world.fleets.keys().next() {
        let template = world.fleets[template_key].clone();
        for (index, system) in systems.iter().copied().enumerate() {
            let fleet_key = if index == 0 {
                template_key
            } else {
                world.fleets.insert(template.clone())
            };
            let fleet = &mut world.fleets[fleet_key];
            fleet.location = system;
            fleet.is_alliance = player_is_alliance;
            fleet.has_death_star = false;
            if fleet.capital_ships.is_empty() {
                if let Some(class) = world.capital_ship_classes.keys().next() {
                    fleet
                        .capital_ships
                        .push(ShipInstance::new(class, 100, player_is_alliance));
                }
            }
        }
        reconcile_fleet_orbits(movement, world);
    }

    let personnel_system = systems.first().copied();
    let mut mission_members = Vec::new();
    for (index, (key, character)) in world.characters.iter_mut().take(15).enumerate() {
        let current_system = systems.get(index).copied().or(personnel_system);
        character.current_system = current_system;
        character.current_fleet = None;
        character.is_alliance = player_is_alliance;
        character.is_empire = !player_is_alliance;
        character.recruited = true;
        character.is_captive = false;
        character.on_mission = false;
        character.on_mandatory_mission = false;
        if index < 3 || index == 13 {
            mission_members.push((
                key,
                current_system.expect("the fixture selected sixteen systems"),
            ));
        }
    }
    let faction = if player_is_alliance {
        MissionFaction::Alliance
    } else {
        MissionFaction::Empire
    };
    for (character, system) in mission_members {
        missions.dispatch(MissionRequest::single(
            MissionKind::Espionage,
            faction,
            character,
            system,
            None,
            0,
        ));
        set_on_mission(world, MissionMember::Character(character), true);
    }
    if personnel_system.is_some() {
        let side_indices = if player_is_alliance { 1..=5 } else { 6..=10 };
        let mut class_dat_ids: Vec<_> = world
            .troop_classes
            .keys()
            .copied()
            .filter(|dat_id| dat_id.family() == 0x10 && side_indices.contains(&dat_id.index()))
            .collect();
        class_dat_ids.sort_by_key(|dat_id| dat_id.raw());
        if class_dat_ids.is_empty() {
            class_dat_ids.push(DatId::new(if player_is_alliance {
                0x1000_0001
            } else {
                0x1000_0006
            }));
        }
        for (index, system) in systems.iter().copied().enumerate() {
            for unit_index in 0..if index == 0 { 15 } else { 1 } {
                let troop = world.troops.insert(TroopUnit {
                    class_dat_id: class_dat_ids[unit_index % class_dat_ids.len()],
                    is_alliance: player_is_alliance,
                    regiment_strength: 100,
                });
                world.systems[system].ground_units.push(troop);
            }
        }
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

fn owner_value(owner: WindowOwner) -> Value {
    match owner {
        WindowOwner::StrategicRoot => json!("root"),
        WindowOwner::Window(id) => json!(id.get()),
    }
}

fn record(snapshot: &RoutingSnapshot, id: WindowId) -> Value {
    let key = snapshot.key(id).expect("snapshot order names a live key");
    json!({
        "id": id.get(),
        "kind": kind_name(key.kind),
        "subject": key.subject.map(|subject| format!("{}:{}", subject.namespace, subject.value)),
        "owner": snapshot.owner(id).map(owner_value),
        "visible": snapshot.is_visible(id),
    })
}

fn point(rect: egui_macroquad::egui::Rect) -> [f32; 2] {
    [rect.center().x, rect.center().y]
}

fn logical_window_rect(
    layout: CockpitLayout,
    origin: (i16, i16),
    width: f32,
    height: f32,
) -> egui_macroquad::egui::Rect {
    egui_macroquad::egui::Rect::from_min_size(
        egui_macroquad::egui::pos2(
            layout.canvas.x + f32::from(origin.0) * layout.scale,
            layout.canvas.y + f32::from(origin.1) * layout.scale,
        ),
        egui_macroquad::egui::vec2(width * layout.scale, height * layout.scale),
    )
}

fn sector_window_rect(
    world: &GameWorld,
    windows: &IdentityHistoryWindows<'_>,
    key: SystemKey,
) -> Option<egui_macroquad::egui::Rect> {
    let sector = world.systems[key].sector;
    windows.sectors.window_id(sector)?;
    let planet = windows
        .sectors
        .planet_screen_rect(world, windows.layout, key)?;
    let positions = match windows.faction {
        rebellion_render::CockpitFaction::Alliance => [(60, 35), (300, 35)],
        rebellion_render::CockpitFaction::Empire => [(120, 40), (365, 40)],
    };
    positions.into_iter().find_map(|origin| {
        let rect = logical_window_rect(
            windows.layout,
            origin,
            rebellion_render::sector_window::SECTOR_WINDOW_WIDTH,
            rebellion_render::sector_window::SECTOR_WINDOW_HEIGHT,
        );
        rect.contains(planet.center()).then_some(rect)
    })
}

fn system_state(world: &GameWorld, windows: &IdentityHistoryWindows<'_>, key: SystemKey) -> Value {
    let system = &world.systems[key];
    let layout = windows.layout;
    let map_point = rebellion_render::galaxy_camera(
        (
            layout.galaxy.x,
            layout.galaxy.y,
            layout.galaxy.width,
            layout.galaxy.height,
        ),
        layout.scale,
        windows.faction,
    )
    .to_screen(f32::from(system.x), f32::from(system.y));
    let sector_rect = sector_window_rect(world, windows, key);
    let system_origin = windows
        .systems
        .open_windows()
        .find_map(|(candidate, origin)| (candidate == key).then_some(origin));
    let system_rect = system_origin.map(|origin| {
        logical_window_rect(layout, origin, SYSTEM_WINDOW_WIDTH, SYSTEM_WINDOW_HEIGHT)
    });
    let system_selection = windows.systems.selection(key).map(|(tab, band)| {
        json!({
            "tab": format!("{tab:?}").to_lowercase(),
            "band": band.map(|value| format!("{value:?}").to_lowercase()),
        })
    });
    let fleet_report =
        windows
            .fleets
            .report(world, windows.movement, windows.fog, windows.transport, key);
    let defenses_report =
        windows
            .defenses
            .report(world, windows.fog, windows.missions, windows.economy, key);
    let missions_report =
        windows
            .missions_windows
            .report(world, windows.fog, windows.missions, key);

    json!({
        "dat_id": system.dat_id.raw(),
        "name": system.name,
        "sector_dat_id": world.sectors[system.sector].dat_id.raw(),
        "map_point": map_point,
        "sector_id": windows.sectors.window_id(system.sector).map(WindowId::get),
        "sector_rect": sector_rect.map(rect_array),
        "sector_title_point": sector_rect.map(|rect| [
            rect.min.x + 100.0 * layout.scale,
            rect.min.y + 9.0 * layout.scale,
        ]),
        "sector_close_point": sector_rect.map(|rect| [
            rect.min.x + 225.0 * layout.scale,
            rect.min.y + 9.0 * layout.scale,
        ]),
        "planet_rect": windows.sectors.planet_screen_rect(world, layout, key).map(rect_array),
        "quadrants": Quadrant::ALL.into_iter().filter_map(|quadrant| {
            windows.sectors.quadrant_screen_rect(world, layout, key, quadrant)
                .map(|rect| (format!("{quadrant:?}").to_lowercase(), rect_array(rect)))
        }).collect::<Vec<_>>(),
        "details": {
            "system": {
                "id": windows.systems.window_id(key).map(WindowId::get),
                "rect": system_rect.map(rect_array),
                "selection": system_selection,
                "tabs": SystemWindowTab::ALL.into_iter().filter_map(|tab| {
                    windows.systems.tab_screen_rect(layout, key, tab)
                        .map(|rect| (format!("{tab:?}").to_lowercase(), rect_array(rect)))
                }).collect::<Vec<_>>(),
            },
            "fleet": {
                "id": windows.fleets.window_id(key).map(WindowId::get),
                "rect": windows.fleets.screen_rect(layout, key).map(rect_array),
                "selection": fleet_report.as_ref().map(|report| json!({
                    "selected": report.selected.map(|entry| format!("{entry:?}")),
                    "tab": format!("{:?}", report.tab).to_lowercase(),
                    "items": report.items,
                })),
                "entry_rect": windows.fleets.entry_screen_rect(layout, key, 0).map(rect_array),
                "tabs": FleetWindowTab::ALL.into_iter().filter_map(|tab| {
                    windows.fleets.tab_screen_rect(layout, key, tab)
                        .map(|rect| (format!("{tab:?}").to_lowercase(), rect_array(rect)))
                }).collect::<Vec<_>>(),
            },
            "defenses": {
                "id": windows.defenses.window_id(key).map(WindowId::get),
                "rect": windows.defenses.screen_rect(layout, key).map(rect_array),
                "selection": defenses_report.as_ref().map(|report| json!({
                    "page": defenses_page_name(report.page),
                    "first_row": report.first_row,
                    "selected": report.selected,
                    "rows": report.rows,
                })),
                "cell_rect": windows.defenses.cell_screen_rect(layout, key, 0).map(rect_array),
                "tabs": DefensesPage::ALL.into_iter().filter_map(|page| {
                    windows.defenses.tab_screen_rect(layout, key, page)
                        .map(|rect| (defenses_page_name(page), rect_array(rect)))
                }).collect::<Vec<_>>(),
            },
            "missions": {
                "id": windows.missions_windows.window_id(key).map(WindowId::get),
                "rect": windows.missions_windows.screen_rect(layout, key).map(rect_array),
                "selection": missions_report.as_ref().map(|report| json!({
                    "tab": missions_tab_name(report.tab),
                    "selected": report.selected,
                    "members": report.members,
                })),
                "tabs": MissionsTab::ALL.into_iter().filter_map(|tab| {
                    windows.missions_windows.tab_screen_rect(layout, key, tab)
                        .map(|rect| (missions_tab_name(tab), rect_array(rect)))
                }).collect::<Vec<_>>(),
            },
        },
    })
}

/// Read-only state sources used by [`IdentityHistoryWatch`].
pub struct IdentityHistoryWindows<'a> {
    pub snapshot: &'a RoutingSnapshot,
    pub layout: CockpitLayout,
    pub faction: rebellion_render::CockpitFaction,
    pub sectors: &'a SectorWindowState,
    pub systems: &'a SystemWindowState,
    pub fleets: &'a FleetWindowState,
    pub defenses: &'a DefensesWindowState,
    pub missions_windows: &'a MissionsWindowState,
    pub movement: &'a MovementState,
    pub fog: &'a FogState,
    pub transport: &'a TroopTransportState,
    pub missions: &'a MissionState,
    pub economy: &'a rebellion_core::economy::EconomyState,
}

/// Emits one state document whenever manager/content/geometry state changes.
#[derive(Debug, Default)]
pub struct IdentityHistoryWatch {
    sequence: u64,
    last_state: Option<Value>,
}

impl IdentityHistoryWatch {
    fn next(
        &mut self,
        request: FixtureRequest,
        world: &GameWorld,
        windows: &IdentityHistoryWindows<'_>,
    ) -> Option<Value> {
        if request.scenario != Scenario::WindowingContract {
            return None;
        }
        let systems = selected_systems(world);
        let snapshot = windows.snapshot;
        let mut registered: Vec<_> = snapshot
            .paint_order()
            .iter()
            .chain(snapshot.presentation_history())
            .copied()
            .collect();
        registered.sort_unstable();
        registered.dedup();
        let state = json!({
            "status": "identity-history-state",
            "code": request.code,
            "available": systems.len() >= REQUIRED_SYSTEMS,
            "required_systems": REQUIRED_SYSTEMS,
            "scale": windows.layout.scale,
            "fleet_finder_point": strategic_primary_controls(windows.faction)
                .iter()
                .find(|control| control.button == CockpitButton::FleetFinder)
                .map(|control| [
                    windows.layout.canvas.x
                        + (control.rect.x + control.rect.width / 2.0) * windows.layout.scale,
                    windows.layout.canvas.y
                        + (control.rect.y + control.rect.height / 2.0) * windows.layout.scale,
                ]),
            "snapshot": {
                "paint_order": snapshot.paint_order().iter().map(|id| id.get()).collect::<Vec<_>>(),
                "history": snapshot.presentation_history().iter().map(|id| id.get()).collect::<Vec<_>>(),
                "registered": registered.iter().map(|id| id.get()).collect::<Vec<_>>(),
                "records": registered.iter().map(|id| record(snapshot, *id)).collect::<Vec<_>>(),
                "focused": snapshot.focused().map(|focus| json!({
                    "window": focus.window.get(),
                    "control": focus.control.map(|control| control.0),
                })),
                "capture": snapshot.capture_owner().map(WindowId::get),
                "active_modal": snapshot.active_modal().map(WindowId::get),
            },
            "systems": systems.iter().copied().map(|key| system_state(world, windows, key)).collect::<Vec<_>>(),
            "rail": reference_rail_slots(snapshot, world, windows.faction, windows.layout)
                .into_iter()
                .map(|slot| json!({
                    "id": slot.id().get(),
                    "kind": kind_name(slot.kind()),
                    "system_dat_id": world.systems[slot.system()].dat_id.raw(),
                    "rect": rect_array(slot.rect()),
                    "point": point(slot.rect()),
                }))
                .collect::<Vec<_>>(),
        });
        if self.last_state.as_ref() == Some(&state) {
            return None;
        }
        self.sequence = self.sequence.saturating_add(1);
        self.last_state = Some(state.clone());
        Some(json!({ "sequence": self.sequence, "state": state }))
    }

    pub fn observe(
        &mut self,
        request: FixtureRequest,
        world: &GameWorld,
        windows: &IdentityHistoryWindows<'_>,
    ) {
        let Some(mut report) = self.next(request, world, windows) else {
            return;
        };
        let object = report
            .as_object_mut()
            .expect("fixture state report is an object");
        let state = object
            .remove("state")
            .expect("fixture state report includes state");
        let state = state.as_object().expect("fixture state is an object");
        for (key, value) in state {
            object.insert(key.clone(), value.clone());
        }
        macroquad::logging::info!(
            "[windowing-contract-state] {}",
            serde_json::to_string(&report).expect("serialize identity/history observation")
        );
    }
}
