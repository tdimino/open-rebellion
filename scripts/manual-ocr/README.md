# Manual OCR

OCR the Star Wars Rebellion (1998) game manual PDF using Mistral OCR 4.1.

## Usage

```bash
# from the repository root
export MISTRAL_API_KEY=...  # your key
uv run --with pymupdf --with 'mistralai>=2' python3 scripts/manual-ocr/ocr_manual.py
```

## Output

- `docs/reference/campaign-history/archive/star-wars-rebellion-manual.md` -- Markdown transcription
- `.artifacts/manual-ocr/star-wars-rebellion-manual.ocr.json` -- raw OCR diagnostics

## Figure annotations

The OCR script produces text only -- no image links, no bbox annotations.
Mistral OCR's `bbox_annotation_format` hallucinates descriptions and recycles
schema example values on scanned pages, so figure annotations are added in a
separate caption-driven step:

1. Find each `FIG. X.Y` caption line in the OCR output.
2. Read the corresponding PDF page to get the actual printed callout labels.
3. Insert one blockquote immediately after the caption line:
   `> **Fig. X.Y -- Title.** Callouts: label -- what it points to; ...`
4. If callouts are unreadable in the scan, write:
   `> **Fig. X.Y -- Title.** Callouts: unreadable in the scan.`
5. Decorative art and pages without a FIG caption get no annotation.

This step is manual (or agent-driven via PDF page reads) and is not automated
by `ocr_manual.py`.

## Post-processing

Flag pages with quality issues:

```bash
uv run --with pymupdf --with numpy python3 ~/.claude/skills/ancient-near-east-research/scripts/ocr_flag_pages.py \
  docs/reference/campaign-history/archive/star-wars-rebellion-manual.pdf \
  --diagnostics .artifacts/manual-ocr/star-wars-rebellion-manual.ocr.json
```
