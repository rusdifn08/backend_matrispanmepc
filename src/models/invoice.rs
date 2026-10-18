use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceRefType {
    BuyerOrder,
    SupplierPo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub description: String,
    pub hs_code: String,
    pub quantity: f64,
    pub unit: String,
    pub unit_price: f64,
    pub subtotal: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccountInfo {
    pub bank_name: String,
    pub account_name: String,
    pub account_number: String,
    pub swift_code: String,
    pub branch: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoicePayload {
    pub invoice_number: String,
    pub title: String,
    pub status: String,
    pub issued_date: String,
    pub due_date: String,
    pub ref_type: InvoiceRefType,
    pub ref_code: String,
    
    // MEPC Corporate Details
    pub company_name: String,
    pub company_address: String,
    pub company_email: String,
    pub company_phone: String,
    pub port_of_loading: String,
    
    // Recipient Details
    pub recipient_name: String,
    pub recipient_company: String,
    pub recipient_email: String,
    pub recipient_phone: String,
    pub destination_country: String,
    pub destination_port: String,
    pub incoterms: String,
    pub payment_terms: String,
    
    // Line items & calculations
    pub items: Vec<InvoiceItem>,
    pub subtotal: f64,
    pub shipping_and_insurance_fee: f64,
    pub total_amount: f64,
    pub currency: String,
    
    // Bank Instructions & Validation
    pub bank_instructions: BankAccountInfo,
    pub qr_verification_code: String,
    pub authorized_signatory: String,
    pub stamp_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub invoice_number: String,
    pub ref_type: InvoiceRefType,
    pub ref_id: String,
    pub ref_code: String,
    pub approved_by_admin: String,
    pub issued_at: String,
    pub total_amount: f64,
    pub currency: String,
    pub payload: InvoicePayload,
}
