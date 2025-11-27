/// # cart01: Learning Rust - Product Struct Fundamentals
///
/// This module teaches core Rust concepts through a `Product` struct with:
/// - Field types: struct, String, f64, DateTime
/// - Methods: impl block, getters, validation
/// - Error handling: Result<T, E> and Option<T>
/// - Testing: #[cfg(test)] and #[test]
/// - Traits: Display, Debug, Clone, Copy
///
/// **Learning Path:**
/// 1. Struct definition (what is ownership?)
/// 2. Methods and impl (what is self?)
/// 3. Error handling (Result vs panic)
/// 4. Tests (how to validate behavior?)

use chrono::{DateTime, Utc, NaiveDate, TimeZone};
use serde::{Deserialize, Serialize};
use std::fmt;

/// A `Product` represents an item in our e-commerce cart.
///
/// **Fields explanation:**
/// - `id`: Unique identifier (u64 = unsigned 64-bit integer, immutable by default)
/// - `name`: Product name (String = owned text, heap-allocated, UTF-8)
/// - `description`: Detailed product info (String, can be empty)
/// - `price`: Cost in currency units (f64 = 64-bit floating point)
/// - `published_date`: When the product was added (DateTime<Utc> = point-in-time, timezone-aware)
///
/// **Rust concept:** `#[derive(...)]` auto-implements traits (Clone, Debug, Serialize)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Product {
    /// Unique product ID - immutable after creation
    id: u64,
    /// Product name - mutable (we might rename it)
    name: String,
    /// Detailed description - mutable
    description: String,
    /// Price in base currency units (e.g., cents, satoshis) - mutable
    price: f64,
    /// When this product was published - immutable (audit trail)
    published_date: DateTime<Utc>,
}

/// Error type for Product validation failures
/// **Rust concept:** `enum` allows multiple variants (like switch/case but type-safe)
#[derive(Clone, Debug, PartialEq)]
pub enum ProductError {
    /// Product name cannot be empty
    EmptyName,
    /// Price must be >= 0.0
    InvalidPrice(f64),
    /// ID cannot be zero (reserved for "no product")
    ZeroId,
    /// Invalid date string format
    InvalidDate(String),
}

/// Implement Display trait so errors print nicely
/// **Rust concept:** Traits = interfaces/contracts. Display = how to convert to String
impl fmt::Display for ProductError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProductError::EmptyName => write!(f, "Product name cannot be empty"),
            ProductError::InvalidPrice(p) => write!(f, "Price must be >= 0, got {}", p),
            ProductError::ZeroId => write!(f, "Product ID cannot be zero"),
            ProductError::InvalidDate(s) => write!(f, "Invalid date format: {}", s),
        }
    }
}

/// Implement std::error::Error trait so our error type integrates with Rust ecosystem
impl std::error::Error for ProductError {}

impl Product {
    /// Create a new Product with validation
    ///
    /// **Rust concept:** `Result<T, E>` = Either success (T) or error (E)
    /// Returns `Ok(Product)` if all fields valid, `Err(ProductError)` otherwise
    ///
    /// # Arguments
    /// * `id` - Unique product identifier (must be > 0)
    /// * `name` - Product name (must not be empty)
    /// * `description` - Product details (can be empty string)
    /// * `price` - Price in currency units (must be >= 0.0)
    /// * `published_date_str` - ISO 8601 format: "2025-11-24" or with time
    ///
    /// # Examples
    /// ```ignore
    /// let product = Product::new(1, "Laptop", "High-end gaming laptop", 1299.99, "2025-11-24")?;
    /// // ^ Note: ? operator = if Err, return it immediately (called "unwrapping")
    /// ```
    pub fn new(
        id: u64,
        name: &str,
        description: &str,
        price: f64,
        published_date_str: &str,
    ) -> Result<Self, ProductError> {
        // Validation step 1: ID cannot be zero
        if id == 0 {
            return Err(ProductError::ZeroId);
        }

        // Validation step 2: Name must not be empty
        if name.trim().is_empty() {
            return Err(ProductError::EmptyName);
        }

        // Validation step 3: Price must be non-negative
        if price < 0.0 {
            return Err(ProductError::InvalidPrice(price));
        }

        // Validation step 4: Parse the date string into DateTime<Utc>
        // **Rust concept:** Pattern matching on Option/Result using match or ?
        let published_date = NaiveDate::parse_from_str(published_date_str, "%Y-%m-%d")
            .ok()
            .and_then(|nd| nd.and_hms_opt(0, 0, 0))
            .map(|ndt| Utc.from_utc_datetime(&ndt))
            .ok_or_else(|| ProductError::InvalidDate(published_date_str.to_string()))?;

        // All validations passed - construct the Product
        // **Rust concept:** Ownership = "self" owns these Strings after construction
        Ok(Product {
            id,
            name: name.to_string(),     // Create owned copy from &str slice
            description: description.to_string(),
            price,
            published_date,
        })
    }

