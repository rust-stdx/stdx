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
pub struct MandateBlik {
    #[doc = "Date at which the mandate expires."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[doc = "Type of the mandate."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::mandates::MandateBlikType,
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
pub struct MandatePayto {
    #[doc = "Amount that will be collected."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[doc = "The type of amount that will be collected."]
    #[json(default)]
    pub amount_type: stripe_types_00::mandates::MandateOptionsPaytoAmountType,
    #[doc = "Date, in YYYY-MM-DD format, after which payments will not be collected."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[doc = "The periodicity at which payments will be collected."]
    #[json(default)]
    pub payment_schedule: stripe_types_02::mandates::MandateOptionsPaytoPaymentSchedule,
    #[doc = "The number of payments that will be made during a payment period."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payments_per_period: Option<i64>,
    #[doc = "The purpose for which payments are made."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<stripe_types_01::mandates::MandateOptionsPaytoPurpose>,
    #[doc = "Date, in YYYY-MM-DD format, from which payments will be collected."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
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
pub struct MandatePix {
    #[doc = "Determines if the amount includes the IOF tax."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_includes_iof: Option<stripe_types_00::mandates::MandatePixAmountIncludesIof>,
    #[doc = "Type of amount."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_type: Option<stripe_types_00::mandates::MandateOptionsPaytoAmountType>,
    #[doc = "Date when the mandate expires and no further payments will be charged, in `YYYY-MM-DD`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[doc = "Schedule at which the future payments will be charged."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<stripe_types_00::mandates::MandatePixPaymentSchedule>,
    #[doc = "Subscription name displayed to buyers in their bank app."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[doc = "Start date of the mandate, in `YYYY-MM-DD`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
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
pub struct MandateUpi {
    #[doc = "Amount to be charged for future payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[doc = "One of `fixed` or `maximum`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_type: Option<stripe_types_00::mandates::MandateOptionsPaytoAmountType>,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<i64>,
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
pub struct MandateUsBankAccount {
    #[doc = "Mandate collection method"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<stripe_types_00::mandates::MandateUsBankAccountCollectionMethod>,
}
