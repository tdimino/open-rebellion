//! Main menu screen — title screen with New Game / Load Game / Quit buttons.
//!
//! Rendered as a full-screen egui layout over the space background.
//! Returns `MainMenuAction` to tell the caller what the player chose.

use egui_macroquad::egui::{self, Align, Color32, Layout, RichText};

use crate::theme;

/// Actions the main menu can produce.
#[derive(Debug, Clone, PartialEq)]
pub enum MainMenuAction {
    /// Player wants to start a new game — transition to GameSetup.
    NewGame,
    /// Player wants to load a saved game — transition to Load screen.
    LoadGame,
    /// Player wants to quit the application.
    Quit,
}

/// Render the main menu screen. Returns an action when the player clicks a button.
pub fn draw_main_menu(ctx: &egui::Context) -> Option<MainMenuAction> {
    let mut action = None;

    egui::CentralPanel::default()
        .frame(egui::Frame::default().fill(theme::BG_SPACE))
        .show(ctx, |ui| {
            let available = ui.available_size();

            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                // Push title down ~30% from top
                ui.add_space(available.y * 0.22);

                // ── Title ────────────────────────────────────────────────
                ui.label(
                    RichText::new("STAR WARS")
                        .color(theme::GOLD)
                        .size(18.0)
                        .strong(),
                );
                ui.add_space(4.0);
                ui.label(
                    RichText::new("REBELLION")
                        .color(theme::GOLD_BRIGHT)
                        .size(42.0)
                        .strong(),
                );
                ui.add_space(2.0);
                ui.label(
                    RichText::new("Open Source Reimplementation")
                        .color(theme::TEXT_SECONDARY)
                        .size(12.0),
                );

                ui.add_space(available.y * 0.08);

                // ── Menu buttons ─────────────────────────────────────────
                let button_width = 260.0;
                let button_height = 48.0;

                if menu_button(ui, "NEW GAME", button_width, button_height).clicked() {
                    action = Some(MainMenuAction::NewGame);
                }
                ui.add_space(8.0);

                if menu_button(ui, "LOAD GAME", button_width, button_height).clicked() {
                    action = Some(MainMenuAction::LoadGame);
                }
                ui.add_space(8.0);

                if menu_button(ui, "QUIT", button_width, button_height).clicked() {
                    action = Some(MainMenuAction::Quit);
                }

                // ── Version / credits at bottom ──────────────────────────
                ui.add_space(available.y * 0.12);
                ui.label(
                    RichText::new("v0.15.0 — github.com/tdimino/open-rebellion")
                        .color(Color32::from_gray(60))
                        .size(10.0),
                );
            });
        });

    action
}

/// Styled menu button with gold border on hover.
fn menu_button(
    ui: &mut egui::Ui,
    label: &str,
    width: f32,
    height: f32,
) -> egui::Response {
    ui.add_sized(
        [width, height],
        egui::Button::new(
            RichText::new(label)
                .color(theme::TEXT_PRIMARY)
                .size(16.0)
                .strong(),
        )
        .fill(Color32::from_rgb(20, 22, 36))
        .stroke(egui::Stroke::new(1.0, theme::GOLD_DIM)),
    )
}
