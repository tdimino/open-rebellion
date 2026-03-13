//! Frame-independent game clock and tick system.
//!
//! The tick is the fundamental unit of simulation time in Open Rebellion.
//! One tick equals one game-day at Normal speed. All simulation systems
//! (manufacturing, missions, AI, events) advance in whole ticks — never
//! fractional ones — so game state is fully deterministic given the same
//! sequence of tick counts.
//!
//! # Usage
//!
//! ```
//! use rebellion_core::tick::{GameClock, GameSpeed};
//!
//! let mut clock = GameClock::new();
//! clock.set_speed(GameSpeed::Normal);
//!
//! // Each frame, pass the real elapsed seconds.
//! let events = clock.advance(1.0 / 60.0);
//! // events.len() == number of ticks that elapsed this frame.
//! ```

/// How fast simulated time flows relative to wall time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameSpeed {
    /// Simulation frozen. No ticks advance.
    Paused,
    /// 1 game-day per `TICK_DURATION` real seconds.
    Normal,
    /// 2× Normal.
    Fast,
    /// 4× Normal.
    Faster,
}

impl GameSpeed {
    /// Real-time multiplier applied to the accumulator each frame.
    ///
    /// Paused returns 0.0 — the accumulator never fills and no ticks fire.
    pub fn multiplier(self) -> f32 {
        match self {
            GameSpeed::Paused => 0.0,
            GameSpeed::Normal => 1.0,
            GameSpeed::Fast => 2.0,
            GameSpeed::Faster => 4.0,
        }
    }
}

/// Seconds of real time that must accumulate (at 1× speed) to complete one game-day.
///
/// Tuned so that at Normal speed the galaxy feels responsive without being
/// overwhelming. Adjust this constant to change pacing.
pub const TICK_DURATION: f32 = 1.0;

/// A single completed game-day tick.
///
/// Currently a marker — future phases will carry a payload (manufacturing
/// completion, mission resolution, event triggers) so downstream systems
/// can process each day's outcomes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TickEvent {
    /// The tick counter value at the moment this day completed.
    pub tick: u64,
}

/// Frame-independent game clock that drives all simulation timing.
///
/// Accumulates real-time elapsed seconds scaled by the current speed
/// multiplier. When the accumulator reaches `TICK_DURATION`, it emits
/// a `TickEvent` and increments the internal counter. Multiple ticks
/// can fire in a single `advance` call (e.g., when a very large `dt`
/// is passed, or when speed is Faster).
///
/// # Determinism
///
/// Given the same sequence of `(speed, dt)` pairs, `advance` always
/// produces the same sequence of tick counts. The accumulator is purely
/// additive — no floating-point branching outside of the subtraction loop.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameClock {
    /// Total game-days elapsed since the scenario started.
    pub tick: u64,
    /// Current simulation speed.
    pub speed: GameSpeed,
    /// Fractional real-time seconds that have not yet completed a full tick.
    accumulator: f32,
}

impl GameClock {
    /// Create a new clock, paused at day zero.
    pub fn new() -> Self {
        GameClock {
            tick: 0,
            speed: GameSpeed::Paused,
            accumulator: 0.0,
        }
    }

    /// Change the simulation speed. Takes effect on the next `advance` call.
    pub fn set_speed(&mut self, speed: GameSpeed) {
        self.speed = speed;
    }

    /// Advance the clock by `dt` real seconds.
    ///
    /// Returns a `Vec<TickEvent>` — one entry per game-day that completed
    /// during this frame. The vec is empty when paused or when `dt` is
    /// too small to complete a tick. Each event carries the tick number
    /// at the moment that day resolved.
    ///
    /// # Notes
    ///
    /// - `dt` should be the real elapsed frame time (e.g. `macroquad::time::get_frame_time()`).
    /// - Negative or zero `dt` is a no-op.
    pub fn advance(&mut self, dt: f32) -> Vec<TickEvent> {
        if dt <= 0.0 || self.speed == GameSpeed::Paused {
            return Vec::new();
        }

        self.accumulator += dt * self.speed.multiplier();

        // How many full ticks completed this frame?
        let ticks_elapsed = (self.accumulator / TICK_DURATION).floor() as u64;

        if ticks_elapsed == 0 {
            return Vec::new();
        }

        // Consume whole ticks from the accumulator.
        self.accumulator -= ticks_elapsed as f32 * TICK_DURATION;

        let events = (0..ticks_elapsed)
            .map(|i| {
                let tick_number = self.tick + i + 1;
                TickEvent { tick: tick_number }
            })
            .collect();

        self.tick += ticks_elapsed;

        events
    }
}

