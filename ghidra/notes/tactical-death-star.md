# Tactical Death Star

The tactical (space battle) Death Star in the owned English `REBEXE.EXE`
(SHA-256 `b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`).
Confidence tags: **code read** (decompile and disassembly), **inference**
(reasoned from reads), **needs VM** (not decidable from the binary alone).
Complements `tactical-death-star-path.md` and `morale-retreat.md`.

## 1. Station object and the strategic export

`FUN_0040b0e0` exports a system's forces to the battle. Per fleet:

| Family range | Iterator | Export | Result |
|---|---|---|---|
| `[0x14,0x18)` ordinary capital ships | `FUN_005001b0` | `FUN_0040b8a0` | full ship record |
| `[0x18,0x1c)` Death Star | `FUN_005003d0` | `FUN_0040bcd0` | short station record |
| `[0x14,0x1c)` (ships and Death Star) children | `FUN_00502db0` | `FUN_0040be90` (squadrons), `FUN_0040c200` (characters), `FUN_0040c4b0`, `FUN_0040c5f0` | squadrons carry the carrier id |

The Death Star never enters `FUN_0040b8a0`, so it has no ship record: no
turbolasers, no strength, not in the manager's ship list (`+0x988`). Its
carried squadrons are ordinary squadrons whose carrier id is the Death Star id
(code read). The paths that touch the station are the hangar launch and recovery
messages `0x19`/`0x1a` (`FUN_005b8b10`, `FUN_005b8bd0`), which accept a carrier
id equal to the station id when `FUN_005ba420` holds, and messages `0x44` and
`0x45` (section 4).

`FUN_0040bcd0` calls `FUN_00595ef0(family, id, a3, a4, a5, a6, a7)`. Live
(`+0x668 == 0`) it reaches `FUN_0059a750` -> `FUN_005a9220`; headless it reaches
`FUN_005af4e0`. Both build the same 0x7c-byte object (vtable `0x66c3c0`) and
register it as the singleton (`FUN_005a9220`: `DAT_006bc470[0x11]`;
`FUN_005af4e0` -> `FUN_005afdc0`: `FUN_00596bd0()+0x44`, read by
`FUN_005a9330`).

| Offset | Value at creation | Source (code read) |
|---|---|---|
| `+0x18` | object id | base `FUN_005a6210` |
| `+0x1c` | hangar sub-object, vtable `0x66c3b8` (`FUN_005b82a0`) | |
| `+0x24` | hangar count 0 | `puVar1[9] = 0` |
| `+0x2c` | hangar capacity = ship slot `+0x26c` - slot `+0x23c` + the number of `[0x1c,0x20)` children | `FUN_0040bcd0` argument 7 (inference for the meaning of `+0x23c`: squadrons aboard, `status-window.md`) |
| `+0x5c`, `+0x60` | family byte, index (id low 24 bits) | |
| `+0x64` | float max hull: ship slot `+0x248` | |
| `+0x68` | float remaining hull: slot `+0x248` - slot `+0x274` (max hull minus hull damage, `bombardment.md`) | zero makes `FUN_005ba420` false |
| `+0x6c` | charge `100.0` (`0x42c80000`): the station starts fully charged | |
| `+0x70` | shield flag: the system holds at least one family `0x25` child (Death Star Shield facility; `FUN_0051c270` iterates `[0x25,0x26)` of the system) | |
| `+0x74` | not completed: ship `+0x50` bit 2 clear (`(flags >> 2 & 1) == 0`) | selects sprite 5020 |
| `+0x78` | committed flag 0 | set to 1 only by message `0x45` |

Predicates: `FUN_005ba420` (operational) = `+0x74 == 0`, `+0x68 != 0.0`,
`+0x78 != 1`. `FUN_005ba450` (attackable by the Alliance) adds `+0x70 == 0`.
Both read the same fields; only `FUN_005ba450` reads the shield.

## 2. Trench-run state machine (code read)

The result object (class vtable `0x66d0a8`, 0x84 bytes, `FUN_005d0820`) is
created by the order setter `FUN_005ca6d0` case 6 (called through
`FUN_005c7130`), only when the sprite kind (`vt+4`) is 4 and the sprite side
`+0x50` is 0, so only Alliance groups can hold order 6. The constructor sets
`DAT_006bc978 = 1` (a trench run exists; `FUN_005d0ae0` reads it, the
destructor `FUN_005d0950` clears it) and posts state 2.

`FUN_005cfe80(result, state)` sets `+0x34 = 1` and posts message `0x3c`
(`TACTICALRESULT_UPDATE`: sprite id, state) through manager slot `+0x28`, only
when the sprite `+0x34 == 0`. The dispatcher `FUN_005a96b0` routes `0x3c` to
`FUN_005caf20` -> `FUN_005caf50` -> `FUN_005cfec0(result, state)` when the sprite
holds a result object and order 6. `FUN_005cfec0` stores the state at `+0x2c`
and clears `+0x34`.

| State | Entered by | Effect |
|---|---|---|
| 2 | constructor post; also `FUN_005d0340` | idle; `FUN_005d0340` (vtable slot 2, run on the first `FUN_005d0430` tick) sets `+0x2c = 2`, `+0x80 = 0`, and voices `0x139`/`0x12d`/`0x131`/`0x135` by `FUN_005c7150(sprite)` (`0x115` when the local side is the Empire) |
| 5 | `FUN_005c6610` (below) | `FUN_005d08d0` snapshots each participant (`+0x3d0 = +0x3cc`, `+0x3c4 = +0x3c0`), sets participant state `+0x170 = 10` and `+0x108 = 1`; sets `+0x30 = 120000.0` (`DAT_0066d080`); fills nine chatter thresholds `120000 * (1 - 0.1 * i)` (`DAT_0066d084`, `_DAT_0066d09c`); voice `0x13a`/`0x12e`/`0x132`/`0x136` (`0x11d` for a local Empire); runs `FUN_005d04e0` immediately when the sprite `+0x34 == 0` |
| 9 | `FUN_005d04e0` success | `+0x30 = 120000.0`; all participants restored to the snapshot |
| 10 | `FUN_005d04e0` failure | `+0x30 = 120000.0`; no restore |
| 6 | `FUN_005d0430` when the state-9 timer is `<= 0.0` | success film (section 10) |
| 7 | `FUN_005d0430` when the state-10 timer is `<= 0.0` | failure film |
| 8 | `FUN_005ad530` (side retreat) for a sprite holding order 6 | broken off |

The 2 to 5 transition: `FUN_005c6610` (an undefined function in the saved
project; recovered by `CreateAndDecompileTargets.java`) is the sprite update.
With order (`+0x64`) 6 it flies to the side's entry point (`FUN_005a7490`),
and when the move call `vt(+0x54 object)+0xc` reports arrival and the result
reports state 2 (`+0x2c == 2`) and `+0xc == 0`, it sets `+0xc = 1` (slot `+0xc`,
`FUN_005cf910`) and calls `FUN_005cfe80(result, 5)` (disassembly `0x5c6924`).
The `+0xc` flag makes the post happen once. State 6 and 7 are posted by
`FUN_005d0430` (`0x5d048f`, `0x5d04c2`), which only ticks in states 9 and 10.

Timing: the countdown constant is `120000.0` for both outcomes and decrements by
the tick argument of `FUN_005d0430`. The outcome is decided at state 5 entry;
the countdown only delays the film. In the VM the film follows the launch
voice (state 5) by about 120 s, one step per wall-clock second (sections 16
and 20).

