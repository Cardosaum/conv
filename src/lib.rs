//! Convert one structured value between encodings.
//!
//! The binary is a stdin/stdout filter. This library is the same conversion,
//! so tests can call it without spawning a process.

mod cbor;
mod codec;
mod convert;
mod error;
mod format;
mod json;
mod msgpack;
mod ron;
mod toml;
mod url;
mod yaml;

pub use convert::convert;
pub use error::{Error, Source};
pub use format::Format;
