//! `heapless` container support.

use crate::{
    decode::{Decode, Decoder},
    encode::{Encode, Encoder, Write},
    error::{ErrorKind, Result},
};

impl<'a, T: Decode<'a>, const N: usize> Decode<'a> for heapless::Vec<T, N> {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.begin_array()?;
        let mut values = heapless::Vec::new();
        let mut first = true;
        while decoder.array_next(first)? {
            first = false;
            let value = decoder.decode::<T>()?;
            values
                .push(value)
                .map_err(|_| decoder.error(ErrorKind::ContainerLimitExceeded))?;
        }
        Ok(values)
    }
}

impl<T: Encode, const N: usize> Encode for heapless::Vec<T, N> {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.begin_array()?;
        for (index, value) in self.iter().enumerate() {
            encoder.comma(index == 0)?;
            value.encode(encoder)?;
        }
        encoder.end_array()
    }
}

impl<'a, const N: usize> Decode<'a> for heapless::String<N> {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        let mut buffer = [0u8; N];
        let text = decoder.decode_str_into(&mut buffer)?;
        text.parse::<heapless::String<N>>()
            .map_err(|_| decoder.error(ErrorKind::ContainerLimitExceeded))
    }
}

impl<const N: usize> Encode for heapless::String<N> {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}
