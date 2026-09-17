pub mod handlers;
pub mod routes;

use std::sync::Arc;
use crate::config::schema::NodeConfig;
use crate::node::lightning::backend::LightningBackend;
use crate::node::gate::{SessionBudgetManager, MacaroonService};
use crate::node::proxy::openai::OpenAiProxy;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<NodeConfig>,
    pub lightning: Arc<dyn LightningBackend>,
    pub budget_manager: Arc<SessionBudgetManager>,
    pub macaroon_service: Arc<MacaroonService>,
    pub proxy: OpenAiProxy,
}
