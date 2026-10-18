use chrono::Utc;
use uuid::Uuid;
use crate::models::{
    BankAccountInfo, BuyerOrder, Invoice, InvoiceItem, InvoicePayload, InvoiceRefType,
};

pub struct InvoiceService;

impl InvoiceService {
    /// Generates a formal Proforma Invoice payload when Admin clicks 'OKE / APPROVE' on a Buyer Order
    pub fn create_buyer_proforma_invoice(
        order: &BuyerOrder,
        counter: usize,
        approved_by_admin: &str,
    ) -> Invoice {
        let year = Utc::now().format("%Y").to_string();
        let invoice_number = format!("INV/MEPC/{}/{:04}", year, counter);
        let today = Utc::now().format("%d %B %Y").to_string();
        let due_date = (Utc::now() + chrono::Duration::days(14)).format("%d %B %Y").to_string();

        let items: Vec<InvoiceItem> = order
            .items
            .iter()
            .map(|item| InvoiceItem {
                description: item.product_name.clone(),
                hs_code: item.hs_code.clone(),
                quantity: item.quantity,
                unit: item.unit.clone(),
                unit_price: item.unit_price_usd,
                subtotal: item.subtotal_usd,
            })
            .collect();

        let subtotal: f64 = items.iter().map(|i| i.subtotal).sum();
        let freight_insurance = if order.incoterms.contains("CIF") {
            subtotal * 0.06 // 6% freight & marine insurance estimate for CIF
        } else {
            0.0 // FOB Belawan: buyer arranges vessel freight
        };
        let total_amount = subtotal + freight_insurance;

        let bank_instructions = BankAccountInfo {
            bank_name: "PT. BANK MANDIRI (PERSERO) TBK".to_string(),
            account_name: "PT. MATRISPAN EXIM PHOENIX CORP".to_string(),
            account_number: "106-00-1899214-7 (USD / IDR FX ACCOUNT)".to_string(),
            swift_code: "BMRIIDJA".to_string(),
            branch: "KCP BINJAI JUANDA - SUMATERA UTARA, INDONESIA".to_string(),
        };

        let qr_verification_code = format!(
            "https://matrispanmepc.com/verify-invoice?code={}&token={}",
            invoice_number,
            &Uuid::new_v4().to_string()[..8]
        );

        let payload = InvoicePayload {
            invoice_number: invoice_number.clone(),
            title: "COMMERCIAL / PROFORMA EXPORT INVOICE".to_string(),
            status: "APPROVED BY ADMIN".to_string(),
            issued_date: today,
            due_date,
            ref_type: InvoiceRefType::BuyerOrder,
            ref_code: order.order_code.clone(),
            
            company_name: "PT. MATRISPAN EXIM PHOENIX CORP".to_string(),
            company_address: "Jl. Ir. H. Juanda No. 21 LK. II, Kota Binjai, Sumatera Utara 20714, Indonesia".to_string(),
            company_email: "info@matrispanmepc.com".to_string(),
            company_phone: "+62 851-8311-0510".to_string(),
            port_of_loading: "Belawan International Port (IDBLW), North Sumatra, Indonesia".to_string(),
            
            recipient_name: order.buyer_name.clone(),
            recipient_company: order.company_name.clone(),
            recipient_email: order.email.clone(),
            recipient_phone: order.whatsapp_number.clone(),
            destination_country: order.destination_country.clone(),
            destination_port: order.destination_port.clone(),
            incoterms: order.incoterms.clone(),
            payment_terms: "30% T/T Advance Deposit upon Proforma confirmation, 70% Irrevocable L/C at sight upon Bill of Lading (B/L)".to_string(),
            
            items,
            subtotal,
            shipping_and_insurance_fee: freight_insurance,
            total_amount,
            currency: "USD".to_string(),
            
            bank_instructions,
            qr_verification_code,
            authorized_signatory: format!("Direktur Operasional & Ekspor / {}", approved_by_admin),
            stamp_url: Some("/assets/stamps/mepc_official_seal.svg".to_string()),
        };

        Invoice {
            id: Uuid::new_v4().to_string(),
            invoice_number,
            ref_type: InvoiceRefType::BuyerOrder,
            ref_id: order.id.clone(),
            ref_code: order.order_code.clone(),
            approved_by_admin: approved_by_admin.to_string(),
            issued_at: Utc::now().to_rfc3339(),
            total_amount,
            currency: "USD".to_string(),
            payload,
        }
    }
}
