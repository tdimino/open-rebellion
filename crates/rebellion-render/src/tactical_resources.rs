//! Source-proven joins from game-data identities to original tactical graphics.
//!
//! `FUN_00597610` constructs the original ship registry in the same ordinal
//! space consumed through vtable slot `+0x24` by `FUN_005ab650`. The registry
//! order is not identical to CAPSHPSD or FIGHTSD record order, so callers must
//! use these explicit joins rather than arithmetic on a [`DatId`].

use rebellion_core::ids::DatId;

/// Original type-301 mesh family selected for one capital-ship class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TacticalCapitalShipResource {
    pub tactical_ordinal: u8,
    pub mesh_resource_base: u32,
}

/// Original type-303 fighter graphics selected for one fighter class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TacticalFighterResource {
    pub tactical_ordinal: u8,
    pub first_side_resource_base: u32,
    pub opposing_side_resource_base: u32,
}

/// The Death Star is selected outside the 29-entry capital-ship mesh table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TacticalDeathStarResource {
    pub tactical_ordinal: u8,
    pub resource_base_without_flag: u32,
    pub resource_base_with_flag: u32,
}

pub const DEATH_STAR_DAT_ID: DatId = DatId(136);
pub const DEATH_STAR_TACTICAL_RESOURCE: TacticalDeathStarResource = TacticalDeathStarResource {
    tactical_ordinal: 37,
    resource_base_without_flag: 5010,
    resource_base_with_flag: 5020,
};

// DAT identity, original tactical ordinal, type-301 resource-family base.
const CAPITAL_SHIP_JOIN: [(u32, u8, u32); 29] = [
    (64, 0, 2010),
    (65, 1, 2020),
    (66, 2, 2030),
    (67, 3, 2040),
    (68, 4, 2050),
    (69, 5, 2060),
    (70, 6, 2070),
    (71, 7, 2080),
    (72, 8, 2090),
    (73, 9, 2100),
    (74, 10, 2110),
    (77, 11, 2120),
    (75, 12, 2130),
    (76, 13, 2140),
    (78, 14, 2150),
    (128, 15, 2510),
    (129, 16, 2520),
    (130, 17, 2530),
    (131, 18, 2540),
    (132, 19, 2550),
    (133, 20, 2560),
    (134, 21, 2570),
    (135, 22, 2580),
    (137, 23, 2590),
    (138, 24, 2600),
    (139, 25, 2610),
    (140, 26, 2620),
    (141, 27, 2630),
    (142, 28, 2640),
];

// DAT identity, original tactical ordinal, first-side and opposing-side
// type-303 bases. `FUN_005ab650` chooses the side variant at battle setup.
const FIGHTER_JOIN: [(u32, u8, u32, u32); 8] = [
    (3, 29, 4000, 4004),
    (4, 30, 4010, 4014),
    (1, 31, 4020, 4024),
    (2, 32, 4030, 4034),
    (5, 33, 4100, 4104),
    (6, 34, 4110, 4114),
    (7, 35, 4120, 4124),
    (8, 36, 4130, 4134),
];

/// Resolve a CAPSHPSD identity to its original type-301 mesh family.
///
/// The Death Star intentionally returns `None`; use
/// [`death_star_tactical_resource`] for its separate path.
#[must_use]
pub fn capital_ship_tactical_resource(dat_id: DatId) -> Option<TacticalCapitalShipResource> {
    CAPITAL_SHIP_JOIN
        .iter()
        .find(|(candidate, ..)| *candidate == dat_id.index())
        .map(
            |(_, tactical_ordinal, mesh_resource_base)| TacticalCapitalShipResource {
                tactical_ordinal: *tactical_ordinal,
                mesh_resource_base: *mesh_resource_base,
            },
        )
}

/// Resolve the original Death Star class and its two source resource bases.
#[must_use]
pub fn death_star_tactical_resource(dat_id: DatId) -> Option<TacticalDeathStarResource> {
    (dat_id.index() == DEATH_STAR_DAT_ID.index()).then_some(DEATH_STAR_TACTICAL_RESOURCE)
}