impl Default for GameClock {
    fn default() -> Self {
        Self::new()
    }
}

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paused_emits_no_ticks() {
        let mut clock = GameClock::new();
        // clock starts Paused
        let events = clock.advance(10.0);
        assert!(events.is_empty());
        assert_eq!(clock.tick, 0);
    }

    #[test]
    fn normal_speed_one_tick_per_duration() {
        let mut clock = GameClock::new();
        clock.set_speed(GameSpeed::Normal);

        // Just under a full tick — no event yet.
        let events = clock.advance(TICK_DURATION - 0.001);
        assert!(events.is_empty());
        assert_eq!(clock.tick, 0);

        // Push over the threshold.
        let events = clock.advance(0.002);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].tick, 1);
        assert_eq!(clock.tick, 1);
    }

    #[test]
    fn fast_speed_doubles_tick_rate() {
        let mut clock = GameClock::new();
        clock.set_speed(GameSpeed::Fast);

        // One real second at 2× = 2 game-days.
        let events = clock.advance(1.0);
        assert_eq!(events.len(), 2);
        assert_eq!(clock.tick, 2);
    }

    #[test]
    fn faster_speed_quadruples_tick_rate() {
        let mut clock = GameClock::new();
        clock.set_speed(GameSpeed::Faster);

        let events = clock.advance(1.0);
        assert_eq!(events.len(), 4);
        assert_eq!(clock.tick, 4);
    }

    #[test]
    fn tick_events_carry_correct_numbers() {
        let mut clock = GameClock::new();
        clock.set_speed(GameSpeed::Fast);

        // 3 real seconds at 2× = 6 ticks.
        let events = clock.advance(3.0);
        assert_eq!(events.len(), 6);
        for (i, event) in events.iter().enumerate() {
            assert_eq!(event.tick, (i + 1) as u64);
        }
    }

    #[test]
    fn accumulator_carries_remainder() {
        let mut clock = GameClock::new();
        clock.set_speed(GameSpeed::Normal);

        // 0.6s — no tick yet (TICK_DURATION = 1.0)
        let e1 = clock.advance(0.6);
        assert!(e1.is_empty());

        // 0.6s more — total 1.2s, one tick fires, 0.2s remains
        let e2 = clock.advance(0.6);
        assert_eq!(e2.len(), 1);
        assert_eq!(clock.tick, 1);

        // 0.6s more — total 0.8s accumulated, no tick yet
        let e3 = clock.advance(0.6);
        assert!(e3.is_empty());
        assert_eq!(clock.tick, 1);
    }

    #[test]
    fn speed_change_takes_effect_next_advance() {
        let mut clock = GameClock::new();
        clock.set_speed(GameSpeed::Normal);
        clock.advance(0.5); // accumulate 0.5s

        clock.set_speed(GameSpeed::Fast);
        // 0.5s at 2× = 1.0s added → total 1.5s → 1 tick fires
        let events = clock.advance(0.5);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn zero_or_negative_dt_is_noop() {
        let mut clock = GameClock::new();
        clock.set_speed(GameSpeed::Normal);

        assert!(clock.advance(0.0).is_empty());
        assert!(clock.advance(-1.0).is_empty());
        assert_eq!(clock.tick, 0);
    }

    #[test]
    fn determinism_same_sequence_same_result() {
        let run = |steps: &[(GameSpeed, f32)]| -> u64 {
            let mut clock = GameClock::new();
            for &(speed, dt) in steps {
                clock.set_speed(speed);
                clock.advance(dt);
            }
            clock.tick
        };

        let steps = [
            (GameSpeed::Normal, 0.3),
            (GameSpeed::Fast, 0.5),
            (GameSpeed::Faster, 1.0),
            (GameSpeed::Paused, 2.0),
            (GameSpeed::Normal, 0.8),
        ];

        assert_eq!(run(&steps), run(&steps));
    }
}
