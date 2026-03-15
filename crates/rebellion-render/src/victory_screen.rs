//! Victory and defeat screen — egui modal overlay.
//!
//! Renders a full-screen-centered modal when the game ends.  The modal
//! blocks all other interaction until the player dismisses it.
//!
//! # Outcomes handled
//! - `VictoryOutcome::HqCaptured` — shows which faction won and which HQ
//!   was taken.
//! - `VictoryOutcome::DeathStarVictory` — Empire victory fanfare with
//!   the name of the destroyed system.
//! - `VictoryOutcome::DeathStarDestroyed` — Alliance victory: Rebels
//!   destroyed the Death Star.
//!
//! # Integration
//! ```ignore
//! // In main.rs — inside the egui_macroquad::ui closure:
//! if let dismissed = draw_victory_screen(ctx, &mut victory_screen_state) {
//!     // player acknowledged — do whatever (return to menu, etc.)
//! }
//! ```
//!
//! # Source
//! - `entity-system.md §4.2` — `SideVictoryConditionsNotif`, VICTORY / VICTORY_II audio IDs
//! - `victory.rs` — `VictoryOutcome` enum

use egui_macroquad::egui::{self, Color32, RichText};
use rebellion_core::victory::VictoryOutcome;

// ---------------------------------------------------------------------------
// VictoryScreenState
// ---------------------------------------------------------------------------

/// Game statistics captured at the moment of game-end, shown in the victory modal.
#[derive(Debug, Clone, Default)]
pub struct GameStats {
    /// Total game-days elapsed when the game ended.
    pub days_played: u64,
    /// Number of space or ground battles won by the player's faction.
    pub battles_won: u32,
    /// Total ships built by the player's faction over the campaign.
    pub ships_built: u32,
}

/// UI state for the end-game victory / defeat modal.
///
/// Set `outcome` to `Some(VictoryOutcome)` when the game ends.  The modal
/// will remain visible until the player clicks "Continue" or "Play Again".
#[derive(Debug, Default)]
pub struct VictoryScreenState {
    /// The terminal outcome to display.  `None` → modal is hidden.
    pub outcome: Option<VictoryOutcome>,
    /// Optional stats to display alongside the outcome.
    pub stats: Option<GameStats>,
    /// Set to `true` by `draw_victory_screen` when the player dismisses it.
    pub acknowledged: bool,
    /// Set to `true` when the player clicks "Play Again" (vs "Continue").
    pub replay_requested: bool,
}

impl VictoryScreenState {
    pub fn new() -> Self {
        VictoryScreenState::default()
    }

    /// Returns `true` if there is an outcome waiting to be displayed.
    pub fn is_pending(&self) -> bool {
        self.outcome.is_some() && !self.acknowledged
    }

    /// Reset for a new game (called when replay is confirmed).
    pub fn reset(&mut self) {
        *self = VictoryScreenState::default();
    }
}

// ---------------------------------------------------------------------------
// draw_victory_screen
// ---------------------------------------------------------------------------

