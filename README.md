# conv

Convert one value from standard input to standard output.

```sh
conv FROM TO
```

`FROM` and `TO` name the encodings. Names are case-insensitive. `conv` reads no files and takes no configuration.

```sh
echo '{"name":"Ada","ok":true,"count":3}' | conv json toml
```

```toml
name = "Ada"
ok = true
count = 3
```

## Install

```sh
make install
```

That builds the release binary, puts `conv` on `~/.local/bin`, and installs the manual under `$XDG_DATA_HOME/man` (`~/.local/share/man` when that variable is unset or empty).

```sh
make install PREFIX=/usr/local   # PREFIX/bin and PREFIX/share/man
make uninstall                    # same paths as install
```

`BINDIR` and `MANDIR` override those two directories. `DESTDIR` stages the tree and skips the `whatis` update. `make uninstall` removes `conv` and `man1/conv.1` only. Other files stay. Pass the same `PREFIX`, `BINDIR`, `MANDIR`, or `XDG_DATA_HOME` you used to install.

```sh
man conv
man -l doc/conv.1    # from this repo, before install
```

`info conv`, `whatis conv`, and `apropos conv` read that same page. If `man conv` does not find it, keep the system manuals and add this one:

```sh
export MANPATH="${XDG_DATA_HOME:-$HOME/.local/share}/man:${MANPATH:-}"
```

## Examples

YAML to JSON. `yml` is the same encoding:

```sh
printf '%s\n' 'name: Ada' 'ok: true' 'count: 3' | conv yml json
```

```json
{"name":"Ada","ok":true,"count":3}
```

JSON to a query string. `www-form` is the same encoding:

```sh
echo '{"name":"Ada","ok":true}' | conv json url
```

```
name=Ada&ok=true
```

Values come back as strings. A trailing newline is part of the last value, so this input has none:

```sh
printf '%s' 'name=Ada&ok=true' | conv url json
```

```json
{"name":"Ada","ok":"true"}
```

Read a file, convert twice, write a file:

```sh
conv yaml json < notes.yml | conv json toml > notes.toml
```

## Formats

| Name | Also accepted | What you get |
| --- | --- | --- |
| `json` | | One value, compact. |
| `yaml` | `yml` | One document. |
| `toml` | | One document. The root must be a table, and null is an error. |
| `ron` | | One value. |
| `url` | `www-form` | A query string. Values are strings. |
| `cbor` | | Raw bytes, as in RFC 8949. |
| `msgpack` | | Raw bytes. |

`json`, `yaml`, `toml`, `ron`, and `url` are text and end with a newline. `cbor` and `msgpack` are raw bytes, including when standard output is a terminal.

Input is one value. A second value or other trailing data is an error, and nothing is written. Comments, anchors, and tags are not preserved.

`conv --help` prints the short form of the manual.

## Exit status

| Status | Meaning |
| --- | --- |
| 0 | The conversion succeeded. |
| 1 | The value could not be converted. One line on standard error, `conv: ...`, and standard output is left empty. |
| 2 | There was a usage error. |

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
