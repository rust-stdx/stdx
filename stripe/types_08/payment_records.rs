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
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceInstallments {
    #[doc = "Installment plan selected for the payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<stripe_types_06::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceInstallmentPlan>,
}

#[cfg(feature = "payment_records")]
#[derive(Debug, Clone)]
pub struct ReportPaymentParamsProcessorDetails {
    #[doc = "Information about the custom processor used to make this payment."]
    pub custom: Option<stripe_types_00::payment_records::ReportPaymentParamsProcessorDetailsCustom>,
    #[doc = "The type of the processor details."]
    pub r#type: stripe_types_05::payment_records::ReportPaymentParamsProcessorDetailsType,
}

#[cfg(feature = "payment_records")]
impl crate::params::FormEncode for ReportPaymentParamsProcessorDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("custom", self.custom.as_ref());
        form.field("type", &self.r#type);
    }
}
