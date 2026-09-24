//! Shared original Game Options surface (PRE-03).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionsContext {
    MainMenu,
    Campaign,
    Tactical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionsAction {
    Save { slot: usize, name: String },
    Load { slot: usize },
    Delete { slot: usize },
    Return,
    Restart,
    Exit,
}

/// UI owns pending intent; no campaign or save mutation occurs before confirmation.
pub struct GameOptionsState {
    pub context: OptionsContext,
    pub suspended: bool,
    pub names: [String; 6],
    pub occupied: [bool; 6],
    pub pending: Option<OptionsAction>,
    pub error: Option<String>,
}

impl GameOptionsState {
    pub fn new(context: OptionsContext) -> Self {
        Self {
            context,
            suspended: false,
            names: Default::default(),
            occupied: [false; 6],
            pending: None,
            error: None,
        }
    }

    pub fn enabled(&self, action: &OptionsAction) -> bool {
        match action {
            OptionsAction::Save { slot, name } => {
                self.context == OptionsContext::Campaign && *slot < 6 && !name.trim().is_empty()
            }
            OptionsAction::Load { slot } | OptionsAction::Delete { slot } => {
                self.context != OptionsContext::Tactical
                    && self.occupied.get(*slot).copied().unwrap_or(false)
            }
            OptionsAction::Restart => self.context != OptionsContext::Tactical,
            OptionsAction::Return | OptionsAction::Exit => true,
        }
    }

    pub fn request(&mut self, action: OptionsAction) -> Option<OptionsAction> {
        if self.suspended || self.pending.is_some() || !self.enabled(&action) {
            return None;
        }
        let needs_confirmation = match &action {
            OptionsAction::Save { slot, .. } => self.occupied[*slot],
            OptionsAction::Load { .. } => self.context == OptionsContext::Campaign,
            OptionsAction::Restart => self.context == OptionsContext::Campaign,
            OptionsAction::Delete { .. } | OptionsAction::Exit => true,
            OptionsAction::Return => false,
        };
        self.error = None;
        if needs_confirmation {
            self.pending = Some(action);
            None
        } else {
            Some(action)
        }
    }

    pub fn confirm(&mut self, accepted: bool) -> Option<OptionsAction> {
        self.pending
            .take()
            .filter(|action| accepted && self.enabled(action))
    }

