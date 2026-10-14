# System squadrons in combat

Recovered 2026-10-09 from REBEXE.EXE with Homebrew Ghidra 12.1.3 (read-only,
no analysis). Companion to `space-combat.md`, `blockade-bit.md`,
`battle-alert-lists.md`. The port's side is
`crates/rebellion-core/src/combat.rs` and
`crates/rebellion-core/src/tactical.rs`.

A system can hold fighter squadrons (type 0x1c) as its own children, not
aboard any ship.  This note traces how those squadrons enter, fight and
leave space combat.

## 1. Battle trigger

### The presence test (`FUN_00509710`)

A side is **present** at a system when it has:

- an active fleet (`FUN_004ffef0`, types 0x08..0x0f, mode 1 of
  `FUN_004f6b90`), **OR**
- an active fighter squadron (`FUN_00503a50`, types 0x1c..0x1f, mode 1).

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

`FUN_0050b8e0` recomputes the system's `+0x88` bit 0x1000 (`battle`):
both `FUN_00509710(system, 1)` and `FUN_00509710(system, 2)` must
return non-zero.  So system squadrons **do** count toward the battle
condition.  If an enemy fleet arrives at a system whose only defenders
are system-held squadrons, both sides are present and the battle bit is
set.

### Defense facilities

Defense facilities (families 0x20..0x2f) do **not** count toward
presence.  `FUN_00509710` checks only for fleets (0x08..0x0f) and
fighter squadrons (0x1c..0x1f).  A system with only shields and
batteries is not "defended" for battle-trigger purposes; the enemy fleet
must face a fleet or system squadron to start combat.

### The battle dispatch (`FUN_0051ddf0`, `FUN_00516ef0`, `FUN_00517270`)

Message 9 in the frame loop calls `FUN_0051ddf0`, which processes one
location by running `FUN_00516ef0` (pre-battle) and `FUN_00517270`
(aftermath) in sequence, draining the scheduler between them.

`FUN_00516ef0` performs the same side-1 / side-2 presence check using
`FUN_004ffef0` and `FUN_00503a50` (`FUN_00516ef0.c:63-82`).  When both
sides are present (`bVar2 && bVar3` at `:96`), it determines which side
is being engaged from the system's `+0x78` bits 6..7 and iterates the
enemy's system squadrons with `FUN_00556710` per squadron (`:126-137`)
and the enemy's fleets and squadrons with `FUN_004f78e0` (`:143-178`).
These calls mark the objects for combat participation: `FUN_00556710`
routes the squadron into a combat context (`FUN_00551c80` at
`FUN_00556710.c:77`) and calls vtable slots `+0xb4` and `+0xb0`,
which hyp: set the squadron's combat-ready state.

`FUN_00517270` runs after the scheduler has processed battle events.
It checks the same presence conditions.  When only one side remains
(`bVar5` xor `cVar2`), it calls `FUN_0050a040` to set the system's
control and `FUN_00517ee0` to resolve the battle outcome. When both
sides are still present, it sets `local_44 = 0` and skips the
resolution — the battle continues next tick.

### Event 0x31a and the orchestrator

The only caller of `FUN_00514a60` (the system battle orchestrator) is
`FUN_0057ba50`, the fire slot of event `0x31a` (factory `FUN_0057bbb0`,
registered at `FUN_0051ef80.c:27`).  The event carries:

| Offset | Field |
|---|---|
| `+0x20` | context data for `FUN_004fd450` |
| `+0x3c` | DatId (first participant — system or object) |
| `+0x4c` | attacker-side reference |
| `+0x50` | defender-side reference |

`FUN_0057ba50` constructs a combat context from `+0x20` and calls
`FUN_00514a60(+0x3c, +0x4c, +0x50, context)`.

## 2. Combatant population

### Space combat entry (`FUN_005457f0`)

`FUN_005457f0` gates auto-resolve on `FUN_005438a0`, which returns true
only for six specific DatIds:

| DatId | Note |
|---|---|
| 0x31000241 | capital ship combat proxy |
| 0x32000242 | capital ship combat proxy |
| 0x33000243 | capital ship combat proxy |
| 0x38000343 | fighter combat proxy |
| 0x34000280 | Emperor Palpatine |
| 0x35000281 | Darth Vader |

These are not individual units — they are combat-context objects that
aggregate the stats of all participating units.  The DatId at event
`+0x3c` identifies which proxy triggered the combat.  When the gate
passes, `FUN_00549910` (the 7-phase auto-resolve) runs.

