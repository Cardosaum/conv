use serde::de::Deserializer;

use crate::codec::Forward;
use crate::codec::{Decode, Encode, Fail, capture, clear_output, finish, reject, text};
use crate::format::Format;

pub(crate) struct Url;

impl Decode for Url {
    fn transcode<E: Encode>(input: &[u8]) -> Result<Vec<u8>, crate::error::Error> {
        clear_output();
        let input = text(input, Format::Url)?;
        finish::<E>(Format::Url, serde_html_form::from_str::<Forward<E>>(input))
    }
}

impl Encode for Url {
    const FORMAT: Format = Format::Url;

    fn accept<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, Fail<D::Error>> {
        let event = capture(deserializer)?;
        let text = serde_html_form::to_string(&event).map_err(reject)?;
        Ok(text.into_bytes())
    }
}

#[cfg(test)]
mod test;
