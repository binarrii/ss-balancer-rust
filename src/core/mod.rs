use std::sync::Mutex;

use serde::{Deserialize, Serialize};

pub mod estimator;
pub mod config;

#[derive(Serialize, Deserialize)]
pub struct ProxyServer {
    #[serde(skip_serializing)]
    pub scheme: String,
    pub name: String,
    pub host: String,
    pub port: i32,
    #[serde(skip_deserializing)]
    pub latency: Mutex<u128>,
}

impl ProxyServer {
    pub fn format(&self) -> String {
        format!("{}://{}:{}", self.scheme, self.host, self.port)
    }

    pub fn get_latency(&self) -> u128 {
        *self.latency.lock().unwrap()
    }

    pub fn set_latency(&self, value: u128) {
        *self.latency.lock().unwrap() = value
    }
}

impl Default for ProxyServer {
    fn default() -> Self {
        ProxyServer {
            scheme: String::from("socks5h"),
            name: String::from("localhost"),
            host: String::from("127.0.0.1"),
            port: 1080,
            latency: Mutex::new(0),
        }
    }
}

impl Clone for ProxyServer {
    fn clone(&self) -> Self {
        ProxyServer {
            scheme: self.scheme.clone(),
            name: self.name.clone(),
            host: self.host.clone(),
            port: self.port.clone(),
            ..Default::default()
        }
    }
}
