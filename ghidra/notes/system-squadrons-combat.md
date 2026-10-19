# System squadrons in combat

Recovered from REBEXE.EXE with Homebrew Ghidra 12.1.3 (read-only, no
analysis). Companion to `simulate-runs-tactical-engine.md`,
`battle-result-text-and-cadence.md`, `blockade-bit.md` and
`battle-alert-lists.md`. The port's side is
`crates/rebellion-core/src/combat.rs` and
`crates/rebellion-core/src/tactical.rs`.

A system can hold fighter squadrons (families `0x1c`..`0x1f`) as its own
children, not aboard any ship. This note traces how those squadrons start a
battle, fight in it and leave it.

## 1. Battle trigger

### The presence test (`FUN_00509710`)

A side is **present** at a system when it has:

- an active fleet (`FUN_004ffef0`, families `0x08`..`0x0f`, mode 1 of
  `FUN_004f6b90`), **OR**
- an active fighter squadron (`FUN_00503a50`, families `0x1c`..`0x1f`,
  mode 1).

Either alone is enough. `FUN_005131b0` returns whether the resulting
iterator is non-empty.

```c
// FUN_00509710.c
pvVar3 = FUN_004ffef0(local_2c, this, 1, param_1);  // fleets of side
bVar1 = FUN_005131b0((int)pvVar3);
pvVar3 = FUN_00503a50(local_2c, this, 1, param_1);  // squadrons of side
bVar2 = FUN_005131b0((int)pvVar3);
if (bVar1 == 0 && bVar2 == 0) return 0;  // not present
return 1;
```

### The battle bit (`FUN_0050b8e0`)

For a system with a side (`+0x24` bits 6-7 nonzero), `FUN_0050b8e0` sets
the system's `+0x88` bit `0x1000` (battle pending) when
`FUN_00509710(system, 1)` and `FUN_00509710(system, 2)` both return
non-zero. System squadrons count: an enemy fleet arriving at a system whose
only defenders are system-held squadrons sets the bit.

### Defense facilities

Defense facilities (families `0x20`..`0x2f`) do not count toward presence.
`FUN_00509710` checks only fleets (`0x08`..`0x0f`) and squadrons
(`0x1c`..`0x1f`). A system with only shields and batteries does not start a
battle.

## 2. The battle

The battle session `FUN_0040a050` finds the first system with bit `0x1000`
and runs the tactical engine on it, windowed or headless
(`simulate-runs-tactical-engine.md`). Nothing else resolves a space battle.

### Export (`FUN_0040b0e0`)

States 7 and 8 call `FUN_0040b0e0` before starting the engine
(`FUN_0040a050.c:124, 159`). It walks each fleet at the system, each ship
(`FUN_00502db0`) and the squadrons aboard it (`FUN_005039d0`, record
`FUN_0040be90`), then the squadrons the system itself holds
(`FUN_005039d0(system, 1)`, `FUN_0040b0e0.c:274-280`), with the same record
builder `FUN_0040be90`. System squadrons enter the battle after every
fleet's units and need no carrier. The Battle Alert lists them in the same
order: last on each side's page, under the system's name (`FUN_00450770`,
`battle-alert-lists.md`).

### In the engine

Squadrons fight as any tactical unit: targets, damage rates and shields
follow `tactical-engagement-ai.md` and `tactical-fighter-combat.md`. When a
side retreats, units with a working hyperdrive leave, the rest dock in an
escaping carrier (`FUN_005afe40`) or are destroyed (`FUN_005af7a0`)
(`tactical-engagement-ai.md`, step 3). hyp: a system squadron has no
hyperdrive and no carrier of its own, so it docks in an escaping carrier of
its side or is destroyed.

### Import and losses (`FUN_0040c780`, `FUN_0040d260`)

States `0x16` / `0x17` import the outcome through `FUN_0040c780`. It walks
the same objects in the same order: per fleet, per ship, the squadrons
aboard (`FUN_0040c780.c:130-136`), then the system's own squadrons
(`FUN_005039d0(system, 1)`, `:171-177`), each through `FUN_0040d260`.

`FUN_0040d260` reads the squadron's tactical record (`FUN_005968e0`) and
compares its surviving strength with the strategic one (slots `+0x244`
maximum and `+0x254`):

- a gain calls `FUN_00503400` (not read);
- a loss calls `FUN_00503370(squadron, loss, 0xc, ctx)`.

