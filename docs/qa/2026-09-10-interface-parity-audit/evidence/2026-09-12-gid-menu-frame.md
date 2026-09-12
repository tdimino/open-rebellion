# GID menu-frame bitmap checkpoint

Status: source-mapped visual and interaction checkpoint, not `CMD-02` acceptance.

The original GID constructor `FUN_004511e0` supplies STRATEGY resources
`10100..10107` to its frame builder. Inspection of those 8-bit BMPs identifies
four `2x2` corner tiles, two `2x1` horizontal edge tiles, and two `1x2`
vertical edge tiles. The retained
[Imperial menu frame](../reference-captures/failed-multiplayer-recording-single-player-frames/0450-imperial-gid-menu.png)
shows the characteristic repeating silver edge. P50 paints the same eight
resources around the root and each submenu. Repeated edges are batched into
four texture meshes per menu to avoid hundreds of separate paint submissions.
Both menus now appear immediately instead of using egui's unrelated fade.

The test-only browser harness decodes the original indexed BMPs and compares
all root-border pixels in unobscured native `640x480` captures. The full
`CMD-02` catalog passed `152/152` fresh, audio-muted Chrome for Testing cases:
38 scenarios, two factions, and two viewport classes. Seventy-four native-size
captures matched all `634` root-border pixels, or `46,916` exact source-pixel
comparisons. Four system-window cases intentionally report that the window
occludes the frame, and 74 responsive captures verify behavior and geometry
without claiming native-pixel identity. Every case made exactly four startup
requests, reported no browser or missing-asset errors, and closed its isolated
browser process. The passing run is
`.artifacts/interface-parity/2026-09-12T15-24-02-716Z-21590` (ignored raw
evidence). The production WASM SHA-256 is
`3971962be99beafaa834ff0ce5cca199eb26c2413f9530a1dbd44a5dc33def10`;
the unchanged runtime pack SHA-256 is
`1ce8e2370d037423ad20683ff747245548b3eca453305596a45ef3083899a862`.

The local root rectangle (`x=425`, `y=230`, `158x159`) is an implementation
geometry used for source-pixel regression. The exact original frame rectangle,
interior fill, text rasterization, row spacing, submenu placement, focus and
disabled states, and original-executable interaction matrix are not proven by
this checkpoint. The exported `FUN_00607740` body is empty, so its layout
rules still require recovery from the saved Ghidra project or original runtime.
The retained video frame is compressed discovery evidence, not an A0 lossless
baseline. All strict `CMD-02` cells therefore remain pending.

The full `make all` gate passed workspace tests, formatting, strict Clippy,
and native build with the clean-repository Clippy configuration. A read-only
Astra medium review independently reproduced the source-pixel comparisons,
confirmed the muted browser and artifact records, and found no blocking defect.
It noted that the harness currently excludes the entire partially occluded
border in system-window cases. A later gate should verify the visible remainder
against known window bounds. No original-runtime or frame-time benchmark is
claimed by this checkpoint.
