#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionScheduleParamsBillingModeFlexibleProrationDiscounts {
    Included,
    Itemized,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscription_schedules")]
impl CreateSubscriptionScheduleParamsBillingModeFlexibleProrationDiscounts {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionScheduleParamsBillingModeFlexibleProrationDiscounts::Included => "included",
            CreateSubscriptionScheduleParamsBillingModeFlexibleProrationDiscounts::Itemized => "itemized",
            CreateSubscriptionScheduleParamsBillingModeFlexibleProrationDiscounts::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscription_schedules")]
impl<'json> json::Decode<'json> for CreateSubscriptionScheduleParamsBillingModeFlexibleProrationDiscounts {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionScheduleParamsBillingModeFlexibleProrationDiscounts::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "included" => CreateSubscriptionScheduleParamsBillingModeFlexibleProrationDiscounts::Included,
            "itemized" => CreateSubscriptionScheduleParamsBillingModeFlexibleProrationDiscounts::Itemized,
            other => CreateSubscriptionScheduleParamsBillingModeFlexibleProrationDiscounts::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsBillingModeFlexibleProrationDiscounts {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiabilityType {
    Account,
    SelfValue,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscription_schedules")]
impl CreateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiabilityType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiabilityType::Account => "account",
            CreateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiabilityType::SelfValue => "self",
            CreateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiabilityType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscription_schedules")]
impl<'json> json::Decode<'json> for CreateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiabilityType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiabilityType::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account" => CreateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiabilityType::Account,
            "self" => CreateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiabilityType::SelfValue,
            other => {
                CreateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiabilityType::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiabilityType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionScheduleParamsDefaultSettingsBillingCycleAnchor {
    Automatic,
    PhaseStart,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscription_schedules")]
impl CreateSubscriptionScheduleParamsDefaultSettingsBillingCycleAnchor {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionScheduleParamsDefaultSettingsBillingCycleAnchor::Automatic => "automatic",
            CreateSubscriptionScheduleParamsDefaultSettingsBillingCycleAnchor::PhaseStart => "phase_start",
            CreateSubscriptionScheduleParamsDefaultSettingsBillingCycleAnchor::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscription_schedules")]
impl<'json> json::Decode<'json> for CreateSubscriptionScheduleParamsDefaultSettingsBillingCycleAnchor {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionScheduleParamsDefaultSettingsBillingCycleAnchor::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "automatic" => CreateSubscriptionScheduleParamsDefaultSettingsBillingCycleAnchor::Automatic,
            "phase_start" => CreateSubscriptionScheduleParamsDefaultSettingsBillingCycleAnchor::PhaseStart,
            other => CreateSubscriptionScheduleParamsDefaultSettingsBillingCycleAnchor::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsDefaultSettingsBillingCycleAnchor {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtDurationInterval {
    Day,
    Month,
    Week,
    Year,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscription_schedules")]
impl CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtDurationInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtDurationInterval::Day => "day",
            CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtDurationInterval::Month => "month",
            CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtDurationInterval::Week => "week",
            CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtDurationInterval::Year => "year",
            CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtDurationInterval::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscription_schedules")]
impl<'json> json::Decode<'json> for CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtDurationInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtDurationInterval::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "day" => CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtDurationInterval::Day,
            "month" => CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtDurationInterval::Month,
            "week" => CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtDurationInterval::Week,
            "year" => CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtDurationInterval::Year,
            other => CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtDurationInterval::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtDurationInterval {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtType {
    Duration,
    Now,
    Timestamp,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscription_schedules")]
impl CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtType::Duration => "duration",
            CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtType::Now => "now",
            CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtType::Timestamp => "timestamp",
            CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscription_schedules")]
impl<'json> json::Decode<'json> for CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "duration" => CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtType::Duration,
            "now" => CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtType::Now,
            "timestamp" => CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtType::Timestamp,
            other => CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior {
    ResumeOnPaymentAttempt,
    ResumeOnPaymentSuccess,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior::ResumeOnPaymentAttempt => {
                "resume_on_payment_attempt"
            }
            SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior::ResumeOnPaymentSuccess => {
                "resume_on_payment_success"
            }
            SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl Default for SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior {
    fn default() -> Self {
        SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl<'json> json::Decode<'json> for SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "resume_on_payment_attempt" => {
                SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior::ResumeOnPaymentAttempt
            }
            "resume_on_payment_success" => {
                SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior::ResumeOnPaymentSuccess
            }
            other => {
                SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior::Unknown(other.to_string())
            }
        })
    }
}
