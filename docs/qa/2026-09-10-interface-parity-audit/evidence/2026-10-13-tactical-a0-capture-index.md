# Tactical A0 capture index

Every A0 frame of the original game held in the ignored manifest
(`.artifacts/interface-parity/a0/manifest.json`), one row per acceptance cell.
Each frame is `.artifacts/interface-parity/a0/<cell>/original.png`, a 640×480
guest-side capture of the unmodified `REBEXE.EXE` (README of
`tools/original-capture`, "A0 captures"). The setup names a save in
`tools/original-capture/saves/INDEX.md` or the journey that reached the state;
the full input trace stays in the manifest.

Coverage: 113 of 114. TAC-01 to TAC-07 are the 106-cell tactical
matrix; TAC-08, the ground assault (`UIP-F-010`), is an A0-only family outside
that denominator (`A0_ONLY_FAMILY_COUNTS` in
`tools/interface-parity/tactical-matrix.mjs`). A frame here is a reference,
not an acceptance: comparison against the port and the strict gate have not
run. Regenerate this file with `node tools/interface-parity/a0-capture-index.mjs`.

Open: none.
Settled without a frame: TAC-03-C020 (the reason is in its row;
`docs/plans/2026-09-22-tooling-tactical-106-cell-acceptance.md`, session 7).

## TAC-01: Main space-battle viewport and faction HUD (13 of 13)

| Cell | Requirement | Setup | What the frame shows | SHA-256 |
|---|---|---|---|---|
| TAC-01-C001 | Alliance | original-save `alliance-transit-day149` | first battle after the transit save, played on without a reload, so Show Planet stays off (no planet in view); reloading the transit save restores it to on | `6569945a6438…` |
| TAC-01-C002 | Imperial | original-save `empire-day30-yaga-alert-control` | no further input (last input step) | `be4cae91f021…` |
| TAC-01-C003 | initial paused | original-save `empire-day30-yaga-alert-control` | no further input (last input step) | `be4cae91f021…` |
| TAC-01-C004 | running | original-save `empire-day30-yaga-alert-ed-stranded` | Running battle: ISD vs Bulk Cruiser with both hyperdrives hit (save_edit.py stranded, output sha256 2a22dda9…abf476). Burst frames show the cruiser and camera moving; the pause button is lit green. (also interaction trace) | `dcfc8cc9d1ad…` |
| TAC-01-C005 | observing | original-save `empire-day30-yaga-alert-ed-stranded` | Observing a running battle: ISD vs Bulk Cruiser with both hyperdrives hit (save_edit.py stranded, output sha256 2a22dda9…abf476); Observe Battle stays on screen while the AI flies the fleet. (also interaction trace) | `9882eb1e9579…` |
| TAC-01-C006 | simulated | original-save `empire-day30-yaga-alert-control` | before: TAC-06-C006, the paused battle with Battle Options open, immediately before Immediate Result | `f558b555f342…` |
| TAC-01-C007 | empty space | original-save `alliance-transit-day149` | Show Planet off, set on the galaxy map; options are not in the save and every load resets them to On, so this view exists only on the first run from the transit save without a reload. The Empire planet-on view on the same camera route (task force 1, Center, Zoom Out x3, rotate left x4) is .artifacts/original-checks/a0/PL-on.png (empire-day30-yaga-alert-control, Yaga Minor), unpaired because the setups differ | `ad8457950853…` |
| TAC-01-C008 | planet backdrop | original-save `empire-day30-yaga-alert-control` | planet battle at Yaga Minor; the planet backdrop is at the top left | `435fbaac5b5e…` |
| TAC-01-C009 | multi-ship battle | original-save `alliance-transit-day149` | two Alliance ships against two Imperial, paused at first contact (the first try, a burst, began after the fleet died and was withdrawn). Show Planet back On after the reload; Coruscant lies outside this view | `ae8d28c83395…` |
| TAC-01-C010 | battle entry from Battle Alert | original-save `empire-day30-yaga-alert-control` | before: E0-alert, the Battle Alert Summary tab | `be4cae91f021…` |
| TAC-01-C011 | battle initialization | original-save `empire-day30-yaga-alert-control` | before: E0-alert, the Battle Alert Summary tab | `be4cae91f021…` |
| TAC-01-C012 | tactical music and voice | original-save `alliance-transit-day149` | the frame anchors the cell's sound: original.wav is the guest's audio, recorded on the host by QEMU wavcapture from just before Resume Battle, through about 8 s of the engagement, this paused frame, and the rest of the battle to Battle Results; audio.json gives the frame's offset into it (also audio capture, interaction trace) | `679ca7e190b3…` |
| TAC-01-C013 | return to strategy | original-save `empire-day30-yaga-alert-ed-stranded` | save_edit.py stranded on empire-day30-yaga-alert-control.sav (sha256 2a22dda9…abf476): Bulk Cruiser 3's hyperdrives out, so the battle ran to its destruction | `a6aa48a36081…` |

