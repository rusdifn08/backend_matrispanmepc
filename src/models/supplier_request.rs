use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SupplierRequestStatus {
    Pending,
    Contacted,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReadinessStatus {
    #[serde(rename = "Ready Stock")]
    ReadyStock,
    #[serde(rename = "Pre-Harvest")]
    PreHarvest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierRequest {
    pub id: String,
    pub req_code: String,
    pub supplier_name: String,
    pub group_or_cv_name: String,
    pub location_regency: String,
    pub location_province: String,
    pub whatsapp_number: String,
    pub commodity_category: String,
    pub commodity_name: String,
    pub qty_monthly_capacity: f64,
    pub unit: String,
    pub moisture_or_grade: String,
    pub offered_price_idr: f64,
    pub readiness_status: String,
    pub notes: Option<String>,
    pub status: SupplierRequestStatus,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateSupplierRequestPayload {
    pub supplier_name: String,
    pub group_or_cv_name: String,
    pub location_regency: String,
    pub location_province: String,
    pub whatsapp_number: String,
    pub commodity_category: String,
    pub commodity_name: String,
    pub qty_monthly_capacity: f64,
    pub unit: String,
    pub moisture_or_grade: String,
    pub offered_price_idr: f64,
    pub readiness_status: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateSupplierStatusPayload {
    pub status: SupplierRequestStatus,
    pub agreed_price_idr: Option<f64>,
}
