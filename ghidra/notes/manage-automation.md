---
title: "Manage Garrisons / Manage Production — Player-Side Agent Automation"
description: "Full reverse engineering of the Agent menu's Manage Garrisons and Manage Production toggles: module architecture, tick dispatch, decision rules, menu wiring, save format, and advisor messages"
category: "ghidra"
created: 2026-10-06
updated: 2026-10-06
sources:
  - type: "ghidra"
    files:
      - "FUN_00439320.c"
      - "FUN_00439550.c"
      - "FUN_00439950.c"
      - "FUN_00439a10.c"
      - "FUN_00439cd0.c"
      - "FUN_00439d60.c"
      - "FUN_00439e30.c"
      - "FUN_00439e80.c"
      - "FUN_00439f20.c"
      - "FUN_004397a0.c"
      - "FUN_0043a0b0.c"
      - "FUN_0041cdf0.c"
      - "FUN_00436020.c"
      - "FUN_00486fb0.c"
      - "FUN_00487900.c"
      - "FUN_00487c20.c"
      - "FUN_00487c30.c"
      - "FUN_00520690.c"
      - "FUN_00520720.c"
      - "FUN_005f4960.c"
      - "FUN_005f5500.c"
      - "FUN_004cc990.c"
      - "FUN_004c7550.c"
      - "FUN_0049e010.c"
      - "FUN_0049e130.c"
      - "FUN_0049df20.c"
      - "FUN_0049df70.c"
      - "FUN_004c6ad0.c"
      - "FUN_004c6ca0.c"
      - "FUN_004c6ce0.c"
      - "FUN_004c6d90.c"
      - "FUN_004c6f00.c"
      - "FUN_004c70e0.c"
      - "FUN_004c7220.c"
      - "FUN_004c7340.c"
      - "FUN_004c7510.c"
      - "FUN_004c7580.c"
      - "FUN_004c7650.c"
      - "FUN_004c7860.c"
      - "FUN_004c79e0.c"
      - "FUN_004c7a90.c"
      - "FUN_004c7bd0.c"
      - "FUN_004c7d20.c"
      - "FUN_004ec940.c"
      - "FUN_0049e1c0.c"
      - "FUN_0049e6b0.c"
      - "FUN_0049e380.c"
      - "FUN_004878f0.c"
      - "FUN_00509670.c"
      - "FUN_004c7cd0.c"
      - "FUN_0049dad0.c"
  - type: "manual"
    pages: "64-66 (PDF pp. 62-64)"
---

# Manage Garrisons / Manage Production — Player-Side Agent Automation

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

The original game's Agent menu offers three automation toggles that instruct the
player's advisor to manage garrisons, production, and ship building
autonomously.  All three start off (see the correction above) and share a common sub-module
dispatch system.  The AI opponent uses a completely separate pipeline
(FUN_00519d00 → FUN_00537180 chain) and does not share this code.

## Manual Reference (pp. 64–66)

- **Alt+G  Manage Garrisons** — Instructs the Agent to fulfill garrison
  requirements (toggle).
- **Alt+U  Manage Production** — Instructs the Agent to build mines and
  refineries to maximize resources (toggle).
- **Alt+B  Build Ships** — Directs the Agent to build a ship using the nearest
  available shipyard.

All three are under the Agent menu.  Additional Agent toggles include Build
Troops (Alt+T), Build Facilities (Alt+F), Translate Counterpart (Alt+V), Agent
Advice (Alt+A), and Game Objectives (Alt+H).

---

## 1.  Agent Object

The player-side agent is a C++ object stored at **galaxyView+0xc0**.
Constructor: `FUN_00439320(game_ptr, view_ptr)`.

### Key Fields

