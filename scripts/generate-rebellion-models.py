#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = ["requests", "fal-client"]
# ///
"""
3D Model Generator — generates ~63 GLB models for Star Wars Rebellion tactical combat

Dual-provider strategy:
  - Hunyuan3D Pro via fal.ai (primary, image-to-3D for hero units)
  - Meshy text-to-3D (fallback, for generic units)

Adapted from World War Watcher's generate-models.py.

Usage:
    uv run scripts/generate-rebellion-models.py                        # all models
    uv run scripts/generate-rebellion-models.py --models isd xwing     # specific
    uv run scripts/generate-rebellion-models.py --list                 # show model IDs
    uv run scripts/generate-rebellion-models.py --status               # check pending
    uv run scripts/generate-rebellion-models.py --provider meshy       # Meshy only
    uv run scripts/generate-rebellion-models.py --provider hunyuan     # Hunyuan only
    uv run scripts/generate-rebellion-models.py --skip-refine          # preview only

Prerequisites:
    FAL_KEY in environment (for Hunyuan3D Pro)
    MESHY_API_KEY in environment (for Meshy fallback)
"""

import argparse
import json
import os
import sys
import time
from datetime import datetime, timezone
from pathlib import Path

import requests

SCRIPT_DIR = Path(__file__).parent
STAGING_DIR = SCRIPT_DIR / "models-staging"
LOG_FILE = SCRIPT_DIR / "logs" / "rebellion-models.jsonl"

MESHY_API_BASE = "https://api.meshy.ai/openapi/v2/text-to-3d"
WAVESPEED_API_BASE = "https://api.wavespeed.ai/api/v3/wavespeed-ai"
POLL_INTERVAL = 5

# ── Model definitions ─────────────────────────────────────────────────────────
# Each model has: prompt, negative_prompt, provider preference, category
# Provider: "hunyuan" for iconic ships needing reference images,
#           "meshy" for generic units where text-to-3D suffices

