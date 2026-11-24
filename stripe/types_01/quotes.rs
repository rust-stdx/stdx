#[cfg(feature = "quotes")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateQuoteParamsCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "quotes")]
impl CreateQuoteParamsCollectionMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateQuoteParamsCollectionMethod::ChargeAutomatically => "charge_automatically",
            CreateQuoteParamsCollectionMethod::SendInvoice => "send_invoice",
            CreateQuoteParamsCollectionMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "quotes")]
impl<'json> json::Decode<'json> for CreateQuoteParamsCollectionMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateQuoteParamsCollectionMethod::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "charge_automatically" => CreateQuoteParamsCollectionMethod::ChargeAutomatically,
            "send_invoice" => CreateQuoteParamsCollectionMethod::SendInvoice,
            other => CreateQuoteParamsCollectionMethod::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for CreateQuoteParamsCollectionMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "quotes")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateQuoteParamsLineItemsItemPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "quotes")]
impl CreateQuoteParamsLineItemsItemPriceDataRecurringInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateQuoteParamsLineItemsItemPriceDataRecurringInterval::Day => "day",
            CreateQuoteParamsLineItemsItemPriceDataRecurringInterval::Month => "month",
            CreateQuoteParamsLineItemsItemPriceDataRecurringInterval::Week => "week",
            CreateQuoteParamsLineItemsItemPriceDataRecurringInterval::Year => "year",
            CreateQuoteParamsLineItemsItemPriceDataRecurringInterval::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "quotes")]
impl<'json> json::Decode<'json> for CreateQuoteParamsLineItemsItemPriceDataRecurringInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateQuoteParamsLineItemsItemPriceDataRecurringInterval::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "day" => CreateQuoteParamsLineItemsItemPriceDataRecurringInterval::Day,
            "month" => CreateQuoteParamsLineItemsItemPriceDataRecurringInterval::Month,
            "week" => CreateQuoteParamsLineItemsItemPriceDataRecurringInterval::Week,
            "year" => CreateQuoteParamsLineItemsItemPriceDataRecurringInterval::Year,
            other => CreateQuoteParamsLineItemsItemPriceDataRecurringInterval::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for CreateQuoteParamsLineItemsItemPriceDataRecurringInterval {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "quotes")]
#[derive(Debug, Clone, Default)]
pub struct CreateQuoteParamsTransferData {
    #[doc = "The amount that will be transferred automatically when the invoice is paid."]
    pub amount: Option<i64>,
    #[doc = "A non-negative decimal between 0 and 100, with at most two decimal places."]
    pub amount_percent: Option<f64>,
    #[doc = "ID of an existing, connected Stripe account."]
    pub destination: String,
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for CreateQuoteParamsTransferData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_percent", self.amount_percent.as_ref());
        form.field("destination", &self.destination);
    }
}

#[cfg(feature = "quotes")]
#[derive(Debug, Clone, Default)]
pub struct ListQuoteComputedUpfrontLineItemsParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for ListQuoteComputedUpfrontLineItemsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(any(feature = "events", feature = "quotes"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct QuotesResourceStatusTransitions {
    #[doc = "The time that the quote was accepted."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<i64>,
    #[doc = "The time that the quote was canceled."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<i64>,
    #[doc = "The time that the quote was finalized."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub finalized_at: Option<i64>,
}
