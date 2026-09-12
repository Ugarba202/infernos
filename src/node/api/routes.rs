use axum::routing::get;
use axum::Router;
use super::handlers;

pub fn create_routes() -> Router {
    Router::new().route("/health", get(handlers::health_check))
}
