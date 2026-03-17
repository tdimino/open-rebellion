---
title: "Fog of War & Sensors"
description: "Fleet-presence visibility, sensor radius detection, advance intel, monotonic reveal"
category: "mechanics"
created: 2026-03-16
updated: 2026-03-16
game_system: "fog-of-war"
sources:
  - type: "code"
    file: "crates/rebellion-core/src/fog.rs"
related:
  - "fleet-movement"
tags: ["fog-of-war", "sensors", "visibility", "intel"]
---

# Fog of War & Sensors

Each faction has limited knowledge of the galaxy. Systems are revealed through fleet presence, sensor range, and advance intel during transit.

## What You See

- Unknown systems appear dimmed on the galaxy map.
- Systems with your fleets are fully visible.
- As fleets approach their destination, the target system is revealed at the halfway point.
- Ships with high detection stats reveal nearby systems within sensor range.

## How It Works

### Visibility Rules

A system is **visible** to a faction if any of these conditions hold:

1. **Fleet presence**: The faction has at least one fleet stationed at that system.
2. **Advance intel**: A fleet in transit has reached 50% progress toward a destination -- the destination is revealed.
3. **Sensor radius**: A fleet's best `detection` stat creates a reveal radius of `detection * 15.0` coordinate units. All systems within that radius are revealed.

### Monotonic Reveal

Once a system is revealed, it stays revealed permanently. There is no fog-of-war regression -- visibility only grows. This matches the original game behavior.

### Sensor Radius Formula

```
radius = max_detection_in_fleet * SENSOR_MULTIPLIER
SENSOR_MULTIPLIER = 15.0 coordinate units per detection point
```

The `detection` field comes from `CapitalShipClass.detection`. The highest detection value among all ships in a fleet determines the sensor radius. Fleets with `detection = 0` have no sensor range beyond their own system.

Distance is computed as Euclidean distance between system coordinates:

```
dx = target.x - fleet_system.x
dy = target.y - fleet_system.y
revealed = (dx*dx + dy*dy) <= radius*radius
```

### Advance Intel

When `MovementOrder.progress() >= 0.5` (fleet is at least halfway to destination), the destination system is revealed. This gives the player early intelligence about what awaits at the other end.

### Seeding

At game start, `FogSystem::seed()` reveals all systems where the faction has a fleet. Subsequent reveals happen via `FogSystem::advance()` each tick.

## Source Material

- **Code**: `crates/rebellion-core/src/fog.rs` -- `FogState`, `FogSystem::advance`, `SENSOR_MULTIPLIER`
- **Code**: `crates/rebellion-core/src/movement.rs` -- `MovementOrder::progress()` for advance intel gate

## Related

- [fleet-movement.md](fleet-movement.md) -- Transit progress that triggers advance intel
