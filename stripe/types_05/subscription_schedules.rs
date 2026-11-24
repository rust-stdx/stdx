#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiability {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_04::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiabilityType,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiability {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "subscription_schedules")]
pub type CreateSubscriptionScheduleParamsDefaultSettingsInvoiceSettingsIssuerType =
    stripe_types_04::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiabilityType;

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionScheduleParamsPauseSchedulesItemPausePauseAt {
    #[doc = "The Unix timestamp at which to pause the subscription."]
    pub timestamp: Option<i64>,
    #[doc = "When to pause the subscription."]
    pub r#type:
        stripe_types_03::subscription_schedules::CreateSubscriptionScheduleParamsPauseSchedulesItemPausePauseAtType,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsPauseSchedulesItemPausePauseAt {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("timestamp", self.timestamp.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtDuration {
    #[doc = "The time unit for the resume duration."]
    pub interval: stripe_types_04::subscription_schedules::CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtDurationInterval,
    #[doc = "The number of intervals after which the subscription resumes."]
    pub interval_count: Option<i64>,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtDuration {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("interval", &self.interval);
        form.field_opt("interval_count", self.interval_count.as_ref());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodStart {
    #[doc = "A precise Unix timestamp for the start of the invoice item period."]
    pub timestamp: Option<i64>,
    #[doc = "Select how to calculate the start of the invoice item period."]
    pub r#type: stripe_types_01::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodStartType,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodStart {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("timestamp", self.timestamp.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceData {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) w..."]
    pub product: String,
    #[doc = "Only required if a default tax behavior..."]
    pub tax_behavior: Option<stripe_types_01::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceDataTaxBehavior>,
    #[doc = "A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge or a negati..."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("currency", &self.currency);
        form.field("product", &self.product);
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("unit_amount", self.unit_amount.as_ref());
        form.field_opt("unit_amount_decimal", self.unit_amount_decimal.as_ref());
    }
}

#[cfg(feature = "subscription_schedules")]
pub type CreateSubscriptionScheduleParamsPhasesItemBillingCycleAnchor =
    stripe_types_04::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsBillingCycleAnchor;

#[cfg(feature = "subscription_schedules")]
pub type CreateSubscriptionScheduleParamsPhasesItemBillingThresholds =
    stripe_types_01::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsBillingThresholds;

#[cfg(feature = "subscription_schedules")]
pub type CreateSubscriptionScheduleParamsPhasesItemCollectionMethod =
    stripe_types_01::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsCollectionMethod;

#[cfg(feature = "subscription_schedules")]
pub type CreateSubscriptionScheduleParamsPhasesItemInvoiceSettingsIssuerType =
    stripe_types_04::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiabilityType;

#[cfg(feature = "subscription_schedules")]
pub type CreateSubscriptionScheduleParamsPhasesItemItemsItemPriceDataRecurringInterval = stripe_types_04::subscription_schedules::CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtDurationInterval;

#[cfg(feature = "subscription_schedules")]
pub type CreateSubscriptionScheduleParamsPhasesItemProrationBehavior = stripe_types_00::subscription_schedules::CreateSubscriptionScheduleParamsPauseSchedulesItemResumeSettingsProrationBehavior;

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
pub struct SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStart {
    #[doc = "A precise Unix timestamp for the start of the invoice item period."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[doc = "Select how to calculate the start of the invoice item period."]
    #[json(rename = "type", default)]
    pub r#type:
        stripe_types_00::subscription_schedules::SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStartType,
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
pub struct SubscriptionSchedulesResourcePauseScheduleOutstandingUsageThrough {
    #[doc = "The type of outstanding usage billing behavior."]
    #[json(rename = "type", default)]
    pub r#type:
        stripe_types_01::subscription_schedules::SubscriptionSchedulesResourcePauseScheduleOutstandingUsageThroughType,
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
pub struct SubscriptionSchedulesResourcePauseSchedulePauseStatus {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<stripe_types_03::subscription_schedules::SubscriptionSchedulesResourcePauseScheduleStatusError>,
    #[doc = "The lifecycle state of the pause operation."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::subscription_schedules::SubscriptionSchedulesResourcePauseSchedulePauseStatusType,
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
pub struct SubscriptionSchedulesResourcePauseScheduleResumeStatus {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<stripe_types_03::subscription_schedules::SubscriptionSchedulesResourcePauseScheduleStatusError>,
    #[doc = "The lifecycle state of the resume operation."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::subscription_schedules::SubscriptionSchedulesResourcePauseScheduleResumeStatusType,
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
pub struct SubscriptionSchedulesResourcePauseScheduleUnusedTimeFrom {
    #[doc = "The type of unused time credit behavior."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::subscription_schedules::SubscriptionSchedulesResourcePauseScheduleUnusedTimeFromType,
}

#[cfg(feature = "subscription_schedules")]
pub type UpdateSubscriptionScheduleParamsDefaultSettingsBillingThresholds =
    stripe_types_01::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsBillingThresholds;

#[cfg(feature = "subscription_schedules")]
pub type UpdateSubscriptionScheduleParamsDefaultSettingsInvoiceSettingsIssuerType =
    stripe_types_04::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiabilityType;

#[cfg(feature = "subscription_schedules")]
pub type UpdateSubscriptionScheduleParamsDefaultSettingsTransferData =
    stripe_types_01::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsTransferData;

#[cfg(feature = "subscription_schedules")]
pub type UpdateSubscriptionScheduleParamsEndBehavior =
    stripe_types_01::subscription_schedules::CreateSubscriptionScheduleParamsEndBehavior;

#[cfg(feature = "subscription_schedules")]
pub type UpdateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodEndType =
    stripe_types_03::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodEndType;

#[cfg(feature = "subscription_schedules")]
pub type UpdateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodStartType = stripe_types_01::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodStartType;

#[cfg(feature = "subscription_schedules")]
pub type UpdateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceDataTaxBehavior = stripe_types_01::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceDataTaxBehavior;

#[cfg(feature = "subscription_schedules")]
pub type UpdateSubscriptionScheduleParamsPhasesItemDurationInterval = stripe_types_04::subscription_schedules::CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtDurationInterval;

#[cfg(feature = "subscription_schedules")]
pub type UpdateSubscriptionScheduleParamsPhasesItemInvoiceSettingsIssuerType =
    stripe_types_04::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiabilityType;

#[cfg(feature = "subscription_schedules")]
pub type UpdateSubscriptionScheduleParamsPhasesItemItemsItemBillingThresholds =
    stripe_types_00::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemItemsItemBillingThresholds;

#[cfg(feature = "subscription_schedules")]
pub type UpdateSubscriptionScheduleParamsPhasesItemTransferData =
    stripe_types_01::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsTransferData;

#[cfg(feature = "subscription_schedules")]
pub type UpdateSubscriptionScheduleParamsProrationBehavior = stripe_types_00::subscription_schedules::CreateSubscriptionScheduleParamsPauseSchedulesItemResumeSettingsProrationBehavior;
