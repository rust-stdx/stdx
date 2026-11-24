#[cfg(feature = "prices")]
pub type CreatePriceParamsRecurringInterval = stripe_types_01::prices::ListPricesParamsRecurringInterval;

#[cfg(feature = "prices")]
pub type CreatePriceParamsRecurringUsageType = stripe_types_03::prices::ListPricesParamsRecurringUsageType;

#[cfg(feature = "prices")]
pub type CreatePriceParamsTaxBehavior = stripe_types_02::prices::CreatePriceParamsCurrencyOptionsValueTaxBehavior;

#[cfg(feature = "prices")]
pub type CreatePriceParamsTiersItem = stripe_types_01::prices::CreatePriceParamsCurrencyOptionsValueTiersItem;

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
pub struct TransformQuantity {
    #[doc = "Divide usage by this number."]
    #[json(default)]
    pub divide_by: i64,
    #[doc = "After division, either round the result `up` or `down`."]
    #[json(default)]
    pub round: stripe_types_04::prices::TransformQuantityRound,
}

#[cfg(feature = "prices")]
pub type UpdatePriceParamsCurrencyOptionsValueCustomUnitAmount =
    stripe_types_04::prices::CreatePriceParamsCurrencyOptionsValueCustomUnitAmount;

#[cfg(feature = "prices")]
pub type UpdatePriceParamsCurrencyOptionsValueTiersItem =
    stripe_types_01::prices::CreatePriceParamsCurrencyOptionsValueTiersItem;