### Stat getter for fighters (`FUN_00543690`)

The fighter stat getter resolves `0x38000343` (the fighter combat proxy)
via `FUN_005066a0`.  The proxy's container (obtained with
`FUN_004f6b50`) provides the per-side stat object.  hyp: the proxy
aggregates all fighter squadrons in the combat context, including system-
held ones that `FUN_00556710` routed into the context.

### Stat getter for weapons (`FUN_005434b0`)

The weapon stat getter calls `FUN_00507010()` for a global combat stat
object, then `FUN_004f6b50` for its per-side descendant.

### Do system squadrons become combatants?

Yes.  `FUN_00516ef0` iterates every system squadron of the enemy side
(via `FUN_00503a50`, `:123-139`) and processes each with `FUN_00556710`.
`FUN_00556710` routes the squadron into the combat context
(`FUN_00551c80`) and calls the squadron's vtable `+0xb0`, `+0xb4` and
`+0xb8` slots, which hyp: register it with the combat stat aggregators.
Once registered, the squadron's firepower and durability enter the same
stat objects that the auto-resolve pipeline reads.

### Defense facilities in space combat

`FUN_00514a60` has a separate path gated by `(system+0x50 & 4) == 0`
(system not completed) that reads `system+0x38` and calls
`FUN_00529d20` through `FUN_00505750`.  `FUN_00529d20` compares the
object's DatId against the system's and conditionally calls
`FUN_0052ac40`.  This path is hyp: a defense-facility combat phase
(batteries and shields modifying the space combat result), running after
space and ground combat but before the final tally.  Shields and
batteries do not appear as individual combatants in the 7-phase pipeline
but may contribute through this separate path.

## 3. Losses

### Squadron damage (`FUN_005032c0`)

A squadron's size lives at `+0x60` (the same offset as hull for ships,
polymorphic by vtable).  `FUN_005032c0` applies damage:

```c
// FUN_005032c0.c
iVar3 = FUN_0053fc90(param_1, 0, max_value);  // clamp to [0, max]
if (iVar3 != 0 && old_value != param_1) {
    *(this + 0x60) = param_1;       // set new squad size
    FUN_00539fd0(this, 1);          // notify side 1
    FUN_00539fd0(this, 2);          // notify side 2
    vtable+0x260(old, new, ctx);    // SquadSizeDamage notification
}
```

There is no system-vs-carried distinction in this function.  The
notification (vtable `+0x260`) fires identically for a squadron aboard a
ship and one at the system.  A squad size of 0 destroys the squadron.

### Destruction path

When `+0x60` reaches 0, the object becomes inactive (`+0xac` bit 0
clear).  The combat result phase (`FUN_005445d0`) reads that bit to
count survivors; the post-combat phase (`FUN_00544a20`) invokes
`FUN_00543e00` per side, which sets `+0x58` bit 0x400 for the
losing side.  No special removal code for system-held squadrons is
visible; they follow the same destruction pipeline as carried ones.

## 4. Outcome

### Who wins (`FUN_005445d0`)

The combat result checks the "alive" flag at `+0xac` bit 0 for five stat
categories.  For the fighter category (local_50):

```c
// FUN_005445d0.c:90-92
(*(byte *)(local_50 + 0xac) & 1) != 0
    || (*(byte *)(local_50 + 0x50) & 8) != 0  // fighter exception
```

Dead fighters whose `+0x50` bit 3 is set still count as combat-ready.
hyp: this bit flags a fighter group whose parent ship was destroyed but
the fighters themselves survived (ejected).

After checking all categories, `FUN_005445d0` calls `FUN_00543d90` per
side.  `FUN_00543d90` sets `+0x58` bit 0x200 on the combat state to
mark the winner.

### System squadrons and presence after combat

After the combat result, `FUN_00544a20` (post-combat) runs.  It checks
whether the combat state has bit 0x1000 and calls `FUN_00543e00` per
side, setting bit 0x400.  `FUN_00517270` then checks whether each side
still has fleets or system squadrons.  A side with only system squadrons
(no fleets) IS present for this check.  So a side that loses all its
fleets but retains system squadrons still counts as present: the system
stays contested.

### Retreat

`FUN_00544da0` (fleet eval, `:64-68`) checks `+0xb0` bit 1 before
issuing a retreat.  hyp: bit 1 of `+0xb0` is a "fleet can retreat" flag.
System squadrons have no fleet to retreat with — they are children of
the system, not of a fleet.  hyp: a system squadron cannot retreat;
it stays at the system until destroyed or the battle ends.

