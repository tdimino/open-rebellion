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

use std::collections::HashMap;

use egui_macroquad::egui::{self, Color32, RichText, Vec2};
use macroquad::prelude::*;
use rebellion_core::ids::{CapitalShipKey, DatId, FighterKey, FleetKey, SystemKey};
use rebellion_core::world::GameWorld;

use crate::bmp_cache::{resources, BmpCache, DllSource};
use crate::sector_window::planet_picture_id;
#[cfg(feature = "interface-test-fixtures")]
use crate::tactical_assets::TacticalLodView;
use crate::tactical_assets::{
    TacticalAssetRenderer, TacticalFighterRenderObject, TacticalRenderObject,
    TacticalScreenProjection,
};
use crate::tactical_resources::{
    capital_ship_tactical_resource, death_star_tactical_resource, fighter_tactical_resource,
    TacticalCapitalShipResource, TacticalDeathStarResource, TacticalFighterResource,
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

/// Deployment zone width (fraction of arena width per side).
const DEPLOY_ZONE_FRACTION: f32 = 0.3;

/// Task force info panel BMP IDs in TACTICAL.DLL.
/// 1001 = attacker/Alliance panel, 1002 = defender/Empire panel.
/// Full range 1001-1037 covers faction-colored task force + squadron frames.
const TACTICAL_TASKFORCE_PANEL_ATTACKER: u32 = 1001;
const TACTICAL_TASKFORCE_PANEL_DEFENDER: u32 = 1002;

/// Weapon recharge gauge BMP IDs (5-step animation, 0%→100%).
/// Frame 0 (empty) = 1206, frame 4 (full) = 1210.
const TACTICAL_RECHARGE_GAUGE_BASE: u32 = 1206;
const TACTICAL_RECHARGE_GAUGE_STEPS: u32 = 5;

/// Hull integrity + shield strength combined display panel.
const TACTICAL_HULL_SHIELD_PANEL: u32 = 1302;

/// Default ship icon size when no sprite is available.
const DEFAULT_SHIP_SIZE: f32 = 40.0;

/// Fighter squadron icon size.
const FIGHTER_SIZE: f32 = 16.0;

/// Spacing between auto-placed ships.
const SHIP_SPACING: f32 = 60.0;

const ORIGINAL_BATTLE_BASE_EXTENT: f32 = 100.0;
const ORIGINAL_BATTLE_OBJECT_INCREMENT: f32 = 3.0;
const ORIGINAL_BATTLE_OUTER_LANE_SCALE: f32 = 0.5;
const ORIGINAL_BATTLE_INNER_LANE_OFFSET: f32 = 20.0;
const ORIGINAL_BATTLE_SLOT_SPACING: f32 = 5.0;

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
    /// True if this ship belongs to the attacker side.
    pub is_attacker: bool,
    /// True if the ship is still alive.
    pub alive: bool,
    /// Whether this ship is currently selected by the player.
    pub selected: bool,
    /// Index into the fleet's `capital_ships` for damage application.
    pub fleet_ship_index: usize,
    /// Sprite resource ID in TACTICAL.DLL (if known).
    pub sprite_id: Option<u32>,
    /// Total turbolaser firepower (sum of fore/aft/port/starboard arcs).
    pub turbolaser_power: i32,
    /// Total ion cannon firepower (sum of all arcs).
    pub ion_cannon_power: i32,
    /// Total laser cannon firepower (sum of all arcs).
    pub laser_cannon_power: i32,
    /// Focus-fire target: index in `ships` that this ship prioritizes.
    pub focus_target: Option<usize>,
    /// True if this ship is retreating (moving off-screen).
    pub retreating: bool,
    /// Retreat progress: 0.0 = just started, 1.0 = off-screen (removed from combat).
    pub retreat_progress: f32,
    /// True if this ship successfully retreated (survived, not destroyed).
    pub retreated: bool,
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
    /// Index into the originating fleet's fighter roster. Classes may repeat.
    pub fleet_fighter_index: usize,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub squad_count: u32,
    pub is_attacker: bool,
    pub alive: bool,
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
    /// Immutable source battle envelope used to place the initial participants.
    pub source_layout: OriginalTacticalLayout,
    /// Index of currently selected ship (in `ships`), if any.
    pub selected_ship: Option<usize>,
    /// Whether the placement phase is confirmed (player clicked "Begin Battle").
    pub placement_confirmed: bool,
    /// Game tick when the battle started.
    pub start_tick: u64,
    /// Combat sub-tick counter (increments each combat step).
    pub combat_tick: u32,
    /// Active weapon fire visual effects (source ship idx -> target ship idx).
    pub weapon_effects: Vec<WeaponEffect>,
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
    /// Index of the firing ship in `ships`.
    pub source: usize,
    /// Index of the target ship in `ships`.
    pub target: usize,
    /// Weapon type determines color and rendering.
    pub kind: WeaponKind,
    /// Remaining frames to display this effect.
    pub ttl: u8,
}

/// Type of weapon for visual rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponKind {
    Turbolaser,
    IonCannon,
    LaserCannon,
    FighterAttack,
}

impl WeaponKind {
    #[must_use]
    pub fn color(self) -> Color {
        match self {
            WeaponKind::Turbolaser => Color::new(0.0, 1.0, 0.0, 0.8), // green
            WeaponKind::IonCannon => Color::new(0.3, 0.5, 1.0, 0.8),  // blue
            WeaponKind::LaserCannon => Color::new(1.0, 0.2, 0.2, 0.8), // red
            WeaponKind::FighterAttack => Color::new(1.0, 0.8, 0.2, 0.6), // yellow
        }
    }
}

