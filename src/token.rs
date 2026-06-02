use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

#[derive(Debug, Clone)]
pub struct Token(String);

impl Token {
    pub fn new(token: String) -> Self {
        Self(token)
    }

    pub fn set_header(&self, headers: &mut HeaderMap) {
        if let Ok(value) = HeaderValue::from_str(&self.0) {
            headers.insert(HeaderName::from_static("x-warp10-token"), value);
        }
    }
}
