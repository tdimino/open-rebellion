//! Runtime simulation types for the game world.
//!
//! These are the "layer 2" types that game logic, rendering, and save/load operate on.
//! They use slotmap keys for entity references (stable, arena-backed handles)
//! and rich enums for state rather than raw bytes.
//!
//! The `GameWorld` struct is the root of the entire simulation state.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::dat::ExplorationStatus;
use crate::ids::*;

/// Force sensitivity tier for a character.
///
/// Maps to the 2-bit value at `entity[9] >> 6 & 3` in REBEXE.EXE's C++ layout:
/// 0=None/Low, 1=Aware (ForcePotential tier), 2=Training (ForceTraining tier),
/// 3=Experienced (ForceExperience tier).
///
/// Characters start as `None`. Those with `jedi_probability > 0` may advance
/// through tiers via the Jedi training system (`jedi.rs`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ForceTier {
    /// No Force sensitivity detected.
    None = 0,
    /// Force potential recognized — character is Force-aware but untrained.
    Aware = 1,
    /// Actively training in the Force.
    Training = 2,
    /// Full Jedi Knight / Sith Lord tier. Maximum Force capability.
    Experienced = 3,
}

impl Default for ForceTier {
    fn default() -> Self {
        ForceTier::None
    }
}

/// A star system in the galaxy — the atomic unit of territory and production.
///
/// Systems belong to a sector and hold all surface assets (facilities, ground units)
/// as well as the fleets in orbit or departing from them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct System {
    /// Original .DAT identifier, preserved for round-trip serialization.
    pub dat_id: DatId,
    pub name: String,
    /// The sector this system belongs to.
    pub sector: SectorKey,
    /// Galactic map X coordinate (in sector-relative units).
    pub x: u16,
    /// Galactic map Y coordinate (in sector-relative units).
    pub y: u16,
    /// Whether this system has been explored (from SYSTEMSD family_id).
    /// Unexplored systems reveal name only; facilities and units are hidden.
    pub exploration_status: ExplorationStatus,
    /// Alliance popularity fraction in [0.0, 1.0].
    pub popularity_alliance: f32,
    /// Empire popularity fraction in [0.0, 1.0].
    pub popularity_empire: f32,
    /// Fleets currently orbiting or departing from this system.
    pub fleets: Vec<FleetKey>,
    /// Ground troop units stationed on the surface.
    pub ground_units: Vec<TroopKey>,
    /// Special forces units assigned to this system.
    pub special_forces: Vec<SpecialForceKey>,
    /// Planetary shields, turbolaser batteries, and similar fixed defenses.
    pub defense_facilities: Vec<DefenseFacilityKey>,
    /// Shipyards and troop training centers.
    pub manufacturing_facilities: Vec<ManufacturingFacilityKey>,
    /// Mines, refineries, and other resource extractors.
    pub production_facilities: Vec<ProductionFacilityKey>,
    /// True if this system is a faction's headquarters (from SYSTEMSD IsHeadquarters flag).
    ///
    /// Capturing the enemy HQ is the primary victory condition.
    pub is_headquarters: bool,
    /// True if this system's planet has been destroyed (Death Star fired; `alive_flag` bit0 == 0).
    ///
    /// From RE: the Death Star fires when the target's alive_flag bit0 == 0 — inverted from
    /// normal combat units. A destroyed planet cannot produce resources or be colonized.
    pub is_destroyed: bool,
    /// Which faction currently controls this system, or `None` for unclaimed/contested.
    ///
    /// Derived from the 2-bit faction_side field (`entity+0x24 bits 6-7`):
    /// 0 = neutral, 1 = Alliance, 2 = Empire, 3 = contested.
    pub controlling_faction: Option<crate::dat::Faction>,
}

/// A sector — a named galactic region containing multiple star systems.
///
/// Sectors are the strategic layer above systems: capturing a sector
/// shifts diplomatic and morale values across all its systems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sector {
    /// Original .DAT identifier.
    pub dat_id: DatId,
    pub name: String,
    /// Galactic region (Core, Inner Rim, Outer Rim).
    pub group: crate::dat::SectorGroup,
    /// Map X coordinate of the sector's representative position.
    pub x: u16,
    /// Map Y coordinate of the sector's representative position.
    pub y: u16,
    /// All systems within this sector.
    pub systems: Vec<SystemKey>,
}

