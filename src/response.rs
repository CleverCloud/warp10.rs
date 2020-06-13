use error::*;

use isahc::{http::status::StatusCode, ResponseExt};

#[derive(Debug)]
pub struct Warp10Response {
    status: StatusCode,
    payload: String,
}

impl Warp10Response {
    pub fn new(response: &mut isahc::http::Response<isahc::Body>) -> Result<Self> {
        Ok(Self {
            status: response.status().clone(),
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
