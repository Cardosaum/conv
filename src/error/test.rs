use std::io;

use crate::format::Format;

use super::Error;

#[test]
fn convert_display_is_the_direction() {
    let err = Error::convert(
        Format::Json,
        Format::Toml,
        io::Error::new(io::ErrorKind::InvalidData, "boom"),
    );
    assert_eq!(err.to_string(), "json -> toml");
    let source = std::error::Error::source(&err).unwrap();
    assert_eq!(source.to_string(), "boom");
}

#[test]
fn trailing_names_the_format() {
    let err = Error::Trailing {
        format: Format::Cbor,
    };
    assert_eq!(err.to_string(), "cbor: trailing data");
}
