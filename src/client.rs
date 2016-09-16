use hyper::{client, Url};
use hyper::header::{Headers, ContentType, Host};
use itertools::Itertools;

use data::*;
use error::*;

#[derive(Debug)]
pub struct Client {
    url:   Url,
    token: String,
}

impl Client {
    pub fn new(url: String, token: String) -> Result<Client> {
        let real_url = try!(Url::parse(&url));
        Ok(Client {
            url:   real_url,
            token: token
        })
    }

    fn get_headers(&self) -> Headers {
        let mut headers = Headers::new();
        headers.set(ContentType::plaintext());
        headers.set(Host {
            hostname: self.url.host_str().unwrap_or("localhost").to_string(),
            port:     self.url.port()
        });
        headers.set_raw("X-Warp10-Token", vec![self.token.clone().into_bytes()]);
        headers
    }

    pub fn post(&self, data: Vec<Data>) -> Result<client::Response> {
        let body = data.iter().map(|d| d.warp10_serialize()).join("\n");
        let url = try!(self.url.join("/api/v0/update"));
        let resp = try!(client::Client::new()
            .post(url)
            .headers(self.get_headers())
            .body(client::Body::BufBody(body.as_bytes(), body.len()))
            .send());
        Ok(resp)
    }
}
