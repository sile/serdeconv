use rmp_serde;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use {Error, Result};

/// Converts from the MessagePack file to a value of `T` type.
pub fn from_msgpack_file<T, P>(path: P) -> Result<T>
where
    T: for<'a> Deserialize<'a>,
    P: AsRef<Path>,
{
    let f = track!(File::open(path).map_err(Error::from))?;
    track!(from_msgpack_reader(f))
}

/// Reads a MessagePack bytes from the reader and converts it to a value of `T` type.
pub fn from_msgpack_reader<T, R>(reader: R) -> Result<T>
where
    T: for<'a> Deserialize<'a>,
    R: Read,
{
    let value = track!(rmp_serde::decode::from_read(reader).map_err(Error::from))?;
    Ok(value)
}

/// Converts from the MessagePack bytes to a value of `T` type.
pub fn from_msgpack_slice<'a, T>(bytes: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let value = track!(rmp_serde::from_slice(bytes).map_err(Error::from))?;
    Ok(value)
}

/// Converts the value to a MessagePack bytes and writes it to the speficied file.
pub fn to_msgpack_file<T, P>(value: &T, path: P) -> Result<()>
where
    T: ?Sized + Serialize,
    P: AsRef<Path>,
{
    let f = track!(File::create(path).map_err(Error::from))?;
    track!(to_msgpack_writer(value, f))
}

/// Converts the value to a MessagePack bytes and writes it to the writer.
pub fn to_msgpack_writer<T, W>(value: &T, mut writer: W) -> Result<()>
where
    T: ?Sized + Serialize,
    W: Write,
{
    track!(rmp_serde::encode::write(&mut writer, value).map_err(Error::from,))?;
    Ok(())
}

/// Converts the value to a MessagePack bytes.
pub fn to_msgpack_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let bytes = track!(rmp_serde::encode::to_vec(value).map_err(Error::from))?;
    Ok(bytes)
}
