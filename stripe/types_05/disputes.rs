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
pub struct DisputeVisaCompellingEvidence3DisputedTransaction {
    #[doc = "User Account ID used to log into business platform."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account_id: Option<String>,
    #[doc = "Unique identifier of the cardholder’s device derived from a combination of at least two hardware and software attribu..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_device_fingerprint: Option<String>,
    #[doc = "Unique identifier of the cardholder’s device such as a device serial number (e.g., International Mobile Equipment Ide..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_device_id: Option<String>,
    #[doc = "The email address of the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_email_address: Option<String>,
    #[doc = "The IP address that the customer used when making the purchase."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_purchase_ip: Option<String>,
    #[doc = "Categorization of disputed payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub merchandise_or_services:
        Option<stripe_types_04::disputes::DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices>,
    #[doc = "A description of the product or service that was sold."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    #[doc = "The address to which a physical product was shipped."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<stripe_types_03::disputes::DisputeTransactionShippingAddress>,
}

#[cfg(feature = "disputes")]
#[derive(Debug, Clone, Default)]
pub struct ListDisputesParams {
    pub charge: Option<String>,
    pub created: Option<stripe_types_04::disputes::ListDisputesParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payment_intent: Option<String>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "disputes")]
impl crate::params::FormEncode for ListDisputesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("charge", self.charge.as_ref());
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("payment_intent", self.payment_intent.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(feature = "disputes")]
pub type UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3PriorUndisputedTransactionsItemShippingAddress = stripe_types_01::disputes::UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionShippingAddress;

#[cfg(feature = "disputes")]
pub type UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompliance =
    stripe_types_03::disputes::UpdateDisputeParamsEvidenceEnhancedEvidenceMastercardCompliance;
