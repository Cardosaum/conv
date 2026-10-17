use std::cell::RefCell;
use std::fmt;
use std::marker::PhantomData;

use serde::Deserialize;
use serde::Serialize;
use serde::de::{self, Deserializer, Error as DeError, MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeMap, SerializeSeq, Serializer};

use crate::error::Source;
use crate::format::Format;

use super::{Encode, Error};

// ponytail: one conversion at a time per thread. `Deserialize` can only return
// the input error, so an output rejection is parked here until `finish` reads
// it. A re-entrant convert on this thread would clobber it.
thread_local! {
    static OUTPUT: RefCell<Option<Source>> = const { RefCell::new(None) };
}

pub(crate) fn clear_output() {
    OUTPUT.with(|slot| *slot.borrow_mut() = None);
}

fn stash(source: Source) {
    OUTPUT.with(|slot| {
        if slot.borrow().is_none() {
            *slot.borrow_mut() = Some(source);
        }
    });
}

fn take_output() -> Option<Source> {
    OUTPUT.with(|slot| slot.borrow_mut().take())
}

fn output_text() -> String {
    OUTPUT.with(|slot| match slot.borrow().as_ref() {
        Some(source) => source.to_string(),
        None => String::from("conversion failed"),
    })
}

pub(crate) fn capture<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Event, Fail<D::Error>> {
    Event::deserialize(deserializer).map_err(Fail::Input)
}

pub(crate) fn emit<'de, D, S>(deserializer: D, serializer: S) -> Result<S::Ok, Fail<D::Error>>
where
    D: Deserializer<'de>,
    S: Serializer,
    S::Error: Into<Source>,
{
    let event = capture(deserializer)?;
    event.serialize(serializer).map_err(reject)
}

pub(crate) fn reject<D>(err: impl Into<Source>) -> Fail<D> {
    stash(err.into());
    Fail::Output
}

pub(crate) enum Fail<D> {
    Input(D),
    Output,
}

pub(crate) fn finish<E: Encode>(
    from: Format,
    result: Result<Forward<E>, impl Into<Source>>,
) -> Result<Vec<u8>, Error> {
    match result {
        Ok(value) => Ok(value.bytes),
        Err(err) => Err(match take_output() {
            Some(source) => Error::convert(from, E::FORMAT, source),
            None => Error::convert(from, E::FORMAT, err),
        }),
    }
}

pub(crate) struct Forward<E> {
    bytes: Vec<u8>,
    _encoder: PhantomData<E>,
}

impl<E: Encode> Forward<E> {
    pub(crate) fn pull<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::deserialize(deserializer)
    }
}

impl<'de, E: Encode> Deserialize<'de> for Forward<E> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match E::accept(deserializer) {
            Ok(bytes) => Ok(Self {
                bytes,
                _encoder: PhantomData,
            }),
            Err(Fail::Input(err)) => Err(err),
            Err(Fail::Output) => Err(DeError::custom(output_text())),
        }
    }
}

/// One serde value, rich enough to serialize more than once.
///
/// ponytail: buffered because `Serialize::serialize` takes `&self` and toml
/// serializes each value once to classify it before writing. A one-shot drive
/// would drop the value on that first pass. Byte strings and non-string keys
/// stay. Integer width is whatever the decoder visits.
pub(crate) enum Event {
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F32(f32),
    F64(f64),
    Char(char),
    Str(String),
    Bytes(Vec<u8>),
    Unit,
    None,
    Some(Vec<Self>),
    Seq(Vec<Self>),
    Map(Vec<(Self, Self)>),
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(Capture)
    }
}