`FUN_00503370` adds the loss to the squad-size damage at `+0x60`, capped at
the maximum (slot `+0x244`), through `FUN_005032c0`, the
`SquadSizeDamage` setter (it writes `+0x60`, updates the Alliance and
Empire views through `FUN_00539fd0(this, 1/2)` and fires slot `+0x260`).
When slot `+0x254` then equals the maximum, it calls slot `+0xac` with
reason `0xc`, which destroys the squadron. Nothing distinguishes system-held
squadrons from carried ones.

## 3. After the battle (`FUN_0051ddf0`)

After the import the session goes to state 9 (`FUN_0040a050.c:251, 268`),
which runs `FUN_0041e270` → `FUN_0051ddf0` → `FUN_00516ef0` then
`FUN_00517270` for the battle system (`battle-result-text-and-cadence.md`).

### `FUN_00516ef0`

- `FUN_00509ad0(system)` returns `+0x54` → `+0x64` of the system, or 0.
  The import `FUN_0040c780` writes the battle's winner there through
  `FUN_0050ceb0(system, winner)` (0..3), so the field is the last battle's
  winner.
- When it is 0 and both sides are present (fleets or squadrons,
  `FUN_00516ef0.c:63-94`), the side is taken from the system's `+0x78`
  bits 6-7: holder 3 gives 3, holder 1 gives 2, holders 2 and 0 give 1.
  `FUN_00508660`, the system's hook for an arriving child, writes the
  arriving fleet's or squadron's side there through `FUN_0050a1b0`, so the
  holder is the side that arrived last and the tie goes to the other side.
  A nonzero `FUN_00509ad0` is used as the side directly.
- The chosen side is the winner; the other side (`local_60`) is the
  loser.
- For that side's system squadrons (`FUN_00503a50`, `:123-137`) it calls
  `FUN_00556710` per squadron.
- For the other side (`local_60`) it calls `FUN_004f78e0(obj, 1, ctx)` on
  every fleet and every system squadron (`:142-178`), which sets `+0x50`
  bit `0x1000` (`GameObjAutoscrapRequestNotif` in `build-delivery.md`) and
  updates both side views.

Before the loop `FUN_00516ef0` calls `FUN_0050a580(system, 0)`, which
clears the system's `+0x88` bit `0x10` and notifies both side views.

### `FUN_00556710`: the winner's squadron check

`FUN_00556710(squadron, ctx)` moves nothing and fights nothing. It
re-validates a squadron's seat in its container in three stages, each run
only while the squadron is not destroyed (`+0x50` bit 3) and not flagged
(`+0x50` bit `0x1000`). Slots are the fighter-squadron vtable `0x0065def0`.

1. Container (`FUN_004f6b50`, the `+0x1c` parent) is destroyed: slot `+0xb4`
   `FUN_004f8710(0, ctx)`. A completed squadron not en route
   (`+0x50` bit 2 set, bit `0x20` clear) is destroyed through `FUN_004f7ed0`
   with the container's reason byte, else it is flagged `0x1000`.
2. The container's slot `+0x54` (a system: `FUN_005072d0`, then the base
   `FUN_004f6490`) answers "may this child sit here" into a two-word status
   record `{1, code}` (`FUN_00520580`). Code `-1` accepts. The base accepts
   when `(child +0x24 ^ container +0x24) & 0xc0 == 0`, that is, the same
   side, and otherwise answers `0x24`. `FUN_005072d0` also accepts a
   regiment (family `0x10..0x13`) at an unpopulated system of side 3
   (`+0x88` bit 0 clear). A refusal calls slot `+0xb0` `FUN_00503510`:
   flag `0x1000` unless destroyed.
3. A route context (`FUN_00551060`) is built for the squadron's side with
   kind `0x3f0` (`FUN_00551100(ctx, side, side, 0x3f0, {1})`), loaded with
   the container (`FUN_00551270`), and the squadron is placed into that
   container (`FUN_00551c80`, the move-order leg builder of `move-order.md`
   and `regiment-unload.md`). Status code `-1` accepts. Kind `0x3f0` lifts
   the blockade refusal (`FUN_005073d0`: a blockaded system, `+0x88` bit
   `0x20`, gives `0x90`/4 and `0x3f0` resets it to accept). A refusal with
   a leg recorded calls slot `+0xb8` `FUN_004f8760`: a completed squadron
   not en route is destroyed with reason 4 (`+0xac(4)`), else it is flagged.
