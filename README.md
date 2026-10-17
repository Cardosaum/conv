# conv

Convert one value from standard input to standard output.

```sh
conv FROM TO
```

`FROM` and `TO` name the encodings. Names are case-insensitive. `conv` reads no files and takes no configuration.

## Examples

A line starting with `$` is what you type. The lines under it are the result.

Read a TOML config with `jq`. Comments are dropped, so this is for the pipe, not for rewriting a file you edit by hand.

```console
$ cat app.toml
[database]
host = "localhost"
port = 5432
name = "app"

$ conv toml json < app.toml
{"database":{"host":"localhost","name":"app","port":5432}}

$ conv toml json < app.toml | jq '.database.port'
5432
```

Turn that JSON into a snippet you can paste into a config:

```console
$ printf '%s\n' '{"database":{"host":"localhost","port":5432,"name":"app"}}' | conv json toml
[database]
host = "localhost"
port = 5432
name = "app"
```

Post a flat JSON object as a form. `"$(...)"` drops the newline `conv` adds, so it does not become part of the password:

```console
$ printf '%s\n' '{"user":"ada","password":"secret"}' | conv json url
user=ada&password=secret

$ curl -fsS --data "$(conv json url < login.json)" \
    -H 'content-type: application/x-www-form-urlencoded' \
    https://example/login
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

`BINDIR` and `MANDIR` override those two directories. `DESTDIR` stages the tree and skips the `whatis` update. `make uninstall` removes `conv`, `man1/conv.1`, and a `man` wrapper this install added. Other files stay. Pass the same `PREFIX`, `BINDIR`, `MANDIR`, or `XDG_DATA_HOME` you used to install.

```sh
man conv
man -l doc/conv.1    # from this repo, before install
```

`info conv`, `whatis conv`, and `apropos conv` read that same page. If `man` does not already search the manual directory, and `BINDIR` is ahead of `man` on `PATH`, `make install` adds a `man` wrapper there. The wrapper puts that directory on `MANPATH` and then runs the real `man`. `whatis` and `apropos` are linked to it. When `BINDIR` is not ahead of `man`, install stops and prints:

```sh
export MANPATH="${XDG_DATA_HOME:-$HOME/.local/share}/man:${MANPATH:-}"
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
