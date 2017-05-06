extern crate serde;
extern crate toml;
#[macro_use]
extern crate trackable;

pub use error::{Error, ErrorKind};
pub use traits::{FromToml, ToToml};

mod error;
mod traits;

/// A specialized `Result` type for this crate.
pub type Result<T> = ::std::result::Result<T, Error>;
