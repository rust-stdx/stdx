#[cfg(feature = "credit_notes")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCreditNoteParamsLinesItemType {
    CustomLineItem,
    InvoiceLineItem,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "credit_notes")]
impl CreateCreditNoteParamsLinesItemType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCreditNoteParamsLinesItemType::CustomLineItem => "custom_line_item",
            CreateCreditNoteParamsLinesItemType::InvoiceLineItem => "invoice_line_item",
            CreateCreditNoteParamsLinesItemType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "credit_notes")]
impl<'json> json::Decode<'json> for CreateCreditNoteParamsLinesItemType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCreditNoteParamsLinesItemType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "custom_line_item" => CreateCreditNoteParamsLinesItemType::CustomLineItem,
            "invoice_line_item" => CreateCreditNoteParamsLinesItemType::InvoiceLineItem,
            other => CreateCreditNoteParamsLinesItemType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "credit_notes")]
impl crate::params::FormEncode for CreateCreditNoteParamsLinesItemType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "credit_notes")]
#[derive(Debug, Clone, Default)]
pub struct CreateCreditNoteParamsRefundsItemPaymentRecordRefund {
    #[doc = "The ID of the PaymentRecord with the refund to link to this credit note."]
    pub payment_record: String,
    #[doc = "The PaymentRecord refund group to link to this credit note."]
    pub refund_group: String,
}

#[cfg(feature = "credit_notes")]
impl crate::params::FormEncode for CreateCreditNoteParamsRefundsItemPaymentRecordRefund {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("payment_record", &self.payment_record);
        form.field("refund_group", &self.refund_group);
    }
}

#[cfg(feature = "credit_notes")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCreditNoteParamsRefundsItemType {
    PaymentRecordRefund,
    Refund,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "credit_notes")]
impl CreateCreditNoteParamsRefundsItemType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCreditNoteParamsRefundsItemType::PaymentRecordRefund => "payment_record_refund",
            CreateCreditNoteParamsRefundsItemType::Refund => "refund",
            CreateCreditNoteParamsRefundsItemType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "credit_notes")]
impl<'json> json::Decode<'json> for CreateCreditNoteParamsRefundsItemType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCreditNoteParamsRefundsItemType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "payment_record_refund" => CreateCreditNoteParamsRefundsItemType::PaymentRecordRefund,
            "refund" => CreateCreditNoteParamsRefundsItemType::Refund,
            other => CreateCreditNoteParamsRefundsItemType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "credit_notes")]
impl crate::params::FormEncode for CreateCreditNoteParamsRefundsItemType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "credit_notes")]
#[derive(Debug, Clone, Default)]
pub struct CreateCreditNoteParamsShippingCost {
    #[doc = "The ID of the shipping rate to use for this order."]
    pub shipping_rate: Option<String>,
}

#[cfg(feature = "credit_notes")]
impl crate::params::FormEncode for CreateCreditNoteParamsShippingCost {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("shipping_rate", self.shipping_rate.as_ref());
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum CreditNoteLineItemObject {
    CreditNoteLineItem,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl CreditNoteLineItemObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreditNoteLineItemObject::CreditNoteLineItem => "credit_note_line_item",
            CreditNoteLineItemObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl Default for CreditNoteLineItemObject {
    fn default() -> Self {
        CreditNoteLineItemObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl<'json> json::Decode<'json> for CreditNoteLineItemObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreditNoteLineItemObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "credit_note_line_item" => CreditNoteLineItemObject::CreditNoteLineItem,
            other => CreditNoteLineItemObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum CreditNoteReason {
    Duplicate,
    Fraudulent,
    OrderChange,
    ProductUnsatisfactory,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl CreditNoteReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreditNoteReason::Duplicate => "duplicate",
            CreditNoteReason::Fraudulent => "fraudulent",
            CreditNoteReason::OrderChange => "order_change",
            CreditNoteReason::ProductUnsatisfactory => "product_unsatisfactory",
            CreditNoteReason::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl<'json> json::Decode<'json> for CreditNoteReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreditNoteReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "duplicate" => CreditNoteReason::Duplicate,
            "fraudulent" => CreditNoteReason::Fraudulent,
            "order_change" => CreditNoteReason::OrderChange,
            "product_unsatisfactory" => CreditNoteReason::ProductUnsatisfactory,
            other => CreditNoteReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum CreditNoteStatus {
    Issued,
    Void,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl CreditNoteStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreditNoteStatus::Issued => "issued",
            CreditNoteStatus::Void => "void",
            CreditNoteStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl Default for CreditNoteStatus {
    fn default() -> Self {
        CreditNoteStatus::Unknown(String::new())
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl<'json> json::Decode<'json> for CreditNoteStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreditNoteStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "issued" => CreditNoteStatus::Issued,
            "void" => CreditNoteStatus::Void,
            other => CreditNoteStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum CreditNoteType {
    Mixed,
    PostPayment,
    PrePayment,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl CreditNoteType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreditNoteType::Mixed => "mixed",
            CreditNoteType::PostPayment => "post_payment",
            CreditNoteType::PrePayment => "pre_payment",
            CreditNoteType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl Default for CreditNoteType {
    fn default() -> Self {
        CreditNoteType::Unknown(String::new())
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl<'json> json::Decode<'json> for CreditNoteType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreditNoteType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "mixed" => CreditNoteType::Mixed,
            "post_payment" => CreditNoteType::PostPayment,
            "pre_payment" => CreditNoteType::PrePayment,
            other => CreditNoteType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum CreditNotesPretaxCreditAmountType {
    CreditBalanceTransaction,
    Discount,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl CreditNotesPretaxCreditAmountType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreditNotesPretaxCreditAmountType::CreditBalanceTransaction => "credit_balance_transaction",
            CreditNotesPretaxCreditAmountType::Discount => "discount",
            CreditNotesPretaxCreditAmountType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl Default for CreditNotesPretaxCreditAmountType {
    fn default() -> Self {
        CreditNotesPretaxCreditAmountType::Unknown(String::new())
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl<'json> json::Decode<'json> for CreditNotesPretaxCreditAmountType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreditNotesPretaxCreditAmountType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "credit_balance_transaction" => CreditNotesPretaxCreditAmountType::CreditBalanceTransaction,
            "discount" => CreditNotesPretaxCreditAmountType::Discount,
            other => CreditNotesPretaxCreditAmountType::Unknown(other.to_string()),
        })
    }
}
