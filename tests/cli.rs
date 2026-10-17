//! Spawns the `conv` binary.

use std::io::Write;
use std::process::{Command, Output, Stdio};

use rstest::rstest;

fn run(args: &[&str], stdin: Option<&[u8]>) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_conv"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn conv");
    if let Some(bytes) = stdin {
        let mut input = child.stdin.take().expect("stdin");
        let _ = input.write_all(bytes);
    }
    child.wait_with_output().expect("wait")
}

#[rstest]
#[case::json_to_toml(&["json", "toml"], br#"{"abc":123}"#, b"abc = 123\n")]
#[case::json_to_yml(&["JSON", "yml"], br#"{"abc":123}"#, b"abc: 123\n")]
#[case::json_to_ron(&["json", "ron"], br#"{"abc":123,"ok":true}"#, b"{\"abc\":123,\"ok\":true}\n")]
#[case::json_to_www_form(&["JSON", "www-form"], br#"{"abc":123,"ok":true}"#, b"abc=123&ok=true\n")]
fn converts(#[case] args: &[&str], #[case] stdin: &[u8], #[case] expected: &[u8]) {
    let out = run(args, Some(stdin));
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(out.stderr, b""[..]);
    assert_eq!(out.stdout, expected);
}

#[rstest]
#[case::help(
    &["--help"],
    &[
        "Usage: conv [OPTION]... FROM TO",
        "standard input",
        "input encoding",
        "display this help and exit",
        "output version information and exit",
        "one value, compact output",
        "yml",
        "www-form",
        "Exit status",
    ][..]
)]
#[case::short_help(
    &["-h"],
    &[
        "Usage: conv [OPTION]... FROM TO",
        "standard input",
        "display this help and exit",
        "yml",
        "www-form",
        "Exit status",
    ][..]
)]
#[case::version(&["--version"], &["conv", env!("CARGO_PKG_VERSION")][..])]
fn help_and_version_exit_zero(#[case] args: &[&str], #[case] needles: &[&str]) {
    let out = run(args, None);
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8(out.stdout).unwrap();
    for needle in needles {
        assert!(stdout.contains(needle), "{stdout}");
    }
}

#[rstest]
#[case::no_args(&[] as &[&str])]
#[case::one_format(&["json"])]
#[case::unknown(&["nope", "json"])]
#[case::extra(&["json", "toml", "extra"])]
fn usage_errors_exit_two(#[case] args: &[&str]) {
    let out = run(args, None);
    assert_eq!(out.status.code(), Some(2), "{args:?} -> {out:?}");
}

#[test]
fn bad_value_exits_one_and_writes_nothing() {
    let out = run(&["json", "toml"], Some(br#"{"a":1}{"b":2}"#));
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(out.stdout, b""[..]);
    let err = String::from_utf8(out.stderr).unwrap();
    assert!(err.starts_with("conv:"), "{err}");
}
