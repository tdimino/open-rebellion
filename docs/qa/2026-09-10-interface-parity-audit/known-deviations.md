# Known Interface Deviations

This is the central register for deliberate or currently tolerated departures
from the original *Star Wars: Rebellion / Supremacy* interface. It is not a
waiver. Only rows marked **Approved extension** may remain outside the parity
denominator, and only while they cannot replace, obscure, overlap, or change an
original path.

**Temporary** and **Unapproved** rows keep every affected original cell open.
Missing work is not a deviation and belongs in the
[surface ledger](2026-10-06-surface-ledger.json).

| ID | State | Departure | Reason and boundary | Affected cells |
|---|---|---|---|---|
| `DEV-UI-001` | Approved extension | The shuttle includes a vector-drawn, music-only mute control that the original did not have. | Player-requested convenience. It occupies a separate top-right region, has identical paint/hit bounds, preserves SFX, and is excluded as `EXT-02-C001`. | `EXT-02-C001`; does not satisfy `PRE-02` |
| `DEV-UI-002` | Approved extension | Deterministic fixture routes expose rare interface states to the browser harness. | Test-only. Fixtures cannot appear in production navigation or count as original UI. | Test infrastructure only |
| `DEV-UI-003` | Temporary | Fleet and Ship Finder lists use wheel scrolling without the original scrollbar. | Bounded implementation shortcut recorded in `ghidra/notes/fleet-finder.md`; original composition and A0 proof remain required. | `OBJ-04-C001`, `OBJ-04-C002`, `OBJ-04-C004` |
| `DEV-UI-004` | Temporary | Fleet Finder uses a top Tooltip layer, centered placement, and different keyboard ownership to remain usable above modeless windows. | Prevents occlusion in the current egui stack. It does not prove original modality, placement, or key handling. | `CMD-04-C017`, `OBJ-04-C001`, `OBJ-04-C002` |
| `DEV-UI-007` | Temporary | Create Fleet allocates a new fleet dynamically instead of consuming an original spare-fleet object. | The port has no spare-fleet pool. The player-facing split route works, but exact identity and downstream presentation still require proof. | `OBJ-05-C011`, `OBJ-14-C003` |
| `DEV-UI-008` | Temporary | Per-area Destination state, command `0x214`, and child-window targeting now work, but the visible manufacturing panel and its shipyard combo remain replacements for the original Manufacturing and Build Selection windows. | The state and order path no longer block the cells by themselves. Authentic type-9 composition, producer selection, resource-backed controls, and exact targeting geometry remain required. | `OBJ-07-C001`–`OBJ-07-C014`; visual and geometry acceptance for `OBJ-07-C015`–`OBJ-07-C020` remains open |
| `DEV-UI-009` | Temporary | Visible Officers, Research, Jedi, Loyalty, Bombardment, Death Star, and live-ground-combat dashboards reorganize original actions. | Transitional legacy surfaces. They are required failures under `EXT-01`, not approved extensions. | `EXT-01-C001`–`EXT-01-C007` |
| `DEV-UI-010` | Planned extension | Enhanced widescreen will anchor original panels while extending only galaxy, system, and tactical viewports. | It may begin only after the 640×480 path is accepted. Fixed 4:3 video and other original surfaces remain pillarboxed with separate baselines. | Separate future extension; cannot satisfy any original cell |
| `DEV-UI-011` | Approved extension | The shuttle includes a vector-drawn Fleet Registry chip and readout that choose canonical fleet names, a naming the original never offers. | Approved by the maintainer on 2026-10-06. It sits left of the music control, outside every original hotspot, holds the cockpit beneath it still while open, and defaults to the original "Fleet N" numbering (`FUN_00517760`); see `docs/mechanics/fleet-names.md`. | `EXT-02-C006`; does not satisfy `PRE-02` |
| `DEV-UI-012` | Temporary | Escape closes the Message Index, and the window is centered like the Finder windows. | This keeps the bounded window operable while native Escape handling and constructor placement remain unresolved. | `CMD-04-C020`, `CMD-08-C003`, `CMD-08-C017`–`CMD-08-C020` |
| `DEV-UI-013` | Temporary | Build Ships, Build Troops, Build Facilities, Galaxy Overview, Objectives, Translate Counterpart, and Messages are disabled in the Agent menu. | Only Manage Garrisons, Manage Production, and Agent Advice are currently routed. The disabled entries must be restored with their original enable predicates and destinations. | `CMD-07-C009`; downstream `CMD-06`, `CMD-08`, and `OBJ-01` routes remain open |
| `DEV-UI-014` | Temporary | Each enabled player-agent module executes one complete cycle per game day. | The original agent uses a resumable state `4/5/6` cycle and order cap. The daily approximation preserves determinism but is not yet an exact cadence port. | `CMD-07-C009`, `CMD-08-C023` |
| `DEV-UI-015` | Temporary | An in-place Rename edit is discarded on Escape or an outside click. | `FUN_004aca40` supports cancellation, but exact CoolStringField focus-loss and outside-click behavior remain unresolved. | `OBJ-14-C019`, `OBJ-14-C020` |
| `DEV-UI-016` | Unapproved | The current detailed System window exposes Personnel, Fleets, Defenses, and Troops tabs. | Native window type 9 builds facility pages only in `FUN_004568a0`; the invented tabs must not remain in parity mode. | `OBJ-02-C004`–`OBJ-02-C009`, `OBJ-07` |
| `DEV-UI-017` | Retired 2026-10-06 | Personnel Finder sent every character at a system to Defenses. | Display now follows `FUN_00429440` (`personnel_finder::character_target`): a fleet parent opens the Fleet window, a visible mission the Missions window (kind 11) with the mission's row and the member's tab (`FUN_004a1e10`), anything else Defenses (kind 10). Unit-tested; the Missions route has no browser case yet. | `OBJ-03-C016` |
| `DEV-UI-018` | Temporary | The Status window (`0x103`, type `0x1a`) is ported for characters only, and its Character Status page differs in five details. | Commanding always reads None (no command assignment, `+0xa8`); Injured never shows (no injury, `+0x94`); the list scrolls with the wheel and draws no scroll bar; Escape closes it with no traced key slot; its Encyclopedia button opens the index, as the menu's Encyclopedia does. The other families keep Status disabled. `ghidra/notes/status-window.md`, `status-window.mjs`. | `OBJ-03-C014`, `OBJ-06`, `OBJ-09`, `OBJ-10` status cells |

## Change rule

Add a row before merging any intentional interface difference. Include the
decision state, reason, affected acceptance cells, and evidence. Moving a row
to **Approved extension** requires explicit maintainer approval and an excluded
`EXT-*` cell when it is visible to players. Removing a deviation requires the
same implementation and evidence updates that close its affected cells.

This register was seeded through the
[manual window cross-check](2026-10-06-manual-window-checklists.md)
and existing `port:` notes. Faction Wars' append-only backport practice
informed the format:
[Faction Wars (TeeJS), backport template](https://github.com/TeeJS/faction-wars/blob/626b9b2bd62e7a83a9938568321c59cf64616a19/docs/BACKPORT-LOG.md#L1-L25).
The entries above are independently stated Open Rebellion decisions; no source
text or code was copied.
