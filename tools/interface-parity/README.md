# Interface parity browser harness

This test-only harness exercises the original Galactic Information Display (GID)
menu and 38 deterministic states, for both factions at 640×480 and a letterboxed
1280×800 viewport. It launches a new pinned Chrome for Testing process for
each case with audio muted. The fixture bridge exists only in the separate
`interface-test-fixtures` WASM build. Production HTML and WASM are checked for
fixture imports and markers.

From this directory, install the pinned Node packages with `npm ci`, then run:

```sh
node run.mjs --smoke        # build both WASM variants; four quick cases
node run.mjs --all          # rebuild; all 152 faction/viewport cases
node run.mjs --all --no-build  # reuse a verified fixture build
node run.mjs --all --scenario=pan --no-build  # four focused faction/viewport cases
node encyclopedia-art.mjs  # build and verify original EDATA browser transport
node encyclopedia-art.mjs --no-build  # reuse a verified fixture build
node message-index-shell.mjs  # verify both original Message Index shells
node message-index-shell.mjs --no-build  # reuse a verified fixture build
node encyclopedia-index-shell.mjs  # verify both original Encyclopedia index shells
node encyclopedia-index-shell.mjs --catalog  # verify source-derived index content
node encyclopedia-index-shell.mjs --no-build  # reuse a verified fixture build
node mission-dialog.mjs  # verify the original mission dialog's chrome on both pages
node mission-dialog.mjs --no-build  # reuse a verified fixture build
node fleet-move.mjs  # verify fleet Move, Confirmed Move and the system window drag
node fleet-window.mjs  # verify the Fleet window, regiments, joining and splitting, rename and destination
node sector-quadrants.mjs  # verify the quadrant icons and the Defenses and Missions windows
node fleet-finder.mjs  # verify the Fleet and Ship Finder
node fleet-finder.mjs --no-build --only=alliance/chrome  # one faction/case, reusing a build
node fleet-registry.mjs  # verify the Fleet Registry, a port extension, from the main menu
node audit-batch.mjs  # verify the Troop/Personnel Finders, Message Index, Alt keys and Agent menu
node status-window.mjs  # verify a character's Status window from its pop-up menu
```

Harness unit regressions can be run from the repository root with
`make test-interface-harness`, or here with `npm run test:unit`.

The encyclopedia-artwork gate is deliberately narrower than `OBJ-01`. Its
test-only fixture paints one owned `EDATA.042` source image at native 400×200
size, then checks every displayed pixel, the four-request startup, muted fresh
process, resource hashes, and browser diagnostics. It neither opens nor
accepts the replacement encyclopedia window. Set `REBELLION_EDATA_DIR` or
`REBELLION_GAME_DIR` when the owned installation is not adjacent to the
repository. Captures and decoded comparisons remain under ignored
`.artifacts/interface-parity/` storage.

The Message Index-shell gate is a bounded step toward `CMD-08`. It
composes the source-recovered 470×331 faction shell, right rail, index content,
and ten category controls. Two fresh muted browser processes compare the normal
shell and every held control state pixel-for-pixel against the owned STRATEGY
resources, then probe the exclusive right edge of the first control. Populated
message rows, selection, navigation, clear/delete actions, Advice slowdown,
chat, and production routing remain open. The authentic Encyclopedia is a
separate `OBJ-01` surface and command `0x131` continues to fail closed.

The Encyclopedia index-shell gate is a bounded step toward `OBJ-01`. It
composes the source-recovered 470x330 faction shell, index content, seven
category controls, and right rail. By default, two fresh muted browser
processes compare
category selection, held rail controls, native clipping, exclusive-edge and
transparent-pixel rejection, press capture, drag cancellation, and modal
click-through blocking pixel-for-pixel against the owned STRATEGY resources.
`--catalog` runs P65's source-derived 356-entry index, seven TEXTSTRA category
labels and filters, scrolling, row selection, and modal-block checks. P65
compares every static shell pixel while treating rendered text/list rectangles
as dynamic; those regions must be visibly populated but remain outside exact
A0 acceptance. Topic pages, text/art bindings, navigation, close routing, and
production routing remain open.

The mission-dialog gate is a bounded step toward `F-019`. Four fresh muted
browser processes open the original mission dialog on its Mission and
Agents pages for both factions, then click the Agents tab once. Each capture
compares the panel, title bars, close box, tabs, headers, arrows, and buttons
pixel-for-pixel against the owned STRATEGY resources. The bottom buttons are
checked cropped to their 64-pixel controls. Text, the mission item, the target
art, and the member lists are masked and kept as captures.

