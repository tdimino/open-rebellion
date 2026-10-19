//! The Planetary Assault order the object menu posts (`0x234`) and the
//! messages its run files (`ghidra/notes/ground-assault.md`).

use rebellion_core::assault::{self, AssaultRefusal};
use rebellion_core::blockade::BlockadeState;
use rebellion_core::dat::Faction;
use rebellion_core::ids::FleetKey;
use rebellion_core::movement::MovementState;
use rebellion_core::troop_transport::TroopTransportState;
use rebellion_core::world::GameWorld;
use rebellion_data::integrator::{AssaultEffects, StrikeStates};
use rebellion_data::text_templates::TextTemplates;
use rebellion_render::message_log::{GameMessage, MessageLog};
use rebellion_render::object_menu::MenuObject;
use std::sync::Arc;

use crate::strike_orders::menu_team;

/// The order's enable slot for the selection's team
/// (`assault::order_enabled`, `FUN_005814d0`).
#[must_use]
pub fn assault_gate(
    world: &GameWorld,
    movement: &MovementState,
    transport: &TroopTransportState,
    blockade: &BlockadeState,
    selection: Option<MenuObject>,
    is_alliance: bool,
) -> bool {
    let team = menu_team(world, selection, is_alliance);
    assault::order_enabled(
        world,
        movement,
        transport,
        blockade,
        &team,
        Faction::of_alliance(is_alliance),
    )
    .is_ok()
}

/// Post the order to `fleets` as the menu row or the `Assault` dev command
/// does (`integrator::order_assault`), then record each run and file its
/// messages.
///
/// # Errors
/// The order's refusal (`assault::order_enabled`).
pub fn issue(
    world: &mut GameWorld,
    fleets: &[FleetKey],
    side: Faction,
    states: &mut StrikeStates<'_>,
    roll: &mut dyn FnMut() -> f64,
    templates: &TextTemplates,
    log: &mut MessageLog,
) -> Result<Vec<AssaultEffects>, AssaultRefusal> {
    let tick = states.tick;
    let runs = rebellion_data::integrator::order_assault(world, fleets, side, states, roll)?;
    for run in &runs {
        rebellion_data::telemetry::native_assault(tick, world, run);
        for message in assault_messages(world, templates, tick, run) {
            log.push(message);
        }
    }
    Ok(runs)
}

