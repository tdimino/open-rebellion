//! Shared command registry for play-test infrastructure.
//!
//! Defines commands available to both the interactive command palette
//! (rebellion-render) and the headless CLI (rebellion-playtest).
//! Pure data — no IO, no PanelAction import.

/// A command definition shared between GUI palette and CLI.
#[derive(Debug, Clone)]
pub struct CommandDef {
    /// Machine-readable identifier (e.g. "advance_1_tick").
    pub id: &'static str,
    /// Human-readable label shown in the palette.
    pub label: &'static str,
    /// Short description of what the command does.
    pub description: &'static str,
    /// Category for grouping: "Time", "Speed", "Inspect", "Control".
    pub category: &'static str,
}

/// All registered play-test commands.
pub fn all_commands() -> Vec<CommandDef> {
    vec![
        // Time Control
        CommandDef { id: "advance_1_tick", label: "Advance 1 tick", description: "Step simulation forward by 1 tick", category: "Time" },
        CommandDef { id: "advance_10_ticks", label: "Advance 10 ticks", description: "Step simulation forward by 10 ticks", category: "Time" },
        CommandDef { id: "advance_100_ticks", label: "Advance 100 ticks", description: "Step simulation forward by 100 ticks", category: "Time" },
        CommandDef { id: "advance_1000_ticks", label: "Advance 1000 ticks", description: "Step simulation forward by 1000 ticks", category: "Time" },
        // Speed
        CommandDef { id: "speed_paused", label: "Set Speed: Paused", description: "Pause the simulation", category: "Speed" },
        CommandDef { id: "speed_normal", label: "Set Speed: Normal", description: "Set simulation to 1x speed", category: "Speed" },
        CommandDef { id: "speed_fast", label: "Set Speed: Fast", description: "Set simulation to 2x speed", category: "Speed" },
        CommandDef { id: "speed_faster", label: "Set Speed: Faster", description: "Set simulation to 4x speed", category: "Speed" },
        // Inspect
        CommandDef { id: "show_game_stats", label: "Show Game Stats", description: "Display current game statistics", category: "Inspect" },
        CommandDef { id: "list_active_missions", label: "List Active Missions", description: "Show all in-progress missions", category: "Inspect" },
        CommandDef { id: "list_active_fleets", label: "List Active Fleets", description: "Show all fleet positions and compositions", category: "Inspect" },
        CommandDef { id: "show_event_count", label: "Show Event Count", description: "Display number of triggered events", category: "Inspect" },
        // Control
        CommandDef { id: "toggle_dual_ai", label: "Toggle Dual AI", description: "Enable/disable AI control for both factions", category: "Control" },
        CommandDef { id: "force_victory_check", label: "Force Victory Check", description: "Immediately evaluate victory conditions", category: "Control" },
        CommandDef { id: "reveal_all_systems", label: "Reveal All Systems", description: "Remove fog of war from all systems", category: "Control" },
        CommandDef { id: "export_game_log", label: "Export Game Log", description: "Write message log to file", category: "Control" },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_command_ids_unique() {
        let cmds = all_commands();
        let mut ids: Vec<&str> = cmds.iter().map(|c| c.id).collect();
        let total = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), total, "Duplicate command IDs found");
    }

    #[test]
    fn all_categories_known() {
        let known = ["Time", "Speed", "Inspect", "Control"];
        for cmd in all_commands() {
            assert!(known.contains(&cmd.category), "Unknown category: {}", cmd.category);
        }
    }
}
