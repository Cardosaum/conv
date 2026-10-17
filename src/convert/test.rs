// The unicode fixture ends in U+0061 U+0303, not precomposed ã.
#![allow(clippy::unicode_not_nfc)]

use rstest::rstest;

use super::convert;
use crate::{Error, Format};

fn cv(from: Format, to: Format, input: &[u8]) -> Result<Vec<u8>, Error> {
    let mut out = Vec::new();
    convert(from, to, input, &mut out)?;
    Ok(out)
}

#[test]
fn failure_writes_nothing() {
    let mut out = Vec::new();
    let err = convert(
        Format::Json,
        Format::Toml,
        &br#"{"a":1}{"b":2}"#[..],
        &mut out,
    )
    .unwrap_err();
    assert_eq!(out, b""[..]);
    assert_eq!(err.to_string(), "json -> toml");
    assert_eq!(
        std::error::Error::source(&err).unwrap().to_string(),
        "trailing characters at line 1 column 8"
    );
}

#[rstest]
#[case::json(Format::Json)]
#[case::ron(Format::Ron)]
#[case::cbor(Format::Cbor)]
#[case::msgpack(Format::Msgpack)]
fn empty_input_is_rejected(#[case] format: Format) {
    assert!(cv(format, Format::Json, b"").is_err());
}

#[rstest]
#[case::cbor(Format::Cbor)]
#[case::msgpack(Format::Msgpack)]
fn trailing_bytes_are_rejected(#[case] format: Format) {
    let mut extra = cv(Format::Json, format, b"{\"a\":1}").unwrap();
    extra.push(0xff);
    assert!(matches!(
        cv(format, Format::Json, &extra),
        Err(Error::Trailing { format: got }) if got == format
    ));
}

#[rstest]
#[case::json_to_json(
    Format::Json,
    Format::Json,
    r#"{"abc":123,"ok":true}"#,
    r#"{"abc":123,"ok":true}
"#
)]
#[case::json_to_yaml(
    Format::Json,
    Format::Yaml,
    r#"{"abc":123,"ok":true}"#,
    r"abc: 123
ok: true
"
)]
#[case::yaml_to_json(
    Format::Yaml,
    Format::Json,
    r"abc: 123
ok: true
",
    r#"{"abc":123,"ok":true}
"#
)]
#[case::json_to_toml(
    Format::Json,
    Format::Toml,
    r#"{"abc":123,"ok":true}"#,
    r"abc = 123
ok = true
"
)]
#[case::toml_to_json(
    Format::Toml,
    Format::Json,
    r"abc = 123
ok = true
",
    r#"{"abc":123,"ok":true}
"#
)]
#[case::json_to_ron(
    Format::Json,
    Format::Ron,
    r#"{"abc":123,"ok":true}"#,
    r#"{"abc":123,"ok":true}
"#
)]
#[case::ron_to_json(
    Format::Ron,
    Format::Json,
    r"// note
(abc: 123, ok: true,)
",
    r#"{"abc":123,"ok":true}
"#
)]
#[case::json_to_url(
    Format::Json,
    Format::Url,
    r#"{"abc":123,"ok":true}"#,
    "abc=123&ok=true\n"
)]
#[case::url_to_json(
    Format::Url,
    Format::Json,
    "abc=123&ok=true",
    r#"{"abc":"123","ok":"true"}
"#
)]
#[case::json_to_toml_scalars(
    Format::Json,
    Format::Toml,
    r#"{"abc":123,"ok":true,"verso":"É ferida que dói e não se sente, tão contrário nos corações.","n":1.5}"#,
    r#"abc = 123
ok = true
verso = "É ferida que dói e não se sente, tão contrário nos corações."
n = 1.5
"#
)]
#[case::toml_to_json_sorted(
    Format::Toml,
    Format::Json,
    r#"abc = 123
