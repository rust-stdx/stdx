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
pub struct Account {
    #[doc = "Business information about the account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<stripe_types_06::account::AccountBusinessProfile>,
    #[doc = "The business type."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub business_type: Option<stripe_types_04::account::AccountBusinessType>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<stripe_types_05::account::AccountCapabilities>,
    #[doc = "Whether the account can process charges."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub charges_enabled: Option<bool>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<stripe_types_12::accounts::LegalEntityCompany>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub controller: Option<stripe_types_08::account::AccountUnificationAccountController>,
    #[doc = "The account's country."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Time at which the account was connected."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[doc = "Three-letter ISO currency code representing the default currency for the account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<String>,
    #[doc = "Whether account details have been submitted."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub details_submitted: Option<bool>,
    #[doc = "An email address associated with the account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "External accounts (bank accounts and debit cards) currently attached to this account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub external_accounts: Option<crate::List<crate::accounts::ExternalAccount>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<stripe_types_08::account::AccountFutureRequirements>,
    #[doc = "The groups associated with the account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<stripe_types_00::account::AccountGroupMembership>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub individual: Option<stripe_types_12::accounts::Person>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::account::AccountObject,
    #[doc = "Whether the funds in this account can be paid out."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payouts_enabled: Option<bool>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub requirements: Option<stripe_types_07::account::AccountRequirements>,
    #[doc = "Options for customizing how the account functions within Stripe."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<crate::account::AccountSettings>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<stripe_types_01::account::AccountTosAcceptance>,
    #[doc = "The Stripe account type."]
    #[json(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<stripe_types_00::account::AccountType>,
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
impl crate::PagedItem for Account {
    fn id(&self) -> &str {
        &self.id
    }
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
pub struct AccountInvoicesSettings {
    #[doc = "The list of default Account Tax IDs to automatically include on invoices."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_account_tax_ids: Option<Vec<crate::Expandable<crate::tax_ids::TaxId>>>,
    #[doc = "Whether to save the payment method after a payment is completed for a one-time invoice or a subscription invoice when..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hosted_payment_method_save: Option<stripe_types_03::account::AccountInvoicesSettingsHostedPaymentMethodSave>,
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
pub struct AccountSettings {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<stripe_types_00::account::AccountBacsDebitPaymentsSettings>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub branding: Option<stripe_types_09::account::AccountBrandingSettings>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<stripe_types_05::account::AccountCardIssuingSettings>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<stripe_types_06::account::AccountCardPaymentsSettings>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub dashboard: Option<stripe_types_01::account::AccountDashboardSettings>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoices: Option<crate::account::AccountInvoicesSettings>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payments: Option<stripe_types_01::account::AccountPaymentsSettings>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payouts: Option<stripe_types_09::account::AccountPayoutSettings>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paypay_payments: Option<stripe_types_09::account::AccountPaypayPaymentsSettings>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<stripe_types_04::account::AccountSepaDebitPaymentsSettings>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub treasury: Option<stripe_types_06::account::AccountTreasurySettings>,
}
