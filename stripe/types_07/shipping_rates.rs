#[cfg(feature = "shipping_rates")]
#[derive(Debug, Clone)]
pub struct CreateShippingRateParamsDeliveryEstimateMinimum {
    #[doc = "A unit of time."]
    pub unit: stripe_types_06::shipping_rates::CreateShippingRateParamsDeliveryEstimateMinimumUnit,
    #[doc = "Must be greater than 0."]
    pub value: i64,
}

#[cfg(feature = "shipping_rates")]
impl crate::params::FormEncode for CreateShippingRateParamsDeliveryEstimateMinimum {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("unit", &self.unit);
        form.field("value", &self.value);
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
    feature = "shipping_rates",
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
pub struct ShippingRateDeliveryEstimate {
    #[doc = "The upper bound of the estimated range."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<stripe_types_06::shipping_rates::ShippingRateDeliveryEstimateBound>,
    #[doc = "The lower bound of the estimated range."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<stripe_types_06::shipping_rates::ShippingRateDeliveryEstimateBound>,
}
