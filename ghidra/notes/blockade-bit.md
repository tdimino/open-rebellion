---
title: "The System Blockade Bit"
description: "Who sets system +0x88 bit 0x20, the rule FUN_0050b8e0 applies, and why the Move confirmation's blockade branch only meets a stale bit"
category: "ghidra"
created: 2026-10-02
updated: 2026-10-02
tags: [blockade, system, move-order, battle-bit]
---

# The System Blockade Bit

Recovered 2026-10-02 with Ghidra 12.1.3 headless (read-only project). Every
function named here has a `FUN_<address>.c` note in this directory.

The system's `+0x88` word holds one flag per bit, each with its own setter
(`FUN_0050a3c0` for `0x1` through `FUN_0050ac10` for `0x100000`, `0x70`
bytes apart). Each setter calls `FUN_0053a640(mask, value, &+0x88)`; when the
bit changes it updates both side views (`FUN_00539fd0(this, 1/2)`) and calls a
notification slot (`+0x20c` for `0x1`, rising by 4 per bit). Bit `0x20`'s
setter is `FUN_0050a5f0` (views `FUN_0050fec0`, slot `+0x220`); its only
caller is `FUN_0050b8e0`.

## The rule (`FUN_0050b8e0`)

`FUN_0050b8e0` recomputes three bits together:

1. `owned`: the system's side bits (`+0x24` bits 6..7) are 1, 2, or 3.
   With side 0 (neutral) all three bits are written clear.
2. `battle` (bit `0x1000`, `FUN_0050a900`): both side 1 and side 2 are
   present (`FUN_00509710(this, 1)` and `(this, 2)`).
3. `blockade` (bit `0x20`, `FUN_0050a5f0`): no battle, the system is
   populated (`+0x88` bit 0) or its side is not 3, and some active fleet in
   the system (`FUN_004ffe70(this, 1)`, types `0x08..0x0f`, no side filter)
   has side bits different from the system's.
4. Bit `0x4000` (`FUN_0050a9e0`): when the blockade bit keeps its value, the
   old `0x4000`; otherwise whether the battle bit changed (old `0x1000` against
   the new battle value). hyp: a "changed this pass" flag; no reader traced.

A side is **present** (`FUN_00509710`) when the system holds an active
(`+0x50` bit 0, mode 1 of `FUN_004f6b90`) fleet (`FUN_004ffef0`, types
`0x08..0x0f`) or an active fighter squadron (`FUN_00503a50`, types
`0x1c..0x1f`) whose side bits equal it (`FUN_005131d0`: type range, mode,
then `+0x24 >> 6 & 3`).

So for a system held by side 1 or 2: blockaded exactly when an enemy fleet
is there and the holder has no active fleet or fighter squadron there. With
both present it is a battle, not a blockade. A contested system (side 3) is
blockaded by any one side's fleets alone, when populated. A neutral system is
never blockaded.

## When it runs

- `FUN_00508250`, system vtable slot `+0xc8` (`0x0065e700`), refreshes
  every derived system state in order, the blockade rule among them.
- `FUN_00515ef0`, called by the command `FUN_00564e40` (no direct callers;
  hyp: an order or network command), resolves a system id and reruns
  `FUN_0050b8e0` when its flag is set.
- A fleet entering a system (`FUN_00508660`, slot `+0xd0`, types
  `0x08..0x0f`) does not rerun the rule. It copies the system's current
  blockade bit into the fleet's `+0x58` bit 5 (`FUN_0050c0b0`).

Who calls slot `+0xc8` and when the command is posted are untraced.

## Consequence for Move (`0x201`)

`FUN_00487cc0` confirms a Move when the first member's system has the
blockade bit and the member's side equals the system's
(`move-order.md`, "Choosing Move or Confirmed Move"). A freshly computed bit
cannot meet that: the member is an active fleet of the holder's side, so the
holder is present, and the enemy fleet that would blockade makes the other
side present too, which is a battle. The branch only meets a stale bit, for
example a holder's fleet that has just arrived in a blockaded system before
the rule reruns. The port recomputes every tick (`BlockadeSystem::advance`),
so it has no stale bit and the branch stays unreachable, matching the
original's fresh state.

## Port

`crates/rebellion-core/src/blockade.rs` `system_is_blockaded` ports the rule
(2026-10-02): both sides' fleets make a battle; a system held by one side
(`Controlled` or `Uprising`) is blockaded by the other side's fleets; a
populated `Contested` system by either side's fleets; a neutral one never.

- presence is a side's fleets or the squadrons the system holds for it
  (`System::fighters`, 2026-10-10); port: every fleet counts as active.
  The withdraw percent counts those squadrons too
  (`blockade-troop-withdrawal.md`);
- hyp: `ControlKind::Contested` is side 3, and an `Uprising` system keeps its
  holder's side bits;
- port: the rule reruns every tick (`BlockadeSystem::advance`), where the
  original reruns on the refresh and the command above, so the port never
  holds a stale bit.

The 1500-tick seed-42 dual-AI playtest is unchanged by the contested and
uprising cases (47,430 events identical apart from wall time; 3 blockades
start, 2 end), and the replay golden stays `v1:7e1061cadd1e6f22`.

Correction: `agent_docs/systems/ai-parity-tracker.md` lists `FUN_0050b8e0` as
"system-level strength scoring". It scores nothing; it writes the battle,
blockade, and change bits.
