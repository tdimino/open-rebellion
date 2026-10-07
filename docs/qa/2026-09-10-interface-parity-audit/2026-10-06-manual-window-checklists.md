# Manual Window Cross-Check (2026-10-06)

This checklist adds manual-facing behaviors that were missing or bundled too
broadly in the interface ledger. Each row has its own stable acceptance cell.
**Present**, **Partial**, **Missing**, and **Disabled** describe the current
port only. They do not accept a cell. Every listed cell remains pending until
it passes the audit's A0, native, browser, input, and transition gates.

The original manual and `REBEXE.EXE` remain authoritative. Faction Wars is a
research lead, not parity evidence. Manual cross-check research was informed by
[Faction Wars by TeeJS](https://github.com/TeeJS/faction-wars). Findings were
independently paraphrased and checked against the manual and `REBEXE.EXE`
traces; no Faction Wars code or documentation was copied.

## Status vocabulary

| Status | Meaning |
|---|---|
| Present | The named element and its observable interaction exist in the port. |
| Partial | Some required behavior exists, but the implementation materially differs or lacks a state. |
| Missing | No matching production behavior exists. |
| Disabled | Original control is shown but deliberately cannot act; its reason must be documented. |

Original-behavior claims additionally follow the
[manual cross-check workflow](../../../agent_docs/manual-cross-check.md):
**Confirmed** requires a matching executable trace, **Contradicted** records a
manual/executable disagreement, and **Unresolved** keeps implementation and
acceptance open.

## Setup and command-center input

| Cell | Original behavior | Port | Original proof |
|---|---|---|---|
| `PRE-02-C015` | New-game difficulty initially selects Easy. | Present | Manual p. 21; executable-default trace still needs a dedicated note. |
| `CMD-01-C007` | Alt+1 selects Popular Support. | Present | TEXTCOMM accelerator 11 and `FUN_00422ce0`; both-faction `audit-batch.mjs` gate. |
| `CMD-01-C008` | Alt+2 selects Uprisings. | Present | TEXTCOMM accelerator 11 and `FUN_00422ce0`; implementation in `8fc9d983`. |
| `CMD-01-C009` | Alt+3 selects Idle Fleets. | Present | TEXTCOMM accelerator 11 and `FUN_00422ce0`; implementation in `8fc9d983`. |
| `CMD-01-C010` | Alt+4 selects Fleets En Route. | Present | TEXTCOMM accelerator 11 and `FUN_00422ce0`; implementation in `8fc9d983`. |
| `CMD-01-C011` | Alt+5 selects Idle Personnel. | Present | TEXTCOMM accelerator 11 and `FUN_00422ce0`; implementation in `8fc9d983`. |
| `CMD-01-C012` | Alt+6 selects Active Personnel. | Present | TEXTCOMM accelerator 11 and `FUN_00422ce0`; implementation in `8fc9d983`. |
| `CMD-01-C013` | Alt+7 selects Idle Shipyards. | Present | TEXTCOMM accelerator 11 and `FUN_00422ce0`; implementation in `8fc9d983`. |
| `CMD-01-C014` | Alt+8 selects Idle Training Facilities. | Present | TEXTCOMM accelerator 11 and `FUN_00422ce0`; implementation in `8fc9d983`. |
| `CMD-01-C015` | Alt+9 selects Idle Construction Yards. | Present | TEXTCOMM accelerator 11 and `FUN_00422ce0`; implementation in `8fc9d983`. |
| `CMD-01-C016` | Alt+G toggles Manage Garrisons. | Present | `FUN_00439d60`; both-faction `audit-batch.mjs` gate. |
| `CMD-01-C017` | Alt+U toggles Manage Production. | Present | `FUN_00439d60`; implementation in `8fc9d983`. |
| `CMD-02-C028` | Right-drag leaves the original fixed galaxy map unchanged. | Present | Confirmed by `FUN_00422ce0`, `ghidra/notes/galaxy-view-input.md`, and the deterministic no-op gate. |
| `CMD-02-C029` | Mouse-wheel input leaves the original fixed galaxy map unchanged. | Present | Confirmed by `FUN_00422ce0`, `galaxy-view-input.md`, and the deterministic no-op gate. |

Faction Wars independently identifies the nine GID accelerators and two
management toggles in its manual digest:
[GID accelerators](https://github.com/TeeJS/faction-wars/blob/626b9b2bd62e7a83a9938568321c59cf64616a19/GAMEPLAY.md#L4067-L4079)
and
[management toggles](https://github.com/TeeJS/faction-wars/blob/626b9b2bd62e7a83a9938568321c59cf64616a19/GAMEPLAY.md#L4051-L4061).

## Sector icons and destination routing

The former `CMD-03-C012` phrase, “double-click object,” was not precise enough
to detect the earlier implementation that responded only to double-clicks.
The following child cells now make every visible and interactive part explicit.

| Cell | Observable behavior | Port | Original proof |
|---|---|---|---|
| `CMD-03-C014` | Each quadrant icon appears only when its original predicate is true. | Present | Confirmed by `ghidra/notes/sector-quadrants.md`. |
| `CMD-03-C015` | Facilities, fleets, defenses, and missions occupy their original quadrants and rectangles. | Present | Confirmed by `sector-quadrants.md` and `sector-window-hit-test.md`. |
| `CMD-03-C016` | Icon art reflects faction, intelligence, and state. | Partial | Base art and faction paths exist; the complete state matrix remains open. |
| `CMD-03-C017` | Left or right press selects the icon before another action. | Present | Confirmed by `FUN_004593e0`, `FUN_0045cc10`, and `FUN_0045b1b0`. |
| `CMD-03-C012` | A double-click on an icon opens its original destination. | Present | Confirmed by the `WM_LBUTTONDBLCLK` route in `sector-quadrants.md`. |
| `CMD-03-C018` | Right release opens the icon's original item menu. | Partial | Menus open, but several commands remain disabled or unimplemented. |
| `CMD-03-C019` | Dragging an icon begins the appropriate system-wide move. | Partial | Fleet-icon drag works; other icon families remain open. |
| `CMD-03-C020` | The destination window opens on the correct system, tab, and selected object without a duplicate. | Partial | Bounded window routes work; every object/state combination is not covered. |

The manual-derived behavior is also summarized in Faction Wars'
[sector-window reading](https://github.com/TeeJS/faction-wars/blob/626b9b2bd62e7a83a9938568321c59cf64616a19/GAMEPLAY.md#L400-L421).

## Window manager and modal catalogue

| Cell | Original behavior | Port | Original proof |
|---|---|---|---|
| `CMD-04-C009` | Sector windows stay at their fixed cockpit positions. | Present | Manual pp. 63–64; constructor/lifetime trace remains incomplete. |
| `CMD-04-C010` | Sector windows have no minimize action. | Present | Manual pp. 63–64; current sector window exposes only flip and close. |
| `CMD-04-C011` | At most two sector windows may be open. | Present | `FUN_00429ce0`; a third request replaces the window on its own galaxy half (`8fc9d983`). |
| `CMD-04-C012` | The side-flip control moves a sector window between the two original columns. | Present | Current fixed-column implementation; exact A0 state open. |
| `CMD-04-C013` | The reference rail holds twelve minimized windows. | Present | Confirmed by `FUN_00421c70` and `REFERENCE_RAIL_SLOTS`. |
| `CMD-04-C014` | A minimized entry retains the system name. | Present | Current rail renderer; original A0 comparison open. |
| `CMD-04-C015` | A minimized entry identifies its window type. | Present | Source-specific System, Fleet, Defenses, and Missions rail resources are routed. |
| `CMD-04-C016` | Status windows block underlying input. | Missing | Manual pp. 63–64; production Status family remains open. |
| `CMD-04-C017` | Finder windows block underlying input. | Partial | Fleet Finder occludes pointer input but differs in z-order and keyboard ownership. |
| `CMD-04-C018` | Battle Summary blocks underlying input. | Partial | Separate tactical result routing exists; the original modal matrix is open. |
| `CMD-04-C019` | Encyclopedia blocks underlying input. | Partial | Recovered shells exist in bounded routes; production topic routing remains open. |
| `CMD-04-C020` | Message windows block underlying input. | Partial | The shell blocks input in fixtures; populated production behavior is absent. |

The sector constraints and rail semantics were located through Faction Wars'
[window-behavior reading](https://github.com/TeeJS/faction-wars/blob/626b9b2bd62e7a83a9938568321c59cf64616a19/GAMEPLAY.md#L3998-L4025).

## Day, messages, and advice

| Cell | Original behavior | Port | Original proof |
|---|---|---|---|
| `CMD-05-C012` | A new campaign displays day 0 before the first elapsed day. | Present | `GameClock::new()` and the cockpit day readout start at zero; dedicated original trace open. |
| `CMD-05-C013` | The time bar is blank during the opening briefing. | Missing | Manual-derived lead; opening briefing parity is itself open. |
| `CMD-08-C017` | A click selects one Message Index row. | Present | `FUN_004665f0`; bounded row-gesture tests in `message_index.rs`. |
| `CMD-08-C018` | Ctrl-click toggles an individual Message Index row. | Present | `FUN_004665f0`; bounded row-gesture tests in `message_index.rs`. |
| `CMD-08-C019` | Shift-click selects a contiguous Message Index range. | Present | `FUN_004665f0`; bounded row-gesture tests in `message_index.rs`. |
| `CMD-08-C020` | Double-click opens or reads the selected message. | Present | `FUN_004665f0`; bounded row-gesture tests in `message_index.rs`. |
| `CMD-08-C021` | Agent Advice starts enabled on Easy. | Present | `FUN_00439320`; difficulty-default tests in `agent_menu.rs`. |
| `CMD-08-C022` | Agent Advice starts disabled on Medium and Hard. | Present | `FUN_00439320`; difficulty-default tests in `agent_menu.rs`. |
| `CMD-08-C023` | The Agent menu toggles advice. | Present | `FUN_00439e80`; speed hold/restore tests and both-faction Agent-menu browser entry gate. |

See Faction Wars'
[Message Index gesture reading](https://github.com/TeeJS/faction-wars/blob/626b9b2bd62e7a83a9938568321c59cf64616a19/GAMEPLAY.md#L368-L380)
and [Agent Advice reading](https://github.com/TeeJS/faction-wars/blob/626b9b2bd62e7a83a9938568321c59cf64616a19/GAMEPLAY.md#L4423-L4432).

## Personnel, fleet, ship, and troop Finders

| Cell | Original behavior | Port | Original proof |
|---|---|---|---|
| `OBJ-03-C014` | Right-click a character, choose Status: the Character Status window (manual p. 101, Fig. 3.46). | Partial | `FUN_0042a440` → type `0x1a` (`FUN_00442d70`), rows `FUN_004486f0`; `status_window.rs`, both-side browser gate `status-window.mjs` (background and buttons pixel-exact, portrait keyed). Differences in `DEV-UI-018`. |
| `OBJ-03-C016` | Personnel Finder has a distinct Character Finder view. | Present | `FUN_00464e10`; source-backed view tabs and both-faction F5 browser entry gate. |
| `OBJ-03-C017` | Personnel Finder has a distinct SpecForces Finder view. | Present | `FUN_00464e10`; source-backed view tabs and renderer tests. |
| `OBJ-03-C018` | SpecForces Finder is a type-by-system count grid. | Present | `FUN_00465bb0`; four-column system rows in `personnel_finder.rs`. |
| `OBJ-03-C019` | Typing a system name moves the SpecForces grid to that system. | Present | `FUN_00609650`; bounded prefix-search tests in `personnel_finder.rs`. |
| `OBJ-04-C009` | F4 opens Troop Finder. | Present | Command `0x130`; both-faction `audit-batch.mjs` gate. |
| `OBJ-04-C010` | Troop Finder searches by system name. | Present | `FUN_00609650`; bounded prefix-search tests in `troop_finder.rs`. |
| `OBJ-04-C011` | Troop Finder has Alliance and Imperial tabs. | Present | `FUN_00462ec0`; source-backed faction tabs in `troop_finder.rs`. |
| `OBJ-04-C012` | Display opens the selected troop's System window. | Present | `FUN_00429440`; bounded navigation tests in `troop_finder.rs`. |
| `OBJ-04-C013` | Fleet Finder navigation opens the selected fleet's Sector window. | Present | Confirmed in `ghidra/notes/fleet-finder.md` and the native journey. |
| `OBJ-04-C014` | Fleet Finder navigation opens the matching Fleet window. | Present | Confirmed in `fleet-finder.md` and the native journey. |
| `OBJ-04-C015` | The selected fleet or ship remains selected in the destination window. | Present | Confirmed in `fleet-finder.md`; complete state coverage open. |

See Faction Wars'
[Personnel Finder reading](https://github.com/TeeJS/faction-wars/blob/626b9b2bd62e7a83a9938568321c59cf64616a19/GAMEPLAY.md#L1823-L1832)
and [Troop Finder reading](https://github.com/TeeJS/faction-wars/blob/626b9b2bd62e7a83a9938568321c59cf64616a19/GAMEPLAY.md#L3079-L3084).

## Hyperspace presentation and input

| Cell | Original behavior | Port | Original proof |
|---|---|---|---|
| `OBJ-05-C016` | An en-route fleet or ship receives its moving-star or engine-glow presentation. | Present | `FUN_0042c3b0` draws static marks, with no animation: 10423/10426 on the fleet (10426 behind the picture), and each object's own GOKRES engine glow on the ships of a travelling fleet (`FUN_004f8240`): minis `+0x5000` in its ship entries and right-list items, and portrait `+0x1000` on one selected ship (`ghidra/notes/fleet-window.md`, "En route marks"). Browser-checked on both sides. |
| `OBJ-05-C017` | A fleet or ship in hyperspace cannot receive orders. | Partial | Core movement refusals exist; the complete Fleet-window disabled-state matrix is open. |
| `OBJ-12-C015` | A mission team in hyperspace receives its moving-star presentation. | Present | The Missions window draws `FUN_0042c3b0`'s static starfield mark over a travelling member's mini: 11501 for characters and most special forces, and GOKRES 21826/21888 for classes `0x3c000003`/`0x3c000005` (`fleet-window.md`, "En route marks"). Only the Missions window shows travellers: the Defenses window's page 1 excludes mission members and the original System window lists only facilities (`fleet-window.md`). |
| `OBJ-12-C016` | Mission controls that would issue orders are disabled in hyperspace. | Missing | Manual pp. 96–97, 112–113; production Mission Status behavior is absent. |

See Faction Wars'
[hyperspace reading](https://github.com/TeeJS/faction-wars/blob/626b9b2bd62e7a83a9938568321c59cf64616a19/GAMEPLAY.md#L2097-L2110)
and [fleet indicator table](https://github.com/TeeJS/faction-wars/blob/626b9b2bd62e7a83a9938568321c59cf64616a19/GAMEPLAY.md#L2221-L2228).

## Manufacturing destinations and item menus

| Cell | Original behavior | Port | Original proof |
|---|---|---|---|
| `OBJ-07-C015` | Each production area retains its own Destination setting. | Present | `FUN_00512700` and `FUN_0052c170`; save v29 persists one destination per production area. |
| `OBJ-07-C016` | Destination targeting accepts a galaxy-map system. | Present | Targeting command `0x214`; both-faction `fleet-window.mjs` destination gate. |
| `OBJ-07-C017` | Destination targeting accepts another system's Defense window. | Present | Child-window release routing in `targeting.rs`; bounded route tests. |
| `OBJ-07-C018` | Destination targeting accepts another system's Manufacturing window. | Present | Current System-window release routing reaches its manufacturing tab; authentic type-9 composition remains open under `DEV-UI-008`. |
| `OBJ-07-C019` | Destination targeting accepts another system's Fleet window. | Present | Fleet-window `+0x70` release routing; bounded route tests. |
| `OBJ-07-C020` | Best Time to Completion is shown as an absolute game day. | Present | `ProductionQueue::completion_days(today)` and queue rendering; authentic Build Selection composition remains open. |
| `OBJ-14-C019` | Fleet item menu exposes Rename. | Present | Order `0x203`, `FUN_004ac7a0`/`FUN_004ac950`; both-faction Rename browser gate. |
| `OBJ-14-C020` | Ship item menu exposes Rename. | Present | Order `0x203`; in-place Fleet-window edit and both-faction Rename browser gate. |

Faction Wars records the per-producer Destination correction in
[BACKLOG item 50](https://github.com/TeeJS/faction-wars/blob/626b9b2bd62e7a83a9938568321c59cf64616a19/BACKLOG.md#L64)
and production-window fields in its
[window checklist](https://github.com/TeeJS/faction-wars/blob/626b9b2bd62e7a83a9938568321c59cf64616a19/docs/window-checklists.md#L184-L195).

## Audit consequence

This pass raises the canonical ledger from 564 to 627 required baseline cells.
No new cell is accepted by documentation. The 63 additions expose requirements
that previously could have been missed by a broad family-level pass. The
[known-deviations register](known-deviations.md) distinguishes approved
extensions from temporary or unapproved differences so an implementation gap
cannot silently become policy.
