use std::io;

use crate::format::Format;

/// Failure while reading, writing, or transcoding one value.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Stdin could not be read.
    #[error("reading input")]
    Read(#[source] io::Error),

    /// Stdout could not be written.
    #[error("writing output")]
    Write(#[source] io::Error),

    /// A text format was not UTF-8.
    #[error("{format}: input is not utf-8")]
    Utf8 {
        /// Format that required text.
        format: Format,
        /// Where the byte sequence broke.
        #[source]
        source: std::str::Utf8Error,
    },

    /// The input held another value, or junk, after the first.
    #[error("{format}: trailing data")]
    Trailing {
        /// Format of the input that was not exhausted.
        format: Format,
    },

    /// A serde format rejected the value.
    #[error("{from} -> {to}")]
    Convert {
        /// Encoding of the input.
        from: Format,
        /// Encoding of the output.
        to: Format,
        /// Format-crate error, kept as the source chain.
        #[source]
        source: Source,
    },
}

/// Format-crate failure stored by value.
#[derive(Debug, thiserror::Error)]
pub enum Source {
    /// JSON rejected the value.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// YAML rejected the input.
    #[error(transparent)]
    Yaml(#[from] serde_saphyr::Error),
    /// YAML rejected the output.
    #[error(transparent)]
    YamlWrite(#[from] serde_saphyr::SerializeError),
    /// TOML rejected the input.
    #[error(transparent)]
    TomlRead(#[from] toml::de::Error),
    /// TOML rejected the output.
    #[error(transparent)]
    TomlWrite(#[from] toml::ser::Error),
    /// RON rejected the input.
    #[error(transparent)]
    RonRead(#[from] ron::error::SpannedError),
    /// RON rejected the output.
    #[error(transparent)]
    RonWrite(#[from] ron::error::Error),
    /// URL encoding rejected the input.
    #[error(transparent)]
    UrlRead(#[from] serde_html_form::de::Error),
    /// URL encoding rejected the output.
    #[error(transparent)]
    UrlWrite(#[from] serde_html_form::ser::Error),
    /// CBOR rejected the input.
    #[error(transparent)]
    CborRead(#[from] ciborium::de::Error<io::Error>),
    /// CBOR rejected the output.
    #[error(transparent)]
    CborWrite(#[from] ciborium::ser::Error<io::Error>),
    /// MessagePack rejected the input.
    #[error(transparent)]
    MsgpackRead(#[from] rmp_serde::decode::Error),
    /// MessagePack rejected the output.
    #[error(transparent)]
    MsgpackWrite(#[from] rmp_serde::encode::Error),
    /// A format crate returned an I/O error.
    #[error(transparent)]
    Io(#[from] io::Error),
    /// A conversion rule rejected the value.
    #[error("{0}")]
    Message(String),
}

impl Error {
    pub(crate) fn convert(from: Format, to: Format, source: impl Into<Source>) -> Self {
        Self::Convert {
            from,
            to,
            source: source.into(),
        }
    }

    pub(crate) fn reason(from: Format, to: Format, message: impl Into<String>) -> Self {
        Self::convert(from, to, Source::Message(message.into()))
    }
}

pub(crate) trait Convert<T> {
    fn convert(self, from: Format, to: Format) -> Result<T, Error>;
}

impl<T, E: Into<Source>> Convert<T> for Result<T, E> {
    fn convert(self, from: Format, to: Format) -> Result<T, Error> {
        self.map_err(|err| Error::convert(from, to, err))
    }
}

#[cfg(test)]
mod test;