Result of state 6: `n = FUN_005c5eb0(list)`; `k = 0` for `n < 2`, `1` for
`n` in `{2, 3}`, else `n >> 1`. The first `k` participants in ascending object
id order take 100000.0 damage and are destroyed; the others are restored from
the snapshot, set to state 2 and re-registered (`FUN_005a9380`). Then
`FUN_005ba890(station)` sets `+0x6c = 0`, `+0x68 = 0` and runs `FUN_005b8d80`
(destroys every squadron in both hangar lists); `FUN_005c1200(station id)`
dispatches the actor event. State 7 destroys every participant. State 8
restores every participant (unless the previous state was 1 or 2) and sets the
sprite order 3.

## 3. The roll `FUN_005d04e0` (disassembly `0x5d04e0`-`0x5d0815`, code read)

Participants are copied into a scratch list keyed by object id `+0xec`
(`FUN_005c6100` -> `FUN_005c6090` -> `FUN_005c5f20` -> `FUN_005e9cc0`). The key
orders an unbalanced binary tree (`FUN_005f5500`: smaller key left,
`FUN_005f5060`: leftmost node), so the head is the lowest id. Pseudocode:

```text
chatter = [0x11e,0x11f,0x120,0x128,0x121,0x122,0x129,0x13d,0x13d]
damaged = false; pass = 0
loop while list not empty:                              # 0x5d0576 / 0x5d0713
    s = list.pop_lowest_id()                            # FUN_005f54d0, node deleted
    alive = false                                       # xor ebp,ebp at 0x5d05a6
    m = trunc(s.vt[0xbc]())                             # __ftol, no clamp (0x5d05ae)
    if rand(1,10) > m:                                  # JBE skip at 0x5d05c7
        damaged = true
        n = trunc(s.hp / 8.0)                           # s.hp = +0x3cc (0x5d05cd..0x5d05e1)
        if n < 2: s.vt[0xc0](100000.0)                  # kills
        else:     s.vt[0xc0](rand(1,n) * 8.0)
                  if s.hp <= 0.0: s.vt[0xc0](100000.0)
    if s.hp > 0.0:                                      # FCOMP 0x66d090 = 0.0
        alive = true
        r = FUN_005ad7e0(mgr, 0)                        # slot +0x950, side 0; 1.0..9.0
        v = (r < 9.0) ? r : 9.0
        c = trunc((FUN_005b23a0(s) + v) * 0.5)          # _DAT_0066d0a4 = 0.5
        if rand(1,100) <= c: SUCCESS                    # JBE at 0x5d06e3
    pass += 1
    if pass > 1000: break                               # CMP 0x3e8, JA at 0x5d06f3
    if alive: list.insert(s)                            # FUN_005c5f20, same key
FAILURE
```

- `FUN_005b23a0(s) = (s.hp / s.hp_max) * (float)(int at +0x134) * 0.53333336`
  (`FUN_005b1770`, `_DAT_0066c4cc`); `+0x3cc` hp, `+0x3c8` hp max. The meaning of
  `+0x134` is open (needs VM or the squadron constructor argument trace).
  Per-roll chance is therefore at most `trunc((0.5333 * size + 9) * 0.5)`
  percent, a few percent, not the commander rating alone.
- Retry is not round robin: a survivor re-enters with the same key, so the
  lowest-id squadron is popped again at once until it dies, succeeds or the
  1001-pass cap ends the loop; only then does the next id come up.
- `rand(a,b) = FUN_0061a310() % (b - a + 1) + a` (`FUN_005a8a70`, disassembly
  `0x5a8a70`-`0x5a8a8a`: `DIV` by `b-a+1`, then `+a`; both bounds inclusive).
  `FUN_0061a310` is the MSVC CRT `rand` on the per-thread seed (`0x61a315`
  reads `+0x14` of the thread block): `seed = seed * 0x343fd + 0x269ec3`,
  result `(seed >> 16) & 0x7fff` (`0x61a318`-`0x61a337`). Every tactical draw
  shares that one stream (`FUN_005aaf90` only returns the manager, which the
  helper ignores). The roll draws in this order per pop: `rand(1,10)`, then
  `rand(1,n)` only when `trunc(hp/8) >= 2` and the first draw missed, then
  `rand(1,100)` only when the squadron is alive.
- The pass cap is exact: the counter increments after every pop (`INC EAX`
  `0x5d06ed`, `CMP EAX,0x3e8` `0x5d06ee`, `JA` `0x5d06f7`), so 1001 pops run
  and the 1001st is rolled in full before the loop leaves for failure.
- `hp` (`+0x3cc`) and `hp_max` (`+0x3c8`) are craft count times 8.0
  (`FUN_005b9c60.c:68-69`, `_DAT_0066c6b8` = 8.0), so `trunc(hp/8)` is the
  craft count and the damage `rand(1,n) * 8` removes 1 to n craft; a single
  craft dies outright.
- `+0x134` of the squadron is the strategic squadron's slot `+0x23c`,
  `FUN_005030c0` = class torpedoes (`FUN_00502f20`) times slot `+0x244` (the
  squadron size): `FUN_005b9c60.c:74` stores it, `FUN_0040be90` pushes the
  slot `+0x23c` result as the constructor argument that lands there (pushes
  `0x40bf67`-`0x40bf6f`; the integer type is the evidence, since the other
  arguments there are floats). It is read as an integer by `FUN_005b23a0`.
- The success term is evaluated at double precision: the CRT sets the x87
  control word `0x27f` (`0x61a364`), `FUN_005b1770` computes
  `hp * x / hp_max` in `ST0` (`0x5b1770`-`0x5b177a`), the sum with `min(rating, 9)`
  is stored as a float (`FSTP` `0x5d06c0`), then `* 0.5` and `__ftol`.
- Slot `+0xbc` on the squadron vtable `0x66c028` is `FUN_005ba270`
  (pointer table `0x66c0e4`), and slot `+0xc0` is `FUN_005b5700`. Its
  `FUN_005ad7e0` result is discarded (`FSTP ST0` `0x5ba2be`): the value is
  `min(+0x3fc + 1.0, 9.0)` (`_DAT_0066c6cc` = -1.0, `_DAT_0066c700` = 9.0), or
  1.0 when slot `+0x90` is 0.0. Slot `+0x90` on `0x66c028` is `FUN_005ba190`,
  the squadron's speed: `(commander rating + +0x3f8) - (...)`, 0.0 when `+0x3b4`
  is set or the difference is not positive.
- Damage slot `+0xc0` is `FUN_005b5700` on all three ship vtables
  (`0x66c140`, `0x66c560`, `0x66c028`): `hp -= x`, `+0x3c0 = 0`, clamp at 0. It
  does not destroy or unlist the squadron; a dead squadron simply is not
  re-inserted and stays in the participant list at hp 0 until state 6, 7 or 8.
- Slot `+0xbc` (maneuver) on the capital vtable `0x66c140` is `FUN_005b0be0`;
  the fighter squadron vtable `0x66c028` takes `FUN_005ba270` (above), so the
  Alliance squadron's maneuver never reads the commander.
