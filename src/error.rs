use std::io;
use serde_json;
use rmp_serde;
use toml;
use trackable::error::{TrackableError, IntoTrackableError};
use trackable::error::{ErrorKind as TrackableErrorKind, ErrorKindExt};

/// The error type for this crate.
pub type Error = TrackableError<ErrorKind>;


/// A list of error kinds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    /// Invalid input.
    Invalid,

    /// Unknown error.
    Other,
}
impl TrackableErrorKind for ErrorKind {}
impl IntoTrackableError<io::Error> for ErrorKind {
    fn into_trackable_error(e: io::Error) -> Error {
        ErrorKind::Other.cause(e)
    }
}
impl IntoTrackableError<toml::de::Error> for ErrorKind {
    fn into_trackable_error(e: toml::de::Error) -> Error {
        ErrorKind::Invalid.cause(e)
    }
}
impl IntoTrackableError<toml::ser::Error> for ErrorKind {
    fn into_trackable_error(e: toml::ser::Error) -> Error {
        ErrorKind::Invalid.cause(e)
    }
}
impl IntoTrackableError<serde_json::Error> for ErrorKind {
    fn into_trackable_error(e: serde_json::Error) -> Error {
        ErrorKind::Invalid.cause(e)
    }
}
impl IntoTrackableError<rmp_serde::encode::Error> for ErrorKind {
    fn into_trackable_error(e: rmp_serde::encode::Error) -> Error {
        ErrorKind::Invalid.cause(e)
    }
}
impl IntoTrackableError<rmp_serde::decode::Error> for ErrorKind {
    fn into_trackable_error(e: rmp_serde::decode::Error) -> Error {
        ErrorKind::Invalid.cause(e)
    }
}