    /// Get the product ID (immutable reference, doesn't consume self)
    /// **Rust concept:** `&self` = borrow (read-only, caller retains ownership)
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get the product name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the product description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get the product price
    pub fn price(&self) -> f64 {
        self.price
    }

    /// Get the published date
    pub fn published_date(&self) -> DateTime<Utc> {
        self.published_date
    }

    /// Update product name (mutable operation)
    /// **Rust concept:** `&mut self` = borrow mutably (read-write)
    pub fn set_name(&mut self, new_name: &str) -> Result<(), ProductError> {
        if new_name.trim().is_empty() {
            return Err(ProductError::EmptyName);
        }
        self.name = new_name.to_string();
        Ok(())
    }

    /// Update product price (with validation)
    pub fn set_price(&mut self, new_price: f64) -> Result<(), ProductError> {
        if new_price < 0.0 {
            return Err(ProductError::InvalidPrice(new_price));
        }
        self.price = new_price;
        Ok(())
    }

    /// Update product description (no validation - can be empty)
    pub fn set_description(&mut self, new_description: &str) {
        self.description = new_description.to_string();
    }

    /// Check if product is reasonably priced (for learning: business logic)
    pub fn is_expensive(&self, threshold: f64) -> bool {
        self.price > threshold
    }

    /// Calculate discount price (immutable - doesn't change self)
    pub fn apply_discount(&self, discount_percent: f64) -> Result<f64, String> {
        if discount_percent < 0.0 || discount_percent > 100.0 {
            return Err("Discount must be 0-100%".to_string());
        }
        let discounted = self.price * (1.0 - discount_percent / 100.0);
        Ok(discounted)
    }
}

/// Implement Display trait for nice string formatting
impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Product #{}: {} (${:.2}) - {} [published: {}]",
            self.id,
            self.name,
            self.price,
            self.description,
            self.published_date.format("%Y-%m-%d")
        )
    }
}

