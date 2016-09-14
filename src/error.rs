extern crate hyper;
extern crate url;

use std::{error, fmt, result};

#[derive(Debug)]
pub enum Warp10Error {
    HttpError(hyper::Error),
    UrlError(url::ParseError),
}

impl fmt::Display for Warp10Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Warp10Error::HttpError(ref err) => write!(f, "Warp10 HTTP error: {}", err),
            Warp10Error::UrlError(ref err)  => write!(f, "Warp10 URL error: {}",  err),
        }
    }
}

impl error::Error for Warp10Error {
    fn description(&self) -> &str {
        match *self {
            Warp10Error::HttpError(ref err) => err.description(),
            Warp10Error::UrlError(ref err)  => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Warp10Error::HttpError(ref err) => Some(err),
            Warp10Error::UrlError(ref err)  => Some(err),
        }
    }
}

impl From<hyper::Error> for Warp10Error {
    fn from(err: hyper::Error) -> Warp10Error {
        Warp10Error::HttpError(err)
    }
}

impl From<url::ParseError> for Warp10Error {
    fn from(err: url::ParseError) -> Warp10Error {
        Warp10Error::UrlError(err)
    }
}

pub type Result<T> = result::Result<T, Warp10Error>;
