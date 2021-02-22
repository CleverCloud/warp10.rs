use isahc::http::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, HOST};

use crate::client::*;

#[derive(Debug)]
pub struct Token<'a> {
    client: &'a Client,
    token: String,
}

impl<'a> Token<'a> {
    pub fn new(client: &'a Client, token: String) -> Self {
        Self { client, token }
    }

    pub fn set_headers(&self, headers: &mut HeaderMap) {
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str("text/plain; charset=utf-8").expect("failed to parse mime type"),
        );
        headers.insert(
            HOST,
            HeaderValue::from_str(&self.client.host_and_maybe_port())
                .unwrap_or_else(|_| HeaderValue::from_static("localhost")),
        );
        if let Ok(token) = HeaderValue::from_str(&self.token) {
            headers.insert(HeaderName::from_static("x-warp10-token"), token);
        }
    }
}
