//! Unified game effect algebra for the Open Souls functional simulation model.
//!
//! Every simulation system produces `Vec<GameEffect>` as its sole output.
//! The `PerceptionIntegrator` (in rebellion-data) applies effects to the
//! `GameWorld`, derives telemetry, and manages transaction rollback.
//!
//! Design: closed enum (not trait objects) for deterministic serialization,
//! exhaustive match, and structural diffing in the autoresearch eval loop.

use serde::{Deserialize, Serialize};

use crate::combat::CombatSide;
use crate::dat::Faction;
// EventAction not used directly — effects carry event_id only.
// The integrator looks up actions from the EventState when applying.
use crate::ids::*;
use crate::manufacturing::BuildableKind;
use crate::missions::{MissionFaction, MissionKind, MissionOutcome};
use crate::research::TechType;
use crate::victory::VictoryOutcome;
use crate::world::{ControlKind, ForceTier};

// ---------------------------------------------------------------------------
// Effect phase ordering
// ---------------------------------------------------------------------------

/// Phase ordering for effect application.
/// Effects within a phase are applied in production order.
/// Effects across phases are applied in discriminant order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum EffectPhase {
    /// Resource production, support drift, collection rates
    Economy = 0,
    /// Build completions, queue advancement
    Manufacturing = 1,
    /// Fleet arrivals, transit updates
    Movement = 2,
    /// Space combat, ground combat, bombardment
    Combat = 3,
    /// Missions, espionage, diplomacy, recruitment
    Diplomacy = 4,
    /// Fog reveals, blockade changes, intelligence
    Intelligence = 5,
    /// AI actions, research, Jedi, events, betrayal, uprising
    Command = 6,
    /// Death Star, victory checks, campaign snapshots
    Endgame = 7,
}

// ---------------------------------------------------------------------------
// Unified game effect enum
// ---------------------------------------------------------------------------

/// A unified game effect. Closed enum — every possible world mutation is a variant.
///
/// Principles enforced:
/// - Effects are values, not instructions (Principle 2)
/// - Effects are serializable and deterministic (Principle 8)
/// - Every effect carries enough data for inversion (Principle 3)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameEffect {
    // ── Economy (Phase 0) ────────────────────────────────────────────
    SupportDrifted {
        system: SystemKey,
        alliance_delta: f32,
        empire_delta: f32,
    },
    CollectionRateChanged {
        system: SystemKey,
        new_rate: f32,
    },
    GarrisonRequirementChanged {
        system: SystemKey,
        new_requirement: u32,
    },
    MaintenanceShortfall {
        system: SystemKey,
        deficit: u32,
    },

    // ── Manufacturing (Phase 1) ──────────────────────────────────────
    BuildCompleted {
        system: SystemKey,
        kind: BuildableKind,
    },

    // ── Movement (Phase 2) ───────────────────────────────────────────
    FleetArrived {
        fleet: FleetKey,
        origin: SystemKey,
        destination: SystemKey,
    },

    // ── Combat (Phase 3) ─────────────────────────────────────────────
    SpaceBattleResolved {
        system: SystemKey,
        winner: CombatSide,
    },
    GroundBattleResolved {
        system: SystemKey,
        winner: CombatSide,
    },
    BombardmentApplied {
        system: SystemKey,
        damage: i32,
    },
    ShipRepaired {
        fleet: FleetKey,
        ship_index: usize,
        hull_before: u32,
        hull_after: u32,
    },

    // ── Diplomacy (Phase 4) ──────────────────────────────────────────
    PopularityShifted {
        system: SystemKey,
        alliance_delta: f32,
        empire_delta: f32,
    },
    ControlChanged {
        system: SystemKey,
        from: ControlKind,
        to: ControlKind,
    },
    MissionResolved {
        kind: MissionKind,
        faction: MissionFaction,
        outcome: MissionOutcome,
        target_system: SystemKey,
    },
    DecoyTriggered {
        system: SystemKey,
        penalty_pct: u32,
    },
    CharacterCaptured {
        character: CharacterKey,
        captor_faction: Faction,
    },
    CharacterEscaped {
        character: CharacterKey,
        escaped_to_alliance: bool,
    },
    CharacterRecruited {
        character: CharacterKey,
        to_faction: MissionFaction,
    },
    CharacterKilled {
        character: CharacterKey,
    },
    FacilitySabotaged {
        system: SystemKey,
    },
    UprisingStarted {
        system: SystemKey,
    },
    UprisingSubdued {
        system: SystemKey,
    },

    // ── Intelligence (Phase 5) ───────────────────────────────────────
    SystemRevealed {
        system: SystemKey,
        faction: Faction,
    },
    BlockadeStarted {
        system: SystemKey,
    },
    BlockadeEnded {
        system: SystemKey,
    },

    // ── Command (Phase 6) ────────────────────────────────────────────
    AiFleetMoved {
        fleet: FleetKey,
        to_system: SystemKey,
    },
    AiMissionDispatched {
        kind: MissionKind,
        character: CharacterKey,
        target_system: SystemKey,
    },
    AiProductionEnqueued {
        system: SystemKey,
        kind: BuildableKind,
    },
    AiResearchDispatched {
        character: CharacterKey,
        tech_type: TechType,
    },
    TechUnlocked {
        faction_is_alliance: bool,
        tech_type: TechType,
        new_level: u32,
    },
    JediTierAdvanced {
        character: CharacterKey,
        new_tier: ForceTier,
    },
    JediDiscovered {
        character: CharacterKey,
    },
    EventFired {
        event_id: u32,
    },
    CharacterBetrayed {
        character: CharacterKey,
        defected_to_alliance: bool,
    },
    UprisingIncident {
        system: SystemKey,
    },
    // SkillModified intentionally deferred — needs a SkillKind discriminant
    // to identify which of the 8 skill pairs changed. Will be added when
    // the PerceptionIntegrator applies skill effects.

    // ── Endgame (Phase 7) ────────────────────────────────────────────
    DeathStarConstructionProgress {
        system: SystemKey,
    },
    PlanetDestroyed {
        system: SystemKey,
    },
    DeathStarShieldStatus {
        active: bool,
    },
    VictoryReached {
        outcome: VictoryOutcome,
    },
    CampaignSnapshot {
        tick: u64,
        alliance_systems: u32,
        empire_systems: u32,
        neutral_systems: u32,
    },
}

