//! Fleets panel — fleet roster with composition editing, character assignment,
//! and fleet merge controls.
//!
//! Rendered as a left-side egui panel. Lists all fleets belonging to the player
//! faction. Clicking a fleet expands a detail row showing capital ships, fighter
//! squadrons, assigned characters, and action buttons (assign/remove officer,
//! merge with another fleet at same system, go to system).

use egui_macroquad::egui::{self, RichText, ScrollArea};
use rebellion_core::ids::{CharacterKey, FleetKey};
use rebellion_core::missions::MissionFaction;
use rebellion_core::movement::MovementState;
use rebellion_core::world::GameWorld;

use crate::theme;
use super::PanelAction;

// ---------------------------------------------------------------------------
// FleetsState
// ---------------------------------------------------------------------------

/// Mutable state for the fleets panel.
#[derive(Debug, Clone, Default)]
pub struct FleetsState {
    /// The fleet currently expanded to show details.
    pub expanded_fleet: Option<FleetKey>,
    /// If set, show the character assignment picker for this fleet.
    pub assigning_to: Option<FleetKey>,
}

// ---------------------------------------------------------------------------
// draw_fleets
// ---------------------------------------------------------------------------

/// Render the fleet roster as a left-side egui panel.
///
/// `player_faction` filters which fleets are shown. Returns panel actions for
/// character assignment, fleet merging, and map navigation.
pub fn draw_fleets(
    ctx: &egui::Context,
    world: &GameWorld,
    movement_state: &MovementState,
    state: &mut FleetsState,
    player_faction: MissionFaction,
) -> Option<PanelAction> {
    let mut action = None;

    egui::SidePanel::left("fleets_panel")
        .min_width(280.0)
        .max_width(360.0)
        .show(ctx, |ui| {
            ui.heading(RichText::new("Fleets").color(theme::GOLD));
            ui.separator();

            let player_fleets: Vec<(FleetKey, &rebellion_core::world::Fleet)> = world
                .fleets
                .iter()
                .filter(|(_, f)| faction_matches(f.is_alliance, player_faction))
                .collect();

            ui.label(
                RichText::new(format!("{} fleet(s)", player_fleets.len()))
                    .color(theme::TEXT_SECONDARY)
                    .size(11.0),
            );
            ui.add_space(4.0);

            ScrollArea::vertical().show(ui, |ui| {
                for &(fleet_key, fleet) in &player_fleets {
                    let system_name = world
                        .systems
                        .get(fleet.location)
                        .map(|s| s.name.as_str())
                        .unwrap_or("Unknown");

                    let ship_count: u32 = fleet.capital_ships.iter().map(|e| e.count).sum();
                    let fighter_count: u32 = fleet.fighters.iter().map(|e| e.count).sum();
                    let is_expanded = state.expanded_fleet == Some(fleet_key);

                    // ── Fleet header row ─────────────────────────────────
                    ui.horizontal(|ui| {
                        let toggle = if is_expanded { "▼" } else { "▶" };
                        if ui.small_button(toggle).clicked() {
                            state.expanded_fleet = if is_expanded {
                                None
                            } else {
                                Some(fleet_key)
                            };
                            state.assigning_to = None;
                        }

                        ui.label(
                            RichText::new(format!("Fleet @ {}", system_name))
                                .color(fleet_color(player_faction))
                                .strong(),
                        );

                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                let mut parts = Vec::new();
                                if ship_count > 0 { parts.push(format!("{} ships", ship_count)); }
                                if fighter_count > 0 { parts.push(format!("{} sqns", fighter_count)); }
                                if fleet.has_death_star { parts.push("DS".to_string()); }
                                ui.label(
                                    RichText::new(parts.join(" · "))
                                        .size(10.0)
                                        .color(theme::TEXT_SECONDARY),
                                );
                            },
                        );
                    });

                    if is_expanded {
                        ui.indent("fleet_detail", |ui| {
                            // ── Capital ships ────────────────────────────
                            if !fleet.capital_ships.is_empty() {
                                ui.label(
                                    RichText::new("CAPITAL SHIPS")
                                        .color(theme::GOLD_DIM)
                                        .size(10.0)
                                        .strong(),
                                );
                                for entry in &fleet.capital_ships {
                                    let class_name = world
                                        .capital_ship_classes
                                        .get(entry.class)
                                        .map(|c| c.name.as_str())
                                        .unwrap_or("Unknown");
                                    ui.label(
                                        RichText::new(format!("  {} ×{}", class_name, entry.count))
                                            .color(theme::TEXT_PRIMARY)
                                            .size(11.0),
                                    );
                                }
                            }

                            // ── Fighter squadrons ────────────────────────
                            if !fleet.fighters.is_empty() {
                                ui.add_space(2.0);
                                ui.label(
                                    RichText::new("FIGHTER SQUADRONS")
                                        .color(theme::GOLD_DIM)
                                        .size(10.0)
                                        .strong(),
                                );
                                for entry in &fleet.fighters {
                                    let class_name = world
                                        .fighter_classes
                                        .get(entry.class)
                                        .map(|c| c.name.as_str())
                                        .unwrap_or("Unknown");
                                    ui.label(
                                        RichText::new(format!("  {} ×{}", class_name, entry.count))
                                            .color(theme::TEXT_PRIMARY)
                                            .size(11.0),
                                    );
                                }
                            }

                            // ── Death Star ───────────────────────────────
                            if fleet.has_death_star {
                                ui.add_space(2.0);
                                ui.label(
                                    RichText::new("DEATH STAR PRESENT")
                                        .color(theme::DANGER_RED)
                                        .size(11.0)
                                        .strong(),
                                );
                            }

                            // ── Assigned officers ────────────────────────
                            ui.add_space(4.0);
                            ui.label(
                                RichText::new("OFFICERS")
                                    .color(theme::GOLD_DIM)
                                    .size(10.0)
                                    .strong(),
                            );
                            if fleet.characters.is_empty() {
                                ui.label(
                                    RichText::new("  None assigned")
                                        .color(theme::TEXT_DISABLED)
                                        .size(10.0),
                                );
                            }
                            for &char_key in &fleet.characters {
                                let char_name = world
                                    .characters
                                    .get(char_key)
                                    .map(|c| c.name.as_str())
                                    .unwrap_or("Unknown");
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(format!("  {}", char_name))
                                            .color(theme::TEXT_PRIMARY)
                                            .size(11.0),
                                    );
                                    // Remove from fleet button
                                    if ui.small_button(
                                        RichText::new("×").color(theme::DANGER_RED).size(11.0),
                                    ).clicked() {
                                        action = Some(PanelAction::RemoveCharacterFromFleet {
                                            character: char_key,
                                            fleet: fleet_key,
                                        });
                                    }
                                });
                            }

                            // ── Assign officer picker ────────────────────
                            if state.assigning_to == Some(fleet_key) {
                                ui.add_space(4.0);
                                ui.label(
                                    RichText::new("Select officer to assign:")
                                        .color(theme::GOLD)
                                        .size(10.0),
                                );
                                let available = available_characters(world, fleet_key, player_faction);
                                if available.is_empty() {
                                    ui.label(
                                        RichText::new("No available officers")
                                            .color(theme::TEXT_DISABLED)
                                            .size(10.0),
                                    );
                                } else {
                                    ScrollArea::vertical()
                                        .id_salt("assign_scroll")
                                        .max_height(120.0)
                                        .show(ui, |ui| {
                                            for (ck, name) in &available {
                                                if ui.button(
                                                    RichText::new(name.as_str())
                                                        .color(theme::TEXT_PRIMARY)
                                                        .size(11.0),
                                                ).clicked() {
                                                    action = Some(PanelAction::AssignCharacterToFleet {
                                                        character: *ck,
                                                        fleet: fleet_key,
                                                    });
                                                    state.assigning_to = None;
                                                }
                                            }
                                        });
                                }
                                if ui.small_button(
                                    RichText::new("Cancel").color(theme::TEXT_DISABLED).size(10.0),
                                ).clicked() {
                                    state.assigning_to = None;
                                }
                            } else if ui.button(
                                RichText::new("Assign Officer")
                                    .color(theme::GOLD)
                                    .size(11.0),
                            ).clicked() {
                                state.assigning_to = Some(fleet_key);
                            }

                            // ── Merge with fleet at same location ────────
                            // Only show merge for stationary fleets (not in transit).
                            ui.add_space(4.0);
                            let this_in_transit = movement_state.get(fleet_key).is_some();
                            let same_loc_fleets: Vec<(FleetKey, u32)> = if this_in_transit {
                                vec![]
                            } else {
                                player_fleets
                                    .iter()
                                    .filter(|(fk, f)| {
                                        *fk != fleet_key
                                            && f.location == fleet.location
                                            && movement_state.get(*fk).is_none()
                                    })
                                    .map(|(fk, f)| {
                                        let sc: u32 = f.capital_ships.iter().map(|e| e.count).sum();
                                        (*fk, sc)
                                    })
                                    .collect()
                            };

                            if !same_loc_fleets.is_empty() {
                                ui.label(
                                    RichText::new("MERGE")
                                        .color(theme::GOLD_DIM)
                                        .size(10.0)
                                        .strong(),
                                );
                                for (other_key, other_ships) in &same_loc_fleets {
                                    if ui.button(
                                        RichText::new(format!("Merge with fleet ({} ships)", other_ships))
                                            .color(theme::TEXT_PRIMARY)
                                            .size(11.0),
                                    ).clicked() {
                                        action = Some(PanelAction::MergeFleets {
                                            fleet_a: fleet_key,
                                            fleet_b: *other_key,
                                        });
                                        state.expanded_fleet = None;
                                    }
                                }
                            }

                            // ── Navigation ───────────────────────────────
                            ui.add_space(4.0);
                            if ui.button(
                                RichText::new("Go to System")
                                    .color(theme::GOLD)
                                    .size(11.0),
                            ).clicked() {
                                action = Some(PanelAction::FocusFleetSystem(fleet.location));
                            }
                        });

                        ui.add_space(4.0);
                    }

                    ui.separator();
                }
            });
        });

    action
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn faction_matches(is_alliance: bool, player: MissionFaction) -> bool {
    match player {
        MissionFaction::Alliance => is_alliance,
        MissionFaction::Empire => !is_alliance,
    }
}

