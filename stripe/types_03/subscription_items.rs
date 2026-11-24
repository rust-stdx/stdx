#[cfg(feature = "subscription_items")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionItemParamsPaymentBehavior {
    AllowIncomplete,
    DefaultIncomplete,
    ErrorIfIncomplete,
    PendingIfIncomplete,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscription_items")]
impl CreateSubscriptionItemParamsPaymentBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionItemParamsPaymentBehavior::AllowIncomplete => "allow_incomplete",
            CreateSubscriptionItemParamsPaymentBehavior::DefaultIncomplete => "default_incomplete",
            CreateSubscriptionItemParamsPaymentBehavior::ErrorIfIncomplete => "error_if_incomplete",
            CreateSubscriptionItemParamsPaymentBehavior::PendingIfIncomplete => "pending_if_incomplete",
            CreateSubscriptionItemParamsPaymentBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscription_items")]
impl<'json> json::Decode<'json> for CreateSubscriptionItemParamsPaymentBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionItemParamsPaymentBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "allow_incomplete" => CreateSubscriptionItemParamsPaymentBehavior::AllowIncomplete,
            "default_incomplete" => CreateSubscriptionItemParamsPaymentBehavior::DefaultIncomplete,
            "error_if_incomplete" => CreateSubscriptionItemParamsPaymentBehavior::ErrorIfIncomplete,
            "pending_if_incomplete" => CreateSubscriptionItemParamsPaymentBehavior::PendingIfIncomplete,
            other => CreateSubscriptionItemParamsPaymentBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscription_items")]
impl crate::params::FormEncode for CreateSubscriptionItemParamsPaymentBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscription_items")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionItemParamsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscription_items")]
impl CreateSubscriptionItemParamsPriceDataRecurringInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionItemParamsPriceDataRecurringInterval::Day => "day",
            CreateSubscriptionItemParamsPriceDataRecurringInterval::Month => "month",
            CreateSubscriptionItemParamsPriceDataRecurringInterval::Week => "week",
            CreateSubscriptionItemParamsPriceDataRecurringInterval::Year => "year",
            CreateSubscriptionItemParamsPriceDataRecurringInterval::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscription_items")]
impl<'json> json::Decode<'json> for CreateSubscriptionItemParamsPriceDataRecurringInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionItemParamsPriceDataRecurringInterval::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "day" => CreateSubscriptionItemParamsPriceDataRecurringInterval::Day,
            "month" => CreateSubscriptionItemParamsPriceDataRecurringInterval::Month,
            "week" => CreateSubscriptionItemParamsPriceDataRecurringInterval::Week,
            "year" => CreateSubscriptionItemParamsPriceDataRecurringInterval::Year,
            other => CreateSubscriptionItemParamsPriceDataRecurringInterval::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscription_items")]
impl crate::params::FormEncode for CreateSubscriptionItemParamsPriceDataRecurringInterval {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscription_items")]
#[derive(Debug, Clone, Default)]
pub struct GetSubscriptionItemParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "subscription_items")]
impl crate::params::FormEncode for GetSubscriptionItemParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "subscription_items")]
#[derive(Debug, Clone, Default)]
pub struct ListSubscriptionItemsParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub subscription: String,
}

#[cfg(feature = "subscription_items")]
impl crate::params::FormEncode for ListSubscriptionItemsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field("subscription", &self.subscription);
    }
}
