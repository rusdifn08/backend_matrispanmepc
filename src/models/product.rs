use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ProductCategory {
    Spices,
    Coconut,
    Seafood,
    Textiles,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSpecification {
    pub grade: Option<String>,
    pub moisture: Option<String>,
    pub volatile_oil: Option<String>,
    pub ash_content: Option<String>,
    pub packaging: Option<String>,
    pub origin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub category: ProductCategory,
    pub name: String,
    pub latin_or_local_name: String,
    pub hs_code: String,
    pub min_order_qty: f64,
    pub unit: String,
    pub price_estimate_usd: f64,
    pub specs: ProductSpecification,
    pub image_url: String,
    pub description: String,
    pub featured: bool,
}
