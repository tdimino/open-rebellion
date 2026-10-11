# Capital Ship Save-File Layout (Original .sav)

Reverse-engineered from `REBEXE.EXE` (x86:LE:32) and empirical analysis of
four original saves: `empire-day2-start.sav`, `empire-day28-coruscant-fighters.sav`,
`empire-fleet-to-yaga-minor.sav`, `alliance-day54-advice.sav`.

## 1  Record identity

Capital ships are stored as **node-type 0x14** records in the save tree.
The value 0x14 appears at runtime offset +0x18 of the save object; it is a
save-format node type, **not** a DatId family byte.  The DatId family bytes
for `CCapitalShip` are 0x30–0x33, 0x35–0x38, 0x3a–0x3b (see
`cpp-class-hierarchy.md` line 114).

Every capital ship appears **twice** in the save file: once in the owning
faction's subtree (the "full" or LONG record, 21 post-name fields), once in
the opposing faction's subtree (the "fog" or SHORT record, 11 post-name
fields).  Empirically confirmed across all four saves; same ship class
appears in both forms (e.g., ISD at 0x1778e LONG / 0x48ee5 SHORT in the
day-2 save).

Class codes observed in node-type 0x14 records (save-internal IDs, not
DatId family bytes):

| Code | Ship class examples                |
|------|-------------------------------------|
| 0x41 | Bulk Cruiser, Deliverance, Enfranchise |
| 0x44 | Alliance Escort Carrier             |
| 0x45 | Corellian Corvette, Arrow, Swift, Haste |
| 0x83 | Carrack Light Cruiser               |
| 0x84 | Tyrant (Victory-class SD)           |
| 0x85 | Imperial Star Destroyer             |
| 0x89 | Galleon                             |

## 2  Record header (pre-payload)

All byte offsets are from the record's start (= the self-referencing key
position).

| Offset | Size | Field                | Notes                              |
|--------|------|----------------------|------------------------------------|
| +0     | u32  | key                  | Self-referencing file position (key == absolute file offset of this record). Read by the parent tree loop, not by the record's own vtable. |
| +4     | u32  | +0x18 (node_type)    | Always 0x14 for capital ships. Read by Level 1 (FUN_005f5610). |
| +8     | u32  | +0x24 (flags)        | Status/flag word. Read by Level 2 (FUN_005405a0). Always 0x077f in day-2 start saves. |
| +12    | u32  | owner_key            | XOR-decoded owner reference. Read by Level 3 (FUN_0053a080). Values: 0x80000081/0x80000041 (owning faction), 0x800000a1/0x80000051 (opposing faction). |
| +16    | u32  | class_code           | Save-internal class identifier (0x85 for ISD, etc.). Read by an intermediate level between Level 3 and CCombatUnit. |
| +20    | u32  | has_sub34            | Boolean (always 0 or 1). Proven from write-side: `setnz al` in FUN_004f9450. When 1, a name string follows. |
| +24    | u16  | name_len             | Only present when has_sub34 == 1. Read via FUN_005f4c40 (u16 transfer). |
| +26    | bytes| name                 | `name_len` raw bytes. Read via FUN_006173b0. |

Header size: 24 bytes (no name) or 26 + name_len bytes (with name).

Example ISD: has_sub34=1, name_len=25, name="Imperial Star Destroyer 1",
header = 51 bytes total.

## 3  Serializer chain (vtable[0x14] read side)

The serializer is **bidirectional**: read and write share the same transfer
primitive (FUN_005f4ce0 → FUN_006173b0).  Every function below works for
both read and write.

