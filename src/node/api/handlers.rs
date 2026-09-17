use super::AppState;
use crate::common::error::Error;
use crate::common::types::{Satoshis, SessionId};
use crate::node::gate::{Caveat, L402Challenge, L402Verifier};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn health_check() -> impl IntoResponse {
    Json(json!({ "status": "ok", "service": "infernos-node" }))
}

pub async fn models(State(_state): State<AppState>) -> impl IntoResponse {
    let data = vec![json!({
        "id": "llama3.2",
        "object": "model",
        "created": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        "owned_by": "infernos-node"
    })];
    Json(json!({ "object": "list", "data": data }))
}

#[derive(Deserialize)]
pub struct NewSessionRequest {
    pub budget_sats: u64,
}

pub async fn new_session(
    State(state): State<AppState>,
    Json(payload): Json<NewSessionRequest>,
) -> Result<impl IntoResponse, Error> {
    let budget = Satoshis(payload.budget_sats);
    let invoice = state
        .lightning
        .create_invoice(budget, "Infernos Session Budget")
        .await
        .map_err(|e| Error::Internal(e.to_string()))?;

    let uuid = uuid::Uuid::new_v4();
    let session_id = SessionId(uuid);

    // Register session in the budget manager
    state
        .budget_manager
        .open_session(&session_id, budget)
        .await;

    // Mint macaroon with session caveats
    let caveats = vec![
        Caveat::Session(uuid.to_string()),
    ];
    let macaroon = state
        .macaroon_service
        .mint(&invoice.payment_hash, caveats)
        .map_err(|e| Error::Internal(e.to_string()))?;

    let challenge = L402Challenge::from_components(&macaroon, &invoice)
        .map_err(|e| Error::Internal(e.to_string()))?;

    // Respond with 402 Payment Required and the WWW-Authenticate challenge header
    let mut headers = HeaderMap::new();
    headers.insert(
        "WWW-Authenticate",
        challenge.to_header_value().parse().unwrap(),
    );

    Ok((StatusCode::PAYMENT_REQUIRED, headers, Json(json!({"status": "payment_required"}))))
}

pub async fn chat_completions(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, Error> {
    let auth_header = headers.get("Authorization");
    if auth_header.is_none() {
        return Err(Error::PaymentRequired {
            invoice: "".to_string(),
            token: "".to_string(),
        });
    }

    let auth_str = auth_header.unwrap().to_str().unwrap_or("");
    
    // Default model requirement if we extract it, but let's pass None for now to avoid complexity in this step
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    let _credentials = L402Verifier::verify_header(
        &state.macaroon_service,
        &*state.lightning,
        auth_str,
        None,
        now,
    )
    .await
    .map_err(|_| Error::VerificationFailed("Invalid L402 credentials".to_string()))?;

    // Proxy the request
    let proxy_resp = state
        .proxy
        .forward_chat_completion_with_headers(payload, headers)
        .await?;

    Ok(Json(proxy_resp))
}

// Error Mapping for Axum
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match &self {
            Error::PaymentRequired { invoice, token } => {
                let challenge = format!("L402 macaroon=\"{}\", invoice=\"{}\"", token, invoice);
                let mut headers = HeaderMap::new();
                headers.insert("WWW-Authenticate", challenge.parse().unwrap());
                return (StatusCode::PAYMENT_REQUIRED, headers, Json(json!({"error": self.to_string()}))).into_response();
            },
            Error::VerificationFailed(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            Error::BudgetExhausted(_) => (StatusCode::PAYMENT_REQUIRED, self.to_string()), // Or 403
            Error::Upstream(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
            Error::Config(_) | Error::Lightning(_) | Error::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, Json(json!({"error": err_msg}))).into_response()
    }
}
