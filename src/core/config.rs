use serde::{Deserialize, Serialize};

use crate::core::ProxyServer;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub address: String,
    pub port: i32,
    pub proxies: Vec<ProxyServer>,
    pub test_uris: Vec<String>,
    pub tolerance: Option<u128>,
}
