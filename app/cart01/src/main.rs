use cart01::Product;

fn main() {
    println!("🛒 cart01: Learning Rust - Product Example\n");

    // Example 1: Create a valid product
    match Product::new(1, "Laptop", "High-end gaming laptop", 1299.99, "2025-11-24") {
        Ok(product) => {
            println!("✅ Created: {}\n", product);
            println!("   ID: {}", product.id());
            println!("   Name: {}", product.name());
            println!("   Price: ${:.2}", product.price());
            println!("   Expensive (> $1000)? {}\n", product.is_expensive(1000.0));

            // Apply discount
            match product.apply_discount(15.0) {
                Ok(discounted) => {
                    println!("   Price after 15% discount: ${:.2}\n", discounted);
                }
                Err(e) => println!("   Error: {}\n", e),
            }
        }
        Err(e) => println!("❌ Error: {}\n", e),
    }

    // Example 2: Try to create invalid products
    let invalid_cases = vec![
        (0, "Zero ID Product", "Should fail", 99.99, "2025-11-24"),
        (2, "", "Empty name", 99.99, "2025-11-24"),
        (3, "Negative Price", "Bad price", -50.0, "2025-11-24"),
        (4, "Bad Date", "Invalid format", 99.99, "invalid-date"),
    ];

    println!("Testing error cases:");
    for (id, name, desc, price, date) in invalid_cases {
        match Product::new(id, name, desc, price, date) {
            Ok(_) => println!("  ✅ Case (id={}) passed", id),
            Err(e) => println!("  ❌ Case (id={}): {}", id, e),
        }
    }

    println!("\n✨ Run 'make test' to see all 30+ test cases");
}