MODELS: dict[str, dict] = {
    # ── Capital Ships (Hero) — Hunyuan image-to-3D ──
    "isd": {
        "name": "Imperial Star Destroyer",
        "prompt": "Imperial Star Destroyer, wedge-shaped hull, bridge tower, Star Wars capital ship, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, labels, annotations, dimension lines, high poly",
        "provider": "hunyuan",
        "category": "capital_ship",
    },
    "ssd": {
        "name": "Super Star Destroyer",
        "prompt": "Super Star Destroyer Executor, elongated wedge, massive capital ship, Star Wars, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, labels, annotations, high poly",
        "provider": "hunyuan",
        "category": "capital_ship",
    },
    "mon-cal": {
        "name": "Mon Calamari Cruiser",
        "prompt": "Mon Calamari Star Cruiser, organic bulbous hull, Star Wars Rebel Alliance capital ship, clean geometry, game ready, low poly",
        "negative_prompt": "organic terrain, blurry, noisy, text, labels, annotations, high poly",
        "provider": "hunyuan",
        "category": "capital_ship",
    },
    "nebulon-b": {
        "name": "Nebulon-B Frigate",
        "prompt": "Nebulon-B Frigate, elongated hull with thin connecting spar, Star Wars Rebel ship, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, labels, annotations, high poly",
        "provider": "hunyuan",
        "category": "capital_ship",
    },
    "corellian-corvette": {
        "name": "Corellian Corvette",
        "prompt": "CR90 Corellian Corvette Tantive IV, hammerhead front, 11 engines at rear, Star Wars, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, labels, annotations, high poly",
        "provider": "hunyuan",
        "category": "capital_ship",
    },
    "interdictor": {
        "name": "Interdictor Cruiser",
        "prompt": "Interdictor Cruiser with four gravity well projector domes, Star Wars Imperial ship, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, labels, annotations, high poly",
        "provider": "hunyuan",
        "category": "capital_ship",
    },
    "assault-frigate": {
        "name": "Assault Frigate",
        "prompt": "Rebel Assault Frigate Mark II, triangular hull with side fins, Star Wars, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, labels, annotations, high poly",
        "provider": "hunyuan",
        "category": "capital_ship",
    },
    "victory-sd": {
        "name": "Victory Star Destroyer",
        "prompt": "Victory-class Star Destroyer, shorter wedge hull than Imperial class, Star Wars, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, labels, annotations, high poly",
        "provider": "hunyuan",
        "category": "capital_ship",
    },
    "death-star": {
        "name": "Death Star",
        "prompt": "Death Star, massive spherical space station with superlaser dish indent, Star Wars, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, labels, annotations, high poly, planet",
        "provider": "hunyuan",
        "category": "capital_ship",
    },
    # ── Capital Ships (Generic) — Meshy text-to-3D ──
    "strike-cruiser": {
        "name": "Strike Cruiser",
        "prompt": "Imperial Strike Cruiser, compact wedge warship, Star Wars, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, high poly",
        "provider": "meshy",
        "category": "capital_ship",
    },
    "carrack-cruiser": {
        "name": "Carrack Light Cruiser",
        "prompt": "Carrack-class Light Cruiser, compact Imperial patrol ship, Star Wars, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, high poly",
        "provider": "meshy",
        "category": "capital_ship",
    },
    "dreadnaught": {
        "name": "Dreadnaught Heavy Cruiser",
        "prompt": "Dreadnaught-class Heavy Cruiser, cylindrical hull with bridge at top, Star Wars, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, high poly",
        "provider": "meshy",
        "category": "capital_ship",
    },
    "escort-carrier": {
        "name": "Escort Carrier",
        "prompt": "Rebel Escort Carrier, boxy hull with fighter bays, Star Wars, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, high poly",
        "provider": "meshy",
        "category": "capital_ship",
    },
    "bulk-cruiser": {
        "name": "Bulk Cruiser",
        "prompt": "Bulk Cruiser transport warship, angular hull, Star Wars Rebel ship, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, high poly",
        "provider": "meshy",
        "category": "capital_ship",
    },
    "galleon": {
        "name": "Imperial Galleon",
        "prompt": "Imperial Galleon transport, angular cargo warship, Star Wars, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, high poly, sailing ship",
        "provider": "meshy",
        "category": "capital_ship",
    },
    "medium-transport": {
        "name": "Medium Transport",
        "prompt": "GR-75 Medium Transport, Rebel supply ship with boxy cargo pods, Star Wars, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, high poly",
        "provider": "meshy",
        "category": "capital_ship",
    },
    # ── Fighters — Hunyuan (all iconic) ──
    "xwing": {
        "name": "X-Wing",
        "prompt": "T-65 X-Wing starfighter, four S-foils in attack position, Star Wars Rebel fighter, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, labels, annotations, high poly",
        "provider": "hunyuan",
        "category": "fighter",
    },
    "ywing": {
        "name": "Y-Wing",
        "prompt": "BTL Y-Wing bomber, twin engine nacelles, cockpit module, Star Wars Rebel bomber, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, labels, annotations, high poly",
        "provider": "hunyuan",
        "category": "fighter",
    },
    "awing": {
        "name": "A-Wing",
        "prompt": "RZ-1 A-Wing interceptor, wedge-shaped fast fighter, Star Wars Rebel, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, labels, annotations, high poly",
        "provider": "hunyuan",
        "category": "fighter",
    },
    "bwing": {
        "name": "B-Wing",
        "prompt": "A/SF-01 B-Wing heavy assault starfighter, cross-shaped profile, Star Wars Rebel, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, labels, annotations, high poly",
        "provider": "hunyuan",
        "category": "fighter",
    },
    "tie-fighter": {
        "name": "TIE Fighter",
        "prompt": "TIE Fighter, spherical cockpit with two hexagonal solar panel wings, Star Wars Imperial fighter, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, labels, annotations, high poly",
        "provider": "hunyuan",
        "category": "fighter",
    },
    "tie-interceptor": {
        "name": "TIE Interceptor",
        "prompt": "TIE Interceptor, spherical cockpit with arrow-shaped solar panel wings, Star Wars Imperial fighter, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, labels, annotations, high poly",
        "provider": "hunyuan",
        "category": "fighter",
    },
    "tie-bomber": {
        "name": "TIE Bomber",
        "prompt": "TIE Bomber, dual-pod hull with solar panel wings, Star Wars Imperial bomber, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, labels, annotations, high poly",
        "provider": "hunyuan",
        "category": "fighter",
    },
    "tie-defender": {
        "name": "TIE Defender",
        "prompt": "TIE Defender, three solar panel wings in triangular arrangement, Star Wars Imperial advanced fighter, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, labels, annotations, high poly",
        "provider": "hunyuan",
        "category": "fighter",
    },
    # ── Troops — Meshy text-to-3D ──
    "stormtrooper-regiment": {
        "name": "Stormtrooper Regiment",
        "prompt": "squad of Stormtroopers in white armor, Star Wars Imperial ground troops, clean geometry, game ready, low poly",
        "negative_prompt": "organic terrain, blurry, noisy, text, high poly, single character",
        "provider": "meshy",
        "category": "troop",
    },
    "rebel-trooper-regiment": {
        "name": "Rebel Trooper Regiment",
        "prompt": "squad of Rebel Alliance soldiers in olive uniforms with helmets, Star Wars ground troops, clean geometry, game ready, low poly",
        "negative_prompt": "organic terrain, blurry, noisy, text, high poly, single character",
        "provider": "meshy",
        "category": "troop",
    },
    "at-at": {
        "name": "AT-AT Walker",
        "prompt": "AT-AT Imperial Walker, four-legged armored transport, Star Wars, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, high poly",
        "provider": "meshy",
        "category": "troop",
    },
    "at-st": {
        "name": "AT-ST Walker",
        "prompt": "AT-ST Scout Walker, two-legged chicken walker, Star Wars Imperial, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, high poly",
        "provider": "meshy",
        "category": "troop",
    },
    "speeder-bike": {
        "name": "Speeder Bike",
        "prompt": "74-Z Speeder Bike, hovering military scout vehicle, Star Wars Imperial, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, high poly, motorcycle",
        "provider": "meshy",
        "category": "troop",
    },
    # ── Special Forces — Meshy text-to-3D ──
    "bounty-hunter": {
        "name": "Bounty Hunter",
        "prompt": "armored bounty hunter with jetpack and rifle, Star Wars mercenary, clean geometry, game ready, low poly",
        "negative_prompt": "organic terrain, blurry, noisy, text, high poly",
        "provider": "meshy",
        "category": "special_force",
    },
    "commando": {
        "name": "Commando Team",
        "prompt": "special forces commando team in tactical gear, Star Wars military operatives, clean geometry, game ready, low poly",
        "negative_prompt": "organic terrain, blurry, noisy, text, high poly",
        "provider": "meshy",
        "category": "special_force",
    },
    "spy": {
        "name": "Espionage Agent",
        "prompt": "covert spy agent in dark cloak, Star Wars intelligence operative, clean geometry, game ready, low poly",
        "negative_prompt": "organic terrain, blurry, noisy, text, high poly",
        "provider": "meshy",
        "category": "special_force",
    },
    # ── Defense Facilities — Meshy text-to-3D ──
    "planetary-shield": {
        "name": "Planetary Shield Generator",
        "prompt": "planetary shield generator dome on a base platform, Star Wars military installation, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, high poly, landscape",
        "provider": "meshy",
        "category": "facility",
    },
    "ion-cannon": {
        "name": "Ion Cannon",
        "prompt": "v-150 Planet Defender ion cannon turret, large barrel on rotating base, Star Wars, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, high poly",
        "provider": "meshy",
        "category": "facility",
    },
    "turbolaser": {
        "name": "Turbolaser Battery",
        "prompt": "ground-based turbolaser battery, dual barrel cannon on platform, Star Wars military defense, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, high poly",
        "provider": "meshy",
        "category": "facility",
    },
    "shipyard": {
        "name": "Orbital Shipyard",
        "prompt": "orbital shipyard space station, docking arms and construction gantries, Star Wars, clean geometry, game ready, low poly",
        "negative_prompt": "organic, blurry, noisy, text, high poly, planet",
        "provider": "meshy",
        "category": "facility",
    },
}

