---
title: "Morale retreat"
description: "Each tactical side withdraws when its strength falls below a threshold of the enemy's; the threshold comes from the computer player's battle evaluator, the permission from the strategic retreat search"
category: "ghidra"
created: 2026-10-16
updated: 2026-10-16
---

# Morale retreat

Traced 2026-10-16 in the saved `REBEXE.EXE` project (headless, read-only, on a
scratch copy). Cited `.c` files are in this folder.

## The check (`FUN_005d19a0`, side vtable `+0x2c`)

`FUN_005d14d0`, the side step, calls vtable `+0x2c` after its forces have
applied their damage for the step (`FUN_005d14d0.c:101`) and before the
shield recharge. `FUN_005d19a0` answers 1 (retreat) when:

1. vtable `+0xc` = `FUN_005d0f50` says the side may retreat
   (`FUN_005d19a0.c:13-14`):
   - side `+0x80` is non-zero and `+0x84` is zero (`FUN_005d0f50.c:10`);
   - `FUN_005c7e40` on the opposing side finds no capital ship (list
     `+0x58`) with unit `+0x150` set (`FUN_005c7e40.c:8-18`; the call is on
     `battle+0x250` when this side's id is 0, else `battle+0x24c`). hyp:
     unit `+0x150` is the gravity-well flag (set in the unit constructor
     `FUN_005b05c0.c:165` from a constructor argument);
   - a squadron (`+0x64`) or capital ship (`+0x58`) has a working hyperdrive
     (`FUN_005b1b70`), or, on the side whose id (vtable `+4`) is 1, a Death
     Star is present
     (`FUN_005d0f50.c:16-40`);
2. and then any of:
   - the stalemate flag `+0xb0` is 1 (`FUN_005d19a0.c:15`);
   - after recomputing its strength (vtable `+0x14` = `FUN_005d0ce0`), the
     side's strength `+0x3c` is 0.0 (`_DAT_0066d0f8` = 0.0;
     `FUN_005d19a0.c:18-21`);
   - with the opponent's strength recomputed and above 0.0
     (`FUN_005afa30` returns the opponent's `+0x3c`), own / opponent is
     below the side's threshold `+0x24`, or equal to 0.0
     (`FUN_005d19a0.c:22-40`).

The side's strength is `FUN_005d0ce0`: firepower against capitals
(`FUN_005b7a80`) of its squadrons and of each capital ship whose attacker's
damage rate does not already meet its shield recharge
(`FUN_005d0ce0.c:21-76`). A side whose every capital ship is out-damaged and
which has no squadrons has strength 0.0 and retreats at once.

On 1, `FUN_005d14d0.c:102-146` sends every hyperdrive unit out
(`FUN_005af910`), docks the rest of the squadrons in an escaping carrier
(`FUN_005afe40`) or destroys them (`FUN_005af7a0`), destroys capital ships
without a hyperdrive, and sets `+0x84` = 5. This is the same retreat the
stalemate rule uses ([tactical-engagement-ai.md](tactical-engagement-ai.md),
step 3).

The headless step `FUN_005ae9c0` steps `battle+0x250` first, then
`battle+0x24c` (`FUN_005ae9c0.c:26-27`). Side `+0x24c` is the Alliance and
`+0x250` the Empire: `FUN_00599c90` gives `+0x24c` the values of the
strategic side 1 and `+0x250` those of side 2 (below), and the tactical
manager's numbering is `side != 1` ([side-numbering.md](side-numbering.md)).

## Where `+0x24`, `+0x7c` and `+0x80` come from

`FUN_005c7cb0`, the side constructor, sets `+0x24` = 0.33 (`0x3ea8f5c3`) and
`+0x80` = 0 (`FUN_005c7cb0.c:50-52`). Every battle the session starts
overwrites both.

`FUN_0040b0e0` (states 7 and 8 of `FUN_0040a050`) calls
`FUN_0041e1f0(side, &p2, &p3, &p4, &p5)` for side 1 and side 2
(`0x0040b2ec`, `0x0040b30a`; `FUN_0041e1f0` → `FUN_0051dce0` →
`FUN_00516730`). `FUN_00516730` reads the battle system:

| Out | Rule | Lines |
|---|---|---|
| p2 | an opposing capital ship with vtable `+0x254` − `+0x280` ≠ 0 (hyp: an active gravity well) | `FUN_00516730.c:59-85` |
| p3 | one of the side's fleets or system squadrons answers vtable `+0x34(0)` | `:93-134` |
| p4 | `FUN_005545e0` finds a retreat system (`0x90`-`0x97`) for that fleet | `:149-178` |
| p5 | transit distance to it, `FUN_0055d8a0` | `:185-196` |

The pushes before `FUN_00595d60` (`0x0040b30f`-`0x0040b3d7`) and before
`FUN_00599c90` (`0x00595d75`-`0x00595e22`) carry them on as follows; the
headless writes are `FUN_00599c90.c:99-186`, the windowed ones
`FUN_00599c90.c:57-76` through `FUN_005a8b10` / `FUN_005a8af0`:

| `FUN_00599c90` stack | Value | Written to |
|---|---|---|
| `+0x18` / `+0x1c` | side 1 / side 2 p5 × 0.4 (`_DAT_0066babc`) | side `+0x7c` (`FUN_005afb50`; windowed × 1000.0, `_DAT_0066c288`) |
| `+0x20` / `+0x24` | side 1 / side 2 p4 | side `+0x80` (`FUN_005afb30`, `FUN_005a8af0`) |
| `+0x28` / `+0x2c` | export `+0x10` / `+0x14` | side `+0x24` |
| `+0x30` | system `+0x78` control ≠ 1 | battle `+0x44` |
| `+0x40` | export `+0xc` (1 = Simulate) | engine `+0x668` |

Both sides' `+0xb0` are cleared (`FUN_00599c90.c:150,162`). p2 and p3 are
not passed to the engine; the engine applies its own gravity-well test
(`FUN_005c7e40`). hyp: `+0x7c` is the delay before a retreating ship jumps;
its reader is untraced.

Export `+0x10` / `+0x14` are the session's `+0xc` / `+0x10`
(`FUN_0040a050.c:121-123`, `156-158`). `FUN_004362b0` fills them in state 6
from the slots' vtable `+0x3c` = `FUN_00488600`, which hands back slot
`+0x3c`: the Alliance slot (`session+0x10`) into session `+0xc`, the Empire
slot (`+0x14`) into session `+0x10` (`0x004362e8`-`0x0043631a`; the
decompile drops the fourth argument).

## The thresholds

**Computer player.** `FUN_00485d50`, the computer slot's answer, zeroes
`+0x3c` and, while slot `+0x418` is 0, has component `0xd` fill it through
`FUN_004bf300` → its object `+0x60`, vtable `+0x10`
(`FUN_00485d50.c:7-13`). `FUN_004bf7c0` builds that object by component
`+0x5c` (1-7); the seven classes' `+0x10` (`FUN_004e6110`, `FUN_004e6a60`,
`FUN_004e77f0`, `FUN_004e87a0`, `FUN_004e94f0`, `FUN_004e9ec0`,
`FUN_004eabb0`) are the same code, each clamping at its own 0.9 constant
(`0x0065c8e4`, `…c914`, `…c944`, `…c974`, `…c9a4`, `…c9d4`, `…ca04`, all
`0x3f666666`). `FUN_004e6110`:

