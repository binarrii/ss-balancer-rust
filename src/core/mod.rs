use std::cell::Cell;
use std::sync::{Mutex, MutexGuard};

use serde::Serialize;

pub mod estimator;

#[derive(Serialize)]
pub struct ProxyServer {
    #[serde(skip_serializing)]
    pub scheme: &'static str,
    pub name: &'static str,
    pub host: &'static str,
    pub port: i32,
    pub latency: Mutex<Cell<u128>>,
}

impl ProxyServer {

    pub fn format(&self) -> String {
        format!("{}://{}:{}", self.scheme, self.host, self.port)
    }

    pub fn get_latency(&self) -> u128 {
        self.latency.lock().unwrap().get()
    }

    pub fn latency_guard(&self) -> MutexGuard<Cell<u128>> {
        self.latency.lock().unwrap()
    }

}

impl Default for ProxyServer {
    fn default() -> Self {
        ProxyServer {
            scheme: "socks5h",
            name: "localhost",
            host: "127.0.0.1",
            port: 1080,
            latency: Mutex::new(Cell::new(0)),
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