| Offset    | Type       | Field                | Notes |
|-----------|------------|----------------------|-------|
| +0x00     | void*      | vtable               | PTR_FUN_00658e10 |
| +0x04     | int        | side                 | 1 at init |
| +0x10     | int        | active               | 1 at init |
| +0x14     | void*      | game_instance        | Passed as param_1 |
| +0x18     | BST        | sub_module_map       | std::map<uint, SubModule*>; key = module index |
| +0x34     | SubModule* | current_module       | Currently executing module |
| +0x3c     | struct     | notification_queue   | Processed in agent tick state 5 |
| +0xc0     | void*      | back-ref (hyp:)      | Loaded from +0xc0 in some callers |
| +0x138    | struct     | advisor_reactions     | FUN_004e5210 |
| +0x144    | void*      | view_ptr             | The galaxy view, used for UI dispatch |
| +0x148    | int        | last_dispatch_tick   | Tick of last notification |
| +0x14c    | int        | dispatch_interval    | 8 at init |
| +0x150    | uint       | dispatch_flags       | Bit 0x10000000 = init complete |
| +0x154    | list       | notification_entries | Linked list of advisor entries |
| +0x160    | int        | toggle_off_counter   | Incremented on each disable |
| +0x170    | byte       | advisor_byte         | 8 at init |
| +0x184    | SubModule* | ships_module_ptr     | Direct pointer to module 0x17 |
| +0x188    | int        | orders_issued        | Count of orders dispatched this cycle |
| +0x18c    | int        | orders_limit         | Set to 5 in FUN_00439950 |

### Global State

| Global        | Type | Meaning |
|---------------|------|---------|
| DAT_006b28b0  | uint | Advisor message flags (see bit map below) |
| DAT_006b28b8  | uint | Secondary advisor state |
| DAT_006b28bc  | uint | Serialized advisor field |
| DAT_006b28c0  | uint | Serialized advisor field |
| DAT_006b28c4  | uint | Serialized advisor field |
| DAT_006b28c8  | uint | Serialized advisor field |
| DAT_006b28d0  | uint | Zeroed at init |
| DAT_006be4e4  | void*| Tactical battle window ptr; 0 = strategic mode |

---

## 2.  Sub-Module System

### Factory (FUN_0049e010)

Creates a polymorphic sub-module object based on index:

| Index | Size   | Constructor    | vtable         | Label (manual)     |
|-------|--------|----------------|----------------|--------------------|
| 0x14  | 0x58 B | FUN_004c6ad0   | 0x0065c630     | Manage Production  |
| 0x15  | 0x5c B | FUN_004c7340   | 0x0065c650     | Manage Garrisons   |
| 0x17  | 0x58 B | FUN_0049e1c0   | 0x0065bc98     | Build Ships        |

### Sub-Module Object Layout (Base Class — FUN_004c7d20)

| Offset | Type  | Field           | Notes |
|--------|-------|-----------------|-------|
| +0x00  | void* | vtable          | Polymorphic |
| +0x04  | void* | BST left        | BST tree linkage |
| +0x08  | void* | BST right       | BST tree linkage |
| +0x10  | void* | BST next        | Linked iteration |
| +0x14  | void* | BST iterator    | Round-robin cursor |
| +0x18  | uint  | module_index    | 0x14, 0x15, or 0x17 |
| +0x1c  | uint  | state           | 0=disabled, 1=ready, 2=enabled |
| +0x20  | uint  | completed       | 1 = done, agent should reset |
| +0x24  | uint  | field_24        | Flags / condition |
| +0x28  | void* | agent_context   | Set by VT[3]; the agent's game-level context |
| +0x2c  | void* | side_data       | Faction/side reference |
| +0x30  | uint  | priority        | Attempt counter; scheduler picks highest |
| +0x34  | uint  | sub_phase_a     | Phase-specific state |
| +0x38  | uint  | phase           | State machine step (0–5) |

### vtable Layout

| Slot | Offset | Method | Base (0x0065c670) | Production (0x0065c630) | Garrison (0x0065c650) |
|------|--------|--------|-------------------|-------------------------|-----------------------|
| 0 | +0x00 | destructor | FUN_004c7d00 | FUN_004c6ab0 | FUN_004c7320 |
| 1 | +0x04 | save | FUN_004c7d70 | FUN_004c6be0 | FUN_004c7450 |
| 2 | +0x08 | load | FUN_004c7dc0 | FUN_004c6c40 | FUN_004c74b0 |
| 3 | +0x0c | set_context | FUN_0049eca0 | FUN_0049eca0 | FUN_0049eca0 |
| 4 | +0x10 | ready_check | FUN_00618b80 (nop) | FUN_004cc990 | FUN_004cc990 |
| 5 | +0x14 | execute | — | FUN_004c6ca0 | FUN_004c7510 |
| 6 | +0x18 | complete | — | FUN_004c7550 | FUN_004c7550 |

