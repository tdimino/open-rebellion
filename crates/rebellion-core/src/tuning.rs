//! Externalized tuning parameters for AI, movement, and production systems.
//!
//! All hardcoded constants are centralized here as `GameConfig`. The struct
//! is `Serialize`/`Deserialize` so the autoresearch loop can mutate parameters
//! via JSON, and the playtest binary can accept `--config <path>`.
//!
//! Each field is tagged as **parity** (matches original REBEXE.EXE behavior)
//! or **augmentation** (our improvement beyond the original).
//!
//! `Default` produces the values that match the current hardcoded constants.

use serde::{Deserialize, Serialize};

/// Root configuration for all tunable game parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GameConfig {
    pub ai: AiConfig,
    pub movement: MovementConfig,
    pub production: ProductionConfig,
    pub scoring: ScoringConfig,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            ai: AiConfig::default(),
            movement: MovementConfig::default(),
            production: ProductionConfig::default(),
            scoring: ScoringConfig::default(),
        }
    }
}

/// AI behavior tuning parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AiConfig {
    /// Game-days between AI re-evaluations. **Augmentation** (original unknown).
    pub tick_interval: u64,

    /// Minimum diplomacy skill to dispatch on diplomacy missions. **Parity.**
    pub diplomacy_skill_threshold: u32,

    /// Maximum popularity at which AI still sends diplomacy missions. **Augmentation.**
    pub diplomacy_target_popularity_cap: f32,

    /// Minimum espionage skill for covert ops dispatch. **Augmentation.**
    pub espionage_skill_threshold: u32,

    /// Minimum expected success probability for covert ops. **Augmentation.**
    pub covert_min_success_prob: f64,

    /// Maximum covert operations dispatched per evaluation cycle. **Augmentation.**
    pub max_covert_ops_per_eval: usize,

    /// Maximum construction yards per system. **Parity.**
    pub max_construction_yards: usize,

    /// Maximum simultaneous attack fronts (capped at fleet count). **Augmentation.**
    pub max_attack_fronts: usize,

    /// Battle cooldown decay ticks — full recovery after this many ticks. **Augmentation.**
    pub battle_cooldown_ticks: f64,

    /// Proximity divisor for attack scoring (higher = less distance penalty). **Augmentation.**
    pub proximity_divisor: f64,

    // ── Attack scoring weights (must sum to 1.0) ── **Augmentation.**
    /// Weight for target weakness in attack scoring.
    pub weight_weakness: f64,
    /// Weight for proximity in attack scoring.
    pub weight_proximity: f64,
    /// Weight for deconfliction in attack scoring.
    pub weight_deconfliction: f64,
    /// Weight for battle freshness in attack scoring.
    pub weight_freshness: f64,

    /// Minimum popularity threshold for Alliance covert target selection. **Augmentation.**
    pub covert_target_popularity_threshold: f32,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            tick_interval: 7,
            diplomacy_skill_threshold: 60,
            diplomacy_target_popularity_cap: 0.8,
            espionage_skill_threshold: 50,
            covert_min_success_prob: 0.30,
            max_covert_ops_per_eval: 3,
            max_construction_yards: 5,
            max_attack_fronts: 3,
            battle_cooldown_ticks: 100.0,
            proximity_divisor: 100.0,
            weight_weakness: 0.30,
            weight_proximity: 0.30,
            weight_deconfliction: 0.25,
            weight_freshness: 0.15,
            covert_target_popularity_threshold: 0.3,
        }
    }
}

/// Fleet movement tuning parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MovementConfig {
    /// Euclidean distance multiplier for transit time. **Augmentation.**
    pub distance_scale: u32,

    /// Minimum transit time in game-days. **Augmentation.**
    pub min_transit_ticks: u32,

    /// Default hyperdrive rating for fighters without a class value. **Parity.**
    pub default_fighter_hyperdrive: u32,
}

impl Default for MovementConfig {
    fn default() -> Self {
        Self {
            distance_scale: 2,
            min_transit_ticks: 10,
            default_fighter_hyperdrive: 60,
        }
    }
}

/// Production doctrine tuning parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ProductionConfig {
    /// Build capital ships when fewer than this many in the fleet. **Augmentation.**
    pub capship_threshold: usize,

    /// Fighter-to-capship ratio target. **Augmentation.**
    pub fighter_ratio: usize,
}

impl Default for ProductionConfig {
    fn default() -> Self {
        Self {
            capship_threshold: 5,
            fighter_ratio: 3,
        }
    }
}

/// Scoring targets for eval_game_quality.py (not used in Rust, but stored
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
    fn default() -> Self {
        Self {
            victory_center: 3000,
            combat_pct_center: 10.0,
            mfg_target: 50,
        }
    }
}
