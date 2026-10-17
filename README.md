# conv

Convert one value from standard input to standard output.

```sh
conv FROM TO
```

```sh
echo '{"abc": 123}' | conv json toml
```

Formats: `json`, `yaml` (`yml`), `toml`, `ron`, `url` (`www-form`), `cbor`, `msgpack`. Names are case-insensitive.

`conv --help` lists them. Exit 0 on success, 1 when a value cannot be converted, 2 on a usage error.

JSON output is compact. Text output ends with a newline. `cbor` and `msgpack` are raw bytes. A second value or other trailing data is an error, and nothing is written. Comments are not preserved. TOML output requires a table at the root, and null is an error. `url` is a query string, and its values are strings.

## Development

The compiler is pinned in `rust-toolchain.toml`.

```sh
cargo fmt --check
cargo lint
cargo nextest run --profile ci
cargo deny check
cargo audit
cargo machete
typos
```

`cargo lint` runs clippy on every target with warnings denied. `cargo deny` and `cargo audit` fail on any known advisory.

## License

[MIT](LICENSE).