ok = true
verso = "É ferida que dói e não se sente, tão contrário nos corações."
n = 1.5
"#,
    r#"{"abc":123,"n":1.5,"ok":true,"verso":"É ferida que dói e não se sente, tão contrário nos corações."}
"#
)]
#[case::json_to_yaml_scalars(
    Format::Json,
    Format::Yaml,
    r#"{"abc":123,"ok":true,"verso":"É ferida que dói e não se sente, tão contrário nos corações.","n":1.5}"#,
    r#"abc: 123
ok: true
verso: É ferida que dói e não se sente, tão contrário nos corações.
"n": 1.5
"#
)]
#[case::json_to_json_whitespace(
    Format::Json,
    Format::Json,
    r#"{"a":1}

"#,
    r#"{"a":1}
"#
)]
#[case::yaml_to_json_empty(Format::Yaml, Format::Json, "", "null\n")]
#[case::toml_to_json_blank(Format::Toml, Format::Json, "   ", "{}\n")]
#[case::json_to_json_nested(
    Format::Json,
    Format::Json,
    r#"{"verso":"É ferida que dói e não se sente, tão contrário nos corações.","i":123,"f":1.5,"b":false,"z":null,"a":[1,"two"],"o":{"k":"v"}}"#,
    r#"{"verso":"É ferida que dói e não se sente, tão contrário nos corações.","i":123,"f":1.5,"b":false,"z":null,"a":[1,"two"],"o":{"k":"v"}}
"#
)]
#[case::json_to_yaml_nested(
    Format::Json,
    Format::Yaml,
    r#"{"verso":"É ferida que dói e não se sente, tão contrário nos corações.","i":123,"f":1.5,"b":false,"z":null,"a":[1,"two"],"o":{"k":"v"}}"#,
    r"verso: É ferida que dói e não se sente, tão contrário nos corações.
i: 123
f: 1.5
b: false
z: null
a:
- 1
- two
o:
  k: v
"
)]
#[case::json_to_ron_nested(
    Format::Json,
    Format::Ron,
    r#"{"verso":"É ferida que dói e não se sente, tão contrário nos corações.","i":123,"f":1.5,"b":false,"z":null,"a":[1,"two"],"o":{"k":"v"}}"#,
    r#"{"verso":"É ferida que dói e não se sente, tão contrário nos corações.","i":123,"f":1.5,"b":false,"z":(),"a":[1,"two"],"o":{"k":"v"}}
"#
)]
#[case::ron_to_json_nested(
    Format::Ron,
    Format::Json,
    r#"{"verso":"É ferida que dói e não se sente, tão contrário nos corações.","i":123,"f":1.5,"b":false,"z":(),"a":[1,"two"],"o":{"k":"v"}}
"#,
    r#"{"verso":"É ferida que dói e não se sente, tão contrário nos corações.","i":123,"f":1.5,"b":false,"z":null,"a":[1,"two"],"o":{"k":"v"}}
"#
)]
#[case::yaml_to_json_nested(
    Format::Yaml,
    Format::Json,
    r"verso: É ferida que dói e não se sente, tão contrário nos corações.
i: 123
f: 1.5
b: false
z: null
a:
- 1
- two
o:
  k: v
",
    r#"{"verso":"É ferida que dói e não se sente, tão contrário nos corações.","i":123,"f":1.5,"b":false,"z":null,"a":[1,"two"],"o":{"k":"v"}}
"#
)]
#[case::json_to_json_unicode(
    Format::Json,
    Format::Json,
    r#"{"u":"éãçõáóíüñø€漢Яשלוםمرحبا😀ã"}"#,
    r#"{"u":"éãçõáóíüñø€漢Яשלוםمرحبا😀ã"}
"#
)]
#[case::json_to_yaml_unicode(
    Format::Json,
    Format::Yaml,
    r#"{"u":"éãçõáóíüñø€漢Яשלוםمرحبا😀ã"}"#,
    r"u: éãçõáóíüñø€漢Яשלוםمرحبا😀ã
