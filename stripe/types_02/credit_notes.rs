#[cfg(feature = "credit_notes")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCreditNoteParamsEmailType {
    CreditNote,
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "credit_notes")]
impl CreateCreditNoteParamsEmailType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCreditNoteParamsEmailType::CreditNote => "credit_note",
            CreateCreditNoteParamsEmailType::None => "none",
            CreateCreditNoteParamsEmailType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "credit_notes")]
impl<'json> json::Decode<'json> for CreateCreditNoteParamsEmailType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCreditNoteParamsEmailType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "credit_note" => CreateCreditNoteParamsEmailType::CreditNote,
            "none" => CreateCreditNoteParamsEmailType::None,
            other => CreateCreditNoteParamsEmailType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "credit_notes")]
impl crate::params::FormEncode for CreateCreditNoteParamsEmailType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "credit_notes")]
#[derive(Debug, Clone, Default)]
pub struct ListCreditNoteLinesParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "credit_notes")]
impl crate::params::FormEncode for ListCreditNoteLinesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(feature = "credit_notes")]
#[derive(Debug, Clone, Default)]
pub struct ListCreditNotesParamsCreated {
    #[doc = "Minimum value to filter by (exclusive)"]
    pub gt: Option<i64>,
    #[doc = "Minimum value to filter by (inclusive)"]
    pub gte: Option<i64>,
    #[doc = "Maximum value to filter by (exclusive)"]
    pub lt: Option<i64>,
    #[doc = "Maximum value to filter by (inclusive)"]
    pub lte: Option<i64>,
}

#[cfg(feature = "credit_notes")]
impl crate::params::FormEncode for ListCreditNotesParamsCreated {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gt", self.gt.as_ref());
        form.field_opt("gte", self.gte.as_ref());
        form.field_opt("lt", self.lt.as_ref());
        form.field_opt("lte", self.lte.as_ref());
    }
}

#[cfg(feature = "credit_notes")]
#[derive(Debug, Clone, Default)]
pub struct UpdateCreditNoteParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Credit note memo."]
    pub memo: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[cfg(feature = "credit_notes")]
impl crate::params::FormEncode for UpdateCreditNoteParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("memo", self.memo.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
    }
}
