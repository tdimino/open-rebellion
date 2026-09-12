//! Fleets panel — fleet roster with composition editing, character assignment,
//! fleet merge controls, and destination-based dispatch.
//!
//! Rendered as a left-side egui panel. Lists all fleets belonging to the player
//! faction. Clicking a fleet expands a detail row showing capital ships, fighter
//! squadrons, assigned characters, and action buttons (assign/remove officer,
//! merge with another fleet at the same system, move, and go to system).

use std::collections::HashMap;

use egui_macroquad::egui::{self, RichText, ScrollArea, Vec2};
use rebellion_core::ids::{CharacterKey, DatId, FleetKey, SystemKey, TroopKey};
use rebellion_core::missions::MissionFaction;
use rebellion_core::movement::{validate_fleet_dispatch, FleetDispatchError, MovementState};
use rebellion_core::troop_transport::TroopTransportState;
use rebellion_core::world::GameWorld;

use super::PanelAction;
use crate::bmp_cache::{resources::gokres, BmpCache, DllSource};
use crate::theme;

// GOKRES.DLL stores fleet mini-icons in faction-specific resource blocks. These
// tables follow CAPSHPSD.DAT and FIGHTSD.DAT record order; they deliberately do
// not derive a resource ID by adding an offset to the compound DatId.
const ALLIANCE_FIGHTER_MINIS: [u32; 4] = [
    gokres::MINI_FIGHTER_A_WING,
    gokres::MINI_FIGHTER_B_WING,
    gokres::MINI_FIGHTER_X_WING,
    gokres::MINI_FIGHTER_Y_WING,
];

const EMPIRE_FIGHTER_MINIS: [u32; 4] = [
    gokres::MINI_FIGHTER_TIE_FIGHTER,
    gokres::MINI_FIGHTER_TIE_INTERCEPTOR,
    gokres::MINI_FIGHTER_TIE_BOMBER,
    gokres::MINI_FIGHTER_TIE_DEFENDER,
];

const ALLIANCE_CAPITAL_SHIP_MINIS: [u32; 15] = [
    gokres::MINI_SHIP_MC80_LIBERTY_CRUISER,
    gokres::MINI_SHIP_BULK_CRUISER,
    gokres::MINI_SHIP_ASSAULT_FRIGATE,
    gokres::MINI_SHIP_NEBULON_B_FRIGATE,
    gokres::MINI_SHIP_ALLIANCE_ESCORT_CARRIER,
    gokres::MINI_SHIP_CORELLIAN_CORVETTE,
    gokres::MINI_SHIP_MEDIUM_TRANSPORT,
    gokres::MINI_SHIP_BULK_TRANSPORT,
    gokres::MINI_SHIP_CORELLIAN_GUNSHIP,
    gokres::MINI_SHIP_ALLIANCE_DREADNAUGHT,
    gokres::MINI_SHIP_CC_7700_FRIGATE,
    gokres::MINI_SHIP_VISCOUNT_STAR_DEFENDER,
    gokres::MINI_SHIP_LIBERATOR_CRUISER,
    gokres::MINI_SHIP_MC30C_FRIGATE,
    gokres::MINI_SHIP_MC80A_HOME_ONE_CRUISER,
];

const EMPIRE_CAPITAL_SHIP_MINIS: [u32; 15] = [
    gokres::MINI_SHIP_STRIKE_CRUISER,
    gokres::MINI_SHIP_LANCER_FRIGATE,
    gokres::MINI_SHIP_INTERDICTOR_CRUISER,
    gokres::MINI_SHIP_CARRACK_LIGHT_CRUISER,
    gokres::MINI_SHIP_VICTORY_I_STAR_DESTROYER,
    gokres::MINI_SHIP_IMPERIAL_I_STAR_DESTROYER,
    gokres::MINI_SHIP_SUPER_STAR_DESTROYER,
    gokres::MINI_SHIP_GLADIATOR_STAR_DESTROYER,
    gokres::MINI_SHIP_DEATH_STAR,
    gokres::MINI_SHIP_ACCLAMATOR_DROP_SHIP,
    gokres::MINI_SHIP_VICTORY_II_STAR_DESTROYER,
    gokres::MINI_SHIP_IMPERIAL_II_STAR_DESTROYER,
    gokres::MINI_SHIP_STAR_GALLEON_FRIGATE,
    gokres::MINI_SHIP_IMPERIAL_ESCORT_CARRIER,
    gokres::MINI_SHIP_IMPERIAL_DREADNOUGHT,
];