/// Class definition for a capital ship — a template, not a unit instance.
///
/// Instances (actual ships in a fleet) would carry a `CapitalShipKey` back to
/// this definition. For now the model records class data; instance counts
/// live in fleets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapitalShipClass {
    pub dat_id: DatId,
    pub name: String,
    /// True if this class is buildable/usable by the Rebel Alliance.
    pub is_alliance: bool,
    /// True if this class is buildable/usable by the Empire.
    pub is_empire: bool,
    pub refined_material_cost: u32,
    pub maintenance_cost: u32,
    /// Position in the tech tree (lower = earlier unlock).
    pub research_order: u32,
    pub research_difficulty: u32,
    pub hull: u32,
    pub shield_strength: u32,
    pub sub_light_engine: u32,
    pub maneuverability: u32,
    pub hyperdrive: u32,
    /// Number of fighter squadrons this ship can carry.
    pub fighter_capacity: u32,
    /// Number of troop units this ship can transport.
    pub troop_capacity: u32,

    // ── Combat stats (from CAPSHPSD.DAT — needed for War Machine) ────────────

    /// Sensor range for detecting enemy units.
    pub detection: u32,
    /// Turbolaser batteries per arc (fore/aft/port/starboard).
    pub turbolaser_fore: u32,
    pub turbolaser_aft: u32,
    pub turbolaser_port: u32,
    pub turbolaser_starboard: u32,
    /// Ion cannon batteries per arc.
    pub ion_cannon_fore: u32,
    pub ion_cannon_aft: u32,
    pub ion_cannon_port: u32,
    pub ion_cannon_starboard: u32,
    /// Laser cannon batteries per arc.
    pub laser_cannon_fore: u32,
    pub laser_cannon_aft: u32,
    pub laser_cannon_port: u32,
    pub laser_cannon_starboard: u32,
    /// Shield recharge rate per combat round.
    pub shield_recharge_rate: u32,
    /// Hull repair rate per combat round.
    pub damage_control: u32,
    /// Orbital bombardment attack stat (used in bombardment formula §4).
    pub bombardment_modifier: u32,
}

/// Class definition for a fighter squadron — template, not an instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FighterClass {
    pub dat_id: DatId,
    pub name: String,
    pub is_alliance: bool,
    pub is_empire: bool,
    pub refined_material_cost: u32,
    pub maintenance_cost: u32,
    /// Number of individual craft in one squadron.
    pub squadron_size: u32,
    pub torpedoes: u32,
    /// Fighter attack stat for combat resolution.
    pub overall_attack_strength: u32,
    /// Bombardment defense modifier.
    pub bombardment_defense: u32,
}

/// A character — either a named major hero/villain or a generic minor character.
///
/// Characters can be assigned as admirals, generals, or diplomats.
/// Their skills are stored as `SkillPair` (base + variance) to support
/// both fixed major characters and procedurally-generated minors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub dat_id: DatId,
    pub name: String,
    pub is_alliance: bool,
    pub is_empire: bool,
    /// Major characters (Luke, Vader, etc.) have fixed identities;
    /// minor characters are generic and reusable.
    pub is_major: bool,
    pub diplomacy: SkillPair,
    pub espionage: SkillPair,
    pub ship_design: SkillPair,
    pub troop_training: SkillPair,
    pub facility_design: SkillPair,
    pub combat: SkillPair,
    pub leadership: SkillPair,
    pub loyalty: SkillPair,
    /// Probability (0–100) this character becomes Force-sensitive.
    pub jedi_probability: u32,
    pub jedi_level: SkillPair,
    pub can_be_admiral: bool,
    pub can_be_commander: bool,
    pub can_be_general: bool,

    // ── Force / Jedi fields (entity-system.md §1.3) ───────────────────────────

    /// Current Force sensitivity tier (None → Aware → Training → Experienced).
    /// Driven by `jedi.rs` JediSystem.
    #[serde(default)]
    pub force_tier: ForceTier,
    /// Accumulated Force experience points. Increments via Jedi training missions;
    /// threshold crossings trigger tier advancement.
    #[serde(default)]
    pub force_experience: u32,
    /// True once the opposing faction has discovered this character's Force ability.
    /// Maps to `!(entity[0x1e] & 1)` in REBEXE.EXE — initially hidden.
    #[serde(default)]
    pub is_discovered_jedi: bool,

    // ── DAT-promoted fields ─────────────────────────────────────────────────

    /// Immune to betrayal missions (Luke, Vader). From MJCHARSD.DAT `is_unable_to_betray`.
    #[serde(default)]
    pub is_unable_to_betray: bool,
    /// Can train other Jedi (Yoda). From MJCHARSD.DAT `is_jedi_trainer`.
    #[serde(default)]
    pub is_jedi_trainer: bool,
    /// Publicly known Force user. From MJCHARSD.DAT `is_known_jedi`.
    #[serde(default)]
    pub is_known_jedi: bool,
    /// Fleet speed bonus (Han Solo). Default 0.
    #[serde(default)]
    pub hyperdrive_modifier: i16,
    /// Mission bonus loyalty, 0-100. Default 0.
    #[serde(default)]
    pub enhanced_loyalty: i16,
    /// Currently assigned to a mission.
    #[serde(default)]
    pub on_mission: bool,
    /// Mission concealed from opponent.
    #[serde(default)]
    pub on_hidden_mission: bool,
    /// Story-forced assignment, blocks resignation.
    #[serde(default)]
    pub on_mandatory_mission: bool,

    // ── Captivity ─────────────────────────────────────────────────────────

    /// Faction that captured this character (None if free).
    #[serde(default)]
    pub captured_by: Option<crate::dat::Faction>,
    /// Tick when character was captured (for escape timing).
    #[serde(default)]
    pub capture_tick: Option<u64>,
    /// True if character is currently held captive.
    #[serde(default)]
    pub is_captive: bool,

    // ── Location tracking ───────────────────────────────────────────────────

    /// System where this character is currently located.
    #[serde(default)]
    pub current_system: Option<SystemKey>,
    /// Fleet this character is currently assigned to.
    #[serde(default)]
    pub current_fleet: Option<FleetKey>,
}

