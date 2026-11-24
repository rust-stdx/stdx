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
    feature = "prices",
    feature = "product_catalog",
    feature = "products",
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
pub struct CurrencyOption {
    #[doc = "When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment L..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub custom_unit_amount: Option<stripe_types_02::common::CustomUnitAmount>,
    #[doc = "Only required if a default tax behavior..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<stripe_types_00::common::CurrencyOptionTaxBehavior>,
    #[doc = "Each element represents a pricing tier."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<stripe_types_02::prices::PriceTier>>,
    #[doc = "The unit amount in cents (or local equivalent) to be charged, represented as a whole integer if possible."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    #[doc = "The unit amount in cents (or local equivalent) to be charged, represented as a decimal string with at most 12 decimal..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
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
pub struct UsBankAccountNetworks {
    #[doc = "The preferred network."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub preferred: Option<String>,
    #[doc = "All supported networks."]
    #[json(default)]
    pub supported: Vec<stripe_types_03::common::UsBankAccountNetworksSupportedItem>,
}
