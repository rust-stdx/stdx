#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionScheduleParamsBillingModeType {
    Classic,
    Flexible,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscription_schedules")]
impl CreateSubscriptionScheduleParamsBillingModeType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionScheduleParamsBillingModeType::Classic => "classic",
            CreateSubscriptionScheduleParamsBillingModeType::Flexible => "flexible",
            CreateSubscriptionScheduleParamsBillingModeType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscription_schedules")]
impl<'json> json::Decode<'json> for CreateSubscriptionScheduleParamsBillingModeType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionScheduleParamsBillingModeType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "classic" => CreateSubscriptionScheduleParamsBillingModeType::Classic,
            "flexible" => CreateSubscriptionScheduleParamsBillingModeType::Flexible,
            other => CreateSubscriptionScheduleParamsBillingModeType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsBillingModeType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionScheduleParamsPauseSchedulesItemPausePauseAtType {
    Now,
    Timestamp,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscription_schedules")]
impl CreateSubscriptionScheduleParamsPauseSchedulesItemPausePauseAtType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionScheduleParamsPauseSchedulesItemPausePauseAtType::Now => "now",
            CreateSubscriptionScheduleParamsPauseSchedulesItemPausePauseAtType::Timestamp => "timestamp",
            CreateSubscriptionScheduleParamsPauseSchedulesItemPausePauseAtType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscription_schedules")]
impl<'json> json::Decode<'json> for CreateSubscriptionScheduleParamsPauseSchedulesItemPausePauseAtType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionScheduleParamsPauseSchedulesItemPausePauseAtType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "now" => CreateSubscriptionScheduleParamsPauseSchedulesItemPausePauseAtType::Now,
            "timestamp" => CreateSubscriptionScheduleParamsPauseSchedulesItemPausePauseAtType::Timestamp,
            other => CreateSubscriptionScheduleParamsPauseSchedulesItemPausePauseAtType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsPauseSchedulesItemPausePauseAtType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsType {
    Subscription,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscription_schedules")]
impl CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsType::Subscription => "subscription",
            CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscription_schedules")]
impl<'json> json::Decode<'json> for CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "subscription" => CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsType::Subscription,
            other => CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodEndType {
    MinItemPeriodEnd,
    PhaseEnd,
    Timestamp,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscription_schedules")]
impl CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodEndType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodEndType::MinItemPeriodEnd => {
                "min_item_period_end"
            }
            CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodEndType::PhaseEnd => "phase_end",
            CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodEndType::Timestamp => "timestamp",
            CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodEndType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscription_schedules")]
impl<'json> json::Decode<'json> for CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodEndType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodEndType::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "min_item_period_end" => {
                CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodEndType::MinItemPeriodEnd
            }
            "phase_end" => CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodEndType::PhaseEnd,
            "timestamp" => CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodEndType::Timestamp,
            other => {
                CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodEndType::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodEndType {
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SubscriptionScheduleCurrentPhase {
    #[doc = "The end of this phase of the subscription schedule."]
    #[json(default)]
    pub end_date: i64,
    #[doc = "The start of this phase of the subscription schedule."]
    #[json(default)]
    pub start_date: i64,
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
pub enum SubscriptionSchedulePhaseConfigurationCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
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
impl SubscriptionSchedulePhaseConfigurationCollectionMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionSchedulePhaseConfigurationCollectionMethod::ChargeAutomatically => "charge_automatically",
            SubscriptionSchedulePhaseConfigurationCollectionMethod::SendInvoice => "send_invoice",
            SubscriptionSchedulePhaseConfigurationCollectionMethod::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for SubscriptionSchedulePhaseConfigurationCollectionMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionSchedulePhaseConfigurationCollectionMethod::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "charge_automatically" => SubscriptionSchedulePhaseConfigurationCollectionMethod::ChargeAutomatically,
            "send_invoice" => SubscriptionSchedulePhaseConfigurationCollectionMethod::SendInvoice,
            other => SubscriptionSchedulePhaseConfigurationCollectionMethod::Unknown(other.to_string()),
        })
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SubscriptionSchedulesResourcePauseScheduleStatusError {
    #[doc = "A machine-readable error code."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A description of the error."]
    #[json(default)]
    pub message: String,
}
