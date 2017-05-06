use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};
use toml;

use Result;

pub trait FromToml: for<'a> Deserialize<'a> {
    fn from_toml_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let f = track_try!(File::open(path));
        track!(Self::from_toml_reader(f))
    }
    fn from_toml_reader<R: Read>(mut reader: R) -> Result<Self> {
        let mut toml = String::new();
        track_try!(reader.read_to_string(&mut toml));
        track!(Self::from_toml_str(&toml))
    }
    fn from_toml_str(toml: &str) -> Result<Self> {
        let this = track_try!(toml::from_str(toml));
        Ok(this)
    }
    fn from_toml(toml: toml::Value) -> Result<Self> {
        let this = track_try!(toml.try_into());
        Ok(this)
    }
}

pub trait ToToml: Serialize {
    fn to_toml_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let f = track_try!(File::create(path));
        track!(self.to_toml_writer(f))
    }
    fn to_toml_writer<W: Write>(&self, mut writer: W) -> Result<()> {
        let toml = track_try!(self.to_toml_string());
        track_try!(writer.write_all(toml.as_bytes()));
        Ok(())
    }
    fn to_toml_string(&self) -> Result<String> {
        let toml = track_try!(toml::to_string(self));
        Ok(toml)
    }
    fn to_toml(&self) -> Result<toml::Value> {
        let toml = track_try!(toml::Value::try_from(self));
        Ok(toml)
    }
}