### State Transitions

```
  ┌──── toggle ON ────┐
  │                    │
  ▼                    │
 [2] enabled/idle      │
  │                    │
  │ VT[4] skips 2      │
  │                    │
  ▼                    │
 [0] disabled ◄── toggle OFF
  │
  │ VT[4]: 0→1
  ▼
 [1] ready (scheduled)
  │
  │ VT[5]: execute
  │   └─ phases run, produce orders
  │
  ▼ on completion (+0x20=1):
  if state==2 → stay 2 (enabled)
  if state!=2 → reset to 0 (was disabled)
```

### Initialization (FUN_00439950, Phase 3)

```c
FUN_0049df20(agent_bst, 0x14, queue);  // Create Production module
FUN_0049df20(agent_bst, 0x15, queue);  // Create Garrison module
FUN_0049df20(agent_bst, 0x17, queue);  // Create Ships module
// Set ALL modules to state 2 (enabled):
for (node = first(bst); node != NULL; node = node->next) {
    node->state = 2;
}
```

All three modules start **enabled** at game start.

---

## 3.  Toggle Mechanism — FUN_00439d60

Called from `FUN_00486fb0:141–146` with menu command IDs:

| Menu ID | Module Index | DAT_006b28b0 ON bit | DAT_006b28b0 OFF bit |
|---------|-------------|---------------------|----------------------|
| 0x115   | 0x15 (Garrisons) | 0x100 | 0x10 |
| 0x116   | 0x14 (Production) | 0x200 | 0x20 |

```c
void FUN_00439d60(agent, cmd_id) {
    node = BST_lookup(agent+0x18, module_index);
    if (node->state == 2) {
        // DISABLE: 2 → 0
        node->state = 0;
        agent->toggle_off_counter++;
        if (counter > 3)
            DAT_006b28b0 |= 0x1000000;  // "too many toggles" message
        else
            DAT_006b28b0 |= off_bit;    // "automation off" message
    } else {
        // ENABLE: → 2
        node->state = 2;
        agent->toggle_off_counter--;
        DAT_006b28b0 |= on_bit;         // "automation on" message
    }
}
```

---

## 4.  Menu System — FUN_00487900

Sets up Agent menu items during WM_INITMENUPOPUP.

```c
void FUN_00487900(view, menu) {
    if (view->agent == NULL) return;

    uint agent_active = FUN_00487c20(view);    // → agent->active (+0x10)
    bool ships_ok     = FUN_00487c30(view);    // → agent->ships_module state==2

    uint build_enabled = agent_active & ships_ok;

    EnableMenuItem(menu, 0x110, build_enabled);  // Build Ships
    EnableMenuItem(menu, 0x111, build_enabled);  // Build Troops
    EnableMenuItem(menu, 0x112, build_enabled);  // Build Facilities
    EnableMenuItem(menu, 0x115, agent_active);   // Manage Garrisons
    EnableMenuItem(menu, 0x116, agent_active);   // Manage Production
    EnableMenuItem(menu, 0x113, 1);              // Agent Status
    EnableMenuItem(menu, 0x11b, agent_active);   // Translate Counterpart
    EnableMenuItem(menu, 0x114, 1);              // Game Objectives
    EnableMenuItem(menu, 0x11e, agent_active);   // Agent Advice

    // Translate Counterpart checked when bit 0x1000 set
    if (DAT_006b28b0 & 0x1000)
        CheckMenuItem(menu, 0x11b);

    // Agent Advice checked when bit 0x8000 clear (advice ON)
    if (!(DAT_006b28b0 & 0x8000))
        CheckMenuItem(menu, 0x11e);

    // Manage Garrisons/Production checked when module NOT active
    // FUN_00439e30 returns 1 when state != 2 (disabled)
    if (FUN_00439e30(agent, 0x115))  // Garrisons disabled?
        CheckMenuItem(menu, 0x115);
    if (FUN_00439e30(agent, 0x116))  // Production disabled?
        CheckMenuItem(menu, 0x116);
    if (FUN_00439e30(agent, 0x117))  // hyp: Build Ships disabled?
        CheckMenuItem(menu, 0x117);
}
```

