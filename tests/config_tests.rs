use infernos::common::types::Satoshis;
use infernos::config::schema::LightningBackendType;
use infernos::config::{ClientConfig, NodeConfig};

#[test]
fn node_config_deserializes_from_structured_toml() {
    let toml = r#"
        [server]
        host = "192.168.1.50"
        port = 9000

        [upstream]
        url = "http://remote-gpu:11434"

        [pricing]
        default_price_sats = 25

        [lightning]
        backend = "lnd"
        lnd_rpc_host = "10.0.0.2:10009"
    "#;

    let config: NodeConfig =
        toml::from_str(toml).expect("structured node config should deserialize");

    assert_eq!(config.server.host, "192.168.1.50");
    assert_eq!(config.server.port, 9000);
    assert_eq!(config.upstream.url, "http://remote-gpu:11434");
    assert_eq!(config.pricing.default_price_sats, Satoshis(25));
    assert_eq!(config.lightning.backend, LightningBackendType::Lnd);
    assert_eq!(
        config.lightning.lnd_rpc_host.as_deref(),
        Some("10.0.0.2:10009")
    );
}

#[test]
fn node_config_partial_override_falls_back_to_defaults() {
    // When an operator only customizes pricing, all other sections must seamlessly use defaults
    let toml = r#"
        [pricing]
        default_price_sats = 100
    "#;

    let config: NodeConfig = toml::from_str(toml).expect("partial node config should deserialize");

    assert_eq!(config.pricing.default_price_sats, Satoshis(100));
    assert_eq!(config.server.host, "0.0.0.0");
    assert_eq!(config.server.port, 8080);
    assert_eq!(config.upstream.url, "http://127.0.0.1:11434");
    assert_eq!(config.lightning.backend, LightningBackendType::Mock);
}

#[test]
fn client_config_deserializes_from_structured_toml() {
    let toml = r#"
        [client]
        node_url = "https://node.infernos.ai"

        [budget]
        max_budget_sats = 5000

        [lightning]
        payment_backend = "nwc"
    "#;

    let config: ClientConfig =
        toml::from_str(toml).expect("structured client config should deserialize");

    assert_eq!(config.client.node_url, "https://node.infernos.ai");
    assert_eq!(config.budget.max_budget_sats, Some(Satoshis(5000)));
    assert_eq!(config.lightning.payment_backend, LightningBackendType::Nwc);
}

#[test]
fn client_config_partial_override_falls_back_to_defaults() {
    let toml = r#"
        [budget]
        max_budget_sats = 250
    "#;

    let config: ClientConfig =
        toml::from_str(toml).expect("partial client config should deserialize");

    assert_eq!(config.client.node_url, "http://127.0.0.1:8080");
    assert_eq!(config.budget.max_budget_sats, Some(Satoshis(250)));
    assert_eq!(config.lightning.payment_backend, LightningBackendType::Mock);
}

#[test]
fn lightning_backend_type_rejects_invalid_variant() {
    let toml = r#"
        [lightning]
        backend = "paypal"
    "#;

    let result: Result<NodeConfig, _> = toml::from_str(toml);
    assert!(result.is_err(), "unknown backend rail should fail to parse");
}
