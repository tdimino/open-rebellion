# Developer Commands

Developer commands drive a native build to the state under test, and through
the input under test, without anyone's pointer or focus. One parser and one
queue serve three surfaces:

- the command palette (backtick), for a person at the machine;
- a command script, run from launch (`--commands FILE`);
- the live channel, which drives a game already running (`--live`).

They exist in debug builds and in native release builds with
`OPEN_REBELLION_DEV` set (1, true, yes or on). Release browser builds compile
none of them. The code lives in `crates/rebellion-app/src/`:

| File | Holds |
|---|---|
| `dev_commands.rs` | the dev gate, the script queue, the `DevCommand` grammar |
| `dev_channel.rs` | the live inbox, the results file, `Capture` |
| `dev_input.rs` | scripted clicks, drags and keys |
| `dev_actions.rs` | `Post message`, `Battle at`, `Blockade` |

## The loop

```bash
E=.artifacts/native-checks/<date>-<slug>
scripts/launch-native.sh --build --live --seed 42 --evidence $E
scripts/dev-send.sh $E "Start game: Empire"
scripts/dev-send.sh $E "Skip briefing"
scripts/dev-send.sh $E "Capture: 01-galaxy"
```

Read each capture, then send the next command. To see the whole run at
once, use the report (below). `dev-send.sh` appends the
line to `$E/commands.in`, waits for its result in `$E/commands.out` and
prints it as one JSON line. It exits 0 when the command ran, 1 when it was
refused or skipped, and 2 on a timeout (30 s by default; `--timeout S`). Every
command also logs `[dev-command] sent …` and, when refused,
`[dev-command] refused: <why>` to `$E/game.log`.

