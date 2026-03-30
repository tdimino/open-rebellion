//! Ground combat display — troop engagement after space combat.
//!
//! After space combat concludes in the tactical view, if the winning attacker's
//! system has enemy ground troops, a simplified ground combat phase runs.
//! This module renders regiment bars, engagement results, and the final
//! battle summary.
//!
//! # Integration
//!
//! ```ignore
//! // In tactical_view.rs, after space combat Results phase:
//! if session.winner == Some(CombatWinner::Attacker) && has_enemy_troops {
//!     ground_state = GroundCombatState::new(...);
//!     // Render ground_combat::draw_ground_combat(...)
//! }
//! ```

use egui_macroquad::egui::{self, Color32, RichText};
use macroquad::prelude::*;
use rebellion_core::ids::SystemKey;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// One regiment in ground combat.
#[derive(Debug, Clone)]
pub struct GroundRegiment {
    pub name: String,
    pub strength: i16,
    pub max_strength: i16,
    pub is_attacker: bool,
    pub alive: bool,
}

/// Ground combat session state.
#[derive(Debug, Clone)]
pub struct GroundCombatState {
    pub system: SystemKey,
    pub system_name: String,
    pub regiments: Vec<GroundRegiment>,
    pub phase: GroundPhase,
    pub combat_tick: u32,
    /// Winner once resolved.
    pub winner: Option<GroundWinner>,
    /// Step accumulator for animation pacing.
    pub step_accumulator: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroundPhase {
    /// Troops engaging — animated regiment bars depleting.
    Engaging,
    /// Combat resolved — showing results.
    Results,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroundWinner {
    Attacker,
    Defender,
    Draw,
}

/// Action returned by draw_ground_combat.
#[derive(Debug, Clone, PartialEq)]
pub enum GroundAction {
    /// Still in ground combat view.
    None,
    /// Ground combat done — return to results/galaxy.
    Done,
}

impl GroundCombatState {
    /// Create ground combat from system troop data.
    ///
    /// `attacker_troops` and `defender_troops` are (name, strength) pairs.
    pub fn new(
        system: SystemKey,
        system_name: String,
        attacker_troops: Vec<(String, i16)>,
        defender_troops: Vec<(String, i16)>,
    ) -> Self {
        let mut regiments = Vec::new();
        for (name, strength) in attacker_troops {
            regiments.push(GroundRegiment {
                name,
                max_strength: strength,
                strength,
                is_attacker: true,
                alive: strength > 0,
            });
        }
        for (name, strength) in defender_troops {
            regiments.push(GroundRegiment {
                name,
                max_strength: strength,
                strength,
                is_attacker: false,
                alive: strength > 0,
            });
        }

        GroundCombatState {
            system,
            system_name,
            regiments,
            phase: GroundPhase::Engaging,
            combat_tick: 0,
            winner: None,
            step_accumulator: 0.0,
        }
    }

    /// Advance ground combat by one tick.
    ///
    /// Each tick, opposing regiments damage each other proportionally.
    /// Returns true if combat ended this tick.
    pub fn step(&mut self) -> bool {
        if self.phase != GroundPhase::Engaging { return false; }

        self.combat_tick += 1;

        // Collect active regiments per side.
        let atk_total: i16 = self.regiments.iter()
            .filter(|r| r.is_attacker && r.alive)
            .map(|r| r.strength)
            .sum();
        let def_total: i16 = self.regiments.iter()
            .filter(|r| !r.is_attacker && r.alive)
            .map(|r| r.strength)
            .sum();

        if atk_total == 0 || def_total == 0 {
            self.finish();
            return true;
        }

        // Each side deals damage proportional to its total strength.
        // Attacker hits defender, defender hits attacker.
        let atk_damage = (atk_total / 8).max(1);
        let def_damage = (def_total / 8).max(1);

        // Apply damage to defender regiments (spread across active ones).
        Self::apply_damage(&mut self.regiments, false, atk_damage);
        // Apply damage to attacker regiments.
        Self::apply_damage(&mut self.regiments, true, def_damage);

        // Check for battle end.
        let atk_alive = self.regiments.iter().any(|r| r.is_attacker && r.alive);
        let def_alive = self.regiments.iter().any(|r| !r.is_attacker && r.alive);

        if !atk_alive || !def_alive {
            self.finish();
            return true;
        }

        false
    }

    fn apply_damage(regiments: &mut [GroundRegiment], is_target_attacker: bool, mut damage: i16) {
        for r in regiments.iter_mut() {
            if damage <= 0 { break; }
            if r.is_attacker != is_target_attacker || !r.alive { continue; }
            let take = damage.min(r.strength);
            r.strength -= take;
            damage -= take;
            if r.strength <= 0 {
                r.alive = false;
            }
        }
    }

    fn finish(&mut self) {
        let atk_alive = self.regiments.iter().any(|r| r.is_attacker && r.alive);
        let def_alive = self.regiments.iter().any(|r| !r.is_attacker && r.alive);
        self.winner = Some(match (atk_alive, def_alive) {
            (true, false) => GroundWinner::Attacker,
            (false, true) => GroundWinner::Defender,
            _ => GroundWinner::Draw,
        });
        self.phase = GroundPhase::Results;
    }
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

/// Draw the ground combat view.
///
/// Shows regiment bars for both sides with animated depletion during
/// the Engaging phase, and a summary during the Results phase.
pub fn draw_ground_combat(state: &mut GroundCombatState) -> GroundAction {
    // Step combat during Engaging phase.
    if state.phase == GroundPhase::Engaging {
        let dt = get_frame_time();
        state.step_accumulator += dt * 6.0; // ~6 ticks per second
        while state.step_accumulator >= 1.0 {
            state.step_accumulator -= 1.0;
            if state.step() { break; }
        }
    }

    // Background.
    clear_background(Color::new(0.03, 0.02, 0.01, 1.0));

    let sw = screen_width();
    let sh = screen_height();
    let mut action = GroundAction::None;

    // Draw regiment bars.
    let bar_width = sw * 0.35;
    let bar_height = 20.0;
    let spacing = 30.0;
    let margin = 40.0;

    // Title.
    let title = format!("Ground Battle — {}", state.system_name);
    let title_size = 24.0;
    let title_dims = measure_text(&title, None, title_size as u16, 1.0);
    draw_text(&title, (sw - title_dims.width) / 2.0, margin, title_size, WHITE);

    // Attacker regiments (left side).
    let atk_x = margin;
    let start_y = margin + 40.0;
    draw_text("ATTACKERS", atk_x, start_y, 16.0, Color::new(0.3, 0.8, 1.0, 0.9));

    let atk_regiments: Vec<(usize, &GroundRegiment)> = state.regiments.iter().enumerate()
        .filter(|(_, r)| r.is_attacker)
        .collect();

    for (row, &(_, reg)) in atk_regiments.iter().enumerate() {
        let y = start_y + 20.0 + row as f32 * spacing;
        let frac = if reg.max_strength > 0 {
            reg.strength as f32 / reg.max_strength as f32
        } else {
            0.0
        };

        // Background bar.
        draw_rectangle(atk_x, y, bar_width, bar_height, Color::new(0.2, 0.2, 0.2, 0.8));
        // Health fill.
        let fill_color = if reg.alive {
            Color::new(0.2, 0.6, 1.0, 0.9)
        } else {
            Color::new(0.4, 0.1, 0.1, 0.6)
        };
        draw_rectangle(atk_x, y, bar_width * frac, bar_height, fill_color);
        // Border.
        draw_rectangle_lines(atk_x, y, bar_width, bar_height, 1.0, Color::new(0.5, 0.5, 0.5, 0.6));
        // Label.
        let label = format!("{} ({}/{})", reg.name, reg.strength, reg.max_strength);
        draw_text(&label, atk_x + 4.0, y + 15.0, 12.0, WHITE);
    }

    // Defender regiments (right side).
    let def_x = sw - margin - bar_width;
    draw_text("DEFENDERS", def_x, start_y, 16.0, Color::new(1.0, 0.4, 0.3, 0.9));

    let def_regiments: Vec<(usize, &GroundRegiment)> = state.regiments.iter().enumerate()
        .filter(|(_, r)| !r.is_attacker)
        .collect();

    for (row, &(_, reg)) in def_regiments.iter().enumerate() {
        let y = start_y + 20.0 + row as f32 * spacing;
        let frac = if reg.max_strength > 0 {
            reg.strength as f32 / reg.max_strength as f32
        } else {
            0.0
        };

        draw_rectangle(def_x, y, bar_width, bar_height, Color::new(0.2, 0.2, 0.2, 0.8));
        let fill_color = if reg.alive {
            Color::new(1.0, 0.3, 0.2, 0.9)
        } else {
            Color::new(0.4, 0.1, 0.1, 0.6)
        };
        draw_rectangle(def_x, y, bar_width * frac, bar_height, fill_color);
        draw_rectangle_lines(def_x, y, bar_width, bar_height, 1.0, Color::new(0.5, 0.5, 0.5, 0.6));
        let label = format!("{} ({}/{})", reg.name, reg.strength, reg.max_strength);
        draw_text(&label, def_x + 4.0, y + 15.0, 12.0, WHITE);
    }

    // Center "VS" indicator.
    let vs_x = sw / 2.0 - 10.0;
    let vs_y = start_y + 50.0;
    draw_text("VS", vs_x, vs_y, 20.0, Color::new(0.8, 0.8, 0.2, 0.8));

    // egui overlay for results/controls.
    egui_macroquad::ui(|ctx| {
        egui::TopBottomPanel::bottom("ground_bottom").show(ctx, |ui| {
            ui.horizontal(|ui| {
                match state.phase {
                    GroundPhase::Engaging => {
                        ui.label(
                            RichText::new(format!("Ground engagement in progress... Tick {}", state.combat_tick))
                                .color(Color32::from_rgb(255, 180, 60))
                        );
                    }
                    GroundPhase::Results => {
                        let (text, color) = match state.winner {
                            Some(GroundWinner::Attacker) => ("Attacker ground victory!", Color32::from_rgb(60, 220, 60)),
                            Some(GroundWinner::Defender) => ("Defender holds ground!", Color32::from_rgb(220, 60, 60)),
                            Some(GroundWinner::Draw) | None => ("Ground combat draw", Color32::from_rgb(200, 200, 60)),
                        };
                        ui.label(RichText::new(text).color(color).strong().size(14.0));

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button(
                                RichText::new("Continue")
                                    .color(Color32::from_rgb(200, 200, 60))
                                    .strong()
                            ).clicked() {
                                action = GroundAction::Done;
                            }
                        });
                    }
                }
            });
        });
    });
    egui_macroquad::draw();

    action
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::ids::SystemKey;
    use rebellion_core::world::GameWorld;

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
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: rebellion_core::world::ControlKind::Uncontrolled,
            espionage_rating: 0.0,
        })
    }

    #[test]
    fn ground_combat_resolves() {
        let sys = make_system_key();
        let mut state = GroundCombatState::new(
            sys,
            "Hoth".into(),
            vec![("Rebel Infantry".into(), 100)],
            vec![("Stormtroopers".into(), 50)],
        );
        assert_eq!(state.phase, GroundPhase::Engaging);

        // Run until resolved.
        for _ in 0..200 {
            if state.step() { break; }
        }
        assert_eq!(state.phase, GroundPhase::Results);
        assert!(state.winner.is_some());
    }

    #[test]
    fn ground_combat_stronger_wins() {
        let sys = make_system_key();
        let mut state = GroundCombatState::new(
            sys,
            "Endor".into(),
            vec![("Rebel Elite".into(), 200)],
            vec![("Scout Troop".into(), 30)],
        );
        for _ in 0..200 {
            if state.step() { break; }
        }
        assert_eq!(state.winner, Some(GroundWinner::Attacker));
    }
}
