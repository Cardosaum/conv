use crate::convert::convert;
use crate::{Error, Format};

#[test]
fn trailing_value_is_rejected() {
    let mut out = Vec::new();
    let err = convert(
        Format::Ron,
        Format::Json,
        &br#"{"abc":123,"ok":true} 1"#[..],
        &mut out,
    )
    .unwrap_err();
    assert_eq!(out, b""[..]);
    let source = std::error::Error::source(&err).unwrap();
    assert_eq!(err.to_string(), "ron -> json");
    assert_eq!(
        source.to_string(),
        "1:22-1:23: Non-whitespace trailing characters"
    );
}

#[test]
fn non_utf8_is_rejected() {
    let mut out = Vec::new();
    let err = convert(Format::Ron, Format::Json, &[0xff][..], &mut out).unwrap_err();
    assert_eq!(out, b""[..]);
    assert!(matches!(
        err,
        Error::Utf8 {
            format: Format::Ron,
            ..
        }
    ));
}
