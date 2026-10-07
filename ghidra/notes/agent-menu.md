---
title: "Agent Menu — Popup, Items, Advice Flag, Automation Bits"
description: "Full trace of the Agent popup menu: trigger path, TEXTSTRA labels, accelerator routing, DAT_006b28b0 bit semantics, difficulty default, save/load, and module 0x17 build commands"
category: "ghidra"
created: 2026-10-06
updated: 2026-10-06
sources:
  - type: "ghidra"
    files:
      - "FUN_00422ce0.c"
      - "FUN_0042d050.c"
      - "FUN_0041d160.c"
      - "FUN_00436650.c"
      - "FUN_00487900.c"
      - "FUN_00486fb0.c"
      - "FUN_00439d10.c"
      - "FUN_00439d60.c"
      - "FUN_00439e80.c"
      - "FUN_00439e30.c"
      - "FUN_0049e360.c"
      - "FUN_00439320.c"
      - "FUN_004c0710.c"
      - "FUN_004c27f0.c"
      - "FUN_004397a0.c"
      - "FUN_00439550.c"
      - "FUN_0043a0b0.c"
      - "FUN_004c0d00.c"
      - "FUN_00439bc0.c"
      - "FUN_00439f20.c"
      - "FUN_00439fb0.c"
      - "FUN_00439ef0.c"
      - "FUN_00439ed0.c"
      - "FUN_0043a2a0.c"
      - "FUN_00487f80.c"
      - "FUN_004c0a60.c"
      - "FUN_0049e6b0.c"
      - "FUN_00401b00.c"
  - type: "textstra"
    ids: [12560, 12561, 12562, 12563, 12564, 12565, 12566, 12567, 12568, 12569, 12570, 12571, 12572, 12573, 12574]
  - type: "manual"
    pages: "22, 64-66"
tags: [agent-menu, advice, difficulty, popup, accelerator, DAT_006b28b0]
---

# Agent Menu — Popup, Items, Advice Flag, Automation Bits

> **Correction (2026-10-06, checked against the decompiles).** A module's
> state 2 means *suspended*: `FUN_004cc990` (VT[4]) readies only a module
> whose state is neither 1 nor 2, so a state-2 module never runs, and
> `FUN_00439950` starts every module in state 2, so all start **off**.
> `FUN_00439e30` returns 1 when the state is not 2, and `FUN_00487900`
> checks the menu item then: a check means the automation runs.
> `FUN_00439d60` turns a module **on** (2 → 0, counter +1) with bit `0x10`
> (Garrisons) or `0x20` (Production), or `0x1000000` once the counter
> passes 3, and **off** (→ 2, counter −1) with `0x100` or `0x200`. Tables
> below that label those bits or states the other way round are wrong.

## 1. How the Agent menu opens

The Agent menu is a Win32 popup built from STRATEGY RT_RCDATA list
`0xdead`. Two paths trigger it:

### Path A — Right-click on a cockpit advisor panel

```
FUN_00422ce0 (galaxy view WndProc, WM_RBUTTONUP = 0x205)
 └─ FUN_004420b0(param_1[0x4a], x, y)  → returns menu-list id
    or FUN_004420b0(param_1[0x4b], x, y)
     └─ FUN_0042d050(galaxyView, x, y, 0xdead)
         └─ FUN_0041d160(menuList, 1)
             └─ FUN_00436650(viewObj, menuList)
                 └─ FUN_00487900(viewObj+0xc, menuList)  // enable/check items
             └─ FUN_00436640(viewObj)                    // hyp: show popup
         └─ iterate items → FUN_00442590()              // add each to popup
         └─ FUN_00442380()                              // track popup menu
```

`param_1[0x4a]` and `param_1[0x4b]` are the two cockpit advisor panel
objects (one per side). `FUN_004420b0` hit-tests the click position; if
the click lands on the advisor panel, it returns the resource list id
(`0xdead` for Agent, `0xbeef` for the other panel). `FUN_0042d050` builds
a popup from that resource list.

### Path B — Accelerator timer (WM_TIMER = 0x483)

