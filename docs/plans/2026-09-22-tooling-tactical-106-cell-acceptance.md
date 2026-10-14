---
title: "Tactical 106-Cell Acceptance Train"
description: "Final local commit train for a runnable and honestly accepted TAC-01 through TAC-07 parity matrix"
category: plan
created: 2026-09-22
updated: 2026-10-14
tags: [interface, parity, tactical, browser, evidence, testing]
status: in_progress
---

# Tactical 106-Cell Acceptance Train

This is the final acceptance companion to the
[Standalone Space-Battle Test Launcher](2026-09-12-tooling-standalone-space-battle-launcher.md).
It scopes the local commit set that makes every `TAC-01` through `TAC-07`
acceptance cell executable, compares the reconstruction with authoritative
original-game evidence, closes remaining production behavior, and produces one
auditable 106-cell verdict before the commits are pushed together.

The 640×480 original composition remains authoritative. Enhanced widescreen is
a later modernization track and is not part of this denominator.

The A0 capture run goes past this train's 106. It also takes the eight
ground-assault cells, `TAC-08-C001` through `C008` (114 in all), so the C6
comparison covers the original's report-only assault flow. Porting that flow
is its own roadmap item (`UIP-F-010`, `RE-GND-01`). `TAC-08` is an A0-only
family (`A0_ONLY_FAMILY_COUNTS` in `tools/interface-parity/tactical-matrix.mjs`).
The ingester takes its cells through `collectA0Cells`, while
`collectTacticalCells` and the strict matrix stay at 106. The manifest schema
allows 114 captures.

## Current boundary

- P58-B06 passes a 120-case deterministic implementation matrix across both
  factions and two browser viewports.
- Independent A1 review found no P0 or P1 defect and left three bounded review
  findings.
- P58-B21 shares played and automatic post-battle bombardment, landing,
  contested ground continuation, unopposed occupation, and result destination
  routing. Focused tests and four muted browser cases pass.
- P58-B22 closes the bounded practical launcher implementation with source-
  compatible tactical RNG sequencing, persisted power allocation, exact
  completion/destruction callbacks, and the timed trench-run producer. This
  advances implementation, not strict A0 acceptance.
- The strict matrix contains exactly 106 pending cells: 13 in `TAC-01`, 13 in
  `TAC-02`, 20 in `TAC-03`, 14 in `TAC-04`, 23 in `TAC-05`, nine in `TAC-06`,
  and 14 in `TAC-07`.
- A0: 113 of 114 references are captured from the original in the A0
  Windows VM, and TAC-03-C020 is settled without a frame (sessions 5 to 7).
- C6: the first strict verdict over all 106 cells is pass 11, fail 66,
  not-run 28, settled 1, with the negative controls failing their rows
  (6 of 6). The remaining work is triaged into the task graph
  (`agent_docs/task-graph.md`), one acceptance node per failing region.
- Original binaries, extracted resources, saves, screenshots, audio, and video
  remain local and ignored. Only metadata, tooling, hashes, and permissible
  evidence summaries enter Git.

## Two numbers, never conflated

`matrix_coverage` is the number of ledger cells with a unique catalog row and
an executable deterministic journey or snapshot. The runner may report
`106/106 runnable` even when cells fail.

`parity_acceptance` is the number of cells whose complete evidence contract
passes. The project may report `106/106 accepted` only when every row has:

1. authoritative original evidence and provenance;
2. exact resource identity where a shipped resource is involved;
3. matching 640×480 composition within the audit thresholds;
4. exact interaction geometry and real action semantics where applicable;
5. native and packaged-WASM behavioral agreement;
6. correct audio or media routing where applicable; and
7. clean console, network, missing-asset, process-cleanup, and state-fingerprint
   results.

Missing evidence is `reference-needed`, not a pass. A fixture-created snapshot
can prove presentation but cannot prove the action that normally reaches it.

## Canonical artifacts

Tracked artifacts:

- `2026-10-06-surface-ledger.json` owns the 106 stable cell IDs and acceptance status.
- `tactical.catalog.json` owns deterministic A1 scenarios and exact `audit_cells`
  mappings.
- `tactical-a0-manifest.schema.json` validates original-capture metadata.
- `tactical-a0-manifest.example.json` documents the local manifest format
  without proprietary paths or media.
- Generated evidence JSON records coverage, results, thresholds, hashes, and
  redacted artifact identifiers.

Ignored local artifacts:

- `a0/<cell-id>/original.png` or the applicable lossless media capture;
- `a1/<cell-id>/reconstruction.png`;
- pixel, edge, geometry, interaction, audio, and state traces;
- original saves and any editor-assisted setup files; and
- full browser logs containing proprietary asset names or paths.

Each A0 record includes the cell ID, original executable SHA-256, installation
identity, language, VM image/version, Windows display settings, unscaled client
rectangle, capture method, fixture/save provenance, ordered input trace,
timestamp, and artifact SHA-256. The captured game client must be exactly
640×480 with no host scaling, filtering, window chrome, or recompression.

## Local commit set

The commits remain local until the final push gate. Each commit is independently
reviewable and must leave all existing focused tests green.

### C1. Define and validate the 106-cell matrix

Status: complete at P58-B07. The generator reports the current honest boundary
as 0 mapped, 0 A0, and 0 accepted while preserving all 106 stable rows.

