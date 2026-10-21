# Original capture rig (A0 evidence)

Tooling for producing lossless 640x480 captures of the unmodified original
`REBEXE.EXE` so the interface audit can register A0 references and the harness
can create goldens. Path 1 of
[the P59 plan](../../docs/plans/archive/2026-09-14-feat-p59-parallel-gid-message-index-lane.md).

Captures are copyrighted game output and are never committed. Only their
SHA-256, metadata, and the accepted-original registration enter the repo.

## Host

The guest runs in UTM 4.7.5 on a Mac Mini (Apple M4, macOS
15.6) under `~/VMs/rebellion/`:

| File | Purpose |
|---|---|
| `Win11_25H2_English_Arm64.iso` | Microsoft multi-edition Arm64 installer (sha256 `638aa2c8…adf0`) |
| `unattend.iso` | ISO 9660 image of `vm/autounattend.xml` and `vm/go.cmd` (built with `hdiutil makehybrid -iso -joliet`) |
| `payload.img` | 2 GiB raw FAT32 volume **with an MBR partition table** (`hdiutil create -layout MBRSPUD -fs "MS-DOS FAT32"`) holding the owned GOG game folder, `vm/install.cmd`, and `vm/capture.ps1`; captures are written back to it |
| `utm-guest-tools-0.1.273.iso` | kept on disk but **not attached** (see below) |
| `webdav-tools.iso` | the pieces taken from the guest tools for the shared folder and guest agent: the Arm64 virtio-serial driver, the Arm64 VC++ runtime, `spice-webdavd-arm64-latest.msi`, `qemu-ga-x86_64.msi` and `webdav.cmd`, which installs the first three |
| `exchange/` | the SPICE WebDAV shared folder (`saves-in/`, `saves-out/`, `captures/`); the guest sees it as `Z:` |
| `vm-ctl.scpt`, `vmctl-run.sh` | [`vm/vm-ctl.applescript`](vm/vm-ctl.applescript) compiled, and [`vm/vmctl-run.sh`](vm/vmctl-run.sh), which runs one of its commands through the GUI launch agent |
| `qemu-img` | a stub that runs UTM's bundled qemu-img, which ships as a library (`clang -o qemu-img stub.c -Wl,-rpath,/Applications/UTM.app/Contents/Frameworks`, the stub `dlopen`s the framework and calls its `main`) |

The VM bundle has no backup copy; it is rebuilt from the files above.

Two facts about Windows Setup shape this layout, both established on
2026-09-14 by watching Setup fall through to its interactive pages:

- Setup's implicit `autounattend.xml` search covers removable media only.
  QEMU's `usb-storage` presents image files as fixed disks, so an answer
  file on a FAT image is never found. A CD is removable read-only media and
  is searched, hence `unattend.iso`.
- The UTM guest tools ISO ships its own `autounattend.xml`; when attached it
  is found first. Attach it only after the install. Its full installer also
  brings vdagent, the network driver and the GPU drivers; we install only
  the pieces in `webdav-tools.iso` ("VM configuration", below).
- A FAT volume without a partition table (an `hdiutil` "superfloppy") is
  invisible to Windows on a fixed disk. `payload.img` therefore carries an
  MBR.

UTM needs a logged-in desktop session on the host, and its AppleScript
bridge refuses callers from an SSH session (TCC error -1743, no prompt). The
working pattern is a user launch agent bootstrapped into the GUI domain
(`launchctl bootstrap gui/$(id -u) ~/VMs/rebellion/com.openrebellion.vmctl.plist`;
no sudo is needed over SSH), whose `osascript` call raises the
Automation prompt on the host display; approve it once over Screen Sharing.
The host is set to log in automatically (`/etc/kcpassword` +
`autoLoginUser`) so a reboot restores the session. The VM is created with:

```sh
osascript vm/create-vm.applescript \
  ~/VMs/rebellion/Win11_25H2_English_Arm64.iso \
  ~/VMs/rebellion/unattend.iso \
  ~/VMs/rebellion/payload.img \
  <host-ip>:1
```

The last argument binds a QEMU VNC display to the host's Tailscale address so
the guest can be driven from another machine with `vncdo` (never bind to
`0.0.0.0`). Drives: two USB CD-ROMs, one 64 GiB NVMe system disk (Windows
Arm64 ships an NVMe driver, so setup needs no virtio storage driver), and the
raw FAT USB volume. UTM copies `payload.img` into the VM bundle
(`~/Library/Containers/com.utmapp.UTM/Data/Documents/Rebellion A0 Capture.utm/Data/`);
captures land in that copy, not in `~/VMs/rebellion/`.

VM configuration:

- No network device (`Network` is `[]` in `config.plist`) and clipboard
  sharing off, so the guest has no network.
- A loopback-only QMP socket for `vm-qmp.py`, which walks the pointer
  (`QEMU.AdditionalArguments`: `-qmp tcp:127.0.0.1:4446,server=on,wait=off`).
  Never bind it to the Tailscale address or `0.0.0.0`: QMP is an
  unauthenticated control channel.
- The SPICE WebDAV shared folder: UTM's `update registry` AppleScript sets
  `~/VMs/rebellion/exchange` (`share PATH` in `vm-ctl.applescript`), and
  `spice-webdavd` in the guest maps it to `Z:`. UTM stops serving the
  folder after the VM starts although its registry still names it; setting
  it again right after a start brings it back, and `vm-game.sh up` does so
  when it started the VM. Never set it on a VM that was already running:
  that leaves the guest agent's RPC timing out until the VM is powered off.
  Its install needed the
  virtio-serial driver and the VC++ runtime; `webdav.cmd` runs all three.
  UTM's QEMU sandbox refuses to open a new image (`change` on a fresh path
  fails with "Operation not permitted"), so the tools ISO was written over
  `unattend.iso` in place, loaded with `change`, and the original image put
  back afterwards.
