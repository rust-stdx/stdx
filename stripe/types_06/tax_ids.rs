#[cfg(feature = "tax_ids")]
#[derive(Debug, Clone)]
pub struct ListTaxIdsParamsOwner {
    #[doc = "Connected Account the tax ID belongs to."]
    pub account: Option<String>,
    #[doc = "Customer the tax ID belongs to."]
    pub customer: Option<String>,
    #[doc = "ID of the Account representing the customer that the tax ID belongs to."]
    pub customer_account: Option<String>,
    #[doc = "Type of owner referenced."]
    pub r#type: stripe_types_00::tax_ids::ListTaxIdsParamsOwnerType,
}

#[cfg(feature = "tax_ids")]
impl crate::params::FormEncode for ListTaxIdsParamsOwner {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field("type", &self.r#type);
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
pub struct TaxIdVerification {
    #[doc = "Verification status, one of `pending`, `verified`, `unverified`, or `unavailable`."]
    #[json(default)]
    pub status: stripe_types_02::tax_ids::TaxIdVerificationStatus,
    #[doc = "Verified address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verified_address: Option<String>,
    #[doc = "Verified name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<String>,
}
