---
title: "Planetary Assault (order 0x234)"
description: "The ground assault: order enable and gates, the command and its run FUN_00563390 -> FUN_0058c1e0 -> FUN_0058c120 (count, defence fire, one engagement per attacking regiment, collateral, landing, cleanup), the exact rolls and GNPRTB values, who sets the fleet and system flags, the report messages, control change and the AI emitter"
category: "ghidra"
created: 2026-10-17
updated: 2026-10-17
tags: [ground-assault, order-0x234, regiments, blockade, defence-facilities]
---

# Planetary Assault (order 0x234)

Confidence tags: `read` (decompiled code read), `inf` (inferred from read
code), `VM` (needs the original in the VM). Function names are
`FUN_<address>.c` in this directory.

## The pieces

| Piece | Address | Role |
|---|---|---|
| Order class vtable | `0x00669e70` | kind getter `0x005814c0` (returns `0x234`), Enable `FUN_005814d0`, Validate `FUN_005815f0`, Execute `FUN_005816f0`, list factories `FUN_005817e0` (+0x48), `FUN_00581850` (+0x4c), `FUN_00581a10` (+0x50) |
| Registration | `FUN_0051f4b0` | `FUN_0051f930(0x234, FUN_00581450)` (order factory, `0x48` bytes) |
| Fleet command | vtable `0x00669af8`, ctor `FUN_0057f970` | slot 6 `FUN_0057fac0` (fleet slot `+0x1bc`), slot 7 `FUN_0057fb90` (`+0x1c0`), slot 8 `FUN_0057fc60` (run: fleet slot `+0x1d0`) |
| System command | vtable `0x006632d8`, factory `FUN_00563500` (`0x50` bytes) | slot 6 `FUN_005631f0` (system `+0x1bc`), slot 7 `FUN_005632c0` (`+0x1c0`), slot 8 `FUN_00563390` (the run) |
| Run | `FUN_00563390` -> `FUN_0058c1e0` -> `FUN_0058c120` | six steps below |

All `read`. The order's two command lists mirror bombardment
(`bombardment.md`, "The order, its commands and the day"): list `+0x40` is one
fleet command per team fleet (`FUN_005201c0` via factory `FUN_005817e0`), list
`+0x44` one system command per distinct system (`FUN_00581850`: `FUN_005187f0`,
`FUN_005535b0`). The fleet commands precede the system command in the
scheduler queue, so the run happens in the step that issued the order (`inf`,
by the bombardment analogy; the queue was not re-traced here).

## 1. Enable (the menu row and the order)

The popup row is enabled by the order's Enable slot `FUN_005814d0`
(`object-popup-menu.md`: global gate `FUN_0051de80`, then slot `+0x18`).
Refusals, in order (`read`):

1. `FUN_0051fe20` base: empty team `1`/`0x16`; each fleet command's `+0x18`
   (`FUN_00553770` -> fleet slot `+0x1c4` `FUN_004fdf70` -> `FUN_004ffa70`)
   refuses a fleet of another side, en route, or without ships; all commands'
   systems must have one holder, else `1`/`0x14`.
2. Fleet slot `+0x1bc` `FUN_004fddd0`: after `FUN_004ffa70` passes, a fleet
   whose `+0x58` bit `0x20` is clear gives `8`/`7`.
   **Fleet `+0x58` bit `0x20` is the copy of the system's blockade bit**
   (`FUN_0050c0b0` writes `system +0x88` bit `0x20` into every active fleet of
   the system through `FUN_004fe3f0`; `blockade-bit.md`). The system bit is set
   only when the system is populated or not contested, has no battle bit, and an
   active fleet of a side other than the holder is there while the holder has no
   active fleet or fighter squadron there. A neutral system is side 3
   (`FUN_0055a080`, `FUN_0050b610`): while populated any fleet blockades it,
   so it can be assaulted (reports `0x7093`, `0x7094`); an unpopulated one is
   never blockaded.
3. `FUN_00581ae0` (team gate): the first fleet of the team that is active
   (`+0x50` bit 0) with `+0x58` bit `0x20` set and has a capital ship holding a
   regiment (`FUN_00502db0` ships, `FUN_00504c40` `[0x10,0x14)` regiments,
   `FUN_005131b0`) passes. No fleet with the bit gives `8`/`7`; a flagged fleet
   without regiments gives `8`/`5`.
4. System slot `+0x1bc` `FUN_00507d00` (run by the system command at submission,
   `FUN_00507e60` wrapper): destroyed system (`+0x50` bit 3) `1`/`0x22`;
   system `+0x88` bit `0x20` clear `0x90`/6; **the count of family-`0x24`
   shield generators at the system, either side, active
   (`FUN_005278c0`: `[0x24,0x25)`, mode 1 = `+0x50` bit 0, no side filter), at
   least GNPRTB `0x1e03` = 2** gives `0x90`/5. The manual's "two shields grey
   it out" is this test; it counts the active generators of both sides.

