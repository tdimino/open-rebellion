# Original-game logging and speedup: research report (2026-10-16)

Researched with Exa and Firecrawl web search. Nothing in the repository or the VM was changed.

## 0. Corrections to the premise, and facts that decide the ranking

1. **The guest is Windows 11 Arm64, not x86 under TCG.** The README installs
   `Win11_25H2_English_Arm64.iso`, and the x64 `qemu-ga` runs "under Windows' x64 emulation".
   So QEMU/UTM runs an Arm64 guest (presumably under HVF; confirm in `config.plist`), and
   `REBEXE.EXE` (x86) is translated inside the guest by Windows' own emulator (Prism in 24H2+, "JIT
   compiling blocks of x86 instructions into Arm64", per
   https://learn.microsoft.com/en-us/windows/arm/apps-on-arm-x86-emulation). Consequences:
   - QEMU gdbstub and QMP memory commands see Arm64 guest state and translated code, not x86
     state. They cannot give you "read x86 global at 0x6b1248". Outside-in memory polling is the
     wrong tool; reading from inside the guest (an in-process DLL, or `ReadProcessMemory` from a
     helper) is right. The x86 process's memory is ordinary guest user memory at the same virtual
     addresses, and REBEXE loads at 0x400000 (1998 exe, no ASLR), so Ghidra VAs are runtime VAs.
   - Microsoft states that "some performance features within Prism require hardware features only
     available in the Snapdragon X series". An M4 guest likely lacks those, which would explain a
     slow Fast speed (unverified; the experiment below measures where the time goes).
