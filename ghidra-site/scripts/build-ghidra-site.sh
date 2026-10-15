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

# Landing page from ghidra/README.md, with the Tanit logo after the frontmatter
python3 - "$PROJECT_DIR/ghidra/README.md" "$SITE_DIR/docs/index.md" <<'EOF'
import sys
src, dst = sys.argv[1:3]
text = open(src).read()
logo = """<div class="tanit-logo" style="text-align:center; margin-bottom:1em;">
  <img class="dark-only" src="/open-rebellion/assets/tanit-dark.png" alt="Symbol of Tanit" style="height:80px; width:auto;">
  <img class="light-only" src="/open-rebellion/assets/tanit-light.png" alt="Symbol of Tanit" width="80">
</div>
"""
end = text.index("\n---\n", 4) + 5
open(dst, "w").write(text[:end] + "\n" + logo + text[end:])
EOF
echo "  index.md (from ghidra/README.md)"

# Relative links in the copies point next to their source note. A target the
# site carries is relinked to its page; any other repository file becomes a
# GitHub link, so the site never links to a page it does not build.
python3 - "$PROJECT_DIR" "$SITE_DIR" <<'EOF'
import os, re, sys
project, site = sys.argv[1:3]
docs = os.path.join(site, "docs")
blob = "https://github.com/tdimino/open-rebellion/blob/main/"
notes = os.path.join(project, "ghidra", "notes")
sources = {"index.md": os.path.join(project, "ghidra")}
for sub in ("scholar", "combat"):
    for name in os.listdir(os.path.join(docs, sub)):
        sources[f"{sub}/{name}"] = notes
sources["indexes/notes-index.md"] = notes
sources["indexes/function-index.md"] = notes
on_site = {}
for page, src in sources.items():
    if src == notes:
        on_site[os.path.normpath(os.path.join(notes, os.path.basename(page)))] = page
link = re.compile(r"\]\(([^)\s]+)\)")
for page, src in sources.items():
    path = os.path.join(docs, page)
    if not os.path.exists(path):
        continue
    def fix(m):
        target = m.group(1)
        if re.match(r"[a-z]+:|#|/", target):
            return m.group(0)
        file, _, anchor = target.partition("#")
        anchor = "#" + anchor if anchor else ""
        if os.path.exists(os.path.join(os.path.dirname(path), file)):
            return m.group(0)
        resolved = os.path.normpath(os.path.join(src, file))
        if resolved in on_site:
            rel = os.path.relpath(on_site[resolved], os.path.dirname(page) or ".")
            return f"]({rel}{anchor})"
        if os.path.exists(resolved):
            return f"]({blob}{os.path.relpath(resolved, project)}{anchor})"
        return m.group(0)
    text = open(path).read()
    open(path, "w").write(link.sub(fix, text))
EOF
echo "  relative links rewritten"

echo ""
echo "Done. Preview: cd ghidra-site && mkdocs serve"
echo "Build:   cd ghidra-site && mkdocs build"
