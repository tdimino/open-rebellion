---
title: "Tactical results: the Battle Summary's per-tab condition rules"
description: "How the end-of-battle Battle Summary (window kind 8) files each capital ship, squadron, regiment and character as operational, destroyed, captured or killed"
category: "ghidra"
created: 2026-10-17
updated: 2026-10-17
tags: [tactical, battle-summary, results, squadrons, death-star]
---

# Tactical results

Companion to `battle-result-text-and-cadence.md` (the result object's text),
`battle-alert-lists.md` (the pre-battle lists) and `tactical-death-star.md`
(the import, 11). The port's side is
`crates/rebellion-render/src/tactical_view.rs` (`battle_result_entries`).
Recovered from the owned English `REBEXE.EXE` (SHA-256
`b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`).

## 1. The window and its tabs (read)

The results box is the Battle Summary: window kind 8, 470x331, `FUN_0046f140`
(`FUN_0042a640.c:27`, reached from the message pump `FUN_00488b90` through
`FUN_0041d730`). Its data object (`+0x174`) is the battle result object
(`FUN_0049ac00`; `battle-result-text-and-cadence.md`). It has four tabs, ids
`0x65`-`0x68`, text `0x1928`, `0x1929`, `0x1930`, `0x1931`
(`FUN_0046f360.c:651-730`), and two side buttons, `0x70` and `0x71`
(`FUN_00471340.c:19-42`).
`FUN_004714e0(window, tab, 0)` refills the list (`FUN_004714e0.c:91-135`) and
fixes the family range each tab shows:

| Tab | Id | Family range | Columns (`FUN_0046f360.c:314-322`, `350-363`) |
|---|---|---|---|
| Capital ships | `0x65` | `[0x14,0x1c)` | Operational `0x1932`, Destroyed `0x1933` |
| Fighter squadrons | `0x66` | `[0x1c,0x20)` | Operational, Destroyed |
| Trooper regiments | `0x67` | `[0x10,0x14)` | Operational, Destroyed |
| Personnel | `0x68` | `[0x30,0x40)` | Survivors `0x1934`, Captured `0x1935`, Killed `0x1936` |

There is no Death Star tab and no Damaged column. The Death Star is a ship of
family `0x18` and sits on the capital-ship tab. Side button `0x70` selects side
1 with the result object's list `+0x78`, `0x71` side 2 with `+0x80`
(`FUN_004714e0.c:80-90`). An empty column shows one row: the first column
"None" (`0x8526`) when the Destroyed and Captured columns are empty too, else
"No Survivors" (`0x1940`); Captured "None"; the last "No Casualties"
(`0x1941`) (`FUN_004714e0.c:315-392`).

### Layout (read)

`FUN_0046f360` builds four layers over the shell (`0x285f` side 1, `0x2860`
otherwise) and the rail art (`0x2a44`/`0x2a45` at x `0x19c`), each with the
title (`+0x88`, font `0xb`, centred in 400 at (`0xc`, `0x12`); red `0xff`
for a side-1 player, green `0xff00` otherwise):

| Layer | Picture | Text |
|---|---|---|
| 1 Summary | `0x2d63` with no victor (`+0x50` 0); the Empire's win `0x2a05` when the Alliance withdrew (`+0x54` 1), else `0x2a08`; the Alliance's `0x2a06` when the Empire withdrew, else `0x2a07`; alike for either player (`:68-109`) | `+0x94` in font `0xb`, centred in 400 at (`0xc`, `0xe1`); `+0xa0` in font 6, left, wrapped in `0x160` at (`0x18`, `0xff`) over a black copy at (`0x19`, `0x100`) (`:226-244`) |
| 2 forces | `0x2a09` | Operational `0x1932` and Destroyed `0x1933`, font 4, white, centred in `0xa5` at x `0x25` and `0xcc`, y `0x76`; "Filters" `0x1952` left at (`0x56`, `0x46`) (`:305-326`) |
| 3 Personnel | `0x2a0a` | Survivors, Captured, Killed centred in `0x6e` at x `0x25`, `0x95`, `0x104`; "Filters" (`:343-368`) |
| 4 Display | `0x2a0b` | `+0xac`, `+0xb8` (`:259-283`) |

The rail (group at (`0x1a2`, `0x58`) or (`0x1aa`, `0x59`)) holds `0x6f`
Battle Summary (`0x1922`), `0x70` Alliance Forces (`0x1937`), `0x71`
Imperial Forces (`0x1938`) and `0x72` Display (`0x1939`); Close is `0xc9`
(`:399-569`, `:737-769`). The filter group sits at (`0x82`, `0x3b`), its tabs
at x 0, `0x31`, `0x62`, `0x93`, `0x31` by `0x2a` (side 1) or `0x2c` by
`0x29`, with the Operation Report's tab art: `0x286c`/`0x286b`,
`0x2872`/`0x2871`, `0x2870`/`0x286f`, `0x286a`/`0x2869` for side 1,
`0x2878`/`0x2877`, `0x287c`/`0x287b`, `0x287a`/`0x2879`,
`0x2876`/`0x2875` otherwise (`:609-735`). The list is at (`0x25`, `0x88`),
`0x15b` by `0xa6`, with the bar of RCDATA `0x299d` (`:588-607`).

Over a forces view `FUN_00471200` paints the filter's name at (`0x25`, 99),
font 4, white, and the view's name centred in 400 at (`0xc`, `0x28`), font
6, red for `0x70` and green for `0x71`. `FUN_004710f0`: Escape closes;
Left and Right step to the neighbouring visible filter, else the group's
first. A cell (`FUN_004724a0`) centres its picture
(`FUN_0042c3b0(.., tab != 0x68, 1)`) and puts the name, font 10, under it;
an empty text sits at y 10 in font 6. `FUN_0042c3b0.c:350-356` adds GOKRES
`(id & 0xfff) + 0x2000`, the burn marks, when `+0x50` bit `0x200` is set. Both the marks and
the filter tabs blit keyed (`FUN_005fd0f0`, `FUN_00602d30`) on the first
stored pixel's index (`FUN_005fbd20` -> `FUN_005fd170`): 252, palette blue,
in all 38 marks `0x2640`-`0x278f` and in the Personnel tabs' matte. A tab
paints only inside its button: the Personnel art is `0x31`/`0x32` by
`0x39`, taller than the button.

### In the original (VM)

Empire game, Battle at Yaga Minor, the Imperial fleet victorious and the
Alliance fleet withdrawn: the Summary shows `0x2a05` (an X-wing and a Mon
Calamari cruiser jumping out), the green status line centred under it and
the body left-aligned below. inf: `+0x50` is the victor's side and `+0x54`
the side that withdrew. `0x2a08`'s own art puts a Star Destroyer's white
engine glows at its lower left, under the status line.

Empire game, day 676, Battle at Coruscant, the Imperial fleet destroyed: the
result box ("Battle at Coruscant", "The Imperial fleet is defeated.", then
the cause) has the rail at its right. Its Imperial emblem opens "Imperial
Forces" with "Filters" and the four icon tabs (ship, TIE, trooper,
officer). On Capital Ships, Operational reads "No Survivors" and Destroyed
lists Imperial Star Destroyer 1, Galleon 2 and the Death Star, each picture
with burn marks, in a scrolling list. Fighter Squadrons lists the TIE
Fighter squadrons destroyed, burned the same way.

