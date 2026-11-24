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
pub struct DisputeEnhancedEligibility {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mastercard_compliance: Option<stripe_types_06::disputes::DisputeEnhancedEligibilityMastercardCompliance>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub visa_compelling_evidence_3:
        Option<stripe_types_06::disputes::DisputeEnhancedEligibilityVisaCompellingEvidence3>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub visa_compliance: Option<stripe_types_06::disputes::DisputeEnhancedEligibilityVisaCompliance>,
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
pub struct DisputeEnhancedEvidenceVisaCompellingEvidence3 {
    #[doc = "Disputed transaction details for Visa Compelling Evidence 3.0 evidence submission."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub disputed_transaction: Option<stripe_types_05::disputes::DisputeVisaCompellingEvidence3DisputedTransaction>,
    #[doc = "List of exactly two prior undisputed transaction objects for Visa Compelling Evidence 3.0 evidence submission."]
    #[json(default)]
    pub prior_undisputed_transactions:
        Vec<stripe_types_06::disputes::DisputeVisaCompellingEvidence3PriorUndisputedTransaction>,
}
