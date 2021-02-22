use isahc::http::status::StatusCode;

#[derive(Debug)]
pub struct Warp10Response {
    status: StatusCode,
    payload: String,
}

impl Warp10Response {
    pub fn new(status: StatusCode, payload: String) -> Self {
        Self { status, payload }
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn payload(&self) -> &str {
        &self.payload
    }
}
