//! Cockpit frame rendering — the chrome border around the galaxy map.
//!
//! Renders a faction-specific decorative border that frames the galaxy map
//! area, replicating the "cockpit" aesthetic of the original game's strategy
//! view.  When BMP assets are staged (`data/base/ui/`), the background texture
//! is loaded via `BmpCache`; otherwise a styled fallback is drawn using theme
//! colors and macroquad primitives.
//!
//! # Screen layout
//!
//! ```text
//! ┌──────────────────────────────────────────────────┐
//! │  [top bar: faction logo + status]                │
//! │                                                  │
//! │  ┌────────────────────────────────────────────┐  │
//! │  │                                            │  │
//! │  │         GALAXY MAP VIEWPORT                │  │
//! │  │                                            │  │
//! │  └────────────────────────────────────────────┘  │
//! │                                                  │
//! │  [bottom bar: cockpit control buttons]           │
//! └──────────────────────────────────────────────────┘
//! ```
//!
//! The original command center is a fixed 640×480 composition. Wider or taller
//! browser windows therefore letterbox one uniformly scaled canvas rather than
//! stretching independent layers. The returned `CockpitViewport` is the exact
//! faction aperture recovered from `FUN_00421c70`.
//!
//! # BMP resource IDs
//!
//! | DLL | ID | Content |
//! |-----|----|---------|
//! | STRATEGY | 900 | Alliance command-center shell (640×481 source, 640×480 display) |
//! | STRATEGY | 901 | Imperial command-center shell (640×481 source, 640×480 display) |
//! | COMMON | 20001 | Main-menu background (640×480) |
//! | COMMON | 11001-11275 | Animated cockpit display sequences, not a sequential logical-button map |
//!
//! The bitmap shells and aperture geometry are faction-specific. No synthetic
//! top or bottom chrome is drawn underneath them.

use egui_macroquad::egui;
use macroquad::prelude::*;

use crate::bmp_cache::{resources, BmpCache, DllSource};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Which player faction owns this cockpit chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CockpitFaction {
    Alliance,
    Empire,
}

/// Logical width of the original strategic command-center surface.
pub const STRATEGIC_LOGICAL_WIDTH: f32 = 640.0;

/// Logical height displayed by the original strategic command center.
///
/// The recovered STRATEGY resources contain one extra source row. It is not
/// part of the displayed 640×480 composition.
pub const STRATEGIC_LOGICAL_HEIGHT: f32 = 480.0;

/// Original strategic command-control identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CockpitButton {
    /// Find a known star system (F2).
    SystemFinder,
    /// Find a fleet or ship (F3).
    FleetFinder,
    /// Find a troop unit (F4).
    TroopFinder,
    /// Find a character or special force (F5).
    PersonnelFinder,
    /// Open the original game-options destination (F7).
    GameOptions,
    /// Open the Encyclopedia.
    Encyclopedia,
    /// Open the Galactic Information Display menu.
    GalacticInformationDisplay,
}

/// Strategic map overlay selected through the original GID menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GidMode {
    /// Default campaign view, sized by the player's popular support.
    PopularSupport,
    Uprisings,
    IdleFleets,
    FleetsEnRoute,
    IdlePersonnel,
    ActivePersonnel,
    AvailableEnergy,
    AvailableRawMaterial,
    Mines,
    Refineries,
    Shipyards,
    IdleShipyards,
    TrainingFacilities,
    IdleTrainingFacilities,
    ConstructionYards,
    IdleConstructionYards,
    Troopers,
    FighterSquadrons,
    DeathStarShields,
    PlanetaryShieldGenerators,
    PlanetaryDefenseBatteries,
    /// Native Display Off item, which removes markers and uses bright resource 902.
    DisplayOff,
}

impl GidMode {
    /// Native command identifier dispatched by the code-built GID menu.
    #[must_use]
    pub const fn command_id(self) -> u8 {
        match self {
            Self::PopularSupport => 0x11,
            Self::Uprisings => 0x12,
            Self::IdleFleets => 0x21,
            Self::FleetsEnRoute => 0x22,
            Self::IdlePersonnel => 0x43,
            Self::ActivePersonnel => 0x44,
            Self::AvailableEnergy => 0x51,
            Self::AvailableRawMaterial => 0x52,
            Self::Mines => 0x53,
            Self::Refineries => 0x54,
            Self::Shipyards => 0x62,
            Self::IdleShipyards => 0x65,
            Self::TrainingFacilities => 0x63,
            Self::IdleTrainingFacilities => 0x66,
            Self::ConstructionYards => 0x64,
            Self::IdleConstructionYards => 0x67,
            Self::Troopers => 0x71,
            Self::FighterSquadrons => 0x72,
            Self::DeathStarShields => 0x73,
            Self::PlanetaryShieldGenerators => 0x74,
            Self::PlanetaryDefenseBatteries => 0x75,
            Self::DisplayOff => 0x80,
        }
    }

    /// English text carried by TEXTSTRA.DLL for the selected display.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::PopularSupport => "Popular Support",
            Self::Uprisings => "Uprisings",
            Self::IdleFleets => "Idle Fleets",
            Self::FleetsEnRoute => "Fleets Enroute",
            Self::IdlePersonnel => "Idle Personnel",
            Self::ActivePersonnel => "Active Personnel",
            Self::AvailableEnergy => "Available Energy",
            Self::AvailableRawMaterial => "Available Raw Material",
            Self::Mines => "Mines",
            Self::Refineries => "Refineries",
            Self::Shipyards => "Shipyards",
            Self::IdleShipyards => "Idle Shipyards",
            Self::TrainingFacilities => "Training Facilities",
            Self::IdleTrainingFacilities => "Idle Training Facilities",
            Self::ConstructionYards => "Construction Yards",
            Self::IdleConstructionYards => "Idle Construction Yards",
            Self::Troopers => "Troopers",
            Self::FighterSquadrons => "Fighter Squadrons",
            Self::DeathStarShields => "Death Star Shields",
            Self::PlanetaryShieldGenerators => "Planetary Shield Generators",
            Self::PlanetaryDefenseBatteries => "Planetary Defense Batteries",
            Self::DisplayOff => "Display Off",
        }
    }

    #[must_use]
    pub const fn is_active(self) -> bool {
        !matches!(self, Self::DisplayOff)
    }
}

/// Root branches built by `FUN_004511e0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GidCategory {
    Loyalty,
    Fleets,
    Personnel,
    Resources,
    Manufacturing,
    Defense,
}

impl GidCategory {
    const ALL: [Self; 6] = [
        Self::Loyalty,
        Self::Fleets,
        Self::Personnel,
        Self::Resources,
        Self::Manufacturing,
        Self::Defense,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::Loyalty => "Loyalty",
            Self::Fleets => "Fleets",
            Self::Personnel => "Personnel",
            Self::Resources => "Resources",
            Self::Manufacturing => "Manufacturing",
            Self::Defense => "Defense",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GidMenuItem {
    mode: GidMode,
    resource_id: u32,
}

/// State for the original code-built GID menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GidUiState {
    pub menu_open: bool,
    pub category: Option<GidCategory>,
}

/// Recovered native control record for one strategic command button.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StrategicControlSpec {
    pub button: CockpitButton,
    pub command_id: u16,
    pub rect: CockpitViewport,
    pub normal_resource: u32,
    pub pressed_resource: u32,
}

/// One original Message Index category control on the command-center rail.
///
/// `FUN_00427270` constructs these nine 27x22 controls for each faction.
/// The resting and illuminated resources are separate; the latter is not
/// selected until the original message-state predicate is recovered.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MessageIndexControlSpec {
    pub command_id: u16,
    pub rect: CockpitViewport,
    pub resting_resource: u32,
    pub illuminated_resource: u32,
}