- the threshold starts at 0.9 (`FUN_004e6110.c:26`);
- when the AI's record of the battle system (`FUN_00403d30`, context
  `+0xc` → `+0x2c`) is own (`+0x30` bit 1) and has own operational
  squadrons (`+0xac` > 0) (`:32-33`):
  - at an HQ system (record `+0x28` bit `0x80000000`) the threshold is 0.0
    (`:71-72`);
  - otherwise it sums the attack (fleet record `+0x58`, ship attack, see
    `ship-damage-repair.md`) of every fleet at the system
    (`FUN_004ffe70(…, 3)`), own fleets (record `+0x38` bit 1, set by
    `FUN_00433620.c:41-43` when the fleet's owner is the record's side)
    apart from the others, and takes own / enemy (own alone when no enemy
    fleet is recorded), clamped to 0.9 (`:40-66`).

So the computer withdraws below 0.9 of the enemy's strength everywhere
except at its own guarded systems, where it holds on to the ratio it
expected, and at its HQ, where only zero strength makes it leave.

**Local player.** `FUN_004865b0` zeroes slot `+0x3c` (`FUN_004865b0.c:17`).
With a threshold of 0.0 the ratio test can only fire at a ratio of 0.0, so
the player's side withdraws on its own only when its strength reaches 0.0.
Whether the Battle Alert writes slot `+0x3c` is untraced.

## Observed

Save `tools/original-capture/saves/empire-day30-yaga-alert-control.sav`, Empire
player, 2026-10-16: the Alliance computer's Bulk Cruiser attacks the
Imperial blockade of Yaga Minor (Star Destroyer, Carrack, TIE squadrons).
Simulate reports "The Imperial fleet is victorious. Imperial forces have
maintained the blockade of Yaga Minor. The Alliance fleet has withdrawn."
(`0x7002`, `0x7015`, `0x700c`); both sides' capital ships are listed
Operational with No Casualties.
Yaga Minor is not the Alliance's, so its threshold is 0.9, and the Bulk
Cruiser's strength against the Imperial force is far below 0.9 of it.

## Port

`rebellion-core` runs the check in `BattleSession::headless_step`, which
steps the Empire's side before the Alliance's and, after each side's damage
and before its recharge, calls `morale_withdraws` (`tactical.rs`). A side
withdraws when `BattleSession::morale` permits it (`retreat_permitted`, not
yet `retreated`), no opposing capital ship with a gravity-well projector is
alive (hyp: unit `+0x150`), the side has a way out, and `stalemate_strength`
is 0 or, with the opponent's above 0, below the side's `threshold`. A way
out is a capital ship with a working hyperdrive, a squadron whose FIGHTSD
class has a hyperdrive or damaged-hyperdrive rating (the fighter factory
`FUN_0040be90` passes that count as the constructor's `+0x3ec`, the field
`FUN_005b1bc0` reads; five of the eight shipped classes have 60), or the
Empire's Death Star (side id 1 of `FUN_005d0f50.c:32-40`, the tactical
Empire; operational or `+0x74` set). The retreat body is
`stalemate_resolve_side`: hyperdrive ships and squadrons jump out, the other
squadrons dock in an escaping carrier or are destroyed, capital ships
without a hyperdrive are destroyed, and an operational Death Star is
committed (`+0x78`), which removes it from the side's remaining forces. A
ship marked `retreated` stays in its fleet at the system when the battle is
written back (`tactical/apply.rs`); the loser's fleets leave through the
post-battle pass (below, "Who leaves the system").

