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
pub struct AccountRequirements {
    #[doc = "Fields that are due and can be resolved by providing the corresponding alternative fields instead."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alternatives: Option<Vec<stripe_types_01::account::AccountRequirementsAlternative>>,
    #[doc = "Date by which the fields in `currently_due` must be collected to keep the account enabled."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub current_deadline: Option<i64>,
    #[doc = "Fields that need to be resolved to keep the account enabled."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub currently_due: Option<Vec<String>>,
    #[doc = "If the account is disabled, this enum describes why."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<stripe_types_00::account::AccountFutureRequirementsDisabledReason>,
    #[doc = "Fields that are `currently_due` and need to be collected again because validation or verification failed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<stripe_types_06::account::AccountRequirementsError>>,
    #[doc = "Fields you must collect when all thresholds are reached."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub eventually_due: Option<Vec<String>>,
    #[doc = "Fields that haven't been resolved by `current_deadline`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub past_due: Option<Vec<String>>,
    #[doc = "Fields that are being reviewed, or might become required depending on the results of a review."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pending_verification: Option<Vec<String>>,
}
