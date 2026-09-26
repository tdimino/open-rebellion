//! Original bitmap-driven unified Game Options surface.

use macroquad::prelude::*;

use crate::audio::AudioVolumeState;
use crate::bmp_cache::{resources, BmpCache, DllSource};
use crate::panels::SaveSlotInfo;

const LOGICAL_WIDTH: f32 = 640.0;
const LOGICAL_HEIGHT: f32 = 480.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameOptionsOrigin {
    ShuttleCockpit,
    CommandCenter,
    TacticalBattle,
}

impl GameOptionsOrigin {
    const fn has_live_campaign(self) -> bool {
        !matches!(self, Self::ShuttleCockpit)
    }

    const fn tactical_toggles_enabled(self) -> bool {
        !matches!(self, Self::TacticalBattle)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameOptionsAction {
    None,
    Return,
    Restart,
    Exit,
    Save { slot: usize, name: String },
    Load { slot: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameOptionsControl {
    Save(usize),
    Load(usize),
    Music,
    MusicVolume,
    SoundVolume,
    Tactical(usize),
    Restart,
    Return,
    Exit,
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
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }
}

const MUSIC_RECT: NativeRect = NativeRect::new(351.0, 76.0, 19.0, 35.0);
const MUSIC_SLIDER_RECT: NativeRect = NativeRect::new(391.0, 130.0, 212.0, 54.0);
const SOUND_SLIDER_RECT: NativeRect = NativeRect::new(391.0, 192.0, 212.0, 54.0);
const TACTICAL_TOGGLE_RECTS: [NativeRect; 5] = [
    NativeRect::new(359.0, 310.0, 35.0, 22.0),
    NativeRect::new(359.0, 337.0, 35.0, 22.0),
    NativeRect::new(359.0, 364.0, 35.0, 22.0),
    NativeRect::new(359.0, 391.0, 35.0, 22.0),
    NativeRect::new(359.0, 418.0, 35.0, 22.0),
];
const RESTART_RECT: NativeRect = NativeRect::new(77.0, 383.0, 42.0, 42.0);
const RETURN_RECT: NativeRect = NativeRect::new(164.0, 383.0, 42.0, 42.0);
const EXIT_RECT: NativeRect = NativeRect::new(251.0, 383.0, 42.0, 42.0);

#[derive(Debug, Clone, Copy)]
struct OptionsCanvas {
    scale: f32,
    offset_x: f32,
    offset_y: f32,
}

impl OptionsCanvas {
    fn new(width: f32, height: f32) -> Self {
        let scale = (width / LOGICAL_WIDTH).min(height / LOGICAL_HEIGHT);
        Self {
            scale,
            offset_x: (width - LOGICAL_WIDTH * scale) * 0.5,
            offset_y: (height - LOGICAL_HEIGHT * scale) * 0.5,
        }
    }

    fn point(self, x: f32, y: f32) -> (f32, f32) {
        (
            self.offset_x + x * self.scale,
            self.offset_y + y * self.scale,
        )
    }

    fn logical_pointer(self) -> (f32, f32) {
        let (x, y) = mouse_position();
        (
            (x - self.offset_x) / self.scale,
            (y - self.offset_y) / self.scale,
        )
    }
}

#[derive(Debug, Clone)]
pub struct GameOptionsState {
    origin: GameOptionsOrigin,
    pressed: Option<GameOptionsControl>,
    pub show_starfield: bool,
    pub show_planet: bool,
    pub show_pyrotechnics: bool,
    pub high_detail_models: bool,
    pub display_holocube: bool,
}

impl Default for GameOptionsState {
    fn default() -> Self {
        Self {
            origin: GameOptionsOrigin::ShuttleCockpit,
            pressed: None,
            show_starfield: true,
            show_planet: true,
            show_pyrotechnics: true,
            high_detail_models: true,
            display_holocube: true,
        }
    }
}

impl GameOptionsState {
    #[must_use]
    pub fn new(origin: GameOptionsOrigin) -> Self {
        Self {
            origin,
            ..Self::default()
        }
    }

    pub fn set_origin(&mut self, origin: GameOptionsOrigin) {
        self.origin = origin;
        self.pressed = None;
    }

    #[must_use]
    pub const fn origin(&self) -> GameOptionsOrigin {
        self.origin
    }

    #[must_use]
    pub const fn tactical_flags(&self) -> [bool; 5] {
        [
            self.show_starfield,
            self.show_planet,
            self.show_pyrotechnics,
            self.high_detail_models,
            self.display_holocube,
        ]
    }
}

fn save_rect(slot: usize) -> NativeRect {
    NativeRect::new(35.0, 81.0 + slot as f32 * 42.0, 42.0, 20.0)
}

fn load_rect(slot: usize) -> NativeRect {
    NativeRect::new(285.0, 81.0 + slot as f32 * 42.0, 41.0, 20.0)
}

fn control_at(origin: GameOptionsOrigin, x: f32, y: f32) -> Option<GameOptionsControl> {
    for slot in 0..6 {
        if origin.has_live_campaign() && save_rect(slot).contains(x, y) {
            return Some(GameOptionsControl::Save(slot));
        }
        if load_rect(slot).contains(x, y) {
            return Some(GameOptionsControl::Load(slot));
        }
    }
    if MUSIC_RECT.contains(x, y) {
        return Some(GameOptionsControl::Music);
    }
    if MUSIC_SLIDER_RECT.contains(x, y) {
        return Some(GameOptionsControl::MusicVolume);
    }
    if SOUND_SLIDER_RECT.contains(x, y) {
        return Some(GameOptionsControl::SoundVolume);
    }
    for (index, rect) in TACTICAL_TOGGLE_RECTS.into_iter().enumerate() {
        if origin.tactical_toggles_enabled() && rect.contains(x, y) {
            return Some(GameOptionsControl::Tactical(index));
        }
    }
    if origin.has_live_campaign() && RESTART_RECT.contains(x, y) {
        return Some(GameOptionsControl::Restart);
    }
    if origin.has_live_campaign() && RETURN_RECT.contains(x, y) {
        return Some(GameOptionsControl::Return);
    }
    EXIT_RECT.contains(x, y).then_some(GameOptionsControl::Exit)
}

fn draw_bitmap(cache: &mut BmpCache, canvas: OptionsCanvas, resource_id: u32, x: f32, y: f32) {
    let Some(texture) = cache.get_macroquad_original(DllSource::Common, resource_id) else {
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

fn draw_label(canvas: OptionsCanvas, text: &str, x: f32, y: f32, size: f32) {
    let (screen_x, screen_y) = canvas.point(x, y);
    draw_text(
        text,
        screen_x,
        screen_y,
        size * canvas.scale,
        Color::from_rgba(0, 255, 24, 255),
    );
}

fn draw_volume(cache: &mut BmpCache, canvas: OptionsCanvas, y: f32, value: f32, alternate: bool) {
    draw_bitmap(
        cache,
        canvas,
        if alternate {
            resources::common::OPTIONS_VOLUME_RAIL_ALT
        } else {
            resources::common::OPTIONS_VOLUME_RAIL
        },
        391.0,
        y,
    );
    let handle_x = 392.0 + value.clamp(0.0, 1.0) * 199.0;
    draw_bitmap(
        cache,
        canvas,
        resources::common::OPTIONS_VOLUME_HANDLE,
        handle_x,
        y + 4.0,
    );
}

fn set_slider(audio: &mut AudioVolumeState, control: GameOptionsControl, logical_x: f32) {
    let value = ((logical_x - 396.0) / 199.0).clamp(0.0, 1.0);
    match control {
        GameOptionsControl::MusicVolume => audio.music_volume = value,
        GameOptionsControl::SoundVolume => audio.sfx_volume = value,
        _ => {}
    }
    audio.dirty = true;
}

fn toggle_tactical(state: &mut GameOptionsState, index: usize) {
    match index {
        0 => state.show_starfield = !state.show_starfield,
        1 => state.show_planet = !state.show_planet,
        2 => state.show_pyrotechnics = !state.show_pyrotechnics,
        3 => state.high_detail_models = !state.high_detail_models,
        4 => state.display_holocube = !state.display_holocube,
        _ => {}
    }
}

fn activate(
    state: &mut GameOptionsState,
    control: GameOptionsControl,
    saves: &[SaveSlotInfo],
    audio: &mut AudioVolumeState,
) -> GameOptionsAction {
    match control {
        GameOptionsControl::Music => {
            audio.toggle_music();
            macroquad::logging::info!(
                "[game_options] control=music enabled={}",
                audio.music_enabled()
            );
        }
        GameOptionsControl::Tactical(index) => {
            toggle_tactical(state, index);
            macroquad::logging::info!(
                "[game_options] control=tactical index={} enabled={}",
                index,
                state.tactical_flags()[index]
            );
        }
        GameOptionsControl::Save(slot) => {
            let name = saves.iter().find(|save| save.slot == slot).map_or_else(
                || format!("Saved Game {}", slot + 1),
                |save| save.name.clone(),
            );
            return GameOptionsAction::Save { slot, name };
        }
        GameOptionsControl::Load(slot) => {
            if saves.iter().any(|save| save.slot == slot) {
                return GameOptionsAction::Load { slot };
            }
        }
        GameOptionsControl::Restart => return GameOptionsAction::Restart,
        GameOptionsControl::Return => return GameOptionsAction::Return,
        GameOptionsControl::Exit => return GameOptionsAction::Exit,
        GameOptionsControl::MusicVolume | GameOptionsControl::SoundVolume => {}
    }
    GameOptionsAction::None
}

/// Draw the original 640×480 Game Options screen and dispatch its controls.
pub fn draw_game_options(
    state: &mut GameOptionsState,
    cache: &mut BmpCache,
    saves: &[SaveSlotInfo],
    audio: &mut AudioVolumeState,
) -> GameOptionsAction {
    let canvas = OptionsCanvas::new(screen_width(), screen_height());
    clear_background(BLACK);
    draw_bitmap(cache, canvas, resources::common::GAME_OPTIONS_BG, 0.0, 0.0);

    draw_label(canvas, "Saved Games", 132.0, 54.0, 17.0);
    draw_label(canvas, "Sound Options", 438.0, 54.0, 17.0);
    draw_label(canvas, "Play Music", 376.0, 100.0, 17.0);
    draw_label(
        canvas,
        if audio.music_enabled() { "On" } else { "Off" },
        580.0,
        100.0,
        17.0,
    );
    draw_label(canvas, "Tactical Display Options", 377.0, 289.0, 17.0);
    for (index, label) in [
        "Show Starfield",
        "Show Planet",
        "Show Pyrotechnics",
        "Use High Detail Models",
        "Display Holocube",
    ]
    .into_iter()
    .enumerate()
    {
        let y = 329.0 + index as f32 * 27.0;
        draw_label(canvas, label, 395.0, y, 16.0);
        draw_label(
            canvas,
            if state.tactical_flags()[index] {
                "On"
            } else {
                "Off"
            },
            580.0,
            y,
            16.0,
        );
    }
    draw_label(canvas, "Version: 1.0.0 Open Rebellion", 84.0, 454.0, 11.0);

    let pointer = canvas.logical_pointer();
    let held = state
        .pressed
        .filter(|_| is_mouse_button_down(MouseButton::Left));
    draw_bitmap(
        cache,
        canvas,
        if !audio.backend_available {
            resources::common::OPTIONS_MUSIC_DISABLED
        } else if held == Some(GameOptionsControl::Music) {
            resources::common::OPTIONS_MUSIC_PRESSED
        } else {
            resources::common::OPTIONS_MUSIC_NORMAL
        },
        MUSIC_RECT.x,
        MUSIC_RECT.y,
    );
    draw_volume(
        cache,
        canvas,
        MUSIC_SLIDER_RECT.y,
        audio.music_volume,
        false,
    );
    draw_volume(cache, canvas, SOUND_SLIDER_RECT.y, audio.sfx_volume, true);

    for (index, rect) in TACTICAL_TOGGLE_RECTS.into_iter().enumerate() {
        let resource = if !state.origin.tactical_toggles_enabled() {
            resources::common::OPTIONS_TOGGLE_OFF
        } else if held == Some(GameOptionsControl::Tactical(index)) {
            resources::common::OPTIONS_TOGGLE_PRESSED
        } else if state.tactical_flags()[index] {
            resources::common::OPTIONS_TOGGLE_ON
        } else {
            resources::common::OPTIONS_TOGGLE_OFF
        };
        draw_bitmap(cache, canvas, resource, rect.x, rect.y);
    }

    for slot in 0..6 {
        let occupied = saves.iter().find(|save| save.slot == slot);
        let save_resource = if !state.origin.has_live_campaign() {
            resources::common::OPTIONS_SAVE_DISABLED
        } else if held == Some(GameOptionsControl::Save(slot)) {
            resources::common::OPTIONS_SAVE_PRESSED
        } else {
            resources::common::OPTIONS_SAVE_NORMAL
        };
        let load_resource = if occupied.is_none() {
            resources::common::OPTIONS_LOAD_DISABLED
        } else if held == Some(GameOptionsControl::Load(slot)) {
            resources::common::OPTIONS_LOAD_PRESSED
        } else {
            resources::common::OPTIONS_LOAD_NORMAL
        };
        let save_bounds = save_rect(slot);
        let load_bounds = load_rect(slot);
        draw_bitmap(cache, canvas, save_resource, save_bounds.x, save_bounds.y);
        draw_bitmap(cache, canvas, load_resource, load_bounds.x, load_bounds.y);
        if let Some(save) = occupied {
            draw_label(canvas, &save.name, 116.0, 99.0 + slot as f32 * 42.0, 14.0);
        }
    }

    draw_bitmap(
        cache,
        canvas,
        if !state.origin.has_live_campaign() {
            resources::common::BTN_RESTART_GAME_DISABLED
        } else if held == Some(GameOptionsControl::Restart) {
            resources::common::BTN_RESTART_GAME_PRESSED
        } else {
            resources::common::BTN_RESTART_GAME_NORMAL
        },
        RESTART_RECT.x,
        RESTART_RECT.y,
    );
    draw_bitmap(
        cache,
        canvas,
        if !state.origin.has_live_campaign() {
            resources::common::BTN_RETURN_COMMAND_CENTER_DISABLED
        } else if held == Some(GameOptionsControl::Return) {
            resources::common::BTN_RETURN_COMMAND_CENTER_PRESSED
        } else {
            resources::common::BTN_RETURN_COMMAND_CENTER_NORMAL
        },
        RETURN_RECT.x,
        RETURN_RECT.y,
    );
    draw_bitmap(
        cache,
        canvas,
        if held == Some(GameOptionsControl::Exit) {
            resources::common::BTN_EXIT_GAME_PRESSED
        } else {
            resources::common::BTN_EXIT_GAME_NORMAL
        },
        EXIT_RECT.x,
        EXIT_RECT.y,
    );

    if let Some(control @ (GameOptionsControl::MusicVolume | GameOptionsControl::SoundVolume)) =
        state.pressed
    {
        if is_mouse_button_down(MouseButton::Left) {
            set_slider(audio, control, pointer.0);
        }
    }
    if is_mouse_button_pressed(MouseButton::Left) {
        state.pressed = control_at(state.origin, pointer.0, pointer.1);
        if let Some(control @ (GameOptionsControl::MusicVolume | GameOptionsControl::SoundVolume)) =
            state.pressed
        {
            set_slider(audio, control, pointer.0);
        }
    }
    if is_mouse_button_released(MouseButton::Left) {
        let pressed = state.pressed.take();
        if pressed == control_at(state.origin, pointer.0, pointer.1) {
            if let Some(control) = pressed {
                return activate(state, control, saves, audio);
            }
        }
    }
    GameOptionsAction::None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_hit_regions_reject_exact_outer_edges() {
        assert_eq!(
            control_at(GameOptionsOrigin::CommandCenter, 351.0, 76.0),
            Some(GameOptionsControl::Music)
        );
        assert_eq!(
            control_at(GameOptionsOrigin::CommandCenter, 370.0, 76.0),
            None
        );
        assert_eq!(
            control_at(GameOptionsOrigin::CommandCenter, 35.0, 81.0),
            Some(GameOptionsControl::Save(0))
        );
        assert_eq!(
            control_at(GameOptionsOrigin::CommandCenter, 285.0, 291.0),
            Some(GameOptionsControl::Load(5))
        );
    }

    #[test]
    fn shuttle_and_tactical_contexts_disable_original_controls() {
        assert_eq!(
            control_at(GameOptionsOrigin::ShuttleCockpit, 80.0, 400.0),
            None
        );
        assert_eq!(
            control_at(GameOptionsOrigin::TacticalBattle, 370.0, 320.0),
            None
        );
        assert_eq!(
            control_at(GameOptionsOrigin::TacticalBattle, 175.0, 400.0),
            Some(GameOptionsControl::Return)
        );
    }

    #[test]
    fn tactical_flags_default_to_original_on_state() {
        assert_eq!(GameOptionsState::default().tactical_flags(), [true; 5]);
    }
}
