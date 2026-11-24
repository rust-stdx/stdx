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
    feature = "plans",
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
pub struct Plan {
    #[doc = "Whether the plan can be used for new purchases."]
    #[json(default)]
    pub active: bool,
    #[doc = "The unit amount in cents (or local equivalent) to be charged, represented as a whole integer if possible."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[doc = "The unit amount in cents (or local equivalent) to be charged, represented as a decimal string with at most 12 decimal..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_decimal: Option<String>,
    #[doc = "Describes how to compute the price per period."]
    #[json(default)]
    pub billing_scheme: stripe_types_01::plans::PlanBillingScheme,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "The frequency at which a subscription is billed."]
    #[json(default)]
    pub interval: stripe_types_03::plans::PlanInterval,
    #[doc = "The number of intervals (specified in the `interval` attribute) between subscription billings."]
    #[json(default)]
    pub interval_count: i64,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The meter tracking the usage of a metered price"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub meter: Option<String>,
    #[doc = "A brief description of the plan, hidden from customers."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::plans::PlanObject,
    #[doc = "The product whose pricing this plan determines."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<crate::Expandable<stripe_types_12::plans::PlanProductExpanded>>,
    #[doc = "Each element represents a pricing tier."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<stripe_types_03::plans::PlanTier>>,
    #[doc = "Defines if the tiering price should be `graduated` or `volume` based."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tiers_mode: Option<stripe_types_04::plans::PlanTiersMode>,
    #[doc = "Apply a transformation to the reported usage or set quantity before computing the amount billed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transform_usage: Option<stripe_types_06::prices::TransformUsage>,
    #[doc = "Default number of trial days when subscribing a customer to this plan using `trial_from_plan=true`..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
    #[doc = "Configures how the quantity per period should be determined."]
    #[json(default)]
    pub usage_type: stripe_types_01::plans::PlanUsageType,
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
    feature = "plans",
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
impl crate::PagedItem for Plan {
    fn id(&self) -> &str {
        &self.id
    }
}