/// Pixel viewport the galaxy map should render into.
///
/// All coordinates are in macroquad screen pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CockpitViewport {
    /// Left edge of the usable map area (pixels from left).
    pub x: f32,
    /// Top edge of the usable map area (pixels from top).
    pub y: f32,
    /// Width of the usable map area in pixels.
    pub width: f32,
    /// Height of the usable map area in pixels.
    pub height: f32,
}

impl CockpitViewport {
    /// Viewport that fills the entire screen (no cockpit chrome).
    #[must_use]
    pub fn fullscreen() -> Self {
        CockpitViewport {
            x: 0.0,
            y: 0.0,
            width: screen_width(),
            height: screen_height(),
        }
    }

    /// Right edge in screen pixels.
    #[must_use]
    pub fn right(self) -> f32 {
        self.x + self.width
    }

    /// Bottom edge in screen pixels.
    #[must_use]
    pub fn bottom(self) -> f32 {
        self.y + self.height
    }

    /// Whether a screen-space point lies inside this viewport.
    #[must_use]
    pub fn contains(self, x: f32, y: f32) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }
}

const ALLIANCE_PRIMARY_CONTROLS: [StrategicControlSpec; 6] = [
    StrategicControlSpec {
        button: CockpitButton::SystemFinder,
        command_id: 0x12d,
        rect: CockpitViewport {
            x: 106.0,
            y: 408.0,
            width: 27.0,
            height: 16.0,
        },
        normal_resource: resources::strategy::ALLIANCE_SYSTEM_FINDER_NORMAL,
        pressed_resource: resources::strategy::ALLIANCE_SYSTEM_FINDER_PRESSED,
    },
    StrategicControlSpec {
        button: CockpitButton::FleetFinder,
        command_id: 0x12e,
        rect: CockpitViewport {
            x: 157.0,
            y: 407.0,
            width: 27.0,
            height: 15.0,
        },
        normal_resource: resources::strategy::ALLIANCE_FLEET_FINDER_NORMAL,
        pressed_resource: resources::strategy::ALLIANCE_FLEET_FINDER_PRESSED,
    },
    StrategicControlSpec {
        button: CockpitButton::PersonnelFinder,
        command_id: 0x12f,
        rect: CockpitViewport {
            x: 258.0,
            y: 405.0,
            width: 27.0,
            height: 16.0,
        },
        normal_resource: resources::strategy::ALLIANCE_PERSONNEL_FINDER_NORMAL,
        pressed_resource: resources::strategy::ALLIANCE_PERSONNEL_FINDER_PRESSED,
    },
    StrategicControlSpec {
        button: CockpitButton::TroopFinder,
        command_id: 0x130,
        rect: CockpitViewport {
            x: 209.0,
            y: 405.0,
            width: 27.0,
            height: 16.0,
        },
        normal_resource: resources::strategy::ALLIANCE_TROOP_FINDER_NORMAL,
        pressed_resource: resources::strategy::ALLIANCE_TROOP_FINDER_PRESSED,
    },
    StrategicControlSpec {
        button: CockpitButton::GameOptions,
        command_id: 0x131,
        rect: CockpitViewport {
            x: 394.0,
            y: 405.0,
            width: 27.0,
            height: 16.0,
        },
        normal_resource: resources::strategy::ALLIANCE_GAME_OPTIONS_NORMAL,
        pressed_resource: resources::strategy::ALLIANCE_GAME_OPTIONS_PRESSED,
    },
    StrategicControlSpec {
        button: CockpitButton::Encyclopedia,
        command_id: 0x132,
        rect: CockpitViewport {
            x: 446.0,
            y: 406.0,
            width: 27.0,
            height: 16.0,
        },
        normal_resource: resources::strategy::ALLIANCE_ENCYCLOPEDIA_NORMAL,
        pressed_resource: resources::strategy::ALLIANCE_ENCYCLOPEDIA_PRESSED,
    },
];

const EMPIRE_PRIMARY_CONTROLS: [StrategicControlSpec; 6] = [
    StrategicControlSpec {
        button: CockpitButton::SystemFinder,
        command_id: 0x12d,
        rect: CockpitViewport {
            x: 143.0,
            y: 434.0,
            width: 37.0,
            height: 24.0,
        },
        normal_resource: resources::strategy::EMPIRE_SYSTEM_FINDER_NORMAL,
        pressed_resource: resources::strategy::EMPIRE_SYSTEM_FINDER_PRESSED,
    },
    StrategicControlSpec {
        button: CockpitButton::FleetFinder,
        command_id: 0x12e,
        rect: CockpitViewport {
            x: 199.0,
            y: 434.0,
            width: 37.0,
            height: 24.0,
        },
        normal_resource: resources::strategy::EMPIRE_FLEET_FINDER_NORMAL,
        pressed_resource: resources::strategy::EMPIRE_FLEET_FINDER_PRESSED,
    },
    StrategicControlSpec {
        button: CockpitButton::PersonnelFinder,
        command_id: 0x12f,
        rect: CockpitViewport {
            x: 412.0,
            y: 433.0,
            width: 34.0,
            height: 22.0,
        },
        normal_resource: resources::strategy::EMPIRE_PERSONNEL_FINDER_NORMAL,
        pressed_resource: resources::strategy::EMPIRE_PERSONNEL_FINDER_PRESSED,
    },
    StrategicControlSpec {
        button: CockpitButton::TroopFinder,
        command_id: 0x130,
        rect: CockpitViewport {
            x: 253.0,
            y: 433.0,
            width: 34.0,
            height: 22.0,
        },
        normal_resource: resources::strategy::EMPIRE_TROOP_FINDER_NORMAL,
        pressed_resource: resources::strategy::EMPIRE_TROOP_FINDER_PRESSED,
    },
    StrategicControlSpec {
        button: CockpitButton::GameOptions,
        command_id: 0x131,
        rect: CockpitViewport {
            x: 465.0,
            y: 434.0,
            width: 35.0,
            height: 24.0,
        },
        normal_resource: resources::strategy::EMPIRE_GAME_OPTIONS_NORMAL,
        pressed_resource: resources::strategy::EMPIRE_GAME_OPTIONS_PRESSED,
    },
    StrategicControlSpec {
        button: CockpitButton::Encyclopedia,
        command_id: 0x132,
        rect: CockpitViewport {
            x: 519.0,
            y: 434.0,
            width: 37.0,
            height: 25.0,
        },
        normal_resource: resources::strategy::EMPIRE_ENCYCLOPEDIA_NORMAL,
        pressed_resource: resources::strategy::EMPIRE_ENCYCLOPEDIA_PRESSED,
    },
];

const ALLIANCE_GID_CONTROL: StrategicControlSpec = StrategicControlSpec {
    button: CockpitButton::GalacticInformationDisplay,
    command_id: 0x133,
    rect: CockpitViewport {
        x: 3.0,
        y: 355.0,
        width: 27.0,
        height: 41.0,
    },
    normal_resource: resources::strategy::ALLIANCE_GID_NORMAL,
    pressed_resource: resources::strategy::ALLIANCE_GID_PRESSED,
};

const EMPIRE_GID_CONTROL: StrategicControlSpec = StrategicControlSpec {
    button: CockpitButton::GalacticInformationDisplay,
    command_id: 0x133,
    rect: CockpitViewport {
        x: 79.0,
        y: 192.0,
        width: 35.0,
        height: 57.0,
    },
    normal_resource: resources::strategy::EMPIRE_GID_NORMAL,
    pressed_resource: resources::strategy::EMPIRE_GID_PRESSED,
};

const ALLIANCE_MESSAGE_INDEX_CONTROLS: [MessageIndexControlSpec; 9] = [
    message_index_control(0x136, 3.0, 109.0, 10050, 10060),
    message_index_control(0x137, 3.0, 134.0, 10051, 10061),
    message_index_control(0x138, 3.0, 159.0, 10052, 10062),
    message_index_control(0x139, 3.0, 184.0, 10053, 10063),
    message_index_control(0x13a, 3.0, 209.0, 10054, 10064),
    message_index_control(0x13b, 3.0, 234.0, 10055, 10065),
    message_index_control(0x13c, 3.0, 259.0, 10056, 10066),
    message_index_control(0x13d, 3.0, 284.0, 10057, 10067),
    message_index_control(0x13e, 3.0, 309.0, 10058, 10068),
];

