# Object state flags (`+0x50`) and the child-list walk

Recovered 2026-09-27 from REBEXE.EXE with Homebrew Ghidra 12.1.3 (read-only,
no analysis). Used by findings F-019 and F-026 in the full-functionality audit.

## Bits

Every game object keeps seven state bits at `+0x50`. Each has a setter that
calls `FUN_0053a640(mask, value, this + 0x50)` and then notifies both side
views; the view calls its own slot `+0x178 + 4 * bit`. The notification names
come from the fleet view vtable `0x0065d438`, whose slots `+0x178..+0x190`
hold the named notifiers.

| Bit | Mask | Setter | View slot | Notifier | Name |
|-----|------|--------|-----------|----------|------|
| 0 | `0x01` | `FUN_004f7410` | `+0x178` | `FUN_004fbf90` | `GameObjUsableNotif` |
| 1 | `0x02` | `FUN_004f7480` | `+0x17c` | `FUN_004fbfe0` | `GameObjCreatedNotif` |
| 2 | `0x04` | `FUN_004f74f0` | `+0x180` | `FUN_004fc030` | `GameObjCompletedNotif` |
| 3 | `0x08` | `FUN_004f7560` | `+0x184` | `FUN_004fc080` | `GameObjDestroyedNotif` |
| 4 | `0x10` | `FUN_004f75d0` | `+0x188` | `FUN_004fc240` | `GameObjEnrouteNotif` |
| 5 | `0x20` | `FUN_004f7640` | `+0x18c` | `FUN_004fc290` | `GameObjEnrouteActiveNotif` |
| 6 | `0x40` | `FUN_004f76b0` | `+0x190` | — | existing |

The derived bits are recomputed by two unnamed base handlers:

- `0x004f7bc0`: existing (bit 6) = created (bit 1) and not destroyed (bit 3).
- `0x004f7b80`: usable (bit 0) = existing (bit 6) and completed (bit 2) and
  not en route (bit 4).
- The fleet override `FUN_004fe540` (fleet vtable slot `+0x84`) adds that at
  least one member ship is completed and not in transit (bit 5): it walks the
  fleet's existing ships of its own side (`FUN_00502e30`, mode 3, side
  `+0x24 >> 6 & 3`) and stops at the first with bit 2 set and bit 5 clear.
  A fleet with no such ship, including one with no ship at all, is not
  usable (`FUN_004fe540.c:20-35`).

## Child-list walk

`FUN_00513090`/`FUN_00513050` build an iterator over one container:
`+4` container, `+8..+0xc` type range, `+0x10` mode, `+0x14`/`+0x18` optional
side filter. `FUN_00513120` starts at the container's first child
(`FUN_00539f70` -> `+0x28` list, `FUN_005f5060`) and `FUN_005130d0` follows
the sibling link at `+0x10` (`FUN_005c7530`). The walk never descends, so it
sees only the container's direct children.

The child list is a tree (`FUN_00584150`, vtable `0x0066a168`; insert
`FUN_005f5440` -> `FUN_005f4f10`) whose comparator, slot 2 `FUN_00584320`,
orders by the child's family (slot `+4`), then by its id `+0x18`. The walk
starts at the leftmost node and follows the in-order links, so every
child-list walk visits a container's children by family, then id: container
order.

`FUN_005131d0` accepts a child whose type (slot `+4`) is in range and whose
state passes `FUN_004f6b90(child, mode)`:

| Mode | Test |
|------|------|
| 1 | `+0x50` bit 0, usable |
| 2 | `+0x50` bit 4, en route |
| 3 | `+0x50` bit 6, existing |
| 4 | always |

Containers nest: a fleet holds its ships (`FUN_00502e30`, types `0x14..0x1c`,
walked by `FUN_004fe540` on the fleet), and a mission holds its agents
(`FUN_00525bb0`, types `0x30..0x40`, walked by `FUN_00520cd0` on the mission).
A character in a fleet or on a mission is therefore not a direct child of the
system.

## Uses

- `FUN_0050d150` codes 3 to 5 (`uprising-incident.md`) call
  `FUN_004f2640(system, 1, side)`, characters `0x30..0x3c` in mode 1: the
  system's own usable characters, not those in fleets, on missions, destroyed,
  or en route.
- `FUN_00511930` (the disaster) spares facilities with bit 4 set.