The fleet gates are steps toward `F-007C`. Each opens its windows only through
the original entries, on both sides, from a fresh muted browser per case, and
drives a fixture scenario (`interface_test_fixture.rs`) whose observations it
asserts. `fleet-move.mjs` covers the fleet pop-up menu, targeting, the
Confirmed Move window and the system window drag. `fleet-window.mjs` opens the
Fleet window from the sector window's fleet icon, compares its chrome against
STRATEGY.DLL, and covers loading and holding regiments, landing, unloading by
hand, a regiment travelling on its own, joining and splitting fleets, Rename
(`0x203`: an emptied name keeps the edit open, Enter submits the typed one)
and the facility icon's Destination (`0x214`, fixture code 54: the release on
a planet sets it). `sector-quadrants.mjs` covers the quadrant icons and the System, System
Defenses and Missions windows they open. `fleet-finder.mjs` opens the Finder
from the cockpit control and F3, compares its chrome in both modes, and opens a
chosen fleet or ship in its Fleet window; its tabs case checks that the map
under the Finder takes no click. The map never zooms or pans
(`FUN_00422ce0`), and the `pan` and `zoom` scenarios check that a right-drag
and a wheel leave the frame unchanged. `fleet-registry.mjs` drives the real
main menu (no fixture): it opens the Fleet Registry from its chip, lights a
naming mode, holds a hovered name's row to show its note, closes on Escape
with the chip's lamp lit, and starts a game whose fleets take canonical
names. `audit-batch.mjs` opens the Troop and Personnel Finders with F4 and
F5 and compares their rails against STRATEGY.DLL, opens the Message Index
with F6 and a rail light once each (a second F6 while open does nothing,
Close closes it), selects a GID mode with Alt+digit, toggles Manage
Garrisons with Alt+G, and opens the Agent menu from the droid.
`status-window.mjs` right-clicks the agent in a system window, chooses
Status, and compares the 379 by 272 window's STRATEGY background and buttons
exactly, with the title, list, name and keyed portrait masked and checked for
content; Close, Escape and the Encyclopedia button each close it.
`--only=<faction>/<case>` runs named cases. Each gate writes `result.json` and its captures under
`.artifacts/interface-parity/<gate>-<timestamp>/`; the native GUI checks of the
same journeys are recorded separately in the audit.

The tactical acceptance denominator is generated directly from the surface
ledger rather than maintained as a second hand-written list:

```sh
node validate-tactical-matrix.mjs
node validate-tactical-matrix.mjs --rows
node validate-tactical-matrix.mjs \
  --a0-manifest=.artifacts/interface-parity/a0/manifest.json --strict
```

The first command reports coverage totals, while `--rows` emits every stable
cell from `TAC-01` through `TAC-07`. Strict mode requires exactly 106 unique
catalog mappings and 106 provenance-complete A0 capture records. Add
`--require-accepted` only at the final release gate; it fails until all 106
canonical ledger cells are marked passed. The manifest schema and safe empty
example are [`schemas/tactical-a0-manifest.schema.json`](schemas/tactical-a0-manifest.schema.json)
and [`tactical-a0-manifest.example.json`](tactical-a0-manifest.example.json).
Actual manifests and captures live under ignored
`.artifacts/interface-parity/a0/`; the exclusion check rejects any tracked
artifact from that store.

Catalog scenarios declare `execution_kind` as `journey`, `snapshot`, or
`negative-control`. The current crosswalk maps 106/106 cells: 87 journeys and
19 snapshots. Negative controls may not map cells. Complete deterministic A1
mapping does not convert any cell into original acceptance; A0 coverage and
strict parity acceptance remain 0/106.

Initialize the ignored local manifest once, then ingest each guest capture with
its JSON sidecar:

```sh
mkdir -p .artifacts/interface-parity/a0
cp tools/interface-parity/tactical-a0-manifest.example.json \
  .artifacts/interface-parity/a0/manifest.json
node tools/interface-parity/ingest-tactical-a0.mjs \
  --manifest=.artifacts/interface-parity/a0/manifest.json \
  --png=/path/to/TAC-01-C001.png \
  --metadata=/path/to/TAC-01-C001.json
```