ALL_MODEL_IDS = list(MODELS.keys())


def get_meshy_key() -> str:
    key = os.environ.get("MESHY_API_KEY", "")
    if not key:
        print("ERROR: MESHY_API_KEY not set", file=sys.stderr)
        sys.exit(1)
    return key


def get_fal_key() -> str:
    key = os.environ.get("FAL_KEY", "")
    if not key:
        print("ERROR: FAL_KEY not set", file=sys.stderr)
        sys.exit(1)
    return key


def get_wavespeed_key() -> str:
    key = os.environ.get("WAVESPEED_API_KEY", "")
    if not key:
        print("ERROR: WAVESPEED_API_KEY not set. Sign up at wavespeed.ai", file=sys.stderr)
        sys.exit(1)
    return key


def log_event(event: dict) -> None:
    LOG_FILE.parent.mkdir(parents=True, exist_ok=True)
    with open(LOG_FILE, "a") as f:
        f.write(json.dumps({**event, "ts": datetime.now(timezone.utc).isoformat()}) + "\n")


# ── Meshy provider ────────────────────────────────────────────────────────────

def meshy_generate(session: requests.Session, model_id: str, model: dict, skip_refine: bool, quiet: bool) -> Path | None:
    """Generate via Meshy text-to-3D. Returns output path or None."""
    payload = {
        "mode": "preview",
        "prompt": model["prompt"],
        "negative_prompt": model["negative_prompt"],
        "model_type": "lowpoly",
    }

    resp = session.post(MESHY_API_BASE, json=payload)
    resp.raise_for_status()
    task_id = resp.json()["result"]

    if not quiet:
        print(f"  [{model_id}] Meshy preview submitted: {task_id}")
    log_event({"action": "meshy_preview", "model_id": model_id, "task_id": task_id})

    # Poll for completion
    task = poll_meshy(session, task_id, model_id, "preview", quiet)
    if task.get("status") != "SUCCEEDED":
        return None

    if not skip_refine:
        refine_payload = {
            "mode": "refine",
            "preview_task_id": task_id,
            "enable_pbr": True,
            "remove_lighting": True,
        }
        resp = session.post(MESHY_API_BASE, json=refine_payload)
        resp.raise_for_status()
        refine_id = resp.json()["result"]

        if not quiet:
            print(f"  [{model_id}] Meshy refine submitted: {refine_id}")
        task = poll_meshy(session, refine_id, model_id, "refine", quiet)
        if task.get("status") != "SUCCEEDED":
            return None

    # Download GLB
    glb_url = task.get("model_urls", {}).get("glb")
    if not glb_url:
        print(f"  [{model_id}] No GLB URL", file=sys.stderr)
        return None

    return download_glb(session, glb_url, model_id, "-meshy")


