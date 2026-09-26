# P58-B17 tactical command voice bank

P58-B17 restores the original faction battle-ready call and the group-specific
maneuver, attack, formation, and mission acknowledgements used by tactical
command. The implementation follows the executable's event-index table and
routes 45 Alliance and 45 Imperial recordings from owned `VOICEFXA.DLL` and
`VOICEFXE.DLL` files. Browser verification remained muted.

This checkpoint does not claim audible native equivalence, mixing or
interruption parity, the remaining result/withdrawal/Death Star voice families,
or strict original-game acceptance.

## Source contract

`FUN_005ba980` stores an event index on the queued sound object.
`FUN_005bad50` resolves that index through the table built by
`FUN_005bae60`. Event index `0x20` begins the Alliance bank at WAVE `14001`;
event `0x9a` begins the Imperial bank at WAVE `15001`.

| Command family | Alliance events / WAVEs | Imperial events / WAVEs | Source callers |
|---|---|---|---|
| Battle ready | `0x20` / `14001` | `0x9a` / `15001` | tactical manager entry |
| Capital maneuver, task forces 1–8 | `0x22–0x29` / `14003–14010` | `0x9c–0xa3` / `15003–15010` | `FUN_005a3020`, `FUN_005a3f00` |
| Fighter maneuver, RGBY | `0x2a–0x2d` / `14011–14014` | `0xa4–0xa7` / `15011–15014` | `FUN_005a4e50` |
| Capital attack, task forces 1–8 | `0x3c–0x43` / `14029–14036` | `0xb6–0xbd` / `15029–15036` | `FUN_005a3aa0` |
| Fighter attack, RGBY | `0x44–0x47` / `14037–14040` | `0xbe–0xc1` / `15037–15040` | fighter attack manager |
| Capital formation, task forces 1–8 | `0x6f–0x76` / `14080–14087` | `0xed–0xf4` / `15084–15091` | `FUN_005a3740`, `FUN_005a3850` |
| Capital mission, task forces 1–8 | `0x78–0x7f` / `14089–14096` | `0xf6–0xfd` / `15093–15100` | `FUN_005a3630` |
| Fighter mission, RGBY | `0x80–0x83` / `14097–14100` | `0xfe–0x101` / `15101–15104` | `FUN_005a48d0` |

The generic recordings at Alliance WAVE `14002`, `14028`, and `14088` and
Imperial WAVE `15002`, `15028`, and `15092` are deliberately not staged: the
current production UI always commands a named task force or fighter group.
The former strategic mission, construction, and fleet-event aliases pointed at
these tactical recordings without source support and have been removed.

## Implementation result

- `dat-dumper --extract-tactical-voice` extracts exactly 90 source-proven
  recordings from owned faction DLLs without committing proprietary audio.
- The native backend can load the same resources directly from the original
  DLLs. The WASM pack stages only the 90 recordings used by this checkpoint.
- Take Command queues the proper faction battle-ready event.
- Maneuver, attack, formation, and mission confirmation select the original
  task-force or RGBY ordinal and its exact faction resource.
- Muted playback still reports resource presence, event identity, faction,
  routing, and cleanup for deterministic browser acceptance.
- The Ghidra decompiler helper now runs under Ghidra 12/PyGhidra without
  shadowing the host `writer` binding.

## Verification

| Gate | Result |
|---|---|
| Complete Rust workspace | 760 passed, 0 failed, 21 ignored in a clean isolated patch tree |
| Focused Rust tests | Renderer 256 passed and 1 ignored; app 17 passed |
| Owned DLL extraction | 45 Alliance plus 45 Imperial WAVs |
| Browser runtime pack | 118 audio entries total; 90 tactical voices |
| Focused muted browser journey | 4 of 4 passed; both factions and both viewports |
| Browser audio route | 22 weapon variants and 90 command voices loaded and routed per case |
| Browser startup and cleanup | 4 four-request starts, 4 muted launches, 4 closed browsers |
| Independent browser review | PASS; populated faction shells and no visible regression; A0 explicitly unclaimed |
| Strict 106-cell gate | Not run |

The focused run is `2026-09-24T20-58-40-136Z-18584`. Raw screenshots,
console logs, request ledgers, and per-case results remain under the ignored
`.artifacts/interface-parity/` tree. Durable hashes and the exact scope are in
the [artifact bundle](p58-b17-tactical-command-voice/README.md).
