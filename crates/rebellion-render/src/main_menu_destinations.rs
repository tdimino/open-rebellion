//! Screens reached by the original cockpit's Credits and Multiplayer controls.

use egui_macroquad::egui::{self, Align2, Color32, FontFamily, FontId, Pos2, Rect, RichText, Vec2};

const CREDIT_LINE_HEIGHT: f32 = 27.0;
const CREDIT_SCROLL_SPEED: f32 = 24.0;

const CREDIT_LINES: &[&str] = &[
    "STAR WARS REBELLION",
    "",
    "Originally developed by Coolhand Interactive",
    "Published by LucasArts",
    "",
    "Music based on themes by John Williams",
    "",
    "OPEN REBELLION",
    "A from-scratch, open-source reimplementation",
    "",
    "Reverse engineering, simulation, rendering, and preservation",
    "by the Open Rebellion contributors",
    "",
    "Original game data remains the property of its rights holders.",
    "A legal copy of Star Wars Rebellion is required.",
    "",
    "May the Force be with you.",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuDestinationAction {
    Back,
}

/// Animation state for the scrolling credits sequence.
#[derive(Debug, Clone)]
pub struct CreditsState {
    scroll_y: f32,
    initialized: bool,
}

impl Default for CreditsState {
    fn default() -> Self {
        Self {
            scroll_y: 0.0,
            initialized: false,
        }
    }
}

impl CreditsState {
    pub fn reset(&mut self) {
        self.scroll_y = 0.0;
        self.initialized = false;
    }

    #[expect(
        clippy::manual_clamp,
        reason = "min/max map NaN to the lower bound; clamp would propagate NaN."
    )]
    fn advance(&mut self, dt: f32, viewport_height: f32) {
        if !self.initialized {
            self.scroll_y = viewport_height * 0.25;
            self.initialized = true;
        }
        self.scroll_y -= dt.max(0.0).min(0.1) * CREDIT_SCROLL_SPEED;
        let content_height = CREDIT_LINES.len() as f32 * CREDIT_LINE_HEIGHT;
        if self.scroll_y + content_height < 0.0 {
            self.scroll_y = viewport_height * 0.25;
        }
    }
}

/// Draw the scrolling credits destination. Escape is handled by the app's
/// shared screen-transition logic; the Back button is keyboard reachable.
pub fn draw_credits(
    ctx: &egui::Context,
    state: &mut CreditsState,
) -> Option<MenuDestinationAction> {
    let mut action = None;
    egui::CentralPanel::default()
        .frame(egui::Frame::default().fill(Color32::BLACK))
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            let dt = ctx.input(|input| input.unstable_dt);
            state.advance(dt, rect.height());
            ctx.request_repaint();

            for (index, line) in CREDIT_LINES.iter().enumerate() {
                let y = rect.top() + state.scroll_y + index as f32 * CREDIT_LINE_HEIGHT;
                if y < rect.top() - CREDIT_LINE_HEIGHT || y > rect.bottom() {
                    continue;
                }
                let title = index == 0 || *line == "OPEN REBELLION";
                ui.painter().text(
                    Pos2::new(rect.center().x, y),
                    Align2::CENTER_CENTER,
                    *line,
                    FontId::new(
                        if title { 24.0 } else { 15.0 },
                        if title {
                            FontFamily::Proportional
                        } else {
                            FontFamily::Monospace
                        },
                    ),
                    if title {
                        Color32::from_rgb(238, 208, 94)
                    } else {
                        Color32::from_rgb(205, 215, 230)
                    },
                );
            }

            let back_rect = Rect::from_center_size(
                Pos2::new(rect.center().x, rect.bottom() - 30.0),
                Vec2::new(120.0, 32.0),
            );
            if ui
                .put(back_rect, egui::Button::new("Back to Main Menu"))
                .clicked()
            {
                action = Some(MenuDestinationAction::Back);
            }
        });
    action
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerTransport {
    Lan,
    Modem,
    Internet,
}

impl MultiplayerTransport {
    pub fn label(self) -> &'static str {
        match self {
            Self::Lan => "LAN",
            Self::Modem => "Modem",
            Self::Internet => "Internet",
        }
    }
}

#[derive(Debug, Clone)]
pub struct MultiplayerSetupState {
    pub player_name: String,
    pub transport: MultiplayerTransport,
    pub status_message: Option<String>,
}

impl Default for MultiplayerSetupState {
    fn default() -> Self {
        Self {
            player_name: "Player".to_string(),
            transport: MultiplayerTransport::Lan,
            status_message: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerSetupAction {
    Back,
    StartRequested,
}

impl MultiplayerSetupState {
    pub fn unavailable_message(&self) -> String {
        format!(
            "{} sessions are not available in this build; authoritative multiplayer is tracked for M4.",
            self.transport.label()
        )
    }
}

/// Draw the original menu's head-to-head setup destination. The historical
/// transport choices remain visible, while the unimplemented networking layer
/// fails explicitly instead of leaving the cockpit control inert.
pub fn draw_multiplayer_setup(
    ctx: &egui::Context,
    state: &mut MultiplayerSetupState,
) -> Option<MultiplayerSetupAction> {
    let mut action = None;
    egui::CentralPanel::default()
        .frame(egui::Frame::default().fill(Color32::from_rgb(4, 8, 18)))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space((ui.available_height() * 0.16).max(24.0));
                ui.heading(
                    RichText::new("HEAD-TO-HEAD SETUP")
                        .size(25.0)
                        .color(Color32::from_rgb(238, 208, 94)),
                );
                ui.add_space(20.0);
                ui.label("Player name");
                ui.add_sized(
                    [260.0, 28.0],
                    egui::TextEdit::singleline(&mut state.player_name)
                        .hint_text("Enter player name"),
                );
                ui.add_space(16.0);
                ui.label("Connection type");
                ui.horizontal(|ui| {
                    for transport in [
                        MultiplayerTransport::Lan,
                        MultiplayerTransport::Modem,
                        MultiplayerTransport::Internet,
                    ] {
                        ui.radio_value(&mut state.transport, transport, transport.label());
                    }
                });
                ui.add_space(18.0);
                ui.label(
                    RichText::new("The original DirectPlay service is retired. Open Rebellion's secure authoritative multiplayer is planned for M4.")
                        .color(Color32::from_rgb(170, 185, 205)),
                );
                if let Some(message) = &state.status_message {
                    ui.add_space(10.0);
                    ui.label(RichText::new(message).color(Color32::from_rgb(255, 190, 90)));
                }
                ui.add_space(20.0);
                ui.horizontal(|ui| {
                    if ui.button("Start Session").clicked() {
                        action = Some(MultiplayerSetupAction::StartRequested);
                    }
                    if ui.button("Back to Main Menu").clicked() {
                        action = Some(MultiplayerSetupAction::Back);
                    }
                });
            });
        });
    action
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn credits_restart_after_last_line_leaves_viewport() {
        let mut state = CreditsState::default();
        state.advance(0.0, 480.0);
        assert_eq!(state.scroll_y, 120.0);
        state.scroll_y = -(CREDIT_LINES.len() as f32 * CREDIT_LINE_HEIGHT) - 1.0;
        state.advance(0.0, 480.0);
        assert_eq!(state.scroll_y, 120.0);
    }

    #[test]
    fn multiplayer_failure_names_selected_transport() {
        let mut state = MultiplayerSetupState::default();
        assert_eq!(state.transport, MultiplayerTransport::Lan);
        state.transport = MultiplayerTransport::Internet;
        assert!(state.unavailable_message().starts_with("Internet sessions"));
    }
}
