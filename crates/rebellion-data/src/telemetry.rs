//! Telemetry payloads shared by the headless integrator and the native game,
//! the conversion of core decision traces into records, and the native
//! game's JSONL sink.
//!
//! Both tick codes build each record's `details` with the functions here, so
//! a query over `telemetry.jsonl` reads a native run and a playtest run
//! alike; `source` and, natively, `seq` tell them apart.

use rebellion_core::ai::AIAction;
use rebellion_core::blockade::BlockadeEvent;
use rebellion_core::economy::EconomyEvent;
use rebellion_core::game_events::{
    GameEventRecord, EVT_AI_DAY, EVT_BLOCKADE_ENDED, EVT_BLOCKADE_STARTED,
    EVT_BLOCKADE_SUPPORT_SHIFT, EVT_BLOCKADE_TROOP_DESTROYED, EVT_COLLECTION_RATE,
    EVT_COMBAT_SPACE, EVT_CONTROL_CHANGED, EVT_ECONOMY_TICK, EVT_GARRISON_REQUIRED,
    EVT_MISSION_ENDED, EVT_MISSION_RESOLVED, EVT_RESEARCH_UNLOCKED, EVT_RESOURCE_DISCOVERY,
    EVT_SHIP_REPAIRED, EVT_SHIP_REPAIR_STARTED, EVT_SUPPORT_CHANGE, EVT_SUPPORT_SHIFT, EVT_TRACE,
    SYS_AI, SYS_BLOCKADE, SYS_COMBAT, SYS_ECONOMY, SYS_MISSIONS, SYS_REPAIR, SYS_RESEARCH,
};
use rebellion_core::ids::SystemKey;
use rebellion_core::missions::{MissionEnded, MissionKind, MissionResult};
use rebellion_core::repair::RepairEvent;
use rebellion_core::research::ResearchResult;
use rebellion_core::trace::TraceRecord;
use rebellion_core::world::{ControlKind, GameWorld};

use crate::integrator::{ai_action_json, sys_name, SystemSpaceCombatResult};

/// One side's strength in a space battle record.
#[must_use]
pub fn force_payload(force: &crate::integrator::SpaceForceSummary) -> serde_json::Value {
    serde_json::json!({
        "capital_ships": force.capital_ships,
        "fighter_squadrons": force.fighter_squadrons,
        "hull": force.hull,
    })
}

/// The `details` of a space battle's `combat_space` record.
#[must_use]
pub fn combat_space_payload(
    world: &GameWorld,
    result: &SystemSpaceCombatResult,
) -> serde_json::Value {
    use rebellion_core::combat::CombatSide;
    let winner_str = match result.winner {
        CombatSide::Attacker => "alliance",
        CombatSide::Defender => "empire",
        CombatSide::Draw => "draw",
    };
    serde_json::json!({
        "system": sys_name(world, result.system),
        "winner": winner_str,
        "seconds": result.seconds,
        "alliance_fleets": result.alliance_fleets,
        "empire_fleets": result.empire_fleets,
        "alliance_before": force_payload(&result.alliance_before),
        "alliance_after": force_payload(&result.alliance_after),
        "empire_before": force_payload(&result.empire_before),
        "empire_after": force_payload(&result.empire_after),
        "stalemate": result.stalemate,
    })
}

/// The `details` of a resolved mission's `mission_resolved` record.
#[must_use]
pub fn mission_resolved_payload(world: &GameWorld, result: &MissionResult) -> serde_json::Value {
    serde_json::json!({
        "mission_id": result.mission_id,
        "kind": format!("{:?}", result.kind),
        "outcome": format!("{:?}", result.outcome),
        "target_system": sys_name(world, result.target_system),
    })
}

/// The second `mission_resolved` record a covert mission emits, so the eval
/// harness sees all eight mission kinds.
#[must_use]
pub fn mission_espionage_payload(
    world: &GameWorld,
    result: &MissionResult,
) -> Option<serde_json::Value> {
    matches!(
        result.kind,
        MissionKind::Sabotage | MissionKind::Assassination | MissionKind::Abduction
    )
    .then(|| {
        serde_json::json!({
            "kind": "Espionage",
            "outcome": format!("{:?}", result.outcome),
            "target_system": sys_name(world, result.target_system),
            "parent_kind": format!("{:?}", result.kind),
        })
    })
}

/// What became of one AI order, as its `ai_action` record reads it.
#[derive(Debug, Clone, Default)]
pub struct AiOutcomeView {
    /// `"applied"`, `"refused"` or `"held"`.
    pub outcome: &'static str,
    /// The refusal's name (`family/Variant`), refused only.
    pub refusal: Option<String>,
    /// The validator's two-word status, mission refusals only.
    pub status: Option<[u32; 2]>,
    /// The running end a creation refusal pre-empts.
    pub end_code: Option<u8>,
    /// The system whose open battle held the order.
    pub held_at: Option<SystemKey>,
    /// The mission an applied dispatch created.
    pub mission_id: Option<u64>,
}

