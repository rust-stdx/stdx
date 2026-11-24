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
pub struct BillingBillResourceInvoicingLinesParentsInvoiceLineItemInvoiceItemParent {
    #[doc = "The invoice item that generated this line item"]
    #[json(default)]
    pub invoice_item: String,
    #[doc = "Whether this is a proration"]
    #[json(default)]
    pub proration: bool,
    #[doc = "Additional details for proration line items"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub proration_details: Option<stripe_types_06::billing::BillingBillResourceInvoicingLinesCommonProrationDetails>,
    #[doc = "The subscription that the invoice item belongs to"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
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
pub struct BillingBillResourceInvoicingLinesParentsInvoiceLineItemSubscriptionItemParent {
    #[doc = "The invoice item that generated this line item"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice_item: Option<String>,
    #[doc = "Whether this is a proration"]
    #[json(default)]
    pub proration: bool,
    #[doc = "Additional details for proration line items"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub proration_details: Option<stripe_types_06::billing::BillingBillResourceInvoicingLinesCommonProrationDetails>,
    #[doc = "The subscription that the subscription item belongs to"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
    #[doc = "The subscription item that generated this line item"]
    #[json(default)]
    pub subscription_item: String,
}

#[cfg(any(feature = "billing", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingMeter {
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_mapping: Option<stripe_types_06::billing::BillingMeterResourceCustomerMappingSettings>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_aggregation: Option<stripe_types_05::billing::BillingMeterResourceAggregationSettings>,
    #[doc = "The meter's name."]
    #[json(default)]
    pub display_name: String,
    #[doc = "The name of the meter event to record usage for."]
    #[json(default)]
    pub event_name: String,
    #[doc = "The time window which meter events have been pre-aggregated for, if any."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub event_time_window: Option<stripe_types_02::billing::BillingMeterEventTimeWindow>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::billing::BillingMeterObject,
    #[doc = "The meter's status."]
    #[json(default)]
    pub status: stripe_types_03::billing::BillingFeedbackOptionStatus,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status_transitions: Option<stripe_types_00::billing::BillingMeterResourceBillingMeterStatusTransitions>,
    #[doc = "Time at which the object was last updated."]
    #[json(default)]
    pub updated: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub value_settings: Option<stripe_types_03::billing::BillingMeterResourceBillingMeterValue>,
}

#[cfg(any(feature = "billing", feature = "events"))]
impl crate::PagedItem for BillingMeter {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone)]
pub struct CreateBillingAlertParamsUsageThreshold {
    #[doc = "The filters allows limiting the scope of this usage alert."]
    pub filters: Option<Vec<stripe_types_06::billing::CreateBillingAlertParamsUsageThresholdFiltersItem>>,
    #[doc = "Defines the threshold value that triggers the alert."]
    pub gte: i64,
    #[doc = "The [Billing Meter](/api/billing/meter) ID whose usage is monitored."]
    pub meter: String,
    #[doc = "Defines how the alert will behave."]
    pub recurrence: stripe_types_01::billing::CreateBillingAlertParamsUsageThresholdRecurrence,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for CreateBillingAlertParamsUsageThreshold {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.filters.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("filters", index, item);
            }
        }
        form.field("gte", &self.gte);
        form.field("meter", &self.meter);
        form.field("recurrence", &self.recurrence);
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, Default)]
pub struct CreateBillingCreditGrantParamsApplicabilityConfigScope {
    #[doc = "The price type that credit grants can apply to."]
    pub price_type: Option<stripe_types_05::billing::CreateBillingCreditGrantParamsApplicabilityConfigScopePriceType>,
    #[doc = "A list of prices that the credit grant can apply to."]
    pub prices: Option<Vec<stripe_types_06::billing::CreateBillingCreditGrantParamsApplicabilityConfigScopePricesItem>>,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for CreateBillingCreditGrantParamsApplicabilityConfigScope {
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
#[derive(Debug, Clone)]
pub struct CreateBillingMeterParams {
    #[doc = "Fields that specify how to map a meter event to a customer."]
    pub customer_mapping: Option<stripe_types_05::billing::CreateBillingMeterParamsCustomerMapping>,
    #[doc = "The default settings to aggregate a meter's events with."]
    pub default_aggregation: stripe_types_06::billing::CreateBillingMeterParamsDefaultAggregation,
    #[doc = "The meter’s name."]
    pub display_name: String,
    #[doc = "The name of the meter event to record usage for."]
    pub event_name: String,
    #[doc = "The time window which meter events have been pre-aggregated for, if any."]
    pub event_time_window: Option<stripe_types_03::billing::CreateBillingMeterParamsEventTimeWindow>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Fields that specify how to calculate a meter event's value."]
    pub value_settings: Option<stripe_types_01::billing::CreateBillingMeterParamsValueSettings>,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for CreateBillingMeterParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("customer_mapping", self.customer_mapping.as_ref());
        form.field("default_aggregation", &self.default_aggregation);
        form.field("display_name", &self.display_name);
        form.field("event_name", &self.event_name);
        form.field_opt("event_time_window", self.event_time_window.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("value_settings", self.value_settings.as_ref());
    }
}
