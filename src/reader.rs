use isahc::http::header::{CONTENT_TYPE, HOST};
use isahc::{
    http::HeaderValue, AsyncBody, AsyncReadResponseExt, Body, ReadResponseExt, Request, RequestExt,
};

use crate::client::*;
use crate::error::*;
use crate::http_handler;
use crate::response::*;

#[derive(Debug)]
pub struct Reader<'a> {
    client: &'a Client,
    token: String,
}

impl<'a> Reader<'a> {
    pub fn new(client: &'a Client, token: String) -> Self {
        Self { client, token }
    }

    pub async fn get(&self, warp_script: &str) -> Result<Warp10Response> {
        let request = self.get_request::<AsyncBody>(warp_script)?;
        let mut response = request.send_async().await?;
        let status = response.status();
        let err = http_handler::extract_header_err(response.headers());
        let payload = response.text().await?;
        http_handler::handle_response(err, status, payload)
    }

    pub fn get_sync(&self, warp_script: &str) -> Result<Warp10Response> {
        let request = self.get_request::<Body>(warp_script)?;
        let mut response = request.send()?;
        let status = response.status();
        let err = http_handler::extract_header_err(response.headers());
        let payload = response.text()?;
        http_handler::handle_response(err, status, payload)
    }

    fn get_request<T: From<String>>(&self, warp_script: &'a str) -> Result<Request<T>> {
        let mut request = Request::post(self.client.exec_uri())
            .body(T::from(warp_script.replace("$TOKEN", &self.token)))?;
        request.headers_mut().insert(
            CONTENT_TYPE,
            HeaderValue::from_str("text/plain; charset=utf-8").expect("failed to parse mime type"),
        );
        request.headers_mut().insert(
            HOST,
            HeaderValue::from_str(&self.client.host_and_maybe_port())
                .unwrap_or_else(|_| HeaderValue::from_static("localhost")),
        );
        Ok(request)
    }
}
