//! Shared filing pipeline for strategic orders that produce one or more runs.

use rebellion_core::world::GameWorld;
use rebellion_render::message_log::{GameMessage, MessageLog};

/// Record and file every completed run while preserving the vector for movie
/// or caller-side follow-up work.
pub(crate) fn file_order_runs<Run>(
    world: &GameWorld,
    tick: u64,
    runs: Vec<Run>,
    log: &mut MessageLog,
    mut record: impl FnMut(u64, &GameWorld, &Run),
    mut messages: impl FnMut(&GameWorld, u64, &Run) -> Vec<GameMessage>,
) -> Vec<Run> {
    for run in &runs {
        record(tick, world, run);
        log.extend(messages(world, tick, run));
    }
    runs
}
