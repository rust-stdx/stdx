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
pub struct AccountBusinessProfile {
    #[doc = "The applicant's gross annual revenue for its preceding fiscal year."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub annual_revenue: Option<stripe_types_00::account::AccountAnnualRevenue>,
    #[doc = "An estimated upper bound of employees, contractors, vendors, etc."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub estimated_worker_count: Option<i64>,
    #[doc = "[The merchant category code for the account](/connect/setting-mcc)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,
    #[doc = "Whether the business is a minority-owned, women-owned, and/or LGBTQI+ -owned business."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub minority_owned_business_designation:
        Option<Vec<stripe_types_03::account::AccountBusinessProfileMinorityOwnedBusinessDesignationItem>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub monthly_estimated_revenue: Option<stripe_types_02::account::AccountMonthlyEstimatedRevenue>,
    #[doc = "The customer-facing business name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Internal-only description of the product sold or service provided by the business."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    #[doc = "A link to the business's publicly available terms related to the Specified Commercial Transaction Act."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub specified_commercial_transactions_act_url: Option<String>,
    #[doc = "A publicly available mailing address for sending support issues to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub support_address: Option<stripe_types_01::common::Address>,
    #[doc = "A publicly available email address for sending support issues to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    #[doc = "A publicly available phone number to call with support issues."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub support_phone: Option<String>,
    #[doc = "A publicly available website for handling support issues."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
    #[doc = "The business's publicly available website."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
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
pub struct AccountCardPaymentsSettings {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub decline_on: Option<stripe_types_03::account::AccountDeclineChargeOn>,
    #[doc = "The default text that appears on credit card statements when a charge is made."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix: Option<String>,
    #[doc = "The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix_kana: Option<String>,
    #[doc = "The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix_kanji: Option<String>,
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
pub struct AccountPaypaySiteSettings {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub accessible: Option<stripe_types_01::account::AccountPaypaySiteAccessibleSettings>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub in_development: Option<stripe_types_04::account::AccountPaypaySiteInDevelopmentSettings>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub restricted: Option<stripe_types_04::account::AccountPaypaySiteRestrictedSettings>,
    #[doc = "The status of your business's website."]
    #[json(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<stripe_types_00::account::AccountPaypaySiteSettingsType>,
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
pub struct AccountRequirementsError {
    #[doc = "The code for the type of error."]
    #[json(default)]
    pub code: stripe_types_02::account::AccountRequirementsErrorCode,
    #[doc = "An informative message that indicates the error type and provides additional details about the error."]
    #[json(default)]
    pub reason: String,
    #[doc = "The specific user onboarding requirement field (in the requirements hash) that needs to be resolved."]
    #[json(default)]
    pub requirement: String,
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
pub struct AccountTreasurySettings {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<stripe_types_03::account::AccountTermsOfService>,
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
pub struct AccountUnificationAccountControllerFees {
    #[doc = "A value indicating the responsible payer of a bundle of Stripe fees for pricing-control eligible products on this acc..."]
    #[json(default)]
    pub payer: stripe_types_04::account::AccountUnificationAccountControllerFeesPayer,
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
pub struct AccountUnificationAccountControllerLosses {
    #[doc = "A value indicating who is liable when this account can't pay back negative balances from payments."]
    #[json(default)]
    pub payments: stripe_types_01::account::AccountUnificationAccountControllerRequirementCollection,
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
pub struct AccountUnificationAccountControllerStripeDashboard {
    #[doc = "A value indicating the Stripe dashboard this account has access to independent of the Connect application."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::account::AccountUnificationAccountControllerStripeDashboardType,
}
