use std::io::{Read, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};

use Result;

/// This trait allows to convert TOML objects to deserializable values.
///
/// # Examples
///
/// ```
/// extern crate serde;
/// #[macro_use]
/// extern crate serde_derive;
/// extern crate serdeconv;
///
/// use serdeconv::FromToml;
///
/// // Defines a deserializable struct.
/// #[derive(Deserialize)]
/// struct Foo {
///     bar: String,
///     baz: usize
/// }
/// impl FromToml for Foo {}
///
/// # fn main() {
/// // Converts from the TOML string to a `Foo` value.
/// let toml = r#"
/// bar = "aaa"
/// baz = 123
/// "#;
/// let foo = Foo::from_toml_str(toml).unwrap();
/// assert_eq!(foo.bar, "aaa");
/// assert_eq!(foo.baz, 123);
/// # }
/// ```
pub trait FromToml: for<'a> Deserialize<'a> {
    /// Converts from the TOML file to an instance of this implementation.
    fn from_toml_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        track!(::from_toml_file(path))
    }

    /// Reads a TOML string from the reader and converts it to an instance of this implementation.
    fn from_toml_reader<R: Read>(reader: R) -> Result<Self> {
        track!(::from_toml_reader(reader))
    }

    /// Converts from the TOML string to an instance of this implementation.
    fn from_toml_str(toml: &str) -> Result<Self> {
        track!(::from_toml_str(toml))
    }

    /// Converts from the TOML bytes to an instance of this implementation.
    fn from_toml_slice(toml: &[u8]) -> Result<Self> {
        track!(::from_toml_slice(toml))
    }
}

/// This trait allows to convert serializable values to TOML objects.
///
/// # Examples
///
/// ```
/// extern crate serde;
/// #[macro_use]
/// extern crate serde_derive;
/// extern crate serdeconv;
///
/// use serdeconv::ToToml;
///
/// // Defines a serializable struct.
/// #[derive(Serialize)]
/// struct Foo {
///     bar: &'static str,
///     baz: usize
/// }
/// impl ToToml for Foo {}
///
/// # fn main() {
/// // Converts the `Foo` value to a TOML string.
/// let foo = Foo { bar: "aaa", baz: 123 };
/// let toml = foo.to_toml_string().unwrap();
/// assert_eq!(toml, "\
/// bar = \"aaa\"
/// baz = 123
/// ");
/// # }
/// ```
pub trait ToToml: Serialize {
    /// Converts this to a TOML string and writes it to the speficied file.
    fn to_toml_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        track!(::to_toml_file(self, path))
    }

    /// Converts this to a TOML string and writes it to the writer.
    fn to_toml_writer<W: Write>(&self, writer: W) -> Result<()> {
        track!(::to_toml_writer(self, writer))
    }

    /// Converts this to a TOML string.
    fn to_toml_string(&self) -> Result<String> {
        track!(::to_toml_string(self))
    }
}

/// This trait allows to convert JSON objects to deserializable values.
///
/// # Examples
///
/// ```
/// extern crate serde;
/// #[macro_use]
/// extern crate serde_derive;
/// extern crate serdeconv;
///
/// use serdeconv::FromJson;
///
/// // Defines a deserializable struct.
/// #[derive(Deserialize)]
/// struct Foo {
///     bar: String,
///     baz: usize
/// }
/// impl FromJson for Foo {}
///
/// # fn main() {
/// // Converts from the JSON string to a `Foo` value.
/// let json = r#"{
///     "bar": "aaa",
///     "baz": 123
/// }"#;
/// let foo = Foo::from_json_str(json).unwrap();
/// assert_eq!(foo.bar, "aaa");
/// assert_eq!(foo.baz, 123);
/// # }
/// ```
pub trait FromJson: for<'a> Deserialize<'a> {
    /// Converts from the JSON file to an instance of this implementation.
    fn from_json_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        track!(::from_json_file(path))
    }

    /// Reads a JSON string from the reader and converts it to an instance of this implementation.
    fn from_json_reader<R: Read>(reader: R) -> Result<Self> {
        track!(::from_json_reader(reader))
    }

    /// Converts from the JSON string to an instance of this implementation.
    fn from_json_str(json: &str) -> Result<Self> {
        track!(::from_json_str(json))
    }

    /// Converts from the JSON bytes to an instance of this implementation.
    fn from_json_slice(json: &[u8]) -> Result<Self> {
        track!(::from_json_slice(json))
    }
}

/// This trait allows to convert serializable values to JSON objects.
///
/// # Examples
///
/// ```
/// extern crate serde;
/// #[macro_use]
/// extern crate serde_derive;
/// extern crate serdeconv;
///
/// use serdeconv::ToJson;
///
/// // Defines a serializable struct.
/// #[derive(Serialize)]
/// struct Foo {
///     bar: &'static str,
///     baz: usize
/// }
/// impl ToJson for Foo {}
///
/// # fn main() {
/// // Converts the `Foo` value to a JSON string.
/// let foo = Foo { bar: "aaa", baz: 123 };
/// let json = foo.to_json_string().unwrap();
/// assert_eq!(json, r#"{"bar":"aaa","baz":123}"#);
/// # }
/// ```
pub trait ToJson: Serialize {
    /// Converts this to a JSON string and writes it to the speficied file.
    fn to_json_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        track!(::to_json_file(self, path))
    }

    /// Converts this to a JSON string and writes it to the writer.
    fn to_json_writer<W: Write>(&self, writer: W) -> Result<()> {
        track!(::to_json_writer(self, writer))
    }

    /// Converts this to a pretty printed JSON string and writes it to the writer.
    fn to_json_writer_pretty<W: Write>(&self, writer: W) -> Result<()> {
        track!(::to_json_writer_pretty(self, writer))
    }

    /// Converts this to a JSON string.
    fn to_json_string(&self) -> Result<String> {
        track!(::to_json_string(self))
    }

    /// Converts this to a pretty printed JSON string.
    fn to_json_string_pretty(&self) -> Result<String> {
        track!(::to_json_string_pretty(self))
    }
}
