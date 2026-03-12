//! Binary-mirror structs matching .DAT file layouts.
//!
//! These types exist solely for data import/export. They preserve the exact
//! field names and semantics from the original binary format so that
//! dat-dumper can deserialize raw bytes into typed Rust without
//! making game-model decisions.
//!
//! Game logic should operate on the `world/` types instead.

use serde::{Deserialize, Serialize};

/// Faction alignment as encoded in .DAT files.
///
/// Many unit/character records carry an alliance or empire flag rather than
/// a richer ownership model — this enum captures that distinction cleanly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Faction {
    Alliance,
    Empire,
    /// Planets and some structures begin as neutral (no controlling faction).
    Neutral,
}

/// Galaxy size categories from the SECTORSD record header.
///
/// Encoded as a u8 in the file; values 1–3 map to these variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GalaxySize {
    Standard = 1,
    Large = 2,
    Huge = 3,
}

/// Sector group (galactic region) from SECTORSD records.
///
/// Encodes where in the galaxy the sector sits; affects strategic context
/// and initial population density.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SectorGroup {
    /// Core worlds — high population, heavily contested.
    Core = 1,
    /// Inner rim — moderate density.
    RimInner = 2,
    /// Outer rim — sparse, strategic outposts.
    RimOuter = 3,
}

/// Exploration status derived from the SYSTEMSD family_id byte.
///
/// Explored systems are fully visible; unexplored systems reveal name only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExplorationStatus {
    /// family_id = 0x90 (144)
    Explored,
    /// family_id = 0x92 (146)
    Unexplored,
}
