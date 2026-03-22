# COMMON.DLL Analysis

**Verdict: Pure resource container. No game logic. No Ghidra decompilation needed.**

## Summary

| Property | Value |
|----------|-------|
| File size | 2,988,544 bytes (2.9 MB) |
| Code section (.text) | 13,312 bytes (0.4%) |
| Resource section (.rsrc) | 2,953,728 bytes (98.8%) |
| Exported functions | **0** |
| Imported DLLs | KERNEL32.dll only (47 CRT functions) |
| Game logic | **None** |

Prior documentation incorrectly described COMMON.DLL as a "DirectPlay networking" DLL or "MFC/Win32 shared library." DirectPlay lives in REBEXE.EXE (address range `0x5a0000–0x5b0000`), which imports `DPLAYX.dll` directly.

## How REBEXE.EXE Loads It

COMMON.DLL is **not** in the PE import table. REBEXE.EXE loads it at runtime as **module ID 8** in the game's resource module system:

```c
// FUN_005ff020 — resource module loader
hModule = LoadLibraryA("common.dll");  // module_id = 8
// Then accessed via:
FindResourceA(hModule, resource_id, RT_BITMAP);
```

### DLL Resource Module Registry

| Module ID | DLL | Content |
|---|---|---|
| 1 | textcomm.dll | Common text strings |
| 3 | texttact.dll | Tactical text strings |
| 5 | encybmap.dll | Encyclopedia bitmaps |
| 6 | tactical.dll | Tactical view sprites |
| 7 | strategy.dll | Strategy map sprites |
| **8** | **common.dll** | **Core UI widgets** |
| 10 | gokres.dll | GOK (game object kit) resources |
| 12 | Rebdlog.dll | Dialog resources |

## Contents

### 321 BMP Resources (2,893,312 bytes)

- **215 button sprites**: Scrollbar arrows, checkboxes, radio buttons, sliders, tab controls
- **4 full-screen backgrounds** (640x480): Main menu, loading screens
- **2 title screens**: Game logo + LucasArts splash
- **~100 UI widget elements**: Borders, frames, separators, progress bars
- **3 named resources**: `COCKPIT_BUTTON_GAMESCALE` (1–3) — speed control buttons

### 5 WAV Resources (42,850 bytes)

UI interaction sounds: button clicks, notification chimes.

### 1 RCDATA Resource (20 bytes)

Widget animation descriptor linking bitmap IDs 10150–10157 as an 8-frame animation sequence.

### 1 RT_VERSION Resource (904 bytes)

LucasArts build metadata. `OriginalFilename` field says "REBEXE.EXE"—a build template artifact (the DLL was compiled from the same project).

## Code Section (13,312 bytes)

~7 functions, all MSVC CRT boilerplate:
- `_DllMainCRTStartup` (entry point)
- Heap initialization
- TLS callbacks
- Runtime error handlers

**Zero game logic. Zero DirectPlay. Zero MFC.**

## Status in Open Rebellion

| Asset | Status |
|-------|--------|
| 321 BMPs | Already extracted to `assets/references/ref-ui-full/common-dll/` |
| 5 WAVs | Not yet extracted (42 KB total — low priority) |
| Code | Not needed — no game logic to port |

## Corrections to Prior Documentation

| Prior Claim | Correction |
|---|---|
| "Handles DirectPlay networking" | DirectPlay is in REBEXE.EXE, imports DPLAYX.dll |
| "MFC/Win32 shared library" | No MFC imports, no exports, pure resource container |
| "May contain shared combat utilities" | 7 CRT functions total, zero game logic |
