use rstest::rstest;

use crate::convert::convert;
use crate::{Error, Format};

fn chain(err: &Error) -> String {
    let mut text = err.to_string();
    let mut source = std::error::Error::source(err);
    while let Some(next) = source {
        text.push_str(": ");
        text.push_str(&next.to_string());
        source = next.source();
    }
    text
}

#[rstest]
#[case::root_number(
    "1",
    "json -> url: top-level serializer supports only maps and structs"
)]
#[case::root_array("[1,2]", "json -> url: unsupported pair")]
#[case::null(r#"{"a":null}"#, "json -> url: unsupported value")]
fn rejects(#[case] input: &str, #[case] expected: &str) {
    let mut out = Vec::new();
    let err = convert(Format::Json, Format::Url, input.as_bytes(), &mut out).unwrap_err();
    assert_eq!(out, b""[..]);
    assert_eq!(chain(&err), expected);
}

#[test]
fn non_utf8_is_rejected() {
    let mut out = Vec::new();
    let err = convert(Format::Url, Format::Json, &[0xff][..], &mut out).unwrap_err();
    assert_eq!(out, b""[..]);
    assert!(matches!(
        err,
        Error::Utf8 {
            format: Format::Url,
            ..
        }
    ));
}