"
)]
#[case::yaml_to_json_unicode(
    Format::Yaml,
    Format::Json,
    r"u: éãçõáóíüñø€漢Яשלוםمرحبا😀ã
",
    r#"{"u":"éãçõáóíüñø€漢Яשלוםمرحبا😀ã"}
"#
)]
#[case::json_to_toml_unicode(
    Format::Json,
    Format::Toml,
    r#"{"u":"éãçõáóíüñø€漢Яשלוםمرحبا😀ã"}"#,
    r#"u = "éãçõáóíüñø€漢Яשלוםمرحبا😀ã"
"#
)]
#[case::json_to_ron_unicode(
    Format::Json,
    Format::Ron,
    r#"{"u":"éãçõáóíüñø€漢Яשלוםمرحبا😀ã"}"#,
    "{\"u\":\"éãçõáóíüñø€漢Яשלוםمرحبا😀a\\u{303}\"}\n"
)]
#[case::ron_to_json_unicode(
    Format::Ron,
    Format::Json,
    "{\"u\":\"éãçõáóíüñø€漢Яשלוםمرحبا😀a\\u{303}\"}\n",
    r#"{"u":"éãçõáóíüñø€漢Яשלוםمرحبا😀ã"}
"#
)]
#[case::json_to_url_unicode(
    Format::Json,
    Format::Url,
    r#"{"u":"éãçõáóíüñø€漢Яשלוםمرحبا😀ã"}"#,
    "u=%C3%A9%C3%A3%C3%A7%C3%B5%C3%A1%C3%B3%C3%AD%C3%BC%C3%B1%C3%B8%E2%82%AC%E6%BC%A2%D0%AF%D7%A9%D7%9C%D7%95%D7%9D%D9%85%D8%B1%D8%AD%D8%A8%D8%A7%F0%9F%98%80a%CC%83\n"
)]
#[case::url_to_json_unicode(
    Format::Url,
    Format::Json,
    "u=%C3%A9%C3%A3%C3%A7%C3%B5%C3%A1%C3%B3%C3%AD%C3%BC%C3%B1%C3%B8%E2%82%AC%E6%BC%A2%D0%AF%D7%A9%D7%9C%D7%95%D7%9D%D9%85%D8%B1%D8%AD%D8%A8%D8%A7%F0%9F%98%80a%CC%83",
    r#"{"u":"éãçõáóíüñø€漢Яשלוםمرحبا😀ã"}
"#
)]
#[case::url_to_json_empty(Format::Url, Format::Json, "", "{}\n")]
#[case::toml_to_json_unicode(
    Format::Toml,
    Format::Json,
    r#"u = "éãçõáóíüñø€漢Яשלוםمرحبا😀ã"
"#,
    r#"{"u":"éãçõáóíüñø€漢Яשלוםمرحبا😀ã"}
"#
)]
fn text(#[case] from: Format, #[case] to: Format, #[case] input: &str, #[case] expected: &str) {
    let out = cv(from, to, input.as_bytes()).unwrap();
    assert_eq!(String::from_utf8(out).unwrap(), expected);
}