4. When it is still valid and the squadron is en route (`+0x50` bit
   `0x10`) or its origin container (`+0x38`) lies in a different system than
   its container, and the container system is blockaded (`+0x88` bit
   `0x20`) by a side (`FUN_00509890`) other than the squadron's, and the
   squadron is not a fleet or ship family, it calls `+0xb8` again.

There is no destination search. A squadron the system holds for its own
side passes stages 1 to 3 untouched. hyp: whether the `iStack_34 != -1`
guard in stage 3 (route object field at `+0x18`) is set by a refusal is not
traced, so a stage-3 refusal may skip `+0xb8`.

### `FUN_00517270`

`FUN_00517270` rechecks presence the same way:

| Alliance present | Empire present | Result |
|---|---|---|
| yes | yes | no winner; `FUN_0050a040` and `FUN_00517ee0` are skipped and the battle bit stays, so the system fights again on the next pass |
| yes | no | winner 1: `FUN_0050a040(system, 1)`, `FUN_00517ee0(system, 1, …)` |
| no | yes | winner 2: the same with 2 |
| no | no | `FUN_0050a040(system, 0)`; no result object |

`FUN_0050a040(system, w)` writes the low two bits of the system's `+0x78`
(the last battle's result, 0 none, 1 Alliance, 2 Empire), notifies both
views and calls slot `+0x1f0`. It runs only when at most one side is
present. `FUN_00517ee0(system, w, FUN_00509ae0(system, 1),
FUN_00509ae0(system, 2))` runs when one side remains. `FUN_00517270` does
not change the system's side, move any object or clear any `0x1000` flag;
it re-reads presence after `FUN_00516ef0` and the flagged objects' handler
(next section). It then calls the character manager's
`FUN_00560d00(system, winner)`. A side left with only system squadrons is
still present, so the system stays contested.

hyp: `FUN_00517ee0` and `FUN_00509ae0` (per-side figures for the result
record) are not read here.

## 3b. Flagged objects: event `0x307` and `FUN_00515480`

`FUN_00516ef0` calls `FUN_004f78e0(obj, 1)` on every fleet and system
squadron of the losing side. It sets `+0x50` bit `0x1000` and calls slot
`+0x13c(1, ctx)`. A fleet's `FUN_004ff690` and a squadron's `FUN_004f2bd0`
(`FUN_004fba70`) both raise event `0x307` through `FUN_0053f9c0(0x307, obj,
flag, ctx)`: `FUN_0053f950` builds the event (`FUN_0053ff90`, keys at
`+0x3c` object and `+0x50` flag) and hands it to the dispatcher callback
`DAT_006b2af8` (`FUN_004fd390`), which is `FUN_004fd010` and inserts the event
into the event queue `+0xb0` (`FUN_0051d8f0`). The event is queued, not run
inside the flagging call: its priority is `0x300` (`FUN_0057b260`), so it is
served in the same frame step after the queued events of lower priority
(`blockade-bit.md`, "The event queue and the order of work"). The handler
`FUN_0057b1f0` calls `FUN_00515480(obj id, flag, ctx)`.

The flag's other names: `GameObjAutoscrapRequestNotif` (`build-delivery.md`).
The handler serves any object whose container no longer suits it, with the
battle as one source. The player's Scrap order uses reason `0x14`, maintenance
autoscrap `0x15` (`FUN_004f8850`).

### Setting the reason: `+0xac(reason, ctx)`

Slot `+0xac` is `FUN_004f8660` -> `FUN_004f71d0`: it writes the destruction
reason byte `+0x40 & 0xff`, notifies both views and calls slot `+0x100`
(`FUN_004fb200` -> `FUN_004f7560`), which sets `+0x50` bit 3 (destroyed) when
the new reason is nonzero. So `+0xac(2)` destroys the object with reason 2
and `+0xac(5)` with reason 5. Reasons read: 2 no accepting place found
(`FUN_00515480`), 3 control change (`FUN_004f8680`), 4 removal
(`FUN_004f8760`), 5 cannot travel (`FUN_00515480`, `FUN_004ff690`), `0xb`
disaster, `0xc` squadron loss, `0xf`/`0x10` Destroy System, `0x14` scrap,
`0x15` autoscrap. hyp: the player-facing text of reasons 2 and 5 is not
traced.

### `FUN_00515480(obj, flag, ctx)` branch by branch

1. Object missing (`FUN_00504dc0`) or destroyed (`+0x50` bit 3): return
   true, nothing else.
2. A character (`+0x14` slot `+4` type in `0x30..0x3f`) that is completed
   (`+0x50` bit 2): the story hook `FUN_00545df0` on manager `FUN_00506e40`
   (`root +0xb8`); no relocation.
3. `flag == 0` (a clear): return true. Nothing runs on a clear.
4. Otherwise the relocation manager is `FUN_00506e60` (`root +0xbc`, the
   holder of the nearness table). A hint id `H` (null by default) is set:
   - not completed (bit 2 clear): `H` = the system holding the object's
     origin object (`+0x38`) when it differs from the system holding the
     object, else null;
   - completed and en route (bit `0x20`): `H` = the transit origin
     (`FUN_004f6e30`: the id at `+0x54 -> +0xc`) unless it equals the
     destination (`FUN_00540450`: `+0x1c`), in which case `H` is cleared and
     the origin id is nulled (`FUN_004f7fa0`);
   - completed and not en route: `H` stays null.
5. When the container (`FUN_0050c640`) is a system (`0x90..0x97`) and the
   manager exists, the retreat-candidate chain
   `FUN_005545e0(mgr, obj, containerId, &H, 0, 1, &D, &status)` runs.
   Status is a `{1, code}` record: `-1` accepted, anything else refused;
   it starts refused. `D` receives the chosen destination id.
6. Status still refused: `+0xac(2, ctx)`: the object is destroyed with
   reason 2. No retreat, no move.
7. Status accepted and the object is completed and not destroyed:
   `FUN_00555410(obj, D, &same)` sets `same = 1` unless the object's
   containing system differs from `D`'s containing system (an unresolved
   id leaves 1). With `same == 0` and speed zero (slot `+0x34(inTransit)`)
   the object gets `+0xac(5, ctx)`. A fleet's speed is `FUN_004fd900`,
   the slowest member (any member at 0 gives 0); a ship's `FUN_00500820`,
   effective hyperdrive (`+0x1ec - +0x210`, then `+0x1f0 - +0x214`), else
   the base `FUN_004f63f0`; a squadron's `FUN_00502f80` likewise, ending in
   `FUN_004f63f0`, which is `DAT_006b9050` (100) while the object exists.
