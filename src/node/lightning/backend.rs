use async_trait::async_trait;
use crate::common::error::Result;

#[async_trait]
pub trait LightningBackend: Send + Sync {
    async fn create_invoice(&self, amount_sats: u64, memo: &str) -> Result<(String, String)>;
    async fn is_invoice_settled(&self, payment_hash: &str) -> Result<bool>;
}
