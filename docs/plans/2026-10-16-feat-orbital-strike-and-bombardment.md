---
title: "Orbital Strike: Bombardment, Destroy System and the Support Shifts"
description: "Replace the invented bombardment formula, the Death Star's invented fire rules and its blanket cleanup with the original's orders 0x220-0x223, per-hit run, destroy cascade and support shifts, for both sides and the AI, issued from the original Planetary Bombardment menu"
type: feat
status: completed
progress: "100% by weight (234 of 234). Phases A-O and the added Phases P (products as objects) and Q (window restore and drag)"
created: 2026-10-16
updated: 2026-10-20
tags: [bombardment, death-star, support, ai, parity]
---

# Orbital Strike: Bombardment, Destroy System and the Support Shifts

Date: 2026-10-16. Approved by Tom the same day ("Start Phase A"), with the
report window left to UIP-B05 and the B and D panels retired here.

Tom, 2026-10-16: "You won't commit until port bombardment and orbital strike
are done." Everything below, plus the held C6 harness fix, lands in one large
commit after the docs and the review subagents, and only on Tom's go-ahead.

## Scope

| Node | State |
|---|---|
| `or-port-bombardment-8usw` (P1) | Closed 2026-10-17. Orders 0x220-0x223 resolve where they are given (`integrator::order_strike`, `strike_orders::issue`, `resolve_ai_strike`) through the gate (`bombardment::gate`, `FUN_00582440`), the per-hit run (`FUN_0058d800`) and finalize (`FUN_0058e320`). Native seed 42 on both sides; 24 headless seeds gave 95 Alliance and 226 Empire strikes |
| `or-port-destroy-system-cascade-imrv` (P1) | Closed. The cascade is in code with tests in `death_star.rs` (`both_sides_fleets_survive_the_destruction_with_everything_aboard`, `the_systems_own_completed_units_of_both_sides_die_with_it`, `characters_left_at_the_system_off_a_mission_go_adrift`, `special_forces_die_unless_en_route`), checked natively for the Empire at Vagran and at Hoth, and with an Alliance player at Yavin (seed 42: an Imperial Death Star fleet's Destroy System killed Yavin's regiments, squadrons and facilities, both sides' fleets kept everything aboard, six characters went adrift and the roll killed two, and the Alliance view reads Yavin destroyed, holder Neutral). The browser probes of the OBJ-14 rows pass (`orbital-strike.mjs`, `live-strike` and `death-star-strike`). The C6 tactical Death Star frames pass, 8 of 8 (Phase J item 12) |
| `or-q-port-support-shifts-y0y` (P1) | Closed 2026-10-17. Control change, blockade end and HQ removal post event 0x373 through `support_shift.rs`; bit 11 is `death_star::refresh_sectors` (`FUN_005178b0`), timer 0x386 and the kill finalize follow the original. Native seed 42 on both sides |
| `or-port-death-star-family-qtde` (P2) | Closed 2026-10-17. `CapitalShipClass::family` and `is_death_star` test 0x18-0x1b, and `GameWorld::fleet_has_death_star` asks a fleet's live ships (`FUN_005003d0`) |
| `or-bombard-command-timing-d1b2` (P3) | Closed 2026-10-17. Message 0x224 runs in the order's own scheduler step, and `FUN_0058d800` has one caller (`FUN_005637c0`), so no bombardment follows a battle |

Audit findings this moves: F-032 (the bombardment formula), F-017 (the
Death Star's fire preconditions), F-025 (Death Star identity), F-037 in part
(the shift hooks). Feature passes P26 (Bombardment) and P27's targeting,
firing and cleanup. Interface cells: OBJ-14-C011 to C013 and C016 to C018
(the menu rows); EXT-01-C005 and C006 (the invented dashboards, removed).

Specs: `ghidra/notes/bombardment.md`, `death-star-strategic.md`,
`support-shifts.md`, `game-end-path.md`, and `ai-order-emission.md` (the
Bombard and Destroy System postures). Manual pp. 57, 90, 105, 122, 124, 136.

## Rules for this plan

- Replace sourceless port inventions with the original mechanism. An
  untraced detail carries `hyp:`; a source-free port choice carries `port:`.
  The provenance baseline is empty and stays empty.
- Both factions in every step, gate and acceptance run. Either side can
  bombard; only the Empire can Destroy System, and that limit comes from the
  data (CAPSHPSD record 136 is the only family-0x18 class), never from code.
- Where the notes leave a question, settle it against the original before
  porting. VM evidence is recorded by save, day and values.
- Tests are behaviour sentences that cite their source, shown failing
  without the change. Before the commit, cargo-mutants on the touched files,
  each survivor closed or explained.
- Cargo only through `scripts/mini-cargo.sh`. One game instance on the Mini.
  Bump the save version freely, with no migrations. Regenerate the seed-42
  golden only for a named cause.
- Original UI only: the player bombards from the original menu.
- Notes state current facts. Docs fold into the feature commit.

## C6 harness fix

- **C6 harness fix (ff3099fd).** The tactical fixture's `ready` message carries every
  fixed control's rect (`tactical_control_layout` in `tactical_view.rs`), and
  the browser probes in `tools/interface-parity/run.mjs` click and check
  those rects by name. 160 of 160 browser executions pass. The
  A0 compare stands at pass 11, fail 66, not-run 28, settled 1, with every
  negative control behaving. TAC-05-C021 passes; TAC-05-C009 now runs and
  fails on pixels, a real difference the 106-cell train owns. The layout
  export's three mutants are caught.

## Phase A: settle the open questions

No port code. Ghidra reading on the existing decompiles (Sonnet agents at
medium effort, headless, local), then VM runs I drive myself. Each answer
goes into its note as a current fact.

1. **Conflicts between the notes**, read in the disassembly:
   - The character roll. `bombardment.md` rolls every character without
     +0x50 bit 5 and destroys only with bit 6; `death-star-strategic.md`
     rolls only minor characters (0x38-0x3b) not in transit and always
     destroys (`FUN_0058d990`, `FUN_0055e9e0`).
   - The gate. "Every team fleet" against "one passing fleet is enough",
     and whether "team" is the ordered fleet group or every fleet at the
     system (`FUN_00582440`, `FUN_004fd6b0`).
2. **The day and the bits.**
   - Message 0x224's place in the day (`FUN_005637c0` → `FUN_004fe020`,
     `FUN_004fec00`; `or-bombard-command-timing-d1b2`).
   - What sets and clears fleet +0x58 bit 6 and the system's bombard bit:
     after a run, after "two sides, nothing happens", when a flagged fleet
     moves, dies or a second order arrives the same day.
3. **Values and formulas.**
   - Bombard strength: the class field behind slot +0x1dc, and the target
     values behind +0x1c0/+0x1c4 (facilities, HQ) and +0x1d8/+0x1e4
     (regiments). Map each to the DAT field the loaders parse
     (`bombardment_modifier`, `bombardment_defense`) or record it `hyp:`.
   - Whether KDY overflow past the shields reaches the hull (`FUN_0058d540`,
     disassembly; export `FUN_00501ba0`/`FUN_00501bc0`, the only missing
     decompiles).
   - The leadership readers: +0x96==3 as the General, role 2
     (`FUN_004fd790`) as the Admiral.
   - System slots +0x1c4/+0x1c8 (`FUN_00563620`, `FUN_005820a0`), including
     a side bombarding its own system.
   - The manual's ion-cannon claim (p. 122: the XDY-150 "robs energy…
     preventing it from being able to bombard") against the gate.
4. **The cascade's edges.**
   - Adrift characters: what `FUN_004ef2e0` sets, the role from
     `FUN_00534790`, and what an adrift character does after.
   - The autoscrap request (+0x50 bit 0x1000, `FUN_004f78e0`): its consumer,
     and whether event 0x304 is it.
   - Missions at a destroyed system (mission end rules 2 and 4, mission
     0x241, regiment landings), builds due for delivery there, and fleets
     and regiments arriving after.
   - What clears a destroyed system's populated bit (`FUN_0050b760`,
     `FUN_0050a3c0`, `FUN_00508660`); +0x88 bit 3 and reason 0xe
     (`FUN_005081f0`).
   - The HQ in transit under Destroy System or capture (`FUN_00520b60`,
     `FUN_00526860`, `FUN_004f7640`; `or-hq-bombard-transit-4rh7`).
   - The refusal of a second family-0x18 ship at one system (`FUN_00550bd0`,
     `FUN_0054fd90`, `FUN_0040f340`).
5. **The shifts.** The control-change chain with its per-day counters and
   uprising records (`FUN_00517aa0`, `FUN_0055de50`, `FUN_00518040`, SDPRTB
   0x1420/0x1421, 0x1440-0x1445); the blockade-end formula and its loss
   counters A and B (`FUN_00517ee0`, `FUN_0055dbf0`, `FUN_00508c60`, cap
   0x1411); HQ removal on a hit, a kill and a capture (`FUN_00517cd0`,
   reason 3, GNPRTB 0x1407); the victory rerun on a Coruscant control
   change (`FUN_00517be0`).
6. **The AI's Destroy System need**: what sets fleet flag 0x10 and system
   record +0x110, and whether AI class flag 0x200000 admits record 136
   (`FUN_004739b0`, `FUN_00433780`, `FUN_0042e960`;
   `or-q-ai-death-star-use-z9ow`).
7. **VM runs** (`tools/original-capture/vm-game.sh`). Support read from the
   system window before and after; A0 frames of the menu rows, enabled and
   disabled, for OBJ-14:
   - Empire Target Military, Target Civilian and General Bombardment from
     `empire-day128-assault-fails-selonia.sav` or
     `empire-day53-yaga-orbit.sav`: kills by kind, energy and raw-material
     loss, support at the system, in the sector and elsewhere.
   - Empire Destroy System from `empire-huge-deathstar-built.sav`, flying
     Fleet 6 to a system holding fleets of both sides, minor and major
     characters, a character on a mission and a unit under construction.
   - An Alliance bombardment of an Imperial system. No save stages one;
     play one forward from an Alliance save and add it to the saves index.
   - A battle won with no order given: do the loser's facilities die?
   - A control change and a blockade's end: the sector shifts.
8. **Wrap-up.** Notes updated, including the stale Port sections of
   `bombardment.md` and `support-shifts.md`. Node notes appended. A manual
   cross-check entry for pp. 57, 90, 122, 124 and 136 (p. 57 says "enemy",
   p. 122 "enemy or neutral"; the code refuses only a destroyed system).

### Phase A results (2026-10-16)

Items 1-6 are read; the notes hold each answer with its `.c`. What changes
the steps below:

- **The day.** The order posts one flag command per fleet and one message
  0x224 per system; both run in the same scheduler step, before the next
  day (`FUN_005820a0`, `FUN_00520040`, `FUN_0051df30`). The VM agrees: the
  Selonia alert opened within a second of the order. The port resolves an
  order when it is issued, through the integrator and the simulation's roll
  stream.
- **No order, no bombardment.** `FUN_0058d800` has one caller, the 0x224
  handler, so the three post-battle bombardments go (B9).
- **The bits.** Fleet bit 6 is set per fleet by the 0x220 command, refused
  for a fleet with an uncompleted or en-route ship; finalize clears it on the
  bombarder's fleets at the system, and fleet arrival clears it. Two
  bombarding sides return before any phase and leave the bits set. A second
  order the same step finds no flagged fleet.
- **The gate.** One passing fleet of the ordered group is enough; inactive
  fleets are skipped. The enable slot also refuses mixed holders (`1/0x14`).
  Either side may bombard its own system.
- **Values.** Ship strength is CAPSHPSD `bombardment_modifier` less a hull
  damage penalty, zeroed by KDY overflow; a squadron's is FIGHTSD
  `bombardment_defense` scaled by losses; facility values are the facility
  DATs' `bombardment_defense` (KDY 5, LNR I 4, LNR II 3, GenCore I 2,
  GenCore II 3, Death Star shield 1); the HQ's is ALLFACSD `extra7` (9),
  which the loader must keep; a regiment's is TROOPSD `bombardment_defense`.