8. The relocation `FUN_00556390(obj, D, ctx)`: when slot `+0x38` says the
   object may travel (`FUN_004fda10` for a fleet, `FUN_004f6410` bit 16
   otherwise) and bit `0x800` is clear it computes `same` again and sets the
   en-route-active bit `0x20` to `same == 0` (`FUN_004f7640`); then, if not
   destroyed, slot `+0xa8(D, ctx)`: a ship or squadron `FUN_004f8630`, a
   fleet `FUN_004feca0`, both reparent through `FUN_004f6fd0`. A changed
   container calls slot `+0xf4(old, new, ctx)` (`FUN_004faf50` for fleets,
   ships and squadrons) which first clears bit `0x1000`
   (`FUN_004f78e0(0)`) and starts the transit event `0x320`. A non-travelling
   object goes straight to `+0xa8`.
9. A fleet's slot `+0x13c` (`FUN_004ff690`) runs after the event: every
   completed ship of the fleet that is not en route and whose speed is 0
   gets `+0xac(5)`. It runs on a clear as well as a set.

### The retreat-candidate chain (`FUN_005545e0`)

Each stage runs only while status is refused, and each candidate is judged
by `FUN_005571c0(obj, candidate, skipCurrent, &D, &status)`, which asks
whether the object could be sent there as a move: route context kind
`0x204` (`0x241` for a character, `FUN_005511d0`), loaded with the
candidate (`FUN_005513a0`) and placed by `FUN_00551630` (the same checks a
Move order makes, `move-order.md`). The first accepted candidate is `D`.

| Stage | Function | Candidates, in order |
|---|---|---|
| 1 | `FUN_00554740` | the object's parent, grandparent and great-grandparent when each is a system, fleet or ship (the object stays where it is when its container still accepts it); skipped when the container is a `0x98..0x9f` object |
| 2 | `FUN_00554ab0` | the hint `H`, then its parent and grandparent (fleet, ship, system) |
| 3 | `FUN_00554e80` | systems of the object's side that are not blockaded (`+0x88` bit `0x20` clear), nearest first |
| 4 | `FUN_00554f80` | the object's side's fleets (`FUN_004ffef0` mode 3) at the nearest systems, in the same order |
| 5 | `FUN_005550e0` | systems of the object's side that are blockaded |
| 6 | `FUN_005551e0` | systems with side bits `3` (neutral) |
| 7 | `FUN_005552e0` | skipped for a side-3 object; systems of the other side |

