#[cfg(feature = "quotes")]
#[derive(Debug, Clone, Default)]
pub struct CreateQuoteParamsFromQuote {
    #[doc = "Whether this quote is a revision of the previous quote."]
    pub is_revision: Option<bool>,
    #[doc = "The `id` of the quote that will be cloned."]
    pub quote: String,
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for CreateQuoteParamsFromQuote {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("is_revision", self.is_revision.as_ref());
        form.field("quote", &self.quote);
    }
}

#[cfg(feature = "quotes")]
#[derive(Debug, Clone, Default)]
pub struct FinalizeQuoteParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "A future timestamp on which the quote will be canceled if in `open` or `draft` status."]
    pub expires_at: Option<i64>,
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for FinalizeQuoteParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("expires_at", self.expires_at.as_ref());
    }
}

#[cfg(feature = "quotes")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListQuotesParamsStatus {
    Accepted,
    Canceled,
    Draft,
    Open,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "quotes")]
impl ListQuotesParamsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListQuotesParamsStatus::Accepted => "accepted",
            ListQuotesParamsStatus::Canceled => "canceled",
            ListQuotesParamsStatus::Draft => "draft",
            ListQuotesParamsStatus::Open => "open",
            ListQuotesParamsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "quotes")]
impl<'json> json::Decode<'json> for ListQuotesParamsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListQuotesParamsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "accepted" => ListQuotesParamsStatus::Accepted,
            "canceled" => ListQuotesParamsStatus::Canceled,
            "draft" => ListQuotesParamsStatus::Draft,
            "open" => ListQuotesParamsStatus::Open,
            other => ListQuotesParamsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for ListQuotesParamsStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(any(feature = "events", feature = "quotes"))]
#[derive(Debug, Clone, PartialEq)]
pub enum QuotesResourceSubscriptionDataBillingModeType {
    Classic,
    Flexible,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "quotes"))]
impl QuotesResourceSubscriptionDataBillingModeType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            QuotesResourceSubscriptionDataBillingModeType::Classic => "classic",
            QuotesResourceSubscriptionDataBillingModeType::Flexible => "flexible",
            QuotesResourceSubscriptionDataBillingModeType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "quotes"))]
impl Default for QuotesResourceSubscriptionDataBillingModeType {
    fn default() -> Self {
        QuotesResourceSubscriptionDataBillingModeType::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "quotes"))]
impl<'json> json::Decode<'json> for QuotesResourceSubscriptionDataBillingModeType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(QuotesResourceSubscriptionDataBillingModeType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "classic" => QuotesResourceSubscriptionDataBillingModeType::Classic,
            "flexible" => QuotesResourceSubscriptionDataBillingModeType::Flexible,
            other => QuotesResourceSubscriptionDataBillingModeType::Unknown(other.to_string()),
        })
    }
}
