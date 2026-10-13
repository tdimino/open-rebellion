//! Strategic-window policy declarations and world-identity adapters.
//!
//! This renderer boundary translates stable DAT identities into the opaque
//! subject domain used by `rebellion-windowing`. It deliberately owns no
//! window lifecycle, geometry, or renderer state.

use rebellion_core::ids::{SectorKey, SystemKey};
use rebellion_core::world::GameWorld;
use rebellion_windowing::{
    Cardinality, ExistingOpenPolicy, FocusOnOpen, FocusRestoration, InputPolicy, OpenPlacement,
    PresentationPolicy, StackBand, SubjectId, WindowKey, WindowKind, WindowOwner, WindowPolicy,
    WindowSpec,
};

/// Subject namespace for original Sector DAT identities.
pub const SUBJECT_SECTOR: u32 = 1;
/// Subject namespace for original System DAT identities.
pub const SUBJECT_SYSTEM: u32 = 2;
/// Subject namespace for coordinator-allocated command contexts.
pub const SUBJECT_COMMAND_CONTEXT: u32 = 3;
/// Subject namespace for coordinator-allocated battle sessions.
pub const SUBJECT_BATTLE: u32 = 4;

/// Resolves a runtime system handle to its stable original DAT identity.
///
/// Fleet detail uses this adapter too: its top-level identity represents the
/// displayed system, while the selected fleet remains renderer-owned content.
#[must_use]
pub fn system_subject(world: &GameWorld, key: SystemKey) -> Option<SubjectId> {
    world
        .systems
        .get(key)
        .map(|system| SubjectId::new(SUBJECT_SYSTEM, u64::from(system.dat_id.raw())))
}

/// Resolves a runtime sector handle to its stable original DAT identity.
#[must_use]
pub fn sector_subject(world: &GameWorld, key: SectorKey) -> Option<SubjectId> {
    world
        .sectors
        .get(key)
        .map(|sector| SubjectId::new(SUBJECT_SECTOR, u64::from(sector.dat_id.raw())))
}

