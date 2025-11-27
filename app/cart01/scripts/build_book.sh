#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT_DIR"

INFILE="README_pdf.md"
BUILDMD="README_build.md"
ASSETSDIR="assets/diagrams"
mkdir -p "$ASSETSDIR"

# If nvm exists, source it so `node`/`npx` are available in non-interactive shells
if [ -s "$HOME/.nvm/nvm.sh" ]; then
  # shellcheck source=/dev/null
  . "$HOME/.nvm/nvm.sh"
fi

echo "1) Processing README (emoji cleanup + mermaid extraction)"
node scripts/process_readme.js "$INFILE"

echo "2) Rendering mermaid diagrams (if any)"
shopt -s nullglob
for m in $ASSETSDIR/*.mmd; do
  svg="${m%.mmd}.svg"
  echo "  - Rendering $m -> $svg"
  npx @mermaid-js/mermaid-cli -i "$m" -o "$svg" || npx mmdc -i "$m" -o "$svg"
done

echo "3) Convert to PDF with pandoc"
OUTPDF="cart01_book_final.pdf"
# Use cover if exists
if [ -f assets/cover.png ]; then
  pandoc README_build.md -o "$OUTPDF" --pdf-engine=xelatex -V lang=it -V fontsize=11pt --toc --toc-depth=2 --number-sections --metadata title="cart01: Learning Rust" --metadata author="Maximiliano Usich" --metadata date="$(date +%F)" --include-in-header=header.tex --include-before-body=cover.tex
else
  pandoc README_build.md -o "$OUTPDF" --pdf-engine=xelatex -V lang=it -V fontsize=11pt --toc --toc-depth=2 --number-sections --metadata title="cart01: Learning Rust" --metadata author="Maximiliano Usich" --metadata date="$(date +%F)" --include-in-header=header.tex
fi

echo "Done: $OUTPDF"
ls -lh "$OUTPDF"