- The QEMU guest agent: the x64 `qemu-ga` runs under Windows' x64 emulation
  (UTM has no Arm64 build, UTM issue 5134). Through UTM's `execute`, `pull`
  and `push` it passed 11 command runs with the right exit code, stdout and
  stderr, 10 MiB and three 2 MiB round trips with matching SHA-256, and a
  reboot. It runs as a service in session 0, so it cannot see or drive the
  desktop. When it connects, UTM sets the guest clock to the host's.

The day-31 alert from the day-5 transit save arrives 360-362 s after the
load; stopping `spice-webdavd` does not change it.

The VM must carry a sound card. `REBEXE.exe` calls `DirectSoundCreate` during
startup (`FUN_00610d00`), and with no audio device the returned interface is
unusable; the game then faults dereferencing it (near `FUN_00611140`; address
unverified from our Ghidra project, exception `0xc000041d`) before a window ever appears. `create-vm.applescript` requests no
sound, so after the VM exists, set one while it is stopped:

```sh
plutil -replace Sound -json '[{"Hardware":"intel-hda"}]' \
  "$HOME/Library/Containers/com.utmapp.UTM/Data/Documents/Rebellion A0 Capture.utm/config.plist"
```

The guest is Windows 11 Arm64, so the x86 `REBEXE.EXE` runs under Windows'
own emulator (Prism). Outside tools see Arm64 state and translated code:
QEMU's gdbstub and monitor, x64dbg and Cheat Engine cannot follow the game's
x86 registers. Instrumentation has to run inside the game's process (a proxy
DLL; [the logging and speed-up report](../../docs/reports/2026-10-16-original-game-logging-and-speedup.md)).
The exe's image base is 0x00400000 and its file header has relocations
stripped (Characteristics 0x10f), so a Ghidra address is the runtime address.
Its clocks, waits and timers are imported: from WINMM `timeGetTime`,
`timeSetEvent`, `timeKillEvent`, `timeBeginPeriod`, `timeEndPeriod` and
`timeGetDevCaps`, from KERNEL32 `GetTickCount`, `QueryPerformanceCounter`,
`QueryPerformanceFrequency`, `Sleep`, `SleepEx`, `WaitForSingleObject`,
`WaitForMultipleObjects`, `GetLocalTime`, `GetSystemTime` and
`GetTimeZoneInformation`, from USER32 `SetTimer` and `KillTimer`.

Driving the guest: QEMU's VNC server does not synthesise the shift modifier,
so `vncdo type` corrupts symbols (`:` arrives as `;`, `&` as `7`). Use
`vnc-type.py <host::port> "<text>" [vncdo actions…]`, which sends shifted
symbols as `shift-<key>` chords. The firmware's "press any key to boot from
CD" window is short; send keys at 0.5 s intervals starting immediately, and
if the firmware lands in the UEFI shell, run `fs0:` then
`efi\boot\bootaa64.efi` and press keys at once.

## Guest install

`vm/autounattend.xml` runs the whole Windows install: it bypasses the TPM,
Secure Boot, RAM, and CPU checks in `LabConfig`, wipes disk 0 into
EFI/MSR/NTFS, installs "Windows 11 Pro" with the generic client key, skips
every OOBE page including the online-account requirement (`BypassNRO`),
creates the local `capture` account with automatic logon, and disables sleep,
screen saver, and display scaling on first logon. The only manual step is
pressing a key when the firmware offers to boot from the CD. If Setup ever
shows its language page anyway, the answer file was not found: open the
WinPE prompt with Shift+F10 and run `go.cmd` from the UNATTEND drive, which
relaunches `setup.exe /unattend:` with the file on its own drive.

After first logon, from an elevated prompt on the payload drive (usually `D:`):

```bat
D:\install.cmd
```

This mirrors `goggame-1421404828.script`: the game goes to `C:\Rebellion`,
GOG's `d3drm.dll`, DDrawCompat `ddraw.dll`, and its ini are applied, the six
LucasArts registry values are written, the GOG compatibility layer flags are
set on the executable, and DirectPlay is enabled. The DDrawCompat ini GOG ships
contains only `CpuAffinity=all` and `LogLevel=none`, so DDrawCompat's defaults
apply. One of them, `DisplayResolution=desktop`, scales the full-screen game's
640×480 mode to the 800×600 desktop, the only mode Windows lists for the
virtio-ramfb display. `a0-setup` adds `DisplayFilter=point` so that scaling
can be undone exactly (see "A0 captures"); GOG's ini is kept as
`C:\a0\DDrawCompat-REBEXE.ini.orig`.

`REBEXE.exe` is 32-bit, so on 64-bit Windows it reads its data locations from
the `WOW6432Node` view of `HKLM\Software`. `install.cmd` writes the six values
with `/reg:32`; a plain `reg add` lands in the 64-bit view and the game reports
"CD Not Found".

## Capture

```powershell
powershell -ExecutionPolicy Bypass -File D:\capture.ps1 -Name alliance-shell -Out D:\captures -Launch C:\Rebellion\REBEXE.exe
```

`capture.ps1` locates the game window (by the `REBEXE` process first, so a
console whose title quotes the launch command is never mistaken for it), reads
its client rectangle through user32, copies exactly those device pixels with
GDI, and keeps a frame only after two consecutive captures hash-equal. It
writes `<name>.png` and `<name>.json` (sha256, size, client origin, screen
size, Windows build, timestamp, method). Captures come from the guest's own
framebuffer, never from the host window or a VNC client.

For a tactical A0 cell, include its exact ledger ID and requirement, the setup
provenance, and the ordered input trace. The script also hashes the running
executable:

```powershell
& D:\capture.ps1 `
  -Name TAC-01-C001 -Out D:\captures `
  -CellId TAC-01-C001 -Requirement Alliance `
  -StateSetupKind original-save -StateSetupIdentifier alliance-battle-01 `
  -StateSetupNotes "Owned English installation" `
  -InputTrace @("load alliance-battle-01", "enter tactical battle", "wait for stable frame")