def poll_meshy(session: requests.Session, task_id: str, model_id: str, stage: str, quiet: bool) -> dict:
    last_progress = -1
    while True:
        resp = session.get(f"{MESHY_API_BASE}/{task_id}")
        resp.raise_for_status()
        task = resp.json()

        status = task.get("status", "UNKNOWN")
        progress = task.get("progress", 0)

        if status == "SUCCEEDED":
            if not quiet:
                print(f"  [{model_id}] {stage} complete!")
            return task

        if status == "FAILED":
            error_msg = task.get("task_error", {}).get("message", "unknown")
            print(f"  [{model_id}] {stage} FAILED: {error_msg}", file=sys.stderr)
            log_event({"action": f"{stage}_failed", "model_id": model_id, "error": error_msg})
            return task

        if not quiet and progress != last_progress:
            print(f"  [{model_id}] {stage}: {progress}%")
            last_progress = progress

        time.sleep(POLL_INTERVAL)


# ── Hunyuan provider ─────────────────────────────────────────────────────────

def hunyuan_generate(model_id: str, model: dict, quiet: bool) -> Path | None:
    """Generate via Hunyuan3D Pro on fal.ai. Returns output path or None."""
    try:
        import fal_client
    except ImportError:
        print("ERROR: fal-client not installed. Run: uv pip install fal-client", file=sys.stderr)
        return None

    if not quiet:
        print(f"  [{model_id}] Hunyuan3D Pro (fal.ai) — text-to-3D...")
    log_event({"action": "hunyuan_submit", "model_id": model_id})

    # NOTE: Using text-to-3D for initial batch generation. For hero units,
    # switch to image-to-3D with nano-banana-pro concept art as input via
    # fal-ai/hunyuan-3d/v3.1/pro/image-to-3d (requires input_image_url field).
    try:
        result = fal_client.run(
            "fal-ai/hunyuan-3d/v3.1/pro/text-to-3d",
            arguments={
                "prompt": model["prompt"],
                "generate_type": "Geometry",
                "face_count": 80000,
            },
        )
    except Exception as e:
        print(f"  [{model_id}] Hunyuan FAILED: {type(e).__name__}: {e}", file=sys.stderr)
        log_event({"action": "hunyuan_failed", "model_id": model_id, "error": str(e)})
        return None

    glb_url = result.get("model_mesh", {}).get("url")
    if not glb_url:
        print(f"  [{model_id}] No GLB URL in Hunyuan response", file=sys.stderr)
        return None

    session = requests.Session()
    return download_glb(session, glb_url, model_id, "-hunyuan")


