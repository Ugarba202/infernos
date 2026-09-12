use reqwest::Client;

#[derive(Clone)]
pub struct OpenAiProxy {
    client: Client,
    upstream_url: String,
}

impl OpenAiProxy {
    pub fn new(upstream_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            upstream_url: upstream_url.into(),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn upstream_url(&self) -> &str {
        &self.upstream_url
    }
}
