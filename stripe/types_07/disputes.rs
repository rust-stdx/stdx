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
pub struct DisputePaymentMethodDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amazon_pay: Option<stripe_types_06::disputes::DisputePaymentMethodDetailsAmazonPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types_06::disputes::DisputePaymentMethodDetailsCard>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_types_04::disputes::DisputePaymentMethodDetailsKlarna>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_types_03::disputes::DisputePaymentMethodDetailsPaypal>,
    #[doc = "Payment method type."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::disputes::DisputePaymentMethodDetailsType,
}

#[cfg(feature = "disputes")]
#[derive(Debug, Clone, Default)]
pub struct UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3PriorUndisputedTransactionsItem {
    #[doc = "Stripe charge ID for the Visa Compelling Evidence 3.0 eligible prior charge."]
    pub charge: String,
    #[doc = "User Account ID used to log into business platform."]
    pub customer_account_id: Option<json::OwnedValue>,
    #[doc = "Unique identifier of the cardholder’s device derived from a combination of at least two hardware and software attribu..."]
    pub customer_device_fingerprint: Option<json::OwnedValue>,
    #[doc = "Unique identifier of the cardholder’s device such as a device serial number (e.g., International Mobile Equipment Ide..."]
    pub customer_device_id: Option<json::OwnedValue>,
    #[doc = "The email address of the customer."]
    pub customer_email_address: Option<json::OwnedValue>,
    #[doc = "The IP address that the customer used when making the purchase."]
    pub customer_purchase_ip: Option<json::OwnedValue>,
    #[doc = "A description of the product or service that was sold."]
    pub product_description: Option<json::OwnedValue>,
    #[doc = "The address to which a physical product was shipped."]
    pub shipping_address: Option<stripe_types_05::disputes::UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3PriorUndisputedTransactionsItemShippingAddress>,
}

#[cfg(feature = "disputes")]
impl crate::params::FormEncode
    for UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3PriorUndisputedTransactionsItem
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("charge", &self.charge);
        form.field_opt("shipping_address", self.shipping_address.as_ref());
    }
}
