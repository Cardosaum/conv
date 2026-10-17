mod transcode;

use serde::de::Deserializer;

pub(crate) use transcode::{Fail, Forward, capture, clear_output, emit, finish, reject};

use crate::error::Error;
use crate::format::Format;

pub(crate) trait Decode {
    fn transcode<E: Encode>(input: &[u8]) -> Result<Vec<u8>, Error>;
}

pub(crate) trait Encode {
    const FORMAT: Format;

    fn accept<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, Fail<D::Error>>;
}

pub(crate) fn text(input: &[u8], format: Format) -> Result<&str, Error> {
    std::str::from_utf8(input).map_err(|err| Error::Utf8 {
        format,
        source: err,
    })
}
