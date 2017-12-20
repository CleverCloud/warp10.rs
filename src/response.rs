use error::*;

use reqwest::{self, StatusCode};

#[derive(Debug)]
pub struct Response {
    status:  StatusCode,
    payload: String,
}

impl Response {
    pub fn new(response: &mut reqwest::Response) -> Result<Response> {
        Ok(Response {
            status:  response.status().clone(),
            payload: response.text()?,
        })
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn payload(&self) -> &str {
        &self.payload
    }
}
