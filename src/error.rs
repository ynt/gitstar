use std::io;
use reqwest;

#[derive(Debug)]
pub enum Error {
    GitstarErr(String),
    StdErr(io::Error),
    ParseErr(reqwest::UrlError),
    ReqwestErr(reqwest::Error),
}

impl Error {
    pub fn new(error: String) -> Error {
        Error::GitstarErr(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::StdErr(error)
    }
}

impl From<reqwest::UrlError> for Error {
    fn from(error: reqwest::UrlError) -> Self {
        Error::ParseErr(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::ReqwestErr(error)
    }
}
