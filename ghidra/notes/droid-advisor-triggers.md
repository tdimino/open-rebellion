---
title: "Droid Advisor Triggers and Playback"
description: "When the cockpit droids animate: the per-side advice agent's reaction slots, the GData SPT action tables, the message codes that fill them, idle chatter, and the droid command player"
category: "ghidra"
created: 2026-10-08
updated: 2026-10-09
---

# Droid Advisor Triggers and Playback

Traced 2026-10-08. The decompiles cited here sit beside this note. The
action tables were read from `GData/C3POACT.SPT` and `GData/IMP22ACT.SPT`,
and the action scripts from the `RT_RCDATA` of ALSPRITE.DLL and
EMSPRITE.DLL.

The droids do not loop. Each rests on one still frame, and animates only
when a reaction slot fires, or when idle chatter is due.

## The droid players

`FUN_0042adb0` builds two players (`FUN_00441a60`, vtable `0x659b60`) on the
galaxy view: the agent droid at `+0x128` and its partner at `+0x12c`.

| Side | Droid | Rect (x, y, w, h) | Rest script | Rest frame |
|---|---|---|---|---|
| Alliance | C-3PO (`+0x128`) | 541, 337, 67, 116 | ALSPRITE 403 | BMP 2001 |
| Alliance | R2-D2 (`+0x12c`) | 316, 411, 47, 69 | ALSPRITE 402 | BMP 3331 |
| Empire | IMP-22 (`+0x128`) | 0, 347, 107, 133 | EMSPRITE 403 | BMP 2001 |
| Empire | SD-7 (`+0x12c`) | 302, 401, 101, 79 | EMSPRITE 401 | BMP 3001 |

Each player ticks once every 67 ms (`param_7` = `0x43`, `FUN_00441d70`).

### Command queue

Each player has a queue (`FUN_004727e0`, vtable `0x65a358`). The two keys are
`0xdead` for the agent droid and `0xbeef` for its partner.

- **Enqueue:** a play request (`FUN_00442070` → `FUN_00472930`) builds a
  command by number (`FUN_00403ed0`, prototypes registered by
  `FUN_00403eb0`) and appends it.
- **Advance:** `FUN_004729c0` runs the head until it reports done, then starts
  the next one.
- **Default:** when the queue is empty, it replays the registered default
  (`FUN_00472b20`):
  - agent droid: command `0x12` on its rest script, 15 ticks, with callback
    `FUN_0041cea0`;
  - partner: command 5 on its rest script, 15 ticks.

So a resting droid shows its rest frame, re-armed about once a second.

| Command | Vtable | Does |
|---|---|---|
| 4 | `0x658520` | Plays an animation once. The script is `(next, wave, wave module, frames, anchor per layer)`. It plays the sound and the anchor's type-302 delta run, one frame a tick (`FUN_00404440`, `FUN_00404360`). |
| 5 | `0x658550` | Holds the script's still frame for N ticks, then calls its callback (`FUN_004049a0`, `FUN_00404910`, `FUN_00404b40`). |
| `0x11` | `0x658848` | Holds a still frame one tick (`FUN_00412420`, `FUN_00412260`), then stores its `(p2, p3)` as a cockpit step (`FUN_00412400` → `FUN_00439eb0`). |
| `0x12` | `0x658878` | Holds the rest frame for N ticks. On start it calls `FUN_0041cea0`, which arms idle chatter (`FUN_00412680`, `FUN_00412660`). |
| `0x13` | `0x6588a8` | A command 4 layout for the partner droid. |
| `0x14` | `0x6587e8` | Turns to the partner: plays the partner's script, then partner script 11575 (`FUN_00411b80`). |
| `0x15` | `0x658818` | Holds a still frame one tick (`FUN_004120a0`), then signs off (`FUN_00412080`). |

An action script, `RT_RCDATA` in the side's sprite DLL, is
`(command, script, p2, p3)`. `FUN_0041d660` / `FUN_0042b1d0` queue it on the
agent droid; `FUN_0041d690` / `FUN_0042b290` queue it on the partner.

`FUN_0042b1d0` loads the action from module 9, or from module 13 when its
last argument is set (`FUN_005fefd0`). It passes the module with the script
id, so the script and its bitmaps come from the same DLL. Module 9 is the
side's sprite DLL and module 13 its briefing DLL (ALBRIEF, EMBRIEF): the WAVE
ids a script names fall in that DLL's range.

### Sounds