#[rstest]
#[case::json_to_cbor(
    Format::Json,
    Format::Cbor,
    br#"{"abc":123,"ok":true}"# as &[u8],
    &[0xa2, 0x63, b'a', b'b', b'c', 0x18, 0x7b, 0x62, b'o', b'k', 0xf5][..]
)]
#[case::cbor_to_json(
    Format::Cbor,
    Format::Json,
    &[0xa2, 0x63, b'a', b'b', b'c', 0x18, 0x7b, 0x62, b'o', b'k', 0xf5][..],
    b"{\"abc\":123,\"ok\":true}\n"
)]
#[case::json_to_msgpack(
    Format::Json,
    Format::Msgpack,
    br#"{"abc":123,"ok":true}"# as &[u8],
    &[0x82, 0xa3, b'a', b'b', b'c', 0x7b, 0xa2, b'o', b'k', 0xc3][..]
)]
#[case::msgpack_to_json(
    Format::Msgpack,
    Format::Json,
    &[0x82, 0xa3, b'a', b'b', b'c', 0x7b, 0xa2, b'o', b'k', 0xc3][..],
    b"{\"abc\":123,\"ok\":true}\n"
)]
#[case::json_to_cbor_nested(
    Format::Json,
    Format::Cbor,
    r#"{"verso":"É ferida que dói e não se sente, tão contrário nos corações.","i":123,"f":1.5,"b":false,"z":null,"a":[1,"two"],"o":{"k":"v"}}"#.as_bytes(),
    &[
        0xa7, 0x65, 0x76, 0x65, 0x72, 0x73, 0x6f, 0x78, 0x43, 0xc3, 0x89, 0x20, 0x66, 0x65, 0x72, 0x69,
        0x64, 0x61, 0x20, 0x71, 0x75, 0x65, 0x20, 0x64, 0xc3, 0xb3, 0x69, 0x20, 0x65, 0x20, 0x6e, 0xc3,
        0xa3, 0x6f, 0x20, 0x73, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x65, 0x2c, 0x20, 0x74, 0xc3, 0xa3,
        0x6f, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0xc3, 0xa1, 0x72, 0x69, 0x6f, 0x20, 0x6e, 0x6f, 0x73,
        0x20, 0x63, 0x6f, 0x72, 0x61, 0xc3, 0xa7, 0xc3, 0xb5, 0x65, 0x73, 0x2e, 0x61, 0x69, 0x18, 0x7b,
        0x61, 0x66, 0xf9, 0x3e, 0x00, 0x61, 0x62, 0xf4, 0x61, 0x7a, 0xf6, 0x61, 0x61, 0x82, 0x01, 0x63,
        0x74, 0x77, 0x6f, 0x61, 0x6f, 0xa1, 0x61, 0x6b, 0x61, 0x76,
    ][..]
)]
#[case::cbor_to_json_nested(
    Format::Cbor,
    Format::Json,
    &[
        0xa7, 0x65, 0x76, 0x65, 0x72, 0x73, 0x6f, 0x78, 0x43, 0xc3, 0x89, 0x20, 0x66, 0x65, 0x72, 0x69,
        0x64, 0x61, 0x20, 0x71, 0x75, 0x65, 0x20, 0x64, 0xc3, 0xb3, 0x69, 0x20, 0x65, 0x20, 0x6e, 0xc3,
        0xa3, 0x6f, 0x20, 0x73, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x65, 0x2c, 0x20, 0x74, 0xc3, 0xa3,
        0x6f, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0xc3, 0xa1, 0x72, 0x69, 0x6f, 0x20, 0x6e, 0x6f, 0x73,
        0x20, 0x63, 0x6f, 0x72, 0x61, 0xc3, 0xa7, 0xc3, 0xb5, 0x65, 0x73, 0x2e, 0x61, 0x69, 0x18, 0x7b,
        0x61, 0x66, 0xf9, 0x3e, 0x00, 0x61, 0x62, 0xf4, 0x61, 0x7a, 0xf6, 0x61, 0x61, 0x82, 0x01, 0x63,
        0x74, 0x77, 0x6f, 0x61, 0x6f, 0xa1, 0x61, 0x6b, 0x61, 0x76,
    ][..],
    r#"{"verso":"É ferida que dói e não se sente, tão contrário nos corações.","i":123,"f":1.5,"b":false,"z":null,"a":[1,"two"],"o":{"k":"v"}}
