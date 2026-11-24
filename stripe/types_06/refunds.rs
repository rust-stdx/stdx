#[cfg(feature = "refunds")]
pub type CancelRefundParams = stripe_types_03::refunds::GetRefundParams;

#[cfg(feature = "refunds")]
#[derive(Debug, Clone, Default)]
pub struct CreateRefundParams {
    pub amount: Option<i64>,
    #[doc = "The identifier of the charge to refund."]
    pub charge: Option<String>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: Option<String>,
    #[doc = "Customer whose customer balance to refund from."]
    pub customer: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "For payment methods without native refund support (e.g., Konbini, PromptPay), use this email from the customer to rec..."]
    pub instructions_email: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Origin of the refund"]
    pub origin: Option<stripe_types_00::refunds::CreateRefundParamsOrigin>,
    #[doc = "The identifier of the PaymentIntent to refund."]
    pub payment_intent: Option<String>,
    #[doc = "String indicating the reason for the refund."]
    pub reason: Option<stripe_types_00::refunds::CreateRefundParamsReason>,
    #[doc = "Boolean indicating whether the application fee should be refunded when refunding this charge."]
    pub refund_application_fee: Option<bool>,
    #[doc = "Boolean indicating whether the transfer should be reversed when refunding this charge."]
    pub reverse_transfer: Option<bool>,
}

#[cfg(feature = "refunds")]
impl crate::params::FormEncode for CreateRefundParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("charge", self.charge.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("instructions_email", self.instructions_email.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("origin", self.origin.as_ref());
        form.field_opt("payment_intent", self.payment_intent.as_ref());
        form.field_opt("reason", self.reason.as_ref());
        form.field_opt("refund_application_fee", self.refund_application_fee.as_ref());
        form.field_opt("reverse_transfer", self.reverse_transfer.as_ref());
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
pub struct RefundNextActionDisplayDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email_sent: Option<stripe_types_00::common::EmailSent>,
    #[doc = "The expiry timestamp."]
    #[json(default)]
    pub expires_at: i64,
}
