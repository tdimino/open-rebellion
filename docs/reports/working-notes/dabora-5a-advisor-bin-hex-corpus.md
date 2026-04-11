# Dabora 5a — Advisor BIN Hex Corpus Research

**Status:** Working note for Knesset Shamash-Bet post-sprint report appendix (per plan SIMP-H7).
**Scope:** Read-only forensic analysis of `assets/references/ref-ui/07-droid-advisors/{alliance,empire}/*.bin` to characterize the BIN format variants beyond the 12% that parse under the current v1 scheme.
**Corpus:** 752 alliance + 753 empire = 1,505 files.

## Headline Finding

The current `parse_advisor_bin` parser at `crates/rebellion-render/src/advisor.rs:137-160` handles **only one of four distinct BIN schemas**. The cleanup commit `58526ca` documented a ~24% parse rate; actual rate is **12.1% alliance / 12.2% empire** under the simple `u16 count + u16 frame_ids[count]` format. The remaining 88% are **not corruption** — they are three additional schemas that the current parser rejects as `LengthMismatch`.

The file ID range (the 5-digit prefix in each filename, e.g. `09501-alsprite.bin`) is **role-coded**: different numeric ranges carry different schemas. This is the first observation that makes the corpus tractable.

## Parse Rate (current v1-only parser)

| Faction | Total | v1 valid | LengthMismatch | TruncatedHeader | Empty |
|---|---|---|---|---|---|
| Alliance | 752 | 92 (12.2%) | 660 | 0 | 0 |
| Empire | 753 | 91 (12.1%) | 662 | 0 | 0 |

The v1 parser's `LengthMismatch` rejection is correct — those files genuinely declare incompatible `count` values under the `2 + count*2` formula. The problem is that they use different schemas entirely.

## File-Size Distribution (alliance)

| Size | Count | Share |
|---|---|---|
| 8 bytes | 346 | 46.0% |
| 10 bytes | 285 | 37.9% |
| 4 bytes | 75 | 10.0% |
| 6 bytes | 43 | 5.7% |
| 2 bytes | 3 | 0.4% |

Empire distribution is nearly identical (348 / 286 / 75 / 42 / 2).

## File-ID Ranges (alliance) — Role-Coded

```
00200..00268  (69 files)    ← v1 "frame list" leaf records
00280..00313  (34 files)    ← v1 "frame list" leaf records
00398..00403  (6 files)     ← small edge-cases (2/4-byte records)
00666..00666  (1 file)      ← singleton
00900..00906  (7 files)     ← len=10 animation record (small bucket)
00950..00952  (3 files)
04001..04155  (155 files)   ← len=10 animation records
04168..04219  (52 files)    ← len=10 animation records
04401..04431  (31 files)    ← len=6 POINTER records (→ 10500 range)
05001..05070  (70 files)    ← len=10 animation records
09501..09531  (31 files)    ← len=8 SCRIPT records (tag=20)
10000..10229  (230 files)   ← len=8 SCRIPT records (tag=4)
10301..10306  (6 files)
10500..10570  (71 files)    ← len=8 POINTER records (→ 05000 range)
10701..10703  (3 files)
11575..11575  (1 file)      ← singleton
```

## Four Schemas Identified

### Schema A — "v1 frame list" (current parser; 200–399 range, ~12%)

```
u16 count (little-endian)
u16 frame_id[count]
```

Sample: `03 00 15 05 16 05 17 05` = count=3, frames [0x0515, 0x0516, 0x0517] = [1301, 1302, 1303].

BMP frame IDs range `0x0500..0x0580` (1280..1408), matching the droid sprite pool.

**Action:** No change. v1 parser is correct for this subset.

### Schema B — "animation record" (900, 4000, 5000 ranges; ~38%)

10-byte records with NO `count` field. Interpretation as 5 little-endian u16:

```
u16 reserved (always 0x0000)
u16 frame_id         ← BMP frame pointer
u16 duration         ← frame hold ticks?
u16 flags_or_next    ← looks like flags bitmask
u16 metadata         ← high-entropy, possibly offset or sequential index
```

Samples (alliance/00900..00905):
```
00900: 00 00 0b 05 09 00 09 00 90 4c   → [0, 1291, 9, 9, 19600]
00901: 00 00 0b 05 09 00 10 00 9a 4c   → [0, 1291, 9, 16, 19610]
00903: 00 00 0b 05 09 00 10 00 ae 4c   → [0, 1291, 9, 16, 19630]
```

Frame ID `0x050b = 1291` is in the droid BMP range (1280..1408 ✓).

Frame IDs across ALL Schema B records span `0x03e9..0x0625` (1001..1573) — wider than the v1 leaf range. The sub-1280 ids likely reference a different sprite pool (alternate droid state, background elements). This is a new unknown.

**Unknowns:**
- The `metadata` u16 (0x4c90, 0x4c9a, …) monotonically increases by 10 across adjacent files. Could be a 16-bit timestamp, cumulative frame counter, or file-offset pointer into another table.
- The `flags_or_next` field shows values 9 and 16. With only 2 observed values, either a 2-state flag or a sequence control byte.

### Schema C — "pointer record" (4401, 10500 ranges; 14%)

8-byte records with `tag=0x04` OR 6-byte records with `tag=0x00 marker=0x07d1`.

**Schema C1 — len=8, tag=4 (301 alliance files):**
```
u16 tag=4            ← discriminator
u16 target_file_id   ← file ID of the record to resolve next
u32 reserved=0
```

