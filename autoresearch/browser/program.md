# Autoresearch: Browser Functionality (WASM)

You are fixing WASM/browser gaps in Open Rebellion so the game runs fully in the browser. Your goal is to improve the browser eval score by fixing ONE gap per iteration.

## Current Browser Score

Baseline: TBD (run `python3 scripts/eval_browser.py --json` to establish)

## Eval Sub-Metrics

| Sub-Metric | Weight | What it checks |
|------------|--------|----------------|
| wasm_build | 0.25 | `bash scripts/build-wasm.sh` succeeds |
| wasm_size | 0.10 | Binary < 8MB |
| dat_loading | 0.20 | Canvas renders + DAT files parse |
| ui_rendering | 0.25 | Page has meaningful content (snapshot > 100 chars) |
| save_load | 0.20 | localStorage accessible + save roundtrip works |

## Rules

1. **Single mutable surface**: Fix ONE WASM gap per iteration.
2. **Tests must pass**: `PATH="/usr/bin:$PATH" cargo test` must pass.
3. **WASM must build**: `bash scripts/build-wasm.sh` must succeed.
4. **Never break native**: Changes must work on both native AND WASM.
5. Use `#[cfg(target_arch = "wasm32")]` for browser-only code paths.

## Priority Order

### P1: BmpCache texture loading on WASM (ui_rendering)
`crates/rebellion-render/src/bmp_cache.rs` returns `None` on WASM — no cockpit, portraits, or events.

Fix: During loading, pre-load BMPs via `macroquad::file::load_file()`. Store raw bytes in a `HashMap<(DllSource, u32), Vec<u8>>`. The `get()` method should decode from the byte cache on WASM instead of reading from disk.

Start with cockpit BMPs (~30 files, ~300KB) — enough to make the UI functional. Lazy-fetch others to avoid OOM.

Key files:
- `crates/rebellion-render/src/bmp_cache.rs` — main cache struct
- `crates/rebellion-render/src/cockpit.rs` — uses BmpCache for cockpit chrome

### P2: Save/load via localStorage (save_load)
`crates/rebellion-data/src/save.rs` has WASM stubs that return errors.

Fix: On WASM, serialize SaveState with bincode, base64-encode, store via `web_sys::Storage` (localStorage). For load, reverse the process. Max 10 save slots. localStorage gives ~5-10MB per origin — should be sufficient.

Key files:
- `crates/rebellion-data/src/save.rs` — save/load with WASM stubs
- Add `web-sys` dependency with `Storage` feature to rebellion-data's Cargo.toml (WASM only)

### P3: Audio byte loading on WASM (dat_loading / ui_rendering)
`crates/rebellion-app/src/audio.rs` has `load_sfx_bytes()` and `load_and_play_music_bytes()` but they're never called on WASM.

Fix: During the loading screen, fetch audio files via `macroquad::file::load_file()` and call the byte-level methods. Wire into the WASM startup path.

Key files:
- `crates/rebellion-app/src/audio.rs` — byte loading methods at lines ~293-324

## Build & Test

```bash
# Native build + tests
PATH="/usr/bin:$PATH" cargo test

# WASM build
bash scripts/build-wasm.sh

# Serve locally
python3 -m http.server 18080 -d web/
# Then open http://localhost:18080
```

## Eval

```bash
python3 scripts/eval_browser.py --json
python3 scripts/eval_browser.py --skip-build --json  # skip rebuild
```
