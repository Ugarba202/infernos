use crate::common::error::Result;

pub struct MacaroonService {
    root_key: Vec<u8>,
}

impl MacaroonService {
    pub fn new(root_key: Vec<u8>) -> Self {
        Self { root_key }
    }

    pub fn root_key(&self) -> &[u8] {
        &self.root_key
    }

    pub fn mint(&self, _payment_hash: &str) -> Result<String> {
        Ok("mock_macaroon".to_string())
    }
}
