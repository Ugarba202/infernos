use serde::{Deserialize, Serialize};

/// Satoshis amount type wrapper
pub type Satoshis = u64;

/// Model identifier
pub type ModelId = String;

/// Session identifier
pub type SessionId = String;

/// 32-byte payment hash (hex-encoded string)
pub type PaymentHash = String;

/// 32-byte preimage (hex-encoded string)
pub type Preimage = String;

/// Token usage accounting
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
