//! JSONL event logger for playtest sessions.

use std::io::{BufWriter, Write};
use std::path::Path;

use rebellion_core::dat::Faction;
use rebellion_core::game_events::GameEventRecord;
use rebellion_core::movement::MovementState;
use rebellion_core::world::GameWorld;

pub struct EventLogger {
    events: Vec<GameEventRecord>,
}

impl EventLogger {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn extend(&mut self, events: Vec<GameEventRecord>) {
        self.events.extend(events);
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Access stored events for inspection (REPL `events N` command).
    pub fn events(&self) -> &[GameEventRecord] {
        &self.events
    }

    /// Export all events as JSONL (one JSON object per line).
    pub fn export_jsonl(&self, path: &Path) -> anyhow::Result<()> {
        let file = std::fs::File::create(path)?;
        let mut writer = BufWriter::new(file);
        for event in &self.events {
            serde_json::to_writer(&mut writer, event)?;
            writer.write_all(b"\n")?;
        }
        writer.flush()?;
        Ok(())
    }

    /// Print summary statistics to stdout.
    pub fn print_summary(&self, world: &GameWorld, movement: &MovementState) {
        let mut counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        for event in &self.events {
            *counts.entry(event.event_type).or_default() += 1;
        }
        let mut sorted: Vec<_> = counts.iter().map(|(&k, &v)| (k, v)).collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));

        println!("\n=== Playtest Summary ===");
        println!("Total events: {}", self.events.len());
        if let Some(last) = self.events.last() {
            println!("Final tick: {}", last.tick);
        }

        // ── Event counts ──────────────────────────────────────────────
        println!("\nEvents by type:");
        for (event_type, count) in &sorted {
            println!("  {:30} {:>6}", event_type, count);
        }

        // ── Galaxy control ────────────────────────────────────────────
        let mut alliance_systems = Vec::new();
        let mut empire_systems = Vec::new();
        let mut neutral_count = 0usize;
        for (_, sys) in world.systems.iter() {
            match sys.controlling_faction {
                Some(Faction::Alliance) => alliance_systems.push(sys.name.as_str()),
                Some(Faction::Empire) => empire_systems.push(sys.name.as_str()),
                _ => neutral_count += 1,
            }
        }
        println!("\nGalaxy control:");
        println!("  Alliance: {} systems", alliance_systems.len());
        if alliance_systems.len() <= 10 {
            for name in &alliance_systems {
                println!("    - {}", name);
            }
        }
        println!("  Empire:   {} systems", empire_systems.len());
        if empire_systems.len() <= 10 {
            for name in &empire_systems {
                println!("    - {}", name);
            }
        }
        println!("  Neutral:  {} systems", neutral_count);

        // ── Fleets in transit ─────────────────────────────────────────
        if !movement.is_empty() {
            println!("\nFleets in transit: {}", movement.len());
            for (_, order) in movement.orders() {
                let origin = world.systems.get(order.origin)
                    .map(|s| s.name.as_str()).unwrap_or("?");
                let dest = world.systems.get(order.destination)
                    .map(|s| s.name.as_str()).unwrap_or("?");
                let faction = world.fleets.get(order.fleet)
                    .map(|f| if f.is_alliance { "Alliance" } else { "Empire" })
                    .unwrap_or("?");
                println!("  {} fleet: {} → {} ({:.0}%, {} ticks left)",
                    faction, origin, dest,
                    order.progress() * 100.0,
                    order.ticks_remaining());
            }
        }

        // ── Combat diagnostics ────────────────────────────────────────
        let move_fleet_attacks = self.events.iter()
            .filter(|e| e.event_type == "ai_action")
            .filter(|e| e.details.get("type").and_then(|v| v.as_str()) == Some("MoveFleet"))
            .filter(|e| e.details.get("reason").and_then(|v| v.as_str()) == Some("Attack"))
            .count();
        let move_fleet_reinforce = self.events.iter()
            .filter(|e| e.event_type == "ai_action")
            .filter(|e| e.details.get("type").and_then(|v| v.as_str()) == Some("MoveFleet"))
            .filter(|e| e.details.get("reason").and_then(|v| v.as_str()) == Some("Reinforce"))
            .count();
        println!("\nCombat diagnostics:");
        println!("  Fleet attack orders:     {}", move_fleet_attacks);
        println!("  Fleet reinforce orders:  {}", move_fleet_reinforce);
        println!("  Space battles:           {}", counts.get("combat_space").unwrap_or(&0));
        println!("  Ground battles:          {}", counts.get("combat_ground").unwrap_or(&0));
        println!("  Bombardments:            {}", counts.get("bombardment").unwrap_or(&0));
    }
}