"#.as_bytes()
)]
#[case::json_to_msgpack_nested(
    Format::Json,
    Format::Msgpack,
    r#"{"verso":"É ferida que dói e não se sente, tão contrário nos corações.","i":123,"f":1.5,"b":false,"z":null,"a":[1,"two"],"o":{"k":"v"}}"#.as_bytes(),
    &[
        0x87, 0xa5, 0x76, 0x65, 0x72, 0x73, 0x6f, 0xd9, 0x43, 0xc3, 0x89, 0x20, 0x66, 0x65, 0x72, 0x69,
        0x64, 0x61, 0x20, 0x71, 0x75, 0x65, 0x20, 0x64, 0xc3, 0xb3, 0x69, 0x20, 0x65, 0x20, 0x6e, 0xc3,
        0xa3, 0x6f, 0x20, 0x73, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x65, 0x2c, 0x20, 0x74, 0xc3, 0xa3,
        0x6f, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0xc3, 0xa1, 0x72, 0x69, 0x6f, 0x20, 0x6e, 0x6f, 0x73,
        0x20, 0x63, 0x6f, 0x72, 0x61, 0xc3, 0xa7, 0xc3, 0xb5, 0x65, 0x73, 0x2e, 0xa1, 0x69, 0x7b, 0xa1,
        0x66, 0xcb, 0x3f, 0xf8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa1, 0x62, 0xc2, 0xa1, 0x7a, 0xc0,
        0xa1, 0x61, 0x92, 0x01, 0xa3, 0x74, 0x77, 0x6f, 0xa1, 0x6f, 0x81, 0xa1, 0x6b, 0xa1, 0x76,
    ][..]
)]
#[case::msgpack_to_json_nested(
    Format::Msgpack,
    Format::Json,
    &[
        0x87, 0xa5, 0x76, 0x65, 0x72, 0x73, 0x6f, 0xd9, 0x43, 0xc3, 0x89, 0x20, 0x66, 0x65, 0x72, 0x69,
        0x64, 0x61, 0x20, 0x71, 0x75, 0x65, 0x20, 0x64, 0xc3, 0xb3, 0x69, 0x20, 0x65, 0x20, 0x6e, 0xc3,
        0xa3, 0x6f, 0x20, 0x73, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x65, 0x2c, 0x20, 0x74, 0xc3, 0xa3,
        0x6f, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0xc3, 0xa1, 0x72, 0x69, 0x6f, 0x20, 0x6e, 0x6f, 0x73,
        0x20, 0x63, 0x6f, 0x72, 0x61, 0xc3, 0xa7, 0xc3, 0xb5, 0x65, 0x73, 0x2e, 0xa1, 0x69, 0x7b, 0xa1,
        0x66, 0xcb, 0x3f, 0xf8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa1, 0x62, 0xc2, 0xa1, 0x7a, 0xc0,
        0xa1, 0x61, 0x92, 0x01, 0xa3, 0x74, 0x77, 0x6f, 0xa1, 0x6f, 0x81, 0xa1, 0x6b, 0xa1, 0x76,
    ][..],
    r#"{"verso":"É ferida que dói e não se sente, tão contrário nos corações.","i":123,"f":1.5,"b":false,"z":null,"a":[1,"two"],"o":{"k":"v"}}
"#.as_bytes()
)]
#[case::json_to_cbor_unicode(
    Format::Json,
    Format::Cbor,
    r#"{"u":"éãçõáóíüñø€漢Яשלוםمرحبا😀ã"}"#.as_bytes(),
    &[
        0xa1, 0x61, 0x75, 0x78, 0x35, 0xc3, 0xa9, 0xc3, 0xa3, 0xc3, 0xa7, 0xc3, 0xb5, 0xc3, 0xa1, 0xc3,
        0xb3, 0xc3, 0xad, 0xc3, 0xbc, 0xc3, 0xb1, 0xc3, 0xb8, 0xe2, 0x82, 0xac, 0xe6, 0xbc, 0xa2, 0xd0,
        0xaf, 0xd7, 0xa9, 0xd7, 0x9c, 0xd7, 0x95, 0xd7, 0x9d, 0xd9, 0x85, 0xd8, 0xb1, 0xd8, 0xad, 0xd8,
        0xa8, 0xd8, 0xa7, 0xf0, 0x9f, 0x98, 0x80, 0x61, 0xcc, 0x83,
    ][..]
)]
#[case::cbor_to_json_unicode(
    Format::Cbor,
    Format::Json,
    &[
        0xa1, 0x61, 0x75, 0x78, 0x35, 0xc3, 0xa9, 0xc3, 0xa3, 0xc3, 0xa7, 0xc3, 0xb5, 0xc3, 0xa1, 0xc3,
        0xb3, 0xc3, 0xad, 0xc3, 0xbc, 0xc3, 0xb1, 0xc3, 0xb8, 0xe2, 0x82, 0xac, 0xe6, 0xbc, 0xa2, 0xd0,
        0xaf, 0xd7, 0xa9, 0xd7, 0x9c, 0xd7, 0x95, 0xd7, 0x9d, 0xd9, 0x85, 0xd8, 0xb1, 0xd8, 0xad, 0xd8,
        0xa8, 0xd8, 0xa7, 0xf0, 0x9f, 0x98, 0x80, 0x61, 0xcc, 0x83,
    ][..],
    r#"{"u":"éãçõáóíüñø€漢Яשלוםمرحبا😀ã"}
