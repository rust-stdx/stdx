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
pub struct SubscriptionItem {
    #[doc = "The time period the subscription item has been billed for."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billed_until: Option<i64>,
    #[doc = "Define thresholds at which an invoice will be sent, and the related subscription advanced to a new billing period"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<stripe_types_01::subscription_items::SubscriptionItemBillingThresholds>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "The end time of this subscription item's current billing period."]
    #[json(default)]
    pub current_period_end: i64,
    #[doc = "The start time of this subscription item's current billing period."]
    #[json(default)]
    pub current_period_start: i64,
    #[doc = "The current trial that is applied to this subscription item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub current_trial: Option<stripe_types_01::subscriptions::SubscriptionsTrialsResourceCurrentTrial>,
    #[doc = "The discounts applied to the subscription item."]
    #[json(default)]
    pub discounts: Vec<crate::Expandable<crate::coupons::Discount>>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::subscription_items::SubscriptionItemObject,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<stripe_types_13::plans::Plan>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<stripe_types_11::prices::Price>,
    #[doc = "The [quantity](https://docs.stripe.com/subscriptions/quantities) of the plan to which the customer should be subscribed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[doc = "The `subscription` this `subscription_item` belongs to."]
    #[json(default)]
    pub subscription: String,
    #[doc = "The tax rates which apply to this `subscription_item`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<stripe_types_06::tax_rates::TaxRate>>,
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
impl crate::PagedItem for SubscriptionItem {
    fn id(&self) -> &str {
        &self.id
    }
}