```
vtable[0x14] = FUN_005000e0 (pure thunk)
    → FUN_00500160 (pure thunk)
      → FUN_00501f00  [Level 7 — CCapitalShip-specific]
          calls FUN_00557940  [Level 6]
              calls FUN_005580e0  [Level 5]
                  calls FUN_00569890 (thunk: mov eax,[esp+4]; push eax; call FUN_004f9340)
                      → FUN_004f9340  [CCombatUnit]
                          calls FUN_0053a080  [Level 3]
                              calls FUN_005405a0  [Level 2]
                                  calls FUN_005f5610  [Level 1]
                                      reads +0x18 (u32)
                              reads +0x24 (u32)
                          reads owner key (u32), XOR-decodes → +0x2c
                      [unknown intermediate: reads class_code (u32)]
                      reads has_sub34 (u32)
                      if has_sub34: reads name via FUN_005f3580 → FUN_005f3850
                          reads name_len (u16 via FUN_005f4c40)
                          reads name_len bytes (via FUN_006173b0)
                      reads +0x50 (u32)
                      reads key@+0x38 (u32 via FUN_004ECE90)
                      reads key@+0x3c (u32 via FUN_004ECE90)
                      reads +0x40 (u32)
                      reads +0x44 (u32)
                      reads +0x48 (u32)
                      reads +0x4c (u32)
                      [conditional sub54: gated by (+0x24 & 0x30) == 0
                       → ALWAYS SKIPPED for observed records (+0x24=0x077f, 0x077f & 0x30 = 0x30 ≠ 0)]
                  reads +0x58 (u32)                          [Level 5]
              reads +0x5c (u32)                              [Level 6]
          reads +0x60 (u32)                                  [Level 7]
          reads +0x64 (u32)                                  [Level 7]
```

Two candidate vtables contain FUN_005000e0 at slot [0x14]:
- 0x0065d650: [0x00]=0x004fffa0, [0x04]=0x00411dc0
- 0x0065d930: [0x00]=0x00500260, [0x04]=0x00500380

The factory table (BSS at 0x006b90fc) is runtime-populated and cannot be
read from the EXE, so the exact vtable for save-file objects remains
unidentified.

NOTE: PTR_LAB_0066c2d0 belongs to CTacticalBattleManager, not CCapitalShip.
CCapitalShip's vtable at 0x0066c2d0 has PURE VIRTUAL STUBS at slots [0x14]
and [0x18] — capital ships serialize through the save-object's vtable, not
their own.

## 4  Post-name payload: SHORT record (11 fields)

SHORT records appear in the opposing faction's subtree (fog-of-war copy).
11 u32 fields between the name and the tree reader overhead.

| Field | Byte¹ | Runtime  | Description                          |
|-------|-------|----------|--------------------------------------|
| f0    | +0    | +0x50    | Status flags (e.g. 0x00030047)       |
| f1    | +4    | key@+0x38| Key reference 1                      |
| f2    | +8    | key@+0x3c| Key reference 2 (can be XOR-encoded) |
| f3    | +12   | +0x40    | Unknown (0x100 at game start)        |
| f4    | +16   | +0x44    | Movement timer / destination index   |
| f5    | +20   | +0x48    | Unknown                              |
| f6    | +24   | +0x4c    | Unknown                              |
| f7    | +28   | +0x58    | Unknown (Level 5)                    |
| f8    | +32   | +0x5c    | Unknown (Level 6)                    |
| f9    | +36   | **+0x60**| **hull_current** (hull damage value) |
| f10   | +40   | **+0x64**| **shield_weapon_packed** (4-bit packed allocations) |

¹ Byte offset from start of post-name payload (= record_start + header_size).

Tree reader overhead immediately follows:
- READ#1 (u32, = 1 for tree nodes)
- container (u32, = 1)
- child_count (u32)
- Children: each child's key (u32, self-referencing) followed by child vtable payload.

## 5  Post-name payload: LONG record (21 fields)

LONG records appear in the owning faction's subtree (full state).
21 u32 fields, with 10 extra fields (f7–f16) between the CCombatUnit
payload and Levels 5–7.

