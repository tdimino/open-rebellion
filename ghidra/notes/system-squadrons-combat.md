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

- `FUN_00509ad0(system)` returns `+0x54` → `+0x64` of the system, or 0
  (hyp: the side of a pending battle record).
- When it is 0 and both sides are present (fleets or squadrons,
  `FUN_00516ef0.c:63-94`), the side is taken from the system's `+0x78`
  bits 6-7: holder 1 gives 2, holder 2 gives 1, 3 gives 3. A nonzero
  `FUN_00509ad0` is used as the side directly.
- For that side's system squadrons (`FUN_00503a50`, `:123-137`) it calls
  `FUN_00556710` per squadron.
- For the other side (`local_60`) it calls `FUN_004f78e0(obj, 1, ctx)` on
  every fleet and every system squadron (`:142-178`), which sets `+0x50`
  bit `0x1000` (`GameObjAutoscrapRequestNotif` in `build-delivery.md`) and
  updates both side views.

`FUN_00556710` takes no combat action. For a squadron that is not
destroyed and not flagged `0x1000` it calls slot `+0xb4(0, ctx)` when the
squadron's container is destroyed, asks the container (slot `+0x54`) and
calls slot `+0xb0`, builds a route context of kind `0x3f0` for the
squadron's side (`FUN_00551100`, `FUN_00551270`, then `FUN_00551c80`, the
route and room helpers of `move-order.md` and `regiment-unload.md`) and
calls slot `+0xb8`. It calls `+0xb8` again when the squadron is en route
(`+0x50` bit `0x10`) or its container changed and the containing system
(`FUN_00555540`, `0x90`..`0x97`) is blockaded (`+0x88` bit `0x20`) by the
other side (`FUN_00509890`). hyp: it re-homes system squadrons after the
battle; the three slots are not resolved.

### `FUN_00517270`

`FUN_00517270` rechecks presence the same way:

| Alliance present | Empire present | Result |
|---|---|---|
| yes | yes | no winner; `FUN_0050a040` and `FUN_00517ee0` are skipped and the battle bit stays, so the system fights again on the next pass |
| yes | no | winner 1: `FUN_0050a040(system, 1)`, `FUN_00517ee0(system, 1, …)` |
| no | yes | winner 2: the same with 2 |
| no | no | `FUN_0050a040(system, 0)`; no result object |

It then calls the character manager's `FUN_00560d00(system, winner)`. A
side left with only system squadrons is still present, so the system stays
contested.

## 4. Not part of the battle

`FUN_00514a60`, run by the event-`0x320` fire slot `FUN_0057ba50` on every
change of container (`FUN_00514a60(+0x3c object, +0x4c from, +0x50 to,
ctx)`), applies no damage. Its hooks are the story-check chain, the
CommandKind hook, the producing manager's hook `FUN_00529d20` and the
travel start (`COMBAT-SUMMARY.md`).

## Still open

- The meaning of system `+0x54` and its `+0x64` field read by
  `FUN_00509ad0`.
- Slots `+0xb0`, `+0xb4`, `+0xb8` of the fighter-squadron class, called by
  `FUN_00556710`, and why `FUN_00516ef0` flags the other side's forces with
  `+0x50` bit `0x1000`.
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
- `resolve_system_space_combat` runs that session headless and reports the
  winner by presence afterwards (`FUN_00517270`): a side left with only its
  system squadrons keeps the system contested. A win by squadrons alone has
  no winning fleet, so no bombardment or landing follows.
  hyp: they start on their side's fighter line; the original placement is
  untraced (above).
- port: `FUN_00516ef0`'s post-battle pass (`FUN_00556710`, the `0x1000`
  flag) has no counterpart.
