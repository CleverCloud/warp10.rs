use isahc::{http::status::StatusCode, prelude::*};

use client::*;
use data::*;
use error::*;
use response::*;
use token::*;

#[derive(Debug)]
pub struct Writer<'a> {
    client: &'a Client,
    token: Token<'a>,
}

impl<'a> Writer<'a> {
    pub fn new(client: &'a Client, token: Token<'a>) -> Self {
        Self { client, token }
    }

    pub fn post(&self, data: Vec<Data>) -> Result<Warp10Response> {
        let body = data
            .iter()
            .map(|d| d.warp10_serialize())
            .fold(String::new(), |acc, cur| {
                if acc.is_empty() {
                    cur
                } else {
                    (acc + "\n") + &cur
                }
            });

        let mut request = Request::post(self.client.update_uri()).body(Body::from(body))?;
        self.token.set_headers(request.headers_mut());
        let response = Warp10Response::new(&mut request.send()?)?;

        match response.status() {
            StatusCode::OK => Ok(response),
            _ => Err(Error::api_error(response)),
        }
    }
}
