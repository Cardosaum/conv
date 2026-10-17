use std::io::{Cursor, Write};

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::codec::{Decode, Encode};
use crate::error::{Convert, Error};
use crate::format::Format;

pub(crate) struct Cbor;

impl Decode for Cbor {
    fn decode<T: DeserializeOwned>(&self, input: &[u8], to: Format) -> Result<T, Error> {
        let mut cursor = Cursor::new(input);
        let value = ciborium::from_reader(&mut cursor).convert(Format::Cbor, to)?;
        if cursor.position() != input.len() as u64 {
            return Err(Error::Trailing {
                format: Format::Cbor,
            });
        }
        Ok(value)
    }
}

impl Encode for Cbor {
    fn encode<T: Serialize, W: Write>(
        &self,
        from: Format,
        value: &T,
        writer: W,
    ) -> Result<(), Error> {
        // `into_writer` maps unit to CBOR null, which is what JSON null needs.
        ciborium::into_writer(value, writer).convert(from, Format::Cbor)
    }
}
