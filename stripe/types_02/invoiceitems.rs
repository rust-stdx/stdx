#[cfg(feature = "invoiceitems")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceitemParamsPeriod {
    #[doc = "The end of the period, which must be greater than or equal to the start."]
    pub end: i64,
    #[doc = "The start of the period."]
    pub start: i64,
}

#[cfg(feature = "invoiceitems")]
impl crate::params::FormEncode for CreateInvoiceitemParamsPeriod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("end", &self.end);
        form.field("start", &self.start);
    }
}

#[cfg(feature = "invoiceitems")]
#[derive(Debug, Clone, Default)]
pub struct GetInvoiceitemParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "invoiceitems")]
impl crate::params::FormEncode for GetInvoiceitemParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
#[derive(Debug, Clone, PartialEq)]
pub enum InvoiceitemFrozenFieldsItem {
    Discounts,
    Pricing,
    Quantity,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
impl InvoiceitemFrozenFieldsItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InvoiceitemFrozenFieldsItem::Discounts => "discounts",
            InvoiceitemFrozenFieldsItem::Pricing => "pricing",
            InvoiceitemFrozenFieldsItem::Quantity => "quantity",
            InvoiceitemFrozenFieldsItem::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
impl<'json> json::Decode<'json> for InvoiceitemFrozenFieldsItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InvoiceitemFrozenFieldsItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "discounts" => InvoiceitemFrozenFieldsItem::Discounts,
            "pricing" => InvoiceitemFrozenFieldsItem::Pricing,
            "quantity" => InvoiceitemFrozenFieldsItem::Quantity,
            other => InvoiceitemFrozenFieldsItem::Unknown(other.to_string()),
        })
    }
}
