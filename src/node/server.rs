use crate::common::error::Result;
use crate::config::schema::NodeConfig;
use crate::node::api::routes::create_routes;
use crate::node::api::AppState;
use crate::node::gate::{SessionBudgetManager, MacaroonService};
use crate::node::lightning::backend::MockLightningBackend;
use crate::node::proxy::openai::OpenAiProxy;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

pub struct InfernosServer {
    pub config: NodeConfig,
}

impl InfernosServer {
    pub fn new(config: NodeConfig) -> Self {
        Self { config }
    }

    pub async fn run(&self) -> Result<()> {
        let addr = format!("{}:{}", self.config.server.host, self.config.server.port);
        tracing::info!("Starting Infernos Node on {}", addr);

        let proxy = OpenAiProxy::new(self.config.upstream.url.clone());
        
        let state = AppState {
            config: Arc::new(self.config.clone()),
            // Using MockLightningBackend by default to keep the implementation simple right now
            lightning: Arc::new(MockLightningBackend::new()),
            budget_manager: Arc::new(SessionBudgetManager::new()),
            macaroon_service: Arc::new(MacaroonService::new(b"test-secret-key-0000000000000000".to_vec(), "infernos-node")),
            proxy,
        };

        let app = create_routes(state)
            .layer(TraceLayer::new_for_http().make_span_with(
                tower_http::trace::DefaultMakeSpan::new().include_headers(false)
            ));

        let listener = tokio::net::TcpListener::bind(&addr).await
            .map_err(|e| crate::common::error::Error::Internal(e.to_string()))?;
            
        axum::serve(listener, app).await
            .map_err(|e| crate::common::error::Error::Internal(e.to_string()))?;

        Ok(())
    }
}
