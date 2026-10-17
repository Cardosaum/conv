use std::io::Write;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::codec::{Decode, Encode};
use crate::error::{Convert, Error};
use crate::format::Format;

pub(crate) struct Json;

impl Decode for Json {
    fn decode<T: DeserializeOwned>(&self, input: &[u8], to: Format) -> Result<T, Error> {
        serde_json::from_slice(input).convert(Format::Json, to)
    }
}

impl Encode for Json {
    fn encode<T: Serialize, W: Write>(
        &self,
        from: Format,
        value: &T,
        writer: W,
    ) -> Result<(), Error> {
        serde_json::to_writer(writer, value).convert(from, Format::Json)
    }
}