`tactical::apply::seat_morale` sets both fields before a battle:

- permission is `post_battle::retreat_destination` finding a place for the
  side's fleet (port: stands in for `FUN_00516730`; the chain without stage
  1 and never the battle system, so the countdown runs from the transit to
  the nearest other own system, as the VM's Carida withdrawal did; a side
  holding no other system retreats to the nearest neutral, else the nearest
  enemy-held one, stages 6 and 7);
- the threshold is `ai_records::battle_withdraw_threshold` (`FUN_004e6110`)
  for a computer side and 0.0 for the human side.

`resolve_system_space_combat` seats both sides as computer players;
`resolve_system_space_combat_for` seats the person's side as human and is
what the game's unwatched battles call. `tactical_flow::begin_player_battle`
seats the player's side as human, so the Battle Alert's Simulate reads them.
The played (windowed) step does not run the check. Further rules of the port:

- An operational Empire Death Star adds `_DAT_0066d0f0` = 20000000.0 (read
  from `REBEXE.EXE`) to its own side's strength (`FUN_005d0ce0.c:80-85`)
  and to the Alliance side's when the station is attackable and the
  Alliance's listed squadrons strictly outnumber the Empire's
  (`FUN_005afcc0`) or an Alliance squadron holds the Attack Death Star
  order, 6 (`:86-103`; `death_star_strengthens_its_attackers`).
- A side has a way out when a capital ship or squadron of it has a working
  hyperdrive (`FUN_005b1b70`, `side_has_working_hyperdrive`) or its Empire
  Death Star is operational or not completed (`FUN_005d0f50.c:32-40`,
  `death_star_allows_retreat`), which tests no fleet or drive. For an
  object that cannot travel the source destroys it, reason 5
  (`FUN_00556390`).
- `death_star_withdrew` holds only when the Death Star was operational at the
  retreat (`FUN_005d14d0.c:124-127`).
- `retreat_side` commits an operational station (`self.death_star`,
  `+0x78`) whether or not a hyperdrive of the side works, and the station
  recalls its squadrons still waiting to launch (`FUN_005ba5e0.c:11-15` →
  `FUN_005b8d10`); an uncompleted station is not operational and never
  departs.
- A side counts as withdrawn (results text, voice, the winner when both
  sides reach nothing) by the windowed rule below (`side_withdrew`): no unit
  alive or in its jump, and a unit jumped out or the Empire's station
  committed. The controller test (`+0x84` 3, 4 or 5; the headless latch,
  `FUN_005ae9c0.c:117-141`, is 5) holds whenever the port jumps a unit or
  commits the station. The tactical screen has no per-ship retreat: the key
  handler `FUN_0059b730` has no case for it and the order panels offer codes
  2 and 4-11 (`tactical-command-assignment.md`); order 3 comes only from
  `FUN_005ad530`.
- The stalemate loser goes through the same permission test; a side that
  may not retreat is destroyed (`FUN_005ae9c0.c:62-103`).
- A retreating squadron in flight docks in its own carrier (the station
  only while operational, a ship only while in the battle), else the first
  ship of its side in the battle, else, for the Empire, its operational
  station, each only with room (`FUN_005afe40.c:17-58`, `FUN_005b84a0`;
  `retreat_berth`). Where a squadron ends strategically is set by the
  battle-end docking and the import (below).
- `+0xac` of the AI's system record counts the active squadrons present, any
  side (`FUN_00433e40.c:564-583`); the port counts every squadron there.
- With one AI, the faction it does not play is the human side (threshold 0.0).

## Windowed withdrawal (code read)

Traced 2026-10-17. A played battle's side withdraws through
`FUN_005ad530(battle, side)` (callers `FUN_0059fd40` and the AI controller
`FUN_005c8780`); the side controller `FUN_005c8780` then runs it each update
with the update's milliseconds.

`FUN_005ad530`:

- sets the side controller's `+0x84` to 2 (`FUN_005ad530.c:20`) and, for the
  local side, queues voice `0x8f` (side 0) or `0x10d` (side 1) (`:21-31`);
- gives each group of the side's list `+0x70` an order through vtable
  `+0x64` (`:32-60`): a fighter group (`vt+4 == 4`) whose first squadron
  (`FUN_005c6070`) has no hyperdrive rating (`FUN_005b1bc0` = 0.0,
  `_DAT_0066c284`) Recover (2), any other group Withdraw (3); a fighter group
  holding order 6 first posts `0x3c` state 8 (the run broken off); a fighter
  group with no squadron gets no order;
- gives each capital ship's group (`FUN_005aded0`) order 3 (`:61-81`);
- sets the side's countdown `+0x7c` to at least 150000.0 ms
  (`DAT_0066c234`) when the side is 1 (the Empire) and the station is
  operational (`FUN_005a9330`, `FUN_005ba420`), else to at least 30000.0 ms
  (`DAT_0066c240`) (`:83-100`). The countdown starts as the retreat
  system's transit distance × 0.4 × 1000.0 (`FUN_00599c90.c:57-76`,
  `FUN_005a8b10`, `_DAT_0066babc`, `_DAT_0066c288`).

