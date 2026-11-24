#[cfg(feature = "quotes")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateQuoteParamsSubscriptionDataBillingModeFlexibleProrationDiscounts {
    Included,
    Itemized,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "quotes")]
impl CreateQuoteParamsSubscriptionDataBillingModeFlexibleProrationDiscounts {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateQuoteParamsSubscriptionDataBillingModeFlexibleProrationDiscounts::Included => "included",
            CreateQuoteParamsSubscriptionDataBillingModeFlexibleProrationDiscounts::Itemized => "itemized",
            CreateQuoteParamsSubscriptionDataBillingModeFlexibleProrationDiscounts::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "quotes")]
impl<'json> json::Decode<'json> for CreateQuoteParamsSubscriptionDataBillingModeFlexibleProrationDiscounts {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateQuoteParamsSubscriptionDataBillingModeFlexibleProrationDiscounts::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "included" => CreateQuoteParamsSubscriptionDataBillingModeFlexibleProrationDiscounts::Included,
            "itemized" => CreateQuoteParamsSubscriptionDataBillingModeFlexibleProrationDiscounts::Itemized,
            other => CreateQuoteParamsSubscriptionDataBillingModeFlexibleProrationDiscounts::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for CreateQuoteParamsSubscriptionDataBillingModeFlexibleProrationDiscounts {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(any(feature = "events", feature = "quotes"))]
#[derive(Debug, Clone, PartialEq)]
pub enum QuoteCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "quotes"))]
impl QuoteCollectionMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            QuoteCollectionMethod::ChargeAutomatically => "charge_automatically",
            QuoteCollectionMethod::SendInvoice => "send_invoice",
            QuoteCollectionMethod::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "quotes"))]
impl Default for QuoteCollectionMethod {
    fn default() -> Self {
        QuoteCollectionMethod::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "quotes"))]
impl<'json> json::Decode<'json> for QuoteCollectionMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(QuoteCollectionMethod::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "charge_automatically" => QuoteCollectionMethod::ChargeAutomatically,
            "send_invoice" => QuoteCollectionMethod::SendInvoice,
            other => QuoteCollectionMethod::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "quotes"))]
#[derive(Debug, Clone, PartialEq)]
pub enum QuotesResourceAutomaticTaxStatus {
    Complete,
    Failed,
    RequiresLocationInputs,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "quotes"))]
impl QuotesResourceAutomaticTaxStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            QuotesResourceAutomaticTaxStatus::Complete => "complete",
            QuotesResourceAutomaticTaxStatus::Failed => "failed",
            QuotesResourceAutomaticTaxStatus::RequiresLocationInputs => "requires_location_inputs",
            QuotesResourceAutomaticTaxStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "quotes"))]
impl<'json> json::Decode<'json> for QuotesResourceAutomaticTaxStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(QuotesResourceAutomaticTaxStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "complete" => QuotesResourceAutomaticTaxStatus::Complete,
            "failed" => QuotesResourceAutomaticTaxStatus::Failed,
            "requires_location_inputs" => QuotesResourceAutomaticTaxStatus::RequiresLocationInputs,
            other => QuotesResourceAutomaticTaxStatus::Unknown(other.to_string()),
        })
    }
}
