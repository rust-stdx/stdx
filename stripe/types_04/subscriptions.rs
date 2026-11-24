#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsAddInvoiceItemsItemPeriodStartType {
    MaxItemPeriodStart,
    Now,
    Timestamp,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsAddInvoiceItemsItemPeriodStartType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsAddInvoiceItemsItemPeriodStartType::MaxItemPeriodStart => "max_item_period_start",
            CreateSubscriptionParamsAddInvoiceItemsItemPeriodStartType::Now => "now",
            CreateSubscriptionParamsAddInvoiceItemsItemPeriodStartType::Timestamp => "timestamp",
            CreateSubscriptionParamsAddInvoiceItemsItemPeriodStartType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for CreateSubscriptionParamsAddInvoiceItemsItemPeriodStartType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionParamsAddInvoiceItemsItemPeriodStartType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "max_item_period_start" => CreateSubscriptionParamsAddInvoiceItemsItemPeriodStartType::MaxItemPeriodStart,
            "now" => CreateSubscriptionParamsAddInvoiceItemsItemPeriodStartType::Now,
            "timestamp" => CreateSubscriptionParamsAddInvoiceItemsItemPeriodStartType::Timestamp,
            other => CreateSubscriptionParamsAddInvoiceItemsItemPeriodStartType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsAddInvoiceItemsItemPeriodStartType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress {
    #[doc = "City, district, suburb, town, or village."]
    pub city: Option<String>,
    #[doc = "Two-letter country code."]
    pub country: Option<String>,
    #[doc = "Address line 1 (for example, street, PO Box, or company name)."]
    pub line1: Option<String>,
    #[doc = "Address line 2 (for example, apartment, suite, unit, or building)."]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    pub postal_code: Option<String>,
    #[doc = "State, county, province, or region."]
    pub state: Option<String>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("city", self.city.as_ref());
        form.field_opt("country", self.country.as_ref());
        form.field_opt("line1", self.line1.as_ref());
        form.field_opt("line2", self.line2.as_ref());
        form.field_opt("postal_code", self.postal_code.as_ref());
        form.field_opt("state", self.state.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem {
    Balances,
    Ownership,
    Transactions,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Balances => "balances",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Ownership => "ownership",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Transactions => "transactions",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json>
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "balances" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Balances,
            "ownership" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Ownership,
            "transactions" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Transactions,
            other => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListSubscriptionsParamsStatus {
    Active,
    All,
    Canceled,
    Ended,
    Incomplete,
    IncompleteExpired,
    PastDue,
    Paused,
    Trialing,
    Unpaid,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl ListSubscriptionsParamsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListSubscriptionsParamsStatus::Active => "active",
            ListSubscriptionsParamsStatus::All => "all",
            ListSubscriptionsParamsStatus::Canceled => "canceled",
            ListSubscriptionsParamsStatus::Ended => "ended",
            ListSubscriptionsParamsStatus::Incomplete => "incomplete",
            ListSubscriptionsParamsStatus::IncompleteExpired => "incomplete_expired",
            ListSubscriptionsParamsStatus::PastDue => "past_due",
            ListSubscriptionsParamsStatus::Paused => "paused",
            ListSubscriptionsParamsStatus::Trialing => "trialing",
            ListSubscriptionsParamsStatus::Unpaid => "unpaid",
            ListSubscriptionsParamsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for ListSubscriptionsParamsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListSubscriptionsParamsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "active" => ListSubscriptionsParamsStatus::Active,
            "all" => ListSubscriptionsParamsStatus::All,
            "canceled" => ListSubscriptionsParamsStatus::Canceled,
            "ended" => ListSubscriptionsParamsStatus::Ended,
            "incomplete" => ListSubscriptionsParamsStatus::Incomplete,
            "incomplete_expired" => ListSubscriptionsParamsStatus::IncompleteExpired,
            "past_due" => ListSubscriptionsParamsStatus::PastDue,
            "paused" => ListSubscriptionsParamsStatus::Paused,
            "trialing" => ListSubscriptionsParamsStatus::Trialing,
            "unpaid" => ListSubscriptionsParamsStatus::Unpaid,
            other => ListSubscriptionsParamsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for ListSubscriptionsParamsStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum MigrateSubscriptionParamsBillingModeType {
    Flexible,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl MigrateSubscriptionParamsBillingModeType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            MigrateSubscriptionParamsBillingModeType::Flexible => "flexible",
            MigrateSubscriptionParamsBillingModeType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for MigrateSubscriptionParamsBillingModeType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(MigrateSubscriptionParamsBillingModeType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "flexible" => MigrateSubscriptionParamsBillingModeType::Flexible,
            other => MigrateSubscriptionParamsBillingModeType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for MigrateSubscriptionParamsBillingModeType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum PauseSubscriptionParamsBillForUnusedTimeFromType {
    ItemCurrentPeriodStart,
    None,
    Now,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl PauseSubscriptionParamsBillForUnusedTimeFromType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PauseSubscriptionParamsBillForUnusedTimeFromType::ItemCurrentPeriodStart => "item_current_period_start",
            PauseSubscriptionParamsBillForUnusedTimeFromType::None => "none",
            PauseSubscriptionParamsBillForUnusedTimeFromType::Now => "now",
            PauseSubscriptionParamsBillForUnusedTimeFromType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for PauseSubscriptionParamsBillForUnusedTimeFromType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PauseSubscriptionParamsBillForUnusedTimeFromType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "item_current_period_start" => PauseSubscriptionParamsBillForUnusedTimeFromType::ItemCurrentPeriodStart,
            "none" => PauseSubscriptionParamsBillForUnusedTimeFromType::None,
            "now" => PauseSubscriptionParamsBillForUnusedTimeFromType::Now,
            other => PauseSubscriptionParamsBillForUnusedTimeFromType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for PauseSubscriptionParamsBillForUnusedTimeFromType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum PauseSubscriptionParamsType {
    Subscription,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl PauseSubscriptionParamsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PauseSubscriptionParamsType::Subscription => "subscription",
            PauseSubscriptionParamsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for PauseSubscriptionParamsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PauseSubscriptionParamsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "subscription" => PauseSubscriptionParamsType::Subscription,
            other => PauseSubscriptionParamsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for PauseSubscriptionParamsType {
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
pub enum SubscriptionPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    EftposAu,
    Girocard,
    Interac,
    Jcb,
    Link,
    Mastercard,
    Unionpay,
    UnknownValue,
    Visa,
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
impl SubscriptionPaymentMethodOptionsCardNetwork {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionPaymentMethodOptionsCardNetwork::Amex => "amex",
            SubscriptionPaymentMethodOptionsCardNetwork::CartesBancaires => "cartes_bancaires",
            SubscriptionPaymentMethodOptionsCardNetwork::Diners => "diners",
            SubscriptionPaymentMethodOptionsCardNetwork::Discover => "discover",
            SubscriptionPaymentMethodOptionsCardNetwork::EftposAu => "eftpos_au",
            SubscriptionPaymentMethodOptionsCardNetwork::Girocard => "girocard",
            SubscriptionPaymentMethodOptionsCardNetwork::Interac => "interac",
            SubscriptionPaymentMethodOptionsCardNetwork::Jcb => "jcb",
            SubscriptionPaymentMethodOptionsCardNetwork::Link => "link",
            SubscriptionPaymentMethodOptionsCardNetwork::Mastercard => "mastercard",
            SubscriptionPaymentMethodOptionsCardNetwork::Unionpay => "unionpay",
            SubscriptionPaymentMethodOptionsCardNetwork::UnknownValue => "unknown",
            SubscriptionPaymentMethodOptionsCardNetwork::Visa => "visa",
            SubscriptionPaymentMethodOptionsCardNetwork::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for SubscriptionPaymentMethodOptionsCardNetwork {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionPaymentMethodOptionsCardNetwork::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "amex" => SubscriptionPaymentMethodOptionsCardNetwork::Amex,
            "cartes_bancaires" => SubscriptionPaymentMethodOptionsCardNetwork::CartesBancaires,
            "diners" => SubscriptionPaymentMethodOptionsCardNetwork::Diners,
            "discover" => SubscriptionPaymentMethodOptionsCardNetwork::Discover,
            "eftpos_au" => SubscriptionPaymentMethodOptionsCardNetwork::EftposAu,
            "girocard" => SubscriptionPaymentMethodOptionsCardNetwork::Girocard,
            "interac" => SubscriptionPaymentMethodOptionsCardNetwork::Interac,
            "jcb" => SubscriptionPaymentMethodOptionsCardNetwork::Jcb,
            "link" => SubscriptionPaymentMethodOptionsCardNetwork::Link,
            "mastercard" => SubscriptionPaymentMethodOptionsCardNetwork::Mastercard,
            "unionpay" => SubscriptionPaymentMethodOptionsCardNetwork::Unionpay,
            "unknown" => SubscriptionPaymentMethodOptionsCardNetwork::UnknownValue,
            "visa" => SubscriptionPaymentMethodOptionsCardNetwork::Visa,
            other => SubscriptionPaymentMethodOptionsCardNetwork::Unknown(other.to_string()),
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
pub enum SubscriptionPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
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
impl SubscriptionPaymentMethodOptionsCardRequestThreeDSecure {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionPaymentMethodOptionsCardRequestThreeDSecure::Any => "any",
            SubscriptionPaymentMethodOptionsCardRequestThreeDSecure::Automatic => "automatic",
            SubscriptionPaymentMethodOptionsCardRequestThreeDSecure::Challenge => "challenge",
            SubscriptionPaymentMethodOptionsCardRequestThreeDSecure::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for SubscriptionPaymentMethodOptionsCardRequestThreeDSecure {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionPaymentMethodOptionsCardRequestThreeDSecure::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "any" => SubscriptionPaymentMethodOptionsCardRequestThreeDSecure::Any,
            "automatic" => SubscriptionPaymentMethodOptionsCardRequestThreeDSecure::Automatic,
            "challenge" => SubscriptionPaymentMethodOptionsCardRequestThreeDSecure::Challenge,
            other => SubscriptionPaymentMethodOptionsCardRequestThreeDSecure::Unknown(other.to_string()),
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
pub struct SubscriptionPaymentMethodOptionsMandateOptionsBlik {
    #[doc = "Date when the mandate expires and no further payments will be charged."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
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
pub enum SubscriptionsResourceBillingModeFlexibleProrationDiscounts {
    Included,
    Itemized,
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
impl SubscriptionsResourceBillingModeFlexibleProrationDiscounts {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionsResourceBillingModeFlexibleProrationDiscounts::Included => "included",
            SubscriptionsResourceBillingModeFlexibleProrationDiscounts::Itemized => "itemized",
            SubscriptionsResourceBillingModeFlexibleProrationDiscounts::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for SubscriptionsResourceBillingModeFlexibleProrationDiscounts {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionsResourceBillingModeFlexibleProrationDiscounts::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "included" => SubscriptionsResourceBillingModeFlexibleProrationDiscounts::Included,
            "itemized" => SubscriptionsResourceBillingModeFlexibleProrationDiscounts::Itemized,
            other => SubscriptionsResourceBillingModeFlexibleProrationDiscounts::Unknown(other.to_string()),
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
pub enum SubscriptionsResourcePauseCollectionBehavior {
    KeepAsDraft,
    MarkUncollectible,
    Void,
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
impl SubscriptionsResourcePauseCollectionBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionsResourcePauseCollectionBehavior::KeepAsDraft => "keep_as_draft",
            SubscriptionsResourcePauseCollectionBehavior::MarkUncollectible => "mark_uncollectible",
            SubscriptionsResourcePauseCollectionBehavior::Void => "void",
            SubscriptionsResourcePauseCollectionBehavior::Unknown(value) => value,
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
impl Default for SubscriptionsResourcePauseCollectionBehavior {
    fn default() -> Self {
        SubscriptionsResourcePauseCollectionBehavior::Unknown(String::new())
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
impl<'json> json::Decode<'json> for SubscriptionsResourcePauseCollectionBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionsResourcePauseCollectionBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "keep_as_draft" => SubscriptionsResourcePauseCollectionBehavior::KeepAsDraft,
            "mark_uncollectible" => SubscriptionsResourcePauseCollectionBehavior::MarkUncollectible,
            "void" => SubscriptionsResourcePauseCollectionBehavior::Void,
            other => SubscriptionsResourcePauseCollectionBehavior::Unknown(other.to_string()),
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
pub enum SubscriptionsResourcePausedStatusDetailsSubscriptionType {
    FinalPaymentFailure,
    FirstPaymentFailure,
    PauseRequested,
    System,
    TrialEndWithoutPaymentMethod,
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
impl SubscriptionsResourcePausedStatusDetailsSubscriptionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionsResourcePausedStatusDetailsSubscriptionType::FinalPaymentFailure => "final_payment_failure",
            SubscriptionsResourcePausedStatusDetailsSubscriptionType::FirstPaymentFailure => "first_payment_failure",
            SubscriptionsResourcePausedStatusDetailsSubscriptionType::PauseRequested => "pause_requested",
            SubscriptionsResourcePausedStatusDetailsSubscriptionType::System => "system",
            SubscriptionsResourcePausedStatusDetailsSubscriptionType::TrialEndWithoutPaymentMethod => {
                "trial_end_without_payment_method"
            }
            SubscriptionsResourcePausedStatusDetailsSubscriptionType::Unknown(value) => value,
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
impl Default for SubscriptionsResourcePausedStatusDetailsSubscriptionType {
    fn default() -> Self {
        SubscriptionsResourcePausedStatusDetailsSubscriptionType::Unknown(String::new())
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
impl<'json> json::Decode<'json> for SubscriptionsResourcePausedStatusDetailsSubscriptionType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionsResourcePausedStatusDetailsSubscriptionType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "final_payment_failure" => SubscriptionsResourcePausedStatusDetailsSubscriptionType::FinalPaymentFailure,
            "first_payment_failure" => SubscriptionsResourcePausedStatusDetailsSubscriptionType::FirstPaymentFailure,
            "pause_requested" => SubscriptionsResourcePausedStatusDetailsSubscriptionType::PauseRequested,
            "system" => SubscriptionsResourcePausedStatusDetailsSubscriptionType::System,
            "trial_end_without_payment_method" => {
                SubscriptionsResourcePausedStatusDetailsSubscriptionType::TrialEndWithoutPaymentMethod
            }
            other => SubscriptionsResourcePausedStatusDetailsSubscriptionType::Unknown(other.to_string()),
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
pub enum SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    Off,
    OnSubscription,
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
impl SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod::Off => "off",
            SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod::OnSubscription => "on_subscription",
            SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "off" => SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod::Off,
            "on_subscription" => SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod::OnSubscription,
            other => SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod::Unknown(other.to_string()),
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
pub struct SubscriptionsResourceSubscriptionPresentmentDetails {
    #[doc = "Currency used for customer payments."]
    #[json(default)]
    pub presentment_currency: String,
}