// ============================================================================
// TESTS: Comprehensive test coverage (aim for 100%)
// **Rust concept:** #[cfg(test)] = compile this module only in test mode
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    // ========== SUCCESS CASES ==========

    #[test]
    fn test_product_creation_success() {
        let product = Product::new(1, "Laptop", "Gaming laptop", 1299.99, "2025-11-24")
            .expect("Should create valid product");

        assert_eq!(product.id(), 1);
        assert_eq!(product.name(), "Laptop");
        assert_eq!(product.description(), "Gaming laptop");
        assert_eq!(product.price(), 1299.99);
        assert_eq!(product.published_date().year(), 2025);
    }

    #[test]
    fn test_product_creation_with_empty_description() {
        let product = Product::new(2, "Mouse", "", 29.99, "2025-11-20")
            .expect("Should allow empty description");

        assert_eq!(product.description(), "");
    }

    #[test]
    fn test_product_creation_zero_price() {
        // Free product should be valid
        let product = Product::new(3, "Free Sample", "No cost", 0.0, "2025-11-15")
            .expect("Should allow zero price");

        assert_eq!(product.price(), 0.0);
    }

    #[test]
    fn test_product_display() {
        let product = Product::new(1, "Keyboard", "Mechanical keyboard", 149.99, "2025-11-24")
            .expect("Should create product");

        let display_str = format!("{}", product);
        assert!(display_str.contains("Keyboard"));
        assert!(display_str.contains("149.99"));
        assert!(display_str.contains("2025-11-24"));
    }

    // ========== ERROR CASES ==========

    #[test]
    fn test_product_creation_zero_id() {
        let result = Product::new(0, "Invalid", "Has zero ID", 99.99, "2025-11-24");

        assert_eq!(result, Err(ProductError::ZeroId));
    }

    #[test]
    fn test_product_creation_empty_name() {
        let result = Product::new(1, "", "Empty name", 99.99, "2025-11-24");

        assert_eq!(result, Err(ProductError::EmptyName));
    }

    #[test]
    fn test_product_creation_whitespace_name() {
        let result = Product::new(1, "   ", "Only spaces", 99.99, "2025-11-24");

        assert_eq!(result, Err(ProductError::EmptyName));
    }

    #[test]
    fn test_product_creation_negative_price() {
        let result = Product::new(1, "Negative", "Bad price", -50.0, "2025-11-24");

        assert!(matches!(result, Err(ProductError::InvalidPrice(-50.0))));
    }

    #[test]
    fn test_product_creation_invalid_date() {
        let result = Product::new(1, "Product", "Bad date", 99.99, "invalid-date");

        assert!(matches!(result, Err(ProductError::InvalidDate(_))));
    }

    #[test]
    fn test_product_creation_malformed_date() {
        let result = Product::new(1, "Product", "Wrong format", 99.99, "24-11-2025");

        assert!(matches!(result, Err(ProductError::InvalidDate(_))));
    }

    // ========== MUTATION TESTS ==========

    #[test]
    fn test_set_name_valid() {
        let mut product = Product::new(1, "Old Name", "Desc", 99.99, "2025-11-24")
            .expect("Should create product");

        product
            .set_name("New Name")
            .expect("Should update name");

        assert_eq!(product.name(), "New Name");
    }

    #[test]
    fn test_set_name_empty() {
        let mut product = Product::new(1, "Name", "Desc", 99.99, "2025-11-24")
            .expect("Should create product");

        let result = product.set_name("");

        assert_eq!(result, Err(ProductError::EmptyName));
        assert_eq!(product.name(), "Name"); // Unchanged
    }

    #[test]
    fn test_set_price_valid() {
        let mut product = Product::new(1, "Product", "Desc", 99.99, "2025-11-24")
            .expect("Should create product");

        product.set_price(199.99).expect("Should update price");

        assert_eq!(product.price(), 199.99);
    }

    #[test]
    fn test_set_price_negative() {
        let mut product = Product::new(1, "Product", "Desc", 99.99, "2025-11-24")
            .expect("Should create product");

        let result = product.set_price(-10.0);

        assert!(matches!(result, Err(ProductError::InvalidPrice(-10.0))));
        assert_eq!(product.price(), 99.99); // Unchanged
    }

    #[test]
    fn test_set_description() {
        let mut product = Product::new(1, "Product", "Old desc", 99.99, "2025-11-24")
            .expect("Should create product");

        product.set_description("New description");

        assert_eq!(product.description(), "New description");
    }

    // ========== BUSINESS LOGIC TESTS ==========

    #[test]
    fn test_is_expensive_true() {
        let product = Product::new(1, "Expensive", "High cost", 1000.0, "2025-11-24")
            .expect("Should create product");

        assert!(product.is_expensive(500.0));
    }

    #[test]
    fn test_is_expensive_false() {
        let product = Product::new(1, "Cheap", "Low cost", 50.0, "2025-11-24")
            .expect("Should create product");

        assert!(!product.is_expensive(100.0));
    }

    #[test]
    fn test_apply_discount_valid() {
        let product = Product::new(1, "Product", "Desc", 100.0, "2025-11-24")
            .expect("Should create product");

        let discounted = product
            .apply_discount(10.0)
            .expect("Should calculate discount");

        assert_eq!(discounted, 90.0);
    }

    #[test]
    fn test_apply_discount_zero() {
        let product = Product::new(1, "Product", "Desc", 100.0, "2025-11-24")
            .expect("Should create product");

        let discounted = product
            .apply_discount(0.0)
            .expect("Should allow 0% discount");

        assert_eq!(discounted, 100.0);
    }

    #[test]
    fn test_apply_discount_hundred_percent() {
        let product = Product::new(1, "Product", "Desc", 100.0, "2025-11-24")
            .expect("Should create product");

        let discounted = product
            .apply_discount(100.0)
            .expect("Should allow 100% discount");

        assert_eq!(discounted, 0.0);
    }

    #[test]
    fn test_apply_discount_invalid_negative() {
        let product = Product::new(1, "Product", "Desc", 100.0, "2025-11-24")
            .expect("Should create product");

        let result = product.apply_discount(-10.0);

        assert!(result.is_err());
    }

    #[test]
    fn test_apply_discount_invalid_over_100() {
        let product = Product::new(1, "Product", "Desc", 100.0, "2025-11-24")
            .expect("Should create product");

        let result = product.apply_discount(150.0);

        assert!(result.is_err());
    }

    // ========== INTEGRATION TESTS ==========

    #[test]
    fn test_product_clone() {
        let product1 = Product::new(1, "Original", "Desc", 99.99, "2025-11-24")
            .expect("Should create product");

        let product2 = product1.clone();

        assert_eq!(product1.name(), product2.name());
        assert_eq!(product1.price(), product2.price());
    }

    #[test]
    fn test_product_debug_format() {
        let product = Product::new(1, "Debug Test", "Desc", 99.99, "2025-11-24")
            .expect("Should create product");

        let debug_str = format!("{:?}", product);
        assert!(debug_str.contains("Debug Test"));
    }

    #[test]
    fn test_error_display() {
        let error = ProductError::EmptyName;
        let error_str = format!("{}", error);
        assert_eq!(error_str, "Product name cannot be empty");
    }

    #[test]
    fn test_product_serialization() {
        let product = Product::new(1, "Serialize Test", "Desc", 99.99, "2025-11-24")
            .expect("Should create product");

        let json = serde_json::to_string(&product).expect("Should serialize");
        let _deserialized: Product = serde_json::from_str(&json).expect("Should deserialize");

        // If we got here, both operations succeeded
    }
}
