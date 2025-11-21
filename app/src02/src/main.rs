mod catalog;
mod models;
mod usage;

use catalog::Catalog;
use models::*;
use usage::{resolve_payment_for_usage, UsageLog};

#[tokio::main]
async fn main() {
    // create data (immutable values) and run some queries
    let p1 = Product::new("p-1", "Email Support", 500);
    let p2 = Product::new("p-2", "Premium Analytics", 1500);
    let p3 = Product::new("p-3", "On-site Consulting", 10000);

    let svc = Service::new("s-1", "SaaS Platform", vec![p1.clone(), p2.clone()]);
    let svc2 = Service::new("s-2", "Consulting", vec![p3.clone()]);

    let mut catalog = Catalog::default();
    catalog = catalog.with_service(svc);
    catalog = catalog.with_service(svc2);

    let alice = User::new(
        "u-alice",
        "Alice",
        Some(PaymentMethod::card("**** **** **** 4242", "Alice")),
    );
    let bob = User::new("u-bob", "Bob", None);

    let usage1 = ServiceUsage::new(&alice.id, &"s-1".into(), &"p-2".into(), None);
    let usage2 = ServiceUsage::new(
        &bob.id,
        &"s-1".into(),
        &"p-1".into(),
        Some(PaymentMethod::paypal("bob@paypal")),
    );
    let usage3 = ServiceUsage::new(&alice.id, &"s-2".into(), &"p-3".into(), None);

    let log = UsageLog::from_vec(vec![usage1.clone(), usage2.clone(), usage3.clone()]);

    println!("All products:");
    for p in catalog.list_all_products() {
        println!("- {} ({}) {}¢", p.name, p.id.0, p.price_cents);
    }

    println!("\nProducts in service s-1:");
    if let Some(products) = catalog.list_products_for_service(&"s-1".into()) {
        for p in products {
            println!("* {} ({})", p.name, p.id.0);
        }
    }

    println!("\nUsages for Alice:");
    for u in log.service_usages_for_user(&alice.id) {
        let resolved = resolve_payment_for_usage(&alice, u.payment_used.clone());
        let payment_desc = match resolved {
            Some(pm) => format!("{:?}", pm),
            None => "NO_PAYMENT".to_string(),
        };
        println!(
            "Service {} used product {} paid with {}",
            u.service_id.0, u.product_id.0, payment_desc
        );
    }

    // demonstrate pure-style addition (returns new log)
    let new_usage = ServiceUsage::new(&bob.id, &"s-2".into(), &"p-3".into(), None);
    let log2 = log.add_usage(new_usage);
    println!(
        "\nOriginal log length: {}, New log length: {}",
        log.usages_len(),
        log2.usages_len()
    );
}
