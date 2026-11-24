#[cfg(any(feature = "events", feature = "invoiceitems"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingBillResourceInvoiceItemParentsInvoiceItemParent {
    #[doc = "Details about the subscription that generated this invoice item"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subscription_details:
        Option<stripe_types_04::billing::BillingBillResourceInvoiceItemParentsInvoiceItemSubscriptionParent>,
    #[doc = "The type of parent that generated this invoice item"]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::billing::BillingBillResourceInvoiceItemParentsInvoiceItemParentType,
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
pub struct BillingClocksResourceStatusDetailsStatusDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub advancing: Option<stripe_types_02::billing::BillingClocksResourceStatusDetailsAdvancingStatusDetails>,
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
pub struct BillingCreditGrantsResourceAmount {
    #[doc = "The monetary amount."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub monetary: Option<stripe_types_01::billing::BillingCreditGrantsResourceMonetaryAmount>,
    #[doc = "The type of this amount."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::billing::BillingCreditGrantsResourceAmountType,
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
pub struct BillingDetails {
    #[doc = "Billing address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_01::common::Address>,
    #[doc = "Email address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Full name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Billing phone number (including extension)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "Taxpayer identification number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingMeterEventSummary {
    #[doc = "Aggregated value of all the events within `start_time` (inclusive) and `end_time` (inclusive)."]
    #[json(default)]
    pub aggregated_value: f64,
    #[doc = "End timestamp for this event summary (exclusive)."]
    #[json(default)]
    pub end_time: i64,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "The meter associated with this event summary."]
    #[json(default)]
    pub meter: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::billing::BillingMeterEventSummaryObject,
    #[doc = "Start timestamp for this event summary (inclusive)."]
    #[json(default)]
    pub start_time: i64,
}

#[cfg(feature = "billing")]
impl crate::PagedItem for BillingMeterEventSummary {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingMeterResourceAggregationSettings {
    #[doc = "Specifies how events are aggregated."]
    #[json(default)]
    pub formula: stripe_types_02::billing::BillingMeterResourceAggregationSettingsFormula,
}

#[cfg(feature = "billing")]
pub type CreateBillingAlertParamsAlertType = stripe_types_02::billing::ListBillingAlertsParamsAlertType;

#[cfg(feature = "billing")]
pub type CreateBillingCreditGrantParamsApplicabilityConfigScopePriceType =
    stripe_types_00::billing::GetBillingCreditBalanceSummaryParamsFilterApplicabilityScopePriceType;

#[cfg(feature = "billing")]
#[derive(Debug, Clone)]
pub struct CreateBillingMeterEventAdjustmentParams {
    #[doc = "Specifies which event to cancel."]
    pub cancel: Option<stripe_types_01::billing::CreateBillingMeterEventAdjustmentParamsCancel>,
    #[doc = "The name of the meter event."]
    pub event_name: String,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Specifies whether to cancel a single event or a range of events for a time period."]
    pub r#type: stripe_types_01::billing::CreateBillingMeterEventAdjustmentParamsType,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for CreateBillingMeterEventAdjustmentParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("cancel", self.cancel.as_ref());
        form.field("event_name", &self.event_name);
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone)]
pub struct CreateBillingMeterParamsCustomerMapping {
    #[doc = "The key in the meter event payload to use for mapping the event to a customer."]
    pub event_payload_key: String,
    #[doc = "The method for mapping a meter event to a customer."]
    pub r#type: stripe_types_02::billing::CreateBillingMeterParamsCustomerMappingType,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for CreateBillingMeterParamsCustomerMapping {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("event_payload_key", &self.event_payload_key);
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "billing")]
pub type DeactivateBillingAlertParams = stripe_types_00::billing::GetBillingAlertParams;

#[cfg(feature = "billing")]
pub type DeactivateBillingFeedbackOptionParams = stripe_types_00::billing::GetBillingAlertParams;

#[cfg(feature = "billing")]
pub type GetBillingCreditGrantParams = stripe_types_00::billing::GetBillingAlertParams;

#[cfg(feature = "billing")]
pub type GetBillingFeedbackOptionParams = stripe_types_00::billing::GetBillingAlertParams;

#[cfg(feature = "billing")]
#[derive(Debug, Clone, Default)]
pub struct ListBillingFeedbackOptionsParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_04::billing::ListBillingFeedbackOptionsParamsStatus>,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for ListBillingFeedbackOptionsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
    }
}

#[cfg(feature = "billing")]
pub type ListBillingMeterEventSummariesParamsValueGroupingWindow =
    stripe_types_03::billing::CreateBillingMeterParamsEventTimeWindow;

#[cfg(feature = "billing")]
pub type ListBillingMetersParamsStatus = stripe_types_04::billing::ListBillingFeedbackOptionsParamsStatus;

#[cfg(feature = "billing")]
pub type ReactivateBillingMeterParams = stripe_types_00::billing::GetBillingAlertParams;