    pub fn escape(&mut self) -> Option<OptionsAction> {
        if self.pending.take().is_some() {
            None
        } else {
            Some(OptionsAction::Return)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pointer_input_respects_modal_and_tactical_display_guards() {
        let directory = std::env::temp_dir().join(format!("options-ui-{}", std::process::id()));
        let common = directory.join("common-dll/BMP");
        std::fs::create_dir_all(&common).unwrap();
        let fixture = image::RgbaImage::from_pixel(2, 2, image::Rgba([90, 90, 90, 255]));
        for id in [20002, 10040, 10041, 10042, 10043, 10044, 10045] {
            fixture.save(common.join(format!("{id}.bmp"))).unwrap();
        }
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        cache.set_base_path(&directory);
        let mut state = GameOptionsState::new(OptionsContext::Campaign);
        let mut audio = AudioVolumeState::default();
        let mut display = TacticalDisplayOptions::default();
        let mut click = |state: &mut GameOptionsState,
                         audio: &mut AudioVolumeState,
                         display: &mut TacticalDisplayOptions,
                         x: f32,
                         y: f32| {
            for down in [None, Some(true), Some(false)] {
                let position = Pos2::new(x, y);
                let mut events = vec![egui::Event::PointerMoved(position)];
                if let Some(pressed) = down {
                    events.push(egui::Event::PointerButton {
                        pos: position,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: Default::default(),
                    });
                }
                let _ = ctx.run(
                    egui::RawInput {
                        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(640.0, 480.0))),
                        events,
                        ..Default::default()
                    },
                    |ctx| {
                        assert_eq!(
                            draw_game_options(ctx, &mut cache, state, audio, display),
                            None
                        );
                    },
                );
            }
        };
        click(&mut state, &mut audio, &mut display, 365.0, 90.0);
        assert!(audio.music_muted);
        state.pending = Some(OptionsAction::Exit);
        click(&mut state, &mut audio, &mut display, 365.0, 90.0);
        assert!(
            audio.music_muted,
            "modal must own input even if its asset is absent"
        );
        state.pending = None;
        click(&mut state, &mut audio, &mut display, 370.0, 320.0);
        assert!(!display.starfield);
        state.context = OptionsContext::Tactical;
        click(&mut state, &mut audio, &mut display, 370.0, 320.0);
        assert!(
            !display.starfield,
            "native battle context locks display switches"
        );
        std::fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn overwrite_waits_for_confirmation_and_cancel_preserves_the_slot() {
        let mut state = GameOptionsState::new(OptionsContext::Campaign);
        state.occupied[2] = true;
        state.names[2] = "Old campaign".into();
        let action = OptionsAction::Save {
            slot: 2,
            name: "New campaign".into(),
        };
        assert_eq!(state.request(action.clone()), None);
        assert_eq!(state.pending, Some(action.clone()));
        assert_eq!(state.confirm(false), None);
        assert_eq!(state.names[2], "Old campaign");
        assert!(state.occupied[2]);
        assert_eq!(state.request(action.clone()), None);
        assert_eq!(state.confirm(true), Some(action));
        assert_eq!(
            state.confirm(true),
            None,
            "confirmation must be consumed once"
        );
    }

    #[test]
    fn empty_slot_save_is_immediate_but_invalid_requests_are_inert() {
        let mut state = GameOptionsState::new(OptionsContext::Campaign);
        let action = OptionsAction::Save {
            slot: 5,
            name: "Campaign".into(),
        };
        assert_eq!(state.request(action.clone()), Some(action));
        for action in [
            OptionsAction::Save {
                slot: 6,
                name: "Invalid".into(),
            },
            OptionsAction::Save {
                slot: 0,
                name: "  ".into(),
            },
            OptionsAction::Load { slot: 0 },
            OptionsAction::Delete { slot: 0 },
        ] {
            assert_eq!(state.request(action), None);
            assert_eq!(state.pending, None);
        }
    }

    #[test]
    fn tactical_context_cannot_write_or_replace_the_campaign() {
        let mut state = GameOptionsState::new(OptionsContext::Tactical);
        state.occupied[0] = true;
        for action in [
            OptionsAction::Save {
                slot: 0,
                name: "Battle".into(),
            },
            OptionsAction::Load { slot: 0 },
            OptionsAction::Delete { slot: 0 },
            OptionsAction::Restart,
        ] {
            assert_eq!(state.request(action), None);
            assert_eq!(state.pending, None);
        }
        assert_eq!(
            state.request(OptionsAction::Return),
            Some(OptionsAction::Return)
        );
    }

    #[test]
    fn escape_cancels_confirmation_before_returning_to_the_caller() {
        let mut state = GameOptionsState::new(OptionsContext::Campaign);
        assert_eq!(state.request(OptionsAction::Restart), None);
        assert_eq!(state.pending, Some(OptionsAction::Restart));
        assert_eq!(state.escape(), None);
        assert_eq!(state.pending, None);
        assert_eq!(state.escape(), Some(OptionsAction::Return));
    }

    #[test]
    fn save_management_respects_confirmation_and_modal_ownership() {
        let mut state = GameOptionsState::new(OptionsContext::Campaign);
        state.refresh_saves(&[SaveSlotInfo {
            slot: 5,
            name: "Legacy".into(),
            game_tick: 3,
            timestamp: "now".into(),
        }]);
        assert!(state.enabled(&OptionsAction::Load { slot: 5 }));
        state.suspended = true;
        assert_eq!(state.request(OptionsAction::Load { slot: 5 }), None);
        assert!(state.pending.is_none());
        state.suspended = false;
        assert_eq!(state.request(OptionsAction::Load { slot: 5 }), None);
        assert_eq!(state.pending, Some(OptionsAction::Load { slot: 5 }));
        assert_eq!(state.request(OptionsAction::Exit), None);
        assert_eq!(state.confirm(true), Some(OptionsAction::Load { slot: 5 }));
    }

    #[test]
    fn main_menu_loads_existing_saves_but_cannot_save_an_unstarted_campaign() {
        let mut state = GameOptionsState::new(OptionsContext::MainMenu);
        state.occupied[1] = true;
        assert_eq!(
            state.request(OptionsAction::Save {
                slot: 1,
                name: "No campaign".into()
            }),
            None
        );
        assert_eq!(
            state.request(OptionsAction::Load { slot: 1 }),
            Some(OptionsAction::Load { slot: 1 })
        );
    }
}

use crate::{AudioVolumeState, BmpCache, DllSource, SaveSlotInfo};
use egui_macroquad::egui::{self, Align2, Color32, FontId, Pos2, Rect, Sense, Vec2};

/// Presentation preferences; never part of the authoritative combat simulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TacticalDisplayOptions {
    pub starfield: bool,
    pub planet: bool,
    pub pyrotechnics: bool,
    pub high_detail: bool,
}

