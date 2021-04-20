use crate::response::*;

use isahc::http::uri::InvalidUri;
use std::{error, fmt, io, result};

#[derive(Debug)]
pub enum Error {
    ApiError(Warp10Response, Option<String>),
    HttpError(isahc::http::Error),
    HttpUriError(isahc::http::uri::InvalidUri),
    HttpBodyError(isahc::Error),
    IoError(io::Error),
    UrlError(url::ParseError),
    #[cfg(feature = "json")]
    JsonError(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::ApiError(ref resp, Some(ref err)) => {
                write!(f, "Warp10 API error: {:?}, {}", resp, err)
            }
            Error::ApiError(ref resp, None) => write!(f, "Warp10 API error: {:?}", resp),
            Error::HttpError(ref err) => write!(f, "Warp10 HTTP error: {}", err),
            Error::HttpUriError(ref err) => write!(f, "Warp10 HTTP URI error: {}", err),
            Error::HttpBodyError(ref err) => write!(f, "Warp10 HTTP body error: {}", err),
            Error::IoError(ref err) => write!(f, "Warp10 IO error: {}", err),
            Error::UrlError(ref err) => write!(f, "Warp10 URL error: {}", err),
            #[cfg(feature = "json")]
            Error::JsonError(ref err) => write!(f, "Serialization error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::ApiError(_, _) => None,
            Error::HttpError(ref err) => Some(err),
            Error::HttpUriError(ref err) => Some(err),
            Error::HttpBodyError(ref err) => Some(err),
            Error::IoError(ref err) => Some(err),
            Error::UrlError(ref err) => Some(err),
            #[cfg(feature = "json")]
            Error::JsonError(ref err) => Some(err),
        }
    }
}

impl Error {
    pub fn api_error(response: Warp10Response, err: Option<String>) -> Error {
        Error::ApiError(response, err)
    }
}

impl From<isahc::http::Error> for Error {
    fn from(err: isahc::http::Error) -> Error {
        Error::HttpError(err)
    }
}

impl From<isahc::Error> for Error {
    fn from(err: isahc::Error) -> Error {
        Error::HttpBodyError(err)
    }
}

impl From<InvalidUri> for Error {
    fn from(err: InvalidUri) -> Error {
        Error::HttpUriError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::UrlError(err)
    }
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonError(err)
    }
}

pub type Result<T> = result::Result<T, Error>;
