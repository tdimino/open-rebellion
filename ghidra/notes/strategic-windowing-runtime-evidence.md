---
title: "Strategic Windowing Runtime Evidence"
description: "Sanitized original-runtime ledger supporting the strategic windowing toolkit contract"
category: "ghidra"
status: complete
created: 2026-10-10
updated: 2026-10-12
---

# Strategic Windowing Runtime Evidence

This ledger records the original-runtime observations used to shape
[`strategic-windowing-toolkit.md`](strategic-windowing-toolkit.md). It closes
that note's API-shaping Wine research queue; it is not native-Windows or
Open Rebellion interface acceptance.

The owned `REBEXE.EXE` observed under Wine 9.0 has SHA-256
`b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`.
The executable, saves, original assets, Wine prefix, local helper binary, and
proprietary captures are not committed. A local manifest retains the full
action log, command transcript, screenshots, HWND/object dumps, and hashes;
its final SHA-256 is
`937ce1aff649a17e68f283df63ce758bf77629bac6bd69218507a61c156ae186`.

## Environment and reproducibility

- Wine: `wine-9.0 (Ubuntu 9.0~repack-4build3)`.
- Isolated X display: `:92`, 1024x768x24, with Fluxbox. The shared capture
  display `:91` was not used.
- Input and capture: `xdotool` plus one-frame `ffmpeg` X11 grabs.
- Inspection: local `enum_windows.exe`, which enumerated the Wine HWND tree
  and read the associated original C++ objects. It is evidence tooling, not
  port production code.
- Alliance save SHA-256:
  `65de2c6bf0557e755ac58674fcd8bfff40a34859c80b64166a0a3e8ebba87709`.
- Empire save SHA-256:
  `d9da6a3bff71a70130ccd7aecb9ab4a98becf7bfef5e3fc7918c0970ea7cbc79`.

The repeatable command forms were:

```sh
Xvfb :92 -screen 0 1024x768x24 -nolisten tcp
env DISPLAY=:92 fluxbox
env DISPLAY=:92 WINEPREFIX=LOCAL_PREFIX \
  ALSA_CONFIG_PATH=LOCAL_ASOUND_CONF WINEDEBUG=-all \
  wine ./REBEXE.EXE -w
ffmpeg -hide_banner -loglevel error -f x11grab \
  -video_size 1024x768 -i :92 -frames:v 1 -y OUTPUT.png
env DISPLAY=:92 WINEPREFIX=LOCAL_PREFIX WINEDEBUG=-all \
  wine ./enum_windows.exe
```

One first isolated X/Wine process group ended unexpectedly during a mission
probe. The pass was restarted from the unchanged save, and no state from the
failed display session was treated as game behavior. The completed isolated
Wine server, Fluxbox, and Xvfb processes were stopped after capture.

## Repeated action sequence

The following policy-changing cases were repeated from both faction saves.
The Alliance character was Leia Organa; the Empire character was Emperor
Palpatine. Fleet and destination names differed, but the steps and observed
policy were the same.

### Status

1. Open Personnel Finder with F5 and double-click a character row to open the
   character's Defenses detail.
2. Right-click the character and choose **Status**.
3. Click visible content outside Status, then press F2.
4. Press Escape.

Status remained above enabled-looking detail content; neither the click nor
F2 activated the underlay or opened Finder. Escape dismissed Status, and the
prior details were visible again on the later dispatch.

### Build Selection

1. Right-click the faction advisor and choose **Build Facilities**.
2. Select the destination system from an already open detail.
3. Click visible underlay content, then press F2.
4. Press Escape.

Build Selection remained active; the click and F2 did not activate the
underlay. Escape dismissed it and exposed the prior details. No normal user
route to a second Build Selection is available while this strategic boundary
owns input. The recovered opener's direct programmatic duplicate behavior is
still not claimed as native fact; the port contract does not expose such a
re-entrant opener.

### Create Mission

1. Right-click the character and choose **Mission**.
2. Select the neutral Averam system by its open system-detail title.
3. Click visible underlay content, then press F2.
4. Press Escape.

Create Mission remained active; the click and F2 did not activate the
underlay. Escape dismissed it and exposed the prior details.

### Move Confirmation

1. Open Fleet Finder with F3 and double-click a fleet.
2. Right-click the fleet and choose **Confirmed Move**.
3. Select another planet in the open Sector.
4. Click visible underlay content, then press F2.
5. Press Escape.

Move Confirmation remained active; the click and F2 did not activate the
underlay. Escape cancelled it and restored the fleet/detail composition.

### Cross-family modeless order and parent close

For Alliance, System Finder and Sector icons opened Averam Manufacturing,
Yavin Defenses, and Coruscant Defenses. `Ctrl+Tab` raised Coruscant Defenses
above the other retained details. Closing the Sseswenna Sector left those
strategic-root-owned details alive.

For Empire, Fleet Finder opened a Coruscant Fleet window while Averam
Manufacturing and Coruscant Defenses remained open. `Ctrl+Tab` raised
Coruscant Defenses. Closing the Sseswenna Sector again left the detail
windows alive. These observations corroborate the static final-owner trace:
Sector supplies launch context, not detail lifetime.

## Live object state

The helper recorded the same semantic type and policy word for each faction:

| Surface | Type | `+0xb8` policy | `+0xf4` state | Alliance dump SHA-256 | Empire dump SHA-256 |
|---|---:|---:|---:|---|---|
| Status | `0x1a` | `1` | `1` | `b4387a0a2c51b0badd668966aebd1c383408278bcc0a7fec1f066cc684623a5f` | `65c2bf8b5919b427b776bbf5481d99557f1c8c234abfdee6e267dc289a5cb61c` |
| Build Selection | `0x62` | `5` | `2` | `986bf0c8844af388f66c057c37d0e1733626967608c1555410fb8e57dce60f5b` | `1126db21289349287eace1b4df221fc2ff301d26523def4ec6fe82cadb14d4aa` |
| Create Mission | `0x0c` | `1` | `1` | `2c56d2ded2380b707ea90e76910dd8be84fdfab6497892e70ff719f38cac04a1` | `d2260682e641d7fa1676d6f2e8aa7d2eaa13a5ba7fcfd1980edeb923bceeebe8` |
| Move Confirmation | `0x10` | `1` | `1` | `8db93cd93f7f3ee50bee884303ecc2ab58f35b80b4f31055a90fd05bbd130e95` | `a13df20aceb765d6c5721489d569a1ea186ed5c9df513685b9e5a49d97541929` |

The observed objects were direct children of the strategic view. The
underlay controls and owner remained enabled, and no dialog held pointer
capture for its lifetime. Therefore the visible whole-strategic-view boundary
is not explained by disabling the owner or permanent capture.

## Selected local artifacts

Paths are relative to the uncommitted local capture manifest.

| File | SHA-256 | Observation |
|---|---|---|
| `shots/2026-10-12-alliance-cross-family-ctrl-tab.png` | `7245e16a6d68b0bb943fe04a9818e2be0f37187eb209c2a31921efa5455d3563` | Alliance modeless cross-family raise |
| `shots/2026-10-12-alliance-sector-close-details-survive.png` | `facec4c696002ad90d0dfe895a4447ec44909d1f51c0c8fd2174f772d61055b5` | Alliance Sector close leaves details |
| `shots/2026-10-12-alliance-status-blocks-underlay.png` | `c4ab0af9d2404b32a6b6f52aa7229c2fb5f6473490e5565858c2b6dbd1f50db5` | Status blocks click and F2 |
| `shots/2026-10-12-alliance-build-selection-blocks-underlay.png` | `e4225ba3bb3aefbad748ccf9f399476de5bfaa485b77879a5f43581b1a8499ea` | Build Selection blocks click and F2 |
| `shots/2026-10-12-mission-target-averal-title.png` | `d71e3f5ee029dd676b324ce4bf2f74e2e01be9eec2e0d7234d86f90342b41cb9` | Alliance Create Mission |
| `shots/2026-10-12-alliance-fleet-move-confirmation.png` | `8e69800c81f5bbb315a8c08ec7fe94fc468cbaf6f2db72bf620eacf0cfe47ab8` | Alliance Move Confirmation |
| `shots/2026-10-12-empire-status-palpatine.png` | `5055a79998b53ec353b5a29fcdcc3de5ba57c2494a46ae443a5d4dbe9e7d4e85` | Empire Status |
| `shots/2026-10-12-empire-build-selection.png` | `d887c7017e04b0935b58775fa3e1a5cba44ef9289d98bc18858b2bf7e5134a1d` | Empire Build Selection |
| `shots/2026-10-12-empire-create-mission.png` | `135b362b2da735c5624ad30ccea3aeb13e2a3d0effb32724a6996e8c821abb3c` | Empire Create Mission |
| `shots/2026-10-12-empire-move-confirmation.png` | `35d1f3f55a58a4e8487c4b29bc02fc13882941a0b507663dea5093dbf9681985` | Empire Move Confirmation |
| `shots/2026-10-12-empire-cross-family-ctrl-tab.png` | `f999f64072dbe77d26c7fbb6b007e00b24c04ac8f08f2ea5fb170dcc9a32e9db` | Empire modeless cross-family raise |
| `shots/2026-10-12-empire-sector-close-details-survive-2.png` | `5a863cb7c5c9ff8501585b6a1a414ad8d1b2bb9f30e45bb406fe376d6ab7becd` | Empire Sector close leaves details |

The earlier local manifest also retains hashed Alliance and Empire
Encyclopedia blocking, focused prefix search, Escape restoration, two-Sector,
and Message-close captures from 2026-10-10.

## Inconclusive runtime families

The Alliance load was retained at displayed day 60 and the Move Confirmation
capture at day 79. The Empire load was retained at displayed day 3 and the
final Sector/detail capture at day 23. No Battle Alert appeared in either
interval. This negative observation is stochastic and does not establish that
Battle Alert is unreachable.

Without a Battle Alert, Battle Results / Summary was unreachable from both
available saves. Battle Alert and Battle Results therefore retain their
static/manual classification and explicit provisional pointer policy. No
visible-runtime parity claim is made for either family.

## Evidence rule

This Wine evidence resolves the public toolkit's API-shaping choices:
modeless details share one strategic owner and ordering authority; the four
newly exercised bit-0 families use a whole-strategic-view input boundary; and
close/restoration consumes the dismissing dispatch. The port may encode a
conservative strategic boundary for Battle Alert and Battle Results, but
their audit cells remain open until each has original-runtime and port-side
acceptance evidence. Wine does not establish native-Windows parity.
