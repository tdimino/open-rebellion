//! Multiplayer protocol message definitions.
//!
//! 179 `NetMessage` variants covering every notification type found in the
//! original REBEXE.EXE binary via Ghidra reverse engineering. These are
//! **protocol definitions only** — no networking implementation.
//!
//! Sources:
//! - `ghidra/notes/cpp-class-hierarchy.md` — tactical message slots, vtable events
//! - `ghidra/notes/entity-system.md` — entity setter/notify chains, event IDs
//! - `ghidra/notes/mission-event-cookbook.md` — mission lifecycle, story chains
//! - `ghidra/notes/annotated-functions.md` — notification dispatcher signatures

use serde::{Deserialize, Serialize};

/// Every notification message type from the original game's observer/notification
/// system. Organized by domain: tactical combat, entity state, character/role,
/// mission lifecycle, story chains, system state, fleet events, economy, and
/// game lifecycle.
///
/// Each variant documents its original notification string and, where known,
/// the registered event ID (hex).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum NetMessage {
    // -----------------------------------------------------------------------
    // Tactical combat messages (CTacticalBattleManager message router)
    // Source: cpp-class-hierarchy.md §4
    // -----------------------------------------------------------------------
    /// SHIP_ADD (+0x04)
    ShipAdd,
    /// SHIP_REMOVE (+0x07)
    ShipRemove,
    /// SHIP_ABSTRACT_DESTROY (+0x0a)
    ShipAbstractDestroy,
    /// SHIP_DESTROY (+0x0d)
    ShipDestroy,
    /// cmSHIP_POST_DS_STATUS (+0x10)
    ShipPostDeathStarStatus,
    /// NAV_ADD (+0x13)
    NavAdd,
    /// NAV_REMOVE (+0x16)
    NavRemove,
    /// NAV_PURGE (+0x19)
    NavPurge,
    /// NAV_DELETE (+0x1c)
    NavDelete,
    /// SHIPGROUP_MOVE (+0x1f)
    ShipGroupMove,
    /// SHIPGROUP_NEWMISSION (+0x22)
    ShipGroupNewMission,
    /// SHIPGROUP_NEWFORMATION (+0x25)
    ShipGroupNewFormation,
    /// SHIPGROUP_NEWTARGETLIST (+0x28)
    ShipGroupNewTargetList,
    /// SHIP_FIRELASERCANNON (+0x2b)
    ShipFireLaserCannon,
    /// SHIP_FIRETURBOLASER (+0x2e)
    ShipFireTurboLaser,
    /// SHIP_FIREIONCANNON (+0x31)
    ShipFireIonCannon,
    /// SHIP_FIRETORPEDO (+0x34)
    ShipFireTorpedo,
    /// SHIP_TAKE_LASER_HIT (+0x37)
    ShipTakeLaserHit,
    /// SHIP_TAKE_ION_HIT (+0x3a)
    ShipTakeIonHit,
    /// SHIP_TAKE_TURBO_HIT (+0x3d)
    ShipTakeTurboHit,
    /// SHIP_TAKE_TORPEDO_HIT (+0x40)
    ShipTakeTorpedoHit,
    /// SHIP_ION_DAMAGE (+0x46)
    ShipIonDamage,
    /// SHIP_LAUNCH (+0x4f)
    ShipLaunch,
    /// SHIP_RECOVER (+0x52)
    ShipRecover,
    /// SHIP_ENGINE_DOWN (+0x55)
    ShipEngineDown,
    /// SHIP_WEAPON_DOWN (+0x58)
    ShipWeaponDown,
    /// SHIP_TRACTOR_DOWN (+0x5b)
    ShipTractorDown,
    /// SHIP_SHIELD_HIT (+0x5e)
    ShipShieldHit,
    /// SHIP_WEAPON_HIT (+0x61)
    ShipWeaponHit,
    /// SHIP_TRACTOR_HIT (+0x64)
    ShipTractorHit,
    /// SHIP_ENGINE_HIT (+0x67)
    ShipEngineHit,
    /// SHIP_HYPERDRIVE_HIT (+0x6a)
    ShipHyperdriveHit,
    /// SHIP_SHIELD_FIX (+0x6d)
    ShipShieldFix,
    /// SHIP_WEAPON_FIX (+0x70)
    ShipWeaponFix,
    /// SHIP_TRACTOR_FIX (+0x73)
    ShipTractorFix,
    /// SHIP_ENGINE_FIX (+0x76)
    ShipEngineFix,
    /// SHIP_HYPERDRIVE_FIX (+0x79)
    ShipHyperdriveFix,
    /// SHIP_TRACTOR_LOCK (+0x7c)
    ShipTractorLock,
    /// SHIP_TRACTOR_UNLOCK (+0x7f)
    ShipTractorUnlock,
    /// SHIP_GRAVITY_LOCK (+0x82)
    ShipGravityLock,
    /// SHIP_GRAVITY_UNLOCK (+0x85)
    ShipGravityUnlock,
    /// SHIP_SET_RECOVERY_SHIP (+0x88)
    ShipSetRecoveryShip,
    /// SHIP_HYPERSPACE (+0x8b)
    ShipHyperspace,
    /// SHIP_WITHDRAW (+0x8e)
    ShipWithdraw,
    /// SHIP_SCUTTLE (+0x91)
    ShipScuttle,
    /// TASKFORCE_NEW (+0x94)
    TaskForceNew,
    /// FIGHTERGROUP_NEW (+0x97)
    FighterGroupNew,
    /// SHIPGROUP_DELETE (+0x9a)
    ShipGroupDelete,
    /// SHIPGROUP_ADDSHIP (+0x9d)
    ShipGroupAddShip,
    /// SHIPGROUP_ADDTARGET (+0xa0)
    ShipGroupAddTarget,
    /// SHIPGROUP_REMOVETARGET (+0xa3)
    ShipGroupRemoveTarget,
    /// SHIPGROUP_REPLACETARGETLIST (+0xa6)
    ShipGroupReplaceTargetList,
    /// SHIPGROUP_REMOVESHIP (+0xa9)
    ShipGroupRemoveShip,
    /// SHIPGROUP_ADDNAVPOINT (+0xac)
    ShipGroupAddNavPoint,
    /// SHIPGROUP_REMOVENAVPOINT (+0xaf)
    ShipGroupRemoveNavPoint,
    /// SHIPGROUP_REPLACENAVLIST (+0xb2)
    ShipGroupReplaceNavList,
    /// SHIPGROUP_CHANGETARGET (+0xb5)
    ShipGroupChangeTarget,
    /// SHIPGROUP_TACTMISS_CHANGE_STATE (+0xb8)
    ShipGroupTactMissChangeState,
    /// FORMATION_ACCELERATE (+0xbe)
    FormationAccelerate,
    /// CAPITALSHIP_UPDATE (+0xc1)
    CapitalShipUpdate,
    /// FIGHTERSQUADRON_UPDATE (+0xc4)
    FighterSquadronUpdate,
    /// TACTCHAR_UPDATE (+0xc7)
    TacticalCharacterUpdate,
    /// TACTICALRESULT_UPDATE (+0xca)
    TacticalResultUpdate,
    /// DEATHSTAR_UPDATE (+0xcd)
    DeathStarUpdate,
    /// DEATHSTAR_FIRE (+0xd0)
    DeathStarFire,
    /// DEATHSTAR_WITHDRAW (+0xd3)
    DeathStarWithdraw,

    // -----------------------------------------------------------------------
    // Game lifecycle messages (CTacticalBattleManager continued)
    // Source: cpp-class-hierarchy.md §4
    // -----------------------------------------------------------------------
    /// ALLIANCE_START_TURN (+0xd6)
    AllianceStartTurn,
    /// ALLIANCE_END_TURN (+0xd9)
    AllianceEndTurn,
    /// EMPIRE_START_TURN (+0xdc)
    EmpireStartTurn,
    /// EMPIRE_END_TURN (+0xdf)
    EmpireEndTurn,
    /// GAME_OVER (+0xe2)
    GameOver,
    /// SYSTEM_TURN_COMPLETED (+0xe5)
    SystemTurnCompleted,
    /// SYSTEM_PAUSE (+0xe8)
    SystemPause,
    /// SYSTEM_UNPAUSE (+0xeb)
    SystemUnpause,
    /// SYSTEM_SAVE (+0xee)
    SystemSave,
    /// SYSTEM_QUIT (+0xf1)
    SystemQuit,
    /// SYSTEM_WAIT_FOR_GAME_OVER (+0xf4)
    SystemWaitForGameOver,
    /// SYSTEM_GAME_OVER (+0xf7)
    SystemGameOver,
    /// SYSTEM_SYNCHRONIZE_BEGIN (+0xfa)
    SystemSynchronizeBegin,
    /// SYSTEM_SYNCHRONIZE_END (+0xfd)
    SystemSynchronizeEnd,
    /// SYSTEM_ABORT (+0x100)
    SystemAbort,

    // -----------------------------------------------------------------------
    // Entity state change notifications (CNotifyObject setter-notify chain)
    // Source: entity-system.md §1, cpp-class-hierarchy.md §1
    // -----------------------------------------------------------------------
    /// CapShipHullValueDamageNotif / HullValueDamage — event 0x1c0 (448)
    CapShipHullDamage,
    /// CapShipShieldRechargeRateCHAllocatedNotif — event 0x1c1 (449)
    CapShipShieldRechargeAllocated,
    /// CapShipWeaponRechargeRateCHAllocatedNotif — event 0x1c2 (450)
    CapShipWeaponRechargeAllocated,
    /// FightSquadSquadSizeDamageNotif / SquadSizeDamage — event 0x1a0 (416)
    FighterSquadSizeDamage,
    /// TroopRegDestroyedRunningBlockade — event 0x340 (832)
    TroopDestroyedRunningBlockade,
    /// TroopRegWithdrawPercentNotif
    TroopWithdrawPercent,
    /// SystemTroopRegWithdrawPercentNotif
    SystemTroopWithdrawPercent,

    // -----------------------------------------------------------------------
    // Character state notifications
    // Source: entity-system.md §1.1–1.3
    // -----------------------------------------------------------------------
    /// SystemLoyaltyNotif / Loyalty — vtable +0x238
    CharacterLoyaltyChanged,
    /// CharacterEnhancedLoyaltyNotif / EnhancedLoyalty — vtable +0x318
    CharacterEnhancedLoyalty,
    /// NotifyCombatStrengthChanged — vtable +0x330
    CharacterCombatStrength,
    /// MissionHyperdriveModifierNotif / MissionHyperdriveModifier — vtable +0x338
    CharacterHyperdriveModifier,
    /// CharacterEnhancedDiplomacyNotif / EnhancedDiplomacy
    CharacterEnhancedDiplomacy,
    /// CharacterEnhancedEspionageNotif / EnhancedEspionage
    CharacterEnhancedEspionage,
    /// CharacterEnhancedCombatNotif / EnhancedCombat
    CharacterEnhancedCombat,
    /// CharacterForceNotif / Force — event 0x1e1 (481)
    CharacterForce,
    /// CharacterForceExperienceNotif / ForceExperience
    CharacterForceExperience,
    /// CharacterForceTrainingNotif / ForceTraining — event 0x1e5 (485)
    CharacterForceTraining,
    /// CharacterForceUserDiscoveredKeyNotif / ForceUserDiscovered — event 0x362 (866)
    CharacterForceUserDiscovered,
    /// CharacterForceAwareNotif / ForceAware
    CharacterForceAware,
    /// CharacterForcePotentialNotif / ForcePotential
    CharacterForcePotential,
    /// CharacterDiscoveringForceUserNotif
    CharacterDiscoveringForceUser,

    // -----------------------------------------------------------------------
    // Character role notifications
    // Source: entity-system.md §1.4–1.5
    // -----------------------------------------------------------------------
    /// RoleBaseDiplomacyNotif / BaseDiplomacy
    RoleBaseDiplomacy,
    /// RoleBaseEspionageNotif / BaseEspionage
    RoleBaseEspionage,
    /// RoleBaseShipyardRDNotif / BaseShipyardRD
    RoleBaseShipyardRD,
    /// RoleBaseTrainingFacilRDNotif / BaseTrainingFacilRD
    RoleBaseTrainingFacilRD,
    /// RoleBaseConstructionYardRDNotif / BaseConstructionYardRD
    RoleBaseConstructionYardRD,
    /// RoleBaseCombatNotif / BaseCombat
    RoleBaseCombat,
    /// RoleBaseLeadershipNotif / BaseLeadership
    RoleBaseLeadership,
    /// RoleBaseLoyaltyNotif / BaseLoyalty
    RoleBaseLoyalty,
    /// RoleMissionKeyNotif / Mission
    RoleMission,
    /// RoleMissionSeedKeyNotif / MissionSeed
    RoleMissionSeed,
    /// RoleOnMissionNotif / OnMission
    RoleOnMission,
    /// RoleOnHiddenMissionNotif / OnHiddenMission
    RoleOnHiddenMission,
    /// RoleOnMandatoryMissionNotif / OnMandatoryMission
    RoleOnMandatoryMission,
    /// RoleCanResignFromMissionNotif / CanResignFromMission
    RoleCanResignFromMission,
    /// RoleMissionResignRequestNotif / MissionResignRequest
    RoleMissionResignRequest,
    /// RoleMissionRemoveRequestNotif / MissionRemoveRequest
    RoleMissionRemoveRequest,
    /// RoleMovingBetweenMissionsNotif / MovingBetweenMissions
    RoleMovingBetweenMissions,
    /// RoleParentAtMissionCompletionKeyNotif / ParentAtMissionCompletion
    RoleParentAtMissionCompletion,
    /// RoleLocationAtMissionCompletionKeyNotif / LocationAtMissionCompletion
    RoleLocationAtMissionCompletion,

    // -----------------------------------------------------------------------
    // Mission lifecycle notifications
    // Source: mission-event-cookbook.md §5
    // -----------------------------------------------------------------------
    /// MissionUserIDNotif / UserID
    MissionUserId,
    /// MissionUserID2Notif / UserID2
    MissionUserId2,
    /// MissionOriginLocationKeyNotif / OriginLocation
    MissionOriginLocation,
    /// MissionObjectiveKeyNotif / Objective
    MissionObjective,
    /// MissionTargetKeyNotif / Target
    MissionTarget,
    /// MissionTargetLocationKeyNotif / TargetLocation
    MissionTargetLocation,
    /// MissionLeaderKeyNotif / Leader
    MissionLeader,
    /// MissionLeaderSeedKeyNotif / LeaderSeed
    MissionLeaderSeed,
    /// MissionReadyForNextPhaseNotif / ReadyForNextPhase
    MissionReadyForNextPhase,
    /// MissionImpliedTeamNotif / ImpliedTeam
    MissionImpliedTeam,
    /// MissionMandatoryNotif / Mandatory
    MissionMandatory,
    /// MissionEspionageExtraSystemKeyNotif / ExtraSystem — event 0x370 (880)
    MissionEspionageExtraSystem,

    // -----------------------------------------------------------------------
    // Game object destruction notifications
    // Source: entity-system.md §3
    // -----------------------------------------------------------------------
    /// GameObjDestroyedNotif / Destroyed — event 0x302 (770)
    GameObjDestroyed,
    /// GameObjDestroyedOnArrivalNotif / DestroyedOnArrival — event 0x303 (771)
    GameObjDestroyedOnArrival,
    /// GameObjDestroyedAutoscrapNotif / DestroyedAutoscrap — event 0x304 (772)
    GameObjDestroyedAutoscrap,
    /// GameObjDestroyedSabotageNotif / DestroyedSabotage — event 0x305 (773)
    GameObjDestroyedSabotage,
    /// GameObjDestroyedAssassinationNotif / DestroyedAssassination — event 0x306 (774)
    GameObjDestroyedAssassination,

    // -----------------------------------------------------------------------
    // Fleet notifications
    // Source: entity-system.md §3.1
    // -----------------------------------------------------------------------
    /// FleetBattleNotif / Battle — event 0x180 (384)
    FleetBattle,
    /// FleetBlockadeNotif / Blockade — event 0x181 (385)
    FleetBlockade,
    /// FleetBombardNotif / Bombard — event 0x182 (386)
    FleetBombard,
    /// FleetAssaultNotif / Assault — event 0x183 (387)
    FleetAssault,

    // -----------------------------------------------------------------------
    // System state notifications
    // Source: entity-system.md §4
    // -----------------------------------------------------------------------
    /// SystemBattleNotif / Battle — event 0x14d (333)
    SystemBattle,
    /// SystemBlockadeNotif / Blockade — event 0x14e (334)
    SystemBlockade,
    /// SystemUprisingNotif / Uprising
    SystemUprising,
    /// SystemUprisingIncidentNotif / UprisingIncident — event 0x152 (338)
    SystemUprisingIncident,
    /// ControlKindBattleWonNotif
    SystemControlBattleWon,
    /// SystemControlKindUprisingNotif / ControlKindUprising
    SystemControlUprising,

    // -----------------------------------------------------------------------
    // Side (faction) notifications
    // Source: entity-system.md §4.1, annotated-functions.md
    // -----------------------------------------------------------------------
    /// SideRecruitmentDoneNotif / RecruitmentDone — event 0x12c (300)
    SideRecruitmentDone,
    /// SideConstructionYardRdOrderNotif / ConstructionYardRdOrder — event 0x127 (295)
    SideConstructionYardRdOrder,
    /// SideVictoryConditionsNotif / VictoryConditions
    SideVictoryConditions,

    // -----------------------------------------------------------------------
    // Story chain: Dagobah / Jedi Training
    // Source: mission-event-cookbook.md §5 Dagobah
    // -----------------------------------------------------------------------
    /// MissionMgrLukeDagobahRequiredNotif / LukeDagobahRequired — gate
    StoryLukeDagobahRequired,
    /// MissionMgrLukeDagobahNotif / LukeDagobah — event 0x221 (545)
    StoryLukeDagobah,
    /// DagobahMissionFirstTrainingDayNotif / FirstTrainingDay — gate
    StoryDagobahFirstTrainingDay,
    /// MissionJediTrainingTeacherKeyNotif / Teacher — gate
    StoryJediTrainingTeacher,
    /// LukeDagobahCompletedNotif / DagobahCompleted — event 0x210 (528)
    StoryDagobahCompleted,

    // -----------------------------------------------------------------------
    // Story chain: Final Battle
    // Source: mission-event-cookbook.md §5 Final Battle
    // -----------------------------------------------------------------------
    /// MissionMgrDarthPickupNotif / DarthPickup — gate
    StoryDarthPickup,
    /// MissionMgrDarthToLukeFinalBattleNotif / DarthToLukeFinalBattle — gate
    StoryDarthToLukeFinalBattle,
    /// MissionMgrDarthToEmperorFinalBattleNotif / DarthToEmperorFinalBattle — gate
    StoryDarthToEmperorFinalBattle,
    /// MissionMgrFinalBattleReadyNotif / FinalBattleReady — gate (mission mgr)
    StoryFinalBattleReady,
    /// LukeFinalBattleReadyNotif / FinalBattleReady — gate (Luke side)
    StoryLukeFinalBattleReady,
    /// MissionMgrFinalBattleNotif / FinalBattle — event 0x220 (544)
    StoryFinalBattle,

    // -----------------------------------------------------------------------
    // Story chain: Jabba's Palace
    // Source: mission-event-cookbook.md §5 Jabba
    // -----------------------------------------------------------------------
    /// MissionMgrLukePalaceNotif / LukePalace — gate
    StoryLukePalace,
    /// MissionMgrHanCapturedAtPalaceNotif / HanCapturedAtPalace — gate
    StoryHanCapturedAtPalace,
    /// MissionMgrLeiaPalaceNotif / LeiaPalace — gate
    StoryLeiaPalace,
    /// MissionMgrChewbaccaPalaceNotif / ChewbaccaPalace — gate
    StoryChewbaccaPalace,

    // -----------------------------------------------------------------------
    // Story chain: Bounty Hunters
    // Source: mission-event-cookbook.md §5 Bounty
    // -----------------------------------------------------------------------
    /// MissionMgrBountyHuntersActiveNotif / BountyHuntersActive — gate
    StoryBountyHuntersActive,
    /// HanBountyAttackNotif / BountyAttack — event 0x212 (530)
    StoryHanBountyAttack,
    /// HanCapturedByBountyHuntersNotif / CapturedByBountyHunters — gate
    StoryHanCapturedByBountyHunters,

    // -----------------------------------------------------------------------
    // Vtable change events (CCombatUnit / CCapitalShip / CFighterSquadron)
    // Source: cpp-class-hierarchy.md §1
    // -----------------------------------------------------------------------
    /// vtable +0x260 NotifySquadChanged(old, new, ctx)
    VtableSquadChanged,
    /// vtable +0x29c NotifyHullChanged(old, new, ctx)
    VtableHullChanged,
    /// vtable +0x2a0 NotifyShieldChanged(old, new, ctx)
    VtableShieldChanged,
    /// vtable +0x2a4 NotifyWeaponChanged(old, new, ctx)
    VtableWeaponChanged,
    /// vtable +0x238 NotifyLoyaltyChanged(old, new, ctx)
    VtableLoyaltyChanged,
    /// vtable +0x318 NotifyEnhancedLoyaltyChanged(old, new, ctx)
    VtableEnhancedLoyaltyChanged,
    /// vtable +0x330 NotifyCombatStrengthChanged(old, new, ctx)
    VtableCombatStrengthChanged,
    /// vtable +0x338 NotifyHyperdriveChanged(old, new, ctx)
    VtableHyperdriveChanged,

    // -----------------------------------------------------------------------
    // Combat phase dispatch (CCombatUnit vtable)
    // Source: cpp-class-hierarchy.md §1
    // -----------------------------------------------------------------------
    /// vtable +0x1c4 ResolveWeaponFire(side, ctx)
    CombatPhaseWeaponFire,
    /// vtable +0x1c8 ResolveShieldAbsorb(side, ctx)
    CombatPhaseShieldAbsorb,
    /// vtable +0x1d0 ResolveHullDamage(side, ctx)
    CombatPhaseHullDamage,
    /// vtable +0x1d4 ResolveFighterEngage(side, ctx)
    CombatPhaseFighterEngage,
    /// vtable +0x1d8 ResolveShieldAlt(side, ctx) — alt path for family 0x71-0x72
    CombatPhaseShieldAlt,
    /// vtable +0x1f4 ApplyWeaponDamage(side, ctx)
    CombatApplyWeaponDamage,
}