`FUN_005c8780.c:28-125`, state `+0x84`:

- 2: the countdown falls by the update's milliseconds; at 0.0 or below
  (`_DAT_0066cbb4`) the state becomes 3, and for side 1 with the station
  operational it posts `0x45` (`FUN_005ba5e0`, the departure). Only the
  countdown ends state 2. The departure sets the station's `+0x78`, recalls
  its waiting squadrons (`FUN_005b8d10`) and drops its decal
  (`FUN_005c1200`); it touches no other unit, so the side's ships and
  squadrons still jump under order 3.
- 3: while a capital ship (`+0x58`) or squadron (`+0x64`) of the side with
  a working hyperdrive (`FUN_005b1b70`) is alive (`FUN_005ad870`: state 2,
  4 or 6) or in state 7 or 8, nothing; otherwise the state becomes 4 and
  each unit without a working hyperdrive gets `0x2f` (`FUN_005b2720`: if
  it is alive with hull above 0.0, its orders are cleared and it returns to
  state 2, back in the fight).

Order 3, the Withdraw executor (`FUN_005ca6d0` case 3, vtable `0x66cdc8`,
which holds no logic of its own), runs in the group's update: a fighter
group in `FUN_005c6610.c:44-110`, a task force in `FUN_005cc300.c:48-145`.
Both steer the group's leader to the point on the side's own edge: the side
facing (`FUN_005a7490`, side 0 (0, 0, +1), side 1 (0, 0, −1)) times −1.0
(`_DAT_0066cb10`, `_DAT_0066cff8`), its z scaled by 0.5 × `battle+0x8cc`
(`_DAT_0066cb14`, `_DAT_0066cffc`), x and y kept, through the move call
(slot `+0xc`, `FUN_005eb1a0`) with tolerance 1.0. Until the leader arrives
nothing else happens. Then, with the side's countdown (`FUN_005a8ad0`) at
0.0 or below (`_DAT_0066cb18`, `_DAT_0066d000`):

- fighter group: each squadron alive with a working hyperdrive and within
  8.0 (`_DAT_0066cb0c`) of the line along the facing that is not in state
  6, 7 or 8 jumps. The test is `−((line z − z) × facing z) ≤ 8.0`: the
  register holds the line's z and the stack copy (`FUN_005a33b0`) the
  unit's, so a unit up to 8.0 short of the line, or beyond it, passes;
- task force: only its front ship (`FUN_005c6070`) may jump, when alive,
  with a working hyperdrive, within 8.0 (`_DAT_0066cff0`) and not in state
  6, 7 or 8; a front ship that fails any test holds the rest back.

A jump sets state 6 and posts `0x2d` (`FUN_005b1e40`): a unit still alive
with a working hyperdrive enters state 7, any other gets `0x2f`. State 7
lasts 4000.0 ms (`FUN_005a6b40`, `_DAT_0066c24c`); then the battle's unit
pass sets state 8 and `+0x108` and posts `0x2e` (`0x005aa29a`-`0x005aa2e5`):
the unit has left.

`FUN_005a6b40`, the unit in state 7, each update:

- on the first (jump time `+0x50` still 0.0, `_DAT_0066c250`) it plays effect
  `0x1d` for a squadron (vtable `+0x20`: `FUN_0040f340` returns 1) or `0x1f`
  for a capital ship (`FUN_006158b0`, 0) through `FUN_005ba980`; both are
  below `0x20`, so TACTICAL.DLL effects, one variant each, WAVE 13063 and
  13065 (`FUN_005bae60.c:68-70`, `:134-136`);
- it turns the unit to the reverse of its side's facing (`FUN_005a7490`) and
  draws it (`FUN_005c12a0`) at its own x and y and at its z moved against the
  facing by (t × 0.001)² × k (`_DAT_0066c26c`), t the jump time held at
  4000.0. The unit's position is not written. k is `DAT_006bc47c`, `488`,
  `484` or `494` by (`+0x13c` + `+0xec`) & 3: the tactical ordinal
  (`FUN_00599c40`, passed to `FUN_005b05c0` and `FUN_005b9c60`) plus the
  object id. The static initialisers at `0x005a69b0`-`0x005a6a7c` set each k
  to a distance over seconds squared: 400.0 by 4.0 s, 575.0 by 3.75 s, 650.0
  by 3.925 s and 325.0 by 3.725 s (`DAT_0066c258`-`264`, 4000.0 less
  `DAT_0066c270`-`278`).

Object ids come from the object manager's two counters per side, side 0 from
0 and side 1 from 1000000 (`FUN_005a6290.c:22-30`), each id one more than the
last (`FUN_005a6620`, `FUN_005a6640`). Registrations through `FUN_005a6660`
(the first counter) include the headless ship `FUN_005afd70` and the
windowed helpers at `FUN_005be400`-`FUN_005be5b0` and `FUN_005c0170`-
`FUN_005c0340`.

