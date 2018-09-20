use reqwest::Url;

use error::*;
use token::*;
use writer::*;

#[derive(Debug)]
pub struct Client {
    url: Url,
}

impl Client {
    pub fn new(url: &str) -> Result<Client> {
        Ok(Client {
            url: Url::parse(url)?,
        })
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn host_and_maybe_port(&self) -> String {
        let host = self.url.host_str().unwrap_or("localhost");

        self.url.port().map(|port| format!("{}:{}", host, port)).unwrap_or_else(|| host.to_string())
    }

    pub fn get_writer(&self, token: String) -> Writer {
        Writer::new(self, Token::new(self, token))
    }
}
