use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};
use toml;

use Result;

/// Converts from the TOML file to a value of `T` type.
pub fn from_toml_file<T, P>(path: P) -> Result<T>
where
    T: for<'a> Deserialize<'a>,
    P: AsRef<Path>,
{
    let f = track_try!(File::open(path));
    track!(from_toml_reader(f))
}

/// Reads a TOML string from the reader and converts it to a value of `T` type.
pub fn from_toml_reader<T, R>(mut reader: R) -> Result<T>
where
    T: for<'a> Deserialize<'a>,
    R: Read,
{
    let mut toml = String::new();
    track_try!(reader.read_to_string(&mut toml));
    track!(from_toml_str(&toml))
}

/// Converts from the TOML string to a value of `T` type.
///
/// # Examples
///
/// ```
/// extern crate serde;
/// #[macro_use]
/// extern crate serde_derive;
/// extern crate serdeconv;
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
/// let foo: Foo = serdeconv::from_toml_str(toml).unwrap();
/// assert_eq!(foo.bar, "aaa");
/// assert_eq!(foo.baz, 123);
/// # }
/// ```
pub fn from_toml_str<'a, T>(toml: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let value = track_try!(toml::from_str(toml));
    Ok(value)
}

/// Converts from the TOML bytes to a value of `T` type.
pub fn from_toml_slice<'a, T>(toml: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let value = track_try!(toml::from_slice(toml));
    Ok(value)
}

/// Converts the value to a TOML string and writes it to the speficied file.
pub fn to_toml_file<T, P>(value: &T, path: P) -> Result<()>
where
    T: ?Sized + Serialize,
    P: AsRef<Path>,
{
    let f = track_try!(File::create(path));
    track!(to_toml_writer(value, f))
}

/// Converts the value to a TOML string and writes it to the writer.
pub fn to_toml_writer<T, W>(value: &T, mut writer: W) -> Result<()>
where
    T: ?Sized + Serialize,
    W: Write,
{
    let toml = track_try!(to_toml_string(value));
    track_try!(writer.write_all(toml.as_bytes()));
    Ok(())
}

/// Converts the value to a TOML string.
///
/// # Examples
///
/// ```
/// extern crate serde;
/// #[macro_use]
/// extern crate serde_derive;
/// extern crate serdeconv;
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
/// let toml = serdeconv::to_toml_string(&foo).unwrap();
/// assert_eq!(toml, "\
/// bar = \"aaa\"
/// baz = 123
/// ");
/// # }
/// ```
pub fn to_toml_string<T>(value: &T) -> Result<String>
where
    T: ?Sized + Serialize,
{
    let toml = track_try!(toml::to_string(value));
    Ok(toml)
}