impl GameEffect {
    /// Which phase this effect belongs to, for ordering.
    pub fn phase(&self) -> EffectPhase {
        match self {
            Self::SupportDrifted { .. }
            | Self::CollectionRateChanged { .. }
            | Self::GarrisonRequirementChanged { .. }
            | Self::MaintenanceShortfall { .. } => EffectPhase::Economy,

            Self::BuildCompleted { .. } => EffectPhase::Manufacturing,

            Self::FleetArrived { .. } => EffectPhase::Movement,

            Self::SpaceBattleResolved { .. }
            | Self::GroundBattleResolved { .. }
            | Self::BombardmentApplied { .. }
            | Self::ShipRepaired { .. } => EffectPhase::Combat,

            Self::PopularityShifted { .. }
            | Self::ControlChanged { .. }
            | Self::MissionResolved { .. }
            | Self::DecoyTriggered { .. }
            | Self::CharacterCaptured { .. }
            | Self::CharacterEscaped { .. }
            | Self::CharacterRecruited { .. }
            | Self::CharacterKilled { .. }
            | Self::FacilitySabotaged { .. }
            | Self::UprisingStarted { .. }
            | Self::UprisingSubdued { .. } => EffectPhase::Diplomacy,

            Self::SystemRevealed { .. }
            | Self::BlockadeStarted { .. }
            | Self::BlockadeEnded { .. } => EffectPhase::Intelligence,

            Self::AiFleetMoved { .. }
            | Self::AiMissionDispatched { .. }
            | Self::AiProductionEnqueued { .. }
            | Self::AiResearchDispatched { .. }
            | Self::TechUnlocked { .. }
            | Self::JediTierAdvanced { .. }
            | Self::JediDiscovered { .. }
            | Self::EventFired { .. }
            | Self::CharacterBetrayed { .. }
            | Self::UprisingIncident { .. } => EffectPhase::Command,

            Self::DeathStarConstructionProgress { .. }
            | Self::PlanetDestroyed { .. }
            | Self::DeathStarShieldStatus { .. }
            | Self::VictoryReached { .. }
            | Self::CampaignSnapshot { .. } => EffectPhase::Endgame,
        }
    }

    /// Produce the inverse effect for undo/rollback.
    /// Returns None for effects that are inherently irreversible
    /// or whose inverse requires world state not captured in the effect.
    pub fn invert(&self) -> Option<GameEffect> {
        match self {
            Self::PopularityShifted {
                system,
                alliance_delta,
                empire_delta,
            } => Some(Self::PopularityShifted {
                system: *system,
                alliance_delta: -alliance_delta,
                empire_delta: -empire_delta,
            }),
            Self::SupportDrifted {
                system,
                alliance_delta,
                empire_delta,
            } => Some(Self::SupportDrifted {
                system: *system,
                alliance_delta: -alliance_delta,
                empire_delta: -empire_delta,
            }),
            Self::ShipRepaired {
                fleet,
                ship_index,
                hull_before,
                hull_after,
            } => Some(Self::ShipRepaired {
                fleet: *fleet,
                ship_index: *ship_index,
                hull_before: *hull_after,
                hull_after: *hull_before,
            }),
            Self::ControlChanged { system, from, to } => Some(Self::ControlChanged {
                system: *system,
                from: to.clone(),
                to: from.clone(),
            }),
            // Effects that create/destroy entities require full entity snapshots
            // for inversion. Use full-state clone for these.
            _ => None,
        }
    }

