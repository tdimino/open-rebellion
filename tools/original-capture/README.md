# Original capture rig (A0 evidence)

Tooling for producing lossless 640x480 captures of the unmodified original
`REBEXE.EXE` so the interface audit can register A0 references and the harness
can create goldens. Path 1 of
[the P59 plan](../../docs/plans/2026-09-14-feat-p59-parallel-gid-message-index-lane.md).

Captures are copyrighted game output and are never committed. Only their
SHA-256, metadata, and the accepted-original registration enter the repo.

## Host

The guest runs in UTM 4.7.5 on the Mac Mini (`mac-mini-ts`, Apple M4, macOS
15.6) under `~/VMs/rebellion/`:

| File | Purpose |
|---|---|
| `Win11_25H2_English_Arm64.iso` | Microsoft multi-edition Arm64 installer (sha256 `638aa2c8…adf0`) |
| `unattend.iso` | ISO 9660 image of `vm/autounattend.xml` and `vm/go.cmd` (built with `hdiutil makehybrid -iso -joliet`) |
| `payload.img` | 2 GiB raw FAT32 volume **with an MBR partition table** (`hdiutil create -layout MBRSPUD -fs "MS-DOS FAT32"`) holding the owned GOG game folder, `vm/install.cmd`, and `vm/capture.ps1`; captures are written back to it |
| `utm-guest-tools-0.1.273.iso` | kept on disk but **not attached** (see below) |

Two facts about Windows Setup shape this layout, both established on
2026-09-14 by watching Setup fall through to its interactive pages:

- Setup's implicit `autounattend.xml` search covers removable media only.
  QEMU's `usb-storage` presents image files as fixed disks, so an answer
  file on a FAT image is never found. A CD is removable read-only media and
  is searched, hence `unattend.iso`.
- The UTM guest tools ISO ships its own `autounattend.xml`; when attached it
  is found first. Attach it only after the install, if SPICE tools are ever
  needed (they are not for the VNC capture path).
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
  100.125.235.4:1
```

The last argument binds a QEMU VNC display to the host's Tailscale address so
the guest can be driven from another machine with `vncdo` (never bind to
`0.0.0.0`). Drives: two USB CD-ROMs, one 64 GiB NVMe system disk (Windows
Arm64 ships an NVMe driver, so setup needs no virtio storage driver), and the
raw FAT USB volume. UTM copies `payload.img` into the VM bundle
(`~/Library/Containers/com.utmapp.UTM/Data/Documents/Rebellion A0 Capture.utm/Data/`);
captures land in that copy, not in `~/VMs/rebellion/`.

The VM must carry a sound card. `REBEXE.exe` calls `DirectSoundCreate` during
startup (`FUN_00610d00`), and with no audio device the returned interface is
unusable; the game then faults dereferencing it (`FUN_00611140`, exception
`0xc000041d`) before a window ever appears. `create-vm.applescript` requests no
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
contains only `CpuAffinity=all` and `LogLevel=none`, so it does not change
rendering.

`REBEXE.exe` is 32-bit, so on 64-bit Windows it reads its data locations from
the `WOW6432Node` view of `HKLM\Software`. `install.cmd` writes the six values
with `/reg:32`; a plain `reg add` lands in the 64-bit view and the game reports
"CD Not Found".

## Capture

```powershell
powershell -ExecutionPolicy Bypass -File D:\capture.ps1 -Name alliance-shell -Out D:\captures -Launch C:\Rebellion\REBEXE.exe -LaunchArgs:-w
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

## Registration

On the development Mac, copy the PNG and JSON out of `payload.img`. Tactical
captures are registered with `tools/interface-parity/ingest-tactical-a0.mjs` as
documented by the [interface harness](../interface-parity/README.md). The host
ingester independently decodes the PNG, verifies its hash and dimensions,
checks the exact ledger requirement and executable provenance, and writes only
to the ignored `.artifacts/interface-parity/a0/` store. A failed capture is not
registered or credited toward coverage.

Strategic captures continue through their surface-specific host checks and
`tools/interface-parity/baselines/accepted-original.json`. No original-game
capture, save, executable, audio, or extracted asset may be committed.
