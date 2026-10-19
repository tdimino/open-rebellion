//! Standing policies for the developer commands' waits: what a wait does
//! when a modal screen stops the clock.
//!
//! A `Wait until` that the clock cannot satisfy refuses as soon as it is
//! clear the clock is stopped (`DevPolicy::blocker`), naming the cause. With
//! `Battle policy: simulate|retreat` or `Advice policy: close`, a wait
//! answers the modal itself instead, by posting a scripted click or key on
//! the control's own rect through `DevInput`, so the input path is the one a
//! hand would use. Every answered modal logs a `[dev-policy]` line, which
//! `scripts/dev-report.py` shows under the command that was waiting.

use crate::dev_commands::{control_rect, WaitCondition};
use crate::dev_input::DevInput;

/// Frames a stopped clock must stay stopped before a wait refuses, so a
/// modal that closes on its own does not fail the wait.
const BLOCKED_FRAMES: u32 = 30;
/// Frames the game may spend outside the galaxy, with nothing handling it,
/// before a wait refuses.
const OFF_GALAXY_FRAMES: u32 = 120;
/// Frames to let a posted click land (press and release, then the game's
/// reply) before the next one.
const CLICK_FRAMES: u32 = 8;
/// Frames the click that closes the Advice index is given.
const CLOSE_FRAMES: u32 = 30;
/// Clicks on one Battle Alert's button before the policy gives up: a
/// Retreat the alert shows disabled never closes it.
const BUTTON_ATTEMPTS: u8 = 3;
/// Frames a retreat's battle may run: Retreat opens the battle screen with
/// every ship that has a working hyperdrive flagged retreating, and the
/// Results come when they are gone.
const RETREAT_FRAMES: u32 = 3600;
/// Frames after the Advice index closes in which its pause alert is
/// answered.
const ADVICE_ALERT_FRAMES: u32 = 180;

/// What to do when a Battle Alert opens during a wait.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BattlePolicy {
    /// Leave it up: the wait refuses as blocked.
    #[default]
    Stop,
    /// Click Simulate, then close the Results.
    Simulate,
    /// Click Retreat.
    Retreat,
}

/// What to do when the Advice index opens during a wait.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AdvicePolicy {
    /// Leave it open: the wait refuses as blocked.
    #[default]
    Stop,
    /// Click `MessageIndex/Close`; the pause alert that follows is answered.
    Close,
}

/// What the game shows this frame, as the policies read it.
#[derive(Debug, Clone)]
pub struct Observed {
    pub day: u64,
    pub mode: String,
    pub galaxy: bool,
    pub main_menu: bool,
    /// The Battle Alert is up, and the system it names.
    pub battle_alert: Option<String>,
    /// The battle's Results window is up.
    pub results: bool,
    pub pause_alert: bool,
    /// The clock is on Paused: no speed, so no day passes.
    pub paused: bool,
    /// The advisor's opening briefing holds the clock.
    pub briefing: bool,
    /// The Message Index shows Advice.
    pub advice_index: bool,
}

/// The standing policies and the counters that act on them.
#[derive(Debug, Default)]
pub struct DevPolicy {
    pub battle: BattlePolicy,
    pub advice: AdvicePolicy,
    /// Dev-only: the clock advances exactly one day each frame.
    pub fast_forward: bool,
    blocked_frames: u32,
    off_galaxy_frames: u32,
    cooldown: u32,
    button_attempts: u8,
    gave_up: bool,
    /// Clicks on this Results window's or Advice index's close control.
    results_attempts: u8,
    advice_attempts: u8,
    advice_alert_frames: u32,
    /// Frames left for the battle a Retreat click opened.
    retreat_frames: u32,
    /// A control a policy needed that the control registry did not file this
    /// frame, and for how many frames running.
    missing: Option<(String, u32)>,
}

impl DevPolicy {
    /// The cause that keeps the clock from advancing for the wait's
    /// condition, once it has held long enough to be sure; `None` while the
    /// wait may still be met. Waits for a battle need no running clock's
    /// help here and never block.
    pub fn blocker(&mut self, observed: &Observed, condition: WaitCondition) -> Option<String> {
        if matches!(
            condition,
            WaitCondition::Battle | WaitCondition::BattleAlert
        ) {
            return None;
        }
        let o = observed;
        let answers_alert = self.battle != BattlePolicy::Stop && !self.gave_up;
        let cause = if let Some(system) = o.battle_alert.as_ref().filter(|_| !answers_alert) {
            Some(format!("battle alert at {system} day={}", o.day))
        } else if o.results && self.results_attempts >= BUTTON_ATTEMPTS {
            Some(format!(
                "Results/Close did not close the Results day={}",
                o.day
            ))
        } else if o.advice_index
            && self.advice == AdvicePolicy::Close
            && self.advice_attempts >= BUTTON_ATTEMPTS
        {
            Some(format!(
                "MessageIndex/Close did not close the Advice index day={}",
                o.day
            ))
        } else if o.pause_alert
            && condition != WaitCondition::PauseAlert
            // The advice policy answers the alert its close brings.
            && !(self.advice == AdvicePolicy::Close && self.advice_alert_frames > 0)
        {
            Some(format!("pause alert (Resume Game Play?) day={}", o.day))
        } else if o.briefing {
            Some(format!(
                "the advisor's briefing holds the clock day={}",
                o.day
            ))
        } else if o.main_menu {
            Some(format!("main menu day={}", o.day))
        } else if o.paused && condition != WaitCondition::PauseAlert {
            Some(format!("the clock is paused day={}", o.day))
        } else {
            None
        };
        if let Some((name, frames)) = self.missing.as_ref() {
            if *frames >= BLOCKED_FRAMES {
                return Some(format!("blocked: {name} not registered day={}", o.day));
            }
        }
        self.blocked_frames = if cause.is_some() {
            self.blocked_frames + 1
        } else {
            0
        };
        if self.blocked_frames >= BLOCKED_FRAMES {
            return cause.map(|cause| format!("blocked: {cause}"));
        }
        // A modal a policy is answering keeps the mode off the galaxy for a
        // while; anything else that does is stuck.
        let answering = (o.battle_alert.is_some() && answers_alert)
            || (o.results
                && self.battle != BattlePolicy::Stop
                && self.results_attempts < BUTTON_ATTEMPTS)
            || (self.retreat_frames > 0 && !o.galaxy);
        let off_galaxy = !o.galaxy && condition != WaitCondition::Galaxy && !answering;
        self.off_galaxy_frames = if off_galaxy {
            self.off_galaxy_frames + 1
        } else {
            0
        };
        (self.off_galaxy_frames >= OFF_GALAXY_FRAMES).then(|| {
            format!(
                "mode {} for {OFF_GALAXY_FRAMES} frames day={}",
                o.mode, o.day
            )
        })
    }