# ── WaveSpeedAI provider ─────────────────────────────────────────────────────

def wavespeed_generate(session: requests.Session, model_id: str, model: dict, quiet: bool) -> Path | None:
    """Generate via WaveSpeedAI Hunyuan3D v3.1 Rapid ($0.0225/model). Returns output path or None."""
    endpoint = f"{WAVESPEED_API_BASE}/hunyuan-3d-v3.1/text-to-3d-rapid"
    payload = {"prompt": model["prompt"]}

    if not quiet:
        print(f"  [{model_id}] WaveSpeedAI Hunyuan3D v3.1 Rapid — text-to-3D...")
    log_event({"action": "wavespeed_submit", "model_id": model_id})

    resp = session.post(endpoint, json=payload)
    resp.raise_for_status()
    data = resp.json().get("data", {})
    task_id = data.get("id")
    result_url = data.get("urls", {}).get("get")

    if not task_id or not result_url:
        print(f"  [{model_id}] WaveSpeedAI: no task ID in response", file=sys.stderr)
        return None

    if not quiet:
        print(f"  [{model_id}] Task submitted: {task_id}")

    # Poll for completion
    for _ in range(60):  # 5 min max
        time.sleep(POLL_INTERVAL)
        poll_resp = session.get(result_url)
        poll_resp.raise_for_status()
        poll_data = poll_resp.json().get("data", {})
        status = poll_data.get("status", "unknown")

        if status == "completed":
            outputs = poll_data.get("outputs", [])
            if not outputs:
                print(f"  [{model_id}] WaveSpeedAI completed but no outputs", file=sys.stderr)
                return None
            glb_url = outputs[0]
            exec_ms = poll_data.get("executionTime", 0)
            if not quiet:
                print(f"  [{model_id}] Complete ({exec_ms}ms)")
            return download_glb(session, glb_url, model_id, "-wavespeed")

        if status == "failed":
            error = poll_data.get("error", "unknown")
            print(f"  [{model_id}] WaveSpeedAI FAILED: {error}", file=sys.stderr)
            log_event({"action": "wavespeed_failed", "model_id": model_id, "error": error})
            return None

    print(f"  [{model_id}] WaveSpeedAI timed out after 5 min", file=sys.stderr)
    return None


# ── Shared utilities ──────────────────────────────────────────────────────────

def download_glb(session: requests.Session, url: str, model_id: str, suffix: str) -> Path | None:
    STAGING_DIR.mkdir(parents=True, exist_ok=True)
    output_path = STAGING_DIR / f"{model_id}{suffix}.glb"

    resp = session.get(url)
    resp.raise_for_status()

    content = resp.content
    if len(content) < 4 or content[:4] != b"glTF":
        print(f"  [{model_id}] Downloaded file is not a valid GLB", file=sys.stderr)
        return None

    output_path.write_bytes(content)
    size_kb = len(content) / 1024
    print(f"  [{model_id}] Downloaded: {output_path.name} ({size_kb:.0f} KB)")
    log_event({"action": "downloaded", "model_id": model_id, "path": str(output_path), "size_kb": round(size_kb)})
    return output_path