```

When `-CellId` is supplied, the script rejects missing requirements, setup
identifiers, and input traces. It still accepts ordinary non-matrix captures
without those tactical parameters.

Launch the game and `capture.ps1` from an elevated prompt: the GOG compat
layer sets `RUNASADMIN`, and a child started from an already-elevated shell
inherits elevation without a UAC prompt. The game window is always-on-top, so
it covers the console it was launched from; the capture console keeps keyboard
focus even while hidden, so commands can be typed into it blind, and
`capture.ps1` brings the game forward itself before each frame. `vnc-type.py`
sends shifted symbols as explicit `shift-<key>` chords, which the QEMU VNC
server does not synthesise on its own.

## A0 captures

VNC frames and `cap` are navigation aids, never A0. The A0 route runs the
game full screen and captures inside the guest:

```bash
tools/original-capture/vm-game.sh up-a0     # tasks: watcher, then REBEXE full screen
tools/original-capture/vm-game.sh click 535 315          # main menu: Saved Games
tools/original-capture/vm-game.sh click 384 376          # load slot 6
tools/original-capture/vm-game.sh wait alert 1 60 /tmp/alert.png
tools/original-capture/vm-game.sh take-command
tools/original-capture/vm-game.sh a0-cell TAC-01-C003 original-save \
  empire-day30-yaga-alert-control 'load slot 6|Battle Alert|Take Command|no further input'
