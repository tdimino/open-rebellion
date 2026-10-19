---
title: "AI Death Star: from the hull request to Destroy System 0x223"
description: "The chain of gates from module 11's hull row to the posture emitter's Destroy System order, each gate against the port"
category: "ghidra"
created: 2026-10-18
updated: 2026-10-19
tags: [ai, death-star, destroy-system, module-11, posture]
---

# AI Death Star: hull to 0x223

Sides are the game's: 1 Alliance, 2 Empire. Companions: `death-star-product.md` (section 6, the budget), `death-star-strategic.md` (AI), `ai-order-emission.md`, `ai-production.md`. Tags: **code read**, **inference**, **hyp**.

## 1. The chain

1. **Hull row** `FUN_004ca5e0` (module 11, Empire only): reached when no counted regional need (`rec+0x64` bits `1`, `0x80`, `4`, `8`, `0x10`, `0x800`) is live and the troop row (`c+0x260 < c+0x20c`) does not apply; needs more than one system in `m+0x78`, `c+4` bit 4 clear, blocker `0x20000000` clear. Mask `0x80`, key `0x200`. Code read.
2. **Class pick** `FUN_004ca910`: clears blockers, budget `FUN_004cc220`, accepts record 136 only if maintenance 600 is at most the budget; a miss sets blocker `0x20000000`, so the row is skipped on the next request and the other rows run. Code read.
3. **Build** `0x211` at the producer system: refusals `{0x90,3}` (one `[0x18,0x1c)` ship per system) and `{0xf3,1}` (spare maintenance 600). Code read.
4. **Fleet flag** `0x10` (`FUN_00433780`, class flag `0x80`): the completed hull makes its fleet record carry `0x10`, and `FUN_0041ba80` sets `c+4` bit 4, so the hull row stops. Code read.
5. **Need bits** `0x20000000` (assault planners `FUN_004b6510`, `FUN_004b8ab0`, after `FUN_004ae240`) or `0x40000000` (bombard need). Code read.
6. **Emit** `FUN_004b1070` on an arrived fleet (not en route, `+0x28 == +0x2c`): fleet bit `0x10` and target `sys+0x110 >= 1` issue `0x223` through `FUN_004b1330`, in place of an assault or bombardment. Code read.
7. **Target value** `sys+0x110`, `FUN_00433e40`: `+1` at a non-own system whose `+0x28` bit 31 is set (Coruscant by id `0x109`, or a family `0x20`-`0x21` object, `FUN_00526a60`), and `+1` for each qualifying enemy character (`:320`). Code read.

No difficulty test, timer or random draw sits on steps 4-7; the only random draws are the planners' ordinary ones.

## 2. Port comparison

| Step | Original | Port | Verdict |
|---|---|---|---|
| 1 | as above | `ai_production.rs` `ship_request` rows and order | match (inference: `listed` is `m+0x78`) |
| 2 | blocker cleared in `FUN_004ca910` | module blockers | match |
| 6 | `r.flags & fleet::DEATH_STAR && s.slot(0x32) > 0`, ASSAULT and BOMBARD arms | `ai_regions.rs` `execute` | match |
| 7 | `+0x110` | `hq_count` (`ai_records.rs`), slot `0x32` | match; `r.a & bit::HQ` carries Coruscant and `is_headquarters` |
| 5 | `FUN_004ae240` | `can_assault` | match, clause by clause |

No gate can never pass in the port, and no trigger is missing from steps 5-7. The silence comes from step 2.

## 3. Why it is silent, and how often the original fires