## TAC-02: Task-force and fighter-group controls (13 of 13)

| Cell | Requirement | Setup | What the frame shows | SHA-256 |
|---|---|---|---|---|
| TAC-02-C001 | task forces 1–8 | original-save `empire-sixteen-tie` | Task forces 1-8 all assigned, one Imperial capital each (manual pp. 143-144: a selected ship clicked into the blank slot forms the next task force); the formation's four two-ship task forces split by hand. Fighter groups red/blue/green/gold unchanged. Battle paused. | `b0837fd29031…` |
| TAC-02-C002 | red squadron | original-save `empire-day30-yaga-alert-control` | click the Red fighter-group slot (game 360,10): Red Group selected (last input step) | `0b6043e5bd2d…` |
| TAC-02-C003 | blue squadron | original-save `empire-sixteen-tie` | Blue fighter group in the strip's second colour slot: 16 TIE squadrons aboard Fleet 1 split red/blue/green/gold by FUN_005ae460 (largest group over seven halves into the next empty colour). Battle paused at Take Command. | `860695d1e2b6…` |
| TAC-02-C004 | green squadron | original-save `empire-sixteen-tie` | Green fighter group in the strip's third colour slot: 16 TIE squadrons aboard Fleet 1 split red/blue/green/gold by FUN_005ae460. Battle paused at Take Command; same frame as TAC-02-C003. | `860695d1e2b6…` |
| TAC-02-C005 | gold squadron | original-save `empire-sixteen-tie` | Gold fighter group in the strip's fourth colour slot: 16 TIE squadrons aboard Fleet 1 split red/blue/green/gold by FUN_005ae460. Battle paused at Take Command; same frame as TAC-02-C003. | `860695d1e2b6…` |
| TAC-02-C006 | assigned | original-save `empire-day30-yaga-alert-control` | click empty tactical space (game 240,240): nothing selected; task forces 1 and 2 and the Red fighter slot populated, pointer away (last input step) | `be4cae91f021…` |
| TAC-02-C007 | unassigned | original-save `empire-sixteen-tie` | Unassigned task-force slots 5-8 (bitmap 1007, FUN_0059be80; manual p. 143, the blank space next to the highest number) beside assigned task forces 1-4. Battle paused at Take Command; same frame as TAC-02-C003. | `860695d1e2b6…` |
| TAC-02-C008 | rest | original-save `empire-day30-yaga-alert-control` | click empty tactical space (game 240,240): nothing selected; task forces 1 and 2 and the Red fighter slot populated, pointer away (last input step) | `be4cae91f021…` |
| TAC-02-C009 | hover | original-save `empire-day30-yaga-alert-control` | pointer resting on the occupied task force 1 control; before: TAC-02-C008, the same strip at rest with the pointer away | `4ee1696ccfee…` |
| TAC-02-C010 | pressed | original-save `empire-day30-yaga-alert-control` | left button held on the occupied task force 1 control; before: TAC-02-C009, the pointer resting on it unpressed | `a022f0f7052d…` |
| TAC-02-C011 | selected | original-save `empire-day30-yaga-alert-control` | Task force 1 selected while paused: the ISD in the status panel, yellow selection box in the viewport. | `530151ea4662…` |
| TAC-02-C012 | disabled | original-save `empire-day30-yaga-alert-control` | before: TAC-02-C008, the strip before the click | `be4cae91f021…` |
| TAC-02-C013 | both factions | original-save `alliance-transit-day149` | Alliance side of the task-force bar; the Empire side is the day-30 Yaga cells (TAC-02-C011). Show Planet off from the galaxy map | `067136a70e03…` |