/// Who issued an AI order, and when.
#[derive(Debug, Clone, Copy)]
pub struct AiOrderOrigin<'a> {
    /// `"Alliance"` or `"Empire"`.
    pub faction: &'a str,
    /// The issuing module's kind, 1 to 14.
    pub module: u8,
    /// The in-day step of the round that issued it.
    pub round: u16,
    /// The day the round ran.
    pub issued: u64,
    /// The second AI of a dual-AI run.
    pub dual: bool,
}

/// The `details` of an AI order's `ai_action` record: the order, who issued
/// it, and its outcome.
#[must_use]
pub fn ai_order_payload(
    world: &GameWorld,
    action: &AIAction,
    origin: &AiOrderOrigin<'_>,
    outcome: &AiOutcomeView,
) -> serde_json::Value {
    let mut payload = ai_action_json(action, world);
    if let Some(obj) = payload.as_object_mut() {
        obj.insert("faction".into(), serde_json::json!(origin.faction));
        obj.insert("outcome".into(), serde_json::json!(outcome.outcome));
        obj.insert("module".into(), serde_json::json!(origin.module));
        obj.insert("round".into(), serde_json::json!(origin.round));
        obj.insert("issued".into(), serde_json::json!(origin.issued));
        if let Some(refusal) = &outcome.refusal {
            obj.insert("refusal".into(), serde_json::json!(refusal));
        }
        if let Some(status) = outcome.status {
            obj.insert("status".into(), serde_json::json!(status));
        }
        if let Some(code) = outcome.end_code {
            obj.insert("end_code".into(), serde_json::json!(code));
        }
        if let Some(system) = outcome.held_at {
            obj.insert("held_at".into(), serde_json::json!(sys_name(world, system)));
        }
        if let Some(id) = outcome.mission_id {
            obj.insert("mission_id".into(), serde_json::json!(id));
        }
        if origin.dual {
            obj.insert("dual_ai".into(), serde_json::json!(true));
        }
    }
    payload
}

/// One side's closed AI day.
#[derive(Debug, Clone, Copy)]
pub struct AiDay<'a> {
    /// `"Alliance"` or `"Empire"`.
    pub faction: &'a str,
    /// The day closed.
    pub day: u64,
    /// `"very_slow"`, `"slow"`, `"medium"` or `"fast"`.
    pub speed: &'a str,
    /// The day ran paused; port: paused counts as fast.
    pub paused: bool,
    /// The per-day count (`FUN_0051dea0`): the rounds a day less one, 4, 12,
    /// 60 or 600 by default.
    pub rate: u16,
    /// The rounds run that day.
    pub rounds: u16,
    /// The day's counted orders (`+0x420` before its reset).
    pub orders: u16,
    pub dual: bool,
}

/// The `details` of an `ai_day` record.
#[must_use]
pub fn ai_day_payload(day: &AiDay<'_>) -> serde_json::Value {
    let mut payload = serde_json::json!({
        "faction": day.faction,
        "day": day.day,
        "speed": day.speed,
        "rate": day.rate,
        "rounds": day.rounds,
        "orders": day.orders,
    });
    if let Some(obj) = payload.as_object_mut() {
        if day.paused {
            obj.insert("paused".into(), serde_json::json!(true));
        }
        if day.dual {
            obj.insert("dual_ai".into(), serde_json::json!(true));
        }
    }
    payload
}

/// The `details` of a `mission_ended` record.
#[must_use]
pub fn mission_ended_payload(ended: &MissionEnded) -> serde_json::Value {
    serde_json::json!({
        "mission_id": ended.mission_id,
        "kind": format!("{:?}", ended.kind),
        "faction": format!("{:?}", ended.faction),
        "end_code": ended.end_code,
        "ordered_tick": ended.ordered_tick,
        "days_run": ended.tick.saturating_sub(ended.ordered_tick),
        "rolled": ended.rolls > 0,
        "rolls": ended.rolls,
    })
}

/// The `details` of the blockade records: the system's name.
#[must_use]
pub fn blockade_payload(world: &GameWorld, system: SystemKey) -> serde_json::Value {
    serde_json::json!({ "system": sys_name(world, system) })
}