- Success: state 9, `+0x34 = 0`, `+0x30 = 120000.0`, all participants restored
  from the snapshot (damage taken in the loop is undone); if `damaged`, chatter
  slots 1..6 become `0x123,0x124,0x125,0x12a,0x12b,0x126`. Failure (empty list
  or pass cap): slots 1..8 become `0x123,0x124,0x125,0x12a,0x12b,0x126,0x12c,0x127`,
  state 10, no restore.
- One result object per attacking group (per sprite with order 6).

## 4. Fire, commit, recharge

| Message | Handler | Effect (code read) |
|---|---|---|
| `0x44` | `FUN_005ba520(target id, station id)` | target must pass `FUN_005ad870` (alive in the battle list, state 2, 4 or 6); sets manager `+0x894 = 1`, voice `0x14` (`0x91` for a local Alliance); `FUN_005c12d0(station id, target id, 0, 1.0, 1)` runs station-actor slot `+0x20` (the beam; the kill after it is not traced, needs VM) |
| `0x45` | `FUN_005ba5e0` | requires `FUN_005ba420`; sets `+0x78 = 1`; `FUN_005b8d10(hangar)` moves each docked-ready squadron (hangar list `+0x1c`, state not 3) into the recovered list `+0x20` with state 5; `FUN_005c1200` dispatches the actor event |

`0x44` is posted by `FUN_005ba720(station, target id)`, which also sets
`+0x6c = 0` and creates the effect object `FUN_005ce500(.., 3, ..)`; so the
charge is reset when the shot is posted. Callers: the human click path
`FUN_005a5740` (target not on the local side) and the AI `FUN_005c8780`.
`0x45` is posted only by `FUN_005c8780` (Empire sprite `vt+4 == 1`, station
operational, controller `+0x84` (`param_1[0x21]`) moving 2 to 3 when its timer
`+0x7c` reaches 0.0 and `param_1[4] == 0`). `FUN_005ad530` sets that
controller state to 2 (side retreat), so `0x45` is the Death Star leaving with
the retreating Empire. `FUN_005ba7f0` recharges at `dt * (1/3) * 0.001` per
step to 100.0 (voice `0x113` at the crossing for a local Empire).
`FUN_005ba890` runs only from `FUN_005cfec0` state 6.

## 5. Shield `+0x70` in `FUN_005ada10` (code read)

`FUN_005ada10` is the advisor script (state `+0xe94`). Case 1 reads `+0x70`: when
the station is operational and `+0x70 != 0` the state becomes 2, whose message
is `+0xe7c = 0x93` (Alliance local); a missing or non-operational station
with `+0x70 == 0` goes through the Alliance/Empire fighter count instead. State
4 polls `FUN_005ad930`, which uses `FUN_005ba450` (so the shield blocks the
"attack warranted" predicate). `FUN_005ba420` ignores the shield.
`FUN_005ad930` = station attackable (`FUN_005ba450`), no trench run
(`FUN_005d0ae0 == 0`), Empire squadron count < Alliance squadron count (over
`+0x988`, vt `+0x20 == 1`), and no sprite of the Alliance side list already on
order 6. `FUN_005afcc0` is its headless counterpart.

## 6. Commander slot `+0x950` (code read)

`FUN_0040b0e0` finds each side's commanders at the battle system into four
exporter fields (`FUN_0040b0e0.c:86-118`): `+0x18` / `+0x20` the Alliance /
Empire Admiral (`FUN_00509460(system, side)`: the CommandKind 2 holder of
each fleet via `FUN_004fd790`, the highest slot `+0x1f4` wins, ties keep the
earlier) and `+0x1c` / `+0x24` the Commander (`FUN_00509530`: seeded by the
system's own rank 1 holder `FUN_00509330(system, 1)`, then each fleet's
CommandKind 1 holder, the highest slot `+0x1f0` wins, ties keep the earlier).
Sides are the game's: 1 Alliance, 2 Empire.

`FUN_0040c200` (character export) builds the 0x64-byte record
(`FUN_00596340` -> `FUN_00596c40`). Argument trace (`0x40c394`-`0x40c435`,
`FUN_00596c40` stores `0x596cc6`-`0x596cf3`):

| Record | Value |
|---|---|
| `+0x44` | side index: 0 when the character's copy number (`+0x24 >> 6 & 3`, `FUN_0040d690`) is 1 (Alliance view), else 1 (`FUN_0040d660`) |
| `+0x48` | the character's family is `[0x30,0x38)` (major) |
| `+0x54` | the character id equals the exporter's Admiral field (`+0x18` for copy `0x40`, `+0x20` for `0x80`) |
| `+0x58` | the character id equals the exporter's Commander field (`+0x1c`, `+0x24`) |
| `+0x4c` | slot `+0x1f4` (`FUN_004edc50`, effective short `+0x88`) over `DAT_006a80e4`, the int 10, as a float (`FUN_0040d670`: `FILD [ESP+4]`, `FIDIV dword [0x006a80e4]`) |
| `+0x50` | slot `+0x1f0` (`FUN_004edc40`, effective combat, short `+0x86`) over `DAT_006a80e8`, the int 20, as a float (`FUN_0040d680`: `FILD [ESP+4]`, `FIDIV dword [0x006a80e8]`), so a combat of 70 gives 3.5 and a rating (`FUN_005afc00`, plus one, at most 9) of 4.5 |
| `+0x5c` | the carrying object's id: container `+0x1c` object, then its `+0x54 -> +4` (`FUN_004f6b50`, `FUN_004f6df0`) |
| `+0x60` | the character's `+0x94` (injury short) is non-zero |

`DAT_006a80e4` and `DAT_006a80e8` have only those two reads
(`0x40d674`, `0x40d684`), so they are constants (10 and 20).
`FUN_005a93e0` stores the record at `manager[+0x958 + side * 4]` when
`+0x54 != 0` and at `manager[+0x950 + side * 4]` when `+0x58 != 0`.
`FUN_005ad7e0(mgr, side)` (slot `+0x950`) returns `clamp(record+0x50 + 1.0,
1.0, 9.0)` when the carrying ship (`record+0x5c`) passes `FUN_005ad870`
(listed in the battle, state 2, 4 or 6), else 1.0. So the Alliance trench
rating is `min(combat / 20 + 1, 9)` of the Alliance Commander (rank 1) with
the highest combat skill who is aboard a surviving ship; with no Commander,
or one whose ship is gone, it is 1.0. `FUN_005ad750` (slot `+0x958`, the
Admiral) returns `clamp(9.0 - record+0x4c, 1.0, 9.0)` and feeds the capital
ships' maneuver (`FUN_005b0be0`), not the squadrons'.

## 7. Hangar and retreat (code read)

The hangar sub-object (vtable `0x66c698`, `FUN_005b82a0`) sits at station
`+0x1c` and at ship `+0x418`. Its fields: `+4` carrier id, `+8` docked count,
`+0xc` reserved count, `+0x10` capacity, `+0x14` launch timer, `+0x1c` list of
docked-ready squadrons, `+0x20` list of recovered squadrons (state 5), `+0x28`
list of squadrons being recovered (`FUN_005b82a0.c:14-53`). Capacity of the
station is `+0x2c` (`FUN_005af4e0.c:40`), argument 7 of `FUN_0040bcd0`: slot
`+0x26c` (class fighter capacity) minus slot `+0x23c` (squadrons aboard) plus
the direct `[0x1c,0x20)` children (`FUN_0040bcd0.c:43-63`).

