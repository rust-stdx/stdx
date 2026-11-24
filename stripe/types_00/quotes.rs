#[cfg(feature = "quotes")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateQuoteParamsAutomaticTaxLiabilityType {
    Account,
    SelfValue,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "quotes")]
impl CreateQuoteParamsAutomaticTaxLiabilityType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateQuoteParamsAutomaticTaxLiabilityType::Account => "account",
            CreateQuoteParamsAutomaticTaxLiabilityType::SelfValue => "self",
            CreateQuoteParamsAutomaticTaxLiabilityType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "quotes")]
impl<'json> json::Decode<'json> for CreateQuoteParamsAutomaticTaxLiabilityType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateQuoteParamsAutomaticTaxLiabilityType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account" => CreateQuoteParamsAutomaticTaxLiabilityType::Account,
            "self" => CreateQuoteParamsAutomaticTaxLiabilityType::SelfValue,
            other => CreateQuoteParamsAutomaticTaxLiabilityType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for CreateQuoteParamsAutomaticTaxLiabilityType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(any(feature = "events", feature = "quotes"))]
#[derive(Debug, Clone, PartialEq)]
pub enum QuoteObject {
    Quote,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "quotes"))]
impl QuoteObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            QuoteObject::Quote => "quote",
            QuoteObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "quotes"))]
impl Default for QuoteObject {
    fn default() -> Self {
        QuoteObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "quotes"))]
impl<'json> json::Decode<'json> for QuoteObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(QuoteObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "quote" => QuoteObject::Quote,
            other => QuoteObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "quotes"))]
#[derive(Debug, Clone, PartialEq)]
pub enum QuoteStatus {
    Accepted,
    Canceled,
    Draft,
    Open,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "quotes"))]
impl QuoteStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            QuoteStatus::Accepted => "accepted",
            QuoteStatus::Canceled => "canceled",
            QuoteStatus::Draft => "draft",
            QuoteStatus::Open => "open",
            QuoteStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "quotes"))]
impl Default for QuoteStatus {
    fn default() -> Self {
        QuoteStatus::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "quotes"))]
impl<'json> json::Decode<'json> for QuoteStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(QuoteStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "accepted" => QuoteStatus::Accepted,
            "canceled" => QuoteStatus::Canceled,
            "draft" => QuoteStatus::Draft,
            "open" => QuoteStatus::Open,
            other => QuoteStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "quotes"))]
#[derive(Debug, Clone, PartialEq)]
pub enum QuotesResourceRecurringInterval {
    Day,
    Month,
    Week,
    Year,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "quotes"))]
impl QuotesResourceRecurringInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            QuotesResourceRecurringInterval::Day => "day",
            QuotesResourceRecurringInterval::Month => "month",
            QuotesResourceRecurringInterval::Week => "week",
            QuotesResourceRecurringInterval::Year => "year",
            QuotesResourceRecurringInterval::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "quotes"))]
impl Default for QuotesResourceRecurringInterval {
    fn default() -> Self {
        QuotesResourceRecurringInterval::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "quotes"))]
impl<'json> json::Decode<'json> for QuotesResourceRecurringInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(QuotesResourceRecurringInterval::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "day" => QuotesResourceRecurringInterval::Day,
            "month" => QuotesResourceRecurringInterval::Month,
            "week" => QuotesResourceRecurringInterval::Week,
            "year" => QuotesResourceRecurringInterval::Year,
            other => QuotesResourceRecurringInterval::Unknown(other.to_string()),
        })
    }
}
