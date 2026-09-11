---
title: "P46B Strategic Command Controls Evidence"
description: "Recovered geometry, resource, input, routing, test, and browser evidence for the six primary strategic controls"
category: qa
created: 2026-09-11
updated: 2026-09-11
tags: [qa, interface, parity, strategy, controls, bitmap]
---

# P46B Strategic Command Controls Evidence

P46B restores the six primary strategic controls for both command centers. The
replacement text-button strip is gone. Each control now uses its original
STRATEGY bitmap, recovered rectangle, command ID, pointer contract, and
keyboard accelerator where the original defines one. Original destination
windows remain closed until they can replace the current reconstructed panels.

## Recovered control contract

`REBEXE.EXE` function `FUN_00427270` constructs the controls below. Resource
pairs are listed as normal/pressed.

| Command | ID | Alliance rectangle and resources | Imperial rectangle and resources |
|---|---:|---|---|
| System Finder | `0x12d` | `(106,408,27,16)`, `10002/10001` | `(143,434,37,24)`, `10016/10015` |
| Fleet Finder | `0x12e` | `(157,407,27,15)`, `10004/10003` | `(199,434,37,24)`, `10018/10017` |
| Personnel Finder | `0x12f` | `(258,405,27,16)`, `10006/10005` | `(412,433,34,22)`, `10020/10019` |
| Troop Finder | `0x130` | `(209,405,27,16)`, `10008/10007` | `(253,433,34,22)`, `10022/10021` |
| Game Options | `0x131` | `(394,405,27,16)`, `10010/10009` | `(465,434,35,24)`, `10024/10023` |
| Encyclopedia | `0x132` | `(446,406,27,16)`, `10012/10011` | `(519,434,37,25)`, `10026/10025` |

The official manual's Figure 3.8, retained as
[`manual-069.jpg`](../reference-captures/manual-pages/manual-069.jpg), confirms
the six destinations and their physical order.

## State and input contract

- `FUN_00602d30` paints the normal resource at rest and the pressed resource
  only while the pointer press is captured. It defines no separate hover or
  persistent selected art.
- `FUN_006028c0`, `FUN_006030c0`, `FUN_006030f0`, and `FUN_006035f0` prove
  capture on a valid press, cancellation after an invalid release, and one
  command dispatch after a valid release over the captured control.
- `FUN_005fca00` rejects all four outer edges and pixels matching the bitmap's
  transparent palette key. `FUN_005fd170` identifies that key from the first
  stored pixel, which is the decoded bottom-left pixel for these positive-height
  BMPs.
- `FUN_005fc140` paints each bitmap at its natural size. The control window
  rectangle clips any excess pixels.
- `FUN_00422ce0` routes the six command IDs. F2 through F5 activate the four
  finders, and F7 activates Game Options.

## Implementation boundary

The controls paint on the faction shell's canonical background layer. Input is
resolved after floating content registers its bounds, so a covered control
cannot receive the covering window's click. All six commands are recognized
and logged, but none opens a replacement panel. Their original destination
windows remain the next implementation boundary.

The always-on replacement message and status bars previously covered these
native control apertures. P46B withholds both reconstructed surfaces from the
strategic composition until their original bitmap-driven versions are restored.

This checkpoint does not complete `CMD-10` or `UIP-T01`. Original-runtime A0
captures, the disabled path, full native and browser matrices, the speed and GID
controls, the window-reference rail, original destination windows, the galaxy,
messages, and status surfaces remain open.

## Verification

| Gate | Result |
|---|---|
| `rebellion-render` source tests | 117 passed, 0 failed |
| `rebellion-app` check | Passed with known pre-existing warnings |
| Workspace suite | 609 passed, 0 failed, 20 ignored |
| Packaged WASM SHA-256 | `ae08857a1a6c551029f5839e7bb313d8b0b940ec2ba2f19e9f3dc834c1f649b9` |
| Runtime pack SHA-256 | `1ce8e2370d037423ad20683ff747245548b3eca453305596a45ef3083899a862` |
| Astra medium browser review | Passed both factions at 640x480 and 1280x800; rest, hover, press, cancel, release, four edges, commands, accelerators, overlap blocking, and four-request startup passed with no runtime or missing-asset errors |