| Field | Byte¹ | Runtime  | Description                          |
|-------|-------|----------|--------------------------------------|
| f0    | +0    | +0x50    | Status flags (same as SHORT f0)      |
| f1    | +4    | key@+0x38| Key reference 1 (same as SHORT f1)   |
| f2    | +8    | key@+0x3c| Key reference 2 (same as SHORT f2)   |
| f3    | +12   | +0x40    | Unknown (same as SHORT f3)           |
| f4    | +16   | +0x44    | Movement timer / destination index   |
| f5    | +20   | +0x48    | Unknown                              |
| f6    | +24   | +0x4c    | Unknown                              |
| f7    | +28   | ?        | Extra field 1 (0 at game start)      |
| f8    | +32   | ?        | Extra field 2 (0 at game start)      |
| f9    | +36   | ?        | **Commander DatId** — family-0x08 character reference (e.g. 0x0800077e). Zero when no commander assigned. |
| f10   | +40   | ?        | Extra field 4 (sometimes non-zero: seen 2, 3) |
| f11   | +44   | ?        | Extra field 5 (sometimes non-zero: seen 0x40000, 0x490001) |
| f12   | +48   | ?        | Extra field 6 (always 1 in owning records) |
| f13   | +52   | ?        | Extra field 7 (always 0x001e0000 = (30 << 16) at game start — possibly timer/capacity) |
| f14   | +56   | ?        | Extra field 8 (0)                    |
| f15   | +60   | ?        | Extra field 9 (0)                    |
| f16   | +64   | ?        | Extra field 10 (0)                   |
| f17   | +68   | +0x58    | Unknown (Level 5)                    |
| f18   | +72   | +0x5c    | Unknown (Level 6)                    |
| f19   | +76   | **+0x60**| **hull_current** (hull damage value) |
| f20   | +80   | **+0x64**| **shield_weapon_packed** (4-bit packed allocations) |

The 10 extra fields (f7–f16) come from a more-derived class used by the
owning faction's serializer.  The opposing faction's serializer (SHORT)
uses a base-class vtable that skips these fields entirely.

## 6  Absolute file offsets for the ISD in empire-day2-start.sav

### LONG record at 0x1778e (owning faction, 21 post-name fields)

```
Record start:     0x01778e
Header (51 bytes):
  key:            0x01778e  (= 0x0001778e, self-referencing)
  +0x18:          0x017792  (value 0x14)
  +0x24:          0x017796  (value 0x077f)
  owner:          0x01779a  (value 0x80000081)
  class:          0x01779e  (value 0x85)
  has_sub34:      0x0177a2  (value 1)
  name_len:       0x0177a6  (value 0x19 = 25)
  name:           0x0177a8  ("Imperial Star Destroyer 1")

Post-name payload starts at 0x0177c1 (byte 51):
  f0  (+0x50):    0x0177c1  value 0x00030047
  f19 (+0x60):    0x01780d  value 0x00000000  ← hull_current
  f20 (+0x64):    0x017811  value 0x00000000  ← shield_weapon_packed

Tree reader:
  READ#1:         0x017815  value 1
  container:      0x017819  value 1
  child_count:    0x01781d  value 10
  first child:    0x017821  (self-referencing key)
```

### SHORT record at 0x48ee5 (opposing faction, 11 post-name fields)

```
Record start:     0x048ee5
Header (51 bytes):
  key:            0x048ee5
  +0x18:          0x048ee9  (value 0x14)
  +0x24:          0x048eed  (value 0x077f)
  owner:          0x048ef1  (value 0x800000a1)
  class:          0x048ef5  (value 0x85)
  has_sub34:      0x048ef9  (value 1)
  name_len:       0x048efd  (value 0x19 = 25)
  name:           0x048eff  ("Imperial Star Destroyer 1")

Post-name payload starts at 0x048f18 (byte 51):
  f0  (+0x50):    0x048f18  value 0x00030047
  f9  (+0x60):    0x048f3c  value 0x00000000  ← hull_current
  f10 (+0x64):    0x048f40  value 0x00000000  ← shield_weapon_packed

Tree reader:
  READ#1:         0x048f44  value 1
  container:      0x048f48  value 1
  child_count:    0x048f4c  value 10
  first child:    0x048f50  (self-referencing key)
```

### Summary: +0x60 and +0x64 absolute offsets

| Record | +0x50 abs | +0x60 abs | +0x64 abs | Post-name field |
|--------|-----------|-----------|-----------|-----------------|
| LONG (0x1778e) | 0x0177c1 | **0x01780d** | **0x017811** | f19, f20 |
| SHORT (0x48ee5) | 0x048f18 | **0x048f3c** | **0x048f40** | f9, f10 |

