---
title: "Community Disassembly Address Remap"
description: "The community disassembly.zip comes from a different REBEXE.EXE build; its addresses map to ours by region shifts"
category: "ghidra"
created: 2026-09-26
updated: 2026-09-26
tags: [community-disassembly, addresses, citations, economy]
---

# Community Disassembly Address Remap

The community dump (`disassembly.zip`, 13,036 functions, read in
`docs/reports/2026-03-26-community-disassembly-cross-reference.md`) was
decompiled from a different build of `REBEXE.EXE` than ours. Its addresses do
not name functions in our binary. Our `FUN_0050a480` lies inside the operand of
a `CALL` at `0x0050a47c`, but the community `FUN_0050a480_adjust_for_kdy` is
our `FUN_0050b310`: the same blockade test, fleet, ship and fighter walks,
and final call.

The builds differ by a constant shift within each region:

| Community region | Shift to ours |
|------------------|---------------|
| `0x00409xxx` | `+0x360` |
| `0x00507xxx` | `+0xe80` |
| `0x00509e00..0x0050d7ff` | `+0xe90` |
| `0x00518f00..0x00519dff` | `+0xe00` |
| `0x0051bxxx` | `+0xe10` |
| `0x00534xxx` | `+0x1800` |
| `0x0053cxxx` | `+0x18a0` |
| `0x00557d00..0x005588ff` | `+0x1880` |
| `0x0055cxxx` | `+0x1830` |
| `0x00587xxx` | `+0x19c0` |

Each pair below was matched by comparing normalized pseudocode tokens between
the community file and our note (similarity in brackets). Every "ours" address
has a note in this directory.

| Community | Community label | Ours | Match |
|-----------|-----------------|------|-------|
| `00409cf0` | `main_game_loop` | `0040a050` | 0.79 |
| `005073d0` | `adjust_and_deploy_each_system` | `00508250` | 0.99 |
| `00509ed0` | `adjust_energy` | `0050ad60` | shift |
| `00509ef0` | — | `0050ad80` | 0.62 |
| `0050a220` | `adjust_mines` | `0050b0b0` | 0.62 |
| `0050a3a0` | `adjust_popular_support` | `0050b230` | shift |
| `0050a430` | `adjust_collection_rate` | `0050b2c0` | 0.87 |
| `0050a480` | `adjust_for_kdy` | `0050b310` | 0.86 |
| `0050a670` | `adjust_something_based_on_troop_count` | `0050b500` | 0.78 |
| `0050a710` | `adjust_garrison_requirement` | `0050b5a0` | shift |
| `0050a780` | `system_join_side` | `0050b610` | 0.80 |
| `0050a970` | — | `0050b800` | 0.74 |
| `0050aa50` | `adjust_for_fleet_fighters` | `0050b8e0` | 0.86 |
| `0050ac00` | `set_troops` | `0050ba90` | shift |
| `0050ac70` | — | `0050bb00` | shift |
| `0050ace0` | `set_orbital_shipyards` | `0050bb70` | shift |
| `0050add0` | `adjust_for_fleets` | `0050bc60` | 0.80 |
| `0050af70` | `adjust_for_fleets2` | `0050be00` | 0.83 |
| `0050b4c0` | `adjust_for_fleets3` | `0050c350` | 0.67 |
| `0050d720` | — | `0050e5b0` | shift |
| `00518f00` | `AutoClass3_seed_galaxy` | `00519d00` | 0.65 |
| `00519c50` | `seed_system_all_units` | `0051aa50` | 0.57 |
| `00519d20` | `prevent_uprising` | `0051ab20` | 0.57 |
| `0051b2c0` | `death_star_shield_type` | `0051c0d0` | shift; both use family `0x25..0x26` |
| `0051b460` | `find_death_star_shield_3` | `0051c270` | family `0x25..0x26` |
| `0053c8d0` | `calculate_percentage` | `0053e170` | 0.90 |
| `0053c8f0` | `calculate_value` | `0053e190` | shift |
| `00534640` | `setup_base_values_for_characters` | `00535e40` | 0.84 |
| `00557df0` | `get_core_energy` | `00559670` | shift |
| `00557e80` | `get_sector_support` | `00559700` | 0.97 |
| `00557fd0` | `seed_core_system` | `00559850` | shift |
| `005580f0` | `get_rim_energy` | `00559970` | 0.97 |
| `005581e0` | `seed_rim_system` | `00559a60` | 0.78 |
| `005582e0` | `adjust_value_for_strong_support` | `00559b60` | identical |
| `00558390` | `calculate_collection_rate` | `00559c10` | 0.94 |
| `005583c0` | `calculate_adjusted_support_value` | `00559c40` | 1.00 |
| `00558660` | `ai_raw_materiels_and_energy_thing` | `00559ee0` | 0.97 |
| `00558760` | `garrison_requirement` | `00559fe0` | 1.00 |
| `005587a0` | — (withdraw percent) | `0055a020` | shift |
| `005587d0` | `uprising_threshold` | `0055a050` | identical |
| `00558800` | `get_adjusted_param_7760` | `0055a080` | shift |
| `0055cbe0` | `get_decoy_table_success` | `0055e410` | tables 10/11 |
| `005871d0` | `decoy_mission` | `00588b90` | 0.83 |

Community `00534640` is not our `FUN_00534640`: our own combat notes call our
`FUN_00534640` from `FUN_005445d0`. Citations of `FUN_00534640` that come from
our Ghidra project stay as they are.

## Not remapped

- `00542050` (`is_emperor_palpatine`): the community decompile is garbled
  (two non-returning calls after a Luke Skywalker compare). Its shift puts it
  at our `FUN_005438a0`, which is true for six named characters (Leia, Luke,
  Han, the Emperor, Vader, Chewbacca). Neither build shows an Emperor combat
  modifier.
- `0055bf00` and `0055bf50` (`*_seed_common_units`): no match above 0.61.
- `00511860` (`net_notify_system_resource_incident`): our `FUN_00512670`
  (0.81) and `FUN_00512620` (0.80, `SystemDisasterIncidentNotif`) tie.
- `005022d0` (`ship_hull_damage`) is a hull getter. The cross-reference
  report paired it with our `FUN_005029a0`, which is
  `CapShipHullValueDamageNotif`, a different function.

## Semantics that change

- Community `adjust_for_kdy` is the blockade withdraw percent; see
  `blockade-troop-withdrawal.md`. `+0x88` bit 5 is the blockade bit.
- Community `get_decoy_table_success` is our `FUN_0055e410`:
  `FUN_0053e340((fdecoy != 0) + 10, (a - b) - FUN_0053e190(c, DAT_006bb710))`,
  i.e. TDECOYTB (table 10) or FDECOYTB (table 11).
- `DAT_00661a88` is the constant 100. `FUN_0053e190(a, b)` = `a * b / 100`.
