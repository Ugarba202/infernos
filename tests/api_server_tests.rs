use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use infernos::node::api::routes::create_routes;
use infernos::node::api::AppState;
use infernos::config::schema::{NodeConfig, PricingConfig};
use infernos::node::gate::{SessionBudgetManager, MacaroonService};
use infernos::node::proxy::openai::OpenAiProxy;
use serde_json::json;
use std::sync::Arc;
use tower::ServiceExt; // for `oneshot`

use infernos::node::lightning::backend::MockLightningBackend;

fn setup_app() -> axum::Router {
    let config = NodeConfig {
        server: infernos::config::schema::ServerConfig { host: "127.0.0.1".to_string(), port: 8080 },
        pricing: PricingConfig { default_price_sats: infernos::common::types::Satoshis(10) },
        upstream: infernos::config::schema::UpstreamConfig {
            url: "http://localhost:8080".to_string(),
        },
        lightning: infernos::config::schema::LightningConfig::default(),
    };

    let proxy = OpenAiProxy::new(config.upstream.url.clone());
    
    let state = AppState {
        config: Arc::new(config),
        lightning: Arc::new(MockLightningBackend::new()),
        budget_manager: Arc::new(SessionBudgetManager::new()),
        macaroon_service: Arc::new(MacaroonService::new(b"test-secret-key-0000000000000000".to_vec(), "infernos-node")),
        proxy,
    };
    
    create_routes(state)
}

#[tokio::test]
async fn test_health_endpoint() {
    let app = setup_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_models_endpoint() {
    let app = setup_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/models")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_chat_completions_requires_payment() {
    let app = setup_app();

    let req_body = json!({
        "model": "llama3.2",
        "messages": [{"role": "user", "content": "Hello"}]
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/chat/completions")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&req_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // A missing or invalid Authorization header should trigger a 402 Payment Required
    assert_eq!(response.status(), StatusCode::PAYMENT_REQUIRED);
    
    // It must return a WWW-Authenticate header containing the L402 challenge
    let auth_header = response.headers().get("WWW-Authenticate");
    assert!(auth_header.is_some());
    let auth_str = auth_header.unwrap().to_str().unwrap();
    assert!(auth_str.starts_with("L402 macaroon="));
}

#[tokio::test]
async fn test_new_session_endpoint_returns_challenge() {
    let app = setup_app();

    let req_body = json!({
        "budget_sats": 1000
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/session/new")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&req_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // The endpoint should initially return a 402 challenge with the required invoice for the budget
    assert_eq!(response.status(), StatusCode::PAYMENT_REQUIRED);
    
    let auth_header = response.headers().get("WWW-Authenticate");
    assert!(auth_header.is_some());
}
