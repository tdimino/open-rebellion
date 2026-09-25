# Message Index recovery

Status: static recovery for `CMD-08`, not acceptance. No cell is promoted.

This traces the Display Message Index window, the category mapping, and how the
Message Notification rail returns to its resting state. It builds on the
[P49 rail evidence](2026-09-12-message-index-rail.md). The decompilations fill
empty `ghidra/notes/` placeholders.

## Window

`FUN_0042a240(owner, mode, category)` opens one `0xd`-type window at a time
(`FUN_00604500`), only while the game state from `FUN_004fcee0` is greater
than 1. The rail commands `0x136..0x13e`, F6 (`0x75` → category `0x79`), and
message `0xd` all route here.

| Element | Recovered contract |
|---|---|
| Window | `FUN_00466350`: 470x331 (`0x1d6 x 0x14b`); vtable `0x0065a1c0`. Layout is in slot 14 (`FUN_004665f0`) and close in slot 12 (`FUN_0046a6a0`). |
| Art | Common background `0x2a46`. Faction frame: Alliance `0x285f`, Empire `0x2860`. Inner panel: Alliance `0x2a44`, Empire `0x2a45`. |
| Category tabs | Ten buttons at x = `0, 0x26, 0x4c, 0x72, 0x97, 0xbd, 0xe3, 0x107, 0x12d, 0x152`, carrying commands `0x79..0x82`. |
| List | Rect `(25,108)`, size `368x194`, four columns (`FUN_004ad620(..., 4)`), filled by `FUN_004697b0(category)`. |
| Text area | Rect `(17,234)`, size `395x80`, used for message detail and chat compose. |
| Modes | `FUN_00468fb0`: 1 = index ("Message Index", TEXTSTRA `0x8019`), 2 = one message, 3 = compose chat. |
| Commands | `0x28` Index, `0x65` Display Message, `0x66` Post Messages Silently, `0x67` Display, `0x68` Compose Chat (disabled in single player), `0x90` navigate, `0x91` Delete Selected Messages, `0x96`/`0x9a` scroll, `0x97` display toggle, `0x98` Encyclopedia, `0x99` detail. |

## Categories

| Category | Manual name | Tab label (TEXTSTRA) | Alert / read bit | Rail command |
|---|---|---|---|---|
| `0x79` | All | `0x8010` All Messages | — | F6 |
| `0x7a` | Loyalty | `0x8011` Popular Support Messages | `0x001` | `0x136` |
| `0x7b` | Fleet | `0x8017` Fleet Messages | `0x080` | `0x137` |
| `0x7c` | Mission | `0x8014` Mission Messages | `0x010` | `0x138` |
| `0x7d` | Resource | `0x8012` Resource Messages | `0x004` | `0x139` |
| `0x7e` | Manufacturing | `0x8013` Manufacturing Messages | `0x008` | `0x13a` |
| `0x7f` | Defense | `0x8016` Defense Messages | `0x040` | `0x13b` |
| `0x80` | Conflict | `0x8018` Conflict Messages | `0x100` | `0x13c` |
| `0x82` | Advice | `0x8032` Advice Messages | `0x200` | `0x13d` |
| `0x81` | Chat | `0x8015` Chat Messages | `0x020` | `0x13e` |

The rail order places Advice above Chat, so the router maps `0x13d` to `0x82`
and `0x13e` to `0x81`.

## Rail resting and illumination

- **Construction.** `FUN_00427270` sets control flag `0x40` and loads the
  resting bitmap into state slot 2.
- **Return to resting.** Closing the window (slot 12) marks the category's
  messages read: `FUN_0048a530` clears node bit `0x10`, then
  `FUN_0041db40` → `FUN_0042da10` re-sets flag `0x40` on each control whose
  bit is in the mask. `FUN_0042da10` directly verifies the nine-bit mask to
  control-offset map.
- **Opening the window does not clear illumination.**

## Illumination on arrival

`FUN_0042d8d0(mask)` mirrors the resting routine: for each set category bit it
clears flag `0x40` (`FUN_006030f0`) on the matching rail control, which paints
the illuminated resource. `FUN_0041db20` reaches it through the command-center
singleton (`FUN_00422ca0`). The three callers define the rule:

| Caller | Rule |
|---|---|
| `FUN_0048a060` (post) | Message kinds 3, 4, 5, and 8 join the list at `+0x10` when their payload exists (`FUN_00583c40`); otherwise the message is destroyed. The class mask at `+0x34` is ORed into the unread mask at `+0x50` and that rail lights. When the category's alert setting (`FUN_0048a1c0`, per-category entry at `+0x44`, else the default at `+0xc`) lacks bit 4 and the message has a display target at `+0x28`, the owner's vtable `+0x14` presents it. The list is then rebuilt. Kinds 1 and 2 go to `+0x1c`, kind 6 to `+0x28`, and kind 7 is posted straight to the window. |
| `FUN_0048a2a0` (rebuild) | Unless batching is on (`+0x8` bit `0x10000000`, set and cleared by `FUN_0048a610`), drops expired messages and recomputes the unread mask from every message whose unread bit `0x10` at `+0x24` is set. Rails outside the mask rest (`FUN_0041db40`); rails in it light. |
| `FUN_00488a30` (load) | Restores the saved unread mask at `+0x50` and lights it. |

The earlier guess `FUN_00487eb0` is the game-speed setter, and
`FUN_00488030` chooses a notification sound, not a rail state.

## Message classes and categories

Every message class constructor calls `FUN_004c4b50` and stores a constant
category mask at `+0x34` from a one-line getter:

| Mask | Getter | Rail category | Constructors |
|---|---|---|---|
| `0x001` | `FUN_0040f340` | Popular Support | `FUN_00490ff0`, `FUN_00498c30`, `FUN_004990b0`, `FUN_004993e0`, `FUN_004996f0` |
| `0x004` | `FUN_004047d0` | Resource | `FUN_0048e720` |
| `0x008` | `FUN_0048c2a0` | Manufacturing | `FUN_0048ab40`, `FUN_0048bd10`, `FUN_004975e0`, `FUN_00497890`, `FUN_004988f0`, `FUN_00499a30` |
| `0x010` | `FUN_0048af30` | Mission | 15 constructors from `FUN_0048adc0` to `FUN_00495460` |
| `0x020` | `FUN_00526bd0` | Chat | `FUN_00498e90` (incoming chat, event `0x11`) |
| `0x080` | `FUN_00526670` | Fleet | `FUN_00498140`, `FUN_00499d50` |
| `0x100` | `FUN_0048bc60` | Conflict | `FUN_0048b8e0`, `FUN_00495dd0`, `FUN_00496080`, `FUN_00496460` |
| `0x200` | `FUN_0048b450` | Advice | `FUN_0048b2e0` (TEXTSTRA `0x6001 + 3n` Alliance, `0x6802 + 3n` Empire) |

No constructor stores Defense (`0x040`), so no message lights that rail.

## Notification types

The factory `FUN_00489740(type, …)` builds one class per notification type;
each class's vtable slot 3 returns its type. TEXTSTRA names every type at
`0x5000 + type` (these are also the Notification Options labels). Types 1
and 2 (Tactical After Action Report, Tactical Pre Battle Message) build
battle reports held apart at `+0x1c`. Advice (`FUN_0048b2e0`) and incoming
chat (`FUN_00498e90`) are built outside the factory.