"#.as_bytes()
)]
#[case::json_to_msgpack_unicode(
    Format::Json,
    Format::Msgpack,
    r#"{"u":"éãçõáóíüñø€漢Яשלוםمرحبا😀ã"}"#.as_bytes(),
    &[
        0x81, 0xa1, 0x75, 0xd9, 0x35, 0xc3, 0xa9, 0xc3, 0xa3, 0xc3, 0xa7, 0xc3, 0xb5, 0xc3, 0xa1, 0xc3,
        0xb3, 0xc3, 0xad, 0xc3, 0xbc, 0xc3, 0xb1, 0xc3, 0xb8, 0xe2, 0x82, 0xac, 0xe6, 0xbc, 0xa2, 0xd0,
        0xaf, 0xd7, 0xa9, 0xd7, 0x9c, 0xd7, 0x95, 0xd7, 0x9d, 0xd9, 0x85, 0xd8, 0xb1, 0xd8, 0xad, 0xd8,
        0xa8, 0xd8, 0xa7, 0xf0, 0x9f, 0x98, 0x80, 0x61, 0xcc, 0x83,
    ][..]
)]
#[case::cbor_bytes_to_msgpack(
    Format::Cbor,
    Format::Msgpack,
    &[0x42, b'h', b'i'][..],
    &[0xc4, 0x02, b'h', b'i'][..]
)]
#[case::msgpack_bytes_to_cbor(
    Format::Msgpack,
    Format::Cbor,
    &[0xc4, 0x02, b'h', b'i'][..],
    &[0x42, b'h', b'i'][..]
)]
#[case::msgpack_to_json_unicode(
    Format::Msgpack,
    Format::Json,
    &[
        0x81, 0xa1, 0x75, 0xd9, 0x35, 0xc3, 0xa9, 0xc3, 0xa3, 0xc3, 0xa7, 0xc3, 0xb5, 0xc3, 0xa1, 0xc3,
        0xb3, 0xc3, 0xad, 0xc3, 0xbc, 0xc3, 0xb1, 0xc3, 0xb8, 0xe2, 0x82, 0xac, 0xe6, 0xbc, 0xa2, 0xd0,
        0xaf, 0xd7, 0xa9, 0xd7, 0x9c, 0xd7, 0x95, 0xd7, 0x9d, 0xd9, 0x85, 0xd8, 0xb1, 0xd8, 0xad, 0xd8,
        0xa8, 0xd8, 0xa7, 0xf0, 0x9f, 0x98, 0x80, 0x61, 0xcc, 0x83,
    ][..],
    r#"{"u":"éãçõáóíüñø€漢Яשלוםمرحبا😀ã"}
"#.as_bytes()
)]
fn bytes(#[case] from: Format, #[case] to: Format, #[case] input: &[u8], #[case] expected: &[u8]) {
    assert_eq!(cv(from, to, input).unwrap(), expected);
}
