/// Validates Indonesian Employee ID (NIK - Nomor Induk Karyawan PT. MEPC)
/// Expected format: MEPC-DIV-XXX (e.g. MEPC-EXP-088, MEPC-LOG-015, MEPC-DIR-001)
pub fn validate_nik(nik: &str) -> Result<(), String> {
    let nik = nik.trim();
    if !nik.starts_with("MEPC-") {
        return Err("NIK harus diawali dengan kode 'MEPC-' (Contoh: MEPC-EXP-088)".to_string());
    }

    let parts: Vec<&str> = nik.split('-').collect();
    if parts.len() != 3 {
        return Err("Format NIK harus MEPC-DIV-XXX (Contoh: MEPC-EXP-088)".to_string());
    }

    let div = parts[1];
    let num = parts[2];

    if div.len() < 2 || div.len() > 4 {
        return Err("Kode divisi dalam NIK tidak valid (Contoh: DIR, EXP, LOG, SOU, FIN)".to_string());
    }

    if num.len() != 3 || !num.chars().all(|c| c.is_ascii_digit()) {
        return Err("Nomor urut NIK harus 3 digit angka (Contoh: 001, 088)".to_string());
    }

    Ok(())
}
