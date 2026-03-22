#!/bin/bash
# Sync scholar docs from gitignored ghidra/notes/ into the mkdocs docs/ tree.
# Run this before committing to ghidra-site/ or before `mkdocs serve`.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
SITE_DIR="$(dirname "$SCRIPT_DIR")"
PROJECT_DIR="$(dirname "$SITE_DIR")"
NOTES_DIR="$PROJECT_DIR/ghidra/notes"

if [ ! -d "$NOTES_DIR" ]; then
    echo "ERROR: ghidra/notes/ not found at $NOTES_DIR"
    echo "This script must be run from a machine with the local Ghidra project."
    exit 1
fi

mkdir -p "$SITE_DIR/docs/scholar" "$SITE_DIR/docs/combat" "$SITE_DIR/docs/indexes"

# Scholar documents (original analysis — safe to publish)
for f in annotated-functions modders-taxonomy rust-implementation-guide \
         cpp-class-hierarchy entity-system mission-event-cookbook \
         economy-systems ai-behavior-analysis; do
    if [ -f "$NOTES_DIR/$f.md" ]; then
        cp "$NOTES_DIR/$f.md" "$SITE_DIR/docs/scholar/"
        echo "  scholar/$f.md"
    fi
done

# Combat subsystem docs
for f in space-combat ground-combat bombardment combat-formulas COMBAT-SUMMARY; do
    if [ -f "$NOTES_DIR/$f.md" ]; then
        cp "$NOTES_DIR/$f.md" "$SITE_DIR/docs/combat/"
        echo "  combat/$f.md"
    fi
done

# Indexes
if [ -f "$NOTES_DIR/FUNCTION_INDEX.md" ]; then
    cp "$NOTES_DIR/FUNCTION_INDEX.md" "$SITE_DIR/docs/indexes/function-index.md"
    echo "  indexes/function-index.md"
fi
if [ -f "$NOTES_DIR/INDEX.md" ]; then
    cp "$NOTES_DIR/INDEX.md" "$SITE_DIR/docs/indexes/notes-index.md"
    echo "  indexes/notes-index.md"
fi

# Landing page from ghidra/README.md
cp "$PROJECT_DIR/ghidra/README.md" "$SITE_DIR/docs/index.md"
echo "  index.md (from ghidra/README.md)"

echo ""
echo "Done. Preview: cd ghidra-site && mkdocs serve"
echo "Build:   cd ghidra-site && mkdocs build"
