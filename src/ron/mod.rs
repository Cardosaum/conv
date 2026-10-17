use serde::de::Deserializer;

use crate::codec::Forward;
use crate::codec::{Decode, Encode, Fail, clear_output, emit, finish, reject, text};
use crate::format::Format;

pub(crate) struct Ron;

impl Decode for Ron {
    fn transcode<E: Encode>(input: &[u8]) -> Result<Vec<u8>, crate::error::Error> {
        clear_output();
        let input = text(input, Format::Ron)?;
        finish::<E>(Format::Ron, ron::from_str::<Forward<E>>(input))
    }
}

impl Encode for Ron {
    const FORMAT: Format = Format::Ron;

    fn accept<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, Fail<D::Error>> {
        let mut buf = String::new();
        let mut ser = ron::Serializer::new(&mut buf, None).map_err(reject)?;
        emit(deserializer, &mut ser)?;
        Ok(buf.into_bytes())
    }
}

#[cfg(test)]
mod test;