No side test beyond item 1, no enemy-ship test (a blockade already excludes the
holder's fleets), no battle-bit test beyond the blockade rule.

## 2. Setting the flags and the notifications

- Fleet slot `+0x1d0` `FUN_004feb60` (run by `FUN_0057fc60`): calls fleet slot
  `+0x1c0` (`FUN_004fdec0` -> `+0x1bc` `FUN_004fddd0`, the check above) and, on
  success, `FUN_004fe4d0(fleet, 1)` = **fleet `+0x58` bit 7 (`0x80`) set**, in
  both side views (`FUN_004ff4e0`). Same shape as bombardment's `FUN_004fec00`
  and bit 6. `read`.
- The bit is cleared by `FUN_004fe4d0(0)` in the run's cleanup (step 6) and by the
  fleet arrival hook `FUN_004ff600` (with bits 4 and 6).
- A view whose bit 7 changes raises event `0x183` (`FleetAssaultNotif`,
  `FUN_004ff890`, fleet slot `+0x20c`, one past bombardment's `+0x208`): the
  slot number is `inf` (vtable `0x0065d644`), the event `read`.
- System `+0x88` bit `0x80` ("assault in progress") is set by `FUN_0050a6d0(system,
  1)` in step 1 and cleared by `FUN_0050a6d0(system, 0)` in step 6; its change
  calls system slot `+0x228` -> `FUN_00512370`: event `0x150`
  (`SystemAssaultNotif`) with the holder side, the new bit, the context. `read`.

### Report messages

Event `0x183` reaches the Operation Reports class (`FUN_00496460`, visitor
`FUN_00496770`; the same visitor serves bombardment `0x182`). The assault
branch (`+0x64` nonzero) at the compose call (`param_1 == 2`) posts
TEXTSTRA (RT_RCDATA) texts, read from `data/base/TEXTSTRA.DLL`:

| Id | Text | When (`FUN_00496770`) |
|---|---|---|
| `0x7090` | "Assault on \| " | title, always (system name) |
| `0x7091` | "\| troops have taken control of the \| system" | defender is side 1 or 2 and the system's holder now equals the assaulting fleet's side |
| `0x7092` | "\| Troops have defended \| from an \| assault." | defender side 1 or 2, holder differs from the assaulting side |
| `0x7093` | "\| troops have seized control of the neutral system \|." | no defender faction (holder 0 or 3), holder now the assaulting side |
| `0x7094` | "The neutral system \| has repulsed an attack by \| troops." | no defender faction, holder not the assaulting side |

Argument order into the `\|` slots is `inf` (the code passes the faction
objects `FUN_004f3dd0(side)`, `+0x30` names, and the system's `+0x30` name).
`+0x6c` is the assaulting fleet's side, `+0x68` the other side; the picture is
`0x2b98` (assaulter side 1) or `0x2b99`. Advice code `0x2f` (viewing side does
not own the fleet; `droid-advisor-triggers.md`). `read`.

The report reads the holder after the run, so `0x7091`/`0x7093` is the
landing's control change (section 4), not a count of survivors. No separate
"Assault Summary window 0x183" exists in the read code: `0x183` is a fleet
event id, and the only message class that consumes it here is this one
(`FindScalarUses`: `FUN_0041e580`, `FUN_00496770`, `FUN_004ff890`).
`FUN_0041e580` was not read.

## 3. The run (`FUN_00563390` -> `FUN_0058c1e0` -> `FUN_0058c120`)

`FUN_00563390` resolves the system from the command's `+0x3c`
(`FUN_00506000`, families `0x90..0x97`, status `1`/`0x12`) and builds the
manager (`FUN_0058c0d0`, vtable `0x0066a8f8`, an enumerator at `+8`).
`FUN_0058c1e0(system, ...)` scans the system's fleets (`FUN_004ffe70`, no side
filter) for `+0x58` bit 7: the **first** flagged fleet's side becomes the
attacker side; a later flagged fleet of another side stops the scan, but the
first side is kept. Manager fields: `+0x10` system, `+0x14` attacker side,
`+0x18` holder side (system `+0x24` bits 6-7), `+0x1c`/`+0x24` attackers
(initial/remaining), `+0x20`/`+0x28` defenders (initial/remaining), `+0x2c`
engagement counter. `FUN_0058c120` returns 0 without acting when the system or
attacker side is 0. `read`.

Every enumerator below passes mode 1, which `FUN_005131d0` evaluates through
`FUN_004f6b90`: `+0x50` bit 0, the active bit (mode 2 = bit 4, mode 3 = bit 6,
mode 4 = always; `FUN_005131d0.c`, `FUN_004f6b90.c`). A constructor without a
side (`FUN_00513050`) takes the mode as its third argument, one with a side
(`FUN_00513090`) as its fifth. A unit under construction or in transit is not
walked. `read`.

**Attackers** (`FUN_0058ba10`, enumerator `+8` = system, `+0xc` = attacker side):
fleets of the system with the attacker side (`FUN_004ffef0` mode 1 side filter)
that have `+0x58` bit 7, then their capital ships (`FUN_00502e30`
`[0x14,0x1c)`, mode 1, side), then the regiments aboard each ship
(`FUN_00504cc0` `[0x10,0x14)`, mode 1, side). Special forces (`0x3c..0x3f`),
characters, fighters and squadrons are never attackers. A regiment already in
the system (landed earlier) is not an attacker. A callback that sets its stop
flag ends the walk. `read`.

**Defenders** (`FUN_0058bbd0`): `FUN_00504cc0(system, 1, holder side)`: the
active regiments `[0x10,0x14)` that are direct children of the system with the
holder's side bits (read live from system `+0x24` bits 6-7 at each call).
Special forces, characters and facilities are not defenders; a system with
characters but no regiments counts as undefended. `read`.

Callback vtables (the enumerator calls slot `+4` with `(regiment, ship, fleet,
&stop, ctx)`; the decompiler lost the returned value of the void callbacks, so
whether the walk continues after a non-stop call is `inf`: it does, since counts
and landings cover every unit):

| Functor ctor | Vtable | Callback | Effect |
|---|---|---|---|
| `FUN_0058bc90` | `0x0066a8d8` | `FUN_0058bce0` | counts: `+8 += 1`, stop 0 |
| `FUN_0058bd00` | `0x0066a8e0` | `FUN_0058bd60` | "pick the N-th": `+0xc` target index, `+8` running index; stores the object at `+0x10`, stop 1 on the match |
| `FUN_0058bdb0` | `0x0066a8e8` | `FUN_0058be10` | lands: while `+8 != +0xc` (limit), `+8 += 1`, `FUN_004f6fd0(regiment, system, ctx)` (move into the system) |
| `FUN_0058be70` | `0x0066a8f0` | `FUN_0058bec0` | one engagement |

### The six steps

| Step | Function | What it does |
|---|---|---|
| 1 | `FUN_0058c2d0` | `FUN_0050a6d0(system, 1)` (bit `0x80`, event `0x150`); counts attackers -> `+0x1c` and `+0x24`; counts defenders -> `+0x20` and `+0x28` |
| 2 | `FUN_0058c3c0` | defence fire |
| 3 | `FUN_0058c500` | engagements; only when `+0x28` and `+0x24` are both nonzero |
| 4 | `FUN_0058c580` | collateral |
| 5 | `FUN_0058c810` | landing; only when `+0x24 != 0` and `+0x28 == 0` |
| 6 | `FUN_0058c8b0` | `FUN_0050a6d0(system, 0)`, then `FUN_004fe4d0(fleet, 0)` for every fleet at the system of the attacker side (`FUN_004ffef0` mode 4) |

Every step runs; a failing step only changes the final return value
(`FUN_0058c120`). `read`.

#### Parameters

GNPRTB `0x200`..`0x205` load into `DAT_006bb788`, `006bb780`, `006bb78c`,
`006bb778`, `006bb784`, `006bb77c` (`FUN_0055ea00`); the shipped values (all
columns) are 5, 30, 10, 4, 6, 8. `FUN_0053e290(n)` is `FUN_005f5700(rng, n+1)`:
an integer in `[0, n]`, inclusive (`rand / (0x7ffffffe / (n+1))`).
`FUN_0053e2f0(x)` is `FUN_0053e2e0() < x` with `FUN_0053e2e0` =
`FUN_0053e290(100 - 1)` = `[0, 99]`. All integer arithmetic, no float. `read`.

#### Step 2: defence fire (`FUN_0058c3c0`, `FUN_0055ead0`)

For each active defence facility of the system (`FUN_00526fd0`: `[0x22,0x28)`,
mode 1, no side filter, container order) the loop computes `FUN_0055ead0(attack / G[0x200])`: the
facility's class `+0x5c` (`FUN_00520b70`, DEFFACSD `attack_strength`: KDY 2000,
LNR I 800, GenCore and shield generators 0) divided by 5 (integer); it fires
when `rand[0,99] < that`. Any class with attack >= 500 fires every time (`inf`
for the shipped data: KDY and LNR weapons always fire, facilities with attack 0
never). A shot picks index `rand[0, initialAttackers - 1]` over the **current**
attacker list (`FUN_0058bd00`): an index past the survivors is a miss. A hit
kills that regiment (object slot `+0xac`, reason `0xe`) and decrements `+0x24`;
the loop ends when `+0x24` reaches 0 (checked before each facility, so no draw
follows). A facility whose attack is 0 still draws its `[0,99]` percent roll
(`rand < 0` is false). `read`.

#### Step 3: engagements (`FUN_0058bec0`, `FUN_0055eb00`)

The enumerator walks every attacker once, in fleet, ship, regiment order, so
**each surviving attacking regiment engages at most once per assault order**;
there are no rounds.

Per attacker, one draw `i = rand[0, initialDefenders - 1]` (`+0x20`, fixed;
`FUN_0058bec0`), taken for every attacker the walk reaches. If `i <
+0x28` (defenders left) the `i`-th defender of the current defender list is the
opponent (uniform; the list shrinks, so once defenders have died a fraction
`(initial - left) / initial` of the picks miss and do nothing). Then
`+0x2c += 1` and:

```
leadA = leadership (slot +0x1f4) of the fleet's General   (FUN_004fd790(fleet, 3): the first
        usable capital ship of the fleet whose crew holds a CommandKind +0x96 == 3; 0 if none)
leadD = leadership of the system's General   (FUN_00509330(system, 3): the first usable character
        [0x30,0x3c) child of the system, holder side, with +0x96 == 3; 0 if none)
r = rand[0,10] + (leadA / 30 + attacker class +0x64) - (leadD / 30 + defender class +0x68)
attacker dies if r <= 4;  defender dies if r >= 6;  r == 5 stands
```

(`FUN_0055eb00`: G[0x201] = 30 divides leadership, G[0x202] = 10 is the roll
range, G[0x203] = 4, G[0x204] = 6; attacker `+0x64` = `FUN_00520b90`, defender
`+0x68` = `FUN_00520ba0`, the TROOPSD attack and defence; divisions are
truncating.) The `rand[0,10]` is the second draw, taken only on a hit of the
pick; a missed pick draws nothing more and counts no engagement. The
leadership lookups return success even when no General exists (`FUN_00509330`,
`FUN_005006f0` return 1), so the roll always follows. `FUN_0055eb00` sets the
two outcomes from one `r`, so exactly one of them can be set. A death is the
unit's slot `+0xac` with reason `0xe`; the counters `+0x24` / `+0x28` fall; when
either reaches 0 the stop flag ends the walk and no later attacker draws. `read`.

Line cites (`read`): the pick `i = FUN_0053e290(initialDefenders - 1)` and its test against the
remaining defenders, `FUN_0058bec0.c:39-40`; the engagement counter, `:63`; the two leadership
lookups (system General, then the fleet's General), `:64-76`; attacker and defender class values
`FUN_00520b90` / `FUN_00520ba0` (`+0x64` / `+0x68` of the class record, `:85-86`); the roll,
`FUN_0055eb00.c:12-17` (`rand[0, G 0x202] + (leadA / G 0x201 + attack) - (leadD / G 0x201 +
defence)`, attacker dies when `<= G 0x203`, defender dies when `>= G 0x204`); the outcome,
`FUN_0058bec0.c:93-106` (a defender death is read first and takes the unit through slot `+0xac`
with reason `0xe`, an attacker death likewise; a stand returns before the stop flag is set); the
stop flag, `:108-115` (either remaining count is 0). The defender's defence carries no facility,
shield, system or support term: the only terms are the two class values and the two Generals'
leadership. GNPRTB.DAT holds `0x200`-`0x205` as 5, 30, 10, 4, 6, 8 in all eight difficulty
columns, so the draw range does not vary with the setting. The draw `FUN_0053e290(10)` is
`FUN_005f5700(rng, 11)`: `state / (0x7ffffffe / 11)` (`0x7ffffffe / 11` is exact), so it is
uniform over `[0, 10]` (the value 11 needs the state `0x7ffffffe`, one in 2^31). The generator is
the Lehmer step `FUN_005f57a0` (`x * 0xa18e mod 0x7fffffff`); a draw with `n <= 0` is
`FUN_005f5700(rng, 1)` and returns 0 without stepping it, so the single-defender pick draws nothing.
The class values come straight from the file: the TROOPSD class reader `FUN_00590040` (class record
`0x6c` bytes, `FUN_0058ff10`) reads `+0x60` (bombardment), `+0x64` (attack) and `+0x68` (defence) one
after another from the record's last three words, and neither the regiment's save words nor its
`+0x96` strength reach the roll (`FUN_00520b90`/`FUN_00520ba0` read the class through object `+0x2c`).
The regiment destroy slot `+0xac` (`FUN_004f8660` -> `FUN_004f71d0`) stores the cause in `+0x40` and
raises the change slot, and returns nonzero, so a death does not stop the attacker walk.
Odds for Alliance Army (attack 3) first, then five Mon Calamari (attack 2), against one Imperial
Army Regiment (defence 5), no General on either side: the Army kills the defender in 3 of 11
rolls (`X >= 8`) and dies in 7 (`X <= 6`); each Mon Calamari kills in 2 (`X >= 9`) and dies in 8
(`X <= 7`); the system is taken with probability `1 - (8/11)(9/11)^5` = 0.7333.

#### Step 4: collateral (`FUN_0058c580`, `FUN_0055eb80`, `FUN_0055ebc0`)

Rounds = `+0x2c` (the engagements fought). Hits = the number of those rounds in
which `rand[0,99] < G[0x205] = 8`. Per hit, `FUN_0055ebc0(F, H, energy, raw)`:
`F` = active facilities `[0x20,0x30)` of the system (`FUN_00539d70`, both
sides, counted again before each hit), `H` = those in `[0x20,0x22)` (the
Alliance HQ family, `FUN_00526cf0`), `N = F - H`, energy =
system `+0x5c`, raw = system `+0x64`. Total `T = N + (energy != 0) + (raw != 0)`;
`T = 0` ends the loop. `r = rand[0, T-1]`: `r < N` destroys the `r`-th facility
that is not the HQ (reason `0xe`; the HQ id is the first `[0x20,0x22)` child);
`r == N` takes one energy (`FUN_00509cc0`, `+0x5c - 1`) when energy is nonzero,
else one raw material (`FUN_00509dc0`, `+0x64 - 1`); `r == N + 1` takes one raw
material. Collateral is therefore uniform over the non-HQ facilities plus one
slot each for nonzero energy and raw material, one unit of resource per hit, and
the HQ is immune. The `r`-th facility is the `r`-th active `[0x20,0x30)` child
whose id differs from the first HQ's, in container order. A hit with `T = 0`
(`FUN_0058c580`: flags `local_60`, `local_5c`, `local_58` all 0) stops the loop
and draws nothing. The `rounds` percent rolls are all drawn first, then one
`rand[0, T-1]` per hit. `read`.

#### Step 5: landing (`FUN_0058c810`, `FUN_00559fe0(side, 0, 0, 0, 0)`)

Runs only when attackers remain and no defender regiment does. The limit is
`FUN_0055a050(0)`: `ceil((G[7761] - 0) / -G[7762])` = `ceil(60 / 10)` = **6**
(the garrison requirement at zero support; GNPRTB 7761 = 60, 7762 = -10). The
first six regiments in enumeration order (fleet, ship, regiment) are moved into
the system with `FUN_004f6fd0`; the rest stay aboard. Fewer than six aboard:
all land. `read`.

#### Step 6: cleanup

`FUN_0050a6d0(system, 0)` and bit 7 cleared on every attacker-side fleet at the
system: surviving attackers stay aboard and the order is spent. A pass that
leaves both sides alive moves nothing: attackers stay aboard, defenders stay in
place, and nothing repeats it; a second order runs the same pass again. `read`
for the code, `VM` for the day-order timing.

## 4. Control change

The system's child hook `FUN_00508660` (vtable slot `+0xc8`, run by event
`0x301` whenever a child's active bit `+0x50` bit 0 changes, and by event
`0x320` when an active child changes container; `blockade-bit.md`) runs
`FUN_0050b610` for a regiment child (families `0x10..0x13`) with the flag 0 or
1. Each kill clears the regiment's active bit, and a landing moves it into the
system, so the hook fires for every killed defender and every landed regiment
after the run; the handlers read the final state (`FUN_0050b610` is a pure
function of the active regiments there and the old word). The holder is the
side that has active `[0x10,0x14)` regiments there (both sides: the old bits
stay; none: the support-derived side, or 3 for an unpopulated system, bits
2-3; `support-shifts.md`). `FUN_0050a300` stores the word and the system raises
event `0x31b`. The assault never writes the holder directly. Consequences
(`read`): landing on a system whose holder has no regiment there flips it to the
assaulter; landing happens only when no defender regiment is left, so the
both-sides-present case cannot arise from a landing. A pass that kills the last
defender and every attacker leaves no active regiment of either side, bits 4-5
are 0 and the holder becomes the support-derived side (the old holder again
when its support still holds). A pass that leaves a defender standing changes
nothing, whatever the attackers lost.

**Characters.** The holder change reaches the characters through the side
setter, not through event `0x31b`. `FUN_0050a300` stores the control word and
calls system slot `+0x208` (`FUN_00510d70`, vtable `0x0065e640` entry
`0x0065e848`); on a change of the holder bits that hook writes the system's
owner bits (`FUN_004f6f40`, `FUN_00510d70.c:19`), which write them first and
then call slot `+0xf0` (`FUN_00510820`, entry `0x0065e730`) with the old and new
side. `FUN_00510820` starts with `FUN_004fae40`, which calls slot `+0xb0` on
every child of the system (`FUN_004fcd00` mode 3, `FUN_004fae40.c:22-27`):
regiments, facilities, fleets, characters and special forces alike. `read`.

- **Character** (families `0x30..0x3f` minus special forces; slot `+0xb0` is
  `FUN_004ef260` in all nine character classes, base `0x0065ca70`, minor
  `0x00665be0`, major base `0x0066b608`). Not on a mission (`+0x78` bit 7) and
  without `+0x50` bit 3, a character whose container's side bits (`+0x24` bits
  6-7) differ from its own: a prisoner (`+0xac` bit 0) is first released by slot
  `+0x218` (`FUN_004ef5b0`, status 2); if the sides still differ, the autoscrap
  request is set (`FUN_004f78e0(1)`, `FUN_004ef260.c:13-21`). It moves nothing
  itself. `read`.
