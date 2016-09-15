use hyper::client::{Body, Client, Response};
use hyper::header::{Headers, ContentType, Host};
use hyper::Url;
use itertools::Itertools;

use data::*;
use error::*;

#[derive(Debug)]
pub struct Warp10Client<'a> {
    url:   Url,
    token: &'a str
}

impl<'a> Warp10Client<'a> {
    pub fn new(url: Url, token: &'a str) -> Warp10Client<'a> {
        Warp10Client {
            url:   url,
            token: token
        }
    }

    fn get_headers(&self) -> Headers {
        let mut headers = Headers::new();
        headers.set(ContentType::plaintext());
        headers.set(Host {
            hostname: self.url.host_str().unwrap_or("localhost").to_string(),
            port:     self.url.port()
        });
        headers.set_raw("X-Warp10-Token", vec![self.token.as_bytes().to_vec()]);
        headers
    }

    pub fn post(&self, data: Vec<Warp10Data>) -> Result<Response> {
        let body = data.iter().map(|d| d.warp10_serialize()).join("\n");
        let url = try!(self.url.join("/api/v0/update"));
        let resp = try!(Client::new()
            .post(url)
            .headers(self.get_headers())
            .body(Body::BufBody(body.as_bytes(), body.len()))
            .send());
        Ok(resp)
    }
}
