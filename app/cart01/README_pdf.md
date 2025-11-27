````markdown
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

**Cheatsheet**

- `docker run --rm -v "$(pwd)":/data pandoc/extra pandoc README.md -o README.pdf --from=markdown+emoji --template=eisvogel --pdf-engine=xelatex`
- `docker run --rm -v "$(pwd)":/data -w /data ghcr.io/pandoc/latex-universe:latest pandoc README.md -o README.pdf --from=markdown+emoji --template=./eisvogel.tex --pdf-engine=xelatex`
- `docker pull ghcr.io/pandoc/latex-universe:latest`
- `curl -L -o eisvogel.tex https://raw.githubusercontent.com/Wandmalfarbe/pandoc-latex-template/master/eisvogel.tex`
- `source /home/dev01/.nvm/nvm.sh && nvm use --silent 24.11.1`
- `which node && node -v`
- `node scripts/process_readme.js`
- `bash scripts/build_book.sh`
- `rsvg-convert -o assets/cover.png assets/cover.svg`
- `convert assets/cover.svg assets/cover.png`
- `base64 README.md > README_base64.txt`
- `printf '```' > README_base64.md; cat README_base64.txt >> README_base64.md; printf '```' >> README_base64.md`
- `cargo test`
- `cargo build`
- `git add . && git commit -m "update: add cheatsheet"`

<!-- End README_pdf.md -->

**Rust Cheatsheet**

- `cargo new cart01 --bin`
- `cargo new cart01 --lib`
- `cargo init --bin`
- `cargo add chrono --features serde`
- `cargo add serde --features derive`
- `cargo add serde_json`
- `cargo build`
- `cargo build --release`
- `cargo test`
- `cargo test -- --nocapture`
- `cargo run`
- `cargo run --release`
- `cargo fmt`
- `cargo clippy`
- `cargo doc --no-deps --open`
- `base64 README.md > README_base64.txt`

**Makefile Targets (commands only)**

- `make build`
- `make check`
- `make test`
- `make test-verbose`
- `make test-coverage`
- `make run`
- `make run-debug`
- `make fmt`
- `make lint`
- `make docs`
- `make clean`
- `make all`

## Rust Topics Cheatsheet (term → first usage line)

- `struct` → `pub struct Product {`  (src/lib.rs)
- `enum` → `pub enum ProductError {`  (src/lib.rs)
- `impl` → `impl Product {`  (src/lib.rs)
- `fn` → `fn main() {`  (src/main.rs)
- `pub` → `pub fn new(`  (src/lib.rs)
- `use` → `use chrono::{DateTime, Utc, NaiveDate, TimeZone};`  (src/lib.rs)
- `crate` → `use cart01::Product;`  (src/main.rs)
- `Result` → `) -> Result<Self, ProductError> {`  (src/lib.rs)
- `Option` → (mentioned in comment)  (src/lib.rs)
- `Ok` → `Ok(Product {`  (src/lib.rs)
- `Err` → `return Err(ProductError::ZeroId);`  (src/lib.rs)
- `?` → (mentioned in comment)  (src/lib.rs)
- `match` → `match self {`  (src/lib.rs)
- `pattern matching` → (mentioned in comment)  (src/lib.rs)
- `&self` → `pub fn id(&self) -> u64 {`  (src/lib.rs)
- `&mut self` → `pub fn set_name(&mut self, new_name: &str) -> Result<(), ProductError> {`  (src/lib.rs)
- `ownership` → (mentioned in comment)  (src/lib.rs)
- `borrow` → (mentioned in comment)  (src/lib.rs)
- `String` → `name: String,`  (src/lib.rs)
- `&str` → (function arg)  (src/lib.rs)
- `u64` → `id: u64,`  (src/lib.rs)
- `f64` → `price: f64,`  (src/lib.rs)
- `DateTime<Utc>` → `published_date: DateTime<Utc>,`  (src/lib.rs)
- `NaiveDate` → `use chrono::{DateTime, Utc, NaiveDate, TimeZone};`  (src/lib.rs)
- `TimeZone` → `use chrono::{DateTime, Utc, NaiveDate, TimeZone};`  (src/lib.rs)
- `chrono` → `use chrono::{DateTime, Utc, NaiveDate, TimeZone};`  (src/lib.rs)
- `serde` → `use serde::{Deserialize, Serialize};`  (src/lib.rs)
- `Serialize` → `#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]`  (src/lib.rs)
- `Deserialize` → `#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]`  (src/lib.rs)
- `serde_json` → `let json = serde_json::to_string(&product).expect("Should serialize");`  (src/lib.rs tests)
- `#[derive]` → `#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]`  (src/lib.rs)
- `Clone` → `#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]`  (src/lib.rs)
- `Debug` → `#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]`  (src/lib.rs)
- `PartialEq` → `#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]`  (src/lib.rs)
- `Display` → `impl fmt::Display for Product {`  (src/lib.rs)
- `fmt::Display` → `impl fmt::Display for Product {`  (src/lib.rs)
- `std::error::Error` → `impl std::error::Error for ProductError {}`  (src/lib.rs)
- `write!` → `write!(f, "Product name cannot be empty"),`  (src/lib.rs)
- `println!` → `println!("🛒 cart01: Learning Rust - Product Example\n");`  (src/main.rs)
- `format!` → `let display_str = format!("{}", product);`  (src/lib.rs tests)
- `assert_eq!` → `assert_eq!(product.id(), 1);`  (src/lib.rs tests)
- `assert!` → `assert!(product.is_expensive(500.0));`  (src/lib.rs tests)
- `matches!` → `assert!(matches!(result, Err(ProductError::InvalidPrice(-50.0))));`  (src/lib.rs tests)
- `#[cfg(test)]` → `#[cfg(test)]`  (src/lib.rs)
- `#[test]` → `#[test]`  (src/lib.rs tests)
- `cargo build` → `cargo build --release`  (Makefile)
- `cargo test` → `cargo test --lib -- --test-threads=1`  (Makefile)
- `cargo run` → `cargo run --release`  (Makefile)
- `cargo fmt` → `cargo fmt`  (Makefile)
- `cargo clippy` → `cargo clippy -- -D warnings || true`  (Makefile)
- `to_string` → `name: name.to_string(),`  (src/lib.rs)
- `trim` → `if name.trim().is_empty() {`  (src/lib.rs)
- `is_empty` → `if name.trim().is_empty() {`  (src/lib.rs)
- `parse_from_str` → `NaiveDate::parse_from_str(published_date_str, "%Y-%m-%d")`  (src/lib.rs)
- `and_then` → `.ok().and_then(|nd| nd.and_hms_opt(0, 0, 0))`  (src/lib.rs)
- `ok_or_else` → `.ok_or_else(|| ProductError::InvalidDate(published_date_str.to_string()))?`  (src/lib.rs)
- `map` → `.map(|ndt| Utc.from_utc_datetime(&ndt))`  (src/lib.rs)

### Code-fenced block example

The following is a markdown code-fenced block showing a small snippet — this is what I mean by "code-fenced":

```rust
// Example: Product struct declaration
pub struct Product {
    id: u64,
    name: String,
    pric