### Manual (read-in-full, pp. 152-153)

| Claim | Status | Port |
|---|---|---|
| The window comes up at the end of every battle, simulated too (p. 152) | Unresolved: the result event's path (2) is hyp | Present: both paths open it |
| Rail: Close, Battle Summary, Alliance Forces, Imperial Forces, Go directly to (Fig. 4.17) | Confirmed (`FUN_0046f360.c:399-569`) | Present |
| Tabs for capital ships, fighters, troops, personnel (Fig. 4.18) | Confirmed (`0x65`-`0x68`) | Present |
| Operational ships carry burn marks when damaged (Fig. 4.18) | Confirmed (`FUN_0042c3b0.c:350`, the setters in 3) | Present |
| Troops or personnel may be injured, captured or killed with their ship (p. 153) | Regiments destroyed, characters wounded or killed: confirmed (3); captured: contradicted, the battle sets no captive bit | Present; the wound is not stored (F-026) |
| Display offers the Fleet window unless the fleet was destroyed (p. 152) | Confirmed (`FUN_004f36a0` gates button `0xca`) | not re-rated here |

## 2. How an entry is filed (read)

Two sources feed each side's columns.

**Operational (and Survivors, Captured).** For each fleet of the side at the
system, `FUN_00472010(fleet, list, captured, tab)` walks the fleet's ships
(`FUN_00502db0`, kind 3) and keeps a ship whose `+0x50` has bit 2 set and bit
`0x20` clear (`FUN_00472010.c:42`). By tab it then lists:

| Tab | Entries | Test |
|---|---|---|
| `0x65` | the ship | the ship test above |
| `0x66` | the ship's squadrons (`FUN_005039d0`) | child `+0x50` bit 2 |
| `0x67` | the ship's regiments (`FUN_00504c40`) | child `+0x50` bit 2 |
| `0x68` | the ship's characters (`FUN_00536da0`) | character `+0xac` bit 0 set goes to Captured, else Survivors |

The fleets come from two places: the side's fleets at the system
(`FUN_004ffef0`, `FUN_004714e0.c:163-172`) and the result object's fleet list
`+0x60` (`:152-162`). On the Fighters tab the system's own squadrons
(`FUN_00503a50`, `+0x50` bit 2, `:175-206`) and the result object's squadron
list `+0x68` (`:208-236`) follow, and on Personnel the list `+0x70`
(`:241-270`), each side-checked (`+0x24 >> 6 & 3`) and unconditioned by
condition.

**Destroyed (and Killed).** The result object's `+0x78` (side 1) or `+0x80`
(side 2), filtered to the tab's family range (`FUN_004714e0.c:275-300`).
`FUN_0049b720` fills it when the result object is built: it walks the Battle
Alert's two side lists (`+0x6c`, `+0x74`: every fleet, ship, squadron,
regiment and character recorded at battle start, `FUN_0049c0d0.c:70-155`,
`FUN_0049c5d0`), looks each object up (`FUN_004f2d10`), and files it by its
`+0x50`:

| `+0x50` | Filed |
|---|---|
| bit 3 (destroyed) set | the side's destroyed list (`+0x78`/`+0x80`), any family (`FUN_0049b720.c:42`, `130-141`, `174`, `262-273`) |
| bit 3 clear, bit `0x20` (in transit) set | fleets (family `[8,0x10)`) into `+0x60`, squadrons `[0x1c,0x20)` into `+0x68`, characters `[0x30,0x3c)` into `+0x70` (`:43-125`) |
| both clear | not listed here (the system walk above finds it) |

inf: bit `0x20` is the in-transit bit (`blockade-troop-withdrawal.md` 4), so
the object lists hold what withdrew. `FUN_0049ac00` builds the result object
from the stored Battle Alert object (`FUN_00489740.c:29`, `FUN_0049ac00.c:56`).
That call follows the strategic import: `FUN_0040a050` moves from state
`0x14`/`0x15` to `0x16`/`0x17` once `FUN_0059a530` reports the battle over,
runs the import there (`FUN_0040c780`, `FUN_0040a050.c:243`, `:260`) and only
then goes to state 9 (`:268`; from `0x16` when the import leaves `local_b0`
at 0, `:251`), whose `FUN_0041e270` raises the result
event (`FUN_00517270` -> `FUN_00517ee0`; `simulate-runs-tactical-engine.md`).
So every flag below is already written, and the Killed column holds the
characters the import's crew pass destroyed. Port: a played battle shows its
results before the write-back, which runs at return
(`tactical::apply::apply_results`); the crew pass is drawn once when the
battle ends (`BattleSession::settle_crew`), so the summary and the write-back
agree.

