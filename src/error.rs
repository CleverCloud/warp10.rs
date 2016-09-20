extern crate hyper;
extern crate url;

use response::*;

use std::{error, fmt, io, result};

#[derive(Debug)]
pub enum Error {
    ApiError(Response),
    HttpError(hyper::Error),
    IoError(io::Error),
    UrlError(url::ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ApiError(ref resp) => write!(f, "Warp10 API error: {:?}", resp),
            Error::HttpError(ref err) => write!(f, "Warp10 HTTP error: {}", err),
            Error::IoError(ref err)   => write!(f, "Warp10 IO error: {}", err),
            Error::UrlError(ref err)  => write!(f, "Warp10 URL error: {}",  err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ApiError(ref resp) => resp.payload(),
            Error::HttpError(ref err) => err.description(),
            Error::IoError(ref err)   => err.description(),
            Error::UrlError(ref err)  => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::ApiError(_)        => None,
            Error::HttpError(ref err) => Some(err),
            Error::IoError(ref err)   => Some(err),
            Error::UrlError(ref err)  => Some(err),
        }
    }
}

impl Error {
    pub fn api_error(response: Response) -> Error {
        Error::ApiError(response)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::HttpError(err)
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