impl Default for TacticalDisplayOptions {
    fn default() -> Self {
        Self {
            starfield: true,
            planet: true,
            pyrotechnics: true,
            high_detail: true,
        }
    }
}

#[derive(Clone, Copy)]
struct Canvas {
    origin: Pos2,
    scale: f32,
}
impl Canvas {
    fn new(rect: Rect) -> Self {
        let scale = (rect.width() / 640.0).min(rect.height() / 480.0);
        Self {
            origin: rect.center() - Vec2::new(320.0, 240.0) * scale,
            scale,
        }
    }
    fn rect(self, x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect::from_min_size(
            self.origin + Vec2::new(x, y) * self.scale,
            Vec2::new(w, h) * self.scale,
        )
    }
}

impl GameOptionsState {
    pub fn refresh_saves(&mut self, saves: &[SaveSlotInfo]) {
        self.occupied = [false; 6];
        self.names = Default::default();
        for save in saves {
            if let Some(name) = self.names.get_mut(save.slot) {
                *name = save.name.clone();
            }
            if let Some(occupied) = self.occupied.get_mut(save.slot) {
                *occupied = true;
            }
        }
    }
}

fn bitmap(ui: &egui::Ui, cache: &mut BmpCache, source: DllSource, id: u32, rect: Rect) -> bool {
    let Some(texture) = cache.get(ui.ctx(), source, id) else {
        return false;
    };
    ui.painter().image(
        texture.id(),
        rect,
        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
        Color32::WHITE,
    );
    true
}

fn label(ui: &egui::Ui, canvas: Canvas, text: &str, x: f32, y: f32, enabled: bool, centered: bool) {
    ui.painter().text(
        canvas.origin + Vec2::new(x, y) * canvas.scale,
        if centered {
            Align2::CENTER_CENTER
        } else {
            Align2::LEFT_CENTER
        },
        text,
        FontId::monospace(if centered { 14.0 } else { 12.0 } * canvas.scale),
        if enabled {
            Color32::GREEN
        } else {
            Color32::from_rgb(0, 128, 0)
        },
    );
}