/// Draw the victory/defeat modal (egui window).
///
/// Displays nothing and returns `false` when no outcome is pending.
///
/// Returns `true` the frame the player clicks "Continue" (i.e., the modal
/// was just dismissed).  The caller can use this to trigger a return-to-menu
/// transition or score screen.
///
/// Call inside the `egui_macroquad::ui` closure.
pub fn draw_victory_screen(
    ctx: &egui::Context,
    state: &mut VictoryScreenState,
) -> bool {
    if !state.is_pending() {
        return false;
    }

    let outcome = match &state.outcome {
        Some(o) => o.clone(),
        None => return false,
    };

    let (title, subtitle, body_lines, title_color) = describe_outcome(&outcome);

    let mut dismissed = false;

    // Dim the background to make the modal feel modal.
    ctx.layer_painter(egui::LayerId::new(
        egui::Order::Foreground,
        egui::Id::new("victory_dim"),
    ))
    .rect_filled(
        ctx.screen_rect(),
        0.0,
        Color32::from_black_alpha(160),
    );

    egui::Window::new(&title)
        .collapsible(false)
        .resizable(false)
        .default_width(400.0)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // ── Title / faction badge ──────────────────────────────
                ui.add_space(8.0);
                ui.label(
                    RichText::new(&title)
                        .color(title_color)
                        .size(28.0)
                        .strong(),
                );
                ui.add_space(4.0);
                ui.label(
                    RichText::new(&subtitle)
                        .color(Color32::LIGHT_GRAY)
                        .size(16.0)
                        .italics(),
                );
                ui.separator();
                ui.add_space(8.0);

                // ── Body lines ────────────────────────────────────────
                for line in &body_lines {
                    ui.label(RichText::new(line).size(14.0).color(Color32::WHITE));
                }

                // ── Game stats ────────────────────────────────────────
                if let Some(ref stats) = state.stats {
                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(8.0);
                    ui.label(RichText::new("Campaign Statistics").size(13.0).color(Color32::GRAY));
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(format!("Days played:   {}", stats.days_played))
                            .size(13.0)
                            .color(Color32::LIGHT_GRAY),
                    );
                    ui.label(
                        RichText::new(format!("Battles won:   {}", stats.battles_won))
                            .size(13.0)
                            .color(Color32::LIGHT_GRAY),
                    );
                    ui.label(
                        RichText::new(format!("Ships built:   {}", stats.ships_built))
                            .size(13.0)
                            .color(Color32::LIGHT_GRAY),
                    );
                }

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(8.0);

                // ── Buttons ───────────────────────────────────────────
                ui.horizontal(|ui| {
                    if ui
                        .add(egui::Button::new(
                            RichText::new("Play Again").size(15.0).strong(),
                        ))
                        .clicked()
                    {
                        state.replay_requested = true;
                        dismissed = true;
                    }
                    ui.add_space(8.0);
                    if ui
                        .add(egui::Button::new(
                            RichText::new("Continue").size(15.0),
                        ))
                        .clicked()
                    {
                        dismissed = true;
                    }
                });
                ui.add_space(8.0);
            });
        });

    if dismissed {
        state.acknowledged = true;
    }

    dismissed
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Returns `(title, subtitle, body_lines, title_color)` for a given outcome.
fn describe_outcome(
    outcome: &VictoryOutcome,
) -> (String, String, Vec<String>, Color32) {
    match outcome {
        VictoryOutcome::HqCaptured { winner, loser, .. } => {
            let winner_name = faction_name(*winner);
            let loser_name = faction_name(*loser);
            let is_alliance_win = matches!(winner, rebellion_core::dat::Faction::Alliance);
            let color = if is_alliance_win {
                Color32::from_rgb(100, 200, 255) // Alliance blue
            } else {
                Color32::from_rgb(220, 60, 60) // Empire red
            };
            (
                format!("{} Victory!", winner_name),
                "Headquarters Captured".into(),
                vec![
                    format!("The {} has captured the {} headquarters.", winner_name, loser_name),
                    String::new(),
                    format!("The {} has been defeated.", loser_name),
                ],
                color,
            )
        }

        VictoryOutcome::DeathStarVictory { .. } => (
            "Empire Victory!".into(),
            "Death Star Fires".into(),
            vec![
                "The Death Star has annihilated the Rebel base.".into(),
                String::new(),
                "The Rebellion has been crushed.".into(),
                "The Emperor's reign is absolute.".into(),
            ],
            Color32::from_rgb(220, 60, 60),
        ),

        VictoryOutcome::DeathStarDestroyed { .. } => (
            "Alliance Victory!".into(),
            "Death Star Destroyed".into(),
            vec![
                "Rebel pilots destroyed the Death Star!".into(),
                String::new(),
                "The Empire's ultimate weapon is no more.".into(),
                "Hope is restored to the galaxy.".into(),
            ],
            Color32::from_rgb(100, 200, 255),
        ),
    }
}