```

- With `DisplayFilter=point`, DDrawCompat scales 640×480 to 800×600 by
  repeating whole pixels: every column with `x % 5 == 3` and 120 fixed rows
  (`DupRows` in `capture.ps1`) copy their neighbour. The pattern was
  calibrated on two different frames on 2026-10-11. `capture.ps1` copies the
  whole screen, checks every repeated column and row equals its neighbour
  (any difference refuses the frame), and drops them, which gives the game's
  640×480 frame exactly. It writes that as `NAME.png` and the source as
  `NAME.screen.png`, records both hashes and the ini's hash, and refuses a
  cell frame unless the screen is 800×600 and the ini has
  `DisplayFilter=point`. Two captures of one paused frame hash-equal.
- The scaling phase is chosen when the game launches, apparently at
  random. Some launches repeat columns 2 and 7 instead of 3 and 8 (and 20
  columns and 21 rows in all sit one pixel off), so A0 refuses every frame:
  "column 3 differs from column 2 … not the calibrated point scaling". Of
  four launches in one session, two came up shifted on the main menu. A
  launch keeps its phase through loads and play. `a0-game` therefore probes
  the main menu after each launch (`a0 phase-probe DIR FullClient=1`; the
  probe PNG keeps the full screen) and relaunches until the repeated
  columns are 3, 8, 13, up to `A0_PHASE_TRIES` launches (default 5; 0 skips
  the check). Never rebuild a shifted frame. Every dropped line would still
  verify, but the 640×480 bytes would differ from the calibrated phase's
  for the same game frame.
- A burst cannot be timed against an input sent separately: the request
  takes several seconds to reach the watcher, so an unpause sent "4 s after
  starting the burst" landed before its first frame, and the superlaser's
  hit was missed twice. `capture.ps1` writes `out\NAME.started` once it has
  taken frame 0, and `A0_ON_START="click 717 397"` makes `a0` (and
  `a0-cell`) wait for that file and only then send the command. That round
  trip still takes over 10 s, which put the unpause after a 12 s burst. For
  a click, use `ClickOnStart=1` instead. The guest then presses and releases
  the left button right after frame 0, with no cursor motion, so first park
  the cursor on the control with `A0_PARK="717 397"`. On a running battle,
  a 2-frame, 500 ms test showed it paused by frame 1. A burst is 2–60
  frames and at most 30 s.
- `run-to-alert TITLE.png TIMEOUT FILE` lets the game run until one
  particular Battle Alert opens and auto-resolves every other one on the
  way. It recognises an alert by its three-button bar, compared with
  `chrome.png` beside TITLE.png, then compares the title band with
  TITLE.png. Both files are full 800×600 frames of an alert, kept in
  `.artifacts/vm/alerts/`. On stored frames, the bar differs by at most 2.1
  between alerts and at least 66 on other screens. The title differs by at
  most 0.5 for the same battle and at least 14.9 for another. A VNC refresh
  that times out reconnects instead of ending the wait (`vm-wait.py`).
- Game speed is the day readout's right-click menu (game coordinates on
  the 800×600 screen: readout 165,30; Pause 208,45, Very Slow 230,70,
  Medium 220,120, Fast 208,145). Playing the Empire, the readout is at
  645,30 and its menu opens to the right: Pause 668,45, Very Slow 710,70,
  Fast 695,137.
  The game's own accelerators step the speed one notch (Alt+NumPad−
  slower, Alt+NumPad+ faster) and Alt+P pauses (manual p. 64;
  `docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-24-game-speed-recovery.md`,
  "Speed keys"; verified on the VM on 2026-10-16).
  VNC cannot send the keypad keys, so send them through the QEMU monitor:
  `tools/original-capture/vm-game.sh mon 'sendkey alt-kp_subtract'` (also
  `alt-kp_add`, `alt-p`). A battle result window holds the day, and a
  step sent while it is up took effect once in three presses, so send one step
  per call and check the day readout. Fast runs about 0.8 days a second, faster
  than `wait day` can sample: a 33-day wait ran 340 days. To cover a set
  number of days at Fast, sleep for the time and then pause. Pause opens
  "Resume Game Play?", whose check mark (398,373) resumes at the kept
  speed. Game Options holds the clock; the `save` verb closes it after
  saving (Escape, then its galaxy icon at 228,490).
- QEMU snapshots (`savevm`) are not possible on this VM. The Windows disk
  is on QEMU's NVMe controller, which cannot save its state ("State blocked
  by non-migratable device '0000:00:05.0/nvme'"). A state that cannot be
  saved in the game, such as a Battle Alert, has to be replayed from the
  nearest save.
- Do not use `-w`. Windowed, the game renders differently (32-bit colour,
  anti-aliased text, other 3D shading; full screen is an 8-bit palette), so
  its frames are not the original's. `capture.ps1` refuses a windowed cell
  frame; `FullClient=1` probes are still allowed.
- Full screen, the game draws its own cursor into the frame. `a0` parks it
  at the screen's bottom-right corner (`A0_PARK`, default `799 599`), where
  it covers the frame's bottom-right 9×9 pixels.
- The guest agent runs as SYSTEM in session 0, which has no desktop, so
  nothing it starts directly can show a window or capture one. `a0-setup`
  registers three tasks for the logged-on `capture` user with an
  interactive, elevated logon (`vm/a0-tasks.ps1`): `A0Watcher` runs
  `capture.ps1 -Watch C:\a0` under `conhost --headless`, so no console covers
  the game, `A0Game` runs `REBEXE.EXE`, and `A0Run` runs `C:\a0\run.ps1` on
  the desktop for checks that need the session (`a0-run FILE`). `up-a0`
  starts the first two with `schtasks /Run`, gives the game the focus, which
  a task's window does not get, with a click on the cockpit ceiling, and
  skips the intros. `a0-watch` restarts the watcher. Nothing is typed into
  the guest, because typing goes to whichever window has the focus.
- `a0` pushes a request through the guest agent and waits for the watcher's
  `.done` or `.err` in calls of about 8 s (one call held for 30 s made UTM
  report "Timed out waiting for RPC"), then pulls the PNG, JSON and screen
  frame, checks the request id, and checks the screen frame's hash. The
  watcher runs each request in a fresh PowerShell process, so an edited
  `capture.ps1` applies to the next capture.
- `capture.ps1` refuses a frame unless the game has the focus, since GDI
  copies whatever is on screen, and keeps one only after two consecutive
  identical frames (`FullClient=1` probes keep the first).
- One capture per cell. A cell about a transition (entry from the Battle
  Alert, a click's effect) pairs its frame with the frame before it:
  capture that first with `a0 NAME .artifacts/original-checks/a0
  StateSetupKind=original-save StateSetupIdentifier=SETUP`, then
  `A0_BEFORE=NAME a0-cell ...`; the ingester stores it as `before.png` and
  checks it shares the setup and came earlier, so a plain `a0 NAME`, which
  records no setup, is refused.
- A cell about motion (a running or observed battle) never holds two equal
  frames, so the stable capture refuses it ("Frame never stabilised"). Take
  a burst instead: `A0_BURST="12 250" a0-cell ...` (or `a0 NAME Burst=12
  IntervalMs=250`) copies the screen 12 times 250 ms apart on a stopwatch,
  keeps the copies in memory so encoding does not skew the cadence, and
  checks and rebuilds each one exactly. The guest zips the frames with
  their sources; `a0` unpacks `NAME.burst/` and checks every hash. The cell's
  frame is the burst's last; the ingester stores the frames as
  `frame-NN.png` and their offsets as `burst.json`. Burst takes 2–60
  frames at 100 ms or more, full screen only. Measured 2026-10-12: 12 frames
  over 2752 ms, every offset within 15 ms of its slot.
- An event the original holds only briefly lands at a frame you learn from
  the burst: a kill is held about 3 s before the results screen, and
  `a0-cell` takes about 27 s to start. Name that frame at registration:
  `a0-register CELL N` ingests the pulled capture with burst frame N as the
  cell's `original.png` (the guest hashed every frame) and records
  `cell_frame` in `burst.json`. `a0-cell` itself calls `a0-register` with no
  frame. To move a registered cell, take it out of the manifest and its
  folder to `.artifacts/original-checks/withdrawn/`, then register again.
- `a0-cell` refuses a cell already in the manifest before capturing, since
  the capture would overwrite that cell's working files. When the guest
  refuses a frame, read `C:\a0\out\NAME.err` with `gexec` rather than
  rerunning `a0-cell` to see the message; a rerun that succeeds registers
  whatever is on screen.

## Registration

On the development Mac, copy the PNG and JSON out of `payload.img`, or have
`capture.ps1` write to `Z:\captures` (the shared folder; not yet tried for
captures). Tactical
captures are registered with `tools/interface-parity/ingest-tactical-a0.mjs` as
documented by the [interface harness](../interface-parity/README.md). The host
ingester independently decodes the PNG, verifies its hash and dimensions,
checks the exact ledger requirement and executable provenance, and writes only
to the ignored `.artifacts/interface-parity/a0/` store. A failed capture is not
registered or credited toward coverage.

Strategic captures continue through their surface-specific host checks and
`tools/interface-parity/baselines/accepted-original.json`. No original-game
capture, executable, audio, or extracted asset may be committed. Saves are the
one exception, by the owner's decision: the games we played and saved on this
VM are kept in [`saves/`](saves/INDEX.md), with the one David Adams gave us
(credited there). Other saves from elsewhere are not.

## Saves

[`saves/INDEX.md`](saves/INDEX.md) lists each save, its side, day and state,
how to load one, how to copy files off the transfer disk, and the header
format. To make a new one, play to the state in the VM, press F1, click an
empty slot's name field, type a lowercase name and press Enter. For fast
retries, save in the game a day or two before the event and load that save.

Driving the game over VNC:

- `vm-game.sh` wraps the steps below from this Mac. `up` starts the VM if
  needed, waits for the desktop, launches `REBEXE.EXE`, accepts UAC, skips
  the intros and selects the tablet; `recover` first kills a wedged QEMU and
  restarts UTM. `up`, `up-a0` and `recover` mute the Mini's output first,
  since a reboot clears the mute (`a0-audio` records from QEMU, not the
  speakers). After a Mini reboot, load the control agent once:
  `launchctl bootstrap gui/$(id -u) ~/VMs/rebellion/com.openrebellion.vmctl.plist`. `point`/`click`/`rclick`/`dclick X Y [FILE]` hit a point in
  the captured 800x600 frame and optionally capture after; `desk X Y`
  clicks on the Windows desktop (the taskbar, a console), where the cursor
  is Windows' own; `save-out`, `save-in` and `gexec` use the guest agent;
  `cap`, `key`, `mon` and `status` cover the rest. `key` takes vncdotool's
  lowercase names (`esc`, `enter`, `f1`, `left`) or a single character;
  `Escape` and other names fail with "ord() expected a character". Saved Games (the main
  menu's disc, 535,315): each slot's right-hand button loads it.
- In-game verbs wrap the clicks you would otherwise script: `load SLOT`,
  `save SLOT NAME` (over a filled row; it clears the old name first),
  `resolve` (auto-resolve a Battle Alert and close the result),
  `take-command` and `withdraw`. Saved Games shows six rows, so only slots
  1-6 can be picked in the game; slots 7-10 take `save-in` and `save-out`
  only. The list shows the filled slots and one empty row and does not
  scroll, so a save in a slot after a gap (slot 8 with 6 and 7 empty) is not
  listed. The Battle Alert's buttons are Retreat, auto-resolve and Take
  Command, left to right.
- Start a tactical run from a fresh `up`. Take Command crashed the game
  (`rebexe.exe` access violation) after about ten saves had been loaded in
  one run, and worked from a fresh launch (2026-10-11). Loading a save from
  the Game Options screen opened inside a battle closed the game outright;
  load from the galaxy map or the main menu (2026-10-12). Every load resets
  the five Tactical Display Options to On; they are in neither the save (two
  saves either side of the Show Planet toggle are byte-identical) nor the
  registry's `DisplayOpts` (set to 0, the toggles still read On), and the
  battle greys them out. To change one for a battle, toggle it on the galaxy
  map after the load and play on into the battle; a save that opens its
  Battle Alert during the load (the day-30 Yaga saves) leaves no window.
- Ctrl and Shift chords with a click: Ctrl must stay held in the VNC
  connection that clicks, so `point X Y`, then `vnc keydown ctrl click 1
  keyup ctrl` (the same QMP-walk-then-VNC-press pattern as `dclick`).
  Separate `vnc keydown` and `click` calls drop the modifier. A
  Ctrl-right-click target (manual p. 145) also needs Ctrl held across a
  frame: `vnc keydown ctrl pause 0.5 mousedown 3 pause 0.3 mouseup 3 pause
  0.5 keyup ctrl`. Without the pauses the click replaced the target three
  times out of three (2026-10-12, TAC-04-C003).
- `vm-game.sh run FILE [OUTDIR]` replays a command file: one `vm-game.sh`
  command per line, `#` comments, `$OUT` for the output folder (FILE's own
  by default). It stops at the first failure. Every call, from a file or
  not, appends a line to `.artifacts/vm/log.jsonl` (`VM_LOG` overrides):
  time, command, arguments, exit code, seconds and the SHA-256 of each PNG
  it wrote, so a run can be audited and replayed.
