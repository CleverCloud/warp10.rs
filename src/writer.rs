use hyper::client;
use hyper::status::StatusCode;
use itertools::Itertools;

use client::*;
use data::*;
use error::*;
use response::*;
use token::*;

#[derive(Debug)]
pub struct Writer<'a> {
    client: &'a Client,
    token:  Token<'a>,
}

impl<'a> Writer<'a> {
    pub fn new(client: &'a Client, token: Token<'a>) -> Writer<'a> {
        Writer {
            client: client,
            token:  token
        }
    }

    pub fn post(&self, data: Vec<Data>) -> Result<Response> {
        let body     = data.iter().map(|d| d.warp10_serialize()).join("\n");
        let url      = try!(self.client.url().join("/api/v0/update"));
        let mut resp = try!(client::Client::new()
            .post(url)
            .headers(self.token.get_headers())
            .body(client::Body::BufBody(body.as_bytes(), body.len()))
            .send());
        let response = try!(Response::new(&mut resp));

        match response.status() {
            StatusCode::Ok => Ok(response),
            _              => Err(Error::api_error(response))
        }
    }
}
