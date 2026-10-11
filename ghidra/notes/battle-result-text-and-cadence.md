# Battle Result Text and Re-Battle Cadence

> **Corrected 2026-10-10:** `+0x58` is the player's side and `+0x5c` the
> holder of a system already under blockade; the branch tables below read
> consistently only that way ("defended" needs the holder to be the
> player's side). The port follows `battle_result_summary_lines`.

Recovered 2026-10-10 from REBEXE.EXE with Homebrew Ghidra 12.1.3 (read-only,
no analysis). Companion to `system-squadrons-combat.md`,
`battle-alert-lists.md`, `message-display.md`. The port's side is
`crates/rebellion-render/src/tactical_view.rs` (`battle_result_summary_lines`,
`draw_battle_result_summary`) and `crates/rebellion-data/src/simulation.rs`
(`combat_is_on_cooldown`, `COMBAT_RETRY_TICKS`).

## 1. Battle Result Text

### The result object (`FUN_0049ac00`)

The battle result object (vtable `PTR_FUN_0065bc08`) carries:

| Offset | Type | Content |
|---|---|---|
| `+0x20` | int | player side (1 Alliance, 2 Empire) |
| `+0x44` | DatId | battle system |
| `+0x48` | DatId | retreat destination system |
| `+0x4c` | DatId | captured character (hyp:) |
| `+0x50` | int | unknown flag |
| `+0x54` | int | retreat flag (0 = destroyed, nonzero = retreated) |
| `+0x58` | int | the player's side (the constructor's `param_2`) |
| `+0x5c` | int | the holder, when its system was already under blockade (event data `+0x4c`) |
| `+0x60..+0x80` | lists | combatant containers (per family, per side) |
| `+0x88` | text | title |
| `+0x94` | text | status line ("victorious" / "defeated" / "indecisive") |
| `+0xa0` | text | body (system status, fleet fates) |
| `+0xac` | text | message index text |
| `+0xb8` | text | navigation prompt |

`FUN_0049ac00(this, outcome, player_side, event_data)` constructs the object:
sets `+0x58` from `player_side`, populates combatants via `FUN_0049b720`,
then calls `FUN_0049b0f0(this, outcome)` which hyp: dispatches to one of
three text builders based on the outcome code.  `FUN_0049b0f0` is not in the
decompiled corpus.

### The three text builders

Each builder calls `FUN_0060b9d0` (the TEXTSTRA RCDATA formatter from
`message-display.md`) to fill the five text buffers.

#### `FUN_0049b330` — player won

| Buffer | RCDATA | Template text |
|---|---|---|
| title +0x88 | `0x7001` | "Battle at \|system\|" |
| status +0x94 | `0x7002` | "The \|winner_adj\| fleet is victorious." |
| message +0xac | `0x7012` | "The \|winner_adj\| fleet is victorious. The world \|system\| is now under \|winner_adj\| blockade." |
| nav +0xb8 | `0x7013` | "Select one of the following buttons to close this display and go directly to..." |

Body +0xa0 — system status (branched on `system[9] >> 6 & 3` = holder):

| Condition | RCDATA | Template |
|---|---|---|
| neutral (0 or 3) | `0x7007` | "\|system\| has been cleared of \|loser_adj\| forces." |
| `+0x5c` == holder and holder == `+0x58` (player held it, under blockade) | `0x7016` | "\|winner_adj\| forces have broken the blockade of \|system\|." |
| `+0x5c` == holder, holder is the enemy (player was blockading) | `0x7015` | "\|winner_adj\| forces have maintained the blockade of \|system\|." |
| holder == `+0x58` (no prior blockade) | `0x7005` | "\|system\| has been successfully defended from \|loser_adj\| forces." |
| else | `0x7006` | "\|system\| is now under blockade by \|winner_adj\| forces." |

Body — loser fate:

| `+0x54` | RCDATA | Template |
|---|---|---|
| 0 (destroyed) | `0x700b` | "The \|loser_adj\| fleet has been completely destroyed." |
| nonzero (retreated) | `0x700c` | "The \|loser_adj\| fleet has withdrawn." |

#### `FUN_0049b4d0` — player lost

| Buffer | RCDATA | Template text |
|---|---|---|
| title +0x88 | `0x7001` | "Battle at \|system\|" |
| status +0x94 | `0x7003` | "The \|player_adj\| fleet is defeated." |