- Wait on the game rather than polling it: `vm-game.sh wait REGION [N]
  [TIMEOUT] FILE` (`vm-wait.py`) holds one VNC connection, polls a region
  twice a second and returns after N changes, saving the frame and the
  region's crop (`FILE-region`); it exits 1 on the timeout (default 120 s).
  `day` is the galaxy's day readout and `center` the middle of the screen,
  where the Battle Alert and message dialogs open; `X,Y,W,H` names any other
  box. An open alert freezes the day readout. `alert` waits for a state, not
  a change: it returns as soon as a Battle Alert is open, including one
  already open when the wait starts, which `center` misses. A day digit
  flip can score as little as 1.86 against the usual 2.0 threshold, so the
  day box counts any change over 0.5 (the box is otherwise exactly 0).
  Each VNC call is bounded at 20 s: vncdotool's default is an hour, and one
  refresh stalled 43 minutes past a 45 s wait while the battle it was
  timing ran to its end (2026-10-12).
- Record the guest's sound with `vm-game.sh a0-audio start CELL` and
  `a0-audio stop CELL`: QEMU's `wavcapture` on UTM's audiodev
  (`spice,id=audio0`), pulled beside the cell's capture as `CELL.wav` with
  both SHA-256 values compared and a `CELL.wav.json` sidecar (hash, start
  and stop times, source). `a0-register CELL` then ingests it as the cell's
  `original.wav` (`audio_capture`) with `audio.json`; the ingester refuses a
  file that is not 16-bit PCM, under 10 s, silent, longer than its start to
  stop, or not recording when the frame was taken. QEMU writes samples only
  while the guest plays, so a WAV can be much shorter than its wall-clock
  span (117 s over 204 s for TAC-01-C012) and `frame_wall_offset_s` is not a
  position in the file. `a0-cell` registers before the sound is pulled, so take
  the frame with `a0 CELL ... CellId=CELL Requirement=...` instead. A
  running battle never gives a still: pause inside the recording for the
  frame, then resume. Never commit a WAV.
- Click through the absolute tablet: `mouse_set 1` on the QEMU monitor. The
  game's cursor follows the tablet's motion at about 1.25x and clamps at the
  screen edge, so an absolute jump lands off target by an amount that depends
  on where the cursor was. Walking the tablet to (0,0) in 4-pixel steps pins
  the cursor to the corner, and walking out to the target divided by 1.25
  lands on it. On a 5x5 grid the arrow's tip landed within 1 px walking over
  QMP and within 2 px over VNC (2026-10-11). `vm-game.sh` walks over QMP, about
  three times faster than over VNC (3-4 s against 9-12 s for a long walk);
  `VM_INPUT=vnc` walks over VNC. Double-clicks press over VNC after the
  walk: two clicks sent over QMP never read as a double-click (0 of 30), two
  over VNC did 20 of 20. UTM's AppleScript `input mouse click` jumps the
  tablet and misses by tens of pixels, so it is not used. Clicks sent with the relative mouse
  (`mouse_set 2`, `mouse_button`) leave the game acting as if the button is
  still held: list selections follow the pointer and prompts confirm
  themselves as the pointer crosses their buttons (2026-10-09). The relative
  `mouse_move` steps (8 overshoot about 6.5%, 4 fall about 12% short) are
  only for pointing without clicking.
