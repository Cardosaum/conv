use serde::de::Deserializer;

use crate::codec::Forward;
use crate::codec::{Decode, Encode, Fail, clear_output, emit, finish};
use crate::format::Format;

pub(crate) struct Yaml;

impl Decode for Yaml {
    fn transcode<E: Encode>(input: &[u8]) -> Result<Vec<u8>, crate::error::Error> {
        clear_output();
        // `from_slice` rejects a second document.
        finish::<E>(Format::Yaml, serde_saphyr::from_slice::<Forward<E>>(input))
    }
}

impl Encode for Yaml {
    const FORMAT: Format = Format::Yaml;

    fn accept<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, Fail<D::Error>> {
        // serde-saphyr writes to `fmt::Write`, then the bytes are copied out.
        let mut buf = String::new();
        let mut ser = serde_saphyr::Serializer::new(&mut buf);
        emit(deserializer, &mut ser)?;
        Ok(buf.into_bytes())
    }
}

#[cfg(test)]
mod test;
