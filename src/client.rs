use reqwest::Url;

use error::*;
use token::*;
use writer::*;

#[derive(Debug)]
pub struct Client {
    url: Url
}

impl Client {
    pub fn new(url: &str) -> Result<Client> {
        Ok(Client {
            url: Url::parse(url)?
        })
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn get_writer(&self, token: String) -> Writer {
        Writer::new(self, Token::new(self, token))
    }
}