/// Displayed size for GOKRES mini-icons in panel lists (original is 61x25).
const MINI_ICON_HEIGHT: f32 = 20.0;

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
    /// If set, the context menu initiated a fleet move to this destination.
    pub pending_move_destination: Option<SystemKey>,
    /// Surface regiments selected to embark when a fleet is dispatched.
    pub selected_troops: HashMap<FleetKey, Vec<TroopKey>>,
}

// ---------------------------------------------------------------------------
// draw_fleets
// ---------------------------------------------------------------------------

/// Render the fleet roster as a left-side egui panel.
///
/// `player_faction` filters which fleets are shown. Returns panel actions for
/// character assignment, fleet merging, and map navigation.
#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
#[expect(
    clippy::cast_precision_loss,
    reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
)]
pub fn draw_fleets(
    ctx: &egui::Context,
    world: &GameWorld,
    movement_state: &MovementState,
    troop_transport: &TroopTransportState,
    state: &mut FleetsState,
    player_faction: MissionFaction,
    bmp_cache: &mut BmpCache,
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

            if let Some(destination) = state.pending_move_destination {
                ui.group(|ui| {
                    let destination_name = world
                        .systems
                        .get(destination)
                        .map_or("Unavailable destination", |system| system.name.as_str());
                    ui.label(
                        RichText::new("MOVE FLEET")
                            .color(theme::GOLD_DIM)
                            .size(10.0)
                            .strong(),
                    );
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("Destination: {destination_name}"))
                                .color(theme::TEXT_PRIMARY)
                                .size(11.0),
                        );
                        if ui
                            .small_button(
                                RichText::new("Cancel")
                                    .color(theme::TEXT_DISABLED)
                                    .size(10.0),
                            )
                            .clicked()
                        {
                            state.pending_move_destination = None;
                            state.selected_troops.clear();
                        }
                    });
                    ui.label(
                        RichText::new("Choose an orbiting fleet below.")
                            .color(theme::TEXT_SECONDARY)
                            .size(10.0),
                    );
                });
                ui.add_space(4.0);
            }

            ScrollArea::vertical().show(ui, |ui| {
                for &(fleet_key, fleet) in &player_fleets {
                    let system_name = world
                        .systems
                        .get(fleet.location)
                        .map_or("Unknown", |s| s.name.as_str());

                    let ship_count: u32 = fleet.ship_count();
                    let fighter_count: u32 = fleet.fighters.iter().map(|e| e.count).sum();
                    let is_expanded = state.expanded_fleet == Some(fleet_key);
                    let location_text = movement_state.get(fleet_key).map_or_else(
                        || format!("Fleet @ {system_name}"),
                        |order| {
                            let destination_name = world
                                .systems
                                .get(order.destination)
                                .map_or("Unknown", |system| system.name.as_str());
                            format!(
                                "En route to {} ({} days)",
                                destination_name,
                                order.ticks_remaining(),
                            )
                        },
                    );

                    // ── Fleet header row ─────────────────────────────────
                    ui.horizontal(|ui| {
                        let toggle = if is_expanded { "▼" } else { "▶" };
                        if ui.small_button(toggle).clicked() {
                            state.expanded_fleet = if is_expanded { None } else { Some(fleet_key) };
                            state.assigning_to = None;
                        }

                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new(location_text)
                                    .color(fleet_color(player_faction))
                                    .strong(),
                            );

                            let mut parts = Vec::new();
                            if ship_count > 0 {
                                parts.push(format!("{ship_count} ships"));
                            }
                            if fighter_count > 0 {
                                parts.push(format!("{fighter_count} sqns"));
                            }
                            if fleet.has_death_star {
                                parts.push("DS".to_string());
                            }
                            if !parts.is_empty() {
                                ui.label(
                                    RichText::new(parts.join(" · "))
                                        .size(10.0)
                                        .color(theme::TEXT_SECONDARY),
                                );
                            }
                        });
                    });

                    if let Some(destination) = state.pending_move_destination {
                        let expected_is_alliance = player_faction == MissionFaction::Alliance;
                        match validate_fleet_dispatch(
                            movement_state,
                            world,
                            fleet_key,
                            destination,
                            expected_is_alliance,
                        ) {
                            Ok(()) => {
                                let capacity = TroopTransportState::fleet_capacity(world, fleet_key)
                                    .unwrap_or_default()
                                    as usize;
                                let carried = troop_transport.carried_count(fleet_key);
                                let free_capacity = capacity.saturating_sub(carried);
                                let available =
                                    available_surface_troops(world, fleet_key, player_faction);
                                let available_keys: Vec<_> =
                                    available.iter().map(|(key, _)| *key).collect();
                                let selected = state.selected_troops.entry(fleet_key).or_default();
                                selected.retain(|key| available_keys.contains(key));
                                while selected.len() > free_capacity {
                                    selected.pop();
                                }

                                ui.label(
                                    RichText::new(format!(
                                        "Troop cargo: {}/{} regiments",
                                        carried + selected.len(),
                                        capacity,
                                    ))
                                    .color(theme::TEXT_SECONDARY)
                                    .size(10.0),
                                );
                                for (troop, label) in &available {
                                    let mut checked = selected.contains(troop);
                                    let can_add = checked || selected.len() < free_capacity;
                                    ui.add_enabled_ui(can_add, |ui| {
                                        if ui.checkbox(&mut checked, label).changed() {
                                            if checked {
                                                selected.push(*troop);
                                                selected.sort_unstable();
                                                selected.dedup();
                                            } else {
                                                selected.retain(|key| key != troop);
                                            }
                                        }
                                    });
                                }
                                if available.is_empty() && carried == 0 {
                                    ui.label(
                                        RichText::new(if capacity == 0 {
                                            "This fleet has no troop capacity"
                                        } else {
                                            "No friendly surface regiments available"
                                        })
                                        .color(theme::TEXT_DISABLED)
                                        .size(10.0),
                                    );
                                }

                                let destination_name = world
                                    .systems
                                    .get(destination)
                                    .map_or("destination", |system| system.name.as_str());
                                if ui
                                    .button(
                                        RichText::new(format!("Dispatch to {destination_name}"))
                                            .color(theme::GOLD)
                                            .size(11.0),
                                    )
                                    .clicked()
                                {
                                    let troops = state
                                        .selected_troops
                                        .remove(&fleet_key)
                                        .unwrap_or_default();
                                    action = Some(PanelAction::DispatchFleet {
                                        fleet: fleet_key,
                                        destination,
                                        troops,
                                    });
                                    state.pending_move_destination = None;
                                    state.selected_troops.clear();
                                }
                            }
                            Err(FleetDispatchError::AlreadyInTransit) => {
                                ui.label(
                                    RichText::new("Already in transit")
                                        .color(theme::TEXT_DISABLED)
                                        .size(10.0),
                                );
                            }
                            Err(FleetDispatchError::AlreadyAtDestination) => {
                                ui.label(
                                    RichText::new("Already at destination")
                                        .color(theme::TEXT_DISABLED)
                                        .size(10.0),
                                );
                            }
                            Err(FleetDispatchError::EmptyFleet) => {
                                ui.label(
                                    RichText::new("No ships available")
                                        .color(theme::TEXT_DISABLED)
                                        .size(10.0),
                                );
                            }
                            Err(_) => {}
                        }
                    }

                    if is_expanded {
                        ui.indent("fleet_detail", |ui| {
                            // ── Capital ships ────────────────────────────
                            if fleet.ship_count() > 0 {
                                ui.label(
                                    RichText::new("CAPITAL SHIPS")
                                        .color(theme::GOLD_DIM)
                                        .size(10.0)
                                        .strong(),
                                );
                                for (class_key, count) in fleet.ship_counts_by_class() {
                                    let (class_name, dat_id) = world
                                        .capital_ship_classes
                                        .get(class_key)
                                        .map_or(("Unknown", DatId::new(0)), |c| {
                                            (c.name.as_str(), c.dat_id)
                                        });
                                    ui.horizontal(|ui| {
                                        // GOKRES.DLL 61x25 mini-icon for this ship class.
                                        if let Some(mini_id) = capital_ship_mini_id(dat_id) {
                                            if let Some(tex) =
                                                bmp_cache.get(ctx, DllSource::Gokres, mini_id)
                                            {
                                                let size = tex.size();
                                                let h = MINI_ICON_HEIGHT;
                                                let w = h * size[0] as f32 / size[1] as f32;
                                                ui.add(egui::Image::new(
                                                    egui::load::SizedTexture::new(
                                                        tex.id(),
                                                        Vec2::new(w, h),
                                                    ),
                                                ));
                                            }
                                        }
                                        ui.label(
                                            RichText::new(format!("{class_name} ×{count}"))
                                                .color(theme::TEXT_PRIMARY)
                                                .size(11.0),
                                        );
                                    });
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
                                    let (class_name, dat_id) = world
                                        .fighter_classes
                                        .get(entry.class)
                                        .map_or(("Unknown", DatId::new(0)), |c| {
                                            (c.name.as_str(), c.dat_id)
                                        });
                                    ui.horizontal(|ui| {
                                        // GOKRES.DLL 61x25 mini-icon for this fighter class.
                                        if let Some(mini_id) = fighter_mini_id(dat_id) {
                                            if let Some(tex) =
                                                bmp_cache.get(ctx, DllSource::Gokres, mini_id)
                                            {
                                                let size = tex.size();
                                                let h = MINI_ICON_HEIGHT;
                                                let w = h * size[0] as f32 / size[1] as f32;
                                                ui.add(egui::Image::new(
                                                    egui::load::SizedTexture::new(
                                                        tex.id(),
                                                        Vec2::new(w, h),
                                                    ),
                                                ));
                                            }
                                        }
                                        ui.label(
                                            RichText::new(format!(
                                                "{} ×{}",
                                                class_name, entry.count
                                            ))
                                            .color(theme::TEXT_PRIMARY)
                                            .size(11.0),
                                        );
                                    });
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

                            let cargo = troop_transport.cargo(fleet_key);
                            let capacity = TroopTransportState::fleet_capacity(world, fleet_key)
                                .unwrap_or_default();
                            ui.add_space(2.0);
                            ui.label(
                                RichText::new(format!("TROOP CARGO  {}/{}", cargo.len(), capacity))
                                    .color(theme::GOLD_DIM)
                                    .size(10.0)
                                    .strong(),
                            );

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
                                    .map_or("Unknown", |c| c.name.as_str());
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(format!("  {char_name}"))
                                            .color(theme::TEXT_PRIMARY)
                                            .size(11.0),
                                    );
                                    // Remove from fleet button
                                    if ui
                                        .small_button(
                                            RichText::new("×").color(theme::DANGER_RED).size(11.0),
                                        )
                                        .clicked()
                                    {
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
                                let available =
                                    available_characters(world, fleet_key, player_faction);
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
                                                if ui
                                                    .button(
                                                        RichText::new(name.as_str())
                                                            .color(theme::TEXT_PRIMARY)
                                                            .size(11.0),
                                                    )
                                                    .clicked()
                                                {
                                                    action =
                                                        Some(PanelAction::AssignCharacterToFleet {
                                                            character: *ck,
                                                            fleet: fleet_key,
                                                        });
                                                    state.assigning_to = None;
                                                }
                                            }
                                        });
                                }
                                if ui
                                    .small_button(
                                        RichText::new("Cancel")
                                            .color(theme::TEXT_DISABLED)
                                            .size(10.0),
                                    )
                                    .clicked()
                                {
                                    state.assigning_to = None;
                                }
                            } else if ui
                                .button(
                                    RichText::new("Assign Officer")
                                        .color(theme::GOLD)
                                        .size(11.0),
                                )
                                .clicked()
                            {
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
                                        let sc: u32 = f.ship_count();
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
                                    if ui
                                        .button(
                                            RichText::new(format!(
                                                "Merge with fleet ({other_ships} ships)"
                                            ))
                                            .color(theme::TEXT_PRIMARY)
                                            .size(11.0),
                                        )
                                        .clicked()
                                    {
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
                            if ui
                                .button(RichText::new("Go to System").color(theme::GOLD).size(11.0))
                                .clicked()
                            {
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

fn available_surface_troops(
    world: &GameWorld,
    fleet_key: FleetKey,
    player_faction: MissionFaction,
) -> Vec<(TroopKey, String)> {
    let Some(fleet) = world.fleets.get(fleet_key) else {
        return Vec::new();
    };
    let Some(system) = world.systems.get(fleet.location) else {
        return Vec::new();
    };
    let expected_is_alliance = player_faction == MissionFaction::Alliance;
    let mut troops: Vec<_> = system
        .ground_units
        .iter()
        .filter_map(|key| {
            let troop = world.troops.get(*key)?;
            (troop.is_alliance == expected_is_alliance && troop.regiment_strength > 0).then(|| {
                (
                    *key,
                    format!(
                        "Regiment {} · strength {}",
                        troop.class_dat_id.index(),
                        troop.regiment_strength,
                    ),
                )
            })
        })
        .collect();
    troops.sort_unstable_by_key(|(key, _)| *key);
    troops
}

pub(crate) fn capital_ship_mini_id(dat_id: DatId) -> Option<u32> {
    match dat_id.index() {
        64..=78 => ALLIANCE_CAPITAL_SHIP_MINIS
            .get((dat_id.index() - 64) as usize)
            .copied(),
        128..=142 => EMPIRE_CAPITAL_SHIP_MINIS
            .get((dat_id.index() - 128) as usize)
            .copied(),
        _ => None,
    }
}

pub(crate) fn fighter_mini_id(dat_id: DatId) -> Option<u32> {
    match dat_id.index() {
        1..=4 => ALLIANCE_FIGHTER_MINIS
            .get((dat_id.index() - 1) as usize)
            .copied(),
        5..=8 => EMPIRE_FIGHTER_MINIS
            .get((dat_id.index() - 5) as usize)
            .copied(),
        _ => None,
    }
}

/// Find characters eligible for fleet assignment:
/// same faction, not captive, not on mission, not already in this fleet.
fn available_characters(
    world: &GameWorld,
    fleet_key: FleetKey,
    player_faction: MissionFaction,
) -> Vec<(CharacterKey, String)> {
    let Some(fleet) = world.fleets.get(fleet_key) else {
        return vec![];
    };

    let mut result = Vec::new();
    for (ck, c) in &world.characters {
        // Faction filter
        let owns = match player_faction {
            MissionFaction::Alliance => c.is_alliance,
            MissionFaction::Empire => c.is_empire,
        };
        if !owns {
            continue;
        }
        if c.is_captive || c.on_mission || c.on_mandatory_mission {
            continue;
        }
        // Not already in this fleet
        if fleet.characters.contains(&ck) {
            continue;
        }
        // Not already in another fleet
        let in_another = world.fleets.values().any(|f| f.characters.contains(&ck));
        if in_another {
            continue;
        }
        result.push((ck, c.name.clone()));
    }
    result.sort_by(|a, b| a.1.cmp(&b.1));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
    )]
    fn maps_every_capital_ship_dat_record_to_its_gokres_miniature() {
        for (offset, &resource_id) in ALLIANCE_CAPITAL_SHIP_MINIS.iter().enumerate() {
            assert_eq!(
                capital_ship_mini_id(DatId::new(0x1400_0040 + offset as u32)),
                Some(resource_id)
            );
        }
        for (offset, &resource_id) in EMPIRE_CAPITAL_SHIP_MINIS.iter().enumerate() {
            assert_eq!(
                capital_ship_mini_id(DatId::new(0x1400_0080 + offset as u32)),
                Some(resource_id)
            );
        }
    }

    #[test]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
    )]
    fn maps_every_fighter_dat_record_to_its_gokres_miniature() {
        for (offset, &resource_id) in ALLIANCE_FIGHTER_MINIS.iter().enumerate() {
            assert_eq!(
                fighter_mini_id(DatId::new(0x1c00_0001 + offset as u32)),
                Some(resource_id)
            );
        }
        for (offset, &resource_id) in EMPIRE_FIGHTER_MINIS.iter().enumerate() {
            assert_eq!(
                fighter_mini_id(DatId::new(0x1c00_0005 + offset as u32)),
                Some(resource_id)
            );
        }
    }

    #[test]
    fn unknown_class_ids_do_not_request_unrelated_bitmaps() {
        assert_eq!(capital_ship_mini_id(DatId::new(0)), None);
        assert_eq!(capital_ship_mini_id(DatId::new(0x1400_003f)), None);
        assert_eq!(capital_ship_mini_id(DatId::new(0x1400_008f)), None);
        assert_eq!(fighter_mini_id(DatId::new(0)), None);
        assert_eq!(fighter_mini_id(DatId::new(0x1c00_0009)), None);
    }
}
