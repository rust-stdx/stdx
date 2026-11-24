#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionScheduleParamsBillingModeFlexible {
    #[doc = "Controls how invoices and invoice items display proration amounts and discount amounts."]
    pub proration_discounts: Option<
        stripe_types_04::subscription_schedules::CreateSubscriptionScheduleParamsBillingModeFlexibleProrationDiscounts,
    >,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsBillingModeFlexible {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("proration_discounts", self.proration_discounts.as_ref());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsBillForOutstandingUsageThrough {
    #[doc = "Determines whether to collect metered usage accrued up to the pause date."]
    pub r#type: Option<stripe_types_02::subscription_schedules::CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsBillForOutstandingUsageThroughType>,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode
    for CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsBillForOutstandingUsageThrough
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsBillForUnusedTimeFrom {
    #[doc = "Determines which point in the billing period unused time is credited from."]
    pub r#type: Option<stripe_types_01::subscription_schedules::CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsBillForUnusedTimeFromType>,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode
    for CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettingsBillForUnusedTimeFrom
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionScheduleParamsPauseSchedulesItemResumeSettings {
    #[doc = "Controls the billing cycle anchor when the subscription resumes."]
    pub billing_cycle_anchor: Option<stripe_types_02::subscription_schedules::CreateSubscriptionScheduleParamsPauseSchedulesItemResumeSettingsBillingCycleAnchor>,
    #[doc = "Controls whether Stripe attempts payment on the resumption invoice and how payment affects the subscription's status."]
    pub payment_behavior: Option<stripe_types_01::subscription_schedules::CreateSubscriptionScheduleParamsPauseSchedulesItemResumeSettingsPaymentBehavior>,
    #[doc = "Determines how to handle prorations when the subscription resumes."]
    pub proration_behavior: Option<stripe_types_00::subscription_schedules::CreateSubscriptionScheduleParamsPauseSchedulesItemResumeSettingsProrationBehavior>,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsPauseSchedulesItemResumeSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("billing_cycle_anchor", self.billing_cycle_anchor.as_ref());
        form.field_opt("payment_behavior", self.payment_behavior.as_ref());
        form.field_opt("proration_behavior", self.proration_behavior.as_ref());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodEnd {
    #[doc = "A precise Unix timestamp for the end of the invoice item period."]
    pub timestamp: Option<i64>,
    #[doc = "Select how to calculate the end of the invoice item period."]
    pub r#type: stripe_types_03::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodEndType,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodEnd {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("timestamp", self.timestamp.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "subscription_schedules")]
pub type CreateSubscriptionScheduleParamsPhasesItemAutomaticTaxLiabilityType =
    stripe_types_04::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiabilityType;

#[cfg(feature = "subscription_schedules")]
pub type CreateSubscriptionScheduleParamsPhasesItemDurationInterval = stripe_types_04::subscription_schedules::CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtDurationInterval;

#[cfg(feature = "subscription_schedules")]
pub type CreateSubscriptionScheduleParamsPhasesItemItemsItemPriceDataTaxBehavior = stripe_types_01::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceDataTaxBehavior;

#[cfg(feature = "subscription_schedules")]
pub type CreateSubscriptionScheduleParamsPhasesItemTransferData =
    stripe_types_01::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsTransferData;

#[cfg(feature = "subscription_schedules")]
pub type ListSubscriptionSchedulesParamsCompletedAt =
    stripe_types_00::subscription_schedules::ListSubscriptionSchedulesParamsCanceledAt;

#[cfg(feature = "subscription_schedules")]
pub type ListSubscriptionSchedulesParamsCreated =
    stripe_types_00::subscription_schedules::ListSubscriptionSchedulesParamsCanceledAt;

#[cfg(feature = "subscription_schedules")]
pub type ListSubscriptionSchedulesParamsReleasedAt =
    stripe_types_00::subscription_schedules::ListSubscriptionSchedulesParamsCanceledAt;

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
pub struct SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEnd {
    #[doc = "A precise Unix timestamp for the end of the invoice item period."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[doc = "Select how to calculate the end of the invoice item period."]
    #[json(rename = "type", default)]
    pub r#type:
        stripe_types_02::subscription_schedules::SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEndType,
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
pub struct SubscriptionSchedulesResourcePauseScheduleResumeSettings {
    #[doc = "The billing cycle anchor that applies when the subscription is resumed."]
    #[json(default)]
    pub billing_cycle_anchor: stripe_types_02::subscription_schedules::SubscriptionSchedulesResourcePauseScheduleResumeSettingsBillingCycleAnchor,
    #[doc = "Controls whether Stripe attempts payment on the resumption invoice and how that affects the subscription's status."]
    #[json(default)]
    pub payment_behavior: stripe_types_04::subscription_schedules::SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior,
    #[doc = "Determines how to handle prorations resulting from the billing_cycle_anchor change on resume."]
    #[json(default)]
    pub proration_behavior: stripe_types_01::subscription_schedules::SubscriptionSchedulePhaseConfigurationProrationBehavior,
}

#[cfg(feature = "subscription_schedules")]
pub type UpdateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiabilityType =
    stripe_types_04::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiabilityType;

#[cfg(feature = "subscription_schedules")]
pub type UpdateSubscriptionScheduleParamsDefaultSettingsBillingCycleAnchor =
    stripe_types_04::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsBillingCycleAnchor;

#[cfg(feature = "subscription_schedules")]
pub type UpdateSubscriptionScheduleParamsDefaultSettingsCollectionMethod =
    stripe_types_01::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsCollectionMethod;

#[cfg(feature = "subscription_schedules")]
pub type UpdateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemDiscountsItem =
    stripe_types_01::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemDiscountsItem;

#[cfg(feature = "subscription_schedules")]
pub type UpdateSubscriptionScheduleParamsPhasesItemAutomaticTaxLiabilityType =
    stripe_types_04::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiabilityType;

#[cfg(feature = "subscription_schedules")]
pub type UpdateSubscriptionScheduleParamsPhasesItemBillingCycleAnchor =
    stripe_types_04::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsBillingCycleAnchor;

#[cfg(feature = "subscription_schedules")]
pub type UpdateSubscriptionScheduleParamsPhasesItemBillingThresholds =
    stripe_types_01::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsBillingThresholds;

#[cfg(feature = "subscription_schedules")]
pub type UpdateSubscriptionScheduleParamsPhasesItemCollectionMethod =
    stripe_types_01::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsCollectionMethod;

#[cfg(feature = "subscription_schedules")]
pub type UpdateSubscriptionScheduleParamsPhasesItemItemsItemPriceDataRecurringInterval = stripe_types_04::subscription_schedules::CreateSubscriptionScheduleParamsPauseSchedulesItemResumeResumeAtDurationInterval;

#[cfg(feature = "subscription_schedules")]
pub type UpdateSubscriptionScheduleParamsPhasesItemItemsItemPriceDataTaxBehavior = stripe_types_01::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceDataTaxBehavior;

#[cfg(feature = "subscription_schedules")]
pub type UpdateSubscriptionScheduleParamsPhasesItemProrationBehavior = stripe_types_00::subscription_schedules::CreateSubscriptionScheduleParamsPauseSchedulesItemResumeSettingsProrationBehavior;
