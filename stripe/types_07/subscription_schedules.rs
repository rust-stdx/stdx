#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsBillFor {
    #[doc = "Controls whether to collect metered usage accrued up to the pause date."]
    pub outstanding_usage_through: Option<stripe_types_06::subscription_schedules::CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsBillForOutstandingUsageThrough>,
    #[doc = "Controls how unused time on subscription items is credited when pausing."]
    pub unused_time_from: Option<stripe_types_06::subscription_schedules::CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsBillForUnusedTimeFrom>,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsBillFor {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("outstanding_usage_through", self.outstanding_usage_through.as_ref());
        form.field_opt("unused_time_from", self.unused_time_from.as_ref());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriod {
    #[doc = "End of the invoice item period."]
    pub end: stripe_types_06::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodEnd,
    #[doc = "Start of the invoice item period."]
    pub start: stripe_types_05::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodStart,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("end", &self.end);
        form.field("start", &self.start);
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionScheduleParamsPhasesItemItemsItemPriceDataRecurring {
    #[doc = "Specifies billing frequency."]
    pub interval: stripe_types_05::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemItemsItemPriceDataRecurringInterval,
    #[doc = "The number of intervals between subscription billings."]
    pub interval_count: Option<i64>,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsPhasesItemItemsItemPriceDataRecurring {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("interval", &self.interval);
        form.field_opt("interval_count", self.interval_count.as_ref());
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
pub struct SubscriptionSchedulesResourcePauseScheduleBillFor {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub outstanding_usage_through: Option<
        stripe_types_05::subscription_schedules::SubscriptionSchedulesResourcePauseScheduleOutstandingUsageThrough,
    >,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub unused_time_from:
        Option<stripe_types_05::subscription_schedules::SubscriptionSchedulesResourcePauseScheduleUnusedTimeFrom>,
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
pub struct SubscriptionSchedulesResourcePauseScheduleResume {
    #[doc = "Time at which the subscription resumes."]
    #[json(default)]
    pub resume_at: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub settings:
        Option<stripe_types_06::subscription_schedules::SubscriptionSchedulesResourcePauseScheduleResumeSettings>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<stripe_types_05::subscription_schedules::SubscriptionSchedulesResourcePauseScheduleResumeStatus>,
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone)]
pub struct UpdateSubscriptionScheduleParamsDefaultSettingsInvoiceSettingsIssuer {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_05::subscription_schedules::UpdateSubscriptionScheduleParamsDefaultSettingsInvoiceSettingsIssuerType,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for UpdateSubscriptionScheduleParamsDefaultSettingsInvoiceSettingsIssuer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone)]
pub struct UpdateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodEnd {
    #[doc = "A precise Unix timestamp for the end of the invoice item period."]
    pub timestamp: Option<i64>,
    #[doc = "Select how to calculate the end of the invoice item period."]
    pub r#type: stripe_types_05::subscription_schedules::UpdateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodEndType,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for UpdateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodEnd {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("timestamp", self.timestamp.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone)]
pub struct UpdateSubscriptionScheduleParamsPhasesItemAutomaticTaxLiability {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type:
        stripe_types_06::subscription_schedules::UpdateSubscriptionScheduleParamsPhasesItemAutomaticTaxLiabilityType,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for UpdateSubscriptionScheduleParamsPhasesItemAutomaticTaxLiability {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}
