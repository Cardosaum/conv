use std::io::Write;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::codec::{Decode, Encode};
use crate::error::{Convert, Error};
use crate::format::Format;

pub(crate) struct Url;

impl Decode for Url {
    fn decode<T: DeserializeOwned>(&self, input: &[u8], to: Format) -> Result<T, Error> {
        let text = std::str::from_utf8(input).map_err(|err| Error::Utf8 {
            format: Format::Url,
            source: err,
        })?;
        serde_html_form::from_str(text).convert(Format::Url, to)
    }
}

impl Encode for Url {
    fn encode<T: Serialize, W: Write>(
        &self,
        from: Format,
        value: &T,
        mut writer: W,
    ) -> Result<(), Error> {
        let text = serde_html_form::to_string(value).convert(from, Format::Url)?;
        writer.write_all(text.as_bytes()).map_err(Error::Write)
    }
}

#[cfg(test)]
mod test;
