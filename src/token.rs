use mime;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, HOST};

use client::*;

#[derive(Debug)]
pub struct Token<'a> {
    client: &'a Client,
    token:  String,
}

impl<'a> Token<'a> {
    pub fn new(client: &Client, token: String) -> Token {
        Token {
            client: client,
            token:  token,
        }
    }

    pub fn get_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_str(mime::TEXT_PLAIN_UTF_8.as_ref()).expect("failed to parse mime type"));
        headers.insert(HOST, HeaderValue::from_str(&self.client.host_and_maybe_port()).unwrap_or_else(|_| HeaderValue::from_static("localhost")));
        if let Ok(token) = HeaderValue::from_str(&self.token) {
            headers.insert(HeaderName::from_static("x-warp10-token"), token);
        }
        headers
    }
}
