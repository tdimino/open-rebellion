//! Runtime simulation types for the game world.
//!
//! These are the "layer 2" types that game logic, rendering, and save/load operate on.
//! They use slotmap keys for entity references (stable, arena-backed handles)
//! and rich enums for state rather than raw bytes.
//!
//! The `GameWorld` struct is the root of the entire simulation state.

use serde::{Deserialize, Serialize};

use crate::dat::ExplorationStatus;
use crate::ids::*;

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

/// A troop regiment stationed at a system — a deployed instance of a troop class.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TroopUnit {
    /// The class definition (from TROOPSD.DAT).
    pub class_dat_id: DatId,
    pub is_alliance: bool,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
}
