//! Shared filing pipeline for strategic orders that produce one or more runs.

use rebellion_core::world::GameWorld;
use rebellion_data::integrator::StrikeStates;
use rebellion_render::message_log::{GameMessage, MessageLog};

/// Run a strategic order, then record and file each completed run through the
/// shared order pipeline.
pub(crate) fn issue_order<Run, Error>(
    world: &mut GameWorld,
    states: &mut StrikeStates<'_>,
    log: &mut MessageLog,
    issue: impl FnOnce(&mut GameWorld, &mut StrikeStates<'_>) -> Result<Vec<Run>, Error>,
    record: impl FnMut(u64, &GameWorld, &Run),
    messages: impl FnMut(&GameWorld, u64, &Run) -> Vec<GameMessage>,
) -> Result<Vec<Run>, Error> {
    let tick = states.tick;
    let runs = issue(world, states)?;
    Ok(file_order_runs(world, tick, runs, log, record, messages))
}

/// Declare a strategic order's public issue function from the pieces that
/// differ while keeping its signature and filing recipe in one place.
macro_rules! define_order_issue {
    (
        $(#[$meta:meta])*
        argument: $argument:ident: $argument_type:ty;
        run: $run:ty;
        error: $error:ty;
        issue: $issue:path;
        record: $record:path;
        messages: $messages:path;
    ) => {
        $(#[$meta])*
        pub fn issue(
            world: &mut GameWorld,
            fleets: &[FleetKey],
            $argument: $argument_type,
            states: &mut StrikeStates<'_>,
            roll: &mut dyn FnMut() -> f64,
            templates: &TextTemplates,
            log: &mut MessageLog,
        ) -> Result<Vec<$run>, $error> {
            $crate::strategic_order_common::issue_order(
                world,
                states,
                log,
                |world, states| $issue(world, fleets, $argument, states, roll),
                $record,
                |world, tick, run| $messages(world, templates, tick, run),
            )
        }
    };
}

pub(crate) use define_order_issue;

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
