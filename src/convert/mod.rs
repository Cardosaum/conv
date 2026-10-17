use std::io::{Read, Write};

use crate::codec::Decode;
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

fn dispatch<W: Write>(from: Format, to: Format, input: &[u8], mut writer: W) -> Result<(), Error> {
    let bytes = match from {
        Format::Json => run::<json::Json>(to, input),
        Format::Yaml => run::<yaml::Yaml>(to, input),
        Format::Toml => run::<toml::Toml>(to, input),
        Format::Ron => run::<ron::Ron>(to, input),
        Format::Url => run::<url::Url>(to, input),
        Format::Cbor => run::<cbor::Cbor>(to, input),
        Format::Msgpack => run::<msgpack::Msgpack>(to, input),
    }?;
    writer.write_all(&bytes).map_err(Error::Write)
}

fn run<D: Decode>(to: Format, input: &[u8]) -> Result<Vec<u8>, Error> {
    match to {
        Format::Json => D::transcode::<json::Json>(input),
        Format::Yaml => D::transcode::<yaml::Yaml>(input),
        Format::Toml => D::transcode::<toml::Toml>(input),
        Format::Ron => D::transcode::<ron::Ron>(input),
        Format::Url => D::transcode::<url::Url>(input),
        Format::Cbor => D::transcode::<cbor::Cbor>(input),
        Format::Msgpack => D::transcode::<msgpack::Msgpack>(input),
    }
}

#[cfg(test)]
mod test;