Budget `= 0.27 × capacity − c+0x24c`. Module 11 keeps buying ordinary capital ships while the same budget exceeds their maintenance, so the budget settles below the cheapest class (about 13-35 in the port's runs) and the 600 the hull needs never opens unless capacity jumps by roughly 2,200 or fleet maintenance drops by 600. Both the original and the port share this equilibrium; the port's three admissions in 240 samples follow from it. Inference: the original fires rarely and late, only after a 600 jump, then a build and a trip to a Coruscant or headquarters system or one with a flagged leader. Not once per game; in 5,000-day soaks zero to a few per 24 seeds is the expected range.

**Which row holds the turn when the budget opens.** The request rows run in a fixed order (`FUN_004ca5e0.c:57-139`), and the blocker lasts one request only: `FUN_004ca910.c:26` clears every blocker before the pick and sets only the failed row's (`:67-68`), so a row whose class fits the budget runs on every pass. The troop-room row comes first and wins whenever one record has `rec+0x64` bit `4` (`R+0x74 < 0` after the takers, `FUN_00477e80`; troop room is `FUN_00477cb0`'s fleet `+0x84` sum), whatever the other counts are. Its classes (mask `0x100004`, key `0x100`) cost little, so its blocker never sets and the hull row, eighth in order, is never reached. The port's seed 22 (dual AI, release playtest) shows this. The Empire's budget clears 600 from day 1,594 (603, rising to 872 by day 1,800). Every request from then to day 1,904 takes the troop-room row, with blockers 0, and builds class key 28 (maintenance 20). That is 41 transports between days 1,300 and 1,700. Fielded capital-ship maintenance still falls, from 736 on day 1,100 to 430 on day 1,552, and a record stays troop-short. The port's request, pick, troop-room counter (`FUN_00477cb0`: fleet `+0x84`), kind-6 requirement (`FUN_004b6a60`: `P+0x3c` rises to the larger of sector `+0x134` and `c+0x170` and never falls while the posture lasts) and takers (`FUN_004782a0`: ships with `0x20000` and none of `0xc03800`, from the nearest fleets with `0x8005`; `FUN_00478690`: none of `0xc23800`, from later plan entries' fleets) match the code. On seed 22 the short record is kind 6 at Sumitra, troop room 3-5 short of 18 from day 1,275. The free transports there never qualify: each one is either en route (`0x800`, in a moving fleet) or damaged (`0x1000`). Transports 13 and 47 are free and damaged from day 1,425 and change systems between Bortras, Coruscant and Yaga Minor every few weeks until day 1,600. The Empire's only repair move in the run is elsewhere (day 1,609, Bothawui to Orto). The Empire fleet moves seen between those systems in that window (fleet 28v5, Sullust to Coruscant on day 1,476 and Coruscant to Bortras on day 1,481) carry no AI order (hyp: post-battle retreats, not checked against battle events). No deviation in this chain explains the stall; whether the original's Empire avoids the losses that cause it is a question about game state, not code.

## 4. The assault planners against the port

Both planners write the same three things on success: system `+0x30 |= 0x200000` (the assault claim), the fleet's target key at `+0x2c`, fleet `+0x38 |= 0x20000000`. The port's `aim` does the same through `set_system_bits` and `set_fleet_need` (`ai_regions.rs:2372-2381`, `sys::ASSAULT_CLAIM` `ai_records.rs:134`, `fleet::ASSAULT` `ai_records.rs:93`). Code read.

**`FUN_004b8ab0` (kind 4)**, port `assault_target` arm 4 (`ai_regions.rs:2428-2433`):

| Clause | Original | Port | Verdict |
|---|---|---|---|
| fleet record | `FUN_00476110(ctx, this+0x28)` | `p.fleet` (`:2421`) | match |
| fleet regiments | `fleet+0x80 > 0` | `r.regiments_active <= 0` returns false (`:2430`) | match (`+0x80` is summed inside the unfinished guard, `FUN_00433780.c`; the port skips incomplete ships, `ai_records.rs:656`) |
| target | system record of `this+0x5c`, then `FUN_004ae240` | `p.target` filtered by `can_assault` (`:2431-2433`) | match |

**`FUN_004b6510` (kind 6)**, port arm 6 (`ai_regions.rs:2434-2459`):

| Clause | Original | Port | Verdict |
|---|---|---|---|
| fleet regiments | `fleet+0x80 > 0` | `:2430` | match |
| sector HQ test | sector record `+0x19c > 0` (slot `0x4b`, `slot::HQ`) | `hq > 0` (`:2436-2439`) | match |
| HQ list | `FUN_0041a340` words `(0,0,4,0,0,0x300200)` (enemy), key `0x32`, direction 1 | `Words::new(0,0,sys::ENEMY,0,0,0x30_0200)` ranked by `0x32` Largest (`:2441-2442`) | match |
| candidate test | id high byte `0x90..0x97`, then `FUN_004ae240` | `can_assault` (`:2456-2458`) | match |
| fall-through | when the HQ list has no accepted candidate, execution continues at `LAB_004b66c9` and runs the two-list pick below (`local_28 == 0`); a missing sector record skips both lists and returns 0 | the two-list pick runs when the HQ list found nothing (`:2459`); a missing record leaves no systems to rank | match |
| two-list pick | `(0,0,4,...)` key `0x21` direction 1 plus `(0,0,0x24,...)` key `0x14` direction 2, rank-sum, first candidate that `FUN_004ae240` accepts | `ranked_two` of enemy `0x21` Largest and enemy+held `0x14` Smallest (`:2449-2453`) | match |

In a sector with an HQ, the planner tries the HQ-ranked systems first and then the two-list pick when none is accepted; with an HQ count of 0 it runs the two-list pick alone. The fall-through never finds a target the HQ list missed: both use the enemy words `(0,0,4,0,0,0x300200)`, the second list `(0,0,0x24,...)` admits a subset, the rank-sum keeps only systems on both lists, and `FUN_004ae240` reads nothing the failed tries changed. Its one effect is the random draws of the two lists' sorts (`FUN_0041c250`).

