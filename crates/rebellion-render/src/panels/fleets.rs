//! Fleets panel — fleet roster with composition and officers.
//!
//! Rendered as a left-side egui panel. Lists all fleets belonging to the player
//! faction. Clicking a fleet expands a detail row showing capital ships, fighter
//! squadrons, the characters aboard, and a go-to-system button. Characters
//! board a fleet through their Move (`0x241`). Fleets move through their pop-up menu and the system
//! window drag, and join or split on the Fleet window
//! (`crate::fleet_window`).

use egui_macroquad::egui::{self, RichText, ScrollArea, Vec2};
use rebellion_core::ids::{DatId, FleetKey};
use rebellion_core::missions::MissionFaction;
use rebellion_core::movement::MovementState;
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
}

// ---------------------------------------------------------------------------
// draw_fleets
// ---------------------------------------------------------------------------

/// Render the fleet roster as a left-side egui panel.
///
/// `player_faction` filters which fleets are shown. Returns the map
/// navigation action.
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
                                    .map_or("Unknown", |c| c.display_name());
                                ui.label(
                                    RichText::new(format!("  {char_name}"))
                                        .color(theme::TEXT_PRIMARY)
                                        .size(11.0),
                                );
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
    )]
    fn maps_every_capital_ship_dat_record_to_its_gokres_miniature() {
        // Source: GOKRES.DLL mini-icon blocks following CAPSHPSD.DAT record order.
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
        // Source: GOKRES.DLL mini-icon blocks following FIGHTSD.DAT record order.
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