A group holding order 6 when its side withdraws (`FUN_005ad530.c:43-46`)
posts `0x3c` with state 8 (`FUN_005ad4a0` queues it on the local side's list,
`FUN_005be290`). The order request that follows (vtable `+0x64`,
`FUN_005ca530`) only stores `+0x68`, so the group still holds order 6 when
the message is dispatched (`FUN_005caf50`) and `FUN_005cfec0.c:165-185`
runs: past the approach (the previous state neither 1 nor 2) every
participant takes back its state-5 snapshot (`+0x3cc` from `+0x3d0`,
`+0x3c0` from `+0x3c4`), state 2, its place in the battle (`FUN_005a9380`)
and `+0x108` = 0; then the group's order setter (vtable `+0x48`,
`FUN_005c7130`) gives order 3, which deletes the result (`FUN_005ca6d0`;
`FUN_005d0950` clears `DAT_006bc978`). No state 6 or 7 follows.

The computer side decides on its own (`FUN_005c8780.c:133-216`). Its
controller (`+0x8` = 1) in state 0 counts `+0x40` down by 1.0
(`_DAT_0066cbd4`) each update and at 0.0 resets it to 6.0 (`DAT_0066cba4`),
assigns targets (`FUN_005c9200`) and recomputes both strengths (vtable
`+0x14`, `FUN_005c7f70`, the step-for-step twin of `FUN_005d0ce0`, its station
term `_DAT_0066cba0` the same 20000000.0 as `_DAT_0066d0f0`). With the
opponent's strength (`FUN_005a9610`) above 0.0 and its own over it below the
threshold `+0x24` or exactly 0.0, it withdraws when it may (vtable `+0xc`,
`FUN_005c7e80`, the twin of `FUN_005d0f50`), is not the local side, and no
trench run exists (`FUN_005d0ae0`, `DAT_006bc978`) (`:152-165`). The pass
goes on to the station: the Alliance facing an operational one, with no run
ordered (`+0x18`) or under way, the run not warranted (`FUN_005ad930`) and no
squadron in its list `+0x64`, also withdraws when it may (`:187-197`).

Port: `BattleSession::begin_side_withdrawal` (`FUN_005ad530`, breaking a
trench run off through `break_off_trench_run`) and `advance_side_withdrawals`
(`FUN_005c8780.c:28-125`, the Withdraw executors and the jumps), run from the
played battle's step; the Battle Options withdrawal and the computer
controller (`advance_computer_controller`, `computer_withdraws`) call the
first, and `tactical_withdraw_enabled` greys Withdraw once the side's
controller has left state 0 (`FUN_005d0f50.c:10`). `apply::seat_morale`
starts each side's countdown at its transit to the retreat system. The
side's state and countdown are `TacticalMorale::withdrawal`; order 3 is
`TacticalOrder::Withdraw` ("Withdraw", TEXTTACT `0xdde3`); a ship's jump is
`retreating` with `retreat_progress` in milliseconds, a squadron's is in
`TacticalMorale::squadron_jumps`. The jump effects are
`TacticalAudioEvent::SquadronJump` and `CapitalShipJump`; the flight is
`ship_display_position` and `fighter_display_position`, which the renderer
draws. A withdrawing ship steers through its waypoint, so the ship movement
(turn and engine power) flies it; a squadron flies at its speed × 0.95.

## Squadrons without a hyperdrive, the winner, and the strategic aftermath (code read)

A played battle where one side withdraws while the other side's units still
fight (the Empire orders Withdraw with an operational station, the Alliance
destroys the station, the Star Destroyer jumps, its TIE squadrons stay).

**What happens to a withdrawing side's squadrons without a hyperdrive.**
They stay and fight; none jumps, none is lost for lack of a drive, none is
captured (no captive bit is set by a battle, `tactical-results.md` 3).

1. The order: a fighter group whose first squadron has no hyperdrive rating
   gets Recover (2), not Withdraw (3) (`FUN_005ad530.c:32-60`).
2. The side controller's state 3 waits while any capital ship or squadron
   with a working hyperdrive (`FUN_005b1b70`, set from the drive rating
   `FUN_005b1bc0` and the unit's slot `+0x90`) is alive (`FUN_005ad870`:
   state 2, 4 or 6) or in state 7 or 8 (`FUN_005c8780.c:72-101`). With none
   left the state becomes 4 and every unit of both lists whose
   `FUN_005b1b70` is 0 gets message `0x2f` (`FUN_005c8780.c:102-129`).
3. `0x2f` is `FUN_005b2720` (`FUN_005a96b0.c:155-156`): a unit alive
   (`FUN_005ad870`) with hull above 0.0 clears its orders and returns to
   state 2 (`FUN_005b2720.c:19-29`). A squadron in a hangar (state 1 or 5)
   is not alive by `FUN_005ad870` and is left docked. State 4 runs no further
   logic, so the recalled units keep fighting until the battle ends.
4. A jumping unit reaches state 8 and message `0x2e` (`FUN_005a9ec0.c:160-167`),
   whose handler `FUN_005b26d0` calls `FUN_005a90c0`: state 9, the unit
   leaves the side's lists, and a side controller with `+0x10` set goes to
   state 5 (`FUN_005a90c0.c:5-19`).

**The battle end.** `FUN_005a9ec0` counts per side the units in state 10, 2,
4, 6, 7 or 8 (`:169-173`; states 1, 5 and 9 add nothing). The battle goes on
while both counts are nonzero, and the Empire side with an operational
station never reaches 0 (`:216-228`). Then, once (`+0x22d` 4 to 5):

