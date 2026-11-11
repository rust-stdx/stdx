//! `small_collections` support.

use small_collections::{SmallString, SmallVec};

use crate::{
    decode::{Decode, Decoder},
    encode::{Encode, Encoder, Write},
    error::{ErrorKind, Result},
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
        // Fast path: unescaped strings borrow directly from the input and are
        // copied once into inline storage.
        let start = decoder.position();
        match decoder.decode_str() {
            Ok(text) => return Ok(SmallString::from_str(text)),
            Err(error) if matches!(error.kind(), ErrorKind::Message(_)) => {
                decoder.set_position(start);
            }
            Err(error) => return Err(error),
        }

        // Escaped strings: unescape into a bounded stack buffer so short
        // values do not allocate at all.
        let mut buffer = [0u8; 128];
        match decoder.decode_str_into(&mut buffer) {
            Ok(text) => return Ok(SmallString::from_str(text)),
            Err(error) if error.kind() == ErrorKind::BufferTooSmall => {
                decoder.set_position(start);
            }
            Err(error) => return Err(error),
        }

        // Long escaped value: use the normal heap path, but still move the
        // result inline when it fits.
        let text = decoder.decode_string()?;
        Ok(if text.len() <= N {
            SmallString::from_str(&text)
        } else {
            SmallString::from_string(text)
        })
    }
}

impl<const N: usize> Encode for SmallString<N> {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}
