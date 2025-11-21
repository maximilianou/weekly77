use src02::catalog::Catalog;
use src02::models::{Product, Service};

#[test]
fn test_get_service_used() {
    let p1 = Product::new("p-1", "Email", 100);
    let svc = Service::new("s-1", "SaaS", vec![p1.clone()]);
    let catalog = Catalog::default().with_service(svc.clone());
    let got = catalog.get_service(&"s-1".into());
    assert!(got.is_some());
    let svc2 = got.unwrap();
    assert_eq!(svc2.name, "SaaS");
}
