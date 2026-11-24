#[cfg(feature = "credit_notes")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCreditNoteParamsReason {
    Duplicate,
    Fraudulent,
    OrderChange,
    ProductUnsatisfactory,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "credit_notes")]
impl CreateCreditNoteParamsReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCreditNoteParamsReason::Duplicate => "duplicate",
            CreateCreditNoteParamsReason::Fraudulent => "fraudulent",
            CreateCreditNoteParamsReason::OrderChange => "order_change",
            CreateCreditNoteParamsReason::ProductUnsatisfactory => "product_unsatisfactory",
            CreateCreditNoteParamsReason::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "credit_notes")]
impl<'json> json::Decode<'json> for CreateCreditNoteParamsReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCreditNoteParamsReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "duplicate" => CreateCreditNoteParamsReason::Duplicate,
            "fraudulent" => CreateCreditNoteParamsReason::Fraudulent,
            "order_change" => CreateCreditNoteParamsReason::OrderChange,
            "product_unsatisfactory" => CreateCreditNoteParamsReason::ProductUnsatisfactory,
            other => CreateCreditNoteParamsReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "credit_notes")]
impl crate::params::FormEncode for CreateCreditNoteParamsReason {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum CreditNoteRefundType {
    PaymentRecordRefund,
    Refund,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl CreditNoteRefundType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreditNoteRefundType::PaymentRecordRefund => "payment_record_refund",
            CreditNoteRefundType::Refund => "refund",
            CreditNoteRefundType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl<'json> json::Decode<'json> for CreditNoteRefundType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreditNoteRefundType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "payment_record_refund" => CreditNoteRefundType::PaymentRecordRefund,
            "refund" => CreditNoteRefundType::Refund,
            other => CreditNoteRefundType::Unknown(other.to_string()),
        })
    }
}