/// The `details` of a `blockade_ended` record: the system, the blockader,
/// each side's loss total and strength sum, and the record they make
/// (`support_shift::blockade_end_record`); `blockader` and `shift` are null
/// when no shift was evaluated.
#[must_use]
pub fn blockade_end_payload(
    world: &GameWorld,
    blockade: &rebellion_core::blockade::BlockadeState,
    system: SystemKey,
) -> serde_json::Value {
    let losses = blockade.losses(system);
    let total = |side| losses.map_or(0, |losses| losses.total(side));
    let [alliance_strength, empire_strength] = rebellion_core::support_shift::strength(world);
    let shift =
        rebellion_core::support_shift::blockade_end_record(world, blockade, system).map(|record| {
            serde_json::json!({
                "direction": format!("{:?}", record.direction),
                "amount": record.amount,
            })
        });
    serde_json::json!({
        "system": sys_name(world, system),
        "blockader": losses.and_then(|losses| losses.blockader).map(|side| format!("{side:?}")),
        "alliance_losses": total(rebellion_core::dat::Faction::Alliance),
        "empire_losses": total(rebellion_core::dat::Faction::Empire),
        "alliance_strength": alliance_strength,
        "empire_strength": empire_strength,
        "shift": shift,
    })
}

/// The `details` of a `blockade_support_shift` record, read after the
/// shift: the blockader, its points before the setter, and both sides'
/// support now.
#[must_use]
pub fn support_shift_payload(
    world: &GameWorld,
    system: SystemKey,
    side: rebellion_core::dat::Faction,
    delta: i32,
) -> serde_json::Value {
    use rebellion_core::dat::Faction;
    let support = |side| {
        world
            .systems
            .get(system)
            .map(|sys| rebellion_core::uprising::support_points(sys, side))
    };
    serde_json::json!({
        "system": sys_name(world, system),
        "side": format!("{side:?}"),
        "delta": delta,
        "alliance_support": support(Faction::Alliance),
        "empire_support": support(Faction::Empire),
    })
}

/// The record an economy event writes, its event type and `details`.
#[must_use]
pub fn economy_record(
    world: &GameWorld,
    event: &EconomyEvent,
) -> Option<(&'static str, serde_json::Value)> {
    let name = |system: &SystemKey| sys_name(world, *system);
    Some(match event {
        EconomyEvent::CollectionRateChanged { system, new_rate } => (
            EVT_COLLECTION_RATE,
            serde_json::json!({ "system": name(system), "rate": new_rate }),
        ),
        EconomyEvent::GarrisonRequirementChanged {
            system,
            new_requirement,
        } => (
            EVT_GARRISON_REQUIRED,
            serde_json::json!({ "system": name(system), "garrison_required": new_requirement }),
        ),
        EconomyEvent::IncidentTriggered {
            system,
            incident_type,
        } => (
            EVT_ECONOMY_TICK,
            serde_json::json!({ "system": name(system), "incident": incident_type }),
        ),
        EconomyEvent::EnergyOvercapped {
            system,
            allocated,
            capacity,
        } => (
            EVT_ECONOMY_TICK,
            serde_json::json!({
                "system": name(system),
                "energy_overcap": true,
                "allocated": allocated,
                "capacity": capacity,
            }),
        ),
        EconomyEvent::RawMaterialOvercapped {
            system,
            allocated,
            capacity,
        } => (
            EVT_ECONOMY_TICK,
            serde_json::json!({
                "system": name(system),
                "raw_material_overcap": true,
                "allocated": allocated,
                "capacity": capacity,
            }),
        ),
        // K1: EVT_SUPPORT_CHANGE (0x100), who holds the system by support
        // alone before and after (`FUN_0055a080`).
        EconomyEvent::SupportChanged { system, from, to } => (
            EVT_SUPPORT_CHANGE,
            serde_json::json!({
                "system": name(system),
                "from": format!("{from:?}"),
                "to": format!("{to:?}"),
            }),
        ),
        // K3: EVT_RESOURCE_DISCOVERY (0x155).
        EconomyEvent::ResourceDiscovered { system, new_output } => (
            EVT_RESOURCE_DISCOVERY,
            serde_json::json!({ "system": name(system), "new_output": new_output }),
        ),
    })
}

/// The `details` of a `research_unlocked` record.
#[must_use]
pub fn research_unlocked_payload(result: &ResearchResult) -> serde_json::Value {
    let ResearchResult::TechUnlocked {
        faction_is_alliance,
        tech_type,
        new_level,
    } = result;
    serde_json::json!({
        "faction_is_alliance": faction_is_alliance,
        "tech_type": format!("{:?}", tech_type),
        "new_level": new_level,
    })
}