/// The messages of an assault, the player's or the AI's: event `0x150`
/// reaches the Operation Reports class (`FUN_00496460`, visitors
/// `FUN_00496770` and `FUN_00496f50`), which files notification `0xf` with
/// the run's lists on the Conflict rail for each side viewing the fleet
/// (`strike_orders::report_messages`, the assault branch: title `0x7090`,
/// body `0x7091`-`0x7094`, picture `0x2b98`/`0x2b99`, advice `0x2f`); then
/// the messages of the blockade the holder change started or lifted, and
/// the Loyalty Change Messages of each system its shifts turned
/// (`FUN_005109f0` -> `FUN_0050b610`).
#[must_use]
pub fn assault_messages(
    world: &GameWorld,
    templates: &TextTemplates,
    tick: u64,
    assault: &AssaultEffects,
) -> Vec<GameMessage> {
    let mut messages = assault.operation.as_ref().map_or_else(Vec::new, |operation| {
        crate::strike_orders::report_messages(world, templates, tick, &Arc::new(operation.clone()))
    });
    for (event, _) in &assault.blockade {
        messages.extend(crate::blockade_transition_messages(world, templates, event));
    }
    for change in &assault.controls {
        messages.extend(crate::control_change_messages(
            world, templates, tick, change,
        ));
    }
    messages
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::carriage::ProductRoom;
    use rebellion_core::assault::AssaultReport;
    use rebellion_core::operation_report::OperationReport;
    use rebellion_core::ids::{DatId, SystemKey};
    use rebellion_core::manufacturing::ManufacturingState;
    use rebellion_core::world::{
        CapitalShipClass, ControlKind, Fleet, ShipInstance, System, TroopUnit,
    };
    use rebellion_render::message_log::{MessageRail, MessageTarget, RailAudience};
    use rebellion_render::quadrant_icons::Quadrant;
    use std::collections::HashMap;

    const BOTH: [(Faction, Faction); 2] = [
        (Faction::Empire, Faction::Alliance),
        (Faction::Alliance, Faction::Empire),
    ];

    /// TEXTSTRA RCDATA bytes for `text`, where `{n:k}` is argument `n`
    /// rendered under kind `k` (`text_templates::format_template`).
    fn rcdata(text: &str) -> String {
        let mut out = String::new();
        let mut rest = text;
        while let Some(open) = rest.find('{') {
            out.push_str(&rest[..open]);
            let close = rest[open..].find('}').unwrap() + open;
            let (index, kind) = rest[open + 1..close].split_once(':').unwrap();
            out.push('|');
            out.push(char::from(index.parse::<u8>().unwrap()));
            out.push(char::from(kind.parse::<u8>().unwrap()));
            out.push_str("\0\0\0");
            rest = &rest[close + 1..];
        }
        out.push_str(rest);
        out.push('\u{1}');
        out
    }

    /// The shipped TEXTSTRA RCDATA entries `0x7090`-`0x7094` and
    /// `0x70a0`-`0x70a3`.
    fn templates() -> TextTemplates {
        let entries: HashMap<u16, String> = [
            (0x7090, "Assault on {1:1}"),
            (
                0x7091,
                "{1:4} troops have taken control of the {2:4} system {3:1}",
            ),
            (
                0x7092,
                "{2:4} Troops have defended {3:1} from an {1:4} assault.",
            ),
            (
                0x7093,
                "{1:4} troops have seized control of the neutral system {2:1}.",
            ),
            (
                0x7094,
                "The neutral system {2:1} has repulsed an attack by {1:4} troops.",
            ),
            (0x70a0, "Fleet Initiates Blockade of {1:1}"),
            (0x70a1, "{3:1} has initiated a blockade of {2:1}."),
            (0x70a2, "{1:1} Under Blockade"),
            (
                0x70a3,
                "{1:4} ships have been detected at {2:1}.  The world is under enemy blockade.",
            ),
        ]
        .into_iter()
        .map(|(id, text)| (id, rcdata(text)))
        .collect();
        TextTemplates::from_json(&serde_json::to_vec(&entries).unwrap()).unwrap()
    }

    fn world(holder: Option<Faction>) -> (GameWorld, SystemKey) {
        let mut world = GameWorld::default();
        let system = world.systems.insert(System {
            name: "Selonia".into(),
            is_populated: true,
            control: holder.map_or(ControlKind::Uncontrolled, ControlKind::Controlled),
            ..System::default()
        });
        (world, system)
    }

    /// A run at `system` that leaves `after` holding it: the report and
    /// the system's holder, which the report's body reads.
    fn assault(
        world: &mut GameWorld,
        system: SystemKey,
        assaulter: Faction,
        holder: Faction,
        after: Faction,
    ) -> AssaultEffects {
        world.systems[system].control = match after {
            Faction::Neutral => ControlKind::Uncontrolled,
            side => ControlKind::Controlled(side),
        };
        AssaultEffects {
            operation: Some(OperationReport::new(system, true, holder, assaulter)),
            report: Some(AssaultReport {
                system: Some(system),
                assaulter: Some(assaulter),
                holder_before: Some(holder),
                holder_after: Some(after),
                ..AssaultReport::default()
            }),
            ..AssaultEffects::default()
        }
    }

    fn adjective(side: Faction) -> &'static str {
        if side == Faction::Alliance {
            "Alliance"
        } else {
            "Imperial"
        }
    }

    #[test]
    fn a_taken_or_defended_system_files_an_operation_report_for_both_sides() {
        // FUN_00496770's assault branch: 0x7090, then 0x7091 when the holder
        // after the run is the assaulter, else 0x7092; picture 0x2b98 for an
        // Alliance assaulter, 0x2b99 for the Empire; advice 0x2f for the side
        // that is not the assaulter.
        let templates = templates();
        for (assaulter, holder) in BOTH {
            let (mut world, system) = world(Some(holder));
            let (by, at) = (adjective(assaulter), adjective(holder));
            for (after, body) in [
                (
                    assaulter,
                    format!("{by} troops have taken control of the {at} system Selonia"),
                ),
                (
                    holder,
                    format!("{at} Troops have defended Selonia from an {by} assault."),
                ),
            ] {
                let effects = assault(&mut world, system, assaulter, holder, after);
                let messages = assault_messages(&world, &templates, 7, &effects);
                assert_eq!(messages.len(), 2, "{assaulter:?}");
                for message in &messages {
                    let display = message.display.as_ref().unwrap();
                    assert_eq!(display.title, "Assault on Selonia");
                    assert_eq!(display.body, body);
                    let picture = if assaulter == Faction::Alliance {
                        0x2b98
                    } else {
                        0x2b99
                    };
                    assert_eq!(display.background, picture);
                    assert_eq!(message.rail, Some(MessageRail::Conflict));
                    assert_eq!(message.target, Some(MessageTarget::System(system)));
                }
                let alliance = assaulter == Faction::Alliance;
                assert_eq!(messages[0].audience, RailAudience::side(alliance));
                assert_eq!(messages[0].advice, None);
                assert_eq!(messages[1].audience, RailAudience::side(!alliance));
                assert_eq!(messages[1].advice, Some(0x2f));
            }
        }
    }

    #[test]
    fn each_assault_report_message_carries_the_runs_lists() {
        // Event 0x150 reaches the Operation Reports class: each side's
        // message is a kind-0xf report (ctor FUN_00496460) holding the
        // run's lists, which the report window shows (FUN_0044c410;
        // operation-report-window.md).
        let templates = templates();
        for (assaulter, holder) in BOTH {
            let (mut world, system) = world(Some(holder));
            for after in [assaulter, holder] {
                let effects = assault(&mut world, system, assaulter, holder, after);
                let messages = assault_messages(&world, &templates, 7, &effects);
                assert_eq!(messages.len(), 2, "{assaulter:?}");
                for message in &messages {
                    assert_eq!(
                        message.report.as_deref(),
                        effects.operation.as_ref(),
                        "{assaulter:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn a_neutral_system_files_the_seized_or_repulsed_report_for_the_assaulter() {
        // 0x7093 / 0x7094 for a holder of no side; a neutral system's view
        // holds no fleet (strike_orders::recipients).
        let templates = templates();
        for (assaulter, _) in BOTH {
            let (mut world, system) = world(None);
            let by = adjective(assaulter);
            for (after, body) in [
                (
                    assaulter,
                    format!("{by} troops have seized control of the neutral system Selonia."),
                ),
                (
                    Faction::Neutral,
                    format!("The neutral system Selonia has repulsed an attack by {by} troops."),
                ),
            ] {
                let effects = assault(&mut world, system, assaulter, Faction::Neutral, after);
                let messages = assault_messages(&world, &templates, 7, &effects);
                assert_eq!(messages.len(), 1, "{assaulter:?}");
                assert_eq!(messages[0].display.as_ref().unwrap().body, body);
            }
        }
    }

    #[test]
    fn a_blockade_the_assault_starts_files_its_blockade_message_and_a_lift_files_none() {
        // Event 0x311's rerun sets or clears the bit with the run
        // (FUN_0050b8e0) and posts event 0x14e. FUN_004960f0 files only
        // while a blockading fleet is there (FUN_0052bed0): the holder reads
        // "| Under Blockade" (0x70a2), the other side "Fleet Initiates
        // Blockade of |" (0x70a0). A lift, or a start with no enemy fleet,
        // files nothing.
        use rebellion_core::blockade::BlockadeEvent;
        let blockade = |event| AssaultEffects {
            blockade: vec![(event, serde_json::Value::Null)],
            ..AssaultEffects::default()
        };
        for (holder, blockader) in BOTH {
            let (mut world, system) = world(Some(holder));
            let started = BlockadeEvent::BlockadeStarted { system, tick: 7 };
            assert!(
                assault_messages(&world, &templates(), 7, &blockade(started.clone())).is_empty()
            );
            // The holder's own fleet there, or an enemy fleet elsewhere,
            // blockades nothing.
            let elsewhere = world.systems.insert(System::default());
            for (location, side) in [(system, holder), (elsewhere, blockader)] {
                world.fleets.insert(Fleet {
                    location,
                    is_alliance: side == Faction::Alliance,
                    ..Fleet::default()
                });
            }
            assert!(
                assault_messages(&world, &templates(), 7, &blockade(started.clone())).is_empty()
            );
            world.fleets.insert(Fleet {
                location: system,
                is_alliance: blockader == Faction::Alliance,
                ..Fleet::default()
            });
            let messages = assault_messages(&world, &templates(), 7, &blockade(started.clone()));
            let shown: Vec<(&str, RailAudience)> = messages
                .iter()
                .map(|message| (message.text.as_str(), message.audience))
                .collect();
            let (under, initiates) = (
                "Selonia Under Blockade",
                "Fleet Initiates Blockade of Selonia",
            );
            let (alliance, empire) = if holder == Faction::Alliance {
                (under, initiates)
            } else {
                (initiates, under)
            };
            assert_eq!(
                shown,
                [
                    (alliance, RailAudience::side(true)),
                    (empire, RailAudience::side(false))
                ],
                "{holder:?}"
            );
            assert!(messages
                .iter()
                .all(|message| message.system == Some(system)));
            // 0x70a3 names the blockading side's ships.
            let held = &messages[usize::from(holder == Faction::Empire)];
            assert_eq!(
                held.display.as_ref().unwrap().body,
                format!(
                    "{} ships have been detected at Selonia.  The world is under enemy blockade.",
                    adjective(blockader)
                )
            );
            let ended = BlockadeEvent::BlockadeEnded { system, tick: 7 };
            assert!(assault_messages(&world, &templates(), 7, &blockade(ended)).is_empty());
        }
    }

    #[test]
    fn no_run_files_nothing() {
        let (world, _) = world(None);
        assert!(assault_messages(&world, &templates(), 7, &AssaultEffects::default()).is_empty());
    }

    /// `side`'s fleet of one transport with one regiment aboard at an
    /// enemy-held `system`. port: test fixture values, not rules.
    fn loaded_fleet(
        world: &mut GameWorld,
        transport: &mut TroopTransportState,
        system: SystemKey,
        side: Faction,
    ) -> FleetKey {
        let is_alliance = side == Faction::Alliance;
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            family: 0x14,
            hull: 100,
            troop_capacity: 2,
            ..CapitalShipClass::default()
        });
        let fleet = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![ShipInstance::new(class, 100, is_alliance)],
            is_alliance,
            ..Fleet::default()
        });
        world.systems[system].fleets.push(fleet);
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1000_0001),
            is_alliance,
        });
        world.systems[system].ground_units.push(troop);
        transport
            .embark(world, &ProductRoom::default(), fleet, &[troop])
            .unwrap();
        fleet
    }

    fn shield_limit(world: &mut GameWorld) {
        let entry = |id: u32, value: i32| rebellion_core::world::GnprtbEntry {
            parameter_id: id,
            development: value,
            alliance_sp_easy: value,
            alliance_sp_medium: value,
            alliance_sp_hard: value,
            empire_sp_easy: value,
            empire_sp_medium: value,
            empire_sp_hard: value,
            multiplayer: value,
        };
        // GNPRTB 0x1e03 (2), 7761 (60) and 7762 (-10) as shipped.
        world.gnprtb = rebellion_core::world::GnprtbParams::new(vec![
            entry(0x1e03, 2),
            entry(7761, 60),
            entry(7762, -10),
        ]);
    }

    #[test]
    fn the_menu_row_follows_the_orders_enable_slot_for_a_fleet_and_its_icon() {
        // FUN_005814d0 on the team: the blockading fleet with a regiment
        // aboard passes; another side's fleet and an unblockaded system
        // refuse (FUN_004ffa70, FUN_004fddd0).
        let movement = MovementState::new();
        for (side, holder) in BOTH {
            let (mut world, system) = world(Some(holder));
            shield_limit(&mut world);
            let mut transport = TroopTransportState::new();
            let fleet = loaded_fleet(&mut world, &mut transport, system, side);
            let mut blockade = BlockadeState::new();
            rebellion_core::blockade::BlockadeSystem::advance(
                &mut blockade,
                &world,
                &[rebellion_core::tick::TickEvent { tick: 1 }],
            );
            let is_alliance = side == Faction::Alliance;
            let icon = Some(MenuObject::SystemIcon {
                system,
                quadrant: Quadrant::Fleets,
            });
            for selection in [Some(MenuObject::Fleet(fleet)), icon] {
                assert!(
                    assault_gate(
                        &world,
                        &movement,
                        &transport,
                        &blockade,
                        selection,
                        is_alliance
                    ),
                    "{side:?}"
                );
                assert!(!assault_gate(
                    &world,
                    &movement,
                    &transport,
                    &BlockadeState::new(),
                    selection,
                    is_alliance
                ));
            }
            assert!(!assault_gate(
                &world,
                &movement,
                &transport,
                &blockade,
                Some(MenuObject::Fleet(fleet)),
                !is_alliance
            ));
        }
    }

    #[test]
    fn a_menu_row_posts_the_order_and_files_its_report_on_both_sides() {
        // FUN_005814d0 passes, the run lands the regiment on the undefended
        // system (FUN_0058c810) and the report goes to both sides; the new
        // holder's fleet ends the blockade at once (event 0x311,
        // FUN_0050b8e0), which files nothing: FUN_004960f0 needs a
        // blockading fleet.
        for (side, holder) in BOTH {
            let (mut world, system) = world(Some(holder));
            shield_limit(&mut world);
            let mut movement = MovementState::new();
            let mut manufacturing = ManufacturingState::new();
            let mut transport = TroopTransportState::new();
            let fleet = loaded_fleet(&mut world, &mut transport, system, side);
            let mut deliveries = rebellion_core::delivery::DeliveryState::new();
            let mut missions = rebellion_core::missions::MissionState::new();
            let uprisings = rebellion_core::uprising::UprisingState::new();
            let mut blockade = BlockadeState::new();
            rebellion_core::blockade::BlockadeSystem::advance(
                &mut blockade,
                &world,
                &[rebellion_core::tick::TickEvent { tick: 1 }],
            );
            let mut log = MessageLog::default();
            let runs = issue(
                &mut world,
                &[fleet],
                side,
                &mut StrikeStates {
                    movement: &mut movement,
                    manufacturing: &mut manufacturing,
                    transport: &mut transport,
                    deliveries: &mut deliveries,
                    missions: &mut missions,
                    uprisings: &uprisings,
                    blockade: &mut blockade,
                    tick: 3,
                    step: 0,
                },
                &mut || 0.5,
                &templates(),
                &mut log,
            )
            .unwrap();
            assert_eq!(runs.len(), 1, "{side:?}");
            assert_eq!(world.systems[system].control, ControlKind::Controlled(side));
            let titles: Vec<&str> = log
                .messages()
                .iter()
                .map(|message| message.text.as_str())
                .collect();
            // The two Operation Reports alone: the landing's holder change
            // has troop bits in its new word, so FUN_00510d70 writes the
            // loyalty bit (+0x88 0x2000) clear before the owner write posts
            // notification 0x100 (FUN_00510d70.c:12-19), and FUN_00499760
            // files no Loyalty Change Message (FUN_00499760.c:23-29).
            assert_eq!(titles, ["Assault on Selonia", "Assault on Selonia"]);
            assert!(!blockade.is_blockaded(system), "{side:?}");
            assert!(transport.cargo(fleet).is_empty());
        }
    }

    #[test]
    fn the_order_runs_once_per_system_of_its_team_with_every_team_fleet() {
        // FUN_00563390 runs one system command per system of the team, and
        // FUN_004feb60 flags every team fleet (+0x58 bit 7) that the run
        // walks (FUN_0058ba10; ground-assault.md, sections 2-3): two fleets
        // over one system are one run with both fleets' regiments, and a
        // third fleet over another system of the holder is a second run.
        for (side, holder) in BOTH {
            let (mut world, system) = world(Some(holder));
            shield_limit(&mut world);
            let other = world.systems.insert(System {
                name: "Drall".into(),
                is_populated: true,
                control: ControlKind::Controlled(holder),
                ..System::default()
            });
            let mut movement = MovementState::new();
            let mut manufacturing = ManufacturingState::new();
            let mut transport = TroopTransportState::new();
            let team = [
                loaded_fleet(&mut world, &mut transport, system, side),
                loaded_fleet(&mut world, &mut transport, system, side),
                loaded_fleet(&mut world, &mut transport, other, side),
            ];
            let mut deliveries = rebellion_core::delivery::DeliveryState::new();
            let mut missions = rebellion_core::missions::MissionState::new();
            let uprisings = rebellion_core::uprising::UprisingState::new();
            let mut blockade = BlockadeState::new();
            rebellion_core::blockade::BlockadeSystem::advance(
                &mut blockade,
                &world,
                &[rebellion_core::tick::TickEvent { tick: 1 }],
            );
            let runs = issue(
                &mut world,
                &team,
                side,
                &mut StrikeStates {
                    movement: &mut movement,
                    manufacturing: &mut manufacturing,
                    transport: &mut transport,
                    deliveries: &mut deliveries,
                    missions: &mut missions,
                    uprisings: &uprisings,
                    blockade: &mut blockade,
                    tick: 3,
                    step: 0,
                },
                &mut || 0.5,
                &templates(),
                &mut MessageLog::default(),
            )
            .unwrap();
            let counted: Vec<(Option<SystemKey>, u32)> = runs
                .iter()
                .map(|run| {
                    let report = run.report.as_ref().unwrap();
                    (report.system, report.attackers)
                })
                .collect();
            assert_eq!(counted, [(Some(system), 2), (Some(other), 1)], "{side:?}");
        }
    }

    #[test]
    fn only_a_holder_change_without_regiments_files_the_loyalty_messages() {
        // FUN_00510d70.c:12-18 sets the loyalty bit (+0x88 0x2000) only when
        // neither control word has the regiment bits 0x30, before the owner
        // write posts notification 0x100 (:19); FUN_00499760.c:23-29 files
        // the Loyalty Change Message only with the bit set: a support flip
        // files it for both sides, a change with regiments at either end
        // files nothing.
        for (holder, side) in BOTH {
            let (world, system) = world(Some(holder));
            let filed = |control, troops| {
                let change = rebellion_core::support_shift::ControlRerun {
                    system,
                    previous: ControlKind::Controlled(holder),
                    control,
                    troops,
                };
                crate::control_change_messages(&world, &templates(), 7, &change)
                    .iter()
                    .map(|message| (message.text.clone(), message.advice))
                    .collect::<Vec<_>>()
            };
            // Each side's "| Joins" (code 1) or "| Joins Enemy" (code 2).
            let mut codes: Vec<_> = filed(ControlKind::Controlled(side), (false, false))
                .into_iter()
                .map(|(text, advice)| {
                    assert_eq!(text, "Loyalty change at Selonia");
                    advice
                })
                .collect();
            codes.sort_unstable();
            assert_eq!(codes, [Some(1), Some(2)], "{side:?}");
            for troops in [(true, false), (false, true), (true, true)] {
                assert!(
                    filed(ControlKind::Controlled(side), troops).is_empty(),
                    "{side:?} {troops:?}"
                );
            }
            // No change of side files nothing.
            assert!(filed(ControlKind::Controlled(holder), (false, false)).is_empty());
        }
    }

    #[test]
    fn the_shipped_templates_render_the_assault_reports() {
        // TEXTSTRA.DLL RCDATA 0x7090-0x7094 (ground-assault.md, "Report
        // messages").
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/base");
        let templates = TextTemplates::load(&path).unwrap();
        let (mut world, system) = world(Some(Faction::Alliance));
        let run = assault(&mut world, system, Faction::Empire, Faction::Alliance, Faction::Empire);
        let run = assault_messages(&world, &templates, 7, &run);
        assert_eq!(
            run[0].display.as_ref().unwrap().body,
            "Imperial troops have taken control of the Alliance system Selonia"
        );
        assert!(run.iter().all(|message| message.display.is_some()));
    }
}
