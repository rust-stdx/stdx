//! `heapless` container support.

use heapless::{LinearMap, String as HeaplessString, Vec as HeaplessVec, index_map::FnvIndexMap};

use crate::{
    bytes::DecodeBytes,
    decode::{Decode, Decoder},
    encode::{Encode, Encoder, Write},
    error::{Error, Result},
};

impl<T: Encode, const N: usize> Encode for HeaplessVec<T, N> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        e.array(self.len())?;
        for item in self {
            item.encode(e)?;
        }
        Ok(())
    }
}

impl<'b, T: Decode<'b>, const N: usize> Decode<'b> for HeaplessVec<T, N> {
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        let mut out = HeaplessVec::new();
        match d.array()? {
            Some(len) => {
                if len > N {
                    return Err(Error::message("array exceeds heapless capacity").at(d.position()));
                }
                for _ in 0..len {
                    out.push(T::decode(d)?)
                        .map_err(|_| Error::message("heapless vector is full").at(d.position()))?;
                }
            }
            None => loop {
                if d.is_break() {
                    d.read_break()?;
                    break;
                }
                out.push(T::decode(d)?)
                    .map_err(|_| Error::message("heapless vector is full").at(d.position()))?;
            },
        }
        Ok(out)
    }
}

impl<const N: usize> Encode for HeaplessString<N> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        e.text(self.as_str())
    }
}

impl<'b, const N: usize> Decode<'b> for HeaplessString<N> {
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        let mut out = HeaplessString::new();
        let head = d.peek_byte()?;
        if head & 0xe0 != 0x60 {
            return Err(Error::type_mismatch(crate::Type::Text, crate::decode::type_of_byte(head)).at(d.position()));
        }
        if head & 0x1f == 31 {
            let start = d.position();
            let iter = d.text_iter()?;
            for chunk in iter {
                let chunk = chunk?;
                out.push_str(chunk)
                    .map_err(|_| Error::message("heapless string is full").at(start))?;
            }
        } else {
            let text = d.text()?;
            out.push_str(text)
                .map_err(|_| Error::message("heapless string is full").at(d.position()))?;
        }
        Ok(out)
    }
}

impl<K: Encode, V: Encode, const N: usize> Encode for FnvIndexMap<K, V, N> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        e.map(self.len())?;
        for (key, value) in self.iter() {
            key.encode(e)?;
            value.encode(e)?;
        }
        Ok(())
    }
}

impl<'b, K, V, const N: usize> Decode<'b> for FnvIndexMap<K, V, N>
where
    K: Decode<'b> + core::hash::Hash + Eq,
    V: Decode<'b>,
{
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        let mut out = FnvIndexMap::new();
        let policy = d.options().duplicate_keys;
        match d.map()? {
            Some(len) => {
                if len > N {
                    return Err(Error::message("map exceeds heapless capacity").at(d.position()));
                }
                for _ in 0..len {
                    insert_heapless(&mut out, K::decode(d)?, V::decode(d)?, policy, d)?;
                }
            }
            None => loop {
                if d.is_break() {
                    d.read_break()?;
                    break;
                }
                let key = K::decode(d)?;
                if d.is_break() {
                    return Err(Error::message("indefinite map has an odd number of items").at(d.position()));
                }
                let value = V::decode(d)?;
                insert_heapless(&mut out, key, value, policy, d)?;
            },
        }
        Ok(out)
    }
}

fn insert_heapless<K, V, S, const N: usize>(
    map: &mut heapless::IndexMap<K, V, S, N>,
    key: K,
    value: V,
    policy: crate::decode::DuplicateKeys,
    d: &Decoder<'_>,
) -> Result<()>
where
    K: core::hash::Hash + Eq,
    S: core::hash::BuildHasher,
{
    if map.contains_key(&key) {
        match policy {
            crate::decode::DuplicateKeys::Error => return Err(Error::duplicate_key().at(d.position())),
            crate::decode::DuplicateKeys::First => return Ok(()),
            crate::decode::DuplicateKeys::Last | crate::decode::DuplicateKeys::Allow => {}
        }
    }
    map.insert(key, value)
        .map_err(|_| Error::message("heapless map is full").at(d.position()))?;
    Ok(())
}

impl<K: Encode + Eq, V: Encode, const N: usize> Encode for LinearMap<K, V, N> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        e.map(self.len())?;
        for (key, value) in self.iter() {
            key.encode(e)?;
            value.encode(e)?;
        }
        Ok(())
    }
}

impl<'b, K, V, const N: usize> Decode<'b> for LinearMap<K, V, N>
where
    K: Decode<'b> + Eq,
    V: Decode<'b>,
{
    fn decode(d: &mut Decoder<'b>) -> Result<Self> {
        let mut out = LinearMap::new();
        let policy = d.options().duplicate_keys;
        match d.map()? {
            Some(len) => {
                if len > N {
                    return Err(Error::message("map exceeds heapless capacity").at(d.position()));
                }
                for _ in 0..len {
                    let key = K::decode(d)?;
                    let value = V::decode(d)?;
                    insert_linear(&mut out, key, value, policy, d)?;
                }
            }
            None => loop {
                if d.is_break() {
                    d.read_break()?;
                    break;
                }
                let key = K::decode(d)?;
                if d.is_break() {
                    return Err(Error::message("indefinite map has an odd number of items").at(d.position()));
                }
                let value = V::decode(d)?;
                insert_linear(&mut out, key, value, policy, d)?;
            },
        }
        Ok(out)
    }
}

fn insert_linear<K: Eq, V, const N: usize>(
    map: &mut LinearMap<K, V, N>,
    key: K,
    value: V,
    policy: crate::decode::DuplicateKeys,
    d: &Decoder<'_>,
) -> Result<()> {
    if map.contains_key(&key) {
        match policy {
            crate::decode::DuplicateKeys::Error => return Err(Error::duplicate_key().at(d.position())),
            crate::decode::DuplicateKeys::First => return Ok(()),
            crate::decode::DuplicateKeys::Last | crate::decode::DuplicateKeys::Allow => {}
        }
    }
    map.insert(key, value)
        .map_err(|_| Error::message("heapless map is full").at(d.position()))?;
    Ok(())
}

impl<'b, const N: usize> DecodeBytes<'b> for HeaplessVec<u8, N> {
    fn decode_bytes(d: &mut Decoder<'b>) -> Result<Self> {
        let mut out = HeaplessVec::new();
        let start = d.position();
        let iter = d.bytes_iter()?;
        for chunk in iter {
            let chunk = chunk?;
            if out.len() + chunk.len() > N {
                return Err(Error::message("byte string exceeds heapless capacity").at(start));
            }
            out.extend_from_slice(chunk)
                .map_err(|_| Error::message("heapless vector is full").at(start))?;
        }
        Ok(out)
    }
}
