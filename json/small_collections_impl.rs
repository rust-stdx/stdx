//! `small_collections` support.

use small_collections::{SmallString, SmallVec};

use crate::{
    decode::{Decode, Decoder},
    encode::{Encode, Encoder, Write},
    error::Result,
};

impl<'a, T: Decode<'a>, const N: usize> Decode<'a> for SmallVec<T, N> {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.begin_array()?;
        let mut values = SmallVec::new();
        let mut first = true;
        while decoder.array_next(first)? {
            first = false;
            values.push(decoder.decode::<T>()?);
        }
        Ok(values)
    }
}

impl<T: Encode, const N: usize> Encode for SmallVec<T, N> {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.begin_array()?;
        for (index, value) in self.iter().enumerate() {
            encoder.comma(index == 0)?;
            value.encode(encoder)?;
        }
        encoder.end_array()
    }
}

impl<'a, const N: usize> Decode<'a> for SmallString<N> {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        let text = decoder.decode_string()?;
        Ok(SmallString::from_string(text))
    }
}

impl<const N: usize> Encode for SmallString<N> {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}
