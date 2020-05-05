use rmp_serde;
use serde_json;
use std::io;
use toml;
use trackable::error::TrackableError;
use trackable::error::{ErrorKind as TrackableErrorKind, ErrorKindExt};

/// The error type for this crate.
#[derive(Debug, Clone, TrackableError)]
pub struct Error(TrackableError<ErrorKind>);
impl From<io::Error> for Error {
    fn from(f: io::Error) -> Self {
        ErrorKind::Other.cause(f).into()
    }
}
impl From<toml::de::Error> for Error {
    fn from(f: toml::de::Error) -> Self {
        ErrorKind::Invalid.cause(f).into()
    }
}
impl From<toml::ser::Error> for Error {
    fn from(f: toml::ser::Error) -> Self {
        ErrorKind::Invalid.cause(f).into()
    }
}
impl From<serde_json::Error> for Error {
    fn from(f: serde_json::Error) -> Self {
        ErrorKind::Invalid.cause(f).into()
    }
}
impl From<rmp_serde::encode::Error> for Error {
    fn from(f: rmp_serde::encode::Error) -> Self {
        ErrorKind::Invalid.cause(f).into()
    }
}
impl From<rmp_serde::decode::Error> for Error {
    fn from(f: rmp_serde::decode::Error) -> Self {
        ErrorKind::Invalid.cause(f).into()
    }
}

/// A list of error kinds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    /// Invalid input.
    Invalid,

    /// Unknown error.
    Other,
}
impl TrackableErrorKind for ErrorKind {}