/// The event type and `details` of a repair record, or `None` for a timer
/// change that repairs nothing. `fast` and `next_due` are the timer the step
/// rearmed: the shipyard interval or the slow one (`FUN_00557860`).
#[must_use]
pub fn repair_payload(
    world: &GameWorld,
    event: &RepairEvent,
) -> Option<(&'static str, serde_json::Value)> {
    match event {
        RepairEvent::ShipRepaired {
            fleet,
            hull_before,
            hull_after,
            counters_before,
            counters_after,
            timer,
            ..
        } => {
            let fleet_location = world
                .fleets
                .get(*fleet)
                .map_or_else(|| "unknown".into(), |f| sys_name(world, f.location));
            Some((
                EVT_SHIP_REPAIRED,
                serde_json::json!({
                    "fleet_location": fleet_location,
                    "fleet": world.fleet_name(*fleet),
                    "hull_before": hull_before,
                    "hull_after": hull_after,
                    "delta": hull_after - hull_before,
                    "counters_before": counters_before,
                    "counters_after": counters_after,
                    "fast": timer.fast,
                    "next_due": timer.due,
                }),
            ))
        }
        RepairEvent::TimerSet { .. } => None,
        RepairEvent::RepairCheckPerformed {
            system,
            fleet,
            ships_checked,
        } => {
            let fleet_commander = world
                .fleets
                .get(*fleet)
                .and_then(|f| {
                    world
                        .characters
                        .get(*f.characters.first()?)
                        .map(|c| c.name.as_str())
                })
                .unwrap_or("uncrewed");
            Some((
                EVT_SHIP_REPAIR_STARTED,
                serde_json::json!({
                    "system": sys_name(world, *system),
                    "fleet": world.fleet_name(*fleet),
                    "fleet_commander": fleet_commander,
                    "ships_checked": ships_checked,
                }),
            ))
        }
    }
}

/// The `details` of a `control_changed` record from the economy's troop and
/// support resolution.
#[must_use]
pub fn control_resolved_payload(
    world: &GameWorld,
    system: SystemKey,
    new_control: ControlKind,
) -> serde_json::Value {
    serde_json::json!({
        "system": sys_name(world, system),
        "new_control": format!("{:?}", new_control),
    })
}

/// The `details` of a `control_changed` record from a Planetary Assault,
/// whose landing or losses rebuilt the holder (`FUN_0050b610`).
#[must_use]
pub fn control_assault_payload(
    world: &GameWorld,
    system: SystemKey,
    previous: ControlKind,
) -> serde_json::Value {
    serde_json::json!({
        "system": sys_name(world, system),
        "from": format!("{previous:?}"),
        "to": format!(
            "{:?}",
            world.systems.get(system).map(|sys| sys.control).unwrap_or_default()
        ),
        "cause": "assault",
    })
}

/// The `details` of an `assault` record: the run's sides, counts and
/// losses (`rebellion_core::assault::AssaultReport`).
#[must_use]
pub fn assault_payload(
    world: &GameWorld,
    report: &rebellion_core::assault::AssaultReport,
) -> serde_json::Value {
    let side = |side: Option<rebellion_core::dat::Faction>| side.map(|side| format!("{side:?}"));
    serde_json::json!({
        "system": report.system.map(|system| sys_name(world, system)),
        "assaulter": side(report.assaulter),
        "holder_before": side(report.holder_before),
        "holder_after": side(report.holder_after),
        "attackers": report.attackers,
        "defenders": report.defenders,
        "shots_fired": report.shots.iter().filter(|shot| shot.fired).count(),
        "engagements": report.engagements.len(),
        "attackers_lost": report.attackers_lost.len(),
        "defenders_lost": report.defenders_lost.len(),
        "facilities_lost": report.facilities_lost.len(),
        "energy": report.energy,
        "raw_material": report.raw_material,
        "landed": report.landed.len(),
    })
}

/// The `details` of a `control_changed` record from a Death Star kill,
/// which turns the system neutral.
#[must_use]
pub fn control_destroyed_payload(
    world: &GameWorld,
    system: SystemKey,
    previous: ControlKind,
) -> serde_json::Value {
    serde_json::json!({
        "system": sys_name(world, system),
        "from": format!("{previous:?}"),
        "to": format!("{:?}", ControlKind::Uncontrolled),
        "cause": "planet_destroyed",
    })
}

/// A decision trace as a telemetry record: `system` names the subsystem,
/// `event_type` is `trace` and `details` holds the `decision` and its
/// inputs, whatever fields they have.
#[must_use]
pub fn trace_record(tick: u64, wall_ms: u64, trace: TraceRecord) -> GameEventRecord {
    let mut details = match trace.details {
        serde_json::Value::Object(map) => map,
        other => serde_json::Map::from_iter([("value".to_owned(), other)]),
    };
    details.insert("decision".to_owned(), trace.kind.into());
    GameEventRecord::new(
        tick,
        wall_ms,
        trace.subsystem,
        EVT_TRACE,
        serde_json::Value::Object(details),
    )
}

/// Take the traces the tick recorded as telemetry records.
#[must_use]
pub fn drain_trace_records(tick: u64, wall_ms: u64) -> Vec<GameEventRecord> {
    rebellion_core::trace::drain()
        .into_iter()
        .map(|trace| trace_record(tick, wall_ms, trace))
        .collect()
}

#[cfg(not(target_arch = "wasm32"))]
mod native_sink {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use std::sync::{Mutex, MutexGuard};
    use std::time::Instant;

    use rebellion_core::game_events::GameEventRecord;