/// A base value paired with a random variance for character skill generation.
///
/// Final skill = `base + rng(0..=variance)` at scenario start.
/// Major characters typically use `variance = 0` to lock their stats.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SkillPair {
    pub base: u32,
    pub variance: u32,
}

/// A fleet — a collection of ships and characters at a system location.
///
/// Fleets are the primary unit of strategic movement. They orbit systems,
/// travel hyperlanes, and carry characters in command roles.
///
/// Ship and fighter slots are stored as `(class_key, count)` pairs so that
/// multiple instances of the same class can be represented without duplicating
/// the class record in the arena.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fleet {
    /// System the fleet is currently orbiting or departing from.
    pub location: SystemKey,
    /// Capital ship class references with counts.  One entry per unique class in
    /// this fleet; `count` indicates how many hulls of that class are present.
    pub capital_ships: Vec<ShipEntry>,
    /// Fighter squadron class references with counts.
    pub fighters: Vec<FighterEntry>,
    /// Characters assigned to this fleet (admiral, general, etc.).
    pub characters: Vec<CharacterKey>,
    /// True if this fleet belongs to the Rebel Alliance; false = Empire.
    pub is_alliance: bool,
    /// True if this fleet contains a Death Star (family `0x34`).
    ///
    /// Enables the Death Star win-condition check in `VictorySystem`.
    /// Set by `rebellion-data` when loading fleet composition from CAPSHPSD.
    pub has_death_star: bool,
}

/// One entry in a fleet's capital-ship roster: a class plus the number of hulls.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipEntry {
    pub class: CapitalShipKey,
    pub count: u32,
}

/// One entry in a fleet's fighter roster: a class plus the number of squadrons.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FighterEntry {
    pub class: FighterKey,
    pub count: u32,
}

/// A single hull of a capital ship — runtime per-hull combat state.
///
/// Distinct from `ShipEntry` which is a (class, count) summary for fleets.
/// `ShipInstance` is the unit-level record used by the combat resolver.
///
/// Mirrors the C++ entity object fields confirmed by Ghidra:
/// - `hull_current` → offset +0x60 (int)
/// - `shield_weapon_packed` → offset +0x64 (bits 0-3 = shield, 4-7 = weapon)
/// - `alive` → offset +0xac bit0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipInstance {
    /// Reference to the class template in `GameWorld::capital_ship_classes`.
    pub class: CapitalShipKey,
    /// Current hull. Starts at `CapitalShipClass::hull`, reduced by combat.
    pub hull_current: i32,
    /// Packed nibbles: bits 0-3 = shield_recharge_allocated, bits 4-7 = weapon_recharge_allocated.
    /// The C++ binary uses XOR-mask writes `(new ^ old) & 0xf ^ old` — functionally a nibble store.
    pub shield_weapon_packed: u8,
    /// True while hull_current > 0 and the ship has not been destroyed.
    pub alive: bool,
    /// True = Rebel Alliance; false = Empire.
    pub faction_is_alliance: bool,
}