def main() -> None:
    parser = argparse.ArgumentParser(description="Generate 3D models for Open Rebellion tactical combat")
    parser.add_argument("--models", nargs="+", help="Specific model IDs to generate")
    parser.add_argument("--list", action="store_true", help="List all model IDs")
    parser.add_argument("--status", action="store_true", help="Show staging directory contents")
    parser.add_argument("--provider", choices=["hunyuan", "meshy", "wavespeed", "both"], default="both",
                        help="Which provider to use (default: both, per model config). wavespeed=$0.0225/model.")
    parser.add_argument("--skip-refine", action="store_true", help="Meshy preview only")
    parser.add_argument("--quiet", action="store_true", help="Minimal output")
    args = parser.parse_args()

    if args.list:
        for mid, m in MODELS.items():
            print(f"  {mid:25s} [{m['provider']:7s}] {m['category']:15s} {m['name']}")
        print(f"\nTotal: {len(MODELS)} models")
        return

    if args.status:
        if not STAGING_DIR.exists():
            print("No staging directory yet.")
            return
        for f in sorted(STAGING_DIR.glob("*.glb")):
            size_kb = f.stat().st_size / 1024
            print(f"  {f.name:40s} {size_kb:>8.0f} KB")
        return

    # Determine which models to generate
    model_ids = args.models if args.models else ALL_MODEL_IDS

    # Validate
    for mid in model_ids:
        if mid not in MODELS:
            print(f"Unknown model ID: {mid}", file=sys.stderr)
            print(f"Available: {', '.join(ALL_MODEL_IDS)}", file=sys.stderr)
            sys.exit(1)

    # Validate API keys early — fail fast before the loop
    needs_hunyuan = args.provider in ("hunyuan", "both")
    needs_meshy = args.provider in ("meshy", "both")
    needs_wavespeed = args.provider == "wavespeed"

    if needs_hunyuan:
        get_fal_key()  # validates FAL_KEY is set

    meshy_session = None
    if needs_meshy:
        key = get_meshy_key()
        meshy_session = requests.Session()
        meshy_session.headers["Authorization"] = f"Bearer {key}"

    wavespeed_session = None
    if needs_wavespeed:
        key = get_wavespeed_key()
        wavespeed_session = requests.Session()
        wavespeed_session.headers["Authorization"] = f"Bearer {key}"
        wavespeed_session.headers["Content-Type"] = "application/json"

    print(f"Generating {len(model_ids)} model(s)...\n")

    results = {"success": 0, "failed": 0, "skipped": 0}

    for mid in model_ids:
        model = MODELS[mid]
        provider = model["provider"] if args.provider == "both" else args.provider

        # Skip if already exists in staging
        existing = list(STAGING_DIR.glob(f"{mid}-*.glb")) if STAGING_DIR.exists() else []
        if existing:
            if not args.quiet:
                print(f"  [{mid}] Already in staging: {existing[0].name}")
            results["skipped"] += 1
            continue

        print(f"[{mid}] {model['name']} via {provider}")

        output = None
        if provider == "wavespeed" and wavespeed_session:
            output = wavespeed_generate(wavespeed_session, mid, model, args.quiet)
        elif provider == "hunyuan":
            output = hunyuan_generate(mid, model, args.quiet)
        elif provider == "meshy" and meshy_session:
            output = meshy_generate(meshy_session, mid, model, args.skip_refine, args.quiet)
        else:
            print(f"  [{mid}] Provider {provider} not available", file=sys.stderr)
            results["failed"] += 1
            continue

        if output:
            results["success"] += 1
        else:
            results["failed"] += 1

        print()

    print(f"\nResults: {results['success']} success, {results['failed']} failed, {results['skipped']} skipped")
    if STAGING_DIR.exists():
        total = len(list(STAGING_DIR.glob("*.glb")))
        print(f"Staging: {total} GLB(s) in {STAGING_DIR}")
        print(f"\nNext: bash scripts/prepare-rebellion-models.sh")


if __name__ == "__main__":
    main()