fn faction_name(faction: rebellion_core::dat::Faction) -> &'static str {
    match faction {
        rebellion_core::dat::Faction::Alliance => "Rebel Alliance",
        rebellion_core::dat::Faction::Empire => "Galactic Empire",
        rebellion_core::dat::Faction::Neutral => "Neutral Forces",
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::victory::VictoryOutcome;
    use rebellion_core::dat::Faction;
    use rebellion_core::world::GameWorld;

    fn make_system_key() -> rebellion_core::ids::SystemKey {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(rebellion_core::world::Sector {
            dat_id: rebellion_core::ids::DatId::new(0),
            name: "S".into(),
            group: rebellion_core::dat::SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        world.systems.insert(rebellion_core::world::System {
            dat_id: rebellion_core::ids::DatId::new(0),
            name: "Test".into(),
            sector,
            x: 0,
            y: 0,
            exploration_status: rebellion_core::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            controlling_faction: None,
        })
    }

    #[test]
    fn state_is_pending_with_outcome() {
        let key = make_system_key();
        let mut state = VictoryScreenState::new();
        assert!(!state.is_pending());
        state.outcome = Some(VictoryOutcome::DeathStarDestroyed { location: key });
        assert!(state.is_pending());
    }

    #[test]
    fn state_not_pending_when_acknowledged() {
        let key = make_system_key();
        let mut state = VictoryScreenState::new();
        state.outcome = Some(VictoryOutcome::DeathStarDestroyed { location: key });
        state.acknowledged = true;
        assert!(!state.is_pending());
    }

    #[test]
    fn state_reset_clears_all_fields() {
        let key = make_system_key();
        let mut state = VictoryScreenState::new();
        state.outcome = Some(VictoryOutcome::DeathStarDestroyed { location: key });
        state.acknowledged = true;
        state.replay_requested = true;
        state.stats = Some(GameStats { days_played: 100, battles_won: 10, ships_built: 5 });
        state.reset();
        assert!(state.outcome.is_none());
        assert!(!state.acknowledged);
        assert!(!state.replay_requested);
        assert!(state.stats.is_none());
    }

    #[test]
    fn game_stats_default() {
        let stats = GameStats::default();
        assert_eq!(stats.days_played, 0);
        assert_eq!(stats.battles_won, 0);
        assert_eq!(stats.ships_built, 0);
    }

    #[test]
    fn describe_hq_captured_empire_wins() {
        let key = make_system_key();
        let outcome = VictoryOutcome::HqCaptured {
            winner: Faction::Empire,
            loser: Faction::Alliance,
            hq_system: key,
        };
        let (title, subtitle, body, color) = describe_outcome(&outcome);
        assert!(title.contains("Empire"), "title should name winner");
        assert!(subtitle.contains("Captured"));
        assert!(body.iter().any(|l| l.contains("Rebel Alliance") || l.contains("headquarters")));
        assert_eq!(color, Color32::from_rgb(220, 60, 60));
    }

    #[test]
    fn describe_hq_captured_alliance_wins() {
        let key = make_system_key();
        let outcome = VictoryOutcome::HqCaptured {
            winner: Faction::Alliance,
            loser: Faction::Empire,
            hq_system: key,
        };
        let (title, _, _, color) = describe_outcome(&outcome);
        assert!(title.contains("Alliance"));
        assert_eq!(color, Color32::from_rgb(100, 200, 255));
    }

    #[test]
    fn describe_death_star_victory() {
        let key = make_system_key();
        let outcome = VictoryOutcome::DeathStarVictory { target_system: key };
        let (title, subtitle, body, _) = describe_outcome(&outcome);
        assert!(title.contains("Empire"));
        assert!(subtitle.contains("Death Star"));
        assert!(body.iter().any(|l| l.contains("Death Star") || l.contains("Rebel")));
    }

    #[test]
    fn describe_death_star_destroyed() {
        let key = make_system_key();
        let outcome = VictoryOutcome::DeathStarDestroyed { location: key };
        let (title, _, body, _) = describe_outcome(&outcome);
        assert!(title.contains("Alliance"));
        assert!(body.iter().any(|l| l.contains("Death Star")));
    }
}