/// Battle outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatWinner {
    Attacker,
    Defender,
    Draw,
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
            source_layout,
            selected_ship,
            placement_confirmed: true,
            start_tick: tick,
            combat_tick: 0,
            weapon_effects: Vec::new(),
            paused: true,
            combat_speed: 1,
            step_accumulator: 0.0,
            winner: None,
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
                name: class.name.clone(),
                x: 0.0,
                y: 0.0,
                hull_current: ship.hull_current,
                hull_max: class.hull.cast_signed(),
                shield: class.shield_strength.cast_signed(),
                shield_max: class.shield_strength.cast_signed(),
                is_attacker,
                alive: true,
                selected: false,
                fleet_ship_index: ship_idx,
                // IDs 2001 through 2130 are not a linear class sprite table.
                // Keep the legacy selected-panel slot empty until its exact
                // bitmap role is independently recovered.
                sprite_id: None,
                turbolaser_power: turbolaser_total,
                ion_cannon_power: ion_cannon_total,
                laser_cannon_power: laser_cannon_total,
                focus_target: None,
                retreating: false,
                retreat_progress: 0.0,
                retreated: false,
            });
        }

        for (fighter_idx, entry) in fleet.fighters.iter().enumerate() {
            let class = &world.fighter_classes[entry.class];
            fighters.push(TacticalFighter {
                class_key: entry.class,
                identity: TacticalObjectIdentity {
                    class_dat_id: class.dat_id,
                    fleet_roster_index: fighter_idx,
                    is_alliance: fleet.is_alliance,
                },
                tactical_resource: fighter_tactical_resource(class.dat_id),
                source_position: TacticalWorldPosition::ORIGIN,
                fleet_fighter_index: fighter_idx,
                name: class.name.clone(),
                x: 0.0,
                y: 0.0,
                squad_count: entry.count,
                is_attacker,
                alive: entry.count > 0,
            });
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

    /// Advance combat by one tick. Applies weapon fire, shield regen, hull
    /// damage, and fighter engagement. Produces weapon effects for rendering.
    ///
    /// Returns `true` if the battle has ended this tick.
    pub fn step(&mut self) -> bool {
        if self.phase != BattlePhase::Combat || self.paused {
            return false;
        }

        self.combat_tick += 1;

        // Decay existing weapon effects.
        self.weapon_effects.retain_mut(|e| {
            e.ttl = e.ttl.saturating_sub(1);
            e.ttl > 0
        });

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
            &atk_alive,
            &def_alive,
            &mut new_effects,
            self.combat_tick,
        );
        Self::fire_side(
            &mut self.ships,
            &def_alive,
            &atk_alive,
            &mut new_effects,
            self.combat_tick,
        );
        self.weapon_effects.extend(new_effects);

        // Phase: Shield regeneration (every 3 ticks).
        if self.combat_tick.is_multiple_of(3) {
            for ship in &mut self.ships {
                if ship.alive && ship.shield < ship.shield_max {
                    // Regen ~5% of max shields per 3 ticks.
                    let regen = (ship.shield_max / 20).max(1);
                    ship.shield = (ship.shield + regen).min(ship.shield_max);
                }
            }
        }

        // Phase: Fighter engagement (every 2 ticks).
        if self.combat_tick.is_multiple_of(2) {
            self.fighter_step();
        }

        // Check for battle end.
        let atk_remaining = self.ships.iter().any(|s| s.is_attacker && s.alive)
            || self
                .fighters
                .iter()
                .any(|f| f.is_attacker && f.alive && f.squad_count > 0);
        let def_remaining = self.ships.iter().any(|s| !s.is_attacker && s.alive)
            || self
                .fighters
                .iter()
                .any(|f| !f.is_attacker && f.alive && f.squad_count > 0);

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

    /// One side's ships fire at the other side.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
    )]
    fn fire_side(
        ships: &mut [TacticalShip],
        firing: &[usize],
        targets: &[usize],
        effects: &mut Vec<WeaponEffect>,
        tick: u32,
    ) {
        if targets.is_empty() {
            return;
        }

        for &fire_idx in firing {
            if !ships[fire_idx].alive || ships[fire_idx].retreating {
                continue;
            }

            // Focus-fire: if this ship has a valid focus target, prefer it.
            let target_idx = if let Some(ft) = ships[fire_idx].focus_target {
                if ft < ships.len() && ships[ft].alive && targets.contains(&ft) {
                    ft
                } else {
                    // Focus target dead or invalid — fall back to pseudo-random.
                    targets[((tick as usize).wrapping_mul(fire_idx + 1).wrapping_add(7))
                        % targets.len()]
                }
            } else {
                // No focus target — pseudo-random.
                targets
                    [((tick as usize).wrapping_mul(fire_idx + 1).wrapping_add(7)) % targets.len()]
            };

            // Calculate total weapon output and kind from actual weapon stats.
            let ship = &ships[fire_idx];
            let (fire_power, kind) = if ship.turbolaser_power >= ship.ion_cannon_power
                && ship.turbolaser_power >= ship.laser_cannon_power
                && ship.turbolaser_power > 0
            {
                (ship.turbolaser_power, WeaponKind::Turbolaser)
            } else if ship.ion_cannon_power >= ship.laser_cannon_power && ship.ion_cannon_power > 0
            {
                (ship.ion_cannon_power, WeaponKind::IonCannon)
            } else if ship.laser_cannon_power > 0 {
                (ship.laser_cannon_power, WeaponKind::LaserCannon)
            } else {
                // Fallback for ships with no weapon data: hull-based approximation.
                ((ship.hull_max / 10).max(1), WeaponKind::LaserCannon)
            };

            // Variance: +-30% using tick-based pseudo-random.
            let variance_seed = tick.wrapping_mul(31).wrapping_add(fire_idx as u32 * 17);
            let variance = ((variance_seed % 60).cast_signed() - 30) * fire_power / 100;
            let damage = (fire_power + variance).max(1);

            // Apply damage: shields first, then hull.
            let target = &mut ships[target_idx];
            let shield_absorb = damage.min(target.shield);
            target.shield -= shield_absorb;
            let hull_damage = damage - shield_absorb;
            target.hull_current = (target.hull_current - hull_damage).max(0);
            if target.hull_current == 0 {
                target.alive = false;
            }

            // Create visual effect.
            effects.push(WeaponEffect {
                source: fire_idx,
                target: target_idx,
                kind,
                ttl: 8,
            });
        }
    }

    /// Fighter squadrons engage: attack enemy ships and each other.
    fn fighter_step(&mut self) {
        // Fighters attack enemy capital ships.
        let atk_fighter_power: u32 = self
            .fighters
            .iter()
            .filter(|f| f.is_attacker && f.alive && f.squad_count > 0)
            .map(|f| f.squad_count)
            .sum();
        let def_fighter_power: u32 = self
            .fighters
            .iter()
            .filter(|f| !f.is_attacker && f.alive && f.squad_count > 0)
            .map(|f| f.squad_count)
            .sum();

        // Attack enemy ships: each squadron's damage = squad_count / 5.
        if atk_fighter_power > 0 {
            let def_ships: Vec<usize> = self
                .ships
                .iter()
                .enumerate()
                .filter(|(_, s)| !s.is_attacker && s.alive)
                .map(|(i, _)| i)
                .collect();
            if !def_ships.is_empty() {
                let target = def_ships[self.combat_tick as usize % def_ships.len()];
                let damage = (atk_fighter_power / 5).cast_signed().max(1);
                // Shields absorb fighter damage first (consistent with auto-resolve path).
                let shield_absorb = damage.min(self.ships[target].shield);
                self.ships[target].shield -= shield_absorb;
                let hull_damage = damage - shield_absorb;
                self.ships[target].hull_current =
                    (self.ships[target].hull_current - hull_damage).max(0);
                if self.ships[target].hull_current == 0 {
                    self.ships[target].alive = false;
                }
            }
        }
        if def_fighter_power > 0 {
            let atk_ships: Vec<usize> = self
                .ships
                .iter()
                .enumerate()
                .filter(|(_, s)| s.is_attacker && s.alive)
                .map(|(i, _)| i)
                .collect();
            if !atk_ships.is_empty() {
                let target = atk_ships[(self.combat_tick as usize + 3) % atk_ships.len()];
                let damage = (def_fighter_power / 5).cast_signed().max(1);
                // Shields absorb fighter damage first (consistent with auto-resolve path).
                let shield_absorb = damage.min(self.ships[target].shield);
                self.ships[target].shield -= shield_absorb;
                let hull_damage = damage - shield_absorb;
                self.ships[target].hull_current =
                    (self.ships[target].hull_current - hull_damage).max(0);
                if self.ships[target].hull_current == 0 {
                    self.ships[target].alive = false;
                }
            }
        }

        // Fighter-vs-fighter attrition.
        if atk_fighter_power > 0 && def_fighter_power > 0 {
            let total = atk_fighter_power + def_fighter_power;
            let atk_loss = (def_fighter_power * 10 / total).max(1);
            let def_loss = (atk_fighter_power * 10 / total).max(1);

            Self::apply_fighter_attrition(&mut self.fighters, true, atk_loss);
            Self::apply_fighter_attrition(&mut self.fighters, false, def_loss);
        }
    }

    fn apply_fighter_attrition(
        fighters: &mut [TacticalFighter],
        is_attacker: bool,
        mut losses: u32,
    ) {
        for f in fighters.iter_mut().rev() {
            if losses == 0 {
                break;
            }
            if f.is_attacker != is_attacker || !f.alive {
                continue;
            }
            let take = losses.min(f.squad_count);
            f.squad_count -= take;
            losses -= take;
            if f.squad_count == 0 {
                f.alive = false;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// TacticalState — top-level UI state
// ---------------------------------------------------------------------------

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
    /// Original Tactical Display faction-wireframe switches.
    pub highlight_alliance: bool,
    pub highlight_empire: bool,
    render_original_participants: bool,
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
            highlight_alliance: true,
            highlight_empire: true,
            render_original_participants: true,
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
        self.highlight_alliance = true;
        self.highlight_empire = true;
        self.render_original_participants = true;
        #[cfg(feature = "interface-test-fixtures")]
        {
            self.suppress_mapped_capital_fallback = false;
            self.suppress_mapped_fighter_fallback = false;
            self.proof_resource_2560 = false;
            self.proof_lod_follows_zoom = false;
        }
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
        self.asset_renderer.disable_original_camera();
    }

    /// Remove mapped participant fallbacks in the browser-only framebuffer control.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn suppress_mapped_capital_fallback(&mut self) {
        self.suppress_mapped_capital_fallback = true;
        self.suppress_mapped_fighter_fallback = true;
    }

    /// End the current battle — clears session. Returns the session for
    /// result processing by the caller.
    pub fn end_battle(&mut self) -> Option<BattleSession> {
        self.dragging_ship = None;
        self.session.take()
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
    /// Toggle combat pause.
    TogglePause,
    /// Set combat speed multiplier.
    SetSpeed(u32),
    /// Retreat selected ships.
    RetreatSelected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TacticalHudControl {
    Pause,
    ZoomIn,
    ZoomOut,
    CameraTarget,
    CameraLeft,
    CameraRight,
    CameraUp,
    CameraDown,
    HighlightAlliance,
    HighlightEmpire,
}

#[derive(Debug, Clone, Copy)]
struct TacticalHudControlSpec {
    control: TacticalHudControl,
    rect: NativeRect,
    hit_resource: u32,
}

const TACTICAL_HUD_CONTROLS: [TacticalHudControlSpec; 10] = [
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
        control: TacticalHudControl::HighlightAlliance,
        rect: NativeRect::new(517.0, 304.0, 30.0, 26.0),
        hit_resource: resources::tactical::DIM_ALLIANCE_SHIPS,
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::HighlightEmpire,
        rect: NativeRect::new(482.0, 304.0, 30.0, 26.0),
        hit_resource: resources::tactical::DIM_EMPIRE_SHIPS,
    },
];

#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "A containing native bitmap rectangle guarantees finite nonnegative local pixels."
)]
fn tactical_hud_control_at(cache: &mut BmpCache, x: f32, y: f32) -> Option<TacticalHudControl> {
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

#[cfg(test)]
fn tactical_hud_rect_control_at(x: f32, y: f32) -> Option<TacticalHudControl> {
    TACTICAL_HUD_CONTROLS
        .iter()
        .find_map(|spec| spec.rect.contains(x, y).then_some(spec.control))
}

fn draw_tactical_bitmap(cache: &mut BmpCache, id: u32, canvas: TacticalCanvas, x: f32, y: f32) {
    let Some(texture) = cache.get_macroquad_original(DllSource::Tactical, id) else {
        return;
    };
    let (screen_x, screen_y) = canvas.point(x, y);
    draw_texture_ex(
        texture,
        screen_x,
        screen_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(
                texture.width() * canvas.scale,
                texture.height() * canvas.scale,
            )),
            ..Default::default()
        },
    );
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
    player_has_fighters: bool,
    selected_name: Option<&str>,
    paused: bool,
    highlight_alliance: bool,
    highlight_empire: bool,
    pressed_control: Option<TacticalHudControl>,
) {
    use resources::tactical as art;

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
        let assigned = index == 0 && selected_name.is_some();
        draw_tactical_bitmap(
            cache,
            if assigned {
                art::BTN_TASK_FORCE_PRESSED
            } else {
                art::BTN_TASK_FORCE_UNASSIGNED
            },
            canvas,
            60.0 + f32::from(index) * 26.0,
            2.0,
        );
        if assigned {
            let (x, y) = canvas.point(69.0, 16.0);
            draw_text("1", x, y, 13.0 * canvas.scale, BLACK);
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
        draw_tactical_bitmap(
            cache,
            if index == 0 && player_has_fighters {
                art::BTN_RED_SQUADRON_NORMAL
            } else {
                art::SQUADRON_UNASSIGNED
            },
            canvas,
            330.0 + f32::from(index) * 26.0,
            2.0,
        );
    }

    draw_tactical_bitmap(
        cache,
        if selected_name.is_some() { 1302 } else { 1301 },
        canvas,
        481.0,
        27.0,
    );
    if let Some(name) = selected_name {
        let (x, y) = canvas.point(497.0, 48.0);
        draw_text(name, x, y, 11.0 * canvas.scale, WHITE);
        let (x, y) = canvas.point(497.0, 222.0);
        draw_text("No Orders", x, y, 11.0 * canvas.scale, WHITE);
    }

    for (id, x) in [(1026, 485.0), (1027, 521.0), (1028, 560.0), (1029, 601.0)] {
        draw_tactical_bitmap(cache, id, canvas, x, 272.0);
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
    draw_tactical_bitmap(cache, 1038, canvas, 606.0, 308.0);

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
}

fn handle_original_tactical_controls(
    state: &mut TacticalState,
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
) -> TacticalAction {
    let (mouse_x, mouse_y) = mouse_position();
    if is_mouse_button_pressed(MouseButton::Left) {
        let (x, y) = canvas.logical_pointer(mouse_x, mouse_y);
        match tactical_hud_control_at(cache, x, y) {
            Some(TacticalHudControl::Pause) => return TacticalAction::TogglePause,
            Some(TacticalHudControl::ZoomIn) => {
                state.zoom = (state.zoom * 1.25).min(2.0);
                state.asset_renderer.zoom_in();
            }
            Some(TacticalHudControl::ZoomOut) => {
                state.zoom = (state.zoom / 1.25).max(0.5);
                state.asset_renderer.zoom_out();
            }
            Some(TacticalHudControl::CameraTarget) => {
                let selected = state.session.as_ref().and_then(|session| {
                    session.selected_ship.and_then(|index| {
                        session
                            .ships
                            .get(index)
                            .map(|ship| (index, ship.x, ship.y, ship.source_position.rendered()))
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
                state.asset_renderer.turn_left();
            }
            Some(TacticalHudControl::CameraRight) => {
                state.asset_renderer.turn_right();
            }
            Some(TacticalHudControl::CameraUp) => {
                state.asset_renderer.pitch_up();
            }
            Some(TacticalHudControl::CameraDown) => {
                state.asset_renderer.pitch_down();
            }
            Some(TacticalHudControl::HighlightAlliance) => {
                state.highlight_alliance = !state.highlight_alliance;
            }
            Some(TacticalHudControl::HighlightEmpire) => {
                state.highlight_empire = !state.highlight_empire;
            }
            None => {}
        }
    }
    if is_key_pressed(KeyCode::Equal) {
        state.zoom = (state.zoom * 1.25).min(2.0);
        state.asset_renderer.zoom_in();
    }
    if is_key_pressed(KeyCode::Minus) {
        state.zoom = (state.zoom / 1.25).max(0.5);
        state.asset_renderer.zoom_out();
    }
    TacticalAction::None
}

fn camera_offset_for_target(x: f32, y: f32) -> (f32, f32) {
    (x - ARENA_WIDTH * 0.5, y - ARENA_HEIGHT * 0.5)
}

/// Draw the tactical combat view (macroquad + egui).
///
/// Call this when `GameMode::TacticalCombat`. Returns a `TacticalAction`
/// indicating whether the main loop should transition modes.
#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
)]
///
/// # Panics
/// Panics if the active battle session disappears during drawing.
pub fn draw_tactical_view(
    state: &mut TacticalState,
    bmp_cache: &mut BmpCache,
    _world: &GameWorld,
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

    // 1. Paint the unscaled-source tactical shell first, then put the battle
    // only inside its original black aperture. Bitmap 1000 includes every
    // frame edge and the empty right-hand panel housing.
    clear_background(BLACK);
    let canvas = TacticalCanvas::new(screen_width(), screen_height());
    draw_tactical_bitmap(bmp_cache, resources::tactical::BACKGROUND, canvas, 0.0, 0.0);
    set_tactical_aperture_clip(Some(canvas.aperture()));
    draw_starfield(canvas);

    // 2. Submit mapped production participants through their original 3D and
    // type-303 resource families. Unmapped or unavailable objects retain the
    // bounded 2D fallback below.
    let (scale, offset_x, offset_y) =
        canvas.arena_transform(state.zoom, state.camera_x, state.camera_y);
    let (production_objects, production_fighters) = if state.render_original_participants {
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
                                close_resource_id: resource.initial_close_resource(player_side),
                                far_resource_id: resource.initial_far_resource(player_side),
                                indicator_resource_id: resource
                                    .initial_indicator_resource(player_side),
                                position: fighter.source_position.rendered(),
                            }
                        })
                    })
                    .collect::<Vec<_>>();
                (ships, fighters)
            })
            .unwrap_or_default()
    } else {
        (Vec::new(), Vec::new())
    };
    let aperture = canvas.aperture();
    let aperture_tuple = (aperture.x, aperture.y, aperture.width, aperture.height);
    let ship_report = state
        .asset_renderer
        .draw_participants(aperture_tuple, &production_objects);
    let fighter_report = state
        .asset_renderer
        .draw_fighters(aperture_tuple, &production_fighters);
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
    let system_name = session.system_name.clone();
    let player_is_attacker = session.player_is_attacker;
    let selected_ship = session.selected_ship;
    let combat_tick = session.combat_tick;
    let paused = session.paused;
    let combat_speed = session.combat_speed;
    let winner = session.winner;

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
        if ship.focus_target.is_some() && ship.is_attacker == player_is_attacker {
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

    // 6. Draw weapon fire effects (laser lines between ships).
    for effect in &session.weapon_effects {
        if let (Some(src), Some(tgt)) = (
            session.ships.get(effect.source),
            session.ships.get(effect.target),
        ) {
            let source = tactical_ship_screen_position(
                effect.source,
                src,
                &ship_projections,
                scale,
                offset_x,
                offset_y,
            );
            let target = tactical_ship_screen_position(
                effect.target,
                tgt,
                &ship_projections,
                scale,
                offset_x,
                offset_y,
            );

            let base_color = effect.kind.color();
            let alpha = (f32::from(effect.ttl) / 8.0).min(1.0);
            let color = Color::new(base_color.r, base_color.g, base_color.b, alpha);

            // Main beam.
            draw_line(source.x, source.y, target.x, target.y, 2.0, color);

            // Impact flash at target (brief bright circle).
            if effect.ttl > 5 {
                let flash_r = 4.0 + f32::from(8 - effect.ttl) * 2.0;
                draw_circle(
                    target.x,
                    target.y,
                    flash_r,
                    Color::new(1.0, 1.0, 0.8, alpha * 0.6),
                );
            }
        }
    }

    // 6b. Draw targeting lines from player ships to their focus targets.
    if phase == BattlePhase::Combat {
        for (ship_index, ship) in session.ships.iter().enumerate() {
            if !ship.alive || !ship.selected {
                continue;
            }
            if ship.is_attacker != player_is_attacker {
                continue;
            }
            if let Some(ft_idx) = ship.focus_target {
                if let Some(target) = session.ships.get(ft_idx) {
                    if target.alive {
                        let source = tactical_ship_screen_position(
                            ship_index,
                            ship,
                            &ship_projections,
                            scale,
                            offset_x,
                            offset_y,
                        );
                        let target = tactical_ship_screen_position(
                            ft_idx,
                            target,
                            &ship_projections,
                            scale,
                            offset_x,
                            offset_y,
                        );
                        // Dashed targeting line.
                        draw_line(
                            source.x,
                            source.y,
                            target.x,
                            target.y,
                            1.0,
                            Color::new(1.0, 0.5, 0.0, 0.4),
                        );
                        // Target reticle on enemy.
                        let r = ship_projections.get(&ft_idx).map_or(
                            DEFAULT_SHIP_SIZE * scale * 0.4,
                            |projection| {
                                (projection.max - projection.min).max_element() * 0.6 + 3.0
                            },
                        );
                        draw_circle_lines(
                            target.x,
                            target.y,
                            r,
                            1.5,
                            Color::new(1.0, 0.3, 0.0, 0.7),
                        );
                    }
                }
            }
        }
    }

    // Collect ship info for the selected ship before mutable borrow.
    let selected_info = selected_ship
        .and_then(|idx| session.ships.get(idx))
        .map(|s| {
            (
                s.name.clone(),
                s.sprite_id,
                s.hull_current,
                s.hull_max,
                s.shield,
                s.shield_max,
                s.alive,
            )
        });
    let player_ship_count = session
        .ships
        .iter()
        .filter(|s| s.is_attacker == player_is_attacker && s.alive)
        .count();
    let enemy_ship_count = session
        .ships
        .iter()
        .filter(|s| s.is_attacker != player_is_attacker && s.alive)
        .count();

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
            scale,
            offset_x,
            offset_y,
            canvas.aperture(),
            &ship_projections,
        );
    }

    if phase == BattlePhase::Combat {
        action = handle_original_tactical_controls(state, bmp_cache, canvas);
        let pressed_control = pressed_tactical_hud_control(bmp_cache, canvas);
        let player_is_alliance = state
            .session
            .as_ref()
            .is_some_and(|session| session.player_is_attacker == session.attacker_is_alliance);
        let player_has_fighters = state.session.as_ref().is_some_and(|session| {
            session
                .fighters
                .iter()
                .any(|fighter| fighter.is_attacker == session.player_is_attacker && fighter.alive)
        });
        draw_original_tactical_hud(
            bmp_cache,
            canvas,
            player_is_alliance,
            player_has_fighters,
            selected_info.as_ref().map(|(name, ..)| name.as_str()),
            paused,
            state.highlight_alliance,
            state.highlight_empire,
            pressed_control,
        );
        if paused {
            let (x, y) = canvas.point(25.0, 48.0);
            draw_text("Battle Paused.", x, y, 16.0 * canvas.scale, RED);
        }
    }

    // The original battle-results composition is a separate pending surface.
    // Preserve its existing functional route without painting replacement HUD
    // panels over the active bitmap-driven combat view.
    if phase == BattlePhase::Results {
        egui_macroquad::ui(|ctx| {
            // Task force info panel overlay (top-left floating window).
            // Shows the faction-colored HUD panel from TACTICAL.DLL for both sides.
            egui::Window::new("task_force_hud")
                .title_bar(false)
                .resizable(false)
                .collapsible(false)
                .frame(egui::Frame::NONE)
                .fixed_pos(egui::pos2(8.0, 40.0))
                .show(ctx, |ui| {
                    let atk_panel_id = if player_is_attacker {
                        TACTICAL_TASKFORCE_PANEL_ATTACKER
                    } else {
                        TACTICAL_TASKFORCE_PANEL_DEFENDER
                    };
                    let def_panel_id = if player_is_attacker {
                        TACTICAL_TASKFORCE_PANEL_DEFENDER
                    } else {
                        TACTICAL_TASKFORCE_PANEL_ATTACKER
                    };

                    // Attacker panel (player side).
                    if let Some(tex) = bmp_cache.get(ctx, DllSource::Tactical, atk_panel_id) {
                        let size = tex.size();
                        let w = (size[0] as f32).min(160.0);
                        let h = w * size[1] as f32 / size[0] as f32;
                        ui.add(egui::Image::new(egui::load::SizedTexture::new(
                            tex.id(),
                            Vec2::new(w, h),
                        )));
                    }

                    ui.add_space(4.0);

                    // Defender panel (enemy side).
                    if let Some(tex) = bmp_cache.get(ctx, DllSource::Tactical, def_panel_id) {
                        let size = tex.size();
                        let w = (size[0] as f32).min(160.0);
                        let h = w * size[1] as f32 / size[0] as f32;
                        ui.add(egui::Image::new(egui::load::SizedTexture::new(
                            tex.id(),
                            Vec2::new(w, h),
                        )));
                    }
                });

            // Top bar: battle title.
            egui::TopBottomPanel::top("tactical_top").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(
                        RichText::new(format!("Battle of {system_name}"))
                            .color(Color32::from_rgb(255, 200, 60))
                            .strong(),
                    );
                    ui.separator();
                    ui.label(
                        RichText::new(match phase {
                            BattlePhase::Placement => "DEPLOYMENT PHASE",
                            BattlePhase::Combat => "COMBAT",
                            BattlePhase::Results => "BATTLE RESULTS",
                        })
                        .color(Color32::from_rgb(200, 200, 200)),
                    );
                });
            });

            // Bottom panel: phase controls.
            egui::TopBottomPanel::bottom("tactical_bottom").show(ctx, |ui| {
            ui.horizontal(|ui| {
                match phase {
                    BattlePhase::Placement => {
                        ui.label(
                            RichText::new(format!(
                                "Your ships: {player_ship_count}  |  Enemy ships: {enemy_ship_count}",
                            ))
                            .color(Color32::from_rgb(180, 180, 180)),
                        );
                        ui.separator();
                        ui.label(
                            RichText::new("Drag your ships to reposition. Fighters auto-deploy.")
                                .color(Color32::from_rgb(140, 140, 140))
                                .italics(),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .button(
                                    RichText::new("Begin Battle")
                                        .color(Color32::from_rgb(60, 220, 60))
                                        .strong(),
                                )
                                .clicked()
                            {
                                action = TacticalAction::BeginCombat;
                            }
                            if ui
                                .button(
                                    RichText::new("Auto-Resolve")
                                        .color(Color32::from_rgb(200, 200, 100)),
                                )
                                .clicked()
                            {
                                action = TacticalAction::AutoResolve;
                            }
                        });
                    }
                    BattlePhase::Combat => {
                        ui.label(
                            RichText::new(format!(
                                "Your ships: {player_ship_count}  |  Enemy ships: {enemy_ship_count}  |  Tick: {combat_tick}",
                            ))
                            .color(Color32::from_rgb(180, 180, 180)),
                        );
                        ui.separator();

                        let status_text = if paused { "PAUSED" } else { "COMBAT" };
                        let status_color = if paused {
                            Color32::from_rgb(200, 200, 60)
                        } else {
                            Color32::from_rgb(255, 140, 60)
                        };
                        ui.label(RichText::new(status_text).color(status_color).strong());

                        // Weapon recharge gauge: 5-step animation from TACTICAL.DLL.
                        // Cycles through IDs 1206-1210 based on combat tick.
                        // Each full cycle = 20 ticks (4 ticks per step).
                        let gauge_step = (combat_tick / 4) % TACTICAL_RECHARGE_GAUGE_STEPS;
                        let gauge_id = TACTICAL_RECHARGE_GAUGE_BASE + gauge_step;
                        if let Some(tex) = bmp_cache.get(ctx, DllSource::Tactical, gauge_id) {
                            let size = tex.size();
                            let h = 20.0_f32;
                            let w = h * size[0] as f32 / size[1] as f32;
                            ui.add(egui::Image::new(egui::load::SizedTexture::new(
                                tex.id(),
                                Vec2::new(w, h),
                            )));
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            // Speed controls.
                            let speed_label = format!("{combat_speed}x");
                            if ui
                                .button(
                                    RichText::new("Faster").color(Color32::from_rgb(120, 200, 120)),
                                )
                                .clicked()
                            {
                                action = TacticalAction::SetSpeed(combat_speed.min(2) * 2);
                            }
                            ui.label(
                                RichText::new(speed_label)
                                    .color(Color32::from_rgb(200, 200, 200))
                                    .strong(),
                            );
                            if ui
                                .button(
                                    RichText::new("Slower").color(Color32::from_rgb(200, 200, 120)),
                                )
                                .clicked()
                            {
                                action = TacticalAction::SetSpeed((combat_speed / 2).max(1));
                            }
                            ui.separator();

                            // Pause/resume.
                            // Retreat selected ships.
                            if ui
                                .button(
                                    RichText::new("Retreat Selected")
                                        .color(Color32::from_rgb(220, 120, 60)),
                                )
                                .clicked()
                            {
                                action = TacticalAction::RetreatSelected;
                            }
                            ui.separator();
                            let pause_label = if paused { "Resume" } else { "Pause" };
                            if ui
                                .button(
                                    RichText::new(pause_label)
                                        .color(Color32::from_rgb(200, 180, 60)),
                                )
                                .clicked()
                            {
                                action = TacticalAction::TogglePause;
                            }
                        });
                    }
                    BattlePhase::Results => {
                        // Show winner.
                        let (winner_text, winner_color) = match winner {
                            Some(CombatWinner::Attacker) => {
                                if player_is_attacker {
                                    ("VICTORY!", Color32::from_rgb(60, 220, 60))
                                } else {
                                    ("DEFEAT", Color32::from_rgb(220, 60, 60))
                                }
                            }
                            Some(CombatWinner::Defender) => {
                                if player_is_attacker {
                                    ("DEFEAT", Color32::from_rgb(220, 60, 60))
                                } else {
                                    ("VICTORY!", Color32::from_rgb(60, 220, 60))
                                }
                            }
                            Some(CombatWinner::Draw) | None => {
                                ("DRAW", Color32::from_rgb(200, 200, 60))
                            }
                        };
                        ui.label(
                            RichText::new(winner_text)
                                .color(winner_color)
                                .strong()
                                .size(16.0),
                        );
                        ui.separator();
                        ui.label(
                            RichText::new(format!(
                                "Your ships: {player_ship_count} remaining  |  Enemy ships: {enemy_ship_count} remaining",
                            ))
                            .color(Color32::from_rgb(180, 180, 180)),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .button(
                                    RichText::new("Return to Galaxy")
                                        .color(Color32::from_rgb(200, 200, 60))
                                        .strong(),
                                )
                                .clicked()
                            {
                                action = TacticalAction::ReturnToGalaxy;
                            }
                        });
                    }
                }
            });
        });

            // Right panel: selected ship info.
            if let Some((ref name, sprite_id, hull_current, hull_max, shield, shield_max, alive)) =
                selected_info
            {
                egui::SidePanel::right("tactical_ship_info")
                    .default_width(200.0)
                    .show(ctx, |ui| {
                        ui.heading(RichText::new(name).color(Color32::from_rgb(255, 220, 100)));
                        ui.separator();

                        // Hull/shield display panel background (TACTICAL.DLL ID 1302).
                        if let Some(tex) =
                            bmp_cache.get(ctx, DllSource::Tactical, TACTICAL_HULL_SHIELD_PANEL)
                        {
                            let size = tex.size();
                            let w = 180.0_f32.min(size[0] as f32);
                            let h = w * size[1] as f32 / size[0] as f32;
                            ui.add(egui::Image::new(egui::load::SizedTexture::new(
                                tex.id(),
                                Vec2::new(w, h),
                            )));
                            ui.add_space(2.0);
                        }

                        // Try to show ship sprite from BmpCache.
                        if let Some(sid) = sprite_id {
                            if let Some(tex) = bmp_cache.get(ctx, DllSource::Tactical, sid) {
                                let size = tex.size();
                                let aspect = size[0] as f32 / size[1] as f32;
                                let display_w = 180.0_f32.min(size[0] as f32);
                                let display_h = display_w / aspect;
                                ui.image(egui::ImageSource::Texture(
                                    egui::load::SizedTexture::new(
                                        tex.id(),
                                        Vec2::new(display_w, display_h),
                                    ),
                                ));
                                ui.add_space(4.0);
                            }
                        }

                        ui.horizontal(|ui| {
                            ui.label("Hull:");
                            let hull_color =
                                if hull_max > 0 && hull_current as f32 / hull_max as f32 > 0.5 {
                                    Color32::from_rgb(100, 220, 100)
                                } else {
                                    Color32::from_rgb(220, 100, 60)
                                };
                            ui.label(
                                RichText::new(format!("{hull_current}/{hull_max}"))
                                    .color(hull_color),
                            );
                        });

                        if shield_max > 0 {
                            ui.horizontal(|ui| {
                                ui.label("Shields:");
                                ui.label(
                                    RichText::new(format!("{shield}/{shield_max}"))
                                        .color(Color32::from_rgb(100, 150, 255)),
                                );
                            });
                        }

                        ui.horizontal(|ui| {
                            ui.label("Status:");
                            ui.label(
                                RichText::new(if alive { "Active" } else { "Destroyed" }).color(
                                    if alive {
                                        Color32::from_rgb(100, 220, 100)
                                    } else {
                                        Color32::from_rgb(220, 60, 60)
                                    },
                                ),
                            );
                        });
                    });
            }
        });
        egui_macroquad::draw();
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
    scale: f32,
    offset_x: f32,
    offset_y: f32,
    aperture: NativeRect,
    projections: &HashMap<usize, TacticalScreenProjection>,
) {
    let (mx, my) = mouse_position();
    let inside_battle = aperture.contains(mx, my);
    let arena_pointer_x = (mx - offset_x) / scale;
    let arena_pointer_y = (my - offset_y) / scale;
    let hit_radius = DEFAULT_SHIP_SIZE * 0.6;
    let screen_pointer = macroquad::math::Vec2::new(mx, my);
    let projected_hit_radius = 7.0 * (aperture.width / BATTLE_APERTURE.width);

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
            // Toggle selection with shift, or single-select.
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                session.ships[idx].selected = !session.ships[idx].selected;
            } else {
                for s in &mut session.ships {
                    s.selected = false;
                }
                session.ships[idx].selected = true;
            }
            session.selected_ship = Some(idx);
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
        }
    }

    // Right-click: issue focus-fire order to selected ships.
    if inside_battle && is_mouse_button_pressed(MouseButton::Right) {
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

        if let Some(target_idx) = target_hit {
            // Assign focus-fire to all selected player ships.
            let mut assigned = Vec::new();
            for ship in &mut session.ships {
                if ship.selected && ship.is_attacker == session.player_is_attacker && ship.alive {
                    ship.focus_target = Some(target_idx);
                }
            }
            for (index, ship) in session.ships.iter().enumerate() {
                if ship.selected && ship.focus_target == Some(target_idx) {
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
        } else {
            // Right-clicked empty space — clear focus targets for selected ships.
            for ship in &mut session.ships {
                if ship.selected && ship.is_attacker == session.player_is_attacker {
                    ship.focus_target = None;
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
// Starfield background
// ---------------------------------------------------------------------------

/// Draw a simple starfield background.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
)]
fn draw_starfield(canvas: TacticalCanvas) {
    // A deterministic interim field inside the bitmap's black viewport. The
    // original 3D star renderer is still a separate TAC-01 parity requirement.
    let aperture = canvas.aperture();
    let star_count = 120;
    let mut seed: u64 = 0xDEAD_BEEF;

    for _ in 0..star_count {
        seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
        let x = aperture.x + (seed % 10_000) as f32 / 10_000.0 * aperture.width;
        seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
        let y = aperture.y + (seed % 10_000) as f32 / 10_000.0 * aperture.height;
        seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
        let brightness = 0.3 + (seed % 70) as f32 / 100.0;

        draw_circle(
            x,
            y,
            canvas.scale.max(0.5),
            Color::new(brightness, brightness, brightness * 1.1, 1.0),
        );
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
            name: format!("ship-{dat_id}"),
            x: 0.0,
            y: 0.0,
            hull_current: 1,
            hull_max: 1,
            shield: 0,
            shield_max: 0,
            is_attacker: is_alliance,
            alive,
            selected: false,
            fleet_ship_index: roster,
            sprite_id: None,
            turbolaser_power: 0,
            ion_cannon_power: 0,
            laser_cannon_power: 0,
            focus_target: None,
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
            fleet_fighter_index: 0,
            name: format!("fighter-{dat_id}"),
            x: 0.0,
            y: 0.0,
            squad_count: 12,
            is_attacker: is_alliance,
            alive: true,
        }
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
