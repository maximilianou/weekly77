#!/usr/bin/env node
const fs = require('fs');
const path = require('path');
const child = require('child_process');

const infile = process.argv[2] || 'README_pdf.md';
const outmd = 'README_build.md';
const assetsDir = path.join('assets','diagrams');
if (!fs.existsSync(assetsDir)) fs.mkdirSync(assetsDir, { recursive: true });

let text = fs.readFileSync(infile, 'utf8');

// Emoji/text replacements
const map = {
  '🛒':'[CART]', '📚':'[BOOK]', '🎓':'[LEARN]', '📁':'[FOLDER]', '🚀':'[START]',
  '🔍':'[SEARCH]', '✅':'[YES]', '❌':'[NO]', '✨':'[STAR]', '🔧':'[TOOL]',
  '💡':'[IDEA]', '📈':'[GRAPH]', '📝':'[DOC]', '🆘':'[HELP]', '🎉':'[SUCCESS]',
  '🧪':'[TEST]', '📊':'[DATA]', '🎨':'[ART]', '🔎':'[FIND]', '📦':'[PKG]',
  '🎯':'[TARGET]', '🔗':'[LINK]', '✔':'[OK]', '✓':'[OK]'
};
for(const k of Object.keys(map)) text = text.split(k).join(map[k]);
// remove variation selectors
text = text.replace(/\uFE0F/g,'').replace(/\u200D/g,'');

// Find mermaid blocks and replace with image links
const mermaidRegex = /```mermaid\s*([\s\S]*?)```/g;
let m;
let idx = 1;
const mmdFiles = [];
while((m = mermaidRegex.exec(text)) !== null){
  const code = m[1].trim();
  const name = `diagram_${String(idx).padStart(2,'0')}`;
  const mmdPath = path.join(assetsDir, `${name}.mmd`);
  fs.writeFileSync(mmdPath, code, 'utf8');
  mmdFiles.push(mmdPath);
  const svgPath = path.join(assetsDir, `${name}.svg`).replace(/\\/g,'/');
  const imgMarkdown = `![Diagram ${idx}](${svgPath}){ width=80% }`;
  // replace the mermaid block with the image markdown
  text = text.slice(0, m.index) + imgMarkdown + text.slice(m.index + m[0].length);
  // reset regex lastIndex to continue after replacement
  mermaidRegex.lastIndex = m.index + imgMarkdown.length;
  idx++;
}

fs.writeFileSync(outmd, text, 'utf8');
console.log('WROTE', outmd);
if(mmdFiles.length>0) console.log('MERMAID_FILES', mmdFiles.join(', '));
process.exit(0);
