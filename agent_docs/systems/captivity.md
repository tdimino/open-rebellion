---
title: "Captivity"
description: "Capture, the captor's side bits, escape timer 0x38a, the jail abodes and release"
category: "agent-docs"
created: 2026-10-17
updated: 2026-10-17
tags: [captivity, escape, jail, rescue, simulation]
---

# Captivity

A prisoner is a character with the prisoner bit (`Character::is_captive`).
Ghidra evidence: `ghidra/notes/jail.md`; the sweep and the abodes:
`ghidra/notes/death-star-strategic.md`, "Jail abodes".

## Side

`Character::is_alliance` is the native side. While held, a prisoner carries
its captor's side bits (`FUN_004f1a80` → `FUN_004eec60`), so code that reads
the original's side bits reads `is_alliance != is_captive`: the adrift sweep's
grouping, the retreat chain a stranded prisoner tries, and the jail it gets.
The Personnel Finder lists a character on its native side's tab
(`FUN_004637a0` stores `FUN_004edc80`).

## Ways in

| Path | Port |
|------|------|
| Abduction, an exposed member's defeat | `MissionEffect::CharacterCaptured` (`captured_by` set) |
| Story capture | `EventAction::CaptureCharacter` |
| A sweep member with no place of its side | `death_star::strand` (`FUN_00549580`): captured with no captor, the retreat chain runs again on the captor's side, so it leaves as a prisoner for that side's system; with none, it is held in the jail of its side bits (`Abode::jail_of`), the other side's |

A jail occupant (`Character::held_abode`) is in no system or fleet. Its
Personnel Finder row is the name and the abode alone. Every placement and
every change of captivity takes it out (`Character::leave_abode`).

## Escape

`MissionSystem::escape_step` (tick step 5b) runs timer `0x38a` on every
prisoner: armed for GNPRTB 2584 + rand(0..=2585) days (shipped 100 and
1000), it fires an ESCAPETB roll on espionage + combat − the average combat
of the captor side's free characters beside the prisoner − the captor's
regiments there (`FUN_00561990`), then re-arms. A success frees a prisoner
off a mission; `death_star::free_escapee` clears captivity and sweeps its
location at once (event `0x307`), so it leaves an enemy system for its own
side's. hyp: a jailed escapee is swept from Yavin (Alliance) or Coruscant
(Empire).

## Ways out

Rescue (target at a system: no player window or AI planner reaches a jail
occupant), the carbonite release, a captives-only sweep group with a place
(`FUN_00549850`), and escape.
