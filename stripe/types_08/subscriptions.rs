#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionParamsBillingMode {
    #[doc = "Configure behavior for flexible billing mode."]
    pub flexible: Option<stripe_types_06::subscriptions::CreateSubscriptionParamsBillingModeFlexible>,
    #[doc = "Controls the calculation and orchestration of prorations and invoices for subscriptions."]
    pub r#type: stripe_types_01::subscriptions::CreateSubscriptionParamsBillingModeType,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsBillingMode {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("flexible", self.flexible.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionParamsBillingSchedulesItemBillUntil {
    #[doc = "Specifies the billing period."]
    pub duration: Option<stripe_types_05::subscriptions::CreateSubscriptionParamsBillingSchedulesItemBillUntilDuration>,
    #[doc = "The end date of the billing schedule."]
    pub timestamp: Option<i64>,
    #[doc = "Describes how the billing schedule will determine the end date."]
    pub r#type: stripe_types_01::subscriptions::CreateSubscriptionParamsBillingSchedulesItemBillUntilType,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsBillingSchedulesItemBillUntil {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("duration", self.duration.as_ref());
        form.field_opt("timestamp", self.timestamp.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionParamsInvoiceSettingsIssuer {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_05::subscriptions::CreateSubscriptionParamsInvoiceSettingsIssuerType,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsInvoiceSettingsIssuer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type: Option<stripe_types_05::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptionsAmountType>,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    pub end_date: Option<i64>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionParamsTrialSettings {
    #[doc = "Defines how the subscription should behave when the user's free trial ends."]
    pub end_behavior: stripe_types_06::subscriptions::CreateSubscriptionParamsTrialSettingsEndBehavior,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsTrialSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("end_behavior", &self.end_behavior);
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct DeleteSubscriptionParamsCancellationDetails {
    #[doc = "Additional comments about why the user canceled the subscription, if the subscription was canceled explicitly by the ..."]
    pub comment: Option<json::OwnedValue>,
    #[doc = "The customer submitted reason for why they canceled, if the subscription was canceled explicitly by the user."]
    pub feedback: Option<stripe_types_05::subscriptions::DeleteSubscriptionParamsCancellationDetailsFeedback>,
    #[doc = "Customized feedback options that provide deeper insight into why the subscription was canceled, if the subscription w..."]
    pub feedback_option: Option<String>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for DeleteSubscriptionParamsCancellationDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("feedback", self.feedback.as_ref());
        form.field_opt("feedback_option", self.feedback_option.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct ListSubscriptionsParams {
    pub automatic_tax: Option<stripe_types_01::subscriptions::ListSubscriptionsParamsAutomaticTax>,
    pub collection_method: Option<stripe_types_03::subscriptions::ListSubscriptionsParamsCollectionMethod>,
    pub created: Option<stripe_types_01::subscriptions::ListSubscriptionsParamsCreated>,
    pub current_period_end: Option<stripe_types_05::subscriptions::ListSubscriptionsParamsCurrentPeriodEnd>,
    pub current_period_start: Option<stripe_types_06::subscriptions::ListSubscriptionsParamsCurrentPeriodStart>,
    pub customer: Option<String>,
    pub customer_account: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub plan: Option<String>,
    pub price: Option<String>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_04::subscriptions::ListSubscriptionsParamsStatus>,
    pub test_clock: Option<String>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for ListSubscriptionsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("automatic_tax", self.automatic_tax.as_ref());
        form.field_opt("collection_method", self.collection_method.as_ref());
        form.field_opt("created", self.created.as_ref());
        form.field_opt("current_period_end", self.current_period_end.as_ref());
        form.field_opt("current_period_start", self.current_period_start.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("plan", self.plan.as_ref());
        form.field_opt("price", self.price.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
        form.field_opt("test_clock", self.test_clock.as_ref());
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
pub struct SubscriptionPaymentMethodOptionsPix {
    #[doc = "The number of seconds (between 10 and 1209600) after which Pix payment will expire."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expires_after_seconds: Option<i64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_types_05::subscriptions::SubscriptionPaymentMethodOptionsMandateOptionsPix>,
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
pub struct SubscriptionsResourceBillingMode {
    #[doc = "Configure behavior for flexible billing mode"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub flexible: Option<stripe_types_05::subscriptions::SubscriptionsResourceBillingModeFlexible>,
    #[doc = "Controls how prorations and invoices for subscriptions are calculated and orchestrated."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::subscriptions::SubscriptionsResourceBillingModeType,
    #[doc = "Details on when the current billing_mode was adopted."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone)]
pub struct UpdateSubscriptionParamsAddInvoiceItemsItemPeriodEnd {
    #[doc = "A precise Unix timestamp for the end of the invoice item period."]
    pub timestamp: Option<i64>,
    #[doc = "Select how to calculate the end of the invoice item period."]
    pub r#type: stripe_types_06::subscriptions::UpdateSubscriptionParamsAddInvoiceItemsItemPeriodEndType,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for UpdateSubscriptionParamsAddInvoiceItemsItemPeriodEnd {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("timestamp", self.timestamp.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters {
    #[doc = "The account subcategories to use to filter for selectable accounts."]
    pub account_subcategories: Option<Vec<stripe_types_05::subscriptions::UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem>>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode
    for UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.account_subcategories.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("account_subcategories", index, item);
            }
        }
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone)]
pub struct UpdateSubscriptionParamsPendingInvoiceItemInterval {
    #[doc = "Specifies invoicing frequency."]
    pub interval: stripe_types_06::subscriptions::UpdateSubscriptionParamsPendingInvoiceItemIntervalInterval,
    #[doc = "The number of intervals between invoices."]
    pub interval_count: Option<i64>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for UpdateSubscriptionParamsPendingInvoiceItemInterval {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("interval", &self.interval);
        form.field_opt("interval_count", self.interval_count.as_ref());
    }
}