| Type | TEXTSTRA name (`0x5000 + type`) | Class | Mask | Rail |
|---|---|---|---|---|
| `3` | Uprising Message | `FUN_004993e0` | `0x001` | Popular Support |
| `4` | System Control Message | `FUN_004996f0` | `0x001` | Popular Support |
| `5` | Research Report | `FUN_00499a30` | `0x008` | Manufacturing |
| `6` | Repair Message | `FUN_00499d50` | `0x080` | Fleet |
| `7` | Blockade Message | `FUN_00496080` | `0x100` | Conflict |
| `9` | Smuggling Message | `FUN_004990b0` | `0x001` | Popular Support |
| `0xb` | Garrison Warning | `FUN_00498c30` | `0x001` | Popular Support |
| `0xc` | Maintenance Shortfall | `FUN_004988f0` | `0x008` | Manufacturing |
| `0xd` | Unit Arrival | `FUN_00498140` | `0x080` | Fleet |
| `0xe` | Unit Deployment | `FUN_00497890` | `0x008` | Manufacturing |
| `0xf` | Operation Reports | `FUN_00496460` | `0x100` | Conflict |
| `0x10` | Deployment Failed | `FUN_004975e0` | `0x008` | Manufacturing |
| `0x13` | Evacuation Losses | `FUN_00495dd0` | `0x100` | Conflict |
| `0x14` | Personnel Arrive | `FUN_00495460` | `0x010` | Mission |
| `0x15, 0x16` | Planet Destroyed / Mission Report | `FUN_00492470` | `0x010` | Mission |
| `0x17` | Mission Failed | `FUN_00491260` | `0x010` | Mission |
| `0x18` | Informant Report | `FUN_00490ff0` | `0x001` | Popular Support |
| `0x19` | Character Captured | `FUN_004902d0` | `0x010` | Mission |
| `0x1a` | Character Health | `FUN_0048fd80` | `0x010` | Mission |
| `0x1b` | Bounty Hunters | `FUN_0048f990` | `0x010` | Mission |
| `0x1c` | Force User Discovered | `FUN_0048f160` | `0x010` | Mission |
| `0x1d` | Force Skill Improvement | `FUN_0048ed00` | `0x010` | Mission |
| `0x1e, 0x1f, 0x20` | Coruscant Captured / Rebel HQ Destroyed / System Resources Messages | `FUN_0048e720` | `0x004` | Resource |
| `0x21` | Construction Complete | `FUN_0048bd10` | `0x008` | Manufacturing |
| `0x22` | Unit Sabotaged | `FUN_0048b8e0` | `0x100` | Conflict |
| `0x23, 0x24` | Death Star Sabotaged / Encounter Messages | `FUN_0048d6f0` | `0x010` | Mission |
| `0x25` | Dagobah Messages | `FUN_0048d070` | `0x010` | Mission |
| `0x26` | Emperor at Coruscant | `FUN_0048ce00` | `0x010` | Mission |
| `0x27` | Traitor Discovered | `FUN_0048c940` | `0x010` | Mission |
| `0x28` | Rescue Attempt | `FUN_0048c3a0` | `0x000` | none stored |
| `0x29` | — | `FUN_0048b660` | `0x010` | Mission |
| `0x2b` | — | `FUN_0048afd0` | `0x010` | Mission |
| `0x2c` | — | `FUN_0048ab40` | `0x008` | Manufacturing |
| `0x2d` | — | `FUN_0048adc0` | `0x010` | Mission |

Type `0x28` (Rescue Attempt) stores no mask; its visitor `FUN_004974f0` only
walks the record, so its rail is whatever `FUN_004c4b50` leaves at `+0x34`.
Types `0x29` and `0x2b`–`0x2d` have no name in the extracted TEXTSTRA table.

## Advice slows the game

`FUN_00487ff0(this, 1)` saves the speed index at `+0x58` and drops the game to
Very Slow (index 1); `FUN_00487ff0(this, 0)` restores the saved index. It
changes the rate only and does not pause.

| Caller | Effect |
|---|---|
| `FUN_004697b0` category `0x82` (Advice) → `FUN_0041d3f0` → `FUN_004369f0` | Showing Advice drops to Very Slow. |
| `FUN_004697b0` every other category → `FUN_0041d410` → `FUN_00436a00` | Showing any other category restores the saved speed. |
| `FUN_0046a6a0` (window close) → `FUN_0041d410` | Closing the Message Index restores the saved speed. |
| `FUN_0045ddc0` → `FUN_0041d3f0`; `FUN_00460070` → `FUN_0041d410` | A second window drops to Very Slow when it is built and restores on close. It is not yet identified. |

The saved index is kept only while `+0x58` is zero, so repeated drops do not
overwrite it.

## Message data and droid menus

- **Manager.** `FUN_005f7cc0` returns the singleton at `DAT_006be3a8`, which
  keeps per-faction lists at `+0x78` and `+0x84`.
- **Node fields.** Each node carries a key at `+0x18`, an object link at
  `+0x1c`, the category nibble at `(+0x20 >> 24) & 0xf`, and unread bit
  `0x10` at `+0x24`.
- **ClearCategory and delete.** ClearCategory (`FUN_005970a0` →
  `FUN_005fa720`) removes a category from both lists. `FUN_005f54a0` deletes
  a single message.
- **Alerts menu.** A right-click on the message droid with list `0xbeef`
  builds the "Message Alerts" toggle menu from STRATEGY `RT_RCDATA`
  `0x171..0x179` (TEXTSTRA `0x3120..0x3128`).
- **Agent menu.** List `0xdead` builds the general Agent menu from `0x110..0x11e`.
