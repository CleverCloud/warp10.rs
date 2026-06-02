use reqwest::StatusCode;

#[derive(Debug)]
pub struct UpdateResponse {
    status: StatusCode,
    body: String,
}

impl UpdateResponse {
    pub fn new(status: StatusCode, body: String) -> Self {
        Self { status, body }
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn body(&self) -> &str {
        &self.body
    }
}

#[derive(Debug, Clone)]
pub struct ExecMeta {
    pub elapsed: Option<String>,
    pub ops: Option<String>,
    pub fetched: Option<String>,
}

#[derive(Debug)]
pub struct ExecResponse {
    pub body: serde_json::Value,
    pub meta: ExecMeta,
}

#[derive(Debug)]
pub struct FindResponse(pub Vec<String>);

impl FindResponse {
    pub fn series(&self) -> &[String] {
        &self.0
    }
}