const EMPIRE_MESSAGE_INDEX_CONTROLS: [MessageIndexControlSpec; 9] = [
    message_index_control(0x136, 611.0, 110.0, 10030, 10040),
    message_index_control(0x137, 611.0, 135.0, 10031, 10041),
    message_index_control(0x138, 611.0, 160.0, 10032, 10042),
    message_index_control(0x139, 611.0, 185.0, 10033, 10043),
    message_index_control(0x13a, 611.0, 210.0, 10034, 10044),
    message_index_control(0x13b, 611.0, 235.0, 10035, 10045),
    message_index_control(0x13c, 611.0, 260.0, 10036, 10046),
    message_index_control(0x13d, 611.0, 285.0, 10037, 10047),
    message_index_control(0x13e, 611.0, 310.0, 10038, 10048),
];

const fn message_index_control(
    command_id: u16,
    x: f32,
    y: f32,
    resting_resource: u32,
    illuminated_resource: u32,
) -> MessageIndexControlSpec {
    MessageIndexControlSpec {
        command_id,
        rect: CockpitViewport {
            x,
            y,
            width: 27.0,
            height: 22.0,
        },
        resting_resource,
        illuminated_resource,
    }
}

/// Exact primary-control table created by `FUN_00427270` for a faction.
#[must_use]
pub fn strategic_primary_controls(faction: CockpitFaction) -> &'static [StrategicControlSpec; 6] {
    match faction {
        CockpitFaction::Alliance => &ALLIANCE_PRIMARY_CONTROLS,
        CockpitFaction::Empire => &EMPIRE_PRIMARY_CONTROLS,
    }
}

/// Exact GID control created by `FUN_00427270` for a faction.
#[must_use]
pub fn strategic_gid_control(faction: CockpitFaction) -> &'static StrategicControlSpec {
    match faction {
        CockpitFaction::Alliance => &ALLIANCE_GID_CONTROL,
        CockpitFaction::Empire => &EMPIRE_GID_CONTROL,
    }
}

/// Exact Message Index rail records constructed by `FUN_00427270`.
#[must_use]
pub fn strategic_message_index_controls(
    faction: CockpitFaction,
) -> &'static [MessageIndexControlSpec; 9] {
    match faction {
        CockpitFaction::Alliance => &ALLIANCE_MESSAGE_INDEX_CONTROLS,
        CockpitFaction::Empire => &EMPIRE_MESSAGE_INDEX_CONTROLS,
    }
}

/// Uniformly scaled strategic canvas and its transparent galaxy aperture.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CockpitLayout {
    pub canvas: CockpitViewport,
    pub galaxy: CockpitViewport,
    pub scale: f32,
}

/// All mutable state owned by the cockpit module.
pub struct CockpitState {
    /// Which faction's chrome to render.
    pub faction: CockpitFaction,
    /// Height of the top decorative bar in pixels.
    pub top_bar_h: f32,
    /// Height of the bottom button bar in pixels.
    pub bottom_bar_h: f32,
    /// Side gutters width in pixels (equal left/right).
    pub side_gutter_w: f32,
    /// Active Galactic Information Display overlay.
    pub gid_mode: GidMode,
    /// Original GID popup and detailed-legend state.
    pub gid_ui: GidUiState,
    /// Strategic control currently holding native-style pointer capture.
    pressed_control: Option<CockpitButton>,
}

impl Default for CockpitState {
    fn default() -> Self {
        CockpitState {
            faction: CockpitFaction::Alliance,
            top_bar_h: 32.0,
            bottom_bar_h: 40.0,
            side_gutter_w: 0.0, // no side gutters for now — full width
            gid_mode: GidMode::PopularSupport,
            gid_ui: GidUiState::default(),
            pressed_control: None,
        }
    }
}

impl CockpitState {
    #[must_use]
    pub fn new(faction: CockpitFaction) -> Self {
        CockpitState {
            faction,
            ..Default::default()
        }
    }

    /// Compute the recovered command-center layout for the current screen.
    #[must_use]
    pub fn layout(&self) -> CockpitLayout {
        self.layout_for(screen_width(), screen_height())
    }

    /// Compute the recovered command-center layout for an arbitrary screen.
    ///
    /// This pure variant keeps the 640×480 composition testable without a
    /// graphics context.
    #[must_use]
    pub fn layout_for(&self, screen_width: f32, screen_height: f32) -> CockpitLayout {
        let scale = (screen_width / STRATEGIC_LOGICAL_WIDTH)
            .min(screen_height / STRATEGIC_LOGICAL_HEIGHT)
            .max(0.0);
        let canvas = CockpitViewport {
            x: (screen_width - STRATEGIC_LOGICAL_WIDTH * scale) / 2.0,
            y: (screen_height - STRATEGIC_LOGICAL_HEIGHT * scale) / 2.0,
            width: STRATEGIC_LOGICAL_WIDTH * scale,
            height: STRATEGIC_LOGICAL_HEIGHT * scale,
        };

        // FUN_00421c70 constructs these exact client rectangles. The right and
        // bottom values are exclusive in the original Win32 RECT contract.
        let (x, y, width, height) = match self.faction {
            CockpitFaction::Alliance => (55.0, 40.0, 485.0, 350.0),
            CockpitFaction::Empire => (120.0, 40.0, 480.0, 355.0),
        };
        let galaxy = CockpitViewport {
            x: canvas.x + x * scale,
            y: canvas.y + y * scale,
            width: width * scale,
            height: height * scale,
        };

        CockpitLayout {
            canvas,
            galaxy,
            scale,
        }
    }

    /// Compute the galaxy map viewport for the current screen.
    #[must_use]
    pub fn galaxy_viewport(&self) -> CockpitViewport {
        self.layout().galaxy
    }
}

// ---------------------------------------------------------------------------
// Draw functions
// ---------------------------------------------------------------------------

/// Prepare the strategic canvas and return its exact layout.
///
/// Call before the macroquad galaxy layers. The authentic shell is painted in
/// egui later in the same frame, above the clipped map and below other windows.
#[must_use]
pub fn draw_cockpit_chrome(state: &CockpitState) -> CockpitLayout {
    clear_background(BLACK);
    state.layout()
}

/// Apply or clear macroquad's top-left-origin scissor rectangle.
///
/// All strategic map layers use this one clip, preventing synthetic map pixels
/// from leaking into advisor and command-control apertures in the shell.
#[expect(
    clippy::cast_possible_truncation,
    reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
)]
pub fn set_cockpit_viewport_clip(viewport: Option<CockpitViewport>) {
    let clip = viewport.map(|viewport| {
        (
            viewport.x.round() as i32,
            viewport.y.round() as i32,
            viewport.width.round() as i32,
            viewport.height.round() as i32,
        )
    });
    // SAFETY: macroquad exposes its immediate drawing state through this API.
    // The clip is reset before egui begins its pass in the same frame.
    unsafe {
        get_internal_gl().quad_gl.scissor(clip);
    }
}

