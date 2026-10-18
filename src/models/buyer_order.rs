use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    Pending,
    Contacted,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Incoterms {
    #[serde(rename = "FOB Belawan")]
    FobBelawan,
    #[serde(rename = "CIF")]
    Cif,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub product_id: String,
    pub product_name: String,
    pub hs_code: String,
    pub quantity: f64,
    pub unit: String,
    pub unit_price_usd: f64,
    pub subtotal_usd: f64,
    pub packaging_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyerOrder {
    pub id: String,
    pub order_code: String,
    pub buyer_name: String,
    pub company_name: String,
    pub email: String,
    pub whatsapp_number: String,
    pub destination_country: String,
    pub destination_port: String,
    pub incoterms: String,
    pub items: Vec<OrderItem>,
    pub status: OrderStatus,
    pub total_amount: f64,
    pub currency: String,
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateBuyerOrderPayload {
    pub buyer_name: String,
    pub company_name: String,
    pub email: String,
    pub whatsapp_number: String,
    pub destination_country: String,
    pub destination_port: String,
    pub incoterms: String,
    pub items: Vec<OrderItem>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateOrderStatusPayload {
    pub status: OrderStatus,
    pub adjusted_total_amount: Option<f64>,
}
