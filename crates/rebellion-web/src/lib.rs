//! WASM bindings exposing a clean game-state API to a TypeScript/JS web UI.
//!
//! Architecture note:
//!
//! This crate is intentionally **decoupled** from `rebellion-core`'s deep
//! internal types. It exposes a stable, JSON-friendly DTO layer that the
//! web UI consumes. The DTO layer is the contract; the underlying engine
//! can evolve without breaking the UI.
//!
//! For the demo, we maintain an in-memory game state in this crate
//! directly. Production wiring would replace `Engine::tick()` with calls
//! through `rebellion-data::simulation::run_simulation_tick()`, projecting
//! results into the DTO types we already define here.
//!
//! Character group profiles — the showcase QoL feature — live entirely in
//! the UI layer (localStorage). The engine only sees the resulting batch of
//! mission dispatch commands.

use std::cell::RefCell;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// ──────────────────────────────────────────────────────────────────────────
// DTO types — mirror webapp/src/types/game.ts exactly
// ──────────────────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Skill {
    pub base: u32,
    pub variance: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub id: u32,
    pub name: String,
    pub faction: String, // "Alliance" | "Empire"
    pub is_major: bool,
    pub on_mission: bool,
    pub on_hidden_mission: bool,
    pub is_captive: bool,
    pub current_system_id: Option<u32>,
    pub diplomacy: Skill,
    pub espionage: Skill,
    pub combat: Skill,
    pub leadership: Skill,
    pub loyalty: Skill,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StarSystem {
    pub id: u32,
    pub name: String,
    pub sector_id: u32,
    pub control: String,
    pub popularity_alliance: f32,
    pub popularity_empire: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActiveMission {
    pub id: u64,
    pub character_id: u32,
    pub character_name: String,
    pub kind: String,
    pub target_system_id: u32,
    pub target_system_name: String,
    pub ticks_remaining: u32,
    pub total_ticks: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WorldState {
    pub current_day: u32,
    pub character_count: u32,
    pub system_count: u32,
    pub active_mission_count: u32,
}

// ──────────────────────────────────────────────────────────────────────────
// Engine singleton
// ──────────────────────────────────────────────────────────────────────────

struct Engine {
    current_day: u32,
    characters: Vec<Character>,
    systems: Vec<StarSystem>,
    missions: Vec<ActiveMission>,
    next_mission_id: u64,
}

thread_local! {
    static ENGINE: RefCell<Engine> = RefCell::new(Engine {
        current_day: 0,
        characters: Vec::new(),
        systems: Vec::new(),
        missions: Vec::new(),
        next_mission_id: 1,
    });
}

#[wasm_bindgen(start)]
pub fn _start() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// ──────────────────────────────────────────────────────────────────────────
// Init / world setup
// ──────────────────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub fn init_demo_world() {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        engine.current_day = 192;
        engine.systems = build_demo_systems();
        engine.characters = build_demo_characters();
        engine.missions = Vec::new();
        engine.next_mission_id = 1;
    });
}

#[wasm_bindgen]
pub fn world_loaded() -> bool {
    ENGINE.with(|e| !e.borrow().characters.is_empty())
}

// ──────────────────────────────────────────────────────────────────────────
// Query API
// ──────────────────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub fn get_world_state() -> JsValue {
    ENGINE.with(|e| {
        let e = e.borrow();
        let state = WorldState {
            current_day: e.current_day,
            character_count: e.characters.len() as u32,
            system_count: e.systems.len() as u32,
            active_mission_count: e.missions.len() as u32,
        };
        serde_wasm_bindgen::to_value(&state).unwrap()
    })
}

#[wasm_bindgen]
pub fn get_characters() -> JsValue {
    ENGINE.with(|e| serde_wasm_bindgen::to_value(&e.borrow().characters).unwrap())
}

#[wasm_bindgen]
pub fn get_characters_on_system(system_id: u32) -> JsValue {
    ENGINE.with(|e| {
        let filtered: Vec<Character> = e
            .borrow()
            .characters
            .iter()
            .filter(|c| c.current_system_id == Some(system_id))
            .cloned()
            .collect();
        serde_wasm_bindgen::to_value(&filtered).unwrap()
    })
}

#[wasm_bindgen]
pub fn get_systems() -> JsValue {
    ENGINE.with(|e| serde_wasm_bindgen::to_value(&e.borrow().systems).unwrap())
}

#[wasm_bindgen]
pub fn get_active_missions() -> JsValue {
    ENGINE.with(|e| serde_wasm_bindgen::to_value(&e.borrow().missions).unwrap())
}

