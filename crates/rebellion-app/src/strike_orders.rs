//! The Planetary Bombardment orders the object menu posts (`0x220`-`0x223`),
//! the messages a strike files (`ghidra/notes/bombardment.md`,
//! `ghidra/notes/move-order.md`, "The Fleet menu"), and the kind-6 movie
//! messages of the win word, Destroy System and Death Star sabotage
//! (`ghidra/notes/movies.md`).

use std::sync::Arc;

use rebellion_core::bombardment::{self, BombardKind, BombardRefusal};
use rebellion_core::dat::Faction;
use rebellion_core::ids::FleetKey;
use rebellion_core::missions::MissionEffect;
use rebellion_core::movement::{self, MovementState};
use rebellion_core::operation_report::OperationReport;
use rebellion_core::world::GameWorld;
use rebellion_data::integrator::{StrikeEffects, StrikeStates};
use rebellion_data::text_templates::{TemplateArg, TextTemplates};
use rebellion_render::message_log::{
    GameMessage, MessageCategory, MessageDisplay, MessageLog, MessageRail, MessageTarget,
    RailAudience,
};
use rebellion_render::object_menu::{BombardGates, MenuObject};
use rebellion_render::quadrant_icons::Quadrant;

use crate::movies::{self, MessageMovie};

/// The four orders in the submenu's order (`0x220..0x223`).
const KINDS: [BombardKind; 4] = [
    BombardKind::Military,
    BombardKind::Civilian,
    BombardKind::General,
    BombardKind::DestroySystem,
];

/// The fleets an order chosen on `selection` commands, its team: the fleet
/// itself (`FUN_0051d990` sets the selection as the team), or a fleet icon's
/// system's fleets of the player's side (`FUN_00512700`, kind `0x10`).
#[must_use]
pub fn menu_team(
    world: &GameWorld,
    selection: Option<MenuObject>,
    is_alliance: bool,
) -> Vec<FleetKey> {
    match selection {
        Some(MenuObject::Fleet(fleet)) => vec![fleet],
        Some(MenuObject::SystemIcon {
            system,
            quadrant: Quadrant::Fleets,
        }) => movement::system_side_fleets(world, system, is_alliance),
        _ => Vec::new(),
    }
}

/// Each order's enable slot for the selection's team
/// (`bombardment::order_enabled`, `FUN_00581e80`).
#[must_use]
pub fn bombard_gates(
    world: &GameWorld,
    movement: &MovementState,
    selection: Option<MenuObject>,
    is_alliance: bool,
) -> BombardGates {
    let team = menu_team(world, selection, is_alliance);
    let side = Faction::of_alliance(is_alliance);
    BombardGates {
        allowed: KINDS
            .map(|kind| bombardment::order_enabled(world, movement, &team, kind, side).is_ok()),
    }
}

/// Post an order to `fleets` as the menu row or the `Bombard`/`Destroy
/// system` dev command does (`integrator::order_strike`), then record each
/// run and file its messages; the caller files the runs' movies
/// ([`strike_movies`]).
///
/// # Errors
/// The order's refusal (`bombardment::gate`).
pub fn issue(
    world: &mut GameWorld,
    fleets: &[FleetKey],
    kind: BombardKind,
    states: &mut StrikeStates<'_>,
    roll: &mut dyn FnMut() -> f64,
    templates: &TextTemplates,
    log: &mut MessageLog,
) -> Result<Vec<StrikeEffects>, BombardRefusal> {
    let tick = states.tick;
    let runs = rebellion_data::integrator::order_strike(world, fleets, kind, states, roll)?;
    for strike in &runs {
        rebellion_data::telemetry::native_strike(tick, world, strike);
        for message in strike_messages(world, templates, tick, strike) {
            log.push(message);
        }
    }
    Ok(runs)
}

/// The sides told of a strike. The fleet's bombard bit changes in each
/// side's view (`FUN_004fe460` -> `FUN_004ff450`), raising event `0x182`
/// there (`FUN_004ff840`), and the message classes take it only through
/// that side's view of the fleet (`FUN_004f36a0` in `FUN_00496770` and
/// `FUN_004950d0`). The bombarder holds its own fleet. hyp: a holder's view
/// holds every fleet in orbit of its system, and a bystander's view of a
/// neutral system holds none; recovered by reading the victim's Message
/// Index after a VM strike.
pub(crate) fn recipients(bombarder: Faction, holder: Faction) -> Vec<bool> {
    let mut sides = vec![bombarder == Faction::Alliance];
    if matches!(holder, Faction::Alliance | Faction::Empire) && holder != bombarder {
        sides.push(holder == Faction::Alliance);
    }
    sides
}

/// The adjective a side renders under kind 4 ("Imperial", "Alliance").
pub(crate) fn side_arg(side: Faction) -> Option<TemplateArg> {
    match side {
        Faction::Alliance => Some(TemplateArg::side(true)),
        Faction::Empire => Some(TemplateArg::side(false)),
        _ => None,
    }
}

