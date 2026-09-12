use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub host: String,
    pub port: u16,
    pub upstream_url: String,
    pub default_price_sats: u64,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            upstream_url: "http://127.0.0.1:11434".to_string(),
            default_price_sats: 10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub node_url: String,
    pub max_budget_sats: Option<u64>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            node_url: "http://127.0.0.1:8080".to_string(),
            max_budget_sats: Some(1000),
        }
    }
}