impl ShipInstance {
    /// Shield recharge allocation nibble (bits 0-3).
    pub fn shield_nibble(&self) -> u8 {
        self.shield_weapon_packed & 0x0f
    }

    /// Weapon recharge allocation nibble (bits 4-7).
    pub fn weapon_nibble(&self) -> u8 {
        (self.shield_weapon_packed >> 4) & 0x0f
    }
}

/// Game-balance parameters loaded from GNPRTB.DAT.
///
/// Each entry in GNPRTB.DAT has a `parameter_id` (0-212) and 8 i32 values
/// keyed by difficulty/faction mode. The `value()` accessor returns the
/// appropriate value for a given difficulty index (0-7).
///
/// Difficulty index mapping (8 levels, from the DAT binary format):
///   0 = development
///   1 = Alliance SP Easy (`alliance_sp_easy`)
///   2 = Alliance SP Medium (`alliance_sp_medium`)
///   3 = Alliance SP Hard (`alliance_sp_hard`)
///   4 = Empire SP Easy (`empire_sp_easy`)
///   5 = Empire SP Medium (`empire_sp_medium`)
///   6 = Empire SP Hard (`empire_sp_hard`)
///   7 = Multiplayer
///
/// The C++ `difficulty_packed` at offset +0x24 bits 4-5 is a 2-bit selector
/// (0-3) used by `FUN_004fd600` to pick Alliance(1) or Empire(2). The full
/// 8-level index is computed at the caller level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GnprtbParams {
    /// All 213 entries, indexed by `parameter_id`.
    entries: Vec<GnprtbEntry>,
}

/// One entry from GNPRTB.DAT.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GnprtbEntry {
    pub parameter_id: u32,
    pub development: i32,
    pub alliance_sp_easy: i32,
    pub alliance_sp_medium: i32,
    pub alliance_sp_hard: i32,
    pub empire_sp_easy: i32,
    pub empire_sp_medium: i32,
    pub empire_sp_hard: i32,
    pub multiplayer: i32,
}

impl Default for GnprtbParams {
    fn default() -> Self {
        Self { entries: Vec::new() }
    }
}

impl GnprtbParams {
    /// Construct from raw entries (called by `rebellion-data` loader).
    pub fn new(entries: Vec<GnprtbEntry>) -> Self {
        Self { entries }
    }

    /// Return the parameter value for `param_id` at `difficulty`.
    ///
    /// `difficulty`: 0=development, 1=alliance_easy, 2=alliance_medium, 3=alliance_hard,
    ///               4=empire_easy, 5=empire_medium, 6=empire_hard, 7=multiplayer.
    /// Returns 0 if `param_id` is out of range.
    pub fn value(&self, param_id: u16, difficulty: u8) -> i32 {
        self.entries
            .iter()
            .find(|e| e.parameter_id == param_id as u32)
            .map(|e| match difficulty {
                0 => e.development,
                1 => e.alliance_sp_easy,
                2 => e.alliance_sp_medium,
                3 => e.alliance_sp_hard,
                4 => e.empire_sp_easy,
                5 => e.empire_sp_medium,
                6 => e.empire_sp_hard,
                _ => e.multiplayer,
            })
            .unwrap_or(0)
    }
}

/// A lookup table loaded from one of the `*MSTB.DAT` / `*TB.DAT` files.
///
/// Each table is a sorted list of `(threshold, value)` pairs where `threshold`
/// is a signed skill delta (negative = below average, 0 = average, positive =
/// above average). `lookup()` performs linear interpolation between the two
/// bracketing entries, matching the C++ table-lookup function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MstbTable {
    /// Entries sorted ascending by threshold.
    entries: Vec<MstbEntry>,
}

/// One row in an `MstbTable`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MstbEntry {
    pub threshold: i32,
    pub value: u32,
}

impl MstbTable {
    /// Construct from raw `(threshold, value)` pairs. Sorts by threshold.
    pub fn new(mut entries: Vec<MstbEntry>) -> Self {
        entries.sort_by_key(|e| e.threshold);
        Self { entries }
    }