A command 4 script names a wave and its module, which the command plays as
it starts (`FUN_00403f70`). The sprite DLLs hold 213 (Alliance) and 216
(Empire) WAVE resources, PCM at 11025 Hz mono or 22050 Hz stereo.

- Idle chatter is silent: Alliance chatter always names wave 1291, and the
  Empire's names 22001..22006, each 0.1 s of silence.
- Reactions speak: for example ALSPRITE wave 1096 is a 7.4 s line.
- Two waves that reachable scripts name are absent from the DLLs: ALSPRITE
  1019 (action 10019) and EMSPRITE 1580 (action 11080).

For example, ALSPRITE 10096 is `(4, 4096, 0, 0)`, and 4096 is
`(0, wave 1096, 9, 91 frames, anchor 3002)`. The run 2001 + 2002..2024,
which the port used to loop, is action 4002 with wave 1002: one reaction
among many.

## The advice agent

The reactions belong to the side's agent object, at session `+0xc0`
(`FUN_004861b0`):
- Alliance: `FUN_004c27f0`, vtable `0x65c4c8`, 88 slots;
- Empire: `FUN_004c0710`, vtable `0x65c4a0`, 86 slots.

Each slot holds two values:
- `+0x168[slot]`: the time it fires by;
- `+0x16c[slot]`: the time it may next fire.

Time here is the scheduler's step count, `FUN_004fcee0` → `FUN_0051ce00()`
`+0xc`. It advances by the speed's rate each day: side `+0xc4` is 600, 60,
12 or 4 (`FUN_0051dea0`, `2026-09-24-game-speed-recovery.md`). A window of
10 steps therefore lasts under a day at Medium.

### The table

`FUN_004c2c70` (Alliance) and `FUN_004c0ba0` (Empire) load
`GData/C3POACT.SPT` and `GData/IMP22ACT.SPT` (`FUN_0049c6f0`).

- **File:** a `u32` slot count, then for each slot:
  - `key u32`, `u32`, `count u32`, `cursor u32`;
  - `count` records of `(u16 action, u32 flags, u32 p2, u32 p3)`
    (`FUN_004c52d0`).
- **Flag bit 0:** the record plays only while `DAT_006b28b0` has `0x1000`
  and lacks `0x4000`. These are the agent droid's spoken advice after the
  partner's beep.
- **Flag bit 1:** the action comes from module 13, ALBRIEF.DLL, the opening
  briefing.

### The pass

The pass is `VT[6]`: `FUN_004c2b20` (Alliance) and `FUN_004c0a60`
(Empire). The session pump `FUN_00486520` calls it while no game message is
pending.

It scans only while `DAT_006b28bc` is set. Command `0x12` sets that every
time it starts (`FUN_00439ef0`).

For each slot from 1 up:
1. A zero or past stamp is cleared and skipped.
2. A live stamp is cleared. If the slot's cooldown has passed, the cooldown
   becomes now + 60. The slot's records are queued, the pass records the
   time in `DAT_006b28d0` and stops: one reaction per pass.

When nothing fires, `FUN_004c2dd0` / `FUN_004c0d00` turn pending flag bits
into stamps, one per pass. Bit `0x80000000` is idle chatter. It rolls
`FUN_0041cd80(12)`, and a result below 6 stamps one of six chatter slots:
- Alliance: slots 80..85 (ALSPRITE 10301..10306), stamp + 40000;
- Empire: slots 78..83 (EMSPRITE 11301..11306), stamp + 5.

`FUN_00439ef0` sets bit `0x80000000` only when more than 50 steps have
passed since the last reaction. That is the whole idle rule:
- no chatter until 50 steps after any reaction;
- then a 50% roll on each pass;
- each chatter slot then waits 60 steps.

While the game is paused the step count stands still, so chatter stops.

At creation, one slot's stamp is set to 40000, so it plays first:
- Alliance: slot 41, the briefing tour from ALBRIEF;
- Empire: slot 74.

### Message codes

A game message carries an advice code at `+0x28`. When `FUN_0048a060` files
a message of kinds 3, 4, 5 or 8, it passes that code to the agent's
`VT[5]`: `FUN_004c44b0` (Alliance) or `FUN_004c22f0` (Empire). That sets the
slot's stamp to now plus a window. It withholds the code when bit 4 of the
message's category flags (`FUN_0048a1c0`) is set: the Message Index's Post
Messages Silently button (`message-index-rows.md`, "The right rail").