## 5. Tactical battle

### Setup (`FUN_005a7500`)

`FUN_005a7500` is the tactical battle manager constructor (0x3a8 words).
It registers ship-group, nav-point and ship-event handlers.  The tactical
view receives its combatant lists from the same presence iterators:
`FUN_004ffef0` for fleets and `FUN_00503a50` for system squadrons.

The Battle Alert (`FUN_00450770`, `battle-alert-lists.md`) lists system
squadrons on pages 1 and 2 (Alliance / Imperial Forces) under the
system's name, after all fleet ships.  Those same squadrons enter the
tactical session.

### Carrier requirement

A system squadron does NOT need a carrier to participate.  `FUN_00503a50`
iterates children of the system itself (types 0x1c..0x1f), and
`FUN_00516ef0` processes them with `FUN_00556710` regardless of whether
they are aboard a ship.  hyp: in the tactical view, system squadrons
spawn as free-flying units (they have no parent ship to launch from).

### `FUN_00450770` caller chain into the tactical session

The Battle Alert window (`FUN_0044f860`) shows the summary and the three
list pages.  Tab cases 2, 3 and 4 call `FUN_00450770` with page 1, 2 or
0, which lists:

- Alliance Forces (1) / Imperial Forces (2): per fleet of the side at
  the system, then per ship and its carried units, and last system
  squadrons under the system's name (`FUN_00503a50`).
- System Assets (0): facilities and ground units.

The player sees system squadrons listed in the force summary before
choosing auto-resolve or tactical.  hyp: the tactical session uses the
same iterator to spawn those squadrons as free units on the battlefield.

## Still open

- **Who raises event 0x31a**: the build-delivery note says vtable slot
  `+0x148` at `0x004fbaf0`, but the caller chain from `+0x148` to the
  event factory `FUN_0057bbb0` is untraced.  hyp: something in
  `FUN_00516ef0` or `FUN_00517270` raises it, but neither decompile
  contains a `FUN_0053fbd0(0x31a, ...)` call.

- **FUN_00509ad0**: called at `FUN_00516ef0.c:62`, its return value
  gates the battle processing.  hyp: it reads a "battle pending" state
  from the system; function not decompiled.

- **Tactical spawn**: the exact function that places system squadrons on
  the tactical grid (coordinates, orientation, formation) is inside the
  `FUN_005a7500` manager and its message handlers; the decompile corpus
  covers the constructor but not the per-unit placement.

- **vtable +0xb0, +0xb4, +0xb8**: the fighter squadron vtable methods
  called by `FUN_00556710` when marking a system squadron for combat.
  These likely register the squadron with the stat aggregator; the exact
  vtable entries have not been resolved.

- **Defense facility participation**: `FUN_00529d20` at
  `FUN_00514a60.c:72-86` runs after space and ground combat when the
  system is not completed.  It compares DatIds and conditionally calls
  `FUN_0052ac40`.  Whether batteries and shields apply damage or absorb
  damage in auto-resolve is inferred from this path but not confirmed
  from the decompile of `FUN_0052ac40`.

- **System squadron retreat rules**: hyp: system squadrons cannot retreat
  (no fleet container, `+0xb0` bit 1 unset).  The decompile of
  `FUN_00544da0` checks `+0xb0` bit 1 for the retreat gate, but the
  squadron's `+0xb0` value is not traced.

## Port

- `rebellion_core::combat::side_present` is `FUN_00509710`: a fleet of the
  side at the system, or squadrons the system holds for it. Both the
  headless trigger (`rebellion-data/src/simulation.rs`) and the live one
  (`main.rs`) start a battle when both sides are present (`FUN_0050b8e0`).
- `CombatForce` is one side: its lead fleet, if any, and its system
  squadrons. The tactical session takes the sides
  (`BattleSession::new_with_forces`) and seats the system squadrons after
  the fleets' (`FUN_00516ef0`) as `TacticalFighter`s with `held_by_system`;
  they launch and return without a carrier, and
  `tactical::apply::apply_results` writes their survivors back to
  `System::fighters` (`FUN_005032c0`).
- `resolve_system_space_combat` runs that session headless and reports the
  winner by presence afterwards (`FUN_00517270`): a side left with only its
  system squadrons keeps the system contested. A win by squadrons alone has
  no winning fleet, so no bombardment or landing follows.
  hyp: they start on their side's fighter line; the original placement is
  untraced (above).
- port: facilities take no part, as traced; `FUN_0052ac40`'s facility path
  stays open.