/// The messages of a strike, the player's or the AI's.
///
/// - An ordinary run files notification `0xf`, an Operation Report with the
///   run's lists (`StrikeEffects::operation`; [`report_messages`]).
/// - Destroy System files notification `0x15`, Planet Destroyed
///   (`FUN_00492470`, visitor `FUN_004950d0`), a kind-6 movie message with
///   no row of its own ([`strike_movies`]). No Operation Report is filed
///   beside it: the kind chooser maps event `0x14f` to none once the
///   side's view of the system has its destroyed bit
///   (`FUN_00488de0.c:176-188`).
/// - A killed character files notification `0x1a`, Character Health
///   (`FUN_0048fdf0.c:42`, `:121-140`): title `0x7164` "| Killed", body
///   `0x7365` "| has been killed.", wave `0x474`, picture `0x42e` for side 1
///   and `0x433` otherwise. hyp: it files for the character's side; the
///   original files for each side whose view holds the character.
/// - A destroyed Alliance headquarters raises the AD bit, whose event
///   `0x128` files notification `0x1f`, Rebel HQ Destroyed, for both sides
///   with no side test (`FUN_0048e5a0`): `0x7188` / `0x7189`, advice code
///   `0x11`, on the Resource rail.
/// - Each system the run's shifts turned with no regiments at either end
///   files the Loyalty Change Messages (`FUN_005109f0` -> `FUN_0050b610`,
///   `crate::control_change_messages`).
#[must_use]
pub fn strike_messages(
    world: &GameWorld,
    templates: &TextTemplates,
    tick: u64,
    strike: &StrikeEffects,
) -> Vec<GameMessage> {
    let mut messages = run_messages(world, templates, tick, strike);
    for change in &strike.controls {
        messages.extend(crate::control_change_messages(
            world, templates, tick, change,
        ));
    }
    messages
}

/// [`strike_messages`] of the run itself.
fn run_messages(
    world: &GameWorld,
    templates: &TextTemplates,
    tick: u64,
    strike: &StrikeEffects,
) -> Vec<GameMessage> {
    let Some(report) = strike.report.as_ref() else {
        return Vec::new();
    };
    let (Some(_), Some(_)) = (report.system, report.bombarder) else {
        return Vec::new();
    };
    if report.planet_destroyed {
        let mut messages = Vec::new();
        for &character in &report.characters {
            let Some(value) = world.characters.get(character) else {
                continue;
            };
            let audience = if value.is_alliance {
                RailAudience::Alliance
            } else if value.is_empire {
                RailAudience::Empire
            } else {
                continue;
            };
            let who = TemplateArg::named(value.display_name());
            // The message has only TEXTSTRA's text; without it, none.
            let (Some(title), Some(body)) = (
                templates.format(0x7164, &[Some(&who)]),
                templates.format(0x7365, &[Some(&who)]),
            ) else {
                continue;
            };
            messages.push(
                GameMessage::new(tick, body.clone(), MessageCategory::Event)
                    .on_rail(MessageRail::Mission, audience)
                    .with_display(MessageDisplay {
                        title,
                        body,
                        background: if value.is_alliance { 0x42e } else { 0x433 },
                        sound: 0x474,
                        ..MessageDisplay::default()
                    }),
            );
        }
        return messages;
    }
    strike
        .operation
        .as_ref()
        .map_or_else(Vec::new, |operation| {
            report_messages(world, templates, tick, &Arc::new(operation.clone()))
        })
}

/// The Operation Reports (notification `0xf`, `FUN_00496460`) of a
/// bombardment or an assault, one per side told ([`recipients`]), each
/// carrying `report`'s lists, on the Conflict rail with its Display button
/// on the system (`+0x5c`). The code-2 visit (`FUN_00496770`) writes the
/// text and picture:
///
/// - a bombardment, whatever its order: title `0x7098` "Orbital
///   bombardment of |", body `0x7099` (fleet side, holder, system) or
///   `0x709a` for a neutral holder (`+0x68` = 0); advice `0x2e`;
/// - an assault: title `0x7090` "Assault on |"; with a holder, `0x7091`
///   when the attacker now holds the system, else `0x7092`; with none,
///   `0x7093`, else `0x7094`; advice `0x2f`;
/// - the picture by the lists ([`OperationReport::picture`]); the side
///   that is not the fleet owner gets the advice code.
#[must_use]
pub fn report_messages(
    world: &GameWorld,
    templates: &TextTemplates,
    tick: u64,
    report: &Arc<OperationReport>,
) -> Vec<GameMessage> {
    let system = report.system;
    // FUN_00496f50 files only for a system found in the side's view.
    let Some(name) = world.systems.get(system).map(|value| value.name.clone()) else {
        return Vec::new();
    };
    let place = TemplateArg::place(name);
    // iVar9: any fleet owner but side 1 reads as side 2.
    let owner = if report.fleet_owner == Faction::Alliance {
        Faction::Alliance
    } else {
        Faction::Empire
    };
    let owner_arg = side_arg(owner);
    let holder_arg = side_arg(report.holder);
    let (title, body, advice) = if report.assault {
        // `held`: the system's holder now (piVar4[9] >> 6 & 3) is not the
        // attacker.
        let held = world
            .systems
            .get(system)
            .and_then(|value| value.control.faction())
            != Some(report.fleet_owner);
        let body = match (&holder_arg, held) {
            (Some(holder), false) => {
                templates.format(0x7091, &[owner_arg.as_ref(), Some(holder), Some(&place)])
            }
            (Some(holder), true) => {
                templates.format(0x7092, &[owner_arg.as_ref(), Some(holder), Some(&place)])
            }
            (None, false) => templates.format(0x7093, &[owner_arg.as_ref(), Some(&place)]),
            (None, true) => templates.format(0x7094, &[owner_arg.as_ref(), Some(&place)]),
        };
        (templates.format(0x7090, &[Some(&place)]), body, 0x2f)
    } else {
        let body = match &holder_arg {
            Some(holder) => {
                templates.format(0x7099, &[owner_arg.as_ref(), Some(holder), Some(&place)])
            }
            None => templates.format(0x709a, &[owner_arg.as_ref(), Some(&place)]),
        };
        (templates.format(0x7098, &[Some(&place)]), body, 0x2e)
    };
    // The report has only TEXTSTRA's text; without it, none.
    let (Some(title), Some(body)) = (title, body) else {
        return Vec::new();
    };
    let display = MessageDisplay {
        title,
        body,
        background: report.picture(),
        overlay: 0,
        sound: 0,
        voice: 0,
    };
    let mut messages = Vec::new();
    for is_alliance in recipients(owner, report.holder) {
        let mut message =
            GameMessage::at_system(tick, display.title.clone(), MessageCategory::Combat, system)
                .with_target(MessageTarget::System(system))
                .on_rail(MessageRail::Conflict, RailAudience::side(is_alliance))
                .with_report(Arc::clone(report))
                .with_display(display.clone());
        if Faction::of_alliance(is_alliance) != owner {
            message = message.with_advice(advice);
        }
        messages.push(message);
    }
    messages
}