- side `s` withdrew (`+0x27c` side 0, `+0x27e` side 1) when its controller
  is in state 3, 4 or 5 and no unit of it is alive by `FUN_005ad870` or in
  state 7 or 8 and a unit of it is in state 9 (side 1 also when its station
  is committed, `+0x78`) (`:245-273`). One TIE squadron in state 2 therefore
  keeps its side from counting as withdrawn;
- side `s` lost (`+0x27d` side 1, `+0x27f` side 0) when its count is 0
  (side 1 only while the station is not operational) (`:274-286`); both
  lost clears both (`:287-290`).

The import reads the seven words `+0x27c`-`+0x282` in order:
`FUN_0059a530` copies them from the battle manager's `+0x9f0` to the engine
object's `+0x66c` once the battle is over, and `FUN_005969d0` returns them
through `FUN_0059a730` (`+0x66c`, seven words). So `FUN_0040c780`'s
`local_108[0]`-`[3]` are side 0 withdrew, side 1 lost, side 1 withdrew and
side 0 lost, and `[6]` is `+0x282`, the flag `FUN_005a9ec0.c:32` ends the
battle on at once.

`FUN_0040c780.c:60-69` takes the winner from the lost flags: the tactical
Empire (side 1) lost alone gives winner 1, the Alliance; the tactical
Alliance (side 0) lost alone gives winner 2, the Empire; both lost leaves 0 and the withdrawn flags decide
(`:82-93`): the side that withdrew wins. The Empire's TIEs alive with the
Alliance count at 0 give winner 2, Empire, with the Empire not withdrawn
and the Alliance destroyed. The winner is written to the system (`+0x64`,
`FUN_0050ceb0`, `:105`). The result object's victor and retreat fields
(`+0x50`, `+0x54`) come from the result message's items (`FUN_0049b0f0.c:6-9`),
whose producer is untraced; `FUN_0046f360` picks `0x2a08` for an Empire win
with `+0x54` clear and `0x2a05` for `+0x54` 1.

**Docking at the end.** Windowed battles run `FUN_005bef90` last
(`FUN_005a9ec0.c:335`, vtable `+0x8c`): every squadron alive by
`FUN_005ad870` or in state 1, 5 or 9 docks, in state 1 with the carrier id
at `+0x164`, in the first capital ship of its side in the battle that is
alive, in state 7 or 8, or jumped (state 9) and has hangar room
(`FUN_005b84a0`), else, for side 1, in its station while operational or
committed (`FUN_005bef90.c:72-96`; headless `FUN_005aee90.c:20-47`). A
squadron with a carrier that jumped therefore ends docked in it.

**The import.** `FUN_005b8e00` marks a squadron docked when its carrier id
resolves in the battle and its state is 1 or 5. `FUN_0040d320.c:54-85` makes
a docked squadron a child of its carrier's strategic ship when both have the
same side bits, and every other squadron a child of the system. So the
squadrons that docked in a jumped ship stay with that ship's fleet, and the
ones that found no room remain at the system as system squadrons. Nothing in
the import moves a ship or fleet: `FUN_0040cfc0` writes hull and damage
counters, `FUN_0040d510` moves squadrons beyond a ship's capacity to the
system.

**Who leaves the system.** The only departure after a battle is the
loser's. `FUN_00516ef0` takes the recorded winner (`FUN_00509ad0`) and, for
the other side, flags every fleet and every system squadron with
`FUN_004f78e0(obj, 1)` (`FUN_00516ef0.c:142-180`); event `0x307` then
relocates each through the retreat chain (`system-squadrons-combat.md` 3b).
A winner's ship that jumped in the tactical battle stays in its fleet at the
system. The Empire winner of the case above keeps its Star Destroyer, with
the docked TIE squadrons, at the battle system.

**Where a fleet that withdrew goes.** Two searches run, both over `FUN_005545e0`
(stages in `system-squadrons-combat.md` 3b).

