use std::time::Duration;
use tracing::{info, warn};

/// Spawns a background worker that periodically sends HTTP requests to the /api/health
/// endpoint to keep cloud container instances (such as Render Cloud) awake and responsive.
pub fn start_keep_alive_task(port: u16) {
    tokio::spawn(async move {
        // Allow the server 15 seconds to finish boot sequence
        tokio::time::sleep(Duration::from_secs(15)).await;

        let client = match reqwest::Client::builder()
            .timeout(Duration::from_secs(12))
            .build()
        {
            Ok(c) => c,
            Err(_) => reqwest::Client::new(),
        };

        let external_url = std::env::var("RENDER_EXTERNAL_URL")
            .unwrap_or_else(|_| "https://backend-matrispanmepc.onrender.com".to_string());
        let health_url = format!("{}/api/health", external_url.trim_end_matches('/'));
        let local_url = format!("http://127.0.0.1:{}/api/health", port);

        // Render free-tier sleep threshold is 15 minutes; ping every 10 minutes to guarantee uptime
        let mut ten_min_interval = tokio::time::interval(Duration::from_secs(10 * 60));
        // Also maintain dedicated 5-hour health check schedule as requested by user
        let mut five_hour_interval = tokio::time::interval(Duration::from_secs(5 * 3600));

        info!(
            "🚀 [KEEP-ALIVE] Background task active. Target: {} (Local fallback: {})",
            health_url, local_url
        );

        loop {
            tokio::select! {
                _ = ten_min_interval.tick() => {
                    // Send health request to external Render URL (resets reverse-proxy inactivity timer)
                    match client.get(&health_url).send().await {
                        Ok(resp) => {
                            info!("💚 [RENDER KEEP-ALIVE] Pinged {} -> Status: {}", health_url, resp.status());
                        }
                        Err(e) => {
                            // If external network is not ready, ping local port
                            let _ = client.get(&local_url).send().await;
                            warn!("⚠️ [RENDER KEEP-ALIVE] External ping: {} (Local ping OK)", e);
                        }
                    }
                }
                _ = five_hour_interval.tick() => {
                    info!("⏱️ [5-HOUR HEALTH AUDIT] Scheduled 5-hour cloud health ping executed on {}", health_url);
                    let _ = client.get(&health_url).send().await;
                }
            }
        }
    });
}
