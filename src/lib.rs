//! This crate provides convenient traits and functions
//! for converting between TOML/JSON/MessagePack strings and serializable values.
//!
//! This is highly depends on the [serde](https://github.com/serde-rs/serde) crate.
//!
//! # Examples
//!
//! Converts from a TOML string to a serializable value:
//!
//! ```
//! extern crate serde;
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate serdeconv;
//!
//! use serdeconv::FromToml;
//!
//! // Defines a deserializable struct.
//! #[derive(Deserialize)]
//! struct Foo {
//!     bar: String,
//!     baz: usize
//! }
//! impl FromToml for Foo {}
//!
//! # fn main() {
//! // Converts from the TOML string to a `Foo` value.
//! let toml = r#"
//! bar = "aaa"
//! baz = 123
//! "#;
//! let foo = Foo::from_toml_str(toml).unwrap();
//! assert_eq!(foo.bar, "aaa");
//! assert_eq!(foo.baz, 123);
//! # }
//! ```
#![warn(missing_docs)]
extern crate serde;
extern crate serde_json;
extern crate rmp_serde;
extern crate toml;
#[macro_use]
extern crate trackable;

pub use convert_json::{from_json_str, from_json_reader, from_json_file, from_json_slice};
pub use convert_json::{to_json_string, to_json_writer, to_json_file, to_json_string_pretty,
                       to_json_writer_pretty};
pub use convert_msgpack::{from_msgpack_slice, from_msgpack_reader, from_msgpack_file};
pub use convert_msgpack::{to_msgpack_vec, to_msgpack_writer, to_msgpack_file};
pub use convert_toml::{from_toml_str, from_toml_slice, from_toml_reader, from_toml_file};
pub use convert_toml::{to_toml_string, to_toml_writer, to_toml_file};
pub use error::{Error, ErrorKind};
pub use traits::{FromToml, ToToml, FromJson, ToJson, FromMsgPack, ToMsgPack};

mod convert_json;
mod convert_msgpack;
mod convert_toml;
mod error;
mod traits;

/// A specialized `Result` type for this crate.
pub type Result<T> = ::std::result::Result<T, Error>;