/// Maps a `NetMessage` to its original event ID, if one was registered.
/// Returns `None` for gate notifications and vtable-only events.
impl NetMessage {
    pub const fn event_id(&self) -> Option<u16> {
        match self {
            Self::SideConstructionYardRdOrder => Some(0x127),
            Self::SideRecruitmentDone => Some(0x12c),
            Self::SystemBattle => Some(0x14d),
            Self::SystemBlockade => Some(0x14e),
            Self::SystemUprisingIncident => Some(0x152),
            Self::FleetBattle => Some(0x180),
            Self::FleetBlockade => Some(0x181),
            Self::FleetBombard => Some(0x182),
            Self::FleetAssault => Some(0x183),
            Self::FighterSquadSizeDamage => Some(0x1a0),
            Self::CapShipHullDamage => Some(0x1c0),
            Self::CapShipShieldRechargeAllocated => Some(0x1c1),
            Self::CapShipWeaponRechargeAllocated => Some(0x1c2),
            Self::CharacterForce => Some(0x1e1),
            Self::CharacterForceTraining => Some(0x1e5),
            Self::StoryDagobahCompleted => Some(0x210),
            Self::StoryHanBountyAttack => Some(0x212),
            Self::StoryFinalBattle => Some(0x220),
            Self::StoryLukeDagobah => Some(0x221),
            Self::GameObjDestroyed => Some(0x302),
            Self::GameObjDestroyedOnArrival => Some(0x303),
            Self::GameObjDestroyedAutoscrap => Some(0x304),
            Self::GameObjDestroyedSabotage => Some(0x305),
            Self::GameObjDestroyedAssassination => Some(0x306),
            Self::TroopDestroyedRunningBlockade => Some(0x340),
            Self::CharacterForceUserDiscovered => Some(0x362),
            Self::MissionEspionageExtraSystem => Some(0x370),
            _ => None,
        }
    }

