use isahc::{AsyncBody, AsyncReadResponseExt, Body, ReadResponseExt, Request, RequestExt};

use crate::client::*;
use crate::data::*;
use crate::error::*;
use crate::http_handler;
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
        let request = self.post_request::<AsyncBody>(data)?;
        let mut response = request.send_async().await?;
        let status = response.status();
        let err = http_handler::extract_header_err(&response.headers());
        let payload = response.text().await?;
        http_handler::handle_response(err, status, payload)
    }

    pub fn post_sync(&self, data: Vec<Data>) -> Result<Warp10Response> {
        let request = self.post_request::<Body>(data)?;
        let mut response = request.send()?;
        let status = response.status();
        let err = http_handler::extract_header_err(&response.headers());
        let payload = response.text()?;
        http_handler::handle_response(err, status, payload)
    }

    fn post_request<T: From<String>>(&self, data: Vec<Data>) -> Result<Request<T>> {
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

        let mut request = Request::post(self.client.update_uri()).body(T::from(body))?;
        self.token.set_headers(request.headers_mut());
        Ok(request)
    }
}
