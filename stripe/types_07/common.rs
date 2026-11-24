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
pub struct LineItemsTaxAmount {
    #[doc = "Amount of tax applied for this rate."]
    #[json(default)]
    pub amount: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<stripe_types_06::tax_rates::TaxRate>,
    #[doc = "The reasoning behind this tax, for example, if the product is tax exempt."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub taxability_reason: Option<stripe_types_01::common::LineItemsTaxAmountTaxabilityReason>,
    #[doc = "The amount on which tax is calculated, in cents (or local equivalent)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub taxable_amount: Option<i64>,
}