- Generate the tactical matrix directly from `2026-10-06-surface-ledger.json`.
- Fail on missing, duplicate, unknown, or non-counting cell IDs.
- Require exactly 106 unique rows distributed 13/13/20/14/23/9/14.
- Add A0 manifest schema validation and redacted evidence paths.
- Add unit tests for denominator drift, duplicate mappings, missing provenance,
  and accidental proprietary-file staging.

Gate: the empty matrix can be generated and must report `0/106 accepted,
106 reference-needed`; it cannot report success merely because the runner
started.

### C2. Capture and ingest authoritative A0 evidence

Status: ingestion infrastructure is complete at P58-B08. The guest capture
script now emits executable identity and per-cell provenance, and the host
ingester fails closed on dimensions, hashes, source identity, ledger drift,
unsafe destinations, or replacement bytes. The owned capture host was offline
on 2026-09-22, so capture coverage remains 0/106 and this phase is not complete.

2026-10-11: the capture VM is the A0 route, with the game full screen.
GOG's DDrawCompat ini leaves `DisplayResolution` at its default, `desktop`,
which scales the 640×480 mode to the guest's 800×600 desktop (DDrawCompat
wiki, Configuration), the only mode its `virtio-ramfb` display lists. `-w`
gives an unscaled window, but it renders differently (32-bit colour,
anti-aliased text, other 3D shading), so windowed frames are not A0 and the
three registered that way were withdrawn. The owner chose to keep 800×600.
With `DisplayFilter=point` the scaling only repeats whole pixels in a fixed
pattern (160 columns, 120 rows), so `capture.ps1` checks every repeated line
equals its neighbour and drops it, recovering the game's 640×480 frame
bit for bit, and keeps the 800×600 source beside it. `vm-game.sh up-a0`
starts the game with the capture watcher, and `vm-game.sh a0` takes each
capture (`tools/original-capture/README.md`, "A0 captures"). A Wine prefix
on the development Mac was set up and dropped the same day.

2026-10-12: 75 of the 106 cells have an A0 frame in the ignored manifest
(`.artifacts/interface-parity/a0/manifest.json`); none is compared or
accepted yet. Damage, subsystem and results states come from edited copies
of our day-30 save (`agent_docs/original-save-editing.md`, recipe 3). A
cell about motion (running, observing) takes a burst of frames at a fixed
cadence instead of one hash-stable frame; the ingester stores the frames and
their offsets beside the cell's frame, which is the burst's last. Three
captures of TAC-01-C004 that showed the results screen were withdrawn. The
31 left. Plan for them (2026-10-12), cheapest first, with what the first
75 taught:

- A ship at 10 hull has dark weapon icons and neither fires nor manoeuvres,
  and the day-30 Alliance has one weak Bulk Cruiser, so it hurts nothing:
  our own ship cannot be made to die there. Deaths and shield damage come
  from the enemy taking the ISD's fire.
- With its hyperdrives intact a hurt cruiser withdraws; with them out the
  battle runs until it dies (`stranded`). The battle ends the moment the
  last enemy dies, so a still of a death is taken by pausing as soon as a
  watched region changes (`vm-game.sh wait X,Y,W,H`), and a burst must end
  before the results screen.
- `FUN_005ae460` fills an empty fighter colour with half of the largest
  group, and only when that group holds more than seven squadrons. Fleet 1's
  six TIE squadrons therefore stay Red: Blue needs eight or more squadrons
  on one side and Gold about sixteen.

Session 1, the unedited day-30 save (`empire-day30-yaga-alert-control`):

| Cell | Route | Capture |
|---|---|---|
| TAC-02-C011 selected | click task force 1 | still |
| TAC-03-C014, C015 previous, next | Ctrl-select the ISD and the Carrack, click the blank task-force slot to make one two-ship task force (manual p. 144), then the status panel's arrows | still each, with before |
| TAC-04-C014 chase camera | unpause, F1 follows task force 1 (manual p. 148) | burst |
| TAC-05-C004 hold | Maneuvers and Tactics, Hold Position (game 556,171), commit (`ghidra/notes/tactical-command-assignment.md`) | still |
| TAC-05-C008 attack fighters | Missions, Attack Fighters, commit | still |
| TAC-07-C013 go to system | Battle Results, Go Directly To…, System (manual p. 152); TAC-07-C012 took the Fleet branch | still |
| TAC-07-C014 strategic persistence | finish the `damage` battle, open Fleet 1's ISD Status, save, reload, read it again | still |
| TAC-01-C007 empty space | find the Show Planet control (manual pp. 141–146), turn the planet off | still |

Session 2, edited saves, the enemy as the casualty:

| Cell | Route | Capture |
|---|---|---|
| TAC-03-C006 shield | done in the session 1 run | |
| TAC-03-C017 target lost | done, burst (see below) | |
| TAC-03-C004 destroyed | done, burst (see below) | |
| TAC-04-C003 multiple targets | Ctrl-right-click a second enemy (manual p. 145); the day-30 Alliance has one capital, so its fighters or session 3 | still |

2026-10-12, session 1 run: 84 of 106 now have an A0 frame. Captured:
TAC-02-C011, TAC-03-C014 and C015, TAC-05-C004 and C008 (chosen, not
committed, like the other TAC-05 cells), TAC-07-C013 and C014 from the
control and `damage` saves, and TAC-04-C014 and TAC-03-C006 from
`stranded`. What it taught:

