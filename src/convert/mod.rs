use std::io::{Read, Write};

use crate::codec::{Codec, Encode};
use crate::error::Error;
use crate::format::Format;
use crate::{cbor, json, msgpack, ron, toml, url, yaml};

/// Read one value in `from` and write it as `to`.
///
/// On failure, `writer` is left untouched.
///
/// # Errors
///
/// Returns an error when the input cannot be read, is not valid `from`,
/// contains trailing data, or cannot be encoded as `to`.
pub fn convert(
    from: Format,
    to: Format,
    mut reader: impl Read,
    mut writer: impl Write,
) -> Result<(), Error> {
    // ponytail: the whole input and output are buffered so a trailing-data
    // error produces no bytes. Stream once a format can prove exhaustion
    // before the first output byte.
    let mut input = Vec::new();
    reader.read_to_end(&mut input).map_err(Error::Read)?;

    let mut staged = Vec::new();
    dispatch(from, to, &input, &mut staged)?;
    if to.is_text() && !staged.ends_with(b"\n") {
        staged.push(b'\n');
    }
    writer.write_all(&staged).map_err(Error::Write)?;
    Ok(())
}

fn dispatch<W: Write>(from: Format, to: Format, input: &[u8], writer: W) -> Result<(), Error> {
    match from {
        Format::Json => pipe(&json::Json, from, to, input, writer),
        Format::Yaml => pipe(&yaml::Yaml, from, to, input, writer),
        Format::Toml => pipe(&toml::Toml, from, to, input, writer),
        Format::Ron => pipe(&ron::Ron, from, to, input, writer),
        Format::Url => pipe(&url::Url, from, to, input, writer),
        Format::Cbor => pipe(&cbor::Cbor, from, to, input, writer),
        Format::Msgpack => pipe(&msgpack::Msgpack, from, to, input, writer),
    }
}

fn pipe<D, W>(decoder: &D, from: Format, to: Format, input: &[u8], writer: W) -> Result<(), Error>
where
    D: Codec,
    W: Write,
{
    let value: serde_json::Value = decoder.decode(input, to)?;
    match to {
        Format::Json => json::Json.encode(from, &value, writer),
        Format::Yaml => yaml::Yaml.encode(from, &value, writer),
        Format::Toml => toml::Toml.encode(from, &value, writer),
        Format::Ron => ron::Ron.encode(from, &value, writer),
        Format::Url => url::Url.encode(from, &value, writer),
        Format::Cbor => cbor::Cbor.encode(from, &value, writer),
        Format::Msgpack => msgpack::Msgpack.encode(from, &value, writer),
    }
}

#[cfg(test)]
mod test;
