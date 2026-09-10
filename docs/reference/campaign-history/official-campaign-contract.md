---
title: "Official Rebellion Campaign Contract"
description: "Manual-backed rules that constrain campaign and AI parity"
category: reference
created: 2026-09-10
updated: 2026-09-10
tags: [reference, campaign, mechanics, parity]
---

# Official campaign contract

This document separates rules stated by the original game manual from later
player observations. The principal source is the
[Coolhand Interactive and LucasArts manual scan](https://archive.org/details/star-wars-rebellion-manual/mode/2up),
with its [searchable OCR text](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt).
A [local PDF copy](archive/star-wars-rebellion-manual.pdf) is preserved in this
repository; provenance and checksum are recorded in the
[source ledger](sources.json).
Heidi Brumbaugh and Doyle Gilstrap are credited as its writers by the
[SW:Rebellion Community manual record](https://swrebellion.net/files/file/23-star-wars-rebellion-game-manual/).

## Identity and scope

*Star Wars: Supremacy* is the United Kingdom title for the same 1998 Windows
game reviewed elsewhere as *Star Wars: Rebellion*. It is not a sequel or a
different campaign ruleset.
[GameSpot review](https://www.gamespot.com/reviews/star-wars-rebellion-review/1900-2532756/).

## Setup and difficulty

The player chooses a faction, galaxy size, Easy, Medium, or Hard difficulty,
and either Standard or Headquarters Only victory. The manual defines galaxy
sizes as 100, 150, or 200 systems. It states that both sides begin Easy with
four loyal systems, while the opponent starts with more systems on Medium and
many more on Hard. It does not say that higher difficulty changes the
opponent's decision logic.
[Manual, campaign setup](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt).

Headquarters Only ends when the Alliance captures Coruscant or the Empire
destroys the Alliance headquarters. The manual describes this as a shorter
alternative because principal-character capture is removed from the victory
requirements.
[Manual, cockpit setup and victory options](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt).

## Standard victory

The Alliance must capture and hold Coruscant, Emperor Palpatine, and Darth
Vader. The Empire must locate and destroy the mobile Alliance headquarters and
capture and hold Mon Mothma and Luke Skywalker.
[Manual, Victory Conditions](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt).

The two headquarters objectives are intentionally asymmetric. Alliance control
of Coruscant satisfies its headquarters objective. Imperial control of the
Alliance headquarters system does not, by itself, destroy the mobile base. The
Empire must bombard and destroy the headquarters and then take the system by
assault, or destroy the entire headquarters system with a Death Star.
[Preserved manual, On to Victory](archive/star-wars-rebellion-manual.pdf).

This is a conjunction, not a score threshold. Winning battles and gaining
popular support help, but neither substitutes for all selected victory targets.
In Standard mode, Death Star fire can satisfy the Imperial headquarters
objective but cannot bypass the Luke and Mon Mothma requirements. Destruction
of a Death Star is not, on its own, an Alliance victory condition. No located
manual passage or other historical source establishes a minimum-day or
grace-period requirement before a valid victory can resolve.
[Manual, Victory Conditions](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt).

## Galaxy control and economy

Each sector contains ten systems. Core systems begin inhabited and generally
have more infrastructure and resources. Most Rim systems begin unexplored and
uninhabited. Systems can be unoccupied, neutral, Imperial-controlled, or
Alliance-controlled.
[Manual, Popular Support and System Control](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt).

Control permits force movement, facility orders, and use of the system's
resources. Diplomacy can win loyalty; troops can impose control on a disloyal
system, but occupation can produce an uprising.
[Manual, Popular Support and System Control](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt).

Mines extract raw materials, refineries convert them, and refined material
feeds construction. Each matched mine/refinery pair supplies 50 maintenance
points. If maintenance is insufficient, units must be removed. Construction yards,
training facilities, and shipyards respectively produce infrastructure,
troops/special forces, and ships.
[Manual, Manufacturing and Production](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt).

## Characters, missions, and forces

Each faction begins with about seven characters and can recruit from a roster
of 30. Characters have different strengths and are intended for specialized
missions and command roles. The manual uses Leia for diplomacy and Han for
espionage as examples.
[Manual, Characters](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt).

Fleets combine capital ships, fighters, troops, characters, and special forces.
Capital ships can fight, bombard, carry fighters, transport troops, or combine
those roles. Conflict can occur between opposing fleets or between a fleet and
planetary defenses.
[Manual, Elements of Conflict](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt).

An orbiting hostile fleet automatically blockades a system. The manual states
that production stops and attempts to cross the blockade can endanger troops or
characters. Local support moves further toward the side it already favors, so a
blockade does not automatically create political support for the blockader.
[Manual, Blockade](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt).

The Empire has access to assassination and can build a Death Star if resources
permit. The Alliance has different unit strengths and the fighter capability
needed to threaten it.
[Manual, Characteristics of Each Side](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt),
[Rogue's unit and Death Star analysis](https://gamefaqs.gamespot.com/pc/198776-star-wars-rebellion/faqs/19754).

## Automation and multiplayer

The manual permits automated production/garrison management and simulated
tactical battles, but explicitly warns that the agent droid may make different
resource choices and may not play better than the human. This statement applies
to delegated management. It is not by itself a specification of the strategic
computer opponent.
[Manual, Game Automation](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt).

Head-to-head play supports LAN, Internet, modem, and null-modem connections.
The manual describes it as potentially more enjoyable than playing the
computer, while the contemporary GameSpot review reports stable LAN/Internet
play but the same slow campaign format.
[Manual, Head-to-Head Play](https://archive.org/stream/star-wars-rebellion-manual/Manual_djvu.txt),
[GameSpot review](https://www.gamespot.com/reviews/star-wars-rebellion-review/1900-2532756/).

## Binding parity consequences

- Difficulty must change starting advantage according to the selected setup.
  Smarter or more aggressive logic may be an Open Rebellion option, but it is
  not an established original rule.
- Production must depend on controlled resources and maintenance, not isolated
  facilities on politically unavailable systems.
- A valid invasion needs troop transport and must retain the possibility of an
  uprising when force replaces loyalty.
- Each faction must be able to use its complete asymmetric toolkit, including
  headquarters search/mobility, principal-character operations, and the Death
  Star path.
- Standard victory must require every selected headquarters and principal
  target. Headquarters Only must not inherit those character requirements.
- Alliance control of Coruscant and Imperial destruction of the mobile
  Alliance headquarters must remain distinct mechanics. Occupation alone is
  not proof that the Alliance headquarters was destroyed.
- Death Star destruction is a strategic event, not an independent Alliance win.
- A parity profile must not impose an uncited minimum campaign day before
  checking otherwise valid victory conditions.

These consequences are direct implementations of the cited contract. Numeric
campaign-volume targets belong in
[AI behavior and parity observables](ai-behavior-parity.md), where they are
labeled separately as engineering hypotheses.

[Return to the campaign-history index](index.md).
