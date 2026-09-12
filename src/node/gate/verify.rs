use crate::common::error::Result;

pub struct L402Verifier;

impl L402Verifier {
    pub fn verify_token_and_preimage(_macaroon: &str, _preimage: &str) -> Result<bool> {
        Ok(true)
    }
}
