use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080);

        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        Self { host, port }
    }
}

impl ServerConfig {
    pub fn socket_addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("Valid socket address")
    }
}
