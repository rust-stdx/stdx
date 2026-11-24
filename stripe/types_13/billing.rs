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
pub struct BillingBillResourceInvoicingPricingPricing {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub price_details: Option<stripe_types_12::billing::BillingBillResourceInvoicingPricingPricingPriceDetails>,
    #[doc = "The type of the pricing details."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::billing::BillingBillResourceInvoicingPricingPricingType,
    #[doc = "The unit amount (in the `currency` specified) of the item which contains a decimal value with at most 12 decimal places."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
