#[cfg(feature = "invoiceitems")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceitemParamsPricing {
    #[doc = "The ID of the price object."]
    pub price: Option<String>,
}

#[cfg(feature = "invoiceitems")]
impl crate::params::FormEncode for CreateInvoiceitemParamsPricing {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("price", self.price.as_ref());
    }
}

#[cfg(feature = "invoiceitems")]
#[derive(Debug, Clone, Default)]
pub struct ListInvoiceitemsParamsCreated {
    #[doc = "Minimum value to filter by (exclusive)"]
    pub gt: Option<i64>,
    #[doc = "Minimum value to filter by (inclusive)"]
    pub gte: Option<i64>,
    #[doc = "Maximum value to filter by (exclusive)"]
    pub lt: Option<i64>,
    #[doc = "Maximum value to filter by (inclusive)"]
    pub lte: Option<i64>,
}

#[cfg(feature = "invoiceitems")]
impl crate::params::FormEncode for ListInvoiceitemsParamsCreated {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gt", self.gt.as_ref());
        form.field_opt("gte", self.gte.as_ref());
        form.field_opt("lt", self.lt.as_ref());
        form.field_opt("lte", self.lte.as_ref());
    }
}

#[cfg(feature = "invoiceitems")]
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateInvoiceitemParamsInvoicingRules {
    Unnamed,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoiceitems")]
impl UpdateInvoiceitemParamsInvoicingRules {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            UpdateInvoiceitemParamsInvoicingRules::Unnamed => "",
            UpdateInvoiceitemParamsInvoicingRules::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoiceitems")]
impl<'json> json::Decode<'json> for UpdateInvoiceitemParamsInvoicingRules {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(UpdateInvoiceitemParamsInvoicingRules::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => UpdateInvoiceitemParamsInvoicingRules::Unnamed,
            other => UpdateInvoiceitemParamsInvoicingRules::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoiceitems")]
impl crate::params::FormEncode for UpdateInvoiceitemParamsInvoicingRules {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
