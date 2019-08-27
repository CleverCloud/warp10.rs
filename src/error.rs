extern crate reqwest;
extern crate url;

use response::*;

use std::{error, fmt, io, result};

#[derive(Debug)]
pub enum Error {
    ApiError(Response),
    HttpError(reqwest::Error),
    HttpUrlError(reqwest::UrlError),
    IoError(io::Error),
    UrlError(url::ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ApiError(ref resp) => write!(f, "Warp10 API error: {:?}", resp),
            Error::HttpError(ref err) => write!(f, "Warp10 HTTP error: {}", err),
            Error::HttpUrlError(ref err) => write!(f, "Warp10 HTTP URL error: {}", err),
            Error::IoError(ref err)   => write!(f, "Warp10 IO error: {}", err),
            Error::UrlError(ref err)  => write!(f, "Warp10 URL error: {}",  err),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::ApiError(_)           => None,
            Error::HttpError(ref err)    => Some(err),
            Error::HttpUrlError(ref err) => Some(err),
            Error::IoError(ref err)      => Some(err),
            Error::UrlError(ref err)     => Some(err),
        }
    }
}

impl Error {
    pub fn api_error(response: Response) -> Error {
        Error::ApiError(response)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::HttpError(err)
    }
}

impl From<reqwest::UrlError> for Error {
    fn from(err: reqwest::UrlError) -> Error {
        Error::HttpUrlError(err)
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

pub type Result<T> = result::Result<T, Error>;
