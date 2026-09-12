use crate::client::error::ClientError;

pub struct L402PaymentHandler;

impl L402PaymentHandler {
    pub async fn handle_challenge(_challenge_header: &str) -> Result<String, ClientError> {
        Ok("mock_l402_auth".to_string())
    }
}
