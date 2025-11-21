use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub http_bind_addr: String,
    pub http_port: u16,
    pub udp_bind_addr: String,
    pub udp_port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        // loads .env file if it exists
        let _ = dotenv();

        let http_bind_addr =
            env::var("HTTP_BIND_ADDR").unwrap_or_else(|_| "0.0.0.0".to_string());
        let http_port = env::var("HTTP_PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(8080);

        let udp_bind_addr =
            env::var("UDP_BIND_ADDR").unwrap_or_else(|_| "0.0.0.0".to_string());
        let udp_port = env::var("UDP_PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(6000);

        AppConfig {
            http_bind_addr,
            http_port,
            udp_bind_addr,
            udp_port,
        }
    }
}
