use std::io::Write;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::codec::{Decode, Encode};
use crate::error::{Convert, Error};
use crate::format::Format;

pub(crate) struct Toml;

impl Decode for Toml {
    fn decode<T: DeserializeOwned>(&self, input: &[u8], to: Format) -> Result<T, Error> {
        let text = std::str::from_utf8(input).map_err(|err| Error::Utf8 {
            format: Format::Toml,
            source: err,
        })?;
        let de = toml::Deserializer::parse(text).convert(Format::Toml, to)?;
        T::deserialize(de).convert(Format::Toml, to)
    }
}

impl Encode for Toml {
    fn encode<T: Serialize, W: Write>(
        &self,
        from: Format,
        value: &T,
        writer: W,
    ) -> Result<(), Error> {
        // ponytail: table and null rules walk a JSON tree, so this clones into one.
        let value = serde_json::to_value(value).convert(from, Format::Toml)?;
        write_table(from, &value, writer)
    }
}

fn write_table(
    from: Format,
    value: &serde_json::Value,
    mut writer: impl Write,
) -> Result<(), Error> {
    if !value.is_object() {
        return Err(Error::reason(from, Format::Toml, "root must be a table"));
    }
    if let Some(path) = null_at(value) {
        return Err(Error::reason(from, Format::Toml, format!("null at {path}")));
    }
    let mut buf = toml::ser::Buffer::new();
    value
        .serialize(toml::Serializer::new(&mut buf))
        .convert(from, Format::Toml)?;
    writer
        .write_all(buf.to_string().as_bytes())
        .map_err(Error::Write)
}

fn null_at(value: &serde_json::Value) -> Option<String> {
    fn walk(value: &serde_json::Value, path: &mut String) -> bool {
        match value {
            serde_json::Value::Null => true,
            serde_json::Value::Array(items) => {
                let mark = path.len();
                for (index, item) in items.iter().enumerate() {
                    path.push('[');
                    path.push_str(&index.to_string());
                    path.push(']');
                    if walk(item, path) {
                        return true;
                    }
                    path.truncate(mark);
                }
                false
            }
            serde_json::Value::Object(map) => {
                let mark = path.len();
                for (key, item) in map {
                    if mark != 0 {
                        path.push('.');
                    }
                    path.push_str(key);
                    if walk(item, path) {
                        return true;
                    }
                    path.truncate(mark);
                }
                false
            }
            _ => false,
        }
    }

    let mut path = String::new();
    walk(value, &mut path).then_some(path)
}

#[cfg(test)]
mod test;
