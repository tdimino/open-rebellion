# Mission & Event Cookbook — REBEXE.EXE Reverse Engineering

*A practical reference for modders who want to add new missions, story events, or era-specific campaigns to Star Wars Rebellion. Built from Ghidra decompilation of REBEXE.EXE (2.8MB), cross-referenced with rebellion-strings.txt (205 strings), rebel-all-functions.txt (4,938 annotated functions), and the 50+ decompiled C files in ghidra/notes/.*

*All game logic lives in REBEXE.EXE. STRATEGY.DLL and TACTICAL.DLL are resource-only. For open-rebellion (the Rust reimplementation), the event chain equivalents live in `rebellion-core/src/events.rs` and `rebellion-core/src/missions.rs`.*

---

## 1. Mission Type Registry

### The Nine Mission Types

REBEXE.EXE recognizes 9 active mission type codes, dispatched via a 13-case switch in `FUN_0050d5a0`. Cases 0 and default are no-ops or guard rails. Each mission type has:

1. An integer type code (`param_1` to FUN_0050d5a0)
2. A matching `*MSTB.DAT` probability table file
3. A set of entity family filters defining the valid target pool
4. Boolean flags controlling decoy eligibility, special forces eligibility, and mandatory status

| Type Code | Mission | MSTB File | Notes |
|-----------|---------|-----------|-------|
| 1 | Diplomacy | DIPLMSTB.DAT | Fleet filter only (no troop/side filters) |
| 2 | Recruitment | RCRTMSTB.DAT | Target filter via `local_14c[2]` |
| 3 | Sabotage | SBTGMSTB.DAT | Two entity family ranges: 0x10/0x20 and 0x22/0x28 |
| 4 | Abduction | ABDCMSTB.DAT | Two ranges: 0xa0/0xb0 and 0x28/0x30 |
| 5 | Incite Uprising | INCTMSTB.DAT | Two ranges: 0x08/0x10 and 0x01/0xff; fleet enabled |
| 6 | Assassination | ASSNMSTB.DAT | Fleet + side filter + `special_flag=true`; two ranges |
| 7 | Rescue | RESCMSTB.DAT | Three ranges (0x08-0x10, 0x01-0xff, 0x10-0x20); special flag; `bVar3=true` decoy enable |
| 8 | Subdue Uprising | SUBDMSTB.DAT | Six ranges; `local_150=1`; `bVar1=true` (vtable path) |
| 9 | Espionage | ESPIMSTB.DAT | Single range 0x30/0x40 |
| 10 | (Death Star Sabotage) | DSSBMSTB.DAT | `bVar1=true`; three ranges; no fleet filter |
| 11 | (unknown) | — | `bVar1=true`; two ranges 0x10-0x40, 0xa0-0xb0 |
| 12 | (unknown) | — | `bVar2=true`; single range 0x20-0x30 |

**Note on case 8 (Subdue Uprising)**: This is the only mission that routes to the vtable dispatch path (`bVar1=true`). Cases 8, 10, and 11 take the `(**(code **)(*local_174 + 0x24))(local_14c + 5)` path instead of the standard `FUN_0050fc80` / `FUN_0050fbf0` dual-check.

#### Mission Probability Table Format (`*MSTB.DAT`)

All `*MSTB.DAT` files use the `IntTableEntry` pattern (16 bytes per entry):
```
id:         u32  — entry index
field2:     u32  — reserved
threshold:  i32  — signed skill delta (negative = below average, 0 = average, positive = above average)
value:      u32  — probability at this threshold (0-100)
```

The probability curve is piecewise-linear between threshold/value pairs. The quadratic formula used at runtime (from `rebellion-core/src/missions.rs`) is:
```
prob = clamp(a·score² + b·score + c, min%, max%)
```
where coefficients are fit to the MSTB curve at runtime.

| File | Entries |
|------|---------|
| DIPLMSTB.DAT | 10 |
| ESPIMSTB.DAT | 12 |
| ASSNMSTB.DAT | 12 |
| INCTMSTB.DAT | 13 |
| DSSBMSTB.DAT | 12 |
| ABDCMSTB.DAT | 12 |
| RCRTMSTB.DAT | 11 |
| RESCMSTB.DAT | 12 |
| SBTGMSTB.DAT | 12 |
| SUBDMSTB.DAT | 13 |

Additional tables consulted during mission resolution:
- `FOILTB.DAT` (14 entries) — counter-intelligence foil probability
- `ESCAPETB.DAT` (9 entries) — character escape after capture
- `FDECOYTB.DAT` (14 entries) — fleet decoy success
- `TDECOYTB.DAT` (14 entries) — troop decoy success
- `RLEVADTB.DAT` (14 entries) — rebel evasion

### Mission Dispatch Flow (`FUN_0050d5a0`, 503 lines)