| Step | Function | Rule |
|---|---|---|
| Battle start | `FUN_0040be90`, `FUN_005964c0.c:133-153` | A squadron whose strategic parent is in `[0x14,0x1c)` (`FUN_0040be90.c:83-94`; the Death Star counts) looks up its carrier with `FUN_005a6360`, which falls back to the station hangar for an id not in the ship table (`FUN_005a6360.c:15-31`). Without a working hyperdrive (`FUN_005b1b70`) it docks ready: `FUN_005b8570` (`+0xc < +0x10`), state 1, carrier `+0x164`. A squadron with a working hyperdrive, and a squadron the system holds, starts in state 2 (`FUN_005b69b0`) |
| Launch | message `0x19`, `FUN_005b8b10` -> `FUN_005b8c70` | Target id equal to the station id needs `FUN_005ba420`; removes the squadron from `+0x1c`, state 2, both counts minus 1 |
| Recovery request | `FUN_005cf980.c:97-104`, `FUN_005b84e0` | Tactical side 1 only; needs `FUN_005ba420`; `+0xc < +0x10`; list `+0x28` |
| Recovery done | message `0x1a`, `FUN_005b8bd0` -> `FUN_005b8630` | Same station gate; list `+0x20`, state 5 |
| Retreat | `FUN_005afe40.c:14-72` | State 2 only: dock in the squadron's carrier (the station needs `FUN_005ba420`), else any own carrier (`FUN_005afc80`: state 2, in the side list), else for side 1 the station when `FUN_005ba420`; capacity gate `FUN_005b84a0` (`+8 < +0x10`); success sets state 5 and removes it from the side list (`FUN_005c60d0`) |
| Station departs | message `0x45`, `FUN_005ba5e0.c:11-15` | `FUN_005b8d10.c:10-22` moves each squadron of `+0x1c` (docked-ready, not state 3: `FUN_005b8ce0`) to `+0x20` with state 5 and removes it from `+0x1c`. Flying squadrons are not recalled |
| Battle end | `FUN_005a9ec0.c:335` (vtable slot `+0x8c`: `FUN_005bef90` local, `FUN_005c0b60` network twin at `0x66c90c`); headless `FUN_005ae9c0.c:196` -> `FUN_005aee90` | Run after the result code is set (state 4 -> 5). Every squadron (`vt+0x20 == 1`) that is alive per `FUN_005ad870` or in state 1, 5, 9 (headless: 2, 1, 5, 9) docks into the first own ship (`vt+0x20 == 0`, side equal) alive per `FUN_005ad870` or in state 7, 8, 9 (headless: 2 or 9) with room (`FUN_005b84a0`, state 1, carrier `+0x164`); else, for side 1, into the station when `FUN_005ba420` holds or `+0x78 != 0` (`FUN_005c0b60.c:57-82`, `FUN_005aee90.c:20-47`). The live variant then posts the RNG sync message `0x4a` (`FUN_005c0b60.c:36-51`) |

State 1 and 5 are not "alive" for `FUN_005ad870` (it accepts 2, 4, 6;
`FUN_005ad870.c:13-21`), so docked squadrons neither fire nor appear in target
lists. Station destruction (`FUN_005ba890.c:5-7`, `FUN_005b8d80.c:8-19`)
destroys every squadron of `+0x1c` and `+0x20` (`FUN_005b2d20`, vtable slot
`+0x1c`: state `0xb`, hit points 0, `FUN_005b2d20.c:20-43`); squadrons of other
carriers are untouched and a flying squadron launched from the station is in
neither list.

## 8. AI (code read)

`FUN_005c9200` only builds the sorted target lists of the side controller
(`FUN_005c8780`); it neither fires nor orders. Per AI controller update
(`param_1[2] == 1`, state `+0x84 == 0`), on every sixth update (timer `6.0`,
step 1.0):

- Empire (`vt+4 == 1`), station operational, `+0x6c == 100.0`: the first enemy
  in the target list with `vt+0x20 == 0` that passes `FUN_005ad870` is fired on
  (`FUN_005ba720`). The station begins full, so the first AI evaluation can fire.
- Alliance: if `param_1[6] == 0`, no trench run, and `FUN_005ad930` is true, the
  head group of `param_1[0x19]` gets order 6 (`vt+0x64(6)`, `param_1[6] = 1`);
  if `FUN_005ad930` is false and the side has no groups, it retreats
  (`FUN_005ad530`).

The headless resolver `FUN_0059a530` (engine `+0x668 == 1`) steps
`FUN_005ae9c0` (`simulate-runs-tactical-engine.md`), which never calls
`FUN_005c8780`, `FUN_005ba720` or `FUN_005ca6d0`. Its side step
`FUN_005d14d0` (vtable `+0x28` of side `+0x250`, the Empire, then `+0x24c`)
runs the Death Star itself, with its own routines (section 18).

## 9. Uncompleted Death Star (code read)

`FUN_0040d760` is a field reader: it copies the uint at `this+0x74` of a
production manager, its ProductKey (`FUN_0040d760.c:5`, `FUN_004f26d0`).
`FUN_0040b0e0` fetches the manager with `FUN_00509670(system, 0)`: the first
child of the system in family range `[0xa0,0xb0)` (`FUN_0052c170.c:8-10`, mode
4: any state) whose slot `+0x1ec` returns 0 (`FUN_00509670.c:18-24`): the ship
manager, whose current product is the first accepted entry of its queue
(`death-star-product.md`, section 7). When the product id (`FUN_0040b0e0.c:186-197`) is in
`[0x18,0x1c)` and `FUN_00506260` returns an object whose `+0x50` bit 2
(completed) is clear, the object is exported through `FUN_0040bcd0`
(`FUN_0040b0e0.c:199-203`). The fleet loop never reaches such a ship: its
iterator uses mode 1, which requires the usable bit (`object-state-flags.md`).
Export is the only path: `+0x74 = 1` (`FUN_0040bcd0.c:62`), hull
`+0x68` = slot `+0x248` minus slot `+0x274` of the unbuilt object (the full class
hull: an unbuilt product holds no damage, `death-star-product.md`, section 7), shield `+0x70` from the system's family
`0x25` children, charge `100.0`.

In battle (`FUN_005ba420` false, `FUN_005ba450` false):

- sprite 5020 (`FUN_005ab0e0`, `FUN_005ab650`);
- no hit, launch or recovery event reaches it (`FUN_005b8b10.c:38-44`,
  `FUN_005b8bd0.c:34-40`), the superlaser message needs the gate
  (`FUN_005ba520`), the `0x45` retreat needs it (`FUN_005ba5e0.c:11`), the
  charge never advances (`FUN_005ba7f0.c:13`) and no squadron docks in it
  (`FUN_005cf980.c:53,100`, `FUN_005afe40.c:32-33,63`, `FUN_005c0b60.c:79`,
  `FUN_005aee90.c:42`);
- the Alliance cannot order a trench run on it (`FUN_005ad930`,
  `FUN_005afcc0`, `FUN_005d14d0.c:21` read `FUN_005ba450`);
- it is not Empire presence: the end test treats the side as empty when the
  station is not operational (`FUN_005a9ec0.c:274-281`, `FUN_005ae9c0.c:142-148`);
- it still lets the Empire side retreat: `FUN_005d0f50.c:37-39` and
  `FUN_005c7e80.c:36-39` accept `+0x74 != 0` for side 1. The station itself
  does not move (`FUN_005ba5e0` requires `FUN_005ba420`);
