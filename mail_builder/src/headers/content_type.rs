use std::borrow::Cow;

use super::{
    Header,
    fold::{FoldWriter, write_control_free},
    rfc2047::write_parameter,
};
use crate::writer::Writer;

/// MIME Content-Type or Content-Disposition header
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContentType<'x> {
    pub c_type: Cow<'x, str>,
    pub attributes: Vec<(Cow<'x, str>, Cow<'x, str>)>,
}

impl<'x> ContentType<'x> {
    /// Create a new Content-Type or Content-Disposition header
    pub fn new(c_type: impl Into<Cow<'x, str>>) -> Self {
        Self {
            c_type: c_type.into(),
            attributes: Vec::new(),
        }
    }

    /// Set a Content-Type / Content-Disposition attribute
    pub fn attribute(mut self, key: impl Into<Cow<'x, str>>, value: impl Into<Cow<'x, str>>) -> Self {
        self.attributes.push((key.into(), value.into()));
        self
    }

    /// Returns true when the part is text/*
    pub fn is_text(&self) -> bool {
        self.c_type
            .get(..5)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("text/"))
    }

    /// Returns true when the part is an attachment
    pub fn is_attachment(&self) -> bool {
        self.c_type.eq_ignore_ascii_case("attachment")
    }
}

impl Header for ContentType<'_> {
    fn write_header(&self, output: &mut impl Writer, column: usize) {
        let mut folder = FoldWriter::new(output, column);
        write_control_free(&mut folder, self.c_type.as_bytes());

        if let Some((last, head)) = self.attributes.split_last() {
            for (key, value) in head {
                folder.semicolon();
                write_parameter(&mut folder, key, value, 1);
            }
            folder.semicolon();
            write_parameter(&mut folder, &last.0, &last.1, 0);
        }

        folder.finish();
    }
}
