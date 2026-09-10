---
title: "Rebellion AI Behavior and Parity Observables"
description: "Evidence-backed computer-opponent profile and measurable Open Rebellion acceptance targets"
category: reference
created: 2026-09-10
updated: 2026-09-10
tags: [reference, ai, parity, observability, campaign]
---

# AI behavior and parity observables

The original computer opponent is part of the historical identity of *Star
Wars: Rebellion*, but its defects are not the desired architecture. Open
Rebellion should reproduce the legal operations, faction asymmetry, pressure,
and campaign rhythm that players encountered. It should repair execution
failures that strand fleets, repeat unchanged states, or make the documented
victory conditions unreachable.

## What the original computer demonstrably did

### It exerted pressure, but unevenly

Human-versus-AI reports describe the computer entering sectors, bombarding
systems, invading backwater Rim worlds, raiding with Mon Calamari cruisers, and
massing Rebel ships toward Coruscant. An unprepared player could lose systems
or be forced to divert patrol fleets and garrisons.
[GameFAQs AI campaign thread](https://gamefaqs.gamespot.com/boards/198776-star-wars-rebellion/50518406?page=2),
[GameFAQs Empire campaign thread](https://gamefaqs.gamespot.com/boards/198776-star-wars-rebellion/50893998).

The same accounts describe fleets that arrived, immediately departed, or moved
several ships independently toward one rendezvous. A human with an interdictor
could destroy those ships one at a time. Tim McDonald's contemporary review
also reports that the Empire could remain inactive for weeks.
[GameFAQs AI campaign thread](https://gamefaqs.gamespot.com/boards/198776-star-wars-rebellion/50518406?page=2),
[GameSpot review](https://www.gamespot.com/reviews/star-wars-rebellion-review/1900-2532756/).

### It used missions, sometimes recklessly

Players inferred enemy diplomats from political changes, detected enemy
missions through espionage, and reported capturing important characters at
predictable destinations. One veteran account describes the AI sending
Palpatine onto a heavily defended world after Vader and Coruscant were already
lost, handing the player the final objective.
[GameFAQs AI campaign thread](https://gamefaqs.gamespot.com/boards/198776-star-wars-rebellion/50518406?page=2),
[FFG forum archive](https://ffg-forum-archive.entropicdreams.com/topic/186253-star-wars-rebellion-1998-pc-anyone-familiar-with-it/).

### It did not scale into a strong human substitute

Farrell's 1999 guide calls the computer opponent the game's principal weakness
and recommends moving to human competition after learning the interface.
Later reviews independently report poor behavior even at the hardest setting
and say it leaves players unprepared for multiplayer strategy.
[Farrell guide](https://the-spoiler.com/STRATEGY/Lucasarts/rebellion.2.html),
[GameFAQs user review](https://gamefaqs.gamespot.com/pc/198776-star-wars-rebellion/reviews/121806),
[Terrence Bosky's 2006 MobyGames review](https://www.mobygames.com/game/1144/star-wars-rebellion/user-review/2336492/).

The official manual explains part of that experience: Medium and Hard increase
the opponent's starting systems. It does not claim a different decision engine.
Therefore a larger initial advantage should not be mistaken for evidence of
more sophisticated strategy.
[Manual, campaign setup](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt).

### Its visibility model remains under investigation

Open Rebellion's reverse-engineering notes show `FUN_00519d00` iterating the
objects returned for family `0x90`, which this project maps as explored systems;
that function has no explicit visibility parameter. This suggests broad
strategic awareness, but the current call-path and object-registry analysis does
not yet prove full omniscience. Validation must cover `FUN_0051cad0(0x90)`, its
ownership context, and handling of family `0x92` before this becomes a parity
claim. This is reverse-engineered evidence, not player testimony.
[AI fog-of-war reverse-engineering analysis](../../plans/2026-03-22-ai-fog-of-war-proposal.md#the-original-was-omniscient-by-design).

## Faction profile

The rules force behavioral asymmetry even if both factions share decision
machinery. The Alliance begins militarily weaker, has stronger early diplomacy,
can move its headquarters, and develops fighter and late capital-ship strength.
The Empire has the fixed Coruscant objective, more initial military/economic
power, stronger capital ships, assassination, and the Death Star, but must
search for a mobile target.
[Manual, Characteristics of Each Side](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt),
[Farrell guide, Alliance and Empire sections](https://the-spoiler.com/STRATEGY/Lucasarts/rebellion.2.html),
[PC Zone Supremacy walkthrough](https://www.cheatbook.de/wfiles/starwsso.htm).

Direct testimony supports Rebel-computer raids and pressure on Coruscant, but
the located corpus does not establish separate original decision weights for
the two computer factions. Open Rebellion should derive different outputs from
the documented asymmetric assets and objectives before inventing undocumented
personality constants.
[GameFAQs Empire campaign thread](https://gamefaqs.gamespot.com/boards/198776-star-wars-rebellion/50893998),
[source limitations](index.md#research-limits).

## Historical behavior, desired capability, and defects

| Evidence class | Preserve or pursue | Repair as an Open Rebellion defect |
|----------------|--------------------|-----------------------------------|
| `desired_enhancement` | Early diplomacy, recruitment, espionage, research, and production pressure. This is human strategy guidance, not direct evidence of original-AI action frequency. [Farrell](https://the-spoiler.com/STRATEGY/Lucasarts/rebellion.2.html), [PC Zone](https://www.cheatbook.de/wfiles/starwsso.htm) | Weeks of total inactivity when legal productive actions exist. The inactivity itself is `observed_original_ai`; removing non-progress is an enhancement. [GameSpot](https://www.gamespot.com/reviews/star-wars-rebellion-review/1900-2532756/) |
| `observed_original_ai` | Raids, bombardment, invasions, and pressure on Coruscant. [GameFAQs Empire campaign](https://gamefaqs.gamespot.com/boards/198776-star-wars-rebellion/50893998) | Fleets that bounce indefinitely or repeatedly restart transit, and Open Rebellion task forces that lose grouping. The testimony documents transient visits and piecemeal arrivals, not their internal cause. [GameFAQs AI campaign](https://gamefaqs.gamespot.com/boards/198776-star-wars-rebellion/50518406?page=2) |
| `desired_enhancement` | Mobile mission platforms, combined fleets, troop carriage, landing, and occupation. The rules and human strategies establish legal, coherent capability, not original-AI frequency. [GOG strategy memo](https://www.gog.com/forum/star_wars_rebellion/strategy_tips_for_swrebellion_part_one), [manual](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt) | Permanent hostile blockades that can neither receive invasion troops, withdraw, nor advance the objective. No historical source requires such a deadlock. |
| `manual_rule` and `desired_enhancement` | Alliance headquarters relocation is legal; dependable relocation under threat and viable Imperial defense of Coruscant are engineering targets. [Farrell](https://the-spoiler.com/STRATEGY/Lucasarts/rebellion.2.html), [manual](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt) | Ignoring a known qualifying headquarters or principal character forever. That prevents the manual's victory contract. [manual](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt) |
| `observed_original_ai` | Transient visits and piecemeal arrivals occurred; their motives are unknown. [GameFAQs AI campaign](https://gamefaqs.gamespot.com/boards/198776-star-wars-rebellion/50518406?page=2) | Illegal orders, unbounded event churn, duplicated fleets, or deterministic non-progress loops. These are implementation failures, not documented tactics. |

## Measurable observables

The source column distinguishes historical support from proposed regression
guards. A proposal can be changed when original telemetry or stronger testimony
appears. It must not be described as an original constant.

| Observable | Acceptance target | Evidence class and basis |
|------------|-------------------|--------------------------|
| Setup difficulty | Easy gives each side four loyal systems. Medium and Hard give the opponent progressively more. No located source establishes difficulty-specific decision changes, so one legal action vocabulary is only an Open Rebellion default. | `manual_rule` for starting advantage; `engineering_guard` for the shared vocabulary. [Source](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt) |
| Early strategic activity | When legal actors/facilities exist, each faction begins recruitment, diplomacy or intelligence, research, and production during the opening evaluation window. | `desired_enhancement`. Strategy-derived capability target, not direct evidence of original-AI frequency; exact window is fixture-defined. [Farrell](https://the-spoiler.com/STRATEGY/Lucasarts/rebellion.2.html), [MasterVodo](https://members.tripod.com/buds_korner/strategy/vodo.htm), [PC Zone](https://www.cheatbook.de/wfiles/starwsso.htm) |
| Faction differentiation | Alliance and Empire traces differ in headquarters behavior, target priority, unit mix, and available mission/Death Star actions. | `manual_rule` for asymmetric assets and objectives; trace expectations are `desired_enhancement`. [Source](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt) |
| Legal campaign operations | Deterministic fixtures prove raid, blockade, bombardment, troop transport, landing, occupation, retreat, reinforcement, and repair for both factions where legal. | `manual_rule` for legal operations; full AI exercise is `desired_enhancement`. [Manual](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt), [Rogue](https://gamefaqs.gamespot.com/pc/198776-star-wars-rebellion/faqs/19754) |
| Blockade progress | A hostile blockade with an uncaptured surface objective receives viable invasion support, chooses another productive action, or withdraws. It cannot remain permanently inert because its first landing failed. | `desired_enhancement` inferred from the documented blockade-to-invasion sequence. [PC Zone](https://www.cheatbook.de/wfiles/starwsso.htm), [GOG](https://www.gog.com/forum/star_wars_rebellion/strategy_tips_for_swrebellion_part_one) |
| Headquarters response | A threatened Alliance AI can relocate its headquarters; an Imperial AI retains a viable Coruscant defense while forces exist. Dedicated fixtures pass every run. | `manual_rule` for legal relocation and objectives; reliable response is `desired_enhancement`, not observed original-AI frequency. [Farrell](https://the-spoiler.com/STRATEGY/Lucasarts/rebellion.2.html), [manual](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt) |
| Principal-character handling | Known enemy principals become actionable objectives. Friendly principals are not repeatedly dispatched alone into a known hostile stronghold without a mission need and support. | `manual_rule` plus an `observed_original_ai` failure; safe pursuit is `desired_enhancement`. [Manual](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt), [FFG archive](https://ffg-forum-archive.entropicdreams.com/topic/186253-star-wars-rebellion-1998-pc-anyone-familiar-with-it/) |
| Standard and HQ-only victory | Both factions pass deterministic fixtures for every objective combination. Imperial HQ destruction, Coruscant capture, and leader custody remain distinct. A Death Star may satisfy the Imperial HQ component but cannot bypass Standard leaders; its loss is nonterminal. | `manual_rule`. [Source](official-campaign-contract.md#standard-victory) |
| Death Star path | A permitted Imperial fixture constructs, escorts, fires, applies political effects, cleans up the destroyed system, and contributes only the correct Imperial HQ component. An Alliance fixture can attempt sabotage or strike resolution without receiving an automatic win merely for destroying the station. | `manual_rule` and `desired_enhancement`. [Manual](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt), [Farrell](https://the-spoiler.com/STRATEGY/Lucasarts/rebellion.2.html) |
| Fleet/transit health | Across five 5,000-tick seeds: transit at most 10%, accepted moves at most 1.5 times arrivals, and final fleet arena at most 3 times initial. | `engineering_guard`, not a historical measurement. [F-007E evidence](../../qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-ai-campaign-logistics.md) |
| Encounter volume and spread | Record completed engagement episodes per seed, excluding retry ticks and unchanged stalemates, with bombardments reported separately. Retain 400 as a runaway upper bound; treat 50 battles, eight systems, and a 40% busiest-system share as diagnostics until calibrated. Bind every result to galaxy size, difficulty, victory mode, AI cadence, and configuration hash. | `engineering_guard`. No located historical source establishes these numbers. [Audit](../../qa/2026-09-08-full-functionality-audit/audit-report.md#f-007-long-running-ai-simulation-exhibits-runaway-fleet-behavior) |
| Faction participation | Dedicated fixtures prove both factions can perform every legal operation. Campaign probes flag prolonged starvation while productive actions exist. A 10% minority attack share remains diagnostic, not a balance or parity constant. | `observed_original_ai` supports activity by both sides; thresholds are `engineering_guard`. [GameFAQs Empire campaign](https://gamefaqs.gamespot.com/boards/198776-star-wars-rebellion/50893998), [GameFAQs AI campaign](https://gamefaqs.gamespot.com/boards/198776-star-wars-rebellion/50518406?page=2) |
| Victory time | Record the distribution by faction, difficulty, size, and game type. Do not set a historical hard limit from the available 300-to-1177-day anecdotes. Require bounded objective progress and dedicated victory fixtures first. | `observed_original_ai` and human reports for broad duration only; bounds remain `engineering_guard`. [GameFAQs durations](https://gamefaqs.gamespot.com/boards/198776-star-wars-rebellion/50518406?page=2), [Pander finale](https://lparchive.org/Star-Wars-Rebellion/Update%2039/) |
| Native/WASM equivalence | Identical seed, setup, commands, and runtime data produce identical checkpoints in both runtimes. | `engineering_guard`, not an original-game claim. [Replay evidence](../../qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-replay-wasm-equivalence.md) |

## Current Open Rebellion diagnosis

The present five-seed F-007E checkpoint has healthy transit, arrival, repair,
troop transport, landing, ground combat, occupation, and capture telemetry. It
still ends every seed without victory, leaves the Alliance with zero or one
controlled system, and produces only 10 to 19 space-plus-ground engagement
episodes per seed. One faction stops mounting a meaningful campaign and the
dominant faction does not close the known objectives. The newly verified
transport and occupation path is real, but campaign noncompletion remains
multi-causal.
[F-007E troop transport and occupation evidence](../../qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-troop-transport-occupation.md#five-seed-campaign-probe).

The next tuning pass should therefore prioritize productive blockade follow-up,
Alliance industrial recovery, faction-aware headquarters/principal targeting,
capture/evasion, headquarters relocation, diplomacy, intelligence, research,
uprisings, bombardment, and complete victory pursuit. The faction-specific
victory contract now passes dedicated fixtures. Raising raw attack counts
without the remaining behaviors would satisfy a counter while missing the
campaign described by the sources.

## Unknowns to keep open

- No located source provides a statistically sampled AI-versus-AI campaign.
- No located source establishes original attack weights, battle-count ranges,
  target-spread percentages, or time-to-victory percentiles.
- The corpus does not prove whether Easy, Medium, and Hard alter hidden AI
  parameters beyond the manual's stated starting-system advantage.
- Player reports disagree in tone about how dangerous the AI feels. Experience,
  faction, random setup, and game speed are uncontrolled variables.
- The source record supports distinct faction outcomes but not a complete map
  of separate Rebel and Imperial decision functions.

These gaps should be addressed with original-binary analysis, controlled play
captures, and save-state telemetry. New evidence should be added to
[sources.json](sources.json) and reflected here before changing a parity claim.

[Return to the campaign-history index](index.md).
