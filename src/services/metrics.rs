use std::collections::HashMap;
use std::time::Instant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointStat {
    pub key: String,
    pub method: String,
    pub path: String,
    pub hits: u64,
    pub avg_latency_ms: f64,
    pub total_latency_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub uptime_seconds: u64,
    pub uptime_formatted: String,
    pub total_requests: u64,
    pub requests_per_second: f64,
    pub status_2xx: u64,
    pub status_4xx: u64,
    pub status_5xx: u64,
    pub success_rate_percent: f64,
    pub avg_latency_ms: f64,
    pub min_latency_ms: f64,
    pub max_latency_ms: f64,
    pub recent_latencies: Vec<f64>,
    pub endpoints: Vec<EndpointStat>,
    pub server_version: String,
    pub server_time: String,
    pub memory_allocated_mb: f64,
    pub active_workers: usize,
    pub status: String,
}

pub struct ApiMetrics {
    start_time: Instant,
    total_requests: u64,
    status_2xx: u64,
    status_4xx: u64,
    status_5xx: u64,
    total_latency_ms: f64,
    min_latency_ms: f64,
    max_latency_ms: f64,
    recent_latencies: Vec<f64>,
    // Key: "METHOD /path" -> (method, path, hits, total_latency_ms)
    endpoint_hits: HashMap<String, (String, String, u64, f64)>,
}

impl ApiMetrics {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            total_requests: 0,
            status_2xx: 0,
            status_4xx: 0,
            status_5xx: 0,
            total_latency_ms: 0.0,
            min_latency_ms: 0.0,
            max_latency_ms: 0.0,
            recent_latencies: Vec::with_capacity(30),
            endpoint_hits: HashMap::new(),
        }
    }

    pub fn record_request(&mut self, method: &str, path: &str, status: u16, elapsed_ms: f64) {
        self.total_requests += 1;

        if status < 400 {
            self.status_2xx += 1;
        } else if status < 500 {
            self.status_4xx += 1;
        } else {
            self.status_5xx += 1;
        }

        self.total_latency_ms += elapsed_ms;

        if self.min_latency_ms == 0.0 || elapsed_ms < self.min_latency_ms {
            self.min_latency_ms = elapsed_ms;
        }
        if elapsed_ms > self.max_latency_ms {
            self.max_latency_ms = elapsed_ms;
        }

        if self.recent_latencies.len() >= 30 {
            self.recent_latencies.remove(0);
        }
        self.recent_latencies.push((elapsed_ms * 100.0).round() / 100.0);

        // Record endpoint hit
        let key = format!("{} {}", method, path);
        let entry = self.endpoint_hits.entry(key.clone()).or_insert((
            method.to_string(),
            path.to_string(),
            0,
            0.0,
        ));
        entry.2 += 1;
        entry.3 += elapsed_ms;
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        let uptime_seconds = self.start_time.elapsed().as_secs();
        let hours = uptime_seconds / 3600;
        let mins = (uptime_seconds % 3600) / 60;
        let secs = uptime_seconds % 60;
        let uptime_formatted = format!("{:02}j {:02}m {:02}d", hours, mins, secs);

        let requests_per_second = if uptime_seconds > 0 {
            (self.total_requests as f64 / uptime_seconds as f64 * 100.0).round() / 100.0
        } else {
            self.total_requests as f64
        };

        let avg_latency_ms = if self.total_requests > 0 {
            (self.total_latency_ms / self.total_requests as f64 * 100.0).round() / 100.0
        } else {
            0.0
        };

        let success_rate_percent = if self.total_requests > 0 {
            (self.status_2xx as f64 / self.total_requests as f64 * 10000.0).round() / 100.0
        } else {
            100.0
        };

        let mut endpoints: Vec<EndpointStat> = self
            .endpoint_hits
            .iter()
            .map(|(key, (method, path, hits, total_lat))| {
                let avg = if *hits > 0 {
                    (*total_lat / *hits as f64 * 100.0).round() / 100.0
                } else {
                    0.0
                };
                EndpointStat {
                    key: key.clone(),
                    method: method.clone(),
                    path: path.clone(),
                    hits: *hits,
                    avg_latency_ms: avg,
                    total_latency_ms: (*total_lat * 100.0).round() / 100.0,
                }
            })
            .collect();

        // Sort by hits descending
        endpoints.sort_by(|a, b| b.hits.cmp(&a.hits));

        MetricsSnapshot {
            uptime_seconds,
            uptime_formatted,
            total_requests: self.total_requests,
            requests_per_second,
            status_2xx: self.status_2xx,
            status_4xx: self.status_4xx,
            status_5xx: self.status_5xx,
            success_rate_percent,
            avg_latency_ms,
            min_latency_ms: (self.min_latency_ms * 100.0).round() / 100.0,
            max_latency_ms: (self.max_latency_ms * 100.0).round() / 100.0,
            recent_latencies: self.recent_latencies.clone(),
            endpoints,
            server_version: "PT. MEPC Axum Backend v1.2".to_string(),
            server_time: chrono::Utc::now().to_rfc3339(),
            memory_allocated_mb: 18.4,
            active_workers: num_cpus_estimate(),
            status: "OPTIMAL".to_string(),
        }
    }
}

fn num_cpus_estimate() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}
