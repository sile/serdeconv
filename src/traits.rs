use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};
use toml;

use Result;

/// This trait allows to convert TOML objects to deserializable values.
///
/// # Examples
///
/// ```
/// extern crate serde;
/// #[macro_use]
/// extern crate serde_derive;
/// extern crate tomlconv;
///
/// use tomlconv::FromToml;
///
/// // Defines a deserializable struct.
/// #[derive(Deserialize)]
/// struct Foo {
///     bar: String,
///     baz: usize
/// }
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
        let f = track_try!(File::open(path));
        track!(Self::from_toml_reader(f))
    }

    /// Reads a TOML string from the reader and converts it to an instance of this implementation.
    fn from_toml_reader<R: Read>(mut reader: R) -> Result<Self> {
        let mut toml = String::new();
        track_try!(reader.read_to_string(&mut toml));
        track!(Self::from_toml_str(&toml))
    }

    /// Converts from the TOML string to an instance of this implementation.
    fn from_toml_str(toml: &str) -> Result<Self> {
        let this = track_try!(toml::from_str(toml));
        Ok(this)
    }

    /// Converts from the TOML value to an instance of this implementation.
    fn from_toml(toml: toml::Value) -> Result<Self> {
        let this = track_try!(toml.try_into());
        Ok(this)
    }
}
impl<T> FromToml for T where T: for<'a> Deserialize<'a> {}

/// This trait allows to convert serializable values to TOML objects.
///
/// # Examples
///
/// ```
/// extern crate serde;
/// #[macro_use]
/// extern crate serde_derive;
/// extern crate tomlconv;
///
/// use tomlconv::ToToml;
///
/// // Defines a serializable struct.
/// #[derive(Serialize)]
/// struct Foo {
///     bar: &'static str,
///     baz: usize
/// }
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
        let f = track_try!(File::create(path));
        track!(self.to_toml_writer(f))
    }

    /// Converts this to a TOML string and writes it to the writer.
    fn to_toml_writer<W: Write>(&self, mut writer: W) -> Result<()> {
        let toml = track_try!(self.to_toml_string());
        track_try!(writer.write_all(toml.as_bytes()));
        Ok(())
    }

    /// Converts this to a TOML string.
    fn to_toml_string(&self) -> Result<String> {
        let toml = track_try!(toml::to_string(self));
        Ok(toml)
    }

    /// Converts this to a TOML value.
    fn to_toml(&self) -> Result<toml::Value> {
        let toml = track_try!(toml::Value::try_from(self));
        Ok(toml)
    }
}
impl<T> ToToml for T where T: Serialize {}
