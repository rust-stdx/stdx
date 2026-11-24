#[cfg(any(feature = "billing", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum BillingAlertAlertType {
    UsageThreshold,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "billing", feature = "events"))]
impl BillingAlertAlertType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingAlertAlertType::UsageThreshold => "usage_threshold",
            BillingAlertAlertType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
impl Default for BillingAlertAlertType {
    fn default() -> Self {
        BillingAlertAlertType::Unknown(String::new())
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
impl<'json> json::Decode<'json> for BillingAlertAlertType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingAlertAlertType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "usage_threshold" => BillingAlertAlertType::UsageThreshold,
            other => BillingAlertAlertType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum BillingAlertObject {
    BillingAlert,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "billing", feature = "events"))]
impl BillingAlertObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingAlertObject::BillingAlert => "billing.alert",
            BillingAlertObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
impl Default for BillingAlertObject {
    fn default() -> Self {
        BillingAlertObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
impl<'json> json::Decode<'json> for BillingAlertObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingAlertObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "billing.alert" => BillingAlertObject::BillingAlert,
            other => BillingAlertObject::Unknown(other.to_string()),
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
#[derive(Debug, Clone, PartialEq)]
pub enum BillingCreditGrantCategory {
    Paid,
    Promotional,
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
impl BillingCreditGrantCategory {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingCreditGrantCategory::Paid => "paid",
            BillingCreditGrantCategory::Promotional => "promotional",
            BillingCreditGrantCategory::Unknown(value) => value,
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
impl Default for BillingCreditGrantCategory {
    fn default() -> Self {
        BillingCreditGrantCategory::Unknown(String::new())
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
impl<'json> json::Decode<'json> for BillingCreditGrantCategory {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingCreditGrantCategory::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "paid" => BillingCreditGrantCategory::Paid,
            "promotional" => BillingCreditGrantCategory::Promotional,
            other => BillingCreditGrantCategory::Unknown(other.to_string()),
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
#[derive(Debug, Clone, PartialEq)]
pub enum BillingCreditGrantObject {
    BillingCreditGrant,
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
impl BillingCreditGrantObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingCreditGrantObject::BillingCreditGrant => "billing.credit_grant",
            BillingCreditGrantObject::Unknown(value) => value,
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
impl Default for BillingCreditGrantObject {
    fn default() -> Self {
        BillingCreditGrantObject::Unknown(String::new())
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
impl<'json> json::Decode<'json> for BillingCreditGrantObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingCreditGrantObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "billing.credit_grant" => BillingCreditGrantObject::BillingCreditGrant,
            other => BillingCreditGrantObject::Unknown(other.to_string()),
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
pub struct BillingCreditGrantsResourceApplicablePrice {
    #[doc = "Unique identifier for the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, PartialEq)]
pub enum BillingMeterEventSummaryObject {
    BillingMeterEventSummary,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "billing")]
impl BillingMeterEventSummaryObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingMeterEventSummaryObject::BillingMeterEventSummary => "billing.meter_event_summary",
            BillingMeterEventSummaryObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "billing")]
impl Default for BillingMeterEventSummaryObject {
    fn default() -> Self {
        BillingMeterEventSummaryObject::Unknown(String::new())
    }
}

#[cfg(feature = "billing")]
impl<'json> json::Decode<'json> for BillingMeterEventSummaryObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingMeterEventSummaryObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "billing.meter_event_summary" => BillingMeterEventSummaryObject::BillingMeterEventSummary,
            other => BillingMeterEventSummaryObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingMeterResourceBillingMeterStatusTransitions {
    #[doc = "The time the meter was deactivated, if any."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub deactivated_at: Option<i64>,
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, Default)]
pub struct GetBillingAlertParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for GetBillingAlertParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, PartialEq)]
pub enum GetBillingCreditBalanceSummaryParamsFilterApplicabilityScopePriceType {
    Metered,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "billing")]
impl GetBillingCreditBalanceSummaryParamsFilterApplicabilityScopePriceType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            GetBillingCreditBalanceSummaryParamsFilterApplicabilityScopePriceType::Metered => "metered",
            GetBillingCreditBalanceSummaryParamsFilterApplicabilityScopePriceType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "billing")]
impl<'json> json::Decode<'json> for GetBillingCreditBalanceSummaryParamsFilterApplicabilityScopePriceType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(GetBillingCreditBalanceSummaryParamsFilterApplicabilityScopePriceType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "metered" => GetBillingCreditBalanceSummaryParamsFilterApplicabilityScopePriceType::Metered,
            other => GetBillingCreditBalanceSummaryParamsFilterApplicabilityScopePriceType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for GetBillingCreditBalanceSummaryParamsFilterApplicabilityScopePriceType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, Default)]
pub struct ListBillingCreditGrantsParams {
    pub customer: Option<String>,
    pub customer_account: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for ListBillingCreditGrantsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum ThresholdsResourceUsageAlertFilterType {
    Customer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "billing", feature = "events"))]
impl ThresholdsResourceUsageAlertFilterType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThresholdsResourceUsageAlertFilterType::Customer => "customer",
            ThresholdsResourceUsageAlertFilterType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
impl Default for ThresholdsResourceUsageAlertFilterType {
    fn default() -> Self {
        ThresholdsResourceUsageAlertFilterType::Unknown(String::new())
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
impl<'json> json::Decode<'json> for ThresholdsResourceUsageAlertFilterType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThresholdsResourceUsageAlertFilterType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "customer" => ThresholdsResourceUsageAlertFilterType::Customer,
            other => ThresholdsResourceUsageAlertFilterType::Unknown(other.to_string()),
        })
    }
}