```
FUN_0050d5a0(this, mission_type, side_id, target_entity_id)
│
├── FUN_00539fd0(this, side_id)       → resolve mission manager for faction
│   └── Returns local_174 (mission context object)
│
├── switch(mission_type):             → set entity family filter arrays
│   └── cases 1-12 set:
│       local_11c[2]  — primary family range (lo, hi)
│       local_114[8]  — secondary family range
│       local_10c[8]  — tertiary family range
│       local_104[8]  — quaternary family range
│       local_16c[4]  — fleet filter flag
│       local_14c[2]  — target type flag
│       local_154     — side filter flag
│       bVar1/bVar2   — vtable vs standard dispatch
│       bVar3         — decoy eligibility
│
├── if bVar1:
│   └── (**(code **)(*local_174 + 0x24))(target)   → vtable dispatch (Subdue/DS Sab)
│
├── if bVar2:
│   ├── FUN_0050fc80(local_174, target)             → validate target
│   ├── FUN_0050fbf0(local_174, target)             → validate mission can proceed
│   ├── FUN_004f9ca0(local_174, target)             → check side affiliation
│   │
│   ├── if fleet flag:
│   │   ├── FUN_005103a0(local_174, target)         → fleet presence check
│   │   └── FUN_0050f390(local_174, target)         → fleet validation
│   │
│   ├── if side flag:
│   │   └── FUN_0050fd10 / FUN_0050f840             → side filter checks
│   │
│   └── if target type flag:
│       └── FUN_0050f410 / FUN_0050f490             → target type validation
│
├── iterate entity family ranges (up to 4):
│   └── FUN_004f7e30(mission_ctx, range, target)   → per-entity validation pass
│
├── if bVar3 (decoy eligible):
│   └── FUN_0050eea0(mission_ctx, target)           → decoy check
│
└── if all checks pass:
    ├── FUN_004ffe70(auStack_2c, local_174, 4)      → build team candidate list
    ├── FUN_00513120(auStack_2c)                    → iterate candidates (outer loop)
    │   └── FUN_00502db0(auStack_4c, candidate, 4) → build sub-candidate list
    │       └── FUN_00513120(auStack_4c)            → iterate sub-candidates (inner loop)
    │           └── FUN_004f7e30(sub, range, target) → sub-candidate validation
    └── return bVar9 (true = mission launched)
```

### How to Add a New Mission Type (REBEXE.EXE Patch Path)

1. **Assign a new type code**: choose the next available integer after case 12. Set `bVar2 = true` for the standard dispatch path.

2. **Set entity family filters**: in the new switch case, populate `local_11c[2]` with `{lo, hi}` family range byte pair. Add additional ranges to `local_114`, `local_10c`, `local_104` if needed. Each pair is `{min_family_byte, max_family_byte}`.

3. **Create a new `*MSTB.DAT`-format table**: copy any existing MSTB file and edit threshold/value pairs to set the skill curve for your mission's success probability.

4. **Add a MISSNSD.DAT entry**: MISSNSD.DAT has 25 entries (112 bytes each). Add one with:
   - `special_force_eligibility` bitmask matching your `SPECFCSD.DAT` entry
   - `max_officers` — max assignable
   - `base_duration` — base turns
   - `target_flags` — `0x10000` if can target enemy systems
   - Set `flag_col6..flag_col21` based on nearest analogous mission

5. **Register the MSTB table**: the mission outcome code reads MSTB tables keyed by mission type code. The binding is in the dispatch chain downstream of `FUN_0050d5a0`. The table must be registered under the new type code at the lookup site.

### How to Add a New Mission Type (open-rebellion Path)

For the Rust reimplementation, no binary patching is needed:

```rust
// In rebellion-core/src/missions.rs — add to MissionKind enum:
pub enum MissionKind {
    Diplomacy,
    Recruitment,
    // ... existing ...
    ThrawnCampaign,  // new
}

// Add MstbCoefficients for the new kind:
impl MissionKind {
    fn mstb_coefficients(&self) -> (f64, f64, f64, f64, f64) {
        match self {
            MissionKind::ThrawnCampaign => (0.02, 0.5, 30.0, 5.0, 95.0),
            // ...
        }
    }
}
```

---

## 2. Mission Lifecycle

### Core Notification Functions

Every mission fires a standardized sequence of notifications as it moves through its lifecycle. These are all small (51-65 byte) functions that call `FUN_0053a010` (validity check) and then `FUN_004f8880` (dispatch notification) + optionally `FUN_0053fdd0` (register event with ID).

The canonical notification dispatch signatures:
```c
// One-entity notification (most mission lifecycle notifs):
FUN_004f8880(this, notif_string, event_name, param_1, null, param_2)

// Two-entity notification (espionage extra system, troop destruction):
FUN_004f8aa0(this, notif_string, event_name, entity_2, entity_1, null, param_3)
```

### Mission Property Notifications

These fire when mission properties are assigned or change:

| Function | Notification String | Event Name | Payload | Notes |
|----------|--------------------|---------|---------|----|
| FUN_00524fc0 | MissionUserIDNotif | UserID | — | Mission instance identity |
| FUN_00525000 | MissionUserID2Notif | UserID2 | — | Secondary ID (multiplayer?) |
| FUN_005250a0 | MissionOriginLocationKeyNotif | OriginLocation | system key | Where mission starts |
| FUN_005250e0 | MissionObjectiveKeyNotif | Objective | entity key | What is being targeted |
| FUN_00525120 | MissionTargetKeyNotif | Target (parent) | entity key | Primary target |
| FUN_00525160 | MissionTargetLocationKeyNotif | TargetLocation | system key | Target system |
| FUN_005251a0 | MissionLeaderKeyNotif | Leader | char key | Assigned leader |
| FUN_005251e0 | MissionLeaderSeedKeyNotif | Leader | seed key | Leader seed |
| FUN_005252xx | MissionTeamSelectListNotif | — | list | Team member selection list |
| FUN_005253xx | MissionDecoySelectListNotif | — | list | Decoy selection list |
| FUN_005254xx | MissionCapturedSelectListNotif | — | list | Captured characters list |
| FUN_005255xx | MissionMemberFinishedMissionSelectListNotif | — | list | Members who completed |
| FUN_00525560 | MissionReadyForNextPhaseNotif | ReadyForNextPhase | — | Phase gate |
| FUN_005255a0 | MissionImpliedTeamNotif | ImpliedTeam | — | Auto-assigned team |
| FUN_005255e0 | MissionMandatoryNotif | Mandatory | — | Marks mission as mandatory |

### Character Role State Notifications

These fire on the character object (Role) when its mission state changes:

| Function | Notification String | Event Name | Meaning |
|----------|--------------------|---------|----|
| FUN_00536800 | RoleMissionKeyNotif | Mission | Character assigned to mission |
| FUN_00536880 | RoleMissionSeedKeyNotif | MissionSeed | Seed assignment |
| FUN_005368c0 | RoleParentAtMissionCompletionKeyNotif | ParentAtMissionCompletion | Where char returns to |
| FUN_00536900 | RoleLocationAtMissionCompletionKeyNotif | LocationAtMissionCompletion | Which system after mission |
| FUN_00536980 | RoleMovingBetweenMissionsNotif | MovingBetweenMissions | In transit between assignments |
| FUN_005369c0 | RoleMissionRemoveRequestNotif | MissionRemoveRequest | Removal from mission requested |
| FUN_00536a00 | RoleMissionResignRequestNotif | MissionResignRequest | Character wants to resign |
| FUN_00536a40 | RoleCanResignFromMissionNotif | CanResignFromMission | Resignation is permitted |
| FUN_00536b00 | RoleOnMissionNotif | OnMission | Character is on an active mission |
| FUN_00536b40 | RoleOnHiddenMissionNotif | OnHiddenMission | Mission is hidden from opponent |
| FUN_00536b80 | RoleOnMandatoryMissionNotif | OnMandatoryMission | Character cannot be reassigned |

### OnMission / OnHiddenMission / OnMandatoryMission States

Three orthogonal states, each tracked via a separate notification channel:

- **OnMission**: fired when a character's `RoleMissionKeyNotif` is set to a non-null mission. Cleared when the mission completes or character is removed. Standard assignments (diplomacy, recruitment, espionage) fire this.

- **OnHiddenMission**: fired for covert operations where the mission should not be visible to the opposing faction's intel. Assassination, abduction, and rescue missions use this path. The game cross-references `OnHiddenMission` during counter-intelligence checks (FOILTB.DAT).

- **OnMandatoryMission**: fired after `MissionMandatoryNotif`. Blocks `CanResignFromMission` — the character is locked into the assignment until completion or death. Story event characters (Luke, Han, Leia during the Palace sequence; Darth Vader during the Final Battle chain) receive mandatory status.

### ReadyForNextPhase

`MissionReadyForNextPhaseNotif` (FUN_00525560) is the phase gate primitive. Multi-phase missions (espionage sequences, the Dagobah training arc, the Final Battle chain) use repeated `ReadyForNextPhase` notifications to advance through scripted stages. When the mission manager receives this notification, it evaluates the next phase's conditions.

### MissionRemoveRequest / MissionResignRequest / CanResignFromMission

Three-step resignation protocol:
1. `MissionResignRequest` — character or player requests resignation
2. `CanResignFromMission` — system evaluates eligibility (blocked if `OnMandatoryMission`)
3. `MissionRemoveRequest` — actual removal authorized and executed

Modders implementing a new mandatory event chain must ensure their event characters receive `OnMandatoryMission` before any resign requests can be generated.

### Espionage Extra System

`FUN_005738f0` fires `MissionEspionageExtraSystemKeyNotif` with a secondary system key, enabling espionage missions to have a distinct secondary target location. Event ID `0x370` (880). This is the highest confirmed event ID in the system. New event IDs for custom events should start above 0x370.

