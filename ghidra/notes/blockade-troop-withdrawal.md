---
title: "Blockade Troop Withdrawal"
description: "How regiments are lost running a blockade: system withdraw percent, per-regiment copy, and the departure roll"
category: "ghidra"
created: 2026-09-26
updated: 2026-09-26
tags: [blockade, troops, withdraw-percent, event-0x340]
---

# Blockade Troop Withdrawal

Recovered 2026-09-26 with Ghidra 12.1.3 headless (read-only project). Every
function named here has a `FUN_<address>.c` note in this directory.

## Rule

1. **System withdraw percent** (`system +0x74`), set by `FUN_0050b310` through
   `FUN_00509fc0` (range-checked 0..100 by `FUN_0053fc90`):
   - 100 by default.
   - If the system is blockaded (`+0x88 & 0x20`) and the family-`0x22` iterator
     `FUN_00527550(system, 1)` counts zero active facilities, walk every active
     fleet (`FUN_004ffe70(system, 1)`, type codes `0x08..0x0f`, no side
     filter), count its active capital ships (`FUN_00502db0`, `0x14..0x1b`),
     count fighters aboard each ship and at the system (`FUN_005039d0`,
     `0x1c..0x1f`), then
     `FUN_0055a020(ships, fighters)` =
     `FUN_0053e120(DAT_00661a88 - ships * DAT_006bb458 - fighters * DAT_006bb3cc)`.
   - `FUN_0053e120(x)` returns `x & ((x < 0) - 1)`: negatives become 0.
   - `DAT_00661a88` is the constant 100 (`.data` bytes `64 00 00 00`, never
     written). It is not a difficulty table.
   - `FUN_00558bb0` loads `DAT_006bb458` from GNPRTB `0x1e04` (7684) and
     `DAT_006bb3cc` from `0x1e05` (7685) via `FUN_0053e390`. Shipped values: 5, 2.
   - Family `0x22` is DEFFACSD record 1, the KDY-150 (TEXTSTRA 8704).
2. **Regiment copy** (`regiment +0x60`, `WithdrawPercent`, default 100 in the
   constructor `FUN_00504150`). `FUN_00514a60` moves an object between
   containers and calls `old_system->+0xd0(obj, 0)` and
   `new_system->+0xd0(obj, 1)`. The system's `+0xd0` slot is `FUN_00508660`;
   for troop type codes `0x10..0x13` with `param_2 != 0` it calls
   `FUN_0050c540`, which calls `FUN_00504470(regiment, system +0x74)`.
   The system's `+0x1ec` change slot is `FUN_00524b10` (returns 1), so a later
   change to `+0x74` does not reach regiments already present.
3. **Reset.** `FUN_004f7410` sets `+0x50` bit 0 (active) and calls slot
   `+0x110`; the regiment override `FUN_00504960` calls `FUN_00504470(100)`
   when the bit turns on.
4. **Departure roll.** `FUN_004f7640` sets `+0x50` bit 5, the in-transit bit
   (`FUN_0052bee0` sets it when an object changes container; arrival paths
   `FUN_00522fc0` and `FUN_00524b70` clear it), then calls slot `+0x124`. The
   regiment override `FUN_00504990` runs the base handler and, when the bit
   turned on, calls `FUN_0053e2f0(+0x60)` = `FUN_0053e2e0() < percent`, where
   `FUN_0053e2e0` is `FUN_0053e290(DAT_00661a88 - 1)`, a draw in `0..=99`
   (`FUN_005f5700`). On a failed roll `FUN_00504400(1)` sets `+0x5c` bit 0.
5. **Destruction and notice.** `FUN_00504400` propagates `+0x5c` bit 0 to both
   side views through `FUN_00504850`, which calls the view's slot `+0x1f8`,
   `FUN_00504a00`: it sends `TroopRegDestroyedRunningBlockade` through
   `FUN_004f8aa0` and registers event `0x340` through `FUN_0053fe40` (valid ids
   `0x300..0x391`). On the master it calls slot `+0x1f0`, `FUN_005049e0`, which
   calls `+0xac(0xd, ctx)`.

## Vtables

| Class | Constructor | Vtable | Slots used |
|-------|-------------|--------|------------|
| Troop regiment | `FUN_00504150` | `0x0065e438` (secondary at `+0x30`: `0x0065e430`) | `+0x110` `FUN_00504960`, `+0x124` `FUN_00504990`, `+0x1f0` `FUN_005049e0`, `+0x1f8` `FUN_00504a00`, `+0x1fc` `FUN_00504af0` (`TroopRegWithdrawPercentNotif`) |
| System | `FUN_00504c40` | `0x0065e638` | `+0xcc` `FUN_00508440`, `+0xd0` `FUN_00508660`, `+0x1ec` `FUN_00524b10`, `+0x228` `FUN_00511300` |
| Fleet | `FUN_004fd650` | `0x0065d438` | `+0x204` `FUN_004ff7f0` (`FleetBlockadeNotif`, event `0x181`) |

## Corrections to earlier readings

- `FUN_004ff3c0` calls slot `+0x204` on **fleet** views, reaching
  `FleetBlockadeNotif`, not `FUN_00504a00`. The regiment vtable starts at
  `0x0065e438`, so `FUN_00504a00` is slot `+0x1f8`. `0x0065e42c` is the vtable
  of the family-`0x3c` iterator built by `FUN_00504040`.
- `FUN_0050c0b0` walks the system's **fleets** (`0x08..0x0f`, mode 3 = `+0x50`
  bit 6) and copies the system's blockade bit into each fleet's `+0x58` bit 5
  when the fleet is active. It destroys nothing.
- Type codes (vtable `+4`) match DAT families: fleets `0x08..0x0f` (runtime
  only), troops `0x10`, capital ships `0x14` (Death Star `0x18`), fighters
  `0x1c`, defense facilities `0x22..0x25`, characters `0x30..0x38`, special
  forces `0x3c`.
- `crates/rebellion-core/src/economy.rs` computes this formula as a "KDY
  production modifier" and cites `FUN_0050a480`, which lies inside
  `FUN_0050a430`. `+0x88` bit 5 is the blockade bit, not a KDY flag.

## Port

`crates/rebellion-core/src/blockade.rs`: `BlockadeSystem::withdraw_percent`,
`running_regiments`, `resolve_running`. Our model has no per-system fighters
outside fleets and treats "active" as present.
