---
title: "Character Move Hook: CommandKind and the Emperor Flag"
description: "FUN_00560d50, the character manager's object-move hook: clears a moved character's CommandKind (+0x96) or those of the characters aboard a moved capital ship, and recomputes the Emperor-at-Coruscant flag (FUN_005617b0); it resolves no ground combat"
category: "ghidra"
created: 2026-03-13
updated: 2026-10-14
---

# Character Move Hook: CommandKind and the Emperor Flag

`FUN_00560d50` is the character manager's hook run by the object-move
dispatcher `FUN_00514a60`. It resolves no combat and has no gate on
regiments (`0x10`..`0x13`) or special forces (`0x3c`..`0x3f`). Ground
units and their art: [ground-unit-assets.md](ground-unit-assets.md);
officer rank: [character-roster.md](character-roster.md); bombardment:
[bombardment.md](bombardment.md).

## Families it tests

| Family (`id >> 24`) | DAT | Role in the hook |
|---|---|---|
| `0x30`..`0x3b` | `MJCHARSD` `0x30`..`0x35`, `MNCHARSD` `0x38` | The moved object is a character |
| `0x34` | Palpatine `0x34000280` | Also runs `FUN_005617b0` |
| `0x14`..`0x1b` | `CAPSHPSD` `0x14`, `0x18` | The moved object, or both containers, is a capital ship |
| `0x08`..`0x0f` | fleets | The two capital ships' fleets are compared |

## Flow

A moved **character** (`0x30`..`0x3b`, object from `FUN_00505190`):

1. If it is Palpatine (family `0x34`), call `FUN_005617b0`.
2. If its CommandKind `+0x96` is 0, stop.
3. If both container ids (`param_2`, `param_3`) are capital ships and
   their fleets (`FUN_00504dc0`, `FUN_004f6b70`) are the same fleet
   (`0x08`..`0x0f`, equal ids), keep the kind.
4. Otherwise clear it: `FUN_004ee350(character, 0)`.

A moved **capital ship** (`0x14`..`0x1b`, object from `FUN_00504f70`):
iterate its characters (`FUN_004f25c0(ship, 3)`, filter `[0x30, 0x3c)`)
and clear every nonzero CommandKind with `FUN_004ee350(character, 0)`.

Any other family returns 1 untouched.

## CommandKind setter: `FUN_004ee350`

```c
int set_command_kind(Character* c, int kind, void* ctx) {
    int ok = FUN_0053a000(c);
    if (ok) ok = FUN_0053fc90(kind, 0, kind);       // accepts any short
    if (ok && c->command_kind != kind) {            // +0x96, short
        short old = c->command_kind;
        c->command_kind = (short)kind;
        FUN_004f04f0(FUN_00539fd0(c, 1), ctx);      // Alliance view
        FUN_004f04f0(FUN_00539fd0(c, 2), ctx);      // Empire view
        c->vtable[0x330](old, c->command_kind, ctx);
    }
    return ok;
}
```

`+0x96` is the character's CommandKind: 0 none, 1 Commander, 2 Admiral,
3 General (`character-roster.md`, error string "Invalid CommandKind
value", notification `CharacterCommandKindNotif`).

## Emperor-at-Coruscant flag: `FUN_005617b0`

Looks up Palpatine (`FUN_005070d0`) and his location (`FUN_004f6b50`).
The flag is 1 when he is not a prisoner (`+0xac` bit 0 clear), is active
(`+0x50` bit 0 set), stands at Coruscant (`0x90000109`) and the system is
Empire-held (`+0x24 & 0xc0 == 0x80`); otherwise 0. `FUN_0055f650` writes it
(bit 0 of his `+0x58`; `story-encounters.md`, row 7a;
`death-star-strategic.md`). The Death Star is `CAPSHPSD` family `0x18`;
nothing here touches it.

## Decompiled files

- `FUN_00560d50.c` — the hook
- `FUN_004ee350.c` — CommandKind setter
- `FUN_005617b0.c` — Emperor-at-Coruscant flag
- `FUN_004f25c0.c` — children iterator, `[0x30, 0x3c)` filter
- `FUN_005070d0.c`, `FUN_0055f650.c`, `FUN_00514a60.c`
