# Test Tooling

Scripts that measure the simulation rather than drive the game: Python 3
standard library and Bash, started from this Mac unless the table says
otherwise. Builds and games run on the Mac Mini over ssh
(`scripts/mini-cargo.sh`, never a local cargo).

| Script | Answers |
|---|---|
| `scripts/telemetry.py` | What did a run emit? Filter by system, type, tick range, command, trace subsystem |
| `scripts/calibrate.py` | What distributions does the headless playtest produce over many seeds? |
| `scripts/divergence.py` | Do the native game and the headless playtest end in the same state? |
| `scripts/dev-send.sh --file`, `scripts/dev-run.sh` | Run a script of developer commands (`dev-commands.md`) |
| `scripts/dev-expand.sh` | Expand a script's `Include: PATH` lines (the call both scripts above make) |
| `scripts/gates.sh` | Do the pre-commit gates pass? |
| `scripts/check-replay-equivalence.py` | Do the native and browser replays of the seed-42 campaign match? Runs on the Mini (`deterministic-replay.md`, "Validation") |
| `scripts/trace-determinism.sh` | Does recording traces change the simulation? |
| `scripts/invariants.py` | Does a telemetry file break a rule? |
| `scripts/native-checks/` | Do the native AI round scheduler's tempo, battle hold and save/reload hold? |

## telemetry.py

```bash
scripts/telemetry.py DIR|FILE [--sys S] [--type T] [--tick A..B] [--seq N] [--trace S] [--json]
scripts/telemetry.py DIR|FILE --orders [--cap N]
scripts/telemetry.py DIR|FILE --strip-wall [-o OUT]
```

Reads a native evidence folder's `telemetry.jsonl` or any playtest JSONL
(`GameEventRecord` lines: `tick`, `wall_ms`, `system`, `event_type`,
`details`). Native lines add `source`, an optional `seq` (the developer
command in flight) and `trace` records carrying a subsystem. Missing fields
are tolerated. `--json` prints the matching lines unchanged. `--orders` reads
`ai_action` events (day = `tick`) and prints per faction the median and max
orders in a side-day, the share at or above `--cap`, the total, and refused
and held counts; an event with an `outcome` counts only when `applied`, and
one with no faction counts under "unknown". `--strip-wall` writes the JSONL
with every `wall_ms` removed and keys sorted, so two runs compare byte for
byte. `load`, `matches` and `order_counts` serve the other scripts.

## calibrate.py

```bash
scripts/calibrate.py --seeds 1..200 --ticks 400 [--dual-ai] [--jobs 4] [--out DIR]
scripts/calibrate.py --from DIR     # analyze seed-*.jsonl already on disk
scripts/calibrate.py --from DIR --speed medium   # orders bound; default fast
```

Builds `rebellion-playtest` once on the Mini (`mini-cargo.sh --keep`), runs
the binary per seed over ssh, `--jobs` at a time, pulls each seed's JSONL to
`OUT/seeds/` (default `.artifacts/calibration/<stamp>`) and deletes it on the
Mini. It writes `report.md` and one CSV per table:

- mission success by kind with 95% Wilson intervals; from the missions
  traces the playtest records (`--trace`, default `missions`; decision
  `member_roll` with `kind`, `chance`, `skill`, `succeeded`), logged chance
  against realized frequency by odds bucket and realized success by skill
  bucket; `table_roll` traces get their own table;
- space battles: outcomes, and the Alliance (attacker) win rate by strength
  ratio, from the hull each side brought (`combat_space`
  `alliance_before.hull / empire_before.hull`; the `combat/pre_battle` trace
  has the same figures but no winner);
- the first tick each side reaches each research level in each tree, over the
  seeds that reached it. A level is credited only when an unlock names it, so
  levels held from the start or skipped past do not appear;
- control share from the campaign snapshots (every 250 ticks, so a run under
  250 ticks has none) and systems changing hands per 100 ticks.

Reference values are in `scripts/calibration-reference.json`, each with a
`src`. Cited: the research timers (`ghidra/notes/timer-scheduler.md:147`,
`research.md:21`), the research points per success (`research.md:33`), the
research mission draw against effective skill (`research.md:31`; the report
runs a consistency check of trace chance against skill/100, printing "not
checked (no traces)" without traces; both come from one variable, so it is not
parity evidence), and the level cap of 15 (`golden_values.json`, mechanics-doc source).
Everything else is `null` with status `TODO` until a Ghidra read or a VM
capture supplies it. Never fill a reference from a headless run: the report
would then grade the code against itself.

## divergence.py

```bash
scripts/divergence.py SAVE --days N [--out DIR]
```

Runs the native game (`launch-native.sh --live --telemetry --load`) and the
headless playtest (`--load --ticks --save-out`) from the same save for N days.
Native resolves its Battle Alerts and closes advice so no modal stops the wait
(`Battle policy: simulate`, `Advice policy: close`), then fast-forwards with
`Fast forward: on`, `Wait until day <D+N>` and `Save: divergence-end` (written to `<evidence>/saves/divergence-end.reb`). It
waits for the seq-1 `Load:` result and fails if the load was refused.