## 7  Edit safety

**No checksum.** The save format uses no CRC, hash, or integrity check.
The tree reader (FUN_0053a1e0) reads key → vtable payload → children with
no validation step.

**No recomputation on load.** +0x60 (hull_current) and +0x64
(shield_weapon_packed) are stored values, not derived.  If they were
recomputed from class data tables, they would not be serialized.  The
bidirectional stream (FUN_005f4ce0 → FUN_006173b0) reads them into the
runtime object fields directly.

**Both copies must be edited.** A ship exists in both faction subtrees.
If you edit only the LONG record, the opposing faction's view will remain
stale.  Edit both records (the LONG at f19/f20 and the SHORT at f9/f10)
for consistent state.

**Self-referencing keys break on insertion.** Every record's key equals its
absolute file position.  Inserting or deleting bytes anywhere invalidates
all subsequent keys.  Field edits that preserve byte count (overwriting a
u32 in place) are safe.

## 8  Open questions

1. **Intermediate level.** Between Level 3 (owner key) and CCombatUnit
   (has_sub34), there is an unidentified intermediate level that reads the
   class_code u32.  The function has not been located in the binary.

2. **10-field source.** The 10 extra fields in LONG records (f7–f16) come
   from a more-derived class.  The exact vtable and constructor are unknown
   because the factory table (BSS 0x006b90fc) is runtime-populated.

3. **Class 0x45 anomaly.** Corellian Corvettes, Arrows, Swifts (class 0x45)
   do not match the standard tree reader pattern and may have additional
   fields or a different serializer branch.

4. **No damaged saves available.** All four saves have hull_current = 0 and
   shield_weapon_packed = 0 (no combat damage).  The f19/f20 (LONG) and
   f9/f10 (SHORT) mapping is confirmed by function chain position but not
   by observing non-zero damage values.

5. **f13 = 0x001e0000.** Consistently (30 << 16) across all owning-faction
   records at game start.  Possibly a timer, capacity, or shield allocation
   starting value.  Not yet traced to a runtime field.

## 9  Key function addresses

| Function       | Address      | Role                                    |
|----------------|--------------|------------------------------------------|
| FUN_005000e0   | 0x005000e0   | vtable[0x14] entry point (thunk)         |
| FUN_00500160   | 0x00500160   | thunk → Level 7                          |
| FUN_00501f00   | 0x00501f00   | Level 7: reads +0x60, +0x64             |
| FUN_00557940   | 0x00557940   | Level 6: reads +0x5c                    |
| FUN_005580e0   | 0x005580e0   | Level 5: reads +0x58                    |
| FUN_00569890   | 0x00569890   | thunk → CCombatUnit                      |
| FUN_004f9340   | 0x004f9340   | CCombatUnit READ (has_sub34, +0x50, keys, +0x40–0x4c) |
| FUN_004f9450   | 0x004f9450   | CCombatUnit WRITE (proves has_sub34 is 0 or 1) |
| FUN_0053a080   | 0x0053a080   | Level 3: owner key decode                |
| FUN_005405a0   | 0x005405a0   | Level 2: reads +0x24                    |
| FUN_005f5610   | 0x005f5610   | Level 1: reads +0x18                    |
| FUN_005f4ce0   | 0x005f4ce0   | Stream transfer primitive                |
| FUN_006173b0   | 0x006173b0   | Bidirectional byte transfer              |
| FUN_005f4c40   | 0x005f4c40   | u16 transfer (name_len)                  |
| FUN_005f4d90   | 0x005f4d90   | u32 transfer (read variant)              |
| FUN_005f4db0   | 0x005f4db0   | u32 transfer (write variant, same impl)  |
| FUN_004ECE90   | 0x004ece90   | Key reader (thunk → FUN_00558970)        |
| FUN_005f3580   | 0x005f3580   | Sub34 (name) read entry                  |
| FUN_005f3850   | 0x005f3850   | Sub34: reads name_len (u16) + bytes      |
| FUN_0053a1e0   | 0x0053a1e0   | Tree reader (recursive node reader)      |
| FUN_00540500   | 0x00540500   | Factory (BSS table lookup)               |
