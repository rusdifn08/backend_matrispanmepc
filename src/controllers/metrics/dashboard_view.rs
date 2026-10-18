use axum::{extract::State, response::Html};
use crate::services::AppState;

pub async fn get_metrics_dashboard_html(State(state): State<AppState>) -> Html<String> {
    let snapshot = {
        let metrics = state.metrics.read().await;
        metrics.snapshot()
    };

    let orders_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM buyer_orders")
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);
    let suppliers_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM supplier_requests")
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);
    let invoices_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM invoices")
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);
    let messages_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM messages")
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);
    let products_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM products")
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);

    let mut endpoints_rows = String::new();
    for ep in &snapshot.endpoints {
        endpoints_rows.push_str(&format!(
            r#"<tr class="border-b border-emerald-900/30 hover:bg-emerald-900/20 font-mono text-xs">
                <td class="p-3 text-amber-400 font-bold">{}</td>
                <td class="p-3 font-semibold text-white">{}</td>
                <td class="p-3 text-right">{}</td>
                <td class="p-3 text-right text-emerald-300 font-bold">{:.2} ms</td>
            </tr>"#,
            ep.method, ep.path, ep.hits, ep.avg_latency_ms
        ));
    }

    let html = format!(r##"<!DOCTYPE html>
<html lang="id">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>PT. MEPC - Axum Performance Dashboard</title>
  <script src="https://cdn.tailwindcss.com"></script>
  <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;600;700;800&family=JetBrains+Mono:wght@400;600&display=swap" rel="stylesheet">
  <style>
    body {{ font-family: 'Plus Jakarta Sans', sans-serif; background-color: #051410; color: #E2E8F0; }}
    .font-mono {{ font-family: 'JetBrains Mono', monospace; }}
  </style>
</head>
<body class="min-h-screen p-4 sm:p-8">
  <div class="max-w-6xl mx-auto space-y-6">
    <header class="flex flex-col sm:flex-row sm:items-center justify-between pb-6 border-b border-emerald-900/40 gap-4">
      <div>
        <div class="flex items-center gap-2 mb-1">
          <span class="px-2.5 py-0.5 rounded-full text-[10px] font-mono bg-emerald-900/60 text-emerald-400 border border-emerald-500/30">
            AXUM v0.7 • TOKIO ASYNC RUNTIME • NEONDB
          </span>
          <span class="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-[10px] bg-emerald-500/20 text-emerald-300 font-bold">
            <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span> SYSTEM HEALTHY
          </span>
        </div>
        <h1 class="text-2xl font-extrabold text-white">Telemetry & Tele-monitoring Server</h1>
        <p class="text-xs text-emerald-300/80">PT. MATRISPAN EXIM PHOENIX CORP (PT. MEPC) • Pelabuhan Belawan Hub</p>
      </div>
      <div class="flex items-center gap-2">
        <a href="http://127.0.0.1:5173" class="px-3 py-1.5 rounded-xl bg-white/10 hover:bg-white/20 border border-white/20 text-xs font-bold text-white transition">
          Buka Web Publik
        </a>
        <a href="http://127.0.0.1:5173/#admin" class="px-3 py-1.5 rounded-xl bg-[#C88A2E] hover:bg-[#d99632] text-xs font-bold text-white transition">
          Admin Portal
        </a>
      </div>
    </header>

    <div class="grid grid-cols-2 lg:grid-cols-4 gap-4">
      <div class="p-4 rounded-2xl bg-emerald-950/40 border border-emerald-800/40">
        <div class="text-[11px] font-mono text-emerald-400 uppercase">Total API Requests</div>
        <div class="text-2xl font-bold font-mono text-white mt-1">{total_requests}</div>
        <div class="text-[10px] text-gray-400 mt-1">2xx: {success_2xx} | 4xx: {client_err_4xx} | 5xx: {server_err_5xx}</div>
      </div>
      <div class="p-4 rounded-2xl bg-emerald-950/40 border border-emerald-800/40">
        <div class="text-[11px] font-mono text-amber-400 uppercase">Avg Response Latency</div>
        <div class="text-2xl font-bold font-mono text-amber-300 mt-1">{avg_latency:.2} ms</div>
        <div class="text-[10px] text-emerald-300/80 mt-1">Tokio Work-Stealing Multi-Threaded</div>
      </div>
      <div class="p-4 rounded-2xl bg-emerald-950/40 border border-emerald-800/40">
        <div class="text-[11px] font-mono text-teal-400 uppercase">Active PostgreSQL Pool</div>
        <div class="text-2xl font-bold font-mono text-white mt-1">{db_entities_sum} Entitas</div>
        <div class="text-[10px] text-gray-400 mt-1">NeonDB Cloud Pooler Singapore</div>
      </div>
      <div class="p-4 rounded-2xl bg-emerald-950/40 border border-emerald-800/40">
        <div class="text-[11px] font-mono text-purple-400 uppercase">Server Uptime</div>
        <div class="text-2xl font-bold font-mono text-white mt-1">{uptime_secs}s</div>
        <div class="text-[10px] text-gray-400 mt-1">Started: {start_time}</div>
      </div>
    </div>

    <div class="p-6 rounded-3xl bg-emerald-950/30 border border-emerald-800/40 space-y-4">
      <div class="flex items-center justify-between">
        <h2 class="text-base font-bold text-white">NeonDB Database Records Summary</h2>
        <span class="text-xs font-mono text-emerald-400">PostgreSQL Cloud</span>
      </div>
      <div class="grid grid-cols-2 sm:grid-cols-5 gap-3 text-center">
        <div class="p-3 rounded-xl bg-black/40 border border-emerald-900/40">
          <div class="text-xl font-bold font-mono text-amber-400">{products_count}</div>
          <div class="text-[11px] text-gray-300">Products</div>
        </div>
        <div class="p-3 rounded-xl bg-black/40 border border-emerald-900/40">
          <div class="text-xl font-bold font-mono text-emerald-400">{orders_count}</div>
          <div class="text-[11px] text-gray-300">Buyer Orders</div>
        </div>
        <div class="p-3 rounded-xl bg-black/40 border border-emerald-900/40">
          <div class="text-xl font-bold font-mono text-teal-400">{suppliers_count}</div>
          <div class="text-[11px] text-gray-300">Suppliers</div>
        </div>
        <div class="p-3 rounded-xl bg-black/40 border border-emerald-900/40">
          <div class="text-xl font-bold font-mono text-blue-400">{invoices_count}</div>
          <div class="text-[11px] text-gray-300">Invoices</div>
        </div>
        <div class="p-3 rounded-xl bg-black/40 border border-emerald-900/40">
          <div class="text-xl font-bold font-mono text-indigo-400">{messages_count}</div>
          <div class="text-[11px] text-gray-300">Inquiries</div>
        </div>
      </div>
    </div>

    <div class="p-6 rounded-3xl bg-emerald-950/30 border border-emerald-800/40 space-y-4">
      <div class="flex items-center justify-between">
        <h2 class="text-base font-bold text-white">Endpoint Latency & Throughput Metrics</h2>
        <span class="text-xs font-mono text-gray-400">Auto-refreshes every 5s</span>
      </div>
      <div class="overflow-x-auto">
        <table class="w-full text-left">
          <thead class="border-b border-emerald-800/60 text-[10px] uppercase font-mono text-gray-400">
            <tr>
              <th class="p-3">Method</th>
              <th class="p-3">Endpoint Path</th>
              <th class="p-3 text-right">Hits</th>
              <th class="p-3 text-right">Avg Latency</th>
              <th class="p-3 text-right">2xx OK</th>
              <th class="p-3 text-right">5xx Error</th>
            </tr>
          </thead>
          <tbody>
            {endpoints_rows}
          </tbody>
        </table>
      </div>
    </div>
  </div>
  <script>
    setTimeout(() => location.reload(), 5000);
  </script>
</body>
</html>"##,
        total_requests = snapshot.total_requests,
        success_2xx = snapshot.status_2xx,
        client_err_4xx = snapshot.status_4xx,
        server_err_5xx = snapshot.status_5xx,
        avg_latency = snapshot.avg_latency_ms,
        uptime_secs = snapshot.uptime_seconds,
        start_time = snapshot.server_time,
        db_entities_sum = products_count + orders_count + suppliers_count + invoices_count + messages_count,
        products_count = products_count,
        orders_count = orders_count,
        suppliers_count = suppliers_count,
        invoices_count = invoices_count,
        messages_count = messages_count,
        endpoints_rows = endpoints_rows
    );

    Html(html)
}