// ──────────────────────────────────────────────────────────────────────────
// Command API
// ──────────────────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub fn dispatch_mission(
    character_id: u32,
    target_system_id: u32,
    kind: &str,
) -> Result<u64, JsValue> {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();

        let duration = synthetic_duration(kind);
        if duration == 0 {
            return Err(JsValue::from_str(&format!("Unknown mission kind: {kind}")));
        }

        // Find the character + system (using JS-friendly id lookups)
        let char_idx = engine
            .characters
            .iter()
            .position(|c| c.id == character_id)
            .ok_or_else(|| JsValue::from_str("Character not found"))?;

        let sys = engine
            .systems
            .iter()
            .find(|s| s.id == target_system_id)
            .ok_or_else(|| JsValue::from_str("System not found"))?
            .clone();

        let mission_id = engine.next_mission_id;
        engine.next_mission_id += 1;

        let mission = ActiveMission {
            id: mission_id,
            character_id,
            character_name: engine.characters[char_idx].name.clone(),
            kind: kind.to_string(),
            target_system_id,
            target_system_name: sys.name,
            ticks_remaining: duration,
            total_ticks: duration,
        };

        engine.characters[char_idx].on_mission = true;
        engine.missions.push(mission);

        Ok(mission_id)
    })
}

#[wasm_bindgen]
pub fn advance_days(n: u32) {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        engine.current_day = engine.current_day.saturating_add(n);

        // Decrement mission timers, collect freed characters
        let mut freed = Vec::new();
        for m in &mut engine.missions {
            m.ticks_remaining = m.ticks_remaining.saturating_sub(n);
            if m.ticks_remaining == 0 {
                freed.push(m.character_id);
            }
        }
        engine.missions.retain(|m| m.ticks_remaining > 0);

        for char_id in freed {
            if let Some(c) = engine.characters.iter_mut().find(|c| c.id == char_id) {
                c.on_mission = false;
            }
        }
    });
}

// ──────────────────────────────────────────────────────────────────────────
// Demo world builders
// ──────────────────────────────────────────────────────────────────────────

fn synthetic_duration(kind: &str) -> u32 {
    match kind {
        "Diplomacy" => 10,
        "Espionage" => 8,
        "Sabotage" => 12,
        "Assassination" => 15,
        "Rescue" => 14,
        "Abduction" => 16,
        "InciteUprising" => 20,
        "Recruitment" => 7,
        _ => 0,
    }
}

fn build_demo_systems() -> Vec<StarSystem> {
    vec![
        StarSystem { id: 0, name: "Coruscant".into(), sector_id: 0, control: "Empire".into(),
                     popularity_alliance: 0.25, popularity_empire: 0.75 },
        StarSystem { id: 1, name: "Yavin".into(), sector_id: 0, control: "Alliance".into(),
                     popularity_alliance: 0.75, popularity_empire: 0.25 },
        StarSystem { id: 2, name: "Hoth".into(), sector_id: 0, control: "Alliance".into(),
                     popularity_alliance: 0.75, popularity_empire: 0.25 },
        StarSystem { id: 3, name: "Tatooine".into(), sector_id: 0, control: "Empire".into(),
                     popularity_alliance: 0.25, popularity_empire: 0.75 },
        StarSystem { id: 4, name: "Bortras".into(), sector_id: 0, control: "Empire".into(),
                     popularity_alliance: 0.30, popularity_empire: 0.70 },
        StarSystem { id: 5, name: "Dagobah".into(), sector_id: 0, control: "Uncontrolled".into(),
                     popularity_alliance: 0.50, popularity_empire: 0.50 },
    ]
}

fn build_demo_characters() -> Vec<Character> {
    // IDs match the seed profiles in webapp/src/hooks/useGroupProfiles.ts
    vec![
        // Alliance majors
        major(0, "Mon Mothma", "Alliance", 1, 95, 70, 20, 90, 95),
        major(1, "Leia Organa", "Alliance", 1, 90, 80, 60, 85, 95),
        major(2, "Luke Skywalker", "Alliance", 1, 60, 70, 95, 80, 95),
        major(3, "Han Solo", "Alliance", 1, 50, 85, 80, 70, 85),
        // Empire majors
        major(4, "Emperor Palpatine", "Empire", 0, 85, 95, 60, 95, 95),
        major(5, "Darth Vader", "Empire", 0, 30, 75, 98, 90, 85),
        // Alliance minors
        minor(6, "Admiral Ackbar", "Alliance", 1, 85, 50, 70, 80, 90),
        minor(7, "Wedge Antilles", "Alliance", 1, 40, 60, 85, 70, 90),
        minor(8, "Lando Calrissian", "Alliance", 1, 75, 70, 60, 75, 70),
        minor(9, "Chewbacca", "Alliance", 1, 20, 80, 90, 50, 90),
        minor(10, "Jan Dodonna", "Alliance", 1, 60, 50, 70, 80, 85),
        // Empire minors
        minor(11, "Admiral Ozzel", "Empire", 0, 40, 60, 65, 70, 80),
        minor(12, "Admiral Piett", "Empire", 0, 45, 70, 70, 80, 90),
        minor(13, "General Grammel", "Empire", 0, 30, 65, 80, 70, 75),
        minor(14, "Grand Admiral Thrawn", "Empire", 0, 75, 90, 85, 95, 85),
        minor(15, "Admiral Daala", "Empire", 0, 40, 70, 80, 75, 70),
    ]
}