- Hold a modifier through the QEMU monitor (`sendkey ctrl 4000` holds Control
  for 4 s) while `vncdo` clicks. A key `vncdo` holds is released when its
  connection closes, before the next call's click.
- If the guest agent answers "Timed out waiting for RPC", restarting the
  guest does not help; `vm-game.sh recover` (it restarts UTM) does.
- If clicks and keys do nothing, the game may have lost focus to a guest
  window (after Win+R, PowerShell): `vm-game.sh desk 510 576` clicks the
  game's taskbar icon. The guest agent works without the desktop and does
  not take focus.
- The VNC link drops Shift on letters; type lowercase. `vnc-type.py` sends
  symbols as shift chords.
- A double-click on a planet opens its manufacturing window; a double-click on
  the star under it opens its units window. Fleets move by dragging the fleet
  from a system's orbit view onto the destination; dragging the sector-map
  icon does nothing.
- Escape steps through the opening briefing; the game opens the Message Index
  when it ends. Renaming `ALBRIEF.DLL` and `EMBRIEF.DLL` is reported to skip
  the briefings (StarWarsRebellionEditor.NET README, "Fast game start"); it
  is untested here, so compare a new game's starting state with and without
  the rename before relying on it. On the Game Options screen (F1) the left button returns to the
  main menu and the middle one to the game.

## Wine on the Mini

The original also runs under Wine on the Mini, beside the VM, and strategic
checks start there: galaxy play, saves, messages, Battle Alerts and
play-forward to a day, up to 16 times faster than the VM. The VM stays the
reference. Wine frames are not A0 captures, and the tactical view renders
black, so tactical, A0 and sound work (`take-command`, `withdraw`, `a0*`)
stay on the VM, and the VM settles any result where the two disagree.
Tactical draws through the Direct3D Ramp device into 8-bit 640x480 surfaces
(`FUN_005d6e10`, `FUN_005d6c10`).
cnc-ddraw has no Direct3D. Wine's builtin ddraw fails the 640x480x8 mode
change (`ChangeDisplaySettings` returns -2, and the window stays 1920x1080
with the game drawn at its origin), creates the tactical surfaces at the
desktop's 32 bits, enumerates no Ramp device, and hangs at the game's first
`Lock` of the render target. dgVoodoo 2.8.2's `ddraw.dll` gives the game a
640x480 window and plays the strategic screens, but needs D3D11: Wine's
wined3d offers no feature level it accepts (OpenGL, or Vulkan on MoltenVK),
and the game then reads a null pointer at `0x005d89ed` on entering
tactical; with DXMT installed in a copy of the engine, feature level 10_1
works but the game faults at an unmapped address during its intros.

`wine-setup.sh` builds everything under `~/or-wine`. It pins the Sikarugir
engine `WS12WineSikarugir10.0_6` and its `Template-1.0.11`, whose
`Frameworks` hold the engine's dylibs, cnc-ddraw 7.1.0.0 with the GDI
renderer and `devmode=true`, the native `d3drm.dll`, a Windows 98 version
for the executable and the six install registry values, all in the 64-bit
WoW64 prefix `prefix-sik`. Upstream macOS Wine sets up 32-bit code through
the LDT, which Rosetta refuses, and GPTK 1.1 aborts in `alloc_pages_vprot`.
macOS strips `DYLD_*` from every SIP-protected program (`sh`, `bash`,
`nohup`, `env`), so `bin/wine` and `bin/wineserver` are wrappers that set
`DYLD_FALLBACK_LIBRARY_PATH` themselves; `env.sh` points `WINE_BIN` and
`WINE_SERVER` at them. The prefix's profile folders (Desktop, Documents,
Downloads …) are real directories, not Wine's default links to the home
folders, so nothing in Wine reaches `~/Desktop` and macOS has no reason to
ask for access. Sound plays through CoreAudio, so the Mini must be muted
first (`agent_docs/agent-tooling.md`, "Keep every run silent"); `wine-game.sh
up` does not mute.

Wine needs the GUI session, which SSH lacks. Setup installs the launch agent
`com.openrebellion.wine` (`~/or-wine/com.openrebellion.wine.plist`, with
`AbandonProcessGroup` so a game outlives the command that started it) and
its runner `bin/wine-run.sh`, which launchd starts whenever `run/q` changes.
Each queued `run/q/ID.cmd` runs in the GUI session, oldest first, with its
output in `run/done/ID.out` and its exit code in `run/done/ID.rc`;
`bin/wine-queue.sh CMD` queues one command, waits (`WINE_QUEUE_LIMIT`
seconds, default 3600, then exit 124), prints its output and exits with its
code. Removing a queued `ID.cmd` before the runner takes it cancels the
command (exit 130), and a waiter that times out or is signalled removes its
own, so a command given up on never reaches the game later; results nobody
collected are deleted after a day. With `WINE_QUEUE_EOF=cancel` the waiter
also treats the end of its stdin as its caller gone and cancels, which
covers a dropped SSH connection: a non-tty session sends the remote side
no signal, only the end of its input. Run over SSH, setup queues itself.
Setup reloads the agent with
`launchctl bootout` and `bootstrap`, never from inside the agent, since a
bootout there would end the setup run. After a reboot, load the agent with
`launchctl bootstrap gui/$(id -u) ~/or-wine/com.openrebellion.wine.plist`.
Setup leaves a running game alone and says to restart it; otherwise it
stops the wineserver so the next launch reads the new settings.

