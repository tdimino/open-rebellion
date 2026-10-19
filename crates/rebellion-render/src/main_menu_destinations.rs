//! The screen the original cockpit's Multiplayer control reaches.

use egui_macroquad::egui::{self, Color32, RichText};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerTransport {
    Lan,
    Modem,
    Internet,
}

impl MultiplayerTransport {
    #[must_use]
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
    #[must_use]
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
    fn multiplayer_failure_names_selected_transport() {
        let mut state = MultiplayerSetupState::default();
        assert_eq!(state.transport, MultiplayerTransport::Lan);
        state.transport = MultiplayerTransport::Internet;
        assert!(state.unavailable_message().starts_with("Internet sessions"));
    }
}