## TAC-03: Tactical ship selection, status, damage, and targeting (19 of 20)

| Cell | Requirement | Setup | What the frame shows | SHA-256 |
|---|---|---|---|---|
| TAC-03-C001 | healthy | original-save `empire-day30-yaga-alert-control` | click task force 1 button (game 70,10): Imperial Star Destroyer selected (last input step) | `530151ea4662…` |
| TAC-03-C002 | damaged | original-save `empire-day30-yaga-alert-ed-damage` | save_edit.py damage on empire-day30-yaga-alert-control.sav (output sha256 db514c7f…cf2032): Carrack hull damage 400 of 1000 (60% left), no subsystem hits; the ISD in TAC-03-C003 is the critical case | `afaccf5b9114…` |
| TAC-03-C003 | critical | original-save `empire-day30-yaga-alert-ed-damage` | save_edit.py damage on empire-day30-yaga-alert-control.sav (output sha256 db514c7f…cf2032): ISD hull damage 2200 of 2750 (20% left) with 1 shield, 2 weapons, 1 tractor, 1 engine hit; Carrack hull damage 400 of 1000. Compare TAC-03-C001 (same ship unedited) | `75ee1301fdaf…` |
| TAC-03-C004 | destroyed | original-save `empire-day30-yaga-alert-ed-fragile` | Bulk Cruiser 3 selected while the ISD destroys it. Burst frames 00-37 run the attack with the cruiser in the panel; frames 38-43 (t 19.0-21.5 s) are one held frame after the kill: the cruiser gone, the panel back to the Imperial seal, the cursor on Resume Battle. Frame 38 is the cell frame (registered with a0-register TAC-03-C004 38); from frame 44 the results screen. The original freezes the battle about 3 s after the last enemy dies and then opens the results, so a pause cannot hold the destroyed state. (also interaction trace) | `68d9d05fc4f0…` |
| TAC-03-C005 | hull | original-save `empire-day30-yaga-alert-control` | click task force 1 button (game 70,10): Imperial Star Destroyer selected (last input step) | `530151ea4662…` |
| TAC-03-C006 | shield | original-save `empire-day30-yaga-alert-ed-stranded` | Bulk Cruiser 3 selected under the ISD attack: its shield bar is shorter than in the before frame (SH-before, the same selection paused 13 s earlier) and two system icons have dimmed. | `772b4e022a96…` |
| TAC-03-C007 | engines | original-save `empire-day30-yaga-alert-ed-engines` | save_edit.py engines on empire-day30-yaga-alert-control.sav (output sha256 ebe9dd3b…cf18c5): four engine hits (the limit), nothing else; two hits (TAC-03-C009-C011 copy) left the icon unchanged | `b3a738b8c6ac…` |
| TAC-03-C008 | hyperdrive | original-save `empire-day30-yaga-alert-isd-hyperdrive` | ISD primary and backup hyperdrives at zero by save edit (saves/INDEX.md); compare TAC-03-C001 (E0), where the same icon is lit | `8b814090821a…` |
| TAC-03-C009 | tractor | original-save `empire-day30-yaga-alert-ed-subsystems` | save_edit.py subsystems on empire-day30-yaga-alert-control.sav (output sha256 75f52eac…364bde): ISD at full hull with two hits each on shield generator, weapons, tractor and engines; the first three icons differ from TAC-03-C001 (same ship unedited), the engine icon does not | `d492e0dceb8c…` |
| TAC-03-C010 | weapons | original-save `empire-day30-yaga-alert-ed-subsystems` | save_edit.py subsystems on empire-day30-yaga-alert-control.sav (output sha256 75f52eac…364bde): ISD at full hull with two hits each on shield generator, weapons, tractor and engines; the first three icons differ from TAC-03-C001 (same ship unedited), the engine icon does not | `d492e0dceb8c…` |
| TAC-03-C011 | shield generator | original-save `empire-day30-yaga-alert-ed-subsystems` | save_edit.py subsystems on empire-day30-yaga-alert-control.sav (output sha256 75f52eac…364bde): ISD at full hull with two hits each on shield generator, weapons, tractor and engines; the first three icons differ from TAC-03-C001 (same ship unedited), the engine icon does not | `d492e0dceb8c…` |
| TAC-03-C012 | contents | original-save `empire-day30-yaga-alert-control` | click task force 1 button (game 70,10): Imperial Star Destroyer selected (last input step) | `530151ea4662…` |
| TAC-03-C013 | current orders | original-save `empire-day30-yaga-alert-control` | click the commit check mark (game 562,242): detail panel reads Attack Capital Ships, Tactics: Stand Off (last input step) | `f8c75d30ed11…` |
| TAC-03-C014 | previous | original-save `empire-day30-yaga-alert-control` | Two-ship task force 2: previous moves the status panel from the ISD back to the Carrack, and the yellow box follows; the before frame shows the ISD. | `2ad85a22bbb3…` |
| TAC-03-C015 | next | original-save `empire-day30-yaga-alert-control` | Two-ship task force 2: next moves the status panel from the Carrack to the ISD, and the yellow box follows; the before frame shows the Carrack. | `86bcf2a1d2bf…` |
| TAC-03-C016 | target selected | original-save `empire-day30-yaga-alert-control` | right-click Bulk Cruiser 3 (game 368,134): orders read Attacking Bulk Cruiser 3 (last input step) | `17da8d384e66…` |
| TAC-03-C017 | target lost | original-save `empire-day30-yaga-alert-ed-fragile` | The ISD selected, attacking the 10-hull Bulk Cruiser, while it dies. Burst frames 00-43 run the attack (Attacking Bulk Cruiser 3); frame 44 is the change; frames 45-50 (t 22.5-25.0 s) are one held frame of target lost: the cruiser gone, the orders line No Orders, Missions and Maneuvers blank. Frame 45 is the cell frame (registered with a0-register TAC-03-C017 45); from frame 51 the results screen. The burst started paused; Resume Battle (718,397) was clicked about 6 s in and the cursor walked back to the park point. (also interaction trace) | `ea760f672b64…` |
| TAC-03-C018 | capital ship selection | original-save `empire-day30-yaga-alert-control` | left-click the Imperial Star Destroyer in the viewport (game 29,429) (last input step) | `c364b4c3cfdf…` |
| TAC-03-C019 | fighter selection | original-save `empire-day30-yaga-alert-control` | click the Red fighter-group slot (game 360,10): Red Group selected (last input step) | `0b6043e5bd2d…` |
| TAC-03-C020 | Death Star selection | — | **No separate original state**: the original draws no selectable Death Star: none in the 3D view at any zoom, rotation or highlight, none among the eight capitals the panel arrows cycle, and clicking it in the Missions display does nothing. Its only selected form is the target bracket in TAC-05-C009's frame | — |