/// Returns the complete policy for one public strategic-window family.
///
/// The exhaustive match is the sole declaration table. Recovered facts come
/// from `ghidra/notes/strategic-windowing-toolkit.md`; conservative choices
/// are labeled `port:` at the affected arms.
#[must_use]
pub const fn policy_for(kind: WindowKind) -> WindowPolicy {
    match kind {
        // fact: FUN_00429ce0 creates at most two low-band Sector windows and
        // returns an existing subject without raising it.
        WindowKind::Sector => policy(
            Cardinality::Subject { max_open: Some(2) },
            StackBand::Low,
            OpenPlacement::BottomOfBand,
            ExistingOpenPolicy::NoOp,
            InputPolicy::Modeless,
            PresentationPolicy::None,
            FocusRestoration::PreviousEligible,
        ),

        // fact: FUN_0045aac0 restores/raises system-keyed modeless details;
        // FUN_00428b40/FUN_00429020 supply their shared presentation history.
        WindowKind::System | WindowKind::Fleet | WindowKind::Defenses | WindowKind::Missions => {
            policy(
                Cardinality::Subject { max_open: None },
                StackBand::Normal,
                OpenPlacement::TopOfBand,
                ExistingOpenPolicy::RaiseRestore,
                InputPolicy::Modeless,
                PresentationPolicy::History,
                FocusRestoration::PreviousEligible,
            )
        }

        // fact: the four Finder openers, Message Index, and Encyclopedia use
        // canonical subjectless keys, no-op on duplicate, and bit-0 dialog
        // policy. The manual and retained Wine runs corroborate the strategic
        // modal boundary for the reachable families.
        WindowKind::SystemFinder
        | WindowKind::FleetFinder
        | WindowKind::TroopFinder
        | WindowKind::PersonnelFinder
        | WindowKind::MessageIndex
        | WindowKind::Encyclopedia => policy(
            Cardinality::Singleton,
            StackBand::Normal,
            OpenPlacement::TopOfBand,
            ExistingOpenPolicy::NoOp,
            InputPolicy::StrategicModal,
            PresentationPolicy::None,
            FocusRestoration::PreviousEligible,
        ),

        // fact: the 2026-10-12 retained original-runtime closeout confirms the
        // blocking boundary for Status, Mission, and Move on both factions.
        // Their per-open logical context remains app-owned.
        WindowKind::Status | WindowKind::MissionDialog | WindowKind::MoveConfirmation => policy(
            Cardinality::Subject { max_open: None },
            StackBand::Normal,
            OpenPlacement::TopOfBand,
            ExistingOpenPolicy::NoOp,
            InputPolicy::StrategicModal,
            PresentationPolicy::None,
            FocusRestoration::PreviousEligible,
        ),

        // port: current Build Selection content is a subjectless singleton;
        // native duplicate behavior remains unresolved. Its strategic-modal
        // boundary is corroborated by the 2026-10-12 runtime closeout.
        WindowKind::BuildSelection => policy(
            Cardinality::Singleton,
            StackBand::Normal,
            OpenPlacement::TopOfBand,
            ExistingOpenPolicy::NoOp,
            InputPolicy::StrategicModal,
            PresentationPolicy::None,
            FocusRestoration::PreviousEligible,
        ),

        // port: Scrap uses the same conservative whole-strategic-view modal
        // safety boundary as the other command confirmations.
        WindowKind::ScrapConfirmation => policy(
            Cardinality::Subject { max_open: None },
            StackBand::Normal,
            OpenPlacement::TopOfBand,
            ExistingOpenPolicy::NoOp,
            InputPolicy::StrategicModal,
            PresentationPolicy::None,
            FocusRestoration::PreviousEligible,
        ),

        // port: Battle Alert has native bit-0 dialog policy, but its complete
        // pointer scope remains an explicit conservative choice.
        WindowKind::BattleAlert => policy(
            Cardinality::Subject { max_open: None },
            StackBand::Normal,
            OpenPlacement::TopOfBand,
            ExistingOpenPolicy::NoOp,
            InputPolicy::StrategicModal,
            PresentationPolicy::None,
            FocusRestoration::PreviousEligible,
        ),

        // fact: the manual and native type 0x1c bit-0 path classify Battle
        // Results/Summary as modal; visible original-runtime capture is open.
        WindowKind::BattleResults => policy(
            Cardinality::Subject { max_open: None },
            StackBand::Normal,
            OpenPlacement::TopOfBand,
            ExistingOpenPolicy::NoOp,
            InputPolicy::StrategicModal,
            PresentationPolicy::None,
            FocusRestoration::PreviousEligible,
        ),

        // port: one active Menu presentation is atomically replaced by its
        // coordinator. Outside press is consumed before explicit dismissal;
        // only Menu restores its saved invoker focus.
        WindowKind::Menu => policy(
            Cardinality::Singleton,
            StackBand::Transient,
            OpenPlacement::AboveOwner,
            ExistingOpenPolicy::NoOp,
            InputPolicy::DismissOnOutsidePress,
            PresentationPolicy::None,
            FocusRestoration::SavedMenuTarget,
        ),
    }
}

/// Builds a subjectless spec using the declared family policy.
#[must_use]
pub const fn singleton_spec(kind: WindowKind, owner: WindowOwner) -> WindowSpec {
    WindowSpec {
        key: WindowKey::singleton(kind),
        owner,
        policy: policy_for(kind),
    }
}

/// Builds a contextual spec using the declared family policy.
#[must_use]
pub const fn contextual_spec(
    kind: WindowKind,
    subject: SubjectId,
    owner: WindowOwner,
) -> WindowSpec {
    WindowSpec {
        key: WindowKey::contextual(kind, subject),
        owner,
        policy: policy_for(kind),
    }
}