/// Draw the faction's authentic STRATEGY.DLL cockpit frame as the first egui
/// layer of the frame. Panels rendered afterward remain readable above it.
pub fn draw_cockpit_background(ctx: &egui::Context, state: &CockpitState, cache: &mut BmpCache) {
    let background_id = if state.faction == CockpitFaction::Alliance {
        resources::strategy::ALLIANCE_COMMAND_CENTER_SHELL
    } else {
        resources::strategy::EMPIRE_COMMAND_CENTER_SHELL
    };
    let Some(texture) = cache.get(ctx, DllSource::Strategy, background_id) else {
        return;
    };

    // `SidePanel` paints on egui's canonical background layer. Painting the
    // cockpit into a separate `Order::Background` layer can still place that
    // layer above side panels, depending on egui's area ordering. Use the same
    // canonical layer instead: this shape is appended first, then panels append
    // their frames, text, and bitmaps over it later in the frame.
    let painter = ctx.layer_painter(egui::LayerId::background());
    let canvas = state.layout().canvas;
    let uv_max_y = cockpit_source_uv_max_y(texture.size());
    painter.image(
        texture.id(),
        egui::Rect::from_min_size(
            egui::pos2(canvas.x, canvas.y),
            egui::vec2(canvas.width, canvas.height),
        ),
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, uv_max_y)),
        egui::Color32::WHITE,
    );
}

#[expect(
    clippy::cast_precision_loss,
    reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
)]
fn cockpit_source_uv_max_y(texture_size: [usize; 2]) -> f32 {
    let visible_source_height = (texture_size[0] as f32 * STRATEGIC_LOGICAL_HEIGHT
        / STRATEGIC_LOGICAL_WIDTH)
        .min(texture_size[1] as f32);
    visible_source_height / texture_size[1] as f32
}

/// Paint the native primary and GID controls over their shell apertures.
///
/// `FUN_00602d30` paints the normal resource at rest and the pressed resource
/// only while a valid press is captured. The original control
/// has no separate hover or persistent-selected bitmap state.
pub fn draw_cockpit_egui_layer(
    ctx: &egui::Context,
    state: &mut CockpitState,
    cache: &mut BmpCache,
) -> Option<GidMode> {
    let layout = state.layout();
    let controls = strategic_primary_controls(state.faction);
    let primary_down = ctx.input(|input| input.pointer.button_down(egui::PointerButton::Primary));
    let painter = ctx.layer_painter(egui::LayerId::background());

    for control in controls {
        draw_control(ctx, cache, &painter, layout, state, control, primary_down);
    }
    draw_control(
        ctx,
        cache,
        &painter,
        layout,
        state,
        strategic_gid_control(state.faction),
        primary_down,
    );
    draw_message_index_rail(ctx, cache, &painter, layout, state.faction);

    if state.gid_mode != GidMode::DisplayOff {
        draw_compact_gid_legend(ctx, cache, &painter, layout, state.faction);
    }

    let selected = draw_gid_menu(ctx, state, cache, layout);
    if let Some(mode) = selected {
        state.gid_mode = mode;
        state.gid_ui.menu_open = false;
        state.gid_ui.category = None;
    }
    selected
}

fn draw_message_index_rail(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    painter: &egui::Painter,
    layout: CockpitLayout,
    faction: CockpitFaction,
) {
    for control in strategic_message_index_controls(faction) {
        let Some(texture_id) = cache
            .get(ctx, DllSource::Strategy, control.resting_resource)
            .map(egui_macroquad::egui::TextureHandle::id)
        else {
            continue;
        };
        painter.image(
            texture_id,
            logical_rect_to_screen(layout, control.rect),
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    }
}

#[expect(
    clippy::cast_precision_loss,
    reason = "Preserve existing conversion of bitmap sizes and bounded UI indices into pixel coordinates."
)]
fn draw_control(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    painter: &egui::Painter,
    layout: CockpitLayout,
    state: &CockpitState,
    control: &StrategicControlSpec,
    primary_down: bool,
) {
    let pressed = primary_down && state.pressed_control == Some(control.button);
    let resource_id = control_resource(control, pressed);
    let Some(original_size) =
        cache.original_resource_size(DllSource::Strategy, control.normal_resource)
    else {
        return;
    };
    let Some(texture_id) = cache
        .get(ctx, DllSource::Strategy, resource_id)
        .map(egui_macroquad::egui::TextureHandle::id)
    else {
        return;
    };
    let screen_rect = logical_rect_to_screen(layout, control.rect);
    let image_rect = egui::Rect::from_min_size(
        screen_rect.min,
        egui::vec2(
            original_size[0] as f32 * layout.scale,
            original_size[1] as f32 * layout.scale,
        ),
    );
    painter.with_clip_rect(screen_rect).image(
        texture_id,
        image_rect,
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
        egui::Color32::WHITE,
    );
}

#[expect(
    clippy::cast_precision_loss,
    reason = "Preserve existing conversion of bitmap sizes and bounded UI indices into pixel coordinates."
)]
fn draw_compact_gid_legend(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    painter: &egui::Painter,
    layout: CockpitLayout,
    faction: CockpitFaction,
) {
    let resource_id = resources::strategy::GID_COMPACT_LEGEND;
    let Some(original_size) = cache.original_resource_size(DllSource::Strategy, resource_id) else {
        return;
    };
    let Some(texture_id) = cache
        .get(ctx, DllSource::Strategy, resource_id)
        .map(egui_macroquad::egui::TextureHandle::id)
    else {
        return;
    };
    let logical_x = match faction {
        CockpitFaction::Alliance => 55.0,
        CockpitFaction::Empire => 113.0,
    };
    let image_rect = egui::Rect::from_min_size(
        egui::pos2(
            layout.canvas.x + logical_x * layout.scale,
            layout.canvas.y + 50.0 * layout.scale,
        ),
        egui::vec2(
            original_size[0] as f32 * layout.scale,
            original_size[1] as f32 * layout.scale,
        ),
    );
    let canvas_clip = egui::Rect::from_min_size(
        egui::pos2(layout.canvas.x, layout.canvas.y),
        egui::vec2(layout.canvas.width, layout.canvas.height),
    );
    painter.with_clip_rect(canvas_clip).image(
        texture_id,
        image_rect,
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
        egui::Color32::WHITE,
    );
}

#[expect(
    clippy::cast_possible_truncation,
    reason = "The category index is bounded by the six-entry GidCategory table."
)]
fn gid_category_resource(category: GidCategory, faction: CockpitFaction) -> u32 {
    let offset = GidCategory::ALL
        .iter()
        .position(|candidate| *candidate == category)
        .unwrap_or_default() as u32;
    match faction {
        CockpitFaction::Alliance => resources::strategy::GID_ALLIANCE_CATEGORY_FIRST + offset,
        CockpitFaction::Empire => resources::strategy::GID_EMPIRE_CATEGORY_FIRST + offset,
    }
}

fn gid_arrow_resource(faction: CockpitFaction) -> u32 {
    match faction {
        CockpitFaction::Alliance => resources::strategy::GID_ALLIANCE_ARROW,
        CockpitFaction::Empire => resources::strategy::GID_EMPIRE_ARROW,
    }
}

