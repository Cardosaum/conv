use serde::de::Deserializer;

use crate::codec::Forward;
use crate::codec::{Decode, Encode, Fail, clear_output, emit, finish};
use crate::format::Format;

pub(crate) struct Json;

impl Decode for Json {
    fn transcode<E: Encode>(input: &[u8]) -> Result<Vec<u8>, crate::error::Error> {
        clear_output();
        finish::<E>(Format::Json, serde_json::from_slice::<Forward<E>>(input))
    }
}

impl Encode for Json {
    const FORMAT: Format = Format::Json;

    fn accept<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, Fail<D::Error>> {
        let mut buf = Vec::new();
        let mut ser = serde_json::Serializer::new(&mut buf);
        emit(deserializer, &mut ser)?;
        Ok(buf)
    }
}