- **The request** raises event `0x307` (slot `+0x13c`, `FUN_004fba70`) and the
  handler `FUN_00515480` runs the character branch `FUN_00545df0` ->
  `FUN_00548da0`, which calls the adrift sweep `FUN_00548e90` at the character's
  location (`death-star-strategic.md`, "The autoscrap request"). `hyp` that the
  event is synchronous (the same assumption as `system-squadrons-combat.md`,
  3b): the sweep then runs once per flagged character, inside the child walk,
  after the owner bits changed.
- **The sweep** gathers the system's flagged characters and, unless the system's
  battle bit (`+0x88` `0x1000`: fleets or fighters of both sides) is set, runs
  `FUN_00549230` -> `FUN_0054c7d0`. A flagged character off a mission, whose
  container is destroyed or of another side, is grouped by side, adrift flag
  and destination (`FUN_0054c7d0.c:91-99`); `FUN_00543040` takes the
  destination from the retreat chain (`FUN_005545e0`: stage 1 offers the
  character's own container, which `FUN_00553410` now refuses because the system
  belongs to the other side; stage 3 then takes the nearest system of the
  character's side that is not blockaded, `system-squadrons-combat.md`, "The
  retreat-candidate chain"). A group with a free member starts the hidden
  Autorouting mission there. `read` for the chain, `inf` that the nearest
  accepting system is the one reached. A character with a captor id keeps its
  flag and stays. `read`.
- **Special force** (slot `+0xb0` is `FUN_00503d90`, entry `0x0065e210`): not on
  a mission and without bit 3, it takes the container's side. When the class
  serves that side (slot `+0x50` = `FUN_004f27d0`: the class record's side
  fields at `+0x40` and `+0x44`) it changes hands (`FUN_004f6f40`); otherwise
  slot `+0xac(3)` removes it (`FUN_00503d90.c:18-27`). `read` for the code,
  `hyp` for which result a shipped Alliance class takes (the class fields were
  not read).
- **What does not move a character.** The assault run itself never touches one
  (engagements and collateral iterate regiments and facilities only), and the
  battle bit is not set by it (`FUN_00509710` counts fleets and fighters, not
  regiments). The blockade bit (`+0x88` `0x20`) triggers no sweep: only a change
  of the battle bit does (event `0x304`). A system under blockade whose holder
  has no fleet there never has the battle bit, so no sweep runs before the
  capture. No AI or system evacuation order for characters was found: the only
  evacuation text is the player's Move warning (`move-order.md`, blockade
  escape). `inf` (not every caller of `FUN_004f78e0` was enumerated; the ones
  traced are the control-change walk, the container-destroyed slot `+0xb4`
  `FUN_004ef2e0`, the status handler `FUN_004f18e0` and the battle loss of
  `FUN_00516ef0`).