impl Serialize for Event {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Bool(v) => serializer.serialize_bool(*v),
            Self::I8(v) => serializer.serialize_i8(*v),
            Self::I16(v) => serializer.serialize_i16(*v),
            Self::I32(v) => serializer.serialize_i32(*v),
            Self::I64(v) => serializer.serialize_i64(*v),
            Self::I128(v) => serializer.serialize_i128(*v),
            Self::U8(v) => serializer.serialize_u8(*v),
            Self::U16(v) => serializer.serialize_u16(*v),
            Self::U32(v) => serializer.serialize_u32(*v),
            Self::U64(v) => serializer.serialize_u64(*v),
            Self::U128(v) => serializer.serialize_u128(*v),
            Self::F32(v) => serializer.serialize_f32(*v),
            Self::F64(v) => serializer.serialize_f64(*v),
            Self::Char(v) => serializer.serialize_char(*v),
            Self::Str(v) => serializer.serialize_str(v),
            Self::Bytes(v) => serializer.serialize_bytes(v),
            Self::Unit => serializer.serialize_unit(),
            Self::None => serializer.serialize_none(),
            Self::Some(inner) => match inner.first() {
                Some(value) => serializer.serialize_some(value),
                None => serializer.serialize_none(),
            },
            Self::Seq(items) => {
                let mut seq = serializer.serialize_seq(Some(items.len()))?;
                for item in items {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
            Self::Map(entries) => {
                let mut map = serializer.serialize_map(Some(entries.len()))?;
                for (key, value) in entries {
                    map.serialize_key(key)?;
                    map.serialize_value(value)?;
                }
                map.end()
            }
        }
    }
}

struct Capture;

impl<'de> Visitor<'de> for Capture {
    type Value = Event;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("any value")
    }

    fn visit_bool<E: de::Error>(self, v: bool) -> Result<Event, E> {
        Ok(Event::Bool(v))
    }

    fn visit_i8<E: de::Error>(self, v: i8) -> Result<Event, E> {
        Ok(Event::I8(v))
    }

    fn visit_i16<E: de::Error>(self, v: i16) -> Result<Event, E> {
        Ok(Event::I16(v))
    }

    fn visit_i32<E: de::Error>(self, v: i32) -> Result<Event, E> {
        Ok(Event::I32(v))
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Event, E> {
        Ok(Event::I64(v))
    }

    fn visit_i128<E: de::Error>(self, v: i128) -> Result<Event, E> {
        Ok(Event::I128(v))
    }

    fn visit_u8<E: de::Error>(self, v: u8) -> Result<Event, E> {
        Ok(Event::U8(v))
    }

    fn visit_u16<E: de::Error>(self, v: u16) -> Result<Event, E> {
        Ok(Event::U16(v))
    }

    fn visit_u32<E: de::Error>(self, v: u32) -> Result<Event, E> {
        Ok(Event::U32(v))
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Event, E> {
        Ok(Event::U64(v))
    }

    fn visit_u128<E: de::Error>(self, v: u128) -> Result<Event, E> {
        Ok(Event::U128(v))
    }

    fn visit_f32<E: de::Error>(self, v: f32) -> Result<Event, E> {
        Ok(Event::F32(v))
    }

    fn visit_f64<E: de::Error>(self, v: f64) -> Result<Event, E> {
        Ok(Event::F64(v))
    }

    fn visit_char<E: de::Error>(self, v: char) -> Result<Event, E> {
        Ok(Event::Char(v))
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Event, E> {
        Ok(Event::Str(v.to_owned()))
    }

    fn visit_string<E: de::Error>(self, v: String) -> Result<Event, E> {
        Ok(Event::Str(v))
    }

    fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Event, E> {
        Ok(Event::Bytes(v.to_vec()))
    }

    fn visit_byte_buf<E: de::Error>(self, v: Vec<u8>) -> Result<Event, E> {
        Ok(Event::Bytes(v))
    }

    fn visit_unit<E: de::Error>(self) -> Result<Event, E> {
        Ok(Event::Unit)
    }

    fn visit_none<E: de::Error>(self) -> Result<Event, E> {
        Ok(Event::None)
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Event, D::Error> {
        Ok(Event::Some(vec![Event::deserialize(deserializer)?]))
    }

    fn visit_newtype_struct<D: Deserializer<'de>>(
        self,
        deserializer: D,
    ) -> Result<Event, D::Error> {
        Event::deserialize(deserializer)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Event, A::Error> {
        let mut items = Vec::new();
        while let Some(item) = seq.next_element()? {
            items.push(item);
        }
        Ok(Event::Seq(items))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Event, A::Error> {
        let mut entries = Vec::new();
        while let Some(key) = map.next_key()? {
            entries.push((key, map.next_value()?));
        }
        Ok(Event::Map(entries))
    }
}
