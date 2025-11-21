use src02::models::{PaymentMethod, Product, Service, ServiceUsage, User};
use src02::persistence;

#[tokio::test]
async fn test_persistence_in_memory() -> Result<(), Box<dyn std::error::Error>> {
    // in-memory sqlite
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await?;
    persistence::init_db(&pool).await?;

    let alice = User::new(
        "u-alice",
        "Alice",
        Some(PaymentMethod::card("****", "Alice")),
    );
    persistence::save_user(&pool, &alice).await?;

    let p1 = Product::new("p-1", "Email Support", 500);
    let svc = Service::new("s-1", "SaaS", vec![p1.clone()]);
    persistence::save_service(&pool, &svc).await?;

    let usage = ServiceUsage::new(&alice.id, &svc.id, &p1.id, None);
    persistence::save_usage(&pool, &usage).await?;

    let users = persistence::get_users(&pool).await?;
    assert_eq!(users.len(), 1);

    let services = persistence::get_services(&pool).await?;
    assert_eq!(services.len(), 1);

    let usages = persistence::get_usages_for_user(&pool, &alice.id.0).await?;
    assert_eq!(usages.len(), 1);

    Ok(())
}
