# Facility Order Lists (GetOrderList vtable +0x3c)

Traced 2026-10-10 from REBEXE.EXE binary (vtable reads) and decompiled
corpus. No Ghidra headless session; vtable entries read directly from the
PE .rdata section.

---

## CFacility base GetOrderList

Every facility class (defense, manufacturing, production, HQ, mine,
refinery) shares the same wrapper at vtable `+0x3c`:

**FUN_005399e0** (wrapper, 14 vtable entries point here):

```
push esi
call FUN_00539c20    ; build list
test eax, eax
jz   short .ret
push output_addr
call FUN_00520600    ; copy list to output
.ret:
ret
```

**FUN_00539c20** (builder):

```
0x00539c2f: push 0x006b9038     ; output list address
0x00539c34: push 0x00
0x00539c3b: call FUN_004f2a10    ; init empty list
0x00539c47: push 0x01
0x00539c49: push 0x00000200      ; Scrap
0x00539c53: call FUN_00520690    ; add order to list
0x00539c68: ret
```

**Result: one entry, Scrap (0x200).** `FUN_0051d990` always appends
Encyclopedia (0x100) and Status (0x103).

### Vtable verification

All ten facility families resolve to FUN_005399e0 at +0x3c:

| Family | Class | Vtable | +0x3c value |
|--------|-------|--------|-------------|
| 0x20 | HQ (ALLFACSD) | 0x0065f280 | FUN_005399e0 |
| 0x22 | KDY-150 (battery) | 0x0065f828 | FUN_005399e0 |
| 0x23 | LNR (battery) | 0x0065f640 | FUN_005399e0 |
| 0x24 | GenCore (shield) | 0x0065fa10 | FUN_005399e0 |
| 0x25 | DS Shield | 0x0065eaf8 | FUN_005399e0 |
| 0x28 | Shipyard | 0x00660280 | FUN_005399e0 |
| 0x29 | Training Facility | 0x00660070 | FUN_005399e0 |
| 0x2a | Construction Yard | 0x0065fe60 | FUN_005399e0 |
| 0x2c | Mine | 0x006608c8 | FUN_005399e0 |
| 0x2d | Refinery | 0x006606a0 | FUN_005399e0 |

All verified by reading the PE binary at the vtable RVA + 0x3c offset.

### Contrast with CCombatUnit

Squadrons, regiments and capital ships use `FUN_00557ce0` (the
CCombatUnit base builder), which adds:

| Order | Kind | TEXTSTRA |
|-------|------|----------|
| Move | 0x201 | 12312 |
| Confirmed Move | 0x202 | 12311 |
| (invisible) | 0x204 | -- |
| Scrap | 0x200 | 12295 |

CFacility is a separate branch from CCombatUnit under CNotifyObject.
Facilities do NOT inherit the combat-unit list.

---

## Defense facility: can it Move?

**No.** Move (0x201) is absent from the facility GetOrderList. The defense
sector icon's right-click menu does show Move, Confirmed Move and Scrap
(`object_menu.rs` line 398, `Quadrant::Defenses`), but that menu comes
from the system class dispatch (`FUN_00507290` -> `FUN_0050f2e0` ->
`sector-icon-menus.md`), not from the facility class. The sector icon
aggregates all objects in the quadrant and applies Move to the mobile
ones (squadrons); the defense facilities in the same quadrant are
immobile and filtered by the Move handler.

Additional evidence: the Defenses window sets the no-drag flag (0x20000)
on pages 4 and 5 (shields and batteries), per `FUN_004a90d0` /
`sector-quadrants.md`.

---

## Menus by object kind

### Defense facility (families 0x22-0x25)

| Order | Kind | Source |
|-------|------|--------|
| Scrap | 0x200 | FUN_00539c20 |
| Encyclopedia | 0x100 | FUN_0051d990 (always appended) |
| Status | 0x103 | FUN_0051d990 (always appended) |

Enable conditions: Scrap enabled when same side and not en route
(`FUN_004f9860`). Encyclopedia always enabled. Status enabled when
selection is an object.

### Manufacturing facility (families 0x28-0x2a)

| Order | Kind | Source |
|-------|------|--------|
| Scrap | 0x200 | FUN_00539c20 (same as defense facility) |
| Encyclopedia | 0x100 | FUN_0051d990 |
| Status | 0x103 | FUN_0051d990 |

The class-level GetOrderList is identical. The manufacturing and
production windows use manager-class menus (Build Ships, Build Troops,
Build Facilities, Stop, Destination, Rename) which are separate from the
object's own order list. When right-clicked as an object (not through
the production window), a shipyard/training facility/construction yard
gets only Scrap + Encyclopedia + Status.

