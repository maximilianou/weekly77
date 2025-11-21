use src02::models::{PaymentMethod, Product, Service, ServiceUsage, User};

// Note: tests are simple unit-style checks using the public API.
#[tokio::test]
async fn test_resolve_payment() {
    let alice = User::new(
        "u-alice",
        "Alice",
        Some(PaymentMethod::card("****", "Alice")),
    );
    let usage = ServiceUsage::new(&alice.id, &"s-1".into(), &"p-1".into(), None);
    let resolved = src02::usage::resolve_payment_for_usage(&alice, usage.payment_used.clone());
    assert!(resolved.is_some());
}

#[test]
fn test_catalog_products() {
    let p1 = Product::new("p-1", "Email Support", 500);
    let svc = Service::new("s-1", "SaaS", vec![p1.clone()]);
    let catalog = src02::catalog::Catalog::default().with_service(svc);
    let prods = catalog
        .list_products_for_service(&"s-1".into())
        .expect("service exists");
    assert_eq!(prods.len(), 1);
}
