use rstest::rstest;

use super::Format;

#[rstest]
#[case::json(Format::Json, "json")]
#[case::yaml(Format::Yaml, "yaml")]
#[case::toml(Format::Toml, "toml")]
#[case::ron(Format::Ron, "ron")]
#[case::url(Format::Url, "url")]
#[case::cbor(Format::Cbor, "cbor")]
#[case::msgpack(Format::Msgpack, "msgpack")]
fn name_is_lowercase(#[case] format: Format, #[case] name: &str) {
    assert_eq!(format.to_string(), name);
}
