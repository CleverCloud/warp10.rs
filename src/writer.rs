use reqwest::header::HeaderMap;

use crate::data::*;
use crate::error::*;
use crate::response::*;
use crate::token::*;

#[derive(Debug, Clone)]
pub struct Writer {
    http: reqwest::Client,
    update_url: url::Url,
    token: Token,
}

impl Writer {
    pub(crate) fn new(http: reqwest::Client, update_url: url::Url, token: Token) -> Self {
        Self {
            http,
            update_url,
            token,
        }
    }

    pub async fn post(&self, data: Vec<Data>) -> Result<UpdateResponse> {
        let body = Self::serialize_data(&data);

        let mut headers = HeaderMap::new();
        self.token.set_header(&mut headers);

        let response = self
            .http
            .post(self.update_url.as_str())
            .headers(headers)
            .header("content-type", "text/plain; charset=utf-8")
            .body(body)
            .send()
            .await?;

        let status = response.status();
        let err = Self::extract_header(response.headers(), "x-warp10-error-message");
        let payload = response.text().await?;

        if status.is_success() {
            Ok(UpdateResponse::new(status, payload))
        } else {
            Err(Error::Api {
                status,
                body: payload,
                message: err,
            })
        }
    }

    fn serialize_data(data: &[Data]) -> String {
        data.iter()
            .map(|d| d.warp10_serialize())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn extract_header(headers: &HeaderMap, name: &str) -> Option<String> {
        headers
            .get(name)
            .and_then(|v| v.to_str().ok())
            .map(String::from)
    }
}
