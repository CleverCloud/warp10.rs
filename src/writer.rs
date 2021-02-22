use isahc::{http::status::StatusCode, Body, Request, Response, RequestExt};

use crate::client::*;
use crate::data::*;
use crate::error::*;
use crate::response::*;
use crate::token::*;

#[derive(Debug)]
pub struct Writer<'a> {
    client: &'a Client,
    token: Token<'a>,
}

impl<'a> Writer<'a> {
    pub fn new(client: &'a Client, token: Token<'a>) -> Self {
        Self { client, token }
    }

    pub async fn post(&self, data: Vec<Data>) -> Result<Warp10Response> {
        let request = self.post_request(data)?;
        let response = request.send_async().await?;
        self.handle_response(response)
    }

    pub fn post_sync(&self, data: Vec<Data>) -> Result<Warp10Response> {
        let request = self.post_request(data)?;
        let response = request.send()?;
        self.handle_response(response)
    }

    fn post_request(&self, data: Vec<Data>) -> Result<Request<Body>> {
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
        Ok(request)
    }

    fn handle_response(&self, mut response: Response<Body>) -> Result<Warp10Response> {
        let response = Warp10Response::new(&mut response)?;
        match response.status() {
            StatusCode::OK => Ok(response),
            _ => Err(Error::api_error(response)),
        }
    }
}