**`FUN_004ae240` against `can_assault` (`ai_regions.rs:2905-2913`).** Every clause matches: `sys+0x30 & 0x101 == 0` (`s.s & 0x101`), `& 0x20` (`sys::HELD`, `ai_records.rs:114`), manoeuvre `max(sys+0xb4, c+0x168) <= fleet+0x5c` when the fleet is not at the system and 0 when it is (`guard`, `:2893-2899`), `sys+0x118 <= fleet+0x80` (slot `0x34`, regiments active), and the squadron clause (`sys+0xa4 <= fleet+0x74`, or `fleet+0x9c > 0`; the `fleet+0x38 & 0x10` branch is the same inequality, so it adds nothing). `fleet+0x9c` counts class-flag-2 ships outside the `0x801000` guard (`FUN_00433780.c:61-67`), the port's `laser_ships` (`ai_records.rs:648`). Code read.

**What the planners need from a hull-carrying fleet.** Either need leads to `0x223`. The assault planners need `fleet+0x80 > 0`, regiments loaded in the fleet; the hull's troop capacity (18) does not count, so a loading pass must put a regiment aboard first (posture slot 13, `loading_system`). The kind-6 bombardment planner `FUN_004b6870` needs no regiment: it sets need `0x40000000` when fleet `+0x68` (bombardment; record 136's is 15) is at least 1 and `FUN_004ae320` finds a strike at an enemy held system of the sector (`ai-order-emission.md`: regiments there, or for General Bombardment system `+0xcc`), and the emitter issues `0x223` in place of that strike (`FUN_004b1070.c:60-68`). Posture state 8 (assault) runs before state 9 (bombardment), so a fleet with a regiment aboard takes the assault need. Both routes are the same in the original and the port; code read, and tested both sides (`ai_regions.rs`, `a_kind_six_pass_takes_a_death_star_fleet_at_an_enemy_headquarters_to_destroy_system_with_or_without_regiments_for_either_side`). The emitter has no side test; the Alliance never reaches it because it has no hull (record 136 is Empire-only, and module 11's hull row is the Empire's).

## 5. `c+0x24c`: capital ships from the order

**Original (code read).** `FUN_00433620` rebuilds a fleet record and, for each child ship (no state filter, destroyed ships excluded by `FUN_00484630`'s `+0x50 & 8` test), calls `FUN_00484630` then `FUN_00433780`. `FUN_00484630.c:108` stores the ship's `+0x70` = class node `+0x20` = maintenance (`ai-production.md`, class record) for every live ship. `FUN_00433780.c` adds `param_1[0xb]` (ship `+0x70`) to fleet `+0x88` outside the `0x2000` (unfinished) guard that fences the combat and capacity sums, so fleet `+0x88` is the maintenance of every live ship in the fleet, finished or not. `FUN_0041ba80` adds fleet `+0x88` (`param_2+0x38`) to `c+0x24c` for own fleets (flag bit 1). An ordered ship exists as an unfinished member of the destination system's spare fleet from the order day (`death-star-product.md` section 1), so `c+0x24c` counts a queued capital ship's maintenance from the order.

**Port.** `Census::take` sums live ships of `world.fleets` and every queued capital ship whose class serves the side, whatever system holds the queue (`ai_production.rs`, `Census::take`); a build joins a fleet only at completion (`rebellion-data/src/integrator.rs:7045`), so the queue stands in for the spare fleet's unfinished ships. The unfinished ship keeps its side when its system changes holder (code read): the holder change walks the system's existing direct children (`FUN_004fae40` → `FUN_004fcd00` mode 3, `FUN_005131d0`, no side filter) and calls each one's slot `+0xb0`; the fleet class (vtable `0x0065d438`) has `FUN_0051ebb0` there, which returns 1 and changes nothing, so the spare fleet and the ships in it stay the side's. When the yard's own system changes holder, the walk reaches its production manager, a completed child whose class accepts any side (slot `+0x50`, `FUN_0051ebb0`), so the manager takes the new side (`FUN_004f8680.c:28-31` → `FUN_004f6f40`). Its side hook (slot `+0xf0`, `FUN_0052b790.c:7-14`) runs `FUN_0052a2c0`, which destroys every product the manager lists with reason `0x16` (`FUN_0052a2c0.c:20-36`), the unfinished ship among them, wherever its fleet sits. A queued ship therefore counts only while its yard's system stays its side's; the port drops that system's queues and ships at the holder change (`manufacturing::serve_manager_sides`). Fighters never count: a fleet holds only capital ships (`FUN_004fe160`, `fleet-join-split.md`). A finished capital ship delivered to another system travels in the port's `DeliveryState`, which the census also reads (`RoundInputs::deliveries`); the original keeps it in its destination fleet, marked en route (`FUN_0052bee0`), so its `+0x70` stays in fleet `+0x88`. The queued ships lower module 11's budget, so the hull fires less often, not more.