- the import never touches it: `FUN_0040d170` walks only fleet children in
  mode 1.

## 10. Films 201 and 202 (code read)

`FUN_005cfec0` states 6 and 7 call `FUN_005c0fb0` and `FUN_005c0ff0`
unconditionally with no side test; both only act when `DAT_006bc45c != 0`
(a window exists). `FUN_0059cc60` -> `FUN_004019a0(0x71)` (`MDATA.201`),
`FUN_0059cc80` -> `FUN_004019a0(0x70)` (`MDATA.202`). The local side changes
only the voice (`+0x9ec == 0`) and, for a local Empire in state 6,
`FUN_005c0fa0` (destroys the window at `DAT_006bc45c+0x110`). An Empire local
player therefore sees both films when an Alliance trench run resolves in a
windowed battle; no film plays headless.

## 11. Import of the station and its hangar (code read)

`FUN_0040c780` makes three passes over the system. Pass 1 reconciles craft
counts of every squadron aboard a ship (the Death Star included, `[0x14,0x1c)`
children) and of the system's own squadrons with `FUN_0040d260`
(`FUN_0040c780.c:118-179`). Pass 2 re-parents them with `FUN_0040d320`
(`:185-237`). Pass 3 imports ships (`FUN_0040cfc0`), the Death Star
(`FUN_0040d170`) and ship children (`FUN_0040d510`) (`:243-288`).

- Squadron record (`FUN_005b8e00.c:17-45`): id, strategic family and id
  (`+0x128`, `+0x12c`), carrier id `+0x164`, a docked flag (carrier found and
  state 1 or 5), craft count (0 for state `0xb`, else `__ftol` of the strength
  with a floor of 1), state. `FUN_0040d260.c:33-42` applies the difference to
  the strategic squadron through `FUN_00503400` (gain) or `FUN_00503370`
  (loss, reason `0xc`).
- `FUN_0040d320.c:54-85`: a squadron with the docked flag takes the carrier
  record from `FUN_00596870`, which falls back to the station record
  (`FUN_00596870.c:14-17`, `FUN_005ba670.c:15-17`: the station's `+0x5c`,
  `+0x60`, the Death Star ship's family and id). When the resolved strategic
  object has the same side bits (`+0x24 & 0xc0`) it becomes the new parent;
  otherwise, or without the flag, the new parent is the system
  (`FUN_0040d320.c:75-82`). A changed parent moves the squadron (slot `+0xa8`).
  Squadrons docked in the station therefore become children of the Death Star
  ship; squadrons still flying, jumped or destroyed return to the system.
- `FUN_0040d170.c:18-58`: the station snapshot (`FUN_005ba620.c:12-17`:
  alive flag `hull > 0`, hull, `+0x78`) is compared with the ship's remaining
  hull (slot `+0x248` minus slot `+0x274`): a lower battle hull applies the
  difference as damage (`FUN_00501ae0`, reason `0xc`; at full damage slot
  `+0xac` destroys the ship, `FUN_00501ae0.c:25-31`), a higher one repairs
  (`FUN_00501b70`). A snapshot with alive flag 0 on an object with `+0x50`
  bit 6 set destroys it (slot `+0xac`, reason `0xc`). `+0x78` is not read back.

## 11. Approach and presentation (code read)

Order 6 is set by `FUN_005ca6d0` case 6 through `FUN_005c7130`, which creates the
result object (`FUN_005d0820`, state 2). The Alliance AI (`FUN_005c8780.c:195-209`) and the human's
Attack Death Star button (section 15) are the two issuers. The group
then flies the `FUN_005c6610` order-6 branch to the side's entry point
(`FUN_005a7490`); when the move call returns 0 (arrival), the result is in state
2 and its `+0xc` is clear, it posts state 5 once. State 5 (`FUN_005cfec0.c:187`,
after the switch, calls `FUN_005d08d0` first): each participant is
removed from the manager's live list (`FUN_005a93b0`, list `+0x8f8` or `+0x900`
by vtable slot `+0x20`), snapshotted, set to unit state 10 with `+0x108 = 1`,
and its actor receives event `FUN_005c1200` (`FUN_0059f4d0`); then the roll
(`FUN_005d04e0`) runs and the result holds state 9 or 10 for the timer
(`FUN_005d0430`, vtable `0x66d0a8` slot 1, called from `FUN_005c65e0` each
sprite update while the sprite `+0x34` is 0). The player hears the nine
chatter slots at 10% steps of the timer, then sees film 201 (state 6) or 202
(state 7) in a windowed battle (section 10). State 6 re-lists the survivors
(`FUN_005a9380`, state 2, `+0x108 = 0`) and destroys the station; state 7
destroys every participant. The timer's argument is traced in section 16.

## 12. Strength term of `FUN_005d0ce0` (code read)

`FUN_005d0ce0.c:80-103`: with a tactical Death Star present and operational
(`FUN_005ba420`): the Empire side (vtable `+4` == 1) adds `_DAT_0066d0f0` =
20000000.0 to its strength; the Alliance side adds the same value when
`FUN_005afcc0` holds (the station is attackable `FUN_005ba450`, and over the
engine's squadron list `+0x64` the Alliance squadrons (`+0x168 == 0`) outnumber
the Empire's strictly) or when one of its own groups (list `+0x70` of the
controller, `param_1[0x1c]`) holds order 6 in `+0x68` or `+0x64`. Without
either the Alliance adds nothing.

`FUN_005d0f50` (retreat permitted): side `+0x80 != 0` and controller state
`+0x84 == 0`; `FUN_005c7e40` (opposing capital with unit `+0x150` set) is 0;
then true when a squadron (`+0x64`) or capital ship (`+0x58`) of the side has
`FUN_005b1b70 == 1` (a hyperdrive works; returns EAX = 1, `0x5b1b9c`), or the
side's vtable `+4` is 1 and the Death Star exists and is operational
(`FUN_005ba420`) or has `+0x74 != 0` (not completed). The test reads nothing
about the Death Star's fleet.

## 13. Alliance order test `FUN_005ad930` (code read)

True when: the station exists (`FUN_005a9330`) and `FUN_005ba450` holds
(`+0x74 == 0`, `+0x68 != 0`, `+0x70 == 0`, `+0x78 != 1`), `DAT_006bc978 == 0`
(no result object), the Alliance squadron count is strictly greater than the
Empire's (list `+0x988`, vtable slot `+0x20 == 1`, side `+0x168`), and no
sprite in the first side controller's list (`manager+8 -> +0x70`) holds order
6 in `+0x68` or `+0x64`.

## 14. Hangar launch cadence (code read)

- Clock. `battle+0x87c` is a step counter: 3 from the battle's base
  constructor (`FUN_005a7500.c:551`, `param_1[0x21f]`; `FUN_00599c90.c:34-37`
  frees the last battle with `FUN_005aafa0` and builds a fresh one through
  `FUN_005be0f0` -> `FUN_005be160`; `FUN_0040b0e0.c:157` reaches it through
  `FUN_00595d60` before the squadron exports at `:221` and `:279`, so no
  squadron docks before it), set again to
  `battle+0x8ac` by the layout (`FUN_005ab650.c:527`) and at battle
  start (`FUN_005bea30.c:38`), and incremented once per step
  (`FUN_005bea30.c:125`). `battle+0x8ac` is 3 for the windowed battle
  (`FUN_005be160`, `MOV [ESI+0x8ac],3` at `0x5be233`; the network twin
  `FUN_005bf830` at `0x5bf929` sets 3, `FUN_005a7500.c` overwrites it from
  the network setup). A step is `battle+0x8a8` = 1000 ms
  (`FUN_005bea30.c:33`). So the hangar's times are in steps, not milliseconds.
