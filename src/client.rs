use hyper::header::{Headers, ContentType, Host};
use hyper::Url;

use error::*;
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
        Writer::new(self, token)
    }

    pub fn get_headers(&self, token: &str) -> Headers {
        let mut headers = Headers::new();
        headers.set(ContentType::plaintext());
        headers.set(Host {
            hostname: self.url.host_str().unwrap_or("localhost").to_string(),
            port:     self.url.port()
        });
        headers.set_raw("X-Warp10-Token", vec![token.as_bytes().to_vec()]);
        headers
    }
}
