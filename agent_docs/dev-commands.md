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
| `dev_policy.rs` | fast forward, the wait blockers, the Battle and Advice policies |
| `dev_actions.rs` | `Post message`, `Battle at`, `Blockade`, `Give Death Star`, `Damage ship`, `Mission:`, `Recruit:` |
| `crates/rebellion-data/src/readouts.rs` | the text of the state readouts, shared with the playtest REPL |
| `crates/rebellion-render/src/control_registry.rs` | the named-control registry the draw functions file into |

## The loop

The game runs on the Mac Mini, from the copy `scripts/mini-cargo.sh` builds
there (`~/or-native/open-rebellion`). Build it from this Mac, then drive it
over ssh and pull the evidence back:

```bash
scripts/mini-cargo.sh -- build --release -p rebellion-app
E=.artifacts/native-checks/<date>-<slug>
ssh mac-mini-ts "cd or-native/open-rebellion && (scripts/launch-native.sh --live --seed 42 --evidence $E >/dev/null 2>&1 &)"
ssh mac-mini-ts "cd or-native/open-rebellion && scripts/dev-send.sh $E 'Start game: Empire'"
ssh mac-mini-ts "cd or-native/open-rebellion && scripts/dev-send.sh $E 'Capture: 01-galaxy'"
rsync -a mac-mini-ts:or-native/open-rebellion/$E/ $E/
```

The launch runs in the background so ssh returns; `$E/binary.txt` records
the binary's hash and the source commit from `.build-source`.

