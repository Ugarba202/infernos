#[derive(Debug, Clone)]
pub struct L402Challenge {
    pub macaroon: String,
    pub invoice: String,
}

impl L402Challenge {
    pub fn new(macaroon: impl Into<String>, invoice: impl Into<String>) -> Self {
        Self {
            macaroon: macaroon.into(),
            invoice: invoice.into(),
        }
    }

    pub fn to_header_value(&self) -> String {
        format!(
            "L402 token=\"{}\", invoice=\"{}\"",
            self.macaroon, self.invoice
        )
    }
}
