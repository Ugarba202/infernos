use crate::common::error::Result;
use crate::config::schema::NodeConfig;

pub struct InfernosServer {
    pub config: NodeConfig,
}

impl InfernosServer {
    pub fn new(config: NodeConfig) -> Self {
        Self { config }
    }

    pub async fn run(&self) -> Result<()> {
        tracing::info!(
            "Starting Infernos Node on {}:{}",
            self.config.host,
            self.config.port
        );
        Ok(())
    }
}