## TAC-04: Navigation, camera, routes, and highlights (14 of 14)

| Cell | Requirement | Setup | What the frame shows | SHA-256 |
|---|---|---|---|---|
| TAC-04-C001 | nav sets 1–4 | original-save `empire-day30-yaga-alert-control` | set 4 shown; before: E0-nav3 (set 3). Sets 1 and 2 from the same run: E0-nav1 fd5daa81c278729e757e5c10aa26d2355436f49be166389b2154f15316c083c3, E0-nav2 dc0deb2f02de3778abd0ebe22fac4bb0abd2ddf89d6e8135112d4dd48234d149 (.artifacts/original-checks/a0/) | `ae8b85807dcb…` |
| TAC-04-C002 | route ordered | original-save `empire-day30-yaga-alert-control` | before: E0-isd-nav4, the ISD selected with set 4 shown, orders No Orders | `cdb69c8ecebf…` |
| TAC-04-C003 | multiple targets | original-save `alliance-transit-day149` | two targets in selection order (manual p. 145: targets are attacked in the order selected; selected targets are boxed). The game reads Ctrl on a frame tick, so a Ctrl pressed and released around a click in the same instant is missed; hold it about half a second. Show Planet off | `253e5c4f2d23…` |
| TAC-04-C004 | selected | original-save `empire-day30-yaga-alert-control` | click task force 1 button (game 70,10): Imperial Star Destroyer selected (last input step) | `530151ea4662…` |
| TAC-04-C005 | deselected | original-save `empire-day30-yaga-alert-control` | before: TAC-04-C004, the Star Destroyer selected | `be4cae91f021…` |
| TAC-04-C006 | Alliance highlight | original-save `empire-day30-yaga-alert-control` | before: TAC-02-C002, the same state before the toggle | `6db409b31f0e…` |
| TAC-04-C007 | Empire highlight | original-save `empire-day30-yaga-alert-control` | before: TAC-04-C006, Alliance highlight only | `3c210de8d06a…` |
| TAC-04-C008 | tilt | original-save `empire-day30-yaga-alert-control` | before: TAC-04-C009, the view before the click | `a7bb567cd3de…` |
| TAC-04-C009 | rotate | original-save `empire-day30-yaga-alert-control` | before: TAC-04-C010, the view before the click | `21d1d2efea49…` |
| TAC-04-C010 | zoom | original-save `empire-day30-yaga-alert-control` | before: TAC-04-C002, the view before the click | `df60faac4d94…` |
| TAC-04-C011 | center | original-save `empire-day30-yaga-alert-control` | before: TAC-04-C008, the ISD off-centre at the frame edge | `cb4f47b4a618…` |
| TAC-04-C012 | memorize | original-save `empire-day30-yaga-alert-control` | before: E0-recalled, the view when Memorize was pressed; this frame, after rotating away and Recall, is identical. The right red button (game 616,458) is Memorize and the left (game 502,458) Recall; tooltips fall off-screen, so the roles were settled by hashes: Memorize pressed at view 7ff7144f (E0-recalled); rotated away to 85ce01f2 (E0-moved2); Recall restored exactly 7ff7144f. The earlier left-button press labelled Memorize in this trace was a Recall. | `7ff7144f1723…` |
| TAC-04-C013 | recall | original-save `empire-day30-yaga-alert-control` | before: E0-moved2, the view rotated away. The right red button (game 616,458) is Memorize and the left (game 502,458) Recall; tooltips fall off-screen, so the roles were settled by hashes: Memorize pressed at view 7ff7144f (E0-recalled); rotated away to 85ce01f2 (E0-moved2); Recall restored exactly 7ff7144f. The earlier left-button press labelled Memorize in this trace was a Recall. | `7ff7144f1723…` |
| TAC-04-C014 | chase camera | original-save `empire-day30-yaga-alert-ed-stranded` | Chase camera on task force 1 (the ISD) attacking the stranded Bulk Cruiser: no holocube, the view moves with the task force. The control save cannot stage this: its Alliance withdraws seconds after the unpause (withdrawn capture kept in .artifacts/original-checks/withdrawn). Before: CH2-before, the paused chase view. (also interaction trace) | `d39be0525a37…` |

