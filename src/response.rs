use error::*;

use hyper::client;
use hyper::status::StatusCode;

use std::io::Read;

#[derive(Debug)]
pub struct Response {
    status:  StatusCode,
    payload: String
}

impl Response {
    pub fn new(response: &mut client::Response) -> Result<Response> {
        let mut payload = String::new();

        response.read_to_string(&mut payload)?;

        Ok(Response {
            status:  response.status,
            payload: payload
        })
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn payload(&self) -> &str {
        &self.payload
    }
}
