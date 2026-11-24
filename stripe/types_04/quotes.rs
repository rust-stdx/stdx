#[cfg(feature = "quotes")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateQuoteParamsLineItemsItemPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "quotes")]
impl CreateQuoteParamsLineItemsItemPriceDataTaxBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateQuoteParamsLineItemsItemPriceDataTaxBehavior::Exclusive => "exclusive",
            CreateQuoteParamsLineItemsItemPriceDataTaxBehavior::Inclusive => "inclusive",
            CreateQuoteParamsLineItemsItemPriceDataTaxBehavior::Unspecified => "unspecified",
            CreateQuoteParamsLineItemsItemPriceDataTaxBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "quotes")]
impl<'json> json::Decode<'json> for CreateQuoteParamsLineItemsItemPriceDataTaxBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateQuoteParamsLineItemsItemPriceDataTaxBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exclusive" => CreateQuoteParamsLineItemsItemPriceDataTaxBehavior::Exclusive,
            "inclusive" => CreateQuoteParamsLineItemsItemPriceDataTaxBehavior::Inclusive,
            "unspecified" => CreateQuoteParamsLineItemsItemPriceDataTaxBehavior::Unspecified,
            other => CreateQuoteParamsLineItemsItemPriceDataTaxBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for CreateQuoteParamsLineItemsItemPriceDataTaxBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "quotes")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateQuoteParamsSubscriptionDataBillingModeType {
    Classic,
    Flexible,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "quotes")]
impl CreateQuoteParamsSubscriptionDataBillingModeType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateQuoteParamsSubscriptionDataBillingModeType::Classic => "classic",
            CreateQuoteParamsSubscriptionDataBillingModeType::Flexible => "flexible",
            CreateQuoteParamsSubscriptionDataBillingModeType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "quotes")]
impl<'json> json::Decode<'json> for CreateQuoteParamsSubscriptionDataBillingModeType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateQuoteParamsSubscriptionDataBillingModeType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "classic" => CreateQuoteParamsSubscriptionDataBillingModeType::Classic,
            "flexible" => CreateQuoteParamsSubscriptionDataBillingModeType::Flexible,
            other => CreateQuoteParamsSubscriptionDataBillingModeType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for CreateQuoteParamsSubscriptionDataBillingModeType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "quotes")]
#[derive(Debug, Clone, Default)]
pub struct GetQuoteParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for GetQuoteParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "quotes")]
#[derive(Debug, Clone, Default)]
pub struct UpdateQuoteParamsSubscriptionData {
    #[doc = "The subscription's description, meant to be displayable to the customer."]
    pub description: Option<json::OwnedValue>,
    #[doc = "When creating a new subscription, the date of which the subscription schedule will start after the quote is accepted."]
    pub effective_date: Option<json::OwnedValue>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that will set metadata on the subscription or subscrip..."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Integer representing the number of trial period days before the customer is charged for the first time."]
    pub trial_period_days: Option<json::OwnedValue>,
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for UpdateQuoteParamsSubscriptionData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
    }
}