- **Defence.** KDY damage overflowing the shields never reaches the hull; it
  zeroes the ship's strength for the run. LNR damage reaches the hull. The
  General is `+0x96`==3 at the system, the Admiral `+0x96`==2 aboard.
- **Character roll.** Minor characters directly at the system, existing and
  not in transit, 50%, always destroyed on a hit.
- **The cascade.** Characters at the system and off mission go adrift with
  no message; a sweep on the system's battle-bit change (event 0x304 flag 0,
  also 0x307 and 0x30e) sends them on the hidden Adrift mission to the
  retreat chain's destination or a jail abode, so they may wait
  indefinitely. The autoscrap request is consumed by event 0x307:
  relocation through the retreat chain, else destruction with reason 2, no
  refund. Uncompleted and en-route units take it; en-route special forces
  are left alone. Uncompleted products follow their manager: the
  destroyed system's own managers destroy every product they queued
  (reason 0x16, no refund); of another manager's products bound for the
  system, a capital ship in the spare fleet is untouched, a special force
  is destroyed, and the rest take the request and keep their queue slot.
  The mission validator runs at once: a landed mission ends with code 7,
  the rest by rules 2, 4 and 7. Regiment landings and 0x241 teams are
  refused there; fleet moves are not. No move is cancelled, the
  blockade is untouched, and an HQ en route survives and relocates.
- **The AI fires the Death Star.** Empire module 11 requests the hull, fleet
  flag 0x10 marks it, and a Coruscant or HQ target (`+0x110`) lets the
  posture emitter issue 0x223.
- **The shifts.** Every shift posts event 0x373 and applies in the same
  step's phase 5, each clamped on its own, a no-op on an unpopulated system.
  Control change: 2 for a capture from the other side, 1 to or from
  neutral, over the sector, capped to the first change per old holder per
  scheduler step; at Coruscant a second shift of 15 (`0x1422`/`0x1423`)
  and a victory rerun. Blockade end: counters A, B and C fed by regiment
  kills by reason (0xe, 0xf, 0xc; 0x10 adds nothing). HQ removal on any
  cause: Alliance support −10 across the HQ's sector. Bit 3 gates sector
  shifts, and a rim system keeps it from setup until its holder changes.
- **Still open**: whether queue `+0xac` always serves its head, the system
  bombard bit's readers, the menu for own systems, counter C's readers, the
  HQ shift's sector on a capture, and event 0x373 against fleet events in
  one drain. None blocks Phase B; each becomes a `hyp:` where the port
  touches it.
