//! Combat results display — message log integration + battle summary modal.
//!
//! Two surfaces:
//!
//! 1. **Message log entries** — after each auto-resolve battle, call
//!    `CombatResult::to_messages()` to produce `GameMessage`s for the scrollable
//!    message log. The `Combat` category ensures red color-coding.
//!
//! 2. **Battle summary modal** — `draw_combat_summary` renders an egui window
//!    with per-system combat results. Shown when `CombatSummaryState::pending`
//!    is non-empty; dismissed by the player clicking "Close".
//!
//! # Integration
//!
//! ```ignore
//! // After combat resolution in main.rs:
//! for result in combat_results {
//!     for msg in result.to_messages() {
//!         message_log.push(msg);
//!     }
//!     combat_summary.pending.push(result);
//! }
//!
//! // In the egui_macroquad::ui closure:
//! draw_combat_summary(ctx, &mut combat_summary);
//! ```

use egui_macroquad::egui::{self, Color32, RichText};
use rebellion_core::ids::SystemKey;

use crate::message_log::{GameMessage, MessageCategory};

// ---------------------------------------------------------------------------
// CombatResult
// ---------------------------------------------------------------------------

/// The outcome of one auto-resolved combat engagement at a star system.
///
/// Produced by the combat system (stub for now) and consumed by the render
/// layer for display. The render layer never mutates game state — it only
/// reads this struct.
#[derive(Debug, Clone)]
pub struct CombatResult {
    /// The system where the battle took place.
    pub system: SystemKey,
    /// Human-readable system name (captured at result creation to avoid
    /// world borrow in the render closure).
    pub system_name: String,
    /// Game tick when the battle resolved.
    pub tick: u64,
    /// High-level outcome.
    pub outcome: BattleOutcome,
    /// Ships the attacker lost (class names + counts).
    pub attacker_losses: Vec<(String, u32)>,
    /// Ships the defender lost (class names + counts).
    pub defender_losses: Vec<(String, u32)>,
    /// Whether bombardment occurred after the space battle.
    pub bombardment_occurred: bool,
    /// Optional narrative summary line (e.g., "Rebel fleet routed!").
    pub summary_text: Option<String>,
}

/// The high-level result of a combat engagement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleOutcome {
    /// The attacker won — defender fled or was destroyed.
    AttackerWon,
    /// The defender held — attacker was destroyed or withdrew.
    DefenderHeld,
    /// Neither side destroyed the other; both remain (stalemate / mutual
    /// withdrawal). Used when both fleets survive the auto-resolve round.
    Stalemate,
}

impl BattleOutcome {
    pub fn label(self) -> &'static str {
        match self {
            BattleOutcome::AttackerWon => "Attacker Won",
            BattleOutcome::DefenderHeld => "Defender Held",
            BattleOutcome::Stalemate => "Stalemate",
        }
    }

    pub fn color(self) -> Color32 {
        match self {
            BattleOutcome::AttackerWon => Color32::from_rgb(255, 120, 60),  // orange-red
            BattleOutcome::DefenderHeld => Color32::from_rgb(100, 220, 100), // green
            BattleOutcome::Stalemate => Color32::from_rgb(200, 200, 80),    // yellow
        }
    }
}

impl CombatResult {
    /// Convert this combat result into one or more `GameMessage`s for the
    /// scrollable message log.
    ///
    /// Always produces at least one message (the outcome line). Produces
    /// additional messages for casualty details and bombardment.
    pub fn to_messages(&self) -> Vec<GameMessage> {
        let mut msgs = Vec::new();

        // Primary outcome line
        let outcome_text = format!(
            "Battle at {}: {}",
            self.system_name,
            self.outcome.label()
        );
        msgs.push(GameMessage {
            text: outcome_text,
            tick: self.tick,
            category: MessageCategory::Combat,
            system: Some(self.system),
            system_name: None,
        });

        // Attacker losses
        if !self.attacker_losses.is_empty() {
            let losses_text = format!(
                "  Attacker losses: {}",
                format_losses(&self.attacker_losses)
            );
            msgs.push(GameMessage {
                text: losses_text,
                tick: self.tick,
                category: MessageCategory::Combat,
                system: Some(self.system),
                system_name: None,
            });
        }

        // Defender losses
        if !self.defender_losses.is_empty() {
            let losses_text = format!(
                "  Defender losses: {}",
                format_losses(&self.defender_losses)
            );
            msgs.push(GameMessage {
                text: losses_text,
                tick: self.tick,
                category: MessageCategory::Combat,
                system: Some(self.system),
                system_name: None,
            });
        }

        // Bombardment note
        if self.bombardment_occurred {
            msgs.push(GameMessage {
                text: format!("  Orbital bombardment at {}.", self.system_name),
                tick: self.tick,
                category: MessageCategory::Combat,
                system: Some(self.system),
                system_name: None,
            });
        }

        // Optional narrative summary
        if let Some(ref summary) = self.summary_text {
            msgs.push(GameMessage {
                text: format!("  {summary}"),
                tick: self.tick,
                category: MessageCategory::Combat,
                system: Some(self.system),
                system_name: None,
            });
        }

        msgs
    }
}

