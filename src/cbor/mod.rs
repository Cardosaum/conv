use std::io::Cursor;

use serde::de::Deserializer;

use crate::codec::Forward;
use crate::codec::{Decode, Encode, Fail, capture, clear_output, finish, reject};
use crate::error::Error;
use crate::format::Format;

pub(crate) struct Cbor;

impl Decode for Cbor {
    fn transcode<E: Encode>(input: &[u8]) -> Result<Vec<u8>, Error> {
        clear_output();
        let mut cursor = Cursor::new(input);
        let bytes = finish::<E>(
            Format::Cbor,
            ciborium::from_reader::<Forward<E>, _>(&mut cursor),
        )?;
        if cursor.position() != input.len() as u64 {
            return Err(Error::Trailing {
                format: Format::Cbor,
            });
        }
        Ok(bytes)
    }
}

impl Encode for Cbor {
    const FORMAT: Format = Format::Cbor;

    fn accept<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, Fail<D::Error>> {
        let event = capture(deserializer)?;
        let mut buf = Vec::new();
        // `into_writer` maps unit to CBOR null, which is what JSON null needs.
        ciborium::into_writer(&event, &mut buf).map_err(reject)?;
        Ok(buf)
    }
}