    /// Click the control the registry filed under `name` this frame; when it
    /// is not filed, note it for `blocker` and click nothing.
    fn click_control(&mut self, name: &str, input: &mut DevInput) -> bool {
        match control_rect(name) {
            Ok(rect) => {
                let (x, y) = rect.center();
                input.click(x, y, false, false);
                self.missing = None;
                true
            }
            Err(_) => {
                let frames = self
                    .missing
                    .take()
                    .filter(|(known, _)| known == name)
                    .map_or(0, |(_, frames)| frames);
                self.missing = Some((name.to_owned(), frames + 1));
                false
            }
        }
    }

    /// Answer the modal on screen, if a policy covers it. Posts through
    /// `input` and logs `[dev-policy] …`; call once a frame while a wait runs.
    pub fn answer_modals(&mut self, observed: &Observed, input: &mut DevInput) {
        self.advice_alert_frames = self.advice_alert_frames.saturating_sub(1);
        self.retreat_frames = self.retreat_frames.saturating_sub(1);
        if observed.galaxy {
            self.retreat_frames = 0;
        }
        if self.cooldown > 0 {
            self.cooldown -= 1;
            return;
        }
        if input.busy() {
            return;
        }
        let o = observed;
        if let Some(system) = o.battle_alert.as_ref() {
            let (control, name) = match self.battle {
                BattlePolicy::Stop => return,
                BattlePolicy::Simulate => ("BattleAlert/Simulate", "simulate"),
                BattlePolicy::Retreat => ("BattleAlert/Retreat", "retreat"),
            };
            if self.button_attempts >= BUTTON_ATTEMPTS {
                if !self.gave_up {
                    self.gave_up = true;
                    eprintln!(
                        "[dev-policy] battle at {system} day {} -> {name} did not close the alert",
                        o.day
                    );
                }
                return;
            }
            if !self.click_control(control, input) {
                return;
            }
            if self.button_attempts == 0 {
                eprintln!("[dev-policy] battle at {system} day {} -> {name}", o.day);
            }
            self.button_attempts += 1;
            if self.battle == BattlePolicy::Retreat {
                self.retreat_frames = RETREAT_FRAMES;
            }
            self.cooldown = CLICK_FRAMES;
            return;
        }
        self.button_attempts = 0;
        self.gave_up = false;
        if !o.results {
            self.results_attempts = 0;
        }
        if !o.advice_index {
            self.advice_attempts = 0;
        }
        if o.results && self.battle != BattlePolicy::Stop {
            if self.results_attempts >= BUTTON_ATTEMPTS {
                return;
            }
            if self.click_control("Results/Close", input) {
                self.results_attempts += 1;
                if self.battle == BattlePolicy::Retreat {
                    eprintln!("[dev-policy] retreat battle ended day {}", o.day);
                }
                eprintln!("[dev-policy] battle results closed day {}", o.day);
                self.retreat_frames = 0;
                self.cooldown = CLICK_FRAMES;
            }
            return;
        }
        if self.advice != AdvicePolicy::Close {
            self.missing = None;
            return;
        }
        if o.advice_index {
            if self.advice_attempts >= BUTTON_ATTEMPTS {
                return;
            }
            if self.click_control("MessageIndex/Close", input) {
                self.advice_attempts += 1;
                eprintln!("[dev-policy] advice closed day {}", o.day);
                self.advice_alert_frames = ADVICE_ALERT_FRAMES;
                self.cooldown = CLOSE_FRAMES;
            }
        } else if o.pause_alert && self.advice_alert_frames > 0 {
            if self.click_control("PauseAlert/Resume", input) {
                eprintln!("[dev-policy] advice pause alert answered day {}", o.day);
                self.advice_alert_frames = 0;
                self.cooldown = CLICK_FRAMES;
            }
        } else {
            self.missing = None;
        }
    }
}
