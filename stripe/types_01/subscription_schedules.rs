#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionScheduleParamsDefaultSettingsBillingThresholds {
    #[doc = "Monetary threshold that triggers the subscription to advance to a new billing period"]
    pub amount_gte: Option<i64>,
    #[doc = "Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached."]
    pub reset_billing_cycle_anchor: Option<bool>,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsDefaultSettingsBillingThresholds {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_gte", self.amount_gte.as_ref());
        form.field_opt("reset_billing_cycle_anchor", self.reset_billing_cycle_anchor.as_ref());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionScheduleParamsDefaultSettingsCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscription_schedules")]
impl CreateSubscriptionScheduleParamsDefaultSettingsCollectionMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionScheduleParamsDefaultSettingsCollectionMethod::ChargeAutomatically => {
                "charge_automatically"
            }
            CreateSubscriptionScheduleParamsDefaultSettingsCollectionMethod::SendInvoice => "send_invoice",
            CreateSubscriptionScheduleParamsDefaultSettingsCollectionMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscription_schedules")]
impl<'json> json::Decode<'json> for CreateSubscriptionScheduleParamsDefaultSettingsCollectionMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionScheduleParamsDefaultSettingsCollectionMethod::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "charge_automatically" => {
                CreateSubscriptionScheduleParamsDefaultSettingsCollectionMethod::ChargeAutomatically
            }
            "send_invoice" => CreateSubscriptionScheduleParamsDefaultSettingsCollectionMethod::SendInvoice,
            other => CreateSubscriptionScheduleParamsDefaultSettingsCollectionMethod::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsDefaultSettingsCollectionMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionScheduleParamsDefaultSettingsTransferData {
    #[doc = "A non-negative decimal between 0 and 100, with at most two decimal places."]
    pub amount_percent: Option<f64>,
    #[doc = "ID of an existing, connected Stripe account."]
    pub destination: String,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsDefaultSettingsTransferData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_percent", self.amount_percent.as_ref());
        form.field("destination", &self.destination);
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionScheduleParamsEndBehavior {
    Cancel,
    None,
    Release,
    Renew,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscription_schedules")]
impl CreateSubscriptionScheduleParamsEndBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionScheduleParamsEndBehavior::Cancel => "cancel",
            CreateSubscriptionScheduleParamsEndBehavior::None => "none",
            CreateSubscriptionScheduleParamsEndBehavior::Release => "release",
            CreateSubscriptionScheduleParamsEndBehavior::Renew => "renew",
            CreateSubscriptionScheduleParamsEndBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscription_schedules")]
impl<'json> json::Decode<'json> for CreateSubscriptionScheduleParamsEndBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionScheduleParamsEndBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cancel" => CreateSubscriptionScheduleParamsEndBehavior::Cancel,
            "none" => CreateSubscriptionScheduleParamsEndBehavior::None,
            "release" => CreateSubscriptionScheduleParamsEndBehavior::Release,
            "renew" => CreateSubscriptionScheduleParamsEndBehavior::Renew,
            other => CreateSubscriptionScheduleParamsEndBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsEndBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsBillForUnusedTimeFromType {
    ItemCurrentPeriodStart,
    None,
    PauseAt,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscription_schedules")]
impl CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsBillForUnusedTimeFromType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsBillForUnusedTimeFromType::ItemCurrentPeriodStart => "item_current_period_start",
            CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsBillForUnusedTimeFromType::None => "none",
            CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsBillForUnusedTimeFromType::PauseAt => "pause_at",
            CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsBillForUnusedTimeFromType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscription_schedules")]
impl<'json> json::Decode<'json>
    for CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsBillForUnusedTimeFromType
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsBillForUnusedTimeFromType::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "item_current_period_start" => CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsBillForUnusedTimeFromType::ItemCurrentPeriodStart,
            "none" => CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsBillForUnusedTimeFromType::None,
            "pause_at" => CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsBillForUnusedTimeFromType::PauseAt,
            other => CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsBillForUnusedTimeFromType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode
    for CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsBillForUnusedTimeFromType
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionScheduleParamsPauseSchedulesItemResumeSettingsPaymentBehavior {
    ResumeOnPaymentAttempt,
    ResumeOnPaymentSuccess,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscription_schedules")]
impl CreateSubscriptionScheduleParamsPauseSchedulesItemResumeSettingsPaymentBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionScheduleParamsPauseSchedulesItemResumeSettingsPaymentBehavior::ResumeOnPaymentAttempt => {
                "resume_on_payment_attempt"
            }
            CreateSubscriptionScheduleParamsPauseSchedulesItemResumeSettingsPaymentBehavior::ResumeOnPaymentSuccess => {
                "resume_on_payment_success"
            }
            CreateSubscriptionScheduleParamsPauseSchedulesItemResumeSettingsPaymentBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscription_schedules")]
impl<'json> json::Decode<'json> for CreateSubscriptionScheduleParamsPauseSchedulesItemResumeSettingsPaymentBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSubscriptionScheduleParamsPauseSchedulesItemResumeSettingsPaymentBehavior::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "resume_on_payment_attempt" => {
                CreateSubscriptionScheduleParamsPauseSchedulesItemResumeSettingsPaymentBehavior::ResumeOnPaymentAttempt
            }
            "resume_on_payment_success" => {
                CreateSubscriptionScheduleParamsPauseSchedulesItemResumeSettingsPaymentBehavior::ResumeOnPaymentSuccess
            }
            other => CreateSubscriptionScheduleParamsPauseSchedulesItemResumeSettingsPaymentBehavior::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsPauseSchedulesItemResumeSettingsPaymentBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemDiscountsItem {
    #[doc = "ID of the coupon to create a new discount for."]
    pub coupon: Option<String>,
    #[doc = "ID of an existing discount on the object (or one of its ancestors) to reuse."]
    pub discount: Option<String>,
    #[doc = "ID of the promotion code to create a new discount for."]
    pub promotion_code: Option<String>,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemDiscountsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("coupon", self.coupon.as_ref());
        form.field_opt("discount", self.discount.as_ref());
        form.field_opt("promotion_code", self.promotion_code.as_ref());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodStartType {
    MaxItemPeriodStart,
    PhaseStart,
    Timestamp,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscription_schedules")]
impl CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodStartType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodStartType::MaxItemPeriodStart => {
                "max_item_period_start"
            }
            CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodStartType::PhaseStart => "phase_start",
            CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodStartType::Timestamp => "timestamp",
            CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodStartType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscription_schedules")]
impl<'json> json::Decode<'json> for CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodStartType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodStartType::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "max_item_period_start" => {
                CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodStartType::MaxItemPeriodStart
            }
            "phase_start" => CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodStartType::PhaseStart,
            "timestamp" => CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodStartType::Timestamp,
            other => {
                CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodStartType::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodStartType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscription_schedules")]
impl CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceDataTaxBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceDataTaxBehavior::Exclusive => "exclusive",
            CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceDataTaxBehavior::Inclusive => "inclusive",
            CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceDataTaxBehavior::Unspecified => {
                "unspecified"
            }
            CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceDataTaxBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscription_schedules")]
impl<'json> json::Decode<'json> for CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceDataTaxBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceDataTaxBehavior::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exclusive" => CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceDataTaxBehavior::Exclusive,
            "inclusive" => CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceDataTaxBehavior::Inclusive,
            "unspecified" => {
                CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceDataTaxBehavior::Unspecified
            }
            other => CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceDataTaxBehavior::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceDataTaxBehavior {
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
pub enum SubscriptionSchedulePhaseConfigurationProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
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
impl SubscriptionSchedulePhaseConfigurationProrationBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionSchedulePhaseConfigurationProrationBehavior::AlwaysInvoice => "always_invoice",
            SubscriptionSchedulePhaseConfigurationProrationBehavior::CreateProrations => "create_prorations",
            SubscriptionSchedulePhaseConfigurationProrationBehavior::None => "none",
            SubscriptionSchedulePhaseConfigurationProrationBehavior::Unknown(value) => value,
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
impl Default for SubscriptionSchedulePhaseConfigurationProrationBehavior {
    fn default() -> Self {
        SubscriptionSchedulePhaseConfigurationProrationBehavior::Unknown(String::new())
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
impl<'json> json::Decode<'json> for SubscriptionSchedulePhaseConfigurationProrationBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionSchedulePhaseConfigurationProrationBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always_invoice" => SubscriptionSchedulePhaseConfigurationProrationBehavior::AlwaysInvoice,
            "create_prorations" => SubscriptionSchedulePhaseConfigurationProrationBehavior::CreateProrations,
            "none" => SubscriptionSchedulePhaseConfigurationProrationBehavior::None,
            other => SubscriptionSchedulePhaseConfigurationProrationBehavior::Unknown(other.to_string()),
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
pub enum SubscriptionSchedulesResourcePauseScheduleOutstandingUsageThroughType {
    None,
    PauseAt,
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
impl SubscriptionSchedulesResourcePauseScheduleOutstandingUsageThroughType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionSchedulesResourcePauseScheduleOutstandingUsageThroughType::None => "none",
            SubscriptionSchedulesResourcePauseScheduleOutstandingUsageThroughType::PauseAt => "pause_at",
            SubscriptionSchedulesResourcePauseScheduleOutstandingUsageThroughType::Unknown(value) => value,
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
impl Default for SubscriptionSchedulesResourcePauseScheduleOutstandingUsageThroughType {
    fn default() -> Self {
        SubscriptionSchedulesResourcePauseScheduleOutstandingUsageThroughType::Unknown(String::new())
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
impl<'json> json::Decode<'json> for SubscriptionSchedulesResourcePauseScheduleOutstandingUsageThroughType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionSchedulesResourcePauseScheduleOutstandingUsageThroughType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "none" => SubscriptionSchedulesResourcePauseScheduleOutstandingUsageThroughType::None,
            "pause_at" => SubscriptionSchedulesResourcePauseScheduleOutstandingUsageThroughType::PauseAt,
            other => SubscriptionSchedulesResourcePauseScheduleOutstandingUsageThroughType::Unknown(other.to_string()),
        })
    }
}
