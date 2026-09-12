use crate::client::error::ClientError;

pub struct InfernosClient {
    pub base_url: String,
    pub budget_sats: Option<u64>,
}

impl InfernosClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            budget_sats: None,
        }
    }

    pub fn with_budget(mut self, budget_sats: u64) -> Self {
        self.budget_sats = Some(budget_sats);
        self
    }

    pub async fn chat(&self, _prompt: &str) -> Result<String, ClientError> {
        Ok("Mock completion response".to_string())
    }
}
