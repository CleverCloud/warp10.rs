use hyper::client::{Body, Client, Response};
use hyper::header::{Headers, ContentType, Host};
use hyper::Url;
use itertools::Itertools;

use data::*;
use error::*;

#[derive(Debug)]
pub struct Warp10Client {
    url:   Url,
    token: String,
}

impl Warp10Client {
    pub fn new(url: String, token: String) -> Result<Warp10Client> {
        let real_url = try!(Url::parse(&url));
        Ok(Warp10Client {
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
