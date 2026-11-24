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
pub struct LinkedAccountOptionsCommon {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<stripe_types_05::payment_intents::PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFilters>,
    #[doc = "The list of permissions to request."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<stripe_types_02::financial_connections::FinancialConnectionsAccountPermissionsItem>>,
    #[doc = "Data features requested to be retrieved upon account creation."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<Vec<stripe_types_01::financial_connections::FinancialConnectionsSessionPrefetchItem>>,
    #[doc = "For webview integrations only."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}