Ingestion verifies the exact ledger requirement, audited executable hash,
unmodified source manifest, 640×480 sidecar and decoded PNG dimensions, capture
provenance, input trace, and SHA-256. It copies accepted bytes only to the
cell's ignored A0 directory and refuses a different replacement for an already
registered cell. It does not resize or otherwise transform original output.
GID actions register a fresh expected command event before input, then wait
through the following paint; the combined wait has a two-second deadline.
This prevents a stale log or an early screenshot from satisfying a new action.
Every case verifies command `0x132` on the corrected bottom GID control and
uses the same control to close and reopen the menu. The Popular Support cases
also probe both factions' physical `0x133` Game Options and `0x131`
Encyclopedia controls, their distinct held bitmaps, F1/F7, and the fail-closed
destination boundary. They also verify that the legacy `E` shortcut cannot
expose the replacement Encyclopedia. Held-state captures wait through two
animation frames.

Browser startup retains its 30-second deadline. A launch timeout gets one
fresh-process retry after a one-second pause; other launch errors fail
immediately. No page, application, or assertion failure is retried. Per-case
`launch_attempts` retain durations and errors, and run summaries report total
launches, timeouts, and recoveries. A recovered startup is visible in evidence
and does not establish original-game visual parity.

The runner expects the exact Chrome for Testing version in `browser.json`.
Set `OPEN_REBELLION_CHROME_FOR_TESTING` to its executable if it is installed
elsewhere. The build needs the local owned game data, the wasm32 Rust target,
and the tools used by `scripts/build-wasm.sh`. The runner serves only on
127.0.0.1. Screenshots, complete browser console/cache diagnostics, resource hashes, interaction probes,
four-request ledgers, and results go to the ignored
`.artifacts/interface-parity/<run-id>/` directory. Failed cases exit nonzero.