    /// Look up the value for `skill_score` using linear interpolation.
    ///
    /// - If `skill_score` is below the lowest threshold, returns the lowest value.
    /// - If `skill_score` is above the highest threshold, returns the highest value.
    /// - Otherwise interpolates between the two bracketing entries.
    pub fn lookup(&self, skill_score: i32) -> u32 {
        if self.entries.is_empty() {
            return 0;
        }
        // Below minimum
        if skill_score <= self.entries[0].threshold {
            return self.entries[0].value;
        }
        // Above maximum
        let last = self.entries.last().unwrap();
        if skill_score >= last.threshold {
            return last.value;
        }
        // Find bracketing pair
        for i in 0..self.entries.len() - 1 {
            let lo = &self.entries[i];
            let hi = &self.entries[i + 1];
            if skill_score >= lo.threshold && skill_score < hi.threshold {
                let span = hi.threshold - lo.threshold;
                if span == 0 {
                    return lo.value;
                }
                let frac = (skill_score - lo.threshold) as f64 / span as f64;
                let interpolated = lo.value as f64 + frac * (hi.value as i64 - lo.value as i64) as f64;
                return interpolated.round().max(0.0) as u32;
            }
        }
        last.value
    }
}

/// A troop regiment stationed at a system — a deployed instance of a troop class.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TroopUnit {
    /// The class definition (from TROOPSD.DAT).
    pub class_dat_id: DatId,
    pub is_alliance: bool,
    /// Current regiment strength (C++ offset +0x96). Starts at class max, reduced by ground combat.
    pub regiment_strength: i16,
}

/// A special-forces unit stationed at a system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialForceUnit {
    /// The class definition (from SPECFCSD.DAT).
    pub class_dat_id: DatId,
    pub is_alliance: bool,
}

/// A defense facility instance on a system surface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefenseFacilityInstance {
    /// The class definition (from DEFFACSD.DAT).
    pub class_dat_id: DatId,
    pub is_alliance: bool,
}

/// A manufacturing facility instance (shipyard, training center, construction yard).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManufacturingFacilityInstance {
    /// The class definition (from MANFACSD.DAT).
    pub class_dat_id: DatId,
    pub is_alliance: bool,
}

/// A production facility instance (mine, refinery).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionFacilityInstance {
    /// The class definition (from PROFACSD.DAT).
    pub class_dat_id: DatId,
    pub is_alliance: bool,
}

