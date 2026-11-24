#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum CreditNoteObject {
    CreditNote,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl CreditNoteObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreditNoteObject::CreditNote => "credit_note",
            CreditNoteObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl Default for CreditNoteObject {
    fn default() -> Self {
        CreditNoteObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl<'json> json::Decode<'json> for CreditNoteObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreditNoteObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "credit_note" => CreditNoteObject::CreditNote,
            other => CreditNoteObject::Unknown(other.to_string()),
        })
    }
}
