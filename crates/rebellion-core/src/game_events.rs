//! Structured game event records for play-test telemetry.
//!
//! `GameEventRecord` is a pure data type — no IO. The caller
//! (rebellion-playtest or rebellion-app) handles serialization to JSONL.

use serde::{Deserialize, Serialize};

/// A single structured game event for play-test telemetry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEventRecord {
    /// Game tick when this event occurred.
    pub tick: u64,
    /// Wall-clock milliseconds since session start.
    pub wall_ms: u64,
    /// Which simulation system emitted this event.
    pub system: &'static str,
    /// Event type within the system.
    pub event_type: &'static str,
    /// System-specific payload.
    pub details: serde_json::Value,
}

impl GameEventRecord {
    pub fn new(
        tick: u64,
        wall_ms: u64,
        system: &'static str,
        event_type: &'static str,
        details: serde_json::Value,
    ) -> Self {
        Self {
            tick,
            wall_ms,
            system,
            event_type,
            details,
        }
    }
}

// ---------------------------------------------------------------------------
// System name constants
// ---------------------------------------------------------------------------

pub const SYS_MANUFACTURING: &str = "manufacturing";
pub const SYS_MOVEMENT: &str = "movement";
pub const SYS_COMBAT: &str = "combat";
pub const SYS_FOG: &str = "fog";
pub const SYS_MISSIONS: &str = "missions";
pub const SYS_EVENTS: &str = "events";
pub const SYS_AI: &str = "ai";
pub const SYS_BLOCKADE: &str = "blockade";
pub const SYS_UPRISING: &str = "uprising";
pub const SYS_DEATH_STAR: &str = "death_star";
pub const SYS_RESEARCH: &str = "research";
pub const SYS_JEDI: &str = "jedi";
pub const SYS_VICTORY: &str = "victory";
pub const SYS_BETRAYAL: &str = "betrayal";
pub const SYS_STORY: &str = "story";

// ---------------------------------------------------------------------------
// Event type constants
// ---------------------------------------------------------------------------

pub const EVT_BUILD_COMPLETE: &str = "manufacturing_complete";
pub const EVT_FLEET_ARRIVED: &str = "fleet_arrived";
pub const EVT_FLEET_DEPARTED: &str = "fleet_departed";
pub const EVT_COMBAT_SPACE: &str = "combat_space";
pub const EVT_COMBAT_GROUND: &str = "combat_ground";
pub const EVT_BOMBARDMENT: &str = "bombardment";
pub const EVT_MISSION_DISPATCHED: &str = "mission_dispatched";
pub const EVT_MISSION_RESOLVED: &str = "mission_resolved";
pub const EVT_EVENT_FIRED: &str = "event_fired";
pub const EVT_AI_ACTION: &str = "ai_action";
pub const EVT_BLOCKADE_STARTED: &str = "blockade_started";
pub const EVT_BLOCKADE_ENDED: &str = "blockade_ended";
pub const EVT_UPRISING_INCIDENT: &str = "uprising_incident";
pub const EVT_UPRISING_BEGAN: &str = "uprising_began";
pub const EVT_DS_CONSTRUCTION: &str = "death_star_construction";
pub const EVT_DS_FIRED: &str = "death_star_fired";
pub const EVT_RESEARCH_UNLOCKED: &str = "research_unlocked";
pub const EVT_JEDI_TIER: &str = "jedi_tier_advanced";
pub const EVT_JEDI_DISCOVERED: &str = "jedi_discovered";
pub const EVT_VICTORY: &str = "victory_check";
pub const EVT_BETRAYAL: &str = "betrayal";
pub const EVT_ESCAPE: &str = "character_escaped";
pub const EVT_STORY: &str = "story_event";
pub const EVT_CAPTURE: &str = "character_captured";
pub const EVT_FOG_REVEALED: &str = "fog_revealed";
pub const EVT_CONTROL_CHANGED: &str = "control_changed";
pub const EVT_CAMPAIGN_SNAPSHOT: &str = "campaign_snapshot";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_serializes_to_json() {
        let record = GameEventRecord::new(
            42,
            1200,
            SYS_MANUFACTURING,
            EVT_BUILD_COMPLETE,
            serde_json::json!({"system": "Coruscant"}),
        );
        let json = serde_json::to_string(&record).expect("serialize");
        assert!(json.contains("\"tick\":42"));
        assert!(json.contains("\"system\":\"manufacturing\""));
        assert!(json.contains("\"event_type\":\"manufacturing_complete\""));
    }

    #[test]
    fn record_deserializes_from_json() {
        let json = r#"{
            "tick": 10,
            "wall_ms": 500,
            "system": "combat",
            "event_type": "combat_space",
            "details": {"winner": "attacker"}
        }"#;
        let record: GameEventRecord = serde_json::from_str(json).expect("deserialize");
        assert_eq!(record.tick, 10);
        assert_eq!(record.wall_ms, 500);
        assert_eq!(record.system, "combat");
        assert_eq!(record.event_type, "combat_space");
    }

    #[test]
    fn all_event_type_constants_are_unique() {
        let all = [
            EVT_BUILD_COMPLETE,
            EVT_FLEET_ARRIVED,
            EVT_FLEET_DEPARTED,
            EVT_COMBAT_SPACE,
            EVT_COMBAT_GROUND,
            EVT_BOMBARDMENT,
            EVT_MISSION_DISPATCHED,
            EVT_MISSION_RESOLVED,
            EVT_EVENT_FIRED,
            EVT_AI_ACTION,
            EVT_BLOCKADE_STARTED,
            EVT_BLOCKADE_ENDED,
            EVT_UPRISING_INCIDENT,
            EVT_UPRISING_BEGAN,
            EVT_DS_CONSTRUCTION,
            EVT_DS_FIRED,
            EVT_RESEARCH_UNLOCKED,
            EVT_JEDI_TIER,
            EVT_JEDI_DISCOVERED,
            EVT_VICTORY,
            EVT_BETRAYAL,
            EVT_ESCAPE,
            EVT_STORY,
            EVT_CAPTURE,
        ];
        let mut seen = std::collections::HashSet::new();
        for evt in &all {
            assert!(
                seen.insert(*evt),
                "duplicate event type constant: {}",
                evt
            );
        }
    }
}
