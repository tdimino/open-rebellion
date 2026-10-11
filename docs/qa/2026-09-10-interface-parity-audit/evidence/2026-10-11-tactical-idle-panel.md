# Tactical right-hand panel: idle seal, Battle Options, Game Options (2026-10-11)

Status: agent native check on both sides, against the unmodified original on
the capture VM. Not a person's check; the 106 `TAC-01`–`TAC-07` cells stay
pending.

## Original

`empire-day30-yaga-alert-isd-hyperdrive.sav`, Take Command, three runs. The
battlefield opens paused with nothing selected, and the right-hand housing
shows a riveted plate with the Imperial seal. Display Holocube on or off makes
no difference. Battle Options shows the circuit-board panel with Withdraw From
Battle, Get Immediate Result, Observe, Game Options and Close (tooltips read
over the VNC link). Opened from the battle, Game Options draws its middle
button as the blue wireframe cube and the Tactical Display Options labels in
grey (128, 128, 128), with the switches disabled.

## Source

- `FUN_005c5800` builds the housing with seven child panels. Only the first,
  constructor `0x5e9b60`, sits at (0, 0). Its `WM_CREATE` (`0x5e9c60`) loads
  TACTICAL 0x517 (1303, Alliance seal) or 0x518 (1304, Imperial seal) by
  side. It has no buttons. 1301, the blank cylinder, is the container's own
  background (`FUN_005c56b0`).
- `FUN_005dab90` places the five Battle Options buttons at panel-local
  (18, 19), (14, 93), (87, 93), (23, 184) and (97, 184).
- `FUN_00407180` picks the Game Options return button by origin: 0x2724
  (galaxy) by default, and 0x272a (10026–10028, the cube) for case 9, the
  battle.

## What the port did, and the fix

| Surface | Before | After |
|---|---|---|
| Idle housing | 1301, blank cylinder; a ship was selected on entry | 1303/1304 by side; nothing selected on entry |
| Battle Options panel | 1303/1304, misnamed as the options panel | 1306 for both sides |
| Battle Options buttons | hand-placed, 3–14 px off, so clicks near the art missed | source positions; matched in the capture to the pixel |
| Game Options from battle | galaxy return icon, green labels | cube icon 10026–10028, grey labels |

## Evidence (ignored)

- Original: `.artifacts/original-checks/2026-10-11-tactical-panel/`
  (`SHA256SUMS`).
- Port, Empire (binary `35491116…`):
  `.artifacts/native-checks/2026-10-11-tactical-panel-empire3/`:
  `01-idle-seal.png` `f380139b…`, `02-battle-options.png` `676c152a…`.
  In the same run, Close, Withdraw (with its confirmation and Cancel),
  Observe (`observing=true`), Game Options and the return to the battle all
  responded.
- Port, Alliance (binary `b4d4bf95…`):
  `.artifacts/native-checks/2026-10-11-tactical-panel-alliance/`:
  `01-idle-seal.png` `8fc91695…`, `02-battle-options.png` `d781655d…`,
  `03-game-options.png` `8d1fc3c6…`. The return left the game in
  `TacticalCombat`.

Still open: the Alliance-side original capture (the plate is read from the
source and the DLL, not yet seen in the original as the Alliance), and the
other five child panels' idle art.