## 3. What sets "destroyed" per kind (read)

The import (`FUN_0040c780`, `tactical-death-star.md` 11) writes the tactical
outcome back; slot `+0xac(0xc)` sets `+0x50` bit 3 once the strategic
damage equals the object's full size (`FUN_00503370.c:21-26`,
`FUN_00501ae0.c:25-31`).

| Kind | Writer | Destroyed when |
|---|---|---|
| Squadron | `FUN_0040d260` -> `FUN_00503370` | the record's craft count is 0. The count (`FUN_005b8e00`, instructions `0x5b8e29`-`0x5b8e56`) is 0 only in state `0xb`, else `max(1, ftol(strength / 8.0))` (`_DAT_0066c6b8` = 8.0). A squadron docked (states 1, 5) or jumped (state 9) keeps count 1 or more, so it is Operational |
| Ship | `FUN_0040cfc0` -> `FUN_00501ae0` | the record's hull value (`FUN_005b0280`: 0 in state `0xb`, else the hull, floored at `_DAT_0066c488` = 1.0) leaves damage equal to the hull. A live ship never reads 0, so only state `0xb` destroys it (`FUN_0040cfc0` `0x40d0ad`: `FLD` of the record's float, no scaling) |
| Death Star | `FUN_0040d170` -> `FUN_00501ae0` | the station snapshot (`FUN_005ba620`) hull is 0; its alive flag is `hull > 0` (`tactical-death-star.md` 11) |
| Regiment | no import | with its ship: the ship's hook `+0x11c` (below) calls each child's slot `+0xb4`, which for a regiment (vtable `0x0065e438`) and a squadron (`0x0065def0`) is `FUN_004f8710`: a child complete and not in transit is destroyed with the ship's reason (`FUN_004f7ed0`) |
| Character | no import | by the ship's hook `+0x11c`, below; captured is `+0xac` bit 0, which the battle never sets |

### The lost ship's characters (read)

A ship the import destroys (reason `0xc`, `FUN_0040cfc0.c:88`,
`FUN_0040d170.c:47,56`) runs its hook `+0x11c` from the destroyed-bit
setter (`FUN_004f71d0` stores the reason at `+0x40`, `FUN_004f7560.c:21`).
Both capital-ship classes, `0x0065d650` (family `0x14`) and `0x0065d930`
(the Death Star, `0x18`), hold `FUN_00502840` there:

1. `FUN_00557b90` -> `FUN_005582b0` -> `FUN_004f2b50` -> `FUN_004fb520`
   calls slot `+0xb4` on every child. A character's is `FUN_004ef2e0`: when
   its container's reason is `0x14` (scrap) it requests autoscrap
   (`FUN_004f78e0(1)`), otherwise it sets the Adrift role
   (`FUN_00534790(1)`, event `0x30e`). Nothing here destroys or captures.
2. With the reason in 1..`0x10`, each character aboard not in transit
   (`+0x50` bit 5 clear; `FUN_004f25c0`, families `[0x30,0x3c)`) takes slot
   `+0x2e8`, `FUN_004ef640` in every character vtable: `FUN_0053ea00`
   rolls `FUN_0053e2f0(max(G2568, 100 - combat))`, combat being slot
   `+0x1f0` (`FUN_004edc40`, short `+0x86`), and on a hit wounds it by
   `rand(chance) + rand(G2570) + G2569` (GNPRTB `0xa08`-`0xa0a`,
   `DAT_006b90a8`, `DAT_006b9098`, `DAT_006b90a4`, shipped 1, 29, 1), then
   slot `+0x2f0` (`FUN_004ef6d0`) adds the wound and destroys the character
   with reason 10 when slot `+0x2e0` holds: `FUN_006158b0` (0) for every
   major, `FUN_0056f160` -> `FUN_0053eb30`, `FUN_0053e2f0(G2577)` (`0xa11`,
   shipped 25), for a minor (`victory-flag-word.md` 3).

