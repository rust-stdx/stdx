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
pub struct InvoicePaymentMethodOptionsAcssDebit {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_types_05::invoice_payments::InvoicePaymentMethodOptionsAcssDebitMandateOptions>,
    #[doc = "Bank account verification method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<stripe_types_03::invoice_payments::InvoicePaymentMethodOptionsAcssDebitVerificationMethod>,
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
pub struct InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<stripe_types_06::invoice_payments::InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsFilters>,
    #[doc = "The list of permissions to request."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<stripe_types_03::invoice_payments::InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissionsItem>>,
    #[doc = "Data features requested to be retrieved upon account creation."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<Vec<stripe_types_01::invoice_payments::InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetchItem>>,
}

#[cfg(feature = "invoice_payments")]
#[derive(Debug, Clone, Default)]
pub struct ListInvoicePaymentsParams {
    pub created: Option<stripe_types_04::invoice_payments::ListInvoicePaymentsParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub invoice: Option<String>,
    pub limit: Option<i64>,
    pub payment: Option<stripe_types_06::invoice_payments::ListInvoicePaymentsParamsPayment>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_00::invoice_payments::ListInvoicePaymentsParamsStatus>,
}

#[cfg(feature = "invoice_payments")]
impl crate::params::FormEncode for ListInvoicePaymentsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("invoice", self.invoice.as_ref());
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("payment", self.payment.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
    }
}
