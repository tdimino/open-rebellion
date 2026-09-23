//! Tactical combat view — 2D battlefield for player-involved space battles.
//!
//! When a battle involves the player's faction, instead of auto-resolving we
//! transition to `GameMode::TacticalCombat` and render this view.
//!
//! # Phases
//!
//! 1. **Placement**: Player positions capital ships in their deployment zone.
//!    Fighters auto-deploy around their carrier. AI side auto-places.
//! 2. **Combat**: Real-time phased combat with HUD (Task #8).
//! 3. **Results**: Summary screen showing losses (Task #10).
//!
//! # Integration
//!
//! ```ignore
//! // In main.rs combat resolution loop:
//! if battle_involves_player {
//!     tactical_state = TacticalState::new_battle(...);
//!     game_mode = GameMode::TacticalCombat;
//! } else {
//!     CombatSystem::resolve_space(...); // auto-resolve AI vs AI
//! }
//! ```

use std::collections::{HashMap, HashSet};

use macroquad::prelude::*;
use rebellion_core::combat::{CombatSide as AutoResolveSide, SpaceCombatResult};
use rebellion_core::ids::{CapitalShipKey, DatId, FighterKey, FleetKey, SystemKey};
use rebellion_core::troop_transport::TroopTransportState;
use rebellion_core::world::GameWorld;

use crate::bmp_cache::{resources, BmpCache, DllSource};
use crate::sector_window::planet_picture_id;
#[cfg(feature = "interface-test-fixtures")]
use crate::tactical_assets::TacticalLodView;
use crate::tactical_assets::{
    OriginalTacticalCamera, TacticalAssetRenderer, TacticalEffectRenderObject,
    TacticalFighterRenderObject, TacticalProjectileRenderObject, TacticalRenderObject,
    TacticalScreenProjection,
};
use crate::tactical_resources::{
    capital_ship_tactical_resource, death_star_tactical_resource, fighter_tactical_resource,
    TacticalCapitalShipResource, TacticalDeathStarResource, TacticalFighterResource,
    DEATH_STAR_TACTICAL_RESOURCE,
};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Battlefield dimensions (logical units).
const ARENA_WIDTH: f32 = 1200.0;
const ARENA_HEIGHT: f32 = 800.0;

/// The original TACTICAL.DLL background is a complete 640×480 composition.
const TACTICAL_WIDTH: f32 = 640.0;
const TACTICAL_HEIGHT: f32 = 480.0;
/// Measured black aperture inside bitmap 1000. All battle primitives stay here.
const BATTLE_APERTURE: NativeRect = NativeRect::new(16.0, 28.0, 444.0, 439.0);

/// Default placement of the modeless 470x331 Battle Alert / Results window
/// on the original 640x480 command-center canvas. The original window is
/// draggable; the deterministic launcher keeps this centered position.
const BATTLE_RESULTS_WINDOW_X: f32 = 85.0;
const BATTLE_RESULTS_WINDOW_Y: f32 = 74.0;
const BATTLE_RESULTS_SCENE_X: f32 = 12.0;
const BATTLE_RESULTS_SCENE_Y: f32 = 13.0;
const BATTLE_RESULTS_RAIL_X: f32 = 412.0;

/// Deployment zone width (fraction of arena width per side).
const DEPLOY_ZONE_FRACTION: f32 = 0.3;

/// Selected-capital-ship display panel.
const TACTICAL_SELECTED_SHIP_PANEL: u32 = 1302;

/// Native meter apertures inside TACTICAL 1302. The left meter is shield
/// strength; the right meter is hull integrity in the selected ship's faction
/// colour, as shown by both faction reference captures.
const TACTICAL_SELECTED_SHIELD_METER: NativeRect = NativeRect::new(514.0, 107.0, 40.0, 8.0);
const TACTICAL_SELECTED_HULL_METER: NativeRect = NativeRect::new(585.0, 107.0, 39.0, 8.0);
const TACTICAL_SELECTED_SHIP_ART_X: f32 = 507.0;
const TACTICAL_SELECTED_SHIP_ART_Y: f32 = 37.0;
/// `FUN_005e45f0` and `FUN_005e7540` place the five condition indicators at
/// panel-local x positions 10, 37, 64, 91, and 118 with y 103.
const TACTICAL_SUBSYSTEM_RESOURCE_BASES: [u32; 5] = [1201, 1206, 1211, 1216, 1221];
const TACTICAL_SUBSYSTEM_X: [f32; 5] = [491.0, 518.0, 545.0, 572.0, 599.0];
const TACTICAL_SUBSYSTEM_Y: f32 = 130.0;

/// Default ship icon size when no sprite is available.
const DEFAULT_SHIP_SIZE: f32 = 40.0;

/// Fighter squadron icon size.
const FIGHTER_SIZE: f32 = 16.0;

/// `_DAT_0066d154`, used by both `FUN_005d3cc0` and `FUN_005d41a0`.
const ORIGINAL_TACTICAL_EFFECT_FRAME_SECONDS: f32 = 0.1;
const ORIGINAL_LASER_PROJECTILE_THRESHOLD: f32 = 28.8;
const ORIGINAL_TURBOLASER_PROJECTILE_THRESHOLD: f32 = 34.666_668;
const ORIGINAL_ION_PROJECTILE_THRESHOLD: f32 = 32.0;
const ORIGINAL_TORPEDO_PROJECTILE_THRESHOLD: f32 = 12.8;
const ORIGINAL_PROJECTILE_DURATION_SHORT: f32 = 1.0;
const ORIGINAL_PROJECTILE_DURATION_LONG: f32 = 2.0;
/// `FUN_005b0330` resets the subsystem-repair timer to 50 * 1000 ms.
const ORIGINAL_SUBSYSTEM_REPAIR_INTERVAL_TICKS: u32 = 200;
/// `FUN_005b05c0` converts the CAPSHPSD sublight rating by 0.4 * 0.6.
const ORIGINAL_SUBLIGHT_ENGINE_SCALE: f32 = 0.24;
/// `FUN_005b9c60` converts the FIGHTSD sublight rating by 0.4 without the
/// capital-ship 0.6 multiplier.
const ORIGINAL_FIGHTER_SUBLIGHT_ENGINE_SCALE: f32 = 0.4;
/// Tactical combat advances four authoritative simulation steps per second.
const ORIGINAL_TACTICAL_STEP_MILLISECONDS: f32 = 250.0;
/// `FUN_005ad750` and `FUN_005afb70` clamp the active mode contribution.
const ORIGINAL_ENGINE_MODE_MIN_BONUS: f32 = 1.0;
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
/// One fighter craft contributes eight floating hull points.
const ORIGINAL_FIGHTER_HULL_PER_CRAFT: f32 = 8.0;
/// `FUN_005b9c00` scales a fighter group's live hull into weapon output.
const ORIGINAL_FIGHTER_WEAPON_HULL_SCALE: f32 = 3.751;
/// `FUN_005b9c30` scales live fighter hull into shield recharge per second.
const ORIGINAL_FIGHTER_SHIELD_HULL_SCALE: f32 = 0.025;
/// `FUN_005b23a0` applies this coefficient to the FIGHTSD torpedo rating.
const ORIGINAL_FIGHTER_TORPEDO_STRENGTH: f32 = f32::from_bits(0x3f08_8889);
/// `FUN_005d2490` and `FUN_005d24e0` target modifiers.
const ORIGINAL_ION_VS_FIGHTER: f32 = 0.0;
const ORIGINAL_LASER_VS_FIGHTER: f32 = 1.0;
const ORIGINAL_TURBOLASER_VS_FIGHTER: f32 = 0.5;
const ORIGINAL_TACTICAL_STEP_SECONDS: f32 = ORIGINAL_TACTICAL_STEP_MILLISECONDS / 1000.0;
/// `FUN_005ba7f0` advances the tactical Death Star laser by elapsed
/// milliseconds times one third times 0.001, capped at 100.
const ORIGINAL_DEATH_STAR_LASER_CHARGE_PER_SECOND: f32 = 1.0 / 3.0;
const ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE: f32 = 100.0;

/// Spacing between auto-placed ships.
const SHIP_SPACING: f32 = 60.0;

const ORIGINAL_BATTLE_BASE_EXTENT: f32 = 100.0;
const ORIGINAL_BATTLE_OBJECT_INCREMENT: f32 = 3.0;
const ORIGINAL_BATTLE_OUTER_LANE_SCALE: f32 = 0.5;
const ORIGINAL_BATTLE_INNER_LANE_OFFSET: f32 = 20.0;
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
    pub(crate) fn from_active_counts(
        first_active_objects: usize,
        second_active_objects: usize,
    ) -> Self {
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
    const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    /// `FUN_005b0f70` initializes the current vector with `FUN_0059fb10`.
    const SOURCE_FORWARD: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    fn normalized(self) -> Self {
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

    fn rendered(self) -> Vec3 {
        // Direct3D retained mode is left-handed and the browser renderer is
        // right-handed. Preserve Y and reflect source Z.
        vec3(self.x, self.y, -self.z)
    }
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

#[derive(Debug, Clone, Copy, PartialEq)]
struct NativeRect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl NativeRect {
    const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    fn contains(self, x: f32, y: f32) -> bool {
        x >= self.x && y >= self.y && x < self.x + self.width && y < self.y + self.height
    }
}

/// One uniformly scaled, letterboxed copy of the original tactical canvas.
#[derive(Debug, Clone, Copy)]
struct TacticalCanvas {
    x: f32,
    y: f32,
    scale: f32,
}

impl TacticalCanvas {
    fn new(width: f32, height: f32) -> Self {
        let scale = (width / TACTICAL_WIDTH)
            .min(height / TACTICAL_HEIGHT)
            .max(0.01);
        Self {
            x: (width - TACTICAL_WIDTH * scale) * 0.5,
            y: (height - TACTICAL_HEIGHT * scale) * 0.5,
            scale,
        }
    }

    fn point(self, x: f32, y: f32) -> (f32, f32) {
        (self.x + x * self.scale, self.y + y * self.scale)
    }

    fn logical_pointer(self, x: f32, y: f32) -> (f32, f32) {
        ((x - self.x) / self.scale, (y - self.y) / self.scale)
    }

    fn aperture(self) -> NativeRect {
        let (x, y) = self.point(BATTLE_APERTURE.x, BATTLE_APERTURE.y);
        NativeRect::new(
            x,
            y,
            BATTLE_APERTURE.width * self.scale,
            BATTLE_APERTURE.height * self.scale,
        )
    }

    fn arena_transform(self, zoom: f32, camera_x: f32, camera_y: f32) -> (f32, f32, f32) {
        let aperture = self.aperture();
        let scale = (aperture.width / ARENA_WIDTH).min(aperture.height / ARENA_HEIGHT) * zoom;
        let x = aperture.x + aperture.width * 0.5 - (ARENA_WIDTH * 0.5 + camera_x) * scale;
        let y = aperture.y + aperture.height * 0.5 - (ARENA_HEIGHT * 0.5 + camera_y) * scale;
        (scale, x, y)
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

/// Original right-rail destination inside the Battle Results window.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BattleResultTab {
    #[default]
    Summary,
    AllianceForces,
    ImperialForces,
    GoDirectlyTo,
}

/// Unit family selected inside an Alliance or Imperial force-detail tab.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BattleResultCategory {
    #[default]
    CapitalShips,
    Fighters,
    Troops,
    Personnel,
}

/// One command from the original unified tactical order field at source offset `+0x68`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TacticalOrder {
    #[default]
    None,
    Recover,
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
    /// Exact numeric value consumed by the original tactical runtime.
    #[must_use]
    pub const fn source_code(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Recover => 2,
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
    const fn label(self) -> &'static str {
        match self {
            Self::None => "No Orders",
            Self::Recover => "Recover",
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

    #[must_use]
    const fn maneuver_graphic(self) -> Option<u32> {
        use resources::tactical as art;
        match self {
            Self::LeftHook => Some(art::MANEUVER_HUD_LEFT_HOOK),
            Self::RightHook => Some(art::MANEUVER_HUD_RIGHT_HOOK),
            Self::Hammer => Some(art::MANEUVER_HUD_HAMMER),
            Self::Anvil => Some(art::MANEUVER_HUD_ANVIL),
            Self::HoldPosition => Some(art::MANEUVER_HUD_HOLD_POSITION),
            _ => None,
        }
    }

    #[must_use]
    const fn mission_graphic(self, player_is_alliance: bool) -> Option<u32> {
        use resources::tactical as art;
        match (self, player_is_alliance) {
            (Self::AttackCapitalShips, true) => {
                Some(art::MISSIONS_HUD_ATTACK_CAPITAL_SHIPS_ALLIANCE)
            }
            (Self::AttackFighters, true) => Some(art::MISSIONS_HUD_ATTACK_FIGHTERS_ALLIANCE),
            (Self::Recover, true) => Some(art::MISSIONS_HUD_RECOVER_ALLIANCE),
            (Self::AttackDeathStar, true) => Some(art::MISSIONS_HUD_ATTACK_DEATH_STAR_ALLIANCE),
            (Self::AttackCapitalShips, false) => {
                Some(art::MISSIONS_HUD_ATTACK_CAPITAL_SHIPS_EMPIRE)
            }
            (Self::AttackFighters, false) => Some(art::MISSIONS_HUD_ATTACK_FIGHTERS_EMPIRE),
            (Self::Recover, false) => Some(art::MISSIONS_HUD_RECOVER_EMPIRE),
            _ => None,
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
    #[must_use]
    pub const fn source_code(self) -> u8 {
        match self {
            Self::Surround => 1,
            Self::StandOff => 2,
        }
    }

    #[must_use]
    const fn label(self) -> &'static str {
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
    /// Separate original Death Star resource path, when applicable.
    pub death_star_resource: Option<TacticalDeathStarResource>,
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
    /// True if this ship is retreating (moving off-screen).
    pub retreating: bool,
    /// Retreat progress: 0.0 = just started, 1.0 = off-screen (removed from combat).
    pub retreat_progress: f32,
    /// True if this ship successfully retreated (survived, not destroyed).
    pub retreated: bool,
}

/// Capital-ship collision volume consumed by `FUN_005b2e60`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OriginalTacticalCollisionEnvelope {
    pub vertical_radius: f32,
    pub planar_diameter: f32,
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
    fn refresh_subsystem_condition(&mut self) {
        self.subsystem_condition = TacticalSubsystemCondition::from_source_state(
            self.hull_current,
            self.hull_max,
            self.subsystem_capacity,
            self.subsystem_damage,
        );
    }

    fn add_subsystem_hit(&mut self, kind: TacticalSubsystemKind) -> bool {
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

    fn effective_weapon_recharge(&self) -> f32 {
        (self.hull_ratio() * self.weapon_recharge_rate
            - self.weapon_recharge_rate * 0.25 * f32::from(self.subsystem_damage.weapons))
        .max(0.0)
    }

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
    /// FIGHTSD torpedo rating and range. The rating is not consumed as ammo.
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
    /// Unified source order code assigned through the command panel.
    pub order: TacticalOrder,
    /// Source Surround or Stand Off behavior assigned with a maneuver.
    pub tactic: TacticalTactic,
    /// Active target selected by a source attack-order executor.
    pub attack_target: Option<TacticalAttackTarget>,
    /// Current `FUN_005cf980` recovery executor state.
    pub recovery_state: TacticalFighterRecoveryState,
    /// Capital-ship index reserved as this squadron's recovery carrier.
    pub recovery_target: Option<usize>,
}

impl TacticalFighter {
    fn effective_maneuverability(&self) -> f32 {
        (self.maneuverability as f32 + 1.0).clamp(1.0, ORIGINAL_CAPITAL_MANEUVERABILITY_MAX)
    }

    fn weapon_hull_factor(&self) -> f32 {
        self.hull_current.max(0.0) * ORIGINAL_FIGHTER_WEAPON_HULL_SCALE
            / ORIGINAL_FIGHTER_HULL_PER_CRAFT
    }

    fn shield_recharge_rate(&self) -> f32 {
        self.hull_current.max(0.0) * ORIGINAL_FIGHTER_SHIELD_HULL_SCALE
            / ORIGINAL_FIGHTER_HULL_PER_CRAFT
    }

    fn refresh_craft_count(&mut self) {
        self.hull_current = self.hull_current.clamp(0.0, self.hull_max.max(0.0));
        self.squad_count = if self.hull_current <= 0.0 {
            0
        } else {
            (self.hull_current / ORIGINAL_FIGHTER_HULL_PER_CRAFT).ceil() as u32
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

/// The Death Star is a manager-owned tactical object rather than a member of
/// the 29-entry capital mesh table. `FUN_005ab0e0` and `FUN_005ab650` select
/// its 5010 or 5020 sprite independently from normal capital participants.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TacticalDeathStar {
    pub resource: TacticalDeathStarResource,
    pub is_attacker: bool,
    pub is_alliance: bool,
    pub source_position: TacticalWorldPosition,
    /// Source field `+0x68`; zero makes `FUN_005ba420` non-operational.
    pub hull: f32,
    /// Source field `+0x6c`, filled by `FUN_005ba7f0` from zero to 100.
    pub laser_charge: f32,
    /// Source field `+0x74`; selects resource 5020 when set.
    pub destroyed: bool,
    /// Source field `+0x78`; set when the Death Star action is committed.
    pub action_committed: bool,
}

/// One committed superlaser shot. The original keeps the Death Star action
/// committed until its retained beam presentation reaches the target.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TacticalDeathStarBeam {
    pub target: usize,
    pub source_position: TacticalWorldPosition,
    pub target_position: TacticalWorldPosition,
    pub elapsed: f32,
    pub duration: f32,
}

impl TacticalDeathStar {
    #[must_use]
    pub fn operational(self) -> bool {
        !self.destroyed && self.hull > 0.0 && !self.action_committed
    }

    #[must_use]
    pub fn resource_id(self) -> u32 {
        if self.destroyed {
            self.resource.resource_base_with_flag
        } else {
            self.resource.resource_base_without_flag
        }
    }

    fn advance_laser_charge(&mut self, seconds: f32) {
        if self.hull > 0.0 && !self.destroyed && seconds > 0.0 {
            self.laser_charge = (self.laser_charge
                + seconds * ORIGINAL_DEATH_STAR_LASER_CHARGE_PER_SECOND)
                .min(ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE);
        }
    }
}

// ---------------------------------------------------------------------------
// BattleSession
// ---------------------------------------------------------------------------

/// All state for one tactical combat encounter.
#[derive(Debug, Clone)]
pub struct BattleSession {
    /// System where the battle takes place.
    pub system: SystemKey,
    pub system_name: String,
    /// Original `SYSTEMSD.picture_id`; selects the active tactical palette.
    pub system_picture_id: u8,
    /// Attacker fleet key (in `GameWorld`).
    pub attacker_fleet: FleetKey,
    /// Defender fleet key (in `GameWorld`).
    pub defender_fleet: FleetKey,
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
    /// Active weapon fire visual effects (source ship idx -> target ship idx).
    pub weapon_effects: Vec<WeaponEffect>,
    /// Target-attached original type-303 hit, damage, and destruction sequences.
    pub impact_effects: Vec<TacticalImpactEffect>,
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

    fn color(&self) -> Color {
        match self.color_selector {
            1 => Color::new(0.0, 1.0, 0.0, 1.0),
            2 => Color::new(0.0, 0.0, 1.0, 1.0),
            _ => Color::new(1.0, 0.0, 0.0, 1.0),
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

    #[must_use]
    pub const fn frame_count(self) -> u8 {
        match self {
            Self::IonHit | Self::StandardHit => 6,
            Self::IonDamage | Self::Destroyed => 16,
            Self::StandardDamage | Self::TurbolaserDamage => 7,
        }
    }

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
enum OriginalTacticalImpactStage {
    Hit = 0,
    Damage = 1,
    Destroyed = 2,
}

/// Type of weapon for visual rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponKind {
    Turbolaser,
    IonCannon,
    LaserCannon,
    FighterAttack,
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
    /// `SHIP_TAKE_*_HIT` codes delivered to the original render dispatcher.
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

fn queue_original_tactical_impact(
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

fn queue_original_tactical_projectile(
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

fn refresh_original_subsystem_conditions(
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

fn attempt_original_subsystem_repair(
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OriginalTacticalDamageRolls {
    shield: u8,
    hull: u8,
}

impl OriginalTacticalDamageRolls {
    fn from_seed(seed: u32) -> Self {
        // The executable requests two inclusive 0..100 draws. Its global RNG
        // sequence is not yet joined to the Rust battle session, so preserve
        // the recovered range and deterministic replay using the existing
        // combat seed until that larger state contract is recovered.
        Self {
            shield: u8::try_from(seed % 101).unwrap_or(100),
            hull: u8::try_from(seed.rotate_left(13).wrapping_add(37) % 101).unwrap_or(100),
        }
    }
}

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

fn apply_original_capital_damage(
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
        if shield_before > 0 {
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
            if let Some(kind) = original_hull_subsystem_kind(score) {
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
    ) -> Self {
        let system_name = world
            .systems
            .get(system)
            .map_or_else(|| "Unknown".into(), |s| s.name.clone());
        let system_picture_id = world
            .systems
            .get(system)
            .map_or(1, |system| planet_picture_id(system.dat_id));

        let mut ships = Vec::new();
        let mut fighters = Vec::new();

        // Expand attacker fleet into individual hulls.
        Self::expand_fleet(world, attacker, true, &mut ships, &mut fighters);
        // Expand defender fleet.
        Self::expand_fleet(world, defender, false, &mut ships, &mut fighters);

        // Retain the working 2D fallback separately from the recovered source
        // world coordinates. P58 will replace each fallback only after its
        // DAT-to-tactical resource identity is proven.
        Self::auto_place_ships(&mut ships);
        Self::auto_place_fighters(&mut fighters, &ships);
        let source_layout = Self::assign_original_world_positions(&mut ships, &mut fighters);
        let navigation_sets = original_navigation_point_sets(source_layout);
        Self::assign_original_task_forces(&mut ships);
        Self::assign_original_fighter_groups(&mut fighters);
        let death_star = [(attacker, true), (defender, false)].into_iter().find_map(
            |(fleet_key, is_attacker)| {
                let fleet = &world.fleets[fleet_key];
                fleet.has_death_star.then_some(TacticalDeathStar {
                    resource: DEATH_STAR_TACTICAL_RESOURCE,
                    is_attacker,
                    is_alliance: fleet.is_alliance,
                    source_position: TacticalWorldPosition {
                        x: 0.0,
                        y: 0.0,
                        z: if fleet.is_alliance {
                            source_layout.outer_negative_z
                        } else {
                            source_layout.outer_positive_z
                        },
                    },
                    hull: ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE,
                    laser_charge: 0.0,
                    destroyed: false,
                    action_committed: false,
                })
            },
        );

        // The original Tactical Display opens with the battle paused. The
        // Battle Alert's Take Command choice is the entry action, not a second
        // invented placement phase inside the battle viewport.
        let selected_ship = ships
            .iter()
            .position(|ship| ship.is_attacker == player_is_attacker);
        if let Some(index) = selected_ship {
            ships[index].selected = true;
        }

        BattleSession {
            system,
            system_name,
            system_picture_id,
            attacker_fleet: attacker,
            defender_fleet: defender,
            attacker_is_alliance: world.fleets[attacker].is_alliance,
            player_is_attacker,
            phase: BattlePhase::Combat,
            ships,
            fighters,
            death_star,
            death_star_beam: None,
            trench_run_outcome: None,
            trench_run_cinematic_pending: false,
            navigation_sets,
            source_layout,
            selected_ship,
            selected_fighter_group: None,
            placement_confirmed: true,
            start_tick: tick,
            combat_tick: 0,
            weapon_effects: Vec::new(),
            impact_effects: Vec::new(),
            field_effects: Vec::new(),
            subsystem_repairs: Vec::new(),
            paused: true,
            combat_speed: 1,
            step_accumulator: 0.0,
            winner: None,
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
    fn advance_original_tactical_movement(&mut self, delta_milliseconds: f32) {
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
                        if ship.source_waypoint.is_none() {
                            ship.order = TacticalOrder::None;
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
    }

    /// Advance the recovered `FUN_005cf980` fighter states. State 2 moves the
    /// group toward its reserved carrier, the exact 2.0-unit gate enters state
    /// 3, and the following docking callback removes the tactical object in
    /// state 4 without changing its strategic squadron count.
    fn advance_original_fighter_recovery(&mut self, delta_milliseconds: f32) {
        if !delta_milliseconds.is_finite() || delta_milliseconds <= 0.0 {
            return;
        }
        let seconds = delta_milliseconds * 0.001;
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
                    fighter.order = TacticalOrder::None;
                    fighter.selected = false;
                    fighter.alive = false;
                }
                TacticalFighterRecoveryState::AwaitingCarrier
                | TacticalFighterRecoveryState::Reserved
                | TacticalFighterRecoveryState::Recovered => {}
            }
        }
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
        for (ship_idx, ship) in fleet
            .capital_ships
            .iter()
            .enumerate()
            .filter(|(_, ship)| ship.alive)
        {
            let class = &world.capital_ship_classes[ship.class];
            let tactical_resource = capital_ship_tactical_resource(class.dat_id);
            let death_star_resource = death_star_tactical_resource(class.dat_id);

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
            let subsystem_damage = TacticalSubsystemDamage::default();
            ships.push(TacticalShip {
                class_key: ship.class,
                identity: TacticalObjectIdentity {
                    class_dat_id: class.dat_id,
                    fleet_roster_index: ship_idx,
                    is_alliance: fleet.is_alliance,
                },
                tactical_resource,
                death_star_resource,
                source_position: TacticalWorldPosition::ORIGIN,
                source_forward: TacticalWorldVector::SOURCE_FORWARD,
                source_desired_forward: TacticalWorldVector::ZERO,
                source_velocity: TacticalWorldVector::ZERO,
                source_waypoint: None,
                navigation_route: Vec::new(),
                source_collision_envelope: None,
                name: class.name.clone(),
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
                retreating: false,
                retreat_progress: 0.0,
                retreated: false,
            });
        }

        for (fighter_idx, entry) in fleet.fighters.iter().enumerate() {
            let class = &world.fighter_classes[entry.class];
            for squadron_index in 0..entry.count {
                let squad_count = class.squadron_size;
                let hull = squad_count as f32 * ORIGINAL_FIGHTER_HULL_PER_CRAFT;
                fighters.push(TacticalFighter {
                    class_key: entry.class,
                    identity: TacticalObjectIdentity {
                        class_dat_id: class.dat_id,
                        fleet_roster_index: fighter_idx,
                        is_alliance: fleet.is_alliance,
                    },
                    tactical_resource: fighter_tactical_resource(class.dat_id),
                    source_position: TacticalWorldPosition::ORIGIN,
                    sublight_engine_power: class.sub_light_engine as f32
                        * ORIGINAL_FIGHTER_SUBLIGHT_ENGINE_SCALE,
                    fleet_fighter_index: fighter_idx,
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
                    torpedo_strength: class.torpedoes,
                    torpedo_range: class.torpedoes_range as f32,
                    maneuverability: class.maneuverability,
                    is_attacker,
                    alive: squad_count > 0,
                    // FUN_005c63f0 initializes color state to 4 (white/unassigned).
                    fighter_group: 4,
                    selected: false,
                    order: TacticalOrder::None,
                    tactic: TacticalTactic::StandOff,
                    attack_target: None,
                    recovery_state: TacticalFighterRecoveryState::AwaitingCarrier,
                    recovery_target: None,
                });
            }
        }
    }

    /// Bind every active battle participant to the source X-slot and Z-lane
    /// initialization performed by `FUN_005ab650` through `FUN_005a9030`.
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
    /// more than seven squadrons.
    fn assign_original_fighter_groups(fighters: &mut [TacticalFighter]) {
        for is_alliance in [true, false] {
            let mut groups: [Vec<usize>; 4] = std::array::from_fn(|_| Vec::new());
            for (index, fighter) in fighters.iter().enumerate().filter(|(_, fighter)| {
                fighter.alive
                    && fighter.squad_count > 0
                    && fighter.identity.is_alliance == is_alliance
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
    fn refresh_original_task_force_geometry(ships: &mut [TacticalShip]) {
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
    /// Ships are stacked vertically, centered.
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

    fn advance_presentational_effects(&mut self, elapsed: f32) {
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
                if let Some(death_star) = self.death_star.as_mut() {
                    death_star.action_committed = false;
                }
                macroquad::logging::info!(
                    "[tactical_death_star] superlaser_resolved target_object_id={}",
                    beam.target + 1
                );
            }
        }
    }

    fn commit_death_star_shot(&mut self, target: usize) -> bool {
        if self.death_star_beam.is_some()
            || self
                .ships
                .get(target)
                .is_none_or(|ship| !ship.alive || ship.is_attacker == self.player_is_attacker)
        {
            return false;
        }
        let Some(death_star) = self.death_star.as_mut().filter(|death_star| {
            death_star.is_attacker == self.player_is_attacker
                && death_star.operational()
                && death_star.laser_charge >= ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE
        }) else {
            return false;
        };
        let target_position = self.ships[target].source_position;
        death_star.action_committed = true;
        death_star.laser_charge = 0.0;
        self.death_star_beam = Some(TacticalDeathStarBeam {
            target,
            source_position: death_star.source_position,
            target_position,
            elapsed: 0.0,
            duration: ORIGINAL_PROJECTILE_DURATION_LONG,
        });
        macroquad::logging::info!(
            "[tactical_death_star] superlaser_committed target_object_id={} charge=0",
            target + 1
        );
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

        let trench_run_was_active = self.has_active_trench_run();
        self.combat_tick += 1;
        self.subsystem_repairs.clear();

        // Process retreating ships: advance retreat progress, move off-screen.
        for ship in &mut self.ships {
            if ship.retreating && ship.alive {
                ship.retreat_progress += 0.05; // ~20 ticks to fully retreat
                                               // Move ship toward the edge.
                let retreat_dir = if ship.is_attacker { -1.0 } else { 1.0 };
                ship.x += retreat_dir * 15.0;
                if ship.retreat_progress >= 1.0 {
                    // Ship has escaped — remove from combat but NOT destroyed.
                    // It survives but doesn't participate further.
                    ship.alive = false;
                    ship.retreated = true;
                }
            }
        }
        self.advance_original_tactical_movement(ORIGINAL_TACTICAL_STEP_MILLISECONDS);
        self.advance_original_fighter_recovery(ORIGINAL_TACTICAL_STEP_MILLISECONDS);
        refresh_original_attack_targets(self);
        for ship in &mut self.ships {
            ship.advance_original_recharge(ORIGINAL_TACTICAL_STEP_SECONDS);
        }
        for fighter in &mut self.fighters {
            fighter.advance_original_recharge(ORIGINAL_TACTICAL_STEP_SECONDS);
        }
        if let Some(death_star) = self.death_star.as_mut() {
            death_star.advance_laser_charge(ORIGINAL_TACTICAL_STEP_SECONDS);
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
        Self::fire_side(
            &mut self.ships,
            &mut self.field_effects,
            &atk_alive,
            &def_alive,
            &mut new_effects,
            &mut self.impact_effects,
            self.combat_tick,
        );
        Self::fire_side(
            &mut self.ships,
            &mut self.field_effects,
            &def_alive,
            &atk_alive,
            &mut new_effects,
            &mut self.impact_effects,
            self.combat_tick,
        );
        self.fire_capitals_at_fighters(&mut new_effects);
        self.weapon_effects.extend(new_effects);

        // FUN_005b0330 invokes FUN_005b1490 every 50,000 ms. Four combat
        // steps represent one source second, so one repair attempt occurs per
        // ship every 200 steps. The global executable RNG sequence remains a
        // separate parity gate; ranges and deterministic replay are preserved.
        if self
            .combat_tick
            .is_multiple_of(ORIGINAL_SUBSYSTEM_REPAIR_INTERVAL_TICKS)
        {
            for (ship_index, ship) in self.ships.iter_mut().enumerate() {
                let hits = ship.subsystem_damage.total_hits();
                if hits == 0 {
                    continue;
                }
                let seed = self
                    .combat_tick
                    .wrapping_mul(53)
                    .wrapping_add(ship_index as u32 * 29)
                    .wrapping_add(11);
                let chance_roll = u8::try_from(seed % 100 + 1).unwrap_or(100);
                let selection_roll =
                    u16::try_from(seed.rotate_left(9) % u32::from(hits) + 1).unwrap_or(hits);
                if let Some(repair) =
                    attempt_original_subsystem_repair(ship, ship_index, chance_roll, selection_roll)
                {
                    self.subsystem_repairs.push(repair);
                }
            }
            refresh_original_subsystem_conditions(&mut self.ships, &self.field_effects);
        }

        // Fighter batteries use the same 250 ms source cadence as capital
        // batteries. Their own readiness and recharge record gates each shot.
        self.fighter_step();
        if trench_run_was_active
            && self.trench_run_outcome.is_none()
            && self
                .death_star
                .is_some_and(|death_star| death_star.hull > 0.0 && !death_star.destroyed)
            && !self.has_active_trench_run()
        {
            self.record_trench_run_outcome(TacticalTrenchRunOutcome::Failure);
        }

        // Check for battle end.
        let atk_remaining = self.ships.iter().any(|s| s.is_attacker && s.alive)
            || self
                .fighters
                .iter()
                .any(|f| f.is_attacker && f.alive && f.squad_count > 0)
            || self
                .death_star
                .is_some_and(|death_star| death_star.is_attacker && !death_star.destroyed);
        let def_remaining = self.ships.iter().any(|s| !s.is_attacker && s.alive)
            || self
                .fighters
                .iter()
                .any(|f| !f.is_attacker && f.alive && f.squad_count > 0)
            || self
                .death_star
                .is_some_and(|death_star| !death_star.is_attacker && !death_star.destroyed);

        if !atk_remaining || !def_remaining {
            self.winner = Some(match (atk_remaining, def_remaining) {
                (true, false) => CombatWinner::Attacker,
                (false, true) => CombatWinner::Defender,
                _ => CombatWinner::Draw,
            });
            self.phase = BattlePhase::Results;
            return true;
        }

        false
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
        tick: u32,
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
                    // The recovered Attack Fighters executor owns a fighter
                    // target. Its arc-based weapon callback remains a separate
                    // source-recovery gate, so do not redirect the shot to an
                    // unrelated capital ship.
                    continue;
                }
                None if ships[fire_idx].order == TacticalOrder::AttackCapitalShips => {
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
                    let damage = original_tactical_integer_damage(candidate.strength);
                    let kind_seed = match kind {
                        WeaponKind::LaserCannon => 0x0d,
                        WeaponKind::Turbolaser => 0x0e,
                        WeaponKind::IonCannon => 0x0f,
                        WeaponKind::FighterAttack => 0x10,
                    };
                    let seed = tick
                        .wrapping_mul(31)
                        .wrapping_add(fire_idx as u32 * 17)
                        .wrapping_add(arc_index as u32 * 43)
                        .wrapping_add(target_idx as u32 * 0x9e37)
                        .wrapping_add(kind_seed);
                    let impact_stage = apply_original_capital_damage(
                        ships,
                        field_effects,
                        target_idx,
                        damage,
                        OriginalTacticalDamageRolls::from_seed(seed),
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

    fn fire_capitals_at_fighters(&mut self, effects: &mut Vec<WeaponEffect>) {
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
                apply_original_fighter_damage(
                    &mut self.fighters,
                    target,
                    strength,
                    source_maneuverability,
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
            (TacticalOrder::AttackDeathStar, _) => Some(OriginalFighterTarget::DeathStar),
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

    fn fighter_target_strength(&self, source: usize, target: OriginalFighterTarget) -> f32 {
        let Some(fighter) = self
            .fighters
            .get(source)
            .filter(|fighter| fighter.alive && fighter.squad_count > 0 && fighter.weapon_arc.ready)
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
            strength += fighter.torpedo_strength as f32 * ORIGINAL_FIGHTER_TORPEDO_STRENGTH;
        }
        strength
    }

    /// Fire each live fighter group through the source fore battery record.
    fn fighter_step(&mut self) {
        let death_star_was_destroyed = self
            .death_star
            .is_some_and(|death_star| death_star.destroyed);
        let mut effects = Vec::new();
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
                        let stage = apply_original_capital_damage(
                            &mut self.ships,
                            &mut self.field_effects,
                            index,
                            original_tactical_integer_damage(strength),
                            OriginalTacticalDamageRolls::from_seed(
                                self.combat_tick
                                    .wrapping_mul(61)
                                    .wrapping_add(source as u32 * 31)
                                    .wrapping_add(index as u32 * 17)
                                    .wrapping_add(kind.original_impact_code() as u32),
                            ),
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
                        apply_original_fighter_damage(
                            &mut self.fighters,
                            index,
                            strength,
                            source_snapshot.effective_maneuverability(),
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
                        if death_star.hull == 0.0 {
                            death_star.destroyed = true;
                            death_star.action_committed = false;
                        }
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
                fired = true;
            }

            if matches!(
                target,
                OriginalFighterTarget::Capital(_) | OriginalFighterTarget::DeathStar
            ) && target_was_unshielded
                && source_snapshot.torpedo_strength > 0
                && distance <= source_snapshot.torpedo_range
            {
                let strength =
                    source_snapshot.torpedo_strength as f32 * ORIGINAL_FIGHTER_TORPEDO_STRENGTH;
                if let OriginalFighterTarget::Capital(index) = target {
                    let stage = apply_original_capital_damage(
                        &mut self.ships,
                        &mut self.field_effects,
                        index,
                        original_tactical_integer_damage(strength),
                        OriginalTacticalDamageRolls::from_seed(
                            self.combat_tick
                                .wrapping_mul(67)
                                .wrapping_add(source as u32 * 37)
                                .wrapping_add(index as u32 * 23),
                        ),
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
                    if death_star.hull == 0.0 {
                        death_star.destroyed = true;
                        death_star.action_committed = false;
                    }
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
            }
            if fired {
                self.fighters[source].consume_weapon_arc();
            }
        }
        self.weapon_effects.extend(effects);
        if !death_star_was_destroyed
            && self
                .death_star
                .is_some_and(|death_star| death_star.destroyed)
        {
            self.record_trench_run_outcome(TacticalTrenchRunOutcome::Success);
        }
    }

    fn has_active_trench_run(&self) -> bool {
        let Some(death_star) = self
            .death_star
            .filter(|death_star| death_star.hull > 0.0 && !death_star.destroyed)
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

    fn record_trench_run_outcome(&mut self, outcome: TacticalTrenchRunOutcome) {
        if self.trench_run_outcome.is_some() {
            return;
        }
        self.trench_run_outcome = Some(outcome);
        self.trench_run_cinematic_pending = true;
        self.paused = true;
    }

    fn take_pending_trench_run_cinematic(&mut self) -> Option<TacticalTrenchRunOutcome> {
        if !self.trench_run_cinematic_pending {
            return None;
        }
        self.trench_run_cinematic_pending = false;
        self.trench_run_outcome
    }
}

// ---------------------------------------------------------------------------
// TacticalState — top-level UI state
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum TacticalCommandPanel {
    #[default]
    Display,
    Maneuvers {
        pending_order: TacticalOrder,
        pending_tactic: TacticalTactic,
    },
    Missions {
        pending_order: TacticalOrder,
    },
}

#[derive(Debug, Clone, Copy)]
struct TacticalCameraBookmark {
    source: OriginalTacticalCamera,
    camera_x: f32,
    camera_y: f32,
    zoom: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TacticalCameraFollow {
    TaskForce(u8),
    FighterGroup(u8),
}

/// Persistent state for the tactical combat view across frames.
pub struct TacticalState {
    /// The current battle session (None when not in combat).
    pub session: Option<BattleSession>,
    /// Ship being dragged during placement (index in session.ships).
    pub dragging_ship: Option<usize>,
    /// Mouse offset from ship center during drag.
    pub drag_offset: (f32, f32),
    /// Camera offset for panning the battlefield.
    pub camera_x: f32,
    pub camera_y: f32,
    /// Zoom level.
    pub zoom: f32,
    /// Independently visible navigation-point sets 1 through 4.
    navigation_sets_visible: [bool; 4],
    logged_navigation_scene: Option<[bool; 4]>,
    /// Default and player-memorized camera states for resources 1040–1043.
    default_camera: Option<TacticalCameraBookmark>,
    memorized_camera: Option<TacticalCameraBookmark>,
    /// Keyboard-driven task-force or fighter-group chase target.
    camera_follow: Option<TacticalCameraFollow>,
    /// Original Tactical Display faction-wireframe switches.
    pub highlight_alliance: bool,
    pub highlight_empire: bool,
    /// Active source tab in the post-battle Battle Results window.
    pub battle_result_tab: BattleResultTab,
    /// Active force family in the Alliance or Imperial result tab.
    pub battle_result_category: BattleResultCategory,
    /// The strategic world already consumed the auto-resolve result. Closing
    /// the result window must not apply the same losses a second time.
    strategic_results_applied: bool,
    /// Whether the authentic Battle Options panel owns the right-hand housing.
    battle_options_open: bool,
    /// Captured Battle Options control. Dispatch occurs only on a matching release.
    battle_options_pressed: Option<TacticalBattleOptionsControl>,
    /// The native withdrawal confirmation replaces the right-hand options panel.
    withdraw_confirmation_open: bool,
    /// Captured confirmation control. Dispatch occurs only on a matching release.
    withdraw_confirmation_pressed: Option<TacticalWithdrawConfirmationControl>,
    /// The player handed command of their force to the tactical simulation.
    player_observing: bool,
    /// Captured press on the source Death Star superlaser control.
    death_star_laser_pressed: bool,
    /// The next valid right-click assigns the superlaser target.
    death_star_targeting: bool,
    command_panel: TacticalCommandPanel,
    warmed_font_sizes: HashSet<u16>,
    render_original_participants: bool,
    render_original_starfield: bool,
    render_original_planet: bool,
    render_pyrotechnics: bool,
    display_holocube: bool,
    asset_renderer: TacticalAssetRenderer,
    #[cfg(feature = "interface-test-fixtures")]
    suppress_mapped_capital_fallback: bool,
    #[cfg(feature = "interface-test-fixtures")]
    suppress_mapped_fighter_fallback: bool,
    #[cfg(feature = "interface-test-fixtures")]
    proof_resource_2560: bool,
    #[cfg(feature = "interface-test-fixtures")]
    proof_lod_follows_zoom: bool,
}

impl Default for TacticalState {
    fn default() -> Self {
        Self {
            session: None,
            dragging_ship: None,
            drag_offset: (0.0, 0.0),
            camera_x: 0.0,
            camera_y: 0.0,
            zoom: 1.0,
            navigation_sets_visible: [false; 4],
            logged_navigation_scene: None,
            default_camera: None,
            memorized_camera: None,
            camera_follow: None,
            highlight_alliance: true,
            highlight_empire: true,
            battle_result_tab: BattleResultTab::Summary,
            battle_result_category: BattleResultCategory::CapitalShips,
            strategic_results_applied: false,
            battle_options_open: false,
            battle_options_pressed: None,
            withdraw_confirmation_open: false,
            withdraw_confirmation_pressed: None,
            player_observing: false,
            death_star_laser_pressed: false,
            death_star_targeting: false,
            command_panel: TacticalCommandPanel::Display,
            warmed_font_sizes: HashSet::new(),
            render_original_participants: true,
            render_original_starfield: true,
            render_original_planet: true,
            render_pyrotechnics: true,
            display_holocube: true,
            asset_renderer: TacticalAssetRenderer::default(),
            #[cfg(feature = "interface-test-fixtures")]
            suppress_mapped_capital_fallback: false,
            #[cfg(feature = "interface-test-fixtures")]
            suppress_mapped_fighter_fallback: false,
            #[cfg(feature = "interface-test-fixtures")]
            proof_resource_2560: false,
            #[cfg(feature = "interface-test-fixtures")]
            proof_lod_follows_zoom: false,
        }
    }
}

impl TacticalState {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Begin a new battle — sets session and resets UI state.
    pub fn begin_battle(
        &mut self,
        world: &GameWorld,
        system: SystemKey,
        attacker: FleetKey,
        defender: FleetKey,
        player_is_attacker: bool,
        tick: u64,
    ) {
        let session =
            BattleSession::new(world, system, attacker, defender, player_is_attacker, tick);
        let player_is_empire = player_is_attacker != session.attacker_is_alliance;
        self.asset_renderer
            .set_palette_selector(session.system_picture_id);
        self.asset_renderer
            .enable_original_camera(player_is_empire, session.source_layout);
        self.session = Some(session);
        self.dragging_ship = None;
        self.drag_offset = (0.0, 0.0);
        self.camera_x = 0.0;
        self.camera_y = 0.0;
        self.zoom = 1.0;
        self.navigation_sets_visible = [false; 4];
        self.logged_navigation_scene = None;
        self.default_camera =
            self.asset_renderer
                .camera_state()
                .map(|source| TacticalCameraBookmark {
                    source,
                    camera_x: self.camera_x,
                    camera_y: self.camera_y,
                    zoom: self.zoom,
                });
        self.memorized_camera = None;
        self.camera_follow = None;
        self.highlight_alliance = true;
        self.highlight_empire = true;
        self.battle_result_tab = BattleResultTab::Summary;
        self.battle_result_category = BattleResultCategory::CapitalShips;
        self.strategic_results_applied = false;
        self.battle_options_open = false;
        self.battle_options_pressed = None;
        self.withdraw_confirmation_open = false;
        self.withdraw_confirmation_pressed = None;
        self.player_observing = false;
        self.death_star_laser_pressed = false;
        self.death_star_targeting = false;
        self.command_panel = TacticalCommandPanel::Display;
        self.render_original_participants = true;
        self.render_original_starfield = true;
        self.render_original_planet = true;
        self.render_pyrotechnics = true;
        self.display_holocube = true;
        #[cfg(feature = "interface-test-fixtures")]
        {
            self.suppress_mapped_capital_fallback = false;
            self.suppress_mapped_fighter_fallback = false;
            self.proof_resource_2560 = false;
            self.proof_lod_follows_zoom = false;
        }
    }

    /// Apply the five original Tactical Display options. The original Game
    /// Options screen disables these controls while a battle is in progress,
    /// so production callers update them only from outside tactical combat.
    pub fn set_display_options(&mut self, flags: [bool; 5]) {
        self.render_original_starfield = flags[0];
        self.render_original_planet = flags[1];
        self.render_pyrotechnics = flags[2];
        self.asset_renderer.set_high_detail(flags[3]);
        self.display_holocube = flags[4];
    }

    /// Test-only direct state for the source-backed empty-space presentation.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_empty_space_fixture(&mut self) {
        self.render_original_starfield = true;
        self.render_original_planet = false;
    }

    /// Enable the source-bound tactical LOD proof in isolated fixture builds.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn enable_resource_2560_proof(&mut self) {
        self.sync_proof_palette();
        self.proof_resource_2560 = true;
    }

    /// Set a deterministic source-coordinate LOD view for browser acceptance.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn set_tactical_lod_fixture(&mut self, view: TacticalLodView) {
        self.sync_proof_palette();
        self.proof_resource_2560 = true;
        self.proof_lod_follows_zoom = false;
        self.asset_renderer.set_view(view);
    }

    /// Drive all three source LOD slots through the authentic zoom controls.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn enable_tactical_lod_journey(&mut self) {
        self.sync_proof_palette();
        self.proof_resource_2560 = true;
        self.proof_lod_follows_zoom = true;
        self.zoom = 2.0;
        self.asset_renderer
            .set_view(TacticalLodView::from_fixture_zoom(self.zoom));
    }

    /// Enable the source-traced camera for isolated browser acceptance.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn enable_original_camera_proof(&mut self, player_is_empire: bool) {
        self.sync_proof_palette();
        self.proof_resource_2560 = true;
        self.proof_lod_follows_zoom = false;
        let source_layout = self.session.as_ref().map_or_else(
            || OriginalTacticalLayout::from_active_counts(0, 0),
            |session| session.source_layout,
        );
        self.asset_renderer
            .enable_original_camera(player_is_empire, source_layout);
    }

    #[cfg(feature = "interface-test-fixtures")]
    fn sync_proof_palette(&mut self) {
        let selector = self
            .session
            .as_ref()
            .map_or(1, |session| session.system_picture_id);
        self.asset_renderer.set_palette_selector(selector);
    }

    /// Preserve the historical proof and negative-control fixtures without
    /// submitting live participant families alongside the isolated object.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn disable_original_participant_rendering(&mut self) {
        self.render_original_participants = false;
        self.render_original_planet = false;
        self.asset_renderer.disable_original_camera();
    }

    /// Remove mapped participant fallbacks in the browser-only framebuffer control.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn suppress_mapped_capital_fallback(&mut self) {
        self.suppress_mapped_capital_fallback = true;
        self.suppress_mapped_fighter_fallback = true;
        // Keep the common system backdrop present so the paired framebuffer
        // remains a participant-only negative control.
        self.render_original_planet = true;
    }

    /// Center the source camera on the player's first production fighter group
    /// for deterministic close/far browser evidence. This never enters a
    /// production build; normal play must reach the same state through the
    /// original fighter-group controls.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn focus_player_fighter_for_fixture(&mut self) {
        let target = self.session.as_ref().and_then(|session| {
            session
                .fighters
                .iter()
                .enumerate()
                .find(|(_, fighter)| {
                    fighter.alive && fighter.is_attacker == session.player_is_attacker
                })
                .map(|(index, fighter)| {
                    (
                        u32::try_from(index)
                            .unwrap_or(u32::MAX)
                            .saturating_add(1001),
                        fighter.source_position.rendered(),
                    )
                })
        });
        if let Some((object_id, position)) = target {
            self.asset_renderer.focus_target(object_id, position);
        }
    }

    /// Populate every source group slot for deterministic browser coverage.
    /// Production battles retain the executable's proven initial states.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_group_presentation_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let mut task_force = 0_u8;
        for ship in &mut session.ships {
            if ship.alive && ship.is_attacker == session.player_is_attacker {
                ship.task_force = task_force.min(7);
                task_force = task_force.saturating_add(1);
            }
        }
        let mut fighter_group = 0_u8;
        for fighter in &mut session.fighters {
            if fighter.alive
                && fighter.squad_count > 0
                && fighter.is_attacker == session.player_is_attacker
            {
                fighter.fighter_group = fighter_group.min(3);
                fighter_group = fighter_group.saturating_add(1);
            }
        }
    }

    /// Put three live player capitals in one task force so the recovered
    /// previous/next controls can prove their exact cyclic selection contract.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_selected_navigation_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let mut selected = None;
        for (index, ship) in session.ships.iter_mut().enumerate() {
            ship.selected = false;
            if ship.alive && ship.is_attacker == session.player_is_attacker {
                ship.task_force = 0;
                selected.get_or_insert(index);
            }
        }
        for fighter in &mut session.fighters {
            fighter.selected = false;
        }
        if let Some(index) = selected {
            session.ships[index].selected = true;
            session.selected_ship = Some(index);
            session.selected_fighter_group = None;
        }
        session.paused = true;
        self.command_panel = TacticalCommandPanel::Display;
    }

    /// Select one player capital and make the first player fighter group
    /// addressable through the authentic command panels. Production play
    /// reaches the same state through the task-force and fighter controls.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_command_assignment_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let Some(ship_index) = session
            .ships
            .iter()
            .position(|ship| ship.alive && ship.is_attacker == session.player_is_attacker)
        else {
            return;
        };
        for ship in &mut session.ships {
            ship.selected = false;
        }
        for fighter in &mut session.fighters {
            fighter.selected = false;
            if fighter.alive && fighter.is_attacker == session.player_is_attacker {
                fighter.fighter_group = 0;
            }
        }
        session.ships[ship_index].selected = true;
        session.selected_ship = Some(ship_index);
        session.selected_fighter_group = None;
        session.paused = true;
        self.command_panel = TacticalCommandPanel::Display;
    }

    /// Keep one player fighter group addressable while preserving the
    /// production Death Star manager object for deterministic browser proof.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_death_star_presentation_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        for fighter in &mut session.fighters {
            fighter.selected = false;
            if fighter.alive && fighter.is_attacker == session.player_is_attacker {
                fighter.fighter_group = 0;
            }
        }
        session.selected_fighter_group = None;
        session.paused = true;
        self.command_panel = TacticalCommandPanel::Display;
    }

    /// Fully charge the production Death Star manager for the deterministic
    /// superlaser control and right-click target journey.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_death_star_laser_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        if let Some(death_star) = session.death_star.as_mut() {
            death_star.laser_charge = ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE;
            death_star.action_committed = false;
        }
        if player_death_star(session).is_some() {
            if let Some(target) = session
                .ships
                .iter_mut()
                .find(|ship| ship.alive && ship.is_attacker != session.player_is_attacker)
            {
                target.source_position.x = 0.0;
                target.source_position.y = 40.0;
                target.source_position.z = session.source_layout.outer_positive_z;
            }
        }
        session.death_star_beam = None;
        session.paused = true;
        self.death_star_laser_pressed = false;
        self.death_star_targeting = false;
    }

    /// Stage an already-resolved source result solely so the browser harness
    /// can prove the app-level 201/202 routing and return path. Production
    /// combat reaches this edge through live fighter success or loss.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_trench_run_outcome_fixture(&mut self, outcome: TacticalTrenchRunOutcome) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        session.record_trench_run_outcome(outcome);
    }

    /// Populate each post-battle condition column and enter the production
    /// Battle Results phase without running an entire deterministic battle.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_battle_results_presentation_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        for alliance in [true, false] {
            let mut ships = session
                .ships
                .iter_mut()
                .filter(|ship| ship.identity.is_alliance == alliance);
            if let Some(ship) = ships.next() {
                ship.alive = true;
                ship.hull_current = ship.hull_max;
                ship.subsystem_damage = TacticalSubsystemDamage::default();
                ship.refresh_subsystem_condition();
            }
            if let Some(ship) = ships.next() {
                ship.alive = true;
                ship.hull_current = (ship.hull_max / 2).max(1);
                ship.subsystem_damage.engines = 1;
                ship.refresh_subsystem_condition();
            }
            if let Some(ship) = ships.next() {
                ship.alive = false;
                ship.hull_current = 0;
                ship.shield = 0;
                ship.refresh_subsystem_condition();
            }

            let mut fighters = session
                .fighters
                .iter_mut()
                .filter(|fighter| fighter.identity.is_alliance == alliance);
            if let Some(fighter) = fighters.next() {
                fighter.alive = true;
                fighter.hull_current = fighter.hull_max;
                fighter.squad_count = 1;
            }
            if let Some(fighter) = fighters.next() {
                fighter.alive = true;
                fighter.hull_current = (fighter.hull_max * 0.5).max(1.0);
                fighter.squad_count = 1;
            }
            if let Some(fighter) = fighters.next() {
                fighter.alive = false;
                fighter.hull_current = 0.0;
                fighter.squad_count = 0;
            }
        }
        session.phase = BattlePhase::Results;
        session.winner = Some(if session.player_is_attacker {
            CombatWinner::Attacker
        } else {
            CombatWinner::Defender
        });
        session.paused = true;
        session.selected_ship = None;
        session.selected_fighter_group = None;
        self.battle_result_tab = BattleResultTab::Summary;
        self.battle_result_category = BattleResultCategory::CapitalShips;
        self.command_panel = TacticalCommandPanel::Display;
    }

    /// Commit one recovered capital maneuver and one fighter recovery order
    /// through the production assignment and executor path.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_command_execution_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let Some(ship_index) = session.ships.iter().position(|ship| {
            ship.alive
                && ship.is_attacker == session.player_is_attacker
                && ship.fighter_capacity > 0
        }) else {
            return;
        };
        for ship in &mut session.ships {
            ship.selected = false;
        }
        for fighter in &mut session.fighters {
            fighter.selected = false;
            if fighter.alive && fighter.is_attacker == session.player_is_attacker {
                fighter.fighter_group = 0;
            }
        }
        let forward_z = if session.ships[ship_index].identity.is_alliance {
            1.0
        } else {
            -1.0
        };
        session.ships[ship_index].source_forward = TacticalWorldVector {
            x: 0.0,
            y: 0.0,
            z: forward_z,
        };
        session.ships[ship_index].selected = true;
        session.selected_ship = Some(ship_index);
        session.selected_fighter_group = None;
        assign_selected_command(
            session,
            TacticalOrder::LeftHook,
            Some(TacticalTactic::Surround),
        );

        if let Some(hold_index) = session.ships.iter().enumerate().find_map(|(index, ship)| {
            (index != ship_index && ship.alive && ship.is_attacker == session.player_is_attacker)
                .then_some(index)
        }) {
            session.ships[ship_index].selected = false;
            let hold_ship = &mut session.ships[hold_index];
            hold_ship.selected = true;
            hold_ship.source_desired_forward = hold_ship.source_forward;
            hold_ship.source_velocity = hold_ship.source_forward;
            hold_ship.source_waypoint = Some(hold_ship.source_position);
            session.selected_ship = Some(hold_index);
            assign_selected_command(session, TacticalOrder::HoldPosition, None);
        }

        for ship in &mut session.ships {
            ship.selected = false;
        }
        for fighter in &mut session.fighters {
            fighter.selected = fighter.alive
                && fighter.is_attacker == session.player_is_attacker
                && fighter.fighter_group == 0;
        }
        session.selected_ship = None;
        session.selected_fighter_group = Some(0);
        assign_selected_command(session, TacticalOrder::Recover, None);

        for fighter in &mut session.fighters {
            fighter.selected = false;
        }
        session.ships[ship_index].selected = true;
        session.selected_ship = Some(ship_index);
        session.selected_fighter_group = None;
        session.paused = true;
        self.command_panel = TacticalCommandPanel::Display;
    }

    /// Advance one turning capital, one arriving capital, and both final
    /// fighter-recovery transitions through the production executors.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_command_progression_fixture(&mut self) {
        let focus = {
            let Some(session) = self.session.as_mut() else {
                return;
            };
            let player_ships = session
                .ships
                .iter()
                .enumerate()
                .filter(|(_, ship)| {
                    ship.alive
                        && ship.is_attacker == session.player_is_attacker
                        && ship.subsystem_capacity.engines
                })
                .map(|(index, _)| index)
                .take(2)
                .collect::<Vec<_>>();
            let player_fighters = session
                .fighters
                .iter()
                .enumerate()
                .filter(|(_, fighter)| {
                    fighter.alive && fighter.is_attacker == session.player_is_attacker
                })
                .map(|(index, _)| index)
                .take(2)
                .collect::<Vec<_>>();
            if player_ships.len() < 2 || player_fighters.len() < 2 {
                return;
            }
            let turning = player_ships[0];
            let arriving = player_ships[1];
            let forward_z = if session.ships[turning].identity.is_alliance {
                1.0
            } else {
                -1.0
            };
            for ship in &mut session.ships {
                ship.selected = false;
                ship.source_velocity = TacticalWorldVector::ZERO;
            }
            for fighter in &mut session.fighters {
                fighter.selected = false;
            }

            let turning_position = session.ships[turning].source_position;
            session.ships[turning].selected = true;
            session.ships[turning].maneuverability = 4;
            session.ships[turning].sublight_engine_power = 20.0;
            session.ships[turning].source_forward = TacticalWorldVector {
                x: 0.0,
                y: 0.0,
                z: forward_z,
            };
            session.ships[turning].source_waypoint = Some(TacticalWorldPosition {
                x: turning_position.x + 50.0,
                y: turning_position.y,
                z: turning_position.z + 50.0 * forward_z,
            });
            session.ships[turning].order = TacticalOrder::LeftHook;

            let arriving_position = session.ships[arriving].source_position;
            session.ships[arriving].sublight_engine_power = 100.0;
            session.ships[arriving].source_forward = TacticalWorldVector {
                x: 0.0,
                y: 0.0,
                z: forward_z,
            };
            session.ships[arriving].source_desired_forward = session.ships[arriving].source_forward;
            session.ships[arriving].source_waypoint = Some(TacticalWorldPosition {
                x: arriving_position.x,
                y: arriving_position.y,
                z: arriving_position.z + forward_z,
            });
            session.ships[arriving].order = TacticalOrder::Anvil;

            session.advance_original_tactical_movement(ORIGINAL_TACTICAL_STEP_MILLISECONDS);

            let carrier_position = session.ships[arriving].source_position;
            let returning = player_fighters[0];
            session.fighters[returning].source_position = TacticalWorldPosition {
                x: carrier_position.x,
                y: carrier_position.y,
                z: carrier_position.z + 2.5 * forward_z,
            };
            session.fighters[returning].sublight_engine_power = 4.0;
            session.fighters[returning].order = TacticalOrder::Recover;
            session.fighters[returning].recovery_state = TacticalFighterRecoveryState::Returning;
            session.fighters[returning].recovery_target = Some(arriving);

            let docking = player_fighters[1];
            session.fighters[docking].source_position = carrier_position;
            session.fighters[docking].order = TacticalOrder::Recover;
            session.fighters[docking].recovery_state = TacticalFighterRecoveryState::Docking;
            session.fighters[docking].recovery_target = Some(arriving);
            session.advance_original_fighter_recovery(ORIGINAL_TACTICAL_STEP_MILLISECONDS);

            session.selected_ship = Some(turning);
            session.selected_fighter_group = None;
            session.paused = true;
            self.command_panel = TacticalCommandPanel::Display;
            let object_id = u32::try_from(turning).unwrap_or(u32::MAX).saturating_add(1);
            (object_id, session.ships[turning].source_position.rendered())
        };
        self.asset_renderer.focus_target(focus.0, focus.1);
    }

    /// Exercise both recovered automatic attack-target executors with capital
    /// and fighter owners. Production play reaches these states by committing
    /// the authentic Missions-panel controls.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_attack_targeting_fixture(&mut self) {
        let focus = {
            let Some(session) = self.session.as_mut() else {
                return;
            };
            let player_ships = session
                .ships
                .iter()
                .enumerate()
                .filter(|(_, ship)| ship.alive && ship.is_attacker == session.player_is_attacker)
                .map(|(index, _)| index)
                .take(2)
                .collect::<Vec<_>>();
            let player_fighters = session
                .fighters
                .iter()
                .enumerate()
                .filter(|(_, fighter)| {
                    fighter.alive && fighter.is_attacker == session.player_is_attacker
                })
                .map(|(index, _)| index)
                .take(2)
                .collect::<Vec<_>>();
            if player_ships.len() < 2 || player_fighters.len() < 2 {
                return;
            }
            for ship in &mut session.ships {
                ship.selected = false;
                ship.attack_target = None;
            }
            for fighter in &mut session.fighters {
                fighter.selected = false;
                fighter.attack_target = None;
            }
            session.fighters[player_fighters[0]].fighter_group = 0;
            session.fighters[player_fighters[1]].fighter_group = 1;

            session.ships[player_ships[0]].selected = true;
            session.selected_ship = Some(player_ships[0]);
            session.selected_fighter_group = None;
            assign_selected_command(session, TacticalOrder::AttackCapitalShips, None);

            session.ships[player_ships[0]].selected = false;
            session.ships[player_ships[1]].selected = true;
            session.selected_ship = Some(player_ships[1]);
            assign_selected_command(session, TacticalOrder::AttackFighters, None);

            session.ships[player_ships[1]].selected = false;
            session.fighters[player_fighters[0]].selected = true;
            session.selected_ship = None;
            session.selected_fighter_group = Some(0);
            assign_selected_command(session, TacticalOrder::AttackCapitalShips, None);

            session.fighters[player_fighters[0]].selected = false;
            session.fighters[player_fighters[1]].selected = true;
            session.selected_fighter_group = Some(1);
            assign_selected_command(session, TacticalOrder::AttackFighters, None);

            session.fighters[player_fighters[1]].selected = false;
            session.ships[player_ships[1]].selected = true;
            session.selected_ship = Some(player_ships[1]);
            session.selected_fighter_group = None;
            session.paused = true;
            self.command_panel = TacticalCommandPanel::Display;

            let selected = player_ships[1];
            let object_id = u32::try_from(selected)
                .unwrap_or(u32::MAX)
                .saturating_add(1);
            (
                object_id,
                session.ships[selected].source_position.rendered(),
            )
        };
        self.asset_renderer.focus_target(focus.0, focus.1);
    }

    /// Invalidate each first-acquired attack target and exercise the recovered
    /// same-class replacement lifecycle. The retired objects remain in the
    /// fixture record so the browser harness can prove why they were skipped.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_attack_target_lifecycle_fixture(&mut self) {
        self.configure_attack_targeting_fixture();
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let first_enemy_ship = session
            .ships
            .iter()
            .position(|ship| ship.alive && ship.is_attacker != session.player_is_attacker);
        let first_enemy_fighter = session
            .fighters
            .iter()
            .position(|fighter| fighter.alive && fighter.is_attacker != session.player_is_attacker);
        let (Some(first_enemy_ship), Some(first_enemy_fighter)) =
            (first_enemy_ship, first_enemy_fighter)
        else {
            return;
        };

        session.ships[first_enemy_ship].retreating = true;
        session.fighters[first_enemy_fighter].recovery_state =
            TacticalFighterRecoveryState::Recovered;
        refresh_original_attack_targets(session);
    }

    /// Display one stable frame from every source-selected impact family.
    /// This bridge is compiled only into interface-fixture builds.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_effect_presentation_fixture(&mut self) {
        let focus = {
            let Some(session) = self.session.as_mut() else {
                return;
            };
            let targets = session
                .ships
                .iter()
                .enumerate()
                .filter(|(_, ship)| ship.is_attacker == session.player_is_attacker)
                .map(|(index, _)| index)
                .take(6)
                .collect::<Vec<_>>();
            let fixtures = [
                (WeaponKind::LaserCannon, OriginalTacticalImpactStage::Hit, 2),
                (WeaponKind::IonCannon, OriginalTacticalImpactStage::Hit, 2),
                (
                    WeaponKind::LaserCannon,
                    OriginalTacticalImpactStage::Damage,
                    3,
                ),
                (
                    WeaponKind::IonCannon,
                    OriginalTacticalImpactStage::Damage,
                    7,
                ),
                (
                    WeaponKind::Turbolaser,
                    OriginalTacticalImpactStage::Damage,
                    2,
                ),
                (
                    WeaponKind::LaserCannon,
                    OriginalTacticalImpactStage::Destroyed,
                    8,
                ),
            ];
            session.impact_effects.clear();
            for (target, (weapon, stage, frame)) in targets.iter().copied().zip(fixtures) {
                queue_original_tactical_impact(&mut session.impact_effects, target, weapon, stage);
                if let Some(effect) = session
                    .impact_effects
                    .iter_mut()
                    .find(|effect| effect.target == target)
                {
                    effect.frame = frame.min(effect.sequence.frame_count().saturating_sub(1));
                }
            }
            session.paused = true;
            targets.get(2).and_then(|&target| {
                session.ships.get(target).map(|ship| {
                    (
                        u32::try_from(target).unwrap_or(u32::MAX).saturating_add(1),
                        ship.source_position.rendered(),
                    )
                })
            })
        };
        if let Some((object_id, position)) = focus {
            self.asset_renderer.focus_target(object_id, position);
        }
    }

    /// Freeze all three projectile shapes plus both shared field families on
    /// live production participants for deterministic browser inspection.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_projectile_field_fixture(&mut self) {
        {
            let Some(session) = self.session.as_mut() else {
                return;
            };
            let alliance = session
                .ships
                .iter()
                .enumerate()
                .filter(|(_, ship)| ship.identity.is_alliance)
                .map(|(index, _)| index)
                .collect::<Vec<_>>();
            let empire = session
                .ships
                .iter()
                .enumerate()
                .filter(|(_, ship)| !ship.identity.is_alliance)
                .map(|(index, _)| index)
                .collect::<Vec<_>>();
            if alliance.len() < 3 || empire.len() < 3 {
                return;
            }
            session.weapon_effects.clear();
            let projectile_fixtures = [
                (
                    alliance[0],
                    empire[0],
                    WeaponKind::LaserCannon,
                    ORIGINAL_LASER_PROJECTILE_THRESHOLD,
                ),
                (
                    empire[1],
                    alliance[1],
                    WeaponKind::Turbolaser,
                    ORIGINAL_TURBOLASER_PROJECTILE_THRESHOLD - 1.0,
                ),
                (
                    alliance[2],
                    empire[2],
                    WeaponKind::IonCannon,
                    ORIGINAL_ION_PROJECTILE_THRESHOLD,
                ),
            ];
            for (source, target, weapon, strength) in projectile_fixtures {
                queue_original_tactical_projectile(
                    &mut session.weapon_effects,
                    &session.ships,
                    source,
                    target,
                    weapon,
                    strength,
                );
            }
            for effect in &mut session.weapon_effects {
                effect.elapsed = 0.5;
            }

            session.field_effects.clear();
            session.set_tractor_field(empire[0], alliance[1], true);
            session.set_tractor_field(alliance[0], empire[1], true);
            session.set_gravity_field(alliance[2], empire[1], true);
            session.field_effects[0].frame = 3;
            session.field_effects[1].frame = 5;
            session.paused = true;
        }
        // Widen the source camera field without moving any participant. This
        // keeps both opposing field targets and the projectile paths inside
        // the 444x439 aperture for one deterministic inspection frame.
        for _ in 0..4 {
            self.asset_renderer.zoom_out();
        }
    }

    /// Select one production capital ship and install stable live damage for
    /// the selected-HUD browser gate. This bridge never enters production.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_selected_damage_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let Some(index) = session
            .ships
            .iter()
            .position(|ship| ship.alive && ship.is_attacker == session.player_is_attacker)
        else {
            return;
        };
        for ship in &mut session.ships {
            ship.selected = false;
        }
        let ship = &mut session.ships[index];
        ship.selected = true;
        ship.hull_current = (ship.hull_max * 2 / 5).max(1);
        ship.shield = (ship.shield_max / 4).max(0);
        session.selected_ship = Some(index);
        session.selected_fighter_group = None;
        session.paused = true;
    }

    /// Install all five selected-capital condition bands plus source-identified
    /// tractor and gravity locks for deterministic browser inspection.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_subsystem_field_command_fixture(&mut self) {
        self.configure_selected_damage_fixture();
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let alliance = session
            .ships
            .iter()
            .enumerate()
            .filter(|(_, ship)| ship.identity.is_alliance)
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        let empire = session
            .ships
            .iter()
            .enumerate()
            .filter(|(_, ship)| !ship.identity.is_alliance)
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        if alliance.len() < 3 || empire.len() < 3 {
            return;
        }
        session.field_effects.clear();
        assert!(session.set_tractor_field(empire[0], alliance[1], true));
        assert!(session.set_tractor_field(empire[2], alliance[1], true));
        assert!(session.set_tractor_field(alliance[0], empire[1], true));
        assert!(session.set_gravity_field(alliance[2], empire[1], true));
        // P58F5 is a resource-band presentation fixture. Field changes now
        // refresh production mobility, so install its deliberately chosen
        // five-band display state after those production mutations complete.
        if let Some(index) = session.selected_ship {
            session.ships[index].subsystem_condition = TacticalSubsystemCondition {
                shields: 0,
                weapons: 24,
                tractor: 25,
                engines: 50,
                hyperdrive: 75,
            };
        }
        session.advance_presentational_effects(0.4);
        session.paused = true;
        for _ in 0..4 {
            self.asset_renderer.zoom_out();
        }
    }

    /// Drive one selected production capital through the source combat-damage
    /// path with deterministic rolls for browser inspection. Direct setup is
    /// confined to the test bridge; every recorded hit uses production logic.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_live_subsystem_damage_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let Some(index) = session.ships.iter().position(|ship| {
            ship.alive
                && ship.is_attacker == session.player_is_attacker
                && ship
                    .subsystem_capacity
                    .hit_limits()
                    .iter()
                    .all(|limit| *limit > 0)
        }) else {
            return;
        };
        for ship in &mut session.ships {
            ship.selected = false;
        }
        let ship = &mut session.ships[index];
        ship.selected = true;
        ship.hull_current = ship.hull_max;
        ship.shield = ship.shield_max;
        ship.subsystem_damage = TacticalSubsystemDamage::default();
        ship.refresh_subsystem_condition();
        session.selected_ship = Some(index);
        session.selected_fighter_group = None;

        let shield_damage = (session.ships[index].shield_max / 5).max(1);
        apply_original_capital_damage(
            &mut session.ships,
            &mut session.field_effects,
            index,
            shield_damage,
            OriginalTacticalDamageRolls { shield: 0, hull: 0 },
        );
        session.ships[index].shield = 0;
        for (roll, hits) in [(66, 2), (81, 3), (91, 4), (96, 2)] {
            for _ in 0..hits {
                apply_original_capital_damage(
                    &mut session.ships,
                    &mut session.field_effects,
                    index,
                    1,
                    OriginalTacticalDamageRolls {
                        shield: 100,
                        hull: roll,
                    },
                );
            }
        }
        session.paused = true;
    }

    /// Present one source-selected repair while an enemy tractor source lowers
    /// the selected ship's effective sublight condition. Direct setup remains
    /// test-only; repair selection and mobility use production functions.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_subsystem_repair_mobility_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let Some(target) = session.ships.iter().position(|ship| {
            ship.alive
                && ship.is_attacker == session.player_is_attacker
                && ship
                    .subsystem_capacity
                    .hit_limits()
                    .iter()
                    .all(|limit| *limit > 0)
        }) else {
            return;
        };
        let Some(source) = session.ships.iter().enumerate().find_map(|(index, ship)| {
            (index != target && ship.alive && ship.is_attacker != session.player_is_attacker)
                .then_some(index)
        }) else {
            return;
        };

        for ship in &mut session.ships {
            ship.selected = false;
        }
        session.ships[target].selected = true;
        session.ships[target].hull_current = session.ships[target].hull_max;
        session.ships[target].sublight_engine_power = 100.0;
        session.ships[target].maneuver_state_value = Some(9.0);
        session.ships[target].damage_control = 100;
        session.ships[target].subsystem_damage = TacticalSubsystemDamage {
            shields: 1,
            weapons: 2,
            tractor: 1,
            engines: 2,
            hyperdrive: 1,
        };
        session.ships[source].hull_current = session.ships[source].hull_max;
        session.ships[source].subsystem_capacity.tractor = true;
        session.ships[source].tractor_beam_power = 25.0;
        session.ships[source].subsystem_damage.tractor = 0;
        session.field_effects.clear();
        session.subsystem_repairs.clear();
        session.selected_ship = Some(target);
        session.selected_fighter_group = None;
        let _ = session.set_tractor_field(source, target, true);

        if let Some(repair) =
            attempt_original_subsystem_repair(&mut session.ships[target], target, 1, 1)
        {
            session.subsystem_repairs.push(repair);
        }
        refresh_original_subsystem_conditions(&mut session.ships, &session.field_effects);
        let target_object_id = u32::try_from(target).unwrap_or(u32::MAX).saturating_add(1);
        let target_position = session.ships[target].source_position.rendered();
        let (camera_x, camera_y) =
            camera_offset_for_target(session.ships[target].x, session.ships[target].y);
        session.paused = true;
        (self.camera_x, self.camera_y) = (camera_x, camera_y);
        self.asset_renderer
            .focus_target(target_object_id, target_position);
    }

    /// Advance one selected production capital through the recovered
    /// maneuver producer, velocity contract, and 250 ms source integrator.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_maneuver_movement_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let Some(target) = session.ships.iter().position(|ship| {
            ship.alive
                && ship.is_attacker == session.player_is_attacker
                && ship.subsystem_capacity.engines
        }) else {
            return;
        };
        for ship in &mut session.ships {
            ship.selected = false;
            ship.source_velocity = TacticalWorldVector::ZERO;
        }
        session.ships[target].selected = true;
        session.ships[target].hull_current = session.ships[target].hull_max;
        session.ships[target].sublight_engine_power = 100.0;
        session.ships[target].subsystem_damage.engines = 1;
        session.ships[target].source_forward = if session.ships[target].identity.is_alliance {
            TacticalWorldVector::SOURCE_FORWARD
        } else {
            TacticalWorldVector {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            }
        };
        session.ships[target].source_desired_forward = session.ships[target].source_forward;
        session.field_effects.clear();
        session.selected_ship = Some(target);
        session.selected_fighter_group = None;
        assert!(session.set_maneuver_state_value(target, Some(4.0)));
        session.advance_original_tactical_movement(ORIGINAL_TACTICAL_STEP_MILLISECONDS);
        refresh_original_subsystem_conditions(&mut session.ships, &session.field_effects);

        let target_object_id = u32::try_from(target).unwrap_or(u32::MAX).saturating_add(1);
        let target_position = session.ships[target].source_position.rendered();
        let (camera_x, camera_y) =
            camera_offset_for_target(session.ships[target].x, session.ships[target].y);
        session.paused = true;
        (self.camera_x, self.camera_y) = (camera_x, camera_y);
        self.asset_renderer
            .focus_target(target_object_id, target_position);
    }

    /// End the current battle — clears session. Returns the session for
    /// result processing by the caller.
    pub fn end_battle(&mut self) -> Option<BattleSession> {
        self.dragging_ship = None;
        self.command_panel = TacticalCommandPanel::Display;
        self.session.take()
    }

    /// Consume the one-shot trench-run presentation edge, if the source
    /// tactical result dispatcher resolved one during the latest combat step.
    pub fn take_pending_trench_run_cinematic(&mut self) -> Option<TacticalTrenchRunOutcome> {
        self.session
            .as_mut()
            .and_then(BattleSession::take_pending_trench_run_cinematic)
    }

    /// Project a deterministic strategic auto-resolve outcome onto the live
    /// tactical roster, then enter the same original Battle Results surface
    /// used after a played battle.
    pub fn present_auto_resolve_result(&mut self, result: &SpaceCombatResult) -> bool {
        let Some(session) = self.session.as_mut() else {
            return false;
        };
        if session.attacker_fleet != result.attacker_fleet
            || session.defender_fleet != result.defender_fleet
            || session.system != result.system
        {
            return false;
        }

        for damage in &result.ship_damage {
            let is_attacker = damage.fleet == session.attacker_fleet;
            // Auto-resolve reports an index into its compressed alive-hull
            // snapshot, whereas played tactical results retain original fleet
            // roster slots. The live session was expanded from that same alive
            // sequence, so select its nth hull for presentation only.
            if let Some(ship) = session
                .ships
                .iter_mut()
                .filter(|ship| ship.is_attacker == is_attacker)
                .nth(damage.ship_index)
            {
                ship.hull_current = damage.hull_after.clamp(0, ship.hull_max);
                ship.alive = ship.hull_current > 0;
                if !ship.alive {
                    ship.shield = 0;
                }
                ship.refresh_subsystem_condition();
            }
        }
        for loss in &result.fighter_losses {
            let is_attacker = loss.fleet == session.attacker_fleet;
            for fighter in session.fighters.iter_mut().filter(|fighter| {
                fighter.is_attacker == is_attacker
                    && fighter.fleet_fighter_index == loss.fighter_index
            }) {
                let survives = fighter.fleet_squadron_index < loss.squads_after;
                fighter.alive = survives;
                if !survives {
                    fighter.hull_current = 0.0;
                    fighter.squad_count = 0;
                    fighter.shield = 0.0;
                }
            }
        }
        session.winner = Some(match result.winner {
            AutoResolveSide::Attacker => CombatWinner::Attacker,
            AutoResolveSide::Defender => CombatWinner::Defender,
            AutoResolveSide::Draw => CombatWinner::Draw,
        });
        session.phase = BattlePhase::Results;
        session.paused = true;
        session.selected_ship = None;
        session.selected_fighter_group = None;
        self.battle_result_tab = BattleResultTab::Summary;
        self.battle_result_category = BattleResultCategory::CapitalShips;
        self.strategic_results_applied = true;
        true
    }

    /// True when auto-resolve has already mutated the strategic world.
    #[must_use]
    pub const fn strategic_results_applied(&self) -> bool {
        self.strategic_results_applied
    }

    /// Returns true if a battle is currently active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.session.is_some()
    }
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

/// Result of drawing the tactical view — tells main.rs what action to take.
#[derive(Debug, Clone, PartialEq)]
pub enum TacticalAction {
    /// Player is still in the tactical view — no transition needed.
    None,
    /// Player clicked "Begin Battle" — advance from placement to combat phase.
    BeginCombat,
    /// Player clicked "Auto-Resolve" — skip tactical view, auto-resolve.
    AutoResolve,
    /// Battle is over or player clicked "Return to Galaxy" — transition back.
    ReturnToGalaxy,
    /// Leave the Battle Results window and open the battle system.
    OpenBattleSystem(SystemKey),
    /// Leave the Battle Results window and open the surviving victorious fleet.
    OpenBattleFleet(FleetKey),
    /// Toggle combat pause.
    TogglePause,
    /// Set combat speed multiplier.
    SetSpeed(u32),
    /// Retreat selected ships.
    RetreatSelected,
    /// Withdraw every eligible player hull after the Battle Options command.
    WithdrawFromBattle,
    /// Route to the original unified Game Options surface.
    OpenGameOptions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TacticalHudControl {
    TaskForce(u8),
    FighterGroup(u8),
    NavigationSet(u8),
    Pause,
    ZoomIn,
    ZoomOut,
    CameraTarget,
    CameraLeft,
    CameraRight,
    CameraUp,
    CameraDown,
    CameraRecall,
    CameraMemorize,
    HighlightAlliance,
    HighlightEmpire,
    BattleOptions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TacticalBattleOptionsControl {
    Withdraw,
    SimulateRemainder,
    ObserveToggle,
    GameOptions,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TacticalWithdrawConfirmationControl {
    Confirm,
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TacticalCommandControl {
    PreviousCapital,
    NextCapital,
    OpenManeuvers,
    OpenMissions,
    LeftHook,
    RightHook,
    Hammer,
    Anvil,
    HoldPosition,
    ToggleTactic,
    AttackCapitalShips,
    AttackFighters,
    Recover,
    AttackDeathStar,
    Confirm,
    Cancel,
}

#[derive(Debug, Clone, Copy)]
struct TacticalHudControlSpec {
    control: TacticalHudControl,
    rect: NativeRect,
    hit_resource: u32,
}

const TACTICAL_HUD_CONTROLS: [TacticalHudControlSpec; 17] = [
    TacticalHudControlSpec {
        control: TacticalHudControl::NavigationSet(0),
        rect: NativeRect::new(485.0, 272.0, 27.0, 27.0),
        hit_resource: resources::tactical::BTN_NAVIGATION_SET_NORMAL[0],
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::NavigationSet(1),
        rect: NativeRect::new(521.0, 272.0, 27.0, 27.0),
        hit_resource: resources::tactical::BTN_NAVIGATION_SET_NORMAL[1],
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::NavigationSet(2),
        rect: NativeRect::new(560.0, 272.0, 27.0, 27.0),
        hit_resource: resources::tactical::BTN_NAVIGATION_SET_NORMAL[2],
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::NavigationSet(3),
        rect: NativeRect::new(601.0, 272.0, 27.0, 27.0),
        hit_resource: resources::tactical::BTN_NAVIGATION_SET_NORMAL[3],
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::Pause,
        rect: NativeRect::new(560.0, 307.0, 28.0, 21.0),
        hit_resource: resources::tactical::BTN_PAUSE_RUNNING,
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::ZoomIn,
        rect: NativeRect::new(486.0, 343.0, 24.0, 24.0),
        hit_resource: resources::tactical::BTN_CAMERA_ZOOM_IN_NORMAL,
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::ZoomOut,
        rect: NativeRect::new(603.0, 343.0, 24.0, 24.0),
        hit_resource: resources::tactical::BTN_CAMERA_ZOOM_OUT_NORMAL,
    },
    // The target button is painted above the four overlapping D-pad arms and
    // therefore owns opaque pixels in their shared bounding region.
    TacticalHudControlSpec {
        control: TacticalHudControl::CameraTarget,
        rect: NativeRect::new(538.0, 379.0, 23.0, 23.0),
        hit_resource: resources::tactical::BTN_CAMERA_TARGET_NORMAL,
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::CameraLeft,
        rect: NativeRect::new(511.0, 376.0, 42.0, 43.0),
        hit_resource: resources::tactical::BTN_CAMERA_LEFT_NORMAL,
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::CameraRight,
        rect: NativeRect::new(557.0, 376.0, 43.0, 43.0),
        hit_resource: resources::tactical::BTN_CAMERA_RIGHT_NORMAL,
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::CameraUp,
        rect: NativeRect::new(537.0, 344.0, 43.0, 43.0),
        hit_resource: resources::tactical::BTN_CAMERA_UP_NORMAL,
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::CameraDown,
        rect: NativeRect::new(537.0, 412.0, 43.0, 43.0),
        hit_resource: resources::tactical::BTN_CAMERA_DOWN_NORMAL,
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::CameraRecall,
        rect: NativeRect::new(484.0, 430.0, 43.0, 25.0),
        hit_resource: resources::tactical::BTN_CAMERA_RECALL_NORMAL,
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::CameraMemorize,
        rect: NativeRect::new(585.0, 430.0, 43.0, 25.0),
        hit_resource: resources::tactical::BTN_CAMERA_MEMORIZE_NORMAL,
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::HighlightAlliance,
        rect: NativeRect::new(517.0, 304.0, 30.0, 26.0),
        hit_resource: resources::tactical::DIM_ALLIANCE_SHIPS,
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::HighlightEmpire,
        rect: NativeRect::new(482.0, 304.0, 30.0, 26.0),
        hit_resource: resources::tactical::DIM_EMPIRE_SHIPS,
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::BattleOptions,
        rect: NativeRect::new(606.0, 308.0, 20.0, 20.0),
        hit_resource: resources::tactical::BTN_BATTLE_OPTIONS_NORMAL,
    },
];

#[derive(Debug, Clone, Copy)]
struct TacticalBattleOptionsSpec {
    control: TacticalBattleOptionsControl,
    rect: NativeRect,
    normal_resource: u32,
    pressed_resource: u32,
}

const TACTICAL_BATTLE_OPTIONS_CONTROLS: [TacticalBattleOptionsSpec; 5] = [
    TacticalBattleOptionsSpec {
        control: TacticalBattleOptionsControl::Withdraw,
        rect: NativeRect::new(497.0, 47.0, 48.0, 34.0),
        normal_resource: resources::tactical::BTN_WITHDRAW_FROM_BATTLE_NORMAL,
        pressed_resource: resources::tactical::BTN_WITHDRAW_FROM_BATTLE_PRESSED,
    },
    TacticalBattleOptionsSpec {
        control: TacticalBattleOptionsControl::SimulateRemainder,
        rect: NativeRect::new(497.0, 114.0, 44.0, 25.0),
        normal_resource: resources::tactical::BTN_SIMULATE_REMAINDER_NORMAL,
        pressed_resource: resources::tactical::BTN_SIMULATE_REMAINDER_PRESSED,
    },
    TacticalBattleOptionsSpec {
        control: TacticalBattleOptionsControl::ObserveToggle,
        rect: NativeRect::new(561.0, 114.0, 45.0, 25.0),
        normal_resource: resources::tactical::BTN_OBSERVE_TOGGLE_NORMAL,
        pressed_resource: resources::tactical::BTN_OBSERVE_TOGGLE_PRESSED,
    },
    TacticalBattleOptionsSpec {
        control: TacticalBattleOptionsControl::GameOptions,
        rect: NativeRect::new(494.0, 203.0, 52.0, 33.0),
        normal_resource: resources::tactical::BTN_GAME_OPTIONS_NORMAL,
        pressed_resource: resources::tactical::BTN_GAME_OPTIONS_PRESSED,
    },
    TacticalBattleOptionsSpec {
        control: TacticalBattleOptionsControl::Close,
        rect: NativeRect::new(565.0, 203.0, 29.0, 33.0),
        normal_resource: resources::tactical::BTN_BATTLE_OPTIONS_CLOSE_NORMAL,
        pressed_resource: resources::tactical::BTN_BATTLE_OPTIONS_CLOSE_PRESSED,
    },
];

#[derive(Debug, Clone, Copy)]
struct TacticalWithdrawConfirmationSpec {
    control: TacticalWithdrawConfirmationControl,
    rect: NativeRect,
    normal_resource: u32,
    pressed_resource: u32,
}

// REBEXE FUN_005f0f40 constructs these controls at panel-local (66, 202)
// and (106, 202), each 27x25, with TACTICAL resources 1113 through 1116.
const TACTICAL_WITHDRAW_CONFIRMATION_CONTROLS: [TacticalWithdrawConfirmationSpec; 2] = [
    TacticalWithdrawConfirmationSpec {
        control: TacticalWithdrawConfirmationControl::Confirm,
        rect: NativeRect::new(547.0, 229.0, 27.0, 25.0),
        normal_resource: resources::tactical::BTN_ASSIGN_CONFIRM_NORMAL,
        pressed_resource: resources::tactical::BTN_ASSIGN_CONFIRM_PRESSED,
    },
    TacticalWithdrawConfirmationSpec {
        control: TacticalWithdrawConfirmationControl::Cancel,
        rect: NativeRect::new(587.0, 229.0, 27.0, 25.0),
        normal_resource: resources::tactical::BTN_ASSIGN_CANCEL_NORMAL,
        pressed_resource: resources::tactical::BTN_ASSIGN_CANCEL_PRESSED,
    },
];

const DEATH_STAR_LASER_GAUGE_RECT: NativeRect = NativeRect::new(474.0, 0.0, 166.0, 25.0);
const DEATH_STAR_LASER_BUTTON_RECT: NativeRect = NativeRect::new(613.0, 2.0, 21.0, 20.0);
const DEATH_STAR_LASER_FILL_RECT: NativeRect = NativeRect::new(508.0, 9.0, 95.0, 6.0);

const TASK_FORCE_BUTTONS: [NativeRect; 8] = [
    NativeRect::new(60.0, 2.0, 25.0, 17.0),
    NativeRect::new(86.0, 2.0, 25.0, 17.0),
    NativeRect::new(112.0, 2.0, 25.0, 17.0),
    NativeRect::new(138.0, 2.0, 25.0, 17.0),
    NativeRect::new(164.0, 2.0, 25.0, 17.0),
    NativeRect::new(190.0, 2.0, 25.0, 17.0),
    NativeRect::new(216.0, 2.0, 25.0, 17.0),
    NativeRect::new(242.0, 2.0, 25.0, 17.0),
];

const FIGHTER_GROUP_BUTTONS: [NativeRect; 4] = [
    NativeRect::new(330.0, 2.0, 25.0, 17.0),
    NativeRect::new(356.0, 2.0, 25.0, 17.0),
    NativeRect::new(382.0, 2.0, 25.0, 17.0),
    NativeRect::new(408.0, 2.0, 25.0, 17.0),
];

const FIGHTER_GROUP_NORMAL_ART: [u32; 4] = [
    resources::tactical::BTN_RED_SQUADRON_NORMAL,
    resources::tactical::BTN_BLUE_SQUADRON_NORMAL,
    resources::tactical::BTN_GREEN_SQUADRON_NORMAL,
    resources::tactical::BTN_GOLD_SQUADRON_NORMAL,
];

const FIGHTER_GROUP_PRESSED_ART: [u32; 4] = [
    resources::tactical::BTN_RED_SQUADRON_PRESSED,
    resources::tactical::BTN_BLUE_SQUADRON_PRESSED,
    resources::tactical::BTN_GREEN_SQUADRON_PRESSED,
    resources::tactical::BTN_GOLD_SQUADRON_PRESSED,
];

const DISPLAY_COMMAND_CONTROLS: [TacticalCommandControl; 4] = [
    TacticalCommandControl::PreviousCapital,
    TacticalCommandControl::NextCapital,
    TacticalCommandControl::OpenMissions,
    TacticalCommandControl::OpenManeuvers,
];

const MANEUVER_COMMAND_CONTROLS: [TacticalCommandControl; 8] = [
    TacticalCommandControl::Hammer,
    TacticalCommandControl::Anvil,
    TacticalCommandControl::LeftHook,
    TacticalCommandControl::RightHook,
    TacticalCommandControl::HoldPosition,
    TacticalCommandControl::ToggleTactic,
    TacticalCommandControl::Confirm,
    TacticalCommandControl::Cancel,
];

const MISSION_COMMAND_CONTROLS: [TacticalCommandControl; 6] = [
    TacticalCommandControl::AttackCapitalShips,
    TacticalCommandControl::AttackFighters,
    TacticalCommandControl::Recover,
    TacticalCommandControl::AttackDeathStar,
    TacticalCommandControl::Confirm,
    TacticalCommandControl::Cancel,
];

fn tactical_command_spec(
    control: TacticalCommandControl,
    player_is_alliance: bool,
) -> (NativeRect, u32) {
    use resources::tactical as art;
    match control {
        TacticalCommandControl::PreviousCapital => (
            NativeRect::new(488.0, 37.0, 11.0, 53.0),
            art::BTN_PREVIOUS_CAPITAL_NORMAL,
        ),
        TacticalCommandControl::NextCapital => (
            NativeRect::new(613.0, 37.0, 11.0, 53.0),
            art::BTN_NEXT_CAPITAL_NORMAL,
        ),
        TacticalCommandControl::OpenMissions => (
            NativeRect::new(493.0, 237.0, 58.0, 22.0),
            art::BTN_MISSIONS_NORMAL,
        ),
        TacticalCommandControl::OpenManeuvers => (
            NativeRect::new(561.0, 237.0, 58.0, 22.0),
            art::BTN_MANEUVERS_TACTICS_NORMAL,
        ),
        TacticalCommandControl::LeftHook => (
            NativeRect::new(494.0, 186.0, 28.0, 28.0),
            art::BTN_LEFT_HOOK_NORMAL,
        ),
        TacticalCommandControl::RightHook => (
            NativeRect::new(524.0, 186.0, 28.0, 28.0),
            art::BTN_RIGHT_HOOK_NORMAL,
        ),
        TacticalCommandControl::Hammer => (
            NativeRect::new(494.0, 156.0, 28.0, 28.0),
            art::BTN_HAMMER_NORMAL,
        ),
        TacticalCommandControl::Anvil => (
            NativeRect::new(524.0, 156.0, 28.0, 28.0),
            art::BTN_ANVIL_NORMAL,
        ),
        TacticalCommandControl::HoldPosition => (
            NativeRect::new(556.0, 171.0, 28.0, 28.0),
            art::BTN_HOLD_POSITION_NORMAL,
        ),
        TacticalCommandControl::ToggleTactic => (
            NativeRect::new(592.0, 159.0, 24.0, 52.0),
            art::BTN_TACTIC_STAND_OFF,
        ),
        TacticalCommandControl::AttackCapitalShips => (
            NativeRect::new(499.0, 156.0, 46.0, 26.0),
            if player_is_alliance {
                art::BTN_ATTACK_CAPITAL_SHIPS_ALLIANCE_NORMAL
            } else {
                art::BTN_ATTACK_CAPITAL_SHIPS_EMPIRE_NORMAL
            },
        ),
        TacticalCommandControl::AttackFighters => (
            NativeRect::new(499.0, 186.0, 46.0, 26.0),
            if player_is_alliance {
                art::BTN_ATTACK_FIGHTERS_ALLIANCE_NORMAL
            } else {
                art::BTN_ATTACK_FIGHTERS_EMPIRE_NORMAL
            },
        ),
        TacticalCommandControl::Recover => (
            NativeRect::new(567.0, 156.0, 46.0, 26.0),
            if player_is_alliance {
                art::BTN_RECOVER_ALLIANCE_NORMAL
            } else {
                art::BTN_RECOVER_EMPIRE_NORMAL
            },
        ),
        TacticalCommandControl::AttackDeathStar => (
            NativeRect::new(567.0, 186.0, 46.0, 26.0),
            art::BTN_ATTACK_DEATH_STAR_NORMAL,
        ),
        TacticalCommandControl::Confirm => (
            NativeRect::new(547.0, 229.0, 27.0, 25.0),
            art::BTN_ASSIGN_CONFIRM_NORMAL,
        ),
        TacticalCommandControl::Cancel => (
            NativeRect::new(587.0, 229.0, 27.0, 25.0),
            art::BTN_ASSIGN_CANCEL_NORMAL,
        ),
    }
}

fn tactical_command_controls(panel: TacticalCommandPanel) -> &'static [TacticalCommandControl] {
    match panel {
        TacticalCommandPanel::Display => &DISPLAY_COMMAND_CONTROLS,
        TacticalCommandPanel::Maneuvers { .. } => &MANEUVER_COMMAND_CONTROLS,
        TacticalCommandPanel::Missions { .. } => &MISSION_COMMAND_CONTROLS,
    }
}

#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "A containing native bitmap rectangle guarantees finite nonnegative local pixels."
)]
#[expect(
    clippy::too_many_arguments,
    reason = "The source control predicate keeps panel, selection, faction, resource mask, and pointer coordinates explicit."
)]
fn tactical_command_control_at(
    cache: &mut BmpCache,
    panel: TacticalCommandPanel,
    has_selection: bool,
    selected_fighter: bool,
    attack_death_star_enabled: bool,
    player_is_alliance: bool,
    x: f32,
    y: f32,
) -> Option<TacticalCommandControl> {
    if !has_selection {
        return None;
    }
    tactical_command_controls(panel)
        .iter()
        .copied()
        .find(|control| {
            if (matches!(
                *control,
                TacticalCommandControl::PreviousCapital | TacticalCommandControl::NextCapital
            ) && selected_fighter)
                || (*control == TacticalCommandControl::Recover && !selected_fighter)
                || (*control == TacticalCommandControl::AttackDeathStar
                    && !attack_death_star_enabled)
            {
                return false;
            }
            let (rect, resource) = tactical_command_spec(*control, player_is_alliance);
            if !rect.contains(x, y) {
                return false;
            }
            cache.is_resource_hit(
                DllSource::Tactical,
                resource,
                (x - rect.x).floor() as usize,
                (y - rect.y).floor() as usize,
            )
        })
}

#[cfg(test)]
fn tactical_command_rect_control_at(
    panel: TacticalCommandPanel,
    has_selection: bool,
    selected_fighter: bool,
    attack_death_star_enabled: bool,
    x: f32,
    y: f32,
) -> Option<TacticalCommandControl> {
    if !has_selection {
        return None;
    }
    tactical_command_controls(panel)
        .iter()
        .copied()
        .find(|control| {
            (!matches!(
                *control,
                TacticalCommandControl::PreviousCapital | TacticalCommandControl::NextCapital
            ) || !selected_fighter)
                && (*control != TacticalCommandControl::Recover || selected_fighter)
                && (*control != TacticalCommandControl::AttackDeathStar
                    || attack_death_star_enabled)
                && tactical_command_spec(*control, true).0.contains(x, y)
        })
}

#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "A containing native bitmap rectangle guarantees finite nonnegative local pixels."
)]
fn tactical_hud_control_at(cache: &mut BmpCache, x: f32, y: f32) -> Option<TacticalHudControl> {
    for (index, rect) in TASK_FORCE_BUTTONS.iter().enumerate() {
        if rect.contains(x, y) {
            let pixel_x = (x - rect.x).floor() as usize;
            let pixel_y = (y - rect.y).floor() as usize;
            if cache.is_resource_hit(
                DllSource::Tactical,
                resources::tactical::BTN_TASK_FORCE_NORMAL,
                pixel_x,
                pixel_y,
            ) {
                return Some(TacticalHudControl::TaskForce(
                    u8::try_from(index).unwrap_or(7),
                ));
            }
        }
    }
    for (index, rect) in FIGHTER_GROUP_BUTTONS.iter().enumerate() {
        if rect.contains(x, y) {
            let pixel_x = (x - rect.x).floor() as usize;
            let pixel_y = (y - rect.y).floor() as usize;
            if cache.is_resource_hit(
                DllSource::Tactical,
                FIGHTER_GROUP_NORMAL_ART[index],
                pixel_x,
                pixel_y,
            ) {
                return Some(TacticalHudControl::FighterGroup(
                    u8::try_from(index).unwrap_or(3),
                ));
            }
        }
    }
    TACTICAL_HUD_CONTROLS.iter().find_map(|spec| {
        if !spec.rect.contains(x, y) {
            return None;
        }
        let pixel_x = (x - spec.rect.x).floor() as usize;
        let pixel_y = (y - spec.rect.y).floor() as usize;
        cache
            .is_resource_hit(DllSource::Tactical, spec.hit_resource, pixel_x, pixel_y)
            .then_some(spec.control)
    })
}

fn tactical_battle_options_control_at(
    cache: &mut BmpCache,
    withdraw_enabled: bool,
    x: f32,
    y: f32,
) -> Option<TacticalBattleOptionsControl> {
    TACTICAL_BATTLE_OPTIONS_CONTROLS.iter().find_map(|spec| {
        if spec.control == TacticalBattleOptionsControl::Withdraw && !withdraw_enabled {
            return None;
        }
        if !spec.rect.contains(x, y) {
            return None;
        }
        cache
            .is_resource_hit(
                DllSource::Tactical,
                spec.normal_resource,
                (x - spec.rect.x).floor() as usize,
                (y - spec.rect.y).floor() as usize,
            )
            .then_some(spec.control)
    })
}

fn tactical_withdraw_confirmation_control_at(
    cache: &mut BmpCache,
    x: f32,
    y: f32,
) -> Option<TacticalWithdrawConfirmationControl> {
    TACTICAL_WITHDRAW_CONFIRMATION_CONTROLS
        .iter()
        .find_map(|spec| {
            if !spec.rect.contains(x, y) {
                return None;
            }
            cache
                .is_resource_hit(
                    DllSource::Tactical,
                    spec.normal_resource,
                    (x - spec.rect.x).floor() as usize,
                    (y - spec.rect.y).floor() as usize,
                )
                .then_some(spec.control)
        })
}

fn player_death_star(session: &BattleSession) -> Option<TacticalDeathStar> {
    session
        .death_star
        .filter(|death_star| death_star.is_attacker == session.player_is_attacker)
}

fn tactical_death_star_laser_ready(session: &BattleSession) -> bool {
    player_death_star(session).is_some_and(|death_star| {
        death_star.operational()
            && death_star.laser_charge >= ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE
            && session.death_star_beam.is_none()
    })
}

#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "The containing original bitmap rectangle guarantees finite nonnegative pixels."
)]
fn tactical_death_star_laser_control_at(cache: &mut BmpCache, x: f32, y: f32) -> bool {
    DEATH_STAR_LASER_BUTTON_RECT.contains(x, y)
        && cache.is_resource_hit(
            DllSource::Tactical,
            resources::tactical::DEATH_STAR_LASER_READY,
            (x - DEATH_STAR_LASER_BUTTON_RECT.x).floor() as usize,
            (y - DEATH_STAR_LASER_BUTTON_RECT.y).floor() as usize,
        )
}

fn tactical_withdraw_enabled(session: &BattleSession) -> bool {
    session.ships.iter().any(|ship| {
        ship.alive
            && !ship.retreating
            && ship.is_attacker == session.player_is_attacker
            && ship.subsystem_capacity.hyperdrive > 0
            && ship.subsystem_condition.hyperdrive > 0
    })
}

#[derive(Debug, Clone, Copy)]
struct TacticalGroupHud<'a> {
    task_forces: [bool; 8],
    fighter_groups: [bool; 4],
    selected_task_force: Option<u8>,
    selected_fighter_group: Option<u8>,
    selected_ship_name: Option<&'a str>,
    selected_ship_hud: Option<u32>,
    selected_ship_hull: Option<(i32, i32)>,
    selected_ship_shield: Option<(i32, i32)>,
    selected_ship_subsystems: Option<TacticalSubsystemCondition>,
    selected_fighter_name: Option<&'a str>,
    selected_fighter_hud: Option<u32>,
    selected_order: TacticalOrder,
    selected_tactic: TacticalTactic,
}

impl<'a> TacticalGroupHud<'a> {
    fn from_session(session: &'a BattleSession) -> Self {
        let mut task_forces = [false; 8];
        let mut fighter_groups = [false; 4];
        for ship in &session.ships {
            if ship.alive && ship.is_attacker == session.player_is_attacker {
                task_forces[usize::from(ship.task_force.min(7))] = true;
            }
        }
        for fighter in &session.fighters {
            if fighter.alive
                && fighter.squad_count > 0
                && fighter.is_attacker == session.player_is_attacker
                && fighter.fighter_group < 4
            {
                fighter_groups[usize::from(fighter.fighter_group)] = true;
            }
        }
        let selected_task_force = selected_player_task_force(session);
        let selected_ship_name = session
            .ships
            .iter()
            .find(|ship| {
                ship.alive && ship.selected && ship.is_attacker == session.player_is_attacker
            })
            .map(|ship| ship.name.as_str());
        let selected_ship = session.ships.iter().find(|ship| {
            ship.alive && ship.selected && ship.is_attacker == session.player_is_attacker
        });
        let selected_fighter = session.selected_fighter_group.and_then(|group| {
            session.fighters.iter().find(|fighter| {
                fighter.alive
                    && fighter.selected
                    && fighter.is_attacker == session.player_is_attacker
                    && fighter.fighter_group == group
            })
        });
        Self {
            task_forces,
            fighter_groups,
            selected_task_force,
            selected_fighter_group: session.selected_fighter_group,
            selected_ship_name,
            selected_ship_hud: selected_ship
                .and_then(|ship| ship.tactical_resource)
                .map(TacticalCapitalShipResource::hud_resource),
            selected_ship_hull: selected_ship.map(|ship| (ship.hull_current, ship.hull_max)),
            selected_ship_shield: selected_ship.map(|ship| (ship.shield, ship.shield_max)),
            selected_ship_subsystems: selected_ship.map(|ship| ship.subsystem_condition),
            selected_fighter_name: selected_fighter.map(|fighter| fighter.name.as_str()),
            selected_fighter_hud: selected_fighter
                .and_then(|fighter| fighter.tactical_resource)
                .map(TacticalFighterResource::hud_resource),
            selected_order: selected_fighter.map_or_else(
                || selected_ship.map_or(TacticalOrder::None, |ship| ship.order),
                |fighter| fighter.order,
            ),
            selected_tactic: selected_fighter.map_or_else(
                || selected_ship.map_or(TacticalTactic::StandOff, |ship| ship.tactic),
                |fighter| fighter.tactic,
            ),
        }
    }
}

fn tactical_meter_fraction(current: i32, maximum: i32) -> f32 {
    if maximum <= 0 {
        0.0
    } else {
        (current as f32 / maximum as f32).clamp(0.0, 1.0)
    }
}

fn draw_tactical_selected_meter(
    canvas: TacticalCanvas,
    rect: NativeRect,
    fraction: f32,
    color: Color,
) {
    let (x, y) = canvas.point(rect.x, rect.y);
    draw_rectangle(
        x,
        y,
        rect.width * canvas.scale * fraction.clamp(0.0, 1.0),
        rect.height * canvas.scale,
        color,
    );
}

fn selected_player_task_force(session: &BattleSession) -> Option<u8> {
    if session.selected_fighter_group.is_some() {
        return None;
    }
    let mut selected = session.ships.iter().filter(|ship| {
        ship.alive && ship.selected && ship.is_attacker == session.player_is_attacker
    });
    let group = selected.next()?.task_force;
    selected
        .all(|ship| ship.task_force == group)
        .then_some(group)
}

fn select_task_force(session: &mut BattleSession, group: u8) -> usize {
    let first = session.ships.iter().position(|ship| {
        ship.alive
            && !ship.retreating
            && ship.is_attacker == session.player_is_attacker
            && ship.task_force == group
    });
    let Some(first) = first else {
        return 0;
    };
    for ship in &mut session.ships {
        ship.selected = ship.alive
            && !ship.retreating
            && ship.is_attacker == session.player_is_attacker
            && ship.task_force == group;
    }
    for fighter in &mut session.fighters {
        fighter.selected = false;
    }
    session.selected_ship = Some(first);
    session.selected_fighter_group = None;
    session.ships.iter().filter(|ship| ship.selected).count()
}

fn assign_selected_to_task_force(session: &mut BattleSession, group: u8) -> usize {
    let mut assigned_groups = [false; 8];
    for ship in &session.ships {
        if ship.alive && ship.is_attacker == session.player_is_attacker {
            assigned_groups[usize::from(ship.task_force.min(7))] = true;
        }
    }
    let next_blank = assigned_groups
        .iter()
        .position(|assigned| !assigned)
        .and_then(|index| u8::try_from(index).ok());
    if !assigned_groups[usize::from(group.min(7))] && next_blank != Some(group.min(7)) {
        return 0;
    }
    let mut assigned = 0;
    for ship in &mut session.ships {
        if ship.alive && ship.selected && ship.is_attacker == session.player_is_attacker {
            ship.task_force = group.min(7);
            assigned += 1;
        }
    }
    if assigned > 0 {
        BattleSession::refresh_original_task_force_geometry(&mut session.ships);
    }
    assigned
}

fn select_fighter_group(session: &mut BattleSession, group: u8) -> bool {
    let assigned = session.fighters.iter().any(|fighter| {
        fighter.alive
            && fighter.squad_count > 0
            && fighter.is_attacker == session.player_is_attacker
            && fighter.fighter_group == group
    });
    if !assigned {
        return false;
    }
    for ship in &mut session.ships {
        ship.selected = false;
    }
    for fighter in &mut session.fighters {
        fighter.selected = fighter.alive
            && fighter.squad_count > 0
            && fighter.is_attacker == session.player_is_attacker
            && fighter.fighter_group == group;
    }
    session.selected_ship = None;
    session.selected_fighter_group = Some(group);
    true
}

fn cycle_selected_capital(session: &mut BattleSession, step: isize) -> Option<(usize, usize, u8)> {
    let current = session.selected_ship.or_else(|| {
        session.ships.iter().position(|ship| {
            ship.alive && ship.selected && ship.is_attacker == session.player_is_attacker
        })
    })?;
    let group = session.ships.get(current)?.task_force;
    let eligible = session
        .ships
        .iter()
        .enumerate()
        .filter(|(_, ship)| {
            ship.alive
                && !ship.retreating
                && ship.is_attacker == session.player_is_attacker
                && ship.task_force == group
        })
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    let current_position = eligible.iter().position(|index| *index == current)?;
    let count = isize::try_from(eligible.len()).ok()?;
    let next_position = (isize::try_from(current_position).ok()? + step).rem_euclid(count);
    let next = eligible[usize::try_from(next_position).ok()?];
    for (index, ship) in session.ships.iter_mut().enumerate() {
        ship.selected = index == next;
    }
    for fighter in &mut session.fighters {
        fighter.selected = false;
    }
    session.selected_ship = Some(next);
    session.selected_fighter_group = None;
    Some((current, next, group))
}

fn selected_command_values(session: &BattleSession) -> (TacticalOrder, TacticalTactic) {
    if let Some(group) = session.selected_fighter_group {
        if let Some(fighter) = session.fighters.iter().find(|fighter| {
            fighter.alive
                && fighter.selected
                && fighter.is_attacker == session.player_is_attacker
                && fighter.fighter_group == group
        }) {
            return (fighter.order, fighter.tactic);
        }
    }
    session
        .ships
        .iter()
        .find(|ship| ship.alive && ship.selected && ship.is_attacker == session.player_is_attacker)
        .map_or((TacticalOrder::None, TacticalTactic::StandOff), |ship| {
            (ship.order, ship.tactic)
        })
}

fn source_delta(to: TacticalWorldPosition, from: TacticalWorldPosition) -> TacticalWorldVector {
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

fn begin_original_capital_order(
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
fn begin_original_attack_order(
    session: &mut BattleSession,
    selected_capitals: &[usize],
    selected_fighters: &[usize],
    order: TacticalOrder,
) {
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
fn refresh_original_attack_targets(session: &mut BattleSession) {
    for index in 0..session.ships.len() {
        let source_is_attacker = session.ships[index].is_attacker;
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
    }

    for index in 0..session.fighters.len() {
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
    }
}

fn begin_original_fighter_recovery(session: &mut BattleSession, selected: &[usize]) {
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
        }
    }
}

fn assign_selected_command(
    session: &mut BattleSession,
    order: TacticalOrder,
    tactic: Option<TacticalTactic>,
) -> (usize, usize) {
    let mut capital_members = 0;
    let mut fighter_members = 0;
    let mut selected_capitals = Vec::new();
    let mut selected_fighters = Vec::new();
    if session.selected_fighter_group.is_some() {
        for (index, fighter) in session.fighters.iter_mut().enumerate() {
            if fighter.alive
                && fighter.selected
                && fighter.is_attacker == session.player_is_attacker
            {
                fighter.order = order;
                if let Some(tactic) = tactic {
                    fighter.tactic = tactic;
                }
                if order != TacticalOrder::Recover {
                    fighter.recovery_state = TacticalFighterRecoveryState::AwaitingCarrier;
                    fighter.recovery_target = None;
                }
                selected_fighters.push(index);
                fighter_members += 1;
            }
        }
    } else {
        for (index, ship) in session.ships.iter_mut().enumerate() {
            if ship.alive && ship.selected && ship.is_attacker == session.player_is_attacker {
                ship.order = order;
                if let Some(tactic) = tactic {
                    ship.tactic = tactic;
                }
                selected_capitals.push(index);
                capital_members += 1;
            }
        }
    }
    begin_original_capital_order(session, &selected_capitals, order);
    begin_original_attack_order(session, &selected_capitals, &selected_fighters, order);
    if order == TacticalOrder::Recover {
        begin_original_fighter_recovery(session, &selected_fighters);
    }
    (capital_members, fighter_members)
}

fn pressed_tactical_hud_control(
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
) -> Option<TacticalHudControl> {
    if !is_mouse_button_down(MouseButton::Left) {
        return None;
    }
    let (mouse_x, mouse_y) = mouse_position();
    let (x, y) = canvas.logical_pointer(mouse_x, mouse_y);
    tactical_hud_control_at(cache, x, y)
}

fn pressed_tactical_battle_options_control(
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
    withdraw_enabled: bool,
) -> Option<TacticalBattleOptionsControl> {
    if !is_mouse_button_down(MouseButton::Left) {
        return None;
    }
    let (mouse_x, mouse_y) = mouse_position();
    let (x, y) = canvas.logical_pointer(mouse_x, mouse_y);
    tactical_battle_options_control_at(cache, withdraw_enabled, x, y)
}

fn pressed_tactical_withdraw_confirmation_control(
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
) -> Option<TacticalWithdrawConfirmationControl> {
    if !is_mouse_button_down(MouseButton::Left) {
        return None;
    }
    let (mouse_x, mouse_y) = mouse_position();
    let (x, y) = canvas.logical_pointer(mouse_x, mouse_y);
    tactical_withdraw_confirmation_control_at(cache, x, y)
}

fn pressed_tactical_command_control(
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
    panel: TacticalCommandPanel,
    has_selection: bool,
    selected_fighter: bool,
    attack_death_star_enabled: bool,
    player_is_alliance: bool,
) -> Option<TacticalCommandControl> {
    if !is_mouse_button_down(MouseButton::Left) {
        return None;
    }
    let (mouse_x, mouse_y) = mouse_position();
    let (x, y) = canvas.logical_pointer(mouse_x, mouse_y);
    tactical_command_control_at(
        cache,
        panel,
        has_selection,
        selected_fighter,
        attack_death_star_enabled,
        player_is_alliance,
        x,
        y,
    )
}

#[cfg(test)]
fn tactical_hud_rect_control_at(x: f32, y: f32) -> Option<TacticalHudControl> {
    TACTICAL_HUD_CONTROLS
        .iter()
        .find_map(|spec| spec.rect.contains(x, y).then_some(spec.control))
}

fn draw_tactical_bitmap(cache: &mut BmpCache, id: u32, canvas: TacticalCanvas, x: f32, y: f32) {
    draw_original_bitmap(cache, DllSource::Tactical, id, canvas, x, y, None);
}

fn draw_strategy_bitmap(cache: &mut BmpCache, id: u32, canvas: TacticalCanvas, x: f32, y: f32) {
    draw_original_bitmap(cache, DllSource::Strategy, id, canvas, x, y, None);
}

fn draw_original_bitmap(
    cache: &mut BmpCache,
    source: DllSource,
    id: u32,
    canvas: TacticalCanvas,
    x: f32,
    y: f32,
    source_rect: Option<Rect>,
) {
    let Some(texture) = cache.get_macroquad_original(source, id) else {
        return;
    };
    let (screen_x, screen_y) = canvas.point(x, y);
    let source_width = source_rect.map_or_else(|| texture.width(), |rect| rect.w);
    let source_height = source_rect.map_or_else(|| texture.height(), |rect| rect.h);
    draw_texture_ex(
        texture,
        screen_x,
        screen_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(
                source_width * canvas.scale,
                source_height * canvas.scale,
            )),
            source: source_rect,
            ..Default::default()
        },
    );
}

/// Cache every glyph and size that the macroquad tactical pass can emit before
/// any texture-backed draw command is queued. Macroquad 0.4.14 replaces its
/// shared font-atlas texture when an uncached glyph expands the atlas, which
/// invalidates earlier commands in the same WebGL frame.
fn prewarm_tactical_font_atlas(
    session: &BattleSession,
    canvas: TacticalCanvas,
    warmed_sizes: &mut HashSet<u16>,
) {
    // The projected unit labels clamp to this complete integer range as the
    // camera zoom changes. Prewarm every value up front: adding only the
    // current zoom's sizes can repack Macroquad's shared glyph atlas during a
    // later zoom and perturb otherwise-static HUD text at fractional scales.
    let mut sizes = vec![7_u16, 8, 9, 10, 11, 12, 13, 14];
    sizes.extend([
        (10.0 * canvas.scale) as u16,
        (11.0 * canvas.scale) as u16,
        (12.0 * canvas.scale) as u16,
        (13.0 * canvas.scale) as u16,
        (14.0 * canvas.scale) as u16,
        (16.0 * canvas.scale) as u16,
        (18.0 * canvas.scale) as u16,
    ]);
    sizes.sort_unstable();
    if sizes.iter().all(|size| warmed_sizes.contains(size)) {
        return;
    }

    let mut characters: std::collections::BTreeSet<_> =
        Font::latin_character_list().into_iter().collect();
    for text in [
        "0123456789x#",
        "Task Force No Orders RETREAT Battle Paused.",
        "Red Group Blue Group Green Group Gold Group",
        "Battle at Alliance Imperial Forces Capital Ships Fighters Troops Personnel",
        "Operational Damaged Destroyed Go Directly To System Fleet victorious withdrawn",
        "Withdraw Confirmation Are you sure you wish to order a fleet withdraw?",
    ] {
        characters.extend(text.chars());
    }
    characters.extend(session.system_name.chars());
    for ship in &session.ships {
        characters.extend(ship.name.chars());
    }
    for fighter in &session.fighters {
        characters.extend(fighter.name.chars());
    }
    let sample: String = characters.into_iter().collect();

    let mut previous = None;
    for size in sizes {
        if size > 0 && previous != Some(size) && warmed_sizes.insert(size) {
            measure_text(&sample, None, size, 1.0);
            previous = Some(size);
        }
    }
}

#[expect(
    clippy::cast_possible_truncation,
    reason = "The 640x480 tactical source canvas uses integer device-pixel clipping."
)]
fn set_tactical_aperture_clip(aperture: Option<NativeRect>) {
    let clip = aperture.map(|rect| {
        (
            rect.x.round() as i32,
            rect.y.round() as i32,
            rect.width.round() as i32,
            rect.height.round() as i32,
        )
    });
    // SAFETY: macroquad exposes its immediate drawing state through this API.
    // The clip is always reset before original HUD controls are painted.
    unsafe {
        get_internal_gl().quad_gl.scissor(clip);
    }
}

fn draw_tactical_assignment_button(
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
    control: TacticalCommandControl,
    resource: u32,
) {
    let rect = tactical_command_spec(control, true).0;
    draw_tactical_bitmap(cache, resource, canvas, rect.x, rect.y);
}

fn draw_tactical_assignment_panel(
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
    player_is_alliance: bool,
    panel: TacticalCommandPanel,
    selected_fighter: bool,
    attack_death_star_enabled: bool,
    pressed: Option<TacticalCommandControl>,
) {
    use resources::tactical as art;
    match panel {
        TacticalCommandPanel::Display => {}
        TacticalCommandPanel::Maneuvers {
            pending_order,
            pending_tactic,
        } => {
            draw_tactical_bitmap(cache, art::MANEUVERS_PANEL, canvas, 481.0, 27.0);
            if let Some(graphic) = pending_order.maneuver_graphic() {
                draw_tactical_bitmap(cache, graphic, canvas, 495.0, 41.0);
            }
            for (control, normal, selected) in [
                (
                    TacticalCommandControl::LeftHook,
                    art::BTN_LEFT_HOOK_NORMAL,
                    pending_order == TacticalOrder::LeftHook,
                ),
                (
                    TacticalCommandControl::RightHook,
                    art::BTN_RIGHT_HOOK_NORMAL,
                    pending_order == TacticalOrder::RightHook,
                ),
                (
                    TacticalCommandControl::Hammer,
                    art::BTN_HAMMER_NORMAL,
                    pending_order == TacticalOrder::Hammer,
                ),
                (
                    TacticalCommandControl::Anvil,
                    art::BTN_ANVIL_NORMAL,
                    pending_order == TacticalOrder::Anvil,
                ),
                (
                    TacticalCommandControl::HoldPosition,
                    art::BTN_HOLD_POSITION_NORMAL,
                    pending_order == TacticalOrder::HoldPosition,
                ),
            ] {
                let active = selected || pressed == Some(control);
                draw_tactical_assignment_button(
                    cache,
                    canvas,
                    control,
                    if active { normal + 1 } else { normal },
                );
            }
            draw_tactical_assignment_button(
                cache,
                canvas,
                TacticalCommandControl::ToggleTactic,
                if pending_tactic == TacticalTactic::Surround {
                    art::BTN_TACTIC_SURROUND
                } else {
                    art::BTN_TACTIC_STAND_OFF
                },
            );
            for (control, normal, pressed_art) in [
                (
                    TacticalCommandControl::Confirm,
                    art::BTN_ASSIGN_CONFIRM_NORMAL,
                    art::BTN_ASSIGN_CONFIRM_PRESSED,
                ),
                (
                    TacticalCommandControl::Cancel,
                    art::BTN_ASSIGN_CANCEL_NORMAL,
                    art::BTN_ASSIGN_CANCEL_PRESSED,
                ),
            ] {
                draw_tactical_assignment_button(
                    cache,
                    canvas,
                    control,
                    if pressed == Some(control) {
                        pressed_art
                    } else {
                        normal
                    },
                );
            }
        }
        TacticalCommandPanel::Missions { pending_order } => {
            draw_tactical_bitmap(cache, art::MISSIONS_PANEL, canvas, 481.0, 27.0);
            draw_tactical_bitmap(
                cache,
                pending_order
                    .mission_graphic(player_is_alliance)
                    .unwrap_or(art::MISSIONS_HUD_EMPTY),
                canvas,
                495.0,
                41.0,
            );
            for (control, normal, pressed_art, selected) in [
                (
                    TacticalCommandControl::AttackCapitalShips,
                    if player_is_alliance {
                        art::BTN_ATTACK_CAPITAL_SHIPS_ALLIANCE_NORMAL
                    } else {
                        art::BTN_ATTACK_CAPITAL_SHIPS_EMPIRE_NORMAL
                    },
                    if player_is_alliance {
                        art::BTN_ATTACK_CAPITAL_SHIPS_ALLIANCE_PRESSED
                    } else {
                        art::BTN_ATTACK_CAPITAL_SHIPS_EMPIRE_PRESSED
                    },
                    pending_order == TacticalOrder::AttackCapitalShips,
                ),
                (
                    TacticalCommandControl::AttackFighters,
                    if player_is_alliance {
                        art::BTN_ATTACK_FIGHTERS_ALLIANCE_NORMAL
                    } else {
                        art::BTN_ATTACK_FIGHTERS_EMPIRE_NORMAL
                    },
                    if player_is_alliance {
                        art::BTN_ATTACK_FIGHTERS_ALLIANCE_PRESSED
                    } else {
                        art::BTN_ATTACK_FIGHTERS_EMPIRE_PRESSED
                    },
                    pending_order == TacticalOrder::AttackFighters,
                ),
                (
                    TacticalCommandControl::Recover,
                    if player_is_alliance {
                        art::BTN_RECOVER_ALLIANCE_NORMAL
                    } else {
                        art::BTN_RECOVER_EMPIRE_NORMAL
                    },
                    if player_is_alliance {
                        art::BTN_RECOVER_ALLIANCE_PRESSED
                    } else {
                        art::BTN_RECOVER_EMPIRE_PRESSED
                    },
                    pending_order == TacticalOrder::Recover,
                ),
            ] {
                let resource = if control == TacticalCommandControl::Recover && !selected_fighter {
                    if player_is_alliance {
                        art::BTN_RECOVER_ALLIANCE_DISABLED
                    } else {
                        art::BTN_RECOVER_EMPIRE_DISABLED
                    }
                } else if selected || pressed == Some(control) {
                    pressed_art
                } else {
                    normal
                };
                draw_tactical_assignment_button(cache, canvas, control, resource);
            }
            draw_tactical_assignment_button(
                cache,
                canvas,
                TacticalCommandControl::AttackDeathStar,
                if !attack_death_star_enabled {
                    art::BTN_ATTACK_DEATH_STAR_DISABLED
                } else if pending_order == TacticalOrder::AttackDeathStar
                    || pressed == Some(TacticalCommandControl::AttackDeathStar)
                {
                    art::BTN_ATTACK_DEATH_STAR_PRESSED
                } else {
                    art::BTN_ATTACK_DEATH_STAR_NORMAL
                },
            );
            for (control, normal, pressed_art) in [
                (
                    TacticalCommandControl::Confirm,
                    art::BTN_ASSIGN_CONFIRM_NORMAL,
                    art::BTN_ASSIGN_CONFIRM_PRESSED,
                ),
                (
                    TacticalCommandControl::Cancel,
                    art::BTN_ASSIGN_CANCEL_NORMAL,
                    art::BTN_ASSIGN_CANCEL_PRESSED,
                ),
            ] {
                draw_tactical_assignment_button(
                    cache,
                    canvas,
                    control,
                    if pressed == Some(control) {
                        pressed_art
                    } else {
                        normal
                    },
                );
            }
        }
    }
}

fn draw_tactical_withdraw_confirmation(
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
    pressed: Option<TacticalWithdrawConfirmationControl>,
) {
    use resources::tactical as art;

    draw_tactical_bitmap(cache, art::WITHDRAW_CONFIRMATION_PANEL, canvas, 481.0, 27.0);
    for spec in TACTICAL_WITHDRAW_CONFIRMATION_CONTROLS {
        draw_tactical_bitmap(
            cache,
            if pressed == Some(spec.control) {
                spec.pressed_resource
            } else {
                spec.normal_resource
            },
            canvas,
            spec.rect.x,
            spec.rect.y,
        );
    }

    // FUN_005f1050 places TEXTTACT 56772 at (20, 28) and TEXTTACT 56771
    // at (14, 92) in the 149x236 panel. Macroquad takes a text baseline, so
    // add the recovered ten-pixel font height while preserving those origins.
    let font_size = 10.0 * canvas.scale;
    let text_color = Color::new(0.84, 0.92, 0.84, 1.0);
    for (text, x, top) in [
        ("Withdraw Confirmation", 501.0, 55.0),
        ("Are you sure you wish to", 495.0, 119.0),
        ("order a fleet withdraw?", 495.0, 131.0),
    ] {
        let (screen_x, screen_y) = canvas.point(x, top + 10.0);
        draw_text(text, screen_x, screen_y, font_size, text_color);
    }
}

/// Paint only resource-backed battle chrome. Unmapped command groups remain
/// visible but are not passed off as implemented interactions.
#[expect(
    clippy::too_many_arguments,
    reason = "The original HUD paints one fixed canvas from explicit battle state."
)]
#[expect(
    clippy::too_many_lines,
    reason = "The authored tactical chrome is one fixed 640×480 source composition."
)]
fn draw_original_tactical_hud(
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
    player_is_alliance: bool,
    groups: TacticalGroupHud<'_>,
    attack_death_star_enabled: bool,
    paused: bool,
    highlight_alliance: bool,
    highlight_empire: bool,
    navigation_sets_visible: [bool; 4],
    pressed_control: Option<TacticalHudControl>,
    command_panel: TacticalCommandPanel,
    pressed_command: Option<TacticalCommandControl>,
    battle_options_open: bool,
    withdraw_confirmation_open: bool,
    player_observing: bool,
    withdraw_enabled: bool,
    pressed_options: Option<TacticalBattleOptionsControl>,
    pressed_withdraw_confirmation: Option<TacticalWithdrawConfirmationControl>,
    death_star: Option<TacticalDeathStar>,
    death_star_laser_pressed: bool,
    death_star_targeting: bool,
) {
    use resources::tactical as art;

    if let Some(death_star) = death_star {
        draw_tactical_bitmap(
            cache,
            art::DEATH_STAR_LASER_GAUGE,
            canvas,
            DEATH_STAR_LASER_GAUGE_RECT.x,
            DEATH_STAR_LASER_GAUGE_RECT.y,
        );
        let fraction =
            (death_star.laser_charge / ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE).clamp(0.0, 1.0);
        if fraction > 0.0 {
            let (x, y) = canvas.point(DEATH_STAR_LASER_FILL_RECT.x, DEATH_STAR_LASER_FILL_RECT.y);
            draw_rectangle(
                x,
                y,
                DEATH_STAR_LASER_FILL_RECT.width * canvas.scale * fraction,
                DEATH_STAR_LASER_FILL_RECT.height * canvas.scale,
                Color::new(0.0, 0.82, 0.0, 1.0),
            );
        }
        let ready = death_star.operational()
            && death_star.laser_charge >= ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE;
        let active =
            death_star_laser_pressed || death_star_targeting || death_star.action_committed;
        draw_tactical_bitmap(
            cache,
            if active {
                art::DEATH_STAR_LASER_FIRED
            } else if ready {
                art::DEATH_STAR_LASER_READY
            } else {
                art::DEATH_STAR_LASER_LOADING
            },
            canvas,
            DEATH_STAR_LASER_BUTTON_RECT.x,
            DEATH_STAR_LASER_BUTTON_RECT.y,
        );
    }

    // The top task-force and fighter-group strips occupy slots in bitmap 1000.
    draw_tactical_bitmap(
        cache,
        if player_is_alliance {
            art::TASK_FORCES_ALLIANCE
        } else {
            art::TASK_FORCES_EMPIRE
        },
        canvas,
        10.0,
        1.0,
    );
    for index in 0..8_u8 {
        let assigned = groups.task_forces[usize::from(index)];
        let active = assigned
            && (groups.selected_task_force == Some(index)
                || pressed_control == Some(TacticalHudControl::TaskForce(index)));
        draw_tactical_bitmap(
            cache,
            if active {
                art::BTN_TASK_FORCE_PRESSED
            } else if assigned {
                art::BTN_TASK_FORCE_NORMAL
            } else {
                art::BTN_TASK_FORCE_UNASSIGNED
            },
            canvas,
            60.0 + f32::from(index) * 26.0,
            2.0,
        );
        if assigned {
            let label = (index + 1).to_string();
            let font_size = 13.0 * canvas.scale;
            let width = measure_text(&label, None, font_size as u16, 1.0).width;
            let (button_x, y) = canvas.point(60.0 + f32::from(index) * 26.0, 16.0);
            draw_text(
                &label,
                button_x + (25.0 * canvas.scale - width) * 0.5,
                y,
                font_size,
                BLACK,
            );
        }
    }
    draw_tactical_bitmap(
        cache,
        if player_is_alliance {
            art::FIGHTER_SQUADRONS_ALLIANCE
        } else {
            art::FIGHTER_SQUADRONS_EMPIRE
        },
        canvas,
        275.0,
        1.0,
    );
    for index in 0..4_u8 {
        let assigned = groups.fighter_groups[usize::from(index)];
        let active = assigned
            && (groups.selected_fighter_group == Some(index)
                || pressed_control == Some(TacticalHudControl::FighterGroup(index)));
        draw_tactical_bitmap(
            cache,
            if active {
                FIGHTER_GROUP_PRESSED_ART[usize::from(index)]
            } else if assigned {
                FIGHTER_GROUP_NORMAL_ART[usize::from(index)]
            } else {
                art::SQUADRON_UNASSIGNED
            },
            canvas,
            330.0 + f32::from(index) * 26.0,
            2.0,
        );
    }

    let has_selection =
        groups.selected_fighter_group.is_some() || groups.selected_ship_name.is_some();
    if withdraw_confirmation_open {
        draw_tactical_withdraw_confirmation(cache, canvas, pressed_withdraw_confirmation);
    } else if battle_options_open {
        draw_tactical_bitmap(
            cache,
            if player_is_alliance {
                art::BATTLE_OPTIONS_ALLIANCE_PANEL
            } else {
                art::BATTLE_OPTIONS_EMPIRE_PANEL
            },
            canvas,
            481.0,
            27.0,
        );
        for spec in TACTICAL_BATTLE_OPTIONS_CONTROLS {
            if spec.control == TacticalBattleOptionsControl::Withdraw && !withdraw_enabled {
                continue;
            }
            let selected =
                spec.control == TacticalBattleOptionsControl::ObserveToggle && player_observing;
            let pressed = pressed_options == Some(spec.control);
            let resource = if selected || pressed {
                spec.pressed_resource
            } else {
                spec.normal_resource
            };
            draw_tactical_bitmap(cache, resource, canvas, spec.rect.x, spec.rect.y);
        }
    } else if command_panel != TacticalCommandPanel::Display {
        draw_tactical_assignment_panel(
            cache,
            canvas,
            player_is_alliance,
            command_panel,
            groups.selected_fighter_group.is_some(),
            attack_death_star_enabled,
            pressed_command,
        );
    } else {
        let panel_resource = if groups.selected_fighter_group.is_some() {
            1307
        } else if groups.selected_ship_name.is_some() {
            TACTICAL_SELECTED_SHIP_PANEL
        } else {
            1301
        };
        draw_tactical_bitmap(cache, panel_resource, canvas, 481.0, 27.0);
    }
    if !withdraw_confirmation_open
        && !battle_options_open
        && command_panel == TacticalCommandPanel::Display
    {
        if let (Some(name), Some(group)) = (groups.selected_ship_name, groups.selected_task_force) {
            if let Some(resource) = groups.selected_ship_hud {
                draw_tactical_bitmap(
                    cache,
                    resource,
                    canvas,
                    TACTICAL_SELECTED_SHIP_ART_X,
                    TACTICAL_SELECTED_SHIP_ART_Y,
                );
            }
            if let Some((shield, maximum)) = groups.selected_ship_shield {
                draw_tactical_selected_meter(
                    canvas,
                    TACTICAL_SELECTED_SHIELD_METER,
                    tactical_meter_fraction(shield, maximum),
                    Color::new(0.0, 0.0, 1.0, 1.0),
                );
            }
            if let Some((hull, maximum)) = groups.selected_ship_hull {
                draw_tactical_selected_meter(
                    canvas,
                    TACTICAL_SELECTED_HULL_METER,
                    tactical_meter_fraction(hull, maximum),
                    if player_is_alliance {
                        Color::new(0.0, 0.2, 1.0, 1.0)
                    } else {
                        Color::new(1.0, 0.0, 0.0, 1.0)
                    },
                );
            }
            if let Some(subsystems) = groups.selected_ship_subsystems {
                for (index, resource_id) in subsystems.resource_ids().into_iter().enumerate() {
                    draw_tactical_bitmap(
                        cache,
                        resource_id,
                        canvas,
                        TACTICAL_SUBSYSTEM_X[index],
                        TACTICAL_SUBSYSTEM_Y,
                    );
                }
            }
            let (x, y) = canvas.point(497.0, 48.0);
            draw_text(name, x, y, 11.0 * canvas.scale, WHITE);
            let (x, y) = canvas.point(497.0, 69.0);
            draw_text(
                &format!("Task Force #{}", group + 1),
                x,
                y,
                11.0 * canvas.scale,
                WHITE,
            );
            let (x, y) = canvas.point(497.0, 218.0);
            draw_text(
                groups.selected_order.label(),
                x,
                y,
                10.0 * canvas.scale,
                WHITE,
            );
            let (x, y) = canvas.point(497.0, 230.0);
            draw_text(
                &format!("Tactics: {}", groups.selected_tactic.label()),
                x,
                y,
                9.0 * canvas.scale,
                WHITE,
            );
        } else if let (Some(name), Some(group), Some(hud_resource)) = (
            groups.selected_fighter_name,
            groups.selected_fighter_group,
            groups.selected_fighter_hud,
        ) {
            draw_tactical_bitmap(cache, hud_resource, canvas, 505.0, 76.0);
            let (x, y) = canvas.point(490.0, 45.0);
            draw_text(name, x, y, 10.0 * canvas.scale, WHITE);
            let labels = ["Red Group", "Blue Group", "Green Group", "Gold Group"];
            let (x, y) = canvas.point(490.0, 58.0);
            draw_text(
                labels[usize::from(group.min(3))],
                x,
                y,
                10.0 * canvas.scale,
                WHITE,
            );
            let (x, y) = canvas.point(490.0, 69.0);
            draw_text(
                groups.selected_order.label(),
                x,
                y,
                9.0 * canvas.scale,
                WHITE,
            );
        }
    }
    if !withdraw_confirmation_open
        && !battle_options_open
        && command_panel == TacticalCommandPanel::Display
        && has_selection
    {
        if groups.selected_ship_name.is_some() {
            for (control, normal, pressed_art) in [
                (
                    TacticalCommandControl::PreviousCapital,
                    art::BTN_PREVIOUS_CAPITAL_NORMAL,
                    art::BTN_PREVIOUS_CAPITAL_PRESSED,
                ),
                (
                    TacticalCommandControl::NextCapital,
                    art::BTN_NEXT_CAPITAL_NORMAL,
                    art::BTN_NEXT_CAPITAL_PRESSED,
                ),
            ] {
                draw_tactical_assignment_button(
                    cache,
                    canvas,
                    control,
                    if pressed_command == Some(control) {
                        pressed_art
                    } else {
                        normal
                    },
                );
            }
        }
        for (control, normal, pressed_art) in [
            (
                TacticalCommandControl::OpenMissions,
                art::BTN_MISSIONS_NORMAL,
                art::BTN_MISSIONS_PRESSED,
            ),
            (
                TacticalCommandControl::OpenManeuvers,
                art::BTN_MANEUVERS_TACTICS_NORMAL,
                art::BTN_MANEUVERS_TACTICS_PRESSED,
            ),
        ] {
            draw_tactical_assignment_button(
                cache,
                canvas,
                control,
                if pressed_command == Some(control) {
                    pressed_art
                } else {
                    normal
                },
            );
        }
    }

    for (index, x) in [485.0, 521.0, 560.0, 601.0].into_iter().enumerate() {
        let selected = navigation_sets_visible[index]
            || pressed_control == Some(TacticalHudControl::NavigationSet(index as u8));
        draw_tactical_bitmap(
            cache,
            if selected {
                art::BTN_NAVIGATION_SET_SELECTED[index]
            } else {
                art::BTN_NAVIGATION_SET_NORMAL[index]
            },
            canvas,
            x,
            272.0,
        );
    }
    draw_tactical_bitmap(
        cache,
        if highlight_empire {
            art::HIGHLIGHT_EMPIRE_SHIPS
        } else {
            art::DIM_EMPIRE_SHIPS
        },
        canvas,
        482.0,
        304.0,
    );
    draw_tactical_bitmap(
        cache,
        if highlight_alliance {
            art::HIGHLIGHT_ALLIANCE_SHIPS
        } else {
            art::DIM_ALLIANCE_SHIPS
        },
        canvas,
        517.0,
        304.0,
    );
    draw_tactical_bitmap(
        cache,
        if paused {
            art::BTN_PAUSE_PAUSED
        } else {
            art::BTN_PAUSE_RUNNING
        },
        canvas,
        560.0,
        307.0,
    );
    draw_tactical_bitmap(
        cache,
        if pressed_control == Some(TacticalHudControl::BattleOptions) {
            art::BTN_BATTLE_OPTIONS_PRESSED
        } else {
            art::BTN_BATTLE_OPTIONS_NORMAL
        },
        canvas,
        606.0,
        308.0,
    );

    draw_tactical_bitmap(
        cache,
        if pressed_control == Some(TacticalHudControl::ZoomIn) {
            art::BTN_CAMERA_ZOOM_IN_PRESSED
        } else {
            art::BTN_CAMERA_ZOOM_IN_NORMAL
        },
        canvas,
        486.0,
        343.0,
    );
    draw_tactical_bitmap(
        cache,
        if pressed_control == Some(TacticalHudControl::ZoomOut) {
            art::BTN_CAMERA_ZOOM_OUT_PRESSED
        } else {
            art::BTN_CAMERA_ZOOM_OUT_NORMAL
        },
        canvas,
        603.0,
        343.0,
    );
    for (id, x, y) in [
        (
            if pressed_control == Some(TacticalHudControl::CameraLeft) {
                art::BTN_CAMERA_LEFT_PRESSED
            } else {
                art::BTN_CAMERA_LEFT_NORMAL
            },
            511.0,
            376.0,
        ),
        (
            if pressed_control == Some(TacticalHudControl::CameraRight) {
                art::BTN_CAMERA_RIGHT_PRESSED
            } else {
                art::BTN_CAMERA_RIGHT_NORMAL
            },
            557.0,
            376.0,
        ),
        (
            if pressed_control == Some(TacticalHudControl::CameraUp) {
                art::BTN_CAMERA_UP_PRESSED
            } else {
                art::BTN_CAMERA_UP_NORMAL
            },
            537.0,
            344.0,
        ),
        (
            if pressed_control == Some(TacticalHudControl::CameraDown) {
                art::BTN_CAMERA_DOWN_PRESSED
            } else {
                art::BTN_CAMERA_DOWN_NORMAL
            },
            537.0,
            412.0,
        ),
        (
            if pressed_control == Some(TacticalHudControl::CameraTarget) {
                art::BTN_CAMERA_TARGET_PRESSED
            } else {
                art::BTN_CAMERA_TARGET_NORMAL
            },
            538.0,
            379.0,
        ),
    ] {
        draw_tactical_bitmap(cache, id, canvas, x, y);
    }
    draw_tactical_bitmap(
        cache,
        if pressed_control == Some(TacticalHudControl::CameraRecall) {
            art::BTN_CAMERA_RECALL_PRESSED
        } else {
            art::BTN_CAMERA_RECALL_NORMAL
        },
        canvas,
        484.0,
        430.0,
    );
    draw_tactical_bitmap(
        cache,
        if pressed_control == Some(TacticalHudControl::CameraMemorize) {
            art::BTN_CAMERA_MEMORIZE_PRESSED
        } else {
            art::BTN_CAMERA_MEMORIZE_NORMAL
        },
        canvas,
        585.0,
        430.0,
    );
}

fn activate_tactical_command(state: &mut TacticalState, control: TacticalCommandControl) {
    match control {
        TacticalCommandControl::PreviousCapital | TacticalCommandControl::NextCapital => {
            let step = if control == TacticalCommandControl::PreviousCapital {
                -1
            } else {
                1
            };
            if let Some((from, to, group)) = state
                .session
                .as_mut()
                .and_then(|session| cycle_selected_capital(session, step))
            {
                macroquad::logging::info!(
                    "[tactical_selection] direction={} task_force={} from={} to={}",
                    if step < 0 { "previous" } else { "next" },
                    group + 1,
                    from,
                    to,
                );
            }
        }
        TacticalCommandControl::OpenManeuvers => {
            if let Some(session) = state.session.as_ref() {
                let (pending_order, pending_tactic) = selected_command_values(session);
                state.command_panel = TacticalCommandPanel::Maneuvers {
                    pending_order,
                    pending_tactic,
                };
            }
        }
        TacticalCommandControl::OpenMissions => {
            if let Some(session) = state.session.as_ref() {
                let (pending_order, _) = selected_command_values(session);
                state.command_panel = TacticalCommandPanel::Missions { pending_order };
            }
        }
        TacticalCommandControl::LeftHook => {
            if let TacticalCommandPanel::Maneuvers { pending_order, .. } = &mut state.command_panel
            {
                *pending_order = TacticalOrder::LeftHook;
            }
        }
        TacticalCommandControl::RightHook => {
            if let TacticalCommandPanel::Maneuvers { pending_order, .. } = &mut state.command_panel
            {
                *pending_order = TacticalOrder::RightHook;
            }
        }
        TacticalCommandControl::Hammer => {
            if let TacticalCommandPanel::Maneuvers { pending_order, .. } = &mut state.command_panel
            {
                *pending_order = TacticalOrder::Hammer;
            }
        }
        TacticalCommandControl::Anvil => {
            if let TacticalCommandPanel::Maneuvers { pending_order, .. } = &mut state.command_panel
            {
                *pending_order = TacticalOrder::Anvil;
            }
        }
        TacticalCommandControl::HoldPosition => {
            if let TacticalCommandPanel::Maneuvers { pending_order, .. } = &mut state.command_panel
            {
                *pending_order = TacticalOrder::HoldPosition;
            }
        }
        TacticalCommandControl::ToggleTactic => {
            if let TacticalCommandPanel::Maneuvers { pending_tactic, .. } = &mut state.command_panel
            {
                *pending_tactic = if *pending_tactic == TacticalTactic::StandOff {
                    TacticalTactic::Surround
                } else {
                    TacticalTactic::StandOff
                };
            }
        }
        TacticalCommandControl::AttackCapitalShips => {
            if let TacticalCommandPanel::Missions { pending_order } = &mut state.command_panel {
                *pending_order = TacticalOrder::AttackCapitalShips;
            }
        }
        TacticalCommandControl::AttackFighters => {
            if let TacticalCommandPanel::Missions { pending_order } = &mut state.command_panel {
                *pending_order = TacticalOrder::AttackFighters;
            }
        }
        TacticalCommandControl::Recover => {
            if let TacticalCommandPanel::Missions { pending_order } = &mut state.command_panel {
                *pending_order = TacticalOrder::Recover;
            }
        }
        TacticalCommandControl::AttackDeathStar => {
            if let TacticalCommandPanel::Missions { pending_order } = &mut state.command_panel {
                *pending_order = TacticalOrder::AttackDeathStar;
            }
        }
        TacticalCommandControl::Confirm => {
            let panel = state.command_panel;
            if let Some(session) = state.session.as_mut() {
                let (kind, order, tactic) = match panel {
                    TacticalCommandPanel::Maneuvers {
                        pending_order,
                        pending_tactic,
                    } => ("maneuvers", pending_order, Some(pending_tactic)),
                    TacticalCommandPanel::Missions { pending_order } => {
                        ("missions", pending_order, None)
                    }
                    TacticalCommandPanel::Display => return,
                };
                let (capital_members, fighter_members) =
                    assign_selected_command(session, order, tactic);
                if order == TacticalOrder::AttackDeathStar {
                    macroquad::logging::info!(
                        "[tactical_death_star] trench_run_launch status={} fighter_members={}",
                        if session.has_active_trench_run() {
                            "launched"
                        } else {
                            "rejected"
                        },
                        fighter_members,
                    );
                }
                macroquad::logging::info!(
                    "[tactical_orders] panel={} event=commit order={} tactic={} capital_members={} fighter_members={}",
                    kind,
                    order.source_code(),
                    tactic.map_or(0, TacticalTactic::source_code),
                    capital_members,
                    fighter_members,
                );
            }
            state.command_panel = TacticalCommandPanel::Display;
        }
        TacticalCommandControl::Cancel => {
            macroquad::logging::info!("[tactical_orders] event=cancel");
            state.command_panel = TacticalCommandPanel::Display;
        }
    }
}

fn activate_tactical_battle_options(
    state: &mut TacticalState,
    control: TacticalBattleOptionsControl,
) -> TacticalAction {
    match control {
        TacticalBattleOptionsControl::Withdraw => {
            state.battle_options_open = false;
            state.battle_options_pressed = None;
            state.withdraw_confirmation_open = true;
            state.withdraw_confirmation_pressed = None;
            macroquad::logging::info!(
                "[tactical_options] command=withdraw status=confirmation_open panel=1310 controls=1113:1114:1115:1116 title=56772 body=56771"
            );
            TacticalAction::None
        }
        TacticalBattleOptionsControl::SimulateRemainder => {
            macroquad::logging::info!("[tactical_options] command=simulate_remainder");
            TacticalAction::AutoResolve
        }
        TacticalBattleOptionsControl::ObserveToggle => {
            state.player_observing = !state.player_observing;
            macroquad::logging::info!(
                "[tactical_options] command={} observing={}",
                if state.player_observing {
                    "observe"
                } else {
                    "take_command"
                },
                state.player_observing
            );
            TacticalAction::None
        }
        TacticalBattleOptionsControl::GameOptions => {
            macroquad::logging::info!("[tactical_options] command=game_options status=routed");
            TacticalAction::OpenGameOptions
        }
        TacticalBattleOptionsControl::Close => {
            state.battle_options_open = false;
            macroquad::logging::info!("[tactical_options] command=close");
            TacticalAction::None
        }
    }
}

fn activate_tactical_withdraw_confirmation(
    state: &mut TacticalState,
    control: TacticalWithdrawConfirmationControl,
) -> TacticalAction {
    state.withdraw_confirmation_open = false;
    state.withdraw_confirmation_pressed = None;
    match control {
        TacticalWithdrawConfirmationControl::Confirm => {
            macroquad::logging::info!(
                "[tactical_options] command=withdraw status=withdrawal_started confirmation=accepted"
            );
            TacticalAction::WithdrawFromBattle
        }
        TacticalWithdrawConfirmationControl::Cancel => {
            macroquad::logging::info!(
                "[tactical_options] command=withdraw status=confirmation_cancelled"
            );
            TacticalAction::None
        }
    }
}

fn update_tactical_camera_follow(state: &mut TacticalState) -> bool {
    let Some(follow) = state.camera_follow else {
        return false;
    };
    let Some(session) = state.session.as_ref() else {
        state.camera_follow = None;
        return false;
    };
    let (source_positions, fallback_positions, object_id): (Vec<_>, Vec<_>, u32) = match follow {
        TacticalCameraFollow::TaskForce(group) => {
            let members = session
                .ships
                .iter()
                .filter(|ship| {
                    ship.alive
                        && !ship.retreating
                        && ship.is_attacker == session.player_is_attacker
                        && ship.task_force == group
                })
                .collect::<Vec<_>>();
            (
                members.iter().map(|ship| ship.source_position).collect(),
                members.iter().map(|ship| (ship.x, ship.y)).collect(),
                10_001 + u32::from(group),
            )
        }
        TacticalCameraFollow::FighterGroup(group) => {
            let members = session
                .fighters
                .iter()
                .filter(|fighter| {
                    fighter.alive
                        && fighter.squad_count > 0
                        && fighter.is_attacker == session.player_is_attacker
                        && fighter.fighter_group == group
                })
                .collect::<Vec<_>>();
            (
                members
                    .iter()
                    .map(|fighter| fighter.source_position)
                    .collect(),
                members
                    .iter()
                    .map(|fighter| (fighter.x, fighter.y))
                    .collect(),
                10_101 + u32::from(group),
            )
        }
    };
    if source_positions.is_empty() {
        state.camera_follow = None;
        return false;
    }
    let count = source_positions.len() as f32;
    let source = TacticalWorldPosition {
        x: source_positions
            .iter()
            .map(|position| position.x)
            .sum::<f32>()
            / count,
        y: source_positions
            .iter()
            .map(|position| position.y)
            .sum::<f32>()
            / count,
        z: source_positions
            .iter()
            .map(|position| position.z)
            .sum::<f32>()
            / count,
    };
    let fallback_count = fallback_positions.len() as f32;
    let fallback_x = fallback_positions
        .iter()
        .map(|position| position.0)
        .sum::<f32>()
        / fallback_count;
    let fallback_y = fallback_positions
        .iter()
        .map(|position| position.1)
        .sum::<f32>()
        / fallback_count;
    state.camera_x = fallback_x - ARENA_WIDTH * 0.5;
    state.camera_y = fallback_y - ARENA_HEIGHT * 0.5;
    state
        .asset_renderer
        .focus_target(object_id, source.rendered());
    true
}

fn handle_original_tactical_controls(
    state: &mut TacticalState,
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
) -> TacticalAction {
    let (mouse_x, mouse_y) = mouse_position();
    let (x, y) = canvas.logical_pointer(mouse_x, mouse_y);
    if state.withdraw_confirmation_open {
        let hovered = tactical_withdraw_confirmation_control_at(cache, x, y);
        if is_mouse_button_pressed(MouseButton::Left) {
            state.withdraw_confirmation_pressed = hovered;
        }
        if is_mouse_button_released(MouseButton::Left) {
            let captured = state.withdraw_confirmation_pressed.take();
            if let Some(captured) = captured.filter(|captured| Some(*captured) == hovered) {
                return activate_tactical_withdraw_confirmation(state, captured);
            }
        } else if !is_mouse_button_down(MouseButton::Left) {
            state.withdraw_confirmation_pressed = None;
        }
        return TacticalAction::None;
    }
    let death_star_ready = state
        .session
        .as_ref()
        .is_some_and(tactical_death_star_laser_ready);
    if state.death_star_targeting && !death_star_ready {
        state.death_star_targeting = false;
    }
    let death_star_hovered = death_star_ready && tactical_death_star_laser_control_at(cache, x, y);
    if is_mouse_button_pressed(MouseButton::Left) && death_star_hovered {
        state.death_star_laser_pressed = true;
        return TacticalAction::None;
    }
    if is_mouse_button_released(MouseButton::Left) && state.death_star_laser_pressed {
        state.death_star_laser_pressed = false;
        if death_star_hovered {
            state.death_star_targeting = true;
            macroquad::logging::info!(
                "[tactical_death_star] superlaser_targeting status=armed input=right_click"
            );
        }
        return TacticalAction::None;
    }
    if !is_mouse_button_down(MouseButton::Left) {
        state.death_star_laser_pressed = false;
    }
    if state.battle_options_open {
        let withdraw_enabled = state
            .session
            .as_ref()
            .is_some_and(tactical_withdraw_enabled);
        let hovered = tactical_battle_options_control_at(cache, withdraw_enabled, x, y);
        if is_mouse_button_pressed(MouseButton::Left) {
            state.battle_options_pressed = hovered;
        }
        if is_mouse_button_released(MouseButton::Left) {
            let captured = state.battle_options_pressed.take();
            if let Some(captured) = captured.filter(|captured| Some(*captured) == hovered) {
                return activate_tactical_battle_options(state, captured);
            }
        } else if !is_mouse_button_down(MouseButton::Left) {
            state.battle_options_pressed = None;
        }
        return TacticalAction::None;
    }
    if is_mouse_button_pressed(MouseButton::Left) {
        let (has_selection, selected_fighter, attack_death_star_enabled, player_is_alliance) =
            state
                .session
                .as_ref()
                .map_or((false, false, false, true), |session| {
                    let selected_fighter = session.selected_fighter_group.is_some();
                    let attack_death_star_enabled = selected_fighter
                        && session.death_star.is_some_and(|death_star| {
                            death_star.operational()
                                && death_star.is_attacker != session.player_is_attacker
                        });
                    (
                        selected_fighter
                            || session.ships.iter().any(|ship| {
                                ship.alive
                                    && ship.selected
                                    && ship.is_attacker == session.player_is_attacker
                            }),
                        selected_fighter,
                        attack_death_star_enabled,
                        session.player_is_attacker == session.attacker_is_alliance,
                    )
                });
        let command_control = if state.player_observing {
            None
        } else {
            tactical_command_control_at(
                cache,
                state.command_panel,
                has_selection,
                selected_fighter,
                attack_death_star_enabled,
                player_is_alliance,
                x,
                y,
            )
        };
        if let Some(control) = command_control {
            activate_tactical_command(state, control);
        } else {
            match tactical_hud_control_at(cache, x, y) {
                Some(TacticalHudControl::NavigationSet(set)) => {
                    if let Some(visible) = state.navigation_sets_visible.get_mut(usize::from(set)) {
                        *visible = !*visible;
                        state.logged_navigation_scene = None;
                        macroquad::logging::info!(
                            "[tactical_navigation] event=set_visibility set={} visible={}",
                            set + 1,
                            *visible,
                        );
                    }
                }
                Some(TacticalHudControl::TaskForce(group)) => {
                    if let Some(session) = state.session.as_mut() {
                        let control =
                            is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl);
                        let changed = if control {
                            assign_selected_to_task_force(session, group)
                        } else {
                            select_task_force(session, group)
                        };
                        if changed > 0 {
                            state.command_panel = TacticalCommandPanel::Display;
                            macroquad::logging::info!(
                                "[tactical_groups] task_force_{} slot={} members={}",
                                if control { "assigned" } else { "selected" },
                                group + 1,
                                changed,
                            );
                        }
                    }
                }
                Some(TacticalHudControl::FighterGroup(group)) => {
                    if let Some(session) = state.session.as_mut() {
                        if select_fighter_group(session, group) {
                            state.command_panel = TacticalCommandPanel::Display;
                            let members = session
                                .fighters
                                .iter()
                                .filter(|fighter| fighter.selected)
                                .count();
                            macroquad::logging::info!(
                                "[tactical_groups] fighter_group_selected slot={} members={}",
                                group + 1,
                                members,
                            );
                        }
                    }
                }
                Some(TacticalHudControl::Pause) => return TacticalAction::TogglePause,
                Some(TacticalHudControl::ZoomIn) => {
                    state.camera_follow = None;
                    state.zoom = (state.zoom * 1.25).min(2.0);
                    state.asset_renderer.zoom_in();
                }
                Some(TacticalHudControl::ZoomOut) => {
                    state.camera_follow = None;
                    state.zoom = (state.zoom / 1.25).max(0.5);
                    state.asset_renderer.zoom_out();
                }
                Some(TacticalHudControl::CameraTarget) => {
                    state.camera_follow = None;
                    let selected = state.session.as_ref().and_then(|session| {
                        session.selected_ship.and_then(|index| {
                            session.ships.get(index).map(|ship| {
                                (index, ship.x, ship.y, ship.source_position.rendered())
                            })
                        })
                    });
                    if let Some((_index, x, y, _source_position)) = selected {
                        (state.camera_x, state.camera_y) = camera_offset_for_target(x, y);
                        state.asset_renderer.focus_target(
                            u32::try_from(_index).unwrap_or(u32::MAX).saturating_add(1),
                            _source_position,
                        );
                    }
                }
                Some(TacticalHudControl::CameraLeft) => {
                    if state.camera_follow.take().is_some() {
                        macroquad::logging::info!(
                            "[tactical_camera] command=follow_exit reason=manual"
                        );
                    }
                    state.asset_renderer.turn_left();
                }
                Some(TacticalHudControl::CameraRight) => {
                    state.camera_follow = None;
                    state.asset_renderer.turn_right();
                }
                Some(TacticalHudControl::CameraUp) => {
                    state.camera_follow = None;
                    state.asset_renderer.pitch_up();
                }
                Some(TacticalHudControl::CameraDown) => {
                    state.camera_follow = None;
                    state.asset_renderer.pitch_down();
                }
                Some(TacticalHudControl::CameraMemorize) => {
                    state.memorized_camera =
                        state
                            .asset_renderer
                            .camera_state()
                            .map(|source| TacticalCameraBookmark {
                                source,
                                camera_x: state.camera_x,
                                camera_y: state.camera_y,
                                zoom: state.zoom,
                            });
                    macroquad::logging::info!(
                        "[tactical_camera] command=memorize stored={}",
                        state.memorized_camera.is_some(),
                    );
                }
                Some(TacticalHudControl::CameraRecall) => {
                    if let Some(bookmark) = state.memorized_camera.or(state.default_camera) {
                        state.asset_renderer.restore_camera_state(bookmark.source);
                        state.camera_x = bookmark.camera_x;
                        state.camera_y = bookmark.camera_y;
                        state.zoom = bookmark.zoom;
                        state.camera_follow = None;
                        macroquad::logging::info!(
                            "[tactical_camera] command=recall source={}",
                            if state.memorized_camera.is_some() {
                                "memorized"
                            } else {
                                "default"
                            },
                        );
                    }
                }
                Some(TacticalHudControl::HighlightAlliance) => {
                    state.highlight_alliance = !state.highlight_alliance;
                }
                Some(TacticalHudControl::HighlightEmpire) => {
                    state.highlight_empire = !state.highlight_empire;
                }
                Some(TacticalHudControl::BattleOptions) => {
                    state.battle_options_open = true;
                    state.withdraw_confirmation_open = false;
                    state.withdraw_confirmation_pressed = None;
                    state.command_panel = TacticalCommandPanel::Display;
                    macroquad::logging::info!("[tactical_options] command=open");
                }
                None => {}
            }
        }
    }
    let task_force_keys = [
        KeyCode::F1,
        KeyCode::F2,
        KeyCode::F3,
        KeyCode::F4,
        KeyCode::F5,
        KeyCode::F6,
        KeyCode::F7,
        KeyCode::F8,
    ];
    for (index, key) in task_force_keys.iter().enumerate() {
        if is_key_pressed(*key) {
            if let Some(session) = state.session.as_mut() {
                if select_task_force(session, u8::try_from(index).unwrap_or(7)) > 0 {
                    state.command_panel = TacticalCommandPanel::Display;
                    let group = u8::try_from(index).unwrap_or(7);
                    state.camera_follow = Some(TacticalCameraFollow::TaskForce(group));
                    macroquad::logging::info!(
                        "[tactical_camera] command=follow kind=task_force group={}",
                        group + 1,
                    );
                }
            }
        }
    }
    let fighter_group_keys = [KeyCode::F9, KeyCode::F10, KeyCode::F11, KeyCode::F12];
    for (index, key) in fighter_group_keys.iter().enumerate() {
        if is_key_pressed(*key) {
            if let Some(session) = state.session.as_mut() {
                if select_fighter_group(session, u8::try_from(index).unwrap_or(3)) {
                    state.command_panel = TacticalCommandPanel::Display;
                    let group = u8::try_from(index).unwrap_or(3);
                    state.camera_follow = Some(TacticalCameraFollow::FighterGroup(group));
                    macroquad::logging::info!(
                        "[tactical_camera] command=follow kind=fighter_group group={}",
                        group + 1,
                    );
                }
            }
        }
    }
    if is_key_pressed(KeyCode::Equal) {
        state.camera_follow = None;
        state.zoom = (state.zoom * 1.25).min(2.0);
        state.asset_renderer.zoom_in();
    }
    if is_key_pressed(KeyCode::Minus) {
        state.camera_follow = None;
        state.zoom = (state.zoom / 1.25).max(0.5);
        state.asset_renderer.zoom_out();
    }
    if is_key_pressed(KeyCode::Enter) {
        state.camera_follow = None;
        let selected = state.session.as_ref().and_then(|session| {
            session.selected_ship.and_then(|index| {
                session.ships.get(index).map(|ship| {
                    (
                        u32::try_from(index).unwrap_or(u32::MAX).saturating_add(1),
                        ship.x,
                        ship.y,
                        ship.source_position.rendered(),
                    )
                })
            })
        });
        if let Some((object_id, x, y, source)) = selected {
            (state.camera_x, state.camera_y) = camera_offset_for_target(x, y);
            state.asset_renderer.focus_target(object_id, source);
        }
    }
    TacticalAction::None
}

fn camera_offset_for_target(x: f32, y: f32) -> (f32, f32) {
    (x - ARENA_WIDTH * 0.5, y - ARENA_HEIGHT * 0.5)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BattleResultCondition {
    Operational,
    Damaged,
    Destroyed,
}

#[derive(Debug, Clone)]
struct BattleResultEntry {
    name: String,
    condition: BattleResultCondition,
}

#[derive(Debug, Clone, Copy)]
struct BattleResultSkin {
    frame: u32,
    rail: u32,
    close: [u32; 2],
    tabs: [[u32; 2]; 4],
    close_rect: NativeRect,
    tab_rects: [NativeRect; 4],
}

fn battle_result_skin(player_is_alliance: bool) -> BattleResultSkin {
    use resources::strategy as art;
    if player_is_alliance {
        BattleResultSkin {
            frame: art::BATTLE_WINDOW_ALLIANCE,
            rail: art::BATTLE_RAIL_ALLIANCE,
            close: [
                art::BATTLE_CLOSE_ALLIANCE_NORMAL,
                art::BATTLE_CLOSE_ALLIANCE_PRESSED,
            ],
            tabs: [
                [
                    art::BATTLE_SUMMARY_ALLIANCE_NORMAL,
                    art::BATTLE_SUMMARY_ALLIANCE_PRESSED,
                ],
                [
                    art::BATTLE_ALLIANCE_FORCES_ALLIANCE_NORMAL,
                    art::BATTLE_ALLIANCE_FORCES_ALLIANCE_PRESSED,
                ],
                [
                    art::BATTLE_EMPIRE_FORCES_ALLIANCE_NORMAL,
                    art::BATTLE_EMPIRE_FORCES_ALLIANCE_PRESSED,
                ],
                [
                    art::BATTLE_DESTINATION_ALLIANCE_NORMAL,
                    art::BATTLE_DESTINATION_ALLIANCE_PRESSED,
                ],
            ],
            close_rect: NativeRect::new(423.0, 25.0, 32.0, 31.0),
            tab_rects: [
                NativeRect::new(418.0, 88.0, 41.0, 41.0),
                NativeRect::new(418.0, 142.0, 41.0, 41.0),
                NativeRect::new(418.0, 196.0, 41.0, 41.0),
                NativeRect::new(418.0, 250.0, 41.0, 41.0),
            ],
        }
    } else {
        BattleResultSkin {
            frame: art::BATTLE_WINDOW_EMPIRE,
            rail: art::BATTLE_RAIL_EMPIRE,
            close: [
                art::BATTLE_CLOSE_EMPIRE_NORMAL,
                art::BATTLE_CLOSE_EMPIRE_PRESSED,
            ],
            tabs: [
                [
                    art::BATTLE_SUMMARY_EMPIRE_NORMAL,
                    art::BATTLE_SUMMARY_EMPIRE_PRESSED,
                ],
                [
                    art::BATTLE_ALLIANCE_FORCES_EMPIRE_NORMAL,
                    art::BATTLE_ALLIANCE_FORCES_EMPIRE_PRESSED,
                ],
                [
                    art::BATTLE_EMPIRE_FORCES_EMPIRE_NORMAL,
                    art::BATTLE_EMPIRE_FORCES_EMPIRE_PRESSED,
                ],
                [
                    art::BATTLE_DESTINATION_EMPIRE_NORMAL,
                    art::BATTLE_DESTINATION_EMPIRE_PRESSED,
                ],
            ],
            close_rect: NativeRect::new(426.0, 17.0, 44.0, 41.0),
            tab_rects: [
                NativeRect::new(426.0, 89.0, 44.0, 41.0),
                NativeRect::new(426.0, 148.0, 44.0, 41.0),
                NativeRect::new(426.0, 207.0, 44.0, 41.0),
                NativeRect::new(426.0, 266.0, 44.0, 41.0),
            ],
        }
    }
}

impl BattleResultTab {
    const fn index(self) -> usize {
        match self {
            Self::Summary => 0,
            Self::AllianceForces => 1,
            Self::ImperialForces => 2,
            Self::GoDirectlyTo => 3,
        }
    }

    const fn from_index(index: usize) -> Self {
        match index {
            1 => Self::AllianceForces,
            2 => Self::ImperialForces,
            3 => Self::GoDirectlyTo,
            _ => Self::Summary,
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::Summary => "summary",
            Self::AllianceForces => "alliance_forces",
            Self::ImperialForces => "imperial_forces",
            Self::GoDirectlyTo => "go_directly_to",
        }
    }
}

impl BattleResultCategory {
    const ALL: [Self; 4] = [
        Self::CapitalShips,
        Self::Fighters,
        Self::Troops,
        Self::Personnel,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::CapitalShips => "Capital Ships",
            Self::Fighters => "Fighters",
            Self::Troops => "Troops",
            Self::Personnel => "Personnel",
        }
    }
}

fn player_is_alliance(session: &BattleSession) -> bool {
    session.player_is_attacker == session.attacker_is_alliance
}

fn winner_is_alliance(session: &BattleSession) -> Option<bool> {
    match session.winner {
        Some(CombatWinner::Attacker) => Some(session.attacker_is_alliance),
        Some(CombatWinner::Defender) => Some(!session.attacker_is_alliance),
        Some(CombatWinner::Draw) | None => None,
    }
}

fn winner_fleet(session: &BattleSession) -> Option<FleetKey> {
    match session.winner {
        Some(CombatWinner::Attacker) => Some(session.attacker_fleet),
        Some(CombatWinner::Defender) => Some(session.defender_fleet),
        Some(CombatWinner::Draw) | None => None,
    }
}

fn result_fleet_for_faction(session: &BattleSession, alliance: bool) -> FleetKey {
    if session.attacker_is_alliance == alliance {
        session.attacker_fleet
    } else {
        session.defender_fleet
    }
}

fn battle_result_entries(
    session: &BattleSession,
    world: &GameWorld,
    troop_transport: &TroopTransportState,
    alliance: bool,
    category: BattleResultCategory,
) -> Vec<BattleResultEntry> {
    match category {
        BattleResultCategory::CapitalShips => {
            let mut entries = session
                .ships
                .iter()
                .filter(|ship| ship.identity.is_alliance == alliance)
                .map(|ship| BattleResultEntry {
                    name: ship.name.clone(),
                    condition: if !ship.alive || ship.hull_current <= 0 {
                        BattleResultCondition::Destroyed
                    } else if ship.hull_current < ship.hull_max
                        || ship.subsystem_damage.total_hits() > 0
                    {
                        BattleResultCondition::Damaged
                    } else {
                        BattleResultCondition::Operational
                    },
                })
                .collect::<Vec<_>>();
            if let Some(death_star) = session
                .death_star
                .filter(|value| value.is_alliance == alliance)
            {
                entries.push(BattleResultEntry {
                    name: "Death Star".into(),
                    condition: if death_star.destroyed || death_star.hull <= 0.0 {
                        BattleResultCondition::Destroyed
                    } else if death_star.hull < ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE {
                        BattleResultCondition::Damaged
                    } else {
                        BattleResultCondition::Operational
                    },
                });
            }
            entries
        }
        BattleResultCategory::Fighters => session
            .fighters
            .iter()
            .filter(|fighter| fighter.identity.is_alliance == alliance)
            .map(|fighter| BattleResultEntry {
                name: fighter.name.clone(),
                condition: if !fighter.alive || fighter.squad_count == 0 {
                    BattleResultCondition::Destroyed
                } else if fighter.hull_current < fighter.hull_max {
                    BattleResultCondition::Damaged
                } else {
                    BattleResultCondition::Operational
                },
            })
            .collect(),
        BattleResultCategory::Troops => {
            let fleet_key = result_fleet_for_faction(session, alliance);
            let surviving_capacity = session
                .ships
                .iter()
                .filter(|ship| ship.identity.is_alliance == alliance && ship.alive)
                .filter_map(|ship| world.capital_ship_classes.get(ship.class_key))
                .map(|class| class.troop_capacity)
                .fold(0_u32, u32::saturating_add) as usize;
            troop_transport
                .cargo(fleet_key)
                .iter()
                .enumerate()
                .map(|(index, troop)| {
                    let strength = world
                        .troops
                        .get(*troop)
                        .map_or(0, |unit| unit.regiment_strength);
                    BattleResultEntry {
                        name: format!("Regiment {}", index + 1),
                        condition: if index >= surviving_capacity || strength <= 0 {
                            BattleResultCondition::Destroyed
                        } else if strength < 100 {
                            BattleResultCondition::Damaged
                        } else {
                            BattleResultCondition::Operational
                        },
                    }
                })
                .collect()
        }
        BattleResultCategory::Personnel => {
            let fleet_key = result_fleet_for_faction(session, alliance);
            world
                .fleets
                .get(fleet_key)
                .into_iter()
                .flat_map(|fleet| fleet.characters.iter())
                .filter_map(|character| world.characters.get(*character))
                .map(|character| BattleResultEntry {
                    name: character.name.clone(),
                    condition: BattleResultCondition::Operational,
                })
                .collect()
        }
    }
}

fn draw_result_text(
    canvas: TacticalCanvas,
    text: &str,
    x: f32,
    baseline_y: f32,
    size: f32,
    color: Color,
) {
    let (screen_x, screen_y) = canvas.point(x, baseline_y);
    draw_text(text, screen_x, screen_y, size * canvas.scale, color);
}

fn draw_result_text_centered(
    canvas: TacticalCanvas,
    text: &str,
    center_x: f32,
    baseline_y: f32,
    size: f32,
    color: Color,
) {
    let font_size = (size * canvas.scale).max(1.0) as u16;
    let dimensions = measure_text(text, None, font_size, 1.0);
    let (screen_x, screen_y) = canvas.point(center_x, baseline_y);
    draw_text(
        text,
        screen_x - dimensions.width * 0.5,
        screen_y,
        f32::from(font_size),
        color,
    );
}

fn draw_result_wrapped_lines(
    canvas: TacticalCanvas,
    lines: &[String],
    x: f32,
    mut baseline_y: f32,
    max_width: f32,
    size: f32,
    color: Color,
) {
    let font_size = (size * canvas.scale).max(1.0) as u16;
    for line in lines {
        let mut current = String::new();
        for word in line.split_whitespace() {
            let candidate = if current.is_empty() {
                word.to_owned()
            } else {
                format!("{current} {word}")
            };
            if !current.is_empty()
                && measure_text(&candidate, None, font_size, 1.0).width > max_width * canvas.scale
            {
                draw_result_text(canvas, &current, x, baseline_y, size, color);
                baseline_y += size + 2.0;
                current = word.to_owned();
            } else {
                current = candidate;
            }
        }
        if !current.is_empty() {
            draw_result_text(canvas, &current, x, baseline_y, size, color);
            baseline_y += size + 2.0;
        }
    }
}

fn battle_result_summary_lines(session: &BattleSession) -> Vec<String> {
    let Some(winner_alliance) = winner_is_alliance(session) else {
        return vec![
            "Neither fleet is victorious.".into(),
            "Both forces have been destroyed or withdrawn.".into(),
        ];
    };
    let winner = if winner_alliance {
        "Alliance"
    } else {
        "Imperial"
    };
    let loser = if winner_alliance {
        "Imperial"
    } else {
        "Alliance"
    };
    let losing_side_is_attacker = session.attacker_is_alliance != winner_alliance;
    let withdrew = session
        .ships
        .iter()
        .any(|ship| ship.is_attacker == losing_side_is_attacker && ship.retreated);
    vec![
        format!("The {winner} fleet is victorious."),
        format!(
            "{} is now under blockade by {winner} forces.",
            session.system_name
        ),
        if withdrew {
            format!("The {loser} fleet has withdrawn from the system.")
        } else {
            format!("The {loser} fleet has been completely destroyed.")
        },
    ]
}

fn draw_battle_result_summary(
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
    session: &BattleSession,
    text_color: Color,
) {
    use resources::strategy as art;
    let scene = match winner_is_alliance(session) {
        Some(true) => art::EVENT_BATTLE_ALLIANCE_VICTORY,
        Some(false) => art::EVENT_BATTLE_EMPIRE_VICTORY,
        None => art::EVENT_BATTLE_MUTUAL_DESTRUCTION,
    };
    draw_strategy_bitmap(
        cache,
        scene,
        canvas,
        BATTLE_RESULTS_WINDOW_X + BATTLE_RESULTS_SCENE_X,
        BATTLE_RESULTS_WINDOW_Y + BATTLE_RESULTS_SCENE_Y,
    );
    draw_result_text_centered(
        canvas,
        &format!("Battle at {}", session.system_name),
        BATTLE_RESULTS_WINDOW_X + 212.0,
        BATTLE_RESULTS_WINDOW_Y + 34.0,
        18.0,
        text_color,
    );
    draw_result_wrapped_lines(
        canvas,
        &battle_result_summary_lines(session),
        BATTLE_RESULTS_WINDOW_X + 25.0,
        BATTLE_RESULTS_WINDOW_Y + 230.0,
        350.0,
        14.0,
        text_color,
    );
}

fn draw_battle_result_force_details(
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
    session: &BattleSession,
    world: &GameWorld,
    troop_transport: &TroopTransportState,
    alliance: bool,
    category: BattleResultCategory,
    text_color: Color,
) {
    use resources::strategy as art;
    draw_strategy_bitmap(
        cache,
        art::BATTLE_FORCE_DETAIL_THREE_COLUMN,
        canvas,
        BATTLE_RESULTS_WINDOW_X + BATTLE_RESULTS_SCENE_X,
        BATTLE_RESULTS_WINDOW_Y + BATTLE_RESULTS_SCENE_Y,
    );
    let faction = if alliance { "Alliance" } else { "Imperial" };
    draw_result_text_centered(
        canvas,
        &format!("Battle at {}", session.system_name),
        BATTLE_RESULTS_WINDOW_X + 212.0,
        BATTLE_RESULTS_WINDOW_Y + 32.0,
        18.0,
        text_color,
    );
    draw_result_text_centered(
        canvas,
        &format!("{faction} Forces"),
        BATTLE_RESULTS_WINDOW_X + 212.0,
        BATTLE_RESULTS_WINDOW_Y + 52.0,
        16.0,
        text_color,
    );

    for (index, item) in BattleResultCategory::ALL.into_iter().enumerate() {
        let center = BATTLE_RESULTS_WINDOW_X + 62.0 + index as f32 * 96.0;
        let color = if item == category { WHITE } else { text_color };
        draw_result_text_centered(
            canvas,
            item.label(),
            center,
            BATTLE_RESULTS_WINDOW_Y + 78.0,
            10.0,
            color,
        );
    }

    for (index, label) in ["Operational", "Damaged", "Destroyed"]
        .into_iter()
        .enumerate()
    {
        draw_result_text_centered(
            canvas,
            label,
            BATTLE_RESULTS_WINDOW_X + 80.0 + index as f32 * 111.0,
            BATTLE_RESULTS_WINDOW_Y + 99.0,
            11.0,
            text_color,
        );
    }

    let entries = battle_result_entries(session, world, troop_transport, alliance, category);
    for (column, condition) in [
        BattleResultCondition::Operational,
        BattleResultCondition::Damaged,
        BattleResultCondition::Destroyed,
    ]
    .into_iter()
    .enumerate()
    {
        for (row, entry) in entries
            .iter()
            .filter(|entry| entry.condition == condition)
            .take(9)
            .enumerate()
        {
            draw_result_text_centered(
                canvas,
                &entry.name,
                BATTLE_RESULTS_WINDOW_X + 80.0 + column as f32 * 111.0,
                BATTLE_RESULTS_WINDOW_Y + 122.0 + row as f32 * 20.0,
                11.0,
                WHITE,
            );
        }
    }
}

fn draw_battle_result_destinations(
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
    session: &BattleSession,
    player_alliance: bool,
    mouse: (f32, f32),
    mouse_down: bool,
    mouse_released: bool,
    text_color: Color,
) -> TacticalAction {
    use resources::strategy as art;
    draw_strategy_bitmap(
        cache,
        art::BATTLE_DESTINATION_BACKGROUND,
        canvas,
        BATTLE_RESULTS_WINDOW_X + BATTLE_RESULTS_SCENE_X,
        BATTLE_RESULTS_WINDOW_Y + BATTLE_RESULTS_SCENE_Y,
    );
    draw_result_text_centered(
        canvas,
        "Go Directly To...",
        BATTLE_RESULTS_WINDOW_X + 212.0,
        BATTLE_RESULTS_WINDOW_Y + 56.0,
        18.0,
        text_color,
    );

    let system_rect = NativeRect::new(
        BATTLE_RESULTS_WINDOW_X + 29.0,
        BATTLE_RESULTS_WINDOW_Y + 182.0,
        169.0,
        96.0,
    );
    let fleet_rect = NativeRect::new(
        BATTLE_RESULTS_WINDOW_X + 229.0,
        BATTLE_RESULTS_WINDOW_Y + 182.0,
        169.0,
        96.0,
    );
    draw_strategy_bitmap(
        cache,
        art::BATTLE_DESTINATION_SYSTEM,
        canvas,
        system_rect.x,
        system_rect.y,
    );
    let winning_alliance = winner_is_alliance(session);
    if let Some(winning_alliance) = winning_alliance {
        draw_strategy_bitmap(
            cache,
            if winning_alliance {
                art::BATTLE_DESTINATION_ALLIANCE_FLEET
            } else {
                art::BATTLE_DESTINATION_EMPIRE_FLEET
            },
            canvas,
            fleet_rect.x,
            fleet_rect.y,
        );
    }
    let selected = if player_alliance {
        art::BATTLE_DESTINATION_ALLIANCE_SELECTED
    } else {
        art::BATTLE_DESTINATION_EMPIRE_SELECTED
    };
    if mouse_down && system_rect.contains(mouse.0, mouse.1) {
        draw_strategy_bitmap(cache, selected, canvas, system_rect.x, system_rect.y);
    }
    if winning_alliance.is_some() && mouse_down && fleet_rect.contains(mouse.0, mouse.1) {
        draw_strategy_bitmap(cache, selected, canvas, fleet_rect.x, fleet_rect.y);
    }
    draw_result_text_centered(
        canvas,
        &session.system_name,
        system_rect.x + system_rect.width * 0.5,
        BATTLE_RESULTS_WINDOW_Y + 174.0,
        14.0,
        WHITE,
    );
    if let Some(winning_alliance) = winning_alliance {
        draw_result_text_centered(
            canvas,
            if winning_alliance {
                "Alliance Fleet"
            } else {
                "Imperial Fleet"
            },
            fleet_rect.x + fleet_rect.width * 0.5,
            BATTLE_RESULTS_WINDOW_Y + 174.0,
            14.0,
            WHITE,
        );
    }

    if mouse_released && system_rect.contains(mouse.0, mouse.1) {
        macroquad::logging::info!("[tactical-results] destination=system");
        return TacticalAction::OpenBattleSystem(session.system);
    }
    if mouse_released && fleet_rect.contains(mouse.0, mouse.1) {
        if let Some(fleet) = winner_fleet(session) {
            macroquad::logging::info!("[tactical-results] destination=fleet");
            return TacticalAction::OpenBattleFleet(fleet);
        }
    }
    TacticalAction::None
}

fn draw_original_battle_results(
    state: &mut TacticalState,
    cache: &mut BmpCache,
    world: &GameWorld,
    troop_transport: &TroopTransportState,
) -> TacticalAction {
    let canvas = TacticalCanvas::new(screen_width(), screen_height());
    let session = state.session.as_ref().expect("active result session");
    prewarm_tactical_font_atlas(session, canvas, &mut state.warmed_font_sizes);
    let player_alliance = player_is_alliance(session);
    let skin = battle_result_skin(player_alliance);
    let text_color = if player_alliance {
        Color::from_rgba(255, 45, 40, 255)
    } else {
        Color::from_rgba(40, 255, 65, 255)
    };

    clear_background(BLACK);
    let shell = if player_alliance {
        resources::strategy::ALLIANCE_COMMAND_CENTER_SHELL
    } else {
        resources::strategy::EMPIRE_COMMAND_CENTER_SHELL
    };
    draw_original_bitmap(
        cache,
        DllSource::Strategy,
        shell,
        canvas,
        0.0,
        0.0,
        Some(Rect::new(0.0, 0.0, 640.0, 480.0)),
    );
    draw_strategy_bitmap(
        cache,
        skin.frame,
        canvas,
        BATTLE_RESULTS_WINDOW_X,
        BATTLE_RESULTS_WINDOW_Y,
    );

    let (screen_mouse_x, screen_mouse_y) = mouse_position();
    let mouse = canvas.logical_pointer(screen_mouse_x, screen_mouse_y);
    let mouse_down = is_mouse_button_down(MouseButton::Left);
    let mouse_released = is_mouse_button_released(MouseButton::Left);

    let mut action = match state.battle_result_tab {
        BattleResultTab::Summary => {
            draw_battle_result_summary(cache, canvas, session, text_color);
            TacticalAction::None
        }
        BattleResultTab::AllianceForces => {
            draw_battle_result_force_details(
                cache,
                canvas,
                session,
                world,
                troop_transport,
                true,
                state.battle_result_category,
                text_color,
            );
            TacticalAction::None
        }
        BattleResultTab::ImperialForces => {
            draw_battle_result_force_details(
                cache,
                canvas,
                session,
                world,
                troop_transport,
                false,
                state.battle_result_category,
                text_color,
            );
            TacticalAction::None
        }
        BattleResultTab::GoDirectlyTo => draw_battle_result_destinations(
            cache,
            canvas,
            session,
            player_alliance,
            mouse,
            mouse_down,
            mouse_released,
            text_color,
        ),
    };

    draw_strategy_bitmap(
        cache,
        skin.rail,
        canvas,
        BATTLE_RESULTS_WINDOW_X + BATTLE_RESULTS_RAIL_X,
        BATTLE_RESULTS_WINDOW_Y,
    );
    let close_active = mouse_down
        && skin.close_rect.contains(
            mouse.0 - BATTLE_RESULTS_WINDOW_X,
            mouse.1 - BATTLE_RESULTS_WINDOW_Y,
        );
    draw_strategy_bitmap(
        cache,
        skin.close[usize::from(close_active)],
        canvas,
        BATTLE_RESULTS_WINDOW_X + skin.close_rect.x,
        BATTLE_RESULTS_WINDOW_Y + skin.close_rect.y,
    );
    if mouse_released
        && skin.close_rect.contains(
            mouse.0 - BATTLE_RESULTS_WINDOW_X,
            mouse.1 - BATTLE_RESULTS_WINDOW_Y,
        )
    {
        action = TacticalAction::ReturnToGalaxy;
    }

    for (index, rect) in skin.tab_rects.into_iter().enumerate() {
        let local_mouse = (
            mouse.0 - BATTLE_RESULTS_WINDOW_X,
            mouse.1 - BATTLE_RESULTS_WINDOW_Y,
        );
        let selected = state.battle_result_tab.index() == index;
        let active = selected || (mouse_down && rect.contains(local_mouse.0, local_mouse.1));
        draw_strategy_bitmap(
            cache,
            skin.tabs[index][usize::from(active)],
            canvas,
            BATTLE_RESULTS_WINDOW_X + rect.x,
            BATTLE_RESULTS_WINDOW_Y + rect.y,
        );
        if mouse_released && rect.contains(local_mouse.0, local_mouse.1) {
            state.battle_result_tab = BattleResultTab::from_index(index);
            macroquad::logging::info!(
                "[tactical-results] tab={} category={}",
                state.battle_result_tab.label(),
                state.battle_result_category.label()
            );
        }
    }

    if matches!(
        state.battle_result_tab,
        BattleResultTab::AllianceForces | BattleResultTab::ImperialForces
    ) && mouse_released
    {
        for (index, category) in BattleResultCategory::ALL.into_iter().enumerate() {
            let rect = NativeRect::new(
                BATTLE_RESULTS_WINDOW_X + 14.0 + index as f32 * 96.0,
                BATTLE_RESULTS_WINDOW_Y + 58.0,
                96.0,
                25.0,
            );
            if rect.contains(mouse.0, mouse.1) {
                state.battle_result_category = category;
                macroquad::logging::info!(
                    "[tactical-results] tab={} category={}",
                    state.battle_result_tab.label(),
                    category.label()
                );
            }
        }
    }
    action
}

/// Draw the tactical combat view (macroquad).
///
/// Call this when `GameMode::TacticalCombat`. Returns a `TacticalAction`
/// indicating whether the main loop should transition modes.
#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
#[expect(
    clippy::cast_precision_loss,
    reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
)]
///
/// # Panics
/// Panics if the active battle session disappears during drawing.
pub fn draw_tactical_view(
    state: &mut TacticalState,
    bmp_cache: &mut BmpCache,
    world: &GameWorld,
    troop_transport: &TroopTransportState,
) -> TacticalAction {
    if state.session.is_none() {
        return TacticalAction::ReturnToGalaxy;
    }

    let mut action = TacticalAction::None;

    // 0. Advance combat if in Combat phase (mutable borrow).
    {
        let session = state.session.as_mut().unwrap();
        if session.phase == BattlePhase::Combat && !session.paused {
            // Step at ~4 ticks per second (at 60fps), scaled by combat_speed.
            let dt = get_frame_time();
            session.advance_presentational_effects(dt);
            let ticks_per_sec = 4.0 * session.combat_speed as f32;
            session.step_accumulator += dt * ticks_per_sec;
            while session.step_accumulator >= 1.0 {
                session.step_accumulator -= 1.0;
                session.step();
                if session.phase == BattlePhase::Results {
                    break;
                }
            }
        }
    }

    if state
        .session
        .as_ref()
        .is_some_and(|session| session.phase == BattlePhase::Results)
    {
        return draw_original_battle_results(state, bmp_cache, world, troop_transport);
    }

    // 1. Paint the unscaled-source tactical shell first, then put the battle
    // only inside its original black aperture. Bitmap 1000 includes every
    // frame edge and the empty right-hand panel housing.
    let canvas = TacticalCanvas::new(screen_width(), screen_height());
    prewarm_tactical_font_atlas(
        state.session.as_ref().unwrap(),
        canvas,
        &mut state.warmed_font_sizes,
    );
    clear_background(BLACK);
    draw_tactical_bitmap(bmp_cache, resources::tactical::BACKGROUND, canvas, 0.0, 0.0);
    set_tactical_aperture_clip(Some(canvas.aperture()));
    let aperture = canvas.aperture();
    let aperture_tuple = (aperture.x, aperture.y, aperture.width, aperture.height);
    update_tactical_camera_follow(state);
    if state.render_original_starfield {
        state.asset_renderer.draw_backdrop(aperture_tuple);
    }
    if state.render_original_planet {
        state.asset_renderer.draw_planet(aperture_tuple);
    }

    // 2. Submit mapped production participants through their original 3D and
    // type-303 resource families. Unmapped or unavailable objects retain the
    // bounded 2D fallback below.
    let (scale, offset_x, offset_y) =
        canvas.arena_transform(state.zoom, state.camera_x, state.camera_y);
    let render_pyrotechnics = state.render_pyrotechnics;
    let (
        production_objects,
        production_fighters,
        production_fields,
        production_projectiles,
        production_effects,
    ) = if state.render_original_participants {
        state
            .session
            .as_ref()
            .map(|session| {
                let ships = session
                    .ships
                    .iter()
                    .enumerate()
                    .filter(|(_, ship)| ship.alive)
                    .filter_map(|(index, ship)| {
                        ship.tactical_resource.map(|resource| TacticalRenderObject {
                            object_id: u32::try_from(index).unwrap_or(u32::MAX).saturating_add(1),
                            resource_base: resource.mesh_resource_base,
                            position: ship.source_position.rendered(),
                        })
                    })
                    .collect::<Vec<_>>();
                let fighters = session
                    .fighters
                    .iter()
                    .enumerate()
                    .filter(|(_, fighter)| fighter.alive)
                    .filter_map(|(index, fighter)| {
                        fighter.tactical_resource.map(|resource| {
                            let player_side = fighter.is_attacker == session.player_is_attacker;
                            TacticalFighterRenderObject {
                                object_id: u32::try_from(index)
                                    .unwrap_or(u32::MAX)
                                    .saturating_add(1001),
                                close_resource_id: resource
                                    .grouped_close_resource(player_side, fighter.fighter_group),
                                far_resource_id: resource
                                    .grouped_far_resource(player_side, fighter.fighter_group),
                                indicator_resource_id: resource
                                    .grouped_indicator_resource(player_side, fighter.fighter_group),
                                position: fighter.source_position.rendered(),
                            }
                        })
                    })
                    .collect::<Vec<_>>();
                let mut effects = if render_pyrotechnics {
                    session
                        .impact_effects
                        .iter()
                        .filter_map(|effect| {
                            let ship = session.ships.get(effect.target)?;
                            let (source_width, source_height) = effect.sequence.source_size();
                            Some(TacticalEffectRenderObject {
                                object_id: u32::try_from(effect.target)
                                    .unwrap_or(u32::MAX)
                                    .saturating_add(1),
                                resource_id: effect.sequence.resource_base()
                                    + u32::from(effect.frame),
                                source_width,
                                source_height,
                                position: ship.source_position.rendered(),
                            })
                        })
                        .collect::<Vec<_>>()
                } else {
                    Vec::new()
                };
                if let Some(death_star) = session.death_star {
                    effects.push(TacticalEffectRenderObject {
                        object_id: 3001,
                        resource_id: death_star.resource_id(),
                        source_width: 128,
                        source_height: 128,
                        position: death_star.source_position.rendered(),
                    });
                }
                let fields = session
                    .field_effects
                    .iter()
                    .filter_map(|effect| {
                        let ship = session.ships.get(effect.target)?;
                        Some(TacticalEffectRenderObject {
                            object_id: u32::try_from(effect.target)
                                .unwrap_or(u32::MAX)
                                .saturating_add(1),
                            resource_id: effect.resource_id()?,
                            source_width: 128,
                            source_height: 128,
                            position: ship.source_position.rendered(),
                        })
                    })
                    .collect::<Vec<_>>();
                let mut projectiles = session
                    .weapon_effects
                    .iter()
                    .enumerate()
                    .map(|(index, effect)| TacticalProjectileRenderObject {
                        object_id: u32::try_from(index)
                            .unwrap_or(u32::MAX)
                            .saturating_add(2001),
                        origin: effect.source_position.rendered(),
                        target: effect.target_position.rendered(),
                        progress: effect.progress(),
                        longitudinal_scale: effect.longitudinal_scale,
                        shape_variant: effect.shape_variant,
                        color_selector: effect.color_selector,
                        color: effect.color(),
                    })
                    .collect::<Vec<_>>();
                if let Some(beam) = session.death_star_beam {
                    projectiles.push(TacticalProjectileRenderObject {
                        object_id: 3999,
                        origin: beam.source_position.rendered(),
                        target: beam.target_position.rendered(),
                        progress: (beam.elapsed / beam.duration).clamp(0.0, 1.0),
                        // The source projectile mesh collapses below one device pixel
                        // at the Death Star's tactical scale. Retain a visible authored-
                        // color beam segment while native dimensions remain unproven.
                        longitudinal_scale: 12.0,
                        shape_variant: 3,
                        color_selector: 1,
                        color: Color::new(0.25, 1.0, 0.25, 1.0),
                    });
                }
                (ships, fighters, fields, projectiles, effects)
            })
            .unwrap_or_default()
    } else {
        (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new())
    };
    let ship_report = state
        .asset_renderer
        .draw_participants(aperture_tuple, &production_objects);
    let fighter_report = state
        .asset_renderer
        .draw_fighters(aperture_tuple, &production_fighters);
    let projectile_report = state
        .asset_renderer
        .draw_projectiles(aperture_tuple, &production_projectiles);
    let _field_report = state
        .asset_renderer
        .draw_fields(aperture_tuple, &production_fields);
    let effect_report = state
        .asset_renderer
        .draw_effects(aperture_tuple, &production_effects);
    if let Some(session) = state.session.as_mut() {
        let mut formation_footprint_changed = false;
        for (object_id, envelope) in &ship_report.collision_envelopes {
            let Some(index) = object_id
                .checked_sub(1)
                .and_then(|index| usize::try_from(index).ok())
            else {
                continue;
            };
            if let Some(ship) = session.ships.get_mut(index) {
                let recovered = OriginalTacticalCollisionEnvelope {
                    vertical_radius: envelope.vertical_radius,
                    planar_diameter: envelope.planar_diameter,
                };
                if ship.source_collision_envelope != Some(recovered) {
                    ship.source_collision_envelope = Some(recovered);
                    formation_footprint_changed = true;
                }
            }
        }
        if formation_footprint_changed && session.combat_tick == 0 && session.paused {
            BattleSession::refresh_original_task_force_geometry(&mut session.ships);
        }
    }
    let rendered_ship_indexes = ship_report
        .rendered_object_ids
        .into_iter()
        .filter_map(|object_id| usize::try_from(object_id.saturating_sub(1)).ok())
        .collect::<Vec<_>>();
    let rendered_fighter_indexes = fighter_report
        .rendered_object_ids
        .into_iter()
        .filter_map(|object_id| usize::try_from(object_id.saturating_sub(1001)).ok())
        .collect::<Vec<_>>();
    let ship_projections = ship_report
        .projections
        .into_iter()
        .filter_map(|projection| {
            usize::try_from(projection.object_id.saturating_sub(1))
                .ok()
                .map(|index| (index, projection))
        })
        .collect::<HashMap<_, _>>();
    let fighter_projections = fighter_report
        .projections
        .into_iter()
        .filter_map(|projection| {
            usize::try_from(projection.object_id.saturating_sub(1001))
                .ok()
                .map(|index| (index, projection))
        })
        .collect::<HashMap<_, _>>();
    let rendered_effect_targets = effect_report
        .rendered_object_ids
        .into_iter()
        .filter_map(|object_id| usize::try_from(object_id.saturating_sub(1)).ok())
        .collect::<HashSet<_>>();
    let rendered_projectiles = projectile_report
        .rendered_object_ids
        .into_iter()
        .filter_map(|object_id| usize::try_from(object_id.saturating_sub(2001)).ok())
        .collect::<HashSet<_>>();
    let navigation_projections = tactical_navigation_projections(state, aperture);
    if state.logged_navigation_scene != Some(state.navigation_sets_visible) {
        let points = navigation_projections
            .iter()
            .map(|projection| {
                format!(
                    "{}:{}:{:.3},{:.3}",
                    projection.set + 1,
                    projection.point + 1,
                    projection.screen.x,
                    projection.screen.y,
                )
            })
            .collect::<Vec<_>>()
            .join(";");
        macroquad::logging::info!(
            "[tactical_navigation] event=scene visible={:?} screen_positions={}",
            state.navigation_sets_visible,
            points,
        );
        state.logged_navigation_scene = Some(state.navigation_sets_visible);
    }

    #[cfg(feature = "interface-test-fixtures")]
    if state.proof_resource_2560 {
        if state.proof_lod_follows_zoom {
            state
                .asset_renderer
                .set_view(TacticalLodView::from_fixture_zoom(state.zoom));
        }
        let aperture = canvas.aperture();
        state
            .asset_renderer
            .draw_proof((aperture.x, aperture.y, aperture.width, aperture.height));
    }

    // Borrow session for rendering (immutable reads).
    let session = state.session.as_ref().unwrap();
    let phase = session.phase;
    let player_is_attacker = session.player_is_attacker;
    let combat_tick = session.combat_tick;
    let paused = session.paused;
    draw_tactical_navigation_points(session, &navigation_projections, canvas.scale);

    // 3. Draw deployment zones (placement phase only).
    if phase == BattlePhase::Placement {
        let zone_w = ARENA_WIDTH * DEPLOY_ZONE_FRACTION * scale;
        draw_rectangle(
            offset_x,
            offset_y,
            zone_w,
            ARENA_HEIGHT * scale,
            Color::new(0.0, 0.15, 0.3, 0.15),
        );
        draw_rectangle_lines(
            offset_x,
            offset_y,
            zone_w,
            ARENA_HEIGHT * scale,
            2.0,
            Color::new(0.0, 0.4, 0.8, 0.5),
        );

        let def_x = offset_x + ARENA_WIDTH * (1.0 - DEPLOY_ZONE_FRACTION) * scale;
        draw_rectangle(
            def_x,
            offset_y,
            zone_w,
            ARENA_HEIGHT * scale,
            Color::new(0.3, 0.05, 0.0, 0.15),
        );
        draw_rectangle_lines(
            def_x,
            offset_y,
            zone_w,
            ARENA_HEIGHT * scale,
            2.0,
            Color::new(0.8, 0.2, 0.0, 0.5),
        );
    }

    // 4. Draw ships (macroquad primitives).
    let aperture = canvas.aperture();
    for (ship_index, ship) in session.ships.iter().enumerate() {
        #[cfg(feature = "interface-test-fixtures")]
        let suppress_fixture_fallback =
            state.suppress_mapped_capital_fallback && ship.tactical_resource.is_some();
        #[cfg(not(feature = "interface-test-fixtures"))]
        let suppress_fixture_fallback = false;
        if !ship.alive || rendered_ship_indexes.contains(&ship_index) || suppress_fixture_fallback {
            continue;
        }
        let sx = offset_x + ship.x * scale;
        let sy = offset_y + ship.y * scale;
        if !aperture.contains(sx, sy) {
            continue;
        }
        let size = DEFAULT_SHIP_SIZE * scale;

        let color = if ship.is_attacker {
            if ship.selected {
                Color::new(0.3, 1.0, 0.3, 0.9)
            } else {
                Color::new(0.2, 0.6, 1.0, 0.8)
            }
        } else {
            Color::new(1.0, 0.3, 0.2, 0.8)
        };

        let half = size / 2.0;
        draw_triangle(
            macroquad::math::Vec2::new(sx, sy - half),
            macroquad::math::Vec2::new(sx - half * 0.6, sy),
            macroquad::math::Vec2::new(sx + half * 0.6, sy),
            color,
        );
        draw_triangle(
            macroquad::math::Vec2::new(sx - half * 0.6, sy),
            macroquad::math::Vec2::new(sx + half * 0.6, sy),
            macroquad::math::Vec2::new(sx, sy + half),
            color,
        );

        let is_alliance = ship.is_attacker == session.attacker_is_alliance;
        let highlighted = if is_alliance {
            state.highlight_alliance
        } else {
            state.highlight_empire
        };
        if highlighted {
            let wire_color = if is_alliance {
                Color::new(0.9, 0.15, 0.12, 0.85)
            } else {
                Color::new(0.1, 0.9, 0.2, 0.85)
            };
            draw_rectangle_lines(
                sx - half * 0.85,
                sy - half * 0.85,
                size * 0.85,
                size * 0.85,
                1.0,
                wire_color,
            );
        }

        let bar_w = size * 0.8;
        let bar_h = 4.0;
        let bar_x = sx - bar_w / 2.0;
        let bar_y = sy + half + 4.0;
        let health_frac = if ship.hull_max > 0 {
            ship.hull_current as f32 / ship.hull_max as f32
        } else {
            0.0
        };
        draw_rectangle(bar_x, bar_y, bar_w, bar_h, Color::new(0.3, 0.0, 0.0, 0.8));
        let health_color = if health_frac > 0.5 {
            Color::new(0.0, 0.8, 0.0, 0.9)
        } else if health_frac > 0.25 {
            Color::new(0.9, 0.7, 0.0, 0.9)
        } else {
            Color::new(0.9, 0.1, 0.0, 0.9)
        };
        draw_rectangle(bar_x, bar_y, bar_w * health_frac, bar_h, health_color);

        if ship.shield_max > 0 {
            let shield_frac = ship.shield as f32 / ship.shield_max as f32;
            let sbar_y = sy - half - 8.0;
            draw_rectangle(bar_x, sbar_y, bar_w, bar_h, Color::new(0.0, 0.0, 0.3, 0.6));
            draw_rectangle(
                bar_x,
                sbar_y,
                bar_w * shield_frac,
                bar_h,
                Color::new(0.3, 0.5, 1.0, 0.8),
            );
        }

        #[expect(
            clippy::manual_clamp,
            reason = "min/max map NaN to the lower bound; clamp would propagate NaN."
        )]
        let font_size = (12.0 * scale).max(8.0).min(14.0) as u16;
        let label = &ship.name;
        let dims = measure_text(label, None, font_size, 1.0);
        draw_text(
            label,
            sx - dims.width / 2.0,
            bar_y + bar_h + dims.height + 2.0,
            f32::from(font_size),
            WHITE,
        );

        // Focus-fire reticle: draw targeting brackets if this ship is being targeted.
        if ship.attack_target.is_some() && ship.is_attacker == player_is_attacker {
            // Small arrow/marker near ship indicating it has orders.
            draw_circle_lines(sx, sy, half + 4.0, 1.5, Color::new(1.0, 0.6, 0.0, 0.6));
        }

        // Retreat indicator: pulsing "R" above ship.
        if ship.retreating {
            let retreat_alpha = 0.5 + 0.5 * (combat_tick as f32 * 0.3).sin().abs();
            draw_text(
                "RETREAT",
                sx - 20.0,
                sy - half - 14.0,
                10.0,
                Color::new(1.0, 0.5, 0.0, retreat_alpha),
            );
        }
    }

    // The retained 3D submission and the interaction layer share these exact
    // projected bounds. This replaces the obsolete arena-coordinate frame.
    for (ship_index, projection) in &ship_projections {
        let Some(ship) = session.ships.get(*ship_index) else {
            continue;
        };
        if ship.alive && ship.selected {
            draw_projected_selection_frame(*projection, canvas.scale);
        }
    }

    // 5. Draw fighter squadrons.
    for (fighter_index, fighter) in session.fighters.iter().enumerate() {
        #[cfg(feature = "interface-test-fixtures")]
        let suppress_fixture_fallback =
            state.suppress_mapped_fighter_fallback && fighter.tactical_resource.is_some();
        #[cfg(not(feature = "interface-test-fixtures"))]
        let suppress_fixture_fallback = false;
        if !fighter.alive
            || rendered_fighter_indexes.contains(&fighter_index)
            || suppress_fixture_fallback
        {
            continue;
        }
        let fx = offset_x + fighter.x * scale;
        let fy = offset_y + fighter.y * scale;
        if !aperture.contains(fx, fy) {
            continue;
        }
        let size = FIGHTER_SIZE * scale;

        let color = if fighter.is_attacker {
            Color::new(0.4, 0.8, 1.0, 0.7)
        } else {
            Color::new(1.0, 0.5, 0.3, 0.7)
        };

        let half = size / 2.0;
        draw_triangle(
            macroquad::math::Vec2::new(fx, fy - half),
            macroquad::math::Vec2::new(fx - half, fy + half),
            macroquad::math::Vec2::new(fx + half, fy + half),
            color,
        );

        let label = format!("x{}", fighter.squad_count);
        #[expect(
            clippy::manual_clamp,
            reason = "min/max map NaN to the lower bound; clamp would propagate NaN."
        )]
        let font_size = (10.0 * scale).max(7.0).min(12.0) as u16;
        draw_text(
            &label,
            fx + half + 2.0,
            fy + 4.0,
            f32::from(font_size),
            color,
        );
    }

    for (fighter_index, projection) in &fighter_projections {
        let Some(fighter) = session.fighters.get(*fighter_index) else {
            continue;
        };
        if fighter.alive && fighter.selected {
            draw_projected_selection_frame(*projection, canvas.scale);
        }
    }

    // 6. The normal path above submits the executable's retained projectile
    // meshes. Keep this bounded 2D fallback only when the source camera cannot
    // render a projectile.
    for (effect_index, effect) in session.weapon_effects.iter().enumerate() {
        let source = macroquad::math::Vec2::new(
            offset_x + effect.fallback_source[0] * scale,
            offset_y + effect.fallback_source[1] * scale,
        );
        let target = session.ships.get(effect.target).map_or_else(
            || {
                macroquad::math::Vec2::new(
                    offset_x + effect.fallback_target[0] * scale,
                    offset_y + effect.fallback_target[1] * scale,
                )
            },
            |ship| {
                tactical_ship_screen_position(
                    effect.target,
                    ship,
                    &ship_projections,
                    scale,
                    offset_x,
                    offset_y,
                )
            },
        );

        let alpha = (1.0 - effect.progress()).clamp(0.0, 1.0);
        let base_color = effect.color();
        let color = Color::new(base_color.r, base_color.g, base_color.b, alpha);
        if !rendered_projectiles.contains(&effect_index) {
            let current = source.lerp(target, effect.progress());
            let travel = target - source;
            let direction = if travel.length_squared() > f32::EPSILON {
                travel.normalize()
            } else {
                macroquad::math::Vec2::X
            };
            let tail = current - direction * (2.0 * canvas.scale).max(1.0);
            draw_line(
                tail.x,
                tail.y,
                current.x,
                current.y,
                canvas.scale.max(1.0),
                color,
            );
        }

        // Retain the bounded fallback only when an authentic target frame
        // was unavailable for this event.
        if effect.progress() < 0.375 && !rendered_effect_targets.contains(&effect.target) {
            let flash_r = 4.0 + effect.progress() * 16.0;
            draw_circle(
                target.x,
                target.y,
                flash_r,
                Color::new(1.0, 1.0, 0.8, alpha * 0.6),
            );
        }
    }

    // 6b. Draw targeting lines for both typed source attack targets.
    if phase == BattlePhase::Combat {
        for (ship_index, ship) in session.ships.iter().enumerate() {
            if !ship.alive || !ship.selected || ship.is_attacker != player_is_attacker {
                continue;
            }
            let source = tactical_ship_screen_position(
                ship_index,
                ship,
                &ship_projections,
                scale,
                offset_x,
                offset_y,
            );
            let targets = if ship.manual_targets.is_empty() {
                ship.attack_target.into_iter().collect::<Vec<_>>()
            } else {
                ship.manual_targets.clone()
            };
            for target_spec in targets {
                match target_spec {
                    TacticalAttackTarget::CapitalShip(target_index) => {
                        let Some(target_ship) = session.ships.get(target_index) else {
                            continue;
                        };
                        if !target_ship.alive {
                            continue;
                        }
                        let target = tactical_ship_screen_position(
                            target_index,
                            target_ship,
                            &ship_projections,
                            scale,
                            offset_x,
                            offset_y,
                        );
                        let radius = ship_projections.get(&target_index).map_or(
                            DEFAULT_SHIP_SIZE * scale * 0.4,
                            |projection| {
                                (projection.max - projection.min).max_element() * 0.6 + 3.0
                            },
                        );
                        draw_tactical_attack_target(source, target, radius);
                    }
                    TacticalAttackTarget::FighterGroup(target_index) => {
                        let Some(target_fighter) = session.fighters.get(target_index) else {
                            continue;
                        };
                        if !target_fighter.alive {
                            continue;
                        }
                        let target = tactical_fighter_screen_position(
                            target_index,
                            target_fighter,
                            &fighter_projections,
                            scale,
                            offset_x,
                            offset_y,
                        );
                        let radius = fighter_projections.get(&target_index).map_or(
                            FIGHTER_SIZE * scale * 0.75,
                            |projection| {
                                (projection.max - projection.min).max_element() * 0.6 + 3.0
                            },
                        );
                        draw_tactical_attack_target(source, target, radius);
                    }
                }
            }
        }

        for (fighter_index, fighter) in session.fighters.iter().enumerate() {
            if !fighter.alive || !fighter.selected || fighter.is_attacker != player_is_attacker {
                continue;
            }
            let source = tactical_fighter_screen_position(
                fighter_index,
                fighter,
                &fighter_projections,
                scale,
                offset_x,
                offset_y,
            );
            match fighter.attack_target {
                Some(TacticalAttackTarget::CapitalShip(target_index)) => {
                    let Some(target_ship) = session.ships.get(target_index) else {
                        continue;
                    };
                    if !target_ship.alive {
                        continue;
                    }
                    let target = tactical_ship_screen_position(
                        target_index,
                        target_ship,
                        &ship_projections,
                        scale,
                        offset_x,
                        offset_y,
                    );
                    let radius = ship_projections
                        .get(&target_index)
                        .map_or(DEFAULT_SHIP_SIZE * scale * 0.4, |projection| {
                            (projection.max - projection.min).max_element() * 0.6 + 3.0
                        });
                    draw_tactical_attack_target(source, target, radius);
                }
                Some(TacticalAttackTarget::FighterGroup(target_index)) => {
                    let Some(target_fighter) = session.fighters.get(target_index) else {
                        continue;
                    };
                    if !target_fighter.alive {
                        continue;
                    }
                    let target = tactical_fighter_screen_position(
                        target_index,
                        target_fighter,
                        &fighter_projections,
                        scale,
                        offset_x,
                        offset_y,
                    );
                    let radius = fighter_projections
                        .get(&target_index)
                        .map_or(FIGHTER_SIZE * scale * 0.75, |projection| {
                            (projection.max - projection.min).max_element() * 0.6 + 3.0
                        });
                    draw_tactical_attack_target(source, target, radius);
                }
                None => {}
            }
        }
    }

    // End immutable borrow before mutable operations.
    let _ = session;

    // Restore full-canvas painting before HUD controls and any later egui pass.
    set_tactical_aperture_clip(None);

    // 6. Handle input per phase.
    if phase == BattlePhase::Placement {
        let session = state.session.as_mut().unwrap();
        handle_placement_input(
            session,
            &mut state.dragging_ship,
            &mut state.drag_offset,
            scale,
            offset_x,
            offset_y,
        );
    } else if phase == BattlePhase::Combat {
        let session = state.session.as_mut().unwrap();
        handle_combat_input(
            session,
            &mut state.death_star_targeting,
            scale,
            offset_x,
            offset_y,
            canvas.aperture(),
            &ship_projections,
            &navigation_projections,
        );
    }

    if phase == BattlePhase::Combat {
        action = handle_original_tactical_controls(state, bmp_cache, canvas);
        let pressed_control = pressed_tactical_hud_control(bmp_cache, canvas);
        let player_is_alliance = state
            .session
            .as_ref()
            .is_some_and(|session| session.player_is_attacker == session.attacker_is_alliance);
        let groups = TacticalGroupHud::from_session(state.session.as_ref().unwrap());
        let death_star = state.session.as_ref().and_then(player_death_star);
        let has_selection =
            groups.selected_fighter_group.is_some() || groups.selected_ship_name.is_some();
        let attack_death_star_enabled = groups.selected_fighter_group.is_some()
            && state.session.as_ref().is_some_and(|session| {
                session.death_star.is_some_and(|death_star| {
                    death_star.operational() && death_star.is_attacker != session.player_is_attacker
                })
            });
        let withdraw_enabled = state
            .session
            .as_ref()
            .is_some_and(tactical_withdraw_enabled);
        let pressed_options = state
            .battle_options_open
            .then(|| pressed_tactical_battle_options_control(bmp_cache, canvas, withdraw_enabled))
            .flatten()
            .filter(|control| state.battle_options_pressed == Some(*control));
        let pressed_withdraw_confirmation = state
            .withdraw_confirmation_open
            .then(|| pressed_tactical_withdraw_confirmation_control(bmp_cache, canvas));
        let pressed_command = (!state.battle_options_open
            && !state.withdraw_confirmation_open
            && !state.player_observing)
            .then(|| {
                pressed_tactical_command_control(
                    bmp_cache,
                    canvas,
                    state.command_panel,
                    has_selection,
                    groups.selected_fighter_group.is_some(),
                    attack_death_star_enabled,
                    player_is_alliance,
                )
            });
        draw_original_tactical_hud(
            bmp_cache,
            canvas,
            player_is_alliance,
            groups,
            attack_death_star_enabled,
            paused,
            state.highlight_alliance,
            state.highlight_empire,
            state.navigation_sets_visible,
            pressed_control,
            state.command_panel,
            pressed_command.flatten(),
            state.battle_options_open,
            state.withdraw_confirmation_open,
            state.player_observing,
            withdraw_enabled,
            pressed_options,
            pressed_withdraw_confirmation
                .flatten()
                .filter(|control| state.withdraw_confirmation_pressed == Some(*control)),
            death_star,
            state.death_star_laser_pressed,
            state.death_star_targeting,
        );
        if paused {
            let (x, y) = canvas.point(25.0, 48.0);
            draw_text("Battle Paused.", x, y, 16.0 * canvas.scale, RED);
        }
    }

    action
}

// ---------------------------------------------------------------------------
// Combat input handling
// ---------------------------------------------------------------------------

fn tactical_ship_screen_position(
    ship_index: usize,
    ship: &TacticalShip,
    projections: &HashMap<usize, TacticalScreenProjection>,
    scale: f32,
    offset_x: f32,
    offset_y: f32,
) -> macroquad::math::Vec2 {
    projections.get(&ship_index).map_or_else(
        || macroquad::math::Vec2::new(offset_x + ship.x * scale, offset_y + ship.y * scale),
        |projection| projection.center,
    )
}

fn tactical_fighter_screen_position(
    fighter_index: usize,
    fighter: &TacticalFighter,
    projections: &HashMap<usize, TacticalScreenProjection>,
    scale: f32,
    offset_x: f32,
    offset_y: f32,
) -> macroquad::math::Vec2 {
    projections.get(&fighter_index).map_or_else(
        || macroquad::math::Vec2::new(offset_x + fighter.x * scale, offset_y + fighter.y * scale),
        |projection| projection.center,
    )
}

#[derive(Debug, Clone, Copy)]
struct TacticalNavigationProjection {
    set: u8,
    point: u8,
    position: TacticalWorldPosition,
    screen: macroquad::math::Vec2,
}

fn tactical_navigation_projections(
    state: &TacticalState,
    aperture: NativeRect,
) -> Vec<TacticalNavigationProjection> {
    let Some(session) = state.session.as_ref() else {
        return Vec::new();
    };
    let aperture = (aperture.x, aperture.y, aperture.width, aperture.height);
    session
        .navigation_sets
        .iter()
        .enumerate()
        .filter(|(set, _)| state.navigation_sets_visible[*set])
        .flat_map(|(set, points)| {
            points
                .iter()
                .enumerate()
                .filter_map(move |(point, position)| {
                    state
                        .asset_renderer
                        .project_source_position(aperture, position.rendered())
                        .map(|screen| TacticalNavigationProjection {
                            set: u8::try_from(set).unwrap_or(3),
                            point: u8::try_from(point).unwrap_or(u8::MAX),
                            position: *position,
                            screen,
                        })
                })
        })
        .collect()
}

fn draw_tactical_navigation_points(
    session: &BattleSession,
    projections: &[TacticalNavigationProjection],
    canvas_scale: f32,
) {
    let colors = [RED, GREEN, BLUE, YELLOW];
    let selected_routes = session
        .ships
        .iter()
        .filter(|ship| ship.selected && ship.is_attacker == session.player_is_attacker)
        .flat_map(|ship| ship.navigation_route.iter().copied())
        .collect::<Vec<_>>();
    for projection in projections {
        let assigned = selected_routes.contains(&projection.position);
        let color = colors[usize::from(projection.set.min(3))];
        let radius = if assigned { 7.0 } else { 5.0 } * canvas_scale;
        let top = vec2(projection.screen.x, projection.screen.y - radius);
        let left = vec2(
            projection.screen.x - radius,
            projection.screen.y + radius * 0.75,
        );
        let right = vec2(
            projection.screen.x + radius,
            projection.screen.y + radius * 0.75,
        );
        draw_triangle(top, left, right, color);
        draw_triangle_lines(top, left, right, canvas_scale.max(1.0), WHITE);
        if assigned {
            draw_circle_lines(
                projection.screen.x,
                projection.screen.y,
                radius + 2.0 * canvas_scale,
                canvas_scale.max(1.0),
                WHITE,
            );
        }
    }
}

fn draw_tactical_attack_target(
    source: macroquad::math::Vec2,
    target: macroquad::math::Vec2,
    radius: f32,
) {
    draw_line(
        source.x,
        source.y,
        target.x,
        target.y,
        1.0,
        Color::new(1.0, 0.5, 0.0, 0.4),
    );
    draw_circle_lines(
        target.x,
        target.y,
        radius,
        1.5,
        Color::new(1.0, 0.3, 0.0, 0.7),
    );
}

fn draw_projected_selection_frame(projection: TacticalScreenProjection, canvas_scale: f32) {
    let padding = 3.0 * canvas_scale;
    let minimum_size = 10.0 * canvas_scale;
    let minimum_half = macroquad::math::Vec2::splat(minimum_size * 0.5);
    let origin = (projection.min - macroquad::math::Vec2::splat(padding))
        .min(projection.center - minimum_half);
    let max = (projection.max + macroquad::math::Vec2::splat(padding))
        .max(projection.center + minimum_half);
    let size = max - origin;
    let corner = (size.min_element() * 0.28).clamp(3.0 * canvas_scale, 8.0 * canvas_scale);
    let color = Color::new(1.0, 0.82, 0.12, 0.95);
    let thickness = canvas_scale.max(1.0);
    for (x, x_direction) in [(origin.x, 1.0), (origin.x + size.x, -1.0)] {
        for (y, y_direction) in [(origin.y, 1.0), (origin.y + size.y, -1.0)] {
            draw_line(x, y, x + corner * x_direction, y, thickness, color);
            draw_line(x, y, x, y + corner * y_direction, thickness, color);
        }
    }
}

/// Handle mouse input during the combat phase.
///
/// - Left-click: select a player ship.
/// - Right-click on enemy: issue focus-fire order to all selected player ships.
/// - R key: retreat selected ships.
fn handle_combat_input(
    session: &mut BattleSession,
    death_star_targeting: &mut bool,
    scale: f32,
    offset_x: f32,
    offset_y: f32,
    aperture: NativeRect,
    projections: &HashMap<usize, TacticalScreenProjection>,
    navigation_projections: &[TacticalNavigationProjection],
) {
    let (mx, my) = mouse_position();
    let inside_battle = aperture.contains(mx, my);
    let arena_pointer_x = (mx - offset_x) / scale;
    let arena_pointer_y = (my - offset_y) / scale;
    let hit_radius = DEFAULT_SHIP_SIZE * 0.6;
    let screen_pointer = macroquad::math::Vec2::new(mx, my);
    let projected_hit_radius = 7.0 * (aperture.width / BATTLE_APERTURE.width);
    let navigation_hit = navigation_projections.iter().find(|projection| {
        projection.screen.distance(screen_pointer) <= 8.0 * (aperture.width / BATTLE_APERTURE.width)
    });
    let control = is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl);

    if inside_battle && is_mouse_button_pressed(MouseButton::Left) && control {
        if let Some(navigation) = navigation_hit {
            let mut changed = 0;
            for ship in &mut session.ships {
                if !ship.selected || !ship.alive || ship.is_attacker != session.player_is_attacker {
                    continue;
                }
                let before = ship.navigation_route.len();
                ship.navigation_route
                    .retain(|point| *point != navigation.position);
                if before != ship.navigation_route.len() {
                    changed += 1;
                    ship.source_waypoint = ship.navigation_route.first().copied();
                    ship.source_desired_forward = ship
                        .source_waypoint
                        .map_or(TacticalWorldVector::ZERO, |waypoint| {
                            source_delta(waypoint, ship.source_position).normalized()
                        });
                }
            }
            macroquad::logging::info!(
                "[tactical_navigation] event=route_remove set={} point={} members={}",
                navigation.set + 1,
                navigation.point + 1,
                changed,
            );
            return;
        }
    }

    // Left-click: select player's ship.
    if inside_battle && is_mouse_button_pressed(MouseButton::Left) {
        let mut hit = None;
        for (i, ship) in session.ships.iter().enumerate() {
            if !ship.alive || ship.retreating {
                continue;
            }
            if ship.is_attacker != session.player_is_attacker {
                continue;
            }
            let projected_hit = projections.get(&i).is_some_and(|projection| {
                projection.contains(screen_pointer, projected_hit_radius)
            });
            let dx = arena_pointer_x - ship.x;
            let dy = arena_pointer_y - ship.y;
            if projected_hit
                || (!projections.contains_key(&i) && dx * dx + dy * dy < hit_radius * hit_radius)
            {
                hit = Some(i);
                break;
            }
        }

        if let Some(idx) = hit {
            // The original manual uses Ctrl to build a multi-ship selection
            // before assigning those ships to a task force.
            if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
                session.ships[idx].selected = !session.ships[idx].selected;
            } else {
                for s in &mut session.ships {
                    s.selected = false;
                }
                session.ships[idx].selected = true;
            }
            session.selected_ship = Some(idx);
            session.selected_fighter_group = None;
            for fighter in &mut session.fighters {
                fighter.selected = false;
            }
            macroquad::logging::info!(
                "[tactical_3d] selection object_id={} source_projection={}",
                idx + 1,
                projections.contains_key(&idx)
            );
        } else {
            // Clicked empty space — deselect all.
            for s in &mut session.ships {
                s.selected = false;
            }
            session.selected_ship = None;
            session.selected_fighter_group = None;
            for fighter in &mut session.fighters {
                fighter.selected = false;
            }
        }
    }

    // Right-click: assign an armed Death Star superlaser or issue focus fire.
    if inside_battle && is_mouse_button_pressed(MouseButton::Right) {
        if let Some(navigation) = navigation_hit {
            let mut assigned = 0;
            let mut longest_route = 0;
            for ship in &mut session.ships {
                if !ship.selected || !ship.alive || ship.is_attacker != session.player_is_attacker {
                    continue;
                }
                if control {
                    if !ship.navigation_route.contains(&navigation.position) {
                        ship.navigation_route.push(navigation.position);
                    }
                } else {
                    ship.navigation_route.clear();
                    ship.navigation_route.push(navigation.position);
                }
                ship.source_waypoint = ship.navigation_route.first().copied();
                ship.source_desired_forward = ship
                    .source_waypoint
                    .map_or(TacticalWorldVector::ZERO, |waypoint| {
                        source_delta(waypoint, ship.source_position).normalized()
                    });
                assigned += 1;
                longest_route = longest_route.max(ship.navigation_route.len());
            }
            macroquad::logging::info!(
                "[tactical_navigation] event=route_assign set={} point={} append={} members={} route_len={}",
                navigation.set + 1,
                navigation.point + 1,
                control,
                assigned,
                longest_route,
            );
            return;
        }
        let mut target_hit = None;
        for (i, ship) in session.ships.iter().enumerate() {
            if !ship.alive {
                continue;
            }
            // Can only target enemy ships.
            if ship.is_attacker == session.player_is_attacker {
                continue;
            }
            let projected_hit = projections.get(&i).is_some_and(|projection| {
                projection.contains(screen_pointer, projected_hit_radius)
            });
            let dx = arena_pointer_x - ship.x;
            let dy = arena_pointer_y - ship.y;
            if projected_hit
                || (!projections.contains_key(&i) && dx * dx + dy * dy < hit_radius * hit_radius)
            {
                target_hit = Some(i);
                break;
            }
        }

        if *death_star_targeting {
            if let Some(target_idx) = target_hit {
                if session.commit_death_star_shot(target_idx) {
                    *death_star_targeting = false;
                }
            }
            return;
        }

        if let Some(target_idx) = target_hit {
            // Assign one or several ordered focus targets to selected ships.
            let mut assigned = Vec::new();
            for ship in &mut session.ships {
                if ship.selected && ship.is_attacker == session.player_is_attacker && ship.alive {
                    let target = TacticalAttackTarget::CapitalShip(target_idx);
                    if control {
                        if let Some(position) = ship
                            .manual_targets
                            .iter()
                            .position(|existing| *existing == target)
                        {
                            ship.manual_targets.remove(position);
                        } else {
                            ship.manual_targets.push(target);
                        }
                    } else {
                        ship.manual_targets.clear();
                        ship.manual_targets.push(target);
                    }
                    ship.attack_target = ship.manual_targets.first().copied();
                }
            }
            for (index, ship) in session.ships.iter().enumerate() {
                if ship.selected
                    && ship.attack_target == Some(TacticalAttackTarget::CapitalShip(target_idx))
                {
                    assigned.push(index + 1);
                }
            }
            macroquad::logging::info!(
                "[tactical_3d] focus source_object_ids={} target_object_id={} source_projection={}",
                assigned
                    .iter()
                    .map(usize::to_string)
                    .collect::<Vec<_>>()
                    .join(","),
                target_idx + 1,
                projections.contains_key(&target_idx)
                    && assigned
                        .iter()
                        .all(|object_id| projections.contains_key(&(*object_id - 1)))
            );
            let maximum_targets = session
                .ships
                .iter()
                .filter(|ship| ship.selected)
                .map(|ship| ship.manual_targets.len())
                .max()
                .unwrap_or(0);
            macroquad::logging::info!(
                "[tactical_navigation] event=target_assign append={} target_object_id={} members={} target_count={}",
                control,
                target_idx + 1,
                assigned.len(),
                maximum_targets,
            );
        } else {
            // Right-clicked empty space — clear focus targets for selected ships.
            for ship in &mut session.ships {
                if ship.selected && ship.is_attacker == session.player_is_attacker {
                    ship.attack_target = None;
                    ship.manual_targets.clear();
                }
            }
        }
    }

    // R key: retreat selected ships.
    if is_key_pressed(KeyCode::R) {
        for ship in &mut session.ships {
            if ship.selected && ship.is_attacker == session.player_is_attacker && ship.alive {
                ship.retreating = true;
                ship.selected = false;
            }
        }
        session.selected_ship = None;
        session.selected_fighter_group = None;
    }
}

// ---------------------------------------------------------------------------
// Placement input handling
// ---------------------------------------------------------------------------

fn handle_placement_input(
    session: &mut BattleSession,
    dragging_ship: &mut Option<usize>,
    drag_offset: &mut (f32, f32),
    scale: f32,
    offset_x: f32,
    offset_y: f32,
) {
    let (mx, my) = mouse_position();

    // Convert mouse to arena coordinates.
    let arena_pointer_x = (mx - offset_x) / scale;
    let arena_pointer_y = (my - offset_y) / scale;

    // Deployment zone bounds for the player's side.
    let (zone_min_x, zone_max_x) = if session.player_is_attacker {
        (0.0, ARENA_WIDTH * DEPLOY_ZONE_FRACTION)
    } else {
        (ARENA_WIDTH * (1.0 - DEPLOY_ZONE_FRACTION), ARENA_WIDTH)
    };

    if is_mouse_button_pressed(MouseButton::Left) {
        // Check if clicking on a player's ship.
        let mut hit = None;
        for (i, ship) in session.ships.iter().enumerate() {
            if !ship.alive {
                continue;
            }
            if ship.is_attacker != session.player_is_attacker {
                continue;
            }
            let dx = arena_pointer_x - ship.x;
            let dy = arena_pointer_y - ship.y;
            if dx * dx + dy * dy < (DEFAULT_SHIP_SIZE * 0.6).powi(2) {
                hit = Some(i);
                break;
            }
        }

        if let Some(idx) = hit {
            *dragging_ship = Some(idx);
            *drag_offset = (
                session.ships[idx].x - arena_pointer_x,
                session.ships[idx].y - arena_pointer_y,
            );
            // Select this ship.
            for s in &mut session.ships {
                s.selected = false;
            }
            session.ships[idx].selected = true;
            session.selected_ship = Some(idx);
        } else {
            // Deselect.
            for s in &mut session.ships {
                s.selected = false;
            }
            session.selected_ship = None;
        }
    }

    if is_mouse_button_down(MouseButton::Left) {
        if let Some(idx) = *dragging_ship {
            let new_x = (arena_pointer_x + drag_offset.0).clamp(
                zone_min_x + DEFAULT_SHIP_SIZE * 0.5,
                zone_max_x - DEFAULT_SHIP_SIZE * 0.5,
            );
            let new_y = (arena_pointer_y + drag_offset.1).clamp(
                DEFAULT_SHIP_SIZE * 0.5,
                ARENA_HEIGHT - DEFAULT_SHIP_SIZE * 0.5,
            );
            session.ships[idx].x = new_x;
            session.ships[idx].y = new_y;
        }
    }

    if is_mouse_button_released(MouseButton::Left) {
        *dragging_ship = None;
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn test_ship(dat_id: u32, roster: usize, is_alliance: bool, alive: bool) -> TacticalShip {
        TacticalShip {
            class_key: CapitalShipKey::default(),
            identity: TacticalObjectIdentity {
                class_dat_id: DatId::new(dat_id),
                fleet_roster_index: roster,
                is_alliance,
            },
            tactical_resource: capital_ship_tactical_resource(DatId::new(dat_id)),
            death_star_resource: death_star_tactical_resource(DatId::new(dat_id)),
            source_position: TacticalWorldPosition::ORIGIN,
            source_forward: if is_alliance {
                TacticalWorldVector::SOURCE_FORWARD
            } else {
                TacticalWorldVector {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                }
            },
            source_desired_forward: if is_alliance {
                TacticalWorldVector::SOURCE_FORWARD
            } else {
                TacticalWorldVector {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                }
            },
            source_velocity: TacticalWorldVector::ZERO,
            source_waypoint: None,
            navigation_route: Vec::new(),
            source_collision_envelope: None,
            name: format!("ship-{dat_id}"),
            x: 0.0,
            y: 0.0,
            hull_current: 1,
            hull_max: 1,
            shield: 0,
            shield_max: 0,
            subsystem_condition: TacticalSubsystemCondition {
                shields: 0,
                weapons: 0,
                tractor: 0,
                engines: 0,
                hyperdrive: 0,
            },
            subsystem_capacity: TacticalSubsystemCapacity {
                shields: false,
                weapons: false,
                tractor: false,
                engines: false,
                hyperdrive: 0,
            },
            subsystem_damage: TacticalSubsystemDamage::default(),
            sublight_engine_power: 100.0,
            maneuverability: 0,
            tractor_beam_power: 25.0,
            fighter_capacity: 1,
            maneuver_state_value: None,
            damage_control: 100,
            is_attacker: is_alliance,
            alive,
            selected: false,
            order: TacticalOrder::None,
            tactic: TacticalTactic::StandOff,
            task_force: u8::try_from(roster.min(7)).unwrap_or(7),
            fleet_ship_index: roster,
            sprite_id: None,
            weapon_arcs: [TacticalWeaponArc::default(); 4],
            weapon_ranges: TacticalWeaponRanges::default(),
            weapon_recharge_rate: 0.0,
            shield_recharge_rate: 0.0,
            shield_recharge_carry: 0.0,
            weapon_recharge_queue: Vec::new(),
            attack_target: None,
            manual_targets: Vec::new(),
            retreating: false,
            retreat_progress: 0.0,
            retreated: false,
        }
    }

    fn test_fighter(dat_id: u32, is_alliance: bool) -> TacticalFighter {
        TacticalFighter {
            class_key: FighterKey::default(),
            identity: TacticalObjectIdentity {
                class_dat_id: DatId::new(dat_id),
                fleet_roster_index: 0,
                is_alliance,
            },
            tactical_resource: fighter_tactical_resource(DatId::new(dat_id)),
            source_position: TacticalWorldPosition::ORIGIN,
            sublight_engine_power: 10.0,
            fleet_fighter_index: 0,
            fleet_squadron_index: 0,
            name: format!("fighter-{dat_id}"),
            x: 0.0,
            y: 0.0,
            squad_count: 12,
            hull_current: 96.0,
            hull_max: 96.0,
            shield: 5.0,
            shield_max: 5.0,
            weapon_arc: TacticalWeaponArc::new(5, 0, 0),
            weapon_ranges: TacticalWeaponRanges {
                laser_cannon: 12.0,
                ion_cannon: 0.0,
                turbolaser: 0.0,
            },
            torpedo_strength: 0,
            torpedo_range: 0.0,
            maneuverability: 5,
            is_attacker: is_alliance,
            alive: true,
            fighter_group: 0,
            selected: false,
            order: TacticalOrder::None,
            tactic: TacticalTactic::StandOff,
            attack_target: None,
            recovery_state: TacticalFighterRecoveryState::AwaitingCarrier,
            recovery_target: None,
        }
    }

    fn test_session(
        ships: Vec<TacticalShip>,
        fighters: Vec<TacticalFighter>,
        player_is_attacker: bool,
    ) -> BattleSession {
        BattleSession {
            system: SystemKey::default(),
            system_name: "Test System".to_string(),
            system_picture_id: 1,
            attacker_fleet: FleetKey::default(),
            defender_fleet: FleetKey::default(),
            attacker_is_alliance: true,
            player_is_attacker,
            phase: BattlePhase::Combat,
            ships,
            fighters,
            death_star: None,
            death_star_beam: None,
            trench_run_outcome: None,
            trench_run_cinematic_pending: false,
            navigation_sets: original_navigation_point_sets(OriginalTacticalLayout::default()),
            source_layout: OriginalTacticalLayout::default(),
            selected_ship: None,
            selected_fighter_group: None,
            placement_confirmed: true,
            start_tick: 0,
            combat_tick: 0,
            weapon_effects: Vec::new(),
            impact_effects: Vec::new(),
            field_effects: Vec::new(),
            subsystem_repairs: Vec::new(),
            paused: true,
            combat_speed: 1,
            step_accumulator: 0.0,
            winner: None,
        }
    }

    #[test]
    fn auto_resolve_enters_results_and_projects_losses_once() {
        let mut first_ship = test_ship(64, 1, true, true);
        first_ship.hull_current = 100;
        first_ship.hull_max = 100;
        let mut second_ship = test_ship(64, 2, true, true);
        second_ship.hull_current = 100;
        second_ship.hull_max = 100;
        let mut fighters = vec![test_fighter(1, true), test_fighter(1, true)];
        fighters[1].fleet_squadron_index = 1;
        let session = test_session(vec![first_ship, second_ship], fighters, true);
        let attacker_fleet = session.attacker_fleet;
        let defender_fleet = session.defender_fleet;
        let system = session.system;
        let mut state = TacticalState {
            session: Some(session),
            ..TacticalState::default()
        };
        let result = SpaceCombatResult {
            attacker_fleet,
            defender_fleet,
            system,
            winner: AutoResolveSide::Attacker,
            ship_damage: vec![rebellion_core::combat::ShipDamageEvent {
                fleet: attacker_fleet,
                ship_index: 0,
                hull_before: 100,
                hull_after: 0,
            }],
            fighter_losses: vec![rebellion_core::combat::FighterLossEvent {
                fleet: attacker_fleet,
                fighter_index: 0,
                squads_before: 2,
                squads_after: 1,
            }],
            tick: 0,
        };

        assert!(state.present_auto_resolve_result(&result));
        assert!(state.strategic_results_applied());
        let result_session = state.session.as_ref().unwrap();
        assert_eq!(result_session.phase, BattlePhase::Results);
        assert_eq!(result_session.winner, Some(CombatWinner::Attacker));
        assert!(!result_session.ships[0].alive);
        assert!(result_session.ships[1].alive);
        assert!(result_session.fighters[0].alive);
        assert!(!result_session.fighters[1].alive);
    }

    #[test]
    fn original_effect_dispatch_preserves_resources_sizes_and_priority_groups() {
        let cases = [
            (
                WeaponKind::LaserCannon,
                OriginalTacticalImpactStage::Hit,
                OriginalTacticalEffectSequence::StandardHit,
                3060,
                6,
                (32, 32),
            ),
            (
                WeaponKind::IonCannon,
                OriginalTacticalImpactStage::Hit,
                OriginalTacticalEffectSequence::IonHit,
                3180,
                6,
                (32, 32),
            ),
            (
                WeaponKind::LaserCannon,
                OriginalTacticalImpactStage::Damage,
                OriginalTacticalEffectSequence::StandardDamage,
                3120,
                7,
                (32, 32),
            ),
            (
                WeaponKind::IonCannon,
                OriginalTacticalImpactStage::Damage,
                OriginalTacticalEffectSequence::IonDamage,
                3240,
                16,
                (64, 32),
            ),
            (
                WeaponKind::Turbolaser,
                OriginalTacticalImpactStage::Damage,
                OriginalTacticalEffectSequence::TurbolaserDamage,
                3300,
                7,
                (32, 32),
            ),
            (
                WeaponKind::LaserCannon,
                OriginalTacticalImpactStage::Destroyed,
                OriginalTacticalEffectSequence::Destroyed,
                3360,
                16,
                (64, 64),
            ),
        ];
        for (weapon, stage, expected, base, frames, size) in cases {
            let actual = original_tactical_effect_sequence(weapon, stage);
            assert_eq!(actual, expected);
            assert_eq!(actual.resource_base(), base);
            assert_eq!(actual.frame_count(), frames);
            assert_eq!(actual.source_size(), size);
        }

        let mut effects = Vec::new();
        assert!(queue_original_tactical_impact(
            &mut effects,
            4,
            WeaponKind::IonCannon,
            OriginalTacticalImpactStage::Hit,
        ));
        assert!(!queue_original_tactical_impact(
            &mut effects,
            4,
            WeaponKind::LaserCannon,
            OriginalTacticalImpactStage::Hit,
        ));
        assert!(queue_original_tactical_impact(
            &mut effects,
            4,
            WeaponKind::LaserCannon,
            OriginalTacticalImpactStage::Damage,
        ));
        assert!(!queue_original_tactical_impact(
            &mut effects,
            4,
            WeaponKind::Turbolaser,
            OriginalTacticalImpactStage::Damage,
        ));
        assert!(queue_original_tactical_impact(
            &mut effects,
            4,
            WeaponKind::IonCannon,
            OriginalTacticalImpactStage::Destroyed,
        ));
        assert_eq!(
            effects[0].sequence,
            OriginalTacticalEffectSequence::Destroyed
        );
    }

    #[test]
    fn original_effect_frames_advance_at_ten_hertz_and_expire_exactly() {
        let mut session = test_session(Vec::new(), Vec::new(), true);
        queue_original_tactical_impact(
            &mut session.impact_effects,
            0,
            WeaponKind::LaserCannon,
            OriginalTacticalImpactStage::Hit,
        );
        session.advance_presentational_effects(0.099);
        assert_eq!(session.impact_effects[0].frame, 0);
        session.advance_presentational_effects(0.001_1);
        assert_eq!(session.impact_effects[0].frame, 1);
        session.advance_presentational_effects(0.5);
        assert!(session.impact_effects.is_empty());
    }

    #[test]
    fn original_projectile_profiles_preserve_thresholds_shapes_scales_colors_and_durations() {
        let cases = [
            (
                WeaponKind::LaserCannon,
                28.7,
                Some(0),
                true,
                (0.5, 1, 0, 1.0),
            ),
            (
                WeaponKind::LaserCannon,
                ORIGINAL_LASER_PROJECTILE_THRESHOLD,
                Some(0),
                true,
                (0.5, 2, 0, 1.0),
            ),
            (
                WeaponKind::Turbolaser,
                30.0,
                Some(15),
                false,
                (0.65, 1, 1, 1.0),
            ),
            (
                WeaponKind::Turbolaser,
                40.0,
                Some(14),
                false,
                (0.75, 2, 1, 1.0),
            ),
            (
                WeaponKind::Turbolaser,
                40.0,
                Some(15),
                false,
                (1.0, 3, 1, 2.0),
            ),
            (
                WeaponKind::IonCannon,
                ORIGINAL_ION_PROJECTILE_THRESHOLD,
                Some(0),
                true,
                (1.0, 3, 2, 2.0),
            ),
            (
                WeaponKind::FighterAttack,
                10.0,
                None,
                true,
                (0.2, 1, 0, 1.0),
            ),
        ];
        for (weapon, strength, ordinal, alliance, expected) in cases {
            let profile = original_projectile_profile(weapon, strength, ordinal, alliance);
            assert_eq!(
                (
                    profile.longitudinal_scale,
                    profile.shape_variant,
                    profile.color_selector,
                    profile.duration,
                ),
                expected
            );
        }
    }

    #[test]
    fn retained_projectile_interpolates_and_expires_by_source_duration() {
        let ships = vec![test_ship(64, 0, true, true), test_ship(128, 0, false, true)];
        let mut session = test_session(ships, Vec::new(), true);
        assert!(queue_original_tactical_projectile(
            &mut session.weapon_effects,
            &session.ships,
            0,
            1,
            WeaponKind::LaserCannon,
            ORIGINAL_LASER_PROJECTILE_THRESHOLD,
        ));
        assert_eq!(session.weapon_effects[0].shape_variant, 2);
        session.advance_presentational_effects(0.5);
        assert_eq!(session.weapon_effects[0].progress(), 0.5);
        session.advance_presentational_effects(0.499);
        assert_eq!(session.weapon_effects.len(), 1);
        session.advance_presentational_effects(0.002);
        assert!(session.weapon_effects.is_empty());
    }

    #[test]
    fn gravity_field_preempts_tractor_then_restores_it_at_ten_hertz() {
        let mut session = test_session(Vec::new(), Vec::new(), true);
        assert!(set_original_tactical_field(
            &mut session.field_effects,
            1,
            4,
            OriginalTacticalFieldKind::Tractor,
            true,
        ));
        assert_eq!(session.field_effects[0].resource_id(), Some(3620));
        session.advance_presentational_effects(0.1);
        assert_eq!(session.field_effects[0].resource_id(), Some(3621));
        assert!(set_original_tactical_field(
            &mut session.field_effects,
            2,
            4,
            OriginalTacticalFieldKind::Gravity,
            true,
        ));
        assert_eq!(session.field_effects[0].resource_id(), Some(3520));
        assert!(set_original_tactical_field(
            &mut session.field_effects,
            3,
            4,
            OriginalTacticalFieldKind::Tractor,
            true,
        ));
        assert_eq!(session.field_effects[0].source_counts(), (2, 1));
        assert!(set_original_tactical_field(
            &mut session.field_effects,
            2,
            4,
            OriginalTacticalFieldKind::Gravity,
            false,
        ));
        assert_eq!(session.field_effects[0].resource_id(), Some(3620));
        assert!(set_original_tactical_field(
            &mut session.field_effects,
            1,
            4,
            OriginalTacticalFieldKind::Tractor,
            false,
        ));
        assert!(set_original_tactical_field(
            &mut session.field_effects,
            3,
            4,
            OriginalTacticalFieldKind::Tractor,
            false,
        ));
        assert!(session.field_effects.is_empty());
    }

    #[test]
    fn exact_field_sources_enforce_one_tractor_and_four_gravity_targets() {
        let ships = (0..6)
            .map(|index| test_ship(64, index, true, true))
            .collect();
        let mut session = test_session(ships, Vec::new(), true);

        assert!(session.set_tractor_field(0, 1, true));
        assert!(!session.set_tractor_field(0, 1, true));
        assert!(!session.set_tractor_field(0, 2, true));
        assert!(!session.set_tractor_field(2, 1, false));
        assert!(session.set_tractor_field(0, 1, false));
        assert!(session.set_tractor_field(0, 2, true));

        for target in 1..=4 {
            assert!(session.set_gravity_field(0, target, true));
        }
        assert!(!session.set_gravity_field(0, 1, true));
        assert!(!session.set_gravity_field(0, 5, true));
        assert!(!session.set_gravity_field(1, 4, false));
        assert!(session.set_gravity_field(0, 4, false));
        assert!(session.set_gravity_field(0, 5, true));

        let target_two = session
            .field_effects
            .iter()
            .find(|effect| effect.target == 2)
            .expect("target two should retain both field kinds");
        assert_eq!(target_two.source_ids(), (&[0][..], &[0][..]));
        assert_eq!(target_two.resource_id(), Some(3520));
    }

    #[test]
    fn subsystem_condition_uses_exact_five_source_bands() {
        let condition = TacticalSubsystemCondition {
            shields: 0,
            weapons: 24,
            tractor: 25,
            engines: 50,
            hyperdrive: 75,
        };
        assert_eq!(condition.resource_ids(), [1201, 1207, 1213, 1219, 1225]);
        assert_eq!(original_tactical_condition_band(1), 1);
        assert_eq!(original_tactical_condition_band(49), 2);
        assert_eq!(original_tactical_condition_band(74), 3);
        assert_eq!(original_tactical_condition_band(100), 4);
    }

    #[test]
    fn subsystem_condition_follows_hull_hits_and_hyperdrive_capacity() {
        let condition = TacticalSubsystemCondition::from_source_state(
            980,
            1000,
            TacticalSubsystemCapacity {
                shields: true,
                weapons: true,
                tractor: true,
                engines: true,
                hyperdrive: 2,
            },
            TacticalSubsystemDamage {
                shields: 1,
                weapons: 2,
                tractor: 3,
                engines: 4,
                hyperdrive: 1,
            },
        );
        assert_eq!(condition.percentages(), [73, 48, 23, 0, 50]);
        assert_eq!(condition.resource_ids(), [1204, 1208, 1212, 1216, 1224]);

        let unavailable = TacticalSubsystemCondition::from_source_state(
            100,
            100,
            TacticalSubsystemCapacity {
                shields: false,
                weapons: false,
                tractor: false,
                engines: false,
                hyperdrive: 0,
            },
            TacticalSubsystemDamage::default(),
        );
        assert_eq!(unavailable.percentages(), [0; 5]);
    }

    #[test]
    fn subsystem_repair_selection_preserves_source_hit_order() {
        let damage = TacticalSubsystemDamage {
            shields: 1,
            weapons: 1,
            tractor: 2,
            engines: 2,
            hyperdrive: 1,
        };
        assert_eq!(damage.total_hits(), 7);
        let expected = [
            TacticalSubsystemKind::Engines,
            TacticalSubsystemKind::Engines,
            TacticalSubsystemKind::Shields,
            TacticalSubsystemKind::Hyperdrive,
            TacticalSubsystemKind::Tractor,
            TacticalSubsystemKind::Tractor,
            TacticalSubsystemKind::Weapons,
        ];
        for (index, kind) in expected.into_iter().enumerate() {
            assert_eq!(
                original_subsystem_repair_kind(damage, index as u16 + 1),
                Some(kind)
            );
        }
        assert_eq!(original_subsystem_repair_kind(damage, 0), None);
        assert_eq!(original_subsystem_repair_kind(damage, 8), None);
    }

    #[test]
    fn subsystem_repair_uses_inclusive_damage_control_chance() {
        let mut ship = test_ship(64, 0, true, true);
        ship.subsystem_capacity.engines = true;
        ship.subsystem_damage.engines = 2;
        ship.damage_control = 40;
        ship.refresh_subsystem_condition();
        assert_eq!(attempt_original_subsystem_repair(&mut ship, 0, 0, 1), None);
        assert_eq!(attempt_original_subsystem_repair(&mut ship, 0, 41, 1), None);
        let repair = attempt_original_subsystem_repair(&mut ship, 0, 40, 1)
            .expect("inclusive chance boundary should repair one engine hit");
        assert_eq!(repair.kind, TacticalSubsystemKind::Engines);
        assert_eq!((repair.hits_before, repair.hits_after), (2, 1));
        assert_eq!(ship.subsystem_damage.engines, 1);
        assert_eq!(ship.subsystem_condition.engines, 75);
        assert_eq!(ORIGINAL_SUBSYSTEM_REPAIR_INTERVAL_TICKS, 50 * 4);
    }

    #[test]
    fn engine_condition_uses_damage_mode_and_active_tractor_power_not_hull() {
        let mut target = test_ship(64, 0, true, true);
        target.hull_current = 10;
        target.hull_max = 100;
        target.subsystem_capacity.engines = true;
        target.sublight_engine_power = 100.0;
        target.maneuver_state_value = Some(4.0);
        target.subsystem_damage.engines = 1;

        let mut source = test_ship(128, 0, false, true);
        source.hull_current = 50;
        source.hull_max = 100;
        source.subsystem_capacity.tractor = true;
        source.tractor_beam_power = 40.0;
        source.subsystem_damage.tractor = 1;

        let mut session = test_session(vec![target, source], Vec::new(), true);
        assert!(session.set_tractor_field(1, 0, true));
        let held = session.subsystem_mobility(0).unwrap();
        assert_eq!(held.base_engine_power, 100.0);
        assert_eq!(held.engine_mode_bonus, 5.0);
        assert_eq!(held.active_tractor_power, 10.0);
        assert_eq!(held.effective_engine_power, 68.75);
        assert_eq!(held.engine_percent, 68);
        assert_eq!(session.ships[0].subsystem_condition.engines, 68);

        assert!(session.set_tractor_field(1, 0, false));
        let released = session.subsystem_mobility(0).unwrap();
        assert_eq!(released.active_tractor_power, 0.0);
        assert_eq!(released.effective_engine_power, 78.75);
        assert_eq!(released.engine_percent, 78);
        assert_eq!(session.ships[0].subsystem_condition.engines, 78);
    }

    #[test]
    fn maneuver_state_produces_the_source_clamped_one_through_nine_bonus() {
        assert_eq!(original_engine_mode_bonus(None), 1.0);
        assert_eq!(original_engine_mode_bonus(Some(0.0)), 9.0);
        assert_eq!(original_engine_mode_bonus(Some(4.0)), 5.0);
        assert_eq!(original_engine_mode_bonus(Some(8.0)), 1.0);
        assert_eq!(original_engine_mode_bonus(Some(9.0)), 1.0);
        assert_eq!(original_engine_mode_bonus(Some(-1.0)), 9.0);
    }

    #[test]
    fn movement_uses_effective_power_alignment_and_millisecond_integration() {
        let mut alliance = test_ship(64, 0, true, true);
        alliance.subsystem_capacity.engines = true;
        alliance.source_position.z = -56.0;
        alliance.sublight_engine_power = 100.0;
        alliance.maneuver_state_value = Some(4.0);
        alliance.subsystem_damage.engines = 1;

        let mut empire = test_ship(128, 0, false, true);
        empire.subsystem_capacity.engines = true;
        empire.source_position.z = 56.0;
        empire.sublight_engine_power = 100.0;
        empire.maneuver_state_value = Some(4.0);
        empire.subsystem_damage.engines = 1;

        let mut session = test_session(vec![alliance, empire], Vec::new(), true);
        session.advance_original_tactical_movement(250.0);
        assert_eq!(session.ships[0].source_velocity.z, 78.75);
        assert_eq!(session.ships[1].source_velocity.z, -78.75);
        assert_eq!(session.ships[0].source_position.z, -36.3125);
        assert_eq!(session.ships[1].source_position.z, 36.3125);

        session.ships[0].source_desired_forward = TacticalWorldVector {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        session.advance_original_tactical_movement(250.0);
        assert_eq!(session.ships[0].source_velocity, TacticalWorldVector::ZERO);
        assert_eq!(session.ships[0].source_position.z, -36.3125);

        session.ships[0].source_forward = TacticalWorldVector {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        session.ships[0].source_desired_forward = session.ships[0].source_forward;
        session.advance_original_tactical_movement(250.0);
        assert_eq!(session.ships[0].source_velocity.y, 78.75);
        assert_eq!(session.ships[0].source_position.y, 19.6875);
    }

    #[test]
    fn collision_predicate_preserves_strict_source_boundaries() {
        let envelope = OriginalTacticalCollisionEnvelope {
            vertical_radius: 1.0,
            planar_diameter: 2.0,
        };
        let mut first = test_ship(64, 0, true, true);
        first.source_collision_envelope = Some(envelope);
        let mut second = test_ship(128, 0, false, true);
        second.source_collision_envelope = Some(envelope);

        second.source_position.z = 1.999;
        assert!(original_tactical_objects_overlap(&first, &second));
        second.source_position.z = 2.0;
        assert!(!original_tactical_objects_overlap(&first, &second));
        second.source_position.z = 0.0;
        second.source_position.y = 1.999;
        assert!(original_tactical_objects_overlap(&first, &second));
        second.source_position.y = 2.0;
        assert!(!original_tactical_objects_overlap(&first, &second));
    }

    #[test]
    fn movement_restores_previous_position_when_mesh_envelopes_overlap() {
        let envelope = OriginalTacticalCollisionEnvelope {
            vertical_radius: 1.0,
            planar_diameter: 2.0,
        };
        let mut moving = test_ship(64, 0, true, true);
        moving.subsystem_capacity.engines = true;
        moving.sublight_engine_power = 4.0;
        moving.source_collision_envelope = Some(envelope);
        moving.source_position.z = 0.0;

        let mut fixed = test_ship(128, 0, false, true);
        fixed.source_collision_envelope = Some(envelope);
        fixed.source_position.z = 2.0;

        let mut session = test_session(vec![moving, fixed], Vec::new(), true);
        session.advance_original_tactical_movement(250.0);
        assert_eq!(session.ships[0].source_position.z, 0.0);
        assert_eq!(session.ships[0].source_velocity.z, 5.0);
    }

    #[test]
    fn current_to_desired_turn_uses_source_rates_and_normalization() {
        let close = |left: f32, right: f32| assert!((left - right).abs() < 0.000_01);
        close(original_capital_turn_rate(0), std::f32::consts::PI / 12.0);
        close(
            original_capital_turn_rate(4),
            5.0 * std::f32::consts::PI / 18.0,
        );
        close(original_capital_turn_rate(20), std::f32::consts::PI / 2.0);

        let mut ship = test_ship(64, 0, true, true);
        ship.maneuverability = 4;
        ship.source_desired_forward = TacticalWorldVector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        turn_original_tactical_ship(&mut ship, 250.0);
        let turn = 5.0 * std::f32::consts::PI / 72.0;
        close(ship.source_forward.x, turn.sin());
        close(ship.source_forward.y, 0.0);
        close(ship.source_forward.z, turn.cos());
        close(
            ship.source_forward
                .x
                .hypot(ship.source_forward.y)
                .hypot(ship.source_forward.z),
            1.0,
        );

        turn_original_tactical_ship(&mut ship, 10_000.0);
        assert_eq!(ship.source_forward, ship.source_desired_forward);
    }

    #[test]
    fn waypoint_arrival_and_fighter_docking_complete_source_executors() {
        let mut carrier = test_ship(64, 0, true, true);
        carrier.subsystem_capacity.engines = true;
        carrier.source_desired_forward = TacticalWorldVector::SOURCE_FORWARD;
        carrier.source_waypoint = Some(TacticalWorldPosition {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        });
        carrier.order = TacticalOrder::Anvil;

        let mut fighter = test_fighter(1, true);
        fighter.source_position.z = 3.5;
        fighter.sublight_engine_power = 4.0;
        fighter.order = TacticalOrder::Recover;
        fighter.recovery_state = TacticalFighterRecoveryState::Returning;
        fighter.recovery_target = Some(0);

        let mut session = test_session(vec![carrier], vec![fighter], true);
        session.advance_original_tactical_movement(250.0);
        assert_eq!(
            session.ships[0].source_position,
            TacticalWorldPosition {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            }
        );
        assert_eq!(session.ships[0].source_waypoint, None);
        assert_eq!(session.ships[0].source_velocity, TacticalWorldVector::ZERO);
        assert_eq!(session.ships[0].order, TacticalOrder::None);

        session.advance_original_fighter_recovery(250.0);
        assert_eq!(session.fighters[0].source_position.z, 2.5);
        assert_eq!(
            session.fighters[0].recovery_state,
            TacticalFighterRecoveryState::Docking
        );
        assert!(session.fighters[0].alive);

        session.advance_original_fighter_recovery(250.0);
        assert_eq!(
            session.fighters[0].recovery_state,
            TacticalFighterRecoveryState::Recovered
        );
        assert!(!session.fighters[0].alive);
        assert_eq!(session.fighters[0].squad_count, 12);
        assert_eq!(session.fighters[0].order, TacticalOrder::None);
    }

    #[test]
    fn maneuver_waypoints_preserve_source_rotations_and_scale() {
        let current = TacticalWorldPosition {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let target = TacticalWorldPosition {
            x: 9.0,
            y: 6.0,
            z: 15.0,
        };
        let (sin, cos) = ORIGINAL_MANEUVER_ANGLE.sin_cos();
        let close = |left: f32, right: f32| assert!((left - right).abs() < 0.000_01);

        let left = original_maneuver_waypoint(TacticalOrder::LeftHook, current, target).unwrap();
        close(left.x, 1.0 + (cos * 8.0 - sin * 12.0) * 0.75);
        close(left.y, 5.0);
        close(left.z, 3.0 + (cos * 12.0 + sin * 8.0) * 0.75);

        let right = original_maneuver_waypoint(TacticalOrder::RightHook, current, target).unwrap();
        close(right.x, 1.0 + (cos * 8.0 + sin * 12.0) * 0.75);
        close(right.y, 5.0);
        close(right.z, 3.0 + (cos * 12.0 - sin * 8.0) * 0.75);

        let hammer = original_maneuver_waypoint(TacticalOrder::Hammer, current, target).unwrap();
        close(hammer.x, 7.0);
        close(hammer.y, 2.0 + (cos * 4.0 - sin * 12.0) * 0.75);
        close(hammer.z, 3.0 + (cos * 12.0 + sin * 4.0) * 0.75);

        let anvil = original_maneuver_waypoint(TacticalOrder::Anvil, current, target).unwrap();
        close(anvil.x, 7.0);
        close(anvil.y, 2.0 + (cos * 4.0 + sin * 12.0) * 0.75);
        close(anvil.z, 3.0 + (cos * 12.0 - sin * 4.0) * 0.75);
    }

    #[test]
    fn committed_maneuver_hold_and_recover_enter_original_executors() {
        let mut player = test_ship(64, 0, true, true);
        player.selected = true;
        player.source_position.z = -50.0;
        player.fighter_capacity = 2;
        let mut enemy = test_ship(128, 0, false, true);
        enemy.source_position = TacticalWorldPosition {
            x: 10.0,
            y: 4.0,
            z: 50.0,
        };
        let mut fighter = test_fighter(1, true);
        fighter.selected = true;
        let mut session = test_session(vec![player, enemy], vec![fighter], true);
        session.selected_ship = Some(0);

        assert_eq!(
            assign_selected_command(
                &mut session,
                TacticalOrder::LeftHook,
                Some(TacticalTactic::Surround),
            ),
            (1, 0)
        );
        assert_eq!(session.ships[0].order, TacticalOrder::LeftHook);
        assert_eq!(session.ships[0].tactic, TacticalTactic::Surround);
        assert!(session.ships[0].source_waypoint.is_some());
        assert_ne!(
            session.ships[0].source_desired_forward,
            TacticalWorldVector::ZERO
        );

        assert_eq!(
            assign_selected_command(&mut session, TacticalOrder::HoldPosition, None),
            (1, 0)
        );
        assert_eq!(session.ships[0].source_waypoint, None);
        assert_eq!(
            session.ships[0].source_desired_forward,
            TacticalWorldVector::ZERO
        );
        assert_eq!(session.ships[0].source_velocity, TacticalWorldVector::ZERO);

        session.ships[0].selected = false;
        session.selected_ship = None;
        session.selected_fighter_group = Some(0);
        assert_eq!(
            assign_selected_command(&mut session, TacticalOrder::Recover, None),
            (0, 1)
        );
        assert_eq!(session.fighters[0].order, TacticalOrder::Recover);
        assert_eq!(
            session.fighters[0].recovery_state,
            TacticalFighterRecoveryState::Returning
        );
        assert_eq!(session.fighters[0].recovery_target, Some(0));
    }

    #[test]
    fn attack_orders_acquire_first_eligible_typed_target_for_both_unit_classes() {
        let mut player_ship = test_ship(64, 0, true, true);
        player_ship.selected = true;
        let dead_enemy_ship = test_ship(128, 0, false, false);
        let live_enemy_ship = test_ship(129, 1, false, true);

        let mut player_fighter = test_fighter(1, true);
        player_fighter.fighter_group = 0;
        let mut recovered_enemy_fighter = test_fighter(5, false);
        recovered_enemy_fighter.recovery_state = TacticalFighterRecoveryState::Recovered;
        let live_enemy_fighter = test_fighter(6, false);
        let mut session = test_session(
            vec![player_ship, dead_enemy_ship, live_enemy_ship],
            vec![player_fighter, recovered_enemy_fighter, live_enemy_fighter],
            true,
        );
        session.selected_ship = Some(0);

        assert_eq!(
            assign_selected_command(&mut session, TacticalOrder::AttackCapitalShips, None,),
            (1, 0)
        );
        assert_eq!(
            session.ships[0].attack_target,
            Some(TacticalAttackTarget::CapitalShip(2))
        );

        assert_eq!(
            assign_selected_command(&mut session, TacticalOrder::AttackFighters, None),
            (1, 0)
        );
        assert_eq!(
            session.ships[0].attack_target,
            Some(TacticalAttackTarget::FighterGroup(2))
        );

        session.ships[0].selected = false;
        session.selected_ship = None;
        session.fighters[0].selected = true;
        session.selected_fighter_group = Some(0);
        assert_eq!(
            assign_selected_command(&mut session, TacticalOrder::AttackFighters, None),
            (0, 1)
        );
        assert_eq!(
            session.fighters[0].attack_target,
            Some(TacticalAttackTarget::FighterGroup(2))
        );
    }

    #[test]
    fn attack_executor_preserves_live_engagement_and_replaces_invalid_target() {
        let mut player = test_ship(64, 0, true, true);
        player.selected = true;
        player.attack_target = Some(TacticalAttackTarget::CapitalShip(2));
        let first_enemy = test_ship(128, 0, false, true);
        let second_enemy = test_ship(129, 1, false, true);
        let mut session = test_session(vec![player, first_enemy, second_enemy], Vec::new(), true);
        session.selected_ship = Some(0);

        assign_selected_command(&mut session, TacticalOrder::AttackCapitalShips, None);
        assert_eq!(
            session.ships[0].attack_target,
            Some(TacticalAttackTarget::CapitalShip(2))
        );

        session.ships[2].alive = false;
        assign_selected_command(&mut session, TacticalOrder::AttackCapitalShips, None);
        assert_eq!(
            session.ships[0].attack_target,
            Some(TacticalAttackTarget::CapitalShip(1))
        );
    }

    #[test]
    fn attack_target_lifecycle_reacquires_same_class_and_clears_exhausted_lists() {
        let mut player_ship = test_ship(64, 0, true, true);
        player_ship.order = TacticalOrder::AttackCapitalShips;
        player_ship.attack_target = Some(TacticalAttackTarget::CapitalShip(1));
        let mut retreating_enemy = test_ship(128, 0, false, true);
        retreating_enemy.retreating = true;
        let live_enemy = test_ship(129, 1, false, true);

        let mut player_fighter = test_fighter(1, true);
        player_fighter.order = TacticalOrder::AttackFighters;
        player_fighter.attack_target = Some(TacticalAttackTarget::FighterGroup(1));
        let mut recovered_enemy_fighter = test_fighter(5, false);
        recovered_enemy_fighter.recovery_state = TacticalFighterRecoveryState::Recovered;
        let live_enemy_fighter = test_fighter(6, false);
        let mut session = test_session(
            vec![player_ship, retreating_enemy, live_enemy],
            vec![player_fighter, recovered_enemy_fighter, live_enemy_fighter],
            true,
        );

        refresh_original_attack_targets(&mut session);
        assert_eq!(
            session.ships[0].attack_target,
            Some(TacticalAttackTarget::CapitalShip(2))
        );
        assert_eq!(
            session.fighters[0].attack_target,
            Some(TacticalAttackTarget::FighterGroup(2))
        );

        session.ships[2].alive = false;
        session.fighters[2].squad_count = 0;
        refresh_original_attack_targets(&mut session);
        assert_eq!(session.ships[0].attack_target, None);
        assert_eq!(session.fighters[0].attack_target, None);

        session.ships[0].order = TacticalOrder::AttackFighters;
        refresh_original_attack_targets(&mut session);
        assert_eq!(
            session.ships[0].attack_target, None,
            "an exhausted fighter list must not fall through to a capital target"
        );
    }

    #[test]
    fn explicit_attack_order_never_redirects_an_invalid_target() {
        let mut source = test_ship(64, 0, true, true);
        source.order = TacticalOrder::AttackCapitalShips;
        source.attack_target = Some(TacticalAttackTarget::CapitalShip(1));
        source.weapon_arcs[0] = TacticalWeaponArc::new(300, 0, 0);
        source.weapon_ranges.laser_cannon = 100.0;
        let dead_target = test_ship(128, 0, false, false);
        let live_target = test_ship(129, 1, false, true);
        let mut ships = vec![source, dead_target, live_target];
        let original_hull = ships[2].hull_current;
        let mut fields = Vec::new();
        let mut projectiles = Vec::new();
        let mut impacts = Vec::new();

        BattleSession::fire_side(
            &mut ships,
            &mut fields,
            &[0],
            &[2],
            &mut projectiles,
            &mut impacts,
            1,
        );

        assert_eq!(ships[2].hull_current, original_hull);
        assert!(projectiles.is_empty());
        assert!(impacts.is_empty());
    }

    #[test]
    fn hull_subsystem_selection_preserves_every_source_boundary() {
        assert_eq!(original_hull_subsystem_kind(50.0), None);
        assert_eq!(
            original_hull_subsystem_kind(50.001),
            Some(TacticalSubsystemKind::Shields)
        );
        assert_eq!(
            original_hull_subsystem_kind(65.0),
            Some(TacticalSubsystemKind::Shields)
        );
        assert_eq!(
            original_hull_subsystem_kind(65.001),
            Some(TacticalSubsystemKind::Weapons)
        );
        assert_eq!(
            original_hull_subsystem_kind(80.001),
            Some(TacticalSubsystemKind::Tractor)
        );
        assert_eq!(
            original_hull_subsystem_kind(90.001),
            Some(TacticalSubsystemKind::Engines)
        );
        assert_eq!(
            original_hull_subsystem_kind(95.001),
            Some(TacticalSubsystemKind::Hyperdrive)
        );
        assert_eq!(original_hull_subsystem_kind(100.001), None);
    }

    #[test]
    fn shield_damage_uses_inclusive_source_probability() {
        let mut ship = test_ship(64, 0, true, true);
        ship.hull_current = 1000;
        ship.hull_max = 1000;
        ship.shield = 100;
        ship.shield_max = 100;
        ship.subsystem_capacity.shields = true;
        ship.refresh_subsystem_condition();
        let mut ships = vec![ship.clone()];
        let mut fields = Vec::new();
        apply_original_capital_damage(
            &mut ships,
            &mut fields,
            0,
            20,
            OriginalTacticalDamageRolls {
                shield: 20,
                hull: 0,
            },
        );
        assert_eq!(ships[0].subsystem_damage.shields, 1);
        assert_eq!(ships[0].subsystem_condition.shields, 75);

        let mut ships = vec![ship];
        apply_original_capital_damage(
            &mut ships,
            &mut fields,
            0,
            20,
            OriginalTacticalDamageRolls {
                shield: 21,
                hull: 0,
            },
        );
        assert_eq!(ships[0].subsystem_damage.shields, 0);
        assert_eq!(ships[0].subsystem_condition.shields, 100);
    }

    #[test]
    fn hull_damage_mutates_each_source_subsystem_category() {
        let cases = [
            (
                51,
                TacticalSubsystemDamage {
                    shields: 1,
                    ..TacticalSubsystemDamage::default()
                },
            ),
            (
                66,
                TacticalSubsystemDamage {
                    weapons: 1,
                    ..TacticalSubsystemDamage::default()
                },
            ),
            (
                81,
                TacticalSubsystemDamage {
                    tractor: 1,
                    ..TacticalSubsystemDamage::default()
                },
            ),
            (
                91,
                TacticalSubsystemDamage {
                    engines: 1,
                    ..TacticalSubsystemDamage::default()
                },
            ),
            (
                96,
                TacticalSubsystemDamage {
                    hyperdrive: 1,
                    ..TacticalSubsystemDamage::default()
                },
            ),
        ];
        for (roll, expected) in cases {
            let mut ship = test_ship(64, 0, true, true);
            ship.hull_current = 1000;
            ship.hull_max = 1000;
            ship.subsystem_capacity = TacticalSubsystemCapacity {
                shields: true,
                weapons: true,
                tractor: true,
                engines: true,
                hyperdrive: 2,
            };
            ship.refresh_subsystem_condition();
            let mut ships = vec![ship];
            let stage = apply_original_capital_damage(
                &mut ships,
                &mut Vec::new(),
                0,
                1,
                OriginalTacticalDamageRolls {
                    shield: 100,
                    hull: roll,
                },
            );
            assert_eq!(stage, OriginalTacticalImpactStage::Damage);
            assert_eq!(ships[0].hull_current, 999);
            assert_eq!(ships[0].subsystem_damage, expected);
        }
    }

    #[test]
    fn destroyed_tractor_subsystem_clears_its_live_field_source() {
        let mut source = test_ship(64, 0, true, true);
        source.hull_current = 1000;
        source.hull_max = 1000;
        source.subsystem_capacity.tractor = true;
        source.subsystem_damage.tractor = 3;
        source.refresh_subsystem_condition();
        let target = test_ship(128, 0, false, true);
        let mut ships = vec![source, target];
        let mut fields = Vec::new();
        assert!(set_original_tactical_field(
            &mut fields,
            0,
            1,
            OriginalTacticalFieldKind::Tractor,
            true,
        ));

        apply_original_capital_damage(
            &mut ships,
            &mut fields,
            0,
            1,
            OriginalTacticalDamageRolls {
                shield: 100,
                hull: 81,
            },
        );
        assert_eq!(ships[0].subsystem_damage.tractor, 4);
        assert_eq!(ships[0].subsystem_condition.tractor, 0);
        assert!(fields.is_empty());
    }

    #[test]
    fn weapon_arc_classifier_preserves_source_order_and_boundaries() {
        let source = test_ship(64, 0, true, true);
        let target_at = |x: f32, z: f32| {
            let mut target = test_ship(128, 0, false, true);
            target.source_position = TacticalWorldPosition { x, y: 0.0, z };
            target
        };

        assert_eq!(
            original_tactical_weapon_arc(&source, &target_at(0.0, 10.0)),
            OriginalTacticalWeaponArc::Fore
        );
        assert_eq!(
            original_tactical_weapon_arc(&source, &target_at(10.0, 0.0)),
            OriginalTacticalWeaponArc::Starboard
        );
        assert_eq!(
            original_tactical_weapon_arc(&source, &target_at(0.0, -10.0)),
            OriginalTacticalWeaponArc::Aft
        );
        assert_eq!(
            original_tactical_weapon_arc(&source, &target_at(-10.0, 0.0)),
            OriginalTacticalWeaponArc::Port
        );
        assert_eq!(
            original_tactical_weapon_arc(&source, &target_at(10.0, 10.0)),
            OriginalTacticalWeaponArc::Fore,
            "the +45 degree boundary belongs to the fore arc"
        );
        assert_eq!(
            original_tactical_weapon_arc(&source, &target_at(10.0, -10.0)),
            OriginalTacticalWeaponArc::Starboard,
            "the +135 degree boundary remains lateral"
        );
        assert_eq!(
            original_tactical_weapon_arc(&source, &target_at(-10.0, -10.0)),
            OriginalTacticalWeaponArc::Port,
            "the -135 degree boundary remains lateral"
        );
    }

    #[test]
    fn direct_weapon_callback_fires_every_family_and_queues_the_arc_once() {
        let mut source = test_ship(64, 0, true, true);
        source.hull_current = 1000;
        source.hull_max = 1000;
        source.weapon_arcs[0] = TacticalWeaponArc::new(60, 40, 80);
        source.weapon_ranges = TacticalWeaponRanges {
            laser_cannon: 100.0,
            ion_cannon: 100.0,
            turbolaser: 100.0,
        };
        source.order = TacticalOrder::AttackCapitalShips;
        source.attack_target = Some(TacticalAttackTarget::CapitalShip(1));
        let mut target = test_ship(128, 0, false, true);
        target.source_position.z = 10.0;
        target.hull_current = 1000;
        target.hull_max = 1000;
        target.shield = 1000;
        target.shield_max = 1000;
        let mut ships = vec![source, target];
        let mut fields = Vec::new();
        let mut projectiles = Vec::new();
        let mut impacts = Vec::new();

        BattleSession::fire_side(
            &mut ships,
            &mut fields,
            &[0],
            &[1],
            &mut projectiles,
            &mut impacts,
            1,
        );

        assert_eq!(
            projectiles
                .iter()
                .map(|effect| effect.kind)
                .collect::<Vec<_>>(),
            vec![
                WeaponKind::IonCannon,
                WeaponKind::LaserCannon,
                WeaponKind::Turbolaser
            ]
        );
        assert_eq!(ships[0].weapon_recharge_queue, vec![0]);
        assert!(!ships[0].weapon_arcs[0].ready);
        assert_eq!(ships[0].weapon_arcs[0].energy, 0.0);
        assert_eq!(ships[1].shield, 965);
    }

    #[test]
    fn automatic_weapon_callback_keeps_first_arc_on_ties_and_fires_each_ready_arc() {
        let mut source = test_ship(64, 0, true, true);
        source.hull_current = 100;
        source.hull_max = 100;
        source.weapon_arcs[0] = TacticalWeaponArc::new(0, 0, 30);
        source.weapon_arcs[1] = TacticalWeaponArc::new(0, 0, 30);
        source.weapon_ranges.turbolaser = 100.0;
        let mut fore = test_ship(128, 0, false, true);
        fore.source_position.z = 10.0;
        fore.hull_current = 1000;
        fore.hull_max = 1000;
        fore.shield = 1000;
        fore.shield_max = 1000;
        let mut starboard = fore.clone();
        starboard.identity.fleet_roster_index = 1;
        starboard.source_position = TacticalWorldPosition {
            x: 10.0,
            y: 0.0,
            z: 0.0,
        };
        let mut ships = vec![source, fore, starboard];
        let mut projectiles = Vec::new();

        BattleSession::fire_side(
            &mut ships,
            &mut Vec::new(),
            &[0],
            &[1, 2],
            &mut projectiles,
            &mut Vec::new(),
            1,
        );

        assert_eq!(
            projectiles
                .iter()
                .map(|effect| effect.target)
                .collect::<Vec<_>>(),
            vec![1, 2]
        );
        assert_eq!(ships[0].weapon_recharge_queue, vec![0, 1]);
    }

    #[test]
    fn weapon_candidates_enforce_range_and_apply_hull_scaling_only_to_auto_fire() {
        let mut source = test_ship(64, 0, true, true);
        source.hull_current = 50;
        source.hull_max = 100;
        source.weapon_arcs[0] = TacticalWeaponArc::new(0, 0, 100);
        source.weapon_ranges.turbolaser = 10.0;
        let mut target = test_ship(128, 0, false, true);
        target.source_position.z = 10.0;
        let ships = vec![source, target];

        let automatic = original_capital_weapon_candidates(&ships, 0, &[1], None);
        let direct = original_capital_weapon_candidates(&ships, 0, &[1], Some(1));
        assert!((automatic[0].turbolaser.strength - 13.333_334).abs() < 0.000_01);
        assert!((direct[0].turbolaser.strength - 26.666_668).abs() < 0.000_01);

        let mut out_of_range = ships;
        out_of_range[1].source_position.z = 10.001;
        assert_eq!(
            original_capital_weapon_candidates(&out_of_range, 0, &[1], None)[0]
                .turbolaser
                .target,
            None
        );
    }

    #[test]
    fn weapon_recharge_is_fifo_and_preserves_leftover_energy() {
        let mut ship = test_ship(64, 0, true, true);
        ship.hull_current = 100;
        ship.hull_max = 100;
        ship.weapon_recharge_rate = 8.0;
        ship.weapon_arcs[0] = TacticalWeaponArc::new(0, 0, 10);
        ship.weapon_arcs[1] = TacticalWeaponArc::new(0, 0, 10);
        for arc in &mut ship.weapon_arcs[..2] {
            arc.ready = false;
            arc.energy = 0.0;
        }
        ship.weapon_recharge_queue = vec![0, 1];

        ship.advance_original_recharge(1.0);
        assert_eq!(ship.weapon_arcs[0].energy, 8.0);
        assert_eq!(ship.weapon_arcs[1].energy, 0.0);
        assert_eq!(ship.weapon_recharge_queue, vec![0, 1]);

        ship.advance_original_recharge(0.5);
        assert_eq!(ship.weapon_arcs[0].energy, 10.0);
        assert!(ship.weapon_arcs[0].ready);
        assert_eq!(ship.weapon_arcs[1].energy, 2.0);
        assert_eq!(ship.weapon_recharge_queue, vec![1]);
    }

    #[test]
    fn shield_recharge_retains_fractional_source_energy_and_damage_penalties() {
        let mut ship = test_ship(64, 0, true, true);
        ship.hull_current = 100;
        ship.hull_max = 100;
        ship.shield = 0;
        ship.shield_max = 10;
        ship.shield_recharge_rate = 3.0;

        for _ in 0..4 {
            ship.advance_original_recharge(0.25);
        }
        assert_eq!(ship.shield, 3);
        assert_eq!(ship.shield_recharge_carry, 0.0);

        ship.subsystem_damage.shields = 1;
        ship.advance_original_recharge(1.0);
        assert_eq!(ship.shield, 5);
        assert!((ship.shield_recharge_carry - 0.25).abs() < f32::EPSILON);
    }

    #[test]
    fn destroyed_weapon_subsystem_disables_and_repair_recharges_live_arcs() {
        let mut ship = test_ship(64, 0, true, true);
        ship.hull_current = 100;
        ship.hull_max = 100;
        ship.subsystem_capacity.weapons = true;
        ship.weapon_recharge_rate = 10.0;
        ship.weapon_arcs[0] = TacticalWeaponArc::new(0, 0, 10);

        for _ in 0..4 {
            ship.add_subsystem_hit(TacticalSubsystemKind::Weapons);
        }
        assert!(!ship.weapon_arcs[0].ready);
        assert_eq!(ship.weapon_arcs[0].energy, 0.0);
        assert_eq!(ship.weapon_recharge_queue, vec![0]);
        ship.remove_subsystem_hit(TacticalSubsystemKind::Weapons);
        ship.advance_original_recharge(4.0);
        assert!(ship.weapon_arcs[0].ready);
        assert_eq!(ship.weapon_arcs[0].energy, 10.0);
        assert!(ship.weapon_recharge_queue.is_empty());
    }

    #[test]
    fn live_ship_fire_uses_the_shared_subsystem_damage_path() {
        let mut source = test_ship(64, 0, true, true);
        source.hull_current = 1000;
        source.hull_max = 1000;
        source.weapon_arcs[0] = TacticalWeaponArc::new(300, 0, 0);
        source.weapon_ranges.laser_cannon = 100.0;
        let mut target = test_ship(128, 0, false, true);
        target.hull_current = 1000;
        target.hull_max = 1000;
        target.shield = 10;
        target.shield_max = 10;
        target.subsystem_capacity.shields = true;
        target.refresh_subsystem_condition();
        let mut ships = vec![source, target];
        let mut fields = Vec::new();
        let mut projectiles = Vec::new();
        let mut impacts = Vec::new();

        BattleSession::fire_side(
            &mut ships,
            &mut fields,
            &[0],
            &[1],
            &mut projectiles,
            &mut impacts,
            1,
        );

        assert!(ships[1].hull_current < 1000);
        assert_eq!(ships[1].subsystem_damage.shields, 1);
        assert!(ships[1].subsystem_condition.shields < 100);
        assert_eq!(projectiles.len(), 1);
        assert_eq!(impacts.len(), 1);
    }

    #[test]
    fn original_slot_sequence_matches_executable_branch_order() {
        let actual: Vec<_> = OriginalTacticalSlots::default().take(9).collect();
        assert_eq!(actual[0].to_bits(), (-0.0f32).to_bits());
        assert_eq!(
            actual,
            vec![0.0, 5.0, -5.0, -10.0, 10.0, 15.0, -15.0, -20.0, 20.0]
        );
    }

    #[test]
    fn production_participants_retain_dat_identity_and_source_world_positions() {
        let mut ships = vec![
            test_ship(64, 0, true, true),
            test_ship(65, 2, true, true),
            test_ship(66, 7, true, false),
            test_ship(133, 0, false, true),
            test_ship(134, 1, false, true),
            test_ship(135, 2, false, true),
        ];
        let mut fighters = vec![test_fighter(3, true), test_fighter(5, false)];

        let layout = BattleSession::assign_original_world_positions(&mut ships, &mut fighters);
        assert_eq!(layout.first_active_objects, 3);
        assert_eq!(layout.second_active_objects, 4);
        assert_eq!(layout.battle_extent, 112.0);
        assert_eq!(
            ships[0].source_position,
            TacticalWorldPosition {
                x: 0.0,
                y: 0.0,
                z: -56.0
            }
        );
        assert_eq!(
            ships[1].source_position,
            TacticalWorldPosition {
                x: 5.0,
                y: 0.0,
                z: -56.0
            }
        );
        assert_eq!(ships[2].source_position, TacticalWorldPosition::ORIGIN);
        assert_eq!(
            ships[3].source_position,
            TacticalWorldPosition {
                x: 0.0,
                y: 0.0,
                z: 56.0
            }
        );
        assert_eq!(
            ships[4].source_position,
            TacticalWorldPosition {
                x: 5.0,
                y: 0.0,
                z: 56.0
            }
        );
        assert_eq!(
            ships[5].source_position,
            TacticalWorldPosition {
                x: -5.0,
                y: 0.0,
                z: 56.0
            }
        );
        assert_eq!(
            fighters[0].source_position,
            TacticalWorldPosition {
                x: 0.0,
                y: 0.0,
                z: -36.0
            }
        );
        assert_eq!(
            fighters[1].source_position,
            TacticalWorldPosition {
                x: 0.0,
                y: 0.0,
                z: 36.0
            }
        );
        assert_eq!(ships[1].identity.class_dat_id, DatId::new(65));
        assert_eq!(ships[1].identity.fleet_roster_index, 2);
    }

    #[test]
    fn fighter_groups_follow_source_ordinals_for_both_factions() {
        let mut fighters = [
            test_fighter(3, true),
            test_fighter(4, true),
            test_fighter(1, true),
            test_fighter(2, true),
            test_fighter(5, false),
            test_fighter(6, false),
            test_fighter(7, false),
            test_fighter(8, false),
        ];
        for fighter in &mut fighters {
            fighter.fighter_group = 4;
        }

        BattleSession::assign_original_fighter_groups(&mut fighters);
        assert_eq!(
            fighters.map(|fighter| fighter.fighter_group),
            [0, 1, 2, 3, 0, 1, 2, 3]
        );
    }

    #[test]
    fn task_force_leaders_follow_weapon_categories_square_root_counts_and_source_slots() {
        let mut ships = (0..9)
            .map(|index| test_ship(64, index, true, true))
            .collect::<Vec<_>>();
        for ship in &mut ships[..4] {
            ship.weapon_arcs[0] = TacticalWeaponArc::new(0, 1, 0);
        }
        for ship in &mut ships[4..8] {
            ship.weapon_arcs[0] = TacticalWeaponArc::new(1, 0, 0);
        }

        BattleSession::assign_original_task_forces(&mut ships);

        assert_eq!(
            ships.iter().map(|ship| ship.task_force).collect::<Vec<_>>(),
            vec![0, 0, 1, 1, 2, 2, 3, 3, 4]
        );
        assert_eq!(ships[0].source_position.y, 0.0);
        assert_eq!(ships[2].source_position.y, 8.0);
        assert_eq!(ships[4].source_position.y, -8.0);
        assert_eq!(ships[6].source_position.y, 16.0);
        assert_eq!(ships[8].source_position.y, -16.0);
    }

    #[test]
    fn task_force_capability_order_prioritizes_range_then_available_sublight() {
        let mut ships = (0..4)
            .map(|index| test_ship(64, index, true, true))
            .collect::<Vec<_>>();
        for ship in &mut ships {
            ship.weapon_arcs[0] = TacticalWeaponArc::new(1, 0, 0);
        }
        ships[0].weapon_ranges.laser_cannon = 10.0;
        ships[0].sublight_engine_power = 50.0;
        ships[1].weapon_ranges.laser_cannon = 10.0;
        ships[1].sublight_engine_power = 200.0;
        ships[2].weapon_ranges.laser_cannon = 20.0;
        ships[2].sublight_engine_power = 1.0;
        ships[3].weapon_ranges.laser_cannon = 5.0;
        ships[3].sublight_engine_power = 500.0;

        BattleSession::assign_original_task_forces(&mut ships);

        assert_eq!(
            ships.iter().map(|ship| ship.task_force).collect::<Vec<_>>(),
            vec![1, 0, 0, 1]
        );
        assert_eq!(ships[2].source_position.x, 0.0);
        assert_eq!(ships[0].source_position.x, 0.0);
    }

    #[test]
    fn task_force_followers_use_largest_footprint_and_alternating_slots() {
        let mut ships = (0..3)
            .map(|index| test_ship(64, index, true, true))
            .collect::<Vec<_>>();
        for ship in &mut ships {
            ship.weapon_arcs[0] = TacticalWeaponArc::new(1, 0, 0);
            ship.source_collision_envelope = Some(OriginalTacticalCollisionEnvelope {
                vertical_radius: 1.0,
                planar_diameter: 4.0,
            });
        }
        ships[1].source_collision_envelope = Some(OriginalTacticalCollisionEnvelope {
            vertical_radius: 2.0,
            planar_diameter: 10.0,
        });
        ships[0].source_position.x = -0.0;

        BattleSession::assign_original_task_forces(&mut ships);

        assert_eq!(ships[0].task_force, 0);
        assert_eq!(ships[1].task_force, 0);
        assert_eq!(ships[2].task_force, 0);
        assert_eq!(ships[0].source_position.x.to_bits(), (-0.0_f32).to_bits());
        assert_eq!(ships[1].source_position.x, 12.0);
        assert_eq!(ships[2].source_position.x, -12.0);
        assert!(ships.iter().all(|ship| ship.source_position.y == 0.0));
    }

    #[test]
    fn task_force_followers_rotate_and_move_with_the_retained_anchor() {
        let mut ships = (0..2)
            .map(|index| test_ship(64, index, true, true))
            .collect::<Vec<_>>();
        for ship in &mut ships {
            ship.weapon_arcs[0] = TacticalWeaponArc::new(1, 0, 0);
            ship.subsystem_capacity.engines = true;
            ship.sublight_engine_power = 4.0;
            ship.source_forward = TacticalWorldVector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            };
            ship.source_desired_forward = ship.source_forward;
            ship.source_collision_envelope = Some(OriginalTacticalCollisionEnvelope {
                vertical_radius: 1.0,
                planar_diameter: 10.0,
            });
        }
        BattleSession::assign_original_task_forces(&mut ships);
        let mut session = test_session(ships, Vec::new(), true);

        session.advance_original_tactical_movement(250.0);

        assert_eq!(session.ships[0].source_position.x, 1.25);
        assert_eq!(session.ships[1].source_position.x, 1.25);
        assert_eq!(session.ships[0].source_position.z, 0.0);
        assert_eq!(session.ships[1].source_position.z, -12.0);
        assert_eq!(
            session.ships[0].source_position.y,
            session.ships[1].source_position.y
        );
    }

    #[test]
    fn empty_fighter_group_splits_first_half_only_above_seven_members() {
        let mut ten = (0..10)
            .map(|index| {
                let mut fighter = test_fighter(3, true);
                fighter.fleet_squadron_index = index;
                fighter.fighter_group = 4;
                fighter
            })
            .collect::<Vec<_>>();
        BattleSession::assign_original_fighter_groups(&mut ten);
        assert!(ten[..5].iter().all(|fighter| fighter.fighter_group == 1));
        assert!(ten[5..].iter().all(|fighter| fighter.fighter_group == 0));

        let mut seven = (0..7)
            .map(|_| {
                let mut fighter = test_fighter(3, true);
                fighter.fighter_group = 4;
                fighter
            })
            .collect::<Vec<_>>();
        BattleSession::assign_original_fighter_groups(&mut seven);
        assert!(seven.iter().all(|fighter| fighter.fighter_group == 0));
    }

    #[test]
    fn fighter_roster_expands_each_strategic_squadron_with_exact_dat_stats() {
        let mut world = GameWorld::default();
        let class = world
            .fighter_classes
            .insert(rebellion_core::world::FighterClass {
                dat_id: DatId::new(1),
                name: "X-wing".into(),
                is_alliance: true,
                squadron_size: 12,
                shield_strength: 5,
                sub_light_engine: 11,
                maneuverability: 8,
                laser_cannon_fore: 5,
                laser_cannon_range: 12,
                torpedoes: 4,
                torpedoes_range: 7,
                ..rebellion_core::world::FighterClass::default()
            });
        let fleet = world.fleets.insert(rebellion_core::world::Fleet {
            location: SystemKey::default(),
            capital_ships: Vec::new(),
            fighters: vec![rebellion_core::world::FighterEntry { class, count: 2 }],
            characters: Vec::new(),
            is_alliance: true,
            has_death_star: false,
        });
        let mut ships = Vec::new();
        let mut fighters = Vec::new();

        BattleSession::expand_fleet(&world, fleet, true, &mut ships, &mut fighters);

        assert!(ships.is_empty());
        assert_eq!(fighters.len(), 2);
        assert_eq!(fighters[0].fleet_squadron_index, 0);
        assert_eq!(fighters[1].fleet_squadron_index, 1);
        assert_eq!(fighters[0].squad_count, 12);
        assert_eq!(fighters[0].hull_current, 96.0);
        assert_eq!(fighters[0].shield, 5.0);
        assert_eq!(fighters[0].sublight_engine_power, 4.4);
        assert_eq!(fighters[0].weapon_arc.laser_cannons, 5);
        assert_eq!(fighters[0].weapon_ranges.laser_cannon, 12.0);
        assert_eq!(fighters[0].torpedo_strength, 4);
        assert_eq!(fighters[0].torpedo_range, 7.0);
    }

    #[test]
    fn fighter_hull_drives_weapon_shield_recharge_and_live_craft_count() {
        let mut fighter = test_fighter(1, true);
        assert!((fighter.weapon_hull_factor() - 45.012).abs() < 0.001);
        assert!((fighter.shield_recharge_rate() - 0.3).abs() < 0.001);

        fighter.weapon_arc.ready = false;
        fighter.weapon_arc.energy = 0.0;
        fighter.shield = 0.0;
        fighter.advance_original_recharge(1.0);
        assert!(fighter.weapon_arc.ready);
        assert_eq!(
            fighter.weapon_arc.energy,
            fighter.weapon_arc.energy_capacity
        );
        assert!((fighter.shield - 0.3).abs() < 0.001);

        fighter.hull_current = 79.9;
        fighter.refresh_craft_count();
        assert_eq!(fighter.squad_count, 10);
        fighter.hull_current = 0.0;
        fighter.refresh_craft_count();
        assert_eq!(fighter.squad_count, 0);
        assert!(!fighter.alive);
    }

    #[test]
    fn fighter_damage_uses_float_shields_hull_and_maneuver_ratio() {
        let mut target = test_fighter(5, false);
        target.shield = 5.0;
        target.shield_max = 5.0;
        target.maneuverability = 7;
        let mut fighters = vec![target];

        assert_eq!(
            apply_original_fighter_damage(&mut fighters, 0, 9.0, 4.0),
            OriginalTacticalImpactStage::Hit
        );
        assert!((fighters[0].shield - 0.5).abs() < 0.001);
        assert_eq!(fighters[0].hull_current, 96.0);

        assert_eq!(
            apply_original_fighter_damage(&mut fighters, 0, 9.0, 4.0),
            OriginalTacticalImpactStage::Damage
        );
        assert_eq!(fighters[0].shield, 0.0);
        assert!((fighters[0].hull_current - 92.0).abs() < 0.001);
        assert_eq!(fighters[0].squad_count, 12);
    }

    #[test]
    fn fighter_fore_battery_fires_source_families_and_torpedoes_at_capitals() {
        let mut fighter = test_fighter(2, true);
        fighter.weapon_arc = TacticalWeaponArc::new(8, 6, 0);
        fighter.weapon_ranges = TacticalWeaponRanges {
            laser_cannon: 20.0,
            ion_cannon: 18.0,
            turbolaser: 0.0,
        };
        fighter.torpedo_strength = 12;
        fighter.torpedo_range = 7.0;
        fighter.order = TacticalOrder::AttackCapitalShips;
        fighter.attack_target = Some(TacticalAttackTarget::CapitalShip(0));
        fighter.source_position.z = 0.0;
        let mut target = test_ship(128, 0, false, true);
        target.source_position.z = 6.0;
        target.hull_current = 1000;
        target.hull_max = 1000;
        target.shield = 0;
        target.shield_max = 0;
        let mut session = test_session(vec![target], vec![fighter], true);

        session.fighter_step();

        assert!(session.ships[0].hull_current < 1000);
        assert!(!session.fighters[0].weapon_arc.ready);
        assert_eq!(session.weapon_effects.len(), 3);
        assert_eq!(session.weapon_effects[0].kind, WeaponKind::IonCannon);
        assert_eq!(session.weapon_effects[1].kind, WeaponKind::LaserCannon);
        assert_eq!(session.weapon_effects[2].kind, WeaponKind::FighterAttack);
        assert_eq!(session.weapon_effects[0].source, usize::MAX);
        assert_eq!(
            session.weapon_effects[0].target_position,
            session.ships[0].source_position
        );
    }

    #[test]
    fn dogfight_ignores_ion_and_applies_target_maneuverability() {
        let mut source = test_fighter(2, true);
        source.weapon_arc = TacticalWeaponArc::new(8, 6, 0);
        source.weapon_ranges = TacticalWeaponRanges {
            laser_cannon: 20.0,
            ion_cannon: 18.0,
            turbolaser: 0.0,
        };
        source.maneuverability = 2;
        source.order = TacticalOrder::AttackFighters;
        source.attack_target = Some(TacticalAttackTarget::FighterGroup(1));
        let mut target = test_fighter(6, false);
        target.shield = 0.0;
        target.shield_max = 0.0;
        target.maneuverability = 8;
        target.weapon_arc = TacticalWeaponArc::default();
        target.source_position.z = 10.0;
        let mut session = test_session(Vec::new(), vec![source, target], true);

        session.fighter_step();

        assert!(session.fighters[1].hull_current < 96.0);
        assert!(session.fighters[1].hull_current > 60.0);
        assert_eq!(session.weapon_effects.len(), 1);
        assert_eq!(session.weapon_effects[0].kind, WeaponKind::LaserCannon);
        assert_eq!(session.weapon_effects[0].target, usize::MAX);
    }

    #[test]
    fn capital_attack_fighters_uses_typed_target_without_capital_fallback() {
        let mut source = test_ship(64, 0, true, true);
        source.weapon_arcs[0] = TacticalWeaponArc::new(300, 10, 0);
        source.weapon_ranges.laser_cannon = 100.0;
        source.weapon_ranges.ion_cannon = 100.0;
        source.maneuverability = 8;
        source.order = TacticalOrder::AttackFighters;
        source.attack_target = Some(TacticalAttackTarget::FighterGroup(0));
        let mut target = test_fighter(5, false);
        target.shield = 0.0;
        target.shield_max = 0.0;
        target.source_position.z = 10.0;
        let mut session = test_session(vec![source], vec![target], true);
        let mut effects = Vec::new();

        session.fire_capitals_at_fighters(&mut effects);

        assert!(!session.fighters[0].alive);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0].kind, WeaponKind::LaserCannon);
        assert!(!session.ships[0].weapon_arcs[0].ready);
    }

    #[test]
    fn battle_phase_variants() {
        assert_ne!(BattlePhase::Placement, BattlePhase::Combat);
        assert_ne!(BattlePhase::Combat, BattlePhase::Results);
    }

    #[test]
    fn original_tactical_canvas_letterboxes_without_distorting_controls() {
        let native = TacticalCanvas::new(640.0, 480.0);
        assert_eq!((native.x, native.y, native.scale), (0.0, 0.0, 1.0));
        let wide = TacticalCanvas::new(1280.0, 800.0);
        assert_eq!(wide.scale, 5.0 / 3.0);
        assert_eq!(wide.x, (1280.0 - 640.0 * wide.scale) * 0.5);
        let (screen_x, screen_y) = wide.point(574.0, 317.0);
        let (logical_x, logical_y) = wide.logical_pointer(screen_x, screen_y);
        assert!((logical_x - 574.0).abs() < 0.001);
        assert!((logical_y - 317.0).abs() < 0.001);
    }

    #[test]
    fn original_tactical_control_rects_stop_at_bitmap_edges() {
        assert_eq!(
            tactical_hud_rect_control_at(560.0, 307.0),
            Some(TacticalHudControl::Pause)
        );
        assert_eq!(
            tactical_hud_rect_control_at(587.9, 327.9),
            Some(TacticalHudControl::Pause)
        );
        assert_eq!(tactical_hud_rect_control_at(588.0, 317.0), None);
        assert_eq!(tactical_hud_rect_control_at(559.9, 317.0), None);
        assert_eq!(
            tactical_hud_rect_control_at(486.0, 343.0),
            Some(TacticalHudControl::ZoomIn)
        );
        assert_eq!(tactical_hud_rect_control_at(510.0, 355.0), None);
        assert_eq!(
            tactical_hud_rect_control_at(603.0, 343.0),
            Some(TacticalHudControl::ZoomOut)
        );
        assert_eq!(
            tactical_hud_rect_control_at(517.0, 304.0),
            Some(TacticalHudControl::HighlightAlliance)
        );
        assert_eq!(
            tactical_hud_rect_control_at(482.0, 304.0),
            Some(TacticalHudControl::HighlightEmpire)
        );
        assert_eq!(
            tactical_hud_rect_control_at(485.0, 272.0),
            Some(TacticalHudControl::NavigationSet(0))
        );
        assert_eq!(
            tactical_hud_rect_control_at(627.9, 298.9),
            Some(TacticalHudControl::NavigationSet(3))
        );
        assert_eq!(tactical_hud_rect_control_at(628.0, 285.0), None);
        assert_eq!(
            tactical_hud_rect_control_at(484.0, 430.0),
            Some(TacticalHudControl::CameraRecall)
        );
        assert_eq!(
            tactical_hud_rect_control_at(627.9, 454.9),
            Some(TacticalHudControl::CameraMemorize)
        );
        assert_eq!(tactical_hud_rect_control_at(628.0, 442.0), None);
    }

    #[test]
    fn original_tactical_controls_use_source_bitmap_hit_masks() {
        let mut cache = BmpCache::new();
        cache.set_base_path(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/base/ui"),
        );

        // Original control hit testing excludes every outer edge, even when
        // the containing rectangle starts there.
        assert_eq!(tactical_hud_control_at(&mut cache, 560.0, 307.0), None);
        assert_eq!(
            tactical_hud_control_at(&mut cache, 562.1, 308.1),
            Some(TacticalHudControl::Pause)
        );

        // Both zoom controls contain transparent matte pixels inside their
        // bounding rectangles. Those pixels must pass through untouched.
        assert_eq!(tactical_hud_control_at(&mut cache, 487.1, 344.1), None);
        assert_eq!(
            tactical_hud_control_at(&mut cache, 491.1, 344.1),
            Some(TacticalHudControl::ZoomIn)
        );
        assert_eq!(tactical_hud_control_at(&mut cache, 604.1, 344.1), None);
        assert_eq!(
            tactical_hud_control_at(&mut cache, 609.1, 344.1),
            Some(TacticalHudControl::ZoomOut)
        );
        assert_eq!(
            tactical_hud_control_at(&mut cache, 520.1, 397.1),
            Some(TacticalHudControl::CameraLeft)
        );
        assert_eq!(
            tactical_hud_control_at(&mut cache, 590.1, 397.1),
            Some(TacticalHudControl::CameraRight)
        );
        assert_eq!(
            tactical_hud_control_at(&mut cache, 558.1, 354.1),
            Some(TacticalHudControl::CameraUp)
        );
        assert_eq!(
            tactical_hud_control_at(&mut cache, 558.1, 444.1),
            Some(TacticalHudControl::CameraDown)
        );

        assert_eq!(
            tactical_hud_control_at(&mut cache, 518.1, 305.1),
            Some(TacticalHudControl::HighlightAlliance)
        );
        assert_eq!(
            tactical_hud_control_at(&mut cache, 483.1, 305.1),
            Some(TacticalHudControl::HighlightEmpire)
        );
        assert_eq!(tactical_hud_control_at(&mut cache, 588.0, 317.0), None);
        assert_eq!(
            tactical_hud_control_at(&mut cache, 616.0, 318.0),
            Some(TacticalHudControl::BattleOptions)
        );

        assert_eq!(
            tactical_battle_options_control_at(&mut cache, true, 521.0, 64.0),
            Some(TacticalBattleOptionsControl::Withdraw)
        );
        assert_eq!(
            tactical_battle_options_control_at(&mut cache, false, 521.0, 64.0),
            None
        );
        assert_eq!(
            tactical_battle_options_control_at(&mut cache, true, 519.0, 126.0),
            Some(TacticalBattleOptionsControl::SimulateRemainder)
        );
        assert_eq!(
            tactical_battle_options_control_at(&mut cache, true, 583.0, 126.0),
            Some(TacticalBattleOptionsControl::ObserveToggle)
        );
        assert_eq!(
            tactical_battle_options_control_at(&mut cache, true, 520.0, 219.0),
            Some(TacticalBattleOptionsControl::GameOptions)
        );
        assert_eq!(
            tactical_battle_options_control_at(&mut cache, true, 579.0, 219.0),
            Some(TacticalBattleOptionsControl::Close)
        );
        assert_eq!(
            tactical_withdraw_confirmation_control_at(&mut cache, 560.0, 242.0),
            Some(TacticalWithdrawConfirmationControl::Confirm)
        );
        assert_eq!(
            tactical_withdraw_confirmation_control_at(&mut cache, 600.0, 242.0),
            Some(TacticalWithdrawConfirmationControl::Cancel)
        );
        assert_eq!(
            tactical_withdraw_confirmation_control_at(&mut cache, 547.0, 229.0),
            None
        );
        assert_eq!(
            tactical_withdraw_confirmation_control_at(&mut cache, 614.0, 242.0),
            None
        );

        assert_eq!(
            tactical_hud_control_at(&mut cache, 72.1, 10.1),
            Some(TacticalHudControl::TaskForce(0))
        );
        assert_eq!(
            tactical_hud_control_at(&mut cache, 254.1, 10.1),
            Some(TacticalHudControl::TaskForce(7))
        );
        assert_eq!(tactical_hud_control_at(&mut cache, 60.1, 2.1), None);
        assert_eq!(
            tactical_hud_control_at(&mut cache, 342.1, 10.1),
            Some(TacticalHudControl::FighterGroup(0))
        );
        assert_eq!(
            tactical_hud_control_at(&mut cache, 420.1, 10.1),
            Some(TacticalHudControl::FighterGroup(3))
        );
    }

    #[test]
    fn withdrawal_requires_native_confirmation_before_dispatch() {
        let mut state = TacticalState::default();
        state.battle_options_open = true;

        assert_eq!(
            activate_tactical_battle_options(&mut state, TacticalBattleOptionsControl::Withdraw,),
            TacticalAction::None
        );
        assert!(!state.battle_options_open);
        assert!(state.withdraw_confirmation_open);

        assert_eq!(
            activate_tactical_withdraw_confirmation(
                &mut state,
                TacticalWithdrawConfirmationControl::Cancel,
            ),
            TacticalAction::None
        );
        assert!(!state.withdraw_confirmation_open);

        state.battle_options_open = true;
        assert_eq!(
            activate_tactical_battle_options(&mut state, TacticalBattleOptionsControl::Withdraw,),
            TacticalAction::None
        );
        assert_eq!(
            activate_tactical_withdraw_confirmation(
                &mut state,
                TacticalWithdrawConfirmationControl::Confirm,
            ),
            TacticalAction::WithdrawFromBattle
        );
        assert!(!state.withdraw_confirmation_open);
    }

    #[test]
    fn task_force_and_fighter_group_selection_are_mutually_exclusive() {
        let mut ships = (0..8)
            .map(|index| test_ship(64, index, true, true))
            .collect::<Vec<_>>();
        ships[0].selected = true;
        let mut fighters = (0..4)
            .map(|index| {
                let mut fighter = test_fighter(1, true);
                fighter.fighter_group = index;
                fighter.identity.fleet_roster_index = usize::from(index);
                fighter.fleet_fighter_index = usize::from(index);
                fighter
            })
            .collect::<Vec<_>>();
        fighters.push(test_fighter(5, false));
        let mut session = test_session(ships, fighters, true);
        session.selected_ship = Some(0);

        assert_eq!(select_task_force(&mut session, 7), 1);
        assert_eq!(selected_player_task_force(&session), Some(7));
        assert_eq!(session.selected_ship, Some(7));

        assert!(select_fighter_group(&mut session, 3));
        assert_eq!(session.selected_ship, None);
        assert_eq!(session.selected_fighter_group, Some(3));
        assert!(session.ships.iter().all(|ship| !ship.selected));
        assert_eq!(
            session
                .fighters
                .iter()
                .filter(|fighter| fighter.selected)
                .count(),
            1
        );

        assert_eq!(select_task_force(&mut session, 0), 1);
        assert_eq!(session.selected_fighter_group, None);
        assert!(session.fighters.iter().all(|fighter| !fighter.selected));
    }

    #[test]
    fn task_force_assignment_accepts_only_an_existing_or_next_blank_slot() {
        let mut ships = vec![
            test_ship(64, 0, true, true),
            test_ship(65, 1, true, true),
            test_ship(128, 0, false, true),
        ];
        ships[0].selected = true;
        ships[1].selected = true;
        let mut session = test_session(ships, Vec::new(), true);

        assert_eq!(assign_selected_to_task_force(&mut session, 4), 0);
        assert_eq!(session.ships[0].task_force, 0);
        assert_eq!(session.ships[1].task_force, 1);
        assert_eq!(assign_selected_to_task_force(&mut session, 2), 2);
        assert_eq!(session.ships[0].task_force, 2);
        assert_eq!(session.ships[1].task_force, 2);
    }

    #[test]
    fn selected_capital_navigation_wraps_within_the_current_task_force() {
        let mut ships = vec![
            test_ship(64, 0, true, true),
            test_ship(65, 1, true, true),
            test_ship(66, 2, true, true),
            test_ship(128, 0, false, true),
        ];
        for ship in &mut ships[..3] {
            ship.task_force = 2;
            ship.selected = true;
        }
        let mut session = test_session(ships, vec![test_fighter(1, true)], true);
        session.selected_ship = Some(0);

        assert_eq!(cycle_selected_capital(&mut session, -1), Some((0, 2, 2)));
        assert_eq!(session.selected_ship, Some(2));
        assert_eq!(
            session
                .ships
                .iter()
                .enumerate()
                .filter_map(|(index, ship)| ship.selected.then_some(index))
                .collect::<Vec<_>>(),
            vec![2]
        );
        assert_eq!(cycle_selected_capital(&mut session, 1), Some((2, 0, 2)));
        assert_eq!(cycle_selected_capital(&mut session, 1), Some((0, 1, 2)));
        assert!(session.fighters.iter().all(|fighter| !fighter.selected));
    }

    #[test]
    fn ordered_navigation_route_advances_without_losing_the_next_waypoint() {
        let first = TacticalWorldPosition {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        let second = TacticalWorldPosition {
            x: 0.0,
            y: 0.0,
            z: 2.0,
        };
        let mut ship = test_ship(64, 0, true, true);
        ship.subsystem_capacity.engines = true;
        ship.source_waypoint = Some(first);
        ship.navigation_route = vec![first, second];
        let mut session = test_session(vec![ship], Vec::new(), true);

        session.advance_original_tactical_movement(10.0);
        assert_eq!(session.ships[0].source_position, first);
        assert_eq!(session.ships[0].source_waypoint, Some(second));
        assert_eq!(session.ships[0].navigation_route, vec![second]);

        session.advance_original_tactical_movement(10.0);
        assert_eq!(session.ships[0].source_position, second);
        assert_eq!(session.ships[0].source_waypoint, None);
        assert!(session.ships[0].navigation_route.is_empty());
    }

    #[test]
    fn ordered_manual_targets_advance_and_clear_as_hostiles_are_destroyed() {
        let mut player = test_ship(64, 0, true, true);
        player.manual_targets = vec![
            TacticalAttackTarget::CapitalShip(1),
            TacticalAttackTarget::CapitalShip(2),
        ];
        player.attack_target = player.manual_targets.first().copied();
        let mut session = test_session(
            vec![
                player,
                test_ship(128, 0, false, true),
                test_ship(129, 1, false, true),
            ],
            Vec::new(),
            true,
        );

        session.ships[1].alive = false;
        refresh_original_attack_targets(&mut session);
        assert_eq!(
            session.ships[0].manual_targets,
            vec![TacticalAttackTarget::CapitalShip(2)]
        );
        assert_eq!(
            session.ships[0].attack_target,
            Some(TacticalAttackTarget::CapitalShip(2))
        );

        session.ships[2].alive = false;
        refresh_original_attack_targets(&mut session);
        assert!(session.ships[0].manual_targets.is_empty());
        assert_eq!(session.ships[0].attack_target, None);
    }

    #[test]
    fn tactical_order_and_tactic_codes_match_the_recovered_source_domain() {
        let orders = [
            (TacticalOrder::None, 0),
            (TacticalOrder::Recover, 2),
            (TacticalOrder::AttackFighters, 4),
            (TacticalOrder::AttackCapitalShips, 5),
            (TacticalOrder::AttackDeathStar, 6),
            (TacticalOrder::LeftHook, 7),
            (TacticalOrder::RightHook, 8),
            (TacticalOrder::Hammer, 9),
            (TacticalOrder::Anvil, 10),
            (TacticalOrder::HoldPosition, 11),
        ];
        for (order, code) in orders {
            assert_eq!(order.source_code(), code);
        }
        assert_eq!(TacticalTactic::Surround.source_code(), 1);
        assert_eq!(TacticalTactic::StandOff.source_code(), 2);
    }

    #[test]
    fn death_star_operational_predicate_resources_and_laser_charge_match_source_fields() {
        let mut death_star = TacticalDeathStar {
            resource: DEATH_STAR_TACTICAL_RESOURCE,
            is_attacker: false,
            is_alliance: false,
            source_position: TacticalWorldPosition::ORIGIN,
            hull: 100.0,
            laser_charge: 0.0,
            destroyed: false,
            action_committed: false,
        };
        assert!(death_star.operational());
        assert_eq!(death_star.resource_id(), 5010);
        death_star.advance_laser_charge(300.0);
        assert_eq!(death_star.laser_charge, 100.0);
        death_star.advance_laser_charge(1.0);
        assert_eq!(death_star.laser_charge, 100.0);

        death_star.action_committed = true;
        assert!(!death_star.operational());
        death_star.action_committed = false;
        death_star.destroyed = true;
        assert!(!death_star.operational());
        assert_eq!(death_star.resource_id(), 5020);
    }

    #[test]
    fn death_star_superlaser_requires_player_ownership_full_charge_and_enemy_target() {
        let target = test_ship(64, 0, true, true);
        let mut session = test_session(vec![target], Vec::new(), false);
        session.death_star = Some(TacticalDeathStar {
            resource: DEATH_STAR_TACTICAL_RESOURCE,
            is_attacker: false,
            is_alliance: false,
            source_position: TacticalWorldPosition::ORIGIN,
            hull: 100.0,
            laser_charge: 99.0,
            destroyed: false,
            action_committed: false,
        });
        assert!(!session.commit_death_star_shot(0));
        session.death_star.as_mut().unwrap().laser_charge = 100.0;
        session.player_is_attacker = true;
        assert!(!session.commit_death_star_shot(0));
        session.player_is_attacker = false;
        session.ships[0].is_attacker = false;
        assert!(!session.commit_death_star_shot(0));
    }

    #[test]
    fn death_star_superlaser_resolves_after_the_source_long_projectile_duration() {
        let mut target = test_ship(64, 0, true, true);
        target.source_position.z = -24.0;
        let mut session = test_session(vec![target], Vec::new(), false);
        session.death_star = Some(TacticalDeathStar {
            resource: DEATH_STAR_TACTICAL_RESOURCE,
            is_attacker: false,
            is_alliance: false,
            source_position: TacticalWorldPosition {
                x: 0.0,
                y: 0.0,
                z: 24.0,
            },
            hull: 100.0,
            laser_charge: 100.0,
            destroyed: false,
            action_committed: false,
        });

        assert!(session.commit_death_star_shot(0));
        assert_eq!(session.death_star.unwrap().laser_charge, 0.0);
        assert!(session.death_star.unwrap().action_committed);
        assert!(session.death_star_beam.is_some());
        session.advance_presentational_effects(1.999);
        assert!(session.ships[0].alive);
        session.advance_presentational_effects(0.002);
        assert!(!session.ships[0].alive);
        assert_eq!(session.ships[0].hull_current, 0);
        assert!(session.death_star_beam.is_none());
        assert!(!session.death_star.unwrap().action_committed);
        assert_eq!(session.impact_effects.len(), 1);
    }

    #[test]
    fn attack_death_star_order_routes_selected_fighter_fire_to_manager_object() {
        let mut fighter = test_fighter(3, true);
        fighter.selected = true;
        fighter.order = TacticalOrder::AttackDeathStar;
        fighter.source_position.z = 0.0;
        fighter.weapon_ranges.laser_cannon = 20.0;
        let mut session = test_session(Vec::new(), vec![fighter], true);
        session.paused = false;
        session.selected_fighter_group = Some(0);
        session.death_star = Some(TacticalDeathStar {
            resource: DEATH_STAR_TACTICAL_RESOURCE,
            is_attacker: false,
            is_alliance: false,
            source_position: TacticalWorldPosition {
                x: 0.0,
                y: 0.0,
                z: 6.0,
            },
            hull: 1.0,
            laser_charge: 0.0,
            destroyed: false,
            action_committed: false,
        });

        assert!(session.step());
        let death_star = session.death_star.unwrap();
        assert!(death_star.destroyed);
        assert_eq!(death_star.hull, 0.0);
        assert_eq!(death_star.resource_id(), 5020);
        assert_eq!(session.winner, Some(CombatWinner::Attacker));
        assert_eq!(session.phase, BattlePhase::Results);
        assert_eq!(session.weapon_effects.len(), 1);
        assert_eq!(
            session.trench_run_outcome,
            Some(TacticalTrenchRunOutcome::Success)
        );
        assert_eq!(
            session.take_pending_trench_run_cinematic(),
            Some(TacticalTrenchRunOutcome::Success)
        );
        assert_eq!(session.take_pending_trench_run_cinematic(), None);
    }

    #[test]
    fn losing_the_committed_death_star_attackers_routes_failure_once() {
        let mut fighter = test_fighter(3, true);
        fighter.order = TacticalOrder::AttackDeathStar;
        fighter.weapon_arc = TacticalWeaponArc::default();
        fighter.shield = 0.0;
        fighter.shield_max = 0.0;
        fighter.hull_current = 1.0;
        fighter.refresh_craft_count();

        let mut defender = test_ship(128, 0, false, true);
        defender.order = TacticalOrder::AttackFighters;
        defender.attack_target = Some(TacticalAttackTarget::FighterGroup(0));
        defender.weapon_arcs[0] = TacticalWeaponArc::new(300, 0, 0);
        defender.weapon_ranges.laser_cannon = 100.0;

        let mut session = test_session(vec![defender], vec![fighter], true);
        session.paused = false;
        session.death_star = Some(TacticalDeathStar {
            resource: DEATH_STAR_TACTICAL_RESOURCE,
            is_attacker: false,
            is_alliance: false,
            source_position: TacticalWorldPosition {
                x: 0.0,
                y: 0.0,
                z: 6.0,
            },
            hull: 100.0,
            laser_charge: 0.0,
            destroyed: false,
            action_committed: false,
        });

        assert!(session.step());
        assert!(!session.fighters[0].alive);
        assert!(!session.death_star.unwrap().destroyed);
        assert_eq!(
            session.take_pending_trench_run_cinematic(),
            Some(TacticalTrenchRunOutcome::Failure)
        );
        assert_eq!(session.take_pending_trench_run_cinematic(), None);
    }

    #[test]
    fn trench_run_result_is_recorded_once_and_failure_remains_source_driven() {
        let mut session = test_session(Vec::new(), Vec::new(), true);
        session.record_trench_run_outcome(TacticalTrenchRunOutcome::Failure);
        session.record_trench_run_outcome(TacticalTrenchRunOutcome::Success);

        assert!(session.paused);
        assert_eq!(
            session.take_pending_trench_run_cinematic(),
            Some(TacticalTrenchRunOutcome::Failure)
        );
        assert_eq!(
            session.trench_run_outcome,
            Some(TacticalTrenchRunOutcome::Failure)
        );
        assert_eq!(session.take_pending_trench_run_cinematic(), None);
    }

    #[test]
    fn tactical_assignment_hit_regions_follow_the_source_panels() {
        assert_eq!(
            tactical_command_rect_control_at(
                TacticalCommandPanel::Display,
                true,
                false,
                false,
                493.0,
                60.0,
            ),
            Some(TacticalCommandControl::PreviousCapital)
        );
        assert_eq!(
            tactical_command_rect_control_at(
                TacticalCommandPanel::Display,
                true,
                false,
                false,
                618.0,
                60.0,
            ),
            Some(TacticalCommandControl::NextCapital)
        );
        assert_eq!(
            tactical_command_rect_control_at(
                TacticalCommandPanel::Display,
                true,
                true,
                false,
                493.0,
                60.0,
            ),
            None
        );
        assert_eq!(
            tactical_command_rect_control_at(
                TacticalCommandPanel::Display,
                true,
                false,
                false,
                500.0,
                245.0,
            ),
            Some(TacticalCommandControl::OpenMissions)
        );
        assert_eq!(
            tactical_command_rect_control_at(
                TacticalCommandPanel::Display,
                true,
                false,
                false,
                570.0,
                245.0,
            ),
            Some(TacticalCommandControl::OpenManeuvers)
        );
        let maneuvers = TacticalCommandPanel::Maneuvers {
            pending_order: TacticalOrder::None,
            pending_tactic: TacticalTactic::StandOff,
        };
        assert_eq!(
            tactical_command_rect_control_at(maneuvers, true, false, false, 500.0, 160.0),
            Some(TacticalCommandControl::Hammer)
        );
        assert_eq!(
            tactical_command_rect_control_at(maneuvers, true, false, false, 600.0, 170.0),
            Some(TacticalCommandControl::ToggleTactic)
        );
        let missions = TacticalCommandPanel::Missions {
            pending_order: TacticalOrder::None,
        };
        assert_eq!(
            tactical_command_rect_control_at(missions, true, false, false, 575.0, 165.0),
            None
        );
        assert_eq!(
            tactical_command_rect_control_at(missions, true, true, false, 575.0, 165.0),
            Some(TacticalCommandControl::Recover)
        );
        assert_eq!(
            tactical_command_rect_control_at(missions, true, true, false, 575.0, 195.0),
            None
        );
        assert_eq!(
            tactical_command_rect_control_at(missions, true, true, true, 575.0, 195.0),
            Some(TacticalCommandControl::AttackDeathStar)
        );
        assert_eq!(
            tactical_command_rect_control_at(missions, false, true, false, 505.0, 165.0),
            None
        );
    }

    #[test]
    fn maneuver_confirm_commits_and_cancel_preserves_selected_capital_orders() {
        let mut ship = test_ship(64, 0, true, true);
        ship.selected = true;
        let mut enemy = test_ship(128, 0, false, true);
        enemy.source_position.z = 50.0;
        let mut session = test_session(vec![ship, enemy], Vec::new(), true);
        session.selected_ship = Some(0);
        let mut state = TacticalState {
            session: Some(session),
            ..TacticalState::default()
        };

        activate_tactical_command(&mut state, TacticalCommandControl::OpenManeuvers);
        activate_tactical_command(&mut state, TacticalCommandControl::Hammer);
        activate_tactical_command(&mut state, TacticalCommandControl::ToggleTactic);
        activate_tactical_command(&mut state, TacticalCommandControl::Cancel);
        let ship = &state.session.as_ref().unwrap().ships[0];
        assert_eq!(ship.order, TacticalOrder::None);
        assert_eq!(ship.tactic, TacticalTactic::StandOff);

        activate_tactical_command(&mut state, TacticalCommandControl::OpenManeuvers);
        activate_tactical_command(&mut state, TacticalCommandControl::Hammer);
        activate_tactical_command(&mut state, TacticalCommandControl::ToggleTactic);
        activate_tactical_command(&mut state, TacticalCommandControl::Confirm);
        let ship = &state.session.as_ref().unwrap().ships[0];
        assert_eq!(ship.order, TacticalOrder::Hammer);
        assert_eq!(ship.tactic, TacticalTactic::Surround);
        assert_eq!(state.command_panel, TacticalCommandPanel::Display);
    }

    #[test]
    fn mission_confirm_assigns_recover_to_the_whole_selected_fighter_group() {
        let mut fighters = vec![test_fighter(1, true), test_fighter(1, true)];
        for (index, fighter) in fighters.iter_mut().enumerate() {
            fighter.selected = true;
            fighter.identity.fleet_roster_index = index;
            fighter.fleet_fighter_index = index;
        }
        let mut session = test_session(Vec::new(), fighters, true);
        session.selected_fighter_group = Some(0);
        let mut state = TacticalState {
            session: Some(session),
            ..TacticalState::default()
        };

        activate_tactical_command(&mut state, TacticalCommandControl::OpenMissions);
        activate_tactical_command(&mut state, TacticalCommandControl::Recover);
        activate_tactical_command(&mut state, TacticalCommandControl::Confirm);
        assert!(state
            .session
            .as_ref()
            .unwrap()
            .fighters
            .iter()
            .all(|fighter| fighter.order == TacticalOrder::Recover));
    }

    #[test]
    fn target_control_owns_its_topmost_mask_and_centers_the_selected_position() {
        let mut cache = BmpCache::new();
        cache.set_base_path(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/base/ui"),
        );
        assert_eq!(
            tactical_hud_control_at(&mut cache, 549.1, 390.1),
            Some(TacticalHudControl::CameraTarget)
        );
        assert_eq!(camera_offset_for_target(120.0, 190.0), (-480.0, -210.0));
        assert_eq!(camera_offset_for_target(600.0, 400.0), (0.0, 0.0));
    }
}