So a space battle kills characters: a minor aboard a lost ship dies on the
G2577 roll after a wound; a major is only wounded. The survivors are adrift
for the sweep (`death-star-strategic.md`, "When a Destroy System sweeps its
adrift characters"). Read in full for the slot-`+0x2f0` callers: a binary
sweep of `CALL [reg + 0x2ec]`, `+0x2f0`, `+0x2e4` and `+0x2e8` finds
Assassination (`FUN_00576620`), the uprising incident (`FUN_0050d150`,
`FUN_00510760`, `FUN_004ef450` via `+0x2e4`), the Luke encounters
(`FUN_00549a20`, `FUN_00560200`, `FUN_00561b60`), Destroy System
(`FUN_0058d990`) and this hook (`0x005028d9`, `+0x2e8`) as the only callers.

### The burn marks' bit (read)

`+0x50` bit 9 (`0x200`) has one setter, `FUN_004f7800`, called by
`FUN_00501c00` (capital ship slot `+0x298`: hull damage or any of the six
damage counters, `ship-damage-repair.md`), `FUN_00503340` (squadron slot
`+0x25c`: its squad-size damage, slot `+0x254`, the `+0x60` that
`FUN_005032c0` writes, is not 0), `FUN_004feab0` (a fleet: any ship with
the bit) and `FUN_004f1720`. The import writes the hull, the counters
(`FUN_0040cfc0`) and the squad-size damage (`FUN_0040d260` from the craft
count), so a ship is burned when the battle left it short of its class hull
or with a hit system, a squadron when it lost craft, and every destroyed
one is (its damage is its whole size).

State `0xb` is set by `FUN_005b2d20` (a squadron destroyed outright) and
`FUN_005af7a0` (100000 damage, `simulate-runs-tactical-engine.md`).
Damage that stops short of that only lowers the count or the hull; no
condition below Operational exists for it.

## 4. The port

`rebellion_core::battle_summary` builds each side's lists;
`rebellion-render/src/tactical_view.rs` (`draw_battle_result_forces`,
`battle_summary_cells`) draws them in the window above.

| Rule | Port | Against the original |
|---|---|---|
| Record | `BattleRecord::record` on the battle's first frame: each session ship's tag, the Death Star's entry, every fleet's regiments (with their carrier tags) and its crew (`battle_summary::crew`) | the Battle Alert's lists (`FUN_0049c0d0`) |
| Capital ship | destroyed unless `(alive or retreated) and hull > 0`, the write-back's test; burned below the class hull or with a hit system | matches (3) |
| Death Star | on Capital Ships; destroyed when `trunc(hull) <= 0` | matches (3) |
| Squadron | destroyed at `squad_count == 0`; the live count is `max(1, trunc(hull / 8))` (`refresh_craft_count`); burned below its class's squadron size | matches `FUN_005b8e00`, `FUN_00503340`; port: a squadron enters at full size |
| Regiment | dies with the session ship that carried its tag | matches (3) |
| Character | `battle_summary::crew_fates`, which `apply_results` applies first: a lost ship's character (it rides `carriage::character_ship`) goes adrift and takes the wound roll, a minor dying on G2577 (`mark_killed`); an emptied fleet's characters stay adrift at the system. The window files Killed from the same draws, a captive aboard a living ship Captured, the rest aboard Survivors, the adrift nowhere | matches (3); port: the rolls draw the battle's stream (the write-back holds no strategic one); the wound is not stored (F-026) |
| Operational | the recorded objects that live on | the system walk lists every live object of the side at the system (`FUN_004ffef0`), not only those that fought |

## Open

- Squadrons docked in a carrier the tactical battle destroys: pass 2
  (`FUN_0040d320`) re-parents a squadron only when its tactical carrier record
  is found, so whether such a squadron survives is untraced.
- The row text for a squadron or ship cell is `FUN_004724a0`; its layout is
  not recorded here.
