//! Space tactical battle engine, shared by played and unplayed battles.
//!
//! `BattleSession` holds one battle's ships, fighters, orders and effects and
//! advances it in source-sized steps: `step` for the windowed battle and
//! `headless_step`/`simulate_to_end` for a battle nobody watches, which the
//! original runs through the same engine (`FUN_0040a050` state 8,
//! `ghidra/notes/simulate-runs-tactical-engine.md`). Drawing, input and audio
//! playback live in `rebellion-render`; this module only queues effect, cue
//! and log records for them.

pub mod apply;
pub mod resource_ids;
#[cfg(any(test, feature = "test-support"))]
pub mod test_support;
#[cfg(test)]
mod hangar_tests;
#[cfg(test)]
mod repair_tests;
#[cfg(test)]
mod station_tests;
#[cfg(test)]
mod withdrawal_tests;
#[cfg(test)]
mod tests;

use crate::combat::CombatForce;
use crate::ids::{CapitalShipKey, DatId, FighterKey, FleetKey, SystemKey};
use crate::world::GameWorld;

use resource_ids::{
    capital_ship_tactical_resource, fighter_tactical_resource, TacticalCapitalShipResource, TacticalDeathStarResource, TacticalFighterResource,
    DEATH_STAR_TACTICAL_RESOURCE,
};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// port: battlefield dimensions in the port's logical arena units.
pub const ARENA_WIDTH: f32 = 1200.0;
/// port: see [`ARENA_WIDTH`].
pub const ARENA_HEIGHT: f32 = 800.0;

/// port: deployment zone width (fraction of arena width per side).
pub const DEPLOY_ZONE_FRACTION: f32 = 0.3;

/// `FUN_005e45f0` and `FUN_005e7540` place the five condition indicators at
/// panel-local x positions 10, 37, 64, 91, and 118 with y 103.
const TACTICAL_SUBSYSTEM_RESOURCE_BASES: [u32; 5] = [1201, 1206, 1211, 1216, 1221];

/// `_DAT_0066d154`, used by both `FUN_005d3cc0` and `FUN_005d41a0`.
const ORIGINAL_TACTICAL_EFFECT_FRAME_SECONDS: f32 = 0.1;
/// `FUN_005b1ea0` compares the event strength with `DAT_0066c4d8`.
pub const ORIGINAL_LASER_PROJECTILE_THRESHOLD: f32 = 28.8;
/// `FUN_005b1f60` compares the event strength with `DAT_0066c4dc`.
pub const ORIGINAL_TURBOLASER_PROJECTILE_THRESHOLD: f32 = 34.666_668;
/// `FUN_005b2080` compares the event strength with `DAT_0066c4e0`.
pub const ORIGINAL_ION_PROJECTILE_THRESHOLD: f32 = 32.0;
/// `FUN_005b2c70` compares the event strength with `DAT_0066c4e4`.
const ORIGINAL_TORPEDO_PROJECTILE_THRESHOLD: f32 = 12.8;
/// `FUN_005ee590` projectile variants one and two live one second.
const ORIGINAL_PROJECTILE_DURATION_SHORT: f32 = 1.0;
/// `FUN_005ee590` projectile variant three lives two seconds.
const ORIGINAL_PROJECTILE_DURATION_LONG: f32 = 2.0;
/// `FUN_005b0330` resets the subsystem-repair timer to 50 * 1000 ms.
const ORIGINAL_SUBSYSTEM_REPAIR_INTERVAL_TICKS: u32 = 200;
/// Played steps in one source second, the unit pass's 1000 ms turn
/// (`FUN_005bea30`, `+0x8a8`).
const ORIGINAL_STEPS_PER_SECOND: u32 = 4;
/// `FUN_005b05c0` converts the CAPSHPSD sublight rating by 0.4 * 0.6.
const ORIGINAL_SUBLIGHT_ENGINE_SCALE: f32 = 0.24;
/// `FUN_005b9c60` converts the FIGHTSD sublight rating by 0.4 without the
/// capital-ship 0.6 multiplier.
const ORIGINAL_FIGHTER_SUBLIGHT_ENGINE_SCALE: f32 = 0.4;
/// hyp: tactical combat advances four simulation steps per second. The
/// rendered window's own step interval is not recovered; its frame timer
/// settles it.
pub const ORIGINAL_TACTICAL_STEP_MILLISECONDS: f32 = 250.0;
/// `FUN_005ad750` and `FUN_005afb70` clamp the active mode contribution.
const ORIGINAL_ENGINE_MODE_MIN_BONUS: f32 = 1.0;
/// `FUN_005ad750` clamps at `DAT_0066c2b8` (9.0).
const ORIGINAL_ENGINE_MODE_MAX_BONUS: f32 = 9.0;
/// `_DAT_0066d07c`, applied after each source maneuver rotation.
const ORIGINAL_MANEUVER_WAYPOINT_SCALE: f32 = 0.75;
/// `_DAT_0066d094` and `_DAT_0066d098` multiply pi by +/- one eighth.
const ORIGINAL_MANEUVER_ANGLE: f32 = std::f32::consts::PI / 8.0;
/// `_DAT_0066d088`, consumed by `FUN_005cf980` before recovery state 3.
const ORIGINAL_FIGHTER_DOCKING_DISTANCE: f32 = 2.0;
/// `FUN_005ba270` clamps the effective capital maneuverability at nine.
const ORIGINAL_CAPITAL_MANEUVERABILITY_MAX: f32 = 9.0;
/// The fallback turn rate installed by `FUN_005b9c60` is pi / 12 radians/s.
const ORIGINAL_CAPITAL_TURN_RATE_FALLBACK: f32 = std::f32::consts::PI / 12.0;
/// `FUN_005d2360` initializes every capital-ship weapon family with this
/// event-strength coefficient.
const ORIGINAL_CAPITAL_WEAPON_STRENGTH: f32 = f32::from_bits(0x3e88_8889);
/// `FUN_005d24e0` makes laser cannon fire deliberately weak against capital
/// ships. Its exact immediate is `0x3e2b020c`.
const ORIGINAL_LASER_VS_CAPITAL: f32 = f32::from_bits(0x3e2b_020c);
/// `FUN_005b05c0` converts the CAPSHPSD weapon recharge rating by 3.75.
const ORIGINAL_WEAPON_RECHARGE_SCALE: f32 = 3.75;
/// One fighter craft contributes eight floating hull points: `FUN_005b9c60`
/// multiplies the craft count by `DAT_0066c6b8` (8.0).
pub const ORIGINAL_FIGHTER_HULL_PER_CRAFT: f32 = 8.0;
/// `FUN_005b9c00` scales a fighter group's live hull into weapon output.
const ORIGINAL_FIGHTER_WEAPON_HULL_SCALE: f32 = 3.751;
/// `FUN_005b9c30` scales live fighter hull into shield recharge per second.
const ORIGINAL_FIGHTER_SHIELD_HULL_SCALE: f32 = 0.025;
/// `_DAT_0066c4cc` (0.53333336): `FUN_005b23a0` weighs a squadron's
/// torpedoes (`+0x134`) by this.
const ORIGINAL_FIGHTER_TORPEDO_FACTOR: f32 = f32::from_bits(0x3f08_8889);
/// `FUN_005d2490` and `FUN_005d24e0` target modifiers.
const ORIGINAL_ION_VS_FIGHTER: f32 = 0.0;
const ORIGINAL_LASER_VS_FIGHTER: f32 = 1.0;
/// `FUN_005d2490` stores `0x3f000000` (0.5).
const ORIGINAL_TURBOLASER_VS_FIGHTER: f32 = 0.5;
/// hyp: [`ORIGINAL_TACTICAL_STEP_MILLISECONDS`] in seconds.
const ORIGINAL_TACTICAL_STEP_SECONDS: f32 = ORIGINAL_TACTICAL_STEP_MILLISECONDS / 1000.0;
/// `_DAT_0066c3e0`: once neither side's strength has fallen to a new low for
/// this many seconds, `FUN_005ae9c0` withdraws the weaker side.
const ORIGINAL_STALEMATE_SECONDS: f32 = 1200.0;
/// `_DAT_0066d0f0`, the Empire's strength while its Death Star is
/// operational (`FUN_005d0ce0.c:83`).
const ORIGINAL_DEATH_STAR_STRENGTH: f32 = 20_000_000.0;
/// port: a ceiling on a battle run without its window, far beyond the
/// stalemate (six hours of battle time).
const SIMULATED_BATTLE_SECOND_LIMIT: u32 = 6 * 60 * 60;
/// `FUN_0059a530` steps the headless battle by `DAT_0066badc` = 1.0 second.
const ORIGINAL_HEADLESS_STEP_SECONDS: f32 = 1.0;
/// `FUN_005ba7f0` advances the tactical Death Star laser by elapsed
/// milliseconds times one third times 0.001, capped at 100.
const ORIGINAL_DEATH_STAR_LASER_CHARGE_PER_SECOND: f32 = 1.0 / 3.0;
/// `FUN_005ba7f0` caps the charge at `DAT_0066c71c` (100.0).
pub const ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE: f32 = 100.0;
/// `FUN_005af5f0.c:38`: the headless battle generator's seed (`0xbc614e`).
const ORIGINAL_BATTLE_RNG_SEED: i32 = 0x00bc_614e;
/// `FUN_005f56b0`: the steps a seeding takes before the first draw.
const ORIGINAL_BATTLE_RNG_WARM_UP: usize = 16;
/// `_DAT_0066d248`: the station decal stands at -2.0 battle extents on x.
const ORIGINAL_STATION_DECAL_X_SCALE: f32 = -2.0;
/// `_DAT_0066d244`: the station decal's y and z offsets reach 0.2 extents.
const ORIGINAL_STATION_DECAL_JITTER: f32 = 0.2;
/// `_DAT_0066d258` (double): the superlaser frame rises 0.11 extents.
const ORIGINAL_STATION_SUPERLASER_RISE: f64 = 0.11;
/// `_DAT_0066d250` (double): the superlaser frame sits 0.09 extents back on z.
const ORIGINAL_STATION_SUPERLASER_SETBACK: f64 = 0.09;
/// `FUN_005ba7f0` adds `dt * (1/3) * 0.001` with `dt` in milliseconds; the
/// headless side step passes `DAT_0066badc` = 1.0 (`FUN_005ae9c0`).
const ORIGINAL_HEADLESS_LASER_SECONDS: f32 = 0.001;
/// `FUN_005aefc0` (`CMP EDI,0x64; JA` at `0x5af047`): the hundredth visit
/// is the last; the walk then destroys every member left.
const ORIGINAL_HEADLESS_TRENCH_RUN_VISITS: u32 = 100;
/// `FUN_005cfec0` arms the Death Star attack state with a 120,000 ms window.
pub const ORIGINAL_TRENCH_RUN_DURATION_MILLISECONDS: f32 = 120_000.0;
/// `_DAT_0066d078` (8.0, a craft's hull): `FUN_005d04e0` divides the hull
/// by it for the craft count and scales the damage draw by it.
const ORIGINAL_TRENCH_RUN_DAMAGE_SCALE: f32 = 8.0;
/// `FUN_005d04e0`'s damage when a squadron of fewer than two craft misses
/// its maneuver (`0x47c35000`).
const ORIGINAL_TRENCH_RUN_FATAL_DAMAGE: f32 = 100_000.0;
/// `FUN_005d04e0` leaves its loop for failure after this many pops
/// (`CMP EAX,0x3e8; JA` at `0x5d06ee`: the 1001st runs in full).
const ORIGINAL_TRENCH_RUN_POP_LIMIT: u32 = 1000;
/// `FUN_005ad7e0` caps the commander rating at 9.0, and `FUN_005d04e0` caps
/// it again (`_DAT_0066d0a0`).
const ORIGINAL_TRENCH_RUN_RATING_MAX: f32 = 9.0;
/// `_DAT_0066cb14`: the trench entry point lies at half the battle extent.
const ORIGINAL_TRENCH_RUN_ENTRY_SCALE: f32 = 0.5;
/// `FUN_005c6610` passes `0x3e4ccccd` (0.2) to the mover `FUN_005eb1a0`,
/// which reports arrival within that distance.
const ORIGINAL_TRENCH_RUN_ARRIVAL_DISTANCE: f32 = 0.2;
/// `FUN_005eb1a0` moves at the unit's speed times 0.95.
const ORIGINAL_TRENCH_RUN_SPEED_SCALE: f32 = 0.95;
/// `FUN_005ad530`: a withdrawing side waits at least 30000.0 ms
/// (`DAT_0066c240`).
const ORIGINAL_WITHDRAWAL_COUNTDOWN_MILLISECONDS: f32 = 30_000.0;
/// `FUN_005ad530`: side 1 with its station operational waits at least
/// 150000.0 ms (`DAT_0066c234`).
const ORIGINAL_STATION_WITHDRAWAL_COUNTDOWN_MILLISECONDS: f32 = 150_000.0;
/// The countdown starts at the retreat system's transit ticks × 0.4
/// (`FUN_00595d60`, `_DAT_0066babc`)...
pub(crate) const ORIGINAL_WITHDRAWAL_TRANSIT_SCALE: f32 = 0.4;
/// ... × 1000.0 (`FUN_005a8b10`, `_DAT_0066c288`).
pub(crate) const ORIGINAL_WITHDRAWAL_TRANSIT_MILLISECONDS: f32 = 1_000.0;
/// The withdrawal line lies at half `battle+0x8cc` on the side's own edge
/// (`_DAT_0066cb14`, `_DAT_0066cffc`); the Withdraw executors pass the mover
/// a tolerance of 1.0 (`FUN_005c6610.c:69`, `FUN_005cc300.c:75`).
const ORIGINAL_WITHDRAWAL_LINE_SCALE: f32 = 0.5;
const ORIGINAL_WITHDRAWAL_ARRIVAL_DISTANCE: f32 = 1.0;
/// A unit within 8.0 of the line may jump (`_DAT_0066cb0c`, `_DAT_0066cff0`).
const ORIGINAL_WITHDRAWAL_JUMP_DISTANCE: f32 = 8.0;
/// A jump (state 7) lasts 4000.0 ms (`FUN_005a6b40`, `_DAT_0066c24c`).
const ORIGINAL_JUMP_MILLISECONDS: f32 = 4_000.0;
/// The jump's flight out, by variant: the distance (`DAT_0066c258`-`264`)
/// covered by the time (4000.0 less `DAT_0066c270`-`278`, variant 0 none)
/// from which the static initialisers at `0x005a69b0`-`0x005a6a7c` set the
/// rates `DAT_006bc47c`, `488`, `484` and `494` (distance over seconds
/// squared, `_DAT_0066c26c` = 0.001).
const ORIGINAL_JUMP_FLIGHTS: [(f32, f32); 4] = [
    (400.0, ORIGINAL_JUMP_MILLISECONDS),
    (575.0, ORIGINAL_JUMP_MILLISECONDS - 250.0),
    (650.0, ORIGINAL_JUMP_MILLISECONDS - 75.0),
    (325.0, ORIGINAL_JUMP_MILLISECONDS - 275.0),
];
/// The object manager counts side 0's (the Alliance's) ids from 0 and side
/// 1's from 1000000 (`FUN_005a6290.c:22-30`); each new unit takes the next
/// (`FUN_005a6620`, through `FUN_005a6660`).
const ORIGINAL_SIDE_1_OBJECT_ID_BASE: u32 = 1_000_000;
/// `FUN_005d03f0` plays no voice for a slot holding `0x13d`.
const ORIGINAL_TRENCH_RUN_SILENT_EVENT: u16 = 0x13d;
/// `FUN_005d04e0` fills the nine slots at `+0x5c` with this sequence first.
const ORIGINAL_TRENCH_RUN_SUCCESS_CHATTER: [u16; 9] = [
    0x11e, 0x11f, 0x120, 0x128, 0x121, 0x122, 0x129, 0x13d, 0x13d,
];
/// `FUN_005d04e0` rewrites slots two through seven when a participant
/// failed its maneuver and the run still succeeds.
const ORIGINAL_TRENCH_RUN_DAMAGED_SUCCESS_CHATTER: [u16; 9] = [
    0x11e, 0x123, 0x124, 0x125, 0x12a, 0x12b, 0x126, 0x13d, 0x13d,
];
/// `FUN_005d04e0` rewrites slots two through nine when the run fails.
const ORIGINAL_TRENCH_RUN_FAILURE_CHATTER: [u16; 9] = [
    0x11e, 0x123, 0x124, 0x125, 0x12a, 0x12b, 0x126, 0x12c, 0x127,
];

/// port: spacing between auto-placed ships in the logical arena.
const SHIP_SPACING: f32 = 60.0;

/// `FUN_005ab650` starts the battle extent at `+0x8cc` = 100.0.
const ORIGINAL_BATTLE_BASE_EXTENT: f32 = 100.0;
/// `FUN_005ab650` subtracts `DAT_0066c2a0` (-3.0) from the extent each time
/// one side's object count passes the other's.
const ORIGINAL_BATTLE_OBJECT_INCREMENT: f32 = 3.0;
/// `FUN_005ab650` scales the extent by `DAT_0066c29c` (0.5) for the lanes.
const ORIGINAL_BATTLE_OUTER_LANE_SCALE: f32 = 0.5;
/// `FUN_005ab650` offsets the inner lanes by `DAT_0066c2ac` (20.0).
const ORIGINAL_BATTLE_INNER_LANE_OFFSET: f32 = 20.0;
/// `FUN_005ab650` steps alternating slots by `DAT_0066c2a4` (-5.0).
const ORIGINAL_BATTLE_SLOT_SPACING: f32 = 5.0;
/// `FUN_005c81d0` multiplies maximum weapon range by 10,000 before adding
/// available sublight power and negating the ordered-list key.
const ORIGINAL_FORMATION_RANGE_PRIORITY: f32 = 10_000.0;
/// `FUN_005ea250` expands each member footprint by 1.2 before retaining the
/// largest value for `FUN_005ec400`'s alternating follower slots.
const ORIGINAL_FORMATION_FOOTPRINT_SCALE: f32 = 1.2;

/// Source-coordinate tactical envelope recovered from `FUN_005ab650`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OriginalTacticalLayout {
    pub first_active_objects: u16,
    pub second_active_objects: u16,
    pub battle_extent: f32,
    pub outer_positive_z: f32,
    pub outer_negative_z: f32,
    pub inner_negative_z: f32,
    pub inner_positive_z: f32,
}

impl OriginalTacticalLayout {
    pub fn from_active_counts(first_active_objects: usize, second_active_objects: usize) -> Self {
        let first_active_objects = u16::try_from(first_active_objects).unwrap_or(u16::MAX);
        let second_active_objects = u16::try_from(second_active_objects).unwrap_or(u16::MAX);
        let widest_force = first_active_objects.max(second_active_objects);
        let battle_extent = ORIGINAL_BATTLE_OBJECT_INCREMENT
            .mul_add(f32::from(widest_force), ORIGINAL_BATTLE_BASE_EXTENT);
        let outer_positive_z = battle_extent * ORIGINAL_BATTLE_OUTER_LANE_SCALE;
        let outer_negative_z = -outer_positive_z;
        let inner_negative_z = ORIGINAL_BATTLE_INNER_LANE_OFFSET - outer_positive_z;
        let inner_positive_z = outer_positive_z - ORIGINAL_BATTLE_INNER_LANE_OFFSET;
        Self {
            first_active_objects,
            second_active_objects,
            battle_extent,
            outer_positive_z,
            outer_negative_z,
            inner_negative_z,
            inner_positive_z,
        }
    }
}

impl Default for OriginalTacticalLayout {
    fn default() -> Self {
        Self::from_active_counts(0, 0)
    }
}

/// Build the four far-to-clustered navigation sets described by the original
/// manual. Their interaction contract is authoritative; the deterministic
/// coordinates are A1 fixtures scaled to the recovered battle envelope until
/// the executable's point producer and A0 positions can be compared.
fn original_navigation_point_sets(
    layout: OriginalTacticalLayout,
) -> [Vec<TacticalWorldPosition>; 4] {
    let extent = layout.battle_extent.max(ORIGINAL_BATTLE_BASE_EXTENT);
    let point = |x: f32, y: f32, z: f32, scale: f32| TacticalWorldPosition {
        x: x * extent * scale,
        y: y * extent * scale,
        z: z * extent * scale,
    };
    [
        vec![
            point(-0.82, 0.22, -0.78, 1.0),
            point(0.78, -0.18, -0.70, 1.0),
            point(-0.70, -0.25, 0.75, 1.0),
            point(0.84, 0.20, 0.68, 1.0),
            point(0.05, 0.48, -0.92, 1.0),
            point(-0.12, -0.44, 0.90, 1.0),
        ],
        vec![
            point(-0.72, 0.18, -0.58, 0.82),
            point(0.66, -0.16, -0.52, 0.82),
            point(-0.58, -0.22, 0.62, 0.82),
            point(0.70, 0.20, 0.55, 0.82),
            point(0.02, 0.36, 0.04, 0.82),
        ],
        vec![
            point(-0.52, 0.15, -0.42, 0.62),
            point(0.48, -0.12, -0.36, 0.62),
            point(-0.42, -0.15, 0.44, 0.62),
            point(0.50, 0.14, 0.40, 0.62),
            point(0.0, 0.26, 0.0, 0.62),
        ],
        vec![
            point(-0.24, 0.10, -0.20, 0.42),
            point(0.22, -0.08, -0.18, 0.42),
            point(-0.18, -0.10, 0.22, 0.42),
            point(0.24, 0.09, 0.20, 0.42),
        ],
    ]
}

/// Stable production identity for one battle participant. Slotmap keys remain
/// runtime-only; the original DAT identity and fleet roster index survive any
/// tactical render reordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TacticalObjectIdentity {
    pub class_dat_id: DatId,
    pub fleet_roster_index: usize,
    pub is_alliance: bool,
}

/// Initial retained-mode world coordinate recovered from the original battle
/// setup. The existing 2D simulation coordinates remain separate until its
/// movement and scale rules are recovered.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TacticalWorldPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Source retained-mode direction or velocity vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TacticalWorldVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl TacticalWorldVector {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    /// `FUN_005b0f70` initializes the current vector with `FUN_0059fb10`.
    pub const SOURCE_FORWARD: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    pub fn normalized(self) -> Self {
        let magnitude = self.x.hypot(self.y).hypot(self.z);
        if magnitude > f32::EPSILON {
            Self {
                x: self.x / magnitude,
                y: self.y / magnitude,
                z: self.z / magnitude,
            }
        } else {
            Self::ZERO
        }
    }

    fn dot(self, other: Self) -> f32 {
        self.x
            .mul_add(other.x, self.y.mul_add(other.y, self.z * other.z))
    }
}

impl TacticalWorldPosition {
    const ORIGIN: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
}

#[derive(Debug, Clone, Copy)]
struct OriginalTacticalSlots {
    index: usize,
    magnitude: f32,
    current: f32,
}

impl Default for OriginalTacticalSlots {
    fn default() -> Self {
        Self {
            index: 0,
            magnitude: 0.0,
            current: 0.0,
        }
    }
}

impl Iterator for OriginalTacticalSlots {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index & 1 == 0 {
            self.current = -self.current;
        } else {
            let prior_was_negative = self.current < 0.0;
            self.magnitude += ORIGINAL_BATTLE_SLOT_SPACING;
            self.current = if prior_was_negative {
                -self.magnitude
            } else {
                self.magnitude
            };
        }
        self.index = self.index.saturating_add(1);
        Some(self.current)
    }
}

// ---------------------------------------------------------------------------
// Battle phase
// ---------------------------------------------------------------------------

/// Current phase of the tactical battle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattlePhase {
    /// Player positions ships in deployment zone.
    Placement,
    /// Real-time combat in progress (Task #8).
    Combat,
    /// Battle over, showing results (Task #10).
    Results,
}

/// One command from the original unified tactical order field at source offset `+0x68`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TacticalOrder {
    #[default]
    None,
    /// Direct right-click protection order. The selected units follow one
    /// friendly capital ship and keep autonomous opportunity fire enabled.
    Escort,
    Recover,
    /// Order 3, given only by a side's withdrawal (`FUN_005ad530`).
    Withdraw,
    AttackFighters,
    AttackCapitalShips,
    AttackDeathStar,
    LeftHook,
    RightHook,
    Hammer,
    Anvil,
    HoldPosition,
}

/// Target identity retained by the original tactical attack-order executors.
/// Capital ships and fighter groups share one source object list, while the
/// reconstruction keeps their typed Rust collections separate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TacticalAttackTarget {
    CapitalShip(usize),
    FighterGroup(usize),
}

impl TacticalOrder {
    /// Exact numeric value consumed by the original tactical runtime:
    /// `FUN_005dd3a0` maps maneuvers 7-11 and `FUN_005df5f0` missions 2 and
    /// 4-6; `FUN_00595c00` applies the order, which `FUN_00596af0` reads at
    /// `+0x68`.
    #[must_use]
    pub const fn source_code(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Escort => 1,
            Self::Recover => 2,
            Self::Withdraw => 3,
            Self::AttackFighters => 4,
            Self::AttackCapitalShips => 5,
            Self::AttackDeathStar => 6,
            Self::LeftHook => 7,
            Self::RightHook => 8,
            Self::Hammer => 9,
            Self::Anvil => 10,
            Self::HoldPosition => 11,
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "No Orders",
            Self::Escort => "Escort",
            Self::Recover => "Recover",
            // TEXTTACT 0xdde3.
            Self::Withdraw => "Withdraw",
            Self::AttackFighters => "Attack Fighters",
            Self::AttackCapitalShips => "Attack Capital Ships",
            Self::AttackDeathStar => "Attack Death Star",
            Self::LeftHook => "Left Hook",
            Self::RightHook => "Right Hook",
            Self::Hammer => "Hammer",
            Self::Anvil => "Anvil",
            Self::HoldPosition => "Hold Position",
        }
    }
}

/// The source tactical behavior toggle at offset `+0x38`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TacticalTactic {
    Surround,
    #[default]
    StandOff,
}

impl TacticalTactic {
    /// `FUN_005dd3a0` maps Surround to 1 and Stand Off to 2; `FUN_00595c20`
    /// applies the tactic, which `FUN_00596b30` reads at `+0x38`.
    #[must_use]
    pub const fn source_code(self) -> u8 {
        match self {
            Self::Surround => 1,
            Self::StandOff => 2,
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Surround => "Surround",
            Self::StandOff => "Stand Off",
        }
    }
}

// ---------------------------------------------------------------------------
// Ship instance (per-hull in the battle)
// ---------------------------------------------------------------------------

/// One individual ship hull participating in the battle.
#[derive(Debug, Clone)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "These independent flags preserve the existing state and serialization model."
)]
pub struct TacticalShip {
    /// Which capital ship class this hull belongs to.
    pub class_key: CapitalShipKey,
    /// Stable DAT and roster identity for production-to-render joins.
    pub identity: TacticalObjectIdentity,
    /// Exact original tactical graphic identity, when this is an original class.
    pub tactical_resource: Option<TacticalCapitalShipResource>,
    /// Original retained-mode position at battle initialization.
    pub source_position: TacticalWorldPosition,
    /// Current retained-mode forward direction at offsets `+0x10` through `+0x18`.
    pub source_forward: TacticalWorldVector,
    /// Desired retained-mode direction at offsets `+0x20` through `+0x28`.
    pub source_desired_forward: TacticalWorldVector,
    /// Latest source velocity stored at offsets `+0x6c` through `+0x74`.
    pub source_velocity: TacticalWorldVector,
    /// Active executor waypoint assigned through `FUN_005a8f70`.
    pub source_waypoint: Option<TacticalWorldPosition>,
    /// Ordered navigation points assigned through the tactical display. The
    /// first entry is mirrored by `source_waypoint` while it is active.
    pub navigation_route: Vec<TacticalWorldPosition>,
    /// Mesh-derived source collision volume installed by `FUN_005ab0e0`.
    /// It becomes available when the participant's close/base mesh family is
    /// decoded and remains invariant across visual LOD changes.
    pub source_collision_envelope: Option<OriginalTacticalCollisionEnvelope>,
    /// Display name (from `CapitalShipClass`).
    pub name: String,
    /// Position on the battlefield (logical coords).
    pub x: f32,
    pub y: f32,
    /// Current hull points.
    pub hull_current: i32,
    /// Maximum hull points.
    pub hull_max: i32,
    /// Shield strength (0-100 scale).
    pub shield: i32,
    pub shield_max: i32,
    /// Shield, weapon, tractor, engine, and hyperdrive condition percentages.
    /// Values are independent of the live hull and shield-energy meters.
    pub subsystem_condition: TacticalSubsystemCondition,
    /// Source component availability used to derive the five condition values.
    pub subsystem_capacity: TacticalSubsystemCapacity,
    /// Source damage counters. The first four cap at four hits; hyperdrive
    /// caps at the number of installed normal and damaged-drive components.
    pub subsystem_damage: TacticalSubsystemDamage,
    /// Normalized maximum sublight power stored at source offset `+0x3f8`.
    pub sublight_engine_power: f32,
    /// Raw `CAPSHPSD.DAT` maneuverability consumed by `FUN_005ba270`.
    pub maneuverability: u32,
    /// Maximum tractor power stored at source offset `+0x3b8`.
    pub tractor_beam_power: f32,
    /// Number of fighter squadrons this hull can recover.
    pub fighter_capacity: u32,
    /// Active state-record value at offset `+0x4c`, or no active record.
    pub maneuver_state_value: Option<f32>,
    /// Inclusive 1 through 100 repair chance used by `FUN_005b1490`.
    pub damage_control: u8,
    /// True if this ship belongs to the attacker side.
    pub is_attacker: bool,
    /// True if the ship is still alive.
    pub alive: bool,
    /// Whether this ship is currently selected by the player.
    pub selected: bool,
    /// Unified source order code assigned through the command panel.
    pub order: TacticalOrder,
    /// Source Surround or Stand Off behavior assigned with a maneuver.
    pub tactic: TacticalTactic,
    /// Zero-based task-force assignment shown in the eight source HUD slots.
    pub task_force: u8,
    /// The strategic fleet this hull belongs to. Every fleet of a side at
    /// the system fights (`FUN_0040b0e0`), not only the lead one.
    pub fleet: Option<FleetKey>,
    /// Index into the fleet's `capital_ships` for damage application.
    pub fleet_ship_index: usize,
    /// Sprite resource ID in TACTICAL.DLL (if known).
    pub sprite_id: Option<u32>,
    /// Four source weapon records in executable order: fore, starboard, aft,
    /// and port. Each record owns readiness and its current energy reserve.
    pub weapon_arcs: [TacticalWeaponArc; 4],
    /// Laser, ion, and turbolaser engagement ranges from CAPSHPSD.DAT.
    pub weapon_ranges: TacticalWeaponRanges,
    /// Base energy restored per source second after the constructor's 3.75
    /// conversion.
    pub weapon_recharge_rate: f32,
    /// Base shield energy restored per source second.
    pub shield_recharge_rate: f32,
    /// Fractional recharge retained while the public shield meter remains an
    /// integer value.
    pub shield_recharge_carry: f32,
    /// Unique FIFO of fired arcs, matching `FUN_005b64e0` and `FUN_005b6320`.
    pub weapon_recharge_queue: Vec<u8>,
    /// Active target selected by a source attack-order executor or focus input.
    pub attack_target: Option<TacticalAttackTarget>,
    /// Player-authored target list. The original completes navigation points
    /// before attacking these targets in selection order.
    pub manual_targets: Vec<TacticalAttackTarget>,
    /// Friendly capital-ship index assigned by the original direct Escort
    /// command (order code 1), or no active escort target.
    pub escort_target: Option<usize>,
    /// Personnel and troops assigned to this physical hull for the selected-
    /// ship contents monitor.
    pub contents: Vec<TacticalShipContent>,
    /// The ship is in its jump (state 7, `FUN_005b1e40`): no longer alive
    /// for `FUN_005ad870`, so it neither fires nor is a target.
    pub retreating: bool,
    /// Milliseconds the jump has run; it ends at 4000.0 (`FUN_005a6b40`).
    pub retreat_progress: f32,
    /// The ship has left the battle (state 8, `0x005aa2ad`).
    pub retreated: bool,
    /// `+0x158`: outstanding subsystem hits. `FUN_005b05c0.c:150-157` seeds
    /// it from the carried engine, shield, hyperdrive and tractor hits (not
    /// the weapon hits), `FUN_005b1970` adds every new hit, even past a
    /// family's cap, and a repair (`FUN_005b1ab0`) takes one off.
    pub outstanding_hits: u32,
    /// `+0x3f4`: steps until the next repair roll, 0 at the start
    /// (`FUN_005b05c0.c:142`).
    pub repair_timer_steps: u32,
}

/// Capital-ship collision volume consumed by `FUN_005b2e60`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OriginalTacticalCollisionEnvelope {
    pub vertical_radius: f32,
    pub planar_diameter: f32,
}

/// One compact GOKRES-backed entry in the selected ship's contents monitor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TacticalShipContent {
    pub label: String,
    pub resource_id: u32,
}

/// Resolve the compact 61x25 GOKRES assignment image used by original system
/// and tactical contents windows. These are separate from the 80x80 officer
/// portraits and preserve the executable's major/minor resource bands.
#[must_use]
pub fn tactical_character_content_resource(dat_id: DatId, is_major: bool) -> Option<u32> {
    character_mini_resource_id(dat_id, is_major)
}

/// One of the four 0x74-byte capital-ship battery records initialized by
/// `FUN_005b0e50` and consumed by `FUN_005b3f10`.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct TacticalWeaponArc {
    pub laser_cannons: u32,
    pub ion_cannons: u32,
    pub turbolasers: u32,
    pub ready: bool,
    pub energy: f32,
    pub energy_capacity: f32,
}

impl TacticalWeaponArc {
    fn new(laser_cannons: u32, ion_cannons: u32, turbolasers: u32) -> Self {
        let energy_capacity = (laser_cannons + ion_cannons + turbolasers) as f32;
        Self {
            laser_cannons,
            ion_cannons,
            turbolasers,
            ready: energy_capacity > 0.0,
            energy: energy_capacity,
            energy_capacity,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct TacticalWeaponRanges {
    pub laser_cannon: f32,
    pub ion_cannon: f32,
    pub turbolaser: f32,
}

/// Source fighter recovery state stored at object offset `+0x35c`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TacticalFighterRecoveryState {
    #[default]
    AwaitingCarrier,
    Reserved,
    Returning,
    Docking,
    Recovered,
}

impl TacticalFighterRecoveryState {
    /// Writers of `+0x35c`: `FUN_005cf940` 0, `FUN_005b84e0` 1, `FUN_005b1e10`
    /// 2, `FUN_005cf980` 3, `FUN_005b8630` 4.
    #[must_use]
    pub const fn source_code(self) -> u8 {
        match self {
            Self::AwaitingCarrier => 0,
            Self::Reserved => 1,
            Self::Returning => 2,
            Self::Docking => 3,
            Self::Recovered => 4,
        }
    }
}

/// The five selected-capital condition values consumed by the original HUD.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TacticalSubsystemCondition {
    pub shields: u8,
    pub weapons: u8,
    pub tractor: u8,
    pub engines: u8,
    pub hyperdrive: u8,
}

/// Installed subsystem components passed to the original tactical constructor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TacticalSubsystemCapacity {
    pub shields: bool,
    pub weapons: bool,
    pub tractor: bool,
    pub engines: bool,
    pub hyperdrive: u8,
}

impl TacticalSubsystemCapacity {
    /// `FUN_005b1970` caps the engine, shield, tractor and weapon counters at
    /// 4 and the hyperdrive counter at the installed component count.
    #[must_use]
    pub const fn hit_limits(self) -> [u8; 5] {
        [
            if self.shields { 4 } else { 0 },
            if self.weapons { 4 } else { 0 },
            if self.tractor { 4 } else { 0 },
            if self.engines { 4 } else { 0 },
            self.hyperdrive,
        ]
    }
}

/// Per-subsystem hit counters used by the original combat object.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TacticalSubsystemDamage {
    pub shields: u8,
    pub weapons: u8,
    pub tractor: u8,
    pub engines: u8,
    pub hyperdrive: u8,
}

impl TacticalSubsystemDamage {
    #[must_use]
    pub const fn hits(self) -> [u8; 5] {
        [
            self.shields,
            self.weapons,
            self.tractor,
            self.engines,
            self.hyperdrive,
        ]
    }

    #[must_use]
    pub const fn total_hits(self) -> u16 {
        self.shields as u16
            + self.weapons as u16
            + self.tractor as u16
            + self.engines as u16
            + self.hyperdrive as u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TacticalSubsystemKind {
    Shields,
    Weapons,
    Tractor,
    Engines,
    Hyperdrive,
}

impl TacticalSubsystemKind {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Shields => "shields",
            Self::Weapons => "weapons",
            Self::Tractor => "tractor",
            Self::Engines => "engines",
            Self::Hyperdrive => "hyperdrive",
        }
    }
}

/// One source-selected subsystem repair applied during live tactical combat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TacticalSubsystemRepair {
    pub ship: usize,
    pub kind: TacticalSubsystemKind,
    pub hits_before: u8,
    pub hits_after: u8,
}

/// Effective sublight state after engine damage and active tractor sources.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TacticalSubsystemMobility {
    pub base_engine_power: f32,
    pub engine_mode_bonus: f32,
    pub active_tractor_power: f32,
    pub effective_engine_power: f32,
    pub engine_percent: u8,
}

impl TacticalSubsystemCondition {
    #[must_use]
    pub const fn percentages(self) -> [u8; 5] {
        [
            self.shields,
            self.weapons,
            self.tractor,
            self.engines,
            self.hyperdrive,
        ]
    }

    #[must_use]
    pub fn resource_ids(self) -> [u32; 5] {
        let percentages = self.percentages();
        std::array::from_fn(|index| {
            TACTICAL_SUBSYSTEM_RESOURCE_BASES[index]
                + u32::from(original_tactical_condition_band(percentages[index]))
        })
    }

    fn from_source_state(
        hull_current: i32,
        hull_max: i32,
        capacity: TacticalSubsystemCapacity,
        damage: TacticalSubsystemDamage,
    ) -> Self {
        fn hull_bound_percent(available: bool, hull_current: i32, hull_max: i32, hits: u8) -> u8 {
            if !available || hull_max <= 0 {
                return 0;
            }
            let hull_percent = hull_current.max(0) as f32 * 100.0 / hull_max as f32;
            (hull_percent - f32::from(hits) * 25.0).clamp(0.0, 100.0) as u8
        }

        let hyperdrive = if capacity.hyperdrive == 0 {
            0
        } else {
            let remaining = capacity.hyperdrive.saturating_sub(damage.hyperdrive);
            (f32::from(remaining) * 100.0 / f32::from(capacity.hyperdrive)) as u8
        };
        Self {
            shields: hull_bound_percent(capacity.shields, hull_current, hull_max, damage.shields),
            weapons: hull_bound_percent(capacity.weapons, hull_current, hull_max, damage.weapons),
            tractor: hull_bound_percent(capacity.tractor, hull_current, hull_max, damage.tractor),
            // FUN_005b17f0 does not apply the hull ratio to sublight power.
            // Active tractor sources and maneuver mode are applied by the
            // session-wide refresh after this local baseline is formed.
            engines: if capacity.engines {
                100_u8.saturating_sub(damage.engines.saturating_mul(25))
            } else {
                0
            },
            hyperdrive,
        }
    }
}

impl TacticalShip {
    pub fn refresh_subsystem_condition(&mut self) {
        self.subsystem_condition = TacticalSubsystemCondition::from_source_state(
            self.hull_current,
            self.hull_max,
            self.subsystem_capacity,
            self.subsystem_damage,
        );
    }

    /// `FUN_005b1970`: one hit on a category's counter, capped as in
    /// [`TacticalSubsystemCapacity::hit_limits`]. Returns whether a tractor hit
    /// left no tractor power, which cancels the source's tractor command.
    fn add_subsystem_hit(&mut self, kind: TacticalSubsystemKind) -> bool {
        self.outstanding_hits = self.outstanding_hits.saturating_add(1);
        match kind {
            TacticalSubsystemKind::Shields => {
                self.subsystem_damage.shields =
                    self.subsystem_damage.shields.saturating_add(1).min(4);
            }
            TacticalSubsystemKind::Weapons => {
                self.subsystem_damage.weapons =
                    self.subsystem_damage.weapons.saturating_add(1).min(4);
                if self.subsystem_damage.weapons == 4 {
                    self.disable_weapon_arcs();
                }
            }
            TacticalSubsystemKind::Tractor => {
                self.subsystem_damage.tractor =
                    self.subsystem_damage.tractor.saturating_add(1).min(4);
            }
            TacticalSubsystemKind::Engines => {
                self.subsystem_damage.engines =
                    self.subsystem_damage.engines.saturating_add(1).min(4);
            }
            TacticalSubsystemKind::Hyperdrive => {
                self.subsystem_damage.hyperdrive = self
                    .subsystem_damage
                    .hyperdrive
                    .saturating_add(1)
                    .min(self.subsystem_capacity.hyperdrive);
            }
        }
        self.refresh_subsystem_condition();
        kind == TacticalSubsystemKind::Tractor && self.subsystem_condition.tractor == 0
    }

    /// hyp: a fourth weapon hit empties and queues every armed arc. No
    /// recovered routine does this on a weapon hit; the call sites of the
    /// executable's arc-clear routine settle it.
    fn disable_weapon_arcs(&mut self) {
        for (index, arc) in self.weapon_arcs.iter_mut().enumerate() {
            if arc.energy_capacity <= 0.0 {
                continue;
            }
            arc.ready = false;
            arc.energy = 0.0;
            let index = u8::try_from(index).unwrap_or(3);
            if !self.weapon_recharge_queue.contains(&index) {
                self.weapon_recharge_queue.push(index);
            }
        }
    }

    fn hull_ratio(&self) -> f32 {
        if self.hull_max <= 0 {
            0.0
        } else {
            self.hull_current.max(0) as f32 / self.hull_max as f32
        }
    }

    /// `FUN_005b0400`: hull-scaled rate less a quarter of the base rate per
    /// weapon hit, floored at zero.
    fn effective_weapon_recharge(&self) -> f32 {
        (self.hull_ratio() * self.weapon_recharge_rate
            - self.weapon_recharge_rate * 0.25 * f32::from(self.subsystem_damage.weapons))
        .max(0.0)
    }

    /// `FUN_005b0460`: the same rule with the shield rate and shield hits.
    fn effective_shield_recharge(&self) -> f32 {
        (self.hull_ratio() * self.shield_recharge_rate
            - self.shield_recharge_rate * 0.25 * f32::from(self.subsystem_damage.shields))
        .max(0.0)
    }

    fn advance_original_recharge(&mut self, seconds: f32) {
        if !self.alive || seconds <= 0.0 {
            return;
        }

        if self.shield < self.shield_max {
            self.shield_recharge_carry += self.effective_shield_recharge() * seconds;
            let whole = self.shield_recharge_carry.floor() as i32;
            if whole > 0 {
                let before = self.shield;
                self.shield = (self.shield + whole).min(self.shield_max);
                self.shield_recharge_carry -= (self.shield - before) as f32;
                if self.shield == self.shield_max {
                    self.shield_recharge_carry = 0.0;
                }
            }
        } else {
            self.shield_recharge_carry = 0.0;
        }

        let mut recharge = self.effective_weapon_recharge() * seconds;
        for &arc_index in &self.weapon_recharge_queue {
            if recharge <= 0.0 {
                break;
            }
            let Some(arc) = self.weapon_arcs.get_mut(usize::from(arc_index)) else {
                continue;
            };
            let missing = (arc.energy_capacity - arc.energy).max(0.0);
            let restored = recharge.min(missing);
            arc.energy += restored;
            recharge -= restored;
            if arc.energy >= arc.energy_capacity {
                arc.energy = arc.energy_capacity;
                arc.ready = arc.energy_capacity > 0.0;
            }
        }
        self.weapon_recharge_queue.retain(|&arc_index| {
            self.weapon_arcs
                .get(usize::from(arc_index))
                .is_some_and(|arc| !arc.ready)
        });
    }

    fn remove_subsystem_hit(&mut self, kind: TacticalSubsystemKind) -> Option<(u8, u8)> {
        let hits = match kind {
            TacticalSubsystemKind::Shields => &mut self.subsystem_damage.shields,
            TacticalSubsystemKind::Weapons => &mut self.subsystem_damage.weapons,
            TacticalSubsystemKind::Tractor => &mut self.subsystem_damage.tractor,
            TacticalSubsystemKind::Engines => &mut self.subsystem_damage.engines,
            TacticalSubsystemKind::Hyperdrive => &mut self.subsystem_damage.hyperdrive,
        };
        if *hits == 0 {
            return None;
        }
        let before = *hits;
        *hits -= 1;
        let after = *hits;
        self.refresh_subsystem_condition();
        Some((before, after))
    }
}

/// `FUN_005e77c0` maps a condition percentage to one of five resource offsets.
fn original_tactical_condition_band(percent: u8) -> u8 {
    match percent {
        0 => 0,
        1..=24 => 1,
        25..=49 => 2,
        50..=74 => 3,
        75..=u8::MAX => 4,
    }
}

/// One fighter squadron in the battle.
#[derive(Debug, Clone)]
pub struct TacticalFighter {
    pub class_key: FighterKey,
    /// Stable DAT and roster identity for production-to-render joins.
    pub identity: TacticalObjectIdentity,
    /// Exact original side-dependent tactical graphic identity.
    pub tactical_resource: Option<TacticalFighterResource>,
    /// Original retained-mode position at battle initialization.
    pub source_position: TacticalWorldPosition,
    /// Normalized maximum sublight power stored at source offset `+0x3f8`.
    pub sublight_engine_power: f32,
    /// The strategic fleet carrying this squadron; `None` for a squadron
    /// the system holds.
    pub fleet: Option<FleetKey>,
    /// Index into the originating fleet's aggregate fighter roster. Classes
    /// may repeat in separate slots.
    pub fleet_fighter_index: usize,
    /// Zero-based squadron within the aggregate strategic roster entry.
    pub fleet_squadron_index: u32,
    pub name: String,
    pub x: f32,
    pub y: f32,
    /// Live craft count derived from the source floating hull pool.
    pub squad_count: u32,
    /// Source fighter hull pool. Every craft contributes exactly eight points.
    pub hull_current: f32,
    pub hull_max: f32,
    /// Source fighter shield pool and its constructor maximum.
    pub shield: f32,
    pub shield_max: f32,
    /// Fore-only battery record used by all eight original FIGHTSD classes.
    pub weapon_arc: TacticalWeaponArc,
    /// Laser, ion, and turbolaser ranges promoted from FIGHTSD.DAT.
    pub weapon_ranges: TacticalWeaponRanges,
    /// Source `+0x134`: the class's torpedoes times the squadron size
    /// (`FUN_005030c0`, stored by `FUN_005b9c60.c:74`), and the FIGHTSD
    /// torpedo range. Torpedoes are not consumed.
    pub torpedo_strength: u32,
    pub torpedo_range: f32,
    /// Raw FIGHTSD maneuverability consumed by the fighter damage ratio.
    pub maneuverability: u32,
    pub is_attacker: bool,
    pub alive: bool,
    /// Zero-based red, blue, green, or gold fighter group; 4 is unassigned.
    pub fighter_group: u8,
    /// Fighter groups are selected as a unit; individual fighters are not.
    pub selected: bool,
    /// Source `+0x108`: off the field, flying a trench run (`FUN_005d08d0`
    /// de-lists it, `FUN_005a93b0`); it neither fires nor is a target until
    /// state 6 lists the survivors again.
    pub off_field: bool,
    /// Its carrier (`+0x164`): the hangar it docked in last, kept in
    /// flight (`FUN_005b8570`, `FUN_005b84a0` write it).
    pub carrier: Option<TacticalBerth>,
    /// In its carrier's hangar (states 1 and 5); a docked squadron neither
    /// fires nor is a target (`FUN_005ad870.c:13-21` accepts states 2, 4
    /// and 6).
    pub docked: Option<TacticalDock>,
    /// Unified source order code assigned through the command panel.
    pub order: TacticalOrder,
    /// Source Surround or Stand Off behavior assigned with a maneuver.
    pub tactic: TacticalTactic,
    /// Active target selected by a source attack-order executor.
    pub attack_target: Option<TacticalAttackTarget>,
    /// Friendly capital-ship index assigned by the direct Escort command.
    pub escort_target: Option<usize>,
    /// Current `FUN_005cf980` recovery executor state.
    pub recovery_state: TacticalFighterRecoveryState,
    /// Capital-ship index reserved as this squadron's recovery carrier.
    pub recovery_target: Option<usize>,
    /// A squadron the battle system holds for its side, not carried by a
    /// fleet (`FUN_0040b0e0` exports it after the fleets' units). Its
    /// `fleet_fighter_index` indexes the side's entries in `System::fighters`.
    pub held_by_system: bool,
}

impl TacticalFighter {
    /// In its side's battle lists: neither flying a trench run (`+0x108`)
    /// nor in a hangar (states 1 and 5, `FUN_005ad870.c:13-21`).
    #[must_use]
    pub fn listed(&self) -> bool {
        !self.off_field && self.docked.is_none()
    }

    fn effective_maneuverability(&self) -> f32 {
        (self.maneuverability as f32 + 1.0).clamp(1.0, ORIGINAL_CAPITAL_MANEUVERABILITY_MAX)
    }

    /// `FUN_005b23a0`: the torpedoes (`+0x134`) times `_DAT_0066c4cc`,
    /// scaled by the remaining hull (`FUN_005b1770`: `hp * x / hp_max`).
    fn torpedo_term(&self) -> f32 {
        if self.hull_max <= 0.0 {
            return 0.0;
        }
        self.hull_current.max(0.0) * (self.torpedo_strength as f32 * ORIGINAL_FIGHTER_TORPEDO_FACTOR)
            / self.hull_max
    }

    /// `FUN_005ba270` (vtable `0x66c028` slot `+0xbc`): the maneuver the
    /// trench run tests, 1.0 for a squadron with no sublight speed
    /// (slot `+0x90`, `FUN_005ba190`), else `min(+0x3fc + 1, 9)`.
    fn trench_maneuver(&self) -> f32 {
        if self.sublight_engine_power == 0.0 {
            1.0
        } else {
            (self.maneuverability as f32 + 1.0).min(ORIGINAL_CAPITAL_MANEUVERABILITY_MAX)
        }
    }

    fn weapon_hull_factor(&self) -> f32 {
        self.hull_current.max(0.0) * ORIGINAL_FIGHTER_WEAPON_HULL_SCALE
            / ORIGINAL_FIGHTER_HULL_PER_CRAFT
    }

    fn shield_recharge_rate(&self) -> f32 {
        self.hull_current.max(0.0) * ORIGINAL_FIGHTER_SHIELD_HULL_SCALE
            / ORIGINAL_FIGHTER_HULL_PER_CRAFT
    }

    /// `FUN_005b2d20.c:20-43`, `FUN_005af7a0`: a squadron destroyed outright
    /// (state `0xb`, no hull), in its hangar or out of it.
    fn destroy(&mut self) {
        self.hull_current = 0.0;
        self.squad_count = 0;
        self.alive = false;
        self.shield = 0.0;
        self.selected = false;
        self.attack_target = None;
    }

    /// `FUN_005b8e00` (`0x5b8e29`-`0x5b8e56`): the craft count is
    /// `max(1, ftol(strength / 8.0))`, 0 only once the squadron is gone.
    fn refresh_craft_count(&mut self) {
        self.hull_current = self.hull_current.clamp(0.0, self.hull_max.max(0.0));
        self.squad_count = if self.hull_current <= 0.0 {
            0
        } else {
            ((self.hull_current / ORIGINAL_FIGHTER_HULL_PER_CRAFT) as u32).max(1)
        };
        if self.squad_count == 0 {
            self.alive = false;
            self.shield = 0.0;
            self.weapon_arc.ready = false;
            self.weapon_arc.energy = 0.0;
        }
    }

    fn advance_original_recharge(&mut self, seconds: f32) {
        if !self.alive || seconds <= 0.0 {
            return;
        }
        if self.shield < self.shield_max {
            self.shield =
                (self.shield + self.shield_recharge_rate() * seconds).min(self.shield_max);
        }

        if !self.weapon_arc.ready && self.weapon_arc.energy_capacity > 0.0 {
            self.weapon_arc.energy = (self.weapon_arc.energy + self.weapon_hull_factor() * seconds)
                .min(self.weapon_arc.energy_capacity);
            if self.weapon_arc.energy >= self.weapon_arc.energy_capacity {
                self.weapon_arc.energy = self.weapon_arc.energy_capacity;
                self.weapon_arc.ready = true;
            }
        }
    }

    fn consume_weapon_arc(&mut self) {
        self.weapon_arc.energy = 0.0;
        self.weapon_arc.ready = false;
    }
}

/// A trench squadron's chance in percent (`FUN_005d04e0`, `0x5d0683`-
/// `0x5d06e3`): `trunc((torpedo term + min(rating, 9)) * 0.5)`, the torpedo
/// term `FUN_005b23a0`'s. `FUN_005b1770` and the sum run at double precision
/// and the sum is stored as a float (`FSTP` at `0x5d06c0`).
fn trench_run_success_chance(fighter: &TacticalFighter, rating: f32) -> u32 {
    let torpedoes = fighter.torpedo_strength as f32 * ORIGINAL_FIGHTER_TORPEDO_FACTOR;
    let term = if fighter.hull_max > 0.0 {
        f64::from(fighter.hull_current) * f64::from(torpedoes) / f64::from(fighter.hull_max)
    } else {
        0.0
    };
    let sum = (term + f64::from(rating.min(ORIGINAL_TRENCH_RUN_RATING_MAX))) as f32;
    (sum * 0.5).trunc() as u32
}

/// The Alliance Commander a trench run is rated by (`FUN_005ad7e0`): the
/// effective combat (`FUN_004edc40`, short `+0x86`) and the fleet that
/// carries the character; `None` for one the system holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TacticalTrenchCommander {
    pub combat: u32,
    pub fleet: Option<FleetKey>,
}

impl TacticalTrenchCommander {
    /// `FUN_005ad7e0`, `FUN_005afc00`: the slot `+0x50` plus one, within
    /// 1..=9. `+0x50` is the combat over the integer 20 as a float
    /// (`FUN_0040d680`: `FILD`, `FIDIV dword [DAT_006a80e8]`), so 70 rates
    /// 4.5.
    #[expect(clippy::cast_precision_loss, reason = "a skill well under 2^24")]
    fn rating(self) -> f32 {
        (self.combat as f32 / 20.0 + 1.0).clamp(1.0, ORIGINAL_TRENCH_RUN_RATING_MAX)
    }

    /// `FUN_0040b0e0.c:86-118`: the Alliance Commander field is
    /// `FUN_00509530`'s, the system's rank-1 holder and then each fleet's,
    /// the highest combat kept (ties keep the earlier). `side_fleets` is the
    /// battle's fleets, the attacker's first; `attacker_is_alliance` names
    /// the Alliance's row.
    #[must_use]
    pub fn of_battle(
        world: &GameWorld,
        system: SystemKey,
        side_fleets: &[Vec<FleetKey>; 2],
        attacker_is_alliance: bool,
    ) -> Option<Self> {
        use crate::officer_rank::{CommandKind, CommandPost, rank_holder};
        let fleets = &side_fleets[usize::from(!attacker_is_alliance)];
        std::iter::once((CommandPost::System(system), None))
            .chain(
                fleets
                    .iter()
                    .map(|&fleet| (CommandPost::Fleet(fleet), Some(fleet))),
            )
            .filter_map(|(post, fleet)| {
                let holder = rank_holder(world, post, CommandKind::Commander, true)?;
                // Slot `+0x1f0`, `FUN_004edc40`: the effective combat.
                let combat = crate::missions::member_skill(
                    world,
                    crate::missions::MissionMember::Character(holder),
                    crate::world::Skill::Combat,
                )?;
                Some(Self { combat, fleet })
            })
            .fold(None, |best: Option<Self>, candidate| match best {
                Some(best) if best.combat >= candidate.combat => Some(best),
                _ => Some(candidate),
            })
    }
}

/// The Death Star Shield's family (`0x25`), whose presence at a system sets
/// the station's `+0x70` (`FUN_0040bcd0`).
const DEATH_STAR_SHIELD_FAMILY: u8 = 0x25;

/// Whether a Death Star Shield stands at `system` (`FUN_0040bcd0` counts
/// the system's family-`0x25` children through `FUN_0051c270`, either side).
fn death_star_shield_at(world: &GameWorld, system: SystemKey) -> bool {
    world.systems.get(system).is_some_and(|value| {
        value.defense_facilities.iter().any(|&key| {
            world
                .defense_facilities
                .get(key)
                .is_some_and(|facility| facility.class_dat_id.family() == DEATH_STAR_SHIELD_FAMILY)
        })
    })
}

/// The Death Star is a manager-owned tactical object rather than a member of
/// the 29-entry capital mesh table. `FUN_005ab0e0` and `FUN_005ab650` select
/// its 5010 or 5020 sprite independently from normal capital participants.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TacticalDeathStar {
    pub resource: TacticalDeathStarResource,
    pub is_attacker: bool,
    pub is_alliance: bool,
    pub source_position: TacticalWorldPosition,
    /// Source field `+0x64`: the class hull (`FUN_0040bcd0` passes slot
    /// `+0x248` to `FUN_00595ef0`).
    pub hull_max: f32,
    /// Source field `+0x68`: the remaining hull, the class hull less the
    /// ship's damage (`+0x248 - +0x274`, `FUN_0040bcd0`); zero makes
    /// `FUN_005ba420` non-operational.
    pub hull: f32,
    /// Source field `+0x6c`, seeded full (100.0) and refilled by
    /// `FUN_005ba7f0` after a shot.
    pub laser_charge: f32,
    /// Source field `+0x70`: an active Death Star Shield (family `0x25`)
    /// stands at the system (`FUN_0040bcd0` through `FUN_0051c270`). Only
    /// `FUN_005ba450` reads it.
    pub shielded: bool,
    /// Source field `+0x74`: the station is not completed (its object's
    /// `+0x50` bit 2 is clear, `FUN_0040bcd0`); selects resource 5020.
    pub not_completed: bool,
    /// Source field `+0x78`; set when the Death Star action is committed.
    pub action_committed: bool,
    /// Hangar capacity, source field `+0x2c`: the class's fighter capacity
    /// (`FUN_0040bcd0`: slot `+0x26c` less the squadrons aboard `+0x23c`
    /// plus the squadrons that are its children).
    pub hangar_capacity: u32,
    /// The strategic ship it was raised from, as its fleet and roster
    /// index; `None` for a station still in production.
    pub ship: Option<(FleetKey, usize)>,
}

/// One committed superlaser shot. The original keeps the Death Star action
/// committed until its retained beam presentation reaches the target.
/// `FUN_005ba520` builds it as an ordinary laser projectile,
/// `FUN_005c12d0(target, source, 0, 1.0, 1)`: weapon type 0, longitudinal
/// scale 1.0 and shape variant 1, which lives one second; `FUN_005d3de0`
/// colours it by the firing side.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TacticalDeathStarBeam {
    pub target: usize,
    pub source_position: TacticalWorldPosition,
    pub target_position: TacticalWorldPosition,
    pub elapsed: f32,
    pub duration: f32,
    /// Direct3DRM material selector: 0 red (Alliance), 1 green (Empire).
    pub color_selector: u8,
}

impl TacticalDeathStar {
    /// `FUN_005ba420`: completed, hull left and not committed.
    #[must_use]
    pub fn operational(self) -> bool {
        !self.not_completed && self.hull != 0.0 && !self.action_committed
    }

    /// `FUN_005ba450`: operational with no Death Star Shield at the system.
    #[must_use]
    pub fn attackable(self) -> bool {
        self.operational() && !self.shielded
    }

    #[must_use]
    pub fn resource_id(self) -> u32 {
        if self.not_completed {
            self.resource.resource_base_with_flag
        } else {
            self.resource.resource_base_without_flag
        }
    }

    /// The station `fleet_key`'s first live Death Star ship raises
    /// (`FUN_005003d0`; `FUN_0040bcd0` → `FUN_00595ef0`), for the side
    /// `is_attacker`.
    #[must_use]
    #[expect(
        clippy::cast_precision_loss,
        reason = "The source stores the hulls as floats."
    )]
    pub fn of_fleet(
        world: &GameWorld,
        system: SystemKey,
        fleet_key: FleetKey,
        is_attacker: bool,
        layout: OriginalTacticalLayout,
    ) -> Option<Self> {
        let fleet = world.fleets.get(fleet_key)?;
        let (index, ship, class) =
            fleet
                .capital_ships
                .iter()
                .enumerate()
                .find_map(|(index, ship)| {
                    // Mode 1 (`FUN_005003d0(fleet, 1)`, `FUN_0040b0e0.c:175`).
                    let class = world.capital_ship_classes.get(ship.class)?;
                    (ship.usable() && class.is_death_star()).then_some((index, ship, class))
                })?;
        let mut station = Self::seated(world, system, fleet.is_alliance, is_attacker, layout);
        station.hull_max = class.hull as f32;
        station.hull = ship.hull_current.max(0) as f32;
        station.hangar_capacity = class.fighter_capacity;
        station.ship = Some((fleet_key, index));
        Some(station)
    }

    /// A station of the side `is_alliance` at `system`, with no hull yet:
    /// charge full, the shield from the system's Death Star Shield
    /// (`FUN_0040bcd0`), completed, uncommitted and with no ship.
    #[must_use]
    pub fn seated(
        world: &GameWorld,
        system: SystemKey,
        is_alliance: bool,
        is_attacker: bool,
        layout: OriginalTacticalLayout,
    ) -> Self {
        Self {
            resource: DEATH_STAR_TACTICAL_RESOURCE,
            is_attacker,
            is_alliance,
            source_position: TacticalWorldPosition::ORIGIN,
            hull_max: 0.0,
            hull: 0.0,
            laser_charge: ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE,
            shielded: death_star_shield_at(world, system),
            not_completed: false,
            action_committed: false,
            hangar_capacity: 0,
            ship: None,
        }
        .placed(layout)
    }

    /// The station decal's frame (`FUN_005d4d10`, `0x5d5505`-`0x5d55a4`):
    /// `-2.0` times the battle extent on x (`_DAT_0066d248`), off the face
    /// opposite the planet's, whichever side owns it, with y and z each
    /// `rand(trunc(0.4 * extent)) - 0.2 * extent` (`_DAT_0066d244` = 0.2).
    /// port: the original draws them from a local generator (`0x5d52ab`)
    /// that seeds itself from the clock (`FUN_005f5750`), after the planet's
    /// two draws when Show Planet is on; the port draws them first from a
    /// generator seeded like the battle's, so the placement is the same for
    /// every call with the same extent.
    #[must_use]
    fn placed(mut self, layout: OriginalTacticalLayout) -> Self {
        let extent = layout.battle_extent;
        let mut rng = OriginalBattleRng::default();
        self.source_position = TacticalWorldPosition {
            x: ORIGINAL_STATION_DECAL_X_SCALE * extent,
            y: original_scene_surround_jitter(&mut rng, extent, ORIGINAL_STATION_DECAL_JITTER),
            z: original_scene_surround_jitter(&mut rng, extent, ORIGINAL_STATION_DECAL_JITTER),
        };
        self
    }

    /// Where the superlaser leaves the station: the decal frame's child
    /// `+0x4c` at `(0, 0.11 * extent, -0.09 * extent)` from it
    /// (`FUN_005d4d10`, `_DAT_0066d258`, `_DAT_0066d250`), on which
    /// `FUN_005d59e0` builds the shot.
    #[must_use]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "The source multiplies by double constants and stores floats."
    )]
    pub fn superlaser_source(self, extent: f32) -> TacticalWorldPosition {
        TacticalWorldPosition {
            x: self.source_position.x,
            y: self.source_position.y
                + (f64::from(extent) * ORIGINAL_STATION_SUPERLASER_RISE) as f32,
            z: self.source_position.z
                - (f64::from(extent) * ORIGINAL_STATION_SUPERLASER_SETBACK) as f32,
        }
    }

    fn advance_laser_charge(&mut self, seconds: f32) {
        if self.hull > 0.0 && !self.not_completed && seconds > 0.0 {
            self.laser_charge = (self.laser_charge
                + seconds * ORIGINAL_DEATH_STAR_LASER_CHARGE_PER_SECOND)
                .min(ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE);
        }
    }
}

/// One offset of the scene surround (`FUN_005d4d10`):
/// `rand(trunc(2 * extent * scale)) - half` with `half = extent * scale`
/// stored as a float. The draw is `FUN_005f5700` (`[0, n)`, nothing for
/// `n <= 1`) and `__ftol` truncates the unrounded product. The station uses
/// 0.2 (`_DAT_0066d244`), the planet 0.1 (`_DAT_0066d23c`).
#[must_use]
#[expect(
    clippy::cast_possible_truncation,
    reason = "The source truncates with __ftol and stores the offset as a float."
)]
pub fn original_scene_surround_jitter(rng: &mut OriginalBattleRng, extent: f32, scale: f32) -> f32 {
    let product = f64::from(extent) * f64::from(scale);
    let half = extent * scale;
    let roll = rng.between(0, (2.0 * product) as i32);
    (f64::from(roll) - f64::from(half)) as f32
}

// ---------------------------------------------------------------------------
// BattleSession
// ---------------------------------------------------------------------------

/// `FUN_005ae9c0`'s stalemate watch: each side's lowest strength so far
/// (`battle+0x264` / `+0x268`) and the seconds since it last fell
/// (`+0x26c` / `+0x270`). Index 0 is the attacker.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct TacticalStalemateWatch {
    pub lowest: [Option<f32>; 2],
    pub quiet_seconds: [f32; 2],
}

/// The computer side's AI controller state `FUN_005c8780` reads for the
/// Death Star: the countdown to its next decision pass (`param_1[0x10]`,
/// less `_DAT_0066cbd4` = 1.0 per update, reset to `DAT_0066cba4` = 6.0 at
/// 0) and whether the Alliance AI has ordered a trench run (`param_1[6]`).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct TacticalDeathStarAi {
    pub countdown: f32,
    pub ordered_run: bool,
}

/// `DAT_0066cba4`: the AI controller's decision pass comes every sixth
/// update.
const ORIGINAL_AI_DECISION_UPDATES: f32 = 6.0;

/// The hangar a docked squadron sits in, its carrier (`+0x164`, resolved by
/// `FUN_005a6360`): a capital ship's (`+0x418`) or the station's (`+0x1c`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TacticalBerth {
    Ship(usize),
    Station,
}

/// A squadron in its carrier's hangar (`FUN_005b82a0.c:14-53`): ready to
/// launch (state 1, list `+0x1c`) or recovered (state 5, list `+0x20`,
/// which never launches again).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TacticalDock {
    Ready,
    Recovered,
}

/// The hangars' launch clock (`FUN_005b8800`). `step` is `battle+0x87c`,
/// 3 from the battle's constructor (`FUN_005a7500.c:551`) and again at
/// battle start (`FUN_005bea30.c:38`, `battle+0x8ac`), and
/// raised once per 1000 ms step (`battle+0x8a8`, `FUN_005bea30.c:33,125`).
/// Each hangar's launch step (`+0x14`) is absolute; 0.0 is unarmed
/// (`_DAT_0066c690`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TacticalHangars {
    pub step: u32,
    pub elapsed_ms: f32,
    /// Launch step of each capital ship's hangar, by ship index.
    pub ships: Vec<f32>,
    pub station: f32,
}

/// `battle+0x8ac`: the windowed battle's first step number (`FUN_005be160`,
/// `MOV [ESI+0x8ac],3` at `0x5be233`).
const ORIGINAL_HANGAR_FIRST_STEP: u32 = 3;
/// `battle+0x8a8`: one battle step is 1000 ms (`FUN_005bea30.c:33`).
const ORIGINAL_BATTLE_STEP_MILLISECONDS: f32 = 1000.0;
/// `DAT_0066c68c` (hangar `+0x18`, `FUN_005b82a0.c:59`): the least steps
/// between launches; `FUN_005b8800.c:28-37` adds `rand(0,8)`.
const ORIGINAL_HANGAR_LAUNCH_STEPS: f32 = 5.0;
/// `FUN_005b8800.c:30-33`: the launch spread is `FUN_005a8a70(0, 8)`.
const ORIGINAL_HANGAR_LAUNCH_SPREAD: u32 = 8;

/// A capital ship's hyperdrive works (`FUN_005b1b70`).
/// How far a unit `elapsed` milliseconds into its jump has flown
/// (`FUN_005a6b40`): the time, held at 4000.0 ms and in seconds, squared,
/// times the rate of variant (`ordinal` + `object_id`) & 3.
fn jump_flight(elapsed: f32, ordinal: u8, object_id: u32) -> f32 {
    let (distance, by) =
        ORIGINAL_JUMP_FLIGHTS[(u32::from(ordinal).wrapping_add(object_id) & 3) as usize];
    let seconds = elapsed.min(ORIGINAL_JUMP_MILLISECONDS) * 0.001;
    let by_seconds = by * 0.001;
    seconds * seconds * distance / (by_seconds * by_seconds)
}

fn ship_hyperdrive_works(ship: &TacticalShip) -> bool {
    ship.subsystem_capacity.hyperdrive > 0 && ship.subsystem_condition.hyperdrive > 0
}

/// Each side's morale record for `FUN_005d19a0`: the withdrawal threshold
/// (`side+0x24`), the retreat permission (`side+0x80`) and the latch
/// `FUN_005d14d0` sets once the retreat has run (`side+0x84`). Index 0 is
/// the attacker. The default permits no retreat, as a side the session
/// builder has not configured.
/// port: the gravity-well ships are the capital ships whose class has a
/// projector (`gravity_well_projector`), as the tactical-unit flag `+0x150`
/// is untraced (hyp); the entries index `BattleSession::ships`. The
/// hyperdrive squadrons are those whose FIGHTSD class has a hyperdrive or
/// damaged-hyperdrive rating (the fighter factory `FUN_0040be90` passes
/// their count as the constructor's `+0x3ec`, which `FUN_005b1bc0` reads);
/// they index `BattleSession::fighters`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TacticalMorale {
    pub threshold: [f32; 2],
    pub retreat_permitted: [bool; 2],
    pub retreated: [bool; 2],
    pub gravity_wells: Vec<usize>,
    pub hyperdrive_squadrons: Vec<usize>,
    /// A ship or squadron of the side jumped out (state 9,
    /// `FUN_005af910`), by side.
    pub jumped: [bool; 2],
    /// Each side's withdrawal in a played battle (side controller `+0x84`
    /// and `+0x7c`), index 0 the attacker.
    pub withdrawal: [TacticalWithdrawal; 2],
    /// The squadrons in their jump (state 7) and the milliseconds it has
    /// run (`FUN_005a6b40`).
    pub squadron_jumps: Vec<(usize, f32)>,
}

/// A played battle's side withdrawal (`ghidra/notes/morale-retreat.md`,
/// "Windowed withdrawal"): the side controller's state `+0x84` and its
/// countdown `+0x7c` in milliseconds, which `apply::seat_morale` starts at
/// the retreat system's transit (`FUN_00599c90.c:57-76`) and
/// `FUN_005ad530` raises to its floor of 30 s or 150 s.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct TacticalWithdrawal {
    pub state: TacticalWithdrawalState,
    pub countdown_milliseconds: f32,
}

/// The side controller's `+0x84` states a withdrawal passes through.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TacticalWithdrawalState {
    /// 0: no withdrawal.
    #[default]
    Fighting,
    /// 2: counting down (`FUN_005ad530`).
    CountingDown,
    /// 3: waiting for the side's hyperdrive units to leave.
    Holding,
    /// 4: the units left behind have been sent back to the fight.
    Over,
}

/// port: task-force slots tracked by the engagement AI: capital task forces
/// 0–7, then fighter groups 0–4 (4 holds the unassigned squadrons). The
/// source keeps each side's task forces in a list.
pub const TACTICAL_ENGAGEMENT_GROUPS: usize = 13;

/// Each side's task-force targets as the tactic objects assign them
/// (`FUN_005c9200` → `FUN_005ec740` / `FUN_005ec6d0` → `FUN_005a8fc0`), and
/// the steering point the port last gave each leader, so a player's own
/// waypoint is never mistaken for one. Index 0 is the attacker.
#[derive(Debug, Clone, PartialEq)]
pub struct TacticalEngagementState {
    pub targets: [[Option<TacticalAttackTarget>; TACTICAL_ENGAGEMENT_GROUPS]; 2],
    pub steering_goals: [[Option<TacticalWorldPosition>; TACTICAL_ENGAGEMENT_GROUPS]; 2],
    /// Fractional damage each capital ship has taken in a battle run without
    /// its window and not yet applied (the port stores hull and shield as
    /// whole points; the source keeps floats).
    pub capital_damage_carry: Vec<f32>,
}

impl Default for TacticalEngagementState {
    /// port: one row per side, index 0 the attacker.
    fn default() -> Self {
        Self {
            targets: [[None; TACTICAL_ENGAGEMENT_GROUPS]; 2],
            steering_goals: [[None; TACTICAL_ENGAGEMENT_GROUPS]; 2],
            capital_damage_carry: Vec::new(),
        }
    }
}

/// All state for one tactical combat encounter.
#[derive(Debug, Clone)]
pub struct BattleSession {
    /// System where the battle takes place.
    pub system: SystemKey,
    pub system_name: String,
    /// Original `SYSTEMSD.picture_id`; selects the active tactical palette.
    pub system_picture_id: u8,
    /// Attacker fleet key (in `GameWorld`); `None` when the side fights with
    /// the squadrons the system holds for it alone.
    pub attacker_fleet: Option<FleetKey>,
    /// Defender fleet key (in `GameWorld`), as the attacker's.
    pub defender_fleet: Option<FleetKey>,
    /// Faction identity is independent of the attacking role.
    pub attacker_is_alliance: bool,
    /// True if the player controls the attacker side.
    pub player_is_attacker: bool,
    /// Current battle phase.
    pub phase: BattlePhase,
    /// Individual ship hulls on the battlefield.
    pub ships: Vec<TacticalShip>,
    /// Fighter squadrons on the battlefield.
    pub fighters: Vec<TacticalFighter>,
    /// Separate manager-owned Death Star state, when either fleet carries it.
    pub death_star: Option<TacticalDeathStar>,
    /// Active superlaser beam and delayed target resolution.
    pub death_star_beam: Option<TacticalDeathStarBeam>,
    /// Resolved Death Star trench-run result, if one has occurred.
    pub trench_run_outcome: Option<TacticalTrenchRunOutcome>,
    /// Remaining source milliseconds in the active state-5 trench run.
    pub trench_run_remaining_ms: Option<f32>,
    /// RGBY group that entered the active run and owns its ordered voice path.
    pub trench_run_group: Option<u8>,
    /// The Alliance Commander `FUN_005ad7e0` rates the run by (slot
    /// `+0x950`): of the rank-1 holders at the system and in the side's
    /// fleets there, the one with the highest combat (`FUN_00509530`).
    pub trench_run_commander: Option<TacticalTrenchCommander>,
    /// Result selected by `FUN_005d04e0` when state 5 begins. State 6 or 7
    /// applies it only after the two-minute presentation timer completes.
    pub trench_run_pending_outcome: Option<TacticalTrenchRunOutcome>,
    /// Stable fighter indices retained by the source result manager.
    pub trench_run_participants: Vec<usize>,
    /// Each participant's hull at state 5 (`FUN_005d08d0`), in
    /// `trench_run_participants` order.
    pub trench_run_snapshot: Vec<f32>,
    /// Nine threshold slots selected by `FUN_005d04e0`; `0x13d` is silent.
    pub trench_run_chatter: [u16; 9],
    /// Next threshold slot consumed by `FUN_005d03f0`.
    pub trench_run_chatter_cursor: usize,
    /// Presentation edge consumed once by the app-level cutscene router.
    pub trench_run_cinematic_pending: bool,
    /// Four independently visible navigation-point sets. Their spread follows
    /// the manual's far-to-clustered ordering and scales with the recovered
    /// battle envelope.
    pub navigation_sets: [Vec<TacticalWorldPosition>; 4],
    /// Immutable source battle envelope used to place the initial participants.
    pub source_layout: OriginalTacticalLayout,
    /// Index of currently selected ship (in `ships`), if any.
    pub selected_ship: Option<usize>,
    /// Selected player fighter group, if the HUD is displaying a squadron.
    pub selected_fighter_group: Option<u8>,
    /// Whether the placement phase is confirmed (player clicked "Begin Battle").
    pub placement_confirmed: bool,
    /// Game tick when the battle started.
    pub start_tick: u64,
    /// Combat sub-tick counter (increments each combat step).
    pub combat_tick: u32,
    /// One source-compatible tactical random stream. The campaign contributes
    /// its seed at battle entry; every combat consumer advances it in order.
    pub tactical_rng: OriginalTacticalRng,
    /// The headless battle's generator (`FUN_005af420`), taken from the
    /// world and handed back to it ([`GameWorld::battle_rng`]).
    pub battle_rng: OriginalBattleRng,
    /// The import's crew pass, drawn once when the battle ends
    /// ([`Self::settle_crew`]); the Battle Summary and the write-back
    /// (`apply::apply_results`) both read it.
    pub settled_crew: Option<crate::battle_summary::SettledCrew>,
    /// Active weapon fire visual effects (source ship idx -> target ship idx).
    pub weapon_effects: Vec<WeaponEffect>,
    /// Target-attached original type-303 hit, damage, and destruction sequences.
    pub impact_effects: Vec<TacticalImpactEffect>,
    /// Source audio events waiting for the application audio backend. The
    /// resource identity is fixed when the combat event is emitted so frame
    /// scheduling cannot change variant selection.
    pub pending_audio_cues: Vec<TacticalAudioCue>,
    /// Source-identified VOICEFXA/VOICEFXE tactical lines waiting for the
    /// application audio backend.
    pub pending_voice_cues: Vec<TacticalVoiceCue>,
    /// Diagnostic lines (`[tactical_death_star]`, `[tactical_stalemate]`,
    /// `[simulate]`) for the front end to log; the engine has no logger.
    pub pending_log_lines: Vec<String>,
    /// Target-attached tractor/gravity field selected through one shared slot.
    pub field_effects: Vec<TacticalFieldEffect>,
    /// Source-selected subsystem repairs emitted by the latest combat step.
    pub subsystem_repairs: Vec<TacticalSubsystemRepair>,
    /// Whether combat is paused.
    pub paused: bool,
    /// Combat speed multiplier (1 = normal, 2 = fast, 4 = faster).
    pub combat_speed: u32,
    /// Accumulator for sub-frame combat stepping.
    pub step_accumulator: f32,
    /// Winner once battle ends (None during combat).
    pub winner: Option<CombatWinner>,
    /// The 1200-second stalemate withdrawal (`FUN_005ae9c0`).
    pub stalemate: TacticalStalemateWatch,
    /// The computer side's Death Star actions (`FUN_005c8780`).
    pub death_star_ai: TacticalDeathStarAi,
    /// The carriers' launch clock (`FUN_005b8800`).
    pub hangars: TacticalHangars,
    /// The per-step morale withdrawal (`FUN_005d19a0`).
    pub morale: TacticalMorale,
    pub engagement: TacticalEngagementState,
    /// The side holding the battle's system (`true` Alliance), `None` when
    /// neutral: `system[9] >> 6 & 3` in the result text builders.
    pub holder_is_alliance: Option<bool>,
    /// The side that arrived last at the battle's system (`true` Alliance),
    /// `None` for none: the system's `+0x78` bits 6-7, which `FUN_00508660`
    /// writes through `FUN_0050a1b0` with each arriving fleet's or
    /// squadron's side (`GameWorld::arrival_holders`).
    pub arrival_holder_is_alliance: Option<bool>,
    /// Whether the holder's system was under blockade when the battle began
    /// (a hostile fleet and no fleet of the holder's): the result object's
    /// `+0x5c` names the holder then.
    pub blockaded_at_start: bool,
    /// Where the losing fleet withdrew to, once known (`+0x48`).
    pub withdrawal_destination: Option<String>,
}

/// A visual weapon fire effect between two ships.
#[derive(Debug, Clone)]
pub struct WeaponEffect {
    /// Index of the firing capital ship in `ships`, or `usize::MAX` for a
    /// fighter source. Rendering uses the captured source position.
    pub source: usize,
    /// Index of the target capital ship in `ships`, or `usize::MAX` for a
    /// fighter target. Rendering uses the captured target position.
    pub target: usize,
    /// Weapon type determines color and rendering.
    pub kind: WeaponKind,
    /// Source point is captured once, matching the retained projectile frame.
    pub source_position: TacticalWorldPosition,
    /// Existing 2D simulation source used only when the original camera path is unavailable.
    pub fallback_source: [f32; 2],
    /// Target point captured once for capital and fighter projectiles alike.
    pub target_position: TacticalWorldPosition,
    /// Existing 2D simulation target used only when the original camera path is unavailable.
    pub fallback_target: [f32; 2],
    /// Source constructor scale along the projectile's local Z axis.
    pub longitudinal_scale: f32,
    /// One of the three exact triangle families built by `FUN_005ee590`.
    pub shape_variant: u8,
    /// Direct3DRM material selector: 0 red, 1 green, 2 blue.
    pub color_selector: u8,
    /// Source lifecycle timer and exact one- or two-second duration.
    pub elapsed: f32,
    pub duration: f32,
}

impl WeaponEffect {
    #[must_use]
    pub fn progress(&self) -> f32 {
        if self.duration <= 0.0 {
            1.0
        } else {
            (self.elapsed / self.duration).clamp(0.0, 1.0)
        }
    }
}

/// One target's shared tractor/gravity field slot from `FUN_005d3ac0`.
#[derive(Debug, Clone, PartialEq)]
pub struct TacticalFieldEffect {
    pub target: usize,
    tractor_sources: Vec<usize>,
    gravity_sources: Vec<usize>,
    pub frame: u8,
    frame_elapsed: f32,
}

/// The source field selector uses bit 1 for tractor and bit 2 for gravity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OriginalTacticalFieldKind {
    Tractor = 1,
    Gravity = 2,
}

impl TacticalFieldEffect {
    fn visible_kind(&self) -> Option<OriginalTacticalFieldKind> {
        if !self.gravity_sources.is_empty() {
            Some(OriginalTacticalFieldKind::Gravity)
        } else if !self.tractor_sources.is_empty() {
            Some(OriginalTacticalFieldKind::Tractor)
        } else {
            None
        }
    }

    /// `FUN_005d39a0` preloads gravity at 3520 and tractor at 3620, eight frames
    /// each, advanced by `FUN_005d3cc0` (`tactical-projectile-field-path.md`).
    #[must_use]
    pub fn resource_id(&self) -> Option<u32> {
        self.visible_kind().map(|kind| match kind {
            OriginalTacticalFieldKind::Gravity => 3520 + u32::from(self.frame),
            OriginalTacticalFieldKind::Tractor => 3620 + u32::from(self.frame),
        })
    }

    #[must_use]
    pub fn source_counts(&self) -> (u8, u8) {
        (
            u8::try_from(self.tractor_sources.len()).unwrap_or(u8::MAX),
            u8::try_from(self.gravity_sources.len()).unwrap_or(u8::MAX),
        )
    }

    #[must_use]
    pub fn source_ids(&self) -> (&[usize], &[usize]) {
        (&self.tractor_sources, &self.gravity_sources)
    }
}

/// One target-attached type-303 impact sequence selected by the original
/// tactical effect dispatcher at `FUN_005d3e90`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TacticalImpactEffect {
    pub target: usize,
    pub sequence: OriginalTacticalEffectSequence,
    pub frame: u8,
    frame_elapsed: f32,
}

/// Original target sprite states. The discriminants preserve the priority and
/// mutual-exclusion bits tested by `FUN_005d3e90`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OriginalTacticalEffectSequence {
    IonHit = 0x04,
    StandardHit = 0x08,
    IonDamage = 0x10,
    StandardDamage = 0x20,
    TurbolaserDamage = 0x40,
    Destroyed = 0x80,
}

impl OriginalTacticalEffectSequence {
    /// Type-303 bases preloaded by `FUN_005d39a0`: 0xbf4, 0xc30, 0xc6c, 0xca8,
    /// 0xce4 and 0xd20 (`tactical-impact-effect-path.md`).
    #[must_use]
    pub const fn resource_base(self) -> u32 {
        match self {
            Self::IonHit => 3180,
            Self::StandardHit => 3060,
            Self::IonDamage => 3240,
            Self::StandardDamage => 3120,
            Self::TurbolaserDamage => 3300,
            Self::Destroyed => 3360,
        }
    }

    /// Frame counts passed to `FUN_005d38d0` by `FUN_005d39a0`.
    #[must_use]
    pub const fn frame_count(self) -> u8 {
        match self {
            Self::IonHit | Self::StandardHit => 6,
            Self::IonDamage | Self::Destroyed => 16,
            Self::StandardDamage | Self::TurbolaserDamage => 7,
        }
    }

    /// Draw sizes preloaded by `FUN_005d39a0` (`tactical-impact-effect-path.md`).
    #[must_use]
    pub const fn source_size(self) -> (u16, u16) {
        match self {
            Self::IonHit | Self::StandardHit | Self::StandardDamage | Self::TurbolaserDamage => {
                (32, 32)
            }
            Self::IonDamage => (64, 32),
            Self::Destroyed => (64, 64),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OriginalTacticalImpactStage {
    Hit = 0,
    Damage = 1,
    Destroyed = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TacticalVoiceGroup {
    Capital(u8),
    Fighter(u8),
}

/// Route `FUN_005a5cf0`'s destruction callback from the local player's
/// perspective. A local attacker names its own firing group in the target-
/// destroyed family; a local casualty names the group that was lost.
fn queue_original_destruction_voice(
    voice_cues: &mut Vec<TacticalVoiceCue>,
    faction: TacticalVoiceFaction,
    player_is_attacker: bool,
    source_is_attacker: bool,
    source_group: TacticalVoiceGroup,
    target_is_attacker: bool,
    target_group: TacticalVoiceGroup,
    stage: OriginalTacticalImpactStage,
) {
    if stage != OriginalTacticalImpactStage::Destroyed {
        return;
    }
    let cue = if source_is_attacker == player_is_attacker
        && target_is_attacker != player_is_attacker
    {
        match source_group {
            TacticalVoiceGroup::Capital(group) => {
                TacticalVoiceCue::target_destroyed_by_capital(faction, group)
            }
            TacticalVoiceGroup::Fighter(group) => {
                TacticalVoiceCue::target_destroyed_by_fighter(faction, group)
            }
        }
    } else if target_is_attacker == player_is_attacker && source_is_attacker != player_is_attacker {
        match target_group {
            TacticalVoiceGroup::Capital(group) => {
                TacticalVoiceCue::capital_destroyed(faction, group)
            }
            TacticalVoiceGroup::Fighter(group) => TacticalVoiceCue::fighter_losses(faction, group),
        }
    } else {
        return;
    };
    voice_cues.push(cue);
}

/// Type of weapon for visual rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponKind {
    Turbolaser,
    IonCannon,
    LaserCannon,
    FighterAttack,
}

/// The eight weapon audio events dispatched by `FUN_005a7500` through the
/// original tactical sound manager, and the two jump effects `FUN_005a6b40`
/// plays. `FUN_005bae60` assigns each event a contiguous TACTICAL.DLL WAVE
/// family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TacticalAudioEvent {
    LaserFire,
    TurbolaserFire,
    IonFire,
    TorpedoFire,
    LaserImpact,
    IonImpact,
    TurbolaserImpact,
    TorpedoImpact,
    SquadronJump,
    CapitalShipJump,
}

impl TacticalAudioEvent {
    /// Event indices 0x0d-0x14 in `FUN_005a7500`'s registration order; a
    /// squadron's jump is 0x1d and a capital ship's 0x1f (`FUN_005a6b40`).
    #[must_use]
    pub const fn event_id(self) -> u8 {
        match self {
            Self::SquadronJump => 0x1d,
            Self::CapitalShipJump => 0x1f,
            Self::LaserFire => 0x0d,
            Self::TurbolaserFire => 0x0e,
            Self::IonFire => 0x0f,
            Self::TorpedoFire => 0x10,
            Self::LaserImpact => 0x11,
            Self::IonImpact => 0x12,
            Self::TurbolaserImpact => 0x13,
            Self::TorpedoImpact => 0x14,
        }
    }

    /// `FUN_005bae60`'s WAVE table: event 0x0d starts at 0x32e9 (13033).
    #[must_use]
    pub const fn resource_base(self) -> u32 {
        match self {
            Self::LaserFire => 13_033,
            Self::TurbolaserFire => 13_036,
            Self::IonFire => 13_039,
            Self::TorpedoFire => 13_042,
            Self::LaserImpact => 13_045,
            Self::IonImpact => 13_048,
            Self::TurbolaserImpact => 13_051,
            Self::TorpedoImpact => 13_054,
            // Events 0x1d and 0x1f hold 0x3307 and 0x3309.
            Self::SquadronJump => 13_063,
            Self::CapitalShipJump => 13_065,
        }
    }

    /// `FUN_005bae60`'s variant counts: 3 for events 0x0d-0x13, 1 for 0x14,
    /// 0x1d and 0x1f.
    #[must_use]
    pub const fn variant_count(self) -> u8 {
        match self {
            Self::TorpedoImpact | Self::SquadronJump | Self::CapitalShipJump => 1,
            _ => 3,
        }
    }

    const fn fire_for(kind: WeaponKind) -> Self {
        match kind {
            WeaponKind::LaserCannon => Self::LaserFire,
            WeaponKind::Turbolaser => Self::TurbolaserFire,
            WeaponKind::IonCannon => Self::IonFire,
            WeaponKind::FighterAttack => Self::TorpedoFire,
        }
    }

    const fn impact_for(kind: WeaponKind) -> Self {
        match kind {
            WeaponKind::LaserCannon => Self::LaserImpact,
            WeaponKind::Turbolaser => Self::TurbolaserImpact,
            WeaponKind::IonCannon => Self::IonImpact,
            WeaponKind::FighterAttack => Self::TorpedoImpact,
        }
    }
}

/// One concrete source event and selected original WAVE resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TacticalAudioCue {
    pub event: TacticalAudioEvent,
    pub resource_id: u32,
}

/// Faction-specific voice bank selected by the tactical sound manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TacticalVoiceFaction {
    Alliance,
    Empire,
}

impl TacticalVoiceFaction {
    /// `FUN_005bae60`: the Alliance bank starts at event 0x20, the Imperial at 0x9a.
    #[must_use]
    pub const fn source_event_base(self) -> u16 {
        match self {
            Self::Alliance => 0x20,
            Self::Empire => 0x9a,
        }
    }

    /// `FUN_005bae60`: event 0x20 is WAVE 0x36b1 (14001), 0x9a is 0x3a99 (15001).
    #[must_use]
    pub const fn resource_base(self) -> u32 {
        match self {
            Self::Alliance => 14_001,
            Self::Empire => 15_001,
        }
    }
}

/// Source family that selected a faction tactical voice line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TacticalVoiceEvent {
    SourceMapped,
    BattleReady,
    TargetTaskForceRejected,
    CapitalManeuver,
    FighterManeuver,
    ManeuverComplete,
    CapitalAttack,
    FighterAttack,
    OrdersComplete,
    TargetDestroyed,
    CapitalDestroyed,
    FighterLosses,
    CapitalFormation,
    CapitalMission,
    FighterMission,
    FighterRecoveryComplete,
    FighterRecoveryCapacity,
    WithdrawalStarted,
    WithdrawalHyperdriveWarning,
    WithdrawalComplete,
    BattleWonWithdrawal,
    BattleWonDestroyed,
    BattleLost,
    DeathStarFiring,
    DeathStarFireWarning,
    DeathStarReady,
    DeathStarUnderAttack,
    DeathStarAttackBrokenOff,
    DeathStarDestroyed,
    TrenchRunStarted,
    TrenchRunCommitted,
    TrenchRunChatter,
    TrenchRunFailed,
    TrenchRunSucceeded,
}

/// One exact tactical voice event and its original WAVE resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TacticalVoiceCue {
    pub event: TacticalVoiceEvent,
    pub faction: TacticalVoiceFaction,
    pub source_event: u16,
    pub resource_id: u32,
}

impl TacticalVoiceCue {
    /// `FUN_005bae60`'s three contiguous banks: events 0x20, 0x9a and 0x11e start
    /// at WAVE 14001, 15001 and 15133 (0x3b1d); `FUN_005bad50` accepts events
    /// below 0x13d.
    const fn from_source_event(event: TacticalVoiceEvent, source_event: u16) -> Self {
        let (faction, event_base, resource_base) = match source_event {
            0x20..=0x99 => (TacticalVoiceFaction::Alliance, 0x20, 14_001),
            0x9a..=0x11d => (TacticalVoiceFaction::Empire, 0x9a, 15_001),
            0x11e..=0x13c => (TacticalVoiceFaction::Alliance, 0x11e, 15_133),
            _ => panic!("source event is outside the recovered tactical voice banks"),
        };
        Self {
            event,
            faction,
            source_event,
            resource_id: resource_base + (source_event - event_base) as u32,
        }
    }

    const fn from_offset(
        faction: TacticalVoiceFaction,
        event: TacticalVoiceEvent,
        offset: u16,
    ) -> Self {
        Self::from_source_event(event, faction.source_event_base() + offset)
    }

    #[must_use]
    pub const fn battle_ready(faction: TacticalVoiceFaction) -> Self {
        Self::from_offset(faction, TacticalVoiceEvent::BattleReady, 0)
    }

    /// Queue the faction-specific rejection used when an enemy target command
    /// cannot resolve the selected capitals to one task-force ordinal.
    /// `FUN_005a3020` and `FUN_005a24d0` queue events 0x84 and 0x102.
    #[must_use]
    pub const fn target_task_force_rejected(faction: TacticalVoiceFaction) -> Self {
        let source_event = match faction {
            TacticalVoiceFaction::Alliance => 0x84,
            TacticalVoiceFaction::Empire => 0x102,
        };
        Self::from_source_event(TacticalVoiceEvent::TargetTaskForceRejected, source_event)
    }

    /// `FUN_005a3020` and `FUN_005a3f00` queue the task-force ordinal + 0x22 or 0x9c.
    #[must_use]
    pub fn capital_maneuver(faction: TacticalVoiceFaction, group: u8) -> Self {
        Self::from_offset(
            faction,
            TacticalVoiceEvent::CapitalManeuver,
            2 + group.min(7) as u16,
        )
    }

    /// `FUN_005a4e50` queues the group ordinal + 0x2a or 0xa4.
    #[must_use]
    pub fn fighter_maneuver(faction: TacticalVoiceFaction, group: u8) -> Self {
        Self::from_offset(
            faction,
            TacticalVoiceEvent::FighterManeuver,
            10 + group.min(3) as u16,
        )
    }

    /// `FUN_005bae60` events 0x2e..0x3a and 0xa8..0xb4. The generic line is
    /// followed by task forces one through eight and RGBY fighter groups.
    #[must_use]
    pub fn capital_maneuver_complete(faction: TacticalVoiceFaction, group: u8) -> Self {
        Self::from_offset(
            faction,
            TacticalVoiceEvent::ManeuverComplete,
            15 + group.min(7) as u16,
        )
    }

    /// `FUN_005bae60` events 0x37..0x3a and 0xb1..0xb4, the RGBY lines after the
    /// task forces (`tactical-voice-map.json` maneuver_complete). Their caller is not recovered.
    #[must_use]
    pub fn fighter_maneuver_complete(faction: TacticalVoiceFaction, group: u8) -> Self {
        Self::from_offset(
            faction,
            TacticalVoiceEvent::ManeuverComplete,
            23 + group.min(3) as u16,
        )
    }

    /// `FUN_005a3aa0` queues the task-force ordinal + 0x3c or 0xb6.
    #[must_use]
    pub fn capital_attack(faction: TacticalVoiceFaction, group: u8) -> Self {
        Self::from_offset(
            faction,
            TacticalVoiceEvent::CapitalAttack,
            28 + group.min(7) as u16,
        )
    }

    /// `FUN_005a4bd0` queues the group ordinal + 0x44 or 0xbe.
    #[must_use]
    pub fn fighter_attack(faction: TacticalVoiceFaction, group: u8) -> Self {
        Self::from_offset(
            faction,
            TacticalVoiceEvent::FighterAttack,
            36 + group.min(3) as u16,
        )
    }

    /// `FUN_005bae60` events 0x48..0x54 and 0xc2..0xce.
    #[must_use]
    pub fn capital_orders_complete(faction: TacticalVoiceFaction, group: u8) -> Self {
        Self::from_offset(
            faction,
            TacticalVoiceEvent::OrdersComplete,
            41 + group.min(7) as u16,
        )
    }

    /// `FUN_005bae60` events 0x51..0x54 and 0xcb..0xce, the RGBY lines after the
    /// task forces (`tactical-voice-map.json` orders_complete). Their caller is not recovered.
    #[must_use]
    pub fn fighter_orders_complete(faction: TacticalVoiceFaction, group: u8) -> Self {
        Self::from_offset(
            faction,
            TacticalVoiceEvent::OrdersComplete,
            49 + group.min(3) as u16,
        )
    }

    /// `FUN_005a5cf0` selects these variants from the local firing group when
    /// its target is destroyed.
    #[must_use]
    pub fn target_destroyed_by_capital(faction: TacticalVoiceFaction, group: u8) -> Self {
        Self::from_offset(
            faction,
            TacticalVoiceEvent::TargetDestroyed,
            54 + group.min(7) as u16,
        )
    }

    /// `FUN_005a5cf0` queues the fighter group ordinal + 0x5e or 0xd8.
    #[must_use]
    pub fn target_destroyed_by_fighter(faction: TacticalVoiceFaction, group: u8) -> Self {
        Self::from_offset(
            faction,
            TacticalVoiceEvent::TargetDestroyed,
            62 + group.min(3) as u16,
        )
    }

    /// `FUN_005a5cf0` selects loss variants from the local destroyed group.
    #[must_use]
    pub fn capital_destroyed(faction: TacticalVoiceFaction, group: u8) -> Self {
        Self::from_offset(
            faction,
            TacticalVoiceEvent::CapitalDestroyed,
            67 + group.min(7) as u16,
        )
    }

    /// `FUN_005a5cf0` queues the group ordinal + 0x6b or 0xe5.
    #[must_use]
    pub fn fighter_losses(faction: TacticalVoiceFaction, group: u8) -> Self {
        Self::from_offset(
            faction,
            TacticalVoiceEvent::FighterLosses,
            75 + group.min(3) as u16,
        )
    }

    /// `FUN_005a3740` and `FUN_005a3850` queue the task-force ordinal + 0x6f or
    /// 0xed.
    #[must_use]
    pub fn capital_formation(faction: TacticalVoiceFaction, group: u8) -> Self {
        let base = match faction {
            TacticalVoiceFaction::Alliance => 79,
            TacticalVoiceFaction::Empire => 83,
        };
        Self::from_offset(
            faction,
            TacticalVoiceEvent::CapitalFormation,
            base + group.min(7) as u16,
        )
    }

    /// `FUN_005a3630` queues the task-force ordinal + 0x78 or 0xf6.
    #[must_use]
    pub fn capital_mission(faction: TacticalVoiceFaction, group: u8) -> Self {
        let base = match faction {
            TacticalVoiceFaction::Alliance => 88,
            TacticalVoiceFaction::Empire => 92,
        };
        Self::from_offset(
            faction,
            TacticalVoiceEvent::CapitalMission,
            base + group.min(7) as u16,
        )
    }

    /// `FUN_005a48d0` queues the group ordinal + 0x80 or 0xfe.
    #[must_use]
    pub fn fighter_mission(faction: TacticalVoiceFaction, group: u8) -> Self {
        let base = match faction {
            TacticalVoiceFaction::Alliance => 96,
            TacticalVoiceFaction::Empire => 100,
        };
        Self::from_offset(
            faction,
            TacticalVoiceEvent::FighterMission,
            base + group.min(3) as u16,
        )
    }

    /// Queue the source group acknowledgement emitted when the final fighters
    /// in a recovering group enter the carrier.
    /// `FUN_005b8630` and `FUN_005cf980` queue events 0x87..0x8a and 0x105..0x108.
    #[must_use]
    pub fn fighter_recovery_complete(faction: TacticalVoiceFaction, group: u8) -> Self {
        let source_event = match faction {
            TacticalVoiceFaction::Alliance => 0x87,
            TacticalVoiceFaction::Empire => 0x105,
        } + u16::from(group.min(3));
        Self::from_source_event(TacticalVoiceEvent::FighterRecoveryComplete, source_event)
    }

    /// Queue the source group warning used when no surviving friendly carrier
    /// has another fighter docking slot.
    /// `FUN_005cf980` queues events 0x8b and 0x109 plus the group ordinal.
    #[must_use]
    pub fn fighter_recovery_capacity(faction: TacticalVoiceFaction, group: u8) -> Self {
        let source_event = match faction {
            TacticalVoiceFaction::Alliance => 0x8b,
            TacticalVoiceFaction::Empire => 0x109,
        } + u16::from(group.min(3));
        Self::from_source_event(TacticalVoiceEvent::FighterRecoveryCapacity, source_event)
    }

    /// `FUN_005ad530` queues events 0x8f and 0x10d.
    #[must_use]
    pub const fn withdrawal_started(faction: TacticalVoiceFaction) -> Self {
        let source_event = match faction {
            TacticalVoiceFaction::Alliance => 0x8f,
            TacticalVoiceFaction::Empire => 0x10d,
        };
        Self::from_source_event(TacticalVoiceEvent::WithdrawalStarted, source_event)
    }

    /// Queue the original warning that ships without an operational
    /// hyperdrive will be left behind by a fleet withdrawal.
    /// `FUN_005a0240` queues events 0x95 and 0x10e.
    #[must_use]
    pub const fn withdrawal_hyperdrive_warning(faction: TacticalVoiceFaction) -> Self {
        let source_event = match faction {
            TacticalVoiceFaction::Alliance => 0x95,
            TacticalVoiceFaction::Empire => 0x10e,
        };
        Self::from_source_event(
            TacticalVoiceEvent::WithdrawalHyperdriveWarning,
            source_event,
        )
    }

    /// `FUN_005bae60` events 0x96 and 0x118 (`tactical-voice-map.json` withdrawal_complete). Their
    /// caller is not recovered.
    #[must_use]
    pub const fn withdrawal_complete(faction: TacticalVoiceFaction) -> Self {
        let source_event = match faction {
            TacticalVoiceFaction::Alliance => 0x96,
            TacticalVoiceFaction::Empire => 0x118,
        };
        Self::from_source_event(TacticalVoiceEvent::WithdrawalComplete, source_event)
    }

    /// `FUN_005bae60` events 0x97/0x98 and 0x119/0x11a (`tactical-voice-map.json`
    /// battle_won_opponent_withdrew, battle_won_opponent_destroyed). Their caller
    /// is not recovered.
    #[must_use]
    pub const fn battle_won(faction: TacticalVoiceFaction, opponent_withdrew: bool) -> Self {
        let source_event = match (faction, opponent_withdrew) {
            (TacticalVoiceFaction::Alliance, true) => 0x97,
            (TacticalVoiceFaction::Alliance, false) => 0x98,
            (TacticalVoiceFaction::Empire, true) => 0x119,
            (TacticalVoiceFaction::Empire, false) => 0x11a,
        };
        Self::from_source_event(
            if opponent_withdrew {
                TacticalVoiceEvent::BattleWonWithdrawal
            } else {
                TacticalVoiceEvent::BattleWonDestroyed
            },
            source_event,
        )
    }

    /// `FUN_005bae60` events 0x99 and 0x11b (`tactical-voice-map.json`
    /// battle_lost). Their caller is not recovered.
    #[must_use]
    pub const fn battle_lost(faction: TacticalVoiceFaction) -> Self {
        let source_event = match faction {
            TacticalVoiceFaction::Alliance => 0x99,
            TacticalVoiceFaction::Empire => 0x11b,
        };
        Self::from_source_event(TacticalVoiceEvent::BattleLost, source_event)
    }

    /// `FUN_005a5740` queues event 0x111.
    #[must_use]
    pub const fn death_star_firing() -> Self {
        Self::from_source_event(TacticalVoiceEvent::DeathStarFiring, 0x111)
    }

    /// `FUN_005ba520.c:25-31` queues Alliance event 0x91, the "fire" line of
    /// the Death Star warnings (WAVE 14114).
    #[must_use]
    pub const fn death_star_fire_warning() -> Self {
        Self::from_source_event(TacticalVoiceEvent::DeathStarFireWarning, 0x91)
    }

    /// `FUN_005ba7f0` queues event 0x113.
    #[must_use]
    pub const fn death_star_ready() -> Self {
        Self::from_source_event(TacticalVoiceEvent::DeathStarReady, 0x113)
    }

    /// `FUN_005d0340` queues event 0x115 for the Imperial viewer.
    #[must_use]
    pub const fn death_star_under_attack() -> Self {
        Self::from_source_event(TacticalVoiceEvent::DeathStarUnderAttack, 0x115)
    }

    /// `FUN_005bae60` event 0x116 (`tactical-voice-map.json` death_star_status: broken off). Its
    /// caller is not recovered.
    #[must_use]
    pub const fn death_star_attack_broken_off() -> Self {
        Self::from_source_event(TacticalVoiceEvent::DeathStarAttackBrokenOff, 0x116)
    }

    /// `FUN_005bae60` event 0x117 (`tactical-voice-map.json` death_star_status: destroyed). Its
    /// caller is not recovered.
    #[must_use]
    pub const fn death_star_destroyed() -> Self {
        Self::from_source_event(TacticalVoiceEvent::DeathStarDestroyed, 0x117)
    }

    /// `FUN_005d0340` maps the group from `FUN_005c7150` to 0x139, 0x12d, 0x131
    /// or 0x135.
    const fn trench_run_source_event(group: u8) -> u16 {
        match group {
            0 => 0x139,
            1 => 0x12d,
            2 => 0x131,
            _ => 0x135,
        }
    }

    #[must_use]
    pub const fn trench_run_started(group: u8) -> Self {
        Self::from_source_event(
            TacticalVoiceEvent::TrenchRunStarted,
            Self::trench_run_source_event(group),
        )
    }

    /// `FUN_005cfec0` queues the group-specific follow-up immediately after
    /// the tactical result manager enters the timed trench-run state.
    #[must_use]
    pub const fn trench_run_committed(group: u8) -> Self {
        Self::from_source_event(
            TacticalVoiceEvent::TrenchRunCommitted,
            Self::trench_run_source_event(group) + 1,
        )
    }

    #[must_use]
    pub const fn trench_run_chatter(source_event: u16) -> Self {
        Self::from_source_event(TacticalVoiceEvent::TrenchRunChatter, source_event)
    }

    /// `FUN_005cfec0` state 7 queues the group's event + 2 on failure. The success
    /// line + 3 is `FUN_005bae60`'s fourth per-group slot (`tactical-voice-map.json`); state 6
    /// computes the group but no queue call is recovered there.
    #[must_use]
    pub const fn trench_run_outcome(group: u8, outcome: TacticalTrenchRunOutcome) -> Self {
        let (event, offset) = match outcome {
            TacticalTrenchRunOutcome::Failure => (TacticalVoiceEvent::TrenchRunFailed, 2),
            TacticalTrenchRunOutcome::Success => (TacticalVoiceEvent::TrenchRunSucceeded, 3),
        };
        Self::from_source_event(event, Self::trench_run_source_event(group) + offset)
    }

    #[cfg(feature = "interface-test-fixtures")]
    pub const fn fixture_source_event(source_event: u16) -> Self {
        Self::from_source_event(TacticalVoiceEvent::SourceMapped, source_event)
    }
}

impl TacticalAudioCue {
    #[must_use]
    pub const fn from_variant(event: TacticalAudioEvent, variant: u8) -> Self {
        let bounded = variant % event.variant_count();
        Self {
            event,
            resource_id: event.resource_base() + bounded as u32,
        }
    }

    #[must_use]
    pub const fn from_seed(event: TacticalAudioEvent, seed: u32) -> Self {
        Self::from_variant(event, (seed % event.variant_count() as u32) as u8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OriginalTacticalWeaponArc {
    Fore = 0,
    Starboard = 1,
    Aft = 2,
    Port = 3,
}

impl OriginalTacticalWeaponArc {
    const fn index(self) -> usize {
        self as usize
    }
}

/// Reproduce the two rotated-forward tests in `FUN_005b6530`. Equality at the
/// forward boundaries belongs to the fore arc; equality at the aft boundaries
/// remains in the corresponding lateral arc.
fn original_tactical_weapon_arc(
    source: &TacticalShip,
    target: &TacticalShip,
) -> OriginalTacticalWeaponArc {
    original_tactical_weapon_arc_to_position(source, target.source_position)
}

fn original_tactical_weapon_arc_to_position(
    source: &TacticalShip,
    target: TacticalWorldPosition,
) -> OriginalTacticalWeaponArc {
    let delta = source_delta(target, source.source_position);
    let forward = source.source_forward;
    let forward_dot = delta.x.mul_add(forward.x, delta.z * forward.z);
    let starboard_dot = delta.x.mul_add(forward.z, -delta.z * forward.x);
    let lateral = starboard_dot.abs();
    if forward_dot >= lateral {
        OriginalTacticalWeaponArc::Fore
    } else if -forward_dot > lateral {
        OriginalTacticalWeaponArc::Aft
    } else if starboard_dot > 0.0 {
        OriginalTacticalWeaponArc::Starboard
    } else {
        OriginalTacticalWeaponArc::Port
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct OriginalTacticalWeaponCandidate {
    strength: f32,
    target: Option<usize>,
}

#[derive(Debug, Clone, Copy, Default)]
struct OriginalTacticalArcCandidates {
    laser: OriginalTacticalWeaponCandidate,
    ion: OriginalTacticalWeaponCandidate,
    turbolaser: OriginalTacticalWeaponCandidate,
}

impl OriginalTacticalArcCandidates {
    fn total(self) -> f32 {
        self.laser.strength + self.ion.strength + self.turbolaser.strength
    }
}

fn original_capital_weapon_strength(
    ship: &TacticalShip,
    count: u32,
    kind: WeaponKind,
    scale_for_hull: bool,
) -> f32 {
    let target_modifier = match kind {
        WeaponKind::LaserCannon => ORIGINAL_LASER_VS_CAPITAL,
        WeaponKind::IonCannon | WeaponKind::Turbolaser | WeaponKind::FighterAttack => 1.0,
    };
    let strength = count as f32 * ORIGINAL_CAPITAL_WEAPON_STRENGTH * target_modifier;
    if scale_for_hull {
        strength * ship.hull_ratio()
    } else {
        strength
    }
}

fn original_weapon_range(ship: &TacticalShip, kind: WeaponKind) -> f32 {
    match kind {
        WeaponKind::LaserCannon => ship.weapon_ranges.laser_cannon,
        WeaponKind::IonCannon => ship.weapon_ranges.ion_cannon,
        WeaponKind::Turbolaser => ship.weapon_ranges.turbolaser,
        WeaponKind::FighterAttack => 0.0,
    }
}

fn original_weapon_count(arc: TacticalWeaponArc, kind: WeaponKind) -> u32 {
    match kind {
        WeaponKind::LaserCannon => arc.laser_cannons,
        WeaponKind::IonCannon => arc.ion_cannons,
        WeaponKind::Turbolaser => arc.turbolasers,
        WeaponKind::FighterAttack => 0,
    }
}

impl WeaponKind {
    /// `SHIP_TAKE_*_HIT` codes delivered to the original render dispatcher,
    /// in the order `FUN_005a7500` registers them.
    const fn original_impact_code(self) -> u8 {
        match self {
            Self::LaserCannon | Self::FighterAttack => 1,
            Self::IonCannon => 2,
            Self::Turbolaser => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct OriginalProjectileProfile {
    longitudinal_scale: f32,
    shape_variant: u8,
    color_selector: u8,
    duration: f32,
}

/// The projectile constructor `FUN_005ee590` (reached through
/// `FUN_005d3de0`): longitudinal scale and mesh variant by weapon, threshold
/// and the special turbolaser ordinals 11, 13, 14, 22 and 25; red Alliance,
/// green Imperial and blue ion material; one second for variants one and
/// two, two seconds for variant three (`tactical-projectile-field-path.md`).
fn original_projectile_profile(
    weapon: WeaponKind,
    event_strength: f32,
    source_tactical_ordinal: Option<u8>,
    source_is_alliance: bool,
) -> OriginalProjectileProfile {
    let strength = if event_strength.is_finite() {
        event_strength.max(0.0)
    } else {
        0.0
    };
    let (longitudinal_scale, shape_variant) = match weapon {
        WeaponKind::LaserCannon if strength >= ORIGINAL_LASER_PROJECTILE_THRESHOLD => (0.5, 2),
        WeaponKind::LaserCannon => (0.5, 1),
        WeaponKind::Turbolaser if strength < ORIGINAL_TURBOLASER_PROJECTILE_THRESHOLD => (0.65, 1),
        WeaponKind::Turbolaser
            if matches!(source_tactical_ordinal, Some(11 | 13 | 14 | 22 | 25)) =>
        {
            (0.75, 2)
        }
        WeaponKind::Turbolaser => (1.0, 3),
        WeaponKind::IonCannon if strength >= ORIGINAL_ION_PROJECTILE_THRESHOLD => (1.0, 3),
        WeaponKind::IonCannon => (0.4, 1),
        WeaponKind::FighterAttack if strength >= ORIGINAL_TORPEDO_PROJECTILE_THRESHOLD => (1.0, 3),
        WeaponKind::FighterAttack => (0.2, 1),
    };
    let color_selector = if weapon == WeaponKind::IonCannon {
        2
    } else if source_is_alliance {
        0
    } else {
        1
    };
    OriginalProjectileProfile {
        longitudinal_scale,
        shape_variant,
        color_selector,
        duration: if shape_variant == 3 {
            ORIGINAL_PROJECTILE_DURATION_LONG
        } else {
            ORIGINAL_PROJECTILE_DURATION_SHORT
        },
    }
}

/// Reproduce the resource branch in `FUN_005d3e90`. Its first argument uses
/// the `SHIP_TAKE_*_HIT` ordering recovered from `FUN_005a7500`; its second is
/// the target effect stage returned immediately after hit resolution.
fn original_tactical_effect_sequence(
    weapon: WeaponKind,
    stage: OriginalTacticalImpactStage,
) -> OriginalTacticalEffectSequence {
    match stage {
        OriginalTacticalImpactStage::Hit if weapon.original_impact_code() == 2 => {
            OriginalTacticalEffectSequence::IonHit
        }
        OriginalTacticalImpactStage::Hit => OriginalTacticalEffectSequence::StandardHit,
        OriginalTacticalImpactStage::Damage if weapon.original_impact_code() == 2 => {
            OriginalTacticalEffectSequence::IonDamage
        }
        OriginalTacticalImpactStage::Damage if weapon.original_impact_code() == 3 => {
            OriginalTacticalEffectSequence::TurbolaserDamage
        }
        OriginalTacticalImpactStage::Damage => OriginalTacticalEffectSequence::StandardDamage,
        OriginalTacticalImpactStage::Destroyed => OriginalTacticalEffectSequence::Destroyed,
    }
}

/// `FUN_005d3e90` rejects an incoming state whose bit is not above the
/// active one, and keeps `0x04`/`0x08` and `0x10`/`0x20`/`0x40` mutually
/// exclusive.
pub fn queue_original_tactical_impact(
    effects: &mut Vec<TacticalImpactEffect>,
    target: usize,
    weapon: WeaponKind,
    stage: OriginalTacticalImpactStage,
) -> bool {
    let sequence = original_tactical_effect_sequence(weapon, stage);
    let incoming = sequence as u8;
    if let Some(current) = effects.iter_mut().find(|effect| effect.target == target) {
        let active = current.sequence as u8;
        if incoming <= active
            || (incoming & 0x70 != 0 && active & 0x70 != 0)
            || (incoming & 0x0c != 0 && active & 0x0c != 0)
        {
            return false;
        }
        *current = TacticalImpactEffect {
            target,
            sequence,
            frame: 0,
            frame_elapsed: 0.0,
        };
    } else {
        effects.push(TacticalImpactEffect {
            target,
            sequence,
            frame: 0,
            frame_elapsed: 0.0,
        });
    }
    true
}

pub fn queue_original_tactical_projectile(
    effects: &mut Vec<WeaponEffect>,
    ships: &[TacticalShip],
    source: usize,
    target: usize,
    weapon: WeaponKind,
    event_strength: f32,
) -> bool {
    let (Some(source_ship), Some(target_ship)) = (ships.get(source), ships.get(target)) else {
        return false;
    };
    let profile = original_projectile_profile(
        weapon,
        event_strength,
        source_ship
            .tactical_resource
            .map(|resource| resource.tactical_ordinal),
        source_ship.identity.is_alliance,
    );
    effects.push(WeaponEffect {
        source,
        target,
        kind: weapon,
        source_position: source_ship.source_position,
        fallback_source: [source_ship.x, source_ship.y],
        target_position: target_ship.source_position,
        fallback_target: [target_ship.x, target_ship.y],
        longitudinal_scale: profile.longitudinal_scale,
        shape_variant: profile.shape_variant,
        color_selector: profile.color_selector,
        elapsed: 0.0,
        duration: profile.duration,
    });
    true
}

fn queue_original_fighter_projectile(
    effects: &mut Vec<WeaponEffect>,
    fighters: &[TacticalFighter],
    source: usize,
    target_ship: Option<(usize, &TacticalShip)>,
    target_fighter: Option<(usize, &TacticalFighter)>,
    weapon: WeaponKind,
    event_strength: f32,
) -> bool {
    let Some(source_fighter) = fighters.get(source) else {
        return false;
    };
    let (target, target_position, fallback_target) = if let Some((index, ship)) = target_ship {
        (index, ship.source_position, [ship.x, ship.y])
    } else if let Some((_index, fighter)) = target_fighter {
        (usize::MAX, fighter.source_position, [fighter.x, fighter.y])
    } else {
        return false;
    };
    let profile = original_projectile_profile(
        weapon,
        event_strength,
        source_fighter
            .tactical_resource
            .map(|resource| resource.tactical_ordinal),
        source_fighter.identity.is_alliance,
    );
    effects.push(WeaponEffect {
        source: usize::MAX,
        target,
        kind: weapon,
        source_position: source_fighter.source_position,
        fallback_source: [source_fighter.x, source_fighter.y],
        target_position,
        fallback_target,
        longitudinal_scale: profile.longitudinal_scale,
        shape_variant: profile.shape_variant,
        color_selector: profile.color_selector,
        elapsed: 0.0,
        duration: profile.duration,
    });
    true
}

/// A fighter's shot at the Death Star, shaped by the `FUN_005ee590`
/// projectile profile. port: the arena-centre fallback target stands in when
/// the Death Star has no source position.
fn queue_original_fighter_to_death_star_projectile(
    effects: &mut Vec<WeaponEffect>,
    fighters: &[TacticalFighter],
    source: usize,
    death_star: TacticalDeathStar,
    weapon: WeaponKind,
    event_strength: f32,
) -> bool {
    let Some(source_fighter) = fighters.get(source) else {
        return false;
    };
    let profile = original_projectile_profile(
        weapon,
        event_strength,
        source_fighter
            .tactical_resource
            .map(|resource| resource.tactical_ordinal),
        source_fighter.identity.is_alliance,
    );
    effects.push(WeaponEffect {
        source: usize::MAX,
        target: usize::MAX,
        kind: weapon,
        source_position: source_fighter.source_position,
        fallback_source: [source_fighter.x, source_fighter.y],
        target_position: death_star.source_position,
        fallback_target: [ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5],
        longitudinal_scale: profile.longitudinal_scale,
        shape_variant: profile.shape_variant,
        color_selector: profile.color_selector,
        elapsed: 0.0,
        duration: profile.duration,
    });
    true
}

/// `FUN_005b23e0`/`FUN_005b2440` hold one tractor target per source and
/// `FUN_005b24d0`/`FUN_005b2480` up to four gravity targets; `FUN_005d3ac0`
/// shows gravity over tractor and resets the frame when the visible kind
/// changes (`tactical-subsystem-field-command-path.md`).
fn set_original_tactical_field(
    effects: &mut Vec<TacticalFieldEffect>,
    source: usize,
    target: usize,
    kind: OriginalTacticalFieldKind,
    active: bool,
) -> bool {
    if active {
        let duplicate = effects.iter().any(|effect| {
            effect.target == target
                && match kind {
                    OriginalTacticalFieldKind::Tractor => effect.tractor_sources.contains(&source),
                    OriginalTacticalFieldKind::Gravity => effect.gravity_sources.contains(&source),
                }
        });
        if duplicate {
            return false;
        }
        let source_assignments = effects
            .iter()
            .filter(|effect| match kind {
                OriginalTacticalFieldKind::Tractor => effect.tractor_sources.contains(&source),
                OriginalTacticalFieldKind::Gravity => effect.gravity_sources.contains(&source),
            })
            .count();
        let source_limit = match kind {
            OriginalTacticalFieldKind::Tractor => 1,
            OriginalTacticalFieldKind::Gravity => 4,
        };
        if source_assignments >= source_limit {
            return false;
        }
    }
    let existing = effects.iter().position(|effect| effect.target == target);
    if existing.is_none() && !active {
        return false;
    }
    let index = existing.unwrap_or_else(|| {
        effects.push(TacticalFieldEffect {
            target,
            tractor_sources: Vec::new(),
            gravity_sources: Vec::new(),
            frame: 0,
            frame_elapsed: 0.0,
        });
        effects.len() - 1
    });
    let effect = &mut effects[index];
    let previous = effect.visible_kind();
    let sources = match kind {
        OriginalTacticalFieldKind::Tractor => &mut effect.tractor_sources,
        OriginalTacticalFieldKind::Gravity => &mut effect.gravity_sources,
    };
    if active {
        sources.push(source);
    } else {
        let Some(source_index) = sources.iter().position(|candidate| *candidate == source) else {
            return false;
        };
        sources.remove(source_index);
    }
    let current = effect.visible_kind();
    if current != previous {
        effect.frame = 0;
        effect.frame_elapsed = 0.0;
    }
    if current.is_none() {
        effects.remove(index);
    }
    true
}

fn remove_tractor_source(effects: &mut Vec<TacticalFieldEffect>, source: usize) {
    for effect in effects.iter_mut() {
        let previous = effect.visible_kind();
        effect
            .tractor_sources
            .retain(|candidate| *candidate != source);
        if effect.visible_kind() != previous {
            effect.frame = 0;
            effect.frame_elapsed = 0.0;
        }
    }
    effects.retain(|effect| effect.visible_kind().is_some());
}

/// `FUN_005b1790`: tractor power times the hull ratio, less a quarter of it
/// per tractor hit, at least zero.
fn original_effective_tractor_power(ship: &TacticalShip) -> f32 {
    if !ship.alive || !ship.subsystem_capacity.tractor || ship.tractor_beam_power <= 0.0 {
        return 0.0;
    }
    let hull_ratio = if ship.hull_max > 0 {
        ship.hull_current.max(0) as f32 / ship.hull_max as f32
    } else {
        0.0
    };
    (ship.tractor_beam_power * hull_ratio
        - ship.tractor_beam_power * 0.25 * f32::from(ship.subsystem_damage.tractor))
    .max(0.0)
}

/// Recover the active maneuver contribution returned by `FUN_005ad750` and
/// `FUN_005afb70`. Missing state records return one. Active records return
/// `9 - value`, clamped to the inclusive source range one through nine.
fn original_engine_mode_bonus(maneuver_state_value: Option<f32>) -> f32 {
    maneuver_state_value.map_or(ORIGINAL_ENGINE_MODE_MIN_BONUS, |value| {
        (ORIGINAL_ENGINE_MODE_MAX_BONUS - value).clamp(
            ORIGINAL_ENGINE_MODE_MIN_BONUS,
            ORIGINAL_ENGINE_MODE_MAX_BONUS,
        )
    })
}

/// `FUN_005b2f30` multiplies effective engine power by the non-negative dot
/// product between the current and desired retained-mode direction vectors.
fn original_movement_alignment(ship: &TacticalShip) -> f32 {
    ship.source_forward
        .normalized()
        .dot(ship.source_desired_forward.normalized())
        .clamp(0.0, 1.0)
}

/// Exact `FUN_005b2e60` overlap test. The source first compares the absolute Y
/// separation against the sum of vertical radii, then compares squared X/Z
/// distance against half the sum of planar diameters.
fn original_tactical_objects_overlap(first: &TacticalShip, second: &TacticalShip) -> bool {
    let (Some(first_envelope), Some(second_envelope)) = (
        first.source_collision_envelope,
        second.source_collision_envelope,
    ) else {
        return false;
    };
    if (first.source_position.y - second.source_position.y).abs()
        >= first_envelope.vertical_radius + second_envelope.vertical_radius
    {
        return false;
    }
    let x = first.source_position.x - second.source_position.x;
    let z = first.source_position.z - second.source_position.z;
    let planar_radius = (first_envelope.planar_diameter + second_envelope.planar_diameter) * 0.5;
    x.mul_add(x, z * z) < planar_radius * planar_radius
}

/// Formation ordering recovered from `FUN_005c81d0`. The source stores the
/// negative of this value in an ascending ordered list, so the effective order
/// is greatest maximum weapon range first and greatest available sublight
/// power second. Stable fleet order resolves exact ties.
fn original_formation_capability_score(ship: &TacticalShip) -> f32 {
    let maximum_weapon_range = ship
        .weapon_ranges
        .laser_cannon
        .max(ship.weapon_ranges.ion_cannon)
        .max(ship.weapon_ranges.turbolaser);
    let undamaged_power =
        ship.sublight_engine_power + original_engine_mode_bonus(ship.maneuver_state_value);
    let available_sublight = (undamaged_power
        - undamaged_power * 0.25 * f32::from(ship.subsystem_damage.engines))
    .max(0.0);
    maximum_weapon_range.mul_add(ORIGINAL_FORMATION_RANGE_PRIORITY, available_sublight)
}

/// Exact alternating slot sequence from `FUN_005ec400`: leader center, first
/// follower right one spacing, second follower left one spacing, then right
/// and left at each increasing multiple.
fn original_formation_lateral_offset(member_ordinal: usize, spacing: f32) -> f32 {
    if member_ordinal == 0 {
        return 0.0;
    }
    let magnitude = member_ordinal.div_ceil(2) as f32 * spacing;
    if member_ordinal & 1 == 1 {
        magnitude
    } else {
        -magnitude
    }
}

/// `FUN_005b9c60` installs pi / 12 for the slowest hulls. Faster hulls use
/// pi * 0.5 * clamp(raw + 1, 9) / 9 radians per second.
fn original_capital_turn_rate(maneuverability: u32) -> f32 {
    let effective = maneuverability.saturating_add(1) as f32;
    if effective > 1.0 {
        std::f32::consts::PI * 0.5 * effective.min(ORIGINAL_CAPITAL_MANEUVERABILITY_MAX)
            / ORIGINAL_CAPITAL_MANEUVERABILITY_MAX
    } else {
        ORIGINAL_CAPITAL_TURN_RATE_FALLBACK
    }
}

fn source_direction_angle(current: TacticalWorldVector, desired: TacticalWorldVector) -> f32 {
    let current = current.normalized();
    let desired = desired.normalized();
    current.dot(desired).clamp(-1.0, 1.0).acos()
}

/// Apply `FUN_005cd460`'s signed XZ turn and snap branch.
fn turn_original_tactical_ship(ship: &mut TacticalShip, delta_milliseconds: f32) {
    if ship.source_desired_forward == TacticalWorldVector::ZERO {
        return;
    }
    let angle = source_direction_angle(ship.source_forward, ship.source_desired_forward);
    if angle <= f32::EPSILON {
        return;
    }
    let turn_step = original_capital_turn_rate(ship.maneuverability) * delta_milliseconds * 0.001;
    if turn_step >= angle {
        ship.source_forward = ship.source_desired_forward.normalized();
        return;
    }

    let current = ship.source_forward.normalized();
    let desired = ship.source_desired_forward.normalized();
    let signed_angle =
        if -current.z * desired.x + desired.z * current.x + desired.y * current.y < 0.0 {
            -turn_step
        } else {
            turn_step
        };
    let (sin, cos) = signed_angle.sin_cos();
    ship.source_forward = TacticalWorldVector {
        x: current.x * cos - sin * current.z,
        y: current.y,
        z: current.x * sin + cos * current.z,
    }
    .normalized();
}

/// `FUN_005b17f0`: base sublight plus the mode bonus, less a quarter per
/// engine hit and each holding source's tractor power, not scaled by hull;
/// `FUN_005b16b0` divides it by base power for the percentage.
fn original_tactical_mobility(
    ships: &[TacticalShip],
    field_effects: &[TacticalFieldEffect],
    target: usize,
) -> Option<TacticalSubsystemMobility> {
    let ship = ships.get(target)?;
    let engine_mode_bonus = original_engine_mode_bonus(ship.maneuver_state_value);
    if !ship.subsystem_capacity.engines || ship.sublight_engine_power <= 0.0 {
        return Some(TacticalSubsystemMobility {
            base_engine_power: ship.sublight_engine_power.max(0.0),
            engine_mode_bonus,
            active_tractor_power: 0.0,
            effective_engine_power: 0.0,
            engine_percent: 0,
        });
    }
    let active_tractor_power = field_effects
        .iter()
        .filter(|effect| effect.target == target)
        .flat_map(|effect| effect.tractor_sources.iter().copied())
        .filter_map(|source| ships.get(source))
        .map(original_effective_tractor_power)
        .sum::<f32>();
    let undamaged_power = (ship.sublight_engine_power + engine_mode_bonus).max(0.0);
    let effective_engine_power = (undamaged_power
        - undamaged_power * 0.25 * f32::from(ship.subsystem_damage.engines)
        - active_tractor_power)
        .max(0.0);
    let engine_percent =
        (effective_engine_power * 100.0 / ship.sublight_engine_power).clamp(0.0, 100.0) as u8;
    Some(TacticalSubsystemMobility {
        base_engine_power: ship.sublight_engine_power,
        engine_mode_bonus,
        active_tractor_power,
        effective_engine_power,
        engine_percent,
    })
}

pub fn refresh_original_subsystem_conditions(
    ships: &mut [TacticalShip],
    field_effects: &[TacticalFieldEffect],
) {
    let engine_percentages: Vec<u8> = (0..ships.len())
        .map(|target| {
            original_tactical_mobility(ships, field_effects, target)
                .map_or(0, |mobility| mobility.engine_percent)
        })
        .collect();
    for (ship, engine_percent) in ships.iter_mut().zip(engine_percentages) {
        ship.refresh_subsystem_condition();
        ship.subsystem_condition.engines = engine_percent;
    }
}

fn original_subsystem_repair_kind(
    damage: TacticalSubsystemDamage,
    mut ordinal: u16,
) -> Option<TacticalSubsystemKind> {
    if ordinal == 0 || ordinal > damage.total_hits() {
        return None;
    }
    // FUN_005b1490 selects uniformly among individual outstanding hits in
    // engine, shield, hyperdrive, tractor, weapon order.
    for (kind, hits) in [
        (TacticalSubsystemKind::Engines, damage.engines),
        (TacticalSubsystemKind::Shields, damage.shields),
        (TacticalSubsystemKind::Hyperdrive, damage.hyperdrive),
        (TacticalSubsystemKind::Tractor, damage.tractor),
        (TacticalSubsystemKind::Weapons, damage.weapons),
    ] {
        if ordinal <= u16::from(hits) {
            return Some(kind);
        }
        ordinal -= u16::from(hits);
    }
    None
}

/// `FUN_005b1490`: a ship with outstanding hits repairs one when an
/// inclusive 1-100 draw is no greater than its CAPSHPSD damage control.
pub fn attempt_original_subsystem_repair(
    ship: &mut TacticalShip,
    ship_index: usize,
    chance_roll: u8,
    selection_roll: u16,
) -> Option<TacticalSubsystemRepair> {
    if ship.subsystem_damage.total_hits() == 0
        || ship.damage_control == 0
        || !(1..=100).contains(&chance_roll)
        || chance_roll > ship.damage_control
    {
        return None;
    }
    let kind = original_subsystem_repair_kind(ship.subsystem_damage, selection_roll)?;
    let (hits_before, hits_after) = ship.remove_subsystem_hit(kind)?;
    Some(TacticalSubsystemRepair {
        ship: ship_index,
        kind,
        hits_before,
        hits_after,
    })
}

/// Marks a damage event that rolls for no subsystem hit; real rolls run
/// 0 through 100.
const ORIGINAL_NO_SUBSYSTEM_ROLL: u8 = u8::MAX;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OriginalTacticalDamageRolls {
    pub shield: u8,
    pub hull: u8,
}

/// Tactical manager random stream recovered from `FUN_0061a310` and
/// `FUN_005a8a70`. Every tactical consumer advances this one stream in event
/// order instead of deriving independent hashes from the frame number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OriginalTacticalRng {
    state: u32,
}

impl OriginalTacticalRng {
    #[must_use]
    pub const fn new(state: u32) -> Self {
        Self { state }
    }

    #[must_use]
    pub const fn state(self) -> u32 {
        self.state
    }

    /// `FUN_0061a310`: state × `0x343fd` + `0x269ec3`, returning bits 16-30.
    fn next_u15(&mut self) -> u16 {
        self.state = self.state.wrapping_mul(0x0003_43fd).wrapping_add(0x0026_9ec3);
        ((self.state >> 16) & 0x7fff) as u16
    }

    fn inclusive(&mut self, minimum: u32, maximum: u32) -> u32 {
        debug_assert!(minimum <= maximum);
        let span = maximum.saturating_sub(minimum).saturating_add(1);
        minimum + u32::from(self.next_u15()) % span
    }
}

/// The headless battle's generator (`FUN_005f5700` on `battle+4`), which
/// `FUN_005af420` draws from for the shield-first damage (`FUN_005b54d0`)
/// and the headless trench run (`FUN_005aefc0`): a Lehmer step
/// (`FUN_005f57a0`, multiplier `0xa18e`) seeded with 12345678 and sixteen
/// warm-up steps (`FUN_005af5f0.c:38`, `FUN_005f56b0`).
/// hyp: `FUN_00596f30` builds the battle object once and no traced code
/// frees it, so the stream runs on across battles; a game held in memory
/// keeps it ([`GameWorld::battle_rng`]) and a loaded one starts it afresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OriginalBattleRng {
    state: i32,
}

impl Default for OriginalBattleRng {
    fn default() -> Self {
        let mut state = ORIGINAL_BATTLE_RNG_SEED;
        for _ in 0..ORIGINAL_BATTLE_RNG_WARM_UP {
            state = Self::step(state);
        }
        Self { state }
    }
}

impl OriginalBattleRng {
    /// `FUN_005f57a0`, in the source's wrapping 32-bit arithmetic.
    fn step(x: i32) -> i32 {
        let next = x
            .wrapping_mul(0xa18e)
            .wrapping_add((x / 0xcad4).wrapping_mul(-0x7fff_ffff));
        if next < 0 {
            next.wrapping_add(0x7fff_ffff)
        } else {
            next
        }
    }

    /// `FUN_005af420(a, b)`: `a` plus a draw of `[0, b - a)`; a span of one
    /// or less draws nothing (`FUN_005f5700`).
    pub fn between(&mut self, minimum: i32, maximum: i32) -> i32 {
        let span = maximum - minimum;
        if span <= 1 {
            return minimum;
        }
        self.state = Self::step(self.state);
        minimum + self.state / (0x7fff_fffe / span)
    }
}

/// A subsystem roll for a shield-generator hit when the ship had shields,
/// and another for a hull subsystem hit when the damage overflows them,
/// each `draw`n from 0-100: `FUN_005af420` on the battle generator in
/// `FUN_005b54d0` (`[0, 99]`), the tactical stream in the played battle's
/// receivers.
fn original_tactical_damage_rolls(
    ships: &[TacticalShip],
    target: usize,
    damage: i32,
    mut draw: impl FnMut() -> u32,
) -> OriginalTacticalDamageRolls {
    let Some(ship) = ships.get(target) else {
        return OriginalTacticalDamageRolls { shield: 0, hull: 0 };
    };
    if !ship.alive || ship.hull_current <= 0 || damage <= 0 {
        return OriginalTacticalDamageRolls { shield: 0, hull: 0 };
    }
    let shield = if ship.shield > 0 {
        u8::try_from(draw()).unwrap_or(100)
    } else {
        0
    };
    let hull = if ship.shield.saturating_sub(damage) < 0 {
        u8::try_from(draw()).unwrap_or(100)
    } else {
        0
    };
    OriginalTacticalDamageRolls { shield, hull }
}

/// The played battle's damage receivers (`FUN_005b5100`, `FUN_005b5750`,
/// `FUN_005b5b60`, `FUN_005b5f50`) roll for subsystem hits (events
/// `0x1e`…`0x22`) only on the player's own ships: `FUN_005c0170` sets
/// `+0x124` on every unit of the other side, and the receivers skip both the
/// roll and its random draw when it is set.
fn original_windowed_damage_rolls(
    ships: &[TacticalShip],
    target: usize,
    damage: i32,
    rng: &mut OriginalTacticalRng,
    player_is_attacker: bool,
) -> OriginalTacticalDamageRolls {
    if ships
        .get(target)
        .is_some_and(|ship| ship.is_attacker != player_is_attacker)
    {
        return OriginalTacticalDamageRolls {
            shield: ORIGINAL_NO_SUBSYSTEM_ROLL,
            hull: ORIGINAL_NO_SUBSYSTEM_ROLL,
        };
    }
    original_tactical_damage_rolls(ships, target, damage, || rng.inclusive(0, 100))
}

/// `FUN_005b54d0`'s score bands, `_DAT_0066c520` through `_DAT_0066c530`
/// and `_DAT_0066c51c` (50, 65, 80, 90, 95, 100), passed to slot `+0xa0`
/// as categories 0-4.
fn original_hull_subsystem_kind(score: f32) -> Option<TacticalSubsystemKind> {
    if score <= 50.0 {
        None
    } else if score <= 65.0 {
        Some(TacticalSubsystemKind::Shields)
    } else if score <= 80.0 {
        Some(TacticalSubsystemKind::Weapons)
    } else if score <= 90.0 {
        Some(TacticalSubsystemKind::Tractor)
    } else if score <= 95.0 {
        Some(TacticalSubsystemKind::Engines)
    } else if score <= 100.0 {
        Some(TacticalSubsystemKind::Hyperdrive)
    } else {
        None
    }
}

pub fn apply_original_capital_damage(
    ships: &mut [TacticalShip],
    field_effects: &mut Vec<TacticalFieldEffect>,
    target: usize,
    damage: i32,
    rolls: OriginalTacticalDamageRolls,
) -> OriginalTacticalImpactStage {
    let (hull_before, hull_after, tractor_disabled) = {
        let Some(ship) = ships.get_mut(target) else {
            return OriginalTacticalImpactStage::Hit;
        };
        if !ship.alive || ship.hull_current <= 0 || damage <= 0 {
            return OriginalTacticalImpactStage::Hit;
        }

        let hull_before = ship.hull_current;
        let shield_before = ship.shield;
        let shield_after = shield_before.saturating_sub(damage);
        ship.shield = shield_after;

        let mut tractor_disabled = false;
        if shield_before > 0 && rolls.shield != ORIGINAL_NO_SUBSYSTEM_ROLL {
            let shield_loss_percent =
                (100.0 / shield_before as f32) * (shield_before - shield_after) as f32;
            if f32::from(rolls.shield) <= shield_loss_percent.max(1.0) {
                tractor_disabled |= ship.add_subsystem_hit(TacticalSubsystemKind::Shields);
            }
        }

        if ship.shield < 0 {
            // FUN_005b54d0 carries negative shield overflow into hull and enforces
            // at least one hull point on the shield-crossing branch.
            let hull_damage = (-ship.shield).max(1);
            ship.shield = 0;
            ship.hull_current = ship.hull_current.saturating_sub(hull_damage).max(0);
            let score = f32::from(rolls.hull)
                + (ship.hull_current - hull_before) as f32 * 100.0 / hull_before as f32;
            if let Some(kind) = original_hull_subsystem_kind(score)
                .filter(|_| rolls.hull != ORIGINAL_NO_SUBSYSTEM_ROLL)
            {
                tractor_disabled |= ship.add_subsystem_hit(kind);
            }
        }

        if ship.hull_current == 0 {
            ship.alive = false;
        }
        (hull_before, ship.hull_current, tractor_disabled)
    };
    if tractor_disabled {
        remove_tractor_source(field_effects, target);
    }
    refresh_original_subsystem_conditions(ships, field_effects);

    if hull_after == 0 {
        OriginalTacticalImpactStage::Destroyed
    } else if hull_after < hull_before {
        OriginalTacticalImpactStage::Damage
    } else {
        OriginalTacticalImpactStage::Hit
    }
}

/// Battle outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatWinner {
    Attacker,
    Defender,
    Draw,
}

/// Result of the source Death Star trench-run branch.
///
/// `TACTICALRESULT_UPDATE` states 6 and 7 route to the successful and failed
/// films respectively. The presentation layer consumes this value once and
/// returns to the live tactical session afterward.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TacticalTrenchRunOutcome {
    Success,
    Failure,
}

/// Per-arc, per-family strongest target for one capital ship over its four
/// arcs: autonomous collection `FUN_005b3a40`, or the direct target alone
/// through `FUN_005b6530` (`tactical-weapon-loop.md`).
fn original_capital_weapon_candidates(
    ships: &[TacticalShip],
    source: usize,
    targets: &[usize],
    direct_target: Option<usize>,
) -> [OriginalTacticalArcCandidates; 4] {
    let mut candidates = [OriginalTacticalArcCandidates::default(); 4];
    let Some(source_ship) = ships.get(source) else {
        return candidates;
    };
    let scale_for_hull = direct_target.is_none();

    for &target in targets {
        if direct_target.is_some_and(|direct| direct != target) {
            continue;
        }
        let Some(target_ship) = ships
            .get(target)
            .filter(|ship| ship.alive && !ship.retreating)
        else {
            continue;
        };
        let arc_index = original_tactical_weapon_arc(source_ship, target_ship).index();
        let arc = source_ship.weapon_arcs[arc_index];
        if !arc.ready {
            continue;
        }
        let delta = source_delta(target_ship.source_position, source_ship.source_position);
        let distance = delta.x.hypot(delta.y).hypot(delta.z);

        for kind in [
            WeaponKind::LaserCannon,
            WeaponKind::IonCannon,
            WeaponKind::Turbolaser,
        ] {
            let count = original_weapon_count(arc, kind);
            let range = original_weapon_range(source_ship, kind);
            if count == 0 || range <= 0.0 || distance > range {
                continue;
            }
            let strength =
                original_capital_weapon_strength(source_ship, count, kind, scale_for_hull);
            let candidate = match kind {
                WeaponKind::LaserCannon => &mut candidates[arc_index].laser,
                WeaponKind::IonCannon => &mut candidates[arc_index].ion,
                WeaponKind::Turbolaser => &mut candidates[arc_index].turbolaser,
                WeaponKind::FighterAttack => continue,
            };
            // The executable keeps the first target on an equal score.
            if candidate.strength < strength {
                *candidate = OriginalTacticalWeaponCandidate {
                    strength,
                    target: Some(target),
                };
            }
        }
    }

    candidates
}

fn original_tactical_integer_damage(strength: f32) -> i32 {
    if !strength.is_finite() || strength <= 0.0 {
        0
    } else {
        // The source event transports a float. The current public hull and
        // shield state remains integral, so round only at that storage edge.
        strength.round().max(1.0) as i32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OriginalFighterTarget {
    Capital(usize),
    Fighter(usize),
    DeathStar,
}

fn original_fighter_weapon_strength(
    fighter: &TacticalFighter,
    count: u32,
    kind: WeaponKind,
    target_is_fighter: bool,
) -> f32 {
    let target_modifier = if target_is_fighter {
        match kind {
            WeaponKind::IonCannon => ORIGINAL_ION_VS_FIGHTER,
            WeaponKind::LaserCannon => ORIGINAL_LASER_VS_FIGHTER,
            WeaponKind::Turbolaser => ORIGINAL_TURBOLASER_VS_FIGHTER,
            WeaponKind::FighterAttack => 0.0,
        }
    } else {
        match kind {
            WeaponKind::LaserCannon => ORIGINAL_LASER_VS_CAPITAL,
            WeaponKind::IonCannon | WeaponKind::Turbolaser => 1.0,
            WeaponKind::FighterAttack => 0.0,
        }
    };
    count as f32 * fighter.weapon_hull_factor() * ORIGINAL_CAPITAL_WEAPON_STRENGTH * target_modifier
}

fn original_fighter_weapon_range(fighter: &TacticalFighter, kind: WeaponKind) -> f32 {
    match kind {
        WeaponKind::LaserCannon => fighter.weapon_ranges.laser_cannon,
        WeaponKind::IonCannon => fighter.weapon_ranges.ion_cannon,
        WeaponKind::Turbolaser => fighter.weapon_ranges.turbolaser,
        WeaponKind::FighterAttack => fighter.torpedo_range,
    }
}

fn apply_original_fighter_damage(
    fighters: &mut [TacticalFighter],
    target: usize,
    event_strength: f32,
    source_maneuverability: f32,
) -> OriginalTacticalImpactStage {
    let Some(fighter) = fighters.get_mut(target) else {
        return OriginalTacticalImpactStage::Hit;
    };
    if !fighter.alive || fighter.hull_current <= 0.0 || event_strength <= 0.0 {
        return OriginalTacticalImpactStage::Hit;
    }

    // FUN_005b5100, FUN_005b5b60, and FUN_005b5f50 divide incoming damage
    // by target/source maneuverability for a fighter recipient, clamped at
    // one tenth before division.
    let maneuver_ratio =
        (fighter.effective_maneuverability() / source_maneuverability.max(1.0)).max(0.1);
    let damage = (event_strength / maneuver_ratio).max(1.0);
    let hull_before = fighter.hull_current;
    fighter.shield -= damage;
    if fighter.shield < 0.0 {
        let overflow = -fighter.shield;
        fighter.shield = 0.0;
        fighter.hull_current = (fighter.hull_current - overflow.max(1.0)).max(0.0);
        fighter.refresh_craft_count();
    }

    if fighter.hull_current <= 0.0 {
        OriginalTacticalImpactStage::Destroyed
    } else if fighter.hull_current < hull_before {
        OriginalTacticalImpactStage::Damage
    } else {
        OriginalTacticalImpactStage::Hit
    }
}

fn queue_original_capital_to_fighter_projectile(
    effects: &mut Vec<WeaponEffect>,
    ships: &[TacticalShip],
    fighters: &[TacticalFighter],
    source: usize,
    target: usize,
    weapon: WeaponKind,
    event_strength: f32,
) -> bool {
    let (Some(source_ship), Some(target_fighter)) = (ships.get(source), fighters.get(target))
    else {
        return false;
    };
    let profile = original_projectile_profile(
        weapon,
        event_strength,
        source_ship
            .tactical_resource
            .map(|resource| resource.tactical_ordinal),
        source_ship.identity.is_alliance,
    );
    effects.push(WeaponEffect {
        source,
        target: usize::MAX,
        kind: weapon,
        source_position: source_ship.source_position,
        fallback_source: [source_ship.x, source_ship.y],
        target_position: target_fighter.source_position,
        fallback_target: [target_fighter.x, target_fighter.y],
        longitudinal_scale: profile.longitudinal_scale,
        shape_variant: profile.shape_variant,
        color_selector: profile.color_selector,
        elapsed: 0.0,
        duration: profile.duration,
    });
    true
}

impl BattleSession {
    /// Create a new battle session from two opposing fleets.
    ///
    /// Expands fleet composition into individual ship hulls and positions them
    /// in deployment zones (attacker left, defender right).
    #[must_use]
    pub fn new(
        world: &GameWorld,
        system: SystemKey,
        attacker: FleetKey,
        defender: FleetKey,
        player_is_attacker: bool,
        tick: u64,
        rng_seed: u32,
    ) -> Self {
        Self::new_with_forces(
            world,
            system,
            CombatForce::of_fleet(world, attacker),
            CombatForce::of_fleet(world, defender),
            player_is_attacker,
            tick,
            rng_seed,
        )
    }

    /// A battle between two sides, each its fleet (if any) and the squadrons
    /// the system holds for it ([`CombatForce`]).
    pub fn new_with_forces(
        world: &GameWorld,
        system: SystemKey,
        attacker_force: CombatForce,
        defender_force: CombatForce,
        player_is_attacker: bool,
        tick: u64,
        rng_seed: u32,
    ) -> Self {
        let (attacker, defender) = (attacker_force.fleet, defender_force.fleet);
        let system_name = world
            .systems
            .get(system)
            .map_or_else(|| "Unknown".into(), |s| s.name.clone());
        let system_picture_id = world
            .systems
            .get(system)
            .map_or(1, |system| planet_picture_id(system.dat_id));
        let holder_is_alliance = world
            .systems
            .get(system)
            .and_then(|system| system.control.faction())
            .map(|faction| faction == crate::dat::Faction::Alliance);
        let blockaded_at_start = holder_is_alliance.is_some_and(|holder| {
            let fleets_of =
                |alliance: bool| !crate::combat::fighting_fleets(world, system, alliance).is_empty();
            fleets_of(!holder) && !fleets_of(holder)
        });

        let mut ships = Vec::new();
        let mut fighters = Vec::new();

        // Expand every fleet of each side at the system into individual
        // hulls, the lead fleet first, then seat the squadrons the system
        // holds for it (FUN_0040b0e0).
        let mut side_fleets: [Vec<FleetKey>; 2] = [Vec::new(), Vec::new()];
        for (force, is_attacker) in [(attacker_force, true), (defender_force, false)] {
            let fleets = &mut side_fleets[usize::from(!is_attacker)];
            // Only usable fleets put children in the battle (FUN_0040b0e0
            // exports a mode-1 walk, `crate::combat::fighting_fleets`), the
            // lead first when it is one.
            fleets.extend(
                force
                    .fleet
                    .filter(|&key| world.fleets.get(key).is_some_and(crate::world::Fleet::usable)),
            );
            fleets.extend(
                crate::combat::fighting_fleets(world, system, force.is_alliance)
                    .into_iter()
                    .filter(|&key| Some(key) != force.fleet),
            );
            for &fleet in fleets.iter() {
                Self::expand_fleet(world, fleet, is_attacker, &mut ships, &mut fighters);
            }
            Self::expand_system_squadrons(
                world,
                system,
                force.is_alliance,
                is_attacker,
                &mut fighters,
            );
        }

        let hyperdrive_squadrons: Vec<usize> = (0..fighters.len())
            .filter(|&index| {
                world
                    .fighter_classes
                    .get(fighters[index].class_key)
                    .is_some_and(|class| class.hyperdrive > 0 || class.hyperdrive_if_damaged > 0)
            })
            .collect();
        // The fleet loop exports a Death Star ship only as the station's
        // short record (`FUN_0040b0e0.c:86-118`), so it is no capital
        // participant (`expand_fleet`).
        let death_star = side_fleets
            .iter()
            .zip([true, false])
            .flat_map(|(fleets, is_attacker)| fleets.iter().map(move |&key| (key, is_attacker)))
            .find_map(|(fleet_key, is_attacker)| {
                TacticalDeathStar::of_fleet(
                    world,
                    system,
                    fleet_key,
                    is_attacker,
                    OriginalTacticalLayout::default(),
                )
            });
        // The squadrons dock (`FUN_005964c0.c:133-153`) before
        // `FUN_005ab650` lays the battle out, which skips a docked one.
        Self::seat_carried_squadrons(
            world,
            &ships,
            &mut fighters,
            &hyperdrive_squadrons,
            death_star.map(|station| station.hangar_capacity),
        );

        // Retain the working 2D fallback separately from the recovered source
        // world coordinates. P58 will replace each fallback only after its
        // DAT-to-tactical resource identity is proven.
        Self::auto_place_ships(&mut ships);
        Self::auto_place_fighters(&mut fighters, &ships);
        let source_layout = Self::assign_original_world_positions(&mut ships, &mut fighters);
        let death_star = death_star.map(|station| station.placed(source_layout));
        let gravity_wells = (0..ships.len())
            .filter(|&index| {
                let ship = &ships[index];
                ship.fleet
                    .and_then(|fleet| world.fleets.get(fleet))
                    .and_then(|fleet| fleet.capital_ships.get(ship.fleet_ship_index))
                    .and_then(|instance| world.capital_ship_classes.get(instance.class))
                    .is_some_and(|class| class.gravity_well_projector > 0)
            })
            .collect();
        let navigation_sets = original_navigation_point_sets(source_layout);
        Self::assign_original_task_forces(&mut ships);
        Self::assign_original_fighter_groups(&mut fighters);

        // The original Tactical Display opens with the battle paused and
        // nothing selected: the right-hand housing shows the idle seal panel,
        // the child `FUN_005c5800` places at (0, 0) (original capture
        // 2026-10-11). The Battle Alert's Take Command choice is the entry
        // action, not a second invented placement phase.
        let selected_ship = None;

        BattleSession {
            system,
            system_name,
            system_picture_id,
            attacker_fleet: attacker,
            defender_fleet: defender,
            attacker_is_alliance: attacker_force.is_alliance,
            player_is_attacker,
            phase: BattlePhase::Combat,
            ships,
            fighters,
            death_star,
            death_star_beam: None,
            trench_run_outcome: None,
            trench_run_remaining_ms: None,
            trench_run_group: None,
            trench_run_commander: TacticalTrenchCommander::of_battle(world, system, &side_fleets, attacker_force.is_alliance),
            trench_run_pending_outcome: None,
            trench_run_participants: Vec::new(),
            trench_run_snapshot: Vec::new(),
            trench_run_chatter: [ORIGINAL_TRENCH_RUN_SILENT_EVENT; 9],
            trench_run_chatter_cursor: 0,
            trench_run_cinematic_pending: false,
            navigation_sets,
            source_layout,
            selected_ship,
            selected_fighter_group: None,
            placement_confirmed: true,
            start_tick: tick,
            combat_tick: 0,
            tactical_rng: OriginalTacticalRng::new(rng_seed),
            battle_rng: world.battle_rng,
            settled_crew: None,
            weapon_effects: Vec::new(),
            impact_effects: Vec::new(),
            pending_audio_cues: Vec::new(),
            pending_voice_cues: Vec::new(),
            pending_log_lines: Vec::new(),
            field_effects: Vec::new(),
            subsystem_repairs: Vec::new(),
            paused: true,
            combat_speed: 1,
            step_accumulator: 0.0,
            winner: None,
            stalemate: TacticalStalemateWatch::default(),
            death_star_ai: TacticalDeathStarAi::default(),
            hangars: TacticalHangars {
                step: ORIGINAL_HANGAR_FIRST_STEP,
                ..TacticalHangars::default()
            },
            morale: TacticalMorale {
                gravity_wells,
                hyperdrive_squadrons,
                ..TacticalMorale::default()
            },
            engagement: TacticalEngagementState::default(),
            holder_is_alliance,
            arrival_holder_is_alliance: world.arrival_holders.get(&system).copied(),
            blockaded_at_start,
            withdrawal_destination: None,
        }
    }

    /// Start or stop one source's tractor-field contribution on a live target.
    /// One tractor source can hold one target while a target can retain several
    /// exact sources, matching `FUN_005b23e0` through `FUN_005b25d0`.
    pub fn set_tractor_field(&mut self, source: usize, target: usize, active: bool) -> bool {
        if source >= self.ships.len()
            || target >= self.ships.len()
            || source == target
            || !self.ships[source].alive
            || !self.ships[target].alive
        {
            return false;
        }
        let changed = set_original_tactical_field(
            &mut self.field_effects,
            source,
            target,
            OriginalTacticalFieldKind::Tractor,
            active,
        );
        if changed {
            refresh_original_subsystem_conditions(&mut self.ships, &self.field_effects);
        }
        changed
    }

    /// Start or stop one source's gravity-field contribution on a live target.
    /// One gravity source can hold four exact targets. Gravity preempts tractor
    /// in each target's shared visual slot, matching `FUN_005b24d0` through
    /// `FUN_005b2550`.
    pub fn set_gravity_field(&mut self, source: usize, target: usize, active: bool) -> bool {
        if source >= self.ships.len()
            || target >= self.ships.len()
            || source == target
            || !self.ships[source].alive
            || !self.ships[target].alive
        {
            return false;
        }
        set_original_tactical_field(
            &mut self.field_effects,
            source,
            target,
            OriginalTacticalFieldKind::Gravity,
            active,
        )
    }

    /// Return the source-derived sublight state for one live participant.
    #[must_use]
    pub fn subsystem_mobility(&self, target: usize) -> Option<TacticalSubsystemMobility> {
        original_tactical_mobility(&self.ships, &self.field_effects, target)
    }

    /// Return the exact non-negative current-to-desired direction multiplier
    /// consumed by the original capital-ship movement routine.
    #[must_use]
    pub fn movement_alignment(&self, target: usize) -> Option<f32> {
        self.ships.get(target).map(original_movement_alignment)
    }

    /// Set or clear the active state-record value consumed by the recovered
    /// maneuver-mode producer. Command routing will call this once restored.
    pub fn set_maneuver_state_value(&mut self, target: usize, value: Option<f32>) -> bool {
        let Some(ship) = self.ships.get_mut(target) else {
            return false;
        };
        if value.is_some_and(|value| !value.is_finite()) {
            return false;
        }
        ship.maneuver_state_value = value;
        refresh_original_subsystem_conditions(&mut self.ships, &self.field_effects);
        true
    }

    /// Apply `FUN_005cd460`'s current-to-desired turn, `FUN_005b2f30`'s
    /// velocity contract, and `FUN_005cd640`'s millisecond position integrator.
    pub fn advance_original_tactical_movement(&mut self, delta_milliseconds: f32) {
        if !delta_milliseconds.is_finite() || delta_milliseconds <= 0.0 {
            return;
        }
        let engine_power = self
            .ships
            .iter()
            .enumerate()
            .map(|(index, _)| {
                original_tactical_mobility(&self.ships, &self.field_effects, index)
                    .map_or(0.0, |mobility| mobility.effective_engine_power)
            })
            .collect::<Vec<_>>();
        let seconds = delta_milliseconds * 0.001;
        let mut completed_player_task_forces = [false; 8];
        for (index, effective_engine_power) in engine_power.into_iter().enumerate() {
            if !self.ships[index].alive || self.ships[index].retreating {
                self.ships[index].source_velocity = TacticalWorldVector::ZERO;
                continue;
            }
            let previous_position = self.ships[index].source_position;
            {
                let ship = &mut self.ships[index];
                let waypoint_delta = ship
                    .source_waypoint
                    .map(|waypoint| source_delta(waypoint, ship.source_position));
                let waypoint_distance =
                    waypoint_delta.map(|delta| delta.x.hypot(delta.y).hypot(delta.z));
                if let Some(delta) = waypoint_delta {
                    ship.source_desired_forward = delta.normalized();
                }
                turn_original_tactical_ship(ship, delta_milliseconds);
                let alignment = original_movement_alignment(ship);
                let forward = ship.source_forward.normalized();
                let speed = effective_engine_power * alignment;
                ship.source_velocity = TacticalWorldVector {
                    x: forward.x * speed,
                    y: forward.y * speed,
                    z: forward.z * speed,
                };
                if let (Some(waypoint), Some(distance)) = (ship.source_waypoint, waypoint_distance)
                {
                    if distance <= speed * seconds {
                        let completed_order = ship.order;
                        ship.source_position = waypoint;
                        ship.source_velocity = TacticalWorldVector::ZERO;
                        if ship.navigation_route.first() == Some(&waypoint) {
                            ship.navigation_route.remove(0);
                        }
                        ship.source_waypoint = ship.navigation_route.first().copied();
                        ship.source_desired_forward = ship
                            .source_waypoint
                            .map_or(TacticalWorldVector::ZERO, |next| {
                                source_delta(next, ship.source_position).normalized()
                            });
                        if ship.source_waypoint.is_none()
                            && !matches!(
                                ship.order,
                                TacticalOrder::Escort | TacticalOrder::Withdraw
                            )
                        {
                            ship.order = TacticalOrder::None;
                            if ship.is_attacker == self.player_is_attacker
                                && matches!(
                                    completed_order,
                                    TacticalOrder::LeftHook
                                        | TacticalOrder::RightHook
                                        | TacticalOrder::Hammer
                                        | TacticalOrder::Anvil
                                )
                            {
                                completed_player_task_forces[usize::from(ship.task_force.min(7))] =
                                    true;
                            }
                        }
                    } else {
                        ship.source_position.x += ship.source_velocity.x * seconds;
                        ship.source_position.y += ship.source_velocity.y * seconds;
                        ship.source_position.z += ship.source_velocity.z * seconds;
                    }
                } else {
                    ship.source_position.x += ship.source_velocity.x * seconds;
                    ship.source_position.y += ship.source_velocity.y * seconds;
                    ship.source_position.z += ship.source_velocity.z * seconds;
                }
            }

            // FUN_005b2f30 performs the spatial query after integration and
            // restores the prior position when FUN_005b2e60 reports overlap.
            // Iterating in stable roster order preserves the source's
            // first-mover rejection behavior.
            if self.ships.iter().enumerate().any(|(other_index, other)| {
                other_index != index
                    && other.alive
                    && !other.retreating
                    && original_tactical_objects_overlap(&self.ships[index], other)
            }) {
                self.ships[index].source_position = previous_position;
            }
        }
        // The original retained formation visual keeps member offsets attached
        // to its moving anchor. Reapply those offsets only after every member
        // has completed its stable-order movement and collision query.
        Self::refresh_original_task_force_geometry(&mut self.ships);
        let faction = self.player_voice_faction();
        for (group, transitioned) in completed_player_task_forces.into_iter().enumerate() {
            if !transitioned {
                continue;
            }
            let group_still_maneuvering = self.ships.iter().any(|ship| {
                ship.alive
                    && ship.is_attacker == self.player_is_attacker
                    && usize::from(ship.task_force) == group
                    && matches!(
                        ship.order,
                        TacticalOrder::LeftHook
                            | TacticalOrder::RightHook
                            | TacticalOrder::Hammer
                            | TacticalOrder::Anvil
                    )
            });
            if !group_still_maneuvering {
                self.pending_voice_cues
                    .push(TacticalVoiceCue::capital_maneuver_complete(
                        faction,
                        group as u8,
                    ));
            }
        }
    }

    /// Advance the recovered `FUN_005cf980` fighter states. State 2 moves the
    /// group toward its reserved carrier, the exact 2.0-unit gate enters state
    /// 3, and the following docking callback removes the tactical object in
    /// state 4 without changing its strategic squadron count.
    pub fn advance_original_fighter_recovery(&mut self, delta_milliseconds: f32) {
        if !delta_milliseconds.is_finite() || delta_milliseconds <= 0.0 {
            return;
        }
        let seconds = delta_milliseconds * 0.001;
        let player_is_attacker = self.player_is_attacker;
        let player_faction = self.player_voice_faction();
        let mut transitioned_groups = [false; 4];
        for fighter in &mut self.fighters {
            match fighter.recovery_state {
                TacticalFighterRecoveryState::Returning => {
                    let Some(carrier) = fighter
                        .recovery_target
                        .and_then(|index| self.ships.get(index))
                        .filter(|ship| ship.alive && ship.is_attacker == fighter.is_attacker)
                    else {
                        fighter.recovery_state = TacticalFighterRecoveryState::AwaitingCarrier;
                        fighter.recovery_target = None;
                        continue;
                    };
                    let delta = source_delta(carrier.source_position, fighter.source_position);
                    let distance = delta.x.hypot(delta.y).hypot(delta.z);
                    if distance < ORIGINAL_FIGHTER_DOCKING_DISTANCE {
                        fighter.recovery_state = TacticalFighterRecoveryState::Docking;
                        continue;
                    }
                    let step = fighter.sublight_engine_power.max(0.0) * seconds;
                    if step > 0.0 {
                        let direction = delta.normalized();
                        let travel = step.min(distance);
                        fighter.source_position.x += direction.x * travel;
                        fighter.source_position.y += direction.y * travel;
                        fighter.source_position.z += direction.z * travel;
                    }
                    let remaining = source_delta(carrier.source_position, fighter.source_position);
                    if remaining.x.hypot(remaining.y).hypot(remaining.z)
                        < ORIGINAL_FIGHTER_DOCKING_DISTANCE
                    {
                        fighter.recovery_state = TacticalFighterRecoveryState::Docking;
                    }
                }
                TacticalFighterRecoveryState::Docking => {
                    fighter.recovery_state = TacticalFighterRecoveryState::Recovered;
                    // `FUN_005b8630`: the hangar's recovered list (`+0x20`,
                    // state 5) under the carrier's id (`+0x164`).
                    if let Some(target) = fighter.recovery_target {
                        fighter.carrier = Some(TacticalBerth::Ship(target));
                    }
                    fighter.docked = Some(TacticalDock::Recovered);
                    fighter.order = TacticalOrder::None;
                    fighter.selected = false;
                    fighter.alive = false;
                    let group = usize::from(fighter.fighter_group);
                    if fighter.is_attacker == player_is_attacker
                        && group < transitioned_groups.len()
                        && !transitioned_groups[group]
                    {
                        transitioned_groups[group] = true;
                    }
                }
                TacticalFighterRecoveryState::AwaitingCarrier
                | TacticalFighterRecoveryState::Reserved
                | TacticalFighterRecoveryState::Recovered => {}
            }
        }
        let completed_groups = transitioned_groups
            .into_iter()
            .enumerate()
            .filter_map(|(group, transitioned)| {
                let group_still_active = self.fighters.iter().any(|fighter| {
                    fighter.alive
                        && fighter.is_attacker == player_is_attacker
                        && usize::from(fighter.fighter_group) == group
                });
                (transitioned && !group_still_active).then_some(group as u8)
            })
            .map(|group| TacticalVoiceCue::fighter_recovery_complete(player_faction, group));
        self.pending_voice_cues.extend(completed_groups);
    }

    /// Expand a fleet's composition into individual TacticalShip/TacticalFighter entries.
    fn expand_fleet(
        world: &GameWorld,
        fleet_key: FleetKey,
        is_attacker: bool,
        ships: &mut Vec<TacticalShip>,
        fighters: &mut Vec<TacticalFighter>,
    ) {
        let fleet = &world.fleets[fleet_key];
        let first_fleet_ship = ships.len();
        // Mode 1 (`FUN_005001b0(fleet, 1)`, `FUN_0040b0e0.c:164`): a ship
        // still on its own leg from its yard stays out.
        for (ship_idx, ship) in fleet
            .capital_ships
            .iter()
            .enumerate()
            .filter(|(_, ship)| ship.usable())
        {
            let class = &world.capital_ship_classes[ship.class];
            // The fleet loop exports a Death Star ship as the station's short
            // record only (`FUN_0040b0e0.c:86-118`; `TacticalDeathStar`).
            if class.is_death_star() {
                continue;
            }
            let tactical_resource = capital_ship_tactical_resource(class.dat_id);

            let turbolaser_total = (class.turbolaser_fore
                + class.turbolaser_aft
                + class.turbolaser_port
                + class.turbolaser_starboard)
                .cast_signed();
            let ion_cannon_total = (class.ion_cannon_fore
                + class.ion_cannon_aft
                + class.ion_cannon_port
                + class.ion_cannon_starboard)
                .cast_signed();
            let laser_cannon_total = (class.laser_cannon_fore
                + class.laser_cannon_aft
                + class.laser_cannon_port
                + class.laser_cannon_starboard)
                .cast_signed();
            let weapon_arcs = [
                TacticalWeaponArc::new(
                    class.laser_cannon_fore,
                    class.ion_cannon_fore,
                    class.turbolaser_fore,
                ),
                TacticalWeaponArc::new(
                    class.laser_cannon_starboard,
                    class.ion_cannon_starboard,
                    class.turbolaser_starboard,
                ),
                TacticalWeaponArc::new(
                    class.laser_cannon_aft,
                    class.ion_cannon_aft,
                    class.turbolaser_aft,
                ),
                TacticalWeaponArc::new(
                    class.laser_cannon_port,
                    class.ion_cannon_port,
                    class.turbolaser_port,
                ),
            ];
            let subsystem_capacity = TacticalSubsystemCapacity {
                shields: class.shield_strength > 0,
                weapons: turbolaser_total + ion_cannon_total + laser_cannon_total > 0,
                tractor: class.tractor_beam_power > 0,
                engines: class.sub_light_engine > 0,
                hyperdrive: u8::from(class.hyperdrive > 0)
                    + u8::from(class.hyperdrive_if_damaged > 0),
            };
            // hyp: a battle starts from the counters the strategic ship
            // carries, as `FUN_0040cfc0` writes them back whole
            // (`ship-damage-repair.md`); the shield, weapon, tractor and
            // engine counters are strategic nibbles 0-3.
            let subsystem_damage = TacticalSubsystemDamage {
                shields: ship.damage_counter(0),
                weapons: ship.damage_counter(1),
                tractor: ship.damage_counter(2),
                engines: ship.damage_counter(3),
                hyperdrive: ship.hyperdrive_damage_hits(class),
            };
            ships.push(TacticalShip {
                class_key: ship.class,
                identity: TacticalObjectIdentity {
                    class_dat_id: class.dat_id,
                    fleet_roster_index: ship_idx,
                    is_alliance: fleet.is_alliance,
                },
                tactical_resource,
                source_position: TacticalWorldPosition::ORIGIN,
                source_forward: TacticalWorldVector::SOURCE_FORWARD,
                source_desired_forward: TacticalWorldVector::ZERO,
                source_velocity: TacticalWorldVector::ZERO,
                source_waypoint: None,
                navigation_route: Vec::new(),
                source_collision_envelope: None,
                // FUN_004f62d0: the ship's own name, else its class's.
                name: ship.name.clone().unwrap_or_else(|| class.name.clone()),
                x: 0.0,
                y: 0.0,
                hull_current: ship.hull_current,
                hull_max: class.hull.cast_signed(),
                shield: class.shield_strength.cast_signed(),
                shield_max: class.shield_strength.cast_signed(),
                subsystem_condition: TacticalSubsystemCondition::from_source_state(
                    ship.hull_current,
                    class.hull.cast_signed(),
                    subsystem_capacity,
                    subsystem_damage,
                ),
                subsystem_capacity,
                subsystem_damage,
                sublight_engine_power: class.sub_light_engine as f32
                    * ORIGINAL_SUBLIGHT_ENGINE_SCALE,
                maneuverability: class.maneuverability,
                tractor_beam_power: class.tractor_beam_power as f32,
                fighter_capacity: class.fighter_capacity,
                maneuver_state_value: None,
                damage_control: u8::try_from(class.damage_control.min(100)).unwrap_or(100),
                is_attacker,
                alive: true,
                selected: false,
                order: TacticalOrder::None,
                tactic: TacticalTactic::StandOff,
                // Source initialization of the first task force is known;
                // automatic distribution across later slots is not yet.
                task_force: 0,
                fleet: Some(fleet_key),
                fleet_ship_index: ship_idx,
                // IDs 2001 through 2130 are not a linear class sprite table.
                // Keep the legacy selected-panel slot empty until its exact
                // bitmap role is independently recovered.
                sprite_id: None,
                weapon_arcs,
                weapon_ranges: TacticalWeaponRanges {
                    laser_cannon: class.laser_cannon_range as f32,
                    ion_cannon: class.ion_cannon_range as f32,
                    turbolaser: class.turbolaser_range as f32,
                },
                weapon_recharge_rate: class.weapon_recharge_rate as f32
                    * ORIGINAL_WEAPON_RECHARGE_SCALE,
                shield_recharge_rate: class.shield_recharge_rate as f32,
                shield_recharge_carry: 0.0,
                weapon_recharge_queue: Vec::new(),
                attack_target: None,
                manual_targets: Vec::new(),
                escort_target: None,
                contents: Vec::new(),
                retreating: false,
                retreat_progress: 0.0,
                retreated: false,
                outstanding_hits: u32::from(subsystem_damage.shields)
                    + u32::from(subsystem_damage.tractor)
                    + u32::from(subsystem_damage.engines)
                    + u32::from(subsystem_damage.hyperdrive),
                repair_timer_steps: 0,
            });
        }

        // The strategic model currently retains personnel at fleet granularity.
        // Bind those entries to the fleet's first surviving capital hull, the
        // deterministic command-ship slot, so the original 1302 contents
        // monitor never invents a separate replacement panel.
        if let Some(command_ship) = ships.get_mut(first_fleet_ship) {
            command_ship
                .contents
                .extend(fleet.characters.iter().filter_map(|key| {
                    let character = world.characters.get(*key)?;
                    if character.is_killed {
                        return None;
                    }
                    let resource_id =
                        tactical_character_content_resource(character.dat_id, character.is_major)?;
                    Some(TacticalShipContent {
                        label: character.name.clone(),
                        resource_id,
                    })
                }));
        }

        for (fighter_idx, entry) in fleet.fighters.iter().enumerate() {
            Self::push_squadrons(
                world,
                entry.class,
                entry.count,
                fighter_idx,
                fleet.is_alliance,
                is_attacker,
                Some(fleet_key),
                fighters,
            );
        }
    }

    /// Push `count` squadrons of `class` as fighters, each tagged with its
    /// roster `index` and its place in that entry, carried by `fleet`, or
    /// held by the battle system when `fleet` is `None`.
    #[expect(
        clippy::cast_precision_loss,
        clippy::too_many_arguments,
        reason = "Retain the existing tactical float conversions; each input names one squadron fact."
    )]
    fn push_squadrons(
        world: &GameWorld,
        class_key: FighterKey,
        count: u32,
        index: usize,
        is_alliance: bool,
        is_attacker: bool,
        fleet: Option<FleetKey>,
        fighters: &mut Vec<TacticalFighter>,
    ) {
        let held_by_system = fleet.is_none();
        let class = &world.fighter_classes[class_key];
        for squadron_index in 0..count {
            let squad_count = class.squadron_size;
            let hull = squad_count as f32 * ORIGINAL_FIGHTER_HULL_PER_CRAFT;
            fighters.push(TacticalFighter {
                class_key,
                identity: TacticalObjectIdentity {
                    class_dat_id: class.dat_id,
                    fleet_roster_index: index,
                    is_alliance,
                },
                tactical_resource: fighter_tactical_resource(class.dat_id),
                source_position: TacticalWorldPosition::ORIGIN,
                sublight_engine_power: class.sub_light_engine as f32
                    * ORIGINAL_FIGHTER_SUBLIGHT_ENGINE_SCALE,
                fleet,
                fleet_fighter_index: index,
                fleet_squadron_index: squadron_index,
                name: class.name.clone(),
                x: 0.0,
                y: 0.0,
                squad_count,
                hull_current: hull,
                hull_max: hull,
                shield: class.shield_strength as f32,
                shield_max: class.shield_strength as f32,
                weapon_arc: TacticalWeaponArc::new(
                    class.laser_cannon_fore,
                    class.ion_cannon_fore,
                    class.turbolaser_fore,
                ),
                weapon_ranges: TacticalWeaponRanges {
                    laser_cannon: class.laser_cannon_range as f32,
                    ion_cannon: class.ion_cannon_range as f32,
                    turbolaser: class.turbolaser_range as f32,
                },
                torpedo_strength: class.torpedoes * squad_count,
                torpedo_range: class.torpedoes_range as f32,
                maneuverability: class.maneuverability,
                is_attacker,
                alive: squad_count > 0,
                // FUN_005c63f0 initializes color state to 4 (white/unassigned).
                fighter_group: 4,
                selected: false,
                off_field: false,
                carrier: None,
                docked: None,
                order: TacticalOrder::None,
                tactic: TacticalTactic::StandOff,
                attack_target: None,
                escort_target: None,
                recovery_state: TacticalFighterRecoveryState::AwaitingCarrier,
                recovery_target: None,
                held_by_system,
            });
        }
    }

    /// `FUN_005964c0.c:133-219`: a fleet's squadron takes its carrier
    /// (`+0x164`), resolved by `FUN_005a6360`: its capital ship in the
    /// battle, else the station's hangar when there is a station (any side's
    /// squadron, `FUN_005a6360.c:15-31`). Without a working hyperdrive
    /// (`FUN_005b1b70`) it starts docked, ready (state 1, `FUN_005b69d0`),
    /// in that hangar's launch list while the hangar has room
    /// (`FUN_005b8570.c:13`, reserved `+0xc` below capacity `+0x10`); with no
    /// room it stays in state 1 outside the list and never launches. A
    /// squadron with a hyperdrive, or held by the system, starts in flight
    /// (`FUN_005b69b0`, state 2).
    /// port: a full hangar's squadron is kept as recovered, which never
    /// launches.
    fn seat_carried_squadrons(
        world: &GameWorld,
        ships: &[TacticalShip],
        fighters: &mut [TacticalFighter],
        hyperdrive_squadrons: &[usize],
        station_capacity: Option<u32>,
    ) {
        let mut reserved = vec![0_u32; ships.len()];
        let mut station_reserved = 0_u32;
        for (index, fighter) in fighters.iter_mut().enumerate() {
            let Some(fleet_key) = fighter.fleet else {
                continue;
            };
            let Some(fleet) = world.fleets.get(fleet_key) else {
                continue;
            };
            let tag = fleet
                .fighters
                .get(fighter.fleet_fighter_index)
                .map_or(0, |entry| entry.carrier);
            let ship = ships.iter().position(|ship| {
                ship.fleet == Some(fleet_key)
                    && fleet
                        .capital_ships
                        .get(ship.fleet_ship_index)
                        .is_some_and(|instance| instance.tag == tag)
            });
            let carrier = match (ship, station_capacity) {
                (Some(ship), _) => TacticalBerth::Ship(ship),
                (None, Some(_)) => TacticalBerth::Station,
                (None, None) => continue,
            };
            fighter.carrier = Some(carrier);
            if hyperdrive_squadrons.contains(&index) {
                continue;
            }
            let (count, capacity) = match carrier {
                TacticalBerth::Ship(ship) => (&mut reserved[ship], ships[ship].fighter_capacity),
                TacticalBerth::Station => (&mut station_reserved, station_capacity.unwrap_or(0)),
            };
            fighter.docked = Some(if *count < capacity {
                *count += 1;
                TacticalDock::Ready
            } else {
                TacticalDock::Recovered
            });
            // port: as a recovered squadron, a docked one is out of the
            // battle's live units until it launches.
            fighter.alive = false;
        }
    }

    /// Seat the squadrons `system` holds for a side as fighters that need no
    /// carrier (`FUN_0040b0e0`, `ghidra/notes/system-squadrons-combat.md`).
    /// hyp: they take their side's fighter line, as a fleet's squadrons do;
    /// where the original places them is untraced.
    fn expand_system_squadrons(
        world: &GameWorld,
        system: SystemKey,
        is_alliance: bool,
        is_attacker: bool,
        fighters: &mut Vec<TacticalFighter>,
    ) {
        let Some(value) = world.systems.get(system) else {
            return;
        };
        let held = value
            .fighters
            .iter()
            .filter(|entry| entry.is_alliance == is_alliance)
            .enumerate();
        for (held_index, entry) in held {
            Self::push_squadrons(
                world,
                entry.class,
                entry.count,
                held_index,
                is_alliance,
                is_attacker,
                None,
                fighters,
            );
        }
    }

    /// Bind every active battle participant to the source X-slot and Z-lane
    /// initialization performed by `FUN_005ab650` through `FUN_005a9030`.
    /// A docked squadron (not `alive` until it launches) joins no placement
    /// list and no count of the battle extent (`FUN_005ab650.c:148-153`,
    /// `:196-210`, `FUN_005b69f0`).
    fn assign_original_world_positions(
        ships: &mut [TacticalShip],
        fighters: &mut [TacticalFighter],
    ) -> OriginalTacticalLayout {
        let active_count = |is_alliance| {
            ships
                .iter()
                .filter(|ship| ship.alive && ship.identity.is_alliance == is_alliance)
                .count()
                + fighters
                    .iter()
                    .filter(|fighter| {
                        fighter.alive
                            && fighter.squad_count > 0
                            && fighter.identity.is_alliance == is_alliance
                    })
                    .count()
        };
        // Original tactical side zero is the Alliance collection at +0x08;
        // side one is the Imperial collection at +0x0c.
        let layout =
            OriginalTacticalLayout::from_active_counts(active_count(true), active_count(false));

        fn place_ships(ships: &mut [TacticalShip], is_alliance: bool, z: f32) {
            let mut slots = OriginalTacticalSlots::default();
            for ship in ships
                .iter_mut()
                .filter(|ship| ship.alive && ship.identity.is_alliance == is_alliance)
            {
                ship.source_position = TacticalWorldPosition {
                    x: slots.next().unwrap_or(0.0),
                    y: 0.0,
                    z,
                };
            }
        }

        fn place_fighters(fighters: &mut [TacticalFighter], is_alliance: bool, z: f32) {
            let mut slots = OriginalTacticalSlots::default();
            for fighter in fighters.iter_mut().filter(|fighter| {
                fighter.alive
                    && fighter.squad_count > 0
                    && fighter.identity.is_alliance == is_alliance
            }) {
                fighter.source_position = TacticalWorldPosition {
                    x: slots.next().unwrap_or(0.0),
                    y: 0.0,
                    z,
                };
            }
        }

        place_ships(ships, true, layout.outer_negative_z);
        place_ships(ships, false, layout.outer_positive_z);
        place_fighters(fighters, true, layout.inner_negative_z);
        place_fighters(fighters, false, layout.inner_positive_z);
        layout
    }

    /// Reproduce `FUN_005ae460`'s class-first red, blue, green, and gold
    /// fighter assignment independently for the Alliance and Empire. Empty
    /// colors split the first half of the largest group only when it contains
    /// more than seven squadrons. It runs at setup, where every squadron
    /// is in flight or docked, and a docked one is grouped too: the counts
    /// and `FUN_005adfa0` walk every object (`mgr+0x9a0`) with no docked
    /// test (`FUN_005ab650.c:297-320`, `:383-406`,
    /// `FUN_005adfa0.c:22-26`).
    fn assign_original_fighter_groups(fighters: &mut [TacticalFighter]) {
        for is_alliance in [true, false] {
            let mut groups: [Vec<usize>; 4] = std::array::from_fn(|_| Vec::new());
            for (index, fighter) in fighters.iter().enumerate().filter(|(_, fighter)| {
                fighter.squad_count > 0 && fighter.identity.is_alliance == is_alliance
            }) {
                let Some(group) = fighter.tactical_resource.and_then(|resource| {
                    let base = if resource.tactical_ordinal < 33 {
                        29
                    } else {
                        33
                    };
                    resource
                        .tactical_ordinal
                        .checked_sub(base)
                        .filter(|group| *group < 4)
                }) else {
                    continue;
                };
                groups[usize::from(group)].push(index);
            }

            for empty in 0..groups.len() {
                if !groups[empty].is_empty() {
                    continue;
                }
                let donor = groups
                    .iter()
                    .enumerate()
                    .max_by_key(|(index, members)| (members.len(), std::cmp::Reverse(*index)))
                    .map(|(index, _)| index)
                    .unwrap_or(0);
                if groups[donor].len() <= 7 {
                    continue;
                }
                let split = groups[donor].len() / 2;
                let moved = groups[donor].drain(..split).collect::<Vec<_>>();
                groups[empty] = moved;
            }

            for (group, members) in groups.iter().enumerate() {
                for &index in members {
                    fighters[index].fighter_group = u8::try_from(group).unwrap_or(3);
                }
            }
        }
    }

    /// Reproduce `FUN_005c81d0` through `FUN_005c83c0`'s three source lists,
    /// exact capability ordering, square-root leader count, stable follower
    /// ownership, and eight leader slots.
    fn assign_original_task_forces(ships: &mut [TacticalShip]) {
        fn category(ship: &TacticalShip) -> usize {
            let has_lasers = ship.weapon_arcs.iter().any(|arc| arc.laser_cannons > 0);
            let has_weapons = ship
                .weapon_arcs
                .iter()
                .any(|arc| arc.laser_cannons > 0 || arc.ion_cannons > 0 || arc.turbolasers > 0);
            if !has_weapons {
                2
            } else if has_lasers {
                1
            } else {
                0
            }
        }

        fn leader_count(members: usize, available_slots: usize) -> usize {
            if members == 0 || available_slots == 0 {
                return 0;
            }
            let root = if members < 2 {
                1
            } else {
                (members as f32).sqrt().floor() as usize
            };
            root.max(1).min(available_slots)
        }

        fn leader_y(slot: u8) -> f32 {
            match slot {
                0 => 0.0,
                1 => 8.0,
                2 => -8.0,
                3 => 16.0,
                4 => -16.0,
                5 => 24.0,
                6 => -24.0,
                _ => 32.0,
            }
        }

        for is_alliance in [true, false] {
            let mut categories: [Vec<usize>; 3] = std::array::from_fn(|_| Vec::new());
            for (index, ship) in ships
                .iter()
                .enumerate()
                .filter(|(_, ship)| ship.alive && ship.identity.is_alliance == is_alliance)
            {
                categories[category(ship)].push(index);
            }
            for members in &mut categories {
                members.sort_by(|left, right| {
                    original_formation_capability_score(&ships[*right])
                        .total_cmp(&original_formation_capability_score(&ships[*left]))
                });
            }

            let mut next_slot = 0_usize;
            for (category_index, members) in categories.iter().enumerate() {
                let category_end = match category_index {
                    0 => 3,
                    1 => (next_slot + 3).min(7),
                    _ => 7,
                };
                let available = category_end.saturating_sub(next_slot) + 1;
                let leaders = leader_count(members.len(), available);
                if leaders == 0 {
                    continue;
                }
                let stride = members.len() / leaders;
                let mut leader_ordinal = 0_usize;
                let mut active_slot = next_slot;
                for (member_ordinal, &ship_index) in members.iter().enumerate() {
                    if member_ordinal % stride == 0 && leader_ordinal < leaders {
                        active_slot = next_slot + leader_ordinal;
                        let slot = u8::try_from(active_slot).unwrap_or(7).min(7);
                        ships[ship_index].source_position.y = leader_y(slot);
                        leader_ordinal += 1;
                    }
                    ships[ship_index].task_force = u8::try_from(active_slot).unwrap_or(7).min(7);
                }
                next_slot = (next_slot + leaders).min(8);
                if next_slot >= 8 {
                    break;
                }
            }
        }
        Self::refresh_original_task_force_geometry(ships);
    }

    /// Apply `FUN_005ea250`, `FUN_005ec400`, and `FUN_005cbbc0`'s retained
    /// follower placement to the current task-force ownership. Before decoded
    /// mesh footprints arrive, the recovered five-unit source setup spacing is
    /// used. A later asset report expands the group to the largest 1.2-scaled
    /// close-mesh footprint without changing its source-selected leader. The
    /// lateral axis follows the leader's current retained-mode forward vector,
    /// so the formation remains attached while the leader turns and moves.
    pub fn refresh_original_task_force_geometry(ships: &mut [TacticalShip]) {
        for is_alliance in [true, false] {
            for task_force in 0_u8..8 {
                let mut members = ships
                    .iter()
                    .enumerate()
                    .filter(|(_, ship)| {
                        ship.alive
                            && ship.identity.is_alliance == is_alliance
                            && ship.task_force == task_force
                    })
                    .map(|(index, _)| index)
                    .collect::<Vec<_>>();
                if members.is_empty() {
                    continue;
                }
                members.sort_by(|left, right| {
                    original_formation_capability_score(&ships[*right])
                        .total_cmp(&original_formation_capability_score(&ships[*left]))
                });
                let leader = members[0];
                let anchor = ships[leader].source_position;
                let forward = ships[leader].source_forward.normalized();
                let lateral = if forward == TacticalWorldVector::ZERO {
                    TacticalWorldVector {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    }
                } else {
                    // `FUN_005ec400` derives a horizontal perpendicular from
                    // the leader direction. This is the equivalent vector in
                    // the reconstruction's retained left-handed coordinates.
                    TacticalWorldVector {
                        x: forward.z,
                        y: 0.0,
                        z: -forward.x,
                    }
                    .normalized()
                };
                let spacing = members
                    .iter()
                    .filter_map(|index| ships[*index].source_collision_envelope)
                    .map(|envelope| {
                        (envelope.planar_diameter * ORIGINAL_FORMATION_FOOTPRINT_SCALE).trunc()
                    })
                    .fold(ORIGINAL_BATTLE_SLOT_SPACING, f32::max);
                for (member_ordinal, index) in members.into_iter().enumerate() {
                    if member_ordinal == 0 {
                        // Retain the source slot's signed zero and every other
                        // exact anchor component without a `+ 0.0` rewrite.
                        ships[index].source_position = anchor;
                        continue;
                    }
                    let offset = original_formation_lateral_offset(member_ordinal, spacing);
                    ships[index].source_position = TacticalWorldPosition {
                        x: anchor.x + lateral.x * offset,
                        y: anchor.y,
                        z: anchor.z + lateral.z * offset,
                    };
                }
            }
        }
    }

    /// Auto-place ships in their respective deployment zones.
    ///
    /// Attacker ships go on the left side, defender ships on the right.
    /// Ships are stacked vertically, centered. port: the flat 2D layout; the
    /// source placement sets `source_position`.
    #[expect(
        clippy::cast_precision_loss,
        reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
    )]
    fn auto_place_ships(ships: &mut [TacticalShip]) {
        let atk_ships: Vec<usize> = ships
            .iter()
            .enumerate()
            .filter(|(_, s)| s.is_attacker)
            .map(|(i, _)| i)
            .collect();
        let def_ships: Vec<usize> = ships
            .iter()
            .enumerate()
            .filter(|(_, s)| !s.is_attacker)
            .map(|(i, _)| i)
            .collect();

        // Attacker: left deployment zone.
        let atk_x = ARENA_WIDTH * DEPLOY_ZONE_FRACTION * 0.5;
        let atk_start_y = (ARENA_HEIGHT - (atk_ships.len() as f32 * SHIP_SPACING)) / 2.0;
        for (row, &idx) in atk_ships.iter().enumerate() {
            ships[idx].x = atk_x;
            ships[idx].y = atk_start_y + row as f32 * SHIP_SPACING;
        }

        // Defender: right deployment zone.
        let def_x = ARENA_WIDTH - ARENA_WIDTH * DEPLOY_ZONE_FRACTION * 0.5;
        let def_start_y = (ARENA_HEIGHT - (def_ships.len() as f32 * SHIP_SPACING)) / 2.0;
        for (row, &idx) in def_ships.iter().enumerate() {
            ships[idx].x = def_x;
            ships[idx].y = def_start_y + row as f32 * SHIP_SPACING;
        }
    }

    /// Auto-place fighter squadrons near their side's capital ships.
    /// port: the flat 2D layout's offsets and row spacing.
    #[expect(
        clippy::cast_precision_loss,
        reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
    )]
    fn auto_place_fighters(fighters: &mut [TacticalFighter], ships: &[TacticalShip]) {
        // Find average Y position for each side's capital ships.
        let atk_center = Self::side_center(ships, true);
        let def_center = Self::side_center(ships, false);

        let atk_fighters: Vec<usize> = fighters
            .iter()
            .enumerate()
            .filter(|(_, f)| f.is_attacker)
            .map(|(i, _)| i)
            .collect();
        let def_fighters: Vec<usize> = fighters
            .iter()
            .enumerate()
            .filter(|(_, f)| !f.is_attacker)
            .map(|(i, _)| i)
            .collect();

        // Attacker fighters: slightly ahead (right) of capital ships.
        let atk_x = ARENA_WIDTH * DEPLOY_ZONE_FRACTION * 0.5 + 80.0;
        let atk_start_y = atk_center - (atk_fighters.len() as f32 * 24.0) / 2.0;
        for (row, &idx) in atk_fighters.iter().enumerate() {
            fighters[idx].x = atk_x;
            fighters[idx].y = atk_start_y + row as f32 * 24.0;
        }

        // Defender fighters: slightly ahead (left) of capital ships.
        let def_x = ARENA_WIDTH - ARENA_WIDTH * DEPLOY_ZONE_FRACTION * 0.5 - 80.0;
        let def_start_y = def_center - (def_fighters.len() as f32 * 24.0) / 2.0;
        for (row, &idx) in def_fighters.iter().enumerate() {
            fighters[idx].x = def_x;
            fighters[idx].y = def_start_y + row as f32 * 24.0;
        }
    }

    #[expect(
        clippy::cast_precision_loss,
        reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
    )]
    /// port: the flat 2D layout's vertical centre for one side.
    fn side_center(ships: &[TacticalShip], is_attacker: bool) -> f32 {
        let side: Vec<f32> = ships
            .iter()
            .filter(|s| s.is_attacker == is_attacker)
            .map(|s| s.y)
            .collect();
        if side.is_empty() {
            ARENA_HEIGHT / 2.0
        } else {
            side.iter().sum::<f32>() / side.len() as f32
        }
    }

    /// Returns true if the player controls this side of the battle.
    pub fn player_ships(&self) -> impl Iterator<Item = (usize, &TacticalShip)> {
        let player_is_atk = self.player_is_attacker;
        self.ships
            .iter()
            .enumerate()
            .filter(move |(_, s)| s.is_attacker == player_is_atk)
    }

    /// Returns true if this side is controlled by the AI.
    pub fn ai_ships(&self) -> impl Iterator<Item = (usize, &TacticalShip)> {
        let player_is_atk = self.player_is_attacker;
        self.ships
            .iter()
            .enumerate()
            .filter(move |(_, s)| s.is_attacker != player_is_atk)
    }

    // -------------------------------------------------------------------
    // Combat step — one tick of real-time combat
    // -------------------------------------------------------------------

    /// Advances projectiles, impact frames (`FUN_005d41a0`) and field frames,
    /// which `FUN_005d3cc0` cycles through eight at 0.1 s each.
    pub fn advance_presentational_effects(&mut self, elapsed: f32) {
        if !elapsed.is_finite() || elapsed <= 0.0 {
            return;
        }
        self.weapon_effects.retain_mut(|effect| {
            effect.elapsed += elapsed;
            effect.elapsed < effect.duration
        });
        self.impact_effects.retain_mut(|effect| {
            effect.frame_elapsed += elapsed;
            while effect.frame_elapsed >= ORIGINAL_TACTICAL_EFFECT_FRAME_SECONDS {
                effect.frame_elapsed -= ORIGINAL_TACTICAL_EFFECT_FRAME_SECONDS;
                effect.frame = effect.frame.saturating_add(1);
            }
            effect.frame < effect.sequence.frame_count()
        });
        for effect in &mut self.field_effects {
            effect.frame_elapsed += elapsed;
            while effect.frame_elapsed >= ORIGINAL_TACTICAL_EFFECT_FRAME_SECONDS {
                effect.frame_elapsed -= ORIGINAL_TACTICAL_EFFECT_FRAME_SECONDS;
                effect.frame = (effect.frame + 1) % 8;
            }
        }
        if let Some(beam) = self.death_star_beam.as_mut() {
            beam.elapsed += elapsed;
            if beam.elapsed >= beam.duration {
                let beam = self.death_star_beam.take().unwrap();
                if let Some(target) = self.ships.get_mut(beam.target) {
                    target.hull_current = 0;
                    target.alive = false;
                    target.selected = false;
                    target.attack_target = None;
                    queue_original_tactical_impact(
                        &mut self.impact_effects,
                        beam.target,
                        WeaponKind::Turbolaser,
                        OriginalTacticalImpactStage::Destroyed,
                    );
                }
                self.pending_log_lines.push(format!(
                    "[tactical_death_star] superlaser_resolved target_object_id={}",
                    beam.target + 1
                ));
            }
        }
    }

    pub fn commit_death_star_shot(&mut self, target: usize) -> bool {
        self.commit_death_star_shot_for(self.player_is_attacker, target)
    }

    /// The side `is_attacker`'s station fires at the capital ship `target`
    /// (`FUN_005ba720`): it must be operational and fully charged, the
    /// target an enemy ship still fighting, and no beam in flight.
    fn commit_death_star_shot_for(&mut self, is_attacker: bool, target: usize) -> bool {
        if self.death_star_beam.is_some()
            || self
                .ships
                .get(target)
                .is_none_or(|ship| !ship.alive || ship.is_attacker == is_attacker)
        {
            return false;
        }
        let Some(death_star) = self.death_star.as_mut().filter(|death_star| {
            death_star.is_attacker == is_attacker
                && death_star.operational()
                && death_star.laser_charge >= ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE
        }) else {
            return false;
        };
        let target_position = self.ships[target].source_position;
        // `FUN_005ba720` zeroes the charge and posts message 0x44; the
        // shot (`FUN_005ba520`) leaves `+0x78` alone, which only the
        // departure `FUN_005ba5e0` sets.
        death_star.laser_charge = 0.0;
        self.death_star_beam = Some(TacticalDeathStarBeam {
            target,
            source_position: death_star.superlaser_source(self.source_layout.battle_extent),
            target_position,
            elapsed: 0.0,
            duration: ORIGINAL_PROJECTILE_DURATION_SHORT,
            color_selector: u8::from(!death_star.is_alliance),
        });
        // `FUN_005a5740`, the player's targeting click, queues 0x111.
        if is_attacker == self.player_is_attacker {
            self.pending_voice_cues
                .push(TacticalVoiceCue::death_star_firing());
        }
        // `FUN_005ba520.c:23-31`, whoever fired: event 0x14 (TACTICAL.DLL
        // WAVE 13054) for either local side, and for a local Alliance
        // (`battle+0x9ec == 0`) its warning 0x91.
        self.pending_audio_cues
            .push(TacticalAudioCue::from_variant(TacticalAudioEvent::TorpedoImpact, 0));
        if self.player_voice_faction() == TacticalVoiceFaction::Alliance {
            self.pending_voice_cues
                .push(TacticalVoiceCue::death_star_fire_warning());
        }
        self.pending_log_lines.push(format!(
            "[tactical_death_star] superlaser_committed target_object_id={} charge=0",
            target + 1
        ));
        true
    }

    /// Advance combat by one tick. Applies weapon fire, shield regen, hull
    /// damage, and fighter engagement. Produces weapon effects for rendering.
    ///
    /// Returns `true` if the battle has ended this tick.
    pub fn step(&mut self) -> bool {
        if self.phase != BattlePhase::Combat || self.paused {
            return false;
        }

        self.combat_tick += 1;
        self.subsystem_repairs.clear();

        self.advance_side_withdrawals(ORIGINAL_TACTICAL_STEP_MILLISECONDS);
        refresh_original_escort_targets(self);
        advance_original_escort_movement(self, ORIGINAL_TACTICAL_STEP_MILLISECONDS);
        advance_original_engagement(self, ORIGINAL_TACTICAL_STEP_MILLISECONDS);
        self.advance_original_tactical_movement(ORIGINAL_TACTICAL_STEP_MILLISECONDS);
        self.advance_original_fighter_recovery(ORIGINAL_TACTICAL_STEP_MILLISECONDS);
        refresh_original_attack_targets(self);
        for ship in &mut self.ships {
            ship.advance_original_recharge(ORIGINAL_TACTICAL_STEP_SECONDS);
        }
        for fighter in &mut self.fighters {
            fighter.advance_original_recharge(ORIGINAL_TACTICAL_STEP_SECONDS);
        }
        let death_star_was_ready = self.death_star.is_some_and(|death_star| {
            death_star.is_attacker == self.player_is_attacker
                && death_star.laser_charge >= ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE
        });
        if let Some(death_star) = self.death_star.as_mut() {
            death_star.advance_laser_charge(ORIGINAL_TACTICAL_STEP_SECONDS);
        }
        if !death_star_was_ready
            && self.death_star.is_some_and(|death_star| {
                death_star.is_attacker == self.player_is_attacker
                    && death_star.laser_charge >= ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE
            })
        {
            self.pending_voice_cues
                .push(TacticalVoiceCue::death_star_ready());
        }

        // Collect alive (non-retreating) ship indices per side for combat.
        let atk_alive: Vec<usize> = self
            .ships
            .iter()
            .enumerate()
            .filter(|(_, s)| s.is_attacker && s.alive && !s.retreating)
            .map(|(i, _)| i)
            .collect();
        let def_alive: Vec<usize> = self
            .ships
            .iter()
            .enumerate()
            .filter(|(_, s)| !s.is_attacker && s.alive && !s.retreating)
            .map(|(i, _)| i)
            .collect();

        // Phase: Weapon fire (each ship fires at one random enemy).
        let mut new_effects = Vec::new();
        let player_is_attacker = self.player_is_attacker;
        let player_faction = self.player_voice_faction();
        Self::fire_side(
            &mut self.ships,
            &mut self.field_effects,
            &atk_alive,
            &def_alive,
            &mut new_effects,
            &mut self.impact_effects,
            &mut self.pending_audio_cues,
            &mut self.pending_voice_cues,
            player_is_attacker,
            player_faction,
            &mut self.tactical_rng,
        );
        Self::fire_side(
            &mut self.ships,
            &mut self.field_effects,
            &def_alive,
            &atk_alive,
            &mut new_effects,
            &mut self.impact_effects,
            &mut self.pending_audio_cues,
            &mut self.pending_voice_cues,
            player_is_attacker,
            player_faction,
            &mut self.tactical_rng,
        );
        self.fire_capitals_at_fighters(&mut new_effects);
        self.weapon_effects.extend(new_effects);

        self.advance_subsystem_repairs();

        // Fighter batteries use the same 250 ms source cadence as capital
        // batteries. Their own readiness and recharge record gates each shot.
        self.fighter_step();
        self.advance_computer_controller();
        self.advance_trench_run_approach(ORIGINAL_TACTICAL_STEP_MILLISECONDS);
        self.advance_original_trench_run(ORIGINAL_TACTICAL_STEP_MILLISECONDS);
        self.advance_hangars(ORIGINAL_TACTICAL_STEP_MILLISECONDS);
        self.destroy_squadrons_docked_in_lost_ships();

        if self.finish_if_decided() {
            return true;
        }

        self.advance_stalemate_watch(ORIGINAL_TACTICAL_STEP_SECONDS)
    }

    /// The played frame (`FUN_005bea30.c:121`) runs the unit pass (battle
    /// slot `+0x88`, `FUN_005a9de0`) once per 1000 ms turn; for each live
    /// capital ship it calls `FUN_005b0330` (slot `+0x9c`), whose repair
    /// timer (`+0x3f4`) counts down only for a unit without `+0x124`
    /// (`FUN_005c0170` sets it on every unit not of the player's side) and
    /// with outstanding hits (`+0x158`) (`FUN_005b0330.c:19-24`). At 0 it is
    /// reset to 50 s and `FUN_005b1490` rolls; both draws advance the one
    /// recovered tactical manager stream. An unplayed battle repairs nothing
    /// (`ghidra/notes/tactical-engagement-ai.md`, "Subsystem repair").
    fn advance_subsystem_repairs(&mut self) {
        if !self.combat_tick.is_multiple_of(ORIGINAL_STEPS_PER_SECOND) {
            return;
        }
        let player = self.player_is_attacker;
        for ship_index in 0..self.ships.len() {
            let ship = &mut self.ships[ship_index];
            if ship.is_attacker != player || !ship.alive || ship.outstanding_hits == 0 {
                continue;
            }
            ship.repair_timer_steps = ship.repair_timer_steps.saturating_sub(ORIGINAL_STEPS_PER_SECOND);
            if ship.repair_timer_steps > 0 {
                continue;
            }
            ship.repair_timer_steps = ORIGINAL_SUBSYSTEM_REPAIR_INTERVAL_TICKS;
            // FUN_005b1490: no draw without damage control (+0x3f0); the
            // hit is drawn among the outstanding ones only when the
            // chance passes and two or more are outstanding.
            if ship.damage_control == 0 {
                continue;
            }
            let chance_roll = u8::try_from(self.tactical_rng.inclusive(1, 100)).unwrap_or(100);
            if chance_roll > ship.damage_control {
                continue;
            }
            let outstanding = ship.outstanding_hits;
            let selection_roll = if outstanding < 2 {
                1
            } else {
                u16::try_from(self.tactical_rng.inclusive(1, outstanding)).unwrap_or(u16::MAX)
            };
            if let Some(repair) =
                attempt_original_subsystem_repair(ship, ship_index, chance_roll, selection_roll)
            {
                ship.outstanding_hits -= 1;
                self.subsystem_repairs.push(repair);
            }
        }
        refresh_original_subsystem_conditions(&mut self.ships, &self.field_effects);
    }

    /// Developer command: put the player's `n`th living ship (1-based, in
    /// roster order) out of hyperspace, through the same subsystem hits a
    /// battle deals (a backup hyperdrive takes a second hit).
    ///
    /// # Errors
    /// Refuses a number past the player's living ships, and a ship whose
    /// hyperdrive is already out.
    pub fn knock_out_player_hyperdrive(&mut self, n: usize) -> Result<String, String> {
        let player = self.player_is_attacker;
        let ship = self
            .ships
            .iter_mut()
            .filter(|ship| ship.is_attacker == player && ship.alive)
            .nth(n.saturating_sub(1))
            .ok_or_else(|| format!("the player has no living ship {n}"))?;
        if ship.subsystem_condition.hyperdrive == 0 {
            return Err(format!("ship {n}'s hyperdrive is already out"));
        }
        while ship.subsystem_condition.hyperdrive > 0 {
            ship.add_subsystem_hit(TacticalSubsystemKind::Hyperdrive);
        }
        Ok(format!(
            "ship {n} (fleet roster {}) hyperdrive out",
            ship.fleet_ship_index
        ))
    }

    /// Whether a side still has a ship, a squadron with craft left, or its
    /// operational Death Star (`FUN_005a9ec0.c:274-281` counts the Empire
    /// empty only when `FUN_005ba420` fails).
    fn side_remains(&self, is_attacker: bool) -> bool {
        self.ships
            .iter()
            .any(|ship| ship.is_attacker == is_attacker && ship.alive)
            || self.fighters.iter().any(|fighter| {
                fighter.is_attacker == is_attacker && fighter.alive && fighter.squad_count > 0
            })
            || self.death_star.is_some_and(|death_star| {
                death_star.is_attacker == is_attacker && death_star.operational()
            })
    }

    /// The side the battle recorded as its winner (`true` Alliance), `None`
    /// for no winner: what the Battle Summary and the battle's telemetry
    /// both report.
    #[must_use]
    pub fn winner_is_alliance(&self) -> Option<bool> {
        match self.winner {
            Some(CombatWinner::Attacker) => Some(self.attacker_is_alliance),
            Some(CombatWinner::Defender) => Some(!self.attacker_is_alliance),
            Some(CombatWinner::Draw) | None => None,
        }
    }

    /// End the battle when one side (or both) has nothing left
    /// (`FUN_005a9ec0.c:274-290`).
    fn finish_if_decided(&mut self) -> bool {
        if self.side_remains(true) && self.side_remains(false) {
            return false;
        }
        let winner = self.decided_winner();
        self.queue_battle_outcome_voice();
        self.winner = Some(winner);
        self.phase = BattlePhase::Results;
        true
    }

    /// The winner `FUN_0040c780` records (`:60-101`). A side alone at
    /// nothing has lost (`FUN_005a9ec0.c:274-286`); both at nothing clears
    /// both losses (`:287-290`) and the side that withdrew wins (`:82-93`,
    /// the flags `+0x27c` and `+0x27e` that `FUN_0059a530` copies to
    /// `+0x66c`). Both withdrawn, the system's `+0x78` holder decides
    /// (`:95-101`): holder 1, the Alliance, gives the Empire; holders 2 and
    /// 0 give the Alliance. Neither withdrawn records no winner (0), and the
    /// post-battle pass falls back to the holder (`FUN_00516ef0.c:95-112`,
    /// `post_battle::pass_winner`).
    /// port: holder 3, which gives winner 3 (both sides), has no
    /// counterpart: `FUN_0050a1b0` writes an arriving fleet's or squadron's
    /// own side, 1 or 2, and `GameWorld::arrival_holders` holds one side.
    fn decided_winner(&self) -> CombatWinner {
        let alliance_wins = |alliance: bool| {
            if alliance == self.attacker_is_alliance {
                CombatWinner::Attacker
            } else {
                CombatWinner::Defender
            }
        };
        match (self.side_remains(true), self.side_remains(false)) {
            (true, false) => CombatWinner::Attacker,
            (false, true) => CombatWinner::Defender,
            (true, true) => CombatWinner::Draw,
            (false, false) => match (self.side_withdrew(true), self.side_withdrew(false)) {
                (true, false) => CombatWinner::Attacker,
                (false, true) => CombatWinner::Defender,
                (true, true) => alliance_wins(self.arrival_holder_is_alliance != Some(true)),
                (false, false) => CombatWinner::Draw,
            },
        }
    }

    /// One second of a battle run without its window (`FUN_005ae9c0`). Each
    /// side in turn assigns targets (`FUN_005c9200`), applies every force's
    /// damage rate to its target (`FUN_005d14d0` through `FUN_005cb650` and
    /// `FUN_005b54d0`) and recharges shields (`FUN_005b6320`); the stalemate
    /// watch follows. Nothing moves and no battery fires.
    /// `FUN_005ae9c0:26-27` steps `battle+0x250` (the Empire) before
    /// `+0x24c` (the Alliance), so the Empire's side goes first
    /// (`ghidra/notes/morale-retreat.md`). Each side's morale check
    /// (`morale_withdraws`, `FUN_005d19a0`) follows its damage and precedes
    /// its recharge (`FUN_005d14d0:101`).
    fn headless_step(&mut self) -> bool {
        self.combat_tick += 4;
        let empire_is_attacker = !self.attacker_is_alliance;
        for is_attacker in [empire_is_attacker, !empire_is_attacker] {
            self.headless_death_star(is_attacker);
            if is_attacker == empire_is_attacker {
                self.launch_headless_hangars();
            }
            let groups = engagement_groups(self);
            for fighters_first in [false, true] {
                assign_engagement_targets(self, &groups, is_attacker, fighters_first);
            }
            let side = usize::from(!is_attacker);
            for group in groups
                .iter()
                .filter(|group| group.is_attacker == is_attacker)
            {
                let Some(target) = self.engagement.targets[side][group.slot]
                    .map(engagement_unit_of)
                    .filter(|target| engagement_unit_live(self, *target))
                else {
                    continue;
                };
                let damage = engagement_group_damage(self, &group.members, target)
                    * ORIGINAL_HEADLESS_STEP_SECONDS;
                self.apply_headless_damage(target, damage);
            }
            if self.morale_withdraws(is_attacker) {
                self.retreat_side(is_attacker);
            }
            for ship in self
                .ships
                .iter_mut()
                .filter(|ship| ship.is_attacker == is_attacker)
            {
                ship.advance_original_recharge(ORIGINAL_HEADLESS_STEP_SECONDS);
            }
            for fighter in self
                .fighters
                .iter_mut()
                .filter(|fighter| fighter.is_attacker == is_attacker)
            {
                fighter.advance_original_recharge(ORIGINAL_HEADLESS_STEP_SECONDS);
            }
        }
        if self.finish_if_decided() {
            return true;
        }
        self.advance_stalemate_watch(ORIGINAL_HEADLESS_STEP_SECONDS)
    }

    /// The side's first fighter group (`+0x70`, kind 4), the squadrons a
    /// headless trench run sends (`FUN_005d14d0.c:28-37`), by index.
    /// port: the lowest fighter group with a live, listed squadron.
    pub(crate) fn first_fighter_group(&self, is_attacker: bool) -> Vec<usize> {
        let flying = |fighter: &TacticalFighter| {
            fighter.is_attacker == is_attacker
                && fighter.alive
                && fighter.squad_count > 0
                && fighter.listed()
        };
        let Some(group) = self
            .fighters
            .iter()
            .filter(|fighter| flying(fighter))
            .map(|fighter| fighter.fighter_group.min(4))
            .min()
        else {
            return Vec::new();
        };
        (0..self.fighters.len())
            .filter(|&index| {
                let fighter = &self.fighters[index];
                flying(fighter) && fighter.fighter_group.min(4) == group
            })
            .collect()
    }

    /// `FUN_005d14d0.c:20-60`, at the head of a side's headless step. The
    /// Alliance side (slot `+4` 0), with the station attackable
    /// (`FUN_005ba450`) and more listed squadrons than the Empire's
    /// (`FUN_005c5eb0` on each side's `+0x64`, strict), sends its first
    /// fighter group on the trench run ([`Self::headless_trench_run`]); it
    /// tries again each step while the test holds. The Empire side, its
    /// station operational (`FUN_005ba420`), charges it short of 100.0 by
    /// one millisecond's worth (`FUN_005ba7f0`) and at exactly 100.0 fires
    /// at the first capital ship of its ranked opponents (`FUN_005d1a70`):
    /// the charge goes to 0 (`FUN_005ba710`, no message, voice or beam) and
    /// the ship is destroyed outright with its hangar (`FUN_005af7a0`).
    fn headless_death_star(&mut self, is_attacker: bool) {
        let Some(station) = self.death_star else {
            return;
        };
        if is_attacker == self.attacker_is_alliance {
            if !station.attackable()
                || self.listed_squadrons(is_attacker) <= self.listed_squadrons(!is_attacker)
            {
                return;
            }
            let members = self.first_fighter_group(is_attacker);
            if !members.is_empty() {
                self.headless_trench_run(members);
            }
            return;
        }
        if !station.operational() {
            return;
        }
        if let Some(station) = self
            .death_star
            .as_mut()
            .filter(|station| station.laser_charge != ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE)
        {
            station.advance_laser_charge(ORIGINAL_HEADLESS_LASER_SECONDS);
        }
        if self.death_star.map(|station| station.laser_charge)
            != Some(ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE)
        {
            return;
        }
        let Some(target) = engagement_priorities(self, is_attacker, false)
            .into_iter()
            .find_map(|unit| match unit {
                EngagementUnit::Capital(index) => Some(index),
                EngagementUnit::Fighter(_) => None,
            })
        else {
            return;
        };
        if let Some(station) = self.death_star.as_mut() {
            station.laser_charge = 0.0;
        }
        let ship = &mut self.ships[target];
        ship.hull_current = 0;
        ship.alive = false;
        ship.selected = false;
        ship.attack_target = None;
        for fighter in &mut self.fighters {
            if fighter.carrier == Some(TacticalBerth::Ship(target)) && fighter.docked.is_some() {
                fighter.destroy();
            }
        }
    }

    /// The side's squadrons in its list `+0x64`.
    fn listed_squadrons(&self, is_attacker: bool) -> usize {
        self.fighters
            .iter()
            .filter(|fighter| {
                fighter.is_attacker == is_attacker
                    && fighter.alive
                    && fighter.squad_count > 0
                    && fighter.listed()
            })
            .count()
    }

    /// `FUN_005aefc0`, the trench run of a battle run without its window.
    /// The members are walked round in id order, survivors keeping their
    /// place, for at most a hundred visits, after which every one left is
    /// destroyed. Each visit draws `rand(1,10)` from the battle generator
    /// ([`OriginalBattleRng`], so 1..=9) against the maneuver
    /// (`FUN_005ba270`): above it, a squadron of fewer than two craft is
    /// destroyed, else it takes `rand(1,n)` craft of its `n` through the
    /// shield-first damage (`FUN_005b54d0`) and is destroyed at no hull. A
    /// survivor rolls `rand(1,100)` (1..=99) against the success chance,
    /// rated by the Alliance Commander when the object carrying it is one
    /// of the battle's (`FUN_005afc00` -> `FUN_005a6340`), alive or not (no
    /// `FUN_005ad870` state check), else 1. A success zeroes the station's hull and charge
    /// (`FUN_005ba8a0`) and destroys every squadron in its hangar
    /// (`0x5af38b-0x5af3f4`); the survivors keep their damage.
    /// port: a squadron's place in `fighters` is its id order (`+0xec`).
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss,
        reason = "The source truncates floats to ints (`__ftol`)."
    )]
    fn headless_trench_run(&mut self, mut members: Vec<usize>) {
        let rating = self.headless_trench_rating();
        let mut at = 0;
        let mut visits = 0;
        while !members.is_empty() {
            visits += 1;
            if visits > ORIGINAL_HEADLESS_TRENCH_RUN_VISITS {
                for index in members {
                    self.fighters[index].destroy();
                }
                return;
            }
            let index = members[at];
            let maneuver = self.fighters[index].trench_maneuver().trunc() as i32;
            if self.battle_rng.between(1, 10) > maneuver {
                let craft =
                    (self.fighters[index].hull_current / ORIGINAL_TRENCH_RUN_DAMAGE_SCALE).trunc() as i32;
                if craft >= 2 {
                    let damage = self.battle_rng.between(1, craft) as f32 * ORIGINAL_TRENCH_RUN_DAMAGE_SCALE;
                    self.damage_squadron_shield_first(index, damage);
                }
                if craft < 2 || self.fighters[index].hull_current <= 0.0 {
                    self.fighters[index].destroy();
                    members.remove(at);
                    if at >= members.len() {
                        at = 0;
                    }
                    continue;
                }
            }
            let chance = trench_run_success_chance(&self.fighters[index], rating);
            if self.battle_rng.between(1, 100) as u32 <= chance {
                if let Some(station) = self.death_star.as_mut() {
                    station.hull = 0.0;
                    station.laser_charge = 0.0;
                }
                for fighter in &mut self.fighters {
                    if fighter.carrier == Some(TacticalBerth::Station) && fighter.docked.is_some() {
                        fighter.destroy();
                    }
                }
                return;
            }
            at = (at + 1) % members.len();
        }
    }

    /// `FUN_005d14d0.c:62-82`: the Empire side's headless step, before its
    /// targets and damage, takes every squadron docked at the start
    /// (`FUN_005af4b0` lists them in `battle+0x58`) out of its hangar
    /// (`FUN_005b8450`) into flight (state 2) and empties the list, so a
    /// battle run without its window launches them all on its first step.
    /// port: a squadron flies for its own side; the source lists an
    /// Alliance one in the Empire side's list, which the shipped FIGHTSD
    /// never stages (its four Alliance classes have hyperdrive 60, so none
    /// docks).
    fn launch_headless_hangars(&mut self) {
        for index in 0..self.fighters.len() {
            if self.fighters[index].squad_count == 0 || self.fighters[index].docked.is_none() {
                continue;
            }
            let position = match self.fighters[index].carrier {
                Some(TacticalBerth::Ship(ship)) => self.ships[ship].source_position,
                Some(TacticalBerth::Station) => self
                    .death_star
                    .map_or(TacticalWorldPosition::ORIGIN, |station| station.source_position),
                None => self.fighters[index].source_position,
            };
            let fighter = &mut self.fighters[index];
            fighter.docked = None;
            fighter.alive = true;
            fighter.recovery_state = TacticalFighterRecoveryState::AwaitingCarrier;
            fighter.source_position = position;
        }
    }

    /// `FUN_005b54d0` for the headless step: shield first, the overflow
    /// (at least one point) into hull, with the capital subsystem rolls.
    fn apply_headless_damage(&mut self, target: EngagementUnit, damage: f32) {
        if !damage.is_finite() || damage <= 0.0 {
            return;
        }
        match target {
            EngagementUnit::Capital(index) => {
                let carry = &mut self.engagement.capital_damage_carry;
                if carry.len() < self.ships.len() {
                    carry.resize(self.ships.len(), 0.0);
                }
                carry[index] += damage;
                let whole = carry[index].floor();
                if whole < 1.0 {
                    return;
                }
                carry[index] -= whole;
                let points = whole as i32;
                let battle_rng = &mut self.battle_rng;
                let rolls = original_tactical_damage_rolls(&self.ships, index, points, || {
                    u32::try_from(battle_rng.between(0, 100)).unwrap_or(0)
                });
                apply_original_capital_damage(
                    &mut self.ships,
                    &mut self.field_effects,
                    index,
                    points,
                    rolls,
                );
            }
            EngagementUnit::Fighter(index) => self.damage_squadron_shield_first(index, damage),
        }
    }

    /// `FUN_005b54d0` on a squadron: nothing without hull; the shield takes
    /// the damage first, with a draw for a shield hit while it had any, and
    /// its overflow (at least one point) the hull, with a draw for the
    /// subsystem roll. A squadron's hits (`FUN_005b1970`) only count, so the
    /// draws keep the stream in step.
    fn damage_squadron_shield_first(&mut self, index: usize, damage: f32) {
        let fighter = &mut self.fighters[index];
        if fighter.hull_current == 0.0 {
            return;
        }
        let shielded = fighter.shield > 0.0;
        fighter.shield -= damage;
        let overflow = (fighter.shield < 0.0).then(|| -fighter.shield);
        if let Some(overflow) = overflow {
            fighter.shield = 0.0;
            fighter.hull_current = (fighter.hull_current - overflow.max(1.0)).max(0.0);
            fighter.refresh_craft_count();
        }
        for _ in 0..u8::from(shielded) + u8::from(overflow.is_some()) {
            self.battle_rng.between(0, 100);
        }
    }

    /// A side's strength for the stalemate watch (`side+0x3c`,
    /// `FUN_005d0ce0`): the firepower (`FUN_005b7a80`) of its squadrons and
    /// of every capital ship whose attackers do not already out-damage its
    /// shield recharge.
    /// An operational Death Star adds `_DAT_0066d0f0` (20000000.0, read from
    /// the executable) to the Empire's strength (`FUN_005d0ce0.c:80-85`),
    /// and to the Alliance's when the station is attackable and the
    /// Alliance lists more squadrons than the Empire (`FUN_005afcc0`), or a
    /// group of the Alliance has order 6 (`FUN_005d0ce0.c:86-103`).
    fn stalemate_strength(&self, is_attacker: bool) -> f32 {
        let groups = engagement_groups(self);
        let enemy_side = usize::from(is_attacker);
        let capitals: f32 = (0..self.ships.len())
            .filter(|index| self.ships[*index].is_attacker == is_attacker)
            .map(EngagementUnit::Capital)
            .filter(|unit| engagement_unit_live(self, *unit))
            .filter(|unit| {
                let EngagementUnit::Capital(index) = *unit else {
                    return true;
                };
                let recovery = self.ships[index].effective_shield_recharge();
                !groups.iter().any(|group| {
                    group.is_attacker != is_attacker
                        && group
                            .ordered_target
                            .or(self.engagement.targets[enemy_side][group.slot])
                            .map(engagement_unit_of)
                            == Some(*unit)
                        && recovery <= engagement_group_damage(self, &group.members, *unit)
                })
            })
            .map(|unit| original_unit_firepower(self, unit))
            .sum();
        let fighters: f32 = (0..self.fighters.len())
            .filter(|index| self.fighters[*index].is_attacker == is_attacker)
            .map(|index| original_unit_firepower(self, EngagementUnit::Fighter(index)))
            .sum();
        let death_star = self
            .death_star
            .filter(|death_star| {
                death_star.operational()
                    && if death_star.is_attacker == is_attacker {
                        !death_star.is_alliance
                    } else {
                        // The other side is the Alliance (side 0).
                        !death_star.is_alliance
                            && self.death_star_strengthens_its_attackers(*death_star)
                    }
            })
            .map_or(0.0, |_| ORIGINAL_DEATH_STAR_STRENGTH);
        capitals + fighters + death_star
    }

    /// The Alliance's half of `FUN_005d0ce0.c:86-103`: `FUN_005afcc0` (the
    /// station attackable and the Alliance's listed squadrons strictly
    /// outnumbering the Empire's) or an Alliance group under order 6.
    fn death_star_strengthens_its_attackers(&self, death_star: TacticalDeathStar) -> bool {
        let attackers = !death_star.is_attacker;
        (death_star.attackable()
            && self.listed_squadrons(attackers) > self.listed_squadrons(death_star.is_attacker))
            || self.fighters.iter().any(|fighter| {
                fighter.is_attacker == attackers
                    && fighter.alive
                    && fighter.order == TacticalOrder::AttackDeathStar
            })
    }

    /// Whether a capital ship or a squadron of the side has a working
    /// hyperdrive (`FUN_005b1b70`, through `FUN_005d0f50.c:16-31`, which
    /// walks the squadron list `+0x64` and the capital list `+0x58`).
    /// port: a squadron's hyperdrive takes no hits (the damage path counts
    /// them on capital ships), so its class rating decides.
    fn side_has_working_hyperdrive(&self, is_attacker: bool) -> bool {
        self.ships.iter().any(|ship| {
            ship.is_attacker == is_attacker && ship.alive && ship_hyperdrive_works(ship)
        }) || self.morale.hyperdrive_squadrons.iter().any(|&index| {
            let fighter = &self.fighters[index];
            fighter.is_attacker == is_attacker && fighter.alive && fighter.squad_count > 0
        })
    }

    /// The Empire's Death Star lets its side retreat (`FUN_005d0f50.c:32-40`:
    /// side id 1, the tactical Empire; the Death Star operational
    /// (`FUN_005ba420`) or not completed (`+0x74`)). The function tests no
    /// fleet or drive: the retreat commits the station anyway
    /// (`FUN_005d14d0.c:124-127`).
    fn death_star_allows_retreat(&self, is_attacker: bool) -> bool {
        self.death_star.is_some_and(|death_star| {
            death_star.is_attacker == is_attacker
                && !death_star.is_alliance
                && (death_star.operational() || death_star.not_completed)
        })
    }

    /// `FUN_005d0f50`: the side may retreat when permitted and not yet
    /// latched, no live opposing ship projects a gravity well, and it has a
    /// way out (a hyperdrive unit or its Death Star).
    /// hyp: `FUN_005c7e40`'s unit `+0x150` is the gravity-well flag.
    fn may_retreat(&self, is_attacker: bool) -> bool {
        let side = usize::from(!is_attacker);
        self.morale.retreat_permitted[side]
            && !self.morale.retreated[side]
            && !self.morale.gravity_wells.iter().any(|&index| {
                self.ships[index].alive && self.ships[index].is_attacker != is_attacker
            })
            && (self.side_has_working_hyperdrive(is_attacker)
                || self.death_star_allows_retreat(is_attacker))
    }

    /// The retreat of `FUN_005d14d0.c:102-146`: units jump, dock or die
    /// (`stalemate_resolve_side`), the latch is set (`+0x84 = 5`), and an
    /// operational Death Star is committed (`+0x78`).
    /// `FUN_005ad530`: the side withdraws. Its controller counts down at
    /// least 30 s, or 150 s for side 1 (the Empire) with the station
    /// operational (`:83-100`); every capital ship and every fighter group
    /// whose first squadron has a hyperdrive rating is ordered to Withdraw
    /// (3), a fighter group whose first squadron has none to Recover (2)
    /// (`:32-81`); a group holding order 6 first breaks its run off
    /// (`:43-46`, `break_off_trench_run`).
    pub fn begin_side_withdrawal(&mut self, is_attacker: bool) {
        let side = usize::from(!is_attacker);
        let empire = is_attacker != self.attacker_is_alliance;
        let station_operational = self
            .death_star
            .is_some_and(|death_star| death_star.operational());
        let floor = if empire && station_operational {
            ORIGINAL_STATION_WITHDRAWAL_COUNTDOWN_MILLISECONDS
        } else {
            ORIGINAL_WITHDRAWAL_COUNTDOWN_MILLISECONDS
        };
        self.break_off_trench_run(is_attacker);
        let withdrawal = &mut self.morale.withdrawal[side];
        withdrawal.state = TacticalWithdrawalState::CountingDown;
        withdrawal.countdown_milliseconds = withdrawal.countdown_milliseconds.max(floor);
        let countdown = withdrawal.countdown_milliseconds;
        self.pending_log_lines.push(format!(
            "[tactical_withdrawal] side={} status=counting_down countdown_ms={countdown} source=FUN_005ad530",
            if empire { "empire" } else { "alliance" },
        ));
        // The order setter drops the group's previous executor
        // (FUN_005ca6d0, `+0x60`), its route with it.
        for ship in self
            .ships
            .iter_mut()
            .filter(|ship| ship.is_attacker == is_attacker && ship.alive)
        {
            ship.order = TacticalOrder::Withdraw;
            ship.navigation_route.clear();
            ship.source_waypoint = None;
        }
        let mut recovering = Vec::new();
        for group in 0..=4_u8 {
            let members: Vec<usize> = (0..self.fighters.len())
                .filter(|&index| {
                    let fighter = &self.fighters[index];
                    fighter.is_attacker == is_attacker
                        && fighter.alive
                        && fighter.docked.is_none()
                        && fighter.fighter_group == group
                })
                .collect();
            let Some(&first) = members.first() else {
                continue;
            };
            let order = if self.morale.hyperdrive_squadrons.contains(&first) {
                TacticalOrder::Withdraw
            } else {
                recovering.extend(members.iter().copied());
                TacticalOrder::Recover
            };
            for &index in &members {
                self.fighters[index].order = order;
            }
        }
        if !recovering.is_empty() {
            begin_original_fighter_recovery(self, &recovering);
        }
    }

    /// State 8 of a withdrawing side's trench run (`FUN_005ad530.c:43-46`,
    /// `FUN_005cfec0.c:165-185`): past its approach, each participant takes
    /// back its state-5 hull and its place in the battle; the group's new
    /// order then drops the run (`FUN_005ca6d0`, `FUN_005d0950`), so no film
    /// or outcome follows.
    fn break_off_trench_run(&mut self, is_attacker: bool) {
        let Some(group) = self.trench_run_group else {
            return;
        };
        // A participant the roll emptied still flies the run until state 6
        // or 7 (`FUN_005d04e0` only sets its hull).
        if !self.fighters.iter().enumerate().any(|(index, fighter)| {
            fighter.is_attacker == is_attacker
                && (fighter.alive || self.trench_run_participants.contains(&index))
                && fighter.fighter_group == group
                && fighter.order == TacticalOrder::AttackDeathStar
        }) {
            return;
        }
        if self.trench_run_remaining_ms.is_some() {
            let participants = std::mem::take(&mut self.trench_run_participants);
            let snapshot = std::mem::take(&mut self.trench_run_snapshot);
            for (index, hull) in participants.into_iter().zip(snapshot) {
                // The roll's damage (state 9 or 10) is undone, a squadron
                // it emptied included: state 2 puts every one back.
                let fighter = &mut self.fighters[index];
                fighter.hull_current = hull;
                fighter.alive = hull > 0.0;
                fighter.off_field = false;
                fighter.refresh_craft_count();
            }
        }
        self.trench_run_group = None;
        self.trench_run_remaining_ms = None;
        self.trench_run_pending_outcome = None;
        self.trench_run_participants.clear();
        self.trench_run_snapshot.clear();
        self.trench_run_chatter_cursor = self.trench_run_chatter.len();
        self.pending_log_lines.push(format!(
            "[tactical_trench] group={group} state=8 status=broken_off source=FUN_005cfec0"
        ));
    }

    /// The side controllers' withdrawal (`FUN_005c8780.c:28-125`), the
    /// Withdraw executors (`FUN_005cc300.c:48-145`, `FUN_005c6610.c:44-110`)
    /// and the units' jumps (`FUN_005a6b40`, `0x005aa29a`), for
    /// `milliseconds` of battle.
    pub fn advance_side_withdrawals(&mut self, milliseconds: f32) {
        self.advance_jumps(milliseconds);
        for is_attacker in [true, false] {
            let side = usize::from(!is_attacker);
            match self.morale.withdrawal[side].state {
                TacticalWithdrawalState::Fighting | TacticalWithdrawalState::Over => continue,
                TacticalWithdrawalState::CountingDown => {
                    let withdrawal = &mut self.morale.withdrawal[side];
                    withdrawal.countdown_milliseconds -= milliseconds;
                    if withdrawal.countdown_milliseconds <= 0.0 {
                        withdrawal.state = TacticalWithdrawalState::Holding;
                        // Side 1 posts 0x45 for its operational station.
                        let empire = is_attacker != self.attacker_is_alliance;
                        let departed = empire && self.depart_death_star(is_attacker);
                        self.pending_log_lines.push(format!(
                            "[tactical_withdrawal] side={} status=holding station_departed={departed} source=FUN_005c8780",
                            if empire { "empire" } else { "alliance" },
                        ));
                    }
                }
                TacticalWithdrawalState::Holding => {
                    if !self.side_hyperdrive_units_in_play(is_attacker) {
                        self.morale.withdrawal[side].state = TacticalWithdrawalState::Over;
                        self.recall_units_left_behind(is_attacker);
                        self.pending_log_lines.push(format!(
                            "[tactical_withdrawal] side={} status=over source=FUN_005c8780",
                            if is_attacker == self.attacker_is_alliance {
                                "alliance"
                            } else {
                                "empire"
                            },
                        ));
                        continue;
                    }
                }
            }
            self.advance_withdraw_orders(is_attacker);
        }
    }

    /// `FUN_005ba5e0` (message `0x45`): an operational station departs
    /// (`+0x78`), and `FUN_005b8d10` moves the squadrons still waiting to
    /// launch from its hangar into its recovered list (state 5); squadrons
    /// in flight are not recalled. Returns whether it departed.
    fn depart_death_star(&mut self, is_attacker: bool) -> bool {
        // An uncompleted station is not operational, so it never departs
        // (`FUN_005ba5e0` tests `FUN_005ba420`).
        let departs = self.death_star.is_some_and(|death_star| {
            death_star.is_attacker == is_attacker
                && !death_star.is_alliance
                && death_star.operational()
        });
        if !departs {
            return false;
        }
        if let Some(death_star) = self.death_star.as_mut() {
            death_star.action_committed = true;
        }
        for fighter in &mut self.fighters {
            if fighter.carrier == Some(TacticalBerth::Station)
                && fighter.docked == Some(TacticalDock::Ready)
            {
                fighter.docked = Some(TacticalDock::Recovered);
            }
        }
        true
    }

    /// `FUN_005c8780.c:72-101`: a capital ship or flying squadron of the
    /// side with a working hyperdrive (`FUN_005b1b70`) still fights or is in
    /// its jump.
    /// hyp: a unit that has left (state 8) is off the side's lists once
    /// `0x2e` is handled.
    fn side_hyperdrive_units_in_play(&self, is_attacker: bool) -> bool {
        let ships = self.ships.iter().any(|ship| {
            ship.is_attacker == is_attacker && ship.alive && ship_hyperdrive_works(ship)
        });
        let squadrons = (0..self.fighters.len()).any(|index| {
            let fighter = &self.fighters[index];
            fighter.is_attacker == is_attacker
                && fighter.alive
                && fighter.docked.is_none()
                && self.morale.hyperdrive_squadrons.contains(&index)
        });
        ships || squadrons
    }

    /// `FUN_005c8780.c:102-125` -> `0x2f` (`FUN_005b2720`): each unit of the
    /// side without a working hyperdrive that is alive with hull above 0.0
    /// drops its orders and fights on (state 2).
    fn recall_units_left_behind(&mut self, is_attacker: bool) {
        for ship in self.ships.iter_mut().filter(|ship| {
            ship.is_attacker == is_attacker
                && ship.alive
                && !ship.retreating
                && ship.hull_current > 0
                && !ship_hyperdrive_works(ship)
        }) {
            ship.order = TacticalOrder::None;
        }
        for index in 0..self.fighters.len() {
            let fighter = &self.fighters[index];
            if fighter.is_attacker == is_attacker
                && fighter.alive
                && fighter.docked.is_none()
                && fighter.hull_current > 0.0
                && !self.morale.hyperdrive_squadrons.contains(&index)
            {
                self.fighters[index].order = TacticalOrder::None;
            }
        }
    }

    /// The side's withdrawal line: z at half the battle extent on its own
    /// edge, the side's facing (`FUN_005a7490`: side 0 (0, 0, +1), side 1
    /// (0, 0, -1), `FUN_005be160.c:27-45`) times -1.0.
    fn withdrawal_line(&self, is_attacker: bool) -> (f32, f32) {
        let facing = if is_attacker == self.attacker_is_alliance {
            1.0
        } else {
            -1.0
        };
        (
            facing,
            -facing * ORIGINAL_WITHDRAWAL_LINE_SCALE * self.source_layout.battle_extent,
        )
    }

    /// The Withdraw executors for the side's units. Each unit flies along z
    /// to the line, x and y kept, and holds within 1.0 of it: a ship through
    /// its movement (its waypoint), a squadron at its speed times 0.95
    /// (`FUN_005eb1a0`);
    /// with the countdown out, a squadron with a working hyperdrive within
    /// 8.0 of the line jumps (`FUN_005c6610.c:76-108`), and of a task force
    /// only its front ship, the first not yet left (`FUN_005c6070`), and
    /// only when it is alive, can jump and is within 8.0 of the line; any
    /// other front ship holds the rest back (`FUN_005cc300.c:77-145`).
    /// port: the source moves the group's leader and the formation brings
    /// the members along; the port moves each unit.
    fn advance_withdraw_orders(&mut self, is_attacker: bool) {
        let side = usize::from(!is_attacker);
        let (facing, line) = self.withdrawal_line(is_attacker);
        let countdown_out = self.morale.withdrawal[side].countdown_milliseconds <= 0.0;
        // `-((line - z) * facing) <= 8.0`: the register holds the line's z,
        // the stack copy the unit's (`FUN_005cc300.c:101`).
        let near = |z: f32| (z - line) * facing <= ORIGINAL_WITHDRAWAL_JUMP_DISTANCE;
        let seconds = ORIGINAL_TACTICAL_STEP_MILLISECONDS * 0.001;
        let step = |position: &mut TacticalWorldPosition, power: f32| {
            let distance = (line - position.z).abs();
            if distance > ORIGINAL_WITHDRAWAL_ARRIVAL_DISTANCE {
                let travel =
                    (power.max(0.0) * ORIGINAL_TRENCH_RUN_SPEED_SCALE * seconds).min(distance);
                position.z += (line - position.z).signum() * travel;
            }
        };
        let mut fronts: Vec<u8> = Vec::new();
        for index in 0..self.ships.len() {
            let ship = &mut self.ships[index];
            if ship.is_attacker != is_attacker
                || !ship.alive
                || ship.retreating
                || ship.order != TacticalOrder::Withdraw
            {
                continue;
            }
            if (line - ship.source_position.z).abs() > ORIGINAL_WITHDRAWAL_ARRIVAL_DISTANCE {
                ship.source_waypoint = Some(TacticalWorldPosition {
                    z: line,
                    ..ship.source_position
                });
            } else {
                ship.source_waypoint = None;
                ship.source_desired_forward = TacticalWorldVector::ZERO;
            }
            let task_force = ship.task_force;
            if fronts.contains(&task_force) {
                continue;
            }
            fronts.push(task_force);
            let in_jump = self.ships.iter().any(|other| {
                other.is_attacker == is_attacker
                    && other.task_force == task_force
                    && other.retreating
            });
            let ship = &mut self.ships[index];
            if countdown_out
                && !in_jump
                && ship_hyperdrive_works(ship)
                && near(ship.source_position.z)
            {
                ship.retreating = true;
                ship.retreat_progress = 0.0;
                ship.selected = false;
                ship.attack_target = None;
            }
        }
        for index in 0..self.fighters.len() {
            let fighter = &mut self.fighters[index];
            if fighter.is_attacker != is_attacker
                || !fighter.alive
                || fighter.docked.is_some()
                || fighter.order != TacticalOrder::Withdraw
                || self
                    .morale
                    .squadron_jumps
                    .iter()
                    .any(|&(jumping, _)| jumping == index)
            {
                continue;
            }
            step(&mut fighter.source_position, fighter.sublight_engine_power);
            if countdown_out
                && self.morale.hyperdrive_squadrons.contains(&index)
                && near(fighter.source_position.z)
            {
                fighter.selected = false;
                fighter.attack_target = None;
                self.morale.squadron_jumps.push((index, 0.0));
            }
        }
    }

    /// State 7 (`FUN_005a6b40`): a unit in its jump plays its effect on the
    /// first update and leaves after 4000.0 ms, when the battle's unit pass
    /// sets state 8 (`0x005aa29a`-`0x005aa2e5`). Its flight out is drawn
    /// (`ship_display_position`, `fighter_display_position`).
    fn advance_jumps(&mut self, milliseconds: f32) {
        let mut jumped = [false; 2];
        for index in 0..self.ships.len() {
            let ship = &self.ships[index];
            if !ship.retreating || !ship.alive {
                continue;
            }
            let id = self.original_object_id(ship.is_attacker, Some(index), None);
            let started = ship.retreat_progress == 0.0;
            if started {
                self.pending_audio_cues.push(TacticalAudioCue::from_variant(
                    TacticalAudioEvent::CapitalShipJump,
                    0,
                ));
                self.log_jump(id, "jumping", 0.0);
            }
            let ship = &mut self.ships[index];
            ship.retreat_progress += milliseconds;
            if ship.retreat_progress >= ORIGINAL_JUMP_MILLISECONDS {
                ship.alive = false;
                ship.retreating = false;
                ship.retreated = true;
                jumped[usize::from(!ship.is_attacker)] = true;
                let elapsed = ship.retreat_progress;
                self.log_jump(id, "jumped", elapsed);
            }
        }
        let mut still = Vec::new();
        for (index, elapsed) in std::mem::take(&mut self.morale.squadron_jumps) {
            let fighter = &self.fighters[index];
            if !fighter.alive {
                continue;
            }
            let id = self.original_object_id(fighter.is_attacker, None, Some(index));
            if elapsed == 0.0 {
                self.pending_audio_cues.push(TacticalAudioCue::from_variant(
                    TacticalAudioEvent::SquadronJump,
                    0,
                ));
                self.log_jump(id, "jumping", 0.0);
            }
            let elapsed = elapsed + milliseconds;
            if elapsed >= ORIGINAL_JUMP_MILLISECONDS {
                let fighter = &mut self.fighters[index];
                fighter.alive = false;
                jumped[usize::from(!fighter.is_attacker)] = true;
                self.log_jump(id, "jumped", elapsed);
            } else {
                still.push((index, elapsed));
            }
        }
        self.morale.squadron_jumps = still;
        for (side, left) in self.morale.jumped.iter_mut().zip(jumped) {
            *side |= left;
        }
    }

    /// port: a unit's jump starting (state 7) and its leaving (state 8)
    /// are logged by object id and the jump's milliseconds, for native runs.
    fn log_jump(&mut self, id: u32, status: &str, elapsed: f32) {
        self.pending_log_lines.push(format!(
            "[tactical_withdrawal] unit={id} status={status} t_ms={elapsed} source=FUN_005a6b40"
        ));
    }

    /// Where a ship is drawn: in its jump, moved against its side's facing
    /// (`FUN_005a6b40`, `jump_flight`); else where it is.
    #[must_use]
    pub fn ship_display_position(&self, index: usize) -> TacticalWorldPosition {
        let ship = &self.ships[index];
        let mut position = ship.source_position;
        if ship.retreating {
            let ordinal = ship.tactical_resource.map_or(0, |resource| resource.tactical_ordinal);
            position.z -= self.withdrawal_line(ship.is_attacker).0
                * jump_flight(
                    ship.retreat_progress,
                    ordinal,
                    self.original_object_id(ship.is_attacker, Some(index), None),
                );
        }
        position
    }

    /// [`Self::ship_display_position`] for a squadron.
    #[must_use]
    pub fn fighter_display_position(&self, index: usize) -> TacticalWorldPosition {
        let fighter = &self.fighters[index];
        let mut position = fighter.source_position;
        if let Some(&(_, elapsed)) = self
            .morale
            .squadron_jumps
            .iter()
            .find(|&&(jumping, _)| jumping == index)
        {
            let ordinal = fighter
                .tactical_resource
                .map_or(0, |resource| resource.tactical_ordinal);
            position.z -= self.withdrawal_line(fighter.is_attacker).0
                * jump_flight(
                    elapsed,
                    ordinal,
                    self.original_object_id(fighter.is_attacker, None, Some(index)),
                );
        }
        position
    }

    /// A unit's object id (`+0xec`): its side's count from the side's base
    /// (`ORIGINAL_SIDE_1_OBJECT_ID_BASE`), one more for each unit created.
    /// hyp: the battle creates each side's ships, then its squadrons, in
    /// the session's order, and nothing else of the side takes an id first.
    fn original_object_id(&self, is_attacker: bool, ship: Option<usize>, fighter: Option<usize>) -> u32 {
        let is_alliance = is_attacker == self.attacker_is_alliance;
        let ships_before = self
            .ships
            .iter()
            .take(ship.unwrap_or(self.ships.len()))
            .filter(|other| other.is_attacker == is_attacker)
            .count();
        let fighters_before = fighter.map_or(0, |index| {
            self.fighters
                .iter()
                .take(index)
                .filter(|other| other.is_attacker == is_attacker)
                .count()
        });
        let base = if is_alliance { 0 } else { ORIGINAL_SIDE_1_OBJECT_ID_BASE };
        base + u32::try_from(ships_before + fighters_before + 1).unwrap_or(u32::MAX)
    }

    fn retreat_side(&mut self, is_attacker: bool) {
        self.stalemate_resolve_side(is_attacker);
        self.morale.retreated[usize::from(!is_attacker)] = true;
        // Only the side's own operational station departs
        // (`depart_death_star`, `FUN_005ba5e0` tests `FUN_005ba420`).
        self.depart_death_star(is_attacker);
    }

    /// The stalemate loser (`FUN_005ae9c0.c:62-103`): a side that may
    /// retreat is flagged (`+0xb0`) and retreats; one that may not has every
    /// ship and squadron destroyed at once (`FUN_005af7a0`), the squadrons
    /// docked in those ships with them (`tactical-death-star.md`, section 18).
    fn stalemate_withdraw(&mut self, is_attacker: bool) {
        if self.may_retreat(is_attacker) {
            self.retreat_side(is_attacker);
            return;
        }
        for ship in self
            .ships
            .iter_mut()
            .filter(|ship| ship.is_attacker == is_attacker && ship.alive)
        {
            ship.hull_current = 0;
            ship.shield = 0;
            ship.alive = false;
            ship.retreating = false;
            ship.selected = false;
            ship.attack_target = None;
        }
        for fighter in self
            .fighters
            .iter_mut()
            .filter(|fighter| fighter.is_attacker == is_attacker && fighter.alive)
        {
            fighter.squad_count = 0;
            fighter.hull_current = 0.0;
            fighter.alive = false;
            fighter.selected = false;
            fighter.attack_target = None;
        }
        self.destroy_squadrons_docked_in_lost_ships();
    }

    /// A side counts as withdrawn (`FUN_005a9ec0.c:245-273`) when no unit of
    /// it is alive (`FUN_005ad870`: state 2, 4 or 6) or in its jump (states
    /// 7 and 8), and a unit of it jumped out (state 9) or, for the Empire,
    /// its station departed (`+0x78`). The original also requires the side's
    /// controller out of the fight (`+0x84` 3, 4 or 5); the port jumps units
    /// and departs the station only in that state (`advance_side_withdrawals`,
    /// `retreat_side`), so the jump implies it.
    #[must_use]
    pub fn side_withdrew(&self, is_attacker: bool) -> bool {
        let side = usize::from(!is_attacker);
        let station_departed = self
            .death_star
            .is_some_and(|death_star| death_star.is_attacker == is_attacker && death_star.action_committed);
        let left = self.morale.jumped[side] || station_departed;
        let live = self
            .ships
            .iter()
            .any(|ship| ship.is_attacker == is_attacker && ship.alive)
            || self
                .fighters
                .iter()
                .any(|fighter| fighter.is_attacker == is_attacker && fighter.alive);
        left && !live
    }

    /// `FUN_005d19a0` for one side: it withdraws when it may retreat
    /// (`FUN_005d0f50`: permitted and not yet latched, no gravity-well ship
    /// on the opposing side, a working hyperdrive) and its strength
    /// (`stalemate_strength`, `FUN_005d0ce0`) is 0, or, with the opponent's
    /// above 0, its ratio to the opponent's is below the side's threshold
    /// (`morale.threshold`, `side+0x24`).
    /// port: the stalemate flag (`+0xb0`, test 1) is the stalemate watch,
    /// which resolves its own sides (`stalemate_withdraw`).
    fn morale_withdraws(&self, is_attacker: bool) -> bool {
        let side = usize::from(!is_attacker);
        if !self.may_retreat(is_attacker) {
            return false;
        }
        let own = self.stalemate_strength(is_attacker);
        if own == 0.0 {
            return true;
        }
        let opponent = self.stalemate_strength(!is_attacker);
        opponent > 0.0 && own / opponent < self.morale.threshold[side]
    }

    /// `FUN_005ae9c0`: once neither side's strength has fallen to a new low
    /// for 1200 seconds, the side with the lower or equal strength (both when
    /// equal) leaves the battle (`stalemate_resolve_side`). Returns `true`
    /// when that ends the battle. While a Death Star is operational
    /// (`FUN_005ba420`), the side facing it counts -1.0 at the decision, so
    /// that side always leaves and the Death Star's side never does.
    fn advance_stalemate_watch(&mut self, seconds: f32) -> bool {
        let strengths = [
            self.stalemate_strength(true),
            self.stalemate_strength(false),
        ];
        for (side, strength) in strengths.into_iter().enumerate() {
            let watch = &mut self.stalemate;
            if watch.lowest[side].is_none_or(|lowest| strength < lowest) {
                watch.lowest[side] = Some(strength);
                watch.quiet_seconds[side] = 0.0;
            } else {
                watch.quiet_seconds[side] += seconds;
            }
        }
        if self
            .stalemate
            .quiet_seconds
            .iter()
            .any(|&quiet| quiet < ORIGINAL_STALEMATE_SECONDS)
        {
            return false;
        }
        let [mut attacker, mut defender] = strengths;
        if let Some(death_star) = self
            .death_star
            .filter(|death_star| death_star.operational())
        {
            if death_star.is_attacker {
                defender = -1.0;
            } else {
                attacker = -1.0;
            }
        }
        for (is_attacker, withdraws) in
            [(true, attacker <= defender), (false, defender <= attacker)]
        {
            if withdraws {
                self.stalemate_withdraw(is_attacker);
            }
        }
        let winner = self.decided_winner();
        self.pending_log_lines.push(format!(
            "[tactical_stalemate] attacker_strength={} defender_strength={} winner={:?}",
            attacker, defender, winner
        ));
        self.queue_battle_outcome_voice();
        self.winner = Some(winner);
        self.phase = BattlePhase::Results;
        true
    }

    /// The stalemate's loser. A side with a ship whose hyperdrive works
    /// (`FUN_005d0f50` through `FUN_005b1b70`) is flagged to retreat
    /// (`side+0xb0`), and `FUN_005d14d0`'s retreat branch (`:102-146`)
    /// takes its squadrons in flight first: one with a working hyperdrive
    /// jumps out (`FUN_005af910`, state 9), any other docks
    /// (`FUN_005afe40`) or is destroyed (`FUN_005af7a0`: 100000 damage,
    /// state `0xb`). Its ships follow: a working hyperdrive jumps, any other
    /// is destroyed. A side with no working hyperdrive is destroyed outright.
    /// Squadrons already in a hangar stay there and share their carrier's
    /// fate. hyp: where `FUN_005af910` sends a jumped squadron is untraced;
    /// it ends the battle undocked ([`BattleSession::end_berths`]).
    fn stalemate_resolve_side(&mut self, is_attacker: bool) {
        let side = usize::from(!is_attacker);
        let escapes = ship_hyperdrive_works;
        let can_retreat = self.side_has_working_hyperdrive(is_attacker);
        let flying: Vec<usize> = (0..self.fighters.len())
            .filter(|&index| {
                let fighter = &self.fighters[index];
                fighter.is_attacker == is_attacker && fighter.alive && fighter.docked.is_none()
            })
            .collect();
        for index in flying {
            if self.morale.hyperdrive_squadrons.contains(&index) {
                self.morale.jumped[side] = true;
            } else if let Some(berth) = self.retreat_berth(index) {
                self.fighters[index].carrier = Some(berth);
                self.fighters[index].docked = Some(TacticalDock::Recovered);
            } else {
                self.fighters[index].squad_count = 0;
                self.fighters[index].hull_current = 0.0;
            }
            let fighter = &mut self.fighters[index];
            fighter.alive = false;
            fighter.selected = false;
            fighter.attack_target = None;
        }
        for ship in self
            .ships
            .iter_mut()
            .filter(|ship| ship.is_attacker == is_attacker && ship.alive)
        {
            if can_retreat && escapes(ship) {
                ship.retreated = true;
                self.morale.jumped[side] = true;
            } else {
                ship.hull_current = 0;
                ship.shield = 0;
            }
            ship.alive = false;
            ship.retreating = false;
            ship.selected = false;
            ship.attack_target = None;
        }
        self.destroy_squadrons_docked_in_lost_ships();
    }

    /// `FUN_005afe40`: a retreating squadron in flight docks in its own
    /// carrier (`+0x164`; the station only while operational, a ship only
    /// in the battle, state 2, `FUN_005afc80`), else in the first ship of
    /// its side in the battle, else, for the Empire, in its operational
    /// station; each only with room (`FUN_005b84a0`: docked `+8` below
    /// capacity `+0x10`).
    fn retreat_berth(&self, index: usize) -> Option<TacticalBerth> {
        let fighter = &self.fighters[index];
        let station_open = self
            .death_star
            .is_some_and(|death_star| death_star.operational());
        let ship_open = |ship: usize| {
            self.ships
                .get(ship)
                .is_some_and(|ship| ship.alive && !ship.retreating && ship.is_attacker == fighter.is_attacker)
        };
        let own = fighter.carrier.filter(|&berth| match berth {
            TacticalBerth::Station => station_open,
            TacticalBerth::Ship(ship) => ship_open(ship),
        });
        own.filter(|&berth| self.hangar_has_room(berth))
            .or_else(|| {
                (0..self.ships.len())
                    .filter(|&ship| ship_open(ship))
                    .map(TacticalBerth::Ship)
                    .find(|&berth| self.hangar_has_room(berth))
            })
            .or_else(|| {
                (!fighter.identity.is_alliance && station_open)
                    .then_some(TacticalBerth::Station)
                    .filter(|&berth| self.hangar_has_room(berth))
            })
    }

    /// `FUN_005b84a0`: a hangar takes a squadron while its docked count
    /// (`+8`) is below its capacity (`+0x10`).
    fn hangar_has_room(&self, berth: TacticalBerth) -> bool {
        self.hangar_count(berth) < self.hangar_capacity(berth)
    }

    fn hangar_count(&self, berth: TacticalBerth) -> u32 {
        self.fighters
            .iter()
            .filter(|fighter| {
                fighter.squad_count > 0
                    && fighter.carrier == Some(berth)
                    && fighter.docked.is_some()
            })
            .count()
            .try_into()
            .unwrap_or(u32::MAX)
    }

    fn hangar_capacity(&self, berth: TacticalBerth) -> u32 {
        match berth {
            TacticalBerth::Ship(ship) => self.ships.get(ship).map_or(0, |ship| ship.fighter_capacity),
            TacticalBerth::Station => self
                .death_star
                .map_or(0, |death_star| death_star.hangar_capacity),
        }
    }

    /// The carrier each squadron ends the battle with, `None` for one left
    /// in no hangar. At the battle's end (`FUN_005bef90.c:72-96`, its
    /// network twin `FUN_005c0b60.c:57-83`, headless `FUN_005aee90.c:20-47`)
    /// every squadron still in the battle, docked or jumped docks again in
    /// the first ship of its side still in the battle or jumped with room
    /// (`FUN_005b84a0`, which adds to the count without leaving the old
    /// hangar), else, for the Empire, in the station while it is operational
    /// or departed (`+0x78`); a squadron that finds no room keeps the
    /// carrier it docked in last if docked, and none in flight.
    #[must_use]
    pub fn end_berths(&self) -> Vec<Option<TacticalBerth>> {
        let mut counts: Vec<u32> = (0..self.ships.len())
            .map(|ship| self.hangar_count(TacticalBerth::Ship(ship)))
            .collect();
        let mut station_count = self.hangar_count(TacticalBerth::Station);
        let station_open = self.death_star.is_some_and(|death_star| {
            death_star.operational() || death_star.action_committed
        });
        let mut berths = Vec::with_capacity(self.fighters.len());
        for fighter in &self.fighters {
            let mut berth = fighter.docked.and(fighter.carrier);
            if fighter.squad_count > 0 {
                let ship = (0..self.ships.len()).find(|&index| {
                    let ship = &self.ships[index];
                    ship.is_attacker == fighter.is_attacker
                        && (ship.alive || ship.retreated)
                        && counts[index] < ship.fighter_capacity
                });
                if let Some(ship) = ship {
                    counts[ship] += 1;
                    berth = Some(TacticalBerth::Ship(ship));
                } else if !fighter.identity.is_alliance
                    && station_open
                    && station_count < self.hangar_capacity(TacticalBerth::Station)
                {
                    station_count += 1;
                    berth = Some(TacticalBerth::Station);
                }
            }
            berths.push(berth);
        }
        berths
    }

    /// Draw the import's crew pass once, as the battle ends: the fates the
    /// write-back applies and the Battle Summary files. The original
    /// imports the battle (`FUN_0040c780`, state `0x16`/`0x17` of
    /// `FUN_0040a050`) before the result event that builds the summary
    /// (state 9). port: a played battle shows its summary before the
    /// write-back, so both read this one draw.
    pub fn settle_crew(&mut self, world: &GameWorld) {
        if self.settled_crew.is_none() {
            self.settled_crew = Some(crate::battle_summary::settle_crew(self, world));
        }
    }

    /// Run the battle to its end without its window, as the Battle Alert's
    /// Simulate does: the session state machine's state 8 steps the battle
    /// one second at a time through `FUN_005ae9c0` (`headless_step`;
    /// `ghidra/notes/simulate-runs-tactical-engine.md`).
    pub fn simulate_to_end(&mut self) {
        self.phase = BattlePhase::Combat;
        self.paused = false;
        self.placement_confirmed = true;
        for _ in 0..SIMULATED_BATTLE_SECOND_LIMIT {
            if self.headless_step() {
                break;
            }
        }
        self.pending_voice_cues.clear();
        self.pending_audio_cues.clear();
        if self.winner.is_none() {
            self.winner = Some(CombatWinner::Draw);
            self.phase = BattlePhase::Results;
        }
        self.pending_log_lines.push(format!(
            "[simulate] seconds={} winner={:?}",
            self.combat_tick / 4,
            self.winner
        ));
        self.weapon_effects.clear();
        self.impact_effects.clear();
        self.paused = true;
        self.selected_ship = None;
        self.selected_fighter_group = None;
    }

    /// One side's ships fire through the four source battery records recovered
    /// from `FUN_005b3a40`, `FUN_005b3f10`, and `FUN_005b6530`.
    fn fire_side(
        ships: &mut [TacticalShip],
        field_effects: &mut Vec<TacticalFieldEffect>,
        firing: &[usize],
        targets: &[usize],
        effects: &mut Vec<WeaponEffect>,
        impact_effects: &mut Vec<TacticalImpactEffect>,
        audio_cues: &mut Vec<TacticalAudioCue>,
        voice_cues: &mut Vec<TacticalVoiceCue>,
        player_is_attacker: bool,
        player_faction: TacticalVoiceFaction,
        rng: &mut OriginalTacticalRng,
    ) {
        if targets.is_empty() {
            return;
        }

        for &fire_idx in firing {
            if !ships[fire_idx].alive || ships[fire_idx].retreating {
                continue;
            }

            // A typed capital target follows the direct-target callback. The
            // autonomous path evaluates every in-range target independently
            // for each family and keeps the first strongest target.
            let direct_target = match ships[fire_idx].attack_target {
                Some(TacticalAttackTarget::CapitalShip(target))
                    if target < ships.len()
                        && ships[target].alive
                        && !ships[target].retreating
                        && targets.contains(&target) =>
                {
                    Some(target)
                }
                Some(TacticalAttackTarget::CapitalShip(_))
                    if ships[fire_idx].order == TacticalOrder::AttackCapitalShips =>
                {
                    // The attack executor never redirects an invalid typed
                    // target into an unrelated class or random capital. Its
                    // target-list lifecycle reacquires before this phase.
                    continue;
                }
                Some(TacticalAttackTarget::FighterGroup(_)) => {
                    // FUN_005d0bb0 accepts only object class 1, and
                    // FUN_005a8fc0 links that typed fighter target. Once the
                    // shared list lifecycle clears it, do not enter the
                    // autonomous capital-target path.
                    continue;
                }
                None if matches!(
                    ships[fire_idx].order,
                    TacticalOrder::AttackCapitalShips | TacticalOrder::AttackFighters
                ) =>
                {
                    // No eligible target of the requested class remains.
                    continue;
                }
                _ => None,
            };
            let mut candidates =
                original_capital_weapon_candidates(ships, fire_idx, targets, direct_target);

            loop {
                let mut strongest = 0.0;
                let mut selected_arc = None;
                for (arc_index, arc_candidates) in candidates.iter().enumerate() {
                    let total = arc_candidates.total();
                    if strongest < total {
                        strongest = total;
                        selected_arc = Some(arc_index);
                    }
                }
                let Some(arc_index) = selected_arc else {
                    break;
                };

                let arc_candidates = candidates[arc_index];
                let arc = ships[fire_idx].weapon_arcs[arc_index];
                let mut energy_spent = 0.0;
                // Preserve the executable's event order: ion, laser, then
                // turbolaser. Every populated family in the selected arc fires.
                for (kind, candidate) in [
                    (WeaponKind::IonCannon, arc_candidates.ion),
                    (WeaponKind::LaserCannon, arc_candidates.laser),
                    (WeaponKind::Turbolaser, arc_candidates.turbolaser),
                ] {
                    let Some(target_idx) = candidate.target else {
                        continue;
                    };
                    let source_is_attacker = ships[fire_idx].is_attacker;
                    let source_group = ships[fire_idx].task_force;
                    let target_is_attacker = ships[target_idx].is_attacker;
                    let target_group = ships[target_idx].task_force;
                    let damage = original_tactical_integer_damage(candidate.strength);
                    let rolls = original_windowed_damage_rolls(
                        ships,
                        target_idx,
                        damage,
                        rng,
                        player_is_attacker,
                    );
                    let impact_stage = apply_original_capital_damage(
                        ships,
                        field_effects,
                        target_idx,
                        damage,
                        rolls,
                    );
                    queue_original_destruction_voice(
                        voice_cues,
                        player_faction,
                        player_is_attacker,
                        source_is_attacker,
                        TacticalVoiceGroup::Capital(source_group),
                        target_is_attacker,
                        TacticalVoiceGroup::Capital(target_group),
                        impact_stage,
                    );
                    queue_original_tactical_impact(impact_effects, target_idx, kind, impact_stage);
                    queue_original_tactical_projectile(
                        effects,
                        ships,
                        fire_idx,
                        target_idx,
                        kind,
                        candidate.strength,
                    );
                    let fire_event = TacticalAudioEvent::fire_for(kind);
                    let impact_event = TacticalAudioEvent::impact_for(kind);
                    audio_cues.push(TacticalAudioCue::from_variant(
                        fire_event,
                        u8::try_from(rng.inclusive(0, u32::from(fire_event.variant_count() - 1)))
                            .unwrap_or(0),
                    ));
                    audio_cues.push(TacticalAudioCue::from_variant(
                        impact_event,
                        u8::try_from(rng.inclusive(0, u32::from(impact_event.variant_count() - 1)))
                            .unwrap_or(0),
                    ));
                    energy_spent += original_weapon_count(arc, kind) as f32;
                }

                if energy_spent > 0.0 {
                    let source = &mut ships[fire_idx];
                    let arc = &mut source.weapon_arcs[arc_index];
                    arc.energy = (arc.energy - energy_spent).max(0.0);
                    arc.ready = false;
                    let arc_index = u8::try_from(arc_index).unwrap_or(3);
                    if !source.weapon_recharge_queue.contains(&arc_index) {
                        source.weapon_recharge_queue.push(arc_index);
                    }
                }
                candidates[arc_index] = OriginalTacticalArcCandidates::default();
            }
        }
    }

    /// A capital ship with a fighter target fires its ready arc's lasers and
    /// turbolasers with the fighter modifiers of `FUN_005d24e0` and
    /// `FUN_005d2490`, then queues the arc for recharge (`FUN_005b64e0`).
    /// port: the gate on the Attack Fighters order; the source weapon loop's
    /// fighter path is not traced.
    fn fire_capitals_at_fighters(&mut self, effects: &mut Vec<WeaponEffect>) {
        let player_is_attacker = self.player_is_attacker;
        let player_faction = self.player_voice_faction();
        for source in 0..self.ships.len() {
            let Some(TacticalAttackTarget::FighterGroup(target)) = self.ships[source].attack_target
            else {
                continue;
            };
            if self.ships[source].order != TacticalOrder::AttackFighters
                || !self.ships[source].alive
                || self.ships[source].retreating
                || !self.fighters.get(target).is_some_and(|fighter| {
                    fighter.alive
                        && fighter.squad_count > 0
                        && fighter.listed()
                        && fighter.is_attacker != self.ships[source].is_attacker
                })
            {
                continue;
            }

            let target_position = self.fighters[target].source_position;
            let delta = source_delta(target_position, self.ships[source].source_position);
            let distance = delta.x.hypot(delta.y).hypot(delta.z);
            let arc_index =
                original_tactical_weapon_arc_to_position(&self.ships[source], target_position)
                    .index();
            let arc = self.ships[source].weapon_arcs[arc_index];
            if !arc.ready {
                continue;
            }
            let source_maneuverability = (self.ships[source].maneuverability as f32 + 1.0)
                .clamp(1.0, ORIGINAL_CAPITAL_MANEUVERABILITY_MAX);
            let mut fired = false;
            for kind in [WeaponKind::LaserCannon, WeaponKind::Turbolaser] {
                let count = original_weapon_count(arc, kind);
                if count == 0 || distance > original_weapon_range(&self.ships[source], kind) {
                    continue;
                }
                let strength =
                    original_capital_weapon_strength(&self.ships[source], count, kind, false)
                        * if kind == WeaponKind::Turbolaser {
                            ORIGINAL_TURBOLASER_VS_FIGHTER
                        } else {
                            ORIGINAL_LASER_VS_FIGHTER / ORIGINAL_LASER_VS_CAPITAL
                        };
                let source_is_attacker = self.ships[source].is_attacker;
                let source_group = self.ships[source].task_force;
                let target_is_attacker = self.fighters[target].is_attacker;
                let target_group = self.fighters[target].fighter_group;
                let stage = apply_original_fighter_damage(
                    &mut self.fighters,
                    target,
                    strength,
                    source_maneuverability,
                );
                queue_original_destruction_voice(
                    &mut self.pending_voice_cues,
                    player_faction,
                    player_is_attacker,
                    source_is_attacker,
                    TacticalVoiceGroup::Capital(source_group),
                    target_is_attacker,
                    TacticalVoiceGroup::Fighter(target_group),
                    stage,
                );
                queue_original_capital_to_fighter_projectile(
                    effects,
                    &self.ships,
                    &self.fighters,
                    source,
                    target,
                    kind,
                    strength,
                );
                let fire_event = TacticalAudioEvent::fire_for(kind);
                let impact_event = TacticalAudioEvent::impact_for(kind);
                self.pending_audio_cues.push(TacticalAudioCue::from_variant(
                    fire_event,
                    u8::try_from(
                        self.tactical_rng
                            .inclusive(0, u32::from(fire_event.variant_count() - 1)),
                    )
                    .unwrap_or(0),
                ));
                self.pending_audio_cues.push(TacticalAudioCue::from_variant(
                    impact_event,
                    u8::try_from(
                        self.tactical_rng
                            .inclusive(0, u32::from(impact_event.variant_count() - 1)),
                    )
                    .unwrap_or(0),
                ));
                fired = true;
            }
            if fired {
                let ship = &mut self.ships[source];
                let arc = &mut ship.weapon_arcs[arc_index];
                arc.energy = 0.0;
                arc.ready = false;
                let arc_index = u8::try_from(arc_index).unwrap_or(3);
                if !ship.weapon_recharge_queue.contains(&arc_index) {
                    ship.weapon_recharge_queue.push(arc_index);
                }
            }
        }
    }

    fn fighter_target(&self, source: usize) -> Option<OriginalFighterTarget> {
        let fighter = self.fighters.get(source)?;
        let requested = match (fighter.order, fighter.attack_target) {
            (
                TacticalOrder::AttackCapitalShips,
                Some(TacticalAttackTarget::CapitalShip(target)),
            ) => Some(OriginalFighterTarget::Capital(target)),
            (TacticalOrder::AttackFighters, Some(TacticalAttackTarget::FighterGroup(target))) => {
                Some(OriginalFighterTarget::Fighter(target))
            }
            // State 5 owns the timed trench-run producer. These fighters do
            // not chip the Death Star through the ordinary battery path.
            (TacticalOrder::AttackDeathStar, _) => return None,
            (TacticalOrder::AttackCapitalShips | TacticalOrder::AttackFighters, _) => return None,
            _ => None,
        };
        if let Some(target) = requested {
            return self
                .fighter_target_in_range(source, target)
                .then_some(target);
        }

        // Autonomous executors retain the first target on equal strength.
        let mut selected = None;
        let mut selected_strength = 0.0;
        for (target, ship) in self.ships.iter().enumerate() {
            if !ship.alive || ship.retreating || ship.is_attacker == fighter.is_attacker {
                continue;
            }
            let candidate = OriginalFighterTarget::Capital(target);
            let strength = self.fighter_target_strength(source, candidate);
            if selected_strength < strength {
                selected = Some(candidate);
                selected_strength = strength;
            }
        }
        for (target, opponent) in self.fighters.iter().enumerate() {
            if target == source
                || !opponent.alive
                || opponent.squad_count == 0
                || opponent.recovery_state == TacticalFighterRecoveryState::Recovered
                || opponent.is_attacker == fighter.is_attacker
            {
                continue;
            }
            let candidate = OriginalFighterTarget::Fighter(target);
            let strength = self.fighter_target_strength(source, candidate);
            if selected_strength < strength {
                selected = Some(candidate);
                selected_strength = strength;
            }
        }
        selected
    }

    fn fighter_target_in_range(&self, source: usize, target: OriginalFighterTarget) -> bool {
        self.fighter_target_strength(source, target) > 0.0
    }

    /// `FUN_005b7780` scores one concrete target: weapon strength per family
    /// in range, times the maneuverability ratio clamped at 0.1 against a
    /// fighter, plus the torpedo rating against an unshielded target.
    fn fighter_target_strength(&self, source: usize, target: OriginalFighterTarget) -> f32 {
        let Some(fighter) = self
            .fighters
            .get(source)
            .filter(|fighter| {
                fighter.alive && fighter.squad_count > 0 && fighter.listed() && fighter.weapon_arc.ready
            })
        else {
            return 0.0;
        };
        let (position, target_is_fighter, target_maneuverability, target_unshielded) = match target
        {
            OriginalFighterTarget::Capital(index) => {
                let Some(ship) = self.ships.get(index).filter(|ship| {
                    ship.alive && !ship.retreating && ship.is_attacker != fighter.is_attacker
                }) else {
                    return 0.0;
                };
                (ship.source_position, false, 1.0, ship.shield <= 0)
            }
            OriginalFighterTarget::Fighter(index) => {
                let Some(opponent) = self.fighters.get(index).filter(|opponent| {
                    opponent.alive
                        && opponent.squad_count > 0
                        && opponent.recovery_state != TacticalFighterRecoveryState::Recovered
                        && opponent.listed()
                        && opponent.is_attacker != fighter.is_attacker
                }) else {
                    return 0.0;
                };
                (
                    opponent.source_position,
                    true,
                    opponent.effective_maneuverability(),
                    false,
                )
            }
            OriginalFighterTarget::DeathStar => {
                let Some(death_star) = self.death_star.filter(|death_star| {
                    death_star.operational() && death_star.is_attacker != fighter.is_attacker
                }) else {
                    return 0.0;
                };
                (death_star.source_position, false, 1.0, true)
            }
        };
        let delta = source_delta(position, fighter.source_position);
        let distance = delta.x.hypot(delta.y).hypot(delta.z);
        let maneuver_modifier = if target_is_fighter {
            (fighter.effective_maneuverability() / target_maneuverability).max(0.1)
        } else {
            1.0
        };
        let mut strength = 0.0;
        for kind in [
            WeaponKind::IonCannon,
            WeaponKind::LaserCannon,
            WeaponKind::Turbolaser,
        ] {
            let count = original_weapon_count(fighter.weapon_arc, kind);
            if count > 0 && distance <= original_fighter_weapon_range(fighter, kind) {
                strength +=
                    original_fighter_weapon_strength(fighter, count, kind, target_is_fighter)
                        * maneuver_modifier;
            }
        }
        if !target_is_fighter
            && target_unshielded
            && fighter.torpedo_strength > 0
            && distance <= fighter.torpedo_range
        {
            strength += fighter.torpedo_term();
        }
        strength
    }

    /// Fire each live fighter group through the source fore battery record.
    fn fighter_step(&mut self) {
        let mut effects = Vec::new();
        let player_is_attacker = self.player_is_attacker;
        let player_faction = self.player_voice_faction();
        for source in 0..self.fighters.len() {
            let Some(target) = self.fighter_target(source) else {
                continue;
            };
            let source_snapshot = self.fighters[source].clone();
            let target_position = match target {
                OriginalFighterTarget::Capital(index) => self.ships[index].source_position,
                OriginalFighterTarget::Fighter(index) => self.fighters[index].source_position,
                OriginalFighterTarget::DeathStar => {
                    let Some(death_star) = self.death_star else {
                        continue;
                    };
                    death_star.source_position
                }
            };
            let delta = source_delta(target_position, source_snapshot.source_position);
            let distance = delta.x.hypot(delta.y).hypot(delta.z);
            let target_is_fighter = matches!(target, OriginalFighterTarget::Fighter(_));
            let target_was_unshielded = match target {
                OriginalFighterTarget::Capital(index) => self.ships[index].shield <= 0,
                OriginalFighterTarget::Fighter(_) => false,
                OriginalFighterTarget::DeathStar => true,
            };
            let mut fired = false;

            // Preserve the executable family-event order used by FUN_005b3f10.
            for kind in [
                WeaponKind::IonCannon,
                WeaponKind::LaserCannon,
                WeaponKind::Turbolaser,
            ] {
                let count = original_weapon_count(source_snapshot.weapon_arc, kind);
                if count == 0 || distance > original_fighter_weapon_range(&source_snapshot, kind) {
                    continue;
                }
                let strength = original_fighter_weapon_strength(
                    &source_snapshot,
                    count,
                    kind,
                    target_is_fighter,
                );
                if strength <= 0.0 {
                    continue;
                }
                match target {
                    OriginalFighterTarget::Capital(index) => {
                        let target_is_attacker = self.ships[index].is_attacker;
                        let target_group = self.ships[index].task_force;
                        let rolls = original_windowed_damage_rolls(
                            &self.ships,
                            index,
                            original_tactical_integer_damage(strength),
                            &mut self.tactical_rng,
                            player_is_attacker,
                        );
                        let stage = apply_original_capital_damage(
                            &mut self.ships,
                            &mut self.field_effects,
                            index,
                            original_tactical_integer_damage(strength),
                            rolls,
                        );
                        queue_original_destruction_voice(
                            &mut self.pending_voice_cues,
                            player_faction,
                            player_is_attacker,
                            source_snapshot.is_attacker,
                            TacticalVoiceGroup::Fighter(source_snapshot.fighter_group),
                            target_is_attacker,
                            TacticalVoiceGroup::Capital(target_group),
                            stage,
                        );
                        queue_original_tactical_impact(
                            &mut self.impact_effects,
                            index,
                            kind,
                            stage,
                        );
                        queue_original_fighter_projectile(
                            &mut effects,
                            &self.fighters,
                            source,
                            Some((index, &self.ships[index])),
                            None,
                            kind,
                            strength,
                        );
                    }
                    OriginalFighterTarget::Fighter(index) => {
                        let target_is_attacker = self.fighters[index].is_attacker;
                        let target_group = self.fighters[index].fighter_group;
                        let stage = apply_original_fighter_damage(
                            &mut self.fighters,
                            index,
                            strength,
                            source_snapshot.effective_maneuverability(),
                        );
                        queue_original_destruction_voice(
                            &mut self.pending_voice_cues,
                            player_faction,
                            player_is_attacker,
                            source_snapshot.is_attacker,
                            TacticalVoiceGroup::Fighter(source_snapshot.fighter_group),
                            target_is_attacker,
                            TacticalVoiceGroup::Fighter(target_group),
                            stage,
                        );
                        queue_original_fighter_projectile(
                            &mut effects,
                            &self.fighters,
                            source,
                            None,
                            Some((index, &self.fighters[index])),
                            kind,
                            strength,
                        );
                    }
                    OriginalFighterTarget::DeathStar => {
                        let Some(death_star) = self.death_star.as_mut() else {
                            continue;
                        };
                        death_star.hull = (death_star.hull - strength).max(0.0);
                        let snapshot = *death_star;
                        queue_original_fighter_to_death_star_projectile(
                            &mut effects,
                            &self.fighters,
                            source,
                            snapshot,
                            kind,
                            strength,
                        );
                    }
                }
                let fire_event = TacticalAudioEvent::fire_for(kind);
                let impact_event = TacticalAudioEvent::impact_for(kind);
                self.pending_audio_cues.push(TacticalAudioCue::from_variant(
                    fire_event,
                    u8::try_from(
                        self.tactical_rng
                            .inclusive(0, u32::from(fire_event.variant_count() - 1)),
                    )
                    .unwrap_or(0),
                ));
                self.pending_audio_cues.push(TacticalAudioCue::from_variant(
                    impact_event,
                    u8::try_from(
                        self.tactical_rng
                            .inclusive(0, u32::from(impact_event.variant_count() - 1)),
                    )
                    .unwrap_or(0),
                ));
                fired = true;
            }

            if matches!(
                target,
                OriginalFighterTarget::Capital(_) | OriginalFighterTarget::DeathStar
            ) && target_was_unshielded
                && source_snapshot.torpedo_strength > 0
                && distance <= source_snapshot.torpedo_range
            {
                let strength = source_snapshot.torpedo_term();
                if let OriginalFighterTarget::Capital(index) = target {
                    let target_is_attacker = self.ships[index].is_attacker;
                    let target_group = self.ships[index].task_force;
                    let rolls = original_windowed_damage_rolls(
                        &self.ships,
                        index,
                        original_tactical_integer_damage(strength),
                        &mut self.tactical_rng,
                        player_is_attacker,
                    );
                    let stage = apply_original_capital_damage(
                        &mut self.ships,
                        &mut self.field_effects,
                        index,
                        original_tactical_integer_damage(strength),
                        rolls,
                    );
                    queue_original_destruction_voice(
                        &mut self.pending_voice_cues,
                        player_faction,
                        player_is_attacker,
                        source_snapshot.is_attacker,
                        TacticalVoiceGroup::Fighter(source_snapshot.fighter_group),
                        target_is_attacker,
                        TacticalVoiceGroup::Capital(target_group),
                        stage,
                    );
                    queue_original_tactical_impact(
                        &mut self.impact_effects,
                        index,
                        WeaponKind::FighterAttack,
                        stage,
                    );
                    queue_original_fighter_projectile(
                        &mut effects,
                        &self.fighters,
                        source,
                        Some((index, &self.ships[index])),
                        None,
                        WeaponKind::FighterAttack,
                        strength,
                    );
                    fired = true;
                } else if target == OriginalFighterTarget::DeathStar {
                    let Some(death_star) = self.death_star.as_mut() else {
                        continue;
                    };
                    death_star.hull = (death_star.hull - strength).max(0.0);
                    let snapshot = *death_star;
                    queue_original_fighter_to_death_star_projectile(
                        &mut effects,
                        &self.fighters,
                        source,
                        snapshot,
                        WeaponKind::FighterAttack,
                        strength,
                    );
                    fired = true;
                }
                self.pending_audio_cues.push(TacticalAudioCue::from_variant(
                    TacticalAudioEvent::TorpedoFire,
                    u8::try_from(self.tactical_rng.inclusive(0, 2)).unwrap_or(0),
                ));
                self.pending_audio_cues.push(TacticalAudioCue::from_variant(
                    TacticalAudioEvent::TorpedoImpact,
                    0,
                ));
            }
            if fired {
                self.fighters[source].consume_weapon_arc();
            }
        }
        self.weapon_effects.extend(effects);
    }

    fn has_active_trench_run(&self) -> bool {
        let Some(death_star) = self
            .death_star
            .filter(|death_star| death_star.operational())
        else {
            return false;
        };
        self.fighters.iter().any(|fighter| {
            fighter.alive
                && fighter.squad_count > 0
                && fighter.is_attacker != death_star.is_attacker
                && fighter.order == TacticalOrder::AttackDeathStar
        })
    }

    /// The computer side's AI controller pass (`FUN_005c8780.c:133-216`):
    /// while the side fights (state 0), every sixth update it decides. Out
    /// of strength (`computer_withdraws`) it withdraws (`:152-165`); then,
    /// its station operational, the Empire (side 1) charged to exactly
    /// 100.0 (`_DAT_0066cbd8`) fires at the first enemy capital ship of its
    /// target list still in the battle (`FUN_005ad870`), and the Alliance,
    /// once, with no run under way, gives its head group order 6 when
    /// `FUN_005ad930` holds, or withdraws with no squadron left and
    /// `FUN_005ad930` failing (`:187-197`).
    /// port: the controller updates once per step; its target list is the
    /// enemy ships in battle order, and its head group the lowest fighter
    /// group with squadrons in the battle.
    pub(crate) fn advance_computer_controller(&mut self) {
        let computer_is_attacker = !self.player_is_attacker;
        let side = usize::from(!computer_is_attacker);
        if self.morale.withdrawal[side].state != TacticalWithdrawalState::Fighting {
            return;
        }
        self.death_star_ai.countdown -= 1.0;
        if self.death_star_ai.countdown > 0.0 {
            return;
        }
        self.death_star_ai.countdown = ORIGINAL_AI_DECISION_UPDATES;
        // A trench run exists (`FUN_005d0ae0`, `DAT_006bc978`).
        let run_exists = self.trench_run_group.is_some() || self.trench_run_outcome.is_some();
        if !run_exists && self.computer_withdraws(computer_is_attacker) {
            self.begin_side_withdrawal(computer_is_attacker);
        }
        let Some(death_star) = self.death_star.filter(|death_star| death_star.operational())
        else {
            return;
        };
        let computer_is_alliance = computer_is_attacker == self.attacker_is_alliance;
        if !computer_is_alliance {
            if death_star.is_attacker == computer_is_attacker
                && death_star.laser_charge == ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE
            {
                if let Some(target) = self.ships.iter().position(|ship| {
                    ship.alive && !ship.retreating && ship.is_attacker != computer_is_attacker
                }) {
                    self.commit_death_star_shot_for(computer_is_attacker, target);
                }
            }
            return;
        }
        if self.death_star_ai.ordered_run || run_exists {
            return;
        }
        if !self.trench_run_order_available() {
            let squadrons = self
                .fighters
                .iter()
                .any(|fighter| fighter.is_attacker == computer_is_attacker && fighter.alive);
            if !squadrons && self.computer_may_withdraw(computer_is_attacker) {
                self.begin_side_withdrawal(computer_is_attacker);
            }
            return;
        }
        let Some(group) = self
            .fighters
            .iter()
            .filter(|fighter| {
                fighter.is_attacker == computer_is_attacker
                    && fighter.alive
                    && fighter.squad_count > 0
                    && fighter.fighter_group < 4
            })
            .map(|fighter| fighter.fighter_group)
            .min()
        else {
            return;
        };
        self.death_star_ai.ordered_run = true;
        let members = (0..self.fighters.len())
            .filter(|&index| {
                let fighter = &self.fighters[index];
                fighter.is_attacker == computer_is_attacker && fighter.fighter_group == group
            })
            .collect::<Vec<_>>();
        for &index in &members {
            self.fighters[index].order = TacticalOrder::AttackDeathStar;
        }
        begin_original_attack_order(self, &[], &members, TacticalOrder::AttackDeathStar);
    }

    /// `FUN_005c8780.c:152-160`: the opponent's strength above 0.0 (through
    /// `FUN_005a9610`), the side's own over it below the threshold
    /// (`+0x24`) or exactly 0.0, and the side free to withdraw. The
    /// windowed strength `FUN_005c7f70` is the headless `FUN_005d0ce0`
    /// step for step (`stalemate_strength`), its station term
    /// `_DAT_0066cba0` the same 20000000.0 as `_DAT_0066d0f0`.
    fn computer_withdraws(&self, computer_is_attacker: bool) -> bool {
        let side = usize::from(!computer_is_attacker);
        let opponent = self.stalemate_strength(!computer_is_attacker);
        if opponent <= 0.0 {
            return false;
        }
        let ratio = self.stalemate_strength(computer_is_attacker) / opponent;
        (ratio < self.morale.threshold[side] || ratio == 0.0)
            && self.computer_may_withdraw(computer_is_attacker)
    }

    /// `FUN_005c7e80` (vtable `+0xc`), the windowed twin of `FUN_005d0f50`:
    /// permitted and still in state 0, with a way out (`may_retreat`).
    fn computer_may_withdraw(&self, computer_is_attacker: bool) -> bool {
        let side = usize::from(!computer_is_attacker);
        self.morale.withdrawal[side].state == TacticalWithdrawalState::Fighting
            && self.may_retreat(computer_is_attacker)
    }

    /// `FUN_0040b0e0.c:184-203`: with no station raised from a fleet, the
    /// system's production entry at queue position 0
    /// (`FUN_00509670(system, 0)`) whose product is a Death Star (family
    /// `[0x18,0x1c)`) not yet completed (`+0x50` bit 2 clear) is exported as
    /// an uncompleted station (`+0x74 = 1`, `FUN_0040bcd0.c:62`): sprite
    /// 5020, charge 100.0 that never advances, the system's shield, and the
    /// class hull (hyp: the unbuilt object holds no damage).
    /// port: the queue heads are read in [`ProductionArea::ALL`] order (the
    /// order of the source's queue children is untraced); the station takes
    /// no squadrons (`FUN_005b8570` docks none at an unfinished one in
    /// practice, `ghidra/notes/tactical-death-star.md` section 9).
    #[expect(
        clippy::cast_precision_loss,
        reason = "The source stores the hulls as floats."
    )]
    pub fn seat_unfinished_station(
        &mut self,
        world: &GameWorld,
        manufacturing: &crate::manufacturing::ManufacturingState,
    ) {
        if self.death_star.is_some() {
            return;
        }
        let Some(class) = crate::manufacturing::ProductionArea::ALL
            .iter()
            .find_map(|&area| manufacturing.queue(self.system, area)?.items().front())
            .and_then(|item| match item.kind {
                crate::manufacturing::BuildableKind::CapitalShip(class) => {
                    world.capital_ship_classes.get(class)
                }
                _ => None,
            })
            .filter(|class| class.is_death_star())
        else {
            return;
        };
        let is_alliance = class.is_alliance;
        let is_attacker = self.attacker_is_alliance == is_alliance;
        let mut station =
            TacticalDeathStar::seated(world, self.system, is_alliance, is_attacker, self.source_layout);
        station.hull_max = class.hull as f32;
        station.hull = station.hull_max;
        station.not_completed = true;
        self.death_star = Some(station);
    }

    /// `FUN_005b8570.c:13-30`: each squadron that docked into its carrier's
    /// launch list at setup (`FUN_005964c0.c:149-153`) arms an unarmed
    /// hangar at `5.0 + rand(0,8)` past the step, which the battle
    /// constructor sets to 3 (`battle+0x87c`, `FUN_005a7500.c:551`), in
    /// docking order; only a battle with its window does
    /// (`FUN_005b8570.c:20-21`, engine `+0x668 == 0`).
    /// port: the session exists before the Battle Alert's choice, so Take
    /// Command arms it, with no draw of the stream before it.
    pub fn arm_docked_hangars(&mut self) {
        self.hangars.ships.resize(self.ships.len(), 0.0);
        // FUN_005ab650.c:148-153: a docked squadron takes no slot and no
        // share of the battle extent; it is out of the live units.
        for side_alliance in [true, false] {
            let docked = self
                .fighters
                .iter()
                .filter(|f| f.identity.is_alliance == side_alliance && f.docked.is_some())
                .count();
            let live_docked = self
                .fighters
                .iter()
                .filter(|f| f.identity.is_alliance == side_alliance && f.docked.is_some() && f.alive)
                .count();
            self.pending_log_lines.push(format!(
                "[tactical_hangar] side={} status=seated docked={docked} live_docked={live_docked} source=FUN_005ab650.c:148-153",
                if side_alliance { "alliance" } else { "empire" },
            ));
        }
        for index in 0..self.fighters.len() {
            let fighter = &self.fighters[index];
            let Some(berth) = fighter.carrier else {
                continue;
            };
            if fighter.docked != Some(TacticalDock::Ready) || self.launch_step(berth) != 0.0 {
                continue;
            }
            let side = if fighter.identity.is_alliance { "alliance" } else { "empire" };
            let armed = self.hangar_arm_step();
            self.set_launch_step(berth, armed);
            self.pending_log_lines.push(format!(
                "[tactical_hangar] side={side} carrier={berth:?} status=armed launch_step={armed} step={} source=FUN_005b8570",
                self.hangars.step,
            ));
        }
    }

    /// Hangar `+0x14`, the launch step of `berth`'s hangar.
    fn launch_step(&self, berth: TacticalBerth) -> f32 {
        match berth {
            TacticalBerth::Station => self.hangars.station,
            TacticalBerth::Ship(index) => self.hangars.ships.get(index).copied().unwrap_or(0.0),
        }
    }

    fn set_launch_step(&mut self, berth: TacticalBerth, step: f32) {
        match berth {
            TacticalBerth::Station => self.hangars.station = step,
            TacticalBerth::Ship(index) => {
                self.hangars.ships.resize(self.ships.len(), 0.0);
                self.hangars.ships[index] = step;
            }
        }
    }

    /// `FUN_005b8800.c:29-36`, `FUN_005b8570.c:22-29`: a hangar arms at
    /// `+0x18` (5.0, `FUN_005b8b00`) plus `FUN_005a8a70(0, 8)` past the
    /// step `battle+0x87c`.
    fn hangar_arm_step(&mut self) -> f32 {
        ORIGINAL_HANGAR_LAUNCH_STEPS
            + self.tactical_rng.inclusive(0, ORIGINAL_HANGAR_LAUNCH_SPREAD) as f32
            + self.hangars.step as f32
    }

    /// The carriers' launches, once per 1000 ms battle step
    /// (`FUN_005bea30.c:121` calls `FUN_005a9de0` before raising the step
    /// counter at `:125`). The station's hangar runs only for a local
    /// Empire (`battle+0x9ec == 1`) and while it is operational
    /// (`FUN_005a9de0.c:14-17`); every capital ship still in the battle runs
    /// its own (`:34-36`).
    /// port: the step accumulates the session's own step length.
    fn advance_hangars(&mut self, delta_milliseconds: f32) {
        self.hangars.elapsed_ms += delta_milliseconds.max(0.0);
        while self.hangars.elapsed_ms >= ORIGINAL_BATTLE_STEP_MILLISECONDS {
            self.hangars.elapsed_ms -= ORIGINAL_BATTLE_STEP_MILLISECONDS;
            let local_is_alliance = self.player_is_attacker == self.attacker_is_alliance;
            if !local_is_alliance
                && self.death_star.is_some_and(|death_star| death_star.operational())
            {
                self.advance_hangar(TacticalBerth::Station);
            }
            for index in 0..self.ships.len() {
                if self.ships[index].alive && !self.ships[index].retreating {
                    self.advance_hangar(TacticalBerth::Ship(index));
                }
            }
            self.hangars.step += 1;
        }
    }

    /// `FUN_005b8800`: an empty launch list does nothing; an unarmed
    /// hangar arms its launch step at `5.0 + rand(0,8)` past the current
    /// step; once the step reaches it the front squadron launches (message
    /// `0x19`, `FUN_005b8c70`: state 2) and the hangar re-arms.
    /// port: the launch message takes effect at once, from the carrier's
    /// position (hyp: the lockstep queue's delay and the launch point are
    /// untraced).
    fn advance_hangar(&mut self, berth: TacticalBerth) {
        let Some(front) = (0..self.fighters.len()).find(|&index| {
            let fighter = &self.fighters[index];
            fighter.squad_count > 0
                && fighter.carrier == Some(berth)
                && fighter.docked == Some(TacticalDock::Ready)
        }) else {
            return;
        };
        let launch_step = self.launch_step(berth);
        if launch_step != 0.0 {
            if (self.hangars.step as f32) < launch_step {
                return;
            }
            let position = match berth {
                TacticalBerth::Station => self
                    .death_star
                    .map_or(TacticalWorldPosition::ORIGIN, |station| station.source_position),
                TacticalBerth::Ship(index) => self.ships[index].source_position,
            };
            let fighter = &mut self.fighters[front];
            fighter.docked = None;
            fighter.alive = true;
            fighter.source_position = position;
            let side = if fighter.identity.is_alliance { "alliance" } else { "empire" };
            self.pending_log_lines.push(format!(
                "[tactical_hangar] side={side} carrier={berth:?} squadron={front} status=launched step={} source=FUN_005d0820",
                self.hangars.step,
            ));
        }
        let armed = self.hangar_arm_step();
        self.set_launch_step(berth, armed);
    }

    /// hyp: a squadron docked in a capital ship destroyed in the battle is
    /// lost with it, as the station's destruction destroys its hangar
    /// (`FUN_005b8d80.c:8-19`); a ship's own loss of its hangar is untraced.
    fn destroy_squadrons_docked_in_lost_ships(&mut self) {
        for fighter in &mut self.fighters {
            let Some(TacticalBerth::Ship(index)) = fighter.carrier else {
                continue;
            };
            if fighter.docked.is_some()
                && self
                    .ships
                    .get(index)
                    .is_some_and(|ship| !ship.alive && !ship.retreated && !ship.retreating)
            {
                fighter.destroy();
            }
        }
    }

    /// `FUN_005ad930.c:15-49`, the test behind the Attack Death Star button
    /// (`FUN_00596bb0`, `FUN_005df7a0.c:19-34`) and the Alliance AI's order 6:
    /// the station attackable (`FUN_005ba450`), no run under way
    /// (`DAT_006bc978`), the Alliance's listed squadrons strictly
    /// outnumbering the Empire's, and no Alliance group under order 6.
    #[must_use]
    pub fn trench_run_order_available(&self) -> bool {
        let Some(death_star) = self.death_star.filter(|death_star| death_star.attackable())
        else {
            return false;
        };
        let listed = |is_alliance: bool| {
            self.fighters
                .iter()
                .filter(|fighter| {
                    fighter.identity.is_alliance == is_alliance
                        && fighter.alive
                        && fighter.squad_count > 0
                        && fighter.listed()
                })
                .count()
        };
        !death_star.is_alliance
            && self.trench_run_group.is_none()
            && self.trench_run_outcome.is_none()
            && listed(true) > listed(false)
            && !self.fighters.iter().any(|fighter| {
                fighter.identity.is_alliance
                    && fighter.alive
                    && fighter.order == TacticalOrder::AttackDeathStar
            })
    }

    /// The point a trench group flies to (`FUN_005c6610.c:112-132`): its
    /// leader steers along `(-z, 0, 0)` of its side's facing
    /// (`FUN_005a7490`; the Alliance, side 0, faces +z, the Empire -z) to
    /// `0.5` (`_DAT_0066cb14`) times the battle extent (`battle+0x8cc`).
    #[must_use]
    pub fn trench_run_entry_point(&self, is_alliance: bool) -> TacticalWorldPosition {
        let facing_z = if is_alliance { 1.0 } else { -1.0 };
        TacticalWorldPosition {
            x: -facing_z * ORIGINAL_TRENCH_RUN_ENTRY_SCALE * self.source_layout.battle_extent,
            y: 0.0,
            z: 0.0,
        }
    }

    /// State 2 (`FUN_005c6610.c:112-145`): the group under order 6 flies to
    /// its entry point at its engine speed times 0.95 (`FUN_005eb1a0`); once
    /// its leader is within 0.2 of it (`0x3e4ccccd`, the arrival distance)
    /// the result enters state 5 (`FUN_005cfe80(result, 5)` at `0x5c6924`):
    /// the state-5 voice, the two-minute timer and the roll.
    /// port: only the leader's steering is traced; the other squadrons of
    /// the group fly the same way. hyp: a group lost before it arrives ends
    /// the run as a failure.
    pub fn advance_trench_run_approach(&mut self, delta_milliseconds: f32) {
        let Some(group) = self.trench_run_group else {
            return;
        };
        if self.trench_run_remaining_ms.is_some()
            || self.trench_run_pending_outcome.is_some()
            || self.trench_run_outcome.is_some()
        {
            return;
        }
        let members = (0..self.fighters.len())
            .filter(|&index| {
                let fighter = &self.fighters[index];
                fighter.alive
                    && fighter.squad_count > 0
                    && fighter.identity.is_alliance
                    && fighter.fighter_group == group
                    && fighter.order == TacticalOrder::AttackDeathStar
            })
            .collect::<Vec<_>>();
        let Some(&leader) = members.first() else {
            self.record_trench_run_outcome(TacticalTrenchRunOutcome::Failure);
            return;
        };
        let target = self.trench_run_entry_point(true);
        let seconds = delta_milliseconds.max(0.0) * 0.001;
        for &index in &members {
            let fighter = &mut self.fighters[index];
            let delta = source_delta(target, fighter.source_position);
            let distance = delta.x.hypot(delta.y).hypot(delta.z);
            if distance <= ORIGINAL_TRENCH_RUN_ARRIVAL_DISTANCE {
                continue;
            }
            let travel = (fighter.sublight_engine_power.max(0.0)
                * ORIGINAL_TRENCH_RUN_SPEED_SCALE
                * seconds)
                .min(distance);
            let direction = delta.normalized();
            fighter.source_position.x += direction.x * travel;
            fighter.source_position.y += direction.y * travel;
            fighter.source_position.z += direction.z * travel;
        }
        let delta = source_delta(target, self.fighters[leader].source_position);
        if delta.x.hypot(delta.y).hypot(delta.z) > ORIGINAL_TRENCH_RUN_ARRIVAL_DISTANCE {
            return;
        }
        self.trench_run_remaining_ms = Some(ORIGINAL_TRENCH_RUN_DURATION_MILLISECONDS);
        if self.player_voice_faction() == TacticalVoiceFaction::Alliance {
            self.pending_voice_cues
                .push(TacticalVoiceCue::trench_run_committed(group));
        }
        self.prepare_original_trench_run();
    }

    /// Advance and resolve `TACTICALRESULT_UPDATE` state 5. `FUN_005cfec0`
    /// supplies the 120 second timer; `FUN_005d04e0` consumes the shared RNG
    /// for maneuver losses and the surviving torpedo attempt.
    pub fn advance_original_trench_run(&mut self, milliseconds: f32) {
        let Some(remaining) = self.trench_run_remaining_ms else {
            return;
        };
        if self.trench_run_outcome.is_some() {
            self.trench_run_remaining_ms = None;
            return;
        }
        if self.trench_run_pending_outcome.is_none() && !self.has_active_trench_run() {
            self.record_trench_run_outcome(TacticalTrenchRunOutcome::Failure);
            return;
        }
        let remaining = (remaining - milliseconds.max(0.0)).max(0.0);
        self.trench_run_remaining_ms = Some(remaining);
        self.queue_original_trench_run_chatter(remaining);
        if remaining == 0.0 {
            self.resolve_original_trench_run();
        }
    }

    /// The headless run's rating (`FUN_005afc00`): the Alliance
    /// Commander's while the object carrying it is one of the battle's
    /// (`FUN_005a6340`), destroyed or not, else 1.
    fn headless_trench_rating(&self) -> f32 {
        self.trench_run_commander
            .filter(|commander| {
                commander
                    .fleet
                    .is_some_and(|fleet| self.ships.iter().any(|ship| ship.fleet == Some(fleet)))
            })
            .map_or(1.0, TacticalTrenchCommander::rating)
    }

    /// `FUN_005ad7e0(manager, 0)`: `combat / 20 + 1` of the Alliance
    /// Commander, clamped to 1 through 9, while the ship carrying the
    /// character is in the battle (`FUN_005ad870`); else 1.0.
    /// port: a character rides a fleet, so a live ship of its fleet stands
    /// for its carrying ship; one the system holds rides none.
    #[must_use]
    pub fn trench_run_rating(&self) -> f32 {
        self.trench_run_commander
            .filter(|commander| {
                commander.fleet.is_some_and(|fleet| {
                    self.ships
                        .iter()
                        .any(|ship| ship.fleet == Some(fleet) && ship.alive)
                })
            })
            .map_or(1.0, TacticalTrenchCommander::rating)
    }

    /// `FUN_005d04e0`, run when state 5 begins. The participants are popped
    /// lowest id first (`thunk_FUN_005f5060`); each tests its maneuver
    /// (`rand(1,10) > trunc(m)` loses `rand(1,n)` of its n craft, or all
    /// with fewer than two), then, alive, rolls `rand(1,100)` against half
    /// the torpedo term plus the rating, truncated. A survivor goes back in
    /// under the same id, so it pops again; the loop ends on a success, an
    /// empty list or the 1001st pop. A success restores every participant's
    /// hull from the state-5 snapshot (`FUN_005d08d0`, `0x5d07f0`).
    /// port: the source keys the list by object id (`+0xec`); a squadron's
    /// place in `fighters` is its order of creation.
    fn prepare_original_trench_run(&mut self) {
        let Some(death_star) = self.death_star.filter(|death_star| death_star.operational())
        else {
            return;
        };
        let group = self.trench_run_group;
        let participants = (0..self.fighters.len())
            .filter(|&index| {
                let fighter = &self.fighters[index];
                fighter.alive
                    && fighter.squad_count > 0
                    && fighter.is_attacker != death_star.is_attacker
                    && fighter.order == TacticalOrder::AttackDeathStar
                    && Some(fighter.fighter_group) == group
            })
            .collect::<Vec<_>>();
        let snapshot = participants
            .iter()
            .map(|&index| self.fighters[index].hull_current)
            .collect::<Vec<_>>();
        for &index in &participants {
            self.fighters[index].off_field = true;
        }
        let rating = self.trench_run_rating().min(ORIGINAL_TRENCH_RUN_RATING_MAX);

        let mut list = participants.clone();
        let mut pops = 0;
        let mut success = false;
        let mut maneuver_damage = false;
        while let Some(&index) = list.first() {
            list.remove(0);
            let fighter = &mut self.fighters[index];
            let maneuver = fighter.trench_maneuver().trunc() as u32;
            if self.tactical_rng.inclusive(1, 10) > maneuver {
                maneuver_damage = true;
                let craft = (fighter.hull_current / ORIGINAL_TRENCH_RUN_DAMAGE_SCALE).trunc() as u32;
                let damage = if craft < 2 {
                    ORIGINAL_TRENCH_RUN_FATAL_DAMAGE
                } else {
                    self.tactical_rng.inclusive(1, craft) as f32 * ORIGINAL_TRENCH_RUN_DAMAGE_SCALE
                };
                fighter.hull_current = (fighter.hull_current - damage).max(0.0);
            }
            let alive = fighter.hull_current > 0.0;
            if alive {
                let chance = trench_run_success_chance(fighter, rating);
                if self.tactical_rng.inclusive(1, 100) <= chance {
                    success = true;
                    break;
                }
            }
            pops += 1;
            if pops > ORIGINAL_TRENCH_RUN_POP_LIMIT {
                break;
            }
            if alive {
                list.insert(0, index);
            }
        }
        for (&index, &hull) in participants.iter().zip(&snapshot) {
            let fighter = &mut self.fighters[index];
            if success {
                fighter.hull_current = hull;
            }
            fighter.refresh_craft_count();
        }

        self.trench_run_participants = participants;
        self.trench_run_snapshot = snapshot;
        self.trench_run_pending_outcome = Some(if success {
            TacticalTrenchRunOutcome::Success
        } else {
            TacticalTrenchRunOutcome::Failure
        });
        self.trench_run_chatter = match (success, maneuver_damage) {
            (true, false) => ORIGINAL_TRENCH_RUN_SUCCESS_CHATTER,
            (true, true) => ORIGINAL_TRENCH_RUN_DAMAGED_SUCCESS_CHATTER,
            (false, _) => ORIGINAL_TRENCH_RUN_FAILURE_CHATTER,
        };
        self.trench_run_chatter_cursor = 0;
        self.queue_original_trench_run_chatter(ORIGINAL_TRENCH_RUN_DURATION_MILLISECONDS);
    }

    /// Emit every `FUN_005d03f0` threshold crossed by this update. Production
    /// advances at 10 Hz, while deterministic fixtures may cross several
    /// thresholds in one call; preserving order makes both paths equivalent.
    fn queue_original_trench_run_chatter(&mut self, remaining_ms: f32) {
        while self.trench_run_chatter_cursor < self.trench_run_chatter.len() {
            let slot = self.trench_run_chatter_cursor;
            let threshold = ORIGINAL_TRENCH_RUN_DURATION_MILLISECONDS
                - slot as f32 * ORIGINAL_TRENCH_RUN_DURATION_MILLISECONDS * 0.1;
            if remaining_ms > threshold {
                break;
            }
            self.trench_run_chatter_cursor += 1;
            let source_event = self.trench_run_chatter[slot];
            if source_event != ORIGINAL_TRENCH_RUN_SILENT_EVENT {
                self.pending_voice_cues
                    .push(TacticalVoiceCue::trench_run_chatter(source_event));
            }
        }
    }

    /// Apply state 6 or 7 after the retained two-minute presentation window
    /// (`FUN_005cfec0`, `tactical-death-star-path.md`).
    fn resolve_original_trench_run(&mut self) {
        let Some(outcome) = self.trench_run_pending_outcome else {
            return;
        };
        let participants = self.trench_run_participants.clone();

        if outcome == TacticalTrenchRunOutcome::Success {
            // `FUN_005ba890` zeroes the hull and the charge; `+0x74` and
            // `+0x78` stay as they were.
            if let Some(death_star) = self.death_star.as_mut() {
                death_star.hull = 0.0;
                death_star.laser_charge = 0.0;
            }
            // `FUN_005ba890.c:5-7` -> `FUN_005b8d80.c:8-19`: every squadron
            // in the station's launch and recovered lists is destroyed;
            // those it launched fly on.
            for fighter in &mut self.fighters {
                if fighter.carrier == Some(TacticalBerth::Station) && fighter.docked.is_some() {
                    fighter.destroy();
                }
            }
            // State 6 removes the first bounded half of the participating list
            // before returning the survivors. Counts 0 and 1 lose none, 2 or
            // 3 lose one, and larger groups lose floor(n / 2).
            let casualties = match participants.len() {
                0 | 1 => 0,
                2 | 3 => 1,
                count => count / 2,
            };
            for &index in participants.iter().take(casualties) {
                self.fighters[index].hull_current = 0.0;
                self.fighters[index].refresh_craft_count();
            }
            // The survivors are listed again (`FUN_005a9380`, state 2).
            for &index in &participants {
                self.fighters[index].off_field = false;
            }
            self.record_trench_run_outcome(outcome);
        } else {
            for &index in &participants {
                self.fighters[index].hull_current = 0.0;
                self.fighters[index].refresh_craft_count();
            }
            self.record_trench_run_outcome(outcome);
        }
    }

    fn record_trench_run_outcome(&mut self, outcome: TacticalTrenchRunOutcome) {
        if self.trench_run_outcome.is_some() {
            return;
        }
        if self.player_is_attacker == self.attacker_is_alliance {
            let group = self.trench_run_group.or_else(|| {
                self.fighters
                    .iter()
                    .find(|fighter| {
                        fighter.identity.is_alliance
                            && fighter.order == TacticalOrder::AttackDeathStar
                    })
                    .map(|fighter| fighter.fighter_group)
            });
            self.pending_voice_cues
                .push(TacticalVoiceCue::trench_run_outcome(
                    group.unwrap_or(0),
                    outcome,
                ));
        } else {
            self.pending_voice_cues.push(match outcome {
                TacticalTrenchRunOutcome::Failure => {
                    TacticalVoiceCue::death_star_attack_broken_off()
                }
                TacticalTrenchRunOutcome::Success => TacticalVoiceCue::death_star_destroyed(),
            });
        }
        self.trench_run_outcome = Some(outcome);
        self.trench_run_remaining_ms = None;
        self.trench_run_pending_outcome = None;
        self.trench_run_participants.clear();
        self.trench_run_snapshot.clear();
        self.trench_run_cinematic_pending = true;
        self.paused = true;
    }

    pub fn take_pending_trench_run_cinematic(&mut self) -> Option<TacticalTrenchRunOutcome> {
        if !self.trench_run_cinematic_pending {
            return None;
        }
        self.trench_run_cinematic_pending = false;
        self.trench_run_outcome
    }

    pub fn take_pending_audio_cues(&mut self) -> Vec<TacticalAudioCue> {
        std::mem::take(&mut self.pending_audio_cues)
    }

    pub fn take_pending_voice_cues(&mut self) -> Vec<TacticalVoiceCue> {
        std::mem::take(&mut self.pending_voice_cues)
    }

    /// Queue the original faction-specific readiness call when command passes
    /// from Battle Alert into the active tactical manager.
    pub fn queue_battle_ready_voice(&mut self) {
        let faction = self.player_voice_faction();
        self.pending_voice_cues
            .push(TacticalVoiceCue::battle_ready(faction));
    }

    /// Queue the original player-faction withdrawal acknowledgement.
    pub fn queue_withdrawal_started_voice(&mut self) {
        self.pending_voice_cues
            .push(TacticalVoiceCue::withdrawal_started(
                self.player_voice_faction(),
            ));
    }

    /// Queue the warning recovered from `FUN_005a0240` when a withdrawing
    /// player fleet contains at least one live ship that cannot enter
    /// hyperspace. The source leaves those ships behind while operational
    /// hyperdrive ships continue the withdrawal.
    pub fn queue_withdrawal_hyperdrive_warning(&mut self) -> bool {
        let has_stranded_ship = self.ships.iter().any(|ship| {
            ship.alive
                && !ship.retreating
                && ship.is_attacker == self.player_is_attacker
                && (ship.subsystem_capacity.hyperdrive == 0
                    || ship.subsystem_condition.hyperdrive == 0)
        });
        if has_stranded_ship {
            self.pending_voice_cues
                .push(TacticalVoiceCue::withdrawal_hyperdrive_warning(
                    self.player_voice_faction(),
                ));
        }
        has_stranded_ship
    }

    pub const fn player_voice_faction(&self) -> TacticalVoiceFaction {
        if self.player_is_attacker == self.attacker_is_alliance {
            TacticalVoiceFaction::Alliance
        } else {
            TacticalVoiceFaction::Empire
        }
    }

    /// The outcome voice (`FUN_005a9ec0.c:291-328`), from the battle's end
    /// flags rather than the recorded winner: each side's loss (nothing
    /// left, `:274-286`, both losses cleared together, `:287-290`) and its
    /// withdrawal (`side_withdrew`). For the local side: the other side lost
    /// and withdrew, "won, they withdrew" (0x97, 0x119); else the local side
    /// withdrew, "withdrawal complete" (0x96, 0x118); else the other side
    /// lost, "won" (0x98, 0x11a); else the local side lost (0x99, 0x11b);
    /// else none.
    fn queue_battle_outcome_voice(&mut self) {
        let local = self.player_is_attacker;
        let lost = [!self.side_remains(local), !self.side_remains(!local)];
        let [local_lost, other_lost] = if lost[0] && lost[1] { [false; 2] } else { lost };
        let faction = self.player_voice_faction();
        let cue = if other_lost && self.side_withdrew(!local) {
            TacticalVoiceCue::battle_won(faction, true)
        } else if self.side_withdrew(local) {
            TacticalVoiceCue::withdrawal_complete(faction)
        } else if other_lost {
            TacticalVoiceCue::battle_won(faction, false)
        } else if local_lost {
            TacticalVoiceCue::battle_lost(faction)
        } else {
            return;
        };
        self.pending_voice_cues.push(cue);
    }
}

// ---------------------------------------------------------------------------
// TacticalState — top-level UI state
// ---------------------------------------------------------------------------

pub fn source_delta(to: TacticalWorldPosition, from: TacticalWorldPosition) -> TacticalWorldVector {
    TacticalWorldVector {
        x: to.x - from.x,
        y: to.y - from.y,
        z: to.z - from.z,
    }
}

fn original_maneuver_waypoint(
    order: TacticalOrder,
    current: TacticalWorldPosition,
    target: TacticalWorldPosition,
) -> Option<TacticalWorldPosition> {
    let delta = source_delta(target, current);
    let (sin, cos) = match order {
        TacticalOrder::LeftHook | TacticalOrder::Anvil => ORIGINAL_MANEUVER_ANGLE.sin_cos(),
        TacticalOrder::RightHook | TacticalOrder::Hammer => (-ORIGINAL_MANEUVER_ANGLE).sin_cos(),
        _ => return None,
    };
    let rotated = match order {
        TacticalOrder::LeftHook | TacticalOrder::RightHook => TacticalWorldVector {
            x: cos.mul_add(delta.x, -sin * delta.z),
            y: delta.y,
            z: cos.mul_add(delta.z, sin * delta.x),
        },
        TacticalOrder::Hammer | TacticalOrder::Anvil => TacticalWorldVector {
            x: delta.x,
            y: cos.mul_add(delta.y, sin * delta.z),
            z: cos.mul_add(delta.z, -sin * delta.y),
        },
        _ => return None,
    };
    Some(TacticalWorldPosition {
        x: current.x + rotated.x * ORIGINAL_MANEUVER_WAYPOINT_SCALE,
        y: current.y + rotated.y * ORIGINAL_MANEUVER_WAYPOINT_SCALE,
        z: current.z + rotated.z * ORIGINAL_MANEUVER_WAYPOINT_SCALE,
    })
}

fn original_maneuver_target(
    ships: &[TacticalShip],
    source: usize,
    order: TacticalOrder,
) -> Option<TacticalWorldPosition> {
    let source_ship = ships.get(source)?;
    let opponents = ships
        .iter()
        .filter(|ship| ship.alive && ship.is_attacker != source_ship.is_attacker)
        .collect::<Vec<_>>();
    let count = opponents.len() as f32;
    if count == 0.0 {
        return None;
    }
    let anchor = opponents
        .iter()
        .fold(TacticalWorldPosition::ORIGIN, |sum, ship| {
            TacticalWorldPosition {
                x: sum.x + ship.source_position.x / count,
                y: sum.y + ship.source_position.y / count,
                z: sum.z + ship.source_position.z / count,
            }
        });
    let delta = source_delta(anchor, source_ship.source_position);
    let selector = match order {
        TacticalOrder::LeftHook => TacticalWorldVector {
            x: -delta.z,
            y: delta.y,
            z: delta.x,
        },
        TacticalOrder::RightHook => TacticalWorldVector {
            x: delta.z,
            y: delta.y,
            z: -delta.x,
        },
        TacticalOrder::Hammer => TacticalWorldVector {
            x: delta.x,
            y: -delta.y,
            z: delta.z,
        },
        TacticalOrder::Anvil => delta,
        _ => return None,
    }
    .normalized();
    let mut selected = None;
    let mut selected_score = f32::NEG_INFINITY;
    for opponent in opponents {
        let position = opponent.source_position;
        let score = selector.dot(TacticalWorldVector {
            x: position.x,
            y: position.y,
            z: position.z,
        });
        if score > selected_score {
            selected = Some(position);
            selected_score = score;
        }
    }
    selected
}

pub fn begin_original_capital_order(
    session: &mut BattleSession,
    selected: &[usize],
    order: TacticalOrder,
) {
    for &index in selected {
        if order == TacticalOrder::HoldPosition {
            let ship = &mut session.ships[index];
            ship.source_waypoint = None;
            ship.navigation_route.clear();
            ship.source_desired_forward = TacticalWorldVector::ZERO;
            ship.source_velocity = TacticalWorldVector::ZERO;
            continue;
        }
        if !matches!(
            order,
            TacticalOrder::LeftHook
                | TacticalOrder::RightHook
                | TacticalOrder::Hammer
                | TacticalOrder::Anvil
        ) {
            continue;
        }
        let current = session.ships[index].source_position;
        let waypoint = original_maneuver_target(&session.ships, index, order)
            .and_then(|target| original_maneuver_waypoint(order, current, target));
        let ship = &mut session.ships[index];
        ship.navigation_route.clear();
        if let Some(waypoint) = waypoint {
            ship.source_waypoint = Some(waypoint);
            ship.source_desired_forward = source_delta(waypoint, current).normalized();
        } else {
            // `FUN_005ca6d0` returns the object to no-orders when no executor
            // target can be created.
            ship.order = TacticalOrder::None;
            ship.source_waypoint = None;
            ship.source_desired_forward = TacticalWorldVector::ZERO;
            ship.source_velocity = TacticalWorldVector::ZERO;
        }
    }
}

/// Apply the original direct Escort order (source order code 1). The manual
/// assigns it by right-clicking a friendly capital ship, outside either
/// assignment panel. The recovered order-1 executor retains that object in its
/// target slot.
pub fn assign_selected_escort(session: &mut BattleSession, target: usize) -> (usize, usize) {
    let Some(target_ship) = session
        .ships
        .get(target)
        .filter(|ship| ship.alive && !ship.retreating)
    else {
        return (0, 0);
    };
    let target_side = target_ship.is_attacker;
    if target_side != session.player_is_attacker {
        return (0, 0);
    }

    let mut capitals = 0;
    for (index, ship) in session.ships.iter_mut().enumerate() {
        if index != target
            && ship.alive
            && !ship.retreating
            && ship.selected
            && ship.is_attacker == target_side
        {
            ship.order = TacticalOrder::Escort;
            ship.escort_target = Some(target);
            ship.attack_target = None;
            ship.manual_targets.clear();
            ship.navigation_route.clear();
            capitals += 1;
        }
    }

    let mut fighters = 0;
    for fighter in &mut session.fighters {
        if fighter.alive
            && fighter.squad_count > 0
            && fighter.selected
            && fighter.is_attacker == target_side
        {
            fighter.order = TacticalOrder::Escort;
            fighter.escort_target = Some(target);
            fighter.attack_target = None;
            fighter.recovery_state = TacticalFighterRecoveryState::AwaitingCarrier;
            fighter.recovery_target = None;
            fighters += 1;
        }
    }
    (capitals, fighters)
}

fn escort_target_is_eligible(session: &BattleSession, source_side: bool, target: usize) -> bool {
    session
        .ships
        .get(target)
        .is_some_and(|ship| ship.alive && !ship.retreating && ship.is_attacker == source_side)
}

/// Clear order-1 target slots when the protected hull is destroyed or leaves
/// combat. This mirrors the executor's object-lifetime boundary and prevents a
/// stale white escort box.
fn refresh_original_escort_targets(session: &mut BattleSession) {
    for index in 0..session.ships.len() {
        if session.ships[index].order != TacticalOrder::Escort {
            session.ships[index].escort_target = None;
            continue;
        }
        let valid = session.ships[index].escort_target.is_some_and(|target| {
            target != index
                && escort_target_is_eligible(session, session.ships[index].is_attacker, target)
        });
        if !valid {
            let ship = &mut session.ships[index];
            ship.order = TacticalOrder::None;
            ship.escort_target = None;
            ship.source_waypoint = None;
            ship.source_desired_forward = TacticalWorldVector::ZERO;
        }
    }
    for index in 0..session.fighters.len() {
        if session.fighters[index].order != TacticalOrder::Escort {
            session.fighters[index].escort_target = None;
            continue;
        }
        let valid = session.fighters[index].escort_target.is_some_and(|target| {
            escort_target_is_eligible(session, session.fighters[index].is_attacker, target)
        });
        if !valid {
            session.fighters[index].order = TacticalOrder::None;
            session.fighters[index].escort_target = None;
        }
    }
}

/// Keep order-1 units around their protected ship while autonomous weapons
/// continue to select nearby opportunity targets. Capital ships use the normal
/// retained movement integrator; fighter groups use their source engine rate.
/// hyp: the escort executor's follow geometry is not traced; the 8- and
/// 6-unit offsets and the 10-unit stand-off are placeholders until it is.
fn advance_original_escort_movement(session: &mut BattleSession, delta_milliseconds: f32) {
    if !delta_milliseconds.is_finite() || delta_milliseconds <= 0.0 {
        return;
    }
    let target_positions = session
        .ships
        .iter()
        .map(|ship| ship.source_position)
        .collect::<Vec<_>>();
    for (index, ship) in session.ships.iter_mut().enumerate() {
        let Some(target) = ship
            .escort_target
            .filter(|_| ship.order == TacticalOrder::Escort)
        else {
            continue;
        };
        let Some(mut waypoint) = target_positions.get(target).copied() else {
            continue;
        };
        let side = if index % 2 == 0 { 1.0 } else { -1.0 };
        waypoint.x += side * 8.0;
        waypoint.z -= 6.0 + (index % 3) as f32 * 3.0;
        ship.navigation_route.clear();
        ship.navigation_route.push(waypoint);
        ship.source_waypoint = Some(waypoint);
        ship.source_desired_forward = source_delta(waypoint, ship.source_position).normalized();
    }

    let seconds = delta_milliseconds * 0.001;
    for fighter in &mut session.fighters {
        let Some(target) = fighter
            .escort_target
            .filter(|_| fighter.order == TacticalOrder::Escort)
        else {
            continue;
        };
        let Some(target_position) = target_positions.get(target).copied() else {
            continue;
        };
        let delta = source_delta(target_position, fighter.source_position);
        let distance = delta.x.hypot(delta.y).hypot(delta.z);
        if distance > 10.0 {
            let travel = (fighter.sublight_engine_power.max(0.0) * seconds).min(distance - 10.0);
            let direction = delta.normalized();
            fighter.source_position.x += direction.x * travel;
            fighter.source_position.y += direction.y * travel;
            fighter.source_position.z += direction.z * travel;
        }
    }
}

fn tactical_attack_target_is_eligible(
    session: &BattleSession,
    source_is_attacker: bool,
    target: TacticalAttackTarget,
    order: TacticalOrder,
) -> bool {
    match (order, target) {
        (TacticalOrder::AttackCapitalShips, TacticalAttackTarget::CapitalShip(index)) => {
            session.ships.get(index).is_some_and(|ship| {
                ship.alive && !ship.retreating && ship.is_attacker != source_is_attacker
            })
        }
        (TacticalOrder::AttackFighters, TacticalAttackTarget::FighterGroup(index)) => {
            session.fighters.get(index).is_some_and(|fighter| {
                fighter.alive
                    && fighter.squad_count > 0
                    && fighter.recovery_state != TacticalFighterRecoveryState::Recovered
                    && fighter.listed()
                    && fighter.is_attacker != source_is_attacker
            })
        }
        _ => false,
    }
}

fn manual_attack_target_is_eligible(
    session: &BattleSession,
    source_is_attacker: bool,
    target: TacticalAttackTarget,
) -> bool {
    match target {
        TacticalAttackTarget::CapitalShip(index) => session.ships.get(index).is_some_and(|ship| {
            ship.alive && !ship.retreating && ship.is_attacker != source_is_attacker
        }),
        TacticalAttackTarget::FighterGroup(index) => {
            session.fighters.get(index).is_some_and(|fighter| {
                fighter.alive
                    && fighter.squad_count > 0
                    && fighter.recovery_state != TacticalFighterRecoveryState::Recovered
                    && fighter.listed()
                    && fighter.is_attacker != source_is_attacker
            })
        }
    }
}

fn first_original_attack_target(
    session: &BattleSession,
    source_is_attacker: bool,
    order: TacticalOrder,
) -> Option<TacticalAttackTarget> {
    match order {
        TacticalOrder::AttackCapitalShips => session
            .ships
            .iter()
            .enumerate()
            .find(|(_, ship)| {
                ship.alive && !ship.retreating && ship.is_attacker != source_is_attacker
            })
            .map(|(index, _)| TacticalAttackTarget::CapitalShip(index)),
        TacticalOrder::AttackFighters => session
            .fighters
            .iter()
            .enumerate()
            .find(|(_, fighter)| {
                fighter.alive
                    && fighter.squad_count > 0
                    && fighter.recovery_state != TacticalFighterRecoveryState::Recovered
                    && fighter.listed()
                    && fighter.is_attacker != source_is_attacker
            })
            .map(|(index, _)| TacticalAttackTarget::FighterGroup(index)),
        _ => None,
    }
}

/// Run the target-acquisition half of `FUN_005d0b10` and `FUN_005d0bb0`.
/// Both source executors traverse one stable tactical-object list, filter it
/// by the vtable type code (capital 0, fighter 1), accept the first eligible
/// hostile object, mark the executor active, and dispatch event `0x36`.
pub fn begin_original_attack_order(
    session: &mut BattleSession,
    selected_capitals: &[usize],
    selected_fighters: &[usize],
    order: TacticalOrder,
) {
    if order == TacticalOrder::AttackDeathStar {
        // `FUN_005ca6d0` case 6 (`.c:141-156`): a fighter group (kind 4) of
        // side 0, the Alliance, with no run in progress (`DAT_006bc978`);
        // `FUN_005d0820` makes the result object in state 2.
        let Some(death_star) = session
            .death_star
            .filter(|death_star| death_star.operational())
        else {
            return;
        };
        let Some(fighter_group) = selected_fighters.iter().find_map(|&index| {
            session
                .fighters
                .get(index)
                .filter(|fighter| {
                    fighter.alive
                        && fighter.squad_count > 0
                        && fighter.identity.is_alliance
                        && fighter.is_attacker != death_star.is_attacker
                })
                .map(|fighter| fighter.fighter_group)
        }) else {
            return;
        };
        if session.trench_run_group.is_some() || session.trench_run_outcome.is_some() {
            return;
        }
        session.trench_run_group = Some(fighter_group);
        // State 2's first update speaks (`FUN_005d0340`): the attacking
        // group's call to the Alliance player, the station's alarm to the
        // Imperial one.
        if session.player_voice_faction() == TacticalVoiceFaction::Alliance {
            session
                .pending_voice_cues
                .push(TacticalVoiceCue::trench_run_started(fighter_group));
        } else if death_star.is_attacker == session.player_is_attacker {
            session
                .pending_voice_cues
                .push(TacticalVoiceCue::death_star_under_attack());
        }
        return;
    }

    if !matches!(
        order,
        TacticalOrder::AttackCapitalShips | TacticalOrder::AttackFighters
    ) {
        return;
    }

    for &index in selected_capitals {
        let source_is_attacker = session.ships[index].is_attacker;
        let current = session.ships[index].attack_target;
        if current.is_some_and(|target| {
            tactical_attack_target_is_eligible(session, source_is_attacker, target, order)
        }) {
            continue;
        }
        let target = first_original_attack_target(session, source_is_attacker, order);
        session.ships[index].attack_target = target;
    }
    for &index in selected_fighters {
        let source_is_attacker = session.fighters[index].is_attacker;
        let current = session.fighters[index].attack_target;
        if current.is_some_and(|target| {
            tactical_attack_target_is_eligible(session, source_is_attacker, target, order)
        }) {
            continue;
        }
        let target = first_original_attack_target(session, source_is_attacker, order);
        session.fighters[index].attack_target = target;
    }
}

/// Maintain the typed target list created by the original attack executors.
///
/// `FUN_005a8fc0` links the first eligible typed target into the owner's group.
/// The manager then exposes separate add, remove, replace, and change-target
/// messages plus stable next/previous target selection. Open Rebellion models
/// that lifecycle directly: an invalid entry is replaced by the first eligible
/// hostile object of the same requested class, and an exhausted class clears
/// the target instead of falling through to another class.
/// `FUN_005c9200` hands the tactic object `DAT_0066cbac`: Stand Off assigns
/// a target only to a force that kills it within this many seconds.
const ORIGINAL_ENGAGEMENT_KILL_SECONDS: f32 = 180.0;
/// `FUN_005ed8e0` (`_DAT_0066da7c` and `_DAT_0066daa0`): a target's value
/// weighs its firepower by 15 and its shield-recharge and hull ratios by 0.01.
const ORIGINAL_TARGET_FIREPOWER_WEIGHT: f32 = 15.0;
/// `FUN_005ed8e0` (`_DAT_0066daa0`): the ratio weight.
const ORIGINAL_TARGET_RATIO_WEIGHT: f32 = 0.01;
/// `FUN_005ed5f0` and `FUN_005ed370` (`_DAT_0066da84`): a force keeps a target
/// it can destroy, with the rest of its side on it, in less than this many
/// seconds.
const ORIGINAL_ENGAGEMENT_ABANDON_SECONDS: f32 = 300.0;
/// `FUN_005ec740`'s key for a force that cannot make the limit
/// (`_DAT_0066da58`); it sorts such forces last.
const ORIGINAL_ENGAGEMENT_UNREACHABLE_KEY: f32 = 100_000.0;
/// `FUN_005ed170`: effective hull is shield plus 0.65 of hull
/// (`_DAT_0066da8c`), at least 1.0; travel seconds count 0.001 each
/// (`_DAT_0066da98`).
const ORIGINAL_ENGAGEMENT_HULL_WEIGHT: f32 = 0.65;
/// `FUN_005ed170` (`_DAT_0066da98`): the travel-time weight.
const ORIGINAL_ENGAGEMENT_TRAVEL_WEIGHT: f32 = 0.001;
/// `FUN_005cc300` tests `distance - _DAT_0066d008` (-2.0) against the band.
const ORIGINAL_ENGAGEMENT_BAND_PAD: f32 = 2.0;
/// `FUN_005cc300` aims this far into the band: 0.5 below three units, 0.75
/// (`_DAT_0066d014`) below six, else 0.9 (`_DAT_0066d010`); never nearer
/// than 2.0 (`_DAT_0066d018`).
const ORIGINAL_ENGAGEMENT_GOAL_MIN: f32 = 2.0;
/// `FUN_005edf80` keeps the band's near edge at `DAT_0066daa8` (5.0) or
/// more; an unarmed force holds at 1.2 (`_DAT_0066dabc`) times the
/// target's reach, at least 10.0 (`_DAT_0066dac0`), out to 1e6.
const ORIGINAL_ENGAGEMENT_NEAR_MIN: f32 = 5.0;
/// `FUN_005edf80` (`_DAT_0066dabc`): the unarmed hold's reach factor.
const ORIGINAL_ENGAGEMENT_UNARMED_REACH: f32 = 1.2;
/// `FUN_005edf80` (`_DAT_0066dac0`): the unarmed hold's nearest edge.
const ORIGINAL_ENGAGEMENT_UNARMED_MIN: f32 = 10.0;
/// `FUN_005edf80` stores `0x497423f0` (999,999.0) as the unarmed far edge.
const ORIGINAL_ENGAGEMENT_UNARMED_FAR: f32 = 999_999.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EngagementUnit {
    Capital(usize),
    Fighter(usize),
}

/// One task force or fighter group, leader first.
struct EngagementGroup {
    is_attacker: bool,
    slot: usize,
    members: Vec<EngagementUnit>,
    tactic: TacticalTactic,
    /// `FUN_005b7ce0`: a force whose leader has laser cannons ranks
    /// fighters first (`FUN_005c9200`'s second list).
    fighters_first: bool,
    /// A player attack order or focus target the force steers to instead.
    ordered_target: Option<TacticalAttackTarget>,
}

fn engagement_unit_of(target: TacticalAttackTarget) -> EngagementUnit {
    match target {
        TacticalAttackTarget::CapitalShip(index) => EngagementUnit::Capital(index),
        TacticalAttackTarget::FighterGroup(index) => EngagementUnit::Fighter(index),
    }
}

fn engagement_target_of(unit: EngagementUnit) -> TacticalAttackTarget {
    match unit {
        EngagementUnit::Capital(index) => TacticalAttackTarget::CapitalShip(index),
        EngagementUnit::Fighter(index) => TacticalAttackTarget::FighterGroup(index),
    }
}

fn engagement_unit_live(session: &BattleSession, unit: EngagementUnit) -> bool {
    match unit {
        EngagementUnit::Capital(index) => session
            .ships
            .get(index)
            .is_some_and(|ship| ship.alive && !ship.retreating),
        EngagementUnit::Fighter(index) => session.fighters.get(index).is_some_and(|fighter| {
            fighter.alive
                && fighter.squad_count > 0
                && fighter.recovery_state != TacticalFighterRecoveryState::Recovered
                && fighter.listed()
        }),
    }
}

fn engagement_unit_position(
    session: &BattleSession,
    unit: EngagementUnit,
) -> TacticalWorldPosition {
    match unit {
        EngagementUnit::Capital(index) => session.ships[index].source_position,
        EngagementUnit::Fighter(index) => session.fighters[index].source_position,
    }
}

fn engagement_unit_is_attacker(session: &BattleSession, unit: EngagementUnit) -> bool {
    match unit {
        EngagementUnit::Capital(index) => session.ships[index].is_attacker,
        EngagementUnit::Fighter(index) => session.fighters[index].is_attacker,
    }
}

fn engagement_family_modifier(kind: WeaponKind, against_fighter: bool) -> f32 {
    if against_fighter {
        match kind {
            WeaponKind::IonCannon => ORIGINAL_ION_VS_FIGHTER,
            WeaponKind::LaserCannon => ORIGINAL_LASER_VS_FIGHTER,
            WeaponKind::Turbolaser => ORIGINAL_TURBOLASER_VS_FIGHTER,
            WeaponKind::FighterAttack => 0.0,
        }
    } else {
        match kind {
            WeaponKind::LaserCannon => ORIGINAL_LASER_VS_CAPITAL,
            WeaponKind::IonCannon | WeaponKind::Turbolaser => 1.0,
            WeaponKind::FighterAttack => 0.0,
        }
    }
}

/// A unit's weapon recharge, its vtable `+0x60`: a capital ship's
/// `FUN_005b0400` (the port's `effective_weapon_recharge`), a fighter
/// group's `FUN_005b9c00` (hull × 3.751 / 8).
fn engagement_unit_recharge(session: &BattleSession, unit: EngagementUnit) -> f32 {
    match unit {
        EngagementUnit::Capital(index) => session.ships[index].effective_weapon_recharge(),
        EngagementUnit::Fighter(index) => session.fighters[index].weapon_hull_factor(),
    }
}

/// A unit's maneuverability, its vtable `+0xbc`: `FUN_005b0be0` gives a
/// capital ship without engine power 1.0 and otherwise one more than its
/// rating, at most 9; a fighter uses `effective_maneuverability`.
fn engagement_unit_maneuverability(session: &BattleSession, unit: EngagementUnit) -> f32 {
    match unit {
        EngagementUnit::Capital(index) => {
            let moving = original_tactical_mobility(&session.ships, &session.field_effects, index)
                .is_some_and(|mobility| mobility.effective_engine_power > 0.0);
            if moving {
                (session.ships[index].maneuverability as f32 + 1.0)
                    .min(ORIGINAL_CAPITAL_MANEUVERABILITY_MAX)
            } else {
                1.0
            }
        }
        EngagementUnit::Fighter(index) => session.fighters[index].effective_maneuverability(),
    }
}

/// A unit's weapon counts per arc: a capital ship's four, a fighter group's
/// one fore record.
fn engagement_unit_arcs(session: &BattleSession, unit: EngagementUnit) -> Vec<TacticalWeaponArc> {
    match unit {
        EngagementUnit::Capital(index) => session.ships[index].weapon_arcs.to_vec(),
        EngagementUnit::Fighter(index) => vec![session.fighters[index].weapon_arc],
    }
}

/// `FUN_005b7780`: the damage a unit deals its target each second. Its arc
/// is the one whose families sum strongest against the target's class
/// (`FUN_005b4c60`, vtable `+0x84`); the rate is that arc's summed family
/// strength times the unit's weapon recharge over the arc's weapon count.
/// Against a fighter, lasers and turbolasers scale by the attacker's over
/// the target's maneuverability (at least 0.1) and ions do nothing. A
/// fighter group adds its torpedoes (`FUN_005b23a0`, rating × 0.5333)
/// against an unshielded capital ship. Position plays no part.
fn original_unit_damage_rate(
    session: &BattleSession,
    unit: EngagementUnit,
    target: EngagementUnit,
) -> f32 {
    if !engagement_unit_live(session, unit) || !engagement_unit_live(session, target) {
        return 0.0;
    }
    let against_fighter = matches!(target, EngagementUnit::Fighter(_));
    let maneuver_ratio = (engagement_unit_maneuverability(session, unit)
        / engagement_unit_maneuverability(session, target))
    .max(0.1);
    let torpedoes = match (unit, target) {
        (EngagementUnit::Fighter(source), EngagementUnit::Capital(capital))
            if session.ships[capital].shield <= 0 =>
        {
            session.fighters[source].torpedo_term()
        }
        _ => 0.0,
    };
    unit_rate_against(session, unit, against_fighter, maneuver_ratio, torpedoes)
}

/// `FUN_005b7a80`: a unit's firepower against capital ships, the figure
/// `FUN_005d0ce0` sums into a side's strength.
fn original_unit_firepower(session: &BattleSession, unit: EngagementUnit) -> f32 {
    if !engagement_unit_live(session, unit) {
        return 0.0;
    }
    unit_rate_against(session, unit, false, 1.0, 0.0)
}

fn unit_rate_against(
    session: &BattleSession,
    unit: EngagementUnit,
    against_fighter: bool,
    maneuver_ratio: f32,
    torpedoes: f32,
) -> f32 {
    let kinds = [
        WeaponKind::LaserCannon,
        WeaponKind::IonCannon,
        WeaponKind::Turbolaser,
    ];
    let family = |arc: &TacticalWeaponArc, kind: WeaponKind| {
        original_weapon_count(*arc, kind) as f32
            * ORIGINAL_CAPITAL_WEAPON_STRENGTH
            * engagement_family_modifier(kind, against_fighter)
    };
    let arcs = engagement_unit_arcs(session, unit);
    let mut selected = None;
    let mut strongest = 0.0;
    for arc in &arcs {
        let strength: f32 = kinds.into_iter().map(|kind| family(arc, kind)).sum();
        if strongest < strength {
            strongest = strength;
            selected = Some(*arc);
        }
    }
    let Some(arc) = selected else {
        return 0.0;
    };
    let total = (arc.laser_cannons + arc.ion_cannons + arc.turbolasers) as f32;
    let recharge = engagement_unit_recharge(session, unit);
    if total <= 0.0 || recharge <= 0.0 {
        return 0.0;
    }
    let strength: f32 = kinds
        .into_iter()
        .map(|kind| {
            let term = family(&arc, kind);
            if against_fighter && kind != WeaponKind::IonCannon {
                term * maneuver_ratio
            } else {
                term
            }
        })
        .sum();
    (strength + torpedoes) * recharge / total
}

/// Each weapon family's reach and its weight against one class of target
/// (`FUN_005b49e0`: count × the unit's weapon recharge × the family's
/// strength).
fn engagement_unit_families(
    session: &BattleSession,
    unit: EngagementUnit,
    against_fighter: bool,
) -> Vec<(f32, f32)> {
    let recharge = engagement_unit_recharge(session, unit);
    let ranges = match unit {
        EngagementUnit::Capital(index) => session.ships[index].weapon_ranges,
        EngagementUnit::Fighter(index) => session.fighters[index].weapon_ranges,
    };
    let range_of = |kind: WeaponKind| match kind {
        WeaponKind::LaserCannon => ranges.laser_cannon,
        WeaponKind::IonCannon => ranges.ion_cannon,
        WeaponKind::Turbolaser => ranges.turbolaser,
        WeaponKind::FighterAttack => 0.0,
    };
    let arcs = engagement_unit_arcs(session, unit);
    [
        WeaponKind::LaserCannon,
        WeaponKind::IonCannon,
        WeaponKind::Turbolaser,
    ]
    .into_iter()
    .filter_map(|kind| {
        let count: u32 = arcs
            .iter()
            .map(|arc| original_weapon_count(*arc, kind))
            .sum();
        let range = range_of(kind);
        (count > 0 && range > 0.0).then(|| {
            (
                range,
                count as f32
                    * recharge
                    * ORIGINAL_CAPITAL_WEAPON_STRENGTH
                    * engagement_family_modifier(kind, against_fighter),
            )
        })
    })
    .collect()
}

fn engagement_unit_reach(session: &BattleSession, unit: EngagementUnit) -> f32 {
    // `FUN_005b18d0`: the longest range among the unit's armed families.
    engagement_unit_families(session, unit, false)
        .into_iter()
        .chain(engagement_unit_families(session, unit, true))
        .map(|(range, _)| range)
        .fold(0.0, f32::max)
}

fn engagement_group_damage(
    session: &BattleSession,
    members: &[EngagementUnit],
    target: EngagementUnit,
) -> f32 {
    // `FUN_005cb650` sums `FUN_005b7780` over the force's active units.
    members
        .iter()
        .map(|unit| original_unit_damage_rate(session, *unit, target))
        .sum()
}

/// `FUN_005ed170`: seconds for a force (adding its damage to `damage`) to
/// destroy the target, plus its travel time weighted by 0.001; -1.0 when it
/// cannot out-damage the target's shield recharge (vtable `+0x64`:
/// `FUN_005b0460` for a capital ship, `FUN_005b9c30` for a fighter group).
fn engagement_kill_seconds(
    session: &BattleSession,
    group: &EngagementGroup,
    target: EngagementUnit,
    damage: &mut f32,
) -> f32 {
    *damage += engagement_group_damage(session, &group.members, target);
    let (hull, shield, recovery) = match target {
        EngagementUnit::Capital(index) => {
            let ship = &session.ships[index];
            (
                ship.hull_current.max(0) as f32,
                ship.shield.max(0) as f32,
                ship.effective_shield_recharge(),
            )
        }
        EngagementUnit::Fighter(index) => {
            let fighter = &session.fighters[index];
            (
                fighter.hull_current.max(0.0),
                fighter.shield.max(0.0),
                fighter.shield_recharge_rate(),
            )
        }
    };
    let effective_hull = (hull * ORIGINAL_ENGAGEMENT_HULL_WEIGHT + shield).max(1.0);
    if *damage - recovery <= 0.0 {
        return -1.0;
    }
    let mut seconds = effective_hull / (*damage - recovery);
    let leader = group.members[0];
    let delta = source_delta(
        engagement_unit_position(session, target),
        engagement_unit_position(session, leader),
    );
    if delta != TacticalWorldVector::ZERO {
        let speed = match leader {
            EngagementUnit::Capital(index) => session.ships[index].sublight_engine_power,
            EngagementUnit::Fighter(index) => session.fighters[index].sublight_engine_power,
        };
        let distance = delta.x.hypot(delta.y).hypot(delta.z);
        let travel = if speed > 0.0 {
            (distance / speed).max(1.0)
        } else {
            1.0
        };
        seconds += travel * ORIGINAL_ENGAGEMENT_TRAVEL_WEIGHT;
    }
    seconds
}

/// `FUN_005edf80`: the distance band a force fights its target from. Each
/// unit's families add `[0, range]` intervals weighted by their strength,
/// the target's subtract theirs, and the force takes the interval with the
/// greatest net strength that its own longest weapon reaches.
fn engagement_band(
    session: &BattleSession,
    group: &EngagementGroup,
    target: EngagementUnit,
) -> (f32, f32) {
    let target_is_fighter = matches!(target, EngagementUnit::Fighter(_));
    let mut intervals: Vec<(f32, f32)> = Vec::new();
    let mut own_reach = 0.0_f32;
    for unit in group
        .members
        .iter()
        .filter(|unit| engagement_unit_live(session, **unit))
    {
        own_reach = own_reach.max(engagement_unit_reach(session, *unit));
        intervals.extend(engagement_unit_families(session, *unit, target_is_fighter));
    }
    let leader_is_fighter = matches!(group.members[0], EngagementUnit::Fighter(_));
    let target_families = engagement_unit_families(session, target, leader_is_fighter);
    let target_reach = engagement_unit_reach(session, target);
    intervals.extend(
        target_families
            .into_iter()
            .map(|(range, strength)| (range, -strength)),
    );
    if own_reach <= 0.0 {
        let near =
            (target_reach * ORIGINAL_ENGAGEMENT_UNARMED_REACH).max(ORIGINAL_ENGAGEMENT_UNARMED_MIN);
        return (near, ORIGINAL_ENGAGEMENT_UNARMED_FAR);
    }
    let mut edges = intervals
        .iter()
        .map(|(range, _)| *range)
        .collect::<Vec<_>>();
    edges.sort_by(f32::total_cmp);
    edges.dedup();
    let mut best: Option<(f32, f32, f32)> = None;
    let mut near = 0.0;
    for far in edges {
        let net: f32 = intervals
            .iter()
            .filter(|(range, _)| *range >= far)
            .map(|(_, strength)| strength)
            .sum();
        if far <= own_reach && best.is_none_or(|(_, _, best_net)| best_net < net) {
            best = Some((near, far, net));
        }
        near = far;
    }
    let (near, far) = best.map_or((0.0, own_reach), |(near, far, _)| (near, far));
    (
        near.max(ORIGINAL_ENGAGEMENT_NEAR_MIN),
        far.max(ORIGINAL_ENGAGEMENT_NEAR_MIN),
    )
}

/// Gather each side's task forces and fighter groups that steer themselves:
/// none of their units follows a maneuver, route, escort, recovery or Death
/// Star order (`FUN_005ed5f0` leaves order codes 2, 3 and 6 alone; the
/// maneuver executors own their waypoints).
fn engagement_groups(session: &BattleSession) -> Vec<EngagementGroup> {
    let mut groups = Vec::new();
    for is_attacker in [true, false] {
        let side = usize::from(!is_attacker);
        for task_force in 0_u8..8 {
            let mut members = session
                .ships
                .iter()
                .enumerate()
                .filter(|(_, ship)| {
                    ship.alive
                        && !ship.retreating
                        && ship.is_attacker == is_attacker
                        && ship.task_force == task_force
                })
                .map(|(index, _)| index)
                .collect::<Vec<_>>();
            if members.is_empty() {
                continue;
            }
            members.sort_by(|left, right| {
                original_formation_capability_score(&session.ships[*right])
                    .total_cmp(&original_formation_capability_score(&session.ships[*left]))
            });
            let slot = usize::from(task_force);
            let goal = session.engagement.steering_goals[side][slot];
            let commanded = members.iter().any(|index| {
                let ship = &session.ships[*index];
                !matches!(
                    ship.order,
                    TacticalOrder::None
                        | TacticalOrder::AttackCapitalShips
                        | TacticalOrder::AttackFighters
                ) || !ship.navigation_route.is_empty()
                    || ship
                        .source_waypoint
                        .is_some_and(|waypoint| Some(waypoint) != goal)
            });
            if commanded {
                continue;
            }
            let leader = &session.ships[members[0]];
            let ordered_target = members
                .iter()
                .map(|index| &session.ships[*index])
                .find(|ship| {
                    !ship.manual_targets.is_empty()
                        || matches!(
                            ship.order,
                            TacticalOrder::AttackCapitalShips | TacticalOrder::AttackFighters
                        )
                })
                .and_then(|ship| ship.attack_target);
            groups.push(EngagementGroup {
                is_attacker,
                slot,
                fighters_first: leader.weapon_arcs.iter().any(|arc| arc.laser_cannons > 0),
                tactic: leader.tactic,
                ordered_target,
                members: members.into_iter().map(EngagementUnit::Capital).collect(),
            });
        }
        for fighter_group in 0_u8..5 {
            let members = session
                .fighters
                .iter()
                .enumerate()
                .filter(|(index, fighter)| {
                    fighter.is_attacker == is_attacker
                        && fighter.fighter_group.min(4) == fighter_group
                        && engagement_unit_live(session, EngagementUnit::Fighter(*index))
                })
                .map(|(index, _)| index)
                .collect::<Vec<_>>();
            if members.is_empty() {
                continue;
            }
            let commanded = members.iter().any(|index| {
                let fighter = &session.fighters[*index];
                !matches!(
                    fighter.order,
                    TacticalOrder::None
                        | TacticalOrder::AttackCapitalShips
                        | TacticalOrder::AttackFighters
                ) || fighter.recovery_state != TacticalFighterRecoveryState::AwaitingCarrier
            });
            if commanded {
                continue;
            }
            let leader = &session.fighters[members[0]];
            let ordered_target = members
                .iter()
                .map(|index| &session.fighters[*index])
                .find(|fighter| {
                    matches!(
                        fighter.order,
                        TacticalOrder::AttackCapitalShips | TacticalOrder::AttackFighters
                    )
                })
                .and_then(|fighter| fighter.attack_target);
            groups.push(EngagementGroup {
                is_attacker,
                slot: 8 + usize::from(fighter_group),
                fighters_first: leader.weapon_arc.laser_cannons > 0,
                tactic: leader.tactic,
                ordered_target,
                members: members.into_iter().map(EngagementUnit::Fighter).collect(),
            });
        }
    }
    groups
}

/// `FUN_005d1a70` ranks a side's opponents into `side+0x90` by the side
/// policy's value (`FUN_005ed8e0` / `FUN_005ed7a0`), highest first, with
/// eligible targets (policy `+0x10`: armed, `FUN_005ed550` / `FUN_005ed300`)
/// ahead of the rest. `FUN_005c9200` then splits it into capital ships first
/// or, for forces with laser cannons, fighters first.
/// port: the value's manager focus lists (`+0xe5c`, `+0xe50`) and the
/// side's damage record (`side+0x88`) are not modeled; without them both
/// policies give the same order.
fn engagement_priorities(
    session: &BattleSession,
    is_attacker: bool,
    fighters_first: bool,
) -> Vec<EngagementUnit> {
    let ranked = |units: Vec<EngagementUnit>| {
        let mut keyed = units
            .into_iter()
            .filter(|unit| {
                engagement_unit_live(session, *unit)
                    && engagement_unit_is_attacker(session, *unit) != is_attacker
            })
            .map(|unit| {
                let eligible = engagement_unit_recharge(session, unit) > 0.0;
                (eligible, engagement_target_value(session, unit), unit)
            })
            .collect::<Vec<_>>();
        keyed.sort_by(|left, right| right.0.cmp(&left.0).then(right.1.total_cmp(&left.1)));
        keyed
            .into_iter()
            .map(|(_, _, unit)| unit)
            .collect::<Vec<_>>()
    };
    let capitals = ranked(
        (0..session.ships.len())
            .map(EngagementUnit::Capital)
            .collect(),
    );
    let fighters = ranked(
        (0..session.fighters.len())
            .map(EngagementUnit::Fighter)
            .collect(),
    );
    if fighters_first {
        fighters.into_iter().chain(capitals).collect()
    } else {
        capitals.into_iter().chain(fighters).collect()
    }
}

/// `FUN_005ed8e0`: 15 × the unit's firepower against capital ships
/// (`_DAT_0066da7c`, `FUN_005b7a80`), less 0.01 × its shield-recharge ratio
/// (`FUN_005b1710`) and 0.01 × its hull ratio (`FUN_005b1640`).
fn engagement_target_value(session: &BattleSession, unit: EngagementUnit) -> f32 {
    let (recharge_ratio, hull_ratio) = match unit {
        EngagementUnit::Capital(index) => {
            let ship = &session.ships[index];
            let recharge = if ship.shield_recharge_rate > 0.0 {
                ship.effective_shield_recharge() / ship.shield_recharge_rate
            } else {
                0.0
            };
            (recharge, ship.hull_ratio())
        }
        EngagementUnit::Fighter(index) => {
            let fighter = &session.fighters[index];
            let ratio = if fighter.hull_max > 0.0 {
                fighter.hull_current.max(0.0) / fighter.hull_max
            } else {
                0.0
            };
            (ratio, ratio)
        }
    };
    ORIGINAL_TARGET_FIREPOWER_WEIGHT * original_unit_firepower(session, unit)
        - ORIGINAL_TARGET_RATIO_WEIGHT * recharge_ratio
        - ORIGINAL_TARGET_RATIO_WEIGHT * hull_ratio
}

/// `FUN_005ed5f0` / `FUN_005ed370`: a force needs a new target when it has
/// none, or when every force of its side on that target together needs 300
/// seconds or more (`_DAT_0066da84`) to destroy it, or cannot.
fn engagement_target_abandoned(
    session: &BattleSession,
    groups: &[EngagementGroup],
    group: &EngagementGroup,
) -> bool {
    let side = usize::from(!group.is_attacker);
    let Some(target) = session.engagement.targets[side][group.slot]
        .map(engagement_unit_of)
        .filter(|target| engagement_unit_live(session, *target))
    else {
        return true;
    };
    let mut damage = 0.0;
    let mut seconds = -1.0;
    for sharing in groups.iter().filter(|other| {
        other.is_attacker == group.is_attacker
            && session.engagement.targets[side][other.slot]
                .is_some_and(|other_target| engagement_unit_of(other_target) == target)
    }) {
        seconds = engagement_kill_seconds(session, sharing, target, &mut damage);
    }
    !(0.0..ORIGINAL_ENGAGEMENT_ABANDON_SECONDS).contains(&seconds)
}

/// Assign targets to one side's forces that have none, through the forces'
/// tactic (`FUN_005c9200` calls the side's tactic object at `+0x34` once per
/// priority list).
fn assign_engagement_targets(
    session: &mut BattleSession,
    groups: &[EngagementGroup],
    is_attacker: bool,
    fighters_first: bool,
) {
    let side = usize::from(!is_attacker);
    let needing = groups
        .iter()
        .filter(|group| {
            group.is_attacker == is_attacker
                && group.fighters_first == fighters_first
                && group.ordered_target.is_none()
                && engagement_target_abandoned(session, groups, group)
        })
        .collect::<Vec<_>>();
    if needing.is_empty() {
        return;
    }
    let priorities = engagement_priorities(session, is_attacker, fighters_first);
    let Some(&first) = priorities.first() else {
        return;
    };
    let (surround, stand_off): (Vec<&EngagementGroup>, Vec<&EngagementGroup>) = needing
        .into_iter()
        .partition(|group| group.tactic == TacticalTactic::Surround);

    // `FUN_005ec6d0` (Surround): every force takes the first target.
    for group in surround {
        session.engagement.targets[side][group.slot] = Some(engagement_target_of(first));
    }

    // `FUN_005ec740` (Stand Off): for each target in priority order, sort the
    // free forces by kill time and commit the fewest weakest forces whose
    // combined damage, added to the best force's, kills it within the limit.
    let mut free = stand_off;
    for &target in &priorities {
        if free.is_empty() {
            break;
        }
        let mut keyed = free
            .iter()
            .map(|group| {
                let mut damage = 0.0;
                let seconds = engagement_kill_seconds(session, group, target, &mut damage);
                let key = if !(0.0..=ORIGINAL_ENGAGEMENT_KILL_SECONDS).contains(&seconds) {
                    ORIGINAL_ENGAGEMENT_UNREACHABLE_KEY - damage
                } else {
                    seconds
                };
                (key, *group)
            })
            .collect::<Vec<_>>();
        keyed.sort_by(|left, right| left.0.total_cmp(&right.0));
        if keyed.len() == 1 {
            // A lone force takes the target and the pass ends there.
            session.engagement.targets[side][keyed[0].1.slot] = Some(engagement_target_of(target));
            return;
        }
        let mut damage = engagement_group_damage(session, &keyed[0].1.members, target);
        let mut committed = None;
        for cut in (1..keyed.len()).rev() {
            let seconds = engagement_kill_seconds(session, keyed[cut].1, target, &mut damage);
            if (0.0..=ORIGINAL_ENGAGEMENT_KILL_SECONDS).contains(&seconds) {
                committed = Some(cut);
                break;
            }
        }
        if let Some(cut) = committed {
            for (_, group) in &keyed[cut..] {
                session.engagement.targets[side][group.slot] = Some(engagement_target_of(target));
            }
            free.retain(|group| {
                !keyed[cut..]
                    .iter()
                    .any(|(_, taken)| taken.slot == group.slot)
            });
        }
    }
    // Forces left over take the first target.
    for group in free {
        session.engagement.targets[side][group.slot] = Some(engagement_target_of(first));
    }
}

/// The source moves each force toward or away from its target until the
/// distance falls inside its band (`FUN_005cc300`'s target branch), then
/// holds. Capital task forces steer their leader, which the retained
/// formation follows; a fighter group moves as one at its sublight power.
/// port: the leader stands in for the source's formation anchor
/// (`FUN_005ea030`), and a holding force keeps its last heading.
fn steer_engagement_group(
    session: &mut BattleSession,
    group: &EngagementGroup,
    target: EngagementUnit,
    delta_milliseconds: f32,
) {
    let side = usize::from(!group.is_attacker);
    let leader = group.members[0];
    let anchor = engagement_unit_position(session, leader);
    let delta = source_delta(engagement_unit_position(session, target), anchor);
    let distance = delta.x.hypot(delta.y).hypot(delta.z);
    let (near, far) = engagement_band(session, group, target);
    let padded = distance + ORIGINAL_ENGAGEMENT_BAND_PAD;
    if near <= padded && padded <= far {
        if let EngagementUnit::Capital(index) = leader {
            if session.engagement.steering_goals[side][group.slot]
                .take()
                .is_some()
            {
                let ship = &mut session.ships[index];
                ship.source_waypoint = None;
                ship.source_desired_forward = TacticalWorldVector::ZERO;
            }
        }
        return;
    }
    let direction = if distance < near {
        // Back off against the side's facing (`FUN_005a7490`), scaled by
        // `_DAT_0066d00c` (-1.1) before the normalization below.
        let own = side_mean_z(session, group.is_attacker);
        let other = side_mean_z(session, !group.is_attacker);
        let facing = if other >= own { 1.0 } else { -1.0 };
        TacticalWorldVector {
            x: 0.0,
            y: 0.0,
            z: -facing,
        }
    } else {
        delta.normalized()
    };
    let fraction = match group.members.len() {
        0..=2 => 0.5,
        3..=5 => 0.75,
        _ => 0.9,
    };
    let reach = (near + fraction * (far - near)).max(ORIGINAL_ENGAGEMENT_GOAL_MIN);
    let goal = TacticalWorldPosition {
        x: anchor.x + direction.x * reach,
        y: anchor.y + direction.y * reach,
        z: anchor.z + direction.z * reach,
    };
    match leader {
        EngagementUnit::Capital(index) => {
            let ship = &mut session.ships[index];
            ship.source_waypoint = Some(goal);
            ship.source_desired_forward = direction;
            session.engagement.steering_goals[side][group.slot] = Some(goal);
        }
        EngagementUnit::Fighter(index) => {
            let speed = session.fighters[index].sublight_engine_power.max(0.0);
            let travel = (speed * delta_milliseconds * 0.001).min(reach);
            let shift = TacticalWorldVector {
                x: direction.x * travel,
                y: direction.y * travel,
                z: direction.z * travel,
            };
            for unit in &group.members {
                if let EngagementUnit::Fighter(member) = unit {
                    let fighter = &mut session.fighters[*member];
                    fighter.source_position.x += shift.x;
                    fighter.source_position.y += shift.y;
                    fighter.source_position.z += shift.z;
                }
            }
        }
    }
}

fn side_mean_z(session: &BattleSession, is_attacker: bool) -> f32 {
    let positions = session
        .ships
        .iter()
        .filter(|ship| ship.is_attacker == is_attacker && ship.alive)
        .map(|ship| ship.source_position.z)
        .chain(
            session
                .fighters
                .iter()
                .filter(|fighter| fighter.is_attacker == is_attacker && fighter.alive)
                .map(|fighter| fighter.source_position.z),
        )
        .collect::<Vec<_>>();
    if positions.is_empty() {
        0.0
    } else {
        positions.iter().sum::<f32>() / positions.len() as f32
    }
}

/// Both sides' forces without orders pick targets by their tactic and close
/// to fighting range: `FUN_005d14d0` calls `FUN_005c9200` for each side on
/// every step, and each force's update (`FUN_005cc300`) steers it.
fn advance_original_engagement(session: &mut BattleSession, delta_milliseconds: f32) {
    let groups = engagement_groups(session);
    for is_attacker in [true, false] {
        for fighters_first in [false, true] {
            assign_engagement_targets(session, &groups, is_attacker, fighters_first);
        }
    }
    for group in &groups {
        let side = usize::from(!group.is_attacker);
        let target = group
            .ordered_target
            .or(session.engagement.targets[side][group.slot])
            .map(engagement_unit_of)
            .filter(|unit| engagement_unit_live(session, *unit));
        if let Some(target) = target {
            steer_engagement_group(session, group, target, delta_milliseconds);
        } else if let (Some(_), EngagementUnit::Capital(index)) = (
            session.engagement.steering_goals[side][group.slot].take(),
            group.members[0],
        ) {
            session.ships[index].source_waypoint = None;
            session.ships[index].source_desired_forward = TacticalWorldVector::ZERO;
        }
    }
}

/// Same-class target replacement for Attack Capital Ships and Attack
/// Fighters (`FUN_005a8fc0`, `FUN_005a8d30`, `tactical-attack-target-lifecycle.md`),
/// with completion cues for the eight task forces (`FUN_0059be80`) and four
/// fighter groups (`FUN_0059c030`).
pub fn refresh_original_attack_targets(session: &mut BattleSession) {
    let mut completed_player_task_forces = [false; 8];
    let mut completed_player_fighter_groups = [false; 4];
    for index in 0..session.ships.len() {
        let source_is_attacker = session.ships[index].is_attacker;
        let had_attack_target = session.ships[index].attack_target.is_some();
        let retained_manual_targets = session.ships[index]
            .manual_targets
            .iter()
            .copied()
            .filter(|target| manual_attack_target_is_eligible(session, source_is_attacker, *target))
            .collect::<Vec<_>>();
        if !retained_manual_targets.is_empty() {
            session.ships[index].attack_target = retained_manual_targets.first().copied();
            session.ships[index].manual_targets = retained_manual_targets;
            continue;
        }
        if !session.ships[index].manual_targets.is_empty() {
            session.ships[index].manual_targets.clear();
            session.ships[index].attack_target = None;
        }
        let order = session.ships[index].order;
        if !matches!(
            order,
            TacticalOrder::AttackCapitalShips | TacticalOrder::AttackFighters
        ) {
            continue;
        }
        if !session.ships[index].alive || session.ships[index].retreating {
            session.ships[index].attack_target = None;
            continue;
        }
        let current = session.ships[index].attack_target;
        if current.is_some_and(|target| {
            tactical_attack_target_is_eligible(session, source_is_attacker, target, order)
        }) {
            continue;
        }
        session.ships[index].attack_target =
            first_original_attack_target(session, source_is_attacker, order);
        if had_attack_target && session.ships[index].attack_target.is_none() {
            let ship = &session.ships[index];
            if ship.is_attacker == session.player_is_attacker {
                completed_player_task_forces[usize::from(ship.task_force.min(7))] = true;
            }
        }
    }

    for index in 0..session.fighters.len() {
        let had_attack_target = session.fighters[index].attack_target.is_some();
        let order = session.fighters[index].order;
        if !matches!(
            order,
            TacticalOrder::AttackCapitalShips | TacticalOrder::AttackFighters
        ) {
            continue;
        }
        if !session.fighters[index].alive
            || session.fighters[index].squad_count == 0
            || session.fighters[index].recovery_state == TacticalFighterRecoveryState::Recovered
        {
            session.fighters[index].attack_target = None;
            continue;
        }
        let source_is_attacker = session.fighters[index].is_attacker;
        let current = session.fighters[index].attack_target;
        if current.is_some_and(|target| {
            tactical_attack_target_is_eligible(session, source_is_attacker, target, order)
        }) {
            continue;
        }
        session.fighters[index].attack_target =
            first_original_attack_target(session, source_is_attacker, order);
        if had_attack_target && session.fighters[index].attack_target.is_none() {
            let fighter = &session.fighters[index];
            if fighter.is_attacker == session.player_is_attacker {
                completed_player_fighter_groups[usize::from(fighter.fighter_group.min(3))] = true;
            }
        }
    }

    let faction = session.player_voice_faction();
    for (group, transitioned) in completed_player_task_forces.into_iter().enumerate() {
        if transitioned
            && !session.ships.iter().any(|ship| {
                ship.alive
                    && ship.is_attacker == session.player_is_attacker
                    && usize::from(ship.task_force) == group
                    && ship.attack_target.is_some()
            })
        {
            session
                .pending_voice_cues
                .push(TacticalVoiceCue::capital_orders_complete(
                    faction,
                    group as u8,
                ));
        }
    }
    for (group, transitioned) in completed_player_fighter_groups.into_iter().enumerate() {
        if transitioned
            && !session.fighters.iter().any(|fighter| {
                fighter.alive
                    && fighter.is_attacker == session.player_is_attacker
                    && usize::from(fighter.fighter_group) == group
                    && fighter.attack_target.is_some()
            })
        {
            session
                .pending_voice_cues
                .push(TacticalVoiceCue::fighter_orders_complete(
                    faction,
                    group as u8,
                ));
        }
    }
}

pub fn begin_original_fighter_recovery(session: &mut BattleSession, selected: &[usize]) {
    let player_is_attacker = session.player_is_attacker;
    let player_faction = session.player_voice_faction();
    let mut warned_groups = [false; 4];
    let mut capacity_cues = Vec::new();
    let mut carrier_slots = session
        .ships
        .iter()
        .enumerate()
        .filter(|(_, ship)| ship.alive && ship.fighter_capacity > 0)
        .map(|(index, ship)| (index, ship.is_attacker, ship.fighter_capacity))
        .collect::<Vec<_>>();
    for (index, fighter) in session.fighters.iter().enumerate() {
        if selected.contains(&index)
            || !matches!(
                fighter.recovery_state,
                TacticalFighterRecoveryState::Reserved
                    | TacticalFighterRecoveryState::Returning
                    | TacticalFighterRecoveryState::Docking
            )
        {
            continue;
        }
        if let Some((_, _, remaining)) = fighter.recovery_target.and_then(|target| {
            carrier_slots
                .iter_mut()
                .find(|(carrier, _, _)| *carrier == target)
        }) {
            *remaining = remaining.saturating_sub(1);
        }
    }
    for &fighter_index in selected {
        let fighter = &mut session.fighters[fighter_index];
        // `FUN_005cf940` always resets the state before `FUN_005cf980`
        // reserves a compatible carrier and begins the inbound leg.
        fighter.recovery_state = TacticalFighterRecoveryState::AwaitingCarrier;
        fighter.recovery_target = None;
        if let Some((carrier, _, remaining)) = carrier_slots
            .iter_mut()
            .find(|(_, side, remaining)| *side == fighter.is_attacker && *remaining > 0)
        {
            fighter.recovery_state = TacticalFighterRecoveryState::Reserved;
            fighter.recovery_target = Some(*carrier);
            *remaining -= 1;
            fighter.recovery_state = TacticalFighterRecoveryState::Returning;
        } else {
            let group = usize::from(fighter.fighter_group);
            if fighter.is_attacker == player_is_attacker
                && group < warned_groups.len()
                && !warned_groups[group]
            {
                warned_groups[group] = true;
                capacity_cues.push(TacticalVoiceCue::fighter_recovery_capacity(
                    player_faction,
                    fighter.fighter_group,
                ));
            }
        }
    }
    session.pending_voice_cues.extend(capacity_cues);
}

/// `SYSTEMSD.DAT` records 100 through 299 are contiguous. This table preserves
/// their original `picture_id` values without adding presentation-only state
/// to the serialized simulation world.
const SYSTEM_PLANET_PICTURES: [u8; 200] = [
    1, 2, 7, 8, 3, 4, 9, 10, 11, 12, 5, 13, 11, 14, 15, 8, 7, 11, 6, 7, 8, 7, 1, 13, 2, 16, 17, 3,
    4, 18, 9, 13, 15, 7, 14, 18, 19, 20, 8, 13, 5, 6, 1, 2, 3, 7, 4, 5, 6, 1, 2, 3, 11, 4, 11, 5,
    6, 1, 2, 3, 9, 4, 5, 11, 19, 6, 1, 13, 19, 11, 7, 2, 13, 3, 13, 8, 4, 5, 6, 1, 1, 8, 2, 3, 4,
    5, 11, 19, 11, 6, 13, 1, 2, 20, 21, 8, 7, 13, 11, 8, 11, 13, 3, 13, 11, 15, 7, 4, 5, 19, 10,
    20, 20, 11, 19, 6, 9, 1, 20, 2, 19, 14, 22, 9, 11, 13, 3, 11, 8, 18, 4, 20, 15, 8, 13, 8, 8,
    19, 13, 8, 5, 19, 13, 6, 1, 13, 20, 8, 19, 9, 11, 13, 2, 3, 13, 18, 4, 5, 7, 11, 19, 11, 6, 13,
    1, 23, 2, 3, 13, 4, 5, 24, 7, 6, 1, 8, 13, 2, 14, 25, 19, 3, 26, 13, 20, 4, 11, 19, 5, 20, 9,
    20, 9, 9, 19, 6, 8, 1, 19, 8,
];

/// A system's planet picture (`SYSTEMSD.DAT` `picture_id`), 1 for a system
/// outside records 100-299.
#[must_use]
pub fn planet_picture_id(dat_id: DatId) -> u8 {
    dat_id
        .index()
        .checked_sub(100)
        .and_then(|index| SYSTEM_PLANET_PICTURES.get(index as usize))
        .copied()
        .unwrap_or(1)
}

/// The compact GOKRES assignment image for a character: major characters
/// 576-579, 592 and 640-641 at index + 17920, minor ones 832-857 and 896-923
/// at index + 18176. `FUN_0042c3b0` draws a character's mini at GOKRES
/// `(class & 0xfff) + 0x4000` (`mission-dialog.md`); the minor range lands on
/// GOKRES 19008 (Ackbar) through 19099 (Villar). hyp: the major offset, until
/// the major characters' class ids are read.
#[must_use]
pub fn character_mini_resource_id(dat_id: DatId, is_major: bool) -> Option<u32> {
    let index = dat_id.index();
    if is_major {
        matches!(index, 576..=579 | 592 | 640..=641).then_some(index + 17_920)
    } else {
        matches!(index, 832..=857 | 896..=923).then_some(index + 18_176)
    }
}