"Nearest first" is the nearness table at manager `+0x68` (`FUN_00554150` ->
`FUN_0058afd0`, row `(id - 100) * 0x324`, 200 systems sorted by distance,
`galaxy-limits.md`, `build-delivery.md`). Stage 3 to 7 candidates are
tried one by one until one is accepted. A candidate system's acceptance is
`FUN_005073d0` over the type range of the moved object (route kind `0x204`;
`0x241` for characters and special forces):

- **Fleets** (`0x204`, range inside `[0x08,0x10]`) are exempt from the
  destroyed refusal (`FUN_005073d0.c:94-102`), and every refusal
  `FUN_00553410` records (another side `0x24`/`0x28`, not completed `0x20`,
  destroyed `0x22`, en route `0x21`) is reset to accepted
  (`FUN_005073d0.c:103-113`), as is the blockade refusal (`:126-141`). A
  system of any side accepts a fleet, so stage 6 (neutral) and stage 7
  (the other side) accept one whenever they are reached.
- **Regiments** (`0x204`, `[0x10,0x14]`) keep every refusal, with one
  exception: a system of side bits 3 with `+0x88` bit 1 set and bit 0 clear
  accepts (`:114-123`, `regiment-unload.md`). The blockade refusal stands for
  them: the lift list is fleets, kind `0x240`, kind `0x241` and `0x3f0`
  (`:134`).
- **Squadrons, capital ships, facilities and any other `0x204` kind** keep
  every refusal, so only a completed, undestroyed, idle system of their own
  side accepts, and a blockaded system refuses.
- **Characters and special forces** (`0x241`): the destroyed and
  `FUN_00553410` refusals stand and the blockade refusal is lifted; kind
  `0x240` (a mission order) lifts all of them like a fleet.

A fleet container (stage 4) accepts a kind only inside `[0x14,0x1c]`
(`FUN_004fe160`), so only a capital ship joins a fleet there; squadrons,
regiments and characters board a ship of it, and facilities find no place at
stage 4.

### What a flagged object finally does

- Mobile, a destination accepted in another system, speed above 0: it
  moves there at once (reparent to the system or carrier `D`) and travels
  as a normal move; the move clears the flag.
- A destination in the same system (stage 1 accepts its own container): no
  container change, so nothing in the handler clears bit `0x1000`. hyp: it
  stays set until the next container change. Readers of the bit:
  `FUN_00556710` skips a flagged squadron in all stages, the object
  invariant `FUN_004f9700` fails while it is set, and six story gates test
  it. Settle by reading the bit after a battle.
- Destination in another system and speed 0: destroyed, reason 5. A fleet
  member ship with speed 0 is destroyed with reason 5 whatever the
  fleet does.
- No candidate accepted: destroyed, reason 2.
- Retreat to a "chosen system" is therefore the first accepted candidate of
  the chain above, not a tactical retreat; `FUN_00516730` (the battle
  retreat options) reuses the same chain (`build-delivery.md`).

The flag clears in `FUN_004fb520` (arrival/slot `+0x11c`, also clears
autorouting and en route active) and `FUN_004faf50` (container change,
first statement). `FUN_004faf50(from, to)` also sets en route active
through event `0x320`.

### Sides and ties

- Winner side (`FUN_00509ad0`, else the holder rule): `FUN_00556710` on
  its system squadrons only; fleets are not touched.
- Loser side: every fleet and every system squadron flagged, then handled
  by `FUN_00515480` above. A loser with no objects flags nothing.
- No winner: `FUN_00509ad0` 0 and not both present gives no winner and no
  calls. Both present gives the holder rule, so a battle that ends with
  both sides present still names a winner and flags the other side's
  objects; `FUN_00517270` independently reports no result and the battle
  bit stays.

## 4. Not part of the battle

`FUN_00514a60`, run by the event-`0x320` fire slot `FUN_0057ba50` on every
change of container (`FUN_00514a60(+0x3c object, +0x4c from, +0x50 to,
ctx)`), applies no damage. Its hooks are the story-check chain, the
CommandKind hook, the producing manager's hook `FUN_00529d20` and the
travel start (`COMBAT-SUMMARY.md`).

