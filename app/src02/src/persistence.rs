use crate::models::{
    PaymentMethod, Product, ProductId, Service, ServiceId, ServiceUsage, User, UserId,
};
use serde_json;
use sqlx::{Row, SqlitePool};

pub async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // create tables
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            display_name TEXT NOT NULL,
            default_payment TEXT NULL
        );"#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS services (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL
        );"#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS products (
            id TEXT PRIMARY KEY,
            service_id TEXT NOT NULL,
            name TEXT NOT NULL,
            price_cents INTEGER NOT NULL,
            FOREIGN KEY(service_id) REFERENCES services(id)
        );"#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS usages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id TEXT NOT NULL,
            service_id TEXT NOT NULL,
            product_id TEXT NOT NULL,
            payment_used TEXT NULL
        );"#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn save_user(pool: &SqlitePool, user: &User) -> Result<(), sqlx::Error> {
    let payment_json = user.profile.default_payment.as_ref()
        .map(|pm| serde_json::to_string(pm).unwrap());
    sqlx::query(
        "INSERT OR REPLACE INTO users (id, display_name, default_payment) VALUES (?, ?, ?)",
    )
    .bind(&user.id.0)
    .bind(&user.profile.display_name)
    .bind(payment_json)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn save_service(pool: &SqlitePool, service: &Service) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT OR REPLACE INTO services (id, name) VALUES (?, ?)")
        .bind(&service.id.0)
        .bind(&service.name)
        .execute(pool)
        .await?;

    for p in &service.products {
        sqlx::query("INSERT OR REPLACE INTO products (id, service_id, name, price_cents) VALUES (?, ?, ?, ?)")
            .bind(&p.id.0)
            .bind(&service.id.0)
            .bind(&p.name)
            .bind(p.price_cents as i64)
            .execute(pool)
            .await?;
    }
    Ok(())
}

pub async fn save_usage(pool: &SqlitePool, usage: &ServiceUsage) -> Result<(), sqlx::Error> {
    let payment_json = usage
        .payment_used
        .as_ref()
        .map(|pm| serde_json::to_string(pm).unwrap());
    sqlx::query(
        "INSERT INTO usages (user_id, service_id, product_id, payment_used) VALUES (?, ?, ?, ?)",
    )
    .bind(&usage.user_id.0)
    .bind(&usage.service_id.0)
    .bind(&usage.product_id.0)
    .bind(payment_json)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_users(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, display_name, default_payment FROM users")
        .fetch_all(pool)
        .await?;
    let mut out = Vec::new();
    for r in rows {
        let id: String = r.get("id");
        let display_name: String = r.get("display_name");
        let default_payment: Option<String> = r.get("default_payment");
        let payment = match default_payment {
            Some(s) => serde_json::from_str::<PaymentMethod>(&s).ok(),
            None => None,
        };
        let u = User::new(&id, &display_name, payment);
        out.push(u);
    }
    Ok(out)
}

pub async fn get_services(pool: &SqlitePool) -> Result<Vec<Service>, sqlx::Error> {
    // fetch services
    let services_rows = sqlx::query("SELECT id, name FROM services")
        .fetch_all(pool)
        .await?;
    let mut services = Vec::new();
    for s in services_rows {
        let sid: String = s.get("id");
        let sname: String = s.get("name");
        let product_rows =
            sqlx::query("SELECT id, name, price_cents FROM products WHERE service_id = ?")
                .bind(&sid)
                .fetch_all(pool)
                .await?;
        let products = product_rows
            .into_iter()
            .map(|pr| {
                let pid: String = pr.get("id");
                let pname: String = pr.get("name");
                let price: i64 = pr.get("price_cents");
                Product::new(&pid, &pname, price as u64)
            })
            .collect();
        services.push(Service::new(&sid, &sname, products));
    }
    Ok(services)
}

pub async fn get_usages_for_user(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<Vec<ServiceUsage>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT user_id, service_id, product_id, payment_used FROM usages WHERE user_id = ?",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    let mut out = Vec::new();
    for r in rows {
        let uid: String = r.get("user_id");
        let sid: String = r.get("service_id");
        let pid: String = r.get("product_id");
        let payment_s: Option<String> = r.get("payment_used");
        let payment = match payment_s {
            Some(s) => serde_json::from_str::<PaymentMethod>(&s).ok(),
            None => None,
        };
        out.push(ServiceUsage::new(
            &UserId(uid),
            &ServiceId(sid),
            &ProductId(pid),
            payment,
        ));
    }
    Ok(out)
}
