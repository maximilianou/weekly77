use axum::routing::{get, post};
use axum::Router;
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;

mod api;
mod db;
mod images;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Initialize SQLite database
    let database_url = "sqlite:src03_backend.db";
    let pool = SqlitePool::connect(database_url).await?;
    db::init_db(&pool).await?;
    db::seed_db(&pool).await?;

    // Build our application with routes
    let app = Router::new()
        .route("/api/health", get(api::health))
        .route("/api/auth/seed", post(api::seed_user))
        .route("/api/products", get(api::list_products))
        .route("/api/uploads", post(api::upload_image))
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("✓ Server starting on http://0.0.0.0:3001");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