```
FUN_00422ce0 (galaxy view WndProc, 0x483)
 └─ FUN_0041d160(menuList, 0)          // load menu list without popup
     └─ FUN_00487900()                 // enable/check items
 └─ check item enabled via FUN_00520720(menuList, cmd_id)
 └─ FUN_0041cdf0(cmd_id, ...)          // dispatch command
```

The timer-based path loads the menu list to verify the item is enabled,
then directly dispatches the command. This is the Alt+key accelerator path.

### Path C — Menu bar WM_INITMENUPOPUP

`FUN_00487900` is also called via the standard Win32 WM_INITMENUPOPUP
handler when the menu bar's Agent menu drops down. The call chain:
`FUN_0041d160 → FUN_00436650 → FUN_00487900`.

---

## 2. Menu items — full table with TEXTSTRA labels

TEXTSTRA IDs are at command_id + 0x3000 (confirmed: 0x110 → 0x3110 =
12560, etc.).

| Cmd    | TEXTSTRA | Label                | Alt  | Timer   | Handler                                | Action |
|--------|----------|----------------------|------|---------|----------------------------------------|--------|
| 0x110  | 12560    | Build Ships          | B    | 0xbc1   | FUN_00439d10 → FUN_0049e360(+0x184, 2) | Build ships via module 0x17 |
| 0x111  | 12561    | Build Troops         | T    | 0xbc2   | FUN_00439d10 → FUN_0049e360(+0x184, 3) | Build troops via module 0x17 |
| 0x112  | 12562    | Build Facilities     | F    | 0xbc3   | FUN_00439d10 → FUN_0049e360(+0x184, 1) | Build facilities via module 0x17 |
| 0x113  | 12563    | Galaxy Overview      | O    | 0xbc4   | FUN_0041d7f0(target)                   | Open encyclopedia for galaxy target |
| 0x114  | 12564    | Objectives           | H    | 0xbd7   | FUN_0041d7f0 + FUN_004ece30            | Open objectives/help encyclopedia |
| 0x115  | 12565    | Manage Garrisons     | G    | 0xbc5   | FUN_00439d60(agent, 0x115)             | Toggle module 0x15 state |
| 0x116  | 12566    | Manage Production    | U    | 0xbc6   | FUN_00439d60(agent, 0x116)             | Toggle module 0x14 state |
| 0x117  | 12567    | Manage Maintenance   | —    | —       | FUN_00439d60(agent, 0x117)             | No-op: no module mapping |
| 0x118  | 12568    | Not An Operation     | —    | —       | FUN_00439d60(agent, 0x118)             | No-op: no module mapping |
| 0x119  | 12569    | Deactivate           | —    | —       | FUN_00439e80(0x119)                    | No-op (guard falls through) |
| 0x11a  | 12570    | Reactivate           | —    | —       | (not dispatched)                       | hyp: separator or unimplemented |
| 0x11b  | 12571    | Translate Counterpart| V    | 0xbc8   | FUN_00439e80 → DAT_006b28b0 ^= 0x1000 | Toggle translate-counterpart bit |
| 0x11c  | 12572    | Messages             | —    | —       | FUN_0041d770(1, 0x79)                  | Open Message Index |
| 0x11d  | 12573    | Message Alerts       | —    | —       | (not dispatched)                       | hyp: separator or unimplemented |
| 0x11e  | 12574    | Agent Advice         | A    | 0xbc9   | FUN_00439e80 → DAT_006b28b0 ^= 0x8000 | Toggle advice on/off |

### Separators and disabled items

Items 0x117, 0x118, 0x119, 0x11a, 0x11d have no accelerator binding.
0x119 is dispatched to FUN_00439e80 but is an explicit no-op (first `if`
guards against it). 0x11a and 0x11d are not in FUN_00486fb0's dispatch
table at all. 0x117 and 0x118 reach FUN_00439d60 but match no module
index, so they are also no-ops. TEXTSTRA labels "Not An Operation" (0x118)
and "Deactivate"/"Reactivate" (0x119/0x11a) suggest these were
planned features that shipped as dead menu entries.

### Enable/disable logic (FUN_00487900)

