use std::io::Cursor;

use serde::de::Deserializer;

use crate::codec::Forward;
use crate::codec::{Decode, Encode, Fail, clear_output, emit, finish};
use crate::error::Error;
use crate::format::Format;

pub(crate) struct Msgpack;

impl Decode for Msgpack {
    fn transcode<E: Encode>(input: &[u8]) -> Result<Vec<u8>, Error> {
        clear_output();
        let mut cursor = Cursor::new(input);
        let mut deserializer = rmp_serde::Deserializer::new(&mut cursor);
        let bytes = finish::<E>(Format::Msgpack, Forward::<E>::pull(&mut deserializer))?;
        if cursor.position() != input.len() as u64 {
            return Err(Error::Trailing {
                format: Format::Msgpack,
            });
        }
        Ok(bytes)
    }
}

impl Encode for Msgpack {
    const FORMAT: Format = Format::Msgpack;

    fn accept<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, Fail<D::Error>> {
        let mut ser = rmp_serde::Serializer::new(Vec::new());
        emit(deserializer, &mut ser)?;
        Ok(ser.into_inner())
    }
}