The setters are the message classes that `FUN_00489740` builds. The table
below names each code from the message text (TEXTSTRA `RT_RCDATA`) beside
its setter.

| Code | Event | Setter | Alliance slot (window) | Empire slot (window) |
|---|---|---|---|---|
| 1 | Uprising news for the side: it begins on an enemy world or ends on its own; a world joins the side (`FUN_00499760`) | `FUN_00499460`, `FUN_00499760` | 52 (10) | 48 (10) |
| 2 | The reverse: an uprising begins on the side's world or ends on an enemy's; a world joins the enemy | `FUN_00499460`, `FUN_00499760` | 53 (10) | 49 (10) |
| 3 | Facility idle, deployed or lost | `FUN_0048be60`, `FUN_00497690`, `FUN_00497940` | 56 (5) | 52 (10) |
| 4 | Research complete | `FUN_00499aa0` | 58 (10) | 54 (10) |
| 5 | A fleet or capital ships arrive (cases 1, 2) | `FUN_004981c0` | 59 (10) | 55 (10) |
| 6 | Other units or the headquarters arrive (cases 3, 4) | `FUN_004981c0` | 61 (10) | 57 (10) |
| 8 | Capital ship repaired | `FUN_00499de0` | 60 (10) | 56 (10) |
| 9 | Squadron at full strength | `FUN_00499de0` | 62 (10) | 58 (10) |
| `0xc` | Saboteurs strike; maintenance shortfall | `FUN_0048b9a0`, `FUN_00498970` | 57 (10) | 53 (10) |
| `0xd` | Our fleet starts a blockade | `FUN_004960f0` | 65 (10) | 61 (10) |
| `0xe` | System under enemy blockade | `FUN_004960f0` | 67 (10) | 62 (5) |
| `0x14` | Personnel arrive | `FUN_00495bf0` and others | 42 (10) | 64 (10) |
| `0x15` | Personnel report (mission, arrival) | message classes 20..40 | 66 (10) | 63 (10) |
| `0x16`..`0x19` | The same, when it concerns a major character (family `0x240..0x243`) | `FUN_004955b0` and others | 45, 43, 44, 46 (40000) | 64 (40000) |
| `0x1a`, `0x1b` | The same, for family `0x280` / `0x281` | `FUN_004955b0` | 42 (40000) | 34, 33 (40000) |
| `0x1c` | Informants report | `FUN_00491060` | 55 (10) | 51 (40000) |
| `0x1e`..`0x23` | A character captured | `FUN_00490340`, `FUN_0048fa00` | 28, 27, 29, 30, 32, 31 (40000) | 42, 41, 43, 44, 46, 45 (40000) |
| `0x24`..`0x28` | A character escapes | `FUN_00490340` | 51, 48, 47, 50, 49 (40000) | 47, 38, 37, 40, 39 (40000) |
| `0x29` | Message from leadership | `FUN_00498e90` | 68 (50) | 65 (50) |
| `0x2a`, `0x2b` | Advice messages (`FUN_0048b2e0`) | — | `FUN_004c4430`, `FUN_004c4480` | `FUN_004c2270`, `FUN_004c22c0` |
| `0x2c` | — | `FUN_00545240` (untraced) | 75 (10) | — |
| `0x2d` | Bounty hunters locate Solo | `FUN_0048fa00` | — | 72 (10) |
| `0x2e` | Orbital bombardment of a neutral system | `FUN_00496770` | 25 (10) | 35 (10) |
| `0x2f` | Assault on a system | `FUN_00496770` | 26 (10) | 36 (10) |

`VT[4]` (`FUN_004c3010`, `FUN_004c0f10`) fills slots 86/87 (Alliance) and
84/85 (Empire) on flag bits `0x100000` and `0x1000000`.

### Flag bits

The agent pass turns these `DAT_006b28b0` bits into stamps
(`FUN_004c2dd0`, `FUN_004c0d00`):

| Bit | Set by | Alliance slot (window) | Empire slot (window) |
|---|---|---|---|
| `0x8000000` | untraced; while agent `+0x10` is 0, it sets `+0x10` to 1 | 34 (5) | 27 (50) |
| `0x4000000` | untraced; sets agent `+0x10` back to 0 | 33 (40000) | 26 (10) |
| 1 | `FUN_0043a2a0` (hyp: an automation order was carried out) | 35 (10) | 28 (10) |
| `0x10`, `0x20` | Garrison / Production automation on | 36, 38 (10) | 29, 31 (10) |
| `0x100`, `0x200` | Garrison / Production automation off | 37, 39 (50) | 30, 32 (50) |
| `0x2000` | `FUN_0043a200` | 40 (40000) | 73 (40000) |
| `0x1000000` | `FUN_00439d60`, a fourth automation turned on | 79 (5) | — |
| `0x2000000` | `FUN_00439ed0` | 76..78 in turn (5) | 75..77 in turn (5) |