/// Resolve a FIGHTSD identity to its original side-dependent type-303 pair.
#[must_use]
pub fn fighter_tactical_resource(dat_id: DatId) -> Option<TacticalFighterResource> {
    FIGHTER_JOIN
        .iter()
        .find(|(candidate, ..)| *candidate == dat_id.index())
        .map(
            |(_, tactical_ordinal, first_side, opposing_side)| TacticalFighterResource {
                tactical_ordinal: *tactical_ordinal,
                first_side_resource_base: *first_side,
                opposing_side_resource_base: *opposing_side,
            },
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn capital_ship_join_preserves_source_registry_permutations() {
        assert_eq!(
            capital_ship_tactical_resource(DatId::new(77)),
            Some(TacticalCapitalShipResource {
                tactical_ordinal: 11,
                mesh_resource_base: 2120,
            })
        );
        assert_eq!(
            capital_ship_tactical_resource(DatId::new(75)),
            Some(TacticalCapitalShipResource {
                tactical_ordinal: 12,
                mesh_resource_base: 2130,
            })
        );
        assert_eq!(
            capital_ship_tactical_resource(DatId::new(133)),
            Some(TacticalCapitalShipResource {
                tactical_ordinal: 20,
                mesh_resource_base: 2560,
            })
        );
        assert_eq!(capital_ship_tactical_resource(DEATH_STAR_DAT_ID), None);
        assert_eq!(
            death_star_tactical_resource(DEATH_STAR_DAT_ID),
            Some(DEATH_STAR_TACTICAL_RESOURCE)
        );
    }

    #[test]
    fn fighter_join_preserves_source_registry_permutations() {
        assert_eq!(
            fighter_tactical_resource(DatId::new(3)),
            Some(TacticalFighterResource {
                tactical_ordinal: 29,
                first_side_resource_base: 4000,
                opposing_side_resource_base: 4004,
            })
        );
        assert_eq!(
            fighter_tactical_resource(DatId::new(1)),
            Some(TacticalFighterResource {
                tactical_ordinal: 31,
                first_side_resource_base: 4020,
                opposing_side_resource_base: 4024,
            })
        );
    }

    #[test]
    fn complete_original_dat_domain_maps_once() {
        let capital_dat_ids: HashSet<_> = CAPITAL_SHIP_JOIN
            .iter()
            .map(|(dat_id, ..)| *dat_id)
            .collect();
        let capital_ordinals: HashSet<_> = CAPITAL_SHIP_JOIN
            .iter()
            .map(|(_, ordinal, _)| *ordinal)
            .collect();
        let capital_bases: HashSet<_> =
            CAPITAL_SHIP_JOIN.iter().map(|(_, _, base)| *base).collect();
        assert_eq!(capital_dat_ids.len(), 29);
        assert_eq!(capital_ordinals, (0_u8..29).collect());
        assert_eq!(capital_bases.len(), 29);

        let fighter_dat_ids: HashSet<_> = FIGHTER_JOIN.iter().map(|(dat_id, ..)| *dat_id).collect();
        let fighter_ordinals: HashSet<_> = FIGHTER_JOIN
            .iter()
            .map(|(_, ordinal, ..)| *ordinal)
            .collect();
        assert_eq!(fighter_dat_ids, (1_u32..=8).collect());
        assert_eq!(fighter_ordinals, (29_u8..=36).collect());
    }

    #[test]
    fn unknown_or_cross_family_ids_never_guess() {
        assert_eq!(capital_ship_tactical_resource(DatId::new(63)), None);
        assert_eq!(capital_ship_tactical_resource(DatId::new(143)), None);
        assert_eq!(fighter_tactical_resource(DatId::new(0)), None);
        assert_eq!(fighter_tactical_resource(DatId::new(9)), None);
        assert_eq!(death_star_tactical_resource(DatId::new(135)), None);
    }
}
