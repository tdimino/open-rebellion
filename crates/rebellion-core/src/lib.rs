// The original simulation mixes fixed-width counters with f32 calculations.
// These casts preserve the game's truncation, wrapping and precision semantics;
// replacing them with checked conversions would change recovered behavior.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

#[cfg(test)]
#[allow(clippy::assert_is_empty)]
macro_rules! assert_empty {
    ($value:expr $(,)?) => {{
        let value = &$value;
        assert!(value.is_empty(), "expected an empty value, got {value:?}");
    }};
}

#[cfg(test)]
#[allow(clippy::assert_is_empty)]
macro_rules! assert_not_empty {
    ($value:expr $(,)?) => {{
        let value = &$value;
        assert!(!value.is_empty(), "expected a non-empty value");
    }};
}

pub mod agent_automation;
pub mod ai;
pub mod ai_production;
pub mod ai_records;
pub mod ai_regions;
pub mod ai_research;
pub mod ai_scheduler;
pub mod ai_strategy;
pub mod assault;
pub mod battle_summary;
pub mod betrayal;
pub mod blockade;
pub mod bombardment;
pub mod build_selection;
pub mod carriage;
pub mod character_tasks;
pub mod combat;
pub mod commands;
pub mod dat;
pub mod death_star;
pub mod delivery;
pub mod economy;
pub mod effects;
pub mod events;
pub mod fleet_join;
pub mod fleet_name_bank;
pub mod fleet_tasks;
pub mod fog;
pub mod game_events;
pub mod ids;
pub mod jedi;
pub mod manufacturing;
mod mission_detection;
mod mission_planning;
pub mod missions;
pub mod movement;
pub mod net_protocol;
pub mod officer_rank;
pub mod operation_report;
pub mod post_battle;
pub mod repair;
pub mod research;
pub mod resources;
pub mod saved_interface;
pub mod scrap;
#[doc(hidden)]
pub mod serde_ordered;
pub mod stockpiles;
pub mod story_events;
pub mod support_shift;
pub mod tactical;
pub mod tick;
pub mod trace;
pub mod troop_transport;
pub mod tuning;
pub mod uprising;
pub mod victory;
pub mod world;