```c
agent_active = FUN_00487c20(view);    // agent->active (+0x10)
ships_ok     = FUN_00487c30(view);    // module 0x17 state == 2
build_ok     = agent_active & ships_ok;

EnableMenuItem(menu, 0x110, build_ok);   // Build Ships
EnableMenuItem(menu, 0x111, build_ok);   // Build Troops
EnableMenuItem(menu, 0x112, build_ok);   // Build Facilities
EnableMenuItem(menu, 0x115, agent_active); // Manage Garrisons
EnableMenuItem(menu, 0x116, agent_active); // Manage Production
EnableMenuItem(menu, 0x113, 1);            // Galaxy Overview (always)
EnableMenuItem(menu, 0x11b, agent_active); // Translate Counterpart
EnableMenuItem(menu, 0x114, 1);            // Objectives (always)
EnableMenuItem(menu, 0x11e, agent_active); // Agent Advice
```

### Checkmark logic

- **0x11b** (Translate Counterpart): checked when `DAT_006b28b0 & 0x1000`
  is set.
- **0x11e** (Agent Advice): checked when `DAT_006b28b0 & 0x8000` is
  **clear** — meaning the checkmark indicates advice is ON.
- **0x115** (Manage Garrisons): checked when `FUN_00439e30(agent, 0x115)`
  returns nonzero — module 0x15 state != 2 (disabled). Checkmark = OFF.
- **0x116** (Manage Production): same logic with module 0x14.
- **0x117** (Manage Maintenance): `FUN_00439e30` returns 0 for 0x117
  (unrecognized), so never checked.

---

## 3. DAT_006b28b0 bit 0x8000 — who reads it, what advice-on does

### Readers of 0x8000

Three functions test `DAT_006b28b0 & 0x8000`:

1. **FUN_00487900** (0x004879b0) — menu init. If bit clear (advice on),
   sets the checkmark on item 0x11e.

2. **FUN_0043a0b0** (0x0043a0b0) — advisor notification dispatcher.
   First line:
   ```c
   if (((DAT_006b28b0 & 0x8000) == 0) && (agent->active != 0))
   ```
   When advice is ON (bit clear) and agent is active, this function:
   - Calls `FUN_00439f20` to build the initial notification entry list
     (if `dispatch_flags & 0x10000000` is not yet set).
   - Matches bits in `dispatch_flags (+0x150)` to notification categories
     (1=item 6, 2=item 2, 4=item 3, 8=item 5, 0x10=item 4).
   - Dispatches matching entries to the advisor UI via `FUN_0048a590`.
   When bit 0x8000 is SET (advice off), the entire function is skipped.

3. **FUN_00439bc0** (0x00439bc0) — periodic advice timer. Checks:
   ```c
   if ((DAT_006b28b0 & 0x8000) == 0)   // advice ON
       && (agent->active != 0)
       && (last_dispatch_tick + 300 <= current_tick)
   ```
   When all three conditions hold, calls `FUN_00439fb0` which walks the
   notification entry list, matches entries against dispatch_flags bits,
   and sends matched entries to the advisor panel.

### What advice-on actually does

When **advice is ON** (bit 0x8000 clear):

- **Notification dispatch** (`FUN_0043a0b0`): proactive advisor entries
  from the notification_entries list (+0x154) are dispatched to the
  advisor panel (+0x144 → +0x6c) based on category flags.

- **Periodic advice** (`FUN_00439bc0`): every 300 ticks, the agent walks
  its notification list and sends unseen entries to the advisor.

- **Initial entry build** (`FUN_00439f20`): on first call, queries the
  game state via `FUN_0048b460` to populate the notification list, then
  dispatches "category 7" and `DAT_006b28d4`-triggered entries immediately.

When **advice is OFF** (bit 0x8000 set):

- `FUN_0043a0b0` returns immediately — no notifications dispatched.
- `FUN_00439bc0` returns immediately — no periodic advice.
- Advisor reactions from FUN_004c0d00 (toggle responses, ship-move
  responses) still fire — they use other bits, not 0x8000.

**Summary**: bit 0x8000 gates the advisor's *proactive* advice system.
Toggle reactions (garrison on/off, production on/off, ship-move feedback)
are independent — they fire regardless of the advice flag.

---

## 4. Initialization, difficulty, and save/load of bit 0x8000

### Initialization (FUN_00439320 — agent base constructor)

