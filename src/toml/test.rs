use rstest::rstest;

use crate::convert::convert;
use crate::{Error, Format};

fn cv(input: &[u8]) -> Result<Vec<u8>, Error> {
    let mut out = Vec::new();
    convert(Format::Json, Format::Toml, input, &mut out)?;
    Ok(out)
}

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
#[case::root_array("[1,2]", "json -> toml: root must be a table")]
#[case::null(r#"{"a":[null]}"#, "json -> toml: null at a[0]")]
fn rejects(#[case] input: &str, #[case] expected: &str) {
    assert_eq!(chain(&cv(input.as_bytes()).unwrap_err()), expected);
}
