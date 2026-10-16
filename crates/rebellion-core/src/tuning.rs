//! Externalized tuning parameters: the autoresearch scoring targets.
//!
//! The AI's tempo, the orders a side issues a day and the rounds it runs,
//! defaults to the original's values; its picks follow the original's rules.
//! `GameConfig` is `Serialize`/`Deserialize` so the autoresearch loop can
//! mutate parameters via JSON, and the playtest binary can accept
//! `--config <path>`.
//!
//! Each field is tagged as **parity** (matches original REBEXE.EXE behavior)
//! or **augmentation** (our improvement beyond the original).
//!
//! `Default` produces the values that match the current hardcoded constants.

use serde::{Deserialize, Serialize};

use crate::tick::GameSpeed;

/// Root configuration for all tunable game parameters.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GameConfig {
    pub ai: AiConfig,
    pub scoring: ScoringConfig,
}

/// The AI's tempo. port: tunable through `--config` so the tempo can be
/// calibrated; the defaults are the original's.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AiConfig {
    /// The orders a side issues a day (`+0x428` = 10, `FUN_00485a50.c:35`;
    /// tested at `FUN_00485b40.c:21`).
    pub day_orders: u16,
    /// The rounds a side runs a day, by speed.
    pub rounds_per_day: RoundsPerDay,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            day_orders: u16::from(crate::ai_scheduler::DAY_ORDERS),
            rounds_per_day: RoundsPerDay::default(),
        }
    }
}

/// The rounds a side runs a day at each speed: the per-day count plus one
/// (`FUN_0051dea0.c:11-30`), a round per in-day step (`FUN_004361a0`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RoundsPerDay {
    pub very_slow: u16,
    pub slow: u16,
    pub medium: u16,
    pub fast: u16,
}

impl Default for RoundsPerDay {
    /// `GameSpeed::original_rate` plus one: 601, 61, 13 and 5
    /// (`FUN_0051dea0.c:11-30`).
    fn default() -> Self {
        let rounds = |speed: GameSpeed| speed.original_rate().map_or(1, |rate| rate + 1);
        Self {
            very_slow: rounds(GameSpeed::VerySlow),
            slow: rounds(GameSpeed::Slow),
            medium: rounds(GameSpeed::Medium),
            fast: rounds(GameSpeed::Fast),
        }
    }
}

impl RoundsPerDay {
    /// The rounds a day at `speed`. port: a paused clock, as a replay's
    /// `AdvanceTicks` or a headless run uses, runs Fast's.
    #[must_use]
    pub const fn at(&self, speed: GameSpeed) -> u16 {
        match speed {
            GameSpeed::VerySlow => self.very_slow,
            GameSpeed::Slow => self.slow,
            GameSpeed::Medium => self.medium,
            GameSpeed::Fast | GameSpeed::Paused => self.fast,
        }
    }
}

/// Scoring targets for `eval_game_quality.py` (not used in Rust, but stored
/// here for autoresearch config symmetry).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ScoringConfig {
    /// Ideal victory tick (bell curve center). **Augmentation.**
    pub victory_center: u32,
    /// Ideal combat event percentage (bell curve center). **Augmentation.**
    pub combat_pct_center: f64,
    /// Ideal manufacturing completions per 5000 ticks. **Augmentation.**
    pub mfg_target: u32,
}

impl Default for ScoringConfig {
    // port: Augmentation targets for the autoresearch scorer, not original rules.
    fn default() -> Self {
        Self {
            victory_center: 3000,
            combat_pct_center: 10.0,
            mfg_target: 50,
        }
    }
}