Body +0xa0 — system status (holder):

| Condition | RCDATA | Template |
|---|---|---|
| neutral (0 or 3) | `0x700a` | "The \|winner_adj\| fleet was victorious at the neutral world \|system\|." |
| holder == winner AND holder == blockade | `0x7015` | "\|winner_adj\| forces have maintained the blockade of \|system\|." |
| holder == winner (not blockade) | `0x7016` | "\|winner_adj\| forces have broken the blockade of \|system\|." |
| holder == blockade side | `0x7008` | "\|system\| is now under blockade by \|winner_adj\| forces." |
| else | `0x7009` | "The \|player_adj\| attack on \|system\| has failed." |

Player fleet fate:

| `+0x54` | Body RCDATA | Message RCDATA |
|---|---|---|
| 0 (destroyed) | `0x700b` "...completely destroyed." | `0x7010` "The \|player_adj\| fleet was defeated at \|system\|. Your fleet has been destroyed." |
| nonzero (retreated) | `0x700d` "...withdrawn to \|retreat_sys\|." | `0x7011` "...defeated at \|system\|. Your fleet has withdrawn to \|retreat_sys\|." |

Navigation: `0x7014` (single button) when destroyed, `0x7013` (multiple) when
retreated.

#### `FUN_0049b150` — draw (indecisive)

| Buffer | RCDATA | Template text |
|---|---|---|
| title +0x88 | `0x7001` | "Battle at \|system\|" |
| status +0x94 | `0x7004` | "The battle at \|system\| is indecisive." |

Body +0xa0 always starts with:

| RCDATA | Template |
|---|---|
| `0x700e` | "There has been no victor." |

Then fleet fates:

| `+0x54` | Body | Message RCDATA |
|---|---|---|
| 0 (both destroyed) | `0x700f` "All \|side1_adj\| and \|side2_adj\| ships have been destroyed." | `0x7010` (defeated, destroyed) |
| == +0x58 (player retreated, enemy destroyed) | `0x700d` "\|player_adj\| fleet has withdrawn to \|retreat\|." + `0x700b` "\|enemy_adj\| fleet has been completely destroyed." | `0x7011` (defeated, withdrawn) |
| else (player destroyed, enemy retreated) | `0x700b` "\|player_adj\| fleet has been completely destroyed." + `0x700c` "\|enemy_adj\| fleet has withdrawn." | `0x7010` (defeated, destroyed) |

### Specific cases

**(a) Draw where both sides still have forces**: does not happen as a
battle result screen.  `FUN_00517270` (`FUN_00517270.c:79-93`) checks
side-1 and side-2 presence via `FUN_004ffef0` (fleets) and `FUN_00503a50`
(system squadrons).  When both sides are present (`bVar5 && cVar2`), it
sets `local_44 = 0` and skips the call to `FUN_00517ee0` — no result
object is created, no text is generated.  The battle re-fires on the
next frame-loop pass for that system (see cadence below).

**(b) Both sides destroyed**: `FUN_0049b150` (draw) with `+0x54 == 0`.
Status: `0x7004` "The battle at \|system\| is indecisive."
Body: `0x700e` "There has been no victor." + `0x700f` "All \|side1_adj\|
and \|side2_adj\| ships have been destroyed."

