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
pub struct AccountBrandingSettings {
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) An icon for the account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A logo for the account that will be used in Check..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub logo: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "A CSS hex color value representing the primary branding color for this account"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,
    #[doc = "A CSS hex color value representing the secondary branding color for this account"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub secondary_color: Option<String>,
}

#[cfg(any(feature = "accounts", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct AccountCapabilityRequirements {
    #[doc = "Fields that are due and can be resolved by providing the corresponding alternative fields instead."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alternatives: Option<Vec<stripe_types_01::account::AccountRequirementsAlternative>>,
    #[doc = "The date by which all required account information must be both submitted and verified."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub current_deadline: Option<i64>,
    #[doc = "Fields that need to be resolved to keep the capability enabled."]
    #[json(default)]
    pub currently_due: Vec<String>,
    #[doc = "Description of why the capability is disabled."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<stripe_types_02::account::AccountCapabilityFutureRequirementsDisabledReason>,
    #[doc = "Fields that are `currently_due` and need to be collected again because validation or verification failed."]
    #[json(default)]
    pub errors: Vec<stripe_types_06::account::AccountRequirementsError>,
    #[doc = "Fields you must collect when all thresholds are reached."]
    #[json(default)]
    pub eventually_due: Vec<String>,
    #[doc = "Fields that haven't been resolved by `current_deadline`."]
    #[json(default)]
    pub past_due: Vec<String>,
    #[doc = "Fields that are being reviewed, or might become required depending on the results of a review."]
    #[json(default)]
    pub pending_verification: Vec<String>,
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
pub struct AccountPayoutSettings {
    #[doc = "A Boolean indicating if Stripe should try to reclaim negative balances from an attached bank account."]
    #[json(default)]
    pub debit_negative_balances: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<stripe_types_06::transfers::TransferSchedule>,
    #[doc = "The text that appears on the bank account statement for payouts."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
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
pub struct AccountPaypayPaymentsSettings {
    #[doc = "Additional files that are required to support the onboarding process of your business."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub additional_files: Option<Vec<String>>,
    #[doc = "The type of goods your business sells."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub goods_type: Option<stripe_types_03::account::AccountPaypayPaymentsSettingsGoodsType>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub site: Option<stripe_types_06::account::AccountPaypaySiteSettings>,
}