Prefer this loop to cua-driver. A capture needs no window id and no Screen
Recording permission, and works while the window is covered. Scripted input
never moves the hardware pointer or brings the game to the front. Keep
cua-driver for what the game cannot do itself (agent-tooling.md, "Native GUI
Acceptance").

## Commands

Names ignore case. Coordinates are the original game's 640×480 logical
screen: the space of every `ghidra/notes` layout and of a Wine capture
scaled to 640×480.

| Command | Runs | Does |
|---|---|---|
| `Capture` / `Capture: <name>` | anywhere | Saves the frame being drawn as `<evidence>/<name>.png` (unnamed: `capture-N`). The name takes letters, digits, `-`, `_` and `.`, and may not start with `.`. The result carries `path` and the pixel size. |
| `Status` | anywhere | Reports `mode`, `day`, `speed`, `pause_alert` (the "Resume Game Play?" alert is up), `battle_alert`, `message_index` (category/view or `closed`), `briefing`, `side`, and each side's top strip as `alliance=`/`empire=` raw/refined/capacity-load. |
| `Wait <n> ms` / `Wait <n> frames` | anywhere | Holds the queue; the result arrives when the wait ends. |
| `Click <x>,<y>` / `Right click <x>,<y>` | anywhere | Move, press and release at the point, one per frame. |
| `Drag <x>,<y> to <x>,<y>` | anywhere | Press, move over 8 frames, release. |
| `Press <key>` | anywhere | Press and release: a letter, a digit, `f1`–`f12`, `escape`, `return`, `space`, `tab`, `delete`, the arrows, `home`, `end`, `pageup`, `pagedown`. |
| `Start game: Alliance` / `Start game: Empire` | main menu | Starts through the menu's faction control. |
| `Skip briefing` | galaxy | The tour's own skip, as Escape: the reply plays (ALBRIEF 10165 / EMBRIEF 11157). Refused when no briefing runs. |
| `Open Message Index: <category> [<row>\|last]` | galaxy | Opens the window on All, Popular Support, Fleet, Mission, Resource, Manufacturing, Defense, Conflict, Chat or Advice. A row (1-based in list order, oldest first) or `last` shows that message in mode 2; the result names it, and the log records its sound (`[message] sound 0x… started volume=…`; 0 under `launch-native.sh`'s mute). Advice drops the speed to Very Slow, as the original does. |
| `Post message: <class> at <system>` | galaxy | Files `uprising began`, `uprising ended`, `blockade`, `fleet arrival`, `loyalty joins` or `loyalty neutral` through the class's builder, to both sides where the original files both. Refused where the original files nothing (a neutral system, no blockading fleet, no fleet). |
| `Battle at <system>` | galaxy | Moves an idle fleet of each side missing there, through `apply_fleet_arrival`. The next tick's combat check opens the Battle Alert, so the game must not be paused. The result names the holder and the fleets moved. |
| `Blockade <system>` | galaxy | Moves an idle fleet of the side not holding the system; the simulation forms the blockade. Refused for a neutral system or where the holder has a fleet. |
| `Advisor: post code <n>` | galaxy | Posts advice code `n` (decimal or `0x` hex, up to `0xff`) to the droids, as a game event does. |
| `Release advice topic <n>` | galaxy | Releases held topic `n` now and files it under Advice. Refused before the opening topics are filed, or for a topic already filed. |
| `List systems` | galaxy | Reports every system as `<name> <holder>/<fleets>`: `A`, `E` or `-`, then the sides with fleets there. Use it to choose a `Battle at` or `Blockade` target. |
| `List advice topics` | galaxy | Reports `held: [n, …]`, the held topics in release order (empty before the opening topics are filed). |
| any palette label | galaxy | `Open sector window: <system>`, `Open <quadrant> window: <system>`, `Open <quadrant> icon menu: <system>`, the speed and time commands. |

"Galaxy" commands wait until the galaxy shows and the previous command's
interface work has finished. "Anywhere" commands run in the main menu,
battles and modal screens too. The palette lists the argument-free ones
(`Capture`, `Skip briefing`, each `Open Message Index` category).

### The Battle Alert's five texts

`Battle at` takes whichever template the holder and blockade produce
(`FUN_0049c0d0`):

| Template | Reach it with |
|---|---|
| `0x7021` your system threatened | `Battle at <your system>` |
| `0x7022` your fleet breaking the enemy's blockade of your system | `Blockade <your system>`, a tick, `Battle at <your system>` |
| `0x7023` entering an enemy system | `Battle at <enemy system>` |
| `0x7024` the enemy breaking your blockade | `Blockade <enemy system>`, a tick, `Battle at <enemy system>` |
| `0x7025` a neutral system | `Battle at <neutral system>` |

## Reading a run

```bash
scripts/dev-report.py $E            # timeline, then a live Status
scripts/dev-report.py $E --all      # with droid traces and bookkeeping
```

The report merges `commands.out` with `game.log`, which it splits at the
`[dev-command] run #N: <line>` marker each command logs as it starts (`#-`
for a script or palette line). Under each command it lists the game's
tagged lines up to the next command: `[message] filed …`,
`[battle_alert] opened`, `[advisor] code …`, `[capture] …` and so on. `!`
marks a refusal or error, and `@` a capture. While the game is still
running, it ends with `now:` and a fresh `Status`.

```text
 #8 Battle at Ghorman  -> done: held by the Alliance; moved Alliance Fleet 1 from Yavin
 #9 Wait 1000 ms  -> done

now: mode=Galaxy day=3 speed=Medium pause_alert=true battle_alert=false …
```

A run that waits forever usually shows here as `pause_alert=true` (the
stop-day pause, which `Click 319,300` on its check button answers) or as
`message_index=Advice/List`. The droids open the Advice index after the
briefing, and Advice holds the game at Very Slow; `Press escape` closes it.

## Scripted input

On macOS the game builds real `NSEvent`s and hands each one to its own
view's handler (`mouseDown:`, `keyDown:` …) with
`performSelector:withObject:afterDelay:0`. The run loop makes that call
between frames, where miniquad dispatches hardware events, so the event
enters miniquad's input path into both macroquad and egui as a hand's would.
A scripted click therefore tests the input path rather than bypassing it.
A press and its release land in different frames. Other platforms refuse the
input commands.

Do not post through `-[NSApplication postEvent:]`. AppKit spends a posted
button press on activating the inactive window: on 2026-10-09 moves arrived
but presses never did, so a Battle Alert tab ignored every click. Calls to
the view never touch activation. Proven 2026-10-09: a background click
switched the Battle Alert to its Alliance Forces tab while another app stayed
frontmost.

Scripted input stands in for a hand, not for the person: an agent check
driven this way is still an agent check, and is never recorded as a check a
person made.

## The live channel

`--live` (with `--evidence DIR`) makes `DIR/commands.in` empty and sets
`OPEN_REBELLION_INBOX` to it. The game:

- polls the inbox every 250 ms and runs only complete (`\n`-terminated)
  lines, so a half-written append never runs;
- numbers lines from 1 in file order, counting blank and `#` lines, which
  are answered `skipped`;
- refuses a line over 512 bytes, or not UTF-8, unread;
- starts again from the top if the file shrinks (it logs this);
- opens nothing else: a line parses into a command or is refused, and only
  `Capture` writes, inside the evidence folder.

`DIR/commands.out` starts with `{"protocol":1}`, then one JSON line per
command:

```json
{"seq":4,"line":"Capture: 01-galaxy","status":"done","detail":"1280x800","path":"…/01-galaxy.png"}
{"seq":6,"line":"Wait twelve ms","status":"refused","detail":"wait takes `<n> ms` or `<n> frames`"}
```

`status` is `done`, `refused` or `skipped`. Senders may run at once:
`dev-send.sh` holds a lock only while it appends and numbers its line, so
concurrent commands interleave in arrival order, each answered under its own
`seq`. A result is written once its
frame (or its wait, or its input) is over, so `done` means the game has
drawn the frame after it. A change to these fields bumps `protocol`.

## Adding a command

1. Add a `DevCommand` variant and its line in `dev_commands::parse_line`,
   with a `Readiness`.
2. Handle it in `main.rs` beside the others ("Developer commands"). World
   changes go in `dev_actions.rs` and call the production function the event
   would; refuse, with the reason, anything the original would not do.
3. If it takes no arguments, list it in the palette's `script_commands`.
4. Add it to the table above.
