use infernos::config::{ClientConfig, NodeConfig};

#[test]
fn node_config_deserializes_from_toml() {
    let toml = r#"
        host = "127.0.0.1"
        port = 9000
        upstream_url = "http://localhost:11434"
        default_price_sats = 25
    "#;

    let config: NodeConfig = toml::from_str(toml).expect("node config should deserialize");

    assert_eq!(config.host, "127.0.0.1");
    assert_eq!(config.port, 9000);
    assert_eq!(config.upstream_url, "http://localhost:11434");
    assert_eq!(config.default_price_sats, infernos::common::types::Satoshis(25));
}

#[test]
fn client_config_deserializes_from_toml() {
    let toml = r#"
        node_url = "http://192.168.1.10:8080"
        max_budget_sats = 5000
    "#;

    let config: ClientConfig = toml::from_str(toml).expect("client config should deserialize");

    assert_eq!(config.node_url, "http://192.168.1.10:8080");
    assert_eq!(config.max_budget_sats, Some(infernos::common::types::Satoshis(5000)));
}

#[test]
fn node_config_defaults_are_applied() {
    let config = NodeConfig::default();

    assert_eq!(config.host, "0.0.0.0");
    assert_eq!(config.port, 8080);
    assert_eq!(config.upstream_url, "http://127.0.0.1:11434");
    assert_eq!(config.default_price_sats, infernos::common::types::Satoshis(10));
}

#[test]
fn client_config_defaults_are_applied() {
    let config = ClientConfig::default();

    assert_eq!(config.node_url, "http://127.0.0.1:8080");
    assert_eq!(config.max_budget_sats, Some(infernos::common::types::Satoshis(1000)));
}
