use sqlx::PgPool;
use serde_json::json;

pub async fn seed_products(pool: &PgPool) -> Result<(), sqlx::Error> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products")
        .fetch_one(pool)
        .await?;

    if count.0 == 0 {
        tracing::info!("Seeding authentic export commodities into NeonDB...");
        let prods = vec![
            (
                "prod-spices-01", "Spices", "Nutmeg Whole ABCD Grade (Biji Pala Utuh)", "Myristica fragrans", "0908.11.00",
                5.0, "MT", 8400.0,
                json!({
                    "grade": "ABCD Grade (Sound, Whole)", "moisture": "Max 10% - 12%",
                    "volatile_oil": "Min 6.5% - 7.5%", "ash_content": "Max 2.0%",
                    "packaging": "50 Kg Double Jute Bags with inner poly liner",
                    "origin": "North Sumatra & Banda, Indonesia"
                }),
                "https://images.unsplash.com/photo-1596040033229-a9821ebd058d?auto=format&fit=crop&w=800&q=80",
                "Biji pala utuh kualitas ekspor tinggi aroma atsiri, disortir cermat bebas jamur dan aflatoksin, siap untuk industri rempah dan pangan internasional.",
                true
            ),
            (
                "prod-spices-02", "Spices", "Cloves Lal Pari Quality (Cengkeh Lal Pari)", "Syzygium aromaticum", "0907.10.00",
                5.0, "MT", 9800.0,
                json!({
                    "grade": "Lal Pari Handpicked", "moisture": "Max 11% - 12%",
                    "volatile_oil": "Min 18% - 20%", "ash_content": "Max 0.5% stems",
                    "packaging": "50 Kg PP Woven Bags with PE Inner",
                    "origin": "Zanzibar & Moluccas cultivars, North Sumatra"
                }),
                "https://images.unsplash.com/photo-1509358271058-acd22cc93898?auto=format&fit=crop&w=800&q=80",
                "Cengkeh kering warna coklat kemerahan (Lal Pari), kadar minyak esensial eugenol pekat, cocok untuk industri farmasi, rokok kretek, dan bumbu dapur Eropa.",
                true
            ),
            (
                "prod-spices-03", "Spices", "Java Green Cardamom (Kapulaga Jawa Super)", "Amomum compactum", "0908.31.00",
                2.0, "MT", 14200.0,
                json!({
                    "grade": "Super Cleaned Bold", "moisture": "Max 11%",
                    "volatile_oil": "Min 4.0%", "ash_content": "Max 1.0%",
                    "packaging": "25 Kg Master Cartons with vacuum pouches",
                    "origin": "West & North Sumatra highlands"
                }),
                "https://images.unsplash.com/photo-1615485290382-441e4d049cb5?auto=format&fit=crop&w=800&q=80",
                "Kapulaga putih/hijau berbentuk bulat utuh beraroma wangi herba kuat, dibersihkan higienis dengan sirkulasi udara hangat konstan.",
                false
            ),
            (
                "prod-spices-04", "Spices", "Gayo & Mandheling Arabica Green Coffee (Grade 1 TP)", "Coffea arabica", "0901.11.10",
                10.0, "MT", 6950.0,
                json!({
                    "grade": "Grade 1 Triple Picked", "moisture": "Max 12.0%",
                    "volatile_oil": "N/A", "ash_content": "Defect < 8",
                    "packaging": "60 Kg GrainPro + Jute Bags",
                    "origin": "Gayo Highlands & Mandailing, North Sumatra"
                }),
                "https://images.unsplash.com/photo-1559056199-641a0ac8b55e?auto=format&fit=crop&w=800&q=80",
                "Kopi arabika proses basah (wet-hulled / giling basah) aroma rempah rempah tanah basah khas Sumatera, cupping score di atas 84.5 poin.",
                true
            ),
            (
                "prod-coconut-01", "Coconut", "Premium Shisha Coconut Charcoal Briquette (Briket Arang Batok)", "Cocos nucifera shell", "4402.90.10",
                18.0, "MT", 1480.0,
                json!({
                    "grade": "Cube 25x25x25 mm / Kalori > 7400 Kcal", "moisture": "Max 5.0%",
                    "volatile_oil": "Fixed Carbon: 82% - 85%", "ash_content": "White Ash Max 2.0%",
                    "packaging": "1 Kg Inner Box with 10 Kg Master Carton",
                    "origin": "Asahan & Batu Bara, North Sumatra"
                }),
                "https://images.unsplash.com/photo-1580984969071-a8da5656c2fb?auto=format&fit=crop&w=800&q=80",
                "Briket batok kelapa murni tanpa campuran kayu atau bahan kimia, panas stabil hingga 2.5 jam, abu putih tipis untuk pasar Timur Tengah, Turki, dan Eropa.",
                true
            ),
            (
                "prod-coconut-02", "Coconut", "White Copra Edible & Regular Grade (Kopra Putih Asalan)", "Cocos nucifera kernel", "1203.00.00",
                25.0, "MT", 1250.0,
                json!({
                    "grade": "Edible Grade Half-Cup / Regular Cup", "moisture": "Max 5.0%",
                    "volatile_oil": "Oil Content: Min 65%", "ash_content": "FFA: Max 0.5%",
                    "packaging": "50 Kg Gunny Bags",
                    "origin": "Coastal North Sumatra & Nias"
                }),
                "https://images.unsplash.com/photo-1546554137-f86b9593a222?auto=format&fit=crop&w=800&q=80",
                "Kopra putih hasil pengeringan oven tidak berjamur, kadar minyak kelapa murni tinggi dan kadar asam lemak bebas sangat rendah.",
                false
            ),
            (
                "prod-seafood-01", "Seafood", "Frozen Black Tiger Shrimp Headless (Udang Windu Beku HLSO)", "Penaeus monodon", "0306.17.10",
                10.0, "MT", 15500.0,
                json!({
                    "grade": "HLSO Grade A / Count 16/20, 21/25", "moisture": "Blast Frozen -40°C",
                    "volatile_oil": "N/A", "ash_content": "Free of Antibiotics",
                    "packaging": "1 Kg Inner Box x 10 per Master Carton (10 Kg Net)",
                    "origin": "East Coast Coastlines of North Sumatra"
                }),
                "https://images.unsplash.com/photo-1565680018434-b513d5e5fd47?auto=format&fit=crop&w=800&q=80",
                "Udang windu laut tangkapan tambak pesisir alami Sumatera, disortir cepat dan dibekukan dengan standar HACCP internasional untuk pasar Jepang dan AS.",
                true
            ),
            (
                "prod-seafood-02", "Seafood", "Frozen Mud Crab Whole & Cut (Kepiting Bakau Beku)", "Scylla serrata", "0306.14.00",
                5.0, "MT", 14800.0,
                json!({
                    "grade": "Male / Female Egg (Sizes 500g - 800g+)", "moisture": "IQF Blast Frozen",
                    "volatile_oil": "N/A", "ash_content": "100% Net Weight",
                    "packaging": "Insulated Styrofoam 20 Kg with Gel packs / Cartons",
                    "origin": "East Coast Mangrove Coast of North Sumatra"
                }),
                "https://images.unsplash.com/photo-1544551763-46a013bb70d5?auto=format&fit=crop&w=800&q=80",
                "Kepiting bakau hidup pilihan langsung dibekukan dengan nitrogen cepat, isi daging padat di atas 90%, capit tebal gurih untuk pasar Asia Timur dan Timur Tengah.",
                false
            ),
            (
                "prod-textile-01", "Textiles", "Woven Viscose Rayon Fabric (Kain Rayon Ekspor)", "100% Regenerated Cellulose", "5408.22.00",
                20000.0, "Meters", 1.85,
                json!({
                    "grade": "30s x 30s / 68 x 68 (Width 58 inch)", "moisture": "Pre-shrunk, OEKO-TEX Standard 100",
                    "volatile_oil": "N/A", "ash_content": "Tensile Strength Certified",
                    "packaging": "Rolls in plastic tubes with woven outer wrapping (100m/roll)",
                    "origin": "Sumatra Integrated Textile Mills"
                }),
                "https://images.unsplash.com/photo-1620799140408-edc6dcb6d633?auto=format&fit=crop&w=800&q=80",
                "Kain rayon lembut jatuh sejuk di kulit dengan serapan pewarna reaktif tinggi, ramah lingkungan berstandar OEKO-TEX untuk industri garmen global.",
                true
            )
        ];

        for (id, cat, name, latin, hs, moq, unit, price, specs, img, desc, feat) in prods {
            sqlx::query(
                r#"
                INSERT INTO products (id, category, name, latin_or_local_name, hs_code, min_order_qty, unit, price_estimate_usd, specs, image_url, description, featured)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                ON CONFLICT (id) DO NOTHING;
                "#
            )
            .bind(id)
            .bind(cat)
            .bind(name)
            .bind(latin)
            .bind(hs)
            .bind(moq)
            .bind(unit)
            .bind(price)
            .bind(specs)
            .bind(img)
            .bind(desc)
            .bind(feat)
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}