    struct Sink {
        file: File,
        started: Instant,
        seq: Option<u64>,
    }

    static SINK: Mutex<Option<Sink>> = Mutex::new(None);

    fn lock() -> MutexGuard<'static, Option<Sink>> {
        SINK.lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    pub fn enable(path: &Path) -> std::io::Result<()> {
        *lock() = Some(Sink {
            file: File::create(path)?,
            started: Instant::now(),
            seq: None,
        });
        Ok(())
    }

    pub fn enabled() -> bool {
        lock().is_some()
    }

    pub fn set_seq(seq: Option<u64>) {
        if let Some(sink) = lock().as_mut() {
            sink.seq = seq;
        }
    }

    /// Wall-clock milliseconds since the sink opened, 0 when it is closed.
    pub fn wall_ms() -> u64 {
        lock().as_ref().map_or(0, |sink| {
            u64::try_from(sink.started.elapsed().as_millis()).unwrap_or(u64::MAX)
        })
    }

    pub fn write(record: GameEventRecord) {
        if let Some(sink) = lock().as_mut() {
            let record = record.native(sink.seq);
            if let Ok(mut line) = serde_json::to_vec(&record) {
                line.push(b'\n');
                let _ = sink.file.write_all(&line);
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod native_sink {
    use rebellion_core::game_events::GameEventRecord;

    pub fn enable(_: &std::path::Path) -> std::io::Result<()> {
        Ok(())
    }
    pub fn enabled() -> bool {
        false
    }
    pub fn set_seq(_: Option<u64>) {}
    pub fn wall_ms() -> u64 {
        0
    }
    pub fn write(_: GameEventRecord) {}
}

/// Open `telemetry.jsonl` in the evidence directory `dir` for the native
/// game, and turn on the traces `OPEN_REBELLION_TRACE` names.
///
/// # Errors
///
/// Returns the file error, or the unknown trace subsystem.
pub fn enable_native(dir: &std::path::Path) -> std::io::Result<()> {
    let mask = match std::env::var("OPEN_REBELLION_TRACE") {
        Ok(spec) => rebellion_core::trace::TraceMask::parse(&spec)
            .map_err(|why| std::io::Error::new(std::io::ErrorKind::InvalidInput, why))?,
        Err(_) => rebellion_core::trace::TraceMask::NONE,
    };
    native_sink::enable(&dir.join("telemetry.jsonl"))?;
    rebellion_core::trace::set_mask(mask);
    Ok(())
}

/// Start the native game's telemetry when `OPEN_REBELLION_TELEMETRY` is set
/// (the caller gates it on the developer palette):
/// `telemetry.jsonl` goes in `OPEN_REBELLION_EVIDENCE`, or the working
/// directory without it. Logs and returns on a failure, so the game runs
/// without telemetry.
pub fn enable_native_from_env() {
    if std::env::var_os("OPEN_REBELLION_TELEMETRY").is_none() {
        return;
    }
    let dir = std::env::var_os("OPEN_REBELLION_EVIDENCE")
        .map_or_else(|| std::path::PathBuf::from("."), std::path::PathBuf::from);
    match enable_native(&dir) {
        Ok(()) => eprintln!("[telemetry] writing {}/telemetry.jsonl", dir.display()),
        Err(error) => eprintln!("[telemetry] disabled: {error}"),
    }
}

/// Whether the native game writes telemetry.
#[must_use]
pub fn native_enabled() -> bool {
    native_sink::enabled()
}

/// Note the developer-channel command running, for the records that follow.
pub fn set_native_seq(seq: Option<u64>) {
    native_sink::set_seq(seq);
}

/// Write one native record; a no-op while telemetry is off.
pub fn emit_native(
    tick: u64,
    system: &'static str,
    event_type: &'static str,
    details: serde_json::Value,
) {
    if native_sink::enabled() {
        native_sink::write(GameEventRecord::new(
            tick,
            native_sink::wall_ms(),
            system,
            event_type,
            details,
        ));
    }
}

/// Write the traces the native tick recorded, and drop them when telemetry
/// is off so the buffer never grows.
pub fn flush_native_traces(tick: u64) {
    let wall_ms = native_sink::wall_ms();
    for trace in rebellion_core::trace::drain() {
        if native_sink::enabled() {
            native_sink::write(trace_record(tick, wall_ms, trace));
        }
    }
}

// ---------------------------------------------------------------------------
// Native call sites: each builds its record with the shared payload above.
// ---------------------------------------------------------------------------

/// A space battle the native game resolved.
pub fn native_combat_space(tick: u64, world: &GameWorld, result: &SystemSpaceCombatResult) {
    if native_enabled() {
        emit_native(
            tick,
            SYS_COMBAT,
            EVT_COMBAT_SPACE,
            combat_space_payload(world, result),
        );
    }
}

/// A mission the native game resolved.
pub fn native_mission_resolved(world: &GameWorld, result: &MissionResult) {
    if native_enabled() {
        emit_native(
            result.tick,
            SYS_MISSIONS,
            EVT_MISSION_RESOLVED,
            mission_resolved_payload(world, result),
        );
        if let Some(payload) = mission_espionage_payload(world, result) {
            emit_native(result.tick, SYS_MISSIONS, EVT_MISSION_RESOLVED, payload);
        }
    }
}

/// The AI orders one native step applied, refused or held, written by the
/// headless emitters (`PerceptionIntegrator::emit_ai_order`, `emit_strike`,
/// `emit_assault`): each order's `ai_action`, an applied dispatch's
/// `mission_dispatched`, a strike's bombardment, Death Star shot and deaths,
/// and an assault's run and control change. `tick` is the day the step ran.
pub fn native_ai_records(
    tick: u64,
    world: &GameWorld,
    records: &[crate::simulation::AiOrderRecord],
) {
    if native_enabled() {
        let mut integrator =
            crate::integrator::PerceptionIntegrator::new(tick, native_sink::wall_ms());
        for record in records {
            integrator.emit_ai_order(
                world,
                &record.issued.action,
                &record.origin(),
                &record.outcome,
            );
            integrator.emit_strike(world, &record.strike);
            integrator.emit_assault(world, &record.assault);
            integrator.emit_fired(world, &record.fired.shifts, &record.fired.controls);
            integrator.emit_swept(world, &record.swept);
        }
        for event in integrator.finish() {
            native_sink::write(event);
        }
    }
}

/// The records of the player's own strike, as `native_ai_records` writes an
/// AI strike's.
pub fn native_strike(tick: u64, world: &GameWorld, strike: &crate::integrator::StrikeEffects) {
    if native_enabled() {
        let mut integrator =
            crate::integrator::PerceptionIntegrator::new(tick, native_sink::wall_ms());
        integrator.emit_strike(world, strike);
        for event in integrator.finish() {
            native_sink::write(event);
        }
    }
}

/// The native game's trips of the characters a holder change sent off, as
/// headless records them (`PerceptionIntegrator::emit_swept`).
pub fn native_swept(
    tick: u64,
    world: &GameWorld,
    swept: &[(SystemKey, Vec<crate::integrator::AdriftTrip>)],
) {
    if native_enabled() && !swept.is_empty() {
        let mut integrator =
            crate::integrator::PerceptionIntegrator::new(tick, native_sink::wall_ms());
        integrator.emit_swept(world, swept);
        for event in integrator.finish() {
            native_sink::write(event);
        }
    }
}

/// The native game's adrift trips, as headless records them
/// (`PerceptionIntegrator::emit_adrift_trips`).
pub fn native_adrift_trips(
    tick: u64,
    world: &GameWorld,
    missions: &rebellion_core::missions::MissionState,
    system: SystemKey,
    started: &[u64],
) {
    if native_enabled() && !started.is_empty() {
        let mut integrator =
            crate::integrator::PerceptionIntegrator::new(tick, native_sink::wall_ms());
        integrator.emit_adrift_trips(world, missions, system, started);
        for event in integrator.finish() {
            native_sink::write(event);
        }
    }
}

/// The native game's overdraft scraps, as headless records them
/// (`PerceptionIntegrator::emit_maintenance_shortfall`).
pub fn native_maintenance_shortfall(
    tick: u64,
    world: &GameWorld,
    transport: &rebellion_core::troop_transport::TroopTransportState,
    events: &[rebellion_core::stockpiles::StockpileEvent],
) {
    if native_enabled() {
        let mut integrator =
            crate::integrator::PerceptionIntegrator::new(tick, native_sink::wall_ms());
        integrator.emit_maintenance_shortfall(world, transport, events);
        for event in integrator.finish() {
            native_sink::write(event);
        }
    }
}

/// A side's AI day the native game closed.
pub fn native_ai_day(tick: u64, day: &AiDay<'_>) {
    if native_enabled() {
        emit_native(tick, SYS_AI, EVT_AI_DAY, ai_day_payload(day));
    }
}

/// The missions the native game removed.
pub fn native_mission_ended(ended: &[MissionEnded]) {
    if native_enabled() {
        for record in ended {
            emit_native(
                record.tick,
                SYS_MISSIONS,
                EVT_MISSION_ENDED,
                mission_ended_payload(record),
            );
        }
    }
}

/// A blockade the native game started or ended. Call it as the event is
/// applied, so an end's record reads the world its shift reads.
pub fn native_blockade_event(
    world: &GameWorld,
    blockade: &rebellion_core::blockade::BlockadeState,
    event: &BlockadeEvent,
) {
    if !native_enabled() {
        return;
    }
    match event {
        BlockadeEvent::BlockadeStarted { system, tick } => emit_native(
            *tick,
            SYS_BLOCKADE,
            EVT_BLOCKADE_STARTED,
            blockade_payload(world, *system),
        ),
        BlockadeEvent::BlockadeEnded { system, tick } => emit_native(
            *tick,
            SYS_BLOCKADE,
            EVT_BLOCKADE_ENDED,
            blockade_end_payload(world, blockade, *system),
        ),
        BlockadeEvent::TroopDestroyed { .. } | BlockadeEvent::SupportShift { .. } => {}
    }
}

/// A blockade support shift the native game applied (timer `0x386`).
pub fn native_support_shift(
    tick: u64,
    world: &GameWorld,
    system: SystemKey,
    side: rebellion_core::dat::Faction,
    delta: i32,
) {
    if native_enabled() {
        emit_native(
            tick,
            SYS_BLOCKADE,
            EVT_BLOCKADE_SUPPORT_SHIFT,
            support_shift_payload(world, system, side, delta),
        );
    }
}

/// The `details` of a `support_shift` record: the system, what posted the
/// shift, the posted Alliance amount, the points it moved and the support
/// after it.
#[must_use]
pub fn applied_shift_payload(
    world: &GameWorld,
    shift: &rebellion_core::support_shift::AppliedShift,
) -> serde_json::Value {
    let support = |side| {
        world
            .systems
            .get(shift.system)
            .map(|sys| rebellion_core::uprising::support_points(sys, side))
    };
    serde_json::json!({
        "system": sys_name(world, shift.system),
        "cause": format!("{:?}", shift.cause),
        "amount": shift.amount,
        "delta": shift.delta,
        "alliance_support": support(rebellion_core::dat::Faction::Alliance),
    })
}

/// What the native game's phase 5 served (`support_shift::fire`): each
/// control change the support hook made ([`control_resolved_payload`]),
/// then each fired shift ([`applied_shift_payload`]).
pub fn native_support_shifts(
    tick: u64,
    world: &GameWorld,
    fired: &rebellion_core::support_shift::Fired,
) {
    if native_enabled() {
        for change in &fired.controls {
            emit_native(
                tick,
                SYS_ECONOMY,
                EVT_CONTROL_CHANGED,
                control_resolved_payload(world, change.system, change.control),
            );
        }
        for shift in &fired.shifts {
            emit_native(
                tick,
                SYS_ECONOMY,
                EVT_SUPPORT_SHIFT,
                applied_shift_payload(world, shift),
            );
        }
    }
}

/// An economy event's record in the native game ([`economy_record`]).
pub fn native_economy_record(tick: u64, world: &GameWorld, event: &EconomyEvent) {
    if native_enabled() {
        if let Some((event_type, details)) = economy_record(world, event) {
            emit_native(tick, SYS_ECONOMY, event_type, details);
        }
    }
}

/// A regiment the native game lost leaving a blockaded system.
pub fn native_troop_destroyed(tick: u64, world: &GameWorld, system: SystemKey) {
    if native_enabled() {
        emit_native(
            tick,
            SYS_BLOCKADE,
            EVT_BLOCKADE_TROOP_DESTROYED,
            blockade_payload(world, system),
        );
    }
}

/// The levels the native game's sides bought.
/// The native game's repair steps and timer starts, as the integrator
/// records them.
pub fn native_repair_events(tick: u64, world: &GameWorld, events: &[RepairEvent]) {
    if native_enabled() {
        for (event_type, details) in events.iter().filter_map(|e| repair_payload(world, e)) {
            emit_native(tick, SYS_REPAIR, event_type, details);
        }
    }
}

pub fn native_research_results(tick: u64, results: &[ResearchResult]) {
    if native_enabled() {
        for result in results {
            emit_native(
                tick,
                SYS_RESEARCH,
                EVT_RESEARCH_UNLOCKED,
                research_unlocked_payload(result),
            );
        }
    }
}

/// The records of the player's own assault, as `native_ai_records` writes
/// an AI assault's.
pub fn native_assault(tick: u64, world: &GameWorld, assault: &crate::integrator::AssaultEffects) {
    if native_enabled() {
        let mut integrator =
            crate::integrator::PerceptionIntegrator::new(tick, native_sink::wall_ms());
        integrator.emit_assault(world, assault);
        for event in integrator.finish() {
            native_sink::write(event);
        }
    }
}

/// A space battle's strength before the native game resolves it.
pub struct NativeBattleBefore {
    alliance: crate::integrator::SpaceForceSummary,
    empire: crate::integrator::SpaceForceSummary,
    alliance_fleets: usize,
    empire_fleets: usize,
}

/// Note a battle's strength just before the native game applies it, and
/// trace it as `pre_battle` (no `rng_seed`: the session holds its own
/// stream). `None` while telemetry is off.
pub fn native_battle_begin(
    tick: u64,
    world: &GameWorld,
    system: SystemKey,
) -> Option<NativeBattleBefore> {
    if !native_enabled() {
        return None;
    }
    let (alliance_fleets, empire_fleets) =
        crate::integrator::orbiting_fleets_by_faction(world, system);
    let before = NativeBattleBefore {
        alliance: crate::integrator::summarize_space_force(world, system, true, &alliance_fleets),
        empire: crate::integrator::summarize_space_force(world, system, false, &empire_fleets),
        alliance_fleets: alliance_fleets.len(),
        empire_fleets: empire_fleets.len(),
    };
    rebellion_core::trace::record(
        rebellion_core::trace::TraceMask::COMBAT,
        "pre_battle",
        || {
            serde_json::json!({
                "system": sys_name(world, system),
                "tick": tick,
                "rng_seed": serde_json::Value::Null,
                "alliance_fleets": before.alliance_fleets,
                "empire_fleets": before.empire_fleets,
                "alliance": force_payload(&before.alliance),
                "empire": force_payload(&before.empire),
            })
        },
    );
    Some(before)
}

/// The `combat_space` record of a battle the native game has applied, from
/// the strength `native_battle_begin` noted, the session's length and who
/// holds the system after the post-battle pass, as an unwatched battle
/// reports it (`crate::integrator::post_battle_outcome`).
pub fn native_battle_end(
    tick: u64,
    world: &GameWorld,
    system: SystemKey,
    before: Option<NativeBattleBefore>,
    seconds: u32,
) {
    let Some(before) = before else {
        return;
    };
    let result = native_battle_result(world, system, before, seconds);
    emit_native(
        tick,
        SYS_COMBAT,
        EVT_COMBAT_SPACE,
        combat_space_payload(world, &result),
    );
}

/// The applied battle's result as `native_battle_end` reports it.
fn native_battle_result(
    world: &GameWorld,
    system: SystemKey,
    before: NativeBattleBefore,
    seconds: u32,
) -> SystemSpaceCombatResult {
    let (alliance_fleets, empire_fleets) =
        crate::integrator::orbiting_fleets_by_faction(world, system);
    let (winner, stalemate) = crate::integrator::post_battle_outcome(world, system);
    SystemSpaceCombatResult {
        system,
        winner,
        winner_fleet: None,
        seconds,
        alliance_fleets: before.alliance_fleets,
        empire_fleets: before.empire_fleets,
        alliance_before: before.alliance,
        alliance_after: crate::integrator::summarize_space_force(
            world,
            system,
            true,
            &alliance_fleets,
        ),
        empire_before: before.empire,
        empire_after: crate::integrator::summarize_space_force(
            world,
            system,
            false,
            &empire_fleets,
        ),
        stalemate,
        log: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::world::{CapitalShipClass, Fleet, ShipInstance, System};

    /// A system holding a usable fleet of each side in `present`.
    /// port: test fixture values, not rules.
    fn with_fleets(present: &[bool]) -> (GameWorld, SystemKey) {
        let mut world = GameWorld::default();
        let system = world.systems.insert(System::default());
        for &is_alliance in present {
            let class = world.capital_ship_classes.insert(CapitalShipClass {
                is_alliance,
                is_empire: !is_alliance,
                hull: 100,
                ..CapitalShipClass::default()
            });
            let fleet = world.fleets.insert(Fleet {
                location: system,
                capital_ships: vec![ShipInstance::new(class, 100, is_alliance)],
                is_alliance,
                ..Fleet::default()
            });
            world.systems[system].fleets.push(fleet);
        }
        (world, system)
    }

    fn before() -> NativeBattleBefore {
        NativeBattleBefore {
            alliance: crate::integrator::SpaceForceSummary::default(),
            empire: crate::integrator::SpaceForceSummary::default(),
            alliance_fleets: 1,
            empire_fleets: 1,
        }
    }

    // After the post-battle pass FUN_00517270.c:79-93 checks who is present
    // (fleets in mode 1 or system squadrons) and reports that side to the
    // galaxy, FUN_0050a040 writing it to the system's +0x78 bits 0-1 and
    // FUN_00517ee0 building the result; with both present it reports none
    // and the system stays contested. FUN_0040c780's winner (+0x64) only
    // steers the pass (FUN_00516ef0 through FUN_00509ad0). An applied
    // battle's record reports the same as an unwatched one's.
    #[test]
    fn an_applied_battle_reports_who_holds_the_system_after_the_pass() {
        for (present, winner, stalemate) in [
            (vec![true], "alliance", false),
            (vec![false], "empire", false),
            (vec![true, false], "draw", true),
        ] {
            let (world, system) = with_fleets(&present);
            let result = native_battle_result(&world, system, before(), 30);
            let payload = combat_space_payload(&world, &result);
            assert_eq!(payload["winner"], winner, "{present:?}");
            assert_eq!(payload["stalemate"], stalemate, "{present:?}");
        }
    }
}
