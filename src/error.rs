use std::{error, fmt::Display};
use tokio_tungstenite::tungstenite;

#[derive(Debug)]
pub enum Error {
    Db(rusqlite::Error),
    Ws(tungstenite::Error),
    Tls(native_tls::Error),
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Db(e) => e.fmt(f),
            Error::Ws(e) => e.fmt(f),
            Error::Tls(e) => e.fmt(f),
        }
    }
}

impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self {
        Error::Db(e)
    }
}

impl From<tungstenite::Error> for Error {
    fn from(e: tungstenite::Error) -> Self {
        Error::Ws(e)
    }
}

impl From<native_tls::Error> for Error {
    fn from(e: native_tls::Error) -> Self {
        Error::Tls(e)
    }
}