- Constants. `_DAT_0066c690` = 0.0, the hangar's "timer unarmed" value
  (`+0x14` is 0 after `FUN_005b82a0.c:63`). `FUN_005b8b00` returns hangar
  `+0x18` (`FUN_005b8b00.c:5`), set to `DAT_0066c68c` = 5.0 at construction
  (`FUN_005b82a0.c:59`).
- `+0x14` is an absolute launch step, not a countdown; nothing decrements it.
  `FUN_005b8800(hangar)` (`FUN_005b8800.c:4-75`) runs once per step:
  list `+0x1c` empty returns (`:23-27`); timer 0.0 arms it to
  `5.0 + rand(0,8) + battle+0x87c` (`:28-37`; `FUN_005b8570.c:19-30` arms it
  the same way when a squadron docks into an unarmed hangar, windowed only,
  engine `+0x668 == 0`, at setup, where `battle+0x87c` is 3, so the first
  launch step is 8 to 16; `rand` is `FUN_005a8a70` on the CRT `rand`,
  `FUN_0061a310`, seeded with `timeGetTime` by `FUN_005a7500`);
  `battle+0x87c < timer` returns (`:39-43`); otherwise the front squadron of
  `+0x1c` (`FUN_005c6070`) is posted as message `0x19` (carrier id
  `hangar+4`, squadron id `+0xec`; `FUN_005ce500`, stamp `battle+0x87c`) through
  battle slot `+0x2c` (`:44-58`) and, while `+8` (docked count) is non-zero, the
  timer is re-armed with a fresh `5.0 + rand(0,8) + battle+0x87c` (`:59-72`).
  One squadron per hangar per expiry, spacing 5 to 13 steps (about 5 to
  13 s). The message is handled by `FUN_005b8b10` -> `FUN_005b8c70` (state 2,
  counts minus 1); it is delivered through the lockstep queue
  (`FUN_005a96b0` dispatch from `FUN_005bea30.c:110`), which is why the
  squadron appears a few steps after the launch step.
- Callers. `FUN_005a9de0` (battle slot `+0x88`, argument 1000.0 per step,
  `FUN_005bea30.c:121`): the station hangar (`station+0x1c`) runs only when
  `battle+0x9ec == 1` and `FUN_005ba420` holds (`FUN_005a9de0.c:14-17`); every
  live capital ship (`vt+0x20 == 0`, `FUN_005ad870`, ship `[0x49] == 0`) runs
  its own hangar at ship `+0x418` (`FUN_005a9de0.c:34-36`). So every carrier
  launches on this timer, not just the Death Star.
- Start state. A squadron without a working hyperdrive (`FUN_005b1b70` 0)
  whose carrier resolves (`FUN_005a6360`) docks ready: state 1, carrier
  `+0x164`, hangar list `+0x1c` (`FUN_005964c0.c:149-153`), for any carrier
  whose strategic parent is in `[0x14,0x1c)` or a ship. With a working
  hyperdrive it starts state 2 (`FUN_005964c0.c:179`).
- Headless. `FUN_005ae9c0` and `FUN_005aee90` never call `FUN_005b8800`
  or `FUN_005a9de0`: no hangar timer runs. The Empire side's first step
  takes every squadron docked at the start (`FUN_005af4b0` lists them in
  `battle+0x58`) out of its hangar (`FUN_005b8450`) into flight, state 2, and
  empties the list (`FUN_005d14d0.c:62-82`), so they fight from the first
  step; `FUN_005aee90.c:20-47` docks the survivors at the end.

## 15. Human issuer of order 6 (code read)

The command panel (window procedure helpers `FUN_005df4f0`, `FUN_005df5f0`)
maps button `0xdd06` to order code 6 (`FUN_005df5f0.c:16-18`); OK (`0xdd01`)
commits it with `FUN_00595c00(code)` -> `FUN_0059eec0` -> the order setter
(vtable slot `+0x48`, `FUN_005ca6d0` for the Alliance group class) via
`FUN_0059f350`. Enablement is `FUN_005df7a0.c:19-34`, for a selected object of
kind 4 (`vt+4 == 4`, a fighter group): the button (`panel+0xa8`) is enabled
only when `FUN_00401060() == 1` (GameOptions `+0x108`, the local faction; 1 is
the Alliance, which also picks the Alliance bitmaps in `FUN_005df110.c:98-111`)
and `FUN_00596bb0()` is true, else disabled. `FUN_00596bb0` is `FUN_005ad930`
on the manager (section 13): the station exists, `FUN_005ba450` (operational,
`+0x70` shield flag 0), `DAT_006bc978 == 0` (`FUN_005d0ae0`, no result object),
Alliance squadrons strictly outnumber the Empire's, and no side-0 group
already holds order 6. The panel constructor starts the button disabled
(`FUN_005df110.c:104`) and `FUN_005df7a0` is the enabling call. The setter
(`FUN_005ca6d0.c:141-157`) additionally requires kind 4 and group side
`+0x50 == 0`, then creates the result object (`FUN_005d0820`, state 2).

## 16. Trench-run timer argument (code read)

`FUN_005d0430(this, dt)` subtracts `dt` from the 120000.0 at `+0x30`
(`FUN_005d0430.c:23-26`; `_DAT_0066d084` = 120000.0, floor `_DAT_0066d090` =
0.0) and is called by `FUN_005c65e0` (`FUN_005c65e0.c:7`), which is slot `+0x10`
of the group vtables (`0x66bee0`, `0x66bf70`), not slot 0. The caller is
`FUN_005a9d40` (battle slot `+0x84`; its group loop calls group slot `+0x10`
with its own argument, `FUN_005a9d40.c:13-16`), invoked once per step from
`FUN_005bea30.c:120` with `(float)battle+0x8a8` = 1000.0 (the step length
in ms, written as a constant at `FUN_005bea30.c:33` and `FUN_005ab650.c:524`).
The timer thus drains by 1000.0 per step: 120 steps. Per-frame code
(`FUN_005a9ec0.c:74`, group slot `+0x14` = `FUN_005c6610` with the sub-step
remainder) does not touch it. In the VM the film follows the launch voice by
about 120 s of wall time (section 20).

## 17. Order-6 approach (`FUN_005c6610.c:112-145`, code read)

- `_DAT_0066cb18` = 0.0, `_DAT_0066cb14` = 0.5, `_DAT_0066cb10` = -1.0
  (read from `REBEXE.EXE`).
- `battle+0x8cc` starts at 100.0 (`FUN_005ab650.c:69`) and gains 3.0
  (`_DAT_0066c2a0` = -3.0 subtracted, `FUN_005ab650.c:203`) each time a
  counted capital ship puts its side's running count ahead of the other side's
  (`:199-208`); the value is 100.0 plus 3.0 per such step of the running count.
  It is passed to `FUN_005c10a0` and `FUN_0059f800` at battle start.
- `FUN_005a7490(this, out, side)` copies the side record's `+0x48..+0x54`
  (four floats) into `out` and returns it (`FUN_005a7490.c:13-16`). The
  record is set by the battle constructor: side 0 (Alliance, `battle+8`) is
  (0, 0, +1.0, -1.0) and side 1 (Empire, `battle+0xc`) is (0, 0, -1.0, -1.0)
  (`FUN_005be160.c:27-30`, `:42-45`): a facing along z, w fixed at -1.0.
