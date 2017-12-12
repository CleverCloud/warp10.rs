use reqwest::header::{ContentType, Headers, Host};

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

    pub fn get_headers(&self) -> Headers {
        let mut headers = Headers::new();
        headers.set(ContentType::plaintext());
        headers.set(Host::new(
            self.client
                .url()
                .host_str()
                .unwrap_or("localhost")
                .to_string(),
            self.client.url().port(),
        ));
        headers.set_raw("X-Warp10-Token", vec![self.token.as_bytes().to_vec()]);
        headers
    }
}
