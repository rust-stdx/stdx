#[cfg(feature = "invoiceitems")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateInvoiceitemParamsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoiceitems")]
impl CreateInvoiceitemParamsPriceDataTaxBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateInvoiceitemParamsPriceDataTaxBehavior::Exclusive => "exclusive",
            CreateInvoiceitemParamsPriceDataTaxBehavior::Inclusive => "inclusive",
            CreateInvoiceitemParamsPriceDataTaxBehavior::Unspecified => "unspecified",
            CreateInvoiceitemParamsPriceDataTaxBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoiceitems")]
impl<'json> json::Decode<'json> for CreateInvoiceitemParamsPriceDataTaxBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateInvoiceitemParamsPriceDataTaxBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exclusive" => CreateInvoiceitemParamsPriceDataTaxBehavior::Exclusive,
            "inclusive" => CreateInvoiceitemParamsPriceDataTaxBehavior::Inclusive,
            "unspecified" => CreateInvoiceitemParamsPriceDataTaxBehavior::Unspecified,
            other => CreateInvoiceitemParamsPriceDataTaxBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoiceitems")]
impl crate::params::FormEncode for CreateInvoiceitemParamsPriceDataTaxBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
#[derive(Debug, Clone, PartialEq)]
pub enum InvoiceitemObject {
    Invoiceitem,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
impl InvoiceitemObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InvoiceitemObject::Invoiceitem => "invoiceitem",
            InvoiceitemObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
impl Default for InvoiceitemObject {
    fn default() -> Self {
        InvoiceitemObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
impl<'json> json::Decode<'json> for InvoiceitemObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InvoiceitemObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "invoiceitem" => InvoiceitemObject::Invoiceitem,
            other => InvoiceitemObject::Unknown(other.to_string()),
        })
    }
}