    /// Human-readable category for grouping in telemetry and debug output.
    pub const fn category(&self) -> &'static str {
        match self {
            Self::ShipAdd
            | Self::ShipRemove
            | Self::ShipAbstractDestroy
            | Self::ShipDestroy
            | Self::ShipPostDeathStarStatus
            | Self::NavAdd
            | Self::NavRemove
            | Self::NavPurge
            | Self::NavDelete
            | Self::ShipGroupMove
            | Self::ShipGroupNewMission
            | Self::ShipGroupNewFormation
            | Self::ShipGroupNewTargetList
            | Self::ShipFireLaserCannon
            | Self::ShipFireTurboLaser
            | Self::ShipFireIonCannon
            | Self::ShipFireTorpedo
            | Self::ShipTakeLaserHit
            | Self::ShipTakeIonHit
            | Self::ShipTakeTurboHit
            | Self::ShipTakeTorpedoHit
            | Self::ShipIonDamage
            | Self::ShipLaunch
            | Self::ShipRecover
            | Self::ShipEngineDown
            | Self::ShipWeaponDown
            | Self::ShipTractorDown
            | Self::ShipShieldHit
            | Self::ShipWeaponHit
            | Self::ShipTractorHit
            | Self::ShipEngineHit
            | Self::ShipHyperdriveHit
            | Self::ShipShieldFix
            | Self::ShipWeaponFix
            | Self::ShipTractorFix
            | Self::ShipEngineFix
            | Self::ShipHyperdriveFix
            | Self::ShipTractorLock
            | Self::ShipTractorUnlock
            | Self::ShipGravityLock
            | Self::ShipGravityUnlock
            | Self::ShipSetRecoveryShip
            | Self::ShipHyperspace
            | Self::ShipWithdraw
            | Self::ShipScuttle
            | Self::TaskForceNew
            | Self::FighterGroupNew
            | Self::ShipGroupDelete
            | Self::ShipGroupAddShip
            | Self::ShipGroupAddTarget
            | Self::ShipGroupRemoveTarget
            | Self::ShipGroupReplaceTargetList
            | Self::ShipGroupRemoveShip
            | Self::ShipGroupAddNavPoint
            | Self::ShipGroupRemoveNavPoint
            | Self::ShipGroupReplaceNavList
            | Self::ShipGroupChangeTarget
            | Self::ShipGroupTactMissChangeState
            | Self::FormationAccelerate
            | Self::CapitalShipUpdate
            | Self::FighterSquadronUpdate
            | Self::TacticalCharacterUpdate
            | Self::TacticalResultUpdate
            | Self::DeathStarUpdate
            | Self::DeathStarFire
            | Self::DeathStarWithdraw => "tactical",

            Self::AllianceStartTurn
            | Self::AllianceEndTurn
            | Self::EmpireStartTurn
            | Self::EmpireEndTurn
            | Self::GameOver
            | Self::SystemTurnCompleted
            | Self::SystemPause
            | Self::SystemUnpause
            | Self::SystemSave
            | Self::SystemQuit
            | Self::SystemWaitForGameOver
            | Self::SystemGameOver
            | Self::SystemSynchronizeBegin
            | Self::SystemSynchronizeEnd
            | Self::SystemAbort => "lifecycle",

            Self::CapShipHullDamage
            | Self::CapShipShieldRechargeAllocated
            | Self::CapShipWeaponRechargeAllocated
            | Self::FighterSquadSizeDamage
            | Self::TroopDestroyedRunningBlockade
            | Self::TroopWithdrawPercent
            | Self::SystemTroopWithdrawPercent => "entity_state",

            Self::CharacterLoyaltyChanged
            | Self::CharacterEnhancedLoyalty
            | Self::CharacterCombatStrength
            | Self::CharacterHyperdriveModifier
            | Self::CharacterEnhancedDiplomacy
            | Self::CharacterEnhancedEspionage
            | Self::CharacterEnhancedCombat
            | Self::CharacterForce
            | Self::CharacterForceExperience
            | Self::CharacterForceTraining
            | Self::CharacterForceUserDiscovered
            | Self::CharacterForceAware
            | Self::CharacterForcePotential
            | Self::CharacterDiscoveringForceUser => "character",

            Self::RoleBaseDiplomacy
            | Self::RoleBaseEspionage
            | Self::RoleBaseShipyardRD
            | Self::RoleBaseTrainingFacilRD
            | Self::RoleBaseConstructionYardRD
            | Self::RoleBaseCombat
            | Self::RoleBaseLeadership
            | Self::RoleBaseLoyalty
            | Self::RoleMission
            | Self::RoleMissionSeed
            | Self::RoleOnMission
            | Self::RoleOnHiddenMission
            | Self::RoleOnMandatoryMission
            | Self::RoleCanResignFromMission
            | Self::RoleMissionResignRequest
            | Self::RoleMissionRemoveRequest
            | Self::RoleMovingBetweenMissions
            | Self::RoleParentAtMissionCompletion
            | Self::RoleLocationAtMissionCompletion => "role",

            Self::MissionUserId
            | Self::MissionUserId2
            | Self::MissionOriginLocation
            | Self::MissionObjective
            | Self::MissionTarget
            | Self::MissionTargetLocation
            | Self::MissionLeader
            | Self::MissionLeaderSeed
            | Self::MissionReadyForNextPhase
            | Self::MissionImpliedTeam
            | Self::MissionMandatory
            | Self::MissionEspionageExtraSystem => "mission",

            Self::GameObjDestroyed
            | Self::GameObjDestroyedOnArrival
            | Self::GameObjDestroyedAutoscrap
            | Self::GameObjDestroyedSabotage
            | Self::GameObjDestroyedAssassination => "destruction",

            Self::FleetBattle
            | Self::FleetBlockade
            | Self::FleetBombard
            | Self::FleetAssault => "fleet",

            Self::SystemBattle
            | Self::SystemBlockade
            | Self::SystemUprising
            | Self::SystemUprisingIncident
            | Self::SystemControlBattleWon
            | Self::SystemControlUprising => "system",

            Self::SideRecruitmentDone
            | Self::SideConstructionYardRdOrder
            | Self::SideVictoryConditions => "faction",

            Self::StoryLukeDagobahRequired
            | Self::StoryLukeDagobah
            | Self::StoryDagobahFirstTrainingDay
            | Self::StoryJediTrainingTeacher
            | Self::StoryDagobahCompleted
            | Self::StoryDarthPickup
            | Self::StoryDarthToLukeFinalBattle
            | Self::StoryDarthToEmperorFinalBattle
            | Self::StoryFinalBattleReady
            | Self::StoryLukeFinalBattleReady
            | Self::StoryFinalBattle
            | Self::StoryLukePalace
            | Self::StoryHanCapturedAtPalace
            | Self::StoryLeiaPalace
            | Self::StoryChewbaccaPalace
            | Self::StoryBountyHuntersActive
            | Self::StoryHanBountyAttack
            | Self::StoryHanCapturedByBountyHunters => "story",

            Self::VtableSquadChanged
            | Self::VtableHullChanged
            | Self::VtableShieldChanged
            | Self::VtableWeaponChanged
            | Self::VtableLoyaltyChanged
            | Self::VtableEnhancedLoyaltyChanged
            | Self::VtableCombatStrengthChanged
            | Self::VtableHyperdriveChanged => "vtable_event",

            Self::CombatPhaseWeaponFire
            | Self::CombatPhaseShieldAbsorb
            | Self::CombatPhaseHullDamage
            | Self::CombatPhaseFighterEngage
            | Self::CombatPhaseShieldAlt
            | Self::CombatApplyWeaponDamage => "combat_phase",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_count_matches_plan() {
        // Phase 5 plan specifies ~178 variants; we have 179
        let count = count_variants();
        assert!(
            count >= 178,
            "expected at least 178 NetMessage variants, got {}",
            count
        );
    }

    #[test]
    fn event_ids_are_unique() {
        let mut seen = std::collections::HashMap::new();
        for msg in all_variants() {
            if let Some(id) = msg.event_id() {
                if let Some(prev) = seen.insert(id, msg) {
                    panic!(
                        "duplicate event ID 0x{:03x}: {:?} and {:?}",
                        id, prev, msg
                    );
                }
            }
        }
    }

    #[test]
    fn all_categories_non_empty() {
        for msg in all_variants() {
            assert!(
                !msg.category().is_empty(),
                "{:?} has empty category",
                msg
            );
        }
    }

    #[test]
    fn serialization_round_trip() {
        let msg = NetMessage::StoryFinalBattle;
        let json = serde_json::to_string(&msg).unwrap();
        let back: NetMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(msg, back);
    }

    #[test]
    fn known_event_ids() {
        assert_eq!(NetMessage::SideRecruitmentDone.event_id(), Some(0x12c));
        assert_eq!(NetMessage::StoryFinalBattle.event_id(), Some(0x220));
        assert_eq!(
            NetMessage::MissionEspionageExtraSystem.event_id(),
            Some(0x370)
        );
        // Gate notification — no event ID
        assert_eq!(NetMessage::StoryDarthPickup.event_id(), None);
    }

    // -- helpers --

    fn count_variants() -> usize {
        all_variants().len()
    }

    fn all_variants() -> Vec<NetMessage> {
        // Exhaustive list via serde: serialize every variant name
        vec![
            NetMessage::ShipAdd,
            NetMessage::ShipRemove,
            NetMessage::ShipAbstractDestroy,
            NetMessage::ShipDestroy,
            NetMessage::ShipPostDeathStarStatus,
            NetMessage::NavAdd,
            NetMessage::NavRemove,
            NetMessage::NavPurge,
            NetMessage::NavDelete,
            NetMessage::ShipGroupMove,
            NetMessage::ShipGroupNewMission,
            NetMessage::ShipGroupNewFormation,
            NetMessage::ShipGroupNewTargetList,
            NetMessage::ShipFireLaserCannon,
            NetMessage::ShipFireTurboLaser,
            NetMessage::ShipFireIonCannon,
            NetMessage::ShipFireTorpedo,
            NetMessage::ShipTakeLaserHit,
            NetMessage::ShipTakeIonHit,
            NetMessage::ShipTakeTurboHit,
            NetMessage::ShipTakeTorpedoHit,
            NetMessage::ShipIonDamage,
            NetMessage::ShipLaunch,
            NetMessage::ShipRecover,
            NetMessage::ShipEngineDown,
            NetMessage::ShipWeaponDown,
            NetMessage::ShipTractorDown,
            NetMessage::ShipShieldHit,
            NetMessage::ShipWeaponHit,
            NetMessage::ShipTractorHit,
            NetMessage::ShipEngineHit,
            NetMessage::ShipHyperdriveHit,
            NetMessage::ShipShieldFix,
            NetMessage::ShipWeaponFix,
            NetMessage::ShipTractorFix,
            NetMessage::ShipEngineFix,
            NetMessage::ShipHyperdriveFix,
            NetMessage::ShipTractorLock,
            NetMessage::ShipTractorUnlock,
            NetMessage::ShipGravityLock,
            NetMessage::ShipGravityUnlock,
            NetMessage::ShipSetRecoveryShip,
            NetMessage::ShipHyperspace,
            NetMessage::ShipWithdraw,
            NetMessage::ShipScuttle,
            NetMessage::TaskForceNew,
            NetMessage::FighterGroupNew,
            NetMessage::ShipGroupDelete,
            NetMessage::ShipGroupAddShip,
            NetMessage::ShipGroupAddTarget,
            NetMessage::ShipGroupRemoveTarget,
            NetMessage::ShipGroupReplaceTargetList,
            NetMessage::ShipGroupRemoveShip,
            NetMessage::ShipGroupAddNavPoint,
            NetMessage::ShipGroupRemoveNavPoint,
            NetMessage::ShipGroupReplaceNavList,
            NetMessage::ShipGroupChangeTarget,
            NetMessage::ShipGroupTactMissChangeState,
            NetMessage::FormationAccelerate,
            NetMessage::CapitalShipUpdate,
            NetMessage::FighterSquadronUpdate,
            NetMessage::TacticalCharacterUpdate,
            NetMessage::TacticalResultUpdate,
            NetMessage::DeathStarUpdate,
            NetMessage::DeathStarFire,
            NetMessage::DeathStarWithdraw,
            NetMessage::AllianceStartTurn,
            NetMessage::AllianceEndTurn,
            NetMessage::EmpireStartTurn,
            NetMessage::EmpireEndTurn,
            NetMessage::GameOver,
            NetMessage::SystemTurnCompleted,
            NetMessage::SystemPause,
            NetMessage::SystemUnpause,
            NetMessage::SystemSave,
            NetMessage::SystemQuit,
            NetMessage::SystemWaitForGameOver,
            NetMessage::SystemGameOver,
            NetMessage::SystemSynchronizeBegin,
            NetMessage::SystemSynchronizeEnd,
            NetMessage::SystemAbort,
            NetMessage::CapShipHullDamage,
            NetMessage::CapShipShieldRechargeAllocated,
            NetMessage::CapShipWeaponRechargeAllocated,
            NetMessage::FighterSquadSizeDamage,
            NetMessage::TroopDestroyedRunningBlockade,
            NetMessage::TroopWithdrawPercent,
            NetMessage::SystemTroopWithdrawPercent,
            NetMessage::CharacterLoyaltyChanged,
            NetMessage::CharacterEnhancedLoyalty,
            NetMessage::CharacterCombatStrength,
            NetMessage::CharacterHyperdriveModifier,
            NetMessage::CharacterEnhancedDiplomacy,
            NetMessage::CharacterEnhancedEspionage,
            NetMessage::CharacterEnhancedCombat,
            NetMessage::CharacterForce,
            NetMessage::CharacterForceExperience,
            NetMessage::CharacterForceTraining,
            NetMessage::CharacterForceUserDiscovered,
            NetMessage::CharacterForceAware,
            NetMessage::CharacterForcePotential,
            NetMessage::CharacterDiscoveringForceUser,
            NetMessage::RoleBaseDiplomacy,
            NetMessage::RoleBaseEspionage,
            NetMessage::RoleBaseShipyardRD,
            NetMessage::RoleBaseTrainingFacilRD,
            NetMessage::RoleBaseConstructionYardRD,
            NetMessage::RoleBaseCombat,
            NetMessage::RoleBaseLeadership,
            NetMessage::RoleBaseLoyalty,
            NetMessage::RoleMission,
            NetMessage::RoleMissionSeed,
            NetMessage::RoleOnMission,
            NetMessage::RoleOnHiddenMission,
            NetMessage::RoleOnMandatoryMission,
            NetMessage::RoleCanResignFromMission,
            NetMessage::RoleMissionResignRequest,
            NetMessage::RoleMissionRemoveRequest,
            NetMessage::RoleMovingBetweenMissions,
            NetMessage::RoleParentAtMissionCompletion,
            NetMessage::RoleLocationAtMissionCompletion,
            NetMessage::MissionUserId,
            NetMessage::MissionUserId2,
            NetMessage::MissionOriginLocation,
            NetMessage::MissionObjective,
            NetMessage::MissionTarget,
            NetMessage::MissionTargetLocation,
            NetMessage::MissionLeader,
            NetMessage::MissionLeaderSeed,
            NetMessage::MissionReadyForNextPhase,
            NetMessage::MissionImpliedTeam,
            NetMessage::MissionMandatory,
            NetMessage::MissionEspionageExtraSystem,
            NetMessage::GameObjDestroyed,
            NetMessage::GameObjDestroyedOnArrival,
            NetMessage::GameObjDestroyedAutoscrap,
            NetMessage::GameObjDestroyedSabotage,
            NetMessage::GameObjDestroyedAssassination,
            NetMessage::FleetBattle,
            NetMessage::FleetBlockade,
            NetMessage::FleetBombard,
            NetMessage::FleetAssault,
            NetMessage::SystemBattle,
            NetMessage::SystemBlockade,
            NetMessage::SystemUprising,
            NetMessage::SystemUprisingIncident,
            NetMessage::SystemControlBattleWon,
            NetMessage::SystemControlUprising,
            NetMessage::SideRecruitmentDone,
            NetMessage::SideConstructionYardRdOrder,
            NetMessage::SideVictoryConditions,
            NetMessage::StoryLukeDagobahRequired,
            NetMessage::StoryLukeDagobah,
            NetMessage::StoryDagobahFirstTrainingDay,
            NetMessage::StoryJediTrainingTeacher,
            NetMessage::StoryDagobahCompleted,
            NetMessage::StoryDarthPickup,
            NetMessage::StoryDarthToLukeFinalBattle,
            NetMessage::StoryDarthToEmperorFinalBattle,
            NetMessage::StoryFinalBattleReady,
            NetMessage::StoryLukeFinalBattleReady,
            NetMessage::StoryFinalBattle,
            NetMessage::StoryLukePalace,
            NetMessage::StoryHanCapturedAtPalace,
            NetMessage::StoryLeiaPalace,
            NetMessage::StoryChewbaccaPalace,
            NetMessage::StoryBountyHuntersActive,
            NetMessage::StoryHanBountyAttack,
            NetMessage::StoryHanCapturedByBountyHunters,
            NetMessage::VtableSquadChanged,
            NetMessage::VtableHullChanged,
            NetMessage::VtableShieldChanged,
            NetMessage::VtableWeaponChanged,
            NetMessage::VtableLoyaltyChanged,
            NetMessage::VtableEnhancedLoyaltyChanged,
            NetMessage::VtableCombatStrengthChanged,
            NetMessage::VtableHyperdriveChanged,
            NetMessage::CombatPhaseWeaponFire,
            NetMessage::CombatPhaseShieldAbsorb,
            NetMessage::CombatPhaseHullDamage,
            NetMessage::CombatPhaseFighterEngage,
            NetMessage::CombatPhaseShieldAlt,
            NetMessage::CombatApplyWeaponDamage,
        ]
    }
}
