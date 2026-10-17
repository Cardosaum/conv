use crate::convert::convert;
use crate::format::Format;

#[test]
fn second_document_is_rejected() {
    let mut out = Vec::new();
    let err = convert(
        Format::Yaml,
        Format::Json,
        &b"a: 1\n---\nb: 2\n"[..],
        &mut out,
    )
    .unwrap_err();
    assert_eq!(out, b""[..]);
    assert_eq!(err.to_string(), "yaml -> json");
    assert!(
        std::error::Error::source(&err)
            .unwrap()
            .to_string()
            .contains("multiple YAML documents"),
        "{err:?}"
    );
}