### Cockpit steps and the briefing tour

The pass first runs a stored cockpit step (`DAT_006b28c4`, `DAT_006b28c8`,
set by command `0x11`) through `FUN_004c3060`, then returns:

- p2 2 runs briefing step p3 (`FUN_004c30c0`);
- p2 3 selects an object (`FUN_0041d830`).

The opening slot (Alliance 41, Empire 74) alternates such steps with module 13
clips. On the Alliance side there are 16 clips, of 50 to 380 frames each, so
the tour runs about three minutes. Each step works the cockpit. It opens a
window by sending `WM_COMMAND` (`FUN_0041d8c0`: `0x11`, `0x13`, `0x15`..`0x17`,
`0x20`, `0x80`, `0x91`), or opens a list on an object (`FUN_0041d890` with
`0x92`, `0x93`): the headquarters (`0x20000005`), Mon Mothma (`0x30000240`)
and Luke (`0x32000242`). Step 12 installs input hooks (`FUN_0041d9d0`), and
step 13 removes them and ends the tour (`FUN_0041da80`).

`FUN_00439320` sets `DAT_006b28b0` bit `0x8000` unless the session mode
(`FUN_00401b00`, `+0x104`) is 1 or 4, a new game. Step 13 checks that bit.

The steps are display modes of the galaxy view (`FUN_0041d8c0` sends
`WM_COMMAND` 0x140, the GID menu's id, to `FUN_00422ce0` → `FUN_00425d00`).
Modes 0x92 and 0x93 mark a list of systems under a caption (`FUN_0041d890`
stores it at `+0x494`):

| Step | Alliance (`FUN_004c30c0`) | Empire (`FUN_004c0fc0`) |
|---|---|---|
| 1 | 0x11 Popular Support | 0x11 |
| 2 | 0x13 | 0x92 Coruscant (0x109) |
| 3 | 0x93 Alliance Headquarters | 0x93 Yavin (0x121) |
| 4 | 0x92 Coruscant | 0x17 |
| 5 | 0x20 (0x21's markers, Idle Fleets) | 0x20 |
| 6 | 0x91 All Defenses | 0x91 |
| 7 | 0x93 Mon Mothma (0x240's system) | 0x92 Emperor Palpatine (0x280) |
| 8, 20 | 0x92 Coruscant | 0x93 Yavin |
| 9 | 0x80 Display Off | 0x80 |
| 11 | 0x11, and the clock runs (`FUN_0041dbe0` → `FUN_0041e320` → `FUN_00401950`) | the same |
| 12 | input hooks (`FUN_0041d9d0`) | the same |
| 13 | end (`FUN_0041da80`); Message Index on Advice (`FUN_0041d770(1, 0x82)`) | the same |
| 14 | 0x13 | 0x13 |
| 15 | 0x15 | 0x15 |
| 16 | 0x16 | 0x14 |
| 17 | 0x17 Unexplored Systems | 0x17 |
| 18 | 0x93 Yavin | 0x92 Coruscant |
| 19 | 0x93 Luke Skywalker (0x242) | 0x92 Darth Vader (0x281) |

`FUN_0042b330` draws the tour's modes:
- 0x13..0x16 give 0x52 to systems whose side (`+0x24` bits 6..7) is the
  player's (0x13, 0x14) or the enemy's (0x15, 0x16) and whose `+0x84` bits
  2..3 match that side (0x13, 0x15) or do not (0x14, 0x16). The thresholds
  are Popular Support's.
- 0x17 gives 0x52 to unexplored systems, in the neutral set.
- 0x91 sums troops, shields and fighters.
- 0x92 / 0x93 give 7 to listed systems, in the Imperial / Alliance set; the
  rest are flagged 0x80 and drawn as 10158.

`FUN_004522f0` has no legend script for these modes. `FUN_00425d00` titles
0x13 / 0x15 by the player's / enemy's side (0x161f, 0x1620), and 0x14 / 0x16
with 0x1621 only where `(mode == 0x14) != (side == 2)` is false.

Until step 11 the clock holds. In the Wine capture the day box and the
Message Index rail stay blank, while the resource counters and the compact
legend show from the first cockpit frame.

### Skipping the tour

While step 12's hooks are in, Escape (on release, `lpfn_0041d8f0`) or a left
or right press (`lpfn_0041d950`) calls `FUN_0043a200`, which sets bit
`0x2000`. The next pass with nothing to fire stamps the skip slot (Alliance
40, `+0xa0`; Empire 73, `+0x124`) for the whole game and empties both
droids' queues (`FUN_0041da30` → `FUN_0042d620`, keys `0xdead` and
`0xbeef`). `DAT_006b14b4` lets this happen once. The skip slot runs step 11,
one clip (ALBRIEF 10165, wave 1165, 4.3 s; EMBRIEF 11157, wave 1157,
1.3 s) and step 13.

### Advice topics

The agent's topics are a TEXTSTRA `RT_RCDATA` table: `0x6000` for the
Alliance, `0x6800` for the Empire. Word 0 is the count (31). Entry `i`
sits at `base + 3i`; its kind is the low nibble of byte 0 and its order is
the u16 at byte 2 (10..310). The topic's title is `base + 3i + 1` and its
body `base + 3i + 2`, both ending at `0x01`. `FUN_0048b460` reads the table
and `FUN_005f5440` keeps it in order.

`FUN_0048b2e0` files a topic in the Advice category with code `0x2a`, the
advice picture (`0x42f` Alliance, `0x430` Empire) and sound (`0x461`,
`0x462`).

- Step 13 calls `FUN_00439f20`. It builds the held list once (`+0x150`
  bit `0x10000000`) and files every kind 7 topic: nine for the Alliance.
- `FUN_0043a0b0(bit)` files the first held topic of a window's kind the
  first time the player opens it: the sector window (`FUN_00429ce0`, bit
  1, kind 6), System (type 9, bit 2, kind 2), Fleet (type 4, bit 4, kind
  3), System Defenses (type 10, bit 8, kind 5) and Missions (type 11, bit
  `0x10`, kind 4). `FUN_0045aac0` opens the typed windows.
- The side update (`FUN_004866b0` → `FUN_00439bc0`) files one more every
  300 steps: `FUN_00439fb0` takes the first held topic whose kind is open.
  Kinds 2..6 wait for their window's bit; the others are always open.

All of it waits on Agent Advice (`DAT_006b28b0` bit `0x8000` clear), which
Easy turns on. `FUN_004397a0` saves neither `+0x150` nor `+0x154`, so a
loaded game builds the list again at its first release.

### Advice codes `0x2a` and `0x2b`

`0x2a` stamps one of four slots ten steps on, and only when none of the
four is pending: `FUN_004c4430` (Alliance, slots 69..72) and `FUN_004c2270`
(Empire, 66..69). `0x2b` stamps one of two whatever is pending:
`FUN_004c4480` (Alliance, 73..74) and `FUN_004c22c0` (Empire, 70..71).
`FUN_0041cd80(n)` picks the slot.

### Mission refusals

A refused mission reaches agent slot `+0xc`: `FUN_004c2940` (Alliance) or
`FUN_004c0870` (Empire). The same status twice stamps the first repeat
slot; a third time stamps the second and clears the memory
(`FUN_004c43d0`, `FUN_004c21d0`). The repeat slots are 15 and 16 for the
Alliance, 16 and 17 for the Empire. A generic `0x40` status rolls
`FUN_0041cd80(2)` and stamps one of two slots ten steps on: 17/18 for the
Alliance, 18/19 for the Empire. The port maps its refusals to statuses in
`MissionRefusal::status`.

### The step clock

The scheduler's step count advances by the speed's rate each day. Timed in
the original under Wine (2026-10-08), a day took 1.19..1.21 s at Fast (rate
4), 3.24 s at Medium (12), 15.1 s at Slow (60) and 150.6 s at Very Slow
(600): 0.25 s per rate unit and about 0.2 s more. A new game starts at
Slow.

## Open

- Who sets bit `0x4000` (`FUN_00487f80`) is untraced.
- The 50% chatter roll uses the session random stream (`FUN_005f5700`).
- What adds the 0.2 s to each day is untraced.
- The session mode at `+0x104` is not named.
- `+0x84` bits 2..3, which split modes 0x13..0x16, are written by no
  recovered function. hyp: the port uses the popularity majority.
