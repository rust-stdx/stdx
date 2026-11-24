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
pub struct Dispute {
    #[doc = "Disputed amount."]
    #[json(default)]
    pub amount: i64,
    #[doc = "List of zero, one, or two balance transactions that show funds withdrawn and reinstated to your Stripe account as a r..."]
    #[json(default)]
    pub balance_transactions: Vec<crate::balance_transactions::BalanceTransaction>,
    #[doc = "ID of the charge that's disputed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub charge: Option<crate::Expandable<crate::charges::Charge>>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "List of eligibility types that are included in `enhanced_evidence`."]
    #[json(default)]
    pub enhanced_eligibility_types: Vec<stripe_types_04::disputes::DisputeEnhancedEligibilityTypesItem>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub evidence: Option<stripe_types_12::disputes::DisputeEvidence>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub evidence_details: Option<stripe_types_11::disputes::DisputeEvidenceDetails>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If true, it's still possible to refund the disputed payment."]
    #[json(default)]
    pub is_charge_refundable: bool,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "Network-dependent reason code for the dispute."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network_reason_code: Option<String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::disputes::DisputeObject,
    #[doc = "ID of the PaymentIntent that's disputed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<crate::Expandable<crate::payment_intents::PaymentIntent>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_details: Option<stripe_types_07::disputes::DisputePaymentMethodDetails>,
    #[doc = "Reason given by cardholder for dispute."]
    #[json(default)]
    pub reason: String,
    #[doc = "The current status of a dispute."]
    #[json(default)]
    pub status: stripe_types_00::disputes::DisputeStatus,
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
impl crate::PagedItem for Dispute {
    fn id(&self) -> &str {
        &self.id
    }
}