### Production facility (mine 0x2c, refinery 0x2d)

Same as manufacturing: Scrap + Encyclopedia + Status from the class
GetOrderList.

### Planet squadron (family 0x1c-0x1f)

| Order | Kind | Source |
|-------|------|--------|
| Move | 0x201 | FUN_00557ce0 (CCombatUnit base) |
| Confirmed Move | 0x202 | FUN_00557ce0 |
| (invisible) | 0x204 | FUN_00557ce0 (no STRATEGY record) |
| Scrap | 0x200 | FUN_00557ce0 |
| Encyclopedia | 0x100 | FUN_0051d990 |
| Status | 0x103 | FUN_0051d990 |

Enable conditions: Move and Confirmed Move enabled when same side and
not en route. Scrap same. The invisible 0x204 has no STRATEGY.DLL
RT_RCDATA record and is never shown.

---

## Port differences

| Object kind | Original menu | Port menu | Port location | Delta |
|-------------|---------------|-----------|---------------|-------|
| Defense facility | Scrap, Encyclopedia, Status | Encyclopedia, Status only | `object_menu.rs:417` (`&[]`) | **missing Scrap** |
| SystemFighter | Move, CM, Scrap, Enc, Status | Move, CM, Scrap, Enc, Status | `object_menu.rs:412-413` | matches |
| Fighter (in fleet) | (no per-squadron menu) | Move, CM, Scrap, Enc, Status | `object_menu.rs:412-413` | hyp: fleet fighters have no individual right-click in the original |
| Manufacturing facility | Scrap, Encyclopedia, Status | (uses Producer menu) | `object_menu.rs:402-408` | different context; Producer menu is the production-window menu, not the object right-click |
| Test assertion | -- | `a_squadrons_menu_offers_move_confirmed_move_and_scrap` (line 1115) | -- | test is correct for squadrons; previously also tested `DefenseFacility` returning empty, which was incorrect |

### Specific code sites

1. **`object_menu.rs:417`** -- `Some(MenuObject::DefenseFacility(_)) | None => &[]`
   Should return `&[SCRAP]`. The `SCRAP` constant is already defined at
   line 253. The defense facility's class GetOrderList builds only Scrap
   (FUN_00539c20). The comment at line 415-416 says "untraced" -- it is
   now traced.

2. **`defenses_window.rs:182`** -- `Self::Defense(key) => Some(MenuObject::DefenseFacility(key))`
   Correctly routes to `MenuObject::DefenseFacility`, which will get the
   empty list from `object_menu.rs:417`. Once that line returns
   `&[SCRAP]`, the Defenses window's defense facility right-click will
   show Scrap + Encyclopedia + Status.

---

## Source files

- `FUN_005399e0`: vtable +0x3c wrapper for all 10 facility classes
- `FUN_00539c20`: facility GetOrderList builder (Scrap only)
- `FUN_00520690`: add order to list
- `FUN_00520600`: copy list to output
- `FUN_004f2a10`: init empty list
- `FUN_00557ce0.c`: CCombatUnit base list (Move, CM, 0x204, Scrap)
- `FUN_00503930`: squadron GetOrderList wrapper
- `FUN_0051d990`: menu builder, appends Encyclopedia and Status
- `facility-ownership.md`: vtable table for all 10 facility families
- `sector-icon-menus.md`: system-class dispatch for sector quadrant icons
- `sector-quadrants.md`: Defenses window page structure, no-drag flag
- `object-popup-menu.md`: FUN_0051d990 mechanism

## Confidence

| Claim | Confidence | Evidence |
|-------|------------|----------|
| All facility classes share FUN_005399e0 at vtable +0x3c | **high** | PE binary read of all 10 vtable entries |
| FUN_00539c20 adds only Scrap (0x200) | **high** | disassembly of 60-byte function |
| Defense facilities cannot Move | **high** | Move absent from GetOrderList; no-drag flag on Defenses window pages 4-5 |
| Sector icon Move comes from system class, not facility | **high** | sector-icon-menus.md, FUN_00507290 dispatch |
| Manufacturing/production facilities share same GetOrderList | **high** | same FUN_005399e0 at all vtable +0x3c entries |
| Port DefenseFacility returns empty instead of [Scrap] | **high** | object_menu.rs line 417 |
| Fleet-carried fighters have no per-squadron right-click | medium | no separate class in the Fleet window; hyp: based on fleet menu analysis |