    /// System name for telemetry derivation (Principle 9).
    pub fn system_name(&self) -> &'static str {
        match self.phase() {
            EffectPhase::Economy => "economy",
            EffectPhase::Manufacturing => "manufacturing",
            EffectPhase::Movement => "movement",
            EffectPhase::Combat => "combat",
            EffectPhase::Diplomacy => "missions",
            EffectPhase::Intelligence => "intelligence",
            EffectPhase::Command => "command",
            EffectPhase::Endgame => "endgame",
        }
    }
}

// ---------------------------------------------------------------------------
// Monoidal composition
// ---------------------------------------------------------------------------

/// Combine two effect sequences with phase-stable sort.
/// This is the semigroup operation: production order preserved within each phase.
///
/// NOTE: must remain `sort_by_key` (stable), not `sort_unstable_by_key` —
/// within-phase ordering is load-bearing for deterministic replay.
pub fn combine_effects(mut a: Vec<GameEffect>, mut b: Vec<GameEffect>) -> Vec<GameEffect> {
    a.append(&mut b);
    a.sort_by_key(|e| e.phase());
    a
}

/// Filter effects by predicate.
pub fn filter_effects(
    effects: Vec<GameEffect>,
    predicate: impl Fn(&GameEffect) -> bool,
) -> Vec<GameEffect> {
    effects.into_iter().filter(predicate).collect()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_ordering_is_correct() {
        assert!(EffectPhase::Economy < EffectPhase::Manufacturing);
        assert!(EffectPhase::Manufacturing < EffectPhase::Movement);
        assert!(EffectPhase::Movement < EffectPhase::Combat);
        assert!(EffectPhase::Combat < EffectPhase::Diplomacy);
        assert!(EffectPhase::Diplomacy < EffectPhase::Intelligence);
        assert!(EffectPhase::Intelligence < EffectPhase::Command);
        assert!(EffectPhase::Command < EffectPhase::Endgame);
    }

    #[test]
    fn combine_effects_preserves_phase_order() {
        let a = vec![GameEffect::FleetArrived {
            fleet: FleetKey::default(),
            origin: SystemKey::default(),
            destination: SystemKey::default(),
        }];
        let b = vec![GameEffect::BuildCompleted {
            system: SystemKey::default(),
            kind: BuildableKind::CapitalShip(CapitalShipKey::default()),
        }];
        let combined = combine_effects(a, b);
        assert_eq!(combined.len(), 2);
        // Manufacturing (phase 1) should come before Movement (phase 2)
        assert_eq!(combined[0].phase(), EffectPhase::Manufacturing);
        assert_eq!(combined[1].phase(), EffectPhase::Movement);
    }

    #[test]
    fn invert_popularity_shift() {
        let effect = GameEffect::PopularityShifted {
            system: SystemKey::default(),
            alliance_delta: 0.1,
            empire_delta: -0.05,
        };
        let inverse = effect.invert().expect("popularity shift is invertible");
        match inverse {
            GameEffect::PopularityShifted {
                alliance_delta,
                empire_delta,
                ..
            } => {
                assert!((alliance_delta - (-0.1)).abs() < f32::EPSILON);
                assert!((empire_delta - 0.05).abs() < f32::EPSILON);
            }
            _ => panic!("expected PopularityShifted"),
        }
    }

    #[test]
    fn irreversible_effects_return_none() {
        let effect = GameEffect::CharacterKilled {
            character: CharacterKey::default(),
        };
        assert!(effect.invert().is_none());
    }

    #[test]
    fn system_name_matches_phase() {
        let eco = GameEffect::SupportDrifted {
            system: SystemKey::default(),
            alliance_delta: 0.0,
            empire_delta: 0.0,
        };
        assert_eq!(eco.system_name(), "economy");

        let combat = GameEffect::SpaceBattleResolved {
            system: SystemKey::default(),
            winner: CombatSide::Attacker,
        };
        assert_eq!(combat.system_name(), "combat");
    }

    #[test]
    fn effects_are_serializable() {
        let effect = GameEffect::FleetArrived {
            fleet: FleetKey::default(),
            origin: SystemKey::default(),
            destination: SystemKey::default(),
        };
        let json = serde_json::to_string(&effect).expect("serialize");
        let _: GameEffect = serde_json::from_str(&json).expect("deserialize roundtrip");
    }

    #[test]
    fn filter_effects_by_phase() {
        let effects = vec![
            GameEffect::BuildCompleted {
                system: SystemKey::default(),
                kind: BuildableKind::CapitalShip(CapitalShipKey::default()),
            },
            GameEffect::FleetArrived {
                fleet: FleetKey::default(),
                origin: SystemKey::default(),
                destination: SystemKey::default(),
            },
            GameEffect::SpaceBattleResolved {
                system: SystemKey::default(),
                winner: CombatSide::Attacker,
            },
        ];
        let combat_only = filter_effects(effects, |e| e.phase() == EffectPhase::Combat);
        assert_eq!(combat_only.len(), 1);
    }
}
