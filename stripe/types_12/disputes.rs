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
pub struct DisputeEvidence {
    #[doc = "Any server or activity logs showing proof that the customer accessed or downloaded the purchased digital product."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub access_activity_log: Option<String>,
    #[doc = "The billing address provided by the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<String>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Your subscription cancellation policy, as shown t..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_policy: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "An explanation of how and when the customer was shown your refund policy prior to purchase."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_disclosure: Option<String>,
    #[doc = "A justification for why the customer's subscription was not canceled."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_rebuttal: Option<String>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any communication with the customer that you feel..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_communication: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "The email address of the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_email_address: Option<String>,
    #[doc = "The name of the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    #[doc = "The IP address that the customer used when making the purchase."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_purchase_ip: Option<String>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A relevant document or contract showing the custo..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_signature: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation for the prior charge that can uniqu..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_documentation: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "An explanation of the difference between the disputed charge versus the prior charge that appears to be a duplicate."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_explanation: Option<String>,
    #[doc = "The Stripe ID for the prior charge which appears to be a duplicate of the disputed charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_id: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub enhanced_evidence: Option<stripe_types_10::disputes::DisputeEnhancedEvidence>,
    #[doc = "A description of the product or service that was sold."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any receipt or message sent to the customer notif..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub receipt: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Your refund policy, as shown to the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_policy: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "Documentation demonstrating that the customer was shown your refund policy prior to purchase."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_policy_disclosure: Option<String>,
    #[doc = "A justification for why the customer is not entitled to a refund."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_refusal_explanation: Option<String>,
    #[doc = "The date on which the customer received or began receiving the purchased service, in a clear human-readable format."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub service_date: Option<String>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation showing proof that a service was pr..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub service_documentation: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "The address to which a physical product was shipped."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<String>,
    #[doc = "The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_carrier: Option<String>,
    #[doc = "The date on which a physical product began its route to the shipping address, in a clear human-readable format."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_date: Option<String>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation showing proof that a product was sh..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_documentation: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "The tracking number for a physical product, obtained from the delivery service."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_tracking_number: Option<String>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any additional evidence or statements."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub uncategorized_file: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "Any additional evidence or statements."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub uncategorized_text: Option<String>,
}

#[cfg(feature = "disputes")]
#[derive(Debug, Clone, Default)]
pub struct UpdateDisputeParamsEvidenceEnhancedEvidence {
    #[doc = "Evidence provided for Mastercard compliance evidence submission."]
    pub mastercard_compliance:
        Option<stripe_types_03::disputes::UpdateDisputeParamsEvidenceEnhancedEvidenceMastercardCompliance>,
    #[doc = "Evidence provided for Visa Compelling Evidence 3.0 evidence submission."]
    pub visa_compelling_evidence_3:
        Option<stripe_types_11::disputes::UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3>,
    #[doc = "Evidence provided for Visa compliance evidence submission."]
    pub visa_compliance: Option<stripe_types_05::disputes::UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompliance>,
}

#[cfg(feature = "disputes")]
impl crate::params::FormEncode for UpdateDisputeParamsEvidenceEnhancedEvidence {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mastercard_compliance", self.mastercard_compliance.as_ref());
        form.field_opt("visa_compelling_evidence_3", self.visa_compelling_evidence_3.as_ref());
        form.field_opt("visa_compliance", self.visa_compliance.as_ref());
    }
}
