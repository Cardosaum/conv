use std::io::Write;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::codec::{Decode, Encode};
use crate::error::{Convert, Error};
use crate::format::Format;

pub(crate) struct Yaml;

impl Decode for Yaml {
    fn decode<T: DeserializeOwned>(&self, input: &[u8], to: Format) -> Result<T, Error> {
        // `from_slice` rejects a second document.
        serde_saphyr::from_slice(input).convert(Format::Yaml, to)
    }
}

impl Encode for Yaml {
    fn encode<T: Serialize, W: Write>(
        &self,
        from: Format,
        value: &T,
        mut writer: W,
    ) -> Result<(), Error> {
        // serde-saphyr writes to `fmt::Write`, then the bytes are copied out.
        let mut buf = String::new();
        let mut ser = serde_saphyr::Serializer::new(&mut buf);
        value.serialize(&mut ser).convert(from, Format::Yaml)?;
        writer.write_all(buf.as_bytes()).map_err(Error::Write)
    }
}

#[cfg(test)]
mod test;