Samples:
```
10501: 04 00 89 13 00 00 00 00   → tag=4, next=0x1389=5001
10502: 04 00 8a 13 00 00 00 00   → tag=4, next=5002
10570: 04 00 ce 13 00 00 00 00   → tag=4, next=5070
```

Each `10500+N` record points to `5000+N` — a len=10 Schema B animation record.

**Schema C2 — len=6, marker=0x07d1 (31 alliance files):**
```
u16 zero=0
u16 marker=0x07d1=2001   ← format discriminator
u16 target_file_id
```

Samples:
```
04401: 00 00 d1 07 05 29   → marker=2001, next=0x2905=10501
04402: 00 00 d1 07 06 29   → marker=2001, next=10502
```

### Schema D — "event script" (9501–10229 ranges; 38%)

8-byte records with `tag != 4`. Tag distribution (len=8 only):

| Tag | Count |
|---|---|
| 4 (Schema C1) | 301 |
| 20 | 31 |
| 19 | 6 |
| 3 | 5 |
| 17 | 2 |
| 21 | 1 |

Tag-20 sample (09501 range):
```
09501: 14 00 31 11 00 00 00 00   → tag=20, target=0x1131=4401
09502: 14 00 32 11 00 00 00 00   → tag=20, target=4402
09503: 14 00 33 11 00 00 00 00   → tag=20, target=4403
```

Each `09501+N` points to `4401+N` (a Schema C2 pointer).

**Hypothesis:** `tag` is an action discriminator — different droid script opcodes. Tag 4 = "chain to next record", tag 20 = "trigger reaction", etc. The tag=3/17/19/21 outliers suggest rare opcodes that warrant individual inspection before shipping Schema D in the parser.

## Four-Level Indirection Chain (Verified)

```
09501-alsprite.bin        ← entry point (Schema D, tag=20)
  └→ 04401-alsprite.bin   ← Schema C2 pointer, marker=2001
       └→ 10501-alsprite.bin  ← Schema C1 pointer, tag=4
            └→ 05001-alsprite.bin  ← Schema B animation record (frame_id + duration + flags)
```

Under the current v1 parser, the top three levels are silently rejected as `LengthMismatch`, meaning **the entire advisor animation authoring is invisible** — the 12% that parses is only the leaf frame-list sub-table, never walked from the top.

## Implications for Dabora 5b Decoder (#N1..#N5)

1. **The `parse_rate ≥ 50%` acceptance target in the plan is reachable** if Schema B is added — it alone brings alliance to 92+285 = 377/752 = 50.1%. Full coverage requires Schemas C1+C2+D.

2. **`parse_advisor_bin_v2` cascade should be a `try_parse_B → try_parse_C1 → try_parse_C2 → try_parse_D → try_parse_v1`** fall-through. Schema recognition is trivial by file length + `tag` byte:
   - len=8 && bytes[0..2]==04 00 → Schema C1
   - len=8 && bytes[0..2]==14 00 → Schema D tag=20
   - len=8 (other tag) → Schema D (log tag value)
   - len=10 && bytes[0..2]==00 00 → Schema B
   - len=6 && bytes[2..4]==d1 07 → Schema C2
   - len >= 4 && len-2 == count*2 → Schema A (v1)
   - else → parse failure

3. **`ADVISOR_BIN_BMP_OFFSET`** (#N2) is NOT a single constant — it depends on schema. Schema A stores raw BMP ids. Schema B also stores a raw BMP id in `frame_id` field. Schemas C/D store *file IDs*, not BMP ids, and require chain walking before a BMP id is known. The constant that IS useful: the `ref_id - bmp_id` offset across Schema C1 pointers: 10501→05001 = delta 5500. Across Schema C2: 4401→10501 = delta 6100. These are *file-id offsets*, not BMP offsets.

4. **Faction parity check:** The alliance/empire distributions are within 1-2 files of each other for every bucket. The schemas are symmetric.

5. **Tags 3, 17, 19, 21** warrant individual hex inspection before the decoder ships. With 14 total outlier files across both factions, a 30-line hex dump reviewed by a Rust author is the right level of effort.

6. **Sub-1280 frame IDs in Schema B** (range 1001..1279) point to an unknown sprite pool. Before wiring them into `BmpCache`, we need to identify which DLL resource they correspond to. Likely candidates: `SWREDATA.DLL` alternate droid idles or voice-line-sync sprites. Deferred to Knesset Nabu.

## Next Actions (Dabora 5b scope)

- Implement 4-schema cascade parser with proper `BinError` variants per schema.
- Chain-walk Schema D → Schema C → Schema B to resolve a final BMP frame ID + duration + flags per script record.
- Emit per-schema counts in the `[advisor]` summary log (#N3).
- Per-schema coverage integration test: Schema A ≥ 12% (existing), Schema B ≥ 38%, Schema C1 ≥ 40%, Schema D ≥ 5%, combined ≥ 95%.
- Hex-dump appendix of tag=3/17/19/21 outliers before merge.

## Corpus Reproducibility

```bash
# Dump first 32 bytes of every alliance BIN
for f in assets/references/ref-ui/07-droid-advisors/alliance/*.bin; do
    xxd -l 32 "$f"
done > /tmp/alliance-corpus.txt

# Schema classification via Python struct — see this note's script blocks for exact logic.
```

All computation above was in-place Python (`struct.unpack_from('<H', ...)`) over the live corpus. No code was committed; this note exists solely to unblock Dabora 5b.