**hyp:** The checkmark semantics are inverted from modern convention.  A
check appears when the module is OFF.  This may be a "needs attention"
indicator rather than a "currently active" marker, or the original menu
strings may read differently ("Manual Garrisons" checked = manual mode).

---

## 5.  Accelerator Keys — FUN_00422ce0:1468–1479

Gated on:
- `DAT_006be4e4 == 0` — no tactical battle in progress
- `param_1[0x30] == 1` — strategic view in default (non-targeting) mode
- Menu item exists and is enabled (+0x1c != 0)

```
Alt+G → cmd 0xbc5 → menu 0x115 → FUN_0041cdf0 → FUN_00436020 → FUN_00486fb0
Alt+U → cmd 0xbc6 → menu 0x116 → same chain
```

---

## 6.  Agent Tick — FUN_00439a10

Called each day-tick.  Cycles through three states:

### State 4 — Execute a Sub-Module

1. If no current module: call `FUN_0049df70` (scheduler) to pick the next
   ready (state 1) module.
2. If module found AND (`orders_issued < orders_limit` OR module == 0x17):
   - Reset module priority to 0.
   - Call **VT[5]** (execute).  Returns an order object, or NULL if more
     phases remain.
   - If order returned:
     - Call order→VT[7] to check compatibility.
     - If compatible: call module→VT[6] (complete) to finalize.
     - Else: dispatch via `FUN_004878f0` to the game and increment
       `orders_issued` (unless module == 0x17).
3. If module's `+0x20` (completed) flag is set:
   - If `state != 2`: set `state = 0` (re-disable if player toggled off).
   - Clear `current_module`.
4. Transition to state 5.

### State 5 — Process Notification Queue

Calls `FUN_0049cc80` on the queue at `agent+0x3c`.

### State 6 — Ready All Modules

Walks every module in the BST.  Calls **VT[4]** (FUN_004cc990) on each:

```c
void FUN_004cc990(module) {
    if (module->state != 1 && module->state != 2) {
        module->state = 1;       // → ready
        module->completed = 0;   // clear completion flag
    }
}
```

This re-readies disabled modules (state 0 → 1), but they harmlessly fail
during execution and get reset back to 0 in state 4.

### Scheduler — FUN_0049df70

1. Call `FUN_0049e130`: scan BST for highest-priority state-1 module.
2. If none found: round-robin through the BST looking for any state-1 module.
3. Return the selected module (or NULL).

---

## 7.  Production Module (0x14) — Decision Logic

**VT[5]**: FUN_004c6ca0 → FUN_004c6ce0

### Phase 1 — FUN_004c6d90: Find a System with Idle Construction Capacity

- Queries the agent's world model for facility-type entities (flag 0x44000,
  sort priority 10, owned by player).
- Refines with flag 0x204, mask 0x1000 to find idle/available facilities.
- Validates the result is a star system (family 0x90–0x97).
- Gets the first available construction slot via `FUN_00509670(system, 1)`.

### Phase 2 — FUN_004c6f00: Decide Mine or Refinery

- Compares `agent_context+0xa8` vs `agent_context+0xac`:
  - **hyp:** raw material production rate vs. refined resource production rate.
- If raw < refined: queue a **mine** (entity type 0x2c, search flags 0x180).
- If raw >= refined: queue a **refinery** (entity type 0x2d, search flags 0x80).
- Stores the chosen facility DatId at `module+0x44`.
- Runs multiple sub-queries (sorts 5, 6, 7) to find the best location.

### Phase 3 — FUN_004c70e0: Create Build-Facility Order (0x214)

- Validates `module+0x3c` is a star system and `module+0x40` is a valid
  build-site entity (family 0xa0–0xa1).
- Creates order object via `FUN_004f5cd0(0x214)`:
  - Sets faction from `module+0x2c`.
  - Sets build target list and destination system.

### Phase 4 — FUN_004c7220: Create Start-Construction Order (0x210)

- Validates `module+0x40` is a valid build-site entity.
- Creates order object via `FUN_004f5cd0(0x210)`:
  - Sets target, faction, build specification from `module+0x44`.
  - Sets build quantity = **1**.

### Summary

The Production module cycles: find idle construction yard → decide mine vs.
refinery based on resource balance → issue a build order → issue a start-
construction order.  It builds **one facility per cycle**.

