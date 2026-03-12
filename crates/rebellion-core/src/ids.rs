use serde::{Deserialize, Serialize};
use slotmap::new_key_type;

/// Newtype preserving the original compound ID from .DAT files.
///
/// Encoding: `(family_byte << 24) | sequential_index`
///
/// The family byte identifies the entity class (e.g. 0x90 = system, 0x92 = sector).
/// The lower 24 bits are the sequential index within that family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DatId(pub u32);

impl DatId {
    /// Construct a `DatId` from a raw u32 read out of a .DAT file.
    pub fn new(raw: u32) -> Self {
        Self(raw)
    }

    /// Return the raw u32 value.
    pub fn raw(self) -> u32 {
        self.0
    }

    /// The high byte — identifies the entity class.
    pub fn family(self) -> u8 {
        (self.0 >> 24) as u8
    }

    /// The lower 24 bits — sequential index within the family.
    pub fn index(self) -> u32 {
        self.0 & 0x00FF_FFFF
    }
}

impl std::fmt::Display for DatId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DatId({:#010x})", self.0)
    }
}

// Slotmap keys for runtime entity references.
// These are the stable handles used throughout the world layer.
// They are NOT the same as DatId — they're runtime-only arena keys.
new_key_type! {
    /// Handle to a star system in the world arena.
    pub struct SystemKey;

    /// Handle to a sector (multi-system region) in the world arena.
    pub struct SectorKey;

    /// Handle to a capital ship class definition.
    pub struct CapitalShipKey;

    /// Handle to a fighter class definition.
    pub struct FighterKey;

    /// Handle to a ground troop unit.
    pub struct TroopKey;

    /// Handle to a special forces unit.
    pub struct SpecialForceKey;

    /// Handle to a character (major or minor).
    pub struct CharacterKey;

    /// Handle to a defense facility on a planet.
    pub struct DefenseFacilityKey;

    /// Handle to a manufacturing facility (ship/troop production).
    pub struct ManufacturingFacilityKey;

    /// Handle to a resource production facility.
    pub struct ProductionFacilityKey;

    /// Handle to a fleet (collection of ships at a location).
    pub struct FleetKey;
}
