use std::io::Write;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::error::Error;
use crate::format::Format;

/// Read one value. `to` is only used when building the error.
pub(crate) trait Decode {
    fn decode<T: DeserializeOwned>(&self, input: &[u8], to: Format) -> Result<T, Error>;
}

/// Write one value. `from` is only used when building the error.
pub(crate) trait Encode {
    fn encode<T: Serialize, W: Write>(
        &self,
        from: Format,
        value: &T,
        writer: W,
    ) -> Result<(), Error>;
}

/// A format that can sit on either side of a conversion.
pub(crate) trait Codec: Decode + Encode {}

impl<T: Decode + Encode> Codec for T {}
