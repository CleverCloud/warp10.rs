extern crate hyper;

use std::{error, fmt, result};

#[derive(Debug)]
pub enum Warp10Error {
    HttpError(hyper::Error)
}

impl fmt::Display for Warp10Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Warp10Error::HttpError(ref err) => write!(f, "Warp10 error: {}", err)
        }
    }
}

impl error::Error for Warp10Error {
    fn description(&self) -> &str {
        match *self {
            Warp10Error::HttpError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Warp10Error::HttpError(ref err) => Some(err)
        }
    }
}

impl From<hyper::Error> for Warp10Error {
    fn from(err: hyper::Error) -> Warp10Error {
        Warp10Error::HttpError(err)
    }
}

pub type Result<T> = result::Result<T, Warp10Error>;