/// The message notification `0x128` files when the win word changes from
/// `old` to `word` (`VictorySystem::word`), to both sides: the kind chooser
/// (`FUN_00488de0.c:134-142`) picks Rebel HQ Destroyed `0x1f`
/// (`FUN_0048e5a0`) when AD turns on, else Coruscant Captured
/// (`FUN_0048f790`) when CA turns on, else nothing. Both are kind 6
/// (`FUN_0048d640`): movie-only messages that play their movie over the
/// body as its caption, with no Message Index row and no advice
/// (`FUN_0048a060`).
/// port: `FUN_0048f790` reads Coruscant's holder through each side's view;
/// the port reads it from the word's CA bit.
pub fn word_movies(templates: &TextTemplates, old: u32, word: u32) -> Vec<MessageMovie> {
    // Title 0x7188 / 0x7170 is filed too, but the movie window shows only
    // the body (FUN_004019a0(entry + 0x5c, entry + 0x50, 0)).
    let movie = |movie, body| MessageMovie {
        movie,
        caption: templates.format(body, &[]).unwrap_or_default(),
    };
    let rises = |bit: u32| word & bit != 0 && old & bit == 0;
    if rises(rebellion_core::victory::AD) {
        vec![movie(movies::HEADQUARTERS_DESTROYED, 0x7189)]
    } else if rises(rebellion_core::victory::CA) {
        vec![movie(movies::CORUSCANT_CAPTURED, 0x7171)]
    } else {
        Vec::new()
    }
}

/// The Planet Destroyed movie of each Destroy System run in `strikes`
/// that `viewer` is told of ([`recipients`]). `FUN_004950d0` stores movie
/// `0x6f` (MDATA.101) and its kind getter `FUN_004953a0` returns 6, so
/// `FUN_0048a060` files it on the movie list with no Message Index row and
/// no advice. The caption is the body: `0x70c9` with the holder's
/// adjective, or `0x70ca` for a neutral system (VM Yaga Minor: "The Death
/// Star has destroyed the Alliance system Yaga Minor."); title `0x70c8` is
/// filed too, but the movie window shows only the body.
pub fn strike_movies<'a>(
    world: &GameWorld,
    templates: &TextTemplates,
    strikes: impl IntoIterator<Item = &'a StrikeEffects>,
    viewer: Faction,
) -> Vec<MessageMovie> {
    strikes
        .into_iter()
        .filter_map(|strike| {
            let report = strike
                .report
                .as_ref()
                .filter(|report| report.planet_destroyed)?;
            let (system, bombarder) = (report.system?, report.bombarder?);
            let holder = report.holder.unwrap_or(Faction::Neutral);
            if !recipients(bombarder, holder).contains(&(viewer == Faction::Alliance)) {
                return None;
            }
            let place = TemplateArg::place(
                world
                    .systems
                    .get(system)
                    .map_or_else(|| "Unknown".to_owned(), |value| value.name.clone()),
            );
            let caption = match side_arg(holder) {
                Some(holder) => templates.format(0x70c9, &[Some(&holder), None, Some(&place)]),
                None => templates.format(0x70ca, &[None, Some(&place)]),
            };
            Some(MessageMovie {
                movie: movies::SYSTEM_DESTROYED,
                caption: caption.unwrap_or_default(),
            })
        })
        .collect()
}