2. **Debuggers on x86-under-Arm64 are unreliable.** x64dbg on Windows 11 Arm gets wrong register
   context (https://github.com/x64dbg/x64dbg/issues/2923); a reverse-engineering Q&A reports
   `STATUS_INSTRUCTION_MISALIGNMENT` then access violations when debugging any x86/x64 GUI program
   under Windows on Arm, answer "not possible, you debug the JIT"
   (https://reverseengineering.stackexchange.com/questions/32087). Cheat Engine "doesn't work
   correctly on Windows 11 ARM64 via built-in translator"
   (https://github.com/cheat-engine/cheat-engine/issues/3016). Both rank last.
3. **In-process inline hooking of x86 code does work on this stack.** Detours issue 292: "There is
   no problem with win32 API hooking of x86 and x64 processes in the ARM64 environment"; only a COM
   vtable hook crashed (0xc000001d) (https://github.com/microsoft/Detours/issues/292). Our own VM
   already runs DDrawCompat (a hooking wrapper) inside REBEXE's process. Frida 16.5 added Windows
   on Arm: "an arm64 version of Frida can inject into native arm64 processes, as well as emulated
   x86_64 and x86 processes" (https://frida.re/news/2024/09/06/frida-16-5-0-released/). Frida
   17.23.3 (2026-10-09) ships `frida-server`/`frida-inject`/`frida-gadget` for windows-arm64 and
   windows-x86 (checked with `gh api repos/frida/frida/releases/latest`).
4. **Time APIs the game uses** (our Ghidra notes): `timeGetTime` in ~15 functions (e.g.
   `FUN_0040d7a0`, `FUN_004115c0`, `FUN_005bea30`, `FUN_005a7500`), `timeSetEvent` in the timer
   class (`FUN_00601f40`, `FUN_00602060`, callback `fptc_006020d0`), `GetTickCount`
   (`FUN_005f8b80`, `FUN_004080c0`), `QueryPerformanceCounter` (HiResTimer, `FUN_005fbbe0`),
   `SetTimer` (`FUN_00410c70`, `FUN_004aab50`). Speed intervals 1:600, 2:60, 3:12, 4:4 "ticks"
   (rebexe-map.md l.262) are the speed row; which API drives the day clock is not yet identified.
5. **Only one `OutputDebugStringA`** in the exe (`FUN_004c0870`, "Processing Invalid Command"), so
   DebugView gives almost nothing.

## A. Logging the original's internals

| # | Option | What / how it applies | Effort | Risks | Rank |
|---|---|---|---|---|---|
| A1 | **In-process DLL (proxy DLL or ASI loader) with MinHook/SafetyHook, written in C, built on the Mac** | Drop a 32-bit `winmm.dll` (or `version.dll`/`dinput8.dll`) next to `REBEXE.EXE`, loaded before the game. Ultimate ASI Loader supports ddraw/dinput/dinput8/version/winmm names, loads `.asi` plugins, and offers inline, mid-function and VMT hooks via SafetyHook (https://github.com/ThirteenAG/Ultimate-ASI-Loader; MinHook https://github.com/TsudaKageyu/minhook, v1.3.4). Plugin hooks `0x005d19a0` etc. (base 0x400000, so Ghidra VA = runtime VA), logs `ecx`/args/return to a file under `C:\a0\` or `Z:\`. A mid-hook captures registers, so thiscall `this` (ecx) and stack args come free. Also hook `timeGetTime`/QPC/`timeSetEvent` for A1+B. Cross-compile with `zig cc -target x86-windows-gnu` or `brew install mingw-w64` (neither installed now). The same DLL runs unchanged under Wine (`WINEDLLOVERRIDES=winmm=n,b`). | M: 1 day for loader + logger skeleton, then hours per hook | Prism and patched code pages (Detours 292 COM crash); `winmm` not in KnownDLLs so side-loading should work, verify with the import table; DDrawCompat already occupies `ddraw.dll`, so use another name; hooks perturb timing but not sim state | **1** |
| A2 | **State poller inside the same DLL** (no code patching) | A thread or the `timeGetTime` hook reads known globals directly (day counter, `DAT_006be3b8` GameOptions, entity lists via the vtable anchors) and appends a JSONL line on each day change. Zero instruction patching, so no Prism risk. Fleet record layout (320 bytes, vtable anchor 0x0066DCA8, arrival_date +0x24, day_counter at GameState +0x20) is documented, unverified, in an open PR to StarWarsRebellionEditor.NET (https://github.com/MetasharpNet/StarWarsRebellionEditor.NET/pull/3: `RUNTIME_MEMORY_ANALYSIS.md`, `FLEET_RECORD_STRUCTURE.md`); cross-check against our own save decode before trusting it (the PR looks machine-written). | S once A1 exists | Reading mid-update gives torn state: sample on the day-change hook, not on a free-running thread | **2** (do first inside A1) |
| A3 | **Frida** (gadget DLL or `frida-server`) | `Interceptor.attach(Module.findBaseAddress('REBEXE.EXE').add(0x1d19a0), …)`; `frida-trace -a "REBEXE.EXE!0x1d19a0"`; JS iterates fast and logs with `send()`. Windows-Arm64 server/inject exist, and 16.5 notes say it injects into emulated x86. The x86 `frida-gadget-17.23.3-windows-x86.dll` can be side-loaded by a proxy DLL, removing the Arm64-to-x86 injection step. Docs: https://frida.re/docs/examples/windows/, https://frida.re/docs/frida-trace/. Note issue #3393 (pip install of frida-tools on Win11 Arm failed, Mar 2025, open) so install from release binaries, not pip. Output goes to a file via a `send` handler or to the host over the server's TCP port (the VM has no NIC, so file plus guest agent `pull`). | S-M to prove, then the fastest iteration loop | Unproven on our exact guest; Stalker (instruction tracing) probably unusable under JIT; Frida's thread suspension may disturb Prism; no network device | **3** (best for exploration, fall back to A1 for production) |
| A4 | **Microsoft Detours** | Same as A1 with Microsoft's library (https://github.com/microsoft/Detours). Proven for Win32 API hooks on Arm64-hosted x86 processes, MSVC-centric build, heavier than MinHook. | M | COM crash report; needs MSVC or clang-cl | 5 |
| A5 | **QEMU monitor/gdbstub/QMP memory reads** | Arm64 level only; QEMU's HVF gdbstub supports guest debugging on Apple Silicon (https://github.com/qemu/qemu/commit/eb2edc42b12683fdbe54003db7701f7ab6cda036) but you would translate x86 VA to Arm64 pages by hand and the JIT cache is opaque. QMP is already loopback-only in our VM. | L | Not meaningful for x86 state | 8 |
| A6 | **x64dbg/x32dbg logging breakpoints and scripts** | Would be ideal on x86 hardware (Ghidra import plugins exist), but see §0.2 | M | Broken contexts under Prism | 7 |
| A7 | **Cheat Engine Lua** | See §0.2 | M | Broken on Arm64 | 9 |
| A8 | **API Monitor, ProcMon, Wine `WINEDEBUG=+relay`** | Good for import-level questions (is `winmm` side-loadable, which DLLs load, which registry keys). API Monitor/ProcMon are x86-hosted tools, ProcMon has an Arm64 build. No game-internal visibility. | S | Low value for AI/rolls | 6 |
| A9 | **OutputDebugString/DebugView** | Only one call in the exe | none | No data | 10 |
| A10 | **Patch REBEXE.EXE itself** (code cave that appends to a log) | Works with no loader, but needs hand-assembled x86 and changes the exe hash (the capture rig records it) | L | Breaks the "unmodified original" claim | 11 |

Getting logs out: the rig already has `save-out`/`gexec`/guest-agent `pull` and the `Z:` WebDAV share
(README: "UTM stops serving the folder after restart", `up` re-arms it). Write logs to `C:\a0\logs\`
and `pull` them; avoid `Z:` for high-rate writes. Flush per line.

## B. Speeding the original up

| # | Option | What / how it applies | Effort | Risks | Rank |
|---|---|---|---|---|---|
| B1 | **First measure where the time goes** | Fast is "about 0.8 days/s". Speed 4 is 4 "ticks" in the exe, so the pacing may be CPU or render bound (emulated Prism, DDrawCompat scaling, 8-bit palette blits, animations) rather than timer bound. If CPU-bound, stretching time does nothing. Use the A1 timer hook to log calls/second of `timeGetTime` and the callers (backtrace of return addresses) to find the day-clock driver. | S | none | **1** |
| B2 | **Time hooks in the A1 DLL** (scale `timeGetTime`, `GetTickCount`, QPC, shorten `timeSetEvent` delay) | Same technique Cheat Engine's speedhack and DxWnd use. DxWnd's relnotes document time stretching hooks for `GetTickCount`, `timeGetTime`, QPC/QPF and window timers (https://github.com/DxWnd/DxWnd.reloaded/blob/master/build/readme-relnotes.txt); it does not list `timeSetEvent`, which REBEXE uses. Run Fast at 4x-8x. | S after A1 | Determinism: the sim is day-stepped and RNG is a Lehmer generator seeded from system time, so stretching changes pace, not outcome, unless the AI/animation code reads wall time (several `timeGetTime` callers exist: audit them); mouse/UI timeouts also stretch | **2** |
| B3 | **Patch the speed-interval table in memory** (1:600, 2:60, 3:12, 4:4) | Locate the table with Ghidra (rebexe-map.md l.262, `FUN_004011a0`, speed word at `DAT_006b1470` and target+0xc4 via message 0x280), write 1 or 2 from the ASI plugin's `InitializeASI`. Cheapest if the day clock is timer paced; worthless if CPU bound. | S | Same as B2 | 3 |
| B4 | **Disable rendering and animation work** | Registry `DisplayOpts` (dword, e.g. 0x13110100, read at `FUN_00413000`; display opts at GameOptions+0x140) probably holds the animation/sound toggles; decode bits in Ghidra, then test days/s with animations off. Also skip briefings: rename `ALBRIEF.DLL`/`EMBRIEF.DLL` to `.OLD` ("skips new game's 5 minute briefings", https://github.com/MetasharpNet/StarWarsRebellionEditor.NET README, "Fast game start"). Minimise or hide the window only after checking the game keeps ticking. | S | Skipping briefings changes new-game state setup; fine for saves already past it | 4 |
| B5 | **Wine/CrossOver on the Mac under Rosetta** | Removes the VM and Prism: native-speed x86 JIT with hardware TSO. Our Mac already runs Sikarugir/Wine for SWG. Wine's WoW64 build runs 32-bit Windows binaries on macOS 64-bit-only (https://news.ycombinator.com/item?id=43632242). The A1 DLL works unchanged. WineHQ AppDB rates Rebellion Bronze, with historic "tactical battle crashes, no text" reports (https://appdb.winehq.org/objectManager.php?iId=181&sClass=version), Wine versions from 2008-2014, so stale. DDrawCompat does not run on Wine; use Wine's ddraw or dgVoodoo (the community patches ship dgVoodoo files). x87 code is slow under Rosetta, though Rebellion's sim is mostly integer (unchecked). | M-L | A0 frame exactness is not preserved (different renderer): use only for logging and speed runs, keep the VM for A0; tactical 3D may crash | 5, but potentially the biggest win for "hundreds of days"; worth a 1-hour spike |
| B6 | **DxWnd** | Injects its own DirectDraw hooks and time stretching; conflicts with DDrawCompat in the same process and is a Windows tool (x86 hosted, so Prism again) | M | Conflicts, window-mode rendering differs | 7 |
| B7 | **Speed hacks via Cheat Engine / kernel** | CE broken on Arm64 (§0.2) | - | - | 9 |
| B8 | **Tune the VM** | More vCPUs, pin priority, disable Spotlight on the Mini (README already saw a 336 s vs 360 s drift), `DDrawCompat.ini` already point filter. HPET/Hyper-V enlightenment advice (https://github.com/utmapp/UTM/issues/4914) applies to x86 guests, not ours. Expect single-digit percent. | S | - | 6 |
| B9 | **Native x86 machine (Windows on bare metal or a cloud x86 VM)** | Removes Prism and TCG; GOG's speed-issue page also says run on one core / high-performance power plan (https://support.gog.com/hc/en-us/articles/213039445). A cloud x86 Windows VM (e.g. an EC2 or Azure instance) is a one-time cost and runs the game at full speed; captures would move over the network. | M | Cost, copyrighted game data on a cloud box (we own the game; saves are already committed) | 4 |
| B10 | **Call the day-advance function in a loop from the DLL** | Find the sim step function in Ghidra and invoke it from a hooked timer callback with rendering skipped: potentially 100x. | L | Re-entrancy, UI desync, undefined state; do last | 8 |

No documented speed switch exists: the community only documents *in-game* speeds. The GOG/Steam
guides list `-w` (windowed) and `-fullscreen` (GOG v4 build) as command-line arguments
(PCGamingWiki https://www.pcgamingwiki.com/wiki/Star_Wars:_Rebellion; README of the Editor), the
registry keys under `HKLM\SOFTWARE\WOW6432Node\LucasArts Entertainment Company LLC\Star Wars
Rebellion` (DisplayOpts, MusicVolume, Volume, locations), and the "Hyperspace Speed" values in
GNPRTB.DAT (travel speed of non-ship units, default 100/0x64, Han's modifier 50/0x32;
https://swrebellion.net/forums/topic/8780-hyperspace-speed-locations/), which change game
mechanics, not the wall-clock pace. The game has no ini file and no documented debug mode, cheat
codes or logging switch in anything found.

## C. Existing projects

- **MetasharpNet/StarWarsRebellionEditor.NET** (https://github.com/MetasharpNet/StarWarsRebellionEditor.NET): editor plus 25th Anniversary Patch ("reverse engineering and exe patching" by Metasharp and Hexdragon); the community knows the exe well enough to patch it. Open PR #3 documents runtime memory with pymem and Frida (see A2). Contact path: their Discord (link in README).
- **TheArchitect2018/Deep-Dive-into-SW-Rebellion-PC-Game-Internals** wiki (https://github.com/TheArchitect2018/Deep-Dive-into-SW-Rebellion-PC-Game-Internals/wiki): file-format notes for the GData DAT tables and saves, with `sub_557330` pages (a function-level page); no runtime logging tool.
- **davidadas/rebellion2** (https://github.com/davidadas/rebellion2): Unity remake, no original-game instrumentation found.
- **swrebellion.net** (RebED, "Rebellion Fix", patches, editing forum): DAT editing, no speed or debug tooling. "Rebellion Fix 2.63.1.2" installs DDraw fixes (installer flagged by antivirus; its contents were not reviewed).
- **DDrawCompat** (https://github.com/narzoul/DDrawCompat): already in the VM; open source and a reference for hooking x86 code under Prism in this exact guest. It has its own logging (`LogLevel`), which logs DirectDraw calls (useful for timing blits), not game logic.
- **Reusable generic projects**: Ultimate ASI Loader, MinHook, SafetyHook, Frida, DxWnd. No Rebellion-specific tracer or other 1998 DirectDraw strategy-game tracer turned up; searches on Cheat Engine tables and trainers returned only the unrelated 2012 "Sins of a Solar Empire: Rebellion".

## Recommended plan

1. **Spike (1-2 h): prove hooking works at all.** Build a 32-bit `winmm.dll` proxy that forwards to the real `winmm.dll` (or use the Ultimate ASI Loader named `winmm.dll`), logs `timeGetTime` caller return addresses and calls per second to `C:\a0\logs\time.log`, installed next to `REBEXE.EXE` by `vm-game.sh`'s push/gexec. This proves load order, build chain and log retrieval in one go.
2. **Quick experiment (the value proof): scale `timeGetTime`, `GetTickCount` and the `timeSetEvent` delay by 4 in that DLL**, load the day-5 transit save, set Fast, and compare days per second with and without (sleep, pause, read the day from the save as the README does). Outcomes: (a) rate rises about 4x, the clock is timer paced, B2 is the answer; (b) rate unchanged, the game is CPU/render bound: go to B4 (animations off) and the B5/B9 platform spikes. Either result is decisive in under an hour of VM time, and the log also names the day-clock driver function for Ghidra.
3. **Day-boundary state dumper (A2):** on the clock hook, dump the game-state, fleet, agent and mission records (layouts from our saves/Ghidra, cross-checked with the PR) to JSONL on each day change. This answers "what did the AI decide" without patching code.
4. **Function tracers (A1 hooks or Frida A3):** entry/exit logs with `ecx` and stack args for the battle system `FUN_005d19a0`, the AI evaluator, event dispatch, and the RNG (hook the Lehmer generator, multiplier 0xa18e, to get every roll with its caller). Use Frida for exploration, port hot hooks to the C DLL.
5. **Platform spike (only if step 2 says CPU bound): Wine/CrossOver under Rosetta on the Mac with the same DLL**, 1 hour, headless day-advance only; keep the VM for A0 frames. Use a native x86 cloud VM as the fallback.

REBEXE's import table names `WINMM.dll` (`timeGetTime`, `timeSetEvent`, `timeBeginPeriod`, `timeEndPeriod`, `timeKillEvent`, `timeGetDevCaps`, two `mci` calls), KERNEL32 (`GetTickCount`, `QueryPerformanceCounter`/`Frequency`, `Sleep`, `SleepEx`, `WaitForSingleObject`, `WaitForMultipleObjects`, `GetLocalTime`, `GetSystemTime`, `GetTimeZoneInformation`) and USER32 (`SetTimer`, `KillTimer`); its image base is 0x00400000 with relocations stripped (file Characteristics 0x10f). Open verifications before step 4: confirm `winmm.dll` is not a KnownDLL on the guest, confirm Prism tolerates inline patches of the exe's own code (DDrawCompat suggests yes for DLL exports), and confirm the guest's QEMU accelerator.
