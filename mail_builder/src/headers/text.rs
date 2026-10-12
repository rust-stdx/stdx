use std::borrow::Cow;

use super::{
    Header,
    fold::{FoldWriter, write_unstructured},
};
use crate::{encoders::encode::EncodingType, writer::Writer};

/// Unstructured text e-mail header.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Text<'x> {
    pub text: Cow<'x, str>,
}

impl<'x> Text<'x> {
    /// Create a new unstructured text header
    pub fn new(text: impl Into<Cow<'x, str>>) -> Self {
        Self {
            text: text.into(),
        }
    }
}

impl<'x, T> From<T> for Text<'x>
where
    T: Into<Cow<'x, str>>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl Header for Text<'_> {
    fn write_header(&self, output: &mut impl Writer, column: usize) {
        let mut folder = FoldWriter::new(output, column);

        match EncodingType::for_header(self.text.as_bytes()) {
            EncodingType::Base64 => folder.write_b_words(&self.text, b""),
            EncodingType::QuotedPrintable(is_ascii) => folder.write_q_words::<false>(&self.text, is_ascii, b""),
            EncodingType::None => write_unstructured(&mut folder, self.text.as_bytes()),
        }

        folder.finish();
    }
}
