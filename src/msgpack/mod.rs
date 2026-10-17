use std::io::{Cursor, Write};

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::codec::{Decode, Encode};
use crate::error::{Convert, Error};
use crate::format::Format;

pub(crate) struct Msgpack;

impl Decode for Msgpack {
    fn decode<T: DeserializeOwned>(&self, input: &[u8], to: Format) -> Result<T, Error> {
        let mut cursor = Cursor::new(input);
        let mut de = rmp_serde::Deserializer::new(&mut cursor);
        let value = T::deserialize(&mut de).convert(Format::Msgpack, to)?;
        if cursor.position() != input.len() as u64 {
            return Err(Error::Trailing {
                format: Format::Msgpack,
            });
        }
        Ok(value)
    }
}

impl Encode for Msgpack {
    fn encode<T: Serialize, W: Write>(
        &self,
        from: Format,
        value: &T,
        writer: W,
    ) -> Result<(), Error> {
        let mut ser = rmp_serde::Serializer::new(writer);
        value.serialize(&mut ser).convert(from, Format::Msgpack)
    }
}
