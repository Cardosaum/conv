use clap::ValueEnum;

/// Encoding named on the command line.
///
/// Variant docs are the clap help for each possible value.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Format {
    /// one value, compact output
    Json,
    /// one document
    #[value(alias = "yml")]
    Yaml,
    /// one document, root must be a table
    Toml,
    /// one value
    Ron,
    /// query string, values are strings
    #[value(alias = "www-form")]
    Url,
    /// raw bytes, RFC 8949
    Cbor,
    /// raw bytes
    Msgpack,
}

impl Format {
    pub(crate) fn is_text(self) -> bool {
        matches!(
            self,
            Self::Json | Self::Yaml | Self::Toml | Self::Ron | Self::Url
        )
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Json => "json",
            Self::Yaml => "yaml",
            Self::Toml => "toml",
            Self::Ron => "ron",
            Self::Url => "url",
            Self::Cbor => "cbor",
            Self::Msgpack => "msgpack",
        })
    }
}

#[cfg(test)]
mod test;
