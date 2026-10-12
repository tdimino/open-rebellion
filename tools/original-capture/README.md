# Original capture rig (A0 evidence)

Tooling for producing lossless 640x480 captures of the unmodified original
`REBEXE.EXE` so the interface audit can register A0 references and the harness
can create goldens. Path 1 of
[the P59 plan](../../docs/plans/2026-09-14-feat-p59-parallel-gid-message-index-lane.md).

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
| `backup-20261011/` | APFS clone of the whole VM bundle taken cold before the 2026-10-11 changes, with `SHA256SUMS` |

Two facts about Windows Setup shape this layout, both established on
2026-09-14 by watching Setup fall through to its interactive pages:

- Setup's implicit `autounattend.xml` search covers removable media only.
  QEMU's `usb-storage` presents image files as fixed disks, so an answer
  file on a FAT image is never found. A CD is removable read-only media and
  is searched, hence `unattend.iso`.
- The UTM guest tools ISO ships its own `autounattend.xml`; when attached it
  is found first. Attach it only after the install. Its full installer also
  brings vdagent, the network driver and the GPU drivers; we install only
  the pieces in `webdav-tools.iso` (2026-10-11, below).
- A FAT volume without a partition table (an `hdiutil` "superfloppy") is
  invisible to Windows on a fixed disk. `payload.img` therefore carries an
  MBR.

UTM needs a logged-in desktop session on the host, and its AppleScript
bridge refuses callers from an SSH session (TCC error -1743, no prompt). The
working pattern is a user launch agent bootstrapped into the GUI domain
(`sudo launchctl bootstrap gui/$UID …`), whose `osascript` call raises the
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

Changes on 2026-10-11, with the bundle cold-copied to `backup-20261011/`
first (restore by copying it back with the VM stopped):

- No network device (`Network` is `[]` in `config.plist`) and clipboard
  sharing off. The guest had no network driver; removing the device keeps
  it that way now that guest software is installed.
- A loopback-only QMP socket for `vm-qmp.py`, which walks the pointer
  (`QEMU.AdditionalArguments`: `-qmp tcp:127.0.0.1:4446,server=on,wait=off`).
  Never bind it to the Tailscale address or `0.0.0.0`: QMP is an
  unauthenticated control channel.
- The SPICE WebDAV shared folder: UTM's `update registry` AppleScript sets
  `~/VMs/rebellion/exchange` (`share PATH` in `vm-ctl.applescript`), and
  `spice-webdavd` in the guest maps it to `Z:`. UTM stops serving the
  folder after the VM restarts although its registry still names it;
  setting it again while the VM runs brings it back, and `vm-game.sh up`
  does so. Its install needed the
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
- For fast retries, save in the game a day or two before the event and
  load that save (`saves/INDEX.md`).

After these changes the menu renders the same (outside the animated
monitors the frames differ only under the cursor), and the day-31 alert from
the day-5 transit save arrives 360-362 s after the load against 336.5 s on
2026-10-11 before them. Stopping `spice-webdavd` did not change that
(362.0 s), and the Mini was busy with Spotlight and media analysis at the
time; the cause is not settled.

The VM must carry a sound card. `REBEXE.exe` calls `DirectSoundCreate` during
startup (`FUN_00610d00`), and with no audio device the returned interface is
unusable; the game then faults dereferencing it (near `FUN_00611140`; address
unverified from our Ghidra project, exception `0xc000041d`) before a window ever appears. `create-vm.applescript` requests no
sound, so after the VM exists, set one while it is stopped:

```sh
plutil -replace Sound -json '[{"Hardware":"intel-hda"}]' \
  "$HOME/Library/Containers/com.utmapp.UTM/Data/Documents/Rebellion A0 Capture.utm/config.plist"
```

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
  capture that first with `a0 NAME`, then `A0_BEFORE=NAME a0-cell ...`; the
  ingester stores it as `before.png` and checks it shares the setup and came
  earlier.
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
VM are kept in [`saves/`](saves/INDEX.md). Saves downloaded from elsewhere are
not.

## Saves

[`saves/INDEX.md`](saves/INDEX.md) lists each save, its side, day and state,
how to load one, how to copy files off the transfer disk, and the header
format. To make a new one, play to the state in the VM, press F1, click an
empty slot's name field, type a lowercase name and press Enter.

Driving the game over VNC:

- `vm-game.sh` wraps the steps below from this Mac. `up` starts the VM if
  needed, waits for the desktop, launches `REBEXE.EXE`, accepts UAC, skips
  the intros and selects the tablet; `recover` first kills a wedged QEMU and
  restarts UTM. `point`/`click`/`rclick`/`dclick X Y [FILE]` hit a point in
  the captured 800x600 frame and optionally capture after; `desk X Y`
  clicks on the Windows desktop (the taskbar, a console), where the cursor
  is Windows' own; `save-out`, `save-in` and `gexec` use the guest agent;
  `cap`, `key`, `mon` and `status` cover the rest. Saved Games (the main
  menu's disc, 535,315): each slot's right-hand button loads it.
- In-game verbs wrap the clicks you would otherwise script: `load SLOT`,
  `save SLOT NAME` (over a filled row; it clears the old name first),
  `resolve` (auto-resolve a Battle Alert and close the result),
  `take-command` and `withdraw`. Saved Games shows six rows, so only slots
  1-6 can be picked in the game; slots 7-10 take `save-in` and `save-out`
  only. The Battle Alert's buttons are Retreat, auto-resolve and Take
  Command, left to right.
- Start a tactical run from a fresh `up`. Take Command crashed the game
  (`rebexe.exe` access violation) after about ten saves had been loaded in
  one run, and worked from a fresh launch (2026-10-11).
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
- Click through the absolute tablet: `mouse_set 1` on the QEMU monitor. The
  game's cursor follows the tablet's motion at about 1.25x and clamps at the
  screen edge, so an absolute jump lands off target by an amount that depends
  on where the cursor was. Walking the tablet to (0,0) in 4-pixel steps pins
  the cursor to the corner, and walking out to the target divided by 1.25
  lands on it. On a 5x5 grid the arrow's tip landed within 1 px walking over
  QMP and within 2 px over VNC; the earlier 1.28 fell up to 20 px short at
  the right and bottom (2026-10-11). `vm-game.sh` walks over QMP, about
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
  guest does not help; `vm-game.sh recover` (it restarts UTM) does. It
  started after the shared folder was set again on a running VM, so `up`
  now sets it only after starting the VM (2026-10-11).
- If clicks and keys do nothing, the game may have lost focus to a guest
  window (after Win+R, PowerShell): `vm-game.sh desk 510 577` clicks the
  game's taskbar icon. The guest agent works without the desktop and does
  not take focus.
- The VNC link drops Shift on letters; type lowercase. `vnc-type.py` sends
  symbols as shift chords.
- A double-click on a planet opens its manufacturing window; a double-click on
  the star under it opens its units window. Fleets move by dragging the fleet
  from a system's orbit view onto the destination; dragging the sector-map
  icon does nothing.
- Escape steps through the opening briefing; the game opens the Message Index
  when it ends. On the Game Options screen (F1) the left button returns to the
  main menu and the middle one to the game.