## TAC-05: Tactical maneuvers, tactics, and missions (23 of 23)

| Cell | Requirement | Setup | What the frame shows | SHA-256 |
|---|---|---|---|---|
| TAC-05-C001 | hook | original-save `empire-day30-yaga-alert-control` | Right Hook chosen; the paired before-frame E0-left-hook is Left Hook chosen, so both hooks are recorded | `7071db2f07f8…` |
| TAC-05-C002 | hammer | original-save `empire-day30-yaga-alert-control` | click Hammer, the top-left maneuver button (game 508,170; tooltip Hammer): chosen, not committed (last input step) | `f67d05a6c7bb…` |
| TAC-05-C003 | anvil | original-save `empire-day30-yaga-alert-control` | click Anvil, the top-right maneuver button (game 538,170; tooltip Anvil): chosen, not committed (last input step) | `24bd57b9a7ad…` |
| TAC-05-C004 | hold | original-save `empire-day30-yaga-alert-control` | Hold Position chosen, not committed, on the Carrack in two-ship task force 2; the hold graphic shows at the top of the panel. | `92b13f7ba227…` |
| TAC-05-C005 | surround | original-save `empire-day30-yaga-alert-control` | before: E0-surround-panel, the toggle switched to its ring icon before the commit | `6564da874da9…` |
| TAC-05-C006 | standoff | original-save `empire-day30-yaga-alert-control` | click the left-hand detail button (game 522,248): Maneuvers and Tactics panel opens with the default tactic, Stand Off (last input step) | `0a4d270b8d45…` |
| TAC-05-C007 | attack capital ships | original-save `empire-day30-yaga-alert-control` | click Attack Capital (game 522,169): chosen, not committed (last input step) | `0f16d4a78335…` |
| TAC-05-C008 | attack fighters | original-save `empire-day30-yaga-alert-control` | Attack Fighters chosen, not committed, on the Carrack (Empire player); the confirmation graphic shows an X-wing in the target frame. | `443c0b0a887e…` |
| TAC-05-C009 | attack Death Star | original-save `alliance-day1169-deathstar-trench-run` | Alliance player, Red Group selected; before: TAC-05-C021 | `06a30297bf38…` |
| TAC-05-C010 | recover | original-save `empire-day30-yaga-alert-control` | click Recover (game 590,169): chosen, not committed; display shows the TIEs returning to the Star Destroyer (last input step) | `ad9e271df5e5…` |
| TAC-05-C011 | escort | original-save `empire-day30-yaga-alert-control` | before: E0-carrack, the Carrack selected with No Orders | `023eac198d38…` |
| TAC-05-C012 | rest | original-save `empire-day30-yaga-alert-control` | click the right-hand detail button (game 590,248): Missions panel opens, nothing chosen (last input step) | `a085fba70f0e…` |
| TAC-05-C013 | pressed | original-save `empire-day30-yaga-alert-control` | mission button (Attack Capital) held down; before: E0r4-hammer-pressed, a maneuver button (Hammer) held down | `28b3309b61c6…` |
| TAC-05-C014 | selected | original-save `empire-day30-yaga-alert-control` | click Attack Capital (game 522,169): chosen, not committed (last input step) | `0f16d4a78335…` |
| TAC-05-C015 | disabled | original-save `empire-day30-yaga-alert-control` | before: TAC-05-C012, the panel before the click | `a085fba70f0e…` |
| TAC-05-C016 | both factions | original-save `alliance-transit-day149` | Alliance side of the orders panel; the Empire side is the day-30 Yaga TAC-05 cells. Attack Capital and Attack Fighters lit, the right-hand missions grey | `384db0270703…` |
| TAC-05-C017 | Death Star laser ready | original-save `empire-death-star-pre-alert` | Empire owns the Death Star (Fleet 6 at Coruscant); battle paused at start; laser control at rest with the gauge full | `3b37af09acf6…` |
| TAC-05-C018 | Death Star laser loading | original-save `empire-death-star-pre-alert` | Battle paused; shot committed on an Alliance capital: charge reset (gauge empty) and the fire control in its loading state; README errata a) Death Star Tactical Controls | `799ea931653c…` |
| TAC-05-C019 | Death Star laser fired | original-save `empire-death-star-pre-alert` | Battle unpaused with a superlaser shot committed on an Alliance capital; the frame shows the shot striking its target (also interaction trace) | `0e5ae202c0bc…` |
| TAC-05-C020 | Death Star charge gauge | original-save `empire-death-star-pre-alert` | Battle paused about two minutes after the superlaser shot: the owner-only charge gauge partly refilled (green over blue) and the fire control still loading | `483d35d28622…` |
| TAC-05-C021 | attack Death Star enabled | original-save `alliance-day1169-deathstar-trench-run` | Alliance player, 59 squadrons against 23-24 TIE squadrons, Death Star 1 in Imperial Fleet 16 | `cfadb0871160…` |
| TAC-05-C022 | attack Death Star disabled | original-save `empire-day30-yaga-alert-control` | Empire player, capital ship selected, no hostile Death Star; before: TAC-05-C015 | `a085fba70f0e…` |
| TAC-05-C023 | trench-run launch | original-save `alliance-day1169-deathstar-trench-run` | Alliance player; Red Group committed to Attack Death Star; burst of 60 frames 500 ms apart across the launch, which the original marks only by voice 15161 and the 120 s timer (no visible change); before: TAC-05-C023-before (also interaction trace, audio capture, interaction trace) | `829ce9db16f4…` |

