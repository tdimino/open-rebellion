# Tactical command assignment recovery

P58F9 recovers the selected-unit Maneuvers and Tactics and Missions panels
from `REBEXE.EXE` with Ghidra 12.1.3. All coordinates below use the original
640 by 480 tactical canvas. The command panel origin is `(482,28)`:
`FUN_0059cac0` builds the right-hand container through `FUN_005c5520` at
`(0x1e2,0x1c)`, 149 by 236, and every panel and button sits inside it
(`FUN_0059cac0.c`, `FUN_005c5520.c`, `FUN_005c55a0.c`). Bitmap matches on A0
frames put panels `1302`, `1304`, `1306` and `1309` at `(482,28)`.

## Selected-unit entry controls

`FUN_005e4110` constructs the capital-ship panel and `FUN_005e7030`
constructs the fighter panel. Both create the same entry controls.

| Command | Rectangle | Message | Normal / pressed BMP |
|---|---:|---:|---:|
| Maneuvers and Tactics | `(494,238) 58x22` | `0xdd02` capital, `0xdd07` fighter | `1107 / 1108` |
| Missions | `(562,238) 58x22` | `0xdd01` | `1105 / 1106` |

The decompile names neither message. The original settles them: the left
button opens Maneuvers and Tactics and the right one Missions, for a capital
ship and for a fighter group (A0 TAC-05-C006, TAC-05-C012, TAC-05-C021). The
art agrees: 1107 draws a ship on a curved arrow, 1105 a fighter on a target.

## Maneuvers and Tactics

`FUN_005dcc70` builds panel `1309`. `FUN_005dd3a0` maps the five buttons to
the shared order field at offset `+0x68`. `FUN_005dd290` restores the selected
order. `FUN_005dd080` commits on the check button and closes without mutation
on cancel. `FUN_005dd140` enforces mutual selection and draws the matching
confirmation graphic at `(496,42)`.

| Order | Source code | Rectangle | Normal / pressed BMP | Graphic |
|---|---:|---:|---:|---:|
| Left Hook | 7 | `(495,187) 28x28` | `1133 / 1134` | `2101` |
| Right Hook | 8 | `(525,187) 28x28` | `1135 / 1136` | `2102` |
| Hammer | 9 | `(495,157) 28x28` | `1137 / 1138` | `2103` |
| Anvil | 10 | `(525,157) 28x28` | `1139 / 1140` | `2104` |
| Hold Position | 11 | `(557,172) 28x28` | `1141 / 1142` | `2105` |

The tactic toggle is `(593,160) 24x52`. Bitmap `1117` selects Stand Off and
bitmap `1118` selects Surround. `FUN_005dd3a0` writes code 1 for Surround and
code 2 for Stand Off to offset `+0x38`. Stand Off is the documented default.
Check uses `1113 / 1114` at `(548,230) 27x25`; cancel uses `1115 / 1116` at
`(588,230) 27x25`.

## Missions

`FUN_005def00` creates panel `1308`. `FUN_005df110` constructs its controls,
`FUN_005df5f0` maps them to the shared order field, `FUN_005df660` restores a
selected order, and `FUN_005df4f0` commits through `FUN_00595c00` or cancels
without mutation. `FUN_005df6b0` selects the faction-specific confirmation
graphic at `(496,42)`.

| Order | Code | Rectangle | Alliance BMP | Empire BMP | Graphic |
|---|---:|---:|---:|---:|---:|
| Attack Capital Ships | 5 | `(500,157) 46x26` | `1179 / 1180` | `1182 / 1183` | `2151 / 2155` |
| Attack Fighters | 4 | `(500,187) 46x26` | `1191 / 1192` | `1194 / 1195` | `2152 / 2156` |
| Recover | 2 | `(568,157) 46x26` | `1170 / 1171 / 1172` | `1173 / 1174 / 1175` | `2153 / 2157` |
| Attack Death Star | 6 | `(568,187) 46x26` | `1176 / 1177 / 1178` | unavailable | `2154` |

`FUN_005df110` picks the Recover art in the same branch as the capital
button: 1179 with 1170..1172, 1182 with 1173..1175. A0 agrees: Empire frames
(TAC-05-C012, C015, C022) draw a disabled Recover as 1175, Alliance frames
draw 1170 (C009, C021) or, disabled, 1172 (C016).

Recover is enabled only for a selected fighter group. The constructor disables
Attack Death Star before later battle-state checks. P58F9 preserves that
disabled state rather than guessing its enablement predicate.

## Shared state boundary

`FUN_00596af0` and `FUN_00596b20` read the unified order at `+0x68`.
`FUN_00596b30` and `FUN_00596b60` read the tactic at `+0x38`.
`FUN_00595c00` applies an order and `FUN_00595c20` applies a tactic.

P58F9 restores exact panel selection, disabled states, cancel behavior, and
committed order values for selected capital ships and fighter groups. It does
not claim order execution, Death Star enablement, turning, collision response,
formation behavior, or lossless A0 visual parity.