fn major(
    id: u32, name: &str, faction: &str, sys: u32,
    diplomacy: u32, espionage: u32, combat: u32, leadership: u32, loyalty: u32,
) -> Character {
    Character {
        id, name: name.into(), faction: faction.into(),
        is_major: true,
        on_mission: false, on_hidden_mission: false, is_captive: false,
        current_system_id: Some(sys),
        diplomacy: Skill { base: diplomacy, variance: 5 },
        espionage: Skill { base: espionage, variance: 5 },
        combat: Skill { base: combat, variance: 5 },
        leadership: Skill { base: leadership, variance: 5 },
        loyalty: Skill { base: loyalty, variance: 5 },
    }
}

fn minor(
    id: u32, name: &str, faction: &str, sys: u32,
    diplomacy: u32, espionage: u32, combat: u32, leadership: u32, loyalty: u32,
) -> Character {
    Character {
        id, name: name.into(), faction: faction.into(),
        is_major: false,
        on_mission: false, on_hidden_mission: false, is_captive: false,
        current_system_id: Some(sys),
        diplomacy: Skill { base: diplomacy, variance: 10 },
        espionage: Skill { base: espionage, variance: 10 },
        combat: Skill { base: combat, variance: 10 },
        leadership: Skill { base: leadership, variance: 10 },
        loyalty: Skill { base: loyalty, variance: 10 },
    }
}

// ──────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn reset() {
        ENGINE.with(|e| {
            let mut engine = e.borrow_mut();
            engine.current_day = 0;
            engine.characters.clear();
            engine.systems.clear();
            engine.missions.clear();
            engine.next_mission_id = 1;
        });
    }

    #[test]
    fn init_populates_world() {
        reset();
        init_demo_world();
        assert!(world_loaded());
        ENGINE.with(|e| {
            let e = e.borrow();
            assert_eq!(e.current_day, 192);
            assert_eq!(e.characters.len(), 16);
            assert_eq!(e.systems.len(), 6);
        });
    }

    #[test]
    fn dispatch_marks_character_on_mission_and_returns_id() {
        reset();
        init_demo_world();
        let id = ENGINE.with(|e| {
            // Han Solo (id=3), target Coruscant (id=0), Sabotage
            let mut engine = e.borrow_mut();
            let duration = synthetic_duration("Sabotage");
            let mission_id = engine.next_mission_id;
            engine.next_mission_id += 1;
            engine.missions.push(ActiveMission {
                id: mission_id,
                character_id: 3,
                character_name: "Han Solo".into(),
                kind: "Sabotage".into(),
                target_system_id: 0,
                target_system_name: "Coruscant".into(),
                ticks_remaining: duration,
                total_ticks: duration,
            });
            if let Some(c) = engine.characters.iter_mut().find(|c| c.id == 3) {
                c.on_mission = true;
            }
            mission_id
        });
        assert_eq!(id, 1);
        ENGINE.with(|e| {
            let e = e.borrow();
            let han = e.characters.iter().find(|c| c.id == 3).unwrap();
            assert!(han.on_mission);
            assert_eq!(e.missions.len(), 1);
        });
    }

    #[test]
    fn advance_decrements_and_frees() {
        reset();
        init_demo_world();
        // Dispatch a sabotage (12 days)
        ENGINE.with(|e| {
            let mut engine = e.borrow_mut();
            engine.missions.push(ActiveMission {
                id: 1, character_id: 3, character_name: "Han Solo".into(),
                kind: "Sabotage".into(), target_system_id: 0,
                target_system_name: "Coruscant".into(),
                ticks_remaining: 12, total_ticks: 12,
            });
            engine.characters[3].on_mission = true;
        });
        advance_days(12);
        ENGINE.with(|e| {
            let e = e.borrow();
            assert_eq!(e.current_day, 192 + 12);
            assert!(e.missions.is_empty());
            assert!(!e.characters[3].on_mission);
        });
    }
}
