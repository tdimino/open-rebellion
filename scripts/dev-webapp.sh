#!/usr/bin/env bash
# Dev server for the webapp with hot reload.
# WASM module must be built first via `scripts/build-webapp.sh` (or the JS mock
# will be used automatically as a fallback).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
APP_DIR="$ROOT/webapp"

cd "$APP_DIR"

if [ ! -d node_modules ]; then
    echo "==> Installing dependencies"
    npm install
fi

echo "==> Starting Vite dev server (http://localhost:5173)"
npm run dev
