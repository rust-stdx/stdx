#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum CreditNoteLineItemType {
    CustomLineItem,
    InvoiceLineItem,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl CreditNoteLineItemType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreditNoteLineItemType::CustomLineItem => "custom_line_item",
            CreditNoteLineItemType::InvoiceLineItem => "invoice_line_item",
            CreditNoteLineItemType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl Default for CreditNoteLineItemType {
    fn default() -> Self {
        CreditNoteLineItemType::Unknown(String::new())
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl<'json> json::Decode<'json> for CreditNoteLineItemType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreditNoteLineItemType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "custom_line_item" => CreditNoteLineItemType::CustomLineItem,
            "invoice_line_item" => CreditNoteLineItemType::InvoiceLineItem,
            other => CreditNoteLineItemType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CreditNotesPaymentRecordRefund {
    #[doc = "ID of the payment record."]
    #[json(default)]
    pub payment_record: String,
    #[doc = "ID of the refund group."]
    #[json(default)]
    pub refund_group: String,
}

#[cfg(feature = "credit_notes")]
#[derive(Debug, Clone, Default)]
pub struct GetCreditNoteParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "credit_notes")]
impl crate::params::FormEncode for GetCreditNoteParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
