# Rust Technical Terms Cheatsheet (First Occurrence)

| Term | First Usage | File |
|------|-------------|------|
| `struct` | `pub struct Product {` | src/lib.rs |
| `enum` | `pub enum ProductError {` | src/lib.rs |
| `impl` | `impl Product {` | src/lib.rs |
| `pub` | `pub struct Product` | src/lib.rs |
| `use` | `use chrono::{DateTime, Utc, ...}` | src/lib.rs |
| `Result` | `-> Result<Self, ProductError>` | src/lib.rs |
| `Option` | (mentioned in docs) | src/lib.rs |
| `Ok` | `Ok(Product { id, ... })` | src/lib.rs |
| `Err` | `Err(ProductError::ZeroId)` | src/lib.rs |
| `match` | `match self { ... }` | src/lib.rs |
| `&self` | `pub fn id(&self) -> u64` | src/lib.rs |
| `&mut self` | `pub fn set_name(&mut self, ...)` | src/lib.rs |
| `String` | `name: String,` | src/lib.rs |
| `&str` | `name: &str,` (parameter) | src/lib.rs |
| `u64` | `id: u64,` | src/lib.rs |
| `f64` | `price: f64,` | src/lib.rs |
| `DateTime<Utc>` | `published_date: DateTime<Utc>,` | src/lib.rs |
| `chrono` | `use chrono::{...}` | src/lib.rs |
| `serde` | `use serde::{...}` | src/lib.rs |
| `Serialize` | `#[derive(..., Serialize, ...)]` | src/lib.rs |
| `Deserialize` | `#[derive(..., Deserialize)]` | src/lib.rs |
| `#[derive]` | `#[derive(Clone, Debug, ...)]` | src/lib.rs |
| `Clone` | `#[derive(Clone, ...)]` | src/lib.rs |
| `Debug` | `#[derive(..., Debug, ...)]` | src/lib.rs |
| `PartialEq` | `#[derive(..., PartialEq, ...)]` | src/lib.rs |
| `Display` | `impl fmt::Display for Product` | src/lib.rs |
| `fmt::Formatter` | `fn fmt(&self, f: &mut fmt::Formatter)` | src/lib.rs |
| `write!` | `write!(f, "message")` | src/lib.rs |
| `println!` | `println!("message")` | src/main.rs |
| `format!` | `format!("{}", product)` | src/lib.rs (tests) |
| `assert_eq!` | `assert_eq!(product.id(), 1)` | src/lib.rs (tests) |
| `assert!` | `assert!(product.is_expensive(...))` | src/lib.rs (tests) |
| `#[cfg(test)]` | `#[cfg(test)] mod tests` | src/lib.rs |
| `#[test]` | `#[test] fn test_name()` | src/lib.rs |
| `to_string()` | `name.to_string()` | src/lib.rs |
| `trim()` | `name.trim().is_empty()` | src/lib.rs |
| `is_empty()` | `name.trim().is_empty()` | src/lib.rs |
| `parse_from_str()` | `NaiveDate::parse_from_str(...)` | src/lib.rs |
| `and_then()` | `.ok().and_then(\|nd\| ...)` | src/lib.rs |
| `ok_or_else()` | `.ok_or_else(\|\| ...)` | src/lib.rs |
| `map()` | `.map(\|ndt\| ...)` | src/lib.rs |
| `fn` | `fn main() {` | src/main.rs |
| `crate` | `use cart01::Product` | src/main.rs |
| `?` | (unwrap operator) | src/lib.rs (docs) |
| `ownership` | (concept) | src/lib.rs (docs) |
| `borrow` | (concept: `&self`) | src/lib.rs |

