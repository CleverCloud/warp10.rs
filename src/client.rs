use hyper::Url;

use error::*;
use token::*;
use writer::*;

#[derive(Debug)]
pub struct Client {
    pub url: Url
}

impl Client {
    pub fn new(url: &str) -> Result<Client> {
        let real_url = try!(Url::parse(url));
        Ok(Client {
            url: real_url
        })
    }

    pub fn get_writer(&self, token: String) -> Writer {
        Writer::new(self, Token::new(self, token))
    }
}