fn gid_check_resource(faction: CockpitFaction) -> u32 {
    match faction {
        CockpitFaction::Alliance => resources::strategy::GID_CHECK_ALLIANCE,
        CockpitFaction::Empire => resources::strategy::GID_CHECK_EMPIRE,
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
fn gid_submenu_items(category: GidCategory, faction: CockpitFaction) -> Vec<GidMenuItem> {
    use resources::strategy;
    match category {
        GidCategory::Loyalty => vec![
            GidMenuItem {
                mode: GidMode::PopularSupport,
                resource_id: gid_category_resource(GidCategory::Loyalty, faction),
            },
            GidMenuItem {
                mode: GidMode::Uprisings,
                resource_id: match faction {
                    CockpitFaction::Alliance => strategy::GID_UPRISING_ALLIANCE,
                    CockpitFaction::Empire => strategy::GID_UPRISING_EMPIRE,
                },
            },
        ],
        GidCategory::Fleets => vec![
            GidMenuItem {
                mode: GidMode::IdleFleets,
                resource_id: gid_category_resource(GidCategory::Fleets, faction),
            },
            GidMenuItem {
                mode: GidMode::FleetsEnRoute,
                resource_id: match faction {
                    CockpitFaction::Alliance => strategy::GID_FLEETS_EN_ROUTE_ALLIANCE,
                    CockpitFaction::Empire => strategy::GID_FLEETS_EN_ROUTE_EMPIRE,
                },
            },
        ],
        GidCategory::Personnel => vec![
            GidMenuItem {
                mode: GidMode::IdlePersonnel,
                resource_id: match faction {
                    CockpitFaction::Alliance => strategy::GID_IDLE_PERSONNEL_ALLIANCE,
                    CockpitFaction::Empire => strategy::GID_EMPIRE_CATEGORY_FIRST + 2,
                },
            },
            GidMenuItem {
                mode: GidMode::ActivePersonnel,
                resource_id: match faction {
                    CockpitFaction::Alliance => strategy::GID_ACTIVE_PERSONNEL_ALLIANCE,
                    CockpitFaction::Empire => strategy::GID_EMPIRE_CATEGORY_LAST + 3,
                },
            },
        ],
        GidCategory::Resources => vec![
            GidMenuItem {
                mode: GidMode::AvailableEnergy,
                resource_id: strategy::GID_AVAILABLE_ENERGY,
            },
            GidMenuItem {
                mode: GidMode::AvailableRawMaterial,
                resource_id: strategy::GID_AVAILABLE_RAW_MATERIAL,
            },
            GidMenuItem {
                mode: GidMode::Mines,
                resource_id: strategy::GID_MINES,
            },
            GidMenuItem {
                mode: GidMode::Refineries,
                resource_id: strategy::GID_REFINERIES,
            },
        ],
        GidCategory::Manufacturing => vec![
            GidMenuItem {
                mode: GidMode::Shipyards,
                resource_id: strategy::GID_SHIPYARDS,
            },
            GidMenuItem {
                mode: GidMode::IdleShipyards,
                resource_id: strategy::GID_SHIPYARDS,
            },
            GidMenuItem {
                mode: GidMode::TrainingFacilities,
                resource_id: strategy::GID_TRAINING_FACILITIES,
            },
            GidMenuItem {
                mode: GidMode::IdleTrainingFacilities,
                resource_id: strategy::GID_TRAINING_FACILITIES,
            },
            GidMenuItem {
                mode: GidMode::ConstructionYards,
                resource_id: strategy::GID_CONSTRUCTION_YARDS,
            },
            GidMenuItem {
                mode: GidMode::IdleConstructionYards,
                resource_id: strategy::GID_CONSTRUCTION_YARDS,
            },
        ],
        GidCategory::Defense => vec![
            GidMenuItem {
                mode: GidMode::PlanetaryDefenseBatteries,
                resource_id: strategy::GID_PLANETARY_BATTERIES,
            },
            GidMenuItem {
                mode: GidMode::PlanetaryShieldGenerators,
                resource_id: match faction {
                    CockpitFaction::Alliance => strategy::GID_PLANETARY_SHIELDS_ALLIANCE,
                    CockpitFaction::Empire => strategy::GID_EMPIRE_CATEGORY_LAST,
                },
            },
            GidMenuItem {
                mode: GidMode::FighterSquadrons,
                resource_id: match faction {
                    CockpitFaction::Alliance => strategy::GID_FIGHTER_SQUADRONS_ALLIANCE,
                    CockpitFaction::Empire => strategy::GID_EMPIRE_CATEGORY_LAST + 2,
                },
            },
            GidMenuItem {
                mode: GidMode::Troopers,
                resource_id: match faction {
                    CockpitFaction::Alliance => strategy::GID_ACTIVE_PERSONNEL_ALLIANCE,
                    CockpitFaction::Empire => strategy::GID_EMPIRE_CATEGORY_LAST + 3,
                },
            },
            GidMenuItem {
                mode: GidMode::DeathStarShields,
                resource_id: strategy::GID_DEATH_STAR_SHIELDS,
            },
        ],
    }
}

fn gid_popup_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(egui::Color32::from_rgba_premultiplied(45, 47, 48, 218))
        // Keep the stroke's layout inset, but paint its visible edge from the
        // eight original STRATEGY bitmap tiles below.
        .stroke(egui::Stroke::new(1.0_f32, egui::Color32::TRANSPARENT))
        .inner_margin(egui::Margin::same(5))
}

/// `FUN_004511e0` passes STRATEGY 10100..10107 to the native GID frame
/// constructor. Corners are 2x2; the four one-pixel strips repeat between
/// them. The video reference also shows the characteristic alternating edge.
fn paint_gid_frame_border(ui: &egui::Ui, cache: &mut BmpCache, rect: egui::Rect, scale: f32) {
    let corner = 2.0 * scale;
    let tile = |ui: &egui::Ui, cache: &mut BmpCache, id, target: egui::Rect| {
        if let Some(texture) = cache.get(ui.ctx(), DllSource::Strategy, id) {
            ui.painter().image(
                texture.id(),
                target,
                egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );
        }
    };
    for (id, pos) in [
        (10100, rect.min),
        (10101, egui::pos2(rect.max.x - corner, rect.min.y)),
        (10102, egui::pos2(rect.min.x, rect.max.y - corner)),
        (10103, rect.max - egui::vec2(corner, corner)),
    ] {
        tile(
            ui,
            cache,
            id,
            egui::Rect::from_min_size(pos, egui::vec2(corner, corner)),
        );
    }
    for (id, start, end, horizontal) in [
        (
            10104,
            egui::pos2(rect.min.x + corner, rect.min.y),
            egui::pos2(rect.max.x - corner, rect.min.y),
            true,
        ),
        (
            10107,
            egui::pos2(rect.min.x + corner, rect.max.y - scale),
            egui::pos2(rect.max.x - corner, rect.max.y - scale),
            true,
        ),
        (
            10105,
            egui::pos2(rect.min.x, rect.min.y + corner),
            egui::pos2(rect.min.x, rect.max.y - corner),
            false,
        ),
        (
            10106,
            egui::pos2(rect.max.x - scale, rect.min.y + corner),
            egui::pos2(rect.max.x - scale, rect.max.y - corner),
            false,
        ),
    ] {
        let Some(texture) = cache.get(ui.ctx(), DllSource::Strategy, id) else {
            continue;
        };
        let mut mesh = egui::Mesh::with_texture(texture.id());
        let mut position = if horizontal { start.x } else { start.y };
        let limit = if horizontal { end.x } else { end.y };
        while position < limit {
            let remaining = (limit - position).min(corner);
            let target = if horizontal {
                egui::Rect::from_min_size(
                    egui::pos2(position, start.y),
                    egui::vec2(remaining, scale),
                )
            } else {
                egui::Rect::from_min_size(
                    egui::pos2(start.x, position),
                    egui::vec2(scale, remaining),
                )
            };
            let uv = egui::Rect::from_min_max(
                egui::Pos2::ZERO,
                if horizontal {
                    egui::pos2(remaining / corner, 1.0)
                } else {
                    egui::pos2(1.0, remaining / corner)
                },
            );
            mesh.add_rect_with_uv(target, uv, egui::Color32::WHITE);
            position += corner;
        }
        ui.painter().add(egui::Shape::mesh(mesh));
    }
}

fn paint_gid_icon(ui: &egui::Ui, cache: &mut BmpCache, resource_id: u32, rect: egui::Rect) {
    if let Some(texture) = cache.get(ui.ctx(), DllSource::Strategy, resource_id) {
        ui.painter().image(
            texture.id(),
            rect,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    }
}

fn gid_menu_row(
    ui: &mut egui::Ui,
    cache: &mut BmpCache,
    label: &str,
    icon_resource: Option<u32>,
    arrow_resource: Option<u32>,
    checked_resource: Option<u32>,
    scale: f32,
) -> egui::Response {
    let height = 21.0 * scale;
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::click(),
    );
    if let Some(resource_id) = icon_resource {
        paint_gid_icon(
            ui,
            cache,
            resource_id,
            egui::Rect::from_min_size(
                egui::pos2(
                    rect.min.x + if arrow_resource.is_some() { 22.0 } else { 2.0 } * scale,
                    rect.min.y + scale,
                ),
                egui::vec2(20.0 * scale, 20.0 * scale),
            ),
        );
    }
    if let Some(resource_id) = checked_resource {
        paint_gid_icon(
            ui,
            cache,
            resource_id,
            egui::Rect::from_min_size(
                egui::pos2(rect.min.x + 22.0 * scale, rect.min.y + 4.0 * scale),
                egui::vec2(14.0 * scale, 14.0 * scale),
            ),
        );
    }
    ui.painter().text(
        egui::pos2(
            rect.min.x + if arrow_resource.is_some() { 45.0 } else { 39.0 } * scale,
            rect.center().y,
        ),
        egui::Align2::LEFT_CENTER,
        label,
        egui::FontId::proportional((11.0 * scale).max(8.0)),
        egui::Color32::WHITE,
    );
    if let Some(resource_id) = arrow_resource {
        paint_gid_icon(
            ui,
            cache,
            resource_id,
            egui::Rect::from_min_size(
                egui::pos2(rect.min.x + 2.0 * scale, rect.min.y + scale),
                egui::vec2(17.0 * scale, 20.0 * scale),
            ),
        );
    }
    response
}

#[expect(
    clippy::too_many_lines,
    clippy::cast_precision_loss,
    reason = "Preserve existing conversion of bitmap sizes and bounded UI indices into pixel coordinates. Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
fn draw_gid_menu(
    ctx: &egui::Context,
    state: &mut CockpitState,
    cache: &mut BmpCache,
    layout: CockpitLayout,
) -> Option<GidMode> {
    if !state.gid_ui.menu_open {
        return None;
    }

    let scale = layout.scale.max(0.5);
    let root_pos = egui::pos2(
        layout.canvas.x + 425.0 * layout.scale,
        layout.canvas.y + 230.0 * layout.scale,
    );
    let root = egui::Area::new(egui::Id::new("original_gid_root_menu"))
        .order(egui::Order::Foreground)
        .fade_in(false)
        .fixed_pos(root_pos)
        .show(ctx, |ui| {
            ui.set_width(158.0 * scale);
            let menu = gid_popup_frame().show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                for category in GidCategory::ALL {
                    let response = gid_menu_row(
                        ui,
                        cache,
                        category.label(),
                        Some(gid_category_resource(category, state.faction)),
                        Some(gid_arrow_resource(state.faction)),
                        None,
                        scale,
                    );
                    if response.hovered() || response.clicked() {
                        state.gid_ui.category = Some(category);
                    }
                }
                let checked = (state.gid_mode == GidMode::DisplayOff)
                    .then_some(gid_check_resource(state.faction));
                gid_menu_row(
                    ui,
                    cache,
                    GidMode::DisplayOff.label(),
                    None,
                    None,
                    checked,
                    scale,
                )
                .clicked()
                .then_some(GidMode::DisplayOff)
            });
            paint_gid_frame_border(ui, cache, menu.response.rect, scale);
            menu.inner
        });

    let mut selected = root.inner;
    let mut submenu_rect = egui::Rect::NOTHING;
    if let Some(category) = state.gid_ui.category {
        let items = gid_submenu_items(category, state.faction);
        let category_index = GidCategory::ALL
            .iter()
            .position(|candidate| *candidate == category)
            .unwrap_or_default() as f32;
        let submenu_y = (root_pos.y + category_index * 21.0 * scale).min(
            layout.canvas.y + (STRATEGIC_LOGICAL_HEIGHT - 10.0) * layout.scale
                - items.len() as f32 * 21.0 * scale,
        );
        let submenu_pos = egui::pos2(root_pos.x - 246.0 * scale, submenu_y);
        let submenu = egui::Area::new(egui::Id::new("original_gid_submenu"))
            .order(egui::Order::Foreground)
            .fade_in(false)
            .fixed_pos(submenu_pos)
            .show(ctx, |ui| {
                ui.set_width(238.0 * scale);
                let menu = gid_popup_frame().show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    let mut chosen = None;
                    for item in items {
                        let checked = (state.gid_mode == item.mode)
                            .then_some(gid_check_resource(state.faction));
                        if gid_menu_row(
                            ui,
                            cache,
                            item.mode.label(),
                            Some(item.resource_id),
                            None,
                            checked,
                            scale,
                        )
                        .clicked()
                        {
                            chosen = Some(item.mode);
                        }
                    }
                    chosen
                });
                paint_gid_frame_border(ui, cache, menu.response.rect, scale);
                menu.inner
            });
        submenu_rect = submenu.response.rect;
        selected = selected.or(submenu.inner);
    }

    if selected.is_none() {
        let (pressed, pointer, escape) = ctx.input(|input| {
            (
                input.pointer.button_pressed(egui::PointerButton::Primary),
                input.pointer.interact_pos(),
                input.key_pressed(egui::Key::Escape),
            )
        });
        let outside = pressed
            && pointer.is_some_and(|pointer| {
                !root.response.rect.contains(pointer) && !submenu_rect.contains(pointer)
            });
        if outside || escape {
            state.gid_ui.menu_open = false;
            state.gid_ui.category = None;
        }
    }
    selected
}