fn fleet_color(faction: MissionFaction) -> egui::Color32 {
    match faction {
        MissionFaction::Alliance => theme::ALLIANCE_BLUE,
        MissionFaction::Empire => theme::EMPIRE_RED,
    }
}

/// Find characters eligible for fleet assignment:
/// same faction, not captive, not on mission, not already in this fleet.
fn available_characters(
    world: &GameWorld,
    fleet_key: FleetKey,
    player_faction: MissionFaction,
) -> Vec<(CharacterKey, String)> {
    let fleet = match world.fleets.get(fleet_key) {
        Some(f) => f,
        None => return vec![],
    };

    let mut result = Vec::new();
    for (ck, c) in world.characters.iter() {
        // Faction filter
        let owns = match player_faction {
            MissionFaction::Alliance => c.is_alliance,
            MissionFaction::Empire => c.is_empire,
        };
        if !owns { continue; }
        if c.is_captive || c.on_mission || c.on_mandatory_mission { continue; }
        // Not already in this fleet
        if fleet.characters.contains(&ck) { continue; }
        // Not already in another fleet
        let in_another = world.fleets.values().any(|f| f.characters.contains(&ck));
        if in_another { continue; }
        result.push((ck, c.name.clone()));
    }
    result.sort_by(|a, b| a.1.cmp(&b.1));
    result
}