- **Emperor flag.** Event `0x31b` (`FUN_00515c00`) calls the character
  manager's `FUN_005613f0` -> `FUN_00561670`, which acts only for the system with
  `+0x18 == 0x109` (Coruscant): the Emperor-at-Coruscant flag. The other
  consumer of `0x31b` is the support shift (`FUN_00517aa0`).


## 5. Casualty accounting

Reason `0xe` is the argument of slot `+0xac` in `FUN_0058c3c0`, `FUN_0058bec0`
and `FUN_0058c580`, so it is this run's casualty reason (regiments and
facilities); the blockade-end loss counter A (`support-shifts.md`) adds the dead
unit's value to `A[owner side]` for exactly this reason. `read`.

What a kill is, per unit kind (slot `+0xac(0xe, ctx)`, the object's destroy):

| Unit | Effect | Counter A |
|---|---|---|
| Attacking regiment (aboard a ship) | leaves the ship and the arena; the ship's regiment count falls | yes, the owner's side: event `0x300` carries family `0x10..0x1f` with the destroyed bit (`FUN_00508c60`, `support-shifts.md`) |
| Defending regiment | leaves the system and the arena | yes, the defender's side |
| Defence or civilian facility (collateral) | leaves the system and the arena | no: the family test in `FUN_00508c60` is `0x10..0x1f` |
| Energy or raw material | system `+0x5c` or `+0x64` falls by 1 (`FUN_00509cc0`, `FUN_00509dc0`) | no |
| Alliance HQ | never chosen | no |
| Special forces, characters, fleets, ships | never chosen | no |

The counter is credited against the container's system and only while the
blockade bit holds (`FUN_00511300` zeroes A and B when it is set). The assault
requires the blockade, so every kill of a run is counted. `read` for the
callers and the table, `inf` for the container resolving to the assaulted
system (the regiment's ship orbits it).

## Draw ledger

One run draws from the session generator in this order (`FUN_0053e290(n)` =
`rand[0, n]`; `FUN_0053e2f0(p)` = `rand[0,99] < p`). `A0` = initial attackers,
`D0` = initial defenders, `a` = attackers left, `d` = defenders left. `read`.

1. Defence fire, per active facility of `[0x22,0x28)` in container order while
   `a > 0`: `rand[0,99]` against `attack / 5`; when it fires, `rand[0, A0-1]`.
2. Engagements, per attacker in (fleet, ship, regiment) order while `a > 0` and
   `d > 0`: `rand[0, D0-1]`; when the index is below `d`, `rand[0,10]`.
3. Collateral: `E` times `rand[0,99]` (`E` = the engagements fought), then per
   hit `rand[0, T-1]` while `T != 0`.
4. Landing and cleanup draw nothing.

The system's holder side, the leaderships and the class stats are read once per
engagement and change nothing between draws except `a` and `d`.

## 6. The AI's assault order

`FUN_004b1070` (the posture emitter, `ai-order-emission.md`): for a fleet
with need bit `0x20000000` that has arrived (`+0x28 == +0x2c`): fleet flag
`0x10` and system record `+0x110 > 0` gives Destroy System `0x223`; otherwise
`FUN_004ae240`; a pass builds `FUN_004b1330(fleet, 0x234)` (the order from the
registry `FUN_004f5cd0(0x234)`, the fleet as team, the target `+0x2c`); a fail
tries `FUN_004ae320` (a bombardment kind). Need bit `0x40000000` never issues
`0x234`.

`FUN_004ae240(this, fleet record, system record)` passes when all hold:

- system record `+0x30 & 0x101 == 0` and `& 0x20 != 0` (unlabeled record bits);
- required strength `R`: when the system's own id equals the fleet's `+0x28`
  (already there) `R = 0`, else `R = max(system +0xb4, context +0x168)` and
  `Q = system +0xa4`; fleet `+0x5c >= R`;
- fleet `+0x80 >= system +0x118` (`inf`: regiments carried against regiments
  needed);
- fleet flag `0x10` set (`+0x38 & 0x10`): `Q <= fleet +0x74`; otherwise
  `fleet +0x9c > 0` or `Q <= fleet +0x74`.

The record field meanings are `inf` (`ai-order-emission.md` leaves them open).
The gate does not mirror the order's refusals (blockade, two shields); a
refused order returns its status and the need persists while the gate passes.
`read` for the shape, `inf` for the meanings.

## 7. Landing without the order

None found. The landing callback `FUN_0058be10` is reached only from the
assault manager (`FUN_0058bdb0`, one caller `FUN_0058c810`); `FUN_0058ba10` has
only the four assault steps as callers; `FUN_00559fe0` has two other callers
(`FUN_0050b5a0` the stored garrison requirement, `FUN_00559fb0`) and neither
moves a unit. Other relocation of a regiment is the generic move into a
container (`FUN_004f8630` slot `+0xa8` -> `FUN_004f6fd0`) by a Move command
(`regiment-unload.md`: refused on an enemy or neutral populated planet,
allowed on an own one or an unpopulated neutral one; at an unpopulated enemy
planet it boards a friendly ship there with room). The AI's Drop (`FUN_004ae440`: system record bit
2, not `8` or `0x200`, fleet `+0x80 > 0`; `FUN_004b1440`) issues order `0x201`
Move for the first regiment aboard the first loaded ship toward the fleet's
target: one regiment per call, under the same refusals. There is no unopposed
automatic drop: an undefended enemy system still needs the assault order, which
then lands up to six regiments (section 3, step 5). `read` for the callers,
`inf` for the absence of other slot-`+0xa8` callers.

## Port-facing facts

- One engagement per attacking regiment per order, a uniform random defender,
  no rounds, `rand[0,10]` plus the strength difference against 4 and 6.
- Defence-facility fire is `attack/5` percent: shipped weapon facilities always
  fire once each, shield generators never.
- Landing is up to six regiments, only after the last defender regiment dies.
- Enable needs a blockaded system (fleet `+0x58` bit `0x20`), a regiment aboard
  and fewer than two family-`0x24` facilities at the system.
- A holder change, by any cause, runs the child walk of section 4:
  `support_shift::move_enemies` releases and flags characters and turns or
  removes special forces, and `death_star::sweep_requests` sends the flagged
  characters off (`integrator::sweep_requests` after each operation).

## In the original (VM)

- `empire-day128-assault-fails-selonia.sav`: Fleet 1 (an Imperial Star Destroyer, a Carrack, one
  Stormtrooper Regiment) assaults Alliance-held Selonia (two Alliance Army Regiments, one GenCore
  Level I). The report opens at once with `0x7092`; one Alliance regiment is destroyed, the
  Stormtrooper Regiment stays with the fleet, no Imperial loss.
- `alliance-day1169-deathstar-trench-run.sav`, day 1171: the Alliance fleet in orbit of
  Imperial-held Ord Pardon (no regiment, defence facility or character there) assaults with its
  six regiments (Alliance Army, Mon Calamari and others). The report opens at once in the
  Alliance art with picture `0x2b98` and `0x7091` "Alliance troops have taken control of the
  Imperial system Ord Pardon"; no casualties on either side; its System Forces emblem is the
  Imperial one, the side that held the system before the run.
- `empire-day101-assault-ready-drall.sav`: until the Battle at Drall alert opens on day 110 (an
  Alliance fleet intercepting), Fleet 1's menu greys every row but Encyclopedia and Status. After
  the auto-resolved battle the fleet keeps its ships and regiment, Planetary Assault is enabled,
  and the assault on bare Drall (no defence facility, regiment or character) opens `0x7091`
  "Imperial troops have taken control of the Alliance system Drall" at once.
- `alliance-day817-orto-transit.sav`: Fleet 1 (a Corellian Corvette, a Medium Transport, two
  Alliance Fleet Regiments) is en route to Imperial-held Orto (three GenCore Level I shields, a
  KDY-150, an LNR Series I). While en route its sector icon's menu greys every row but
  Encyclopedia. In orbit (day 911), Planetary Assault and Status are greyed; Move, Confirmed
  Move, Encyclopedia, Scrap and Planetary Bombardment are enabled, and the bombardment submenu
  enables Target Military Facilities, Target Civilian Facilities and General Bombardment and
  greys Destroy System.
- `empire-day211-tralus-corellia-assaults.sav` (from `empire-day128-assault-fails-selonia.sav`, Fleet 1
  ordered to Corellia and Fleet 2 to Tralus on day 131):
  - Tralus (Alliance-held; one KDY-150, no shield, regiment, fighter or fleet), day 189: Fleet 2 (two
    Galleons and Imperial Fleet Regiments; Planetary Bombardment greyed, Planetary Assault enabled)
    assaults. The report reads `0x7091` "Imperial troops have taken control of the Alliance system
    Tralus"; the KDY-150 killed one Imperial Fleet Regiment, the rest landed, and the KDY stays
    operational.
  - Corellia (Alliance-held; one KDY-150, no shield, regiment, fighter or fleet), day 210: Fleet 1
    assaults with its one Stormtrooper Regiment. The KDY-150 kills it, so no attacker remains, and the
    report reads "Alliance Troops have defended Corellia from an Imperial assault". The system's
    Personnel tab (Survivors, Captured, Killed) is empty.
- `empire-day211-tralus-corellia-assaults.sav`, continued to day 277: Alliance-held Talus holds three
  Alliance officers (family `0x38`, ids 844, 855, 857, location word Talus `0x90000092`) and one
  Alliance special force (`0x3c`, id 2418) on day 220, with Imperial Fleets 1 and 2 in orbit. On day
  272 Fleet 1 runs Planetary Assault and takes Talus (`0x7091`; the Alliance System Forces Personnel
  tab reads Survivors None, Captured None, Killed No Casualties). On day 277 all three officers sit
  at Alliance-held Corfai (`0x9000008e`), not travelling; for officer 844 the location word changed
  Talus -> Corfai, and three other words changed (0x200 -> 0x300, 3 -> 6, 0x80000 -> 0x40000). The
  save pair does not date the departure inside days 220-272; the code puts it at the capture
  (section 4, "Characters").
- `empire-day488-late.sav`, day 493: Fleet 2 in orbit of Alliance-held Duros (no defence facility or
  regiment) assaults and takes it (`0x7091`); the Personnel tab lists no survivor, capture or kill.
- Under Wine, `alliance-day1169-ord-pardon-garrison.sav` (one Imperial Army Regiment, defence 5,
  moved into Imperial-held Ord Pardon): the Alliance fleet (Alliance Army
  Regiment, attack 3, then five Mon Calamari Regiments, attack 2; no General on either side)
  assaults. A capture reads "Alliance troops have taken control of the Imperial system Ord Pardon",
  a repulse "Imperial Troops have defended Ord Pardon from an Alliance assault."; both show the
  Alliance picture `0x2b98`. From `alliance-day1169-ord-pardon-dark-trooper.sav` (a Dark Trooper
  Regiment, defence 8, instead) every assault is
  repulsed and all six attackers die.
- The session generator's state (`*DAT_006b9054 + 4`) is restored by a load and holds between
  draws: from the garrison save it reads `0x2c552f70` through day 1169 and `0x58aa06a7` from the first
  step of day 1170 (six draws later). An order resolved on day 1170 starts from `0x58aa06a7`; the
  ledger's rolls (`rand[0,10]`, `state / (0x7ffffffe / 11)`) give 2, 6, 2, 4, 2, 6, every attacker
  dies, and the generator ends at `0x3611f6e0`, 12 draws on (six rolls, six collateral rolls).
  An order resolved before the day's draws starts from `0x2c552f70`; the rolls 3, 3, 6, 10 kill
  the defender in the fourth engagement and the generator ends 9 draws on (four rolls, four
  collateral rolls, one hit). Both outcomes are the ones the draw ledger predicts, so outcomes
  repeat for every order resolved under the same state: repeated orders within one day are one
  sample, not many.

## Open / needs VM

- The day-order timing of the run, and the shown text of a `0x90` refusal.
- `FUN_0041e580` (another consumer of `0x183` / `0x150`) is unread.
- Characters on a captured system: the day of the officers' departure (capture day or earlier), the
  fate of special force 2418, and the meaning of officer 844's words 3, 10 and 11 are not observed.
- An assault with an enemy fleet present cannot be ordered: the enable test needs the fleet's
  blockade bit (`FUN_004fddd0`, `8`/`7`), which `FUN_0050c0b0` never sets while the holder has an
  active fleet or fighter squadron there.
- The order of event `0x301` / `0x320` handlers against the run's remaining
  steps (only matters if a handler reads a count the run still changes; the
  holder rebuild reads active regiments, which no later step changes).
- The `+0x20c` slot number of the fleet hook and the argument order of the
  message slots.

## Decompiled files

`FUN_00563390`, `FUN_0058c1e0`, `FUN_0058c120`, `FUN_0058c2d0`, `FUN_0058c3c0`,
`FUN_0058c500`, `FUN_0058c580`, `FUN_0058c810`, `FUN_0058c8b0`, `FUN_0058bec0`,
`FUN_0058ba10`, `FUN_0058bbd0`, `FUN_0058bc90`, `FUN_0058bd00`, `FUN_0058bdb0`,
`FUN_0058be70`, `FUN_0058bce0`, `FUN_0058bd60`, `FUN_0058be10`, `FUN_0058c0d0`,
`FUN_0055ead0`, `FUN_0055eb00`, `FUN_0055eb80`, `FUN_0055ebc0`, `FUN_0055ea00`,
`FUN_00559fe0`, `FUN_0055a050`, `FUN_0055a0b0`, `FUN_00558bb0`, `FUN_0053e290`,
`FUN_0053e2e0`, `FUN_0053e2f0`, `FUN_005f5700`, `FUN_00520b70`, `FUN_00520b90`,
`FUN_00520ba0`, `FUN_00504cc0`, `FUN_00504c40`, `FUN_00502e30`, `FUN_00502db0`,
`FUN_004ffef0`, `FUN_00513050`, `FUN_00513090`, `FUN_00513120`, `FUN_005131d0`,
`FUN_00526fd0`, `FUN_00526cf0`, `FUN_00539d70`, `FUN_005278c0`, `FUN_00509330`,
`FUN_004fd790`, `FUN_0050a6d0`, `FUN_00509cc0`, `FUN_00509dc0`, `FUN_004fe4d0`,
`FUN_004fe3f0`, `FUN_0050c0b0`, `FUN_0050b610`, `FUN_0050a300`, `FUN_00564b30`,
`FUN_00515c00`, `FUN_005613f0`, `FUN_00561670`, `FUN_004f6fd0`, `FUN_004f8630`,
`FUN_005814d0`, `FUN_00581ae0`, `FUN_005815f0`, `FUN_005816f0`, `FUN_00581850`,
`FUN_005817e0`, `FUN_0057f970`, `FUN_0057fac0`, `FUN_0057fb90`, `FUN_0057fc60`,
`FUN_00507d00`, `FUN_00507e60`, `FUN_004fddd0`, `FUN_004fdec0`, `FUN_004feb60`,
`FUN_004ff600`, `FUN_004ff890`, `FUN_00512370`, `FUN_00496770`, `FUN_00553770`,
`FUN_0051f4b0`, `FUN_00563500`, `FUN_004b1070`, `FUN_004b1330`, `FUN_004b1440`,
`FUN_004ae240`, `FUN_004ae440`,
`FUN_004ef260`, `FUN_004fae40`, `FUN_00510820`, `FUN_00510d70`, `FUN_00503d90`,
`FUN_004f78e0`, `FUN_00515480`, `FUN_00545df0`, `FUN_00548da0`, `FUN_00548e90`,
`FUN_0054c7d0`, `FUN_00543040`.