- **VM (item 7), done.** Selonia (`empire-day128-assault-fails-selonia.sav`):
  General Bombardment against one shield generator killed nothing and moved
  no support. Yaga Minor (`empire-huge-deathstar-built.sav` played to day
  240, saves before and after each order): General Bombardment killed both
  Alliance regiments and four of five civilian facilities; Destroy System
  left both Imperial fleets and their contents in orbit, removed the last
  facility, set the holder to neutral, and moved every core system below its
  limit by 10 plus 1 per holder change in its sector (halved to 5, less 1, in
  the Death Star's sector) and no rim system. Both orders resolved at once.
- **Alliance bombardment of an Imperial system** (`alliance-day927-orto-orbit.sav`):
  Target Military at Orto opened its alert at once; the defence fire
  destroyed the Corvette and Orto lost nothing (`bombardment.md`, "In the
  original").
- **Carried to the next plan**: the original's blockade end and the cascade's characters, on
  Wine or the VM (the save reader does not list characters yet). The Drall save's Fleet 1 was locked by a pending
  intercept battle, not by a bombardment rule.

## Phase B: the order, the one resolver and the per-hit run

The player path (`main.rs` `OrderBombardment`, `FireDeathStar`, resolved on
the click) and the AI's (`integrator::resolve_ai_strike`, resolved in the AI
round) become one order type and one resolver in the integrator, run inside
the tick so the per-hit rolls come from the simulation's roll stream.

1. **Death Star identity** (`or-port-death-star-family-qtde`).
   `CapitalShipClass::is_death_star` tests families 0x18-0x1b; the 0x34 test
   goes. `Fleet::has_death_star` becomes a query over the fleet's ships, with
   every reader moved: `refresh_sectors` (bit 11), `death_star_fleet`,
   `reconcile_death_star_result`, the AI's `fleet::DEATH_STAR` flag.
2. **The order and its slot in the day.** A `BombardOrder` carries one of
   the four kinds 0x220-0x223 and their flags (`FUN_0058d800`). Issuing it
   sets the fleet's bombard bit and a pending order (saved state, save bump).
   Every pending order, player's and AI's, resolves at the slot Phase A
   found for message 0x224, drawing from `take_rolls`; that slot is named in
   `agent_docs/simulation.md`. Bits clear as Phase A found.
3. **The gate** (`FUN_00582440`) as Phase A settled it; 0x223 also needs a
   family-0x18 ship. The only target refusal is a destroyed system. Remove
   `DeathStarSystem::fire`'s Empire-held refusal, the self-bombardment guard,
   and the test that encodes the refusal (the VM run is the evidence). The
   one-per-system Death Star refusal is ported if Phase A finds it in the
   build path, else carried.
4. **Setup and targets** (`FUN_0058cba0`). The bombarder is the owner shared
   by every bombard-flagged fleet; two such sides means no run. Targets are
   the holder's regiments, military facilities and HQ (military), civilian
   facilities (civilian), and the system's energy and raw material while
   nonzero.
5. **Ship phase** (`FUN_0058d990`). With Destroy System and a family-0x18
   ship present: the system's destruction first (Phase C), then the roll of
   every minor character still directly at the system and not in transit at
   GNPRTB 0x606, a hit always destroying it, so an adrift minor character is
   rolled too.
6. **Defence phase** (`FUN_0058dc10`). Each KDY-150, then each LNR, fires
   once at a random bombarding ship: DEFFACSD attack × (the General's
   leadership / GNPRTB 0x600 + 1), integer division, shields first, the hull
   as Phase A read it. It runs even after a kill.
7. **Effect phase** (`FUN_0058df60`). Hits = Σ fleets (bombard strength ×
   (the Admiral's leadership / GNPRTB 0x601 + 1)) minus the holder's
   family-0x24 shield generators' DEFFACSD defence; the Death Star shield
   0x25 does not count, and hits ≤ 0 skip the loop. Collateral for 0x220.
   Each hit picks a random target and succeeds with chance (10 − v)/10
   (GNPRTB 0x602, 0x603; energy 0x604, raw material 0x605); a success
   destroys the target (cause 0x10 at the bombarder's own system, else 0xf)
   or lowers energy or raw material by one. Regiment kills feed the blockade
   loss counters (`FUN_00508c60`). The run's +0x28 and +0x2c come from what
   died.
8. **Finalize** with the real run: GNPRTB 7705 at the system, then the
   galaxy shift after a kill or the sector shift after civilian, energy or
   raw-material loss. An HQ destroyed by a hit clears `is_headquarters` and
   raises the HQ removal (Phase D).
9. **The post-battle bombardments go** (`tactical_flow::apply_automatic_bombardment`,
   and in `simulation.rs` the space-win follow-up and the unopposed-HQ
   invasion): the original bombards only on the order, and the AI's posture
   emitter issues the strike. Their tests
   (`decisive_imperial_bombardment_…`, `unopposed_imperial_invasion_…`)
   follow the settled behaviour; the bombardment part of
   `or-port-headless-battles-tdl` unblocks.
10. **Delete the invention**, now that nothing calls it:
    `BombardmentSystem::resolve_bombardment`, `BombardmentResult`, the stat
    helpers and their four formula tests; `victory.rs`
    `apply_headquarters_bombardment` and its two tests.
11. **The AI honours its strike kind.** `AIAction::Bombard`'s `Strike`
    (General 0x222, Military 0x220) and `DestroySystem` (0x223) post the
    order; the resolver takes the side from the fleets, never a default.
12. **Effects.** The run flows through the integrator's ordered effects
    (`GameEffect::BombardmentApplied`, `PopularityShifted`), or the unused
    variants go.
13. **Dev commands** for native acceptance: `Bombard: <fleet> <kind>` and
    `Destroy system: <fleet>`, posting the same order the menu posts;
    documented in `agent_docs/dev-commands.md`.
14. **Tests**: the gate on both sides; two bombarding sides; the target
    set; the hit count against the formula; the roll chance; collateral;
    the defence damage per KDY and LNR; finalize's flags; the Empire
    bombarding its own system (cause 0x10, the shift's direction); two
    orders at one system in a day; a flagged fleet that leaves or dies
    before the run; save and load with an order pending.
15. **E2E** (dev commands, native, both sides): an Empire and an Alliance
    bombardment of each kind, matching the deterministic quantities exactly
    (the target set, defence damage, each shift's delta given the kills) and
    the hit count against the formula; the VM numbers checked the same way.
    Headless playtest, seed named in the evidence: at least one
    `EVT_BOMBARDMENT` per faction issued by the AI.

### Phase B as built

- **B1, narrowed.** `CapitalShipClass` carries its DAT `family`;
  `is_death_star` tests 0x18-0x1b; `Give Death Star` adds a real ship of that
  class; the gate and the run read ships. Superseded by Phase G item 1:
  `Fleet::has_death_star` is a query of the fleet's ships, with no stored
  flag.
- **B2, no pending order.** Message 0x224 runs in the step the order is
  given (Phase A), so the order resolves where it is issued: the player's
  through `sim_rng`, the AI's inside `ai_step` on its roll stream, both via
  `integrator::order_strike`. Saved state is `Fleet::bombarding` (v54), which
  outlives a run only when two sides are flagged at one system; arrival
  clears it (`FUN_004ff600`).
- **B12.** The strike writes the world through the integrator's
  `resolve_strike`, not new `GameEffect` variants; `BombardmentApplied`, which
  nothing emitted, is gone.
- **B13.** The commands are `Bombard military|civilian|general: <fleet>` and
  `Destroy system: <fleet>` (`agent_docs/dev-commands.md`).
- **E2, done here.** Retiring the panels fell out of B9-B10: `panels/bombardment.rs`,
  `panels/death_star.rs`, their `PanelAction` variants, the B and D keys and
  `MoveDeathStar` are gone.
- **B15, native (seed 42, day 0, `Blockade` to stage, `Show system` before
  and after).** Empire at Vagran (one KDY, one LNR, 3 regiments): General
  with one Star Destroyer (shields 300, hull 2750) took defence 2800, 0 hits
  (the KDY's overflow zeroed its strength) and hull 2750 → 1950 (LNR 800).
  With the Death Star added, Military made 17 hits (2 + 15; the KDY spent
  itself on the Death Star's 2000 shields), killed 3 regiments and the KDY,
  and left support alone. Civilian killed 9 facilities and moved Imperial
  support 31 → 11 (GNPRTB 7705); General killed a facility, energy 12 → 10
  and raw material 12 → 11, and clamped support at 0. Alliance at Denab
  (one KDY): Military made 3 of 4 hits (one ship zeroed, hulls untouched),
  killed the KDY and, by collateral, a production facility (support
  35 → 15); Civilian and General followed the same rules. The bombarder
  gets no Message Index row: the "Orbital bombardment" line has no rail and
  no TEXTSTRA source (Phase E).
- **B15, headless (24 seeds, 1500 days, both sides AI).** Alliance 95 and
  Empire 226 AI strikes; seed 1 alone has 10 and 13. Every one is 0x220:
  `FUN_004ae320` picks General only where the bombarder holds no more of the
  sector than the enemy, and the port matches it line for line. No run met
  a battery, since none is seeded (the seed tables hold only the HQ's shield
  generator).
- **Carried to Phase D.** The destruction cause (0x10 at the bombarder's own
  system, else 0xf) and the blockade loss counters: `StrikeReport.regiments`
  holds each killed regiment and its side for them.

## Phase C: the destroy cascade

`cleanup_destroyed_system` becomes the original cascade (`FUN_005080d0` →
`FUN_004f8660` → `FUN_004f71d0`; hook +0x11c, event 0x319).

1. **Fleets survive**, every side, with their contents (their regiments and
   fighters included), the Death Star's own fleet too (`FUN_00524fb0`).
   Bit 11 refreshes with the surviving Death Star.
2. **The system's own units**, not fleet contents: completed regiments,
   ships, fighter squadrons, HQ and facilities not in transit die
   (`FUN_004f8710`); uncompleted ones follow their manager: the system's own
   managers destroy their queue (reason 0x16, no refund), and of another
   manager's products bound for the system a capital ship in the spare
   fleet is untouched, a special force is destroyed, and the rest take the
   autoscrap request, consumed as Phase A found, keeping their queue slot.
   The blanket clear of the manufacturing queue and of
   `death_star.under_construction` goes.
3. **Special forces** die unless in transit; off-mission ones get autoscrap
   (`FUN_00503e20`). One a mission return finds no destination for is
   destroyed with reason 9 (`FUN_00549580` → `FUN_00503eb0`).
4. **Characters directly at the system**, not on a mission, go adrift
   (`FUN_004ef2e0`). A new adrift state if the port lacks one (save bump),
   with its original message and its display in the character and personnel
   windows, and what follows as Phase A found.
5. **Missions, deliveries and arrivals** at a destroyed system follow
   Phase A: mission end rules 2 and 4, mission 0x241 and landing refusals,
   builds due there, fleets arriving after.
6. **What stays.** Ownership, population and resources are not written.
   Fleets may still move to the system and stay there: `movement.rs`'s
   `DestinationDestroyed` refusal goes. The blockade is not lifted by the
   kill; the blockade-end rule keeps the blockading side. Movement orders
   into the system are not cancelled.
7. **Victory**: the bit clears, hook +0x128 raises event 0x300, and
   `FUN_005180e0` sets AD (the HQ gone) or CD (Coruscant destroyed).
8. **Dead code goes**: the `PlanetDestroyed` arm in `simulation.rs` step 11,
   the `unreachable!` in `main.rs`, and the integrator arm that never fires.
9. **Tests**: each child kind; a fleet's regiments and fighters surviving;
   both sides' fleets at the target; an adrift character surviving a save
   round trip; a fleet moving to the destroyed system; Destroy System on
   Coruscant (CD), on the Alliance HQ (AD) and on the last system; a second
   Destroy System on a destroyed target refused.
10. **E2E** (dev commands, native): Destroy System on a system holding
    fleets of both sides, minor and major characters, a character on a
    mission and a unit under construction, compared with the VM run; once
    as the Empire player and once from an Alliance game against the
    Imperial Death Star (`Give Death Star:` and the AI's order, or a staged
    save per `original-save-editing.md` Recipe 2).

### Phase C as built

- `cleanup_destroyed_system` is the cascade and returns a `CascadeReport`
  (what died, the HQ, the adrift characters, the autoscrapped objects).
  Special forces are never autoscrapped (`FUN_00503e20` takes argument 1).
  Adrift is `Character::adrift`; `sweep_adrift` starts the hidden trip as a
  Move (`port:`). Superseded by Phase G item 3: the sweep runs on each
  change of the battle bit and on the kill, with the jail abodes.
- **Products.** The destroyed system's own managers destroy their queue
  with reason 0x16, no refund (`FUN_0052b7d0` → `FUN_0052a2c0`,
  `CascadeReport::products`). Another manager's product bound for the
  system: a capital ship in the spare fleet is untouched (`FUN_00524fb0`),
  a special force is destroyed (`FUN_00503e20`), and squadrons, regiments
  and facilities take the autoscrap request with the producer's system as
  the hint and keep their queue slot (`FUN_00528890`).
- **Mission returns.** A member's `+0x70` is the container it joined from
  (`ActiveMission::set_out_from`, `FUN_00536490`), a fleet or a system;
  return policy 1 sends it back there when that passes the Move test, else
  down the retreat chain (`FUN_00542ee0`). A special force with no
  destination is destroyed with reason 9 (`FUN_00549580` → slot `+0x20c`,
  `FUN_00503eb0`); it is never jailed.
- The unfinished Death Star, and every other capital ship product, is an
  object in the destination's spare fleet from its order day (`+0x50` bit 6,
  `death-star-product.md` §1; Phase P). Open questions:
  `or-q-unfinished-ship-owner-on-capture-y2nz`,
  `or-q-second-death-star-order-same-day-y2cw`,
  `or-q-carrier-ship-lost-en-route-hpxe`.
- **C6, corrected by the trace.** The populated bit clears (`FUN_0050b760`
  → `FUN_0050a3c0(0)`), so the system turns neutral (hook `+0x20c`,
  `FUN_0050b5a0`; the VM's Yaga Minor reads neutral). The Kill branch runs
  that control change before the cascade: the HQ's removal with reason 3
  and the change's shift (the VM's "took 1 back"). Support and resources
  are not written.
- The cascade's own units feed blockade counter B with the strike's reason
  (`FUN_0058ce70`, `FUN_00508c60`).
- **C10, native (seed 42).** Empire at Vagran with an Alliance fleet in
  orbit and two Alliance minor characters on the surface: both fleets
  survived intact; 3 regiments and 12 facilities died; both characters went
  adrift and the 0x606 roll killed both; "Vagran Destroyed!" was filed for
  both sides and the deaths for the Alliance; a second Destroy System was
  refused (`Destroyed`). At Hoth, the Alliance HQ: the system reads neutral
  and destroyed, the HQ is gone, the rim sector Churba did not shift, core
  systems took the kill's +10. A Standard game continues (AD also needs the
  leaders' capture); Headquarters Only is covered by tests. The sector
  window draws a destroyed system as STRATEGY 10240, the asteroid field,
  keyed, with no status bars (`FUN_0045bbb0.c:88-95`), as the VM's Yaga
  Minor shows. The galaxy dot and System window of a destroyed system, the
  adrift display and the jail abodes are Phase G items 2 and 3.

## Phase D: the remaining support shifts

1. **Control change** (`FUN_00517aa0`, `FUN_0055de50`) with its per-day
   counters and uprising records, hooked at every control-change site:
   `integrator.rs` `ControlResolved` and `apply_ground_occupation`, native
   `ControlResolved`, `tactical_flow.rs` `apply_system_occupation`. A
   Coruscant control change reruns the victory check (`FUN_00517be0`).
2. **Blockade end** (`FUN_00517ee0`, `FUN_0055dbf0`, the weighted formula
   over loss counters A and B, cap 0x1411), at both `BlockadeEnded` sites.
3. **HQ removal** (`FUN_00517cd0`, GNPRTB 0x1407) on a hit (Phase B), a kill
   (Phase C) and a capture (reason 3, at the control-change sites).
4. **One setter.** Every shift goes through `uprising::apply_support_change`
   (`FUN_0050c9f0`, with bit 11's halving), which takes over the unpopulated
   no-op its callers check today. Whether a shift applies at once or through
   event 0x373 follows Phase A; record the choice.
5. **Tests** per shift and per hook site.
6. **E2E** (native, both sides): a captured system, a lifted blockade and a
   destroyed HQ, sector support read before and after, each delta matching
   the formula and the VM.

### Phase D as built

- `support_shift.rs` holds the control change (with the per-step cap,
  keyed by day: `port:`), Coruscant's own shift, the blockade end from
  counters A, B and C, and the HQ's removal; every shift is posted with its
  cause, gated at the post, and fired at the end of its operation from the
  event queue (event `0x373`, priority `0x100`, behind every holder event at
  `0x10`, and in posting order within a priority); GNPRTB 7705 stays
  immediate.
  `apply_support_change` holds the unpopulated no-op.
- Bit 3 is `GameWorld::shift_excluded`; the system family comes from
  SYSTEMSD (`GameWorld::system_family`). The Phase B sector shift read a
  family byte real ids lack, so no real system shifted; it now posts
  through the shared gate.
- Telemetry: each fired shift is a `support_shift` record with its cause;
  `bombardment` records carry the bombarder's `support_change`;
  `blockade_ended` carries the blockader, each side's loss total and
  strength sum, and the record they make (null when none is made); a
  Death Star kill writes its `control_changed` record (`planet_destroyed`).
  The native game writes each blockade record as its event is applied.
- **Troop bits.** The control word's bits 4-5 (`FUN_0050b610`) are
  `GameWorld::troop_held` (save v54), rebuilt by
  `support_shift::refresh_troop_bits` each time control reruns
  (`rebuild_control`) and at seeding (`refresh_all_troop_bits`): usable
  regiments of one side set them, none clear them, both keep them.
  `control_changed` takes the old and new bits and picks the record (`FUN_0055de50`): neither,
  `0x1420` per side under the cap; old only, `0x1442` (10); new only,
  `0x1440` (0); both, `0x1444` (0). The cap applies only when neither has
  them. Destroy System's kill posts its control change only when the
  system held no regiments of its side at the populated clear.
- **Stalemates.** While both sides' regiments stand, the system's troop
  bits and holder stay the earlier troop side's (`FUN_0050b610.c:34-70`);
  with no earlier troop side, support decides (`:64-70`). The port's
  `System::control` keeps the earlier holder the same way, so the
  stalemate's end changes holder from it (no shift when the earlier side
  keeps the system; its loss record and the HQ's removal when it goes
  neutral) and a blockade's end reads it.
- **Bit 11 at once.** `Give Death Star` refreshes the sector marks
  (`death_star::refresh_sectors`), so a shift the same day is halved.
- **D6, native (seed 42).** Empire: Vagran blockaded and bombarded, 15
  Alliance losses, strength 69/98, the fleet moved off: `blockade_ended`
  then −1 on all 10 Corellian systems ((0 − 71·15/100) / 10); Corellia
  destroyed with 3 Empire regiments standing: the Bombardment shift,
  +10 on 29 core systems, and no control-change record; neutrals the +10
  tipped joined the next day (116 `ControlChange` records), the Corellian
  ones halved to 0 by bit 11. Alliance: Corellia blockaded and its 3 regiments
  bombarded away (15 Empire losses, strength 84/83): the control change
  with old troop bits gave +10 (`0x1442`) over the sector, and the
  blockade ending at that change gave +2 ((2·99·15/100 − 0) / 10); a
  lift with no losses (Denab) shifts nothing.
- Counter A is fed by a planetary assault's regiment kills, attackers and
  defenders alike (`assault::resolve`, reason `0xe`; Phase K item 5).
  Open: no traced reader of counter C. `hyp:` the designers' meaning of
  SDPRTB `0x1440`-`0x1445`. Control reruns on a support change
  (`FUN_005109f0`) and a regiment change (`FUN_00508660`), a killed
  system's dying regiments included, are Phase O items 3 and 6.

## Phase E: the original Planetary Bombardment menu

1. Enable the object menu's Planetary Bombardment row 0x120 and its four
   targets 0x220-0x223 (`object_menu.rs`), each enabled through the Phase B
   gate (`FUN_00581e80` → `FUN_00553770`), Destroy System always listed
   (`FUN_0040f340`) and disabled on a destroyed system. A row posts the
   Phase B order.
2. Retire the invented egui panels (`panels/bombardment.rs`,
   `panels/death_star.rs`, their `PanelAction` variants, the B and D keys).
   `MoveDeathStar` goes with them; the ordinary fleet move covers the Death
   Star's fleet (checked natively).
3. The result reaches the player through the original messages
   (`strike_messages`: 0x15, 0x1a, 0x1f), each to the side Phase A found
   (bombarder, victim or both); event 0x319 shows the wreck to both sides.
4. **E2E**: native, both sides, one strike of each kind issued by
   `Right click` through the menu; browser check of the rows, enabled and
   disabled, on both sides against the Phase A frames for OBJ-14.

### Phase E as built

- The menu's enable slot is `bombardment::order_enabled` (`FUN_00581e80`:
  `FUN_0051fe20`'s fleet check, the empty team 1/0x16, then the gate). A
  fleet entry passes that fleet; a sector fleet icon passes the side's
  fleets at its system (`FUN_0051d990`, `FUN_00512700`). A destroyed
  system disables all four rows (the 0x224 command refuses 1/0x22).
- Every submenu parent is enabled when a child is (`FUN_00442380` →
  `FUN_00442c90`), the Command parent included.
- A row and `Bombard …:` share `strike_orders::issue`. An ordinary run
  files Operation Report 0xf (TEXTSTRA 0x7098/0x7099, picture 0x2b9a) on
  the Conflict rail for the bombarder and the holder, advice 0x2e to the
  side that did not bombard; Planet Destroyed 0x15 and Rebel HQ Destroyed
  0x1f carry their TEXTSTRA wording.
- **E4, native (seed 42).** Empire at Vagran: the fleet icon's Planetary
  Bombardment opened three enabled rows and Destroy System disabled;
  General Bombardment from the menu left the same hull (2750 → 1950) as
  the dev command, and the Conflict row read "Imperial ships have conducted
  an orbital strike on the Alliance system of Vagran"; with the Death Star
  added, Destroy System enabled. Alliance at Denab: Target Military from
  the menu killed the KDY and filed "Alliance ships have conducted an
  orbital strike on the Imperial system of Denab"; Hoth's fleet, strength
  0, showed the parent disabled. Browser, both sides
  (`tools/interface-parity/orbital-strike.mjs`): `live-strike` lists the
  parent enabled with three rows enabled and Destroy System grey, a click
  on the grey row does nothing, and General Bombardment opens its report;
  `death-star-strike` enables all four rows with the Death Star aboard,
  and after Destroy System the destroyed system greys the parent. The
  OBJ-14 cells still wait on the A0 compare against the Phase A frames.

## Phase F: docs, reviews, commit

1. **Telemetry.** `EVT_BOMBARDMENT` carries the order kind, hits, kills by
   kind, defence damage and the support change, and fires on a run with no
   kills; a Destroy System event lists the surviving fleets, adrift
   characters and autoscrapped units; a support shift carries its cause.
   `telemetry_coverage` stays green.
2. **Golden.** `trace-determinism.sh` first, then one regeneration of the
   seed-42 replay golden named "the bombardment per-hit run and the destroy
   cascade (the roll stream's draws move)", plus any save-version cause.
3. **Docs.** The notes' Port sections, `agent_docs/simulation.md`,
   `agent_docs/dev-commands.md`, the audit ledger (F-017, F-025, F-032,
   F-037, P26, P27), `known-deviations.md` (DEV-UI-009 for the removed
   dashboards), the nodes closed or narrowed, `scripts/task-index.py`.
4. **Reviews.** Code-review and silent-failure subagents over the diff,
   every finding fixed.
5. **Gates.** `scripts/gates.sh --keep-going > LOG 2>&1; echo $?` with every
   PASS/FAIL line read; scoped cargo-mutants on the touched files;
   `invariants.py`.
6. **Commit** on Tom's go-ahead (ff3099fd carried Phases A-G and the C6
   harness fix; the rest follows in the next commit).

## Phase G: the node remainders to parity

Tom, 2026-10-17: the two open nodes go to parity before the commit.

1. **Death Star identity, done.** `Fleet::has_death_star` asks the fleet's
   live ships for family `0x18`-`0x1b` (`FUN_005003d0`); the stored field
   leaves the save, a Death Star destroyed in battle kills its ship
   (`tactical::apply`), and `bombardment` reads the same query.
   `or-port-death-star-family-qtde` closed.
2. **Destroyed-system display, settled in the original** (VM, the Yaga Minor
   saves before and after Destroy System, days 240-315). The sector window
   draws the planet as a dust field with no status bars; the System window is
   identical, pixel for pixel, before and after; the galaxy marker follows
   `FUN_0042b330`'s destroyed exit: the white cross (STRATEGY 10158) under
   Popular Support, the fleets' own marker under Idle Fleets. The port has the
   asteroid field and no destroyed branch in the System window;
   `gid_marker_for_system` now takes the exit by mode (the cross except in
   modes `0x20`-`0x22`, `0x43`, `0x44`, `0x92`, `0x93`; modes `0x22` and
   `0x44` never take it, for an unexplored system either). The colour of
   the fleet and personnel modes by the counted units' side is
   `or-port-gid-unit-mode-colour-xgrr`.
3. **Adrift sweep and jail abodes, ported.** A Destroy System sweeps its
   adrift characters the day of the kill, after the character roll (event
   `0x30e`, `death_star::sweep_adrift` from `integrator::resolve_strike`); a
   member with no place of its side goes through `FUN_00549580` (Phase H), a
   captives-only group with a place is freed and leaves (`FUN_00549850`), a
   captor-held character stays flagged, and a jailed leader counts for the
   Standard win (`FUN_005180e0`). Each change of a system's battle bit
   (`+0x88` `0x1000`, `FUN_0050a900`; `BlockadeState::battle`) sweeps it as
   event `0x304` does (`FUN_00548e90`), the same in the native and headless
   steps; any placement or change of captivity takes a character out of its
   jail (`Character::leave_abode`, read through `held_abode`).
   `FUN_0050b8e0` sets the battle bit when both sides are present at a
   system of side 1, 2 or 3. A neutral system is side 3 (`FUN_0055a080`):
   fleets of both sides there set the battle bit
   (`BlockadeSystem::battle_bit`, `blockade.rs:848`;
   `both_sides_fleets_at_a_neutral_system_set_the_battle_bit`), and a
   populated one is blockaded by any one side's fleet, an unpopulated one
   never (`system_is_blockaded`, `blockade.rs:865`;
   `either_sides_fleets_alone_blockade_a_populated_neutral_system`,
   `an_unpopulated_neutral_system_is_never_blockaded`).
4. **Removed with no source:** the egui system info side panel (uncalled since
   2026-09-11, no fog of war) and the uncalled file-list seeding path.
5. **Final reviews, fixed.** A stalemated system is bombarded as its earlier
   holder's (`FUN_0050b610.c:45-70` keeps that holder, `FUN_0058cba0` reads
   it); every KDY and LNR fires whatever its side
   (`FUN_00527550`, `FUN_005272e0`); Target Military rolls its collateral
   even with no military target, and the collateral takes the first civilian
   facility (`FUN_0058df60`, `FUN_0058d080`); the control-change cap is
   saved; the dev Bombard runs the menu's enable test; strike and occupation
   telemetry write the cause before its shifts. A Death Star committed to its
   side's retreat leaves even when no hyperdrive of the side works
   (`FUN_005d14d0.c:124-127`; `BattleSession::retreat_side` commits
   `BattleSession::death_star`),
   and every placement leaves a jail abode.
6. **Then:** the golden for the named causes, the C6 browser probes over the
   tactical fixture's Death Star frames, mutants, gates, the native checks,
   and the AI's 0x223 record.

## Acceptance

- Every deterministic quantity of a run matches the original exactly: the
  gate, the target set, defence damage, each support delta given the kills,
  the cascade's survivors and casualties by kind. Random quantities (which
  targets die) match the formula's distribution, not the VM's draw.
- Each of the five nodes is closed, or narrowed to a named remainder in its
  notes: `or-port-bombardment-8usw`, `or-port-destroy-system-cascade-imrv`,
  `or-q-port-support-shifts-y0y`, `or-port-death-star-family-qtde`,
  `or-bombard-command-timing-d1b2`. The phases also cite
  `or-hq-bombard-transit-4rh7` (Phase A item 4),
  `or-q-ai-death-star-use-z9ow` (Phase A item 6, Phase I item 8),
  `or-f-death-star-rdq` (Phase J item 12), `or-q-ground-resolution-nms`
  (Phase K) with `or-c-gnd-alliance-qqg` (its item 4), and
  `or-q-port-victory-rules-ce9` (Phase L) with
  `or-port-movie-triggers-45gd` (its item 5).
- Both sides: a player bombardment of each kind, AI bombardments in a named
  playtest seed, and Destroy System from an Empire game and against the
  Alliance. The AI issues 0x223 once it holds the hull, in the seed tests
  on both sides; no 24-seed soak builds the hull, the budget equilibrium
  the original's request code shares (`ai-death-star.md` section 3; Phase
  I item 8).
- Phases H-N each meet their own checks, on both sides.
- No invented panel, formula, refusal or post-battle bombardment remains
  without a source.

## Decisions

1. **The bombardment report window (EVT-03)** stays with the interface train
   (UIP-B05). This plan ships the original messages. (Tom, 2026-10-16;
   superseded by Decision 5 for Phases H-N, which moves the window here.)
2. **The B and D panels retire in this commit**, with `MoveDeathStar`, once
   the menu works. (Tom, 2026-10-16.)
3. **The support writes that bypass the setter** (`EconomyEvent::SupportDrifted`,
   `EventAction::ShiftPopularity`) and the per-tick support recompute
   (F-037's timer chain): out of scope as first written (Tom, 2026-10-16),
   folded in as Phase O (Tom, 2026-10-17: "seek full parity").

## Folded in (Tom, 2026-10-17)

Tom, 2026-10-17: "can we fold these into our current plan and recalculate
its completion rate?" The jail remainders and the five areas this plan had
carried out of scope join it, researched the same day against the notes,
the decompiles on disk and the code. Sizes: S, M, L. Each phase lists what
the port has that no source supports; each goes, or gets its source.

### Phase H: the jail remainders

Traced in `ghidra/notes/jail.md`. A prisoner carries its captor's side bits
(`FUN_004f1a80` → `FUN_004eec60`), so the jail of a stranded Alliance
character is the Empire's.

1. **Trace, done.** The abode's slot `+0xc` returns its own key; the trip
   is 1 day (`FUN_00555b30` writes 1 tick between keys not both systems).
2. **The trip and the side flip, done.** `death_star::strand` ports
   `FUN_00549580`: a free member is captured (captor's side bits) and the
   retreat chain runs again on that side, so it leaves as a prisoner for the
   other side's system; with none, it goes to the jail of its side bits,
   the other side's. A prisoner groups on its captor's side bits. `port:`
   the member is held at once with its en route bit clear for the 1-day
   trip, which no reader of an abode occupant sees.
3. **The finder row, done.** Name and abode alone (the family-`0x98` branch
   at `0x00465fed`); the Characters view lists a character on its native
   side's tab (`FUN_004637a0` stores `FUN_004edc80`). The VM settles the
   display in item 8.
4. **Trace** the second way in, done: the order (`jail.md`, section 4), the
   return policies and the hidden Return mission
   (`ghidra/notes/mission-return.md`).
5. **Port** that path, both sides, done: the mission-end return
   (`FUN_00548120`, `death_star::return_members`). M, after 4.
6. **Escape, done.** Timer `0x38a` (GNPRTB 2584 + rand(0..=2585) days,
   repeating) fires the ESCAPETB roll on espionage + combat − the guards'
   average combat − the captor's regiments (`FUN_00561990`); a success frees
   the prisoner off a mission and its autoscrap request sweeps its location
   at once (`death_star::free_escapee`). The invented daily loyalty roll is
   gone. hyp: a jailed escapee is swept from Yavin or Coruscant.
7. **Rescue targeting, done.** Legal for the native side, but no player
   window and no AI planner state reaches a jail occupant; the port's
   windows and `evaluate_rescue` (a captive at a system) match.
8. **VM, done** (`save_tree.py jail`, `jail.md`):
   the finder rows "Emperor Palpatine - Alliance Jail" and "Leia Organa -
   Empire Jail" carry no state; no system window lists a jailed captive, so
   a Rescue has no target; Leia escapes within about 50 days ("Leia Organa
   escaped from the Empire at Empire Jail.") and arrives at Yavin. The
   1-day trip rests on the code read (timer `0x387`): a natural jailing
   needs both sides without an accepting system (`FUN_005571c0`, `jail.md`
   section 4), a state no small save edit reaches. L.
9. **The final battle, done in the notes.** The three characters move
   together by Move (`FUN_0054b040` → `FUN_00547480`); no abode
   (`jail.md` section 7, `story-missions.md`, `story-encounters.md`).
   `or-q-final-battle-jail-xkv9` is closed.
10. **Native,** both sides: the finder row and the Standard win with a
    jailed leader, through a dev command that stages one. S, after 2 and 3.
    Native, release build, seed 42: the Characters finder lists "Luke
    Skywalker - Empire Jail" (Empire) and "Emperor Palpatine - Alliance
    Jail" (Alliance). Empire Standard win: Luke jailed by `Jail:`, Hoth
    destroyed from the fleet menu with Mon Mothma on a mission there, her
    stranded team captured her, EW set, ending slot 84 and movie `0x6d`,
    then the main menu. Alliance Standard win, seed 3 (seed 42's Vader
    runs Recruitment back to back): Vader and Palpatine jailed by `Jail:`
    on days 4 and 16; Coruscant's garrison removed by `Strip defenders at`
    (the holder stayed the Empire); two Lelmra regiments boarded a Medium
    Transport through the regiment Move order, flew to Coruscant, and the
    fleet menu's Planetary Assault took it ("Alliance troops have taken
    control of the Imperial system Coruscant"); AW set on the next day.
11. **Escape messages, done.** Notification `0x1e0` (`FUN_00490340`) files
    to every side with a view: "{name} Escaped" `0x715c`/`0x715d` to the
    native side, `0x715e`/`0x715f` to the other, waves `0x405`/`0x469`,
    advice by major (`0x15` for an own minor, `0x24` for another). S.

### Phase I: the Death Star as a product (F-017, F-033, F-034)

The original has no construction system and no Death Star timer: the
station is the shipyard product CAPSHPSD record 136 (cost 584, maintenance
600), built by the ordinary path (`FUN_0052e580`, `FUN_0052a430`,
`FUN_0052b960`, already ported). Timer `0x386` is the blockade's shift. The
dual-AI batch (seeds 1-24, 1,500 days) built no Death Star on either side,
so no 0x223 was issued; 234 bombardments ran (Empire 0x220 143 and 0x222
12, Alliance 0x220 79).

1. **Remove the invented construction, done:** the 1,825-day countdown, the
   300-unit nearby warning, their two messages, `EVT_DS_STATUS` and
   `EVT_DS_CONSTRUCTION`, `DeathStarState::death_star_fleet` and
   `shield_generator_active`, `VictoryState::death_star_active` and
   `death_star_location`, the telemetry fixture's use of them and
   `a_completed_death_star_is_recorded_with_its_system`; bump the save. S.
2. **The one-per-system cap, done** (`FUN_00507ab0`, `FUN_0040f340`, refusal
   `{0x90,3}`) for Build, Move and the retreat test, both sides. M.
3. **Placement, done** (`Fleet::product_day`): trace where a finished capital ship goes (`FUN_0052bee0`,
   the order-time container; the VM showed a new Fleet 6 at Coruscant) and
   replace `apply_build_completion_inner`'s first-own-fleet rule. S, then a
   golden cause (every capital ship build moves).
4. **The player's maintenance refusal `0xf3`, done** (`FUN_0052e970`), shared with
   F-035. S.
5. **Object targeting in the mission dialog, done,** for Sabotage and Death Star
   Sabotage (today the dispatch sends no target, so both are refused). M,
   after `or-port-mission-creation-check-f4h2`.
6. **The AI's sabotage planner, done:** the regional machine
   (`FUN_004bda60`: target system by `sys+0xbc`, one object draw, staging
   nearness, `(stack + 2) / 2` decoys; `ai-sabotage-target.md`). Traced `FUN_00593500`, `FUN_00482330`,
   `FUN_00482560`; port its Death Star Sabotage and replace the
   facility-count heuristic (`or-3m5f`). M.
7. **A mission target names a ship or a product, done** (an uncompleted
   Death Star is no target, `FUN_00593500`), so an uncompleted Death
   Star can be sabotaged. S-M, after 6.
8. **The AI's hull, settled.** The module-11 budget (`FUN_004ca5e0`) is read:
   the hull needs capacity 2223. No seed issues 0x223
   (`or-q-ai-death-star-use-z9ow`). Measured 2026-10-17 on the release
   playtest, dual AI, standard galaxy and medium difficulty (the playtest has
   no flag for either), seeds 1-24: no Death Star build, no `DestroySystem`
   order and no `death_star_fired` in 1,500 days or in 5,000 days, and no
   game ended by victory. Capacity is not the gate: the Empire's reaches
   2,223 by day 1,500 in 17 seeds (median 2,725 at day 1,500, highest 6,100
   by day 5,000), with 600 spare as well in 15. The gate is the fielded
   capital-ship maintenance `c+0x24c`: the module's budget
   (`Census::budget`, `0.27 × capacity − c+0x24c`) has median 35 at day
   1,500, 15 at 3,000 and 13 at 5,000 (fielded maintenance median 636, 833,
   800), because module 11 spends its own share on ordinary ships. Of 240
   samples (days 500-5,000 by 500) three reach 600: seed 22 day 2,000 (730),
   seed 7 day 2,500 (640), seed 4 day 3,000 (607); the class is listed in
   all 240. Around seed 22's day 2,000 module 11 builds capital-ship keys 17
   and 18 (CAPSHPSD `0x81`, `0x82`) instead, so an earlier row of
   `FUN_004ca5e0` holds the turn: the troop-room row comes first in the
   fixed request order (`FUN_004ca5e0.c:57-139`), costs little, so its
   blocker never sets (`FUN_004ca910.c:26`), and the hull row, eighth, is
   never reached (`ai-death-star.md` section 3). The port's request code
   matches the original's. The same 1,500-day batch ran 320
   bombardments: Empire 0x220 162 and 0x222 36, Alliance 0x220 122. S, then
   M if a fix follows.
   The 0x223 seed tests, both sides: a Death Star fleet at an enemy
   headquarters (`sys+0x110 >= 1`) issues `DestroySystem` with a regiment
   aboard (assault need, `FUN_004b6510`) and with none against a garrison
   (bombardment need, `FUN_004b6870`), through a whole kind-6 posture pass;
   without the hull the same passes assault and bombard
   (`ai_regions.rs`). The pass test found one port fault, fixed: an order
   for the record's last fleet ends the emitter's pass (`FUN_004b1070`
   `*param_1 = 1`, `FUN_004adb70` case `0xe`), where the port re-issued
   the same assault or Destroy System on each call while the need stood.
   The Alliance never issues 0x223: it has no hull (record 136 is
   Empire-only; module 11's hull row is tested Empire-only).
9. **Checks:** VM, a one-yard build timed by day, Death Star Sabotage
   against a completed and an uncompleted station, the second-hull refusal;
   native, both sides, build, refusal and sabotage. M.
   Native, Empire, seed 42: at the starting capacity (330 of 350 used) a
   Death Star order is refused `0xf3` (advisor slot 19); after
   `Maintenance capacity: empire +1300` the order queues at Corellia on day
   6 (584 refined, maintenance 600 charged at the order), a second order
   there is refused `{0x90,3}` (slot 18), and the station completes on day
   487 into a new fleet at Corellia, "Construction complete at Corellia".
   Native, Alliance, seed 42, a completed station in Fleet 2 at Coruscant
   (6 regiments, 10 squadrons), teams sent from Yavin by `Mission:`: Han
   Solo with decoys Chewbacca and Jan Dodonna reached the timer phase and
   destroyed the station (movie, "The Rebel Alliance has sabotaged the
   Death Star at Coruscant."); Wedge Antilles with decoy Leia, arriving
   after it, filed "Death Star Sabotage Mission Failed"; Han alone was
   detected on arrival ("Han Solo Mission Foiled" to the Alliance, "Enemy
   Mission Foiled" to the Empire) and went home; Wedge with three decoys
   was detected too, Jan Dodonna captured. Native, Alliance, seed 42,
   `Queue build: empire Death Star at Corellia` (a staging command; Build
   Selection's own checks and enqueue): the station queues (0/584); a
   second order there is refused `DeathStarRoom` `{0x90,3}`; Coruscant,
   which has no shipyard, refuses the order. The Create Mission dialog for
   a system target lists Espionage and Incite Uprising only, on both sides
   (Luke on Corellia with the station in its queue; Palpatine on Hoth),
   and `Mission:` reports Death Star Sabotage not offered at Corellia: the
   queued station is no object the order can name, so the `{0x40,0x34}`
   refusal of an uncompleted target (`FUN_00593500.c:49-58`) is not
   reached natively.
   VM, Empire, the route-1 game (`agent_docs/original-save-editing.md`),
   one Orbital Shipyard at Coruscant: the Death Star order's progress read
   2 on day 123, 7 on day 144 and 23 on day 206, one unit every 4 days
   (MANFACSD `processing_rate` 4, `FUN_0052cc90`), so 584 units take about
   2336 days on one yard; `stockpiles.rs`
   `one_shipyard_adds_a_unit_of_work_every_four_days`. On day 253, with
   that order building and 12 maintenance left, Coruscant's Build
   Selection lists "Death Star" in grey (Carrack in white); every class
   reads "n/a" and Confirm does nothing, so the second-hull refusal is not
   separated from the maintenance shortfall (the grey entry's trace:
   `manufacturing-build-selection.md`). VM, Alliance,
   `alliance-day731-imperial-deathstar-built.sav`'s source game with the
   converted order set back to progress 10 (the Imperial station
   uncompleted at Coruscant, day 748): Han Solo's Create Mission on
   Coruscant lists Incite Uprising and Espionage only. That is the target
   family rule (`FUN_00593500` refuses a target outside families
   `[0x10,0x40)` with `{0x40,0x27}`, so neither Sabotage is offered for a
   system), not the product; an uncompleted object target is refused
   `{0x40,0x34}` (`FUN_00593500.c:49-58`, `death-star-product.md` section
   4). VM, Alliance, `alliance-day830-after-coruscant-battle.sav`
   (`abe0dc2d…`), the completed station Devastator in Fleet 325 at
   Coruscant: Han Solo's Mission order from Yavin's Personnel tab, with
   the Devastator as target, opens Create Mission on Death Star Sabotage,
   and the list holds that one entry (no ordinary Sabotage:
   `FUN_0056a110` refuses families `[0x18,0x1c)` with `{0x40,0x28}`). Han
   alone, sent on day 781 (save `84db1a82…`), was detected: "Han Solo
   Mission Foiled", "My Death Star Sabotage mission to Coruscant has been
   foiled by opposing forces. Personnel are returning to Yavin."; on day
   828 Han is at Yavin and the station is intact in Fleet 325.

### Phase J: the tactical Death Star and the trench run (TAC-05)

The port: the station starts at hull 100 (the laser constant) and charge 0;
its ship also fights as an ordinary capital ship; the trench run launches on
the order and rolls one pass; the 3D view draws object 3001; the browser
probe asserts hull 100 and charge 0.

1. **Trace, done:** the producer of the trench run's state 2 → 5
   (`FUN_005caf20`), `FUN_005e9cc0`'s keying, damage slot `0xc0`,
   `FUN_005ba890`, the sender of message 0x45, the charge reset, `+0x70` in
   `FUN_005ada10`; correct the roles of `FUN_005afe40` and `FUN_005c0b60`
   (hangar docking) and `+0x74` (not completed) in
   `tactical-death-star-path.md`. S-M.
2. **Trace, done,** the AI's order 6 and laser use and the headless path
   (`FUN_005c9200`, `FUN_0059a530`). M.
3. **Trace and port, done,** the commander slot `+0x950` from the fleet's
   commander. S, after 1.
4. **Seed the station from its record, done** (`FUN_005af4e0`, `FUN_0040bcd0` →
   `FUN_00595ef0`): the remaining strategic hull, charge 100, `+0x74` not
   completed, `+0x70` the Death Star Shield (family `0x25`); the probe's
   expectations follow. S, after 1.
5. **The station leaves `ships`, done** (`FUN_0040b0e0` exports `[0x18,0x1c)` only
   as its short record): no turbolasers, no target, no strength; delete the
   unreachable fighter hull-chip branches; recheck
   `death_star_ship_index`, retreat, withdrawal and the write-back that
   kills the ship. M, after 4.
6. **The approach, done:** state 2 then the launch (`FUN_005d0820`,
   `FUN_005cfe80(2)`, the 61 s the VM showed), participants off the field;
   drop the 3001 sprite (TAC-03-C020; `or-p-trench-run-timing-e71`). M.
7. **The roll, done:** `FUN_005d04e0`'s retry loop over up to 1001 iterations,
   `__ftol` truncation with no clamp, the state-5 snapshot restored on
   success (`FUN_005d08d0`), state 8, one result per group. Confirm the loop
   in the disassembly first. M, after 1 and 6.
8. **Strength and retreat, done:** the Alliance's order-6 strength term
   (`FUN_005d0ce0.c:86-103`); `+0x74` in the retreat test (`FUN_005d0f50`);
   `death_star_departs` sourced or removed. S.
9. **The hangar, done** (every carrier, `hangar_tests.rs`): Imperial squadrons docking in the station and leaving
   with it (`FUN_005afe40`, `FUN_005c0b60`). M, after 5. With it, T3 and T4:
   a hangar arms from step 3 (`FUN_005b8570`, `FUN_005a7500.c:551`), and a
   docked squadron takes no slot and no share of the battle extent
   (`FUN_005ab650.c:148-153`), both sides.
10. **The AI's Death Star actions, done,** as item 2 finds them: the
    windowed AI (`FUN_005c8780`), its shot voices (`FUN_005ba520`), and the
    headless Death Star (`FUN_005d14d0.c:20-60`, `FUN_005aefc0`) on the
    headless battle generator (`FUN_005af420`). M.
11. **An uncompleted Death Star in battle, done** (`FUN_0040d760`, no decompile on
    disk yet) and the shield flag from Phase I. M, after 4.
12. **Acceptance:** A1 fixtures from both saves (`or-f-death-star-rdq`); the
    C6 A0 compare passes TAC-05-C009 and C017-C023; a native trench run on
    both sides, with the hangars' launches (T3, T4); a VM trench run to its
    outcome on the Alliance save and an Empire-side run. M.
    Run: the Alliance save's trench run (Red Group, Attack Death Star from
    the Missions panel) plays its movie about 3 minutes after the order and
    ends with the station gone from the 3D view; zoomed out, the station
    decal shows off one face of the battle box and the planet off the other
    (`tactical-death-star.md`). Empire side, `empire-day660-deathstar-pre-alert.sav`
    (Battle at Coruscant, day 676, Take Command): the same decal and planet
    show, and the Alliance AI made no run in 5 minutes of battle. With
    the TIE squadrons emptied (`empire-day660-trench-tieless.sav`) the
    Alliance AI runs the trench on its own and the Battle Summary lists
    the Death Star destroyed (`tactical-death-star.md`).
    Native, release build, seed 42, a Death Star given to a fleet staged by
    `Battle at`. Alliance at Yavin: Red Group's Attack Death Star from the
    Missions panel launches the run (`TrenchRunStarted`); film 201 plays
    about 2 minutes after the unpause, the station decal is removed
    (`trench_run_success`), and the summary lists Death Star 1 destroyed
    and ISD 1 operational. Empire at Hoth (6 Alliance squadrons): the
    Alliance AI orders the run on its own, film 201 at about 130 s, the
    Death Star destroyed on the first run, and the summary lists ISD 1,
    Death Star 1 and the TIE squadrons destroyed. On both sides the
    Imperial squadrons enter the scene only after the unpause. With the
    `[tactical_hangar]` lines (seed 42, Battle at Hoth for the Empire and
    at Yavin for the Alliance): the ISD's 6 TIE squadrons seat docked, none
    live (`docked=6 live_docked=0`, T4); its hangar arms at Take Command for
    step 15 and step 14 (3 + 5 + rand(0,8), T3) and launches a squadron at
    that step, then re-arms 5 to 13 steps on (steps 20 and 24). The
    Alliance squadrons all have hyperdrives and seat in flight
    (`docked=0`); the station's hangar was not exercised.
    Done. The C6 compare passes TAC-05-C009 and C017-C023 (8 of 8) on the
    browser battle run with the traced tactical camera (`FUN_005d9640`),
    the 3D window (17, 30, 439 by 437) and the holocube, with the capital
    meshes at the cache's 3.0 scale (`FUN_005d5b10`) and 0.3-unit fighter
    decals (`FUN_005d43c0`). TAC-05-C019 (empire-death-star-pre-alert, day
    676, superlaser strike) passes on position: displacement 0, MAE
    0.0023, changed pixels 0.0046. Its SSIM (0.952) and edge IoU (0.7185)
    wait on `or-d3drm-raster-match-vog4`: Direct3D RM's 8-bit palette
    matching, line stepping and rasterizer, measured as 977 ship, 736 star
    and 412 holocube edge misses of 7,549. Browser battle gate 180/180 on
    both sides at both viewports; gid 150/152 as before.
13. **The Battle Summary, done** (`tactical-results.md`;
    window kind 8, `FUN_0046f140`, the Operation Report's art family). The
    port showed an invented forces page: text categories, a Death Star
    tab, Operational, Damaged and Destroyed on every tab, every character
    Operational, regiments lost by the fleet's room. Now each side's view
    has "Filters" and the four icon tabs (`0x65`-`0x68`), Operational and
    Destroyed (Personnel: Survivors, Captured, Killed), the empty texts
    "None", "No Survivors" and "No Casualties", the list's pictures with the
    burn marks of damage, its bar and Left/Right. The lists come from the
    Battle Alert's record (`rebellion_core::battle_summary`): the Death
    Star on Capital Ships, regiments dying with their carrying ship,
    captives Captured. A lost ship's characters go adrift and take its
    wound roll, a minor dying on G2577 (`FUN_00502840`); the battle no
    longer takes the loser's officers captive. Burn marks follow the
    damaged bit (`FUN_00501c00`, `FUN_00503340`). The live craft count is
    `max(1, trunc(hull / 8))` (`FUN_005b8e00`). M-L. Native, build ac90a418, seed 42, release:
    - Empire, a Death Star given to the fleet staged by `Battle at Hoth`,
      Take Command, the Alliance AI's trench run: the station is destroyed
      at about 80 s, film 201 plays, the Empire wins with Imperial Star
      Destroyer 1 Operational with burn marks, picture `0x2a08`.
    - Empire Withdraw with no station: a 30 s countdown, the Star Destroyer
      jumps, picture `0x2a06`, "The Imperial fleet has withdrawn".
    - Empire Withdraw at Yavin with the station: a 150 s countdown, picture
      `0x2a05` after the Alliance's own withdrawal.
    - The Alliance AI withdrawing on morale: picture `0x2a05`.
    - Alliance, `Battle at Yavin`: picture `0x2a07`, the Imperial Star
      Destroyer destroyed.
    - Alliance Withdraw: a 34.4 s countdown, picture `0x2a05`.
    - Personnel reads Survivors, Captured and Killed.
    Browser, production-battle-results-presentation and yaga-day30-options,
    both sides at both viewports: 4 of 4 each. At 640×480 the frame,
    summary, close and tab probes read source-bitmap-exact; the summary
    pictures are `0x2a07` (Alliance win, Fwillsving), `0x2a08` (Empire
    win, Norval II) and `0x2a05` (Empire at Yaga Minor, "The Alliance fleet
    has withdrawn."); Capital Ships list a Mon Calamari Cruiser 1/2
    Operational with burn marks and 3 Destroyed; Personnel reads None,
    None, No Casualties on both forces views. The fixture sets every
    condition column on both sides at once, so a winner's summary text sits
    beside the loser's live ships; native runs cover the real outcomes.
    No browser fixture yet opens Results for `0x2a06`, after a trench run,
    with "No Survivors", or with the Left/Right filter keys
    (`or-acc-results-dlw`).

### Phase K: planetary assault (`or-q-ground-resolution-nms`)

The port: the Planetary Assault row (`0x234`) is always disabled; fleets
land their regiments by themselves (`simulation.rs:636-651`, kept by Tom
2026-10-04 until sourced); the ground battle (`combat.rs` `resolve_ground`,
the 256-round loop, `regiment_strength`, GNPRTB 0x1400 misread) and the live
ground screen (`ground_combat.rs`, the tactical continuation) have no source;
an occupation captures every enemy character, with no source.

The original (`FUN_00563390` → `FUN_0058c1e0` → `FUN_0058c120`): count and
flag (`+0x88` `0x80`, hook `+0x228`); the defence facilities fire
(`FUN_0055ead0` on class `+0x5c`, a hit kills a random attacker with reason
`0xe`); each attacking regiment fights one random defender once
(`FUN_0055eb00`: rand(0..G[0x202]) + attack + the fleet General's
leadership/G[0x201] − defence − the system General's/G[0x201]; ≤ G[0x203]
the attacker dies, ≥ G[0x204] the defender); collateral rounds
(`FUN_0055eb80`, `FUN_0055ebc0`: a facility, 1 energy or 1 raw); landing
when attackers remain and no defender does (`FUN_00559fe0(side, 0)`);
cleanup (`FUN_004fe4d0`); control by the troop rule already ported.

1. **Trace, done** (`ghidra/notes/ground-assault.md`): `FUN_0055ead0`, `FUN_0055eb80`, the functors at `0058bc90`,
   `bd00`, `be70`, `bdb0` and `FUN_0058c0d0`; write
   `ghidra/notes/ground-assault.md` (`or-q-regiment-combat-eaxn`). M.
2. **Trace, done,** the `0x234` row's enable test (the manual's two shields), its
   factory, fleet bit 7's setter and notification `+0x228`. S-M, after 1.
3. **Trace and port, done** (`ground-assault.md`, section 4): a holder
   change's owner hook walks the system's children (`FUN_00510820` ->
   `FUN_004fae40`, slot `+0xb0`): a prisoner whose captor's side differs is
   released, a character still of another side takes the autoscrap
   request (`FUN_004ef260`), whose event `0x307` sweeps it to the nearest
   system of its side unless the battle bit is set
   (`support_shift::move_enemies`, `death_star::sweep_requests`, after each
   operation); a special force turns to the new side when its class serves
   it, else is removed (`FUN_00503d90`). M.
4. **VM:** an Alliance assault (`or-c-gnd-alliance-qqg`), a battery killing
   a regiment, the two-shield greying, an enemy fleet present. M, after 1-2.
   Run: the Alliance takes Ord Pardon, the Empire takes Drall after an
   intercept and is repulsed at Selonia (`ground-assault.md`, "In the
   original"); an Alliance fleet in orbit of Orto (three shields) sees
   Planetary Assault greyed and the bombardment rows enabled. A KDY-150
   kills an attacking regiment at Tralus (taken) and at Corellia (the sole
   attacker dies, "Alliance Troops have defended Corellia"); an enemy fleet
   present blocks the order (the blockade gate). Imperial Fleet 1 takes
   Alliance-held Talus on day 272 (`0x7091`, the Personnel tab lists no
   survivor, capture or kill); the three Alliance officers there on day 220
   are at Alliance-held Corfai on day 277.
5. **Port, done** (`assault::resolve`, save v55): `assault::resolve` in core, the kills through the destroy path
   with reason `0xe` (feeding loss counter A), the landing, the control pass;
   remove `resolve_ground`, `regiment_strength`, the round loop and the
   tactical ground continuation; bump the save. L, after 1-2.
6. **The menu row and the AI, done** (`assault_orders.rs`,
   `resolve_ai_assault`), through the one resolver. M, after 5.
7. **Automatic landing and the occupation capture, gone** (item 3 sources
   neither), **re-measured.** Dual-AI batch, seeds 1-24, 1,500 days, no
   victory: 11 AI Assault orders (Alliance 9, Empire 2), all applied; each of
   the 11 runs landed (48 and 8 regiments) and took its system; attackers
   lost 2 regiments, defenders 4. Control changed 11 times by assault, 410 by
   the economy's troop and support rule, never by a destroyed planet. The
   Drop need (`FUN_004b1440`) issued 157 landings (Alliance 44, Empire 113)
   and every one was refused `troops/OtherSideDestination`: `can_drop` aims
   at a neutral unpopulated system and `TroopTransportState::accept_planet`
   refused a system its side does not hold. Fixed: `FUN_005073d0` lifts the
   side refusal for a regiment at a side-3 unpopulated planet
   (`regiment-unload.md`), so the Drop lands; the need staying set while
   `FUN_004ae440` passes is the original's (`ai-order-emission.md`). The
   batch re-measure after the fix, 24 seeds over 1,500 days: 13 Drops
   issued, 12 applied, none refused (1,343 issued and 1,337 refused
   before). Module 8 also
   applied 25 regiment moves to a system that were not Drop orders (Alliance
   9, Empire 16). The same seeds and days before the change: 362 regiments
   landed automatically, 54 control changes by occupation (Alliance 33,
   Empire 21), 446 by the economy, no assault, 8 module-8 moves to a system.
   M.
8. **Messages, done,** `ground_combat.rs` removed; the Assault Summary is
   Phase M's window, filled from the run (`fill_assaulted_system`). M.
9. **Gates:** tests citing each function, the golden for its named cause,
   mutants, native both sides, the TAC-08 cells. M.
   Native, both sides, seed 42, regiments boarded through the regiment Move
   order and the assault given from the fleet menu: Empire at Vagran, the
   first assault repulsed (picture 11161, "Alliance Troops have defended
   Vagran"), the second after three bombardments taking the system;
   Alliance at Corellia from a Bulk Transport (picture 11160, "Imperial
   Troops have defended Corellia from an Alliance assault"). Both reports
   opened at once with their regiment lists filled. Mutants on the changed
   lines leave no survivor. The TAC-08 cells are carried to the next plan
   by Tom's decision of 2026-10-20 (`or-d-gnd-plan-glw`).

### Phase L: the victory rules beyond AD and CD (`or-q-port-victory-rules-ce9`)

1. **The tie:** AW unless EW was set earlier (the port gives the Empire the
   win), both game types. S.
2. **Leaders and Coruscant by DatId** (`0x32000242`, `0x30000240`,
   `0x35000281`, `0x34000280`), and the word frozen while one is missing
   (`FUN_005180e0` returns 0); first trace whether a principal leader can
   die (`FUN_00505190`), Ghidra and a VM check. S + M.
   The death trace rests on the code read (a major is injured, never
   destroyed: slot `+0x2e0` returns 0). Wine, done: with Han undefended at
   Caprionril (`save_tree.py undefended`), Daala's Assassination files
   "Han Solo Injured" and a Mission Report reading "Han Solo has not been
   eliminated"; Han's injury reads 36 and he stays on the map
   (`victory-flag-word.md` 3).
3. **The messages from the flag word, done:** Rebel HQ Destroyed `0x1f` on
   any AD change, a capture included (`FUN_0048e5a0`); Coruscant Captured
   (`FUN_0048f790`, TEXTSTRA `0x7170`/`0x7171`, movie `0x74`;
   `strike_orders.rs`, `word_movies`). S-M.
4. **Remove** the no-HQ fallback (`main.rs` first two systems) and fix
   `victory.md` (notification `0x128`). S.
5. **The ending, done:** the advisor ending script (ALSPRITE 10093/10094,
   EMSPRITE 11093/11094), the movie by code `0x10`-`0x13` (MDATA 105-108,
   `movies.rs` `ending_code`, `ending_movie`), the main menu; the invented
   `VictoryModal` and its text are gone (`or-port-movie-triggers-45gd`). M-L.
6. **Checks, done:** native and browser, both sides and both game types;
   one ending in the VM. M.
   Native, Empire, Standard: the ending plays its script, movie `0x6d`
   (MDATA.108) and the main menu; Planet Destroyed and Rebel HQ Destroyed
   play their movies with captions; a save taken after the win replays the
   ending when loaded. Native, Alliance, Standard (the H10 run): Coruscant
   Captured plays movie MDATA.103 with "The Alliance has seized the
   Imperial capital system of Coruscant.", ending slot 86 is stamped,
   movie `0x6b` (MDATA.105) plays, then the main menu. VM, done, Empire
   viewer, Standard: with Palpatine in the Alliance Jail
   (`empire-day660-palpatine-jailed.sav`) the Alliance wins between days
   682 and 692, the ending film runs on its own and ends on the main menu
   (`victory-flag-word.md` section 5). Native, Headquarters Only (the
   menu's game-type toggle): Empire, seed 42, Hoth destroyed from the fleet
   menu with no leader held, ending slot 84, movie `0x6d`, the main menu;
   Alliance, seed 3, Coruscant taken by the fleet menu's assault after
   `Strip defenders at`, no leader held, ending slot 86, movie `0x6b`, the
   main menu. Browser, production build, `tools/interface-parity/endings.mjs`,
   8 of 8 cases, Chrome muted with no audio output: the menu's game-type
   toggle changes the Cloud City control's art and starts a new game of
   that type ("Easy, Small Galaxy, Standard Game" or "... Headquarters
   Only"); over it, Game Options loads a native save of the same type one
   order before the win. Empire, seed 42, day 9, the Death Star fleet at
   Hoth (Standard with Luke and Mon Mothma jailed): Destroy System (`0x223`)
   from the fleet menu plays `0x6f` (MDATA.101) and `0x73` (MDATA.102),
   ending slot 84 is stamped, and movie `0x6d` plays full screen in the
   `<video>` overlay to its end (114.46 s), then the main menu. Alliance,
   seed 3, day 60, regiments aboard the fleet at a stripped Coruscant
   (Standard with Vader and Palpatine jailed): Planetary Assault (`0x234`)
   opens the Operation Report; Coruscant Captured (`0x74`, MDATA.103) waits
   until the report is closed (`movies::clear_for_movie`), ending slot 86
   is stamped, and `0x6b` plays to its end (140.13 s), then the main menu.
   Each won campaign, saved from Game Options (Empire day 12, Alliance day
   62), stamps its slot again when loaded in a fresh page and replays the
   same ending to the main menu. The gate fails with `0x12` given movie
   `0x6c`, with the game-type toggle disabled, and with the shell's
   `ended` handler removed.
7. **Movies in the browser, done:** an HTML `<video>` overlay driven by the
   `or_movie_play` and `or_movie_stop` imports, with the `or_movie_ended`
   export called on end, error, Escape, a click or a key
   (`video_player.rs`, `web/index.html`); `scripts/stage-web-movies.sh`
   stages the movies into the web package, and a missing file is skipped. M.

### Phase M: the bombardment report window (EVT-03, UIP-B05)

One class serves the bombardment (`0x182`) and the assault (`0x183`)
reports (`FUN_00496770.c`): six unit lists, the picture by which list holds
units (`0x2b9b`, `0x2b9c`, else `0x2b9a`), a rail of five buttons and a
System Forces view with six tabs (guide captures 061-063). In the VM the
alert opens itself within a second of the order. The port files message
`0xf` with picture `0x2b9a` always and opens it only from the Message Index.

1. **Trace, done** (`ghidra/notes/operation-report-window.md`): constructor,
   resources, rail, tabs, list painting, and the live alert's opener. M-L.
2. **Trace, done,** what fills lists `+0x80` and `+0x88`, and the picture. S-M.
3. **VM A0 captures, done** from `empire-day53-yaga-orbit` and
   `empire-day128-assault-fails-selonia`: the three orders a fleet without
   a Death Star offers, a shield-blocked run, the rail, the tabs, Display,
   the Index reopen (`operation-report-window.md`, "In the original"). M.
4. **Port, done** (`rebellion-render/src/operation_report.rs`), on the
   windowing toolkit. L, after 1 and 3.
5. **Fill, done,** its lists from the strike and assault reports. M.
6. **Open it, done** (`MessageLog::take_alerts`), for the bombarder at once,
   and again from the Index. S-M.
7. **Checks, done:** native and browser, both sides, EVT-03's 15 cells. M.
   Native, both sides, orders from the fleet menu: the report opens at once.
   - Empire at Vagran: 11163 for each order.
   - Alliance at Corellia: 11164 (a ship lost), 11162 (nothing destroyed)
     and 11163 (a facility destroyed).
   - Each picture is whole in the shell's aperture.

   EVT-03 cells, native: Empire at Vagran, at Tralus (seed 3) and at
   Ghorman; Alliance at Corfai and Corellia. All 15 pass on both sides:
   - C001 military, C002 civilian, C003 general (11163);
   - C004 shield blocked: Empire at Tralus, GenCore Level I; Alliance at
     Corellia on day 53, under a GenCore Level I the Empire built by day
     51. Target Military Facilities lands no hit; LNR Series I and the
     generator are listed operational; "No Casualties";
   - C005/C006 pictures (11163 something destroyed, 11162 nothing);
   - C007/C008 assault summary and success (11161 Empire, 11160 Alliance);
   - C009 assault failure: Alliance at Corellia on day 35, an Army Regiment
     boarded at Hoth against three Imperial regiments, 0x7092, the regiment
     destroyed; Empire 11161;
   - C010 the six force tabs;
   - C011 destroyed and C013 operational, with "None" and "No Casualties";
   - C012 damaged, as the original lists it (Wine,
     `alliance-day927-orto-transport-damaged.sav`, `FUN_0042c3b0.c:350-410`,
     `FUN_00501c00`): a damaged survivor under Operational with burn marks
     (Imperial Star Destroyer 1, 2250/2750, at Vagran; Alliance Escort
     Carrier 1, 300/900, at Corellia); a ship the batteries destroyed under
     Destroyed with the damage art (Corellian Corvette 1 at Corellia on day
     2; Imperial Star Destroyer 1 at Ghorman on day 294, from 171/2750);
   - C014 Go To system;
   - C015 Go To fleet, which opens the system's Fleet window with nothing
     selected (`FUN_0045aac0`, `operation-report-window.md`).

   Browser, both sides: `tools/interface-parity/orbital-strike.mjs`, 22 of
   22 cases, Chrome for Testing 151 at 640x480. The interface-test
   fixture's scenarios 70-74 stage the shielded strike, both assaults and
   both damaged losses.
   - C001-C003: each order opens its report at once.
   - C004: 0 hits, picture 0x2b9a, the generators operational.
   - C005/C006: pictures 0x2b9b and 0x2b9a.
   - C007-C009: the assault Summary (0x2b98 or 0x2b99) with the capture and
     the defence texts.
   - C010: six tabs per Forces view.
   - C011/C013: the lists.
   - C012: the damage art (GOKRES portrait + 0x2000 under the keyed
     portrait) pixel for pixel under Operational and under Destroyed; the
     stack matches the Wine capture of Corellian Corvette 1.
   - C014/C015: Goto System and Goto Fleet close the report on the
     system's sector and Fleet windows.
   - Each probe fails with its feature disabled.

   The audit ledgers carry these results: P26 passes and the EVT-03
   surface is complete, on Chrome evidence by Tom's decision of 2026-10-19
   (Firefox and Safari parity is not claimed).

### Phase N: the seeding fallback

1. `apply_seeds_legacy`, a proximity spread used when the data lacks
   Coruscant or Yavin, has no source: remove it (the loader refuses such
   data) or source it. S.

### Phase O: the support writes (Decision 3)

Traced in `ghidra/notes/support-shifts.md` (Port), `timer-scheduler.md` and
`uprising-incident.md`. Support (`+0x58`) has one setter, `FUN_00509c30`,
whose hook (slot `+0x1d0`, `FUN_005109f0`) reruns control at once
(`FUN_0050b610`); no clock writes support.

1. **The drift, done.** `FUN_0050b230` -> `FUN_00559c40` goes to `+0x6c`
   through `FUN_00509ec0`, the output diversion roll's odds (`FUN_005166a0`),
   never to support, and no `EconomyEvent`, `GameEffect` or telemetry
   record carries it. The drift, the garrison requirement (`+0x80`) and the
   collection rate (`+0x70`) are per-system state the support hook writes
   (`economy::SupportValues` in `GameWorld::support_values`), and the
   revolt bit (`+0x88` bit 2, `FUN_0050a130`) is `GameWorld::revolting`. M.
2. **`EventAction::ShiftPopularity`, done:** it had no producer; removed. S.
3. **The setter's hook, done** (`support_shift::support_changed`,
   `FUN_005109f0.c:14-29`): control reruns at once, then
   `economy::refresh_support_values` writes the three values
   (`FUN_0050b5a0`, `FUN_0050b230`, `FUN_0050b2c0`), and event `0x31b`
   joins the queue `fire` serves, so a kill's joins land in its step under
   the per-step cap. Strike and
   assault effects carry the changes (`controls`) for their records and
   Loyalty Change Messages. M-L.
4. **Timer `0x384`, done:** a revolting holder loses GNPRTB 7697 (1)
   support every 7698 (30) days (`FUN_00510f20`, `FUN_0050caa0`;
   `UprisingEvent::RevoltSupportLoss`). Timer `0x385`'s change (GNPRTB 7691)
   is 0 in the shipped data. S-M.
5. **Checks:** the replay golden regenerated for the cause; a native run on
   both sides where a kill turns a system the same day, done (build
   ac90a418, seed 42, release). Alliance, day 1, Corellia: a Military
   bombardment killed the one Imperial regiment and the holder went Empire
   to Neutral that day (support 69/31 to 59/41). Empire, day 0, Vagran:
   Destroy System turned the Alliance system Neutral and destroyed on day 0,
   with film `0x6f`. The golden was regenerated for these writes in
   ff3099fd. M.
6. **Control after a regiment change, done:** every port path that adds,
   moves or removes a regiment at a system (load, unload, a regiment's own
   move and arrival, a finished build, scrap, a kill by bombardment,
   assault, Destroy System or sabotage, an uprising incident's loss) queues
   event `0x301`/`0x320` (`support_shift::regiments_changed`), whose
   `FUN_00508660` reruns `FUN_0050b610` when the operation's phase 5
   serves the queue (`fire`; the VM: the last Imperial
   regiment taken aboard at Tralus turns it to the Alliance that day). An
   unpopulated system is held by its only side's regiments
   (`FUN_0050b610.c:24-70`). A garrisoned system the Death Star kills turns
   neutral as its last regiment dies, with the troop-bit record
   (`FUN_004f7410` -> `FUN_004fb310`, SDPRTB `0x1442`). The daily control
   pass (`EconomyEvent::ControlResolved`) is retired: support, regiments and
   the populated bit are the holder's only inputs, each hooked, and the
   seeding leaves every holder as `FUN_00508250` would (a data-backed
   test). Loyalty Change Messages file only for a change without regiment
   bits at either end (`FUN_00510d70.c:12-18`, `FUN_00499760.c:23-29`). M.

## Decisions for Phases H-N

Tom, 2026-10-17: "We'll commit when we hit 100% on this plan" and "Use
your judgement, favoring parity". So:

1. **Automatic landing** and the occupation capture go once the assault is
   ported, unless Phase K item 3 sources them.
2. **The station leaves `ships`** as `FUN_0040b0e0` exports it (Phase J
   item 5), with retreat, withdrawal and the write-back reworked to match.
3. **Mission-dialog object targeting** joins Phase I (item 5).
4. **The two invented Death Star messages** go with the countdown.
5. **The ending** (Phase L item 5) and **the report window** (Phase M) move
   here from UIP-B05, which keeps its other rows.
6. **One commit** at 100%, carrying Phases A-O and the C6 harness fix.

### Phase P: capital-ship products as objects (`or-port-unfinished-capital-ship-products-czev`)

Added under "We go for parity" (Tom, 2026-10-18). An ordered capital ship
exists from its order as an unfinished member of its side's bit-1 spare
fleet at its destination, or of its destination fleet (`FUN_0052a140`,
`FUN_004f7d50`, `FUN_005097d0`, `FUN_0050bc60`), and its yard completes it in
place (`FUN_0052bee0`). Save v63.

1. **P1, the object at the order** (M): `ShipInstance::completed`,
   `QueueItem::ship`; a Build replaces the area's queue (`FUN_00529ab0`)
   and its Death Star room first gives back the area's own products
   (`FUN_00552210` -> `FUN_00552000` -> `FUN_00552dd0`), so the original's
   Build at a yard already building one is accepted, the new ship takes the
   old one's place in the same fleet, and that fleet stays. Another yard's
   queued Death Star still refuses `{0x90, 3}`. The give-back covers the
   area's queued squadrons and regiments too (`ProductRoom::excluding`).
   Ship numbering runs in event `0x300` (`FUN_005140c0` -> `FUN_00517760`),
   one frame after the order (`port:` the port numbers at the order).
2. **P2, the reader audit** (M-L): `usable()` reads `completed`, so a fleet
   of unfinished ships neither fights, blockades nor moves; Join moves only
   completed ships (`FUN_004feca0`); Bombard and Assault refuse `{1, 0x10}`
   (`FUN_004ffa70`); the per-unit Confirmed Move.
3. **P3, room, maintenance and the AI census** (S).
4. **P4, missions** (S): an uncompleted target is refused `0x34`
   (`FUN_00593500`).
5. **P5, the UI mark, done** (S-M): the under-construction backdrop
   11570/11572 (`FUN_0042c3b0.c:412-431`) stacks under an unfinished ship's
   mini and portrait; the Build Selection list and box show the class
   portrait (`FUN_00437880`, `+0x30 & 0xfff`), the name starting the
   portrait's height down (`:138-139`). Mutants: 28, none missed. Browser,
   both sides: fleet-window `unfinished` (158 and 153 mark pixels, none with
   the mark disabled) and build-selection, which presses the Stop
   confirmation (`FUN_00487cc0`). Native, both sides: release build, seed
   42, the red and green marks under the unfinished ships.
6. **A manager's side change** (within P1): a holder change destroys every
   product of the system's own managers, wherever it sits, with reason
   `0x16` and no refund (`FUN_004f8680.c:28-31` -> `FUN_004f6f40` -> manager
   `+0xf0` `FUN_0052b790` -> `FUN_0052a2c0`). The port drains a transient,
   unsaved list at each control change (`support_shift`), and only an
   emptied fleet's cleanup follows. Wine: the Duros replay's lost Orbital
   Shipyard is a dead record with reason `0x16` after the capture.
   The other managers' facility, regiment and squadron products at the
   system (`0x1000` request, event `0x307` -> `FUN_00515480`) and special
   forces (`FUN_00503d90`) are `or-port-holder-change-products-kr13`,
   ported (see Completion).
7. **Death Star room for fleets in transit**: Build counts the same set as
   Move (`FUN_004ffe70` mode 3 counts ships of en-route fleets, whose
   container changes at the order; `movement::death_star_room`).

### Phase Q: strategic windows on load, and window drag

1. **Restore on load, done** (`or-port-window-restore-on-load-iclp`, M-L):
   the save writes the galaxy's child windows bottom to top, hidden ones at
   their own z place (`FUN_00440ea0.c:19-24`); the load appends hidden ones
   to the rail (`FUN_00428b40`), and `GalaxyFocusState::after_load` sets the
   focus. A Fleet window saves unless it is on the rail
   (`SystemWindowState::rail_holds_fleet`), whatever the frame paints: Game
   Options suspends the strategic scene, while `FUN_00440ea0` walks the
   view's child windows, which it leaves in place. Native, both sides, seed
   42: a Fleet window saved over a System window comes back in front with
   the keyboard; Escape closes the Fleet window, then the System window.
   Browser, both sides, production build
   (`tools/interface-parity/window-restore.mjs`, 2 of 2, Chrome; Firefox
   and Safari parity is not claimed): on the Empire seed 42 day 9 and the
   Alliance seed 3 day 60 campaigns, a System window dragged and shown on
   Shipyards, a second System window on the rail, a Fleet window dragged
   over the first and the Uprisings GID mode, saved from Game Options and
   loaded in a fresh page, come back pixel for pixel: the Fleet window in
   front where it was dragged, the System window where it was dragged on
   its Overview page, the rail's first slot as saved. Escape closes the
   Fleet window and the System window takes the focus; a second Escape
   closes it, a third leaves the rail. With the sector closed, the galaxy
   shows Uprisings. The gate fails when the load restores no window.
   Whether a restored window keeps a place outside the galaxy view, which
   the port clamps, is `or-q-restored-window-clamp-wqt5`.
2. **Window drag** (S-M): a window's title and body drags
   (`FUN_00606ee0`, `FUN_006071e0`, `FUN_00607010`).

## Completion

Weights S 1, S-M 1.5, M 2, M-L 3, L 4; Phases A-G's 52 items count as M
each. Overlaps count once (Phase I's tactical shield and uncompleted
station sit in Phase J item 11; Phase K's summary window is Phase M).

| Part | Items | Weight | Done |
|---|---|---|---|
| Phases A-G | 52 | 104 | 104 |
| H jail | 11 | 18.5 | 18.5 |
| I Death Star product | 9 | 13.5 | 13.5 |
| J tactical Death Star | 13 | 23.5 | 23.5 |
| K planetary assault | 9 | 19.5 | 19.5 |
| L victory | 7 | 13.5 | 13.5 |
| M report window | 7 | 16 | 16 |
| N seeding fallback | 1 | 1 | 1 |
| O support writes | 6 | 11.5 | 11.5 |
| P products as objects | 5 | 8.5 | 8.5 |
| Q window restore and drag | 2 | 4.5 | 4.5 |
| **Total** | **122** | **234** | **234 (100%)** |

The other managers' products at a captured system (Phase P item 6,
`or-port-holder-change-products-kr13`) are ported: the captured system's
managers destroy every product they list (`FUN_0052b790` ->
`FUN_0052a2c0`, reason 0x16), and another yard's products, deliveries and
units bound for the system take the 0x307 autoscrap request
(`FUN_00515480`: stay if accepted, else yard or origin, nearest own-side
system, else reason 2; special forces by class, `FUN_00503d90`). Tests
cover both factions; seed 42 is unchanged. Native runs on the Mini (seed
42) confirm both sides end to end: when the Empire loses Corellia, its
Training Facility queue dies and Denab's TIE squadrons bound for Corellia
finish at Denab, and when the Alliance loses Ghorman to an assault from
Fleet 1 (regiments embarked with the dev command `Embark`), Ghorman's
queued products die, including the unfinished Nebulon-B in Fleet 8 at
Vagran and the Mine bound for Yavin. On the Alliance side, a product bound
for a captured system going back to its yard stays covered by unit tests.

Carried to the next plan: C019's raster match and the tactical projection
scale (`or-d3drm-raster-match-vog4`) and the Alliance hulls' layout in
the day-676 Coruscant battle, up to about 45 px off A0
(`or-port-c019-alliance-hull-layout-fqrk`); the
original's blockade end and the Destroy System cascade's characters, run
on Wine or the VM (Phase A results); the TAC-08 cells of the planetary
assault (A1 fixtures, compare regions, the Alliance's A0 frames; Phase K
item 9, `or-d-gnd-plan-glw`).

## Out of scope (carried queue)

- The Headquarters' Move and its in-transit branch (orders `0x201`,
  `0x202`, `0x204`).
- The message-log text outside this plan's features that has no TEXTSTRA
  source (combat results, manufacturing and delivery, arrivals, uprisings,
  missions, session lines): `or-invented-log-text-82ba`.
