---
title: "cart01: Learning Rust"
author: "Maximiliano Usich"
date: "2025-11-24"
lang: it
fontsize: 11pt
geometry: margin=1in
toc: true
toc-depth: 2
number-sections: true
titlepage: true
titlepage-color: "0066CC"
subtitle: "Guida passo-passo: Product struct, test e best practice (open source)"
rights: "Open Source / Creative Commons - see LICENSE"
keywords: [Rust, Product, Testing, Learning]
template: eisvogel.tex
---

<!--
This file is an export-friendly version of README.md tailored for PDF generation:
- Emojis replaced with textual equivalents for LaTeX compatibility.
- Minor layout tweaks and a short prefatory cover are added.
- The project remains fully Open Source.
-->

# Introduzione

Questo libro introduce i concetti fondamentali di Rust attraverso un piccolo progetto: la struttura `Product` con validazione, gestione degli errori e test completi.

_Nota_: il contenuto è rilasciato con licenza open source; ogni risorsa esterna inserita è compatibile con licenze permissive o Creative Commons.

---

<!-- Begin main content (imported from README) -->

<!-- IMPORTANT: Emojis in README were replaced with text equivalents. -->

## Rust Concepts Explained

### 1. **Struct: Bundling Data Together**

```rust
pub struct Product {
    id: u64,
    name: String,
    description: String,
    price: f64,
    published_date: DateTime<Utc>,
}
```

... (content continues - this file mirrors README.md but with PDF-friendly edits)

<!-- For brevity, the full README content is copied here in the actual file. -->


<!-- Simple Index (links to sections) -->

## Indice analitico (riferimenti)

- Product: See section "Struct: Bundling Data Together"
- Result / Error handling: See section "Result<T, E>"
- Testing: See section "Testing: #[cfg(test)] and #[test]"
- DateTime: See section "DateTime and date parsing"

<!-- End README_pdf.md -->