/// The Death Star Sabotaged movie of each Death Star a mission's `effects`
/// destroy. Event `0x305` on an object of family `0x18`-`0x1b` in a system
/// raises `FUN_0048d470`: movie `0x72` (MDATA.104), kind 6
/// (`FUN_0048d640`), so no Message Index row and no advice; the caption is
/// body `0x71be` with the system's name. It tests no side, so both sides
/// file it whichever side sabotaged. hyp: every side's view holds every
/// system (`FUN_004f2d10(side, system)`).
pub fn sabotage_movies(
    world: &GameWorld,
    templates: &TextTemplates,
    effects: &[MissionEffect],
) -> Vec<MessageMovie> {
    effects
        .iter()
        .filter_map(|effect| match effect {
            MissionEffect::TargetSabotaged {
                death_star: true,
                system,
                ..
            } => {
                let place = TemplateArg::place(
                    world
                        .systems
                        .get(*system)
                        .map_or_else(|| "Unknown".to_owned(), |value| value.name.clone()),
                );
                Some(MessageMovie {
                    movie: movies::DEATH_STAR_SABOTAGED,
                    caption: templates
                        .format(0x71be, &[Some(&place)])
                        .unwrap_or_default(),
                })
            }
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::missions::MissionTarget;
    use rebellion_core::bombardment::StrikeReport;
    use rebellion_core::ids::{CapitalShipKey, SystemKey};
    use rebellion_core::manufacturing::ManufacturingState;
    use rebellion_core::world::{
        CapitalShipClass, Character, ControlKind, Fleet, ShipInstance, System,
    };
    use std::collections::HashMap;

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

    /// The shipped TEXTSTRA RCDATA entries the messages use.
    fn templates() -> TextTemplates {
        let entries: HashMap<u16, String> = [
            (0x7098, "Orbital bombardment of {1:4}"),
            (
                0x7099,
                "{1:4} ships have conducted an orbital strike on the {2:4} system of {3:1}",
            ),
            (
                0x709a,
                "{1:4} ships have conducted an orbital strike on the non-aligned system {2:1}",
            ),
            (0x70c8, "{2:1} Destroyed!"),
            (
                0x70c9,
                "The Death Star has destroyed the {1:4} system {3:1}.",
            ),
            (
                0x70ca,
                "The Death Star has destroyed the neutral system {2:1}.",
            ),
            (0x7164, "{1:1} Killed"),
            (0x7365, "{1:1} has been killed."),
            (0x7170, "Coruscant Captured"),
            (
                0x7171,
                "The Alliance has seized the Imperial capital system of Coruscant.",
            ),
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
            (0x71bd, "Death Star Sabotaged"),
            (
                0x71be,
                "The Rebel Alliance has sabotaged the Death Star at {1:1}.",
            ),
            (0x7188, "Alliance Headquarters Destroyed"),
            (
                0x7189,
                "The Empire has destroyed the Alliance Headquarters.",
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
            control: holder.map_or(ControlKind::Uncontrolled, ControlKind::Controlled),
            ..System::default()
        });
        (world, system)
    }

    fn armed(world: &mut GameWorld) -> CapitalShipKey {
        world.capital_ship_classes.insert(CapitalShipClass {
            family: 0x14,
            bombardment_modifier: 3,
            hull: 100,
            ..CapitalShipClass::default()
        })
    }

    fn fleet(
        world: &mut GameWorld,
        system: SystemKey,
        side: Faction,
        class: CapitalShipKey,
    ) -> FleetKey {
        let is_alliance = side == Faction::Alliance;
        let key = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![ShipInstance::new(class, 100, is_alliance)],
            is_alliance,
            ..Fleet::default()
        });
        world.systems[system].fleets.push(key);
        key
    }

    fn strike(
        system: SystemKey,
        bombarder: Faction,
        holder: Faction,
        set: impl FnOnce(&mut StrikeReport),
    ) -> StrikeEffects {
        let mut report = StrikeReport {
            system: Some(system),
            kind: Some(BombardKind::General),
            bombarder: Some(bombarder),
            holder: Some(holder),
            ..StrikeReport::default()
        };
        set(&mut report);
        StrikeEffects {
            report: Some(report),
            operation: Some(OperationReport::new(system, false, holder, bombarder)),
            ..StrikeEffects::default()
        }
    }

    const BOTH: [(Faction, Faction); 2] = [
        (Faction::Empire, Faction::Alliance),
        (Faction::Alliance, Faction::Empire),
    ];

    #[test]
    fn a_fleets_team_is_itself_and_a_fleet_icons_is_its_systems_fleets_of_the_side() {
        // FUN_0051d990 sets the selection as the team; an icon's system
        // resolves kind 0x10 to its fleets of the order's side (FUN_00512700).
        let (mut world, system) = world(Some(Faction::Alliance));
        let class = armed(&mut world);
        let imperial = fleet(&mut world, system, Faction::Empire, class);
        let rebel = fleet(&mut world, system, Faction::Alliance, class);
        let icon = Some(MenuObject::SystemIcon {
            system,
            quadrant: Quadrant::Fleets,
        });
        assert_eq!(
            menu_team(&world, Some(MenuObject::Fleet(rebel)), false),
            [rebel]
        );
        assert_eq!(menu_team(&world, icon, false), [imperial]);
        assert_eq!(menu_team(&world, icon, true), [rebel]);
        let defenses = Some(MenuObject::SystemIcon {
            system,
            quadrant: Quadrant::Defenses,
        });
        assert!(menu_team(&world, defenses, true).is_empty());
    }

    #[test]
    fn the_gates_enable_an_own_armed_fleets_three_orders_on_both_sides() {
        // FUN_00581e80 (VM Selonia: three rows enabled, Destroy System
        // disabled); another side's fleet is refused by its fleet command
        // (FUN_004ffa70, 1/1).
        let movement = MovementState::new();
        for (side, holder) in BOTH {
            let (mut world, system) = world(Some(holder));
            let class = armed(&mut world);
            let own = fleet(&mut world, system, side, class);
            let theirs = fleet(&mut world, system, holder, class);
            let is_alliance = side == Faction::Alliance;
            let selonia = [true, true, true, false];
            for selection in [
                Some(MenuObject::Fleet(own)),
                Some(MenuObject::SystemIcon {
                    system,
                    quadrant: Quadrant::Fleets,
                }),
            ] {
                assert_eq!(
                    bombard_gates(&world, &movement, selection, is_alliance).allowed,
                    selonia,
                    "{side:?} {selection:?}"
                );
            }
            assert_eq!(
                bombard_gates(
                    &world,
                    &movement,
                    Some(MenuObject::Fleet(theirs)),
                    is_alliance
                )
                .allowed,
                [false; 4],
                "{side:?}"
            );
        }
    }

    #[test]
    fn an_ordinary_strike_files_an_operation_report_for_the_bombarder_and_the_holder() {
        // FUN_00496770 on event 0x182 for each side viewing the fleet:
        // 0x7098 / 0x7099 on the Conflict rail; advice 0x2e for the side
        // that is not the bombarder (VM Selonia's wording).
        let templates = templates();
        for (bombarder, holder) in BOTH {
            let (world, system) = world(Some(holder));
            let messages = strike_messages(
                &world,
                &templates,
                7,
                &strike(system, bombarder, holder, |_| {}),
            );
            assert_eq!(messages.len(), 2, "{bombarder:?}");
            let (by, at) = if bombarder == Faction::Empire {
                ("Imperial", "Alliance")
            } else {
                ("Alliance", "Imperial")
            };
            for message in &messages {
                let display = message.display.as_ref().unwrap();
                assert_eq!(message.text, "Orbital bombardment of Selonia");
                assert_eq!(display.title, "Orbital bombardment of Selonia");
                assert_eq!(
                    display.body,
                    format!(
                        "{by} ships have conducted an orbital strike on the {at} system of Selonia"
                    )
                );
                assert_eq!(display.background, 0x2b9a);
                assert_eq!(message.rail, Some(MessageRail::Conflict));
                assert_eq!(message.target, Some(MessageTarget::System(system)));
            }
            let bombarder_alliance = bombarder == Faction::Alliance;
            assert_eq!(messages[0].audience, RailAudience::side(bombarder_alliance));
            assert_eq!(messages[0].advice, None);
            assert_eq!(
                messages[1].audience,
                RailAudience::side(!bombarder_alliance)
            );
            assert_eq!(messages[1].advice, Some(0x2e));
        }
    }

    #[test]
    fn a_report_message_carries_the_runs_lists_and_picture_and_alerts_only_the_owner() {
        // FUN_00496770 code 2: 0x2b9b once +0x80 holds an object; slot 4
        // (FUN_00497530) is 7 only for the fleet owner's copy, whose
        // filing opens the window (FUN_0048a060).
        use rebellion_core::ids::DatId;
        use rebellion_core::operation_report::ReportUnit;
        let templates = templates();
        for (bombarder, holder) in BOTH {
            let (world, system) = world(Some(holder));
            let mut strike = strike(system, bombarder, holder, |_| {});
            let report = strike.operation.as_mut().unwrap();
            report.system_destroyed.push(ReportUnit {
                dat_id: DatId::new(0x1000_0005),
                name: "Mon Calamari Regiment".into(),
                major: false,
                object: None,
            });
            let lists = report.clone();
            let messages = strike_messages(&world, &templates, 7, &strike);
            assert_eq!(messages.len(), 2);
            for message in &messages {
                assert_eq!(message.display.as_ref().unwrap().background, 0x2b9b);
                assert_eq!(message.report.as_deref(), Some(&lists));
            }
            assert!(messages[0].alerts(), "{bombarder:?}");
            assert!(!messages[1].alerts());
        }
    }

    #[test]
    fn an_assault_report_names_its_outcome_and_advises_the_other_side() {
        // FUN_00496770 code 2 for +0x64 = 1: title 0x7090; with a holder
        // (+0x68) 0x7091 when the attacker holds the system now, else
        // 0x7092; with none 0x7093, else 0x7094; picture 0x2b98 for side 1,
        // else 0x2b99; advice 0x2f.
        let templates = templates();
        for (attacker, defender) in BOTH {
            let (by, at) = if attacker == Faction::Empire {
                ("Imperial", "Alliance")
            } else {
                ("Alliance", "Imperial")
            };
            for (holder, now, body) in [
                (
                    defender,
                    Some(attacker),
                    format!("{by} troops have taken control of the {at} system Selonia"),
                ),
                (
                    defender,
                    Some(defender),
                    format!("{at} Troops have defended Selonia from an {by} assault."),
                ),
                (
                    Faction::Neutral,
                    Some(attacker),
                    format!("{by} troops have seized control of the neutral system Selonia."),
                ),
                (
                    Faction::Neutral,
                    None,
                    format!("The neutral system Selonia has repulsed an attack by {by} troops."),
                ),
            ] {
                let (world, system) = world(now);
                let report = Arc::new(OperationReport::new(system, true, holder, attacker));
                let messages = report_messages(&world, &templates, 7, &report);
                let display = messages[0].display.as_ref().unwrap();
                assert_eq!(display.title, "Assault on Selonia");
                assert_eq!(display.body, body);
                let picture = if attacker == Faction::Alliance {
                    0x2b98
                } else {
                    0x2b99
                };
                assert_eq!(display.background, picture);
                assert_eq!(messages[0].advice, None);
                assert!(messages[0].alerts());
                if holder == Faction::Neutral {
                    assert_eq!(messages.len(), 1);
                } else {
                    assert_eq!(messages[1].advice, Some(0x2f), "{attacker:?}");
                    assert!(!messages[1].alerts());
                }
            }
        }
    }

    #[test]
    fn a_strike_on_a_neutral_or_own_system_tells_only_the_bombarder() {
        // 0x709a for a system no side holds (+0x68 = 0); the holder is the
        // bombarder on its own system.
        let templates = templates();
        for side in [Faction::Empire, Faction::Alliance] {
            let (world, system) = world(None);
            let messages = strike_messages(
                &world,
                &templates,
                7,
                &strike(system, side, Faction::Neutral, |_| {}),
            );
            assert_eq!(messages.len(), 1);
            assert_eq!(
                messages[0].audience,
                RailAudience::side(side == Faction::Alliance)
            );
            let by = if side == Faction::Empire {
                "Imperial"
            } else {
                "Alliance"
            };
            assert_eq!(
                messages[0].display.as_ref().unwrap().body,
                format!(
                    "{by} ships have conducted an orbital strike on the non-aligned system Selonia"
                )
            );
            let own = strike_messages(&world, &templates, 7, &strike(system, side, side, |_| {}));
            assert_eq!(own.len(), 1);
        }
    }

    #[test]
    fn a_word_with_ad_plays_rebel_hq_destroyed_and_one_with_ca_coruscant_captured() {
        // Notification 0x128: FUN_0048e5a0 files 0x1f (movie 0x73) while the
        // word holds AD, FUN_0048f790 Coruscant Captured (movie 0x74) while
        // the Alliance holds Coruscant; both kind 6 (FUN_0048d640), the body
        // as the movie's caption (FUN_00422ce0, 0x482 wParam 1).
        use rebellion_core::victory::{AD, AW, CA, CD, EW, GO, LC};
        let templates = templates();
        assert!(word_movies(&templates, 0, LC | CD).is_empty());
        let hq = MessageMovie {
            movie: 0x73,
            caption: "The Empire has destroyed the Alliance Headquarters.".to_owned(),
        };
        let coruscant = MessageMovie {
            movie: 0x74,
            caption: "The Alliance has seized the Imperial capital system of Coruscant."
                .to_owned(),
        };
        // Both game types: AD alone (Standard, no win) and with EW
        // (Headquarters Only) file the same movie.
        assert_eq!(word_movies(&templates, 0, AD), [hq.clone()]);
        assert_eq!(word_movies(&templates, 0, AD | EW | GO), [hq.clone()]);
        assert_eq!(word_movies(&templates, 0, CA | AW | GO), [coruscant]);
        assert_eq!(word_movies(&templates, LC, AD | LC), [hq]);
    }

    #[test]
    fn a_word_files_one_movie_on_the_edge_where_ad_or_else_ca_turns_on() {
        // FUN_00488de0.c:134-142 maps the 0x128 group of (old, new) to kind
        // 0x1f when new has AD and old does not, else to 0x1e when new has
        // CA and old does not; nothing else files.
        use rebellion_core::victory::{AD, CA, LC};
        let templates = templates();
        let movies = |old, new| -> Vec<u8> {
            word_movies(&templates, old, new)
                .iter()
                .map(|movie| movie.movie)
                .collect()
        };
        assert_eq!(movies(0, AD | CA), [0x73]);
        assert_eq!(movies(AD, AD | CA), [0x74]);
        assert!(movies(AD, AD | LC).is_empty());
        assert!(movies(CA, CA | LC).is_empty());
        assert!(movies(CA | AD, CA | AD | LC).is_empty());
    }

    #[test]
    fn a_word_movie_without_its_text_has_no_caption() {
        // FUN_00421790 WM_CREATE makes the caption field only for a caption.
        let none = TextTemplates::default();
        let movies = word_movies(&none, 0, rebellion_core::victory::AD);
        assert_eq!(
            movies,
            [MessageMovie {
                movie: 0x73,
                caption: String::new(),
            }]
        );
    }

    #[test]
    fn a_strike_files_no_rebel_hq_destroyed_of_its_own() {
        // 0x1f comes from the win word's change (FUN_0048e5a0), not from
        // the run.
        let templates = templates();
        let (world, system) = world(Some(Faction::Alliance));
        let run = strike(system, Faction::Empire, Faction::Alliance, |report| {
            report.planet_destroyed = true;
            report.headquarters = true;
        });
        let messages = strike_messages(&world, &templates, 7, &run);
        assert!(messages.iter().all(|m| m.advice != Some(0x11)));
        // Planet Destroyed is kind 6 (FUN_004953a0): no row of its own.
        assert!(messages.is_empty());
        for viewer in [Faction::Alliance, Faction::Empire] {
            let movies: Vec<_> = strike_movies(&world, &templates, [&run], viewer)
                .into_iter()
                .map(|movie| movie.movie)
                .collect();
            assert_eq!(movies, [movies::SYSTEM_DESTROYED], "{viewer:?}");
        }
    }

    #[test]
    fn destroy_system_plays_the_system_destroyed_movie_to_both_sides_with_no_row_or_advice() {
        // FUN_004950d0 stores movie 0x6f and body 0x70c9 (VM Yaga Minor:
        // the movie played with "The Death Star has destroyed the Alliance
        // system Yaga Minor."); its kind getter FUN_004953a0 returns 6, so
        // FUN_0048a060 files it on the movie list (+0x28), with no Message
        // Index row and no advice. Each side whose view holds the fleet is
        // told (FUN_004f36a0).
        let templates = templates();
        for (bombarder, holder) in BOTH {
            let (world, system) = world(Some(holder));
            let run = strike(system, bombarder, holder, |report| {
                report.planet_destroyed = true;
            });
            let messages = strike_messages(&world, &templates, 7, &run);
            assert!(messages.is_empty(), "{bombarder:?}: {messages:?}");
            let at = if holder == Faction::Alliance {
                "Alliance"
            } else {
                "Imperial"
            };
            let played = MessageMovie {
                movie: 0x6f,
                caption: format!("The Death Star has destroyed the {at} system Selonia."),
            };
            for viewer in [bombarder, holder] {
                assert_eq!(
                    strike_movies(&world, &templates, [&run], viewer),
                    [played.clone()],
                    "{bombarder:?} bombards, {viewer:?} views"
                );
            }
        }
    }

    #[test]
    fn destroying_a_neutral_system_plays_its_movie_only_to_the_bombarder() {
        // FUN_004950d0 code 2: no holder side (+100) gives body 0x70ca; a
        // bystander's view of a neutral system holds no fleet (recipients).
        let templates = templates();
        for (bombarder, bystander) in BOTH {
            let (world, system) = world(None);
            let run = strike(system, bombarder, Faction::Neutral, |report| {
                report.planet_destroyed = true;
            });
            assert!(strike_messages(&world, &templates, 7, &run).is_empty());
            assert_eq!(
                strike_movies(&world, &templates, [&run], bombarder),
                [MessageMovie {
                    movie: 0x6f,
                    caption: "The Death Star has destroyed the neutral system Selonia.".to_owned(),
                }],
                "{bombarder:?}"
            );
            assert!(strike_movies(&world, &templates, [&run], bystander).is_empty());
        }
    }

    #[test]
    fn an_ordinary_strike_plays_no_movie() {
        // FUN_00496770 files the Operation Report; only FUN_004950d0's
        // Planet Destroyed stores a movie (+0x5c).
        let templates = templates();
        for (bombarder, holder) in BOTH {
            let (world, system) = world(Some(holder));
            let run = strike(system, bombarder, holder, |_| {});
            for viewer in [bombarder, holder] {
                assert!(strike_movies(&world, &templates, [&run], viewer).is_empty());
            }
            let idle = StrikeEffects::default();
            assert!(strike_movies(&world, &templates, [&idle], bombarder).is_empty());
        }
    }

    fn death_star_class(world: &mut GameWorld) -> CapitalShipKey {
        world.capital_ship_classes.insert(CapitalShipClass {
            family: 0x18,
            hull: 100,
            ..CapitalShipClass::default()
        })
    }

    #[test]
    fn a_sabotaged_death_star_plays_its_movie_to_both_sides_with_no_row_or_advice() {
        // FUN_0048d470 on event 0x305 for an object of family 0x18-0x1b:
        // body 0x71be with the system's name, movie 0x72 (MDATA.104); kind
        // 6 (FUN_0048d640), so FUN_0048a060 files it on the movie list with
        // no Message Index row and no advice. No side test: both sides,
        // whichever side ran the mission.
        let templates = templates();
        for holder in [Faction::Empire, Faction::Alliance] {
            let (mut world, system) = world(Some(holder));
            let class = death_star_class(&mut world);
            let fleet = fleet(&mut world, system, holder, class);
            rebellion_core::carriage::tag_all(&mut world);
            let effect = MissionEffect::TargetSabotaged {
                target: MissionTarget::CapitalShip(world.fleets[fleet].capital_ships[0].tag),
                system,
                death_star: true,
            };
            assert_eq!(
                sabotage_movies(&world, &templates, &[effect]),
                [MessageMovie {
                    movie: 0x72,
                    caption: "The Rebel Alliance has sabotaged the Death Star at Selonia."
                        .to_owned(),
                }],
                "{holder:?}"
            );
        }
    }

    #[test]
    fn sabotage_of_any_other_object_plays_no_movie() {
        // FUN_0048d470 tests the destroyed object's family against
        // 0x18-0x1b; a yard or regiment raises no movie.
        use rebellion_core::ids::ManufacturingFacilityKey;
        let templates = templates();
        let (world, system) = world(Some(Faction::Empire));
        let effects = [
            MissionEffect::TargetSabotaged {
                target: MissionTarget::ManufacturingFacility(ManufacturingFacilityKey::default()),
                system,
                death_star: false,
            },
            MissionEffect::SystemIntelligenceGathered {
                system,
                faction: rebellion_core::missions::MissionFaction::Alliance,
            },
        ];
        assert!(sabotage_movies(&world, &templates, &effects).is_empty());
    }

    #[test]
    fn a_report_without_its_textstra_entries_files_nothing() {
        // The Operation Report's text is TEXTSTRA 0x7090-0x7094 (assault)
        // and 0x7098-0x709a (bombardment) alone (FUN_00496770); without
        // them there is no original text to file.
        let empty = TextTemplates::default();
        for (owner, holder) in BOTH {
            let (world, system) = world(Some(holder));
            for assault in [false, true] {
                let report = Arc::new(rebellion_core::operation_report::OperationReport::new(
                    system, assault, holder, owner,
                ));
                let messages = report_messages(&world, &empty, 7, &report);
                assert!(messages.is_empty(), "{owner:?} {assault}: {messages:?}");
            }
        }
    }

    #[test]
    fn a_killed_character_without_its_textstra_entries_files_nothing() {
        // The Character Health message's text is TEXTSTRA 0x7164 / 0x7365
        // alone; without them there is no original text to file.
        let empty = TextTemplates::default();
        for alliance in [true, false] {
            let (mut world, system) = world(Some(Faction::Alliance));
            let character = world.characters.insert(Character {
                name: "Ackbar".into(),
                is_alliance: alliance,
                is_empire: !alliance,
                ..Character::default()
            });
            let messages = strike_messages(
                &world,
                &empty,
                7,
                &strike(system, Faction::Empire, Faction::Alliance, |report| {
                    report.planet_destroyed = true;
                    report.characters = vec![character];
                }),
            );
            assert!(messages.is_empty(), "{alliance}: {messages:?}");
        }
    }

    #[test]
    fn a_character_the_roll_kills_is_reported_to_its_own_side() {
        // Notification 0x1a, Character Health, for the character's side.
        let templates = templates();
        for alliance in [true, false] {
            let (mut world, system) = world(Some(Faction::Alliance));
            let character = world.characters.insert(Character {
                name: "Ackbar".into(),
                is_alliance: alliance,
                is_empire: !alliance,
                ..Character::default()
            });
            let messages = strike_messages(
                &world,
                &templates,
                7,
                &strike(system, Faction::Empire, Faction::Alliance, |report| {
                    report.planet_destroyed = true;
                    report.characters = vec![character];
                }),
            );
            let killed = &messages[0];
            assert_eq!(killed.text, "Ackbar has been killed.");
            assert_eq!(killed.display.as_ref().unwrap().title, "Ackbar Killed");
            assert_eq!(
                killed.display.as_ref().unwrap().body,
                "Ackbar has been killed."
            );
            assert_eq!(killed.audience, RailAudience::side(alliance));
            assert_eq!(killed.rail, Some(MessageRail::Mission));
            // FUN_0048fdf0.c:42, :121: wave 0x474, and the picture 0x42e
            // for side 1, else 0x433.
            let display = killed.display.as_ref().unwrap();
            assert_eq!(display.sound, 0x474);
            assert_eq!(display.background, if alliance { 0x42e } else { 0x433 });
        }
    }

    #[test]
    fn a_menu_row_posts_the_order_and_files_its_report_on_both_sides() {
        // The row's order runs at once on its team (FUN_005820a0 ->
        // message 0x224 in the same step), as the dev command's does.
        for (side, holder) in BOTH {
            let (mut world, system) = world(Some(holder));
            let class = armed(&mut world);
            let own = fleet(&mut world, system, side, class);
            let mut movement = MovementState::new();
            let mut log = MessageLog::new(16);
            let team = menu_team(
                &world,
                Some(MenuObject::Fleet(own)),
                side == Faction::Alliance,
            );
            let runs = issue(
                &mut world,
                &team,
                BombardKind::Military,
                &mut StrikeStates {
                    movement: &mut movement,
                    manufacturing: &mut ManufacturingState::default(),
                    transport: &mut rebellion_core::troop_transport::TroopTransportState::default(),
                    deliveries: &mut rebellion_core::delivery::DeliveryState::new(),
                    missions: &mut rebellion_core::missions::MissionState::new(),
                    uprisings: &rebellion_core::uprising::UprisingState::new(),
                    blockade: &mut rebellion_core::blockade::BlockadeState::new(),
                    tick: 7,
                    step: 0,
                },
                &mut || 0.5,
                &templates(),
                &mut log,
            )
            .unwrap();
            assert_eq!(runs.len(), 1, "{side:?}");
            let report = runs[0].report.as_ref().unwrap();
            assert_eq!(
                (report.bombarder, report.kind),
                (Some(side), Some(BombardKind::Military))
            );
            assert!(!world.fleets[own].bombarding, "finalize clears the flag");
            let rails: Vec<_> = log
                .messages()
                .iter()
                .map(|message| message.audience)
                .collect();
            assert_eq!(
                rails,
                [
                    RailAudience::side(side == Faction::Alliance),
                    RailAudience::side(holder == Faction::Alliance)
                ]
            );
        }
    }

    #[test]
    fn no_run_files_nothing() {
        let (world, _) = world(None);
        assert!(strike_messages(&world, &templates(), 7, &StrikeEffects::default()).is_empty());
    }

    #[test]
    #[ignore = "reads the original TEXTSTRA.DLL from data/base"]
    fn the_shipped_templates_render_the_vm_wording() {
        // VM Selonia and Yaga Minor (bombardment.md, death-star-strategic.md).
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/base");
        let templates = TextTemplates::load(&path).unwrap();
        let (world, system) = world(Some(Faction::Alliance));
        let run = strike_messages(
            &world,
            &templates,
            7,
            &strike(system, Faction::Empire, Faction::Alliance, |_| {}),
        );
        assert_eq!(
            run[0].display.as_ref().unwrap().body,
            "Imperial ships have conducted an orbital strike on the Alliance system of Selonia"
        );
        let kill = strike(system, Faction::Empire, Faction::Alliance, |report| {
            report.planet_destroyed = true;
            report.headquarters = true;
        });
        assert_eq!(
            strike_movies(&world, &templates, [&kill], Faction::Alliance)[0].caption,
            "The Death Star has destroyed the Alliance system Selonia."
        );
        let sabotage = MissionEffect::TargetSabotaged {
            target: MissionTarget::CapitalShip(1),
            system,
            death_star: true,
        };
        assert_eq!(
            sabotage_movies(&world, &templates, &[sabotage])[0].caption,
            "The Rebel Alliance has sabotaged the Death Star at Selonia."
        );
        // Every template id the strike messages name is in the DLL.
        assert!(run.iter().all(|message| message.display.is_some()));
    }
}