The verdict comes from the event streams, restricted to the types both runs
emit (`combat_space`, `mission_resolved`, `ai_action`, `blockade_*`,
`research_unlocked`, `control_changed`) and compared per type in order, so the
first difference names a tick and a type. The end saves' header fingerprints
are printed for information only. The runs' RNG streams desync by construction
(the playtest draws 1024 rolls a tick up front, the native game draws per
system), and the rolls decide outcomes, so the worlds differ and no state hash
would match. The event-stream verdict stands until the native tick draws rolls
as `run_simulation_tick` does; until then a divergence report is the expected
result, and it shows where the runs part.

At most one native game runs per checkout on the Mini, and `launch-native.sh`
stops its checkout's earlier instance. Never run `divergence.py` while a game
from the same checkout is running: it refuses when it finds one. It kills the
game it started when it finishes.

## gates.sh, trace-determinism.sh, invariants.py

```bash
scripts/gates.sh [--keep-going] [--only NAME[,NAME]]
scripts/trace-determinism.sh [--seed N] [--ticks N]
scripts/invariants.py DIR|FILE [--cap N]
```

`gates.sh` runs the pre-commit gates through `mini-cargo.sh` (workspace tests,
the same tests with `interface-test-fixtures` as `fixtures`, the ignored replay
golden, `telemetry_coverage`, `provenance-scan`, the WASM `cargo check`,
`beads` (no `br lint` warning on an open bead, no dependency cycle, no
closed bead with an unchecked box, `task-index.py --check` clean), two leak greps on `.beads/issues.jsonl`), one
PASS/FAIL line each, and stops at the first failure unless `--keep-going`. The second leak
grep reads its pattern from `$GATES_PRIVATE_NAME` and skips when unset. It
never regenerates a golden.

`trace-determinism.sh` runs the built release playtest on the Mini twice
(`--dual-ai`, `--trace all` and no trace), compares the fingerprints of the
saves each writes, and prints MATCH or DIFFER.

`invariants.py` checks rules in its `RULES` list against one telemetry file and
exits 1 on any FAIL. `orders-per-side-per-day` caps applied AI orders per
(day, faction) and SKIPs while records carry no per-order `faction`. It shares
`telemetry.order_counts` with `telemetry.py --orders` and calibrate.py, which
grades the maximum against the original's bound for `--speed` (5 at Fast, 10
otherwise; the telemetry does not record the speed).

## native-checks (the AI round scheduler's native run)

Command scripts under `scripts/native-checks/` plus one reader. Each script
exists per player side (`-empire`, `-alliance`) and includes
`scripts/stages/unattended.txt` after `Start game: <side>` and `Skip briefing`;
the AI that runs is the other side,
so a run's `ai_day` records name one faction.

| Script | Stages |
|---|---|
| `scheduler-tempo-<side>.txt` | `Game Speed: Fast`, a real-time stretch, fast forward to day 40, `Game Speed: Medium`, real time, day 60 |
| `scheduler-battle-hold-<side>.txt` | Very Slow, `Battle policy: stop`, `Battle at Coruscant` and `Battle at Ghorman` (day 0 for the Empire player, day 1 for the Alliance), alert held open 300 frames, `Simulate` with fast forward still on so the next frame crosses the day; run with `OPEN_REBELLION_TRACE=ai` |
| `scheduler-save-reload-<side>.txt` | `Game Speed: Slow`, a mid-day `Save: midday`, `Load: midday`, the saved day finishes |

```bash
E=.artifacts/native-checks/<date>-scheduler-tempo-empire
ssh mac-mini-ts "cd or-native/open-rebellion && (scripts/launch-native.sh --live --seed 42 --telemetry --evidence $E >/dev/null 2>&1 &)"
scripts/dev-run.sh $E scripts/native-checks/scheduler-tempo-empire.txt --keep-going
scripts/native-checks/check-scheduler.py $E                  # tempo
scripts/native-checks/check-scheduler.py $E --battle-hold    # battle-hold run (SYSTEM optional)
scripts/native-checks/check-scheduler.py $E --save-reload    # save-reload run; add --timeout 60 to dev-run
```

`check-scheduler.py` prints PASS, FAIL or SKIP per check and exits 1 on a FAIL:
orders at most 5 at Fast and 10 elsewhere (and 10 on a day whose speed rose
mid-day, which ran the slower speed's rounds, such as under the Advice index's
Very Slow hold) and at most one a round, `rounds ==
rate+1` on complete days (the first day and a speed-change day are skipped and
counted), and the `rate` field matching `speed`, all from `ai_day`. With no
`ai_day` it falls back to `telemetry.order_counts` and says so. `--battle-hold`
reads the window from `commands.out` (`Battle at`, `Wait until battle alert`,
the Simulate click) and the records' `seq`: every `held` record names a
`Battle at` system other than the alert's, at the Simulate click or later, and
with the ai trace on no `ai/round` record falls inside the open alert (without
it, no `applied` order whose `from`, `at` or `system` is the alert's system). `--save-reload` needs the first
`ai_day` after the `Load:` to hold `rate+1` rounds. Game speed has no dedicated
command: the palette label `Game Speed: <Pause|Very Slow|Slow|Medium|Fast>`
works as a script line.

