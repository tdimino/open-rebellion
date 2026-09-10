---
title: "Rebellion Campaign History and AI Parity Index"
description: "Source-backed campaign behavior baseline for Star Wars: Rebellion and Star Wars: Supremacy"
category: reference
created: 2026-09-10
updated: 2026-09-10
tags: [reference, campaign, ai, parity, rebellion, supremacy]
---

# Rebellion campaign history and AI parity

This reference set defines how campaigns in the 1998 Windows game *Star Wars:
Rebellion* should unfold. The same game was released as *Star Wars: Supremacy*
in the United Kingdom. It does not cover the Fantasy Flight board game or
*Empire at War*. The identity is explicit in the contemporary
[GameSpot review by Tim McDonald](https://www.gamespot.com/reviews/star-wars-rebellion-review/1900-2532756/).

The 2026-09-10 research pass used Exa for semantic discovery and bounded
content extraction, Keenable for historical search and prompt-guided page
extraction, and a read-only `gpt-5.6-sol` researcher at medium effort through
`codex-orchestrator` for independent synthesis. Claims were then checked
against the linked pages and classified in the [source ledger](sources.json).

## Documents

| Document | Purpose |
|----------|---------|
| [Official campaign contract](official-campaign-contract.md) | Rules established by the original manual: setup, difficulty, economy, control, forces, headquarters, and victory. |
| [Human campaign testimony](human-campaign-testimony.md) | Dated strategy guides, player reports, and a documented day 0 to 1177 campaign, organized by campaign phase. |
| [AI behavior and parity observables](ai-behavior-parity.md) | What the original computer opponent demonstrably did, what it did badly, and how Open Rebellion should measure parity without preserving defects. |
| [Source ledger](sources.json) | Machine-readable source identity, provenance, confidence, uses, and limitations. |
| [Preserved original manual](archive/star-wars-rebellion-manual.pdf) | Local copy of the 1998 manual, retained with its source URL and SHA-256 in the source ledger. |

## Canonical campaign shape

1. Early campaigns are an information, diplomacy, recruitment, research, and
   production race. The Alliance is urged to disperse from exposed Yavin and
   build quietly, while the Empire converts its initial material advantage into
   search and pressure. This pattern appears across the
   [1999 Farrell guide](https://the-spoiler.com/STRATEGY/Lucasarts/rebellion.2.html),
   [PC Zone's Supremacy walkthrough](https://www.cheatbook.de/wfiles/starwsso.htm),
   and the [GOG archive of a launch-era strategy memo](https://www.gog.com/forum/star_wars_rebellion/strategy_tips_for_swrebellion_part_one).
2. Middle campaigns develop specialized production worlds, intelligence
   networks, raids, blockades, bombardment, and combined-arms invasions. Human
   strategy increasingly concentrates ships, fighters, troops, commanders, and
   special forces instead of moving isolated units. The
   [Rogue strategy guide](https://gamefaqs.gamespot.com/pc/198776-star-wars-rebellion/faqs/19754)
   and [MasterVodo guide](https://members.tripod.com/buds_korner/strategy/vodo.htm)
   describe these systems in concert.
3. Late campaigns become infrastructure denial and a search for the remaining
   headquarters and principal characters. Pander's medium, 150-system campaign
   moved from economic strain around day 103 to systematic Imperial
   infrastructure destruction on days 640 to 682, then to the final leader hunt
   and Rebel victory on day 1177.
   [Early campaign](https://lparchive.org/Star-Wars-Rebellion/Update%208/),
   [middle campaign](https://lparchive.org/Star-Wars-Rebellion/Update%2031/),
   [late campaign](https://lparchive.org/Star-Wars-Rebellion/Update%2038/),
   [finale](https://lparchive.org/Star-Wars-Rebellion/Update%2039/).

The original computer opponent was not regarded as a strong strategist. A
contemporary review reports long idle stretches, veteran guides call the AI
poor, and players document scattered arrivals, transient system visits, and
risky use of principal characters. It could still raid, bombard, invade, mass
near Coruscant, and punish an unprepared player. These are behavioral
observations, not evidence that every defect should be reproduced.
[GameSpot review](https://www.gamespot.com/reviews/star-wars-rebellion-review/1900-2532756/),
[Farrell guide](https://the-spoiler.com/STRATEGY/Lucasarts/rebellion.2.html),
[GameFAQs AI campaign thread](https://gamefaqs.gamespot.com/boards/198776-star-wars-rebellion/50518406?page=2),
[GameFAQs Empire campaign thread](https://gamefaqs.gamespot.com/boards/198776-star-wars-rebellion/50893998).

## How to use this baseline

Open Rebellion uses five evidence classes:

1. **Manual rule:** explicit manual rules. These are parity requirements.
2. **Observed original AI:** repeated or detailed player testimony about the
   computer opponent. These define
   recognizable campaign shape, with confidence recorded per claim.
3. **Reverse-engineered behavior:** evidence from the original executable,
   retained separately until the relevant call paths are fully established.
4. **Desired enhancement:** coherent capabilities supported by the rules or
   human strategy record but not demonstrated as original AI policy.
5. **Engineering guard:** numeric gates created to detect regressions when
   no original telemetry exists. The current five-seed battle range and
   geographic-spread bounds are useful stress tests, but no source establishes
   them as original-game constants.

The working target is faithful capability plus stable execution. Open Rebellion
should preserve the original asymmetry, objectives, and recognizable campaign
rhythm. It should not preserve stranded fleets, non-progress loops, illegal
orders, or failures that prevent either side from completing the documented
victory contract. The concrete distinction is maintained in
[AI behavior and parity observables](ai-behavior-parity.md).

## Research limits

No located source provides original AI-versus-AI telemetry, source-level
decision weights, or a statistically valid distribution for battles or victory
time. Campaign durations of roughly 300 to 400 days for fast play, about 800
days for a deliberately divided galaxy, roughly 1,000 days for thorough play,
and Pander's day-1177 conclusion are player reports, not release thresholds.
[GameFAQs duration discussion](https://gamefaqs.gamespot.com/boards/198776-star-wars-rebellion/50518406?page=2),
[Pander finale](https://lparchive.org/Star-Wars-Rebellion/Update%2039/).