---

## 3. Scripted Event Sequences

All story events share the same structural pattern:
```c
void __thiscall EventHandler(void *this, int param_1, int param_2) {
    bool valid = FUN_0053a010((int)this);   // validity check
    if (valid) {
        FUN_004f8880(this,                  // dispatch notification
            s_MissionMgr*Notif*,            // notification type string
            s_EventName*,                   // event name string
            param_1, null, param_2);
        FUN_0053fdd0(event_id,              // register event with ID (only on full-fire events)
            this, param_1, param_2);
    }
}
```

The `FUN_0053fdd0` call appears only on the "terminal" notification in each chain (the one that triggers the final scripted outcome). Intermediate gate notifications (like `LukeDagobahRequired`, `DarthPickup`, `FinalBattleReady`) do not call `FUN_0053fdd0`.

### Event ID Sequence

Event IDs are assigned in causal order — earlier events have lower IDs:

| Event ID (hex) | Event ID (dec) | Notification / Event Name | Function |
|----------------|----------------|--------------------------|----------|
| 0x12c | 300 | RecruitmentDone | FUN_00532f40 |
| 0x1e1 | 481 | CharacterForceNotif / Force | — |
| 0x1e5 | 485 | CharacterForceTrainingNotif / ForceTraining | — |
| 0x210 | 528 | DagobahCompleted / LukeDagobahCompleted | FUN_0056fc70 |
| 0x212 | 530 | BountyAttack / HanBountyAttack | FUN_00572b40 |
| 0x220 | 544 | FinalBattle | FUN_0054ba00 |
| 0x221 | 545 | LukeDagobah | FUN_0054b7b0 |
| 0x370 | 880 | MissionEspionageExtraSystem | FUN_005738f0 |

The gap between 0x221 and 0x370 is large, leaving extensive room for custom event IDs.

---

### 3.1 Luke Dagobah Training Chain

**Alliance-only story arc.** Triggers when Luke Skywalker achieves sufficient Force Experience.

```
[Precondition: Luke's Force Experience meets threshold]
│
├── FUN_0054b770  →  MissionMgrLukeDagobahRequiredNotif / LukeDagobahRequired
│   └── Gate notification. No event ID registered. Sets the Dagobah mission as required.
│       FUN_0053a010 validity check only — no FUN_0053fdd0.
│
├── [Luke is dispatched to Dagobah system]
│
├── FUN_00575320  →  DagobahMissionFirstTrainingDayNotif / FirstTrainingDay
│   └── Fires when Luke arrives at Dagobah. Marks training start day.
│       No event ID (gate notification).
│
├── FUN_00571590  →  MissionJediTrainingTeacherKeyNotif / Teacher
│   └── Assigns Yoda as the training teacher key. Fires when teacher is confirmed present.
│
├── [N training ticks pass — duration controlled by GNPRTB parameters]
│
├── FUN_0054b7b0  →  MissionMgrLukeDagobahNotif / LukeDagobah
│   └── Full-fire event. Registers event 0x221.
│       FUN_0053fdd0(0x221, this, param_1, param_2)
│       Triggers Luke's Force skill upgrades.
│
└── FUN_0056fc70  →  LukeDagobahCompletedNotif / DagobahCompleted
    └── Completion event. Registers event 0x210.
        FUN_0053fdd0(0x210, ...) — lower ID than 0x221, fires after.
        Luke returns to active duty. Dagobah mission slot freed.
```

**Key data**: Force detection system (`FUN_0058a3f0`) uses a 2-bit Force potential tier from `character[9] >> 6 & 3` and Force experience from `character[0x23]` (short). The Dagobah mission gates on Force potential tier > 0.

---

### 3.2 Darth Vader / Luke Final Battle Chain

**Triggers when Luke completes Dagobah training and the Empire controls sufficient systems.**

```
FUN_0054b900  →  MissionMgrDarthPickupNotif / DarthPickup
│  Gate: Darth Vader dispatched. No event ID.
│  Vader is placed in OnMandatoryMission state.
│
FUN_0054b940  →  MissionMgrDarthToLukeFinalBattleNotif / DarthToLukeFinalBattle
│  Gate: Vader is routed toward Luke's location. No event ID.
│  Fires when Vader's mission target is confirmed as Luke's system.
│
FUN_0054b980  →  MissionMgrDarthToEmperorFinalBattleNotif / DarthToEmperorFinalBattle
│  Gate: Secondary routing — Vader to Emperor first. No event ID.
│  The Emperor path appears to be an alternate branch (Vader retreats to Emperor
│  before confronting Luke directly).
│
FUN_0054b9c0  →  MissionMgrFinalBattleReadyNotif / FinalBattleReady
│  Gate: All prerequisites met. No event ID.
│  Luke and Vader are both in position.
│
FUN_0056fc30  →  LukeFinalBattleReadyNotif / FinalBattleReady
│  Luke's side fires its own FinalBattleReady — confirms both sides ready.
│  Note: this is a distinct notification from the MissionMgr version above.
│  Same event name, different notif string, different function address.
│
FUN_0054ba00  →  MissionMgrFinalBattleNotif / FinalBattle
   Full-fire event. Registers event 0x220.
   FUN_0053fdd0(0x220, this, param_1, param_2)
   Triggers the final battle sequence. Both characters go mandatory.
   Victory condition checked: if Alliance wins, game ends.
```

