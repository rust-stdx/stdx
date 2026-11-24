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
pub struct CancellationDetails {
    #[doc = "Additional comments about why the user canceled the subscription, if the subscription was canceled explicitly by the ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "The customer submitted reason for why they canceled, if the subscription was canceled explicitly by the user."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub feedback: Option<stripe_types_02::common::CancellationDetailsFeedback>,
    #[doc = "Customized feedback options that provide deeper insight into why the subscription was canceled, if the subscription w..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub feedback_option: Option<crate::Expandable<stripe_types_06::billing::BillingFeedbackOption>>,
    #[doc = "Why this subscription was canceled."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<stripe_types_00::common::CancellationDetailsReason>,
}