/// Resolve control input after floating panels have registered their areas.
///
/// This prevents a control hidden by a modeless window from receiving the
/// window's click. Paint remains on egui's canonical background layer.
pub fn handle_cockpit_egui_input(
    ctx: &egui::Context,
    state: &mut CockpitState,
    cache: &mut BmpCache,
) -> Option<CockpitButton> {
    let layout = state.layout();
    let controls = strategic_primary_controls(state.faction);
    let (pointer_pos, primary_pressed, primary_down, primary_released) = ctx.input(|input| {
        (
            input.pointer.interact_pos(),
            input.pointer.button_pressed(egui::PointerButton::Primary),
            input.pointer.button_down(egui::PointerButton::Primary),
            input.pointer.button_released(egui::PointerButton::Primary),
        )
    });
    let pointer_hit = if ctx.is_pointer_over_area() {
        None
    } else {
        pointer_pos.and_then(|pointer| {
            control_at_pointer(cache, controls, layout, pointer).or_else(|| {
                control_at_pointer(
                    cache,
                    std::slice::from_ref(strategic_gid_control(state.faction)),
                    layout,
                    pointer,
                )
            })
        })
    };

    let clicked = update_control_capture(
        &mut state.pressed_control,
        primary_pressed,
        primary_down,
        primary_released,
        pointer_hit,
    );

    clicked.or_else(|| keyboard_control(ctx))
}

fn update_control_capture(
    captured: &mut Option<CockpitButton>,
    primary_pressed: bool,
    primary_down: bool,
    primary_released: bool,
    pointer_hit: Option<CockpitButton>,
) -> Option<CockpitButton> {
    if primary_pressed {
        *captured = pointer_hit;
    } else if !primary_down && !primary_released {
        // Browser focus loss can omit the release event. Do not leave a
        // native pressed frame latched when capture has ended.
        *captured = None;
    }

    if primary_released {
        let pressed = captured.take();
        pressed.filter(|button| Some(*button) == pointer_hit)
    } else {
        None
    }
}

fn control_resource(control: &StrategicControlSpec, pressed: bool) -> u32 {
    if pressed {
        control.pressed_resource
    } else {
        control.normal_resource
    }
}

fn logical_rect_to_screen(layout: CockpitLayout, rect: CockpitViewport) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(
            layout.canvas.x + rect.x * layout.scale,
            layout.canvas.y + rect.y * layout.scale,
        ),
        egui::vec2(rect.width * layout.scale, rect.height * layout.scale),
    )
}