- The tactical retreat system p4 (`FUN_00516730.c:156`, run from
  `FUN_0040b0e0` when the battle session is set up, before any Withdraw)
  passes the fleet's container and stage-1 flag 0, so stage 1 is skipped and
  the search starts at stage 3 (`FUN_00554e80`): the systems of the fleet's
  side whose `+0x88` bit `0x20` (`FUN_0050b8e0`, `blockade-bit.md`) is clear,
  nearest first; then stage 5 (`FUN_005550e0`), the blockaded ones. Fleets
  accept a candidate of any side at the system test (`FUN_005073d0.c:94-113`);
  stage 3 filters on the side bits. The nearness row of a system lists the
  system itself first at distance 0 (`FUN_0058b060.c:19-27`), yet the search
  never ends on the battle system. VM, `empire-day660-deathstar-pre-alert.sav`,
  Imperial player, fresh launch: "Battle at Carida" on day 669 (Imperial-held
  Carida, "The Alliance fleet is threatening Carida. Imperial forces are
  moving to intercept"), Take Command, Withdraw confirmed; the Imperial ship
  stayed 16-23 s, then jumped, and the result read "The Imperial fleet is
  defeated. Carida is now under blockade by Alliance forces. The Imperial
  fleet has withdrawn to Halowan." A second run, "Battle at Coruscant" on day
  676 (the Imperial fleet with the Death Star entered Coruscant), ended "The
  Imperial attack on Coruscant has failed. The Imperial fleet has withdrawn to
  Ando" after about 120 s and the trench-run film. Coruscant was Imperial-held
  before that battle (its window's title bar was the Imperial green on day 670
  and the Alliance red on day 677, after the withdrawal), so this is a second
  own-system withdrawal that ended at another system, with a longer
  countdown. The result text calls it an attack although the Empire held the
  system.
  hyp, mechanism: either `FUN_005571c0` refuses the fleet's own container as a
  candidate (the placement `FUN_00551630` with the source credited by
  `FUN_005515d0`), or bit `0x20` is still set when p4 is picked (Carida was
  blockaded by the Alliance fleet before the Imperial one arrived). Settle by
  breaking on `FUN_005571c0` with the candidate equal to the fleet's container,
  or by reading the system's `+0x88` at `FUN_00516730`.
- The loser pass `FUN_00516ef0.c:142-180` flags each fleet and event `0x307`
  runs `FUN_00515480` with stage 1 enabled (the `FUN_005545e0(.., 0, 1, ..)`
  call): stage 1 (`FUN_00554740`) offers the fleet's own container first, with
  the third argument of `FUN_005571c0` nonzero (the container is already the
  route's source; stages 3 and 4 pass 0 and load it through `FUN_005515d0`).
  The decompile leaves stage 1's container variable unresolved
  (`unaff_EBX`), so the code read does not show it accepting. VM, same save,
  day 711, "Battle at Ando" (Imperial-held, auto-resolved with the alert's
  middle button, so no tactical Withdraw): "The Imperial fleet is defeated.
  Ando is now under blockade by Alliance forces. The Imperial fleet has
  withdrawn to Halowan." A loser at its own held Carida, Coruscant and Ando
  therefore left for another own system by the pass alone: the tactical jump
  moves no fleet (`FUN_0040c780`), and the auto-resolved battle had none. Wine,
  same save, day 669, "Battle at Carida" auto-resolved (no Withdraw): the same
  text ("withdrawn to Halowan"); "Fleet 2 has arrived at Halowan" on day 672
  (a three-day transit) and no "Battle at Carida" reopened by day 676. A loser
  that never withdrew leaves exactly as a withdrawn one does. The
  pass and p4 both reach the nearest other own system, so the system the
  summary names is the one the fleet ends at. hyp: the cause is stage 1
  refusing the own container. A system's `+0x88` bit `0x20` (`FUN_0050b8e0`,
  read by `FUN_005073d0.c:126-141` for the placement) is the other
  candidate, but it is clear while both sides are present and set only after
  the loser has gone, so a refusal by the bit and an unconditional refusal
  cannot be told apart by a loser: the pass runs only when the winner is there.
  The case where stage 1 accepts the container is not found; the port refuses
  it for every loser. A loser with no other own system is not destroyed at
  once: after the own-side stages the chain tries stage 6 (`FUN_005551e0`, the
  systems whose side bits are 3, nearest first) and stage 7 (`FUN_005552e0`,
  the systems of the other side, skipped for a neutral object), each through
  the same candidate test `FUN_005571c0`, and a fleet is accepted at a system
  of any side (`FUN_005073d0.c:103-113`); only when none accepts is it
  destroyed with reason 2 (`FUN_00515480` branch 6). So a one-system side's
  fleet goes to the nearest neutral system, else the nearest enemy-held one.
  `FUN_00516730` runs the same chain from stage 3 (`FUN_005545e0` stages 3 to
  7 run in turn while the status is refused), so the tactical retreat system
  p4 follows it too. Neither stage's loop excludes the container's own system
  (the nearness row lists it first); the refusal that moved the VM's losers out
  of their held systems is the candidate test `FUN_005571c0` they share
  (hyp), so a loser at a neutral battle system leaves it for another neutral
  one. A side holding nothing retreats to the other side's
  nearest system with a countdown from that transit. The port's `place` has
  stages 6 and 7 for a fleet. The VM or Wine check that confirms them: a
  battle lost by a side that holds only the battle system, on a map with
  neutral and enemy systems, reading where its fleet ends.

**Presence and blockade.** A side is present at a system through a usable
fleet or a system squadron (`FUN_00509710`; `FUN_00517270.c:49-78`,
`FUN_00516ef0.c:63-94`); a squadron has no usable-fleet gate (`FUN_004fe540`
gates fleets only). Squadrons alone therefore hold presence: the other side's
fleet arriving finds a battle, and `FUN_00517270` skips the result while both
sides are present. The blockade bit needs an enemy fleet
(`FUN_0050b8e0`, `blockade-bit.md`; `FUN_0050c0b0` copies the bit to fleets
only), so system squadrons alone set no blockade and keep none. The Battle
Summary's system line comes from the holder comparison
(`FUN_0049b330.c:28-52`), not from the bit.

Port:

| Step | Port | Against the original |
|---|---|---|
| Recover order for drive-less groups, countdown, Withdraw executors | `begin_side_withdrawal`, `advance_side_withdrawals` (`tactical.rs:5785`, `:5892`) | matches |
| Recall to state 2 once no drive unit remains | `recall_units_left_behind` (`tactical.rs:5982`) | matches |
| Battle end counts | `side_remains` (`tactical.rs:5292`) counts live ships and live squadrons with craft; docking clears a squadron's `alive` and launching sets it, so a docked squadron counts for nothing | matches `:169-173` |
| Winner | `finish_if_decided`, `decided_winner` (`tactical.rs:5309`, `:5332`): one side at nothing loses; both at nothing, the side that withdrew wins; both withdrawn, the side that did not arrive last (`BattleSession::arrival_holder_is_alliance`); neither, a draw | matches `:274-290` and `FUN_0040c780.c:60-101`; a draw records no winner and `post_battle::pass_winner` takes the holder as `FUN_00516ef0.c:95-112` does; holder 3 (winner 3) has no counterpart |
| Outcome voice | `queue_battle_outcome_voice` (`tactical.rs:8017`) from the lost and withdrawn flags | matches `FUN_005a9ec0.c:291-328` |
| Withdrawn | `side_withdrew` (`tactical.rs:6317`) | matches `:245-273` |
| Docking at the end | `end_berths` (`tactical.rs:6545`) | matches `FUN_005bef90.c:72-96` |
| Squadron write-back | `squadron_homes`, `apply_results` (`apply.rs:117`, `:292`) | matches `FUN_0040d320.c:54-85` |
| Ships and fleets at the write-back | `apply_results` moves none; a jumped ship keeps its hull and its fleet | matches `FUN_0040c780` |
| Loser's relocation | `post_battle::run_pass` (`post_battle.rs:93-183`), which returns each loser fleet's outcome; the results name the player's fleet's destination from it | matches `FUN_00516ef0.c:142-180` |
| Summary text and picture | `battle_result_summary_lines`, `battle_result_picture` (`tactical_view.rs:5352`, `:5453`) | match for the case above: Empire victorious, Alliance destroyed, `0x2a08` |

## Open

- The order in which a windowed battle registers each side's units, and so
  their object ids; the port numbers a side's ships, then its squadrons, in
  battle order (`original_object_id`, hyp).
- Slot `+0x418` and component `+0x5c` of the computer player.
- What the Battle Alert's Retreat writes.
- `FUN_005b1b70` counts a unit's hyperdrive as working only when its drive
  rating (`FUN_005b1bc0`) and its slot `+0x90` are both nonzero; slot
  `+0x90` of a capital ship is untraced, and the port's
  `ship_hyperdrive_works` tests the hyperdrive subsystem alone.
- The headless end (`FUN_005ae9c0`, `0x5aed43`-`0x5aee68`) counts only units
  in state 2 and withholds a side's loss while the other side has withdrawn
  (`0x5aedc1`, `0x5aee4f`); the port's headless step ends by the windowed
  rule (`finish_if_decided`), and its stalemate ending
  (`advance_stalemate_watch`) gives a draw when both sides leave.

## Decompiles

`FUN_005d19a0.c`, `FUN_005d14d0.c`, `FUN_005d0f50.c`, `FUN_005d0ce0.c`,
`FUN_005c7e40.c`, `FUN_005afa30.c`, `FUN_005afb30.c`, `FUN_005afb50.c`,
`FUN_005c7cb0.c`, `FUN_005ae9c0.c`, `FUN_00599c90.c`, `FUN_00595d60.c`,
`FUN_0040b0e0.c`, `FUN_0040a050.c`, `FUN_0041e1f0.c`, `FUN_0051dce0.c`,
`FUN_00516730.c`, `FUN_004362b0.c`, `FUN_00488600.c`, `FUN_00485d50.c`,
`FUN_004865b0.c`, `FUN_004bf300.c`, `FUN_004bf7c0.c`, `FUN_004e6110.c`,
`FUN_004e6a60.c`, `FUN_004e77f0.c`, `FUN_004e87a0.c`, `FUN_004e94f0.c`,
`FUN_004e9ec0.c`, `FUN_004eabb0.c`, `FUN_00433620.c`, `FUN_005ad530.c`,
`FUN_005c8780.c`, `FUN_005a8ad0.c`, `FUN_005a8b10.c`, `FUN_005b1e40.c`,
`FUN_005b2720.c`, `FUN_005a6b40.c`, `FUN_005c6610.c`, `FUN_005cc300.c`,
`FUN_005ca6d0.c`, `FUN_005a7490.c`, `FUN_005a6290.c`, `FUN_005a6620.c`,
`FUN_005a6640.c`, `FUN_005a9610.c`, `FUN_005c7e80.c`, `FUN_005c7f70.c`, `FUN_005d0ae0.c`,
`FUN_005ba980.c`, `FUN_005ca530.c`, `FUN_005caf50.c`, `FUN_005cfec0.c`,
`FUN_005be290.c`, `FUN_005ad4a0.c`, `FUN_0055d8a0.c`, `FUN_005bae60.c`,
`FUN_005b26d0.c`, `FUN_005a90c0.c`, `FUN_005a9ec0.c`, `FUN_005bef90.c`,
`FUN_005b84a0.c`, `FUN_005b8e00.c`, `FUN_0040d320.c`, `FUN_00516ef0.c`,
`FUN_0040c780.c`, `FUN_0059a530.c`, `FUN_005969d0.c`, `FUN_0059a730.c`.