On this Mac, `wine-game.sh` has no `~/or-wine/env.sh` and runs each verb
on `WINE_HOST` (default `mac-mini-ts`) through that queue. Input files go to
a transfer folder there first, and output files come back with their
SHA-256. Its `ssh` reads a pipe that only the client holds open and runs
under `WINE_QUEUE_EOF=cancel`, so however the client ends (HUP, INT, QUIT,
ALRM or TERM, trapped to remove the queued command and the transfer folder,
or an untrappable kill, or a dropped connection) the Mini's waiter cancels
a command still queued. A command already running finishes. It refuses (exit 125) while its copies of `wine-game.sh`,
`wine-game-lib.py`, `wine-drive.c` or `wine-setup.sh` differ from the
Mini's, so a verb never runs against helpers it does not match;
`wine-game.sh sync` copies the four over and runs setup.

`wine-game.sh` mirrors the `vm-game.sh` verbs in the same 800x600 reference
frame; `drive.exe` (`wine-drive.c`) scales them to the 640x480 client.
Every verb was checked on both sides, except `drag` and `dclick`, checked
on the Empire side only. A
double-click opens a window only from a sector window's quadrant icons
(`FUN_004593e0` case `0x203` hit-tests their list, not the planets'), as in
the original. The helpers treat the client's top-left 640x480 as the game,
which under cnc-ddraw is the whole client. `up` checks that the game survives Escape past the intros;
it does not check the main menu by image. `load` and `save` take the six
slots the Saved Games list shows (1-6); `save-in` and `save-out` copy the
slot files directly and take 1-10. `load` checks the clock afterwards and
fails only when the clock is unchanged in scheduler address, day and step
count; a load builds a new scheduler, but one that reuses the old one's
address only draws a warning. With a game in play, treat that warning as a
failed load: all six seen kept the old day, behind an open Battle Alert or
a stuck F1. `save` checks that the slot file's hash
changed. Mouse verbs refuse a point outside the 800x600 frame (X 0-799,
Y 0-599), and `drive.exe` exits non-zero when Wine refuses any of a
gesture's input. `speed pause` opens the game's
"Resume Game Play?" dialog, which takes every click until it closes, so
`speed resume` presses its check mark before the next speed change. The
Empire's speed menu takes `pause`, `vslow` and `fast`, the Alliance's also
`medium`. The Empire's Very Slow item sits at 568,56 in a 640×480 capture of
the open menu, 710,70 in the 800×600 coordinates `drive` takes.

Scripted play hits these hazards, all seen in October 2026 save loops:

- An open Battle Alert takes F1, so `load` and `save` fail behind it: run
  `resolve` first.
- F1 can stop opening Game Options while clicks, typing and Escape still
  work. `save` then reports that the slot file did not change, and `load`
  keeps the old day. It struck 6 of 9 saves sent right after `stop-at 0` in
  one run from the `undefended` save (`agent_docs/original-save-editing.md`).
  A click on an open window's close box, sent after the release, brings F1
  back. With no window open, a click on the galaxy map at 370,200 opens the
  Sesswenna sector window, whose close box is 431,59. A loaded save reopens
  the windows it was saved with.
- A new `stop-at DAY` or `run-to-day DAY` leaves a hold already in place: a
  game held on day 465 stayed there through a 600 s `run-to-day 485`. Send
  `stop-at 0` first. The clock runs again from the release, so aim the next
  DAY at least two days ahead, and chain hold, read, release and save with
  `&&` so that a refused verb ends the chain.
- `load` keeps the timescale and restores the speed the save was made at.
  Steps per day show the speed: about 9 a day after `speed fast empire`,
  about 600 a day in the copies of the `undefended` save, which is 35 s a
  day at K = 4, so an 18-day `run-to-day` outran its 600 s default LIMIT.
  After a `load`, send `speed fast SIDE` or size LIMIT from two `clock`
  reads.
- `speed pause` reports success without checking. Behind an open Battle
  Alert it does nothing, and after the alert's `resolve` and a `save` the
  game ran on at Fast (day 801 to 876 in about 100 s). `resolve` first and
  confirm a pause with two `clock` reads.
- A save made while `run-to-day` holds the game writes a slot file the
  parser cannot read: send `stop-at 0` before `save`. A slot written after
  the release can still fail: a day-801 save gave "no object header at
  0xde5b5" (cause not traced). Run `save_tree.py check` on a save before
  building on it.
- At normal speed each helper call costs about a game day, and `run-to-day`
  at timescale 8 overshot its day by 20-45 days in one loop.
  Check each `save`'s exit and hash, stop at the first failure, and drop to
  timescale 1-2 near a target day. `load`'s clock warning goes to stderr.

- Input is hardware input. The game polls `GetAsyncKeyState` for Escape and
  button 1 (`FUN_005f7240`, `FUN_006036a0`) and its keyboard and mouse hooks
  see only hardware messages, so the helpers make the game Wine's foreground
  window first; cnc-ddraw reports every key up while it is not. Keys go
  through `SendInput`, held 200 ms (60 ms in runs of more than three); a
  button is held 60 ms. The macOS frontmost app does not change.
- Wine warps the Mac pointer after injected mouse input only, so mouse verbs
  go to `drive64.exe`, the 64-bit build of the same source. It sends moves
  and buttons the way Wine's macOS driver sends its own, through win32u's
  `NtUserCallHwndParam` (`SendHardwareInput`, 28 in Wine 10.0) with flags 0,
  which a 32-bit caller cannot lay out. Wine's `GetCursorPos` reports the
  Mac pointer 100 ms after the last mouse input, so `drive.exe cursor on`
  points the game's `user32!GetCursorPos` at the last point `drive64.exe`
  sent. `up` runs both after `drive64.exe selftest`, which checks that a
  move reaches Wine's cursor and leaves the Mac pointer where it was;
  `status` prints `mouse=drive64.exe`. `up` waits up to 180 s for the game
  window, since Wine's services took about 80 s to start on a loaded Mini,
  and a shorter wait skipped the selftest. If the selftest fails, mouse verbs
  stay on `SendInput`, whose moves warp the pointer, and the Wine game's
  mouse must not be driven while the VM is in use. Loading, the speed
  menu, `drag` and `dclick` left the Mini's pointer where it was.
- Wine's macOS driver keeps a window's pixels inside the process that owns
  it, so `cap` loads `drive-cap.dll` into `REBEXE.EXE` and copies the window
  there. macOS `screencapture` has no Screen Recording grant on the Mini and
  shows only the wallpaper. The game keeps the copy of `drive-cap.dll` it
  first loaded, so after rebuilding the helpers restart the game; `drive.exe`
  compares its build ID (the first 16 hex digits of the source's SHA-256)
  with the loaded copy's and refuses to call into another build. The
  patches `drive-cap.dll` writes into code (the hold below, `cursor on`)
  land with every other thread of the game suspended outside the bytes
  being replaced.
- `clock` reads the day and step count from memory: the day scheduler
  `*DAT_006b2fcc` (`FUN_0051ce00`), day `+0x10`, steps `+0xc`. A day runs
  the larger side speed word plus one steps. `peek PTR OFF [N]` reads the
  dword at `*PTR + OFF` (PTR and OFF decimal or `0x` hex), or prints a
  histogram of N reads 1 ms apart.
- The game busy-waits: the main loop (`FUN_00413560`) steps the session on
  every pass. The strategic step waits in phase 0 until the speed timer at
  GameOptions `+0x110` passes `_DAT_006588f0` (`FUN_0041e390`,
  `FUN_00415e20`), and that timer reads `QueryPerformanceCounter`
  (`FUN_005fbbe0`); animations and the droids tick on `timeGetTime`
  (`FUN_00441d70`). Fast therefore runs 0.8 days a second under Wine as on
  the VM. `timescale K` points REBEXE's imports of both functions at
  versions that run K times faster (1-64), leaving cnc-ddraw's frame limiter
  on real time: at Fast, K = 4 gave 3.3 days a second and K = 16 gave 13.
  Pace changes; the work of each step does not. At K = 16, F1, the speed
  menu's right-click and its clicks still land.
- `stop-at DAY` holds the game at the first step of DAY the way its own
  Pause does. In phase 2 the step waits while the current step
  (`FUN_0041e1b0`, read by `FUN_0041e300`) equals the step object's stop at
  `*DAT_006b1218 + 0x10`, and Pause sets that stop and both sides' holds
  (`side + 0x48`, `FUN_004888d0`) to the current step; `FUN_004361d0`
  clears the stop once both holds are 0. `drive-cap.dll` replaces the start
  of `FUN_0041e300` with a jump to its own check (`load` installs it),
  which sets all three at the first step of DAY. DAY must be after the
  current day: a day that has begun is past its first step, so `stop-at`
  and `run-to-day` refuse it. `stop-at 0` releases the hold. While held the
  frame is frozen and the game acts on no input: two captures seconds apart
  are byte-identical, the day readout keeps the previous day, F1 opens no
  Game Options, and a click on a sector window's close box had no effect
  then or after the release. The main loop (`FUN_00413560`) still
  dispatches window messages, but each pass of the step parked in phase 2
  runs `FUN_004361d0`, which calls both sides' slot `+0x28`
  (`FUN_00486520`, vtable `0x0065ad70`), and that runs the side's pending
  orders (`FUN_00488b90`) only while the side's hold is 0 and
  `FUN_0041dc10` finds no stop. Unlike the game's Pause, the hold opens no
  "Resume Game Play?" dialog (only `FUN_00415e60` opens it). `clock` and
  `messages` work while held, so read the state then and save after the
  release. `run-to-day DAY [LIMIT]` sets the hold and waits for it; on
  timeout it disarms the stop (`drive.exe stopat disarm`, which leaves a
  hold alone), so the game does not hold at DAY later, unasked.
- `messages [N]` prints the newest N Message Index entries from memory, run
  on the game's own thread inside the step check: the list `*(*DAT_006b14e4
  + 0xc) + 0x7c` is a threaded tree whose rightmost node is the newest, and
  each entry gives its kind (vtable slot `+0x10`), category mask `+0x34`,
  read bit `+0x24`, filing step `+0x38`, title `+0x48` and body `+0x54`.
  Entries hold no day, so the check logs the day of every step it sees and
  prints `day=?` for an entry filed before that log began (before `load`).
  The index drops entries: kind-3 entries from days 439-464, listed on day
  465, were gone by day 796 in the same game, while the kind-8 advice and
  two early Manufacturing entries stayed. Read `messages` within days of
  the event.
- A Battle Alert is detected by its button bar against reference frames that
  `alert-ref NAME` saves on the Mini as `ref/alert-NAME.png` (game output,
  not committed; the Mini holds `alert-empire`, so save an Alliance one
  before matching Alliance alerts), and its title by the share of changed
  pixels in the title box (two titles differ in 8%). `resolve` presses the
  auto-resolve button only when an alert is open (exit 1 when none is);
  with no reference saved it warns and presses it unchecked. `run-to-alert
  TITLE.png LIMIT FILE` resolves every other alert.
- Play forward is deterministic and the time scale does not change it. From
  `empire-fleet-to-yaga-minor.sav`, two fresh launches at K = 1 and two at
  K = 16, each held at the first step of day 45, printed identical message
  lists and held at the same step (2776). Fleet 1's arrival at Yaga Minor
  and its blockade were filed on day 42 in all four; the VM's captures of
  the same transit show the arrival by day 43.
- `resolve` auto-resolves, which rolls the battle afresh, so a run through
  it follows a VM run only until a battle's outcome matters. From
  `empire-day660-palpatine-jailed.sav` at K = 16 the alerts open on the VM's
  days (Carida 669, Coruscant 676), but with those battles auto-resolved the
  Empire still stood on day 924, where the VM run's Alliance had won by day
  692 (the Standard win also needs Coruscant, `victory-flag-word.md`).