```c
DAT_006b28b0 = 0;                       // clear all bits
int mode = FUN_00401b00();               // reads DAT_006be3b8 + 0x104
if ((mode != 1) && (mode != 4)) {
    DAT_006b28b0 = DAT_006b28b0 | 0x8000;  // advice OFF
}
```

`FUN_00401b00` returns a game-mode value from the global game settings
object at `DAT_006be3b8 + 0x104`.

**Confirmed**: advice starts ON (bit clear) for modes 1 and 4, OFF for
all others. Per the manual (p. 22), Agent Advice is on for Easy and off
for Medium/Hard. This means:
- Mode 1 = Easy (advice ON)
- Mode 4 = hyp: another advice-on mode (tutorial? custom Easy?)
- Modes 2, 3, etc. = Medium/Hard/multiplayer (advice OFF)

### Subclass constructors

Both `FUN_004c0710` (alliance advisor) and `FUN_004c27f0` (empire
advisor) call `FUN_00439320` first, then set `DAT_006b28b0 |= 0x1000`
(Translate Counterpart starts on). The advice bit from FUN_00439320 is
preserved.

### Save/load

**Save** (`FUN_004397a0`): serializes DAT_006b28b0 as a uint in the
agent's save stream. Position: after `+0x04`, `+0x10`, `+0x14`, `+0x0c`,
`DAT_006b28bc`, `DAT_006b28c0`, then `DAT_006b28b0`.

**Load** (`FUN_00439550`): deserializes in the same order. DAT_006b28b0
is fully restored from the save file, including the 0x8000 bit. The
player's toggle state persists across save/load.

---

## 5. DAT_006b28b0 bits 0x10, 0x20, 0x100, 0x200, 0x1000000 — advisor reactions

These bits are set by `FUN_00439d60` (the Manage Garrisons / Manage
Production toggle handler) and consumed by `FUN_004c0d00` (the advisor
reaction processor).

### Writers — FUN_00439d60

| Bit        | Set when                          |
|------------|-----------------------------------|
| 0x10       | Garrisons (0x15) toggled OFF      |
| 0x20       | Production (0x14) toggled OFF     |
| 0x100      | Garrisons (0x15) toggled ON       |
| 0x200      | Production (0x14) toggled ON      |
| 0x1000000  | Toggle-off counter exceeds 3      |

When the player has toggled off more than 3 modules total (cumulative
counter at agent+0x160), the "too many toggles" bit 0x1000000 fires
instead of the specific off-bit.

### Readers — FUN_004c0d00

FUN_004c0d00 processes each bit one at a time, sets a timer in the
advisor reaction array (at `param_1+0x168`, indexed by slot), then clears
the bit. The timers trigger advisor animation/speech at specific offsets:

| Bit        | Advisor slot offset | Timer value      | Reaction |
|------------|---------------------|------------------|----------|
| 0x10       | +0x74               | DAT_006b28cc + 10  | Garrisons OFF response |
| 0x20       | +0x7c               | DAT_006b28cc + 10  | Production OFF response |
| 0x100      | +0x78               | DAT_006b28cc + 50  | Garrisons ON response |
| 0x200      | +0x80               | DAT_006b28cc + 50  | Production ON response |
| 0x1000000  | (not in FUN_004c0d00 directly) | — | hyp: "too many toggles" annoyance |

Additional bits consumed by FUN_004c0d00:

| Bit         | Advisor slot offset | Timer value          | Meaning |
|-------------|---------------------|----------------------|---------|
| 0x1         | +0x70               | DAT_006b28cc + 10   | Ship-move order acknowledged |
| 0x2000      | +0x124              | DAT_006b28cc + 40000| hyp: long-delay advice |
| 0x2000000   | +0x124..+0x134      | DAT_006b28cc + 5    | Variable slot (DAT_006b28b8 index) |
| 0x4000000   | +0x68               | DAT_006b28cc + 10   | hyp: agent deactivation response |
| 0x8000000   | +0x6c               | DAT_006b28cc + 50   | hyp: agent activation response |
| 0x80000000  | +0x138..+0x14c      | DAT_006b28cc + 5    | Random advisor quip (6 slots) |

