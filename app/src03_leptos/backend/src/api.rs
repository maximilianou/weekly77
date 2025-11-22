use axum::extract::{Multipart, State};
use axum::response::Json;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::images;

#[derive(Serialize)]
pub struct Health {
    pub status: &'static str,
}

pub async fn health() -> Json<Health> {
    Json(Health { status: "ok" })
}

#[derive(Deserialize)]
pub struct SeedBody {
    pub username: String,
}

pub async fn seed_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<SeedBody>,
) -> Json<serde_json::Value> {
    let username = payload.username;
    let password = format!("{}01", username);
    let id = Uuid::new_v4().to_string();

    let result = sqlx::query(
        r#"
        INSERT OR IGNORE INTO users (id, username, password_hash, created_at)
        VALUES (?, ?, ?, datetime('now'))
        "#,
    )
    .bind(&id)
    .bind(&username)
    .bind(&password)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Json(serde_json::json!({"id": id, "username": username, "password": password})),
        Err(e) => Json(serde_json::json!({"error": format!("Failed to seed user: {}", e)})),
    }
}

pub async fn list_products(State(pool): State<SqlitePool>) -> Json<serde_json::Value> {
    let products = sqlx::query_as::<_, crate::db::Product>(
        "SELECT id, seller_id, name, description, price, created_at, updated_at FROM products"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    Json(serde_json::to_value(products).unwrap_or(serde_json::json!([])))
}

pub async fn upload_image(
    State(pool): State<SqlitePool>,
    mut multipart: Multipart,
) -> Json<serde_json::Value> {
    let mut processed = vec![];

    while let Ok(Some(field_res)) = multipart.next_field().await {
        let file_name_opt = field_res.file_name().map(|s| s.to_string());
        let data = match field_res.bytes().await {
            Ok(b) => b.to_vec(),
            Err(e) => {
                processed.push(serde_json::json!({"file": file_name_opt, "error": format!("Failed to read bytes: {}", e)}));
                continue;
            }
        };

        match images::process_image_bytes(&data).await {
            Ok(info) => {
                // Store in database
                let id = Uuid::new_v4().to_string();
                let product_id = "prod_001"; // Default product ID; in real app, this would come from context
                let seller_id = "seller01"; // Default seller; in real app, from auth token

                let _db_result = sqlx::query(
                    r#"
                    INSERT INTO product_images (id, product_id, seller_id, image_url, file_size_bytes, original_filename, approval_status, created_at)
                    VALUES (?, ?, ?, ?, ?, ?, 'pending', datetime('now'))
                    "#,
                )
                .bind(&id)
                .bind(product_id)
                .bind(seller_id)
                .bind(format!("uploads/{}", id))
                .bind(data.len() as i64)
                .bind(&file_name_opt.clone().unwrap_or_default())
                .execute(&pool)
                .await;

                processed.push(serde_json::json!({"file": file_name_opt, "info": info, "id": id}));
            }
            Err(e) => processed.push(serde_json::json!({"file": file_name_opt, "error": format!("{}", e)})),
        }
    }

    Json(serde_json::json!({"processed": processed}))
}