/// The complete game world state — the root of all simulation data.
///
/// All entity arenas live here. Cross-entity references use slotmap keys;
/// they're meaningless outside the arena they index into.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameWorld {
    pub systems: slotmap::SlotMap<SystemKey, System>,
    pub sectors: slotmap::SlotMap<SectorKey, Sector>,
    pub capital_ship_classes: slotmap::SlotMap<CapitalShipKey, CapitalShipClass>,
    pub fighter_classes: slotmap::SlotMap<FighterKey, FighterClass>,
    pub characters: slotmap::SlotMap<CharacterKey, Character>,
    pub fleets: slotmap::SlotMap<FleetKey, Fleet>,
    /// Deployed troop regiments (instances, not class definitions).
    pub troops: slotmap::SlotMap<TroopKey, TroopUnit>,
    /// Deployed special-forces units.
    pub special_forces: slotmap::SlotMap<SpecialForceKey, SpecialForceUnit>,
    /// Defense facilities on system surfaces.
    pub defense_facilities: slotmap::SlotMap<DefenseFacilityKey, DefenseFacilityInstance>,
    /// Manufacturing facilities (shipyards, training centers, construction yards).
    pub manufacturing_facilities: slotmap::SlotMap<ManufacturingFacilityKey, ManufacturingFacilityInstance>,
    /// Production facilities (mines, refineries).
    pub production_facilities: slotmap::SlotMap<ProductionFacilityKey, ProductionFacilityInstance>,
    /// Game-balance parameters from GNPRTB.DAT (combat formulas, bombardment divisors, etc.).
    pub gnprtb: GnprtbParams,
    /// Mission probability tables keyed by DAT file stem (e.g. "DIPLMSTB", "ESPIMSTB").
    pub mission_tables: HashMap<String, MstbTable>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: create a minimal Character with all fields at their Default values.
    fn default_character() -> Character {
        Character {
            dat_id: DatId::new(0),
            name: "Test".into(),
            is_alliance: false,
            is_empire: false,
            is_major: false,
            diplomacy: SkillPair { base: 0, variance: 0 },
            espionage: SkillPair { base: 0, variance: 0 },
            ship_design: SkillPair { base: 0, variance: 0 },
            troop_training: SkillPair { base: 0, variance: 0 },
            facility_design: SkillPair { base: 0, variance: 0 },
            combat: SkillPair { base: 0, variance: 0 },
            leadership: SkillPair { base: 0, variance: 0 },
            loyalty: SkillPair { base: 0, variance: 0 },
            jedi_probability: 0,
            jedi_level: SkillPair { base: 0, variance: 0 },
            can_be_admiral: false,
            can_be_commander: false,
            can_be_general: false,
            force_tier: ForceTier::None,
            force_experience: 0,
            is_discovered_jedi: false,
            is_unable_to_betray: false,
            is_jedi_trainer: false,
            is_known_jedi: false,
            hyperdrive_modifier: 0,
            enhanced_loyalty: 0,
            on_mission: false,
            on_hidden_mission: false,
            on_mandatory_mission: false,
            captured_by: None,
            capture_tick: None,
            is_captive: false,
            current_system: None,
            current_fleet: None,
        }
    }

    #[test]
    fn character_is_unable_to_betray_serde_roundtrip() {
        let mut c = default_character();
        c.is_unable_to_betray = true;
        let json = serde_json::to_string(&c).unwrap();
        let c2: Character = serde_json::from_str(&json).unwrap();
        assert!(c2.is_unable_to_betray);
    }

    #[test]
    fn captive_character_serde_roundtrip() {
        let mut c = default_character();
        c.is_captive = true;
        c.captured_by = Some(crate::dat::Faction::Empire);
        c.capture_tick = Some(42);
        let json = serde_json::to_string(&c).unwrap();
        let c2: Character = serde_json::from_str(&json).unwrap();
        assert!(c2.is_captive);
        assert_eq!(c2.captured_by, Some(crate::dat::Faction::Empire));
        assert_eq!(c2.capture_tick, Some(42));
    }

    #[test]
    fn default_character_new_fields_are_zero_false_none() {
        let c = default_character();
        assert!(!c.is_unable_to_betray);
        assert!(!c.is_jedi_trainer);
        assert!(!c.is_known_jedi);
        assert_eq!(c.hyperdrive_modifier, 0);
        assert_eq!(c.enhanced_loyalty, 0);
        assert!(!c.on_mission);
        assert!(!c.on_hidden_mission);
        assert!(!c.on_mandatory_mission);
        assert!(c.current_system.is_none());
        assert!(c.current_fleet.is_none());
    }

    #[test]
    fn is_known_jedi_implies_aware_tier_convention() {
        // Verifies the convention: is_known_jedi should pair with ForceTier::Aware
        // (enforced in convert_character, tested here as a struct invariant).
        let mut c = default_character();
        c.is_known_jedi = true;
        c.force_tier = ForceTier::Aware;
        assert_eq!(c.force_tier, ForceTier::Aware);
        assert!(c.is_known_jedi);
    }

    #[test]
    fn current_system_and_fleet_default_to_none() {
        let c = default_character();
        assert_eq!(c.current_system, None);
        assert_eq!(c.current_fleet, None);
    }

    #[test]
    fn hyperdrive_modifier_defaults_to_zero() {
        let c = default_character();
        assert_eq!(c.hyperdrive_modifier, 0);
    }

    #[test]
    fn serde_backward_compat_missing_new_fields() {
        // Simulate deserializing a save file that lacks the new fields.
        let json = r#"{
            "dat_id": 0,
            "name": "Old Save Luke",
            "is_alliance": true,
            "is_empire": false,
            "is_major": true,
            "diplomacy": {"base": 80, "variance": 0},
            "espionage": {"base": 60, "variance": 0},
            "ship_design": {"base": 40, "variance": 0},
            "troop_training": {"base": 50, "variance": 0},
            "facility_design": {"base": 30, "variance": 0},
            "combat": {"base": 90, "variance": 0},
            "leadership": {"base": 85, "variance": 0},
            "loyalty": {"base": 95, "variance": 0},
            "jedi_probability": 100,
            "jedi_level": {"base": 80, "variance": 0},
            "can_be_admiral": true,
            "can_be_commander": true,
            "can_be_general": true
        }"#;
        let c: Character = serde_json::from_str(json).unwrap();
        // All new fields should default gracefully
        assert!(!c.is_unable_to_betray);
        assert!(!c.is_jedi_trainer);
        assert!(!c.is_known_jedi);
        assert_eq!(c.hyperdrive_modifier, 0);
        assert_eq!(c.enhanced_loyalty, 0);
        assert!(!c.on_mission);
        assert!(!c.on_hidden_mission);
        assert!(!c.on_mandatory_mission);
        assert!(c.current_system.is_none());
        assert!(c.current_fleet.is_none());
        assert_eq!(c.force_tier, ForceTier::None);
    }
}
