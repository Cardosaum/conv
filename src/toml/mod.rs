use serde::de::Deserializer;

use crate::codec::Forward;
use crate::codec::{Decode, Encode, Fail, clear_output, emit, finish, text};
use crate::error::Convert;
use crate::format::Format;

pub(crate) struct Toml;

impl Decode for Toml {
    fn transcode<E: Encode>(input: &[u8]) -> Result<Vec<u8>, crate::error::Error> {
        clear_output();
        let input = text(input, Format::Toml)?;
        let deserializer = toml::Deserializer::parse(input).convert(Format::Toml, E::FORMAT)?;
        finish::<E>(Format::Toml, Forward::<E>::pull(deserializer))
    }
}

impl Encode for Toml {
    const FORMAT: Format = Format::Toml;

    fn accept<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, Fail<D::Error>> {
        let mut buf = toml::ser::Buffer::new();
        emit(deserializer, toml::Serializer::new(&mut buf))?;
        Ok(buf.to_string().into_bytes())
    }
}

#[cfg(test)]
mod test;
