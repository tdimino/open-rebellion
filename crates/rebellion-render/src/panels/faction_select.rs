//! Faction selection panel — startup screen that gates all other War Room panels.
//!
//! Shown once before game-play begins.  The player chooses Alliance or Empire;
//! the choice is communicated back via `PanelAction::SelectFaction`.  After
//! selection the panel becomes a no-op (callers should stop calling it once
//! `state.chosen` is set).

use egui_macroquad::egui::{self, Color32, RichText};
use rebellion_core::missions::MissionFaction;

use super::PanelAction;

// ---------------------------------------------------------------------------
// FactionSelectState
// ---------------------------------------------------------------------------

/// Mutable state for the faction selection panel.
#[derive(Debug, Clone, Default)]
pub struct FactionSelectState {
    /// Set after the player confirms their choice.  The caller uses this to
    /// stop rendering the panel.
    pub chosen: Option<MissionFaction>,
}

// ---------------------------------------------------------------------------
// draw_faction_select
// ---------------------------------------------------------------------------

/// Render the faction selection screen as a full-screen modal window.
///
/// Returns `Some(PanelAction::SelectFaction(_))` the frame the player confirms,
/// `None` every other frame.
pub fn draw_faction_select(
    ctx: &egui::Context,
    state: &mut FactionSelectState,
) -> Option<PanelAction> {
    if state.chosen.is_some() {
        return None;
    }

    let mut action = None;

    egui::Window::new("Choose Your Side")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .min_width(360.0)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(8.0);

                ui.label(
                    RichText::new("A galaxy divided.")
                        .color(Color32::from_gray(200))
                        .size(14.0),
                );
                ui.label(
                    RichText::new("Choose the faction you will command.")
                        .color(Color32::from_gray(150))
                        .size(12.0),
                );

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(12.0);

                // ── Alliance button ───────────────────────────────────────────
                let alliance_resp = ui.add_sized(
                    [300.0, 64.0],
                    egui::Button::new(
                        RichText::new("Rebel Alliance")
                            .color(Color32::from_rgb(100, 160, 255))
                            .size(16.0)
                            .strong(),
                    ),
                );
                ui.label(
                    RichText::new("Fight for freedom across the Outer Rim.")
                        .color(Color32::from_gray(140))
                        .size(11.0),
                );

                ui.add_space(8.0);

                // ── Empire button ─────────────────────────────────────────────
                let empire_resp = ui.add_sized(
                    [300.0, 64.0],
                    egui::Button::new(
                        RichText::new("Galactic Empire")
                            .color(Color32::from_rgb(220, 80, 80))
                            .size(16.0)
                            .strong(),
                    ),
                );
                ui.label(
                    RichText::new("Crush the Rebellion and restore order.")
                        .color(Color32::from_gray(140))
                        .size(11.0),
                );

                ui.add_space(12.0);

                if alliance_resp.clicked() {
                    state.chosen = Some(MissionFaction::Alliance);
                    action = Some(PanelAction::SelectFaction(MissionFaction::Alliance));
                }
                if empire_resp.clicked() {
                    state.chosen = Some(MissionFaction::Empire);
                    action = Some(PanelAction::SelectFaction(MissionFaction::Empire));
                }
            });
        });

    action
}
