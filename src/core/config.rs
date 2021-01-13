use crate::core::ProxyServer;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub proxies: Vec<ProxyServer>,
    pub test_uris: Vec<String>,
}
