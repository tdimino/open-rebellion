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
committed (`+0x78`), which removes it from the side's remaining forces. Ships
marked `retreated` become a `withdraw_fleet` in `tactical/apply.rs`, which
also sends a Death Star's fleet along.

`tactical::apply::seat_morale` sets both fields before a battle:

- permission is `post_battle::retreat_destination` finding a place for the
  side's fleet (port: stands in for `FUN_00516730`);
- the threshold is `ai_records::battle_withdraw_threshold` (`FUN_004e6110`)
  for a computer side and 0.0 for the human side.

`resolve_system_space_combat` seats both sides as computer players;
`resolve_system_space_combat_for` seats the person's side as human and is
what the game's unwatched battles call. `tactical_flow::begin_player_battle`
seats the player's side as human, so the Battle Alert's Simulate reads them.
The played (windowed) step does not run the check. Further rules of the port:

- An operational Empire Death Star adds `_DAT_0066d0f0` = 20000000.0 (read
  from `REBEXE.EXE`) to the side's strength (`FUN_005d0ce0.c:80-85`). The
  Alliance's trench-run term (`:86-103`) is not modeled.
- The Death Star is a way out only when its fleet can enter hyperspace
  (`fleet_speed` reads capital ships); a Death Star-only fleet cannot depart
  in the port, so it never counts as withdrawn. For an object that cannot
  travel the source destroys it, reason 5 (`FUN_00556390`).
- `death_star_withdrew` holds only when the Death Star was operational at the
  retreat (`FUN_005d14d0.c:124-127`).
- A side counts as withdrawn (results text, voice) when its latch is set and
  a unit jumped or its Death Star was committed (`FUN_005ae9c0.c:117-141`),
  or a ship left on the player's own Retreat.
- The stalemate loser goes through the same permission test; a side that
  may not retreat is destroyed (`FUN_005ae9c0.c:62-103`).
- A squadron docks in its fleet's escaping carriers first, then any escaping
  carrier of the side (`FUN_005afe40.c:17-58`; port: per fleet, not per
  ship; the Death Star's hangar is not modeled). Where a jumped squadron
  goes strategically is untraced (hyp: it stays where the world holds it).
- `+0xac` of the AI's system record counts the active squadrons present, any
  side (`FUN_00433e40.c:564-583`); the port counts every squadron there.
- With one AI, the faction it does not play is the human side (threshold 0.0).

Not modeled: the delay `+0x7c`.

## Open

- Whether a played (windowed) battle runs this check; `FUN_00599c90` sets
  `+0x24` and `+0x80` on the windowed sides as well.
- The reader of side `+0x7c`.
- Slot `+0x418` and component `+0x5c` of the computer player.
- What the Battle Alert's Retreat writes.

## Decompiles

`FUN_005d19a0.c`, `FUN_005d14d0.c`, `FUN_005d0f50.c`, `FUN_005d0ce0.c`,
`FUN_005c7e40.c`, `FUN_005afa30.c`, `FUN_005afb30.c`, `FUN_005afb50.c`,
`FUN_005c7cb0.c`, `FUN_005ae9c0.c`, `FUN_00599c90.c`, `FUN_00595d60.c`,
`FUN_0040b0e0.c`, `FUN_0040a050.c`, `FUN_0041e1f0.c`, `FUN_0051dce0.c`,
`FUN_00516730.c`, `FUN_004362b0.c`, `FUN_00488600.c`, `FUN_00485d50.c`,
`FUN_004865b0.c`, `FUN_004bf300.c`, `FUN_004bf7c0.c`, `FUN_004e6110.c`,
`FUN_004e6a60.c`, `FUN_004e77f0.c`, `FUN_004e87a0.c`, `FUN_004e94f0.c`,
`FUN_004e9ec0.c`, `FUN_004eabb0.c`, `FUN_00433620.c`.