When agent is inactive (+0x10 == 0), FUN_004c0d00 only checks bit
0x8000000 (activation) and then masks DAT_006b28b0 to `& 0xc000`,
preserving only bits 0x4000 and 0x8000 (the persistent toggles).

### Bit flow for Manage Garrisons toggle example

```
Player clicks "Manage Garrisons" (Alt+G):
  FUN_00486fb0 dispatches 0x115
    → FUN_00439d60(agent, 0x115)
      → If state == 2 (enabled): set state = 0, DAT_006b28b0 |= 0x10
      → If state != 2 (disabled): set state = 2, DAT_006b28b0 |= 0x100

Next tick, FUN_004c0d00 processes DAT_006b28b0:
  → Sees 0x10: sets advisor_reactions[0x74/4] = tick + 10, clears 0x10
  → Or sees 0x100: sets advisor_reactions[0x78/4] = tick + 50, clears 0x100

FUN_004c0a60 (advisor renderer) fires the reaction:
  → When tick reaches the timer value, plays the advisor animation/speech
  → If 0x1000 is set AND 0x4000 is clear, dispatches the message
```

---

## 6. Commands 0x110/0x111/0x112 — confirmed as Build Ships/Troops/Facilities

### TEXTSTRA confirmation

| Cmd   | TEXTSTRA ID | Label            |
|-------|-------------|------------------|
| 0x110 | 12560       | Build Ships      |
| 0x111 | 12561       | Build Troops     |
| 0x112 | 12562       | Build Facilities |

### Code path (FUN_00486fb0 → FUN_00439d10 → FUN_0049e360)

FUN_00486fb0 dispatches 0x110, 0x111, 0x112 to FUN_00439d10 (line 103):
```c
if (0x10f < (int)param_1) {
    FUN_00439d10(*(void **)((int)this + 0xc0), param_1);
    FUN_0041d5e0(0);
    return;
}
```

FUN_00439d10 maps each command to `FUN_0049e360(agent+0x184, param)`:
```c
0x110 → FUN_0049e360(agent+0x184, 2)  // Build Ships
0x111 → FUN_0049e360(agent+0x184, 3)  // Build Troops
0x112 → FUN_0049e360(agent+0x184, 1)  // Build Facilities
```

`agent+0x184` is the direct pointer to module 0x17 (the Build Ships
module, stored at construction time in FUN_00439550/FUN_00439950).

FUN_0049e360 sets `module+0x34 = param` (the build-type selector) and
resets the module's state machine (`+0x3c`, `+0x40` via FUN_004ece80):
```c
void FUN_0049e360(module, param) {
    module->build_type = param;    // +0x34
    FUN_004ece80(module + 0x3c);   // reset
    FUN_004ece80(module + 0x40);   // reset
}
```

**Confirmed**: all three commands target module 0x17. The param value
selects what to build:

| Param | Build type   | Command | Alt key |
|-------|--------------|---------|---------|
| 1     | Facilities   | 0x112   | F       |
| 2     | Ships        | 0x110   | B       |
| 3     | Troops       | 0x111   | T       |

Module 0x17 is a unified "build" module, not a ships-only module. The
manual labels it "Build Ships" in the Agent menu, but it handles all
three build types through the param-selected state machine.

---

## 7. DAT_006b28b0 complete bit map

| Bit         | Writer              | Consumer            | Meaning |
|-------------|---------------------|----------------------|---------|
| 0x1         | FUN_0043a2a0        | FUN_004c0d00 (+0x70) | Ship-move completed for watched system |
| 0x10        | FUN_00439d60        | FUN_004c0d00 (+0x74) | Garrisons automation OFF |
| 0x20        | FUN_00439d60        | FUN_004c0d00 (+0x7c) | Production automation OFF |
| 0x100       | FUN_00439d60        | FUN_004c0d00 (+0x78) | Garrisons automation ON |
| 0x200       | FUN_00439d60        | FUN_004c0d00 (+0x80) | Production automation ON |
| 0x1000      | FUN_00439e80, constructors | FUN_00487900, FUN_004c0a60 | Translate Counterpart state |
| 0x2000      | (untraced writer)   | FUN_004c0d00 (+0x124)| Long-delay advice trigger |
| 0x4000      | FUN_00487f80        | FUN_004c0a60         | hyp: agent active notification |
| 0x8000      | FUN_00439e80, FUN_00439320 | FUN_0043a0b0, FUN_00439bc0, FUN_00487900 | Agent Advice OFF (set = disabled) |
| 0x1000000   | FUN_00439d60        | (untraced)           | "Too many toggles" annoyance |
| 0x2000000   | FUN_00439ed0        | FUN_004c0d00         | Variable advisor reaction |
| 0x4000000   | (untraced writer)   | FUN_004c0d00 (+0x68) | Agent deactivation response |
| 0x8000000   | (untraced writer)   | FUN_004c0d00 (+0x6c) | Agent activation response |
| 0x10000000  | FUN_0049e6b0        | FUN_0043a2a0         | Ship module dispatched a move |
| 0x80000000  | FUN_00439ef0        | FUN_004c0d00 (+0x138)| Random advisor quip |

