#[cfg(feature = "shipping_rates")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateShippingRateParamsDeliveryEstimateMaximumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "shipping_rates")]
impl CreateShippingRateParamsDeliveryEstimateMaximumUnit {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateShippingRateParamsDeliveryEstimateMaximumUnit::BusinessDay => "business_day",
            CreateShippingRateParamsDeliveryEstimateMaximumUnit::Day => "day",
            CreateShippingRateParamsDeliveryEstimateMaximumUnit::Hour => "hour",
            CreateShippingRateParamsDeliveryEstimateMaximumUnit::Month => "month",
            CreateShippingRateParamsDeliveryEstimateMaximumUnit::Week => "week",
            CreateShippingRateParamsDeliveryEstimateMaximumUnit::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "shipping_rates")]
impl<'json> json::Decode<'json> for CreateShippingRateParamsDeliveryEstimateMaximumUnit {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateShippingRateParamsDeliveryEstimateMaximumUnit::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "business_day" => CreateShippingRateParamsDeliveryEstimateMaximumUnit::BusinessDay,
            "day" => CreateShippingRateParamsDeliveryEstimateMaximumUnit::Day,
            "hour" => CreateShippingRateParamsDeliveryEstimateMaximumUnit::Hour,
            "month" => CreateShippingRateParamsDeliveryEstimateMaximumUnit::Month,
            "week" => CreateShippingRateParamsDeliveryEstimateMaximumUnit::Week,
            other => CreateShippingRateParamsDeliveryEstimateMaximumUnit::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "shipping_rates")]
impl crate::params::FormEncode for CreateShippingRateParamsDeliveryEstimateMaximumUnit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "shipping_rates")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateShippingRateParamsFixedAmountCurrencyOptionsValueTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "shipping_rates")]
impl CreateShippingRateParamsFixedAmountCurrencyOptionsValueTaxBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateShippingRateParamsFixedAmountCurrencyOptionsValueTaxBehavior::Exclusive => "exclusive",
            CreateShippingRateParamsFixedAmountCurrencyOptionsValueTaxBehavior::Inclusive => "inclusive",
            CreateShippingRateParamsFixedAmountCurrencyOptionsValueTaxBehavior::Unspecified => "unspecified",
            CreateShippingRateParamsFixedAmountCurrencyOptionsValueTaxBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "shipping_rates")]
impl<'json> json::Decode<'json> for CreateShippingRateParamsFixedAmountCurrencyOptionsValueTaxBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateShippingRateParamsFixedAmountCurrencyOptionsValueTaxBehavior::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exclusive" => CreateShippingRateParamsFixedAmountCurrencyOptionsValueTaxBehavior::Exclusive,
            "inclusive" => CreateShippingRateParamsFixedAmountCurrencyOptionsValueTaxBehavior::Inclusive,
            "unspecified" => CreateShippingRateParamsFixedAmountCurrencyOptionsValueTaxBehavior::Unspecified,
            other => CreateShippingRateParamsFixedAmountCurrencyOptionsValueTaxBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "shipping_rates")]
impl crate::params::FormEncode for CreateShippingRateParamsFixedAmountCurrencyOptionsValueTaxBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