**Two FinalBattleReady notifications**: the split between `MissionMgrFinalBattleReadyNotif` (FUN_0054b9c0) and `LukeFinalBattleReadyNotif` (FUN_0056fc30) corresponds to the two sides confirming readiness independently. The actual FinalBattle (0x220) fires only after both confirm.

---

### 3.3 Jabba's Palace Chain

**Triggers when the Alliance sends characters to Tatooine (Jabba's Palace system).** Four parallel capture events, each firing independently when the respective character arrives:

```
FUN_0054b800  →  MissionMgrLukePalaceNotif / LukePalace
   Gate: Luke enters Jabba's Palace mission context.
   Returns CONCAT31(extraout_var, bVar1) — bidirectional success flag.
   No event ID registered (no FUN_0053fdd0 call).

FUN_0054b840  →  MissionMgrHanCapturedAtPalaceNotif / HanCapturedAtPalace
   Gate: Han Solo is captured at the Palace.
   Same pattern — no event ID. Sets Han to OnMandatoryMission.

FUN_0054b880  →  MissionMgrLeiaPalaceNotif / LeiaPalace
   Gate: Leia enters Palace context.
   No event ID.

FUN_0054b8c0  →  MissionMgrChewbaccaPalaceNotif / ChewbaccaPalace
   Gate: Chewbacca enters Palace context.
   No event ID.
```

**Design note**: none of the four Palace events register an event ID. The Palace arc is a trigger-only chain — the outcomes (rescue missions, character availability changes) are handled downstream by the rescue mission system (RESCMSTB.DAT) rather than by chained event IDs.

---

### 3.4 Bounty Hunter Chain

**Empire-side event. Activates when the Empire has sufficient credits and Han Solo is in play.**

```
FUN_0054ba50  →  MissionMgrBountyHuntersActiveNotif / BountyHuntersActive
│  Gate: Bounty Hunters become available. No event ID.
│  Sets internal flag enabling the bounty attack mission path.
│
FUN_00572b40  →  HanBountyAttackNotif / BountyAttack
│  Full-fire event. Registers event 0x212.
│  FUN_0053fdd0(0x212, this, param_1, param_2)
│  Fires when a Bounty Hunter successfully attacks Han Solo.
│  Han is targeted regardless of his current mission state.
│
FUN_00572b90  →  HanCapturedByBountyHuntersNotif / CapturedByBountyHunters
   Gate: Han has been captured. No event ID.
   Han enters OnMandatoryMission (in carbonite — cannot be reassigned).
   Triggers rescue mission availability for the Alliance.
```

**Event ID 0x212 ordering**: BountyAttack (0x212) fires before DagobahCompleted (0x210) in the ID space but after it temporally — the ID assignment reflects source code ordering, not strict causal order.

---

## 4. How to Create New Story Events (Modder Guide)

### 4.1 The Notification Pattern

Every story event in REBEXE.EXE is a 51-65 byte function following this exact template:

```c
// "Gate" notification — no event ID (intermediate chain step)
int __thiscall FireGateNotification(void *this, int param_1, int param_2) {
    bool valid = FUN_0053a010((int)this);           // validity check
    if (CONCAT31(extraout_var, valid) != 0) {
        FUN_004f8880(this,
            s_MissionMgr{EventName}Notif_ADDRESS,   // notif type (string from .exe data)
            s_{EventName}_ADDRESS,                   // event key name
            param_1, (char*)0x0, param_2);           // entity refs + context
    }
    return CONCAT31(extraout_var, valid);           // return validity
}

// "Full-fire" notification — registers event ID (terminal chain step)
void __thiscall FireFullEvent(void *this, int param_1, int param_2) {
    bool valid = FUN_0053a010((int)this);
    if (CONCAT31(extraout_var, valid) != 0) {
        FUN_004f8880(this,
            s_MissionMgr{EventName}Notif_ADDRESS,
            s_{EventName}_ADDRESS,
            param_1, (char*)0x0, param_2);
        FUN_0053fdd0(0xNNN,                          // event ID (> 0x370 for new events)
            this, param_1, param_2);
    }
    // void return — no success flag
}
```

### 4.2 Required Notifications for a New Event Chain

Every complete event chain needs at minimum:

1. **One gate notification** per prerequisite condition (the "Required" / "Active" / "Pickup" step)
2. **One full-fire notification** with a unique event ID (the terminal step that locks in the story beat)
3. **One completion notification** if the event resolves over time (the "Completed" / "Done" step)

For a capture/rescue sub-arc (Jabba pattern), add:
4. **One capture notification per capturable character** (the "CapturedAt*" / "*Palace" step)

### 4.3 How to Hook Into the Mission Manager

The mission manager object (`CMissionMgr`) holds all active notification handlers. The `this` pointer passed to each notification function is the `CMissionMgr` instance. The validity check `FUN_0053a010` confirms the object is initialized and in a valid game state.

**Registration pattern**: to make the mission manager respond to a new event, the new notification function must be called from the mission manager's update loop or from whatever condition triggers the chain. In REBEXE.EXE, this is wired at compile time in `missionmgr.cpp`. For a binary mod, you would:

1. Write your event function using the template above
2. Find the call site in the mission manager update that scans for the prerequisite condition
3. Add a call to your new function at the appropriate evaluation point

For open-rebellion's Rust implementation, hook into `EventSystem::advance()` in `rebellion-core/src/events.rs`:

```rust
// In EventState initialization:
let thrawn_chain_start = GameEvent {
    id: EventId(0x400),  // new ID above 0x370
    name: "ThrawnAppears".to_string(),
    conditions: vec![
        EventCondition::TickAtLeast(tick_number),
        EventCondition::EventFired(EventId(0x210)),  // after Dagobah
    ],
    actions: vec![
        EventAction::DisplayMessage("Grand Admiral Thrawn has entered the Outer Rim.".to_string()),
        EventAction::RelocateCharacter { character: thrawn_id, system: wayland_id },
    ],
    is_repeatable: false,
    enabled: true,
};
```

### 4.4 Event ID Assignment

Assign new event IDs sequentially above the current maximum of `0x370` (880):

| Range | Assignment |
|-------|-----------|
| 0x000–0x12b | System/combat events |
| 0x12c–0x1ff | Recruitment and character state events |
| 0x200–0x22f | Story event chain (Dagobah, Final Battle, Bounty) |
| 0x300–0x37f | Espionage/side-effect events |
| 0x380–0x3ff | **Available for mods** |
| 0x400+ | **Custom event chains** |

### 4.5 Example: "Thrawn Campaign" Event Chain

A complete New Republic-era campaign chain, showing the pattern for a non-OT total conversion:

**Design**: After the Alliance wins, Grand Admiral Thrawn emerges from the Unknown Regions. A five-beat chain: Thrawn Appears → Thrawn Strikes → Noghri Assassins Active → Thrawn Final Offensive → Thrawn Defeated or Pellaeon Surrenders.

**Step 1: Define notification names**

```
MissionMgrThrawnAppearsNotif / ThrawnAppears       (gate, no event ID)
MissionMgrThrawnStrikesNotif / ThrawnStrikes        (full-fire, event ID 0x401)
MissionMgrNoghriActiveNotif / NoghriActive          (gate, no event ID)
MissionMgrThrawnOffensiveNotif / ThrawnOffensive    (full-fire, event ID 0x402)
MissionMgrThrawnDefeatedNotif / ThrawnDefeated      (full-fire, event ID 0x403)
```

**Step 2: Implement each function (REBEXE patch)**

Each is 51-65 bytes following the template in §4.1. `ThrawnStrikes`, `ThrawnOffensive`, and `ThrawnDefeated` include the `FUN_0053fdd0` call with their respective event IDs.

**Step 3: Wire prerequisites**

- `ThrawnAppears` fires when `VictoryConditions` notification has been received (game-end was nearly reached by Alliance) and a tick threshold has passed
- `ThrawnStrikes` fires when Thrawn character reaches a Core World system
- `NoghriActive` fires when `ThrawnStrikes` (0x401) is registered and the Noghri are available via their MISSNSD entry
- `ThrawnOffensive` fires when Thrawn has controlled 10+ systems
- `ThrawnDefeated` fires when Thrawn's fleet is destroyed

**Step 4: Set mandatory mission flags**

Thrawn, like Vader in the Final Battle chain, must be placed in `OnMandatoryMission` when `ThrawnOffensive` fires. Fire `MissionMandatoryNotif` on his Role object immediately after `MissionMgrThrawnOffensiveNotif`.

**Step 5: open-rebellion EventState equivalent**

```rust
// In rebellion-core/src/events.rs — add to EventState initialization:

// Beat 1: Thrawn Appears (gate — fires silently, no message)
GameEvent {
    id: EventId(0x400),
    conditions: vec![
        EventCondition::TickAtLeast(alliance_near_victory_tick),
    ],
    actions: vec![
        EventAction::RelocateCharacter { character: thrawn_key, system: wayland_key },
    ],
    is_repeatable: false,
    enabled: true,
}

// Beat 2: Thrawn Strikes (full-fire with player notification)
GameEvent {
    id: EventId(0x401),
    conditions: vec![
        EventCondition::EventFired(EventId(0x400)),
        EventCondition::CharacterAtSystem { character: thrawn_key, system: coruscant_region_key },
    ],
    actions: vec![
        EventAction::DisplayMessage("Grand Admiral Thrawn has launched an offensive against the New Republic.".to_string()),
        EventAction::ModifyCharacterSkill { character: thrawn_key, skill: SkillKind::Combat, delta: 10 },
    ],
    is_repeatable: false,
    enabled: true,
}

// Beat 3: Noghri Assassins Active (gate)
GameEvent {
    id: EventId(0x402),
    conditions: vec![
        EventCondition::EventFired(EventId(0x401)),
    ],
    actions: vec![
        EventAction::DisplayMessage("Noghri death commandos have been activated.".to_string()),
        // Enable Noghri special force eligibility in mission system
    ],
    is_repeatable: false,
    enabled: true,
}
```

### 4.6 Disabling OT Events for a Total Conversion

To remove the Luke/Vader/Palace/Bounty chains entirely:

**REBEXE patch**: replace the body of each `MissionMgr*Notif` function (FUN_0054b770 through FUN_0054ba50) with a `ret` instruction (0xC3). The validity check at the top will never be reached, so the chains never fire.

**open-rebellion**: set `enabled: false` on the corresponding `GameEvent` entries in `EventState`:

```rust
// Disable all OT story events:
event_state.set_enabled(EventId(0x210), false);  // DagobahCompleted
event_state.set_enabled(EventId(0x212), false);  // BountyAttack
event_state.set_enabled(EventId(0x220), false);  // FinalBattle
event_state.set_enabled(EventId(0x221), false);  // LukeDagobah
```

The `is_repeatable: false` flag combined with `enabled: false` ensures the events never auto-re-enable.

---

## 5. Quick Reference: All Mission & Event Functions

### Mission Dispatch

| Address | Size | Function |
|---------|------|---------|
| FUN_0050d5a0 | 3050 | Mission dispatch — 13-case switch on type, entity family filter setup, team candidate loop |
| FUN_004fc080 | 439 | Mission destruction outcomes (DestroyedSabotage, DestroyedAssassination, DestroyedAutoscrap) |
| FUN_00536800 | 120 | RoleMissionKeyNotif / Mission |

### Mission Lifecycle

| Address | Size | Notif String | Event Name |
|---------|------|-------------|-----------|
| FUN_00524fc0 | 56 | MissionUserIDNotif | UserID |
| FUN_00525000 | 56 | MissionUserID2Notif | UserID2 |
| FUN_005250a0 | 56 | MissionOriginLocationKeyNotif | OriginLocation |
| FUN_005250e0 | 56 | MissionObjectiveKeyNotif | Objective |
| FUN_00525120 | 56 | MissionTargetKeyNotif | Target (parent) |
| FUN_00525160 | 56 | MissionTargetLocationKeyNotif | TargetLocation |
| FUN_005251a0 | 56 | MissionLeaderKeyNotif | Leader |
| FUN_005251e0 | 56 | MissionLeaderSeedKeyNotif | Leader (seed) |
| FUN_00525560 | 51 | MissionReadyForNextPhaseNotif | ReadyForNextPhase |
| FUN_005255a0 | 51 | MissionImpliedTeamNotif | ImpliedTeam |
| FUN_005255e0 | 51 | MissionMandatoryNotif | Mandatory |

### Character Role State

| Address | Size | Notif String | Event Name |
|---------|------|-------------|-----------|
| FUN_00536980 | 51 | RoleMovingBetweenMissionsNotif | MovingBetweenMissions |
| FUN_005369c0 | 51 | RoleMissionRemoveRequestNotif | MissionRemoveRequest |
| FUN_00536a00 | 51 | RoleMissionResignRequestNotif | MissionResignRequest |
| FUN_00536a40 | 51 | RoleCanResignFromMissionNotif | CanResignFromMission |
| FUN_00536b00 | 51 | RoleOnMissionNotif | OnMission |
| FUN_00536b40 | 51 | RoleOnHiddenMissionNotif | OnHiddenMission |
| FUN_00536b80 | 51 | RoleOnMandatoryMissionNotif | OnMandatoryMission |
| FUN_005368c0 | 56 | RoleParentAtMissionCompletionKeyNotif | ParentAtMissionCompletion |
| FUN_00536900 | 56 | RoleLocationAtMissionCompletionKeyNotif | LocationAtMissionCompletion |

### Dagobah / Jedi Training

| Address | Size | Notif String | Event ID |
|---------|------|-------------|---------|
| FUN_0054b770 | 51 | MissionMgrLukeDagobahRequiredNotif / LukeDagobahRequired | — (gate) |
| FUN_0054b7b0 | 65 | MissionMgrLukeDagobahNotif / LukeDagobah | 0x221 |
| FUN_00575320 | 56 | DagobahMissionFirstTrainingDayNotif / FirstTrainingDay | — (gate) |
| FUN_00571590 | 56 | MissionJediTrainingTeacherKeyNotif / Teacher | — (gate) |
| FUN_0056fc70 | — | LukeDagobahCompletedNotif / DagobahCompleted | 0x210 |

### Final Battle Chain

| Address | Size | Notif String | Event ID |
|---------|------|-------------|---------|
| FUN_0054b900 | 51 | MissionMgrDarthPickupNotif / DarthPickup | — (gate) |
| FUN_0054b940 | 51 | MissionMgrDarthToLukeFinalBattleNotif / DarthToLukeFinalBattle | — (gate) |
| FUN_0054b980 | 51 | MissionMgrDarthToEmperorFinalBattleNotif / DarthToEmperorFinalBattle | — (gate) |
| FUN_0054b9c0 | 51 | MissionMgrFinalBattleReadyNotif / FinalBattleReady | — (gate) |
| FUN_0056fc30 | 51 | LukeFinalBattleReadyNotif / FinalBattleReady | — (gate) |
| FUN_0054ba00 | 65 | MissionMgrFinalBattleNotif / FinalBattle | 0x220 |

### Jabba's Palace

| Address | Size | Notif String | Event ID |
|---------|------|-------------|---------|
| FUN_0054b800 | 51 | MissionMgrLukePalaceNotif / LukePalace | — (gate) |
| FUN_0054b840 | 51 | MissionMgrHanCapturedAtPalaceNotif / HanCapturedAtPalace | — (gate) |
| FUN_0054b880 | 51 | MissionMgrLeiaPalaceNotif / LeiaPalace | — (gate) |
| FUN_0054b8c0 | 51 | MissionMgrChewbaccaPalaceNotif / ChewbaccaPalace | — (gate) |

### Bounty Hunters

| Address | Size | Notif String | Event ID |
|---------|------|-------------|---------|
| FUN_0054ba50 | 51 | MissionMgrBountyHuntersActiveNotif / BountyHuntersActive | — (gate) |
| FUN_00572b40 | 65 | HanBountyAttackNotif / BountyAttack | 0x212 |
| FUN_00572b90 | 51 | HanCapturedByBountyHuntersNotif / CapturedByBountyHunters | — (gate) |

### Espionage / Force

| Address | Size | Notif String | Event ID |
|---------|------|-------------|---------|
| FUN_005738f0 | 73 | MissionEspionageExtraSystemKeyNotif / ExtraSystem | 0x370 |
| FUN_0058a3f0 | — | Force user detection logic | — |
| FUN_004f1e00 | — | CharacterForceNotif / Force | 0x1e1 |
| FUN_004f1ea0 | — | CharacterForceTrainingNotif / ForceTraining | 0x1e5 |
| FUN_004ee470 | 129 | MissionHyperdriveModifierNotif / MissionHyperdriveModifier | — |
| FUN_00532f40 | 65 | SideRecruitmentDoneNotif / RecruitmentDone | 0x12c |

---

## 6. DAT Files That Control Mission Behavior

| File | What It Controls | How to Mod |
|------|-----------------|-----------|
| MISSNSD.DAT | 25 mission definitions: duration, max officers, special force eligibility, target flags | Edit entries 1-25; add entry 26 for new mission type |
| DIPLMSTB.DAT | Diplomacy success probability curve | Change threshold/value pairs to shift difficulty |
| ESPIMSTB.DAT | Espionage success probability | Same |
| ASSNMSTB.DAT | Assassination success probability | Same |
| INCTMSTB.DAT | Incite uprising success probability | Same |
| DSSBMSTB.DAT | Death Star sabotage success probability | Same |
| ABDCMSTB.DAT | Abduction success probability | Same |
| RCRTMSTB.DAT | Recruitment success probability | Same |
| RESCMSTB.DAT | Rescue success probability | Same |
| SBTGMSTB.DAT | Sabotage success probability | Same |
| SUBDMSTB.DAT | Subdue uprising success probability | Same |
| FOILTB.DAT | Counter-intel foil probability | Lower all values to make espionage harder to counter |
| ESCAPETB.DAT | Character escape after capture | Higher values = characters escape more easily |
| FDECOYTB.DAT | Fleet decoy success | Affects Rescue and some covert missions |
| TDECOYTB.DAT | Troop decoy success | Affects missions with troop decoys |
| SPECFCSD.DAT | Special forces: mission eligibility bitmasks | Set `mission_id` bitmask to match new mission type codes |

**GNPRTB parameters likely controlling missions** (not yet fully confirmed — RE target):

- Parameters 0x0a00–0x0a21 include mission duration multipliers, success probability bonuses per difficulty, and reward magnitude scalars. Edit the Alliance Easy/Med/Hard and Empire Easy/Med/Hard columns independently to create asymmetric difficulty curves.

---

*For combat formulas (bombardment, space combat 7-phase pipeline, ground combat), see `combat-formulas.md` and `annotated-functions.md`. For the full function taxonomy, see `modders-taxonomy.md`.*