#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
)]
fn resource_pixel_at_pointer(
    screen_rect: egui::Rect,
    scale: f32,
    pointer: egui::Pos2,
) -> Option<(usize, usize)> {
    if scale <= 0.0
        || pointer.x <= screen_rect.min.x
        || pointer.y <= screen_rect.min.y
        || pointer.x >= screen_rect.max.x
        || pointer.y >= screen_rect.max.y
    {
        return None;
    }
    Some((
        ((pointer.x - screen_rect.min.x) / scale).floor() as usize,
        ((pointer.y - screen_rect.min.y) / scale).floor() as usize,
    ))
}

fn control_at_pointer(
    cache: &mut BmpCache,
    controls: &[StrategicControlSpec],
    layout: CockpitLayout,
    pointer: egui::Pos2,
) -> Option<CockpitButton> {
    controls.iter().find_map(|control| {
        let screen_rect = logical_rect_to_screen(layout, control.rect);
        let (x, y) = resource_pixel_at_pointer(screen_rect, layout.scale, pointer)?;
        cache
            .is_resource_hit(DllSource::Strategy, control.normal_resource, x, y)
            .then_some(control.button)
    })
}

fn keyboard_control(ctx: &egui::Context) -> Option<CockpitButton> {
    let egui_key = ctx.input(|input| {
        [
            (egui::Key::F2, CockpitButton::SystemFinder),
            (egui::Key::F3, CockpitButton::FleetFinder),
            (egui::Key::F4, CockpitButton::TroopFinder),
            (egui::Key::F5, CockpitButton::PersonnelFinder),
            (egui::Key::F7, CockpitButton::GameOptions),
        ]
        .into_iter()
        .find_map(|(key, button)| input.key_pressed(key).then_some(button))
    });
    egui_key.or_else(|| {
        [
            KeyCode::F2,
            KeyCode::F3,
            KeyCode::F4,
            KeyCode::F5,
            KeyCode::F7,
        ]
        .into_iter()
        .find(|key| is_key_pressed(*key))
        .and_then(macroquad_accelerator)
    })
}

