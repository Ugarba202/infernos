use crate::common::error::Result;
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;
use futures_util::Stream;

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

    pub fn new_with_timeout(upstream_url: impl Into<String>, timeout: Duration) -> Self {
        Self {
            client: Client::builder().timeout(timeout).build().unwrap_or_default(),
            upstream_url: upstream_url.into(),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn upstream_url(&self) -> &str {
        &self.upstream_url
    }

    pub async fn forward_chat_completion(&self, _request: Value) -> Result<Value> {
        unimplemented!("Task 4.2 - Implement proxy forwarding")
    }

    pub async fn forward_chat_completion_with_headers(
        &self,
        _request: Value,
        _headers: http::HeaderMap,
    ) -> Result<Value> {
        unimplemented!("Task 4.2 - Implement proxy forwarding with headers")
    }

    pub async fn stream_chat_completion(
        &self,
        _request: Value,
    ) -> Result<impl Stream<Item = Result<bytes::Bytes>>> {
        unimplemented!("Task 4.2 - Implement proxy streaming");
        // Returning a dummy stream just to satisfy the type checker for unimplemented
        #[allow(unreachable_code)]
        Ok(futures_util::stream::empty())
    }
}
