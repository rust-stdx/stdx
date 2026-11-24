#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionParamsAddInvoiceItemsItemPeriodEnd {
    #[doc = "A precise Unix timestamp for the end of the invoice item period."]
    pub timestamp: Option<i64>,
    #[doc = "Select how to calculate the end of the invoice item period."]
    pub r#type: stripe_types_02::subscriptions::CreateSubscriptionParamsAddInvoiceItemsItemPeriodEndType,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsAddInvoiceItemsItemPeriodEnd {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("timestamp", self.timestamp.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionParamsAddInvoiceItemsItemPeriodStart {
    #[doc = "A precise Unix timestamp for the start of the invoice item period."]
    pub timestamp: Option<i64>,
    #[doc = "Select how to calculate the start of the invoice item period."]
    pub r#type: stripe_types_04::subscriptions::CreateSubscriptionParamsAddInvoiceItemsItemPeriodStartType,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsAddInvoiceItemsItemPeriodStart {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("timestamp", self.timestamp.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsBillingModeFlexible {
    #[doc = "Controls how invoices and invoice items display proration amounts and discount amounts."]
    pub proration_discounts:
        Option<stripe_types_02::subscriptions::CreateSubscriptionParamsBillingModeFlexibleProrationDiscounts>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsBillingModeFlexible {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("proration_discounts", self.proration_discounts.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionParamsBillingSchedulesItemAppliesToItem {
    #[doc = "The ID of the price object."]
    pub price: Option<String>,
    #[doc = "Controls which subscription items the billing schedule applies to."]
    pub r#type: stripe_types_00::subscriptions::CreateSubscriptionParamsBillingSchedulesItemAppliesToItemType,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsBillingSchedulesItemAppliesToItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("price", self.price.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "subscriptions")]
pub type CreateSubscriptionParamsCollectionMethod =
    stripe_types_03::subscriptions::ListSubscriptionsParamsCollectionMethod;

#[cfg(feature = "subscriptions")]
pub type CreateSubscriptionParamsItemsItemPriceDataRecurringInterval =
    stripe_types_02::subscriptions::CreateSubscriptionParamsBillingSchedulesItemBillUntilDurationInterval;

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetails {
    #[doc = "The address the company or entity is registered with."]
    pub registered_address: Option<stripe_types_04::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress>,
    #[doc = "Company or entity name."]
    pub registered_name: Option<String>,
    #[doc = "The official registration number for the given registration type."]
    pub registration_number: Option<String>,
    #[doc = "Type of registration the company or entity holds in their registered country."]
    pub registration_type: Option<stripe_types_01::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType>,
    #[doc = "VAT ID number."]
    pub vat: Option<String>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("registered_address", self.registered_address.as_ref());
        form.field_opt("registered_name", self.registered_name.as_ref());
        form.field_opt("registration_number", self.registration_number.as_ref());
        form.field_opt("registration_type", self.registration_type.as_ref());
        form.field_opt("vat", self.vat.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptions {
    #[doc = "The maximum amount that can be collected in a single invoice."]
    pub amount: Option<i64>,
    #[doc = "The purpose for which payments are made."]
    pub purpose: Option<stripe_types_00::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("purpose", self.purpose.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
pub type CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsSepaDebit =
    stripe_types_01::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsKonbini;

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters {
    #[doc = "The account subcategories to use to filter for selectable accounts."]
    pub account_subcategories: Option<Vec<stripe_types_00::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem>>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.account_subcategories.as_ref() {
            form.field_array("account_subcategories", values.as_slice());
        }
    }
}

#[cfg(feature = "subscriptions")]
pub type CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod = stripe_types_01::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod;

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsTrialSettingsEndBehavior {
    #[doc = "Indicates how the subscription's billing cycle anchor is reset when a trial ends."]
    pub billing_cycle_anchor:
        Option<stripe_types_03::subscriptions::CreateSubscriptionParamsTrialSettingsEndBehaviorBillingCycleAnchor>,
    #[doc = "Indicates how the subscription should change when the trial ends if the user did not provide a payment method."]
    pub missing_payment_method:
        Option<stripe_types_01::subscriptions::CreateSubscriptionParamsTrialSettingsEndBehaviorMissingPaymentMethod>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsTrialSettingsEndBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("billing_cycle_anchor", self.billing_cycle_anchor.as_ref());
        form.field_opt("missing_payment_method", self.missing_payment_method.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
pub type ListSubscriptionsParamsCurrentPeriodStart = stripe_types_01::subscriptions::ListSubscriptionsParamsCreated;

#[cfg(feature = "subscriptions")]
pub type MigrateSubscriptionParamsBillingModeFlexibleProrationDiscounts =
    stripe_types_02::subscriptions::CreateSubscriptionParamsBillingModeFlexibleProrationDiscounts;

#[cfg(feature = "subscriptions")]
pub type ResumeSubscriptionParamsBillingCycleAnchorType =
    stripe_types_03::subscriptions::CreateSubscriptionParamsTrialSettingsEndBehaviorBillingCycleAnchor;

#[cfg(feature = "subscriptions")]
pub type ResumeSubscriptionParamsProrationBehavior =
    stripe_types_00::subscriptions::CreateSubscriptionParamsProrationBehavior;

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
pub struct SubscriptionPaymentMethodOptionsBlik {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_types_04::subscriptions::SubscriptionPaymentMethodOptionsMandateOptionsBlik>,
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
pub struct SubscriptionPendingInvoiceItemInterval {
    #[doc = "Specifies invoicing frequency."]
    #[json(default)]
    pub interval: stripe_types_01::subscriptions::SubscriptionPendingInvoiceItemIntervalInterval,
    #[doc = "The number of intervals between invoices."]
    #[json(default)]
    pub interval_count: i64,
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
pub struct SubscriptionsResourceBillingSchedulesBillUntilDuration {
    #[doc = "Specifies billing duration."]
    #[json(default)]
    pub interval: stripe_types_01::subscriptions::SubscriptionPendingInvoiceItemIntervalInterval,
    #[doc = "The multiplier applied to the interval."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<i64>,
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
pub struct SubscriptionsResourcePauseCollection {
    #[doc = "The payment collection behavior for this subscription while paused."]
    #[json(default)]
    pub behavior: stripe_types_04::subscriptions::SubscriptionsResourcePauseCollectionBehavior,
    #[doc = "The time after which the subscription will resume collecting payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub resumes_at: Option<i64>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SubscriptionsTrialsResourceEndBehavior {
    #[doc = "Indicates how the subscription should change when the trial ends if the user did not provide a payment method."]
    #[json(default)]
    pub missing_payment_method:
        stripe_types_03::subscriptions::SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod,
}

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsAddInvoiceItemsItemPeriodEndType =
    stripe_types_02::subscriptions::CreateSubscriptionParamsAddInvoiceItemsItemPeriodEndType;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsAddInvoiceItemsItemPeriodStartType =
    stripe_types_04::subscriptions::CreateSubscriptionParamsAddInvoiceItemsItemPeriodStartType;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsAddInvoiceItemsItemPriceDataTaxBehavior =
    stripe_types_01::subscriptions::CreateSubscriptionParamsAddInvoiceItemsItemPriceDataTaxBehavior;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsAutomaticTaxLiabilityType =
    stripe_types_02::subscriptions::CreateSubscriptionParamsAutomaticTaxLiabilityType;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsPaymentBehavior =
    stripe_types_01::subscriptions::CreateSubscriptionParamsPaymentBehavior;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage = stripe_types_01::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress = stripe_types_04::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBlikMandateOptions =
    stripe_types_00::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBlikMandateOptions;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork =
    stripe_types_02::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure =
    stripe_types_00::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsPaymentSchedule = stripe_types_02::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsPaymentSchedule;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptionsAmountType = stripe_types_01::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem = stripe_types_00::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem = stripe_types_04::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsPendingInvoiceItemIntervalInterval =
    stripe_types_02::subscriptions::CreateSubscriptionParamsBillingSchedulesItemBillUntilDurationInterval;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsProrationBehavior =
    stripe_types_00::subscriptions::CreateSubscriptionParamsProrationBehavior;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsTrialSettingsEndBehaviorBillingCycleAnchor =
    stripe_types_03::subscriptions::CreateSubscriptionParamsTrialSettingsEndBehaviorBillingCycleAnchor;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsTrialSettingsEndBehaviorMissingPaymentMethod =
    stripe_types_01::subscriptions::CreateSubscriptionParamsTrialSettingsEndBehaviorMissingPaymentMethod;