fn macroquad_accelerator(key: KeyCode) -> Option<CockpitButton> {
    match key {
        KeyCode::F2 => Some(CockpitButton::SystemFinder),
        KeyCode::F3 => Some(CockpitButton::FleetFinder),
        KeyCode::F4 => Some(CockpitButton::TroopFinder),
        KeyCode::F5 => Some(CockpitButton::PersonnelFinder),
        KeyCode::F7 => Some(CockpitButton::GameOptions),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 0.001,
            "expected {expected}, got {actual}"
        );
    }

    fn assert_viewport(actual: CockpitViewport, x: f32, y: f32, width: f32, height: f32) {
        assert_close(actual.x, x);
        assert_close(actual.y, y);
        assert_close(actual.width, width);
        assert_close(actual.height, height);
    }

    #[test]
    fn alliance_uses_recovered_640_by_480_aperture() {
        let layout = CockpitState::new(CockpitFaction::Alliance).layout_for(640.0, 480.0);

        assert_close(layout.scale, 1.0);
        assert_viewport(layout.canvas, 0.0, 0.0, 640.0, 480.0);
        assert_viewport(layout.galaxy, 55.0, 40.0, 485.0, 350.0);
    }

    #[test]
    fn empire_uses_recovered_640_by_480_aperture() {
        let layout = CockpitState::new(CockpitFaction::Empire).layout_for(640.0, 480.0);

        assert_close(layout.scale, 1.0);
        assert_viewport(layout.canvas, 0.0, 0.0, 640.0, 480.0);
        assert_viewport(layout.galaxy, 120.0, 40.0, 480.0, 355.0);
    }

    #[test]
    fn widescreen_is_uniformly_scaled_and_pillarboxed() {
        let layout = CockpitState::new(CockpitFaction::Alliance).layout_for(1280.0, 800.0);
        let scale = 800.0 / 480.0;

        assert_close(layout.scale, scale);
        assert_viewport(
            layout.canvas,
            (1280.0 - 640.0 * scale) / 2.0,
            0.0,
            640.0 * scale,
            800.0,
        );
        assert_close(layout.galaxy.x, layout.canvas.x + 55.0 * scale);
        assert_close(layout.galaxy.y, 40.0 * scale);
        assert_close(layout.galaxy.width, 485.0 * scale);
        assert_close(layout.galaxy.height, 350.0 * scale);
    }

    #[test]
    fn tall_screen_is_uniformly_scaled_and_letterboxed() {
        let layout = CockpitState::new(CockpitFaction::Empire).layout_for(640.0, 600.0);

        assert_close(layout.scale, 1.0);
        assert_viewport(layout.canvas, 0.0, 60.0, 640.0, 480.0);
        assert_viewport(layout.galaxy, 120.0, 100.0, 480.0, 355.0);
    }

    #[test]
    fn extra_strategy_source_row_is_not_displayed() {
        assert_close(cockpit_source_uv_max_y([640, 481]), 480.0 / 481.0);
        assert_close(cockpit_source_uv_max_y([1280, 962]), 960.0 / 962.0);
        assert_close(cockpit_source_uv_max_y([640, 480]), 1.0);
    }

    #[test]
    fn recovered_aperture_uses_exclusive_right_and_bottom_edges() {
        let viewport = CockpitState::new(CockpitFaction::Empire)
            .layout_for(640.0, 480.0)
            .galaxy;

        assert!(viewport.contains(120.0, 40.0));
        assert!(viewport.contains(599.999, 394.999));
        assert!(!viewport.contains(600.0, 394.0));
        assert!(!viewport.contains(599.0, 395.0));
    }

    #[test]
    fn alliance_primary_controls_match_recovered_constructor_records() {
        let controls = strategic_primary_controls(CockpitFaction::Alliance);
        let records: Vec<_> = controls
            .iter()
            .map(|control| {
                (
                    control.button,
                    control.command_id,
                    control.rect,
                    control.normal_resource,
                    control.pressed_resource,
                )
            })
            .collect();

        assert_eq!(
            records,
            vec![
                (
                    CockpitButton::SystemFinder,
                    0x12d,
                    CockpitViewport {
                        x: 106.0,
                        y: 408.0,
                        width: 27.0,
                        height: 16.0,
                    },
                    10002,
                    10001,
                ),
                (
                    CockpitButton::FleetFinder,
                    0x12e,
                    CockpitViewport {
                        x: 157.0,
                        y: 407.0,
                        width: 27.0,
                        height: 15.0,
                    },
                    10004,
                    10003,
                ),
                (
                    CockpitButton::PersonnelFinder,
                    0x12f,
                    CockpitViewport {
                        x: 258.0,
                        y: 405.0,
                        width: 27.0,
                        height: 16.0,
                    },
                    10006,
                    10005,
                ),
                (
                    CockpitButton::TroopFinder,
                    0x130,
                    CockpitViewport {
                        x: 209.0,
                        y: 405.0,
                        width: 27.0,
                        height: 16.0,
                    },
                    10008,
                    10007,
                ),
                (
                    CockpitButton::GameOptions,
                    0x131,
                    CockpitViewport {
                        x: 394.0,
                        y: 405.0,
                        width: 27.0,
                        height: 16.0,
                    },
                    10010,
                    10009,
                ),
                (
                    CockpitButton::Encyclopedia,
                    0x132,
                    CockpitViewport {
                        x: 446.0,
                        y: 406.0,
                        width: 27.0,
                        height: 16.0,
                    },
                    10012,
                    10011,
                ),
            ]
        );
    }

    #[test]
    fn empire_primary_controls_match_recovered_constructor_records() {
        let controls = strategic_primary_controls(CockpitFaction::Empire);
        assert_eq!(
            controls
                .iter()
                .map(|control| (
                    control.command_id,
                    control.rect.x,
                    control.rect.y,
                    control.rect.width,
                    control.rect.height,
                    control.normal_resource,
                    control.pressed_resource,
                ))
                .collect::<Vec<_>>(),
            vec![
                (0x12d, 143.0, 434.0, 37.0, 24.0, 10016, 10015),
                (0x12e, 199.0, 434.0, 37.0, 24.0, 10018, 10017),
                (0x12f, 412.0, 433.0, 34.0, 22.0, 10020, 10019),
                (0x130, 253.0, 433.0, 34.0, 22.0, 10022, 10021),
                (0x131, 465.0, 434.0, 35.0, 24.0, 10024, 10023),
                (0x132, 519.0, 434.0, 37.0, 25.0, 10026, 10025),
            ]
        );
    }

    #[test]
    fn gid_controls_match_recovered_constructor_records() {
        let alliance = strategic_gid_control(CockpitFaction::Alliance);
        assert_eq!(alliance.button, CockpitButton::GalacticInformationDisplay);
        assert_eq!(alliance.command_id, 0x133);
        assert_eq!(
            alliance.rect,
            CockpitViewport {
                x: 3.0,
                y: 355.0,
                width: 27.0,
                height: 41.0,
            }
        );
        assert_eq!(alliance.normal_resource, 10013);
        assert_eq!(alliance.pressed_resource, 10014);

        let empire = strategic_gid_control(CockpitFaction::Empire);
        assert_eq!(empire.button, CockpitButton::GalacticInformationDisplay);
        assert_eq!(empire.command_id, 0x133);
        assert_eq!(
            empire.rect,
            CockpitViewport {
                x: 79.0,
                y: 192.0,
                width: 35.0,
                height: 57.0,
            }
        );
        assert_eq!(empire.normal_resource, 10027);
        assert_eq!(empire.pressed_resource, 10028);
    }

    #[test]
    fn message_index_rails_match_recovered_constructor_records() {
        for (faction, x, first_y, resting_first, illuminated_first) in [
            (CockpitFaction::Alliance, 3.0, 109.0, 10050, 10060),
            (CockpitFaction::Empire, 611.0, 110.0, 10030, 10040),
        ] {
            let controls = strategic_message_index_controls(faction);
            for (index, control) in controls.iter().enumerate() {
                let index = u16::try_from(index).expect("nine message controls fit in u16");
                assert_eq!(control.command_id, 0x136 + index);
                assert_viewport(
                    control.rect,
                    x,
                    first_y + f32::from(index) * 25.0,
                    27.0,
                    22.0,
                );
                assert_eq!(control.resting_resource, resting_first + u32::from(index));
                assert_eq!(
                    control.illuminated_resource,
                    illuminated_first + u32::from(index)
                );
            }
        }
    }

    #[test]
    fn message_index_rail_follows_letterboxed_canvas() {
        let layout = CockpitState::new(CockpitFaction::Empire).layout_for(1280.0, 800.0);
        let first = strategic_message_index_controls(CockpitFaction::Empire)[0];
        let rect = logical_rect_to_screen(layout, first.rect);

        assert_close(rect.min.x, layout.canvas.x + 611.0 * layout.scale);
        assert_close(rect.min.y, layout.canvas.y + 110.0 * layout.scale);
        assert_close(rect.width(), 27.0 * layout.scale);
        assert_close(rect.height(), 22.0 * layout.scale);
    }

    #[test]
    fn popular_support_is_the_default_gid_mode() {
        assert_eq!(
            CockpitState::new(CockpitFaction::Alliance).gid_mode,
            GidMode::PopularSupport
        );
    }

    #[test]
    fn primary_control_rects_follow_uniform_canvas_scaling() {
        let layout = CockpitState::new(CockpitFaction::Empire).layout_for(1600.0, 960.0);
        let screen_rect = logical_rect_to_screen(
            layout,
            strategic_primary_controls(CockpitFaction::Empire)[0].rect,
        );

        assert_close(layout.scale, 2.0);
        assert_close(screen_rect.min.x, 160.0 + 143.0 * 2.0);
        assert_close(screen_rect.min.y, 434.0 * 2.0);
        assert_close(screen_rect.width(), 74.0);
        assert_close(screen_rect.height(), 48.0);
    }

    #[test]
    fn native_pointer_conversion_excludes_exact_outer_edges() {
        let rect = egui::Rect::from_min_max(egui::pos2(10.0, 20.0), egui::pos2(64.0, 52.0));

        assert_eq!(resource_pixel_at_pointer(rect, 2.0, rect.min), None);
        assert_eq!(
            resource_pixel_at_pointer(rect, 2.0, egui::pos2(10.1, 20.1)),
            Some((0, 0))
        );
        assert_eq!(
            resource_pixel_at_pointer(rect, 2.0, egui::pos2(12.0, 22.0)),
            Some((1, 1))
        );
        assert_eq!(
            resource_pixel_at_pointer(rect, 2.0, egui::pos2(63.999, 51.999)),
            Some((26, 15))
        );
        assert_eq!(resource_pixel_at_pointer(rect, 2.0, rect.max), None);
        assert_eq!(
            resource_pixel_at_pointer(rect, 2.0, egui::pos2(rect.max.x, 30.0)),
            None
        );
        assert_eq!(
            resource_pixel_at_pointer(rect, 2.0, egui::pos2(30.0, rect.max.y)),
            None
        );
    }

    #[test]
    fn native_capture_dispatches_only_after_release_over_the_same_control() {
        let mut captured = None;
        assert_eq!(
            update_control_capture(
                &mut captured,
                true,
                true,
                false,
                Some(CockpitButton::FleetFinder),
            ),
            None
        );
        assert_eq!(captured, Some(CockpitButton::FleetFinder));

        assert_eq!(
            update_control_capture(
                &mut captured,
                false,
                false,
                true,
                Some(CockpitButton::FleetFinder),
            ),
            Some(CockpitButton::FleetFinder)
        );
        assert_eq!(captured, None);
    }

    #[test]
    fn native_capture_cancels_on_outside_release_or_lost_capture() {
        let mut captured = Some(CockpitButton::PersonnelFinder);
        assert_eq!(
            update_control_capture(&mut captured, false, false, true, None),
            None
        );
        assert_eq!(captured, None);

        captured = Some(CockpitButton::TroopFinder);
        assert_eq!(
            update_control_capture(&mut captured, false, false, false, None),
            None
        );
        assert_eq!(captured, None);
    }

    #[test]
    fn recovered_gid_menu_covers_distinct_native_commands_for_both_factions() {
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let mut commands = std::collections::HashSet::new();
            for category in GidCategory::ALL {
                let items = gid_submenu_items(category, faction);
                assert!(!items.is_empty());
                for item in items {
                    assert!(commands.insert(item.mode.command_id()));
                    assert!(item.mode.is_active());
                    assert!(item.resource_id >= 10_000);
                }
            }
            assert_eq!(commands.len(), 21);
            assert!(!commands.contains(&GidMode::DisplayOff.command_id()));
            assert_eq!(GidMode::DisplayOff.command_id(), 0x80);
        }
    }

    #[test]
    fn control_art_and_macroquad_accelerators_match_native_states() {
        let control = &strategic_primary_controls(CockpitFaction::Alliance)[0];
        assert_eq!(control_resource(control, false), control.normal_resource);
        assert_eq!(control_resource(control, true), control.pressed_resource);
        assert_eq!(
            macroquad_accelerator(KeyCode::F2),
            Some(CockpitButton::SystemFinder)
        );
        assert_eq!(
            macroquad_accelerator(KeyCode::F3),
            Some(CockpitButton::FleetFinder)
        );
        assert_eq!(
            macroquad_accelerator(KeyCode::F4),
            Some(CockpitButton::TroopFinder)
        );
        assert_eq!(
            macroquad_accelerator(KeyCode::F5),
            Some(CockpitButton::PersonnelFinder)
        );
        assert_eq!(
            macroquad_accelerator(KeyCode::F7),
            Some(CockpitButton::GameOptions)
        );
        assert_eq!(macroquad_accelerator(KeyCode::F6), None);
    }
}