- Leader steer vector: `(-z, 0.0, x = 0.0, -1.0)` stored into the leader
  `+0x1c` (`FUN_005c6610.c:118-122`): Alliance (-1, 0, 0), Empire (+1, 0, 0)
  (x and z swap roles: the unit heads along x). Once the result is in state 2 the
  target is `0.5 * battle+0x8cc * that vector` with w = -1.0
  (`:126-132`): Alliance (-50, 0, 0), Empire (+50, 0, 0) at 8cc = 100.0, i.e.
  the point `0.5 * 8cc` from the origin on the side's own edge.
- The move call is leader sub-object slot `+0xc` = `FUN_005eb1a0`
  (`FUN_005eb1a0.c:4-62`, vtable `0x66d9e8`): `(target, dt, tolerance)`. It
  subtracts the leader position (`+0x30..+0x38`); when the distance is
  greater than `tolerance` (0.2, `0x3e4ccccd`) it moves the leader by
  `dt * 0.001 * speed * direction` (`_DAT_0066d894` = 0.001, speed =
  slot `-0xc` `+0x34` times `_DAT_0066d84c` = 0.95) and returns 1; within
  `tolerance` it returns 0 (arrived). The 0.2 is the arrival distance
  tolerance (state 3 passes 1.0, `FUN_005c6610.c:69`).
- At arrival (`:135-143`) the result's slot `+0x10` must return 0, then
  slot `+0xc`(1) and `FUN_005cfe80(result, 5)` post state 5. Participants are the whole group
  list (`FUN_005d08d0.c:8-26` walks the group sprite's `+0x1c` list), no
  distance filter; the distance test at `FUN_005c6610.c:89-105` belongs
  to the state-3 approach and only decides which squadrons flip to state 6.

## 18. Headless Death Star (`FUN_005d14d0.c:20-60`, `FUN_005aefc0`, code read)

The headless battle object (`DAT_006bc4a8`, 0x274 bytes, built by
`FUN_00596f30` when null and freed by no traced code) carries its own
generator at `+4`: `FUN_005af420(a, b)` = `a + FUN_005f5700(b - a)`, a
Lehmer step `FUN_005f57a0` (`x * 0xa18e - (x / 0xcad4) * 0x7fffffff`, plus
`0x7fffffff` if negative), seeded 12345678 with 16 warm-up steps
(`FUN_005af5f0.c:38`, `FUN_005f56b0`). A draw is `state / (0x7ffffffe / d)`
with `d = b - a`, so it lies in `[a, b - 1]`, and `d <= 1` draws nothing.
`FUN_005b54d0` and `FUN_005aefc0` draw from it; the windowed roll and launch
use `FUN_005a8a70`.

Each side's step begins with its Death Star branch:

- Alliance (`vt+4 == 0`): station attackable (`FUN_005ba450`) and the
  Empire side's `+0x64` list strictly shorter than its own (`FUN_005c5eb0`):
  the first group of `+0x70` whose kind is 4 (fighter group, insertion
  order) runs `FUN_005aefc0`. Every step while the test holds; no timer,
  order flag or `FUN_005ad930`.
- Empire: station operational (`FUN_005ba420`). A charge other than 100.0
  runs `FUN_005ba7f0(dt = 1.0)`: `dt * (1/3) * 0.001` while hull is left and
  `+0x74` is 0 (`_DAT_0066c708`, `_DAT_0066c718`). At exactly 100.0
  (`_DAT_0066d0f4`) `FUN_005d1a70` ranks the opponents and the first unit
  with `vt+0x20 == 0` (a capital ship) is shot: `FUN_005ba710` zeroes
  `+0x6c` only (no message, voice or effect) and `FUN_005af7a0` destroys the
  ship, its hangar lists with it. The station starts full, fires on step 1,
  and refills in about 300,000 steps, past the 1200-step stalemate: one shot
  per battle.

`FUN_005aefc0(battle, group)` copies the group's members (`+0x1c`) into a
list keyed by id `+0xec` and walks it round in ascending order, survivors
keeping their place. The 101st visit (`CMP EDI,0x64; JA` at `0x5af047`)
destroys every member left (`FUN_005af7a0`) and ends with no success. Per
visit: `rand(1,10)` against `trunc(FUN_005ba270)` (strict `>` takes damage);
damage with `n = trunc(hull / 8)` below 2 destroys at once with no draw,
else `rand(1,n) * 8.0` goes through `FUN_005b54d0` (shield first, a draw
for a shield hit while it had any, the overflow at least 1.0 into hull, a
draw for the subsystem roll) and hull 0 destroys. A survivor rolls
`rand(1,100) <= trunc(float32((term + min(rating, 9)) * 0.5))`, the term
that of `FUN_005b23a0` and the rating `FUN_005afc00`'s
(`clamp(+0x50 + 1, 1, 9)`, 1.0 with no commander, with no `FUN_005ad870`
check). Success runs `FUN_005ba8a0` (`+0x6c` and `+0x68` to 0) and destroys
every unit in the station's hangar lists (`+0x38`, `+0x3c`,
`0x5af38b-0x5af3f4`); the runners keep their damage. New files:
`FUN_005aefc0.asm.txt`, `FUN_005ba8a0.c`, `FUN_005ba710.c`,
`FUN_005af420.c`, `FUN_005f6590.c`.

## 19. Station decal and picking (disassembly, code read)

The Ghidra decompiles of `FUN_005c10a0`, `FUN_005c1d30` and `FUN_005d4d10`
drop arguments; the listing (`FUN_005d4d10.asm.txt`, `RET 0x14`) has five.

- Caller. `FUN_005ab650` (`0x5ac59b`-`0x5ac5bc`, battle layout) and
  `FUN_005ab0e0` (`0x5ab510`-`0x5ab535`) call
  `FUN_005c10a0(station id, battle+0x8cc, (short)(battle+0x8d0 + 0x157c), (short)battle+0x8d4)`.
  `+0x8d4` is 5010, 5020 for `+0x74 != 0`, or 0 with no station;
  `FUN_005ab0e0` also leaves 0 for a completed station that fails
  `FUN_005ba420`. `0x157c + +0x8d0` is the planet resource `5500 + selector`
  (`tactical-planet-resource-path.md`). `FUN_005c1d30` passes them to
  `FUN_005d4d10(scene frame window+0x138, extent, station id, planet, station)`
  and files the result in the window's object tree `+0x5d8` and at `+0x628`.
