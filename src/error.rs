use std::{error, fmt, result};

#[derive(Debug, Clone)]
pub struct ExecErrorDetail {
    pub message: String,
    pub line: Option<String>,
    pub position: Option<String>,
}

impl fmt::Display for ExecErrorDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(ref line) = self.line {
            write!(f, " (line {})", line)?;
        }
        if let Some(ref pos) = self.position {
            write!(f, " (position {})", pos)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    Api {
        status: reqwest::StatusCode,
        body: String,
        message: Option<String>,
    },
    Exec(ExecErrorDetail),
    Http(reqwest::Error),
    Url(url::ParseError),
    Json(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Api {
                status,
                message: Some(msg),
                ..
            } => write!(f, "Warp10 API error ({}): {}", status, msg),
            Error::Api { status, body, .. } => {
                write!(f, "Warp10 API error ({}): {}", status, body)
            }
            Error::Exec(detail) => write!(f, "Warp10 exec error: {}", detail),
            Error::Http(err) => write!(f, "HTTP error: {}", err),
            Error::Url(err) => write!(f, "URL error: {}", err),
            Error::Json(err) => write!(f, "JSON error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Http(err) => Some(err),
            Error::Url(err) => Some(err),
            Error::Json(err) => Some(err),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Http(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Error::Url(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

pub type Result<T> = result::Result<T, Error>;
