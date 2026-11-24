#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsAddInvoiceItemsItemPriceData {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) w..."]
    pub product: String,
    #[doc = "Only required if a default tax behavior..."]
    pub tax_behavior:
        Option<stripe_types_01::subscriptions::CreateSubscriptionParamsAddInvoiceItemsItemPriceDataTaxBehavior>,
    #[doc = "A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge or a negati..."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsAddInvoiceItemsItemPriceData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("currency", &self.currency);
        form.field("product", &self.product);
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("unit_amount", self.unit_amount.as_ref());
        form.field_opt("unit_amount_decimal", self.unit_amount_decimal.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionParamsAutomaticTaxLiability {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_02::subscriptions::CreateSubscriptionParamsAutomaticTaxLiabilityType,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsAutomaticTaxLiability {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionParamsBillingSchedulesItemBillUntilDuration {
    #[doc = "Specifies billing duration."]
    pub interval: stripe_types_02::subscriptions::CreateSubscriptionParamsBillingSchedulesItemBillUntilDurationInterval,
    #[doc = "The multiplier applied to the interval."]
    pub interval_count: Option<i64>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsBillingSchedulesItemBillUntilDuration {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("interval", &self.interval);
        form.field_opt("interval_count", self.interval_count.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
pub type CreateSubscriptionParamsInvoiceSettingsIssuerType =
    stripe_types_02::subscriptions::CreateSubscriptionParamsAutomaticTaxLiabilityType;

#[cfg(feature = "subscriptions")]
pub type CreateSubscriptionParamsItemsItemPriceDataTaxBehavior =
    stripe_types_01::subscriptions::CreateSubscriptionParamsAddInvoiceItemsItemPriceDataTaxBehavior;

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    #[doc = "Transaction type of the mandate."]
    pub transaction_type: Option<stripe_types_02::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("transaction_type", self.transaction_type.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBancontact {
    #[doc = "Preferred language of the Bancontact authorization page that the customer is redirected to."]
    pub preferred_language: Option<stripe_types_01::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBancontact {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preferred_language", self.preferred_language.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBlik {
    #[doc = "Configuration options for setting up a mandate"]
    pub mandate_options: Option<
        stripe_types_00::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBlikMandateOptions,
    >,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBlik {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardMandateOptions {
    #[doc = "Amount to be charged for future payments, specified in the presentment currency."]
    pub amount: Option<i64>,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type: Option<stripe_types_01::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType>,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("description", self.description.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer {
    #[doc = "Configuration for eu_bank_transfer funding type."]
    pub eu_bank_transfer: Option<stripe_types_02::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>,
    #[doc = "The bank transfer type that can be used for funding."]
    pub r#type: Option<String>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("eu_bank_transfer", self.eu_bank_transfer.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "Determines if the amount includes the IOF tax."]
    pub amount_includes_iof: Option<stripe_types_01::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof>,
    #[doc = "Date when the mandate expires and no further payments will be charged, in `YYYY-MM-DD`."]
    pub end_date: Option<String>,
    #[doc = "Schedule at which the future payments will be charged."]
    pub payment_schedule: Option<stripe_types_02::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsPaymentSchedule>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_includes_iof", self.amount_includes_iof.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
        form.field_opt("payment_schedule", self.payment_schedule.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
pub type CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptionsAmountType = stripe_types_01::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType;

#[cfg(feature = "subscriptions")]
pub type CreateSubscriptionParamsPendingInvoiceItemIntervalInterval =
    stripe_types_02::subscriptions::CreateSubscriptionParamsBillingSchedulesItemBillUntilDurationInterval;

#[cfg(feature = "subscriptions")]
pub type DeleteSubscriptionParamsCancellationDetailsFeedback =
    stripe_types_03::subscriptions::UpdateSubscriptionParamsCancellationDetailsFeedback;

#[cfg(feature = "subscriptions")]
pub type ListSubscriptionsParamsCurrentPeriodEnd = stripe_types_01::subscriptions::ListSubscriptionsParamsCreated;

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone)]
pub struct PauseSubscriptionParamsBillForOutstandingUsageThrough {
    #[doc = "When to bill metered usage in the current period."]
    pub r#type: stripe_types_03::subscriptions::PauseSubscriptionParamsBillForOutstandingUsageThroughType,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for PauseSubscriptionParamsBillForOutstandingUsageThrough {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone)]
pub struct PauseSubscriptionParamsBillForUnusedTimeFrom {
    #[doc = "When to credit for unused time."]
    pub r#type: stripe_types_04::subscriptions::PauseSubscriptionParamsBillForUnusedTimeFromType,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for PauseSubscriptionParamsBillForUnusedTimeFrom {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
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
pub struct SubscriptionPaymentMethodOptionsBillieCompanyDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub registered_address:
        Option<stripe_types_01::subscriptions::SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress>,
    #[doc = "Company or entity name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub registered_name: Option<String>,
    #[doc = "The official registration number for the given registration type."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    #[doc = "Type of registration the company or entity holds in their registered country."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub registration_type:
        Option<stripe_types_03::subscriptions::SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType>,
    #[doc = "VAT ID number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub vat: Option<String>,
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
pub struct SubscriptionPaymentMethodOptionsMandateOptionsPix {
    #[doc = "Amount to be charged for future payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[doc = "Determines if the amount includes the IOF tax."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_includes_iof:
        Option<stripe_types_03::subscriptions::SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof>,
    #[doc = "Date when the mandate expires and no further payments will be charged, in `YYYY-MM-DD`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[doc = "Schedule at which the future payments will be charged."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_schedule:
        Option<stripe_types_01::subscriptions::SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule>,
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
pub struct SubscriptionsResourceBillingModeFlexible {
    #[doc = "Controls how invoices and invoice items display proration amounts and discount amounts."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub proration_discounts:
        Option<stripe_types_04::subscriptions::SubscriptionsResourceBillingModeFlexibleProrationDiscounts>,
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
pub struct SubscriptionsResourcePausedStatusDetailsSubscription {
    #[doc = "The reason that the subscription was paused."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::subscriptions::SubscriptionsResourcePausedStatusDetailsSubscriptionType,
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
pub struct SubscriptionsResourceTrialSettingsEndBehavior {
    #[doc = "Indicates how the subscription's billing cycle anchor is reset when a trial ends."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor:
        Option<stripe_types_01::subscriptions::SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor>,
    #[doc = "Indicates how the subscription should change when the trial ends if the user did not provide a payment method."]
    #[json(default)]
    pub missing_payment_method:
        stripe_types_03::subscriptions::SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod,
}

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsAddInvoiceItemsItemDiscountsItem =
    stripe_types_02::subscriptions::CreateSubscriptionParamsAddInvoiceItemsItemDiscountsItem;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsBillingCycleAnchorType =
    stripe_types_03::subscriptions::CreateSubscriptionParamsTrialSettingsEndBehaviorBillingCycleAnchor;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsBillingThresholds =
    stripe_types_02::subscriptions::CreateSubscriptionParamsBillingThresholds;

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSubscriptionParamsCancellationDetails {
    #[doc = "Additional comments about why the user canceled the subscription, if the subscription was canceled explicitly by the ..."]
    pub comment: Option<json::OwnedValue>,
    #[doc = "The customer submitted reason for why they canceled, if the subscription was canceled explicitly by the user."]
    pub feedback: Option<stripe_types_03::subscriptions::UpdateSubscriptionParamsCancellationDetailsFeedback>,
    #[doc = "Customized feedback options that provide deeper insight into why the subscription was canceled, if the subscription w..."]
    pub feedback_option: Option<String>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for UpdateSubscriptionParamsCancellationDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("feedback", self.feedback.as_ref());
        form.field_opt("feedback_option", self.feedback_option.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsCollectionMethod =
    stripe_types_03::subscriptions::ListSubscriptionsParamsCollectionMethod;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsInvoiceSettingsIssuerType =
    stripe_types_02::subscriptions::CreateSubscriptionParamsAutomaticTaxLiabilityType;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsItemsItemBillingThresholds =
    stripe_types_01::subscriptions::CreateSubscriptionParamsItemsItemBillingThresholds;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsItemsItemCurrentTrial =
    stripe_types_02::subscriptions::CreateSubscriptionParamsItemsItemCurrentTrial;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsItemsItemPriceDataRecurringInterval =
    stripe_types_02::subscriptions::CreateSubscriptionParamsBillingSchedulesItemBillUntilDurationInterval;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsItemsItemPriceDataTaxBehavior =
    stripe_types_01::subscriptions::CreateSubscriptionParamsAddInvoiceItemsItemPriceDataTaxBehavior;

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone)]
pub struct UpdateSubscriptionParamsPauseCollection {
    #[doc = "The payment collection behavior for this subscription while paused."]
    pub behavior: stripe_types_00::subscriptions::UpdateSubscriptionParamsPauseCollectionBehavior,
    #[doc = "The time after which the subscription will resume collecting payments."]
    pub resumes_at: Option<i64>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for UpdateSubscriptionParamsPauseCollection {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("behavior", &self.behavior);
        form.field_opt("resumes_at", self.resumes_at.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType = stripe_types_02::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod = stripe_types_01::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType = stripe_types_01::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType = stripe_types_01::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer = stripe_types_02::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsKonbini =
    stripe_types_01::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsKonbini;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose = stripe_types_00::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof = stripe_types_01::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsSepaDebit =
    stripe_types_01::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsKonbini;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem = stripe_types_00::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod = stripe_types_01::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsPaymentSettingsSaveDefaultPaymentMethod =
    stripe_types_03::subscriptions::CreateSubscriptionParamsPaymentSettingsSaveDefaultPaymentMethod;

#[cfg(feature = "subscriptions")]
pub type UpdateSubscriptionParamsTransferData = stripe_types_00::subscriptions::CreateSubscriptionParamsTransferData;