On agent-inactive transition, FUN_004c0d00 masks to `& 0xc000`,
preserving only 0x4000 and 0x8000.

---

## Cited decompiles

| Function       | Role |
|----------------|------|
| FUN_00422ce0.c | Galaxy view WndProc — WM_RBUTTONUP trigger, timer dispatch |
| FUN_0042d050.c | Popup menu builder — routes 0xdead to Agent menu |
| FUN_0041d160.c | Menu list loader — calls FUN_00436650 |
| FUN_00436650.c | Agent menu init thunk — calls FUN_00487900 |
| FUN_00487900.c | Agent menu enable/check logic |
| FUN_00486fb0.c | Command dispatcher — routes 0x110-0x11e |
| FUN_00439d10.c | Build command router — 0x110/0x111/0x112 to module 0x17 |
| FUN_0049e360.c | Build-type setter on module 0x17 |
| FUN_00439d60.c | Manage Garrisons/Production toggle |
| FUN_00439e80.c | Advice/Translate Counterpart toggle |
| FUN_00439e30.c | Module state query for checkmarks |
| FUN_00439320.c | Agent base constructor — initializes DAT_006b28b0 |
| FUN_004c0710.c | Alliance advisor subclass constructor |
| FUN_004c27f0.c | Empire advisor subclass constructor |
| FUN_00401b00.c | Game mode getter (DAT_006be3b8 + 0x104) |
| FUN_004397a0.c | Agent save — serializes DAT_006b28b0 |
| FUN_00439550.c | Agent load — deserializes DAT_006b28b0 |
| FUN_0043a0b0.c | Advisor notification dispatcher (0x8000 gate) |
| FUN_004c0d00.c | Advisor reaction processor (bit consumer) |
| FUN_00439bc0.c | Periodic advice timer (0x8000 gate) |
| FUN_00439f20.c | Initial notification list builder |
| FUN_00439fb0.c | Periodic notification scanner |
| FUN_00439ef0.c | Random quip trigger (0x80000000) |
| FUN_00439ed0.c | Variable reaction trigger (0x2000000) |
| FUN_0043a2a0.c | Ship-move completion handler (0x10000000 → 0x1) |
| FUN_00487f80.c | Agent active notification (0x4000) |
| FUN_004c0a60.c | Advisor animation renderer (reads 0x1000, 0x4000) |
| FUN_0049e6b0.c | Module 0x17 execute — sets 0x10000000 on ship move |

## Open

- The exact advisor animation/speech text triggered by each reaction slot
  (offsets +0x68..+0x14c in the reaction array). These are likely TEXTSTRA
  ids or SPT/BIN animation indices, but the mapping from slot to visible
  advisor content is untraced.
- What game mode value 4 represents (FUN_00401b00). Mode 1 = Easy is
  confirmed by manual p. 22. Mode 4 may be a tutorial variant.
- The writer of bit 0x4000000 (agent deactivation trigger). FUN_00487f80
  writes 0x4000, not 0x4000000.
- Whether items 0x117 (Manage Maintenance), 0x118 (Not An Operation),
  0x119 (Deactivate), 0x11a (Reactivate), 0x11d (Message Alerts) were
  ever functional in development or were always dead entries.
- The bit 0x1000000 ("too many toggles") consumer — FUN_004c0d00 does
  not test this bit. It may be consumed by a different function or may be
  an unused trigger.
