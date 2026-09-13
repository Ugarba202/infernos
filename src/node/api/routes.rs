use super::handlers;
use axum::routing::get;
use axum::Router;

pub fn create_routes() -> Router {
    Router::new().route("/health", get(handlers::health_check))
}