## Still open

- Whether a flagged object that stays in its own system keeps bit `0x1000`.
- The unread half of system acceptance for fleets and squadrons at enemy
  systems (`FUN_005073d0` per family) and `FUN_00517ee0`.
- Where the engine places exported system squadrons (coordinates,
  formation); `FUN_0040be90` builds the record and the placement is inside
  the tactical setup (`FUN_005a7500` and its handlers).
- `FUN_00503400`, the import's gain path.

## Port

- `rebellion_core::combat::side_present` is `FUN_00509710`: a fleet of the
  side at the system, or squadrons the system holds for it. Both the
  headless trigger (`rebellion-data/src/simulation.rs`) and the live one
  (`main.rs`) start a battle when both sides are present (`FUN_0050b8e0`).
- `CombatForce` is one side: its lead fleet, if any, and its system
  squadrons. The tactical session takes the sides
  (`BattleSession::new_with_forces`) and seats the system squadrons after
  the fleets' units (`FUN_0040b0e0`) as `TacticalFighter`s with
  `held_by_system`; they launch and return without a carrier, and
  `tactical::apply::apply_results` writes their survivors back to
  `System::fighters` (`FUN_0040d260` → `FUN_00503370`).
- `combat::fighting_fleets` is a side's fleets that fight: the usable ones
  (`Fleet::usable`, `FUN_004fe540`) in container order. `FUN_0040b0e0`
  walks every existing fleet (`FUN_004ffe70`, mode 3) but exports only
  each one's usable children (mode 1), and the Commander field
  (`FUN_00509530`) and the post-battle pass (`FUN_00516ef0`) walk the
  side's fleets in mode 1 (`FUN_004ffef0`), so a fleet whose only ship is
  on its own leg neither fights, commands nor takes the post-battle chain.
  port: the
  original has no lead fleet; the live trigger (`main.rs`), the headless
  battle (`resolve_system_space_combat_for`) and the session's fleet
  gather (`BattleSession::new_with_forces`) take the first of these.
- `resolve_system_space_combat` runs that session headless, and it and a
  played battle's record report the winner by presence after the pass
  (`integrator::post_battle_outcome`, `FUN_00517270.c:79-93`, which hands
  that side to `FUN_0050a040` and `FUN_00517ee0`; the battle's own winner,
  `FUN_0040c780`'s `+0x64`, is read only by `FUN_00516ef0` and
  `FUN_0050c6c0`): a side left with only its system squadrons keeps the
  system contested. A win by squadrons alone has
  no winning fleet, so no bombardment or landing follows.
  hyp: they start on their side's fighter line; the original placement is
  untraced (above).
- `rebellion_core::post_battle` is the pass (`FUN_00516ef0`), run at the
  end of `tactical::apply::apply_results`, which every played, simulated
  and unwatched battle returns through. `pass_winner` takes the battle's
  winner, else, with both sides present, the side that did not arrive last
  (`GameWorld::arrival_holders`, written on each fleet arrival and each
  squadron arrival on a planet). `run_pass` checks the winner's system
  squadrons (`FUN_00556710`: a destroyed system takes them, a system of the
  other side sends them through the chain) and sends every fleet and
  system squadron of the loser through the chain (`FUN_00515480`): stay
  when the battle system is theirs (a squadron also needs it unblockaded),
  else the nearest unblockaded system of their side, else (a squadron) the
  nearest fleet of their side with room, else (a fleet) the nearest
  blockaded system of their side; none destroys the object (reason 2), and
  a fleet that must travel at speed 0 is destroyed (reason 5). A fleet that
  stays loses each ship that cannot travel (`FUN_004ff690`). A fleet that
  withdrew from the battle goes where the chain without its own system
  sends it (`retreat_destination`).
- port: `resolve_system_space_combat` seats the Alliance as the session's
  attacker, so `CombatSide::Attacker` means Alliance there.
- port: stage 4 of `FUN_00556710` (an en-route squadron, or one whose
  origin lies elsewhere) has no counterpart: the port keeps no origin for a
  squadron the system holds.
- hyp: the chain's stages 6 and 7 accept nothing, the characters aboard a
  destroyed fleet are captured, a flagged object that stays keeps no flag
  (the port keeps none), and a withdrawal skips the current system.
- `FUN_00517270`'s presence check reports the result; its `+0x78` low bits
  are not kept.
