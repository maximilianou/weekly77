use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub id: String,
    pub seller_id: String,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProductImage {
    pub id: String,
    pub product_id: String,
    pub seller_id: String,
    pub image_url: String,
    pub file_size_bytes: i64,
    pub original_filename: String,
    pub approval_status: String, // "pending", "approved", "rejected"
    pub approval_notes: Option<String>,
    pub approved_by: Option<String>,
    pub approved_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Sale {
    pub id: String,
    pub buyer_id: String,
    pub product_id: String,
    pub quantity: i32,
    pub total_price: f64,
    pub created_at: String,
}

/// Initialize SQLite database with schema
pub async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Create tables
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            created_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS products (
            id TEXT PRIMARY KEY,
            seller_id TEXT NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            price REAL NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY(seller_id) REFERENCES users(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS product_images (
            id TEXT PRIMARY KEY,
            product_id TEXT NOT NULL,
            seller_id TEXT NOT NULL,
            image_url TEXT NOT NULL,
            file_size_bytes INTEGER NOT NULL,
            original_filename TEXT NOT NULL,
            approval_status TEXT NOT NULL DEFAULT 'pending',
            approval_notes TEXT,
            approved_by TEXT,
            approved_at TEXT,
            created_at TEXT NOT NULL,
            FOREIGN KEY(product_id) REFERENCES products(id),
            FOREIGN KEY(seller_id) REFERENCES users(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sales (
            id TEXT PRIMARY KEY,
            buyer_id TEXT NOT NULL,
            product_id TEXT NOT NULL,
            quantity INTEGER NOT NULL,
            total_price REAL NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY(buyer_id) REFERENCES users(id),
            FOREIGN KEY(product_id) REFERENCES products(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Seed database with sample data
pub async fn seed_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Create seed user
    let user_id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO users (id, username, password_hash, created_at)
        VALUES (?, ?, ?, datetime('now'))
        "#,
    )
    .bind(&user_id)
    .bind("seller01")
    .bind("hashed_password_here") // In production, use bcrypt/argon2
    .execute(pool)
    .await?;

    // Create sample products
    let prod_id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO products (id, seller_id, name, description, price, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))
        "#,
    )
    .bind(&prod_id)
    .bind(&user_id)
    .bind("Premium Widget")
    .bind("High-quality widget with excellent features")
    .bind(29.99)
    .execute(pool)
    .await?;

    Ok(())
}
