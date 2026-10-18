use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use sqlx::Row;
use crate::{
    models::{Product, ProductCategory, ProductSpecification},
    services::AppState,
};

#[derive(Debug, Deserialize)]
pub struct ProductQuery {
    pub category: Option<String>,
}

fn row_to_product(row: &sqlx::postgres::PgRow) -> Product {
    let cat_str: String = row.get("category");
    let category = match cat_str.to_lowercase().as_str() {
        "coconut" => ProductCategory::Coconut,
        "seafood" => ProductCategory::Seafood,
        "textiles" => ProductCategory::Textiles,
        _ => ProductCategory::Spices,
    };

    Product {
        id: row.get("id"),
        category,
        name: row.get("name"),
        latin_or_local_name: row.get::<Option<String>, _>("latin_or_local_name").unwrap_or_default(),
        hs_code: row.get("hs_code"),
        min_order_qty: row.get("min_order_qty"),
        unit: row.get("unit"),
        price_estimate_usd: row.get("price_estimate_usd"),
        specs: ProductSpecification {
            grade: row.get("grade"),
            moisture: row.get("moisture"),
            volatile_oil: row.get("volatile_oil"),
            ash_content: row.get("ash_content"),
            packaging: row.get("packaging"),
            origin: row.get("origin"),
        },
        image_url: row.get::<Option<String>, _>("image_url").unwrap_or_default(),
        description: row.get::<Option<String>, _>("description").unwrap_or_default(),
        featured: row.get("featured"),
    }
}

pub async fn list_products(
    State(state): State<AppState>,
    Query(query): Query<ProductQuery>,
) -> Json<Vec<Product>> {
    let result = if let Some(cat_str) = query.category {
        sqlx::query(
            r#"
            SELECT * FROM products
            WHERE LOWER(category) = LOWER($1)
            ORDER BY id ASC
            "#,
        )
        .bind(cat_str)
        .fetch_all(&state.pool)
        .await
    } else {
        sqlx::query(
            r#"
            SELECT * FROM products
            ORDER BY id ASC
            "#,
        )
        .fetch_all(&state.pool)
        .await
    };

    match result {
        Ok(rows) => {
            let products = rows.iter().map(row_to_product).collect();
            Json(products)
        }
        Err(e) => {
            tracing::error!("Failed to fetch products from NeonDB: {}", e);
            Json(Vec::new())
        }
    }
}

pub async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Product>, StatusCode> {
    let row = sqlx::query("SELECT * FROM products WHERE id = $1")
        .bind(&id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some(r) => Ok(Json(row_to_product(&r))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductPayload {
    pub price_estimate_usd: Option<f64>,
    pub min_order_qty: Option<f64>,
    pub featured: Option<bool>,
}

pub async fn update_product(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateProductPayload>,
) -> Result<Json<Product>, StatusCode> {
    let row = sqlx::query(
        r#"
        UPDATE products
        SET price_estimate_usd = COALESCE($1, price_estimate_usd),
            min_order_qty = COALESCE($2, min_order_qty),
            featured = COALESCE($3, featured)
        WHERE id = $4
        RETURNING *
        "#,
    )
    .bind(payload.price_estimate_usd)
    .bind(payload.min_order_qty)
    .bind(payload.featured)
    .bind(&id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some(r) => {
            let prod = row_to_product(&r);
            Ok(Json(prod))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}