---

## 8.  Garrison Module (0x15) — Decision Logic

**VT[5]**: FUN_004c7510 → FUN_004c7580

### Phase 2 (default entry) — FUN_004c7650: Find Under-Garrisoned Systems

- Queries for owned systems needing garrisons (entity type 0x21000000,
  sort 0x19 — **hyp:** priority by garrison deficit).
- Refines with flag 0x10000005, sort 0x14 to match troop-eligible systems.
- **Fallback**: if no result, tries entity type 0x1000000 with secondary
  criteria.

### Phase 1 — FUN_004c7860: Identify Training Capacity at Target

- Queries for entities with flag 0x22000 referencing the target system from
  phase 2, sorted by 2 (closest with capacity).
- Refines with flag 0x104, mask 0x800 to find systems with available training.
- Gets training facility slot via `FUN_00509670(system, 2)`.

### Phase 3 — FUN_004c79e0: Calculate Troop Deficit

```c
deficit = agent_context->garrison_need (+0x80)
        - agent_context->current_troops (+0x84);
if (deficit > 0) {
    yard = find_construction_yard(target);  // FUN_004c7cd0
    if (yard->flags & 1)  // yard type bit
        unit_capacity = 1;
    else
        unit_capacity = 0x2000;
    available = count_available(type=0x29, yard, ...);
    build_count = min(deficit / unit_capacity, available);
}
```

Stores calculated count at `module+0x3c`.

### Phase 4 — FUN_004c7a90: Create Build Order (0x214)

- Validates `module+0x40` is a star system (0x90–0x97) and `module+0x44` is
  a training facility (0xa4–0xa5).
- Creates order object 0x214 for the training facility at the target system.

### Phase 5 — FUN_004c7bd0: Create Train-Troops Order (0x212)

- Validates `module+0x44` is a training facility (0xa4–0xa5).
- Creates order object `FUN_004f5cd0(0x212)`:
  - Sets training facility entity.
  - Sets troop specification from `module+0x48`.
  - Sets build quantity = `module+0x3c` (the calculated deficit).

### Summary

The Garrison module cycles: find the most under-garrisoned system → locate a
training facility near it → calculate how many troops are needed → issue
build + train orders.  It trains **deficit-many troops per cycle**, clamped
by available training capacity.

---

## 9.  DAT_006b28b0 — Advisor Message Flags

| Bit        | Set When | Meaning |
|------------|----------|---------|
| 0x10       | FUN_00439d60 | Garrison automation just turned OFF |
| 0x20       | FUN_00439d60 | Production automation just turned OFF |
| 0x100      | FUN_00439d60 | Garrison automation just turned ON |
| 0x200      | FUN_00439d60 | Production automation just turned ON |
| 0x1000     | FUN_00439e80 | Translate Counterpart toggle state |
| 0x8000     | FUN_00439e80, FUN_00439320 | Agent Advice OFF (set = disabled) |
| 0x1000000  | FUN_00439d60 | "Too many toggles" — counter > 3 |
| 0x10000000 | FUN_0049e6b0 | Ships module dispatched a move order |

At game init (FUN_00439320), if game mode is not 1 or 4:
`DAT_006b28b0 |= 0x8000` (Agent Advice defaults to OFF in multiplayer).

### Save Format

DAT_006b28b0 is serialized and deserialized through the agent's save/load
functions (FUN_004397a0 / FUN_00439550).  It is a single uint in the save
stream, positioned after the agent's core fields.

---

## 10.  Save / Load

### Agent Save — FUN_004397a0

Serializes (in order): `+0x04`, `+0x10`, `+0x14`, `+0x0c`,
`DAT_006b28bc`, `DAT_006b28c0`, `DAT_006b28b0`, `DAT_006b28c4`,
`DAT_006b28c8`, queue at `+0x3c`, advisor at `+0x138`, `+0x38`,
BST at `+0x18`, `+0x170`, `+0x160`, `DAT_006b28b8`, `+0x164` (count),
`+0x188`, `+0x18c`, `+0x190`, dynamic array at `+0x168`, `+0x148`,
`+0x14c`, flags at `+0x150`, linked list at `+0x154`.