- Ctrl-select works on ships in the viewport, not on task-force buttons, and
  needs Ctrl held in the same VNC connection as the click: `point X Y`, then
  `vnc keydown ctrl click 1 keyup ctrl`. A Ctrl-clicked ship leaves its own
  task force for the selected one, so one Ctrl-click of the ISD with task
  force 2 selected made a two-ship task force 2 and emptied task force 1.
- A before frame needs the cell's state setup: `a0 NAME OUTDIR
  StateSetupKind=original-save StateSetupIdentifier=...`, or the ingester
  refuses the pair.
- The control save's Alliance withdraws seconds after the unpause, so any
  running burst comes from `stranded`; the first chase burst caught only the
  victory screen and was withdrawn.
- Go Directly To's system button closes onto the sector map without opening a
  system window, unlike the Fleet button.
- The strategic Status reads Hull Value 550:2750 after the `damage` battle,
  and Ship Damaged: No although the subsystem hits stand. Saved again on
  day 46 (the Status dialog must be closed first, with Escape, or it takes
  the save keys), all three copies of the ISD's record still hold 2200 hull
  damage and the packed hits 0x1121.
- Loading a save from Game Options opened inside a battle closed the game.
  Load only from the galaxy map or the main menu.
- TAC-01-C007 is open. Show Planet cannot change in a battle (manual p. 77;
  greyed out in the battle's Game Options), and every load resets the five
  display options to On: turned off on the main menu before the load, the
  planet still showed. The option is not in the save (saves either side of
  the toggle are byte-identical) nor the registry's `DisplayOpts` (0 left
  all five On), so no save edit can stage it, and the day-30 Yaga saves open
  their alert during the load. C007 moves to session 3: turn Show Planet off
  on the galaxy map after loading `alliance-day54-advice.sav` and play on
  into the battle. The Empire planet-on view (task force 1, Center, Zoom Out
  ×3, rotate left ×4) is kept as `PL-on`, unpaired; the cell is a still.

The same day, session 2 on `fragile` gave TAC-03-C017 and C004, so 86 of 106
have a frame:

- `a0-cell` takes about 27 s from the request to a burst's first frame, and
  the cruiser died inside that gap, so the first try caught only the results
  screen and was withdrawn. A burst for an event starts paused, and a
  background job clicks Resume Battle about 24 s after the request.
- The original holds the battle still for about 3 s after the last enemy
  dies, then opens the results, so no pause can keep the death on screen.
  The burst holds it: C017's frames 45–50 show the target lost (No Orders,
  Missions and Maneuvers blank), and C004's frames 38–43 show the destroyed
  cruiser's panel fall back to the Imperial seal. Their first registration
  took the burst's last frame, the results screen, as the cell frame; both
  were withdrawn and registered again with `a0-register CELL N`, which takes
  a named burst frame as the cell frame (C017 frame 45, C004 frame 38).
- After several loads in one run the game crashed on its own; a fresh
  `a0-game` and the main menu's Saved Games load worked.

Session 3, an Alliance battle: play `alliance-day54-advice.sav` forward,
send a fleet against an Imperial system, and save at the alert as a new
library save (`saves/INDEX.md`). It gives TAC-01-C001, TAC-02-C013,
TAC-05-C016 and the Alliance side of every both-factions gate, and with
two or more capitals a side TAC-01-C009 (multi-ship) and TAC-04-C003. The
audio cell TAC-01-C012 records the guest's sound through the QEMU monitor
(`wavcapture`, `vm-game.sh mon`) during an engagement in this or session 1.
Turn Show Planet off on the galaxy map before the fleet arrives, for
TAC-01-C007 (empty space); the alert save itself reloads with the planet on.

2026-10-12, session 3 run: 92 of 106 have a frame. From
`alliance-day54-advice.sav`, Fleet 1 (Corellian Corvette 1, Medium
Transport 1) was sent from Ithor to Coruscant and saved in transit on day
149 (`saves/alliance-transit-day149.sav`); the Battle at Coruscant alert
opens about 400 s after its load, against Judicator and Tyrant. The first
run, played on from the planet-off galaxy without a reload, gave TAC-01-C001,
TAC-02-C013, TAC-05-C016, TAC-01-C007 (planet off, on the `PL-on` camera
route) and TAC-04-C003; a reload of the transit save gave TAC-01-C009.
What it taught:

- The Alliance's alert draws its auto-resolve button red, not green;
  `wait alert` now detects both. Its middle button auto-resolves, so use
  `take-command` (an earlier click on it lost the first Coruscant battle).
- Ctrl-right-click adds a target only with Ctrl held about half a second
  either side of the click; a quick chord replaced the target three times
  out of three. Two targets show as two boxes, and the orders keep naming
  the first (manual p. 145: targets are attacked in selection order).
- The fleet dies about a minute after Resume Battle, inside `a0-cell`'s 27 s
  start-up, so the C009 burst caught only Battle Results and was withdrawn.
  It was staged again as a still: run in 15 s steps, pause at first contact.
- `vm-wait.py` stalled 43 minutes on one VNC refresh (vncdotool's default
  call timeout is an hour); every call is now bounded at 20 s.
- TAC-01-C012 (93 of 106): the ingester now takes audio (`--audio`,
  `--audio-metadata`; `vm-game.sh a0-audio start|stop CELL`). From a reload
  of the transit save, stepped to first contact: recording started, about
  8 s of combat, Pause, the cell's still, Resume to Battle Results, stop. The
  WAV holds 117 s over a 204 s span; `audio.json` places the frame 65 s
  into the span. Its first registration lacked a stop time, which the
  sidecar gained from the VM log (`a0-audio stop` at 15:10:54 local); it was
  withdrawn and registered again. Two earlier recordings stay unregistered:
  227 s from the first run (`s3/TAC-01-C012.wav`, `561a9a54…85ec735`,
  no frame inside it), and one whose burst caught only Battle Results
  (`withdrawn/TAC-01-C012-results-screen`).
- The Alliance fleet in this save has no fighters, so the squadron cells
  move to session 4.

Session 4, long play, one save each side: a battle with eight capitals in
one fleet (TAC-02-C001, task forces 1–8) and sixteen or more fighter
squadrons on one side, fleet and system garrison together (TAC-02-C003–C005
blue, green, gold). Records cannot be inserted, so these are built in play
and saved before the alert. TAC-02-C007 (unassigned) is settled from
`FUN_005ae460` and the manual before it is staged; it may be a ship outside
every task force or a garrison squadron joining late.

Session 4 run (2026-10-12): 98 of 106. Four findings came first.

- C007 is the blank slot. `FUN_0059be80` gives each task-force control an
  unassigned bitmap, 1007, and the manual (p. 143) calls it "the blank space
  next to the highest number". It is not a squadron joining late.
- Squadrons count across the whole battle side, not per fleet
  (`ghidra/notes/battle-alert-lists.md`).
- Strength edits cannot add squadrons. Squadron `+0x60` is damage within one
  record, so only real records count.
- The galaxy size decides the start. A Standard Easy reroll gave the Empire
  3 capitals (the old day-2 save has 7). Huge Easy gave 5 and a maintenance
  readout of 738, so capitals could be built: the day-170 Standard game had
  run maintenance down to 5.

The run itself:

- Coruscant built three Orbital Shipyards (40 days each with its two
  construction yards) and three Carracks, which made eight capitals.
- Dragging a fleet's icon onto another fleet's icon in the system window
  merges them. The same window takes squadrons dragged from the planet's
  Fighter Squadrons tab: Fleet 1 carried 16 of 16 TIEs. A drag onto the
  sector map only selects.
- Fleet 1 was ordered to Xyquine, an Alliance system. The alert opened about
  21 days later; `wait alert` fired falsely on the galaxy view twice, so day
  waits polled instead.
- Take Command showed task forces 1–4, two ships each, with the 16 TIEs as
  red, blue, green and gold. That one frame gave C003, C004, C005 and C007
  (slots 5–8 blank). Splitting each task force's second ship into the next
  blank slot gave C001 (1–8).
- A0 refused a frame after many loads in one game run with "column 3
  differs from column 2": the point scaling had drifted (duplicate columns
  2, 7, 13 instead of 3, 8, 13). `a0-game` and a reload restored it. Check
  the pattern on a capture before a long setup. Session 5 showed the phase
  is set at launch, not by loads (below).
- The Battle Alert cannot be saved (F1 does nothing there). The save sits 21
  days before it, as `saves/empire-sixteen-tie-day415.sav`.

Session 5, the Death Star: route 1 of recipe 2. Order a Death Star in an
Empire save, diff saves a cycle apart for the manager's progress, set it one
short of cost, and let the original finish it; then bring it to a battle
the Alliance can attack with fighters (laser ready, loading, fired, the
charge gauge, Attack Death Star enabled and the trench-run launch:
TAC-03-C020, TAC-05-C009, C017–C021, C023; `ghidra/notes/tactical-death-star-path.md`).
Route 2's converted ISD crashes when its fleet moves, so it serves only a
battle where the Death Star already sits.

### Session 5 run (2026-10-12): in progress, 102 of 106

- Route 1 worked (`agent_docs/original-save-editing.md`, "Route 1 in the
  original"). The progress is in the manufacturing order, not in the ship
  records. Set to 583 of 584, the original finished "Death Star 1" into
  Fleet 6 at Coruscant.
- The first trip found no battle. Fleet 6 (with Fleet 1 merged in) flew to
  Yavin (an Alliance system; about 80 days on a Huge map) and nothing
  defended it. The Alliance had meanwhile parked Fleets 32 and 45 at
  Coruscant, so Fleet 6 was sent back (`ds-to-coruscant`, day 605).
  Unrelated alerts (Ketaris, Orto, Ando, Carida) stop the day polling and
  were auto-resolved one by one.
- From `ds-pre-alert` (day 660, local in `.artifacts/original-checks/s4/`)
  the Battle at Coruscant alert opens on day 676 ("The Imperial fleet has
  entered the Coruscant system"), after a Carida alert on day 669. It was
  the same on two separate launches.
- A0 refused that battle twice with "column 3 differs from column 2". The
  phase came up shifted even on a fresh launch's main menu: of four
  launches, two were shifted. The phase is set at launch and kept through
  loads and play (README, "A0 captures"). The fix is to relaunch until a
  probe shows 3, 8, 13.
- Taken as the Empire, the battle opens paused with the Death Star strip
  above the panel: the image, a full green gauge and a green fire button
  (TAC-05-C017, laser ready, `3b37af09…`). The Ghidra constructor sets the
  charge to zero (`FUN_0059ef60`), yet the gauge is full at the start.
- How the controls work comes from the original's `README.TXT`, "Manual
  Errata a) Death Star Tactical Controls":
  1. Click the fire button. It turns red and the cursor becomes a target.
  2. Right-click an enemy capital ship.
  3. Unpaused, the laser destroys it.
  4. Left-click the display to cancel before choosing a target.

  Fighters and the planet cannot be targeted. The README calls the image
  just "a Death Star image (to tell you what it's for)". Clicking or
  double-clicking it does nothing, the status panel's next arrow cycles only
  the two capitals, and no Death Star sprite is visible in the 3D view. So
  what TAC-03-C020 "Death Star selection" shows is still open.
- Once the target is right-clicked (still paused), the gauge turns blue
  (empty) and the button grey: TAC-05-C018, loading (`799ea931…`). The red, armed frame
  is kept unregistered as `s5-laser-armed` (`cdf214a0…`).
- About two minutes of unpaused battle after the shot, the gauge has partly
  refilled (green over blue): TAC-05-C020 (`483d35d2…`).
- TAC-05-C019, laser fired (`0e5ae202…`). The original draws no beam: in a
  60-frame, 200 ms burst, the target catches fire about 5 s after the unpause
  (frame 27) and explodes in frame 28, which is the registered frame. By
  frame 30 it is gone.

  Two earlier bursts missed. The unpause was sent from the host, and the
  request and the round trip each take several seconds. The first attempt
  was registered with the aftermath and withdrawn.

  The unpause that worked is clicked in the guest right after frame 0
  (`ClickOnStart=1`), with the cursor parked on the pause control. The
  parked cursor's hover highlight on that control is visible from frame 27.
- The A0 count is 102 of 106. Still open: TAC-03-C020, and the Alliance
  cells (C009, C021, C023).
- Side-swap trial: `ds-rebels.sav` is `ds-pre-alert` with only the slot name
  (`ds as rebels`, same length) and header word 5 (2 → 1) changed
  (SHA-256 `46af7f0a…`, local only).
  - It loads as the Alliance: the menu shows the Alliance emblem, and the
    game opens with the Alliance cockpit and droids.
  - The same alerts come, now in the Alliance's red chrome: Carida on day 669,
    then Battle at Coruscant on day 676. The Empire AI kept the Death Star
    fleet on its course.
  - Take Command, though, opened the battle from the Empire's side: the
    Imperial emblem and the owner-only Death Star strip (full gauge, green
    fire button), with fleet tabs 1 and 2.
  - So the tactical player side is not header word 5. Find where the tactical
    setup reads the player side before editing further.
  - Re-saving in-game as the Alliance (`ds-rebels-resave`, `29e45f02…`)
    does not fix the battle side.
    - The game rewrote header word 4 (3 → 1). That word is not the side: it
      is 1 or 6 in the Alliance saves and 2 to 4 in the Empire saves.
    - Reloaded after a fresh launch, the save gives the same Alliance-red
      alerts on days 669 and 676. Take Command still opens the battle as the
      Empire.
    - A byte diff is unusable: the longer slot name shifts every
      offset-valued record key.
  - The tactical side comes from the current player controller's `+0x24`
    (`FUN_0040b0e0` → `FUN_0041cdb0` → manager `+0x0c` → `FUN_0040d660`),
    not from the header.
    - The controller section (`FUN_00435db0`, written by `FUN_00435ec0`) is
      six manager words, then two slots, each a class tag followed by its
      payload. The payload's first u32 is the side (`FUN_0048a7c0`).
    - Only tag 1, the player class, sets manager `+0x0c` on load
      (`FUN_00436690`); tags 2 and 3 are AI. No separate current-slot field
      exists. A new Alliance game puts the player in slot 0 (side 1), an
      Empire game in slot 1 (side 2) (`FUN_00435bc0`).
    - `ds-pre-alert`: slot 0 is tag 3, side 1 at `0xD963C`; slot 1 is tag 1,
      side 2 at `0xFDB68`. The earlier `1, side, 1, 0, N, N-1` hits
      (`0xD9694`, `0xF1522`) lie inside the slot-0 AI payload, so the
      `ds-rebels2` edit (`0xF1526`, `3a1dd466…`) changed the wrong record and
      the battle stayed Empire.
    - Setting the player's side to 1 (`ds-rebels3`, `0xFDB6C` 2 → 1,
      `0b19ae4f…`) crashes the game to the desktop on load. Also moving the
      AI to side 2 (`ds-rebels4`, `0xD9640` 1 → 2, `a05c8eb5…`) crashes the
      same way. Each payload carries its own faction's state, so a side swap
      by scalar edits is a hard limit; the route is dropped.
- TAC-03-C020: zoomed out until the whole battle cube showed, and rotated, the
  Empire's battle draws no Death Star object. Only the two capitals and the
  Alliance group appear, so the owner sees the Death Star only as the strip
  above the panel.
  - The recovered code renders manager sprite 5010/5020 at a position of its
    own (`ghidra/notes/tactical-death-star-path.md`).
  - Inference: that sprite, and so a selectable Death Star, appears only on
    the opponent's side. C020 then needs the same Alliance-side battle as
    C009, C021 and C023.
- The Alliance cells need a battle the Alliance player owns. The side-swap
  copy of `ds-pre-alert` failed (above), so the remaining routes are a
  native Alliance game that reaches an Imperial Death Star, or an Alliance
  save with a Death Star staged through the Empire's own production.
  - Attack Death Star is enabled only when all of these hold (`FUN_005df7a0`,
    `FUN_005ad930`, `FUN_005ba450`): a fighter group is selected, the UI side
    is the Alliance, the Death Star is attackable and unshielded, no trench
    run is active, no order 6 is pending, and strictly more Alliance fighters
    than Imperial ones are in the battle. The manual's fighter rule holds.
- The four Death Star cells are shelved until a native Alliance game reaches
  an Imperial Death Star.

### Session 6 run (2026-10-13): 110 of 114

All eight `TAC-08` cells are captured. Every cell in the A0 run, with what
each frame shows, is listed in the
[capture index](../qa/2026-09-10-interface-parity-audit/evidence/2026-10-13-tactical-a0-capture-index.md).

- **Success (C003, C004, C007, C008).** Source: `empire-day53-yaga-orbit`.
  Fleet 1 carries three Stormtrooper Regiments into orbit of Alliance-held,
  ungarrisoned Yaga Minor. Planetary Assault takes the system at once and the
  Assault Summary opens.
- **Failure (C005, C006).** Source: `empire-day128-assault-fails-selonia`.
  One regiment attacks Selonia, which holds two Alliance Army Regiments. The
  assault fails and destroys one defender.
- **Order and recall (C001, C002).** The same save gives the fleet menu with
  Planetary Assault highlighted, and the summary reopened from the Message
  Index.
- **What the original showed.** An assault never costs the attacker
  regiments, and the Empire cannot see an Alliance garrison: the planet's
  Status entry is greyed out. Moving the troops off a captured system hands
  it straight back to the Alliance.
- **Assault Summary layout.** The buttons sit at x=720:
  - Close, y=115;
  - Summary, y=200;
  - Imperial Forces, y=265;
  - System or Alliance Forces, y=345;
  - Go to System, y≈425.
- **Save library.** Every save in `tools/original-capture/saves/` is also on
  the VM in `C:\Rebellion\SaveLibrary\`, with an `INDEX.txt`
  (`tools/original-capture/saves/INDEX.md`, "The library on the VM").
- **Still open:** TAC-03-C020, TAC-05-C009, C021 and C023.

### Session 7 (2026-10-13): an Alliance battle against the Death Star

- **Source.** David Adams (author of rebellion2) gave us an Alliance save,
  "Death Star at Coruscant", on day 1119
  (`alliance-deathstar-at-coruscant.sav`). Death Star 1 sits in Imperial
  Fleet 16 at Coruscant.
- **Staged in play, unedited:**
  - Fleets 2 (30 X-wing squadrons) and 7 (29 squadrons) met at Averam and
    were merged into Fleet 2: 26 ships, 59 squadrons.
  - Fleet 2 was then sent to Coruscant.
  - The saved result is `alliance-day1169-deathstar-trench-run.sav`.
- **The alert.** On day 1173, Battle at Coruscant opens against Fleets 16
  and 193, with 23–24 TIE squadrons. 59 against 23–24 satisfies Attack Death
  Star's fighter majority (`FUN_005ad930`).
- **Squadron size can be edited** (recipe 4 in
  `agent_docs/original-save-editing.md`). This route did not need it.

**Result: 113 of 114 captured, and C020 settled without a frame.** Every
cell is in the
[capture index](../qa/2026-09-10-interface-parity-audit/evidence/2026-10-13-tactical-a0-capture-index.md).

- **Route.** On a fresh launch, Take Command is enabled. Click the Red
  fighter-group slot (game 360,10), giving Red Group with 23 X-wings. Open the
  right-hand detail button (590,248) to show the Missions panel.
  - The fighter Missions panel holds four buttons: Attack Capital (top left),
    Attack Fighters (top right), Escort (bottom left) and Attack Death Star
    (bottom right, game 590,199).
- **C021, enabled.** Attack Death Star is drawn solid. The disabled
  capital-ship panel in TAC-05-C022 draws it hollow.
- **C009, chosen.** The display shows the Death Star in a yellow target
  bracket, with an X-wing on a red attack line. The button turns magenta.
- **C023, the trench-run launch.** The check mark (game 562,242) commits the
  order. Attack Death Star then greys and the Red slot greys. Three runs agree
  on the timing:
  - At the commit, the original plays only Red Group's fighter-mission
    acknowledgement: voice 14097, Alliance event 0x80, matched at 0.98.
  - At the unpause it plays 15160 (matched at 0.63 over the battle sound).
  - About 61 s after the unpause, 15161 marks the launch (0.96–0.98).
  - Chatter (15133, 15138 …) follows about every 12 s, which is 10% of the
    120 s timer.
  - About 120 s after 15161, the trench-run film plays (about 40 s), and then
    play returns to the tactical view.
  - Nothing changes on screen at the launch. Red Group's X-wings never appear
    in the 3D view.
  - The cell is a 60-frame, 500 ms burst across the launch, with the committed
    paused frame as `before`. Frame 40 is the cell frame. It falls within
    about 3 s of 15161: the guest and host clocks agree to a few seconds. The
    recording is attached as `original.wav`.
  - Runs 1 and 2, which missed the launch, are kept in
    `.artifacts/original-checks/withdrawn/TAC-05-C023-run{1,2}/`.
- **C020 has no separate original state.** The original draws no selectable
  Death Star in this Alliance battle or in the Empire's (session 5):
  - none in the 3D view at any zoom, rotation or highlight;
  - none among the eight capitals that the panel arrows cycle;
  - clicking the Death Star in the Missions display does nothing.
  - Its only selected form is the C009 target bracket. The index and the C6
    compare spec mark it `settled` (Tom's decision).
- **A1 defects found.** The port treats commit as launch:
  - It starts the trench-run timer at the commit and queues 0x139 and 0x13a
    there (`queue_selected_command_voice` returns early for Attack Death Star).
    The original plays the ordinary fighter-mission acknowledgement at the
    commit and launches about 61 s later.
  - The port also renders a selectable 5010 Death Star at the far face's
    centre, which neither side's battle in the original shows.
  - Both belong to the C6 backlog, not to this capture.
- **VM note.** "Timed out waiting for RPC" from the guest agent ended one
  run's `a0-game` partway. `vm-game.sh recover a0` fixed it.

### Session 8 (2026-10-14): the first C6 verdict, and the panel against A0

- **Compare tool.** `tools/interface-parity/tactical-a0-compare.mjs` and its
  spec `scenarios/tactical-a0-compare.json` compare each mapped A1 frame with
  its A0 cell by region (SSIM, normalized MAE, changed-pixel ratio, edge IoU,
  displacement). A region whose A1 scenario does not reproduce the A0 save
  is `not-run`, never a pass. The `settled` status carries TAC-03-C020.
- **Yaga fixture (step B).** Fixture scenario 37 rebuilds the day-30 Battle
  at Yaga Minor unit for unit; four journeys replay the A0 traces, and 67
  cells map onto them with `state_matches`.
- **Fixed against A0 and Ghidra** (each verified in the running game and
  the battle runner):
  - The right-hand container sits at `(482,28)`, not `(481,27)`
    (`FUN_0059cac0`); every panel coordinate moved by one pixel.
  - Hull and shield meters: blue fill over red, hull on the left
    (`FUN_005e4110`, `FUN_005e4b40`); a red meter is empty.
  - The four panel labels, their font size and the ship art position
    (`FUN_005e4880`, `FUN_005e4b40`).
  - Recover art was swapped between the sides (`FUN_005df110`).
  - Maneuvers and Missions were wired to the wrong buttons (`FUN_005e4110`).
  - Capital ships carry their numbered names, "Imperial Star Destroyer 1"
    (`FUN_00517760`; save v38).
  - Ghidra notes: `ghidra/notes/tactical-selected-ship-panel.md`,
    `fleet-names.md`, `tactical-command-assignment.md`.
- **Result.** Battle runner 80 of 80 (run
  `2026-10-14T15-49-45-646Z-8570`). C6 on that run: pass 11, fail 66,
  not-run 28, settled 1; negative controls 6 of 6.
- **Where the failures are.** The Results window is the only failing
  region in 10 cells; the control cluster fails in 19, the space view in 23,
  the status panel in 16, Battle Options in 9. 28 cells cannot run until an
  A1 fixture reproduces their A0 save (step D). Each is a node in
  `.beads/issues.jsonl`, with its cells.

- Automate the owned original English installation in the existing Windows VM.
- Use original saves, normal inputs, and, where necessary, editor-assisted save
  setup only to reach states. The displayed frame must always come from the
  unmodified original executable.
- Capture held, selected, disabled, keyboard, edge-probe, camera, damage,
  command, Death Star, options, results, return, audio, and media states named by
  the 106 ledger cells.
- Normalize only by lossless client-area crop, or by dropping the verified
  repeated lines of the point-filtered 800×600 frame (owner's decision,
  2026-10-11, confirmed 2026-10-12; above). Never resize, sharpen, recolor, interpolate, or
  otherwise reconstruct an A0 image.
- Hash every artifact and validate complete provenance before comparison.

Gate: all 106 rows have an authoritative source. Any state that cannot be
captured remains `reference-needed` and blocks the completion push.

### C3. Close `TAC-01` through `TAC-05` production gaps

- Correct exact planet placement, tactical-camera framing, global RNG order,
  original arrival callbacks, recovery trajectories, and power allocation.
- Complete Battle Alert entry, faction music/voice routing, and return-to-map
  behavior through the shared production flow.
- Recover and implement the native Death Star beam contract and validate the
  Alliance Death Star framing finding.
- Fill any control, hover, pressed, disabled, keyboard, edge-probe, navigation,
  maneuver, tactic, mission, target, damage, and destroyed-state gaps exposed by
  A0 comparison.

Gate: native tests and packaged-WASM journeys agree on state and action traces;
no fixture-only mutation is credited as functioning behavior.

### C4. Close `TAC-06` and `TAC-07` outcomes

- Restore the authentic Game Options destination used from Battle Options.
- Complete withdraw, simulate, observe/take-command, close, and confirmation
  paths with their native disabled and repeat-rejection behavior.
- Preserve the P58-B21 shared bombardment, landing, ground follow-up,
  occupation, and destination route while completing exact reports,
  exceptional navigation, and save-state fingerprints for every outcome.
- Prove successful and failed trench-run playback, skip/natural completion,
  return routing, persistent Death Star outcome, and victory separation.
- Complete Battle Results overview, both force tabs, damaged/destroyed rows,
  and Go to Fleet/Go to System destinations for every required outcome.

Gate: both factions, victory/loss/draw/withdraw, partial and total losses,
Death Star outcomes, and return destinations preserve exact strategic state.

### C5. Map deterministic A1 journeys to all 106 cells

Status: deterministic A1 mapping complete at P58-B15. The 36 scenarios map all
106 cells: 87 to browser journeys and 19 to deterministic snapshots. A mapped
cell is runnable, not accepted; A0 coverage, comparison, and strict acceptance
remain 0/106, and the strict gate has not been run.

P58-B16 deepens the mapped audio journey without changing the denominator: it
corrects event `0x14` to torpedo impact, routes all eight recovered weapon
events and all 22 WAVE variants, and passes the focused four-case muted browser
gate. Exact original RNG sequencing and audible native comparison remain C6
evidence work.

P58-B19 deepens the existing production group journey without changing the
denominator. It selects two capitals from different task forces, proves the
source rejection leaves their orders unchanged, and verifies exact muted
Alliance and Imperial voice routes. Strict A0 acceptance remains 0/106.

P58-B20 deepens the command-assignment, command-progression, and withdrawal
journeys without changing the denominator. It verifies exact faction/group
capacity and completion feedback, final-group completion gating, and the
disabled-hyperdrive withdrawal warning in twelve fresh muted browser cases.
Strict A0 acceptance remains 0/106.

P58-B21 establishes the common production return route without changing the
denominator. Played and automatic battles now share bombardment, troop landing,
contested ground continuation, unopposed occupation, and Battle Results
destination routing. Strict A0 acceptance remains 0/106.

P58-B22 closes the bounded practical implementation without changing the
denominator. It adds the recovered tactical random stream, power nibbles,
completion/destruction callbacks, and timed trench-run outcome/chatter path.
Whole-process RNG continuity, strategic commander binding, native visual and
audible comparison, and all strict A0 verdicts remain C6 evidence work.

- Give every catalog scenario one or more exact `audit_cells` values.
- Distinguish journeys from snapshots in the schema and report.
- Drive real pointer and keyboard input for every interactive row, including
  four one-pixel inside edges, four one-pixel outside edges, and corners where
  required by the audit contract.
- Capture canonical 640×480 output. Keep 1280×800 letterboxed runs as regression
  coverage rather than additional parity cells.
- Record native/WASM fingerprints, resource hits, action traces, screenshots,
  audio/media events, browser requests, diagnostics, and cleanup.

Gate: the runner reports `matrix_coverage: 106/106`; every row can execute
independently from a clean, muted browser process, except scenarios whose
purpose is explicit audio verification.

### C6. Compare, classify, and render evidence

- Compare each A1 capture with its A0 counterpart using the audit's existing
  SSIM, channel-delta, displacement, unauthorized-pixel, edge-IoU, and geometry
  thresholds.
- Use narrow, named masks only for proven nondeterministic original regions.
  Store every mask justification and never mask an authentic control, label,
  ship, effect, or outcome.
- Produce per-cell JSON plus an aggregate human-readable report with
  `pass`, `fail`, `reference-needed`, and `not-run` states.
- Generate the repeated Markdown status summaries from the canonical JSON.

Gate: deliberate one-pixel geometry, wrong-bitmap, wrong-action, missing-audio,
and stale-state negative controls all fail the correct row.

Status (session 8): built and run. The negative controls fail their rows,
6 of 6; the verdict is pass 11, fail 66, not-run 28, settled 1. The fixes
are tracked in the task graph, not by tuning thresholds.

### C7. Final acceptance and documentation

- Run the complete workspace suite, scoped lint/format gates, production and
  fixture WASM builds, production-fixture exclusion, and all 106 cells.
- Run both factions and required outcome variants with clean four-request
  startup and complete process cleanup.
- Have an independent browser-use reviewer inspect the aggregate evidence and
  selected full-resolution journeys.
- Update `progress.json`, both audits, the surface ledger, this plan, the
  launcher reference, evidence indexes, and roadmap from the same canonical
  results.
- Mark only passing cells accepted. Do not round, extrapolate, or convert
  implementation coverage into parity acceptance.

Gate: `matrix_coverage: 106/106`, `parity_acceptance: 106/106`, no P0/P1
findings, no missing authoritative evidence, and no open tactical release gate.

## Final command contract

The implementation will expose one documented orchestration command that:

1. validates the local A0 manifest and exact 106-cell denominator;
2. builds production and fixture artifacts;
3. runs native functional/state gates;
4. launches one fresh muted browser per A1 scenario;
5. captures and compares A0/A1 evidence;
6. closes every browser and local server;
7. emits a redacted aggregate JSON and Markdown report; and
8. exits nonzero unless the requested gate passes.

It will support a coverage mode for development and a strict mode for release.
Strict mode requires both `106/106 runnable` and `106/106 accepted`.

## Push rule

The next tactical push may contain C1 through C7 as separate local commits.
Push them together only after the strict command passes, documentation matches
the generated result, the independent review has no P0/P1 finding, and
`HEAD` contains no original-game media or fixture path in production artifacts.

If any original state cannot be captured or any parity row fails, retain the
local commits, leave that cell open, and report the exact blocker. Do not call
the launcher 100% complete and do not push a fabricated 106/106 result.

## Definition of done

- The exact denominator is generated from the authoritative ledger.
- Every one of the 106 rows is independently runnable and reproducible.
- Every cell has authoritative original evidence with verified provenance.
- All visual, geometry, interaction, functional, audio/media, and diagnostic
  gates pass.
- Campaign and standalone entry use the same production renderer and return
  path.
- Test-only fixtures are absent from production HTML and WASM.
- No proprietary original-game artifact is committed.
- The audits and roadmap report 106/106 from generated evidence.
- The complete local commit train is pushed to `main` as one verified batch.