- `FUN_005d4d10` (vtable `0x66d278`) is the scene surround. Its base
  `FUN_005d8980` creates its frame under the scene and sets the frame's app
  data to the station id. Frame methods by D3DRM slot: `0xf4` SetPosition,
  `0x48` AddVisual, `0xa4` DeleteVisual, `0xcc` SetSceneBackgroundImage;
  texture: `0x3c` SetColors, `0x40` SetShades, `0x44` SetDecalSize, `0x48`
  SetDecalOrigin, `0x50` SetDecalTransparency.
  - `DAT_006bcbd4+0x67c`: the scene background is resource 5030
    (`0x5d52c7`-`0x5d534e`).
  - `DAT_006bcbd4+0x680` (Show Planet): the planet decal, origin (128,128),
    size `extent`, on frame `+0x54` at
    `(2.0 * extent, rand(0.2 * extent) - 0.1 * extent, rand(0.2 * extent) - 0.1 * extent)`
    in the scene (`0x5d5372`-`0x5d54f2`; `_DAT_0066d23c` = 0.1).
  - Station argument non-zero (`0x5d54f7`): the station decal `+0x64`, origin
    (64,64), transparent, size `0.5 * extent` (`_DAT_0066d260` = 0.5), on
    frame `+0x50` at
    `(-2.0 * extent, rand(0.4 * extent) - 0.2 * extent, rand(0.4 * extent) - 0.2 * extent)`
    in the scene (`0x5d5505`-`0x5d55a4`; `_DAT_0066d248` = -2.0,
    `_DAT_0066d244` = 0.2).
  - `rand(n)` is `FUN_005f5700` (`[0, n)`, no draw for `n <= 1`; `n` is
    `__ftol` of the doubled product) on a local generator built at `0x5d52ab`
    (`FUN_005f5650`, state -1) and freed at `0x5d56ed`. Its first draw seeds
    it from the clock: `FUN_005f56b0(-1)` -> `FUN_005f5750` calls
    `srand(time(NULL))` (`FUN_0061b120`, `FUN_0061a300`), takes four `rand()`
    (`FUN_0061a310`) as `r1 * r2 + r3 * r4`, then steps `FUN_005f57a0`
    sixteen times. The offsets therefore differ every battle, and building the
    surround reseeds the CRT stream that the tactical rolls draw from. The
    planet's two draws come first when Show Planet is on; each frame takes y
    from its first draw and z from its second.
  - Child frame `+0x4c` sits at `(0, 0.11 * extent, -0.09 * extent)` from it
    (`_DAT_0066d258` = 0.11, `_DAT_0066d250` = 0.09); `FUN_005d59e0`
    (vtable slot 8, `+0x20`) builds the superlaser effect `FUN_005da150` on
    that frame. The window tree files this object under the station id, so it
    is the station actor whose slot `+0x20` the shot `FUN_005c12d0` runs
    (section 4; inference: the lookup in `FUN_005c12d0` is not re-read).
    Vtable slot 2 (`FUN_005d5970`) adds the decal to frame `+0x50` and slot 12
    (`0x5d59b0`) removes it. A zero station argument leaves `+0x64` at 0
    (`0x5d56dc`).
  - The constructor ends with `+0x34 = 0` (`0x5d56e3`).
- Removal. `FUN_005c1200(id)` looks the id up in the window tree and runs the
  object's slot 1; for the station id that is `FUN_005d5950`, which runs slot
  12 (`0x5d59b0`, DeleteVisual of the decal from frame `+0x50`) and clears
  `+0x38`. Two callers pass the station id (`+0x18`): state 6 of
  `FUN_005cfec0` (`0x5d0267`), right after `FUN_005ba890` zeroes the hull and
  charge and before the film (`FUN_005c0fb0`), and the departure
  `FUN_005ba5e0` (`0x5ba611`), which sets `+0x78`. State 7 does not call it.
  The other callers, `FUN_0059f4d0` and `FUN_0059f560`, pass unit ids
  (`+0xec`) and navigation-marker ids. The show dispatcher `FUN_005c1220`
  (slot 2) is called only from those two and `FUN_005a6d60` (unit ids), so
  nothing shows the decal again. The rebuild `FUN_005ab0e0`, which leaves no
  station resource for a completed station that fails `FUN_005ba420`, runs
  from `FUN_0059a4f0`, whose one caller `FUN_00409480` reads a stream
  (inference: the battle load path).
- Picking. `FUN_005c2d20(x, y)` runs the viewport's Pick (slot `0x70`,
  `0x5c2d7d`) and, for each picked frame path, reads each frame's app data and
  looks the id up in the window tree `+0x5d8` (`FUN_005f5500`); it returns the
  first id whose object has `+0x34 != 0` (`0x5c2e0f`). No method of vtable
  `0x66d278` or of its base `0x66d320` writes `+0x34`, so a pick never returns
  the station. The station is no tactical sprite either: it is in none of the
  manager's unit lists (section 1), which the strips, the panel arrows and the
  selection walk.
- VM, both sides: in `alliance-day1169-deathstar-trench-run.sav` (Battle at
  Coruscant on day 1173, Take Command, paused) and in
  `empire-day660-deathstar-pre-alert.sav` (Battle at Coruscant on day 676),
  the view zoomed out with PgDn shows the Death Star decal outside the battle
  box, off one face, about half a box-width across. Rotating with Left brings
  the larger textured planet into view off the opposite face, then the
  station again. The default zoom shows neither.

## 20. Trench run in the VM

`alliance-day1169-deathstar-trench-run.sav`, Battle at Coruscant on day 1173,
Alliance, Take Command:

- Red Group (23 X-wings, 276/276) is ordered to Attack Death Star from the
  fighter panel's Missions panel while paused; the panel art then shows the
  station and an X-wing.
- After the unpause the Red and Gold group buttons grey out.
- The launch voice 15161 (state 5, the end of the order-6 approach of
  section 17) comes about 61 s after the unpause, and the trench-run film
  about 120 s after 15161 (`docs/plans/2026-09-22-tooling-tactical-106-cell-acceptance.md`,
  session 7, three runs).
- The trench-run film (a fighter over the station's surface) starts about
  175-190 s of wall time after the unpause (a later run, launch voice not
  timed).
- By about 248 s after the unpause the film is over, the station decal is gone
  from the 3D view, and the Red group button is active again.
- Escape, Alt+P and Alt+NumPad+ have no visible effect in the tactical view.
- Empire side (`empire-day660-trench-tieless.sav`, the six TIE squadrons of Fleet 6 emptied so the Alliance
  has the fighter majority; fresh launch, Battle at Coruscant on day 676, Take Command, no player orders):
  the Alliance AI makes the trench run on its own (order 6, section 13). Three trench-run films play over
  about nine minutes of battle (a run through the trench; fighters over the station's surface; the station
  whole with X-wings flying off), the Imperial groups leave the group bar, and after a withdrawal and Get
  Immediate Result the Battle Summary reads "The Imperial fleet is defeated" and lists Imperial Star
  Destroyer 1, Galleon 2 and the Death Star under Capital Ships, Destroyed, and the TIE squadrons under
  Fighter Squadrons, Destroyed (`tactical-results.md`).

The two timings agree: about 61 s of approach plus the 120000.0 timer
(section 16), which drains 1000.0 per step at about one step per wall-clock
second, puts the film near 181 s after the unpause, inside the 175-190 s of
the later run. The approach time is the group's flight to its entry point
(`0.5 * extent` on its own edge, section 17), not to the station decal.

`empire-day660-deathstar-pre-alert.sav`, Battle at Coruscant on day 676,
Empire, Take Command: the station decal and the planet show off opposite faces
of the battle box (section 19). In 5 minutes of battle the Alliance AI made no
trench run.

## Open items (needs VM)

- The approach's wall time (section 20) against the group's flight distance
  and speed to its entry point (section 17) is not traced.
- The kill of the target after `0x44`.
- Whether import (`FUN_0040c780`) writes `+0x68 == 0` or `+0x78` back to the
  strategic Death Star.
- Whether a dead squadron stays in the group list `+0x1c` (it would still
  draw `rand(1,10)` when popped).
