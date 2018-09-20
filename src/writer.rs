use reqwest::{self, StatusCode};

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
            token:  token,
        }
    }

    pub fn post(&self, data: Vec<Data>) -> Result<Response> {
        let body     = data.iter()
            .map(|d| d.warp10_serialize())
            .fold(String::new(), |acc, cur| {
                if acc.is_empty() {
                    cur
                } else {
                    (acc + "\n") + &cur
                }
            });
        let response = Response::new(&mut reqwest::Client::new()
            .post(self.client.url().join("/api/v0/update")?)
            .headers(self.token.get_headers())
            .body(reqwest::Body::from(body))
            .send()?)?;

        match response.status() {
            StatusCode::OK => Ok(response),
            _              => Err(Error::api_error(response))
        }
    }
}