**(c) Loser had no fleet, only planet-held squadrons**: the text builders
use template `0x700b` ("The \|loser_adj\| fleet has been completely
destroyed.") and `0x700c` ("The \|loser_adj\| fleet has withdrawn.")
regardless of the loser's force composition.  The word "fleet" is
hardcoded in the RCDATA template, not a variable.  System squadrons are
described as "fleet" even when no fleet object existed.

**(d) Winner had no fleet (squadrons only)**: the templates use "fleet"
unconditionally.  `0x7002`: "The \|winner_adj\| fleet is victorious."
For the system status, when the winner held only system squadrons:
- If the system was neutral: `0x7007` "\|system\| has been cleared of
  \|loser_adj\| forces." — no blockade mention.
- If the system belonged to the loser: `0x7006` "\|system\| is now under
  blockade by \|winner_adj\| forces." — hyp: system squadrons cannot
  actually blockade, so this text may be misleading; in practice this
  case is rare because the loser's fleet would need to have been present
  at its own system for the battle to start, and system squadrons alone
  don't trigger the battle bit on the attacker side.

### RCDATA template ID table

| ID (hex) | ID (dec) | Template |
|---|---|---|
| 0x7001 | 28673 | Battle at \|system\| |
| 0x7002 | 28674 | The \|1,adj\| fleet is victorious. |
| 0x7003 | 28675 | The \|1,adj\| fleet is defeated. |
| 0x7004 | 28676 | The battle at \|1,name\| is indecisive. |
| 0x7005 | 28677 | \|1,name\| has been successfully defended from \|2,adj\| forces. |
| 0x7006 | 28678 | \|1,name\| is now under blockade by \|2,adj\| forces. |
| 0x7007 | 28679 | \|1,name\| has been cleared of \|2,adj\| forces. |
| 0x7008 | 28680 | \|1,name\| is now under blockade by \|2,adj\| forces. |
| 0x7009 | 28681 | The \|2,adj\| attack on \|1,name\| has failed. |
| 0x700a | 28682 | The \|2,adj\| fleet was victorious at the neutral world \|1,name\|. |
| 0x700b | 28683 | The \|1,adj\| fleet has been completely destroyed. |
| 0x700c | 28684 | The \|1,adj\| fleet has withdrawn. |
| 0x700d | 28685 | The \|1,adj\| fleet has withdrawn to \|2,name\|. |
| 0x700e | 28686 | There has been no victor. |
| 0x700f | 28687 | All \|1,adj\| and \|2,adj\| ships have been destroyed. |
| 0x7010 | 28688 | The \|1,adj\| fleet was defeated at \|2,name\|. Your fleet has been destroyed. |
| 0x7011 | 28689 | The \|1,adj\| fleet was defeated at \|2,name\|. Your fleet has withdrawn to \|3,name\|. |
| 0x7012 | 28690 | The \|1,adj\| fleet is victorious. The world \|2,name\| is now under \|1,adj\| blockade. |
| 0x7013 | 28691 | Select one of the following buttons to close this display and go directly to... |
| 0x7014 | 28692 | Select the following button to close this display and go directly to... |
| 0x7015 | 28693 | \|2,adj\| forces have maintained the blockade of \|1,name\|. |
| 0x7016 | 28694 | \|2,adj\| forces have broken the blockade of \|1,name\|. |

## 2. Re-Battle Cadence

### Original behavior

The frame loop (`FUN_0040a050`) sends message 9, which calls
`FUN_0041e270` -> `FUN_0051ddf0` (`timer-scheduler.md:56-58`).
`FUN_0051ddf0` runs two passes for one location:

1. `FUN_00516ef0` — pre-battle: checks `FUN_00509ad0(system)` for a
   pending battle state.  If 0 (no battle), checks both-sides-present
   via `FUN_004ffef0` and `FUN_00503a50`; if both present, sets the
   engaged side from the system holder (`system+0x78 >> 6 & 3`).  If
   non-zero, reuses the pending state.  Either way, when a side is
   engaged, it processes system squadrons via `FUN_00556710` and fleets
   via `FUN_004f78e0`.

2. `FUN_00517270` — aftermath: rechecks both-sides-present.
   - If both sides remain (`bVar5 && cVar2`): `local_44 = 0`, skips
     `FUN_0050a040` and `FUN_00517ee0`, returns 0.  The system's battle
     state persists.
   - If one side remains: `local_44 = winner`, calls `FUN_0050a040`
     (set system control) and `FUN_00517ee0` (result object + message).

When `FUN_00517270` finds both sides present and returns 0, the
system's battle state (`+0x88` bit `0x1000`) remains set.  On the next
frame-loop pass for that location, `FUN_00509ad0` returns non-zero and
`FUN_00516ef0` triggers combat again.

**There is no cooldown, no counter, no delay.**  The battle re-fires
every frame-loop pass that finds both sides present.  The frame loop
cycles through locations; `system-squadrons-combat.md` describes this
as "the battle continues next tick".

### Port behavior (differences)

Resolved 2026-10-10: the cooldown below is gone from `simulation.rs`,
`main.rs` and `tactical_flow.rs`, and saves (v36) no longer carry it. The
table records what was removed.

The port used a 5-tick cooldown (`COMBAT_RETRY_TICKS = 5` at
`crates/rebellion-data/src/simulation.rs:38`) and a permanent stalemate
cooldown (`UNCHANGED_STALEMATE_COOLDOWN = u64::MAX` at `:40`).

| Code | Location | What |
|---|---|---|
| `COMBAT_RETRY_TICKS = 5` | `simulation.rs:38` | 5-tick delay between battles |
| `UNCHANGED_STALEMATE_COOLDOWN` | `simulation.rs:40` | Stalemate blocks re-battle until a change event |
| `combat_is_on_cooldown()` | `simulation.rs:42-45` | Returns true for stalemate OR within 5 ticks |
| `combat_cooldowns.insert(sys_key, ...)` | `simulation.rs:226-233` | Stores current tick (normal) or MAX (stalemate) |
| `combat_cooldowns.remove()` | `simulation.rs:172,180,192` | Clears on fleet arrival, unit deployment, manufacturing completion |
| Same 5-tick check | `main.rs:2551-2555` | Duplicated in the interactive path |

The original has none of this.  The only gate is `FUN_00509ad0`'s return
value: when the system's battle state is still set, the next pass
re-triggers.  The 5-tick cooldown and the permanent stalemate cooldown
are port inventions.

## Differences from our code

### Battle result text (`tactical_view.rs:10870-10903`)

> **Resolved 2026-10-10** except the last row: `battle_result_summary_lines`
> now builds the won / lost / draw texts and the five system-status lines,
> and names the withdrawal destination for the player's fleet after Simulate.
> The table records the old behaviour. Still open: the separate Message Index
> text (`0x7010`–`0x7012`), and the destination after a played battle.

| Our code | Original |
|---|---|
| Draw: "Neither fleet is victorious." / "Both forces have been destroyed or withdrawn." | Draw never shown when both sides survive; when both destroyed: `0x7004` "...is indecisive." + `0x700e` "There has been no victor." + `0x700f` "All ... ships have been destroyed." |
| Win: always "The {winner} fleet is victorious." + "{system} is now under blockade by {winner} forces." + loser fate | Win: `0x7002` + one of 5 system-status templates based on holder/blockade state + `0x700b`/`0x700c` |
| No "player lost" perspective | Original shows `0x7003` "defeated" + different system-status and message text for the losing player |
| Loser: "withdrawn from the system" | Original: `0x700c` "has withdrawn" (no destination in winner view); `0x700d` "withdrawn to |retreat_sys|" (in loser view) |
| Always says "blockade" on victory | Original says "cleared of forces" (neutral), "defended" (own system), "maintained/broken blockade" (blockade context), "under blockade" (only when establishing new blockade) |
| No separate message-index text | Original generates a separate message for the Message Index (`0x7010`/`0x7011`/`0x7012`) |

### Re-battle cadence (`simulation.rs:38-45`, `main.rs:2551-2555`)

| Our code | Original |
|---|---|
| 5-tick cooldown between battles | No cooldown; re-fires on next frame-loop pass |
| Permanent stalemate cooldown (`u64::MAX`) | No permanent cooldown; battle repeats every pass |
| Cooldown cleared by fleet arrival / manufacturing | No cooldown system to clear |

## Confidence

| Claim | Evidence | Level |
|---|---|---|
| RCDATA template IDs and text | Extracted from TEXTSTRA.DLL (`textstra-rcdata.json`) | Confirmed |
| FUN_0049b330 = player won | `0x7002` "victorious" in status line | Confirmed |
| FUN_0049b4d0 = player lost | `0x7003` "defeated" in status line | Confirmed |
| FUN_0049b150 = draw | `0x7004` "indecisive" in status line | Confirmed |
| System-status branching on holder | `system[9] >> 6 & 3` checked in both FUN_0049b330 and FUN_0049b4d0 | Confirmed |
| Dispatch via FUN_0049b0f0 | `FUN_0049ac00.c:57`, function not in corpus | hyp: |
| Both-sides-present = no result shown | `FUN_00517270.c:79-93` logic traced | Confirmed |
| No cooldown in original | No delay/counter visible in FUN_0051ddf0, FUN_00516ef0, FUN_00517270 | Confirmed |
| FUN_00509ad0 = battle-pending state reader | Returns side number, compared with object side at `FUN_0050c6c0.c:35`; returns 0 for no battle at `FUN_00516ef0.c:95` | hyp: (function not decompiled) |
| "Fleet" text for squadron-only forces | Template text is hardcoded "fleet" regardless of composition | Confirmed |
