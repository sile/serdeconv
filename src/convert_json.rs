use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json;

use Result;

/// Converts from the JSON file to a value of `T` type.
pub fn from_json_file<T, P>(path: P) -> Result<T>
    where T: for<'a> Deserialize<'a>,
          P: AsRef<Path>
{
    let f = track_try!(File::open(path));
    track!(from_json_reader(f))
}

/// Reads a JSON string from the reader and converts it to a value of `T` type.
pub fn from_json_reader<T, R>(reader: R) -> Result<T>
    where T: for<'a> Deserialize<'a>,
          R: Read
{
    let value = track_try!(serde_json::from_reader(reader));
    Ok(value)
}

/// Converts from the JSON string to a value of `T` type.
pub fn from_json_str<'a, T>(json: &'a str) -> Result<T>
    where T: Deserialize<'a>
{
    let value = track_try!(serde_json::from_str(json));
    Ok(value)
}

/// Converts from the JSON bytes to a value of `T` type.
pub fn from_json_slice<'a, T>(json: &'a [u8]) -> Result<T>
    where T: Deserialize<'a>
{
    let value = track_try!(serde_json::from_slice(json));
    Ok(value)
}

/// Converts the value to a JSON string and writes it to the speficied file.
pub fn to_json_file<T, P>(value: &T, path: P) -> Result<()>
    where T: ?Sized + Serialize,
          P: AsRef<Path>
{
    let f = track_try!(File::create(path));
    track!(to_json_writer(value, f))
}

/// Converts the value to a JSON string and writes it to the writer.
pub fn to_json_writer<T, W>(value: &T, writer: W) -> Result<()>
    where T: ?Sized + Serialize,
          W: Write
{
    let json = track_try!(serde_json::to_writer(writer, value));
    Ok(json)
}

/// Converts the value to a pretty printed JSON string and writes it to the writer.
pub fn to_json_writer_pretty<T, W>(value: &T, writer: W) -> Result<()>
    where T: ?Sized + Serialize,
          W: Write
{
    let json = track_try!(serde_json::to_writer_pretty(writer, value));
    Ok(json)
}

/// Converts the value to a JSON string.
pub fn to_json_string<T>(value: &T) -> Result<String>
    where T: ?Sized + Serialize
{
    let json = track_try!(serde_json::to_string(value));
    Ok(json)
}

/// Converts the value to a pretty printed JSON string.
pub fn to_json_string_pretty<T>(value: &T) -> Result<String>
    where T: ?Sized + Serialize
{
    let json = track_try!(serde_json::to_string_pretty(value));
    Ok(json)
}