// ---------------------------------------------------------------------------
// CombatSummaryState
// ---------------------------------------------------------------------------

/// UI state for the battle summary modal.
///
/// `pending` holds combat results waiting to be displayed. The modal pops
/// results from the front of the queue; the player dismisses each one.
#[derive(Debug, Default)]
pub struct CombatSummaryState {
    /// Results queued for display. Front = next to show.
    pub pending: Vec<CombatResult>,
    /// Index of the currently displayed result in `pending`.
    pub current_index: usize,
}

impl CombatSummaryState {
    pub fn new() -> Self {
        CombatSummaryState {
            pending: Vec::new(),
            current_index: 0,
        }
    }

    /// Returns `true` if there are unacknowledged results to display.
    pub fn has_pending(&self) -> bool {
        !self.pending.is_empty()
    }

    /// Push a new combat result to be shown.
    pub fn push(&mut self, result: CombatResult) {
        self.pending.push(result);
    }

    /// Dismiss the current result and advance to the next.
    fn dismiss_current(&mut self) {
        if !self.pending.is_empty() {
            self.pending.remove(self.current_index);
            if self.current_index >= self.pending.len() && self.current_index > 0 {
                self.current_index -= 1;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// draw_combat_summary
// ---------------------------------------------------------------------------

/// Draw the battle summary modal (egui window).
///
/// Shows the first pending `CombatResult`. The "Close" button dismisses it.
/// Returns `true` if at least one result was shown this frame.
///
/// Call inside the `egui_macroquad::ui` closure.
pub fn draw_combat_summary(ctx: &egui::Context, state: &mut CombatSummaryState) -> bool {
    let Some(result) = state.pending.get(state.current_index) else {
        return false;
    };
    // Clone the data we need before the mutable borrow in the window closure.
    let result = result.clone();

    let mut dismissed = false;

    egui::Window::new("Battle Report")
        .collapsible(false)
        .resizable(false)
        .default_width(320.0)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            // System name header
            ui.heading(&result.system_name);
            ui.separator();

            // Outcome badge
            ui.horizontal(|ui| {
                ui.label("Outcome:");
                ui.label(
                    RichText::new(result.outcome.label())
                        .color(result.outcome.color())
                        .strong(),
                );
            });

            ui.add_space(4.0);

            // Losses
            if result.attacker_losses.is_empty() && result.defender_losses.is_empty() {
                ui.label(RichText::new("No ships destroyed.").italics());
            } else {
                if !result.attacker_losses.is_empty() {
                    ui.label(
                        RichText::new(format!("Attacker losses: {}", format_losses(&result.attacker_losses)))
                            .color(Color32::from_rgb(240, 140, 80)),
                    );
                }
                if !result.defender_losses.is_empty() {
                    ui.label(
                        RichText::new(format!("Defender losses: {}", format_losses(&result.defender_losses)))
                            .color(Color32::from_rgb(100, 200, 255)),
                    );
                }
            }

            // Bombardment note
            if result.bombardment_occurred {
                ui.add_space(4.0);
                ui.label(
                    RichText::new("Orbital bombardment conducted.")
                        .color(Color32::from_rgb(255, 180, 60)),
                );
            }

            // Optional narrative
            if let Some(ref summary) = result.summary_text {
                ui.add_space(4.0);
                ui.label(RichText::new(summary).italics());
            }

            // Queue progress
            if state.pending.len() > 1 {
                ui.add_space(4.0);
                ui.label(
                    RichText::new(format!(
                        "Battle {} of {}",
                        state.current_index + 1,
                        state.pending.len()
                    ))
                    .small()
                    .color(Color32::GRAY),
                );
            }

            ui.add_space(8.0);
            if ui.button("Close").clicked() {
                dismissed = true;
            }
        });

    if dismissed {
        state.dismiss_current();
    }

    true
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn format_losses(losses: &[(String, u32)]) -> String {
    losses
        .iter()
        .map(|(name, count)| {
            if *count == 1 {
                name.clone()
            } else {
                format!("{count}x {name}")
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::ids::SystemKey;
    use rebellion_core::world::{ControlKind, GameWorld};

    fn make_system_key() -> SystemKey {
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
            control: ControlKind::Uncontrolled,
        })
    }

    fn make_result(outcome: BattleOutcome) -> CombatResult {
        CombatResult {
            system: make_system_key(),
            system_name: "Hoth".into(),
            tick: 42,
            outcome,
            attacker_losses: vec![("Star Destroyer".into(), 1)],
            defender_losses: vec![("Mon Calamari Cruiser".into(), 2)],
            bombardment_occurred: false,
            summary_text: None,
        }
    }

    #[test]
    fn to_messages_produces_outcome_line() {
        let result = make_result(BattleOutcome::AttackerWon);
        let msgs = result.to_messages();
        assert!(!msgs.is_empty());
        assert!(msgs[0].text.contains("Hoth"));
        assert!(msgs[0].text.contains("Attacker Won"));
        assert_eq!(msgs[0].category, MessageCategory::Combat);
    }

    #[test]
    fn to_messages_includes_losses() {
        let result = make_result(BattleOutcome::DefenderHeld);
        let msgs = result.to_messages();
        let all_text: String = msgs.iter().map(|m| m.text.as_str()).collect::<Vec<_>>().join("\n");
        assert!(all_text.contains("Star Destroyer"));
        assert!(all_text.contains("Mon Calamari Cruiser"));
    }

    #[test]
    fn to_messages_includes_bombardment_note() {
        let mut result = make_result(BattleOutcome::AttackerWon);
        result.bombardment_occurred = true;
        let msgs = result.to_messages();
        assert!(msgs.iter().any(|m| m.text.contains("bombardment")));
    }

    #[test]
    fn to_messages_includes_summary_text() {
        let mut result = make_result(BattleOutcome::Stalemate);
        result.summary_text = Some("Impasse.".into());
        let msgs = result.to_messages();
        assert!(msgs.iter().any(|m| m.text.contains("Impasse.")));
    }

    #[test]
    fn combat_summary_state_push_and_dismiss() {
        let result = make_result(BattleOutcome::AttackerWon);
        let mut state = CombatSummaryState::new();
        assert!(!state.has_pending());
        state.push(result);
        assert!(state.has_pending());
        state.dismiss_current();
        assert!(!state.has_pending());
    }

    #[test]
    fn combat_summary_state_multiple_results() {
        let mut state = CombatSummaryState::new();
        state.push(make_result(BattleOutcome::AttackerWon));
        state.push(make_result(BattleOutcome::DefenderHeld));
        assert_eq!(state.pending.len(), 2);
        state.dismiss_current();
        assert_eq!(state.pending.len(), 1);
    }

    #[test]
    fn format_losses_singular_and_plural() {
        let losses = vec![
            ("Star Destroyer".into(), 1u32),
            ("TIE Fighter".into(), 3u32),
        ];
        let result = format_losses(&losses);
        assert!(result.contains("Star Destroyer"));
        assert!(result.contains("3x TIE Fighter"));
    }

    #[test]
    fn battle_outcome_labels() {
        assert_eq!(BattleOutcome::AttackerWon.label(), "Attacker Won");
        assert_eq!(BattleOutcome::DefenderHeld.label(), "Defender Held");
        assert_eq!(BattleOutcome::Stalemate.label(), "Stalemate");
    }
}
