//! JSON support for [`IndexMap`] and [`IndexSet`].
//!
//! Maps are encoded as JSON objects, preserving insertion order. Sets are
//! encoded as arrays, also preserving insertion order.

use alloc::string::String;
use core::hash::BuildHasher;

use ::json::{Decode, Decoder, Encode, Encoder, ErrorKind, Key, Write};

use crate::{IndexMap, IndexSet};

impl<K, V, S> Encode for IndexMap<K, V, S>
where
    K: AsRef<str>,
    V: Encode,
{
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error> {
        encoder.begin_object()?;
        for (index, (key, value)) in self.iter().enumerate() {
            encoder.comma(index == 0)?;
            encoder.key(key.as_ref())?;
            value.encode(encoder)?;
        }
        encoder.end_object()
    }
}

impl<'a, V, S> Decode<'a> for IndexMap<String, V, S>
where
    V: Decode<'a>,
    S: BuildHasher + Default,
{
    fn decode(decoder: &mut Decoder<'a>) -> ::json::Result<Self> {
        decoder.begin_object()?;
        let mut map = IndexMap::with_hasher(S::default());
        let mut first = true;
        while decoder.object_next(first)? {
            first = false;
            let key = decoder.key()?.into_cow().into_owned();
            let seen = map.contains_key(key.as_str());
            if decoder.duplicate_action(seen, false)? {
                let value = decoder.decode_inline::<V>()?;
                map.insert(key, value);
            } else {
                decoder.skip_value()?;
            }
        }
        Ok(map)
    }
}

impl<'a, V, S> Decode<'a> for IndexMap<&'a str, V, S>
where
    V: Decode<'a>,
    S: BuildHasher + Default,
{
    fn decode(decoder: &mut Decoder<'a>) -> ::json::Result<Self> {
        decoder.begin_object()?;
        let mut map = IndexMap::with_hasher(S::default());
        let mut first = true;
        while decoder.object_next(first)? {
            first = false;
            let key = match decoder.key()? {
                Key::Borrowed(key) => key,
                Key::Escaped(_) => {
                    return Err(decoder.error(ErrorKind::Message("escaped key cannot be stored as &str")));
                }
            };
            let seen = map.contains_key(key);
            if decoder.duplicate_action(seen, false)? {
                let value = decoder.decode_inline::<V>()?;
                map.insert(key, value);
            } else {
                decoder.skip_value()?;
            }
        }
        Ok(map)
    }
}

impl<T, S> Encode for IndexSet<T, S>
where
    T: Encode,
{
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error> {
        encoder.begin_array()?;
        for (index, value) in self.iter().enumerate() {
            encoder.comma(index == 0)?;
            value.encode(encoder)?;
        }
        encoder.end_array()
    }
}

impl<'a, T, S> Decode<'a> for IndexSet<T, S>
where
    T: Decode<'a> + Eq + core::hash::Hash,
    S: BuildHasher + Default,
{
    fn decode(decoder: &mut Decoder<'a>) -> ::json::Result<Self> {
        decoder.begin_array()?;
        let mut set = IndexSet::with_hasher(S::default());
        let mut first = true;
        while decoder.array_next(first)? {
            first = false;
            set.insert(decoder.decode_inline::<T>()?);
        }
        Ok(set)
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::{String, ToString};

    use super::*;

    #[test]
    fn map_and_set_roundtrip() {
        let mut map = IndexMap::new();
        map.insert("b".to_string(), 2i64);
        map.insert("a".to_string(), 1i64);
        let text = ::json::to_string(&map);
        assert_eq!(text, r#"{"b":2,"a":1}"#);
        let decoded: IndexMap<String, i64> = ::json::from_str(&text).unwrap();
        assert_eq!(decoded, map);

        let set: IndexSet<i64> = [3, 1, 2].into_iter().collect();
        let text = ::json::to_string(&set);
        assert_eq!(text, "[3,1,2]");
        let decoded: IndexSet<i64> = ::json::from_str(&text).unwrap();
        assert_eq!(decoded, set);
    }
}