## TAC-06: Battle Options, withdraw, simulate, and observe (9 of 9)

| Cell | Requirement | Setup | What the frame shows | SHA-256 |
|---|---|---|---|---|
| TAC-06-C001 | options | original-save `empire-day30-yaga-alert-control` | click Battle Options (game 616,318): the options panel opens (last input step) | `ef200f133a4e…` |
| TAC-06-C002 | withdraw | original-save `empire-day30-yaga-alert-control` | before: TAC-06-C003, the Withdraw Confirmation (an earlier opening in the same run, cancelled then); this frame follows a second Withdraw and Confirm | `635b53c11065…` |
| TAC-06-C003 | withdraw confirmation | original-save `empire-day30-yaga-alert-control` | click Withdraw (game 524,62): Withdraw Confirmation opens (last input step) | `1dfb5366c582…` |
| TAC-06-C004 | simulate remainder | original-save `empire-day30-yaga-alert-control` | before: TAC-06-C006, the paused battle with Battle Options open, immediately before Immediate Result | `f558b555f342…` |
| TAC-06-C005 | observe | original-save `empire-day30-yaga-alert-control` | before: E0r2-opts, the options panel in commanding mode | `a4337e819e49…` |
| TAC-06-C006 | take command | original-save `empire-day30-yaga-alert-control` | before: TAC-06-C005, observing | `b93eae34dd24…` |
| TAC-06-C007 | game options | original-save `empire-day30-yaga-alert-control` | click Game Options, the blue disc (game 524,228): the Game Options screen opens (last input step) | `4694b3ccc5e1…` |
| TAC-06-C008 | close | original-save `empire-day30-yaga-alert-control` | before: TAC-06-C001, the open options panel | `6564da874da9…` |
| TAC-06-C009 | disabled | original-save `empire-day30-yaga-alert-control` | after a confirmed withdrawal; before: TAC-06-C001, the same panel with all five controls. The no-hyperdrive comparison (EH, R5a) is still to capture | `997593074250…` |

