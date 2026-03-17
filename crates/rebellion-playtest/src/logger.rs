//! JSONL event logger for playtest sessions.

use std::io::{BufWriter, Write};
use std::path::Path;

use rebellion_core::game_events::GameEventRecord;

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
    pub fn print_summary(&self) {
        let mut counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        for event in &self.events {
            *counts.entry(event.event_type).or_default() += 1;
        }
        let mut sorted: Vec<_> = counts.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));

        println!("\n=== Playtest Summary ===");
        println!("Total events: {}", self.events.len());
        if let Some(last) = self.events.last() {
            println!("Final tick: {}", last.tick);
        }
        println!("\nEvents by type:");
        for (event_type, count) in &sorted {
            println!("  {:30} {:>6}", event_type, count);
        }
    }
}
