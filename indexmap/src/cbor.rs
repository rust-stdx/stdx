//! CBOR support for [`IndexMap`] and [`IndexSet`].
//!
//! Maps are encoded as CBOR maps and sets as CBOR arrays, preserving insertion
//! order. Decoding accepts both definite- and indefinite-length containers.

use core::hash::{BuildHasher, Hash};

use ::cbor::{Decode, Decoder, Encode, Encoder, Error, Write};

use crate::{IndexMap, IndexSet};

impl<K, V, S> Encode for IndexMap<K, V, S>
where
    K: Encode,
    V: Encode,
{
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> Result<(), W::Error> {
        encoder.map(self.len())?;
        for (key, value) in self.iter() {
            key.encode(encoder)?;
            value.encode(encoder)?;
        }
        Ok(())
    }
}

impl<'b, K, V, S> Decode<'b> for IndexMap<K, V, S>
where
    K: Decode<'b> + Eq + Hash,
    V: Decode<'b>,
    S: BuildHasher + Default,
{
    fn decode(decoder: &mut Decoder<'b>) -> Result<Self, Error> {
        let mut map = IndexMap::with_hasher(S::default());
        match decoder.map()? {
            Some(len) => {
                for _ in 0..len {
                    let key = K::decode(decoder)?;
                    let value = V::decode(decoder)?;
                    map.insert(key, value);
                }
            }
            None => {
                while !decoder.is_break() {
                    let key = K::decode(decoder)?;
                    let value = V::decode(decoder)?;
                    map.insert(key, value);
                }
                decoder.read_break()?;
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
        encoder.array(self.len())?;
        for value in self.iter() {
            value.encode(encoder)?;
        }
        Ok(())
    }
}

impl<'b, T, S> Decode<'b> for IndexSet<T, S>
where
    T: Decode<'b> + Eq + Hash,
    S: BuildHasher + Default,
{
    fn decode(decoder: &mut Decoder<'b>) -> Result<Self, Error> {
        let mut set = IndexSet::with_hasher(S::default());
        match decoder.array()? {
            Some(len) => {
                for _ in 0..len {
                    set.insert(T::decode(decoder)?);
                }
            }
            None => {
                while !decoder.is_break() {
                    set.insert(T::decode(decoder)?);
                }
                decoder.read_break()?;
            }
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
        let bytes = ::cbor::to_vec(&map);
        let decoded: IndexMap<String, i64> = ::cbor::from_slice(&bytes).unwrap();
        assert_eq!(decoded, map);

        let set: IndexSet<i64> = [3, 1, 2].into_iter().collect();
        let bytes = ::cbor::to_vec(&set);
        let decoded: IndexSet<i64> = ::cbor::from_slice(&bytes).unwrap();
        assert_eq!(decoded, set);
    }
}