## TAC-07: Battle Results and force-detail tabs (14 of 14)

| Cell | Requirement | Setup | What the frame shows | SHA-256 |
|---|---|---|---|---|
| TAC-07-C001 | overview | original-save `empire-day30-yaga-alert-control` | wait about 75 s: Battle Results, Summary: The Imperial fleet is defeated; withdrawn to Ghorman (last input step) | `c1ae407d3b56…` |
| TAC-07-C002 | Alliance victory | original-save `empire-day30-yaga-alert-control` | the Empire player loses: Alliance forces have broken the blockade | `c1ae407d3b56…` |
| TAC-07-C003 | Imperial victory | original-save `empire-day30-yaga-alert-control` | the Empire player wins after Immediate Result | `f558b555f342…` |
| TAC-07-C004 | retreat | original-save `empire-day30-yaga-alert-control` | retreat after a confirmed withdrawal; before: TAC-06-C002, the battlefield just after Confirm | `c1ae407d3b56…` |
| TAC-07-C005 | capital ships | original-save `empire-day30-yaga-alert-control` | click Imperial Forces, the green tab (game 574,282): Capital Ships filter, ISD 1 and Carrack 2 Operational, No Casualties (last input step) | `8b577e260f13…` |
| TAC-07-C006 | fighters | original-save `empire-day30-yaga-alert-control` | click the Fighters filter (game 322,130): Fighter Squadrons, TIE Fighters Operational, No Casualties (last input step) | `04dec5c3f929…` |
| TAC-07-C007 | troops | original-save `empire-day30-yaga-alert-control` | click the Troops filter (game 374,130): Trooper Regiments, Stormtrooper Regiments Operational, No Casualties (last input step) | `33e973d1edfd…` |
| TAC-07-C008 | personnel | original-save `empire-day30-yaga-alert-control` | click the Personnel filter (game 422,130): Personnel, Darth Vader under Survivors, Captured None, Killed No Casualties (last input step) | `c90330a4e05d…` |
| TAC-07-C009 | destroyed | original-save `empire-day30-yaga-alert-ed-fragile` | save_edit.py fragile v2 on empire-day30-yaga-alert-control.sav (sha256 66dea9a8…e0d80): Bulk Cruiser 3 at 10 of 1200 hull with both hyperdrives out | `cc4173bf7db3…` |
| TAC-07-C010 | damaged | original-save `empire-day30-yaga-alert-ed-damage` | save_edit.py damage copy (sha256 db514c7f…cf2032): both Imperial ships entered damaged (ISD 20% hull, Carrack 60%). The original's results page has no Damaged column, only Operational and Destroyed, and lists damaged survivors as Operational | `8e5064ae3163…` |
| TAC-07-C011 | operational | original-save `empire-day30-yaga-alert-control` | click Imperial Forces, the green tab (game 574,282): Capital Ships filter, ISD 1 and Carrack 2 Operational, No Casualties (last input step) | `8b577e260f13…` |
| TAC-07-C012 | go to fleet | original-save `empire-day30-yaga-alert-control` | before: E0r2-goto, the go-directly-to page; the save itself opens with the Sesswenna sector and a Coruscant window | `8a6c07c58ffc…` |
| TAC-07-C013 | go to system | original-save `empire-day30-yaga-alert-control` | Go Directly To, the system branch: the display closes onto the Sesswenna sector map; unlike the Fleet branch (TAC-07-C012) no Yaga Minor window opens, and the Coruscant window the save opens with stays. Before: GS-before, the go-directly-to page. | `2a357a5349a7…` |
| TAC-07-C014 | strategic state persistence | original-save `empire-day30-yaga-alert-ed-damage` | Strategic Capital Ship Status of the ISD after the battle: Hull Value 550:2750, the 2200 hull damage the edited copy took into the battle (20%), Ship Damaged: No, Hyperdrive 80:80, Awaiting Orders in Fleet 1; the galaxy clock runs (day 33). | `a4398b1b232b…` |