#[expect(
    clippy::too_many_arguments,
    reason = "A source bitmap control explicitly names its geometry and three resource states."
)]
fn button(
    ui: &egui::Ui,
    cache: &mut BmpCache,
    canvas: Canvas,
    source: DllSource,
    command: u32,
    geometry: [f32; 4],
    resources: [u32; 3],
    enabled: bool,
    selected: bool,
) -> bool {
    let [x, y, w, h] = geometry;
    let rect = canvas.rect(x, y, w, h);
    // A missing source image must never leave an invisible active control.
    let available = cache.get(ui.ctx(), source, resources[0]).is_some();
    let enabled = enabled && available;
    let response = ui.interact(
        rect,
        ui.id().with(command),
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    let id = if !enabled {
        resources[2]
    } else if selected || response.is_pointer_button_down_on() {
        resources[1]
    } else {
        resources[0]
    };
    bitmap(ui, cache, source, id, rect);
    enabled && response.clicked()
}

fn volume_slider(
    ui: &egui::Ui,
    cache: &mut BmpCache,
    canvas: Canvas,
    command: u32,
    y: f32,
    id: u32,
    volume: &mut f32,
    enabled: bool,
) -> bool {
    let track = canvas.rect(392.0, y, 201.0, 54.0);
    let available = bitmap(ui, cache, DllSource::Common, id, track)
        && cache.get(ui.ctx(), DllSource::Common, 10054).is_some();
    let response = ui.interact(
        track,
        ui.id().with(command),
        if enabled && available {
            Sense::click_and_drag()
        } else {
            Sense::hover()
        },
    );
    let before = *volume;
    if (response.dragged() || response.clicked()) && enabled && available {
        if let Some(pointer) = response.interact_pointer_pos() {
            *volume = ((pointer.x - track.left() - 5.5 * canvas.scale) / (190.0 * canvas.scale))
                .clamp(0.0, 1.0);
        }
    }
    bitmap(
        ui,
        cache,
        DllSource::Common,
        10054,
        canvas.rect(392.0 + 190.0 * volume.clamp(0.0, 1.0), y + 3.0, 11.0, 47.0),
    );
    *volume != before
}

/// Render the shared source bitmap window. The app retains its caller mode and
/// owns save operations and navigation; this function only emits validated intent.
#[expect(
    clippy::too_many_lines,
    reason = "Source-ordered composition of the unified options surface."
)]
pub fn draw_game_options(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    state: &mut GameOptionsState,
    audio: &mut AudioVolumeState,
    display: &mut TacticalDisplayOptions,
) -> Option<OptionsAction> {
    let mut requested = None;
    let mut answer = None;
    egui::CentralPanel::default()
        .frame(egui::Frame::NONE.fill(Color32::BLACK))
        .show(ctx, |ui| {
            let canvas = Canvas::new(ui.max_rect());
            if !bitmap(
                ui,
                cache,
                DllSource::Common,
                20002,
                canvas.rect(0.0, 0.0, 640.0, 480.0),
            ) {
                return;
            }
            let active = state.pending.is_none() && !state.suspended;
            label(ui, canvas, "Saved Games", 180.0, 47.0, true, true);
            label(ui, canvas, "Sound Options", 490.0, 47.0, true, true);
            label(
                ui,
                canvas,
                "Tactical Display Options",
                490.0,
                283.0,
                true,
                true,
            );
            for slot in 0..6 {
                let y = 81.0 + slot as f32 * 42.0;
                let save = OptionsAction::Save {
                    slot,
                    name: state.names[slot].clone(),
                };
                let load = OptionsAction::Load { slot };
                if button(
                    ui,
                    cache,
                    canvas,
                    DllSource::Common,
                    0x6f + slot as u32,
                    [34.0, y, 42.0, 20.0],
                    [10046, 10047, 10048],
                    active && state.enabled(&save),
                    false,
                ) {
                    requested = Some(save);
                }
                if button(
                    ui,
                    cache,
                    canvas,
                    DllSource::Common,
                    0x79 + slot as u32,
                    [287.0, y, 41.0, 20.0],
                    [10049, 10050, 10051],
                    active && state.enabled(&load),
                    false,
                ) {
                    requested = Some(load);
                }
                let rect = canvas.rect(119.0, y, 160.0, 20.0);
                if active && state.context == OptionsContext::Campaign {
                    ui.put(
                        rect,
                        egui::TextEdit::singleline(&mut state.names[slot])
                            .id_source(("original_save_name", slot))
                            .frame(false)
                            .margin(Vec2::ZERO)
                            .font(FontId::monospace(12.0 * canvas.scale))
                            .text_color(Color32::WHITE),
                    );
                } else {
                    ui.painter().with_clip_rect(rect).text(
                        rect.left_center(),
                        Align2::LEFT_CENTER,
                        &state.names[slot],
                        FontId::monospace(12.0 * canvas.scale),
                        Color32::WHITE,
                    );
                }
            }
            if button(
                ui,
                cache,
                canvas,
                DllSource::Common,
                0x99,
                [352.0, 76.0, 19.0, 35.0],
                [10040, 10041, 10042],
                active,
                !audio.music_muted,
            ) {
                audio.music_muted = !audio.music_muted;
                audio.dirty = true;
            }
            label(
                ui,
                canvas,
                "Play Music",
                376.0,
                95.0,
                !audio.music_muted,
                false,
            );
            label(
                ui,
                canvas,
                if audio.music_muted { "Off" } else { "On" },
                581.0,
                95.0,
                !audio.music_muted,
                false,
            );
            audio.dirty |= volume_slider(
                ui,
                cache,
                canvas,
                0x8d,
                130.0,
                10052,
                &mut audio.music_volume,
                active,
            );
            audio.dirty |= volume_slider(
                ui,
                cache,
                canvas,
                0x8e,
                190.0,
                10053,
                &mut audio.sfx_volume,
                active,
            );
            let display_enabled = active && state.context != OptionsContext::Tactical;
            for (command, y, text, value) in [
                (0xa1, 311.0, "Show Starfield", &mut display.starfield),
                (0xa2, 337.0, "Show Planet", &mut display.planet),
                (0xa3, 365.0, "Show Pyrotechnics", &mut display.pyrotechnics),
                (
                    0xa4,
                    392.0,
                    "Use High Detail Models",
                    &mut display.high_detail,
                ),
            ] {
                if button(
                    ui,
                    cache,
                    canvas,
                    DllSource::Common,
                    command,
                    [357.0, y, 35.0, 22.0],
                    [10043, 10044, 10045],
                    display_enabled,
                    *value,
                ) {
                    *value = !*value;
                }
                label(
                    ui,
                    canvas,
                    text,
                    394.0,
                    y + 10.0,
                    *value && display_enabled,
                    false,
                );
                label(
                    ui,
                    canvas,
                    if *value { "On" } else { "Off" },
                    577.0,
                    y + 10.0,
                    *value && display_enabled,
                    false,
                );
            }
            // Upstream has no Holocube renderer. Do not advertise a working setting.
            button(
                ui,
                cache,
                canvas,
                DllSource::Common,
                0xa6,
                [357.0, 419.0, 35.0, 22.0],
                [10043, 10044, 10045],
                false,
                false,
            );
            label(ui, canvas, "Display Holocube", 394.0, 429.0, false, false);
            label(ui, canvas, "Off", 577.0, 429.0, false, false);
            for (command, geometry, resources, action, allowed) in [
                (
                    0x91,
                    [76.0, 381.0, 42.0, 42.0],
                    [10035, 10036, 10037],
                    OptionsAction::Restart,
                    state.context != OptionsContext::Tactical,
                ),
                (
                    0x90,
                    [162.0, 382.0, 42.0, 42.0],
                    [10020, 10021, 10022],
                    OptionsAction::Return,
                    state.context != OptionsContext::MainMenu,
                ),
                (
                    0x8f,
                    [248.0, 381.0, 42.0, 42.0],
                    [10038, 10039, 10038],
                    OptionsAction::Exit,
                    true,
                ),
            ] {
                if button(
                    ui,
                    cache,
                    canvas,
                    DllSource::Common,
                    command,
                    geometry,
                    resources,
                    active && allowed,
                    false,
                ) {
                    requested = Some(action);
                }
            }
            if let Some(pending) = &state.pending {
                let dialog = Canvas {
                    origin: canvas.rect(114.0, 152.0, 412.0, 176.0).min,
                    scale: canvas.scale,
                };
                if bitmap(
                    ui,
                    cache,
                    DllSource::Rebdlog,
                    10623,
                    dialog.rect(0.0, 0.0, 412.0, 176.0),
                ) {
                    let (message, question) = match pending {
                        OptionsAction::Save { .. } => (
                            "Saving the selected game will destroy a previously saved game",
                            "Save anyway?",
                        ),
                        OptionsAction::Load { .. } => (
                            "Loading the selected game will destroy unsaved changes",
                            "Load without saving?",
                        ),
                        OptionsAction::Delete { .. } => (
                            "The selected saved game will be deleted",
                            "Delete this saved game?",
                        ),
                        OptionsAction::Restart => (
                            "Returning to the shuttle cockpit will cause unsaved changes to be lost",
                            "Return without saving?",
                        ),
                        OptionsAction::Exit => ("Unsaved changes will be lost", "Exit the game?"),
                        OptionsAction::Return => ("", "Return?"),
                    };
                    let text = ui.painter().layout(message.to_string(), FontId::monospace(12.0 * canvas.scale),
                    Color32::GREEN, 310.0 * canvas.scale);
                ui.painter().galley(dialog.rect(46.0, 25.0, 320.0, 65.0).center() - text.size() / 2.0,
                    text, Color32::GREEN);
                    label(ui, dialog, question, 206.0, 85.0, true, true);
                    for (command, x, ids, value) in [
                        (0x165, 138.0, [10624, 10625, 10624], true),
                        (0x166, 229.0, [10626, 10627, 10626], false),
                    ] {
                        if button(
                            ui,
                            cache,
                            dialog,
                            DllSource::Rebdlog,
                            command,
                            [x, 135.0, 57.0, 28.0],
                            ids,
                            true,
                            false,
                        ) {
                            answer = Some(value);
                        }
                        }
                }
            }
            if let Some(error) = &state.error {
                ui.painter()
                    .with_clip_rect(canvas.rect(25.0, 437.0, 590.0, 21.0))
                    .text(
                        canvas.rect(25.0, 437.0, 590.0, 21.0).left_center(),
                        Align2::LEFT_CENTER,
                        error,
                        FontId::monospace(11.0 * canvas.scale),
                        Color32::LIGHT_RED,
                    );
            }
        });
    if let Some(accepted) = answer {
        return state.confirm(accepted);
    }
    requested.and_then(|action| state.request(action))
}
