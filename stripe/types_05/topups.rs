#[cfg(feature = "topups")]
#[derive(Debug, Clone)]
pub struct CreateTopupParamsPaymentMethodOptionsUsBankAccount {
    pub network: stripe_types_00::topups::CreateTopupParamsPaymentMethodOptionsUsBankAccountNetwork,
}

#[cfg(feature = "topups")]
impl crate::params::FormEncode for CreateTopupParamsPaymentMethodOptionsUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("network", &self.network);
    }
}

#[cfg(feature = "topups")]
pub type ListTopupsParamsCreated = stripe_types_04::topups::ListTopupsParamsAmount;

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
pub struct TopupResourceUsBankAccount {
    #[doc = "The US bank transfer network used for this top-up."]
    #[json(default)]
    pub network: stripe_types_03::topups::TopupResourceUsBankAccountNetwork,
}