const fn policy(
    cardinality: Cardinality,
    band: StackBand,
    placement: OpenPlacement,
    existing: ExistingOpenPolicy,
    input: InputPolicy,
    presentation: PresentationPolicy,
    restoration: FocusRestoration,
) -> WindowPolicy {
    WindowPolicy {
        cardinality,
        band,
        placement,
        existing,
        input,
        presentation,
        focus_on_open: FocusOnOpen::Window,
        restoration,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use rebellion_core::dat::{ExplorationStatus, Faction, SectorGroup};
    use rebellion_core::ids::DatId;
    use rebellion_core::world::{ControlKind, GameWorld, Sector, System};
    use rebellion_windowing::{
        Cardinality, ExistingOpenPolicy, FocusOnOpen, FocusRestoration, InputPolicy, OpenPlacement,
        PresentationPolicy, StackBand, SubjectId, WindowKind, WindowOwner, WindowPolicy,
    };

    use super::{
        contextual_spec, policy_for, sector_subject, singleton_spec, system_subject,
        SUBJECT_SECTOR, SUBJECT_SYSTEM,
    };

    const fn policy(
        cardinality: Cardinality,
        band: StackBand,
        placement: OpenPlacement,
        existing: ExistingOpenPolicy,
        input: InputPolicy,
        presentation: PresentationPolicy,
        restoration: FocusRestoration,
    ) -> WindowPolicy {
        WindowPolicy {
            cardinality,
            band,
            placement,
            existing,
            input,
            presentation,
            focus_on_open: FocusOnOpen::Window,
            restoration,
        }
    }

    const SUBJECT: Cardinality = Cardinality::Subject { max_open: None };
    const SINGLETON: Cardinality = Cardinality::Singleton;

    #[test]
    fn windowing_policy_inventory_covers_all_nineteen_kinds() {
        // `ghidra/notes/strategic-windowing-toolkit.md`, "Window-family
        // matrix", supplies the recovered family axes. Scrap modality and
        // outside-press Menu dismissal are explicitly labeled port policy.
        let cases = [
            (
                WindowKind::Sector,
                policy(
                    Cardinality::Subject { max_open: Some(2) },
                    StackBand::Low,
                    OpenPlacement::BottomOfBand,
                    ExistingOpenPolicy::NoOp,
                    InputPolicy::Modeless,
                    PresentationPolicy::None,
                    FocusRestoration::PreviousEligible,
                ),
                false,
            ),
            (
                WindowKind::System,
                policy(
                    SUBJECT,
                    StackBand::Normal,
                    OpenPlacement::TopOfBand,
                    ExistingOpenPolicy::RaiseRestore,
                    InputPolicy::Modeless,
                    PresentationPolicy::History,
                    FocusRestoration::PreviousEligible,
                ),
                false,
            ),
            (
                WindowKind::Fleet,
                policy(
                    SUBJECT,
                    StackBand::Normal,
                    OpenPlacement::TopOfBand,
                    ExistingOpenPolicy::RaiseRestore,
                    InputPolicy::Modeless,
                    PresentationPolicy::History,
                    FocusRestoration::PreviousEligible,
                ),
                false,
            ),
            (
                WindowKind::Defenses,
                policy(
                    SUBJECT,
                    StackBand::Normal,
                    OpenPlacement::TopOfBand,
                    ExistingOpenPolicy::RaiseRestore,
                    InputPolicy::Modeless,
                    PresentationPolicy::History,
                    FocusRestoration::PreviousEligible,
                ),
                false,
            ),
            (
                WindowKind::Missions,
                policy(
                    SUBJECT,
                    StackBand::Normal,
                    OpenPlacement::TopOfBand,
                    ExistingOpenPolicy::RaiseRestore,
                    InputPolicy::Modeless,
                    PresentationPolicy::History,
                    FocusRestoration::PreviousEligible,
                ),
                false,
            ),
            (
                WindowKind::SystemFinder,
                policy(
                    SINGLETON,
                    StackBand::Normal,
                    OpenPlacement::TopOfBand,
                    ExistingOpenPolicy::NoOp,
                    InputPolicy::StrategicModal,
                    PresentationPolicy::None,
                    FocusRestoration::PreviousEligible,
                ),
                true,
            ),
            (
                WindowKind::FleetFinder,
                policy(
                    SINGLETON,
                    StackBand::Normal,
                    OpenPlacement::TopOfBand,
                    ExistingOpenPolicy::NoOp,
                    InputPolicy::StrategicModal,
                    PresentationPolicy::None,
                    FocusRestoration::PreviousEligible,
                ),
                true,
            ),
            (
                WindowKind::TroopFinder,
                policy(
                    SINGLETON,
                    StackBand::Normal,
                    OpenPlacement::TopOfBand,
                    ExistingOpenPolicy::NoOp,
                    InputPolicy::StrategicModal,
                    PresentationPolicy::None,
                    FocusRestoration::PreviousEligible,
                ),
                true,
            ),
            (
                WindowKind::PersonnelFinder,
                policy(
                    SINGLETON,
                    StackBand::Normal,
                    OpenPlacement::TopOfBand,
                    ExistingOpenPolicy::NoOp,
                    InputPolicy::StrategicModal,
                    PresentationPolicy::None,
                    FocusRestoration::PreviousEligible,
                ),
                true,
            ),
            (
                WindowKind::MessageIndex,
                policy(
                    SINGLETON,
                    StackBand::Normal,
                    OpenPlacement::TopOfBand,
                    ExistingOpenPolicy::NoOp,
                    InputPolicy::StrategicModal,
                    PresentationPolicy::None,
                    FocusRestoration::PreviousEligible,
                ),
                true,
            ),
            (
                WindowKind::Encyclopedia,
                policy(
                    SINGLETON,
                    StackBand::Normal,
                    OpenPlacement::TopOfBand,
                    ExistingOpenPolicy::NoOp,
                    InputPolicy::StrategicModal,
                    PresentationPolicy::None,
                    FocusRestoration::PreviousEligible,
                ),
                true,
            ),
            (
                WindowKind::Status,
                policy(
                    SUBJECT,
                    StackBand::Normal,
                    OpenPlacement::TopOfBand,
                    ExistingOpenPolicy::NoOp,
                    InputPolicy::StrategicModal,
                    PresentationPolicy::None,
                    FocusRestoration::PreviousEligible,
                ),
                false,
            ),
            (
                WindowKind::BuildSelection,
                policy(
                    SINGLETON,
                    StackBand::Normal,
                    OpenPlacement::TopOfBand,
                    ExistingOpenPolicy::NoOp,
                    InputPolicy::StrategicModal,
                    PresentationPolicy::None,
                    FocusRestoration::PreviousEligible,
                ),
                true,
            ),
            (
                WindowKind::MissionDialog,
                policy(
                    SUBJECT,
                    StackBand::Normal,
                    OpenPlacement::TopOfBand,
                    ExistingOpenPolicy::NoOp,
                    InputPolicy::StrategicModal,
                    PresentationPolicy::None,
                    FocusRestoration::PreviousEligible,
                ),
                false,
            ),
            (
                WindowKind::MoveConfirmation,
                policy(
                    SUBJECT,
                    StackBand::Normal,
                    OpenPlacement::TopOfBand,
                    ExistingOpenPolicy::NoOp,
                    InputPolicy::StrategicModal,
                    PresentationPolicy::None,
                    FocusRestoration::PreviousEligible,
                ),
                false,
            ),
            (
                WindowKind::ScrapConfirmation,
                policy(
                    SUBJECT,
                    StackBand::Normal,
                    OpenPlacement::TopOfBand,
                    ExistingOpenPolicy::NoOp,
                    InputPolicy::StrategicModal,
                    PresentationPolicy::None,
                    FocusRestoration::PreviousEligible,
                ),
                false,
            ),
            (
                WindowKind::BattleAlert,
                policy(
                    SUBJECT,
                    StackBand::Normal,
                    OpenPlacement::TopOfBand,
                    ExistingOpenPolicy::NoOp,
                    InputPolicy::StrategicModal,
                    PresentationPolicy::None,
                    FocusRestoration::PreviousEligible,
                ),
                false,
            ),
            (
                WindowKind::BattleResults,
                policy(
                    SUBJECT,
                    StackBand::Normal,
                    OpenPlacement::TopOfBand,
                    ExistingOpenPolicy::NoOp,
                    InputPolicy::StrategicModal,
                    PresentationPolicy::None,
                    FocusRestoration::PreviousEligible,
                ),
                false,
            ),
            (
                WindowKind::Menu,
                policy(
                    SINGLETON,
                    StackBand::Transient,
                    OpenPlacement::AboveOwner,
                    ExistingOpenPolicy::NoOp,
                    InputPolicy::DismissOnOutsidePress,
                    PresentationPolicy::None,
                    FocusRestoration::SavedMenuTarget,
                ),
                true,
            ),
        ];

        assert_eq!(cases.len(), WindowKind::ALL.len());
        assert_eq!(
            cases
                .iter()
                .map(|(kind, _, _)| *kind)
                .collect::<BTreeSet<_>>(),
            WindowKind::ALL.into_iter().collect()
        );

        let subject = SubjectId::new(77, 91);
        for (kind, expected, singleton) in cases {
            assert_eq!(policy_for(kind), expected, "policy mismatch for {kind:?}");
            let spec = if singleton {
                singleton_spec(kind, WindowOwner::StrategicRoot)
            } else {
                contextual_spec(kind, subject, WindowOwner::StrategicRoot)
            };
            assert_eq!(spec.policy, expected);
            assert_eq!(spec.key.kind, kind);
            assert_eq!(spec.key.subject, (!singleton).then_some(subject));
        }
    }

    #[test]
    fn windowing_subject_fleet_detail_uses_system_dat_id() {
        // `FUN_0045aac0`: Fleet is a detail keyed from the displayed system;
        // a selected FleetKey remains renderer content, not window identity.
        let (world, _, system) = fixture_world();
        let subject = system_subject(&world, system).expect("fixture system exists");
        let spec = contextual_spec(WindowKind::Fleet, subject, WindowOwner::StrategicRoot);

        assert_eq!(subject, SubjectId::new(SUBJECT_SYSTEM, 0x9000_0123));
        assert_eq!(spec.key.subject, Some(subject));
        assert_eq!(spec.key.kind, WindowKind::Fleet);
    }

    #[test]
    fn windowing_subject_sector_and_system_namespaces_do_not_alias_dat_values() {
        // `FUN_00429ce0` and `FUN_0045aac0`: Sector and detail identities use
        // different semantic domains even when their DAT values coincide.
        let (world, sector, system) = fixture_world();

        assert_eq!(
            sector_subject(&world, sector),
            Some(SubjectId::new(SUBJECT_SECTOR, 0x9000_0123))
        );
        assert_eq!(
            system_subject(&world, system),
            Some(SubjectId::new(SUBJECT_SYSTEM, 0x9000_0123))
        );
        assert_eq!(sector_subject(&world, Default::default()), None);
        assert_eq!(system_subject(&world, Default::default()), None);
    }

    fn fixture_world() -> (
        GameWorld,
        rebellion_core::ids::SectorKey,
        rebellion_core::ids::SystemKey,
    ) {
        let mut world = GameWorld::default();
        let dat_id = DatId::new(0x9000_0123);
        let sector = world.sectors.insert(Sector {
            dat_id,
            name: "Fixture Sector".into(),
            group: SectorGroup::Core,
            x: 0,
            y: 0,
            systems: Vec::new(),
        });
        let system = world.systems.insert(System {
            dat_id,
            name: "Fixture System".into(),
            sector,
            x: 0,
            y: 0,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 1.0,
            popularity_empire: 0.0,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: Vec::new(),
            ground_units: Vec::new(),
            special_forces: Vec::new(),
            fighters: Vec::new(),
            defense_facilities: Vec::new(),
            manufacturing_facilities: Vec::new(),
            production_facilities: Vec::new(),
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Alliance),
        });
        world.sectors[sector].systems.push(system);
        (world, sector, system)
    }
}
