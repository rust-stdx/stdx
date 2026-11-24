#[cfg(feature = "billing")]
pub type ActivateBillingAlertParams = stripe_types_00::billing::GetBillingAlertParams;

#[cfg(feature = "billing")]
pub type ArchiveBillingAlertParams = stripe_types_00::billing::GetBillingAlertParams;

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
pub struct BillingBillResourceInvoicingLinesCommonProrationDetails {
    #[doc = "For a credit proration `line_item`, the original debit line_items to which the credit proration applies."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub credited_items: Option<stripe_types_03::billing::BillingBillResourceInvoicingLinesCommonCreditedItems>,
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
pub struct BillingCreditGrantsResourceScope {
    #[doc = "The price type that credit grants can apply to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub price_type: Option<stripe_types_01::billing::BillingCreditGrantsResourceScopePriceType>,
    #[doc = "The prices that credit grants can apply to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub prices: Option<Vec<stripe_types_00::billing::BillingCreditGrantsResourceApplicablePrice>>,
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "billing_portal",
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
pub struct BillingFeedbackOption {
    #[doc = "An arbitrary string attached to the object."]
    #[json(default)]
    pub description: String,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::billing::BillingFeedbackOptionObject,
    #[doc = "The feedback option's status."]
    #[json(default)]
    pub status: stripe_types_03::billing::BillingFeedbackOptionStatus,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status_transitions: Option<stripe_types_02::common::FeedbackOptionsStatusTransitions>,
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "billing_portal",
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
impl crate::PagedItem for BillingFeedbackOption {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingMeterEvent {
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "The name of the meter event."]
    #[json(default)]
    pub event_name: String,
    #[doc = "A unique identifier for the event."]
    #[json(default)]
    pub identifier: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::billing::BillingMeterEventObject,
    #[doc = "The payload of the event."]
    #[json(default)]
    pub payload: std::collections::HashMap<String, String>,
    #[doc = "The timestamp passed in when creating the event."]
    #[json(default)]
    pub timestamp: i64,
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingMeterEventAdjustment {
    #[doc = "Specifies which event to cancel."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cancel: Option<stripe_types_02::billing::BillingMeterResourceBillingMeterEventAdjustmentCancel>,
    #[doc = "The name of the meter event."]
    #[json(default)]
    pub event_name: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::billing::BillingMeterEventAdjustmentObject,
    #[doc = "The meter event adjustment's status."]
    #[json(default)]
    pub status: stripe_types_03::billing::BillingMeterEventAdjustmentStatus,
    #[doc = "Specifies whether to cancel a single event or a range of events for a time period."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::billing::BillingMeterEventAdjustmentType,
}

#[cfg(any(feature = "billing", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingMeterResourceCustomerMappingSettings {
    #[doc = "The key in the meter event payload to use for mapping the event to a customer."]
    #[json(default)]
    pub event_payload_key: String,
    #[doc = "The method for mapping a meter event to a customer."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::billing::BillingMeterResourceCustomerMappingSettingsType,
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone)]
pub struct CreateBillingAlertParamsUsageThresholdFiltersItem {
    #[doc = "Limit the scope to this usage alert only to this customer."]
    pub customer: Option<String>,
    #[doc = "What type of filter is being applied to this usage alert."]
    pub r#type: stripe_types_02::billing::CreateBillingAlertParamsUsageThresholdFiltersItemType,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for CreateBillingAlertParamsUsageThresholdFiltersItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("customer", self.customer.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone)]
pub struct CreateBillingCreditGrantParamsAmount {
    #[doc = "The monetary amount."]
    pub monetary: Option<stripe_types_04::billing::CreateBillingCreditGrantParamsAmountMonetary>,
    #[doc = "The type of this amount."]
    pub r#type: stripe_types_02::billing::CreateBillingCreditGrantParamsAmountType,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for CreateBillingCreditGrantParamsAmount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("monetary", self.monetary.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "billing")]
pub type CreateBillingCreditGrantParamsApplicabilityConfigScopePricesItem =
    stripe_types_01::billing::GetBillingCreditBalanceSummaryParamsFilterApplicabilityScopePricesItem;

#[cfg(feature = "billing")]
#[derive(Debug, Clone)]
pub struct CreateBillingMeterParamsDefaultAggregation {
    #[doc = "Specifies how events are aggregated."]
    pub formula: stripe_types_02::billing::CreateBillingMeterParamsDefaultAggregationFormula,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for CreateBillingMeterParamsDefaultAggregation {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("formula", &self.formula);
    }
}

#[cfg(feature = "billing")]
pub type DeactivateBillingMeterParams = stripe_types_00::billing::GetBillingAlertParams;

#[cfg(feature = "billing")]
pub type ExpireBillingCreditGrantParams = stripe_types_00::billing::GetBillingAlertParams;

#[cfg(feature = "billing")]
#[derive(Debug, Clone, Default)]
pub struct GetBillingCreditBalanceSummaryParamsFilterApplicabilityScope {
    #[doc = "The price type that credit grants can apply to."]
    pub price_type:
        Option<stripe_types_00::billing::GetBillingCreditBalanceSummaryParamsFilterApplicabilityScopePriceType>,
    #[doc = "A list of prices that the credit grant can apply to."]
    pub prices:
        Option<Vec<stripe_types_01::billing::GetBillingCreditBalanceSummaryParamsFilterApplicabilityScopePricesItem>>,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for GetBillingCreditBalanceSummaryParamsFilterApplicabilityScope {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("price_type", self.price_type.as_ref());
        if let Some(values) = self.prices.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("prices", index, item);
            }
        }
    }
}

#[cfg(feature = "billing")]
pub type GetBillingCreditBalanceTransactionParams = stripe_types_00::billing::GetBillingAlertParams;

#[cfg(feature = "billing")]
pub type GetBillingMeterParams = stripe_types_00::billing::GetBillingAlertParams;

#[cfg(feature = "billing")]
#[derive(Debug, Clone, Default)]
pub struct ListBillingAlertsParams {
    pub alert_type: Option<stripe_types_02::billing::ListBillingAlertsParamsAlertType>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub meter: Option<String>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for ListBillingAlertsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("alert_type", self.alert_type.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("meter", self.meter.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(feature = "billing")]
pub type VoidBillingCreditGrantParams = stripe_types_00::billing::GetBillingAlertParams;
