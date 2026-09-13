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

use egui_macroquad::egui::{self, Color32, RichText, Vec2};
use macroquad::prelude::*;
use rebellion_core::ids::{CapitalShipKey, FighterKey, FleetKey, SystemKey};
use rebellion_core::world::GameWorld;

use crate::bmp_cache::{resources, BmpCache, DllSource};
#[cfg(feature = "interface-test-fixtures")]
use crate::tactical_assets::TacticalProofRenderer;

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

/// Ship sprite IDs in TACTICAL.DLL (resource range 2001-2130).
const TACTICAL_SHIP_SPRITE_START: u32 = 2001;
const TACTICAL_SHIP_SPRITE_END: u32 = 2130;

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

        let mut ships = Vec::new();
        let mut fighters = Vec::new();

        // Expand attacker fleet into individual hulls.
        Self::expand_fleet(world, attacker, true, &mut ships, &mut fighters);
        // Expand defender fleet.
        Self::expand_fleet(world, defender, false, &mut ships, &mut fighters);

        // Auto-place all ships in deployment zones.
        Self::auto_place_ships(&mut ships);
        Self::auto_place_fighters(&mut fighters, &ships);

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
            attacker_fleet: attacker,
            defender_fleet: defender,
            attacker_is_alliance: world.fleets[attacker].is_alliance,
            player_is_attacker,
            phase: BattlePhase::Combat,
            ships,
            fighters,
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
        for (ship_idx, ship) in fleet.capital_ships.iter().filter(|s| s.alive).enumerate() {
            let class = &world.capital_ship_classes[ship.class];
            let sprite_id = Self::class_to_sprite_id(class.dat_id.index());

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
                sprite_id,
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

    /// Map a ship class `DatId` index to a TACTICAL.DLL sprite resource ID.
    ///
    /// The original game uses a lookup table; we approximate with a linear
    /// mapping into the 2001-2130 range. Each class gets ~2 sprites
    /// (different orientations) — use the base orientation.
    fn class_to_sprite_id(class_index: u32) -> Option<u32> {
        let id = TACTICAL_SHIP_SPRITE_START + (class_index * 2);
        if id <= TACTICAL_SHIP_SPRITE_END {
            Some(id)
        } else {
            None
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
    #[cfg(feature = "interface-test-fixtures")]
    proof_resource_2560: bool,
    #[cfg(feature = "interface-test-fixtures")]
    proof_renderer: TacticalProofRenderer,
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
            #[cfg(feature = "interface-test-fixtures")]
            proof_resource_2560: false,
            #[cfg(feature = "interface-test-fixtures")]
            proof_renderer: TacticalProofRenderer::default(),
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
        self.session = Some(BattleSession::new(
            world,
            system,
            attacker,
            defender,
            player_is_attacker,
            tick,
        ));
        self.dragging_ship = None;
        self.drag_offset = (0.0, 0.0);
        self.camera_x = 0.0;
        self.camera_y = 0.0;
        self.zoom = 1.0;
        self.highlight_alliance = true;
        self.highlight_empire = true;
    }

    /// Enable the source-bound P56 mesh proof in isolated fixture builds.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn enable_resource_2560_proof(&mut self) {
        self.proof_resource_2560 = true;
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
    HighlightAlliance,
    HighlightEmpire,
}

#[derive(Debug, Clone, Copy)]
struct TacticalHudControlSpec {
    control: TacticalHudControl,
    rect: NativeRect,
    hit_resource: u32,
}

const TACTICAL_HUD_CONTROLS: [TacticalHudControlSpec; 5] = [
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
        (1048, 511.0, 376.0),
        (1050, 557.0, 376.0),
        (1052, 537.0, 344.0),
        (1055, 537.0, 412.0),
        (1058, 538.0, 379.0),
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
            Some(TacticalHudControl::ZoomIn) => state.zoom = (state.zoom * 1.25).min(2.0),
            Some(TacticalHudControl::ZoomOut) => state.zoom = (state.zoom / 1.25).max(0.5),
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
    }
    if is_key_pressed(KeyCode::Minus) {
        state.zoom = (state.zoom / 1.25).max(0.5);
    }
    TacticalAction::None
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

    // 2. Transform the provisional 2D battle into the original aperture.
    let (scale, offset_x, offset_y) =
        canvas.arena_transform(state.zoom, state.camera_x, state.camera_y);

    #[cfg(feature = "interface-test-fixtures")]
    if state.proof_resource_2560 {
        let aperture = canvas.aperture();
        state.proof_renderer.draw(
            bmp_cache,
            (aperture.x, aperture.y, aperture.width, aperture.height),
        );
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
    for ship in &session.ships {
        if !ship.alive {
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

    // 5. Draw fighter squadrons.
    for fighter in &session.fighters {
        if !fighter.alive {
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
            let sx = offset_x + src.x * scale;
            let sy = offset_y + src.y * scale;
            let tx = offset_x + tgt.x * scale;
            let ty = offset_y + tgt.y * scale;

            let base_color = effect.kind.color();
            let alpha = (f32::from(effect.ttl) / 8.0).min(1.0);
            let color = Color::new(base_color.r, base_color.g, base_color.b, alpha);

            // Main beam.
            draw_line(sx, sy, tx, ty, 2.0, color);

            // Impact flash at target (brief bright circle).
            if effect.ttl > 5 {
                let flash_r = 4.0 + f32::from(8 - effect.ttl) * 2.0;
                draw_circle(tx, ty, flash_r, Color::new(1.0, 1.0, 0.8, alpha * 0.6));
            }
        }
    }

    // 6b. Draw targeting lines from player ships to their focus targets.
    if phase == BattlePhase::Combat {
        for ship in &session.ships {
            if !ship.alive || !ship.selected {
                continue;
            }
            if ship.is_attacker != player_is_attacker {
                continue;
            }
            if let Some(ft_idx) = ship.focus_target {
                if let Some(target) = session.ships.get(ft_idx) {
                    if target.alive {
                        let sx = offset_x + ship.x * scale;
                        let sy = offset_y + ship.y * scale;
                        let tx = offset_x + target.x * scale;
                        let ty = offset_y + target.y * scale;
                        // Dashed targeting line.
                        draw_line(sx, sy, tx, ty, 1.0, Color::new(1.0, 0.5, 0.0, 0.4));
                        // Target reticle on enemy.
                        let r = DEFAULT_SHIP_SIZE * scale * 0.4;
                        draw_circle_lines(tx, ty, r, 1.5, Color::new(1.0, 0.3, 0.0, 0.7));
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
        handle_combat_input(session, scale, offset_x, offset_y, canvas.aperture());
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
) {
    let (mx, my) = mouse_position();
    let inside_battle = aperture.contains(mx, my);
    let arena_pointer_x = (mx - offset_x) / scale;
    let arena_pointer_y = (my - offset_y) / scale;
    let hit_radius = DEFAULT_SHIP_SIZE * 0.6;

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
            let dx = arena_pointer_x - ship.x;
            let dy = arena_pointer_y - ship.y;
            if dx * dx + dy * dy < hit_radius * hit_radius {
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
            let dx = arena_pointer_x - ship.x;
            let dy = arena_pointer_y - ship.y;
            if dx * dx + dy * dy < hit_radius * hit_radius {
                target_hit = Some(i);
                break;
            }
        }

        if let Some(target_idx) = target_hit {
            // Assign focus-fire to all selected player ships.
            for ship in &mut session.ships {
                if ship.selected && ship.is_attacker == session.player_is_attacker && ship.alive {
                    ship.focus_target = Some(target_idx);
                }
            }
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

    #[test]
    fn class_to_sprite_id_in_range() {
        assert_eq!(BattleSession::class_to_sprite_id(0), Some(2001));
        assert_eq!(BattleSession::class_to_sprite_id(10), Some(2021));
        // Beyond range returns None.
        assert_eq!(BattleSession::class_to_sprite_id(200), None);
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
            tactical_hud_control_at(&mut cache, 518.1, 305.1),
            Some(TacticalHudControl::HighlightAlliance)
        );
        assert_eq!(
            tactical_hud_control_at(&mut cache, 483.1, 305.1),
            Some(TacticalHudControl::HighlightEmpire)
        );
        assert_eq!(tactical_hud_control_at(&mut cache, 588.0, 317.0), None);
    }
}
