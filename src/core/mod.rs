use std::cell::Cell;
use std::sync::Mutex;

use serde::Serialize;

pub mod estimator;

#[derive(Serialize)]
pub struct ProxyServer {
    pub scheme: &'static str,
    pub name: &'static str,
    pub host: &'static str,
    pub port: i32,
    pub rating: Mutex<Cell<u128>>,
}

impl ProxyServer {
    pub fn format(&self) -> String {
        format!("{}://{}:{}", self.scheme, self.host, self.port)
    }
}

impl Default for ProxyServer {
    fn default() -> Self {
        ProxyServer {
            scheme: "socks5h",
            name: "localhost",
            host: "127.0.0.1",
            port: 1080,
            rating: Mutex::new(Cell::new(0)),
        }
    }
}

impl Clone for ProxyServer {
    fn clone(&self) -> Self {
        ProxyServer {
            scheme: self.scheme,
            name: self.name,
            host: self.host,
            port: self.port,
            ..Default::default()
        }
    }
}