Read each capture, then send the next command. To see the whole run at
once, use the report (below). `dev-send.sh` appends the
line to `$E/commands.in`, waits for its result in `$E/commands.out` and
prints it as one JSON line. It exits 0 when the command ran, 1 when it was
refused or skipped, and 2 on a timeout or bad usage (30 s by default; `--timeout S`). Every
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
| `Capture` / `Capture: <name>` | anywhere | Saves the frame being drawn as `<evidence>/<name>.png` (unnamed: `capture-N`). The name takes letters, digits, `-`, `_` and `.`, and may not start with `.`. The result carries `path` and the pixel size. `Capture <anything>` without the colon is refused. |
| `Status` | anywhere | Reports `mode`, `day`, `speed`, `pause_alert` (the "Resume Game Play?" alert is up), `battle_alert`, `message_index` (category/view or `closed`), `briefing`, `side`, each side's top strip as `alliance=`/`empire=` raw/refined/capacity-load, and `fast_forward=on|off`. |
| `Wait <n> ms` / `Wait <n> frames` | anywhere | Holds the queue; the result arrives when the wait ends. At most 600,000 ms or 36,000 frames; a larger count is cut to the cap. |
| `Wait until day <n>` / `Wait until battle alert` / `Wait until battle` / `Wait until galaxy` | anywhere | Holds the queue until the galaxy clock reaches the day, the Battle Alert opens, the battlefield shows (not the Battle Alert or the Results, which share its mode), or the galaxy view shows; the result notes `day=… after N frames`. After 36,000 frames it is refused with the day and mode, and the queue moves on. Give `dev-send.sh` a `--timeout` long enough for the wait. At Very Slow a day can take about 8,000 frames (2026-10-11), so raise the speed before waiting several days. A wait that needs the clock (every one but `battle alert` and `battle`) is refused at once, as `blocked: <cause>`, once the cause has held 30 frames: `battle alert at <system> day=N` (unless the Battle policy answers it), `pause alert (Resume Game Play?)`, `the advisor's briefing holds the clock`, `main menu`, `the clock is paused` (the speed is Paused), `Results/Close` or `MessageIndex/Close` not closing their window after three clicks; or, after 120 frames outside the galaxy with no policy answering, `mode <mode> for 120 frames`. |
| `Wait until <anything> within <n> frames` | anywhere | Any `Wait until` line may end in `within <n> frames` (1 to 36,000): the wait is refused as `waited <n> frames for <condition>; day=… mode=…` when the budget runs out, and the queue moves on. It is the way out of a wait whose event may never come, such as `Wait until battle alert within 600 frames`. Without it the cap is 36,000 frames. |
| `Wait until mission resolved` / `Wait until mission <id> resolved` | anywhere | Holds the queue until the player's side resolves a mission after the wait began, or the mission with that id (`mission_id`) has resolved. Only the player's side's missions count; an id that is not an active mission of the player's side when the wait starts is refused. A `Load:` clears the count. Read-only. |
| `Wait until message <category>` | anywhere | Holds the queue until a message reaches the player's Message Index in the category (`All`, `Popular Support`, `Fleet`, `Mission`, `Resource`, `Manufacturing`, `Defense`, `Conflict`, `Chat`, `Advice`) after the wait began; All takes any message that reaches the index. Read-only. |
| `Wait until pause alert` | anywhere | Holds the queue until the "Resume Game Play?" alert is up. Read-only. |
| `Wait until tech <alliance\|empire> <ship\|troop\|facility> <n>` | anywhere | Holds the queue until that side's research level in the tree is at least `n` (met at once when it already is). Read-only. |
| `Fast forward: on` / `Fast forward: off` | anywhere | Dev-only, no menu entry, not a speed: while on and a wait that needs days is pending (`Wait until day`, `tech`, `mission resolved`, `message`, `pause alert`, `battle alert`, `battle`), the frame loop advances the clock exactly one day a frame (`seconds_per_day`), never more, so every system sees one `tick_events` entry. Between commands, and during `Wait <n> ms` and `Wait until galaxy`, the clock runs at the chosen speed. A paused clock or a stop day still holds. `Status` reports the flag. |
| `Battle policy: simulate` / `retreat` / `stop` | anywhere | What a running wait does when the Battle Alert opens. `stop` (the default) leaves it up and the day waits refuse as blocked. `simulate` and `retreat` click that button on its own rect through the input path (`BattleAlert/Simulate`, `BattleAlert/Retreat`, looked up in the control registry each frame), and `simulate` then clicks `Results/Close` so the galaxy returns and the wait goes on. `retreat` runs the battle it opens (every ship with a working hyperdrive flees) for up to 3,600 frames, then closes the Results and logs `[dev-policy] retreat battle ended day <n>`; past the budget the wait refuses as blocked. A button still not closing the alert after three clicks (Retreat shown disabled) is left, and the wait refuses as blocked. Logs `[dev-policy] battle at <system> day <n> -> simulate`. |
| `Advice policy: close` / `stop` | anywhere | What a running wait does when the Message Index opens on Advice. `close` clicks `MessageIndex/Close`, and answers the pause alert that follows within 180 frames by clicking `PauseAlert/Resume`. A control not in the registry for 30 frames refuses the wait as `blocked: <name> not registered`. Logs `[dev-policy] advice closed day <n>`. |
| `Click <x>,<y>` / `Right click <x>,<y>` / `Ctrl click <x>,<y>` | anywhere | Move, press and release at the point, one per frame; `Ctrl click` holds Control around them. |
| `Click control: <window>/<control>` / `Right click control: <window>/<control>` | anywhere | Clicks the center of a named control, found in the last drawn frame's control registry, through the same scripted input as `Click <x>,<y>`. Names ignore case, and `-`, `_` and space match. A name that matches nothing is refused with the names registered under its window. See "Named controls" below. |
| `Locate <window>/<control>` | anywhere | Reports the control's logical rect and center, or refuses as above. |
| `List controls` / `List controls <window>` | anywhere | Reports the names registered in the last drawn frame, all or one window's. |
| `Drag <x>,<y> to <x>,<y>` | anywhere | Press, move over 8 frames, release. |
| `Hold drag <x>,<y> to <x>,<y>` / `Release at <x>,<y>` | anywhere | A `Drag` in two halves: press and move, leaving the left button down, then release. Between them another app can take focus (`open -a Finder` on the host), which a `Drag` ends too quickly to meet. If a post fails during the hold, the button stays down until a `Release at`. |
| `Scroll <x>,<y> <n>` | anywhere | Moves to the point, then posts `n` mouse-wheel lines, one a frame: positive scrolls down a list, negative up (1 to 100 either way). The port's lists without a drawn scroll bar (the Create Mission kind list, its agent lists) move one row per line. |
| `Press <key>` | anywhere | Press and release: a letter, a digit, `f1`–`f12`, `escape`, `return`, `space`, `tab`, `delete`, the arrows, `home`, `end`, `pageup`, `pagedown`. |
| `Start game: Alliance` / `Start game: Empire` | main menu | Starts through the menu's faction control. |
| `Skip briefing` | galaxy | The tour's own skip, as Escape: the reply plays (ALBRIEF 10165 / EMBRIEF 11157). Refused when no briefing runs. |
| `Open Message Index: <category> [<row>\|last]` | galaxy | Opens the window on All, Popular Support, Fleet, Mission, Resource, Manufacturing, Defense, Conflict, Chat or Advice. A row (1-based in list order, oldest first) or `last` shows that message in mode 2; the result names it, and the log records its sound (`[message] sound 0x… started volume=…`; 0 under `launch-native.sh`'s mute). Advice drops the speed to Very Slow, as the original does. |
| `Post message: <class> at <system>` | galaxy | Files `uprising began`, `uprising ended`, `blockade`, `fleet arrival`, `loyalty joins` or `loyalty neutral` through the class's builder, to both sides where the original files both. Refused where the original files nothing (a neutral system, no blockading fleet, no fleet). |
| `Battle at <system>` | galaxy | Moves an idle fleet of each side missing there, through `apply_fleet_arrival`. The next tick's combat check opens the Battle Alert, so the game must not be paused. The result names the holder and the fleets moved. |
| `Give Death Star: <fleet>` | galaxy | Marks the named Imperial fleet as carrying the Death Star (`Fleet::has_death_star`, `DeathStarState::death_star_fleet`), as a finished construction leaves it. Send it after `Battle at`, which brings only fleets without one. Only Imperial fleets are searched. Refused for an unknown name, or while another fleet carries it. |
| `Damage ship: <fleet>` | galaxy | Halves the hull of the first live capital ship in the player's fleet of that name, as a battle's damage import leaves a hit ship. The repair system arms the ship's timer on the next tick by its own rules: the short interval at the side's own working shipyard, the long one elsewhere. Native telemetry records each step as `ship_repaired` (`fast`, `next_due`). Staging only; the damage counters are untouched. Refused for an unknown name or a fleet with no capital ship. |
| `Knock out hyperdrive: <n>` | anywhere | In a battle (the Battle Alert included), puts the player's nth living ship (1-based, roster order) out of hyperspace through the battle's own subsystem hits. Refused with no battle, past the last ship, or when that hyperdrive is already out. |
| `Blockade <system>` | galaxy | Moves an idle fleet of the side not holding the system; the simulation forms the blockade. Refused for a neutral system or where the holder has a fleet. |
| `Advisor: post code <n>` | galaxy | Posts advice code `n` (decimal or `0x` hex, up to `0xff`) to the droids, as a game event does. |
| `Release advice topic <n>` | galaxy | Releases held topic `n` now and files it under Advice. Refused before the opening topics are filed, or for a topic already filed. |
| `List systems` | galaxy | Reports every system as `<name> <holder>/<fleets>`: `A`, `E` or `-`, then the sides with fleets there; a system with squadrons adds `squadrons <a>/<e> fleets <a>/<e>`, those the system holds and those its fleets carry, a system with a shipyard adds `yards <n>`, and a populated system adds `support <a>/<e>`, each side's support in points. Use it to choose a `Battle at` or `Blockade` target. |
| `List advice topics` | galaxy | Reports `held: [n, …]`, the held topics in release order (empty before the opening topics are filed). |
| `List characters [alliance\|empire]` | anywhere | One row per character: name and `#<dat id>`, side (`major` for a major), where it stands (a system, `<fleet> at <system>`, `<fleet> in transit <from> to <to> (n days)`, `en route to <system>`, `not placed (recruit pool)`), its current mission (`<kind> at <system> (#<id> <phase>)`), the skills `dip esp com lead` (the kinds') and `ship troop fac` (Research's), rank, and `free`, `captive of <side>` or `killed`. The port keeps no health or injury for a character. First row: the count. |
| `List missions [alliance\|empire]` | anywhere | One row per active mission: `#<id> <side> <kind> team […] [decoys […]] target <system> phase <n> (<transit\|timer\|outcome\|end\|step>) days left <n>`; `decoys` appears only when the mission has some; days left is `-` outside the transit and timer waits. |
| `Show research` | anywhere | Six rows, `<side> <ship\|troop\|facility> level <n> points <n> next order <n> cost <n>` (`ResearchSystem::next_order`, `scaled_cost`), or `next none (tree exhausted)`. |
| `List production [alliance\|empire]` | anywhere | Per system and area with a queue: `<system> (A\|E) <shipyard\|training\|construction>: <n> items`, then `[i] <class> <work done>/<cost> eta day <d> -> <destination>`. The eta is the Status window's best case with the holder's yards working and materials on hand; `-> stays` builds in place. |
| `Show side alliance\|empire` | anywhere | `<side> raw <n> refined <n> maintenance used <n> of <capacity> systems <n> fleets <n> characters <n> missions <n>`: the top strip and counts. |
| `Save: <name>` | galaxy | Saves the campaign through the game's own save path (`LiveCampaign::snapshot`, `save::save_file`) to `<evidence>/saves/<name>.reb`; the result carries `path`, the tick and the save's fingerprint. Names take letters, digits, `-`, `_` and `.`. Refused outside the galaxy (the main menu, a battle) and while the Battle Alert, the pause alert or the mission dialog is open. It opens no slot and logs no message. |
| `Load: <name>` | anywhere | Loads `<evidence>/saves/<name>.reb` through the game's own load path (the `LoadGame` handler, which resets the windows and the rest of the interface), from the main menu or the galaxy; the result carries the tick and fingerprint. Refused in a battle, behind the same three modals, or for a missing file or one the loader rejects (wrong `SAVE_VERSION`, bad fingerprint). Staging shortcut: it stands in for Game Options' Load Game, never for a check of that dialog. |
| `Mission: <character>[, <character>…] <kind> at <system> [decoys <character>, …]` | galaxy | Checks the team against `missions::available_kinds`, the list the Create Mission dialog offers, and refuses a kind it would not offer, naming the kinds it does; a copy of the campaign then runs `dispatch_guarded`, so a refusal the validator would give is named too. A passing order queues the `PanelAction::DispatchMission` that Begin Mission builds, which ends in the same `dispatch_guarded`. Kinds as the dialog names them (`Diplomacy`, `Recruitment`, `Espionage`, `Incite Uprising`, `Subdue Uprising`, `Move`, `Ship Design Research` …); `Research(Ship)`, `Research(Troop)` and `Research(Facility)` also work. Sabotage, Death Star Sabotage, Assassination, Abduction and Rescue need a target object or character the line cannot name, so they are not accepted. The result is written once the dispatch has run: `dispatched`, or `refused: <reason> (0x<a>/0x<b>)` when the real `dispatch_guarded` refuses it, with the advisor's status code. Characters of the player's side only, by name or `Name #<id>` when two share a name. System targets only. Staging shortcut: it never replaces a check of the Create Mission dialog itself. |
| `Recruit: <character> for <alliance\|empire> [at <system>]` | galaxy | Applies Recruitment's success effect, `missions::recruit_character`, to one character of that side's recruit pool: it joins at the system (one the side holds), or at the side's headquarters. Refused for a system the side does not hold, a character already recruited, a major, one of the other side, or captive. Staging shortcut for a Recruitment mission's outcome; it does not run the mission and draws no random number, so the campaign's random stream afterwards differs from one where the mission succeeded. |
| any palette label | galaxy | `Open sector window: <system>`; `Open System\|Defenses\|Fleet\|Missions window: <system>`, which opens the window at its icon as a double click there does; `Open system\|defenses\|fleet\|missions icon menu: <system>`, which selects the icon and opens its pop-up menu as a right click does; the simulation commands (advance days, game speed, reveal all). A window or menu behind a hidden icon is refused (the icon is hidden); only the System window opens without one. |

The policies act only while a `Wait <n> ms`, `Wait <n> frames` or `Wait until`
is running, not in the hold frames after a click, drag or key, and not while
the wait is `Wait until battle alert` or `Wait until battle`, so a script can
still stop at the alert it asked for. Closing the Results or the Advice index
is tried three times; a window still up then refuses the wait as blocked.

"Galaxy" commands wait until the galaxy shows and the previous command's
interface work has finished. A galaxy command still waiting after 600 frames
(about 10 s; for example behind a Battle Alert or in a battle) is refused with
`waited 600 frames for the galaxy view; the game is in <mode>`, and the queue
moves on. "Anywhere" commands run in the main menu,
battles and modal screens too. The palette lists the argument-free ones
(`Capture`, `Skip briefing`, each `Open Message Index` category).

### Readouts: dev truth

`List characters`, `List missions`, `Show research`, `List production` and
`Show side` read the world and its states directly. They see through fog of
war, so they show the enemy's characters, missions and queues too: dev
truth, not the player's view. Never record one as something the player saw.
The playtest REPL (`--repl`) answers the same lines in lower case
(`list characters alliance`, `show side empire`).

### Save and Load

A save holds the campaign state of `SaveState` (`agent_docs/save-load.md`):
the world, clock, every system's state, both fog states, the RNG, the player's
side and the configuration. `Load:` rebuilds the interface around it, so a
save leaves out:

- the message log (a loaded game starts with one line, `Loaded save …`), and
  with it every Message Index row and its sound;
- the advisor and advice-topic state: the opening briefing, held and filed
  topics, `Advisor: post code` effects, and the saved speed Advice restores;
- every open window and dialog (sector, system, fleet, Create Mission, the
  Message Index, Status, finders), the map's pan and selection, targeting and
  the object menu;
- a battle, the Battle Alert and any ground assault in progress;
- the Game Speed UI state and the Build Selection and Move confirmation
  states.

A check about any of these never starts from a save: reach it from a new
game. The clock keeps its speed and partial day, so a game saved paused
loads paused. `scripts/launch-native.sh --live --evidence DIR --load FILE`
copies `FILE` into `DIR/saves/`, puts `Load: <name>` first in the inbox and
so starts the game in the save; `Load:` is seq 1 in `commands.out`, and
`dev-send.sh` numbers its lines after it.

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

### Named controls

A dev run (the gate above) files each window's hit rects by name, every frame,
in `rebellion_render::control_registry`, in the same 640×480 logical space as
`Click`. The registry holds the frame just drawn, so a name is findable once
its window has drawn a frame. Off a dev run `set_enabled` stays off and each
registration is one atomic load; nothing reads game state. A name filed twice
in a frame keeps its last rect (the focused window draws last).

| Window | Controls |
|---|---|
| `BattleAlert` | `summary`, `alliance-forces`, `imperial-forces`, `system-summary` (tabs); `Retreat`, `Simulate`, `Tactical` (filed even while Retreat is disabled) |
| `Results` | `Close`; `summary`, `alliance_forces`, `imperial_forces`, `go_directly_to` (tabs); `Capital Ships`, `Fighters`, `Troops`, `Personnel` (on the force tabs) |
| `PauseAlert` | `Resume` |
| `MissionDialog` | `Close`, `MissionTab`, `AgentsTab`, `KindDropdown`, `KindList` (open drop-down), `AgentList`, `DecoyList`, `Agent <name>`, `Decoy <name>`, `ToDecoys`, `ToAgents`, `Cancel`, `Confirm`, `Encyclopedia` |
| `ContextMenu` | each row by its label (`Mission`, `Move` …), the object menu and its open submenu |
| `AgentMenu`, `SpeedMenu` | each row by its label |
| `Defenses` | `Sector`, `Minimize`, `Close`, and each tab by its name, for every open window (the focused one wins a clash) |
| `Personnel` | each character row of an open Defenses window, by name |
| `MessageIndex` | the categories by name (`All`, `Popular Support` …), `Close`, `DisplayMessage`, `Silently`, `Display`, `ComposeChat`, `SelectAll`, `Delete` |

Not yet registered: the sector, system, fleet, manufacturing, missions and
status windows, the finders, the build-selection and move/scrap confirmations,
the cockpit's buttons, the main menu, the tactical battle HUD, and the
Message Index's list rows.

### Command scripts

`scripts/launch-native.sh --commands FILE` runs `FILE` from launch: one line per
frame, `#` starts a comment, blank lines are skipped. It sets
`OPEN_REBELLION_COMMANDS` and `OPEN_REBELLION_DEV=1`; without the dev switch a
release build ignores the file. A line runs once the game shows the main menu
or the galaxy and the previous line's commands have run, and the script waits
while a start is setting up. `Start game: Alliance` and `Start game: Empire`
run from the main menu, so a script can begin there; the palette lists no
start, since it draws only in the galaxy. Each line logs `[dev-command] sent`
or `[dev-command] unknown` to `game.log`, a `refused: <why>` line belongs to
the line just before it, and `[dev-command] done` follows the last command.
Every new campaign takes a fresh random seed; `--seed N`
(`OPEN_REBELLION_SEED`, which also needs the dev switch, set by the launcher)
makes a script meet the same galaxy each run. Record the seed with the
evidence.

State commands (`Start game`, `Open … window`, `Battle at`, `Post message`)
reach a state without testing the input that leads to it. Drive the step under
test with `Click`, `Drag` and `Press`, or with cua-driver, and say in the
evidence summary which steps a command took. cua-driver's `press_key` cannot
send the backtick, so drive the palette by hand or through a script.

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

### Batches and telemetry

```bash
scripts/dev-run.sh $E steps.txt     # from this Mac: one ssh, then the evidence pulled back
scripts/dev-send.sh $E --file -     # on the Mini: the same batch from stdin or a file
scripts/telemetry.py $E --type combat_space --tick 100..200
```

`steps.txt` holds one command per line (blank and `#` lines skipped). An
`Include: PATH` line splices in another script before anything is sent, so the
game never sees it: `scripts/dev-expand.sh` does it for both `dev-run.sh` and
`dev-send.sh --file`. PATH is relative to the including file, else to the
repository root; includes nest up to 8 deep, and a cycle or a missing file stops
the batch with `FILE:LINE: …` before the first send. Every native new-game
script carries `Include: scripts/stages/unattended.txt` (after
`Start game: <side>` and `Skip briefing`): the battle and advice policies, the
Message Index close, and the stray pause alert. Each sent
line goes through the same lock and wait loop as a single send, with
`--timeout` per line, and its result prints. The batch stops at the first
refusal and exits 1; `--keep-going` sends the rest and still exits 1. A
timeout stops it either way (exit 2). `dev-run.sh` rsyncs `$E` back without
deleting, even after a stopped batch.

When `$E/telemetry.jsonl` exists, `dev-report.py` lists its records under the
command (`seq`) they ran under, marked `~`, and marks `[dev-policy]` lines
`*`. `--no-telemetry` leaves them out. Query the file directly with
`scripts/telemetry.py`, which reads headless playtest logs the same way.
Calibration and the native-versus-headless check are in `test-tooling.md`.

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
