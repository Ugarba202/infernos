use crate::common::types::Satoshis;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LightningBackendType {
    #[default]
    Mock,
    Lnd,
    Nwc,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamConfig {
    pub url: String,
}

impl Default for UpstreamConfig {
    fn default() -> Self {
        Self {
            url: "http://127.0.0.1:11434".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PricingConfig {
    pub default_price_sats: Satoshis,
}

impl Default for PricingConfig {
    fn default() -> Self {
        Self {
            default_price_sats: Satoshis(10),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LightningConfig {
    pub backend: LightningBackendType,
    pub lnd_rpc_host: Option<String>,
    pub lnd_macaroon_path: Option<String>,
    pub lnd_tls_cert_path: Option<String>,
    pub nwc_uri: Option<String>,
}

impl Default for LightningConfig {
    fn default() -> Self {
        Self {
            backend: LightningBackendType::Mock,
            lnd_rpc_host: None,
            lnd_macaroon_path: None,
            lnd_tls_cert_path: None,
            nwc_uri: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct NodeConfig {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub upstream: UpstreamConfig,
    #[serde(default)]
    pub pricing: PricingConfig,
    #[serde(default)]
    pub lightning: LightningConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientEndpointConfig {
    pub node_url: String,
}

impl Default for ClientEndpointConfig {
    fn default() -> Self {
        Self {
            node_url: "http://127.0.0.1:8080".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetConfig {
    pub max_budget_sats: Option<Satoshis>,
}

impl Default for BudgetConfig {
    fn default() -> Self {
        Self {
            max_budget_sats: Some(Satoshis(1000)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientPaymentConfig {
    pub payment_backend: LightningBackendType,
}

impl Default for ClientPaymentConfig {
    fn default() -> Self {
        Self {
            payment_backend: LightningBackendType::Mock,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ClientConfig {
    #[serde(default)]
    pub client: ClientEndpointConfig,
    #[serde(default)]
    pub budget: BudgetConfig,
    #[serde(default)]
    pub lightning: ClientPaymentConfig,
}