The generated test site also contains `battle.html`, a test-only launcher for the
real tactical scene. Its proof and negative-control links use the versioned codes in
[`tactical.catalog.json`](scenarios/tactical.catalog.json) and start muted. The
fixture proves deterministic production-path battle entry, the current
shell/control checkpoint, and fixed selection from one source-bound three-LOD
family, one live LOD journey, and the source camera/target/layout/participant
journey. Run
`node run.mjs --battle --no-build` for four smoke cases or
`node run.mjs --battle --all --no-build` for all 128 entry, negative-control,
fixed-LOD, live-LOD, camera, production-participant, fighter-detail, group,
impact-effect, projectile/field, selected-damage, subsystem/field-command,
live-subsystem-damage, subsystem-repair/mobility, maneuver/movement,
command-assignment, command-execution, command-progression, attack-targeting,
attack-target-lifecycle, Death Star presentation and superlaser, trench-run
success and failure routing, Battle Results, Battle Options and withdrawal,
faction, and viewport cases. The tactical probes cover pause stability,
faction highlights, zoom round trips, source-shaped control misses, held pressed
art, aperture isolation, typed resource diagnostics, runtime measurements, and
proof-on/off pixel differences. The camera journey also checks the recovered
active-force extent, four lane values, stable DAT and fleet-roster identities,
exact signed-zero X slots, and selected source-world target. Each scenario uses a fresh muted
browser. Source-bound 3D cases also assert the recovered Gouraud, dither,
filtering, culling, depth, specular, diffuse, and emissive render-state contract.
Fixture schema 29 additionally asserts representative capital-ship and fighter
DAT-to-tactical-resource joins, exact production rendering of two joined
capital-ship families, exact fighter close/far/indicator resource triplets,
the test-only fighter focus used by the detail journey, and a 3D-off control
with all mapped fallbacks suppressed. Per-object
framebuffer probes require changed pixels around every logged capital and
fighter projection. Production journeys deselect, reselect, and target through
the projected capital bounds. The fighter journey uses the original zoom
buttons and asserts all nine independent detail transitions without another
family load. The trench-run fixtures assert source result states 6 and 7 route
to MDATA.201 and MDATA.202 respectively, then return to the paused production
tactical session. The effect snapshot requires all six executable-selected hit,
damage, and destruction frame families, exact target attachment, draw size,
priority, transparency, and 0.1-second cadence. Every tactical run requires all 87 meshes and
397 textures in the four-request runtime pack. The projectile/field snapshot
requires all three retained projectile variants and material selectors, exact
source-to-target interpolation and lifecycle, visible selector-colored pixels,
both 128 by 128 field families, 10 Hz animation, and gravity-over-tractor
priority with tractor restoration.
The selected-damage snapshot checks panel 1302, the exact source-ordinal
capital portrait, lime-matte composition, live shield and hull fractions, and
faction-correct meter colors.
The subsystem/field-command snapshot checks all five source-quantized condition
families at their exact panel positions, exact tractor and gravity source IDs,
one-target tractor and four-target gravity capacity, gravity priority, and
visible-kind frame reset.
The live-subsystem-damage snapshot sends deterministic hits through the shared
production damage path and checks source-derived hit limits, all five hit
counters, condition percentages and resource bands, and tractor cancellation.
The repair/mobility snapshot checks the source timer, inclusive repair roll,
subsystem selection, engine condition, active tractor drag, bitmap bands, and
full 128-by-128 field containment inside the tactical aperture.
The maneuver/movement snapshot checks the recovered state-to-bonus producer,
effective-power velocity, signed faction direction, exact 250-millisecond
integration, and equality between integrated and rendered source positions.
The command-assignment journey opens both source panels, verifies their exact
normal, pressed, and disabled BMP states, cancels without mutation, commits
Hammer with Surround and Attack Capital Ships to selected capitals, and commits
Recover to a selected fighter group.
The command-execution snapshot records a recovered Left Hook waypoint and
normalized desired direction, verifies Hold clears movement intent, and enters
fighter Recover state 2 with a compatible same-faction carrier.
The command-progression snapshot verifies executable-derived capital turn rates,
normalized signed XZ rotation and snap, waypoint completion, the 2.0-unit
fighter docking gate, tactical removal, and preservation of strategic counts.
The attack-targeting snapshot checks both executable attack orders with both
capital and fighter owners, the typed first eligible hostile target, retained
live engagements, and the absence of a wrong-class fallback.
The attack-target-lifecycle snapshot invalidates the first hostile capital and
fighter entries, then verifies stable same-class replacement for capital and
fighter owners without cross-class or random fallback.
The navigation/camera journey verifies all four original navigation-set
controls, ordered routes and multiple targets, camera memorize/recall, and
task-force chase in both factions and viewports. Navigation-point coordinates
remain provisional until original-executable comparison.
The Death Star journeys verify the owner-gated 1021 through 1024 control,
resource 5030 as the sparse 440 by 438 tactical star surface, right-click
target assignment, a visible delayed beam, destruction, and the hostile-only
Attack Death Star mission state. The source uses the standard arrow cursor,
and exact result states 6 and 7 now route to trench-run films 201 and 202.
Exact beam dimensions and timing, native playback comparison, and lossless A0
comparison remain open.

Each logical case uses a fresh muted browser process. A Playwright startup,
ready-signal, or screenshot timeout may retry once in another fresh process;
the result retains both attempts and both cleanup records. Pixel, state,
request, console, and other product assertions never retry.
Tactical controls and images still have no lossless original-game baselines. See the
[standalone battle plan](../../docs/plans/2026-09-12-tooling-standalone-space-battle-launcher.md).

The PNG comparison directory contains *implementation regression* baselines,
not original-game truth. `--update-goldens` creates only missing implementation
baselines, never overwrites a mismatch, and requires an accepted, SHA-verified
original source in `baselines/accepted-original.json`. A passing browser run without
original lossless captures does not complete a strict interface-parity cell.
At native 640x480, every run also checks all nine Message Index rail icons
pixel-for-pixel against the corresponding original STRATEGY BMPs. This proves
resting-art identity only, not the unread states or Message Index interactions.
For native-size GID menu captures, the runner checks all 634 root-border pixels
against STRATEGY 10100 through 10107 when unobscured. With a foreground system
window, it first confirms the overlap, then checks every still-visible border
pixel and records the covered remainder separately. A root-row hover probe
also guards against reintroducing the unsupported synthetic color wash.
Menu interior, typography, and original geometry remain separate acceptance work.
The runner compares captured menu, hover, legend, and pan/zoom states as well as
the initial frame once each state has its own reviewed original reference and
implementation regression baseline. Until then their comparison is recorded as
`unbaselined`, not an assertion of visual fidelity.
See the [interface audit](../../docs/qa/2026-09-10-interface-parity-audit/README.md)
and the [execution sidecar](../../docs/plans/2026-09-11-tooling-interface-parity-acceleration-sidecar.md).