## TAC-08: Ground assault presentation (8 of 8)

| Cell | Requirement | Setup | What the frame shows | SHA-256 |
|---|---|---|---|---|
| TAC-08-C001 | assault action | original-save `empire-day128-assault-fails-selonia` | Fleet right-click menu over Alliance-held Selonia with Planetary Assault highlighted; Fleet 1 carries one Stormtrooper Regiment. | `4ae961e70f48…` |
| TAC-08-C002 | assault summary | original-save `empire-day128-assault-fails-selonia` | Assault Summary reopened from the Message Index after the failed Selonia assault; same report the live alert shows, reached through the message log. | `15111bcfa405…` |
| TAC-08-C003 | force details | original-save `empire-day53-yaga-orbit` | Assault on Yaga Minor, Imperial Forces damage summary (manual Fig. 3.67), Trooper Regiments tab: three Stormtrooper Regiments operational, Destroyed: No Casualties. | `61d6d94ef0a6…` |
| TAC-08-C004 | success | original-save `empire-day53-yaga-orbit` | Assault on Yaga Minor summary after a successful assault by 3 Stormtrooper Regiments against an ungarrisoned Alliance system: Imperial troops have taken control of the Alliance system Yaga Minor. | `78b18b2f6baf…` |
| TAC-08-C005 | failure | original-save `empire-day128-assault-fails-selonia` | Assault on Selonia summary after a failed assault by one Stormtrooper Regiment against a garrisoned Alliance system: Alliance Troops have defended Selonia from an Imperial assault. | `8de34a314302…` |
| TAC-08-C006 | casualties | original-save `empire-day128-assault-fails-selonia` | Assault on Selonia, System Forces damage summary, Trooper Regiments tab: one Alliance Army Regiment operational, one destroyed. The Imperial Forces tab shows the Stormtrooper Regiment operational, No Casualties. | `648472c44d91…` |
| TAC-08-C007 | occupation | original-save `empire-day53-yaga-orbit` | After the successful assault Yaga Minor is an Imperial system: its planet window carries the Imperial green title bar and emblem tab (Alliance red before the assault, scratch capture sys1) and the sector map labels it green. | `8eefd6385ed6…` |
| TAC-08-C008 | return | original-save `empire-day53-yaga-orbit` | Closing the Assault Summary returns to the Yaga Minor orbit window: Fleet 1's troop tab is empty (0 regiments, the three Stormtrooper Regiments landed) and Yaga Minor is now Imperial green on the sector map. | `82b38085adf1…` |
