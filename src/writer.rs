use hyper::client;
use itertools::Itertools;

use client::*;
use data::*;
use error::*;

#[derive(Debug)]
pub struct Writer<'a> {
    client: &'a Client,
    token:  String,
}

impl<'a> Writer<'a> {
    pub fn new(client: &Client, token: String) -> Writer {
        Writer {
            client: client,
            token:  token
        }
    }

    pub fn post(&self, data: Vec<Data>) -> Result<client::Response> {
        let body = data.iter().map(|d| d.warp10_serialize()).join("\n");
        let url = try!(self.client.url.join("/api/v0/update"));
        let resp = try!(client::Client::new()
            .post(url)
            .headers(self.client.get_headers(&self.token))
            .body(client::Body::BufBody(body.as_bytes(), body.len()))
            .send());
        Ok(resp)
    }
}
