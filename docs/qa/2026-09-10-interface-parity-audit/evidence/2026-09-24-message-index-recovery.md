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

The path that clears `0x40` when a new message arrives is **not yet
recovered**. The subagent's first report attributed it to `FUN_00487eb0`, but
that is the game-speed setter (see the
[Game Speed recovery](2026-09-24-game-speed-recovery.md)). Next targets:
`FUN_0048aa90`, `FUN_00488030`, and the toggle-button state
(`FUN_00603aa0`).

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
