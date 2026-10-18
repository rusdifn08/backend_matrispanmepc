use crate::models::{BuyerOrder, SupplierRequest};

pub const MEPC_ADMIN_WA: &str = "6285183110510";

pub struct WhatsAppService;

impl WhatsAppService {
    /// Formats the official WhatsApp payload message for new Buyer Orders (PRD Section 07)
    pub fn format_buyer_order_message(order: &BuyerOrder) -> String {
        let mut items_summary = String::new();
        for (i, item) in order.items.iter().enumerate() {
            items_summary.push_str(&format!("{}. {} ({} {})\n", i + 1, item.product_name, item.quantity, item.unit));
        }

        format!(
            "*[NOTIFIKASI ORDER BUYER BARU - PT. MEPC]*\n\
            No. Order : #{}\n\
            Perusahaan : {} ({})\n\
            PIC / Buyer : {}\n\
            Pesanan :\n{}\
            Incoterms : {}\n\
            Pelabuhan Tujuan : {}\n\
            Kontak WA Buyer : {}\n\
            Status : Menunggu Follow-Up & Approval Admin\n\n\
            _Pesan otomatis dikirim melalui Sistem Terintegrasi matrispanmepc.com_",
            order.order_code,
            order.company_name,
            order.destination_country,
            order.buyer_name,
            items_summary,
            order.incoterms,
            order.destination_port,
            order.whatsapp_number
        )
    }

    /// Formats the official WhatsApp payload message for new Supplier Requests
    pub fn format_supplier_request_message(req: &SupplierRequest) -> String {
        format!(
            "*[NOTIFIKASI PENGAJUAN SUPLAI BARU - PT. MEPC]*\n\
            No. Pengajuan : #{}\n\
            Mitra / Kelompok Tani : {}\n\
            Nama Kontak : {}\n\
            Komoditas : {} ({})\n\
            Kapasitas Rutin : {} {}/Bulan\n\
            Lokasi Kebun/Gudang : {}, {}\n\
            Kadar Air / Mutu : {}\n\
            Harga Pengajuan : IDR {:.*} / {}\n\
            Status Kesiapan : {}\n\
            Kontak WA Mitra : {}\n\
            Catatan : {}\n\n\
            _Pesan otomatis tercatat di Antrean Dashboard Admin PT. MEPC_",
            req.req_code,
            req.group_or_cv_name,
            req.supplier_name,
            req.commodity_name,
            req.commodity_category,
            req.qty_monthly_capacity,
            req.unit,
            req.location_regency,
            req.location_province,
            req.moisture_or_grade,
            0,
            req.offered_price_idr,
            req.unit,
            req.readiness_status,
            req.whatsapp_number,
            req.notes.as_deref().unwrap_or("-")
        )
    }

    /// Generates Direct Click-to-Chat URL
    pub fn generate_click_to_chat_url(phone_number: &str, text: &str) -> String {
        let clean_phone = phone_number
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>();
        
        let encoded_text = urlencoding(text);
        format!("https://wa.me/{}?text={}", clean_phone, encoded_text)
    }
}

fn urlencoding(input: &str) -> String {
    let mut encoded = String::new();
    for b in input.bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(b as char);
            }
            b' ' => encoded.push_str("%20"),
            b'\n' => encoded.push_str("%0A"),
            _ => {
                encoded.push_str(&format!("%{:02X}", b));
            }
        }
    }
    encoded
}