### Agent Load — FUN_00439550

Deserializes in the same order.  After loading the BST (via FUN_0049dd50),
it looks up module 0x17 and stores its pointer at `+0x184`, setting
`module+0x54` to point back to the agent.

### Sub-Module Save/Load

Each module's VT[1]/VT[2] serializes its type-specific fields:

**Production** (FUN_004c6be0 / FUN_004c6c40):
base fields, `+0x34`, `+0x38`, DatId at `+0x3c`, DatId at `+0x40`,
iterator at `+0x48`, DatId at `+0x44`.

**Garrison** (FUN_004c7450 / FUN_004c74b0):
base fields, `+0x34`, `+0x3c` (count), `+0x38`, DatId at `+0x40`,
DatId at `+0x44`, iterator at `+0x4c`, DatId at `+0x48`.

---

## 11.  Relationship to AI Pipeline

The computer opponent's AI uses a **separate pipeline** that does not share
any code with the player-side Agent modules:

| AI Pipeline Function | Purpose |
|---------------------|---------|
| FUN_00519d00 | Galaxy-wide system categorization (7 buckets) |
| FUN_00537180 | Per-system fleet deployment decisions |
| FUN_00502020 | Garrison/fleet strength aggregation |
| FUN_00508250 | Pre-deployment validation (18 sub-checks) |
| FUN_00508660 | Entity dispatch by family type |

The Rust port's `crates/rebellion-core/src/ai.rs` implements the AI-side
logic (garrison_strength scoring, troop_garrison_min thresholds, AI fleet
deployment).  It has **no equivalent** for the player-side Agent automation
modules.

---

## 12.  Port Requirements

To port the player-side Agent automation:

1. **Sub-module registry** — a map from module index (0x14, 0x15, 0x17) to
   a trait object with `execute()`, `complete()`, `save()`, `load()`.  All
   three start enabled.

2. **Toggle** — Agent menu items flip module state between 0 and 2.  Track
   a toggle counter; if > 3, fire an advisor annoyance message.

3. **Tick dispatch** — each day-tick, cycle: pick highest-priority ready
   module → execute its state machine → dispatch resulting orders → process
   notifications → re-ready all modules via VT[4].  Cap at 5 orders per
   cycle (except Ships module which is uncapped).

4. **Production logic** — compare raw-material vs. refined-resource rates;
   build a mine if raw < refined, a refinery otherwise; one facility per
   cycle.

5. **Garrison logic** — find the most under-garrisoned owned system; locate
   a nearby training facility; calculate troop deficit; issue train order
   for deficit-many troops, clamped by training capacity.

6. **Advisor messages** — DAT_006b28b0 bits drive advisor notifications.
   Map each bit to a message string and display through the advisor system.

7. **Save/load** — serialize module states, phase counters, cached DatIds,
   and the advisor flags bitfield.

---

## Open

- The entity query functions (FUN_0049d610, FUN_0049d770, FUN_0049da10,
  FUN_0049da70) use flag/sort parameters whose exact semantics need further
  tracing.  The flag values (0x44000, 0x21000000, 0x22000, 0x1000000)
  likely encode entity type filters and ownership masks.

- Family bytes 0xa0–0xa5 in the order targets need verification against
  the entity type tables.  **hyp:** 0xa0–0xa1 may be construction-yard
  slots and 0xa4–0xa5 training-facility slots, but these are not among the
  known family ranges in entity-system.md.

- Menu items 0x117 and 0x118 are also handled by FUN_00439d60 (via
  FUN_00486fb0:141–146), but the factory (FUN_0049e010) only creates
  modules 0x14, 0x15, and 0x17.  **hyp:** 0x117 maps to module 0x17
  (Build Ships) and 0x118 to another automation (Build Troops?).
  FUN_00439d60 would need additional cases for these.

- The resource comparison in FUN_004c6f00 (`agent_context+0xa8` vs
  `+0xac`) needs mapping to known GNPRTB parameters or the economy
  system's resource fields.

- The checkmark inversion (checked = disabled) may reflect original menu
  string phrasing or a Win32 convention used in 1998-era LucasArts code.

- Whether the disabled-module VT[4] re-ready cycle is intentional (allowing
  one-shot execution after disable) or an oversight needs testing against
  the original binary.
