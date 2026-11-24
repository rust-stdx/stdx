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
pub struct DisputeEvidenceDetails {
    #[doc = "Date by which evidence must be submitted in order to successfully challenge dispute."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub due_by: Option<i64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub enhanced_eligibility: Option<stripe_types_08::disputes::DisputeEnhancedEligibility>,
    #[doc = "Whether evidence has been staged for this dispute."]
    #[json(default)]
    pub has_evidence: bool,
    #[doc = "Whether the last evidence submission was submitted past the due date."]
    #[json(default)]
    pub past_due: bool,
    #[doc = "The number of times evidence has been submitted."]
    #[json(default)]
    pub submission_count: i64,
}

#[cfg(feature = "disputes")]
#[derive(Debug, Clone, Default)]
pub struct UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3 {
    #[doc = "Disputed transaction details for Visa Compelling Evidence 3.0 evidence submission."]
    pub disputed_transaction: Option<stripe_types_06::disputes::UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransaction>,
    #[doc = "List of exactly two prior undisputed transaction objects for Visa Compelling Evidence 3.0 evidence submission."]
    pub prior_undisputed_transactions: Option<Vec<stripe_types_07::disputes::UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3PriorUndisputedTransactionsItem>>,
}

#[cfg(feature = "disputes")]
impl crate::params::FormEncode for UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3 {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("disputed_transaction", self.disputed_transaction.as_ref());
        if let Some(values) = self.prior_undisputed_transactions.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("prior_undisputed_transactions", index, item);
            }
        }
    }
}
